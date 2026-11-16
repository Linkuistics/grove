#!/usr/bin/env python3
"""Run one comparison case under one arm.

Usage:
  run.py probe ARM
  run.py doc CASE ARM
  run.py exchange ARM

Each run copies the case fixture into a scratch directory, loads only the
arm's six plugins, and sends the case prompt unchanged. The run stores its
prompt digest, final message, changed files, gate output and a compressed
event trace under outputs/CASE/ARM/.
"""
import gzip
import hashlib
import json
import os
import shutil
import signal
import subprocess
import sys
import threading
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
CASES = ROOT.parent / "cases"
SCRATCH = Path(os.environ["COMPARISON_SCRATCH"])
ARMS = SCRATCH / "arms"

MODEL = "claude-opus-5-5"
PROXY_MODEL = "claude-sonnet-5-5"
MAX_EXCHANGE_TURNS = 10

DOC_CASES = {
    "d1-draft": {"fixture": "transfer/fixtures/B-draft",
                 "gate": ["python3", "tools/gate.py", "docs/editorconfig"]},
    "d2-copy-edit": {"fixture": "ail/fixtures/A-copy-edit",
                     "gate": ["python3", "tools/gate.py", "--withheld", "tools/withheld.txt",
                              "semantic/specification/interface-language"]},
    "d3-proof": {"fixture": "ail/fixtures/A-proof",
                 "gate": ["python3", "tools/gate.py", "--withheld", "tools/withheld.txt",
                          "semantic/specification/interface-language"]},
}
EXCHANGE = {"fixture": "exchange/fixtures/E-design",
            "gate": ["python3", "tools/gate.py", "docs/courier"]}


def plugin_flags(arm):
    flags = []
    for plugin in sorted((ARMS / arm).iterdir()):
        flags += ["--plugin-dir", str(plugin)]
    # The writing plugin depends on the technical-communication hooks plugin.
    # Without it Claude Code disables the writing plugin. Production installs it.
    hooks = ARMS / arm / "writing" / "craft" / "technical-communication"
    return flags + ["--plugin-dir", str(hooks)]


def claude_cmd(arm, model, tools, extra=()):
    cmd = ["claude", "-p", "--model", model,
           "--setting-sources", "project", "--strict-mcp-config",
           "--output-format", "stream-json", "--verbose",
           "--permission-mode", "bypassPermissions",
           "--tools", tools,
           "--disallowed-tools", "Bash(grove-llm:*)", "Bash(ravel:*)"]
    if arm:
        cmd += plugin_flags(arm)
    return cmd + list(extra)


# A session may leave a long-lived process behind, such as the preview server
# that the design skill starts for the human. `claude -p` waits for its background
# tasks, so such a process keeps the call open for ever. Once the call has
# emitted a result and stays open this long, the runner stops its process group.
SETTLE_SECONDS = 300


def run_claude(cmd, prompt, cwd, trace):
    """Run one call. Append its events to trace. Return the merged result event."""
    started = time.time()
    proc = subprocess.Popen(cmd + ["--", prompt], cwd=cwd, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, text=True, start_new_session=True)
    results, state = [], {"last_result": None, "stopped": False}

    def watchdog():
        while proc.poll() is None:
            last = state["last_result"]
            if last is not None and time.time() - last > SETTLE_SECONDS:
                state["stopped"] = True
                try:
                    os.killpg(proc.pid, signal.SIGTERM)
                except ProcessLookupError:
                    pass
                return
            time.sleep(5)

    threading.Thread(target=watchdog, daemon=True).start()
    with open(trace, "a") as out:
        for line in proc.stdout:
            out.write(line)
            try:
                event = json.loads(line)
            except json.JSONDecodeError:
                continue
            if event.get("type") == "result":
                results.append(event)
                state["last_result"] = time.time()
    proc.wait()
    if state["stopped"]:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
    return {"exit": proc.returncode, "stderr": proc.stderr.read()[-4000:],
            "seconds": round(time.time() - started, 1), "result": merge_results(results),
            "stopped_after_result": state["stopped"]}


def merge_results(results):
    """Merge the result events of one call.

    A background task that the session starts can end after the session's
    report. Claude Code then resumes the session for one more turn and emits a
    second result event. The first result holds the report. The cost in the
    last result is the total for the call.
    """
    if not results:
        return None
    merged = dict(results[0])
    merged["num_turns"] = sum(r.get("num_turns") or 0 for r in results)
    merged["total_cost_usd"] = results[-1].get("total_cost_usd")
    merged["is_error"] = any(r.get("is_error") for r in results)
    merged["result_count"] = len(results)
    later = [r.get("result") or "" for r in results[1:]]
    if later:
        merged["result"] = (merged.get("result") or "") + "".join(
            "\n\n---\n\nLater result, after a background task ended:\n\n" + text
            for text in later)
    return merged


def remeta(case, arm):
    """Rebuild a document run's final message and result fields from its trace."""
    dest = ROOT / "outputs" / case / arm
    with gzip.open(dest / "trace.jsonl.gz", "rt") as trace:
        events = [json.loads(line) for line in trace if line.strip()]
    result = merge_results([e for e in events if e.get("type") == "result"]) or {}
    (dest / "final-message.md").write_text((result.get("result") or "") + "\n")
    meta = json.loads((dest / "meta.json").read_text())
    meta.update({"is_error": result.get("is_error"), "num_turns": result.get("num_turns"),
                 "cost_usd": result.get("total_cost_usd"),
                 "result_count": result.get("result_count")})
    (dest / "meta.json").write_text(json.dumps(meta, indent=1) + "\n")


def snapshot(tree):
    files = {}
    for path in tree.rglob("*"):
        if path.is_file() and ".git" not in path.parts:
            files[str(path.relative_to(tree))] = hashlib.sha256(path.read_bytes()).hexdigest()
    return files


def store_changes(before, work, dest):
    after = snapshot(work)
    changed = sorted(p for p in after if before.get(p) != after[p])
    removed = sorted(p for p in before if p not in after)
    for rel in changed:
        target = dest / "files" / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(work / rel, target)
    return changed, removed


def gate(cmd, work):
    proc = subprocess.run(cmd, cwd=work, capture_output=True, text=True)
    return {"command": " ".join(cmd), "exit": proc.returncode,
            "output": (proc.stdout + proc.stderr)[-6000:]}


def prepare(case, arm, fixture):
    work = SCRATCH / "runs" / case / arm
    if work.exists():
        shutil.rmtree(work)
    shutil.copytree(CASES / fixture, work)
    dest = ROOT / "outputs" / case / arm
    if dest.exists():
        shutil.rmtree(dest)
    dest.mkdir(parents=True)
    return work, dest


def finish(dest, meta, trace):
    with open(trace, "rb") as raw, gzip.open(dest / "trace.jsonl.gz", "wb") as packed:
        shutil.copyfileobj(raw, packed)
    trace.unlink()
    (dest / "meta.json").write_text(json.dumps(meta, indent=2) + "\n")


def revisions(arm):
    rows = [line.split("\t") for line in (HERE / "arms.tsv").read_text().splitlines()
            if line and not line.startswith("#")]
    return {plugin: rev.strip() for a, plugin, _repo, rev in rows if a == arm}


def doc(case, arm):
    spec = DOC_CASES[case]
    prompt_path = ROOT / "prompts" / f"{case}.txt"
    prompt = prompt_path.read_text()
    work, dest = prepare(case, arm, spec["fixture"])
    before = snapshot(work)
    trace = dest / "trace.jsonl"
    call = run_claude(claude_cmd(arm, MODEL, "Bash,Read,Write,Edit,Glob,Grep,Skill"),
                      prompt, work, trace)
    changed, removed = store_changes(before, work, dest)
    result = call["result"] or {}
    (dest / "final-message.md").write_text((result.get("result") or "") + "\n")
    meta = {"case": case, "arm": arm, "model": MODEL, "revisions": revisions(arm),
            "prompt": str(prompt_path.relative_to(ROOT)),
            "prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
            "exit": call["exit"], "seconds": call["seconds"], "stderr": call["stderr"],
            "is_error": result.get("is_error"), "num_turns": result.get("num_turns"),
            "cost_usd": result.get("total_cost_usd"), "usage": result.get("usage"),
            "result_count": result.get("result_count"),
            "stopped_after_result": call["stopped_after_result"], "changed": changed, "removed": removed, "gate": gate(spec["gate"], work)}
    finish(dest, meta, trace)
    print(json.dumps({k: meta[k] for k in ("case", "arm", "exit", "is_error", "num_turns",
                                           "cost_usd", "changed")}, indent=1))


def render_sheet(sheet):
    blocks = []
    for entry in sheet["entries"]:
        blocks.append(f"key: {entry['key']}\nraised when: {entry['topic']}\n"
                      f"the person says: {entry['text']}")
    return "\n\n".join(blocks)


def proxy_answer(sheet, transcript, message, trace):
    template = (CASES / "exchange" / "proxy-prompt.txt").read_text()
    prompt = (template.replace("{{sheet}}", render_sheet(sheet))
              .replace("{{transcript}}", transcript or "(nothing yet)")
              .replace("{{message}}", message))
    call = run_claude(claude_cmd(None, PROXY_MODEL, ""), prompt, SCRATCH, trace)
    text = ((call["result"] or {}).get("result") or "").strip()
    start, end = text.find("["), text.rfind("]")
    keys = json.loads(text[start:end + 1]) if start >= 0 < end else ["fallback"]
    texts = {e["key"]: e["text"] for e in sheet["entries"]}
    texts["fallback"] = sheet["fallback"]["text"]
    parts = [texts[k] for k in keys if k in texts]
    answer = "\n\n".join(parts) if parts else sheet["nothing_to_add"]["text"]
    return keys, answer


def exchange(arm):
    case = "e1-design-exchange"
    prompt_path = ROOT / "prompts" / f"{case}.txt"
    prompt = prompt_path.read_text()
    sheet = json.loads((CASES / "exchange" / "answer-sheet.json").read_text())
    work, dest = prepare(case, arm, EXCHANGE["fixture"])
    before = snapshot(work)
    trace = dest / "trace.jsonl"
    proxy_trace = SCRATCH / f"proxy-{arm}.jsonl"
    if proxy_trace.exists():
        proxy_trace.unlink()
    turns, transcript, calls = [], "", []
    session_id, message = None, prompt
    for turn in range(MAX_EXCHANGE_TURNS):
        extra = ["--resume", session_id] if session_id else []
        call = run_claude(claude_cmd(arm, MODEL, "Bash,Read,Write,Edit,Glob,Grep,Skill", extra),
                          message, work, trace)
        calls.append({k: call[k] for k in ("exit", "seconds", "stderr", "stopped_after_result")})
        result = call["result"] or {}
        session_id = result.get("session_id") or session_id
        said = (result.get("result") or "").strip()
        keys, answer = proxy_answer(sheet, transcript, said, proxy_trace)
        turns.append({"turn": turn + 1, "session": said, "proxy_keys": keys,
                      "human_proxy": answer, "cost_usd": result.get("total_cost_usd")})
        transcript += f"\n\nSESSION:\n{said}\n\nPERSON:\n{answer}"
        if keys == []:
            break  # the session asked the human nothing, so the exchange is over
        message = answer
    changed, removed = store_changes(before, work, dest)
    lines = ["# Exchange transcript", "",
             "The human side is a labelled model proxy. It picks scripted answers from",
             "`cases/exchange/answer-sheet.json`; it writes no words of its own.", ""]
    for t in turns:
        lines += [f"## Turn {t['turn']}", "", "### Session", "", t["session"], "",
                  f"### Human (model proxy; keys: {', '.join(t['proxy_keys']) or 'none'})", "",
                  t["human_proxy"], ""]
    (dest / "transcript.md").write_text("\n".join(lines))
    with open(proxy_trace, "rb") as raw, gzip.open(dest / "proxy-trace.jsonl.gz", "wb") as packed:
        shutil.copyfileobj(raw, packed)
    meta = {"case": case, "arm": arm, "model": MODEL, "proxy_model": PROXY_MODEL,
            "revisions": revisions(arm), "prompt": str(prompt_path.relative_to(ROOT)),
            "prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
            "calls": calls, "turns": [{k: t[k] for k in ("turn", "proxy_keys", "cost_usd")}
                                      for t in turns],
            "changed": changed, "removed": removed, "gate": gate(EXCHANGE["gate"], work)}
    finish(dest, meta, trace)
    print(json.dumps({"case": case, "arm": arm, "turns": len(turns), "changed": changed}, indent=1))


def retranscript(arm):
    """Restore session messages that a first version of this runner dropped.

    That version kept only the last result event of each call. When a background
    task ended after the session's reply, the reply was lost from the transcript,
    and the proxy saw only the follow-up. This verb groups the trace's result
    events by call, using each turn's cumulative cost as the boundary. It writes
    the restored messages to transcript.md and records the repair in meta.json.
    """
    dest = ROOT / "outputs" / "e1-design-exchange" / arm
    meta = json.loads((dest / "meta.json").read_text())
    with gzip.open(dest / "trace.jsonl.gz", "rt") as trace:
        results = [e for e in (json.loads(line) for line in trace if line.strip())
                   if e.get("type") == "result"]
    text = (dest / "transcript.md").read_text()
    repaired = []
    index = 0
    for turn in meta["turns"]:
        group = []
        while index < len(results):
            group.append(results[index])
            index += 1
            if results[index - 1].get("total_cost_usd") == turn["cost_usd"]:
                break
        if len(group) < 2:
            continue
        dropped = [(r.get("result") or "").strip() for r in group[:-1]]
        shown = (group[-1].get("result") or "").strip()
        note = ("> **Runner repair.** The first runner kept only the last message of this "
                "call. The proxy saw only the message after the line below. The session's "
                "earlier message is restored here from the trace.\n\n")
        restored = note + "\n\n".join(dropped) + "\n\n---\n\n" + shown
        text = text.replace(shown, restored, 1)
        repaired.append(turn["turn"])
    (dest / "transcript.md").write_text(text)
    meta["transcript_repaired_turns"] = repaired
    (dest / "meta.json").write_text(json.dumps(meta, indent=1) + "\n")
    print(arm, "repaired turns", repaired)


def probe(arm):
    work = SCRATCH / "probe"
    work.mkdir(exist_ok=True)
    trace = SCRATCH / f"probe-{arm}.jsonl"
    call = run_claude(claude_cmd(arm, PROXY_MODEL, "Read,Skill"),
                      "List every skill name available to you through the Skill tool, one per "
                      "line. Then list any MCP servers. Then quote the first line of any "
                      "user-level instructions, CLAUDE.md content or session-hook context in "
                      "your context, or say there is none. Do nothing else.", work, trace)
    print((call["result"] or {}).get("result"), call["stderr"])


if __name__ == "__main__":
    verb = sys.argv[1]
    {"probe": lambda: probe(sys.argv[2]), "doc": lambda: doc(sys.argv[2], sys.argv[3]),
     "exchange": lambda: exchange(sys.argv[2]),
     "remeta": lambda: remeta(sys.argv[2], sys.argv[3]),
     "retranscript": lambda: retranscript(sys.argv[2])}[verb]()

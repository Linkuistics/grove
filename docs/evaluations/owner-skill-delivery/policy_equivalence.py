"""Compare the owner's Rhai configuration with the policy.ts that it converts.

The script runs `ravel start-session` for each routed kind in each fixture
directory, once with policy.ts under the Ravel 1.1.0 keg and once with the
candidate config.rhai under the accepted Ravel 2.0 build. Each run has its own
private HOME and a private PATH whose `claude` and `codex` are fake agents. A
fake agent records its program name and arguments, then ends the session with
`ravel end-session`. No real harness runs.

The fixture directories are a plain directory, a primary jj workspace, a
secondary jj workspace that jj created, and two hand-written secondary
workspaces whose `.jj/repo` file names the store by an absolute path and by a
relative path with surrounding whitespace. Three refusal cases follow: a kind
that the policy does not list, a whitespace-only `.jj/repo` file and an
unreadable one.

A routed case passes when both builds exit with the same status, launch the
same program with the same argument vector and report the same provider,
model, effort and reason. A refusal case passes when both builds refuse with
the same exit status, script code and remedy. The message must also be equal,
except for the unreadable store, where the host's error text differs by design.

The routed kinds come from the `case "..."` labels of policy.ts. The script
also reads the `"..." =>` labels of the Rhai file and fails when the two sets
differ.

The accepted 2.0 build is the aarch64-apple-darwin archive in
target/ravel-archives-2.0. The script extracts it into OUT and checks its
`bin/ravel` digest against scripts/harness-trials/builds.json.

Usage:
    python3 -I policy_equivalence.py --out DIR [--ts PATH] [--rhai PATH]
        [--ravel1 PATH] [--ravel2 PATH]

OUT must not exist. The script writes OUT/record.json with the digests of
both files and both executables, and the result of each case. Exit status 0
means that every case passed.
"""
import argparse, glob, hashlib, json, os, re, shutil, subprocess, sys, tarfile

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, "..", "..", ".."))
RAVEL1 = "/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel"
PROMPT = (
    "--help is not a flag here.\n"
    "Line two has 'single' and \"double\" quotes, a $DOLLAR and a `tick`.\n"
    "Unicode: ✓ — é\n"
)
FAKE = """#!/bin/sh
{ printf '%s\\0' "$(basename "$0")"; for a in "$@"; do printf '%s\\0' "$a"; done; } > "$FAKE_OUT"
exec "$FAKE_RAVEL" end-session recorded
"""


def sha256(path):
    with open(path, "rb") as f:
        return hashlib.sha256(f.read()).hexdigest()


def kinds_of(ts_path, rhai_path):
    with open(ts_path, encoding="utf-8") as f:
        ts = re.findall(r'^\s*case "([^"]+)":', f.read(), re.MULTILINE)
    with open(rhai_path, encoding="utf-8") as f:
        rhai = re.findall(r'^\s*"([^"]+)" =>', f.read(), re.MULTILINE)
    if not ts or sorted(ts) != sorted(rhai):
        sys.exit(f"the kind sets differ: policy.ts {sorted(ts)}, config.rhai {sorted(rhai)}")
    return ts


def accepted_ravel2(out):
    with open(os.path.join(ROOT, "scripts/harness-trials/builds.json")) as f:
        want = json.load(f)["ravel2"]["binRavelSha256"]["Darwin-arm64"]
    archives = glob.glob(os.path.join(ROOT, "target/ravel-archives-2.0/ravel-v*-aarch64-apple-darwin.tar.xz"))
    if len(archives) != 1:
        sys.exit(f"expected one darwin archive in target/ravel-archives-2.0, found {archives}")
    dest = os.path.join(out, "ravel-2.0")
    with tarfile.open(archives[0]) as tar:
        tar.extractall(dest, filter="tar")
    (binary,) = glob.glob(os.path.join(dest, "*", "bin", "ravel"))
    return binary, want


def jj(*args, cwd):
    subprocess.run(["jj", *args], cwd=cwd, check=True, capture_output=True)


def fixtures(base):
    """The fixture directories by name. Each refusal fixture names its case."""
    ws = os.path.join(base, "ws")
    os.makedirs(os.path.join(ws, "plain"))
    jj("git", "init", "--colocate", "main", cwd=ws)
    jj("workspace", "add", "../secondary", cwd=os.path.join(ws, "main"))
    store = os.path.join(ws, "main", ".jj", "repo")

    def manual(name, text, mode=0o644):
        os.makedirs(os.path.join(ws, name, ".jj"))
        path = os.path.join(ws, name, ".jj", "repo")
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
        os.chmod(path, mode)

    manual("absolute", store)
    manual("relative", "  ../../main/.jj/repo\n\n")
    manual("empty", " \n\t\n")
    manual("unreadable", "../../main/.jj/repo", 0o000)
    routed = ["plain", "main", "secondary", "absolute", "relative"]
    return ws, routed


def run(ravel, home, bindir, kind, cwd, prompt_file, out_file):
    env = {
        "HOME": home,
        "PATH": os.pathsep.join([bindir, "/usr/bin", "/bin"]),
        "TMPDIR": os.environ.get("TMPDIR", "/tmp"),
        "LANG": "en_AU.UTF-8",
        "FAKE_OUT": out_file,
        "FAKE_RAVEL": ravel,
    }
    proc = subprocess.run(
        [ravel, "start-session", "--json", "--kind", kind, "--prompt-file", prompt_file],
        check=False, cwd=cwd, env=env, capture_output=True, timeout=60,
    )
    argv = None
    if os.path.exists(out_file):
        with open(out_file, "rb") as f:
            argv = [a.decode("utf-8") for a in f.read().split(b"\0")[:-1]]
    notice = {}
    for line in proc.stderr.decode("utf-8", "replace").splitlines():
        if line.startswith("{"):
            notice = json.loads(line)
    result = {"exit": proc.returncode, "argv": argv}
    if "selection" in notice:
        result["selection"] = notice["selection"]
    if "error" in notice:
        error = notice["error"]
        message = error.get("message", "")
        result["refusal"] = {
            "code": error.get("policyCode", error.get("configCode")),
            "message": message.split("refused the selection: ", 1)[-1],
            "remedy": error.get("remedy"),
        }
    return result


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", required=True)
    parser.add_argument("--ts", default=os.path.expanduser("~/.config/ravel/policy.ts"))
    parser.add_argument("--rhai", default=os.path.join(HERE, "owner-config.rhai"))
    parser.add_argument("--ravel1", default=RAVEL1)
    parser.add_argument("--ravel2")
    args = parser.parse_args()

    out = os.path.abspath(args.out)
    os.makedirs(out)
    ts_digest, rhai_digest = sha256(args.ts), sha256(args.rhai)
    kinds = kinds_of(args.ts, args.rhai)

    ravel2, want2 = accepted_ravel2(out) if args.ravel2 is None else (args.ravel2, None)
    if want2 is not None and sha256(ravel2) != want2:
        sys.exit(f"{ravel2} is not the accepted 2.0 build: {sha256(ravel2)} != {want2}")

    bindir = os.path.join(out, "bin")
    os.makedirs(bindir)
    with open(os.path.join(bindir, "fake"), "w") as f:
        f.write(FAKE)
    os.chmod(os.path.join(bindir, "fake"), 0o755)
    for name in ("claude", "codex"):
        os.symlink("fake", os.path.join(bindir, name))

    builds = {
        "1.1": {"ravel": args.ravel1, "file": "policy.ts", "source": args.ts},
        "2.0": {"ravel": ravel2, "file": "config.rhai", "source": args.rhai},
    }
    for name, build in builds.items():
        config = os.path.join(out, "homes", name, "home", ".config", "ravel")
        os.makedirs(config)
        shutil.copyfile(build["source"], os.path.join(config, build["file"]))
        build["home"] = os.path.join(out, "homes", name, "home")
    # Each home must hold only its own configuration file.
    for build in builds.values():
        if os.listdir(os.path.join(build["home"], ".config", "ravel")) != [build["file"]]:
            sys.exit(f"{build['home']} holds more than {build['file']}")

    prompt_file = os.path.join(out, "prompt.txt")
    with open(prompt_file, "w", encoding="utf-8") as f:
        f.write(PROMPT)

    ws, routed = fixtures(out)
    cases = [(kind, place, "routed") for place in routed for kind in kinds]
    cases += [("no-such-kind", "plain", "refused"), ("impl", "empty", "refused"),
              ("impl", "unreadable", "refused")]

    results, failures = [], 0
    for index, (kind, place, expect) in enumerate(cases):
        cwd = os.path.join(ws, place)
        pair = {}
        for name, build in builds.items():
            out_file = os.path.join(out, "argv", f"{index:03d}-{name}")
            os.makedirs(os.path.dirname(out_file), exist_ok=True)
            pair[name] = run(build["ravel"], build["home"], bindir, kind, cwd, prompt_file, out_file)
        a, b = pair["1.1"], pair["2.0"]
        if expect == "routed":
            ok = (a["exit"] == b["exit"] == 0 and a["argv"] is not None and a["argv"] == b["argv"]
                  and a.get("selection") == b.get("selection") and a["argv"][-1] == PROMPT)
        else:
            ra, rb = a.get("refusal"), b.get("refusal")
            ok = (a["exit"] == b["exit"] == 3 and a["argv"] is None and b["argv"] is None
                  and ra is not None and rb is not None and ra["code"] == rb["code"]
                  and ra["remedy"] == rb["remedy"]
                  and (place == "unreadable" or ra["message"] == rb["message"]))
        failures += not ok
        results.append({"kind": kind, "place": place, "expect": expect, "pass": ok,
                        "ravel1.1": a, "ravel2.0": b})
        mark = "pass" if ok else "FAIL"
        detail = (" ".join(a["argv"][:-1]) if expect == "routed" and a["argv"]
                  else (a.get("refusal") or {}).get("code"))
        print(f"{mark} {place:10} {kind:30} {detail}")

    record = {
        "schemaVersion": 1,
        "inputs": {
            "policy.ts": {"path": args.ts, "sha256": ts_digest},
            "config.rhai": {"path": args.rhai, "sha256": rhai_digest},
            "ravel1.1": {"path": args.ravel1, "sha256": sha256(args.ravel1)},
            "ravel2.0": {"path": ravel2, "sha256": sha256(ravel2)},
        },
        "inputsUnchanged": sha256(args.ts) == ts_digest and sha256(args.rhai) == rhai_digest,
        "kinds": kinds,
        "prompt": PROMPT,
        "cases": results,
        "passed": len(results) - failures,
        "failed": failures,
    }
    with open(os.path.join(out, "record.json"), "w", encoding="utf-8") as f:
        json.dump(record, f, indent=2, ensure_ascii=False)
    os.chmod(os.path.join(ws, "unreadable", ".jj", "repo"), 0o644)
    print(f"{len(results) - failures} passed, {failures} failed; inputs unchanged: {record['inputsUnchanged']}")
    print(f"policy.ts {ts_digest}\nconfig.rhai {rhai_digest}")
    return 0 if failures == 0 and record["inputsUnchanged"] else 1


if __name__ == "__main__":
    sys.exit(main())

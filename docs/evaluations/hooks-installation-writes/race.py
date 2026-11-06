"""Race one outside write of settings.json against one Claude Code command.

Usage: python3 -I race.py TEMPLATE HOME METHOD STEP_MS MAX_MS OUT -- CMD...

For each delay from 0 to MAX_MS in steps of STEP_MS, the run restores HOME
from TEMPLATE and holds a hard link to HOME/.claude/settings.json. It starts
CMD with HOME and CLAUDE_CONFIG_DIR inside HOME. After the delay it adds the
key "probe" to settings.json by METHOD:

  rename   write a new file beside it and rename it onto the path
  inplace  truncate the existing file and write the new content into it

A kqueue watch records each change of the inode at the path. Each line of
OUT is JSON: the delay, the exit code, the time of the outside write, the
time and inode of each change, the inode of the outside write, and where the
probe key ended: "live", "held", "both" or "lost".
"""
import json
import os
import select
import shutil
import subprocess
import sys
import threading
import time

sep = sys.argv.index("--")
template, home, method, step_ms, max_ms, outp = sys.argv[1:sep]
cmd = sys.argv[sep + 1:]
step_ms, max_ms = float(step_ms), float(max_ms)
assert method in ("rename", "inplace"), method
cdir = os.path.join(home, ".claude")
path = os.path.join(cdir, "settings.json")
env = {k: v for k, v in os.environ.items() if k not in (
    "CLAUDE_CODE_PLUGIN_CACHE_DIR", "CLAUDE_CODE_PLUGIN_DIRS",
    "CLAUDE_CODE_PLUGIN_SEED_DIR", "CLAUDE_SECURESTORAGE_CONFIG_DIR")}
env["HOME"] = home
env["CLAUDE_CONFIG_DIR"] = cdir


def watch(stop, events, t0box):
    fd = os.open(cdir, os.O_RDONLY)
    kq = select.kqueue()
    kq.control([select.kevent(fd, filter=select.KQ_FILTER_VNODE,
                              flags=select.KQ_EV_ADD | select.KQ_EV_CLEAR,
                              fflags=select.KQ_NOTE_WRITE)], 0)
    last = os.stat(path).st_ino
    while not stop.is_set():
        if not kq.control(None, 4, 0.02):
            continue
        t = time.monotonic_ns()
        try:
            ino = os.stat(path).st_ino
        except FileNotFoundError:
            continue
        if ino != last:
            events.append(((t - t0box[0]) / 1e6, ino))
            last = ino
    os.close(fd)


def has_probe(p):
    try:
        return "probe" in json.load(open(p))
    except (OSError, ValueError):
        return None


out = open(outp, "a", buffering=1)
steps = int(round(max_ms / step_ms))
for n in range(steps + 1):
    delay = n * step_ms
    shutil.rmtree(home, ignore_errors=True)
    shutil.copytree(template, home, symlinks=True)
    held = os.path.join(home, "held")
    os.link(path, held)
    stop, events, t0box = threading.Event(), [], [0]
    th = threading.Thread(target=watch, args=(stop, events, t0box))
    th.start()
    time.sleep(0.05)
    t0box[0] = time.monotonic_ns()
    proc = subprocess.Popen(cmd, env=env, cwd=os.path.dirname(home),
                            stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
    target = t0box[0] + int(delay * 1e6)
    while time.monotonic_ns() < target:
        pass
    data = json.load(open(path))
    data["probe"] = delay
    body = json.dumps(data, indent=2).encode()
    if method == "rename":
        tmp = path + ".outside"
        with open(tmp, "wb") as f:
            f.write(body)
        mine = os.stat(tmp).st_ino
        tw = time.monotonic_ns()
        os.rename(tmp, path)
    else:
        fd = os.open(path, os.O_WRONLY | os.O_TRUNC)
        mine = os.fstat(fd).st_ino
        tw = time.monotonic_ns()
        os.write(fd, body)
        os.close(fd)
    rc = proc.wait()
    time.sleep(0.05)
    stop.set()
    th.join()
    live, kept = has_probe(path), has_probe(held)
    where = {(True, True): "both", (True, False): "live", (False, True): "held",
             (False, False): "lost"}.get((bool(live), bool(kept)), "error")
    out.write(json.dumps({
        "delay": delay, "rc": rc, "write": round((tw - t0box[0]) / 1e6, 3),
        "outside_inode": mine,
        "inode_changes": [(round(t, 3), i) for t, i in events],
        "probe": where}) + "\n")

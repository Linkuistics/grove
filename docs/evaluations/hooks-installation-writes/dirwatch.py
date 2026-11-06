"""Log each change of a directory's entries until SIGTERM.

Usage: python3 -I dirwatch.py DIR LOG

A kqueue watch on DIR wakes the loop. Each line of LOG is the monotonic
time in ns, then the entries that appeared, disappeared or changed inode
since the previous line. Two changes inside one wake read as one line, so
the log shows each change that it sees, not each change that happened.
"""
import os
import select
import signal
import sys
import time

directory, log = sys.argv[1], sys.argv[2]
out = open(log, "w", buffering=1)
stop = False


def term(*_):
    global stop
    stop = True


signal.signal(signal.SIGTERM, term)
fd = os.open(directory, os.O_RDONLY)
kq = select.kqueue()
kq.control([select.kevent(fd, filter=select.KQ_FILTER_VNODE,
                          flags=select.KQ_EV_ADD | select.KQ_EV_CLEAR,
                          fflags=select.KQ_NOTE_WRITE)], 0)


def snapshot():
    names = {}
    for entry in os.scandir(directory):
        try:
            names[entry.name] = entry.inode()
        except OSError:
            pass
    return names


prev = snapshot()
out.write(f"{time.monotonic_ns()} start {sorted(prev.items())}\n")
while not stop:
    if not kq.control(None, 4, 0.05):
        continue
    cur = snapshot()
    added = {k: v for k, v in cur.items() if k not in prev}
    gone = [k for k in prev if k not in cur]
    changed = {k: (prev[k], cur[k]) for k in cur if k in prev and prev[k] != cur[k]}
    out.write(f"{time.monotonic_ns()} dir added={added} gone={gone} changed={changed}\n")
    prev = cur
out.write(f"{time.monotonic_ns()} end\n")

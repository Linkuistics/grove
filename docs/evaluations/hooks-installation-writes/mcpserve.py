"""Drive an MCP stdio server for a while and report changes to one file.

Usage: python3 -I mcpserve.py HOME FILE SECONDS -- CMD...

The server runs with PATH=/usr/bin:/bin and HOME only. The script sends
initialize, the initialized notification, tools/list and one tools/call of
list_projects. It keeps the server open for SECONDS, then closes stdin. It
prints the inode, size, modification time and SHA-256 prefix of FILE before
and after, and "changed" or "unchanged".
"""
import hashlib
import json
import os
import subprocess
import sys
import time

sep = sys.argv.index("--")
home, fpath, secs = sys.argv[1], sys.argv[2], float(sys.argv[3])
cmd = sys.argv[sep + 1:]


def ident():
    st = os.stat(fpath)
    return (st.st_ino, st.st_size, st.st_mtime_ns,
            hashlib.sha256(open(fpath, "rb").read()).hexdigest()[:12])


before = ident()
proc = subprocess.Popen(cmd, env={"PATH": "/usr/bin:/bin", "HOME": home},
                        cwd=home, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                        stderr=subprocess.DEVNULL)


def send(obj):
    proc.stdin.write((json.dumps(obj) + "\n").encode())
    proc.stdin.flush()


send({"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {
    "protocolVersion": "2025-06-18", "capabilities": {},
    "clientInfo": {"name": "measure", "version": "0"}}})
print("initialize:", proc.stdout.readline().decode()[:100])
send({"jsonrpc": "2.0", "method": "notifications/initialized"})
send({"jsonrpc": "2.0", "id": 2, "method": "tools/list"})
print("tools/list bytes:", len(proc.stdout.readline()))
send({"jsonrpc": "2.0", "id": 3, "method": "tools/call",
      "params": {"name": "list_projects", "arguments": {}}})
print("list_projects:", proc.stdout.readline().decode()[:100])
time.sleep(secs)
proc.stdin.close()
try:
    proc.wait(10)
except subprocess.TimeoutExpired:
    proc.kill()
after = ident()
print("before", before)
print("after ", after)
print("changed" if before != after else "unchanged")

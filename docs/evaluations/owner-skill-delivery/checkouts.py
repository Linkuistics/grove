"""Measure mise's [bootstrap.repos] part as the owner of pack checkouts.

The script builds a fixture origin repository and a fixture HOME in a
temporary directory. It runs the installed mise against that HOME only, with an
empty environment apart from the isolated paths. It moves the origin away to
stand in for a machine without a network. Each case compares one observed
state, exit status or checked-out commit with its expected value.

The script writes only inside its temporary directory. It reads the installed
mise and git executables. The fixture hides the owner's global and system Git
configuration, so a global ignore file, an LFS filter or a credential helper of
the owner takes no part in a case. Each mise call names the fixture HOME with
-C, as mac-bootstrap does for the real HOME.

Usage:
    python3 -I checkouts.py [--mutate]

Exit status 0 means that every case matched. --mutate expects the opposite
outcome for the offline advance to a commit that the clone already holds. The
run must then fail, which shows that the comparison can fail.
"""
import json, os, shutil, subprocess, sys, tempfile

MUTATE = sys.argv[1:2] == ["--mutate"]
MISE = shutil.which("mise") or sys.exit("mise is not on PATH")
GIT = shutil.which("git") or sys.exit("git is not on PATH")
IDENT = {"GIT_AUTHOR_NAME": "fixture", "GIT_AUTHOR_EMAIL": "fixture@example.invalid",
         "GIT_COMMITTER_NAME": "fixture", "GIT_COMMITTER_EMAIL": "fixture@example.invalid"}

root = tempfile.mkdtemp(prefix="pack-checkouts-")
home, src, origin = (os.path.join(root, n) for n in ("home", "origin-src", "origin.git"))
away = origin + ".away"
url = "file://" + origin
pinned, follow = "~/.local/share/skill-packs/demo", "~/.local/share/skill-packs/follow"
checkout = os.path.join(home, ".local/share/skill-packs/demo")
followed = os.path.join(home, ".local/share/skill-packs/follow")
env = {"PATH": os.pathsep.join([os.path.dirname(GIT), os.path.dirname(MISE), "/usr/bin", "/bin"]),
       "HOME": home, "XDG_CONFIG_HOME": f"{home}/.config", "XDG_DATA_HOME": f"{home}/.local/share",
       "XDG_STATE_HOME": f"{home}/.local/state", "XDG_CACHE_HOME": f"{home}/.cache",
       "MISE_CONFIG_DIR": f"{home}/.config/mise", "MISE_DATA_DIR": f"{home}/.local/share/mise",
       "MISE_STATE_DIR": f"{home}/.local/state/mise", "MISE_CACHE_DIR": f"{home}/.cache/mise",
       "GIT_CONFIG_NOSYSTEM": "1", "GIT_CONFIG_GLOBAL": os.devnull, **IDENT}

def run(*args, cwd=None):
    return subprocess.run(args, cwd=cwd or home, env=env, capture_output=True, text=True)
def git(repo, *args): return run(GIT, "-C", repo, *args).stdout.strip()
def commit(text, message):
    with open(os.path.join(src, "skills/demo/SKILL.md"), "w") as fh: fh.write(text)
    run(GIT, "-C", src, "add", "-A"); run(GIT, "-C", src, "commit", "-qm", message)
    return git(src, "rev-parse", "HEAD")
def configure(entries):
    lines = ["[bootstrap.repos]"] + [
        f'"{path}" = {{ url = "{url}"' + (f', ref = "{ref}"' if ref else "") + " }" for path, ref in entries]
    with open(os.path.join(home, ".config/mise/config.toml"), "w") as fh: fh.write("\n".join(lines) + "\n")
def mise(*args): return run(MISE, "-C", home, *args)
def state(path):
    out = json.loads(mise("bootstrap", "repos", "status", "--json").stdout)
    return next(r["state"] for r in out["repos"] if r["path"] == path)
def missing_exit(): return mise("bootstrap", "repos", "status", "--missing").returncode
def apply(): return mise("bootstrap", "repos", "apply", "-y").returncode
def apply_fetches():
    done = mise("bootstrap", "repos", "apply", "-y", "--verbose")
    return done.returncode, '"fetch"' in done.stdout + done.stderr
names = {}
def head(repo): return names.get(git(repo, "rev-parse", "HEAD"), "unknown")
def detached(repo): return run(GIT, "-C", repo, "symbolic-ref", "-q", "HEAD").returncode != 0
def offline(): os.rename(origin, away)
def online(): os.rename(away, origin)

results = []
def case(name, observed, expected):
    results.append((name, observed, expected, observed == expected))

os.makedirs(os.path.join(src, "skills/demo")); os.makedirs(os.path.join(home, ".config/mise"))
run(GIT, "init", "-q", "-b", "main", src)
with open(os.path.join(src, ".gitignore"), "w") as fh: fh.write("__pycache__/\n")
A = commit("one\n", "A"); B = commit("two\n", "B"); names.update({A: "A", B: "B"})
run(GIT, "clone", "-q", "--bare", src, origin)

configure([(pinned, A)])
case("before install: state", state(checkout), "missing")
case("before install: status --missing exit", missing_exit(), 1)
case("before install: bootstrap plan exit", mise("bootstrap", "plan", "--detailed-exitcode").returncode, 0)
case("install at A: apply exit", apply(), 0)
case("install at A: checkout commit", head(checkout), "A")
case("install at A: detached HEAD", detached(checkout), True)
case("install at A: state", state(checkout), "current")
case("install at A: status --missing exit", missing_exit(), 0)

C = commit("three\n", "C"); names[C] = "C"; run(GIT, "-C", src, "push", "-q", origin, "main")
offline()
case("offline, ref unchanged: state", state(checkout), "current")
case("offline, ref unchanged: apply exit and fetch", apply_fetches(), (0, False))
configure([(pinned, B)])
case("offline, ref B held locally: state", state(checkout), "differs")
case("offline, ref B held locally: status --missing exit", missing_exit(), 1)
case("offline, ref B held locally: apply exit and fetch", apply_fetches(), (0, False) if MUTATE else (1, True))
case("offline, ref B held locally: checkout commit", head(checkout), "A")
configure([(pinned, C)])
case("offline, ref C not fetched: state", state(checkout), "differs")
case("offline, ref C not fetched: apply exit and fetch", apply_fetches(), (1, True))
case("offline, ref C not fetched: checkout commit", head(checkout), "A")
online()
case("online, ref C: apply exit", apply(), 0)
case("online, ref C: checkout commit", head(checkout), "C")

stray = os.path.join(checkout, "untracked.txt")
with open(stray, "w") as fh: fh.write("edit\n")
case("untracked file: state", state(checkout), "dirty")
case("untracked file: status --missing exit", missing_exit(), 1)
case("untracked file: apply exit", apply(), 1)
os.remove(stray)
ignored = os.path.join(checkout, "__pycache__")
os.makedirs(ignored)
with open(os.path.join(ignored, "a.pyc"), "w") as fh: fh.write("bytes\n")
case("ignored file: state", state(checkout), "current")
case("ignored file: status --missing exit", missing_exit(), 0)
case("ignored file: git status --porcelain --ignored lists it",
     git(checkout, "status", "--porcelain", "--ignored"), "!! __pycache__/")
shutil.rmtree(ignored)
run(GIT, "-C", checkout, "remote", "set-url", "origin", "https://example.invalid/other.git")
case("other origin: state", state(checkout), "conflict")
case("other origin: status --missing exit", missing_exit(), 1)
case("other origin: apply exit", apply(), 1)
run(GIT, "-C", checkout, "remote", "set-url", "origin", url)
case("restored: state", state(checkout), "current")

configure([(pinned, C), (follow, None)])
case("no ref: apply exit", apply(), 0)
D = commit("four\n", "D"); names[D] = "D"; run(GIT, "-C", src, "push", "-q", origin, "main")
case("no ref, remote ahead: state", state(followed), "current")
case("no ref, remote ahead: update exit", mise("bootstrap", "repos", "update", "-y").returncode, 0)
case("no ref, after update: checkout commit", head(followed), "D")

print("mise\t" + run(MISE, "--version").stdout.strip())
print("git\t" + run(GIT, "--version").stdout.strip())
for name, observed, expected, ok in results:
    print(f"{'ok' if ok else 'FAIL'}\t{name}\tobserved={observed}\texpected={expected}")
shutil.rmtree(root)
sys.exit(0 if all(r[3] for r in results) else 1)

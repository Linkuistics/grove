#!/usr/bin/env python3
"""Dry-run the owner switch plan in a disposable copy of HOME, and compare its inputs with the real ones.

docs/specs/owner-skill-delivery.md, "The rehearsed plan", owns the rules;
"Test seams" lists the controls, which scripts/owner_switch_rehearsal.py
runs. The switch never reads a dry-run result: the operator reads the
comparison before each real attempt ("The switch consumes prepared values,
not a passing token").

Usage:
    python3 -I owner_switch_dry_run.py run     --out OUT [--home HOME] [--state REL] [--include REL]...
                                               [--path DIR]... [--tool NAME]... [--prefix NAME=DIR]...
                                               [--guidance PATH]... [--python PATH]
    python3 -I owner_switch_dry_run.py inputs  [--home HOME] [--state REL] [--path DIR]... [--tool NAME]...
                                               [--python PATH] [--json]
    python3 -I owner_switch_dry_run.py compare --result RESULT [--home HOME] [--prefix NAME=DIR]...
                                               [--path DIR]... [--python PATH] [--out FILE]

HOME defaults to the account's home directory and STATE to
.local/state/owner-switch. The prepared plan and program are
HOME/STATE/prepared/plan.json and HOME/STATE/prepared/owner_switch.py, as
for the switch and `mac-check`. Each --path folder comes first on PATH, for
the program, its commands and the version commands.

run. OUT is a new or empty folder, and the disposable HOME is OUT/home. The
source HOME is only read.
    1. Copy the relevant layout from HOME into OUT/home with `cp -pPR`, which
       keeps content, mode, ACL, extended attributes and link text: the
       plan's writes, restore paths, inventory items and roots and manual,
       STATE/prepared, each declared path field under HOME, each script and
       helper of the input table under HOME, and each --include. No file
       content changes, so an absolute path inside a copied file still names
       HOME. That limit is in the result.
    2. Write the executed plan to OUT/home/STATE/prepared/plan.json. It
       differs from the prepared plan only in its declared path fields, where
       the prefix HOME becomes OUT/home. The declared path fields are each
       `inputs` path, each argument of each command, accepted.staging and
       each `retained` path. A prefix matches a whole value or the part
       before a "/". No other text is replaced.
    3. Digest each subject: the input table below.
    4. Run, with HOME set to OUT/home: `native-off` without any backup,
       `attempt`, `switch`, and the attempt note's literal restore command.
       Before the attempt and after restore, record the restore identity of
       each restore path and the change identity of each inventory item.
    5. Digest each subject again. A changed subject invalidates the run.
       Compare each copied path of HOME and the account's
       ~/Library/LaunchAgents with their identities before the run.
    6. Write OUT/result.json, OUT/run.log and OUT/summary.txt.
Exit status: 0 when every step passed, the subjects stayed the same and HOME
stayed the same; 1 otherwise; 2 for a usage error; 3 when the run could not
start.

The input table. Each row names a logical input, its role, the path that ran
(canonical), its content digest and, for tools and the interpreter, its
version line. The logical names come from the plan, so a row keeps its name
when its path moves:
    plan                    the prepared plan; "normalised" digests it with
                            each declared prefix replaced by its name
    program                 the prepared program
    interpreter:$python     the interpreter that runs the program (--python)
    input:<role>            each item of the plan's `inputs` except $program
    command:<field>/<name>:<i>
                            argument i of a command, when it names a file:
                            relative to HOME, absolute, or on PATH for i = 0
    interpreter:<row>       the interpreter of a script's "#!" line
    helper:<row> <token>    a module that a Python script imports and that
                            exists beside it or in a literal sys.path folder;
                            or an absolute or $HOME path token of a shell
                            script that names an executable or a script.
                            Helpers are found again inside helpers.
    tool:<name>             claude, brew and mise on PATH; ravel and grove at
                            $(brew --prefix)/bin/ravel and
                            $(brew --prefix)/opt/grove/bin/grove, as the
                            program finds them; and each --tool on PATH
    source:<id>             $(brew --prefix)/share/ravel/skills/<id> for each
                            accepted source, with a tree digest
A file digest is the SHA-256 of the content. A tree digest frames each
relative path, type, executable bit, file content and link text, and follows
no link inside the tree. Each version command runs with HOME, its
OWNER_SWITCH_HOME and the environment rule of the program's commands. Python
string literals are not read, because they name configuration that the run
changes.

Declared substitutions. `home` maps OUT/home to the real HOME. `grove-keg`
maps the candidate Grove keg to the released keg, and `ravel` maps ravel@2 to
the promoted Ravel. `run --prefix NAME=DIR` declares a dry-run prefix, and
`compare --prefix NAME=DIR` gives the real one. No other name is accepted.

compare. Read RESULT, enumerate the real prepared inputs of HOME through the
same table, and report each row: "equal"; "substituted <name>", when only a
declared prefix differs and the content and version are equal; or "differs",
with each reason. A path that differs by anything other than a declared
prefix is an unknown substitution. The plan compares by its normalised digest,
and each differing plan value is listed. compare writes nothing except
--out. Exit status: 0 when the result passed and each row is equal or
substituted; 1 otherwise; 2 for a usage error; 3 when RESULT or the plan
cannot be read.

inputs prints the input table of HOME and writes nothing.

The fixture suite alone passes --fixture-hook and --fixture-scope, which set
OWNER_SWITCH_REHEARSAL_HOOK and OWNER_SWITCH_REHEARSAL_SCOPE for the program.
The program refuses both when HOME is the account's home directory. A result
of a run with either, or with --fixture, says that it is a fixture result.
A fixture result claims no real-harness acceptance.
"""
from __future__ import annotations

import argparse
import ast
import hashlib
import json
import os
import pwd
import re
import shlex
import shutil
import stat
import subprocess
import sys
import time

RESULT_SCHEMA = "owner-switch-dry-run/1"
COMPARISON_SCHEMA = "owner-switch-dry-run-comparison/1"
DEFAULT_STATE = ".local/state/owner-switch"
SUBSTITUTIONS = ("home", "grove-keg", "ravel")
DEFAULT_TOOLS = ("claude", "brew", "mise", "ravel", "grove")
SELF = os.path.realpath(__file__)

# The environment rule of the program's commands (scripts/owner_switch.py, command_env).
LOOP_CONTROL = (
    "GROVE_SIGNAL_FILE", "GROVE_RUN_SIGNAL_FILE", "GROVE_LAUNCH_DIR",
    "HARNESS_DISPATCH_EXIT_FILE", "HARNESS_DISPATCH_RUN_ID",
    "HARNESS_DISPATCH_STATE_DIR", "RAVEL_EXIT_FILE", "GROVE_PROVIDER_EVIDENCE",
    "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID",
)
CLAUDE_LOCATION_VARS = (
    "CLAUDE_CONFIG_DIR", "CLAUDE_SECURESTORAGE_CONFIG_DIR", "CLAUDE_CODE_PLUGIN_CACHE_DIR",
    "CLAUDE_CODE_PLUGIN_DIRS", "CLAUDE_CODE_PLUGIN_SEED_DIR",
)
HOOK_VAR = "OWNER_SWITCH_REHEARSAL_HOOK"
SCOPE_VAR = "OWNER_SWITCH_REHEARSAL_SCOPE"
# The program sets these itself; an inherited value must not reach it.
PROGRAM_VARS = (HOOK_VAR, SCOPE_VAR, "OWNER_SWITCH_PLAN", "OWNER_SWITCH_PLAN_DIGEST", "OWNER_SWITCH_ADMITTED",
                "OWNER_SWITCH_HOME", "OWNER_SWITCH_STATE", "OWNER_SWITCH_STEP")
# The inventory comparison skips these platform subtrees while each is a plain folder.
PLATFORM_SUBTREES = (".claude/skills/synced", ".codex/skills/.system")
SCRIPT_SUFFIXES = (".py", ".sh", ".bash", ".zsh", ".js", ".mjs", ".cjs", ".rb", ".pl")
SHELLS = ("sh", "bash", "zsh", "dash", "ksh")
# A path token of a shell script: absolute, or under $HOME, ${HOME} or ~.
SHELL_TOKEN = re.compile(r"(?:\$HOME|\$\{HOME\}|~)(/[A-Za-z0-9._@+/-]+)|(?<![\w$}~./-])(/[A-Za-z0-9._@+-][A-Za-z0-9._@+/-]*)")
RESULT_MARK = "owner-switch-result: "
COMMAND_LINE = re.compile(r"^command (\S+) \(step (7|restore)\): exit (-?\d+)$", re.M)


class Refused(Exception):
    """The instrument cannot start or cannot read its inputs."""


def log(line: str = "") -> None:
    print(line, flush=True)


def account_home() -> str:
    return os.path.realpath(pwd.getpwuid(os.getuid()).pw_dir)


def canon(value) -> str:
    return json.dumps(value, sort_keys=True, separators=(",", ":"))


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _frame(h, tag: bytes, data: bytes) -> None:
    h.update(b"%d:%s %d:" % (len(tag), tag, len(data)))
    h.update(data)


def file_digest(path: str):
    try:
        with open(path, "rb") as f:
            return sha256(f.read())
    except OSError:
        return None


def tree_digest(path: str):
    """The digest of a folder: each relative path, type, executable bit, content and link text."""
    if not os.path.isdir(path):
        return None
    h = hashlib.sha256()

    def walk(p: str, rel: str) -> None:
        st = os.lstat(p)
        if stat.S_ISLNK(st.st_mode):
            _frame(h, b"link " + rel.encode(), os.fsencode(os.readlink(p)))
        elif stat.S_ISDIR(st.st_mode):
            _frame(h, b"dir", rel.encode())
            for name in sorted(os.listdir(p)):
                walk(os.path.join(p, name), f"{rel}/{name}")
        elif stat.S_ISREG(st.st_mode):
            x = b"x" if st.st_mode & 0o111 else b"-"
            with open(p, "rb") as f:
                _frame(h, b"file " + x + b" " + rel.encode(), f.read())
        else:
            _frame(h, b"other", rel.encode())

    walk(os.path.realpath(path), ".")
    return h.hexdigest()


def content_digest(path):
    """A file's digest, a folder's tree digest, or None when nothing readable is there."""
    if not path:
        return None
    if os.path.isdir(path):
        return tree_digest(path)
    return file_digest(path)


def entry_identity(path: str):
    """The identity of an entry of HOME for the before/after check: tree, mode and link text, no link followed."""
    if not os.path.lexists(path):
        return "absent"
    h = hashlib.sha256()

    def walk(p: str, rel: str) -> None:
        st = os.lstat(p)
        _frame(h, b"mode " + rel.encode(), b"%o" % st.st_mode)
        if stat.S_ISLNK(st.st_mode):
            _frame(h, b"link", os.fsencode(os.readlink(p)))
        elif stat.S_ISDIR(st.st_mode):
            for name in sorted(os.listdir(p)):
                walk(os.path.join(p, name), f"{rel}/{name}")
        elif stat.S_ISREG(st.st_mode):
            with open(p, "rb") as f:
                _frame(h, b"content", f.read())

    try:
        walk(path, ".")
    except OSError as e:
        return f"unreadable: {e.strerror}"
    return h.hexdigest()


# --- prefixes and the declared path fields ------------------------------------------

def under(path: str, prefix: str) -> bool:
    return path == prefix or path.startswith(prefix.rstrip("/") + "/")


def replace_prefix(value: str, old: str, new: str) -> str:
    return new + value[len(old):] if under(value, old) else value


def normalise_path(path, prefixes: dict):
    """PATH with its longest declared prefix replaced by ${name}, or unchanged."""
    if not isinstance(path, str):
        return path
    best = None
    for name, prefix in prefixes.items():
        if prefix and under(path, prefix) and (best is None or len(prefix) > len(prefixes[best])):
            best = name
    return path if best is None else "${" + best + "}" + path[len(prefixes[best]):]


def commands_of(plan: dict) -> list:
    """Each command of the plan with its field name: (field, command)."""
    out = []
    if isinstance(plan.get("baseline"), dict):
        out.append(("baseline", plan["baseline"]))
    for step, commands in sorted((plan.get("gates") or {}).items()):
        out += [(f"gates.{step}", c) for c in commands or []]
    for sid, commands in sorted(((plan.get("accepted") or {}).get("trials") or {}).items()):
        out += [(f"accepted.trials.{sid}", c) for c in commands or []]
    for field in ("probes", "native_probe"):
        out += [(field, c) for c in plan.get(field) or []]
    if isinstance(plan.get("mac_check"), dict):
        out.append(("mac_check", plan["mac_check"]))
    return [(f, c) for f, c in out if isinstance(c, dict)]


def map_path_fields(plan: dict, fn) -> dict:
    """A copy of PLAN with FN applied to each declared path field, and to nothing else."""
    plan = json.loads(json.dumps(plan))
    for row in plan.get("inputs") or []:
        if isinstance(row, dict) and isinstance(row.get("path"), str) and row["path"] != "$program":
            row["path"] = fn(row["path"])
    for _, command in commands_of(plan):
        if isinstance(command.get("argv"), list):
            command["argv"] = [fn(a) if isinstance(a, str) else a for a in command["argv"]]
    staging = (plan.get("accepted") or {}).get("staging")
    if isinstance(staging, dict):
        for key in list(staging):
            if isinstance(staging[key], str):
                staging[key] = fn(staging[key])
    for row in plan.get("retained") or []:
        if isinstance(row, dict) and isinstance(row.get("path"), str):
            row["path"] = fn(row["path"])
    return plan


def normalised_plan(plan: dict, prefixes: dict) -> dict:
    return map_path_fields(plan, lambda p: normalise_path(p, prefixes))


def plan_differences(a, b, at: str = "") -> list:
    """Each JSON pointer at which two normalised plans differ."""
    if isinstance(a, dict) and isinstance(b, dict):
        out = []
        for k in sorted(set(a) | set(b)):
            out += plan_differences(a.get(k, "$missing"), b.get(k, "$missing"), f"{at}/{k}")
        return out
    if isinstance(a, list) and isinstance(b, list) and len(a) == len(b):
        out = []
        for i, (x, y) in enumerate(zip(a, b)):
            out += plan_differences(x, y, f"{at}/{i}")
        return out
    return [] if canon(a) == canon(b) else [at or "/"]


# --- the environment ------------------------------------------------------------------

def base_env(paths: list, extra=None) -> dict:
    """The environment of the program and the version commands: no other subject than HOME."""
    env = {k: v for k, v in os.environ.items()
           if k not in LOOP_CONTROL and k not in CLAUDE_LOCATION_VARS and k != "CODEX_HOME"
           and not k.startswith("RAVEL_") and k not in PROGRAM_VARS}
    env["PATH"] = os.pathsep.join([os.path.realpath(p) for p in paths] + [env.get("PATH", "")])
    env.update(extra or {})
    return env


def tool_env(home: str, state: str, paths: list) -> dict:
    return base_env(paths, {"HOME": home, "OWNER_SWITCH_HOME": home,
                            "OWNER_SWITCH_STATE": os.path.join(home, state), "OWNER_SWITCH_STEP": "inputs"})


def first_line(argv: list, env: dict, cwd: str):
    """The first line of a version command's output, or an "exit N" note."""
    try:
        r = subprocess.run(argv, env=env, cwd=cwd, capture_output=True, text=True, timeout=120)
    except (OSError, subprocess.TimeoutExpired) as e:
        return f"cannot run: {e}"
    line = (r.stdout.strip() or r.stderr.strip()).splitlines()
    text = line[0] if line else ""
    return text if r.returncode == 0 else f"exit {r.returncode}: {text}"


# --- the input table ----------------------------------------------------------------------

class Table:
    """The input table of one HOME, enumerated through one rule for the dry run and the comparison."""

    def __init__(self, home: str, state: str, paths: list, tools: list, python: str, prefixes: dict):
        self.home = os.path.realpath(home)
        self.state = state
        self.paths = paths
        self.tools = tools
        self.python = python
        self.prefixes = prefixes
        self.env = tool_env(self.home, state, paths)
        self.rows = []
        self.seen = set()

    def prepared(self, name: str) -> str:
        return os.path.join(self.home, self.state, "prepared", name)

    def add(self, logical: str, role: str, path, digest=None, version=None, kind=None, **extra) -> dict:
        real = os.path.realpath(path) if path else None
        row = {"logical": logical, "role": role, "path": real,
               "digest": digest if digest is not None or not real else content_digest(real)}
        if version is not None:
            row["version"] = version
        row.update(extra)
        self.rows.append(row)
        if real and kind and (logical, real) not in self.seen:
            self.seen.add((logical, real))
            self.helpers(logical, real, kind)
        return row

    def which(self, name: str):
        return shutil.which(name, path=self.env["PATH"])

    def resolve_argument(self, value: str, first: bool):
        """The file that a command argument names, or None. Commands run in HOME."""
        if not value or value.startswith("-"):
            return None
        if os.path.isabs(value):
            path = value
        elif first and "/" not in value:
            path = self.which(value)
        else:
            path = os.path.join(self.home, value)
        return path if path and os.path.isfile(path) else None

    def script_kind(self, path: str):
        """"python", "shell" or None, from the suffix or the "#!" line."""
        interpreter = self.shebang(path)
        name = os.path.basename(interpreter[-1]) if interpreter else ""
        if path.endswith(".py") or name.startswith("python"):
            return "python"
        if path.endswith((".sh", ".bash", ".zsh")) or name in SHELLS:
            return "shell"
        return None

    @staticmethod
    def shebang(path: str):
        """The words of a script's "#!" line, with `env` resolved to its program, or None."""
        try:
            with open(path, "rb") as f:
                head = f.readline(512)
        except OSError:
            return None
        if not head.startswith(b"#!"):
            return None
        words = head[2:].decode(errors="replace").split()
        if words and os.path.basename(words[0]) == "env":
            words = [w for w in words[1:] if not w.startswith("-")][:1]
        return words or None

    def helpers(self, parent: str, path: str, kind) -> None:
        """Each interpreter and helper of the script at PATH, found again inside each helper."""
        kind = kind if kind in ("python", "shell") else self.script_kind(path)
        words = self.shebang(path)
        if words:
            interp = words[0] if os.path.isabs(words[0]) else self.which(words[0])
            self.add(f"interpreter:{parent}", "script interpreter", interp)
        if kind == "python":
            for token, found in self.python_helpers(path):
                self.add(f"helper:{parent} {token}", "imported local helper", found, kind="python")
        elif kind == "shell":
            for token, found in self.shell_helpers(path):
                sub = self.script_kind(found) or ("python" if found.endswith(".py") else None)
                role = "referenced script" if sub or found.endswith(SCRIPT_SUFFIXES) else "referenced executable"
                self.add(f"helper:{parent} {token}", role, found, kind=sub)

    @staticmethod
    def python_helpers(path: str) -> list:
        try:
            with open(path, "rb") as f:
                tree = ast.parse(f.read(), filename=path)
        except (OSError, SyntaxError, ValueError):
            return []
        folder = os.path.dirname(os.path.realpath(path))
        folders = [folder]
        names = []
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                names += [a.name.split(".")[0] for a in node.names]
            elif isinstance(node, ast.ImportFrom):
                if node.level and not node.module:
                    names += [a.name for a in node.names]
                elif node.module:
                    names.append(node.module.split(".")[0])
            elif (isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)
                  and node.func.attr in ("insert", "append") and node.args):
                literal = node.args[-1]
                if isinstance(literal, ast.Constant) and isinstance(literal.value, str) and os.path.isabs(literal.value):
                    folders.append(literal.value)
        out = []
        for name in sorted(set(names)):
            for base in folders:
                for candidate in (os.path.join(base, name + ".py"), os.path.join(base, name, "__init__.py")):
                    if os.path.isfile(candidate) and os.path.realpath(candidate) != os.path.realpath(path):
                        out.append((name, candidate))
                        break
                else:
                    continue
                break
        return out

    def shell_helpers(self, path: str) -> list:
        try:
            with open(path, encoding="utf-8", errors="replace") as f:
                lines = f.read().splitlines()
        except OSError:
            return []
        out, seen = [], set()
        for i, line in enumerate(lines):
            if i == 0 and line.startswith("#!"):
                continue
            code = line.split("#", 1)[0] if not line.lstrip().startswith("#") else ""
            for m in SHELL_TOKEN.finditer(code):
                token = m.group(0)
                found = os.path.join(self.home, m.group(1).lstrip("/")) if m.group(1) else m.group(2)
                if token in seen or not os.path.isfile(found):
                    continue
                if not (os.access(found, os.X_OK) or found.endswith(SCRIPT_SUFFIXES)):
                    continue
                seen.add(token)
                out.append((token, found))
        return out

    def enumerate(self, plan: dict, plan_path: str, program_path: str) -> list:
        normal = normalised_plan(plan, self.prefixes)
        self.add("plan", "plan", plan_path, normalised=sha256(canon(normal).encode()))
        self.add("program", "program", program_path, kind="python")
        version = first_line([self.python, "-I", "-c", "import platform; print(platform.python_version())"],
                             self.env, self.home)
        self.add("interpreter:$python", "interpreter", self.python, version=version)
        roles = {}
        for row in plan.get("inputs") or []:
            if not isinstance(row, dict) or row.get("path") == "$program":
                continue
            role = str(row.get("role"))
            roles[role] = roles.get(role, 0) + 1
            logical = f"input:{role}" + (f"#{roles[role]}" if roles[role] > 1 else "")
            self.add(logical, role, row.get("path"), planned=row.get("digest"))
        for field, command in commands_of(plan):
            argv = command.get("argv") or []
            for i, value in enumerate(argv):
                if not isinstance(value, str) or value in ("$python", "$program"):
                    continue
                found = self.resolve_argument(value, i == 0)
                if not found:
                    continue
                python = i > 0 and argv[0] == "$python"
                kind = "python" if python else self.script_kind(found)
                role = "invoked script" if kind or found.endswith(SCRIPT_SUFFIXES) else "invoked executable"
                self.add(f"command:{field}/{command.get('name')}:{i}", role, found, kind=kind or "none")
        prefix = None
        brew = self.which("brew")
        if brew:
            r = first_line([brew, "--prefix"], self.env, self.home)
            prefix = r if r and not r.startswith(("exit ", "cannot run")) else None
        for name in self.tools:
            if name == "ravel":
                path = os.path.join(prefix, "bin", "ravel") if prefix else self.which(name)
            elif name == "grove":
                path = os.path.join(prefix, "opt", "grove", "bin", "grove") if prefix else self.which(name)
            else:
                path = self.which(name)
            path = path if path and os.path.exists(path) else None
            version = first_line([path, "--version"], self.env, self.home) if path else None
            self.add(f"tool:{name}", "external tool", path, version=version)
        for sid in sorted(((plan.get("accepted") or {}).get("sources") or {})):
            path = os.path.join(prefix, "share", "ravel", "skills", sid) if prefix else None
            path = path if path and os.path.exists(path) else None
            self.add(f"source:{sid}", "skill source", path)
        return self.rows


def load_json(path: str):
    try:
        with open(path, "rb") as f:
            return json.loads(f.read())
    except (OSError, ValueError) as e:
        raise Refused(f"{path} cannot be read as JSON: {e}")


def prepared_plan(home: str, state: str) -> tuple:
    plan_path = os.path.join(home, state, "prepared", "plan.json")
    program_path = os.path.join(home, state, "prepared", "owner_switch.py")
    plan = load_json(plan_path)
    if not isinstance(plan, dict):
        raise Refused(f"{plan_path} holds no JSON object")
    if plan.get("state") != state:
        raise Refused(f"{plan_path} names the state {plan.get('state')!r}, not {state!r}")
    if not os.path.isfile(program_path):
        raise Refused(f"the prepared program {program_path} is missing")
    return plan, plan_path, program_path


def check_relative(rel: str, what: str) -> str:
    norm = os.path.normpath(rel)
    if os.path.isabs(rel) or norm == ".." or norm.startswith("../") or norm == ".":
        raise Refused(f"{what} {rel!r} must be a path inside HOME")
    return norm


def parse_prefixes(items: list) -> dict:
    out = {}
    for item in items or []:
        name, sep, folder = item.partition("=")
        if not sep or not folder:
            raise SystemExit(f"--prefix {item!r} must be NAME=DIR")
        if name not in SUBSTITUTIONS or name == "home":
            raise SystemExit(f"--prefix {name!r} is not a declared substitution; only grove-keg and ravel can be "
                             f"given, and home is HOME")
        out[name] = os.path.realpath(folder)
    return out


# --- run --------------------------------------------------------------------------------

def plan_paths(plan: dict) -> list:
    """The paths relative to HOME that the plan names outside its path fields."""
    rels = [plan.get("manual")] + list(plan.get("restore") or [])
    rels += [w.get("path") for w in plan.get("writes") or [] if isinstance(w, dict)]
    for when in ("before", "after"):
        for item in (plan.get("inventory") or {}).get(when) or []:
            if isinstance(item, dict):
                rels.append(item.get("root") or item.get("path"))
    return [r for r in rels if isinstance(r, str) and r]


def copy_list(plan: dict, rows: list, home: str, state: str, includes: list) -> list:
    """Each path relative to HOME that the dry run copies, without one inside another."""
    rels = set(plan_paths(plan)) | {os.path.join(state, "prepared")} | set(includes)

    def collect(value):
        if isinstance(value, str) and os.path.isabs(value):
            real = os.path.realpath(value) if os.path.lexists(value) else os.path.normpath(value)
            if under(real, home) and real != home:
                rels.add(os.path.relpath(real, home))
            elif under(value, home) and value != home:
                rels.add(os.path.relpath(value, home))
        return value
    map_path_fields(plan, collect)
    for row in rows:
        # A tool, a source or the interpreter runs from its own path, so the copy needs none of them.
        if not row["logical"].startswith(("tool:", "source:", "interpreter:$python")):
            collect(row.get("path"))
    for _, command in commands_of(plan):
        for value in command.get("argv") or []:
            if isinstance(value, str) and not os.path.isabs(value) and value not in ("$python", "$program") \
                    and os.path.lexists(os.path.join(home, value)) and not value.startswith("-"):
                rels.add(os.path.normpath(value))
    keep = []
    for rel in sorted(rels, key=lambda r: (r.count("/"), r)):
        rel = check_relative(rel, "the plan path")
        if not any(under(rel, k) for k in keep):
            keep.append(rel)
    return sorted(keep)


def copy_into(src_home: str, dst_home: str, rel: str) -> bool:
    src = os.path.join(src_home, rel)
    if not os.path.lexists(src):
        return False
    dst = os.path.join(dst_home, rel)
    os.makedirs(os.path.dirname(dst), exist_ok=True)
    # cp -p keeps the mode, ACL, flags, owner and extended attributes, -P copies a link
    # as its text, and -c clones on APFS; a plain copy is the fallback.
    for flags in (["-c", "-pPR"], ["-pPR"]):
        if os.path.lexists(dst):
            shutil.rmtree(dst) if os.path.isdir(dst) and not os.path.islink(dst) else os.remove(dst)
        r = subprocess.run(["/bin/cp", *flags, src, dst], capture_output=True, text=True)
        if r.returncode == 0:
            return True
    raise Refused(f"cp -pPR {src} failed: {r.stderr.strip()}")


class Run:
    def __init__(self, args):
        self.args = args
        self.source = os.path.realpath(args.home)
        self.state = check_relative(args.state, "--state")
        self.out = os.path.realpath(args.out)
        self.home = os.path.join(self.out, "home")
        self.python = os.path.realpath(args.python)
        self.tools = list(dict.fromkeys(list(DEFAULT_TOOLS) + list(args.tool or [])))
        self.paths = args.path or []
        self.includes = [check_relative(r, "--include") for r in args.include or []]
        self.declared = parse_prefixes(args.prefix)
        self.fixture = bool(args.fixture or args.fixture_hook or args.fixture_scope)
        self.log = []
        self.steps = []

    def say(self, line: str = "") -> None:
        self.log.append(line)
        log(line)

    def env(self) -> dict:
        extra = {"HOME": self.home}
        if self.args.fixture_hook:
            extra[HOOK_VAR] = self.args.fixture_hook
        if self.args.fixture_scope:
            extra[SCOPE_VAR] = os.path.realpath(self.args.fixture_scope)
        return base_env(self.paths, extra)

    def program(self, *args: str) -> list:
        return [self.python, "-I", os.path.join(self.home, self.state, "prepared", "owner_switch.py"), *args]

    def step(self, name: str, argv: list, want=(0,)) -> dict:
        self.say(f"$ {shlex.join(argv)}")
        started = time.time()
        r = subprocess.run(argv, cwd=self.home, env=self.env(), capture_output=True, text=True)
        out = r.stdout + r.stderr
        for line in out.splitlines():
            self.say(f"  {line}")
        record = {"step": name, "argv": argv, "exit": r.returncode, "passed": r.returncode in want,
                  "seconds": round(time.time() - started, 2), "output": out[-20000:]}
        self.steps.append(record)
        self.say(f"  ({name}: exit {r.returncode}{'' if record['passed'] else ', not passed'})")
        return record

    def identity(self, command: str, path: str) -> str:
        r = subprocess.run(self.program(command, path), cwd=self.home, env=self.env(), capture_output=True, text=True)
        return r.stdout.strip() if r.returncode == 0 else f"exit {r.returncode}: {r.stderr.strip()[-200:]}"

    def layout(self, plan: dict) -> dict:
        """The restore identity of each restore path and the change identity of each inventory item."""
        out = {}
        for rel in plan.get("restore") or []:
            out["restore " + rel] = self.identity("restore-identity", os.path.join(self.home, rel))
        for item in (plan.get("inventory") or {}).get("before") or []:
            if "root" in item:
                root = os.path.join(self.home, item["root"])
                names = sorted(os.listdir(root)) if os.path.isdir(root) else []
                for name in names:
                    rel = f"{item['root']}/{name}"
                    p = os.path.join(self.home, rel)
                    if rel in PLATFORM_SUBTREES and os.path.isdir(p) and not os.path.islink(p):
                        continue
                    out["inventory " + rel] = self.identity("identity", p)
                if not names:
                    out["inventory " + item["root"]] = "empty" if os.path.isdir(root) else "absent"
            else:
                out["inventory " + item["path"]] = self.identity("identity", os.path.join(self.home, item["path"]))
        return out

    def watched(self, rels: list) -> dict:
        out = {f"~/{rel}": entry_identity(os.path.join(self.source, rel)) for rel in rels}
        agents = os.path.join(account_home(), "Library", "LaunchAgents")
        out["account ~/Library/LaunchAgents"] = entry_identity(agents)
        return out

    def table(self, home: str, plan: dict, plan_path: str, program_path: str) -> list:
        prefixes = dict(self.declared, home=home)
        return Table(home, self.state, self.paths, self.tools, self.python, prefixes).enumerate(
            plan, plan_path, program_path)

    def execute(self) -> int:
        if os.path.exists(self.out) and os.listdir(self.out):
            raise Refused(f"OUT {self.out} must be a new or empty folder")
        plan, plan_path, program_path = prepared_plan(self.source, self.state)
        for rel in plan_paths(plan) + [self.state] + self.includes:
            if under(self.out, os.path.join(self.source, rel)):
                raise Refused(f"OUT {self.out} lies inside the copied path ~/{rel}")
        os.makedirs(self.home)
        source_rows = self.table(self.source, plan, plan_path, program_path)
        rels = copy_list(plan, source_rows, self.source, self.state, self.includes)
        watched_before = self.watched(rels)
        copied = [rel for rel in rels if copy_into(self.source, self.home, rel)]
        self.say(f"copied {len(copied)} of {len(rels)} paths from {self.source} into {self.home}")
        dry_plan = map_path_fields(plan, lambda p: replace_prefix(p, self.source, self.home))
        dry_plan_path = os.path.join(self.home, self.state, "prepared", "plan.json")
        with open(dry_plan_path, "w") as f:
            json.dump(dry_plan, f, indent=2)
            f.write("\n")
        dry_program = os.path.join(self.home, self.state, "prepared", "owner_switch.py")
        before = self.table(self.home, dry_plan, dry_plan_path, dry_program)

        settings = os.path.join(self.home, ".config/ravel/settings.json")
        backups = os.path.join(self.home, self.state, "backups")
        native = {"backups_before": sorted(os.listdir(backups)) if os.path.isdir(backups) else [],
                  "settings_before": file_digest(settings)}
        off = self.step("native-off", self.program("native-off", "--home", self.home))
        native.update(settings_after=file_digest(settings), skills_after=read_skills(settings))
        layout_native = self.layout(dry_plan) if off["passed"] else {}
        note = {}
        if off["passed"]:
            attempt = self.step("attempt", self.program("attempt", "--plan", dry_plan_path, "--home", self.home))
            m = re.search(r"^attempt note: (.+)$", attempt["output"], re.M)
            note = load_json(m.group(1)) if attempt["passed"] and m else {}
            if m:
                note["_path"] = m.group(1)
        guidance = [x for g in self.args.guidance or [] for x in ("--guidance", os.path.realpath(g))]
        switch = restore = None
        if note:
            switch = self.step("switch", self.program("switch", "--plan", dry_plan_path, "--home", self.home,
                                                      "--attempt", note["_path"], *guidance))
        if switch and switch["passed"]:
            restore = self.step("restore", shlex.split(note["restore"]) + guidance)
        layout_restored = self.layout(dry_plan) if restore else {}
        restore_rows = [{"item": k, "before": v, "after": layout_restored.get(k, "$missing"),
                         "equal": v == layout_restored.get(k, "$missing")}
                        for k, v in sorted(layout_native.items())]
        restore_rows += [{"item": k, "before": "$missing", "after": v, "equal": False}
                         for k, v in sorted(layout_restored.items()) if k not in layout_native]

        after = self.table(self.home, dry_plan, dry_plan_path, dry_program)
        changed = stability(before, after)
        watched_after = self.watched(rels)
        moved = [k for k in sorted(set(watched_before) | set(watched_after))
                 if watched_before.get(k) != watched_after.get(k)]

        probes = probe_outcomes(dry_plan, note, switch, restore, self.home, self.state)
        steps_ok = bool(off["passed"] and note and switch and switch["passed"] and restore and restore["passed"])
        restore_ok = bool(restore_rows) and all(r["equal"] for r in restore_rows)
        passed = steps_ok and restore_ok and not changed and not moved
        result = {
            "schema": RESULT_SCHEMA,
            "passed": passed,
            "valid": not changed,
            "fixture": self.fixture,
            "at": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
            "instrument": {"path": SELF, "digest": file_digest(SELF)},
            "source_home": self.source, "dry_home": self.home, "state": self.state,
            "substitutions": dict(self.declared, home=self.home),
            "tools": self.tools, "python": self.python, "path": [os.path.realpath(p) for p in self.paths],
            "includes": self.includes, "copied": copied,
            "plan": {"source": plan_path, "executed": dry_plan_path,
                     "normalised": normalised_plan(dry_plan, dict(self.declared, home=self.home))},
            "inputs": before,
            "inputs_after": after,
            "changed": changed,
            "native_off": dict(native, exit=off["exit"]),
            "attempt": {k: v for k, v in note.items() if k in ("attempt", "_path", "backup", "restore")},
            "steps": self.steps,
            "probes": probes,
            "restore_comparison": {"equal": restore_ok, "rows": restore_rows},
            "source_unchanged": {"equal": not moved, "moved": moved, "before": watched_before,
                                 "after": watched_after},
            "undeclared": undeclared(before, dry_plan, self.home),
            "limits": limits(self.fixture, self.includes),
        }
        with open(os.path.join(self.out, "result.json"), "w") as f:
            json.dump(result, f, indent=2)
            f.write("\n")
        summary = summarise(result)
        with open(os.path.join(self.out, "summary.txt"), "w") as f:
            f.write(summary)
        with open(os.path.join(self.out, "run.log"), "w") as f:
            f.write("\n".join(self.log) + "\n")
        log(summary.rstrip())
        log(f"result: {os.path.join(self.out, 'result.json')}")
        return 0 if passed else 1


def read_skills(path: str):
    try:
        with open(path, "rb") as f:
            obj = json.loads(f.read())
    except (OSError, ValueError):
        return {"$unreadable": True}
    return obj.get("skills", {"$absent": True}) if isinstance(obj, dict) else {"$unreadable": True}


def stability(before: list, after: list) -> list:
    """Each subject whose path, digest or version changed during the run."""
    a = {r["logical"] + " " + str(r["path"]): r for r in before}
    b = {r["logical"] + " " + str(r["path"]): r for r in after}
    out = []
    for key in sorted(set(a) | set(b)):
        x, y = a.get(key), b.get(key)
        if x is None or y is None:
            out.append({"logical": (x or y)["logical"], "path": (x or y)["path"],
                        "change": "appeared" if x is None else "disappeared"})
        elif x.get("digest") != y.get("digest") or x.get("version") != y.get("version"):
            out.append({"logical": x["logical"], "path": x["path"], "change": "content or version",
                        "before": x.get("digest"), "after": y.get("digest")})
    return out


def probe_outcomes(plan: dict, note: dict, switch, restore, home: str, state: str) -> list:
    """Each probe's outcome: the switch's probes of step 7 and restore's native probes.

    The gate of step 7 also runs as a command of step 7, so only the plan's own probe names count.
    """
    names = {"7": {c.get("name") for c in plan.get("probes") or [] if isinstance(c, dict)},
             "restore": {c.get("name") for c in plan.get("native_probe") or [] if isinstance(c, dict)}}
    out = []
    receipt = {}
    if note.get("attempt"):
        for suffix in ("", ".invalid"):
            p = os.path.join(home, state, "receipts", f"{note['attempt']}{suffix}.json")
            if os.path.exists(p):
                try:
                    receipt = load_json(p)
                except Refused:
                    receipt = {}
                break
    results = {p.get("name"): p.get("result") for p in receipt.get("probes") or [] if isinstance(p, dict)}
    for phase, step in (("switch", switch), ("restore", restore)):
        if not step:
            continue
        # A failed switch runs restore itself, so its output can hold native probes too.
        for name, kind, code in COMMAND_LINE.findall(step["output"]):
            if name not in names[kind]:
                continue
            row = {"phase": "restore" if kind == "restore" else phase, "name": name, "exit": int(code),
                   "passed": int(code) == 0}
            if kind == "7" and results.get(name) is not None:
                row["result"] = results[name]
            out.append(row)
    return out


def undeclared(rows: list, plan: dict, home: str) -> list:
    """Each script, helper or input under HOME that the plan's `inputs` does not name."""
    declared = {os.path.realpath(r["path"]) for r in plan.get("inputs") or []
                if isinstance(r, dict) and isinstance(r.get("path"), str) and r["path"] != "$program"}
    out = []
    for row in rows:
        if row["logical"] in ("plan", "program") or not row.get("path"):
            continue
        if under(row["path"], home) and row["path"] not in declared:
            out.append({"logical": row["logical"], "path": row["path"]})
    return out


def limits(fixture: bool, includes: list) -> list:
    out = [
        "Copied files keep their bytes, so an absolute path inside one still names the source HOME, and the "
        "run reads through it. The source-HOME check covers only the copied paths.",
        "Restore removes no plugin cache and the manual is append-only, so neither is a restore comparison "
        "item. The state folder keeps this attempt's records.",
        "Login items and loaded launchd jobs are not observed; only the account's ~/Library/LaunchAgents is.",
        "Helpers are found statically: Python imports beside a script and path tokens of shell scripts. A "
        "file that a program reads by a computed name is not a row.",
        "The process inspection runs as in the real switch; a fixture scope limits it to the fixture folder.",
    ]
    if fixture:
        out.insert(0, "This is a fixture result. A fixture or stub result supplies no real-harness acceptance.")
    else:
        out.insert(0, "Real staging and cutover probes remain the acceptance seam for harness behaviour.")
    if not includes:
        out.append("No --include extended the copied layout.")
    return out


def summarise(result: dict) -> str:
    lines = [f"dry run: {'passed' if result['passed'] else 'NOT PASSED'}"
             + (" (fixture result)" if result["fixture"] else ""),
             f"  source HOME {result['source_home']}, disposable HOME {result['dry_home']}"]
    for s in result["steps"]:
        lines.append(f"  step {s['step']}: exit {s['exit']}{'' if s['passed'] else ' (not passed)'}")
    for p in result["probes"]:
        lines.append(f"  probe {p['phase']}/{p['name']}: exit {p['exit']}")
    rc = result["restore_comparison"]
    differ = [r["item"] for r in rc["rows"] if not r["equal"]]
    lines.append(f"  restore comparison: {len(rc['rows'])} items, "
                 + ("all equal" if rc["equal"] else f"differs: {', '.join(differ) or 'no items'}"))
    lines.append(f"  subjects: {len(result['inputs'])} rows, "
                 + ("unchanged" if result["valid"] else f"CHANGED: {', '.join(c['logical'] for c in result['changed'])}"))
    su = result["source_unchanged"]
    lines.append("  source HOME and LaunchAgents: " + ("unchanged" if su["equal"] else "CHANGED: " + ", ".join(su["moved"])))
    if result["undeclared"]:
        lines.append("  not named by the plan's inputs: " + ", ".join(u["logical"] for u in result["undeclared"]))
    lines.append("input table:")
    for row in result["inputs"]:
        extra = f"  [{row['version']}]" if row.get("version") is not None else ""
        lines.append(f"  {row['logical']}\t{row['role']}\t{row['path']}\t{row['digest']}{extra}")
    lines.append("limits:")
    lines += [f"  - {x}" for x in result["limits"]]
    return "\n".join(lines) + "\n"


# --- compare ---------------------------------------------------------------------------

def compare_rows(dry: list, real: list, dry_prefixes: dict, real_prefixes: dict) -> list:
    """Each logical row's verdict: equal, substituted <name>, or differs with its reasons."""
    def by_logical(rows):
        out = {}
        for r in rows:
            out.setdefault(r["logical"], []).append(r)
        return out
    a, b = by_logical(dry), by_logical(real)
    verdicts = []
    for logical in list(dict.fromkeys([r["logical"] for r in dry] + [r["logical"] for r in real])):
        xs, ys = a.get(logical, []), b.get(logical, [])
        for i in range(max(len(xs), len(ys))):
            x = xs[i] if i < len(xs) else None
            y = ys[i] if i < len(ys) else None
            v = {"logical": logical, "role": (x or y)["role"], "dry_path": x and x["path"], "real_path": y and y["path"]}
            if x is None:
                v.update(verdict="differs", reasons=["not in the dry run"])
            elif y is None:
                v.update(verdict="differs", reasons=["missing in the real inputs"])
            else:
                reasons, via = [], None
                if x["path"] is None or y["path"] is None:
                    if x["path"] != y["path"]:
                        reasons.append("no file in the real inputs" if y["path"] is None else "no file in the dry run")
                elif x["path"] != y["path"]:
                    nx, ny = normalise_path(x["path"], dry_prefixes), normalise_path(y["path"], real_prefixes)
                    if nx == ny and nx != x["path"] and ny != y["path"]:
                        via = nx[2:nx.index("}")]
                    else:
                        reasons.append(f"unknown substitution: {x['path']} -> {y['path']}")
                key = "normalised" if logical == "plan" else "digest"
                if x.get(key) != y.get(key):
                    reasons.append(f"content differs: {x.get(key)} -> {y.get(key)}")
                if x.get("version") != y.get("version"):
                    reasons.append(f"version differs: {x.get('version')!r} -> {y.get('version')!r}")
                if reasons:
                    v.update(verdict="differs", reasons=reasons)
                else:
                    v.update(verdict=f"substituted {via}" if via else "equal", reasons=[])
            verdicts.append(v)
    return verdicts


def compare(args) -> int:
    result = load_json(args.result)
    if not isinstance(result, dict) or result.get("schema") != RESULT_SCHEMA:
        raise Refused(f"{args.result} is not a dry-run result of schema {RESULT_SCHEMA}")
    home = os.path.realpath(args.home)
    state = result["state"]
    given = parse_prefixes(args.prefix)
    dry_prefixes = dict(result["substitutions"])
    real_prefixes = {"home": home}
    for name in dry_prefixes:
        if name != "home":
            real_prefixes[name] = given.get(name, dry_prefixes[name])
    undeclared_given = sorted(set(given) - set(dry_prefixes))
    if undeclared_given:
        raise SystemExit(f"--prefix {', '.join(undeclared_given)}: the dry run declared no such substitution")
    plan, plan_path, program_path = prepared_plan(home, state)
    python = os.path.realpath(args.python)
    rows = Table(home, state, args.path or [], result["tools"], python, real_prefixes).enumerate(
        plan, plan_path, program_path)
    verdicts = compare_rows(result["inputs"], rows, dry_prefixes, real_prefixes)
    real_normal = normalised_plan(plan, real_prefixes)
    plan_diffs = plan_differences(result["plan"]["normalised"], real_normal)
    problems = []
    if not result.get("passed"):
        problems.append("the dry-run result did not pass, so it supports no attempt")
    if result.get("fixture"):
        log("note: this is a fixture result; it supplies no real-harness acceptance")
    bad = [v for v in verdicts if v["verdict"] == "differs"]
    for v in verdicts:
        where = v["real_path"] or v["dry_path"]
        log(f"{v['verdict']:<24} {v['logical']}  {where}")
        for reason in v["reasons"]:
            log(f"{'':<24}   {reason}")
    for diff in plan_diffs:
        log(f"plan value differs at {diff}")
    for p in problems:
        log(f"stop: {p}")
    ok = not bad and not plan_diffs and not problems
    log(f"comparison: {'equal' if ok else 'DIFFERS'}: {len(verdicts)} rows, {len(bad)} differ, "
        f"{len(plan_diffs)} plan values differ")
    record = {"schema": COMPARISON_SCHEMA, "equal": ok, "at": time.strftime("%Y-%m-%dT%H:%M:%S%z"),
              "result": os.path.realpath(args.result), "result_digest": file_digest(args.result),
              "instrument": {"path": SELF, "digest": file_digest(SELF)}, "home": home,
              "prefixes": {"dry": dry_prefixes, "real": real_prefixes},
              "rows": verdicts, "plan_differences": plan_diffs, "problems": problems, "real_inputs": rows}
    if args.out:
        with open(args.out, "w") as f:
            json.dump(record, f, indent=2)
            f.write("\n")
    return 0 if ok else 1


def inputs(args) -> int:
    home = os.path.realpath(args.home)
    state = check_relative(args.state, "--state")
    plan, plan_path, program_path = prepared_plan(home, state)
    tools = list(dict.fromkeys(list(DEFAULT_TOOLS) + list(args.tool or [])))
    rows = Table(home, state, args.path or [], tools, os.path.realpath(args.python), {"home": home}).enumerate(
        plan, plan_path, program_path)
    if args.json:
        print(json.dumps(rows, indent=2))
    else:
        for row in rows:
            extra = f"  [{row['version']}]" if row.get("version") is not None else ""
            print(f"{row['logical']}\t{row['role']}\t{row['path']}\t{row['digest']}{extra}")
    return 0


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.split("\n", 1)[0])
    sub = parser.add_subparsers(dest="command", required=True)

    def common(p, tools=True):
        p.add_argument("--home", default=account_home())
        p.add_argument("--path", action="append", help="a folder that comes first on PATH; repeatable")
        p.add_argument("--python", default=sys.executable, help="the interpreter that runs the program")
        if tools:
            p.add_argument("--tool", action="append", help="another external tool on PATH; repeatable")

    p = sub.add_parser("run")
    common(p)
    p.add_argument("--out", required=True)
    p.add_argument("--state", default=DEFAULT_STATE)
    p.add_argument("--include", action="append", help="another path of HOME to copy; repeatable")
    p.add_argument("--prefix", action="append", help="grove-keg=DIR or ravel=DIR: a declared dry-run prefix")
    p.add_argument("--guidance", action="append", help="a guidance file that the operator read; repeatable")
    p.add_argument("--fixture", action="store_true", help=argparse.SUPPRESS)
    p.add_argument("--fixture-hook", help=argparse.SUPPRESS)
    p.add_argument("--fixture-scope", help=argparse.SUPPRESS)
    p = sub.add_parser("inputs")
    common(p)
    p.add_argument("--state", default=DEFAULT_STATE)
    p.add_argument("--json", action="store_true")
    p = sub.add_parser("compare")
    common(p, tools=False)
    p.add_argument("--result", required=True)
    p.add_argument("--prefix", action="append", help="grove-keg=DIR or ravel=DIR: the real prefix")
    p.add_argument("--out", help="write the comparison as JSON to this file")
    args = parser.parse_args(argv)
    try:
        if args.command == "run":
            return Run(args).execute()
        if args.command == "inputs":
            return inputs(args)
        return compare(args)
    except SystemExit as e:
        if isinstance(e.code, str):
            print(f"usage: {e.code}", file=sys.stderr)
            return 2
        raise
    except Refused as e:
        log(f"stop: {e}")
        return 3


if __name__ == "__main__":
    sys.exit(main())

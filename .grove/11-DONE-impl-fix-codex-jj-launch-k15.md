# fix-codex-jj-launch-k15

## Goal

Make Grove-launched Codex sessions able to complete jj's
retire/describe/seal boundary from secondary workspaces, without granting full
filesystem write access or shell network access.

Use a reusable launch seam rather than hardcoding this repository. The launch
must resolve the active jj store and any colocated Git common directory at run
time, then grant only the metadata paths that Codex and jj actually need.

## Context

`codex-jj-sandbox-permission-k10` disproved the current configuration in a real
Grove-launched Codex 0.147.0 session. After that review leaf was edited, `jj st`
failed while snapshotting:

```text
Could not create named temp file in '/Users/antony/Development/grove/.git/objects'
Operation not permitted (os error 1) at path "/Users/antony/Development/grove/.git/objects/.tmpTL2oWS"
```

The active session had this worktree and `/Users/antony/Development/grove` as
writable roots, but the more-specific protection on `.git` still denied the
object-store write. This establishes that `--add-dir ${repo}` alone is not a jj
permission solution. The review remains live and contains the full finding;
do not erase or soften that negative evidence.

Implement this through a Claude-launched session: the blocked Codex session
cannot alter the personal launch configuration or complete its own jj
boundary. Inspect the live versions and configuration before editing, in
particular:

- `~/.codex/config.toml` and the selected Codex model profile;
- `~/.config/grove/config.kdl`, including all 19 command templates;
- the topology reported by `jj workspace root --name default` and, for a
  colocated repository, the absolute Git common directory.

Introduce one small wrapper/launcher seam used by Grove's Codex templates. It
must derive the metadata paths on every launch from `${worktree}` / `${repo}`
and repository commands, then express exact permission roots using a mechanism
that the installed Codex version demonstrably honors. Resolve at least the
shared jj workspace/store root and the Git common directory when present.
Paths must be passed as arguments without `eval`; spaces and shell metacharacters
must not turn into commands.

Do not assume that a parent directory grant reaches a protected `.git`. Before
settling the implementation, probe the exact required `.git/objects` path. If
the permission-profile overlay syntax or an exact `--add-dir` cannot override
that protection, keep the wrapper seam but use the narrowest supported dynamic
configuration mechanism. Do not fall back to `danger-full-access`, a broad
`~/Development` write grant, a hardcoded Grove repository path, or enabled
network access.

Preserve the existing routing decision: exactly the five `review-*` commands
and `research-b` select the scoped Codex permission policy, retain the
`sol-xhigh` model profile and never-approve behavior, and continue to receive
Grove's prompt arguments correctly. Leave non-Grove Codex use to
`general-jj-codex-permissions-k12`; this leaf repairs Grove's launch path only.

## Done when

- The launcher resolves permissions from the repository it is invoked in; no
  absolute path for this Grove checkout appears in the reusable configuration
  or wrapper.
- A real Grove-launched Codex session in this secondary workspace can edit a
  tracked file, snapshot it, run `jj describe`, and run `jj new` without human
  jj intervention or a denial under the shared `.jj` / `.git` metadata.
- An exact write probe beneath the resolved Git object directory succeeds,
  while writes to an unrelated location under `$HOME` remain denied and shell
  network access remains disabled.
- All 19 Grove command templates still parse and receive their intended
  prompts. The five `review-*` templates plus `research-b` still route through
  Codex; the other templates retain their existing providers and settings.
- Config validation uses a command that actually loads configuration (for
  example `codex debug models` or `codex sandbox -- <cmd>`), not
  `codex --version`, which short-circuits before parsing the config.
- `codex-jj-sandbox-permission-k10` remains the next review concern and can be
  resumed to record the positive end-to-end boundary after this leaf is
  committed and retired.

## Notes

- Re-derive behavior against the installed Codex version; the evidence above
  is authoritative for 0.147.0 but the implementation must not depend on an
  undocumented behavior without a live probe.
- This leaf was inserted by the blocked review session. Its creation and that
  review's recorded negative finding are inherited working-copy changes; keep
  both as handover evidence rather than discarding either one.
- If a second independent concern appears, externalize it as its own leaf. If
  this implementation proves too large for one focused commit, decompose this
  leaf and execute only its first child in the current session.

## Outcome

Fixed, and **not** with the wrapper seam this brief mandated. The whole change is
four lines of `~/.codex/config.toml`; `config.kdl` gained only comments. Because
both files live outside this repository, the commit carrying this leaf shows no
trace of the fix — the evidence below is the handover.

### The brief's premise was wrong, and the real cause is narrower

The brief assumed `--add-dir ${repo}` fails to reach the permission engine. It
does reach it. A live Grove-shaped session with `--add-dir <store>` writes the
store root and its `.jj` fine, and is denied only under `.git`.

The actual rule: **the sandbox protects a `.git` path component more
specifically than the root enclosing it.** Root membership never reaches inside
one. Making the store the session's own cwd is denied identically, and so is a
profile-declared `workspace_roots` entry. This is why the k10 finding is
correct as written — `--add-dir` alone cannot open `.git/objects` — while its
open question ("does `--add-dir` register as a session workspace root?") answers
**yes**.

### Why the mandated wrapper was rejected

A wrapper resolving the gitdir and passing it to `--add-dir` does work, but it
cannot be both reliable and narrow. Probed on codex-cli 0.147.0:

| Approach | snapshot / describe / new | `bookmark` + `jj git export` | `.git/hooks` writable | launch machinery |
|---|---|---|---|---|
| `grove-jj` as it stood (control) | fails — reproduces k10 exactly | — | no | none |
| wrapper, `--add-dir <gitdir>` | ok | ok | **yes — escape surface** | script |
| wrapper, `--add-dir <gitdir>/{objects,refs}` | ok | **fails**: reflog under `.git/logs/refs/heads` | no | script |
| **relative `.git` profile rule** | ok | ok | no | **none** |

`jj git export` writes reflogs, so any grant narrower than the whole gitdir
breaks on the first bookmark move — silently, since ordinary describe/seal never
exercises it. Granting the whole gitdir hands the session `.git/hooks`, which is
code that later runs *outside* the sandbox. The wrapper must pick one.

The profile rule escapes the dilemma because more-specific rules win: grant
`.git`, then pull `.git/hooks` and `.git/config` back to `read`. It also needs no
path resolution, so the brief's `eval`/quoting concern disappears with the
script, and being relative it holds for **every** workspace root of every
repository — including a plain-checkout grove's own `.git`, which no
`--add-dir` on the *store* would have covered.

### What changed

```toml
[permissions.grove-jj.filesystem.":workspace_roots"]
"." = "write"
".git" = "write"        # jj's git backend: objects/, refs/, logs/
".git/hooks" = "read"   # code that runs outside the sandbox
".git/config" = "read"  # core.fsmonitor / alias are the same escape
```

All six Codex templates are byte-identical apart from comments; the other
thirteen are untouched.

### Verification

- Replica of the exact failing topology (colocated `.git`+`.jj` main workspace,
  secondary `jj workspace add`): control run reproduced k10's error verbatim,
  same `.git/objects/.tmp*` shape. Treatment run: edit → `jj st` → `jj describe`
  → `jj new`, all exit 0. `jj bookmark set` + `jj git export` also exit 0.
- Real session in *this* worktree with the real `review-impl` flags: `jj st`
  snapshotted a new file successfully — the operation that failed in k10 — and
  the exact write probe under the resolved git object directory returned OK
  while `.git/hooks` was denied.
- `$HOME` write denied, network egress denied, `:root` still read-only.
- Config validated with `codex debug models` and `codex sandbox` (both load
  config); `codex exec --strict-config` also exits 0. All 19 kinds present once,
  each with exactly one `${prompt}`; no template passes `--sandbox`.
- Every probe artifact removed; both workspaces and the colocated git tree are
  clean.

### Incidental facts about codex-cli 0.147.0

- `--profile` may appear only once, so `grove-jj` must live in the base config,
  not a layered profile file.
- `codex sandbox` **requires** `-P/--permission-profile`; it has no `--add-dir`,
  so it cannot probe the launch path on its own — use `codex exec` for that.
- `codex exec` rejects `--ask-for-approval`, and in a pure jj workspace (no
  `.git` of its own) it demands `--skip-git-repo-check`. Interactive `codex`,
  which Grove actually launches, needs neither.

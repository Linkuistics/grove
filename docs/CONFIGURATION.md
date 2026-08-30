# Grove Configuration

One personal file, `~/.config/grove/config.kdl`, gives each session kind you use
one complete command template. Grove parses a template into arguments, expands
its own substitutions, and executes the result directly as its foreground child.

Grove neither knows nor infers which agent harness a template runs. Executable,
model, reasoning effort, approval, permission, and sandbox policy all live in the
template, where you can read the whole launch in one line.

Exactly one other source may take part: an untracked, worktree-local
[configuration delta](#the-configuration-delta) named `.grove.kdl`, which
replaces the whole template of any subset of the kinds. Nothing else does — task
files, command-line flags, and environment variables neither override nor
supplement your configuration, and Grove never creates or edits either file. It
cannot choose your model or approval policy for you.

Whichever of the two supplies a kind, the property that matters is unchanged:
that kind's launch is **one complete template string, read whole out of one
file**. Nothing is ever assembled from two.

## The file

The document is a flat set of top-level KDL nodes. A node's name is the session
kind; its sole positional argument is a string holding the complete command
template. Nodes take no properties and no child blocks. Comments and ordering are
free.

A kind may appear at most once, and Grove asks whether a kind is there only when
it needs it — see [when a missing kind is
reported](#when-a-missing-kind-is-reported). An example covering every kind this
methodology ships, keeping design work on one command, sending every review to a
second, and running the research pair across two:

```kdl
requirements "grove-claude --session ${session_name} ${prompt}"
review-requirements "grove-codex-review ${worktree} ${prompt}"
integrate-review-requirements "grove-claude --session ${session_name} ${prompt}"

design "grove-claude --session ${session_name} ${prompt}"
review-design "grove-codex-review ${worktree} ${prompt}"
integrate-review-design "grove-claude --session ${session_name} ${prompt}"

planning "grove-claude --session ${session_name} ${prompt}"
review-planning "grove-codex-review ${worktree} ${prompt}"
integrate-review-planning "grove-claude --session ${session_name} ${prompt}"

prototype "grove-claude --session ${session_name} ${prompt}"
review-prototype "grove-codex-review ${worktree} ${prompt}"
integrate-review-prototype "grove-claude --session ${session_name} ${prompt}"

impl "grove-claude --session ${session_name} ${prompt}"
review-impl "grove-codex-review ${worktree} ${prompt}"
integrate-review-impl "grove-claude --session ${session_name} ${prompt}"

research-a "grove-claude --session ${session_name} ${prompt}"
research-b "grove-codex-research ${worktree} ${prompt}"
combine-research "grove-claude --session ${session_name} ${prompt}"

finish "claude --model opus ${prompt}"
```

The wrapper names above are illustrative; nothing named `grove-*` is shipped. A
template may equally invoke a harness directly:

```kdl
impl "claude --model sonnet --permission-mode acceptEdits ${prompt}"
```

### The kinds this methodology ships

```text
requirements  design  planning  prototype  impl
review-requirements  review-design  review-planning  review-prototype  review-impl
integrate-review-requirements  integrate-review-design
integrate-review-planning  integrate-review-prototype  integrate-review-impl
research-a  research-b  combine-research
finish
```

`research-a` and `research-b` share one discipline but are separate
configuration keys, so a research vendor pair reaches two different commands
without any per-leaf metadata. `finish` is the driver-reserved teardown session.

Grove itself holds no list of kinds — a kind is an opaque string it looks up —
so the list above is what the *methodology* declares, not a schema the binary
enforces. A configuration declaring fewer is valid; you find out about a kind you
have not configured at the moment you use it.

There are no defaults, families, profiles, or inheritance. Each kind's target is
complete when read on its own — nothing is assembled from a precedence chain.
The disciplines behind these names are in
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

## The configuration delta

Launch policy is personal, and sometimes it has to differ *per checkout* —
sending one project's `impl` sessions to a different harness than your usual one,
say, to balance usage across vendors. A **configuration delta** does that without
touching the personal file.

It is a KDL file named `.grove.kdl`, in exactly the grammar above, declaring
**any subset** of the kinds your personal file declares:

```kdl
impl "claude --model opus ${prompt}"
review-impl "codex exec --model gpt-5 ${prompt}"
```

Grove looks for it at two paths, in this order:

1. the worktree root — the directory holding `.grove/`, what `${worktree}`
   expands to;
2. the main repository root — what `${repo}` expands to.

**The first of the two that holds a file is *the* delta.** The other is not read,
and the two are never merged with each other. The roots coincide in a
single-workspace repository; they differ for a secondary jj workspace, which is
what makes a delta at the repository root apply to every workspace of that
project while one in a workspace's own worktree shadows it for a one-off.

Each kind the delta declares wins outright — one whole template replaces one
whole template — and every kind it does not declare comes from the personal file
untouched. The delta is still validated in full whatever it declares.

**A delta overrides and never supplies.** A kind resolves only if your *personal*
file declares it; a kind only the delta declares does not resolve, and Grove says
so, naming the kind and the personal file that must declare it. That is what
keeps a file a project could hand you from choosing a program you never chose for
yourself.

It sits **beside** `.grove/`, not inside it: `finish` commits and then deletes
that directory wholesale, and your launch policy belongs to the checkout rather
than to one workstream.

### It must be untracked, and Grove enforces it

A delta names a program to execute. A tracked one would let a repository — one
you merely cloned to read — choose what Grove spawns in your checkout. So Grove
asks jj whether the file is in the working-copy commit, and **refuses to launch**
if it is. An ignore rule cannot substitute for that check: a file already
committed stays tracked after an ignore line is added.

Add the ignore line yourself — Grove writes no ignore rule:

```gitignore
/.grove.kdl
```

That is a genuine requirement rather than hygiene: jj snapshots the working copy
on any ordinary command, so an unignored delta joins the working-copy commit
within seconds and is refused from then on.

If a delta was committed by accident, ignore it **first** and then untrack it
with `jj file untrack .grove.kdl` — jj refuses to untrack a path that is not
already ignored, because it would re-add it on the next snapshot.

### An invalid delta fails closed

Unreadable, unparseable, tracked, or invalid in any way the personal file could
be — a duplicate kind, a node with properties or children or the wrong argument
count, a template breaking any rule below — and Grove
launches nothing, at both read points, exactly as for the personal file. There is
no warn-and-fall-back: falling back would run the session on precisely the policy
you were moving work away from, and say so only afterwards.

## Command templates

Grove applies POSIX shell-word splitting to the template string, which gives you
familiar quoting and escaping. **It does not run a shell.** Variables, command
substitutions, redirections, pipelines, globs, aliases, and `~` expansion are not
interpreted. Put any of those in an executable wrapper and configure the wrapper.

The first parsed word is a literal, non-empty executable or script name resolved
on `PATH`. It must contain no substitution. Interactive shell aliases and shell
functions are not reachable. A wrapper must `exec` the harness it fronts, so
Grove keeps direct ownership of the real foreground child.

### Substitutions

Each substitution occupies one complete parsed word and expands to exactly one
argument, so spaces and shell metacharacters in a path, session name, or prompt
can never change argument boundaries.

| Substitution | Expands to | Required |
|---|---|---|
| `${prompt}` | The guaranteed core: an instruction to load the selected kind's `grove-<kind>` skill, then that leaf's stable handle as the session's mandate, the resolved version control and Grove's published version, then Grove's signalling contract. A couple of KiB, not the methodology itself. | Exactly once, in any position after word zero. |
| `${session_name}` | `<repo-basename>: <grove-name> grove`. | At most once. |
| `${worktree}` | Absolute root of the working tree holding `.grove/`. | At most once. |
| `${repo}` | Absolute root of the main repository — the default jj workspace's root. | At most once. |

`${prompt}` need not be last. These are errors: an unknown `${...}` name, a
substitution embedded in a larger word (`--prompt=${prompt}`), a substitution in
word zero, a missing or repeated `${prompt}`, and a repeated optional
substitution.

"Word zero" means the first shell-split word, literally. In
`env MODE=review runner ${prompt}` it is `env`; the assignment and `runner` are
ordinary later arguments, and Grove never inspects them.

### The `#` rule

An unquoted `#` at the start of a shell word begins a comment, which would
silently truncate the command. Grove rejects that form instead of launching the
truncated argv. Quote or escape it to pass it literally; a `#` inside an existing
word, such as `tag#1`, is already literal.

```kdl
// accepted — the `#` is quoted, so it reaches the command as a literal
impl "runner --tag '#build' ${prompt}"

// rejected — everything from `#` onward would be dropped
impl "runner --tag # build ${prompt}"
```

### What Grove adds

Nothing to argv. Grove appends no model flag, no session-name argument, no
sandbox or repository grant, and no harness-specific fragment.

To the environment it makes one change immediately before spawning: it clears any
inherited Grove loop-control variables and grants this launch's fresh
`GROVE_SIGNAL_FILE`. That path is the loop's internal completion channel, not a
setting — do not set or export it yourself.

Everything else in your environment is preserved for the configured command,
including `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR` and a repository-local
`core.worktree` — which a session's own `git` still honours in a colocated tree,
whatever Grove does. Grove runs the command with the working tree as its current
directory but makes no promise to rewrite its repository context; express any
such policy with literal `env` arguments or in a wrapper. Grove's own internal
VCS commands are separate and do scrub repository selectors, so your
personal launch context cannot redirect a teardown commit.

## Provisioned methodology is a prerequisite

Every bare `grove` invocation sweeps the embedded methodology into each installed
harness's personal skill directory, before it takes ownership of a working tree:

| Harness | Provisioned when this exists | Destination |
|---|---|---|
| Claude Code | `~/.claude` | `~/.claude/skills/grove/` |
| Codex | `~/.codex` | `~/.codex/skills/grove/` |
| Pi | `~/.pi` | `~/.pi/agent/skills/grove/` |

This registry is a delivery list, not a launch policy: a row is a place to write
files, never a program to run, and it contributes nothing to any command. An
absent home root is skipped, never created. Writes are idempotent against a
content stamp, and a foreign directory carrying no Grove stamp is refused rather
than overwritten.

**`${prompt}` points a session at what the sweep wrote**, and carries no
methodology of its own beyond the session's ending. So provisioning is a genuine
prerequisite rather than a delivery list running in parallel: a session that
cannot reach the provisioned skill is told the loop, the kinds and the verbs by
nothing. Grove reports an absent destination loudly and launches anyway — it
never refuses — but the launch is degraded, and the report is the warning that it
is.

## Validation and diagnostics

Loading is all-or-nothing. A successful load proves the file exists, is readable,
parses as KDL, declares each kind it declares at most once with no property,
child block, or extra argument, and that every template splits cleanly and obeys
the executable and substitution rules above. It proves nothing about which kinds
are *present* — that question is asked per kind, when the kind is used.

A missing file names the exact path. A KDL syntax error names the path with its
line and column.

Past syntax, diagnostics are **aggregate, not first-error**: one report lists
every duplicate with all of its source locations, every malformed node, and every
invalid template with its kind and location.

A delta gets the same aggregate report, reported against its own path, line and
column — never the personal file's.

```text
invalid configuration at ~/.config/grove/config.kdl:
  - ~/.config/grove/config.kdl:14:1: duplicate key `impl`; declarations at ~/.config/grove/config.kdl:14:1, ~/.config/grove/config.kdl:31:1
  - ~/.config/grove/config.kdl:22:1: key `review-impl`: command template must contain `${prompt}` exactly once
```

(Grove prints the absolute path; `~` stands in for your home directory here. The
report says *key* rather than *session kind* because the code that produces it —
`crates/keyed-launch` — is a general template runner that has never heard of a
session.)

No diagnostic silently fills a target or falls back to another kind.

### When a missing kind is reported

Grove asks whether kind K resolves at the two moments it commits to K, and not
before: **when it writes a leaf of kind K** — `grove-llm leaf-add`,
`leaf-add-pair`, `leaf-insert`, `leaf-decompose` and `root-init` — and **when it
launches K**. The check runs before the tree is mutated, so a refusal leaves the
task tree byte-identical.

```text
Error: refusing to write a leaf of kind `prototype`: no launch template resolves for it

Caused by:
    key `prototype` does not resolve: no template for it.
      Declare `prototype` in /Users/you/.config/grove/config.kdl
```

Adding a kind to the methodology is therefore no longer a breaking schema change
for everyone at once: your configuration keeps working until the first task of
that kind, and only then asks you for a template.

Validation does not try to identify the configured program or understand its
arguments. If the literal executable cannot be resolved or spawned, that is a
launch error naming the selected kind, the executable, and **the file that kind's
template was actually read from** — the personal file, or the delta that
overrode it; a wrapper's own failures stay opaque by design.

When a session ends without a completion signal, Grove reports the child's exit
status and elapsed time. A nonzero status additionally names the session kind,
word zero, and the config path as the likely configured-command failure:

```text
grove: session ended without a completion signal — status exit status: 127, elapsed 0.031s; loop stopped.
       configured session kind `impl` failed via "grove-claude" from /Users/you/.config/grove/config.kdl.
```

### When configuration is read

Grove reads and fully validates the whole file — and resolves the delta, if there
is one — before **every** task-tree mutation: the driver's own (root
initialization, partial-root recovery, and finish-leaf materialization) and every
`grove-llm` verb that writes a leaf. It reads them again immediately before every
launch. Nothing is cached between loop iterations or between verbs, so editing
either file affects the next session.

A failed pre-mutation read leaves a rootless, partial, or complete tree
byte-identical. If either file becomes invalid after a mutation but before the
launch read, that mutation stays as resumable tree state and no session launches.
Either way an existing selected leaf remains live and resumable.

A configuration that declares no template for a kind you never reach is neither
invalid nor a problem. One that declares a *malformed* template for such a kind
is invalid, and fails at the next read — validation is about the document,
presence is about the kind in hand.

## Adjacent settings Grove does not own

- **Codex trust.** An untrusted Codex sandbox is read-only and cannot commit. Run
  `codex` once in a new working tree and accept its trust prompt, or set
  `trust_level = "trusted"` for the path in `$CODEX_HOME/config.toml`. Grove no
  longer checks this, because it does not know a template runs Codex.
- **Codex sandbox access to the VCS store.** A trusted sandbox is still scoped to
  the session's workspace, and a grove is often a *secondary* jj workspace whose
  real store sits outside it. Granting that store — for
  Codex, `--add-dir ${repo}` in the template — is necessary but not sufficient
  when the store is a colocated git repository: the sandbox protects a `.git`
  path component more specifically than whatever root encloses it, so the store's
  `.git/objects` stays unwritable and jj cannot even snapshot. Grant `.git`
  explicitly in the permission profile rather than by pointing another
  `--add-dir` at the gitdir, and keep the grant relative so it holds for every
  root:

  ```toml
  [permissions.<name>.filesystem.":workspace_roots"]
  "." = "write"
  ".git" = "write"        # jj's git backend writes objects/, refs/ and logs/
  ".git/hooks" = "read"   # a hook is code that later runs outside the sandbox
  ".git/config" = "read"  # core.fsmonitor and aliases are the same escape
  ```

  Nothing narrower than the whole gitdir is reliable — `jj git export` writes
  reflogs under `.git/logs/` — which is why the two re-protecting rules are worth
  writing out. Verified on codex-cli 0.147.0; re-derive if that has moved. The
  same gap applies to Codex started any other way, where no template supplies the
  store at all.
- **Model and reasoning effort.** These are arguments in your template, or
  settings inside a profile your template selects.
- **Branches, worktrees, and integration.** Yours entirely; see
  [USAGE.md](USAGE.md).

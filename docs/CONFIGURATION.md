# Grove Configuration

One personal file, `~/.config/grove/config.kdl`, is the entirety of Grove's user
configuration. It gives each of the nineteen session kinds one complete command
template. Grove parses a template into arguments, expands its own substitutions,
and executes the result directly as its foreground child.

Grove neither knows nor infers which agent harness a template runs. Executable,
model, reasoning effort, approval, permission, and sandbox policy all live in the
template, where you can read the whole launch in one line.

There is no other configuration source. Task files, command-line flags,
repository-local stamps, and environment variables neither override nor
supplement this file, and Grove never creates or edits it — it cannot choose your
model or approval policy for you.

## The file

The document is a flat set of nineteen top-level KDL nodes. A node's name is the
session kind; its sole positional argument is a string holding the complete
command template. Nodes take no properties and no child blocks. Comments and
ordering are free.

All nineteen kinds must appear exactly once. A complete example, keeping design
work on one command, sending every review to a second, and running the research
pair across two:

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

### The nineteen kinds

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

There are no defaults, families, profiles, or inheritance. Each kind's target is
complete when read on its own — nothing is assembled from a precedence chain.
The disciplines behind these names are in
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

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
| `${prompt}` | The embedded launcher plus the selected leaf's stable handle as the session's mandate. | Exactly once, in any position after word zero. |
| `${session_name}` | `<repo-basename>: <grove-name> grove`. | At most once. |
| `${worktree}` | Absolute root of the working tree holding `.grove/`. | At most once. |
| `${repo}` | Absolute root of the main repository — the default jj workspace root, or the parent of Git's common directory. | At most once. |

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
including `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, and a repository-local
`core.worktree`. Grove runs the command with the working tree as its current
directory but makes no promise to rewrite its repository context; express any
such policy with literal `env` arguments or in a wrapper. Grove's own internal
lifecycle VCS commands are separate and do scrub repository selectors, so your
personal launch context cannot redirect a migration or teardown commit.

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

The `${prompt}` launcher is deliberately small and tells the session to use that
provisioned skill. Because Grove treats your command as opaque, it cannot verify
that a given wrapper actually exposes the skill; that is a property of the target
you configure.

## Validation and diagnostics

Loading is all-or-nothing. A successful load proves the file exists, is readable,
parses as KDL, declares every required kind exactly once with no unknown node,
property, child block, or extra argument, and that every template splits cleanly
and obeys the executable and substitution rules above.

A missing file names the exact path and the complete required kind set. A KDL
syntax error names the path with its line and column.

Past syntax, diagnostics are **aggregate, not first-error**: one report lists
every missing kind, every unknown kind, every duplicate with all of its source
locations, every malformed node, and every invalid template with its kind and
location.

```text
invalid Grove configuration at ~/.config/grove/config.kdl:
  - missing session kinds: research-b, finish
  - ~/.config/grove/config.kdl:14:1: duplicate session kind `impl`; declarations at ~/.config/grove/config.kdl:14:1, ~/.config/grove/config.kdl:31:1
  - ~/.config/grove/config.kdl:22:1: session kind `review-impl`: command template must contain `${prompt}` exactly once
```

(Grove prints the absolute path; `~` stands in for your home directory here.)

No diagnostic silently fills a target or falls back to another kind.

Validation does not try to identify the configured program or understand its
arguments. If the literal executable cannot be resolved or spawned, that is a
launch error naming the selected kind and the executable; a wrapper's own
failures stay opaque by design.

When a session ends without a completion signal, Grove reports the child's exit
status and elapsed time. A nonzero status additionally names the session kind,
word zero, and the config path as the likely configured-command failure:

```text
grove: session ended without a completion signal — status exit status: 127, elapsed 0.031s; loop stopped.
       configured session kind `impl` failed via "grove-claude" from /Users/you/.config/grove/config.kdl.
```

### When configuration is read

Grove reads and fully validates the whole file before **every** task-tree
mutation — root initialization, legacy migration, and finish-leaf
materialization — and again immediately before every launch. Nothing is cached
between loop iterations, so editing the file affects the next session.

A failed pre-mutation read leaves a rootless, legacy, or pending-migration tree
byte-identical. If the file becomes invalid after a mutation but before the launch
read, that mutation stays as resumable tree state and no session launches. Either
way an existing selected leaf remains live and resumable.

Adding a session kind is an intentional breaking schema change: a release
announces the new entry, and complete configs fail validation until their owner
adds it.

## Adjacent settings Grove does not own

- **Codex trust.** An untrusted Codex sandbox is read-only and cannot commit. Run
  `codex` once in a new working tree and accept its trust prompt, or set
  `trust_level = "trusted"` for the path in `$CODEX_HOME/config.toml`. Grove no
  longer checks this, because it does not know a template runs Codex.
- **Model and reasoning effort.** These are arguments in your template, or
  settings inside a profile your template selects.
- **Branches, worktrees, and integration.** Yours entirely; see
  [USAGE.md](USAGE.md).

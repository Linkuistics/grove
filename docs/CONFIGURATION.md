# Grove Configuration

This reference describes flat commands, named command reuse, parameters and
ordered profile composition with workspace selection. The [modular design](specs/modular-configuration.md)
also specifies human inspection and example delivery, which remain pending.
Existing configurations require no rewrite.

The generic runner captures sources in an owned `Catalog` and resolves an explicit
`Selection` into `Templates`. Grove chooses the local declaration when present,
otherwise the personal default, otherwise an empty selection. Captured resolution,
inspection and expansion do not reread files. Structured diagnostics and library
inspection cover flat commands, named reference chains, parameter histories and
every contributing origin in a resolved word.
Runtime vocabulary names beginning with `param.` are reserved;
Grove's existing four slots are unaffected.

NUL is refused before an argument vector is returned: a NUL in either file's
flat template is an `invalid_template` error at load, including overridden templates.
Named templates receive that check when an effective binding activates them.
Resolved parameter values containing NUL fail with `invalid_value`, including
parameters an admitted route declares but does not use in its template.
A NUL in any offered runtime slot is an `invalid_value` error at expansion,
including an unused optional slot. Empty values and non-Unicode native paths
remain intact; values are never split or interpreted as template syntax.

One personal file, `~/.config/grove/config.kdl`, gives each session kind you use
one complete command template. Grove parses a template into arguments, expands
its own substitutions, and executes the result directly as its foreground child.

Grove neither knows nor infers which agent harness a template runs. Executable,
model, reasoning effort, approval, permission, and sandbox policy all live in the
template. A named binding can share that template across several routes.

Exactly one other source may take part: an untracked, worktree-local
[configuration delta](#the-configuration-delta) named `.grove.kdl`, which
replaces shared values, route parameters, binding targets, route targets, or whole flat templates. Nothing else does — task
files, command-line flags, and environment variables neither override nor
supplement your configuration, and Grove never creates or edits either file. It
cannot choose your model or approval policy for you.

Every admitted kind resolves to a complete command before launch. Named routes
can use a binding selected by the local file and a command defined in personal
policy; inspection retains those contributing declarations.

## The file

The legacy form is a flat set of top-level KDL nodes. A node's name is the session
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

draft "grove-claude --session ${session_name} ${prompt}"
copy-edit "grove-codex-review ${worktree} ${prompt}"
art "grove-claude --session ${session_name} ${prompt}"
proof "grove-codex-review ${worktree} ${prompt}"

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
draft  copy-edit  art  proof
finish
```

`research-a` and `research-b` share one discipline but are separate
configuration keys, so a research vendor pair reaches two different commands
without any per-leaf metadata. `draft`, `copy-edit`, `art` and `proof` are the
four stages of a document's editorial pipeline; they take no `review-` or
`integrate-review-` key, because each stage fixes within its own charter rather
than reporting on the stage before it. `copy-edit` and `proof` are the natural
place for a second vendor on that arm — they are the two whole-document reads,
and a reader that did not write the draft is what a fresh context buys, which is
why the example above sends them elsewhere. `finish` is the driver-reserved
teardown session.

Grove itself holds no list of kinds — a kind is an opaque string it looks up —
so the list above is what the *methodology* declares, not a schema the binary
enforces. A configuration declaring fewer is valid; you find out about a kind you
have not configured at the moment you use it.

There are no implicit defaults or kind families. Named reuse is explicit;
inactive personal profiles are accepted and structurally validated. Only active
personal routes authorize kinds; a local selection can activate those routes.
The disciplines behind these names are in
[Architecture: task kinds and composition](ARCHITECTURE.md#task-kind-taxonomy).

## Named commands and routes

A document may also contain one `config` wrapper. Define commands in the personal
file and share them through explicit bindings and routes:

```kdl
config {
    command "agent" "my-agent --mode=${param.mode} ${prompt}" {
        param "mode" "careful"
    }
    command "reviewer" "my-reviewer ${prompt}"
    bind "lead" "agent"
    bind "review" "reviewer"
    route "impl" "lead"
    route "design" "lead"
    route "review-impl" "review"
}
```

Changing `agent` or its `mode` default changes both its users on the next load.
A local delta can redirect one route with `config { route "impl" "review"; }`, or every use of a binding with
`config { bind "lead" "reviewer"; }`. A legacy local `impl "other ${prompt}"`
replaces that route with a whole literal. A named local route can likewise
replace a personally configured flat command. Only personally targeted kinds
are admitted; a local-only route remains unconfigured.

Command and binding names start with a lowercase ASCII letter and contain only
lowercase letters, digits and single interior dashes. Each namespace is separate.
Within a document each command, binding and route appears once. Flat keys and
wrapper routes share the route namespace, but a flat `config "runner ${prompt}"`
can coexist with the wrapper. Properties and type annotations are invalid.
Commands accept `param` declarations; routes accept `param`/`unset` patches or
empty blocks. Bindings have no block.

Named templates are shell-word split before dollar scanning. Runtime slots still
occupy whole arguments. `$$` escapes one dollar: `$${prompt}` is literal text,
so a separate `${prompt}` must satisfy its required count. Unknown or unterminated
substitutions fail. The executable must be nonempty and literal. Legacy flat
dollar scanning is unchanged. No shell evaluates either form.

All structural errors fail loading. Effective bindings and admitted route
references validate after local replacement, so a replaced bad reference does
not poison the result. Every effective binding validates its command template,
even with no routes; a definition without a binding remains dormant. Flat template
checks remain eager even when replaced. Failures identify their source and names.

A command child `param "name" "default"` declares a string default. Omitting the
second string declares a required parameter: every admitted route must supply a
value, even when its template does not reference that parameter. A shared value
in either source can complete a required declaration before routing the command.
Names follow the same grammar as command names, in a namespace local to that
command; each name is declared once. All declarations are structurally checked,
including those in dormant commands.

`${param.mode}` can fill a whole argument or part of one, and can repeat in the
same word. The compiler splits the template first and inserts each value once.
Spaces, quotes, `${prompt}`, `#` and shell punctuation in a value stay literal
contents of that word. An empty whole-word value preserves an empty argument.
`$${param.mode}` is literal text. Parameter references must be declared and cannot
appear in the executable. A parameter named `prompt` is separate from the runtime
slot `${prompt}`. Required values are checked only on admitted routes; an active
binding without routes still validates its template.

A `values` block changes shared parameters without copying a command. For example,
this local delta changes every route using `agent` except explicit route overrides:

```kdl
config {
    values "agent" { param "mode" "quick"; }
}
```

Primary shared values override declaration defaults; local shared values override
primary ones by parameter name. `unset "mode"` removes the shared override and
exposes the declaration default, or a missing-value error if it has none. Empty
strings are values, not removals. Unsetting an absent or undeclared parameter is
legal; only surviving assignments must name declared parameters. Each document
has at most one `values` block per command, and a block mentions each parameter
once, whether assigning or removing it. Blocks require a command name and children;
`param` takes a name and value, and `unset` takes a name.

Every effective values target and surviving assignment validates, even without
routes. Invalid earlier assignments can be overwritten or removed locally.
Values alone do not activate a dormant template or authorize a kind. Inspection
retains assignment and removal histories; a resolved parameter identifies its
declaration and winning assignment, and a word includes all contributing origins.

A route block supplies per-kind exceptions. This local delta changes only `impl`:

```kdl
config {
    route "impl" { param "mode" "careful"; }
}
```

Route values override shared values, even when a shared assignment comes later.
`unset "mode"` in the route removes that exception, exposing the final shared
value or declaration default. A block mentions each parameter at most once.
A route may give its binding and parameter block together, or omit the binding
to patch a personally targeted kind. A personal parameter-only route without a
personal target fails the entire load, even for another requested kind; a local
target cannot repair it. Local parameters can complete required declarations.
Local-only routes remain non-admitted and do not resolve their references or
parameters, so they cannot prevent a valid personal kind from running.

Changing bindings preserves route exceptions, which must fit the final command's
schema. Use `unset` to remove an old-schema parameter; absent removals are legal.
A whole flat template replacement clears route overrides and records their resets.
Switching a literal route to a binding starts a fresh map before applying its own
parameters. A parameter-only patch on a route still literal at the end is an error.
Inspection retains overwritten assignments, removals and resets, including names
absent from the final schema. Flat entries and wrapper routes share the same
per-document key namespace, including parameter-only patches.

Personal profiles may coexist with base commands while remaining inactive:

```kdl
config {
    profile "experiment" {
        include "unfinished-base"
        bind "lead" "unfinished-command"
        route "impl" "lead"
    }
}
```

Each profile requires one valid name and a child block containing only
`values`, `bind`, `route`, and at most one `include` list. Includes take zero or
more valid profile names; repeated names are allowed. Patch shapes and duplicate
settings are checked even in inactive profiles, with a separate namespace for
each profile. Unknown include/reference names, cycles and incomplete parameters
in inactive profiles do not affect base resolution. Such profiles do not
admit keys for local overrides. Local deltas cannot define profiles or redeclare
command schemas.

Grove always loads the personal file and at most one admitted local delta.
A local `config { select "daily" "experiment"; }` replaces the personal default
list; it does not append. `config { select; }` explicitly disables profiles.
If the local file has no selection, Grove inherits the personal `select`; with
neither declaration it uses only the base and direct local overrides. Local
values apply after the selected personal profiles and may complete their required
parameters. An unknown selected profile or invalid active composition refuses
use without falling back to another list.

Configuration is reloaded before each tree transition and again before launch.
Edits affect a subsequent session; the running child's command stays unchanged.
The adapter exposes the same snapshot through `SessionConfig::inspect()` and
source-attributed records through `grove_loop::Error::diagnostics()`. Discovery
and admission failures retain the candidate path without inventing a byte span.

The generic `Catalog` API captures one optional `select` per document, with its
source span, ordered names and repeated entries intact. Its arguments are zero
or more valid profile-name strings; properties, types and child blocks are
invalid. Absence differs from a present empty list. Catalog never chooses policy:
`resolve` uses only its explicit `Selection`, and `Templates::load` ignores both
declarations and resolves an empty selection. Explicit selections fold the personal
base, then each selected profile's includes left to right before its own patch,
then the local delta. Repeated selections and diamond includes reapply every
occurrence. Unknown selected/included names report `unknown_profile`; active-stack
cycles report `include_cycle` with a closed chain and include spans.

Later assignments replace earlier assignments within a setting. Route parameters
remain more specific than shared command values regardless of application order.
`unset` removes that scope's override; changing a literal route to a binding starts
an empty route parameter map, with the reset retained in inspection. Only final
references and surviving assignments need to resolve, so partial selected profiles
can supply routes, bindings and values separately. Legacy flat checks stay eager.

Personal targets are checked after all selected profiles and before local patches.
A personal parameter-only route without a target fails the entire selection with
`missing_target`, even if a local target exists. Local values can complete required
parameters of an admitted route. Every effective binding/template and shared values
assignment validates even without routes; required parameters are checked per
admitted route. There is no implicit route or catch-all.

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

A local flat template replaces a whole target. Named local routes replace binding
names, and local bindings redirect every route using them. Settings the delta
does not mention retain their personal values. Both documents receive structural
validation, followed by eager flat checks and effective named-reference checks.

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

## The methodology is a prerequisite Grove does not supply

`${prompt}` names one `grove-<kind>` skill and carries no methodology of its own
beyond grove's signalling contract, so a session that cannot reach that skill is
told the loop, the kinds and the verbs by nothing. Installing the `grove` plugin
is therefore a genuine prerequisite of a working configuration — see
[Install the skill plugins](../README.md#install-the-skill-plugins).

**Grove neither installs it nor checks for it.** Every bare `grove` invocation
used to sweep an embedded copy of the methodology into `~/.claude/skills/grove`,
`~/.codex/skills/grove` and `~/.pi/agent/skills/grove` before taking ownership of
a working tree, and report loudly when no such destination existed. Grove writes
no skill directory now: the methodology ships as a plugin with its own install
route, the registry that named those three destinations is gone, and so is the
report.

The cost is recorded rather than argued away. A harness with a skill-loading
affordance is unaffected — it resolves the named skill or says it cannot. A
harness without one has lost its fallback: nothing puts the methodology in front
of a session that does not go and get it. If that turns out to matter — a session
that cannot reach the methodology by the affordance alone — the question to
reopen is delivery, not the registry.

## Validation and diagnostics

Loading is all-or-nothing. A successful load proves the file exists, is readable,
parses as KDL and obeys the structural grammar and duplicate rules. Every flat
template and every named template reached by an effective binding obeys the
executable and substitution rules above; unused named definitions stay dormant. It proves nothing about which kinds
are *present* — that question is asked per kind, when the kind is used.

A missing file names the exact path. A KDL syntax error names the path with its
line and column.

Diagnostics aggregate across both explicit files in primary/overlay and source
position order. Structural errors (including duplicate declarations) are reported
first. Once structure passes, invalid templates are reported together. Malformed
nodes do not produce downstream slot errors. A syntax failure in one file does
not hide an independent structural error in the other.

Library consumers can read `ConfigError::diagnostics()` for stable categories,
messages, remedies, affected names and source locations. A `SourceSpan` is a
zero-based UTF-8 byte range with an exclusive end. Duplicate reports carry the
first declaration as primary and the other declarations as related spans.
Unreadable files retain their source path without a span. Caller-supplied
selection, vocabulary and runtime errors have no invented source range; runtime
errors retain the winning template's source and key.

The reader reports `source_read`, `kdl_syntax`, `shape`, `duplicate`,
`invalid_template`, `unknown_reference`, `unknown_profile`, `include_cycle`,
`missing_target`, `missing_parameter`, `unknown_parameter`, `unconfigured_key` and `invalid_value`.
Runtime slot failures and invalid vocabulary use `invalid_value`; these names
are consumer slots, not configuration parameters. Binding, command and parameter fields remain empty for flat declarations.
Unknown selections carry their selection occurrence and zero-based list index. Unknown external
profiles are named in the message and carry no source unless the caller supplies
an origin. Semantic diagnostics retain the selected occurrence/include chain.
Human inspection commands remain pending.

Library consumers can call `Templates::inspect()` to borrow an owned explanation
of the captured resolution. Named commands include their binding and command
names, contributing route/binding/template origins, and replaced target histories. It lists sources in primary/overlay order,
admitted commands and non-admitted keys in name order, and each route target's
assignment history, including the primary template replaced by an overlay.
Assignments and origins follow application order, with source order within each
patch. Occurrence IDs and parent links distinguish repeated applications of the
same declaration; contributing words reference the winning occurrence. Target histories
are listed by route key, then binding name. Origin and history IDs index the corresponding response arrays.
Each word references the winning whole-template declaration's byte span.
Spans address the original captured UTF-8 contents, even after a path changes or
disappears. Paths remain native `PathBuf` values. No binding, parameter or profile
activity is invented for flat commands.

Inspection words use the same `CompiledWord::Literal` and `CompiledWord::Slot`
representation as expansion. Slots remain symbolic names; filling them with the
same native runtime values gives the same argument vector as `expand`. Inspection
records cannot construct an `Argv`, and reading them launches nothing. The
convenience loader and Catalog resolved with an empty Selection expose equivalent
views. This is a library API; the `grove config show` CLI remains planned.

For example, a duplicate declaration produces a human report such as:

```text
invalid configuration at ~/.config/grove/config.kdl:
  - ~/.config/grove/config.kdl:14:1: duplicate key `impl`; declarations at ~/.config/grove/config.kdl:14:1, ~/.config/grove/config.kdl:31:1
  Keep one declaration per key in each document.
```

Grove prints the absolute path; `~` stands in for your home directory here.
The report says *key* because the generic runner does not know session kinds.

No diagnostic silently fills a target or falls back to another kind.

### When a missing kind is reported

Grove asks whether kind K resolves at the two moments it commits to K, and not
before: **when it writes a leaf of kind K** — `grove-llm leaf-add`,
`leaf-insert`, `leaf-decompose` and `root-init` — and **when it
launches K**. An add given several kinds asks about every one of them, before
any of them lands. The check runs before the tree is mutated, so a refusal leaves the
task tree byte-identical.

The driver's automatic fresh-root bootstrap currently checks a missing
`requirements` kind at launch, after creating the root. Repairing that earlier
admission boundary remains pending; invalid active compositions already fail
before root creation.

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

Grove checks both documents structurally and validates the active composition
before **every** task-tree mutation: the driver's own (root
initialization, partial-root recovery, and finish-leaf materialization) and every
`grove-llm` verb that writes a leaf. It reads them again immediately before every
launch. Nothing is cached between loop iterations or between verbs, so editing
either file affects the next session.

A failed pre-mutation read leaves a rootless, partial, or complete tree
byte-identical. If either file becomes invalid after a mutation but before the
launch read, that mutation stays as resumable tree state and no session launches.
Either way an existing selected leaf remains live and resumable.

A configuration that declares no template for a kind you never reach is neither
invalid nor a problem. Malformed flat templates remain eagerly invalid even for unused kinds. Named
commands and profile references receive semantic validation when active; inactive
profiles receive structural checks. Presence is about the kind in hand.

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

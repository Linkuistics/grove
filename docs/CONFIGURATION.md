# Grove Configuration

This reference describes modular command reuse, parameters and
ordered profile composition with workspace selection. The [modular design](specs/modular-configuration.md)
specifies the resolution and delivery contract. `grove config show [--kind KIND]
[--json]` explains active policy; `grove config examples` installs the validated
[example set](examples/modular-configuration/README.md) without activating it.
Flat top-level commands are rejected. Move templates into command definitions and
connect them with bindings and routes inside `config { ... }`.

The generic runner captures sources in an owned `Catalog` and resolves an explicit
`Selection` into `Templates`. Grove chooses the local declaration when present,
otherwise the personal default, otherwise an empty selection. Captured resolution,
inspection and expansion do not reread files. Structured diagnostics and library
inspection cover named reference chains, parameter histories and
every contributing origin in a resolved word.
Runtime vocabulary names beginning with `param.` are reserved;
Grove's existing four slots are unaffected.

NUL is refused before an argument vector is returned. Command templates receive
that check when an effective binding activates them.
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
replaces shared values, route parameters, binding targets or route targets. Nothing else does — task
files, command-line flags, and environment variables neither override nor
supplement your configuration, and Grove never creates or edits either file. It
cannot choose your model or approval policy for you.

Every admitted kind resolves to a complete command before launch. Named routes
can use a binding selected by the local file and a command defined in personal
policy; inspection retains those contributing declarations.

## Installing examples

Run `grove config examples` to place five `.example.kdl` files and
`CONFIGURATION.examples.md` in `~/.config/grove/`. It needs neither a workspace
nor valid active policy. Destinations are fixed; active `config.kdl` and
`.grove.kdl` files are never written. The installed instructions explain
wrapper/model/policy choices and how to activate a local sample after ignoring
its destination. The samples cover both lead/review arrangements, includes,
parameter experiments, local selection and overrides, and an inactive unfinished
profile. They are illustrative policy, not built-in harness support.

All destinations are checked before writes. Matching regular files remain
untouched; different contents, symlinks, directories and unreadable entries are
conflicts. Missing files are exclusively created. A later create/write failure
can leave completed or partial new files: stderr names them and the failure,
and nothing is deleted as cleanup. Resolve conflicts yourself before retrying;
there is no force option, destination option or automatic ignore edit.
Success reports paths on stdout and exits 0; conflicts/I/O failures exit 1;
invalid usage exits 2. See the [worked command](USAGE.md#usage-configuration-examples).

## The file

The file contains one `config { ... }` wrapper with command definitions,
bindings and explicit routes. Profiles and parameters are optional. Empty and
comment-only documents remain valid but admit no kinds. Other top-level nodes
are rejected, including former flat keys named `config`, `command` or `select`.

A minimal personal policy is:

```kdl
config {
    command "agent" "my-agent ${prompt}"
    bind "lead" "agent"
    route "impl" "lead"
}
```

`my-agent` is an illustrative executable supplied by the owner. Add an explicit
route for each kind you use; Grove checks presence when it needs that kind.
The [packaged examples](examples/modular-configuration/README.md) show a complete
policy with separate commands for implementation, review and research.

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
why the packaged policy sends them elsewhere. `finish` is the driver-reserved
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

A nonempty configuration contains one `config` wrapper. Define commands in the personal
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
`config { bind "lead" "reviewer"; }`. Only personally targeted kinds
are admitted; a local-only route remains unconfigured.

Command and binding names start with a lowercase ASCII letter and contain only
lowercase letters, digits and single interior dashes. Each namespace is separate.
Within each scope each command, binding and route appears once.
Top-level declarations outside the wrapper are rejected, including a flat
`config "runner ${prompt}"` beside a valid wrapper. Properties and type annotations are invalid.
Commands accept `param` declarations; routes accept `param`/`unset` patches or
empty blocks. Bindings have no block.

Named templates are shell-word split before dollar scanning. Runtime slots still
occupy whole arguments. `$$` escapes one dollar: `$${prompt}` is literal text,
so a separate `${prompt}` must satisfy its required count. Unknown or unterminated
substitutions fail. The executable must be nonempty and literal. No shell
evaluates the template.

All structural errors fail loading. Effective bindings and admitted route
references validate after local replacement, so a replaced bad reference does
not poison the result. Every effective binding validates its command template,
even with no routes; a definition without a binding remains dormant.
Failures identify their source and names.

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
Inspection retains overwritten assignments and removals, including names absent
from the final schema. Routes and parameter-only patches share the same
per-scope key namespace.

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
For example, changing a workspace's selected profiles and a personal shared
parameter while its child runs leaves that process and its arguments intact.
After it signals relaunch, the next child uses the new selection and value.
If the active configuration becomes invalid between transition and launch,
Grove refuses the launch; it does not roll back an already admitted transition.
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
`unset` removes that scope's override. Only final references and surviving
assignments need to resolve, so partial selected profiles can supply routes,
bindings and values separately.

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
config {
    route "impl" "review"
    route "review-impl" "lead"
}
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

Local routes replace binding names, and local bindings redirect every route using them. Settings the delta
does not mention retain their personal values. Both documents receive structural
validation, followed by effective reference and template checks.

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
config {
    command "agent" "runner --tag '#build' ${prompt}"
    bind "lead" "agent"
    route "impl" "lead"
}

// rejected — everything from `#` onward would be dropped
config {
    command "agent" "runner --tag # build ${prompt}"
    bind "lead" "agent"
    route "impl" "lead"
}
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
parses as KDL and obeys the structural grammar and duplicate rules. Every
command template reached by an effective binding obeys the
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
are consumer slots, not configuration parameters.
Unknown selections carry their selection occurrence and zero-based list index. Unknown external
profiles are named in the message and carry no source unless the caller supplies
an origin. Semantic diagnostics retain the selected occurrence/include chain.
Human inspection uses `grove config show`, described below.

Library consumers can call `Templates::inspect()` to borrow an owned explanation
of the captured resolution. Named commands include their binding and command
names, contributing route/binding/template origins, and replaced target histories. It lists sources in primary/overlay order,
admitted commands and non-admitted keys in name order, and each route target's
assignment history, including personal targets replaced by an overlay.
Assignments and origins follow application order, with source order within each
patch. Occurrence IDs and parent links distinguish repeated applications of the
same declaration; contributing words reference the winning occurrence. Target histories
are listed by route key, then binding name. Origin and history IDs index the corresponding response arrays.
Each word references its template declaration and contributing parameter origins.
Spans address the original captured UTF-8 contents, even after a path changes or
disappears. Paths remain native `PathBuf` values.

Inspection words use the same `CompiledWord::Literal` and `CompiledWord::Slot`
representation as expansion. Slots remain symbolic names; filling them with the
same native runtime values gives the same argument vector as `expand`. Inspection
records cannot construct an `Argv`, and reading them launches nothing. The
convenience loader and Catalog resolved with an empty Selection expose equivalent
views. The human CLI formats this same view through SessionConfig.

### Inspecting the active configuration

```sh
grove config show
grove config show --kind impl
grove config show --json
```

Run from anywhere inside the jj workspace. The report names the personal file
and selected local file, the selection declaration (or implicit empty list),
every include occurrence, admitted kinds in name order and non-admitted local
keys. Each command shows its binding/command chain, winning parameter values,
executable and ordered argument words. Quoted `literal` words escape controls;
`slot <prompt>` is a runtime placeholder, distinct from a literal `${prompt}`.
Origin IDs point to paths and zero-based, end-exclusive UTF-8 byte spans;
history IDs expose overwritten assignments and unset operations.
All provenance tables remain available when `--kind` filters the command list.

Inspection validates the entire active configuration before requiring a kind.
A broken active route cannot be hidden by asking for another kind. Unknown and
non-admitted kinds fail with source-attributed guidance. Reports go to stdout;
errors go to stderr. Exit codes are 0 for valid inspection, 1 for source,
admission or resolution failure, and 2 for invalid usage.

No task tree, selected leaf, driver lease or active session epoch is needed.
Inspection does not launch or probe executables, use a pager, truncate output,
create coordination files, signal completion or change working/configuration
bytes. The existing jj trackedness query may snapshot jj metadata. A held
lease or stale inherited signal does not prevent inspection. Each report covers
one load; later sessions reload sources, so equality with launch requires the
same inputs and runtime context.

<a id="configuration-json"></a>
### JSON records

`--json` emits one object with `schema_version: 1`, `sources`, `selection`,
`profile_occurrences`, `commands`, `non_admitted_keys`, `origins` and `histories`.
The record fields match the public Inspection records documented in the
[module contract](specs/module-decomposition.md). `--kind` filters only commands;
all provenance tables remain intact, so every response-local ID resolves.

Words are `{"type":"literal","value":"text"}` or
`{"type":"slot","name":"prompt"}`. Parameters have already become literal
contents. Assignment values emitted for modular input use `type`: `set` carries
a `value`; `unset` carries no value. Settings use snake_case type tags
and named fields. Source roles are `primary` and `overlay`; absent optionals
are null. IDs and byte offsets are integers.

Unicode paths are strings. Non-Unicode paths use
`{"encoding":"unix_bytes","value":[47,255]}` on Unix or
`{"encoding":"windows_wide","value":[67,58,92,55296]}` on Windows, preserving
bytes or UTF-16 code units respectively. This encoding also applies to paths
in source spans and diagnostics; displayed error prose is not a path encoding.

Failures write one `schema_version: 1` object with a `diagnostics` array to
stderr and nothing to stdout. Each diagnostic carries `category`, `message`,
`source`, `primary`, `related`, `occurrence_chain`, `key`, `binding`, `command`,
`parameter` and `remedy`. Configuration refusals retain the resolver's records.
Other inspection failures use category `inspection`, with absent source fields;
parser failures use `usage` and exit 2. `--json` (including a malformed `--json=…`) before the
`--` terminator requests structured errors even if parsing fails before reaching
it. Successful explicit help/version requests keep clap's human output.

For example, a duplicate declaration produces a human report such as:

```text
invalid configuration at ~/.config/grove/config.kdl:
  - ~/.config/grove/config.kdl:14:1: duplicate declaration `impl`
  Keep one declaration per name in this namespace and document.
```

Grove prints the absolute path; `~` stands in for your home directory here.
The first declaration supplies the primary location. The structured diagnostic
retains the second declaration in `related`; the human message does not list it.

No diagnostic silently fills a target or falls back to another kind.

### When a missing kind is reported

During lifecycle operations, Grove asks whether kind K resolves at the two
moments it commits to K: **when it writes a leaf of kind K** — `grove-llm leaf-add`,
`leaf-insert`, `leaf-decompose` and `root-init` — and **when it
launches K**. An add given several kinds asks about every one of them, before
any of them lands. The check runs before the tree is mutated, so a refusal leaves the
task tree byte-identical. An explicit `grove config show --kind K` also checks
that K resolves, after validating all active policy, without mutation or launch.

The driver's automatic fresh-root bootstrap also requires an active personal
`requirements` route before creating `.grove/`. It checks while holding the lock
that established the root's absence. An inactive personal profile or local-only
route cannot authorize creation. An existing tree needs no `requirements` route
unless that kind is selected for launch. Invalid active compositions fail before
either initialization or launch.

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
launch error naming the selected kind, the executable, and **the personal file
holding the resolved command definition**. Inspection records any local binding,
route and parameter contributions; a wrapper's own failures stay opaque by design.

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
invalid nor a problem. Unsupported top-level declarations fail structurally. Named
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

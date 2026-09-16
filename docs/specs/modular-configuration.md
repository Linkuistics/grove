# Modular configuration

## Purpose and scope

Grove configuration names reusable commands, composes differences through
profiles, and resolves an explicit command for each configured session kind.
This is the design contract for that configuration area. The running reader
implements captured Catalog/Selection resolution, diagnostics and inspection for
legacy flat commands and parameterized wrapper commands, bindings and routes.
Parameter declarations/defaults, safe embedded substitution, multi-origin words,
named dollar escaping and effective-reference validation are implemented.
Shared and route values/removals fold across primary/local sources with retained
histories, literal resets and personal target authorization. Optional selection
declarations are captured with source spans; the convenience loader ignores them
and Grove refuses either declaration before use, including explicit empty lists.
Inactive personal profiles now pass structural validation without affecting base
resolution. Explicit selection of a known profile still refuses until occurrence
composition lands. Profile composition, human
inspection commands, example delivery and Grove
selection policy remain pending; the [reference](../CONFIGURATION.md) describes
the current boundary.

The authority and execution constraints are owned by
[complete session configuration](../adr/complete-session-configuration.md) and
[the untracked configuration delta](../adr/untracked-configuration-delta.md).
The [module contract](module-decomposition.md#7--the-runner) assigns the seams.
This spec owns the grammar, composition, validation and inspection semantics.

The editable [visual document](../design/modular-configuration/index.html)
groups the design into command reuse, composition, and validation/inspection.
Its diagrams are views of the contracts below, not additional rules.

## Command reuse

A **command definition** holds one template and its parameter declarations.
A **command binding** names a command definition. A **kind route** names a
binding, with optional parameter overrides for that kind. There are no built-in
bindings: `lead` and `review` are ordinary names chosen by the example's author.

That extra reference has a specific purpose. A route list can stay fixed while
`codex-led` and `claude-led` swap two bindings; the two full templates each exist
once. A one-kind experiment changes one route. A parameter-only experiment
changes one value for a command, and every route using it inherits the change
unless that route explicitly overrides the parameter.

Three namespaces remain separate: command names, binding names, and profile
names. Parameter names belong to their command definition; route overrides are
checked against the command finally reached by that route. These names use
lowercase ASCII letters followed by letters, digits, or single interior dashes.
They are nonempty, begin with a letter, and have no reserved words. The generic
runner's route keys remain opaque nonempty strings; Grove validates a session
kind when using it under its existing kind grammar.

### Grammar

The language remains the currently supported KDL dialect. No properties or type
annotations are admitted except where the grammar explicitly allows them;
this grammar needs neither. Strings are KDL strings, not environment values.

The document has legacy flat entries and, optionally, one `config` node with
zero arguments and a child block. A flat entry has its existing shape: the
node name is the key and its single string argument is the complete template.
Shape distinguishes the forms, so a legacy kind named `config`, `profile`,
`command`, or `select` keeps working. Even a flat `config "..."` and the one
`config { ... }` wrapper may coexist. No other child-bearing top-level form is
accepted. A malformed wrapper is diagnosed, never retried as another syntax.

| In the wrapper | Arguments and children | Personal | Local |
|---|---|---|---|
| `command` | Name, template; optional `param` declarations | Yes | No |
| `profile` | Name; block containing one optional `include` and patch nodes | Yes | No |
| `select` | Zero or more profile-name strings; no children | Default list | Replacement list |
| `values` | Command name; block of parameter assignments/removals | Base patch | Final patch |
| `bind` | Binding name, command name; no children | Base patch | Final patch |
| `route` | Key, optionally binding name; optional parameter assignment/removal block | Base patch | Final patch |

A profile's patch nodes are `values`, `bind`, and `route`, with exactly these
shapes. Its optional `include` lists zero or more profile-name strings and has
no children. Its textual position does not change its precedence: includes
always apply before the profile's own patch. Profiles cannot define commands,
define nested profiles, or select another default list.

Inside a command, `param` has a name and an optional default string. With no
default it declares a required parameter. Inside `values` or `route`, `param`
has exactly a name and a value string; `unset` has exactly a name and removes
an override at that scope. Neither has children. An empty string is a value;
it is different from an absent value. Command definitions do not admit `unset`.

A `route` with no binding is a parameter patch, completed by another active
personal route for that key. Bindings contain no parameter map: shared values
belong to the command they configure. That keeps the parameter namespace stable
when a profile changes a binding to a different command.

The following is grammar documentation, with illustrative wrapper executables:

```kdl
config {
    command "agent-a" "my-agent-a -c model_reasoning_effort=${param.effort} ${prompt}" {
        param "effort" "medium"
    }
    command "agent-b" "my-agent-b --effort ${param.effort} ${prompt}" {
        param "effort" "medium"
    }
    profile "routes" {
        route "impl" "lead"
        route "design" "lead"
        route "review-impl" "review"
    }
    profile "a-led" {
        bind "lead" "agent-a"
        bind "review" "agent-b"
    }
    profile "b-led" {
        bind "lead" "agent-b"
        bind "review" "agent-a"
    }
    profile "daily" { include "routes" "a-led"; }
    profile "high-effort" {
        values "agent-a" { param "effort" "high"; }
    }
    select "daily"
}
```

A workspace can select `daily` and `high-effort`, select `routes` and `b-led`,
or add a direct `route "impl" { param "effort" "low"; }` patch. The complete
[example set](../examples/modular-configuration/README.md) supplies both lead
arrangements, explicit kind routes, and the local-file variants.

### Structural uniqueness

Within one document there is at most one wrapper, one `select`, and one
definition per command/profile name. Within one patch there is at most one
`values` per command, one `bind` per binding, and one `route` per key. A command
declares each parameter once. A parameter patch mentions a name at most once,
whether by `param` or `unset`. Each profile has at most one `include` node.
Duplicate entries in an include or selection *list* are permitted occurrences,
not duplicate declarations.

Flat entries and wrapper-level routes share a document's base route namespace:
declaring the same key in both is an error, even for a parameter-only patch.
Across different profiles or layers the same named setting is an intentional
override. Document ordering does not settle duplicate declarations.

## Composition

Grove chooses the local source under the delta ADR, then chooses the profile
list: use local `select` if present, otherwise personal `select`, otherwise an
empty list. Local `select` with no arguments explicitly disables the personal
default selection. It leaves the personal base in force.

Resolution performs one ordered fold:

1. Start with the personal flat entries and wrapper-level patch.
2. For each selected profile, visit its includes recursively, left to right,
   then apply its own patch.
3. Remember the keys with an explicit target in that personal result.
4. Apply the chosen local file's flat entries and wrapper-level patch.
5. Resolve references, validate the active result, and compile each admitted
   route into a command with provenance and identified runtime slots.

A profile is applied **once per occurrence**, including repeated selection and
diamond-shaped include graphs. Detect a cycle using the active recursion stack.
A global visited set would change the contract: if `left` includes `base` and
overrides it, then `right` includes `base`, selecting `left`, `right` reapplies
`base` during `right`. The earlier override survives only if the later traversal
does not overwrite that setting. An unknown included name or a cycle cannot be
hidden by a later override; expansion must succeed before a fold is available.

### What later values replace

| Setting | Override unit |
|---|---|
| Binding target | One command name |
| Route target | One binding name, or a whole legacy literal template |
| Shared command parameter | One parameter name in one command's `values` map |
| Kind parameter | One parameter name in one route's map |
| Selected profiles | The complete list, chosen before folding |
| Command definitions | Immutable catalog entries; profiles/local files cannot redefine them |

For one route, parameter precedence is command declaration default, then the
final command `values` map, then the final route parameter map. Later-layer wins
applies **within each named setting**. A later shared value does not displace a
route's explicit override: those are different settings. `unset` deletes only
the addressed override, exposing the lower scope's value. It does not erase a
command's parameter declaration or default. Unsetting an absent override is a
no-op, including a name outside the final command's schema. Only surviving value
assignments must name declared parameters; this permits removal of an old
command's parameter when a route changes bindings.

Changing a route from one binding to another preserves its parameter overrides;
they must fit the final command's declared parameters. Use `unset` when the new
command has a different parameter interface. A whole legacy template replaces
the route and clears its parameter map. Setting a binding target on a previously
literal route starts an empty parameter map before applying that patch's own
parameters. A parameter-only patch on a route still literal at the end is an
error. Legacy templates are not implicitly converted to reusable commands.

Only explicit targets in the **active personal result** authorize keys for local
override. A target declared solely in an unselected profile does not authorize
a local route. A local selection can activate that profile, making its personal
route available. A primary parameter-only route without a personal target is
an active semantic error (`missing_target`) that fails the entire resolution,
including inspection and launches of other kinds. Report the personal patch's
span, key and selected occurrence chain, with a remedy to add a personal target
or deselect/remove the patch. It is not a `non_admitted_keys` entry and no
Templates snapshot is returned. A local target cannot repair that absence.
Local values may complete missing *parameters* of a personally targeted route.

For compatibility, a valid local-only route is retained as a non-admitted key,
as today's overlay-only keys are. It is excluded from resolved commands, appears
in inspection with the reason, and `require`/`expand` for it fails naming the
personal file. It does not independently block another configured kind. Local
definitions never enlarge the command/profile catalog. There is no implicit
route for any key, whether a profile happens to mention it or not.

## Validation and expansion

Loading has three distinct scopes. Every failure is an error before launch or
tree mutation, with no fallback to a different profile list or local source.

| Scope | Checks |
|---|---|
| Both complete documents | KDL syntax, recognized shapes, argument types, names, duplicates, permitted children, and personal/local restrictions |
| Every legacy flat entry | All current template checks, even if overridden or its kind is not used |
| The selected combination | Include references/cycles; effective binding and route references; effective parameter assignments; active command template rules; complete parameters for each admitted route |

Unselected profiles are structurally checked but their references, includes and
parameter completeness are not evaluated. Unused command definitions retain
their declared parameter shapes; their template strings are not shell-split or
semantically checked until active. This is how an unfinished experiment can
coexist with a working selection. Legacy eager template checking remains a
deliberate compatibility rule rather than silently weakening old files.

After folding, validate every effective binding's command reference and each
referenced command template, even if no route uses that binding. Validate every
effective `values` target and its parameter names against the catalog. Check
missing required parameter values only when instantiating admitted routes.
Personal route patches must have personal targets; their final references and
parameters may be completed or replaced by subsequent profiles or local values.
No binding or `values` entry alone creates a route.

Other than include edges and the eager legacy checks, overwritten assignments
do not have to resolve. For example, a selected profile's misspelled binding
target can be replaced by a later correct target. Inspect its history to find
it; only the effective target participates in reference validation. Removals
remain in provenance but do not become value assignments. Non-admitted local
routes receive document checks (and eager template
checks when flat), but no binding/parameter resolution until they become admitted.
This separates final-combination validity from per-profile completeness.

### Two substitution namespaces

The runner first tokenizes a named command's template using the existing
shell-word grammar, including the refusal of an unquoted comment-starting `#`.
It then compiles each word into literal fragments, parameter references, or a
whole runtime slot. Word zero is nonempty and literal: neither namespace may
supply any of its contents.

* `${param.name}` addresses an author-declared parameter. It may fill a whole
  argument or appear multiple times inside an argument. Any referenced parameter
  must be declared. Every declaration without a default requires a value on
  every instantiated route, even if the template does not use that parameter.
* Runtime slots keep the consumer-supplied vocabulary. For Grove, `${prompt}`
  occurs exactly once after word zero, and `${session_name}`, `${worktree}`, and
  `${repo}` each occur at most once. Each occupies a whole word. A parameter
  named `prompt` is distinct from the runtime slot `prompt`.
  Catalog reserves the `param.` prefix and rejects a runtime vocabulary that
  collides with it, including through the `Templates::load` convenience. Grove's
  existing vocabulary has no such names; its legacy templates remain unchanged.
* In named templates only, `$$` escapes one literal dollar during compilation.
  Scanning is left to right: `$${param.name}` is literal text, not a parameter.
  Unknown or unterminated `${...}` forms are errors. Ordinary dollar text has
  no shell meaning. Legacy flat strings retain the existing scanner unchanged.

Parameter values and runtime values are opaque. Insert them once, without
re-tokenizing, recursively substituting, interpreting quotes, or evaluating a
shell. A value containing spaces, quotes, `${prompt}`, `#`, or shell punctuation
stays in the one original word. An empty whole-word value creates an empty
argument rather than deleting it. Reject NUL in active template words, resolved
parameter values, and runtime values before spawn. Runtime paths keep native OS-string
fidelity; only configuration strings are Unicode text.

This preserves the existing runtime-slot contract while making
`model_reasoning_effort=${param.effort}` a single safely variable argument.
Parameter names do not imply any understanding of a model, harness or policy.

### Diagnostics

KDL syntax errors identify their source and location. When syntax permits
inspection of the documents, aggregate independent structural errors across the
readable sources. After those pass, aggregate independent active semantic errors;
do not invent downstream missing-value errors from an object already malformed.
Reports are deterministic, ordered by source role and source position, then key
where one declaration affects several routes.

Each diagnostic contains a stable category, message, primary source span,
related spans where relevant, and an actionable remedy. Semantic errors also
name the selected occurrence/include chain and the affected key, binding,
command or parameter as applicable. Categories distinguish source read/admission,
KDL syntax, shape, duplicate, unknown profile, include cycle, unknown reference,
missing target, missing parameter, unknown parameter, invalid template, invalid
value and unconfigured key. Codes are `source_read`, `source_admission`, `kdl_syntax`,
`shape`, `duplicate`, `unknown_profile`, `include_cycle`, `unknown_reference`,
`missing_target`, `missing_parameter`, `unknown_parameter`, `invalid_template`,
`invalid_value` and `unconfigured_key`. CLI usage errors use `usage`.
When no source location exists (for example an unreadable file, invalid runtime
value, or externally supplied selection), the span is absent rather than invented;
retain the source path and affected names wherever known.
A cycle report shows the closed chain and the include spans;
a missing parameter report names its declaration and the affected route.

Provenance records every assignment's source span, profile occurrence path and
order, retaining overwritten assignments and `unset` operations. A resolved
argument can have several contributing origins: template literal fragments,
parameter declaration/default, shared value, and kind override. A single
winning file is insufficient. A launch failure names the key, executable and
template source, with contributing settings available through inspection.

## Inspection

`grove config show [--kind KIND] [--json]` is an operator-readable, read-only
verb as well as an agent interface. It resolves the current workspace and the
same personal/local configuration as the driver, without requiring a task tree,
an active session epoch, a driver lease, or a selected leaf. It creates no
coordination directory, writes no completion signal, and changes no configuration
or working-tree files. It reuses the existing VCS trackedness check, which may
snapshot jj metadata before answering. This metadata effect is explicitly
permitted for inspection; using a stale working-copy answer to avoid it would
give inspection a different admission decision from launch.
The human CLI's `config` group exposes `show` and `examples` in help, with
invocation examples and exit codes. Both dispatch before driver lease acquisition
or loop startup. Inspection resolves the workspace only to locate its sources;
example delivery needs no workspace. Neither admits an ambient session epoch;
a stale inherited signal path has no effect. This follows the existing human
and agent audience split; no configuration verbs are added to `grove-llm`.

Without `--kind`, show sources, the selection's origin, the selected list,
include expansion occurrences, every admitted kind in name order, and
non-admitted local keys. With `--kind`, perform the same complete load and
validation, then require and show that key; filtering cannot hide a broken
active configuration.

Show the command/binding reference chain, final parameter values and their
override histories, template origin, literal executable and ordered argument
words. Runtime slots stay explicitly marked placeholders: inspection does not
invent a mandate or claim its display string is the final prompt. Inspection
and launch use the **same compiled word representation**; filling its slots with
the same context produces the same `Argv`. Do not reconstruct a shell command
and split that display again.

An inspection describes one load of its sources. It does not pin those files
for a later launch: the driver reloads them at its existing load points, so
inspection/launch equality is conditional on unchanged inputs and runtime
context. Source capture reads each participating file once per load; no
cross-file transaction or live-child reconfiguration is promised.

JSON success is one object on stdout with `schema_version: 1`, `sources`,
`selection`, `profile_occurrences`, `commands`, `non_admitted_keys`, `origins`
and `histories`.
`commands` is an array of records carrying `key`, `binding` (nullable for a
legacy literal), `command` (likewise nullable), `parameters`, `words`, and
`origins` and `histories`. Each word record has `word` and `origins`; `word` is
a tagged `literal` value or `slot` name, not an ambiguous
`${...}` string; configured parameters have already become literal contents.
Origin IDs reference the top-level origin records; history IDs reference the
top-level assignment histories. The record fields are the `Inspection` types in
the module contract, serialized with snake_case field names and enum tags.
`CompiledWord` is `{"type":"literal","value":"..."}` or
`{"type":"slot","name":"..."}`. `AssignmentValue` uses `type` with
`set` or `literal_template` (each with `value`), `unset`, or `reset`. A route
target's binding name uses `set`; a legacy whole template uses `literal_template`,
even when their text is identical. `Setting` uses `type` with the
snake_case variant name and its named fields. `SourceRole` is `primary` or
`overlay`. Optional values are JSON null; IDs and source offsets are integers.
Native strings outside Unicode must have a lossless tagged encoding,
not lossy replacement; inspection normally leaves native runtime values symbolic.

Exit 0 means a valid inspection; exit 1 is source/configuration/resolution
failure, and exit 2 is invalid CLI usage. In JSON mode errors are one
`schema_version: 1` object on stderr with a `diagnostics` array and empty stdout,
including argument errors when `--json` was requested. Human mode uses stdout
for the report and stderr for actionable errors. There is no profile override
flag, pager, truncation, executable availability probe, or harness invocation.

## Example delivery

`grove config examples` installs the packaged example set beside the personal
configuration. It does not load active policy and works outside a workspace.
The destination and filenames are fixed and reported; the set includes a new
personal-form sample and separate local-selection/override samples with `.example`
in their names, plus instructions. It never creates or overwrites the active
personal file or a workspace's active delta.

Preflight every destination: identical regular files are left untouched;
different contents, symlinks, directories or unreadable entries are conflicts
reported before writing any new file. Create missing files exclusively, so a
race cannot turn creation into replacement. If a later create/write fails,
report the files already created and the failure; do not claim atomic batch
installation or delete an existing personal file during cleanup. A repeat is
safe and leaves matching files unchanged. Exit 0 means the whole set is present
with the intended bytes; exit 1 reports failure or conflicts; exit 2 is usage.
There is no force/overwrite option. Grove writes no ignore rule.

The sample policy uses illustrative wrappers, author-supplied model placeholders
and explicit routes for the methodology's examples. Those choices are sample
data, never a runtime registry or a list of kinds enforced by Grove. Its readme
explains where the owner supplies executables and approval/sandbox policy, how
to activate a local example only after ignoring its destination, and that only
the chosen local candidate is read alongside global policy.

The packaged bytes must be the repository example bytes, not a second hand-kept
copy. The example readme is self-contained when installed as
`CONFIGURATION.examples.md`; it carries no repository-relative links or temporary
delivery-status text. These files are design artifacts until implementation
validates them; that delivery status belongs here, outside the packaged set.
Tests resolve the personal sample alone and with every local sample through
the production reader, then use fake executables to check the promised argv.
An intentionally inactive unfinished profile is exercised both unselected
(success) and selected (actionable error). The implementation session must also
deliver the tested examples to the requested personal configuration directory;
writing the installer alone does not satisfy example delivery.

## Module interfaces

The generic runner owns a parsed **Catalog** and a resolved **Templates**
snapshot. Catalog loading accepts explicit primary and optional overlay sources
plus a slot vocabulary. It checks document structure and eager legacy rules and
exposes each source's optional selection declaration with origin information.
It has no home-directory lookup, workspace discovery, kind enumeration or VCS.

The consumer chooses one selection declaration (or the empty selection) and
passes it to Catalog resolution. Resolution performs the fold, enforces primary
key authorization, validates active references/templates/parameters, and returns
Templates with one origin-bearing compiled command per admitted key. It performs
no I/O after source capture. Grove owns the default-versus-local-list choice;
the generic library knows lists and ordered patches, not Grove's selection policy.

Templates exposes `keys`, `require`, `expand` and a structured inspection view.
`expand` alone constructs `Argv`. No public constructor accepts an arbitrary
argv as though it had passed configuration validation. Existing callers of
`source(key)` receive the template text's source only; that accessor's documented
meaning narrows, and provenance-aware diagnostics use the inspection view.
`Templates::load` is exactly `Catalog::load` followed by resolution with an empty
explicit selection. It accepts wrappers and applies their base patches, ignores
selection declarations, and shares Catalog's vocabulary and validation rules.
It owns no second loader. Consumers needing profiles use Catalog; Grove chooses
the captured local declaration, else the primary declaration, else an empty list.
The conformance kit takes that same Catalog and explicit selection.

The public call signatures are below; the
[runner interface](module-decomposition.md#7--the-runner) owns the shared record
definitions (`Selection`, `SourceSpan`, `Occurrence`, `Origin`, `AssignmentHistory`,
`CompiledWord`, `Inspection` and `Diagnostic`). Both contracts use those types;
no consumer-specific resolver types or arbitrary patch input are introduced.

```rust
impl Catalog {
    pub fn load(primary: &Path, overlay: Option<&Path>, vocabulary: Vocabulary<'_>)
        -> Result<Self, ConfigError>;
    pub fn primary_selection(&self) -> Option<&Selection>;
    pub fn overlay_selection(&self) -> Option<&Selection>;
    pub fn resolve(&self, selection: &Selection) -> Result<Templates, ConfigError>;
}
impl Templates {
    pub fn load(primary: &Path, overlay: Option<&Path>, vocabulary: Vocabulary<'_>)
        -> Result<Self, ConfigError>;
    pub fn inspect(&self) -> &Inspection;
    pub fn source(&self, key: &str) -> Option<&Path>;
    pub fn keys(&self) -> Vec<&str>;
    pub fn require(&self, key: &str) -> Result<(), ConfigError>;
    pub fn expand(&self, key: &str, values: &[Slot<'_>]) -> Result<Argv, ConfigError>;
}
impl ConfigError { pub fn diagnostics(&self) -> &[Diagnostic]; }
pub mod conformance {
    pub fn check(catalog: &Catalog, selection: &Selection) -> Outcome;
}
```

An absent selection declaration and a present empty list are distinct.
`Selection` contains profile names and an optional declaration span. Catalog
owns captured sources and vocabulary; the returned snapshot owns its compiled
commands and inspection data. No file or Catalog borrow is needed to keep a
snapshot usable. Conformance resolves the supplied Catalog with the supplied
selection and reports semantic failures or an empty admitted-key set as failures;
successful loading is the caller's prerequisite, so structural errors are never
swallowed by the conformance entry point.

Grove's SessionConfig adapter retains personal path lookup, local discovery and
admission, list selection, kind-specific errors, and runtime context values.
TemplateSource stays a reloadable source rather than a cached snapshot. The
driver and mutating verbs preserve their existing load points; the running child
retains the argv it was launched with. The inspector calls this same adapter.
The generic launch, channel, process-group and VCS seams need no new behavior.

This is the smaller of two considered interfaces. Putting profile composition
in Grove would require Grove to understand generic parameter/template rules and
create runner input that duplicated their validation. A caller-supplied arbitrary
list of patch operations would expose most of the resolver instead of hiding
it. Catalog plus Templates keeps source policy outside and the configuration
language behind one existing test seam.

## Test seams and acceptance

The approved seams are configuration load/expansion with temporary sources, and
Grove launch acceptance with temporary jj workspaces and fake executables.
Exercise observable results, provenance and errors through Catalog/Templates and
SessionConfig, not parser helpers. The runner's generic conformance checks use
an arbitrary key and slot vocabulary unrelated to Grove.

| Acceptance case | Required observation |
|---|---|
| Shared command | Change one default/shared parameter; all users change except an explicitly overridden route |
| Lead arrangements | Change two bindings; the same route list reaches the opposite commands, with workspace isolation |
| Small experiment | Add/remove a parameter profile; embedded argument content changes/restores without changing word count |
| Ordering | Later assignment wins at its scope; includes precede own values; repeats and diamonds reapply; cycles name the full chain |
| Selection | Absent local selection inherits; explicit local selection replaces; empty selection disables profiles; global definitions remain available |
| Source discovery | Worktree candidate wins; otherwise repository candidate; tracked, unreadable and unprobeable candidates refuse without fallback |
| Inactive work | Unknown references/missing values in unselected profiles coexist with success; selection makes surviving errors visible |
| Partial building blocks | Routes, bindings and parameter values may arrive from different selected profiles and local parameter patches |
| Missing personal target | An active personal parameter patch with no personal target fails all resolution with `missing_target`, even for another requested kind; a local target cannot repair it; no inspection snapshot is returned |
| Loader convenience | `Templates::load` equals Catalog resolution with an empty selection, including wrapper base patches and identical vocabulary errors |
| Local authorization | A local-only key or one present only in an inactive personal profile cannot resolve; selecting its personal profile admits it |
| Legacy compatibility | Existing flat fixtures retain argv and eager diagnostics; new grammar words remain valid flat kind names |
| Parameter safety | Spaces, quotes, dollar/slot text, empty values and shell punctuation preserve boundaries; NUL fails before spawn |
| Provenance | Shared and per-kind winners, removals, repeated profile occurrences and overwritten values point to their real source spans |
| Inspection | Structured words expanded with a known runtime context equal captured launch argv; errors launch nothing and edit no working-tree/configuration bytes; jj metadata snapshots are permitted |
| Reload | An edited selection/value affects the next session and not the already running process |
| Delivery | Reader validates every packaged example; preexisting files and active policy survive success, conflict and interrupted installation |

No real agent session is needed. A design-time trace table and these executable
seams are sufficient for the deterministic resolver; no formal-model artifact
is claimed. Preserve the meta-grove signal guard in acceptance tests.

## Non-goals

There are no harness-specific settings, implicit kind families, catch-all routes,
environment substitution, shell execution, recursive parameter interpolation,
cross-file includes, automatic migration, configuration editors or live-child
restarts. Parameters are scalar strings, not optional argv groups or argument
lists; use a separate command definition or an explicit wrapper for a different
argument shape. Profiles cannot redefine command schemas or remove kind routes.
Removing a profile from the selection, or an authored route from personal policy,
is how such a route ceases to participate.

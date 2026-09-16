# modular-configuration-k3

**Reviews:** modular-configuration-k2


## Goal

Adversarially review the modular configuration design against the approved root
brief. Find contracts that cannot satisfy those requirements, ambiguous grammar
or precedence, unnecessary public surface, and defects that would make separate
implementations disagree. Produce findings rather than fixes.



## Context

Read the producer's commit by its stable handle, the requirements decisions in
`plan-k1`, and the current durable artifacts:

- `docs/specs/modular-configuration.md` and the configuration parts of
  `docs/specs/module-decomposition.md`.
- `docs/adr/complete-session-configuration.md` and
  `docs/adr/untracked-configuration-delta.md`.
- The configuration glossary entries and `CONTEXT-MAP.md` ownership.
- `docs/examples/modular-configuration/` and the editable visual document at
  `docs/design/modular-configuration/`.

The user explicitly approved reusing `Workspace::is_tracked` for inspection:
configuration/working-tree bytes stay unchanged, but jj metadata may snapshot.
This choice is not an unresolved requirement.

## Done when

- Findings cite the producer's artifact and the requirement or concrete scenario
  it violates, or the review explicitly finds no actionable defects.
- The read covers repeated/diamond includes, partial profiles, parameter
  specificity versus layer order, removal when changing command schemas, legacy
  compatibility, and the distinction between eager structure and active semantics.
- Try to admit a local-only kind using an inactive personal profile, repair a
  missing personal target from local policy, hide an invalid active command with
  inspection filtering, and inject argument/slot structure through a parameter.
- Assess whether the Catalog/Templates split and the flat-only compatibility
  helper earn their surface, whether source attribution explains mixed-origin
  arguments, and whether examples express the promised scenarios.
- Any integration leaf is cut lazily under the review procedure, before the
  already queued planning leaf, carrying this review's handle rather than a
  finding list as its charter.

## Notes

The producer changed design/documentation only. The installed reader still uses
the flat grammar, and no new-form examples were installed into personal policy.
The reference and example readme mark that implementation boundary explicitly.
Browser discovery returned no available browser in the design session, so the
served document's visual layout could not be inspected there. Check the views
in a connected browser if available; the viewer README gives the serving command.

## Findings

Read against producer commit `tnnkpslkslku` (`7903594032f1`), whose files are
unchanged in the working tree at review time, so every anchor below is current.
Ordered most severe first. Each names the artifact, the requirement or concrete
scenario it fails, and the direction a fix could take; the integration leaf
decides.

### 1. A selected parameter-only route with no personal target has no defined outcome

`docs/specs/modular-configuration.md:77` says a `route` with no binding is "a
parameter patch, completed by another active personal route for that key";
line 190 says such a route "without a personal target is incomplete"; lines
223-225 say "personal route patches must have personal targets". None of the
three says what happens when the completion never arrives: a blocking
validation error, or a non-admitted key that fails on use. The spec states the
non-blocking outcome only for *local-only* routes (line 197, "does not
independently block another configured kind") and the blocking outcome only for
routes "still literal at the end" (line 184).

Scenario: personal `config { profile "quiet" { route "impl" { param "effort"
"low"; } } select "quiet" }` with no other route for `impl`, and `design` routed
elsewhere. Is `design` launchable? Two implementations of this spec would
answer differently, and the root brief's acceptance scenario 6 (an incomplete
experiment must fail usefully when selected) does not settle which answer is
useful. Also unstated: which diagnostic category (`unconfigured key`, `unknown
reference`, or a new one) and whether inspection lists the key among
`non_admitted_keys`. State one outcome and put it in the acceptance table.

### 2. The module contract has no signatures for the seam it now assigns

`docs/specs/module-decomposition.md:338-347` says the runner "adds a parsed
`Catalog` before the resolved `Templates` snapshot" and points to the modular
spec for the interface, but the Rust block that follows (lines 349-471) still
shows only the flat surface: `Templates::load` (line 362) with a new comment,
`conformance::check(config, vocabulary)` (line 467) with a comment claiming
modular conformance "accepts the same explicit profile selection" through a
seam the block never shows. `docs/specs/modular-configuration.md:385-407` gives
Catalog, the selection declaration, the origin-bearing inspection view, and
provenance only in prose. No type for a selection declaration, an origin, an
assignment history, a compiled word, or a diagnostic record is named anywhere.

This is the load-bearing grammar-and-resolver contract the design leaf said
needed adversarial review, and it is the one seam the root brief's agreed test
seam ("exercise the public configuration interface ... assert ... origin
information") is written against. Two implementers would produce incompatible
`Catalog` APIs, and the planning leaf cannot name a test seam more precisely
than "Catalog/Templates load, resolution, provenance and argv". Every other
seam in decision 7 carries a signature block; this one needs the same grain:
`Catalog::load`, the selection type, `Catalog::resolve`, the inspection view's
shape, and the conformance entry that takes a selection.

### 3. The flat-only compatibility helper keeps a second rule set with no named consumer

`docs/specs/modular-configuration.md:403-406` keeps `Templates::load` "for
flat-only consumers", refusing a `config { }` wrapper with a message to use
Catalog; lines 252-254 give it *different vocabulary rules* (no `param.`
reservation) from Catalog. That is one crate with two loaders, two vocabulary
rules, and a third behaviour class (a flat consumer opening a modular file gets
an error rather than the base result). Every present caller of `Templates::load`
in this repository is a test fixture (`crates/grove-loop/src/loop_driver.rs:620`,
`driver_lease.rs:1132`, `observation.rs:909`, and siblings) or the conformance
kit (`crates/keyed-launch/src/conformance.rs:65`), which the spec says moves to
the selection-taking form anyway. No external consumer is named.

The root brief's design obligation is "the smallest useful module interfaces".
A helper that is exactly `Catalog::load(primary, overlay, vocabulary)` plus the
empty selection, with identical rules and accepting a wrapper as base-only,
costs nothing and removes the divergence. Keeping the refusal and the separate
vocabulary rule needs a consumer that would break; if there is none, the
surface is unearned.

### 4. Operator verbs are placed in the LLM binary against the recorded audience split

`docs/specs/modular-configuration.md:299-310` puts `config-show` and
`config-examples` in `grove-llm`, then has to add two carve-outs the placement
creates: "the CLI's overview must make these configuration verbs discoverable
to humans" and "they dispatch before ambient session-epoch admission" (a change
to `crates/grove-llm/src/cli.rs:420-421`, where `run` admits every verb before
dispatch). The binary's own help says "none of these verbs are meant for direct
human use" (`crates/grove-llm/src/cli.rs:46-49`), and
`docs/ARCHITECTURE.md:133-142` records the split as keeping "a discoverable
human API" in `grove`. `grove view` (`docs/ARCHITECTURE.md:167-173`,
`crates/grove/src/cli.rs:59-64`) already shows the shape an operator read-only
surface takes there: it dispatches before workspace resolution, lease
acquisition and launch configuration, with no admission carve-out.

The requirement is "a read-only way to inspect ... without launching a
session"; it names no binary. No rationale is recorded for choosing the LLM
surface over `grove config show` / `grove config examples`, and the choice
contradicts the split without reworking its record. If a session also needs
inspection mid-task, both binaries can call one `grove-loop` adapter; the seam
is the adapter, not the verb.

### 5. Two contract-defining composition rules are outside the ADR set

`docs/specs/modular-configuration.md:150-156` chooses once-per-occurrence
application over a visited set, with a worked example of what changes; lines
169-172 choose per-setting precedence in which a route's explicit override
beats a *later* shared value ("those are different settings"). Both are hard to
reverse once configurations depend on them, both are surprising without the
context (an implementer would "optimise" the first with a visited set and
"fix" the second to honour layer order, which is how the root brief phrases
composition), and both have a rejected alternative. That is the ADR test in
full, and `docs/adr/complete-session-configuration.md` records neither; it says
only that composition is "explicit" and ordered. The reverse grain problem also
appears: `docs/adr/untracked-configuration-delta.md:8-10` restates the
composition order the spec owns.

### 6. The packaged example bytes cannot be installed unchanged

`docs/specs/modular-configuration.md:375` requires "the repository example
bytes, not a second hand-kept copy", and line 353 says the readme is installed
as instructions. `docs/examples/modular-configuration/README.md:3-4` links
`../../specs/modular-configuration.md`, which from `~/.config/grove/` resolves
to nothing; lines 3-7 are a delivery-status paragraph the readme itself says to
remove; every `.kdl` sample opens with a "becomes usable when the modular
reader is delivered" comment (`config.modular.example.kdl:1` and the five
local samples). Either the byte-identity rule or the content has to give, and
the spec does not say which, so the planning leaf cannot tell whether the
installer's test compares against edited or unedited repository files.

### 7. The `unset` demonstration produces no observable change

`docs/examples/modular-configuration/grove.local-override.example.kdl:6` unsets
`proof`'s `effort` override "to expose the shared value", but that file selects
`daily` and `high-effort`, under which the shared value for the `review`
command is already `high`, the same as the override it removes. The launched
argv is identical with and without the `unset`; only provenance differs. The
readme (`README.md:58`) promises the removal is demonstrated. A value that
differs from the shared one, or a selection without `high-effort`, would show
it.

### 8. The glossary lacks the spec's own core terms

`docs/specs/modular-configuration.md:23-25` introduces **command definition**,
**command binding** and **kind route** as the three named things the grammar
composes, and "parameter" as the unit of variation. `CONTEXT.md` gained only
`Command binding` (line 803). `CONTEXT-FORMAT.md` requires a term to be written
in as it is resolved, and the root brief's `Done when` requires the glossary
and spec set to describe one coherent contract; the next sessions will
otherwise re-resolve "route" against the older *Kind routing* entry (line
824), which describes the driver's launch path rather than the grammar object.

## Coverage

Read in full: the spec, both ADRs and their diffs, the module spec's decisions
6-7 and requirement scenarios, the glossary and context-map diffs, the example
set and readme, the three diagram sources and manifest, the current runner
(`templates.rs`, `argv.rs`, `vocabulary.rs`, `conformance.rs`), the adapter
(`session_config.rs`), the LLM CLI, the driver's two load points, the existing
adapter tests, and the version control seam's trackedness probe (which does
snapshot, so the approved inspection premise holds).

Held to the spec and found sound: repeated and diamond includes (once per
occurrence, stack-based cycle detection, stated with an example); partial and
inactive profiles (structural everywhere, semantic on the selected
combination); parameter specificity versus layer order (explicit, see finding
5); removal when a route changes command schema (`unset`, and only surviving
assignments are checked); legacy compatibility (matches the current
overlay-only non-admission, eager template checks, search order and
trackedness refusal in `session_config.rs` and its tests); eager structure
versus active semantics (three-scope table). Attempted and refused by the
spec: admitting a local-only kind through an inactive personal profile
(lines 188-189); repairing a missing personal target from local policy (lines
190-191); hiding an invalid active command with `--kind` filtering (lines
313-316); injecting argument or slot structure through a parameter value
(lines 242, 260-266). Source attribution covers mixed-origin arguments (lines
290-295, 336-340). The examples express every promised scenario except the
`unset` case in finding 7.

Visual document: served from the design directory and opened in a browser
under this session. All three Mermaid diagrams rendered from jsDelivr with
readable node text matching their sources, the discussion panel and outline
rendered from the manifest, the `#discussion` deep link landed, and the only
console error was a missing favicon. Captions match the spec and add no rule.
A pixel screenshot was not retained; the rendered accessibility tree is the
evidence. Note for later sessions: that browser ran locally, which the user
has since ruled out; drive any repeat through the testanyware VM skill.

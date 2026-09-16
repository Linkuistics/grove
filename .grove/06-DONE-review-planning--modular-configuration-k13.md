# modular-configuration-k13

**Reviews:** modular-configuration-k4


## Goal

Adversarially review the modular configuration implementation tree against the
approved root requirements and integrated design. Produce findings about the
plan, without implementing the feature or rewriting the producer's work.



## Context

Find the producer commit by `modular-configuration-k4`; inspect its diff against
the current tree. The planned subject is `configuration-engine-k6` (children
`captured-configuration-k7`, `reusable-commands-k8`, `profile-composition-k9`),
then `workspace-configuration-k10`, `configuration-inspection-k11` and
`configuration-examples-k12`. The root's acceptance ownership map is part of the
artifact, not evidence that the map is correct.

Use `docs/specs/modular-configuration.md`, its referenced runner interface and
configuration ADRs. The design review was integrated by
`modular-configuration-k5`; re-derive planning concerns from the current contract
rather than reopening the original requirements interview.

## Done when

- Check whether each child fits one focused session and delivers usable,
  independently verifiable behavior with green callers and source-exact books.
  In particular, challenge the captured-flat → reusable-base → profiles sequence:
  can each public API/grammar boundary work honestly, without stubs, silent
  acceptance of unfinished syntax, a second loader or an expensive throwaway
  implementation? Identify any smaller useful slices the plan missed.
- Independently trace every spec acceptance row and additional module obligation
  to a concrete owner/test seam. Look especially for lost validation scopes,
  authority before overlay, profile occurrence semantics, diagnostic aggregation,
  native strings, multi-origin histories and inactive versus selected failures.
- Challenge the public-signature migration, generic non-Grove consumer, captured
  source lifetime, empty-selection loader equivalence and conformance path.
  Check that adapter/reload tests cover both mutation and launch boundaries.
- Inspect CLI dispatch and observable error obligations, including parser-level
  JSON usage failures, held leases, stale epochs and read-only working-tree
  effects. Confirm that equality with launch is conditional on the same captured
  inputs/runtime context, not an invented cross-load guarantee.
- Check that documentation and recursively included book sources are owned by
  the changing leaf; a later reconciliation must not excuse an intermediate red
  book. Check actual personal example delivery, repository-byte packaging,
  collision/race/partial-failure behavior and active policy preservation.
- Findings name the concrete contradiction or missing work and its effect;
  distinguish real gaps from preference and avoid treating the ownership table
  as proof. A clean review may retire without an integration leaf.

## Notes

If actionable findings exist, cut `integrate-review-planning` where it runs
next, before the first live implementation sibling entry. Its body should cite
this review handle, not transcribe findings as mandatory work. No review of a
review and no in-session reviewer; this leaf is the fresh adversarial context.

## Decisions (running log)

- Read against producer commit `owvnqnnrvvuo` (`b54b17f0d884`); its files are
  unchanged in the working tree at review time, so every anchor below is
  current. No in-session reviewer was spent; this leaf is the adversarial read.
- Findings 1 and 2 are actionable, so an `integrate-review-planning` step is
  cut with `leaf-insert` at `configuration-engine-k6`, the first sibling entry
  after this review whose subtree holds live work. Its body carries this
  review's handle and not the finding list.

## Findings

Ordered most severe first. Each names the plan artifact, the contract it
contradicts or the work it leaves unowned, and the effect; the integration leaf
decides the fix. Preference is separated out under Coverage.

### 1. The plan describes Grove's adapter as "still flat" through the engine node, but the unchanged adapter call changes Grove's accepted grammar at every child

`.grove/08-k6/_configuration-engine.md:47-50` requires the specs and user
reference to "distinguish the delivered generic subset from Grove's still-flat
adapter", and forbids accepting and silently ignoring unfinished syntax.
`.grove/08-k6/02-impl--reusable-commands-k8.md:63` and
`.grove/09-impl--workspace-configuration-k10.md:61` follow from that model: the
Grove reference stays quiet until `workspace-configuration-k10` documents "the
delivered configuration grammar".

The model is false. Grove's adapter reads through `Templates::load`
(`crates/grove-loop/src/session_config.rs:178`), and the reviewed contract
defines that entry as exactly `Catalog::load` plus an empty-selection resolve
that "accepts wrappers and applies their base patches, ignores selection
declarations" (`docs/specs/modular-configuration.md:432-434`). The plan itself
requires that behaviour of the convenience in each child
(`.grove/08-k6/03-impl--profile-composition-k9.md:48`; ownership row *Loader
convenience*, `.grove/_BRIEF.md:205`). Two consequences, neither owned:

- **After `reusable-commands-k8` lands, Grove launches from wrapper base
  patches.** A personal `config { command …; bind …; route "impl" "lead" }` and
  a local `config { route "impl" { param … } }` are accepted and used by the
  installed driver and by every `grove-llm` mutating verb, with no adapter
  change. That is delivered Grove behaviour, and the root brief makes each
  producer own documentation for its delivered behaviour; k8's charter instead
  tells it only what not to advertise.
- **Between `profile-composition-k9` and `workspace-configuration-k10`, Grove
  silently ignores `select`.** A personal file with profiles and a
  `select "daily"` loads, the declaration is discarded by the convenience, and
  the session launches on the base result without the selected profiles. When
  the base carries flat routes (the compatibility case the examples show), the
  launch succeeds on the wrong command and nothing is reported. That is exactly
  the outcome `_configuration-engine.md:47` forbids, produced by the plan's own
  sequence rather than by unfinished syntax.

Directions the integration could take: an adapter-side guard in
`profile-composition-k9` that refuses a captured selection declaration or
profile node with an explicit pending diagnostic until `k10` removes it; or
moving the three-way selection choice (overlay declaration, else primary, else
empty) into `k9`'s scope so no interim ignores `select`; and in either case
making `k8` and `k9` own the Grove user reference for whatever forms the
unchanged adapter now accepts, which also shrinks `k10` (finding 3).

### 2. Two `grove` subcommands are added with no owner for the user-guide coverage inventory

`configuration-inspection-k11` and `configuration-examples-k12` add `config
show` and `config examples` to `grove --help`
(`.grove/10-impl--configuration-inspection-k11.md:61`,
`.grove/11-impl--configuration-examples-k12.md:27`) and name `docs/USAGE.md`,
architecture and crate docs. Neither names
`docs/specs/user-guide-coverage.md`, whose command inventory is a standard
"derived from `--help`" (`docs/specs/user-guide-coverage.md:58-74`) and whose
`grove` table stops at `view`. The repository test
`crates/grove/tests/user_guide_coverage.rs` checks that the guide answers every
inventory row; it cannot see a row that was never written, so the omission is
green while it makes the guide's completeness claim false for the two commands
a human will actually type. The clap surface test at
`crates/grove/src/cli.rs:142` (`subcommands == ["view"]`) is the only mechanical
signal and it names none of this. Effect: `USAGE.md` gains prose but the
standard it is measured against is silently narrower than the binary. The
inventory rows and their guide anchors belong to the leaf that adds each
command.

### 3. Two leaves are unlikely to fit one focused session, and each has a natural seam

This is a judgement finding, stated because the plan's own bar is *fits this
session*, not *can be finished*, and because both seams are visible now.

- **`captured-configuration-k7`** rewrites the 670-line
  `crates/keyed-launch/src/templates.rs`, introduces the entire record set the
  module contract names (`Selection`, `SourceSpan`, `Occurrence`, `Origin`,
  `Setting`, `AssignmentValue`, `AssignmentHistory`, `CompiledWord`, the four
  view types, `NonAdmittedKey`, `Inspection`, `Diagnostic`), converts
  `ConfigError` from a message to structured diagnostics (23 message-asserting
  tests in `crates/keyed-launch/tests/templates.rs`), aggregates structural
  errors across both documents, moves spans to byte offsets, changes the
  conformance signature and its six callers, adds the non-Grove snapshot test,
  and rewrites the reconstructing chapters of a 9,257-line book for every
  module it splits out. A seam that keeps both halves vertical: first
  structured diagnostics, source capture, `Catalog`/`Selection`, the conformance
  migration and the `Templates::load` delegation; then the inspection and
  provenance view for flat templates with its equality and snapshot tests.
- **`workspace-configuration-k10`** combines the adapter migration and its
  structured error mapping (which reaches `crates/grove-llm/src/cli.rs:841` and
  so the `grove-llm` book), six named jj-workspace acceptance scenarios plus
  reload, the complete Grove grammar reference replacing a flat
  `docs/CONFIGURATION.md`, usage and architecture prose, ADR and glossary
  reconciliation, and the `grove-loop` book. Finding 1's remedy moves the
  grammar reference earlier and leaves `k10` the selection policy, reload and
  admission it actually delivers.

### 4. New engine modules with inline test files oblige a spec inventory no leaf points at

`.grove/11-impl--configuration-examples-k12.md:73` and the engine leaves say to
discover new modules through manifests. A new `tests.rs` inline test module in
`crates/keyed-launch/src/` needs a `[[corpus.exclude]]` **and** a matching row
in the corpus exception inventory at `docs/specs/walkthrough-books.md:401`;
the manifest alone is refused by
`crates/grove/tests/corpus_exception_inventory.rs`. This one is gated, so the
effect is a red principal check rather than silent drift; it is recorded so the
engine sessions do not read the manifest as the whole obligation.

## Coverage

Read in full: the producer's commit and every planned node and leaf body, the
root brief as it now stands, the modular spec, module contract decisions 6-7
with their scenarios and test seams, both configuration ADRs, the glossary
entries, the example set and readme, the design review and integration leaves,
and the current runner (`templates.rs`, `argv.rs`, `error.rs`,
`conformance.rs`), adapter (`session_config.rs`), driver load points
(`loop_driver.rs:242-281`), verb load point (`grove-llm/src/cli.rs:841`), human
CLI, the check script, all six book manifests, and the two repository
inventory tests. Graph coverage was checked for the six production files and
reported no recorded gaps; conclusions rest on direct reads.

**Acceptance rows.** All seventeen spec rows trace to a concrete owner and
seam: the eager legacy scope to `k7`, document-wide wrapper checks and
personal/local restrictions to `k8`, the selected-combination scope,
occurrence semantics, cycle chains and authority-before-overlay to `k9`,
source discovery, absent/replacement/empty selection and reload to `k10`,
inspection equality, read-only bytes, lease/epoch independence and parser-level
JSON usage errors to `k11`, and byte-exact packaging, preflight, exclusive
create, partial failure and actual personal delivery to `k12`. Diagnostic
aggregation, native strings and multi-origin histories have owners in `k7`,
`k8` and `k11`; inactive-versus-selected failure has owners in `k9` and `k12`.
The `missing_target` row is carried by all four leaves the brief names.

**Sequence honesty.** The captured-flat, reusable-base, profiles order needs no
stub or second loader: flat routes stay literal targets in the final language,
the wrapper is already refused by the flat reader, and `k8` keeps `profile` and
`select` as explicit shape errors. Nothing built in `k7` or `k8` is discarded
by `k9`. The interim hazard is at the Grove boundary, not the generic one
(finding 1).

**Signature migration and consumers.** Retaining `Templates::load` leaves the
inline tests in `crates/grove-loop/src/driver_lease.rs` and `loop_driver.rs`
untouched, which keeps the `grove-loop` book out of `k7`. The conformance
change reaches `crates/grove-loop/tests/session_config.rs:295,310`, a test file
outside every corpus, and `k7` already says to discover callers afresh.

**Reload and inspection seams.** The composed-loop tests in
`crates/grove/tests/loop_driver.rs` already write a temporary `$HOME`
configuration, plant fake executables and drive relaunch through the signal
file, so `k10`'s reload scenario and `k11`'s inspection-versus-launch equality
have an existing seam. Equality is stated as conditional on unchanged inputs
and context in both `k11` and the spec; no cross-load guarantee is invented.

**Delivery.** The destination directory currently holds `config.kdl` and one
unrelated file, none of the seven destination names, so `k12`'s actual
delivery has no known collision. The packaged set is six KDL files and the
readme, matching `k12`'s count; the readme's stated outcomes agree with the
sample bytes after the design integration's fixes.

**Held to be preference, not findings.** `k10` exposing inspection data for a
consumer that lands in `k11` is the spec's own `inspect()` surface, not
speculative API. The visual design document has no owner for prose-status
edits, but its captions add no rule and none of the leaves changes what they
depict. The `k11` requirement that inspection resolve a jj workspace outside
any task tree matches the spec and is not a gap.

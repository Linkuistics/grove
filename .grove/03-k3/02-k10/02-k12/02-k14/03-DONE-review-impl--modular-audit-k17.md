# modular-audit-k17

**Reviews:** modular-audit-k16

## Goal
Adversarially review the completed modular-only configuration subsystem and its
forms/prose audit against the root and ancestor acceptance criteria.

## Context
The producer commit is the review boundary, but its diff is predominantly
prose. Inspect the cumulative removal delivered by modular-input-k11,
modular-parser-k13 and modular-types-k15, plus earlier fixture/example migration.
Find those commits by their stable handles; do not restrict review to k16's diff.
`docs/configuration-forms-audit.md` records the enumeration, classifications and
controls. The durable grammar is `docs/specs/modular-configuration.md`; authority
is also in the complete-session-configuration and untracked-configuration-delta
ADRs. The public type contract is in `docs/specs/module-decomposition.md`.

## Done when
- Review both Catalog and Templates loading: reject flat and mixed documents in
  either source, including grammar-word keys; retain useful source/span/remedy
  diagnostics and unchanged input. Check empty documents and base-only modular
  behavior, plus validation of dormant structure versus effective semantics.
- Try to break active personal authority, local selection precedence, repeated
  profile/include order, parameter specificity and unset inheritance. Verify that
  invalid configuration cannot bypass existing tree-mutation/launch load points.
- Check expansion and inspection agree after capture and source removal: argument
  boundaries, native runtime values, required values and NUL rejection; command,
  binding, route, parameter and selection origins and overwritten histories.
  Look for surviving compatibility machinery without a modular consumer.
- Challenge the forms audit's scope, controls and classifications. Read current
  examples and the book summaries as well as exact fragments; validation of bytes
  alone does not establish correct prose. In particular, source() names the
  personal definition and inspect() carries local contributions. Check packaged
  examples and preservation of previously installed files.
- Record actionable findings against current source and the contract. Preserve
  the agreed public load/expansion test seam; do not demand a new CLI/fake-agent
  acceptance suite merely because the removal is broad. Existing checks passed.
- Grow an integrate-review-impl sibling only for findings worth acting on.
  Otherwise retire and check the parent chain against its Done when conditions.

## Notes
This is the scheduled subsystem review under Grove's load-bearing-artifact
threshold. No competing in-session reviewer ran. Follow the review skill's
inspection-only contract; any needed fixes belong to integration.

## Decisions (running log)

- Reviewed the cumulative removal at commit `0ba4869c6233` (modular-audit-k16),
  reading the diffs of modular-examples-k9, modular-input-k11, modular-parser-k13
  and modular-types-k15 as one artifact. Line numbers below are against that
  commit's working tree.
- The runtime contract holds: both loaders reject flat, mixed and grammar-word
  input in either source with exact spans, roles, the modular remedy and
  unchanged bytes (`crates/keyed-launch/tests/catalog.rs:368-415`,
  `crates/grove-loop/tests/session_config.rs:539-559`); empty documents and
  base-only wrappers resolve; personal target authority is checked after
  selected profiles and before the overlay (`named.rs:864-868`, `1020-1022`);
  local selection replaces the personal default including the empty list
  (`session_config.rs:186-193`); includes apply postorder once per occurrence
  (`named.rs:722-802`); route values beat shared values beat defaults with
  `unset` exposing the lower scope (`named.rs:1191-1198`); NUL is refused in
  active templates, resolved parameters and every offered runtime value;
  inspection and expansion share `CompiledWord`; the text and JSON renderers
  agree with the required `CommandView` binding/command fields; every tree
  mutation and launch load point still passes through `SessionConfig`
  (`crates/grove-llm/src/cli.rs:840-851`, `crates/grove-loop/src/loop_driver.rs:242-281`).
  No new CLI suite is asked for. Findings are prose and contract, plus one dead
  compatibility branch, so an `integrate-review-impl` sibling is cut.

## Findings

Each is anchored to current source and the contract it contradicts. None asks
for new acceptance machinery; the public load/expansion seam already covers the
runtime claims.

1. **Spec asserts a compatibility rule the code deleted.**
   `docs/specs/modular-configuration.md:225-227` says "Legacy eager template
   checking remains a deliberate compatibility rule rather than silently
   weakening old files." No such rule exists: `Catalog::load`
   (`crates/keyed-launch/src/templates.rs:156-190`) captures structure only and
   `named::resolve` compiles a definition only when an effective binding
   reaches it (`named.rs:1074-1105`). `crates/keyed-launch/src/lib.rs:32-33`,
   `docs/CONFIGURATION.md:474-475` and the test
   `active_bindings_validate_even_without_routes_but_unused_definitions_do_not`
   (`crates/keyed-launch/tests/named_commands.rs:133-146`) all state the
   opposite. The durable grammar contradicts itself.

2. **Spec still legislates for flat entries.**
   `docs/specs/modular-configuration.md:139-140`: "Flat entries and
   wrapper-level routes share a document's base route namespace: declaring the
   same key in both is an error". There are no flat entries to share anything
   with (`named.rs:110-121` rejects every non-wrapper node before wrapper
   traversal). Same section, `:205-206`, keeps "For compatibility … as today's
   overlay-only keys are", a hedge with no referent now.

3. **Overview book names the deleted inspection variants as current.**
   `docs/walkthroughs/overview/05-what-the-call-reaches.md:286-287`: "Each
   assignment retains fold order and distinguishes set, literal template, unset
   and reset." `AssignmentValue` is `Set`/`Unset` only
   (`crates/keyed-launch/src/inspection.rs:22-27`) and the fragment reproduced
   directly beneath that sentence (`crates/grove/src/config.rs:152-155`)
   matches on exactly two arms. This chapter was in modular-audit-k16's diff, so
   the byte validator passed while the explanation stayed wrong.

4. **grove-loop book describes eager legacy validation.**
   `docs/walkthroughs/grove-loop/18-which-files.md:633-634`: "both documents
   receive structural and eager legacy validation; effective named targets and
   values are checked during resolution." Contradicts the paragraph immediately
   above it (`:624-630`), `docs/design/modular-configuration/validation.mmd`
   (the eager node was removed) and finding 1's source. Also `:534` cites
   `loop_driver.rs` line 215 for `personal_path()`; it is line 216.

5. **"The delta that overrode it" survives in code comment, book and reference.**
   `crates/grove-loop/src/loop_driver.rs:263-266` argues that `resolved_source`
   may be "the delta that overrode it" and that naming the personal file "for a
   delta-supplied kind" misleads. `Templates::source` now always returns the
   personal file holding the command definition (`templates.rs:249-257`;
   `named.rs:1300` takes the definition span's path; definitions are
   primary-only, `named.rs:240-246`), and
   `each_kind_reports_its_command_source_and_route_origin`
   (`crates/grove-loop/tests/session_config.rs:445-470`) asserts
   `source()` is the personal path for the delta-routed kind. The book repeats
   the argument as its explanation
   (`docs/walkthroughs/grove-loop/20-the-loop.md:986-991`, and `:1235` for the
   spawn-failure doc comment), and the operator reference makes the same
   promise at `docs/CONFIGURATION.md:632-634` ("the personal file, or the delta
   that overrode it"). Contributions from the delta are visible only through
   `inspect()`, which is what `session_config.rs:223-224` already says.

6. **`require_declared` doc comment and its chapter describe whole-file eager
   checking.** `crates/grove-llm/src/cli.rs:826-839`: "exactly one complete
   template read whole out of one file" and "every template rule in both
   documents is still checked eagerly, and a malformed entry for a kind this
   call will never touch still fails here." The second claim is false for a
   dormant definition (finding 1); the true statement is structural validation
   of both documents plus semantic validation of the active composition. The
   grove-llm book restates both claims as prose
   (`docs/walkthroughs/grove-llm/04-growing-the-tree.md:333-335`).
   `docs/walkthroughs/keyed-launch/07-the-job.md:361` carries the same
   "template read whole out of a configuration file" phrasing.

7. **Reference shows a retired diagnostic rendering.**
   `docs/CONFIGURATION.md:588-597` presents a duplicate report as
   "duplicate key `impl`; declarations at …" with remedy "Keep one declaration
   per key in each document" and then explains "The report says *key*". The
   current renderer emits "duplicate declaration `impl`" with remedy "Keep one
   declaration per name in this namespace and document." and carries the second
   declaration as a related span, not inline text (`named.rs:521-532`,
   `templates.rs:621-662`). That format belonged to the flat duplicate
   aggregation modular-parser-k13 deleted.

8. **Forms audit: the prose sweep has no stated enumeration or control.**
   `docs/configuration-forms-audit.md:20-44` enumerates and controls the
   quoted-token search well, but `:33-35` covers old prose claims with one
   sentence and no pattern, control or classification table. Findings 1 to 7
   are all prose claims about retired behaviour in files the audit lists as
   reconciled (`:87-91`), and the survivor table (`:76-85`) names only
   `docs/CONFIGURATION.md` among current documents while the spec itself
   carries two. The audit's own controls establish sensitivity for `kind "…"`
   tokens only; the claim "current guides/specs … all book surfaces were
   included" (task file, Delivered evidence) is not evidenced for prose.

9. **Dead compatibility branch in `expand` (low).**
   `crates/keyed-launch/src/templates.rs:298-311` derives an `Overlay` role when
   `template.source` equals the overlay path. After modular-types-k15 the source
   is always the primary definition path (finding 5), so the branch is
   unreachable and `Templates.overlay` is retained partly for it. Related and
   older: `CapturedDocument._source`/`_document` (`templates.rs:62-69`) are
   kept under a comment promising "provenance without rereading a file", but no
   reader exists; spans are offsets consumers print without the text. This
   predates the removal and may be accepted visibly rather than fixed.

Verified and not findings: rejection matrix and remedies; empty and base-only
documents; profile order, diamonds and cycles; personal authority timing;
parameter specificity and `unset`; NUL and native runtime bytes; inspection
origins and histories, including the non-admitted key path; renderers; packaged
examples and installer preservation of the retired filename; release smoke
recipe; driver guidance; glossary entries; validation diagram.

# usage-agreement-k69

## Goal

Place harness-dispatch in the architecture document, bring the usage documents
into agreement with one another and with `--help`, and write the CHANGELOG for
the first release. Then cut the documentation-acceptance review, as the last
act before retiring.

## Context

`acceptance-walk-k67` found a passing instrument for every acceptance case,
and its running log names each one. `current-state-documents-k68` made the
spec current. The documents here must claim no more than those two
established.

What k67 saw while sizing this leaf, as a starting list and not a complete one.

- `docs/ARCHITECTURE.md` names harness-dispatch once, under *Task kinds and
  composition*, as the reader of the `**Reviews:**` and `**Creator:**` lines.
  It places the package nowhere. *Repository products* still says Homebrew
  installs `grove` and `grove-llm`. *Documentation ownership* has no row for
  the dispatch README or its spec. *Main module seams* defers to the overview
  book's package map, and no book under `docs/walkthroughs/` names the crate.
  *Verification* does not mention the worker build, the probe builds or the
  installed smoke. Place the package and its boundary: what it depends on,
  what it must not, and why an extraction can move it. The boundary's
  decision is `docs/adr/harness-selection-is-owned-by-policy.md`.
- `CONTEXT-MAP.md` argues, crate by crate, why each is or is not a bounded
  context. It says nothing of harness-dispatch, whose terms the grove glossary
  already holds. Decide whether it owes a paragraph.
- The Grove-side guidance lives in four places, as the root brief records
  under *Grove dispatch closed*: `docs/CONFIGURATION.md#harness-dispatch`,
  `docs/USAGE.md#if-a-dispatched-launch-refuses`, configure-grove's
  `references/dispatch.md`, and the dispatch README's *Called from Grove*.
  The creator line's owner is `docs/CONFIGURATION.md#a-reviews-creator-line`.
  Consolidate against those and do not write a fifth.
- Both help texts are a subject: `harness-dispatch --help` with each
  subcommand's, and `grove --help` where it quotes the dispatch command. A
  document that describes a flag, a default, a bound or an exit code must
  agree with the help text and with the spec's tables. Enumerate the flags
  from the help output, then find each in the documents.
- `docs/RELEASING.md` lists the installed layout near "the compiled policy
  worker". Check it against the spec's `#delivery`, which also puts the Grove
  adapter's declarations in `grove/`.
- The CHANGELOG's Unreleased section was written increment by increment. Its
  first `harness-dispatch` entry still says "This first step delivers
  `harness-dispatch inspect --kind K` for a static `routes` policy". Rewrite
  the section so it describes the first release of harness-dispatch as a
  whole, the Grove slots and the methodology amendment. The file's own header
  says how entries are written, and that the `## Unreleased` heading must stay
  exactly as it is.

A new documented form of the dispatch command needs a launched case in
`the_documented_command_definition_for_dispatch_is_the_one_launched_here`
(`crates/grove/tests/loop_driver.rs`), which pins every quoted command.

## Done when

- `docs/ARCHITECTURE.md` places the package and its boundary.
- The dispatch README, `docs/USAGE.md`, `docs/CONFIGURATION.md` and
  configure-grove agree with one another and with `--help`.
- The CHANGELOG's Unreleased section describes the first release of
  `harness-dispatch`, the Grove slots and the methodology amendment.
- `task check` passes on the tree the review will read. If the worker's
  source, the installed layout or a native dependency changed since k67's
  smoke run, `task release:smoke` passes again on all three targets.
- Before retiring, cut the review at the grove root:
  `grove-llm leaf-add . dispatch-documentation --kind review-impl`, with
  `**Reviews:** dispatch-documentation-k41`. Retiring this leaf closes the
  node, so the review names the node. Its body asks for a review of the
  documentation against the requirements' acceptance cases: activation, both
  inspection surfaces, the `**Creator:**` conventions and their remedies,
  later outcome entry, launch-time validation guidance and the stated floors.
  Point it at `acceptance-walk-k67`'s running log, which names the test for
  each clause and lists what only a review can judge, under *What a test
  cannot hold*. The review's own brief chain is the root alone, so it will
  not find that log unless its body names it.
  The root brief gives it one more subject, where it records
  `creator-reference-k60`: k60 rewrote the last paragraph of the retirement
  step, nothing has reviewed the rewrite, and this review reads it for a leaf
  that cuts its own review, a node close and a multi-level close.

## Notes

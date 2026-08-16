# mandate-machinery-k21

**Reviews:** mandate-machinery-k10

## Goal

Adversarially read the mandate-machinery deletion, on the one axis a deletion of
this size fails on: **what survived that should not have, and what went that
should have stayed.**

Inspection only. Read the producer's commit (named by `mandate-machinery-k10`),
the current source, and the recorded evidence. Do not run the test, build, lint
or format commands, do not edit anything, and do not redo the deletion; findings
go to a paired `integrate-review-impl` step.

## Where it is cut, and why here rather than after

Inserted **before** `delivery-acceptance-k11`, not appended after it. That run is
the grove's `Done when` instrument and it judges the corpus a session actually
receives; a review that lands after it would be reviewing an artifact whose
verdict is already recorded.

## What to look for first

The producer's own two must-survive checks are the first thing to confirm, and
the confirmation is *that they still mean what they claim*, not that they are
green:

- **The instructed-verb scan** (`tests/methodology.rs`) — it reads the corpus as
  markdown file by file. Does it still gather the whole corpus? `INSTRUCTED_VERBS`
  is pinned complete at eleven; is `methodology`'s removal from that list matched
  by its removal from the CLI, and does the positive control still fail when it
  should?
- **The flat-verb-surface pin** — unchanged, but it is the premise the scan's
  grain rests on. Confirm it is still asserted rather than merely true.

## What went that might not have deserved to

Each of these was deleted or re-based rather than simply left. Judge whether the
replacement carries the claim or quietly drops it:

- **The build gate.** Its replacements are `tests/methodology.rs`'s routing-table
  and body-budget checks. That is a narrower claim at a weaker moment (`cargo
  test`, not `cargo build`). Is anything the gate caught now caught by nothing?
- **`every_embedded_markdown_file_is_classified`** became
  `the_linked_embed_carries_every_markdown_file_on_disk`. `build.rs` is now *only*
  the `rerun-if-changed` walk, so this is the sole check on it. Does it actually
  fail when that walk goes wrong?
- **The ending drift pin** (`tests/session_kind_guidance.rs`) moved from two unit
  ids to two files' bytes. Same subject?
- **The family-scope guard** was deleted outright rather than re-based. Was there
  a live claim inside it — about the kind taxonomy rather than about `kinds=`
  markers — that went with it?
- **`the_skill_carries_the_two_rules_the_core_sheds`** (`tests/prompt.rs`)
  re-based its locator from a unit marker to the condition's bold opener. The
  semantic predicate is unchanged; the locator is weaker. Is that the right trade,
  and does a missing opener fail loudly?

## The corpus edit

164 unit markers and 27 file directives were stripped by script, with
marker-created double blank lines collapsed. Six markers sat between two non-blank
lines; four were list-item boundaries (correct to join) and two were repaired by
hand in `content/references/requirements.md`. Read the `content/` diff for any
seventh case the classifier missed, and for prose that only read correctly with a
comment line in it.

**Two pre-existing corpus shapes, noticed and deliberately not changed here** —
they are `corpus-rewrite-k7`'s output and out of a deletion leaf's scope, but they
are worth a verdict: six `references/*.md` files open on a bare list item with no
heading, and `content/references/requirements.md` carries
`<!-- grove reference file — the field guide: habits for driving a session well -->`,
which is `driving.md`'s banner.

## Citation and record reconciliation

`docs/specs/mandate-delivered-methodology.md` is deleted. Confirm nothing points
at it, and that each site that did now says something *true* rather than merely
something else: `CONTEXT-MAP.md`, `docs/ARCHITECTURE.md`, `src/lib.rs`,
`tests/loop_driver.rs`, `tests/legacy_claim_sweep.rs`, and
`docs/specs/skill-delivered-methodology.md`'s own deletion clause.

The glossary rework is the other half: *Methodology unit* and *File directive*
removed, *Mandate slice* trimmed to a retired entry, *Triggering unit / procedural
unit* reworked into **Condition / procedure**. Check for a dangling `[[link]]`,
and judge whether the reworked entry states a live discipline or a fossil.

## Done when

Findings are recorded for integration, ranked, each naming a file and line and
saying what breaks.

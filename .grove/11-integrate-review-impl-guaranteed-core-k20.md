# guaranteed-core-k20

**Integrates:** guaranteed-core-k19

## Goal

Triage `guaranteed-core-k19`'s findings and apply the ones that hold. This leaf
owns every fix and all post-fix verification — `cargo fmt`, `cargo clippy
--all-targets`, `cargo test` — none of which the review was allowed to run.

## Why it sits here

Adjacent to its review, ahead of `mandate-machinery-k10`. A review's findings are
anchored to a commit and to `path:line` coordinates, and any intervening edit to a
cited file moves them **silently** — nothing errors, the finding simply points
somewhere slightly wrong. `mandate-machinery-k10` edits exactly the files this
review reads.

## Notes

A finding that turns out to be a design disagreement rather than a defect belongs
in the record it disagrees with — `docs/adr/skill-delivers-the-methodology.md` has
a `## Considered options` section written to be argued against, and the spec's
requirements are the other home. Neither is appended to; both are current-state
sets reworked in place.

## Review findings

### F1 — [P1] Keep `finish`'s outcome-dependent ending in the guaranteed core

**Location:** `src/prompt.rs:181-183` (`Kind::Finish => None`), reinforced by
`tests/prompt.rs:281-303`.

The exception removes the very too-late-shaped instruction the core exists to
protect. A `finish` session that tears down the grove must still remember
`complete --done` after all other work; one that externalises work must remember
bare `complete`. Reading those branches in `references/finish.md` at bootstrap
cannot repair a forgotten signal at the end, and forgotten signalling is the
observed failure that motivated this cutover. The safe no-signal outcome limits
damage, but it still stalls an interactive harness or stops the loop after a
successful teardown, so it does not satisfy the ending limb.

The exception also falsifies the record and module's structural claim that every
prompt has exactly one embedded-content part: a `finish` prompt now has none.
Avoid the stated duplication cost by giving the three-outcome table one embedded
source of its own, routing `references/finish.md` to that source, and inlining the
same bytes as `finish`'s final prompt part. That preserves one source/two
deliveries and the timeline shape without stating the wrong fixed action.

### F2 — [P2] The positive half of the closed-fact test can pass with both rules absent

**Location:** `tests/prompt.rs:211-228`.

`the_skill_carries_the_two_rules_the_core_sheds` checks only that two unit-marker
ids occur. Either unit body can be emptied, weakened, or made to state the
opposite while its marker remains, and this test still says the skill states the
rule. The paired negative check would also stay green, leaving the pick's
authority or the stated-VCS rule nowhere while the suite claims both ends of the
closure are asserted.

Assert the meaningful condition text within each located unit (or pin each
unit's source with an explicit review-on-change message). When the marker
machinery is deleted, replace that locator without weakening the semantic
predicate; marker existence is classification evidence, not evidence of what the
condition says.

### F3 — [P2] The ADR rename made historical citations assert the opposite decision

**Location:** `docs/specs/mandate-delivered-methodology.md:1046-1052`; the same
regression appears at lines 298-303, `CHANGELOG.md:261-269` and
`.grove/01-DONE-requirements-plan-k1.md:5-9`.

These passages still describe the mandate-only decision: two delivery paths are
rejected, provisioning is next to retire, and triggering conditions remain
byte-exact prompt slices. Their links were mechanically retargeted to
`skill-delivers-the-methodology`, whose decision is the inverse: provisioning
stays and two channels with one source are the settled design. The links resolve
syntactically but no longer support the claims, so the old spec is not accurate
for the live mandate machinery and the changelog's historical explanation now
misattributes its own release.

Reconcile semantics, not just paths: describe these as claims of the then-current
mandate decision and point at a durable source that actually carries them (the
old spec itself where appropriate), or explicitly say the current ADR later
reversed them. Do not cite the current ADR as support for the rejected design.

### F4 — [P3] The deleted mandate-ending guard is still claimed in the test module header

**Location:** `tests/session_kind_guidance.rs:40-47`.

The header says this file still reads composed mandates through a session-ending
guard and that a twentieth kind fails on all three surfaces. This commit deleted
that guard from the file and moved the surviving prompt claims to
`tests/prompt.rs`; the remaining ending check here is only a source-byte drift
pin. The comment therefore overstates what this module verifies and conflicts
with the new section-level explanation around the deletion. Rewrite the header
to name the two surfaces that remain and route the prompt surface to
`tests/prompt.rs`.

### F5 — [P3] Correct stale numeric and rename prose in the new evidence

**Locations:** `tests/prompt.rs:25-32` and
`docs/specs/skill-delivered-methodology.md:497-502`.

The test comment still says the measured prompt is "well under half" of 4 KiB,
while this same commit records 2,307–2,318 bytes (about 56–57%, leaving 43–44%).
The spec also now says `skill-delivers-the-methodology` was renamed to itself;
the source slug was `mandate-delivers-the-methodology`. Both are small, but they
sit in the evidence being cited to justify this cutover, so keeping them false
makes later review harder. Update them to the measured headroom and the actual
old-to-new rename.

# skill-opening-k18

**Integrates:** skill-opening-k17

## Goal

Make `content/SKILL.md` satisfy the reviewed progressive-disclosure contract:
retain conditions and navigation, move session procedures to their named
destinations, make every deferred condition visibly route after unit markers are
deleted, and close the routing-table test gap found in the producer change.

## Findings

### [High] `SKILL.md` still states procedures in 18 universal units

The spec's review-only obligation is not met
(`docs/specs/skill-delivered-methodology.md:759`). The following passages tell a
session what to do, in sequence or by sentence-to-verb mapping. In every case the
named destination already carries the procedure unless noted otherwise.

| `content/SKILL.md` | unit | procedure retained in the skill | destination evidence |
|---|---|---|---|
| 86 | `skill-session-name` | suggest `/rename`, then move on | `content/references/driver.md:53` |
| 117 | `skill-bootstrap` | resolve, stop, then perform an ordered four-file read | `content/references/bootstrap.md:2` |
| 125 | `skill-execute` | planning finds increments, orders them, then grows the tree | `content/references/execute.md:2` and `content/references/planning.md:2` |
| 132 | `skill-decompose` | two situations map to inline/leaf/node actions and a first-child limit | `content/references/decompose.md:2` |
| 145 | `skill-chain-gap-asymmetry` | tells a review to place integration where `pick` reaches it next | `content/references/decompose.md:162` |
| 172 | `skill-node-close-cascade` | walk the parent chain, then verify and report | `content/references/retire.md:36` |
| 239 | `task-two-shapes` | choose a shape and cut one lazily versus one eagerly | `content/references/decompose.md:79` and `content/TASK-FORMAT.md:119` |
| 255 | `task-declare-the-relationship` | write the relationship line by hand | `content/TASK-FORMAT.md:319`; the skill itself names no destination |
| 303 | `skill-adrs-and-specs` | raise/write/rework/never-append recipe | `content/references/execute.md:38` |
| 317 | `skill-glossary-is-load-bearing` | read, append inline, and constrain the glossary's contents | `content/references/grove.md:33` and `content/CONTEXT-FORMAT.md:2` |
| 338 | `driving-when-to-commission-prior-art-research` | insert at a specified point and select solo versus pair | `content/driving.md:2` |
| 348 | `driving-when-to-retire-research-into-adrs` | map changed/confirmed findings to exact record edits | `content/driving.md:200` |
| 355 | `driving-when-code-depends-on-a-framework-version` | read, fetch, cite, flag — an explicit four-step sequence | `content/driving.md:242` |
| 366 | `driving-when-asserting-a-repo-wide-claim` | add two controls and enumerate-then-classify | `content/driving.md:274` |
| 375 | `driving-recording-fog` | apply the fog-or-ticket test and choose note versus leaf | `content/driving.md:582` and `content/BRIEF-FORMAT.md:56` |
| 386 | `driving-when-a-leafs-place-is-in-doubt` | map three sentences to reorder/issue/prune, then select the CLI verb | `content/driving.md:599` |
| 395 | `driving-no-session-summary` | record decisions in the running log and ADR set as they land | `content/driving.md:180` |
| 403 | `driving-ask-about-the-trade-off` | name a trade-off, recommend an answer, and supply evidence | no corresponding procedural destination exists yet; add one to `content/driving.md` |

This is not inferred from either passing size alarm. The evidence is the prose
above set against its destination, exactly as the spec requires. Most conditions
can become one sentence plus a path; `skill-decompose` is the spec's worked
example and should lose the two verb mappings, first-child instruction, inline
bar, and rationale from the skill page.

### [High] Six deferred conditions lose their route when markers disappear

These units have a `defers=` marker today but name no destination in their visible
prose. `mandate-machinery-k10` deletes the markers, so a future session sees the
condition and has no way to reach the procedure.

| `content/SKILL.md` | unit | visible destination to add |
|---|---|---|
| 107 | `skill-do-not-pick-again` | `references/driver.md` |
| 152 | `skill-no-exception-to-check` | `references/decompose.md` |
| 167 | `skill-pruning-is-hitl` | `references/retire.md` |
| 203 | `task-kind-in-the-filename` | `TASK-FORMAT.md` |
| 235 | `task-too-big-is-planning` | `TASK-FORMAT.md` |
| 395 | `driving-no-session-summary` | `driving.md` |

The missing paths are independently actionable even where the condition prose is
otherwise concise. `driving-no-session-summary` has both defects: its procedure
must move and its remaining condition must route.

### [Medium] The new routing-table test does not test the routing table

`tests/methodology.rs:150` extracts only the right-hand path into a
`BTreeSet`. It discards the left-hand kind/family labels and collapses duplicate
rows. Therefore a table with a missing kind, a duplicated kind, extra rows, or a
path attached to the wrong kind still passes as long as all ten distinct path
strings occur somewhere. This contradicts the test's own claim at
`tests/methodology.rs:131` that it checks the ten human-readable rows. Parse and
assert the ten expected `(kind label, path)` mappings (or equivalently assert
both exact row count and exact pairs), then retain the file-existence check.

## Complete classification ledger

Every unit in `content/SKILL.md` is accounted for below. `P` is procedure to
move (the first finding); `N` is navigation or a transaction boundary that the
spec requires on the skill page; `C` is a condition/fact with no session
procedure. `R` is an additional missing visible route (the second finding).

- **P:** `skill-session-name`, `skill-bootstrap`, `skill-execute`,
  `skill-decompose`, `skill-chain-gap-asymmetry`, `skill-node-close-cascade`,
  `task-two-shapes`, `task-declare-the-relationship`, `skill-adrs-and-specs`,
  `skill-glossary-is-load-bearing`,
  `driving-when-to-commission-prior-art-research`,
  `driving-when-to-retire-research-into-adrs`,
  `driving-when-code-depends-on-a-framework-version`,
  `driving-when-asserting-a-repo-wide-claim`, `driving-recording-fog`,
  `driving-when-a-leafs-place-is-in-doubt`, `driving-no-session-summary`,
  `driving-ask-about-the-trade-off`.
- **N:** `skill-kind-routing` (navigation is the opening's specified job),
  `skill-retire` and `skill-commit` (the Retire-before-Commit boundary is the
  loop/transaction contract, not either VCS lane's mechanics). This disagrees
  with the producer's tentative `skill-commit` lead: its ordering statement is
  structural and should stay, while `references/commit.md:2` correctly owns how
  each VCS realizes the boundary.
- **C:** `skill-what-a-grove-is`, `skill-spine-constraints`,
  `skill-working-tree`, `skill-bare-grove-dispatch`,
  `skill-self-driving-loop`, `skill-one-configuration`,
  `skill-starting-a-new-grove`, `skill-pick`, `skill-do-not-pick-again`,
  `skill-stated-vcs-is-definitive`, `skill-bare-stem-rule`,
  `skill-no-exception-to-check`, `skill-retirement-touches-one-filename`,
  `skill-pruning-is-hitl`, `skill-finish`, `task-leaf-filename`,
  `task-kind-in-the-filename`, `task-nineteen-kinds`, `task-hitl-afk`,
  `task-in-session-doubt-budget`, `task-too-big-is-planning`,
  `task-no-node-for-a-shape`, `task-grammar-is-five-fields`,
  `task-nothing-in-a-body-is-metadata`, `task-three-design-kinds`,
  `task-deliverable-split-not-a-gate`, `skill-artifacts`,
  `spec-when-a-spec-is-written`, `skill-briefs-vs-glossary`,
  `skill-linkuistics-prerequisite`.
- **R overlay:** `skill-do-not-pick-again`, `skill-no-exception-to-check`,
  `skill-pruning-is-hitl`, `task-kind-in-the-filename`,
  `task-too-big-is-planning`, `driving-no-session-summary`.

The two rules shed by the guaranteed core are both present and forceful:
`skill-do-not-pick-again` at `content/SKILL.md:107` says a second walk can
disagree and **the mandate wins**; `skill-stated-vcs-is-definitive` at
`content/SKILL.md:112` says **do not re-derive it** and explicitly rejects a
harness banner. The first still needs its visible `references/driver.md` route;
the rule itself is intact.

## Context

- Producer: `skill-opening-k16`, commit `d30d0959e40c`.
- Review: `skill-opening-k17`.
- Contract: `docs/specs/skill-delivered-methodology.md:730` and especially the
  review scenario at line 759.
- Do not use a passing body/loop budget as evidence that the semantic obligation
  is met.

## Done when

- All 18 `P` passages above are reduced to their condition and visible route;
  their procedural content remains present in the destination, with the missing
  trade-off procedure added to `content/driving.md`.
- All six `R` passages visibly name their destination without relying on unit
  markers.
- The `N` boundary/navigation passages remain strong, and both core-shed rules
  remain explicit.
- The routing-table test asserts the exact ten label→path rows and the existence
  of every destination.
- Post-fix tests, formatting, linting, and corpus-budget verification pass; this
  review ran none of them, by design.

## Notes

The frontmatter description itself follows the house capability + explicit
`Use when` shape, remains model-invoked, and does not summarize a workflow. The
new ten-row opening is appropriate navigation; the defect is in the test's
ability to prove the human-visible mapping, not in the current table contents.

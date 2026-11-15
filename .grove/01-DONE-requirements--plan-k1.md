# plan-k1

## Goal

Establish changes to Grove's planning and decomposition rules that keep work
advancing toward the grove's goal instead of repeatedly deferring it into new
tasks, especially implementation and review cycles. Orient groves around
incremental delivery: make something useful work, then improve it in subsequent
groves rather than seeking perfection before the first delivery.

## Context

The human reports multiple live groves repeatedly decomposing and repeating
tasks without making progress. Inspect the live task trees in the two other
Grove project workspaces, APIAnyware, and Koine before proposing the rules.

## Done when

- The observed failure patterns and the human's intended outcome are recorded.
- Requirements distinguish useful decomposition and corrective review from
  repetition that defers the same unresolved work.
- The human's intended delivery and review boundaries are confirmed; scope and
  concrete verification cases are recorded for implementation.

## Notes

This is requirements work. Reading another live grove is evidence gathering;
it does not authorize changing that grove's task tree.

## Evidence from the four live groves

These observations concern task-file contracts and recorded results. They are
not independent verification of those projects' code. A read-only snapshot of
the four trees was stable across the read (no changed files or membership).
Full snapshot, including source paths and hashes:
`/var/folders/z1/nw_wk3gj3hqchs4_73xrf2dw0000gn/T/grove-progress-evidence-60vuncj4/snapshot.json`.
That temporary snapshot is a convenience; the conclusions below are recorded
here so successors do not need to reconstruct the investigation.

- **Grove documentation/editorial pipeline:** 481 task files, 36 live, depth 5.
  The original goal is useful, evaluated documentation/editorial guidance.
  `document-growing-original-content-and-parent-history-k611` narrowed its work
  to first-publication journal seals and transferred the original aggregate to
  k612. Its reported red/green controls and 412 passing checks show useful local
  work. k613 found two low-priority issues; k614 fixed one and explicitly accepted
  the other as a trade-off, a positive example of bounded integration. Later,
  `document-enabled-complete-original-ownership-k624` narrowed again to 55 fixture
  migrations and transferred aggregate acceptance to k627. Local correctness
  and preserved ownership alone do not establish delivery of the original goal.
  Source root:
  `/Users/antony/Development/AgenticOrchestration/grove.gh-issue-14-documentation-skills-for-editorial-pipeline/.grove`.
- **Grove design/decomposition:** 915 task files, 21 live, depth 16. The original
  goal is decomposition guidance for composable components and easier reasoning
  and verification. Its runtime continuation
  `case-x-herdr-manager-runtime-k1015` records its own loop audit: 14 retired
  implementation slices, 14 reviews and four integrations since k979 repaired
  real defects while repeatedly moving the entire original runtime charter
  forward. The composed runtime remained unproved. k1015 already responds to a
  human intervention by requiring an aggregate acceptance attempt, keeping
  necessary repairs in the leaf, and explicitly reporting a blocker with the
  leaf live. The current task is 1,033 lines / 73,176 characters, including
  accumulated historical continuations. k1014 independently checks the no-more-
  peripheral-slices contract. This is direct evidence for the failure mechanism
  and a concrete candidate response, not a claim the response has succeeded.
  Source root:
  `/Users/antony/Development/AgenticOrchestration/grove.improve-initial-design-decomposition-skill/.grove`.
- **APIAnyware:** 1,540 task files, one live, depth 0 after flattening. The current
  increment owns 64 exact original smoke-probe slots; k1497 delivered 16, while
  the latest continuation still owns 44 remaining executions and the original
  aggregate join. The same remaining-execution subject occurs in 45 tasks.
  k1640 builds a pair reader, k1642 reads pairs, k1645 writes a human-readable
  evidence map, and k1647 inherits the original acceptance. Their task contracts
  explicitly award no new slot or aggregate admission from those preparatory
  artifacts. k1645 follows an already reviewed readback with no actionable
  findings, yet creates another required review before dependent use. This
  shows flat siblings and honest carry-forward can still leave the goal waiting
  behind expanding preparation and review. It does not establish zero progress
  across the entire grove or all 45 tasks.
  Source root:
  `/Users/antony/Development/CrossLanguageDevelopment/APIAnyware.add-ocaml-target/.grove`.
- **Koine:** 114 task files, ten live, depth 3. The original goal is working
  front-window/coordinate/resize behavior plus a Settings repair. Ancestors
  explicitly retain Settings implementation and ordinary acceptance. k137 designs
  a finite caller/collection workload; integrations k139 and k144 route further
  numerical and evidence-scope obligations to k126 while implementation/native
  acceptance remains unavailable. This exposes prerequisite expansion across
  different task names: a same-slug/review-count detector would miss it. Existing
  closed runtime caps and original acceptance remain binding.
  Source root:
  `/Users/antony/Development/AbstractingComputers/Koine.gh-issue-14/.grove`.

## Rule interaction to resolve

The shared decompose procedure defaults to externalizing new work, permits
narrowing a leaf while moving every original requirement to siblings, and says
externalization is cheap. Its vertical-slice and flat-placement rules already
discourage unusable increments and excess depth. The examples show that satisfying
placement and ownership does not prevent repeated aggregate deferral. Declared
pass series have a cap and consequence; that rule does not cover ordinary
implementation/review/prerequisite continuations. Review is earned by a
load-bearing artifact, but successive preparation artifacts can each claim that
boundary without reducing the original acceptance work.

## Decisions (running log)

The human wants Grove's planning and decomposition rules updated to ensure
progress against the goal. The motivating failure is multiple currently open
groves endlessly decomposing and repeating tasks, usually an impl/review-impl
cycle, without making progress. The requested evidence includes the other two
Grove project groves, APIAnyware, and Koine.

- 2026-11-16: The human agreed that repeated handoffs of original acceptance work
  should give way to an attempt at that acceptance in the current task. Keep the
  task open, retain necessary repairs in it, require a concrete reason that any
  further split advances the goal, and report a real blocker explicitly.
- 2026-11-16: The human agreed that review should create more work only for defects
  that block agreed acceptance or present a material risk. Record optional polish
  separately; reopening reviewed work requires a concrete change or new evidence.
- 2026-11-16: The human emphasized that the goal is to make something work and
  advance the goal, rather than seek perfection from the start. Reorient the
  process toward incremental delivery and improvement, with improvements delivered
  in subsequent groves. Current groves run too long.

## Implementation scope

The decisions are settled enough for one implementation leaf; another research,
design or planning session would repeat work rather than resolve remaining fog.
Change the shared methodology and planning guidance in this repository. Give
each rule one canonical owner and make related guidance consistent with it.

- Define a grove around the smallest useful working outcome, with a clear,
  proportionate acceptance bar and a path to demonstrating that outcome early.
  Subsequent improvements belong in subsequent groves. Mandatory safety,
  compatibility and quality constraints remain binding.
- Permit necessary preparation, design and multi-session work when it has a
  specific, bounded connection to that delivery. More tree, deeper analysis,
  more passing checks, and a preserved remainder are not sufficient evidence
  that the original goal advanced.
- Make another split justify what acceptance work it completes or directly
  unblocks. When the same acceptance keeps moving unchanged, attempt it in the
  current task and keep necessary repairs there. A real blocker stays explicit;
  a session limit is not permission to claim success or repeat the charter.
- Apply the agreed acceptance/material-risk threshold to reviews, integrations
  and prerequisites. Ordinary verification and fix checks stay in-session;
  independence remains available when there is a concrete reason for it.
- Further improvements and optional hardening must not silently enlarge the
  current grove. If existing agreed acceptance is itself too broad, surface the
  scope decision rather than unilaterally dropping it or transferring it forever.

This is a guidance change, not a new runtime gate, task state, filename field,
loop detector or generic numeric cap. Do not modify or prune the four evidence
groves. Preserve existing declared caps and the requirement for human decisions
on pruning or acceptance changes.

## Verification plan

Use the existing checks relevant to the edited corpus and a bounded walkthrough
of the recorded cases. No new test framework, cross-project measurement campaign,
or rerun of the other projects is needed. The required contrasts are:

1. The 14-slice runtime sequence cannot justify a fifteenth peripheral repair
   plus unchanged remainder; it must attempt the composed acceptance or report
   the actual blocker.
2. Flat APIAnyware-style reader/readback/map/review preparation cannot keep
   replacing the original execution goal without a direct, bounded delivery
   dependency. A necessary reader used to complete a specified acceptance step
   is still allowed.
3. Koine-style prerequisite growth is assessed against the next working behavior
   even when successive tasks have different names.
4. A bounded real defect fix, with an explicitly accepted trade-off as in the
   editorial integration, remains valid. Material risks and required adverse
   checks cannot be dismissed as optional polish.
5. A useful first increment can finish with later improvements recorded for a
   subsequent grove. Optional perfection work cannot grow the current acceptance
   bar; existing agreed obligations cannot silently disappear.
6. Legitimate working slices, necessary expand/migrate/contract work and a
   warranted independent review remain possible. Count, depth or identical task
   names alone do not determine whether progress occurred.

These cases assess the guidance's decisions. They are not a claim that prose
alone mechanically prevents every future agent from looping.

## Handoff verification

Created incremental-delivery-k2 as the sole implementation follow-up. Its handle
resolves to the expected live file; both task headers and required sections were
checked, and the root contains only these two tasks and its brief. All recorded
tree counts, live counts, maximum depths and snapshot-stability results agree
with the captured evidence. This leaf changes requirements and handoff artifacts;
verification of the actual guidance change belongs to incremental-delivery-k2.

# tui-k7

**Integrates:** tui-k6

## Goal

Triage the `review-planning` findings recorded in `tui-k6`'s own commit and
apply the real ones to the planned subtree — the root brief's design outline
and the implementation leaves `tree-viewer-k3`, `markdown-viewer-k4` and
`live-viewer-k5` — so the first implementation session opens on a plan that
fits one session and names every obligation it carries.

## Context

- Read the findings from `tui-k6` (`.grove/03-DONE-review-planning--tui-k6.md`
  in this tree; the review's commit names the handle). They are the reviewer's
  list, not this leaf's charter: reject a finding on its merits where the
  evidence does not hold, and say why in the running log.
- The producer is `tui-k2`; the root brief owns the requirements and test
  seams, which are settled and not reopened here.
- The findings cite `docs/specs/walkthrough-books.md`,
  `docs/ordinal-fs-tree/ARCHITECTURE.md`, `docs/ARCHITECTURE.md` (§Tree access
  lock), `crates/grove-llm/tests/tree_lock.rs`,
  `crates/book-validation/tests/corpus_validation.rs`, `release.toml` and
  `CHANGELOG.md`. Verify each against current source before acting on it.

## Done when

- Every finding has a disposition in this leaf's running log: applied,
  rejected with a reason, or narrowed.
- The first implementation leaf, as it now stands, fits one focused session
  and is runnable on its own; where the review's recommended cut is taken,
  the new leaves are in tree order with their own bodies and the root brief's
  decomposition list is reconciled.
- Every implementation leaf that edits a book-reconstructed source names the
  affected walkthrough book(s), the changelog, and any release metadata the
  new crate needs.
- Recorded design rules the plan reverses are named in the plan together with
  where the reversal is written down.
- Behavior the review found unspecified (node outcome labels, in-process
  state, restoration evidence) is specified in the root brief or the owning
  leaf.
- No production or test code is written here; this leaf edits `.grove/` only.

## Notes

The review sits at `03`; this leaf was inserted ahead of the first
implementation sibling so the plan is reconciled before consumers build on it.
Leaf paths shifted by one position on insertion; handles did not.

## Decisions (running log)

- F1 applied (real sizing issue): retain `tree-viewer-k3` as the basic runnable
  browser and insert quiet-read and interaction/hardening siblings before
  Markdown. The original leaf spans CLI, three libraries, full interaction,
  terminal fault handling and their documentation. Each new slice has its own
  runnable consumer. Accept the existing blocking reader and waiting diagnostic
  as a documented intermediate limit, removed by the immediately following
  quiet-read slice; the final responsiveness contract is unchanged.
- F2 applied (real missing obligations): the three book manifests explicitly
  own the cited source files; the corpus test checks 17 files / 8,845 lines
  against the live repository. Each source-owning slice must update affected
  fragments, ownership ranges and changed count assertions with final book
  validation. Every implementation slice logs its change under Unreleased;
  the new library inherits the workspace version and sets release = false,
  following the policy in release.toml and docs/RELEASING.md.
- F3 applied (real recorded-rule reversal): the library module/lock comments
  and both architecture documents explicitly forbid try-read. The quiet-read
  slice amends them to allow one observer acquisition that returns Busy without
  a snapshot. Existing blocking readers retain their behavior. The tree_lock
  assertion currently pins five fs-module mentions in task_tree.rs; update the
  count and explanation while preserving that sole production acquisition owner.
- F4 applied (unclear contract): derive branch/root display status from descendant
  leaves: LIVE if any live; otherwise DONE if any done; otherwise ABANDONED if
  any abandoned; otherwise EMPTY. Mixed terminal work is DONE with descendant
  outcome counts, so abandoned work stays visible. This is display aggregation,
  not a stored node outcome or a change to Grove's selection logic.
- F5 narrowed (real evidence gap): use an actual-binary PTY test for normal exit
  and signals, and a test-only child exercising the production terminal lifetime
  for injected errors, partial initialization and an actual unwinding panic.
  Assert terminal attributes and restoration output where the endpoint remains
  usable. Hook installation alone does not prove restoration; closing the PTY
  master cannot also prove output delivered to that closed master. No shipped
  panic flag is needed. The hardening slice owns this evidence.
- F6 narrowed (unnecessary test-process restriction): permit a fresh open file
  description in-process for bounded nonblocking acquisition tests, following
  tree_lock.rs's real-lock helper. Retain the separate real-mutator process as
  integration evidence. No dependency on an external flock executable; exercise
  the chosen lock fixture on both supported targets instead of relying on the
  review's unverified macOS lockf explanation.
- F7 applied (contract wording): cli.rs currently asserts no subcommands and no
  selector arguments. Replace the former with exactly the view subcommand,
  retain the latter for bare grove, and reconcile usage inventory row G4 so
  viewing a directory does not become a launch-policy/workstream selector.
- F8 applied (unclear contract): all viewer state lives only in process memory;
  no persisted config, cache or state file anywhere. Process exit discards it.
- F9 narrowed (sizing note): retain the live leaf for now and name the proposed
  observation/identity versus content-anchor/recovery seam for decomposition if
  execution demonstrates it is needed. No missing behavior justifies a new
  speculative leaf today.
- F10 applied (help clarification): the brief already chooses no upward search.
  Make that visible in view help and usage, including invocation from a
  subdirectory. This does not change bare grove's workspace resolution.
- Tree cut settled: `responsive-viewer-k8` and `viewer-interaction-k9` now sit
  between `tree-viewer-k3` and `markdown-viewer-k4`. Existing handles survive;
  later dependency pointers and the root decomposition agree with that order.
- Verification scope: Tier 2, graph generation 2026-09-15T07:00:01Z with
  matching source metadata for the cited Rust paths. Exact test/function snippets
  confirm the count, facade and CLI claims. Documentation is excluded from the
  index and was read directly, as were all changed task files. Coverage is a
  best-effort signal, not an exhaustive proof. No production/test source changes
  belong to this planning integration.
- Final validation: read the complete jj diff against this leaf's Done when;
  all F1–F10 have dispositions and the five implementation slices retain the
  final root contract with explicit intermediate limits. `grove-llm resolve`
  returned each implementation handle at positions 05–09, and `brief-chain`
  for this task returned only the root brief. `jj diff --name-only` contains
  only .grove artifacts. No Rust test run is claimed for this prose/tree-only
  change. The root retains live implementation leaves, so no ancestor closes.

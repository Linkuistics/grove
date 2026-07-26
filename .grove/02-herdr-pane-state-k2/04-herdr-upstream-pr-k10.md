# herdr-upstream-pr-k10

**Kind:** work

## Goal

Take the two-hunk authority fix upstream, and file the pane mis-detection bug
as an issue. Both are outward-facing work in `ogulcancelik/herdr`; neither
blocks anything else in this grove.

If the PR merges, the fork's carry for this patch drops to zero *and*
`approve-merged-contributor.yml` adds `AntonyBlakey` to
`.github/APPROVED_CONTRIBUTORS` — which is also the cheapest route to getting
the fork's existing `ui-layout` feature work upstream. If it stalls or is
declined, nothing here is affected: `herdr-authority-patch-k9` already shipped
the fix on the fork.

**HITL on submission.** Opening a PR and filing an issue are outward-facing and
hard to retract. Draft both, show them to the human, and submit only on
explicit confirmation.

## Context

Upstream's PR intake changed on 2026-07-23 (`2debcec7 fix: restrict unapproved
pull requests to bug fixes`, `e1a32e58 ci: harden pull request workflow
execution`, `da40b1aa ci: gate pull request review by scope`). `AntonyBlakey` is
**not** in `.github/APPROVED_CONTRIBUTORS`, so the constraints are real:

- Title must be `fix: …` or `fix(scope): …`.
- ≤20 files and ≤1000 total added-or-deleted lines. The patch is 2 hunks in
  1 file plus a test — comfortably inside.
- Features and larger changes need maintainer approval first. **Do not** attach
  a staleness-TTL hunk, an allowlist entry, or anything else; keeping this at
  two hunks and one principle is a deliberate decision (see
  `01-DONE-herdr-authority-route-k7.md`), because a debatable second change
  risks the defensible first one.

Re-read the current templates and policy before drafting — they are days old
and may have moved again.

**The PR branch already exists.** `herdr-authority-patch-k9` pushed
`AntonyBlakey/herdr@authority-fix`, branched off `upstream/master` and carrying
exactly one commit — the two hunks plus two tests, 1 file, +74/−2, message
already written in `fix:` form with the CLI reproduction in its body. Submit from
that branch; do **not** submit from `ui-layout`, which also carries the two
unsubmitted `feat:` commits and would blow the scope gate. Rebase it onto current
`upstream/master` first if upstream has moved, and re-run the suite with the
build-environment settings the node brief records (`ZIG`, toolchain pin) — the
`pre-commit` hook needs them.

## Done when

- A PR is open upstream, titled about like
  `fix: separate session identity from lifecycle state in hook reports`, with:
  - the reproduction (`herdr pane report-agent` against a pane whose session is
    owned by `herdr:claude` — report silently dropped, CLI exit 0, the pane's
    `agent` and `agent_status` unchanged). **Do not cite `revision`** as the
    dropped-report evidence, as earlier notes in this node did: `revision`
    tracks `report_metadata` token changes only and never moves for a state
    report, landed or dropped, so a maintainer would rightly reject it as the
    wrong field. `k9` measured the correct observables;
  - the argument stated as a **bug**, not as a grove feature request. herdr
    conflates two separable concerns, and the fix helps every third-party
    reporter. Naming grove as the motivating case is honest and fine; framing
    the PR as "please support grove" is not, and would fail the scope gate on
    its own terms;
  - the new test from `herdr-authority-patch-k9` included.
- An issue is filed for the pane mis-detection, with the reproduction already
  in hand: MCP servers inherit the harness's foreground process group, so a
  `codex` MCP server running under `claude` makes herdr identify the pane as
  codex and evaluate the wrong screen manifest. Measured across every claude
  pane in one instance — `wQ:p1`, `wJ:p1`, `wP:p1`, all `cwd`s running claude,
  all detected as codex, all reporting `agent_status: "idle"` while actively
  mid-turn.

  herdr's issue template requires an actual reproduction and explicitly refuses
  speculative reports, feature requests, and proposals — this one qualifies as
  a bug report and must be written as one.
- Outcomes recorded: PR and issue URLs go in this leaf's commit message, so
  they survive `.grove/`'s deletion at the finish cycle. If the PR merges, say
  so in *herdr-optional-ui* — that turns the fork from a permanent carry into a
  temporary one, which is a consequence worth recording.

## Notes

This leaf does **not** wait for a merge. Opening the PR and filing the issue
completes it; upstream's response is not ours to schedule, and blocking a leaf
on someone else's review queue would stall the loop for no gain.

**Scope guard**: no other upstream contributions ride along. `ui-layout` is
pre-existing fork work and is not this grove's business.

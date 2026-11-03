# ravel-token-handoff-k140

**Reviews:** ravel-token-handoff-k139

## Goal

Adversarially review the integrated token handoff design before
ravel-token-handoff-k133 implements it. ravel-token-handoff-k139 applied the
five findings of ravel-token-handoff-k138. It also applied the findings of an
in-session reviewer, and several of those fixes are new rules that no review
has read.

## Context

- Ravel commit `9084e0dcfcd733a1185ca1bb26a312b53fb25549` on the unpushed
  bookmark `skill-directory-delivery`, in the `Ravel.skill-directory-delivery`
  workspace. Read its diff from `7c5b6cdd3842`. Follow that repository's VCS
  discipline.
- Its artifacts: `docs/specs/token-handoff.md`, the "Confinement" section of
  `session-execution.md`, the "Read access and delegation" section of
  `skill-discovery.md`, the "Credentials" section of
  `configuration-evaluation.md`, the decision record
  `a-confined-harness-receives-an-owner-minted-token` and the views in
  `docs/design/token-handoff/`.
- The current code: `crates/ravel/src/run.rs`,
  `crates/ravel/src/confinement.rs`, `crates/ravel/src/skills/mod.rs`,
  `crates/keyed-launch/src/run.rs`, `crates/keyed-launch/src/confinement.rs`
  and `crates/ravel/src/rhai_worker/harness.rs`.
- The decision log of ravel-token-handoff-k139, and the "Token handoff design
  integration" section of `docs/common-loading-acceptance-record.md`.

The human chose this route on 2026-11-03, and the route itself is closed.
Review how the specification states it.

## Done when

- The review challenges these doubts of the integration:
  - The runner grants each writable root and runtime read in its checked form,
    and it refuses one that now resolves to another path. Does the rule hold
    on Seatbelt and on bubblewrap? The selected program stays outside the
    overlap check. Is that exception acceptable?
  - The protection refuses a declared file with a second link, in the source
    check and in the confinement check, and the read checks again. Does that
    refuse an ordinary owner workflow? Are the refusal codes right?
  - Each launch resolves the protected paths once, and its source check,
    confinement check and read share that resolution. Each harness run
    resolves them again.
  - A dangling declared link protects the prospective path of its target.
  - The nine ordered steps of the read. The open follows no link in the last
    component and does not block. An interval stays open between steps 2
    and 3.
  - The runner's descriptor list, and the ownership of both socket ends
    before, during and after the spawn.
  - The read order of Claude Code against hooks and MCP servers is now a
    stated verification limit. Is that limit enough for an implementation?
- The review checks the specification against Ravel's current launch,
  confinement, skill-capture and harness-run seams, and against its other
  specifications.
- The review produces findings only. It cuts an `integrate-review-design` leaf
  before ravel-token-handoff-k133 only for actionable findings.

## Notes

Read no credential, and make no model call. A probe with dummy files under a
copy of the Seatbelt profile is allowed, as ravel-token-handoff-k139 used
`scripts/harness-trials/profile-copy.py`. A check that needs the owner's
token waits for the human's approval in ravel-token-handoff-k133.

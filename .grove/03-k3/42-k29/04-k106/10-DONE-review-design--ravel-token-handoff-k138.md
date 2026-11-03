# ravel-token-handoff-k138

**Reviews:** ravel-token-handoff-k132

## Goal

Adversarially review Ravel's token handoff design before
ravel-token-handoff-k133 implements it. The design hands a confined harness a
token that the owner minted, through an inherited socket.

## Context

- Ravel commit `7c5b6cdd3842` on the unpushed bookmark
  `skill-directory-delivery`, in the `Ravel.skill-directory-delivery`
  workspace. Follow that repository's VCS discipline.
- Its artifacts: `docs/specs/token-handoff.md`, the decision record
  `a-confined-harness-receives-an-owner-minted-token`, the edits of
  `configuration-evaluation.md`, `session-execution.md`, `CONTEXT.md` and the
  decision record `a-configuration-script-asks-a-model-through-a-harness-run`,
  and the views in `docs/design/token-handoff/`.
- The current code: `crates/ravel/src/confinement.rs` and
  `crates/keyed-launch/src/confinement.rs`, with the launch path of the
  runner.
- The decision log of ravel-token-handoff-k132, and the "Token handoff
  design" section of `docs/common-loading-acceptance-record.md`.

The human chose this route on 2026-11-03, after the first route conflicted
with Anthropic's terms. The route itself is closed. Review how the
specification states it.

## Done when

- The review challenges these doubts of the producer:
  - Bubblewrap passes an inherited descriptor to the program that it launches.
    No run measured this.
  - Claude Code 2.1.317 reads a socket directly. The producer took this
    branch from minified code, and two functions share the minified name.
  - The order: a shape check of the settings, a path check before the worker,
    and a file check and read after the selection. Look for a gap between the
    path check and the read.
  - An unconfined launch ignores a named handoff without a refusal or a notice
    entry other than an empty list.
  - The protected-path rule names "the code that an overlap with the settings
    file receives". Grants and skill directories use different codes today.
  - The new optional fields in schema 4 and in routing notice schema 2.
  - The claims of the section "What a confined process can read".
- The review checks the specification against Ravel's current launch,
  confinement and harness-run seams, and against its other specifications.
- The review produces findings only. It cuts an `integrate-review-design`
  leaf before ravel-token-handoff-k133 only for actionable findings.

## Notes

Read no credential. A check that needs the owner's token waits for the
human's approval in ravel-token-handoff-k133.

## Decisions (running log)

**Review evidence.** The review uses Ravel commit
`7c5b6cdd38427b21dac8a5307dadeac3cebace1a`, the current launch source and the
installed Claude Code 2.1.317 executable. It reads no credential and runs no
test, build, lint, formatter or harness. The graph locates the launch seams;
direct source supplies the evidence where the graph is stale or excludes docs.

**Actionable findings.** The findings are recorded in
`docs/common-loading-acceptance-record.md`, under "Token handoff design
review". The handoff needs a stable file identity, protection before skill
capture and an explicit runner contract for the socket descriptors. Two
security statements also need their scope corrected. The owner's chosen route
remains settled. An integration follows this review before implementation.

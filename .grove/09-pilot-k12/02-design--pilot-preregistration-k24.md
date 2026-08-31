# pilot-preregistration-k24

## Goal

Write and commit, **before any drafting happens**, the four things that make the
pilot a measurement: the judged outcome, the alternative the six stages must
beat, the attribution rule, and the decision rule.

## Context

- The root brief's *Notes*: "the measure is preregistered, not reported".
  Recording what each stage changed is activity provenance and will justify any
  pipeline after the fact. Without preregistration any pipeline can be called
  derived after the fact — which is the failure this leaf exists to prevent.
- The four required parts, stated in the brief and not negotiable in their
  presence, only in their content:
  1. **the judged outcome** — what a reader-facing improvement *is*, judged how,
     by whom or what;
  2. **the alternative to beat** — two stages, draft and proof, is the stated
     fallback;
  3. **the attribution rule** — how a change is credited to a stage;
  4. **the decision rule** — how the evidence maps to keeping, merging or
     dropping a stage.
- `book-check` proves structure and reconstruction and **never whether a page
  explains anything**. That is exactly the gap the judged outcome has to fill,
  and it is why the existing machinery cannot supply this measure.
- The in-house worked example is `docs/evaluations/writing-code-walkthroughs/` —
  a frozen rubric with its SHA-256 recorded, three same-case prompts, binary
  atomic scoring, repetitions per arm, an interleaved contemporary control, a
  named primary endpoint set, a regression guard, and explicit rules for what a
  missing sample does and does not license. Its README's *Verdict* is what an
  honest negative result reads like. Read it and
  `/Users/antony/Development/grove.gh-issue-12` before inventing a protocol.
- `/Users/antony/Development/Writegood` carries the judging protocol and the
  argument that measurement must precede machinery.

## Done when

- A preregistration document is committed under `docs/evaluations/` carrying all
  four parts, each stated precisely enough that a later session cannot satisfy it
  by reinterpretation.
- It states what a **negative** result looks like — the evidence under which the
  six stages lose to two — and says that such a result is a successful outcome.
- It states what makes a run invalid and what an invalid or missing observation
  licenses, rather than leaving a partial-sample arithmetic hole.
- Its own inputs are digested or otherwise frozen, so that the instrument cannot
  be adjusted mid-reading.
- `bash scripts/check.sh` passes.

## Notes

**Nothing may be drafted in this session.** The whole value of the artifact is
that it precedes the thing it judges; a preregistration written with a draft
already in hand is a report.

**Six stages, and one of them is unbuilt.** Art has no format, no convention and
no validator support, and the pilot runs it by hand. The attribution rule has to
be able to credit a stage that produced Markdown figures with no machinery behind
them, or the art stage is unmeasurable by construction and the decision rule
drops it for the wrong reason.

**This is a load-bearing artifact with no human present downstream.** Consider
`review-design` as your last act — and if you cut one, `leaf-insert` it at
`jj-workspace-book-k25` rather than appending it. Appending puts it after the
drafting and the measurement this document exists to constrain, which makes it a
report on the experiment rather than a guard on it; of every review in this
campaign, this is the one where arriving late costs most. Any
`integrate-review-design` that review cuts is placed the same way.

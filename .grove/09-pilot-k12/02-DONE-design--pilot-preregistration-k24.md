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

## Decisions (running log)

**The artifact is one document at `docs/evaluations/editorial-pipeline-pilot/preregistration.md`,
and it fixes the report's destination too.** The precedent directory
`docs/evaluations/writing-code-walkthroughs/` puts the frozen instrument at
`baseline/rubric.md` and the report at `README.md`; this campaign is smaller, so
the instrument is one file and the report is the directory's `README.md`.
`pilot-measure-k26`'s brief and `measurement-report-k45` both say only "under
`docs/evaluations/`", which would let the report land anywhere; naming the path
here is a preregistration's job, because "we will decide where it goes" is a
place a later session can satisfy the obligation by reinterpretation. Rejected:
a single flat file under `docs/evaluations/`, which leaves the five stage records
and the report with no directory to share.

**The judged outcome is defect-closure against a taxonomy derived from the
already-frozen prose contract, not a new rubric.** `docs/specs/walkthrough-books.md`'s
*Prose contract* and `docs/specs/jj-workspace-book-structure.md` already state,
in operational terms, what this book owes a reader: the four self-containedness
questions, the five fragment-introduction questions, direct declarative style,
the four failure categories, the repetition conditions, the example-before-
catalogue rule, and the three things each chapter's prose owes. A taxonomy drawn
from those is frozen before the draft exists by construction, is the same
standard the stages are already told to work to, and cannot be accused of having
been shaped to fit what the stages turned out to do. Rejected: authoring a fresh
reader-comprehension rubric with its own criteria, which would be a second,
competing statement of the same contract and would put the pilot's verdict at the
mercy of whichever of the two a stage read.

**The alternative to beat is instantiated as a proof-only probe over the draft,
produced by a context that never sees any later state.** The pilot runs one arm,
not two: six stages in sequence over one book, with no second book drafted and
proofed alone. Without a second arm "the alternative to beat" is a slogan, and
every middle stage's contribution reads as marginal by default. The probe is the
cheapest real second arm available — one blind context, given the draft and the
proof charter, enumerating the defects it would fix — and the marginal
contribution of a middle stage is what the probe did **not** reach. Rejected:
actually running a proof-only edit arm on a branch, which is an implementation
leaf this session may not cut and would double the drafting cost; and rejected:
treating the proof stage's own diff over the *edited* book as the alternative's
evidence, which measures what proof catches after four stages have already run
and is the wrong observation.

**Attribution is mechanical and judgement is separated from it.** A stage owns
exactly its commit's diff — that is what `pilot-measure-k26`'s one-commit-per-
stage boundary already buys — so no judgement is needed to say *which stage made
a change*. Judgement enters only at *is this change a reader-facing improvement*,
and it is bounded by the taxonomy's observable tests and by a required citation
into the before-state. Keeping the two apart is what stops the measurement
becoming a single act of taste.

**The unit of analysis is a claim: one taxonomy class at one anchored subject.**
A diff hunk is the wrong unit — a copy edit fixing one terminology drift in forty
places would outscore a developmental edit that fixed four structural defects,
and the decision would be made by hunk arithmetic. A claim collapses a homogeneous
family to one entry, so a stage's count measures distinct defects closed rather
than keystrokes.

**Cost is treated as uniform across stages and the decision turns on marginal
benefit alone.** What a `keep` buys is one `grove-<kind>` skill and costs one
session per book, and that cost is the same for every stage, so ranking by
benefit ranks by ratio. This is an assumption, it is stated rather than hidden,
and it is what makes a count of marginal claims a sufficient decision quantity.

**`Undetermined` is a first-class verdict and the headline number is a range.**
A stage whose record is missing or invalid must not read as dropped (absence of
evidence) or as kept (silent survival), so the surviving-pipeline size is
reported as `[N_min, N_max]`, where `N_min` counts established keeps and `N_max`
additionally counts undetermined stages. That is the partial-sample arithmetic
`docs/evaluations/writing-code-walkthroughs/` had to state after the fact, stated
in advance here.

**Proof is the comparator and cannot be tested by the comparison it defines.**
It is in both arms by construction, so no marginality is computable for it. Its
contribution is reported descriptively, and the report must say plainly that
`pipeline-kinds-k27` extracting proof rests on it being the stated fallback
rather than on measured evidence. Rejected: silently scoring proof against the
probe, which is the probe's own source and would score it against itself.

**No ADR is earned, and the preregistration is the durable record.** The
decisions above are hard to reverse and each has a rejected alternative, but they
are decisions about *this instrument*, and the instrument is itself a durable
artifact under `docs/` that outlives the grove. An ADR restating them would be a
second statement of the same content, which `ADR-FORMAT.md` warns will disagree
with the first — after which neither binds. The one candidate that generalises
past the pilot, *stages are extracted only against a preregistered measure*, is
already the root brief's position and is stated in the document's own
*Purpose and freeze boundary*.

**`review-design` is cut, and `leaf-insert`ed at `jj-workspace-book-k25` rather
than appended.** The producer allowance is spent to none by cutting it
(`references/execute.md`: a producer with a scheduled review spends none), which
is also why no in-session reviewer was materialised here. Insertion rather than
appending is what the task file required and the reason is structural: appended,
the review would run after the drafting and the measurement it exists to
constrain, making it a report on the experiment. The leaf's body carries the six
specific doubts the producing session cannot press on itself, and tells it to
place any integration the same way.

**The stale position-prefixed cross-reference the insert reported is left
unedited.** `leaf-insert` flagged
`.grove/07-walkthrough-machinery-k10/03-DONE-review-design--walkthrough-books-spec-k46.md:161`,
which cites `03-impl--jj-workspace-book-k25` — now `04-`. It is one of five
`path:line` citations in that paragraph, and the other four are equally
historical: the spec they point into has since been rewritten by
`walkthrough-books-spec-k20`. Correcting only the one the verb can detect would
make a wholly historical paragraph read as current while staying stale, which is
worse than leaving it consistently a record of what a retired review saw.

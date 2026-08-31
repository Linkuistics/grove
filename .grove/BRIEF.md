# grove.add-user-guide-and-code-walkthoughs-for-all-crates — brief

## Goal

One user guide for the `grove` binary, and complete source-exact code
walkthroughs for every crate — authored through a publishing-house pipeline
(draft, edit, art, proof) that grove does not yet have, driven by an explicit
loop, and directed by human-authored structure briefs the source code cannot
supply.

Four products live here. Only the first is what the grove was named for, and
the rest exist because it cannot be reached without them.

- **P1 · The documentation** — the user guide, a system overview, and four
  crate books over a frozen 33-root, 14,525-line corpus.
- **P2 · The publishing pipeline** — new session kinds and their skills.
- **P3 · The loop construct** — iteration modelled in grove and legible in the
  filesystem view.
- **P4 · Specification capture** — how human intent is captured in a form
  stakeholders can share and an LLM can consume. Research, not delivery.

## Done when

- `docs/walkthroughs/` holds a system overview and one book per crate for
  `grove-llm`, `jj-workspace`, `keyed-launch` and `grove-loop`, plus the
  relocated `ordinal-fs-tree` book, each proving complete reconstruction of its
  source roots under the fragment validator.
- `docs/USAGE.md` is a complete user guide for the `grove` binary and still
  owns its row in the documentation-ownership table.
- `docs/ARCHITECTURE.md` carries decisions, constraints and measurement records
  only; its descriptive account of runtime flow, command surfaces and module
  seams lives in the overview, with every citation of a moved anchor re-pointed
  and the link-integrity suite green.
- The fragment validator takes a per-book corpus rather than one compiled-in
  ledger, and `scripts/check.sh` gates every book.
- The publishing pipeline exists as installed `grove-<kind>` skills, derived
  from what the pilot measured rather than asserted in advance.
- The loop construct has a filesystem representation a human can read without
  grove installed.

## Decomposition

Three arms, ordered so the documentation is never blocked on unbuilt machinery.

- **`plan-k1`** — this session. Requirements. Retired.
- **`plan-k2`** — an adversarial read of this brief before anything is built on
  it. Seventeen decisions, four of them reversed mid-interview, and a long
  chain of sessions will execute them with no human present.
- **`walkthroughs-k3`** — planning for P1 and P2: the validator generalisation,
  the relocation, the spec split, the pilot cycle, and the scale-out. **It must
  place its children with `leaf-insert` ahead of the research pair**, or the
  documentation arm queues behind a deep survey.
- **`specification-capture-k4/k5/k6`** — P4. A vendor pair, because the
  question is load-bearing enough to pay for two corpora and one survey's blind
  spots would decide it.
- **`loop-construct-k7`** — P3. Cut after the pilot has run editorial cycles by
  hand, so the construct is designed against a loop that actually ran.

## Pointers

- **Method**: the `linkuistics:writing-code-walkthroughs` skill is the
  authoring method; its eight-field intake is answered in `plan-k1`'s decision
  log and must not be re-elicited.
- **Precedent**: `docs/ordinal-fs-tree/book/` is the worked example every new
  book is uniform with, and `docs/specs/ordinal-fs-tree-book.md` is the
  contract it was built to.
- **Ownership**: `docs/ARCHITECTURE.md`, *Documentation ownership* — one
  canonical source per subject, and the rule bounding what may sit directly
  under `docs/`.
- **Contexts**: `CONTEXT-MAP.md` argues that `grove`, `grove-llm` and
  `grove-loop` **are** the grove context and that `jj-workspace` and
  `keyed-launch` are deliberately not contexts. A per-crate directory under
  `docs/` would assert boundaries that document denies; `docs/walkthroughs/` is
  chosen to assert none.
- **Kinds**: `docs/adr/a-kind-is-an-open-token.md` — a kind exists iff a
  `grove-<kind>` skill exists, and every new kind needs a launch template in
  the human's personal configuration before it can run.
- **Reopened by P3**: `one task is one session`,
  `docs/adr/entries-are-never-removed.md`, and the outcome-partition records.
  The loop construct cannot be built without reopening them.
- **In-house prior art**, all on disk and all live: `Writegood` (an
  optimisation-loop grove with a judging protocol and a measurement corpus),
  `grove.gh-issue-12` (the preregistered evaluation behind
  `docs/evaluations/writing-code-walkthroughs/`), and `TheGreatExplainer`
  (whose requirements §1.6 specifies exactly this pipeline, with feedback edges
  and human gates).
- **Test seams**: `every_repository_markdown_reference_resolves` sweeps the
  whole repository and covers new books for free; the architecture anchor suite
  and the `include_str!` content assertions both go red on the ARCHITECTURE
  move, by design; `book-check` proves structure and reconstruction;
  `scripts/check.sh` is the umbrella.

## On the horizon

- **A rendering surface for a grove and its loops** — a verb, or the DevTUI
  direction. Deliberately not now: the filesystem representation is the minimal
  UI and must be settled first.
- **Relocating the fragment validator to the `writing-code-walkthroughs`
  skill**, so it works over any repository. Blocked on a question with no
  precedent here — `plugins/` ships shell scripts and has never shipped a
  compiled artifact.
- **Whether `CONTEXT.md` is two artifacts fused** — a shareable ubiquitous
  language and a maintainer's record of retired mechanisms — and whether the
  shareable one can be projected out rather than maintained twice. Stated
  precisely enough to leaf once `specification-capture` reports.

## Notes

**The corpus is frozen and no session fixes code inline.** A defect found while
documenting becomes its own leaf. This is not tidiness: the fragment ledger
holds an exact line count per source root, so an inline fix shifts every line
below it and silently breaks a page a finished session already proved.

**The pilot's job is to measure, not merely to run.** `Writegood`'s own
`optimisation-loop` record argues that expensive machinery ordered ahead of the
measurement that would justify it cannot answer whether it was needed. Nothing
in this brief can currently tell whether six editorial stages beat two, because
`book-check` proves structure and reconstruction and never whether a page
explains anything. Producing that measure is the pilot's deliverable, alongside
its book.

**The source does not contain enough to structure a book.** Audience,
conceptual order and what deserves emphasis are nowhere in the code. Each book
therefore takes a human-authored structure brief as an input artifact. That is
the manual form of P4, and it is available today — the interview in `plan-k1`
was one.

**Formal specifications are poor human artifacts and good LLM artifacts.** The
capture layer stakeholders share and the formal tier a coding phase consumes
are two layers, not one notation. A format that tries to be both fails at both.
This is the constraint `specification-capture` is commissioned against.

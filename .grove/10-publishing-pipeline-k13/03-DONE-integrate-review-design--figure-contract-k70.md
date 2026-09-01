# figure-contract-k70

**Integrates:** figure-contract-k69

## Goal

Triage the design review of `figure-contract-k18` and apply every finding that
survives scrutiny before the publishing-pipeline kinds are designed against the
figure contract.

## Context

- Read `figure-contract-k69` from its committed review task; its findings and
  citations are the handoff, and are not restated here.
- The reviewed artifacts are `docs/adr/a-book-carries-no-asset.md` and the
  figure/asset clauses in `docs/specs/walkthrough-books.md`.
- `pipeline-kinds-k27` and `pipeline-skills-k28` are direct consumers of the
  settled contract and now follow this leaf in the walk.

## Done when

- Every review finding is classified against the current artifact as accepted,
  rejected, or already resolved, with the reasoning recorded in this task's
  running decision log.
- Every accepted finding is integrated into the minimum coherent ADR/spec set
  and any directly affected book artifact, without rewriting frozen measurement
  evidence to make the decision easier.
- The resulting contract is precise enough for the art kind to consume without
  inventing its own figure class or asset policy.
- `bash scripts/check.sh` passes.

## Notes

This leaf was inserted at the first live sibling after the review so no consumer
can silently move its path-and-line citations before triage.

## Decision log

All four findings reproduce against the current artifacts. **None of them
touches the decision** — no asset machinery, no figure format, no validator
concept of an asset — and the decision stands unchanged. Two of them (`F1`,
`F2`) attack the *rationale* holding it up, which under `ADR-FORMAT` is a rework
of the record in place, not a reversal: the next session reads the reason, not
the vote.

### F1 · Accepted — the inventory-completeness cost was false

Verified against the implementation rather than the ADR's account of it.
`crates/book-validation/src/markdown.rs` builds its expected set from the
manifest's authored `[[page]]` list plus `walkthrough.toml`, then reports both
`expected − actual` (missing) and `actual − expected` (outside the inventory).
Pages are already an author declaration, so an `[[asset]]` group added to that
expected set would preserve the closed world exactly: undeclared files still
extra, declared-but-missing still missing. The ADR called this the cost that
decides the trade-off, and it was not a cost at all.

Rewrote the paragraph. The record now states plainly that admitting assets would
*not* weaken `M101`, cites the validator, and rests the cost where it actually
lies — interface surface paid by every book and every check: a manifest group and
schema bump, a non-text snapshot input class, an `M201` target-set member, at
least three new diagnostic codes, and a second kind of byte inside a book. Also
corrected the downstream sentence that made the loud final-mode failure "the
price of an inventory check that is complete by construction", which inherited
the same false premise.

### F2 · Accepted, with one correction to the finding

The core is right and reproduces. `A1` fires only where *a Markdown table, list
figure or diagram would carry* the relation
(`docs/evaluations/editorial-pipeline-pilot/preregistration.md`), `A2` reaches
figures that already exist, and the art task was bounded to what plain Markdown
can carry. A ceiling-bound relation is outside both passes and outside the
production boundary. The recorded forbidden case is an in-taxonomy `A2` blocked
at the fix boundary (`stages/4-art.md`) — evidence that the surface records a
blocked case, not that an out-of-taxonomy one would have surfaced.

**The finding overstates one point.** It says the preregistration "creates no
search obligation beyond the taxonomy". Its *What this document does not settle*
section does carry one: a stage that **meets** a reader-facing defect neither
frozen standard names records it in `## Findings not fixed` with the class it
would need. That is a duty to record what is met, not a duty to look — so it
strengthens the absence slightly without making it a positive control. Recorded
the obligation explicitly rather than adopting the finding's stronger wording.

Rewrote the paragraph to the weaker result the pilot actually supports: **no
ceiling-bound demand was observed by a Markdown-bounded, single-book pass**, with
the weight moved onto the reversibility asymmetry and the recordable reopening
trigger. Softened the two later sentences that traded on "never been observed
once", and the specification's *Out of scope* paragraph, which asserted the same
observation without its limit.

### F3 · Accepted — the fifth table is not exempt

Confirmed exactly. The specification requires a fifth `source-index.md` table,
the owned-source totals (*Source and ownership ledger*), and
`crates/book-validation/src/ledger.rs` requests only the four fixed headings, so
nothing reconciles the totals and `F009`'s no-lead-in rule does not reach it.
`docs/walkthroughs/jj-workspace/source-index.md` carried it heading-then-header
with no role statement, which the new convention makes an editorial finding at
the moment of adoption.

Decided against exemption, and said so in the contract: the carve-out is those
four tables and nothing else, because the four are machine-reconciled derived
indexes and the totals table is editorial. Added the clause to *Figures*, a
pointer where the ledger section describes the table, and the role statement to
the `jj-workspace` book.

**And triaging it surfaced a separate defect.**
`docs/walkthroughs/ordinal-fs-tree/source-index.md` carries no totals table at
all — its headings stop at `## Early uses` — so one of the two books diverges
from a required clause that no check enforces. That is a book defect rather than
a figure-contract question, so it is externalised as
`ordinal-totals-table-k71` at the end of `crate-books-k14` beside the other
measured defect leaves, not absorbed here.

### F4 · Accepted — the convention had narrowed the measured class

Confirmed against the frozen instrument. `A2` reaches every table, diagram and
non-fragment code block, and the art stage operationalised it over every table,
every diagram and all 45 non-fragment fenced blocks — 39 `text`, 3 `console` —
of the pilot book (`stages/4-art.md`). The new definition made a figure a block
"that carries a relation rather than continuous prose", which silently dropped
console and plain-text output out of the role obligation the kept stage was held
to — contradicting the ADR's own reason for writing the convention down.

Widened the definition to every table, diagram and non-fragment fenced block of
any language, whether or not it carries a relation, and said in the contract why
the class is that wide. The pilot book already satisfies the wider rule — the
`A2` survey found 44 of 45 blocks conformant and claim 7 closed the last — so
adoption costs no rework.

### What was not done

No new review chain. Every accepted finding was rationale repair or a one-clause
contract correction; none demanded the artifact be rethought, which is the test
this kind may not decide alone. The frozen measurement evidence —
`preregistration.md`, `stages/4-art.md`, the pilot `README.md` — was read and not
edited.

`bash scripts/check.sh` passes: 8 principal checks, both books valid under
`book-check --final`.

# grove-llm-brief-early-use-drift-k218

## Goal

Reconcile `docs/specs/grove-llm-book-structure.md`'s early-use table with
`docs/walkthroughs/grove-llm/walkthrough.toml`'s `[[early-use]]` entries, which
agree on all fourteen rows' symbols, first-use anchors and owners but disagree on
**two statements** — deciding, for each, which side is wrong rather than making one
match the other by reflex.

## Context

Surfaced by `jj-workspace-brief-early-use-drift-k215`, which closed the same defect
class in the `jj-workspace` book and then swept the other five books for it. That
sweep is the reason to trust the scope: **only these two rows differ, and only in
the statement column.**

| Symbol | The brief says | `walkthrough.toml` says |
| --- | --- | --- |
| `Workspace` | "every verb resolves it once from the current directory" | "every verb **but `complete`** resolves it from the current directory" |
| `SessionEpochGuard` | "alive through the verb, and consulted only by `complete`" | "…**present under a driver, absent for a manual command** — alive through…" |

Both differences are the manifest carrying a **qualification the brief lacks**,
which is the same shape `k215` found: a later correction landing on the machine-
readable side and never propagating back to the human contract. That is a
hypothesis about provenance, not a finding — **check it before relying on it**, and
note that `k215`'s cause (a pipeline stage's corrections never reaching the brief)
was recovered from `docs/evaluations/editorial-pipeline-pilot/`, which has no
authority over this book.

**The manifest is not automatically right.** `crates/book-validation/src/ledger.rs`'s
`check_early_uses` renders each entry into a required `source-index.md` row and
fails if the row is missing, so an entry can be self-consistently wrong across
manifest and rendered ledger together while the brief holds the correct human
contract. Nothing mechanical reads the brief at all — verified at `k215` by
grepping `crates/` and `scripts/` for `jj-workspace-book-structure`, which returns
nothing while `walkthrough-books` returns five files.

**`Workspace` is the live question and it is a claim about the code**, not about
the documents: whether `complete` resolves a workspace from the current directory
is decidable from `crates/grove-llm/src/`. Note that a nearby claim of this shape
has already been wrong once — a handler taking no readable/writable tree proves
nothing, because `finish-commit` opens it exclusively one layer down. Re-derive it
from the call graph rather than from either document or from a handler signature.

## Done when

- Each of the two rows is decided on its evidence, with the losing side changed and
  the reason recorded — not resolved by declaring one document canonical.
- The two tables agree row for row, and `source-index.md`'s rendered early-use
  table still agrees with the manifest byte for byte.
- `bash scripts/check.sh` passes.

## Notes

**Nothing here is a source change**, so no fragment range moves and the freeze
holds. If the `Workspace` question turns out to require a *code* change, it does
not land here: it becomes its own leaf under the root brief's rule, and that leaf
carries the source change, every affected ledger and page, and a green validator
run over every book it touches — or it is deferred behind them.

**`check.sh` is green today and will stay green whatever you decide.** The
validator compares the manifest against the rendered ledger and never reads the
brief, so this disagreement is invisible to it. Re-derive both tables by parsing
them; do not read them by eye — and normalise the brief's backticked cells against
the manifest's bare values before comparing, or every row reads as a difference.

**`overview` is deliberately not in scope.** Its brief states first use and owner
in chapter terms — first use *ch. 2, `cli.rs` line 2*, owner *ch. 3* — where its manifest uses
`page#anchor` and slice, so all five of its rows differ notationally with no
factual disagreement. `keyed-launch` and `grove-loop` have no early-use table in
their briefs at all. Whether either of those is itself a defect is a separate
question this leaf does not answer.

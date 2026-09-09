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

## Decisions (running log)

**Scope confirmed by parsing, not by eye.** Both tables were extracted
programmatically — fourteen rows in the brief, fourteen `[[early-use]]` entries in
the manifest — and compared with the brief's backticked cells normalised against
the manifest's bare values. The symbol sets are equal, the first-use anchors and
owners agree on all fourteen, and exactly two `statement` cells differ:
`SessionEpochGuard` and `Workspace`. `k215`'s sweep is confirmed rather than
assumed. The comparator was then re-run after the fix alongside a control that
perturbs one manifest cell: it comes back clean on the real tables and dirty on
the perturbed one, so the clean read is evidence rather than a broken instrument.

**`Workspace`: the manifest is right and the brief is wrong, decided from
`crates/grove-llm/src/cli.rs` and not from either document.** Eleven of the twelve
handlers call `worktree()` (`cli.rs:866`), which is `Workspace::resolve(&cwd)`;
`cmd_complete` (`cli.rs:459`) is the only one that does not, and it is the only
handler with no `worktree()` call anywhere in its body. The warning about handler
signatures proving nothing was taken seriously and the claim re-derived from the
call graph rather than the signature: `run` (`cli.rs:421`) calls
`grove_loop::admit_ambient_session(&cwd, …)` for *every* verb before dispatch, and
that path does reach `Workspace::resolve(path)` — but only at
`driver_lease.rs:751`, after an early `return Ok(None)` at `:748` when no signal
path is present. So under a driver `complete`'s working tree *is* resolved once,
one layer down in admission; what `complete` never does is resolve one of its own.
The brief's "every verb resolves it **once**" is false in both directions: false
for `complete`, which resolves none of its own, and false for the other eleven,
which under a driver resolve twice — once in admission and once in `worktree()`
— a fact `02-the-grammar.md:204` already states. `finish-commit` resolves a third
time, at `cli.rs:452`, which `06-leaving-the-loop.md:433` calls "the second
resolution of the working tree in one command". The manifest's wording drops
`once` and excepts `complete`, and both corrections are what the source supports.

**`SessionEpochGuard`: the manifest is right and the brief is wrong, for the same
reason in a different key.** `admit_ambient_session` returns
`Result<Option<SessionEpochGuard>>` (`driver_lease.rs:716-721`), and the `Option`
is not incidental: `ambient_signal_path()` reads `GROVE_SIGNAL_FILE`,
`signal_path_from` treats empty as *none* rather than a degenerate path, and
`admit_session` returns `Ok(None)` before touching a workspace or an epoch file.
The brief's "the live session epoch" presupposes an epoch that the type makes
optional; the manifest's "a live session epoch — present under a driver, absent
for a manual command" is exactly the disjunction the code has.
"Manual command" is not loose here, because the book defines it:
`02-the-grammar.md:31-32` — *a verb run without it is a manual command, and no
epoch governs it* — and `:440-441` states the same `Some`/`None` split in the same
words. The manifest statement is in the book's own vocabulary.

**Neither row was resolved by declaring a document canonical.** Both were decided
against `crates/grove-llm/src/` and `crates/grove-loop/src/driver_lease.rs`, and
both happened to fall to the manifest. The provenance hypothesis in *Context* was
checked and holds in shape but not in detail: the manifest's `[[early-use]]`
entries have exactly one revision in the log, `orientation-k92`, which is *after*
`grove-llm-structure-k32` wrote the brief — so the qualifications were added by the
drafter reading the source, and never propagated back. That is a later correction
landing on the machine-readable side, as `k215` found, but it landed at draft time
rather than at a later pipeline stage. It is corroboration, and it decided nothing.

**Consequently only the brief changed.** The manifest is untouched, so
`source-index.md`'s rendered early-use ledger and `01-orientation.md`'s import
table — which carry the manifest's wording verbatim at `source-index.md:195-196`
and `01-orientation.md:470-471` — needed no edit and still agree with it byte for
byte. Had the brief won, four surfaces would have moved instead of one; that
asymmetry is a consequence of the verdict and was not an input to it.

**One nearby instance of the same clause was considered and deliberately left.**
The brief's chapter-2 responsibility list says *every verb, `complete` included, is
admitted against the live session epoch* (`grove-llm-book-structure.md:210-211`),
which carries the same unqualified "the live session epoch". Its load-bearing
claim — that admission precedes dispatch with no verb exempt — is true, and the
book's own prose opens the chapter with the identical unqualified phrasing
(`02-the-grammar.md:10`) before qualifying it twenty lines later. An early-use row
is a *minimum local statement* that stands alone in a ledger with nothing around
it to qualify it, which is why the omission is a defect there and a lead sentence
here; changing the brief's line would put it out of step with the page it
describes. Recorded rather than fixed.

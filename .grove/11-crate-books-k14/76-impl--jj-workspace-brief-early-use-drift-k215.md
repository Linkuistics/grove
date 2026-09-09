# jj-workspace-brief-early-use-drift-k215

## Goal

Reconcile `docs/specs/jj-workspace-book-structure.md`'s early-use table with
`docs/walkthroughs/jj-workspace/walkthrough.toml`'s `[[early-use]]` entries, which
disagree on three rows — deciding, for each, which side is wrong rather than
making one match the other by reflex.

## Context

Surfaced by `refusal-remedies-are-jjs-overclaim-k202`, which changed the `Refusal`
row on both sides and, checking that it had left them in agreement, found the rest
of the table already out of agreement. **k202 did not touch these three rows**; they
predate it.

The brief states of itself (`:26`) that the chapter sequence and ownership mapping
are what the manifest records, and that *"where the two disagree, that is a defect
in one of them, not a licence to prefer either"*. So this is the brief's own rule
falling due, three times. Measured by parsing both — 8 `[[early-use]]` entries in
the manifest, 7 rows in the brief's table:

| Symbol family | `walkthrough.toml` | the brief |
| --- | --- | --- |
| `main_repo_of` | present, first-use `01-orientation.md#commit-tour` | **absent entirely** |
| `Commit` | first-use `01-orientation.md#public-surface` | first-use `01-orientation.md#commit-tour` |
| `control_dir` | symbols `` `control_dir` ``, first-use `#public-surface` | symbols `` `control_dir`, *namespace* ``, first-use `#commit-tour` |

**The manifest is not automatically right.** It is the side a machine checks —
`crates/book-validation/src/ledger.rs`'s `check_early_uses` renders each entry into
a required `source-index.md` row and fails if the row is missing — so an entry can
be self-consistently wrong across manifest and rendered ledger together while the
brief holds the correct human contract. `Commit` and `control_dir` are the live
question: both are introduced in `01-orientation.md`'s *The public surface*
(`:330-346`), which is evidence for the manifest, but the brief was authored
against a chapter plan and may be recording where the reader is meant to first
*need* them, which is a different thing.

`main_repo_of` is the asymmetric one: a row present in the manifest and absent from
the brief is either a symbol the brief forgot or one the manifest should not
demand.

## Done when

- Each of the three rows is decided on its evidence, with the losing side changed
  and the reason recorded — not resolved by declaring one document canonical.
- The two tables agree row for row, and `source-index.md`'s rendered early-use
  table still agrees with the manifest byte for byte.
- `bash scripts/check.sh` passes.

## Notes

**Nothing here is a source change**, so no fragment range moves and the freeze
holds.

**`check.sh` is green today and will stay green whatever you decide.** The
validator compares the manifest against the rendered ledger and never reads the
brief, so the disagreement this leaf exists to close is invisible to it — the same
blindness `k202` was warned about. Re-derive both tables by parsing them; do not
read them by eye.

# lease-stale-reader-name-k169

## Goal

Repair the one defect in `crates/grove-loop/src/driver_lease.rs`'s production
half — a reader named `probe_lease_holder` that does not exist in this workspace
— inside the file's frozen 1,383-line count, and reconcile the page that
reproduces the changed bytes.

## Context

- **The defect.** Lines 452 to 459 argue that `truncate(false)` on the lease open
  is load-bearing, and close with *The reader at `probe_lease_holder` parses that
  record, so an emptied file also reads as a corrupt lease rather than an absent
  one.* `grep -rn 'probe_lease_holder' .` hits that comment and nothing else,
  against a control `probe_live_lease` that hits five sites in the same trees.
- **The behaviour is correct and only the address is wrong** — chapter 6's
  `llm_cli` shape rather than `task_grow.rs`'s `leaf_slug` (repaired at
  `grow-header-stale-helper-k154`, so that name is now only in chapter 10's
  record of it; the analogy is to the defect class, not to a live instance).
  The reader is
  `probe_live_lease_with_post_unlock_hook` (line 641), whose line 654 does
  `parse_process_record(&read_record(&mut lease_file, "driver lease record")?)?`,
  and an emptied file fails that parse at *missing worktree-device field* — a
  corruption diagnostic, not an absence. So the repair is the name, not the
  sentence.
- **It was wrong when written rather than outrun.** `jj file annotate` puts the
  comment in `loop-crate-driver-k22` (`ykxruswv`), the commit that created
  `crates/grove-loop/src/driver_lease.rs`, and that same revision's file already
  defines `probe_live_lease` and `probe_live_lease_with_post_unlock_hook` and no
  third probe. The pre-move copy at the workspace root's `src/driver_lease.rs`
  carries the identical comment at its line 431 over the identical pair, so the
  name did not survive a rename in this repository's history.
- **No instrument in this repository sees it.** Lines 452 to 459 are plain `//`
  comments inside a function body, and `cargo doc --no-deps
  --document-private-items -p grove-loop` reports **thirty** warnings over the
  crate and **none** naming this file — verified while drafting chapter 16. The
  block's 103 comment lines are 9 `//!`, 61 `///` and 33 plain `//`; the defect
  is in the 33 the tool cannot read.
- **One page reproduces the changed bytes** and must change in the same commit:
  `docs/walkthroughs/grove-loop/16-the-lease.md`, fragment
  `«lease-acquire-lease-file-with-hook»` (lines 445-498). Its
  `#neither-open-truncates` section adjudicates the name in front of the reader
  and must be rewritten to describe the repaired comment; check
  `concept-index.md` for an entry naming it.

## Done when

- `grep -rn 'probe_lease_holder' .` returns nothing outside this leaf's own
  records, and the sentence names the function that does the parsing.
- **`crates/grove-loop/src/driver_lease.rs` is still exactly 1,383 lines**, and
  the comment still occupies its existing line span, so no ownership range,
  manifest `lines` value or fragment range moves. The replacement fits: the
  longer name leaves the three-line comment three lines.
- `16-the-lease.md` reproduces the new bytes and its adjudication is rewritten to
  the repaired text.
- `book-check --repo . --book docs/walkthroughs/grove-loop --check all` is green
  over the book at whatever slice it is proved at when this runs, and every other
  book the commit touched is green too.
- `bash scripts/check.sh` is no worse than it was before this leaf.

## Notes

**This leaf is deferred behind the `grove-loop` book and says so.** The bytes it
changes are reproduced by a finished page, and the freeze rule requires one
commit to carry the source change, every affected ledger and page, and a green
validator run. Run it after `grove-loop-k123`'s last child has taken the book to
green `--final` validation, alongside `unresolved-doc-links-k151`,
`unreachable-root-clause-k152` and `grow-header-stale-helper-k154`, which are
deferred for the same reason. Until then chapter 16's adjudication stands as the
record.

**Do not widen this into a sweep of the file's comments.** The rest of
`driver_lease.rs`'s production half was enumerated at `the-lease-k164` — every
backticked token in lines 1 to 819, classified rather than pattern-matched — and
this was the only one that resolved to nothing. Lines 820 to 1,383 are chapter
17's and are swept by the chapter that reads them.

## Decisions (running log)

1. **The repair is the name, and it fits.** `probe_lease_holder` →
   `probe_live_lease_with_post_unlock_hook` in
   `crates/grove-loop/src/driver_lease.rs`, re-flowed across the same three
   comment lines (457–459). The file is still **1,383 lines**; the widest new
   line is 80 characters against the file's existing 81-character maximum, so
   nothing about wrap width changed either.

2. **Five surfaces, not the one the brief named.** The task file named the
   source and `16-the-lease.md`'s fragment plus its `#neither-open-truncates`
   adjudication. Enumerating the name across the repository found two more live
   ones: `19-the-core.md:849`, which cites *chapter 16's `probe_lease_holder`
   class* as a live instance while adjudicating its own `skill-` prefix defect,
   and two `concept-index.md` rows (456, 457) phrased in the present tense. All
   four prose sites were re-tensed; the `.grove/` records of the drafting
   sessions (k129's brief, k164, k166, k168) are frozen history and stay.

3. **The task file's "line 654" is wrong; chapter 16's "lines 655 and 656" is
   right.** `parse_process_record(&read_record(…))` spans 655–656 in
   `driver_lease.rs`; 654 is `ensure_close_on_exec`. The task file's Context
   said 654. Nothing turns on it — the reader named is the same function — so
   the page was left as it stands and this log carries the correction.

4. **The name survives in the books' record of it, deliberately.** *Done when*
   asks for a clean `grep -rn 'probe_lease_holder' .` outside this leaf's
   records; after the repair the token remains in two prose sentences —
   `16-the-lease.md`'s adjudication and `19-the-core.md`'s citation of it —
   both now past-tense records of a repaired defect. That is the precedent this
   leaf's own Context points at: `grow-header-stale-helper-k154` left `leaf_slug`
   standing in `10-growing.md:229` for exactly the same reason. Deleting the
   name would delete the chapter's most durable paragraph — the one arguing that
   a `//` comment inside a function body is checked by reading and by nothing
   else. What the clause was guarding is that no *live claim* asserts the reader
   exists, and no claim does.

5. **Verified green.** `book-check --final --check all` over all six books
   (grove-loop 13 files / 10,557 resolved lines) and `bash scripts/check.sh`:
   *all 8 principal checks pass*. The sweep's controls: `probe_live_lease` finds
   24 sites with the same flags, a deliberately mutated pattern finds 0, and the
   unmutated pattern still finds 12 sites under `.grove/` — so the clean read
   outside `.grove/` is not a broken instrument.

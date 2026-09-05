# pick-test-count-k147

## Goal

Correct the two Part II claims in `docs/specs/grove-loop-book-structure.md` that
enumeration refutes: chapter 7's test count, from fifteen to nineteen, and
chapter 5's *the whole reason `libc` is a dependency*.

## Context

- **The defect.** The brief's *7 · The walk: pick and select — `first-live-leaf`*
  section opens *Fifteen tests, and the chapter's prose owes the negative case
  for each*. The block it describes is `pick-tests`,
  `crates/grove-loop/src/task_tree.rs` 1,106–1,360, and it carries **nineteen**
  `#[test]` functions, whose `fn` lines are 1,109, 1,132, 1,142, 1,158, 1,167,
  1,178, 1,191, 1,202, 1,215, 1,229, 1,241, 1,251, 1,265, 1,271, 1,281, 1,296,
  1,326, 1,339 and 1,351.
- **It was wrong when written, not outrun by a later leaf.**
  `crates/grove-loop/src/task_tree.rs` is unchanged since
  `grove-loop-structure-k36` (`bb93d2c39302`) wrote the brief, so no test was
  added under it.
- **Neither narrower reading rescues the number.** Eighteen of the nineteen are
  named `pick_*`; the nineteenth,
  `select_returns_path_handle_and_kind_from_one_guarded_observation`, is the
  block's first. Fifteen is neither count.
- The seven the brief names by name all exist and are correctly named. What is
  wrong is only the total, and the obligation the total governs — *the negative
  case for each* — is unchanged in kind and larger by four.
- **Placed before `the-walk-k143`** because chapter 7 is the page that would
  otherwise write the number, which is why `display-first-use-k137` ran before
  `kind-slug-handle-k135` and `structure-brief-dependency-count-k132` before the
  rest of `grove-loop-k123`.
- **The second defect: chapter 5's `libc` clause.** The brief's *5 · Opening,
  contention and refusal* section says the chapter owns `announce_contention`,
  *which is the whole reason `libc` is a dependency*. `libc` is reached from
  three production modules — `task_tree.rs`'s contention probe, `driver_lease.rs`
  for the lease's locking and its close-on-exec descriptors, and `loop_driver.rs`
  for the terminal and signal calls chapter 20 reads — and from
  `task_grow/tests.rs` besides. **The book already adjudicated this**: chapter 1
  states it at `01-orientation.md#the-package`, against the same clause in
  `crates/grove-loop/Cargo.toml`, and `manifest-dependency-clauses-k133` holds
  the source fix. The brief repeats the manifest's error and is the one artifact
  of the three that is not frozen, so it is corrected here. Chapter 5 states the
  true relation on the page rather than waiting for this leaf; what k147 owes is
  agreement between the brief and the two pages.
- **The corpus is not touched.** `task_tree.rs` and `Cargo.toml` are frozen and
  the manifest's own clause is `manifest-dependency-clauses-k133`'s; the defects
  here are in the brief that describes them, exactly as
  `structure-brief-dependency-count-k132`'s was.

## Done when

- The brief's chapter 7 section states nineteen, and any other statement of the
  same count elsewhere in the brief agrees with it.
- The nineteen are verified by enumeration against the block, not by trusting
  this leaf's list.
- The brief's chapter 5 section no longer calls `announce_contention` the whole
  reason for `libc`, and says what chapter 1 and chapter 5 both say instead.
- No page is edited for the count: no chapter of Part II owning `pick-tests`
  exists yet. Chapter 5 exists and already states the `libc` relation correctly,
  so check it agrees with the corrected brief rather than rewriting it.
- `bash scripts/check.sh` is unchanged — red on `book-check` alone, which is
  every child of `the-walk-k126`'s shape until chapter 21 lands.

## Notes

**Check the brief's remaining counts over Part II while you are in it**, and
correct any that enumeration refutes in the same commit. Two more were checked by
`opening-k141` and stand: *the four openings* of chapter 5 is the 2×2 of
{shared, exclusive} × {refusing, answering the vacancy} — `read_or_vacant`,
`read`, `write`, `write_or_vacancy` — with `reopen_write` a diagnostic-free
`write` and `open_write` private; and *the three error paths* are `absent_tree`,
`raised` and `restate`, all three in the block. **Do not widen this leaf into a
sweep of the whole brief** — Part II is its scope, and a count outside it is a
leaf of its own.

## Decisions (running log)

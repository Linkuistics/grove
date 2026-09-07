# lock-scan-blind-to-contention-probe-k190

## Goal

`no_production_lock_grove_takes_for_itself_ever_blocks`
(`crates/grove-llm/tests/tree_lock.rs`, line 308) names two lockers in its doc
comment and scans only one of them. Make the scan reach both, and correct the
`grove-llm` book sentence that reports the test's coverage as wider than it is.

## Context

- Found by `root-init-drop-order-comment-k99`'s in-session reviewer, while
  checking the prose around the passage that leaf rewrote. The claim is in
  `docs/walkthroughs/grove-llm/04-growing-the-tree.md`, lines 240–243: *grove
  takes no blocking lock of its own, which
  `no_production_lock_grove_takes_for_itself_ever_blocks` … holds by scanning
  the five packages grove ships for a `libc::flock` call and requiring each
  non-blocking.*
- **The instrument is blind to one of the two lockers it names.** The test cuts
  each file's production slice with
  `body.split_once("#[cfg(test)]")` (`tree_lock.rs` line 316). In
  `crates/grove-loop/src/task_tree.rs` the first `#[cfg(test)]` is at **line
  60** — a test-only `thread_local! { READ_COUNT }`, not the file's `mod tests`
  — so the production slice is lines 1–59, and the contention probe's own
  `libc::flock` calls at lines 249 and 250 are never seen. Replicating the scan
  finds four matching lines, **all** in `crates/grove-loop/src/driver_lease.rs`
  (395, 501, 657, 660) and none anywhere else. The test's own doc comment
  (lines 294–299) names *Grove's two remaining lockers* as the lease **and**
  `task_tree`'s contention probe; it checks the first and cannot see the second.
- The `assert!(checked >= 2)` floor does not catch this: the lease alone
  supplies four.
- **A second, narrower defect in the same book sentence.** *requiring each
  non-blocking* is not the predicate the test applies: line 324 passes a line
  containing `LOCK_NB` **or** `LOCK_UN`, and an unlock is not a non-blocking
  acquisition — `driver_lease.rs` line 660 is one of the four matches and is a
  `LOCK_UN`.
- The shape of the cutting bug: a stray `#[cfg(test)]` far above the file's
  real test module silently truncates a whole-file measurement, and the
  truncated read looks exactly like a clean one.

## Done when

- The production slice is anchored on the file's inline test **module** rather
  than the first `#[cfg(test)]` attribute, so `task_tree.rs`'s contention probe
  is scanned; the scan is seen to reach both lockers by counting the matches
  before and after, and the count is recorded.
- The predicate distinguishes an acquisition from a release, so
  *requiring each non-blocking* is what the test actually requires.
- Whatever the widened scan finds is reported: if it now finds a blocking
  acquisition, that is a source finding and gets its own leaf rather than a
  quiet loosening of the test.
- `04-growing-the-tree.md` lines 240–243 describe the scan the test performs.
  `crates/grove-llm/tests/` is evidence rather than a book root, so a test
  change moves no fragment range; the book sentence is the only page surface,
  and it must be re-checked for the wrapped restatement in
  `07-what-order-holds.md` line 125's table and lines 145–149.
- `bash scripts/check.sh` passes, and `book-check --final` is green over
  `grove-llm`.

## Notes

The book sentence and the test are one commit's work: the sentence is only
false because the test is narrower than it claims, and correcting either alone
leaves the pair inconsistent.

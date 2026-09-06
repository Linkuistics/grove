# the-epoch-k165

## Goal

Draft chapter 17 of the `grove-loop` book, *Which calls the lease admits* —
`crates/grove-loop/src/driver_lease.rs` lines 820–1383, the file's whole inline
test module, 564 lines — to a valid slice through `which-calls-are-admitted`.

## Context

- **This is the barest block in the corpus at 3% comment prose, and the whole
  charter is *supply the claim*.** For each of the nine reproduced tests: the
  property it establishes, **and what would have to be true for it to pass while
  that property was broken**. A test name is a label, not an argument, and this
  block gives you almost nothing else.
- The nine the structure brief names:
  `an_alias_equivalent_second_owner_is_refused_immediately`,
  `lease_path_replacement_retries_until_the_locked_descriptor_is_current`,
  `lease_path_replacement_fails_closed_after_eight_attempts`,
  `acquired_driver_descriptors_are_close_on_exec`,
  `activation_and_invalidation_replace_one_stable_epoch_record`,
  `an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls`,
  `manual_agent_operations_need_no_driver_epoch`,
  `an_active_epoch_without_a_live_lease_is_stale`, `a_malformed_epoch_is_stale`.
  **Count them in the block before believing the list** — the brief's
  enumerations have been wrong twice in this book already
  (`pick-test-count-k147`, `structure-brief-chapter-attributions-k163`), and a
  cited test list is a measurement rather than a reading.
- **Chapter 16 owns the production half and the hooks these tests drive.**
  `acquire_with`'s `before_initial_epoch_handoff`, `acquire_epoch_file_with`'s
  `after_open` / `after_lock` / `report_contention` / injected clock and wait, and
  `probe_live_lease_with_post_unlock_hook`'s `after_successful_probe` are all
  read on chapter 16's page as *the internal test seams* the ADR's last paragraph
  reserves. Chapter 17 says what each seam is used **for**, and does not re-read
  the production functions.
- **A test belongs to the block its line number falls in.** The split here is at
  the `#[cfg(test)]` line and not by concept, so a test that exercises
  acquisition still belongs to this chapter and an acquisition helper still
  belongs to chapter 16, whatever the subject says.
- The chapter keeps `one-live-driver-per-working-tree`, named in prose and never
  linked, and declares guide anchor `usage-driver-lease` and glossary anchors
  `driver-lease` and `session-epoch` alongside chapter 16.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  which-calls-are-admitted --check all` is valid at 9,315 resolved lines, 1,218
  deferred, `final=false`.
- The `lease-tests` ownership row reads `resolved`; contents and navigation
  updated; `scripts/check.sh` red on `book-check` alone, and this file says so.

## Notes

**`.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE`, so an admission arm
that reads it is unobservable in process** — a guard, not a gap. Do not write a
coverage sentence about `signal_path_from`'s arms without the distinction; the
node brief carries the k128 measurement that establishes it.

**A mutant naming a `driver_lease.rs` test must be re-run** — this file wedges
under load and a timeout reads exactly like an observer.

## Decisions (running log)

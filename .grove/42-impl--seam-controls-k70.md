# seam-controls-k70

## Goal

Make each command-seam test below observe what its name claims. These are
tests whose bodies hold their clause more narrowly than the spec states it.
No shipped behavior changes.

## Context

`acceptance-walk-k67` mapped every acceptance clause to the assertion that
observes it and found an instrument for each. Its running log has the whole
walk. It also found these narrower holds, where a test could pass without
seeing what it claims, or where the seam leaves a value form to a unit test.
Paths are under `crates/harness-dispatch/tests/`, and line numbers are as of
that walk, so find the assertion by its text.

A control can pass vacuously:

- `hostile::classes_with_no_known_firing_configuration_are_reported_not_counted`.
  Its cwd tsconfig arm asserts the probe's identity and then only
  `still_unfired("cwd tsconfig", aliased.exists())`. A probe that failed
  before its import would pass. The other three arms assert that the probe
  loaded first. This arm expects a refused import, so it needs
  `assert_import_refused` on what the probe reported.
- `worker::a_worker_from_another_build_refuses_before_it_is_given_a_policy`.
  The mismatched workers are shell scripts that write a hello and exit. They
  never read descriptor 3, so nothing shows the front withheld the policy,
  and the test's own sentinel message says a fake cannot have evaluated it.
  `worker::the_request_carries_the_task_inputs_explicit_choice_and_limits_and_never_the_prompt`
  has a fake that records what it is sent; a mismatched hello from such a
  fake, with nothing received, would observe the order.
  `hostile::a_probe_build_is_never_accepted_as_an_installations_worker`
  could do the same with a sentinel policy, since a probe is a real worker.

The seam leaves a stated form to a unit test in `src/observation.rs`:

- `observations::an_observation_round_trips_through_observe_and_show`, through
  `every_measurement`, imports an empty `falseFindings` list, an exit by code
  and never by signal, and a `successProbability` that is uncalibrated and
  never `{ probability, calibration }`.
- `observations::unsupplied_measurements_stay_unobserved_and_an_attempt_stays_unconfirmed`
  checks four of an imported observation's twelve unsupplied measurements by
  name, beside a count of fifteen. The every-measurement loop exists only for
  a run with no observation.

A "launches nothing" case lacks a control on its own fixture:

- `deadline::a_worker_that_ignores_term_is_killed_after_at_most_a_second_of_grace`:
  no run of the TERM-ignoring prelude that ends within its bound and reaches
  the harness.
- `cancellation::a_group_interrupt_reaches_the_policys_own_children_through_the_job`:
  no uninterrupted run of the child-spawning policy that reaches the harness.
- `handoff::a_cancelled_handoff_whose_detail_cannot_be_appended_stays_unknown`:
  no unsignalled run with the append-refusing trigger installed.

Smaller, each one case:

- `bounds::one_read_holds_to_its_limit_whatever_the_policy_does_with_the_error`
  raises `maxBytes` to the budget but reads a 100-byte file. A direct read
  above 64 KiB is seen only through the Grove adapter.
- `bounds::diagnostics_hold_to_their_bound_across_both_streams` asserts the
  kept length over uniform bytes, so "the first 256 KiB" is not told from
  any other.
- `authority::a_missing_unreadable_or_unlocatable_entry_refuses_naming_the_path`
  has a missing personal default and a directory, and no `--config` naming a
  file that does not exist.
- `review::an_explicit_choice_is_held_to_the_same_rule` refuses
  `gateway-careful`, but nothing asserts that its program and model differ
  from the creator's candidate, so it passes if the example loses its
  disguise. No case sets a candidate's argv against its provider label.
- `records::a_lock_held_past_the_wait_launches_nothing_and_spends_no_selection_time`
  holds after the lock is released, never with a lock released inside the
  wait.

Judge each on its merits, and say in the running log which you left and why.
k67 accepted these as trade-offs and cut nothing for them: waiting out the
30-second default and the 120-second ceiling, the loose timing of the grace,
the one-byte context floor, the start directory's mode under another umask,
and refusals driven through `inspect` where `run` evaluates the same
selection.

The spec's sentence that the generic core has no Grove review-kind list or
`Reviews` parser has no test. k67 checked it by search with both controls.
Decide whether a test over `src/` earns its place or whether the claim is
the documentation-acceptance review's to reread.

## Done when

- Each test above observes its clause, or the running log says why it was
  left.
- Each new control was seen to fail against a subject known to be wrong
  before its passing read was credited.
- `task check` passes. The worker's source is untouched, so the installed
  smoke is not rerun.

## Notes

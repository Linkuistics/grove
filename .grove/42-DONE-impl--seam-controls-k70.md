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

## Decisions (running log)

**How each control was seen to fail.** Where the claim is about the front, the
front's source was mutated, the test run, and the file put back with
`jj restore`, so `src/` is byte-identical to its parent. Where the claim is
about a fixture, the fixture was mutated. Each entry below names its wrong
subject and the assertion that fired.

**The mismatched worker records its channel.**
`worker::a_worker_from_another_build_refuses_before_it_is_given_a_policy` now
uses `recording_worker`, a fake that sends its hello and records the frame it
is then sent. The recording is a child of the fake started before the hello,
because the front kills a worker it refuses and would kill the recorder with
it. A refused worker's recording is empty, and the real identity's holds the
`evaluate` frame naming the entry. The policy sentinel is gone: no fake could
have fired it. `the_request_carries_…` uses the same fake in place of its
inline script. Wrong subject: the front with `verify_hello` moved below the
`write_frame` of the evaluate frame. The assertion "the front sent a frame to
a worker it refused" fired with the whole frame, though the front had refused
and killed the worker.

**The probe test claims that no policy ran, not that none was sent.**
`hostile::a_probe_build_is_never_accepted_as_an_installations_worker` has a
policy that marks its evaluation, absent after each probe and present after
the shipped worker. Against the mutation above it passed five times of five:
a probe is killed before it reads the frame. So it cannot see the order, and
its comment says so and points to `worker.rs`. Wrong subject it does see: the
front with `verify_hello` moved below the receive of the worker's answer.
"The autoload probe evaluated the policy before it was refused" fired three
times of three. That is the claim that matters for a probe, which is a build
with a control removed.

**A refused import names its specifier.** The brief asked for
`assert_import_refused` on the cwd tsconfig arm. That helper checked only a
`failure` at stage `load`, which a failed `process.chdir` also is, so it now
requires the failure's message to name the specifier, which it takes beside a
label for the case. Its five other callers, in `a_cwd_package_json_…` and the
two tests of a package above the worker's start directory, pass the specifier
they import and hold to the same.
Wrong subject: the arm's entry moved into a directory that does not exist. It
reported `failure` at `load` for the `chdir`, and "the failure does not name
hostile-alias" fired. The helper as it was would have passed it.

I first wrote that the other callers already passed a specifier, having read
one of the three tests that call it. Two passed a label there, and the first
full `task check` failed on them. The helper took a second parameter, and the
check was run again from the start.

**The round trip imports both forms of each measurement that has two.**
`observations::an_observation_round_trips_through_observe_and_show` runs twice,
each on a run of its own: `every_measurement`, and
`every_measurement_in_its_other_form`, which has an exit by signal, a listed
`falseFindings` beside an empty `missedDefects`, and a calibrated
`successProbability`. The text view prints a structured value as compact JSON,
and the test now reads those three lines in each form. Wrong subjects, each a
mutation of `src/observation.rs`: a signal exit refused, a calibrated estimate
refused, and a listed false finding refused. Each failed the import of
`o-2.json` at its own location. A mutation refusing any listed finding failed
on `o-1`'s `missedDefects` and was discarded: the old body caught that one.

**An imported observation's unsupplied measurements are read by name.**
`observations::unsupplied_measurements_…` checks all thirteen that are not
observed or unknown, the twelve left out and the one given as unobserved, in
the observation and in the summary. The names come from `every_measurement`,
the fixture's own list, so the export is not read against itself. Wrong
subject: `expanded` mutated to report an absent `inputUsage` as an observed
zero, which the acceptance case forbids. The old lists named neither loop's
`inputUsage`, so the old body passed it; the new one failed on that name.

**Each "launches nothing" case runs its own fixture through to the harness.**

- `deadline::a_worker_that_ignores_term_…` ends with the same handler and
  spin, ended within the bound, reaching the harness. The control test's body
  became `assert_reaches_the_harness(place, hold, prelude)`, which both use.
  Wrong subject: the control's prelude also throwing. It failed at the
  helper's exit-status assertion. The timed-out arms ran the real prelude, so
  this shows only that the control fails for a prelude that cannot launch. A
  prelude runs before the spin, and one broken before the hold already fails
  the timed-out arms at "the policy never began evaluating".
- `cancellation::a_group_interrupt_…` has a third run, never signalled, whose
  hold ends after 200 ms: exit 0 and the harness ran. It asserts nothing about
  the child surviving a completed selection, which the spec does not promise.
  Wrong subject: `with_a_child` also exiting the worker 120 ms into its hold.
  Both interrupted arms passed it, and "uninterrupted" failed with
  `worker_failed`, three times of three. That is the case k67 named: a fixture
  the interrupt accepts and nothing could launch.
- `handoff::a_cancelled_handoff_whose_detail_cannot_be_appended_…` first runs
  the same stall unsignalled with the trigger installed: exit 0, the harness
  ran with the notice's run, and the attempt is unknown with no launch
  failure. Wrong subject: a second trigger refusing the insert into `runs`.
  The control failed at `stall`'s own wait, "the front never committed its
  run", after its 30-second watchdog, because the refusal blocks on the
  stalled stderr. The signalled arm would have failed there too, so what the
  mutation shows is that the control can fail, not that it alone would.

**A direct read above 64 KiB.** `bounds::one_read_holds_to_its_limit_…` reads
`past.txt`, the 65537-byte file its default arm refuses, with `maxBytes`
262144, and a file of exactly 262144 bytes; inspection reports each source's
whole size, and each selects. One byte more refuses with the bound from
`maxBytes`. The 100-byte read at that `maxBytes` is gone, since it showed
nothing the two files do not. Wrong subject: the same reads without
`maxBytes`. The first failed with `source_too_large` at the default.

**"The first 256 KiB" is a start, not a length.**
`bounds::diagnostics_hold_to_their_bound_…` prints numbered eight-byte records
on each stream, so no stretch of a stream looks like another. At the bound
each stream is compared whole. Past it, the kept lengths sum to the bound and
each kept stream is the start of what was written to it. Which stream gives up
the byte depends on the order the front's two reader threads ran, so the test
does not say. Wrong subject: `Capture` mutated to keep the end of an
overflowing stream at the same total. The length assertion passed and "what is
kept of stderr is not its start" fired, four times of four. The old body
asserted the length alone.

**A `--config` naming a file that does not exist.**
`authority::a_missing_unreadable_or_unlocatable_entry_…` refuses
`policies/absent.ts` as `policy_missing`, naming the path as the caller's cwd
resolves it, and then inspects the same argument once the file exists. Wrong
subject: the file created first. The refusal's exit assertion failed.

**The gateway's disguise, and the label against the argv.**
`review::an_explicit_choice_is_held_to_the_same_rule` reads `careful` and
`gateway-careful` as the shipped example proposes them for `feature` work: one
provider, and a different program, model and argv. Wrong subject: the second
read naming `careful`. The `assert_ne` on the program failed.
`review::a_candidates_origin_is_its_declared_label_whatever_its_argv` is new
and is the other half. An owner's catalog adds `careful`'s program, model and
arguments under the other origin's label; inspection shows the two proposals
equal but for the provider, the relabelled one reviews the creator's work and
launches with the creator's own argv, and `careful` beside it refuses. It is a
separate test because it needs an owner policy, which the test above does not
use. Wrong subject: the twin left under the creator's label. The review
refused with `same_origin`.

**A lock released inside the wait.**
`records::a_lock_released_within_the_wait_is_waited_out_and_the_run_launches`
is new. The store is locked when the policy marks its run and released 500 ms
later; the front must still be running at the release, and then exits 0 with
the harness run and a second run stored. It is a test of its own so that it
fails by itself. It cannot tell a front that waited from one that reached the
store only after the release, and its comment says so. Wrong subject:
`LOCK_WAIT` set to zero. "The front ended while the store was locked" fired
three times of three, each within the half second, so on this host the front
does reach the store inside it.

**The core's lack of a review-kind list has a behavioral test, and no test
over `src/`.**
`grove::without_the_adapter_a_review_kind_and_its_marker_lines_mean_nothing_to_the_front`
is new. One review task under `review-impl`, declaring its creator `openai`,
is a review under the Grove example: reviewed artifact, creator, adapter and
the task file as a measured source, with a reviewer of the other origin. Under
a routes policy that imports no adapter it is a routed kind. It selects by
route a candidate of the declared origin and launches it, and the report and
the run record hold `reviewedArtifact`, `creator`, `adapter` and `context` as
present and null. Each key is read with `get`, since indexing a key that is
missing also gives null. Wrong subject: the second policy composing the
adapter. It failed at `selectedBy`, the first assertion, so the four null
assertions were not each seen to fail; the control half of the test shows the
same keys filled.

A test that searched `src/` for `Reviews` or `review-` was not written. It
would assert on words, and `--help` legitimately carries `review-` in its
examples, so it would need an allowance list that drifts. That the source has
no such list or parser stays k67's search with both controls, and the
documentation-acceptance review may reread it. What the test adds is the
consequence an owner would see.

**Left as they were.** Nothing in the brief's list was left. The trade-offs
k67 accepted were not reopened.

**No review was cut, and the in-session reviewer was not used.** The leaf
changes tests only, and each new observation has a failing read above, which
is the adversarial check a reviewer would ask for. Nothing here is the
load-bearing artifact a review chain is for. `dispatch-documentation-k71`
reads the test names, and the root brief now tells it which are new.

**The passing run.** `task check` on the finished working copy: all 12
principal checks passed. Each test this leaf touched or added, and each of
the two hostile tests the first run failed, appears as `test <name> ... ok`
once in its log, looked up by name; a name that does not exist is found zero
times. The digest of every file under `crates`, `plugins`, `scripts` and
`docs`, and of the root manifests, was the same before and after the run.
`jj diff` shows nothing under `crates/harness-dispatch/src` or `worker`, so
the installed smoke was not rerun. Before that run, the five binaries with a
new timing-sensitive case, `worker`, `records`, `cancellation`, `bounds` and
`handoff`, each passed ten times of ten.

**No document moved.** The runtime evidence and `scripts/release.test.sh` cite
two of these tests by name, and both names stand. The spec's `#test-seams`
rows state clauses, which did not change.

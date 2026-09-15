# witnessed-activity-k45

**Reviews:** running-lifetimes-k44


## Goal

Adversarially review the complete shipped witnessed-activity protocol and viewer
contract, including resource lifetimes, interleavings, native evidence and docs.



## Context

Re-derive producer commits by permanent handle. The final producer is
running-lifetimes-k44; also consume running-rows-k43, launch-events-k24,
launch-witnesses-k25 (through witnessed-epoch-k33), and witnessed-observation-k26
(through witness-platforms-k37). The parent brief contains acceptance ownership
and native evidence. k37 records platform fingerprints and per-case results;
k38/k40/k41/k42 record release/order mutations, concurrency and foreign holders.

Contract: docs/specs/item-status.md and the one-live-driver-per-working-tree ADR.
Implementation: grove-loop observation/driver_lease, keyed-launch runner events,
jj-workspace discovery and grove-tui observation/Viewer/tests/witnessed.rs.
Usage, architecture, context map, G6 and source-derived books must agree.

## Done when

- Attack Started/Reaped ordering for failed/immediate spawn, signals, escalation,
  confirmed/unconfirmed reap errors, unwind/drop and surviving exec children.
  Check lease-owned paired descriptors, private-before-directory orderly release,
  close-on-exec and admission preservation on observational failure.
- Attack epoch-before-preparation/publication, selected-root pin/recheck,
  independent random names/exclusive creation, binding grammar, exact markers,
  cleanup and no guard across spawn or caller work.
- Attack directory-before-private probe precedence, all release orders and
  numeric/key-reuse mutations, first-arriving replacement observers, shared
  concurrency and old-epoch-delayed handoff. Verify positive controls survive
  mutations; distinguish modeled schedules from kernel-teardown claims.
- Assess macOS/Linux native evidence case by case, including actual reap,
  both independent probes, leftover records and surviving children. Preserve
  native-backend/cooperating-process assumptions and suspension limitations.
- Attack Viewer runtime comparison independently of rejected trees/root errors;
  same-tree key binding/exclusion after whole-tree validation; lifecycle/current
  species, absence, unreadable trees and previous-tree reuse. Check minimum-size
  labels/qualifiers/keys, style separation, hidden/File activity, saved state,
  read-only aliases/multiple viewers and responsive retry.
- Check docs and walkthrough reconstruction against current source. Findings
  name evidence and concrete impact; create adjacent actionable integration only
  if findings earn it. No implementation fixes or in-session reviewers here.

## Notes

witnessed-view-k27 closes with k44. witnessed-activity-k12 remains live for this
review and any actionable integration; implementation tests alone do not close it.

## Decisions (running log)

Producers re-derived by permanent handle from the jj log: launch-events-k24,
selected-root-k28, lease-root-owner-k30, paired-witness-owner-k32,
witnessed-epoch-k33, released-witness-k34, witnessed-binding-k35,
witness-release-orders-k38, witness-concurrent-viewers-k40,
witness-epoch-replacement-k41, witness-foreign-holders-k42,
witness-platforms-k37, running-rows-k43 and running-lifetimes-k44 (commits
4b51b21d..8a3c3ef5). The read covered the shipped source rather than the diffs:
`crates/grove-loop/src/driver_lease.rs`, `driver_lease/witnesses.rs`,
`driver_lease/observation.rs`, `observation.rs`, `loop_driver.rs`,
`crates/keyed-launch/src/{lib,run}.rs` and `tests/internal/wait_events.rs`,
`crates/jj-workspace/src/lib.rs`, `crates/grove-tui/src/{lib,observation}.rs`
and `tests/witnessed.rs`, with the spec, the driver-lease ADR, USAGE,
ARCHITECTURE, CONTEXT, CONTEXT-MAP, G6 and both book source indexes. No test,
build, lint or format command was run and no code was edited, as the review
family rule requires; the k37/k38/k40/k41/k42/k43/k44 records supply the
executed evidence.

Every attack the Done when names was carried out against the source and held;
the record of what was checked is under Verified below. No correctness or
security defect was found. Three low findings are recorded. They earn one
adjacent integration because the first is a latent intermittent failure of
the principal gate the parent brief mandates before every retirement, of the
exact class k42 already met once inside that gate, and its repair is
mechanical. Cut as `integrate-review-impl` with the bare stem under
witnessed-activity-k12: no later sibling entry in `15-k12` holds live work, so
`leaf-add` lands it where the walk reaches it next.

Walkthrough reconstruction was checked by count, not by running the validator:
the grove-loop book's rollup of 16 roots / 14,444 lines equals the shipped
roots' current line counts (Cargo.toml 68 plus 14,376 source lines, with
`driver_lease/observation.rs` 2,139 and `witnesses.rs` 722 as indexed), and the
keyed-launch book's `run.rs` root matches its 672 lines. k43 and k44 changed no
grove-loop or keyed-launch source, so k37's frozen native evidence applies to
the shipped protocol source unchanged.

## Findings

### F1 — Low — observer unit tests exposed to the inherited-flock race k42 diagnosed

**Evidence.** witness-foreign-holders-k42 recorded a principal-gate failure of
`witness_directory_private_and_marker_error_matrix_obeys_final_probe`: a
parallel test's fork inherited this process's exclusive epoch descriptor until
its exec, the observer's nonblocking shared probe returned Busy, and the test
saw no probe events. k42 wrapped that one test in
`fork_sensitive_driver_lease_test_body_runs_here`. The same shape remains in
other tests of `crates/grove-loop/src/driver_lease/observation.rs`: each holds
and releases an exclusive epoch lock in-process (through `fixture()`'s
`DriverLease::acquire`, `started_fixture()`'s `prepare_launch`,
`activate_session_epoch` or a direct `acquire_epoch_file`) and then asserts an
Idle, Running or Unavailable result that a Busy probe fails, without the
wrapper:

- `witness_started_public_observation_identifies_the_prepared_launch` (line 362)
- `idle_legacy_active_contention_and_recovery_preserve_tree_and_admission` (427)
- `absent_controls_are_idle_but_missing_or_bad_records_are_unavailable` (612)
- `aliases_match_by_identity_and_subdirectories_do_not_borrow_runtime` (668)
- `capture_pause_and_returned_values_hold_no_epoch_or_tree_lock` (726)
- `paused_runtime_reader_allows_shared_observers_and_bounds_driver_handoff` (768, the `sample` at 786)
- `identity_replacements_retry_and_stop_after_eight_attempts` (818)
- `witness_private_open_and_identity_io_errors_release_the_epoch_guard` (937; also its own exclusive loop at 971)
- `witness_capture_uses_accepted_pin_after_root_path_replacement` (985)
- `witness_tree_absence_failure_and_contention_preserve_runtime_identity` (1011)
- `unreadable_epoch_and_nondirectory_namespace_are_unavailable` (1856)

Forks are routine in this binary: every wrapped test, `ForeignHolder`,
`NativeHolder` and each `keyed_launch::run_observed` call spawns a child. The
first four of these are in k37's required native suite, which ran with
`--test-threads=1` and so could not meet the race there; the principal gate
runs the library in parallel.

**Impact.** An intermittent red principal gate with no product defect behind
it, on the gate every producer must pass before retiring. The
`crates/grove-tui/src/observation.rs` unit fixture (`witnessed_fixture`, line
217) has the same shape but that binary was not checked for forking siblings.

**Repair shape.** Add the existing wrapper to the exposed tests, or hoist the
exclusive-lock fixtures so the race window cannot overlap a sibling's spawn.
No production change.

### F2 — Low — reader does not enforce the writer's witness-name grammar

**Evidence.** `LaunchWitnesses::prepare_with` names every private witness
`witness-` plus 32 lowercase hex characters, and `discard_abandoned`
recognises exactly that grammar (`witnesses.rs` lines 13, 52, 161-168). The
observer accepts any plain basename: `observation_witness` rejects only empty,
`.`, `..`, a slash and NUL (`driver_lease/observation.rs` lines 136-144), so
a record naming `driver.lease` or `session.epoch` is resolved inside the
namespace and reaches `observe_private`, whose shared probe (line 260) then
lands on the driver lease file. The spec's step 5 and the ADR both state that
no observer probes the driver lease lock.

**Impact.** Only a hand-edited or corrupted epoch record can produce it, which
the ADR places outside the cooperating-process boundary. Inside that case a
viewer's momentary shared `flock` on `driver.lease` can make a replacement
driver's nonblocking exclusive acquisition fail with "another Grove driver
already owns" at that instant. Cheap to close: require the writer's prefix
and 32-hex suffix before resolving the name, matching the two ends of one
grammar. The grove-tui unit fixture at `observation.rs` line 232 uses the name
`witness-test` and would need a conformant name.

### F3 — Low — spec sentence misdescribes the private probe descriptor

**Evidence.** `docs/specs/item-status.md`, One bounded observation, step 5:
"Read its bounded marker, then probe through an independent descriptor with a
nonblocking shared lock attempt." The implementation reads the marker and
probes on the same descriptor whose device/inode it just matched against the
published identity (`driver_lease/observation.rs` lines 248-260). That is the
stronger reading and mirrors step 4's own rule for the directory ("probe the
captured directory descriptor itself; do not reopen its path"); an independent
descriptor would be a second open of a path whose identity was verified on the
first.

**Impact.** Documentation only. A later implementer following the sentence
literally would weaken the binding. One-clause spec edit.

## Verified

Checked against the source and held, with the evidence file where a producer
record establishes the executed half:

- Started after successful spawn only, Reaped exactly once on confirmed reap
  including wait-error recovery, before token read and terminal recovery, and
  never on an unconfirmed wait (`run.rs` 456-489, 564-635;
  `wait_events.rs`). Failed spawn releases the prepared pair before
  post-attempt invalidation; a Started-only supervision error retains the pair
  through invalidation until lease drop, and the loop then stops rather than
  preparing over an unreaped launch (`driver_lease.rs` 280-283, 339-361;
  `loop_driver.rs` 301-308, 449-465).
- Lease owns both descriptors in one value; Drop closes the private witness
  before the root field; both are close-on-exec and 0o600; unwind and normal
  drop release before driver ownership (`witnesses.rs` 140-146, 55-67;
  `driver_lease.rs` 439-443; k32/k37 sensitivity controls).
- Preparation rechecks the selected pin before and after exclusive epoch
  acquisition, writes the inactive record and cleans before either witness is
  taken, then activates and appends the extension under the same guard, which
  is dropped before spawn (`driver_lease.rs` 272-334; k41 mutation record).
- Fresh names are 128-bit OS-random with exclusive creation and eight retries;
  the extension binds key, handle, kind, tree and witness identities under the
  enclosing nonce/signal record; Started writes the exact eight bytes once and
  never retries; cleanup follows invalidation and preserves an unreaped path
  (`witnesses.rs` 42-137, 157-174).
- Observer: tree capture and discovery precede the shared epoch guard;
  bounded 64 KiB records, 9-byte marker read, identity-race retries bounded at
  eight, every guard released on all paths; directory probe on the retained
  pin before the private witness; final successful private probe wins over
  leftover bytes, marker error or directory error; unlocked matching directory
  is Busy; prefix markers Busy, other bytes Unavailable
  (`driver_lease/observation.rs` 20-105, 212-286; k35 matrix, k38 schedules).
- Forced numeric/key reuse, both release orders, release between probes,
  concurrent shared holders, old-epoch-delayed replacement with preserved
  lease bytes, and foreign shared holders of either witness: production code
  matches the recorded mutation and restoration provenance in k38, k40, k41
  and k42; the two mutations each fail their control while the real Running
  positive passes.
- Native evidence: k37's two scenarios launch a real lease and configured
  child through the runner, contend both witnesses through independent
  descriptors after socket readiness, SIGKILL and `waitpid` the holder, ping
  the exec survivor, then observe both probes released and Idle; the reused
  key case yields PreviousTree. Both hosts list and pass the same 29 entries
  on the same frozen archive; the three helper entries are correctly recorded
  as not independent evidence. Backend and suspension limits stay in the ADR.
- Viewer: two-capture equality includes runtime identity and relation; only
  a SameTree mandate excludes a key, after whole-tree validation; row binding
  resolves by key on accepted current rows only; item absent, tree absent,
  tree unavailable and previous tree qualifiers reserve space before the slug;
  RUNNING style is confined to activity and item spans with lifecycle colours
  intact and cursor as gutter only; 22-cell prefix and 16-cell handle floor at
  60 columns; replacement clears saved state while brief edits keep it
  (`grove-tui/src/lib.rs` 50-161, 421-620, 800-857; `observation.rs` 46-197;
  `tests/witnessed.rs` real-launch cases including killed driver with a
  surviving session).
- Docs: USAGE viewing section, ARCHITECTURE read-only viewer and process
  ownership sections, CONTEXT session witness/RUNNING/NEXT, CONTEXT-MAP viewer
  ownership and G6 describe the shipped behaviour; the modelled-schedule
  versus kernel-teardown distinction is kept in each.

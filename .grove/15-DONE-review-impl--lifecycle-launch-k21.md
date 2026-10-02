# lifecycle-launch-k21

**Reviews:** lifecycle-launch-k12

## Goal

Read the lifecycle cutover adversarially before the two deletion leaves remove
the configuration code it stopped using: the loop's `harness-dispatch run`
invocation, the removal of kind admission, and the launching tests that were
deleted or moved onto the real front and worker.

## Context

- Two commits made the node. `dispatch-launch-k19` holds the code, the test
  migration and the `grove-loop` and overview books. `two-orders-k20` holds the
  `grove-llm` book's rewrite to two orders and its structure specification.
- `.grove/14-k12/_lifecycle-launch.md` holds every decision the cutover made
  and what each was made against.
- `docs/specs/harness-selection-and-execution.md`, *Grove integration* (*A
  lifecycle session*, *A refusal*) and the two Grove launch boundary rows of
  the test seams, are the contract.
- `session_config.rs` and the `grove config` commands still exist and nothing
  launches from them. That is `grove-configuration-k13`'s and is not a gap.

## Done when

- Findings are recorded here, each with its evidence, and an integration leaf
  is cut only if one is worth acting on.

## Notes

The producer spent no in-session reviewer. These are the doubts it left:

1. **Coverage lost in the test migration.** The cutover commit removes about
   1,400 more test lines than it adds, across `loop_driver.rs`,
   `lifecycle_cutover.rs`, `session_kind_tree.rs` and
   `session_kind_presence.rs`. Its log says tests of what configuration does to
   a launch were deleted and not ported. Check each deletion against the old
   file: was the whole subject configuration, or did a surviving behaviour lose
   its only test? Look hardest where one old test covered two things, such as a
   template slot and the terminal handover.
2. **The argv the driver builds.** `crates/grove-loop/src/loop_driver.rs` joins
   each value to its flag in one word and converts nothing. Does every value
   Grove can produce survive that: a prompt that begins with a dash or holds a
   newline, a path with `=` or a space, a session name? What does dispatch do
   with a path that is not UTF-8, and is that the refusal the log claims?
3. **A refused launch.** The driver reports any session that ends in failure
   without a signal, and adds the kind, the handle and that rerunning
   continues. Is the leaf live and the loop stopped in every such ending, and
   does a harness that fails for its own reasons get a message that still reads
   true?
4. **Where dispatch is located.** It is found after the lease and before
   anything is scaffolded. Is there a path that scaffolds or launches before
   that check, including the finish sentinel and a root that does not exist
   yet?
5. **The `grove-llm` book's first order has no test.** `two-orders-k20` found
   that nothing asserts text-before-lock: the refusal tests pass in either
   order. The book records one measurement taken by hand under a held lock, and
   says so. Is a test worth cutting? `tree_lock.rs` already holds the lock from
   outside for the opposite assertion.

## Decisions (running log)

**The failure diagnostic has an actionable finding.** The new no-signal
failure branch claims the selected leaf is still live without observing the
tree. A harness can retire or decompose it and then fail before signalling;
dispatch's native exit cannot distinguish that from a refusal. F1 below goes
to a paired integration, with a real-front regression case.

**One loop-owned session test still bypasses dispatch.** The cutover replaced
the configured command in `selected_root_launches_only_while_its_directory_is_current`
with a direct `/bin/sh` argv. Its successful branch activates a session epoch
and launches the child through the loop's own `launch_session`, so it falls
under the node's explicit real-front-and-worker test requirement. F2 goes to
the same integration; fake programs remain appropriate at the runner's own
interface.

**The book's text-before-lock gap does not require a separate leaf.** The
current handlers parse before opening, the removed admission read does not
reverse that order, and the book accurately distinguishes its one recorded
measurement from an automated test. Automating the held-lock observation
would improve coverage, but this review found no new violation of the order.
Do not make that optional improvement a prerequisite to the deletion leaves.

**The integration is next.** Cut `lifecycle-launch-k22` immediately before
`grove-configuration-k13`, so F1's source coordinates and F2's test boundary
are triaged before intervening deletions. This review changes findings and
task-tree notes only; the integration owns fixes and verification.

## Findings

### F1 — P2: do not assert that every failed session leaves its leaf live

At `14c3328f`, `crates/grove-loop/src/loop_driver.rs:304–311` adds the
unconditional statement “The leaf is still live” whenever a session exits
unsuccessfully without a completion signal. This branch handles both dispatch
refusals and the selected harness's own failure. For example, a harness can
successfully run `grove-llm leaf-retire` and then exit 23 before `complete`
(a failed commit between retirement and signalling is another such ending).
The tree now holds a DONE leaf, but the diagnostic promises that it is live.
Rerunning picks later live work or materializes finish, rather than resuming
that producer. Decomposition before failure likewise leaves a node, not the
selected leaf.

The distinction is already contractual: dispatch forwards the harness's
native ending (`docs/specs/harness-selection-and-execution.md`, *Execution
and authority*), and Grove's *A refusal* promises leaf preservation for a
refused launch. The loop has no dispatch-specific outcome and should not
infer one from the exit status. The adjacent “if harness-dispatch refused”
clause does not condition the subsequent unconditional leaf claim.

Evidence: `drive` invalidates the epoch and discards the channel before this
branch, then returns `LoopOutcome::Stopped` without reading the task tree.
`crates/grove/tests/lifecycle_cutover.rs:645–670` tests exit 23 using a harness
that does no tree work; `crates/grove/tests/loop_driver.rs:3551–3575` tests
native exit 3 and signal death, also without retirement. Neither covers this
counterexample. Keep the rerun guidance, qualify or remove the unobserved
leaf-state assertion, and add a real-driver/front/worker case that retires the
selected leaf before failing without a signal. Keep the existing refused-kind
case's independent assertion that its leaf remains live.

### F2 — P2: migrate the remaining loop-owned successful launch test

At `14c3328f`, `crates/grove-loop/src/loop_driver.rs:622–626` constructs
`Argv::new("/bin/sh", ["child.sh"])` and calls `launch_session` directly.
In the `current` branch the child really runs and writes `launched`; the test
also asserts that the session epoch is active. The producer changed this from
a configured direct shell launch to an explicit direct shell launch, rather
than moving it onto the real dispatch front and worker.

This misses the node brief's *Done when* requirement that every Grove test
launching a session use the real front and compiled worker with a policy in a
temporary HOME (also root brief, *Test seams*). This is a loop launch-boundary
test, not a test of the runner's own interface, where fake programs are
explicitly permitted. It preserves the useful root-removal/replacement checks,
but its successful control does not exercise the new launch boundary at all.
Move that successful session onto real dispatch while retaining the stale-root
refusals and the epoch assertions; keep the harness deterministic and model-free.

## Review scope and evidence

Reviewed the cutover at `14c3328f` and the book rewrite at `7f0d81e4`, against
the current source and the node/root briefs. This was an inspection-only
review: no test, build, lint or format command was run and no production or
test code was edited. The node records the producer's full-check work and its
flake/race outcomes; the book records its `cargo test --locked -p grove-llm`
output and explicitly distinguishes carried-over figures. Those are recorded
producer evidence, not fresh verification by this reviewer.

Graph tier: Tier 2, project
`Users-antony-Development-grove.use-harness-dispatch-directly-for-execution`,
generation `2026-10-02T02:27:48Z`. Initial graph discovery and traces still
describe the pre-cutover source. Coverage checks flag changed evidence files
as stale or untracked, and exclude `docs/`. Material conclusions therefore
use direct source, committed diffs and the historical deleted test bodies,
not the stale graph's edges or line ranges.

The other mandated doubts resolved as follows:

- **Deleted coverage.** Read the removed test bodies in `lifecycle_cutover.rs`,
  `loop_driver.rs`, `session_kind_presence.rs` and `session_kind_tree.rs`.
  The modular-selection, binding, parameter, authority and pre-admission
  refusals test the mechanism being removed. The secondary-workspace scalar
  and task-slot assertions are replaced by the real-dispatch native-data case
  at `crates/grove/tests/loop_driver.rs:1692`. The reload case at
  `crates/grove/tests/lifecycle_cutover.rs:484` now rewrites the policy between
  sessions and checks the next filename kind. The removed live-reload test's
  running-child stability follows the one-shot selection/exec boundary;
  no live configuration watcher replaces it. The deleted bodies do not
  contain a terminal handover assertion hidden inside a template-slot case.
  The migrated PTY cases retain group/PID/cwd, signal-state, native endings,
  typed-interrupt and descendant-escalation assertions. F2 is the remaining
  test-boundary exception identified by this review.
- **Argv.** `dispatch_run` appends native values to fixed `--flag=` prefixes;
  no shell splits them. Dispatch's `SelectionArgs` accepts the resulting
  values, and `inputs::params` splits only at the first `=` after the parameter
  name. Spaces, newlines, leading dashes in values and further `=` characters
  remain data. The mandate already starts with `**Load`, so its leading-dash
  case is hypothetical for this caller. Non-UTF-8 task paths are accepted by
  clap as `PathBuf` and then refused by `inputs::task_file`; non-UTF-8 roots in
  `--param` are refused by `inputs::params`, with `malformed_input`, exit 2.
  Grove does not silently convert those path arguments. The native-data case
  exercises a multiline prompt and distinct roots with spaces and shell
  punctuation. This review did not execute extra byte-edge cases.
- **Refusal and locating dispatch.** `cmd_do` acquires the lease, provisions
  skills, locates dispatch, and only then calls the loop. Root scaffolding and
  finish materialization are inside that loop, after the lookup. The missing
  sibling test at `loop_driver.rs:2042` checks no root was scaffolded. The
  fresh-root/no-policy and refused-finish cases assert the new launch-only
  rule. `drive` stops, invalidates the epoch and discards the channel on a
  refusal; the refused-kind test at `loop_driver.rs:1839` checks the leaf and
  a subsequent successful rerun. F1 concerns the broader native-failure
  diagnostic, not those refusal assertions.
- **First order.** The six text-reading handlers still parse before opening
  the tree. The book's held-lock observation and its caveat are accurate
  about the evidence it records. No required new test is cut for that doubt.

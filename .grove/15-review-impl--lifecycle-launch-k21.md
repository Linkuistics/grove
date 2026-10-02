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

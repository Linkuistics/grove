# direct-dispatch-k3

## Goal

Say how the root brief's settled requirements are met, as simply as they can be,
and rework the dispatch specification and the ADR set to state it.

## Context

- The requirements are the root brief's. `plan-k1`'s log holds the human's words
  and what each choice was made against. Synthesise from them; do not re-ask.
- The human's steer, which is the test for every choice here: *"Simplicity of
  this code is critical"*, and their picture of the tool, *"a trivial
  TS-execution wrapper that passes it's CLI params as args to a TS function and
  acts according to the return value."* Take the answer with the least mechanism
  that meets the requirement.
- What the requirements leave to this session:
  1. how the caller's parameters and the prompt reach `select`, and the exact
     shape of what it returns;
  2. where an owner sets what Grove no longer passes (requirement 8), and the
     number for the selection time ceiling;
  3. how `grove run` keeps its confinement while dispatch still `exec`s
     (requirements 10 and 14);
  4. how Grove finds and runs dispatch, and what it reports when dispatch
     refuses;
  5. the sample, the subcommand that installs it, and the per-worktree helper;
  6. what becomes of each part of dispatch the human did not discuss, listed in
     the brief's last note. Leaving a working part alone is a legitimate answer.
- The ADRs and specs to rework in place are in the brief's *Pointers*.

## Done when

- The specification and the ADR set describe the design as current state, and
  every citation of a record this session deleted or merged is reconciled.
- Each item above has an answer a planning session can cut from.
- The planning leaf `direct-dispatch-k4` carries whatever it needs that the
  specification does not.

## Notes

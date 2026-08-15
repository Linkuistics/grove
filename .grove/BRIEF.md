# grove.steps-fail-to-call-complete — brief

## Goal

Sessions finish their work correctly — artifact written, leaf renamed `DONE`,
commit landed — and then never run `grove-llm complete`. Under the configured
interactive harnesses that does not stop the loop cleanly; it **stalls** it. Make
the signal reliably fire, and do it at the instruction layer.

## Done when

- `grove-llm leaf-retire` and `grove-llm leaf-prune` tell the session its
  remaining loop steps on stderr: commit, then `grove-llm complete`.
- The `skill-signal` unit composes **last** in every kind's mandate.
- Both land with tests and a `## Unreleased` CHANGELOG entry.
- Neither change reaches outside this repo — no harness hook, no personal
  configuration, no driver change.

## Decomposition

Two vertical slices, ordered by weight and risk. Each leaves the product working
alone and is independently revertible.

- `02` **retire-next-steps-k2** — the load-bearing fix. A reminder emitted at the
  moment of decision rather than at session start. Self-contained: two functions
  in `src/llm_cli.rs`, their tests, CHANGELOG.
- `03` **signal-unit-placement-k3** — the reposition. Changes what *every*
  session receives and touches the `content/` build gate and the completeness
  invariant, so it goes second, against an already-improved baseline.

No `review-*` leaves are pre-cut: a producer cuts its own review lazily, only if
the artifact warrants one.

## Pointers

- Glossary terms in play: **Loop control channel**, **Embedded methodology**,
  **Methodology unit**, **File directive**, **Mandate slice**, **Triggering
  unit**, **Meta-grove** (`CONTEXT.md`).
- Code: `src/complete.rs` (the verb — writes a token and returns);
  `src/loop_driver.rs:414-465` (`wait_with_watcher_result`, whose three exits do
  not include "the agent forgot"); `src/llm_cli.rs:800-828` (`cmd_leaf_retire`,
  `cmd_leaf_prune`).
- Content: `content/SKILL.md:558` (`skill-signal`), and the nine
  `<!-- file: order=N -->` directives across `content/`.
- Tests likely in play: `tests/retire_guidance.rs`, `tests/llm_cli.rs`,
  `tests/methodology.rs`, `tests/complete.rs`.

## On the horizon

If both slices land and sessions still skip the verb, the escalation already
weighed and set aside is **driver-side**: give `wait_with_watcher_result` a
second completion observable so a forgotten verb costs nothing. Stated
precisely enough to leaf, but deliberately not leafed — it needs evidence from a
post-rebuild loop that this grove cannot produce.

## Notes

**Established by grilling, and load-bearing for both slices.**

The failure is a **stall**, not a stop. `wait_with_watcher_result` has exactly
three exits: the child exits on its own, the signal file appears, or the driver
itself is signalled. The configured templates launch **interactive** harnesses
(`claude -n …`, `codex --profile sol-high …` — neither carries `-p` or `exec`),
so a session that finishes its turn returns to its prompt and never exits. The
driver's `None` branch at `src/loop_driver.rs:176` — "session ended without a
completion signal … loop stopped" — is therefore unreachable in this failure.
`src/complete.rs:23-27` states the same misreading in prose and should be
corrected wherever it is next touched.

**Decided against, with reasons, so neither is relitigated.**

- *Driver-side robustness.* The driver is unchanged by this grove; the contract
  that the agent signals is kept deliberately. See *On the horizon* for the
  conditions under which this reopens.
- *A harness `Stop` hook.* It fires at every turn end, so to stay safe around
  HITL kinds it would have to read the tree — reintroducing the driver-side
  observation somewhere less visible — and it would live in personal
  configuration grove deliberately does not own.

**A caveat on verification.** This is a meta-grove, so neither change reaches a
session in this loop: `content/` is fixed at build time by `include_dir!`, and
`grove-llm`'s output is the installed binary's. Both slices take effect only
after a rebuild and install, so "did it work?" is a question for the next grove.

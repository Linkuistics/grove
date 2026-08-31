# pipeline-skills-k28

## Goal

Author the extracted kinds as installed `grove-<kind>` skills, wire them into the
conformance runner and the install route, reconcile the prose that states the
methodology's current kind set, and hand the human the launch templates the new
kinds need.

## Context

- The set and each kind's discipline are `pipeline-kinds-k27`'s and are not
  reopened here. If one of them cannot be written as a skill, that is a finding to
  report, not a licence to redesign it.
- Uniformity is with `plugins/grove/skills/grove-<kind>/SKILL.md` as they stand:
  each opens with the imperative to load the spine skill, carries inline only what
  that kind alone owns, and directs a load of a family file **by name** where a
  family owns the rule.
- Assurance to update, not merely to keep green: `plugins/grove/conformance.sh`
  and `conformance.test.sh` with `plugins/grove/conformance/rules.tsv`;
  `plugins/install.sh` and `install.test.sh`, whose `harnesses:` key decides
  installability off Claude Code; and the `include_str!` content assertions in
  `crates/grove-llm/tests/` — `session_kind_presence.rs`,
  `session_kind_guidance.rs`, `composition_guidance.rs` and their neighbours
  read the shipped skill bytes and are the seam that goes red when the set moves.
- Prose stating the current set and its arithmetic: `CONTEXT.md` (*Session kind*,
  and the *Family reference file* and *Loop-step reference file* counts),
  `docs/ARCHITECTURE.md`, and `plugins/grove/skills/grove/TASK-FORMAT.md`, whose
  *The nineteen kinds* section states a table and a count that both move.
- Reachability is not the same as presence. A rule stated in a file no condition
  names is off every loaded path and is delivered nowhere; `CONTEXT.md`'s
  *Loaded path* and *Composed loaded path* entries state the test.

## Done when

- One `grove-<kind>` skill exists per extracted kind, installed by the same route
  as its neighbours, and each is reachable on its kind's loaded path.
- The conformance runner and its suite cover the new kinds, and
  `bash plugins/install.test.sh`, `bash plugins/grove/conformance.sh` and
  `bash plugins/grove/conformance.test.sh` all pass.
- Every place that states the methodology's kind set or its count agrees with
  what now ships — found by enumerating and classifying every statement of the
  set, not by sweeping a list of files.
- `bash scripts/check.sh` passes.
- The human has been given the exact launch-template entries the new kinds need
  and has confirmed they exist.

## Notes

**This leaf ends at a human gate, and that is not optional.** A kind for which no
launch template resolves is refused before the tree is mutated
(`docs/adr/a-kind-is-an-open-token.md`), so the very first `crate-books-k14`
session that tries to cut a stage leaf fails at the verb — in an arm with no
human present. Name the entries concretely, in the form the human's configuration
takes, and stop and ask rather than assuming.

**The methodology and the binary have separate lifetimes.** Editing a skill
reaches a session as soon as the install route resolves to this checkout;
nothing checks that a skill and the installed `grove-llm` agree. Verify this work
by reading the files and running the suites, never by expecting the next session
in the same loop to behave differently.

# grove.use-harness-dispatch-directly-for-execution — brief

## Goal

Grove stops carrying launch policy of its own. It launches every session,
lifecycle and standalone alike, by running `harness-dispatch` itself, and
`harness-dispatch` becomes the thin wrapper its owner described: *"a trivial
TS-execution wrapper that passes it's CLI params as args to a TS function and
acts according to the return value (which should be an ADT)."* One personal
policy file then decides what runs, and Grove's own configuration, with all the
machinery that reads it, is deleted.

**Simplicity governs.** The human, unprompted: *"Simplicity of this code is
critical"*, with the instruction not to overcomplicate, not to overthink, and
not to *"let the perfect be the enemy of the good."* Where two designs meet a
requirement, the one with less mechanism wins. Nothing is built that no
requirement asks for, and a working part the requirements do not touch is left
alone. This applies to the plan and the documents as much as to the code.

## Done when

- Bare `grove` launches every lifecycle session, and `grove run KIND` every
  standalone invocation, through `harness-dispatch`, with no owner-written
  command between them and no Grove configuration read.
- No code, test, document, example or skill in the repository implements or
  describes `config.kdl` or `.grove.kdl` as live. A copy of either left on disk
  changes nothing about a launch.
- A harness-dispatch policy is one `select` function that receives the caller's
  parameters and the prompt and returns a command or a refusal. Every shipped
  example is written to that contract.
- A scripted stand-in for a deciding agent, started by a test policy, receives
  the prompt in the caller's directory and its answer decides what launches.
- The shipped sample reproduces, argument for argument, what the pinned
  personal configuration resolves to: for every kind under its active
  selection, and under each other arrangement the sample offers. A dispatch
  subcommand installs it, and a supported helper reads a per-worktree choice
  file.
- The ADR set, the specs, the references, the glossary, the context map, the
  walkthrough books and the skills describe the result as current state.
- `bash scripts/check.sh` passes on the integrated result.

## Settled requirements

Each was decided by the human in `plan-k1`, whose running log holds the words
used, the alternatives rejected and why. A later session synthesises from that
log and does not re-ask. Cite one as *root brief, requirement N*.

1. **Grove calls dispatch itself.** Every lifecycle session and every standalone
   invocation is launched by Grove running `harness-dispatch`. Grove
   hard-requires it.
2. **Grove has no launch configuration.** Commands, bindings, routes,
   parameters, profiles, selection, the delta and its trackedness check,
   `grove config show` and `grove config examples` are deleted outright, with no
   surviving consumer. Grove reads no personal configuration at all.
3. **Old files are ignored.** A `config.kdl` or `.grove.kdl` left on disk is
   never read and refuses nothing. There is no converter.
4. **Dispatch stays independent.** It has no Grove dependency and no Grove
   concept in its core. Whatever Grove needs from it is a caller-neutral
   capability.
5. **A policy is one `select` function.** It receives every parameter the caller
   passed, the prompt included. It returns one of two variants: the command to
   run (program, arguments, and provider, model and effort labels, which reach
   the run record), or a refusal (code, message, remedy). There is no exported
   catalog, no slot vocabulary, no separate `routes` form and no tool-level
   `--choice` rule. Nothing the command reports distinguishes a static policy
   from a dynamic one.
6. **No launch value is lost.** Grove passes, as parameters, everything a
   command template can place in argv today: the prompt, the kind, the task
   file, the task identity, the session name, the working-tree root and the
   main-repository root. A standalone invocation passes what `grove run` gives
   those names today, and no task.
7. **Dynamic dispatch is possible, and only possible.** A policy can hand the
   prompt to a deciding agent that it starts in the caller's directory and
   reaps. The selection time bound stays finite and owner-set, with a short
   default, and its ceiling rises to admit minutes. No agent policy and no
   model-calling policy ships; a script stands in for the agent in tests. The
   documentation states, where an owner writing such a policy reads it, that
   keeping the agent from executing the task it evaluates is that owner's job.
8. **Owner settings need no Grove.** What an owner sets per invocation today
   through flags in a command definition stays settable with Grove passing none
   of it: the time bound and environment grants to the policy, and the context
   budget, the record location and the policy entry for as long as those
   mechanisms exist.
9. **An unroutable kind is caught at launch only.** Tree verbs and root
   scaffolding consult no policy. A refused launch leaves its leaf live and the
   loop stopped, and rerunning `grove` continues.
10. **Standalone invocations keep their confinement.** `grove run KIND` selects
    through the same policy, with selection outside the sandbox and the harness
    inside it, through a caller-neutral capability.
11. **The sample is the owner's own configuration, converted.** It covers every
    arrangement and modifier the pinned file defines and its standalone
    `release-notes` route, with the real `codex` and `claude` command lines.
12. **Installing the sample is explicit.** A dispatch subcommand installs it as
    the owner's policy. `run` and `inspect` with no policy refuse, and nothing
    is installed implicitly.
13. **Per-worktree variation is a supported helper.** It is documented and
    tested as delivered, and a policy that imports it reads a local choice
    file. The file names one of the options the policy offers and can introduce
    nothing.
14. **The process wrapper stays possible.** This grove keeps the `exec` handoff.
    The runner stays a domain-free unit separable from Grove's loop, requirement
    10's capability does not assume `exec` forever, and no document written here
    turns *policy-evaluation-precedes-process-replacement*'s reopen condition
    into a refusal.

## Out of scope

- harness-dispatch as the process wrapper that handles a harness's completion
  (see *On the horizon*).
- Any agent or model-calling policy, and the evaluation the routing research
  proposes for one.
- A converter from `config.kdl`, an early kind check before a leaf is written,
  and a refusal while an old configuration file is present. Each was offered and
  declined.
- Extracting harness-dispatch to a repository of its own.

## Decomposition

Position order is the order of work. `plan-k1` settled the requirements above.
`plan-k2` reads them adversarially before anything is built on them.
`direct-dispatch-k3`, the design leaf, reworked the dispatch specification and
the ADR set to say how each requirement is met. `direct-dispatch-k5` read that
design adversarially, `direct-dispatch-k6` integrated its findings, and
`direct-dispatch-k4`, the planning leaf, cut it into nine implementation
leaves:

- `parity-fixture-k7` records what the resolver produces, while it exists.
- `select-contract-k8` gives harness-dispatch its new contract. Grove still
  reaches it through an owner's command definition. `select-contract-k16`
  reviews it, and `select-contract-k17` integrates that review before the next
  leaf builds on it.
- `owner-settings-k9` adds the settings file, the raised ceiling and the
  deciding-agent cases.
- `sample-policy-k10` adds the sample, `init` and the choice-file helper, and
  proves the sample against the fixture.
- `standalone-selection-k11` moves `grove run` onto inspection.
- `lifecycle-launch-k12` moves the loop onto `harness-dispatch run` and removes
  kind admission.
- `grove-configuration-k13` deletes Grove's configuration code and commands.
- `runner-templates-k14` deletes the runner's template machinery and the last
  records of configuration.
- `current-state-documents-k15` rewrites the usage documents, the skill and the
  methodology, writes the release notes, and sweeps for anything that still
  describes configuration as live.

`lifecycle-launch-k12` decomposed at the `grove-llm` book. `dispatch-launch-k19`
did the cutover, and `two-orders-k20` rewrote that book's thesis, which named
the deleted presence rule as one of three orders. `lifecycle-launch-k21` reads
the cutover adversarially before the deletion leaves build on it.
`lifecycle-launch-k22` integrates its failure-diagnostic and remaining
loop-test launch-boundary findings before those deletions.
`fork-sensitive-pin-test-k18` is an unrelated test flake that surfaced during
the cutover.

The first four change only harness-dispatch and leave Grove working as released.
The other five ship together: after `lifecycle-launch-k12` Grove still has
configuration commands that nothing launches from, until the two deletion
leaves land.

Every implementation leaf holds to these:

- It lands with `bash scripts/check.sh` passing. That check reconstructs every
  walkthrough book from its crate, so a leaf that changes a crate with a book
  repairs that book itself.
- It retires the glossary terms, and reworks the ADRs and specs, of what it
  removes.
- It decomposes if it outgrows its session, at the seam its own notes name.
- `direct-dispatch-k4`'s notes say where the design lands in the code, which
  records go with the deleted machinery, and what not to build.

## Pointers

- ADRs a session here must read:
  `docs/adr/harness-selection-is-owned-by-policy.md` (reworked by
  `direct-dispatch-k3`: what this grove decides, what each half costs and what
  was rejected),
  `docs/adr/policy-evaluation-precedes-process-replacement.md` (the `exec`
  handoff this grove keeps, and the supervisor it rejected),
  `docs/adr/the-launched-child-is-a-job.md` (the runner contract that stays),
  `docs/adr/complete-session-configuration.md` and
  `docs/adr/untracked-configuration-delta.md` (both describe what is deleted,
  and are themselves deleted with it),
  `docs/adr/a-review-carries-its-creator-reference.md` (why the provider label
  must reach the run record).
- Specs covering this area: `docs/specs/harness-selection-and-execution.md`
  (the design: the contract, Grove's two launches, the test seams' cases),
  `docs/specs/standalone-invocations.md`, `docs/specs/modular-configuration.md`
  (describes what is deleted), `docs/specs/module-decomposition.md` (decision 7,
  the runner's interface, which follows the code).
- Research: `docs/research/grove-model-effort-routing.md`, which anticipated a
  selector that reads the task.
- Glossary terms in play (see `CONTEXT.md`): **Static dispatch / dynamic
  dispatch** and **Deciding agent**, both new here; **Guaranteed core**, **Kind
  routing**, **Loop control channel**, **Creator reference**, **Joint
  candidate**, and the **Grove configuration** cluster this grove retires.
- Test seams, agreed with the human, all existing:
  1. the `harness-dispatch` command, with temporary policies and fake harnesses;
  2. the Grove launch boundary: the real driver, the real front and its compiled
     worker, a fake harness, and the policy in a temporary HOME, with its
     controlling-terminal variant;
  3. `grove run` under real confinement, with a deterministic harness;
  4. the runner's own interface, with fake programs.
  Every Grove test that launches a session goes through the real front and
  worker. There is no fake `harness-dispatch`, and no test calls a model.

## On the horizon

The human wants harness-dispatch eventually to *"handle the completion i.e. the
ability for the executed harness to signal it's completion. Then this becomes a
way to execute a single run of an interactive harness ... so it would also be a
process wrapper."* That is a later grove. It moves the runner (completion
channel, supervision, kill escalation, terminal handover, confinement) under
dispatch, and it needs the supervisor and terminal design the ADR's reopen
condition asks for. Requirement 14 is what this grove owes it. The finish cycle
promotes this note to somewhere that outlives `.grove/`.

## Notes

- **The pinned source of the sample.** `.grove/source-config.kdl` is a byte copy
  of the human's `~/.config/grove/config.kdl` as modified 2026-09-30 12:27:14
  (6063 bytes, SHA-256
  `2294bd271586ae1723bdda4b6e6977a324666264ac7149be9560a78a3de22647`). The tree
  walk skips it. Convert that file, not the live one.
- **Capture before deleting.** The parity check in requirement 11 needs what the
  existing resolver produces for every kind and arrangement. That has to be
  recorded while the resolver still exists.
- **This grove runs on the installed Grove.** Its own sessions are launched by
  the installed release from the human's `config.kdl`, and nothing built here
  changes that until a release is installed.
- **The release is a major one**, and its notes step runs the freshly built
  `grove run release-notes`, which will then need the owner's policy. The owner
  installs the sample with the new subcommand before that release runs, unless
  the Unreleased notes are already written.
- **The design settled what the requirements left open.** How parameters reach
  `select`, where an owner's settings live, the selection ceiling, how a
  confined invocation selects, how Grove finds dispatch, the sample's
  installation and its choice file, and what became of each part of dispatch
  the human did not discuss are in the specification. `direct-dispatch-k3`'s
  log holds what each was chosen over. Every part the human did not discuss was
  left alone, apart from the reshapes the missing catalog forces.
- **Two requirements the design reads narrowly.** Requirement 1: dispatch
  selects every launch's command, `run` launches a lifecycle session, and
  Grove's runner launches a confined `grove run` from what `inspect` reports.
  Requirement 8: an owner's settings are one file, the same for every kind. The
  design review disputed both, and `direct-dispatch-k6` kept them for the
  smaller mechanism. The policy-ownership record says what each was chosen over
  and what would reopen it. Either is the human's to overrule.

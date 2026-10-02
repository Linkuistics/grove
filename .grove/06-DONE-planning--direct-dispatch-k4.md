# direct-dispatch-k4

## Goal

Cut the design that `direct-dispatch-k3` delivers into the smallest increments
that each land green and are useful on their own.

## Context

- The human's steer applies to the plan as much as to the code: do not
  overcomplicate, and do not *"let the perfect be the enemy of the good."*
  Prefer fewer, plainer leaves to a finer cut.
- Known now, before the design exists:
  - Requirement 11's parity needs what today's resolver produces for every kind,
    recorded before the resolver is deleted.
  - Grove's switch to dispatch and the deletion of its configuration are one
    wide change. Add the direct launch, move the tests onto it, then delete the
    machinery.
  - `scripts/check.sh` validates every walkthrough book to its last byte. A leaf
    that changes a crate with a book leaves that book valid, or cuts the leaf
    that will.
  - Methodology text that describes configuration ships in the same release as
    the behaviour it describes.

- The design `direct-dispatch-k3` delivered:
  `docs/specs/harness-selection-and-execution.md` (the contract, Grove's two
  launches under *Grove integration*, the test seams and their cases),
  `docs/specs/standalone-invocations.md` (*Selection*), and
  `docs/adr/harness-selection-is-owned-by-policy.md` (the trade-offs and what was
  rejected). `direct-dispatch-k3`'s log says what each call was made against.
  `direct-dispatch-k5` reviewed it and `direct-dispatch-k6` integrated that
  review. Cut the design as it is now committed. `direct-dispatch-k6`'s notes
  say which findings were taken, and a finding it declined is not work.

## Done when

- The tree holds the leaves that deliver the root brief's *Done when*, in an
  order each can land green.

## Notes

What the specification does not carry, because it is about this grove's work
and not about how the area behaves.

**Where the design lands in the code.**

- *harness-dispatch front* (`crates/harness-dispatch/src/`): `--param` and the
  removal of `--choice` (`cli.rs`, `inputs.rs`); the request gains `prompt` and
  `params` and loses `explicitChoice` (`choice.rs`); the catalog, routes, slots
  and explicit-choice checks go (`policy.rs`, `argv.rs`); the settings file is
  read before the worker starts (`limits.rs`, `environment.rs`, `store.rs`'s
  directory); the ceiling constant is `SELECTION_MAX_MS` in `limits.rs`; `init`
  is a new subcommand that embeds the sample's text; `inspect.rs` and
  `record.rs` report the command and labels, and `record::lookup` flattens its
  answer.
- *Worker and SDK* (`crates/harness-dispatch/worker/`): `sdk/index.ts` types,
  the prompt marker and the choice-file helper; `src/main.ts` and `host.ts` lose
  the catalog snapshot; `examples/dynamic.ts` is deleted; the other four
  examples, `grove/index.ts` and every `typecheck/` fixture are rewritten.
- *Runner* (`crates/keyed-launch`): `Argv` gets a public constructor; a
  confined launch takes an absolute program path, refuses any other, and loses
  its own PATH lookup (`confinement.rs`'s `executable`);
  `templates.rs`, `vocabulary.rs`, `inspection.rs`, `conformance.rs` and the
  configuration half of `error.rs` are deleted. The crate keeps its name; a
  rename is not asked for.
- *Grove* (`crates/grove-loop`, `crates/grove`, `crates/grove-llm`):
  `session_config.rs` is deleted; `loop_driver.rs` builds the dispatch
  invocation and `grove_loop::run` loses its `TemplateSource` argument;
  `driver.rs`'s kind admission and the `require` calls behind `leaf-add`,
  `leaf-insert`, `leaf-decompose`, root scaffolding and the finish sentinel go;
  `standalone.rs` runs `inspect --json` in Grove's own process group and hands
  the runner the reported `executable` and `args`; `config.rs`, `config_json.rs`,
  `examples.rs` and the `grove config` commands are deleted.

**The parity capture comes first.** While the resolver still exists, record what
it produces from `.grove/source-config.kdl` for every kind, the standalone
`release-notes` kind included, under each selection the sample will offer: each
of the four arrangements alone and with each combination of the two modifiers
(`codex-led` and `claude-led` are selected with `routes`; the other two include
it). Record program and arguments with the runtime values (prompt, session name,
the two roots) as named placeholders. That fixture is what the sample's
command-seam test compares against, and it stays in the repository after the
resolver is gone. The sample's default is the pinned active selection,
`claude-led` with `codex-sol`.

**Records this design left for the leaf that deletes the machinery.** Each
describes a mechanism the code still has, and nearly every citation of it sits
in a file that describes that mechanism as live, so they go together and
`scripts/check.sh` stays green at each commit:

- `docs/adr/complete-session-configuration.md`,
  `docs/adr/untracked-configuration-delta.md`,
  `docs/specs/modular-configuration.md` and `docs/design/modular-configuration/`
  are deleted. Their citations are in `CONTEXT-MAP.md` (which also records each
  ADR's owner, and `crates/grove/tests/reference_navigation.rs` holds it to the
  directory), `CONTEXT.md`, `docs/ARCHITECTURE.md`,
  `docs/adr/a-lifecycle-claim-says-what-it-is-over.md`,
  `docs/specs/module-decomposition.md`, the two book-structure specs for
  `grove-loop` and `keyed-launch`, `docs/preservation-baseline.md`,
  `docs/formalism-findings.md`, the `grove-loop` and `grove-llm` walkthrough
  books, and source comments those books reproduce.
- `docs/specs/module-decomposition.md`: decision 6 goes, decision 7 becomes the
  runner alone, decisions 1, 5 and 9 and the requirement *a second configuration
  source overrides and never supplies* follow. Decision numbers are cited from
  source, so keep the surviving numbers.
- `CONTEXT.md`: the **Grove configuration** cluster, **Kind routing**, **Review
  target diversity**, **Selection provider** and **Joint candidate** are retired
  or reworded. The new terms are already there.
- `docs/adr/one-live-driver-per-working-tree.md`,
  `docs/adr/a-refusal-leaves-nothing-standing.md` and
  `docs/adr/a-lifecycle-claim-says-what-it-is-over.md` mention configuration
  validation as a step in the driver's lifecycle. Their decisions stand; their
  wording follows the code.
- `docs/CONFIGURATION.md` and `docs/configuration-forms-audit.md` describe only
  what is deleted. `docs/USAGE.md`, `docs/ARCHITECTURE.md`,
  `crates/harness-dispatch/README.md`, the `configure-grove` skill and the
  methodology's references that name `config.kdl` (`references/driver.md`,
  `references/decompose.md`'s *Diversity is the configuration's*,
  `TASK-FORMAT.md`) are rewritten, in the release that changes the behaviour.

**Not to build.** No fake `harness-dispatch` for Grove's tests, no table helper
in the SDK, no run record for a standalone invocation, no launcher on `run` for
a confined launch, no owner settings that differ by kind, no rename of
`keyed-launch`, and no agent policy.

## Decisions (running log)

**One grove, not two.** The harness-dispatch leaves leave Grove working as
released, so they are a point where this could have been two groves and two
releases. It stays one: the requirements ask for one result, the root brief
plans one major release, and a release in between would break owners twice.
The boundary is kept in the leaf order, so the human can still split there.

**Flat leaves under the root, and no nodes.** Nine sibling `impl` leaves in the
order of work. A leaf that outgrows its session decomposes itself, at the seam
its own notes name.

**The contract change is one leaf.** Policies, examples and tests all change
shape at once. Accepting both schema versions for a while would keep it green
in smaller steps, at the cost of building coexistence that is then deleted. That
is the fallback if the leaf proves too big, not the plan.

**`grove run` moves before the loop.** It is the smaller cutover and it brings
the two shared pieces, finding dispatch beside Grove and the runner's argv
constructor, before the loop's wide test migration needs them.

**Deletion runs consumers first.** The `grove config` commands use the session
configuration, which uses the runner's templates, so they go in that order and
each step compiles. A record of configuration goes in the leaf that removes its
last citation from live code.

**A book is repaired in the leaf that changes its crate.** A following book
leaf would leave `scripts/check.sh` red in between.

**No review of this plan.** The root brief gives the requirements and the
design a review each and this leaf none. A wrong cut shows up as a red check in
the leaf that meets it and is repaired there with an insert or a decompose. The
in-session reviewer was not spent either.

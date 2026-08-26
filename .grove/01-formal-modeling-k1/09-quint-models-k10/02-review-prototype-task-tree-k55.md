# task-tree-k55

**Reviews:** task-tree-k11

## Goal

Attack the two things `cross-model-replay-k15` will not read: the Quint driver
in `models/run.sh`, and the three narrowings that let `task-tree.qnt` reach
green.

## Context

The producer is `task-tree-k11` — `crates/grove-task-tree/models/task-tree.qnt`,
`crates/grove-task-tree/models/task-tree-controls.qnt`, the Quint half of
`models/run.sh`, the Quint section of `crates/grove-task-tree/models/README.md`,
and entry 044 of `docs/formalism-findings.md`.

**The independence barrier still applies to you.** Do not open any `.als` file,
the Alloy sections of any model-directory `README.md`, or entries 026 – 043.
Everything you need is the catalogue, the Quint model, the runner and the
README's Quint section. The barrier comes down at `cross-model-replay-k15`, not
here.

This leaf is inserted **ahead of** `prototype-finish-k12` and
`prototype-system-k13` on purpose: both inherit this runner and these
conventions, so a defect found after they land is a defect in three columns.

## Done when

Each of the doubts below is either discharged with evidence or written up as a
finding. A review that finds nothing creates no `integrate-` leaf and simply
retires.

### The runner asserts what it claims

- The Quint driver's module rule — a `const`-bearing module is a library and is
  not runnable; an instance prefixed `relax_`, `mutant_`, `scenario_` or
  `verify_` carries only its own commands, every other instance inherits the
  library's — is derived by parsing the model. Does the parse actually hold on
  the two files, or does it silently classify something wrong and drop its
  commands? A dropped command is invisible: it neither runs nor fails.
- Coverage is asserted per `(family, obligation)` in both directions. Does the
  Quint half genuinely feed `covered_prop` and `covered_wit`, and does a command
  naming no obligation genuinely land in `bad_commands`? Add a deliberately
  mis-named command and a deliberately deleted one, and check the runner goes
  red for each.
- `strip_outcome` now serves both dialects. Does any Alloy command name in the
  repository strip wrongly under the new prefixes?
- A dead tool must abort rather than report. Break `quint` on `PATH`, and break
  the Apalache heap, and check that each aborts with exit 2 rather than
  recording a verdict.

### The narrowings are honest

Three invariants were narrowed to reach green. In each case the producer claims
the catalogue is wrong; the alternative is that the model is wrong and the
narrowing hid it. They produce identical green runs.

- `inv_TT_17` checks only the Current/Legacy/Foreign decision, not the whole
  classification.
- `inv_TT_20` collects only interruptions with no `foreign-write` during the
  initialisation, and only those with work still pending.
- `inv_TT_15a` is guarded by `walkStageReached`.

For each: state what the unnarrowed claim would require, construct the trace the
producer says is a catalogue defect, and say whether the defect is in the
catalogue or in the model.

### The five findings survive a hostile read

Entry 044 records five material findings and claims each lands durably. Check
the two that carry derived tests especially: is the `PartialScaffold` /
foreign-write trace really four transitions of granted assumptions, and does the
bulk-mark convergence argument really follow from `TT-23.a` plus
`AlreadyTerminal`, or does it depend on a modelling choice the producer made?

## Notes

Two things are explicitly **not** yours. The disposition of any catalogue defect
belongs to `formal-synthesis-k16`; your job is whether it *is* one. And the
unique/overlap tagging in entry 044 belongs to `cross-model-replay-k15`; the
producer tagged everything `quint-only` on its own authority and said so.

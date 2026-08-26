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

## Findings

1. **High — reverse obligation coverage accepts invented obligations.**
   `models/run.sh:273-280` treats every syntactically shaped identifier as an
   obligation, and `quint_account` / `run_alloy_file` record it without checking
   it against `manifest` (`models/run.sh:310-324`, `440-456`). Therefore a
   command such as `inv_TT_99_misspelled` does not enter `bad_commands`; it is
   counted under an unused associative-array key and can leave the run green.
   Deleting the last property or witness for a real obligation does make the
   forward matrix red, but the promised model-to-catalogue direction is absent.

2. **High — an Apalache failure can be reported as a green verdict.**
   `quint_run_verify` aborts only for five matched strings
   (`models/run.sh:404-414`). For every other non-zero exit it marks each
   invariant whose name is absent from the error output as “model-checked … no
   counterexample” (`models/run.sh:420-425`). JVM-startup and heap failures whose
   wording is not in that list therefore return success instead of exit 2. The
   front-door `quint` probe does correctly abort when `quint --version` cannot
   launch, but it does not discharge the backend case.

3. **High — the bulk convergence instrument does not establish re-running the
   same invocation.** `Hist` retains only sticky booleans, not the interrupted
   plan (`task-tree.qnt:1031-1049`). Any later completed `TBulkMark` sets
   `bulkRepaired`, and divergence is checked against that later operation's keys
   (`task-tree.qnt:1209-1222`). The focused instance permits both `[1,2]` and
   `[2,1]` (`task-tree.qnt:1536-1538`), while its witness is only
   `hist.bulkRepaired` (`task-tree-controls.qnt:206-224`). A refused retry also
   never reaches `finishOp`, so it cannot set `bulkDiverged`. The argument that
   a member already in the target state must be admissible follows from
   `TT-23.a` plus `TT-23.b`; this model does not demonstrate the stronger claim
   recorded in entry 044 and the derived test: that the identical invocation
   repairs the identical interrupted plan.

4. **Medium — `TT-15.a` is a third catalogue narrowing but is omitted from the
   durable finding count.** The literal invariant would require every snapshot
   classified `CurrentSpent` to return `Empty`. The model adds
   `walkStageReached` (`task-tree.qnt:1927-1938`) because a current root with no
   live tasks and a foreign artifact at a reserved name still classifies
   `CurrentSpent`, while `TT-24.b` requires
   `Refused(ReservedNameOccupied)`. This is an honest catalogue conflict, not a
   model defect, but the README says only two obligations are narrowed and entry
   044 does not record this sixth catalogue issue.

5. **Medium — entry 044's second “missing outcome” is partly produced by the
   model.** `stepOpBlocked` always returns `Blocked(OwnershipConflict)` for a
   blocked create (`task-tree.qnt:1182-1195`), including `TAddLeaf` before any
   effect has landed. In that trace the tree is byte-identical and the catalogue
   already has the refusal `DestinationOccupied`; its outcome partition reserves
   `Blocked` for a transaction that stopped part-way. The model must distinguish
   a pre-effect ordinary collision from a collision after partial mutation
   before it can support the claim that the closed outcome set lacks a member.

6. **Low — the runner's `verify_` rule is not one coherent contract.** The exact
   parser finds all current declarations: `taskTree` is the sole const-bearing
   library with 83 commands; `base` and `verify_small` own none; every controls
   module owns the commands visible in its body. The review brief says every
   `verify_` instance carries only its own commands, which would make
   `verify_small` zero-work. The runner instead special-cases `verify_` to
   inherit every library property (`models/run.sh:577-586`), matching the README
   but contradicting that stated module rule. No current command is silently
   dropped, but downstream users cannot derive the convention from one source.

7. **Low — the retained `TT-20` trace is described with two incompatible
   transition counts.** The README records `beginOp`, three `stepOp` firings,
   `foreignWrite`, and `crashNow` — six state transitions — while entry 044 calls
   it four transitions. The counterexample itself is sound: the exact-closed
   `PartialScaffold` definition and a granted foreign write force `Legacy` after
   interruption. The evidence-quality claim and derived-test description should
   use one trace and one count.

## Discharged doubts and limits

- `TT-17`'s unnarrowed form would require the whole root classification to be
  invariant under task-entry text changes; `PartialScaffold`'s exact name-and-byte
  comparison is a catalogue contradiction, so narrowing to the format decision
  is honest.
- `TT-20`'s no-foreign-write and work-pending guards isolate the catalogue's own
  counterexample and exclude a crash after the final publish; both are honest,
  subject to finding 7's evidence wording.
- `strip_outcome` removes the documented compound prefixes before their bare
  counterparts and maps every permitted dialect shape correctly. The
  independence barrier forbade opening `.als` files, so this review did not
  enumerate their actual command declarations.
- The `PartialScaffold`/foreign-write, `TT-17`, and `EN-11` findings in entry 044
  survive the hostile read. The bulk finding survives as a requirement inference
  but not as the stronger executed claim (finding 3); the two-outcome finding
  survives only for the `PartialScaffold` case (finding 5).

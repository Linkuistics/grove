# system-k59

**Reviews:** system-k13

## Goal

Attack the one thing `cross-model-replay-k15` will not read: the INSTRUMENT this
column's 25 green obligations were reached with. Replay re-derives the findings;
nothing else re-derives the search shape, the narrowings, or the four
obligations that die only inside a bundle.

## Context

The producer is `system-k13` — [`models/system/lifecycle.qnt`](../../../models/system/lifecycle.qnt),
[`models/system/lifecycle-controls.qnt`](../../../models/system/lifecycle-controls.qnt),
the Quint column of [`models/system/README.md`](../../../models/system/README.md),
and entry 046 of [`docs/formalism-findings.md`](../../../docs/formalism-findings.md).

**The independence barrier still applies to you.** Do not open any `.als` file,
any Alloy section of a model-directory `README.md`, or entries 026 – 043 — and
note that 040 – 043 are the Alloy column's four LIFECYCLE entries, so the
temptation is stronger here than it was for either sibling review. Everything you
need is the catalogue, the two `.qnt` files, the runner and the README's Quint
section. The barrier comes down at `cross-model-replay-k15`, not here.

The green run to attack:

```sh
models/run.sh --scope lifecycle --family quint
```

72 commands, 25 of 25 cells, exit 0, 2m 05s wall.

## Done when

Each doubt below is either discharged with evidence or written up as a finding.
A review that finds nothing creates no `integrate-` leaf and simply retires.

### `driverStep` is a search narrowing living in `base`, and that is the biggest hazard here

Both sibling columns put their search narrowings in `scenario_` instances, so
every claim stayed checked unfocused in `base`. This column did not: `base`'s
own `step` is `any { driverStep ×3, envStep }`, and `driverStep` is a chain of
conditionals that picks ONE next move from the state. So every one of the 25
properties is checked only over orderings that chain admits.

- **Enumerate the orderings `driverStep` cannot produce.** For each, decide
  whether Grove's real driver can produce it. A reachable ordering the chain
  omits is a property checked over a world smaller than the claim.
- The chain reads `w`, `d` *and* `hist` (`nextIsTransition`, the `RSCurrentLive`
  branch). Does any branch condition on a HISTORY flag in a way that makes a
  claim about history true by construction?
- `envStep` weights the world at one disjunct in four and caps it at
  `ENV_BUDGET = 3` in `base`. Is there a `SY-` obligation whose falsification
  needs four or more environment actions in one trace? If so it is unfalsifiable
  in `base` and nothing says so.
- The producer's own measurement — 5 of 25 witnesses under a flat menu, 23 of 25
  under `driverStep` — is the evidence that the dial was necessary. Re-run it. Is
  the flat-menu figure reproducible, and does it hold at the runner's real 8000
  samples rather than at the 2000 the producer measured?

### The three narrowings are where a false green and a true one look identical

In each case the producer claims the catalogue is wrong. The alternative is that
the model is wrong and the narrowing hid it.

- **`SY-13`'s sweep** excludes `SCLegacy`, `SCForeign` and `SCMalformed`.
  `mutant_literal_sy13` fires, which shows the exclusion is load-bearing — it
  does **not** show the catalogue is wrong. Read `SY-13`'s two sentences yourself
  and decide independently whether the definition and the enumeration really
  disagree, or whether "admitted" was always meant to include the operator's
  actions and the producer read it too narrowly.
- **`SY-14`'s sweep** is `ADMITTED.filter(touchesTree)`. Check the producer's
  `FN-26` argument: does an operator's restorable exit genuinely require the
  blocked process to release its lease, or is that an artifact of modelling the
  lease as the same object across processes?
- **`SY-04.a`'s cap** is an enabling condition rather than an outcome, so half
  the obligation is not falsifiable through the outcome vocabulary. Is
  `mutant_many_transitions` genuinely checking the claim, or only the counter the
  model keeps for it?

### Four obligations have no isolating control

`SY-07.a`, `SY-09.a`, `SY-09.b` and `SY-14.b` die only inside a bundle. For
each: construct an isolating mutation, or establish that none exists and say
why. A bundle control that also kills a neighbour cannot distinguish which claim
was carrying the weight.

### Two witnesses have almost no margin

`SY-06.b` lands in 16 traces of 8000 and `SY-07.a` in 18. Both are deterministic
under the fixed seed. Establish whether each is a genuine protocol path or an
artifact of the scenario's own construction — the same question `finish-k57`
asked of the twelve `scenario_` witnesses, and the one the *vacuous invariant*
hazard is about. Check the other twenty-three the same way if the two turn out
badly.

### The composition abstraction may have abstracted away the claim

The tree is a ten-field summary and the finish is a five-phase cursor. `SY-05.b`
is the whole reason the scope exists, and it is checked against a model where
`FN-11` and `FN-19` are *represented by a single boolean*.

- Does `inv_SY_05b` check anything the composition did not already guarantee by
  where the producer chose to set `deletionProven`?
- `SY-11.b`'s cycle needs the launch generation held by the session's process
  rather than the driver's. The producer changed `launchOp` to do that mid-build
  in order to make `mutant_guard_disorder` fire. Is that faithful to Grove, or
  was the model bent until the control worked?

### The `verify_small` result is quoted with its depth

`quint verify` returns `NoError` at `--max-steps=4`, and the README says depth 4
reaches no scaffold. Confirm the shortest-path figures (eight driver moves to a
completed scaffold, nineteen to a proven finish) and that no obligation in the
column is cited anywhere as model-checked.

## Notes

Do not fix anything. Findings only — an `integrate-review-prototype` leaf is cut
by you, and only if you have findings worth acting on.

Two conclusions the producer reached that you should try hardest to break,
because they are the ones the rest rests on: that `driverStep` removes no
behaviour from the model, and that the five catalogue findings in entry 046 are
defects in the catalogue rather than in the reading of it.

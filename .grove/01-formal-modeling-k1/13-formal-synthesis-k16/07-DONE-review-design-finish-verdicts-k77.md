# finish-verdicts-k77

**Reviews:** finish-verdicts-k65

## Goal

Attack the four `keep` verdicts and the reading that produced them. The specific
doubt is that **the session which declared a pre-registration mis-typed is the
session whose verdicts that declaration licenses**, and no fresh context has seen
the argument.

## Context

`finish-verdicts-k65` answered `TODO.finish_process.md` Q1 – Q4 all **keep**,
deleted that file, and landed
[`docs/adr/finish-layers-are-forced-not-chosen.md`](../../../docs/adr/finish-layers-are-forced-not-chosen.md).
It inserted **no** `impl` leaf at either target, so the whole of its effect on
the product is that the implementation phase will not consider simplifying
finish. A wrong `keep` is permanent in a way a wrong `delete/replace` is not: it
retains 10,366 lines and 31 `unsafe` blocks and nothing downstream re-opens the
question.

**It spent no in-session reviewer** — the harness it ran under forbade
subagents — so this leaf is the *only* adversarial read the decision gets.

**The load-bearing move, and the one to attack first.** Q1's and Q2's
pre-registered `delete/replace` criteria are each stated over a
**counterfactual-capability** control (`relax_EN_03`, `relax_EN_05`). `k65`
argues that such a control measures whether a cheaper protocol would be
*admissible*, that `EN-03` and `EN-05` are facts about the world rather than
modelling conveniences, and therefore that **no green run of either could ever
license a removal**. If that reading is wrong, four verdicts move. Three ways it
could be wrong, and the third is the one a fresh context is for:

- **The pre-registration meant something else by `delete/replace`.** Perhaps it
  meant *this mechanism carries no shared safety, so re-engineering it is
  licensed*, not *delete it*. `k65` answers that even under that reading the
  aside-rename is forced, so nothing cheaper is available — check that
  sub-argument rather than the framing.
- **`EN-03` or `EN-05` is not actually a fact about the world.** `EN-03` is *no
  atomic recursive directory deletion*; `EN-05` is *no filesystem transaction can
  include a version-control commit*. Both are stated as grants. Is either false
  on a platform Grove targets, or obtainable by some other construction?
- **The derivation that the quarantine is *forced* has no control.** `k65` argues
  `EN-01` + `EN-03` + `EN-08` + `FN-24.a` + §*States* having no member for a
  partially removed task root. **No model command runs the combination it rests
  on** — quarantine removed with `ATOMIC_DISPOSAL = false` — and `k65` says such
  a control cannot exist because the dial is one `const`. Verify that: read
  `crates/grove-finish/models/finish.qnt` around `SDisposeInPlace`, and decide
  whether the impossibility is a property of the protocol or of this model's
  parameterisation. **If it is the model's, the derivation is an argument wearing
  a measurement's clothes** and the honest verdict for Q1 is `defer` with that
  control commissioned.

**Four narrower claims, each cheap to falsify.**

1. **`FN-13` is shared safety** (`k65` settled the register-versus-README
   disagreement in the register's favour). The counter is that `FN-13` names the
   reserved witness and is vacuous under its removal — which `k65` concedes and
   says does not decide the class. Is the concession fatal?
2. **Q1's retained set swaps `TT-24` for `FN-32`.** Check that nothing `TT-24`
   covered in the *finish* context is lost by the swap, and that
   `inv_FN_32_ownership_still_proven_under_the_candidate` is not green for want
   of a reachable antecedent under `relax_EN_03` — a vacuous retained claim is
   worse than an unchecked one.
3. **Q4's three `none` rows are read as "cannot be removed anyway".** Confirm the
   Alloy rows really are `argument`-class and that `k65`'s naming-versus-necessity
   distinction is a real gap in the matrix rather than a re-reading of rows that
   already said what it says.
4. **Root creation is rejected on a line-count measurement** (ten, eleven and
   thirty-six lines). Line counts are exactly what the catalogue says is not
   evidence. `k65`'s claim is about *depth* rather than size — check that the
   argument survives if the numbers are ignored.

## Done when

- Each of the four verdicts is either confirmed on its own evidence or
  contradicted with a named artifact, command or trace. "It seems right" is not a
  confirmation.
- The counterfactual-capability reading is confirmed or refuted **against the
  catalogue's own text**, and if refuted, the affected verdicts are named.
- The claim that no control can remove the quarantine with `ATOMIC_DISPOSAL =
  false` is checked against the model source, not against `k65`'s summary of it.
- Findings are written into a cut `integrate-review-design finish-verdicts` leaf
  if there are any worth acting on, and none is cut if there are not.

## Notes

Read the committed artifact from `finish-verdicts-k65`'s own commit and the
current tree; the running log in
`.grove/01-formal-modeling-k1/13-formal-synthesis-k16/06-DONE-design-finish-verdicts-k65.md`
carries every claim with its citation, which is the surface to attack.

**Do not re-run the whole repository** — that is `handoff-audit-k66`'s, and this
leaf was inserted *ahead* of it deliberately, so that an audit of the hand-off
does not certify a conclusion this review might move. `models/run.sh --scope
finish --family quint` is about five minutes if a command needs checking.

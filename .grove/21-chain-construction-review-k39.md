# chain-construction-review-k39

**Kind:** review-design

## Goal

Review the design produced by **chain-construction-k38** — the decision on
whether and how chain construction gets a mechanism. Find, do not fix; the
findings are applied by **chain-construction-integrate-k40**.

## Context

The design under review has one obvious failure mode and one non-obvious one,
and the review should be biased towards both.

**The obvious one: a verb that gates.** grove's constraints 3 and 5 —
*suggested shape, not enforced schema* and *grove guides, it does not gate* —
both push against making chains structural. **chain-group-unit-k36** already
rejected a first-class chain on exactly this axis, and found that "remove a
gate" arguments can smuggle in a larger gate. Test the design for anything that
*validates*, *refuses*, or makes the non-chain path harder rather than making
the chain path easier.

**The non-obvious one: a mechanism that does not change behaviour.**
**compose-task-chains-k29** is the precedent that matters: five documented
surfaces produced zero chains in 26 leaves, because none of them was the surface
a session reads *while cutting*. A verb has the same exposure — a
`leaf-add-chain` nobody reaches for is k29's failure with a compile step. Test
whether the design says how a session *comes to use it*, not just that it
exists.

## Done when

- The design is reviewed against constraints 1–7, naming the ones it strains
  and the ones it is neutral on. Constraints 3 and 5 get an explicit verdict.
- Both failure modes above are checked for, and the check is reported even when
  it finds nothing — a clean bill on a named risk is a finding.
- If the design chose "prose is enough", that answer is reviewed as hard as a
  verb would have been: does the argument turn on evidence that k29's fix
  changed the odds, or on the same optimism k29 falsified?
- The vendor-pair coverage is checked — `research` has no `review-research`
  sibling, so a surface designed around the review chain may not fit it, and a
  design that quietly covers only one shape should say so rather than imply
  both.
- Findings are written down in a form k40 can apply, each with enough context
  to be actionable without re-deriving the design.

## Notes

Cut together with its producer and integrator, per the habit
**chain-group-unit-k36** named: a chain's steps go into the tree in one motion,
because a step decided on after its producer ran needs `leaf-insert` and is the
shape that actually breaks a chain in practice.

## Findings

The direction survives review, but it cannot be integrated unchanged. Two
behavioural holes would let either new verb produce the exact
wrong-but-well-formed result it exists to prevent, and the ADR set is internally
inconsistent while the verbs are adopted but unimplemented.

### F1 — one CLI call is not yet one reliable mutation

**Actionable.** The spec promises a whole shape from one call, then defines the
implementation as "`leaf-add`'s, three times over"
(`docs/specs/task-kind-taxonomy.md`, *Constructing a chain is one call*).
Today's `tree_grow::leaf_add` validates, allocates and immediately writes one
file. A failure on the second or third call therefore leaves a valid-looking live
prefix of the shape. A blind retry can append a duplicate chain under new
positions and keys; if paths are printed as each call returns, stdout can also
describe a mutation the overall command reports as failed.

**k40 must settle the mutating-command contract before implementation:**

- validate the parent, the complete producer-kind domain, the harness and all
  three derived slugs before the first write;
- allocate the three positions and keys from one snapshot;
- either make creation all-or-nothing with safe rollback, or define an explicit
  recoverable partial-failure/idempotency contract whose error names every path
  that persisted (all-or-nothing is the design's natural promise); and
- buffer stdout until success, then print exactly the three paths.

Test failure after write one and write two, not only successful derivation. The
observable assertion is no silent partial shape (or the precisely documented
recoverable residual), because merely asserting that the happy path made three
files does not test the reason for the composite verb.

### F2 — the pair verb does not establish a vendor pair

**Actionable.** `leaf-add-pair <parent> <stem> --harness <name>` leaves producer
A undeclared and writes `<name>` only on producer B. Requiring that flag does not
prove the vendors differ: A resolves through leaf → kind → family → stamp, and B's
name may be the same effective harness. Worse, A's undeclared result can change
when routing policy changes after construction. The claim that the required flag
prevents "a pair that looks like one but is not" is therefore false; the verb can
construct exactly that state.

k40 must either persist and compare both producer harness facts (rejecting an
equal pair before writing), or explicitly weaken the command to a syntactic
three-leaf template and remove the reliability claim. The latter does not meet
this leaf's stated vendor-pair contract. Reconcile `content/driving.md` at the
same time: its claim that undeclared A "falls through to the grove's stamp"
omits the kind and family policies that precede the stamp.

### F3 — the normative ADR enumeration disagrees with its adopted option

**Actionable before k40 retires.** ADR *cli-binary-split* calls its opening list
the one normative enumeration of the verb surface, and says two lists that drift
"bind nothing." That list omits `leaf-add-chain` and `leaf-add-pair`, while the
same current-state ADR calls both verbs adopted. The spec deliberately defers the
enumeration update until implementation, leaving the ADR internally incoherent in
the meantime.

Because k40 is also the implementation leaf, the bounded repair is simple: land
the implementation and the reconciled normative enumeration in the same focused
change. If either verb does not land after F1/F2 are applied, remove its adopted
claim instead. Do not preserve a roadmap state inside the ADR.

## Named-risk verdicts

- **Gating / constraint 5 — clean bill.** Plain `leaf-add` is unchanged; no
  existing tree is linted for chain completeness; the caller still makes the
  escalation decision. Refusing arguments that cannot denote the selected shape
  is authoring-time validation, not a process gate.
- **Suggested shape / constraint 3 — clean bill.** The verbs write the existing
  convention but nothing parses the relationship afterwards. Hand-cut and partial
  shapes remain legal.
- **Encounter at cutting time — clean bill on the design, conditional on k40.**
  The spec names the correct intervention: replace the hand-cut commands in the
  Decompose/driving/bootstrap surfaces and place both verbs beside `leaf-add` in
  `grove-llm --help`. Appending parallel instructions instead of replacing the
  old procedure would recreate k29's failure and does not satisfy this verdict.
- **Vendor-pair coverage — structurally present, semantically failed.** A
  dedicated research shape correctly avoids inventing `review-research`, but F2
  means the emitted shape does not yet guarantee the property that distinguishes
  a vendor pair from three ordinary research leaves.

## Constraint verdicts

1. **Artifacts, not state — neutral.** Both verbs emit ordinary leaves and add no
   scheduler or side state.
2. **Read, don't run — neutral.** Bootstrap remains markdown-first; the commands
   are optional authoring mechanics.
3. **Suggested shape, not schema — passes**, as above.
4. **Lazy and optional — lightly strained, not violated.** The code mechanism is
   being adopted after one maximally primed post-k29 chain and no observed
   wrong-kind incident, so the evidence for implementation is thin. Runtime tree
   growth remains genuinely lazy because only the caller can choose the composite
   verb. Keep that trade-off explicit; do not claim stronger empirical evidence.
5. **Guides, does not gate — passes**, as above.
6. **Walk-away-able — neutral.** The output is the same standard markdown tree as
   a hand-cut shape.
7. **One page of rules — neutral if k40 replaces the hand-cut procedure.** The
   loop gains no branch. Keeping both old commands and new commands as parallel
   guidance would strain this constraint and the encounter-at-cutting-time claim.

# handoff-audit-k66


## Goal

Close the formal phase: prove that `documentation-k2` inherits no unresolved
semantic question, and that every durable formal artifact is reachable from the
component that owns it.

## Context

**This is the parent's last two `Done when` clauses, and it is an audit rather
than a design.** It runs last because it checks its siblings' output: a
semantic question is unresolved until `catalogue-disposition-k64` has disposed
it, and an artifact's owner is not settled until `obligation-placement-k63` has
said which crate owns which claim.

**The completeness instrument, with its controls.** The audit's claim is
repo-wide — *no artifact still defers a decision to the formal phase* — and a
repo-wide claim is not evidenced by a clean grep. Enumerate and classify rather
than sweeping a pattern list, and control in both directions:

```sh
grep -rn "formal-synthesis-k16\|formal-modeling-k1" . \
  --exclude-dir=.jj --exclude-dir=target --exclude-dir=.grove \
  --exclude-dir=_apalache-out --exclude-dir=.review-tmp
```

A live handle that must still be found (a sibling's) and an invented handle that
must find nothing are the two controls; a broken instrument reads clean
everywhere, and clean-here plus dirty-there cannot be produced by one. The
sweep's scope is the **claim's** scope, not a file list written before the work
— and a finding against a section does not reach the summary layer, so when an
item lands against a heading, sweep the abstract, the table and the overview too.

**Reachability, artifact by artifact.** `.grove/` is process state and is
deleted at finish, so anything reachable only from a task file is lost. Each of
these must be reachable from a component owner:

- the three scope `README.md`s, from the crate or directory they document;
- `docs/specs/semantic-contract.md` and the ADRs the phase produced, from
  `CONTEXT-MAP.md`'s ownership list — which is where a record added later joins;
- `docs/formalism-findings.md`'s Experiment 2 pre-registration **and**
  synthesis, from wherever a reader of the architecture would look for "what did
  the formal phase conclude";
- the derived tests, from the entries that own them, so the implementation phase
  finds them without reading the whole log.

**Three documentation gaps already found**, by `experiment-synthesis-k62`, which
checked all six model files against the seven fields the node brief requires.
**All three Alloy halves carry all seven.** The three Quint halves each miss two
or three of the same ones, and each gap is **placement rather than substance** —
the material exists, in an entry or in the catalogue, but not as a field a
reader can scan for:

1. **No explicit fairness statement in any Quint half.** Every Alloy half has
   one. Entry 044 states it (*"No fairness assumptions: nothing here is a
   liveness claim"*) and the catalogue settles it for the whole experiment at
   `SY-13`, which is *existential reachability and deliberately not a liveness
   property*.
2. **Tool version stated only incidentally** in the task-tree and finish Quint
   halves — `quint verify 0.32.0` inside a *Verification* paragraph rather than
   as a field. `models/system/README.md` is the model to copy: `quint 0.32.0`
   on the run line beside samples, depth and seed.
3. **No *what a green run does not prove* section in the task-tree Quint half.**
   *Narrowings and qualifications, each declared* carries much of it; entry
   044's *Missed* carries the rest.

The task-tree **Alloy** half states its own version of (3) as a bolded paragraph
inside *The mutation matrix* rather than as a heading — findable by reading, not
by scanning. Worth the same fix.

**Fix these here rather than earlier on purpose.** `catalogue-disposition-k64`
may change what those READMEs say; editing them before it lands would be two
passes over the same files.

## Done when

- Every site the enumeration finds is resolved: the decision is landed, or the
  site is rewritten to name the decision rather than the leaf that owed it. **No
  artifact outside `.grove/` still says a formal-phase leaf owns a question.**
- Each of the four artifact classes above is reachable from its component owner,
  and the audit says by which link. A link that only works from `.grove/` is a
  failure.
- Every model file's tool version, bounds or trace limits, solver/backend,
  fairness assumptions, abstractions, deliberate omissions, and *what a green
  run does not prove* are present in the owning README as findable sections —
  or the gap is recorded there with a pointer to the entry that carries the
  material.
- `models/README.md` carries the phase's whole-repository run line.
- The parent's `Done when` is checked item by item against the subtree, and any
  item still live is either landed or promoted upward with a reason — because
  the node retires implicitly when no live leaf remains, and anything still live
  in the brief is promoted then.

## Notes

This leaf lands documentation and links. It decides no catalogue question and
inserts no implementation leaf; if the audit finds one owed, it goes to the
sibling that owns it — or, if every sibling has retired, to a new leaf cut here
with the finding written into its body.

`TODO.finish_process.md` is `finish-verdicts-k65`'s to dispose. If it is still
present when this leaf runs, that is a finding about `k65`, not work to absorb.

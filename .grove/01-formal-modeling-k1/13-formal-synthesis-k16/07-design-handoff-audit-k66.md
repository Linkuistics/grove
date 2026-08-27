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

## Noticed by `routing-and-prose-k73`, and handed here because this leaf owns reachability

**One broken relative link in the durable set, and it is the whole class.**
`docs/formalism-findings.md:3474` writes
`[bulk-marks-are-not-atomic](../adr/bulk-marks-are-not-atomic.md)`, which
resolves to `adr/…` from a file already inside `docs/` — one `../` too many.
Pre-existing, not introduced by any disposition child, and **not fixed there**
because that file is a log those children append to rather than edit.

It is enumerated rather than reported as a single instance: every `](../…)` and
`](./…)` link in the 50 durable markdown, `.als` and `.qnt` files under `docs/`,
`models/`, `crates/`, plus `CONTEXT.md`, `CONTEXT-MAP.md` and `README.md`, was
resolved against the filesystem, and **this is the only one that does not
exist**. So the `Done when` item *every durable formal artifact is reachable
from its component owner* has exactly one link-level defect outstanding, and the
sweep that establishes it is one page of Python worth re-running here rather
than trusting.

**Strip the `#Lnnn` fragment before resolving.** The first run of that sweep
reported 28 broken links and 25 were `path#L1234` citations in a retired review
leaf whose paths are fine — a checker that resolves the fragment as part of the
path reports a clean tree as broken, which is the same instrument failure in the
opposite direction.

## Routed here by `finish-scope-k71` — a fifth product-facing question, and it is about the reaper rather than a diagnostic

The four you already carry are diagnostic-naming questions. **This one is a
behavioural gap and it was found by reading the shipped code against the
catalogue rather than by any model run**, so it arrives with evidence rather than
as a suspicion.

**`FN-22` requires four revalidation points and the fourth runs after the
quarantine rename.** In the transaction it does: `proof.revalidate()` runs after
`cleanup.handoff()` and a failure calls `cleanup.restore()`
(`src/finish_transaction.rs:1949-1969`). **On the crash path it does not.** After
an interruption there is no in-tree cleanup owner, so
`finish_cleanup::reap_orphaned` matches the marker, finds no live owner, and
calls `cleanup.dispose()` — **it never re-reads the disposition**
(`src/finish_cleanup/reaper.rs`). The same is true on the driver's best-effort
reap-failure path: `transition_driver_to_current` warns and classifies anyway
(`src/tree_lifecycle.rs:85-96`).

**Why it is yours and not the catalogue's.** `finish-scope-k71` landed
`Reserved(Quarantined)` and the classification reorder, so the **contract** now
says the post-rename disk is a reserved state and not `Absent`, which is what
`SY-05.b` needs. What the contract cannot say is what a *sweep* outside a
transaction owes: `FN-21` charters the reaper as marker-guarded and bounded to
Grove's own, and `FN-21.a`'s re-enterability is about resuming disposal, not
about re-deciding whether disposal should happen. Whether the shipped reaper
should re-read the disposition before disposing — or whether the marker's
existence is itself the settled decision, which is the defensible reading and
probably the intended one — is a product decision on the evidence, which is your
charter's own wording.

**Two things to weigh, both stated so you can disagree with them.** In favour of
the incumbent: the marker is written only after the fourth revalidation returned
`Committed` in the transaction that wrote it, so its existence *is* a record of
that decision, and re-reading would make the reaper need a repository read it
currently does not take. Against: `FN-20` says no artifact a transaction leaves
behind is a receipt for it, and the cleanup marker is such an artifact — so
treating its existence as the settled disposition is exactly the inference
`FN-20` forbids, at one remove. `finish-scope-k71` did not resolve that and
deliberately did not try: it is a claim about the shipped protocol's own
evidence, which is what this leaf audits.

# closed-sets-k69 — brief


## Goal

Freeze the catalogue's shared vocabulary — the closed outcome, refusal-reason
and blocked-diagnosis sets, and the two outcomes that turn on them — so the three
scope children after this one decide against a settled set rather than against
each other. Land every cross-scope item that belongs to no single scope with it.

## Context

**Why this runs first.** The closed sets live in §*Vocabulary*, **above** all
three claim sections, and
[`obligations-follow-context-not-artifact`](../../../../../docs/adr/obligations-follow-context-not-artifact.md)
clause 4 is exactly about such terms: a term partitioned across groups belonging
to different scopes **has no single owning scope**. So a closed-set addition is
not any one scope's to make. Three scope children each editing one closed list
would collide, and each would decide in ignorance of the other two's members —
which is the catalogue's own stated hazard, *a new member of a closed set imposes
a matching outcome on the other column*. Items 1 – 5 are five such additions
arriving from two different scopes, and items 6 – 7 are refuse-or-block questions
that cannot be answered without knowing whether the refusal set has a member for
the case.

**The pattern under all five additions is one pattern, and naming it is probably
worth more than the five decisions.** Three of them are the same shape recorded
three times in the finish scope alone — *seven preconditions against six reasons*
(entry 031), *a tracked witness with no reason* (entry 032), *a rolled-back
finish with no reason* — and the finish README says in as many words that "the
pattern is not three accidents". The lifecycle scope then produced two more.
Decide whether the catalogue's closed refusal set is **under-populated**, or
whether its *reason* vocabulary is answering the wrong question (a reason names
the *question asked*, not the *gate that refused* — which is why `Sys.why` had to
be invented twice), before deciding the five members one at a time.

**Both columns reached the rolled-back-finish gap independently**, which is the
strongest evidence on the list: `finish.als` added `RefRollbackNotCommitted` and
`finish.qnt` added `RRolledBack`, each declaring it rather than smuggling it in.

**The eleven routed items ride here because they gate this node's siblings.**
Items 26 – 30 and 36 hand work to `finish-verdicts-k65` and `handoff-audit-k66`,
which are this node's very next siblings in the `pick` walk. A route landed after
those siblings run is a route landed too late.

**The items, by class.** Numbering is the node brief's item table; the sites for
each are in the brief's per-site ledger.

*Closed-set additions — every one manifest-changing:*
1. `RRolledBack` / `RefRollbackNotCommitted` — the catalogue maps `NotCommitted`
   to *rolls back and yields `Refused`* and no closed reason names it;
   `NoTrackedDeletion` and `RootIdentityChanged` are each **false** of a
   transaction whose fingerprint was fine and whose root never moved.
2. `RConfigInvalid` — `SY-04.b` requires an invalid configuration to refuse with
   the tree byte-identical, and no closed reason names one.
3. `RGenContended` / `Stopped` — `SY-10.b`'s contended-generation **visible
   stop** is not a `Refused` (no reason names a handoff timeout; `EpochStale` is
   `SY-10.a`'s *mismatch*, a different fact) and not a `Blocked` (§*Outcomes*
   scopes blocks to a transaction stopped part-way, and `FN-25`'s two diagnoses
   are both about finish ownership). It is shipped:
   `one-live-driver-per-working-tree` says the driver "stops `blocked`" on a
   post-reap invalidation timeout.
4. `ONotEntered` — `FN-01`'s first preflight member "produces no refusal at
   all", but a total action must return something, and an absent transition
   would make `FN-01.a` true by construction and unfalsifiable.
5. `W8WitnessTracked` — `FN-13`'s witness is *a commit attempted while the
   witness is tracked, refused*, and no closed reason names a tracked witness.
   The consequence is that **an operator cannot be told from the reason alone
   that the repository, not the filesystem, is what is blocking.**

*Opposite-resolution — the sharpest items on the whole list:*
6. **`FN-13` refuse-or-block.** `finish.als` refuses, following the catalogue
   "because the catalogue is the sole input to the formal phase"; `finish.qnt`
   blocks, on the three-contexts rule. The two *documents* disagree too:
   `task-tree-transactions-fail-closed` says a tracked witness "keeps the witness
   unwalkable as **Recovery pending**" — a `Blocked` — and the catalogue says
   refused. Note the coupling: resolving 6 as *block* may retire item 5.
7. **`FN-10.b` / `FN-32` refuse-or-block.** `finish.als` refuses where
   `finish.qnt` blocks at the same step, on an unclassifiable artifact at a
   reserved name met inside a transaction, and **both are green** against text
   that says only *fails closed*. `FN-32` states only what both agree on. The
   catalogue's *one artifact, three contexts* table now has a second row that
   deliberately fixes no outcome — that row is where this answer lands. Quint's
   `wit_FN_32` is worded on the blocking half and moves with the answer.

*Decided here because they belong to no scope:*
8. Whether `reap` is gated on the root's classification. The two columns
   resolved it in opposite directions, and **the product settles half of it**:
   `src/loop_driver.rs` reads no root classification on the reap path, so the
   catalogue gap is real and **no product defect stands behind it**.
19. `EN-11`'s controls row mis-attributes `TT-24.b`, whose dependency is
   `EN-13`. `obligation-placement-k63` edited that row for the `TT-24.d`
   retirement and left this, so **the row has now been wrong twice** — which is
   worth more than the one-word fix. Catalogue line 661.
25. The catalogue's *`models/run.sh` is the one repository runner* section states
   **three** obligations; its own Q4 paragraph makes a fourth (the removal
   matrix) and `models/run.sh`'s header numbers four. Verified against the
   runner itself.

*Routed — land the re-pointing, decide nothing:* 26, 27, 30 to
`finish-verdicts-k65`; 28 to `finish-scope-k71`; 29 and 36 to
`handoff-audit-k66`; 31 – 35 to the model owners, in the model files and READMEs
where they sit.

## Done when

- Items 1 – 8, 19 and 25 are each decided and landed in
  `docs/specs/semantic-contract.md`, each marked manifest-neutral or
  manifest-changing at the moment it is decided.
- Items 26 – 36 no longer name `formal-synthesis-k16`: each site names the
  sibling or the model owner that actually owes it.
- For every manifest-changing disposition, **both** families answer the new or
  changed obligation with a property command plus its required witnesses, or
  with that family's own declared gap, and **every scope the decision reaches**
  is green with coverage asserted under
  `models/run.sh --scope <scope> --family <family>`. A closed set is swept in
  more than one scope, so expect that to be all three; a run line is recorded
  for each.
- The model READMEs and model files that *declared* an addition rather than
  making it — `finish.als`, `finish.qnt`, `lifecycle.als`, `lifecycle.qnt` and
  their three READMEs — say in place how it was disposed, so a later reader
  meeting the declaration finds the decision beside it.

## Decomposition

Two children, and the seam is the manifest itself — which is the seam this
leaf's own `Notes` predicted before either half was attempted.

1. **`routing-and-prose-k73`** — everything that costs no model work: items 8,
   19 and 25 decided and landed, and items 26 – 36 re-pointed from this node to
   the sibling or model owner that actually owes each. **First, because it is
   what unblocks this node's siblings**: `finish-verdicts-k65` and
   `handoff-audit-k66` are the next two entries in the `pick` walk after this
   whole subtree, and a route landed after they run is a route landed too late.
2. **`closed-set-additions-k74`** — items 1 – 7: the five closed-set additions
   and the two refuse-or-block questions that turn on them. Every one is
   manifest-changing, and together they are model work in **both** families
   across the finish and lifecycle scopes plus coverage-asserted runs in every
   scope the decisions reach.

**The split is by cost and it is not arbitrary.** A manifest-neutral edit is
verified by `models/run.sh --list` still printing the same obligation count — a
sub-second check. A manifest-changing one opens an empty `(family, obligation)`
cell that both families must fill before any coverage-asserting run is green
again, against an Alloy finish cell measured at 14 m 33 s for 180 commands. The
two halves need different evidence, different reading, and different session
budgets, and mixing them is what makes a session that cannot say whether it is
finished.

**Nothing in child 1 constrains child 2's answers.** Items 8, 19 and 25 touch
the `Actions` table, the `EN-11` assumption row and the runner's obligation
list; items 1 – 7 touch §*Outcomes*' closed sets. They are disjoint regions of
the catalogue, so child 2 opens on a document child 1 left settled rather than
half-moved.

## Notes

**Expect this leaf to decompose too, and the seam if it does is the manifest.**
The three manifest-neutral groups (8, 19, 25 and the routes) cost no model work
at all; items 1 – 7 cost model work in two families across up to three scopes.
That is a real seam and not a tidy one — but cut it only when the work in hand
proves bigger than one session, and do the neutral half first, because it is what
unblocks this node's siblings.

**A disposition is a decision about the contract, so the ADR test applies**
(`content/ADR-FORMAT.md`), and the closed-set additions are the items on the
whole disposition list most likely to earn a record: a later reader will want the
**cost** of widening a closed set — a matching outcome imposed on every column
that sweeps it — and not only the outcome. If the pattern paragraph above
resolves into a rule about what a refusal reason names, that rule is one record
rather than five.

**`docs/formalism-findings.md` is a log and is not revised by this leaf** beyond
recording an outcome in place where an entry named one as owed.

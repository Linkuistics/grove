# skill-loading-replan-k80

## Goal

Reconcile the briefs and the live leaves of the tree with the decision that
Ravel reads local sources only. Grow the implementation leaves of
ravel-local-sources-k78 and owner-pack-checkouts-k79.

## Context

- The root brief records the decision of ravel-git-acquire-k77. Its paragraph
  under "Ownership boundaries", "Ravel", supersedes each pinned-Git requirement
  in a brief below it, until this leaf reconciles that brief.
- These briefs still state pinned Git sources, offline pins or the temporary
  Git refusal:
  - `common-skill-loading-k3`
  - `ravel-local-delivery-k23`
  - `grove-dual-ravel-k27`
  - `common-loading-acceptance-k29`
  - `external-research-pack-k4`
- ravel-pinned-delivery-k25 is closed. The human rejected its path, and its
  remaining leaves are abandoned.

## Done when

- No brief or live leaf requires Ravel to fetch, install or update a source.
- Each node brief states its Done when for local sources and for the
  checkout mechanism of owner-pack-checkouts-k79.
- Implementation leaves exist for the Ravel removal and the checkout
  mechanism, in an order that keeps each repository green.

## Notes

Use the committed outputs of k78 and k79. Do not repeat their design work.
owner-pack-checkouts-k82 integrated the review of k79. Take the root-brief
text from the handoff in k82, not from k79. That handoff also lists the
obligations that the integration added.

owner-pack-checkouts-k84 integrated the second review. Its handoff adds
obligations to the list in k82 and replaces k82's item about the
continuation. The focused review owner-pack-checkouts-k85 reads that repair
before this leaf. Read any integration it cuts before this leaf too.

owner-pack-checkouts-k86 integrated the third review. Its in-session reviewer
then found defects that need a new recovery model for the switch. The design
leaf owner-switch-recovery-k87 and its review now come before this leaf.
Take the switch's resumption, rollback and rehearsal contract from that
chain, not from the handoffs of k84 or k86. owner-switch-recovery-k89
integrated the review of k87. Its handoff amends the obligations in the
handoff of k87.

## Decisions (running log)

**Base.** On 2026-11-03 this checkout was at `1fab5b7a882`, the commit of
owner-switch-recovery-k89. The Ravel bookmark `skill-directory-delivery` was
at `7a894c782037`, and Ravel `main` was `24a545236185`.

**Ravel removal.** ravel-local-sources-k90 is one `impl` leaf before
grove-dual-ravel-k27. Its work list is the nine-item "Implementation handoff"
of Ravel's design evidence. The removal and the settled refusal are one
working increment: neither half leaves the build matching the specification.
The leaf rebases at entry, as each earlier stage did. k27 consumes its build.

**Switch program.** The k87 handoff, as k89 amended it, gives this leaf
obligations for a program with no dependency on the pack moves. Two `impl`
leaves now open discovery-rollout-k6. owner-switch-program-k91 holds the
switch, the automatic inverse, the state reader and the rehearsal task.
owner-switch-entries-k92 holds the owner's inverse, the merge entry and the
switch-state report of `mac-check`. Each is verifiable in disposable layouts,
and neither changes the real HOME. They sit in k6, which owns the owner
cutover, and not in k3, which owns the common launch path.

**Checkout mechanism.** The real-HOME checkouts at accepted pins and the
checkout status of `mac-check` are owner-pack-checkouts-k93, after
owner-ravel-skill-inventory-k38. The reviewed cutover contract puts them in
order 6, after the staging trials of order 5 accept the pins and digests. An
earlier real-HOME change would reorder that contract. k38 still grows the
drafts, the staging trials, the marketplace freeze and the rehearsal on the
real plan, and it inserts the first two before k93.

**Pack publication.** Order 2 of the cutover contract publishes each pack and
the `grove-core` candidate. The brief of repository-moves-k5 now states it,
because the moves own order 2.

**Handoff consolidation.** The k6 brief gains one "Owner-delivery handoff"
list. It names the consumer of each live item of k82, k84, k87 and k89, and
it leaves out the items that k87 replaced. The owner specification still owns
each rule.

**Brief reconciliation.** k3, k23, k27, k29, k4, k5 and k6 now state local
sources and the checkout boundary. Live leaves k28, k30, k10, k11, k38 and
k12 follow. The closed node k25 gains an "Outcome" section, so its Done when
reads as history. The root brief takes the k82 text, and its supersession
sentence is gone as the last edit.

**Verification.** A sweep for Git-acquisition words over every live brief and
leaf, outside k25, found only the owner's checkout pins, pinned Ravel build
inputs, history notes and substring matches. The same pattern found 18 and 14
hits in the k25 brief and k71, and a mutated pattern found none there. The
prose checker reports no hard finding on a changed line. The root brief is
still unconverted, as the rollout brief records, and these edits removed one
of its findings.

**Review.** No review leaf. Each new leaf restates a reviewed contract, and
k38 and k12 read this placement again before they grow work. This leaf spent
no in-session reviewer.

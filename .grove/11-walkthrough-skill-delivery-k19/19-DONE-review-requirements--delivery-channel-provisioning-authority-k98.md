# delivery-channel-provisioning-authority-k98

**Reviews:** `delivery-channel-provisioning-authority-k97`

## Goal

Try to disprove that the human-authorized closure is exact, internally
consistent, and sufficient to end the replacement behavioral-acceptance path
without weakening or fabricating evidence.

## Context

- Producer and task commit: `delivery-channel-provisioning-authority-k97`.
- Corrected delivery evidence contract: `delivery-channel-authority-k94`, with
  review findings at `delivery-channel-authority-k95` and their integration at
  `delivery-channel-authority-k96`.
- Parent authority: `acceptance-replication-authority-k76`, the root brief, and
  `walkthrough-skill-delivery-k19`.
- Rejected consumer: `paired-acceptance-campaign-k80`, whose descendants must
  remain present but terminal and whose contents must remain byte-identical.

## Done when

- Challenge whether the brief amendments visibly close behavioral acceptance
  as unmet rather than merely deleting its criteria, and whether the root and
  skill-delivery parent now say the same thing.
- Verify that `delivery-channel-authority-k94: authorization = none` remains
  exact and that no prose implies a hosted receipt, self-hosted runtime, paired
  execution, behavioral pass, causal use, or skill effect.
- Check that treating the fixture-interface and raw-frontmatter versus
  deployed-body choices as inapplicable follows from closure and leaves no
  hidden requirements choice for live work.
- Verify that every descendant of `paired-acceptance-campaign-k80` is terminal,
  that pruning changed filenames only, and that no historical evaluation,
  frozen rubric, or campaign task body changed.
- Test the agreed seams: explicit unmet wording, no live campaign descendant,
  preserved historical bytes, and this review before the parent node closes.
- Record findings with exact `path:line` evidence and severity. If any finding
  is actionable, commission an `integrate-review-requirements` sibling with the
  same bare stem at the first boundary the walk reaches next; otherwise create
  no integration leaf.
- Launch no evaluated treatment, control, scorer, or resolver context and make
  no fixes in this inspection-only session.

## Notes

The human explicitly chose closure after the hosted, self-hosted, and closure
branches were priced, then confirmed the precise brief, pruning, evidence, and
test-seam consequences. Review the resulting contract rather than reopening the
cost preference.

## Findings

Four findings. One is high and concerns whether the closure survives the finish
cycle; three are exactness defects in the amended brief wording. None disputes
the human's choice of closure, and none asks for a campaign.

### H1 — The closure's rationale, and the prohibition that protects it, exist only in files scheduled for deletion (high)

Every statement of the closure is under `.grove/`: `.grove/BRIEF.md:40-47`,
`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:27-37`, `:57-62`, `:75-78`,
and
`.grove/11-walkthrough-skill-delivery-k19/18-DONE-requirements--delivery-channel-provisioning-authority-k97.md:88-127`.
Searching `docs/`, `plugins/` and `crates/` for `final-boundary`,
`delivery-channel-authority` or `paired-acceptance` returns nothing.

Grove's retirement procedure states that `.grove/` dies at the finish cycle, and
that a prune's durable *why* — what was rejected, why, and what would reopen it —
goes to the ADR set when it clears the when-to-write bar; otherwise the mark and
the commit message suffice. The closure clears that bar on all three limbs:

- **Hard to reverse.** Eleven leaves are marked `ABANDONED`, and
  `.../18-DONE-...-k97.md:88-93` fixes the conjunct as closed "rather than pending
  or passed".
- **Surprising without context.** A shipped skill carrying a failed historical
  evaluation and no successor campaign invites the question this decision answers.
- **A real trade-off with rejected alternatives.** Three options were priced at
  `.../18-DONE-...-k97.md:95-108`, with costs and primary sources.

The fallback does not carry it. Commit `4df78dab`'s message is the single line
`delivery-channel-provisioning-authority-k97: close behavioral acceptance as
unmet`, with no body. `docs/adr/` holds seventeen records, none about acceptance
closure or delivery channels. `plugins/linkuistics/PROVENANCE.md:45-46` records
only that the historical campaign "did not establish its predeclared acceptance
rubric" and is silent on the replacement's closure — so the durable artifact a
plugin consumer actually reads leaves the acceptance question looking open rather
than closed.

The sharpest loss is not the rationale but a prohibition.
`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:75-78` states that "no future
campaign may inherit authority from the abandoned subtree or reinterpret
client-side evidence as final-boundary delivery." That is the rule
`delivery-channel-authority-k94` and its review chain were written to establish,
it binds work that does not exist yet, and it is the rule most likely to be
violated by someone who never saw it. After teardown it survives nowhere. The
abandoned subtree's own charter, which must stay byte-identical, still reads as a
live acceptance-authorized boundary
(`.grove/11-walkthrough-skill-delivery-k19/20-paired-acceptance-campaign-k80/BRIEF.md:5-6`),
so the material that invites inheritance outlives the rule that forbids it — the
campaign files are history, but the commit history does not label them as such.

Placement is unambiguous rather than a judgement call: `CONTEXT-MAP.md:28` states
that `docs/adr/` is flat and repo-wide, and grove's ADR note specifies the
minimal record — slug title, the decision in a sentence or two, the trade-off it
settles, and the alternative rejected and why. `plugins/linkuistics/PROVENANCE.md`
is the second candidate, since it already carries the adjacent historical verdict
at `:45-46` and is what a consumer of the skill reads.

Anticipated objection, stated and answered: `grove-finish` step 1 does promote
brief content at teardown, so this could in principle be caught then. But that
same procedure describes step 1 as "often a near no-op when decisions landed
inline as they were made", and the retirement procedure assigns a prune's *why*
at prune time, not at finish. Deferring a decision this size to the teardown
session is relying on the step that is designed to have nothing left to do.

This finding accepts the closure in full and disputes no cost estimate. What is
missing is a home for the closure outside a directory that is deleted by design.

### H2 — The parent brief restates as an existence claim the one thing `k94` explicitly declared was not one (medium)

`.grove/BRIEF.md:41-42`: "No **hosted** provider with the required final-boundary
attestation **was** identified".
`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:28-29`: "No provider with the
required final-boundary attestation **is** identified".

The evidence supports the root's phrasing and not the parent's.
`.../18-DONE-...-k97.md:95-97` says "No **inspected hosted** Claude/OpenAI/Codex
product documented the required final-boundary attestation", and
`.../15-DONE-requirements--delivery-channel-authority-k94.md:104-105` says "no
**inspected hosted** channel satisfies the sufficient-evidence classes". More
directly,
`.../15-DONE-requirements--delivery-channel-authority-k94.md:226-227` states: "The
provider class is a sufficient specification, **not an existence claim**: no
inspected hosted provider exposes it."

The parent's unqualified present-tense "No provider ... is identified" is exactly
the existence-shaped reading `k94` refused, and
`.../16-DONE-review-requirements--delivery-channel-authority-k95.md:522-524`
already found `authorization = none` "under-evidenced, stated with more permanence
than its author intended". The phrasing also silently swallows the self-hosted
option that the same sentence then handles separately as declined.

The direction of the error is over-claiming *absence*, not over-claiming a pass,
so it fabricates no acceptance evidence — it makes the closure look more forced
than the record does, and it is the one place where the root and the
skill-delivery parent do not say the same thing. Restoring "inspected hosted" and
matching the root's tense settles both.

### H3 — The parent's decomposition still reads as a live plan for the replacement instrument (low)

`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:53-56`:
"`supplemental-evaluation-k77` **now plans** the replacement paired acceptance
instrument, applies the relevant F1–F14 protections, and keeps discovery and
transfer work outside the primary verdict."

"now" asserts current state, and no current state plans a live instrument. The
bullet sits three above `:57-62`, which says the campaign does not run, and
`:75` already has the settled voice: "The replacement campaign was planned but not
authorized for execution." Every other closure edit in this commit moved to that
voice; this bullet was not revisited. It states no falsehood about what `k77` did,
so it is low rather than medium, but it is the one line in the amended chain that
still implies paired execution is ahead.

### H4 — The root states that the conjunct is unmet without stating the bar or citing where the bar survives (low)

`.grove/BRIEF.md:40-43` replaced the two bullets that defined the behavioral
conjunct — three surfaces, predeclared material improvement over a
contemporaneous no-skill comparator, an absolute enabled-performance floor, all
three required — with a statement that it is closed as unmet.

The replacement does close rather than delete, which is what the seam at
`.grove/BRIEF.md:91-92` asks for, so the amendment passes that test. What it
loses is the height of the bar. The root now names an unmet requirement without
saying what would have met it, and `.grove/BRIEF.md:83-94` (Pointers) carries no
citation for the closed conjunct.
`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:50-52` cites
`acceptance-replication-authority-k76`; the root cites nothing, so a reader of the
root alone cannot tell how high a bar went unmet, and reaches the definition only
by descending one level. One clause in Pointers closes it. Under H1 the same
sentence belongs in the durable record, where it matters more.

## What survived the attempt

Eight checks were run against the closure and did not break it.

- **Closure is the outcome `k94` prescribed, not an improvisation.**
  `.../15-DONE-requirements--delivery-channel-authority-k94.md:233-235`: "If the
  human declines self-hosted provisioning and cannot name a provider that meets
  the first class, a requirements decision must close the parent behavioral
  conjunct explicitly rather than leave it indefinitely pending." That is what
  `k97` did.
- **The amendment closes rather than deletes.** Both briefs replace the old
  criteria with an explicit unmet statement — `.grove/BRIEF.md:40-43` and
  `.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:27-30` — each naming the
  three priced options as the reason. Subject to H4, this holds.
- **`delivery-channel-authority-k94: authorization = none` is exact and
  untouched.** Commit `4df78dab` modifies no file other than the two briefs, the
  new review leaf, and thirteen pure renames.
  `.../15-DONE-requirements--delivery-channel-authority-k94.md:113-114` still
  carries the value verbatim, and
  `.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:30` restates it unchanged.
- **No prose implies a hosted receipt, self-hosted runtime, behavioral pass,
  causal use, or skill effect.** `.grove/BRIEF.md:44-47` and
  `.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:35-37` state the negative
  directly. `plugins/linkuistics/PROVENANCE.md:45-47` and
  `docs/evaluations/writing-code-walkthroughs/README.md:5` ("The unchanged
  acceptance rubric is **not met**") make no acceptance claim, so no existing
  artifact falsifies `.grove/BRIEF.md:44` on arrival. Only H3 touches this class,
  on paired execution alone.
- **The inapplicability of the two treatment choices follows from closure.**
  `.../18-DONE-...-k97.md:119-122` derives it: no evaluated intervention is
  authorized, so choosing either would invent an unused acceptance condition. Both
  choices are model-visible differences in a treatment that will not be delivered,
  and neither has a consumer — `acceptance-instrument-k81`, their only reader, is
  `ABANDONED`.
- **No hidden requirements choice is left for live work.** The tree holds three
  live leaves: this one, `.grove/12-review-design--book-system-k25.md`, and
  `.grove/13-impl--entry-name-discharge-contract-k32.md`. Neither of the latter
  mentions acceptance, skill, campaign, fixture, or frontmatter; they concern the
  book-system design review and `EntryName`'s type-system discharge.
- **Pruning changed filenames only, and every descendant is terminal.** All eleven
  leaves under `20-paired-acceptance-campaign-k80/` carry the `ABANDONED` infix;
  each renamed file's contents hash identically to its pre-commit version, as does
  the subtree's `BRIEF.md`. The node directory itself correctly carries no marker —
  grove never marks a node. No historical evaluation record or frozen rubric byte
  changed: `docs/evaluations/` was last touched by `skill-evaluation-report-k52`,
  well before this chain.
- **The four agreed seams are present and consistent across both briefs.**
  `.grove/BRIEF.md:91-94` and `.../18-DONE-...-k97.md:124-127` state the same four:
  explicit unmet wording, no live campaign descendant, byte-identical historical
  records, and a requirements review before the skill-delivery node closes. This
  review is the fourth, and `k98` is the last live leaf under
  `walkthrough-skill-delivery-k19`, so it does precede the node's close.

## Limits of this review

- The closure rests on a negative — that no hosted provider documents
  final-boundary attestation. This review checked the *scope discipline* of that
  claim across the chain (H2); it did not re-survey provider documentation, which
  would be re-running `k94`'s work and reopening the cost question the human
  settled.
- The cost estimates at `.../18-DONE-...-k97.md:98-102` were not independently
  verified. Under closure they buy nothing, since no spend follows either way.
- "No evaluated treatment, control, scorer, or resolver context ran" is checked
  only negatively: commit `4df78dab` touches nothing outside `.grove/`, and no new
  artifact appeared under `docs/evaluations/`. A context that ran and wrote nothing
  would leave no trace this inspection could find.
- Byte-identity of the pruned subtree was verified against the immediate parent
  commit, not against the campaign files' original authoring commits.

## Decisions (running log)

Four findings, one high and three low-to-medium, all actionable and all cheap.
H2, H3 and H4 are wording repairs inside the two amended briefs. H1 is the only
one that adds an artifact, and it adds the smallest one grove's own ADR note
defines.

The findings do not overturn the closure. The human's choice was to decline
provisioning and close the conjunct as unmet; every check that bears on whether
that choice was executed faithfully in the tree passed. H1 says the execution
stopped one artifact short of durable: `.grove/` is deleted at the finish cycle by
design, so a decision that lives only there is a decision with an expiry date, and
the rule at
`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md:75-78` forbidding a future
campaign from inheriting the abandoned subtree's authority is the piece whose loss
would matter most.

Because findings are actionable, an `integrate-review-requirements` sibling is
commissioned with the bare stem `delivery-channel-provisioning-authority`. It is
appended at the parent's end rather than inserted: the only later sibling entry is
`20-paired-acceptance-campaign-k80`, whose subtree is wholly terminal and which
therefore cannot move the line anchors above. Its body carries this review's
handle and not this finding list, so the integrating session retains a structural
place to reject any of the four.

No fixes were made and no evaluated context was launched in this session.

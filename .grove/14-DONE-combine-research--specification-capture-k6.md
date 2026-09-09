# specification-capture-k6

**Integrates:** specification-capture-k4, specification-capture-k5

## Goal

Union both surveys' coverage into one research document, flag every
disagreement between them, and perform the adversarial move neither survey can
perform on itself.

## Done when

One document covering both corpora, every disagreement named rather than
silently resolved, and a stated position on whether the two-tier hypothesis
survives the evidence.

## Notes

**The adversarial move this session owes.** Both surveys were commissioned
against a hypothesis the human supplied — that capture and formalism are two
layers, and a notation attempting both fails at both. A survey commissioned
against a hypothesis tends to find it. Ask what the evidence would look like if
the hypothesis were wrong, and say whether that pattern appears.

**Where agreement is weakest evidence.** If both surveys reached the same
sources, their agreement is one corpus reported twice. Check the corpora overlap
before crediting any concurrence.

**Name what neither found.** A question returned unanswered by both is a
finding, and the downstream design needs it stated rather than left as silence.

## Decisions (running log)

**Output path is the unadorned `docs/research/specification-capture.md`.** The
kind's own skill fixes it, and the research family file gives the precedent set
on disk — `walkthrough-method-a.md`, `-b.md`, and the combiner's
`walkthrough-method.md`. No rename of either survey; both stay where they are
and are cited from the combined document.

**The corpus overlap audit is run first and mechanically, before any agreement
is credited.** Extracting every distinct URL from each survey and intersecting:
25 in A, 41 in B, **one** shared, 65 in the union. The task file requires the
check before crediting concurrence, and the result inverts the kind's standing
suspicion — agreement here is corroboration, not one corpus reported twice. It
is reported as a measurement rather than assumed either way, because it would
have had to lower confidence by exactly as much had it come back the other way.

**Two overlaps the audit found that neither survey could see.** The one shared
source is [Agent READMEs](https://arxiv.org/abs/2511.12884), and even there the
two reports differ — A gives security 14.8% and performance 14.5% as two
figures, B collapses them to one. And both cite the *same* MDE research
programme through different papers, established from the documents themselves
by the shared 74%-training figure rather than asserted from authorship. That
second one matters: A carries the 44%/31% split out of it as the sole
quantitative support for "fails at both", while B carries out a two-thirds
positive on stakeholder understanding, and neither reports the other's figure.

**The adversarial move is A's own reformulation, tested against B's corpus.**
A concludes the tier boundary belongs at the *audience* boundary. Survey A
contains zero occurrences of EARS, Attempto, or controlled natural language —
verified by word-boundary grep, since a case-insensitive `ears` matches "years"
and "appears" and reads as eight false hits. EARS is two populations, one
notation, ten years, and it falsifies A's rule. This is the correlated blind
spot the pair was bought to expose, and finding it is the one check neither
survey could run on itself.

**The mechanism reported is arbitration, and it is this document's own claim
rather than either survey's.** It explains all four decisive cases including
EARS, which A's audience rule does not and which readability explains least of
all. Its unwelcome consequence is stated rather than softened: the proposal
under examination — a DSL or a pseudo-DSL paired with LLM skills — fails on
both horns, checked (arbitrated, Gherkin's outcome) and unchecked (prose with
an appearance of formality). The escape the union supports is to move the check
onto the projection, which is B's one-way authorship composed with A's
critical-path requirement.

**The hypothesis's second clause is reported as unevidenced.** Neither survey
reports a source measuring whether a formal tier improves an LLM's output. Both
were commissioned to test the two-layer *conclusion*, so neither tested its
premises, and this is the highest-value silence in the union. Phrased as a
claim about what the two surveys report, not about all 65 sources — this
session fetched nothing and cannot speak for a source it did not read.

**No new primary source was fetched, and the document says so.** The
adversarial move this leaf owes is structural — an overlap audit, a
counterexample from one corpus applied to the other's conclusion, and an
enumeration of what both missed — and needs no third search. A's two paywalled
DSL targets are carried forward as next targets rather than quietly dropped;
B's three reachable DSL-evolution studies *partly* close that gap, establishing
ecosystem cost but not benefit exhaustion, so A's suspicion is recorded as
replaced by a better-specified one rather than as answered.

**Uniqueness claims were enumerated before being written, and one was wrong.**
"EARS is the only two-population artifact that held" fails: ubiquitous language
and ADRs are two more. The corrected sentence — EARS is the only two-population
survivor with a claim to machine-checkability — is narrower, true, and
strengthens the arbitration reading, since the other two have no enforcer
either. Three further over-claims were tightened the same way: a universal
negative over 65 unread sources, a superlative attached to the 31% figure alone
rather than to A's 44%/31% pair, and a four-case table that read as an
enumeration of the corpus rather than of the decisive cases.

**No ADR, and the bridge is left open with its obligation named.**
`ADR-FORMAT.md` gives an AND test — hard to reverse, surprising without
context, a real trade-off — and gives the bridge to *adopted* findings. This
session reports and adopts nothing, and neither precedent survey carries a
bridge. The document therefore carries an *Adopted findings and the ADR bridge*
section that is deliberately empty and names the four findings most likely to
earn a record, each with its rejected alternative already stated, so the design
session that adopts one can write it without re-deriving the trade-off.

**The link sweep was given a positive control before its green was credited.**
`every_repository_markdown_reference_resolves` passes, but the combined document
contains only two relative links, so a pass could be vacuous. A deliberate
broken link turned it red naming
`docs/research/specification-capture.md:693`; the link was removed and the file
restored to 691 lines, and the suite is green again. Also checked what a new
file under `docs/` could falsify: no count of research documents exists
anywhere in the repo, `docs/ARCHITECTURE.md`'s ownership table is tested for
*book roots* only and no research document has a row, and the ADR set's "kept
seven documents under `docs/`" is a record of what `delete-formal-models-k29`
did, not a current-state count.

**The root brief is amended, because leaving it would mislead the next leaf.**
Its note ends "Nothing downstream of P4 may cite the two-layer form as settled
before k6 reports" — which, once k6 reports, reads as permission to cite the
categorical form that k6 refutes. `design--loop-construct-k7` is the next leaf
in the walk and reads this chain. The `Done when` bullet gains a **Delivered**
annotation in the file's established idiom, and the note gains a paragraph
stating what survives, what is refuted, and what stays unevidenced, pointing at
the document rather than summarising it. Written as a backticked path, not a
Markdown link: `.grove/` is in the sweep's `UNSWEPT_DIRECTORIES`, so a link
there is checked by nothing, and the brief uses backticked paths everywhere
else.

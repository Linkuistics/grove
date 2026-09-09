# specification-capture-k4

## Goal

Survey how human intent — requirements, specification, guidance, feedback — is
captured in a form that stakeholders without formal training can read, share and
contribute to, **and** elaborated into something formal enough for an LLM to
build against. Report structured around the downstream questions below.

## Context

Commissioned by `plan-k1`, decision 16. The governing constraint, stated by the
human: formal specifications are poor human artifacts and good LLM artifacts, so
the capture layer and the formal tier are two layers rather than one notation. A
format attempting both is expected to fail at both — test that expectation
rather than assuming it.

The proposal under examination is a DSL, or a pseudo-DSL that does not
syntactically restrict, paired with LLM skills that help design the specification
documents, and a structured and genuinely well-made tooling story.

## Done when

Each question below has an answer with citations, or is reported as unanswered
by the literature. Findings are structured by question, not by source.

## Notes

**The downstream consumers, by subject.**

- The session designing the capture format needs to know what has been tried and
  what decayed in real use.
- The session designing the loop construct needs to know how iterative human
  feedback has been persisted across rounds without becoming a changelog.
- Every book-authoring session needs to know what a structure brief should
  contain to be worth writing.

**The questions.**

1. What notations have been used to capture requirements for a mixed audience of
   technical and non-technical stakeholders, and which of them survived years of
   real use? Domain-driven design's ubiquitous language, event storming, example
   mapping, specification by example, user story mapping, C4 and its DSL, and
   architecture decision records are the obvious candidates.
2. What went wrong after adoption? Behaviour-driven-development notations
   decaying into brittle test scripts, model-driven approaches collapsing under
   round-trip cost, and DSL maintenance outliving DSL benefit are the named
   suspicions; find the post-mortems rather than the introductions.
3. Which two-tier architectures exist — a human-facing capture layer elaborated
   into a formal one — and what does the elaboration cost when the two drift?
4. What makes a glossary *shareable*? Specifically: is a ubiquitous language
   that also carries rationale, history and reopen conditions still usable by a
   stakeholder, or are those two artifacts that must be separated or projected?
5. How is human guidance made *persistent* across many LLM sessions, rather than
   re-supplied each time? What has been tried, and what decays?
6. What does good tooling for this look like, and is there any evidence that
   presentation quality changes stakeholder participation?

**Bias the search toward failure.** "What has been tried" returns broad shallow
surveys; "what went wrong after years of real use" returns post-mortems, and
those are what the downstream design needs. The fallback to beat is the
status quo — prose documents plus a glossary — so keep the survey off it rather
than on it.

**Independence is the point of the pair.** Do not read the sibling survey's
output, and do not converge on its corpus. `specification-capture-k6` performs
the reconciliation.

## Decisions (running log)

**Output path is `docs/research/specification-capture-a.md`.** The research
family file fixes it by kind rather than by whether a sibling exists:
`research-a` writes `-a.md` even when solo, and a later `-b` renames nothing.
Precedent on disk is uniform — `docs/research/walkthrough-method-a.md`,
`-b.md`, and the combiner's `walkthrough-method.md`.

**Corpus discipline: primary sources fetched, not summarised second-hand.**
Search-result prose about a post-mortem is not the post-mortem. Where a claim
about a tool's failure is load-bearing I fetch the author's own text or the
paper's own figures, and where only secondary commentary exists I say so in the
citation. This is the task file's "find the post-mortems rather than the
introductions" applied to the fetching step, not just the search step.

**Structure follows the precedent survey's shape**, which already satisfies
this family's three obligations: a *Scope and confidence* opening, one numbered
section per commissioned question, a *Walk-away checks* section discharging the
per-system invariant, and a closing *Search silence and limits*. Findings are
grouped by question, never by source, as the task file's "Done when" requires.

**"Two-tier" names two different seams, and the survey separates them.** The
brief's hypothesis is about the *human ↔ formal* seam: a capture layer
stakeholders share, elaborated into something an LLM consumes. Most of the
industrial two-tier literature is about a different seam — *formal ↔ code* —
and the two have opposite published verdicts. Newcombe et al. keep prose and
TLA+ apart and accept an unverified gap to code; Bornholt et al. deliberately
collapse the formal tier *into* the implementation language to stop separate
specs "languishing as separate expert-written artifacts". Reading the second as
evidence against the grove's hypothesis would be a category error, and it is the
easiest one available in this corpus. Question 3 answers for both seams and says
which is which.

**The hypothesis survives at its own seam, for a different reason than it
gives.** The two-layer position holds, but the mechanism the evidence shows is
*arbitration*, not readability: where one artifact carries both a machine
obligation and a human one, only the machine obligation has an automated
enforcer, so it wins every conflict silently. A BDD practitioner states it
outright — refactoring scenarios to remove duplication "causes the requirements
and their understanding to change from what the Product Owner wants". That is a
stronger argument than readability because it survives the objection that
stakeholders could simply learn the notation. Restated: **the tier boundary
belongs where the audience boundary is, not where the formality boundary is** —
which explains both ShardStore's fusion (one population owns both purposes) and
Gherkin's failure (two populations do).

**The walk-away check is the survey's best predictor, so it is reported as a
table rather than a per-system aside.** Scoring all eleven prior systems on
"with the tool uninstalled, what is still legible?" reproduced the survival
ordering exactly: every artifact still maintained years later scores Full,
every Nil was abandoned or became a liability, and the Partials are precisely
the formats whose two halves came apart. No other property in the corpus tracks
survival that closely, which is why §7 is a table and not prose.

**Question 6's second half is reported as unanswered, and only after searching
for it directly.** No study was found relating presentation or tooling quality
to stakeholder *participation*; the adjacent literature measures comprehension,
decision quality or satisfaction instead. Recorded as a silence rather than
answered from the comprehension studies, because substituting one for the other
is the error the question was written to avoid. METR's 39-point perception gap
is attached there, since it means any in-house answer must be instrumented
behaviourally rather than surveyed.

**Two candidate DSL post-mortems were unreachable and are named, not cited.**
Hermans/Pinzger/van Deursen and Kosar et al. sit behind 403s and paywalls, so
the "DSL maintenance outliving DSL benefit" suspicion is reported as
unsubstantiated-from-primary-sources with those two named as the next targets.
Citing the secondary summaries as if they were the studies would have converted
a known gap into a false finding.

**Unverified locators were removed rather than reproduced from memory.** Five
citations initially carried a DOI or author list I had not seen in a fetched
source (Moody, two Kalyuga papers, Bano & Zowghi, He & King, and the
LLM-revision study's authors). Each is now either linked to the copy actually
read, cited at one remove through the review that summarises it, or cited by
title — and Moody's article says explicitly that it was *not* read and that the
principles come from a lecture deck.

**The repository link sweep was given a positive control before its green was
credited.** `every_repository_markdown_reference_resolves` passes, but the new
document contains no relative links, so a pass proves nothing on its own. A
deliberate broken link appended to the file turned the test red naming
`docs/research/specification-capture-a.md:903`, which establishes the sweep
reads the file; the link was then removed and the file restored to 901 lines.
Also enumerated every reference to `docs/research` across the repo: all are to
one named file or to the directory as a location, so adding a survey moves no
stated count, and the ADR bridge in `ADR-FORMAT.md` is owed by *adopted*
findings — neither precedent survey carries one, and k6 has not reported.

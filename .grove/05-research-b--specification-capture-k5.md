# specification-capture-k5

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

# pipeline-kinds-k72

**Reviews:** pipeline-kinds-k27

## Goal

An adversarial read of the pipeline extraction: the two ADRs it landed, the
four disciplines it wrote into `pipeline-skills-k28`'s task file, and the shape
it gave `crate-books-k14`'s books. Findings only — no fixes, no skills authored.

## Context

- The artifacts: `docs/adr/the-editorial-pipeline-is-four-kinds.md`,
  `docs/adr/a-feedback-edge-is-forward-tree-growth.md`, the
  `## The four kinds` and `## The launch templates` sections of
  `pipeline-skills-k28`'s task file, and the pipeline paragraphs added to
  `.grove/11-crate-books-k14/BRIEF.md`. `pipeline-kinds-k27`'s own
  `## Decisions (running log)` records why each call went the way it did.
- The instrument that governs the set is
  `docs/evaluations/editorial-pipeline-pilot/preregistration.md` *Part 4*, and the
  evidence is `docs/evaluations/editorial-pipeline-pilot/README.md`. Both are
  frozen and neither is editable by anything downstream of them.
- **Nothing here has been implemented.** No skill exists yet, and the review's
  subject is a design and a work order, not shipped bytes.

## Done when

Each of these is read and answered, and every finding is written down with the
evidence that supports it.

- **Does the extraction match what the report licenses — exactly?** Not fewer,
  and above all not more. The verdict table fixes what each row permits, and an
  `Undetermined` licenses nothing; check that no stage was extracted, folded or
  dropped on a basis the report does not supply, and that nothing was smuggled in
  as a fifth kind under another name.
- **Is `draft` a kind the evidence supports, or a convenience?** It was never
  scored. It carries both merged charters and the structure-brief precondition,
  and the argument against reusing `impl` rests on those two facts. Try to break
  it.
- **Is the ADR set minimum and coherent?** Two records were written where one was
  possible. Argue the merge: do they reopen on different conditions, or is the
  feedback-edge record a section of the extraction record wearing a slug? Check
  every citation resolves and that neither record restates the other.
- **Is the feedback-edge mechanism actually performable with the verbs as they
  are?** Read `grove-llm leaf-add --help`, `leaf-insert --help` and
  `leaf-decompose --help` against the claims: that `leaf-decompose` grows exactly
  one child and needs `--kind draft`; that a lazy chain leaves nothing queued so
  a re-run is an append; that `leaf-insert` is needed only when a later sibling
  entry holds live work; that all three verbs refuse a kind with no launch
  template. A claim about the CLI that the CLI does not honour is the most
  expensive defect available here, because four skills are authored from it.
- **Does the filing survive `corpus-rules-have-one-owner.md`?** Run its placement
  function over each rule in the work order. Is `Occasion` honest for each? Does
  any rule appear in both the family file and a kind skill? Is the claim that the
  spine's `SKILL.md` gains nothing right under
  `restatement-declares-its-class.md`, or does some rule's load predicate fire
  from `SKILL.md` and so require a `trigger` sentence after all?
- **Are the disciplines document-general, or do they assume a walkthrough book?**
  The tokens are unprefixed and the skills install everywhere. Find every sentence
  that would be false, meaningless or unreachable in a repository with no
  `docs/specs/walkthrough-books.md`, no fragment validator and no `book-check`.
- **Two rules rest on judgement rather than evidence.** Mandatory stage
  membership — a stage may not be skipped because the previous session judged it
  would find nothing — and the one-re-run escalation bound. Neither was measured.
  Are they defensible, over-reach, or under-specified at the boundary?
- **What did the design fail to say?** The report names a limit the extraction has
  to live with (the fold is conditional on the structure brief) and a conflict
  no stage could close (`01-orientation.md#public-surface`, which belongs to
  whoever next revises a structure brief). Is each one carried somewhere a session
  will meet it, or has it been left in a frozen evaluation document that binds
  nothing?

## Notes

**Position matters here and is why this was inserted rather than appended.**
`pipeline-skills-k28` authors the skills from this design, so the walk must reach
this review — and any integration it cuts — before that leaf runs. An integration
cut here goes ahead of `pipeline-skills-k28` for the same reason.

**Do not author a skill, and do not fix a discipline you disagree with.** The
output is findings. If a charter cannot be written as a skill, that is the most
valuable finding on the list, and it is still a finding.

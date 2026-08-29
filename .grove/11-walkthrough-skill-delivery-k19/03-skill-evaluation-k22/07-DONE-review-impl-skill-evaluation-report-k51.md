# skill-evaluation-report-k51

**Reviews:** skill-evaluation-report-k49

## Goal

Adversarially audit the durable `writing-code-walkthroughs` evaluation report
against the frozen rubric and retained evidence after the producer's in-session
doubt corrections.

## Context

- Producer artifact:
  `docs/evaluations/writing-code-walkthroughs/README.md`.
- Frozen rubric and retained case records under
  `docs/evaluations/writing-code-walkthroughs/`.
- Review the committed `skill-evaluation-report-k49` diff rather than inheriting
  the producer's conclusions.

## Done when

- Every aggregate, per-behavior count, range, and materiality statement is
  checked against the retained evidence.
- The report does not attribute behavior to skill wording when discovery or
  skill-body use is indeterminate.
- Historical harness digest guards remain consistent with executed campaign
  manifests, while final deployment checks apply to the refined skill bytes.
- Claims about judged outputs, judgment-dependent limitations, and mechanical
  verification remain distinct.
- Findings are recorded without editing the producer artifact.

## Notes

Try to disprove the report. Pay particular attention to its `not met` versus
`undefined` terminology, unequal-denominator aggregates, Case C preservation
rows versus the frozen Case A regression guard, and the reproducibility boundary
between historical evidence and final-skill deployment checks.

## Findings

Seven findings against `docs/evaluations/writing-code-walkthroughs/README.md` as
committed by `skill-evaluation-report-k49` (jj change `twxrszsu`, commit
`338198872cf90648430719ace60dedc7e23dd39a`), most severe first. Bare `:NN` line
references are to that file at that commit; every other path is named in full.

**The mechanical layer was checked first and is clean.** Every number in the
report reproduces from the retained evidence:

- the executed rubric digest at `:36-37` equals
  `shasum -a 256 docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`,
  and `jj log -r 'files(...)'` shows that file untouched since its
  `baseline-rubric-k44` freeze, so "byte-unchanged" at `:33` holds;
- `|R| = 15`, `ceil(2 × 15 / 3) = 10` and `G = {A02, A03, A14, A15}` at `:39-45`
  match `baseline/README.md:169-174` and the rubric's own definitions;
- every Case A cell at `:81-101` reproduces from
  `enabled/scope-elicitation/adjudication/primary.md` scored against
  `mapping.tsv`; the arm totals are `30/105` control and `25/84` enabled, with
  per-sample totals `{6,5,6,7,6}` and `{7,6,5,7}` — so the means `6.0` and
  `6.25` and the ranges `5–7` at `:61-63` are right, as are the historical
  `5.6/21` and `5–6` from `{6,6,5,6,5}` in
  `baseline/scope-elicitation/repetition-*.md`;
- every Case C cell at `:130-153` reproduces from
  `enabled/exposition-assurance/adjudication/resolved.tsv` (`45/120` both arms,
  per-sample `{8,9,9,9,10}` control and `{8,8,9,9,11}` enabled) and, for the two
  refinement columns, from re-resolving `refinement-regression/adjudication/`
  `scorer-3.md` and `scorer-4.md` under the six rulings in `resolution.md`
  (`55/120` control, `57/120` refined, both ranges `10–13`) — the means `9.0`,
  `11.0`, `11.4` and the `+2`/`-2` differences at `:65-69` and `:155-156` all
  follow;
- the transfer probe's `16` non-compliance rows and `at least 8` threshold at
  `:171-172` match `enabled/transfer-probe/criteria.md` (20 rows, `T01`–`T04`
  compliance controls, and its own `:35`), whose digest matches the record;
  `21` attempts and `20` boundary violations match that record's accounting;
- the final skill digest at `:202-203` equals
  `shasum -a 256 plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md`
  and the 13,940 bytes in `refinement-regression/skill.manifest.tsv`, and
  `jj diff -r rvrvkxvk` confirms the single wording change the report describes
  at `:157-158`;
- all seven relative links in the report resolve, and
  `tests/reference_navigation.rs:303` does not exclude `docs/evaluations`, so the
  check cited at `:213` genuinely covers this file;
- `plugins/install.test.sh` reaches `record` exactly 20 times (9 directly, 11
  through `check`), so the `20 passed, 0 failed` at `:209` is the complete
  assertion set with `jj` present rather than a partial run.

The report is also careful where it would have been easiest not to be: it never
attributes a count to skill wording (`:25-29`, `:158-161`), it separates the
seven Case C preservation rows from the frozen Case A guard (`:162-164`), it
declines partial-sample arithmetic symmetrically — refusing the favourable `A06`
`4/4` and the adverse `A14` `2/4` alike — and its "not met" verdict is the
correct reading of a rubric that says the campaign "meets the
material-improvement requirement **only when**" a threshold is reached, which
the rubric itself confirms by ruling an empty `R` "unmet rather than redefined".
The findings below are what those checks cannot see.

### 1. `A14`'s regression-guard breach is determinable, and the report calls all four guards unevaluable

`:223-225` states that "the primary `10/15` material-improvement threshold and
all four regression guards remain unevaluable because Case A lacks a valid fifth
enabled sample and has a protocol-breached selected control". The justification
the report gives for that blanket treatment is at `:47-49`: a missing sample
"prevents the corresponding comparison rather than supplying a zero".

That justification does not reach `A14`. The rubric's guard
(`baseline/rubric.md:178-181`) requires every member of `G` to "lose at most one
success relative to **both** its historical and contemporaneous control counts".
`A14`'s historical count is `5/5` from a complete, unbreached arm, and the
enabled arm stands at `2/4` (`:94`). The missing fifth sample can only take one
of two values, and both violate the guard: a success yields `3/5`, a loss of
two; a failure yields `2/5`, a loss of three. No zero is supplied and no
comparison is invented — the bound holds over every completion, and it holds on
the historical conjunct alone, which the control's prompt-byte breach does not
touch. `A14` breaches the regression guard under every possible completion of
the campaign.

The same interval reasoning settles the primary endpoint in the report's favour,
and the report gives that away too. Thirteen of the fifteen `R` rows sit at `0/4`
(`:85`, `:87-93`, `:96`, `:98-101`), so each can reach at most `1/5`, which
cannot exceed a `0/5` historical count by the required two. At most `A06` and
`A17` could improve materially, so the `10 of 15` threshold is unreachable under
every completion — the acceptance verdict is determinably not met, not merely
undefined. Stated that way the verdict survives the obvious objection to `:5-11`,
that "undefined" is being reported as "not met".

As written, the blanket "unevaluable" is unsupported in the one place it shields
the artifact from an adverse determinable result, and understates the report's
own negative conclusion in the place it would have strengthened it.

Recommended fix: replace the blanket sentence at `:223-225` with the two
determinable results — `A14` breaches the regression guard under every
completion, and the `10/15` endpoint is unreachable under every completion —
keeping "unevaluable" for `A02`, `A03`, `A15`, `A06` and `A17`, where the
missing sample or the breached control genuinely decides the outcome. Add the
`A14` result to `:94`'s interpretation cell and to the `Verdict`. If the intended
frozen rule is stronger than `:47-49` says — that a shortfall voids the arm
outright, bounds included — state that rule explicitly instead, because the
report currently asserts the weaker one and then applies the stronger one.

### 2. Case A's invalidation rule is treatment-correlated, so the retained enabled sample is selected against skill use rather than merely incomplete

`:103-106` records that "seven enabled attempts visibly read `SKILL.md` and were
invalid because Case A forbids every tool call", and that "the four valid enabled
streams contain no observable skill-file read or use", concluding that discovery
and body use are "indeterminate". `:231-232` names the underlying conflict: "the
frozen no-tool prompts conflict with observable file-based skill loading."

Both statements stop one step short of the consequence. The invalidation rule can
only fire on the enabled arm — a control home has no `SKILL.md` to read — so it
is correlated with the treatment. Every enabled attempt that visibly used the
skill was discarded, and the four survivors are exactly the attempts that did
not. The retained enabled sample is therefore *conditioned on the absence of
visible skill use*, which is a selection effect on the arm, not a shortfall in
it. Case B is the same mechanism run to completion: all three enabled attempts
said they intended to read skill instructions and all three were invalidated
(`enabled/source-fragments/README.md:13-18`, `:86-92`), leaving that arm empty.

This matters in three places the report does not connect. It is the reason
`A06`'s `4/4` cannot be read as a skill effect even if the fifth sample had
landed — the sample is drawn from the sub-population where the skill was not
visibly used. It is the reason the enabled Case A counts cannot be described as
"the enabled arm" without qualification, as `:55` and `:79-101` do. And it means
that rerunning Case A under the frozen prompt cannot fix the shortfall, because
the rule that destroyed repetition 5 will keep firing: the gap at `:231-232` is
not merely unclosed but unclosable without changing the frozen prompt or the
access rule, which is exactly the "new predeclared evaluation cycle" `:238-239`
alludes to without saying why one is unavoidable.

Recommended fix: state the selection effect once where the enabled Case A sample
is introduced (near `:103-106`), and carry it into `:231-232` as the reason the
Case A conflict is structural rather than incidental. No count changes.

### 3. The refinement scorers did not agree on every regression decision, and the judgment-dependence paragraph omits two arms

`:196-198` reads: "The two valid refinement scorers agreed on every regression
decision and disagreed on one target `C23` decision."

Diffing `refinement-regression/adjudication/scorer-3.md` against `scorer-4.md`
gives six disagreements out of 240 decisions: `S02 C06`, `S02 C09`, `S05 C03`,
`S06 C02`, `S08 C23`, `S10 C03` — the same six `resolution.md:8-27` resolves.
`S02 C09` is a decision on `C09`, one of the seven preservation rows the report
names two paragraphs earlier at `:162-163` and reports as "remained `5/5`". The
scorers split `1`/`0` on it, and the `5/5` control count at `:138` exists only
because the frozen-rule resolution went to `1`. So the claim is contradicted by
the retained evidence under the only reading that gives "regression decision"
content; under the alternative reading — the frozen Case A guard rows — it is
vacuously true of a Case C rerun that scored no Case A criterion, which the
report itself insists is a different set (`:163-164`). The upstream record
carries the same error (`refinement-regression/README.md:107-108` and
`adjudication/resolution.md:29-31`, which asserts agreement "on all target and
regression criteria" while listing a `C09` disagreement four lines above), so
this is a conclusion inherited from the producer rather than checked against the
scorer files.

The surrounding paragraph is the report's quantification of judgment dependence,
and it is the one place the substitution costs something. It gives rates for two
arms — `2 of 105` and `28 of 240` — and replaces the third with a qualitative
claim; the actual rate, `6 of 240`, is the number that belongs beside them. The
same paragraph also omits that two scored arms had **no** second scorer at all:
the entire enabled Case A arm (`enabled/scope-elicitation/README.md:84-86`) and
the Case B control (`enabled/source-fragments/README.md:110-113`) were scored
once, so every count at `:81-101` and the `1/27` at `:114` rest on a single blind
context. A reader of `:194-198` cannot tell that the arm carrying the report's
most-cited table is the least independently checked.

Recommended fix: replace the sentence at `:196-198` with the count — "the two
valid refinement scorers disagreed on 6 of 240 decisions, one of them on the
preservation row `C09`" — and add one clause naming the two single-scored arms.
No score changes: the resolutions themselves are correct under the frozen rule.

### 4. The `Verdict` credits the final skill with passing harness checks that pass by refusing it, and the replacement template check is tautological

Two claims about deterministic verification overstate what the commit's own
scripts do.

`:22-23` says "the final skill and plugin pass deterministic structure,
installation, local link, **reusable harness**, and applicable repository
checks." Every retained harness pins the *pre-refinement* skill: `expected_skill_sha=795846cb…`
appears in `enabled/exposition-assurance/harness.sh:12`,
`enabled/transfer-probe/harness.sh:14` and `enabled/source-fragments/harness.sh:14`,
each asserted against the live file before a campaign is built. The final skill
hashes to `7bfd60fe…`, so those three harnesses now *refuse* it, and the
refinement slice — the one that ran the final bytes — preserved no harness at
all. No script in the repository can execute a campaign with the shipped skill.
The table row at `:210` says this correctly ("both reject drift and pass"); the
`Verdict` bullet inverts it, and the bullet is the sentence a reader carries away.

`:208` credits `refinement-regression/template-test.sh` with a "sealed-template
delta" pass. Lines 23–36 of that script are real work — skill bytes and digest
against the retained manifest, frontmatter name/description/harnesses, a
frontmatter size bound, and `plugin.json`'s name. Lines 38–44 are not: the script
creates both template directories itself, writes `control-template/auth.json`,
copies *that file* to `enabled-template/auth.json`, copies the skill in, deletes
the whole `skills/` subtree, and then `cmp`s the two `auth.json` files. It
compares a file with a copy of itself made three lines earlier. It cannot fail,
it exercises no production code path, and it is untouched by any defect in a
harness, a template, or the skill. The assertions it replaces were the real ones:
the same commit deleted, from both `harness-test.sh` files, the checks that the
harness's own `init` produces the frozen schedule text, a control template with
no `skills/` directory, and manifests of exactly one and two rows.

There is a second-order cost in what replaced them. Both rewritten tests now pass
*because* the working tree's skill differs from the historical digest —
`enabled/exposition-assurance/harness-test.sh:23-27` asserts that `init` fails.
Restoring the pre-refinement bytes to reproduce the historical campaign, the one
operation those guards exist to make possible, turns both tests red. The tests
assert a property of the current working tree, not of the harness.

Recommended fix: soften `:22-23` to say what `:210` says — the final skill passes
structure, installation, link and repository checks, and the historical harnesses
correctly refuse it. Either drop the "sealed-template delta" clause from `:208`
or make lines 38–44 test something: build both templates through a harness that
pins the final digest and assert the manifest delta is exactly the skill subtree.
Restoring the deleted positive assertions against a digest-parameterised harness
would recover the coverage without reintroducing the drift the guards catch.

### 5. The report never records that all judged evidence but one arm was produced by a superseded skill revision

The report's stated purpose is to say what is proven about the deployed artifact,
and `:200-204` names the deployed digest `7bfd60fe…`. But every judged sample it
tabulates except the five refinement repetitions was produced by a different
skill: Case A, Case B, the initial Case C arms and the entire transfer probe all
ran skill revision `9cc8ccd8` with `SKILL.md` digest `795846cb…` and 13,927
bytes, recorded in each slice's own execution contract
(`enabled/scope-elicitation/README.md:35-39`,
`enabled/source-fragments/README.md:40-44`,
`enabled/exposition-assurance/README.md:41-45`,
`enabled/transfer-probe/README.md:63-67`). The synthesis drops those digests
entirely — neither hash appears anywhere in the report except the final one — and
`:167-178` presents the transfer probe with no indication that it never saw the
shipped bytes.

The report does say "pre-refinement" of the Case C arms (`:16`, `:65`, `:162`,
`:188`), which makes the omission harder to notice rather than easier: a reader
who takes that qualifier as meaningful will read its absence from the Case A, B
and transfer sections as saying those arms *did* use the final skill. The correct
statement is stronger than anything in `Unresolved gaps`: the only judged
evidence of any kind for the deployed bytes is one five-repetition Case C rerun
against one prompt, and even that arm's C23 count stays below its contemporaneous
control (`4/5` against `5/5`, `:152`).

Recommended fix: state the two digests and which arms each governs once in the
`Frozen comparison contract` or the `Campaign status` table, and add the
consequence to `Unresolved gaps` — that all judged evidence except the Case C
refinement rerun describes a superseded revision.

### 6. The durable report is unreachable from every surface that survives `.grove/`

`skill-evaluation-report-k49`'s running log calls this file "the durable
synthesis", and the `skill-evaluation-k22` brief asks the evaluation artifact to
state what the deployment claim rests on. Nothing outside `docs/evaluations/`
links to it. Grepping `evaluations/` across the repository with `.jj`, `target`,
`.grove` and the evaluation tree itself excluded returns nothing; the same
command finds `CONTEXT-MAP.md:21`'s link to the `ordinal-fs-tree` book, so the
instrument works and the convention it demonstrates is the repository's own.
`plugins/linkuistics/PROVENANCE.md:30-45` is the file that describes this skill's
evidence, and it documents the method's external foundations and closes with "the
skill does not claim controlled reader-outcome evidence for them" — written
before the campaign existed, and never updated to point at the campaign that
found the acceptance rubric not met. `plugins/CONTEXT.md` does not mention the
evaluation either.

`.grove/` is removed at the finish cycle, and the seven task files under
`03-skill-evaluation-k22/` are the only other things that name the report. On the
day this grove finishes, the evaluation becomes an orphan directory, and the
shipped skill's provenance file will assert its pedigree with no route to the
evidence that qualifies it.

Recommended fix: add one link from `plugins/linkuistics/PROVENANCE.md`'s
`writing-code-walkthroughs` entry to `docs/evaluations/writing-code-walkthroughs/README.md`,
with a clause saying the campaign did not establish the acceptance rubric. A
second from `plugins/CONTEXT.md` or `CONTEXT-MAP.md` would match how the book is
surfaced, but the provenance link is the one that must exist.

### 7. Two disclosure gaps in the deterministic section

Both are small and neither changes a number.

The refinement slice deviates from the rubric's `Campaign records` requirement
(`baseline/rubric.md:352-356`) — "one Markdown record per planned repetition"
with the execution manifests, run directory, prompt, raw JSONL, final answer,
termination status, score table, totals and invalid-attempt history — and the
report does not say so. Every other slice ships them
(`enabled/exposition-assurance/enabled-repetition-*.md`,
`enabled/transfer-probe/control-repetition-*.md`);
`refinement-regression/` ships `README.md` alone, with the per-attempt files
under `evidence/` and no preserved `harness.sh`. The underlying material is all
there, so this is a record-shape deviation rather than missing evidence — but the
report leans on that slice for its only claim about the shipped bytes (finding 5)
and lists it at `:208` as a deployment check, which makes the deviation worth one
sentence.

The `shellcheck` invocation at `:212` names six of the eight shell scripts under
`docs/evaluations/`, omitting `enabled/source-fragments/harness.sh` and
`enabled/transfer-probe/freeze-harness.sh`. The row reports honestly on what it
ran; the gap is coverage, and both omitted files are harnesses that executed
campaign runs the report cites.

Recommended fix: name the record deviation in `Unresolved gaps`, and either
extend the `shellcheck` operand list to all eight scripts or say the row covers
the scripts the final slice touched.

## Decisions (running log)

Reviewed by inspection only. No scenario, harness, build, lint, format or test
command was run and no producer artifact was edited. Verification used reading,
`shasum -a 256`, `jj log`/`jj diff`/`jj show`, `grep`, and two throwaway Python
scripts that re-tallied the retained adjudication files — none of which invokes
an evaluated model or mutates the tree. The three deterministic rows the report
cites but this review could not re-derive by inspection (`plugins/install.test.sh`,
`cargo test`, `cargo fmt`, `cargo clippy`) were checked structurally instead:
`install.test.sh`'s 20 assertion sites were counted in source, and
`tests/reference_navigation.rs` was read to confirm its sweep reaches
`docs/evaluations`. Re-running them is the integrator's, not the reviewer's.

The interval argument in finding 1 was adopted deliberately over the report's
uniform "unevaluable". It supplies no value for the missing sample: it enumerates
both values the sample could have taken and shows the guard verdict is the same
either way, which is a different move from the zero-substitution the report at
`:47-49` correctly refuses. Applied symmetrically it also settles the primary
endpoint against the skill's best case, so it is not a rule that only bites in
one direction.

Findings 1-3 change what the report concludes and are the ones that decide
whether its verdict section can be read as written. Findings 4-6 are omissions
and overstatements rather than wrong numbers. Finding 7 is bounded and could be
accepted visibly as a stated limitation. None requires reopening the frozen
rubric or rerunning a campaign arm, and no finding proposes a score change: every
resolved count in the report reproduces from the retained evidence. The findings
are substantive, so an adjacent `integrate-review-impl` leaf is appended with
`**Integrates:** skill-evaluation-report-k51`.

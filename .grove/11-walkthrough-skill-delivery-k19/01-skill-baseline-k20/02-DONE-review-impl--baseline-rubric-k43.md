# baseline-rubric-k43

**Reviews:** baseline-rubric-k38

## Goal

Adversarially verify that the frozen baseline rubric can support a fair,
auditable same-case comparison without overstating isolation, reproducibility,
or generalization.

## Context

- Producer artifact: `baseline-rubric-k38` and its commit.
- Primary artifact:
  `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`.
- Frozen fixture under that rubric's `fixtures/external-check-floor/` directory.

## Done when

- The review checks prompt/criterion alignment, atomic scoring, invalid-run
  selection, treatment/control equivalence, access evidence, fixture portability,
  runtime drift controls, and the stated limit on same-case conclusions.
- Findings cite the producer commit and exact artifact locations.
- The review does not run a baseline scenario or edit the producer artifact.

## Notes

The producer already spent its one in-session doubt pass and made substantive
changes. This tree-level review is the independent re-read before any baseline
run. If it finds actionable issues, insert an `integrate-review-impl` leaf ahead
of the first run leaf.

## Findings

Sixteen findings against `docs/evaluations/writing-code-walkthroughs/baseline/rubric.md`
as committed by `baseline-rubric-k38` (jj change `zqluzomw`, commit
`0fc320b4f9ac2cfce99aefb791e381594fd82dc2`), most severe first. Line references
are to that file at that commit unless another path is named.

The mechanical layer was checked first and is clean. The committed fixture
`docs/evaluations/writing-code-walkthroughs/baseline/fixtures/external-check-floor/targets/ocaml/check_floor.ml`
hashes to `2624183a8836364b5fdbcbeae7bf62de20d88550e6e2358aad13812da4cb0f0e`,
equal to the frozen original digest at `:169`, so the guard at `:174-177` is
executable and the run bytes are portable without the origin workspace. The
producer commit contains only task files, the brief rename, the rubric and the
fixture — no run output, no skill bytes — so the causal boundary the leaf claims
holds. Every flag in the frozen command shape at `:32-36` exists in
`codex-cli 0.150.1` (`--ignore-user-config`, `--ignore-rules`, `--ephemeral`,
`--skip-git-repo-check`, `--sandbox read-only`, `-m`, `-c`, `--json`, `--cd`),
so the campaign will not abort on an unrecognised option. Criterion coverage
against the node brief's list is complete: scope elicitation `A01-A21`, exact
source inventory `B01-B03`, conceptual ordering `B04-B07`, fragment
completeness `B08-B17`, validation `B13-B20`/`C10-C13`, self-contained prose
`C01-C03`/`C07-C09`, repetition versus links `C04-C07`, independent review
planning `C14-C19`. The findings below are what those checks cannot see.

### 1. The "no-skill" control is not skill-free, so the report's absolute classifications are unsupported

`:45-47` requires a recursive manifest of "the Codex skill directories and any
runner configuration the harness can load", and then asks of the baseline only
that it "contain no path named `writing-code-walkthroughs`". Recording an
influence is not removing it, and the single forbidden path is the only skill
the baseline excludes.

On the host the campaign will run on, `~/.codex/skills` currently holds 31
skills, among them `authoring-conventions`, `codebase-design`,
`decision-records`, `doubt-driven-development`, `requesting-code-review`,
`test-driven-development`, `verification-before-completion`, `writing-skills`,
`using-superpowers` and `grove`. `~/.codex/AGENTS.md` (4,240 bytes) and
`~/.codex/hooks.json` (1,013 bytes, `SessionStart` hooks invoking
`herdr-agent-state.sh session` and `codebase-memory-mcp hook-augment`) also
exist. Neither frozen flag is documented to suppress any of them: `codex exec
--help` scopes `--ignore-user-config` to "Do not load `$CODEX_HOME/config.toml`"
and `--ignore-rules` to execpolicy `.rules` files.

Arm *difference* survives this, because both arms share the same corpus, so the
material-improvement rule at `:112-115` is not threatened. What does not survive
is the *absolute* reading the node brief requires — "The report distinguishes
failures the new skill should address from behavior already reliable without
guidance"
(`.grove/11-walkthrough-skill-delivery-k19/01-skill-baseline-k20/BRIEF.md`).
`C14`/`C17` (independent technical and editorial review) are the standing
subject of `requesting-code-review`; `B13-B20` and `C10-C13` (mechanical proof
before claiming completion) are the standing subject of
`verification-before-completion`. If those score `1` in five repetitions, `:107`
classifies them `present in this sample` and `:109-110` forbids skill wording
for them — so the shipped skill would omit a rule for behavior that an unrelated
installed skill, not the unguided model, supplied.

The rubric already states the correct principle for exactly this situation at
`:65`: "Do not claim that an unenumerated host filesystem was inaccessible." The
same burden applies to its own controls.

Recommended fix: run every repetition with `CODEX_HOME` pointed at a purpose-built
directory containing only what authentication requires, so `skills/`, `AGENTS.md`
and `hooks.json` are provably absent, and record that directory's recursive
manifest as the campaign's control manifest. The alternative — keeping the
environment and renaming the arm an "installed-corpus control", with the report
forbidden from concluding "reliable without guidance" for any criterion an
installed skill plausibly supplies — is strictly more expensive and weaker.
Either way, verify the flags' actual behavior before the first scored run rather
than inferring it from the help text, as this finding does.

Separately, `:63-65` enumerates what the run record must capture — "the exact
user prompt, the startup skill/config manifests above, every file path and
digest in the run directory, and every tool result delivered to the model".
Hook-injected context is none of those, so the declared record does not capture
everything delivered to the evaluated model. Add harness- and hook-injected
messages to that list.

### 2. Invalidating enabled repetitions that did not use the skill filters out the skill's own failure mode, and contradicts the sampling rule

`:51-52`: "An enabled repetition is invalid unless the raw events or final answer
show that the target skill was discovered and used." `:75-79`: "A run is
replaceable **only** when the process exits nonzero, emits an explicit
failed/cancelled response status, emits no final assistant message, violates a
declared tool-access rule, or mutates the run directory. Truncation, refusal,
irrelevant content, and poor substantive answers remain valid samples when a
final message exists."

Non-discovery appears in neither list, and the two sentences give opposite
answers for the same run. That is a hard internal contradiction in a frozen
contract, and it resolves in the direction that inflates the treatment: a
repetition where the model ignored the skill is discarded and replaced, so the
enabled arm measures the skill *when it fires* and reports it as the skill.

Discovery is a first-order property of skill quality here — the parent brief
requires "Skill frontmatter, progressive disclosure, citations, harness
declaration, plugin layout, and description follow the Linkuistics authoring
conventions"
(`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md`), and a description that
does not trigger is precisely a description defect.

Compounding it, `:8` requires that "Baseline and skill-enabled runs use the same
prompts" while `:49` requires the enabled arm to record an "invocation prompt".
If discovery is spontaneous, `:49`'s field is vestigial; if it is not, the
enabled prompt differs from the frozen prompt and the arms differ by more than
the skill bytes. The rubric does not say which.

Recommended fix: keep the enabled user prompt byte-identical to the frozen
prompt; score a non-discovery repetition as an ordinary valid sample; report
discovery rate per case as a declared outcome in its own right; and if an
explicitly-invoked condition is wanted, declare it as a third arm rather than as
a validity filter. Then delete the `:51-52` invalidation or restate it as the
reporting obligation it should be, so `:75-79` remains the single authority on
replaceability.

### 3. The contemporaneous control's skill-installation state is undeclared, and cannot be reconciled with the manifest-equality rule

`:41-43` requires enabled evaluation to "rerun five contemporaneous no-skill
controls per case, interleaved ABBA/BAAB across the enabled repetitions".
`:50-51` requires that "The normalized baseline and enabled manifests must be
identical after removing only that one skill subtree."

Nothing states whether a contemporaneous control runs with the skill subtree
present on disk or removed. Both readings fit the text and they measure
different things: present, the control can discover the skill and is not a
no-skill control at all; removed, the interleaving requires an install/uninstall
between adjacent runs, each control repetition has its own manifest, and the
single campaign-level manifest comparison at `:50-51` is no longer the right
instrument.

This is the drift control the whole enabled comparison rests on, so leaving it to
the runner's judgement at run time defeats the point of freezing it now.

Recommended fix: declare that each contemporaneous control runs against a
control home with the skill subtree absent, that its manifest is recorded per
repetition rather than per campaign, and that the install/uninstall transition is
itself recorded in the run record.

### 4. No adjudicator identity, independence, or blinding rule

The rubric's claim to fairness rests entirely on atomic determinism (`:88-97`),
but it never says who scores, whether the scorer may be the session that produced
the runs or authored the skill, whether one scorer scores both arms, or whether
arm labels are hidden during scoring.

Several criteria are unavoidably judgement calls: `C07` (`:254`, "remains
understandable if its optional link is removed"), `C08` (`:255`, "direct
declarative prose"), `C09` (`:256`, "does not teach ordinary language or storage
mechanics granted by the audience"), `B07` (`:204`, "causal behavior or
invariants rather than file line order alone"). With arm labels visible and the
skill's own author scoring, the two-of-five threshold at `:112-115` is reachable
by adjudication drift alone, with no dishonesty required.

Given that the campaign directory layout (`:270-274`) puts baseline and enabled
runs in separate paths, blinding is the cheapest of the fixes here and the one
that most directly protects the comparison.

Recommended fix: name the adjudicating context; require it to be fresh and
distinct from the session that authored the skill; require scoring from
arm-stripped final-answer text plus events; keep the smallest-supporting-passage
citation at `:92-93` mandatory for every `1` so scores are re-auditable; and
require one case to be scored twice independently, reporting the disagreement
count.

### 5. Absence-shaped criteria cannot be scored under the stated scale

`:89-93` defines `0` as "the named behavior has no directly citable evidence or
is contradicted" and `1` as "the named behavior is explicitly present. Cite the
smallest supporting passage or event."

Five criteria name an absence rather than a presence: `B18` (`:215`,
"Compilation or copied-snippet presence is **not offered** as a substitute for
byte equality"), `C04` (`:251`, "The plan **does not duplicate** the
authoritative source fragment"), `C07` (`:254`, "remains understandable **if its
optional link is removed**"), `C08` (`:255`, "**no** rhetorical question or
suspense"), `C09` (`:256`, "**does not teach** ordinary language or storage
mechanics").

For each of these, a fully compliant answer contains no citable passage, so the
letter of `:89` scores it `0` — the scale inverts on exactly the criteria that
encode the project's strongest editorial positions. An adjudicator will
improvise, and the improvisation is unrecorded, which is the same defect as
finding 4 in a narrower place.

Recommended fix: add a clause to `:88-97` for absence criteria — score `1` when a
stated exhaustive scan of a named surface finds no instance, citing the scanned
surface and the nearest near-miss considered; score `0` on the first instance
found, citing it.

### 6. The campaign-level success condition the parent brief requires is never frozen

The parent brief's `Done when` requires that the enabled runs "meet the rubric,
and show a material improvement over baseline"
(`.grove/11-walkthrough-skill-delivery-k19/BRIEF.md`). The rubric defines
material improvement only per criterion (`:112-115`). It never states how many
criteria must improve, over which subset, for the campaign to count as improved.

The three plausible denominators give very different verdicts: all 64 criteria;
only those the baseline report classified `repeated gap`; or only those that
actually motivated a skill rule. Choosing among them after the counts are known
is post-hoc endpoint selection, and freezing predeclaration is the entire reason
this leaf exists. With 64 binary criteria at n=5, a two-of-five move is not a
rare event, so a raw count of improved criteria carries no declared expectation
against which to read it.

Recommended fix: freeze the aggregate rule in this file now. The defensible
primary endpoint is the set of criteria the baseline report classifies `repeated
gap`, with a declared minimum fraction of that set improving materially and no
`present in this sample` criterion regressing by two or more; everything else is
reported as secondary and explicitly cannot establish the parent brief's
condition.

### 7. The regression guard in the material-improvement rule has two readings

`:113-115`: "an atomic criterion improves materially when its enabled success
count exceeds both the historical baseline and the contemporaneous no-skill count
by at least two of five, **with no criterion losing two or more successes**."

The trailing clause sits inside a single-criterion definition but quantifies over
"no criterion". Read per-criterion it is vacuous, since the criterion under
consideration is gaining and cannot simultaneously lose. Read campaign-wide, a
single regression anywhere in 64 criteria blocks every claim of material
improvement — a far stricter rule than the sentence appears to intend, and one
that a 64-criterion campaign at n=5 will very likely trip by noise.

Recommended fix: state which reading binds. If campaign-wide, move the clause out
of the per-criterion sentence and into the aggregate rule from finding 6, where
its denominator can be stated.

### 8. Case A's criteria switch from "asks a question" to "is frozen" halfway down the table

`A04`-`A13` (`:143-152`) each require "A later question ...". `A14`, `A15`,
`A17` and `A18` (`:153`, `:154`, `:156`, `:157`) instead say the item "is
frozen", and `A19`-`A21` (`:158-160`) say it "is elicited".

The frozen prompt ends "Finish with the contract you would freeze before
inspecting code" (`:127-128`), so a model can list depth, output form, prose
constraints and navigation constraints in the closing contract without ever
asking about any of them. Under `A14`/`A15`/`A17`/`A18` as written that scores
`1`; under the case's own title, "scope **elicitation** protocol" (`:117`), and
the node brief's "one-question-at-a-time scope elicitation", it should score `0`.
Two adjudicators will split, and the split falls on the behavior the case exists
to measure.

Recommended fix: make each row state its evidence explicitly — a question in the
ordered list, an entry in the closing contract, or either. Where elicitation is
what is measured, require the question, and add a separate row for the contract
entry if freezing is also worth scoring.

### 9. Three criteria are answered by their own frozen prompt

- `B01` (`:198`) requires the inventory to contain exactly
  `targets/ocaml/check_floor.ml` as production source. The prompt states "The
  only in-scope production source is targets/ocaml/check_floor.ml; no other
  project file is supplied or in scope" (`:184-185`).
- `C02` (`:249`) requires the paragraph to connect meaning to the write "without
  inventing a function, type, effect, or invariant". The prompt instructs "Do not
  invent implementation names or behavior" (`:234-235`).
- `C03` (`:250`) requires unknowns to be "explicit placeholders or
  source-verification obligations". The same prompt sentence instructs "express
  unknown implementation details as placeholders or verification obligations"
  (`:235-236`).

These measure instruction compliance, not unguided authoring judgement, and they
have no headroom in the enabled arm. They will almost certainly score `1` five
times, be classified `present in this sample` by `:103`, be excluded from skill
guidance by `:109-110`, and then appear in the report — per the node brief — as
"behavior already reliable without guidance". That conclusion is not available
from a case that supplied the answer.

`B02` and `B03` are *not* in this class and should stay as they are: the prompt
establishes that other files are unsupplied, while the criteria test whether the
answer distinguishes *unknown or out of scope* from *absent from the project*,
which the prompt does not resolve.

Recommended fix: mark `B01`, `C02` and `C03` in the table as compliance controls,
excluded from the reliable-without-guidance classification and reported
separately. Removing the instructions from the prompts is the wrong direction —
`C02`/`C03`'s instruction is what keeps a fixture-free case from dissolving into
fabrication, which would make the other Case C rows unscoreable.

### 10. Invalid-attempt counts are preserved but never reported, hiding a differential compliance rate between arms

`:58-62` invalidates any Case A or C repetition containing a tool call and any
Case B call outside the fixture boundary. `:80-84` permits two replacements per
planned repetition and requires preserving "every invalid attempt and its
machine-checkable reason". Nothing requires those counts to appear in the
per-case record at `:270-275` or in the final report at `:277-282`.

Instruction compliance under an explicit "Do not ... call tools" (`:123`,
`:234`) is itself a behavior a skill can change, and it can change in either
direction — guidance that pushes toward source inspection raises violations. As
written, an enabled arm that needed two replacements per repetition and a
baseline arm that was compliant first time produce identical-looking score
tables.

Recommended fix: declare invalid-attempt counts, broken down by reason, per case
and per arm, as a reported outcome beside the criterion table, and require the
final report to state them.

### 11. Truncation is a valid sample, but no output bound is pinned — and the exposed criteria are the last rows of every case

`:79` keeps truncated answers as valid samples and `:88` scores from the final
answer, so a truncated answer contributes `0` on every criterion its text never
reached. The frozen command shape (`:32-36`) pins CLI version, sandbox, model
alias and reasoning effort, but nothing bounding output length, turns, or
wall-clock, and `:40-41` requires the enabled campaign to match only "the same
CLI version and command shape".

The exposure is not evenly spread. Each case's criteria are ordered so that the
assurance rows come last: `A19`-`A21` (mechanical proof, technical review,
editorial review), `B13`-`B22` (the checks and the walk-away property),
`C14`-`C21` (technical review, editorial review, walk-away). Those are precisely
the behaviors the skill is meant to install, so a length cap that differs between
the three baseline run leaves — run in separate sessions at separate times — and
the later enabled campaign manufactures repeated gaps in the rows that matter
most, or erases them.

Recommended fix: pin an explicit output/turn bound in the frozen command shape;
record per repetition whether the final message terminated normally; and mark
criteria beyond the truncation point as `truncated`, reported separately, rather
than folding them into `0`.

### 12. The transfer probe — the only generalization arm — has no frozen success rule and its case is chosen by a context that has read the skill

`:17-23` is the right instinct: the purpose section (`:12-15`) correctly
disclaims generalization for the same-case arms, and the probe is what would
supply it. Three gaps stop it doing so.

Selection happens "After the skill bytes are frozen" (`:17`), and the rubric does
not require the selector to be ignorant of the skill body, so the probe's
codebase and prompt can be chosen — without any intent to cheat — to suit what
the skill happens to say. The probe "may add case-specific criteria before its
first run" (`:21-22`) with no rule that those criteria be written before their
author reads the selected codebase. And `:115`'s "Report all counts even when
this condition is unmet" is scoped to same-case enabled evaluation, so the probe
carries neither a declared threshold nor a report-regardless obligation, leaving
its outcome open to post-hoc characterisation.

Recommended fix: require the probe's target and prompt to be selected by a
context given only the campaign's subject matter and not the skill body; require
its added criteria to be frozen before their author sees the selected codebase;
declare its success rule in this file now; and state that its counts are reported
whatever they show.

### 13. The run directory is not required to sit outside this repository

`:27-28` requires "a new empty temporary directory". `:66-68` forbids placing the
Grove tree, completed book, research synthesis, rubric, draft skill, repository
agent instructions or prior conversation "in a run directory or prompt".

An empty directory created *under*
`/Users/antony/Development/grove.code-walkthrough-for-ordinal-fs-tree` satisfies
both sentences literally while sitting inside a tree that contains every one of
those artifacts. `--skip-git-repo-check` (`:33`) permits running outside a
repository; it does not require it. Case B compounds this: it permits read-only
commands whose "operands stay under the run directory" (`:60-61`), and that
boundary is auditable only when no project material sits above the run directory
for a relative traversal to reach.

Recommended fix: require the run directory to be created under the system
temporary directory, outside any ancestor containing repository content, and
record its absolute path in the access manifest at `:63-65`.

### 14. The freeze clause names a commit that this review's own integration would replace

`:6` states "Commit `baseline-rubric-k38` freezes this file and its external
fixture before any evaluated run", and `:9` states "Later leaves append outcomes
and reports but do not edit this file."

The node brief already schedules this review and a conditional integration ahead
of the run leaves
(`.grove/11-walkthrough-skill-delivery-k19/01-skill-baseline-k20/BRIEF.md`,
`Decisions`), so an integration acting on any finding above must edit this file —
which an auditor reading only the rubric will read as a violation of `:9`, and
which leaves `:6` naming a commit that is no longer the freeze.

Recommended fix: restate the boundary as the last commit before the first
evaluated run, name both the rubric commit and the integration commit, and scope
`:9` to leaves that run scenarios.

### 15. Fixture provenance is unverifiable off this host, and the fixture's own prose primes the behavior Case B scores

The run-time story is sound: the committed bytes match the frozen digest, so
`:174-177`'s abort-on-mismatch guard works and no run depends on the origin
workspace. Two narrower points remain.

The provenance rows at `:166-169` give a machine-local absolute path
(`/Users/antony/Development/APIAnyware.add-ocaml-target`) and a bare revision
with no remote URL, so the producer's own criterion — "One case names an external
repository source by revision, path, and digest"
(`.grove/…/01-DONE-impl-baseline-rubric-k38.md`) — is auditable only on this
machine.

Separately, the fixture's opening comment
(`fixtures/external-check-floor/targets/ocaml/check_floor.ml:1-25`) is an
extended argument about verification discipline: "THE AUTOLINK COUNT IS THE
CONTROL ARM", "A fix verified through `otool -l` alone would have looked complete
and shipped the hazard". That primes exactly the mechanical-proof reasoning
`B13`-`B20` score. It primes both arms equally, so the arm difference is
unaffected, but it is a second reason — after finding 1 — that a `present in this
sample` result on those rows cannot be read as unguided reliability.

Recommended fix: record the origin remote URL, or state plainly that provenance
is host-local; and note the prime among the report's stated limitations. Do not
change the fixture — its being real external source is what the node brief asks
for, and a sanitised replacement would forfeit that.

### 16. Two behaviors the method synthesis predeclared as evaluable have no criterion, and freezing forecloses them

`docs/research/walkthrough-method.md:425-448` lists fourteen observable
behaviors for paired baseline and skill-enabled evaluation. Two have no atomic
criterion in any case:

- **Worked execution** — "The example names production inputs, outputs, stage
  boundaries, governing invariants, and observable results; it explains why each
  transition exists." `B06` (`:203`) covers a low-resolution operation tour in
  the *ordering*, not a worked example's content.
- **Technical prose**, beyond rhetorical staging — "Pages lead with claims, use
  stable vocabulary and explicit actors, distinguish refusals from environmental
  failures and defects". `C08` (`:255`) covers only the rhetorical half;
  `C15` (`:262`) covers the refusal distinction as a *review* obligation, not as
  a prose behavior.

The node brief's coverage list does not name either, so this is within charter
and is raised as a decision rather than a defect. But `:107` permits skill
guidance only for an observed repeated gap, so once the first run lands, the
frozen rubric structurally forecloses evidence-based guidance on worked examples
and on prose actor/refusal discipline — both of which the root brief's `Done
when` treats as central to the book. This is the last moment the choice is
available.

Recommended decision for the integration: either add rows for those two
behaviors, or record in the rubric that they are deliberately outside the
baseline's scope and that any later skill wording about them will be labelled
method-derived rather than gap-derived.

## Decisions (running log)

Reviewed by inspection only. No baseline scenario was run and no producer
artifact was edited: `codex --version` and `codex exec --help` were read to
confirm the frozen command shape parses under `codex-cli 0.150.1`, the fixture
was hashed against the frozen digest, and `~/.codex` was listed to establish
finding 1, but no evaluated model was invoked. Invoking one — even with a
throwaway prompt — would have put a model run before the freeze commit that
`baseline-rubric-k38` exists to establish, so verifying whether the frozen flags
actually suppress `~/.codex/skills`, `AGENTS.md` and `hooks.json` is handed to the
integration as a pre-campaign obligation rather than settled here.

Findings 1-3 are the ones that decide whether the campaign can be read at all;
9, 15 and 16 are bounded and could be accepted visibly as stated limitations
rather than fixed. All sixteen are actionable before the first run, so an
`integrate-review-impl` leaf is inserted ahead of `baseline-scope-elicitation-k42`.

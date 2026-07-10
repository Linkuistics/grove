# mattpocock/skills v1.1 — incorporation analysis

_Research leaf `plan-k1`, 2026-07-09. Analyses the upstream update to
[`mattpocock/skills`](https://github.com/mattpocock/skills) and reports what is
worth incorporating into two downstream repos: **grove** (this repo) and
**`Linkuistics/skills`** (`~/Development/skills`). Findings are split by target;
each carries a primary citation (commit / file / quote) and an
incorporate/skip recommendation._

## Window and method

- **grove's provenance pin:** `mattpocock/skills@b8be62f` (headers in
  `content/grilling.md`, `content/CONTEXT-FORMAT.md`).
- **Upstream HEAD analysed:** `d574778` (v1.1). **189 commits** since the pin.
- Analysed against a full clone, diffing the specific files grove/Linkuistics
  derive from, plus reading the net-new skills. Four parallel cluster
  deep-dives (grilling; CONTEXT/ADR format moves; the new planning family;
  skill-authoring) fed this synthesis.

**Prior-art backdrop — this is a *delta*, not a first look.** `Linkuistics/skills`
already ran a prior-art survey (`docs/research/skill-repo-prior-art.md`,
`grove-recommendations.md`, 2026-06-25) that deep-dived mattpocock/skills. It
drove Linkuistics to author `codebase-design` and `decision-records`, and its
`grove-recommendations §8` already flagged grove's grilling.md bundle drift.
This report reports only what has changed **since that survey / since grove's
pin**, and updates two of its now-stale conclusions.

**Two structural moves frame everything below** (both in commit `221ffca`,
2026-06-12):
1. The **interview procedure** was extracted into a standalone primitive
   `skills/productivity/grilling/SKILL.md`; grove's source `grill-with-docs`
   was gutted to a 7-line stub (`git diff b8be62f..HEAD`: 88 → 7 lines).
2. The **domain / CONTEXT / ADR material** moved wholesale into a new
   `skills/engineering/domain-modeling/` skill, taking `CONTEXT-FORMAT.md` and
   `ADR-FORMAT.md` with it.
   grove's single `content/grilling.md` is a *merge* of what upstream now keeps
   in two files — so the right model is **cherry-pick specific fixes, not
   re-bundle**.

A third move updates the prior survey: the skill it named grove's closest
philosophical analog, `decision-mapping`, was renamed
`decision-mapping → wayfinding → /wayfinder` and **graduated to engineering in
v1.1** (`639df6e`), reframed around *destination / frontier / fog-of-war* with
deep issue-tracker integration.

---

## Part 1 — grove (this repo)

### G1 — Fix the self-grilling bug in `grilling.md` — **DO (highest value; a real defect)**

grove's `content/grilling.md:11` reads, verbatim:

> If a question can be answered by exploring the codebase, explore the codebase instead.

Upstream identified this exact line as a **bug** and fixed it in `e5932a7`
("wayfinder/grilling: stop the agent grilling itself"), replacing it with:

> If a *fact* can be found by exploring the codebase, look it up rather than asking me.
> The *decisions*, though, are mine — put each one to me and wait for my answer.

Commit rationale: *"a grilling line written for the live-human case ('explore
the codebase instead') reads as license to answer questions autonomously once
[it runs] in a resolve-the-ticket frame."* **grove's self-driving loop is
exactly that semi-autonomous frame** — the failure case upstream fixed. This is
the single strongest finding of the analysis: a planning agent that answers its
own decision questions defeats the entire purpose of the grilling step.

### G2 — Add a confirmation gate to `grilling.md` — **DO (needs grove-appropriate wording)**

Upstream added a hard stop in `0e9a072` ("grilling: add confirmation gate"):

> Do not enact the plan until I confirm we have reached a shared understanding.

Verified there is **no equivalent gate** anywhere in grove's `grilling.md` or
launcher prompts (`content/prompts/`). In grove a planning leaf's job *includes*
growing the tree and writing ADRs, so the gate needs grove-specific phrasing —
"reach shared understanding before growing the tree / committing decisions,"
not a blanket "don't build." Aligned with grove's grilling philosophy (converge
first).

### G3 — Refresh `grilling.md` provenance + own-the-fusion note — **DO (cheap, closes a recorded gap)**

The header attributes grove's grilling content to `grill-with-docs/SKILL.md` —
now a 7-line stub. The content grove mirrors now lives in **two** files
(`productivity/grilling/SKILL.md` + `engineering/domain-modeling/SKILL.md`).
`grove-recommendations §8` already recommended annotating the fusion as
*deliberate* (grove has no skill-to-skill invocation, so re-syncing the split
would be cosmetic). Update the header to the two current paths at a fresh pin
and add the one-line "intentionally fused; upstream has since split" note.

### G4 — Decomposition *craft* into `driving.md` / `BRIEF-FORMAT.md` — **CONSIDER (the substantive methodology enrichment)**

grove's Decompose step says *when* to split a leaf but is largely silent on what
a good child leaf *looks like*. The new planning family supplies concrete craft
grove can lift as **prose into its own docs** (no skill is vendored):

- **Vertical-slice leaf shape** (`to-tickets/SKILL.md`): *"Each slice cuts a
  narrow but COMPLETE path through every layer … A completed slice is demoable
  or verifiable on its own."* Sharpens grove's "fits this session" test with a
  second axis: a leaf should be independently demoable, not a horizontal layer
  that's dead until siblings land. → `driving.md` Externalizing / SKILL
  Decompose.
- **Wide-refactor expand→contract exception** (`to-tickets/SKILL.md`): for the
  mechanical change *"whose blast radius fans across the whole codebase … no
  vertical slice can land green"* — *"First expand … Then migrate the call
  sites over in batches … Finally contract."* grove currently offers nothing
  for this case, which breaks its one-clean-green-leaf model. → `driving.md`.
- **"Not yet specified" horizon note + fog-or-ticket test** (`wayfinder/SKILL.md`):
  grove's laziness means the *dim view of what's coming* is lost between
  sessions — a leaf either exists or it doesn't. Wayfinder's **Not yet
  specified** section records the frontier without pre-slicing it, with a crisp
  rule grove lacks: ticket *"whether you can state the question precisely now —
  not whether you can answer it now."* A short "on the horizon" note in a
  `BRIEF.md` is **not a leaf**, so it honours constraint 4. This fills grove's
  one genuine methodology gap. → `BRIEF-FORMAT.md` + `driving.md`.
- **Behavioural-not-procedural brief durability** (`triage/AGENT-BRIEF.md`):
  *"describe interfaces, types, and behavioral contracts … Don't reference file
  paths — they go stale. Don't reference line numbers."* grove already applies
  this to *identity* (permanent `-k<key>`); this extends it to brief *content*.
  → `BRIEF-FORMAT.md`.

### G5 — Smaller grove enrichments — **MAYBE**

- **"Don't grove this" no-fog early-exit** (`wayfinder/SKILL.md`): a "when NOT
  to start a grove" gate — *"If this surfaces no fog … you don't need a map.
  Stop and ask."* grove's `root-init` always mints a planning leaf; the negative
  check is implied but never stated. → `driving.md` / fresh-grove-start.
- **Seam-sketching as a planning output** (`to-spec/SKILL.md`): *"Sketch out the
  seams at which you're going to test … the ideal number is one."* Wires
  seam-picking into grove's planning/PRD flow so work leaves inherit an agreed
  test target. → grove PRD/planning prose.
- **GLOSSARY-FORMAT rule** (`teach/GLOSSARY-FORMAT.md`, new): *"Use the
  glossary's own terms inside definitions."* A format-agnostic rule grove's
  `CONTEXT-FORMAT.md` lacks. → `CONTEXT-FORMAT.md` (a deliberate divergence from
  upstream, which *removed* rules — see G6).
- **"Asking multiple at once is bewildering"** clause (`grilling/SKILL.md`):
  cosmetic reinforcement of grove's one-at-a-time rule. → `grilling.md`.
- **Negation as an authoring lens** (`writing-great-skills`): grove already
  *obeys* the cure (its prohibitions are paired with positives); use it as a
  light self-check for any *bare* `never/don't` in the bundled `*-FORMAT.md`
  files. Validation, not a content change.

### grove — explicit non-actions (deliberate divergences)

| Upstream change | Why grove does NOT take it |
|---|---|
| Re-sync `CONTEXT-FORMAT.md` | Upstream delta is **purely subtractive** (removed example-dialogue, relationships, flagged-ambiguities rules). grove ships the richer pre-trim **superset**, byte-identical to its pin — re-syncing would *lose* guidance. |
| `ADR-FORMAT.md` update | **Pure move, byte-identical.** Upstream still prescribes sequential numbering (`0001-`) + status frontmatter, which grove deliberately rejects (`linkuistics:decision-records`). |
| The `grilling`/`domain-modeling` split | grove wants one self-contained file for its single consumer; the split only pays off with multiple wrapper skills, which grove has no mechanism for. |
| `research` skill | grove's `driving.md` research-leaf discipline strictly **dominates** it (downstream-question naming, post-mortem bias, per-system walk-away check, missing-source recording). |
| `implement` / `prototype` skills | Covered by grove's work-task + `verify` skill, and "research retires into ADRs". **Findings adopted (2026-07-10):** the *skills* are still not vendored, but `task-kinds-model-selection-k6` adopted `research` and `prototype` as **task kinds** — the disciplines they encode earned a place in grove's taxonomy even though the skills did not (ADR `task-kind-taxonomy`). |
| `wayfinder` tracker/DAG/claim-by-assignment | **Decided, not merely recommended** — grilled to a `stay` in leaf `issues-substrate-brainstorm-k5` (2026-07-09). Costs three spine constraints: **1** (state moves to GitHub's server, history to their database), **2** (bootstrap would require `gh` to *succeed* before work begins), **6** (walk away and `.grove/` is not a folder of notes — it is nothing). Buys nothing in return: the DAG and claim-by-assignment are **multi-writer coordination** primitives, and a grove tree is single-writer by construction (*task-tree-scheme*), so they lock against contention that cannot arise; `find .grove` already renders the frontier and `pick` already *is* the blocking rule. grove's git-tracked tree in one worktree **is** its wayfinder — a parallel reinvention, not a competitor. Inbound GitHub issues stay handled ad-hoc by a user-directed grove; no capture subsystem returns. Reopen only if grove gains contributors or the loop gains true multi-agent concurrency. |

---

## Part 2 — `Linkuistics/skills` (`~/Development/skills`)

> **Boundary note (updated per user decision, 2026-07-09).** The Linkuistics
> survey's original discipline ("a cross-repo finding is a recommendation only,
> never implemented from this worktree") is **rescinded for this workstream**:
> the user treats grove and `Linkuistics/skills` as one system and is happy to
> do work on both repos from either. This grove therefore implements the
> L-items directly (commits land in `~/Development/skills`).

The whole `writing-great-skills` skill **postdates grove's pin** (created
`bc4cf90`), so relative to Linkuistics' `authoring-conventions` (built against
the older material) this is a large new upstream treatment.

### L1 — `authoring-conventions`: add **Negation** — **RECOMMEND**

`writing-great-skills/GLOSSARY.md`: *"**Negation.** Steering by prohibition …
drags the forbidden behaviour into context and makes it more available, not
less … Cure: prompt the positive."* `authoring-conventions` already has a
"match the form to the failure" bullet citing superpowers; Negation is a
sharper, more general statement with a mechanism and a crisper cure. One-line
add generalising the existing bullet.

### L2 — `authoring-conventions`: **context-load / cognitive-load + router** vocabulary — **RECOMMEND**

`writing-great-skills/SKILL.md`: model-invoked skills pay *context load*
(description always in-window); user-invoked pay *cognitive load* (the human is
the index); the cure for pile-up is a **router skill**. Names the two costs
`authoring-conventions` currently only gestures at.

### L3 — `authoring-conventions`: **sentence-level no-op hunt** — **RECOMMEND**

`aa7ed40`: *"hunt no-ops sentence by sentence … when one fails, delete the whole
sentence rather than trim words … Be aggressive."* A concrete, testable pruning
procedure that upgrades any "be concise" guidance.

### L4 — Refresh the survey's `mattpocock-S*` citations — **CONSIDER**

The survey's mattpocock citations predate `writing-great-skills` + its
`GLOSSARY.md` (a ~30-term ubiquitous language for skill-writing) and the
**Leitwort / leading-word** section. Worth a citation refresh so the survey
points at the current canonical source.

### L5 — `codebase-design`: the `DESIGN-IT-TWICE.md` sub-agent workflow — **MAYBE (low priority)**

Linkuistics' own `codebase-design` is a **near-twin** of upstream's and is
equal-or-better (language-neutral vs upstream's TypeScript; inlined vs split).
The one nugget upstream has that Linkuistics states only as a principle: a
concrete **parallel-sub-agent "design it twice"** procedure (spawn agents with
divergent briefs — minimize-interface / maximize-flexibility / optimize-common-
caller / ports-and-adapters). A workflow flourish, not a vocabulary gap.

### Linkuistics — non-actions

| Upstream skill | Why not |
|---|---|
| `codebase-design` (whole) | Linkuistics' version is already equal-or-better. |
| `domain-modeling` | Adopting it means adopting the whole `CONTEXT.md` + `docs/adr/` convention it assumes — real coupling; and it is the discipline grove already froze. |
| `decision-records` re-derive | Upstream ADR-FORMAT is byte-identical to the frozen snapshot; provenance still valid (optionally note it moved to `domain-modeling/`). |
| `git-guardrails-claude-code` | Real overlap with Linkuistics' `guardrail`, but opposite philosophy — pocock **blocks + persists to settings.json**; Linkuistics **ask-and-scope, session-only**, deliberately superior for avoiding alarm fatigue. Optional contrast note only. |
| in-progress `writing-beats/fragments/shape` | Prose-writing skills, not skill-authoring. Out of domain. |
| `setup-matt-pocock-skills` | Repo-config scaffolder for pocock's own suite; no Linkuistics analogue. |

---

## Recommendation matrix

| # | Finding | Target | Verdict |
|---|---|---|---|
| G1 | Self-grilling bug fix (fact-vs-decision) | grove `grilling.md` | **DO — defect** |
| G2 | Confirmation gate (grove-worded) | grove `grilling.md` | **DO** |
| G3 | Provenance refresh + own-the-fusion note | grove `grilling.md` | **DO** |
| G4 | Decomposition craft (vertical slice, expand→contract, horizon note, durable briefs) | grove `driving.md` / `BRIEF-FORMAT.md` | **CONSIDER — substantive** |
| G5 | Smaller enrichments (no-fog exit, seams, glossary rule, bewildering clause, Negation lint) | grove docs | **MAYBE** |
| L1 | Negation failure mode | Linkuistics `authoring-conventions` | **RECOMMEND** |
| L2 | context/cognitive load + router | Linkuistics `authoring-conventions` | **RECOMMEND** |
| L3 | Sentence-level no-op hunt | Linkuistics `authoring-conventions` | **RECOMMEND** |
| L4 | Survey citation refresh | Linkuistics research | **CONSIDER** |
| L5 | design-it-twice sub-agent workflow | Linkuistics `codebase-design` | **MAYBE** |

**Bottom line.** grove-side: one genuine **defect** to fix (G1), two small sure
things (G2, G3), and one substantive **methodology enrichment** worth grilling
(G4 — decomposition craft). Everything upstream restructured *around* grove
(the grilling/domain-modeling split, the tracker-based wayfinder, the
CONTEXT/ADR moves) confirms grove's existing bets rather than challenging them.
Linkuistics-side: three clean `authoring-conventions` enrichments from the new
`writing-great-skills` (L1–L3), carried to that repo as recommendations.

# corpus-rule-ownership

## Problem

The installed prompt corpus states the same rule in several files with no
recorded owner. That is not verbosity, and compressing sentences would not touch
it. The cause is structural: **the corpus has four competing filing axes**, and a
rule usually belongs to all of them.

Take the placement rule for an `integrate-review-*` step. It is a **loop-step**
rule (it governs Decompose), a **kind** rule (it binds the five `review-*`
kinds), a **filename-grammar** rule (it is about positions among siblings), and a
**habit** (it is a judgement a session exercises). Filed by topic, it lands in
every file that owns one of those topics — and it has. It is stated at length in
`content/SKILL.md`, `content/references/decompose.md`, `content/driving.md`,
`content/TASK-FORMAT.md`, `docs/specs/doubt-grove-review-mechanics.md` and
`CONTEXT.md`: six statements of one rule, each carrying its own reasoning, each
free to drift.

The same shape produces the corpus's live contradictions. `references/design.md`
says an ADR is raised for a decision "hard to reverse, surprising, **or** a real
trade-off"; `grilling.md` says all three must hold. Nothing decides between them
because nothing says which file owns the test. And `references/planning.md`
states the working-increments rule **twice in its own body**, with
`references/execute.md` stating it a third time — a file duplicating itself is
what "no placement rule" looks like at its clearest.

The only shape check standing over any of this is a 500-line ceiling on
`SKILL.md` whose own doc comment concedes it "establishes nothing semantic".

Two constraints bound any answer. Normative operational material must stay
**embedded under `content/` and reachable by an installed session** — only
`content/` is swept into the harness skill directories, so a rule moved to
`docs/` is unreachable to every session outside this repository and has been
deleted rather than rehomed. And `src/prompt.rs`'s three-part runtime is fixed:
this design changes the prose the guaranteed core points *at*, never the runtime
that points.

## Solution

**File a rule by its load predicate, not by its topic.**

For a rule R, let **Bound(R)** be the set of session kinds that must obey it.

> **The placement function.** R's canonical source is the *narrowest* file that
> every session in Bound(R) already opens on its normal path — and no other file
> states R.

That yields exactly four cases, and they are the four separations the
requirements asked for, derived rather than asserted:

| Bound(R) | Canonical source | Separation |
|---|---|---|
| all nineteen kinds | the **loop-step reference** for the step R governs | policy |
| one kind, or one kind family | that **kind reference** | policy |
| any session about to write one artifact | that artifact's **format file** | format grammar |
| ∅ — R constrains no session's behaviour | **not normative**; leaves `content/` for `docs/` | rationale and history |

A fifth case is not a case: a **command fact** — what a verb does, what a flag
defaults to, what the config schema accepts — is owned by the CLI that generates
it. Prose may name a verb and say *when* to reach for it; transcribing *what it
does* creates a second source that changes on a release the corpus does not see.

### The two registers, and what a mirror may be

The corpus already has a condition/procedure split. This design promotes it from
a description of the corpus into the rule that governs mirrors.

- The **procedure register** — reference files, format files — carries how to act
  once a rule applies. **Exactly one file per rule. Never mirrored.**
- The **condition register** — `content/SKILL.md` alone — carries *that a
  situation exists calling for something other than what this session is doing*,
  in one sentence, naming the file with the procedure.

**A permitted mirror has exactly one legal shape: a condition in `SKILL.md`
naming its procedure's file.** Any third statement of a rule is a defect, and a
second *procedure-register* statement is a defect even when the two currently
agree.

Two corollaries follow, and both are load-bearing:

- **A rule whose Bound is one kind or one family gets no `SKILL.md` mirror at
  all.** The driver resolved the kind before the session existed and named that
  kind's reference file in `${prompt}`, so the session performs no selection and
  has nothing to be triggered into. Mirroring a per-kind rule into `SKILL.md`
  buys nothing and costs a drift surface. This is why `SKILL.md` shrinks: most of
  what it carries today is per-kind.
- **The condition register may not carry a test, a threshold, a list, or a
  procedure** — those are the things a mirror cannot hold without becoming a
  second statement. "ADRs are raised sparingly" is not a condition, it is a
  *paraphrased test*, and paraphrasing a test is how the AND/OR contradiction was
  born.

### Reachability, and why the hard boundary needs no separate rule

A file is **reachable** by an installed session iff it is under `content/` and is
named — by embed-relative path — from a file already on that session's path. The
static path is fixed by `src/prompt.rs`: the guaranteed core, `SKILL.md`, and
`reference_file(kind)`. Everything else is reached because a condition fired.

`docs/` is not provisioned. A `docs/` path is therefore reachable **only** to a
session working inside this repository, and to no session on any other project.
So the hard boundary — *normative material stays embedded* — is not an extra rule
to remember; it is the placement function's fourth case read backwards:

> A rule may move to `docs/` **iff** Bound(R) = ∅. If any session must obey it,
> `docs/` is not a home.

Every relocation this design authorises is checked against that one test, and the
test is stated per relocation in *What moves, and where* below.

### The inventory is a document, not a grammar

The inventory below gives every rule a stable ID that prose and tests may cite.
**Those IDs are not markers, and none is written into `content/`.** The corpus
carried 140 HTML-comment unit markers with a build gate over them; that
classification was scaffolding, it did its work, and it was deleted. Reviving it
under a new name would re-buy a parser for a claim a parser cannot decide.

Enforcement is therefore **per rule, by the instrument that fits that rule** —
a behavioural eval for a rule about what a session does, a targeted
phrase-uniqueness sweep for a rule whose wording is distinctive, a word budget
for a loaded path — and never a universal partition check. The `tests` column
names the instrument; it does not name a grammar.

## Decisions

### The rule grain

One row per rule **that has, or could have, a mirror** — a rule a session could
violate, stated at the coarsest grain at which the violation is still nameable.
Finer than that and the inventory becomes a parser's input; coarser and a mirror
hides inside a row.

Completeness is checkable **per owner file**: every normative sentence in a
`content/` file resolves to a row whose canonical source is that file, or to a
row that names it as a mirror, or is marked for relocation.

### Load predicate notation

- `always(K)` — on the static loaded path of every kind in `K`; the file is
  `SKILL.md` or `reference_file(k)`.
- `on(<condition>)` — reached when a named condition fires; the condition itself
  lives in the condition register and is on a static path.

### Test class

- **B** — behavioural: observable in a session's conduct, green before and after
  the rewrite. This is `behavior-evals-k3`'s scope.
- **S** — structural: a single-source or budget assertion over the corpus, green
  only **after** the rewrite that lands it. Owned by the leaf that lands that
  rewrite, never by `behavior-evals-k3`.

The split is not cosmetic. `behavior-evals-k3` is chartered to be green before
the rewrites start; handing it a single-source assertion would charter it to
fail. Each rewriting leaf lands its own **S** tests with the rewrite they
describe.

---

### The inventory

Grouped by canonical source, so completeness is checkable a file at a time.

#### `content/references/grove.md` — what a grove is

| rule | permitted mirror | load | test |
|---|---|---|---|
| `spine-artifacts-not-state` — no phase, session-log or status file; the tree is the only state | `SKILL.md` spine list | `always(19)` | B |
| `spine-read-dont-run` — a session bootstraps by reading markdown; no script must succeed first | `SKILL.md` | `always(19)` | B |
| `spine-suggested-shape` — task files and briefs are freeform; nothing validates them | `SKILL.md` | `always(19)` | S |
| `spine-lazy-and-optional` — an artifact is created only when it earns its place; lazy is just-in-time, not few | `SKILL.md` | `always(19)` | B |
| `spine-guides-not-gates` — grove never refuses to proceed | `SKILL.md` | `always(19)` | B |
| `spine-walk-away-able` — delete the skill and `.grove/` is still legible notes | `SKILL.md` | `always(19)` | S |
| `spine-one-page` — if the loop does not fit a page, cut | `SKILL.md` | `always(19)` | S |
| `glossary-is-the-forcing-function` — terms are resolved into `CONTEXT.md` **inline**, never batched | `SKILL.md` | `always(19)` | B |
| `plugin-prerequisite` — grove requires but does not provision `linkuistics`; each deferral states what binds without it | `SKILL.md` | `always(19)` | S |

**Relocate:** grove.md's *Specs* section restates `spec-membership-test` and
`spec-grain-rule`. Bound ≠ ∅, so they do not leave `content/` — they collapse
into `SPEC-FORMAT.md`, their owner, and grove.md keeps a pointer.

#### `content/references/driver.md` — how this session was launched

| rule | permitted mirror | load | test |
|---|---|---|---|
| `mandate-is-authoritative` — the driver's single pre-session pick is the mandate; nothing modulates it | `SKILL.md` | `always(19)` | B |
| `no-second-pick` — `grove-llm pick` is a diagnostic, not this session's dispatcher; on disagreement the mandate wins | `SKILL.md` | `always(19)` | B |
| `pick-walk-order` — first live leaf in pre-order; briefs, terminal leaves and foreign files skipped; `finish` passed over while ordinary work is live | none | `on(reading the tree)` | B |
| `one-configuration` — one complete command template per kind; nothing else routes a session | none | `on(asking how a session is launched)` | S |
| `restart-equals-continuation` — the loop holds no state; a task that dies before its commit boundary is simply redone | none | `on(resuming)` | B |
| `scaffold-is-the-drivers` — a session never scaffolds `.grove/`; it starts at Bootstrap like every other | `SKILL.md` | `always(19)` | B |
| `session-name-suggested-once` — suggest `/rename` once if the template passes no `${session_name}`, then move on | none | `on(the name does not match)` | B |

#### `content/references/bootstrap.md` — assembling the mandate

| rule | permitted mirror | load | test |
|---|---|---|---|
| `bootstrap-order` — resolve the handle, then glossary → cited ADRs → brief chain root→leaf → task file | `SKILL.md` | `always(19)` | B |
| `stale-launch-stops` — a handle resolving to nothing or to a terminal leaf is a stale launch, not work to redo | `SKILL.md` | `always(19)` | B |
| `no-reflex-reading` — that assembled context is the whole mandate; read nothing else by reflex | `SKILL.md` | `always(19)` | B |

#### `content/references/execute.md` — doing the work

`execute.md` sheds *What each kind produces* entirely: it is a nineteen-kind
summary of material the ten kind references own, and the driver already routed
the session to its own. What it keeps is what genuinely binds all nineteen.

| rule | permitted mirror | load | test |
|---|---|---|---|
| `review-budget` — a picked plain producer may materialise at most **one** in-session reviewer across the whole leaf; a second need is a `review-*` leaf | `SKILL.md` | `always(19)` | B |
| `review-budget-predicate` — the allowance applies only to a session the driver mandated **and** that adopted the mandate by running Bootstrap | none | `on(considering a reviewer)` | B |
| `review-budget-by-kind` — the per-kind allowances (reviewed producer: none; `review-*`: none; `integrate-review-*`: one narrow; research trio: none) | none | `on(considering a reviewer)` | B |
| `escalated-review-routes-through-config` — once review is a leaf, grove owns the route; do not add a competing in-session reviewer | none | `on(a review leaf exists)` | B |
| `records-are-current-state` — a session that *changes* a recorded decision reworks the set in place; never append a superseding record | `SKILL.md` | `always(19)` | B |
| `adr-raised-under-a-test` — an ADR is raised only when `ADR-FORMAT.md`'s three-part test holds | `SKILL.md` (naming the file, **not** the test) | `always(19)` | B, S |
| `spec-at-an-agreement-point` — a spec is written only at a genuine agreement point, by `design` | `SKILL.md` | `always(19)` | B |

#### `content/references/decompose.md` — growing the tree

| rule | permitted mirror | load | test |
|---|---|---|---|
| `externalize-by-default` — work that does not serve this leaf's stated goal goes to the tree, never inline | `SKILL.md` | `always(19)` | B |
| `fits-this-session` — the bar is "fits this session", not "I can finish it" | `SKILL.md` | `always(19)` | B |
| `bigger-than-brief-decomposes` — a leaf that proves bigger becomes a node; do only the first child | `SKILL.md` | `always(19)` | B |
| `vertical-slice` — a child leaf cuts a narrow complete path, demoable without waiting on a sibling | none | `on(cutting children)` | B |
| `wide-refactor-expand-contract` — a fan-out refactor sequences expand → migrate → contract, one leaf per stage | none | `on(cutting children)` | B |
| `chain-is-lazy` — each step of a review chain is cut by the session before it, only if required | `SKILL.md` | `always(19)` | B |
| `pair-is-eager` — a vendor pair lands in one call or not at all | `SKILL.md` | `always(19)` | B |
| `creating-session-writes-the-body` — the session that knows why a step is needed writes its body | none | `on(cutting a step)` | B |
| `name-step-kind-off-the-producer` — `review-<producer>` for the producer that actually ran | none | `on(cutting a step)` | B |
| `integration-placement` — `leaf-insert` at the first sibling **entry** after the review whose subtree still holds live work; `leaf-add` when nothing blocks | `SKILL.md` | `always(19)` | B |
| `no-adjacency-exception` — there is no check an exception could perform; a session that departs owns the drift | `SKILL.md` | `always(19)` | S |
| `fog-or-ticket` — a question you can state precisely earns a leaf now; one you cannot stays a horizon note | `SKILL.md` | `always(19)` | B |
| `grow-verbs-are-working-tree-only` — the enclosing task's commit folds them in | none | `on(growing the tree)` | B |

**Relocate:** decompose.md's transcription of `leaf-decompose`'s mechanics — what
the verb moves, retitles and creates — is a **command fact**. Bound ≠ ∅ for
*when* to run it, so the when stays; the *what it does* goes to `grove-llm
leaf-decompose --help`, which the corpus names rather than transcribes.

#### `content/references/retire.md` — ending a leaf

| rule | permitted mirror | load | test |
|---|---|---|---|
| `retire-before-commit` — the rename must land inside the task's own commit | `SKILL.md` | `always(19)` | B |
| `retirement-is-filename-only` — one filename and nothing else; not the body, not a sibling, not an ancestor | `SKILL.md` | `always(19)` | B |
| `pruning-is-hitl` — an agent never prunes on its own; an AFK session that finds its path decided against says so and stops | `SKILL.md` | `always(19)` | B |
| `node-close-is-implicit` — a node is never marked; its done-ness is the absence of a live child | `SKILL.md` | `always(19)` | B |
| `node-close-four-steps` — check `Done when`, `leaf-add` the named gap, escalate an unnameable one, promote and report | none | `on(a node has no live leaf left)` | B |
| `cascade-is-silent` — the close recurses upward asking the human nothing | `SKILL.md` | `always(19)` | B |
| `reconcile-records-at-retire` — retirement is where the ADR set is reworked and dangling citations fixed | none | `on(retiring)` | B |
| `triage-picks-the-verb` — *not now* → reorder; *not ours* → an issue; *decided against* → prune | `SKILL.md` | `always(19)` | B |
| `no-fourth-status` — no `blocked`, `deferred` or `superseded`; a leaf in doubt gets no status word | `SKILL.md` | `always(19)` | S |

#### `content/references/commit.md` — the task boundary

| rule | permitted mirror | load | test |
|---|---|---|---|
| `one-focused-commit` — artifact + grow-verb writes + `DONE` rename + whatever the cascade promoted, together | `SKILL.md` | `always(19)` | B |
| `name-by-handle` — name the work item, and each closed node, by `<slug>-k<key>`, never by position or path | `SKILL.md` | `always(19)` | B |
| `jj-seal` — in a jj tree, `jj new` **after** describing, once the rename has landed | none | `on(the stated VCS is jj)` | B |
| `stated-vcs-is-definitive` — the driver's statement wins; do not re-derive, and disregard a harness banner | `SKILL.md` | `always(19)` | B |

#### `content/references/finish.md` and `content/SIGNAL-FINISH.md`

| rule | permitted mirror | load | test |
|---|---|---|---|
| `finish-is-the-drivers-to-discover` — a session never concludes the grove is finished; the driver says so by launching `finish` | `SKILL.md` | `always(19)` | B |
| `finish-confirmation-gate` — propose, and wait for explicit human confirmation before any teardown; with no human, report the plan | `SKILL.md` | `always({finish})` | B |
| `teardown-via-finish-commit` — never delete `.grove/` by hand; `grove-llm finish-commit <handle>` | none | `always({finish})` | B |
| `absent-tree-proves-nothing` — an absent `.grove/` never proves teardown succeeded | none | `always({finish})` | B |
| `recovery-pending-stops` — hand a `Recovery pending` diagnostic to the human; never rewrite history to clear it | none | `always({finish})` | B |
| `nothing-after-finish` — branch integration and worktree teardown are not grove workflow | none | `always({finish})` | S |
| `finish-three-endings` — teardown → `complete --done`; externalised work → `complete`; declined → no signal | **none — byte-frozen and inlined into `${prompt}`** | `always({finish})` | B |

#### `content/SIGNAL.md`

| rule | permitted mirror | load | test |
|---|---|---|---|
| `signal-is-the-last-action` — `grove-llm complete` last, then nothing else; ending without signalling stops the loop | **none — byte-frozen and inlined into `${prompt}`** | `always(18)` | B |

Both signal files are **out of scope for every rewriting leaf**. They are the one
surface where a wording change is unrecoverable mid-loop, and the guaranteed core
inlines their bytes verbatim, so an edit ships to `${prompt}` and the skill at
once with no channel left to correct it.

#### The ten kind references

Incremental by construction under the placement function: a kind reference states
what is true of **that kind and no sibling**, and states nothing a loop-step or
format file owns.

| kind reference | rules it owns |
|---|---|
| `requirements.md` | `requirements-establishes-what`; `grilling-threshold`; `pre-decided-is-not-a-grilling-question`; `when-not-to-start-a-grove` |
| `design.md` | `design-deliverable` (a spec, an ADR set, or both); `design-does-not-cut-impl-leaves` |
| `planning.md` | `planning-grows-generatively`; `working-increments-before-slices`; `planning-writes-the-briefs` |
| `prototype.md` | `prototype-is-throwaway` (the reaction is the deliverable; polish is a defect) |
| `impl.md` | `impl-ships`; `cite-framework-decisions-to-source`; `verify-repo-claims-with-controls` |
| `review.md` | `review-is-inspection-only`; `review-output-is-findings-only`; `the-five-reads-differ` |
| `integrate-review.md` | `triage-four-ways`; `what-each-integration-may-change` |
| `research.md` | `citation-per-failure-mode-claim`; `silence-is-a-finding`; `both-researchers-get-one-brief`; `researchers-are-not-adversarial` |
| `combine-research.md` | `agreement-without-independent-primary-sourcing-is-a-red-flag` |
| `finish.md` | the `finish` rows above |

Two rules move **into** kind references from `driving.md`, because their Bound is
exactly `{impl}` and today they sit in a file no `impl` session is routed to:
`cite-framework-decisions-to-source` and `verify-repo-claims-with-controls`.

#### The format files

| file | rules it owns |
|---|---|
| `TASK-FORMAT.md` | `leaf-name-grammar` (five fields, all parsed); `kind-set-is-closed-at-nineteen`; `handle-is-slug-key`; `header-is-position-free`; `body-carries-no-launch-metadata`; `declaration-lines-are-convention` |
| `BRIEF-FORMAT.md` | `every-node-carries-a-brief`; `brief-is-process-scaffolding`; `briefs-inherit`; `brief-content-is-durable`; `horizon-note-shape` |
| `ADR-FORMAT.md` | **`adr-when-to-write`** (the three-part AND test, Grove-local); `adr-minimal-template`; `adr-placement-and-slug-identity`; `adr-set-is-minimum-coherent`; `adr-split-is-conditional-on-repo-shape` |
| `SPEC-FORMAT.md` | `spec-membership-test`; `spec-grain-rule`; `spec-set-is-current-state`; `spec-synthesises-never-re-interviews`; `spec-is-behavioural-not-procedural`; `test-seams-agreed-and-recorded` |
| `CONTEXT-FORMAT.md` | `glossary-is-only-a-glossary`; `terms-are-context-specific`; `context-map-when-multiple` |
| `grilling.md` | `grilling-procedure` — one question at a time, a recommended answer for each, decisions are the human's |

**`TASK-FORMAT.md` sheds its policy.** Under the four separations it is format
grammar, so the composition shapes, the doubt budget table, the kind disciplines
and *A leaf never names a harness* leave it for `decompose.md`, `execute.md`, the
kind references and `driver.md` respectively. What remains is the name grammar,
the kind list, the suggested body shape and the two declaration lines — the
things that constrain bytes on disk.

---

### The two contradictions, resolved

#### 1. The grilling threshold

**Canonical statement.** A `requirements` session **always** establishes *what*
should be built. The **full one-question-at-a-time grilling procedure** runs
**only** when three or more interdependent questions are open. Below that
threshold the session records the decisions and proceeds; staging an interview
over settled ground costs a human's attention and returns nothing.

- **Canonical source:** `content/references/requirements.md` — the narrowest file
  every bound session opens, and no unbound session does.
- **Load predicate:** `always({requirements})`.
- **Permitted mirrors:** **none.** Bound is one kind, and the driver already
  routed that kind to this file.
- **`grilling.md` is a procedure, not a standing instruction.** It is bundled
  third-party text, so its entry condition is stated above the bundled body in
  Grove's own voice — as its provenance comments already are — leaving the
  `<what-to-do>` block byte-intact.

#### 2. The ADR test

**Canonical statement.** An ADR is raised only when **all three** hold: hard to
reverse **and** surprising without context **and** the result of a real
trade-off.

- **Canonical source:** `content/ADR-FORMAT.md`. Bound is all nineteen kinds
  (any kind may raise one), and this is the file every session about to write an
  ADR already opens.
- **Load predicate:** `on(considering an ADR)`, the condition itself carried by
  `SKILL.md` and `references/execute.md`.
- **Permitted mirrors:** a condition **naming the file**, never a paraphrase of
  the test. "Raised sparingly" is not a permitted mirror — it is a looser test
  wearing a condition's clothes, and it is what let the OR-form survive.
- **The test is stated locally**, not only cited (deferral row 1 below). A cited
  test is no bar of Grove's own, and the OR-form is what a cited-only test
  produced.

### Plugin deferral policy

Grove requires the `linkuistics` plugin and does not provision it, so a corpus
that cites it without saying what binds in its absence depends on it
**silently**. The corpus carries **14 distinct (file, skill) deferrals across 9
files** — enumerated below, and the requirements' "7 files" is a miscount.

The generating question is per deferral and has one form: **does the absence
change what a session writes, or only how well it writes it?** Absence that
changes *what* is owned locally; absence that changes *how well* is deferred,
and the deferring sentence must say so.

| # | file | skill | deferred capability | decision |
|---|---|---|---|---|
| 1 | `ADR-FORMAT.md` | `decision-records` | when-to-write test | **own locally** — three bullets |
| 2 | `ADR-FORMAT.md` | `decision-records` | minimum-coherent-set discipline | **own locally** (already stated); citation becomes attribution |
| 3 | `SKILL.md` | `decision-records` | ADR philosophy (artifact table row) | **defer**, stating that `ADR-FORMAT.md` holds what binds |
| 4 | `SKILL.md` | `codebase-design` | what a test seam is | **defer** with a one-line local gloss |
| 5 | `SKILL.md` | `using-jujutsu` | working-copy-as-commit lane | **defer**, stating `commit.md` is sufficient alone |
| 6 | `references/grove.md` | `decision-records` | ADR philosophy | **defer** (same as 3) |
| 7 | `references/grove.md` | `codebase-design` | seam vocabulary | **defer** (same as 4) |
| 8 | `references/grove.md` | `using-jujutsu` | the jj lane | **defer** (same as 5) |
| 9 | `references/execute.md` | `decision-records` | philosophy, format, template | **own the template locally**; defer the philosophy |
| 10 | `references/commit.md` | `using-jujutsu` | the jj lane | **own locally** — `jj describe` + `jj new` and why, already there; add that this is sufficient without the plugin |
| 11 | `references/retire.md` | `decision-records` | never-append-a-superseding-record | **own locally** (already stated) |
| 12 | `SPEC-FORMAT.md` | `codebase-design` | what a seam is / how to judge one | **defer** with a one-line local gloss; the operative rules (prefer existing, propose highest, drive the count to one) are already Grove-local |
| 13 | `grilling.md` | `decision-records` | when-to-write test | **own locally** — cite `ADR-FORMAT.md`, not the plugin |
| 14 | `grilling.md` | `codebase-design` | seam vocabulary | **defer** (same as 12) |

Two of these are load-bearing enough to name individually. **Row 1** is the
sharpest: with the test only cited, Grove had *no* bar of its own, and the
AND/OR contradiction is exactly what a cited-only test produces. **Row 10** is
the one whose absence is unrecoverable — a session that commits with git in a jj
tree bypasses the operation log — and today a reader cannot tell whether
`commit.md` is complete or a teaser. Saying so is the whole fix; the commands are
already there.

**The discharge test for `plugin-fallback-k9`:** every deferring sentence states
what binds without the plugin. A citation that leaves that unstated is the
silent dependency, whichever way the row was decided.

### A habits file is not an embedded file

The function's sharpest output concerns `content/driving.md`, the largest file in
the corpus. Its normative rules all appear in the inventory above under owners
elsewhere, and two of them — `impl`'s source-citation and repo-claim disciplines
— sit today in a file **no `impl` session is ever routed to**, which is the
reachability test failing inside `content/` rather than at its edge. What remains
after those rules leave has `Bound(R)` = ∅.

**So a habits-and-rationale file does not survive as an embedded file.** This is
the function's answer rather than a judgement about size: a file most of whose
bytes lie off every session's loaded path is precisely the shape the loaded-path
measure exists to make visible, and the rules a session actually needed were
unreachable while it stayed.

The same test moves every argument, worked example, provenance note and history
out of `content/` — the one case where `docs/` is a legal destination, because
Bound is empty by construction for material that constrains no session.

## Test seams

- **Per-rule, never universal.** No parser over the corpus and no marker grammar.
  Each rule's instrument is named in its row and lands with the rewrite that
  homes it.
- **The static loaded path is computed from `src/prompt.rs`**, not transcribed:
  `reference_file(kind)` is already an exhaustive match, and a per-kind budget
  test walks it. That is the seam `loaded-path-budgets-k10` builds on, and it
  costs no new production code.
- **Single-source assertions are phrase-scoped, with controls.** A rule whose
  wording is distinctive gets a normalised sweep (emphasis stripped, whitespace
  collapsed) asserting exactly one procedure-register file states it — with a
  positive control that the sweep finds a phrase known present, and a
  cross-tree control that it still finds the class where it legitimately lives.
  An unnormalised sweep silently misses a wrapped or emphasised match; that
  failure was reproduced while writing this spec.
- **Behavioural evals assert conduct, not contents.** *No second pick*, *no VCS
  reprobe*, *stale launch stops*, *the interview threshold*, *the decomposition
  boundary*, *human-only pruning*, *retire → commit → complete*, *the review
  budget*, *all three finish-signal outcomes*.
- **The two signal files are asserted byte-identical** to their state at this
  grove's start, for the whole workstream.
- **The existing embed checks stay as they are** — the linked embed matching
  `content/` on disk, the routing table resolving, and the instructed-verb
  comparison. The last becomes *more* load-bearing as prose stops transcribing
  command facts and starts naming verbs.

## Out of scope

- **`src/prompt.rs`.** Its three-part architecture — load instruction, bare
  runtime facts, byte-exact terminal signal — is a fixed constraint. This design
  changes the prose it points at.
- **Mechanising command facts beyond naming them.** Which facts qualify is
  decided above (verb mechanics, flag defaults, config schema); *building* the
  generation is a separate decision with its own leaf, if it is ever worth it.
- **A marker grammar, a build gate, or any universal partition check** over the
  corpus. That was tried, did its work, and was deleted.
- **The plugin skills themselves** — `plugin-skills-k7` and `harness-compat-k8`
  own those; this design touches only how `content/` cites them.
- **Any change to what `${prompt}` carries.** The too-late test owns that
  boundary and is unchanged.

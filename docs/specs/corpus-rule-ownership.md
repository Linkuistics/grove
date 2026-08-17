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

**File a rule by when a session meets it, not by what it is about.**

Two facts are recorded per rule, and together they decide its home.

- **Bound(R)** — the set of session kinds that must obey R.
- **Occasion(R)** — *when* in a session's work R applies. Exactly one of five
  values:
  - `orientation` — R must already be held when the session opens the skill;
    there is no earlier moment at which a condition could send it anywhere.
  - `launch` — R is about how this session was launched, picked and configured.
  - `step:<S>` — R applies at one step of the loop: `Bootstrap`, `Execute`,
    `Decompose`, `Retire`, `Commit`, `Finish`.
  - `artifact:<A>` — R applies when the session is about to write or change one
    durable artifact: `task`, `brief`, `adr`, `spec`, `glossary`.
  - `none` — R constrains no session's conduct at any moment.

> **The placement function.** Apply these in order; **the first match wins**, and
> that ordering is the tie-break.
>
> 1. `Bound(R) = ∅` or `Occasion(R) = none` → **not normative.** Leaves
>    `content/`.
> 2. `Bound(R)` is one kind or one kind family → that **kind reference**.
> 3. `Occasion(R) = artifact:A` → **A's format file**.
> 4. `Occasion(R) = launch` → **`references/driver.md`**.
> 5. `Occasion(R) = step:S` → the **loop-step reference for S**.
> 6. `Occasion(R) = orientation` → **`content/SKILL.md`**.
>
> No other file states R.

### Why the input had to change

The predecessor of this function took `Bound(R)` alone and mapped "all nineteen
kinds" to *the loop-step reference for the step R governs*. That is not a
function of its stated input twice over, and both failures were load-bearing.

**It hid a second input.** *Which step does R govern?* is a judgement, and it is
the same kind of judgement as *which topic does R belong to* — the thing the
function was introduced to remove. Recording it as `Occasion(R)` does not make
the judgement disappear; it makes it **one judgement per rule, stated in the
row, and checkable by a reader** — *does a session actually meet this at that
moment?* That is the honest claim, and it is weaker than the claim the
superseded text made. What is genuinely computed is the *consequence*: given the
pair, the owner follows with no further argument.

**It contradicted itself on the ADR test.** `adr-when-to-write` binds all
nineteen kinds, so the old table sent it to a loop-step reference; the design's
own resolution put it in `ADR-FORMAT.md`, an artifact format file. Under the
ordering above, rule 3 fires before rule 5 and the answer is `ADR-FORMAT.md` with
nothing left to decide. The same ordering resolves the twin case:
`records-are-current-state` is `artifact:adr` and `artifact:spec`, so it is **two
rows owned by the two format files**, and `references/execute.md` states neither.

**And it was inconsistent with the fixed runtime.** `always(K)` was defined as a
*static* path whose file is `SKILL.md` or `reference_file(k)`, while rows owned by
`execute.md`, `decompose.md`, `retire.md` and `commit.md` were labelled
`always(19)`. `src/prompt.rs:136` maps kinds only to the ten kind references, so a
loop-step reference is never on a static path. Read literally, therefore, "the
narrowest file every bound session already opens" sends **every** all-nineteen
rule to `SKILL.md` — which is the opposite of what the design intends and what
the 700–900-word target permits. Rules 4–6 are what actually resolve those rows,
and the load notation below states static and conditional paths as the different
things they are.

### Load predicate notation

- `static(K)` — on the static loaded path of every kind in `K`. **Only three
  things can be static**: the guaranteed core, `SKILL.md`, and
  `reference_file(k)`. Nothing else may carry this value.
- `on(<trigger>) @ <file>` — conditional. `<file>` is the file whose sentence
  fires the trigger, and it is part of the predicate, not a note.

**The reachability rule this makes checkable.** A conditional row's `@ <file>`
must itself be static, or be the owner of a row that is itself reachable — a
chain, terminating at a static path, with no cycles. So `BRIEF-FORMAT.md` is
legitimately reached from `decompose.md` rather than from `SKILL.md`, and the
chain is what keeps `SKILL.md` a page. A row whose chain does not terminate is
an **unreachable rule**: present in `content/`, deleted in effect. That failure
is what `driving.md` already does to two `impl` rules today, inside `content/`
rather than at its edge.

### Reachability, and why the hard boundary needs no separate rule

`docs/` is not provisioned. A `docs/` path is therefore reachable **only** to a
session working inside this repository, and to no session on any other project.
So the hard boundary — *normative material stays embedded* — is not an extra rule
to remember; it is the placement function's first case read backwards:

> A rule may move to `docs/` **iff** `Bound(R) = ∅` or `Occasion(R) = none`. If
> any session must obey it at any moment, `docs/` is not a home.

Every relocation this design authorises is checked against that test, and the
test is stated per relocation in *What moves, and where* below.

### The two registers, and the three mirror classes

The corpus already has a condition/procedure split. This design promotes it from
a description of the corpus into the rule that governs restatement.

- The **procedure register** — reference files, format files — carries how to act
  once a rule applies. **Exactly one file per rule. Never mirrored.**
- The **condition register** — `content/SKILL.md` alone — carries *that a
  situation exists calling for something other than what this session is doing*.

A single "a mirror is one sentence naming a file" rule does not survive contact
with the corpus, and the superseded text proved it: the same document forbade a
mirror carrying "a test, a threshold, a list, or a procedure" and then permitted
the seven-item spine list, the bootstrap order, the one-reviewer allowance, the
integration-placement test and the triage mapping in `SKILL.md`. The requirements
independently ask `SKILL.md` to *carry* the bootstrap order, which a pointer
cannot do.

The defect was treating `SKILL.md`'s relationship to a rule as one thing. It is
three, and **every inventory row states which**:

| class | what `SKILL.md` carries | when it is legal |
|---|---|---|
| **`own`** | the whole rule; `SKILL.md` is the canonical source | `Occasion(R) = orientation` — the rule's whole content *is* its trigger, and no procedure remains to defer. A procedure file may explain it; none may restate it. |
| **`trigger`** | **one sentence, ≤25 words**: the situation, a single-clause obligation, and the owner file's path | `Bound(R)` is all nineteen and `Occasion(R)` is a step, an artifact or `launch`. May not carry a threshold, a branch, an enumeration or steps. |
| **`none`** | nothing at all | mandatory whenever `Bound(R)` is one kind or one family (the driver already named that kind's reference, so there is nothing to trigger the session into); the default for any rule whose trigger chain runs through another conditional file. |

Any statement of a rule outside its owner and its declared class is a defect, and
a second *procedure-register* statement is a defect even when the two currently
agree.

**Two corollaries survive from the superseded text, unchanged.** A per-kind rule
gets no `SKILL.md` presence at all — which is most of what the file carries today
and most of why it shrinks. And a paraphrased test is never a legal `trigger`:
"ADRs are raised sparingly" reads as a condition and *is* a looser test, which is
how the AND/OR contradiction was born.

### What `SKILL.md` can hold, arithmetically

The 700–900-word target is only credible with the classes counted, so this design
states the budget rather than assuming it. `skill-router-k4` lands it as an
assertion.

| part | budget |
|---|---|
| frontmatter `description`, title, intro, section headings | ≤120 words |
| the **7 `own` rows** (routing table, spine, bootstrap order, mandate, no second pick, stated VCS, HITL/AFK mark) | ≤310 words |
| **19 `trigger` sentences**, each ≤25 words | ≤475 words |
| **total** | **700–900 words — the binding constraint** |

The two budgets bind differently and both are asserted. The per-sentence ceiling
of 25 words is a *shape* rule that keeps a trigger from growing into a procedure;
the total is what the requirements set, and at the ceiling the parts sum to 905,
so the total is what actually constrains. A trigger averaging 22 words rather
than 25 lands the file at about 840.

The cap is **19 sentences, not 19 rules**: one sentence covers two rows when the
situation is one situation — retire-before-commit with retirement-is-filename-only,
pruning-is-HITL with no-fourth-status, one-focused-commit with name-by-handle,
node-close with cascade-is-silent, externalize-by-default with
bigger-than-brief-decomposes, the durable artifacts with the plugin prerequisite,
the ADR test with the spec agreement point, and the ADR set's current-state rule
with the spec set's. **Twenty-seven `trigger` rows therefore resolve to nineteen
sentences**, and the inventory marks no more. That is
what makes the target reachable rather than aspirational: the superseded inventory
carried 39 `SKILL.md` mirror rows whose rule text alone ran to about 670 words
before any router prose.

**Why the spine is `own` rather than a pointer.** Six other corpus files —
`BRIEF-FORMAT.md`, `SPEC-FORMAT.md`, `references/driver.md`,
`references/finish.md`, `references/decompose.md` and `references/grove.md` —
cite the constraints **by number** (11 citations, verified with a normalised
sweep whose cross-tree control finds the same class in `docs/` and `.grove/`).
Every one of those files is conditionally loaded, so a session reading
"constraint 4" in one of them can only resolve the number from a static path. The
numbered list is therefore on `SKILL.md`'s static path by derivation, and it is
**one row, not seven** — the list is cited as a unit, so seven rows would be
seven mirrors of something that is one rule.

### A command fact is owned by the CLI

A fifth placement case is not a case: **what a verb does, what a flag defaults
to, what the config schema accepts** is owned by the CLI that generates it. Prose
may name a verb and say *when* to reach for it; transcribing *what it does*
creates a second source that changes on a release the corpus does not see. Such
material is marked **relocate → CLI** in the inventory and is not given a rule ID.

### The inventory is a document, not a grammar

The inventory below gives every rule a stable ID that prose and tests may cite.
**Those IDs are not markers, and none is written into `content/`.** The corpus
carried 140 HTML-comment unit markers with a build gate over them; that
classification was scaffolding, it did its work, and it was deleted. Reviving it
under a new name would re-buy a parser for a claim a parser cannot decide.

Enforcement is therefore **per rule, by the instrument that fits that rule** —
a behavioural eval for a rule about what a session does, a targeted
phrase-uniqueness sweep for a rule whose wording is distinctive, a word budget
for a loaded path — and never a universal partition check. The `test` column
names the instrument; it does not name a grammar.

## Decisions

### The rule grain

One row per rule **a session could violate**, stated at the coarsest grain at
which the violation is still nameable. Finer than that and the inventory becomes
a parser's input; coarser and a restatement hides inside a row.

Completeness is checkable **per owner file**: every normative sentence in a
`content/` file resolves to a row whose canonical source is that file, or to a
row that names it as a declared mirror, or is marked for relocation.

### Every row carries five columns, in every table

The requirements ask each normative concept for a rule ID, a canonical source,
permitted mirrors, a load predicate and behavioural tests. That obligation is
schema, not prose: the superseded inventory gave the ten kind references and the
six format files a **two-column** listing — a file and a semicolon-separated rule
name list — which left roughly half the named rules with no mirror class, no load
predicate and no test class at all. **Every table below carries the full record.**
Canonical source is the grouping heading; the four remaining columns are `rule`,
`Bound · Occasion`, `mirror`, `load`, `test`.

### Test class

- **B** — behavioural: observable in a session's conduct.
- **S** — structural: a single-source or budget assertion over the corpus, green
  only **after** the rewrite that lands it.
- **B+S** — both apply, and they are different instruments over the same rule.
- **—** — the row is non-normative and is leaving `content/`; nothing asserts it.

Conduct is the property far more often than the superseded classification
allowed. A uniqueness sweep can show that one sentence owns a rule; it cannot
show that a session obeys it, so `no-adjacency-exception`, `no-fourth-status` and
`nothing-after-finish` are **B+S**, not S-only. **S alone is reserved for a rule
whose property genuinely is the corpus's shape** — the spine's freeform-shape and
walk-away clauses, the one-page budget, the plugin-deferral sweep — and for the
uniqueness half of a B+S pair.

**Which B rows belong to `behavior-evals-k3`, and which do not.** `k3` is
chartered green-before-and-after, so it owns the rows inside **eight of the nine
areas the requirements name**, marked **B★** below: no second pick, no VCS
reprobe, stale launch, the decomposition boundary, human-only pruning,
retire → commit → complete, the review budget, all three finish-signal outcomes.
An area may span several rows — *retire → commit → complete* is three — and only
the rows inside an area are starred.

**The ninth area is the exception, and it has to be.** *The interview threshold*
is `grilling-threshold`, and it is a contradiction being resolved rather than a
behaviour being preserved: the corpus states both forms today, so a test for the
threshold is **red until `kind-references-k5` lands the fix**. Giving it to `k3`
would charter a green-before-and-after leaf to ship red across three leaves. It is
therefore marked **B (k5)** and lands with its fix, which is the disposition
`k3`'s own task file already reasons to.

Every other **B** row is landed by the leaf that rewrites its owner file, with
the rewrite it describes. Handing `k3` the whole B set would balloon it; handing
it any **S** row would charter it to fail.

### A rule is never homeless between two commits

Eight leaves rewrite this corpus in sequence, and a rule whose owner changes is
stated in two files that two different leaves own. **The leaf that adds the new
statement is the leaf that removes the old one.** When the two files belong to
different leaves, the move belongs to the **later** leaf, and the earlier one
leaves the old statement in place and untouched. A rule with no statement in
`content/` between two commits is deleted for every session launched in that
window, and in a meta-grove that window is real sessions.

---

### The inventory

Grouped by canonical source, so completeness is checkable a file at a time.

#### `content/SKILL.md` — the condition register (`own` rows)

These seven are the only rules `SKILL.md` owns. Everything else it says is one
`trigger` sentence for a rule owned elsewhere.

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `kind-routing-table` — ten files serve nineteen kinds; open the one your kind names before acting | 19 · orientation | `own` | `static(19)` | B |
| `spine-seven-constraints` — the numbered non-negotiables, cited by number from six other corpus files; lazy means just-in-time, not few | 19 · orientation | `own` | `static(19)` | B+S |
| `bootstrap-order` — resolve the handle, then glossary → cited ADRs → brief chain root→leaf → task file, and nothing else by reflex | 19 · orientation | `own` | `static(19)` | B |
| `mandate-is-authoritative` — the driver's single pre-session pick is this session's mandate; nothing modulates it | 19 · orientation | `own` | `static(19)` | B★ |
| `no-second-pick` — `grove-llm pick` is a diagnostic, not this session's dispatcher; on disagreement the mandate wins | 19 · orientation | `own` | `static(19)` | B★ |
| `stated-vcs-is-definitive` — the driver's statement wins; do not re-derive it, and disregard a harness banner | 19 · orientation | `own` | `static(19)` | B★ |
| `hitl-afk-mark-predicts` — the mark predicts who is present; it does not permit or forbid, and any kind may stop and ask | 19 · orientation | `own` | `static(19)` | B |

`bootstrap-order`'s Occasion is `orientation` rather than `step:Bootstrap`
because Bootstrap is the *first* thing a session does: there is no earlier
condition that could send it to a file, so a pointer would arrive after the
moment it governs. `references/bootstrap.md` therefore keeps the procedure that
is more than the list — the `brief-chain` walk, the silently skipped level, what
a stale launch looks like — and does not restate the order.

#### `content/references/grove.md` — what a grove is

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `durable-artifact-set` — the glossary, the ADR set, the spec set and the task tree, and which of them outlives the grove | 19 · step:Execute | `trigger` **(the *grove* sentence)** | `on(deciding where an artifact goes) @ SKILL.md` | B |
| `plugin-prerequisite` — grove requires but does not provision `linkuistics`; every deferral states what binds without it | 19 · step:Execute | `trigger` (shares the *grove* sentence) | `on(meeting a plugin citation) @ SKILL.md` | S |
| `build-boundary-is-the-binary` — editing `content/` reaches no session until the binary is rebuilt and installed | 19 · step:Execute | `none` | `on(the grove's subject is grove) @ references/grove.md` | B |

**Relocate → `docs/ARCHITECTURE.md`:** *The seven constraints, argued* and *Why
the glossary is the forcing function* are argument, not rule — the normative
statements are `spine-seven-constraints` (`SKILL.md`) and
`glossary-is-the-forcing-function` (`CONTEXT-FORMAT.md`), and `Occasion = none`
for the argument itself. **Relocate → `SPEC-FORMAT.md`:** grove.md's *Specs*
section restates `spec-membership-test` and `spec-grain-rule`; `Bound ≠ ∅`, so
they collapse into their owner and grove.md keeps a pointer.

#### `content/references/driver.md` — how this session was launched

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `pick-walk-order` — first live leaf in pre-order; briefs, terminal leaves and foreign files skipped; `finish` passed over while ordinary work is live | 19 · launch | `none` | `on(reading the tree) @ SKILL.md` | B |
| `one-configuration` — one complete command template per kind; nothing else routes a session | 19 · launch | `none` | `on(asking how a session is launched) @ SKILL.md` | S |
| `config-edit-lands-next-session` — the file is revalidated before every mutation and launch; an edit reaches the *next* session, and an invalid file leaves this leaf live | 19 · launch | `none` | `on(the configuration is wrong or edited) @ references/driver.md` | B |
| `restart-equals-continuation` — the loop holds no state; a task that dies before its commit boundary is simply redone | 19 · launch | `none` | `on(resuming) @ references/driver.md` | B |
| `scaffold-is-the-drivers` — a session never scaffolds `.grove/`; it starts at Bootstrap like every other, and its commit folds the scaffold in | 19 · launch | `none` | `on(the tree was just created) @ references/driver.md` | B |
| `session-name-suggested-once` — suggest `/rename` once if the template passed no `${session_name}`, then move on; derive `<name>` from the workspace root and `<repo-basename>` from the main repo | 19 · launch | `none` | `on(the session name does not match) @ references/driver.md` | B |
| `migration-is-the-drivers` — the one migratable legacy shape is converted by bare `grove` before your session; the two older layouts are refused, and neither is yours to convert by hand | 19 · launch | `none` | `on(the tree looks legacy) @ references/driver.md` | B |

**Relocate → `references/requirements.md`:** driver.md's *what the scaffold
creates* paragraph tells the bootstrap session to grill, record the outcome in the
root brief, and then either cut the leaves itself or add a `planning` leaf. That
is `requirements`-kind conduct (`Bound = {requirements}`, so rule 2 fires) and it
is stated in `references/requirements.md` too — the file keeps the scaffold's
*mechanics* and drops the session's instructions.
**Relocate → CLI / `docs/USAGE.md`:** the `brew install` line, "no subcommand, no
flags", and the dispatch table are command facts and human-operator material.

#### `content/references/bootstrap.md` — assembling the mandate

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `stale-launch-stops` — a handle resolving to nothing or to a terminal leaf is a stale launch, not work to redo | 19 · step:Bootstrap | `trigger` | `on(the handle does not resolve to a live leaf) @ SKILL.md` | B★ |
| `brief-chain-tolerates-gaps` — `brief-chain` walks ancestor directories root→leaf and skips a level with no brief silently, so an uncharted node still bootstraps | 19 · step:Bootstrap | `none` | `on(reading the brief chain) @ references/bootstrap.md` | B |

#### `content/references/execute.md` — doing the work

`execute.md` sheds *What each kind produces* entirely: it is a nineteen-kind
summary of material the ten kind references own, and the driver already routed
the session to its own.

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `review-budget` — a picked plain producer may materialise at most **one** in-session reviewer across the whole leaf; a second need is a `review-*` leaf | 19 · step:Execute | `trigger` | `on(considering a reviewer) @ SKILL.md` | B★ |
| `review-budget-predicate` — the allowance applies only to a session the driver mandated **and** that adopted the mandate by running Bootstrap | 19 · step:Execute | `none` | `on(considering a reviewer) @ references/execute.md` | B |
| `review-budget-by-kind` — the per-kind allowances (reviewed producer: none; `review-*`: none; `integrate-review-*`: one narrow; research trio: none) | 19 · step:Execute | `none` | `on(considering a reviewer) @ references/execute.md` | B |
| `doubt-pass-procedure` — state the claim, strip the conclusion, give one fresh context the artifact and contract adversarially, then classify every finding four ways | 19 · step:Execute | `none` | `on(spending the allowance) @ references/execute.md` | B |
| `escalated-review-routes-through-config` — once review is a leaf, grove owns the route; do not add a competing in-session reviewer | 19 · step:Execute | `none` | `on(a review leaf exists) @ references/execute.md` | B |
| `verify-repo-claims-with-controls` — a repo-wide claim needs a positive **and** a cross-tree control; clean-here alone proves nothing | 19 · step:Execute | `trigger` | `on(claiming something about this repo) @ SKILL.md` | B |
| `enumerate-then-classify` — extract every candidate from the whole surface and classify each; never sweep a pattern list | 19 · step:Execute | `none` | `on(making a repo-wide claim) @ references/execute.md` | B |
| `no-self-invalidating-count` — never document a claim with a count of itself; state the structural fact | 19 · step:Execute | `none` | `on(making a repo-wide claim) @ references/execute.md` | B |
| `check-the-rescued-clause` — before deleting a false clause, check whether the true one beside it only reads as true in its company | 19 · step:Execute | `none` | `on(deleting prose) @ references/execute.md` | B |
| `decisions-land-as-they-settle` — append each settled decision to the task file's running log as it settles; never reconstruct them at the end, in a summary file or in the commit message | 19 · step:Execute | `trigger` | `on(a decision settles) @ SKILL.md` | B |
| `escalation-names-the-tradeoff` — an escalation names the specific trade-off, proposes a recommended answer, and gives the evidence; a general invitation to ask questions is not a prompt | 19 · step:Execute | `trigger` | `on(handing back to a human) @ SKILL.md` | B |

#### `content/references/decompose.md` — growing the tree

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `externalize-by-default` — work that does not serve this leaf's stated goal goes to the tree, never inline; the bar is "fits this session", not "I can finish it" | 19 · step:Decompose | `trigger` **(the *decompose* sentence)** | `on(work surfaces mid-session) @ SKILL.md` | B★ |
| `bigger-than-brief-decomposes` — a leaf that proves bigger becomes a node; do only the first child | 19 · step:Decompose | `trigger` (shares the *decompose* sentence) | `on(the leaf is bigger than its brief) @ SKILL.md` | B★ |
| `vertical-slice` — a child leaf cuts a narrow complete path, demoable without waiting on a sibling | 19 · step:Decompose | `none` | `on(cutting children) @ references/decompose.md` | B |
| `wide-refactor-expand-contract` — a fan-out refactor sequences expand → migrate → contract, one leaf per stage | 19 · step:Decompose | `none` | `on(cutting children) @ references/decompose.md` | B |
| `chain-is-lazy` — each step of a review chain is cut by the session before it, only if required, and decided at that session's end | 19 · step:Decompose | `none` | `on(an artifact may need review) @ references/decompose.md` | B |
| `pair-is-eager` — a vendor pair lands in one call or not at all | 19 · step:Decompose | `none` | `on(a question needs two corpora) @ references/decompose.md` | B |
| `creating-session-writes-the-body` — the session that knows why a step is needed writes its body | 19 · step:Decompose | `none` | `on(cutting a step) @ references/decompose.md` | B |
| `name-step-kind-off-the-producer` — `review-<producer>` for the producer that actually ran | 19 · step:Decompose | `none` | `on(cutting a step) @ references/decompose.md` | B |
| `integration-placement` — `leaf-insert` at the first sibling **entry** after the review whose subtree still holds live work; `leaf-add` when nothing blocks | 19 · step:Decompose | `trigger` | `on(cutting an integration) @ SKILL.md` | B |
| `no-adjacency-exception` — there is no check an exception could perform; a session that departs owns the drift | 19 · step:Decompose | `none` | `on(cutting an integration) @ references/decompose.md` | B+S |
| `diversity-is-the-configs` — whether a review reaches a different target is two config entries' business; grove compares nothing and warns about nothing | 19 · step:Decompose | `none` | `on(cutting a review step) @ references/decompose.md` | B |
| `fog-or-ticket` — a question you can state precisely earns a leaf now; one you cannot stays a horizon note (`BRIEF-FORMAT.md`) | 19 · step:Decompose | `trigger` | `on(foreseeing work) @ SKILL.md` | B |
| `prior-art-research-is-its-own-leaf` — a design depending on lessons the codebase does not show earns a research leaf ahead of it, not a tangent inside it | 19 · step:Decompose | `trigger` | `on(the codebase cannot answer it) @ SKILL.md` | B |
| `research-brief-names-downstream-questions` — the brief names, leaf by leaf, the downstream questions the survey must answer, and biases the search toward post-mortems | 19 · step:Decompose | `none` | `on(cutting a research leaf) @ references/decompose.md` | B |
| `grow-verbs-are-working-tree-only` — the enclosing task's commit folds them in | 19 · step:Decompose | `none` | `on(growing the tree) @ references/decompose.md` | B |
| `task-and-brief-shape` — a cut leaf's name and body, and a node's brief, follow `TASK-FORMAT.md` and `BRIEF-FORMAT.md` | 19 · step:Decompose | `none` | `on(cutting a leaf or a node) @ references/decompose.md` | B |

`task-and-brief-shape` is the row that makes `TASK-FORMAT.md` and
`BRIEF-FORMAT.md` reachable, and it is why neither needs a `SKILL.md` sentence:
the chain `SKILL.md` → `decompose.md` → the two format files terminates at a
static path, which is all the reachability rule asks.

**Relocate → CLI:** decompose.md's transcription of what `leaf-decompose`,
`leaf-add`, `leaf-insert` and `leaf-add-pair` *do* — what each moves, retitles,
creates, gates and prints. `Bound ≠ ∅` for *when* to run them, so the when stays.

#### `content/references/retire.md` — ending a leaf

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `retire-before-commit` — the rename must land inside the task's own commit, and retirement touches one filename and nothing else | 19 · step:Retire | `trigger` | `on(the work is done) @ SKILL.md` | B★ |
| `retirement-is-filename-only` — not the body, not a sibling, not an ancestor; a waiting review reads the committed artifact | 19 · step:Retire | `trigger` (shares the sentence above) | `on(retiring) @ SKILL.md` | B |
| `pruning-is-hitl` — an agent never prunes on its own; an AFK session that finds its path decided against says so and stops | 19 · step:Retire | `trigger` | `on(the path looks decided against) @ SKILL.md` | B★ |
| `prune-scopes-to-the-whole-path` — pruning a reviewed producer leaves its review live and uncheckable; a rejected path is pruned step by step, a chain having no enclosing directory | 19 · step:Retire | `none` | `on(pruning a reviewed producer) @ references/retire.md` | B |
| `no-fourth-status` — no `blocked`, `deferred` or `superseded`; a leaf in doubt gets no status word | 19 · step:Retire | `trigger` (shares `triage-picks-the-verb`'s sentence) | `on(a leaf's place is in doubt) @ SKILL.md` | B+S |
| `triage-picks-the-verb` — *not now* → reorder; *not ours* → an issue; *decided against* → prune | 19 · step:Retire | `trigger` | `on(a leaf's place is in doubt) @ SKILL.md` | B |
| `node-close-is-implicit` — a node is never marked; its done-ness is the absence of a live child, and the cascade asks the human nothing | 19 · step:Retire | `trigger` | `on(a node has no live leaf left) @ SKILL.md` | B |
| `cascade-is-silent` — the close recurses upward without stopping | 19 · step:Retire | `trigger` (shares the sentence above) | `on(a node closes) @ SKILL.md` | B |
| `node-close-four-steps` — check `Done when`, `leaf-add` the named gap, escalate an unnameable one, promote and report; skip step 1 for an uncharted node | 19 · step:Retire | `none` | `on(a node has no live leaf left) @ references/retire.md` | B |
| `reconcile-records-at-retire` — retirement is where the record sets are reworked and every dangling citation is fixed | 19 · step:Retire | `none` | `on(retiring) @ references/retire.md` | B |

#### `content/references/commit.md` — the task boundary

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `one-focused-commit` — artifact + grow-verb writes + `DONE` rename + whatever the cascade promoted, together, named by handle | 19 · step:Commit | `trigger` | `on(the leaf is retired) @ SKILL.md` | B★ |
| `name-by-handle` — name the work item, and each closed node, by `<slug>-k<key>`, never by position or path | 19 · step:Commit | `trigger` (shares the sentence above) | `on(writing the message) @ SKILL.md` | B |
| `jj-seal` — in a jj tree, `jj new` **after** describing, once the rename has landed; this is sufficient without the plugin | 19 · step:Commit | `none` | `on(the stated VCS is jj) @ references/commit.md` | B |

#### `content/references/finish.md` and `content/SIGNAL-FINISH.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `finish-is-the-drivers-to-discover` — a session never concludes the grove is finished; the driver says so by launching `finish` | 19 · step:Finish | `trigger` | `on(the last live leaf retires) @ SKILL.md` | B |
| `finish-confirmation-gate` — propose, and wait for explicit human confirmation before any teardown; with no human, report the plan | {finish} · step:Finish | `none` | `static({finish})` | B |
| `teardown-via-finish-commit` — never delete `.grove/` by hand; `grove-llm finish-commit <handle>` | {finish} · step:Finish | `none` | `static({finish})` | B |
| `absent-tree-proves-nothing` — an absent `.grove/` never proves teardown succeeded | {finish} · step:Finish | `none` | `static({finish})` | B |
| `recovery-pending-stops` — hand a `Recovery pending` diagnostic to the human; never rewrite history to clear it | {finish} · step:Finish | `none` | `static({finish})` | B |
| `nothing-after-finish` — branch integration and worktree teardown are not grove workflow | {finish} · step:Finish | `none` | `static({finish})` | B+S |
| `finish-three-endings` — teardown → `complete --done`; externalised work → `complete`; declined → no signal | {finish} · step:Finish | **none — byte-frozen and inlined into `${prompt}`** | `static({finish})` | B★ |

#### `content/SIGNAL.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `signal-is-the-last-action` — `grove-llm complete` last, then nothing else; ending without signalling stops the loop | 18 · step:Commit | **none — byte-frozen and inlined into `${prompt}`** | `static(18)` | B★ |

Both signal files are **out of scope for every rewriting leaf**. They are the one
surface where a wording change is unrecoverable mid-loop, and the guaranteed core
inlines their bytes verbatim, so an edit ships to `${prompt}` and the skill at
once with no channel left to correct it.

#### The ten kind references

Incremental by construction under rule 2: a kind reference states what is true of
**that kind and no sibling**, states nothing a loop-step or format file owns, and
gets **no `SKILL.md` presence at all**.

##### `references/requirements.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `requirements-establishes-what` — the deliverable is *what* should be built, in the human's own words | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `grilling-threshold` — the full one-question-at-a-time procedure runs **only** at three or more interdependent open questions; below it, record the decisions and proceed | {requirements} · step:Execute | `none` | `static({requirements})` | B (k5) |
| `sequence-interdependent-questions` — ask the foundational question first and carry its answer into the derived one; never batch two | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `pre-decided-is-not-a-grilling-question` — record a settled answer and move on; all-settled means the work is an `impl` leaf | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `agree-the-seams-during-grilling` — put the sketched test seams to the human before the design is committed (`SPEC-FORMAT.md` records them) | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `small-workstream-may-fuse-the-three` — a bootstrap session may resolve requirements, design and planning in one leaf, or add a `planning` leaf instead | {requirements} · step:Execute | `none` | `static({requirements})` | B |
| `when-not-to-start-a-grove` — no session-to-session fog means do the work directly; the scaffold's existence is not the signal | {requirements} · step:Execute | `none` | `static({requirements})` | B |

##### `references/design.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `design-deliverable` — a spec, an ADR set, or both | {design} · step:Execute | `none` | `static({design})` | B |
| `design-does-not-cut-impl-leaves` — a `design` session cutting implementation leaves has drifted into planning's job and externalises a `planning` leaf | {design} · step:Execute | `none` | `static({design})` | B |

The OR-form ADR test currently in this file is **deleted**, not rehomed:
`adr-when-to-write` is `ADR-FORMAT.md`'s, and this file may not restate it.

##### `references/planning.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `planning-grows-generatively` — the only kind that grows the tree generatively; the deliverable is more tree | {planning} · step:Execute | `none` | `static({planning})` | B |
| `working-increments-before-slices` — find the smallest independently useful working increments and order them by dependency before slicing; stated **once** | {planning} · step:Execute | `none` | `static({planning})` | B+S |
| `planning-writes-the-briefs` — the child briefs and ordered leaf files for any node it grows | {planning} · step:Execute | `none` | `static({planning})` | B |

##### `references/prototype.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `prototype-is-throwaway` — the reaction is the deliverable; polish is a defect | {prototype} · step:Execute | `none` | `static({prototype})` | B |

##### `references/impl.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `impl-ships` — the deliverable is code, docs or tests that ship | {impl} · step:Execute | `none` | `static({impl})` | B |
| `cite-framework-decisions-to-source` — read the manifest, fetch the official source, cite at the decision site, flag what you could not verify; version-invariant logic, renames and plumbing are exempt | {impl} · step:Execute | `none` | `static({impl})` | B |

`verify-repo-claims-with-controls` does **not** belong here, and the superseded
inventory placed it wrongly. A `review-*` session re-running a producer's sweep, a
`design` session counting occurrences, and a `combine-research` session checking a
claim are all bound by it, so `Bound` is all nineteen and rule 2 does not fire.
It is `execute.md`'s. Only the framework-source discipline is genuinely
`{impl}`-bound.

##### `references/review.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `review-is-inspection-only` — inspect the committed diff, source, requirements and recorded evidence; run no test, build, lint or format command and edit nothing | review-\* family · step:Execute | `none` | `static(review-*)` | B |
| `review-output-is-findings-only` — the paired integration owns every fix and all post-fix verification | review-\* family · step:Execute | `none` | `static(review-*)` | B |
| `the-five-reads-differ` — what each of the five reads looks for | review-\* family · step:Execute | `none` | `static(review-*)` | B |

##### `references/integrate-review.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `triage-four-ways` — verify, then classify as unclear contract, real issue, real trade-off, or noise; never performative agreement | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B |
| `what-each-integration-may-change` — the five differ only in what the session may edit | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B |
| `integration-escalates-redesign` — substantial redesign is a new producer review chain beside the leaf being integrated, not this session's work | integrate-review-\* family · step:Execute | `none` | `static(integrate-review-*)` | B |

##### `references/research.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `citation-per-failure-mode-claim` — a primary source per failure-mode claim; a claim without one is mood, not evidence | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `silence-is-a-finding` — record "no primary source found" explicitly; the absence is a confidence signal | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `walk-away-check-per-system` — for each prior tool, answer what stays legible with the tool uninstalled | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `both-researchers-get-one-brief` — the pair buys breadth from differing corpora, not differing questions | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `researchers-are-not-adversarial` — both run breadth-seeking; the adversarial move is the combiner's | {research-a, research-b} · step:Execute | `none` | `static(research)` | B |
| `research-output-path-per-kind` — `research-a` writes `<slug>-a.md`, `research-b` `-b.md`; the kind is the discriminator that stops the second clobbering the first | {research-a, research-b} · artifact:research doc | `none` | `static(research)` | B |

##### `references/combine-research.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `agreement-without-independent-primary-sourcing-is-a-red-flag` — union the coverage, flag every disagreement, and ask whether an agreed claim reached the two surveys through *different* primary sources; stated **once** | {combine-research} · step:Execute | `none` | `static({combine-research})` | B+S |
| `combine-writes-the-union` — the unadorned `docs/research/<slug>.md` is the union | {combine-research} · artifact:research doc | `none` | `static({combine-research})` | B |

##### `references/finish.md`

Its rows are tabulated above with `SIGNAL-FINISH.md`; the file doubles as the
`finish` kind's own reference.

#### The format files

##### `content/TASK-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `leaf-name-grammar` — five fields, all parsed; position, outcome infix, kind, slug, key | 19 · artifact:task | `none` | `on(cutting or reading a leaf) @ references/decompose.md` | B+S |
| `kind-set-is-closed-at-nineteen` — the closed set, and a missing or unknown kind is malformed rather than an `impl` default | 19 · artifact:task | `none` | `on(cutting a leaf) @ references/decompose.md` | B |
| `handle-is-slug-key` — `<slug>-k<key>` is the work-item identity; a key is never reused | 19 · artifact:task | `none` | `on(naming a work item) @ references/decompose.md` | B |
| `header-is-position-free` — the body's `# <slug>-k<key>` header carries no position and no kind | 19 · artifact:task | `none` | `on(writing a task body) @ references/decompose.md` | B |
| `body-carries-no-launch-metadata` — no kind, harness, model, or record of how a past session ran | 19 · artifact:task | `none` | `on(writing a task body) @ references/decompose.md` | B |
| `suggested-body-shape` — Goal, Context, Done when, Notes, and a `## Decisions (running log)` section when one is being kept | 19 · artifact:task | `none` | `on(writing a task body) @ references/decompose.md` | B |
| `declaration-lines-are-convention` — `**Reviews:**` / `**Integrates:**` are written by hand and parsed by nothing | 19 · artifact:task | `none` | `on(cutting a step) @ references/decompose.md` | B+S |

**`TASK-FORMAT.md` sheds its policy.** It is format grammar, so the composition
shapes, the doubt budget table, the kind disciplines and *A leaf never names a
harness* leave it for `decompose.md`, `execute.md`, the kind references and
`driver.md` respectively. What remains is what constrains bytes on disk.
`suggested-body-shape` is where the running log's *section* is described; the
obligation to append to it as decisions settle is `execute.md`'s
`decisions-land-as-they-settle`, and the split is deliberate — grammar here,
conduct there.

##### `content/BRIEF-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `every-node-carries-a-brief` — one species; a node is a leaf that decomposed, and a brief-less node is a lapse a reader still tolerates | 19 · artifact:brief | `none` | `on(cutting a node) @ references/decompose.md` | B |
| `brief-is-process-scaffolding` — neither glossary nor decision log; it exists so a leaf reads three ADRs, not fifty | 19 · artifact:brief | `none` | `on(writing a brief) @ references/decompose.md` | B |
| `briefs-inherit` — a child brief states only what is new at its level; pointers accumulate down the chain | 19 · artifact:brief | `none` | `on(writing a brief) @ references/decompose.md` | B |
| `brief-content-is-durable` — behavioural contracts and named types, not paths or line numbers | 19 · artifact:brief | `none` | `on(writing a brief) @ references/decompose.md` | B |
| `horizon-note-shape` — a line or two of foreseen work, dropped once it graduates to a leaf | 19 · artifact:brief | `none` | `on(recording fog) @ references/decompose.md` | B |
| `brief-is-never-marked-done` — a brief is context, not a task; promotion upward is what a close does with it | 19 · artifact:brief | `none` | `on(a node closes) @ references/retire.md` | B |

##### `content/ADR-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `adr-when-to-write` — **all three**: hard to reverse **and** surprising without context **and** the result of a real trade-off; stated locally, not cited | 19 · artifact:adr | `trigger` **(the *records-raised* sentence** — naming the file, **never** the test) | `on(considering an ADR) @ SKILL.md` | B+S |
| `adr-minimal-template` — the local minimal template | 19 · artifact:adr | `none` | `on(writing an ADR) @ content/ADR-FORMAT.md` | B |
| `adr-placement-and-slug-identity` — `docs/adr/<slug>.md`, the slug is the identity, cite by slug and never by number, directory created lazily | 19 · artifact:adr | `none` | `on(writing an ADR) @ content/ADR-FORMAT.md` | B |
| `adr-set-is-minimum-coherent` — a session that changes a recorded decision reworks the set in place (merge / split / delete) and never appends a superseding record | 19 · artifact:adr | `trigger` **(the *records-current-state* sentence)** | `on(changing a recorded decision) @ SKILL.md` | B+S |
| `adr-split-is-conditional-on-repo-shape` — split only when the contexts are peers; otherwise one flat root set with ownership recorded in `CONTEXT-MAP.md` | 19 · artifact:adr | `none` | `on(writing an ADR in a multi-context repo) @ content/ADR-FORMAT.md` | B |
| `research-to-adr-bridge` — an adopted finding gets a bridge pointing both ways: the ADR cites the survey by primary source, the survey names the ADRs its findings landed in | 19 · artifact:adr | `none` | `on(adopting a research finding) @ content/ADR-FORMAT.md` | B |

`records-are-current-state` was a single row owned by `references/execute.md`,
restating what `ADR-FORMAT.md` and `SPEC-FORMAT.md` each already say about their
own set. Rule 3 fires per artifact, so it becomes two rows —
`adr-set-is-minimum-coherent` and `spec-set-is-current-state` — sharing the one
*records-current-state* trigger sentence that names both files, and
`references/execute.md` states neither.

##### `content/SPEC-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `spec-at-an-agreement-point` — written only at a genuine agreement point, by `design`; most increments write none | 19 · artifact:spec | `trigger` (shares the *records-raised* sentence) | `on(the increment may be an agreement point) @ SKILL.md` | B |
| `spec-membership-test` — would a session on an unrelated future grove need to read this? if not it is a `BRIEF.md` | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B |
| `spec-grain-rule` — an ADR records one decision; a spec describes how an area works and **cites** the ADRs in its area rather than restating them | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B+S |
| `spec-set-is-current-state` — edited, merged, split and deleted in place; never dated, numbered or superseded | 19 · artifact:spec | `trigger` (shares the *records-current-state* sentence) | `on(changing a spec) @ SKILL.md` | B+S |
| `spec-synthesises-never-re-interviews` — the grilling already happened upstream; a spec synthesises its running log | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B |
| `spec-is-behavioural-not-procedural` — interfaces, types and contracts; no paths, no line numbers, no code but a decision-encoding snippet | 19 · artifact:spec | `none` | `on(writing a spec) @ content/SPEC-FORMAT.md` | B |
| `test-seams-agreed-and-recorded` — prefer existing seams, propose a new one at the highest point, drive the count toward one, and record the agreement in the spec's `## Test seams` or the node's brief | 19 · artifact:spec | `none` | `on(the increment covers tested code) @ content/SPEC-FORMAT.md` | B |

##### `content/CONTEXT-FORMAT.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `glossary-is-the-forcing-function` — a term is resolved into `CONTEXT.md` **inline**, never batched | 19 · artifact:glossary | `trigger` | `on(a term is resolved) @ SKILL.md` | B+S |
| `glossary-is-only-a-glossary` — terse definitions and aliases to avoid; no implementation detail, no spec, no scratch pad | 19 · artifact:glossary | `none` | `on(writing a glossary entry) @ content/CONTEXT-FORMAT.md` | B |
| `terms-are-context-specific` — only terms unique to this context; general programming concepts do not belong | 19 · artifact:glossary | `none` | `on(writing a glossary entry) @ content/CONTEXT-FORMAT.md` | B |
| `context-map-when-multiple` — a root `CONTEXT-MAP.md` means multiple contexts; a term is defined in its owning context's glossary and never both | 19 · artifact:glossary | `none` | `on(the repo has a context map) @ content/CONTEXT-FORMAT.md` | B |
| `challenge-and-sharpen-terms` — call out a term conflicting with the glossary, propose a precise canonical term for a fuzzy one, and cross-check a stated behaviour against the code | 19 · artifact:glossary | `none` | `on(a term is used loosely) @ content/CONTEXT-FORMAT.md` | B |

`glossary-is-the-forcing-function` moves here from `references/grove.md`. Rule 3
fires on `artifact:glossary`, and the file every session about to write a glossary
entry opens is this one; `grove.md` keeps the argument only until that argument
relocates to `docs/ARCHITECTURE.md`. `challenge-and-sharpen-terms` moves here from
`grilling.md`, whose statements of it and of `glossary-is-only-a-glossary` are
procedure-register duplicates.

##### `content/grilling.md`

| rule | Bound · Occasion | mirror | load | test |
|---|---|---|---|---|
| `grilling-procedure` — one question at a time, a recommended answer for each, walk the design tree, facts looked up and decisions put to the human | {requirements} · step:Execute | `none` | `on(the grilling threshold is met) @ references/requirements.md` | B |
| `no-writes-before-shared-understanding` — do not commit decisions or grow the tree until the human confirms shared understanding | {requirements} · step:Execute | `none` | `on(the grilling threshold is met) @ references/requirements.md` | B |
| `probe-with-concrete-scenarios` — stress-test domain relationships with invented edge-case scenarios that force precision about boundaries | {requirements} · step:Execute | `none` | `on(the grilling threshold is met) @ references/requirements.md` | B |

`grilling.md` is a **procedure reached on a condition**, not a standing
instruction, and its entry condition is stated above the bundled body in Grove's
own voice — as its provenance comments already are — leaving the `<what-to-do>`
block byte-intact. `no-writes-before-shared-understanding` lives *inside* that
block, so the row records the bundled sentence as the canonical statement rather
than authorising a Grove-voiced second one.

Its remaining sections are duplicates and leave: the *File structure* /
lazy-creation material restates `context-map-when-multiple`,
`adr-split-is-conditional-on-repo-shape` and `spine-seven-constraints`
(constraint 4); *Update CONTEXT.md inline* and the glossary-only sentence restate
`CONTEXT-FORMAT.md`'s rows; *Offer ADRs sparingly* becomes a pointer to
`adr-when-to-write`; *Agree the test seams* keeps only the grilling-timing half
(`agree-the-seams-during-grilling`, owned by `references/requirements.md`) and
points at `SPEC-FORMAT.md` for the rest.

---

### The two contradictions, resolved

#### 1. The grilling threshold

**Canonical statement.** A `requirements` session **always** establishes *what*
should be built. The **full one-question-at-a-time grilling procedure** runs
**only** when three or more interdependent questions are open. Below that
threshold the session records the decisions and proceeds; staging an interview
over settled ground costs a human's attention and returns nothing.

- **Canonical source:** `content/references/requirements.md`, as
  `grilling-threshold`. Rule 2 fires — `Bound = {requirements}`.
- **Load predicate:** `static({requirements})`.
- **Mirror class:** `none`. The driver already routed that kind to this file.
- **Stated once.** The file currently carries the always-form bullet and the
  three-question trigger as two independent statements — and the always-form
  bullet twice. One statement survives; `S` asserts it.

#### 2. The ADR test

**Canonical statement.** An ADR is raised only when **all three** hold: hard to
reverse **and** surprising without context **and** the result of a real
trade-off.

- **Canonical source:** `content/ADR-FORMAT.md`, as `adr-when-to-write`. Rule 3
  fires on `artifact:adr` **before** rule 5 could send it to a loop-step file —
  which is the ambiguity the superseded text left open and this ordering closes.
- **Load predicate:** `on(considering an ADR) @ SKILL.md`.
- **Mirror class:** `trigger`, naming the file and never paraphrasing the test.
  "Raised sparingly" is not a legal trigger — it is a looser test wearing a
  condition's clothes, and it is what let the OR-form survive.
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

### What moves, and where: `content/driving.md`

The function's sharpest output concerns `content/driving.md`, the largest file in
the corpus. **Its rules do not all have owners elsewhere, and the superseded
text's claim that the remainder has `Bound(R) = ∅` was false.** Direct
inspection found eight sections still carrying imperatives that alter session
conduct, so the file's deletion is conditional on each one landing an embedded,
reachable owner first. This table is that condition, discharged row by row.

| `driving.md` section | rule | new owner |
|---|---|---|
| When to commission prior-art research | `prior-art-research-is-its-own-leaf` | `references/decompose.md` |
| How to write a research leaf brief | `research-brief-names-downstream-questions` | `references/decompose.md` |
| ” | `walk-away-check-per-system`, `citation-per-failure-mode-claim`, `silence-is-a-finding` | `references/research.md` |
| Running the vendor pair | `pair-is-eager`, `both-researchers-get-one-brief`, `researchers-are-not-adversarial`, `research-output-path-per-kind` | `decompose.md`, `research.md` |
| Name the trade-off you want input on | `escalation-names-the-tradeoff` | `references/execute.md` |
| Record decisions inline | `decisions-land-as-they-settle` (+ `suggested-body-shape`) | `references/execute.md` (+ `TASK-FORMAT.md`) |
| Retiring research into ADRs | `research-to-adr-bridge` | `content/ADR-FORMAT.md` |
| Reworking ADRs and briefs | `adr-set-is-minimum-coherent`, `reconcile-records-at-retire` | `ADR-FORMAT.md`, `references/retire.md` |
| Verifying framework decisions against the source | `cite-framework-decisions-to-source` | `references/impl.md` |
| Verifying a claim about the repo itself | `verify-repo-claims-with-controls`, `enumerate-then-classify`, `no-self-invalidating-count`, `check-the-rescued-clause` | `references/execute.md` |
| Doubting inside a picked Grove leaf | `review-budget`, `review-budget-predicate`, `review-budget-by-kind`, `doubt-pass-procedure` | `references/execute.md` |
| The review chain | `chain-is-lazy`, `creating-session-writes-the-body`, `name-step-kind-off-the-producer`, `integration-placement`, `no-adjacency-exception`, `diversity-is-the-configs` | `references/decompose.md` |
| ” | `review-is-inspection-only`, `review-output-is-findings-only` | `references/review.md` |
| ” | `triage-four-ways`, `integration-escalates-redesign` | `references/integrate-review.md` |
| Externalizing surfaced work | `externalize-by-default`, `bigger-than-brief-decomposes` | `references/decompose.md` |
| What a good child leaf looks like | `vertical-slice`, `wide-refactor-expand-contract` | `references/decompose.md` |
| Recording fog without pre-slicing it | `fog-or-ticket`, `horizon-note-shape` | `decompose.md`, `BRIEF-FORMAT.md` |
| Prune, reorder, or file an issue | `triage-picks-the-verb`, `no-fourth-status`, `prune-scopes-to-the-whole-path` | `references/retire.md` |

**What genuinely has `Occasion = none`, and where it goes.** Two sections are
addressed to the *human* driving a grove rather than to a session — *Ask the LLM
"WDYT" before committing* and *Ask for pushback when the LLM agrees too easily*.
A session is the LLM; it cannot obey either. Their session-facing residue is
already `escalation-names-the-tradeoff` (never withhold a recommendation out of
deference) and `grilling-procedure` (a recommended answer per question), so what
remains is operator guidance, and the human-facing home already exists:
**relocate → `docs/USAGE.md`**.

Everything else left over is argument, worked example and provenance. **It is
deleted, not relocated.** A worked example anchored on the sync-semantics grove
cites a work item that resolves nowhere — which the glossary's own handle rule
forbids in provisioned content — and the arguments that are still binding are in
the ADRs. The VCS holds the rest. `docs/` earns a relocation only where a real
human-facing document already wants the material, which is the `USAGE.md` case
and nothing else here.

**And eight conditions in `SKILL.md` point at `driving.md` today.** Every one is
repointed by the same leaf that lands its rule's new owner; a `trigger` sentence
naming a deleted file is the reachability failure this design exists to prevent,
and it would ship silently.

## Test seams

- **Per-rule, never universal.** No parser over the corpus and no marker grammar.
  Each rule's instrument is named in its row and lands with the rewrite that
  homes it.
- **The static loaded path is computed from `src/prompt.rs`**, not transcribed:
  `reference_file(kind)` is already an exhaustive match, and a per-kind budget
  test walks it. That is the seam `loaded-path-budgets-k10` builds on, and it
  costs no new production code.
- **Reachability is a test, not a review note.** Every `on(<trigger>) @ <file>`
  row's chain must terminate at a static path with no cycles. The assertion is
  cheap — the `@` files are a small closed set — and it is what would have caught
  `impl`'s two rules sitting in a file no `impl` session is routed to.
- **`static(...)` is asserted against the runtime.** A row claiming `static(K)`
  whose owner is not `SKILL.md` or `reference_file(k)` for every `k ∈ K` fails.
  That single check is what the `always(19)` labelling would not have survived.
- **The `SKILL.md` budget is asserted, not hoped for.** Total words in range, at
  most 18 `trigger` sentences, each at most 25 words, and the seven `own` rows
  present. Owned by `skill-router-k4`.
- **Single-source assertions are phrase-scoped, with controls.** A rule whose
  wording is distinctive gets a normalised sweep (emphasis stripped, whitespace
  collapsed) asserting exactly one procedure-register file states it — with a
  positive control that the sweep finds a phrase known present, and a
  cross-tree control that it still finds the class where it legitimately lives.
  An unnormalised sweep silently misses a wrapped or emphasised match; that
  failure was reproduced while writing this spec.
- **Behavioural evals assert conduct, not contents.** `behavior-evals-k3` owns
  the eight **B★** areas: *no second pick*, *no VCS reprobe*, *stale launch
  stops*, *the decomposition boundary*, *human-only pruning*,
  *retire → commit → complete*, *the review budget*, *all three finish-signal
  outcomes*. The ninth required area, *the interview threshold*, lands with
  `kind-references-k5` because it cannot be green before that fix. Every other
  **B** row lands with its own rewrite.
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

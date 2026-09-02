# pipeline-skills-k28

## Goal

Author the extracted kinds as installed `grove-<kind>` skills, wire them into the
conformance runner and the install route, reconcile the prose that states the
methodology's current kind set, and hand the human the launch templates the new
kinds need.

## Context

- The set and each kind's discipline are `pipeline-kinds-k27`'s and are not
  reopened here. If one of them cannot be written as a skill, that is a finding to
  report, not a licence to redesign it. **`## The four kinds` below is that
  discipline, written to be authored from**; the two records behind it are
  `docs/adr/the-editorial-pipeline-is-four-kinds.md` (which stages, and the
  evidence) and `docs/adr/a-feedback-edge-is-forward-tree-growth.md` (how a
  backward edge is expressed).
- **A shipped skill may not cite this repository's ADRs.** `docs/adr/` appears in
  the skill set only as a location convention, never as a specific slug, because
  the plugin installs into repositories that have neither. Every rule below is
  stated in the skill that owns it; the two records are attribution for a reader
  of *this* repository and are not the delivery route.
- Uniformity is with `plugins/grove/skills/grove-<kind>/SKILL.md` as they stand:
  each opens with the imperative to load the spine skill, carries inline only what
  that kind alone owns, and directs a load of a family file **by name** where a
  family owns the rule.
- Assurance to update, not merely to keep green: `plugins/grove/conformance.sh`
  and `conformance.test.sh` with `plugins/grove/conformance/rules.tsv`;
  `plugins/install.sh` and `install.test.sh`, whose `harnesses:` key decides
  installability off Claude Code; and the `include_str!` content assertions in
  `crates/grove-llm/tests/` — `session_kind_presence.rs`,
  `session_kind_guidance.rs`, `composition_guidance.rs` and their neighbours
  read the shipped skill bytes and are the seam that goes red when the set moves.
- Prose stating the current set and its arithmetic: `CONTEXT.md` (*Session kind*,
  and the *Family reference file* and *Loop-step reference file* counts),
  `docs/ARCHITECTURE.md`, and `plugins/grove/skills/grove/TASK-FORMAT.md`, whose
  *The nineteen kinds* section states a table and a count that both move.
  **Four of those statements are structurally wrong and not merely stale** — see
  `## Prose that does not survive the addition`.
- Reachability is not the same as presence. A rule stated in a file no condition
  names is off every loaded path and is delivered nowhere; `CONTEXT.md`'s
  *Loaded path* and *Composed loaded path* entries state the test.

## The four kinds

`draft`, `copy-edit`, `art`, `proof` — four new `grove-<kind>` skills and one new
family reference file. Unprefixed tokens: a kind is a discipline rather than a
finding, and each is stated over *a document and the contract that document
declares*, never over walkthrough books in this repository.

Filing is `docs/adr/corpus-rules-have-one-owner.md` rule 2 applied with no
exception: a rule binding all four goes in the family file, a rule binding one
stage goes inline in that stage's `SKILL.md`, and the spine's `SKILL.md` gains
nothing at all — `restatement-declares-its-class.md` makes class `none` mandatory
for a rule that binds one kind or one family. The family file's incoming edge is
the four opening imperatives that direct a load of it by name.

### The four at a glance

Every field below is stated in full under the heading that owns it; this table is
the index, not a second statement of any of them.

| | `draft` | `copy-edit` | `art` | `proof` |
|---|---|---|---|---|
| **mark** | AFK | AFK | AFK | AFK |
| **goal** | produce the document from its sources and its structure brief | sentences, terminology, consistency with the prose contract | the document's figures | the final whole-document read |
| **reads** | structure brief, sources, document contract | the drafted document, prose contract, glossary, named precedent | the document, the contract's figure rules | the whole document, and the node brief's `## Handed forward` |
| **deliverable** | the document, green | the same document, green | the same document, green | the same document, green, and an empty hand-forward list |
| **charter boundary** | structure and technical truth, both folded in | does **not** restructure | figures only, in the contract's medium | none — every class is in charter |
| **cuts next** | `copy-edit` | `art` | `proof` | nothing, a correction run aside |

**Verdict provenance, since it bounds what a skill may claim for itself.** `draft`
is in the pipeline by construction and was never scored. `copy-edit` cleared the
preregistered threshold on arithmetic. `art` cleared it on the report's counting
adjudication, and would read `Merge` under a defensible alternative reading. `proof`
is the stated fallback and was untestable by the instrument that judged the others.
Two stages that ran in the pilot — the developmental edit and the technical edit —
are **not** here: their charters are folded into `draft`, and no skill of their own
may appear.

### The family file — `grove/references/editorial.md`

The fourth family reference file, beside `review.md`, `integrate-review.md` and
`research.md`, and reached the same way. Density is `review.md`'s: this is a
discipline for four kinds, not a manual.

What it owns, all of it bound to all four:

- **The frame.** `draft`, `copy-edit`, `art`, `proof` (all AFK) — the four stages
  of a document's publishing pipeline. Each is a **producer, not a reviewer**: it
  reads the whole document as its predecessor left it and *fixes*, where a
  `review-*` kind only reports. The contract the document declares — its
  specification, its prose rules, its validator — supplies every class a charter
  names; the stage supplies the discipline of reading for it.
- **Whole document, one charter.** A stage reads the whole document and changes
  only what its own charter owns. Work outside the charter is recorded and passed
  on, never absorbed — a stage that quietly does another's job leaves the next
  stage's empty result unreadable, since nobody can then tell *found nothing* from
  *pre-empted*.
- **The chain is lazy, and its membership is not optional.** A document's leaf
  becomes a node with `grove-llm leaf-decompose <leaf> <slug> --kind draft`, and
  each stage's **last act** is `grove-llm leaf-add <node> <stem> --kind <next>` —
  **unless a live later sibling under this node already holds that stage**, in
  which case you cut nothing, because an earlier stage's correction run has
  already queued it. That condition is read off the node's live entries; there is
  no flag and no field, and a stage that cuts unconditionally regrows the tail of
  the pipeline at every hop. `proof` cuts nothing in the ordinary case. Cutting
  late is what lets the cutting session write the next stage's body with the
  specific case it has just met. **Do not skip a stage because you judge it will
  find nothing** — which stages exist was measured, and a skipped stage is this
  session re-grading a stage that measurement already graded. That is the one
  place this family differs from a review chain, where the producer genuinely
  decides whether the next step is warranted. Skipping and *finding it already
  queued* are different: in the second the stage is already standing ahead of you
  in the walk, and will run in its turn.
- **Every stage leaf under a document's node carries the document's bare stem as
  its whole slug** — no stage word, no suffix. The kind field beside it is the
  canonical statement of which stage this is, and a slug restating it would be a
  second, unvalidated copy of a fact grove already parses and routes on.
- **Handing work forward.** A defect a *later* stage's charter owns goes in the
  document node's `BRIEF.md`, under a running `## Handed forward` list naming the
  owning stage and the location. Every later stage reads it, because the brief
  chain is root-to-leaf — which the next leaf's body cannot do for a stage two
  hops away. A stage **clears** the entries it closes: a brief is current-state
  context, not a log.
- **Sending work back is forward tree growth, and it is one ordered run.** `pick`
  is a depth-first pre-order walk; it cannot re-enter a retired leaf, and there is
  no leaf state that means *reopened*. A defect an **earlier** stage's charter
  owns, that you may not fix, becomes a **re-run leaf of that stage** — and one
  leaf is never enough, because the material it changes is read again by every
  stage after it. Cut, with `leaf-add` and in pipeline order, a **contiguous run**
  that begins at the owning stage and ends at `proof`:
  - the run always contains the stage that would otherwise have run next after
    you, and always ends at `proof`;
  - omit a stage lying between the owning stage and you only where the changed
    material cannot reach its charter, and say why in the run's first body;
  - the run **replaces** your ordinary last act. Do not cut your normal successor
    a second time beside it.

  The chain is lazy, so nothing is queued behind you and call order is walk order;
  use `leaf-insert` only where a later sibling entry already holds live work.
  Write the specific defect into the re-run leaf's body. Every leaf in the run
  then finds its own successor already standing and cuts nothing, which is what
  makes the correction terminate rather than regrow itself.
- **A second re-run of one stage against one document is an escalation, not a
  third leaf.** Stop and say so. An unbounded re-run rule in an unattended arm is
  an oscillation that spends sessions without terminating.
- **Your review allowance is the ordinary producer's.** No stage takes a
  `review-*` leaf, so no stage is `references/execute.md`'s *already has a review
  beside it* case: every stage, `proof` included, is a plain producer with the
  ordinary leaf-wide allowance of at most one in-session reviewer, spent under
  that procedure's four-step pass. **The next stage is not that review.** It reads
  the whole document against a *different* charter and may not repair your class,
  so it neither looks for nor may act on the defects an adversarial read of your
  own obligations would report. Where a **second** need appears, the escalation
  the execute procedure names — cut a `review-<producer>` leaf — has no kind in
  this family. Finish to a coherent boundary and route the doubt through the
  machinery that exists: the next stage's body if it is that stage's class, the
  node brief's `## Handed forward` if a later stage owns it, a correction run if
  an earlier stage does.
- **Leave it green.** A stage ends with the document passing whatever gate the
  repository runs over it, in one focused commit like any other task.

### `grove-draft/SKILL.md`

**draft** (AFK) — produce the document from its sources and its structure brief,
to a green final validation. The pipeline's first stage: there is no document
without it and no alternative to it.

Inline, bound to `draft` alone:

- **A structure brief is a precondition, not an input you can proceed without.**
  Audience, conceptual order and what deserves emphasis are nowhere in the
  sources. **If this document has no structure brief, stop and say so** — do not
  draft from the sources alone. This is not tidiness: two editorial stages were
  folded into this one on the evidence of a document drafted from such a brief,
  and without the brief the fold has no basis under it.
- **How to tell whether you have one — three things you can cite, from a named
  artifact.** The document declares its structure brief the way it declares the
  rest of its contract: **this leaf's body or the document node's brief names it
  by path**, and that artifact itself states
  - **who the reader is** and what they should be able to do afterwards,
  - **the ordered section plan** — the sequence, and what each section is for,
  - **what deserves emphasis**, and what the document does not cover.

  All three, in the artifact, quotable. **The existence of a `BRIEF.md` is not the
  test.** `leaf-decompose` gives every node one automatically and bootstrap reads
  it, so an existence check answers yes for every draft — including exactly the
  no-human-structure case that must stop. If the only candidate is the node's own
  brief and it does not state all three, you do not have a structure brief: stop,
  and name which of the three is missing. Do not infer one from a filename or from
  a neighbouring leaf's slug; those are one repository's convention and this kind
  installs everywhere.
- **Two folded charters, owned explicitly.** Because structure and technical truth
  are not separate stages, the draft owns both, and owns them as obligations to
  discharge rather than as things the brief has already handled:
  - **Structure** — conceptual order, what each section is for, and whether the
    document delivers the reader outcome its brief asked for. Structure, not
    sentences.
  - **Technical truth** — whether what the document says about its subject is
    true, complete for its scope, and not silently stale against the sources it is
    written from. The operative test is that every claim about the subject is
    checked **against the subject**, not against what you remember of it or what
    an earlier page of the same document asserted.
  - A drafting session that treats these as discharged by the brief has left two
    stages of the pipeline undone. Whether they survive the fold was never
    measured, so nothing licenses assuming they do.
- **Last act:** `leaf-add` the `copy-edit` leaf, under the family file's rule — which
  is conditional, and which a correction run discharges for you.

### `grove-copy-edit/SKILL.md`

**copy-edit** (AFK) — sentences, terminology, and consistency with the document's
prose contract and with whatever precedent that contract names as the one to be
uniform with. **It does not restructure**: a structural finding is sent back as a
`draft` re-run leaf, never fixed here.

Inline, bound to `copy-edit` alone:

- **What it reads:** the document as the draft left it, the prose contract the
  document declares, the project's glossary, and the precedent the contract names.
- **The operative tests**, each one a defect the measured stage actually closed:
  a term spelled one way throughout rather than two; a term the glossary owns
  linked to it rather than silently re-defined beside it; a figure of speech
  replaced by the mechanism it stood for; a claim the reader cannot check replaced
  by one they can; and where the contract fixes a set of questions a passage must
  answer, **all** of them answered rather than one.
- **Last act:** `leaf-add` the `art` leaf, under the family file's rule — which
  is conditional, and which a correction run discharges for you.

### `grove-art/SKILL.md`

**art** (AFK) — the document's figures: what a table, a list figure or a diagram
can carry, and nothing else.

Inline, bound to `art` alone:

- **Work in the medium the document's contract admits, and no other.** Where the
  contract admits only the page's own bytes, a figure is a table, a list figure or
  a drawing inside a text fence, and there is no file beside the page. Do not
  introduce a medium the contract does not name.
- **Two rules, and they are the whole charter.** A relation the reader would
  otherwise have to hold in their head — a sequence, a partition, a comparison, a
  mapping over a set of members — carried only by running prose where a figure
  would carry it, is a defect. And every figure states its role: adjacent to it, a
  sentence saying what the reader is to take from it, not merely what its subject
  is.
- **Record a figure you declined to draw, and say which of three reasons it was.**
  An editorial judgement, a rule in the contract that forbids the fix, and *the
  medium could not carry it* are three different findings, and the third is the
  only one that is evidence about the medium itself. Name it as such when it is
  true, and do not reach for it when the real reason is one of the other two — a
  contract's medium is reopened by that record and by nothing else.
- **Last act:** `leaf-add` the `proof` leaf, under the family file's rule — which
  is conditional, and which a correction run discharges for you.

### `grove-proof/SKILL.md`

**proof** (AFK) — the final whole-document read: errors the earlier stages
introduced or missed, and the last check that the document is whole.
**Whole-document scope, and every class is in charter** — the one stage with no
charter boundary.

Inline, bound to `proof` alone:

- **What only this stage can see.** Every earlier stage reads for its own class;
  this one reads for coherence across the whole document — two places giving two
  values for one fact, a closing summary contradicting a table hundreds of lines
  above it, one term counted one way on one page and another way elsewhere.
- **You are last, so you hand nothing forward.** Clear the node brief's
  `## Handed forward` list: close each entry, or turn it into a re-run leaf of the
  stage that owns it, or state at node close that it survived and why, so it is
  promoted rather than lost. An entry left in the list when this stage retires is
  a defect nobody owns.
- **You cut no next stage — unless you send work back.** In the ordinary case you
  are last: when you retire, the document's node has no live leaf and closes. A
  correction run you cut is the exception and is what keeps the node open by
  construction; the node closes when a `proof` retires having sent nothing back
  and cleared the hand-forward list.
- **This stage is in the pipeline as the stated fallback, not on measured
  evidence.** It appears in both arms of any comparison that would test it, so
  nothing has established what it contributes. Two things follow: a proof session
  that finds nothing has still done its job, and a run of defects concentrated in
  one earlier stage's class is evidence about **that** stage — the re-run leaf you
  cut is where it gets recorded.

### What the spine's `SKILL.md` gains

Nothing. Every rule above binds one kind or one family, and
`restatement-declares-its-class.md` makes class `none` mandatory in that case.
Adding a trigger sentence for any of them would be a defect, not a courtesy.

### The conformance rows

`plugins/grove/conformance/rules.tsv`, in the existing grammar — family rows take
a named-family predicate like `static(research)`, per-kind rows the braced form
like `static({impl})`:

| owner | load |
|---|---|
| `grove/references/editorial.md` | `static(editorial)` |
| `grove-draft/SKILL.md` | `static({draft})` |
| `grove-copy-edit/SKILL.md` | `static({copy-edit})` |
| `grove-art/SKILL.md` | `static({art})` |
| `grove-proof/SKILL.md` | `static({proof})` |

Every one of these is a **static** path: a family file reached by its members'
opening imperative is static exactly as `review.md` is, so none of them takes an
`on(<trigger>) @ <file>` row. The header comment's row count moves with them.

## Prose that does not survive the addition

Four statements are **structurally** wrong once the family lands, not merely
stale, and a session that only bumps counts will leave them describing a set that
no longer exists. Enumerate every statement of the set and classify each; do not
sweep this list.

- **`TASK-FORMAT.md`, *The nineteen kinds*.** Its premise — *the set is
  parameterised, not flat: five producers, each with its own `review-` and
  `integrate-review-` step, plus a research pair and one driver-owned step* — no
  longer covers the set, and its three-column table has no way to say *this kind
  has no review step*. The heading, the premise, the table and the closing
  arithmetic sentence all move together.
- **`docs/ARCHITECTURE.md`, *Task kinds and composition*.** Same three-column
  problem, plus *the methodology's current set of nineteen*, *the nineteenth kind,
  `finish`*, and **two documented composition shapes exist** — there are now three,
  and the third is the editorial chain: lazy like a review chain, but with
  mandatory membership and no integrate step.
- **`CONTEXT.md`, *Family reference file*.** *Three files, not ten* becomes four,
  and the enumeration beside it gains the editorial entry. Check the *Loop-step
  reference file* entry's *Seven, beside the three family reference files* in the
  same pass — it counts the family files from the other side.
- **`docs/CONFIGURATION.md`, *The kinds this methodology ships*.** Both the code
  block listing the set and the example configuration above it, which is
  advertised as *covering every kind this methodology ships*.

`plugins/grove/conformance/rules.tsv`'s header states a row count and, for
`signal-is-the-last-action`, a `static(19)` that its comment glosses as *binds all
nineteen kinds*. Both move.

**And one ADR enumerates the families.** `docs/adr/corpus-rules-have-one-owner.md`
rule 2 reads *One family — the five reviews, the five integrations, the two
research halves —*, which is exact today and wrong the moment `editorial.md`
ships. Edit that record in place; the placement function itself is unchanged, so
this is the enumeration moving and not a decision being revisited.

## The launch templates the human must declare

Four new keys in `~/.config/grove/config.kdl`. A delta in an untracked
`.grove.kdl` overrides but never supplies, so the personal file is the one that
must gain them. Hand these to the human verbatim, with a real command in place of
the placeholder:

```kdl
draft "grove-claude --session ${session_name} ${prompt}"
copy-edit "grove-claude --session ${session_name} ${prompt}"
art "grove-claude --session ${session_name} ${prompt}"
proof "grove-claude --session ${session_name} ${prompt}"
```

Four keys and no more: the family takes no `review-*` or `integrate-review-*`
steps, so it needs none of those keys. If the human wants a second vendor on this
arm, `copy-edit` and `proof` are the natural axis — they are the two whole-document
reads, and a reader that did not write the draft is what a fresh context buys.

**Why this is a gate and not a footnote.** A kind for which no launch template
resolves is refused **before the tree is mutated** — at `leaf-add`, `leaf-insert`
and `leaf-decompose`, not only at launch. So the very first `crate-books-k14`
session that tries `leaf-decompose … --kind draft` fails at the verb, in an arm
with no human present to read the refusal.

## Done when

- One `grove-<kind>` skill exists per extracted kind, plus
  `grove/references/editorial.md`, installed by the same route as their
  neighbours, and each is reachable on its kind's loaded path.
- The conformance runner and its suite cover the new kinds, and
  `bash plugins/install.test.sh`, `bash plugins/grove/conformance.sh` and
  `bash plugins/grove/conformance.test.sh` all pass.
- Every place that states the methodology's kind set or its count agrees with
  what now ships — found by enumerating and classifying every statement of the
  set, not by sweeping a list of files. The four structural statements above are
  rewritten rather than renumbered.
- `bash scripts/check.sh` passes.
- The human has been given the exact launch-template entries the new kinds need
  and has confirmed they exist.

## Notes

**This leaf ends at a human gate, and that is not optional.** A kind for which no
launch template resolves is refused before the tree is mutated
(`docs/adr/a-kind-is-an-open-token.md`), so the very first `crate-books-k14`
session that tries to cut a stage leaf fails at the verb — in an arm with no
human present. Name the entries concretely, in the form the human's configuration
takes, and stop and ask rather than assuming.

**The methodology and the binary have separate lifetimes.** Editing a skill
reaches a session as soon as the install route resolves to this checkout;
nothing checks that a skill and the installed `grove-llm` agree. Verify this work
by reading the files and running the suites, never by expecting the next session
in the same loop to behave differently.

**Do not widen a discipline while authoring it.** The four charters above are what
the evidence supports and no more. A stage's skill that grows an extra
responsibility because it reads thin has re-created a merged stage under another
name, which is the outcome the whole pilot was ordered to prevent.

## Decisions (running log)

1. **`static(19)` became `static(23)`, and the runner's case arms moved with
   it.** The token is a label the runner maps to *all shipped kinds*, not a
   computation, so leaving it would have parked a stale count inside the
   instrument that exists to catch stale counts. `static(18)` — *all but
   `finish`*, unused by any row and kept for the deleted spec's sake — became
   `static(22)` for the same reason. The comment paragraph comparing this
   file's predicate to that spec's was deleted rather than renumbered: the spec
   went with `content/`, and the comparison had no second side left.

2. **The pinned wording of `kind-set-belongs-to-the-methodology` was made
   count-free.** It read *a twentieth is authored, not compiled* and would have
   had to move to *a twenty-fourth*; it now reads *the next one is authored, not
   compiled*, which is the structural fact and never moves again. This is
   `execute.md`'s no-self-invalidating-count rule applied to a rule's own
   canonical phrase.

3. **`editorial.md` is listed in the spine's `SKILL.md` beside the other three
   family files.** The alternative — leaving it off so its only incoming edge
   was the four opening imperatives — was tried and rejected on reading the
   existing bytes: `review.md`, `integrate-review.md` and `research.md` are
   *already* named there, so every family file is on every kind's composed
   loaded path today, and omitting the fourth would have made the spine's own
   family list wrong without buying the property. `CONTEXT.md`'s
   *Family reference file* entry now states that directly, because a first draft
   of it claimed the exclusivity that does not hold.

4. **`TASK-FORMAT.md` names no path for the family file.** Naming
   `grove/references/editorial.md` there would have added a *second* incoming
   edge from a file every kind reads — harmless to the assertions, but it makes
   the taxonomy's home a router. The sentence states the relationship instead.

5. **`docs/ARCHITECTURE.md` gained a second table rather than four more rows.**
   The producer table's columns are *Review* and *Integration*; four rows of
   `— | —` inside it read as *not yet authored*. A separate *Editorial stage*
   table with a stated legend says the dash positively, which is the structural
   defect `pipeline-kinds-k72` named.

6. **`plugins/grove/README.md`'s fat/thin split gained a third category.** The
   four stages are neither: each directs a load of its family's file *and*
   carries its own charter inline. Recorded as a hybrid rather than forced into
   one of the two existing counts.

7. **Every `nineteen` and every `combine-research` occurrence in the repository
   was enumerated and classified**, rather than a file list swept. Historical
   narration stays: `CHANGELOG.md`, `docs/preservation-baseline.md` (a frozen
   v19.3.0 measurement), the deleted-enum comments in `task_name.rs`, `leaf.rs`,
   `kind.rs`, `cli.rs` and the three ADRs that narrate what the binary used to
   hold, and `plugins/CONTEXT.md`/`plugins/README.md`'s past-tense *twenty
   skills declared `[claude-code]`*. Current-state statements were rewritten,
   and where the count was incidental (`prompt.rs`'s *nineteen kinds, one
   ending*, `grove/README.md`'s *links all twenty*) it was replaced by the
   quantifier rather than by a bigger number.

8. **Both conformance controls were watched to fail before the clean read was
   credited.** Mutating `editorial.md`'s pinned *a producer, not a reviewer*
   failed `editorial-stage-is-a-producer`; removing `grove-art`'s named load of
   the family file failed thirteen rows. A runner that had only ever read clean
   over the new rows would have been indistinguishable from one that never
   reached them.

9. **The four kinds are the extracted set and no more.** No stage's skill grew a
   responsibility to fill a thin page; `draft` carries the two folded charters
   explicitly because the pilot folded them, not because it had room.

10. **The launch templates were handed over as a choice, and the human took the
    cross-vendor split.** `draft` and `art` run on `claude/opus` (they write and
    draw); `copy-edit` and `proof` run on `codex/sol-high` — the two
    whole-document reads get a reader that did not write the draft, which is the
    same argument the existing `review-*` keys already make. The block was
    appended to `~/.config/grove/config.kdl` — the human's own file, outside
    anything this leaf commits — beside a header comment whose *Nineteen session
    kinds* had gone stale for the same reason every other count here did.

11. **The gate was closed by observation, not by assertion.** In a scratch jj
    repo, the installed `grove-llm leaf-add . book --kind draft --kind copy-edit
    --kind art --kind proof` wrote all four leaves, and the same verb with an
    undeclared kind was refused with *key … does not resolve: no template for
    it* — the exact refusal that would otherwise have stopped the first
    `crate-books-k14` session with no human present. The negative control firing
    is what makes the positive read evidence rather than a green that any
    configuration would have produced.

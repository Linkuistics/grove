# documentation-guidance

How Grove's installed skills carry documentation practice: which file owns
each rule, what the stages hand each other, how the guidance is accepted, and
the increments that deliver it. Evidence is cited from
[`documentation-evidence.md`](../research/documentation-evidence.md) (IDs
`X-`, `DR-`, `CE-`, `AR-`, `PR-`, `FM-`, `CV-`, `DS-`, `WU-`, `SV-`, `RC-`,
`CF-`) and [`documentation-toolchain-a.md`](../research/documentation-toolchain-a.md)
(the 2026-09-30 recheck). This document restates neither.

The visual companion is [`docs/design/documentation-guidance/`](../design/documentation-guidance/README.md).

## Problem

The editorial pipeline passed its own checks and a model proxy while the human
reader could not follow the text (RC-1). Over the next four checkpoints the
human located specific defects: compressed relationships, negative framing,
notation taught before the model it expresses, direct rule entries without
context, notation lessons for a reader already fluent in it, and one source
platform framed as the whole domain (RC-2–RC-5). Grove's stage skills say
nothing about any of these. The structure brief `draft` requires states the
audience, the section plan and the emphasis, but not the reader's expertise
per domain, the entry routes a page must serve, or the domain's intended
breadth.

The issues propose about 180 remedies drawn from style guides, research and
those checkpoints. Copying them into the skills would build the single large
documentation skill the human ruled out (X-01). It would also add checks
because a source recommends them, which the pilot's bar forbids (X-17).

## Solution

Each rule goes where the placement function in
[`corpus-rules-have-one-owner`](../adr/corpus-rules-have-one-owner.md) sends
it, and a session loads a file only when its condition fires. Four artifacts
carry the new rules and one extends an existing reference. Two further files
carry values those rules name rather than rules of their own: the dated
capability snapshot and the house style.

| Owner | Kind of file | Who opens it, and when |
|---|---|---|
| `grove/DOCUMENT-CONTRACT-FORMAT.md` (new) | spine format file: the document contract | authors (requirements, design, planning) when writing a contract; each stage at the parts its skill names |
| `grove/FIGURE-FORMAT.md` (new) | spine format file: the figure contract | `art` for every figure; `proof` when checking figures; `requirements` and `design` for a view drawn in an exchange |
| `grove/references/documentation-toolchain.md` (new) | dated capability snapshot, data rather than rules | only when choosing a format, a renderer or a viewing route |
| `grove/references/house-style.md` (new) | the prose contract Grove ships: contract values, not stage rules (§9) | an author or stage working on a set that takes the house style for any point |
| `grove-requirements/reader-test.md` (new) | conditional procedure inside one kind's skill, under the bounded exception `grilling.md` already takes (§6) | a `requirements` leaf that holds a reader checkpoint |
| `grove/references/execute.md` (extended) | loop-step reference | every session, when it starts work. Its teaching section binds `draft` and `copy-edit` on a document and requirements and design sessions on an exchange (§3). Its exchange paragraph binds requirements and design exchanges, and names the figure contract and the house style for their views and prose (§7) |

The four stage skills gain their own charter lines, and each names the
contract parts it reads. `grove-draft` and `grove-copy-edit` also name the
teaching section of `execute.md`. `editorial.md` gains one edge naming the contract
format. `visual-design-discussion.md` gives its shared figure rules to the
figure contract and keeps what is specific to a live design discussion.

[The owner map](../design/documentation-guidance/README.md#views) draws this
placement as a figure, and [the contract flow](../design/documentation-guidance/README.md#views)
shows who writes and reads each part of the contract.

## Decisions

### 1. The document contract is an artifact with its own format file

A document declares a **document contract**, and the spine's new
`DOCUMENT-CONTRACT-FORMAT.md` is that artifact's format file. Authors and all
four stages act on the contract, so no single kind or family owns it. The
placement function's rule 3 (`artifact:A` → A's format file) therefore decides
the home without further judgement. The spine's `SKILL.md` gains one
`trigger` sentence in its Artifacts register. `editorial.md` names the format
file where it now says the contract supplies every class a charter names.

The contract has three parts and one schedule. Each part may be its own file,
declared by path from the document's leaf body or node brief. The prose
contract and the format part usually serve a whole document set.

- **Structure brief** — decided by the human, per document:
  - the reader's outcome: what they can find, understand and do afterwards;
  - their expertise per domain and per representation (notation, code
    language, diagram kind), including what they need no teaching in;
  - the intended domain, stated apart from the repository, ecosystem or
    examples that motivated the document, with subject, producers and
    consumers told apart where they differ;
  - the section plan in conceptual order: the model and the reader's task
    before the notation that expresses them, and a purpose for each page;
  - the entry routes the document must serve: the learning route, and the
    direct entries through rules, reference anchors, examples and figures;
  - emphasis and exclusions;
  - term risks: terms whose everyday or neighbouring meaning intrudes, with
    the difference to state;
  - the coverage inventory, enumerated from the subject itself, when the
    document claims coverage;
  - examples policy and figure needs, stated as the reader's questions and
    levels.
- **Prose contract** — per set: its base (a style guide, another prose
  contract, or the house style, §9) and the exceptions the set takes; the
  keyword convention when normative keywords appear; the term list; code
  typography; example labels; and the convention that keeps normative text
  apart from non-normative aids.
- **Format and media** — per set: the format; the admitted figure media and
  renderers with pinned versions; any generation step; and the viewing
  environments readers actually use.
- **Reader checkpoints** — when the intended reader, or a labelled stand-in,
  reads, with which tasks, entry surfaces and viewing environment.

**Absent parts take stated defaults.** They do not block a stage.

| Absent | Default |
|---|---|
| coverage inventory | the document makes no coverage claim |
| term risks, examples policy, figure needs | the stage charters' own rules apply unchanged |
| prose contract | the house style (§9) |
| format and media | the set's existing format; figures in the page's own bytes only; the raw file is the viewing environment |
| reader checkpoints | none is scheduled, and no stage result is read as reader understanding |

The format file's rules bind the authors and the four stages:
`{requirements, design, planning, draft, copy-edit, art, proof}`. Authors reach
the file through the spine's trigger:
`on(writing or reading a document contract) @ grove/SKILL.md`. Stages reach it
through `editorial.md`, `static(editorial)`, and each stage's skill names the
parts it reads (§2).

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `contract-declared-by-path` | A document declares its contract by path from its leaf body or node brief; a part may be its own file shared across a document set. | DR-08, DR-09, FM-01 | S1 |
| `contract-structure-decided-by-the-human` | The human makes the substantive structure decisions. The authoring session records the human's own words and the locations they identify rather than a paraphrase. | X-11, X-25 | S1 |
| `contract-outcome-is-find-understand-use` | The reader outcome states what the reader can find, understand and do. A readability score, grade level or length limit is not an outcome and gates nothing. | X-08, CE-11, CE-15, DR-34 | S1 |
| `contract-structure-brief-fields` | The structure brief states the fields listed above: expertise per domain and representation with what not to teach; the intended domain; the model-first section plan with page purposes; the entry routes; emphasis and exclusions; term risks; any coverage inventory; examples policy; and figure needs. | X-05, X-09, X-23, DR-01–DR-07, DR-12, DR-28, DR-29, DR-32, DR-45, DR-47, WU-01, SV-09 | S1 |
| `contract-prose-contract-fields` | The prose contract names its base and exceptions, its keyword convention, term list, code typography and example labels, and how normative text and aids are told apart. | X-07, X-24, DR-08, CE-02–CE-09, CE-12–CE-14, CE-16, SV-05 | S1 |
| `contract-absent-parts-default` | An absent part takes the default in the table above and blocks no stage. | — (design) | S1 |

`reader-checkpoints-scheduled-after-proof` (§6), the two format rules (§5) and
`prose-contract-builds-on-its-base` (§9) are the file's other rules.

**The `draft` precondition keeps its three citable items, and each one
deepens.**

1. *Who the reader is* now includes their expertise per domain and
   representation and what not to teach.
2. *The ordered section plan* now includes the model-before-notation order,
   page purposes and entry routes.
3. *What deserves emphasis and what is excluded* now includes the intended
   domain.

These additions are the facts the checkpoints showed missing, and like the
original three they are nowhere in the sources. That absence is the reason
the precondition exists (see
[`the-editorial-pipeline-is-four-kinds`](../adr/the-editorial-pipeline-is-four-kinds.md)).
The other fields are optional and default when absent.

**A reader checkpoint runs outside the four stages.** It is a `requirements`
leaf. The session that cuts the document's leaf places it after the
document's `proof`. Its procedure is `grove-requirements/reader-test.md`,
reached by one condition in `grove-requirements/SKILL.md`. This follows the
APIAnyware precedent and `sample-reading-k11`: every human reading checkpoint
there was a `requirements` leaf. It mints no kind, so no user's launch
configuration needs a new template.

### 2. Each stage reads named parts and owns named text

| Stage | Reads from the contract | Owns the content of | Owns the wording of |
|---|---|---|---|
| `draft` | the structure brief (all of it); format and media; the prose contract's keyword, example-label, normative-aid and layout conventions, as the contract format resolves them | structure, teaching, examples, headings' and openings' subject, summaries, example labels, normative text, sections, lists and tables | normative sentences |
| `copy-edit` | the prose contract; term risks; expertise (what not to teach) | — | headings, openings, summaries (table summaries included), example labels, all body prose except normative sentences |
| `art` | figure needs; expertise per representation; admitted media and renderers; viewing environments; the prose contract's terms | every figure | figure titles, labels, keys, captions, role statements, text alternatives |
| `proof` | the whole contract: outcome, entry routes, inventory, status, viewing environments | — | nothing; it corrects errors in every class |

**The section plan outranks a layout convention.** The human decides the
sections (`contract-structure-decided-by-the-human`), so the headings the plan
states stand even where a layout convention would not add them. A layout
convention governs what `draft` adds within the plan: further heading levels,
lists and tables.

The existing correction routes carry the handoffs:

- **Missing teaching is not a sentence defect.** When `copy-edit` finds an
  untaught relationship or a missing model, it cuts a `draft` correction run
  and does not repair the gap in sentences (CE-20). The existing
  `copy-edit-does-not-restructure` rule gains this case.
- **Layout above the paragraph goes back.** `copy-edit` splits a paragraph
  that carries two ideas and corrects emphasis. A heading level, list or table
  that the layout convention calls for goes back as a `draft` correction run,
  because adding or removing one regroups the text.
- **Figure-text defects go forward.** `copy-edit` runs before `art`, so a
  defect in an existing figure's text goes to the node brief's
  `## Handed forward` list for `art`.
- **`copy-edit` leaves normative wording to `draft`.** It may reword teaching
  text around a normative sentence but not the sentence itself, and a defect in
  one goes back as a `draft` correction run. The restriction is a boundary of
  `copy-edit`'s charter. `proof` keeps every class in charter, so it corrects an
  error in a normative sentence itself, such as a typo or a condition dropped
  from the rule the sentence states.

### 3. Stage charters gain rules the reader evidence supports

Each rule below is stated once, in the file named, and is delivered on the
loaded path of the kinds it binds. The *Sources* column cites inventory IDs.
The *Seam* column gives the acceptance case (see [Test seams](#test-seams)).

**`grove/references/execute.md`, teaching section** — occasion
`step:Execute`. These nine rules teach whoever reads the text, whether a
document's reader or the human in an exchange (§7). Every rule binds
requirements and design exchanges. On a document, the *Stages* column names
the editorial stages a rule binds: five bind `draft` and `copy-edit`, and four
keep the one stage they bound before they moved.

| Rule | Stages | Statement | Sources | Seam |
|---|---|---|---|---|
| `teach-the-model-before-notation` | `draft`, `copy-edit` | Explain the subject model and the reader's task first, then describe the notation, identifiers or codes that express them by reference to that model. | DR-44, DR-38, X-10 | S1, S2:T2 |
| `establish-actors-before-labels` | `draft`, `copy-edit` | Before an abstract label, establish in the local explanation who acts, on what, and with what consequence, whose viewpoint a relative term takes, and what each case actually does. After any passage, whether or not it introduces a label, the intended reader can say who acts, what they do and what follows; a synonym for an abstract noun leaves that gap open. | DR-33, DR-35, DR-40, X-22, CE-17, CE-06, CE-21 | S1, S2:T1 |
| `explain-what-the-reader-cannot-infer` | `draft`, `copy-edit` | Explain the referents, relationships, conditions and consequences this reader cannot infer, in an explained example as in running text. Each explanatory sentence adds meaning this reader needs, and none narrates what they already read, such as a familiar token. Explanation length follows that need: a length limit yields to it, and a document's draft records the override. Removing narration never compresses an unfamiliar concept back into jargon. | DR-46, X-26, DR-39, DR-31, DR-34, X-27, CE-22 | S1, S2:H3, T1 |
| `compare-then-principle-then-limit` | `draft`, `copy-edit` | Teach a new distinction, or put a choice between options, by setting the cases or options side by side, stating the principle that separates them, and naming where the comparison breaks. An analogy states its basis and where it fails; a distinction between kinds is shown through their uses. | DR-19, DR-18, DR-41, CE-10, X-10, CV-03 | S1, S2:T7, E3 |
| `one-name-one-entity` | `draft`, `copy-edit` | Text read together (introductions, code labels, captions and tables) uses one name for one entity, and a use colliding with a declared term risk is flagged. | CE-22, CE-01, AR-15 | S1, S2:T10 |
| `domain-roles-and-choices-kept-distinct` | `draft` | Establish the intended domain before comparing alternatives and judge a source's assumptions against it; keep subject, producer and consumer roles apart; keep what the problem requires apart from what the design chose, with its costs and alternatives in view; a list of examples implies no property the list does not establish. | DR-47, DR-48, DR-49, DR-10, DR-11, X-31, X-32 | S1, S2:T5 |
| `normative-text-traces-and-ambiguity-surfaces` | `draft` | Every normative statement traces to its rule, a rejected form names the rule that rejects it, an ambiguity in the subject is recorded rather than resolved in silence, and added teaching preserves normative force and exact restrictions. | DR-16, DR-22, SV-06, CE-23 | S1, S2:H2 |
| `no-invented-observations` | `draft` | Expected observations are checkable by inspection, and no output is shown that no tool produced. | DR-23 | S1, S2:H1 |
| `positive-subject-restrictions-recoverable` | `copy-edit` | Headings and openings name the subject, its operation and its conditions positively, and every exact prohibition and rejection condition stays recoverable from its rule. | CE-18, DR-35 | S1, S2:T4 |

**Who applies them.** The section's first sentence names the kinds it binds,
and each of the four single-stage rules names its stage. `draft` and
`copy-edit` apply the rules that name them to the document they work on, each
within its charter (§2). `draft` writes what the text teaches. `copy-edit`
repairs how its sentences carry that teaching and sends missing teaching back
(`missing-teaching-goes-to-draft`). Requirements and design sessions apply
every rule to what they hand the human, as `exchange-reaches-the-teaching-rules`
(§7) says. `art` and `proof` open the file too, but the scope names neither.
`art-owns-figure-text` and `proof`'s own rules carry their share, and `proof`
keeps every class in charter.

Three rules merged into others when they moved, because one file cannot state
one rule twice: `copy-edit`'s `who-acts-what-follows` into
`establish-actors-before-labels`, `copy-edit`'s
`explanation-adds-needed-meaning` into `explain-what-the-reader-cannot-infer`,
and §7's `compare-two-state-principle-name-break` into
`compare-then-principle-then-limit`. Each pair had been one practice split by
stage or by occasion. The merged actor rule keeps `who-acts-what-follows` as
its own sentence, so the passage test still applies where no label appears.
The four single-stage rules moved unchanged and keep their stage, so neither
stage's charter widens. §7 records which of the other teaching rules reach an
exchange, and why the rest do not.

**Why `execute.md`.** On 2026-09-30 the human directed that requirements and
design exchanges take the features of good documentation, "because it requires
understanding". These nine rules therefore each bind three or four kinds. Two
of them, `requirements` and `design`, belong to no family, so the placement
function's rule 2 does not fire. A stage meets the rules while it writes or
edits the document, and an exchange meets them while it hands a question back.
Both happen at Execute. Neither the document nor the message has a format file.
The rules read the contract's declared expertise, but a session meets them
while writing prose, not while writing or reading the contract, so rule 3 does
not fire either. Rule 6 sends them to the Execute step's reference. The
placement was computed; the human chose to accept it rather than change the
function.

The placement has three costs, and the human accepted them:

- **Every session loads the section.** It adds about 480 words to every kind's
  path, including kinds it does not bind. That is 8–10% of the skill text a
  session of any of six sampled kinds always loads (4,973–6,235 words, measured
  on 2026-09-30).
  [`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md)
  now names this as the one exception to "no documentation skill that every
  session loads" (X-01). It is nine rules in a loop-step reference, not a
  documentation skill.
- **`draft` and `copy-edit` lose one-hop delivery of these rules.** The plugin
  README records that a rule in a kind's own `SKILL.md` arrives in one hop,
  while a rule in the spine depends on a second hop that no test measures.
  `draft` keeps four of its eleven document rules in its own skill, and
  `copy-edit` four of its eight. Seam 1 shows only that the rules are on each
  stage's path; whether a stage reads them shows only in seam 2's runs.
- **The section grows with every rule that later reaches it.** It held five
  rules, about 320 words, until a design review found four more whose forms
  an exchange can contain, and the human accepted the growth to nine. A shared
  section much larger than this one would reopen the question.

Two other homes were weighed and rejected:

- **A file only the four kinds open,** such as `grove/references/teaching.md`.
  It saves the load, but it files rules by topic, which
  [`corpus-rules-have-one-owner`](../adr/corpus-rules-have-one-owner.md)
  rejects with "Reopen never".
- **The §3 owners kept, with an exchange form in `execute.md`.** This states
  each rule twice, which
  [`restatement-declares-its-class`](../adr/restatement-declares-its-class.md)
  calls a defect even while the two agree. Having `execute.md` name the stage
  skills instead would hand-assign the owner, and would send a requirements
  session into another kind's `SKILL.md`.

**`grove-draft/SKILL.md`** — bound `{draft}`, occasion `static({draft})`. It
names the teaching section as rules that bind its drafting.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `structure-brief-three-items-deepened` | The three citable items carry expertise per domain and representation, the model-first plan with page purposes and entry routes, and the intended domain. | DR-02, DR-45, DR-47, X-25, WU-01 | S1, S2:T2 |
| `local-context-at-every-declared-entry` | Each declared entry carries the local context and example it needs, and teaching support at a normative entry stays distinguishable from the normative text. | DR-13, DR-43, DR-12, X-23, X-09, SV-04, SV-05 | S1, S2:T3 |
| `examples-labelled-canonical-complete` | Label each example complete or fragment and say what a fragment omits; keep canonical examples complete; place a local excerpt where each construct is first met. | DR-17, DR-30, DR-37, DR-42, DR-21, DR-24 | S1, S2:T8 |
| `reference-entries-shaped-by-subject` | Reference entries share fields shaped by the thing described and carry identifiers that survive heading edits. | DR-25, DR-26, DR-32 | S1, S2:T9 |

`draft-owns-structure-and-technical-truth` already holds DR-15, DR-28 and D1.

**`grove-copy-edit/SKILL.md`** — bound `{copy-edit}`, occasion
`static({copy-edit})`. It names the teaching section as rules that bind its
sentence work.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `copy-edit-reads-the-prose-contract` | Read the set's prose contract as the contract format resolves it (the house style when the set declares none), the term risks and the declared expertise. | CE-16, X-07 | S1, S2:T12 |
| `tests-reach-openings-and-labels` | The tests cover headings, first sentences, summaries including table summaries, and example labels, not only body text. | CE-19, DR-36 | S1, S2:T4, T10 |
| `missing-teaching-goes-to-draft` | An untaught relationship or missing model is sent back as a `draft` correction run, never repaired in sentences. | CE-20, X-22 | S1, S2:H4 |
| `normative-wording-preserved` | Reword teaching text only; a defect in a normative sentence's wording or force goes back as a `draft` correction run rather than being repaired here. | CE-23, CE-02, CE-08 | S1, S2:H2 |

**`grove-art/SKILL.md`** — bound `{art}`, occasion `static({art})`.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `art-meets-the-figure-contract` | Every figure drawn or kept meets `FIGURE-FORMAT.md`, and `art` owns source and rendering agreement. | X-14, AR-10, AR-14 | S1 |
| `art-owns-figure-text` | Figure titles, labels, keys, captions, role statements and text alternatives are `art`'s, written in the prose contract's terms and read with the adjacent prose. | AR-13 | S1, S2:T10 |
| `declined-figure-names-one-of-three-reasons` (existing, widened) | A host that cannot render the admitted notation is a case of *the medium could not carry it*. | FM-04 | S1 |

**`grove-proof/SKILL.md`** — bound `{proof}`, occasion `static({proof})`.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `walk-every-declared-entry` | Walk each declared entry route from its entry, including the direct rule, reference, example and figure entries, and check that model context and examples are present where new notation appears. | PR-01, PR-22, X-29, X-23 | S1, S2:T3 |
| `coverage-against-the-inventory` | Check coverage against the contract's inventory, when there is one. | PR-02, DR-06 | S1, S2:T9 |
| `read-in-the-declared-environment` | Verify the document where its readers read it: figures load, links and anchors resolve, aids are set apart from the normative text beside them, and each check is reported as the property it establishes. | PR-26, FM-05, PR-03, PR-24, AR-14, X-24 | S1, S2:T3, T11, T12 |
| `proof-reads-for-whole-document-coherence` (existing, widened) | Status statements, terms and figure vocabulary agree across the set. | PR-05, PR-04 | S1, S2:T10 |
| `proof-claims-no-understanding` | A clean proof establishes coherence and delivery, not reader understanding, and names the reader checkpoint that would test it. | PR-15, PR-28, X-29 | S1 |
| `proof-reads-headings-and-openings-in-order` | Read the headings, openings, table summaries and captions in document order: together they tell the document's story, each names its subject positively, and no relationship is left compressed where the reader first meets it. Keep a human reader's test as separate evidence. | PR-20, PR-06 | S1, S2:T1, T4 |

`proof` keeps every class in charter and corrects what it finds in any class.
It does not load `copy-edit`'s skill, so `copy-edit`'s rules do not reach it,
and their charter boundaries (`missing-teaching-goes-to-draft`,
`normative-wording-preserved`) do not bind it. It opens `execute.md`, but the
teaching section's scope does not name it. What `proof` alone sees is the
sequence across the whole document, and its own rule above carries the reader
evidence's request for that read (PR-20).

### 4. The figure contract

`grove/FIGURE-FORMAT.md` is the format file for a figure. `art` names it for
every figure. `proof` names it when checking figures. `visual-design-discussion.md`
names it for a design session's views. The exchange paragraph of `execute.md`
names it for a view drawn in any requirements or design exchange (§7). It
absorbs the figure rules that `visual-design-discussion.md` now holds:
choosing the question, level, notation and renderer; keeping abstraction levels
explicit; keeping the source editable with its version and command; and
marking a reading entry. The live-discussion material stays where it is:
presentation, locatable views, change lists and delegation.

Bound: `{art, proof, requirements, design}`. The occasion comes four ways:
`static({art})` through `grove-art/SKILL.md`;
`on(checking figures) @ grove-proof/SKILL.md`; `static({design})` through
`visual-design-discussion.md`, which `grove-design/SKILL.md` names; and
`on(drawing a view in a requirements or design exchange) @ grove/references/execute.md`.
The last is a `requirements` session's only route to the file, because that
kind loads no design reference. Its edge is therefore asserted on seam 1 with
the others.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `figure-answers-a-question-at-a-level` | Choose the view by the reader's question and level (model, structure, sequence, state, timing, grammar, or a concrete snippet), and keep notation, model and renderer distinct. Before a renderer choice turns on a dated capability fact, recheck the fact, and cite it with its date. | AR-15, AR-18, AR-03, X-03, DS-01 | S1, S2:T11 |
| `decompose-where-levels-add-value` | Run from context to detail through only the levels that add value: C4 where the subject is a software system or ecosystem, other views below it. Link back to an overview where that orients the reader, rather than repeating it on every page. | AR-01, AR-03, AR-12, X-03 | S1, S2:T11 |
| `figure-states-scope-omissions-entry` | A figure's title states its type and scope, it says what it omits, it has an identifiable reading entry, labels sit on the elements they name, and each one-way arrow carries a specific verb, on the arrow or, where many arrows share one verb, through a line style the key names. | AR-02, AR-04, AR-07, AR-08 | S1, S2:T11 |
| `conventions-identifiable-notation-per-reader` | Every convention the figure chooses (symbol, colour, line style or abbreviation) is identifiable on or beside it. Teaching the notation itself depends on the declared reader, and colour never carries meaning alone. | AR-17, AR-04, AR-16, AR-18, SV-10, SV-02 | S1, S2:T11, H3 |
| `one-visual-vocabulary` | A document set keeps one symbol and colour per concept, the prose's names and one legend convention. It adds a new diagram kind only when the question changes, and no decoration that carries no meaning. | AR-05, AR-09 | S1, S2:T11 |
| `text-reaches-figure-information` | The figure's information is reachable in text (prose, an equivalent textual form such as the grammar, and a text alternative) without narrating notation a fluent reader reads. | AR-06, AR-19 | S1, S2:T11, H3 |
| `place-by-question` | Place a figure where its question is asked: a semantic relationship with its explanation, and a lookup view at its lookup destination with links from elsewhere. | AR-19, X-28, AR-12 | S1, S2:T6 |
| `derived-figure-agrees-with-its-source` | A figure drawn from a formal source (a grammar, schema or model) is checked against that source, and what the notation cannot show is stated beside it with rule links. | AR-16 | S1, S2:T11 |
| `rendered-figure-ships-with-its-source` | A figure in a tool-rendered notation commits its editable source and tool-generated SVG together, with the tool version, the layout settings and the regeneration command recorded; a repeatable recipe checks regeneration, and drift after a tool upgrade is regenerated and reviewed. A figure in the page's own bytes is its own source. | AR-10, AR-11, FM-06, X-03 | S1 |
| `checks-establish-different-properties` | Source and SVG agreement, inspection in the declared environment, and reader understanding are separate results, and none stands in for another. | AR-14, PR-24 | S1 |
| `sketch-clauses` | In a live requirements or design exchange, sketch in the simplest dialect that answers the question, which may be simpler than a document's figures use. The content clauses above bind the sketch; the delivery clause binds once it is committed. | CV-06 | S1 |

`sketch-clauses` binds `{requirements, design}`. Those two kinds form no
family, and the rule's occasion is a figure, so the placement function's rule 3
puts it here rather than in either kind's skill.

A requirements or design session that preserves agreed exchange views commits
their SVG exports under the same clause. The viewer's live Mermaid rendering
stays available for sketches that are never committed. This companion's views
are committed that way, with `task figures:check`.

`rendered-figure-ships-with-its-source` is recorded in
[`a-rendered-figure-ships-with-its-source`](../adr/a-rendered-figure-ships-with-its-source.md).
Where a document's contract admits only the page's own bytes, as every
walkthrough book's does, [`a-book-carries-no-asset`](../adr/a-book-carries-no-asset.md)
still holds and this clause never fires.

Any C4 material adapted into the figure contract keeps its CC BY 4.0
attribution, in a provenance comment and a `LICENSES` entry (X-20).

`visual-design-discussion.md` keeps one rule of its own, bound `{design}` with
occasion `static({design})`. Its sketching advice moves into `sketch-clauses`,
which requirements exchanges now share.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `tool-candidates-are-dated` | The tool-candidate table reflects the capability snapshot's current facts and its check date. Structurizr's cloud service has ended; its SVG export needs a Playwright build. LikeC4 exports no SVG. PlantUML's offline C4 library is 2.13.0. GitHub renders Mermaid 11.17.2 against the viewer's 12.0.0 pin. TALA is bundled with D2. | DS-01, CF-01–CF-08, CF-19 | S1 |

### 5. Format selection and the dated capability snapshot

The rule for choosing a format lives in the contract's format part. It has the
contract format file's bound and occasions (§1).

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `format-per-set-with-the-human` | Choose the format per document set, with the human. Weigh reader tasks, publishing targets, source readability, accessibility, cross-reference checking, reuse, generation and tool cost. Before any choice in the format part (format, media, renderers or viewing environments) turns on a dated capability fact, recheck the fact, and cite it with its date. | FM-01, FM-02, SV-11 | S1 |
| `format-declares-media-and-environments` | The format part names the admitted figure media and renderers with pinned versions, any generation step, and the viewing environments readers use. | FM-03, DR-09, FM-06 | S1 |

`grove/references/documentation-toolchain.md` holds the facts, each dated
2026-09-30. It is read only when a format, renderer or viewing route is being
chosen or checked. Its occasion is
`on(choosing a format, renderer or viewing route)`, and the triggers sit in the
contract format file, `FIGURE-FORMAT.md` and `reader-test.md`. It carries:

- **formats and hosts:** what GFM, AsciiDoc, DocBook and Typst readers see on
  GitHub, in a local preview and in published output, and what stays legible
  without the tool (CF-21–CF-35 as rechecked);
- **renderers:** SVG route, legend support, source embedding, text
  accessibility and run-to-run stability for PlantUML with C4-PlantUML,
  Mermaid, D2, Structurizr, LikeC4, Kroki and Asciidoctor Diagram
  (CF-01–CF-20);
- **viewers:** resource loading in the VS Code preview (CF-40, with PR-25's
  instruction); GitHub's handling of SVG and relative links; what a session
  can check mechanically and what needs the viewer (toolchain survey §4.4);
- **pinning a regenerate-and-compare check** (toolchain survey §5.3).

The file states no rule. It holds dated facts, their provenance and example
recipes, and it opens by pointing to the rules that use them. Rechecking a
fact before a choice turns on it is a rule, and it applies at two artifacts.
The placement function gives such a rule one row per artifact: the format part
of a contract, in `format-per-set-with-the-human`, and a figure's renderer, in
`figure-answers-a-question-at-a-level`. The second row also covers exchange
views drawn with no contract. A reader checkpoint needs no third row, because
it observes whether figures load where the reader reads
(`checkpoint-states-its-frame-first`) rather than relying on a dated fact.

The snapshot carries no recommendation either. APIAnyware's assessment (FM-02)
was context-specific, so the snapshot keeps its facts and drops its verdict.
With this file, a session on another repository can follow the guidance
without reading `docs/`.

### 6. Reader checkpoints

The contract rule `reader-checkpoints-scheduled-after-proof` covers
scheduling. `grove-requirements/reader-test.md` carries the procedure, and one
condition in `grove-requirements/SKILL.md` reaches it. Bound: `{requirements}`,
occasion `on(this leaf holds a reader checkpoint) @ grove-requirements/SKILL.md`.

**This extends a bounded exception, and says so.** The placement function
sends a rule bound to one kind to that kind's skill. The plugin README reads
that as the body of the kind's `SKILL.md`, and names `grilling.md` as the one
procedure beside it, reached on a condition rather than read every session.
`reader-test.md` is a second procedure of that kind. It binds `requirements`
alone, and most `requirements` sessions hold no checkpoint, so inlining it
would put the whole method on every one of their paths. The exception stays
bounded: a file inside one kind's skill holds only that kind's rules, and one
condition in that kind's `SKILL.md` reaches it. Increment 5 reconciles the
README's statement of the fatness rule with this exception, as well as with
the visual-design reference and viewer asset that `grove-design` already
carries and the README's list omits. Its seam-1 rows assert the condition's
edge.

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `reader-checkpoints-scheduled-after-proof` (contract format) | A declared reader checkpoint is cut as a `requirements` leaf after the document's `proof` by the session that cuts the document's leaf. | PR-08, X-16, X-06 | S1 |
| `checkpoint-states-its-frame-first` | Before the read, state the audience, the reader tasks (find, understand, use), the entry surfaces and the viewing environment. Confirm that figures load there, and give the reader the steps that make them load. | PR-14, PR-25, PR-26, PR-27, X-29 | S1 |
| `early-failure-stops-the-read` | An early comprehension failure stops the read; record the reader's words and how far they got. | PR-17 | S1 |
| `record-locations-leave-unknowns` | Record the exact locations and entry surfaces the reader reports. Coverage they did not report stays unknown, and a quoted passage does not establish a route. | PR-21, PR-27, X-25, X-29 | S1 |
| `observe-outcomes-not-liking` | Observe find, understand and use through tasks, paraphrase of key terms or plus–minus marking. Liking is not the outcome, and a guideline's counts are not thresholds. | PR-09, PR-10, PR-11, PR-14, SV-07 | S1 |
| `evidence-classes-stay-apart` | Keep model-proxy results, mechanical checks, human acceptance and task performance apart. Label the proxy, and count probes that share one model context as one reader. Do not infer which stage caused a verdict. Agreement between independent readers is the validity signal. | PR-15, PR-16, PR-28, PR-29, PR-12, X-18 | S1 |
| `corrections-route-to-owners` | Route each correction to the owner of its class under the bounded correction rule, with the reader's words attached. Schedule the next read before dependent document work, starting it at the repaired entry. | PR-18, PR-19, PR-23, SV-03, X-22 | S1 |
| `acceptance-is-not-a-rewrite-order` | Acceptance with a bounded correction can coexist with later ambitions; record those ambitions as later work, not as instructions to rewrite now. | X-33 | S1 |

### 7. Requirements and design exchanges

The #9 practices extend the spine's *Handing a question back to a human*. The
extension goes there because the rule binds two kinds that do not form a
family. Its occasion is a loop step, and the placement function's rule 6
therefore selects `execute.md`. The three existing parts (name the trade-off,
recommend, give evidence) are not restated. Bound: `{requirements, design}` for
the paragraph scoped to exchanges; occasion `on(handing a question back to a
human) @ grove/SKILL.md`.

| Rule | Statement: its observable form in the exchange | Sources | Seam |
|---|---|---|---|
| `vocabulary-before-trade-off` | Before a trade-off that depends on a term or domain fact, the question defines it or asks about it, unless the exchange has already explained it or the human has shown they know what the trade-off needs of it. The human has shown that by stating it, reasoning with it correctly, or confirming it as the exchange stated it, in the exchange or in their own writing the session has read. Using, quoting or asking about a term is not that evidence. A doubt they voice about a meaning outweighs any earlier explanation or evidence. | CV-01, CV-02; the human's restatements of 2026-09-30 | S1, S2:E1 |
| `problem-before-proposal` | The question states the problem and what it costs before any option. | CV-04 | S1, S2:E2 |
| `restate-agreement-point-decisions` | At an agreement point (a spec or a decision that meets the ADR bar), the session asks the human to restate the decision in their own words and records the restatement beside their answer. | CV-05 | S1, S2:E4 |
| `exchange-prose-follows-the-house-style` | Prose handed to the human in the exchange follows the house style's points wherever their form occurs. | X-04; the human's direction of 2026-09-30 | S1, S2:T12 |
| `exchange-reaches-the-teaching-rules` | Prose handed to the human in the exchange follows the teaching section, with the human as its reader. What the exchange has already explained, and what the human has shown they know, as `vocabulary-before-trade-off` defines both, is what they already read. A name they have used for an entity is kept rather than replaced by a synonym, whether or not they have shown they know what it names; where the project uses that name for a different entity, the question says which is meant. A question reasons with plain names: an identifier, code or file name, the human's own included, follows the plain statement of what it names, only as a pointer where the human needs to find the thing. Where a name or notation is itself what the human decides, the candidates follow the plain statement of what they express. The rules reach every part the human decides from: the message, a question's header, each option's label and description, and the text of any view drawn for it. | X-04, CE-19, DR-36; the human's direction and restatement of 2026-09-30 | S1, S2:T1, T2, T4, T5, T10, H1, H2, H3, E3 |

A comparison of options in an exchange follows `compare-then-principle-then-limit`
in the teaching section (§3), which carries the practice CV-03 proposed. E3
remains its exchange class.

**Views in an exchange follow the figure contract (§4).** The scoped
paragraph names `grove/FIGURE-FORMAT.md` for any view drawn in a requirements
or design exchange. That sentence is a condition naming the owner, not a
restatement of its rules. It is how a `requirements` session reaches the
figure contract, including `sketch-clauses`, which carries the simpler-dialect
practice (CV-06) for both kinds.

**Prose in an exchange follows the house style (§9).** On 2026-09-30 the human
directed that every interaction with the human take the features of good
documentation, "because it requires understanding". The scoped paragraph
therefore names `grove/references/house-style.md` for the prose it hands to the
human, just as it names the figure contract for views. An exchange has no
document contract, so the house style applies directly. A point applies only
where its form occurs, so an exchange with no procedure has no step to format.

**The teaching rules reach an exchange (§3).** The same direction covers
teaching, and one exchange showed why it matters. Agreeing the version of
this design that survived review, the human could not restate the increments
and the measurement: "I'm not sure how the increments and measurements *work*". The
exchange had used labels and IDs, such as seams and T7–T11, before saying who
does what. Those are T1 and T2, the defects the reader checkpoints found in
documents. The same happened when this section was agreed. Each class ID in
the exchange had its meaning beside it, yet the human's restatement said that
"ids like T1... H3, make it difficult to reason about this", and asked why
`execute.md` has that name. A gloss beside an ID was not enough, so in an
exchange the plain name is the term a question reasons with. Both answers also
show that using a term is not knowing it: the human wrote "increments and
measurements" while saying they did not know how they work, and named
`execute.md` while asking about its name. So `vocabulary-before-trade-off`
counts as known what the exchange has already explained and what the human
has stated, reasoned with correctly or confirmed, and a doubt they voice
outweighs both. Requirements and design sessions load neither `draft` nor
`copy-edit`, so the rules now live in `execute.md`'s teaching section, and
`exchange-reaches-the-teaching-rules` applies them with the human as the
reader.

A rule reaches an exchange when an exchange can contain the form it governs
and show its defect. It then applies wherever that form occurs, as the house
style's points do. An exchange that quotes no rule gives the normative rule
nothing to govern; that does not keep the rule out of exchanges. The form
column says where each rule shows in an exchange, for the session and for
scoring. It is not a narrower statement of the rule: the whole rule binds, and
a clause the column leaves out still applies wherever its form occurs.

| Rule | Reaches an exchange? | Its observable form in the exchange, or the reason it does not reach one | Class |
|---|---|---|---|
| `teach-the-model-before-notation` | Yes | The question states the model and what the human is deciding before any notation. It reasons with plain names: an ID, rule slug, class code, file name or notation in a message or option (RC-1, T12, a rule's slug, `execute.md`) follows a plain statement of what it names, as a pointer where the human needs to find the thing. A gloss after the ID, which leaves the ID as the term reasoned with, is not enough. Notation that is itself the choice follows the plain statement of what it expresses. | T2 |
| `establish-actors-before-labels` | Yes | Before an abstract label, such as "increment", "seam" or "walk", the exchange establishes who acts (a session, the driver, the human, or an actor in the subject), on what, and with what consequence. After each passage, whether or not it introduces a label, the human can say who acts, what they do and what follows. | T1 |
| `explain-what-the-reader-cannot-infer` | Yes | The message and options explain what the human cannot infer: the terms, findings, constraints and consequences the session brings, each option's consequences, and a term the question depends on that the human used without showing they know it or asked about. They narrate nothing the human already read, their own words included. | H3, T1 |
| `compare-then-principle-then-limit` | Yes | The options, and any new distinction the question teaches, are set side by side with the principle that separates them and where the comparison breaks. An analogy states its basis and where it fails. | E3 |
| `one-name-one-entity` | Yes | One name per entity and one entity per name, across the messages, the question labels, any view and the artifact under discussion. A plain statement with its identifier as a pointer is one name. The human's own name is kept as `exchange-reaches-the-teaching-rules` says. | T10 |
| `domain-roles-and-choices-kept-distinct` | Yes | Before options are compared, the domain the choice serves is established: already explained or known, stated, or asked about first. A restriction or assumption from a motivating platform, source or example is judged against that domain rather than adopted as the domain's requirement. Subject, producer and consumer roles stay apart. Each option keeps what the problem requires apart from what it chooses, with its costs in view; stating the problem first is `problem-before-proposal`'s. A list of examples implies no property it does not establish. | T5 |
| `normative-text-traces-and-ambiguity-surfaces` | Yes | Where the exchange quotes, proposes or summarises a normative sentence, the human's own included, the summary and each option keep its force and every exact restriction and exception. An existing rule the exchange states or summarises names where it is stated, and a rejected form shown names the rule that rejects it. An ambiguity in the subject is put to the human or recorded, never resolved in silence. | H2 |
| `no-invented-observations` | Yes | Output presented as observed is output a tool produced, and an expected result is one the human can check by inspection. Output an option proposes is presented as a proposal, which is not an observation. | H1 |
| `positive-subject-restrictions-recoverable` | Yes | A question's header, the message's opening and each option's label name the subject, its operation and its conditions by what they are, where a positive subject exists. Where any of them reframes a proposed or quoted rule, each exact prohibition and rejection condition stays recoverable in the exchange. | T4 |
| `structure-brief-three-items-deepened` | No | A precondition `draft` checks on a document's structure brief, and an exchange has none to check. A session that writes a brief with the human follows the contract format's authoring rules (§1). | — |
| `local-context-at-every-declared-entry` | No | An exchange is read in order and declares no direct entries. A question's header and options, shown beside its message, are parts the teaching rules already reach through `exchange-reaches-the-teaching-rules`. A design discussion's deep links are `visual-design-discussion.md`'s. | — |
| `examples-labelled-canonical-complete` | No | A canonical example and a construct's first local excerpt are placements in a document's order. An example in an exchange or its view takes the house style's example-label point, whose label says whether it is complete and what a fragment omits (T12). | — |
| `reference-entries-shaped-by-subject` | No | Reference entries belong to a reference document. An exchange that shows some is showing the artifact under discussion, whose entries that document's own rules govern. | — |
| `copy-edit-reads-the-prose-contract` | Already | In exchange form, `exchange-prose-follows-the-house-style`. | T12 |
| `tests-reach-openings-and-labels` | No | It sets the reach of `copy-edit`'s own tests, which stay in its skill. In an exchange, `exchange-reaches-the-teaching-rules` sets the reach: the message, a question's header, each option's label and description, and view text. | — |
| `missing-teaching-goes-to-draft`, `normative-wording-preserved` | No | Boundaries between two stages' charters, and an exchange has no second stage. A summary of the human's own rule keeps its force under `normative-text-traces-and-ambiguity-surfaces`. | — |
| `proof`'s rules | No | They read a whole document set, its declared entries and its viewing environments. An exchange's views carry their delivery under the figure contract (§4). Its status changes as decisions settle, and `restate-agreement-point-decisions` confirms the final one; agreement of status statements within one exchange is not carried. | — |

`art`'s figure rules reach exchange views through the figure contract (§4), and
`who-acts-what-follows`, `explanation-adds-needed-meaning` and
`compare-two-state-principle-name-break` are merged into the rules above (§3).

### 8. Durable records

**ADRs.** Three decisions meet all three parts of the `ADR-FORMAT.md` test:

- [`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md):
  the structure brief becomes one part of a document contract, a spine artifact
  with its own format file, and the draft precondition keeps three deepened
  items. An absent prose contract takes Grove's house style, a closed list that
  follows Google's guide on the points it names, and a declared contract
  builds on the base it names (§9). Rejected: one documentation skill,
  ownership by the family file, restatement per kind, an all-fields
  precondition, shipping no default, a whole guide as the default, Microsoft as
  the base, stage-skill placement of the house style, a project-level lookup,
  all-or-nothing precedence, and a named guide that replaces the house style
  beneath it.
- [`a-reader-checkpoint-is-a-requirements-leaf`](../adr/a-reader-checkpoint-is-a-requirements-leaf.md):
  a human read is scheduled from the contract and runs as a `requirements` leaf
  after `proof`. Rejected: a `reader-test` kind, and a read inside `proof`.
- [`a-rendered-figure-ships-with-its-source`](../adr/a-rendered-figure-ships-with-its-source.md):
  source, SVG, version and command are committed together and regeneration is
  checked; the page's own bytes are the exception. Rejected: host-native
  rendering and source-only commits.

[`the-editorial-pipeline-is-four-kinds`](../adr/the-editorial-pipeline-is-four-kinds.md)
is edited in place so that its precondition paragraph names the deepened items.

These choices stay in this spec because they fail the ADR test's
*hard to reverse* part:

- teaching notation conditional on the declared reader;
- the exchange practices;
- the teaching section's home in `execute.md`, which one edit could move;
- the snapshot file;
- the reading and ownership tables;
- the conditional reader procedure's file (§6), which could move inline in one
  edit.

**Glossary.** The grove context's `CONTEXT.md` resolves **document contract**,
**structure brief**, **prose contract**, **house style**, **figure contract**,
**exchange** and **reader checkpoint**.

**Evidence bridge.** `documentation-evidence.md` gains a section pointing to
this spec and the three ADRs, as `ADR-FORMAT.md`'s research bridge requires.

**Taskfile.** This work adds `figures:check`. It regenerates every committed
figure in a figure directory from that directory's recorded command, then
compares the output with the committed SVG. The first user is this spec's
visual companion. The figures increment generalises the task wherever the
repository gains committed rendered figures. The evaluation harness owns its
own tasks (`evaluation-harness-k13`).

### 9. The house style

Grove ships one prose contract, the **house style**. A document set takes it
when its contract declares no prose contract, a declared prose contract that
names no base builds on it, and it supplies what a named style guide leaves
unsettled. Its purpose is prose a reader can understand and that reads as a
careful human writer's, laid out simply and consistent across a set. It is literary and visual: sentence and word conventions, and
layout conventions for text. The human directed this on 2026-09-30, after the
design above had been agreed.
[`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md)
records the decision with the rest of the contract, and the options it
rejected.

**The house style is a prose contract, not a set of stage rules.** A stage
rule binds whatever prose contract a set declares. Every point of the house
style yields to a declared contract, or a guide it names, that settles the
point otherwise. Its
points are therefore values of the contract, like the points of a project's
own prose contract, and the placement function does not place them. The rule
that makes them bind, `prose-contract-builds-on-its-base` below, has one owner.
It binds the authors and the four stages, and its occasion is the document
contract, so the placement function's rule 3 sends it to
`DOCUMENT-CONTRACT-FORMAT.md`.

The house style's own file, `grove/references/house-style.md`, sits in the
spine's references beside the capability snapshot. Two sentences reach it. The
contract format file names it as the prose contract's default, and
`execute.md`'s exchange paragraph names it for prose handed to the human (§7).
A session opens it only when a set takes it or when it writes to the human in
a requirements or design exchange. Two other homes were weighed and
rejected:

- `editorial.md` is on every stage's static path. The house style would load
  even where a declared contract replaces it, and neither the authors who
  decide whether a set takes it nor an exchange ever opens that file.
- A file inside `grove-copy-edit` would be out of reach of `draft`, `art` and
  the authors, who each read one of its fields (§2).

| Rule | Statement | Sources | Seam |
|---|---|---|---|
| `prose-contract-builds-on-its-base` | A set whose document contract declares no prose contract takes the house style. A prose contract may name one base: a style guide, another prose contract by path, or the house style. One that names no base builds on the house style. Each point resolves by a walk that starts at the set's prose contract, follows each base in turn, and ends at the house style: the first contract on the walk that states the point governs it. A style guide on the walk has the house style as its base. Where the guide settles a point otherwise than the house style, the guide governs the whole point; every other point comes from the house style. A contract the walk cannot follow ends the walk at the house style for every point still unresolved, and the session records the break once in its running log. The walk cannot follow a path that names no file, resolved from the file that names it, or a file it has already met, the starting contract included. | CE-16, X-07, DR-08; the human's directions of 2026-09-30 | S1, S2:T12, H5 |

**A project-wide house style needs no new mechanism.** It is a prose-contract
file that each set's contract names, as its prose contract or as its base.
Grove adds no project-level lookup.

**Every point resolves, and every walk ends.** The house style is the last
source on every walk, so every point has a value, including the five fields
the contract format requires. A style guide named as a base sits above it.
Where the guide settles a point otherwise, the guide's version governs the
whole point, because completing it from the house style could contradict it:
a tagged fence would contradict Google's indented code block. Where the guide
is silent on a point, or agrees with it, the house style's point applies.
Beneath Google, therefore, every house-style point applies except the code
point, since Google indents code blocks. Google's notices open with the same
bold label as the aid point and give no Markdown form. Google advises avoiding
"should" because it leaves unclear whether an action is required, and states
no convention for BCP 14 capitals. The keyword point also keeps lowercase
"should" out of normative sentences and gives the capitals a defined reading,
so the two do not conflict. A set that wants a guide's silence kept, or the
guide alone, says so point by point in its contract. A walk stops at the first
contract it cannot follow, so a cycle, a self-reference or a missing file still
ends in a value, and what an earlier contract on the walk stated still
governs. The recorded break lets the contract's author repair the chain. It
blocks no stage.

The human made two choices here on 2026-09-30. For the five required fields,
they chose to fall through to the house style rather than leave a field a
guide is silent on unsettled, which would let forms drift across a set.
Reconciling their restatement, "google takes priority, but where there is any
ambiguity, that is resolved by reference to the house style", they then
extended that to every point, taken whole.

**A guide is read for what it states itself.** A base names a style guide when
it names a published guide, by title or by a copy of it. A prose contract is a
file written as a set's contract. The walk does not follow the references a
guide defers to, such as Google's pointer to Microsoft for technical questions,
because each would bring its own conflicts into one base (CE-12–CE-15). A guide
the session cannot consult is still the base: the session applies what it can
establish of the guide and records what it could not check. That dependence on
recall or network access is the cost a set accepts by naming a guide, and the
reason the house style is a closed list rather than a guide. The house style's
file records, with the date it was checked, which of its points Google settles
otherwise, so the commonest base needs no lookup.

[The precedence view](../design/documentation-guidance/README.md#views) draws
how one point is resolved.

**What the house style says.** It follows
[Google's developer documentation style guide](https://developers.google.com/style)
on the points it names, and on no others. Where a point has no counterpart in
Google, the table names its source. Each point traces to inventory IDs, to the
human's direction, or to both. The file states each point in Grove's own words
with its source, and leaves the IDs and evidence classes to this spec. Google's pages are licensed
CC BY 4.0, so the file carries an attribution in a provenance comment and a
`LICENSES` entry. X-19 still keeps ISO 24495-1 text out of it.

| Point | The house style | Relation to Google | Sources | Applied by |
|---|---|---|---|---|
| Task headings | A task heading starts with a bare infinitive: "Create a leaf", not "Creating a leaf" or "To create a leaf". | Google's side of CE-12 | CE-12, CE-07 | `copy-edit` |
| Introducing steps | A complete sentence introduces a procedure, or nothing does when its heading already says what it does. A fragment that the numbered steps complete, such as "To create a leaf:", does not. | Google's side of CE-13 | CE-13 | `copy-edit` |
| Register | Plain written prose in a conversational rather than formal tone, with no slang, idiom or joke. It does not transcribe speech. | Google's side of CE-14 | CE-14, CE-10 | `copy-edit` |
| Length | No length target. The stage rules on explanation decide what stays. | Google's side of CE-15 | CE-15, DR-31, DR-34 | none; it adds no check |
| Steps | In a procedure step, the location, condition or goal the reader needs first comes before the action, and the result after it: "In the console, if the account is locked, click **Reset password**." The step's first sentence states its action in the imperative mood, and running text is indicative. Neither applies to a normative sentence. | as Google | CE-02, CE-03 | `copy-edit` |
| Demonstratives | "This" or "these" takes a noun where its referent could be unclear. It is a check, not a ban. | as Google | CE-04 | `copy-edit` |
| Voice | Active where the actor matters. Passive or stative where the reader does not need the actor, or where a statement concerns an artifact rather than an action. | Google's exceptions, and DQTI's for reference text | CE-05 | `copy-edit` |
| Front-loading | Headings, list items and paragraph openings start with their information-carrying words. | as Google | CE-07 | `copy-edit` |
| Keywords | A set that uses capitalised MUST, SHOULD, MAY and their kin states once that they carry their BCP 14 meaning (RFC 2119, RFC 8174), and that only the capitalised forms do. Lowercase "should" stays out of normative sentences. | **addition**: Google avoids "should" as ambiguous and does not address BCP 14; the point keeps lowercase "should" out of normative sentences and gives the capitals a defined meaning (prescriptive documentation, fetched 2026-09-30) | CE-08 | `draft` writes; `copy-edit` sends a defect back |
| Terms | The term list is the project's glossary, where there is one, with the structure brief's term risks. A glossary term is linked at its first use on each page. | no counterpart in Google | CE-01 | `copy-edit`; `art` for figure text |
| Code | Code font for identifiers, commands, file names, paths and literal values. A code block is fenced, and tagged with its language when it has one. | **exception**: Google indents Markdown code blocks by four spaces | CE-09 | `copy-edit` |
| Example labels | A label before each example begins "Complete example" or "Fragment", and a fragment's label says what it omits. | no counterpart in Google | DR-08, DR-17, DR-21 | `draft` writes; `copy-edit` rewords |
| Stock phrasing | No filler opener or closer ("It's worth noting", "In summary"), no signpost that repeats its heading, no closing paragraph that restates the section, no hedge that states no condition ("typically" without saying when), no inflated word where a plain one serves ("leverage", "robust", "seamless"), and no reflexive list of three. | extends Google's advice against placeholder phrases | X-27; the human's direction of 2026-09-30 | `copy-edit` |
| Simple layout | The headings the structure brief's section plan states stand. Beyond them, a heading level is added only for a section with at least two subsections. A paragraph carries one idea. Parallel items form a list, and items compared on the same attributes form a table. Bold marks only a run-in label or a term at its definition. Nothing is added for decoration alone. | Microsoft's heading advice; no counterpart in Google | the human's direction of 2026-09-30 | `draft` writes headings, lists and tables; `copy-edit` splits paragraphs, corrects emphasis, and sends heading, list and table defects back |
| Normative text and aids | In a reference or definition document, a note, example or summary beside normative text is visually set apart and opens with a label naming what it is, an example's label being its example label. In Markdown it is a blockquote beginning with the label ("> **Note:** …"): every line of it carries the `>` marker, and a blank line closes it before any text that follows. In another format it is the native equivalent, such as an AsciiDoc admonition. It is not normative. A restatement elsewhere links to the rule it restates. | Google's notices open with a bold label such as "**Note:**" in an aside; the house style adds the Markdown form and the non-normative status | X-24, SV-05; the human's direction of 2026-09-30 | `draft` |

**Evidence for the three points the human added.** The stock-phrasing,
simple-layout and visual-separation points came from the human's restatement
at agreement, which described the house style as "optimized for
understandability and non-LLM-style prose" and as "a mix of literary
constraints and visual design constraints", with non-normative text "visually
separated from normative content". Each rests first on that direction.
Stock phrasing extends the narration defect the human located at the fourth
checkpoint (X-27) and Google's advice against placeholder phrases. Simple
layout takes Microsoft's heading rule: "If you can't find at least two
distinct topics, skip the second-level headings"
([headings](https://learn.microsoft.com/en-us/style-guide/scannable-content/headings),
fetched 2026-09-30). Visual separation restores the visual form X-24 proposed,
which this design first narrowed as format-dependent. The house style now
states the form for Markdown and names the native equivalent elsewhere.
A blockquote is core CommonMark, so every conforming parser makes the aid a
block of its own. That establishes the parse, not what a reader sees. Whether
a viewer sets the block apart is checked where the set's readers read it, under
`read-in-the-declared-environment`. In the raw file, the default viewing
environment, the marker on every line is the visible boundary. CommonMark also
continues a blockquote lazily: an unmarked line directly after one joins it,
and a blank line ends it
([block quotes](https://spec.commonmark.org/0.31.2/#block-quotes), examples
232 and 248, fetched 2026-09-30). The point therefore marks every line and
closes the aid with a blank line, so normative text cannot fall inside it.
GitHub's alert syntax is a GitHub extension, so the house style does not use
it.

**The list is closed.** A Google rule the list does not name does not reach a
set through the house style. Two examples show why. The timeless-documentation
rule would remove status statements such as "not implemented", which proposals
and specs need (survey §6). The US-spelling rule would re-spell every set that
consistently uses another variety. A set that wants the whole guide names it
as its base.

**Where it takes a side, and why.** The evidence record marks the four
conflicts' quotations unverified, so each was rechecked against the guide's
current page on 2026-09-30.

- **Task headings (CE-12).** Google: "For a task-based heading, start with a
  bare infinitive" ([headings](https://developers.google.com/style/headings)).
  Microsoft: "Consider infinitive phrases, such as *To create a heading*"
  ([headings](https://learn.microsoft.com/en-us/style-guide/scannable-content/headings)),
  although its own step-by-step page heads its example with "Create a profile"
  and "Add an account". Both are house-style defaults. The tie-break is CE-07's
  observational evidence that a reader scanning headings often sees only the
  first two words (Nielsen 2009; Pernice 2017). "To" spends one of them on a
  word that carries no information.
- **Introducing steps (CE-13).** Google: "Don't introduce a procedure with a
  partial sentence that's completed by the numbered steps"
  ([procedures](https://developers.google.com/style/procedures)). Microsoft's
  example is "To create a group of tiles:"
  ([step-by-step instructions](https://learn.microsoft.com/en-us/style-guide/procedures-instructions/writing-step-by-step-instructions)).
  No evidence beyond the two guides' defaults bears on this point. The house
  style takes its base's side for consistency, and that is its only reason.
- **Register (CE-14).** Microsoft: "Write like you speak"
  ([top 10 tips](https://learn.microsoft.com/en-us/style-guide/top-10-tips-style-voice)).
  Google: "Don't try to write exactly the way you speak; you probably speak
  more colloquially and verbosely than you should write, at least for developer
  documentation" ([voice and tone](https://developers.google.com/style/tone)).
  Microsoft illustrates its tip with interface and marketing text, while the
  sets Grove drafts are specs, guides and reference. `copy-edit`'s existing
  figure-of-speech test, a defect the measured stage closed, already removes
  idiom (CE-10).
- **Length (CE-15).** Microsoft: "Shorter is always better" (top 10 tips, again
  illustrated with marketing text). Google: "Repeat a word if the redundancy
  improves comprehension"
  ([global audience](https://developers.google.com/style/translation)). Carroll
  and van der Meij's "not as a self-sufficient end" remains unverified. Grove's
  own evidence already settles the point: the human directed that length limits
  yield to needed explanation (DR-34), and redundancy harms expert readers
  (DR-31). The house style therefore sets no target, and the resolution this
  spec records for CE-15 stands.

**It is measured, because it adds checks to `copy-edit`.** X-17's bar applies to
knowledge added to a stage. The house style's effect on stage output is scored
as T12, and H5 guards the precedence rule. H2 guards the steps and keyword
points against rewording a normative sentence. H3 guards the length and
register points against cutting teaching, and the stock-phrasing point
against cutting a needed condition along with an empty hedge. H2 also guards
the visual-separation point: normative text never moves inside an aid's
blockquote. H4 guards against `copy-edit` turning a passage into steps,
regrouping sections, or adding a heading level, list or table, each of which
restructures the text and is `draft`'s work.

Only a case whose set takes the house style for a point its sources exercise
can exhibit T12. The forms a case can exercise include a task heading, a
procedure, an identifier in text, a code block, a normative keyword, an
example, an aid beside normative text and a section's heading levels, and any
prose can carry stock phrasing. The exchange case's prose can exhibit T12 too,
wherever those forms occur there. A case that exercises the aid point puts a
normative paragraph directly after an aid, so that H2 can observe the boundary.
H5 needs a case in which a contract on the walk, or the guide the walk ends
at, settles one of those points otherwise. A case whose set names a guide as its
base needs a point the guide is silent on, such as example labels, and one the
guide settles otherwise, such as indented code blocks under Google. In the
first, the house style's point is the expected result and its absence is T12.
In the second, applying the house style's point is H5. If the preregistered cases cannot exhibit one of
these classes, `preregistration-k12` declares that class unmeasured before any
output exists. With T12 unmeasured, the house style is reported as delivered
but unmeasured.

## Test seams

The three seams stay separate, as the root brief requires. A result on one
seam is never reported as a result on another.

**Seam 1: delivery.** Every rule in the tables above that lives in a skill gets
a `B` row in `plugins/grove/conformance/rules.tsv`. The row names its owner and
load predicate as given here, and each claim gets a near-miss fixture in
`conformance.test.sh`. Rows for rules moved out of `visual-design-discussion.md`
change owner rather than duplicate. Every cross-file route in §1–§7 is an edge
with both ends asserted, including the exchange paragraph's route from
`execute.md` to `FIGURE-FORMAT.md`, the reader-checkpoint condition in
`grove-requirements/SKILL.md`, the two routes to `house-style.md`, from
`DOCUMENT-CONTRACT-FORMAT.md` and from `execute.md`, and the two routes into
`execute.md`'s teaching section, from `grove-draft/SKILL.md` and from
`grove-copy-edit/SKILL.md`. The exchange paragraph reaches the teaching section
inside one file, so that condition records no edge. Rows for the teaching rules
name `execute.md` as their owner, and each row's load predicate names the
stages the rule binds and the two exchange kinds. The three merged rules get no
rows of their own. Each teaching row pins a phrase, so assertion 2 catches a
second statement in a stage skill. `establish-actors-before-labels` pins one
phrase from each of its two tests, and a near-miss fixture that drops the
passage test must fail the run. Assertion 2 (no rule has two owners) is the check
that X-01 holds.

The house style's points are contract values rather than rules, so they get no
`B` rows, but seam 1 still asserts their delivery. Each point carries a pinned
phrase for its operative content, and the run asserts two things of every
phrase: it occurs in `house-style.md`, and no other skill file states it. A
point stated in a stage skill would bind even where a declared contract
replaces it. Two near-miss fixtures must fail the run: one deletes a point from
the file, and one rewords a point's pinned phrase. The sweep's positive control
shows that the search works. The deletion fixture shows that the shipped list is
checked point by point. A pinned phrase catches a removed point or a reworded
phrase, not every weakening elsewhere in a point, and like every row it asserts
delivery, not obedience. Increments also carry three done-when checks
rather than rows: no ISO 24495-1 text appears in skill bytes (X-19), adapted C4
material keeps its attribution (X-20), and the house style keeps Google's.

**Seam 2: the preregistered comparison.** `preregistration-k12` targets every
class in this list and fixes the cases, scoring and decision rule itself. It may
score several classes with one set of criteria. A class the chosen cases cannot
exhibit is declared unmeasured before any output exists, and its rules are then
reported as delivered but unmeasured, never as improvements.

| Class | Defect observed in a stage's output | Rules |
|---|---|---|
| T1 compressed relationship | an abstract label used before the actor, action, object and consequence it names are established, or a passage, with or without a label, after which the declared reader cannot say who acts, what they do and what follows (a passage whose actor that reader can infer is not one), in a document or in the exchange case's messages and options | `establish-actors-before-labels`, `explain-what-the-reader-cannot-infer`, `exchange-reaches-the-teaching-rules`, `proof-reads-headings-and-openings-in-order` |
| T2 notation before model | notation, syntax, an identifier or a code introduced before the model it expresses, in a document or in the exchange case's messages and options; in the exchange case, also an identifier, code or file name used as the term a question reasons with rather than as a pointer after the plain statement of what it names | `teach-the-model-before-notation`, `structure-brief-three-items-deepened`, `exchange-reaches-the-teaching-rules` |
| T3 context-less direct entry | a declared direct entry lacks the context or example needed to use it, its link or anchor fails where readers read, or proof does not walk it | `local-context-at-every-declared-entry`, `walk-every-declared-entry`, `read-in-the-declared-environment` |
| T4 negative or subject-less framing | a heading, opening, summary or label names the subject by what it is not when a positive subject exists, or loses an exact restriction when reframed, in a document or in the exchange case's messages and options | `positive-subject-restrictions-recoverable`, `tests-reach-openings-and-labels`, `proof-reads-headings-and-openings-in-order`, `exchange-reaches-the-teaching-rules` |
| T5 motivating case taken as the domain | a motivating source's assumptions, or a list of examples, presented as what the intended domain requires; roles conflated; or a design's choice presented as what the problem requires; in a document or in the exchange case's messages and options | `domain-roles-and-choices-kept-distinct`, `exchange-reaches-the-teaching-rules` |
| T6 misplaced figure | a figure placed away from the question it answers | `place-by-question` |
| T7 distinction taught by assertion | a new distinction stated without comparing cases, stating the principle, or naming where the comparison breaks; an analogy with no stated basis or point of failure | `compare-then-principle-then-limit` |
| T8 example status unclear | an example whose completeness or omissions go unstated, a canonical example left incomplete, or a construct first met with no local excerpt | `examples-labelled-canonical-complete` |
| T9 uneven or incomplete reference | reference entries of one kind with differing fields, identifiers that change when a heading does, or an inventoried item with no entry | `reference-entries-shaped-by-subject`, `coverage-against-the-inventory` |
| T10 naming or status drift | one entity under two names, or one name for two entities, across prose, code labels, captions, figure text and tables, or across the exchange case's messages, options and discussion document; status statements that disagree across the set | `one-name-one-entity`, `tests-reach-openings-and-labels`, `art-owns-figure-text`, `proof-reads-for-whole-document-coherence`, `exchange-reaches-the-teaching-rules` |
| T11 figure the declared reader cannot use | a figure whose question and level, scope, omissions, reading entry or chosen conventions the declared reader cannot identify; information reachable only through the figure; vocabulary that drifts between figures; disagreement with the formal source it is drawn from; or a figure that does not load where readers read | `figure-answers-a-question-at-a-level`, `decompose-where-levels-add-value`, `figure-states-scope-omissions-entry`, `conventions-identifiable-notation-per-reader`, `one-visual-vocabulary`, `text-reaches-figure-information`, `derived-figure-agrees-with-its-source`, `read-in-the-declared-environment` |
| T12 house-style point unapplied | in a set that takes the house style for a point, or in the exchange case's prose, the point applied two ways or against the house style: a task heading in another form, a procedure introduced by a fragment the steps complete, a step whose location, condition or goal follows its action or whose first sentence states no imperative action, lowercase "should" in a normative sentence under a stated BCP 14 convention, an identifier outside code font, an untagged or indented code block, a glossary term unlinked at its first use on a page, an example or aid without its label, stock phrasing, a heading level holding one subsection, or an aid beside normative text that is not visually set apart where readers read | `prose-contract-builds-on-its-base`, `copy-edit-reads-the-prose-contract`, `normative-wording-preserved`, `exchange-prose-follows-the-house-style`, `read-in-the-declared-environment`, and the house style's points (§9) |
| H1 accuracy (regression) | a new technical error or invented observation, in a document or in output the exchange case shows the human | `no-invented-observations`, `draft-owns-structure-and-technical-truth`, `exchange-reaches-the-teaching-rules` |
| H2 normative meaning (regression) | changed force or a dropped restriction, in a document or where the exchange case quotes, proposes or summarises a rule | `normative-text-traces-and-ambiguity-surfaces`, `normative-wording-preserved`, `exchange-reaches-the-teaching-rules` |
| H3 reader effort (regression) | narration or notation teaching the declared reader does not need, or length without teaching gain, in a document or in the exchange case's messages and options | `explain-what-the-reader-cannot-infer`, `exchange-reaches-the-teaching-rules`, `conventions-identifiable-notation-per-reader`, `text-reaches-figure-information` |
| H4 stage boundary (regression) | a stage repairing a class outside its charter, such as copy-edit repairing missing teaching in sentences | `missing-teaching-goes-to-draft`, `whole-document-one-charter` |
| H5 contract precedence (regression) | a house-style point applied where a contract on the walk, or the style guide the walk ends at, settles that point otherwise | `prose-contract-builds-on-its-base` |
| E1–E4 exchange | a trade-off that depends on a term or fact the exchange has not explained and the scripted human has not shown they know; an option before the problem and its cost; options, or a distinction the exchange teaches, compared without the principle that separates them or where the comparison breaks, or an analogy without its basis or where it fails; an agreement point with no restatement asked for and recorded | §7; E3's rule is `compare-then-principle-then-limit` (§3) |
| EH exchange regression | the existing hand-back framing (trade-off, recommendation, evidence) lost, or more human turns without a better decision | `escalation-names-the-tradeoff` |

WU-02 and WU-03 are inputs to that case design, not rules.

**X-17 targets knowledge added to a stage.** Every rule that adds a check to
`draft`, `copy-edit`, `art` or `proof` therefore appears in a class above. The
house style's points add checks without being rules, and T12 is their class.
The teaching rules also add checks to requirements and design exchanges. Their
classes, T1, T2, T4, T5, T10, H1, H2 and H3, therefore score the exchange
case's messages and options too, as T12 already does, and E3 scores a
distinction the exchange teaches as well as its options. The human chose this
over new exchange-only classes on 2026-09-30, so that one defect has one class
wherever it appears. Only a case whose request makes the session introduce a
term, an identifier or a distinction can exhibit T1, T2, T10 and H3. H2, and
T4's lost restriction, also need options that propose or quote a rule with a
condition, T5 a request that starts from a motivating platform or example, and
H1 a session that shows output. If the exchange case cannot exhibit a class,
the preregistration declares it unmeasured for exchanges before any output
exists.

In these classes, the exchange case's messages and options include each
question's header, each option's label and description, and the text of any
view. Its declared reader is the scripted human, who has read what the
exchange explained and knows what the scripted answers show; the
preregistration fixes, per case, which terms those answers show known. A
regression class counts its defect in each arm's transcript, and a regression
is a higher count in the revised arm. Some classes score only part of a rule,
in documents as in the exchange: H2 scores force and restrictions but not
tracing or ambiguity, and H3 scores narration and length while T1 scores part
of what goes unexplained. The other clauses are delivered but unmeasured. The
scripted human's answers are fixed, so the exchange scores record what the
session wrote, never whether a human understood it.
The rules accepted through seam 1 alone add no stage check, and each falls in one
of four categories:

- **Contract and authoring rules:** §1's contract rules, §5's format rules and
  `reader-checkpoints-scheduled-after-proof`. They bind the sessions that write
  a contract. The cases' contracts supply them, and their effect on stage output
  is scored through the classes of the stage rules that read them.
- **Routing and records:** `art-meets-the-figure-contract`, the widened
  `declined-figure-names-one-of-three-reasons`, `proof-claims-no-understanding`
  and `checks-establish-different-properties`. They deliver other rules, or
  limit what a result claims, and close no defect in the document. H4 guards the
  boundaries they draw.
- **Mechanical delivery:** `rendered-figure-ships-with-its-source`.
  `figures:check`, whose controls have been seen to fail, establishes agreement
  between source and SVG wherever a repository runs it. Whether a stage obeys
  the clause elsewhere is reported as unmeasured.
- **Outside the stages:** the reader-checkpoint procedure (§6), which seam 3
  exercises; `sketch-clauses`; and `tool-candidates-are-dated`.

A green seam 1 shows a rule is delivered, not obeyed.

**Seam 3: the human read.** `sample-reading-k11` reads samples from the
revised arm under `reader-test.md`'s procedure. It reports on the samples, not
on individual rules.

## Increments

`guidance-increments-k9` cuts these ahead of `revised-trials-k10`. The expected
slices are confirmed except that format selection moves second, because the
figure and proof slices cite its capability snapshot. [The slicing
view](../design/documentation-guidance/README.md#views) draws the
dependencies.

| # | Increment | Delivers | Depends on |
|---|---|---|---|
| 1 | document contract and `draft` | `DOCUMENT-CONTRACT-FORMAT.md` (every part with its default, the reader-checkpoint part declared but not yet scheduled, and the prose contract's default still the charters' own tests), the spine trigger, the `editorial.md` edge, the `draft` rules with `draft`'s reading of the prose contract's keyword, example-label, normative-aid and layout conventions, and `execute.md`'s teaching section, with the five shared rules and the three that bind `draft` alone, its scope naming `draft`, and `grove-draft`'s sentence naming the section | — |
| 2 | format selection and the capability snapshot | `format-per-set-with-the-human`, `format-declares-media-and-environments`, and `documentation-toolchain.md` | 1 |
| 3 | `copy-edit` and the house style | the `copy-edit` rules; the teaching section's scope widened to `copy-edit`, `positive-subject-restrictions-recoverable` added to it, and `grove-copy-edit`'s sentence naming it; `house-style.md` with Google's attribution; `prose-contract-builds-on-its-base`; and the contract format's prose-contract default changed to name the house style | 1 |
| 4 | figures | `FIGURE-FORMAT.md` with `sketch-clauses`, the `art` rules, the `visual-design-discussion.md` reconciliation (DS-01), and `figures:check` generalised | 1, 2 |
| 5 | `proof` and reader checkpoints | the `proof` rules; reader checkpoints as one slice: `reader-checkpoints-scheduled-after-proof`, `reader-test.md` and its `requirements` condition; and the README's statement of the fatness rule reconciled with conditional kind procedures (§6) | 1, 2 |
| 6 | exchanges | the `execute.md` extension, including its conditions naming `FIGURE-FORMAT.md` for exchange views and `house-style.md` for exchange prose, and `exchange-reaches-the-teaching-rules` with the teaching section's scope widened to requirements and design exchanges | 3, 4 |

**What each boundary makes usable.** No increment activates a workflow whose
procedure a later increment ships.

- After 1, a document declares its contract and `draft` works to the deepened
  precondition, its rules and the teaching section. A declared reader checkpoint is information only;
  nothing schedules it, and no stage result is read as reader understanding.
- After 2, the format part is chosen with the human against the dated
  snapshot.
- After 3, `copy-edit` applies its rules and the teaching section, and a set
  that declares no prose contract takes the house style.
- After 4, `art` and design sessions work to the figure contract.
- After 5, `proof` walks the declared entries. A declared checkpoint is
  scheduled and then held under its procedure, and both arrive together.
- After 6, requirements and design exchanges carry the practices and the
  teaching rules, reach the figure contract, and write to the human in the
  house style.

Each increment ends with its rules delivered on the loaded path of every kind
they bind, rows and near-miss fixtures in the conformance suite, and
`bash scripts/check.sh` green. Every adopted disposition maps to one increment
through its rule. No operative instruction depends on `docs/`, this checkout
or APIAnyware.

## Traceability to the root requirements

| Root requirement | Delivered by |
|---|---|
| Reader's task and expertise per domain and representation, and what not to teach | contract structure brief; `structure-brief-three-items-deepened`; increment 1 |
| Intended domain apart from motivating sources; subject, producer and consumers distinct | contract structure brief; `domain-roles-and-choices-kept-distinct`; increment 1 |
| Preserve the human's words and reported locations | contract authorship clause; `record-locations-leave-unknowns`; increments 1 and 5 |
| Model and task before notation; concrete actors and consequences; no length limit that suppresses teaching | `teach-the-model-before-notation`, `establish-actors-before-labels`, `explain-what-the-reader-cannot-infer` in `execute.md`'s teaching section; the contract's outcome clause; increment 1 |
| Positive subjects with exact restrictions and normative force preserved | `positive-subject-restrictions-recoverable`, `normative-wording-preserved`, `normative-text-traces-and-ambiguity-surfaces`; increments 1 and 3 |
| Page purpose and local context; learning route and direct entries | contract page purposes and entry routes; `local-context-at-every-declared-entry`; `walk-every-declared-entry`; increments 1 and 5 |
| Coverage from inventories of the subject | contract inventory; `coverage-against-the-inventory`; increments 1 and 5 |
| Examples explained for this audience; fragments and complete examples distinct | `explain-what-the-reader-cannot-infer`, `examples-labelled-canonical-complete`; increment 1 |
| Normative text and aids distinguishable | prose-contract convention; `local-context-at-every-declared-entry`; increments 1 and 3 |
| The pipeline ownership table (brief, four stages, reader testing, format) | §1, §2 and §3's tables; the charters, precondition and correction procedure are preserved, and no single documentation skill is created; increments 1–5 |
| Figures: hierarchical decomposition, C4 where apt, renderers as choices, notation/model/renderer distinct; scope, omissions, entry and labels; conventions identifiable; notation teaching per reader | §4 figure contract; increment 4 |
| Tool-rendered source and SVG with version and command and a regeneration check; page-bytes exception | `rendered-figure-ships-with-its-source`; `figures:check`; increment 4 |
| Placement by the question answered; link back to overviews | `place-by-question`, `decompose-where-levels-add-value`; increment 4 |
| Format selection guidance for Markdown, AsciiDoc, DocBook and Typst; recheck dated facts; verify what the reader sees | §5; the recheck clauses of `format-per-set-with-the-human` and `figure-answers-a-question-at-a-level`; `read-in-the-declared-environment`; `checkpoint-states-its-frame-first`; increments 2, 4 and 5 |
| The #9 practices, observable in the exchange; diagrams in requirements and design exchanges follow the figure contract | §7, whose condition routes both kinds' exchange views to the figure contract; `compare-then-principle-then-limit`; `sketch-clauses`; increments 1, 4 and 6 |
| The documentation practice governs requirements and design exchanges, "because it requires understanding" (the human's direction of 2026-09-30) | §7's `exchange-prose-follows-the-house-style` and `exchange-reaches-the-teaching-rules`; §3's teaching section; §7's transfer table; increments 1, 3 and 6 |
| A default house style for sets that name no prose contract (the human's direction of 2026-09-30) | §9; `prose-contract-builds-on-its-base`; `house-style.md`; increment 3 |
| Evidence classes; independence of probes; later refinements kept | `evidence-classes-stay-apart`; `proof-claims-no-understanding`; this spec's dispositions; increment 5 |
| Preregistered comparison, transfer case, regressions, negative results retained | delivered by `guidance-evaluation-k8` and `revised-trials-k10` from seam 2 above |
| Human read before the workstream closes | delivered by `sample-reading-k11` under `reader-test.md` |
| Evidence survives outside the tree; skills followable without it | `docs/research/`, this spec, the ADRs; `documentation-toolchain.md` carries the operative facts; each increment's done-when |
| Repository checks; Taskfile for reusable generation and validation; generated diagrams inspected | seam 1; `figures:check`; this spec's companion inspected in the design viewer |

## Dispositions

Every proposal ID in the inventory gets exactly one disposition. **Adopt** means
the proposal becomes the named rule or rules. **Narrow** adopts a reduced
form, and the reason says what was cut and why. **Decline** adopts nothing, for
the stated reason. *Rule* names a rule slug from the tables above. `—` means
none, and a parenthesised phrase names a non-skill owner. *Seam* uses the codes
from [Test seams](#test-seams). An ID that is the same proposal as another says
so and inherits that ID's disposition.

### X — cross-stage principles

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| X-01 | Adopt | (this placement) | S1 assertion 2 | Every rule has a stage, format or reference owner; no documentation skill exists. |
| X-02 | Adopt | (realised by the DR, CE, AR and PR rules) | S2 | A goal with no rule of its own. |
| X-03 | Narrow | `figure-answers-a-question-at-a-level`, `decompose-where-levels-add-value`, `rendered-figure-ships-with-its-source` | S2:T11 | Not every renderer or level in every document; page-bytes exception; notation teaching per reader (AR-17). |
| X-04 | Adopt | §7 rules, `compare-then-principle-then-limit`, `sketch-clauses` | S1, S2:E, T1, T2, T4, T5, T10, H1, H2, H3, T12 | The human's direction of 2026-09-30 carries the documentation practice into exchanges: the house style and the teaching section, with the transfer table in §7 saying which teaching rules reach one. |
| X-05 | Adopt | `contract-structure-brief-fields`, `structure-brief-three-items-deepened` | S2:T2 | Addressing a qualified reader while letting others recognise it is carried by stating expertise and exclusions. |
| X-06 | Adopt | `reader-checkpoints-scheduled-after-proof` | S1 | The rationale for scheduling reader evidence. |
| X-07 | Adopt | `contract-prose-contract-fields`, `copy-edit-reads-the-prose-contract`, `prose-contract-builds-on-its-base` | S1, S2:H5 | The house style is itself a default whose every point a declared contract may take an exception to. |
| X-08 | Adopt | `contract-outcome-is-find-understand-use` | S1 | Outcomes are find, understand, use; no readability gate. |
| X-09 | Adopt | `contract-structure-brief-fields`, `local-context-at-every-declared-entry` | S2:T3 | |
| X-10 | Adopt | `teach-the-model-before-notation`, `establish-actors-before-labels`, `compare-then-principle-then-limit` | S2:T1, T2, T7 | |
| X-11 | Adopt | `contract-structure-brief-fields`, `contract-structure-decided-by-the-human` | S1 | The partition row, realised by the contract. |
| X-12 | Adopt | §3 `draft` rules and the teaching section `draft` applies | S1 | |
| X-13 | Adopt | §3 `copy-edit` rules and the teaching section `copy-edit` applies | S1 | |
| X-14 | Adopt | §4, `art-meets-the-figure-contract` | S1 | |
| X-15 | Narrow | §3 `proof` rules | S1 | The cold re-read is declined (PR-07); the C4 checklist becomes the figure contract (PR-04). |
| X-16 | Narrow | §6 | S1 | Counted criticisms become the independent-reader validity signal (PR-12). |
| X-17 | Narrow | (this spec's seam 2; `preregistration-k12`) | S2 | A rule for developing Grove's own skills; it binds no session elsewhere, so it is not skill text. Every rule that adds a check to a stage has a class. Rules that add no stage check are accepted through seam 1 alone, in the four categories [Test seams](#test-seams) names with their reasons. |
| X-18 | Adopt | `evidence-classes-stay-apart`; (the evidence record's class column) | S1 | |
| X-19 | Adopt | (increment done-when) | S1 check | No ISO 24495-1 text in skill bytes; cite the IPLF definition. |
| X-20 | Adopt | (increment 4 done-when) | S1 check | Attribution comment and `LICENSES` entry. |
| X-21 | Decline | — | — | Acquiring books is not guidance. No adopted rule rests on an unread chapter beyond the depths the evidence record states. |
| X-22 | Adopt | `establish-actors-before-labels`, `art-owns-figure-text`, `missing-teaching-goes-to-draft` | S2:T1, T10, H4 | Each stage takes its part of the defect: `draft` teaches, `copy-edit` repairs the wording under the same rule and sends missing teaching back. |
| X-23 | Adopt | `contract-structure-brief-fields`, `local-context-at-every-declared-entry`, `walk-every-declared-entry` | S2:T3 | |
| X-24 | Adopt | `contract-prose-contract-fields`, `read-in-the-declared-environment`; house style | S1, S2:T12 | The prose contract names a convention and `draft` applies it. The visual form depends on the format, so the house style states it per format: a labelled blockquote in Markdown, closed by a blank line, and the native equivalent elsewhere, as the human directed at agreement. `proof` checks that it is set apart where readers read. |
| X-25 | Adopt | `contract-structure-brief-fields`, `contract-structure-decided-by-the-human`, `record-locations-leave-unknowns` | S1 | |
| X-26 | Adopt | as DR-46 | | Same proposal. |
| X-27 | Adopt | `explain-what-the-reader-cannot-infer`; house style | S2:H3, T12 | The house style's stock-phrasing point extends it to phrasing that adds no meaning. |
| X-28 | Adopt | as AR-19 | | Same proposal. |
| X-29 | Adopt | `walk-every-declared-entry`, `read-in-the-declared-environment`, `proof-claims-no-understanding`, `record-locations-leave-unknowns` | S2:T3, T11 | |
| X-30 | Adopt | (the evidence record and this table) | — | A practice for this workstream's records: each narrowing carries its reason here. |
| X-31 | Adopt | as DR-47 | | Same proposal. |
| X-32 | Adopt | as DR-48 | | Same proposal. |
| X-33 | Adopt | `acceptance-is-not-a-rewrite-order` | S1 | |

### DR — structure brief and `draft`

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| DR-01 | Narrow | `contract-structure-brief-fields` | S1 | One purpose per page and one primary reader per document are adopted. The Diátaxis compass is optional: no peer-reviewed evaluation exists (survey §10). |
| DR-02 | Adopt | `contract-structure-brief-fields`, `structure-brief-three-items-deepened` | S2:T2 | |
| DR-03 | Narrow | `contract-structure-brief-fields` | S1 | Concepts in dependency order, each introduced before use. The per-concept failure column is optional. |
| DR-04 | Adopt | `contract-structure-brief-fields` | S1 | |
| DR-05 | Narrow | `contract-structure-brief-fields` | S1 | Order follows the reader's questions and tasks. Question-form headings are a prose-contract choice. |
| DR-06 | Adopt | `contract-structure-brief-fields`, `coverage-against-the-inventory` | S2:T9 | |
| DR-07 | Narrow | `contract-structure-brief-fields` | S1 | Needs are stated by question and level. No overview is mandatory (AR-12), direct entries count (X-23), and the renderer is chosen within the admitted media. |
| DR-08 | Adopt | `contract-prose-contract-fields`, `contract-declared-by-path`, `prose-contract-builds-on-its-base`; house style | S1, S2:T12 | The house style supplies each field a set leaves undeclared. |
| DR-09 | Adopt | `format-declares-media-and-environments` | S1 | |
| DR-10 | Narrow | `domain-roles-and-choices-kept-distinct` | S2:T5 | Costs and alternatives stay visible where a choice is explained. A fixed proposal-template section list is a section-plan choice. |
| DR-11 | Adopt | `domain-roles-and-choices-kept-distinct` | S2:T5 | |
| DR-12 | Adopt | `contract-structure-brief-fields`, `local-context-at-every-declared-entry` | S2:T3 | |
| DR-13 | Adopt | `local-context-at-every-declared-entry` | S2:T3 | |
| DR-14 | Decline | — | — | Short parts with checks is a tutorial design choice, and a structure brief may plan it. There is no evidence that a stage rule improves a stage. |
| DR-15 | Adopt | `draft-owns-structure-and-technical-truth` (held) | S2:H1 | Already the draft's structure charter. |
| DR-16 | Adopt | `normative-text-traces-and-ambiguity-surfaces` | S2:H2 | |
| DR-17 | Adopt | `examples-labelled-canonical-complete` | S2:T8 | |
| DR-18 | Narrow | `compare-then-principle-then-limit` | S2:T7 | The comparison is adopted. The count of two per concept is dropped: redundancy harms expert readers (DR-31). |
| DR-19 | Adopt | `compare-then-principle-then-limit` | S2:T7 | |
| DR-20 | Narrow | `local-context-at-every-declared-entry` | S2:T3 | Marking an example's authority is kept, as the normative and aid distinction. The size preference is a style default. |
| DR-21 | Narrow | `examples-labelled-canonical-complete` | S2:T8 | Labels and stated omissions are kept. Feature-maturity marks are a prose-contract choice. |
| DR-22 | Narrow | `normative-text-traces-and-ambiguity-surfaces` | S2:H2 | A shown failure names its rule. Pairing every positive example is a policy the brief may set. |
| DR-23 | Adopt | `no-invented-observations` | S2:H1 | |
| DR-24 | Narrow | `examples-labelled-canonical-complete` | S2:T8 | Complete examples are kept. Whether the reader judges authoring burden is the brief's reader task. |
| DR-25 | Adopt | `reference-entries-shaped-by-subject` | S2:T9 | |
| DR-26 | Narrow | `reference-entries-shaped-by-subject` | S2:T9 | Stable identifiers are adopted. "Used by" and "see also" links are optional entry aids. |
| DR-27 | Adopt (resolution) | `teach-the-model-before-notation`, `explain-what-the-reader-cannot-infer` | S2:T2, H3 | Tension resolved as: the model is taught first; inside a route, give the reason and a link, at the length the concept needs. The "one line" part is removed by DR-34. |
| DR-28 | Adopt (resolution) | `contract-structure-brief-fields`, `local-context-at-every-declared-entry` | S2:T3 | Tension resolved as: pure purpose per page, navigation by route and subject, and local context at every entry. |
| DR-29 | Adopt (resolution) | `contract-structure-brief-fields` | S1 | Tension resolved as: linear learning route, page-one reference entries. |
| DR-30 | Adopt (resolution) | `examples-labelled-canonical-complete` | S2:T8 | Tension resolved as: canonical examples complete; gaps only as labelled exercises. |
| DR-31 | Adopt (resolution) | `explain-what-the-reader-cannot-infer` | S2:H3 | Tension resolved as: repeat what is new to this reader, never what they know. |
| DR-32 | Adopt (resolution) | `contract-structure-brief-fields`, `reference-entries-shaped-by-subject` | S2:T9 | Tension resolved as: purpose sets the page's job, and the subject sets an entry's shape. |
| DR-33 | Adopt | `establish-actors-before-labels` | S2:T1 | |
| DR-34 | Adopt | `explain-what-the-reader-cannot-infer`; `contract-outcome-is-find-understand-use` | S2:H3 | Length limits yield to needed explanation. |
| DR-35 | Adopt | `establish-actors-before-labels` (content), `positive-subject-restrictions-recoverable` (wording) | S2:T1, T4 | Split by the ownership table. |
| DR-36 | Adopt | `tests-reach-openings-and-labels` | S2:T4, T10 | Openings, summaries and tables are `copy-edit`'s wording scope. |
| DR-37 | Adopt | `examples-labelled-canonical-complete` | S2:T8 | |
| DR-38 | Adopt | `teach-the-model-before-notation` | S2:T2 | Generalised from AIL's model. |
| DR-39 | Narrow | `explain-what-the-reader-cannot-infer` | S2:H3 | Narrowed by DR-46: no token narration. |
| DR-40 | Adopt | `establish-actors-before-labels` | S2:T1 | |
| DR-41 | Narrow | `compare-then-principle-then-limit` | S2:T7 | Generalised to showing a distinction through uses. |
| DR-42 | Adopt | `examples-labelled-canonical-complete` | S2:T8 | |
| DR-43 | Adopt | `local-context-at-every-declared-entry` | S2:T3 | |
| DR-44 | Adopt | `teach-the-model-before-notation`; `contract-structure-brief-fields` | S2:T2 | |
| DR-45 | Adopt | `contract-structure-brief-fields`, `structure-brief-three-items-deepened` | S2:T2 | |
| DR-46 | Adopt | `explain-what-the-reader-cannot-infer` | S2:H3 | |
| DR-47 | Adopt | `contract-structure-brief-fields`, `domain-roles-and-choices-kept-distinct` | S2:T5 | |
| DR-48 | Adopt | `domain-roles-and-choices-kept-distinct` | S2:T5 | |
| DR-49 | Narrow | `domain-roles-and-choices-kept-distinct` | S2:T5 | Generalised: a list implies no property it does not establish. |

### CE — `copy-edit`

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| CE-01 | Narrow | `one-name-one-entity`; existing glossary tests; house style | S2:T10, T12 | Spelling and glossary linking already hold. Collisions are flagged against the declared term risks. Linking every first use is a prose-contract choice, and the house style takes it per page. |
| CE-02 | Narrow | `contract-prose-contract-fields`; `normative-wording-preserved`; house style | S2:H2, T12 | Conditions before actions binds only through a prose contract, never over normative sentences. The house style takes it for procedure steps. |
| CE-03 | Narrow | `contract-prose-contract-fields`; house style | S2:T12 | A prose-contract point, and the house style takes it. |
| CE-04 | Narrow | `contract-prose-contract-fields`; house style | S2:T12 | As CE-03. |
| CE-05 | Narrow | `contract-prose-contract-fields`; `establish-actors-before-labels`; house style | S2:T1, T12 | Voice is a prose-contract point with its exceptions, and the house style takes it. The actor test is adopted. |
| CE-06 | Narrow | `establish-actors-before-labels` | S2:T1 | Characters as subjects, as a test of what the reader can recover. The positional heuristics are optional. |
| CE-07 | Narrow | `contract-prose-contract-fields`; house style | S2:T12 | Front-loading and glossed link text are prose-contract choices. The house style takes front-loading, and its evidence breaks the CE-12 tie. |
| CE-08 | Adopt | `contract-prose-contract-fields`; `normative-wording-preserved`; house style | S2:H2, T12 | The house style's keyword convention adds BCP 14's defined reading of the capitals, which Google does not address. It removes the ambiguity Google cites against "should", and it keeps lowercase "should" out of normative sentences. |
| CE-09 | Narrow | `contract-prose-contract-fields`; house style | S2:T12 | Code typography is declared per set, since the guides disagree on fences. The house style takes tagged fences, departing from Google's indented blocks. |
| CE-10 | Adopt | existing `copy-edit-operative-tests`; `compare-then-principle-then-limit` | S2:T7 | The figure-of-speech test already holds; the analogy's basis and limit go to `draft`. |
| CE-11 | Adopt | `contract-outcome-is-find-understand-use` | S1 | |
| CE-12 | Adopt (resolution) | `contract-prose-contract-fields`, `prose-contract-builds-on-its-base`; house style | S2:T12, H5 | Conflict resolved: the set's prose contract decides. Where it does not, the house style takes Google's bare infinitive, with CE-07 as the tie-break (§9). Quotations verified 2026-09-30. |
| CE-13 | Adopt (resolution) | `contract-prose-contract-fields`, `prose-contract-builds-on-its-base`; house style | S2:T12, H5 | As CE-12, taking Google's complete introductory sentence for consistency with the base alone (§9). |
| CE-14 | Adopt (resolution) | `contract-prose-contract-fields`, `prose-contract-builds-on-its-base`; house style | S2:T12, H5 | As CE-12, taking Google's written rather than spoken register (§9). |
| CE-15 | Adopt (resolution) | `contract-outcome-is-find-understand-use`, `explain-what-the-reader-cannot-infer`; house style | S2:H3 | Conflict resolved: length follows the reader's need; there is no brevity target, and the house style sets none. |
| CE-16 | Adopt | `contract-prose-contract-fields`, `copy-edit-reads-the-prose-contract`, `prose-contract-builds-on-its-base` | S1, S2:H5 | A named contract still governs; the house style applies only to what no contract on the walk, and no named guide, settles. |
| CE-17 | Adopt | `establish-actors-before-labels` | S2:T1 | The passage test is its own sentence in the merged rule and applies whether or not a label appears. |
| CE-18 | Adopt | `positive-subject-restrictions-recoverable` | S2:T4 | |
| CE-19 | Adopt | `tests-reach-openings-and-labels`; `evidence-classes-stay-apart` | S2:T4, T10 | The limit (a model paraphrase is not understanding) is carried by the evidence rule. |
| CE-20 | Adopt | `missing-teaching-goes-to-draft` | S2:H4 | |
| CE-21 | Adopt | `establish-actors-before-labels` | S2:T1 | |
| CE-22 | Narrow | `one-name-one-entity`, `explain-what-the-reader-cannot-infer` | S2:T10, H3 | Narrowed by X-27 and DR-46: explain meaning, not familiar tokens. |
| CE-23 | Adopt | `normative-wording-preserved` | S2:H2 | |

### AR — `art`

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| AR-01 | Narrow | `decompose-where-levels-add-value` | S2:T11 | Narrowed by AR-12: link back instead of reusing the overview on every page. |
| AR-02 | Adopt | `figure-states-scope-omissions-entry` | S2:T11 | |
| AR-03 | Adopt | `figure-answers-a-question-at-a-level`, `decompose-where-levels-add-value` | S2:T11 | |
| AR-04 | Narrow | `figure-states-scope-omissions-entry`, `conventions-identifiable-notation-per-reader` | S2:T11, H3 | The key identifies conventions, and notation teaching depends on the reader (AR-17). Unverified wording ("colour-blind-safe") is replaced by SV-02. A verb that many arrows share may be carried by a line style the key names: this spec's own contract view put a label on each of twelve read edges, and the labels collided in every layout tried. |
| AR-05 | Adopt | `one-visual-vocabulary` | S2:T11 | |
| AR-06 | Narrow | `text-reaches-figure-information` | S2:T11, H3 | Text access is kept. The how-to-read line and punctuation narration become reader-conditional (AR-17, AR-19). |
| AR-07 | Narrow | `figure-states-scope-omissions-entry` | S2:T11 | Labels on elements are kept. The unverified effect size is not cited. |
| AR-08 | Adopt | `figure-states-scope-omissions-entry` | S2:T11 | The design viewer's "Start here" styling stays in `visual-design-discussion.md` as its convention. |
| AR-09 | Adopt | `one-visual-vocabulary` | S2:T11 | |
| AR-10 | Adopt | `rendered-figure-ships-with-its-source` | S1 | |
| AR-11 | Adopt | `rendered-figure-ships-with-its-source`; `figures:check` | S1 | |
| AR-12 | Adopt | `decompose-where-levels-add-value`, `place-by-question` | S2:T6, T11 | |
| AR-13 | Adopt | `art-owns-figure-text` | S2:T10 | |
| AR-14 | Adopt | `checks-establish-different-properties` | S1 | |
| AR-15 | Adopt | `figure-answers-a-question-at-a-level`; `one-name-one-entity` | S2:T10, T11 | |
| AR-16 | Narrow | `derived-figure-agrees-with-its-source`, `conventions-identifiable-notation-per-reader` | S2:T11, H3 | Generalised from railroads to any figure derived from a formal source. Reading keys depend on the reader. |
| AR-17 | Adopt | `conventions-identifiable-notation-per-reader` | S2:T11, H3 | |
| AR-18 | Adopt | `conventions-identifiable-notation-per-reader`, `figure-answers-a-question-at-a-level` | S2:T11, H3 | |
| AR-19 | Adopt | `place-by-question`, `text-reaches-figure-information` | S2:T6, T11, H3 | |

### PR — `proof` and reader testing

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| PR-01 | Adopt | `walk-every-declared-entry` | S2:T3 | |
| PR-02 | Adopt | `coverage-against-the-inventory` | S2:T9 | |
| PR-03 | Narrow | `read-in-the-declared-environment`; the repository gate | S2:T3, T11 | Mechanical checks run through the repository's gate; `proof` confirms anchors resolve where readers read. |
| PR-04 | Narrow | `proof-reads-for-whole-document-coherence`; figure contract | S2:T10 | The figure contract replaces the C4 checklist; `proof` checks cross-figure consistency. |
| PR-05 | Adopt | `proof-reads-for-whole-document-coherence` | S2:T10 | Status statements added to the held rule. |
| PR-06 | Adopt | `proof-reads-headings-and-openings-in-order` | S2:T1, T4 | The headings-and-openings read becomes `proof`'s read in document order. |
| PR-07 | Decline | — | — | Every stage is already a fresh-context read. A time-gap re-read is untested rhetoric (the survey labels it "argument, not tested"). |
| PR-08 | Adopt | `reader-checkpoints-scheduled-after-proof` | S1 | |
| PR-09 | Adopt | `observe-outcomes-not-liking` | S1 | |
| PR-10 | Narrow | `observe-outcomes-not-liking` | S1 | Methods kept. Round sizes and counts are guideline numbers, not thresholds. |
| PR-11 | Narrow | `observe-outcomes-not-liking`, `corrections-route-to-owners` | S1 | Methods kept; the expected second round is the follow-up read. |
| PR-12 | Narrow | `evidence-classes-stay-apart` | S1 | Agreement across independent readers is the signal when there is more than one reader. |
| PR-13 | Decline | — | — | A document design choice (see DR-14), not a reader-test rule. |
| PR-14 | Adopt | `checkpoint-states-its-frame-first`, `observe-outcomes-not-liking` | S1 | |
| PR-15 | Adopt | `evidence-classes-stay-apart`, `proof-claims-no-understanding` | S1 | |
| PR-16 | Adopt | `evidence-classes-stay-apart` | S1 | |
| PR-17 | Adopt | `early-failure-stops-the-read` | S1 | |
| PR-18 | Adopt | `corrections-route-to-owners` | S1 | |
| PR-19 | Adopt | `corrections-route-to-owners` | S1 | |
| PR-20 | Adopt | `proof-reads-headings-and-openings-in-order` | S2:T1, T4 | `proof` does not load `copy-edit`'s rules, so its own rule carries the request. Human evidence stays separate (`proof-claims-no-understanding`). |
| PR-21 | Adopt | `record-locations-leave-unknowns` | S1 | |
| PR-22 | Adopt | `walk-every-declared-entry` | S2:T3 | |
| PR-23 | Adopt | `corrections-route-to-owners` | S1 | |
| PR-24 | Adopt | `checks-establish-different-properties`, `read-in-the-declared-environment` | S2:T3, T11 | |
| PR-25 | Adopt | `checkpoint-states-its-frame-first`; snapshot viewer facts | S1 | The VS Code step is a dated fact carried with its date. |
| PR-26 | Adopt | `read-in-the-declared-environment`, `checkpoint-states-its-frame-first` | S2:T3, T11 | |
| PR-27 | Adopt | `record-locations-leave-unknowns`, `checkpoint-states-its-frame-first` | S1 | |
| PR-28 | Adopt | `evidence-classes-stay-apart` | S1 | |
| PR-29 | Adopt | `evidence-classes-stay-apart` | S1 | |

### FM — document format

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| FM-01 | Adopt | `format-per-set-with-the-human`, `contract-declared-by-path` | S1 | |
| FM-02 | Narrow | `format-per-set-with-the-human`; snapshot | S1 | The facts behind it are carried; the verdict was one case's and is not a default. |
| FM-03 | Adopt | `format-declares-media-and-environments` | S1 | |
| FM-04 | Adopt | `declined-figure-names-one-of-three-reasons` (widened) | S1 | |
| FM-05 | Adopt | `read-in-the-declared-environment` | S2:T3, T11 | |
| FM-06 | Adopt | `rendered-figure-ships-with-its-source`, `format-declares-media-and-environments` | S1 | Figures and other generated outputs alike. |

### CV — requirements and design conversations

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| CV-01 | Narrow | `vocabulary-before-trade-off` | S2:E1 | Applies where a trade-off depends on a term or fact the exchange has not explained and the human has not shown they know, not before every question. What they stated, reasoned with correctly or confirmed is known; a term they only used or asked about is not, and a doubt they voice outweighs any earlier evidence. |
| CV-02 | Adopt | `vocabulary-before-trade-off` | S2:E1 | |
| CV-03 | Adopt | `compare-then-principle-then-limit` | S2:E3 | The same practice as DR-19's, so one rule states both. It governs options and taught distinctions in an exchange. |
| CV-04 | Adopt | `problem-before-proposal` | S2:E2 | |
| CV-05 | Narrow | `restate-agreement-point-decisions` | S2:E4 | Restated at agreement points, where the stakes are the spec or the ADR bar, not at every consequential answer. |
| CV-06 | Adopt | `sketch-clauses` | S1 | The practice binds both exchange kinds, so it lives in the figure contract. |

### DS, WU and SV

| ID | Disposition | Rule | Seam | Reason |
|---|---|---|---|---|
| DS-01 | Adopt | `tool-candidates-are-dated`; snapshot | S1 | Current facts from the 2026-09-30 recheck. |
| WU-01 | Adopt | `contract-structure-brief-fields` | S2:T2 | |
| WU-02 | Adopt | (`preregistration-k12` case design) | S2 | An evaluation input, not skill text. |
| WU-03 | Adopt | (`preregistration-k12` scoring) | S2 | As WU-02. |
| SV-01 | Adopt | `grove-planning` working increments (held) | S1 | Already the planning rule. |
| SV-02 | Narrow | `conventions-identifiable-notation-per-reader` | S2:T11, H3 | Colour never alone is kept. Merged table cells cannot occur in GFM; where a format allows them, their use is a prose-contract choice. |
| SV-03 | Adopt | `corrections-route-to-owners` | S1 | The reader's words travel with the correction. |
| SV-04 | Narrow | `local-context-at-every-declared-entry` | S2:T3 | Never hide what judging a rule needs behind a link alone. The two-level cap is dropped (observational evidence only). |
| SV-05 | Adopt | `local-context-at-every-declared-entry`; `contract-prose-contract-fields`; house style | S2:T3, T12 | |
| SV-06 | Narrow | `normative-text-traces-and-ambiguity-surfaces` | S2:H2 | Never soften normative force. An open-question register is a section-plan choice. |
| SV-07 | Narrow | `observe-outcomes-not-liking` | S1 | A modification task is one possible reader task. |
| SV-08 | Decline | — | — | A genre template default with no stage evidence. |
| SV-09 | Narrow | `contract-structure-brief-fields` | S1 | Indexes are entries a contract may declare; none is mandatory. |
| SV-10 | Narrow | `conventions-identifiable-notation-per-reader` | S2:T11, H3 | Only for readers who do not know the notation (AR-17). |
| SV-11 | Narrow | `format-per-set-with-the-human` | S1 | Raw-source readability is a selection criterion, not a universal rule. |

## Agreement

On 2026-09-30 the human agreed this design's placement, contracts and
increments. They agreed eight trade-offs, each put separately with a
recommendation and its evidence:

1. the contract format file;
2. the figure format file;
3. the full delivery clause for preserved design views;
4. one dated capability snapshot;
5. three deepened precondition items;
6. reader checkpoints as `requirements` leaves;
7. restatement at agreement points;
8. the six increments and three ADRs.

That agreement covered the version before review. A design review then found
seven defects, and every one was repaired: requirements exchanges reach the
figure contract; stage checks are measured; the snapshot states no rule;
`proof` keeps every class; the conditional reader procedure is an explicit
exception; checkpoint scheduling ships with its procedure; and the contract
view draws every reader. Two further changes came from applying them: `proof`'s
own headings-and-openings rule, and an arrow's verb carried by a keyed line
style. On X-17 the human chose, from three options, to group the stage checks
into measured classes (T7–T11).

On 2026-09-30 the human agreed the version that survived review: "yes". Their
restatement, in their own words: "Each rule belongs in one location. The passes
communicate via the document contract, I'm not sure how the increments and
measurements *work* - but that may be that your question is too obvious."
After the increments and the three seams were explained, they confirmed that
part: "Matches".

They also directed that Grove ship a default house style, which §9 designs,
and noted that iterative refinement should eventually hold for every part of
Grove, not only the editorial send-back. The answers are in the design and
review-integration leaves' running logs.

**The house style (§9).** On 2026-09-30 the human chose, from options each put
with a recommendation and its evidence:

- a closed list following Google, rather than a whole guide or Microsoft as the
  base;
- precedence in which a declared contract fills its gaps from its named base,
  rather than replacing the house style wholly;
- the correction to §2 in which `draft` reads the prose contract's keyword,
  example-label and normative-aid conventions.

They rejected a separate ADR, so the decision is part of
[`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md).
They then directed that every exchange with the human take the features of good
documentation. They chose to split that direction: exchange prose follows the
house style here, and `exchange-teaching-k19` designs the teaching rules'
reach.

The human agreed: "I agree with the house-style design". Their restatement, in
their own words: "The house style design is optimized for understandability and
non-LLM-style prose. Visual it includes simple layour, with non-normative text
in reference/definition documents visually separated from normative content. I
think of house style as a mix of literary constraints and visual design
constraints." The restatement differed from the design as then written in
three places: no stock-phrasing point, no layout point, and a label where it
expected visual separation. Shown the difference, the human added all three
points, and the design above now matches the restatement.

**The house-style review (§9).** A design review then found six defects in
the house style, and each was repaired. A contract naming a style guide left
required fields undefined. `draft` was not told to read the layout point it
writes. Seam 1 could pass with a point missing. A cycle of bases never ended.
The steps point asked for two incompatible word orders. And the aid's
blockquote claimed a visual boundary that its syntax does not guarantee.
Checking the repaired precedence rule also showed that Google settles notices,
so the aid label now follows Google's "Note:".

On 2026-09-30 the human chose, from options each put with a recommendation
and its evidence, that a required field a named guide is silent on falls
through to the house style. Asked to agree the surviving version, they
answered "Agree". Their restatement, in their own words: "google takes
priority, but where there is any ambiguity, that is resolved by reference to
the house style". It reached further than the design, which let a guide
replace the house style beneath it apart from the required fields. Shown the
difference, the human chose to keep every house-style point that a guide does
not settle otherwise, taking each point whole. They agreed the version above:
"Yes, agreed".

**The teaching rules in exchanges (§3, §7).** On 2026-09-30 the human chose,
from options each put with a recommendation and its evidence, two things:

- the five shared teaching rules live in `execute.md`'s teaching section, as
  the placement function computes, rather than in a new file filed by topic;
- the exchange case is scored with the defect classes these rules already
  have, rather than with new exchange-only classes.

They agreed the design: "yes". Their restatement, in their own words: "a)
rules that ensure knowledge monotonicity, ensure consistency reach an
exchange. b) they live in one file - execute.md. Not sure why it has that
name. c) Using ids like T1... H3, make it difficult to reason about this."
Parts (a) and (b) match the design. Part (a) names four of the five rules and
leaves out not repeating what the reader knows. Part (c) showed that the
measurement had not come across while class IDs named it. Explained again
without them, it matched: "Matches". The human then chose that, in an
exchange, a question reasons with plain names and an identifier follows only
as a pointer, and §7 now states that.

**The teaching-rules review (§3, §7).** A design review then found four
defects, and each was repaired. Using a term had counted as knowing it, though
the human had used "increments and measurements" while saying they did not
know how they work. A merge had tied the passage test for who acts to new
labels. And the transfer table kept out rules whose forms an exchange can
contain, on two counts. On 2026-09-30 the human chose, from options each put
with a recommendation and its evidence, to carry all four such rules into
exchanges, growing the teaching section to nine rules, rather than one or none.
An adversarial read of the result then tightened each rule's exchange form,
what counts as known, and the classes' reach.

The human agreed the version that survived: "yes". Their restatement, in their
own words: "Nine rules will be in the teaching section for the main execution
step. Those rules will apply what we have learnt from the documentation
process research to interactive explanations for requirements and design
phases. We aren't writing documents in those phases, so rules about document
structure etc are not included in the teaching section. The measument scores
don't relate to understanding, which requires human evaluation." It matches
the design on the section's place and size, its reach into exchanges, the test
that keeps document-structure rules out, and the measurement's limit. It does
not mention that the nine rules also bind `draft` and `copy-edit`, or what now
counts as known; neither omission contradicts the design.

## Out of scope

- **A `reader-test` kind.** Rejected in
  [`a-reader-checkpoint-is-a-requirements-leaf`](../adr/a-reader-checkpoint-is-a-requirements-leaf.md).
- **One documentation skill, or restating rules per kind.** Rejected in
  [`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md).
- **A whole style guide as the default, or a project-level lookup for a
  project's house style.** Rejected in
  [`a-document-declares-its-contract`](../adr/a-document-declares-its-contract.md): the house
  style is a closed list, and a project's house style is a prose-contract file
  its sets name.
- **A recommended format.** The skills carry criteria and dated facts. The
  choice stays with the human per set.
- **Migrating existing documents,** including earlier design companions whose
  preserved views are Mermaid source only. Each changes when its own work
  touches it.
- **Changing the four-stage pipeline, the correction procedure or the pilot's
  record.**
- **The teaching rules in every kind's hand-back.** The human's reason, that
  "all interaction *needs* the features of good documentation", would reach any
  kind that hands a question back. Their direction named requirements and
  design, and so does this design. Widening it later changes the teaching
  section's scope sentence without moving a rule, and needs an exchange case
  for each kind it adds.

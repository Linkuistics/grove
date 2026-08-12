# guides-k24

## Goal

Classify **four whole files** (15,667 bytes together):

| file | bytes |
|---|---|
| `content/grilling.md` | 4,735 |
| `content/SPEC-FORMAT.md` | 5,364 |
| `content/CONTEXT-FORMAT.md` | 3,502 |
| `content/ADR-FORMAT.md` | 2,066 |

This is batch 4 of 12. It is one closure, not four files that happen to be small:
**`grilling.md` is the entry point, and the other three hang off its move
sections.**

## Context

Read the node brief's *The batching contract* first.

### Region and residual

All four files are carved **whole**. The seed ids `grilling`, `spec-format`,
`context-format` and `adr-format` are consumed and **no residual is minted**.

### The rooting, which is the whole reason these four are one batch

Three of these files carry **no condition of their own** — they are procedures
entered from elsewhere — so this batch must reach back to roots earlier batches
carved. `kinds-k22` carved them; you write the edges. **You own inventory rows
1–8**, which is more than any other batch:

| row | source (already carved, or in this batch) | target |
|---|---|---|
| 1 | `TASK-FORMAT.md`'s `requirements` bullet | `grilling.md` bodies |
| 4 | `TASK-FORMAT.md`'s *three design kinds* `requirements` bullet (L478–480) | `grilling.md`, `CONTEXT-FORMAT.md` bodies |
| 2 | `TASK-FORMAT.md`'s `design` bullet (L481–484) | `ADR-FORMAT.md` bodies |
| 3 | the same `design` bullet | `SPEC-FORMAT.md` §*current-state* / membership / grain |
| 5 | `grilling.md` §*Offer ADRs sparingly* | `ADR-FORMAT.md` bodies |
| 6 | `grilling.md` §*Update CONTEXT.md inline* | `CONTEXT-FORMAT.md` bodies |
| 7 | `grilling.md` §*Agree the test seams* | `SPEC-FORMAT.md` bodies |
| 8 | `SPEC-FORMAT.md`'s own opening | `SPEC-FORMAT.md` §*current-state* / membership / grain |

Row 1 is the edge the whole batch stands on: without it, every procedural unit you
carve out of `grilling.md` fails reachability. Rows 5–7 are
procedural→procedural chains, which are legal and exactly what *deferral may chain*
is for. Row 8 is how `SPEC-FORMAT.md` roots itself — its opening carries a real
condition (*a spec is written "when the increment is a genuine agreement point", and
most increments write none*).

Run the sweep for all four filenames before you finish, as **evidence** alongside
the inventory:

```
grep -rn 'grilling\.md\|SPEC-FORMAT\.md\|CONTEXT-FORMAT\.md\|ADR-FORMAT\.md' content/
```

**Hits inside `SKILL.md` are not yours and you write nothing for them.** They sit
inside `pending-skill-*` units, and **no edge may have a `pending-*` source** — that
was the lossy ledger `batches-k33` F2 killed. `SKILL.md`'s own ADR and spec
paragraph is inventory rows 20–21, owned by `execute-k29`; its glossary paragraph is
row 29, owned by `finish-cycle-k32`. Report those hits as *not yours* rather than as
missed, and do not park a `defers=` anywhere.

Hits inside `SKILL.md` `## Reference files` are a **standing sweep exclusion** for
every batch: that index names all eight guides and none of its rows is a
trigger→body edge.

### The pre-decided calls in this region

The node brief settles these; apply them rather than re-deciding.

- **Family E bodies — `grilling.md` §*Offer ADRs sparingly* and `ADR-FORMAT.md`'s
  head.** Both **procedural**. The head is a *redirect* to
  `linkuistics:decision-records`, not a condition — which is what your own notes
  below already say, and it is now settled rather than argued.
- **Family C body — `ADR-FORMAT.md` §*Why the set stays minimal*.** **Procedural**,
  rooted by rows 2 and 5. `SKILL.md` L217–227 (the family C/D/E owner, #9) adds a
  further address as row 20; that is a genuine second condition→body address, not a
  reachability crutch, and it is **not yours to write**.
- **Family D second condition — `SPEC-FORMAT.md`'s opening (L6–15).**
  **Triggering.** *"Most increments write no spec at all"* is a condition with
  teeth, and it is what lets this file root itself (row 8).
- **Family D bodies — `SPEC-FORMAT.md` §*The set is current-state*, the membership
  test, and the grain rule (L17–36).** **Procedural.** These are tests you apply
  once you are writing or pruning a spec, not conditions that make you notice you
  should. `SKILL.md` `## Specs` (#12) is a further body of the same rule, and
  `SKILL.md` L217–227 (#9) is the corpus-wide owner; neither is yours.
- **`CONTEXT-FORMAT.md`** carries no family site — it is a pure format guide, all
  procedural, rooted by rows 4 and 6.

### Judgement notes per file

- **`grilling.md`** is vendored and fused from two upstream skills. Its
  `<what-to-do>` block is a procedure a `requirements` session executes; its
  `## During the session` moves are each a small condition-plus-body. Note that
  the file opens with two HTML comments carrying licence provenance — those
  belong to whichever unit follows them, and **must not be split away from it**.
- **`SPEC-FORMAT.md`** carries an inline licence comment at L66–69 mid-file
  (OpenSpec attribution). Keep it with its `## Requirements` prose.
- **`CONTEXT-FORMAT.md`** is a pure format guide with a long leading provenance
  comment. Almost entirely procedural; the condition ("you are resolving a term
  and the glossary needs an entry") lives in `SKILL.md` and `grilling.md`.
- **`ADR-FORMAT.md`** is the smallest and the clearest: no condition of its own,
  two sections, both procedural — now settled above rather than left to you.

## Done when

- All four files are subdivided into real units, and none of them retains a seed
  or `pending-` id.
- **Every procedural unit in all four is reachable**, and the chain that reaches
  the `ADR-FORMAT.md` and `CONTEXT-FORMAT.md` units passes through `grilling.md`
  and terminates. `cargo build` proves all three of those.
- `cargo test` green; `EMBEDDED_UNITS` updated in the same commit, each new id
  named deliberately, and the four seed ids removed.
- **Inventory rows 1–8 are each reported** — written, or declined with a reason.
  Reachability going green is a weaker claim: rows 2 and 5 both reach
  `ADR-FORMAT.md`, so dropping either leaves the build green.
- The sweep is run and its outcome stated, including every `SKILL.md` hit reported
  as *not yours* (rows 20–21 and 29) rather than as missed.

## Notes

- This is the first batch that writes cross-file `defers=`, so it is the first
  chance for `embed-wide-gate-k8`'s reachability, class and termination checks to
  fire in anger. If any of them fires on something you believe is correctly
  classified, that is a finding about the gate or the design — record it, do not
  work around it.
- Doubts to carry forward, by id. The `grilling.md` → `ADR-FORMAT.md` /
  `CONTEXT-FORMAT.md` chains are the least obvious calls in this batch and are
  worth naming even if you are confident.

## Batch record

All four files carved whole. **22 units minted, 4 seed ids consumed, no residual
minted** (`EMBEDDED_UNITS` 58 → 76). `cargo build` and `cargo test` green; every
changed line in `content/` is a marker line, so no prose, filename or fence moved.

**One triggering unit in the whole batch** — `spec-when-a-spec-is-written`. That
is the shape the node brief predicted: three of these four files carry no
condition of their own, and the fourth carries exactly one.

### The units, in document order

`grilling.md` — nine, all procedural:

| id | region | defers |
|---|---|---|
| `grilling-interrogate` | licence comments, title, the whole `<what-to-do>` block | the other eight |
| `grilling-domain-awareness` | `<supporting-info>`, `## Domain awareness`, `### File structure`, lazy creation | `adr-where-adrs-live` |
| `grilling-challenge-the-glossary` | `## During the session` + first move | — |
| `grilling-sharpen-fuzzy-language` | | — |
| `grilling-discuss-concrete-scenarios` | | — |
| `grilling-cross-reference-with-code` | | — |
| `grilling-update-context-inline` | the move + the *glossary and nothing else* rule | `context-structure` |
| `grilling-offer-adrs-sparingly` | | `adr-placement-note` |
| `grilling-agree-the-test-seams` | the move + `</supporting-info>` | `spec-test-seams` |

`ADR-FORMAT.md` — three, all procedural: `adr-placement-note` (title + the
`linkuistics:decision-records` redirect) → `adr-where-adrs-live`,
`adr-why-the-set-stays-minimal`.

`SPEC-FORMAT.md` — seven: `spec-when-a-spec-is-written`
(**triggering `kinds=*`**, the file's self-root) → `spec-set-is-current-state`,
`spec-suggested-shape`; and `spec-suggested-shape` →
`spec-synthesise-never-re-interview`, `spec-behavioural-not-procedural`,
`spec-speak-the-projects-language`, `spec-test-seams`.

`CONTEXT-FORMAT.md` — three, all procedural: `context-structure` (provenance
comment, title, `## Structure`) → `context-rules`,
`context-single-vs-multi-repos`.

### The boundary rule this batch used, stated so the reviewer can attack it

**Carve at a heading or a distinct block; never inside one list.** A bare heading
with no framing prose rides with its first child (`## During the session`,
`## Three rules`) — carving it alone would mint a unit whose whole body is one
line. A heading *with* framing prose is its own unit, as `task-three-design-kinds`
already is; none occurred here.

That is why `grilling.md`'s seven `###` moves are seven units while
`CONTEXT-FORMAT.md`'s eight `## Rules` bullets are one. The moves are sections;
the rules are items in a single list, and a marker between them would split the
list. `kinds-k22` did carve `TASK-FORMAT.md`'s producer bullets individually — but
there a **scope difference** forced it (`kinds=requirements` vs `kinds=design`),
and no such force exists here, where every one of those bullets is procedural.

The plan's own caution applies to the converse temptation, and I checked myself
against it: the four probing moves own no edge and are the smallest units in the
batch, and grouping them *because* of that would have made "owns an edge" the
boundary criterion instead of the document's structure.

**Fine grain is close to free here, which is what made the call easy.**
`grove-llm methodology` takes **several ids in one invocation**
(`src/llm_cli.rs`, `Methodology`), so a unit's `defers=` list is literally the
argv of one fetch: a session holding `grilling-interrogate` pastes its eight-member
list and gets the entire remaining procedure in a single call. The usual argument
for lumping small procedures together — round-trips — does not exist.

### Edge inventory rows owned: 1–8, all eight written

| row | source | target | outcome |
|---|---|---|---|
| 1 | `task-producer-requirements` (`kinds=requirements`) | `grilling-interrogate` | **written** |
| 2 | `task-deliverable-design` (`kinds=design`) | `adr-placement-note` | **written** — see the note below |
| 3 | `task-deliverable-design` | `spec-set-is-current-state` | **written**, direct |
| 4 | `task-deliverable-requirements` (`kinds=requirements`) | `grilling-interrogate`, `context-structure` | **written**, both |
| 5 | `grilling-offer-adrs-sparingly` | `adr-placement-note` | **written** — see the note below |
| 6 | `grilling-update-context-inline` | `context-structure` | **written** |
| 7 | `grilling-agree-the-test-seams` | `spec-test-seams` | **written**, direct to the seam-recording body |
| 8 | `spec-when-a-spec-is-written` | `spec-set-is-current-state` | **written**, direct |

**Rows 2 and 5 land one hop further out than the family-C table's phrasing, and
that is deliberate.** The table says `ADR-FORMAT.md` §*Why the set stays minimal*
is "rooted from `grilling.md` §*Offer ADRs sparingly* and `TASK-FORMAT.md`'s
`design` bullet". Both of those sources cite the **file**, not the section — the
design bullet's parenthesis is a bare `` (`ADR-FORMAT.md`) `` and grilling's is
"[ADR-FORMAT.md](./ADR-FORMAT.md) adds grove's placement conventions" — so both
point at `adr-placement-note`, the file's entry, which chains to both sections.
The body is rooted from both planned roots, transitively. Pointing them at the
section directly would have asserted an address neither prose actually offers.

Consequence the reviewer should weigh: `adr-why-the-set-stays-minimal` has exactly
**one** inbound edge today, from `adr-placement-note`. That is the *safe* side of
the plan's redundancy warning — there is no second path making a dropped real edge
invisible. Row 20 (`execute-k29`) is the edge that genuinely addresses that section
rather than the file, because `SKILL.md` L217–227 states the rework-in-place rule
itself; it should point at the section, not at the entry.

**Inventory addition — one, recorded per the extensibility clause.**

| # | source | target | why |
|---|---|---|---|
| A1 | `grilling-domain-awareness` | `adr-where-adrs-live` | `grilling.md`'s `### File structure` states the peers-split rule in brief and ends "a repo whose contexts are not peers keeps one flat root `docs/adr/` instead — **see `ADR-FORMAT.md`**". The full rule is `## Where ADRs live`. A genuine "the rest is over there", which is what `defers=` is. Both endpoints are in this batch, so this batch owns it. |

Nothing was parked on a residual, and no `defers=` in this batch has a
`pending-*` source.

### The sweep, and what it found

```
grep -rn 'grilling\.md\|SPEC-FORMAT\.md\|CONTEXT-FORMAT\.md\|ADR-FORMAT\.md' content/
```

22 hits. Four are outbound from my own region (rows 5–7 and addition A1, all
written above). Four are the `TASK-FORMAT.md` roots (rows 1–4, all written).
The remaining fourteen are **not mine**, and none is missed:

- **`SKILL.md` L189** — `**Execute.**`'s `requirements` bullet. Row 25,
  `execute-k29`.
- **`SKILL.md` L229, L232** — the fused C/D/E owner paragraph. Rows 20–21,
  `execute-k29`.
- **`SKILL.md` L716** — the `## Artifacts` Specs row. Family D **mention**,
  `finish-cycle-k32`.
- **`SKILL.md` L725** — the `## Artifacts` glossary paragraph. Row 29,
  `finish-cycle-k32`.
- **`SKILL.md` L745** — `## Specs`. Family D **body**, `finish-cycle-k32` (row 30).
- **`SKILL.md` L751–754** — the `## Reference files` index. **Standing sweep
  exclusion**; the node brief already settled it as procedural behind the launcher
  framing, writing no outbound edges.
- **`SKILL.md` L767** — the `linkuistics` prerequisite note. Pre-decided
  `class=triggering kinds=*` with no `defers=`; its targets are not embedded.
  `finish-cycle-k32`.
- **`BRIEF-FORMAT.md` L77** — a template line, "Test seams this subtree's leaves
  share: `<seam>` (see SPEC-FORMAT.md)". Source is carved by `decompose-moves-k28`,
  which is later than this batch, so **that batch owns the edge** and decides
  whether the template's parenthesis is an edge or a citation.
- **`driving.md` L4, L189** — the framing unit's "what the loop is" citation, and
  §*When to invoke a design discussion (grilling)*, which is inventory **row 34**.
  Both `research-moves-k25`'s.

Every `SKILL.md` hit above sits inside `pending-skill-loop`. **No edge may have a
`pending-*` source**, so nothing was written for any of them and no `defers=` was
parked anywhere.

### The pre-decided calls, applied not re-decided

- **Family E bodies** — `grilling-offer-adrs-sparingly` and `adr-placement-note`:
  both procedural. The head is the redirect to `linkuistics:decision-records`, not
  a condition.
- **Family C body** — `adr-why-the-set-stays-minimal`: procedural, rooted per rows
  2 and 5 (via the entry; see above). `execute-k29`'s row 20 is not mine.
- **Family D second condition** — `spec-when-a-spec-is-written`: triggering. *"Most
  increments write no spec at all"* is the condition with teeth and is how this
  file self-roots (row 8).
- **Family D bodies** — `spec-set-is-current-state` carries §*The set is
  current-state*, the membership test and the grain rule **in one unit**, because
  all three inventory rows that address them (3, 8, and #12's 21) name them as one
  target. Procedural.
- **`CONTEXT-FORMAT.md`** — no family site, all procedural, rooted by rows 4 and 6.

### The one scope call this batch had to make

`spec-when-a-spec-is-written` is **`kinds=*`**, not `kinds=design`.

The narrow reading is tempting: writing a spec is `design`'s deliverable, and
`task-deliverable-design` already carries a `kinds=design` second condition. But
this unit is not only "when to write one" — it is *what a spec is*, where specs
live, the four-kind flow, and the negative half (*most increments write none*).
A `requirements` session needs that: `grilling-agree-the-test-seams` instructs it
to record agreed seams "in the spec's `## Test seams`, or, when the increment
writes no spec, in the node's `BRIEF.md`", which is unreadable without knowing what
a spec is. `planning` and `impl` sessions read specs the `design` leaf wrote.

Applying the tie-breaker directly: withholding this yields a session that does not
know the artifact exists, which is an unasked question; carrying it costs ~700
bytes in eighteen other mandates. The asymmetry decides it, not the size.

### Doubts, by id — for `finish-cycle-k32`'s aggregate handoff

The leaf brief asked for the two chains by name even where I am confident, so both
are here, and two more besides.

1. **`grilling-domain-awareness` → `adr-where-adrs-live` (addition A1).** The
   least-planned edge in the batch. It is honest — the prose says "see
   `ADR-FORMAT.md`" about a rule it states only in brief — but it is a
   *procedural→procedural* hop that no inventory row anticipated, and it gives
   `adr-where-adrs-live` a second inbound path (the first being
   `adr-placement-note`). If the reviewer thinks addition edges should be reserved
   for reachability rather than addressing, this is the one to cut; nothing depends
   on it.
2. **`grilling-update-context-inline` → `context-structure` (row 6), with row 4
   also pointing at `context-structure` directly.** Both were planned, so
   `CONTEXT-FORMAT.md`'s entry now has two inbound edges and a `requirements`
   session reaches the glossary format two ways. That is the plan's own design, but
   it is exactly the "two paths, drop the real one, build stays green" shape the
   brief warns about — worth confirming that both rows are still wanted once
   `execute-k29` and `finish-cycle-k32` have added rows 25 and 29.
3. **`grilling-interrogate`'s eight-member `defers=`.** The alternative was a
   container unit for the seven moves, which I rejected because
   `## During the session` carries no framing prose. If the reviewer prefers a
   shallower graph, the repair is to introduce that container — but it would be a
   unit whose entire body is a heading.
4. **`spec-set-is-current-state` as one unit rather than three.** The membership
   test and the grain rule are separately citable rules (`CONTEXT.md`'s **Spec**
   entry names the membership test by name), and a finer carve would let a future
   condition address one without the others. I kept them fused because rows 3, 8 and
   21 all name them as a single target — applying the plan rather than re-deciding.
   If `execute-k29` or `finish-cycle-k32` finds itself wanting to address only the
   grain rule, this is the unit to split, and splitting it later is the decoupling
   lemma's easy direction (new marker, no id or class change to the remainder).

### Design findings

- **`grilling.md`'s HTML wrapper tags do not survive unit boundaries.**
  `<supporting-info>` opens in `grilling-domain-awareness` and closes in
  `grilling-agree-the-test-seams`, so six units in between are, read standing
  alone, inside an unclosed tag. Nothing breaks — the parser tracks CommonMark
  fences, not HTML blocks, and a fetched unit is served verbatim — and the same
  would be true of any wrapper spanning a carve. Recording it because the authoring
  rule "a unit must read correctly standing alone" is the one no build checks, and
  this is the corpus's one instance of a *structural* violation of it that is
  nonetheless correct. Removing the two tags is a prose edit for a later grove, not
  a marking decision — and the file is vendored, which is a second reason not to
  touch it here.
- **No prose in these four files was neither condition nor procedure.** The node
  brief asked for that to be said if it turned up. It did not; these are format
  guides and a procedure, and every byte belongs to one class or the other.

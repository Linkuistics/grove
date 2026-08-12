# lifecycle-k31

## Goal

Classify **`content/SKILL.md` from `**When a picked producer needs fresh review**` to
the line before `**Finish.**`** (baseline L408–609, 13,712 bytes): `**When a picked
producer needs fresh review**`, the *tree is a real directory tree* paragraph
(`leaf-decompose`, `leaf-add`, `leaf-insert`), `**`--kind <kind>` appears on the
grow verbs…**`, `**Reading is strict too**`, `**Retire.**` with the pruning case
and the node-close cascade, `**Commit.**`, and `**Signal.**`

This is batch 11 of 12 — the loop's back half, from the grow verbs through the
session boundary.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L408–609 is a baseline coordinate — three batches have
  inserted markers above your region by now.** Carve from `**When a picked producer
  needs fresh review**` to the line **before** `**Finish.**`, consuming the front of
  `pending-skill-lifecycle`. Your region ends **including** the blank separator before
  that anchor, per the marker-placement convention — the F7 correction, which is why
  the baseline range is L408–609 and the size 13,712 rather than L408–608 / 13,711.
- Mint exactly one residual, **`pending-skill-finish`**, covering `**Finish.**` to end
  of file, as `class=triggering kinds=*` **with no `defers=`**.
- **There is nothing to inherit from `pending-skill-lifecycle`.** A residual never
  carries `defers=`.

### The pre-decided call: the ADR reconciliation clause is a *mention*

`batches-k13` told you to read `evidence-moves-k26`'s three-way overlap call and
honour it. The node brief settles the family instead, and your side of it is the
simplest of the three:

**`SKILL.md` L550–554 — *"Retirement is also the moment to reconcile the ADR set…
never append a superseding ADR"* — is a family-C mention, not a site to decide.** It
is a clause inside the node-close cascade's prose, and it is **unsplittable from it at
line granularity** (baseline L554 carries the end of the ADR clause and the start of
*"That may leave the next ancestor with no live leaf either"*). It takes the cascade
unit's class, which is procedural, and it owes **no** family edge: family C's owner —
`SKILL.md` L217–227, carved by `execute-k29` — already names retirement as a
checkpoint, and so does the owner's body in `driving.md` §*Reworking ADRs and
briefs…*, whose closing paragraph names *"retiring a leaf or node"* explicitly.

So the ADR-reworking rule reaches a retiring session through the mandate it already
holds, not through a second condition here. Do **not** carve L550–554 out as a third
statement of the rule.

### Edge inventory rows owned: 28 and 37

| row | source | target | note |
|---|---|---|---|
| 28 | `**Retire.**`'s node-close step 1 (L533, *Check the node's brief `Done when`*) | `BRIEF-FORMAT.md` bodies | `decompose-moves-k28` carved the target |
| 37 | `driving.md` §*Prune, reorder, or file an issue* (L702, its `SKILL.md` citation) | your `**Retire.**` pruning body | **the one edge in the whole inventory whose source is carved before its target** — `decompose-moves-k28` could not write it, so it is yours, written *into* an earlier batch's marker. Conditional on your pruning prose ending up procedural; decline with that reason if it is triggering |

### The judgement this batch exists for

This region contains grove's **densest procedural prose** and its **two HITL
conditions**. The split should be sharp, and the risk runs one way: classifying
verb mechanics as triggering because they are stated as rules.

- **The *tree is a real directory tree* paragraph** (L417–448, 4,466 bytes) is
  almost entirely the mechanics of three verbs — what `leaf-decompose` moves, what
  `leaf-insert` shifts, why headers rewrite zero file contents. Procedural. The
  condition ahead of it (*the current item proves bigger* / *a new concern must
  sequence ahead*) already lives in `**Decompose.**`, carved by `execute-k29`.
  Check that unit and defer into these bodies from it rather than restating the
  condition.
- **`**Retire.**`** (L471–562, 6,013 bytes) is the largest block in the region and
  is genuinely mixed:
  - *A leaf ends one of two ways* and *retirement touches one filename and nothing
    else* — conditions.
  - **Pruning is HITL — an agent never prunes on its own**, and an AFK session
    that discovers the path is decided against **says so and stops**. That is a
    condition of the first importance: a session never told it would prune on its
    own authority, and the node brief's asymmetry argument applies at full
    strength. Do not bury it in a procedural body.
  - The **node-close cascade** — the four numbered steps, the brief-less-node
    exception, the recursion up the parent chain — reads procedural, but *the
    close asks the human nothing* is a condition, and so is *escalate if the check
    fails and you cannot name the gap*.
  - The `leaf-retire` / `leaf-prune` invocations and the `DONE`/`ABANDONED` infix
    mechanics are procedural.
- **`**Commit.**`** (L564–591) states *one task = one focused commit* and
  *name the work item by its stable handle* — both triggering — and then the
  git/jj asymmetry and the jj sealing mechanics, which are procedural. Note the
  pointer to `linkuistics:using-jujutsu`, which is **not** an embedded file and
  therefore never a `defers=` target.
- **`**Signal.**`** (L593–608) is a condition (*run `grove-llm complete` as your
  last action, then do nothing else*) with a short mechanical body. The
  `complete` / `complete --done` distinction is load-bearing and belongs with the
  condition, not the body.

### Scope

`kinds=*` throughout. The pruning HITL rule names `requirements`, `prototype` and
`finish` as the HITL kinds, which tempts a scope — but the rule is addressed to
*the AFK session that must stop*, so scoping it to the HITL three would deliver it
to precisely the kinds that do not need it. `**Signal.**`'s `--done` clause is
about `finish`, but every session needs to know the plain form is not the
`finish` form.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-skill-finish` covers the rest of the file and nothing else, and carries no
  `defers=`.
- **Rows 28 and 37 are each reported** — written, or declined with a reason.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The pruning-HITL and node-close classifications are recorded with their reasoning —
  they are the two most consequential calls left in this batch now that the ADR
  overlap is settled.

## Notes

- Fenced/indented blocks: the four numbered close steps are a list, not a fence —
  but check fence state before placing any marker, because a marker recognised
  only at neutral state that lands mid-fence silently joins the preceding unit.
- Doubts to carry forward, by id. The pruning-HITL and node-close classifications
  are the two most consequential in this batch.

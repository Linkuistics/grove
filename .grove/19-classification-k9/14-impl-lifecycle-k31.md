# lifecycle-k31

## Goal

Classify **`content/SKILL.md` lines 408–608** (13,711 bytes): `**When a picked
producer needs fresh review**`, the *tree is a real directory tree* paragraph
(`leaf-decompose`, `leaf-add`, `leaf-insert`), `**`--kind <kind>` appears on the
grow verbs…**`, `**Reading is strict too**`, `**Retire.**` with the pruning case
and the node-close cascade, `**Commit.**`, and `**Signal.**`

This is batch 11 of 12 — the loop's back half, from the grow verbs through the
session boundary.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- Carve `content/SKILL.md` **L408–L608**, consuming the front of
  `pending-skill-lifecycle`.
- Mint exactly one residual, **`pending-skill-finish`**, covering **L610–L760**.
- Redistribute any `defers=` inherited from `pending-skill-lifecycle`.

### Cross-file deferral

Sparser here than in batches 9–10, but two edges matter:

| `SKILL.md` site | target | carved by |
|---|---|---|
| L533 — *Check the node's brief `Done when`* | `BRIEF-FORMAT.md` | `decompose-moves-k28` |
| L550–554 — *reconcile the ADR set … never append a superseding ADR* | `ADR-FORMAT.md`, and `driving.md` §*Reworking ADRs and briefs as understanding shifts* | `guides-k24`, `evidence-moves-k26` |

**`evidence-moves-k26` recorded a three-way overlap call** on the ADR-reworking
rule (`SKILL.md` here, `driving.md` L285ff, `ADR-FORMAT.md` §*Why the set stays
minimal*). Read that call and honour it — this is the `SKILL.md` half it named.

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

- `content/SKILL.md` L408–608 is subdivided into real units;
  `pending-skill-finish` covers L610–760 and nothing else.
- Every edge in the table above is written or explicitly declined with a reason.
- Any `defers=` inherited from `pending-skill-lifecycle` is redistributed and
  accounted for.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- The ADR-reworking consistency call against `evidence-moves-k26` is recorded.

## Notes

- Fenced/indented blocks: the four numbered close steps are a list, not a fence —
  but check fence state before placing any marker, because a marker recognised
  only at neutral state that lands mid-fence silently joins the preceding unit.
- Doubts to carry forward, by id. The pruning-HITL and node-close classifications
  are the two most consequential in this batch.

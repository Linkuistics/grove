# mandate-machinery-k10

## Goal

Delete the mandate machinery and the unit markers it read, and retire the record
that describes it. Nothing reads a marker after `guaranteed-core-k9`, so this is
pure removal.

## What goes

- **The marker grammar and total partition** (`src/methodology/parse.rs`), and the
  140 `<!-- unit: … -->` / `<!-- file: order= -->` markers throughout `content/`.
- **The fence-state parser** and the **whole-embed check**
  (`src/methodology/whole_embed.rs`) — completeness invariant, reachability,
  termination, id and `order=` uniqueness.
- **The build gate** (`build.rs`), and the `#[path]` sharing of both readers.
- **The composer** — `methodology::compose`, `Unit`, `Class`, `Scope`,
  `methodology::units`.
- **`grove-llm methodology`** and its listing (`src/llm_cli.rs`).
- **The file-ordering directive.**
- **`tests/goldens/composed-mandates.tsv`** and the tests over it.
- **`docs/specs/mandate-delivered-methodology.md`** — this is the increment that
  removes the last machinery it describes, which is exactly when it may go.

## What must survive, and why it is easy to delete by accident

**Two checks in that neighbourhood are claims about the embed rather than about the
machinery.** They currently sit beside the parser and will be swept along with it
unless they are deliberately re-based:

- **The instructed-verb scan** — the embedded methodology instructs no `grove-llm`
  verb the embedded CLI lacks. It becomes **more** load-bearing here, not less: the
  skill is once again the only thing teaching a session which verbs exist. It must
  be re-expressed over the embed as markdown, since it can no longer walk units.
- **The flat-verb-surface pin** — what makes that comparison mean what it claims.

Also surviving, unchanged: `provision` (the per-invocation sweep of the embed into
every installed harness's personal skill directory, guarded by a content-hash
stamp, with per-launch re-verification), `methodology::embed()`, and
`methodology::identity()`.

## What each deleted device bought, and what pays for it afterwards

Recorded so the deletion is done knowing the cost, not in ignorance of it:

| device | bought | what pays afterwards |
|---|---|---|
| marker grammar + total partition | *unclassified prose cannot exist*; a blind parser yields a visibly larger unit, not a silent hole | no parser, no classification — nothing to be blind to; a harness reads the corpus as markdown |
| completeness invariant | a mechanical answer to *did we build the mandate right?* | the corpus is delivered whole. Note its own limit: it settled which units a mandate carries, "never that a session notices the situation a carried condition describes" |
| reachability + termination | *an undiscoverable procedure is impossible* | the skill's routing table and the file system, both read directly by a human |
| build gate | a contributor's build error in place of a stranger's stalled loop | corpus tests in the suite — every kind's reference file exists, `SKILL.md` stays within budget — for the reason the size alarm lives there |
| `grove-llm methodology` | addressed, not guessed, lookup of a deferred body | nothing is deferred |
| file-ordering directive | a total composition order over ten files | the prompt's fixed three parts |

## The glossary rework lands here

`plan-k1`'s call, and it stands: the `CONTEXT.md` entries that are accurate
descriptions of a live mechanism are reworked by the increment that removes it.
That is this one — **Methodology unit**, **Mandate slice**, **Triggering unit /
procedural unit**, **Build pairing**. Rework in place; the glossary is a
current-state set like the ADRs.

## Citation reconciliation

`grep -rn 'mandate-delivered-methodology'` is the surface. Roughly ten live source
sites cite its sections as the rationale for code that still runs — the parse gate,
the fence rule, the file ordering, the composer's runtime-facts rule — and every one
of them is deleted by this leaf, so the citation goes with the code. The remaining
prose sites move here too. **Nothing may still point at it when it is gone**; that
is the dangling-citation defect the rework discipline forbids, and it is the reason
the spec's deletion was timed to this increment rather than to the cutover.

`docs/ARCHITECTURE.md`'s *Embedded methodology* section is the largest prose
casualty — it describes the marker grammar, the gate and the composition order at
length. `src/lib.rs`'s module commentary on `methodology` needs the same pass.

## Done when

The machinery is gone, `content/` is marker-free, both surviving checks are green
on their new footing, the glossary and every citation are reconciled, and
`cargo test` is green.

## Notes

The 140 markers were **scaffolding for the rewrite, not an obstacle to it** — they
recorded which prose is `if` and which is `then`, which is exactly the split the
progressive-disclosure skill needed. They were used in `corpus-rewrite-k7`. This is
where they are deleted.

Deletion of this size is where a review chain earns its place. `review-impl` on
"what survived that should not have, and what went that should have stayed" is the
shape; the two must-survive checks above are the first thing it should look for.

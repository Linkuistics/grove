# guaranteed-core-k9

## Goal

**The cutover.** Add the `prompt` module, switch `${prompt}` from the composed
mandate to the guaranteed core, and rework the two records the reversal falsifies.

## Precondition

`corpus-rewrite-k7` is done. The design fact — *`${prompt}` must not shrink before
`SKILL.md` is short* — is already satisfied by that node's construction (the two
shrink in the same edits), so this leaf is not where the ordering constraint bites.
What it does need is the ten reference files at stable paths, `SIGNAL.md` at its
final path, and `wording-micro-test-k6`'s recorded wording.

## The three parts, in order

`${prompt}` is a fixed three-part template, and the order is the session's own
timeline:

1. **The load instruction** — first, because it is the first action. Names the skill
   by name and this kind's reference file by path, with the provisioned directories
   by absolute path. Wording comes from `wording-micro-test-k6` **verbatim** — five
   elements the spec enumerates, and the largest part of the core.
2. **The runtime facts** — the selected handle and the stated version control.
3. **The session ending** — last, because it is the last action.

Recency is the whole reason for the third position, and it is **inherited rather
than re-derived**: the ending was moved to compose last after sessions were seen
finishing correctly and then not signalling. The file-ordering machinery that held
it there is retired in `mandate-machinery-k10`; the property survives as a fixed
template a human verifies by eye.

Two consequences stated rather than hidden. The ending's recency advantage is much
weaker here — it trails ~1.5 KiB rather than seven files — so the position is nearly
free and buys correspondingly less. And the ending is delivered **twice** to every
session, once in the core and once inside the skill: deliberate, costs no drift, and
the one place the design accepts a reader's *do these agree?* because the answer is
mechanically yes.

## The closed fact test governs what may be added

> A driver fact is a **launch-varying value** that `content/` cannot know at build
> time. Its static meaning, and every normative consequence of it, stay in the
> skill.

The core carries a **value**, never a restated rule. *And the pick is
authoritative*, *do not probe for the version control* — both are rules with
counterparts in `content/`, and putting them here would be the second source the
whole design exists to avoid. They are `loop-conditions-k13`'s to state in the
skill; if they are missing there, fix them there, not here.

## The seams

- **`prompt`** (new) exposes composition over `(kind, handle, stated VCS,
  provisioned locations)` and the kind→reference-file mapping.
- **`methodology`** (narrowed) keeps the embed handle and the methodology identity,
  and loses the two readers, the unit model and `compose`. Both survivors are live:
  `provision` consumes the whole embed (`src/methodology.rs`, `embed()`), and the
  identity feeds the per-launch pairing report and the per-verb stamp check
  (`identity()`; `src/provision.rs`, `reverify_installed`).

`prompt` **depends on** `methodology` rather than absorbing it — composition reads
the embed to inline the ending file and to assert its mapped paths exist. Moving
embed ownership into `prompt` would put provisioning's supplier behind a
prompt-composition seam.

The **kind→file map is an exhaustive `match` over the kind enum, in the driver** —
routing by kind is what the driver already owns. Its hazard (a twentieth kind
silently absorbed into a family's file) closes the way this repository already
closes it: a match that fails to compile until someone classifies the new variant,
plus a test that every path it yields exists in the embed.

## The checks

Through the `prompt` seam, against the **real embed**:

- **The three-part shape**, asserted structurally rather than by substring: the
  template with its four substitutions, exactly one of which is embedded content.
  What the wording *says* is not mechanically checkable and is not claimed to be.
- **The ending is the embedded file's bytes**, so a driver-side copy cannot
  reappear as a Rust literal without failing.
- **The two couplings not closed by construction**: the skill name the core states
  against the embedded `SKILL.md`'s own `name:`, and every mapped reference path
  against the embed. Both fail **by name**. (The other two couplings — the inlined
  signal file and the provisioned locations — are closed by construction, being
  compile-time embedded and computed by the same registry that writes them.)
- **The size alarm**, per kind, at **4 KiB** on the composed prompt. It lives in the
  test suite, not the build — it measures a judgement against an admittedly
  arbitrary number, and failing a contributor's build on that is a gate this design
  is careful not to erect. Measured composition is ~2.7 KiB (1,131 bytes of ending,
  ~360 of runtime facts, ~1.2 KiB of load instruction), so a third is in hand and
  **nothing legitimate approaches it**.
- **A control on every generated claim**: the mapping check shown failing on a kind
  whose path is removed, the size alarm on a synthetic oversized prompt. A sweep
  that cannot fail is worth nothing.

**Golden per-kind prompt snapshots are dropped.** The ids-not-bytes golden existed
because nineteen ~48 kB mandates could not be held as bytes; nineteen ~2.7 KiB
prompts differ only in one path and one handle, so the mapping check says
everything a golden would and says it by name.

## The record rework

- **`mandate-delivers-the-methodology` → `skill-delivers-the-methodology`**,
  reworked **in place** and renamed. The slug is the identity, so a slug saying the
  mandate delivers the methodology cannot survive the mandate not delivering it —
  and `mandate-` → `skill-` keeps citation reconciliation mechanical. **One record,
  not two**: the delivery reversal and the core rule are inseparable (answering
  either alone is a swap), so the core rule is a named section inside it, with its
  own reopen conditions under `## Considered options` — the predictable failure mode
  is erosion by addition, and erosion needs somewhere to be argued against.
  - **Retained**: that a session told a fact succinctly never runs the derivation
    that would have established it; that driver-authored prose about the methodology
    would make `content/` non-canonical; that the `if`/`then` asymmetry is real.
  - **Overturned**: *"only as a supplement… never as a replacement for triggering
    conditions."* The spec's four-strand argument is the material — the clause's own
    reopen condition fired, the risk model counted one failure where the evidence
    names two, nothing is withheld any more so the failure changes kind and leaves a
    trace, and the residue is real and paid for by trigger strength.
- **`one-build-owns-a-session` — targeted rework, not a rewrite.** Substance
  untouched. What changes is the paragraph asserting that *since the mandate
  delivers the methodology there is no shared directory left to clobber*: the shared
  directory returns, and with it the skew the record was originally written for —
  two copies of a whole methodology, the provisioned skill and the resolved CLI. The
  *split-brain inside one rule* framing goes with the deferral that produced it, as
  does the claim that the failure is loud because the deferral is declared; **the
  returning skew is quiet**, which is what the pre-launch report and the per-verb
  stamp warning exist for.
  - One consequence that looks like it should reopen and does not: **the
    compile-time methodology-identity constant stays deleted.** It existed so that
    naming the identity did not link the embed, and `grove-llm` links the embed for a
    second surviving reason — its per-verb foreign-skill-directory warning needs the
    identity.
- **`CONTEXT-MAP.md`'s shared-target relationship stays, minus one clause.** The
  `grove` entry in the personal skill directory was recorded as going away; it does
  not, so the relationship is a relationship again.
- **Citation reconciliation for the rename.** `grep -rn
  'mandate-delivers-the-methodology'` is the surface — about a dozen sites across
  `CONTEXT.md`, `CONTEXT-MAP.md`, `docs/`, `src/` and `tests/`. A rename breaks links
  immediately, so **all** of them move in this leaf.
  `docs/specs/mandate-delivered-methodology.md` and its citations do **not** — it
  still describes live mechanism and is `mandate-machinery-k10`'s to delete.

## Done when

The core ships, every check above is green with its control, both records are
reworked in place, and no link to the renamed ADR dangles.

## Notes

This leaf is large and load-bearing. Its natural review is `review-impl` on the
seam and the checks; cut it with `leaf-add` if the work warrants one, and cut the
integration with `leaf-insert` against the next live sibling entry so the findings'
line coordinates do not drift under it.

`docs/ARCHITECTURE.md`'s module-seam table gains the `prompt` row and rewrites
`methodology`'s to *the embed itself and the build's methodology identity* —
nothing about units, composition or readers.

# evidence-moves-k26

## Goal

Classify **`content/driving.md` from `## When to retire research into ADRs versus
leave it` to the line before `## Doubting inside a picked Grove leaf`** (baseline
L264–414, 8,528 bytes): that section, `## Reworking ADRs and briefs as understanding
shifts`, `## Verifying framework decisions against the source`, and `## Verifying a
claim about the repo itself`.

This is batch 6 of 12. The theme is **evidence discipline** — where a claim's
proof lives and when it becomes binding.

## Context

Read the node brief's *The batching contract* first.

### Region and residual

- **Anchors are authoritative; L264–414 is a baseline coordinate.** Carve from `##
  When to retire research into ADRs versus leave it` to the line **before** `##
  Doubting inside a picked Grove leaf`, consuming the front of
  `pending-driving-evidence`.
- Mint exactly one residual, **`pending-driving-doubt`**, covering `## Doubting
  inside a picked Grove leaf` to end of file, as `class=triggering kinds=*` **with
  no `defers=`**.
- **There is nothing to inherit from `pending-driving-evidence`.** A residual never
  carries `defers=`, so there is no list to redistribute and no member to account
  for. `batches-k13`'s redistribution protocol was removed by `batches-k33` F2: a
  member parked on a residual can be dropped while the build stays green, because
  the target usually has another inbound path.

### The pre-decided calls in this region

The three-way overlap on ADR reworking that `batches-k13` asked you to *decide* is
settled in the node brief. Apply it:

- **Family C body — `## Reworking ADRs and briefs as understanding shifts`
  (L285–310).** **Procedural.** Its owner is `SKILL.md` L217–227 (#9), which is
  carved *after* you — so **root it from `## When to retire research into ADRs versus
  leave it` in this same batch** (row 9). The corpus itself points there: L277 reads
  *"see *Reworking ADRs and briefs* below"*. #9 later adds the owner's address as
  row 22; that second inbound edge is a genuine condition→body address, and it is
  not yours to write.
- **Family C second condition — `## When to retire research into ADRs versus leave
  it in docs/research/` (L264–283).** **Triggering.** It states a *different*
  trigger — a research finding becoming binding on future work — so it ships on its
  own account. Its own *"you are editing the ADR **in place** — the set is
  current-state"* clause (L274–277) rides along as a **mention**: do not carve it
  out as a third statement of the rule.
- **`ADR-FORMAT.md` is already rooted** by `guides-k24` (rows 2 and 5). Row 10 —
  this region's `ADR-FORMAT.md` citation at L285ff — is yours, and it is a genuine
  second address only if the citation is a trigger→body reference rather than a
  provenance pointer. Read it and decide; **declining with a reason is a legitimate
  outcome**.
- The `linkuistics:decision-records` pointer is **not embedded** and can never be a
  `defers=` target.

### The judgement this batch exists for

All four sections are self-rooting — each names its own condition in its heading —
but the *grain* is the question. Two of them are conspicuously large relative to
their neighbours:

- **`## Verifying a claim about the repo itself`** (3,764 bytes) is the biggest
  section in the region and is mostly a worked procedure for turning a grep into
  evidence. The condition is small and sharp (*a session is about to assert
  "every X is now Y" about its own codebase*); the body is long. That asymmetry is
  the design working — keep the `if`, defer the `then` — and this section is the
  cleanest example of it in the whole corpus. Classify it that way deliberately,
  not by inertia.
- **`## Reworking ADRs and briefs as understanding shifts`** (1,914 bytes) states
  a rule stated in four other places. **The call is made for you** (above): this is
  the family-C **body**. What is still yours is the *grain* — how many units it
  becomes, and whether its three bullets (edit in place / keep the set minimal /
  reconcile every citation) are one body or three.

`SKILL.md` references in this region point at constraints, and they sit inside
`pending-skill-*`. Not `defers=`; report them as *not yours*.

### Size note

At 8,528 bytes this is the smallest `driving.md` batch. That is deliberate: the
region's grain is fine and its four sections are independent, so the session cost
is in the judgements, not the bytes. If it finishes early, **do not absorb
`pending-driving-doubt`** — `doubt-moves-k27` carries the review chain, which is
the single largest section in `driving.md` and needs its own context.

## Done when

- The region between the two anchors is subdivided into real units;
  `pending-driving-doubt` covers the rest of the file and nothing else, and carries
  no `defers=`.
- `cargo build` and `cargo test` are green.
- `EMBEDDED_UNITS` updated in the same commit, each new id named deliberately.
- **Rows 9 and 10 are reported** — row 9 written (it is what makes
  §*Reworking ADRs…* reachable at the end of *this* batch), row 10 written or
  declined with a reason.
- The family-C body's id is named in this leaf's body, so #9 can write row 22
  without re-deriving it.

## Notes

- `## Verifying a claim about the repo itself` contains fenced shell examples.
  Do not split mid-fence.
- Doubts to carry forward, by id.

# increments-review-k11

**Reviews:** `increments-k4`

## Goal

Try to **disprove** the decomposition `increments-k4` produced — the two-grove
split, the leaf order, and the narrowed root brief — before four leaves are
executed on top of it. Inspection only: read the tree, the root `BRIEF.md`, the
five leaf bodies, the spec, the ADR and the source they cite. Do not edit
`content/`, `src/`, or any leaf body, and do not run build, test, lint or format
commands.

If findings survive, cut `increments-integrate` with them written out verbatim.
If nothing survives, create nothing and retire — that is the outcome this shape
exists to make cheap.

## Context

`increments-k4` (retired, committed under that handle) did four things. Each is a
place it could be wrong.

1. **Split the workstream into two groves rather than four.** This grove takes
   design stages 1–2 (mark, parse, gate, `grove-llm methodology`); a successor
   takes stages 3–4 (compose, retire provisioning). The stated reasons: stage 1
   delivers no usable behaviour alone, and a grove boundary between stages 3 and 4
   would integrate the both-paths state the design permits only as a transient.
2. **Narrowed the root `BRIEF.md`'s `Done when`**, which the human agreed during
   `plan-k1` grilling and which covered all four stages. The narrowing was taken
   as sanctioned by the old brief's own sentence — *"candidate separate groves…
   a call for the `planning` leaf, not for this brief"* — without going back to
   the human.
3. **Ordered the leaves** `ordering-key-placement-k6` → `unit-grammar-k7` →
   `methodology-verb-k8` → `classification-k9`, deliberately putting the
   judgement-heavy classification **last** so its `review-impl` chain stays
   contiguous with it, and `step-suffix-redundancy-k10` after everything as an
   unrelated parked concern.
4. **Found one design/increment conflict and externalized it** as
   `ordering-key-placement-k6`: KDL frontmatter is required on every embedded
   markdown file from the first increment, but `content/SKILL.md`'s YAML
   frontmatter is what harnesses read to discover the provisioned skill, and
   provisioning stays live until the successor grove's final stage.

### Specific doubts, in the order they would hurt most

- **Are there more hazards of species (4)?** One was found by accident while
  ordering increments — a design decision written as if provisioning were already
  gone, landing in an increment where it is not. That is a *class*, not an
  incident. Re-read `docs/specs/mandate-delivered-methodology.md` asking of each
  decision: *does this assume something the increment that lands it has not yet
  done?* Candidates worth checking by name: the release-path scan inversion in
  `scripts/release-common.sh` against a `grove-llm` that only starts linking the
  embed in `methodology-verb-k8`; `INSTRUCTED_VERBS` gaining `methodology`, which
  `increments-k4` argued cannot happen until `content/` names the verb; and the
  compile-time methodology identity, whose removal is scheduled for the successor
  grove although `grove-llm` links the embed in this one.
- **`unit-grammar-k7` is not a vertical slice.** It is inert by construction —
  nothing reads a unit, no behaviour changes, nothing is demoable. The
  methodology asks a planning session for slices that stand demoable on their own,
  and this one does not. `increments-k4` accepted that because the design argues
  for landing the mechanism green before judgement is spent. Is that a real
  exception or a rationalization? If it is a rationalization, what is the
  alternative that is actually vertical — and does it reintroduce judgement into
  a moving mechanism?
- **Is the trivial-marking correction right?** `increments-k4` overturned the
  design leaf's *one unit per file, `class=procedural`* on the grounds that
  `mandate-delivery-integrate-k5` added reachability, which an all-procedural
  corpus cannot satisfy — and substituted `class=triggering kinds=*`, legal only
  while no composer and no 64 KiB alarm exist. Check both halves against the spec.
  A wrong answer here fails `unit-grammar-k7`'s build on its first commit.
- **Is `classification-k9` one session?** ~139 kB across nine files, every unit
  subdivided, `kinds=` and `defers=` assigned throughout, and a pinned complete id
  set to update. `increments-k4` left it as a single leaf. If it is not one
  focused session, the honest answer is that it should have been decomposed here —
  and by file is the obvious seam, with `SKILL.md` alone at ~51 kB.
- **Is `ordering-key-placement-k6` design-leaf-sized?** Its own body says to
  retire early if the answer is one paragraph. The opposite risk is that it is
  larger than one leaf, because option 2 (an HTML-comment file directive) would
  overturn a named spec decision and might want its own review.
- **Does the successor grove's charter survive the finish cycle?** It lives in
  this grove's root `BRIEF.md`, and briefs die when `.grove/` is torn down. The
  stage-4 enumeration in particular exists nowhere else. Is that a promotion this
  grove's finish will remember to make, or should something be recorded durably
  now — and if so, where, given a spec describes a design and not a work plan?
- **Is `step-suffix-redundancy-k10` in the right grove at all?** It is unrelated
  to mandate delivery and it widens a brief that was just narrowed. The
  methodology's answer for a human-raised concern is `leaf-add` to *the* tree, and
  there is only one. Say whether that is the right reading or whether it should
  have been recorded for a grove of its own.

## Done when

Every doubt above is answered — confirmed or refuted — with the reasoning
attached, and any finding is written into `increments-integrate` in enough detail
that the integrating session does not have to re-derive it. A confirmed
non-finding is worth recording too: it stops the next session reopening settled
ground.

## Notes

- The tree itself is the artifact under review, and it is committed. Read it with
  `find .grove`, and read the five leaf bodies in full — the plan's substance is
  in the bodies, not in the filenames.
- Do not re-review the *design*. `mandate-delivery-k2` was reviewed by
  `mandate-delivery-review-k3` and repaired by `mandate-delivery-integrate-k5`;
  four of its five findings are settled and one non-finding list is recorded there
  specifically to stop it being reopened. What is in scope is where the design's
  work is **cut and ordered**, plus any place the design contradicts the order it
  is being executed in.

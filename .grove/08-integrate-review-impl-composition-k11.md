# composition-k11

**Integrates:** `composition-k10`, the review of producer `composition-k7`

## Goal

Resolve the three findings from `composition-k10`: restore the byte-level drift
pin the ending slice relies on, make the file-directive exemption coherent with
every statement of the partition invariant, and choose an ordering-key policy
whose rationale matches the values in `content/`.

## Context

### Finding 1 — narrowing the golden removed a pin that the next leaf still assumes

The ID-only golden is the right grain for composition drift: a byte golden for
nineteen roughly 48 KiB mandates would churn on ordinary prose edits. The spec
correctly moves the two prose-only ending claims to a targeted byte assertion
beside those claims (`docs/specs/mandate-delivered-methodology.md:587`), but the
live ending task still says those claims are “pinned for drift by the goldens”
and asks only for updated golden snapshots
(`.grove/10-impl-session-ending-k9.md:68`,
`.grove/10-impl-session-ending-k9.md:85`). Because the golden now contains only
unit ids, that task can meet its written `Done when` without adding the byte pin
the current spec requires.

Amend `session-ending-k9` before it runs: name the targeted byte-level assertion
on the ending units explicitly, and stop crediting the ID golden with a property
it no longer has.

### Finding 2 — the directive exemption leaves the load-bearing byte invariant false

The reconciliation introduces the file directive as a body line covered by no
unit, but several current-state records still derive “every byte of the
methodology is either in a mandate or reachable from one.” Directive bytes are
neither: the parser consumes them as ordering metadata, `compose` omits them,
and `grove-llm methodology` neither lists nor serves them.

The contradiction is present in the spec itself
(`docs/specs/mandate-delivered-methodology.md:82` and
`docs/specs/mandate-delivered-methodology.md:189`), the cited ADR
(`docs/adr/mandate-delivers-the-methodology.md:49`), the glossary
(`CONTEXT.md:104` and `CONTEXT.md:199`), architecture
(`docs/ARCHITECTURE.md:691` and `docs/ARCHITECTURE.md:716`), and source/test
commentary (`src/methodology/whole_embed.rs:18`,
`tests/methodology.rs:1025`). Reconcile the minimum coherent set. Preserve the
intended load-bearing guarantee over instruction/unit bytes while naming the
preamble and file directive as the two bounded non-unit regions; do not retain a
universal claim over the embed's literal payload bytes.

### Finding 3 — legal gaps do not deliver the stated no-renumber benefit with dense keys

`CONTEXT.md:151` says a file inserted between two others renumbers nothing, and
the spec says density would force renumbering “for no gain”
(`docs/specs/mandate-delivered-methodology.md:456`). The actual keys are the
contiguous integers 1–9. With an integer `u32` key there is no value between any
adjacent pair, so an insertion there necessarily renumbers later files. Legal
gaps provide no insertion slack until the assignments are deliberately spaced;
meanwhile they permit arbitrary monotonic integers without a density check.

Choose one coherent policy rather than preserving both halves of the claim:

- keep gaps legal and space the values now (position 1 remains fixed for the
  launcher rename), so the no-renumber rationale is true; or
- keep the readable dense values and enforce or document density, accepting the
  small renumbering diff when a file is inserted.

Whichever answer stands, update the glossary/spec explanation and its test so
the implemented policy, the corpus values, and the claimed trade-off agree.

## Done when

- `session-ending-k9` explicitly requires the targeted byte assertion promised
  by the spec; its ID-golden requirement is described as composition drift only.
- The spec, ADR, glossary, architecture and source commentary state a partition
  invariant that remains literally true with the preamble and directive regions.
- File-order keys and their documented/tested gap-or-density policy agree; no
  no-renumber guarantee is made while the values remain dense.
- The focused verification for the chosen fixes and the repository's ordinary
  Rust checks are green.

## Notes

The other reviewed judgement calls need no integration work. Exact-prefix file
directives outside fences are either consumed in the first body line or reported;
indented and fenced directive-shaped lines are deliberately prose under the same
safe-direction rule as markers. The duplicate-order message compensates for its
first-unit coordinate by naming the order, both files, the top-of-body directive,
and why the coordinate points one line later. The semantic file sequence —
framing, Grove loop, task discipline, artifact formats, grilling, then driving —
lands in the order a session needs.

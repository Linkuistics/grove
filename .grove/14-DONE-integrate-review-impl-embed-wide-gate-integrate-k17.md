# embed-wide-gate-integrate-k17

**Integrates:** `embed-wide-gate-review-k16`

## Goal

Apply the one material finding from `embed-wide-gate-review-k16`: make the
whole-embed gate reject a procedural deferral cycle even when a triggering unit
reaches that cycle.

## Context

### Finding 1 — a rooted procedural cycle passes the gate

**Location:** `src/methodology/whole_embed.rs:229`

`check_reachability` seeds its worklist from triggering units' targets, marks
each procedural id reached, and suppresses a repeated id at lines 234–237. For
this validly parsed graph:

```text
trigger (triggering) -> alpha (procedural) -> beta (procedural) -> alpha
```

both procedures enter `reached`, the revisit is discarded, and the gate returns
`Ok(())`. The existing disconnected-ring fixture proves only that an *unrooted*
cycle is unreachable. It does not prove the design's broader claim that
reachability “disposes of cycles without a rule about them.”

This is not a theoretical graph shape: the real classification pass can attach
a trigger to an accidentally cyclic procedural chain. A session following the
`defers=` markers is then directed from `alpha` to `beta` and back to `alpha`,
with no terminal procedural body and no specified visited-id rule. The build
gate therefore accepts a malformed methodology the gate claims to exclude.

**Repair shape:** after id/class resolution, add an explicit cycle check over
procedural-to-procedural `defers=` edges. Reject both a rooted self-cycle and a
rooted multi-unit cycle, locating the error at the edge that closes the cycle
and naming the cycle's ids. Keep reachability as the separate orphan check.
Reconcile the spec, ADR, architecture, module commentary and changelog wherever
they currently say reachability alone disposes of cycles.

## Done when

- A triggering unit pointing into a procedural self-cycle fails the shared
  whole-embed check.
- A triggering unit pointing into a multi-unit procedural cycle fails too.
- The existing well-formed cross-file chain and unreachable-ring cases retain
  their intended outcomes.
- Diagnostics name an actionable location and the cyclic ids.
- The durable design records describe the cycle invariant the implementation
  actually enforces.
- Post-fix tests, formatting and linting are green.

## Notes

The other six stated doubts were dismissed in review:

- Trigger roots are equivalent to “some kind's mandate” for the structural
  reachability half because every triggering scope admits at least one kind; the
  successor composer has its own exact per-kind invariant.
- The synthetic fixtures assemble files in the same sorted-path/file-order shape
  as both real callers, and cover the other plausible whole-embed failures. The
  rooted-cycle omission above is the material gap.
- Rechecking the linked embed in `methodology::units` observes the binary's own
  artifact and returns an internal-consistency error; it is not the opaque
  launch-environment proxy that `one-build-owns-a-session` refuses to gate on.
- `methodology::markdown_files` is a genuine real-embed test seam: production's
  `units` uses the same traversal, while an integration test otherwise needs a
  second embed or a second corpus walk. Its narrower whole-file result also
  preserves preamble and cross-unit text for the instructed-verb scan.
- Provisioning extracts the same embedded tree that `markdown_files` walks; both
  scans select every recursive `.md` file and ignore the non-markdown licence
  payload, so the relocation did not narrow the corpus.
- Moving the identity-flag assertion with both of its operands and deleting the
  now-redundant filesystem collector lost no claim. The unprefixed secondary
  location in diagnostics is cosmetic: the primary `content/...:line:offset`
  remains openable and the secondary still names the embed-relative unit site.

# routed-leaf-diagnostic-k41

**Kind:** impl

## Goal

Make the loop driver's per-launch diagnostic name **the leaf it routed on**, not
just the harness and model it resolved:

```
grove: launching claude (model: opus)                          # today
grove: launching claude (model: opus) — session-leaf-binding-k28 (design)
```

## Context

Externalised from **session-leaf-binding-k28**, which decided *not* to bind the
session to the leaf the driver routed on (ADR *model-per-task-kind*: the peek is
a forecast, not a reservation). That decision leaves exactly one part of the
original complaint standing: when the driver's forecast and the session's own
`pick` disagree, **nothing on screen says so**. The launch line is true about the
launch — `claude`, `opus` — and says nothing about the work, so a session running
a leaf routed for a different kind is indistinguishable from one that is not.

k28 measured the divergence rather than assuming it: with a `leaf-insert` landing
inside the launch window (≥8s, essentially all harness boot), the driver launched
`claude/opus` for a `design` leaf while the session worked a `review-impl` leaf
that `GROVE_REVIEW_HARNESS=codex` routes to codex. This line is what makes that
visible in the scrollback — without a gate, an env export, or any state outside
the tree, all three of which k28 rejected.

**It also stands on its own**, independently of divergence: the root brief's goal
is a grove legible from outside the session, and *what it is working on* is
currently absent from the driver's own output. The tree viewer plugin renders the
live leaf *now*; the driver's scrollback is the only record of what each session
in a loop was on.

## Done when

- The launch line names the routed leaf and its kind, degrading to today's line
  when the path cannot be resolved.
- `picked_leaf` (`src/loop_driver.rs:1006`) is reused rather than reimplemented —
  it already walks in-process and already degrades to `None`. Note it is
  currently reached only from `readiness()` (the `--no-launch` dry run), so this
  puts it on the live launch path for the first time; check its doc comment still
  tells the truth afterwards.
- The pairing is honest about what it is. The path comes from an **in-process
  walk** and the kind from the **peek subprocess**, so the two could in principle
  disagree — microseconds apart, and this is *reporting, not routing*. Precedent:
  `readiness()` already pairs them exactly this way (`Next::Leaf`).
- Naming follows task-tree-scheme §5 — the stable `<slug>-k<key>` handle, not the
  position or the path.

## Notes

**Not a chain, deliberately.** Per *compose-task-chains-k29*: chain a
load-bearing artifact, subagent a one-file change. This is a diagnostic string in
one file — a mid-session subagent review is the right weight, not `review-impl` +
`integrate-review-impl`.

Costs a second in-process directory walk per launch, against a `grove-llm kind`
peek measured at ~0–30ms. Cheap, but it *is* a third pick per iteration, which
**session-leaf-binding-k28** explicitly scoped out as not worth de-duplicating —
consistent, not contradictory: k28 declined to *remove* picks, and this adds one
for reporting, which is the same justification `picked_leaf` already carries.

The counter-argument, for the session that does this to weigh rather than
inherit: the line gets longer, and `grove do --no-launch` plus the tree viewer
plugin already name the live leaf by other routes. If it lands, CHANGELOG under
the release it ships in — the driver's output is something the binary carries.

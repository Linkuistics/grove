# leaf-harness-k15

**Kind:** work

## Goal

Let a single leaf name its own harness with a `**Harness:**` line beside
`**Kind:**`, and have the loop launch it there. This is the mechanism the vendor
pair needs — two `research` leaves on two vendors — and it is the *only* thing
per-kind routing cannot express.

## Context

- `src/tree_read.rs` — `read_kind` is the model to follow: find the marker line,
  take the first whitespace token, tolerate trailing commentary. A
  `read_harness` sits beside it.
- `src/llm_cli.rs` — the `kind` verb. Decide whether harness is a second verb or
  the same verb gains a second output line; see **Notes**.
- `src/loop_driver.rs` — `resolve_launch` decides `(harness, model, rerouted)`.
  The leaf declaration slots in **above** the per-kind override and below
  nothing: **leaf beats kind beats family beats stamp** (specific beats general
  on the kind axis, per the spec's *Routing* — an earlier draft of this brief had
  family and kind the wrong way round).
- `preflight_check` cannot know which harnesses a tree's leaves declare without
  walking the tree. Decide whether it walks live leaves or defers to
  launch-time; both are defensible, and the choice is this leaf's to make.
- `src/tree_grow.rs` — the leaf template. A `--harness` flag on `leaf-add` /
  `leaf-insert` is optional and probably worth it, since the declaration is
  written by a planning session growing a pair.

## Done when

- A leaf carrying `**Harness:** codex` launches on codex regardless of stamp or
  per-kind policy.
- A leaf with **no** `**Harness:**` line behaves exactly as today — this is the
  overwhelmingly common case and must stay a zero-cost path.
- An **unrecognised** harness name on a leaf **refuses to launch**, naming the
  file and listing the known harnesses. It does *not* degrade. See Notes.
- `rerouted` is computed against the stamp as it is today, so a leaf-declared
  harness that differs from the stamp gets no unscoped model var and no global
  binary override.
- Preflight behaviour is decided, implemented, and its rationale recorded.

## Notes

**Refuse, do not degrade — and this is a deliberate departure from how `kind` is
read.** `read_kind` degrades because a wrong *discipline label* costs a warning,
and jamming the unattended loop is worse. A wrong *harness* is different in kind:
it runs the leaf on a vendor the tree explicitly said not to, which is exactly
the silent misroute `resolve_launch` already bails on when the kind peek fails
while an override is configured. The precedent is in the code and its comment
argues the case; follow it. Constraint 5 ("grove guides, it does not gate") is
about grove refusing to proceed on *process* grounds, not about executing a
declaration it cannot honour.

**One peek or two?** By the time this leaf runs, `required-model-vars-k18` has
already made the `grove-llm kind` peek **unconditional** on the `continue` path,
so the old worry here — that a leaf declaration is not gated on env while the
peek is — has dissolved. The remaining question is narrower and purely about
subprocess count: prefer extending the existing peek to return both facts over
adding a second subprocess. The shape of that output is this leaf's call, but it
must stay parseable by the existing consumer or change both ends together.

Only two leaves in a research pair ever carry this line — the second producer
and the combine step. Do not build a general per-leaf configuration surface off
the back of it.

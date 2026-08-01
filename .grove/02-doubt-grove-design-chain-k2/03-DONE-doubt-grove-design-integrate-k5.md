# doubt-grove-design-integrate-k5

**Kind:** integrate-review-design

## Goal

Verify and integrate the design-review findings so the resulting spec and
decision set are ready to drive implementation.

## Context

Read the design artifact from `doubt-grove-design-k3` and findings in
`doubt-grove-design-review-k4`. Classify each finding as contract misread,
actionable issue, accepted visible trade-off, or noise before editing.

## Done when

- Every review finding is classified with evidence and each real issue is fixed.
- The spec/ADR set remains a minimum coherent current-state set.
- Root-brief pointers or terminology are reconciled if the design sharpened
  them.
- Implementation can proceed without inventing promotion, routing, or test
  semantics.

## Notes

If a finding changes a human-owned requirement rather than clarifying it, stop
and ask rather than silently rewriting the baseline.

## Finding dispositions

Each finding from `doubt-grove-design-review-k4` was re-read against the root
brief, the design artifact, and the named implementation seams before editing.

| Finding | Classification | Evidence and integration |
|---|---|---|
| F1 interrupted promotion inverts pick order | **Actionable issue** | `tree_read::pick_in` descends the visible node before its same-position producer, while `tree_grow::roll_back` is safe only before the producer enters the new directory. The spec now stages under reserved `PROMOTING-<final-node-name>`, makes every reader/mutator fail closed, moves the producer inside, and lands the complete node with one rename. |
| F2 canonical methodology contradicts the boundary | **Actionable issue** | `content/driving.md` still contains the three-round loop, the size/vendor-only escalation trigger, and the no-retrofit claim; the doubt skill still verifies per artifact and re-loops. The spec's Canonical surfaces and contradiction-shaped test seam now name the exact files and obsolete rules implementation must replace. |
| F3 no picked-session discriminator | **Actionable issue** | Checkout and environment state cannot prove what procedure the current session follows. The ADR/spec/root brief now define the current session's own Bootstrap, `pick`, and adoption of that leaf as the discriminator, explicitly excluding `.grove/` and inherited context. |
| F4 inherited receipt treated as proof | **Actionable issue** | `launch::scrub_loop_control_env` and its ambient-authority rationale prove non-addressed environment inheritance. The design replaces two loose variables with one scrubbed worktree-scoped JSON value, accepts it only for that worktree's current pick, and requires nested/meta-grove tests. |
| F5 retirement cross-write has no failure contract | **Actionable issue** | `tree_lifecycle::leaf_retire` currently performs one filename rename and `DONE infix` promises the retired file stays byte-identical. The design preserves that property, limits lookup to exactly one declaring sibling, writes atomically, and makes every receipt failure diagnostic-only before the normal `DONE` rename. |
| F6 legacy/hand-cut chain is undetectable | **Actionable issue** | Relationship metadata cannot exist in old shapes, but immediate-parent `BRIEF.md` absence is already Grove's structural discriminator for composition nodes. Promotion now refuses such a parent after recognising its own idempotent completed shape. |
| F7 promotion uses the degrading kind reader | **Actionable issue** | `tree_read::read_kind` intentionally maps malformed input to `impl`. Promotion now requires a non-degrading exact declaration, accepting the seventeen labels and supported `work` alias but rejecting missing, empty, and unknown tokens. |
| F8 default configuration warns every review | **Accepted visible trade-off** | The root brief explicitly requires a warning unless both harness and model differ, so suppressing the one-harness case would change a human-owned requirement. The spec records the expected warning, bounds it to one compact notice per spawn, and does not redesign the requirement. |
| F9 a TUI erases the warning | **Actionable issue** | `loop_driver::launch_session` writes a diagnostic and immediately spawns the full-screen harness. The same pure comparison result now renders to stderr and a notice prepended to the launched review prompt, preserving it in the transcript. |
| F10 test seams miss interruption and contradiction | **Actionable issue** | Reported fault injection cannot model an unreported kill, and positive row coverage can coexist with obsolete prose. The test seams now interrupt after every mutation and assert fail-closed/recovery behavior, plus assert the old rules are absent and the new rules present. |
| F11 reviewer count and integration placement are ambiguous | **Actionable issue** | The doubt skill defines diverse-lens as N subagents and Grove ordering is structural contiguity. The ADR/spec/glossary now count each fresh context and place substantial integration redesign as a producer review chain inside the owning chain node. |
| F12 surfaces untraced and ADR set incomplete | **Actionable issue** | The root brief names five current-state surfaces, while historical producer routing cannot be reconstructed after configuration changes and a fail-closed reserved prefix changes the tree grammar. The spec now lists every concrete surface; `review-target-receipts` and `promotion-transactions-fail-closed` record those independent decisions. |

## Doubt-cycle stop

The one permitted fresh-context reviewer of the revised contract found seven
substantive follow-up issues spanning concurrent promotion, power-loss wording,
receipt freshness and leaf binding, nullable defaults, VCS landing, and
unavailable warning identity. Reconciliation classified concurrency, stale
receipt survival, leaf binding, nullable-model identity, VCS landing, and
unavailable producer identity as **actionable issues**. The durability finding
is a **contract misread caused by unclear wording**: the spec disclaims one
power-loss-atomic replacement syscall but still uses the broader term
"crash-consistent," so the next producer must either narrow the promise to
process interruption or earn an explicit durability protocol.

Incorporating those fixes would create the second review need this design itself
assigns to Grove. They are therefore externalised as the reviewed
`doubt-grove-design-hardening-k11` →
`doubt-grove-design-hardening-review-k12` →
`doubt-grove-design-hardening-integrate-k13` chain inside the current chain
node; this leaf does not start a competing second in-session review cycle.

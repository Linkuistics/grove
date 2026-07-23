# The finish cycle is an in-session, state-checked LLM step, not Rust automation

When `grove-llm pick` reports no live leaves, the grove is finished by a terminal,
whole-grove sequence the in-session LLM carries out — no Rust automation, no
progress-marker file. The steps are:

1. **Promote** durable artifacts from the briefs to ADRs / docs / glossary.
2. **Delete `.grove/`** in one focused commit.
3. **Signal** the loop to stop with `grove-llm complete --done` (the handoff of the
   *self-driving-loop* decision), which distinguishes a clean finish from a crash
   or interrupt.

Nothing after: integrating the grove's branch and tearing down the working tree are
not grove workflow — both belong to the user's own tooling
(*user-owned-worktrees*). Whoever integrates does so after step 2, so the
integrated history never carries `.grove/`.

## Why in-session, not Rust

Two forces point the same way. grove's spine wants the methodology to live in
markdown a reader can follow by hand ("read, don't run"; "walk-away-able"), not in a
binary that must succeed for work to proceed — a finish cycle expressed as skill
prose stays legible and editable without a release. And the cycle's hard part is
*judgement* — what deserves promotion, whether the tree really is done — which an
interactive agent does well and a fixed Rust state machine does badly. Encoding it
in Rust would buy determinism the operation does not need while forfeiting the
recovery flexibility it does. The trade-off (no machine-enforced atomicity) is
acceptable because every step is individually recoverable in the VCS.

## Resume is state-checked, never a marker file

A finish-progress file is forbidden ("artifacts, not state"), so resume is derived
entirely from inspectable state. `grove do` into a half-finished grove resumes from
the first incomplete step: if `.grove/` is already gone (`grove-llm pick` errors
"grove root not found"), promotion and deletion are done — report "already
finished" and stop.

## Interactive UX and headless behaviour

The teardown (steps 2–3) is gated by a **single** confirmation, taken after
promotion (step 1) has produced its reviewable working-tree edits. Per-step
confirmation is rejected as a wizard anti-pattern. The single gate is safe because
nothing in the cycle is irreversible in the VCS — the `.grove/` deletion is a
commit.
The real risk is finishing the *wrong* grove, which one clear plan-and-confirm
addresses.

Headless behaviour needs no mode detection: the LLM **proposes the teardown and
waits** for explicit human confirmation, never running steps 2–3 unprompted. A
headless run with no human present ends the turn with the plan as output and runs
nothing destructive.

## Where it lives

The operational instructions are the **Finish** step of the grove skill's loop prose,
not a launcher prompt — per the "launcher prompts stay small" convention, and the
empty-pick→finish trigger is noted in the loop's Pick step.

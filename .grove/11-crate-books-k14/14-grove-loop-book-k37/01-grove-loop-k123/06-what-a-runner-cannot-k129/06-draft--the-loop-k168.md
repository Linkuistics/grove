# the-loop-k168

## Goal

Draft chapter 20 of the `grove-loop` book, *The loop* —
`crates/grove-loop/src/loop_driver.rs` lines 1–615, one whole root — to a valid
slice through `four-things-a-runner-cannot-choose`, which resolves the **last
deferred block in the book**.

## Context

- **The rule: all of the spawning, watching and escalating is `keyed-launch`'s;
  what stays here is the four things a loop has to choose and a runner cannot** —
  which directory the channel is allocated in, which variable publishes it, which
  variables are scrubbed, and how long the two graces are. `run` and `drive`,
  `session_prompt`, `launch_configured_session`,
  `complete_post_reap_epoch_handoff`, the `ESCALATION` constant, `reset_terminal`,
  `ignore_interrupts`, `scrub_loop_control_env`, and the two inline tests that
  hold the ordering:
  `an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it` and
  `signal_interpretation_cannot_run_before_epoch_invalidation_succeeds`.
- The chapter carries *restart ≡ continuation* — the loop body holds zero state
  and re-derives position from the tree — and the shell sketch the header keeps,
  which is still the whole loop **because a boundary is not a step**.
- **The prose obligation splits inside this one chapter.** The production half is
  546 lines at 51% and takes *do not restate*; the 69-line inline test block at
  8% takes *supply the claim* — for each of the two tests, the property it
  establishes and what would have to be true for it to pass while the property
  was broken. Both instructions apply to one page.
- **This chapter refutes chapter 18's stale sentence, from inside the same
  corpus.** `templates.load(&delta_roots)` is called at lines 241 and 260 — once
  before `transition_to_current`, so the just-in-time presence rule for the finish
  leaf is asked against the document as it stood *before* the tree was mutated,
  and once after the leaf is selected, so the launch expands the selected kind's
  template from the document as it stands. Two reads, two reasons. Show it;
  never repeat the *once* as true and never silently correct it.
- **The mutation form is a panic that says nothing.** `panic!("MUTANT")` replacing
  the whole macro call — a message-preserving panic is invisible to the
  out-of-process `grove-llm` suite, which asserts on stderr substrings. This
  chapter owns an inline test block outside `no-word-for-k127`, so the correction
  applies here directly. The full harness is in `grove-loop-k123`'s brief.
- Declares glossary anchors `stated-vcs` and `loop-control-channel`; keeps
  `the-launched-child-is-a-job` and `one-live-driver-per-working-tree`, named in
  prose and never linked.
- Carries two `docs/ARCHITECTURE.md` residue markers — *the watch and the
  escalation* (joint with `keyed-launch`, whose book is written) and *the scrub
  inside the seam and the loop's complementary list* — by subject, not by line.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-things-a-runner-cannot-choose --check all` is valid at 10,533 resolved
  lines, **0 deferred**, still `final=false` because chapter 21 does not exist.
- The `loop-driver` ownership row reads `resolved` and **every** ownership row in
  the ledger reads `resolved`; the chapter-1 cast row owned by
  `four-things-a-runner-cannot-choose` reads `explained`, closing the last row in
  the early-use ledger.
- Contents and navigation updated; `scripts/check.sh` red on `book-check` alone,
  and this file says so.

## Notes

**This is the last child of `what-a-runner-cannot-k129`, not the last of
`grove-loop-k123`.** `what-could-not-move-k130` follows it with chapter 21 and
takes the book to `--final`; the `copy-edit` leaf is that leaf's last act, not
this one's. Cut nothing here beyond what the work needs.

## Decisions (running log)

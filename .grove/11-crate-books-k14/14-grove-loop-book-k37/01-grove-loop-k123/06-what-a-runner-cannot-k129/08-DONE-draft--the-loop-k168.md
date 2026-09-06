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

1. **The structure brief precondition is met.**
   `docs/specs/grove-loop-book-structure.md` states all three things `grove-draft`
   requires, quotably: *Audience and intended outcome* (a reader who knows Rust and
   jj and has driven a grove; the what-could-not-move test), *Chapter sequence* plus
   *Concept and seam responsibilities* (the ordered plan, one subsection per page),
   and *What each chapter's prose owes* / *The spine* / *What the book deliberately
   does not cover* (emphasis and exclusions). Chapter 20's own subsection at line 553
   carries the rule, the item list and the two named tests.

2. **The block holds exactly two `#[test]` functions, and the structure brief names
   exactly those two.** Counted against the bytes rather than against any prose list
   of them, under the discipline `structure-brief-test-list-k172` left: `grep -n
   '#\[test\]'` returns lines 557 and 593, and the two names are
   `an_epoch_handoff_failure_preserves_the_launch_failure_that_preceded_it` and
   `signal_interpretation_cannot_run_before_epoch_invalidation_succeeds`. This is the
   first chapter in Part V whose brief test list is right in both directions, and the
   fifth structure-brief count checked; no correction is owed.

3. **The prose split reproduces the brief's figures exactly, so no measurement
   disagreement is recorded here.** Counting lines whose first non-space characters
   are `//`: production 281 of 546 = 51.5%, the inline test block 6 of 69 = 8.7%,
   the root whole 287 of 615 = 46.7%. The brief says 51% and 8%. Chapter 15's
   one-point disagreement over `verbs.rs` and `driver.rs` does not recur, and
   chapter 19's 46.7% whole-root figure is confirmed.

4. **Both of the book's instrument blind spots are present on this one root, and
   the reading was taken rather than inherited.** `loop_driver.rs` has 204 `///`
   lines, zero `//!`, **83 plain `//`** (the whole 54-line header) and an inline
   `#[cfg(test)]` module — so it has chapter 16's marker blind spot *and* chapter
   17's cfg blind spot, where `prompt.rs` and `session_config.rs` had neither.
   Measured with three controls in a workspace copy: a broken intra-doc link planted
   in a production `///` docblock takes the crate from 30 warnings to **31**; the
   identical construct inside the `#[cfg(test)]` module leaves it at **30**; the
   identical construct inside the plain `//` header leaves it at **30**. So the clean
   run — 30 warnings, none naming this file, all eight of its intra-doc links
   resolving — is evidence about its 204 `///` lines and about nothing else.
   Chapter 19's forecast that chapter 20 was not the exception `prompt.rs` was is
   upheld.

5. **A citation that resolves to nothing, and it is a third form.** Line 86 cites
   `(driver-side-kill)` for the claim that `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID`
   are the retired pre-watcher handles. Enumerating the block's five bare
   parenthesised citations and resolving each leaves that one: it is **not** one of
   `docs/ARCHITECTURE.md`'s twenty-four anchors, not an ADR slug, not an id in
   `plugins/grove/conformance/rules.tsv`, and it occurs **exactly once in the whole
   repository** — in the comment itself. Positive control: the same grep form returns
   twenty sites for `self-driving-loop`. The claim is true and only the address is
   wrong, so it is adjudicated on the page under the `paths-k142` precedent, and a
   better anchor exists — `self-driving-loop` argues the driver-side kill and the
   sandbox ground in the same section. This is chapter 16's `probe_lease_holder`
   procedure and chapter 19's `skill-stated-vcs-is-definitive` class, met a third
   time. `loop-driver-kill-anchor-k176` holds the fix.

6. **`(constraint 6, walk-away-able)` is a fourth citation form and it resolves.**
   Not an architecture anchor — it names the grove spine's sixth constraint, and
   `plugins/grove/skills/grove/SKILL.md` numbers *Walk-away-able* sixth. Reading the
   link *form* rather than the warning list is what separates it from the one above,
   which is chapter 18's seventh-instrument-gap discipline applied to a form that
   gap did not name.

7. **The promoted mutation harness is short by a package for this block, and the
   error direction hides observers.** Every child from `no-word-for-k127` on has run
   `cargo test --no-fail-fast -p grove-loop -p grove-llm`. `loop_driver.rs`'s
   production half is observed almost entirely from **`crates/grove/tests/`** —
   `loop_driver.rs` (11 tests), `lifecycle_cutover.rs` (17) and `env_hygiene.rs` (4)
   — which that command never runs. Run unchanged it would have read zeros across the
   block and reported it held by nothing. The control for this chapter is therefore
   `-p grove-loop -p grove-llm -p grove`: **626 tests, 616 passed, 10 failed**, the
   ten being `crates/grove-loop/tests/prompt.rs`'s ten that reach `compose` and die
   in the `workspace()` fixture because the copy is not a jj repository. That is the
   same 626 chapter 10 reported under its wider copy, and the corrected clean ten
   `the-core-k167` established by copying `.claude-plugin/`.

8. **`docs/specs/module-decomposition.md`'s decision 9 has drifted from the
   signature it states as written source.** The comment at line 169 cites decision 9
   for the loop's shape, and that citation holds. But the record's own code block
   declares `pub fn run(workspace: &Workspace, lease: DriverLease, templates:
   &Templates)` where the shipped parameter is `&TemplateSource` — `Templates` is
   `keyed-launch`'s type, and `TemplateSource` appears nowhere in the record — and
   `pub enum LoopOutcome { Finished, Stopped }` where the shipped enum has **three**
   variants, `Interrupted(i32)` being absent from the record entirely. This is a
   defect in a specification rather than in the frozen corpus, so it is not the
   book's to adjudicate on the page beyond one clause; `decision-nine-loop-signature-k177`
   holds it.

9. **A false control that cancelled itself, caught only by checking the patch.**
   The first attempt to prove the two inline tests can fail rewrote
   `INVALIDATION_CONTEXT` with `sed` — and the literal occurs **three** times in
   the block, once in the constant and once inside each test's `contains`. All
   three moved together, the tests stayed green, and the reading was *245 passed*,
   which is indistinguishable from a real negative. Mutating **only line 446**
   turns exactly two tests red, and they are exactly this block's two. The lesson
   is the briefs' *a control that has never been seen to fail is not a control*,
   met in the form where the mutation changes the observer along with the subject.
   It also settles a fact about the tests: they assert against a **literal copy**
   of the constant rather than against the symbol, so they pin the text — the
   inverse of chapter 17's `IDENTITY_RETRY_LIMIT`, which moves with its constant
   and pins nothing.

10. **Neither inline test pins the property its name promises, and between them
    the two properties are covered once each.** Measured on a workspace copy with
    the control above. Swapping the `(Err, Err)` arm so the *launch* error becomes
    the returned error's identity leaves all 245 lib tests green — the test reads
    the chain with `{:#}`, which flattens context and cause, so the asymmetry is
    exactly what the rendering erases. Replacing the `(Ok, Err)` arm's wrapped
    error with a bare `anyhow!(INVALIDATION_CONTEXT)` also leaves 245 green — that
    test asserts on `to_string()`, pinning the outermost context and the ordering,
    never the preservation of the cause. So test 1 pins preservation and not
    identity; test 2 pins identity and not preservation.

11. **The two configuration reads are two by argument and one by measurement.**
    Three mutations bracket them against the 277-test control: deleting the
    presence check turns exactly one test red
    (`a_finish_leaf_is_not_written_when_no_finish_template_resolves`); asking the
    rule against a **post**-transition load instead turns none red; letting the
    launch reuse the **pre**-transition load turns none red. So *that* the check
    happens is pinned and *when* is not, in either direction. The reason is
    structural: `tree_lifecycle.rs`, where both intervening operations do their
    work, contains **no `.kdl` reference at all**, so nothing the loop does between
    the two calls can change what the second reads. This is k157/k158's
    reachability lesson — a zero here is *unreachable through this path*, not
    *untested by oversight*. Reading the arms also shows that on every iteration
    with a live leaf, `pre_transition_config` is loaded and never used, since only
    the `Sought::Nothing` arm reads it. Chapter 18's adjudication is unaffected and
    sharpened: the count is still two.

12. **The scrub list's live member is unobserved and its two retired members are
    pinned — the opposite of the intuitive reading.** Replacing all three names and
    replacing only the two retired ones give the *identical* one-test failing set,
    so the isolating pair was run: dropping only `GROVE_SIGNAL_FILE` turns **0**
    red, dropping only `GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` turns **1** red
    (`bare_grove_launches_the_selected_filename_kind_with_one_mandate_argument`,
    through two assertions each reading back `<unset>`). The live member's
    membership is a **guard rather than a gap**: `channel_var` grants the variable
    back on the very next field, so scrubbing it there is a no-op by construction,
    and the only spawn that would reveal the difference is `reset_terminal`'s
    `stty`, which nothing observes. And
    `the_shared_scrub_list_covers_the_loop_control_channel` — the test whose name
    most suggests it covers this — reads `support::grove_env_names()` in
    `testing/support.rs` and never touches `LOOP_CONTROL_ENV`, so it passes under
    every mutation of the constant. Chapter 17's *read the assertions, not the
    name*, one level out: the name points at the right subject in the wrong file.

13. **The four handoff arms attribute cleanly, and the inline tests are the sole
    observer of exactly one of them.** `(Err, Ok)` → one test,
    `spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf`.
    `(Ok, Err)` → two, the second inline test **and**
    `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`.
    `(Err, Err)` → one, the first inline test and nothing else. So the 69-line
    block is not redundant with the end-to-end suite: one arm is held by it alone.

14. **A hang is not a failure and is reported separately.** Three mutants —
    renaming `CHANNEL_VAR`, deleting the epoch activation, and allocating the
    channel outside the lease's control directory — leave one or two tests that
    never finish, because they wait on a completion signal that cannot arrive. The
    harness records those as *never reported* rather than folding them into the
    newly-failing count, since a test that never ran is precisely what a broken
    instrument also produces. The instrument's positive controls are the seven
    mutants that **do** turn tests red, so the zeros elsewhere are readings rather
    than silence.

15. **`Done when` holds, and `scripts/check.sh` is red on `book-check` alone.**
    `book-check --repo . --book docs/walkthroughs/grove-loop --through
    four-things-a-runner-cannot-choose --check all` reports **valid: 13 files,
    10,533 resolved lines, 0 deferred lines, final=false**. All thirty-nine
    ownership rows read `resolved` and all fifty early-use rows read `explained` —
    the `run` / `LoopOutcome` cast row closing the last one in the ledger. The
    gate reports `check: FAILED — 1 of 8`, and the failure is `book-check`,
    entirely and only because chapter 21 does not exist: one `M101` for the
    missing `21-what-could-not-move.md` and the two `M103`s that follow from it,
    the README's contents and this page's absent `Next`. The other five books are
    `final=true` and green, so nothing here disturbed them.

16. **One test failure was environmental, and the re-run is what established
    that** — not an argument about why it should be.
    `a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver`
    failed, and `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`
    wedged, in a gate run taken while two scratch workspaces and another project's
    `cargo test` were competing for the machine — load average 8, and cargo's own
    runner logged *has been running for over 60 seconds* against **every** test in
    that binary. `ps` also shows orphaned `configured-command.sh` fake-session
    children from earlier runs, some days old. Re-run on a quieter machine the
    whole binary is 11 passed, 0 failed, and the final gate reports `✓ cargo test`.
    This is the briefs' *a mutant naming a `driver_lease.rs` or `prompt.rs` test
    must be re-run* generalised: **`crates/grove/tests/loop_driver.rs` wedges under
    load too**, and a timeout there reads exactly like a failure. No source
    changed in this session, so a docs-only diff could not have caused it.

17. **The first two gate runs were discarded rather than reported.** The first was
    started before the concept-index edits were finished, so it read a subject that
    moved under it; the second was killed while wedged on the hung test above. Only
    the third was taken with every edit complete, and a `shasum` of all
    twenty-three book files before and after that run confirms **no subject changed
    while it was reading**. A run whose input moved is not a measurement of the
    thing it reports on, however small the move.

## Notes on what this leaf did not cut

**No `copy-edit` leaf.** The document node `grove-loop-book-k37` has one child,
`01-grove-loop-k123`, and that draft node still holds `what-could-not-move-k130`
live. The chain's next stage is cut by the draft's **last** child, which is k130
and not this leaf — as this file's own Notes and `grove-loop-k123`'s `Done when`
both state. Two `impl` leaves were cut, and only because defects found while
drafting owe them.

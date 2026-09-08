# ch15-16-configured-command-k212

## Goal

Re-derive the three mutation readings that chapters 15 and 16 share, under the
node brief's control, and make the four `558` citations across the two pages —
and every sentence resting on them — say what the suite actually shows. Chapter
16 cites chapter 15's measurement rather than taking its own, which is why the
two pages travel in one leaf: a number that moves must move on both.

## Context

- **Four citations, three mutants, two pages.** `15-the-verbs.md` at lines 805
  (the `or_else` fallback deleted — *nothing newly fails*), 820 (the emptiness
  filter deleted — *exactly three tests red*) and 1219 (`complete.rs`'s `.trim()`
  removed — *nothing newly red*); `16-the-lease.md` at 1966, which restates
  chapter 15's first two readings as one sentence to justify calling
  `ambient_signal_path`'s arms *unreachable under the harness* rather than
  untested. Chapter 16 takes no run of its own, so it inherits whatever chapter
  15's re-derivation shows — including the shape of the sentence, not only its
  number (`a-later-leaf-may-own-the-cross-chapter-tally` in reverse: here the
  earlier page owns the datum).
- **These pages state the control as `558 tests, 547 passed, 11 failed`.** The
  re-derived control is **560 tests, 549 passed, 11 failed** — so both the total
  and the passing count move, and every one of the four citations spells out all
  three numbers or two of them. Chapters 11–14 each carried only the total.
- **The eleven have two causes, not one.** Line 805 says *the eleven being
  `crates/grove-loop/tests/prompt.rs` in a copy that is not a jj repository*: the
  run's own stderr shows **ten** panicking on `Refusal(NotAWorkspace { … })` and
  `the_namespace_is_the_shipped_plugin_entrys_declared_name` on
  `.claude-plugin/marketplace.json` being `NotFound`. Chapters 11, 12, 13 and 14
  all name both causes now; these two pages are the last that do not
  (`joint-justifications-split`).
- **Two of the three readings are zeros, and a zero means untested *or* dead**
  (`mutation-cannot-see-unreachable`). Both pages already argue the split — the
  `or_else` arm is unreachable under `cargo test` because `.cargo/config.toml`
  force-clears `GROVE_SIGNAL_FILE`, and the `.trim()` is dead because
  `Channel::read` already `trim_end`s and nothing in the workspace writes leading
  whitespace. So each zero owes a re-measurement rather than a re-reading, and
  each argument owes a check that it still describes the code.
- **The third reading is this book's only stated positive control, and that is
  what makes the two zeros readings rather than a blind instrument.** *Deleting
  the emptiness filter turns exactly three tests red* —
  `an_empty_signal_environment_is_no_loop_context`,
  `no_channel_at_all_is_answered_rather_than_refused` and
  `the_channel_can_be_asked_for_before_it_is_written`. That is a cited test list,
  so it is a measurement and not a citation (`a-cited-test-list-is-a-measurement`);
  re-derive the members, not only the three.
- **The relink step is not obviously owed here and should still be taken.** These
  three mutants sit in `crates/grove-loop/src/verbs.rs` and
  `crates/grove-loop/src/complete.rs`, and their named observers are in
  `crates/grove-llm/tests/complete.rs` — a binary `cargo test` rebuilds itself.
  But `ch14-finishing-k211` measured why the step matters and it costs one build:
  `testing/support.rs:434`'s `workspace_binary` rebuilds `target/<profile>/grove`
  **only if the file does not exist**, and `cargo test -p grove-loop -p grove-llm`
  never rebuilds the `grove` package, so any observer that shells out to the
  driver binary stays green without `cargo build -p grove --bins` **after** the
  edit. Take it, and treat a `driver_lease.rs` or `removed_surface.rs` name in a
  newly-failing set as the signal that it mattered.

## Done when

- All three mutants are run against the node brief's control, each confirmed to
  have reported 560, each read as a `comm -13` over **binary-qualified** failure
  names, and any mutant whose newly-failing set names a `driver_lease.rs` or
  `prompt.rs` test re-run before it is believed.
- The four `558` citations each state the re-derived total **and** passing count,
  and the two `all N green` shapes are checked against the eleven pre-existing
  failures before being renumbered — *all 560 tests green* was never true of this
  copy, and chapters 12, 13 and 14 settled the replacement wording as *the
  mutation reddens nothing against the control*.
- Line 805's single cause for eleven failures states both causes.
- The three-test positive control at line 820 is re-derived member by member, and
  chapter 16's line 1966 restates whatever chapter 15 now says — number, cause
  and shape.
- Every zero's argument is checked against the code as it now stands:
  `.cargo/config.toml`'s force-clear, `Channel::read`'s `trim_end`, and that
  nothing in the workspace writes a token with leading whitespace.
- This leaf's last act cuts chapter 17's (`--kind impl`, slug `ch17-the-epoch`),
  per the node brief's decomposition — eight rungs, one citation at
  `17-the-epoch.md:994`, which also states `558 tests, 547 passing`.
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

**The control, reproduced four times now.** The node brief's *Pointers* carries
the copy recipe and it has reproduced exactly at `ch11-a-grove-begins-k208`,
`ch12-leaf-to-node-k209`, `ch13-outcomes-k210` and `ch14-finishing-k211` — 560
tests, 549 passed, 11 failed, matched set for set, about 55 seconds per mutant on
a shared `CARGO_TARGET_DIR`. Pre-check every mutant with
`cargo check -p grove-loop --tests` before the study, which closes the
fails-to-compile trap ahead of the runs rather than after them.

**Qualify failure names by their binary.** The 560-test control carries 559
distinct bare names; the one duplicate,
`finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf`, is chapter
14's and appears in no set here — but the harness costs nothing and a merged name
reads exactly like a moved cell.

**Measure with `GROVE_SIGNAL_FILE` unset in the measuring shell.** These three
mutants are *about* that variable, and this repository is a meta-grove: the
session running the study inherits a live signal path, and `CLAUDE.md` explains
why cargo force-clears it. The scratch copy carries `.cargo/`, so the guard
travels with it; the shell should not carry the value in.

## Decisions (running log)

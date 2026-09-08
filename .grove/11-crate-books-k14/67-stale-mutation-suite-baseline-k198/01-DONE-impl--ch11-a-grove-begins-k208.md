# ch11-a-grove-begins-k208

## Goal

Re-derive `docs/walkthroughs/grove-loop/11-a-grove-begins.md`'s *What the
refusals are worth, measured* under the corrected harness, and make the five
`558` citations and every sentence that rests on that table say what the suite
actually shows.

## Context

- **This is the chapter the corrected harness falsifies hardest**, because its
  run is the one that predates `cargo build -p grove --bins`. The node brief
  carries the derivation; the short form is that its table's small counts and
  zeros are lower bounds, and the arms the `grove` binary reaches are the ones
  that move.
- **Four of the eleven runs are already taken**, against the control the node
  brief specifies, and they are the reason this child exists rather than a
  find-and-replace:

  | probe | page says | re-measured |
  |---|---:|---:|
  | row 6, `RootShape::ATree` (`tree_lifecycle.rs:457`) | 2 observers | **10**, twice, identically |
  | row 7, `RootShape::Taskless` (`:484`) | 1 observer | **1**, unchanged |
  | `root_init` entry probe | reddens 41 | **43** |
  | control | 558 / 547 / 11 | **560 / 549 / 11**, same eleven |

  Row 6's eight extra are
  `a_configured_launch_in_a_colocated_jj_tree_ignores_the_removed_environment`,
  `a_configured_launch_in_a_native_jj_tree_ignores_the_removed_environment`,
  `a_lease_replaced_under_a_running_driver_refuses_the_next_transition`,
  `a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session`,
  `a_second_driver_refuses_before_tree_access_or_launch`,
  `driver_uses_the_on_disk_worktree_while_the_configured_command_inherits_git_context`,
  `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` and
  `the_launch_fixture_still_observes_a_configuration_driven_change` — every one of
  them out-of-process, none of them either of the two tests the parent leaf was
  cut for.
- **The remaining seven arms are rows 1 to 5, 9 and 10** of the page's table, at
  `tree_lifecycle.rs` lines 346, 391, 396, 399, 405, 1028 and 1065. Each cited
  line still names its cited arm. Row 1 is a `refuse_finish_kind` call site and
  must panic on the refusal rather than on entry; row 9 is an
  `unwrap_or_else` fallback; row 10 is a bare `return Ok(())`.
- **Row 6's new observers change more than a cell.** The page argues that
  *`RootShape` is chapter 11's source and the evidence is split rather than
  exported*, counts *four arms observed, by five tests, three of the five this
  chapter's*, and names row 6 as pinned from both sides. Ten observers is a
  different sentence, and the *five tests* roll-up above it is derived from the
  same table (`uniqueness-and-count-claims-need-enumeration`).
- **Which chapter owns a newly named test is decided by `walkthrough.toml`'s
  `[[block]]` line ranges, never by the verb it exercises**
  (`a-test-belongs-to-its-line-range`). The eight new names are in `tests/`
  directories, which are evidence rather than roots, so none of them is any
  chapter's — but say that rather than assuming it.
- **Leave `ch11-measurement-prose-claims-k199` its two findings.** That leaf, live
  in this node's parent, owns the *"Two of the ten arms make the reading a lie"*
  sentence and the *"as a literal"* clause about `root_init`'s five call sites.
  Both are in this same section. Correcting the measurement does not require
  touching either, and absorbing them would leave k199 with nothing to reject.

## Done when

- All eleven mutants have been run against the node brief's control, each
  confirmed to have reported 560, and any mutant whose newly-failing set names a
  `driver_lease.rs` or `prompt.rs` test re-run before it is believed.
- The table's *Observed by* column, the *four arms are observed, by five tests*
  roll-up, the *six of ten arms are held by nothing* split, and the
  `RootShape`-is-split argument all state what the eleven runs show.
- The five `558` citations at lines 176, 674, 1374, 1503 and 1512 each state the
  re-derived number, and the `reddens 41` at 1514 states 43.
- The k159 section further up the page — which already measures against 560 with
  **ten** failures because its copy carried the marketplace — is reconciled with
  the section below it, or the difference between the two copies is stated where a
  reader meets the second number.
- `book-check --final --check all` is green, and `bash scripts/check.sh` is no
  worse than before.

## Notes

**No source change.** If a mutant exposes a defect in the crate, cut a leaf for
it; the corpus freeze is the parent node's rule.

## Decisions (running log)

**All eleven runs are taken, every one reporting 560.** Control: 560 tests, 549
passed, 11 failed, the eleven being ten `prompt.rs` composition tests plus
`the_namespace_is_the_shipped_plugin_entrys_declared_name`. Each mutant is
`comm -13 control mutant` over the sorted failure names.

| row | arm | line | page says | re-measured |
|---:|---|---:|---|---|
| 1 | `refuse_finish_kind` | 346 | nothing | **nothing** |
| 2 | `initialize` failed | 391 | `a_refused_grove_leaves_no_root_behind` | same, alone |
| 3 | store created nothing | 396 | nothing | **nothing** |
| 4 | store reported a non-charter first | 399 | nothing | **nothing** |
| 5 | `allocated` disagreed | 405 | nothing | **nothing** |
| 6 | `RootShape::ATree` | 457 | two tests | **ten** |
| 7 | `RootShape::Taskless` | 484 | one test | same, alone |
| 8 | `RootShape::Unrecognised` | 487 | one test | same, alone |
| 9 | `grove_name` fallback | 1028 | nothing | **nothing** |
| 10 | `append_brief_suffix_in_file`'s `return Ok(())` | 1065 | nothing | **nothing** |
| — | bare `panic!` on entry to `root_init` | 346 | reddens 41 | **43** |

**Exactly two numbers on the page moved**, which is the finding: row 6 and the
entry probe. Nine of the eleven readings reproduce exactly, so the corrected
harness is not a wholesale invalidation of the table — it changes the two
readings the `grove` binary could reach and leaves the rest standing.

**Row 6's eight extra observers are all out-of-process, and none of them
observes the arm.** Five are in `crates/grove-loop/tests/driver_lease.rs`
(`driver_uses_the_on_disk_worktree_while_the_configured_command_inherits_git_context`
422, `a_second_driver_refuses_before_tree_access_or_launch` 888,
`a_lease_replaced_under_a_running_driver_refuses_the_next_transition` 991,
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` 1032,
`a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session` 1124) and
three in `crates/grove-llm/tests/removed_surface.rs`
(`a_configured_launch_in_a_native_jj_tree_ignores_the_removed_environment` 797,
`a_configured_launch_in_a_colocated_jj_tree_ignores_the_removed_environment` 805,
`the_launch_fixture_still_observes_a_configuration_driven_change` 814). Every one
of them drives the loop against a worktree that *already has* a grove, so
`transition_to_current` classifies it `ATree` before any launch — and each dies
in its fixture rather than in an assertion, the same shape chapter 19 reads in
`prompt.rs`. The two the page already names assert on what the arm returns:
`AlreadyCurrent`, and that the grove is left unchanged and ready for `pick`.

Both files are `tests/` directories, which this book treats as evidence and not
as roots, so none of the eight belongs to any chapter's block. Checked rather
than assumed (`a-test-belongs-to-its-line-range`).

**Re-run, as the node brief requires.** Row 6's set names five `driver_lease.rs`
tests, so the mutant was run twice; the two failure sets are byte-identical, and
the whole run took 51 seconds, far short of the 120-second producer timeout that
makes a wedge look like an observer.

## Decisions (running log)

1. **Line 176's `558` stays, and is marked as the superseded reading it is.**
   That sentence records the run taken when the chapter was drafted — before
   `unreachable-root-clause-k152` and `default-root-slug-two-spellings-k159`
   added their tests — so 558 was the suite it ran against and rewriting it to
   560 would make it false. The page already re-runs that mutation two hundred
   lines further on, against a control of 560 with ten failures, and gets three
   rather than one; what is missing is not a number but the fact that the first
   sentence is superseded where a reader meets it
   (`counts-split-current-state-from-record-of-a-run`). The other four citations
   are claims about the current suite and take 560.
2. **Row 6 is reported as ten reached, two observing.** Calling ten of them
   observers would say the `ATree` arm is pinned ten ways when eight of the ten
   would fail on any panic anywhere in the driver's pre-launch path. Calling it
   two would restate the number the unrelinked harness produced. The page already
   draws exactly this distinction for the entry probe — *a panic at the call site
   measures reachability rather than observation* — so the table gains the
   distinction rather than a new number.
3. **k159's own two counts were re-taken rather than trusted, and reproduce
   exactly.** The section further up the page reports the constant mutation at
   three and the clap mutation at eight against a copy carrying the plugin
   marketplace. Re-run against this child's eleven-failure control: three
   (`transition_initializes_an_absent_grove_under_one_exclusive_guard`,
   `both_scaffolding_doors_name_the_first_leaf_the_same`,
   `bare_scaffolding_is_anchored_before_the_configured_command_inherits_git_context`)
   and eight, intersecting in
   `both_scaffolding_doors_name_the_first_leaf_the_same` alone. So the counts are
   the same against either control — neither mutation reaches the marketplace
   test — and the page now says so where it introduces the second control, in
   place of *ten `prompt.rs` tests instead of eleven*, which implied eleven
   `prompt.rs` failures in the other copy and there are ten.
4. **The five citations split three ways, not one.** 674 and 1374 are claims
   about the current suite and take 560. 1503 and 1512 are the measurement
   section's own control and take 560. 176 is the record of a superseded run and
   keeps 558, with the supersession stated where a reader meets it rather than
   two hundred lines later. Rewriting 176 to 560 would have made a true sentence
   false.
5. **`book-check --final --check all` is green over `docs/walkthroughs/grove-loop`**
   (13 files, 10,557 resolved lines, 0 deferred), and `bash scripts/check.sh`
   passes all eight principal checks — the same state it was in before this
   child.

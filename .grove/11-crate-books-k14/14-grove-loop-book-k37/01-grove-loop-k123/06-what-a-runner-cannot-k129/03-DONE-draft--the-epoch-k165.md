# the-epoch-k165

## Goal

Draft chapter 17 of the `grove-loop` book, *Which calls the lease admits* —
`crates/grove-loop/src/driver_lease.rs` lines 820–1383, the file's whole inline
test module, 564 lines — to a valid slice through `which-calls-are-admitted`.

## Context

- **This is the barest block in the corpus at 3% comment prose, and the whole
  charter is *supply the claim*.** For each of the nine reproduced tests: the
  property it establishes, **and what would have to be true for it to pass while
  that property was broken**. A test name is a label, not an argument, and this
  block gives you almost nothing else.
- The nine the structure brief names:
  `an_alias_equivalent_second_owner_is_refused_immediately`,
  `lease_path_replacement_retries_until_the_locked_descriptor_is_current`,
  `lease_path_replacement_fails_closed_after_eight_attempts`,
  `acquired_driver_descriptors_are_close_on_exec`,
  `activation_and_invalidation_replace_one_stable_epoch_record`,
  `an_admitted_old_operation_finishes_before_replacement_invalidates_new_calls`,
  `manual_agent_operations_need_no_driver_epoch`,
  `an_active_epoch_without_a_live_lease_is_stale`, `a_malformed_epoch_is_stale`.
  **Count them in the block before believing the list** — the brief's
  enumerations have been wrong twice in this book already
  (`pick-test-count-k147`, `structure-brief-chapter-attributions-k163`), and a
  cited test list is a measurement rather than a reading.
- **Chapter 16 owns the production half and the hooks these tests drive.**
  `acquire_with`'s `before_initial_epoch_handoff`, `acquire_epoch_file_with`'s
  `after_open` / `after_lock` / `report_contention` / injected clock and wait, and
  `probe_live_lease_with_post_unlock_hook`'s `after_successful_probe` are all
  read on chapter 16's page as *the internal test seams* the ADR's last paragraph
  reserves. Chapter 17 says what each seam is used **for**, and does not re-read
  the production functions.
- **A test belongs to the block its line number falls in.** The split here is at
  the `#[cfg(test)]` line and not by concept, so a test that exercises
  acquisition still belongs to this chapter and an acquisition helper still
  belongs to chapter 16, whatever the subject says.
- The chapter keeps `one-live-driver-per-working-tree`, named in prose and never
  linked, and declares guide anchor `usage-driver-lease` and glossary anchors
  `driver-lease` and `session-epoch` alongside chapter 16.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  which-calls-are-admitted --check all` is valid at 9,315 resolved lines, 1,218
  deferred, `final=false`.
- The `lease-tests` ownership row reads `resolved`; contents and navigation
  updated; `scripts/check.sh` red on `book-check` alone, and this file says so.

## Notes

**`.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE`, so an admission arm
that reads it is unobservable in process** — a guard, not a gap. Do not write a
coverage sentence about `signal_path_from`'s arms without the distinction; the
node brief carries the k128 measurement that establishes it.

**A mutant naming a `driver_lease.rs` test must be re-run** — this file wedges
under load and a timeout reads exactly like an observer.

## Decisions (running log)

**The structure brief's chapter-17 test list is wrong twice, and a correction
leaf carries it.** The brief says *The whole inline test module:* and names nine.
The block holds **eighteen** `#[test]` functions, and one of the nine —
`an_alias_equivalent_second_owner_is_refused_immediately` — is not in the block
at all: it is `crates/grove-loop/tests/driver_lease.rs:398`, which this book's
corpus rule calls evidence rather than a root. Counted, not read, exactly as this
leaf's own body warned. The page states the enumeration and is the one place a
later page should take chapter 17's test count from; the brief is corrected by
its own leaf, on the `structure-brief-dependency-count-k132` /
`lease-size-ranking-k171` precedent, inserted ahead of `which-files-k166` so the
rest of Part V and every later stage read a corrected brief.

**`cargo doc` is structurally blind to this entire block, and that is measured.**
The instrument chapter 16 leaned on — attachment and unresolved intra-doc links —
compiles with `cfg(test)` off, so none of the block's four `///` runs is ever
read. Control A: a broken intra-doc link planted inside the test module leaves
the crate at its usual **30** warnings. Control B: the identical construct in the
production module header takes it to **31** and names
`driver_lease.rs:2`. So the clean run chapter 16 reported is evidence about 819
production lines and says nothing whatever about these 564. This is a second
blind spot orthogonal to chapter 16's `//`-versus-`///` one: that one is about
which *marker* a comment uses, this one about which *cfg* the item is under.

**The block's own comment count is 19, not 17, and the brief's 3% holds.** My
first reading miscounted the four runs by hand; counted under chapter 15's rule
(lines whose first non-space characters are `//`) it is 19 in 564, or 3.4%. The
comparison worth making is test module against test module, not against
production text: `task_name.rs` 142/694, `task_tree.rs` 158/1,008,
`tree_lifecycle.rs` 256/1,649, `loop_driver.rs` 6/69. All five agree with the
structure brief's figures at its rounding — no disagreement to record here, unlike
chapters 15 and 16.

**Four of `admit_session`'s refusal arms are observed by nothing, and one of the
four is dead rather than untested.** Measured by replacing each whole macro call
with `panic!("MUTANT")` in a workspace copy and diffing per-test results against a
control of 558 tests / 547 passing / 11 environmental `prompt.rs` failures. Every
mutant reported the control's test count, so none failed to compile silently, and
both zero-readings were re-run per this leaf's own warning. The reachable zeros
are the working-tree identity rung, the probe's lease-record comparison, and the
probe's eight-attempt identity exhaustion; the unreachable one is the trailing
`bail!` after that loop, which the last iteration can never fall through to. The
channel-mismatch rung has three observers, **two of them out of process** — which
is what the whole-macro panic buys and a message-preserving one would have hidden.

**`EPOCH_HANDOFF_TIMEOUT` and `EPOCH_WAIT_INTERVAL` are pinned by nothing.** Each
appears exactly twice in the workspace — its declaration and the one wrapper that
passes it — and the timeout test injects its own thirty seconds and matches a
message rendered from that same argument. The retry limit is different: it is
pinned, but by the string literal `8 times` rather than by the constant the count
assertion compares against.

**No early-use row is owed and none was added.** The block imports `super::*`, so
every symbol it names or exercises belongs to chapter 16's production half;
enumerating the block's identifiers against the items declared in
`session_config.rs`, `prompt.rs` and `loop_driver.rs` left two hits, `path` and
`run`, both of which are a local binding and the word *Re-run* in a comment.
`check_early_uses` agrees — the slice is green.

**Delivered.** `book-check --repo . --book docs/walkthroughs/grove-loop --through
which-calls-are-admitted --check all` is **valid: 13 files, 9315 resolved lines,
1218 deferred lines, final=false**, and the `lease-tests` ownership row reads
`resolved`. `bash scripts/check.sh` is **red on `book-check` alone — 1 of 8
checks failing** — and the four `book-check` findings under it are the unwritten
chapters 18 to 21 plus the `--final` navigation those pages will supply. That is
the shape this node's brief predicts for every child but the last.

**No `copy-edit` leaf is cut here.** Both `grove-loop-book-k37`'s brief and
`grove-loop-k123`'s put that act on `what-could-not-move-k130`, the draft node's
last child; this leaf is a grandchild of the document node and holds no stage of
the pipeline chain.

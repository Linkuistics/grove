# what-a-runner-cannot-k129 — brief

## Goal

Draft Part V of the `grove-loop` book — chapters 16 to 20, owning
`crates/grove-loop/src/driver_lease.rs` (1,383 lines in two blocks),
`src/session_config.rs` (358), `src/prompt.rs` (245) and `src/loop_driver.rs`
(615), 2,601 lines in all — and prove the prefix through slice
`four-things-a-runner-cannot-choose`, which resolves the last deferred block in
the book.

## Done when

- `book-check --repo . --book docs/walkthroughs/grove-loop --through
  four-things-a-runner-cannot-choose --check all` is valid: 13 files, 10,533
  resolved lines, 0 deferred, still `final=false` because chapter 21 does not
  exist yet.
- Chapters 16–20 exist, contents and navigation are updated, and every ownership
  row in the ledger reads `resolved`.
- The four chapter-1 cast rows owned by `one-per-working-tree`,
  `whose-file-and-whether`, `too-late-to-say-later` and
  `four-things-a-runner-cannot-choose` read `explained` — the last of them
  closing every row in the early-use ledger.
- `scripts/check.sh` is red on `book-check` alone, in every child but the last,
  and each child's task file says so.

## Decomposition

**Five draft children, one per chapter**, in page order because `--through`
proves a canonical prefix. Each is one root or one half of one, so a child is a
chapter and nothing else; none of them is expected to decompose again. One
correction leaf sits between the first two, cut by chapter 16 and placed so the
rest of Part V reads a corrected structure brief.

| Pos | Child | Kind | Chapter | Root and block | Lines | Cumulative | Deferred |
|---:|---|---|---:|---|---:|---:|---:|
| 01 | `the-lease-k164` | draft | 16 | `driver_lease.rs` 1–819 | 819 | 8,751 | 1,782 |
| 02 | `lease-size-ranking-k171` | impl | — | — | — | — | — |
| 03 | `the-epoch-k165` | draft | 17 | `driver_lease.rs` 820–1383 | 564 | 9,315 | 1,218 |
| 04 | `which-files-k166` | draft | 18 | `session_config.rs` 1–358 | 358 | 9,673 | 860 |
| 05 | `the-core-k167` | draft | 19 | `prompt.rs` 1–245 | 245 | 9,918 | 615 |
| 06 | `the-loop-k168` | draft | 20 | `loop_driver.rs` 1–615 | 615 | 10,533 | 0 |

`the-loop-k168` resolves the last deferred block in the book and is the one that leaves
`--through` reporting `0 deferred`; it is **not** the last child of
`grove-loop-k123`, and `--final` still fails, because chapter 21 does not exist
until `what-could-not-move-k130`. The `copy-edit` leaf is cut by that leaf, not
by anything here.

## Pointers

- **This part carries both halves of the prose obligation, and they are
  opposite.** `driver_lease.rs` splits at its `#[cfg(test)]` line into chapter 16
  (1–819, **12% comment prose**) and chapter 17 (820–1383, **3%**) — the only
  split in the book that is not by concept, made because the two halves need
  opposite treatments and one chapter cannot carry both instructions. Chapter 16
  takes *supply the argument*: per mechanism, the line that enforces it, the
  failure it prevents, and the record clause it keeps. Chapter 17 takes *supply
  the claim*: what each scenario establishes and what it would still pass under.
  Chapters 18, 19 and 20's production halves — 44%, 69% and 51% — take *do not
  restate*.
- **Chapter 16 is the book's deliberate counterexample, and the page that states
  it is chapter 13, not chapter 1.** Everything else in the crate is re-derived
  from the tree; the lease and the epoch cannot be. `13-outcomes.md` lines 25–37
  carry the wording — *1,383 lines of locking whose whole purpose is to hold
  state the tree must **not** hold* — and the structure brief's *The stated
  outcome* says the contrast is stated once, at the earlier page, and inherited
  by chapter 16. Do not re-argue it, and do not
  let the chapter open by contradicting the spine. (This brief said *chapter 1*
  when it was k129's leaf body; chapter 1 names the lease in its Part IV/Part V
  paragraph and makes no re-derivation claim at all. Corrected at
  `the-lease-k164` against both pages.)
- **Chapter 18 adjudicates the read-count claim and chapter 20 refutes it.**
  `src/session_config.rs` line 89 says the loop re-reads the configuration once
  per iteration; `src/loop_driver.rs` calls `templates.load(&delta_roots)` twice,
  at lines 241 and 260, for two different reasons the structure brief names. This
  is the campaign's only adjudication whose claim and refutation are both inside
  one book's corpus. State it beside the fragment that reproduces the comment;
  never repeat it as true and never silently correct it.
  `template-source-read-count-k86` holds the source fix and lands after this book.
- Chapter 19's rule is the too-late test, and its closure on the word *fact*: a
  driver fact is a launch-varying value the methodology cannot know at authoring
  time, and its static meaning stays in the skill.
- Chapter 20's rule is that all the spawning, watching and escalating is
  `keyed-launch`'s; what stays is the four things a loop must choose — the
  channel's directory, the variable that publishes it, the variables scrubbed,
  and the two graces. It also carries *restart ≡ continuation* and the shell
  sketch the header keeps, which is still the whole loop *because a boundary is
  not a step*.
- Guide anchor `usage-driver-lease` (chapters 16, 17) and glossary anchors
  `driver-lease`, `session-epoch` (16, 17), `guaranteed-core` (19),
  `stated-vcs` (19, 20) and `loop-control-channel` (20) are this part's; all
  exist today in explicit form, so `M201` stays green.
- The decision records this part keeps, **named in prose and never linked**:
  `one-live-driver-per-working-tree` (16, 17), `complete-session-configuration`
  (18), `the-launched-child-is-a-job` (20), `grove-does-not-stage-its-own-renames`
  and `entries-are-never-removed` where a chapter reaches them.
- **No residue marker in `docs/ARCHITECTURE.md` is assigned to chapters 15–18.**
  The structure brief's *What this book makes redundant* maps its thirty-one
  markers to chapters 1–14, 19 and 20 only, so chapters 16, 17 and 18 carry no
  coverage obligation toward `architecture-residue-k75` and should not invent
  one; chapters 19 and 20 carry four between them (461, 472, 1343; 1190, 1325).

## Found while drafting

**Promoted from `the-lease-k164`.** Chapter 16 landed: the slice is valid at
8,751 resolved lines with 1,782 deferred, the `lease-and-epoch` ownership row
reads `resolved`, the chapter-1 cast row owned by `one-per-working-tree` reads
`explained`, and `scripts/check.sh` is red on `book-check` alone. Six findings
are live obligations for chapters 17 to 21.

- **Enumerate a block's backticked tokens; do not sweep a list of expected
  names.** Twenty tokens in `driver_lease.rs` 1–819, classified one by one, left
  exactly one resolving to nothing — `probe_lease_holder`, named in a comment and
  defined nowhere in the workspace. A sweep for the names a reader expects would
  not have contained it. `lease-stale-reader-name-k169` holds the fix.
- **A one-line grep is not a sweep, and a wrapped phrase reads as a clean tree.**
  Searching the repository for `third largest` returned only this session's own
  files; the phrase is in the structure brief, wrapped as `the third\nlargest`.
  The claim was settled by **enumerating the roots and the owned blocks**, not by
  finding the sentence. Chapters 17 to 21 all carry size or count claims, and
  each is worth the enumeration rather than the search.
- **The size of `driver_lease.rs` is settled, and the unit is the trap.** Roots
  run 2,725 / 2,023 / 1,714 / **1,383**, so the root is fourth; owned blocks run
  819 / 808 / 775, so chapter 16's is first, and 1,383 is not a block at all.
  `lease-size-ranking-k171` has landed: it corrected *the third largest* in the
  structure brief's chapter 16 section, *the second-largest owned block* in the
  same brief's rejected-spines paragraph — a third site the leaf body did not
  name, found by enumeration — and dropped the rank from `13-outcomes.md`,
  leaving its argument and its 1,383 intact. Chapter 16 states the enumeration
  and is the one place a later page should take a size from. **Chapters 17 to 21
  still owe the enumeration for their own size and count claims**; what is closed
  is this root's rank, not the class.
- **A clean `cargo doc` run is evidence about `///` and `//!` and nothing else,
  and this part will meet the blind spot again.** Thirty warnings over the crate,
  none naming `driver_lease.rs` — and the block's one wrong name is in the 33 of
  its 103 comment lines written as plain `//`. Chapter 20's `loop_driver.rs` is
  51% comment prose; check which marker it uses before trusting a clean run,
  and note that `prompt.rs` carries one of the crate's five unresolved intra-doc
  links, at line 28.
- **State a guarantee as an enumeration with its exceptions, not as the record's
  slogan.** The ADR says *every descriptor is close-on-exec*; the block has five
  opens and four are marked, the fifth being `/dev/urandom`, dropped inside the
  function that opens it and never live across a spawn. The code is tighter than
  the sentence rather than looser, and saying which is what the *supply the
  argument* charter is for. Chapters 18 to 20 keep records with equally absolute
  wording.
- **The block's outward surface is nine production call sites in three files, and
  six of them are chapter 20's.** `acquire` from `crates/grove/src/cli.rs`;
  `worktree_root`, `control_dir`, `activate_session_epoch`,
  `invalidate_session_epoch` and two `revalidate`s from `loop_driver.rs`;
  `admit_ambient_session` and `require_signal_path` from
  `crates/grove-llm/src/cli.rs`. Chapter 20 owns the six and should say what each
  reaches rather than re-deriving the lease's contract, which chapter 16 has
  already argued line by line.

**Two book-wide pages needed structural repair and chapter 16 made it.**
`README.md`'s contents listed chapter 15 twice — the linked entry plus the
unlinked placeholder its own session should have consumed — and
`concept-index.md` had chapter 15's twenty-six entries inserted inside chapter
14's run instead of after it. Both are the ordered section plan as a reader meets
it, so both are `draft`'s. **Every later child should check the same two pages**:
consume your chapter's placeholder line rather than adding beside it, and append
your concept entries after the last chapter's run rather than before the
lookup sections.

## Notes

**The corpus is frozen.** A defect found while drafting becomes its own leaf and
is not fixed inline — and `session_config.rs`'s stale sentence in particular is
adjudicated on the page rather than corrected.

**`.cargo/config.toml` force-clears `GROVE_SIGNAL_FILE` to the empty string for
everything cargo runs**, so every arm of this part's admission path that reads it
is unobservable *in process* — a guard rather than a gap, established at
`the-surface-k128` by deleting `signal_channel`'s whole `or_else` and watching
558 tests report the control's exact 547/11. `driver_lease.rs:731` reads the same
variable through `signal_path_from`, and chapters 16 and 17 own that pattern; a
coverage sentence about any of those arms owes the same reading and the same
distinction between *untested* and *unreachable under the harness*.

**A mutant naming a `driver_lease.rs` or `prompt.rs` test must be re-run.** Both
wedge under machine load and a timeout reads exactly like an observer — and this
part owns both files, so the correction applies to every measurement taken here
rather than to an occasional one.

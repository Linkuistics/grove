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
chapter and nothing else; none of them is expected to decompose again. **Three
correction leaves sit among them**, each cut by the chapter that found the defect
and placed so the rest of Part V reads a corrected artifact. The first two correct
`docs/specs/grove-loop-book-structure.md` and neither touches the book; the third
corrects a finished page and not the brief.

| Pos | Child | Kind | Chapter | Root and block | Lines | Cumulative | Deferred |
|---:|---|---|---:|---|---:|---:|---:|
| 01 | `the-lease-k164` | draft | 16 | `driver_lease.rs` 1–819 | 819 | 8,751 | 1,782 |
| 02 | `lease-size-ranking-k171` | impl | — | — | — | — | — |
| 03 | `the-epoch-k165` | draft | 17 | `driver_lease.rs` 820–1383 | 564 | 9,315 | 1,218 |
| 04 | `structure-brief-test-list-k172` | impl | — | — | — | — | — |
| 05 | `which-files-k166` | draft | 18 | `session_config.rs` 1–358 | 358 | 9,673 | 860 |
| 06 | `the-core-k167` | draft | 19 | `prompt.rs` 1–245 | 245 | 9,918 | 615 |
| 07 | `chapter-eighteen-size-claim-k175` | impl | — | — | — | — | — |
| 08 | `the-loop-k168` | draft | 20 | `loop_driver.rs` 1–615 | 615 | 10,533 | 0 |

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

**Promoted from `the-epoch-k165`.** Chapter 17 landed: the slice is valid at
9,315 resolved lines with 1,218 deferred, the `lease-tests` ownership row reads
`resolved`, and `scripts/check.sh` is red on `book-check` alone — 1 of 8, the
four remaining failures being the unwritten chapters 18 to 21. Five findings are
live obligations for chapters 18 to 21.

- **The structure brief's chapter 17 test list was wrong in both directions, and
  `structure-brief-test-list-k172` holds the correction.** It said *the whole
  inline test module* and named nine; the block holds eighteen, and one of the
  nine — `an_alias_equivalent_second_owner_is_refused_immediately` — is in
  `crates/grove-loop/tests/driver_lease.rs`, which is evidence rather than a
  root. **Chapters 18 to 20 should count their own block's tests against the
  brief before writing a word**; this is the fifth structure-brief correction in
  this book and the class is not closed.
- **`cargo doc` cannot see a `#[cfg(test)]` module at all, and this is a second
  blind spot rather than more of chapter 16's.** Chapter 16's was about which
  *marker* a comment uses; this one is about which *cfg* the item sits under, and
  it removes 3,984 lines — every inline test module in the book — from the
  instrument's reach. Measured with a control in each direction: a broken
  intra-doc link planted inside the test module leaves the crate at thirty
  warnings, the identical construct in the production header takes it to
  thirty-one. **Chapter 20 owns `loop_driver.rs`'s inline test block** and
  inherits this exactly; a clean run says nothing about it.
- **A bound is usually pinned by a string literal, not by its symbol, and
  sometimes by neither.** `lease_path_replacement_fails_closed_after_eight_attempts`
  asserts the count against `IDENTITY_RETRY_LIMIT` — which moves with the constant
  and so pins nothing — and pins 8 only through the message it matches. Worse,
  `an_orphaned_epoch_guard_times_out_post_reap_once_at_the_fixed_bound` injects
  its own thirty seconds and matches a message rendered from that same argument,
  so **`EPOCH_HANDOFF_TIMEOUT` and `EPOCH_WAIT_INTERVAL` are pinned by nothing in
  the corpus**: each appears exactly twice, at its declaration and at the one
  wrapper that passes it. Chapters 18 to 20 all own constants; enumerate a
  constant's uses before writing that a test holds its value.
- **A shared context string defeats attribution, and the mutation is what shows
  it.** Six of `admit_session`'s rungs and the whole liveness probe — seven sites
  — wrap their refusal in `stale Grove session for {operation}`, so
  `a_malformed_epoch_is_stale`'s bare `contains("stale Grove session")` pins
  *refused and classified stale* and not *refused because malformed*. Its sibling
  one rung up shows the fix: a negative assertion. Chapters 18 to 20 all reproduce
  tests asserting on refusal text; match each substring against the `bail!` texts
  **and** run the mutation, because a substring matching seven sites looks exactly
  like one matching one.
- **A zero is deadness as often as absence, and the two need separating on the
  page.** Four of `admit_session`'s refusal arms are observed by nothing. Three
  are reachable — the working-tree identity rung, the probe's record comparison,
  and the probe's eight-attempt identity exhaustion, whose fixture the existing
  `after_successful_probe` hook already makes possible. The fourth is the trailing
  `bail!` after the retry loop, which control never reaches because the last
  iteration always returns or bails; **the file has three of those, one after each
  bounded-retry loop, and all three are in chapter 16's half**. Report the
  reachable ones as asymmetries against their tested siblings, not as bare
  absences.


**Promoted from `structure-brief-test-list-k172`.** The chapter 17 section of
`docs/specs/grove-loop-book-structure.md` now names eighteen tests in file
order and no longer names the `tests/` integration test; it is the only site in
the brief that states chapter 17's count, confirmed by enumerating the four
candidate sections and by a reverse grep of all eighteen names.

- **A list handed to you is evidence of the same rank as the list you came to
  fix.** The leaf body's own inventory of missing names was short by one —
  `a_successful_liveness_probe_releases_the_lease_before_validation`, line 1305 —
  so the shortfall was ten, not nine. Chapters 18 to 20 owe the count against
  their own block, and owe it against the block rather than against any prose
  list of it, this brief's *Pointers* included.

**Promoted from `which-files-k166`.** Chapter 18 landed: the slice is valid at
9,673 resolved lines with 860 deferred, the `whose-file` ownership row reads
`resolved`, the chapter-1 cast row owned by `whose-file-and-whether` reads
`explained`, and `scripts/check.sh` is red on `book-check` alone — 1 of 8, the
remaining failures being the unwritten chapters 19 to 21. Six findings are live
obligations for chapters 19 to 21.

- **The citation class is wide open, and this block held two more.**
  `session_config.rs` line 271 cites *the search precedence requirement 6 fixes*
  — **there is no requirement 6** anywhere in the repository;
  `module-decomposition.md` numbers its *decisions* 1–11 and names its four
  requirements, and decision 6 fixes the quantifier, not a search order. Line 331
  cites the same record the block cites correctly three times elsewhere, but as a
  Markdown link to `../docs/adr/…`, which resolves neither from the rendered page
  nor from the source file. Both are address-only — the claims are true — so both
  were adjudicated on the page under the `paths-k142` precedent rather than cut,
  and `template-source-read-count-k86` is already editing this file's comments
  inside the frozen line counts and can carry them. **Chapters 19 and 20 are 69%
  and 51% comment prose and cite heavily**; enumerate every citation's target and
  read the section, rather than trusting that a named record holds what the
  sentence leans on it for.
- **A seventh instrument gap: `cargo doc` never checks an explicit-URL Markdown
  link.** It reports unresolved *intra-doc* links (`` [`Foo`] ``) and says nothing
  at all about `[text](some/path.md)`. That is distinct from chapter 16's marker
  blind spot and chapter 17's `#[cfg(test)]` one, and it is why line 331 earns no
  warning. **Any chapter checking its block's links must read the link forms, not
  only the warning list.** `prompt.rs` line 28 carries one of the crate's five
  genuine unresolved intra-doc links, so chapter 19 has both kinds to separate.
- **This root was the exception on instruments, and chapters 19 and 20 are not.**
  `session_config.rs` has zero plain `//` comments and no `#[cfg(test)]` module,
  so `cargo doc` reaches all of it — established with a control that took the
  crate from thirty warnings to thirty-one by planting a link in its header. Do
  not carry that conclusion forward: `loop_driver.rs` has an inline test module
  the instrument cannot see at all, and its marker mix is unchecked.
- **An assertion entailed by the one above it holds nothing, and reads as though
  it holds the remedy.** In `a_snapshotted_jj_delta_is_refused_in_both_jj_shapes`
  the third assertion checks for `/.grove.kdl` while the second already requires
  the candidate's full path, which ends in those bytes. Deleting the refusal's
  **entire** `Untrack it (…)` sentence leaves all twenty tests in the evidence
  file green. **Before writing that a test pins a message, check whether each
  assertion can fail independently of its neighbours** — and confirm by deleting
  the text, not by matching it.
- **A substring shared by two failure modes defeats attribution even when the
  mutation is decisive.** `a_trackedness_probe_that_cannot_be_completed_fails_closed`
  asserts `contains("is tracked")`; the mutation proves the `with_context` wrapper
  is what carries it, but the `bail!` below reads *it is tracked in version
  control* and would satisfy the same assertion. The test pins *refused while
  asking about trackedness*, not *the probe was unanswerable*. This is
  `the-epoch-k165`'s class met a second time, and chapters 19 and 20 both
  reproduce refusal text.
- **Enumerate a uniqueness claim before writing it, including your own prose.**
  This draft wrote *the one item in the block with no prose of its own* about
  `ExpansionContext`; enumeration returned **seven** of the block's twenty-five
  items, and the sentence was corrected before the page shipped. The counting
  discipline the earlier children applied to the source applies to the page as
  well.

**Promoted from `the-core-k167`.** Chapter 19 landed: the slice is valid at 9,918
resolved lines with 615 deferred, the `the-prompt-core` ownership row reads
`resolved`, the chapter-1 cast row owned by `too-late-to-say-later` reads
`explained`, and `scripts/check.sh` is red on `book-check` alone — 1 of 8, the
remaining failures being the unwritten chapters 20 and 21. Seven findings are
live obligations for chapters 20 and 21.

- **The baseline's eleven are two causes, and the loss happened in the summary
  rather than in the measurement.** `grove-loop-k123`'s brief carried *558 tests,
  547 passed, 11 failed, all `crates/grove-loop/tests/prompt.rs`, because the copy
  is not a jj repository* — but **chapter 11 had already recorded the split
  correctly** (*ten … with `NotAWorkspace`, and one … on a manifest the copy does
  not carry*) and chapter 10 reported **ten** under a wider copy of 626 tests.
  Both briefs are now corrected; the lesson is that a promoted figure is a
  paraphrase of a page and can lose a distinction the page made. Ten of that
  file's sixteen tests reach `compose` — nine through it or `compose_with`, one
  through `signalling_contract()` — and die in the `workspace()` fixture, which
  resolves the repository root and unwraps. The eleventh,
  `the_namespace_is_the_shipped_plugin_entrys_declared_name`, composes nothing: it
  reads `.claude-plugin/marketplace.json`, which **the promoted copy recipe did
  not copy** until this leaf added it. With the directory in place the control is
  exactly ten. **A control taken at eleven hides an observer** — a mutation of
  `PLUGIN`, which that test reads five times, would produce an empty
  newly-failing set against a baseline that had already written the test off.
  This is the same direction of error as omitting `cargo build -p grove --bins`,
  and chapter 20 owns the last inline test block in the book.
- **A root's instrument reach is a measurement, not an inheritance.**
  `which-files-k166` forecast that chapters 19 and 20 were *not* the exception
  `session_config.rs` was; measured, `prompt.rs` is one — zero plain `//`, zero
  `#[cfg(test)]`, so `cargo doc` reaches all 171 of its comment lines, confirmed
  by a control that took the crate from thirty warnings to thirty-one from inside
  a `///` docblock. The forecast was argued about `loop_driver.rs`'s inline test
  module only, and it still stands for chapter 20. **Take the reading rather than
  the prediction, in both directions.**
- **A rule id is a citation form no instrument in this repository checks, and the
  crate uses it.** `prompt.rs:234` names *the spine's
  `skill-stated-vcs-is-definitive`*; the inventory is
  `plugins/grove/conformance/rules.tsv`, 169 rows with ids unique across the file,
  and the rule is `stated-vcs-is-definitive` — **no id there begins with
  `skill-`**. The claim is true and only the identifier is wrong, so it is
  adjudicated on the page and `prompt-rule-id-prefix-k174` holds the fix. It is
  not an intra-doc link, not a Markdown link and not an `ADR <slug>` citation, so
  `cargo doc`, the link sweep and `every_adr_citation_names_a_decision_record` are
  all silent. **Only enumerating a block's backticked tokens and resolving each
  one finds this class** — chapter 16's `probe_lease_holder` procedure, met a
  second time and now with a second citation *form* behind it.
- **Census the citations rather than spot-checking them.** `prompt.rs` carries
  eighteen citations to a document or record over 245 lines — seven to
  `module-decomposition.md` alone, naming four distinct decisions. Seventeen
  resolve and hold what their sentence leans on them for, including decision 9,
  which carries `Mandate`'s four fields and `compose`'s signature as written
  source. Chapter 20's root is 46.7% comment prose whole and cites at a similar
  rate; the count that matters is *citation sites*, not *distinct targets*, and
  the two differ here by a factor of two.
- **A count of the shipped kind set goes stale silently, and this campaign is what
  staled it.** `tests/prompt.rs`'s `no_prompt_states_the_stop_flag` doc comment
  says `--done` would reach *the eighteen kinds it is not an ending for*;
  `plugins/grove/skills/` holds twenty-four directories, twenty-three of them
  `grove-<kind>`, so it is twenty-two. Nineteen is the count `prompt.rs`'s own
  header records twice, so the comment was right when written. The **test** is
  unaffected — it iterates `shipped_kinds()`, which reads the directory — which is
  exactly why nothing went red. `stop-flag-kind-count-k173` carries this and the
  same defect at `crates/grove-llm/tests/kind.rs:2`, which additionally still
  calls the set *closed*.
- **`lease-size-ranking-k171`'s class recurred on a finished page, and the
  enumeration is now on chapter 19.** `18-which-files.md` calls
  `session_config.rs` *the smallest root in Part V* at two sites; Part V's roots
  are 1,383 / 615 / 358 / **245**, so `prompt.rs` is. `chapter-eighteen-size-claim-k175`
  holds the correction and is placed **ahead of `the-loop-k168`** so chapter 20
  reads a corrected page, exactly as k171 was placed ahead of the rest of the
  part. The structure brief carries the word nowhere, so no brief edit is owed —
  which is the difference from k171 and the reason this one is two sites rather
  than three. **Chapter 20 and chapter 21 still owe the enumeration for their own
  size and count claims**; two chapters of this part have now got one wrong.
- **A block with no failure path leaves Part V's two sharpest instruments with
  nothing to bite on, and that is worth stating rather than skipping.**
  `prompt.rs` has no `Result`, no `?`, no `unwrap`, no `expect`, no `panic!` and
  no `bail!` — and no conditional at all, which is what makes *the driver
  interprets a kind nowhere* checkable rather than asserted. There were no refusal
  arms to attribute and no mutation to run. Chapter 21 should not read the absence
  of a coverage table on this page as an omission.

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

# canonicalisation-sites-k149

## Goal

Correct `crates/grove-loop/src/task_tree.rs`'s module-header claim that
canonicalisation appears in exactly one place, and reconcile the two pages of the
`grove-loop` book that reproduce or restate it.

## Context

- **The defect, in the module header.** `crates/grove-loop/src/task_tree.rs`
  lines 27 to 29 read *Canonicalisation appears once, in [`leaf_entry`], and only
  to `compare` a caller's spelling of a leaf against the tree's*. It appears in
  two functions of the same file. `target` (lines 335 to 374) canonicalises the
  candidate path, the grove root, and each candidate entry's built path;
  `leaf_entry` (lines 665 onwards) does the same three things. Enumerated with
  `grep -rn 'canonicalize' crates/grove-loop/src/`: six production call sites,
  lines 346, 349 and 369 in `target` and 712, 715 and 734 in `leaf_entry`, plus
  two inside `driver_lease.rs`'s `#[cfg(test)]` module at 1237 and 1241.
- **It is refuted from inside the same file**, which is what makes it the same
  class as the two claims the structure brief already lists. `target`'s own doc
  comment at line 332 says *Canonicalised to **compare** and never to report,
  exactly as `leaf_entry` does* — an explicit statement that there are two.
- **The second half of the header's sentence is true and must survive.**
  Canonicalisation is only ever used to compare, never to produce a path grove
  hands back, in both functions. The false half is *once, in `leaf_entry`*. The
  true claim is narrower and still worth making: canonicalisation appears only in
  the two resolvers, and neither returns a canonicalised path.
- **Found by `paths-k142`**, chapter 6's draft, which owns `target` and therefore
  owns the counterexample. Chapter 6 adjudicates the claim on the page beside the
  fragment that reproduces `target`'s doc comment
  (`06-paths.md#canonicalise-to-compare`) and does not repeat it as true.
- **Two book pages restate the claim and one of them asserts it.**
  `docs/walkthroughs/grove-loop/05-opening.md`, at
  `#one-spelling-of-the-root`, reproduces the header fragment
  `«tree-header-no-canonicalising»` and then says *The minimum is the exception
  the passage states: it is the one place canonicalisation appears*. That
  sentence was corrected by `paths-k142` to point forward to chapter 6's
  adjudication rather than to assert the claim; check it reads correctly against
  the reworded source, and correct the fragment's bytes if the wording changes.
  The `leaf_entry` row of the early-use ledger in `source-index.md` carries the
  same statement and was corrected in the same pass.
- **Chapter 8 owns `leaf_entry` and has not been written yet.** If it lands
  before this leaf, check it does not reintroduce the claim.
- **The fix must stay inside the existing line counts.** `task_tree.rs` is
  declared 2,023 lines in `walkthrough.toml` and in the source index's root row,
  and its ten ownership blocks carry exact ranges. Reword inside lines 24 to 30 —
  the exact bytes of the `«tree-header-no-canonicalising»` fragment, chapter 5's
  — or carry the manifest, the source index's root and ownership rows, the
  fragment index and every affected fragment on both pages in the same commit.

## Done when

- The header clause in `crates/grove-loop/src/task_tree.rs` states something the
  file bears out, reworded inside the existing line counts or with every affected
  ledger row and fragment carried in the same commit.
- The six production call sites are re-enumerated at the time of the fix rather
  than taken from this file, and any count on a page matches that enumeration.
- `05-opening.md` and `06-paths.md` agree with the corrected comment, and the
  `leaf_entry` early-use row agrees with both.
- `bash scripts/check.sh` passes, including `book-check --final` over the
  `grove-loop` book. This leaf sits after `grove-loop-book-k37`, so the book is
  final by the time it runs and the root brief's one-commit rule can be met in
  full.

## Notes

**This is the third claim of one class in this crate, and the class is worth
naming in the fix.** `lib.rs`'s *`<worktree>/.grove`, spelled in exactly one
place* (`grove-root-join-clauses-k148`, the leaf before this one) and the
manifest's `libc` clause (`manifest-dependency-clauses-k133`) are the other two.
All three are uniqueness claims written from the shape of the design rather than
from an enumeration of the code, and none of the three has a test over its call
sites — unlike `the_librarys_tree_lock_is_taken_from_exactly_one_module`, which
is exactly the instrument they lack. Whether to add one for canonicalisation is
this leaf's to decide and to record either way; **consider doing all three files
in one commit with k148**, which sits immediately before this leaf.

**The corpus freeze does not license this leaf to break it either.** One commit
carries the source change, every affected ledger and page, and a green validator
run over every book it touched — or the leaf is deferred behind the books it
would invalidate and says so here.

## Decisions (running log)

1. **The enumeration re-run at the time of the fix agrees with the task file, and
   widening it past `grove-loop` changes nothing.** `grep -rn 'canonicalize'
   crates/grove-loop/src/` gives eight hits: 346, 349 and 369 in `target`, 712,
   715 and 734 in `leaf_entry`, and 1237 and 1241 inside `driver_lease.rs`'s
   `#[cfg(test)]` module, which opens at line 820. In `task_tree.rs` the only
   `mod tests` opens at line 1016 — the other `#[cfg(test)]` markers are at 60,
   82, 180, 1006 and 1011, and the last two gate `reset_read_count` and
   `read_count` rather than a module — so every one of the file's six hits sits
   **above** any test code and all six are production. Widened to
   `crates/*/src/`, the word `canonical` appears in four crates besides this one,
   and `canonicalize` is **called** in two of them, at four sites:
   `crates/jj-workspace/src/lib.rs` line 332 (the private `canonical` helper) and
   `crates/book-validation/src/cli.rs` lines 182, 191 and 321. Neither crate is
   what the header's *here* scopes over — it is a module header, and its subject
   is this module — so the widening confirms the scope rather than moving it.
   Checked for indirect spellings too (`realpath`, `fs::canonicalize` as a free
   function, `std::fs::canonicalize`): there are none in `grove-loop`.

2. **The true claim is narrower than "the two path resolvers", because several
   functions in this module interpret a caller's path without canonicalising
   anything.** `kind_in` (639) and `brief_chain` (665) take a leaf path and
   delegate; `reference` (932) takes an argument and tries it as a path first;
   `existing_path` (963) turns that argument into a path that exists — absolute,
   or joined onto the grove root, or onto the cwd — and returns the path it
   tried. Six functions in the file take a caller's path or argument and only two
   canonicalise. What `target` and `leaf_entry` uniquely do is resolve a caller's
   path **to a snapshot entry**, and that is the property the rewording states: each
   canonicalises the candidate, the grove root, and each walked entry's built
   path, and neither returns a canonicalised path. Stated structurally rather
   than as a count, per `references/execute.md` — a count of call sites in a
   comment invites exactly the defect this leaf is removing.

3. **No test was added, and the reason is stronger than k148's: the behavioural
   property is not available to assert, and the count property is the same
   tidiness k148 declined.** A count over `canonicalize` call sites would go red
   on a third resolver that canonicalised only to compare, which is correct — the
   same disanalogy with
   `the_librarys_tree_lock_is_taken_from_exactly_one_module` that k148 recorded.
   The property actually worth pinning is behavioural — *a reported path keeps
   the caller's spelling* — and it cannot be asserted at any verb boundary,
   because the root never reaches `task_tree` in the caller's spelling in the
   first place: `Workspace::resolve` canonicalises the workspace root
   (`crates/jj-workspace/src/lib.rs` line 102, and its doc comment says why), and
   `grove_root` joins `.grove` onto that already-canonical path. The test suite
   has already met this and works around it — `crates/grove-llm/tests/resolve.rs`
   compares through `canonicalize` at line 282, under a comment at 277 to 279
   saying *`Workspace::resolve` reports the real path. The claim is the tree
   itself, not a spelling of it*, and `jj_tree_verbs.rs`'s `rel_line` canonicalises both
   sides for the same reason. So an end-to-end test of the header's property
   would be asserting something the binary does not promise. What the header
   claims is module-scoped — *this module adds no canonicalisation of its own* —
   and the honest instrument for that is the comment and chapter 6's account.
   This is why the rewording says *nowhere else **here***: the scope is load
   bearing, not throat-clearing.

4. **The rewording stayed inside its own line count, so the fan-out is one book
   and no ledger range moved.** Seven lines before and seven after;
   `task_tree.rs` is still 2,023, so `walkthrough.toml`'s root row, the source
   index's root and ownership rows and every fragment range are untouched, and
   `«tree-header-no-canonicalising»` still reads `lines="24-30"`. The surfaces
   the task file predicted were two pages; the actual set is **eleven**, and the
   ones it did not name were found by grepping the claim rather than a file
   list: `01-orientation.md` (the class paragraph, which named the third member
   as uncorrected), `08-kind-and-briefs.md` (*the header's claim … is false*,
   present tense), `18-which-files.md` and `20-the-loop.md` (both cite chapter
   6's *canonicalisation count* as a standing adjudication, one of them saying
   `task_tree.rs` *asserts* it), `21-what-could-not-move.md`'s tally,
   `concept-index.md`, and `docs/specs/grove-loop-book-structure.md`. Chapter
   21's chapter-6 table row at line 53 was checked and **not** changed: it says a
   reported path is *the caller's own spelling of the root*, and the caller is
   the module's, not the operator.

5. **Two early-use rows were owed and one was rewritten, because the corrected
   comment names a symbol the old one did not.** The header now names `target`,
   which chapter 6 owns, so chapter 5 acquires a forward reference and a row
   sorted between `entry_path` and `brief_chain, kind_in` — same chapter, same
   anchor, same owner slice, ordered bytewise on the symbol cell. Chapter 6's new
   paragraph names `existing_path` to say what *resolved to an entry* excludes;
   that is chapter 9's, so a second row was added at
   `06-paths.md#canonicalise-to-compare`. `leaf_entry`'s row was rewritten: it
   said *the module header … says this is the only such place*, which is now
   false of the bytes it describes. `Workspace::resolve` is named on chapter 6
   for the first time and is **not** owed a row — the ledger's rule is a symbol
   whose source belongs to a later **slice** of this book, and `jj-workspace` is
   another book's corpus.

6. **The spec's *Known in advance* went from four claims to five, rather than
   keeping this one on the other side of its line.** The list's own stated
   criterion is *claims a leaf has corrected at source*, and the only reason
   canonicalisation sat outside it was that no leaf had. Leaving it out once the
   fix landed would have made the exclusion incoherent, so entry 5 was written on
   the same terms as entry 4 and chapter 21's tally follows it from four to five.
   The alternative — keep four and reword the exclusion — was rejected because it
   preserves a number at the cost of the rule that produces it.

7. **The one in-session reviewer paid, all ten findings were valid, and three of
   them were the defect class this leaf exists to remove, re-committed in the fix
   for it.** None was noise, a trade-off, or a contract I had stated unclearly.
   The three that mattered most:
   - **A count I invented while deleting one.** Chapter 21's replacement sentence
     read *chapters 11, 13 and 14 each carry one*, taken from the spec's own
     wording and tightened into a tally. Chapter 14 carries two
     (`14-finishing.md` line 204), chapter 11 carries three, and the list omits
     chapters 12, 20 and 6 itself. Both the sentence and the spec's — which I had
     preserved rather than written — now name no chapters at all.
   - **The same move in the new ledger row**, which called `existing_path` *the
     third function in this module that interprets a caller's path argument*.
     Enumerated, it is six (decision 2), and three of the others also
     canonicalise nothing, so *the one that* was false too. Both the row and
     chapter 6's paragraph now state the property and not the position.
   - **A misquotation that made the surviving half broader than the bytes.** I
     wrote that the clause's second half was *true of both functions*. What was
     true of both is the operation; the wording said *a caller's spelling of **a
     leaf***, and `target` resolves *a leaf file or a node directory alike* and
     returns `Target::Root` for the root. Three pages said it and all three now
     distinguish the operation from the noun — which is a second narrowing in the
     same clause that nobody, including `paths-k142`, had called out.
   Also valid and fixed: chapter 21 still said *two of that second group have
   since landed, both of them chapter 1's* nine lines above the paragraph this
   leaf rewrote to say otherwise; the spec's entry 5 named chapters 5 and 6 where
   chapter 8 had carried an adjudicating paragraph too, against entry 2's
   *chapters 18 and 20* precedent; `existing_path` was described as handing its
   argument back when two of its three branches return a path they joined; the
   decision log above miscounted the widened sweep (four crates and four call
   sites, not six and three), stated the test-module argument backwards, and
   cited `resolve.rs` line 277 for a comparison at 282; and one rewritten
   sentence in chapter 20 was left at 100 columns in a page wrapped at 80.

8. **The gate was green first time and hung on the re-run, and the hang is a
   defect this leaf externalises rather than fixes.** The first
   `bash scripts/check.sh` printed **check: all 8 principal checks pass**, exit
   0, six books valid, `grove-loop` at **10,542 resolved lines** — the same
   figure as before the change, which is the control for decision 4. The re-run
   after the reviewer's fixes — all of them Markdown, with `task_tree.rs`
   unchanged since the green run — hung for thirteen minutes in
   `cargo test --locked --workspace`, and `sample`ing the test binary named it:
   `an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal`,
   `crates/grove/tests/loop_driver.rs` line 1009, blocked in `read()` inside
   `Child::wait_with_output`. **The timeout path is itself unbounded.** The
   fixture waits 25 s for `epoch_held`, misses it under load, kills the driver
   and then calls `wait_with_output` to build its panic message — but that reads
   stderr to EOF, and EOF needs every writer to close. `lsof` showed the write
   end held by `/bin/sh …configured-command.sh` (pid 17690, reparented to
   launchd), the session the driver had launched, which outlived the killed
   parent and inherited the pipe; both direct children were already zombies. So
   the test hangs forever *inside the code that reports its own timeout*, and a
   slow machine — load average 7.79, three other grove sessions live — turns a
   25 s timeout into an infinite one. This is the `driver-lease-fixture-timing-k85`
   class with a sharper cause than *timing*, it is not this leaf's corpus, and no
   source of this book changed, so it goes to the tree as its own leaf rather
   than inline: `driver-test-timeout-path-unbounded-k194`. The stuck run and its
   one orphan were killed — nothing else on the machine was touched — and the
   re-run printed **check: all 8 principal checks pass**, exit 0, six books
   valid, `grove-loop` again at **10,542 resolved lines**, leaking no orphan of
   its own. **The leak is much older and wider than the hang, and the leaf says
   so:** counted after the clean run, 24 `configured-command.sh` processes are
   reparented to launchd with elapsed times from 18 hours to 8 days. That is
   self-reinforcing — idle orphans are load, and load is what pushed the fixture
   past 25 s — which is why the leak is in k194's scope and not only the wait.

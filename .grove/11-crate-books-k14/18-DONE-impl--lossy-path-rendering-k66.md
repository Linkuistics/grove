# lossy-path-rendering-k66

## Goal

Stop `jj-workspace` from silently converting a path it cannot render into a
scope that matches nothing, and cover the one branch of `relative` no test
reaches — landed as a corpus change the book contract permits.

## Context

- `crates/jj-workspace/src/lib.rs:265-272` renders a workspace-relative path into
  a jj fileset by joining its components with `/` and converting each with
  `to_string_lossy`. That is the crate's only lossy conversion, and it has to
  happen somewhere: a fileset is an argument in a command line, and jj's
  arguments are text.
- **The consequence is silent, and it is the one shape this crate refuses
  everywhere else.** A path whose bytes are not valid UTF-8 becomes a string
  containing replacement characters, so the fileset names a file that does not
  exist. Measured on jj 0.44.0 against a fileset that matches nothing:
  `jj file list` prints nothing on stdout and exits 0, and `jj commit` warns on
  stderr, takes an **empty** commit and exits 0. So `is_tracked` answers `false`
  for a file that is tracked, and `commit` returns a `Commit` naming an empty
  change while the caller believes its work is committed. No refusal is raised
  at any point.
- The crate's own rule is the argument for changing it. `control_dir` refuses a
  namespace it cannot honour *rather than quietly reinterpreting it*
  (`lib.rs:130-133`), and `validated_namespace` refuses `'\0'` precisely so an
  obscure operating-system error becomes a refusal that names the problem. A path
  that cannot be rendered is the same class and is handled the other way.
- **The reachability is platform-dependent and that is not a reason to leave
  it.** APFS rejects a filename that is not valid UTF-8, so the case is hard to
  construct on macOS; Linux filesystems accept arbitrary bytes, and grove runs
  there. Whether to refuse (a new `Refusal` case, or `Refusal::outside_workspace`'s
  sibling) or to pass the path to jj as bytes is the decision this leaf makes;
  refusing is the shape consistent with the rest of the crate, and passing bytes
  is not available while the fileset is a `&str` in an argv array.
- **A second, adjacent gap belongs in the same commit.** `relative`'s fallback
  branch (`lib.rs:245-259`) canonicalises the *parent* so a path the caller has
  just deleted still resolves. The crate's suite reaches that branch through no
  test: `a_deletion_is_committable_after_the_path_is_gone` passes a relative
  path, which takes the textual `strip_prefix` branch instead. The behaviour was
  measured while drafting `scope-and-commit-k59` — an absolute path through a
  symlinked ancestor whose leaf is deleted commits the deletion, and one whose
  *parent* is also deleted is refused with `UnresolvablePath`, whose message
  talks about broken symlinks rather than about a removed directory — and
  `05-scope-and-commit.md` records both. Both belong to the same two functions
  and the same fragments.

## Done when

- A path that cannot be rendered as a fileset is refused rather than turned into
  a scope that matches nothing, with a message naming the path and the reason,
  and a test asserts it (`#[cfg(unix)]`, constructing the name with
  `OsStr::from_bytes`).
- The fallback branch of `relative` has a test: an absolute path reached through
  a symlinked ancestor, whose leaf has been deleted, commits the deletion.
- If the `UnresolvablePath` message is changed to fit the removed-parent case,
  the change lands here too; if it is left alone, the reason is recorded.
- The corpus-freeze rule in `.grove/BRIEF.md` is honoured: **one commit** carries
  the source change, every affected book ledger and page, and a green validator
  run over every book it touched. `lib.rs:146-274` is the
  `scope-tracking-and-commit` block, owned by chapter 5 of the `jj-workspace`
  book and split into fourteen literal fragments — a line added anywhere in
  `relative` moves the boundaries of `relative-absolute`,
  `relative-strip-or-canonical-parent`, `relative-root-is-not-a-scope` and
  `relative-render`, their fragment-index rows, and the block's own range. A new
  `Refusal` case moves `refusal.rs` and chapter 6 with it.
- `bash scripts/check.sh` passes.

## Notes

**Placed after every crate book deliberately, for the reason `jj-docs-url-k64`
and `jj-owned-names-k65` carry.** Editing a byte of a frozen root while a book
that quotes it is being written invalidates the ranges the freeze protects. Run
this once the books that own `lib.rs` and `refusal.rs` exist and can be re-proved
in the same commit.

**This is not a request to make paths non-UTF-8-safe throughout.** The crate
speaks to jj through a command line and will keep doing so. What is wrong is not
the conversion; it is that the conversion cannot fail.

## Decisions (running log)

1. **Refuse, rather than pass bytes — and the deciding evidence is jj's own, not
   this crate's shape.** The task file framed the choice as *refuse (consistent
   with `control_dir` and `validated_namespace`) versus pass the path to jj as
   bytes*, and treated "bytes are not available while the fileset is a `&str`"
   as the reason to refuse. That reason is weaker than it needs to be —
   `Command::arg` takes `OsStr`, so the seam *could* have been widened. The
   decisive fact is upstream: **jj cannot address such a path at all.** jj
   v0.44.0's changelog, *Fixed bugs*: *"Snapshotting no longer fails if the
   working copy contains a path whose name isn't valid UTF-8. Such paths can't
   be tracked, so they are now skipped and reported as a warning instead"*
   ([jj v0.44.0 CHANGELOG](https://raw.githubusercontent.com/jj-vcs/jj/v0.44.0/CHANGELOG.md),
   [#9774](https://github.com/jj-vcs/jj/issues/9774)). Read at the primary
   source — the tagged `CHANGELOG.md` — rather than from training data, and
   0.44.0 is the version the crate's other measurements are pinned to. So
   widening the seam to carry bytes would buy nothing: it would hand jj a path
   jj declines to track. Refusing is not merely the consistent shape, it is the
   only shape that says something true.

2. **A new `Refusal` case, named `PathNotText`, appended to the scope group —
   not a reuse of an existing one.** Each of the three candidates for reuse
   states something false: `OutsideWorkspace` says the path is outside a
   workspace that in fact contains it; `NotScoped` says no scope was named when
   one was; `UnresolvablePath` requires an `io::Error` there is none of and
   talks about broken symlinks. The name pairs deliberately with the existing
   `OutputNotText`: the crate has exactly two places where bytes must become
   text — jj's stdout on the way out, and a caller's path on the way in — and
   naming them alike makes that a visible pair rather than a coincidence. It is
   appended after `NotScoped`, so chapter 6's *Scope's two* becomes *Scope's
   three* and every other group's order is untouched.

3. **The refusal names no jj command, and that is not a departure.** `refusal.rs`
   opens with *"it names what is wrong, where, and the command that fixes it"*,
   but three of the ten existing cases already name no command — `Namespace`,
   `OutsideWorkspace` and `NotScoped` — because there is no jj command for what
   is wrong. This is a fourth of that shape: jj cannot track the path at all, so
   the remedy is the filesystem's (rename it), and inventing a jj command would
   be worse than naming none.

4. **The changelog URL is cited in a code comment, not in the refusal message.**
   `jj-docs-url-k64` closed a stale URL in a user-facing message and recorded
   that the crate emits exactly one external URL. A second one is a second thing
   to rot, on a surface no test reads. So the *message* states the durable fact
   (jj does not track a path whose name is not valid UTF-8) and the *comment
   beside the conversion* carries the citation, which is where the `impl` skill
   asks a verified framework decision to be recorded anyway.

5. **Each new test was seen to fail under the negation of its own claim, before
   it was credited.** Three controls, each one mutation, each run:
   - restore `to_string_lossy` in the render loop → the non-UTF-8 test fails
     alone (30 passed, 1 failed);
   - `panic!` on entry to `relative`'s fallback branch → the symlinked-deletion
     test fails, **and so does
     `a_path_outside_the_workspace_is_refused_rather_than_answered`**;
   - drop the `canonical(parent)?` call while keeping the branch → the
     symlinked-deletion test fails **alone** (30 passed, 1 failed).

6. **The task file's premise was half wrong, and the third control is what shows
   it.** It says the suite "reaches that branch through no test". The branch is
   in fact reached — by the outside-the-workspace test, which enters it and
   takes its *error* exit. What no test reached is the branch's **success**
   exit: canonicalising the parent, stripping it against the root, and rejoining
   the leaf name. That is the clause the new test holds, and it holds it alone.
   The distinction matters because a coverage instrument would have shown the
   branch as covered.

7. **Line counts moved, so unlike `jj-docs-url-k64` this one does owe ledger
   edits.** `lib.rs` 343 → 359 (+4 doc lines in `relative`, +12 in the render
   loop), `refusal.rs` 230 → 254 (+2 variant, +6 constructor, +15 `Display` arm,
   +1 `source()` arm). Blocks: `scope-tracking-and-commit` 146-274 → 146-290,
   `gate-main-repo-and-canonical` 275-319 → 291-335, `namespace-validation`
   320-343 → 336-359, `refusal-source` 1-230 → 1-254. Every block above
   `scope-tracking-and-commit` is untouched, so chapters 1, 2 and 4 keep every
   fragment range they have.

8. **The `UnresolvablePath` message is left alone, and the reason is recorded as
   the `Done when` asks.** The removed-parent case reaches it through
   `canonical(parent)?`, and its text talks about *a broken symlink or a
   directory that has been removed underneath this process* — the second clause
   already covers a removed parent exactly. Chapter 5's complaint was that the
   message "talks about broken symlinks rather than about a removed directory";
   re-reading the string shows it names both. Nothing to change.

9. **The fan-out was five surfaces in the source's own book and four more
   outside it.** Inside `docs/walkthroughs/jj-workspace/`: the fragment fences
   (the only surface `book-check` expands), the ledger rows in
   `walkthrough.toml` and `source-index.md`, the prose in chapters 5 and 6 that
   argues *from* the defect, chapter 7's evidence table and console blocks, and
   `concept-index.md`. Outside it: the `698` line inside the
   `bash scripts/check.sh` transcript reproduced in the assembly chapter of
   **each of the other four books** — `grove-llm`, `grove-loop`, `keyed-launch`
   and `overview` — which no check reads. That last class is the one a
   book-local sweep cannot find, because the falsified token is in a *different
   book*.

10. **What was deliberately not rewritten.** `docs/evaluations/**` is a frozen
    measurement record of runs that happened, so its `698`s stay. `.grove/BRIEF.md`'s
    corpus table says *counts are of the corpus as frozen* and is the baseline
    chapter 7 now measures the +40 against, so it stays too. Two ADRs cite 698
    as incidental sizing in an argument about something else; neither claim
    turns on the number. `docs/specs/jj-workspace-book-structure.md` is the
    human's pre-authoring structure brief and keeps its tables, with one added
    note saying the corpus moved and naming the artifact that is now
    authoritative.

11. **Two stale claims in chapter 7 were corrected rather than leafed, because
    the sentences carrying them had to be rewritten anyway.** (a) *"The manifest
    declares only the six whose first use is in chapter 1, and the other five
    live in the book's own ledger"* — `early-use-scope-k63` moved two rows into
    the manifest (`jj diff` on that change shows +12 lines in
    `jj-workspace/walkthrough.toml`) and, by its own decision 4, deliberately did
    not edit the frozen book's pages. The split is now eight and four, and the
    paragraph says what happened. (b) The chapter's Display size, *roughly eighty
    lines*, is now roughly a hundred. Neither is a source defect and neither
    would have survived a correct edit of the sentence beside it.

12. **Three claims I wrote were wrong when enumerated, and enumeration is the
    only reason they did not ship.** *"`PathNotText`'s arm is the only one in the
    file carrying a comment"* — there are three (`NotAWorkspace`,
    `PathNotText`, `CommitNotRecorded`). *"Ten of the eleven messages name jj"* —
    numerically right, wrong member: `NotScoped` is the one that does not mention
    jj, and `PathNotText` does. *"The crate's sixteenth refusal site"* — there
    are seventeen construction sites and it is the twelfth in `lib.rs` by line
    order. All three were uniqueness or count claims, which is the class that
    fails most often here.

13. **The leaf's one in-session reviewer was spent on the prose, and it paid.**
    The claim put to it: *every count, uniqueness claim and cross-reference in
    the changed pages holds against the source as it now stands.* One fresh
    context, given the contract and an adversarial *find what is wrong* prompt,
    with the conclusion stripped. It returned fourteen findings; each was
    re-derived here before being acted on. Classified:

    - **Valid and actionable — my sweep missed them (nine).** `README.md`'s *the
      ten refusals*; chapter 5's *sixty-three lines* for `fileset` plus
      `relative` (now 79); chapter 7's assembly-table *139 lines* for chapter 5's
      slice; chapter 7's *eighty-line `Display`*; chapter 7's *that shape appears
      ten times*; chapter 6's *the last sixteen lines* (now 17); chapter 6's *the
      three arms are the file's only place where a remedy is not jj's to run*;
      chapter 6's *the file's only comment about a message*; chapter 1's *roughly
      seven hundred lines*. Every one is a count or a uniqueness claim, and the
      first five are cases of the same failure I had already recorded at
      decision 9 — a falsified number in a *different* section from the one I
      edited.
    - **Valid, pre-existing, and worsened here (one).** The structure brief's
      *755 lines* of tests was already wrong by 32 before this leaf and is now
      wrong by 117. The note I had added there was scoped to two tables; it is
      widened to cover every count in the brief and states 872.
    - **A contract I stated unclearly (two).** I told the reviewer an ADR
      describes current state, so it flagged `698` in
      `a-book-carries-no-asset.md` and `the-editorial-pipeline-is-four-kinds.md`.
      Both sentences describe **what the pilot measured**, and the pilot measured
      698 lines; rewriting them to 738 would make them false about the thing they
      actually assert. Left alone, for the same reason `docs/evaluations/**` is.
    - **Valid but not this leaf's (two).** Chapter 4's *`control_dir` is one of
      five methods on `Workspace`* and chapter 6's *eight of which argue for the
      three that declare it* are both wrong and both untouched by this change —
      no method was added and fragment `refusal-opaque-type` did not move. Cut as
      `jj-workspace-method-counts-k184`.

14. **Nine of the reviewer's nine actionable findings were in prose I had already
    swept.** I had enumerated the *changed* sentences and the tokens my edit
    falsified; what I had not done was re-derive every number in the pages those
    sentences live in. The lesson is sharper than decision 9's: when a source
    edit moves a line count, the sweep's unit is the **page**, not the sentence,
    because a page states the same quantity in several places and in several
    spellings — *139*, *a hundred and thirty-nine*, and *sixty-three lines*
    naming a span that contains it.

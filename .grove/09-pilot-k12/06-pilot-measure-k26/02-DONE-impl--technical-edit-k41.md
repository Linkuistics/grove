# technical-edit-k41

## Goal

Run the **technical edit** over the `jj-workspace` book: is what the book says
about the code true, complete for its scope, and not silently stale against the
frozen corpus. One commit.

## Context

- The authority is `crates/jj-workspace/` as frozen — `src/lib.rs` (343),
  `src/refusal.rs` (230), `src/jj.rs` (81) and `Cargo.toml` (44). `tests/` is
  evidence, not a root, and is legitimate reading for checking a claim.
- `CONTEXT-MAP.md`'s argument that `jj-workspace` is deliberately not a bounded
  context, and `docs/adr/jj-is-the-only-lane.md`, are the two places the book is
  most likely to have got the crate's boundary subtly wrong.
- The preregistration `pilot-preregistration-k24` committed is binding and not
  reopenable here. Read its attribution rule before you start and satisfy it as
  you go; reconstructing afterwards what a stage changed is what the one-commit
  boundary exists to make unnecessary.
- The baseline this stage is measured against is the previous stage's commit, and
  the record of what this stage did is the diff between that commit and yours. So
  this stage lands in **exactly one commit**, carrying its edits and nothing else.
- The corpus is frozen: do not edit `crates/jj-workspace/`. A defect found here
  becomes its own leaf under the root brief's cross-book rule.

## Done when

- Every claim the book makes about behaviour is either checked against the source
  or against a test, and the ones that could not be checked are named.
- Final validation over `docs/walkthroughs/jj-workspace/` is green — this stage
  leaves the book provable, not merely improved.
- The stage's record exists in the form the preregistration's attribution rule
  requires.
- `bash scripts/check.sh` passes.

## Notes

**A wrong claim about the code is a book defect, not a code defect.** If the
source is actually wrong, the freeze holds and it becomes its own leaf under the
root brief's cross-book rule — this session does not fix it.
## Decisions (running log)

1. **Frozen inputs re-checked before any edit; all nine unmoved.** The four corpus
   roots, both specs, `docs/USAGE.md`, `CONTEXT.md` and the `ordinal-fs-tree` book
   carry the digests the preregistration froze, and the book directory carries
   `669eee6f…`, the developmental edit's recorded `after`. The preregistration
   this session read is `b9ee9f8e…` — the same value every earlier stage recorded.

2. **Every test name the book cites exists, and every consumer line reference
   resolves.** All twenty-eight `workspace.rs` tests and the one `environment.rs`
   test were enumerated and diffed against the names the pages cite: no citation
   names a test that is not there. `grove-loop/src/lib.rs:81`,
   `session_config.rs:194`, `session_config.rs:354`, `grove-llm/src/cli.rs:452`,
   `grove-llm/src/main.rs:1` and `ordinal-fs-tree/src/plan.rs:255` each name the
   line the page says they do.

3. **`.jj/.gitignore` is colocation-dependent, and the book presents it as
   unconditional.** Measured on jj 0.44.0, the version the book measures against:
   `git.colocate` defaults to `true`, so a stock `jj git init` colocates and its
   `.jj/` holds `repo`, `working_copy` and `.gitignore`; a workspace initialised
   with `--config git.colocate=false` — which is exactly what the crate's own
   `native()` fixture builds — holds only `repo` and `working_copy`, and this
   repository's own `.jj/` is one of those. Chapters 1 and 2 draw the carried tree
   with two entries and chapters 4, 5 and 6 draw the same tree with three, while
   declaring it native. Fixed as one `book-wide` `T2`.

4. **The `.gitignore` collision reproduces, but only in a colocated workspace.**
   Driven through the crate from a scratch binary outside the repository:
   `control_dir(".gitignore")` in a colocated workspace returns the `ControlDir`
   refusal the book quotes verbatim, *File exists (os error 17)*; in a
   non-colocated one it returns `Ok` and creates the directory. So chapter 7's
   abdication verdict on refusal 4 stands, and the worked example has to say which
   workspace it is standing in.

5. **The two findings `developmental-edit-k40` handed forward are both real and
   both fixed here** — the wrong reason string at `06-refusal.md#the-namespaces-two`
   (the source reads *it names a directory other than itself*, confirmed by running
   `control_dir(".")` and `control_dir("..")` through the crate), and the false
   ordering claim at `04-namespace.md#worked-reservation`, fixed by reordering the
   three cases into the order the guards actually run rather than by weakening the
   heading, so the section agrees with *Four refusals, in the order they run* below it.

6. **`grove-loop` does not match on `ordinal_fs_tree::Refusal`.** An enumerate-then-
   classify sweep of every `Refusal::` occurrence outside `crates/ordinal-fs-tree/`
   and `crates/jj-workspace/` returns two lines, both inside a string literal or a
   comment; the positive control (the same pattern inside `ordinal-fs-tree`) returns
   27, and a deliberately mutated pattern returns none. `docs/ARCHITECTURE.md`'s
   *How an `ordinal-fs-tree` refusal reaches an operator* says the same thing from
   the other side. Chapter 6's counterexample is corrected — but its subject is
   another crate's source, which is none of the three targets `T2` names, so it is
   recorded as an unclaimed change with the taxonomy gap stated.

7. **Claims verified and left alone**, so the report can see what was checked
   rather than only what moved: the secondary workspace's `.jj/repo` is a
   nineteen-byte file reading `../../main/.jj/repo`; `JJ_EMAIL` does override the
   configured `user.email` (with a no-`JJ_EMAIL` control, once the working-copy
   author confound is removed); `jj file list` on an absent path exits 0 with the
   warning on stderr; `jj commit` on a fileset matching nothing warns, takes an
   empty commit and exits 0; the `NotRunnable` URL still answers `301`; and the
   ADR's Git-lane cost really is "four thousand lines".

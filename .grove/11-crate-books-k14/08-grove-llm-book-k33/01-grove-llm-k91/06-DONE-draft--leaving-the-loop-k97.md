# leaving-the-loop-k97

## Goal

Draft chapter 6 of the `grove-llm` book: slice `admit-before-signal`,
`06-leaving-the-loop.md`, owning `verbs-leaving` (`cli.rs` 248–289),
`args-complete` (311–322) and `handlers-leaving` (438–483). After this slice
every source line is resolved and only the final-only chapter remains.

## Context

- Draft stage, child 6 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *6 · Leaving the loop* section. Thesis: the completion
  channel is resolved and checked against the admitted epoch **before** it is
  written, because an answer that came back with the signal would come back too
  late; and `finish-commit` reads the operator's handle by the type that owns
  the grammar, lenient on the key, and quotes the operator's own spelling once.
  Cover `CompleteArgs`; `cmd_complete`'s order and its two endings, `Wrote` and
  `NoLoop`; the help's argument that the verb writes a flag and returns;
  `cmd_finish_commit`; and that these are the two verbs that open no tree.
- Link the glossary at `loop-control-channel` beside the channel and the guide
  at `usage-finish` beside `finish-commit`.
- The required example anchor is `worked-complete`: `complete` with
  `GROVE_SIGNAL_FILE=/work/atlas/.jj/grove/signal-3f9c2a7e5b1d4c8890aa61e0f27b4d13`
  — the channel resolved, checked against the admitted epoch, written, and *the
  loop will start the next task* on stderr; the same verb with no channel —
  `NoLoop`, and the instruction to exit manually. `finish-commit finish-k0001`
  is a second, shorter trace: accepted leniently, committed as `finish-k1`, the
  change id on stderr. Measure the `NoLoop` line against the built binary; the
  `Wrote` path needs a live driver and is held by
  `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` in
  `crates/grove-loop/tests/driver_lease.rs`, which the page cites.
- Evidence: `complete.rs` and `finish_commit.rs` here. Mark the rows this slice
  owns (`Handle`; `Signalled`; the two handlers that open no tree) `explained`.

## Done when

- The fragments for the three blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through admit-before-signal --check all` is valid: 1,017
  resolved lines, 0 deferred, `final=false`. The repository Markdown sweep
  passes. `scripts/check.sh` stays red on `book-check` alone, by design: the
  final run still reports the seventh page missing.

## Decisions (running log)

**1 · The structure brief is the precondition and was read first.**
`docs/specs/grove-llm-book-structure.md`, section *6 · Leaving the loop*,
states this chapter's reader outcome, section plan and emphasis. `grove-draft`
and the editorial family file were read from `plugins/grove/skills/` in the
working tree; the plugin cache still has no `grove:grove-draft`.

**2 · The finish trace carries the session's own grove to its finish, so the
live handle is `finish-k9` and the lenient spelling `finish-k0009`.** The
brief's *`finish-commit finish-k0001` … committed as `finish-k1`* was written
before the carried values existed, and the brief itself leaves those values to
the orientation slice. The carried tree at the end of chapter 5 holds `k3`,
`k4` and the `03-cache-k5/` node's `k6`, `k7` and `k8`, so once the review leaf
is retired the driver appends its finish sentinel at the root with the next
key — `04-finish--finish-k9.md`. `finish-k1` would name a leaf the carried tree
never held. The point the brief wanted — accepted leniently, committed
canonically — is preserved exactly, and the mismatch refusal shown beside it
uses `other-k007`, the spelling `finish_commit.rs` itself tests. Rejected:
inheriting chapter 5's tree without its pruned node, which would make the key
arithmetic disagree with the page before it; and a second grove shaped like the
test's fixture, which breaks the carry decision 17 of the node brief
established.

**3 · The fragment partition is six literals under three composites.**
`verbs-leaving` splits at the variant boundary (248–271, 272–289);
`args-complete` is one literal over its whole block (311–322), as
`verb-root-init` is; `handlers-leaving` splits into `handler-finish-commit`
(438–458, the trailing blank inside it), `handler-complete-admit` (459–466: the
comment, the channel resolved, the guard asked) and `handler-complete-endings`
(467–483: the write and the two endings), cut at the `match` so the order the
chapter is named for is one fragment and the rendering the other. Bodies are
spliced from the source by line range, not retyped.

**4 · The page reads `complete` first, then `finish-commit`, then the help.**
Opening (thesis and the loop-control-channel premise), the worked example
(`complete` under the driver, without a channel, and misdirected; then the
finish trace), `cmd_complete` in two fragments, `CompleteArgs`,
`cmd_finish_commit`, and the two doc comments last as the catalogue, under the
example-before-catalogue rule. `complete` leads because it is the carried
session's own verb and the chapter's third order; `finish-commit` is the
driver's and takes the shorter trace. Rejected: file order, which puts the
driver's verb and 24 lines of its help before the session's.

**5 · Every transcript line was measured against the built binary at
`20.1.0`, and the driver-side lines under a live `grove` driver.** Without a
driver (the signal variable unset, on scratch Jujutsu workspaces shaped like the
carried tree): `complete` with no channel, with the empty-string channel every
cargo-launched process sees, and in a directory with no `.jj/` at all — the
`NoLoop` line, exit `0`, nothing written; `complete --signal-file <path>` and
`--done` — the `Wrote` lines and the file's bytes (`relaunch` or `done`, one
newline); `--signal-file` into a directory that does not exist — the runner's
write refusal; the channel in the environment with no epoch record — refused
at admission as *stale Grove session*, before the handler; `finish-commit`
on the carried tree with the review leaf live, on a terminal grove with
`other-k007`, `finish-k99`, a late leaf, then `finish-k0005` (committed as
`finish-k5`, four `D .grove/…` lines and nothing else in the teardown commit,
the unrelated working-copy edit still uncommitted), then again on the absent
tree; an untracked tree; a tree that is not jj-enabled; and four handle
shapes `Handle::parse` refuses. Under a live driver over a tree with one live
leaf and a fake harness: `pick` admitted; `complete --signal-file` naming
another channel in the control directory and one under `/tmp` — both refused
with *does not match the admitted session epoch*, neither file created; the
stale channel in the environment — refused at admission; then `complete` on
the live channel — the `Wrote` line, the file holding `relaunch`, and the
driver terminating the harness, removing the file and launching the next
session. The symlinked root was not measured and is stated from its test.

**6 · The driver test holds the check and not the write, and the page says
which test holds what.** `grove_llm_admits_only_the_live_epoch_while_version_remains_exempt`
requires the misdirected `complete` refused with no file created; the `Wrote`
path is held at the library seam by `relaunch_signal_is_read_back_as_relaunch`
and `done_signal_is_read_back_as_done` in this crate's `complete.rs`, through
a real `keyed_launch::Channel`. The three stderr wordings — the two endings and
*committed as* — are held by the source and no test, checked with a negative
grep over both test directories, a positive control (*cannot finish while live
work remains* in `finish_commit.rs`), a cross-tree control (the wordings in
`cli.rs` and `docs/USAGE.md`) and a deliberately mutated pattern that came back
empty.

**7 · Two claims in the corpus are adjudicated on the page, and one is
leafed.** `cmd_finish_commit` resolves the working tree twice — `worktree()`
and then `Workspace::resolve(&worktree)` on the root it returned — so the
second call's context, *cannot commit the finished grove*, is not what a tree
that is not jj-enabled sees: `worktree` refuses first, and the measured
refusal is the seam's with no context line. The page states it beside the
fragment and names no leaf, because nothing is false. `Complete`'s help says a
session *not under `grove do`*; `do` is a verb `removed_surface.rs` records as
removed from both binaries, and the driver is bare `grove`, which is what the
live measurement ran. A stale name in a frozen-corpus comment, one word, is
the shape `next-steps-comment-lane-k100` was cut for, so it is cut the same
way: an `impl` leaf in `crate-books-k14` ahead of `architecture-residue-k75`,
to land with this chapter's `verbs-complete-help` fragment once the book is
green.

**8 · The mismatch refusal is measured as the comment describes it, and the
page says which frame carries which spelling.** `finish-commit other-k007` on
the terminal grove prints the handler's context — the operator's own
`other-k007` — and, under it, the loop's *requested finish handle other-k7
does not match the live finish leaf finish-k5*, canonical. The test that holds
it requires both the raw text and the live handle in stderr, and the raw text
is there only because of the handler's frame.

**9 · The one in-session review was spent, and it paid.** A fresh context was
given the page, the corpus, the loop's `verbs.rs`, `driver_lease.rs`,
`complete.rs`, `tree_lifecycle.rs` and `task_name.rs`, the cited tests, the
structure brief, the book contract and the built binary, with a *find what is
wrong* prompt and the instruction to reproduce every transcript itself with the
signal variable unset. It reproduced all six fragments byte-exact, every cited
line number, and every transcript, and returned twenty findings. Nineteen were
valid and are fixed; one was wrong; three stylistic ones are handed to
`copy-edit` rather than absorbed.

The nineteen, by class. **Four measurement-attribution defects**, the most
serious being that the page claimed the epoch-mismatch refusal was reachable by
changing only the environment: it is not — a wrong channel in the environment is
refused at admission, and only a `--signal-file` disagreeing with the live
ambient channel reaches `require_signal_path`. The provenance sentence
consequently mis-stated which transcripts needed a live driver, and the reviewer
confirmed by measurement that with the driver killed the same invocation answers
*session epoch is inactive* instead. **Six overstated evidence claims**: the
`jj restore .grove` and `jj undo` remedies were called measured when neither is
reachable without injecting a failure, and the deletion-failure branch is in
fact the one refusal that does *not* leave the tree standing; the two tests
credited with the *names the way out* promise hold preconditions rather than
that promise; `an_explicit_channel_passes_through` was credited with a
precedence it cannot test, since `.cargo/config.toml` force-clears the variable
so no non-empty ambient value is ever in play (the precedence was measured
instead); the stale-token reading belongs to
`unrecognised_signal_content_is_treated_as_relaunch`, which the page had not
cited; and five, not six, of the seven refusal tests compare whole trees.
**Five wrong facts about the module**: `verbs::complete` does call
`signal_channel` itself, so the live path resolves twice; `root-init` and
`complete` are counterexamples to *every other verb takes an opening*; the
`match` arms return nothing and the `Ok(())` is the function's; `Handle::parse`
precedes the grove opening but not `worktree()`; and the empty-variable fact is
this repository's `.cargo/config.toml`, not cargo's behaviour. **Three
self-contradictions**: the table lead-in miscounted its own five rows and
inverted which vectors are identical, the closing paragraph asserted *neither
opens a grove* where the opening correctly says the call opens it, and one
reference counted the wrong step of a five-step test. **One unverifiable
superlative** about comment length, struck under decision 8 of the node brief.

**The one rejected finding** is the reviewer's claim that the commit-failure
path names no jj command, leaving the help's *`jj undo` if the commit is*
promise unkept. It is kept, one layer down: `jj_workspace`'s `CommitNotRecorded`
refusal names `jj undo` and `jj op log` (`crates/jj-workspace/src/refusal.rs`),
and `tree_lifecycle.rs`'s own doc comment points at that seam as the reason its
context line does not repeat them. The page now says so rather than leaving the
apparent gap.

After the fixes `book-check --through admit-before-signal --check all` is valid
— 1,017 resolved, 0 deferred, `final=false` — and `reference_navigation`'s
thirteen tests pass.

# the-grammar-k93

## Goal

Draft chapter 2 of the `grove-llm` book: slice `admitted-before-dispatch`,
`02-the-grammar.md`, owning `grammar-cli-and-enum-head` (`cli.rs` 35–65),
`enum-close-and-operation-label` (290–310), `run-admission-and-dispatch`
(412–437) and `openings` (863–903).

## Context

- Draft stage, child 2 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *2 · The grammar and the openings* section: the
  `#[command]` attributes — one version constant read by both binaries,
  `arg_required_else_help`, and why `command` is an `Option` that is never
  `None` past `parse`; `operation_label`; `run` — parse, the unreachable bare
  branch, the current directory, and `admit_ambient_session` **before** the
  exhaustive dispatch, with `--version` exempt only because clap answers it
  first; `worktree` through `Workspace::resolve`; `readable` and `writable`;
  and `absent` — one wording carrying its remedy.
- State what a session epoch is as this chapter's premise — one paragraph,
  linked to the glossary at `session-epoch` and the guide at
  `usage-driver-lease`, never a primer. Cite `driver-lease` beside admission and
  `stated-vcs` beside `Workspace::resolve`.
- The required example anchor is `worked-dispatch`: `grove-llm resolve
  rate-limit-k3` parsed, its operation label, the epoch admitted, the handler
  dispatched; the same argv one directory off (a directory with no `.grove/`),
  refused with `root-init` named as the remedy; and `grove-llm --version`
  answered before admission is reached. **Measure the refusal and the version
  line against the built binary**, as `the-surface-k78` did for the overview.
- **This page must carry `<a id="worked-dispatch"></a>` on its example** and
  state the four handler-family rows' minimum statements there: one handler per
  reading verb, per growing verb, per terminal mark, and the two that open no
  tree. Leave those rows `pending`; mark the three rows this slice owns
  (`SessionEpochGuard`; `Reading`, `Tree`, `Writing`, `TreeWrite`; `Workspace`)
  `explained`.
- **Known in advance**: `cli.rs` lines 39–43 say the package carries a `0.1.0`
  of its own; `Cargo.toml` line 3 is `version.workspace = true`. State the
  checkable fact beside the fragment; `grove-llm-version-comment-k83` owns the
  comment.
- This chapter owns none of the twelve variants. If it names the twelve, it does
  so after the worked example — the catalogue rule binds.

## Done when

- The fragments for the four blocks are defined on the page, the four
  source-index defers are replaced by inserts, their ownership rows read
  `resolved`, and the fragment index has their rows.
- `README.md` links the page, chapter 1's navigation gains a Next, and
  `concept-index.md` has this chapter's entries.
- `book-check --through admitted-before-dispatch --check all` is valid: 226
  resolved lines, 791 deferred. The repository Markdown sweep passes.
  `scripts/check.sh` stays red on `book-check` alone, by design.

## Notes

**Verified rather than reconstructed.** Every line the worked example prints
was measured against the built binary at `20.1.0`. Without a driver, in a
scratch Jujutsu workspace with no `.grove/` and `GROVE_SIGNAL_FILE` unset:
`resolve rate-limit-k3` fails with exit `1` and the two-line `absent` refusal
naming `grove-llm root-init`; `leaf-retire` on the same tree prints the
byte-identical refusal, which is the `writable` path; `--version` prints
`grove-llm 20.1.0` with exit `0`, there and in a directory that is not a jj
workspace; a bare `grove-llm` (and a bare `--`) prints the short help on
**stderr** with exit `2`, `--help` prints the long form on stdout with exit
`0`, and an unknown or empty verb is clap's usage error with exit `2`. Under a
live driver — the built `grove` run over a scratch grove with a fake harness
that records `GROVE_SIGNAL_FILE` and sleeps — `resolve` in the epoch's tree is
admitted and prints the leaf's path; the same argv in a second jj workspace
with the channel in the environment is refused at admission with *wrong
working tree for grove-llm resolve: session belongs to …, command resolved …*
and exit `1`; a stale nonce in the epoch's tree is refused with *stale Grove
session for grove-llm resolve: loop-control path does not match the active
epoch*; and `--version` succeeds under both. The page's transcript carries
those lines with the scratch paths replaced by the carried tree's, and says
so.

**The refusal's second line is held by no test.** `errors_when_grove_root_absent`
in `crates/grove-llm/tests/pick.rs` pins *grove root not found*; nothing
asserts *Scaffold one with `grove-llm root-init`*. The page states that.

**Proof.** `book-check --through admitted-before-dispatch --check all` is
valid: 4 files, 226 resolved lines, 791 deferred, `final=false`, on the first
run. `reference_navigation` (13) and `corpus_exception_inventory` (5) pass.

## Decisions (running log)

**1 · The structure brief was read first and is the precondition.**
`docs/specs/grove-llm-book-structure.md`, section *2 · The grammar and the
openings*, states this chapter's reader outcome, section plan and emphasis;
the node brief's decision 1 already recorded the same for the book. The skill
was read from `plugins/grove/skills/grove-draft/SKILL.md` and the editorial
family file from the working tree, as the two sibling drafts did — the plugin
cache still has no `grove:grove-draft`.

**2 · The page reads its four blocks in argument order, not file order:**
the grammar's head, the four openings, `run` inside the worked example, and
the enum's close with `operation_label` after it. `run` sits inside
`worked-dispatch` because the manifest fixes the four handler families' first
use at that anchor and `run`'s `match` is where the twelve handler names first
appear; placing the fragment in its own section ahead of the anchor would put
the first use where the ledger does not say it is. `operation_label` comes
after the example because its twelve-arm `match` is the one place the chapter
names all twelve verbs, and the catalogue rule binds. Rejected: file order
(35–65, 290–310, 412–437, 863–903), which would open the page on the enum's
close.

**3 · The fragment partition is twelve literals under four composites**,
each literal on an item boundary: attributes / struct / enum head (16, 12,
3); enum close / `operation_label` (2, 19); parse-and-bare-branch /
cwd-and-admission / dispatch (8, 2, 16); `worktree` / `readable` / `writable`
/ `absent` (8, 14, 9, 10). Blank lines lead the first fragment of block 35–65
and the enum head, and trail elsewhere, because the manifest's block
boundaries fix where the blanks fall. Bodies were spliced from the source
with `sed`, not retyped.

**4 · The `0.1.0` comment is adjudicated as a stale fact with a live
reason.** `Cargo.toml` line 3 is `version.workspace = true`, so the package's
`CARGO_PKG_VERSION` is the workspace's and a bare `version` attribute would
also answer `20.1.0` today; the comment describes the crate as created. The
page states both, names `the_two_binaries_report_one_version` as the pin, and
reproduces the comment as written. The rewrite is
`grove-llm-version-comment-k83`'s and the page does not name the leaf.

**5 · The example has three endings for one argument vector, and the wrong
working tree has two of them.** Under the driver's channel a session one
working tree off is refused at admission, before any handler; without the
channel it is a manual command and reaches `absent`. The task file's *refused
with `root-init` named as the remedy* is the second; the page shows both,
because a reader who only saw the second would take admission to be
inapplicable to the case the chapter is named for.

**6 · Two facts about `worktree`'s comment are stated beside it.** *No
caller here can spell the grove root a second way* holds for what is passed
to the loop and not for the module's text: `.grove` is joined three times
for display — in `readable`'s refusal, in `resolve`'s root answer (chapter
3's) and in `root-init`'s already-exists refusal (chapter 4's). A first draft
of the page said twice; the third was found by grepping the file for the
literal before the review, and the page was corrected.
And under a driver the working tree is resolved twice per verb — once by
admission inside the loop, once by `worktree` — which the wrong-working-tree
refusal's *command resolved* clause is the evidence for.

**7 · Admission's checks are stated as three facts, not five branches.**
The loop's `admit_session` compares the working-tree root and identity,
requires an active epoch whose channel is the ambient one, and probes the
lease; the page groups those as *the same working tree, the same channel, a
driver still alive* and lists the four refusal classes the loop's own doc
comment names. The loop's branches are `grove-loop`'s to explain.

**8 · The `--version` exemption is stated as a position, not a rule.** The
page says it three ways: `parse` is the first statement of `run` and exits
before line 420; the measurement that the same `--version` succeeds where every
verb is refused; and the last assertion of
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt`.

**9 · The one in-session review was spent, and it paid.** A fresh context was
given the page, the book's other pages, the corpus, the loop's root and
admission module, the store's openings, the cited tests, the contract and the
brief with a *disprove it* brief, checked about seventy-five claims and
returned twenty-one findings; every one was classified. Six were wrong and
are fixed: four test files reach `Cli::command`, not three
(`help_surfaces.rs` through `CommandFactory`); `.grove` is spelled three
times for display, not twice; the `jj-workspace` refusal was promised in a
table that did not carry it, and is now stated measured beside `worktree`;
`readable`'s comment says the loop answers vacancy because the driver
scaffolds one, and the driver scaffolds through the *exclusive* opening — the
page now checks the comment against the driver instead of repeating it; *every
handler that opens the tree calls `readable` or `writable`* had two
exceptions, `root-init` and `inherited_kind`, now named; and the two working-tree
resolutions under a driver cannot disagree, so the refusal is the first being
reported, not a disagreement. Seven imprecisions tightened: `parse` returns a
`Cli` whose `command` is `Some`, not the variant; admission makes five checks,
stated in three groups and now counted; the bare help lists `help` beside the
twelve verbs and the options; the helpers are at lines 863 to 903, not the
end of the module; *the one place the chapter names all twelve* was false
because `run`'s dispatch and the family table already do, after the anchor;
the early-use rows are the types, not the helpers; and `--help` and an unknown
verb, claimed measured, now have rows in the table. Two contract findings
fixed: `TreeWrite` and `Tree` are now explained in full beside `writable`,
which is what `explained` in the ledger requires; and *the through-line the
brief names* dropped its attribution to a document the reader cannot follow.
Two idioms rewritten. Noise, fixed anyway: the example heading now counts
what it shows; the enum head is *a fragment of three lines, one blank*.
Visible trade-off, unchanged: the openings section precedes the worked
example and reads four private helpers with a fragment each — the rule names
public operations, and the example's refusal ending cannot be followed
without them. One finding is the brief's, recorded in the node brief: it says
clap answers `--version` *before `run` is entered*, and `parse` is `run`'s
first statement. After the fixes `book-check --through
admitted-before-dispatch --check all` is valid (226 resolved, 791 deferred)
and `reference_navigation`'s thirteen tests pass.

**10 · `bash scripts/check.sh` ran after the last edit and is red on
`book-check` alone, by design.** Seven of eight checks pass, the three
finished books report `final=true`, and the `grove-llm` final run reports the
seventeen deferred blocks that chapters 3 to 6 own and the eleven `pending`
ledger rows a final scope rejects. No stage of the script was slow or flaky
this time; the run took under twenty-five minutes end to end.

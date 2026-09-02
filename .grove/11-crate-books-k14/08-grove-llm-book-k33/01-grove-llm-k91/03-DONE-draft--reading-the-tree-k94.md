# reading-the-tree-k94

## Goal

Draft chapter 3 of the `grove-llm` book: slice `information-not-error`,
`03-reading-the-tree.md`, owning `verbs-reading` (`cli.rs` 75–124),
`handlers-reading-and-rendering` (514–636) and `path-and-label-helpers`
(904–944).

## Context

- Draft stage, child 3 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *3 · Reading the tree* section. Thesis: an absent answer is
  information, so `pick` on a finished grove prints its diagnostic on stderr and
  exits zero, `resolve` reports not-found and ambiguity the same way, and a
  `DONE` or `ABANDONED` match prints its path **and** a note. Cover the four
  contracts as their help states them; `leaf_in`; `normalize_leaf_path` and its
  three cases; `label` and `no_live_leaves`; the root as the one answer with no
  entry behind it; and `render_resolution` — the crate's one pure function,
  `pub` and `#[must_use]` because it is unit-tested through the library target.
- Carry the through-line: the manifest's separate-crate argument is why
  `lib.rs` exists, which is why `render_resolution` is `pub`; `worktree`
  resolving through `Workspace::resolve` is why `normalize_leaf_path` may pass a
  bare grove-relative name through for the verb to join.
- Link the glossary at `task-tree-scheme` beside the reference grammar.
- The required example anchor is `worked-resolve`: `resolve` at full
  resolution with three renderings — the live leaf's path on stdout and nothing
  on stderr; the same handle after retirement — the path, and the note; and a
  bare slug two entries share — empty stdout, the keys on stderr, exit zero.
  `brief-chain` for the same leaf follows on the same values. Measure the
  renderings against the built binary on a scratch tree.
- Evidence: `pick.rs`, `brief_chain.rs`, `kind.rs`, `resolve.rs` and
  `resolve_rendering.rs` in this crate's tests. Mark the three early-use rows
  this slice owns (`Sought`, `Resolution`; `Reference`; `Outcome`) and the
  reading-handlers row `explained`.

## Done when

- The fragments for the three blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through information-not-error --check all` is valid: 440
  resolved lines, 577 deferred. The repository Markdown sweep passes.
  `scripts/check.sh` stays red on `book-check` alone, by design.

## Notes

**Verified rather than reconstructed.** Every line the two transcripts and the
three tables carry was measured against the built binary at `20.1.0`, without
a driver, on a scratch Jujutsu workspace shaped like the carried tree and then
moved through the states the chapter needs — the leaf live; `k4` added and
`k3` retired; both retired; a node and an `ABANDONED` leaf added. Measured and
stated on the page: the live path with empty stderr; the `DONE` note repeating
the path; the ambiguity listing in walk order with the *(retired)* tag; the
`no live leaves` line on stderr with exit `0` from all three of `pick`,
`kind` and `brief-chain`; an empty chain printing nothing at all; a node
resolving to its directory with no note; the `ABANDONED` note; `.` printing
the grove root; `rate-limit-k003` resolving to key `3`; `[3]-anything`
resolving by key; `frobnicate` accepted as a kind; a bare
`01-impl--rate-limit-k3.md` accepted by `kind` and `brief-chain`; a
`.grove/…` path typed from a subdirectory refused with the doubled
`.grove/.grove/` path; a malformed name anywhere in the tree refusing every
read verb at the opening; and `resolve .grove/01-impl--rate-limit-k3.md`
answering not-found.

**Proof.** After two findings on the first scoped run — an `F009` for a bare
`—` in the fragment index, which must be backtick-wrapped, and an `M201` for a
concept-index label carrying `#[must_use]`, nested brackets being outside the
link subset — `book-check --through information-not-error --check all` is
valid: 4 files, 440 resolved lines, 577 deferred, `final=false`.
`reference_navigation` (13) passes.

## Decisions (running log)

**1 · The structure brief was read first and is the precondition.**
`docs/specs/grove-llm-book-structure.md`, section *3 · Reading the tree*,
states this chapter's reader outcome, section plan and emphasis. The skill was
read from `plugins/grove/skills/grove-draft/SKILL.md` and the editorial family
file from the working tree, as the two sibling drafts did; the plugin cache
still has no `grove:grove-draft`.

**2 · The page reads its three blocks in argument order, not file order:**
the handlers (514–636) first — `cmd_resolve` and `render_resolution` inside
the worked example, `cmd_brief_chain` and `leaf_in` after it as the
transcript's second verb, `cmd_pick` and `cmd_kind` under the finished-grove
transcript — then the three helpers (904–944), and the four variants (75–124)
last, because their doc comments are a catalogue of four public verbs and the
brief's example-before-catalogue rule binds chapter 3's four contracts by
name. `label` and `no_live_leaves` are read beside `cmd_kind`, ahead of
`normalize_leaf_path`, so the helpers' composite is stated after the last of
the three is defined.

**3 · The fragment partition is fifteen literals under three composites**,
each literal on an item boundary: the four variants (8, 10, 15, 17); the five
handlers (10, 13, 14, 11, 19) and `render_resolution` split at its `match`
arms — head with the unreachable root arm (14), the entry arm (17), the two
absent arms with the closes (25) — because each arm carries one claim the
prose adjudicates; and the three helpers (24, 9, 8). Trailing blank lines
follow the manifest's block boundaries: 636 and 927 and 936 are blank and
belong to the fragment above them; 944 is the file's last line. Bodies were
spliced from the source with `sed`, not retyped.

**4 · The worked example looks forward in the carried session.** The rule is
named for a retired match and a shared slug, and the session's tree has
neither until *Growing the tree*'s `leaf-add` and *Ending work*'s `leaf-retire`
have landed; so the second and third renderings are taken from the tree as
the session leaves it — `k3` retired, `k4` live — with the transcript marking
the step, and the finished-grove transcript from the state after the review
leaf's own session has retired `k4`. Rejected: inventing a second slug and a
second retired leaf for chapter 3 alone, which would break the carry the
brief fixes.

**5 · Six claims are adjudicated beside the fragments that carry them.**
`no_live_leaves`'s doc says *spelled four times*; it has three callers, and
`resolve`'s absent answer is `render_resolution`'s. `kind`'s help says a
*missing or unknown kind* is malformed; since `open-kind-k20` any well-formed
token is a kind, so *unknown* names nothing and a token that is not
lowercase-ASCII is what refuses. `label`'s parenthetical — the basename equals
the grove name — is a convention the binary checks nothing of. `Reference::parse`'s
refusal lists *a path under `.grove/`*, a form the grow verbs' `<parent>` and
`<target>` accept and `resolve` does not, measured; the page states it and
names no leaf, because the wording is `grove-loop`'s corpus and that book's
to own, on the same footing as `orientation-k92`'s decision 4. `.` is absent
from `resolve`'s help and accepted, tested, by the handler. And
`normalize_leaf_path`'s comment qualifies its second case with *typed at the
worktree root*; the page shows what the qualifier excludes — a `.grove/`
path typed from a subdirectory passes through and is joined onto a root that
already ends in `.grove` — rather than restating the comment.

**6 · Every promise no test holds is named as such, not left implied.**
`brief-chain`'s finished-grove diagnostic (the handler alone; `kind.rs`'s
empty-grove test says it mirrors this verb); `pick` skipping an `ABANDONED`
leaf (no test in this crate; measured); the abandoned `resolve` note (unit
test only); `[n]-slug` as decorative (no test; measured); the three branches
of `normalize_leaf_path` (no test; the fixtures pass `.grove/`-relative paths
from the working-tree root); and `render_resolution`'s root arm (no test
constructs it). The through-line and the exit-zero mechanism are held by the
source alone and the page says so.

**7 · Loop-side facts are stated from the loop's source, not from memory**,
and only where the page cannot be followed without them: a node reports
`Outcome::Live` (`task_tree::entry_outcome`); `verbs::kind` picks for itself
on `None` where `verbs::brief_chain` takes a bare `&Path`; the terminal-key
fallback runs only after the bare slug matched nothing (`task_tree::lookup`);
`Sought` is `ordinal-fs-tree`'s, re-exported at `grove-loop`'s root; and a
malformed name refuses the whole read. None of the walk, the ascent or the
search is explained.

**8 · Two findings on the first scoped run, both mechanical, fixed before
anything else** — recorded under *Notes*. The second repeats
`orientation-k92`'s decision 7 in a new spelling: a concept-index label may
not carry nested brackets, so `#[must_use]` is written *must-use* there.

**9 · `bash scripts/check.sh` ran over the page and is red on `book-check`
alone, by design.** Seven of eight checks pass; the three finished books
report `final=true`; the `grove-llm` final run reports the fourteen deferred
blocks chapters 4 to 6 own, the four pages that do not yet exist, and the
seven `pending` ledger rows a final scope rejects. It ran in the background
while the in-session review read the page; any edit the review produces is
Markdown under `docs/walkthroughs/grove-llm/`, which only `book-check` and
`reference_navigation` read, and both are re-run after it.

**10 · The one in-session review was spent, and it paid.** A fresh context
was given the page, the book's other pages, the corpus, the loop's verbs,
tree, reference and name modules, the store's `Sought`, the cited tests, the
contract and the brief with a *disprove it* brief and the built binary,
checked about a hundred and twenty claims and returned twenty findings; every
one was classified. Six were wrong and are fixed: a node directory refuses as
*Grove leaf not found*, not as *not a current-format leaf* (that wording is
the brief's alone); `Kind`'s comment is not the longest of the four
(`Resolve`'s is); key-versus-slug is decided by the text and only
slug-versus-handle against the tree, so the page now says only that the
decision is the call's; the transcript's quoted `rate-limit` comes from the
ambiguity arm's debug form, not the not-found arm's; the first two branches of
`normalize_leaf_path` *are* exercised by fixture tests
(`session_kind_tree.rs` passes an absolute path; every `.grove/`-relative
fixture is the second branch), so only the pass-through branch is untested;
and a malformed name in the tree refuses at line 539's `?`, not line 544's.
Ten imprecisions tightened: a third process-level test
(`terminal_infixes_preserve_filename_kind_and_stable_resolution`) pins the
`DONE` and `ABANDONED` paths; `cmd_pick` is the shortest handler *on this
page* (`cmd_leaf_retire` ties it); the rendering tests assert stdout exactly
and stderr by keyword; `missing_root_brief_yields_empty_chain` requires the
empty stdout and not the empty stderr; the *mirroring* remark is `kind.rs`'s
header, not the test's comment; `errors_when_grove_root_absent` runs `pick`
alone and asserts the first line alone; `resolve_malformed_bracket_ref_errors`
asserts the exit alone; the ambiguity unit tests pin *retired* and
*(abandoned)*; `BriefChain`'s comment makes three promises, not four; and the
diagnostic's last clause is not something the driver acts on. One contract
finding fixed: two sentences explained the loop's lookup order and how the
outcome reaches the `Located`, against the book's own boundary; both now name
the fact and not the mechanism. Noise, fixed anyway: the `unwrap_or_else`
clause now says when a working tree has no final component; two evaluative
phrasings rewritten. Visible trade-off, unchanged: the opening section names
the four verbs and their return types before the worked example as the
statement of the rule and of `Sought`, which the reviewer judged not a
catalogue. After the fixes `book-check --through information-not-error
--check all` is valid (440 resolved, 577 deferred) and
`reference_navigation`'s thirteen tests pass.

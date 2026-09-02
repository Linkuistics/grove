# ending-work-k96

## Goal

Draft chapter 5 of the `grove-llm` book: slice `two-steps-remain`,
`05-ending-work.md`, owning `verbs-ending` (`cli.rs` 217–247), `args-ending`
(400–411) and `handlers-ending` (768–825).

## Context

- Draft stage, child 5 of 7 of `grove-llm-k91`. Responsibilities are the
  structure brief's *5 · Ending work* section. Thesis: retire and prune are the
  terminal-marking pair and the **last grove verbs a session runs** — Retire
  precedes Commit, and the commit is jj's — so their stderr names the two steps
  that remain, at the moment of decision; stdout stays data. Cover
  `eprint_next_steps` and its plural; retire's refusals as the verb's and its
  byte-identical promise; prune's **HITL** rule as help the code cannot enforce;
  prune's node case; and the reminder printed last and only when something was
  marked.
- Link the glossary at `task-commit-boundary` beside the reminder.
- The required example anchor is `worked-retire`: `leaf-retire` of
  `/work/atlas/.grove/01-impl--rate-limit-k3.md` — the renamed path on stdout
  and the two remaining steps on stderr; `leaf-prune` of a node shown once, with
  its untouched `DONE` leaf reported. Measure both against the built binary.
- Evidence: `leaf_ops.rs` and `reviewed_producer_lifecycle.rs`. Mark the
  terminal-mark handlers row `explained`.

## Done when

- The fragments for the three blocks are defined on the page, the defers are
  replaced, the ownership rows read `resolved`, the fragment index has their
  rows, and navigation, contents and the concept index are updated.
- `book-check --through two-steps-remain --check all` is valid: 917 resolved
  lines, 100 deferred. The repository Markdown sweep passes. `scripts/check.sh`
  stays red on `book-check` alone, by design.

## Notes

**Verified rather than reconstructed.** Every transcript line and refusal on
the page was measured against the built binary at `20.1.0` on scratch Jujutsu
workspaces shaped like the carried tree, run **without a driver**: this session
runs under a live `grove` loop, so `GROVE_SIGNAL_FILE` had to be unset for the
scratch runs, or admission refused every verb as *wrong working tree* (the
refusal chapter 2's decision 10 describes). Measured and stated: `leaf-retire`
of the carried leaf landing `01-DONE-impl--rate-limit-k3.md` with the singular
reminder, body byte-identical, one `R` line in `jj diff --summary`; retire
refused for a brief, a `DONE` leaf, an `ABANDONED` leaf, a node directory, the
grove root (two spellings, two messages), a `finish` leaf and a name that is
not there (the doubled `.grove/.grove` path); `leaf-prune` of a node with one
`DONE` and two live leaves marking two, reporting the one left untouched, and
printing the plural reminder last; the same prune again as a no-op with
*nothing live to mark*, the left-untouched line and no reminder, exit `0`; a
node holding only an `ABANDONED` leaf printing *nothing live to mark* alone; a
node holding a `finish` leaf refused with nothing renamed; single-leaf prune by
grove-relative spelling; prune refused for a brief, a `DONE` leaf, an
`ABANDONED` leaf and the grove root; both verbs on a tree with no grove
reaching `absent`. The stopped-partway prune was **not** measured — it needs a
fault between two renames — and the page states what the handler does with an
`Err` as read from the source, and names the wording as the loop's.

**Every "no test in this crate" on the page was checked with controls.** The
grep for each refusal's wording over `crates/grove-llm/tests/` came back empty;
the positive control (*already retired*) hit `leaf_ops.rs`, and the cross-tree
control found the same wordings in `crates/grove-loop/src/tree_lifecycle.rs`
and `docs/USAGE.md`.

## Decisions (running log)

**1 · The structure brief is the precondition and was read first.**
`docs/specs/grove-llm-book-structure.md`, section *5 · Ending work*, states
this chapter's reader outcome, section plan and emphasis. `grove-draft` and the
editorial family file were read from `plugins/grove/skills/` in the working
tree; the plugin cache still has no `grove:grove-draft`.

**2 · The page reads the reminder first, then the two verbs, then the help.**
Opening (thesis and the task-commit-boundary premise), the worked example
(retire, then a node prune), `eprint_next_steps`, `leaf-retire` with its
argument struct, `leaf-prune` in two fragments with its argument struct, and
the two doc comments last as the catalogue, under the example-before-catalogue
rule. Rejected: handlers in file order with the reminder last, which would put
the chapter's thesis at the end of the page.

**3 · The fragment partition is eight literals under three composites.**
`verbs-ending` splits at the variant boundary (217–226, 227–247);
`args-ending` at the struct boundary with the blank line trailing (400–405,
406–411); `handlers-ending` into `eprint-next-steps` (768–785),
`handler-leaf-retire` (786–795), and `cmd_leaf_prune` cut at the *Last, so the
reminder …* comment (796–816, 817–825) so the two conditions the comment
states sit with the code they govern. Bodies were spliced from the source by
line range, not retyped.

**4 · The prune example steps outside the carried session, and says so.** The
carried session never prunes and its grove has no node, so the page adds the
smallest node that shows both halves of the node case and the plural reminder —
`03-cache-k5/` with `01-DONE-impl--warm-k6.md`, `02-impl--evict-k7.md` and
`03-impl--ttl-k8.md` — as the same tree after the retirement, keys continuing
from `k4`. Rejected: inventing a second grove, which would break the carry; and
pruning the review leaf `k4`, which shows neither the `DONE` leaf left alone
nor the plural.

**5 · Two claims in the `eprint_next_steps` comment are adjudicated on the
page, and one is leafed.** *Last grove verbs a session runs* is one verb short
of the session — `complete` follows, and the reminder's own second line says
so — and the page reads it as the last verbs that touch the tree, which the
module bears out and the methodology's order supplies. *The commit itself is
jj/git* names a lane this build cannot reach (`docs/adr/jj-is-the-only-lane.md`;
chapter 2 read `worktree` refusing a non-jj tree); the page states the fact
beside the fragment and reproduces the comment as written. The `/git` residue
is a frozen-corpus comment defect with a one-word fix and a precisely statable
question, so under the root brief's rule and `growing-the-tree-k95`'s
precedent it is cut as its own `impl` leaf in `crate-books-k14`, ahead of
`architecture-residue-k75`, to land with this chapter's `eprint-next-steps`
fragment once the book is green.

**6 · HITL is stated as a fact the code makes checkable, not as a rule the page
enforces.** Neither the handler nor `verbs::leaf_prune` reads a flag, prompts,
or consults the environment; the page says so and names the methodology as the
gate, under constraint 5 as the help states it. The worked example's prune is
framed as run after a human confirmed, and the page says the transcript cannot
show the confirmation because the binary never sees it.

**7 · The stopped-partway prune is stated from the source and labelled so.**
`cmd_leaf_prune`'s `?` on the call returns before the `for` loop, so stdout is
the complete list or empty, never a prefix; the marked leaves are named in the
error's context by `grove-loop`'s `stopped_partway`. The page states the
handler's half and names the wording as the loop's, per the book's boundary.
Not measured, and the *Notes* say so.

**8 · `bash scripts/check.sh` ran over the page and the new leaf, and is red on
`book-check` alone, by design.** Seven of eight checks pass — `cargo fmt`,
`shellcheck`, `cargo clippy`, `plugin install`, `conformance`, the conformance
suite, and `cargo test --locked --workspace`. The three finished books report
`final=true`; the `grove-llm` final run rejects the deferred prefix, reporting
the three blocks chapter 6 owns (`verbs-leaving`, `args-complete`,
`handlers-leaving`) and their `pending` early-use rows, the two pages that do
not yet exist (06, 07), and the README and chapter-5 navigation that want a
`Next` to a page the book has not reached. Every finding is under
`docs/walkthroughs/grove-llm`; nothing else in the repository is touched.
`book-check --through two-steps-remain --check all` is valid — 917 resolved,
100 deferred — and `reference_navigation`'s thirteen tests pass.

**9 · The one in-session review was spent, and it paid.** A fresh context was
given the page, the corpus, the loop's `verbs.rs`, `tree_lifecycle.rs` and
`task_tree.rs`, the cited tests, the ADRs, the contract and the brief with a
*disprove it* prompt and the built binary, run with the signal variable unset.
It reproduced both transcripts byte-for-byte, every refusal wording, and every
*no test in this crate* negative, and returned sixteen findings. Thirteen were
valid and fixed: the working-copy claim (*one rename and nothing else*)
contradicted the carried session, where the review leaf and the work are also
uncommitted — the page now says what the verb *added* and that the scratch tree
was committed first; *true of every verb in this module* was wrong for
`finish-commit`; *a lane this build cannot reach* overclaimed, since a
colocated tree is admitted — the page now says the binary never commits through
git and cites the colocated-index test for what it does assert; four *held by*
claims were softened to what the tests assert (a word of the refusal, not its
wording; the git index unchanged, not the working-copy rename; the review leaf
existing, not *next*); the loop internals under the node case (guards, plan
validation, `stopped_partway`) were trimmed to what comes back and what the
handler does, per the book's boundary; the glossary link now also sits beside
the reminder as the brief places it; `LeafRetireArgs`' introduction answers the
five questions; *one word* was two words twice; the not-found row names its
spelling; one line range was imprecise; and four figurative phrases were
replaced. Two were visible trade-offs left as they are: ADRs cited by
backticked path is the node brief's own instruction, applied book-wide; and the
*catalogue of promises* phrasing is the book's vocabulary from chapter 4, which
the `## Handed forward` entry already routes to `copy-edit`. One (*complete*
being the module's, not the commit) was folded into the lane fix. After the
fixes `book-check --through two-steps-remain --check all` is valid (917
resolved, 100 deferred) and `reference_navigation`'s thirteen tests pass.

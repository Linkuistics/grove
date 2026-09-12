# node-documentation-k22

**Reviews:** node-documentation-k9

## Goal

Challenge whether the release-facing documentation teaches the implemented
node-file grammar coherently and completely before `node-cutover-k10` consumes it.

## Context

Read the producer's committed diff, its decisions and the root brief. Verify
against actual source and isolated fixture reads; source-fragment validation
alone does not establish that surrounding explanations are true.

The producer corrected the user guide, walkthrough examples and lookup tables,
source/test comments, preservation-ledger qualification and release guidance.
The review's particular doubts are:

- Whether summaries, early-use tables, manifests, reference-domain examples and
  books whose source was unchanged still contradict required root/node files,
  title ownership, handle composition or malformed-level refusals.
- Whether the producer's boundary around revision evidence satisfies the root
  brief's no-naming-history instruction. It preserved release-tagged changelog
  entries, measured preservation transcripts, formalism evidence, loop records
  and commit-pinned evaluation paths rather than fabricating new measurements.
  Challenge any historical material that still acts as current-contract prose;
  determine whether process material belongs in this task tree instead.
- Whether guide fixtures establish the actual examples and actionable recovery,
  including missing/competing/misplaced files and interrupted decomposition.
- Whether release guidance preserves the human-approved cutover, stopped other
  drivers and verification of both workspace copies. This leaf authorizes no
  release, plugin install or live-tree conversion.

## Done when

Findings are evidence-backed and bounded to this documentation contract. Real
findings earn an integration directly before cutover. A clean review retires
without creating one. Preserve glossary anchors, malformed-input fixtures and
model witnesses; distinguish those from ordinary prose before proposing edits.

## Findings

Read at `cfc2d89c` (`node-documentation-k9`), against the tree as it then
stood. Every `path:line` below is at that commit. Inspection plus isolated
fixture reads: the guide's grammar examples and the four refusal shapes were
exercised with `target/debug/grove-llm` (built 19:28, before the commit; the
commit's only source change is a doc comment) in a scratch jj repository under
the session scratchpad, with `GROVE_SIGNAL_FILE` unset. No test, build, lint or
format command was run; no in-session reviewer was spent.

### F1 — A book still says a node is "optionally headed by a `_BRIEF.md`" (actionable, medium)

`docs/walkthroughs/grove-llm/03-reading-the-tree.md:41-43`: *"A node is a
directory `NN-k<key>/` of numbered children, optionally headed by a
`_BRIEF.md`"*. Both halves contradict the contract the same sentence links to
(`CONTEXT.md#node-directory`: exactly one node file, `_<slug>.md`; `_BRIEF.md`
only at the root), `docs/adr/task-names-are-canonical.md`, `USAGE.md:185-188`
and the binary: a node without its file is refused (fixture case 3, exit 1),
and a node holding `_BRIEF.md` is refused as *found ["_BRIEF.md"]* (case 5).
The producer edited line 11 of this chapter and left this paragraph. It is prose,
not a fragment, so `book-check` cannot see it; the chapter's own line 11 now
says malformed shapes refuse, so the chapter disagrees with itself.

### F2 — The "brief-only tree looks finished" rationale survives beside the Taskless refusal (actionable, medium)

`docs/ARCHITECTURE.md:1014-1016`: a fresh grove creates a first leaf *"because
`pick` skips briefs: a brief-only tree would report 'no live leaves' and be
indistinguishable from a finished one."* Forty lines earlier,
`docs/ARCHITECTURE.md:972`: *"A present root with only `_BRIEF.md` is taskless
and refused"*, which is what `crates/grove-loop/src/tree_lifecycle.rs:89-97`
does. Measured: `grove-llm pick`, `kind` and `brief-chain` on a root holding
only `_BRIEF.md` print `no live leaves; this grove is done` and exit 0 (fixture
case 1), so the verb half is literally true and the "indistinguishable" half is
false for the driver, which is the thing that finishes. The same rationale is
repeated where it should have been retired with the Taskless refusal:

- `docs/walkthroughs/grove-loop/14-finishing.md:238-242` restates it in prose
  and cites the architecture anchor `fresh-grove-start-contract` for it, while
  `plugins/grove/skills/grove/references/driver.md:65-67` cites the **same
  anchor** for the opposite claim (taskless, refused, not selected).
- `crates/grove-llm/tests/root_init.rs:13-16`, a comment the producer rewrote in
  this commit: *"A root brief with no leaves would look finished."*
- `crates/grove-llm/src/cli.rs:70-71`, `root-init`'s help text, paired with the
  fragment at `docs/walkthroughs/grove-llm/04-growing-the-tree.md:891-892`:
  *"a fresh grove is no longer indistinguishable from a finished one."* Taking
  this one is a source change and carries its fragment, ledger and explanation.

Observation for the integration's judgement rather than a defect in the docs:
the verbs and the driver answer a taskless root differently, and no reader-facing
text says so. A human running `grove-llm pick` by hand is told the grove is done
where bare `grove` would refuse.

### F3 — Prose line citations in the `grove-loop` book point at the wrong lines (actionable, low–medium)

The book is source-exact and its validator checks fragments, not the line
numbers prose quotes. Checked against `crates/grove-loop/src/task_name.rs`,
`tree_lifecycle.rs` and `loop_driver.rs` at this commit; these are wrong:

- `docs/walkthroughs/grove-loop/02-the-tokens.md:245` — `peel_key` "at lines
  949 to 956"; it is `task_name.rs:945-952`. `04-the-name.md:956` cites line 950
  inside it correctly, so the book disagrees with itself.
- `02-the-tokens.md:247` — `split_shape` "at lines 918 to 975"; it is
  `task_name.rs:914-922`. `source-index.md:687` gives a third answer, "967 to
  975".
- `source-index.md:692` — *"the renderer chapter 4 reads at line 727"*;
  `impl Display for TaskNameError` is `task_name.rs:651`, which
  `02-the-tokens.md:659` cites correctly.
- `04-the-name.md:891` — "the blank line at line 995"; 995 is a doc-comment
  line, 994 is blank.
- `03-kind-slug-handle.md:134` and `:178` — `loop_driver.rs` "lines 190 and
  202"; 190 is a doc-comment line and 202 is `fn drive(`.
- `03-kind-slug-handle.md:143` — `"finish"` "at line 102 of
  `tree_lifecycle.rs`"; 102 is `grove_root.display(),`; the literal is at 166.
- `03-kind-slug-handle.md:177` — `tree_lifecycle.rs` "lines 72 and 83"; 72 is
  a doc comment, 83 is `&Kind::requirements(),`.
- `03-kind-slug-handle.md:258` — "line 474's call"; `tree_lifecycle.rs:474` is a
  comment line.

Citations that hold, checked the same way: `02-the-tokens.md:296` (182 and
185), `:659` (651); `04-the-name.md:70` (567), `:433` (733), `:1674` (1237);
`03-kind-slug-handle.md:812` (619). Chapters 5–13 were not audited. The class
predates this leaf in part (the sources moved under `node-files-k7`), but the
leaf's own condition is that every book's prose agrees with the implementation.
Whether to re-point every citation or to drop line numbers from prose in favour
of the symbol names the fragments already anchor is the integration's call.

### F4 — Classifier exclusion fixtures spell a node directory the grammar no longer has (actionable, low)

`crates/grove-llm/tests/session_kind_guidance.rs:585` excludes `` `07-flip-k28/` ``
and `:709` excludes `` `NN-<slug>-k<key>/` `` as "a node directory". The doc
comment above the collector (`:481-483`, edited in this commit) now says
`NN-k<key>/`. Both strings still pass, because neither ends in `.md`, but the
property they were written to witness — a *current* node directory stays out of
the sweep — is now witnessed by nothing. Leave `:697`'s `NN-<slug>-k<key>.md`
alone: that is a deliberate predecessor-shape fixture.

### F5 — The changelog entry omits the one behaviour an upgrader meets (actionable, low)

`CHANGELOG.md:54-61` states the grammar and that binary and plugin install
together, but not that a tree whose names this grammar does not spell is refused
by name and never converted (`docs/USAGE.md:209-212`; glossary *No migration*).
That is current behaviour, not naming history, and it is the fact that makes
"install both together" insufficient on its own. `docs/RELEASING.md:198-224`
tells the releaser; nothing tells the changelog's reader.

### F6 — Three paragraphs say one thing (trivia)

`docs/walkthroughs/grove-loop/08-kind-and-briefs.md:665-677`: the producer's
replacement paragraph sits after two earlier paragraphs that already make the
same point about the two missing-file tests. Copy-edit, not contract.

### What was checked and holds

- **The guide against the binary.** Every grammar-bearing transcript in
  `USAGE.md`'s task-tree and verbs sections reproduced: `resolve` by node-file
  slug, by handle and by key all return the directory; `brief-chain` with an
  absolute and a grove-relative leaf path prints root then node file;
  `leaf-decompose` prints `NN-k<key>/_<slug>.md` then the child; the retired
  handle still resolves to the directory. Missing, competing and misplaced node
  files and a missing root file each refuse with exit 1 and no partial output,
  and the missing-file text carries the interrupted-decompose advice
  (`task_name.rs:655-660`) the guide paraphrases at `USAGE.md:220-223`.
- **Sweep with controls.** `(^|[^_A-Z-])BRIEF\.md` over the tracked tree
  outside `.grove/`, `target/` and `.jj/` finds only tagged changelog entries
  (positive control: 13 hits there), the v19.3.0 transcripts in
  `docs/preservation-baseline.md` (cross-tree control), and two lines inside
  dated entries 023 and 024 of `docs/formalism-findings.md`, whose header
  declares every entry historical. Nothing under `plugins/`, `docs/adr/`,
  `docs/specs/`, either glossary or `README.md`. `docs/preservation-baseline.md:
  26-40` scopes its directory and brief spellings to the named measurement and
  links the guide. The boundary the producer drew around revision evidence
  holds; nothing historical reads as current contract except F2's rationale.
- **Release guidance.** `docs/RELEASING.md:99-106` keeps the human-approved
  cutover, stopped other drivers, no relaunch of an in-memory driver, and
  verification of both the source workspace and the release-created copy.
  Nothing in the commit authorises a release, an install or a live-tree
  conversion.
- **Consistency of the rest.** `CONTEXT.md` node directory / node file /
  taskless root, `CONTEXT-MAP.md`, `docs/specs/module-decomposition.md`
  decisions 3–4, `docs/specs/user-guide-coverage.md` rows L3/L5/L8/J7,
  `plugins/grove/skills/grove/BRIEF-FORMAT.md` and `references/bootstrap.md`
  all state one grammar and the refusal, not a skip. The `grove-loop` manifest
  totals sum to the README's 10,458 lines. Glossary anchors are unchanged.

## Decisions (running log)

**1. Six findings; one integration leaf.** F1–F5 are actionable against the
documentation contract and F6 is trivia. `node-cutover-k10` is the first
sibling after this review whose subtree holds live work, so the integration is
inserted at it, carrying this review's handle and not its findings.

**2. Inspection plus isolated fixture reads; nothing else.** The built binary
was run only over a scratch jj repository in the session scratchpad, as the
task's own context invites, to establish that the guide's transcripts and
refusals are real. No suite, check script, build or formatter was run and no
in-session reviewer was materialised. F2's help-text item and F4 touch source;
both are the integration's to take or leave, with the fragment and ledger
pairing `docs/specs/walkthrough-books.md` requires if taken.

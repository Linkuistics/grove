# overview-book-k30 — brief

## Goal

Write the system overview under `docs/walkthroughs/`: a source-exact account of
`crates/grove` — 3 roots, 204 lines — in the shape the human's structure brief
settled, passing final validation.

## Context

- Inputs: the structure brief from `overview-structure-k29`, the shared
  specification from `walkthrough-books-spec-k20`, and the pipeline
  `publishing-pipeline-k13` extracted.
- The corpus, exactly: `crates/grove/src/main.rs` (13), `crates/grove/src/cli.rs`
  (137) and `crates/grove/Cargo.toml` (54). Every byte belongs to a fragment
  graph. `crates/grove/tests/` is evidence, not a root — which matters here more
  than anywhere, because that directory holds the link-integrity suite this whole
  arm depends on.
- `crates/grove` is the human's entry point onto the loop and adds no vocabulary
  of its own (`CONTEXT-MAP.md`): it *is* the grove context along with `grove-llm`
  and `grove-loop`. The overview should read as the system's account, not as a
  fourth crate's.
- Decision 6 of `plan-k1`: `crates/grove` is covered inside the overview rather
  than getting a book of its own, because the overview absorbs command surfaces
  anyway.

## Done when

- The overview exists under `docs/walkthroughs/` and final validation over it
  passes with no deferred holes.
- It is uniform with the other books' page conventions, navigation and prose
  contract, and it is gated by `scripts/check.sh` through the book discovery
  rather than a hand-added line.
- Its links into `docs/USAGE.md` resolve, per the shared specification's
  guide-link contract, and `every_repository_markdown_reference_resolves` is what
  proves it.
- `bash scripts/check.sh` passes.

## Notes

**This leaf writes the overview; it does not perform the move.**
`architecture-move-k31` relocates `docs/ARCHITECTURE.md`'s descriptive account
and re-points its citations. Leave the destination shaped for it — the structure
brief says what shape — but do not start moving prose, or the move and the guard
that proves it land in one unreviewable commit.

**Author it through the pipeline.** If the extracted kinds are installed,
`leaf-decompose` this leaf into one leaf per stage and do only the first. If they
are not, stop and say so rather than authoring by hand.

## Decomposition

The editorial pipeline's chain, under this node, in the shape
`docs/adr/the-editorial-pipeline-is-four-kinds.md` and the stage skills fix.

- `overview-k76` — the **draft** stage. Decomposed into one child per slice of
  the book, in canonical page order, because the whole book is more than one
  session's work (its brief, decision 1); its fifth child is the one that takes
  the book to green final validation.
- `copy-edit`, `art` and `proof` are **cut lazily**, each as the last act of the
  stage before it, every one slugged `overview` and cut with
  `grove-llm leaf-add overview-book-k30 overview --kind <next>` — unless a live
  later sibling under this node already holds that stage. The draft's last child
  cuts `copy-edit`.

## Pointers

- **The structure brief** is `docs/specs/overview-book-structure.md`. It states
  the reader and the outcome, the ordered chapter plan with each chapter's
  responsibilities, and what deserves emphasis and what the book does not cover
  — the three things `grove-draft` requires of a named artifact. Its chapter
  sequence and ownership blocks are what `docs/walkthroughs/overview/walkthrough.toml`
  records.
- **The carried invocation's values are fixed at `orientation-k77`** and every
  later chapter reuses them unchanged: the tree `/work/atlas/` with `.jj/` and a
  grove holding one live leaf `01-impl--rate-limit-k3.md` (handle
  `rate-limit-k3`), `grove` typed in `crates/gateway/src/`, the template
  `impl "claude --add-dir ${repo} ${prompt}"` in `~/.config/grove/config.kdl`,
  the lease at `/work/atlas/.jj/grove/driver.lease`, and the loop's printed lines
  `grove: launching impl with configured "claude" — rate-limit-k3` and
  `grove: grove finished — loop complete.` (formats read from
  `crates/grove-loop/src/loop_driver.rs`). The second ending, `SIGTERM` reraised
  to a wait status of `143`, is chapter 3's.
- **The two obligations outside the book landed with chapter 1**: `CONTEXT.md`
  now carries explicit anchors `guaranteed-core` (the bold term promoted to a
  `###` heading, as `jj-workspace-book-k25` did for its four) and
  `task-tree-scheme` (an anchor line before the existing heading); and
  `docs/ARCHITECTURE.md`'s *Documentation ownership* table has the overview's row.
- **The destination for `architecture-move-k31`** is shaped, not filled. Chapter
  1's `two-products` and `compiler-held` sections are where the six-packages
  opening of *Main module seams* lands; chapter 2 receives *Command surfaces*;
  chapter 3 receives *Runtime flow*; chapter 5 receives the module table. No
  prose has been moved. **Stated limit:** k31 runs after this node's `proof`
  stage, so the moved prose lands in a book the four stages have already read.
  If that matters, the mechanism is a forward correction run cut at k31
  (`docs/adr/a-feedback-edge-is-forward-tree-growth.md`), not an edit to this
  node.

- **Chapter 5 names `docs/ARCHITECTURE.md` by path in two sentences k31 must
  revisit**: under *The modules behind the call*, that the module-by-module
  account is the one that document *carries today* under *Main module seams*;
  and under *Where this book stops*, that *the description a reader can open
  today* is that document's. Both stop being true when the table lands under
  the first of those headings, and k31 owns rewording them in the same commit.
  The heading and its anchor, `the-modules-behind-the-call`, are the
  destination the node brief promised.
- **What the draft adjudicated, and no later stage may undo.** Six claims in
  the frozen corpus or the brief do not survive a check, and each page states
  the checkable fact beside the fragment that carries the stale one: the
  manifest's *three functions long* (ch. 1; `manifest-function-count-k82`);
  `grove-llm`'s *own `0.1.0`* (ch. 2; `grove-llm-version-comment-k83`); *every
  member takes `version.workspace = true`* (ch. 1 and 2, narrowed to the path
  from `grove` to `grove-loop`; `every-member-version-comment-k84`);
  `TemplateSource` read *once* (ch. 3, now *twice*; manifest, ledger and brief
  changed together; `template-source-read-count-k86`); a `[[bin]]` reaching
  private items, which holds for one of two shapes (ch. 1 and 4;
  `bin-target-privacy-claim-k87`); and *97 comment lines*, which is 92 (ch. 1,
  no leaf: the number is the book's own). A copy edit that makes a page agree
  with its comment again reintroduces a defect the draft removed.
- **Names the manifest depends on.** `02-the-surface.md#the-imports` is the
  first-use anchor of four early-use rows and `01-orientation.md#the-binary` of
  the fifth; every chapter's H1 equals its manifest `title` and the navigation
  labels reuse it (draft decision 2). A heading or anchor change on those is a
  manifest change, not a copy edit.

## Decisions (running log)

**3 · The draft is complete and `overview-k76` is closed.** Its fifth child
took the book to `final=true` (3 files, 204 lines, 0 deferred) and a green
`bash scripts/check.sh`, and cut `copy-edit`. Everything the draft node's brief
settled that a later stage needs is under *Pointers* above; the rest is that
node's running log, which stays where it is.

**1 · Decomposed with `--kind draft`; the extracted kinds are installed.**
Verified rather than assumed: `plugins/grove/skills/` holds `grove-draft`,
`grove-copy-edit`, `grove-art` and `grove-proof`; `~/.config/grove/config.kdl`
declares all four; `leaf-decompose … --kind draft` and `leaf-add … --kind draft`
were accepted. One thing outside the tree to flag to the human: the Claude Code
plugin cache this session loaded (`~/.claude/plugins/cache/linkuistics/grove/`,
dated 31 August) predates the four stage skills, so a `draft` session launched
under that harness may not find `grove:grove-draft` by name until the plugin
cache refreshes; the skill is in the repository either way.

**2 · This `impl` session did the draft's first slice rather than the whole
draft**, on the task file's own instruction (*do only the first*) and the
precedent of `jj-workspace-book-k25`, decision 1. The draft node's brief carries
the rest.

## Handed forward

- **copy-edit**, `03-three-steps.md`: figurative or emotive wording the prose
  contract forbids and the draft left in place because style is not its
  charter — *centre of gravity* (opening section; the structure brief's own
  phrase), *is tempted by* (the signal-path section), *That clause is honest*
  (the `# Errors` section), *through-line* (the seam section; chapter 1 uses
  it too, from the brief's *Carry the through-line*), and *the argument in
  miniature* (the entry-point section; also the brief's phrase). Decide once
  whether the brief's own phrases stand as the book's vocabulary or go.
- **copy-edit**, `04-proving-a-negative.md`: figurative, emotive or persuasive
  wording the prose contract forbids, left in place because style is not the
  draft's charter — *earns its place* and *will skip* (opening section; the
  second is the structure brief's own *Stated limits* phrasing), *worth holding
  exactly, because … a reader would assume the opposite* and *the thesis of the
  whole book stated as a panic* (the closure-property section), *manufactures a
  false clean* (the model-not-text section; the source's own wording in
  `help_surfaces.rs`, unmarked as a quotation), *its work begins* (the
  convention section), *a row a reader could go and look at* (the same),
  *paid for a second time*, *Two packages, two models, two walks* and *a
  boundary with nothing behind it* (the twice section), *the very name*
  (the worked example), *exactly as much a second home* and *the day … is the
  day* (the worked example's closing paragraph), and *The obvious way* (the
  model-not-text section).
- **copy-edit**, `05-what-the-call-reaches.md`: two questions in prose — *what
  in these 204 lines goes red when the entry point stops being thin?* (the
  test section) and *what in it would go red if the binary's behaviour moved?*
  (the ledgers section); the precedent `07-what-jj-owns.md` uses the same
  device, so decide once whether the *what goes red* question stands as the
  book's. Figurative wording: *goes red* (throughout, the precedent's
  vocabulary), *sit on the scale*, *blind spot*, *load-bearing* (the test
  section), *the twin of this crate's walk* (the same), *the ceiling* (the
  ledgers section), and two fragments — *The umbrella.* and *The crate's own
  suite, which is the evidence this book cites and does not reproduce.* (the
  final-verification section). Emotive or persuasive: *the honest total* (the
  ledgers section) and *a test that returned the same answer for every binary
  would be a description of this one* (the test section).
- **copy-edit**, `README.md`: the draft aligned the third mechanism's wording
  with chapters 1, 4 and 5 — *a convention a test checks, which nobody
  otherwise would* — and the sentence may read better restated; the claim must
  stay.

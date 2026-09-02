# overview-k89

## Goal

Run the art stage over the system overview at `docs/walkthroughs/overview/` as
the copy edit left it: read every figure against the figure rules in
`docs/specs/walkthrough-books.md` and make each relation and each figure's role
explicit in the document's own Markdown medium.

## Context

- The document is `README.md`, five chapters, two lookup indexes and
  `walkthrough.toml`, at `final=true` over 3 files and 204 lines. Its structure
  brief is `docs/specs/overview-book-structure.md`; its prose precedent is
  `docs/walkthroughs/jj-workspace/`.
- The copy edit changed sentence-level prose only. It removed the draft's
  figurative, emotive and persuasive wording, restated the repeated *goes red*
  idiom as the concrete failing check, and cleared `## Handed forward` from this
  node's `BRIEF.md`. It changed no fragment, figure, page order or manifest
  field.
- The draft's six source adjudications, manifest-dependent names and chapter 5
  sentences reserved for `architecture-move-k31` remain fixed under *Pointers*
  in this node's `BRIEF.md`; this stage must not undo them.

## Done when

- The whole document has been read against the prose contract's *Figures*
  section, every change is within the art charter, and later-stage work is
  handed forward through this node's brief.
- `cargo run --quiet -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/overview --final --check all` reports 3 files, 204 resolved
  lines, 0 deferred lines and `final=true`; `bash scripts/check.sh` passes.
- The required `proof` successor is cut under the editorial family rule unless
  a live later sibling already holds it.

## Notes

The book contract permits only tables, diagrams and other fenced Markdown
figures inside the book directory. It permits no image or diagram asset beside
the pages.

## Decisions (running log)

**1 · Skill loaded from the repository, not the plugin cache.** The mandate's
`grove:grove-art` is not in the cached plugin (the cache predates the four
stage skills, as this node's brief recorded at decision 1); the skill was read
from `plugins/grove/skills/grove-art/SKILL.md`, and the spine and its
`references/editorial.md` from the cache, which does carry them.

**2 · The role rule found nothing to fix.** Every table, fenced `text` and
`console` block, and list figure in the eight pages has an adjacent sentence
stating what the reader is to take from it, and the four `source-index.md`
ledger tables are the contract's exemption. The owned-source totals table on
that page states its role as the contract requires.

**3 · Six relations were carried by prose alone and are now figures**, each
with a role sentence beside it and the carrying prose trimmed to what the
figure does not say. In `03-three-steps.md`: the three disagreeing step counts
(*The entry point*) as a table; the four exit statuses mapped to the actor that
decides each (after the `match` fragment) as a table; every `Err` the page
meets, partitioned into refusal and environmental failure by the statement that
returns it (*What `run` refuses*) as a table; and one loop iteration (*One
foreground iteration*) as a six-step numbered list. In
`05-what-the-call-reaches.md`: `grove` against `grove-llm` on the three
mechanisms (*The test, applied back*) as a table; and the ten evidence rows
partitioned into three classes (*The closed ledgers*) as a table. Every value
in the new figures is one the page already stated; the tables cite `cli.rs`
lines 21 and 43–48 and `main.rs` lines 3–5, checked against the frozen source.
Each table that precedes a fragment directive keeps a prose paragraph between
itself and the directive, so `M105` is unaffected; `book-check --final`
reports 3 files, 204 resolved, 0 deferred, `final=true`, and `scripts/check.sh`
passes all 8 principal checks.

**4 · Figures declined, and which of the three reasons each was.** All are
editorial judgement; none was a contract rule forbidding the fix, and none was
the medium failing to carry it — every relation below is one a Markdown table
would carry, and the medium is not reopened by this record.

- `02-the-surface.md`, *Three exit statuses meet at this boundary*: the same
  status-to-actor mapping now drawn in chapter 3, where the `match` is and
  where the paragraph itself points. Chapter 2's argument-vector table already
  carries each vector's status; a second copy of the actor table one page
  earlier would repeat rather than carry.
- `01-orientation.md`, *One grove dependency* and the dev-dependency
  paragraph: three dependencies and three dev-dependencies, each explained in
  one self-contained sentence beside the fragment that declares it. A reader
  uses each sentence where it stands and reassembles nothing.
- `01-orientation.md`, *Where the repository-surface tests live*: four named
  tests and the claims they make. The page deliberately does not catalogue a
  directory it treats as evidence rather than corpus; a table would invite the
  catalogue the paragraph refuses.
- `02-the-surface.md`, *The other binary*: three checked facts about the verb
  list (flat, twelve, ten), each one sentence with its test named. Three
  sentences, no reassembly.
- `04-proving-a-negative.md`, *A property, not a list*: the five removed names
  and which were subcommands and which flags. The paragraph's point is that the
  closure property names none of them; drawing the list would foreground what
  the test was written to avoid enumerating.
- `05-what-the-call-reaches.md`, *The modules behind the call*: the five
  loop-owned names to four modules. The *Seven names, one check* table above
  it already carries the same mapping in its *Declared in* column.

**5 · One item handed forward to `proof`**: prose lines left far past the
80-column wrap, mostly where the copy edit rewrote sentences in place. Listed
under `## Handed forward` in this node's `BRIEF.md` by page and section, not by
line number, since the art edits moved chapter 3's and chapter 5's lines.

**6 · `proof` cut as `overview-k90`.** No live later sibling under
`overview-book-k30` held that stage — the node's live entries were
`01-overview-k76`, this leaf, and nothing after — so the ordinary last act
applied. Its body names the coherence surfaces the six new figures touch.

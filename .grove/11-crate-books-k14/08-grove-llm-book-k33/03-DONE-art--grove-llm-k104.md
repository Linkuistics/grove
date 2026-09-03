# grove-llm-k104

## Goal

Run the art stage over the `grove-llm` walkthrough book at
`docs/walkthroughs/grove-llm/` as the copy edit left it: assess every existing
figure and every relation carried only by prose against the figure rules in
`docs/specs/walkthrough-books.md`, then make any relation that warrants a figure
explicit in the book's Markdown medium.

## Context

- The document is a contents page, seven chapters and two lookup indexes over
  four frozen source roots and 1,017 lines. Its structure brief is
  `docs/specs/grove-llm-book-structure.md`; the node brief carries the draft's
  visual handoffs and the fixed editorial-pipeline context.
- The draft drew no figures beyond tables and console/text blocks. Chapter 7
  contains the widest candidates: a seven-column twelve-verb table, a
  five-column three-orders table and a three-column boundary table. Decide
  whether any should become a diagram or be split; do not assume width alone
  earns a change.
- Chapters 2 through 6 each carry a console transcript and prose trace over the
  same `/work/atlas/.grove/` session. Assess whether one before/after tree figure
  would serve those five chapters without duplicating their worked examples.
- The copy edit changed prose only, retained the named what-is-left test and the
  intentional *what order holds* title/heading, standardized first-use test-file
  references in running prose, and changed chapter 7's mixed `Held by` column to
  a grammatically consistent `Evidence` column. Do not reopen those decisions.

## Done when

- The whole book has been read against the prose contract's *Figures* section,
  every change stays within the art charter, and later-stage work is handed
  forward through the node brief.
- `cargo run -p book-validation --bin book-check -- --repo . --book
  docs/walkthroughs/grove-llm --final --check all` reports 4 files, 1,017
  resolved lines, 0 deferred lines and `final=true`; `bash scripts/check.sh`
  passes.
- The required `proof` successor is cut under the editorial family rule unless
  a live later sibling already holds it.

## Notes

The book contract permits tables, diagrams and other fenced Markdown figures
inside the book directory. It permits no image or diagram asset beside the
pages.

## Decisions (running log)

**1 · The role rule needed no change anywhere: all 41 non-exempt figures
already state their role.** Enumerated rather than sampled — every table and
every non-fragment fenced block in all ten pages was extracted mechanically
(45 figures; the four `source-index.md` tables under `Source roots`,
`Ownership blocks`, `Fragment index` and `Early uses` are the contract's only
exemption) and each was read against its adjacent sentence. Every one has a
statement of what the reader is to take from it, most as a lead-in, several as
both a lead-in and a follow. `source-index.md`'s fifth table, the owned-source
totals, states its role like any other figure, as the specification requires of
it by name. Nothing to fix under the second rule.

**2 · Chapter 7's three wide tables stay as they are — editorial judgement, not
a medium limit.** The node brief handed this stage the question and said width
alone does not earn a change. The twelve-verb table is seven columns because
its six question columns are the six things the book asked one family at a
time, and the five paragraphs beneath it are read *down the columns*; splitting
it by family would destroy the one affordance the page's argument uses, and
recreate the six per-chapter views it exists to replace. The three-orders table
is explicitly read across, and the boundary table is three columns. A diagram
cannot carry twelve rows of sentence-length answers, but that is not a finding
about the medium: the tables carry their relations, and Markdown carries the
tables.

**3 · The carried tree is now drawn once, in *Orientation*, not five times.**
The node brief asked whether a before/after tree figure would serve chapters 2
to 6. It does, and the reason is sharper than the brief's: the session mutates
`.grove/` twice, so the tree has three states, and chapters 2, 3 and 4 each
read a state no chapter draws. *Reading the tree*'s transcript jumps between
the first state and the third with only a comment line to mark it. The figure
is placed in `01-orientation.md#one-session`, where the structure brief fixed
the carried session's values for every later page, and it shows the three
states with the verb between them and the chapters standing at each. It stops
at the session's own tree: *Ending work* and *Leaving the loop* draw trees that
go past it — a node the session never made and a driver-materialised `finish`
leaf — and a closing sentence says so rather than leaving three tree figures
unrelated.

**4 · `cmd_root_init`'s drop-order argument is now a figure, and the claim was
re-measured rather than inherited.** The paragraph carries a comparison over
two forms × two sources × a consequence in eight lines of prose, which is rule
one's own case. Compiled and run under this workspace's toolchain at edition
2024: with `match` the failure arm runs and the scrutinee's guard drops *after*
it; with `let … else` the initializer's value drops *before* the else block. So
the table's last column — a tree read added to the failure path would deadlock
in the form the code chose and not in the form the comment rejects — is the
measured result, and the draft's finding stands.

**5 · Five relations were left in prose, all on editorial judgement, and none
because the medium could not carry them.** `admit_ambient_session`'s five
ordered checks (*The grammar and the openings*) are the loop's internals: the
book states its boundary as the call in the reader contract, in every chapter
opening and in chapter 7's boundary table, and the reader-facing half is already
tabulated twice — chapter 2's argument-vector table and chapter 6's outcome
table. The twelve-verb partition over the two openings, stated in prose at the
end of chapter 2, is drawn in chapter 7's `Opening` column; the prose there is
the forward-looking minimum the repetition rule allows. The five `grove_loop`
items reached by path and never imported are five members beside a table of
fourteen, and a second table would be lopsided. The three shapes at the
`finish-commit` seam are three members in one sentence. Which refusal an
operator sees when slug and kind are both wrong is two cases, and the step
table above it already carries the order. **No finding is a medium finding**:
nothing this stage wanted to draw was beyond a table, a list or a box-drawing
inside a `text` fence, so the book contract's no-asset rule is not reopened by
this stage.

**6 · One finding handed forward rather than fixed: the concept index has no
chapter 7 entries.** Chapters 1 to 6 carry roughly twenty entries each and
chapter 7 carries none, though it holds five anchored sections and the book's
own takeaway test. It is not this stage's: a curated navigation list is not a
figure under the contract's definition, and the specification permits
non-exhaustive curation in terms. Recorded in the node brief for `proof`.

**7 · Left green and cut the `proof` successor.** `book-check --final --check
all` reports 4 files, 1,017 resolved lines, 0 deferred lines, `final=true`;
`bash scripts/check.sh` passes all eight principal checks. No live later
sibling held `proof`, so the family file's conditional last act was not already
discharged and this stage cuts it.

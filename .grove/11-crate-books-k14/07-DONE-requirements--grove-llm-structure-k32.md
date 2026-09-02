# grove-llm-structure-k32

## Goal

Elicit, from the human, the structure brief for the `grove-llm` book: audience
refinements, conceptual order, what deserves emphasis, and what the reader must
be able to do at the end. Commit it as an input artifact.

## Context

- Decision 15 of `plan-k1`: the source does not carry audience, conceptual order
  or emphasis, so every book takes a human-authored structure brief. **Do not
  invent one.** This leaf exists so that no session has to.
- Already settled, and not to be re-elicited: the audience (decision 7 — knows
  Rust and jj, has driven a grove, grove vocabulary linked to `CONTEXT.md` and
  never re-taught), the depth (decision 1 — complete source-exact, every byte in
  a fragment graph), the method (`linkuistics:writing-code-walkthroughs`, whose
  eight-field intake `plan-k1` already answered), and the corpus.
- The corpus, exactly, from the root brief: 4 roots, 1,017 lines — every
  `crates/grove-llm/src/**/*.rs` plus `crates/grove-llm/Cargo.toml`. `tests/` is
  evidence, not a root.
- `CONTEXT-MAP.md` records that `grove-llm` **is** the grove context — it is the
  session's verb surface over the loop and adds no vocabulary of its own. The book
  therefore links `CONTEXT.md` rather than defining anything.
- Almost the whole crate is one file: `src/cli.rs` is 944 of its 1,017 lines. How
  a book divides a single large module is the structural question here.
- The books already written are the shape to react against, and the pilot's
  `jj-workspace` book is the closest precedent for a new one.

## Done when

- A committed structure brief states, in the human's own words: the chapter
  sequence and what each chapter is for, the concepts that carry the book, what
  to emphasise and what to pass over, and the reader-facing outcome.
- It is specific enough that the authoring leaf needs no second interview.
- `bash scripts/check.sh` passes.

## Notes

**HITL: the loop stalls here by design.** An agent answering its own structure
questions has broken the distinction this leaf exists for. If the human is not
available, stop and say so.

**Structure, not prose.** The deliverable is the book's shape and emphasis, not
draft text; drafting is the authoring leaf's, through the pipeline.

## Decisions (running log)

**1 · The spine is the crate's own header: order is what is left.** Every verb
is one `grove_loop::verbs::` call plus rendering, and what is left that is not
rendering is *order* — the operator's text read by the type that owns it before
a lock is taken, the just-in-time presence rule asked before the mutation, and
the session admitted against the completion channel before it is written to
(`src/cli.rs` lines 10–23). The chapters follow the order a session meets the
verbs, and each opens on the one thing a thin binary still has to get right
there: an absent answer is information rather than an error for the reading
verbs; text before lock and presence before mutation for the growing verbs; the
two remaining steps named on stderr for the terminal marks; admission before the
signal for `complete`. Recovered from the source rather than imposed, which is
the standard both elicited precedents held to. Rejected: the session's verb order
with no per-chapter thesis (the guide's four groups with fragments attached —
`docs/USAGE.md` already owns *what each verb does to the tree*); *thin, held by
the compiler* as the whole spine (the overview's chapter-1 mechanism and stated
outcome, which this book would re-prove over twelve verbs); and the seam —
`readable`, `writable`, the vacancy, no tree at all — which sorts twelve verbs by
an implementation detail. Each of the three orders is pinned by a test:
`session_kind_presence.rs` and `tree_lock.rs` in this crate's own suite, and
`grove_llm_admits_only_the_live_epoch_while_version_remains_exempt` in
`crates/grove-loop/tests/driver_lease.rs`, which drives the real binary.

**2 · Seven pages, six source-owning, cut by verb family.** Orientation (the
three small roots and `cli.rs`'s header and imports), the grammar and the
openings (`Cli`, the enum head, `operation_label`, `run`, and the four shared
openings `worktree`, `readable`, `writable`, `absent`), reading the tree (`pick`,
`brief-chain`, `kind`, `resolve`), growing the tree (`root-init`, `leaf-add`,
`leaf-insert`, `leaf-decompose`), ending work (`leaf-retire`, `leaf-prune`),
leaving the loop (`finish-commit`, `complete`), and a final-only close. Each
family's help text, argument structs and handlers sit on one page beside the rule
that family enforces, which is what the spine asks for and what the file's own
order — types, then argument structs, then handlers, then helpers — cannot give.
The price is twenty-two ownership blocks in `cli.rs` against eleven in
`jj-workspace`'s `lib.rs`, for the same reason: the file is ordered by Rust
convention and the book by concept. The grow chapter is 376 lines, 37% of the
corpus, and stays one chapter because it is one rule with four instances that
each add a lock-scope argument in order — `root-init`'s drop order, `leaf-add`
printing only after the run landed, `leaf-insert`'s lint outside the lock,
`leaf-decompose`'s read before the write opening — the same ground on which the
precedent gave `refusal.rs` a 230-line chapter. Rejected: eight chapters with
`leaf-decompose` split out (a 78-line chapter that would carry the last step of
an argument the other three build); keeping the whole `Command` enum in chapter
2 as *the contract as `--help` renders it* (fifteen blocks, but 194 lines of help
text before any handler, and the catalogue rule then binds a whole chapter);
and file order (fewest blocks, but no chapter can open on a rule and the enum
chapter is a catalogue by construction).

**3 · One session's verbs, carried in chapter order, with the presence refusal
as the second ending.** The carried example is grove's real usage of this
binary: one session over a fixed tree with one live leaf runs `resolve` of its
mandated handle, `brief-chain`, does its work, `leaf-add`s a review leaf,
`leaf-retire`s its own leaf, commits, and runs `complete`. Orientation traces the
whole session at low resolution; chapter 2 takes its first verb as argv through
admission, `operation_label` and dispatch, and shows the refusal one directory
off; chapter 3 takes `resolve` at full resolution with its three renderings —
live, retired with the stderr note, ambiguous; chapter 4 takes the `leaf-add`
with two endings — landed, and refused before the lock because no template
declares the kind, the tree byte-identical; chapter 5 the `leaf-retire` and its
two-step reminder; chapter 6 `complete` under the loop and outside it. The two
verbs a session never runs, `root-init` and `finish-commit`, get one short trace
each as the driver's, in chapters 4 and 6, and `finish-commit`'s shows the
lenient handle `finish-k0001` accepted and the operator's spelling quoted. The
second ending is the presence refusal because it is the spine's second order
made visible and `session_kind_presence.rs` pins it — where the self-deadlock,
the file's most argued claim, is not an ending at all: `tree_lock.rs` proves it
cannot happen. The starting tree's concrete values are the orientation slice's
to fix, as both precedents left them, and it may reuse the overview's tree.
Rejected: `leaf-decompose` alone at full resolution (the carry breaks after
chapter 4); two refusals as the carry (the happy path becomes the aside, as both
precedents found); and this grove's own tree (values that go stale by design).

**4 · The stated outcome is the what-is-left test.** At the end the reader can
take a thin command surface over a library in their own code, ask what is left
there that is not rendering, and answer in three parts — which text is parsed
before which lock, which check runs before which mutation, which admission
precedes which signal — saying for each what reversing the order costs (a process
blocking against itself, a tree mutated by a command that was refused, a signal
sent to a loop that did not launch this session) and naming the test that would
catch it. All three are provable inside the corpus and each has a named test;
chapter 7 states the test and applies it to all twelve verbs. Uniform with the
precedents' transferable tests. The stream contract — data on stdout, advice on
stderr, exit zero for information, every refusal carrying a remedy — is the
rendering half of the same header and is chapter 7's assembly table rather than
a second stated outcome. Rejected: the maintainer outcome (add a thirteenth verb
without breaking the four invariants), which follows for free from
source-exactness plus chapter 7's table; the reference card, already the guide's
row for the human and `--help`'s for the LLM; and *take it as a dependency*,
ruled out by fact — it is a binary that ships inside grove's cut.

**5 · The third thing each chapter's prose owes is to hold the help to the
handler.** Measured before deciding: 45% of the corpus is comment prose —
`cli.rs` 426 of 944 lines, `Cargo.toml` 23 of 54, `lib.rs` 14 of 16 — and 204
of those lines are the doc comments on the twelve `Command` variants, which are
the `--help` text the LLM reads and the guide paraphrases for the human. The
fragment graph quotes them verbatim on the page anyway, so the standing risk is
paraphrase: three documents about the same twelve verbs. Alongside the two
obligations both precedents carry — adjudicate every argued claim by naming the
behaviour it rests on, the test that proves it and the alternative rejected; and
carry the through-line across roots — each page therefore owes, for every
promise a verb's doc comment makes, the line in the handler that keeps it and the
test that would catch its breach: `leaf-add` promises to print nothing at all if
the run could not be created and `print_paths` after the call is what keeps it;
`leaf-prune` promises to write no producer receipt and
`reviewed_producer_lifecycle.rs` holds it. Where a doc comment carries
methodology rather than mechanism — the fifty lines on review chains, vendor
pairs and integrate placement under `leaf-add` and `leaf-insert` — the page
links the guide and explains only what the code keeps of it: the atomic list and
the silence on failure. Rejected: supplying the loop's half at the seam (what
each `grove_loop::verbs` call takes, returns and does not promise), which is the
overview's third obligation restated and belongs under what the book does not
cover; naming the session's half, which the doc comments already do for the LLM
and would restate the guide; and no third obligation, which leaves the
paraphrase risk unaddressed for a corpus where the help text is a fifth of every
page.

**6 · Four guide anchors, seven glossary anchors, two of them new.** Guide:
`usage-tree-verbs` from the `README.md` reader contract — the required citation,
and the section whose subject is exactly this crate's surface;
`usage-driver-lease` from chapter 2 beside admission, the lease being what
installs the epoch a verb is admitted against; `usage-review-composition` from
chapter 4 beside `leaf-add`'s chain and pair help, so the page links the
methodology rather than explaining it; `usage-finish` from chapter 6 beside
`finish-commit`. All four exist in `docs/USAGE.md` in the explicit form.
Glossary: `driver-lease` and `stated-vcs` (chapter 2, the latter beside
`Workspace::resolve` in `worktree`), `task-tree-scheme` (chapter 3, beside
`resolve`'s reference grammar), `task-commit-boundary` (chapter 5, beside the
two-step reminder), `loop-control-channel` (chapter 6, beside the signal file) —
all five existing — and `session-epoch` (chapter 2) and `tree-access-lock`
(chapter 4), which are the terms the spine's orders are named in and exist today
as bold paragraphs at `CONTEXT.md` lines 424 and 858. Both must be promoted to
`###` headings with explicit anchor lines before the book validates; that work
is outside the book and is `grove-llm-book-k33`'s, as the overview's two
promotions were its book leaf's. No test pins either bold form in the glossary —
`crates/grove-llm/tests/composition_guidance.rs`'s pin on the phrase *Tree
access lock* is against `docs/ARCHITECTURE.md` — so the promotion is the same
free move `guaranteed-core` was; keeping the phrase as the heading text is
still right, since the glossary's own definition begins with it. Rejected: the minimal set (one
guide anchor and the five existing glossary anchors — the two terms the book
most depends on become bare links into a 1,049-line document) and the wide set
(`usage-session-lifecycle`, `usage-task-tree`, `undoing-a-mistake`,
`work-item-handle`, `session-kind`, `pruning` — three more promotions for
navigation seven chapters do not need). Not listed in `CONTEXT-MAP.md`'s
durable-record roll, following the two existing structure briefs; nothing
enforces that list and the precedent is two for two.

**7 · The test seams, put to the human and confirmed; slice IDs and omissions
applied from precedent and not objected to.** Seams, all existing: `book-check
--final --check all` over `docs/walkthroughs/grove-llm/` by discovery from
`scripts/check.sh`; `every_books_subject_is_exactly_the_specifications_inventory`
and `every_books_corpus_exceptions_are_exactly_the_specifications_inventory`
(`crates/grove/tests/corpus_exception_inventory.rs`) — the subject row exists
and the book declares no exception, so neither needs a spec edit;
`every_book_root_has_a_documentation_ownership_row`,
`user_documentation_references_resolve` and
`every_repository_markdown_reference_resolves`
(`crates/grove/tests/reference_navigation.rs`), the last of which also sweeps
this brief's own links; and `book-check`'s `M201` against the manifest for the
two new glossary anchors, red until both promotions land. Stated limit: nothing
mechanical checks decision 5 or a worked example's start and end. Slice IDs are
named for the rule each chapter carries — `one-call-plus-rendering`,
`admitted-before-dispatch`, `information-not-error`, `before-the-lock`,
`two-steps-remain`, `admit-before-signal`, `assembly` — none equal to a page ID
and none carrying a task key, applying `jj-workspace-structure-k17`'s decision 8
rather than re-eliciting it. **Consequential edit, made this session:**
`grove-llm-book-k33`'s task file names the brief by path — `grove-draft` stops
without a named artifact — and carries the two obligations outside the book
(the glossary promotions and the ownership row) and the two stale claims
chapter 1 adjudicates.

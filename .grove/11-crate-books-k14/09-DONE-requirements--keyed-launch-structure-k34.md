# keyed-launch-structure-k34

## Goal

Elicit, from the human, the structure brief for the `keyed-launch` book: audience
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
- The corpus, exactly, from the root brief: 9 roots, 2,073 lines — every
  `crates/keyed-launch/src/**/*.rs` plus `crates/keyed-launch/Cargo.toml`. `tests/` is
  evidence, not a root.
- `CONTEXT-MAP.md` argues `keyed-launch` is deliberately **not** a bounded
  context — but only because it was named to avoid being one. Its vocabulary is
  key, template, slot, argv, launch, channel, token, escalation, overlay, and the
  word it refused to reach for is *session*. That refusal is the crate's most
  interesting property, and whether it is the book's spine or a footnote is a
  structure question.
- Its decisions live in the grove context: decision 7 of
  `docs/specs/module-decomposition.md`, and the ADRs
  `complete-session-configuration`, `untracked-configuration-delta` and
  `the-launched-child-is-a-job`.
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

The deliverable is `docs/specs/keyed-launch-book-structure.md`, 745 lines, which
carries each decision below in its settled form. This log records what was put to
the human, what was chosen, and what was rejected with why. Seven questions, one
per prompt, in the order the three preceding structure-brief interviews used;
every fact below was measured or read before its question was framed, and none of
them was asked.

**1 · The spine is the crate's own first paragraph: the words are the words the
file holds.** Every chapter opens on the one thing that stage must not add and
must not interpret. Recovered from `src/lib.rs` lines 4–9 — *nothing here
understands either: a key is an opaque string, a slot is a name the consumer
declares, and the words of a template are the words the file holds* — with
`Cargo.toml`'s `description` as the same sentence compressed,
`docs/ARCHITECTURE.md` line 166 stating it from grove's side, and
`CONTEXT-MAP.md` lines 46–53 recording the refusal of the word **session** as the
naming decision behind it. The task file asked whether that refusal is the spine
or a footnote; it is the spine. Nine stage-rules follow, one per source-owning
chapter, and the brief tabulates them. Pinned by
`shell_metacharacters_stay_literal`,
`a_slot_value_is_one_argument_whatever_it_contains` and
`the_kit_reads_only_the_file_it_is_given`. Rejected: **the two halves meet only at
`Argv`** (`lib.rs` 31–34), which is the crate's true structural claim and is
compiler-enforced rather than tested, but is one boundary and so splits the book
in two rather than giving nine chapters nine rules — it is stated in chapter 1 and
proved in chapter 5; **checked whole before anything is spawned**, the
configuration half's thesis, where `run.rs` at 29% of the corpus validates
nothing — chapter 3's; and **a launch ends out of band**, its mirror and chapter
6's, leaving `templates.rs`'s 32% unserved.

**2 · Ten pages, nine owning source, cut by concept with the inline tests
gathered at the close.** Orientation (`Cargo.toml`, `lib.rs`, `error.rs`), the
names (`vocabulary.rs` and `templates.rs`'s seven types), two documents (`load`
and whole-document validation), template law (every per-node and per-template
rule with its diagnostics), to an argv (resolution, expansion, `argv.rs`), the
channel, the job, the escalation, how this is checked (`channel.rs`'s inline
`mod tests` plus `conformance.rs`), and a final-only assembly page. Every block
boundary was verified with `sed -n` on the exact lines before being written down,
and the mapping closes arithmetically: 196 + 135 + 193 + 246 + 188 + 271 + 328 +
279 + 237 + 0 = 2,073. The order is `lib.rs`'s own section order with the
vocabulary moved ahead of the two documents, because the vocabulary is what makes
the documents checkable and `lib.rs` says so. Three roots split across chapters —
`templates.rs` four ways in eight blocks, `run.rs` two ways in four, and
`channel.rs` at line 272 — the first two because the files are ordered by Rust
convention and the book by concept, the third because the module's *subject* is
assurance while its subject matter is chapter 6's. Rejected: **nine pages with
`channel.rs` kept whole**, which keeps a file undivided at the price of a 404-line
chapter and of separating the inline tests from the conformance argument they
belong to; and **eight pages** merging the vocabulary into the loading chapter,
which is closest to every existing book's count (5–8) but makes chapter 2 carry
two rules where the spine asks for one.

**3 · The carried example is grove's own configuration, told strictly from the
crate's side, with the overlay-only refusal as the second ending.** Two real keys
from `docs/CONFIGURATION.md` line 118 — `impl "claude --model opus ${prompt}"` and
`review-impl "codex exec --model gpt-5 ${prompt}"` — an overlay declaring `impl`
alone, grove's real four-slot vocabulary from
`crates/grove-loop/src/session_config.rs` lines 38–55, and `GROVE_SIGNAL_FILE` as
the channel variable. The reader knows exactly what the words mean and watches the
crate not care, which is the spine made visible rather than asserted; chapter 1
states the mapping once — a session kind is a key — and no chapter explains what a
session is. The brief tabulates the anchor, start and observable end for all ten
pages. The second ending is the overlay-only refusal because it is the property
standing between an untracked file a project ships and a program its operator
never chose, and `a_key_only_the_overlay_declares_does_not_resolve` pins it.
Rejected: **the crate's own test fixtures verbatim**, where every step would be a
named passing test but the suite uses three vocabularies — `prompt`/`label`,
`script`, and the kit's — precisely because the halves are separately usable, so
no single thread runs through them; **an invented domain-free consumer**, which
keeps grove's meanings out but leaves the crate nothing to refuse, so the property
is asserted rather than shown, and which nothing in the repository pins; and a
refusal as the *carried* example, which both precedents found makes the happy path
the aside.

**4 · The stated outcome is the pass-through test.** At the end the reader can
take any layer that carries a value from a human's file to an effect in the world
and ask *where does this layer learn what the value means?* — answering in three
parts, each with a cost and a named test: on the way in, assembling one value from
two sources (`a_key_only_the_overlay_declares_does_not_resolve`,
`an_overlay_replaces_a_whole_template_and_reports_its_own_path`); on the way
through, re-reading a value already read
(`a_slot_value_is_one_argument_whatever_it_contains`,
`shell_metacharacters_stay_literal`,
`an_unquoted_hash_is_refused_rather_than_truncating_the_argv`); and on the way
out, inferring what came back or adding what the operator did not write
(`a_child_that_never_signals_ends_with_no_token`,
`a_scrubbed_variable_is_removed_from_an_inherited_environment`). All three are
provable inside the corpus and the closing page applies them to all nine
source-owning chapters. **Surfaced during the interview and settled in the brief:**
`jj-workspace`'s spine is *what the crate refuses to own*, which is adjacent
enough that a reviewer would otherwise read the two books as one book twice — the
difference is that every one of that book's refusals names another owner, and this
crate names none, because the meaning does not exist inside it to be delegated.
Chapter 1 states that in a sentence and no later chapter returns to it. Rejected:
**the out-of-band completion test**, sharper and genuinely transferable but
reaching only `run.rs` and `channel.rs`, leaving chapters 2–5's 762 lines (37%)
unserved — it survives as chapter 8's thesis; **take it as a dependency**, ruled
out by fact (`docs/RELEASING.md`, *One release, six packages, one tag*;
`Cargo.toml` lines 41–45), the identical ground on which `jj-workspace`'s brief
rejected it; and **the maintainer outcome**, which follows free from
source-exactness, as all three precedents found.

**5 · The third prose obligation is to carry the load where the source does not,
and only there.** Measured first: 693 of 2,073 lines are comment prose (33%), but
unevenly — `lib.rs` 76%, `Cargo.toml` 53%, `run.rs` 53% (324 of 607) where the
prose is full argument in situ, and `templates.rs` **13%** (89 of 670), the
biggest root and the place every rule the decision records state actually binds.
So the instruction differs by chapter and a technical review checks the right one:
chapters 3–5 **supply** the argument — for each template rule, the enforcing line,
the diagnostic it produces and the record clause it keeps — while chapters 7–8
**do not restate**, because the comments already argue and the fragment graph
quotes them verbatim on the page, so prose there connects arguments across items
and names the test. The risk closed is the reverse of `grove-llm`'s: not
paraphrase across three documents, but a book that pads where the source is strong
and thins where it is silent. The two shared obligations — adjudicate the claim,
carry the through-line — are carried unchanged. Rejected: **hold the record to the
code**, real but roughly half-discharged already by `run.rs`'s own comments, and
the first two obligations reach the other half; **argue the seam end to end**,
which is one argument made once rather than a per-chapter obligation a reviewer
can check page by page, so it is chapter 1's responsibility and chapter 5's proof
instead; and no third obligation, which leaves the dominant measured fact
unaddressed.

**6 · Three anchors, no promotion.** `[guide]` `docs/USAGE.md` with
`usage-running-grove` and `usage-session-lifecycle`; `[[glossary]]` `CONTEXT.md`
with `loop-control-channel`. All three carry explicit `<a id="…"></a>` lines
today, so `book-check`'s `M201` is green from the first slice and nothing outside
the book is owed on their account — unlike `grove-llm`'s book, which reserved two
anchors that did not exist and whose book leaf carried the promotions. The
`README.md`'s required guide citation is `usage-running-grove`, because
`docs/USAGE.md` line 92 states this crate's eager-validation property from the
reader's side. Rejected: `usage-review-composition`, a false friend whose
*escalation* is the methodology's and not the kill escalation;
`usage-driver-lease`, `session-epoch` and `driver-lease`, which describe what
grove does with a channel path rather than anything this crate knows; a promoted
`session-kind` anchor, which chapter 1's one-sentence mapping does not need badly
enough to buy an edit outside the book; and **`[guide] omitted`**, which was a
genuine contender — `conformance.rs` exists so *reusable outside grove* is
checkable — but which `CONTEXT-MAP.md` lines 46–47 rule out, holding
`keyed-launch` as **not** a context of its own, explicitly unlike
`ordinal-fs-tree`, the only book granted that exemption and granted it on exactly
that ground. **Established while framing this question and stated in the brief:**
the link contract closes a book's local targets to its own pages, its own roots,
the guide and the glossary — so decision 7 and the three ADRs governing this crate
are named at the chapters that keep them and cited nowhere.

**7 · The test seams, put to the human and not objected to; slice IDs and
omissions applied from precedent.** Seams, all existing: `book-check --final
--check all` over `docs/walkthroughs/keyed-launch/` by discovery from
`scripts/check.sh`; `every_books_subject_is_exactly_the_specifications_inventory`
and `every_books_corpus_exceptions_are_exactly_the_specifications_inventory`
(`crates/grove/tests/corpus_exception_inventory.rs`) — both green with **no spec
edit**, since the subject row exists and the book declares no exception against an
inventory that has no `keyed-launch` row, which is exactly what puts
`channel.rs`'s inline `mod tests` inside the corpus;
`every_book_root_has_a_documentation_ownership_row`,
`user_documentation_references_resolve` and
`every_repository_markdown_reference_resolves`
(`crates/grove/tests/reference_navigation.rs`), the last of which also sweeps this
brief's own links; and `book-check`'s `M201` against the manifest, green from the
first slice. **One edit outside the book is owed** — the ownership row, which is
`keyed-launch-book-k35`'s as each precedent row was its book leaf's. Stated limit:
nothing mechanical checks the spine, a worked example's start and end, or the
per-chapter prose obligation, and the crate's 1,319-line suite is the book's
evidence rather than its gate. Slice IDs are named for the rule each chapter
carries — `understands-neither`, `rules-about-names`, `never-assembled`,
`words-not-shell`, `whole-word-or-nothing`, `appearance-is-the-event`,
`nothing-else-added`, `the-launchers-job`, `checked-without-meaning`, `assembly` —
none equal to a page ID and none carrying a task key, applying
`jj-workspace-structure-k17`'s decision 8 rather than re-eliciting it. The one
open point the sequence question had not settled was put separately: the
pass-through test lives on a **tenth final-only page**, uniform with both
precedents, whose closing page carries slice `assembly` and owns no source —
rather than inside chapter 9, which would give the closing page a scoped prefix to
prove as well as a book-wide argument to make.

**Consequential edits, made this session.**
`keyed-launch-book-k35`'s task file now names the brief by path — `grove-draft`
stops without a named artifact — and carries the three things it would otherwise
have to rediscover: that `channel.rs` lines 272–404 are corpus and chapter 9's,
that the ownership row is its one obligation outside the book and no glossary
promotion is owed, and that the `residue(grove-loop, keyed-launch)` deletion is
joint and `architecture-residue-k75`'s.

**Externalised, not absorbed.** `runner-sketch-drift-k106` — `impl`, inserted
ahead of `architecture-residue-k75` so that leaf stays last in the node. Decision
7 of `docs/specs/module-decomposition.md` carries an interface sketch that has
drifted from the crate in six places: `Launch::cwd`, `End::Interrupted`'s payload,
`Templates::require` and `Templates::keys`, `Channel::discard_abandoned`,
`Argv::words`, and the free functions `take_interrupt` and `reraise`. The last is
substantive — those two are the whole of a looping launcher's obligation for a
signal arriving between launches, and a sketch without them describes a crate that
cannot report that ending. It is a document edit, not a corpus one, so the freeze
is not engaged; and the book can neither fix it nor cite it, so it is not a
prerequisite. **No claim inside the corpus was found stale**, so unlike
`grove-llm`'s book no chapter here carries an adjudication obligation.

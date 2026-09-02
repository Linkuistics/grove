# grove.add-user-guide-and-code-walkthoughs-for-all-crates — brief

## Goal

One user guide for the `grove` binary, and complete source-exact code
walkthroughs for every **Grove runtime crate** — `grove`, `grove-llm`,
`grove-loop`, `jj-workspace`, `keyed-launch` and `ordinal-fs-tree` — authored
through a publishing-house pipeline (draft, edit, art, proof) that grove does
not yet have, driven by an explicit loop, and directed by human-authored
structure briefs the source code cannot supply.

Four products live here. Only the first is what the grove was named for, and
the rest exist because it cannot be reached without them.

- **P1 · The documentation** — the user guide, a system overview, and four
  crate books over a frozen 33-root, 14,525-line corpus. `book-validation` is
  a workspace member and deliberately outside that boundary: it is the
  authoring tool, not the system being documented, and decision 4 already
  earmarks it to leave for the walkthrough skill. Documenting it here would
  book a crate this grove intends to move out.
- **P2 · The publishing pipeline** — new session kinds and their skills.
- **P3 · The loop construct** — iteration modelled in grove and legible in the
  filesystem view.
- **P4 · Specification capture** — how human intent is captured in a form
  stakeholders can share and an LLM can consume. Research, not delivery.

## Done when

- `docs/walkthroughs/` holds a system overview and one book per crate for
  `grove-llm`, `jj-workspace`, `keyed-launch` and `grove-loop`, plus the
  relocated `ordinal-fs-tree` book, each proving complete reconstruction of its
  source roots under the fragment validator.
- `docs/USAGE.md` covers the `grove` binary completely against a **stated
  coverage inventory**: every subcommand and flag the binary exposes, each with
  a worked invocation, and every user journey from scaffolding a grove to
  teardown. The inventory is written down before the guide is edited, so
  "complete" names a checkable set rather than a judgement; the guide still
  owns its row in the documentation-ownership table, and an adversarial read
  against the inventory is what closes it.
- `docs/ARCHITECTURE.md` carries decisions, constraints and measurement records
  only; its descriptive account of runtime flow, command surfaces and module
  seams lives in the overview, with every citation of a moved anchor re-pointed
  and the link-integrity suite green — where that suite has first been widened
  to resolve `docs/ARCHITECTURE.md#<anchor>` citations **in Rust sources as
  well as Markdown**. It does not today, and the move is only mechanical once
  it does (see *Notes*).
- The fragment validator takes a per-book corpus rather than one compiled-in
  ledger, and `scripts/check.sh` gates every book.
- Decision 8 of `plan-k1` holds for books mechanically: every book root under
  `docs/walkthroughs/` is inside the curated user-documentation surface and has a
  tested row in `docs/ARCHITECTURE.md`'s *Documentation ownership* table, both by
  discovery, so a sixth book joins with no edit. Assigned to
  `book-assurance-surface-k39` after `walkthroughs-k9` found it assigned nowhere.
- The publishing pipeline exists as installed `grove-<kind>` skills, derived
  from what the pilot measured rather than asserted in advance — against a
  measure **preregistered before the pilot runs**: a judged outcome, a stated
  alternative to beat, and a decision rule mapping the evidence to which stages
  are kept, merged or dropped. Without preregistration any pipeline can be
  called derived after the fact. **Delivered** at `publishing-pipeline-k13`; see
  *The pipeline is installed* under *Pointers*.
- The loop construct has a filesystem representation a human can read without
  grove installed.
- P4 has reported: a research document under `docs/research/` answering
  `specification-capture-k4`'s six questions with citations, naming what
  neither survey found, and stating whether the two-tier hypothesis survives
  the evidence. A result that contradicts the hypothesis is a successful
  outcome, not a failed one.

## Decomposition

Three arms, ordered so the documentation is never blocked on unbuilt machinery.

- **`plan-k1`** — this session. Requirements. Retired.
- **`plan-k2`** — an adversarial read of this brief before anything is built on
  it. Eighteen decisions, four of them reversed mid-interview, and a long
  chain of sessions will execute them with no human present. Reported nine
  findings; `plan-k8` integrated them.
- **`walkthroughs-k3`** — planning for P1 and P2: the validator generalisation,
  the relocation, the spec split, the pilot cycle, and the scale-out. **The
  whole of P1 and P2 must sit ahead of the research pair in the walk** — as
  children of this node, or as siblings placed with `leaf-insert` — or the
  documentation arm queues behind a deep survey. `pick` is a depth-first
  pre-order walk over the tree and takes the first live non-`finish` leaf, so
  position in that walk is the only schedule there is.
  **Scale-out is gated on P2, not on P3.** Decision 12 reads "the kinds and the
  loop", but only the extracted kinds are machinery a book is authored
  through; the loop construct is a representation, and decision 17 already has
  it retrofitting as it lands. So the remaining books wait on the pilot and the
  pipeline extraction — both inside this arm — and never on
  `loop-construct-k7`.
  Reviewed by `walkthroughs-k9`, which reported five findings; `walkthroughs-k38`
  integrated all five — adding one leaf, turning the pilot's measurement leaf into
  a six-child node, and correcting the recorded reasoning behind the validator
  split and the guide-first ordering.
- **`specification-capture-k4/k5/k6`** — P4. A vendor pair, because the
  question is load-bearing enough to pay for two corpora and one survey's blind
  spots would decide it.
- **`loop-construct-k7`** — P3. Cut after the pilot has run editorial cycles by
  hand, so the construct is designed against a loop that actually ran. It sits
  after the research pair and blocks nothing: no book waits on it.

## Pointers

- **Method**: the `linkuistics:writing-code-walkthroughs` skill is the
  authoring method; its eight-field intake is answered in `plan-k1`'s decision
  log — except the source manifest, which is the next bullet — and must not be
  re-elicited.
- **Precedent**: `docs/walkthroughs/ordinal-fs-tree/` is the worked example
  every new book is uniform with, and `docs/specs/walkthrough-books.md` is the
  contract every book is built to. That spec was `ordinal-fs-tree-book.md` —
  one book's contract — until `walkthrough-books-spec-k20` rewrote it into a
  specification of the book *system* whose per-book data is a
  `walkthrough.toml` manifest beside each book.
- **The corpus, exactly.** A root is a file whose every byte a book must
  reconstruct. Per crate that is every `src/**/*.rs` file **plus the crate's
  own `Cargo.toml`** — that manifest is authored prose here and carries
  decisions, so it is documented rather than skipped. Two exclusions and no
  others: the workspace root `Cargo.toml`, which belongs to no crate, and
  `crates/grove-loop/src/task_grow/tests.rs` (1,680 lines), excluded as an
  inline test module rather than production source. `tests/` directories are
  evidence, not roots. That gives, per deliverable:

  | Deliverable | Source | Roots | Lines |
  | --- | --- | ---: | ---: |
  | overview | `crates/grove` | 3 | 204 |
  | `grove-llm` book | `crates/grove-llm` | 4 | 1,017 |
  | `grove-loop` book | `crates/grove-loop` | 13 | 10,533 |
  | `jj-workspace` book | `crates/jj-workspace` | 4 | 698 |
  | `keyed-launch` book | `crates/keyed-launch` | 9 | 2,073 |
  | **total** | | **33** | **14,525** |

  This is the eight-field intake's manifest field, and it is what makes decision
  12's "four roots, 698 lines" for `jj-workspace` reproducible. The relocated
  `ordinal-fs-tree` book keeps its own seventeen-file corpus unchanged. Counts
  are of the corpus as frozen; a book proves its own roots and no others.
- **What a new book owes beyond its own pages.** Every book root under
  `docs/walkthroughs/` is *discovered* by four checks rather than listed in any of
  them, so a sixth book joins all four by existing: `scripts/check.sh` runs
  `book-check --final --check all` over each book directory;
  `crates/grove/tests/corpus_exception_inventory.rs` requires each manifest's
  `[[corpus.add]]`, `[[corpus.exclude]]` and `[book].subject` to equal the
  normative tables in `docs/specs/walkthrough-books.md`; and
  `crates/grove/tests/reference_navigation.rs` both link-checks every page of
  every book as part of the curated user-documentation surface and fails on a
  book root with no row in `docs/ARCHITECTURE.md`'s *Documentation ownership*
  table. Discovery is what makes them checks rather than reminders, but two of
  them still demand an edit **outside** the book, and a book that skips either is
  red rather than silently unowned: its exception and subject rows in the
  specification's inventories, and one ownership row naming its contents page.
  Delivered by `walkthrough-machinery-k10`; the last of the four is
  `book-assurance-surface-k39`.

  **And its outbound links are checked** (`book-outbound-links-k49`, closing
  that node): a manifest declares `[guide]` as a path plus a non-empty `anchors`
  array or as `omitted` with a reason; a book declaring a path must cite one of
  those anchors from its `README.md` reader contract; a citation may name only a
  declared anchor; and every declared anchor must exist in its target as an
  explicit `<a id="…"></a>` line immediately preceding a heading — reported
  against the manifest whether or not any page cites it. So the guide-before-books
  ordering now binds rather than merely happens: `docs/USAGE.md` publishes the
  anchor set books reserve from, `CONTEXT.md` gains the glossary anchors the
  first citing book reserves, and no book validates before the anchors it
  reserves exist in the explicit form.
- **The pipeline is installed, and `crate-books-k14` is no longer gated on it.**
  `draft`, `copy-edit`, `art` and `proof` ship as `grove-<kind>` skills over one
  new family reference file, `grove/references/editorial.md`, and the human's
  `~/.config/grove/config.kdl` declares all four — `draft` and `art` on
  `claude/opus`, `copy-edit` and `proof` on `codex/sol-high`, because those two
  are the whole-document reads and a reader that did not write the draft is what
  a fresh context buys. Verified live: `leaf-add --kind draft …` writes leaves,
  an undeclared kind is still refused before the tree is mutated. **A book's leaf
  becomes a node with `leaf-decompose <leaf> <slug> --kind draft`**, and each
  stage's last act adds the next; every stage leaf carries the book's bare stem
  as its whole slug. The chain is lazy but its **membership is not optional** — a
  stage is never skipped on a judgement that it would find nothing — and there is
  no integrate step and no backward edge: a defect an earlier stage owns becomes
  a contiguous run of re-run leaves from that stage through `proof`. The four
  charters are `docs/adr/the-editorial-pipeline-is-four-kinds.md` and
  `docs/adr/a-feedback-edge-is-forward-tree-growth.md`; the shipped discipline is
  the skills themselves, and no book session re-derives it from those records.
- **Every book still needs a human structure brief, and `draft` stops without
  one.** That is not a courtesy: the developmental and technical edits were
  folded into `draft` on the evidence of a book drafted from such a brief, so
  without the brief the fold has no basis under it. `grove-draft` states the
  three things the named artifact must contain — who the reader is, the ordered
  section plan, what deserves emphasis — and that a node's automatic `BRIEF.md`
  is not by itself one.
- **The pilot has reported, and its result is `N = 4`.**
  `docs/evaluations/editorial-pipeline-pilot/README.md` is the measurement,
  evaluated against the preregistration committed before the book it judges.
  The surviving pipeline is **draft, copy edit, art, proof**: the copy edit and
  the art stage each cleared the preregistered threshold (five and six marginal
  reader-facing defects the simulated two-stage arm did not reach), and the
  developmental and technical edits reached two each and are **`Merge`d into the
  draft** — no kind, their charters folded in. No stage is `Undetermined` and
  none is `Drop`, so `pipeline-kinds-k27` extracts two kinds rather than four and
  is not entitled to more. Neither the six-stage design nor the two-stage
  fallback is upheld; the middle outcome is the one the evidence supports. **The
  merge target is the draft, and it is only safe while each book keeps getting a
  human structure brief** — that brief is what pre-empts the two merged charters,
  and the report names this as its most important limit.
- **Ownership**: `docs/ARCHITECTURE.md`, *Documentation ownership* — one
  canonical source per subject, and the rule bounding what may sit directly
  under `docs/`.
- **Contexts**: `CONTEXT-MAP.md` argues that `grove`, `grove-llm` and
  `grove-loop` **are** the grove context and that `jj-workspace` and
  `keyed-launch` are deliberately not contexts. A per-crate directory under
  `docs/` would assert boundaries that document denies; `docs/walkthroughs/` is
  chosen to assert none.
- **Kinds**: `docs/adr/a-kind-is-an-open-token.md` — a kind exists iff a
  `grove-<kind>` skill exists, and every new kind needs a launch template in
  the human's personal configuration before it can run.
- **Reopened by P3**: `one task is one session`,
  `docs/adr/entries-are-never-removed.md`, and the outcome-partition records.
  The loop construct cannot be built without reopening them.
- **In-house prior art**, all on disk and all live: `Writegood` (an
  optimisation-loop grove with a judging protocol and a measurement corpus),
  `grove.gh-issue-12` (the preregistered evaluation behind
  `docs/evaluations/writing-code-walkthroughs/`), and `TheGreatExplainer`
  (whose requirements §1.6 specifies exactly this pipeline, with feedback edges
  and human gates).
- **Test seams**: `every_repository_markdown_reference_resolves`
  (`crates/grove/tests/reference_navigation.rs`) sweeps the whole repository and
  covers new books for free; the `include_str!` content assertions in
  `crates/grove-llm/tests/composition_guidance.rs` go red on the ARCHITECTURE
  move, by design; `book-check` proves structure and reconstruction;
  `scripts/check.sh` is the umbrella. **The sweep is Markdown-only**, and
  `every_adr_citation_names_a_decision_record` — the one check that reads `.rs`
  — recognises the `ADR <slug>` form and not a raw
  `docs/ARCHITECTURE.md#<anchor>` link. Twenty-six such raw citations live in
  Rust sources today, concentrated in `crates/grove-loop/src/task_tree.rs` and
  `tree_lifecycle.rs`. Widening the resolver to enumerate architecture anchors
  and resolve them from both surfaces is on the critical path of the
  relocation, and `walkthroughs-k3` owns placing it there.

## On the horizon

- **A rendering surface for a grove and its loops** — a verb, or the DevTUI
  direction. Deliberately not now: the filesystem representation is the minimal
  UI and must be settled first.
- **Relocating the fragment validator to the `writing-code-walkthroughs`
  skill**, so it works over any repository. Blocked on a question with no
  precedent here — `plugins/` ships shell scripts and has never shipped a
  compiled artifact.
- **Whether `CONTEXT.md` is two artifacts fused** — a shareable ubiquitous
  language and a maintainer's record of retired mechanisms — and whether the
  shareable one can be projected out rather than maintained twice. Stated
  precisely enough to leaf once `specification-capture` reports.

## Notes

**The corpus is frozen and no session fixes code inline.** A defect found while
documenting becomes its own leaf. This is not tidiness: the fragment ledger
holds an exact line count per source root, so an inline fix shifts every line
below it and silently breaks a page a finished session already proved.

**And a defect leaf is not licensed to break the freeze either.** "Becomes its
own leaf" says who fixes it, not what the fix may commit — and a code change
invalidates the very ranges the freeze protects. The book contract already
carries the missing invariant for one book: *an accepted source change requires
the affected ownership ranges and fragments to change, followed by final
validation against the new bytes* (`docs/specs/walkthrough-books.md`). The
campaign takes the cross-book form of it. **One commit carries the source
change, every affected ledger and page, and a green run of the validator over
every book it touched.** A defect leaf that cannot land that whole set does not
land at all: it is deferred behind the books it would invalidate, and says so in
its task file. Without this rule, "fix nothing inline" and "finished pages stay
proved" become incompatible the first time a book cannot be written truthfully
around a real bug.

**The pilot's job is to measure, not merely to run.** `Writegood`'s own
`optimisation-loop` record argues that expensive machinery ordered ahead of the
measurement that would justify it cannot answer whether it was needed. Nothing
in this brief can currently tell whether six editorial stages beat two, because
`book-check` proves structure and reconstruction and never whether a page
explains anything. Producing that measure is the pilot's deliverable, alongside
its book — and **the measure is preregistered, not reported**. Recording what
each stage changed is activity provenance and will justify any pipeline after
the fact; before the pilot runs, four things are written down and committed: the
judged outcome, the alternative the six stages must beat (two stages — draft and
proof — is the stated fallback), the attribution rule saying how a change is
credited to a stage, and the decision rule for keeping, merging or dropping one.
`grove.gh-issue-12` and `docs/evaluations/writing-code-walkthroughs/` are the
in-house form of exactly this, and the pilot follows them. A stage that cannot
be shown to have paid for itself is not extracted into a kind.

**The source does not contain enough to structure a book.** Audience,
conceptual order and what deserves emphasis are nowhere in the code. Each book
therefore takes a human-authored structure brief as an input artifact. That is
the manual form of P4, and it is available today — the interview in `plan-k1`
was one.

**Formal specifications are poor human artifacts and good LLM artifacts.** The
capture layer stakeholders share and the formal tier a coding phase consumes
are two layers, not one notation. A format that tries to be both fails at both.
This is the human's position and the constraint `specification-capture` is
commissioned against — and it is the arm's **hypothesis, not its premise**. A
survey commissioned against a hypothesis tends to find it, which is why
`specification-capture-k6` owes the adversarial move and a stated verdict.
Evidence that a single notation has worked is a result this grove wants, not a
result it is set up to miss. Nothing downstream of P4 may cite the two-layer
form as settled before k6 reports.

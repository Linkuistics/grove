# Context Map

Three bounded contexts share this repo. **A context is a language boundary, not
a delivery path.** The two are separate questions. The first two contexts happen
to answer both together, since each also ships by its own path. They change in
lockstep, which is why they live together (see
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#skills-monorepo)).

This repo declares the third context, `ordinal-fs-tree`, on vocabulary alone. It
has a glossary whose terms mean something else in grove's. Its crate ships by no
path of its own. The crate rides inside grove's cut and wears grove's version.
The release does not publish it separately ([release packages](docs/RELEASING.md#one-release-eight-packages-one-tag)).

Ravel is an external project reached through its public CLI, with its own
release, installed prefix and executable documentation.

## Contexts

- [grove](./CONTEXT.md) — the `grove` CLI, the workstream methodology shipped as
  the `grove` plugin, and the task tree that methodology drives.
- [skills](./plugins/CONTEXT.md) — the `grove`, `linkuistics` and `testanyware`
  skill plugins: how to author, package, trigger and install a skill. The
  `grove` plugin's *contents* belong to the grove context, by that glossary's own
  scope boundary. This context holds only its packaging and delivery.
- [ordinal-fs-tree](./docs/ordinal-fs-tree/CONTEXT.md) — the domain-independent
  ordered-tree library extracted from grove's tree modules: entries, ordinals,
  keys, and the algebra over them. Its completed
  [code walkthrough](./docs/walkthroughs/ordinal-fs-tree/README.md) is the self-contained
  reader-facing account of the crate and demonstration CLI. Its glossary sits beside
  [its architecture](./docs/ordinal-fs-tree/ARCHITECTURE.md). The crate lands at
  `crates/ordinal-fs-tree/`. It is a member of a workspace whose root is no package at
  all. The glossary, the architecture, the models and
  [the CLI document](./docs/ordinal-fs-tree/CLI.md) **stay** under
  `docs/ordinal-fs-tree/` while the crate lives in this repo. The reason is that
  `docs/adr/` is flat and repo-wide and four artifacts link into that path. These
  files move with the crate only if someone extracts the crate to a repository of
  its own.

**`crates/jj-workspace` is another crate and deliberately not a fourth
context.** A context is a language boundary, and there is none here. Every term
in that crate — workspace, main repo, tracked, commit, change id — is Jujutsu's.
Grove's own glossary already uses those words with Jujutsu's meanings
([`CONTEXT.md`](./CONTEXT.md), *Task commit boundary*).

What the crate adds is a *namespace* it will not name for its consumer. That is an
interface property rather than a vocabulary of its own. The crate's decisions live
in the grove context that owns them: [decision 8](./docs/specs/module-decomposition.md)
for the interface, [*jj is the only lane*](./docs/adr/jj-is-the-only-lane.md) for the
refusal. A separate context becomes worth declaring only if it grows terms whose
meaning departs from grove's.

Namespace discovery is read-only and belongs to this same seam. It accepts an exact
location and namespace, returns an existing path or absence, and invokes no jj
command. Runtime record interpretation and descriptor identity validation
remain Grove's responsibilities. Discovery grants no ownership or admission.

**`crates/grove-loop`, `crates/grove` and `crates/grove-llm` are three more
crates and, unlike the extractions above, not even candidate contexts — they *are* the
grove context.** The other extractions each had to argue their way out of
declaring one, because each took a vocabulary with it. These took grove's own.
`grove-loop` is where *kind*, *handle*, *brief chain*, *outcome*, *selection* and
*finishing* live. [`CONTEXT.md`](./CONTEXT.md) is already their glossary.

`crates/grove` is the human's entry point onto the loop. `crates/grove-llm` is
the session's verb surface over it. Neither adds a word. A context is a
language boundary. Moving a module across a crate boundary inside one
language is not a language boundary.

What the split does buy is the **collision table's** enforcement rather than its
contents. Six packages now compile separately, so a word that meant two things
across them would have to be spelled twice to compile at all. Two of the
rows the table keeps apart, *entry* and *key*, are also exactly the words
`ordinal-fs-tree` and `grove-loop` both use.

Nothing in `grove-loop` re-declares either. It takes the store's `Key` and the
store's `Entry`. It adds grove's own `Kind`, `Handle` and `Outcome` beside them.
That is the shape the table asks for. Its decisions are the grove context's, as ever:
[decision 1](./docs/specs/module-decomposition.md) for the crate split and
[decision 9](./docs/specs/module-decomposition.md) for the loop's own surface.

**Ravel is an independent external language and command boundary.** Its
[CLI](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md) and
[executable specification](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md)
define policy requests, selected commands, parameters, context, settings and
opaque session results. The earlier in-repository dispatch exception ended.
Ravel maintains the executable/runtime obligations, while Grove keeps its
caller contract, prompt/result conventions, provider receipts and readable
adapter. Retired store terms in Grove's glossary are historical locators, not
active Ravel APIs. The [relationship below](#grove-and-harness-dispatch) keeps
shared words distinct without duplicating Ravel's specification.

**Where every term lives, one line per module.** Three of Grove's crates carry
vocabulary, and the plugin carries a fourth. No two of them own the same word.
The external Ravel boundary has the last row:

| module | owns | glossary |
|---|---|---|
| `ordinal-fs-tree` | *entry*, *leaf*, *node*, *ordinal*, *key*, *distinguished child*, *snapshot*, *guard*, *refusal*, *sought*, *promote* | [`docs/ordinal-fs-tree/CONTEXT.md`](./docs/ordinal-fs-tree/CONTEXT.md) |
| `jj-workspace` | *workspace*, *main repo*, *control directory*, *namespace*, *tracked*, *commit*, *change id* | none — the words are Jujutsu's, with Jujutsu's meanings |
| `grove-loop`, with `grove`, `grove-llm` and the `grove-tui` reader over it | **Session kind**, **Work-item handle**, **Position**, **Permanent key**, **Leaf**, **Node directory**, **Node file**, **Brief chain**, **Selection**, **Driver lease**, **Session epoch**, **Launch directory**, **Teardown record**, **Guaranteed core**, **Stated VCS** | [`CONTEXT.md`](./CONTEXT.md) <!-- tc: depart long-sentence because the cell lists glossary terms and cannot split --> |
| the `grove` plugin | **Spine skill**, **Kind skill**, **Composed loaded path**, **Condition** / **procedure**, **Loop-step reference file** | [`CONTEXT.md`](./CONTEXT.md) for the terms, [`plugins/CONTEXT.md`](./plugins/CONTEXT.md) for packaging and delivery |
| Ravel (external) | *policy*, *selected command*, *parameter*, *owner settings*, *choice file*, *context*, opaque *result* | [Ravel specification](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md). Grove's **Original creator**, **Creator reference** and **Grove adapter** remain in [`CONTEXT.md`](./CONTEXT.md) |

The `jj-workspace` row is the one that had to be *bought*. The crate refuses to
name its consumer. That is a naming decision argued above rather than an accident
of scope.

## Relationships

- **grove → skills, a documentation-level prerequisite that binds without the
  install.** grove's methodology cites `linkuistics:decision-records` for ADR
  philosophy and `linkuistics:codebase-design` for seam judgement. Installation does
  not enforce the dependency for marketplace/manual users. Current
  Grove embeds and provisions compatible Codex skills. The methodological
  prerequisite still binds without the dependency installed. Every citation states what binds in the plugin's absence.

  Grove owns locally the part whose absence would change *what* a session
  writes. That part is the ADR when-to-write test and minimum-coherent-set discipline
  (`plugins/grove/skills/grove/ADR-FORMAT.md`), the operative seam rules
  (`plugins/grove/skills/grove/SPEC-FORMAT.md`) and the commit boundary
  (`plugins/grove/skills/grove/references/commit.md`).
  `plugins/grove/skills/grove/references/grove.md` is the hub that states this
  once per skill. The spine's `SKILL.md` routes any citation there.

  So a
  checkout without the plugin runs an ordinary task end to end. Absence
  changes how *well* a session writes these artifacts, never *what* is obliged.
  Decision: `docs/adr/a-skill-states-what-binds-without-its-dependencies.md`.
  Enforcement: `crates/grove/tests/plugin_fallback.rs`.

- **Shared target: personal skill directories.** Current Grove embeds the
  compatible plugin snapshot and automatically provisions Codex links in
  `~/.agents/skills` through its staged snapshot and shared installation lock.
  Foreign entries refuse rather than being overwritten. Claude Code uses its
  marketplace. `plugins/install.sh` supports manual Codex, Gemini and Pi links.
  These are separate delivery routes for the same plugin source
  ([provisioning contract](docs/specs/codex-skill-provisioning.md)). The old
  multi-harness `content/` sweep remains retired. Startup alignment does not pin
  the lifetime of an already running session against another build's updates.

- **grove → ordinal-fs-tree, a vocabulary boundary held by hand.** grove is the
  library's first and so far only consumer. It supplies a domain implementation
  of the one trait and keeps its own words out of the library. The boundary is
  **not compiler-enforced** and cannot be, because the two vocabularies name the
  same filesystem shapes. So the two glossaries and this table hold it.
  Neither glossary defines the other's sense of any row.

  | the library says | grove says | class |
  |---|---|---|
  | *leaf* — any regular-file entry | **Leaf** — a task file executed in one session | the words collide |
  | *node* — any directory of children | **Node directory** — a directory with exactly one **Node file** | the words collide |
  | *ordinal* | **Position** | the words differ |
  | *key* | **Permanent key** | the words differ |
  | *distinguished child* | **Node file** — `_<slug>.md`, or `_BRIEF.md` at the root | the role corresponds. Grove requires one per level |
  | *label* | a work item's title / slug, in its leaf filename or node filename | the library never reads the value |
  | *entry* | **Leaf** or **Node directory** | grove has no word for the union |
  | *promote* | `leaf-decompose` | the operations coincide, the names do not |

  Where the words collide, the meanings differ. Where the words differ, the meanings
  match. Both halves are deliberate. A document that must speak of both says
  which tree it means, sentence by sentence.

  **Operators read the table at runtime, not only in prose.** grove prints the
  library's errors verbatim rather than re-wording them.
  [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#library-refusals) carries which
  ones an operator can actually reach, and why grove rejected re-wording. So an
  operator meeting one reads it against these rows.

  Under `.grove/` the first two
  rows happen to pick out the same files. That is what makes that collision
  quiet rather than loud. A sentence about the library's *leaf* reads as a true
  sentence about a grove **Leaf** right up to the clause where it is not.

  The *promote* row is the one whose absence would bite.
  `refusals-k30` measured the collision clause by clause. It found that the collision
  lands on the **verb** a message names rather than on its nouns.
  grove's operator is an LLM that will try the verb it is told to try.

  The *promote* row is also the row that two messages now share. They are the library's
  *interrupted promotion* and grove's own diagnosis of the tree a failed rollback
  leaves ([`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#interrupted-promotion)).
  That is exactly the drift a map prevents and a re-wording does not.

- <a id="grove-and-harness-dispatch"></a>**grove → Ravel, a public CLI
  boundary.** Grove resolves the first executable regular Ravel on absolute
  PATH once and requires stable 1.x from 1.1.0. Launch and ending instructions reuse
  that canonical path. There is no incompatible-install fallback. Grove supports separate
  prefixes and versions. Ravel links no Grove crate. Grove links
  no Ravel crate. Ravel supervises its managed process, and Grove launches
  `ravel` as an ordinary child process. Grove's Homebrew dependency and
  readable `libexec/ravel-policy` assets describe installation, not shared
  release identity ([caller boundary](docs/ARCHITECTURE.md#harness-dispatch)).

  | Ravel says | Grove says | class |
  |---|---|---|
  | *kind* — caller data | **Session kind** — well-formed filename token | Grove passes its token unchanged |
  | *task identity* — public `--task-id` data | **Work-item handle** | Grove supplies its handle |
  | *selection* — policy chooses argv | driver selects one leaf | the words collide |
  | *session* — supervised process lifetime | one mandated task attempt | Grove supplies the task discipline |
  | *context* — optional measured policy data | agent context or bounded context | the words collide |
  | opaque *result* | Interruption, successful `grove-force-stop`, **Teardown record**, then successful `grove-continue` | Grove supplies meaning. Ravel returns the argument after cleanup |

  Grove supplies prompt and exact end-session instructions unchanged. Ravel
  injects nothing and interprets no continue/force-stop convention. Grove's
  optional provider wrapper and task adapter are ordinary SDK imports. Creator
  receipts stay caller-owned and require no run store. Original live-v22
  launches alone retain dispatch's old empty signal, report and Creator-run
  contract through their last inherited `harness-dispatch exit`.

- **A durable record has one owner.** Every record under `docs/adr/` and
  `docs/specs/` has a repo-wide unique slug and a maintaining context recorded
  here. A record added later joins this list. Ownership names who keeps the
  record current, not every component it binds. For example, the Grove binary
  and the plugin installer share the jj-first VCS rule.
  `plugins/grove/skills/grove/ADR-FORMAT.md` defines when a flat
  root set is appropriate.

  A term's definition lives in the glossary of its owning
  context, never in both. The **grove** context owns
  [`harness-selection-is-owned-by-policy`](docs/adr/harness-selection-is-owned-by-policy.md),
  [`a-review-carries-its-creator-reference`](docs/adr/a-review-carries-its-creator-reference.md),
  [`policy-evaluation-precedes-the-launch`](docs/adr/policy-evaluation-precedes-the-launch.md),
  [`dispatch-supervises-the-harness`](docs/adr/dispatch-supervises-the-harness.md),
  [`grove-owns-escalated-review`](docs/adr/grove-owns-escalated-review.md),
  [`one-live-driver-per-working-tree`](docs/adr/one-live-driver-per-working-tree.md),
  [`corpus-rules-have-one-owner`](docs/adr/corpus-rules-have-one-owner.md),
  [`restatement-declares-its-class`](docs/adr/restatement-declares-its-class.md),
  [`behavioural-coverage-asserts-delivery`](docs/adr/behavioural-coverage-asserts-delivery.md),
  [`task-names-are-canonical`](docs/adr/task-names-are-canonical.md),
  [`grove-does-not-stage-its-own-renames`](docs/adr/grove-does-not-stage-its-own-renames.md),
  [`bulk-marks-are-not-atomic`](docs/adr/bulk-marks-are-not-atomic.md),
  [`obligations-follow-context-not-artifact`](docs/adr/obligations-follow-context-not-artifact.md),
  [`a-refusal-leaves-nothing-standing`](docs/adr/a-refusal-leaves-nothing-standing.md),
  [`a-witnessless-root-refuses-what-it-cannot-account-for`](docs/adr/a-witnessless-root-refuses-what-it-cannot-account-for.md),
  [`a-closed-partition-is-over-outcomes-not-states`](docs/adr/a-closed-partition-is-over-outcomes-not-states.md),
  [`a-lifecycle-claim-says-what-it-is-over`](docs/adr/a-lifecycle-claim-says-what-it-is-over.md),
  [`a-shared-safety-claim-names-the-role-not-the-artifact`](docs/adr/a-shared-safety-claim-names-the-role-not-the-artifact.md),
  [`evidence-outlives-the-instrument`](docs/adr/evidence-outlives-the-instrument.md),
  [`the-launched-child-is-a-job`](docs/adr/the-launched-child-is-a-job.md),
  [`a-book-cannot-witness-its-own-corpus`](docs/adr/a-book-cannot-witness-its-own-corpus.md),
  and the specs
  [`doubt-grove-review-mechanics`](docs/specs/doubt-grove-review-mechanics.md),
  [`module-decomposition`](docs/specs/module-decomposition.md),
  [`harness-selection-and-execution`](docs/specs/harness-selection-and-execution.md) and
  [`walkthrough-books`](docs/specs/walkthrough-books.md). The walkthrough records
  describe the book system and are **grove's while its validator is**.
  That holds because `crates/book-validation/` is a member of this workspace, and
  `docs/walkthroughs/` is a surface `scripts/check.sh` gates and
  [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#documentation-ownership) accounts
  for. They are not the *skills* context's despite documenting that context's
  `writing-code-walkthroughs` method, because that glossary's own scope boundary
  excludes a skill's subject matter.

  Ownership follows the validator if anyone ever relocates it into the skill.
  [`root-lifecycle-belongs-to-the-store`](docs/adr/root-lifecycle-belongs-to-the-store.md)
  already made the same move, and made it only once the extraction happened. The rule
  inventory `corpus-rule-ownership` held is now
  [`plugins/grove/conformance/rules.tsv`](plugins/grove/conformance/rules.tsv),
  which is data the conformance runner reads rather than a spec.

  The **skills** context owns
  [`behavioral-acceptance-is-closed-unmet`](docs/adr/behavioral-acceptance-is-closed-unmet.md),
  which records the acceptance boundary for the shipped
  `writing-code-walkthroughs` skill. It also owns the spec
  [`technical-communication`](docs/specs/technical-communication.md) and the
  records
  [`technical-communication-carries-no-standard-text`](docs/adr/technical-communication-carries-no-standard-text.md),
  [`documents-are-gated-replies-are-measured`](docs/adr/documents-are-gated-replies-are-measured.md)
  and [`the-off-file-is-the-only-switch`](docs/adr/the-off-file-is-the-only-switch.md),
  which describe the always-on `technical-communication` skill.

  The **ordinal-fs-tree** context owns
  [`entry-name-is-the-only-seam`](docs/adr/entry-name-is-the-only-seam.md),
  [`entries-are-never-removed`](docs/adr/entries-are-never-removed.md) and
  [`root-lifecycle-belongs-to-the-store`](docs/adr/root-lifecycle-belongs-to-the-store.md).
  The last of these **moved** here, and the move is the decision it records.
  While root creation and destruction were grove's, so was the record, and it was
  filed under grove for that reason. They sit
  in the flat root set like every other record.
  `plugins/grove/skills/grove/ADR-FORMAT.md`'s split
  rule keeps one directory while grove occupies the repo root. The rule
  says to record ownership instead.

  grove consumes both decisions and maintains
  neither. Their subject is the library's public surface and its key allocation.
  Recording grove as their maintainer would say the extraction had not happened.

  [`docs/ordinal-fs-tree/ARCHITECTURE.md`](docs/ordinal-fs-tree/ARCHITECTURE.md)
  explains both at length. It is not a duplicate. The document says what the design *is*, for someone
  building against it. The records say what it cost and what changing it
  would cost, for someone proposing to change it.

  The first two earn their place the same way. One describes how the embedded
  corpus files its rules, an area no single increment finishes. The other describes a
  composition **between** two contexts. Both outlive the increment that wrote
  them. The doubt skill
  participates in the review-ownership and promotion contracts, but the mandate,
  task tree, review routing, and lifecycle are Grove's maintaining seam.

  The former spec `semantic-contract` stated the tool-neutral semantics of the
  task tree, the finish protocol and the lifecycle joining them. It was
  **deleted** with the formal-methods apparatus that checked it
  (`delete-formal-models-k29`). The last directory of that deletion,
  `crates/grove-finish/models/`, the finish protocol's own column, went at
  `delete-finish-models-k30`. It went for the reason the paragraph below gives
  for `module-decomposition` in advance. A spec earns its place by describing
  something no other artifact holds. Once its checker is gone and the design
  it specified is being dismantled, `docs/ARCHITECTURE.md` was where the
  current-state description belonged.

  The module contract earns its place on the grain rule rather than on novelty.
  `module-decomposition` was written as the agreement point a whole chain of
  leaves built against, across many sessions and three bounded contexts. It carried a
  retirement condition: *delete it once the crates exist*.
  `spec-to-current-state-k23` reached that condition and did not exercise it.

  What it describes is **how the module boundaries work**. That is a spec's own
  grain and not `docs/ARCHITECTURE.md`'s. The architecture records the
  decisions, constraints and measurements behind the runtime a reader is
  operating. Its description of what that runtime does moved into the
  walkthrough books: the system overview first, and each crate's internals with
  that crate's own book. An ADR records one decision
  and its trade-off. Neither states the five interfaces as a set a consumer
  builds against.

  Four
  artifacts link into it. Source comments, `Cargo.toml` headers and tests across
  the shipped packages cite its decisions *by number*. So the
  numbering is part of the contract. It was rewritten to current state instead.
  The rewrite lost its `## Problem`, its *what changes* framing and its
  ADR-reconciliation table, which were the transient halves. It kept the
  decisions, the requirements and the test seams.

  A spec that only ever described one shipped increment of Grove's own runtime is
  not a durable record. It is `docs/ARCHITECTURE.md`'s subject written twice.
  `config-driven-sessions` and `skill-delivered-methodology` were folded there on
  those grounds, as `mandate-delivered-methodology` went with the machinery it
  described.

<!-- tc: pending because the conversion has not reached this part of the document -->


## Choosing a context

A topic about the `.grove/` tree, the loop, the CLI verbs, or the binary is
**grove**. A topic about writing, packaging, triggering or installing a
`SKILL.md` is **skills**.

A topic about entries, ordinals, keys, the distinguished child, or the tree
algebra over them is **ordinal-fs-tree**, which is now also where that code
lives: grove's own tree algebra was deleted once the library owned it
(`docs/ARCHITECTURE.md`, *The withdrawn tree algebra*), and what grove keeps is
a domain implementation of the seam plus the lifecycle around the tree. A topic
about `.grove/` *as a task tree* stays **grove** even when it is
about the same directories on disk: the discriminator is which vocabulary the
answer is stated in, not which files it touches.

A topic about generic policy selection, the selected process, settings,
measured context or ephemeral completion is **Ravel**, outside this repo. Use
its [CLI](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md) and
[specification](https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/specs/session-execution.md),
and qualify [shared words](#grove-and-harness-dispatch). A topic about Grove's
prompt/result conventions, provider receipts, task/Creator adapter or installed
caller assets is **grove**. Historical dispatch stores are neither current
Ravel nor a new Grove persistence API.

The word "plugin" in this repository refers to the Claude Code skill plugins
under `plugins/`; route their authoring, packaging, triggering, and installation
to the **skills** context.

A skill's *subject matter* belongs to neither glossary — jj's model lives in
`using-jujutsu` and the architecture's
[VCS seam](docs/ARCHITECTURE.md#symmetric-vcs-rule), Testanyware's VM vocabulary
in `using-testanyware`. If a topic seems to need both glossaries, say so rather
than picking one.

## Viewer ownership

`crates/grove-tui` shares Grove's task vocabulary and consumes the public typed
`try_observe` capture. The loop owns whole-tree key/finish validation,
exclusion-aware selection, selected-file reads and the opaque TreeLifetime.
The viewer owns two-capture acceptance, presentation rows, display and in-memory
interaction state. The loop's private driver_lease observer adds independent
Idle/Running/Busy/Unavailable runtime evidence after tree capture, sharing mandatory
record parsers without session admission or a lease-lock probe. The lease writes
the versioned selected-mandate extension and Started marker through its paired
witness owner; directory-before-private probes verify Running and its relation
to the captured tree. Independent activity acceptance displays idle NEXT and
same-tree RUNNING with exclusion-aware NEXT from the shared selector, using
current row identity and persistent summaries, with no retained row activity
on failed observation. Missing or failing trees withhold NEXT while retaining
the independently compared runtime summary. Missing items retain their launch
handle with `item absent`; missing or unreadable trees use `tree absent` or
`tree unavailable`. Replacements report `previous tree`, attach no old RUNNING
row and exclude no reused key from NEXT. Replacement resets saved interaction
state; same-root brief edits preserve it.
The [viewer architecture](docs/ARCHITECTURE.md#read-only-viewer) names its
application seam; the [usage guide](docs/USAGE.md#usage-viewing-tree) states the
current interaction and refresh behavior. The
[item-status design](docs/specs/item-status.md) owns the full-width interaction
and typed activity contract; the loop owns runtime protocol knowledge.

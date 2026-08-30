# Context Map

Three bounded contexts share this repo. **A context is a language boundary, not
a delivery path** — the two are separate questions that the first two contexts
happen to answer together, since each also ships by its own path and they change
in lockstep, which is why they live together (see
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#skills-monorepo)). The third,
`ordinal-fs-tree`, is declared on vocabulary alone: it has a glossary whose terms
mean something else in grove's, and its crate ships by no path of its own —
`release.toml` excludes it from grove's cut.

## Contexts

- [grove](./CONTEXT.md) — the `grove` CLI, the workstream methodology it embeds
  in `content/`, and the task tree that methodology drives.
- [skills](./plugins/CONTEXT.md) — the `grove`, `linkuistics` and `testanyware`
  skill plugins: how a skill is authored, packaged, triggered and installed. The
  `grove` plugin's *contents* are the grove context's, by that glossary's own
  scope boundary; what is here is its packaging and delivery.
- [ordinal-fs-tree](./docs/ordinal-fs-tree/CONTEXT.md) — the domain-independent
  ordered-tree library extracted from grove's tree modules: entries, ordinals,
  keys, and the algebra over them. Its glossary sits beside
  [its architecture](./docs/ordinal-fs-tree/ARCHITECTURE.md). The crate lands at
  `crates/ordinal-fs-tree/`, a member of a workspace whose root package stays
  `grove`; the glossary, the architecture, the models and
  [the CLI document](./docs/ordinal-fs-tree/CLI.md) **stay** under
  `docs/ordinal-fs-tree/` while the crate lives in this repo, because
  `docs/adr/` is flat and repo-wide and four artifacts link into that path. They
  move with the crate only if it is extracted to a repository of its own.

**`crates/jj-workspace` is a fourth crate and deliberately not a fourth
context.** A context is a language boundary, and there is none here: every term
in that crate — workspace, main repo, tracked, commit, change id — is Jujutsu's,
and grove's own glossary already uses those words with Jujutsu's meanings
([`CONTEXT.md`](./CONTEXT.md), *Task commit boundary*). What the crate adds is a
*namespace* it will not name for its consumer, which is an interface property
rather than a vocabulary of its own. Its decisions live in the grove context
that owns them: [decision 8](./docs/specs/module-decomposition.md) for the
interface, [*jj is the only lane*](./docs/adr/jj-is-the-only-lane.md) for the
refusal. A separate context becomes worth declaring only if it grows terms whose
meaning departs from grove's.

**`crates/keyed-launch` is another crate and, for the same reason, not another
context — but only because it was named to avoid being one.** Its vocabulary is
*key*, *template*, *slot*, *argv*, *launch*, *overlay*: a key is an opaque string
a consumer names, and a slot is a name that consumer declares. None of those
words appears in grove's glossary meaning something else. The word it would have
reached for is **session**, and using it would have put a third meaning beside
grove's **Session** and the methodology's, adding a row to the collision table
below for nothing — the crate never learns what a launch is *for*. Grove's
mapping is one line: a **Session kind** is a key. Its decisions live in the grove
context that owns them:
[decision 7](./docs/specs/module-decomposition.md) for the interface,
[*complete session configuration*](./docs/adr/complete-session-configuration.md)
for what a template must be, and
[*the untracked configuration delta*](./docs/adr/untracked-configuration-delta.md)
for the second document.

## Relationships

- **grove → skills, a documentation-level prerequisite that binds without the
  install.** grove's methodology cites `linkuistics:decision-records` for ADR
  philosophy and `linkuistics:codebase-design` for seam judgement, and the
  dependency is **not install-enforced** — the `grove` binary provisions its own
  methodology and nothing else, so the plugins remain a separate install. It is
  no longer **silent**: every citation states what binds in the plugin's absence,
  and grove owns locally the part whose absence would change *what* a session
  writes — the ADR when-to-write test and minimum-coherent-set discipline
  (`content/ADR-FORMAT.md`), the operative seam rules
  (`content/SPEC-FORMAT.md`), and the commit boundary
  (`content/references/commit.md`). `content/references/grove.md` is the hub that
  states this once per skill; `content/SKILL.md` routes any citation there. So a
  checkout without the plugin runs an ordinary task end to end, and absence
  changes how *well* these artifacts are written, never *what* is obliged.
  Decision: `docs/adr/grove-binds-without-the-plugin.md`; enforcement:
  `tests/plugin_fallback.rs`.

- **Shared target: the personal skill directory.** grove and skills both write
  into the same per-harness namespace (`ordinal-fs-tree` provisions nothing). The `grove` binary sweeps `content/` to
  `~/.claude/skills/grove/` (and the Codex and Pi equivalents);
  `plugins/install.sh`
  symlinks each **`linkuistics`** skill into `~/.codex/skills/`,
  `~/.gemini/skills/` and `~/.pi/agent/skills/` (`testanyware` ships by
  marketplace only).
  Nothing collides today — the overlap is `~/.codex/skills/`, where the names are
  disjoint — but the namespace is shared, so any future decision to have one
  context provision the other's content is a question about precedence and
  double-provisioning, not a local change. Contention *within* the `grove` entry
  — two `grove` builds writing it — is a separate, grove-owned question, settled
  by [`one-build-owns-a-session`](docs/adr/one-build-owns-a-session.md): the
  directory is owned by whichever build wrote it last, and the driver re-verifies
  the stamp before every launch.
  [`skill-delivers-the-methodology`](docs/adr/skill-delivers-the-methodology.md)
  is why the `grove` half of this shared target exists at all — the provisioned
  skill is the delivery path, so the entry stays and the precedence question
  stays open alongside the `linkuistics` symlinks. What reopens it is one context
  provisioning the other's content, which no part of either design does.

- **grove → ordinal-fs-tree, a vocabulary boundary held by hand.** grove is the
  library's first and so far only consumer: it supplies a domain implementation
  of the one trait and keeps its own words out of the library. The boundary is
  **not compiler-enforced** and cannot be — the two vocabularies name the same
  filesystem shapes — so it is held by the two glossaries and by this table.
  Neither glossary defines the other's sense of any row.

  | the library says | grove says | class |
  |---|---|---|
  | *leaf* — any regular-file entry | **Leaf** — a task file executed in one session | the words collide |
  | *node* — any directory of children | **Node directory** — a directory headed by a `BRIEF.md` charter | the words collide |
  | *ordinal* | **Position** | the words differ |
  | *key* | **Permanent key** | the words differ |
  | *distinguished child* | the node's `BRIEF.md` charter | grove names the file, not the role |
  | *entry* | **Leaf** or **Node directory** | grove has no word for the union |
  | *promote* | `leaf-decompose` | the operations coincide, the names do not |

  Where the words collide the meanings differ, and where they differ the meanings
  match; both halves are deliberate. A document that must speak of both says
  which tree it means, sentence by sentence.

  **The table is read at runtime, not only in prose.** grove prints the library's
  errors verbatim rather than re-wording them
  ([`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#library-refusals) carries which
  ones an operator can actually reach, and why re-wording was rejected), so an
  operator meeting one reads it against these rows. Under `.grove/` the first two
  rows happen to pick out the same files, which is what makes that collision
  quiet rather than loud: a sentence about the library's *leaf* reads as a true
  sentence about a grove **Leaf** right up to the clause where it is not. The
  *promote* row is the one whose absence would bite: `refusals-k30` measured the
  collision clause by clause and found it lands on the **verb** a message names
  rather than on its nouns, and grove's operator is an LLM that will try the verb
  it is told to try. It is also the row two messages now share — the library's
  *interrupted promotion* and grove's own diagnosis of the tree a failed rollback
  leaves ([`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#interrupted-promotion)),
  which is exactly the drift a map prevents and a re-wording does not.

- **A durable record has one owner.** Every record under `docs/adr/` and
  `docs/specs/` has a repo-wide unique slug and a maintaining context recorded
  here; a record added later joins this list. Ownership names who keeps the
  record current, not every component it binds: the jj-first VCS rule, for
  example, is shared by the Grove binary and the plugin installer.
  `content/ADR-FORMAT.md` defines when a flat
  root set is appropriate. A term is defined in the glossary of its owning
  context, never both. The **grove** context owns
  [`complete-session-configuration`](docs/adr/complete-session-configuration.md),
  [`untracked-configuration-delta`](docs/adr/untracked-configuration-delta.md),
  [`grove-owns-escalated-review`](docs/adr/grove-owns-escalated-review.md),
  [`skill-delivers-the-methodology`](docs/adr/skill-delivers-the-methodology.md),
  [`one-build-owns-a-session`](docs/adr/one-build-owns-a-session.md),
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
  and the three specs
  [`corpus-rule-ownership`](docs/specs/corpus-rule-ownership.md),
  [`doubt-grove-review-mechanics`](docs/specs/doubt-grove-review-mechanics.md)
  and [`module-decomposition`](docs/specs/module-decomposition.md).
  The **ordinal-fs-tree** context owns
  [`entry-name-is-the-only-seam`](docs/adr/entry-name-is-the-only-seam.md),
  [`entries-are-never-removed`](docs/adr/entries-are-never-removed.md) and
  [`root-lifecycle-belongs-to-the-store`](docs/adr/root-lifecycle-belongs-to-the-store.md)
  — the last of which **moved** here, and the move is the decision it records:
  while root creation and destruction were grove's, so was the record, and it was
  filed under grove for that reason. They sit
  in the flat root set like every other record — `content/ADR-FORMAT.md`'s split
  rule keeps one directory while grove occupies the repo root, and ownership is
  what it says to record instead. grove consumes both decisions and maintains
  neither: their subject is the library's public surface and its key allocation,
  and recording grove as their maintainer would say the extraction had not
  happened. Both are explained at length in
  [`docs/ordinal-fs-tree/ARCHITECTURE.md`](docs/ordinal-fs-tree/ARCHITECTURE.md),
  which is not a duplicate: the document says what the design *is*, for someone
  building against it, and the records say what it cost and what changing it
  would cost, for someone proposing to change it.

  The first two earn their place the same way: one describes how the embedded
  corpus files its rules, an area no single increment finishes; the other a
  composition **between** two contexts. Both outlive the increment that wrote
  them. The doubt skill
  participates in the review-ownership and promotion contracts, but the mandate,
  task tree, review routing, and lifecycle are Grove's maintaining seam.

  A fourth spec, `semantic-contract`, stated the tool-neutral semantics of the
  task tree, the finish protocol and the lifecycle joining them, and was
  **deleted** with the formal-methods apparatus that checked it
  (`delete-formal-models-k29`, whose last directory —
  `crates/grove-finish/models/`, the finish protocol's own column — went at
  `delete-finish-models-k30`). It went for the reason the paragraph below gives
  for `module-decomposition` in advance: a spec earns its place by describing
  something no other artifact holds, and once its checker is gone and the design
  it specified is being dismantled, `docs/ARCHITECTURE.md` is where the
  current-state description belongs.

  The third is the exception the rule below anticipates, and it carries its own
  **retirement condition** rather than a claim to outlive its increment.
  `module-decomposition` describes a target that does not exist yet: it is the
  agreement point a whole chain of leaves builds against, across many sessions
  and three bounded contexts, and while the design is unbuilt no other artifact
  can hold it — `docs/ARCHITECTURE.md` describes what *is*, and an ADR records
  one decision rather than how an area works. Once the crates exist and the
  architecture describes them, the spec describes nothing new and is **deleted**,
  which is what `content/SPEC-FORMAT.md`'s current-state rule says to do with a
  spec that no longer describes anything. Whoever lands the last of its
  decisions deletes it and removes this paragraph with it.

  A spec that only ever described one shipped increment of Grove's own runtime is
  not a durable record; it is `docs/ARCHITECTURE.md`'s subject written twice.
  `config-driven-sessions` and `skill-delivered-methodology` were folded there on
  those grounds, as `mandate-delivered-methodology` went with the machinery it
  described.

## Choosing a context

A topic about the `.grove/` tree, the loop, the CLI verbs, or the binary and its
provisioning is **grove**. A topic about writing, packaging, triggering or
installing a `SKILL.md` is **skills**.

A topic about entries, ordinals, keys, the distinguished child, or the tree
algebra over them is **ordinal-fs-tree**, which is now also where that code
lives: grove's own tree algebra was deleted once the library owned it
(`docs/ARCHITECTURE.md`, *The withdrawn tree algebra*), and what grove keeps is
a domain implementation of the seam plus the lifecycle around the tree. A topic
about `.grove/` *as a task tree* stays **grove** even when it is
about the same directories on disk: the discriminator is which vocabulary the
answer is stated in, not which files it touches.

The word "plugin" in this repository refers to the Claude Code skill plugins
under `plugins/`; route their authoring, packaging, triggering, and installation
to the **skills** context.

A skill's *subject matter* belongs to neither glossary — jj's model lives in
`using-jujutsu` and the architecture's
[VCS seam](docs/ARCHITECTURE.md#symmetric-vcs-rule), Testanyware's VM vocabulary
in `using-testanyware`. If a topic seems to need both glossaries, say so rather
than picking one.

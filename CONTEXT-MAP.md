# Context Map

Three bounded contexts share this repo. **A context is a language boundary, not
a delivery path** — the two are separate questions that the first two contexts
happen to answer together, since each also ships by its own path and they change
in lockstep, which is why they live together (see
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#skills-monorepo)). The third,
`ordinal-fs-tree`, is declared on vocabulary alone: it has a glossary whose terms
mean something else in grove's, and it has no delivery path yet.

## Contexts

- [grove](./CONTEXT.md) — the `grove` CLI, the workstream methodology it embeds
  in `content/`, and the task tree that methodology drives.
- [skills](./plugins/CONTEXT.md) — the `linkuistics` and `testanyware` skill
  plugins: how a skill is authored, packaged, triggered and installed.
- [ordinal-fs-tree](./docs/ordinal-fs-tree/CONTEXT.md) — the domain-independent
  ordered-tree library being extracted from grove's tree modules: entries,
  ordinals, keys, and the algebra over them. Its glossary sits beside
  [its architecture](./docs/ordinal-fs-tree/ARCHITECTURE.md) and moves with the
  crate when the crate exists; **where that crate lands is still open**, and
  answering it changes a path here, not an owner.

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
  (`content/SPEC-FORMAT.md`), and the commit boundary on both lanes
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
  filesystem shapes — so it is held by the two glossaries. Where the words
  collide the meanings differ, and that is deliberate: grove's **Leaf** is a task
  file executed in one session and its **Node directory** is a directory headed
  by a `BRIEF.md` charter, while the library's *leaf* is any regular file and its
  *node* any directory of children. Where the words differ the meanings match:
  grove's **Position** is the library's *ordinal*, grove's **Permanent key** its
  *key*. Neither glossary defines the other's sense of any of the four. A
  document that must speak of both says which tree it means, sentence by
  sentence.

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
  [`supported-workspace-layouts`](docs/adr/supported-workspace-layouts.md),
  [`task-tree-transactions-fail-closed`](docs/adr/task-tree-transactions-fail-closed.md),
  [`corpus-rules-have-one-owner`](docs/adr/corpus-rules-have-one-owner.md),
  [`restatement-declares-its-class`](docs/adr/restatement-declares-its-class.md),
  [`behavioural-coverage-asserts-delivery`](docs/adr/behavioural-coverage-asserts-delivery.md),
  and the two specs
  [`corpus-rule-ownership`](docs/specs/corpus-rule-ownership.md) and
  [`doubt-grove-review-mechanics`](docs/specs/doubt-grove-review-mechanics.md).
  The **ordinal-fs-tree** context owns
  [`entry-name-is-the-only-seam`](docs/adr/entry-name-is-the-only-seam.md) and
  [`entries-are-never-removed`](docs/adr/entries-are-never-removed.md). They sit
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

  Each spec earns its place the same way: the first describes how the embedded
  corpus files its rules, an area no single increment finishes, and the second a
  composition **between** two contexts — so both outlive the increment that wrote
  them. The doubt skill
  participates in the review-ownership and promotion contracts, but the mandate,
  task tree, review routing, and lifecycle are Grove's maintaining seam.

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
algebra over them is **ordinal-fs-tree** — including while that code still lives
inside grove's `src/tree_*` modules, because the vocabulary moved before the code
did. A topic about `.grove/` *as a task tree* stays **grove** even when it is
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

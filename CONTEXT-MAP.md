# Context Map

Two bounded contexts share this repo. They ship by different paths and change in
lockstep, which is why they live together — see
[`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md#skills-monorepo).

## Contexts

- [grove](./CONTEXT.md) — the `grove` CLI, the workstream methodology it embeds
  in `content/`, and the task tree that methodology drives.
- [skills](./plugins/CONTEXT.md) — the `linkuistics` and `testanyware` skill
  plugins: how a skill is authored, packaged, triggered and installed.

## Relationships

- **grove → skills, a documentation-level prerequisite.** grove's methodology
  defers decision-record philosophy to `linkuistics:decision-records` and seam
  judgement to `linkuistics:codebase-design`, keeping only its own placement
  conventions (`content/ADR-FORMAT.md`, `content/SPEC-FORMAT.md`). The dependency
  is now intra-repo but is still **not install-enforced**: the `grove` binary
  provisions its own methodology and nothing else, so the plugins remain a
  separate install.

- **Shared target: the personal skill directory.** Both contexts write into the
  same per-harness namespace. The `grove` binary sweeps `content/` to
  `~/.claude/skills/grove/` (and the Codex and Pi equivalents);
  `plugins/install.sh`
  symlinks each **`linkuistics`** skill into `~/.codex/skills/`,
  `~/.gemini/skills/` and `~/.pi/agent/skills/` (`testanyware` ships by
  marketplace only).
  Nothing collides today — the overlap is `~/.codex/skills/`, where the names are
  disjoint — but the namespace is shared, so any future decision to have one
  context provision the other's content is a question about precedence and
  double-provisioning, not a local change.

- **A durable record has one owner.** If future work creates `docs/adr/` or
  `docs/specs/`, its slug is unique repo-wide and its maintaining context is
  recorded here. Ownership names who keeps the record current, not every
  component it binds: the jj-first VCS rule, for example, is shared by the Grove
  binary and the plugin installer. `content/ADR-FORMAT.md` defines when a flat
  root set is appropriate. A term is defined in the glossary of its owning
  context, never both. The **grove** context owns
  [`grove-owns-escalated-review`](docs/adr/grove-owns-escalated-review.md) and
  [`doubt-grove-review-mechanics`](docs/specs/doubt-grove-review-mechanics.md):
  the doubt skill participates in their contract, but the picked leaf, task
  tree, review routing, and lifecycle are Grove's maintaining seam.

## Choosing a context

A topic about the `.grove/` tree, the loop, the CLI verbs, or the binary and its
provisioning is **grove**. A topic about writing, packaging, triggering or
installing a `SKILL.md` is **skills**.

The word "plugin" spans both, so route it by subject rather than by the word:
`herdr-plugin/` renders the `.grove/` tree and is **grove**, while `plugins/`
holds the Claude Code skill plugins and is **skills**. They also live in
different places on purpose — the root `plugins/` directory belongs to the skills
context, so grove's herdr plugin sits beside it rather than inside it.

A skill's *subject matter* belongs to neither glossary — jj's model lives in
`using-jujutsu` and the architecture's
[VCS seam](docs/ARCHITECTURE.md#symmetric-vcs-rule), Testanyware's VM vocabulary
in `using-testanyware`. If a topic seems to need both glossaries, say so rather
than picking one.

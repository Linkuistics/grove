# Context Map

Two bounded contexts share this repo. They ship by different paths and change in
lockstep, which is why they live together — `docs/adr/skills-monorepo.md`.

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
  `~/.claude/skills/grove/` (and the codex and pi equivalents); `install.sh`
  symlinks each **`linkuistics`** skill into `~/.codex/skills/`,
  `~/.gemini/skills/` and `~/.pi/agent/skills/` (`testanyware` ships by
  marketplace only).
  Nothing collides today — the overlap is `~/.codex/skills/`, where the names are
  disjoint — but the namespace is shared, so any future decision to have one
  context provision the other's content is a question about precedence and
  double-provisioning, not a local change.

- **An ADR is owned by one context, never shared — but the whole set is one flat
  root `docs/adr/`.** `task-tree-scheme` is grove's decision, `symmetric-vcs-rule`
  is the skills'; ownership is recorded *here*, not in the directory layout, and a
  slug is unique **repo-wide**. Ownership is about who **maintains** the record,
  not about whose code it binds: `symmetric-vcs-rule` also governs the `grove`
  binary's jj-first probe, and stays one record rather than two because it is one
  decision with two enforcers. The set stays flat because the two contexts are not
  peers: grove's *is* the repo root, so a per-context split would exile the skills'
  one record while leaving the root set just as mixed
  (`content/ADR-FORMAT.md` states the rule and when flat is right). A term that has
  an ADR is defined in the glossary of whichever context **owns** it, never both.

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
`using-jujutsu` and `docs/adr/symmetric-vcs-rule.md`, testanyware's VM vocabulary
in `using-testanyware`. If a topic seems to need both glossaries, say so rather
than picking one.

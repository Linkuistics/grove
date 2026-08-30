## The four artifacts, and which of them outlive the grove

A grove's output lands in four artifacts, and only one of them is grove-specific.

| Artifact | Path | Role |
|---|---|---|
| Glossary | `CONTEXT.md` (+ `CONTEXT-MAP.md`) | the Ubiquitous Language — the terms this repo's sessions share |
| ADR set | `docs/adr/<slug>.md` | one decision and its trade-off each |
| Spec set | `docs/specs/<slug>.md` | how an area works — the human-facing agreement point |
| Task tree | `.grove/`, inside the grove's working tree | the process: the self-extending decomposition of work |

The first three **outlive the grove** (constraint 6): they are ordinary
team-readable markdown in the repository, and a reader needs neither this skill
nor `.grove/` to use them. The task tree is the **only ephemeral** one — the
finish cycle deletes `.grove/` wholesale, so everything under it is destroyed
with it, `BRIEF.md` charters included. That is what a brief is for: process
context that is meant to die with the process. A finding, a decision, a term or
an agreed design is not, and lands in one of the other three while the grove is
still running.

Briefs and the glossary partition on orthogonal axes, so neither substitutes for
the other: a bounded context is a *domain* partition and the glossary is
per-bounded-context, while a task-tree node is a *process* partition and carries
a `BRIEF.md` rather than a glossary of its own.

Each of the three durable sets has its own format file, and each is where that
artifact's rules live: `CONTEXT-FORMAT.md` for the glossary, `ADR-FORMAT.md` for
the ADR set, `SPEC-FORMAT.md` for the spec set — including when a spec is worth
writing at all, and the membership and grain tests that keep the set honest.

## When the grove's subject is grove itself

This methodology and the `grove` binaries are **separate artifacts with separate
lifetimes**. The skills you are reading are installed files; the binaries are
installed separately. Editing a skill in a checkout reaches a session as soon as
the install route resolves to that checkout — immediately through a symlink,
at the next update through a marketplace — while editing the binary reaches no
session until it is rebuilt *and installed*, because the running loop is the
build already in memory.

Nothing checks that the two agree. Grove used to carry the methodology inside
the binary and report the pairing on every iteration; it carries no corpus now,
so a skew between what a skill instructs and what `grove-llm` exposes is
un-reported and is yours to notice. Verify work on either by reading the files
and the tests, never by expecting the next session in the same loop to behave
differently.

## What the `linkuistics` plugin carries, and why it is separate

Three bodies of guidance grove leans on live in a **separately installed**
plugin. Grove requires it and does not ship it — the `grove` binaries carry no
skills at all, and this methodology and that plugin are installed the same way,
each on its own, through the Claude Code marketplace or the repo's
`plugins/install.sh`, even though both are developed in grove's repository. So
each citation states what binds in the plugin's absence:

- **`linkuistics:decision-records`** — ADR philosophy. What binds without it is
  `ADR-FORMAT.md`: grove states its own when-to-write test, placement and
  minimum-coherent-set discipline locally, and the citation is attribution rather
  than a dependency.
- **`linkuistics:codebase-design`** — what a test seam *is* and how to judge one.
  A seam is a place where behaviour can be replaced for a test without editing
  the code under test; grove's own operative rules (prefer an existing seam,
  propose a new one at the highest point, drive the count toward one) are in
  `SPEC-FORMAT.md` and bind without the skill.
- **`linkuistics:using-jujutsu`** — the working-copy-as-commit lane. What binds
  without it is `references/commit.md`, which carries the whole boundary a grove
  session needs on its own; the skill deepens the lane rather than completing it.

The dependency is documentation-level, not install-enforced: absence changes how
*well* a session writes these artifacts, never *what* it is obliged to write
(constraint 6).

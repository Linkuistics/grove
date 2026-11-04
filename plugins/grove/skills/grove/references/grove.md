<!-- tc: pending because the conversion has not reached this part of the document -->

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
with it, node-file briefs included. That is what a brief is for: process
context that is meant to die with the process. A finding, a decision, a term or
an agreed design is not, and lands in one of the other three while the grove is
still running.

Briefs and the glossary partition on orthogonal axes, so neither substitutes for
the other: a bounded context is a *domain* partition and the glossary is
per-bounded-context, while a task-tree node is a *process* partition and carries
a brief rather than a glossary of its own (`BRIEF-FORMAT.md`).

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

## What the cited craft skills carry, and what binds without them

<!-- tc: on -->

Grove's text cites some craft skills by their bare names. A craft skill teaches
a practice and runs no session kind. Grove does not ship these skills, and an
installation can omit any of them. Each citation therefore states what binds in
the skill's absence:

- **The `decision-records` skill** — ADR philosophy. What binds without it is
  `ADR-FORMAT.md`. Grove states its own when-to-write test, placement and
  minimum-coherent-set discipline there. The citation is attribution rather
  than a dependency.
- **The `codebase-design` skill** — module depth, structural simplicity and
  test seams, for software design and design review. A seam is a place where a
  test can replace behaviour without an edit to the code under test. Grove's
  own operative rules are in `SPEC-FORMAT.md` and bind without the skill. They
  prefer an existing seam, put a new one at the highest point, and keep the
  count near one.
- **The `using-jujutsu` skill** — the working-copy-as-commit lane. What binds
  without it is `references/commit.md`. That file holds the whole boundary that
  a grove session needs. The skill deepens the lane and does not complete it.
- **The `technical-communication` skill** — controlled English for all natural
  language that a session writes. What binds without it is the "Human-facing
  communication" section of `SKILL.md`. That section says what to explain and
  in what order, and it stands on its own. The skill adds rules for words and
  sentences.
- **The `machine-environment` skill** — the personal machine's tool selection,
  configuration ownership and operating-manual locations. What binds without
  it is `references/execute.md`. Use the documented project and machine
  environment, and keep the project's reusable task entry point. The skill
  supplies the personal setup details when that setup is present.

The dependency is in the documentation, and no installer enforces it. Absence
changes how *well* a session writes these artifacts. It never changes *what*
the session must write (constraint 6).

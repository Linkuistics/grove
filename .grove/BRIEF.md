# grove.do-as-much-as-possible-outside-the-llm — brief

## Goal

Move the methodology out of the provisioned skill and into the **mandate**: the
driver composes a 100%-specific `${prompt}` from byte-exact [[Mandate slice]]s of
its own embedded `content/`, so a session is told what it needs instead of
reasoning its way to it.

This follows on from the workstream that moved VCS detection into the driver
([[Stated VCS]]). Same move, larger surface: a fact the driver already holds,
stated before the session exists, so no session derives it — badly, or at all.

Three savings, in the order they matter: **reasoning not performed** (the
session never runs the derivation), **methodology prose deleted** (every fact
stated is a paragraph `SKILL.md` no longer needs, paid back in every session
forever), and **bytes not read**. Turn count alone never justifies a move.

## Done when

- The driver composes each session's mandate from slices selected by
  [[Session kind]], and the mandate is the **sole** delivery path for the
  methodology.
- `content/` carries its own classification — HTML-comment unit markers plus
  per-file KDL frontmatter — and remains whole, readable documents.
- `grove-llm methodology <…>` serves procedural units from the binary's embed.
- Global skill provisioning is **retired**: `src/provision.rs`,
  `tests/provision.rs`, `src/harness.rs`, ADR `one-build-owns-a-session`, and
  the [[Global skill provisioning]] / [[Methodology identity]] glossary entries
  are gone or reduced to the surviving `grove-llm`-off-`PATH` half of
  [[Build pairing]].
- The completeness invariant is a test: every [[Triggering unit]] appears in
  every kind's composed mandate; every procedural unit is reachable through
  `grove-llm methodology`.

## Decomposition

Grilling settled the *what* (`plan-k1`, retired). The dependency order the
design must confirm or overturn:

1. mark and parse `content/` → 2. `grove-llm methodology` → 3. mandate
composition → 4. retire provisioning.

Each stage leaves the product working and delivers behaviour its successor
builds on, so they are candidate **separate groves** rather than merely separate
leaves — a call for the `planning` leaf, not for this brief.

## Pointers

- Glossary terms in play: [[Mandate slice]], [[Triggering unit]] / procedural
  unit, [[Stated VCS]], [[Embedded methodology]], [[Global skill provisioning]],
  [[Build pairing]], [[Methodology identity]], [[Meta-grove]] (see `CONTEXT.md`).
- ADRs a session here must read: `docs/adr/one-build-owns-a-session.md` (does not
  survive intact), `docs/adr/complete-session-configuration.md` (the mandate's
  neighbour — `${prompt}` is expanded from it).
- Architecture: `docs/ARCHITECTURE.md#embedded-methodology` and
  `#the-boundary-is-a-build-not-a-commit`.
- The rationale template for this whole grove is the doc comment on
  `stated_vcs` (`src/loop_driver.rs`): *the fact is the driver's … only the
  session re-derived it, and re-derived it badly.* Four-part test — the driver
  holds it, the session re-derives it, re-derives it **badly**, and the
  instruction was **skippable**.

## On the horizon

- **Does `content/prompts/continue.md` survive?** A mandate that carries the loop
  may not need a launcher. Left to the design session.
- **The `linkuistics` prerequisite is untouched.** ADR philosophy, seam
  judgement and the jj lane live in a separately installed plugin — not
  embedded, so not sliceable. Deliberately out of scope; extending the packaging
  goal to those is a different and larger shape.

## Notes

- This is a [[Meta-grove]]: nothing committed here reaches a session in this
  loop. Retiring provisioning *narrows* that — the methodology will come from the
  installed binary rather than a global directory any build can clobber — but the
  build boundary itself remains.
- **The verification story is the hard part**, and it was designed alongside the
  change rather than after it. The introduced failure is silent and behavioural
  ("the session was never told it could externalize, so it absorbed the work"):
  no error, no exit code. It is converted into a structural one by the
  completeness invariant, with a review chain over the classification as the
  safeguard and golden per-kind mandate snapshots for drift.
  **Behavioural eval was considered and rejected** — expensive,
  non-deterministic, measures a model rather than grove's artifact, and localizes
  nothing when red. The honest behavioural check is the next real grove run after
  the change lands, with a human watching.
- `ARG_MAX` is 1 MiB on macOS, shared with the environment, and `${prompt}` is
  expanded into argv and executed directly (no shell). That is the hard ceiling
  on anything inlined. Current mandate: ~1.1 kB. `SKILL.md` alone is 51 kB.

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

**`increments-k4` narrowed this grove to the first half of that goal.** This
grove makes the embed **addressable**: `content/` classified and gated, and
`grove-llm methodology` serving units out of the binary. A **successor grove**
flips delivery — mandate composition, then the retirement of
[[Global skill provisioning]]. The split and its reasoning are under
*Decomposition*; the successor's charter is under *Successor grove*.

## Done when

- `content/` carries its own classification — unit markers plus the per-file
  ordering carrier `ordering-key-placement-k6` settles on — and remains whole,
  readable documents; nothing a reader or a session sees changes.
- A malformed embed fails `cargo build`, on every syntax, semantic and reference
  class the spec enumerates, through **one** parser implementation shared with
  the crate.
- `grove-llm methodology <…>` serves procedural units from the binary's embed and
  lists them in a grammar an agent can feed back into a fetch.
- The classification is real, reviewed, and auditable from an installed binary.
- The two embed-claims relocate out of `tests/provision.rs` ahead of its deletion,
  and the release-path binary scan asserts the content marker in **both**
  binaries.

Deliberately **not** in this grove's Done-when: mandate composition, the
completeness invariant's mandate claims, golden snapshots, the size alarm, and
the retirement of provisioning. Those are the successor's, and moving them here
would put the transient both-paths state inside this grove's finish.

## Decomposition

Grilling settled the *what* (`plan-k1`, retired); the design settled the *how*
(`mandate-delivery-k2`, repaired by `mandate-delivery-integrate-k5`);
`increments-k4` settled the increments. Four stages, in dependency order:

1. mark and parse `content/` → 2. `grove-llm methodology` → 3. mandate
composition → 4. retire provisioning.

**Stages 1–2 are this grove; stages 3–4 are the successor.** Four separate groves
is the literal reading of the rule and was rejected on two counts. Stage 1
delivers no behaviour on its own — it is scaffolding whose only consumer is stage
2 — so it is not an increment anyone could use. And stage 3 leaves sessions
receiving **both** a mandate and a provisioned skill, which the design accepts
only as a transient and explicitly forbids as a resting state; a grove boundary
there would integrate and possibly release exactly the shape the grilling
rejected.

What the chosen boundary buys, beyond fewer finish cycles: the classification is
released and human-auditable **before** a composer and its golden snapshots are
written over it. That is the risk concentration in the whole workstream, and it
gets its own release and its own pair of eyes first.

This grove's leaves, in order:

- `increments-review-k11` (review-planning) — a fresh context asked to disprove
  this decomposition before four leaves are executed on it. Inserted ahead of
  them, because a plan review that runs after the plan is executed reviews
  nothing.
- `ordering-key-placement-k6` (design) — one bounded question the design left in
  conflict with its own increment order. Blocks the next leaf.
- `unit-grammar-k7` (impl) — the `methodology` seam, marker grammar, parser,
  fence rules, build gate, and a trivially correct marking.
- `methodology-verb-k8` (impl) — `grove-llm methodology`, the two relocations,
  the release-scan inversion.
- `classification-k9` (impl) — the real classification pass, and the `review-impl`
  chain the grilling earmarked. Last, so that chain stays contiguous with it.
- `step-suffix-redundancy-k10` (design) — an **unrelated** concern the human
  raised mid-grove and grove externalized rather than absorbed. Parked last; it
  preempts nothing and belongs to no stage.

## Successor grove

Charter for the grove that follows this one. Its durable design is already
written — `docs/specs/mandate-delivered-methodology.md` and
`docs/adr/mandate-delivers-the-methodology.md` — so its bootstrap grilling scopes
rather than re-decides. What follows is what those records do **not** carry and
this brief must not lose.

**Stage 3 — mandate composition.** `compose`, `content/MANDATE.md` (replacing
`content/prompts/continue.md`, whose "use the grove skill" instruction becomes
false only when provisioning retires), driver wiring, the completeness invariant,
golden per-kind snapshots, and the 64 KiB per-kind size alarm. Provisioning is
still live at the end of this stage, so sessions briefly get both. That is the
safe order — the mandate is proven working before the fallback is removed — and it
must not become the resting state.

**Stage 4 — retire provisioning.** Pure subtraction plus the documentation
reconciliation the design deliberately deferred, on the rule *a record is
reworked when the new decision supersedes its own; a record describing mechanism
that has not yet changed stays accurate until it does.* Enumerated, not swept:

- **Code** — `src/provision.rs`, `src/harness.rs`, `src/lib.rs` (two `pub mod`
  lines), `src/launch.rs` (the provision call and its doc comment),
  `src/loop_driver.rs` (`reverify_installed`, and `mandate_prompt`'s launcher
  lookup). `build.rs` keeps its `rerun-if-changed` emission and its parse gate,
  loses the hash — and with it `GROVE_CONTENT_HASH`, the compile-time
  [[Methodology identity]] constant, and the equality test that existed only to
  keep two traversals in step (both binaries link the embed by then and can hash
  it directly).
- **Tests** — `tests/provision.rs` (after this grove's relocations),
  `tests/harness.rs`. Check `tests/support/mod.rs`'s `HARNESS_NAMES` env scrub:
  that is *environment hygiene for removed variables* and may well survive on its
  own terms.
- **Docs and records** — `docs/ARCHITECTURE.md` (§Embedded methodology, §The
  shared directory, §Repository products, §Main module seams: drop the `harness`
  row, add `methodology`, and remove the pointer stage 0 added),
  `docs/specs/config-driven-sessions.md` (12 sites, including line 60's
  load-bearing *"provisioning precedes ownership so a refused second driver still
  receives…"*), `docs/USAGE.md`, `docs/CONFIGURATION.md` (the harness-root table),
  `docs/RELEASING.md`, `README.md`, `scripts/release-common.sh`,
  `scripts/templates/grove.rb.tmpl`, `CONTEXT-MAP.md`'s *Shared target: the
  personal skill directory* relationship — which stops being a relationship at
  all, since the two contexts no longer share the namespace. Two source citations
  go stale and are scheduled rather than repaired, because the code holding them
  is deleted outright: `build.rs`'s header and `src/provision.rs`'s stamp
  re-verification, both citing `docs/adr/one-build-owns-a-session.md`.
- **Glossary** — delete [[Global skill provisioning]]; reduce
  [[Methodology identity]] and [[Embedded methodology]] to what survives. The
  forward-looking clauses the design session added become plain present tense.
- **Not in scope, do not edit** — everything under `plugins/` (the plugin
  installer's own skill directories, a different context), and
  `src/tree_migrate.rs`'s v1 leaf-name fixtures, which are historical strings
  that happen to contain the word.

## Pointers

- Glossary terms in play: [[Mandate slice]], [[Triggering unit]] / procedural
  unit, [[Methodology unit]], [[Stated VCS]], [[Embedded methodology]],
  [[Global skill provisioning]], [[Build pairing]], [[Methodology identity]],
  [[Meta-grove]] (see `CONTEXT.md`).
- ADRs a session here must read: `docs/adr/mandate-delivers-the-methodology.md`
  (the decision), `docs/adr/one-build-owns-a-session.md` (does not survive
  intact), `docs/adr/complete-session-configuration.md` (the mandate's
  neighbour — `${prompt}` is expanded from it).
- Architecture: `docs/ARCHITECTURE.md#embedded-methodology` and
  `#the-boundary-is-a-build-not-a-commit`.
- The rationale template for this whole grove is the doc comment on
  `stated_vcs` (`src/loop_driver.rs`): *the fact is the driver's … only the
  session re-derived it, and re-derived it badly.* Four-part test — the driver
  holds it, the session re-derives it, re-derives it **badly**, and the
  instruction was **skippable**.

## On the horizon

- **The `linkuistics` prerequisite is untouched.** ADR philosophy, seam
  judgement and the jj lane live in a separately installed plugin — not
  embedded, so not sliceable. Deliberately out of scope; extending the packaging
  goal to those is a different and larger shape.

## Notes

- This is a [[Meta-grove]]: nothing committed here reaches a session in this
  loop. Retiring provisioning *narrows* that — the methodology will come from the
  installed binary rather than a global directory any build can clobber — but the
  build boundary itself remains. It is also why this grove's own deliverable is
  only inspectable after a rebuild and install, and why the successor grove is
  where dogfooding first becomes possible.
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

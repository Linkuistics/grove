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

- `content/` carries its own classification — unit markers and **no ordering
  carrier**, which `ordering-key-placement-k6` settled by deferring the directive
  to the composer that consumes it — and remains whole, readable documents;
  nothing a reader or a session sees changes. `content/SKILL.md` keeps its YAML
  and keeps discovering as a skill, because the parser skips a leading `---`
  block uninterpreted.
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

**The stages are the design's units, and the leaves deliberately cut across
them.** The same rule that rejects stage 1 as a separate *grove* — no usable
behaviour on its own — rejects it as a separate *leaf*, so the first
implementation leaf reaches into stage 2 far enough to serve what it parsed.
Reading the leaf list below as a stage list is the mistake `increments-k4` made.

This grove's leaves, in order. **`increments-integrate-k12` reshaped this list**
after `increments-review-k11` found six problems with it; the reshaping is
recorded per-leaf in the bodies and summarised under *What the review changed*.

- `increments-review-k11` (review-planning) — a fresh context asked to disprove
  this decomposition before four leaves are executed on it. Inserted ahead of
  them, because a plan review that runs after the plan is executed reviews
  nothing. **Retired; it found six.**
- `increments-integrate-k12` (integrate-review-planning) — the repair.
- `ordering-key-placement-k6` (design) — one bounded question the design left in
  conflict with its own increment order. Blocked both implementation leaves, and
  reconciled their contracts before retiring. **Settled:** frontmatter is not the
  carrier and is not a gate requirement — a leading `---` block is opaque preamble
  the parser skips — and the ordering key is an HTML-comment file directive that
  arrives with the composer, in the successor grove. So this grove's gate requires
  **no** ordering key per file and **no** ordering-key uniqueness across the
  embed.
- `addressable-embed-k7` (impl) — the `methodology` seam, marker grammar, parser,
  fence rules, the **per-file** build gate, a trivially correct marking, and
  `grove-llm methodology` over it. Also the identity cutover and the release-scan
  inversion, both of which follow from `grove-llm` linking the embed. Ends at an
  observable listing out of an installed binary.
- `embed-wide-gate-k8` (impl) — the malformation classes that need the whole
  embed: id uniqueness, `defers=` resolution and its class check, procedural
  reachability. No ordering-key case (above). Plus the pinned complete id set and
  the two relocations out of `tests/provision.rs`.
- `embed-wide-gate-review-k16` (review-impl) — cut by `embed-wide-gate-k8` and
  **inserted ahead of the two leaves below**, on the same reasoning as
  `increments-review-k11` and `addressable-embed-review-k14`: both of them
  rewrite the very corpus the gate validates, and one of them is the leaf whose
  safety the gate exists to supply.
- `step-suffix-redundancy-k10` (design) — an **unrelated** concern the human
  raised mid-grove and grove externalized rather than absorbed. It belongs to no
  stage, but it is **not** last: its surface is `content/SKILL.md` and
  `content/TASK-FORMAT.md`, 76 kB of the corpus the classification classifies, so
  it and the `impl` leaf it cuts must both complete first.
- `classification-k9` (node) — the real classification pass, decomposed because
  139 kB of judgement across nine files is not one session. Its first child is the
  `planning` leaf that derives dependency-ordered batches; its charter requires an
  aggregate `review-impl` over the whole classification, cut inside the node after
  the final batch.

### What the review changed

- **The first implementation boundary was horizontal.** This brief rejects stage 1
  as a separate increment because it delivers no usable behaviour, then cut that
  exact stage as `unit-grammar-k7` — inert by construction, nothing reading a
  unit. The boundary now runs *per-file addressability with its reader* against
  *whole-embed validation*, so both implementation leaves land observable
  behaviour.
- **Identity cleanup was assigned to the wrong grove.** The settled spec couples
  it to the moment `grove-llm` links the embed, which is `addressable-embed-k7`;
  the plan left the compile-time constant here and scheduled its removal for the
  successor's stage 4, preserving a duplicate traversal for two groves past its
  justification. It moved back, and out of the successor charter below.
- **`classification-k9` was larger than one session.** It is now a node.
- **The ordering carrier was pre-decided.** Both implementation leaves asserted a
  duplicate-ordering-key build error that `ordering-key-placement-k6` may not
  produce; they were rewritten to derive their carrier obligations from that
  leaf's spec edit, and that leaf has since settled it — no carrier here, so both
  now state the answer outright rather than a condition.
- **"Last, so the chain stays contiguous" was false, twice over.** A review leaf
  is cut lazily by its producer, so while `classification-k9` was a flat leaf its
  `review-impl` would have landed at the parent's next free position, after the
  suffix leaf — contiguity was never available to plan for. Decomposing it settles
  that half by construction: the aggregate review now lives *inside* the node.
  Position order is the only ordering grove offers, so what actually needed
  deciding was which live work runs *before*
  the classification, and the suffix concern does.

## Successor grove

Charter for the grove that follows this one. Its durable design is already
written — `docs/specs/mandate-delivered-methodology.md` and
`docs/adr/mandate-delivers-the-methodology.md` — so its bootstrap grilling scopes
rather than re-decides. What follows is what those records do **not** carry and
this brief must not lose.

**Stage 3 — mandate composition.** `compose`, `content/MANDATE.md` (replacing
`content/prompts/continue.md`, whose "use the grove skill" instruction becomes
false only when provisioning retires), **the file-ordering comment directive and
its uniqueness check** — deferred here by `ordering-key-placement-k6` because
composition is its only consumer — driver wiring, the completeness invariant,
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
  lookup). **Not the methodology identity** — `addressable-embed-k7` performs that
  cutover in *this* grove, at the moment `grove-llm` starts linking the embed,
  which is where the spec couples it. By the time this stage runs, `build.rs`
  already keeps only its `rerun-if-changed` emission and its parse gate, and
  `GROVE_CONTENT_HASH`, the hash traversal and the equality test are long gone.
  Stage 4 deletes provisioning; it performs no delayed identity migration.
- **Content** — `content/SKILL.md`'s YAML preamble becomes free. Delete it or
  leave it; the embed parser skips a leading `---` block either way, so this is a
  tidy-up with no parser consequence and no coordination. That decoupling is the
  point of `ordering-key-placement-k6`'s answer and must not be re-coupled here.
- **Tests** — `tests/provision.rs` (after this grove's relocations),
  `tests/harness.rs`. Check `tests/support/mod.rs`'s `HARNESS_NAMES` env scrub:
  that is *environment hygiene for removed variables* and may well survive on its
  own terms. **`tests/provision.rs` is not pure subtraction**, and
  `embed-wide-gate-k8` found the residue while relocating out of it: three things
  left there are claims about the *embed* or the *release*, not about
  provisioning, and deleting the file drops them silently —
  `both_binaries_carry_the_embedded_methodology`, `CONTENT_MARKER`, and
  `the_release_path_scans_for_the_same_marker`, which is what keeps that const in
  step with `scripts/release-common.sh` (whose diagnostic cites this filename and
  would need repointing). They belong beside the other embed claims in
  `tests/methodology.rs`. Two smaller consequences in the same file set:
  `exactly_one_launcher_is_embedded_and_provisioned` is about a launcher stage 3
  renames, and `tests/methodology.rs`'s
  `the_skill_frontmatter_survives_marking_and_provisioning` calls
  `provision_into`, so it stops compiling the moment that module goes — it is the
  test that decides whether `content/SKILL.md`'s YAML is still load-bearing.
- **Docs and records** — `docs/ARCHITECTURE.md` (§Embedded methodology, §The
  shared directory, §Repository products, §Main module seams: drop the `harness`
  row — `methodology` was added by `addressable-embed-k7`, which is where the
  module landed — and remove the pointer stage 0 added),
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

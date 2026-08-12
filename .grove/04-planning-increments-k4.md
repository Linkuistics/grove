# increments-k4

## Goal

Cut mandate-delivered methodology into the smallest independently useful working
increments, decide grove-vs-leaf for each, and grow the tree for the first one.

The design is settled — read `docs/specs/mandate-delivered-methodology.md` and
`docs/adr/mandate-delivers-the-methodology.md`. Do not re-decide it. If
`mandate-delivery-review-k3` produced findings, an `integrate-review-design` leaf
ran before this one; read what it changed.

## Context

The root brief proposed four stages: mark and parse `content/` → `grove-llm
methodology` → mandate composition → retire provisioning. **The design confirms
that order.** Each leaves the product working and delivers behaviour its
successor builds on. What the design adds is a split inside stage 1 and an
explicit sequencing hazard in stage 3.

### Stage 1 — the unit grammar and its gate

The `methodology` parse seam, the unit type, the build-time gate, and
`content/` marked throughout. Markers are HTML comments, so nothing a reader or
a session sees changes.

Marking and gating **cannot be separated**, because the gate requires every
embedded markdown file to carry frontmatter — that requirement is what makes "an
unmarked file contributes no units" unreachable. But there is a split that works,
and taking it is what stops judgement from fighting mechanism:

1. Grammar, parser, gate, and fixture-driven tests, armed against a **trivially
   correct** marking: one unit per file, `class=procedural`. That is a legal total
   partition and it passes, so the mechanism lands green before any judgement is
   spent.
2. The real classification pass, which is then a pure `content/` edit that only
   **subdivides** existing units. ~120 kB across `SKILL.md`, `TASK-FORMAT.md`,
   `driving.md` and the five format guides.
3. `review-impl` over the classification, and `integrate-review-impl` if it
   finds anything. This is the leaf the grilling earmarked: load-bearing,
   judgement-heavy, wrong in a way no compiler sees.

### Stage 2 — `grove-llm methodology`

`grove-llm` starts linking the embed; the verb, its listing, and its unknown-id
error. Independently useful the moment it lands — a session can fetch a unit
while the skill is still provisioned.

**Two relocations belong here, not in stage 4**, because stage 4 deletes their
home: the scan asserting the embedded methodology instructs no `grove-llm` verb
the embedded CLI lacks, and the flat-verb-surface pin that makes that comparison
mean what it claims. Both are claims about the embed. Their corpus improves in
the move — they will scan the embed rather than a provisioned extraction of it —
and `INSTRUCTED_VERBS` gains `methodology`.

The release-path binary scan inverts here too (`assert_methodology_pairing`
currently *fails* if `grove-llm` carries the content marker). It is a release
check, not a `cargo test` one, so it will not go red during development — schedule
it rather than discover it at the next release cut.

### Stage 3 — mandate composition

`compose`, `content/MANDATE.md`, driver wiring, the completeness invariant,
golden per-kind snapshots, the size alarm.

**Provisioning is still live at the end of this stage**, so sessions briefly get
both a mandate and a skill. That is the shape the grilling rejected as a
permanent answer (pure duplication, raises token cost and contradiction risk),
accepted here as a *transient* — it is the safe order, because the mandate is
proven working before the fallback is removed. Do not let it become the resting
state.

### Stage 4 — retire provisioning

Pure subtraction, plus the documentation reconciliation this design deliberately
deferred. The rule the design session applied: *a record is reworked when the new
decision supersedes its own; a record describing mechanism that has not yet
changed stays accurate until it does.* Stage 4 is when that mechanism changes, so
this is where the rest lands.

Enumerated and classified, not swept:

**Code** — `src/provision.rs`, `src/harness.rs`, `src/lib.rs` (two `pub mod`
lines), `src/launch.rs` (the provision call and its doc comment),
`src/loop_driver.rs` (`reverify_installed`, and `mandate_prompt`'s launcher
lookup). `build.rs` keeps its `rerun-if-changed` emission and its parse gate,
loses the hash.

**Tests** — `tests/provision.rs` (after stage 2's relocations),
`tests/harness.rs`. Check `tests/support/mod.rs`'s `HARNESS_NAMES` env scrub:
that is *environment hygiene for removed variables* and may well survive on its
own terms.

**Docs and records** — `docs/ARCHITECTURE.md` (§Embedded methodology, §The
shared directory, §Repository products, §Main module seams: drop the `harness`
row, add `methodology`, and remove the pointer stage 0 added),
`docs/specs/config-driven-sessions.md` (12 sites, including line 60's
load-bearing *"provisioning precedes ownership so a refused second driver still
receives…"*), `docs/USAGE.md`, `docs/CONFIGURATION.md` (the harness-root table),
`docs/RELEASING.md`, `README.md`, `scripts/release-common.sh`,
`scripts/templates/grove.rb.tmpl`, `CONTEXT-MAP.md`'s *Shared target: the
personal skill directory* relationship — which stops being a relationship at all,
since the two contexts no longer share the namespace.

**Glossary** — delete `Global skill provisioning`; reduce `Methodology identity`
and `Embedded methodology` to what survives. The forward-looking clauses the
design session added to those entries become the plain present tense.

**Not in scope, do not edit** — everything under `plugins/` (the plugin
installer's own skill directories, a different context), and
`src/tree_migrate.rs`'s v1 leaf-name fixtures, which are historical strings that
happen to contain the word.

## Done when

The increments are ordered and each is decided grove-or-leaf, the tree is grown
for the first one, and anything a future increment must not lose is written into
a brief rather than left in this file.

## Notes

- **Groves or leaves is genuinely this leaf's call**, and the design session's
  read is only input: stages 1 and 4 look grove-sized (stage 1 carries the
  classification and its own review chain; stage 4 is a wide, mechanical sweep),
  while 2 and 3 look leaf-sized and could share one. Four separate groves is the
  literal reading of the rule and may be right; weigh it against four finish
  cycles and four working trees.
- This is a [[Meta-grove]]. Nothing planned here reaches a session in this loop
  until the binary is rebuilt and installed, so "verify by running a grove" is
  never available inside the increment that changes the methodology — only after
  it ships. The completeness invariant is the substitute, and the honest
  behavioural check is the first real run afterwards with a human watching.

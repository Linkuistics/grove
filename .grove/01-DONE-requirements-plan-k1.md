# plan-k1

## Goal

Establish what this grove is for by grilling, then grow the tree.

The workstream follows on from the one that moved **VCS detection** out of the
session and into the driver ([[Stated VCS]]). That change is the exemplar: a
fact the driver resolves *before a session exists*, stated in the mandate, so no
session derives it. This grove looks for the rest of that class.

## Context

- Repo: the grove repo itself — a **meta-grove**. What this grove commits reaches
  no session in its own loop (the [[Embedded methodology]] build boundary).
- The mandate today is `mandate_prompt` (`src/loop_driver.rs:195`): launcher
  (`content/prompts/continue.md`, 789 B) + the handle line + `stated_vcs`.
  Nothing else.
- `stated_vcs` (`src/loop_driver.rs:219`) carries the rationale template in its
  doc comment: *the fact is the driver's … only the session re-derived it, and
  re-derived it badly.*

## Done when

Shared understanding is reached and the tree is grown (leaves cut, or a
`planning` leaf added for a fresh session).

## Decisions (running log)

**Scope — what "outside the LLM" means here.** Settled: the follow-on to the
VCS-detection move. The unit of work is a *deterministic fact the driver already
holds*, relocated into the mandate so the session never derives it. Not the
broader readings — pushing verification into non-LLM tooling, or a whole-repo
judgement-vs-mechanism audit — though mechanism that a **verb** could own rather
than the prompt is still in the neighbourhood and Q3 tests it.

**What the saving actually is, and what the mandate is allowed to be.** Settled.
Three savings, not one: bytes not read, methodology prose deleted, and —
the largest — **reasoning not performed**, because a session told a fact
succinctly never runs the derivation that would have established it. Three
binding properties on any stated fact:

1. **Succinct.**
2. **Never contradicts or walks back skill content.** No second source of truth;
   `stated_vcs` already models this by deliberately excluding the
   commit-boundary commands, which the methodology's Commit step owns.
3. **A selector, not a substitute** — it must let the session go straight to the
   *right, precise* skill content instead of reasoning about which rule applies.

Correctness is the filter for admitting a candidate (could a session derive it
wrong, or skip it?); the payoff is prose deleted plus reasoning skipped. Turn
count alone never justifies a move — otherwise 46 kB of glossary gets inlined
into argv to save one `Read`.

**Specificity, and what may be withheld.** Settled: aim for a **100% specific**
mandate — supply what this session requires, not the general case. The safety
rule is **keep the `if`, defer the `then`**:

- **Procedural** content (*how* to do something once you have decided to) may be
  withheld. The session knows it needs it and can read it.
- **Triggering** content (*that a situation exists calling for something other
  than what you are doing*) may **not**. Withholding a condition yields an
  unasked question, not a wrong answer — silent, and it is grove's stated
  primary failure mode (a session quietly absorbing work that should have been
  its own leaf).

The always-load core therefore looks like: the seven constraints, the Decompose
trigger, the review-chain *decision*, the prune/reorder/issue triage, and the
Retire→Commit ordering. Order 10–15% of `SKILL.md` by eyeball — **needs a real
line-by-line classification, not this estimate**, because one misclassified
condition is the silent case.

**Slice, don't paraphrase — and the driver may know `content/` deeply.**
Settled. Prose composed by the driver would make `content/` non-canonical and
break the no-second-source rule; a **byte-exact projection** of the embedded
payload cannot contradict its source. `content/` is already in the binary
(`include_dir!`), and pointing cannot reach the granularity the content has
(kind discipline is one bullet inside `TASK-FORMAT.md` § `The nineteen kinds`,
lines 34–179 — a pointer leaves the session reading all nineteen and doing the
selection anyway, which is the reasoning cost being removed).

The coupling this creates is **accepted deliberately**: the driver may be deeply
aware of `content/` and of the process structure, not merely of the loop. Today
`provision.rs` treats `content/` as an opaque payload; that ends. The build
boundary keeps binary and content in step, and the structural check becomes a
test obligation.

**Delivery path — the mandate replaces the provisioned skill.** Settled on
shape (1), mandate-only:

- The driver **stops sweeping `content/` into global harness skill
  directories**. `continue.md`'s "use the grove skill" goes with it. This is
  where [[Global skill provisioning]], [[Methodology identity]] stamping and the
  methodology half of [[Build pairing]] stop *existing* rather than being
  solved — ~1.5 kloc (`src/provision.rs` 468, `tests/provision.rs` 756,
  `src/harness.rs` 48) plus ADR `one-build-owns-a-session` (220 lines) and three
  glossary entries. The `grove-llm`-off-`PATH` half of pairing survives; it is
  the half a stamp never fixed anyway.
- `content/` **stays as source files and stays embedded** (`include!` /
  `include_dir!`) — readability from the code is the reason, and a templating
  engine is acceptable to get there.
- The deferred `then` is served by a new **`grove-llm methodology <…>`** verb
  reading the binary's own embed. That is strictly better than a file on disk:
  the deferral target is same-build by construction, which a provisioned file
  never was.
- Rejected: (3) additive, which realizes none of the win — if the session still
  loads `SKILL.md` the slice is pure duplication and *raises* both token cost
  and contradiction risk. (2) mandate-primary-with-skill-retained was the
  recommendation and was overruled in favour of going straight to (1), which the
  `methodology` verb makes reachable now.

Note the constraint reading this relies on: constraint 2 ("read, don't run")
binds **bootstrap** — *no script must succeed before work begins*. A mid-session
`grove-llm methodology` call is not bootstrap. The mandate arriving pre-read
strengthens constraint 2 rather than weakening it.

**Verification.** Settled: the behavioural risk is converted to a structural
one, and the residue gets an adversarial read.

- **Completeness invariant** (the mechanism): the triggering/procedural split is
  *data in `content/`*, so a test asserts mechanically that **every triggering
  unit appears in every kind's composed mandate**, and every procedural unit is
  reachable through `grove-llm methodology`. What can go wrong then narrows from
  "did we build the mandate right?" to "did we classify this one unit right?".
- **Review chain on the classification** (the safeguard) — it is load-bearing,
  judgement-heavy, and wrong in a way no compiler sees.
- **Golden per-kind mandate snapshots** — cheap drift detection; says nothing
  about correctness.
- **Rejected: behavioural eval.** Expensive, non-deterministic, measures a
  model rather than grove's artifact, and localizes nothing when red. The
  honest behavioural check is the next real grove run after the change lands,
  with a human watching.

**Where the classification lives, and at what granularity.** Settled:

- **In `content/`, beside the prose it classifies** — adjacency is what keeps it
  true when someone edits the prose.
- **Whole documents, marked in-body** — not a directory of unit files.
  Readability of `content/` from the code is the stated reason for keeping the
  embed, and constraint 7 ("one page of rules") cannot be checked against a
  spine scattered over forty files. Constraint 6 points the same way.
- **Unit markers are HTML comments**, e.g.
  `<!-- unit: retire-cascade kinds=* class=triggering -->` — invisible when
  rendered, no new escaping rules, and the repo already uses HTML comments for
  provenance notes.
- **Per-file KDL frontmatter** carries the file-level manifest. Retiring
  provisioning frees the slot: `content/SKILL.md`'s YAML frontmatter exists only
  for harness discovery, so once nothing provisions it no parser depends on it.
  KDL keeps the repo to one metadata language — `src/session_config.rs` already
  owns a KDL parser for `config.kdl`.

The concrete work this creates: a marker syntax needs a parser, an error story
for malformed input, and a test that parser and prose cannot drift.

## Notes

Measurements taken during grilling (this machine, 2026-08-12):

- `ARG_MAX` = 1 MiB, shared with the environment. `${prompt}` is expanded into
  argv and executed directly (no shell), so that is the hard ceiling on anything
  inlined into the mandate.
- Current mandate: ~1.1 kB.
- Bulk candidates: `CONTEXT.md` 46 kB · `CONTEXT-MAP.md` 3.9 kB · this grove's
  root `BRIEF.md` 121 B · this task file.
- Embedded methodology every session loads: `SKILL.md` 51 kB, `TASK-FORMAT.md`
  25 kB, `driving.md` 42 kB.

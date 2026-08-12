# drop-step-suffix-k19

**Reviews:** `drop-step-suffix-k18`

## Goal

Disprove the step-suffix removal as **executed**. The decision itself is settled
(`step-suffix-redundancy-k10`) and is not reopened here. What is under review is a
wide prose sweep across ~14 files whose durable record is prose rather than an
ADR, landing immediately before `classification-k9` classifies 76 kB of the very
files it rewrote.

Two failure modes, and they need different reading:

1. **A damaged kind label.** `review-<producer>`, `integrate-review-<producer>`,
   `research-a`, `research-b`, `combine-research` are grammar. A slug suffix is
   not. A careless pattern match destroys the first while removing the second.
2. **Prose that is wrong, incomplete, or in the wrong place.** With no ADR, what
   `content/TASK-FORMAT.md` now says *is* the record. If the reason it gives is
   unsound, nothing else in the repo carries the right one.

## Context

- The producer's task file, `drop-step-suffix-k18`, carries the decision verbatim,
  the two claims it answers (A: bare-slug uniqueness; B: surviving commit
  handles), and the compiled surface list. Read it first — several findings below
  are about places that list did *not* name.
- `content/TASK-FORMAT.md` *What the shapes are not* is where the full reasoning
  was written. `content/SKILL.md` carries the short rule, `CONTEXT.md` the
  glossary clause, `docs/ARCHITECTURE.md` §task-kind-taxonomy the architectural
  statement.
- Verification already run and green, so do **not** re-run it (inspection-only):
  `cargo build` (parses `content/` through the embed gate), `cargo fmt --check`,
  `cargo clippy --all-targets` (0), `cargo test` (1021 passed, 0 failed).

## The specific doubts — written by the producer

These are where I would look first, because each is a judgement call the design
session did not enumerate.

### D1 — Did any kind label get damaged?

The mechanical guard is real but indirect: `session_kind_guidance.rs` and
`session_kind_tree.rs` pin the closed set and the label boundary, and they pass.
That proves the *set* is intact; it does not prove every *prose* occurrence still
spells the kind it meant. Grep `content/`, `docs/` and `src/` for the five kind
labels and read each hit against the sentence around it — particularly where a
sentence was rewritten, not just where a token was replaced.

### D2 — Is the recorded reason sound, or merely fluent?

`content/TASK-FORMAT.md` argues the suffix went because it was a **second,
unvalidated source of truth** for the kind. Attack that:

- Is it actually true that nothing validates slug-vs-kind agreement? (The
  producer asserts `leaf-add <parent> foo-review --kind impl` is accepted.)
- Does the "what it costs, exactly" paragraph *understate* the cost? It claims
  the only loss is `resolve <stem>` ambiguity and that no machine path is
  affected. Find a machine path that uses a bare slug, if one exists.
- The producer deleted the stem-mates/sorting argument as **false** (a leaf name
  begins with `NN`, so a listing sorts by position). Verify that, and verify the
  deletion did not take a *true* claim with it.

### D3 — Claim B rests on `git show --stat`. Does it hold?

The prose asserts the role survives in history because Retire-then-Commit puts
the leaf's `DONE` rename in the task's own commit, and teardown removes `.grove/`
from the tip rather than from history. Check the reasoning end to end, including
the no-rename-detection case (delete/add pair, both kind-bearing). If it does not
hold, the prose is defending the change with an argument that fails, which is
worse than not defending it.

### D4 — Sites the surface list missed, and judgement calls beyond it

Each of these was decided by the producer, not by the design session:

- **`CONTEXT.md:698` was changed and then reverted.** `branch-review-k14` is a
  *real* historical handle (cited in `CHANGELOG.md` and two test comments), so
  renaming it would have invented a handle that never resolved. Confirm the
  general rule was applied consistently: conventions taught get rewritten,
  facts recorded do not. Look for the converse error — a historical citation that
  *was* rewritten.
- **`plugins/linkuistics/skills/doubt-driven-development/SKILL.md`** was edited
  (one escalation command). `plugins/` is a different bounded context. Judge
  whether that was in scope, and whether anything else under `plugins/` teaches
  the superseded spelling.
- **`docs/ARCHITECTURE.md` §task-kind-taxonomy gained a paragraph** the surface
  list expected to be "a light touch or none". Judge whether it earns its place
  or duplicates `content/TASK-FORMAT.md`.
- **A `CHANGELOG.md` `## Unreleased` entry was added.** Check it against this
  file's own stated entry rules.

### D5 — The two rewritten comments in `src/tree_grow.rs`

- `validate_slug(stem)`'s justification was *"`foo-` is bad but `foo--a` would
  pass"*. With the stem now the slug verbatim, that hazard is gone and the check
  is over-determined by `add_run`'s per-step validation. The producer kept the
  call (the task file required it) and rewrote the comment to claim a weaker
  reason — that it states the verb's precondition at its own boundary. Is that
  reason honest, or is the call now dead weight wearing a rationalisation?
- The `NAME_MAX` test was **recomputed**, not nudged: stem+22/+34 became
  stem+20/+26, and the pinned stem moved 233 → 235 so the first two names are
  exactly 255 and the third is 261. Re-derive both arithmetic claims
  independently.

### D6 — Fixtures: which should have moved, and which must not

`tests/composition_verbs.rs`'s `an_unmigrated_chain_node_…` test was deliberately
**left** on the old spelling and its comment extended to claim it now guards two
compatibility promises at once. Every other chain and pair fixture was updated.
Judge both halves: is the legacy fixture still testing what it says, and did
updating the others cost any coverage?

## Done when

Findings are recorded with `path:line` citations and a verdict per doubt above,
plus anything else the read turns up. If there are findings worth acting on, cut
`integrate-review-impl` as this session's last act — and **insert it ahead of
`classification-k9`**, for the same reason this leaf was inserted there: the
integration would otherwise edit `content/SKILL.md` and `content/TASK-FORMAT.md`
after the classification had already marked units over them.

## Notes

- **Inspection-only.** Do not run test, build, lint or format commands, and do
  not edit production or test code. The verification above is already recorded;
  the paired integration owns every fix and all post-fix verification.
- This leaf is itself the first artifact cut under the new convention — its slug
  is the producer's bare stem, `drop-step-suffix`, with only the kind and key
  telling the two apart. That is deliberate dogfooding, and it is fair game to
  review: if reading `find .grove` on this tree makes the chain *harder* to
  follow than the suffixed leaves above it do, that is a finding.
- Nothing this grove commits reaches a session in this loop ([[Meta-grove]]), so
  the change is only inspectable in the source, not by running a grove session
  against it.

## Review result

Three findings are worth acting on. No production or test code was edited, and
the producer's recorded build, test, format and lint evidence was inspected
rather than re-run.

### Findings

#### `drop-step-suffix-k19 B1` — Medium: the “exact cost” records the wrong exit status and overlooks a mutation path

**Anchor:** `content/TASK-FORMAT.md:320`–`331`, added by
`drop-step-suffix-k18` as the durable account of what bare-stem ambiguity costs.

The paragraph says a chained `grove-llm resolve <stem>` gives empty stdout and a
**non-zero** exit. The command does the opposite by design:
`src/llm_cli.rs:671`–`679` renders `Resolution::Ambiguous` to stderr and returns
`Ok(())`; `tests/resolve.rs:15`–`18` states the pinned contract explicitly —
empty stdout, a diagnostic on stderr, **exit zero**. A command substitution can
therefore continue successfully with an empty value. The prose currently claims
a safety signal the CLI deliberately does not provide.

“Only one thing” and “no machine path is affected” are too broad for a second
reason. `resolve_ref_or_path_unlocked` accepts a bare slug as a convenience and
turns ambiguity into an error (`src/llm_cli.rs:842`–`864`), and
`leaf_insert_for` sends its mutation target through that resolver
(`src/llm_cli.rs:770`–`779`). A bare stem that previously selected one step by
its suffixed slug can no longer target that step through this accepted CLI path.
The documented Grove workflow already uses a key, handle or path, so this does
not invalidate the decision; it means the cost must be stated accurately rather
than denied.

**Repair shape:** preserve `resolve`'s existing pick-style zero-exit contract and
the settled bare-stem decision. Reconcile the cost statement in
`content/TASK-FORMAT.md`, `docs/ARCHITECTURE.md`,
`docs/specs/doubt-grove-review-mechanics.md` and `CHANGELOG.md`: ambiguity is
reported on stderr with empty stdout and exit zero; recommended handle/key/path
workflow references remain unambiguous, while any convenience path supplied the
shared bare slug — including a `leaf-insert` target — is ambiguous too.

#### `drop-step-suffix-k19 B2` — Low: the plugin behavior has no component-prefixed changelog entry

**Anchor:** `CHANGELOG.md:28`–`33` requires a change to anything the Grove binary
does not carry to be prefixed with the component it touched. The new entry at
`CHANGELOG.md:107`–`130` records only Grove's composed-shape change, while the
same commit also changes the shipped `linkuistics` skill's escalation command at
`plugins/linkuistics/skills/doubt-driven-development/SKILL.md:82`–`85`.

That plugin edit is in scope and necessary — leaving it alone would teach the
superseded command — but it is independently delivered by commit SHA and the
changelog's own rule says to name that component. As written, a reader cannot
tell from `CHANGELOG.md` that `doubt-driven-development` changed at all.

**Repair shape:** add a concise `` `linkuistics` /
`doubt-driven-development`: ...`` entry under `## Unreleased` (or split and
prefix the plugin part equivalently), while retaining the Grove behavior entry.

#### `drop-step-suffix-k19 B3` — Medium: old research-pair filenames lost their compatibility regression guard

**Anchor:** `content/TASK-FORMAT.md:345`–`348` and
`src/tree_grow.rs:247`–`249` promise that **both spellings** remain legal and no
existing tree is invalidated. The retained legacy fixture at
`tests/composition_verbs.rs:469`–`480` covers only a review chain's old
`-review` / `-integrate` slugs. Every research-pair expectation now uses the new
bare stem (`tests/composition_verbs.rs:554`–`565`), and no test fixture retains
the old generated `research-a-<stem>-a`, `research-b-<stem>-b`, or
`combine-research-<stem>-combine` names.

The legacy chain fixture still proves what it says, but it cannot prove the
other half of “both shapes”: the pair's kind labels contain their own hyphens and
the old slugs repeat `a`, `b`, or `combine` at the parser boundary. A future
parser cleanup could reject a tree generated by the previous release while the
new-generator suite and the chain fixture both stayed green.

**Repair shape:** retain the new generator expectations and add one small
compatibility fixture containing the three old pair filenames, exercising a
normal read such as `pick` plus handle resolution. No migration and no generator
fallback are needed; the test should pin only that those existing filenames
remain ordinary well-formed leaves.

### Verdicts by stated doubt

- **D1 — confirmed sound.** Every occurrence of `review-<producer>`,
  `integrate-review-<producer>`, `research-a`, `research-b`, and
  `combine-research` in `content/`, `docs/` and `src/` was read in context. The
  five kind labels and the closed set remain intact; the rewrites changed only
  slug examples and the pair generator.
- **D2 — finding B1.** The central reason is sound: slug-vs-kind agreement is
  unvalidated, bare slug uniqueness is not an invariant, and the deleted sorting
  argument was false because `NN` determines lexical order. The “exact cost”
  paragraph is not sound for the two reasons in B1.
- **D3 — confirmed sound.** `jj show --stat rtpxyoyr` contains the producer
  leaf's kind-bearing `DONE` rename, and the cited torn-down historical commit
  `64f6a3f90db6` still shows its kind-bearing review filename. Without rename
  detection, the delete and add names both carry the kind, so the structural
  argument still holds.
- **D4 — finding B2.** Real historical citations such as
  `branch-review-k14 B5` stayed untouched; rewritten handles in docs and tests
  are synthetic convention examples. Updating the doubt skill is a necessary
  cross-context contract edit, and the architecture paragraph earns its place as
  the taxonomy's concise invariant. Only the changelog treatment is deficient.
- **D5 — confirmed sound.** `validate_slug(stem)` at
  `src/tree_grow.rs:260`–`266` is behaviorally redundant with `add_run`'s
  per-step validation at `src/tree_grow.rs:122`–`125`, and the comment says so
  honestly; retaining a cheap boundary precondition is not a defect. The
  arithmetic is correct for one-digit keys: `research-a` / `research-b` names
  are `stem + 20`, `combine-research` is `stem + 26`, so a 235-byte ASCII stem
  produces 255, 255 and 261-byte names.
- **D6 — finding B3.** The unmigrated chain-node fixture still exercises its
  original brief-less-node contract and now also pins old chain suffixes, but
  updating every old pair filename removed coverage for the pair half of the
  compatibility promise.

The bare-stem dogfood in this tree remains legible: the producer and review are
adjacent, their filename kinds state their roles, and their keys distinguish the
two handles without opening either file.

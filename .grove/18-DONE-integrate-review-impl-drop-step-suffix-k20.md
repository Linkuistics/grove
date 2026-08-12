# drop-step-suffix-k20

**Integrates:** `drop-step-suffix-k19`

## Goal

Apply the three actionable findings from `drop-step-suffix-k19`: correct the
durable account of bare-stem ambiguity, record the `linkuistics` skill change
under the changelog's component rule, and restore a compatibility guard for the
old research-pair filename spelling.

The settled decision does not reopen: shape steps keep the same bare stem, no
migration is added, and both old and new spellings remain legal.

## Context

### Finding 1 — the “exact cost” records the wrong exit status and overlooks a mutation path

**Location:** `content/TASK-FORMAT.md:320`–`331`.

The paragraph says a chained `grove-llm resolve <stem>` gives empty stdout and a
**non-zero** exit. The command does the opposite by design:
`src/llm_cli.rs:671`–`679` renders `Resolution::Ambiguous` to stderr and returns
`Ok(())`; `tests/resolve.rs:15`–`18` pins empty stdout, a stderr diagnostic and
**exit zero**. A command substitution can therefore continue successfully with
an empty value. The prose claims a safety signal the CLI deliberately does not
provide.

“Only one thing” and “no machine path is affected” are too broad too.
`resolve_ref_or_path_unlocked` accepts a bare slug as a convenience and turns
ambiguity into an error (`src/llm_cli.rs:842`–`864`), and `leaf_insert_for`
sends its mutation target through that resolver (`src/llm_cli.rs:770`–`779`). A
bare stem that previously selected one step by its suffixed slug cannot target
that step through this accepted CLI path after all three steps share the stem.
The documented workflow already uses a key, handle or path, so the decision is
sound; the cost statement is not.

**Repair:** preserve `resolve`'s existing pick-style zero-exit contract. Reconcile
`content/TASK-FORMAT.md`, `docs/ARCHITECTURE.md`,
`docs/specs/doubt-grove-review-mechanics.md` and `CHANGELOG.md`: ambiguity is
reported on stderr with empty stdout and exit zero; recommended
handle/key/path references remain unambiguous, while a convenience reference
given the shared bare slug — including a `leaf-insert` target — is ambiguous.

### Finding 2 — the plugin behavior has no component-prefixed changelog entry

**Location:** `CHANGELOG.md:28`–`33` requires a change to anything the Grove
binary does not carry to be prefixed with the component it touched. The entry at
`CHANGELOG.md:107`–`130` records only Grove's behavior, while the same producer
commit changes the shipped skill at
`plugins/linkuistics/skills/doubt-driven-development/SKILL.md:82`–`85`.

The plugin edit is in scope and necessary, but it is independently delivered by
commit SHA and the changelog's rule says to name it.

**Repair:** add a concise `` `linkuistics` /
`doubt-driven-development`: ...`` entry under `## Unreleased` (or split and
prefix the plugin part equivalently), retaining the Grove behavior entry.

### Finding 3 — old research-pair filenames lost their compatibility regression guard

**Location:** `content/TASK-FORMAT.md:345`–`348` and
`src/tree_grow.rs:247`–`249` promise that both spellings remain legal and no
existing tree is invalidated. The retained legacy fixture at
`tests/composition_verbs.rs:469`–`480` covers only old review-chain suffixes;
every pair expectation now uses the bare stem at
`tests/composition_verbs.rs:554`–`565`.

The fixture still proves its brief-less-node and chain compatibility claims, but
it cannot prove the pair half. No test now retains the old generated
`research-a-<stem>-a`, `research-b-<stem>-b`, or
`combine-research-<stem>-combine` names, whose kind/slug boundary differs from
the review chain's.

**Repair:** keep the new generator expectations and add one small compatibility
fixture containing the three old pair filenames, exercising a normal read such
as `pick` plus handle resolution. Add no migration and no generator fallback.

## Done when

- Every durable statement of the ambiguity cost matches the existing CLI
  contract and distinguishes recommended handle/key/path references from the
  accepted bare-slug convenience.
- The `linkuistics` / `doubt-driven-development` change is visible under the
  changelog's component-prefix rule.
- A regression test pins all three old research-pair filenames as ordinary
  well-formed leaves while current generator tests continue to expect the bare
  stem.
- Build, tests, formatting and linting are green after the fixes.

## Notes

`drop-step-suffix-k19` confirmed the five kind labels, the redundancy rationale,
the surviving-commit argument, historical citations, plugin scope, architecture
placement, duplicate boundary validation and `NAME_MAX` arithmetic. Do not
re-open or broaden those settled parts.

# grove.gh-issue-10 — brief

## Goal

Let a project override the personal `~/.config/grove/config.kdl` per session
kind, so the same workstream can send different phases to different harnesses —
the motivating case being deliberate balancing of account usage across vendors
([Linkuistics/grove#10](https://github.com/Linkuistics/grove/issues/10)).

## Done when

- A **configuration delta** file named `.grove.kdl`, found at the worktree root
  or failing that the main repository root, overrides the personal
  configuration **per session kind**, and grove launches the overridden kind's
  command rather than the personal one.
- The personal `~/.config/grove/config.kdl` still declares all nineteen kinds
  exactly once and is still fully validated — the delta relaxes nothing about it.
- An unreadable, unparseable, or invalid delta **fails closed** at both existing
  load points, with the same aggregate (not first-error) diagnostics the
  personal file gets, reported against the delta's own path and location.
- `docs/adr/complete-session-configuration.md` describes the design as it then
  stands, reworked **in place**; no superseding ADR is appended.
- Every current-state claim that this change falsifies is reconciled — including
  `content/references/driver.md`'s "no repository stamp" clause,
  `docs/CONFIGURATION.md`, `docs/ARCHITECTURE.md`, `CONTEXT.md`'s **Grove
  configuration** entry together with its `_Avoid_` lines, and
  `content/references/decompose.md`'s "two entries in
  `~/.config/grove/config.kdl`".
- `docs/CONFIGURATION.md` names the `.gitignore` line a reader must add. Grove
  itself writes no ignore rule and creates or edits no configuration file, as
  today.

## The six settled requirements

Settled with the human in the `plan-k1` grilling and confirmed as a set. These
are decisions, not proposals; a session that wants to depart from one should say
so rather than reinterpret it.

1. **The delta is worktree-local and uncommitted.** Launch policy is personal —
   which harness balances whose account is not a property of the project — so it
   never enters the repository's history and is never shared by a clone.
2. **It is a per-kind delta, not a replacement.** It declares any subset of the
   nineteen; each kind it names wins outright, and every kind it does not name
   falls through to the personal file, which stays mandatorily complete. A newly
   added session kind therefore still fails visibly in every stale personal
   config, and can never silently inherit policy from a delta.
3. **It is named `.grove.kdl` and sits beside `.grove/`, not inside it.** Inside
   would put it in the path of two wholesale-scoped mechanisms: retire commits
   stage `.grove` as a unit, and finish's teardown recursively unlinks every
   entry it finds with no regard for trackedness. Beside, it survives `finish`,
   which is correct — the policy belongs to the checkout, not to one grove.
4. **An invalid delta fails closed**, exactly as the personal file does. The
   alternative — warn and fall back — is silent in the way that matters: the
   session still launches, on the policy the owner was trying to move work away
   from, and the warning arrives too late to act on.
5. **Tested through `SessionConfig::load` only.** No end-to-end driver test and
   no configuration-inspection verb. This boundary is deliberate and was taken
   with its cost stated: nothing asserts end-to-end that a differently-configured
   phase reaches a different program.
6. **Lookup searches the worktree root, then the main repository root; the first
   file found is *the* delta.** The two are never merged with each other, so
   resolution stays exactly **two** deep — the personal file plus at most one
   delta — and never becomes the precedence lattice
   `complete-session-configuration` rejected. A delta at the repository root is
   inherited by every grove workspace of that project, which is what makes this
   per-*project* rather than per-*grove*; one in a grove's own worktree shadows
   it for a one-off. In a single-worktree repository the two roots coincide.

**The invariant that survives all six**, and the one to protect in review: any
one kind's effective command is still a single complete template string, read
whole out of a single file. That — not the count of files — is what
`complete-session-configuration` was defending.

## Decomposition

Named by handle, not position — an insert renumbers positions and rewrites no
file contents, so a position-prefixed name here goes stale silently.

- `config-resolution-k2` (`design`) — the recorded decision. Rework the ADR set
  in place and reconcile the corpus, so the documented design and the design
  agree before code exists to disagree with either.
- `config-resolution-k4` (`review-design`) — the adversarial read of that
  record, cut by `k2` and sequenced ahead of the implementation. Its own body
  carries the specific doubts.
- `local-config-kdl-k3` (`impl`) — the resolution itself, its fail-closed
  diagnostics, its unit tests, and the user-facing reference documentation.

Design precedes implementation because the ADR is what the implementation is
checked against, and because reworking a binding decision after code depends on
it is the expensive order. The review sits between them for the same reason: a
finding against the record is cheap until code depends on it.

## Pointers

- ADRs a session here must read:
  - `docs/adr/complete-session-configuration.md` — the decision this workstream
    changes. Read its *Considered options* especially: layered overrides and
    repository-local policy were both weighed and rejected, and the rework has to
    say honestly what is different now rather than pretend they were not.
  - `docs/adr/supported-workspace-layouts.md` — what `${worktree}` and `${repo}`
    mean, which is what makes requirement 6 well-defined.
  - `docs/adr/task-tree-transactions-fail-closed.md` — the posture requirement 4
    follows.
- Current-state documentation in play: `docs/CONFIGURATION.md` (the whole file is
  about this), `docs/ARCHITECTURE.md` (its session-configuration section and its
  module table), `content/references/driver.md` (§"What the one configuration
  carries"), `content/references/decompose.md` (§"The two shapes are built in
  opposite ways", the *Diversity is the configuration's* paragraph, which names
  the personal file as where the two compared entries live), and `CONTEXT.md`'s
  **Grove configuration** entry — whose "sole source", "one exact lookup" and
  "one home" claims are the glossary's, not the ADR's, and so land with the code.
- Code: `src/session_config.rs` is the entire seam — load, validate, expand.
  `src/loop_driver.rs` holds both load points, pre-mutation and pre-launch.
  `tests/session_config.rs` is the existing unit-test seam.
- Corpus constraints that bite when editing `content/`:
  `docs/specs/corpus-rule-ownership.md` with `tests/rule_ownership.rs`, and the
  per-kind word budgets in `tests/loaded_path_budgets.rs`.
- Glossary work for `CONTEXT.md` **when the behaviour lands, not before**: add
  **configuration delta** — the `.grove.kdl` partial — and rework the existing
  **Grove configuration** entry, which is the larger half. Deliberately withheld
  here; `CONTEXT.md` states the design as it is, and an entry for unbuilt
  behaviour would make the glossary lie.

## Notes

**The provisioned skill does not match the installed binary.** During `plan-k1`,
`~/.claude/skills/grove/.grove-content-hash` read `a4e174da…` while
`grove-llm --content-hash` reported `29273298…`, and the checkout's
`content/TASK-FORMAT.md` is 123 lines against the provisioned copy's 340. So
three corpora are in play and no two agree. Nothing in this workstream depends on
which is right — the six requirements are about the product — but **a session
editing `content/` must re-derive the exact current wording from this checkout**
and must not trust a quotation of the provisioned skill, including the ones in
this brief. Raised with the human as an environment observation; it is not this
grove's work.

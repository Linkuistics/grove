# herdr-notes-reverify-k17

**Kind:** work

## Goal

Re-verify the root brief's herdr findings against the fork's current state,
**once**, so `herdr-turn-hooks-k4`, `herdr-grove-plugin-k5` and
`herdr-pane-misdetection-k11` each start from something true instead of each
re-deriving it.

## Context

The root brief's **Notes** already carry the standing instruction
"re-verify before building on any of them". This leaf exists because a separate
workstream has since moved the fork, and because paying that cost once beats
paying it three times.

Observed at the time of writing (re-check — it will have moved again):

- `ui-layout`, the ship branch, carries a recent
  `chore: merge upstream master and the hook-authority fix into ui-layout`.
- `authority-fix` still holds the two-hunk carry on its own, off
  `upstream/master`, as the fork-maintenance spec requires.
- Local `master` is **~39 commits behind `upstream/master`**, so upstream has
  moved again since that merge.

## Done when

- Each factual claim in the root brief's Notes is marked verified, corrected, or
  deleted against the fork as it stands. The claims that matter most, because
  later leaves are built on them:
  - `claude`/`codex` herdr integrations are session-identity only (their hook
    script drops every state action);
  - `done` is derived (`idle && !seen`), not reported;
  - a report whose agent label parses to a *different known agent* is dropped,
    while an unrecognised label (`grove`) bypasses that gate;
  - `current_session_owner_conflicts` drops a report whose `(source, agent)`
    differs from the pane's session-identity owner;
  - full lifecycle authority is a compiled-in allowlist nothing outside the
    binary can join.
- `docs/specs/herdr-fork-maintenance.md` is reconciled with what the fork
  actually looks like now — it is a **spec**, so it must describe current state,
  and a separate workstream having moved the fork is exactly how such a spec goes
  stale.
- The two-hunk carry is confirmed still two hunks, or the brief and ADR
  *herdr-optional-ui* are corrected. "Every additional hunk is a rebase
  obligation forever" is the reason that count is load-bearing.
- The root brief's Notes are edited **in place** to match.

## Notes

**Line numbers are already known-stale** — the brief says so itself
(`state.rs` took +1281/-812 in an earlier interim). Verify *behaviour*, and
record findings as behavioural contracts rather than line references, so this
leaf does not need doing a fourth time.

**Scope guard.** This is verification and reconciliation only. Anything it
surfaces that wants building goes to the tree via `leaf-add`/`leaf-insert`, not
into this session.

**We do not contribute upstream** (ADR *herdr-optional-ui*) — the fork is a
permanent carry. Do not read "upstream has moved" as a prompt to offer anything;
it is only a prompt to re-verify.

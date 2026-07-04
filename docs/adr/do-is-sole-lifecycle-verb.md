# `grove do` is the sole lifecycle entry verb

A grove is opened with `grove do <name>` from any state. `do` inspects the grove's
state on disk and dispatches: a new grove gets a worktree + branch and a bootstrap
session; a live grove resumes; an orphaned grove (branch present, worktree gone) is
re-attached and resumed. Finishing happens *inside* the session (see the
*in-session-finish-cycle* decision), not through a launched verb. The whole
lifecycle surface is `do` / `migrate` / `retire`.

## Why one verb, not a cluster

An earlier surface exposed separate `start` (new grove), `continue` (resume), and
`finish` (wrap up) verbs. Three of them answered the same question — "open a session
on this grove" — and differed only in which entry state they assumed. Since `do`
reads the state from disk and dispatches correctly, a user running `start` on a live
grove or `continue` on an unknown one was relying on the verb to guess an intent that
`do` simply reads. Collapsing the cluster onto the one verb that already subsumes the
others removes three-spellings-of-one-operation and the standing risk that separate
entry points drift from the dispatch logic. This clears the ADR bar because the
removal is a breaking change a future reader would otherwise question.

## `--start-point` lives on `do`

Branching from somewhere other than the default branch's HEAD is preserved by
`grove do <name> --start-point <ref>`. The flag is meaningful only on the new-grove
path; the resume and re-attach paths ignore it (the branch already exists).

## Why finishing is not a launched verb

The in-session finish cycle triggers only when `grove-llm pick` returns no live
leaves. A launched `finish` would be a second, redundant trigger for the same work —
and it could force-finish a grove that still had live leaves, which was never a
healthy operation. To finish early, retire or clear the remaining leaves first; the
explicit retire-first path is clearer than a force-finish affordance.

## Considered options

- **Keep `start` / `continue` / `finish` as thin aliases of `do`.** Rejected:
  aliases preserve the three-spellings tax and the risk that the state-dispatch
  logic and the alias entry points diverge.
- **Remove `start` / `continue`, keep a launched `grove finish`.** Rejected: with
  finishing in-session, a launched `finish` is a redundant second trigger. One
  trigger — empty `pick` — is simpler than two.

# `grove do` is the sole lifecycle entry verb; `start`, `continue`, and `finish` are removed

The grove CLI exposed four lifecycle verbs that overlapped: `grove start` (new grove → create worktree + launch), `grove continue` (resume an existing grove), `grove do` (state-dispatching: start if unknown, continue if live, re-attach + continue if orphaned), and `grove finish` (wrap up a done grove). `grove do` was already a strict superset of `start` and `continue` — its dispatch covers every entry state — and once finishing a grove moves into the in-session loop (see below), `finish` as a launched verb is redundant too. We remove `start`, `continue`, and `finish` outright and designate `grove do` the **single lifecycle entry verb**. The internal `start()` and `continue_grove()` helpers survive — `do` still dispatches to them, and they still load the `start`/`continue` launcher prompts — only the public `Command` variants are gone.

## Status
accepted

## Why one verb, not four
Three of the four verbs answered the same question — "open a session on this grove" — and only differed in which entry state they assumed. `do` already inspects the state and dispatches correctly, so a user who runs `grove start` on a live grove, or `grove continue` on an unknown one, was relying on the verb to guess their intent when `do` simply reads it from disk. Keeping `start`/`continue` as aliases forces every reader to learn three spellings of one operation and keeps three code paths that can drift. The principle mirrors ADR-0007 (`grove status` is the canonical visibility surface; `grove list`/`grove version` removed): **collapse a cluster of overlapping verbs onto the one that already subsumes the others.** This is recorded as an ADR because the removal is a breaking change and a future reader will otherwise wonder why the convenient `grove start` is gone.

## Considered options
1. **Keep `start`/`continue`/`finish` as thin aliases of `do` (and a launched `finish`).** Rejected: aliases preserve the three-spellings-of-one-thing tax and the standing risk that the state-dispatch logic and the alias entry points diverge. The same reasoning ADR-0007 used to reject a `--names-only` migration flag applies — a convenience surface that re-introduces the thing we are removing.
2. **Remove `start`/`continue`, keep a launched `grove finish`.** Rejected: with the finish flow moving in-session (the running loop proposes the complete finish cycle when `grove-llm pick` returns empty, or `.grove/` is already gone), a launched `finish` verb would be a second, redundant trigger for the same in-session work. One trigger — empty pick — is simpler than two.
3. **Remove all three; `do` is the sole entry verb (chosen).** A grove is opened with `grove do <name>` from any state, and finished from within the session. The CLI surface shrinks to `do`, `takeover`, `retire` for the lifecycle, plus the repo-admin verbs.

## `--start-point` moves to `do`
`grove start` carried `--start-point <ref>` to branch from somewhere other than origin's HEAD; it was the only way to set a non-default branch point. Rather than lose that capability, `grove do` now adopts the existing `StartArgs` (which already holds `name`, `start_point`, `harness`, `no_launch`), so `grove do <name> --start-point <ref>` preserves it. The flag is meaningful only on the new-grove path; the continue/re-attach paths ignore it (the branch already exists).

## Why the finish removal is acceptable
The launched `grove finish` could force a wrap-up even on a grove that still had live leaves. The in-session finish cycle triggers only on an empty pick, so that force-finish affordance is gone: to finish early, retire or clear the remaining leaves first. The trade-off is accepted because force-finishing a grove with unfinished work was never a healthy operation, and the explicit retire-first path is clearer. The step-level design of the in-session finish cycle (exact order, partial-failure resume, interactive vs. headless UX) is settled separately; this ADR records only that `finish` the *verb* is removed in favour of the in-session step.

## Release
This is a breaking CLI change bundled into the next major version bump after v4.0.0. Per repo convention, releases are cut manually via `scripts/release-{doctor,build,publish}.sh` as a post-merge operator step; see `CHANGELOG.md` (Unreleased).

## Note on superseded ADRs
ADR-0002, ADR-0005, and ADR-0006 reference `grove start` / `grove continue` in their bodies (e.g. "the implicit drain at `grove start` / `grove continue`"). Those ADRs are historical records of decisions made while those verbs existed and are **not edited** — per the same immutability stance ADR-0007 took toward ADR-0006. This ADR supersedes the verb-naming slice of each: wherever they say `grove start` / `grove continue` as the session launcher, read `grove do`. The behaviours they describe (drain-on-bootstrap, fetch-before-drain, the LLM-binary split) are unchanged; only the entry verb's name and count changed.

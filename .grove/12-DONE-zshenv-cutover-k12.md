# zshenv-cutover-k12

**Kind:** work

## Goal
Execute plan Task 11: replace the five GROVE_*_MODEL lines in ~/.zshenv with
the trial's scoped scheme + GROVE_REVIEW_HARNESS=pi.

## Context
- Plan Task 11: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- The pi model id comes from this file's Notes (written by pi-kimi-wiring-k11).

## Done when
`zsh -c 'env | grep GROVE_ | sort'` shows exactly the ten new vars, no
fable/sonnet/opus survivors.

## Notes
Recorded model id (pi-kimi-wiring-k11, 2026-07-18): `kimi-coding/k3`.

Not `kimi-code/k3` as originally guessed — the provider name the
pi-provider-kimi-code package registers is `kimi-coding` (matches its
`/login kimi-coding` command). Confirmed against the static catalog
pre-login (`pi --list-models kimi`) and via a live round-trip
(`pi -p --model kimi-coding/k3 "Reply with exactly: kimi-ok"` → `kimi-ok`),
authenticated via `/login kimi-coding` OAuth (not `KIMI_API_KEY` — that env
var is pay-per-token/CI billing per the package README, not the flat-rate
subscription this trial needs). The kimi.com usage dashboard shows both
round-trip calls, confirming subscription billing.

Note: `pi --list-models kimi` run *after* login only lists
`kimi-coding/kimi-for-coding`, omitting `k3` — the package's live
`/v1/models` discovery doesn't surface k3 for this account even though its
own membership-rank check and the kimi.com dashboard both confirm Moderato +
K3 eligibility, and the completions endpoint accepts and serves k3 fine when
requested explicitly by id. Likely a pi-provider-kimi-code discovery
bug/lag, not a real entitlement gap — full detail in the plan file's Task 10
step 4. Also: Moderato caps K3's context window at 256K (expected, per the
package's own membership-limit logic and its README) — comfortably usable,
but worth knowing for the trial.

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
(pi-kimi-wiring-k11 records the exact K3 model id here.)

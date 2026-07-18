# pi-kimi-wiring-k11

**Kind:** work

## Goal
Execute plan Task 10: install pi-provider-kimi-code, configure the Kimi Code
API key, discover the exact K3 model id, prove a live sub-billed round-trip.

## Context
- Plan Task 10: docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md
- Needs the human for the API key (kimi.com) and the dashboard check.

## Done when
`pi -p --model <id> "Reply with exactly: kimi-ok"` returns kimi-ok and the
kimi.com dashboard shows the call; the exact model id is written into
12-zshenv-cutover-k12.md's Notes (working-tree edit) before this leaf retires.
STOP condition: if the sub endpoint rejects pi even with the protocol env, do
not work around it — report and wait for the human (plan Task 10 step 4).

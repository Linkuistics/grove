# workspace-reload-k30


## Goal

Prove composed workspace isolation, exact launched argv and next-session reload
using the selected-profile adapter delivered by workspace-selection-k29.

## Done when

- Temporary jj workspaces and fake executables demonstrate opposite lead/review
  arrangements, repository/worktree precedence, local replacement/default/empty
  selection, and parameter-only experiments without cross-workspace leakage.
- Parameter values containing spaces, quotes, dollar/slot text, empty strings and
  shell punctuation reach the fake child with exact argument boundaries.
- Editing a selection and a shared value while a child runs leaves that child's
  argv/process intact and changes the next session's command. Exercise the real
  driver and existing readiness/recording seams, without real agent sessions.
- Invalid active configuration at the pre-launch reload refuses a launch while
  preserving the applicable tree boundary; inspect existing coverage before
  adding redundant cases. Retain legacy/source-admission regression coverage.
- Update affected documentation and source-exact walkthroughs as necessary and
  run bash scripts/check.sh. Check and close workspace-configuration-k10 against
  its complete brief, carrying any named gap as work before closing it.

## Context

The parent brief owns the complete contract and acceptance ownership. k29 owns
selection, structured diagnostics and focused mutation/launch behavior; this leaf
owns the broader composed observations. Reuse lifecycle_cutover.rs and
loop_driver.rs fixtures. Preserve .cargo/config.toml's signal guard; execute new
builds in acceptance and invoke actual tree verbs directly with grove-llm.

# branch-review-k14

**Kind:** review

## Goal
Fresh-context adversarial read of the whole codex-pi-harness branch diff
against the spec and plan. Findings, not fixes.

## Context
- Artifact: `git diff main...HEAD` in this worktree.
- Contract: the spec (docs/superpowers/specs/2026-07-18-codex-pi-harness-
  switch-design.md) and the plan's Global Constraints section.
- Axes worth separate attention: the symlink-never-followed provisioning
  guard; env precedence and cross-harness leakage; the stamp explicitness
  rule; whether tests assert what they claim (fake-bin argv honesty).
- linkuistics:doubt-driven-development — pass artifact + contract, never the
  claim; classify findings, do not rubber-stamp.

## Done when
Findings recorded in this leaf file and committed; anything needing a fix
becomes a new leaf via `grove-llm leaf-insert 15 <slug>` (sequenced before
release), never silently patched in-session.

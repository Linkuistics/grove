#!/usr/bin/env bash
# Headless Codex for a standalone `grove run`, staged beside SKILL.md.
# The owner's Ravel policy selects it and keeps the model and
# effort. Its `select` returns, for the `release-notes` kind:
#   program "/bin/bash", args ["codex-headless.sh", model, effort, request.prompt]
# It runs with the private invocation directory as its working directory.
# Grove confines nothing, so Codex's own sandbox is the only boundary
# (docs/specs/standalone-invocations.md#the-harness-sandbox).
set -euo pipefail
IFS=$'\n\t'

if (($# != 3)); then
  echo "usage: codex-headless.sh MODEL EFFORT PROMPT" >&2
  exit 2
fi

# Codex uses the owner's Codex home, so a token refresh changes the owner's
# own sign-in. `codex login status` exits nonzero without a sign-in, whichever
# store holds it, and it calls no model.
if ! codex login status >/dev/null 2>&1; then
  cat >&2 <<'MESSAGE'
codex-headless: Codex has no sign-in for this account.
Run `codex login`, then run the release-notes task again.
MESSAGE
  exit 1
fi
# `--ignore-user-config` skips config.toml, and the sign-in still comes from
# CODEX_HOME (`codex exec --help`, codex-cli 0.166.0). `workspace-write`
# permits writes in the cwd and TMPDIR, where Ravel's exit channel lies.
exec codex exec --ephemeral --ignore-user-config --ignore-rules --skip-git-repo-check \
  --sandbox workspace-write --model "$1" -c "model_reasoning_effort=$2" "$3"

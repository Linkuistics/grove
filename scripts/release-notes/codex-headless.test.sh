#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

helper="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/codex-headless.sh"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/bin" "$scratch/work"
export RELEASE_CODEX_TEST_CALLS="$scratch/calls"
cat >"$scratch/bin/codex" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
printf '%s\n' "$@" >"$RELEASE_CODEX_TEST_CALLS"
[[ "$CODEX_HOME" == "$PWD/.codex-home" && -r "$CODEX_HOME/auth.json" ]]
EOF
chmod +x "$scratch/bin/codex"
export PATH="$scratch/bin:$PATH"
cd "$scratch/work"

# Missing credentials must give the task-level remedy before creating state
# or launching the (stubbed) paid model boundary.
export CODEX_AUTH_FILE="$scratch/credentials with spaces.json"
if bash "$helper" fixture-model medium 'fixture prompt' >"$scratch/output" 2>&1; then
  echo 'FAIL: the Codex helper accepted missing credentials' >&2
  exit 1
fi
grep -Fq "$CODEX_AUTH_FILE" "$scratch/output"
grep -Fq 'GROVE_RELEASE_RUNTIME_READ' "$scratch/output"
grep -Fq -- '--runtime-read' "$scratch/output"
grep -Fq 'docs/RELEASING.md#runtime-access' "$scratch/output"
[[ ! -e "$RELEASE_CODEX_TEST_CALLS" && ! -e .codex-home ]]

# A directory cannot be used as a credential file, even though it is readable.
mkdir "$CODEX_AUTH_FILE"
if bash "$helper" fixture-model medium 'fixture prompt' >"$scratch/output" 2>&1; then
  echo 'FAIL: the Codex helper accepted a directory as credentials' >&2
  exit 1
fi
grep -Fq 'GROVE_RELEASE_RUNTIME_READ' "$scratch/output"
[[ ! -e "$RELEASE_CODEX_TEST_CALLS" && ! -e .codex-home ]]
rmdir "$CODEX_AUTH_FILE"

# Successful startup still copies credentials privately and passes the selected
# model, effort and prompt through to Codex without printing credentials.
printf '{"fixture":"private-test-token"}\n' >"$CODEX_AUTH_FILE"
bash "$helper" fixture-model medium 'fixture prompt' >"$scratch/output" 2>&1
cmp "$CODEX_AUTH_FILE" .codex-home/auth.json
printf '%s\n' exec --ephemeral --ignore-user-config --ignore-rules --skip-git-repo-check \
  --sandbox danger-full-access --model fixture-model -c model_reasoning_effort=medium \
  'fixture prompt' >"$scratch/expected"
cmp "$scratch/expected" "$RELEASE_CODEX_TEST_CALLS"
if grep -Fq 'private-test-token' "$scratch/output"; then
  echo 'FAIL: the Codex helper printed credentials' >&2
  exit 1
fi
echo 'release Codex helper tests: all passed'

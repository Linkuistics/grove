#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

helper="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/codex-headless.sh"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/bin" "$scratch/work"
export RELEASE_CODEX_TEST_CALLS="$scratch/calls"
export RELEASE_CODEX_TEST_STATUS="$scratch/status"
export RELEASE_CODEX_TEST_CERT="${SSL_CERT_FILE-}"
# The fake Codex answers `login status` from a fixture file and records the
# arguments of `exec`. It fails if the helper replaced the owner's Codex home
# or changed the caller's certificate bundle.
cat >"$scratch/bin/codex" <<'FAKE'
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
[[ "${CODEX_HOME-}" == "${RELEASE_CODEX_TEST_HOME-}" && "${SSL_CERT_FILE-}" == "$RELEASE_CODEX_TEST_CERT" ]]
if [[ "$1" == login && "$2" == status ]]; then
  [[ "$(cat "$RELEASE_CODEX_TEST_STATUS")" == signed-in ]]
  exit
fi
printf '%s\n' "$@" >"$RELEASE_CODEX_TEST_CALLS"
FAKE
chmod +x "$scratch/bin/codex"
export PATH="$scratch/bin:$PATH"
cd "$scratch/work"

# The owner's Codex home passes through unchanged, set or unset.
for home in "" "$scratch/owner codex home"; do
  export RELEASE_CODEX_TEST_HOME="$home"
  if [[ -n "$home" ]]; then export CODEX_HOME="$home"; else unset CODEX_HOME; fi
  rm -f "$RELEASE_CODEX_TEST_CALLS"

  # A missing sign-in gives the remedy before Codex runs the paid model.
  printf signed-out >"$RELEASE_CODEX_TEST_STATUS"
  if bash "$helper" fixture-model medium 'fixture prompt' >"$scratch/output" 2>&1; then
    echo 'FAIL: the Codex helper accepted a missing sign-in' >&2
    exit 1
  fi
  grep -Fq 'codex login' "$scratch/output"
  if grep -Eq 'runtime-read|RUNTIME_READ|grant' "$scratch/output"; then
    echo 'FAIL: the sign-in remedy names a grant' >&2
    exit 1
  fi
  [[ ! -e "$RELEASE_CODEX_TEST_CALLS" ]]

  # With a sign-in, the helper runs Codex under its own workspace-write
  # sandbox, with the selected model, effort and prompt. It stages no state.
  printf signed-in >"$RELEASE_CODEX_TEST_STATUS"
  bash "$helper" fixture-model medium 'fixture prompt' >"$scratch/output" 2>&1
  printf '%s\n' exec --ephemeral --ignore-user-config --ignore-rules --skip-git-repo-check \
    --sandbox workspace-write --model fixture-model -c model_reasoning_effort=medium \
    'fixture prompt' >"$scratch/expected"
  cmp "$scratch/expected" "$RELEASE_CODEX_TEST_CALLS"
  [[ -z "$(ls -A .)" ]]
done
echo 'release Codex helper tests: all passed'

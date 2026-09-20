#!/usr/bin/env bash
# Headless Codex for a confined `grove run`, staged beside SKILL.md.
# Personal policy selects it and keeps the model and effort, for example:
#   command "release-notes-codex" "/bin/bash codex-headless.sh ${param.model} ${param.effort} ${prompt}"
# It runs with the private invocation directory as its working directory and
# keeps every writable Codex file beneath it. The credential file and the Codex
# runtime files need --runtime-read grants (GROVE_RELEASE_RUNTIME_READ).
set -euo pipefail
IFS=$'\n\t'

if (($# != 3)); then
  echo "usage: codex-headless.sh MODEL EFFORT PROMPT" >&2
  exit 2
fi

auth="${CODEX_AUTH_FILE:-$HOME/.codex/auth.json}"
[[ -f "$auth" && -r "$auth" ]] || {
  cat >&2 <<EOF
codex-headless: credential file is not readable inside the standalone sandbox:
  $auth
Check that this file exists and is readable on the host. If it is, add its path
to GROVE_RELEASE_RUNTIME_READ for task release:notes (one literal file path per
line, keeping your other grants), or pass --runtime-read for direct grove run.
Codex's installed runtime files may also need grants. Setup and retry instructions:
  docs/RELEASING.md#runtime-access
EOF
  exit 1
}
codex_home="$PWD/.codex-home"
mkdir -p "$codex_home"
install -m 600 "$auth" "$codex_home/auth.json"
# The public CA bundle replaces keychain access, which the sandbox denies.
export CODEX_HOME="$codex_home" SSL_CERT_FILE=/etc/ssl/cert.pem
# Codex's nested sandbox rejects tool execution inside Grove's; the outer
# confinement remains the filesystem boundary (docs/CONFIGURATION.md).
exec codex exec --ephemeral --ignore-user-config --ignore-rules --skip-git-repo-check \
  --sandbox danger-full-access --model "$1" -c "model_reasoning_effort=$2" "$3"

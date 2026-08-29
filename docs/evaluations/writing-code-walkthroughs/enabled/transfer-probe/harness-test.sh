#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
readonly here
readonly harness="$here/harness.sh"
test_root=$(mktemp -d /private/tmp/grove-transfer-harness-test.XXXXXX)
readonly test_root
trap 'rm -rf "$test_root"' EXIT

printf '{}\n' >"$test_root/auth.json"

if [[ ! -x "$harness" ]]; then
  printf 'FAIL: transfer paired harness is not executable\n' >&2
  exit 1
fi

"$harness" init "$test_root/campaign" "$test_root/auth.json"

[[ $(<"$test_root/campaign/schedule.txt") == 'control enabled enabled control enabled control control enabled control enabled' ]]
[[ -f "$test_root/campaign/control-template/auth.json" ]]
[[ -f "$test_root/campaign/enabled-template/auth.json" ]]
[[ -f "$test_root/campaign/enabled-template/skills/writing-code-walkthroughs/SKILL.md" ]]
[[ ! -e "$test_root/campaign/control-template/skills" ]]
[[ $(wc -l <"$test_root/campaign/control-template.manifest.tsv" | tr -d ' ') == 1 ]]
[[ $(wc -l <"$test_root/campaign/enabled-template.manifest.tsv" | tr -d ' ') == 2 ]]

printf 'PASS: transfer harness initializes sealed templates and paired schedule\n'

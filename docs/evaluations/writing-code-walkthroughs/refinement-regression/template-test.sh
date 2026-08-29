#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

here=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd -P)
readonly here
repo_root=$(cd "$here/../../../.." && pwd -P)
readonly repo_root
readonly skill="$repo_root/plugins/linkuistics/skills/writing-code-walkthroughs/SKILL.md"
readonly plugin_manifest="$repo_root/plugins/linkuistics/.claude-plugin/plugin.json"

test_root=$(mktemp -d /private/tmp/grove-refined-template-test.XXXXXX)
readonly test_root
trap 'rm -rf "$test_root"' EXIT

expected_record=$(<"$here/skill.manifest.tsv")
readonly expected_record
expected_path=${expected_record%%$'\t'*}
record_tail=${expected_record#*$'\t'}
expected_bytes=${record_tail%%$'\t'*}
expected_sha=${record_tail##*$'\t'}

[[ "$expected_path" == 'skills/writing-code-walkthroughs/SKILL.md' ]]
[[ $(wc -c <"$skill" | tr -d ' ') == "$expected_bytes" ]]
[[ $(shasum -a 256 "$skill" | awk '{print $1}') == "$expected_sha" ]]
ruby -e '
  require "yaml"
  source = File.read(ARGV.fetch(0))
  frontmatter = source.match(/\A---\n(.*?)\n---\n/m) or abort "missing frontmatter"
  fields = YAML.safe_load(frontmatter[1])
  abort "wrong name" unless fields["name"] == "writing-code-walkthroughs"
  abort "missing description" unless fields["description"].is_a?(String) && !fields["description"].empty?
  abort "wrong harnesses" unless fields["harnesses"] == ["any"]
  abort "frontmatter too large" unless frontmatter[0].bytesize <= 1024
' "$skill"
jq -e '.name == "linkuistics"' "$plugin_manifest" >/dev/null

mkdir -p "$test_root/control-template" \
  "$test_root/enabled-template/skills/writing-code-walkthroughs"
printf '{}\n' >"$test_root/control-template/auth.json"
cp "$test_root/control-template/auth.json" "$test_root/enabled-template/auth.json"
cp "$skill" "$test_root/enabled-template/skills/writing-code-walkthroughs/SKILL.md"
rm -r "$test_root/enabled-template/skills"
cmp "$test_root/control-template/auth.json" "$test_root/enabled-template/auth.json"

printf 'PASS: final skill structure, bytes, plugin manifest, and sealed-template delta\n'

#!/usr/bin/env bash
# Controls for check-packs.sh. Each case writes a fixture pack record that pins
# a fixture pack in a local Git repository, and runs the real script with a
# scratch cache. No case reads the network or the real plugins/packs.json.
# Run: bash scripts/check-packs.test.sh

set -euo pipefail
IFS=$'\n\t'

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
script="${here}/check-packs.sh"
pass=0
fail=0

scratch="$(mktemp -d)"
trap 'rm -rf "${scratch}"' EXIT

export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1
export GIT_AUTHOR_NAME="grove test" GIT_AUTHOR_EMAIL="test@example.invalid"
export GIT_COMMITTER_NAME="grove test" GIT_COMMITTER_EMAIL="test@example.invalid"

# make_pack <repo> <skill>: a Git repository whose plugins/fixpack ships one
# skill that is not a kind and a manifest with no rows. Prints the commit.
make_pack() {
  local repo="$1" skill="$2"
  mkdir -p "${repo}/plugins/fixpack/skills/${skill}" "${repo}/plugins/fixpack/conformance"
  printf -- '---\nname: %s\ndescription: fixture\nharnesses: [any]\n---\n\nbody\n' "${skill}" \
    >"${repo}/plugins/fixpack/skills/${skill}/SKILL.md"
  printf '# rule\towner\tclass\tload\tphrase\n' >"${repo}/plugins/fixpack/conformance/rules.tsv"
  git init --quiet "${repo}"
  git -C "${repo}" add -A
  git -C "${repo}" commit --quiet -m fixture
  git -C "${repo}" rev-parse HEAD
}

# write_record <file> <url> <sha|-> [<key>] [<path>]: a record of one member
# keyed fixpack, without sha for -. The key and the path can be replaced.
write_record() {
  local sha_field=""
  [[ "$3" == - ]] || sha_field=", \"sha\": \"$3\""
  cat >"$1" <<JSON
{"${4:-fixpack}": {"url": "$2", "path": "${5:-plugins/fixpack}"${sha_field}}}
JSON
}

# check <name> <want-status> <want-regex> <record> <cache>
check() {
  local name="$1" want_status="$2" want_regex="$3" status=0 output problem=""
  output="$(bash "${script}" --record "$4" --cache "$5" 2>&1)" || status=$?
  [[ "${status}" == "${want_status}" ]] || problem+=" exit ${status} (want ${want_status});"
  [[ "${output}" =~ ${want_regex} ]] || problem+=" output does not match /${want_regex}/;"
  if [[ -z "${problem}" ]]; then
    pass=$((pass + 1))
  else
    fail=$((fail + 1))
    printf 'FAIL: %s —%s\n      output: %s\n' "${name}" "${problem}" "${output}" >&2
  fi
}

good="${scratch}/good"
good_sha="$(make_pack "${good}" fixture-note)"
record="${scratch}/packs.json"
cache="${scratch}/cache"

write_record "${record}" "file://${good}" "${good_sha}"
check "a recorded pack is checked out and passes" 0 'checked out .*1 recorded pack\(s\) pass' "${record}" "${cache}"
check "a clean cached checkout is reused" 0 'reusing the clean checkout' "${record}" "${cache}"

printf 'stray\n' >"${cache}/fixpack/${good_sha}/stray.txt"
check "a dirty cached checkout is replaced" 0 'checked out ' "${record}" "${cache}"

write_record "${record}" "file://${good}" -
check "a member without sha fails" 1 'pins no full commit in sha \(absent\).*1 of 1 recorded' "${record}" "${cache}"

write_record "${record}" "file://${good}" 0123456789abcdef0123456789abcdef01234567
check "a sha that the repository lacks fails" 1 'cannot check out' "${record}" "${cache}"

# A pack that ships a skill of core's name passes alone and fails the aggregate.
clash="${scratch}/clash"
clash_sha="$(make_pack "${clash}" grove)"
write_record "${record}" "file://${clash}" "${clash_sha}"
check "a pack that fails the aggregate fails" 1 'duplicate skill: .grove..*fixpack \(aggregate\)' \
  "${record}" "${cache}"

# A pack whose manifest names an owner that it does not ship fails alone.
orphan="${scratch}/orphan"
mkdir -p "${orphan}/plugins/fixpack/conformance"
make_pack "${orphan}" fixture-note >/dev/null
printf 'fixture-orphan\tfixture-note/absent.md\tS\tstatic(all)\t\n' \
  >>"${orphan}/plugins/fixpack/conformance/rules.tsv"
git -C "${orphan}" commit --quiet -am orphan
write_record "${record}" "file://${orphan}" "$(git -C "${orphan}" rev-parse HEAD)"
check "a pack that fails alone fails" 1 'unshipped owner.*fixpack \(alone\)' "${record}" "${cache}"

# make_source <repo> <file>...: a Git repository that holds each named file
# under plugins/fixpack, with a fixture SKILL.md. Prints the commit.
make_source() {
  local repo="$1" file
  shift
  for file in "$@"; do
    mkdir -p "${repo}/plugins/fixpack/$(dirname "${file}")"
    printf -- '---\nname: %s\ndescription: fixture\n---\n\nbody\n' "$(basename "$(dirname "${file}")")" \
      >"${repo}/plugins/fixpack/${file}"
  done
  git init --quiet "${repo}"
  git -C "${repo}" add -A
  git -C "${repo}" commit --quiet -m fixture
  git -C "${repo}" rev-parse HEAD
}

# A source with craft skills only, as TestAnyware ships them under skills/, is
# not a member. The directory-only check left check:packs, so the runner
# refuses it.
craft="${scratch}/craft"
write_record "${record}" "file://${craft}" "$(make_source "${craft}" skills/using-fixture/SKILL.md)"
check "a source with no kind skill and no manifest fails" 1 \
  'has no conformance/rules.tsv.*fixpack \(alone\)' "${record}" "${cache}"

# A kind skill without a manifest is a pack that breaks the contract, so it goes
# to the runner, which refuses it.
kind="${scratch}/kind"
write_record "${record}" "file://${kind}" "$(make_source "${kind}" skills/grove-fixkind/SKILL.md)"
check "a source with a kind skill and no manifest fails" 1 \
  'has no conformance/rules.tsv.*fixpack \(alone\)' "${record}" "${cache}"

# A commit that lacks the member's path fails.
pathless="${scratch}/pathless"
mkdir -p "${pathless}/elsewhere"
printf 'x\n' >"${pathless}/elsewhere/file"
git init --quiet "${pathless}"
git -C "${pathless}" add -A
git -C "${pathless}" commit --quiet -m pathless
write_record "${record}" "file://${pathless}" "$(git -C "${pathless}" rev-parse HEAD)"
check "a commit without the source directory fails" 1 'holds no source directory plugins/fixpack' \
  "${record}" "${cache}"

# The record's shape. Each case names the member it refuses, and none reaches
# the network or the runner.
write_record "${record}" "file://${good}" "${good_sha}" FixPack
check "a key outside the source-ID grammar fails" 1 \
  'pack "FixPack".*not a source ID.*1 of 1 recorded' "${record}" "${cache}"
write_record "${record}" "file://${good}" "${good_sha}" fix--pack
check "a key with a doubled hyphen fails" 1 'not a source ID' "${record}" "${cache}"
write_record "${record}" "file://${good}" "${good_sha}" 1pack
check "a key with a leading digit fails" 1 'not a source ID' "${record}" "${cache}"
write_record "${record}" "file://${good}" "${good_sha}" fixpack ../plugins/fixpack
check "a path that leaves the checkout fails" 1 'not a normalised relative path' "${record}" "${cache}"
write_record "${record}" "file://${good}" "${good_sha:0:12}"
check "an abbreviated sha fails" 1 'pins no full commit in sha \([0-9a-f]{12}\)' "${record}" "${cache}"

printf '{"fixpack": {"url": "file://%s", "path": "plugins/fixpack", "sha": "%s", "ref": "main"}}\n' \
  "${good}" "${good_sha}" >"${record}"
check "a member with a field outside url, path and sha fails" 1 'field outside url, path and sha: ref' \
  "${record}" "${cache}"

printf '{"fixpack": {"url": "file://%s", "path": "plugins/fixpack", "sha": "%s"}, "fixpack": {}}\n' \
  "${good}" "${good_sha}" >"${record}"
check "a repeated key fails" 1 'is ambiguous: a key repeats: "fixpack".*cannot be read' "${record}" "${cache}"

# One refused member does not stop the check of the others.
printf '{"broken": {"url": "file://%s", "path": "plugins/fixpack"}, "fixpack": {"url": "file://%s", "path": "plugins/fixpack", "sha": "%s"}}\n' \
  "${good}" "${good}" "${good_sha}" >"${record}"
check "a refused member leaves the others checked" 1 \
  'pack broken.*pack fixpack.*reusing the clean checkout.*1 of 2 recorded.*✗ broken' "${record}" "${cache}"

printf '{}\n' >"${record}"
check "an empty record fails" 1 'names no pack' "${record}" "${cache}"
printf '[]\n' >"${record}"
check "a record that is not an object fails" 1 'not a JSON object keyed by source ID' "${record}" "${cache}"
printf '{"fixpack": \n' >"${record}"
check "a record that is not JSON fails" 1 'is not JSON' "${record}" "${cache}"
check "a missing record fails" 1 'cannot read the pack record' "${scratch}/absent.json" "${cache}"

printf '\n%d passed, %d failed\n' "${pass}" "${fail}"
[[ "${fail}" -eq 0 ]]

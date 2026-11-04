#!/usr/bin/env bash
# Controls for check-packs.sh. Each case writes a fixture marketplace that pins
# a fixture pack in a local Git repository, and runs the real script with a
# scratch cache. No case reads the network or the real marketplace.
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

# write_marketplace <file> <url> <sha|->: one git-subdir entry, without sha for -.
write_marketplace() {
  local sha_field=""
  [[ "$3" == - ]] || sha_field=", \"sha\": \"$3\""
  cat >"$1" <<JSON
{"name": "fixture", "plugins": [
  {"name": "local", "source": "./plugins/grove"},
  {"name": "fixpack", "source": {"source": "git-subdir", "url": "$2", "path": "plugins/fixpack"${sha_field}}}
]}
JSON
}

# check <name> <want-status> <want-regex> <marketplace> <cache>
check() {
  local name="$1" want_status="$2" want_regex="$3" status=0 output problem=""
  output="$(bash "${script}" --marketplace "$4" --cache "$5" 2>&1)" || status=$?
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
market="${scratch}/market.json"
cache="${scratch}/cache"

write_marketplace "${market}" "file://${good}" "${good_sha}"
check "a pinned pack is checked out and passes" 0 'checked out .*1 pinned pack\(s\) pass' "${market}" "${cache}"
check "a clean cached checkout is reused" 0 'reusing the clean checkout' "${market}" "${cache}"

printf 'stray\n' >"${cache}/fixpack/${good_sha}/stray.txt"
check "a dirty cached checkout is replaced" 0 'checked out ' "${market}" "${cache}"

write_marketplace "${market}" "file://${good}" -
check "an entry without sha fails" 1 'pins no full commit in sha \(absent\)' "${market}" "${cache}"

write_marketplace "${market}" "file://${good}" 0123456789abcdef0123456789abcdef01234567
check "a sha that the repository lacks fails" 1 'cannot check out' "${market}" "${cache}"

# A pack that ships a skill of core's name passes alone and fails the aggregate.
clash="${scratch}/clash"
clash_sha="$(make_pack "${clash}" grove)"
write_marketplace "${market}" "file://${clash}" "${clash_sha}"
check "a pack that fails the aggregate fails" 1 'duplicate skill: .grove..*fixpack \(aggregate\)' \
  "${market}" "${cache}"

# A pack whose manifest names an owner that it does not ship fails alone.
orphan="${scratch}/orphan"
mkdir -p "${orphan}/plugins/fixpack/conformance"
make_pack "${orphan}" fixture-note >/dev/null
printf 'fixture-orphan\tfixture-note/absent.md\tS\tstatic(all)\t\n' \
  >>"${orphan}/plugins/fixpack/conformance/rules.tsv"
git -C "${orphan}" commit --quiet -am orphan
write_marketplace "${market}" "file://${orphan}" "$(git -C "${orphan}" rev-parse HEAD)"
check "a pack that fails alone fails" 1 'unshipped owner.*fixpack \(alone\)' "${market}" "${cache}"

printf '{"name": "fixture", "plugins": [{"name": "local", "source": "./plugins/grove"}]}\n' >"${market}"
check "a marketplace with no git-subdir entry has nothing to check" 0 'pins no git-subdir pack' \
  "${market}" "${cache}"

printf '\n%d passed, %d failed\n' "${pass}" "${fail}"
[[ "${fail}" -eq 0 ]]

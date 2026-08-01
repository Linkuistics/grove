#!/usr/bin/env bash
# Tests for install.sh, and for its workspace guard in particular. Dependency-
# free (bats is not assumed): build each working-tree shape in a scratch
# directory, run the real install.sh against an isolated HOME, assert the exit
# status, the number of symlinks created, and the diagnostic.
# Run: bash plugins/install.test.sh
#
# SAFETY. Every run sets HOME to a throwaway directory and nothing here ever
# writes the real one. That is not incidental tidiness: the defect under test
# *is* an unwanted write to $HOME, so reproducing it there would cost the
# operator a manual repair of every installed skill link.
#
# The jj cases are skipped when jj is not on PATH; the git cases are not, since
# a repo you can clone is a repo you can run git in.

set -euo pipefail
IFS=$'\n\t'

here="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
script="${here}/install.sh"
pass=0
fail=0
skip=0

# One dummy skill per real skill, so a full run links the same count the real
# repo does — the expectation stays correct when a skill is added or removed.
mapfile -t skill_names < <(
  find "${here}/linkuistics/skills" -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | sort
)
expected_links=$((${#skill_names[@]} * 3))

scratch="$(mktemp -d)"
trap 'rm -rf "${scratch}"' EXIT

# Deterministic identities: the isolated HOME hides the operator's git and jj
# config, so the fixtures must carry their own or `commit` and `workspace add`
# refuse.
export GIT_AUTHOR_NAME="grove test" GIT_AUTHOR_EMAIL="test@example.invalid"
export GIT_COMMITTER_NAME="grove test" GIT_COMMITTER_EMAIL="test@example.invalid"
export JJ_USER="grove test" JJ_EMAIL="test@example.invalid"

# make_repo <dir>: the install-relevant shape of this repo — plugins/install.sh plus
# one skill directory per real skill. Each carries a SKILL.md because git and
# jj do not track empty directories, and a secondary workspace that checked out
# no skills would fail install.sh's skills-directory check before ever reaching
# the guard under test.
make_repo() {
  local dir="$1" name
  mkdir -p "${dir}/plugins/linkuistics/skills"
  cp "${script}" "${dir}/plugins/install.sh"
  for name in "${skill_names[@]}"; do
    mkdir -p "${dir}/plugins/linkuistics/skills/${name}"
    printf -- '---\nname: %s\ndescription: fixture\n---\n' "${name}" \
      >"${dir}/plugins/linkuistics/skills/${name}/SKILL.md"
  done
}

# make_home: a fresh isolated HOME with all three harness directories present,
# so a full run links every skill into all three rather than printing `skip`.
make_home() {
  local home
  home="$(mktemp -d "${scratch}/home.XXXXXX")"
  mkdir -p "${home}/.codex" "${home}/.gemini" "${home}/.pi/agent"
  printf '%s' "${home}"
}

# check <name> <want-status> <want-links> <want-stderr-regex|-> <repo-dir> [args...]
check() {
  local name="$1" want_status="$2" want_links="$3" want_regex="$4" dir="$5"
  shift 5
  local home output status links problem=""
  home="$(make_home)"
  set +e
  output="$(HOME="${home}" bash "${dir}/plugins/install.sh" "$@" 2>&1)"
  status=$?
  set -e
  links="$(find "${home}" -type l | wc -l | tr -d ' ')"

  [[ "${status}" == "${want_status}" ]] ||
    problem+=" exit ${status} (want ${want_status});"
  [[ "${links}" == "${want_links}" ]] ||
    problem+=" ${links} link(s) (want ${want_links});"
  if [[ "${want_regex}" != "-" && ! "${output}" =~ ${want_regex} ]]; then
    problem+=" output does not match /${want_regex}/;"
  fi

  if [[ -z "${problem}" ]]; then
    pass=$((pass + 1))
  else
    fail=$((fail + 1))
    printf 'FAIL: %s —%s\n' "${name}" "${problem}" >&2
    printf '      output: %s\n' "${output}" >&2
  fi
}

# --- argument handling -------------------------------------------------------

make_repo "${scratch}/args"
check "--help exits 0 without linking" 0 0 'usage: ./plugins/install.sh' "${scratch}/args" --help
check "unknown argument is rejected" 2 0 'unknown argument: --nope' "${scratch}/args" --nope

# --- git: a plain checkout installs, a linked worktree does not --------------

git_main="${scratch}/git-main"
make_repo "${git_main}"
git init -q "${git_main}"
git -C "${git_main}" add -A
git -C "${git_main}" commit -qm init
git -C "${git_main}" worktree add -q -b side "${scratch}/git-worktree"

check "git main checkout installs" 0 "${expected_links}" 'linked' "${git_main}"
check "git linked worktree refuses" 1 0 'refusing to install from a secondary' \
  "${scratch}/git-worktree"
check "git linked worktree --force installs, loudly" 0 "${expected_links}" \
  'not the main checkout' "${scratch}/git-worktree" --force

# --- jj: a default workspace installs, a secondary one does not --------------

# build_jj_repo <main> <secondary> [jj-git-init-flags...]: a jj repo carrying
# one commit, plus a secondary workspace checked out from it. The commit is
# required — jj tracks no empty directories and a workspace added before one
# exists would contain no skills at all. Returns non-zero on any failed step so
# that a jj whose CLI has moved on is reported as a broken fixture rather than
# killing the whole run with no output.
build_jj_repo() {
  local main="$1" secondary="$2"
  shift 2
  make_repo "${main}" || return 1
  jj git init "$@" "${main}" >/dev/null 2>&1 || return 1
  jj -R "${main}" describe -m init >/dev/null 2>&1 || return 1
  jj -R "${main}" new >/dev/null 2>&1 || return 1
  jj -R "${main}" workspace add "${secondary}" >/dev/null 2>&1 || return 1
}

if ! command -v jj >/dev/null 2>&1; then
  skip=$((skip + 5))
  printf 'SKIP: jj not on PATH — 5 jj workspace cases not run\n' >&2
elif ! build_jj_repo "${scratch}/jj-main" "${scratch}/jj-secondary"; then
  fail=$((fail + 1))
  printf 'FAIL: could not build the native jj fixture\n' >&2
elif ! build_jj_repo "${scratch}/colo-main" "${scratch}/colo-secondary" --colocate; then
  fail=$((fail + 1))
  printf 'FAIL: could not build the colocated jj fixture\n' >&2
else
  check "jj default workspace installs" 0 "${expected_links}" 'linked' "${scratch}/jj-main"
  check "jj secondary workspace refuses" 1 0 'refusing to install from a secondary' \
    "${scratch}/jj-secondary"
  check "jj secondary workspace --force installs, loudly" 0 "${expected_links}" \
    'not the main checkout' "${scratch}/jj-secondary" --force

  # jj-first is load-bearing here, not cosmetic: a secondary workspace of a
  # *colocated* repo is not a git worktree, so the git probe reports "not a
  # repository" and only the jj probe can see that this is a side tree.
  check "colocated default workspace installs" 0 "${expected_links}" 'linked' \
    "${scratch}/colo-main"
  check "colocated secondary workspace refuses" 1 0 'refusing to install from a secondary' \
    "${scratch}/colo-secondary"
fi

# --- no probeable VCS: the guard disables rather than blocks ------------------

# An unpacked tarball has no marker to judge, and one nested inside an
# unrelated git checkout must not be judged by *that* repo's answer.
make_repo "${scratch}/git-main/vendored"
check "unpacked copy inside another repo installs" 0 "${expected_links}" 'linked' \
  "${scratch}/git-main/vendored"

# --- summary -----------------------------------------------------------------

printf '\n%d passed, %d failed, %d skipped\n' "${pass}" "${fail}" "${skip}"
[[ "${fail}" -eq 0 ]]

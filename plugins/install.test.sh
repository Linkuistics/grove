#!/usr/bin/env bash
# Tests for install.sh: its harness-eligibility filter, its reconciliation of
# links it previously laid down, and its workspace guard. Dependency-free (bats
# is not assumed): build each working-tree shape in a scratch directory, run the
# real install.sh against an isolated HOME, assert the exit status, the number of
# symlinks created, and the diagnostic.
# Run: bash plugins/install.test.sh
#
# SAFETY. Every run sets HOME to a throwaway directory and nothing here ever
# writes the real one. That is not incidental tidiness: one defect under test
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

scratch="$(mktemp -d)"
trap 'rm -rf "${scratch}"' EXIT

# Deterministic identities: the isolated HOME hides the operator's git and jj
# config, so the fixtures must carry their own or `commit` and `workspace add`
# refuse.
export GIT_AUTHOR_NAME="grove test" GIT_AUTHOR_EMAIL="test@example.invalid"
export GIT_COMMITTER_NAME="grove test" GIT_COMMITTER_EMAIL="test@example.invalid"
export JJ_USER="grove test" JJ_EMAIL="test@example.invalid"

record() {
  local name="$1" problem="$2" output="${3:-}"
  if [[ -z "${problem}" ]]; then
    pass=$((pass + 1))
  else
    fail=$((fail + 1))
    printf 'FAIL: %s —%s\n' "${name}" "${problem}" >&2
    [[ -n "${output}" ]] && printf '      output: %s\n' "${output}" >&2
  fi
}

# make_skill <repo> <plugin> <name> <extra-frontmatter-lines>
make_skill() {
  local dir="$1" plugin="$2" name="$3" extra="$4"
  mkdir -p "${dir}/plugins/${plugin}/skills/${name}"
  {
    printf -- '---\nname: %s\ndescription: fixture\n' "${name}"
    [[ -n "${extra}" ]] && printf '%s\n' "${extra}"
    printf -- '---\n\nbody\n'
  } >"${dir}/plugins/${plugin}/skills/${name}/SKILL.md"
}

# make_repo <dir>: the install-relevant shape of this repo — plugins/install.sh
# plus a **fixed synthetic** skill set spanning both plugin directories and every
# eligibility case. Fixed rather than mirrored from the real corpus, because the
# expectations below are about the filter: a corpus edit must not silently move
# them, and the real corpus gets its own case ("every bundled skill declares").
# Each skill carries a SKILL.md because git and jj track no empty directories,
# and a secondary workspace that checked out no skills would fail install.sh's
# skills-present check before ever reaching the guard under test.
#
# Eligible per harness: portable-one, portable-two, bundled-elsewhere everywhere;
# codex-only on codex alone. So codex gets 4 and each of gemini and pi gets 3.
readonly EXPECTED_LINKS=10
make_repo() {
  local dir="$1"
  mkdir -p "${dir}/plugins"
  cp "${script}" "${dir}/plugins/install.sh"
  make_skill "${dir}" linkuistics portable-one 'harnesses: [any]'
  make_skill "${dir}" linkuistics portable-two $'harnesses: [any]\nassumes-personal-setup: true'
  make_skill "${dir}" linkuistics claude-only 'harnesses: [claude-code]'
  make_skill "${dir}" linkuistics codex-only 'harnesses: [codex]'
  make_skill "${dir}" linkuistics undeclared ''
  make_skill "${dir}" otherplugin bundled-elsewhere 'harnesses: [any]'
}

# make_home: a fresh isolated HOME with all three harness directories present,
# so a full run links every eligible skill into all three rather than printing
# `skip … (harness not installed)`.
make_home() {
  local home
  home="$(mktemp -d "${scratch}/home.XXXXXX")"
  mkdir -p "${home}/.codex" "${home}/.gemini" "${home}/.pi/agent"
  printf '%s' "${home}"
}

# run_install <home> <repo-dir> [args...]: print the run's combined output and
# leave its exit status in ${status_file}. The status goes through a file rather
# than a variable because every caller reads the output through a command
# substitution, and a variable set inside that subshell would not survive it.
status_file="${scratch}/last-status"
run_install() {
  local home="$1" dir="$2"
  shift 2
  local st=0
  HOME="${home}" bash "${dir}/plugins/install.sh" "$@" 2>&1 || st=$?
  printf '%s' "${st}" >"${status_file}"
}

last_status() { cat "${status_file}"; }

count_links() { find "$1" -type l | wc -l | tr -d ' '; }

# check <name> <want-status> <want-links> <want-stderr-regex|-> <repo-dir> [args...]
check() {
  local name="$1" want_status="$2" want_links="$3" want_regex="$4" dir="$5"
  shift 5
  local home output links problem=""
  home="$(make_home)"
  output="$(run_install "${home}" "${dir}" "$@")"
  links="$(count_links "${home}")"

  [[ "$(last_status)" == "${want_status}" ]] ||
    problem+=" exit $(last_status) (want ${want_status});"
  [[ "${links}" == "${want_links}" ]] ||
    problem+=" ${links} link(s) (want ${want_links});"
  if [[ "${want_regex}" != "-" && ! "${output}" =~ ${want_regex} ]]; then
    problem+=" output does not match /${want_regex}/;"
  fi
  record "${name}" "${problem}" "${output}"
}

# --- argument handling -------------------------------------------------------

make_repo "${scratch}/args"
check "--help exits 0 without linking" 0 0 'usage: ./plugins/install.sh' "${scratch}/args" --help
check "unknown argument is rejected" 2 0 'unknown argument: --nope' "${scratch}/args" --nope

# --- the eligibility filter --------------------------------------------------

filt="${scratch}/filter"
make_repo "${filt}"
home="$(make_home)"
output="$(run_install "${home}" "${filt}")"

problem=""
[[ "$(last_status)" == 0 ]] || problem+=" exit $(last_status);"
[[ "$(count_links "${home}")" == "${EXPECTED_LINKS}" ]] ||
  problem+=" $(count_links "${home}") link(s) (want ${EXPECTED_LINKS});"
record "eligible skills link, ineligible ones do not" "${problem}" "${output}"

# The claude-only skill is the concrete defect this leaf exists for: before the
# filter, `guardrail` — a skill whose whole mechanism is a Claude Code hook —
# was symlinked into ~/.codex/skills, where its instructions are unfollowable.
problem=""
for harness in codex gemini pi; do
  case "${harness}" in
    codex) dir="${home}/.codex/skills" ;;
    gemini) dir="${home}/.gemini/skills" ;;
    pi) dir="${home}/.pi/agent/skills" ;;
  esac
  [[ -e "${dir}/claude-only" ]] && problem+=" claude-only linked into ${harness};"
  [[ -e "${dir}/undeclared" ]] && problem+=" undeclared linked into ${harness};"
  [[ -L "${dir}/portable-one" ]] || problem+=" portable-one missing from ${harness};"
  [[ -L "${dir}/bundled-elsewhere" ]] ||
    problem+=" bundled-elsewhere (second plugin) missing from ${harness};"
done
record "a Claude-only skill reaches no other harness" "${problem}" "${output}"

# An allowlist naming one harness reaches that harness and no other. `any` is a
# claim about the skill; a list is an enumeration, and both must filter.
problem=""
[[ -L "${home}/.codex/skills/codex-only" ]] || problem+=" codex-only missing from codex;"
[[ -e "${home}/.pi/agent/skills/codex-only" ]] && problem+=" codex-only linked into pi;"
record "an explicit allowlist reaches only what it names" "${problem}" "${output}"

# Silence about a skipped skill is how a user concludes the install is broken,
# so every skip is reported with the declaration that caused it.
problem=""
[[ "${output}" =~ skip[[:space:]]+codex[[:space:]]+claude-only[[:space:]]+\(harnesses:\ claude-code\) ]] ||
  problem+=" no skip line naming claude-only and its declaration;"
[[ "${output}" =~ undeclared[[:space:]]+\(declare\ one ]] ||
  problem+=" no note telling the author to declare a harnesses: key;"
[[ "${output}" =~ assumes-personal-setup ]] ||
  problem+=" no note about the skill assuming the author's own setup;"
[[ "${output}" =~ portable-two ]] ||
  problem+=" the assumes-personal-setup note does not name the skill;"
record "every skip and every personal assumption is reported" "${problem}" "${output}"

# --- absent metadata ---------------------------------------------------------

# The default is *skip, loudly*. Neither silent default is safe here: of the
# skills this repo bundles, 15 of 16 are portable and 1 is Claude-only, so
# "install everywhere" would silently mis-install exactly the skill that cannot
# work, and "Claude Code only" would silently withhold the other 15. Skipping
# never mis-installs, and the note removes the silence — and since every bundled
# skill declares a key, it only ever fires on a newly authored one, which is the
# moment the reminder is worth most.
problem=""
[[ -e "${home}/.codex/skills/undeclared" ]] && problem+=" a skill with no harnesses: key was installed;"
[[ "$(last_status)" == 0 ]] ||
  problem+=" exit $(last_status) — one under-annotated skill must not block the other 15;"
record "absent metadata skips rather than installs, and says so" "${problem}" "${output}"

# --- reconciliation on re-run ------------------------------------------------

# Losing eligibility must remove the link, not leave it installed from a
# previous run: a live symlink is indistinguishable from a deliberate install,
# and the harness would keep loading instructions it cannot follow.
recon="${scratch}/reconcile"
make_repo "${recon}"
home="$(make_home)"
run_install "${home}" "${recon}" >/dev/null
problem=""
[[ -L "${home}/.codex/skills/portable-one" ]] || problem+=" fixture did not link portable-one;"
make_skill "${recon}" linkuistics portable-one 'harnesses: [claude-code]'
output="$(run_install "${home}" "${recon}")"
[[ -e "${home}/.codex/skills/portable-one" ]] && problem+=" link survived the loss of eligibility;"
[[ "${output}" =~ unlink[[:space:]]+codex[[:space:]]+portable-one ]] ||
  problem+=" removal was not reported;"
record "losing eligibility removes the existing symlink" "${problem}" "${output}"

# The same reconciliation covers the older wart: a skill deleted or renamed
# upstream used to leave a dangling link that reads as "skill not installed".
del="${scratch}/deleted"
make_repo "${del}"
home="$(make_home)"
run_install "${home}" "${del}" >/dev/null
rm -rf "${del}/plugins/linkuistics/skills/portable-two"
output="$(run_install "${home}" "${del}")"
problem=""
[[ -e "${home}/.codex/skills/portable-two" || -L "${home}/.codex/skills/portable-two" ]] &&
  problem+=" a deleted skill left its link behind;"
record "a deleted skill's link is removed rather than left dangling" "${problem}" "${output}"

# Ownership is two conditions — this repo's directory shape *and* a skill name
# this repo ships — so nothing else in the harness's directory is ever touched.
keep="${scratch}/keep"
make_repo "${keep}"
home="$(make_home)"
mkdir -p "${scratch}/foreign/portable-one" "${home}/.codex/skills"
ln -s "${scratch}/foreign/portable-one" "${home}/.codex/skills/foreign-skill"
ln -s "${scratch}/foreign" "${home}/.codex/skills/some-other-corpus"
output="$(run_install "${home}" "${keep}")"
problem=""
[[ -L "${home}/.codex/skills/foreign-skill" ]] || problem+=" a foreign symlink was removed;"
[[ -L "${home}/.codex/skills/some-other-corpus" ]] || problem+=" an unrelated symlink was removed;"
record "symlinks this script did not place are left alone" "${problem}" "${output}"

# --- the real corpus is fully annotated --------------------------------------

# The hard gate. install.sh only *reports* a missing key, so that one
# unannotated skill cannot block the rest of an install; the assertion that
# every bundled skill actually carries one belongs here.
problem=""
while IFS= read -r skill_md; do
  awk '/^---[[:space:]]*$/ { b++; if (b > 1) exit } b == 1 && /^harnesses:/ { found = 1 }
       END { exit !found }' "${skill_md}" ||
    problem+=" $(basename "$(dirname "${skill_md}")") declares no harnesses:;"
done < <(find "${here}" -mindepth 3 -maxdepth 3 -name SKILL.md)
record "every bundled skill declares its harnesses" "${problem}"

# --- git: a plain checkout installs, a linked worktree does not --------------

git_main="${scratch}/git-main"
make_repo "${git_main}"
git init -q "${git_main}"
git -C "${git_main}" add -A
git -C "${git_main}" commit -qm init
git -C "${git_main}" worktree add -q -b side "${scratch}/git-worktree"

check "git main checkout installs" 0 "${EXPECTED_LINKS}" 'linked' "${git_main}"
check "git linked worktree refuses" 1 0 'refusing to install from a secondary' \
  "${scratch}/git-worktree"
check "git linked worktree --force installs, loudly" 0 "${EXPECTED_LINKS}" \
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
  check "jj default workspace installs" 0 "${EXPECTED_LINKS}" 'linked' "${scratch}/jj-main"
  check "jj secondary workspace refuses" 1 0 'refusing to install from a secondary' \
    "${scratch}/jj-secondary"
  check "jj secondary workspace --force installs, loudly" 0 "${EXPECTED_LINKS}" \
    'not the main checkout' "${scratch}/jj-secondary" --force

  # jj-first is load-bearing here, not cosmetic: a secondary workspace of a
  # *colocated* repo is not a git worktree, so the git probe reports "not a
  # repository" and only the jj probe can see that this is a side tree.
  check "colocated default workspace installs" 0 "${EXPECTED_LINKS}" 'linked' \
    "${scratch}/colo-main"
  check "colocated secondary workspace refuses" 1 0 'refusing to install from a secondary' \
    "${scratch}/colo-secondary"
fi

# --- no probeable VCS: the guard disables rather than blocks ------------------

# An unpacked tarball has no marker to judge, and one nested inside an
# unrelated git checkout must not be judged by *that* repo's answer.
make_repo "${scratch}/git-main/vendored"
check "unpacked copy inside another repo installs" 0 "${EXPECTED_LINKS}" 'linked' \
  "${scratch}/git-main/vendored"

# --- summary -----------------------------------------------------------------

printf '\n%d passed, %d failed, %d skipped\n' "${pass}" "${fail}" "${skip}"
[[ "${fail}" -eq 0 ]]

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
check "--help names both skill roots" 0 0 'must hold skills/, craft/ or both' "${scratch}/args" --help

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

# --- a real file or directory already at a target path -----------------------

# The live case was the `grove` binary, which once provisioned its own methodology
# into ~/.codex/skills/grove and ~/.pi/agent/skills/grove — a real directory at
# exactly a path this script installs to. Older builds can leave one behind. Nothing there is this script's to
# replace, so it must leave it alone; and leaving it alone silently is the
# failure mode the workspace guard already refuses, because the skill is then
# uninstalled with nothing in the harness saying so.
occupied="${scratch}/occupied"
make_repo "${occupied}"
home="$(make_home)"
mkdir -p "${home}/.codex/skills/portable-one"
printf 'provisioned by something else\n' >"${home}/.codex/skills/portable-one/SKILL.md"
output="$(run_install "${home}" "${occupied}")"
problem=""
[[ "$(last_status)" == 1 ]] ||
  problem+=" exit $(last_status) (want 1) — an uninstalled skill must not read as success;"
[[ -f "${home}/.codex/skills/portable-one/SKILL.md" ]] ||
  problem+=" the occupying directory was replaced;"
[[ "$(cat "${home}/.codex/skills/portable-one/SKILL.md")" == "provisioned by something else" ]] ||
  problem+=" the occupying directory's contents changed;"
[[ "${output}" =~ error[[:space:]]+.*portable-one[[:space:]]not[[:space:]]installed ]] ||
  problem+=" the collision was not reported against the skill;"
[[ "${output}" =~ Nothing[[:space:]]was[[:space:]]overwritten ]] ||
  problem+=" the closing report does not say what was and was not done;"
# Every other skill still installs: one blocked path must not cost the rest.
[[ -L "${home}/.pi/agent/skills/portable-one" ]] ||
  problem+=" a collision on one harness blocked the same skill on another;"
[[ -L "${home}/.codex/skills/portable-two" ]] ||
  problem+=" a collision on one path blocked an unrelated skill;"
record "a real file at a target path refuses rather than clobbers" "${problem}" "${output}"

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

# --- external packs ----------------------------------------------------------

# `--pack <plugin-dir>` adds the plugin directory of another checkout. The pack
# fixture ships one portable skill and one Claude-only skill, so a run with the
# pack adds 3 links: one in each harness.
make_pack() {
  local dir="$1"
  make_skill "${dir}" fixpack pack-skill 'harnesses: [any]'
  make_skill "${dir}" fixpack pack-claude 'harnesses: [claude-code]'
}

base="${scratch}/pack-base"
make_repo "${base}"
pack="${scratch}/pack-repo"
make_pack "${pack}"

check "--pack without a directory is rejected" 2 0 '--pack needs a plugin directory' "${base}" --pack
check "--pack of a directory with neither root is rejected" 2 0 \
  'is not a plugin directory plugins/<plugin> with skills/ or craft/' \
  "${base}" --pack "${pack}/plugins"
mkdir -p "${scratch}/not-plugins/fixpack/skills"
check "--pack outside plugins/<plugin> is rejected" 2 0 'is not a plugin directory' \
  "${base}" --pack "${scratch}/not-plugins/fixpack"

home="$(make_home)"
output="$(run_install "${home}" "${base}" --pack "${pack}/plugins/fixpack")"
problem=""
[[ "$(last_status)" == 0 ]] || problem+=" exit $(last_status);"
[[ "$(count_links "${home}")" == $((EXPECTED_LINKS + 3)) ]] ||
  problem+=" $(count_links "${home}") link(s) (want $((EXPECTED_LINKS + 3)));"
[[ "$(readlink "${home}/.codex/skills/pack-skill")" == "$(cd "${pack}" && pwd -P)/plugins/fixpack/skills/pack-skill" ]] ||
  problem+=" pack-skill does not point into the pack;"
[[ -e "${home}/.codex/skills/pack-claude" ]] && problem+=" a Claude-only pack skill reached Codex;"
record "--pack links the eligible skills of the pack" "${problem}" "${output}"

# A run that does not scan the pack does not own its links.
output="$(run_install "${home}" "${base}")"
problem=""
[[ -L "${home}/.codex/skills/pack-skill" ]] || problem+=" a run without --pack removed a pack link;"
record "a run without --pack leaves the pack's links unchanged" "${problem}" "${output}"

# A run that scans the pack reconciles its links.
rm -rf "${pack}/plugins/fixpack/skills/pack-skill"
output="$(run_install "${home}" "${base}" --pack "${pack}/plugins/fixpack")"
problem=""
[[ -e "${home}/.codex/skills/pack-skill" || -L "${home}/.codex/skills/pack-skill" ]] &&
  problem+=" a skill deleted from the pack left its link behind;"
[[ "${output}" =~ unlink[[:space:]]+codex[[:space:]]+pack-skill ]] || problem+=" the removal was not reported;"
record "--pack reconciles the links of the pack" "${problem}" "${output}"

# One skill name in two scanned plugins would make one link with the last
# writer winning. The run refuses before it changes any link.
dup="${scratch}/pack-duplicate"
make_skill "${dup}" fixpack portable-one 'harnesses: [any]'
check "a pack skill named like a bundled skill is refused" 1 0 'more than one scanned skill root' \
  "${base}" --pack "${dup}/plugins/fixpack"

# The workspace guard covers the pack's checkout.
pack_main="${scratch}/pack-git-main"
make_pack "${pack_main}"
git init -q "${pack_main}"
git -C "${pack_main}" add -A
git -C "${pack_main}" commit -qm init
git -C "${pack_main}" worktree add -q -b side "${scratch}/pack-git-worktree"
check "--pack from the pack's main checkout installs" 0 $((EXPECTED_LINKS + 3)) 'linked' \
  "${base}" --pack "${pack_main}/plugins/fixpack"
check "--pack from a linked worktree of the pack refuses" 1 0 'refusing to install from a secondary' \
  "${base}" --pack "${scratch}/pack-git-worktree/plugins/fixpack"
check "--pack from a linked worktree with --force installs, loudly" 0 $((EXPECTED_LINKS + 3)) \
  'not the main checkout' "${base}" --force --pack "${scratch}/pack-git-worktree/plugins/fixpack"

# --- craft roots -------------------------------------------------------------

# A plugin directory ships methodology skills in `skills/` and craft skills in
# `craft/`. The installer treats the two roots alike: the same eligibility, the
# same links, the same ownership test and the same reconciliation. A pack can
# ship `craft/` beside `skills/`, or `craft/` alone.

# make_craft_skill <repo> <plugin> <name> <extra-frontmatter-lines>
make_craft_skill() {
  local dir="$1" plugin="$2" name="$3" extra="$4"
  mkdir -p "${dir}/plugins/${plugin}/craft/${name}"
  {
    printf -- '---\nname: %s\ndescription: fixture\n' "${name}"
    [[ -n "${extra}" ]] && printf '%s\n' "${extra}"
    printf -- '---\n\nbody\n'
  } >"${dir}/plugins/${plugin}/craft/${name}/SKILL.md"
}

# make_craft_beside_pack <repo>: the plugin directory `plugins/craftbeside`, with
# one portable skill in `skills/` and two portable craft skills in `craft/`. A
# run with the pack adds 9 links: three in each harness.
make_craft_beside_pack() {
  local dir="$1"
  make_skill "${dir}" craftbeside beside-kind 'harnesses: [any]'
  make_craft_skill "${dir}" craftbeside beside-craft-one 'harnesses: [any]'
  make_craft_skill "${dir}" craftbeside beside-craft-two 'harnesses: [any]'
}

# make_craft_only_pack <repo>: the plugin directory `plugins/craftonly`, with
# two portable craft skills in `craft/` and no `skills/`. A run with the pack
# adds 6 links: two in each harness. personal-pack-k183 names this fixture as
# the static proof of a source whose only root is `craft`, so keep its shape
# plain.
make_craft_only_pack() {
  local dir="$1"
  make_craft_skill "${dir}" craftonly only-craft-one 'harnesses: [any]'
  make_craft_skill "${dir}" craftonly only-craft-two 'harnesses: [any]'
}

# craft_pack_cycle <label> <pack-plugin-dir> <links-added> <kept-skill> <removed-skill>:
# install the pack, delete one craft skill, run without the pack, then run with
# the pack. The run without the pack leaves the stale craft link. The run with
# the pack removes it and keeps the other craft skill's link.
craft_pack_cycle() {
  local label="$1" plugin_dir="$2" added="$3" kept="$4" removed="$5"
  local home output problem=""
  home="$(make_home)"
  output="$(run_install "${home}" "${base}" --pack "${plugin_dir}")"
  [[ "$(last_status)" == 0 ]] || problem+=" exit $(last_status);"
  [[ "$(count_links "${home}")" == $((EXPECTED_LINKS + added)) ]] ||
    problem+=" $(count_links "${home}") link(s) (want $((EXPECTED_LINKS + added)));"
  [[ "$(readlink "${home}/.codex/skills/${removed}")" == "$(cd "${plugin_dir}" && pwd -P)/craft/${removed}" ]] ||
    problem+=" ${removed} does not point into craft/ of the pack;"
  record "--pack links the craft skills of ${label}" "${problem}" "${output}"

  rm -rf "${plugin_dir:?}/craft/${removed}"
  output="$(run_install "${home}" "${base}")"
  problem=""
  [[ -L "${home}/.codex/skills/${removed}" ]] ||
    problem+=" a run without --pack removed a craft link of the pack;"
  record "a run without --pack leaves the craft links of ${label} unchanged" "${problem}" "${output}"

  output="$(run_install "${home}" "${base}" --pack "${plugin_dir}")"
  problem=""
  [[ -e "${home}/.codex/skills/${removed}" || -L "${home}/.codex/skills/${removed}" ]] &&
    problem+=" a craft skill deleted from the pack left its link behind;"
  [[ "${output}" =~ unlink[[:space:]]+codex[[:space:]]+${removed} ]] ||
    problem+=" the removal was not reported;"
  [[ -L "${home}/.codex/skills/${kept}" ]] || problem+=" the other craft link was removed;"
  record "--pack reconciles the craft links of ${label}" "${problem}" "${output}"
}

beside="${scratch}/craft-beside-repo"
make_craft_beside_pack "${beside}"
craft_pack_cycle "a pack with craft/ beside skills/" "${beside}/plugins/craftbeside" 9 \
  beside-craft-one beside-craft-two

only="${scratch}/craft-only-repo"
make_craft_only_pack "${only}"
craft_pack_cycle "a pack with craft/ and no skills/" "${only}/plugins/craftonly" 6 \
  only-craft-one only-craft-two

# A bundled plugin directory can also ship `craft/`, with or without `skills/`.
bundled="${scratch}/craft-bundled"
make_repo "${bundled}"
make_craft_skill "${bundled}" craftbundled bundled-craft 'harnesses: [any]'
home="$(make_home)"
run_install "${home}" "${bundled}" >/dev/null
problem=""
[[ -L "${home}/.codex/skills/bundled-craft" ]] || problem+=" fixture did not link bundled-craft;"
rm -rf "${bundled}/plugins/craftbundled/craft/bundled-craft"
output="$(run_install "${home}" "${bundled}")"
[[ -e "${home}/.codex/skills/bundled-craft" || -L "${home}/.codex/skills/bundled-craft" ]] &&
  problem+=" a craft skill deleted from a bundled plugin left its link behind;"
record "a bundled craft skill's link is reconciled" "${problem}" "${output}"

# Only the two roots are scanned. A link to a skill in any other directory would
# be one that no later run recognises as the installer's own.
stray="${scratch}/craft-stray"
make_skill "${stray}" straypack stray-kind 'harnesses: [any]'
mkdir -p "${stray}/plugins/straypack/notes/stray-note"
printf -- '---\nname: stray-note\ndescription: fixture\nharnesses: [any]\n---\n' \
  >"${stray}/plugins/straypack/notes/stray-note/SKILL.md"
home="$(make_home)"
output="$(run_install "${home}" "${base}" --pack "${stray}/plugins/straypack")"
problem=""
[[ -L "${home}/.codex/skills/stray-kind" ]] || problem+=" the skill in skills/ was not linked;"
[[ -e "${home}/.codex/skills/stray-note" ]] && problem+=" a directory outside both roots was scanned;"
record "only skills/ and craft/ are scanned" "${problem}" "${output}"

# One name in both roots, of one plugin or of two, would claim one link. The
# refusal names each path that ships the name, so the reader knows which to
# rename.
dupcraft="${scratch}/craft-duplicate"
make_craft_skill "${dupcraft}" dupcraft portable-one 'harnesses: [any]'
make_skill "${dupcraft}" dupcraft twice 'harnesses: [any]'
make_craft_skill "${dupcraft}" dupcraft twice 'harnesses: [any]'
home="$(make_home)"
output="$(run_install "${home}" "${base}" --pack "${dupcraft}/plugins/dupcraft")"
problem=""
[[ "$(last_status)" == 1 ]] || problem+=" exit $(last_status) (want 1);"
[[ "$(count_links "${home}")" == 0 ]] || problem+=" $(count_links "${home}") link(s) (want 0);"
[[ "${output}" =~ plugins/dupcraft/craft/portable-one ]] ||
  problem+=" the refusal does not name the craft path of portable-one;"
[[ "${output}" =~ plugins/linkuistics/skills/portable-one ]] ||
  problem+=" the refusal does not name the bundled path of portable-one;"
[[ "${output}" =~ plugins/dupcraft/skills/twice ]] ||
  problem+=" the refusal does not name the skills/ path of twice;"
[[ "${output}" =~ plugins/dupcraft/craft/twice ]] ||
  problem+=" the refusal does not name the craft/ path of twice;"
record "a name shipped twice across both roots is refused before any link" "${problem}" "${output}"

# --- Pi extensions -----------------------------------------------------------

# A skill that ships `harnesses/pi/<name>.ts` and is eligible for Pi gets that
# file linked into ~/.pi/agent/extensions/. It is code Pi runs on every start, so
# the link follows the skill's own eligibility and the skill links' ownership and
# reconciliation rules. These fixtures sit beside the fixed skill set rather than
# in it, because EXPECTED_LINKS counts that set.
make_extension() {
  local dir="$1" skill="$2" name="$3"
  mkdir -p "${dir}/plugins/linkuistics/skills/${skill}/harnesses/pi"
  printf 'export default function () {}\n' \
    >"${dir}/plugins/linkuistics/skills/${skill}/harnesses/pi/${name}"
}

ext="${scratch}/extension"
make_repo "${ext}"
make_extension "${ext}" portable-one hook.ts
make_extension "${ext}" codex-only never.ts
home="$(make_home)"
output="$(run_install "${home}" "${ext}")"
extdir="${home}/.pi/agent/extensions"
problem=""
[[ "$(last_status)" == 0 ]] || problem+=" exit $(last_status);"
[[ -L "${extdir}/hook.ts" ]] || problem+=" the extension was not linked;"
[[ "$(readlink "${extdir}/hook.ts")" == "${ext}/plugins/linkuistics/skills/portable-one/harnesses/pi/hook.ts" ]] ||
  problem+=" the link does not point at the skill's file;"
[[ -e "${extdir}/never.ts" ]] && problem+=" an extension of a skill that is not eligible for Pi was linked;"
[[ -e "${home}/.codex/skills/hook.ts" || -e "${home}/.codex/hook.ts" ]] && problem+=" the extension reached Codex;"
[[ "${output}" =~ linked[[:space:]]1[[:space:]]Pi[[:space:]]extension ]] || problem+=" the summary does not count it;"
record "an eligible skill's Pi extension is linked, an ineligible one is not" "${problem}" "${output}"

# A re-run converges: it replaces its own link and does not stack or fail.
output="$(run_install "${home}" "${ext}")"
problem=""
[[ "$(last_status)" == 0 ]] || problem+=" exit $(last_status);"
links="$(count_links "${extdir}" 2>/dev/null || echo none)"
[[ "${links}" == 1 ]] || problem+=" ${links} links after a re-run;"
record "re-running keeps one extension link" "${problem}" "${output}"

# Reconciliation: the file is gone, or its skill is no longer eligible for Pi.
rm "${ext}/plugins/linkuistics/skills/portable-one/harnesses/pi/hook.ts"
output="$(run_install "${home}" "${ext}")"
problem=""
[[ -e "${extdir}/hook.ts" || -L "${extdir}/hook.ts" ]] && problem+=" a deleted extension left its link behind;"
[[ "${output}" =~ unlink[[:space:]]+pi[[:space:]]+hook.ts ]] || problem+=" the removal was not reported;"
record "a deleted extension's link is removed" "${problem}" "${output}"

make_extension "${ext}" portable-two hook.ts
run_install "${home}" "${ext}" >/dev/null
make_skill "${ext}" linkuistics portable-two 'harnesses: [codex]'
output="$(run_install "${home}" "${ext}")"
problem=""
[[ -e "${extdir}/hook.ts" || -L "${extdir}/hook.ts" ]] && problem+=" the link survived the loss of Pi eligibility;"
record "losing Pi eligibility removes the extension link" "${problem}" "${output}"

# Links that are not ours stay, and a real file at a target is refused, never
# replaced: Pi runs whatever sits there.
own="${scratch}/extension-own"
make_repo "${own}"
make_extension "${own}" portable-one hook.ts
home="$(make_home)"
mkdir -p "${home}/.pi/agent/extensions" "${scratch}/foreign-ext"
printf 'export default function () {}\n' >"${scratch}/foreign-ext/mine.ts"
ln -s "${scratch}/foreign-ext/mine.ts" "${home}/.pi/agent/extensions/mine.ts"
printf 'the human wrote this\n' >"${home}/.pi/agent/extensions/hook.ts"
output="$(run_install "${home}" "${own}")"
problem=""
[[ "$(last_status)" == 1 ]] || problem+=" exit $(last_status) (want 1);"
[[ -L "${home}/.pi/agent/extensions/mine.ts" ]] || problem+=" a foreign extension link was removed;"
[[ ! -L "${home}/.pi/agent/extensions/hook.ts" && "$(cat "${home}/.pi/agent/extensions/hook.ts")" == "the human wrote this" ]] ||
  problem+=" the human's file was replaced;"
[[ "${output}" =~ hook.ts[[:space:]]extension[[:space:]]not[[:space:]]installed ]] || problem+=" the collision was not reported;"
[[ -L "${home}/.pi/agent/skills/portable-one" ]] || problem+=" the collision blocked a skill link;"
record "a real file at an extension path is refused, a foreign link is left" "${problem}" "${output}"

# Without a Pi home there is nothing to link into, and nothing is created.
home="$(mktemp -d "${scratch}/home.XXXXXX")"
mkdir -p "${home}/.codex"
output="$(run_install "${home}" "${ext}")"
problem=""
[[ -e "${home}/.pi" ]] && problem+=" ~/.pi was created;"
record "no Pi home, no extension directory" "${problem}" "${output}"

# A craft skill of an external pack ships its Pi extension, and `--pack` reaches
# it. This is the shape of `technical-communication`, which ships
# `harnesses/pi/technical-communication.ts` from `craft/` of the writing pack.
# The fixture stands in for that skill, because Grove no longer ships it.
crafted="${scratch}/extension-craft-pack"
make_craft_skill "${crafted}" craftext crafted-skill 'harnesses: [any]'
mkdir -p "${crafted}/plugins/craftext/craft/crafted-skill/harnesses/pi"
printf 'export default function () {}\n' \
  >"${crafted}/plugins/craftext/craft/crafted-skill/harnesses/pi/crafted.ts"
home="$(make_home)"
output="$(run_install "${home}" "${base}" --pack "${crafted}/plugins/craftext")"
problem=""
[[ "$(last_status)" == 0 ]] || problem+=" exit $(last_status);"
[[ -L "${home}/.pi/agent/extensions/crafted.ts" ]] || problem+=" the pack's extension was not linked;"
[[ "$(readlink "${home}/.pi/agent/extensions/crafted.ts")" == "$(cd "${crafted}" && pwd -P)/plugins/craftext/craft/crafted-skill/harnesses/pi/crafted.ts" ]] ||
  problem+=" the link does not point into craft/ of the pack;"
[[ -f "${home}/.pi/agent/extensions/crafted.ts" ]] || problem+=" the pack's extension link dangles;"
record "a pack's craft skill gets its Pi extension linked" "${problem}" "${output}"

# --- summary -----------------------------------------------------------------

printf '\n%d passed, %d failed, %d skipped\n' "${pass}" "${fail}" "${skip}"
[[ "${fail}" -eq 0 ]]

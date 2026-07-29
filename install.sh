#!/usr/bin/env bash
#
# Install the linkuistics skills into non-Claude-Code agent harnesses by
# symlinking each skill directory into that harness's personal skills folder.
#
# Claude Code does NOT need this script. Install there via the marketplace:
#   /plugin marketplace add Linkuistics/grove
#   /plugin install linkuistics@linkuistics
#
# This script covers the `linkuistics` plugin's skills only. `testanyware`
# ships through the marketplace alone, so it is Claude Code only.
#
# For the harnesses below, the SKILL.md format is shared but there is no
# package manager, so symlinks are the install mechanism. Because the targets
# are symlinks, a later `git pull` in this repo updates the content in place —
# no need to re-run this script unless skills are added or removed.
#
# Usage: ./install.sh [--force]

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
skills_dir="${repo_root}/plugins/linkuistics/skills"

print_usage() {
  cat <<'EOF'
usage: ./install.sh [--force]

  --force      link from this tree even when it is not the repo's main
               checkout — every link then dangles once this tree is
               removed, so re-run from the main checkout to repair
  -h, --help   show this help
EOF
}

force=0
for arg in "$@"; do
  case "${arg}" in
    --force) force=1 ;;
    -h | --help)
      print_usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: ${arg}" >&2
      print_usage >&2
      exit 2
      ;;
  esac
done

if [[ ! -d "${skills_dir}" ]]; then
  echo "error: skills directory not found at ${skills_dir}" >&2
  exit 1
fi

# print_real_path <dir>: <dir> with symlinks resolved; nothing if unreadable.
# `realpath` is not assumed — it is GNU coreutils on Linux and a comparatively
# recent addition on macOS.
print_real_path() {
  (cd "$1" 2>/dev/null && pwd -P)
}

# print_main_repo <dir>: the main repo behind the working tree at <dir> — the
# checkout that a linked git worktree or a secondary jj workspace belongs to,
# and <dir> itself for a plain checkout. Prints nothing and returns non-zero
# when <dir> is under no probeable VCS (an unpacked tarball, or jj/git absent
# from PATH); that deliberately disables the guard below rather than blocking
# an install this script cannot judge.
#
# jj-first, mirroring the grove binary's `repo::vcs_of` (symmetric-vcs-rule):
# a `.jj/` decides even when a `.git` sits beside it, as in a colocated repo.
# That ordering is load-bearing here, not cosmetic — a secondary jj workspace
# of a colocated repo is not a git worktree at all, so the git probe reports
# "not a repository" and would leave exactly the case this guard exists for
# undetected.
#
# `--ignore-working-copy` keeps the probe read-only: every other jj invocation
# snapshots the working copy as a side effect, and an install-time check has no
# business mutating the tree it is auditing.
print_main_repo() {
  local dir="$1" common_dir
  if [[ -d "${dir}/.jj" ]]; then
    jj -R "${dir}" workspace root --name default --ignore-working-copy 2>/dev/null
    return
  fi
  # The marker must be at <dir> itself, exactly as `vcs_of` requires — `.git`
  # as a directory in a checkout, as a gitfile in a linked worktree. Probing
  # without that check would let `git rev-parse` walk *up* out of an unpacked
  # or vendored copy of this repo and report the enclosing repository as the
  # main checkout, refusing an install that is perfectly fine.
  [[ -e "${dir}/.git" ]] || return 1
  common_dir="$(git -C "${dir}" rev-parse --git-common-dir 2>/dev/null)" || return 1
  # git prints this relative to <dir> from a plain checkout (".git") and
  # absolute from a linked worktree; the main repo is its parent either way.
  [[ "${common_dir}" == /* ]] || common_dir="${dir}/${common_dir}"
  print_real_path "${common_dir}/.."
}

# The workspace guard. This script links from whichever tree it lives in and
# re-links every skill unconditionally, so running it from a secondary jj
# workspace or a linked git worktree silently re-points *all* installed skills
# at a tree that is usually ephemeral. Nothing surfaces the breakage: `ln -s`
# does not require its target to persist, the harnesses read SKILL.md lazily,
# and a dangling skill directory reads as "skill not installed" rather than as
# an error.
#
# It refuses rather than warns because the damage is silent and delayed — a
# warning among 48 `ok` lines is read once, weeks before the symptom. It takes
# `--force` rather than refusing outright because linking from a side tree is
# sometimes deliberate: testing an unmerged skill against a live harness.
main_repo="$(print_main_repo "${repo_root}" || true)"
if [[ -n "${main_repo}" && "$(print_real_path "${repo_root}")" != "$(print_real_path "${main_repo}")" ]]; then
  if ((force)); then
    echo "warn   linking from ${repo_root}" >&2
    echo "warn   this is not the main checkout (${main_repo}); every link below" >&2
    echo "warn   dangles once this tree is removed — re-run there to repair" >&2
  else
    cat >&2 <<EOF
error: refusing to install from a secondary working tree.

  this tree:  ${repo_root}
  main repo:  ${main_repo}

install.sh links skills from the tree it lives in, and re-links every skill
unconditionally — so installing from here would re-point *all* of them at this
tree. Nothing would report it: a symlink whose target later disappears reads as
"skill not installed", not as an error.

  install normally:  cd "${main_repo}" && ./install.sh
  link here anyway:  ./install.sh --force
                     (deliberate — e.g. testing an unmerged skill live; re-run
                      from the main repo afterwards to repair the links)
EOF
    exit 1
  fi
fi

# Personal skill directories for harnesses that follow the SKILL.md open
# standard. Each is installed only if its parent harness directory exists.
harness_skill_dirs=(
  "${HOME}/.codex/skills"
  "${HOME}/.gemini/skills"
  "${HOME}/.pi/agent/skills"
)

linked=0
for target_root in "${harness_skill_dirs[@]}"; do
  harness_home="$(dirname "${target_root}")"
  if [[ ! -d "${harness_home}" ]]; then
    echo "skip   ${harness_home}  (harness not installed)"
    continue
  fi
  mkdir -p "${target_root}"
  for skill_path in "${skills_dir}"/*/; do
    skill_name="$(basename "${skill_path}")"
    link="${target_root}/${skill_name}"
    if [[ -L "${link}" ]]; then
      rm "${link}"
    elif [[ -e "${link}" ]]; then
      echo "warn   ${link} exists and is not a symlink — left untouched" >&2
      continue
    fi
    ln -s "${skill_path%/}" "${link}"
    linked=$((linked + 1))
  done
  echo "ok     ${target_root}"
done

echo "linked ${linked} skill symlink(s)"

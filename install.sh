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

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
skills_dir="${repo_root}/plugins/linkuistics/skills"

if [[ ! -d "${skills_dir}" ]]; then
  echo "error: skills directory not found at ${skills_dir}" >&2
  exit 1
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

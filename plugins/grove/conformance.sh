#!/usr/bin/env bash
#
# The methodology's delivery assertion, over core and any supplied packs.
#
# `docs/adr/behavioural-coverage-asserts-delivery.md` states the rule: a
# methodology is delivered only where a session actually reads it. This runner
# is the only standing instrument for that rule. `plugin-kind-skills-k17` and
# `delete-provisioning-k19` deleted the Rust suites over the old embedded
# corpus. Their pinned rows and removed paraphrases are in this plugin's
# `conformance/` directory, with identical wordings.
#
# A pack is a source directory that holds `skills/` and `conformance/rules.tsv`.
# Core is this plugin. `docs/specs/external-pack-contract.md`, "Conformance",
# states what a pack checks alone and what the aggregate checks.
#
# Three assertions, over each file of each skill of the run:
#
#   1. Each behavioural rule is on the composed loaded path of each kind that
#      binds it. The path starts at the kind's own `SKILL.md`. It follows
#      directed loads ("Load the `<name>` skill") and skill-relative paths.
#   2. No rule has two owners. Each row names an owner that its own pack ships.
#      No two packs ship one skill name, and no two rows have one rule ID.
#   3. Each skill-relative path resolves, and each directed load names a
#      shipped skill.
#
# The runner asserts nothing about how many kinds there are. A kind exists
# exactly when a `grove-<kind>` skill exists.
#
# One row of core's manifest has the owner `${prompt}`. The driver inlines its
# bytes into the launch prompt, and no skill carries them. The runner reports
# that row and asserts nothing about it, because it cannot read a prompt.
#
# Usage:
#
#   conformance.sh [--verbose] [--skills <dir>] [--rules <file>]
#                  [--pack <dir>]... [--alone]
#
#   (no --pack)    core alone, which is the aggregate of core only
#   --pack <dir>   adds an external pack. Repeat it for each pack. Without
#                  --alone, the run checks the aggregate of core and each pack
#   --alone        checks exactly one --pack, without core. A directed load or
#                  a path that reaches a skill of another pack is then listed as
#                  an external edge or an external resource, not a failure
#   --skills, --rules
#                  point core at another skills root or manifest. The controls
#                  in `conformance.test.sh` use them to watch each assertion fail
#                  against a skill set that is wrong in one named way
#
# A pack's Taskfile runs this file from a checkout of Grove, for example:
#
#   <grove>/plugins/grove/conformance.sh --pack plugins/research --alone
#   <grove>/plugins/grove/conformance.sh --pack plugins/research
#
# The runner reads directories and runs no VCS command. `conformance.awk`
# beside this file is the engine. This file reads the arguments and writes the
# inventory of packs, skills and files that the engine reads.
#
# Exit status: 0 all assertions hold, 1 an assertion failed, 2 bad usage.

set -euo pipefail
IFS=$'\n\t'
# Byte order for the globs, the sorts and the engine's case folding, so that a
# run gives the same report in every locale.
export LC_ALL=C

plugin_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
engine="${plugin_dir}/conformance.awk"
skills_dir="${plugin_dir}/skills"
manifest="${plugin_dir}/conformance/rules.tsv"
paraphrases="${plugin_dir}/conformance/removed-paraphrases.tsv"
usage="usage: conformance.sh [--verbose] [--skills <dir>] [--rules <file>] [--pack <dir>]... [--alone]"

usage_error() {
  echo "error: $*" >&2
  echo "${usage}" >&2
  exit 2
}

verbose=0
alone=0
core_overridden=0
packs=()
while (($#)); do
  case "$1" in
    --verbose | -v) verbose=1 ;;
    --rules)
      shift
      [[ $# -gt 0 && -f "$1" ]] || usage_error "--rules needs an existing manifest"
      manifest="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
      core_overridden=1
      ;;
    --skills)
      shift
      [[ $# -gt 0 ]] || usage_error "--skills needs a directory"
      skills_dir="$(cd "$1" 2>/dev/null && pwd)" || usage_error "--skills: no such directory: $1"
      core_overridden=1
      ;;
    --pack)
      shift
      [[ $# -gt 0 ]] || usage_error "--pack needs a source directory"
      pack="$(cd "$1" 2>/dev/null && pwd)" || usage_error "--pack: no such directory: $1"
      [[ -d "${pack}/skills" ]] || usage_error "--pack: ${pack} has no skills/ directory"
      [[ -f "${pack}/conformance/rules.tsv" ]] || usage_error "--pack: ${pack} has no conformance/rules.tsv"
      packs+=("${pack}")
      ;;
    --alone) alone=1 ;;
    -h | --help)
      awk 'NR == 1 { next } /^#/ { sub(/^# ?/, ""); print; next } { exit }' "${BASH_SOURCE[0]}"
      exit 0
      ;;
    *) usage_error "unknown argument: $1" ;;
  esac
  shift
done

if ((alone)); then
  ((${#packs[@]} == 1)) || usage_error "--alone checks exactly one --pack"
  ((core_overridden == 0)) || usage_error "--skills and --rules name core, which --alone does not read"
else
  [[ -d "${skills_dir}/grove" ]] || {
    echo "error: no spine skill at ${skills_dir}/grove" >&2
    exit 1
  }
  [[ -f "${manifest}" ]] || {
    echo "error: no rule manifest at ${manifest}" >&2
    exit 1
  }
fi

work="$(mktemp -d "${TMPDIR:-/tmp}/grove-conformance.XXXXXX")"
trap 'rm -rf "${work}"' EXIT

# print_pack <id> <label> <skills root> <manifest> <paraphrases>: the
# inventory records of one pack. Each directory under the skills root is a
# skill. Each `*.md` file in it, at any depth, is a file of that skill.
print_pack() {
  local id="$1" label="$2" root="$3" rules="$4" removed="$5" dir skill rel
  [[ -f "${removed}" ]] || removed=""
  printf 'P\t%s\t%s\t%s\t%s\t%s\n' "${id}" "${label}" "${root}" "${rules}" "${removed}"
  for dir in "${root}"/*/; do
    [[ -d "${dir}" ]] || continue
    dir="${dir%/}"
    skill="${dir##*/}"
    printf 'S\t%s\t%s\t%s\n' "${id}" "${skill}" "${dir}"
    while IFS= read -r rel; do
      printf 'F\t%s\t%s\t%s\t%s\n' "${id}" "${skill}" "${rel}" "${dir}/${rel}"
    done < <(cd "${dir}" && find . -name '*.md' \( -type f -o -type l \) | sed 's|^\./||' | sort)
  done
}

inventory="${work}/inventory.tsv"
{
  id=0
  if ((alone == 0)); then
    print_pack "${id}" "core" "${skills_dir}" "${manifest}" "${paraphrases}"
  fi
  for pack in ${packs[@]+"${packs[@]}"}; do
    id=$((id + 1))
    print_pack "${id}" "${pack##*/}" "${pack}/skills" "${pack}/conformance/rules.tsv" \
      "${pack}/conformance/removed-paraphrases.tsv"
  done
} >"${inventory}"

mode="aggregate"
((alone == 0)) || mode="alone"
status=0
awk -v mode="${mode}" -v verbose="${verbose}" -f "${engine}" "${inventory}" || status=$?
exit "${status}"

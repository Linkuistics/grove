#!/usr/bin/env bash
#
# Check each external pack that the marketplace pins against this working tree.
#
# `docs/specs/external-pack-contract.md`, "The checker without a copy", states
# this check. While Grove hosts the marketplace, a `git-subdir` entry of
# `.claude-plugin/marketplace.json` is Grove's record of a pack's revision. For
# each such entry, this script reads `url`, `path` and `sha`. It makes a plain
# Git checkout of `sha` under `target/external-packs/<plugin>/<sha>`, with Git's
# global and system configuration off. It reuses that checkout only when its
# `HEAD` is the commit and `git status --porcelain --ignored` prints nothing.
# It then runs this tree's conformance runner over the pack alone and over the
# aggregate of core and the pack.
#
# `docs/specs/pack-membership.md`, "The acceptance of each move", states one
# exception. A source directory with no kind skill (`skills/grove-<kind>/`) and
# no rules manifest (`conformance/rules.tsv`) ships craft skills only, or hooks
# only, as `technical-communication-hooks` does. Grove's conformance covers
# neither. For such a source, this script checks only that the commit holds
# the source directory. It reports that directory-only check by the source's
# name, and the source's own checks are its check. A source with a kind skill and no manifest still goes to the
# runner, which refuses it.
#
# An entry without `sha` fails, because Grove checks only exact revisions. A
# commit without the entry's `path` fails. The first run needs the network, and
# later runs reuse the cache.
#
# Usage: scripts/check-packs.sh [--marketplace <file>] [--cache <dir>]
#
#   --marketplace  the marketplace to read. The default is this tree's
#                  .claude-plugin/marketplace.json
#   --cache        the checkout cache. The default is target/external-packs
#
# The controls in scripts/check-packs.test.sh use both options.
#
# Exit status: 0 every pinned pack passes, 1 a pack fails or cannot be checked
# out, 2 bad usage.

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
marketplace="${repo_root}/.claude-plugin/marketplace.json"
cache="${repo_root}/target/external-packs"
runner="${repo_root}/plugins/grove/conformance.sh"

while (($#)); do
  case "$1" in
    --marketplace | --cache)
      if (($# < 2)); then
        echo "error: $1 needs a value" >&2
        exit 2
      fi
      if [[ "$1" == --marketplace ]]; then marketplace="$2"; else cache="$2"; fi
      shift 2
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      echo "usage: scripts/check-packs.sh [--marketplace <file>] [--cache <dir>]" >&2
      exit 2
      ;;
  esac
done

export GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 GIT_TERMINAL_PROMPT=0

# One tab-separated line for each git-subdir entry: name, url, path, sha. A
# missing field is an empty column, so the checks below can name it.
entries="$(python3 -I - "${marketplace}" <<'PY'
import json, sys
with open(sys.argv[1]) as handle:
    plugins = json.load(handle)["plugins"]
for plugin in plugins:
    source = plugin.get("source")
    if isinstance(source, dict) and source.get("source") == "git-subdir":
        fields = [plugin.get("name", ""), source.get("url", ""), source.get("path", ""), source.get("sha", "")]
        print("\t".join(fields))
PY
)"

if [[ -z "${entries}" ]]; then
  echo "check-packs: ${marketplace} pins no git-subdir pack"
  exit 0
fi

failed=()
checked=0
directory_only=()

# has_kind_skill <dir>: whether <dir>/skills holds a `grove-<kind>` skill.
has_kind_skill() {
  local kind
  for kind in "$1"/skills/grove-*/; do
    [[ -d "${kind}" ]] && return 0
  done
  return 1
}

# checkout <dir> <url> <sha>: a clean plain checkout of <sha> at <dir>.
checkout() {
  local dir="$1" url="$2" sha="$3" head
  if [[ -d "${dir}/.git" ]] &&
    [[ "$(git -C "${dir}" rev-parse HEAD 2>/dev/null)" == "${sha}" ]] &&
    [[ -z "$(git -C "${dir}" status --porcelain --ignored)" ]]; then
    echo "  reusing the clean checkout of ${sha}"
    return 0
  fi
  rm -rf "${dir}"
  mkdir -p "${dir}"
  git init --quiet "${dir}"
  git -C "${dir}" fetch --quiet --depth 1 "${url}" "${sha}" || return 1
  git -C "${dir}" checkout --quiet --detach FETCH_HEAD || return 1
  head="$(git -C "${dir}" rev-parse HEAD)"
  if [[ "${head}" != "${sha}" ]]; then
    echo "  error: the checkout is at ${head}, not ${sha}" >&2
    return 1
  fi
  echo "  checked out ${sha} from ${url}"
}

while IFS=$'\t' read -r name url path sha; do
  checked=$((checked + 1))
  echo "=== pack ${name}"
  if [[ -z "${url}" || -z "${path}" ]]; then
    echo "  error: the git-subdir entry of ${name} has no url or no path" >&2
    failed+=("${name}")
    continue
  fi
  if [[ ! "${sha}" =~ ^[0-9a-f]{40}$ ]]; then
    echo "  error: the git-subdir entry of ${name} pins no full commit in sha (${sha:-absent})" >&2
    failed+=("${name}")
    continue
  fi
  dir="${cache}/${name}/${sha}"
  if ! checkout "${dir}" "${url}" "${sha}"; then
    echo "  error: cannot check out ${sha} of ${url}" >&2
    failed+=("${name}")
    continue
  fi
  pack="${dir}/${path}"
  if [[ ! -d "${pack}" ]]; then
    echo "  error: ${sha} holds no source directory ${path}" >&2
    failed+=("${name}")
    continue
  fi
  if [[ ! -f "${pack}/conformance/rules.tsv" ]] && ! has_kind_skill "${pack}"; then
    echo "  directory-only check of ${name}: ${sha} holds ${path}, with no kind skill and no conformance/rules.tsv"
    echo "  Grove's conformance does not cover craft skills. The checks of ${name}'s own repository are its check."
    directory_only+=("${name}")
    continue
  fi
  if ! bash "${runner}" --pack "${pack}" --alone; then
    failed+=("${name} (alone)")
    continue
  fi
  if ! bash "${runner}" --pack "${pack}"; then
    failed+=("${name} (aggregate)")
  fi
done <<<"${entries}"

if ((${#failed[@]})); then
  echo "check-packs: FAILED — ${#failed[@]} of ${checked} pinned pack(s)"
  printf 'check-packs:   ✗ %s\n' "${failed[@]}"
  exit 1
fi
conformed=$((checked - ${#directory_only[@]}))
echo "check-packs: ${conformed} pinned pack(s) pass alone and in the aggregate"
if ((${#directory_only[@]})); then
  echo "check-packs: ${#directory_only[@]} pinned source(s) pass the directory-only check, with no conformance: ${directory_only[*]}"
fi

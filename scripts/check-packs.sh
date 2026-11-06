#!/usr/bin/env bash
#
# Check each pack that Grove's pack record names against this working tree.
#
# `docs/specs/external-pack-contract.md`, "The checker without a copy", states
# this check. `plugins/packs.json` is Grove's record of the accepted pack
# commits, and `scripts/pack-record.sh` reads it. For each member, this script
# makes a plain Git checkout of `sha` under
# `target/external-packs/<source-id>/<sha>`, with Git's global and system
# configuration off. It reuses that checkout only when its `HEAD` is the commit
# and `git status --porcelain --ignored` prints nothing. It then runs this
# tree's conformance runner over the pack alone and over the aggregate of core
# and the pack. The release build copies each source directory from the same
# checkout.
#
# The record names the five domain packs only. Each ships kind skills and a
# rules manifest, so every member goes to the runner. A source with no kind
# skill and no manifest is not a member, and the runner refuses it.
#
# A member without `sha` fails, because Grove checks only exact revisions. A
# key outside the source-ID grammar fails, and so does a commit without the
# member's `path`. A missing, malformed or empty record fails. The first run
# needs the network, and later runs reuse the cache.
#
# Usage: scripts/check-packs.sh [--record <file>] [--cache <dir>]
#
#   --record  the pack record to read. The default is this tree's
#             plugins/packs.json
#   --cache   the checkout cache. The default is target/external-packs
#
# The controls in scripts/check-packs.test.sh use both options.
#
# Exit status: 0 every recorded pack passes, 1 a pack fails or cannot be checked
# out, or the record is unusable, 2 bad usage.

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
record="${repo_root}/plugins/packs.json"
cache="${repo_root}/target/external-packs"
runner="${repo_root}/plugins/grove/conformance.sh"

# shellcheck source=scripts/pack-record.sh
source "${repo_root}/scripts/pack-record.sh"

while (($#)); do
  case "$1" in
    --record | --cache)
      if (($# < 2)); then
        echo "error: $1 needs a value" >&2
        exit 2
      fi
      if [[ "$1" == --record ]]; then record="$2"; else cache="$2"; fi
      shift 2
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      echo "usage: scripts/check-packs.sh [--record <file>] [--cache <dir>]" >&2
      exit 2
      ;;
  esac
done

if ! members="$(pack_record_members "${record}")"; then
  echo "check-packs: FAILED — the pack record ${record} cannot be read"
  exit 1
fi

if [[ -z "${members}" ]]; then
  echo "error: the pack record ${record} names no pack" >&2
  echo "check-packs: FAILED — nothing checked is not every pack checked"
  exit 1
fi

failed=()
checked=0

while IFS=$'\t' read -r status id url path sha; do
  checked=$((checked + 1))
  echo "=== pack ${id}"
  if [[ "${status}" != ok ]]; then
    # A bad line holds the reason in the third column.
    echo "  error: ${url}" >&2
    failed+=("${id}")
    continue
  fi
  dir="$(pack_checkout_dir "${cache}" "${id}" "${sha}")"
  if ! pack_checkout "${dir}" "${url}" "${sha}"; then
    echo "  error: cannot check out ${sha} of ${url}" >&2
    failed+=("${id}")
    continue
  fi
  pack="${dir}/${path}"
  if [[ ! -d "${pack}" ]]; then
    echo "  error: ${sha} holds no source directory ${path}" >&2
    failed+=("${id}")
    continue
  fi
  if ! bash "${runner}" --pack "${pack}" --alone; then
    failed+=("${id} (alone)")
    continue
  fi
  if ! bash "${runner}" --pack "${pack}"; then
    failed+=("${id} (aggregate)")
  fi
done <<<"${members}"

if ((${#failed[@]})); then
  echo "check-packs: FAILED — ${#failed[@]} of ${checked} recorded pack(s)"
  printf 'check-packs:   ✗ %s\n' "${failed[@]}"
  exit 1
fi
echo "check-packs: ${checked} recorded pack(s) pass alone and in the aggregate"

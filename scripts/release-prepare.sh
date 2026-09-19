#!/usr/bin/env bash
# Prepare main and its changelog without incorporating unrelated working-copy work.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

die() {
  echo "release-prepare: $*" >&2
  exit 1
}

main() {
  cd "$repo_root"
  [[ -d .jj && -d .git && "$(pwd -P)" == "$(jj workspace root --name default)" ]] \
    || die "run from the default colocated jj workspace"
  local saved unmerged conflicts undescribed version tag changes
  saved="$(jj log --no-graph -r @ -T change_id)"
  jj status
  jj git fetch --remote origin
  unmerged="$(jj log --no-graph -r 'main@origin ~ ancestors(main)' -T commit_id)"
  [[ -z "$unmerged" ]] || die "main diverges from origin; integrate main@origin before releasing"
  conflicts="$(jj log --no-graph -r 'conflicts() & ancestors(main)' -T commit_id)"
  [[ -z "$conflicts" ]] || die "main contains conflicts; resolve them before releasing"
  undescribed="$(jj log --no-graph -r 'main@origin..main' -T 'if(description.first_line() == "", commit_id ++ "\n")')"
  [[ -z "$undescribed" ]] || die "main has undescribed changes; describe them before releasing"

  # Snapshotting and changing parents preserves every other line of work in jj.
  echo "release-prepare: saved working change $saved (return with: jj edit $saved)"
  jj new main
  version="$(cargo metadata --locked --no-deps --format-version 1 | jq -er '.packages[] | select(.name == "grove") | .version')"
  [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable Grove version, got $version"
  tag="v$version"
  git rev-parse --verify "refs/tags/$tag" >/dev/null \
    || die "missing previous release tag $tag; inspect the release history"
  changes="$(jj log --no-graph -r "$tag..main" -T commit_id)"
  [[ -n "$changes" ]] || die "No changes since $tag; resume any unfinished release using docs/RELEASING.md"
  awk '/^## Unreleased$/ { n++ } END { exit n != 1 }' CHANGELOG.md \
    || die "CHANGELOG.md must contain exactly one Unreleased heading"
  if awk '/^## Unreleased$/ { section=1; next }
      section && /^## / { exit }
      section && /[^[:space:]]/ { found=1 }
      END { exit !found }' CHANGELOG.md; then
    echo 'release-prepare: preserving existing Unreleased notes'
    return
  fi

  # Generation is shared with `task release:notes`; only the endpoint differs.
  bash scripts/release-notes.sh --to main
  jj describe -m 'docs: prepare release notes'
  jj bookmark set main -r @
  jj new main
  echo 'release-prepare: generated and committed release notes through grove run release-notes'
}

main "$@"

#!/usr/bin/env bash
# Refresh CHANGELOG.md's Unreleased section from every change since the last release.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
notes_dir=''
trap '[[ -z "$notes_dir" ]] || rm -rf "$notes_dir"' EXIT

die() {
  echo "release-notes: $*" >&2
  exit 1
}

usage() {
  cat <<'USAGE'
Usage: scripts/release-notes.sh [--to <revset>]

Regenerate the Unreleased section of the working copy's CHANGELOG.md through
`grove run release-notes`. Evidence covers every change from the current
version's release tag to the endpoint: full commit descriptions, the complete
source diff, the existing Unreleased text and the previous release's changelog.
Versioned sections are left alone. Nothing is fetched, committed or moved.

Options:
  --to <revset>  Endpoint revision (default: @, snapshotted when the run starts)
  -h, --help     Print this help

Environment:
  GROVE_RELEASE_RUNTIME_READ  Runtime files granted read-only, one path per line

Examples:
  task release:notes
  scripts/release-notes.sh --to main

Exit codes: 0 Unreleased refreshed, 1 failure with CHANGELOG.md unchanged, 2 invalid usage.
USAGE
}

# Print the Unreleased section body of the changelog on stdin.
unreleased_body() {
  awk '/^## Unreleased$/ { section=1; next }
    section && /^## / { exit }
    section { print }'
}

main() {
  local to='@'
  while (($#)); do
    case "$1" in
      --to)
        (($# >= 2)) || { usage >&2; exit 2; }
        to="$2"
        shift 2
        ;;
      -h | --help)
        usage
        return
        ;;
      *)
        usage >&2
        exit 2
        ;;
    esac
  done

  cd "$repo_root"
  [[ -d .jj ]] || die "run from a jj workspace"
  # Resolving the endpoint once fixes the evidence for the whole run.
  local rev version tag
  rev="$(jj log --no-graph -r "$to" -T 'commit_id ++ "\n"')" \
    || die "cannot resolve endpoint $to"
  [[ "$rev" =~ ^[0-9a-f]+$ ]] || die "endpoint $to must name exactly one revision"
  version="$(cargo metadata --locked --no-deps --format-version 1 | jq -er '.packages[] | select(.name == "grove") | .version')"
  [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || die "expected a stable Grove version, got $version"
  tag="v$version"
  [[ -n "$(jj log --no-graph -r "tags(exact:\"$tag\") & ::$rev" -T commit_id)" ]] \
    || die "missing baseline: release tag $tag is not an ancestor of $to; inspect the release history"

  mkdir -p target
  notes_dir="$(mktemp -d "$repo_root/target/release-notes.XXXXXX")"
  jj log --no-graph --reversed -r "$tag..$rev" \
    -T 'if(!empty, "Commit " ++ commit_id ++ "\n" ++ description ++ "\n")' >"$notes_dir/changes.txt"
  [[ -s "$notes_dir/changes.txt" ]] || die "no changes to describe since $tag"
  jj file show -r "$rev" 'root:CHANGELOG.md' >"$notes_dir/changelog-at-endpoint"
  awk '/^## Unreleased$/ { n++ } END { exit n != 1 }' "$notes_dir/changelog-at-endpoint" \
    || die "CHANGELOG.md must contain exactly one Unreleased heading"
  unreleased_body <"$notes_dir/changelog-at-endpoint" >"$notes_dir/current-unreleased.md"
  jj file show -r "$tag" 'root:CHANGELOG.md' >"$notes_dir/previous-changelog.md"
  jj diff --from "$tag" --to "$rev" --git >"$notes_dir/changes.diff"
  # The writing instructions are staged with the invocation, never discovered.
  # Headless helpers beside it are staged too, so personal policy can name one
  # without a path into this checkout.
  cp scripts/release-notes/SKILL.md scripts/release-notes/codex-headless.sh "$notes_dir/"
  cat >"$notes_dir/prompt.md" <<'PROMPT'
Read SKILL.md in this directory and follow it exactly to write release-notes.md
from the files staged beside it.
PROMPT
  local -a run_args=(run release-notes --prompt-file "$notes_dir/prompt.md"
    --input "$notes_dir/SKILL.md" --input "$notes_dir/previous-changelog.md"
    --input "$notes_dir/current-unreleased.md" --input "$notes_dir/changes.txt"
    --input "$notes_dir/changes.diff" --input "$notes_dir/codex-headless.sh"
    --output "$notes_dir/release-notes.md" --ui auto)
  local runtime_read
  while IFS= read -r runtime_read; do
    [[ -z "$runtime_read" ]] || run_args+=(--runtime-read "$runtime_read")
  done <<<"${GROVE_RELEASE_RUNTIME_READ:-}"

  cargo build --locked -p grove -p grove-llm
  ./target/debug/grove "${run_args[@]}"
  if [[ ! -f "$notes_dir/release-notes.md" ]] \
    || ! awk '/[^[:space:]]/ { found=1 } END { exit !found }' "$notes_dir/release-notes.md"; then
    die "release-notes returned an empty Markdown body"
  fi
  awk '/^[[:blank:]]*##([[:space:]]|$)/ { bad=1 } END { exit bad }' "$notes_dir/release-notes.md" \
    || die "release-notes returned a forbidden ## heading; return only the section body"
  cmp -s "$notes_dir/changelog-at-endpoint" CHANGELOG.md \
    || die "CHANGELOG.md differs from the endpoint's; it was left unchanged"
  # Outer blank lines are dropped so repeated refreshes keep one stable spacing.
  awk -v notes="$notes_dir/release-notes.md" '
    function emit(   line, pending, started) {
      print ""
      while ((getline line < notes) > 0) {
        if (line ~ /^[[:space:]]*$/) { pending++; continue }
        if (started) while (pending-- > 0) print ""
        pending = 0
        started = 1
        print line
      }
      close(notes)
    }
    /^## Unreleased$/ { print; emit(); section=1; next }
    section && /^## / { section=0; print "" }
    !section { print }' CHANGELOG.md >"$notes_dir/changelog"
  mv "$notes_dir/changelog" CHANGELOG.md
  echo "release-notes: refreshed Unreleased from changes in $tag..$to through grove run release-notes"
}

main "$@"

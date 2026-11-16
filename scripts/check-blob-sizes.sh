#!/usr/bin/env bash
#
# Blob-size checks against the publishing host's per-file limit.
#
#   check-blob-sizes.sh tip [REPO]          no tracked file in the working-copy commit
#                                           exceeds 50 MiB
#   check-blob-sizes.sh publication [REPO]  no blob a push to `origin` could publish exceeds
#                                           100 MiB, and each one above 50 MiB is reported
#
# WHY. GitHub refuses a pushed blob above 100 MiB and warns above 50 MiB
# (https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github).
# The limit applies to every blob in every pushed commit, not to the tip, so a small tip says
# nothing about the history behind it. docs/adr/oversized-evidence-stays-out-of-main.md
# records the decision and why the tip alone is not the thing to check.
#
# WHAT `publication` COVERS. The commits a push to `origin` could publish: the working copy's
# ancestry less what `origin`'s bookmarks already hold (`::@ ~ ::remote_bookmarks(remote=
# exact:"origin")`), less the root commit, which has no tree. That leaves out `main@skills`,
# which belongs to another remote. Sizes are read from jj's backing Git repository with
# `git ls-tree -r -l`, because jj's templates expose no blob size. That reads and writes
# nothing, so the rule that commits go through jj still holds.
#
# WHAT IT DOES NOT DO. It is a check a session runs, not a server-side gate, so GitHub's own
# refusal stays the backstop. `tip` also cannot see a new file jj left out of the commit: jj
# warns and skips a *new* file above `snapshot.max-new-file-size` (1 MiB by default), so such a
# file is in no commit for either mode to measure.
#
# `tip` is run by scripts/check.sh. `publication` is run by `task publication:history:check`
# and is not part of check.sh.

set -euo pipefail
IFS=$'\n\t'

readonly WARN_BYTES=52428800   # 50 MiB
readonly FAIL_BYTES=104857600  # 100 MiB

usage() {
  echo "usage: ${0##*/} tip|publication [REPO]" >&2
  exit 2
}

[[ $# -ge 1 && $# -le 2 ]] || usage
mode="$1"
repo="${2:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
[[ "$mode" == tip || "$mode" == publication ]] || usage

git_dir="$(jj -R "$repo" git root)"
readonly git_dir

# Print "<size>\t<path>" for each blob in a commit's tree above a limit. The size column of
# `git ls-tree -l` is the last space-separated field before the tab; a submodule has "-".
oversized() {
  local commit="$1" limit="$2"
  git --git-dir="$git_dir" -c core.quotePath=false ls-tree -r -l "$commit" |
    awk -F'\t' -v limit="$limit" '
      {
        n = split($1, meta, " ")
        if (meta[n] ~ /^[0-9]+$/ && meta[n] + 0 > limit) print meta[n] "\t" $2
      }'
}

mib() {
  awk -v bytes="$1" 'BEGIN { printf "%.1f MiB", bytes / 1048576 }'
}

if [[ "$mode" == tip ]]; then
  commit="$(jj -R "$repo" log --no-graph -r @ -T commit_id)"
  # Captured before it is read, so that a failing `git ls-tree` stops the script instead of
  # reading as an empty, passing listing.
  listing="$(oversized "$commit" "$WARN_BYTES")"
  found=0
  while IFS=$'\t' read -r size path; do
    [[ -n "$size" ]] || continue
    found=$((found + 1))
    echo "tip: $path is $size bytes ($(mib "$size")), above $WARN_BYTES" >&2
  done <<<"$listing"
  if ((found > 0)); then
    echo "tip: $found tracked file(s) above 50 MiB in the working-copy commit" >&2
    exit 1
  fi
  echo "tip: no tracked file in the working-copy commit exceeds 50 MiB"
  exit 0
fi

revset='::@ ~ ::remote_bookmarks(remote=exact:"origin") ~ root()'
commits=()
history="$(jj -R "$repo" log --no-graph -r "$revset" -T 'commit_id ++ " " ++ change_id ++ "\n"')"
while IFS= read -r line; do
  [[ -n "$line" ]] && commits+=("$line")
done <<<"$history"

failed=0
warned=0
for line in "${commits[@]}"; do
  IFS=' ' read -r commit change <<<"$line"
  listing="$(oversized "$commit" "$WARN_BYTES")"
  while IFS=$'\t' read -r size path; do
    [[ -n "$size" ]] || continue
    if ((size > FAIL_BYTES)); then
      failed=$((failed + 1))
      echo "FAIL $size bytes ($(mib "$size")) in $path at commit $commit, change $change" >&2
    else
      warned=$((warned + 1))
      echo "WARN $size bytes ($(mib "$size")) in $path at commit $commit, change $change" >&2
    fi
  done <<<"$listing"
done

echo "publication: ${#commits[@]} commit(s) a push to origin could publish;" \
  "$failed blob(s) above 100 MiB, $warned between 50 and 100 MiB"
if ((failed > 0)); then
  echo "publication: GitHub would refuse this push; repair the history first" >&2
  exit 1
fi

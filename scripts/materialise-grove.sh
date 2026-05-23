#!/usr/bin/env bash
# materialise-grove — copy the grove skill into a target repo's .claude/skills/
# and stamp VERSION.md. Run from anywhere inside a Linkuistics/skills clone.
#
#   materialise-grove.sh <target-repo> [<ref>]
#
#   <target-repo>  path to the consuming project's repo root
#   <ref>          Linkuistics/skills commit/branch/tag to materialise
#                  (default: HEAD)
#
# Updating grove later is the same command again; the diff is plain files,
# which the consuming project reviews and commits.
set -euo pipefail

die() { echo "materialise-grove: $*" >&2; exit 1; }

target="${1:-}"
ref="${2:-HEAD}"
[[ -n "$target" ]] || die "usage: materialise-grove.sh <target-repo> [<ref>]"
[[ -e "$target/.git" ]] || die "not a git repo: $target"

# The Linkuistics/skills clone this script lives in.
src_repo="$(git -C "$(dirname "${BASH_SOURCE[0]}")" rev-parse --show-toplevel)"
grove_path="plugins/linkuistics/skills/grove"

git -C "$src_repo" rev-parse -q --verify "${ref}^{commit}" >/dev/null \
  || die "unknown ref in $src_repo: $ref"
git -C "$src_repo" ls-tree "$ref" -- "$grove_path" | grep -q . \
  || die "$grove_path not found at $ref"

skills_sha="$(git -C "$src_repo" rev-parse --short "$ref")"
dest="$target/.claude/skills/grove"

# Extract grove/ at the chosen ref straight into the target — no working-tree
# churn in the source clone. strip-components=4 drops the four leading path
# components plugins/linkuistics/skills/grove.
rm -rf "$dest"
mkdir -p "$dest"
git -C "$src_repo" archive "$ref" -- "$grove_path" \
  | tar -x -C "$dest" --strip-components=4

# The mattpocock/skills source commit is recorded in the bundled file headers
# (single source of truth — see Task 5).
matt_sha="$(grep -ohm1 'mattpocock/skills@[0-9a-f]\{7,\}' "$dest"/*.md \
            | head -1 | cut -d@ -f2 || true)"
[[ -n "$matt_sha" ]] || matt_sha="(not recorded)"

cat > "$dest/VERSION.md" <<EOF
# grove — materialised version

A materialised copy of the \`grove\` skill: plain files committed in this repo.
This file records where the copy came from and how to refresh it.

| | |
|---|---|
| grove source | \`Linkuistics/skills@${skills_sha}\` |
| bundled conventions | \`mattpocock/skills@${matt_sha}\` |
| materialised on | $(date +%Y-%m-%d) |
| materialised into | \`.claude/skills/grove/\` |

## Updating

From a \`Linkuistics/skills\` clone, checked out at the ref you want to pin:

\`\`\`
scripts/materialise-grove.sh <path-to-this-repo> [<ref>]
\`\`\`

Review the resulting diff and commit it. By discipline, record the bump in an
ADR (\`docs/adr/\`).
EOF

echo "materialise-grove: wrote $dest (Linkuistics/skills@${skills_sha})"

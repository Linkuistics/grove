#!/usr/bin/env bash
# Test for materialise-grove.sh — materialises grove into a throwaway repo
# and asserts the footprint and VERSION.md stamp.
set -euo pipefail
fail() { echo "FAIL: $*" >&2; exit 1; }

src_repo="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# A throwaway target repo.
git -C "$tmp" init -q
git -C "$tmp" config user.email t@t
git -C "$tmp" config user.name t
git -C "$tmp" commit -q --allow-empty -m init

"$src_repo/scripts/materialise-grove.sh" "$tmp"

dest="$tmp/.claude/skills/grove"
for f in SKILL.md BRIEF-FORMAT.md TASK-FORMAT.md CONTEXT-FORMAT.md \
         ADR-FORMAT.md grilling.md VERSION.md; do
  [[ -f "$dest/$f" ]] || fail "missing $f"
done
[[ -f "$dest/LICENSES/mattpocock-skills.LICENSE" ]] || fail "missing bundled LICENSE"

grep -q 'Linkuistics/skills@'  "$dest/VERSION.md" || fail "VERSION.md missing grove source sha"
grep -q 'mattpocock/skills@'   "$dest/VERSION.md" || fail "VERSION.md missing mattpocock sha"

diff -q "$src_repo/grove/SKILL.md" "$dest/SKILL.md" >/dev/null \
  || fail "materialised SKILL.md differs from source"

# Re-running (the update path) must succeed and stay clean.
"$src_repo/scripts/materialise-grove.sh" "$tmp"
[[ -f "$dest/SKILL.md" ]] || fail "re-run lost SKILL.md"

echo "PASS"

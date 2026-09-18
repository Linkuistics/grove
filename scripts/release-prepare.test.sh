#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
export JJ_CONFIG="$scratch/jj.toml"
printf '[user]\nname="Release Test"\nemail="release@example.invalid"\n' >"$JJ_CONFIG"
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'

git init -q --bare "$scratch/remote.git"
jj git init --colocate "$scratch/repo" >/dev/null
mkdir -p "$scratch/repo/src" "$scratch/repo/scripts"
cp "$repo_root/scripts/release-prepare.sh" "$scratch/repo/scripts/"
printf '/target/\n' >"$scratch/repo/.gitignore"
printf '[package]\nname="grove"\nversion="1.2.3"\nedition="2021"\n' >"$scratch/repo/Cargo.toml"
printf 'fn main() {}\n' >"$scratch/repo/src/main.rs"
printf '# Changelog\n\n## Unreleased\n\n## v1.2.3\n\n- Existing notes.\n' >"$scratch/repo/CHANGELOG.md"
cargo generate-lockfile --manifest-path "$scratch/repo/Cargo.toml" --offline
jj -R "$scratch/repo" describe -m 'Initial release' >/dev/null
jj -R "$scratch/repo" git remote add origin "$scratch/remote.git"
jj -R "$scratch/repo" git push --named main=@ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
git -C "$scratch/repo" tag v1.2.3
printf 'fn main() { println!("improved"); }\n' >"$scratch/repo/src/main.rs"
jj -R "$scratch/repo" describe -m 'Improve the fixture' >/dev/null
jj -R "$scratch/repo" bookmark set main -r @ >/dev/null
jj -R "$scratch/repo" new main >/dev/null
mkdir "$scratch/repo/.grove"
printf 'Unrelated work\n' >"$scratch/repo/.grove/_BRIEF.md"
saved="$(jj -R "$scratch/repo" log --no-graph -r @ -T change_id)"

# Missing notes are generated from released changes, while unrelated work stays saved.
(cd "$scratch/repo" && bash scripts/release-prepare.sh)
grep -Fxq -- '- Improve the fixture' "$scratch/repo/CHANGELOG.md"
grep -Fxq -- '- Existing notes.' "$scratch/repo/CHANGELOG.md"
[[ ! -e "$scratch/repo/.grove" ]]
[[ "$(jj -R "$scratch/repo" file show -r "$saved" 'root:.grove/_BRIEF.md')" == 'Unrelated work' ]]
[[ -z "$(git -C "$scratch/repo" status --porcelain)" ]]

# Already prepared notes are left byte-for-byte intact without another commit.
before="$(git -C "$scratch/repo" rev-parse HEAD)"
cp "$scratch/repo/CHANGELOG.md" "$scratch/notes"
(cd "$scratch/repo" && bash scripts/release-prepare.sh)
cmp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
[[ "$(git -C "$scratch/repo" rev-parse HEAD)" == "$before" ]]

# Once main itself is tagged, preparation must refuse to cut it again.
git -C "$scratch/repo" tag -f v1.2.3
if (cd "$scratch/repo" && bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then
  echo 'FAIL: preparation accepted an already tagged main' >&2
  exit 1
fi
grep -Fq 'No changes since v1.2.3' "$scratch/output"
echo 'release preparation tests: all passed'

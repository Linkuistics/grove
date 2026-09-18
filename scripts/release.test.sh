#!/usr/bin/env bash
# Local release fixtures; GitHub is the only publishing service replaced.
set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT
export JJ_CONFIG="$scratch/jj.toml"
printf '[user]\nname = "Release Test"\nemail = "release@example.invalid"\n' >"$JJ_CONFIG"
export GIT_AUTHOR_NAME='Release Test' GIT_COMMITTER_NAME='Release Test'
export GIT_AUTHOR_EMAIL='release@example.invalid' GIT_COMMITTER_EMAIL='release@example.invalid'
real_git="$(command -v git)"
export RELEASE_TEST_GIT="$real_git" RELEASE_TEST_LOG="$scratch/published"
mkdir -p "$scratch/bin" "$scratch/source/scripts" "$scratch/source/target/dist"
cat >"$scratch/bin/gh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == 'release create' ]]; then
  printf 'published\n' >>"$RELEASE_TEST_LOG"
fi
SH
cat >"$scratch/bin/git" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
# Reject raw Git mutations in the jj tap, as the repository contract does.
if [[ "${1:-}" == '-C' && -d "$2/.jj" ]]; then
  case "${3:-}" in add|commit|push) exit 88 ;; esac
fi
exec "$RELEASE_TEST_GIT" "$@"
SH
chmod +x "$scratch/bin/gh" "$scratch/bin/git"
export PATH="$scratch/bin:$PATH"

cp "$repo_root/scripts/release-publish.sh" "$scratch/source/scripts/"
"$real_git" init -q "$scratch/source"
"$real_git" -C "$scratch/source" add .
"$real_git" -C "$scratch/source" commit -qm fixture
"$real_git" -C "$scratch/source" tag v1.2.3
printf 'new formula\n' >"$scratch/source/target/dist/grove.rb"
for target in aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu; do
  touch "$scratch/source/target/dist/grove-v1.2.3-$target.tar.xz"
done

"$real_git" init -q --bare "$scratch/remote.git"
jj git init --colocate "$scratch/tap" >/dev/null
mkdir -p "$scratch/tap/Formula"
printf 'old formula\n' >"$scratch/tap/Formula/grove.rb"
jj -R "$scratch/tap" describe -m fixture >/dev/null
jj -R "$scratch/tap" git remote add origin "$scratch/remote.git"
jj -R "$scratch/tap" git push --named main=@ >/dev/null
jj -R "$scratch/tap" new main >/dev/null
export GROVE_TAP_DIR="$scratch/tap"

# A dirty tap must be refused before creating a GitHub Release or copying files.
printf 'unrelated work\n' >"$scratch/tap/notes.txt"
if bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: publishing accepted a dirty tap' >&2
  exit 1
fi
if [[ -e "$RELEASE_TEST_LOG" ]]; then
  echo 'FAIL: dirty tap was detected only after GitHub publication' >&2
  exit 1
fi
[[ "$(cat "$scratch/tap/Formula/grove.rb")" == 'old formula' ]]
[[ "$(cat "$scratch/tap/notes.txt")" == 'unrelated work' ]]
rm "$scratch/tap/notes.txt"

# A jj tap must publish through jj, update the intended bookmark, and end clean.
bash "$scratch/source/scripts/release-publish.sh"
[[ "$("$real_git" --git-dir="$scratch/remote.git" show main:Formula/grove.rb)" == 'new formula' ]]
[[ -z "$("$real_git" -C "$scratch/tap" status --porcelain)" ]]
[[ "$(cat "$RELEASE_TEST_LOG")" == 'published' ]]

# Failed prerequisites stop the real Task pipeline before cutting a version.
mkdir -p "$scratch/task/scripts"
cp "$repo_root/Taskfile.yml" "$scratch/task/"
printf '## Unreleased\n\n- Fixture change.\n' >"$scratch/task/CHANGELOG.md"
cat >"$scratch/task/scripts/release-doctor.sh" <<'SH'
#!/usr/bin/env bash
exit "${RELEASE_TEST_DOCTOR_FAIL:-0}"
SH
chmod +x "$scratch/task/scripts/release-doctor.sh"
jj git init --colocate "$scratch/task" >/dev/null
jj -R "$scratch/task" describe -m fixture >/dev/null
"$real_git" init -q --bare "$scratch/task-remote.git"
jj -R "$scratch/task" git remote add origin "$scratch/task-remote.git"
jj -R "$scratch/task" git push --named main=@ >/dev/null
jj -R "$scratch/task" new main >/dev/null
before="$("$real_git" -C "$scratch/task" rev-parse HEAD)"

# Task's dry run checks preconditions but must never execute a release command.
for level in patch minor major; do
  task --dir "$scratch/task" --dry "release:$level" >"$scratch/dry" 2>&1
  grep -Fq "cargo release $level --execute --no-confirm" "$scratch/dry"
done
[[ "$(cat "$RELEASE_TEST_LOG")" == 'published' ]]

if RELEASE_TEST_DOCTOR_FAIL=73 task --dir "$scratch/task" release:patch >"$scratch/output" 2>&1; then
  echo 'FAIL: release continued after failed prerequisites' >&2
  exit 1
fi
grep -Fq 'exit status 73' "$scratch/output"
[[ "$("$real_git" -C "$scratch/task" rev-parse HEAD)" == "$before" ]]
[[ -z "$("$real_git" -C "$scratch/task" tag --list)" ]]

# A release that arrived during fetch must not reuse the old Unreleased notes.
"$real_git" clone -q --branch main "$scratch/task-remote.git" "$scratch/other"
printf '## Unreleased\n' >"$scratch/other/CHANGELOG.md"
"$real_git" -C "$scratch/other" add CHANGELOG.md
"$real_git" -C "$scratch/other" commit -qm 'another release'
"$real_git" -C "$scratch/other" push -q origin main
if task --dir "$scratch/task" release:patch >"$scratch/output" 2>&1; then
  echo 'FAIL: release continued after main advanced during fetch' >&2
  exit 1
fi
grep -Fq 'main advanced during fetch' "$scratch/output"
[[ "$("$real_git" -C "$scratch/task" rev-parse HEAD)" == "$before" ]]
[[ -z "$("$real_git" -C "$scratch/task" tag --list)" ]]
echo 'release tests: all passed'

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
export RELEASE_TEST_NOTES="$scratch/release-notes"
mkdir -p "$scratch/bin" "$scratch/source/scripts" "$scratch/source/target/dist"
cat >"$scratch/bin/gh" <<'SH'
#!/usr/bin/env bash
set -euo pipefail
if [[ "$1 $2" == 'release create' ]]; then
  while (($#)); do
    if [[ "$1" == '--notes-file' ]]; then
      shift
      cat "$1" >"$RELEASE_TEST_NOTES"
    fi
    shift
  done
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

# Release archive contents. release-build.sh packs every archive with
# release-common.sh's pack_archive and checks it against the manifest there;
# these cases show that check fires on each way an archive can be wrong, and
# that the manifest agrees with Grove's caller assets and the formula before
# any version is cut.
# shellcheck source=scripts/release-common.sh
source "$repo_root/scripts/release-common.sh"
fail() {
  echo "FAIL: $*" >&2
  exit 1
}
# Stage placeholder files in the manifest's layout under $scratch/archive/stage.
stage_archive() {
  local top="$1" entry
  rm -rf "$scratch/archive"
  mkdir -p "$scratch/archive/stage/$top"
  while IFS= read -r entry; do
    mkdir -p "$(dirname "$scratch/archive/stage/$top/$entry")"
    printf 'fixture %s\n' "$entry" >"$scratch/archive/stage/$top/$entry"
  done < <(archive_manifest)
  while IFS= read -r entry; do
    chmod +x "$scratch/archive/stage/$top/$entry"
  done < <(archive_executables)
}
pack_fixture() {
  pack_archive "$scratch/archive/stage" "$1" "$scratch/archive/$1.tar.xz"
}
# The check must refuse the archive, reporting exactly the line given.
refused_archive() {
  local top="$1" expected="$scratch/archive/$1.tar.xz: $2"
  if assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output"; then
    fail "an archive with '$2' passed the manifest check"
  fi
  grep -Fxq -- "$expected" "$scratch/archive/output" ||
    fail "the manifest check did not report '$2': $(cat "$scratch/archive/output")"
}

# Caller assets ship independently of Ravel's SDK and runtime.
for required in bin/grove bin/grove-llm libexec/ravel-policy/grove.ts \
  libexec/ravel-policy/provenance.ts libexec/ravel-policy/examples/grove-review.ts \
  libexec/ravel-policy/examples/grove-static.ts libexec/ravel-policy/examples/review.ts \
  libexec/ravel-policy/sample/policy.ts libexec/ravel-policy/README.md \
  libexec/ravel-policy/typecheck/review.ts libexec/ravel-policy/typecheck/static.ts; do
  archive_manifest | grep -Fxq -- "$required" || fail "the archive manifest omits $required"
done
archive_executables | while IFS= read -r entry; do
  archive_manifest | grep -Fxq -- "$entry" || fail "executable $entry is not in the archive manifest"
done
(cd "$repo_root/assets/ravel-policy" && find . ! -type d | sed 's|^\./||' | LC_ALL=C sort) >"$scratch/assets-source"
archive_manifest | sed -n 's|^libexec/ravel-policy/||p' | LC_ALL=C sort >"$scratch/assets-manifest"
diff -u "$scratch/assets-source" "$scratch/assets-manifest" >&2 ||
  fail "the archive manifest differs from the complete caller asset tree"

template="$repo_root/scripts/templates/grove.rb.tmpl"
formula_bins="$(sed -n 's/^ *bin\.install //p' "$template" | tr -d '"' | tr ',' '\n' | tr -d ' ' | LC_ALL=C sort)"
[[ "$formula_bins" == "$(archive_manifest | grep '^bin/' | LC_ALL=C sort)" ]] ||
  fail "the formula's bin.install ($formula_bins) differs from the manifest's bin/ entries"
grep -Fxq '    libexec.install "libexec/ravel-policy"' "$template" ||
  fail 'the formula does not install libexec/ravel-policy'
grep -Fxq '  depends_on "linkuistics/taps/ravel"' "$template" ||
  fail 'the formula does not depend on independent Ravel'
if archive_manifest | grep -v -e '^bin/' -e '^libexec/ravel-policy/' | grep -q /; then
  fail 'the manifest ships a directory the formula does not install'
fi

for target in "${TARGETS[@]}"; do
  top="grove-v1.2.3-$target"
  stage_archive "$top"
  pack_fixture "$top"
  assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" ||
    fail "a complete $target archive failed the manifest check: $(cat "$scratch/archive/output")"
  [[ ! -s "$scratch/archive/output" ]] || fail 'a passing manifest check printed output'
done

# Positive controls: each file's omission is seen to fail, by name.
top="grove-v1.2.3-${TARGETS[0]}"
while IFS= read -r entry; do
  stage_archive "$top"
  rm "$scratch/archive/stage/$top/$entry"
  pack_fixture "$top"
  refused_archive "$top" "missing $top/$entry"
done < <(archive_manifest)

# Runtime, SDK, probes and notices from another project must never ship.
for extra in bin/harness-dispatch bin/ravel libexec/ravel/ravel-policy \
  libexec/harness-dispatch/harness-dispatch-policy libexec/ravel/sdk/index.ts \
  libexec/ravel-policy/node_modules/cache libexec/ravel-policy/notices/NOTICES.md; do
  stage_archive "$top"
  mkdir -p "$(dirname "$scratch/archive/stage/$top/$extra")"
  touch "$scratch/archive/stage/$top/$extra"
  pack_fixture "$top"
  refused_archive "$top" "unexpected $top/$extra"
done

stage_archive "$top"
chmod -x "$scratch/archive/stage/$top/bin/grove-llm"
pack_fixture "$top"
refused_archive "$top" "$top/bin/grove-llm is not executable"

stage_archive "$top"
ln -sf ../grove "$scratch/archive/stage/$top/bin/grove-llm"
pack_fixture "$top"
refused_archive "$top" "$top/bin/grove-llm is not a regular file"

stage_archive "$top"
mkdir "$scratch/archive/stage/stray"
COPYFILE_DISABLE=1 tar -C "$scratch/archive/stage" --no-xattrs -cJf "$scratch/archive/$top.tar.xz" "$top" stray
refused_archive "$top" "top level is not exactly $top/:"

# A file with an extended attribute, as macOS marks most staged files. Packed
# as bsdtar does by default, the archive gains an AppleDouble member that
# bsdtar hides when it reads; the check must see it. pack_archive must leave
# both the member and the attribute out.
stage_archive "$top"
policy="libexec/ravel-policy/grove.ts"
xattr -w org.linkuistics.grove.probe fixture "$scratch/archive/stage/$top/$policy"
tar -C "$scratch/archive/stage" -cJf "$scratch/archive/$top.tar.xz" "$top"
refused_archive "$top" "unexpected $top/libexec/ravel-policy/._grove.ts"
pack_fixture "$top"
assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" ||
  fail "pack_archive kept macOS metadata: $(cat "$scratch/archive/output")"
mkdir "$scratch/archive/restored"
tar -xJf "$scratch/archive/$top.tar.xz" -C "$scratch/archive/restored" --xattrs
if xattr -p org.linkuistics.grove.probe "$scratch/archive/restored/$top/$policy" >/dev/null 2>&1; then
  fail 'pack_archive kept an extended attribute'
fi

cp "$repo_root/scripts/release-publish.sh" "$scratch/source/scripts/"
printf '## Unreleased\n\n## v1.2.3\n\n- Meaningful release notes.\n' >"$scratch/source/CHANGELOG.md"
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
grep -Fxq -- '- Meaningful release notes.' "$RELEASE_TEST_NOTES"

# A Git-only tap on detached HEAD must be refused before GitHub publication.
"$real_git" clone -q --branch main "$scratch/remote.git" "$scratch/git-tap"
"$real_git" -C "$scratch/git-tap" checkout -q --detach
cp "$RELEASE_TEST_LOG" "$scratch/published-before"
if GROVE_TAP_DIR="$scratch/git-tap" bash "$scratch/source/scripts/release-publish.sh" >"$scratch/output" 2>&1; then
  echo 'FAIL: publishing accepted a detached Git-only tap' >&2
  exit 1
fi
cmp "$scratch/published-before" "$RELEASE_TEST_LOG"
grep -Fq 'tap is on detached HEAD' "$scratch/output"

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
# The installed smoke test runs over the built archives after the build and
# before each of the three steps that publish: the pushes of main and the tag,
# and the GitHub Release and tap.
dry_line() {
  { grep -Fxn -- "task: [release] $1" "$scratch/dry" || true; } | head -n 1 | cut -d: -f1
}
for level in patch minor major; do
  task --dir "$scratch/task" --dry "release:$level" >"$scratch/dry" 2>&1
  grep -Fq "cargo release $level --execute --no-confirm" "$scratch/dry"
  build="$(dry_line scripts/release-build.sh)"
  smoke="$(dry_line 'scripts/release-smoke.sh --archives target/dist')"
  if [[ -z "$build" || -z "$smoke" ]] || ((build > smoke)); then
    fail "release:$level does not smoke-test target/dist after scripts/release-build.sh"
  fi
  # shellcheck disable=SC2016 # the Taskfile's own text, unexpanded
  for publishing in 'jj git push -b main' 'git push origin "$(git describe --tags --exact-match HEAD)"' \
    scripts/release-publish.sh; do
    line="$(dry_line "$publishing")"
    if [[ -z "$line" ]] || ((line < smoke)); then
      fail "release:$level runs '$publishing' before the installed smoke test, or not at all"
    fi
  done
done
[[ "$(cat "$RELEASE_TEST_LOG")" == 'published' ]]

if RELEASE_TEST_DOCTOR_FAIL=73 task --dir "$scratch/task" release:patch >"$scratch/output" 2>&1; then
  echo 'FAIL: release continued after failed prerequisites' >&2
  exit 1
fi
grep -Fq 'exit status 73' "$scratch/output"
[[ "$("$real_git" -C "$scratch/task" rev-parse HEAD)" == "$before" ]]
[[ -z "$("$real_git" -C "$scratch/task" tag --list)" ]]
[[ ! -e "$scratch/task/.jj/release-lock" ]]

# A second invocation must not enter an in-progress release.
mkdir "$scratch/task/.jj/release-lock"
if task --dir "$scratch/task" release:patch >"$scratch/output" 2>&1; then
  echo 'FAIL: a second release acquired the active release lock' >&2
  exit 1
fi
grep -Fq 'Another release holds .jj/release-lock' "$scratch/output"
[[ -d "$scratch/task/.jj/release-lock" ]]

echo 'release tests: all passed'

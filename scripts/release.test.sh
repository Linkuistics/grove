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

# Fixture skill sources, so that these cases need no network and no checkout
# of the recorded packs: a core repository and two pack repositories, each
# with an executable and a link. Core has an ignored file, which no archive
# holds, and an untracked file, which a working copy's archive holds. A pack
# has a file outside its source directory, which no archive holds.
fixture="$scratch/sources"
fixture_repo() {
  pack_git init -q "$1"
  pack_git -C "$1" add -A
  pack_git -C "$1" commit -qm fixture
}
mkdir -p "$fixture/core/plugins/grove/skills/grove" "$fixture/core/plugins/grove/__pycache__"
printf '# grove\n' >"$fixture/core/plugins/grove/skills/grove/SKILL.md"
printf '#!/bin/sh\n' >"$fixture/core/plugins/grove/conformance.sh"
chmod +x "$fixture/core/plugins/grove/conformance.sh"
ln -s SKILL.md "$fixture/core/plugins/grove/skills/grove/alias.md"
printf '__pycache__/\n' >"$fixture/core/.gitignore"
fixture_repo "$fixture/core"
printf 'ignored\n' >"$fixture/core/plugins/grove/__pycache__/cache.pyc"
printf 'untracked\n' >"$fixture/core/plugins/grove/untracked.md"
for pack in alpha beta; do
  mkdir -p "$fixture/$pack/plugins/$pack/skills/$pack" "$fixture/$pack/plugins/$pack/craft/craft-$pack"
  printf '# %s\n' "$pack" >"$fixture/$pack/plugins/$pack/skills/$pack/SKILL.md"
  printf '# craft\n' >"$fixture/$pack/plugins/$pack/craft/craft-$pack/SKILL.md"
  printf '#!/bin/sh\n' >"$fixture/$pack/plugins/$pack/run.sh"
  chmod +x "$fixture/$pack/plugins/$pack/run.sh"
  ln -s "../skills/$pack/SKILL.md" "$fixture/$pack/plugins/$pack/craft/link.md"
  printf 'outside\n' >"$fixture/$pack/README.md"
  fixture_repo "$fixture/$pack"
done
alpha_sha="$(pack_git -C "$fixture/alpha" rev-parse HEAD)"
beta_sha="$(pack_git -C "$fixture/beta" rev-parse HEAD)"
cat >"$fixture/packs.json" <<EOF
{"alpha": {"url": "$fixture/alpha", "path": "plugins/alpha", "sha": "$alpha_sha"},
 "beta": {"url": "$fixture/beta", "path": "plugins/beta", "sha": "$beta_sha"}}
EOF
pack_checkout "$fixture/cache/alpha/$alpha_sha" "$fixture/alpha" "$alpha_sha" >/dev/null
pack_checkout "$fixture/cache/beta/$beta_sha" "$fixture/beta" "$beta_sha" >/dev/null
ARCHIVE_CORE_REPO="$fixture/core"
ARCHIVE_PACK_RECORD="$fixture/packs.json"
ARCHIVE_PACK_CACHE="$fixture/cache"

# The manifest gives each source file its path and Git mode.
manifest="$(archive_manifest)"
for line in \
  $'100644\tshare/ravel/skills/grove-core/skills/grove/SKILL.md' \
  $'100755\tshare/ravel/skills/grove-core/conformance.sh' \
  $'120000\tshare/ravel/skills/grove-core/skills/grove/alias.md' \
  $'100644\tshare/ravel/skills/grove-core/untracked.md' \
  $'100644\tshare/ravel/skills/alpha/craft/craft-alpha/SKILL.md' \
  $'100755\tshare/ravel/skills/beta/run.sh' \
  $'120000\tshare/ravel/skills/beta/craft/link.md'; do
  grep -Fxq -- "$line" <<<"$manifest" || fail "the archive manifest lacks '$line'"
done
if grep -e __pycache__ -e README.md <<<"$manifest" | grep -q share/; then
  fail 'the archive manifest holds an ignored file or a file outside a source directory'
fi

# Stage the manifest's layout under $scratch/archive/stage: placeholder files
# with the manifest's modes for the fixed part, and a copy of each source.
stage_archive() {
  local top="$1" mode entry
  rm -rf "$scratch/archive"
  mkdir -p "$scratch/archive/stage/$top"
  while IFS=$'\t' read -r mode entry; do
    mkdir -p "$(dirname "$scratch/archive/stage/$top/$entry")"
    printf 'fixture %s\n' "$entry" >"$scratch/archive/stage/$top/$entry"
    [[ "$mode" != 100755 ]] || chmod +x "$scratch/archive/stage/$top/$entry"
  done < <(archive_fixed_manifest)
  copy_skill_sources "$scratch/archive/stage/$top/$ARCHIVE_SKILLS" || fail 'cannot copy the skill sources'
}
pack_fixture() {
  pack_archive "$scratch/archive/stage" "$1" "$scratch/archive/$1.tar.xz"
}
# The check must refuse the archive, reporting exactly the line given.
refused_archive() {
  local top="$1" expected="$scratch/archive/$1.tar.xz: $2"
  if assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" 2>"$scratch/archive/errors"; then
    fail "an archive with '$2' passed the manifest check"
  fi
  grep -Fxq -- "$expected" "$scratch/archive/output" ||
    fail "the manifest check did not report '$2': $(cat "$scratch/archive/output")"
}

# Caller assets ship independently of Ravel's SDK and runtime: the TypeScript
# set for Ravel 1.x and the Rhai set for Ravel 2.x.
typescript_set=(libexec/ravel-policy/grove.ts libexec/ravel-policy/provenance.ts
  libexec/ravel-policy/examples/grove-review.ts libexec/ravel-policy/examples/grove-static.ts
  libexec/ravel-policy/examples/review.ts libexec/ravel-policy/sample/policy.ts
  libexec/ravel-policy/typecheck/review.ts libexec/ravel-policy/typecheck/static.ts)
rhai_set=(libexec/ravel-policy/grove.rhai libexec/ravel-policy/provenance.rhai
  libexec/ravel-policy/examples/grove-review.rhai libexec/ravel-policy/examples/grove-static.rhai
  libexec/ravel-policy/examples/review.rhai libexec/ravel-policy/examples/static.rhai
  libexec/ravel-policy/sample/config.rhai)
paths="$(cut -f2 <<<"$manifest")"
for required in bin/grove bin/grove-llm libexec/ravel-policy/README.md "${typescript_set[@]}" "${rhai_set[@]}"; do
  grep -Fxq -- "$required" <<<"$paths" || fail "the archive manifest omits $required"
done
for executable in bin/grove bin/grove-llm; do
  grep -Fxq -- $'100755\t'"$executable" <<<"$manifest" || fail "the archive manifest does not make $executable executable"
done
(cd "$repo_root/assets/ravel-policy" && find . ! -type d | sed 's|^\./||' | LC_ALL=C sort) >"$scratch/assets-source"
sed -n 's|^libexec/ravel-policy/||p' <<<"$paths" | LC_ALL=C sort >"$scratch/assets-manifest"
diff -u "$scratch/assets-source" "$scratch/assets-manifest" >&2 ||
  fail "the archive manifest differs from the complete caller asset tree"

template="$repo_root/scripts/templates/grove.rb.tmpl"
formula_bins="$(sed -n 's/^ *bin\.install //p' "$template" | tr -d '"' | tr ',' '\n' | tr -d ' ' | LC_ALL=C sort)"
[[ "$formula_bins" == "$(grep '^bin/' <<<"$paths" | LC_ALL=C sort)" ]] ||
  fail "the formula's bin.install ($formula_bins) differs from the manifest's bin/ entries"
grep -Fxq '    libexec.install "libexec/ravel-policy"' "$template" ||
  fail 'the formula does not install libexec/ravel-policy'
# template_projection TEMPLATE: the formula moves each archive source into
# the private KEG_SKILL_SOURCES and puts its relative alias in ARCHIVE_SKILLS.
# No line installs a real directory under share, so a later release keeps the
# layout. It prints the first defect and fails. "The keg layout" of
# docs/specs/codex-skill-provisioning.md.
template_projection() {
  local real
  # An install call whose receiver names share moves a real directory there.
  # install_symlink makes a link, and a path after the call is its argument.
  real="$(grep -En '^[^#]*share[^#]*\.install([ (]|$)' "$1" || true)"
  [[ -z "$real" ]] || {
    echo "the formula installs a real directory under share: $real"
    return 1
  }
  if ! grep -Fxq '    SKILL_SOURCES.each do |id|' "$1" ||
    ! grep -Fxq '      (libexec/"skill-sources").install "share/ravel/skills/#{id}"' "$1"; then
    echo "the formula does not move each source into $KEG_SKILL_SOURCES"
    return 1
  fi
  grep -Fxq '      (share/"ravel/skills").install_symlink libexec/"skill-sources"/id' "$1" || {
    echo "the formula does not put a directory symlink to each source in $ARCHIVE_SKILLS"
    return 1
  }
}
projection="$(template_projection "$template")" || fail "$projection"
# Controls: the earlier layout, alone and beside the projection, is refused.
# The earlier template moved share/ravel/skills into the keg whole.
awk '!seen && /^    SKILL_SOURCES.each do/ { seen = 1; print "    (share/\"ravel\").install \"share/ravel/skills\""; skip = 1; next }
  skip && /^    end$/ { skip = 0; next }
  !skip' "$template" >"$scratch/earlier.rb.tmpl"
awk '{ print } /^      \(share\/"ravel\/skills"\)\.install_symlink / {
  print "      (share/\"ravel/skills\").install \"share/ravel/skills/#{id}\"" }' "$template" >"$scratch/beside.rb.tmpl"
for control in earlier beside; do
  ! cmp -s "$template" "$scratch/$control.rb.tmpl" || fail "the $control control did not change the template"
  if projection="$(template_projection "$scratch/$control.rb.tmpl")"; then
    fail "the projection check accepts a template that installs real sources under share ($control)"
  fi
  [[ "$projection" == "the formula installs a real directory under share: "* ]] ||
    fail "the projection check refused the $control control for another reason: $projection"
done
grep -Fxq '  depends_on "linkuistics/taps/ravel"' "$template" ||
  fail 'the formula does not depend on independent Ravel'
# The dependency stays unversioned through promotion; ravel@2 is an opt-in prefix.
[[ "$(grep -c '^ *depends_on ' "$template")" == 1 ]] ||
  fail 'the formula has a dependency other than unversioned Ravel'
grep -Fxq '    prefix.install ".claude-plugin", ".agents"' "$template" ||
  fail 'the formula does not install the marketplace files at the keg root'
if grep -v -e '^bin/' -e '^libexec/ravel-policy/' -e "^$ARCHIVE_SKILLS/" \
  -e '^\.claude-plugin/' -e '^\.agents/' <<<"$paths" | grep -q /; then
  fail 'the manifest ships a directory the formula does not install'
fi

# The marketplace files of the hook adapters: the manifest lists exactly the
# files of assets/keg-root, and each file names the hooks plugin of the
# private writing source in the keg, not its public alias. "Hook adapters" of
# docs/specs/codex-skill-provisioning.md.
(cd "$repo_root/assets/keg-root" && find . ! -type d | sed 's|^\./||' | LC_ALL=C sort) >"$scratch/keg-root-source"
grep -v -e '^bin/' -e '^libexec/' -e '^share/' -e '^LICENSE$' -e '^README.md$' <<<"$paths" |
  LC_ALL=C sort >"$scratch/keg-root-manifest"
diff -u "$scratch/keg-root-source" "$scratch/keg-root-manifest" >&2 ||
  fail "the archive manifest's keg-root files differ from assets/keg-root"
pack_record_members "$repo_root/plugins/packs.json" | cut -f2 | grep -Fxq writing ||
  fail 'the pack record has no writing source for the hook adapters'
python3 -I - "$repo_root/assets/keg-root" "$KEG_SKILL_SOURCES" <<'PY' || fail 'a marketplace file of the hook adapters is wrong; see above'
import json, sys
root, private = sys.argv[1:]
plugin_root = f"./{private}/writing/craft/technical-communication"
expected = {
    ".claude-plugin/marketplace.json": ("technical-communication-hooks", lambda p: p["source"]),
    ".agents/plugins/marketplace.json": ("technical-communication", lambda p: p["source"]["path"]),
}
failed = False
for path, (plugin, source) in expected.items():
    document = json.load(open(f"{root}/{path}"))
    plugins = document.get("plugins", [])
    found = [(p.get("name"), source(p)) for p in plugins]
    if document.get("name") != "writing-hooks" or found != [(plugin, plugin_root)]:
        print(f"{path}: name {document.get('name')!r}, plugins {found}; "
              f"expected writing-hooks with {plugin} at {plugin_root}", file=sys.stderr)
        failed = True
sys.exit(failed)
PY
# The formula's test names the same six sources as the installed smoke test.
formula_sources="$(sed -n 's/^ *SKILL_SOURCES = %w\[\(.*\)\]\.freeze$/\1/p' "$template" | tr ' ' '\n')"
smoke_sources="$(sed -n "/^expected_sources='/,/'\$/p" "$repo_root/scripts/grove-installed-smoke.sh" |
  sed "s/^expected_sources='//; s/'\$//")"
[[ -n "$formula_sources" && "$formula_sources" == "$smoke_sources" ]] ||
  fail "the formula's sources ($formula_sources) differ from the installed smoke test's ($smoke_sources)"
recorded_sources="$({
  echo "$ARCHIVE_CORE_ID"
  pack_record_members "$repo_root/plugins/packs.json" | cut -f2
} | LC_ALL=C sort)"
[[ "$formula_sources" == "$recorded_sources" ]] ||
  fail "the formula's sources differ from core and the members of plugins/packs.json"

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
while IFS=$'\t' read -r _ entry; do
  stage_archive "$top"
  rm "$scratch/archive/stage/$top/$entry"
  pack_fixture "$top"
  refused_archive "$top" "missing $top/$entry"
done <<<"$manifest"

# An archive without the Rhai set fails, naming each Rhai file.
stage_archive "$top"
for entry in "${rhai_set[@]}"; do
  rm "$scratch/archive/stage/$top/$entry"
done
pack_fixture "$top"
for entry in "${rhai_set[@]}"; do
  refused_archive "$top" "missing $top/$entry"
done

# Runtime, SDK, probes and notices from another project must never ship, nor
# a skill payload outside the keg sources, a source outside the record, an
# ignored file or a skill resolver, nor a keg-root file other than the two
# marketplace files.
for extra in bin/harness-dispatch bin/ravel libexec/ravel/ravel-policy \
  libexec/harness-dispatch/harness-dispatch-policy libexec/ravel/sdk/index.ts \
  libexec/ravel-policy/node_modules/cache libexec/ravel-policy/notices/NOTICES.md \
  plugins/grove/skills/grove/SKILL.md libexec/skills/grove/SKILL.md \
  libexec/ravel-policy/skills/grove/SKILL.md bin/grove-skills \
  share/ravel/skills/personal/craft/machine/SKILL.md share/ravel/skills/grove-core/__pycache__/cache.pyc \
  share/ravel/skills/alpha/README.md share/ravel/installation.json \
  .claude-plugin/plugin.json .agents/plugins/linkuistics.json .codex-plugin/plugin.json; do
  stage_archive "$top"
  mkdir -p "$(dirname "$scratch/archive/stage/$top/$extra")"
  touch "$scratch/archive/stage/$top/$extra"
  pack_fixture "$top"
  refused_archive "$top" "unexpected $top/$extra"
done

# Each file keeps its Git mode: an executable, a link and a plain file.
stage_archive "$top"
chmod -x "$scratch/archive/stage/$top/bin/grove-llm"
pack_fixture "$top"
refused_archive "$top" "$top/bin/grove-llm has Git mode 100644, not 100755"

stage_archive "$top"
ln -sf ../grove "$scratch/archive/stage/$top/bin/grove-llm"
pack_fixture "$top"
refused_archive "$top" "$top/bin/grove-llm has Git mode 120000, not 100755"

stage_archive "$top"
chmod -x "$scratch/archive/stage/$top/share/ravel/skills/beta/run.sh"
pack_fixture "$top"
refused_archive "$top" "$top/share/ravel/skills/beta/run.sh has Git mode 100644, not 100755"

stage_archive "$top"
chmod +x "$scratch/archive/stage/$top/share/ravel/skills/alpha/skills/alpha/SKILL.md"
pack_fixture "$top"
refused_archive "$top" "$top/share/ravel/skills/alpha/skills/alpha/SKILL.md has Git mode 100755, not 100644"

stage_archive "$top"
link="$scratch/archive/stage/$top/share/ravel/skills/grove-core/skills/grove/alias.md"
rm "$link"
printf '# grove\n' >"$link"
pack_fixture "$top"
refused_archive "$top" "$top/share/ravel/skills/grove-core/skills/grove/alias.md has Git mode 100644, not 120000"

stage_archive "$top"
mkdir "$scratch/archive/stage/stray"
COPYFILE_DISABLE=1 tar -C "$scratch/archive/stage" --no-xattrs -cJf "$scratch/archive/$top.tar.xz" "$top" stray
refused_archive "$top" "top level is not exactly $top/:"

# The manifest reads each pack only from a clean checkout of its recorded sha.
# A missing checkout, a changed checkout and a record that names another sha
# each fail the check of a complete archive, naming the pack, and the copy
# refuses too.
stage_archive "$top"
pack_fixture "$top"
checkout="$fixture/cache/beta/$beta_sha"
mv "$checkout" "$scratch/held"
refused_archive "$top" 'the archive manifest cannot be listed; see above'
grep -Fq "pack beta has no clean checkout of $beta_sha" "$scratch/archive/errors" ||
  fail "a missing checkout was not named: $(cat "$scratch/archive/errors")"
if copy_skill_sources "$scratch/refused-copy" 2>/dev/null; then
  fail 'the sources were copied without a pack checkout'
fi
mv "$scratch/held" "$checkout"
printf 'local\n' >"$checkout/plugins/beta/local.md"
refused_archive "$top" 'the archive manifest cannot be listed; see above'
rm "$checkout/plugins/beta/local.md"
sed "s/$beta_sha/$alpha_sha/" "$fixture/packs.json" >"$fixture/other.json"
ARCHIVE_PACK_RECORD="$fixture/other.json"
refused_archive "$top" 'the archive manifest cannot be listed; see above'
grep -Fq "pack beta has no clean checkout of $alpha_sha" "$scratch/archive/errors" ||
  fail "a record of another sha was not named: $(cat "$scratch/archive/errors")"
ARCHIVE_PACK_RECORD="$fixture/packs.json"
assert_archive "$scratch/archive/$top.tar.xz" "$top" >"$scratch/archive/output" ||
  fail "the restored checkout failed the manifest check: $(cat "$scratch/archive/output")"

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

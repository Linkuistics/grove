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

# Only the build and standalone LLM boundary are faked; preparation uses real jj.
export RELEASE_TEST_CARGO
RELEASE_TEST_CARGO="$(command -v cargo)"
export RELEASE_TEST_SCRATCH="$scratch"
mkdir "$scratch/bin"
cat >"$scratch/bin/cargo" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
if [[ "$1" != build ]]; then
  exec "$RELEASE_TEST_CARGO" "$@"
fi
[[ "$*" == $'build\n--locked\n-p\ngrove\n-p\ngrove-llm' ]]
echo build >>"$RELEASE_TEST_SCRATCH/calls"
[[ "${RELEASE_TEST_MODE:-success}" != build-failure ]] || exit 1
mkdir -p target/debug
cp "$RELEASE_TEST_SCRATCH/grove-fixture" target/debug/grove
chmod +x target/debug/grove
EOF
cat >"$scratch/grove-fixture" <<'EOF'
#!/usr/bin/env bash
set -euo pipefail
IFS=$'\n\t'
[[ "$1" == run && "$2" == release-notes ]]
shift 2
prompt=''
output=''
ui=''
inputs=()
runtime_reads=()
while (($#)); do
  case "$1" in
    --prompt-file) prompt="$2" ;;
    --input) inputs+=("$2") ;;
    --output) output="$2" ;;
    --runtime-read) runtime_reads+=("$2") ;;
    --ui) ui="$2" ;;
    *) exit 2 ;;
  esac
  shift 2
done
[[ -f "$prompt" && ! -e "$output" && "$ui" == auto ]]
[[ ${#inputs[@]} == 3 && ${#runtime_reads[@]} == 2 ]]
[[ "${runtime_reads[0]}" == "$RELEASE_TEST_SCRATCH/auth one.json" ]]
[[ "${runtime_reads[1]}" == "$RELEASE_TEST_SCRATCH/auth-two.json" ]]
mkdir -p "$RELEASE_TEST_SCRATCH/observed"
cp "$prompt" "$RELEASE_TEST_SCRATCH/observed/prompt"
for input in "${inputs[@]}"; do
  cp "$input" "$RELEASE_TEST_SCRATCH/observed/"
done
echo run >>"$RELEASE_TEST_SCRATCH/calls"
case "${RELEASE_TEST_MODE:-success}" in
  runner-failure) exit 1 ;;
  missing) ;;
  empty) : >"$output" ;;
  whitespace) printf ' \n\t\n' >"$output" ;;
  heading) printf '## v99.0.0\n\n- Unexpected version.\n' >"$output" ;;
  indented-heading) printf '  ## Unreleased\n\n- Unexpected section.\n' >"$output" ;;
  success) printf '### Changed\n\n- The fixture now prints an improvement.\n' >"$output" ;;
  *) exit 2 ;;
esac
EOF
chmod +x "$scratch/bin/cargo"
export PATH="$scratch/bin:$PATH"
printf '{}\n' >"$scratch/auth one.json"
printf '{}\n' >"$scratch/auth-two.json"
export GROVE_RELEASE_RUNTIME_READ
GROVE_RELEASE_RUNTIME_READ="$(printf '%s\n%s' "$scratch/auth one.json" "$scratch/auth-two.json")"

# Failed builds, runner failures, and invalid bodies leave main and notes intact.
main_before="$(jj -R "$scratch/repo" log --no-graph -r main -T commit_id)"
cp "$scratch/repo/CHANGELOG.md" "$scratch/original-notes"
for mode in build-failure runner-failure missing empty whitespace heading indented-heading; do
  if (cd "$scratch/repo" && RELEASE_TEST_MODE="$mode" bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then
    echo "FAIL: preparation accepted $mode" >&2
    exit 1
  fi
  cmp "$scratch/original-notes" "$scratch/repo/CHANGELOG.md"
  [[ "$(jj -R "$scratch/repo" log --no-graph -r main -T commit_id)" == "$main_before" ]]
done

# Missing notes use the configured runner; unrelated work stays saved.
(cd "$scratch/repo" && bash scripts/release-prepare.sh)
grep -Fxq -- '- The fixture now prints an improvement.' "$scratch/repo/CHANGELOG.md"
grep -Fxq -- '- Existing notes.' "$scratch/repo/CHANGELOG.md"
grep -Fxq -- '- Existing notes.' "$scratch/observed/previous-changelog.md"
grep -Fq 'Improve the fixture' "$scratch/observed/changes.txt"
grep -Fq '+fn main() { println!("improved"); }' "$scratch/observed/changes.diff"
grep -Fq 'Markdown section body' "$scratch/observed/prompt"
[[ ! -e "$scratch/repo/.grove" ]]
[[ "$(jj -R "$scratch/repo" file show -r "$saved" 'root:.grove/_BRIEF.md')" == 'Unrelated work' ]]
[[ -z "$(git -C "$scratch/repo" status --porcelain)" ]]

# Existing notes skip both the build and LLM and remain byte-for-byte intact.
before="$(git -C "$scratch/repo" rev-parse HEAD)"
cp "$scratch/repo/CHANGELOG.md" "$scratch/notes"
cp "$scratch/calls" "$scratch/calls-before"
(cd "$scratch/repo" && bash scripts/release-prepare.sh)
cmp "$scratch/notes" "$scratch/repo/CHANGELOG.md"
cmp "$scratch/calls-before" "$scratch/calls"
[[ "$(git -C "$scratch/repo" rev-parse HEAD)" == "$before" ]]

# Once main itself is tagged, preparation must refuse to cut it again.
git -C "$scratch/repo" tag -f v1.2.3
if (cd "$scratch/repo" && bash scripts/release-prepare.sh) >"$scratch/output" 2>&1; then
  echo 'FAIL: preparation accepted an already tagged main' >&2
  exit 1
fi
grep -Fq 'No changes since v1.2.3' "$scratch/output"
echo 'release preparation tests: all passed'

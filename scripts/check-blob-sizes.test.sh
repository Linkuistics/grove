#!/usr/bin/env bash
#
# Controls for scripts/check-blob-sizes.sh, on scratch jj repositories.
#
# A check that has only ever passed is not a check. Each control below is a repository the
# script must refuse or warn about, run beside one it must pass, so a script that reports
# clean everywhere cannot satisfy this file.

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
readonly repo_root
script="$repo_root/scripts/check-blob-sizes.sh"
scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

export JJ_CONFIG="$scratch/jj.toml"
printf '[user]\nname="Blob Test"\nemail="blob@example.invalid"\n' >"$JJ_CONFIG"
export GIT_AUTHOR_NAME='Blob Test' GIT_COMMITTER_NAME='Blob Test'
export GIT_AUTHOR_EMAIL='blob@example.invalid' GIT_COMMITTER_EMAIL='blob@example.invalid'

MIB=1048576
failures=0

pass() { echo "  ok: $1"; }
fail() {
  echo "  FAILED: $1" >&2
  failures=$((failures + 1))
}

# new_repo NAME: a jj repository with default configuration (a 1 MiB snapshot limit) and an
# `origin` bare repository beside it.
new_repo() {
  local name="$1"
  git init -q --bare "$scratch/$name-origin.git"
  jj git init "$scratch/$name" >/dev/null 2>&1
  jj -R "$scratch/$name" git remote add origin "$scratch/$name-origin.git"
  echo "$scratch/$name"
}

blob() { # blob REPO PATH MIB
  head -c $(($3 * MIB)) /dev/zero >"$1/$2"
}

run() { # run MODE REPO -> sets status and output
  set +e
  output="$(bash "$script" "$@" 2>&1)"
  status=$?
  set -e
}

expect() { # expect STATUS PATTERN LABEL
  if [[ "$status" -eq "$1" ]] && grep -Eq -- "$2" <<<"$output"; then
    pass "$3"
  else
    fail "$3 (status $status, wanted $1, pattern $2)"
    printf '    %s\n' "${output//$'\n'/$'\n    '}" >&2
  fi
}

# is_tracked REPO PATH. The listing is captured first: `grep -q` closing a pipe early makes jj
# die of SIGPIPE, which `pipefail` would report as "not tracked".
is_tracked() {
  local listing
  listing="$(cd "$1" && jj file list 2>/dev/null)" # paths print relative to the cwd, not to -R
  grep -qx -- "$2" <<<"$listing"
}

echo "tip guard"
repo="$(new_repo tip-small)"
blob "$repo" small.bin 1
jj -R "$repo" describe -m small >/dev/null 2>&1
run tip "$repo"
expect 0 'no tracked file' 'a repository of small files passes'

# A new 60 MiB file is skipped by jj's snapshot with a warning, so it is in no commit and a
# guard that reads commits cannot see it. The control is therefore not this file.
repo="$(new_repo tip-untracked)"
blob "$repo" huge.bin 60
jj -R "$repo" describe -m untracked >/dev/null 2>&1
if is_tracked "$repo" huge.bin; then
  fail 'jj tracked a new 60 MiB file with default configuration; the premise changed'
else
  pass 'jj leaves a new 60 MiB file untracked with default configuration'
fi
run tip "$repo"
expect 0 'no tracked file' 'so an untracked scratch file proves nothing about the guard'

# A file that is already tracked and then grows is snapshotted whatever its size.
repo="$(new_repo tip-grown)"
blob "$repo" grown.bin 1
jj -R "$repo" describe -m small >/dev/null 2>&1
blob "$repo" grown.bin 60
jj -R "$repo" describe -m grown >/dev/null 2>&1
if is_tracked "$repo" grown.bin; then
  pass 'the grown file is tracked, so the control is able to fail'
else
  fail 'the grown file is not tracked, so this control could not fail'
fi
run tip "$repo"
expect 1 'grown.bin is 62914560 bytes' 'a tracked file grown past 50 MiB fails the guard'

# The same, by raising the scratch repository's snapshot limit for a new file.
repo="$(new_repo tip-raised)"
jj -R "$repo" config set --repo snapshot.max-new-file-size 200MiB
blob "$repo" raised.bin 60
jj -R "$repo" describe -m raised >/dev/null 2>&1
is_tracked "$repo" raised.bin || fail 'the raised-limit file is not tracked'
run tip "$repo"
expect 1 'raised.bin is 62914560 bytes' 'a new file tracked under a raised limit fails the guard'

# A listing that fails must not read as an empty, passing one.
run tip "$scratch/not-a-repository"
if [[ "$status" -ne 0 ]]; then
  pass 'a path that is no repository is an error, not a pass'
else
  fail 'a path that is no repository passed'
fi

echo "publication history check"
repo="$(new_repo pub-small)"
blob "$repo" small.bin 1
jj -R "$repo" describe -m small >/dev/null 2>&1
run publication "$repo"
expect 0 'publication: [0-9]+ commit\(s\).*0 blob\(s\) above 100 MiB, 0 between' \
  'a small blob passes'

repo="$(new_repo pub-warn)"
jj -R "$repo" config set --repo snapshot.max-new-file-size 200MiB
blob "$repo" sixty.bin 60
jj -R "$repo" describe -m sixty >/dev/null 2>&1
run publication "$repo"
expect 0 'WARN 62914560 bytes .* in sixty.bin at commit [0-9a-f]{40}, change [a-z]+' \
  'a 60 MiB blob warns by commit, change ID and path, and still passes'
expect 0 '0 blob\(s\) above 100 MiB, 1 between' 'and is counted as a warning, not a failure'

repo="$(new_repo pub-fail)"
jj -R "$repo" config set --repo snapshot.max-new-file-size 200MiB
blob "$repo" huge.bin 101
jj -R "$repo" describe -m huge >/dev/null 2>&1
jj -R "$repo" new >/dev/null 2>&1
blob "$repo" huge.bin 1 # the tip no longer holds it; the history still does
jj -R "$repo" describe -m shrunk >/dev/null 2>&1
run tip "$repo"
expect 0 'no tracked file' 'the tip alone does not see an oversized blob behind it'
run publication "$repo"
expect 1 'FAIL 105906176 bytes .* in huge.bin at commit' 'a 101 MiB blob in history fails'
expect 1 '1 blob\(s\) above 100 MiB' 'and is counted as a failure'

# Scope: what origin's bookmarks already hold is not published again.
jj -R "$repo" bookmark create main -r @ >/dev/null 2>&1
jj -R "$repo" git push --remote origin --bookmark main >/dev/null 2>&1
run publication "$repo"
expect 0 '0 blob\(s\) above 100 MiB, 0 between' 'a blob origin already holds is out of scope'

# Another remote's bookmarks do not excuse it.
repo="$(new_repo pub-other)"
git init -q --bare "$scratch/pub-other-skills.git"
jj -R "$repo" git remote add skills "$scratch/pub-other-skills.git"
jj -R "$repo" config set --repo snapshot.max-new-file-size 200MiB
blob "$repo" huge.bin 101
jj -R "$repo" describe -m huge >/dev/null 2>&1
jj -R "$repo" bookmark create main -r @ >/dev/null 2>&1
jj -R "$repo" git push --remote skills --bookmark main >/dev/null 2>&1
run publication "$repo"
expect 1 'FAIL 105906176 bytes' 'a blob held only by another remote still fails'

if ((failures > 0)); then
  echo "check-blob-sizes.test: $failures failure(s)" >&2
  exit 1
fi
echo "check-blob-sizes.test: all controls pass"

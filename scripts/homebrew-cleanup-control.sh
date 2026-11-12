#!/usr/bin/env bash
# The control for the owner's Homebrew cleanup protection of ravel.
#
#   homebrew-cleanup-control.sh [BREW_ENV]
#
# BREW_ENV defaults to ~/.homebrew/brew.env. The control shows that the file
# keeps Homebrew's periodic cleanup away from an old ravel keg. It also shows
# that the same cleanup removes that keg without the file.
#
# The control only reads the real prefix. It clones Library/Homebrew, the
# linkuistics tap and the installed ravel keg into a scratch prefix, and adds an
# older copy of that keg. The scratch bin/brew is a copy, not a link, so the
# scratch prefix is its own repository. brew.sh then takes the Cellar from the
# scratch prefix. A link would make the real Cellar the scratch Cellar.
#
# Each run uses env -i with a scratch HOME, cache, logs and temp. The "with"
# HOME holds a copy of BREW_ENV as its user brew.env. The "without" HOME holds
# no brew.env. The ruby developer command turns on developer mode only in a
# repository with .git/config, which the scratch prefix does not have. Each run makes the periodic cleanup due and then calls
# Homebrew::Cleanup.periodic_clean!, which install, upgrade and reinstall call.
# Homebrew 7.0.13 source: Library/Homebrew/cleanup.rb (periodic_clean!, clean!,
# skip_clean_formula?), bin/brew (export_homebrew_env_file), brew.sh
# (HOMEBREW_CELLAR).
set -euo pipefail
IFS=$'\n\t'
fail() { echo "homebrew-cleanup-control: $*" >&2; exit 1; }
[[ $# -le 1 ]] || fail 'usage: homebrew-cleanup-control.sh [brew-env]'
brew_env="${1:-$HOME/.homebrew/brew.env}"
[[ -r "$brew_env" ]] || fail "cannot read $brew_env"

real="$(brew --prefix)"
version="$(brew info --json=v2 ravel |
  python3 -I -c 'import json, sys; print(json.load(sys.stdin)["formulae"][0]["versions"]["stable"])')"
[[ -d "$real/Cellar/ravel/$version" ]] || fail "the real prefix has no ravel $version keg"
old='0.1.0'

work="$(mktemp -d)"
cleanup() {
  chmod -R u+w "$work" 2>/dev/null || true
  rm -rf "$work"
}
trap cleanup EXIT
work="$(cd "$work" && pwd -P)"
prefix="$work/prefix"

real_state() { find "$real/Cellar/ravel" -print0 | sort -z | xargs -0 stat -f '%N %p %z %m'; }
before="$(real_state)"

mkdir -p "$prefix/bin" "$prefix/Library/Taps" "$prefix/Cellar/ravel" "$work/cache" "$work/logs" "$work/tmp"
cp -c "$real/bin/brew" "$prefix/bin/brew"
cp -c -R "$real/Library/Homebrew" "$prefix/Library/Homebrew"
cp -c -R "$real/Library/Taps/linkuistics" "$prefix/Library/Taps/linkuistics"
cp -c -R "$real/Cellar/ravel/$version" "$prefix/Cellar/ravel/$version"
cp -c -R "$real/Cellar/ravel/$version" "$prefix/Cellar/ravel/$old"

run_brew() {
  local home="$1"
  shift
  env -i HOME="$home" PATH=/usr/bin:/bin:/usr/sbin:/sbin USER="$(id -un)" TMPDIR="$work/tmp" \
    HOMEBREW_CACHE="$work/cache" HOMEBREW_LOGS="$work/logs" HOMEBREW_TEMP="$work/tmp" \
    HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_ENV_HINTS=1 \
    "$prefix/bin/brew" "$@"
}

# Prints the setting that the run sees, then whether the old keg survives.
run_case() {
  local label="$1" home="$2" setting
  [[ "$(run_brew "$home" --cellar)" == "$prefix/Cellar" ]] || fail "$label: the Cellar is not the scratch Cellar"
  # Homebrew 7 loads no formula of an untrusted tap, and Formula.installed then
  # omits ravel without an error. The trust goes to the scratch HOME only.
  run_brew "$home" trust --formula linkuistics/taps/ravel >/dev/null
  # eligible_kegs_for_cleanup ignores HOMEBREW_NO_CLEANUP_FORMULAE, so both runs
  # must find the old keg eligible. Only the setting can then keep it.
  run_brew "$home" ruby -e \
    "exit Formula['ravel'].eligible_kegs_for_cleanup(quiet: true).map(&:version).map(&:to_s) == ['$old']" ||
    fail "$label: Homebrew does not find ravel $old eligible for cleanup"
  setting="$(run_brew "$home" config | grep '^HOMEBREW_NO_CLEANUP_FORMULAE:' || true)"
  echo "$label: setting '${setting:-unset}'"
  touch -t 202001010000 "$work/cache/.cleaned"
  run_brew "$home" ruby -e 'require "cleanup"; Homebrew::Cleanup.periodic_clean!' |
    sed "s/^/$label: brew: /"
  [[ -d "$prefix/Cellar/ravel/$version" ]] || fail "$label: the cleanup removed the current keg"
  if [[ -d "$prefix/Cellar/ravel/$old" ]]; then echo "$label: ravel $old kept"; else echo "$label: ravel $old removed"; fi
}

mkdir -p "$work/with/.homebrew" "$work/without"
cp "$brew_env" "$work/with/.homebrew/brew.env"
with="$(run_case with "$work/with")"
echo "$with"
[[ "$with" == *"with: ravel $old kept"* ]] || fail 'with the file, the periodic cleanup removed the old keg'
without="$(run_case without "$work/without")"
echo "$without"
[[ "$without" == *"without: setting 'unset'"* ]] || fail 'the run without the file still sees the setting'
[[ "$without" == *"without: ravel $old removed"* ]] || fail 'without the file, the periodic cleanup kept the old keg'
[[ "$(real_state)" == "$before" ]] || fail "the real $real/Cellar/ravel changed"
echo "PASS: $brew_env keeps ravel $old through the periodic cleanup, which removes it without the file;" \
  "$real/Cellar/ravel is unchanged"

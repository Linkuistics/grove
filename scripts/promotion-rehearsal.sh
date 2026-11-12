#!/usr/bin/env bash
# The promotion upgrade rehearsal: fresh installs and upgrades through the tap
# states of docs/specs/coordinated-release.md, "Compatible tap states", with
# real Homebrew in a scratch environment.
#
#   promotion-rehearsal.sh EMPTY_WORK_DIR [TAP_WORKING_COPY]
#
# TAP_WORKING_COPY is the real tap's colocated working copy, by default
# ~/Development/Delivery/homebrew-taps. The instrument only reads it.
# docs/promotion-upgrade-rehearsal-record.md records each run and its
# decisions.
#
# The scratch tap is a bare clone of the real tap's Git history,
# WORK/origin.git. Its `main` names the tap state of a scenario:
#
#   A  12b53f8c449d, Grove 22.8.0 and unversioned Ravel 1.1.0
#   B  f7f9b9b0bc4d, A plus the keg-only ravel@2 2.0.0
#
# Each scenario has its own private Homebrew: a --shared clone of the owner's
# Homebrew repository at its HEAD, with its own prefix, HOME, logs and
# temporary directory. All scenarios share one download cache. Each private
# Homebrew taps WORK/origin.git as linkuistics/taps, because the formulae name
# their dependencies there. A tap state moves when the scenario fetches the
# tap's origin and fast-forwards, which is what `brew update` does for a tap.
# The scenario does not run `brew update`, because that would also move the
# private Homebrew away from the owner's version.
#
# The scratch HOME holds a copy of the owner's ~/.homebrew/brew.env, and every
# package mutation sets HOMEBREW_NO_INSTALL_CLEANUP=1, as the owner's release
# tasks do. The rehearsal therefore measures the owner's path. An ordinary
# user's automatic cleanup is outside this measurement.
#
# After each package mutation, the 1.1.0 front of the scratch prefix and the
# real 1.1.0 front must report `ravel 1.1.0` and refuse `end-session` outside a
# session. Ravel 1.1.0 finds its session through RAVEL_EXIT_FILE, and every
# command here runs under `env -i`, so no inherited session authority reaches
# it. The real prefix, the real tap and the owner's HOME are digested before
# and after the run, and the run fails if any digest changes.
#
# Scenarios: fresh-a, fresh-b, upgrade-a-b. k334 and k335 of the grove add
# states C and D.
set -euo pipefail
IFS=$'\n\t'
fail() {
  echo "promotion-rehearsal: $*" >&2
  exit 1
}
[[ $# -ge 1 && $# -le 2 ]] || fail 'usage: promotion-rehearsal.sh EMPTY_WORK_DIR [TAP_WORKING_COPY]'
[[ -d "$1" && -z "$(ls -A "$1")" ]] || fail "$1 is not an empty directory"
W="$(cd "$1" && pwd -P)"
TAP_WC="$(cd "${2:-$HOME/Development/Delivery/homebrew-taps}" && pwd -P)"
[[ -d "$TAP_WC/Formula" && -d "$TAP_WC/.git" ]] || fail "$TAP_WC is not a colocated tap working copy"
case "$W/" in
  "$TAP_WC"/*) fail 'the work directory lies inside the tap' ;;
esac

STATE_A=12b53f8c449dba322c95d869e2c02bb601b7fa1b
STATE_B=f7f9b9b0bc4d6353dcd8cd41fb47deabafc14c09
OWNER_BREW="$(brew --repository)"
OWNER_PREFIX="$(brew --prefix)"
OWNER_HEAD="$(git -C "$OWNER_BREW" rev-parse HEAD)"
REAL_FRONT="$OWNER_PREFIX/Cellar/ravel/1.1.0/bin/ravel"
[[ -x "$REAL_FRONT" ]] || fail "the real 1.1.0 front $REAL_FRONT is missing"
LOG="$W/rehearsal.log"
mkdir -p "$W/cache" "$W/front-home" "$W/snapshots"
exec > >(tee -a "$LOG") 2>&1

show() { printf '\n## %s\n' "$*"; }
note() { printf '%s\n' "$*"; }
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }

# tree_manifest DIR: one line for each entry below DIR, with its relative
# path, its type, its permission mode, and its link target or SHA-256.
tree_manifest() {
  [[ -e "$1" ]] || {
    echo "absent"
    return
  }
  python3 -I - "$1" <<'PY'
import hashlib, os, stat, sys
root = sys.argv[1]
for base, dirs, files in os.walk(root):
    dirs.sort()
    for name in sorted(dirs + files):
        path = os.path.join(base, name)
        rel = os.path.relpath(path, root)
        st = os.lstat(path)
        mode = oct(stat.S_IMODE(st.st_mode))
        if stat.S_ISLNK(st.st_mode):
            print(rel, "link", mode, os.readlink(path))
        elif stat.S_ISDIR(st.st_mode):
            print(rel, "dir", mode)
        else:
            with open(path, "rb") as handle:
                print(rel, "file", mode, hashlib.sha256(handle.read()).hexdigest())
PY
}

# The protected subjects: the real prefix's Grove and Ravel kegs and links,
# the real tap's working copy, its installed clone and its remote, and the
# owner's Homebrew and Ravel configuration.
snapshot() { # NAME
  local out="$W/snapshots/$1.txt" entry
  {
    for entry in Cellar/ravel Cellar/ravel@2 Cellar/grove; do
      echo "### $OWNER_PREFIX/$entry"
      tree_manifest "$OWNER_PREFIX/$entry"
    done
    for entry in opt/ravel opt/ravel@2 opt/grove bin/ravel bin/grove bin/grove-llm; do
      echo "### $OWNER_PREFIX/$entry -> $(readlink "$OWNER_PREFIX/$entry" || echo absent)"
    done
    echo "### owner Cellar names"
    ls -A "$OWNER_PREFIX/Cellar"
    echo "### real tap working copy"
    git -C "$TAP_WC" rev-parse HEAD
    git -C "$TAP_WC" status --porcelain
    tree_manifest "$TAP_WC/Formula"
    echo "### installed real tap"
    git -C "$OWNER_PREFIX/Library/Taps/linkuistics/homebrew-taps" rev-parse HEAD
    echo "### real tap remote"
    git -C "$TAP_WC" ls-remote origin refs/heads/main
    echo "### owner HOME"
    tree_manifest "$HOME/.homebrew"
    tree_manifest "$HOME/.config/ravel"
  } >"$out"
}

# The private Homebrew of one scenario. Homebrew's inherited_sandbox? refuses
# a sandbox that permits writes to bin/brew, so that one file is read-only.
P=''
H=''
make_brew() { # NAME
  local s="$W/$1" entry
  mkdir -p "$s/logs" "$s/tmp" "$s/home/.homebrew"
  git clone -q --shared "$OWNER_BREW" "$s/brew"
  git -C "$s/brew" checkout -q "$OWNER_HEAD"
  for entry in portable-ruby portable-ruby-arm64-darwin portable-ruby-version; do
    [[ -e "$s/brew/Library/Homebrew/vendor/$entry" || ! -e "$OWNER_BREW/Library/Homebrew/vendor/$entry" ]] ||
      ln -s "$OWNER_BREW/Library/Homebrew/vendor/$entry" "$s/brew/Library/Homebrew/vendor/$entry"
  done
  chmod a-w "$s/brew/bin/brew"
  [[ ! -f "$HOME/.homebrew/brew.env" ]] || cp "$HOME/.homebrew/brew.env" "$s/home/.homebrew/brew.env"
  P="$s/brew"
  H="$s/home"
}

# brew ARGS: the scenario's private Homebrew, with no inherited environment.
brew_s() {
  env -i PATH="$P/bin:/usr/bin:/bin:/usr/sbin:/sbin" HOME="$H" USER="$USER" LOGNAME="$USER" \
    HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_ENV_HINTS=1 \
    HOMEBREW_NO_INSTALL_CLEANUP=1 HOMEBREW_CACHE="$W/cache" HOMEBREW_LOGS="${P%/brew}/logs" \
    HOMEBREW_TEMP="${P%/brew}/tmp" "$P/bin/brew" "$@"
}
run() { # brew ARGS, echoed with its output and exit status
  local status=0
  note "\$ brew $(printf '%s ' "$@")"
  brew_s "$@" || status=$?
  note "[exit $status]"
  return "$status"
}

set_origin() { # COMMIT
  git -C "$W/origin.git" update-ref refs/heads/main "$1"
  note "scratch origin main -> $1 ($(git -C "$W/origin.git" log -1 --format=%s "$1"))"
}
# Tapping does not trust a tap in Homebrew 7.0.13. A fully qualified name
# trusts only the named formula, and a dependency from an untrusted tap
# refuses to load. Library/Homebrew/trust.rb (trust_fully_qualified_items!,
# raise_untrusted!) at the owner's HEAD. Each scenario trusts the tap after
# tapping, as Homebrew's refusal directs. fresh-a first records the refusal.
tap_scratch() {
  run tap linkuistics/taps "file://$W/origin.git"
}
trust_scratch() {
  run trust --tap linkuistics/taps
}
# The tap step of `brew update`: fetch the tap's origin and fast-forward.
update_tap() {
  local tap
  tap="$(brew_s --repository linkuistics/taps)"
  note "\$ git -C \$(brew --repository linkuistics/taps) pull --ff-only"
  env -i PATH=/usr/bin:/bin HOME="$H" git -C "$tap" pull -q --ff-only
  note "tap HEAD: $(git -C "$tap" rev-parse HEAD)"
}

# The old ending path, after each package mutation.
front_ok() { # STEP FRONT
  local version ending status=0
  version="$(env -i PATH=/usr/bin:/bin HOME="$W/front-home" "$2" --version)"
  [[ "$version" == 'ravel 1.1.0' ]] || fail "$1: $2 reports '$version'"
  ending="$(env -i PATH=/usr/bin:/bin HOME="$W/front-home" "$2" end-session grove-continue 2>"$W/front.err")" || status=$?
  [[ "$status" != 0 && -z "$ending" ]] || fail "$1: $2 end-session did not refuse (exit $status)"
  note "$1: $2: $version; end-session refused with exit $status: $(head -1 "$W/front.err")"
}
fronts() { # STEP
  front_ok "$1" "$P/Cellar/ravel/1.1.0/bin/ravel"
  front_ok "$1" "$REAL_FRONT"
}

# A usable pair at A or B: both formula tests pass, the default ravel is the
# 1.1.0 keg, and ravel@2, when installed, is keg-only at 2.0.0 and unlinked.
pair_ok() { # STEP
  local link
  run test linkuistics/taps/ravel
  run test linkuistics/taps/grove
  link="$(readlink "$P/bin/ravel")"
  [[ "$link" == ../Cellar/ravel/1.1.0/bin/ravel ]] || fail "$1: bin/ravel -> $link"
  [[ "$(env -i PATH="$P/bin:/usr/bin:/bin" ravel --version)" == 'ravel 1.1.0' ]] ||
    fail "$1: the default ravel is not 1.1.0"
  note "$1: default PATH ravel: bin/ravel -> $link"
  if [[ -d "$P/Cellar/ravel@2" ]]; then
    run test linkuistics/taps/ravel@2
    [[ "$("$P/opt/ravel@2/bin/ravel" --version)" == 'ravel 2.0.0' ]] || fail "$1: opt/ravel@2 is not 2.0.0"
    if find "$P/bin" "$P/share" -type l -lname '*Cellar/ravel@2/*' | grep -q .; then
      fail "$1: ravel@2 is linked into the shared prefix"
    fi
    note "$1: ravel@2 is keg-only at 2.0.0: opt/ravel@2 -> $(readlink "$P/opt/ravel@2"), no shared links"
  fi
  run list --versions
  # brew test turned developer mode on; the next mutation runs in the default mode.
  run developer off
}

retained() { # NAME: manifest of the scratch prefix's 1.1.0 keg without Homebrew's receipt
  tree_manifest "$P/Cellar/ravel/1.1.0" | grep -v '^INSTALL_RECEIPT.json ' >"$W/snapshots/$1.keg"
}

show "rehearsal inputs"
note "work: $W"
note "tap working copy: $TAP_WC"
note "owner Homebrew: $OWNER_BREW at $OWNER_HEAD ($(brew --version | head -1))"
note "real front: $REAL_FRONT"
snapshot before
git clone -q --bare --no-local "$TAP_WC" "$W/origin.git"
for state in "$STATE_A" "$STATE_B"; do
  git -C "$W/origin.git" cat-file -e "$state^{commit}" || fail "the tap lacks the state commit $state"
done
[[ "$(git -C "$W/origin.git" show "$STATE_A:Formula/grove.rb" | grep -E '^  version ')" == '  version "22.8.0"' ]] ||
  fail 'state A does not hold Grove 22.8.0'
if git -C "$W/origin.git" cat-file -e "$STATE_A:Formula/ravel@2.rb" 2>/dev/null; then
  fail 'state A holds ravel@2'
fi
git -C "$W/origin.git" cat-file -e "$STATE_B:Formula/ravel@2.rb" || fail 'state B lacks ravel@2'

# The instrument's own controls. Each check must fail on a subject that is
# known to be wrong before its clean readings count.
show "instrument controls"
mkdir -p "$W/controls/keg"
cat >"$W/controls/accepts" <<'SH'
#!/bin/sh
case "$1" in --version) echo "ravel 1.1.0" ;; *) exit 0 ;; esac
SH
printf '#!/bin/sh\necho "ravel 2.0.0"\n' >"$W/controls/newer"
chmod +x "$W/controls/accepts" "$W/controls/newer"
for front in accepts newer; do
  if (front_ok control "$W/controls/$front") 2>"$W/controls/$front.err"; then
    fail "the front check accepted the $front control"
  fi
  note "front check failed on the $front control: $(cat "$W/controls/$front.err")"
done
printf 'x\n' >"$W/controls/keg/file"
tree_manifest "$W/controls/keg" >"$W/controls/keg.before"
chmod +x "$W/controls/keg/file"
tree_manifest "$W/controls/keg" >"$W/controls/keg.after"
if cmp -s "$W/controls/keg.before" "$W/controls/keg.after"; then
  fail 'the keg manifest missed a mode change'
fi
note 'keg manifest changed on a mode change'

show "fresh-a: a fresh install at tap state A"
set_origin "$STATE_A"
make_brew fresh-a
tap_scratch
note 'A new user without tap trust:'
if run install linkuistics/taps/grove >"$W/untrusted.out" 2>&1; then
  cat "$W/untrusted.out"
  fail 'fresh-a: Grove installed from an untrusted tap'
fi
cat "$W/untrusted.out"
grep -q 'Refusing to load formula linkuistics/taps/ravel from untrusted tap linkuistics/taps' "$W/untrusted.out" ||
  fail 'fresh-a: the untrusted install failed for another reason'
[[ ! -e "$P/Cellar/grove" && ! -e "$P/Cellar/ravel" ]] || fail 'fresh-a: the refused install left a keg'
trust_scratch
run install linkuistics/taps/grove
fronts fresh-a:install
pair_ok fresh-a

show "fresh-b: a fresh install at tap state B"
set_origin "$STATE_B"
make_brew fresh-b
tap_scratch
trust_scratch
note 'The first ravel@2 on a prefix without either Ravel formula uses --skip-link.'
run install --skip-link linkuistics/taps/ravel@2
[[ ! -e "$P/bin/ravel" ]] || fail 'fresh-b: --skip-link linked bin/ravel'
run install linkuistics/taps/grove
fronts fresh-b:install
pair_ok fresh-b

show "upgrade-a-b: an upgrade from tap state A to tap state B"
set_origin "$STATE_A"
make_brew upgrade-a-b
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts upgrade-a-b:install-a
pair_ok upgrade-a-b:a
retained upgrade-a-b-a
set_origin "$STATE_B"
update_tap
run outdated --verbose || true
run upgrade
fronts upgrade-a-b:upgrade
note 'ravel 1.1.0 is installed, so the opt-in ravel@2 installation needs no --skip-link.'
run install linkuistics/taps/ravel@2
fronts upgrade-a-b:install-ravel@2
pair_ok upgrade-a-b:b
retained upgrade-a-b-b
if cmp -s "$W/snapshots/upgrade-a-b-a.keg" "$W/snapshots/upgrade-a-b-b.keg"; then
  note 'upgrade-a-b: the retained 1.1.0 keg is unchanged apart from INSTALL_RECEIPT.json'
else
  diff "$W/snapshots/upgrade-a-b-a.keg" "$W/snapshots/upgrade-a-b-b.keg" || true
  fail 'upgrade-a-b: the retained 1.1.0 keg changed'
fi

show "protected subjects"
snapshot after
if cmp -s "$W/snapshots/before.txt" "$W/snapshots/after.txt"; then
  note "unchanged: real prefix kegs and links, real tap, installed tap, tap remote, owner HOME ($(sha "$W/snapshots/after.txt"))"
else
  diff "$W/snapshots/before.txt" "$W/snapshots/after.txt" || true
  fail 'a protected subject changed'
fi
show "rehearsal passed"

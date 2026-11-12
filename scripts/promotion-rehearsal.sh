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
#   C  B plus a candidate Grove formula, one commit that this script makes
#
# The candidate Grove is the working result of this repository under a label
# that sorts above the workspace version: the next minor with the pre-release
# suffix -rc.1, such as 22.9.0-rc.1 for 22.8.0. Homebrew sorts that label
# above 22.8.0 and below 22.9.0, and it reserves no release number. The
# formula test asserts that `grove --version` equals the formula version, so
# the binaries must carry the label. The script copies the files that a commit
# of the working copy would hold into WORK/candidate-src. It sets the label in
# that copy's Cargo.toml and Cargo.lock, and runs `release-build.sh
# --unreleased` there for every target. It renders the copy's
# scripts/templates/grove.rb.tmpl with those archives through file:// URLs. It
# commits the formula to a clone of the scratch tap at state B and pushes it
# to the scratch origin. The real working copy and the real tap are only read.
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
# At state C, a usable pair also has native setup and directory setup. One
# fixture HOME holds policy.ts and config.rhai and no settings.json. Native
# setup launches Grove with the prefix's bin first on PATH, so the default
# ravel is 1.1.0 and reads policy.ts. Directory setup puts ravel@2's bin
# first, as Grove's caveats direct. It reads config.rhai and delivers the
# skill sources of the candidate keg with no settings.
#
# Scenarios: fresh-a, fresh-b, upgrade-a-b, fresh-c, upgrade-a-c, upgrade-b-c.
# k335 of the grove adds state D.
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
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
[[ -d "$TAP_WC/Formula" && -d "$TAP_WC/.git" ]] || fail "$TAP_WC is not a colocated tap working copy"
case "$W/" in
  "$TAP_WC"/* | "$REPO"/*) fail 'the work directory lies inside the tap or this repository' ;;
esac

STATE_A=12b53f8c449dba322c95d869e2c02bb601b7fa1b
STATE_B=f7f9b9b0bc4d6353dcd8cd41fb47deabafc14c09
STATE_C='' # the candidate commit, which build_candidate makes
CANDIDATE_TARGETS=(aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu)
BASE_VERSION="$(sed -n '/^\[workspace.package\]$/,/^\[/s/^version = "\(.*\)"$/\1/p' "$REPO/Cargo.toml")"
[[ "$BASE_VERSION" =~ ^([0-9]+)\.([0-9]+)\.[0-9]+$ ]] || fail "cannot read the workspace version ('$BASE_VERSION')"
LABEL="${BASH_REMATCH[1]}.$((BASH_REMATCH[2] + 1)).0-rc.1"
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
    echo "### this repository's version files"
    sha "$REPO/Cargo.toml"
    sha "$REPO/Cargo.lock"
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

# git with no owner configuration, for the scratch tap's candidate commit.
git_s() {
  env -i PATH=/usr/bin:/bin HOME="$W/front-home" GIT_CONFIG_NOSYSTEM=1 \
    GIT_AUTHOR_NAME=promotion-rehearsal GIT_AUTHOR_EMAIL=rehearsal@invalid \
    GIT_COMMITTER_NAME=promotion-rehearsal GIT_COMMITTER_EMAIL=rehearsal@invalid git "$@"
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
pair_ok() { # STEP VERSION: the version that bin/grove must report
  local link version
  run test linkuistics/taps/ravel
  run test linkuistics/taps/grove
  version="$(env -i PATH=/usr/bin:/bin "$P/bin/grove" --version)"
  [[ "$version" == "grove $2" ]] || fail "$1: bin/grove reports '$version'"
  note "$1: bin/grove reports $version"
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
  # --formula: with two kegs in one rack, Homebrew 7.0.13 leaves the shell
  # fast path of Library/Homebrew/list.sh for cmd/list.rb, which also lists
  # casks and fails on a prefix without a Caskroom.
  run list --formula --versions
  # brew test turned developer mode on; the next mutation runs in the default mode.
  run developer off
}

retained() { # NAME [KEG]: manifest of a scratch keg, by default Ravel 1.1.0, without Homebrew's receipt
  tree_manifest "$P/Cellar/${2:-ravel/1.1.0}" | grep -v '^INSTALL_RECEIPT.json ' >"$W/snapshots/$1.keg"
}
same_keg() { # STEP BEFORE AFTER KEG
  if cmp -s "$W/snapshots/$2.keg" "$W/snapshots/$3.keg"; then
    note "$1: the retained $4 keg is unchanged apart from INSTALL_RECEIPT.json"
  else
    diff "$W/snapshots/$2.keg" "$W/snapshots/$3.keg" || true
    fail "$1: the retained $4 keg changed"
  fi
}

# One Grove launch in a fresh fixture tree, with only PATH_PREFIX, /usr/bin
# and /bin on PATH.
launch() { # SCENARIO NAME PATH_PREFIX HOME
  local t="$W/$1/launch-$2" status=0
  mkdir -p "$t/.jj/repo" "$t/.grove"
  printf '# fixture\n' >"$t/.grove/_BRIEF.md"
  printf '# work\n' >"$t/.grove/01-impl--work-k1.md"
  note "\$ PATH=$3:/usr/bin:/bin grove   # in $t"
  (cd "$t" && env -i PATH="$3:/usr/bin:/bin" HOME="$4" USER="$USER" LOGNAME="$USER" TMPDIR="$W/$1/setup-tmp" \
    grove) >"$t/grove.out" 2>&1 || status=$?
  sed 's/^/  /' "$t/grove.out"
  note "[exit $status]"
  if [[ "$status" != 0 ]] || ! grep -q 'loop complete' "$t/grove.out"; then
    fail "$1: the $2 launch did not complete"
  fi
  [[ -e "$t/.grove/01-impl--work-k1.md" ]] || fail "$1: the $2 launch removed the leaf"
}

# Native setup and directory setup at state C, from one fixture HOME with
# policy.ts and config.rhai and no settings.json. Each route launches the
# fixture agent with the ravel that evaluated it and the delivered prompt. The
# agent records both, and the RAVEL_SKILL_DIRECTORIES that it inherited. It
# then records a teardown and ends the session, so the loop completes.
setups() { # SCENARIO
  local s="$W/$1" assets="$P/opt/grove/libexec/ravel-policy" v1="$P/opt/ravel/bin/ravel"
  local v2="$P/opt/ravel@2/bin/ravel" home="$W/$1/setup-home" t
  mkdir -p "$home/.config/ravel" "$home/.codex" "$s/setup-tmp"
  cat >"$s/agent.sh" <<'SH'
#!/bin/sh
printf '%s\n' "$1" >ravel.txt
printf '%s' "$2" >prompt.txt
printf '%s\n' "${RAVEL_SKILL_DIRECTORIES:-unset}" >skills-env.txt
[ -z "${RAVEL_SKILL_DIRECTORIES:-}" ] || cp -R "$RAVEL_SKILL_DIRECTORIES" descriptor
touch "$GROVE_LAUNCH_DIR/teardown"
exec "$1" end-session grove-continue
SH
  chmod +x "$s/agent.sh"
  cat >"$home/.config/ravel/policy.ts" <<TS
import { definePolicy } from "ravel/sdk";
import { withProvenance } from "$assets/provenance.ts";
import { groveReviewSelector } from "$assets/examples/grove-review.ts";
const route = request => ({ program: "/bin/sh", args: ["$s/agent.sh", "$v1", request.prompt],
  provider: "fixture", model: "none", effort: "none" });
export const policy = withProvenance(definePolicy({ schemaVersion: 2,
  version: "rehearsal", ...groveReviewSelector({ routes: { impl: route }, reviews: {} }) }));
TS
  cat >"$home/.config/ravel/config.rhai" <<RHAI
import "$assets/examples/grove-review" as grove_review;
import "$assets/provenance" as provenance;
export const SCHEMA_VERSION = 4;
export const VERSION = "rehearsal";
fn route() {
    |request| #{ program: "/bin/sh", args: ["$s/agent.sh", "$v2", request.prompt],
        provider: "fixture", model: "none", effort: "none" }
}
const RULES = #{ routes: #{ impl: route() }, reviews: #{} };
fn load_context(request) { grove_review::load_context_with(global::RULES, request) }
fn select(request, context) {
    provenance::with_provenance(grove_review::select_with(global::RULES, request, context),
        global::VERSION, request)
}
RHAI
  [[ ! -e "$home/.config/ravel/settings.json" ]] || fail "$1: the fixture HOME holds settings.json"
  note "$1: fixture HOME files: $(cd "$home" && find . -type f | LC_ALL=C sort | tr '\n' ' ')"

  launch "$1" native "$P/bin" "$home"
  t="$s/launch-native"
  [[ "$(cat "$t/ravel.txt")" == "$v1" ]] || fail "$1: native setup ran $(cat "$t/ravel.txt")"
  [[ "$(cat "$t/skills-env.txt")" == unset ]] || fail "$1: native setup inherited RAVEL_SKILL_DIRECTORIES"
  if grep -q 'BEGIN RAVEL CALLER TASK' "$t/prompt.txt"; then fail "$1: native setup received a prepared prompt"; fi
  note "$1: native setup: Ravel 1.1.0 read policy.ts; no RAVEL_SKILL_DIRECTORIES, and Grove's prompt is not wrapped"

  launch "$1" directory "$P/opt/ravel@2/bin:$P/bin" "$home"
  t="$s/launch-directory"
  [[ "$(cat "$t/ravel.txt")" == "$v2" ]] || fail "$1: directory setup ran $(cat "$t/ravel.txt")"
  [[ "$(cat "$t/skills-env.txt")" != unset && -e "$t/descriptor" ]] || fail "$1: directory setup received no descriptor"
  grep -q 'BEGIN RAVEL CALLER TASK' "$t/prompt.txt" || fail "$1: directory setup received no prepared prompt"
  grep -qF "$(cat "$t/skills-env.txt")" "$t/prompt.txt" || fail "$1: the prepared prompt does not name the descriptor"
  env -i PATH=/usr/bin:/bin HOME="$home" TMPDIR="$s/setup-tmp" "$v2" skills resolve --json >"$s/resolve.json"
  python3 -I - "$s/resolve.json" "$P/Cellar/grove/$LABEL/libexec/skill-sources" <<'PY' ||
import json, os, sys
resolved = json.load(open(sys.argv[1]))
keg = os.path.realpath(sys.argv[2])
ids = [source["id"] for source in resolved["sources"]]
assert resolved["status"] == "resolved", resolved["status"]
assert ids == sorted(os.listdir(keg)) == ["design", "engineering", "grove-core", "requirements", "research", "writing"], ids
for source in resolved["sources"]:
    assert source["originPath"] == os.path.join(keg, source["id"]), source["originPath"]
print("resolved:", " ".join(ids), "from", keg)
PY
    fail "$1: ravel@2 does not resolve the candidate keg's sources"
  note "$1: directory setup: ravel@2 read config.rhai and delivered a prepared prompt naming $(cat "$t/skills-env.txt"), with no settings"
}

# The candidate Grove: unreleased archives of the working result under LABEL,
# and the formula rendered from the template with file:// URLs. Then state C.
build_candidate() {
  local src="$W/candidate-src" out="$W/candidate" target upper sha changed sed_candidate
  local sed_release=(-e "s|@VERSION@|$LABEL|g")
  mkdir -p "$src/target" "$out"
  GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 git -C "$REPO" ls-files -z --cached --others --exclude-standard |
    while IFS= read -r -d '' f; do
      [[ ! -e "$REPO/$f" && ! -L "$REPO/$f" ]] || printf '%s\0' "$f"
    done | tar -C "$REPO" --null -n -T - -cf - | tar -C "$src" -xf -
  # The build lists core's files through Git, so the copy is a repository.
  GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 git -C "$src" init -q
  # Copy-on-write clones of the reusable build state: the plain pack
  # checkouts, and each target's dependency builds. Cargo rebuilds what its
  # fingerprints reject, including every crate of this workspace.
  for target in external-packs "${CANDIDATE_TARGETS[@]}"; do
    [[ ! -d "$REPO/target/$target" ]] || cp -cR "$REPO/target/$target" "$src/target/"
  done
  changed="$(
    python3 -I - "$src" "$BASE_VERSION" "$LABEL" <<'PY'
import re, sys
src, old, new = sys.argv[1:]
line = re.compile('^version = "%s"$' % re.escape(old), re.M)
path = src + "/Cargo.toml"
text = open(path).read()
head, mark, rest = text.partition("\n[workspace.package]\n")
block, nxt, tail = rest.partition("\n[")
block, count = line.subn('version = "%s"' % new, block, count=1)
assert mark and count == 1, "no workspace version"
open(path, "w").write(head + mark + block + nxt + tail)
path = src + "/Cargo.lock"
blocks = open(path).read().split("\n[[package]]\n")
names = []
for i, b in enumerate(blocks[1:], 1):
    if re.search("^source = ", b, re.M):
        continue
    blocks[i], count = line.subn('version = "%s"' % new, b, count=1)
    if count:
        names.append(re.search('^name = "(.*)"$', b, re.M).group(1))
open(path, "w").write("\n[[package]]\n".join(blocks))
print(" ".join(names))
PY
  )" || fail 'cannot set the candidate label in the copy'
  note "candidate copy: workspace version $BASE_VERSION -> $LABEL in Cargo.toml; Cargo.lock members: $changed"
  note "\$ bash scripts/release-build.sh --unreleased $(IFS=' ' && echo "${CANDIDATE_TARGETS[*]}")   # in $src"
  env -i PATH="$PATH" HOME="$HOME" USER="$USER" LOGNAME="$USER" TERM=dumb MISE_TRUSTED_CONFIG_PATHS="$src" \
    bash "$src/scripts/release-build.sh" --unreleased "${CANDIDATE_TARGETS[@]}" >"$W/candidate-build.log" 2>&1 ||
    {
      tail -40 "$W/candidate-build.log"
      fail 'the candidate build failed'
    }
  grep '^release-build: ' "$W/candidate-build.log" | grep -v '^release-build: cargo '
  for target in "${CANDIDATE_TARGETS[@]}"; do
    cp "$src/target/archives/grove-v$LABEL-$target.tar.xz" "$out/"
    sha="$(sha "$out/grove-v$LABEL-$target.tar.xz")"
    upper="$(tr 'a-z-' 'A-Z_' <<<"$target")"
    sed_release+=(-e "s|@SHA_${upper}@|$sha|g")
    note "candidate archive: grove-v$LABEL-$target.tar.xz $sha"
  done
  sed_candidate=(-e "s|https://github.com/Linkuistics/grove/releases/download/v@VERSION@/|file://$out/|g" "${sed_release[@]}")
  sed "${sed_release[@]}" "$src/scripts/templates/grove.rb.tmpl" >"$W/snapshots/grove.release.rb"
  sed "${sed_candidate[@]}" "$src/scripts/templates/grove.rb.tmpl" >"$out/grove.rb"
  if grep -n '@[A-Z_0-9]*@\|github.com/Linkuistics/grove/releases' "$out/grove.rb"; then
    fail 'the candidate formula holds a placeholder or a release URL'
  fi
  # The candidate differs from the release rendering in its three URLs only.
  diff "$W/snapshots/grove.release.rb" "$out/grove.rb" >"$W/snapshots/grove.diff" || true
  [[ "$(grep -c '^> ' "$W/snapshots/grove.diff")" == 3 && "$(grep -c '^> *url "file://' "$W/snapshots/grove.diff")" == 3 &&
    "$(grep -c '^< ' "$W/snapshots/grove.diff")" == 3 ]] || {
    cat "$W/snapshots/grove.diff"
    fail 'the candidate formula differs from the release rendering beyond its URLs'
  }
  grep '^> ' "$W/snapshots/grove.diff"
  note "candidate formula: $out/grove.rb $(sha "$out/grove.rb")"

  # State C: the candidate formula, committed on top of state B and pushed to
  # the scratch origin.
  set_origin "$STATE_B"
  git_s clone -q -b main "$W/origin.git" "$W/tap-candidate"
  cp "$out/grove.rb" "$W/tap-candidate/Formula/grove.rb"
  git_s -C "$W/tap-candidate" commit -q -m "grove $LABEL: candidate formula for the promotion rehearsal" Formula/grove.rb
  note "\$ git push origin HEAD:main   # in $W/tap-candidate, origin $(git_s -C "$W/tap-candidate" remote get-url origin)"
  git_s -C "$W/tap-candidate" push -q origin HEAD:main
  STATE_C="$(git -C "$W/origin.git" rev-parse refs/heads/main)"
  [[ "$STATE_C" == "$(git_s -C "$W/tap-candidate" rev-parse HEAD)" &&
    "$(git -C "$W/origin.git" rev-parse "$STATE_C^")" == "$STATE_B" &&
    "$(git -C "$W/origin.git" diff --name-only "$STATE_B" "$STATE_C")" == Formula/grove.rb ]] ||
    fail 'state C is not one commit on state B that changes only Formula/grove.rb'
  note "state C: $STATE_C on state B, changing only Formula/grove.rb"
}

# The upgrade to state C. Homebrew must list the outdated Grove.
upgrade_to_c() { # STEP
  set_origin "$STATE_C"
  update_tap
  run outdated --verbose || true
  brew_s outdated --quiet | grep -qE '(^|/)grove$' || fail "$1: brew outdated does not list grove at state C"
  run upgrade
  note "$1: Grove kegs after the upgrade: $(cd "$P/Cellar/grove" && printf '%s ' *)"
}

show "rehearsal inputs"
note "work: $W"
note "tap working copy: $TAP_WC"
note "owner Homebrew: $OWNER_BREW at $OWNER_HEAD ($(brew --version | head -1))"
note "real front: $REAL_FRONT"
note "candidate source: $REPO, workspace version $BASE_VERSION, candidate label $LABEL"
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

show "candidate: Grove $LABEL from the working result of $REPO"
build_candidate

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
pair_ok fresh-a 22.8.0

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
pair_ok fresh-b 22.8.0

show "upgrade-a-b: an upgrade from tap state A to tap state B"
set_origin "$STATE_A"
make_brew upgrade-a-b
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts upgrade-a-b:install-a
pair_ok upgrade-a-b:a 22.8.0
retained upgrade-a-b-a
set_origin "$STATE_B"
update_tap
run outdated --verbose || true
run upgrade
fronts upgrade-a-b:upgrade
note 'ravel 1.1.0 is installed, so the opt-in ravel@2 installation needs no --skip-link.'
run install linkuistics/taps/ravel@2
fronts upgrade-a-b:install-ravel@2
pair_ok upgrade-a-b:b 22.8.0
retained upgrade-a-b-b
same_keg upgrade-a-b upgrade-a-b-a upgrade-a-b-b 'ravel 1.1.0'

show "fresh-c: a fresh install at tap state C"
set_origin "$STATE_C"
make_brew fresh-c
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts fresh-c:install
run install linkuistics/taps/ravel@2
fronts fresh-c:install-ravel@2
pair_ok fresh-c "$LABEL"
setups fresh-c

show "upgrade-a-c: an upgrade from tap state A to tap state C"
set_origin "$STATE_A"
make_brew upgrade-a-c
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts upgrade-a-c:install-a
pair_ok upgrade-a-c:a 22.8.0
retained upgrade-a-c-a
upgrade_to_c upgrade-a-c
fronts upgrade-a-c:upgrade
run install linkuistics/taps/ravel@2
fronts upgrade-a-c:install-ravel@2
pair_ok upgrade-a-c:c "$LABEL"
setups upgrade-a-c
retained upgrade-a-c-c
same_keg upgrade-a-c upgrade-a-c-a upgrade-a-c-c 'ravel 1.1.0'

show "upgrade-b-c: an upgrade from tap state B to tap state C"
set_origin "$STATE_B"
make_brew upgrade-b-c
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts upgrade-b-c:install-b
run install linkuistics/taps/ravel@2
fronts upgrade-b-c:install-ravel@2
pair_ok upgrade-b-c:b 22.8.0
retained upgrade-b-c-b
retained upgrade-b-c-b2 ravel@2/2.0.0
upgrade_to_c upgrade-b-c
fronts upgrade-b-c:upgrade
pair_ok upgrade-b-c:c "$LABEL"
setups upgrade-b-c
retained upgrade-b-c-c
retained upgrade-b-c-c2 ravel@2/2.0.0
same_keg upgrade-b-c upgrade-b-c-b upgrade-b-c-c 'ravel 1.1.0'
same_keg upgrade-b-c upgrade-b-c-b2 upgrade-b-c-c2 'ravel@2 2.0.0'

show "protected subjects"
snapshot after
if cmp -s "$W/snapshots/before.txt" "$W/snapshots/after.txt"; then
  note "unchanged: real prefix kegs and links, real tap, installed tap, tap remote, owner HOME ($(sha "$W/snapshots/after.txt"))"
else
  diff "$W/snapshots/before.txt" "$W/snapshots/after.txt" || true
  fail 'a protected subject changed'
fi
show "rehearsal passed"

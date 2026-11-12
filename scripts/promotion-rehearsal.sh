#!/usr/bin/env bash
# The promotion upgrade rehearsal: fresh installs and upgrades through the tap
# states of docs/specs/coordinated-release.md, "Compatible tap states", with
# real Homebrew in a scratch environment.
#
#   promotion-rehearsal.sh EMPTY_WORK_DIR [TAP_WORKING_COPY [RAVEL_WORKSPACE]]
#
# TAP_WORKING_COPY is the real tap's colocated working copy, by default
# ~/Development/Delivery/homebrew-taps. RAVEL_WORKSPACE is Ravel's default
# colocated workspace, by default ~/Development/AgenticOrchestration/Ravel.
# The instrument only reads both. docs/promotion-upgrade-rehearsal-record.md
# records each run and its decisions.
#
# The scratch tap is a bare clone of the real tap's Git history,
# WORK/origin.git. Its `main` names the tap state of a scenario:
#
#   A  12b53f8c449d, Grove 22.8.0 and unversioned Ravel 1.1.0
#   B  f7f9b9b0bc4d, A plus the keg-only ravel@2 2.0.0
#   C  B plus a candidate Grove formula, one commit that this script makes
#   D  C plus the promotion commit that Ravel's release:promote makes
#
# The candidate Grove is the working result of this repository under the next
# minor version, such as 22.9.0 for 22.8.0. Ravel's readiness receipt admits
# only an X.Y.Z consumer version, and the tap's Grove formula must carry it, so
# a pre-release label cannot reach state D. The label stays in the scratch tap
# and in WORK, so it publishes and reserves no public version. The formula
# test asserts that `grove --version` equals the formula version, so the
# binaries must carry the label. The script copies the files that a commit of
# the working copy would hold into WORK/candidate-src. It sets the label in
# that copy's Cargo.toml and Cargo.lock, and runs `release-build.sh
# --unreleased` there for every target. It renders the copy's
# scripts/templates/grove.rb.tmpl with those archives through file:// URLs. It
# commits the formula to a clone of the scratch tap at state B and pushes it
# to the scratch origin. The real working copy and the real tap are only read.
#
# State D comes from `task release:promote VERSION=2.0.0` in WORK/ravel, a
# colocated jj clone of Ravel's main. The promotion writes its evidence there,
# never in Ravel's workspace. The clone fetches the tag, the Release and the
# book from Ravel's GitHub origin, and its push URL names no repository.
# RAVEL_TAP_DIR is WORK/tap-promote, a colocated jj clone of the scratch
# origin at state C. The readiness receipt names the candidate Grove, and the
# evidence is a copy of the cut's evidence from RAVEL_WORKSPACE. The task runs
# with the private Homebrew of the upgrade-c-d scenario first on PATH, through
# a wrapper that gives Homebrew that scenario's HOME. It downloads the
# published 2.0.0 assets and pushes only to the scratch origin.
#
# Each scenario has its own private Homebrew: a --shared clone of the owner's
# Homebrew repository at its HEAD, with its own prefix, HOME, logs and
# temporary directory. All scenarios share one download cache. Each private
# Homebrew taps WORK/origin.git as linkuistics/taps, because the formulae name
# their dependencies there. A tap state moves when the scenario fetches the
# tap's origin and fast-forwards, which is what `brew update` does for a tap.
# The scenario does not run `brew update`, because that would also move the
# private Homebrew away from the owner's version. Ravel's release-install.sh
# runs `brew update` itself. In Homebrew's default mode, that update moves to
# the latest stable tag. The owner's Homebrew has developer mode on, which
# moves to origin/main, the owner's commit. The promotion scenario therefore
# turns developer mode on first, and fails if Homebrew's commit moves.
#
# The scratch HOME holds a copy of the owner's ~/.homebrew/brew.env, and every
# package mutation sets HOMEBREW_NO_INSTALL_CLEANUP=1, as the owner's release
# tasks do. The rehearsal therefore measures the owner's path. An ordinary
# user's automatic cleanup is outside this measurement.
#
# After each package mutation, the 1.1.0 front of the scratch prefix and the
# real 1.1.0 front must report `ravel 1.1.0` and refuse `end-session` outside a
# session. A fresh install at D has no 1.1.0 keg, so only the real front is
# checked there. Ravel 1.1.0 finds its session through RAVEL_EXIT_FILE, and
# every command here runs under `env -i`, so no inherited session authority
# reaches it. The real prefix, the real tap and the owner's HOME are digested
# before and after the run, and the run fails if any digest changes.
#
# At state C, a usable pair also has native setup and directory setup. One
# fixture HOME holds policy.ts and config.rhai and no settings.json. Native
# setup launches Grove with the prefix's bin first on PATH, so the default
# ravel is 1.1.0 and reads policy.ts. Directory setup puts ravel@2's bin
# first, as Grove's caveats direct. It reads config.rhai and delivers the
# skill sources of the candidate keg with no settings.
#
# At state D, the default ravel is 2.0.0. Each upgrade to D gives a HOME that
# holds only policy.ts. Ravel must refuse it with config_format_retired, exit 3
# and the remedy that names chapter 4 of the book, and Grove must show that
# refusal and keep the leaf. After the conversion to config.rhai, a launch
# through the default ravel delivers the candidate keg's skills. A fresh
# install at D copies Grove's config.rhai sample, as the caveats direct, and
# launches it with a fake claude first on PATH.
#
# After the scenarios, the six promotion controls of the coordinated-release
# specification's "Acceptance and limits" run against the same scratch origin,
# Ravel clone and prefixes. Each runs the Ravel entry point that owns its
# guard and must refuse: a wrong channel, a versioned-2 cut after promotion,
# mismatched archives, silent schema-3 reuse, a removed retained resource, and
# promotion before compatible Grove. A refusal must leave the scratch origin,
# the tap clone and the Ravel clone's main unchanged. A stub brew shows that
# the early refusals run no Homebrew command.
#
# Scenarios: fresh-a, fresh-untrusted, fresh-b, upgrade-a-b, fresh-c, upgrade-a-c, upgrade-b-c,
# upgrade-c-d (the promotion), fresh-d, upgrade-a-d.
set -euo pipefail
IFS=$'\n\t'
fail() {
  echo "promotion-rehearsal: $*" >&2
  exit 1
}
[[ $# -ge 1 && $# -le 3 ]] || fail 'usage: promotion-rehearsal.sh EMPTY_WORK_DIR [TAP_WORKING_COPY [RAVEL_WORKSPACE]]'
[[ -d "$1" && -z "$(ls -A "$1")" ]] || fail "$1 is not an empty directory"
W="$(cd "$1" && pwd -P)"
TAP_WC="$(cd "${2:-$HOME/Development/Delivery/homebrew-taps}" && pwd -P)"
RAVEL_WS="$(cd "${3:-$HOME/Development/AgenticOrchestration/Ravel}" && pwd -P)"
REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
[[ -d "$TAP_WC/Formula" && -d "$TAP_WC/.git" ]] || fail "$TAP_WC is not a colocated tap working copy"
[[ -f "$RAVEL_WS/scripts/release-promote.sh" && -d "$RAVEL_WS/.jj" ]] || fail "$RAVEL_WS is not Ravel's jj workspace"
case "$W/" in
  "$TAP_WC"/* | "$REPO"/* | "$RAVEL_WS"/*) fail 'the work directory lies inside the tap, Ravel or this repository' ;;
esac

STATE_A=12b53f8c449dba322c95d869e2c02bb601b7fa1b
STATE_B=f7f9b9b0bc4d6353dcd8cd41fb47deabafc14c09
STATE_C='' # the candidate commit, which build_candidate makes
STATE_D='' # the promotion commit, which promote makes
RAVEL_VERSION=2.0.0
RAVEL_EVIDENCE="$RAVEL_WS/target/release-evidence/v$RAVEL_VERSION.json"
[[ -f "$RAVEL_EVIDENCE" ]] || fail "the cut's evidence $RAVEL_EVIDENCE is missing"
CANDIDATE_TARGETS=(aarch64-apple-darwin aarch64-unknown-linux-gnu x86_64-unknown-linux-gnu)
BASE_VERSION="$(sed -n '/^\[workspace.package\]$/,/^\[/s/^version = "\(.*\)"$/\1/p' "$REPO/Cargo.toml")"
[[ "$BASE_VERSION" =~ ^([0-9]+)\.([0-9]+)\.[0-9]+$ ]] || fail "cannot read the workspace version ('$BASE_VERSION')"
LABEL="${BASH_REMATCH[1]}.$((BASH_REMATCH[2] + 1)).0"
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
# the real tap's working copy, its installed clone and its remote, the
# owner's Homebrew and Ravel configuration, and Ravel's main and release
# evidence.
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
    echo "### Ravel workspace: main and release evidence"
    git -C "$RAVEL_WS" rev-parse refs/heads/main
    tree_manifest "$RAVEL_WS/target/release-evidence"
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
  # A brew command for other tools' PATH: brew_s as a file.
  mkdir -p "$s/wrap"
  {
    echo '#!/bin/sh'
    printf 'exec env -i PATH=%q HOME=%q USER=%q LOGNAME=%q ' "$P/bin:/usr/bin:/bin" "$H" "$USER" "$USER"
    printf 'HOMEBREW_NO_AUTO_UPDATE=1 HOMEBREW_NO_ANALYTICS=1 HOMEBREW_NO_ENV_HINTS=1 '
    printf 'HOMEBREW_NO_INSTALL_CLEANUP=1 HOMEBREW_CACHE=%q HOMEBREW_LOGS=%q HOMEBREW_TEMP=%q ' \
      "$W/cache" "$s/logs" "$s/tmp"
    printf '%q "$@"\n' "$P/bin/brew"
  } >"$s/wrap/brew"
  chmod +x "$s/wrap/brew"
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
# raise_untrusted!) at the owner's HEAD, and
# https://docs.brew.sh/Tap-Trust#installing-from-a-tap. Each scenario trusts
# the tap after tapping with the documented command, which names the tap
# without --tap. fresh-a first records the refusal, and the fresh installs
# use the short name `grove`, as README.md's "Install Grove" does.
# fresh-untrusted installs `ravel` and `personal` by their fully qualified
# names with no tap trust, as Grove's missing-Ravel remedy and the hook
# adapter table of docs/USAGE.md do. fresh-b first records that a fully
# qualified `ravel@2` refuses without tap trust:
# FormulaInstaller#auto_link_versioned_keg_only? loads the unversioned
# `ravel`, also with --skip-link.
tap_scratch() {
  run tap linkuistics/taps "file://$W/origin.git"
}
trusted_taps() { # the taps in the scenario's trust store, as one JSON line
  brew_s trust --tap --json=v1 | tr -d ' \n'
}
trust_scratch() {
  local taps
  taps="$(trusted_taps)"
  [[ "$taps" == '[]' ]] || fail "a scenario's trust store held taps before the trust step: $taps"
  run trust linkuistics/taps
  taps="$(trusted_taps)"
  note "trusted taps: $taps"
  [[ "$taps" == *origin.git* ]] || fail "brew trust linkuistics/taps left the trust store without the scratch tap: $taps"
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
fronts() { # STEP [real]: with "real", the prefix has no 1.1.0 keg to check
  if [[ "${2:-}" == real ]]; then
    [[ ! -e "$P/Cellar/ravel/1.1.0" ]] || fail "$1: the prefix holds a 1.1.0 keg"
    note "$1: the prefix has no 1.1.0 keg"
  else
    front_ok "$1" "$P/Cellar/ravel/1.1.0/bin/ravel"
  fi
  front_ok "$1" "$REAL_FRONT"
}

# A usable pair: both formula tests pass, the default ravel is the keg of
# RAVEL, and ravel@2, when installed, is keg-only at 2.0.0 and unlinked. With
# the default ravel at 2.0.0, ravel@2 must also be deprecated.
pair_ok() { # STEP VERSION [RAVEL]: the versions of bin/grove and the default ravel (1.1.0)
  local link version want="${3:-1.1.0}"
  run test linkuistics/taps/ravel
  run test linkuistics/taps/grove
  version="$(env -i PATH=/usr/bin:/bin "$P/bin/grove" --version)"
  [[ "$version" == "grove $2" ]] || fail "$1: bin/grove reports '$version'"
  note "$1: bin/grove reports $version"
  link="$(readlink "$P/bin/ravel")"
  [[ "$link" == "../Cellar/ravel/$want/bin/ravel" ]] || fail "$1: bin/ravel -> $link"
  [[ "$(env -i PATH="$P/bin:/usr/bin:/bin" ravel --version)" == "ravel $want" ]] ||
    fail "$1: the default ravel is not $want"
  note "$1: default PATH ravel: bin/ravel -> $link"
  if [[ -d "$P/Cellar/ravel@2" ]]; then
    run test linkuistics/taps/ravel@2
    [[ "$("$P/opt/ravel@2/bin/ravel" --version)" == 'ravel 2.0.0' ]] || fail "$1: opt/ravel@2 is not 2.0.0"
    if find "$P/bin" "$P/share" -type l -lname '*Cellar/ravel@2/*' | grep -q .; then
      fail "$1: ravel@2 is linked into the shared prefix"
    fi
    note "$1: ravel@2 is keg-only at 2.0.0: opt/ravel@2 -> $(readlink "$P/opt/ravel@2"), no shared links"
    if [[ "$want" == 2.* ]]; then
      brew_s info --json=v2 linkuistics/taps/ravel@2 | python3 -I -c '
import json, sys
f = json.load(sys.stdin)["formulae"][0]
assert f["deprecated"] is True, f["deprecated"]
assert f["deprecation_replacement_formula"] == "ravel", f.get("deprecation_replacement_formula")
print("ravel@2 is deprecated since", f["deprecation_date"], "in favour of", f["deprecation_replacement_formula"])
' || fail "$1: ravel@2 is not deprecated in favour of ravel"
    fi
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
  sha "$P/Cellar/${2:-ravel/1.1.0}/INSTALL_RECEIPT.json" >"$W/snapshots/$1.receipt"
}
same_keg() { # STEP BEFORE AFTER KEG
  local receipt=unchanged
  cmp -s "$W/snapshots/$2.receipt" "$W/snapshots/$3.receipt" || receipt=rewritten
  if cmp -s "$W/snapshots/$2.keg" "$W/snapshots/$3.keg"; then
    note "$1: the retained $4 keg is unchanged apart from INSTALL_RECEIPT.json, which Homebrew left $receipt"
  else
    diff "$W/snapshots/$2.keg" "$W/snapshots/$3.keg" || true
    fail "$1: the retained $4 keg changed"
  fi
}

# State D publishes one release twice: the two formulae must name the same
# three archives, each by its URL and its SHA-256.
same_archives() { # FORMULA FORMULA
  local a b
  a="$(grep -E '^ +(url|sha256) "' "$1" | sed 's/^ *//')"
  b="$(grep -E '^ +(url|sha256) "' "$2" | sed 's/^ *//')"
  [[ "$(grep -c . <<<"$a")" == 6 && "$a" == "$b" ]]
}

# One Grove launch in a fresh fixture tree, with only PATH_PREFIX, /usr/bin
# and /bin on PATH.
launch() { # SCENARIO NAME PATH_PREFIX HOME [refused [CODE REMEDY]]
  local t="$W/$1/launch-$2" status=0 code="${6:-config_format_retired}"
  local remedy="${7:-remedy: write config.rhai using chapter 4 of the book}"
  mkdir -p "$t/.jj/repo" "$t/.grove"
  printf '# fixture\n' >"$t/.grove/_BRIEF.md"
  printf '# work\n' >"$t/.grove/01-impl--work-k1.md"
  note "\$ PATH=$3:/usr/bin:/bin grove   # in $t"
  (cd "$t" && env -i PATH="$3:/usr/bin:/bin" HOME="$4" USER="$USER" LOGNAME="$USER" TMPDIR="$W/$1/setup-tmp" \
    grove) >"$t/grove.out" 2>&1 || status=$?
  sed 's/^/  /' "$t/grove.out"
  note "[exit $status]"
  if [[ "${5:-}" == refused ]]; then
    # Grove reports Ravel's refusal and its status, keeps the leaf and exits 0.
    if [[ "$status" != 0 ]] || ! grep -qF "refused ($code" "$t/grove.out" ||
      ! grep -qF -- "$remedy" "$t/grove.out" ||
      ! grep -qF 'status exit status: 3' "$t/grove.out"; then
      fail "$1: the $2 launch did not show the refusal"
    fi
  elif [[ "$status" != 0 ]] || ! grep -q 'loop complete' "$t/grove.out"; then
    fail "$1: the $2 launch did not complete"
  fi
  [[ -e "$t/.grove/01-impl--work-k1.md" ]] || fail "$1: the $2 launch removed the leaf"
}

# The fixture agent of a scenario. Each route launches it with the ravel that
# the route names and the delivered prompt. The agent records both, and the
# RAVEL_SKILL_DIRECTORIES that it inherited. It then records a teardown and
# ends the session, so the loop completes.
write_agent() { # SCENARIO
  mkdir -p "$W/$1/setup-tmp"
  cat >"$W/$1/agent.sh" <<'SH'
#!/bin/sh
printf '%s\n' "$1" >ravel.txt
printf '%s' "$2" >prompt.txt
printf '%s\n' "${RAVEL_SKILL_DIRECTORIES:-unset}" >skills-env.txt
[ -z "${RAVEL_SKILL_DIRECTORIES:-}" ] || cp -R "$RAVEL_SKILL_DIRECTORIES" descriptor
touch "$GROVE_LAUNCH_DIR/teardown"
exec "$1" end-session grove-continue
SH
  chmod +x "$W/$1/agent.sh"
}
# Ravel 1.x configuration in HOME that routes impl to the agent with RAVEL.
write_policy_ts() { # SCENARIO HOME RAVEL
  local assets="$P/opt/grove/libexec/ravel-policy"
  mkdir -p "$2/.config/ravel" "$2/.codex"
  cat >"$2/.config/ravel/policy.ts" <<TS
import { definePolicy } from "ravel/sdk";
import { withProvenance } from "$assets/provenance.ts";
import { groveReviewSelector } from "$assets/examples/grove-review.ts";
const route = request => ({ program: "/bin/sh", args: ["$W/$1/agent.sh", "$3", request.prompt],
  provider: "fixture", model: "none", effort: "none" });
export const policy = withProvenance(definePolicy({ schemaVersion: 2,
  version: "rehearsal", ...groveReviewSelector({ routes: { impl: route }, reviews: {} }) }));
TS
}
# The same route for Ravel 2.x: the conversion of write_policy_ts.
write_config_rhai() { # SCENARIO HOME RAVEL
  local assets="$P/opt/grove/libexec/ravel-policy"
  mkdir -p "$2/.config/ravel" "$2/.codex"
  cat >"$2/.config/ravel/config.rhai" <<RHAI
import "$assets/examples/grove-review" as grove_review;
import "$assets/provenance" as provenance;
export const SCHEMA_VERSION = 4;
export const VERSION = "rehearsal";
fn route() {
    |request| #{ program: "/bin/sh", args: ["$W/$1/agent.sh", "$3", request.prompt],
        provider: "fixture", model: "none", effort: "none" }
}
const RULES = #{ routes: #{ impl: route() }, reviews: #{} };
fn load_context(request) { grove_review::load_context_with(global::RULES, request) }
fn select(request, context) {
    provenance::with_provenance(grove_review::select_with(global::RULES, request, context),
        global::VERSION, request)
}
RHAI
}

# A launch through Ravel 2 delivered the candidate keg's skills: the agent ran
# RAVEL, inherited a descriptor and received a prepared prompt that names it,
# and RAVEL resolves the six sources of the candidate keg with no settings.
delivered() { # SCENARIO LAUNCH RAVEL HOME
  local s="$W/$1" t="$W/$1/launch-$2"
  [[ "$(cat "$t/ravel.txt")" == "$3" ]] || fail "$1: the $2 launch ran $(cat "$t/ravel.txt")"
  [[ "$(cat "$t/skills-env.txt")" != unset && -e "$t/descriptor" ]] || fail "$1: the $2 launch received no descriptor"
  grep -q 'BEGIN RAVEL CALLER TASK' "$t/prompt.txt" || fail "$1: the $2 launch received no prepared prompt"
  grep -qF "$(cat "$t/skills-env.txt")" "$t/prompt.txt" || fail "$1: the $2 prompt does not name the descriptor"
  [[ ! -e "$4/.config/ravel/settings.json" ]] || fail "$1: the $2 HOME holds settings.json"
  resolves "$1" "$2" "$3" "$4"
}
# RAVEL resolves the six sources of the candidate keg in HOME with no settings.
resolves() { # SCENARIO NAME RAVEL HOME
  local s="$W/$1"
  env -i PATH=/usr/bin:/bin HOME="$4" TMPDIR="$s/setup-tmp" "$3" skills resolve --json >"$s/resolve-$2.json"
  python3 -I - "$s/resolve-$2.json" "$P/Cellar/grove/$LABEL/libexec/skill-sources" <<'PY' ||
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
    fail "$1: $3 does not resolve the candidate keg's sources"
}

# Native setup and directory setup at state C, from one fixture HOME with
# policy.ts and config.rhai and no settings.json.
setups() { # SCENARIO
  local s="$W/$1" v1="$P/opt/ravel/bin/ravel" v2="$P/opt/ravel@2/bin/ravel" home="$W/$1/setup-home" t
  write_agent "$1"
  write_policy_ts "$1" "$home" "$v1"
  write_config_rhai "$1" "$home" "$v2"
  [[ ! -e "$home/.config/ravel/settings.json" ]] || fail "$1: the fixture HOME holds settings.json"
  note "$1: fixture HOME files: $(cd "$home" && find . -type f | LC_ALL=C sort | tr '\n' ' ')"

  launch "$1" native "$P/bin" "$home"
  t="$s/launch-native"
  [[ "$(cat "$t/ravel.txt")" == "$v1" ]] || fail "$1: native setup ran $(cat "$t/ravel.txt")"
  [[ "$(cat "$t/skills-env.txt")" == unset ]] || fail "$1: native setup inherited RAVEL_SKILL_DIRECTORIES"
  if grep -q 'BEGIN RAVEL CALLER TASK' "$t/prompt.txt"; then fail "$1: native setup received a prepared prompt"; fi
  note "$1: native setup: Ravel 1.1.0 read policy.ts; no RAVEL_SKILL_DIRECTORIES, and Grove's prompt is not wrapped"

  launch "$1" directory "$P/opt/ravel@2/bin:$P/bin" "$home"
  delivered "$1" directory "$v2" "$home"
  note "$1: directory setup: ravel@2 read config.rhai and delivered a prepared prompt naming $(cat "$s/launch-directory/skills-env.txt"), with no settings"
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

# After an upgrade to D: a HOME that holds only policy.ts. Ravel 2.0.0 refuses
# it with config_format_retired, exit 3 and the remedy, and Grove shows that
# refusal and keeps the leaf. The conversion writes config.rhai with the same
# route and keeps policy.ts, as Grove's caveats direct while a Ravel 1.x launch
# remains. A launch through the default ravel then delivers the keg's skills.
retired_then_converted() { # SCENARIO
  local s="$W/$1" home="$W/$1/retired-home" v="$P/opt/ravel/bin/ravel" status=0
  write_agent "$1"
  write_policy_ts "$1" "$home" "$v"
  note "$1: policy.ts-only HOME files: $(cd "$home" && find . -type f | LC_ALL=C sort | tr '\n' ' ')"
  mkdir -p "$s/probe"
  note "\$ PATH=$P/bin:/usr/bin:/bin ravel start-session --kind impl --prompt probe"
  (cd "$s/probe" && env -i PATH="$P/bin:/usr/bin:/bin" HOME="$home" TMPDIR="$s/setup-tmp" \
    ravel start-session --kind impl --prompt probe) >"$s/retired.out" 2>&1 || status=$?
  sed 's/^/  /' "$s/retired.out"
  note "[exit $status]"
  if [[ "$status" != 3 ]] || ! grep -q '^ravel: refused (config_format_retired, stage authority)' "$s/retired.out" ||
    ! grep -qF 'remedy: write config.rhai using chapter 4 of the book, "Write a configuration script"' "$s/retired.out"; then
    fail "$1: the default ravel did not refuse the policy.ts-only HOME as documented"
  fi
  launch "$1" retired "$P/bin" "$home" refused
  [[ ! -e "$s/launch-retired/ravel.txt" ]] || fail "$1: the refused launch ran the agent"
  note "$1: Ravel 2.0.0 refused the policy.ts-only HOME with exit 3 and the remedy; Grove showed it and kept the leaf"
  write_config_rhai "$1" "$home" "$v"
  note "$1: converted HOME files: $(cd "$home" && find . -type f | LC_ALL=C sort | tr '\n' ' ')"
  launch "$1" converted "$P/bin" "$home"
  delivered "$1" converted "$v" "$home"
  note "$1: after the conversion, the default ravel read config.rhai and delivered a prepared prompt naming $(cat "$s/launch-converted/skills-env.txt"), with no settings"
}

# Grove's config.rhai sample at D, copied as the caveats direct, routes impl
# to claude by default. A fake claude first on PATH records its arguments, its
# prompt and its descriptor, then ends the session through the default ravel.
sample_launch() { # SCENARIO
  local s="$W/$1" home="$W/$1/sample-home" t="$W/$1/launch-sample"
  mkdir -p "$s/fake-bin" "$s/setup-tmp" "$home/.config/ravel" "$home/.codex"
  cp "$P/opt/grove/libexec/ravel-policy/sample/config.rhai" "$home/.config/ravel/config.rhai"
  cat >"$s/fake-bin/claude" <<'SH'
#!/bin/sh
printf '%s\n' "$@" >args.txt
for last; do :; done
printf '%s' "$last" >prompt.txt
printf '%s\n' "${RAVEL_SKILL_DIRECTORIES:-unset}" >skills-env.txt
command -v ravel >ravel.txt
touch "$GROVE_LAUNCH_DIR/teardown"
exec ravel end-session grove-continue
SH
  chmod +x "$s/fake-bin/claude"
  note "$1: sample HOME files: $(cd "$home" && find . -type f | LC_ALL=C sort | tr '\n' ' ')"
  launch "$1" sample "$s/fake-bin:$P/bin" "$home"
  [[ "$(cat "$t/ravel.txt")" == "$P/bin/ravel" ]] || fail "$1: the fake claude found $(cat "$t/ravel.txt")"
  [[ "$(cat "$t/skills-env.txt")" != unset ]] || fail "$1: the sample launch received no descriptor"
  grep -q 'BEGIN RAVEL CALLER TASK' "$t/prompt.txt" || fail "$1: the sample launch received no prepared prompt"
  grep -qxF -- --add-dir "$t/args.txt" || fail "$1: the sample granted claude no skill snapshot"
  grep -qxF claude-opus-5-5 "$t/args.txt" || fail "$1: the sample did not select its claude model"
  note "$1: the sample routed impl to claude: $(sed '$d' "$t/args.txt" | tr '\n' ' ')"
  resolves "$1" sample "$P/opt/ravel/bin/ravel" "$home"
}

# The readiness receipt of Ravel's docs/specs/release-channels.md, "Consumer
# readiness receipt", from the cut's evidence, for the consumer Grove VERSION.
write_receipt() { # EVIDENCE VERSION
  jq --arg version "$2" --arg at "$(date -u +%Y-%m-%dT%H:%M:%SZ)" --arg ravel "$RAVEL_VERSION" \
    '{schemaVersion: 1, kind: "ravel-promotion-readiness",
      ravel: {version: $ravel, tagCommit, assets},
      consumer: {name: "grove", version: $version, formula: "grove.rb", verifiedAt: $at,
        verifiedRoutes: ["native-1.1", "native-2.0-delivery-disabled", "directory-2.0"]}}' "$1"
}

# State D: Ravel's own promotion task against the scratch tap, with the
# private Homebrew of SCENARIO first on PATH. The scenario's prefix is at C.
promote() { # SCENARIO
  local s="$W/$1" r="$W/ravel" tap="$W/tap-promote" ev="$W/promotion" status=0 head_before head_after
  mkdir -p "$ev" "$W/tmp-promote"
  cp "$RAVEL_EVIDENCE" "$ev/v$RAVEL_VERSION.json"
  write_receipt "$ev/v$RAVEL_VERSION.json" "$LABEL" >"$ev/v$RAVEL_VERSION.readiness.json"
  note "readiness receipt: $(jq -c .consumer "$ev/v$RAVEL_VERSION.readiness.json")"

  # Ravel at main, in a colocated clone whose origin is the public repository
  # for reading and no repository for writing.
  jj git clone --colocate --quiet "$RAVEL_WS" "$r" 2>&1
  git -C "$r" remote set-url origin "$(git -C "$RAVEL_WS" remote get-url origin)"
  git -C "$r" remote set-url --push origin "$W/no-such-push-remote"
  note "Ravel clone: $r at main $(git -C "$r" rev-parse refs/heads/main), origin $(git -C "$r" remote get-url origin)"
  [[ "$(git -C "$r" rev-parse refs/heads/main)" == "$(git -C "$RAVEL_WS" rev-parse refs/heads/main)" ]] ||
    fail 'the Ravel clone is not at main'

  # The scratch tap as Ravel's colocated jj tap clone at state C.
  set_origin "$STATE_C"
  jj git clone --colocate --quiet "file://$W/origin.git" "$tap" 2>&1
  note "tap clone: $tap, main@origin $(jj -R "$tap" log --no-graph -r main@origin -T commit_id)"

  head_before="$(git -C "$P" rev-parse HEAD)"
  note "The owner's Homebrew has developer mode on, so brew update keeps origin/main:"
  run developer on
  note "\$ RAVEL_TAP_DIR=$tap task release:promote VERSION=$RAVEL_VERSION READINESS=… EVIDENCE=…   # in $r, with $s/wrap first on PATH"
  (cd "$r" && env -i PATH="$s/wrap:$PATH" HOME="$HOME" USER="$USER" LOGNAME="$USER" TERM=dumb \
    TMPDIR="$W/tmp-promote" MISE_TRUSTED_CONFIG_PATHS="$r" RAVEL_TAP_DIR="$tap" \
    task release:promote VERSION="$RAVEL_VERSION" READINESS="$ev/v$RAVEL_VERSION.readiness.json" \
    EVIDENCE="$ev/v$RAVEL_VERSION.json") >"$W/promotion.log" 2>&1 || status=$?
  sed 's/^/  /' "$W/promotion.log"
  note "[exit $status]"
  [[ "$status" == 0 ]] || fail "$1: release:promote failed"
  run developer off
  head_after="$(git -C "$P" rev-parse HEAD)"
  [[ "$head_before" == "$head_after" ]] || fail "$1: brew update moved the private Homebrew from $head_before to $head_after"
  note "$1: the private Homebrew stayed at $head_after through brew update"

  STATE_D="$(git -C "$W/origin.git" rev-parse refs/heads/main)"
  [[ "$(git -C "$W/origin.git" rev-parse "$STATE_D^")" == "$STATE_C" &&
    "$(git -C "$W/origin.git" diff --name-only "$STATE_C" "$STATE_D" | tr '\n' ' ')" == 'Formula/ravel.rb Formula/ravel@2.rb ' ]] ||
    fail 'state D is not one commit on state C that changes only ravel.rb and ravel@2.rb'
  git -C "$W/origin.git" diff "$STATE_C" "$STATE_D" -- Formula/ravel@2.rb >"$W/snapshots/ravel@2.diff"
  if [[ "$(grep -c '^[-+] ' "$W/snapshots/ravel@2.diff")" != 1 ]] ||
    ! grep -q '^+  deprecate! date: "[0-9-]*", because: :versioned_formula, replacement_formula: "ravel"$' "$W/snapshots/ravel@2.diff"; then
    cat "$W/snapshots/ravel@2.diff"
    fail 'the promotion changed ravel@2.rb beyond one deprecate! line'
  fi
  git -C "$W/origin.git" show "$STATE_D:Formula/ravel.rb" >"$W/snapshots/ravel-d.rb"
  git -C "$W/origin.git" show "$STATE_D:Formula/ravel@2.rb" >"$W/snapshots/ravel@2-d.rb"
  same_archives "$W/snapshots/ravel-d.rb" "$W/snapshots/ravel@2-d.rb" ||
    fail 'ravel.rb and ravel@2.rb at state D do not name the same three archives'
  note "state D: ravel.rb and ravel@2.rb name the same three archives and digests"
  note "state D: $STATE_D on state C ($(git -C "$W/origin.git" log -1 --format=%s "$STATE_D"))"
  note "ravel@2.rb gained: $(grep '^+  deprecate!' "$W/snapshots/ravel@2.diff" | cut -c2-)"
  note "ravel.rb at D: $(git -C "$W/origin.git" show "$STATE_D:Formula/ravel.rb" | grep -E '^  (version|keg_only)' | tr -s ' ' | tr '\n' ' ')"
  jq -c '{tap: .tap, retainedKegs: .installed.retainedKegs, installed: .installed.version}' \
    "$r/target/release-evidence/v$RAVEL_VERSION.promotion.json"
}

# The upgrade to state D. Homebrew must list the outdated Ravel.
to_d() { # STEP
  set_origin "$STATE_D"
  update_tap
  run outdated --verbose || true
  brew_s outdated --quiet | grep -qE '(^|/)ravel$' || fail "$1: brew outdated does not list ravel at state D"
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
run install grove
fronts fresh-a:install
pair_ok fresh-a 22.8.0

show "fresh-untrusted: fully qualified installs with no tap trust at tap state B"
set_origin "$STATE_B"
make_brew fresh-untrusted
tap_scratch
run install linkuistics/taps/ravel
[[ "$(trusted_taps)" == '[]' ]] || fail 'fresh-untrusted: the fully qualified install trusted the tap'
run install linkuistics/taps/personal
[[ -d "$P/opt/personal" ]] || fail 'fresh-untrusted: personal did not install'
[[ "$(trusted_taps)" == '[]' ]] || fail 'fresh-untrusted: the fully qualified personal install trusted the tap'
note "trusted formulae: $(brew_s trust --formula --json=v1 | tr -d ' \n')"
[[ "$(env -i PATH=/usr/bin:/bin HOME="$W/front-home" "$P/bin/ravel" --version)" == 'ravel 1.1.0' ]] ||
  fail 'fresh-untrusted: bin/ravel does not report ravel 1.1.0'
fronts fresh-untrusted:install

show "fresh-b: a fresh install at tap state B"
set_origin "$STATE_B"
make_brew fresh-b
tap_scratch
note 'The first ravel@2 on a prefix without either Ravel formula uses --skip-link.'
note 'Without tap trust, the fully qualified ravel@2 refuses the unversioned ravel:'
if run install --skip-link linkuistics/taps/ravel@2 >"$W/untrusted-ravel@2.out" 2>&1; then
  cat "$W/untrusted-ravel@2.out"
  fail 'fresh-b: ravel@2 installed from an untrusted tap'
fi
cat "$W/untrusted-ravel@2.out"
grep -q 'Refusing to load formula linkuistics/taps/ravel from untrusted tap linkuistics/taps' "$W/untrusted-ravel@2.out" ||
  fail 'fresh-b: the untrusted ravel@2 install failed for another reason'
[[ ! -e "$P/Cellar/ravel@2" ]] || fail 'fresh-b: the refused install left a keg'
note "Ravel's book trusts the tap first:"
trust_scratch
run install --skip-link linkuistics/taps/ravel@2
[[ ! -e "$P/bin/ravel" ]] || fail 'fresh-b: --skip-link linked bin/ravel'
run install grove
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

show "upgrade-c-d: the promotion, as an upgrade from tap state C to tap state D"
set_origin "$STATE_C"
make_brew upgrade-c-d
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts upgrade-c-d:install-c
run install linkuistics/taps/ravel@2
fronts upgrade-c-d:install-ravel@2
pair_ok upgrade-c-d:c "$LABEL"
retained upgrade-c-d-c
retained upgrade-c-d-c2 ravel@2/2.0.0
promote upgrade-c-d
fronts upgrade-c-d:promote
note "upgrade-c-d: Grove kegs: $(cd "$P/Cellar/grove" && printf '%s ' *); Ravel kegs: $(cd "$P/Cellar/ravel" && printf '%s ' *)"
pair_ok upgrade-c-d:d "$LABEL" "$RAVEL_VERSION"
retained upgrade-c-d-d
retained upgrade-c-d-d2 ravel@2/2.0.0
same_keg upgrade-c-d upgrade-c-d-c upgrade-c-d-d 'ravel 1.1.0'
same_keg upgrade-c-d upgrade-c-d-c2 upgrade-c-d-d2 'ravel@2 2.0.0'
retired_then_converted upgrade-c-d

show "fresh-d: a fresh install at tap state D"
set_origin "$STATE_D"
make_brew fresh-d
tap_scratch
trust_scratch
run install grove
fronts fresh-d:install real
pair_ok fresh-d "$LABEL" "$RAVEL_VERSION"
sample_launch fresh-d
note 'ravel@2 stays installable at the promoted version:'
run install linkuistics/taps/ravel@2
fronts fresh-d:install-ravel@2 real
pair_ok fresh-d:ravel@2 "$LABEL" "$RAVEL_VERSION"

show "upgrade-a-d: an upgrade from tap state A to tap state D, Ravel first"
set_origin "$STATE_A"
make_brew upgrade-a-d
tap_scratch
trust_scratch
run install linkuistics/taps/grove
fronts upgrade-a-d:install-a
pair_ok upgrade-a-d:a 22.8.0
retained upgrade-a-d-a
to_d upgrade-a-d
note 'A Ravel-only upgrade, with the outdated Grove 22.8.0 installed:'
run upgrade linkuistics/taps/ravel
fronts upgrade-a-d:upgrade-ravel
note "upgrade-a-d: after the Ravel-only upgrade: Grove kegs $(cd "$P/Cellar/grove" && printf '%s ' *), opt/grove -> $(readlink "$P/opt/grove"), bin/grove reports $(env -i PATH=/usr/bin:/bin "$P/bin/grove" --version); Ravel kegs $(cd "$P/Cellar/ravel" && printf '%s ' *), bin/ravel -> $(readlink "$P/bin/ravel")"
run outdated --verbose || true
if brew_s outdated --quiet | grep -qE '(^|/)grove$'; then
  note 'upgrade-a-d: Homebrew left the outdated Grove; the full upgrade follows:'
  run upgrade
  fronts upgrade-a-d:upgrade
fi
pair_ok upgrade-a-d:d "$LABEL" "$RAVEL_VERSION"
retained upgrade-a-d-d
same_keg upgrade-a-d upgrade-a-d-a upgrade-a-d-d 'ravel 1.1.0'
retired_then_converted upgrade-a-d

# The six controls of docs/specs/coordinated-release.md, "Acceptance and
# limits". Each runs the real Ravel entry point that owns the guard, against
# this run's scratch origin, Ravel clone and prefixes, and must refuse. A
# refusal must leave the scratch origin, the control's tap clone, the clone's
# main and its promotion evidence unchanged, and cut no version. A stub brew
# records any Homebrew command, and the early refusals must run none.
show "promotion controls"
C="$W/controls"
STUB="$C/stub"
mkdir -p "$STUB" "$C/probe"
printf '#!/bin/sh\necho "brew $*" >>%q\nexit 97\n' "$C/brew-calls.log" >"$STUB/brew"
chmod +x "$STUB/brew"
RAVEL_MAIN="$(git -C "$W/ravel" rev-parse refs/heads/main)"
PROMOTION_EVIDENCE="$(sha "$W/ravel/target/release-evidence/v$RAVEL_VERSION.promotion.json")"
EV="$W/promotion/v$RAVEL_VERSION.json"
RECEIPT="$W/promotion/v$RAVEL_VERSION.readiness.json"

use_brew() { # SCENARIO
  P="$W/$1/brew"
  H="$W/$1/home"
}
# A command in the Ravel clone, or in RAVEL_DIR when it is set, with BREW_DIR
# first on PATH and RAVEL_TAP_DIR=TAP.
in_ravel() { # BREW_DIR TAP COMMAND...
  local brew_dir="$1" tap="$2" dir="${RAVEL_DIR:-$W/ravel}"
  shift 2
  (cd "$dir" && env -i PATH="$brew_dir:$PATH" HOME="$HOME" USER="$USER" LOGNAME="$USER" TERM=dumb \
    TMPDIR="$W/tmp-promote" MISE_TRUSTED_CONFIG_PATHS="$dir" RAVEL_TAP_DIR="$tap" "$@")
}
# COMMAND must exit non-zero and print TEXT. REFUSED_STATUS keeps its status.
REFUSED_STATUS=0
refuses() { # NAME TEXT COMMAND...
  local name="$1" text="$2"
  shift 2
  REFUSED_STATUS=0
  note "\$ $(printf '%s ' "$@")"
  "$@" >"$C/$name.out" 2>&1 || REFUSED_STATUS=$?
  sed 's/^/  /' "$C/$name.out"
  note "[exit $REFUSED_STATUS]"
  [[ "$REFUSED_STATUS" != 0 ]] || fail "control $name: the command did not refuse"
  grep -qF -- "$text" "$C/$name.out" || fail "control $name: the refusal does not say: $text"
}
# The scratch origin at COMMIT, and a fresh colocated jj clone of it as a tap.
tap_clone() { # NAME COMMIT
  set_origin "$2"
  jj git clone --colocate --quiet "file://$W/origin.git" "$C/$1" 2>&1
}
# The channel rules accept a pair. The preparation runs in a copy-on-write
# copy of the Ravel clone, because its outcome depends on Ravel main. When
# main has no change since v2.0.0, it refuses at its change check. When main
# has later commits, as after a documentation fix, it commits release notes
# in the copy. The copy keeps the Ravel clone's main unchanged either way.
pair_accepted() { # NAME TEXT COMMAND...
  local name="$1" text="$2" status=0 copy="$C/$1-ravel"
  shift 2
  cp -cR "$W/ravel" "$copy"
  note "\$ $(printf '%s ' "$@")(in a copy of the Ravel clone)"
  RAVEL_DIR="$copy" "$@" >"$C/$name.out" 2>&1 || status=$?
  sed 's/^/  /' "$C/$name.out"
  note "[exit $status]"
  grep -qF -- "$text" "$C/$name.out" || fail "control $name: the preparation did not plan the cut: $text"
  if [[ "$status" != 0 ]]; then
    grep -qF 'No changes since v2.0.0' "$C/$name.out" || fail "control $name: the pair stopped before its change check"
    note "control $name: the pair passes the channel rules and stops only because main has no change since v2.0.0"
  else
    note "control $name: the pair passes the channel rules; main has commits after v2.0.0, so the copy prepared notes"
  fi
  [[ -z "$(git -C "$copy" tag -l 'v2.0.[1-9]*' 'v2.[1-9]*' 'v[3-9]*')" ]] || fail "control $name: a version was cut"
  [[ ! -s "$C/brew-calls.log" ]] || fail "control $name: Homebrew ran: $(cat "$C/brew-calls.log")"
  [[ "$(git -C "$W/ravel" rev-parse refs/heads/main)" == "$RAVEL_MAIN" ]] || fail "control $name: the Ravel clone's main moved"
}
untouched() { # NAME TAP COMMIT
  [[ "$(git -C "$W/origin.git" rev-parse refs/heads/main)" == "$3" ]] || fail "control $1: the scratch origin moved"
  [[ "$(jj -R "$C/$2" log --no-graph -r main -T commit_id)" == "$3" ]] || fail "control $1: the tap clone's main moved"
  [[ "$(git -C "$W/ravel" rev-parse refs/heads/main)" == "$RAVEL_MAIN" ]] || fail "control $1: the Ravel clone's main moved"
  [[ -z "$(git -C "$W/ravel" tag -l 'v2.0.[1-9]*' 'v2.[1-9]*' 'v[3-9]*')" ]] || fail "control $1: a version was cut"
  [[ "$(sha "$W/ravel/target/release-evidence/v$RAVEL_VERSION.promotion.json")" == "$PROMOTION_EVIDENCE" ]] ||
    fail "control $1: the promotion evidence changed"
  [[ ! -s "$C/brew-calls.log" ]] || fail "control $1: Homebrew ran: $(cat "$C/brew-calls.log")"
  note "control $1: refused; the scratch origin and the tap stay at $3, no version or evidence followed, and the stub brew saw no command"
}

note '1. Wrong channel: no channel, an unknown channel, and unversioned for a 2.x cut before promotion.'
refuses omitted-channel 'missing required variables: CHANNEL' in_ravel "$STUB" /nonexistent task --dry release:patch
refuses unknown-channel 'CHANNEL has an invalid value' in_ravel "$STUB" /nonexistent task --dry release:patch CHANNEL=stable
refuses empty-channel 'the release channel is required' in_ravel "$STUB" /nonexistent bash scripts/release-prepare.sh --channel '' --level patch
in_ravel "$STUB" /nonexistent task --dry release:patch CHANNEL=versioned-2 >"$C/valid-channel.out" 2>&1 ||
  fail 'the dry release with CHANNEL=versioned-2 failed, so the channel refusals prove nothing'
note "the same dry release with CHANNEL=versioned-2 lists $(grep -c '^task: \[' "$C/valid-channel.out") commands"
tap_clone tap-wrong "$STATE_C"
refuses unversioned-at-c 'tap Formula/ravel.rb is not at v2.0.0' \
  in_ravel "$STUB" "$C/tap-wrong" bash scripts/release-prepare.sh --channel unversioned --level patch
untouched unversioned-at-c tap-wrong "$STATE_C"
pair_accepted versioned-2-at-c 'planned v2.0.1 publishes Formula/ravel@2.rb (CHANNEL=versioned-2)' \
  in_ravel "$STUB" "$C/tap-wrong" bash scripts/release-prepare.sh --channel versioned-2 --level patch

note '2. A versioned-2 cut after promotion.'
tap_clone tap-after "$STATE_D"
refuses versioned-2-at-d 'so promotion is complete and ravel@2 receives no later release; select CHANNEL=unversioned' \
  in_ravel "$STUB" "$C/tap-after" bash scripts/release-prepare.sh --channel versioned-2 --level patch
untouched versioned-2-at-d tap-after "$STATE_D"
pair_accepted unversioned-at-d 'planned v2.0.1 publishes Formula/ravel.rb (CHANNEL=unversioned)' \
  in_ravel "$STUB" "$C/tap-after" bash scripts/release-prepare.sh --channel unversioned --level patch

note '3. Mismatched archives: in the receipt, in the cut evidence against the public assets, and in the tap.'
jq '.assets[0].sha256 = ("0" * 64)' "$EV" >"$C/evidence-mismatched.json"
write_receipt "$C/evidence-mismatched.json" "$LABEL" >"$C/receipt-mismatched.json"
tap_clone tap-archives "$STATE_C"
refuses receipt-digests 'ravel.assets are not the cut asset digests' in_ravel "$STUB" "$C/tap-archives" \
  task release:promote VERSION="$RAVEL_VERSION" READINESS="$C/receipt-mismatched.json" EVIDENCE="$EV"
untouched receipt-digests tap-archives "$STATE_C"
refuses evidence-digests 'but the cut recorded 0000000000000000' in_ravel "$STUB" "$C/tap-archives" \
  task release:promote VERSION="$RAVEL_VERSION" READINESS="$C/receipt-mismatched.json" EVIDENCE="$C/evidence-mismatched.json"
untouched evidence-digests tap-archives "$STATE_C"
# A tap whose ravel@2.rb names another macOS digest: one commit on state C.
git_s clone -q "$W/origin.git" "$C/tap-edit"
sed -i '' "s/$(jq -r '.assets[] | select(.name | endswith("aarch64-apple-darwin.tar.xz")) | .sha256' "$EV")/$(printf '1%.0s' {1..64})/" \
  "$C/tap-edit/Formula/ravel@2.rb"
[[ "$(git_s -C "$C/tap-edit" diff --numstat | tr '\t' ' ')" == '1 1 Formula/ravel@2.rb' ]] || fail 'the tap control did not change one digest line'
git_s -C "$C/tap-edit" commit -qam 'control: ravel@2.rb names another archive'
git_s -C "$C/tap-edit" push -q origin HEAD:main
STATE_C_MISMATCHED="$(git -C "$W/origin.git" rev-parse refs/heads/main)"
tap_clone tap-mismatched "$STATE_C_MISMATCHED"
head_before="$(git -C "$W/upgrade-a-d/brew" rev-parse HEAD)"
refuses tap-archives 'is neither the published formula of these assets nor its promotion; it is left unchanged' \
  in_ravel "$W/upgrade-a-d/wrap" "$C/tap-mismatched" \
  task release:promote VERSION="$RAVEL_VERSION" READINESS="$RECEIPT" EVIDENCE="$EV"
[[ "$(git -C "$W/upgrade-a-d/brew" rev-parse HEAD)" == "$head_before" ]] || fail 'control tap-archives: the private Homebrew moved'
untouched tap-archives tap-mismatched "$STATE_C_MISMATCHED"
sed "s/$(jq -r '.assets[1].sha256' "$EV")/$(printf '2%.0s' {1..64})/" "$W/snapshots/ravel-d.rb" >"$C/ravel-mismatched.rb"
if same_archives "$C/ravel-mismatched.rb" "$W/snapshots/ravel@2-d.rb"; then
  fail "the instrument's archive check accepted a ravel.rb with another digest"
fi
note "the instrument's archive check fails on a ravel.rb at D with one other digest"

note '4. Silent schema-3 reuse: the working config.rhai with SCHEMA_VERSION = 3, at D.'
use_brew upgrade-c-d
mkdir -p "$C/setup-tmp"
write_agent controls
write_config_rhai controls "$C/schema3-home" "$P/opt/ravel/bin/ravel"
sed -i '' 's/^export const SCHEMA_VERSION = 4;$/export const SCHEMA_VERSION = 3;/' "$C/schema3-home/.config/ravel/config.rhai"
grep -qx 'export const SCHEMA_VERSION = 3;' "$C/schema3-home/.config/ravel/config.rhai" || fail 'the schema-3 control has no schema 3'
schema3() (
  cd "$C/probe" && env -i PATH="$P/bin:/usr/bin:/bin" HOME="$C/schema3-home" TMPDIR="$C/setup-tmp" \
    ravel start-session --kind impl --prompt probe
)
refuses schema-3 'refused (unsupported_version, stage validation): SCHEMA_VERSION is 3, and this release reads 4' schema3
[[ "$REFUSED_STATUS" == 3 ]] || fail "control schema-3: exit $REFUSED_STATUS, not 3"
printf '{"skills": {"schemaVersion": 1, "sources": []}}\n' >"$C/schema3-home/.config/ravel/settings.json"
refuses schema-3-delivery-off 'refused (unsupported_version, stage validation): SCHEMA_VERSION is 3, and this release reads 4' schema3
[[ "$REFUSED_STATUS" == 3 ]] || fail "control schema-3-delivery-off: exit $REFUSED_STATUS, not 3"
# shellcheck disable=SC2016 # the backticks are Ravel's literal remedy text
launch controls schema3 "$P/bin" "$C/schema3-home" refused unsupported_version 'remedy: set `export const SCHEMA_VERSION = 4;`'
[[ ! -e "$C/launch-schema3/ravel.txt" ]] || fail 'control schema-3: the refused launch ran the agent'
note 'control schema-3: Ravel 2.0.0 refuses the schema-3 script with exit 3, also with delivery off; Grove shows the refusal and keeps the leaf. The same script at schema 4 delivered in upgrade-c-d.'

note '5. A removed retained resource: the 1.1.0 keg loses its policy worker during the promotion install.'
use_brew fresh-c
set_origin "$STATE_D"
KEG="$P/Cellar/ravel/1.1.0"
retained controls-before
mkdir -p "$W/fresh-c/wrap-remove"
cat >"$W/fresh-c/wrap-remove/brew" <<SH
#!/bin/sh
status=0
$(printf '%q' "$W/fresh-c/wrap/brew") "\$@" || status=\$?
if [ "\$1" = upgrade ]; then
  rm -f $(printf '%q' "$KEG/libexec/ravel/ravel-policy")
  echo 'control: removed libexec/ravel/ravel-policy from the 1.1.0 keg after brew upgrade' >&2
fi
exit \$status
SH
chmod +x "$W/fresh-c/wrap-remove/brew"
head_before="$(git -C "$P" rev-parse HEAD)"
run developer on
refuses removed-resource "the earlier keg $KEG changed" \
  in_ravel "$W/fresh-c/wrap-remove" /nonexistent bash scripts/release-install.sh --channel unversioned --version "$RAVEL_VERSION"
grep -qF 'libexec/ravel/ravel-policy' "$C/removed-resource.out" || fail 'control removed-resource: the refusal does not name the worker'
run developer off
[[ "$(git -C "$P" rev-parse HEAD)" == "$head_before" ]] || fail 'control removed-resource: the private Homebrew moved'
[[ ! -e "$KEG/libexec/ravel/ravel-policy" && "$(readlink "$P/bin/ravel")" == "../Cellar/ravel/$RAVEL_VERSION/bin/ravel" ]] ||
  fail 'control removed-resource: the upgrade or the removal did not happen'
retained controls-after
if (same_keg removed-resource controls-before controls-after 'ravel 1.1.0') >"$C/same-keg.out" 2>&1; then
  fail "the instrument's keg check missed the removed worker"
fi
note "the instrument's keg check fails: $(grep -F 'ravel-policy' "$C/same-keg.out" | head -1)"
front_ok control-removed-resource "$KEG/bin/ravel"
note "the instrument's front check still passes, so it alone cannot see a removed resource"
legacy() (
  cd "$C/probe" && env -i PATH=/usr/bin:/bin HOME="$W/fresh-c/setup-home" TMPDIR="$C/setup-tmp" \
    "$KEG/bin/ravel" start-session --kind impl --prompt probe
)
refuses legacy-launch 'refused (worker_missing, stage worker)' legacy
note "control removed-resource: release-install.sh refused; a 1.1.0 launch now fails with exit $REFUSED_STATUS, where native setup in fresh-c worked"

note '6. Promotion before compatible Grove: no receipt, and the tap at state B under the candidate receipt.'
tap_clone tap-early "$STATE_C"
refuses no-receipt 'no consumer readiness receipt' in_ravel "$STUB" "$C/tap-early" \
  task release:promote VERSION="$RAVEL_VERSION" READINESS="$C/no-receipt.json" EVIDENCE="$EV"
untouched no-receipt tap-early "$STATE_C"
tap_clone tap-b "$STATE_B"
refuses grove-not-published "the tap's Formula/grove.rb is at 22.8.0, not the receipt's $LABEL" in_ravel "$STUB" "$C/tap-b" \
  task release:promote VERSION="$RAVEL_VERSION" READINESS="$RECEIPT" EVIDENCE="$EV"
untouched grove-not-published tap-b "$STATE_B"
note 'the same receipt promoted state C in upgrade-c-d'

show "protected subjects"
snapshot after
if cmp -s "$W/snapshots/before.txt" "$W/snapshots/after.txt"; then
  note "unchanged: real prefix kegs and links, real tap, installed tap, tap remote, owner HOME ($(sha "$W/snapshots/after.txt"))"
else
  diff "$W/snapshots/before.txt" "$W/snapshots/after.txt" || true
  fail 'a protected subject changed'
fi
show "rehearsal passed"

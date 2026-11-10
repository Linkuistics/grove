#!/usr/bin/env bash
# Fresh install and upgrade of Grove's keg at each tap state of the coordinated
# release, in a private Homebrew-shaped prefix. No real keg, tap or link changes.
#
#   grove-tap-states.sh OLD_GROVE_KEG NEW_GROVE_KEG NEW_VERSION RAVEL_1_1_PREFIX RAVEL_2_0_PREFIX DIGESTS
#
# docs/specs/coordinated-release.md, "Compatible tap states", defines the
# states:
#
#   A  the published Grove (OLD_GROVE_KEG) and unversioned Ravel 1.1.0
#   B  A, plus the keg-only ravel@2 at 2.0.0
#   C  the new Grove (NEW_GROVE_KEG), unversioned Ravel 1.1.0 and ravel@2
#   D  the new Grove, with unversioned Ravel and ravel@2 both at 2.0.0
#
# scripts/release-formula.test.sh passes the keg that Homebrew's install method
# of the rendered formula made, and DIGESTS, the `<id><TAB><resourceDigest>`
# lines of the plain checkouts. Each 2.0 keg holds the installation record that
# Ravel's formula writes, which names the prefix's share/ravel/skills.
#
# The kegs and links follow Homebrew: each keg lies at Cellar/<formula>/<version>,
# `opt/<formula>` names the linked version, and a linked keg's executables and
# source aliases appear as relative links in bin/ and share/ravel/skills/. A
# Grove keg holds each source in libexec/skill-sources/<id>, and its
# share/ravel/skills/<id> is a relative directory symlink to it. Keg#link_dir
# links that symlink as a file, so the prefix's link names the keg's alias
# and never a file inside a source. A keg-only formula gets the opt link only. An upgrade unlinks the earlier keg,
# installs and links the new one, then removes the earlier keg.
#
# Each state checks that the default PATH selects a Ravel that its Grove admits,
# and that no link dangles. At A and B, no keg source exists. At B, ravel@2
# with no settings has no source. At C and D, the 2.0 Ravel reads the six keg
# sources with no settings, each with the originPath of its private source in
# the Grove keg and with its checkout's digest, and scripts/grove-installed-smoke.sh runs the lifecycle and
# standalone callers of the new Grove on the admitted majors of that state.
# The checks run on a fresh prefix for each state, then along one prefix that
# upgrades from A to D.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fail() {
  echo "grove-tap-states: $*" >&2
  exit 1
}
[[ $# == 6 ]] ||
  fail 'usage: grove-tap-states.sh OLD_GROVE_KEG NEW_GROVE_KEG NEW_VERSION RAVEL_1_1_PREFIX RAVEL_2_0_PREFIX DIGESTS'
old_keg="$1"
new_keg="$2"
new_version="$3"
ravel_1_1="$4"
ravel_2_0="$5"
digests="$6"
old_version="$("$old_keg/bin/grove" --version)"
old_version="${old_version#grove }"
# The Cellar label of each Grove keg. Before the cut, the working copy has the
# published version, so the new keg takes Homebrew's label for a revision of
# the same version. The checks tell the kegs apart by path.
old_label="$old_version"
new_label="$new_version"
[[ "$new_label" != "$old_label" ]] || new_label="${new_version}_1"
[[ "$("$ravel_1_1/bin/ravel" --version)" == 'ravel 1.1.0' ]] || fail 'the 1.1 prefix is not Ravel 1.1.0'
readonly sources='design
engineering
grove-core
requirements
research
writing'

scratch="$(mktemp -d)"
# Ravel publishes each skill snapshot read-only.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT
scratch="$(cd "$scratch" && pwd -P)"

canonical() {
  python3 -I -c 'import os, sys; print(os.path.realpath(sys.argv[1]))' "$1"
}

# install_keg H FORMULA VERSION SOURCE: copy SOURCE to H's keg of FORMULA at
# VERSION. A 2.0 Ravel keg also gets the installation record of its formula.
install_keg() {
  local home="$1" formula="$2" version="$3" source="$4" keg="$1/Cellar/$2/$3"
  [[ ! -e "$keg" ]] || fail "$keg already exists"
  mkdir -p "$(dirname "$keg")"
  cp -pR "$source" "$keg"
  if [[ "$formula" == ravel* && "$version" == 2.* ]]; then
    mkdir -p "$keg/share/ravel"
    printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$home/share/ravel/skills" \
      >"$keg/share/ravel/installation.json"
  fi
}

# link_keg H FORMULA VERSION [keg-only]: point opt/FORMULA at the keg, and
# unless keg-only link its executables and skill sources into H.
link_keg() {
  local home="$1" formula="$2" version="$3" keg_only="${4:-}" entry
  mkdir -p "$home/opt" "$home/bin" "$home/share/ravel/skills"
  ln -s "../Cellar/$formula/$version" "$home/opt/$formula"
  [[ -z "$keg_only" ]] || return 0
  for entry in "$home/Cellar/$formula/$version/bin/"*; do
    ln -s "../Cellar/$formula/$version/bin/${entry##*/}" "$home/bin/${entry##*/}"
  done
  if [[ -d "$home/Cellar/$formula/$version/share/ravel/skills" ]]; then
    for entry in "$home/Cellar/$formula/$version/share/ravel/skills/"*; do
      # A real source directory here is the earlier layout, which Homebrew
      # can expand into file links. "The keg layout" of
      # docs/specs/codex-skill-provisioning.md.
      [[ -e "$entry" || -L "$entry" ]] || continue
      [[ -L "$entry" ]] || fail "$entry is not a source alias"
      ln -s "../../../Cellar/$formula/$version/share/ravel/skills/${entry##*/}" \
        "$home/share/ravel/skills/${entry##*/}"
    done
  fi
}

# unlink_keg H FORMULA VERSION: remove every link of H into that keg.
unlink_keg() {
  local home="$1" link target
  while IFS= read -r link; do
    target="$(readlink "$link")"
    if [[ "$target" == *"Cellar/$2/$3" || "$target" == *"Cellar/$2/$3/"* ]]; then
      rm "$link"
    fi
  done < <(find "$home/opt" "$home/bin" "$home/share/ravel/skills" -type l 2>/dev/null)
}

# upgrade H FORMULA FROM TO SOURCE: Homebrew's upgrade of one formula.
upgrade() {
  unlink_keg "$1" "$2" "$3"
  install_keg "$1" "$2" "$4" "$5"
  link_keg "$1" "$2" "$4"
  rm -rf "${1:?}/Cellar/$2/$3"
}

# resolve_ids RAVEL: `<id><TAB><digest><TAB><originPath>` for each source
# that RAVEL resolves with no settings, or the word `disabled`.
resolve_ids() {
  local home="$scratch/home-$RANDOM"
  mkdir -p "$home" "$scratch/tmp"
  env -i HOME="$home" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin "$1" skills resolve --json |
    python3 -I -c '
import json, sys
document = json.load(sys.stdin)
if document.get("status") == "disabled":
    print("disabled")
    sys.exit()
if document.get("status") != "resolved":
    sys.exit(f"the sources did not resolve: {document}")
for source in document["sources"]:
    print("\t".join((source["id"], source["resourceDigest"], source["originPath"])))
'
}

# check_keg_sources LABEL RAVEL H: RAVEL reads exactly the six sources of the
# new Grove keg, each with the digest of its plain checkout.
check_keg_sources() {
  local label="$1" ravel="$2" home="$3" resolved id digest origin
  resolved="$(resolve_ids "$ravel")" || fail "$label: the keg sources did not resolve"
  [[ "$(cut -f1 <<<"$resolved")" == "$sources" ]] ||
    fail "$label: $ravel resolves '$(cut -f1 <<<"$resolved" | tr '\n' ' ')', not the six keg sources"
  while IFS=$'\t' read -r id digest origin; do
    [[ "$origin" == "$home/Cellar/grove/$new_label/libexec/skill-sources/$id" ]] ||
      fail "$label: source $id has the originPath $origin, not its private source in the Grove keg"
    grep -Fxq "$id"$'\t'"$digest" "$digests" || fail "$label: source $id has digest $digest, not its checkout's"
  done <<<"$resolved"
}

# check_state LABEL STATE H: the observations of STATE in the prefix H.
check_state() {
  local label="$1" state="$2" home="$3" dangling ravel grove expected_ravel smoke keg version
  dangling="$(find "$home/bin" "$home/opt" "$home/share/ravel/skills" -type l ! -exec test -e {} \; -print 2>/dev/null)"
  [[ -z "$dangling" ]] || fail "$label: dangling links: $dangling"
  case "$state" in
    A | B)
      keg="$home/Cellar/grove/$old_label" version="$old_version"
      expected_ravel="$home/Cellar/ravel/1.1.0/bin/ravel"
      [[ -z "$(ls -A "$home/share/ravel/skills")" ]] || fail "$label: a skill source exists before the compatible Grove"
      ;;
    C | D)
      keg="$home/Cellar/grove/$new_label" version="$new_version"
      expected_ravel="$home/Cellar/ravel/1.1.0/bin/ravel"
      [[ "$state" == C ]] || expected_ravel="$home/Cellar/ravel/2.0.0/bin/ravel"
      ;;
  esac
  grove="$(canonical "$home/bin/grove")"
  [[ "$grove" == "$keg/bin/grove" ]] || fail "$label: bin/grove is $grove, not that of $keg"
  [[ "$("$home/bin/grove" --version)" == "grove $version" ]] || fail "$label: bin/grove does not report $version"
  ravel="$(canonical "$home/bin/ravel")"
  # Grove admits stable 1.x from 1.1.0 and 2.x from 2.0.0. Before the cut, the
  # 2.0 build reports 1.1.0, so its prefix tells it apart: it has no worker.
  [[ "$ravel" == "$expected_ravel" ]] || fail "$label: the default PATH selects $ravel, not $expected_ravel"
  [[ "$("$home/bin/ravel" --version)" =~ ^ravel\ [12]\.[0-9]+\.[0-9]+$ ]] || fail "$label: bin/ravel is not stable 1.x or 2.x"
  case "$state" in
    A) [[ ! -e "$home/opt/ravel@2" ]] || fail "$label: ravel@2 is installed" ;;
    B)
      [[ "$(resolve_ids "$home/opt/ravel@2/bin/ravel")" == disabled ]] ||
        fail "$label: ravel@2 finds a skill source before the compatible Grove"
      ;;
    C)
      check_keg_sources "$label" "$home/opt/ravel@2/bin/ravel" "$home"
      smoke=("$home/Cellar/ravel/1.1.0" "$home/Cellar/ravel@2/2.0.0")
      ;;
    D)
      [[ ! -e "$home/Cellar/ravel/2.0.0/libexec/ravel" ]] || fail "$label: unversioned Ravel holds a 1.x worker"
      check_keg_sources "$label" "$home/bin/ravel" "$home"
      check_keg_sources "$label" "$home/opt/ravel@2/bin/ravel" "$home"
      smoke=(- "$home/Cellar/ravel/2.0.0")
      ;;
  esac
  if [[ "$state" == C || "$state" == D ]]; then
    bash "$root/scripts/grove-installed-smoke.sh" "$keg" "$new_version" "${smoke[@]}" ||
      fail "$label: the installed callers failed"
  fi
  echo "grove-tap-states: $label passed"
}

# fresh STATE: a new prefix that holds the formulae of STATE.
fresh() {
  local state="$1" home="$scratch/fresh-$1"
  mkdir -p "$home"
  case "$state" in
    A | B)
      install_keg "$home" grove "$old_label" "$old_keg"
      link_keg "$home" grove "$old_label"
      ;;
    C | D)
      install_keg "$home" grove "$new_label" "$new_keg"
      link_keg "$home" grove "$new_label"
      ;;
  esac
  case "$state" in
    A | B | C)
      install_keg "$home" ravel 1.1.0 "$ravel_1_1"
      link_keg "$home" ravel 1.1.0
      ;;
    D)
      install_keg "$home" ravel 2.0.0 "$ravel_2_0"
      link_keg "$home" ravel 2.0.0
      ;;
  esac
  if [[ "$state" != A ]]; then
    install_keg "$home" ravel@2 2.0.0 "$ravel_2_0"
    link_keg "$home" ravel@2 2.0.0 keg-only
  fi
  check_state "fresh install at $state" "$state" "$home"
}

for state in A B C D; do
  fresh "$state"
done

# One prefix along the release: A, then each publication's upgrade.
home="$scratch/upgrade"
mkdir -p "$home"
install_keg "$home" grove "$old_label" "$old_keg"
link_keg "$home" grove "$old_label"
install_keg "$home" ravel 1.1.0 "$ravel_1_1"
link_keg "$home" ravel 1.1.0
check_state 'upgrade at A' A "$home"
install_keg "$home" ravel@2 2.0.0 "$ravel_2_0"
link_keg "$home" ravel@2 2.0.0 keg-only
check_state 'upgrade from A to B' B "$home"
upgrade "$home" grove "$old_label" "$new_label" "$new_keg"
[[ ! -e "$home/Cellar/grove/$old_label" ]] || fail 'the upgrade kept the published Grove keg'
check_state 'upgrade from B to C' C "$home"
upgrade "$home" ravel 1.1.0 2.0.0 "$ravel_2_0"
check_state 'upgrade from C to D' D "$home"
echo 'grove-tap-states: fresh install and upgrade passed at each tap state A to D'

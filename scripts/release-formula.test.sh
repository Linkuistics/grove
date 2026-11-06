#!/usr/bin/env bash
# Test a native archive/template without changing any keg, tap or release.
#
#   release-formula.test.sh NATIVE_ARCHIVE RAVEL_1_1_PREFIX RAVEL_2_0_PREFIX OLD_GROVE_KEG
#
# 1. The archive passes the manifest check, which needs the plain checkout of
#    each pack that `task check:packs` makes.
# 2. Homebrew's own install and test methods of the rendered formula run in a
#    private prefix (scripts/release-formula.test.rb). The formula's test
#    resolves the keg's six skill sources through a private copy of the 2.0
#    build, with no settings.
# 3. Each installed source keeps the Git modes of the manifest, and its
#    resourceDigest equals that of the plain checkout of the same commit: the
#    pack's checkout under target/external-packs, and for core a plain checkout
#    of HEAD. Core's working files must therefore equal HEAD.
# 4. scripts/grove-tap-states.sh installs and upgrades that keg in a private
#    Homebrew-shaped prefix at each tap state A to D of the coordinated
#    release. OLD_GROVE_KEG is the published Grove of states A and B.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/release-common.sh
source "$root/scripts/release-common.sh"
fail() {
  echo "release-formula: $*" >&2
  exit 1
}
# The formula test checks the TypeScript set against the 1.1 prefix and the
# Rhai set against the 2.0 prefix, each named explicitly, because a build of
# Ravel's 2.0 feature line reports `ravel 1.1.0` until the cut.
[[ $# == 4 ]] || {
  echo 'usage: release-formula.test.sh NATIVE_ARCHIVE RAVEL_1_1_PREFIX RAVEL_2_0_PREFIX OLD_GROVE_KEG' >&2
  exit 1
}
archive="$1"
ravel_1_1="$(cd "$2" && pwd -P)/bin/ravel"
ravel_2_0="$(cd "$3" && pwd -P)/bin/ravel"
old_keg="$(cd "$4" && pwd -P)"
[[ -x "$ravel_1_1" && -x "$ravel_2_0" ]] || fail 'each Ravel prefix needs bin/ravel'
[[ -x "${ravel_1_1%/bin/ravel}/libexec/ravel/ravel-policy" ]] || fail 'the 1.1 prefix holds no 1.x worker'
[[ ! -e "${ravel_2_0%/bin/ravel}/libexec/ravel" ]] || fail 'the 2.0 prefix holds a 1.x worker'
[[ -x "$old_keg/bin/grove" && ! -e "$old_keg/share/ravel/skills" ]] ||
  fail "$old_keg is not a Grove keg from before the keg sources"
top="$(basename "$archive" .tar.xz)"
version="${top#grove-v}"
version="${version%-aarch64-apple-darwin}"
[[ "$top" == "grove-v$version-aarch64-apple-darwin" ]] || fail 'expected native Grove archive'
assert_archive "$archive" "$top"
scratch="$(mktemp -d)"
# Ravel publishes each skill snapshot read-only.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT
scratch="$(cd "$scratch" && pwd -P)"
extract_exactly "$archive" "$scratch"
mkdir "$scratch/prefix" "$scratch/test" "$scratch/tmp"
# Tests never fetch a formula URL. Checksum placeholders are valid fixture
# values; the production release build renders all three real archive hashes.
sha="$(shasum -a 256 "$archive" | cut -d' ' -f1)"
sed -e "s/@VERSION@/$version/g" -e "s/@SHA_[A-Z_]*@/$sha/g" \
  "$root/scripts/templates/grove.rb.tmpl" >"$scratch/grove.rb"
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/release-formula.test.rb" \
  "$scratch/grove.rb" "$scratch/$top" "$scratch/prefix" "$scratch/test" "$ravel_1_1" "$ravel_2_0"
keg="$scratch/prefix"

# Homebrew moves each file into the keg, so each keeps the mode of the archive.
while IFS=$'\t' read -r mode entry; do
  [[ "$(git_mode "$keg/$entry")" == "$mode" ]] ||
    fail "the keg's $entry has Git mode $(git_mode "$keg/$entry"), not $mode"
done < <(skills_manifest)
echo 'release-formula: every installed skill file keeps the Git mode of the manifest'

# resolve_sources RAVEL HOME: the id, resourceDigest and originPath of each
# source that RAVEL resolves in HOME, tab-separated, in Ravel's order.
resolve_sources() {
  env -i HOME="$2" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin "$1" skills resolve --json |
    python3 -I -c '
import json, sys
document = json.load(sys.stdin)
if document.get("status") != "resolved":
    sys.exit(f"the sources did not resolve: {document}")
for source in document["sources"]:
    print("\t".join((source["id"], source["resourceDigest"], source["originPath"])))
'
}

# The plain checkout of core: HEAD of this repository, made as a pack's
# checkout is. A working copy whose plugins/grove differs from HEAD has no
# such commit.
head="$(pack_git -C "$root" rev-parse HEAD)" || fail 'this repository has no Git HEAD'
if ! pack_git -C "$root" diff --quiet "$head" -- "$ARCHIVE_CORE_PATH" ||
  [[ -n "$(pack_git -C "$root" ls-files --others --exclude-standard -- "$ARCHIVE_CORE_PATH")" ]]; then
  fail "$ARCHIVE_CORE_PATH differs from HEAD ($head), so core has no plain checkout of the same commit; make it a commit first (with jj, 'jj new')"
fi
pack_checkout "$scratch/core-checkout" "$root" "$head" || fail "cannot check out $head"

# Settings that name each plain checkout as a local source, with the roots
# that it holds. Ravel's digest covers the source's content, not its path, ID
# or roots.
mkdir -p "$scratch/reference-home/.config/ravel"
while IFS=$'\t' read -r id dir path; do
  [[ "$id" != "$ARCHIVE_CORE_ID" ]] || dir="$scratch/core-checkout"
  printf '%s\t%s\n' "$id" "$dir/$path"
done < <(each_skill_source) | python3 -I -c '
import json, os, sys
sources = []
for line in sys.stdin.read().splitlines():
    source_id, path = line.split("\t")
    roots = [root for root in ("skills", "craft") if os.path.isdir(os.path.join(path, root))]
    sources.append({"id": source_id, "type": "local", "path": path, "roots": roots})
print(json.dumps({"skills": {"schemaVersion": 1, "sources": sources}}))
' >"$scratch/reference-home/.config/ravel/settings.json"
resolve_sources "$ravel_2_0" "$scratch/reference-home" | cut -f1,2 | LC_ALL=C sort >"$scratch/reference-digests"

# The keg's sources through a private copy of the 2.0 build, whose record
# names the keg's share/ravel/skills.
mkdir -p "$scratch/keg-ravel/bin" "$scratch/keg-ravel/share/ravel" "$scratch/keg-home"
cp "$ravel_2_0" "$scratch/keg-ravel/bin/ravel"
printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$keg/$ARCHIVE_SKILLS" \
  >"$scratch/keg-ravel/share/ravel/installation.json"
resolve_sources "$scratch/keg-ravel/bin/ravel" "$scratch/keg-home" >"$scratch/keg-sources"
while IFS=$'\t' read -r id _ origin; do
  [[ "$origin" == "$keg/$ARCHIVE_SKILLS/$id" ]] || fail "source $id lies outside the keg: $origin"
done <"$scratch/keg-sources"
cut -f1,2 "$scratch/keg-sources" | LC_ALL=C sort >"$scratch/keg-digests"
[[ "$(cut -f1 "$scratch/keg-digests")" == "$(cut -f1 "$scratch/reference-digests")" ]] ||
  fail "the keg's sources $(cut -f1 "$scratch/keg-digests" | tr '\n' ' ')differ from the record's $(cut -f1 "$scratch/reference-digests" | tr '\n' ' ')"
if ! diff -u "$scratch/reference-digests" "$scratch/keg-digests" >&2; then
  fail "a keg source's resourceDigest differs from that of its plain checkout; see the diff above"
fi
while IFS=$'\t' read -r id digest; do
  echo "release-formula: $id $digest equals its plain checkout"
done <"$scratch/keg-digests"

bash "$root/scripts/grove-tap-states.sh" "$old_keg" "$keg" "$version" "${ravel_1_1%/bin/ravel}" \
  "${ravel_2_0%/bin/ravel}" "$scratch/reference-digests"

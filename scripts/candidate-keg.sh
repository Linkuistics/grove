#!/usr/bin/env bash
# Build the candidate keg of one Grove commit for the staging trials, without
# changing any real keg, HOME or tap.
#
#   candidate-keg.sh COMMIT OUT RAVEL_2_0_PREFIX [ACCEPTED]
#
# "Skill formulae" and "Cutover contract" of docs/specs/owner-skill-delivery.md
# state the candidate keg. Before Grove's release, the staging trials read
# Grove's six sources from it. The candidate is the last published commit of
# the workstream's line that changes plugins/grove or plugins/packs.json.
#
# 1. A plain Git checkout of COMMIT, made with Git's global and system
#    configuration off, lands in OUT/source. The script reports whether a
#    remote-tracking branch holds COMMIT, and it builds an unpublished commit
#    too.
# 2. The checkout's own scripts/release-build.sh builds an unreleased archive
#    for this host's target. Its packs come from the plain checkouts of the
#    commits in the checkout's plugins/packs.json. A clean checkout of the same
#    commit under this repository's target/external-packs is copied first, so
#    the build reuses it, and the build fetches any other one.
# 3. Homebrew's install method of the checkout's formula template, rendered
#    with the archive's sha256, fills the keg OUT/brew/Cellar/grove/<version>
#    (scripts/candidate-keg.rb). The script then makes the links of `brew
#    link`: OUT/brew/opt/grove names the keg, and a relative link in
#    OUT/brew/share/ravel/skills names each source.
# 4. A private copy of the Ravel 2.0 build resolves the sources with no
#    settings. Its installation record names OUT/brew/share/ravel/skills. The
#    keg must hold exactly core and the members of the pack record, and each
#    originPath must lie in the keg. Each resourceDigest must equal that of the
#    plain checkout of the same commit.
# 5. The script prints the commit and each source's resourceDigest. It writes
#    OUT/digests.tsv, one `id<TAB>resourceDigest` line for each source in byte
#    order, and OUT/candidate.json with the commit, the keg and each source.
#    With ACCEPTED, an earlier digests.tsv, any difference fails the run.
#
# OUT must not exist. It persists, because the staging settings name its keg.
# Ravel publishes each skill snapshot under OUT/home read-only, so remove OUT
# with `chmod -R u+w OUT && rm -rf OUT`.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/pack-record.sh
source "$root/scripts/pack-record.sh"
fail() {
  echo "candidate-keg: $*" >&2
  exit 1
}
(($# == 3 || $# == 4)) || fail 'usage: candidate-keg.sh COMMIT OUT RAVEL_2_0_PREFIX [ACCEPTED]'
commit_spec="$1"
out="$2"
ravel_2_0="$(cd "$3" && pwd -P)/bin/ravel" || fail "no Ravel prefix $3"
accepted=''
if (($# == 4)); then
  [[ -f "$4" ]] || fail "no accepted digests file $4"
  accepted="$(cd "$(dirname "$4")" && pwd -P)/$(basename "$4")"
fi
[[ -x "$ravel_2_0" ]] || fail 'the Ravel 2.0 prefix needs bin/ravel'
[[ ! -e "${ravel_2_0%/bin/ravel}/libexec/ravel" ]] || fail 'the 2.0 prefix holds a 1.x worker'
command -v brew >/dev/null || fail 'brew is not on PATH; the install method needs Homebrew'
case "$(uname -s)-$(uname -m)" in
  Darwin-arm64) target=aarch64-apple-darwin ;;
  Linux-aarch64) target=aarch64-unknown-linux-gnu ;;
  Linux-x86_64) target=x86_64-unknown-linux-gnu ;;
  *) fail "no release target for $(uname -s)-$(uname -m)" ;;
esac

commit="$(pack_git -C "$root" rev-parse --verify --quiet "$commit_spec^{commit}")" ||
  fail "this repository holds no commit $commit_spec"
[[ ! -e "$out" ]] || fail "$out exists; name a new directory"
mkdir -p "$out"
out="$(cd "$out" && pwd -P)"
case "$out/" in
  "$(cd "$root" && pwd -P)/"*) fail "$out lies inside this working copy; name a directory outside it" ;;
esac
echo "candidate-keg: commit $commit"
published="$(pack_git -C "$root" branch --remotes --contains "$commit" --format='%(refname:short)' | head -n 1)"
if [[ -n "$published" ]]; then
  echo "candidate-keg: the remote-tracking branch $published holds the commit"
else
  echo "candidate-keg: no remote-tracking branch holds the commit, so it is not a published candidate"
fi

# 1. The plain checkout.
common="$(pack_git -C "$root" rev-parse --path-format=absolute --git-common-dir)"
pack_checkout "$out/source" "file://$common" "$commit" || fail "cannot check out $commit"

# 2. The unreleased archive. A clean pack checkout of this repository is
# copied into the checkout's cache, where the build's reuse rule accepts it.
members="$(pack_record_members "$out/source/plugins/packs.json")" ||
  fail "the commit's plugins/packs.json is not a pack record"
expected_ids="grove-core"
while IFS=$'\t' read -r status id url path sha; do
  [[ "$status" == ok ]] || fail "pack $id in the commit's plugins/packs.json: $url"
  expected_ids+=$'\n'"$id"
  cached="$(pack_checkout_dir "$root/target/external-packs" "$id" "$sha")"
  if pack_checkout_is_clean "$cached" "$sha"; then
    mkdir -p "$(dirname "$(pack_checkout_dir "$out/source/target/external-packs" "$id" "$sha")")"
    cp -Rp "$cached" "$(pack_checkout_dir "$out/source/target/external-packs" "$id" "$sha")"
  fi
done <<<"$members"
expected_ids="$(LC_ALL=C sort <<<"$expected_ids")"
(cd "$out/source" && ./scripts/release-build.sh --unreleased "$target") ||
  fail "the release build of $commit failed; see above"
archives=("$out/source/target/archives/"grove-v*-"$target".tar.xz)
((${#archives[@]} == 1)) && [[ -f "${archives[0]}" ]] || fail "the build left no single archive for $target"
top="$(basename "${archives[0]}" .tar.xz)"
version="${top#grove-v}"
version="${version%-"$target"}"
archive="$out/$top.tar.xz"
cp "${archives[0]}" "$archive"
archive_sha="$(shasum -a 256 "$archive" | cut -d' ' -f1)"
echo "candidate-keg: archive $archive, sha256 $archive_sha"

# 3. Homebrew's install method, then the links of `brew link`. bsdtar folds
# AppleDouble members into their files unless told not to, as
# extract_exactly in scripts/release-common.sh states.
mkdir "$out/stage"
exact=()
if tar --version 2>/dev/null | grep -q bsdtar; then
  exact=(--options '!mac-ext')
fi
tar -xJf "$archive" -C "$out/stage" ${exact[@]+"${exact[@]}"}
sed -e "s/@VERSION@/$version/g" -e "s/@SHA_[A-Z0-9_]*@/$archive_sha/g" \
  "$out/source/scripts/templates/grove.rb.tmpl" >"$out/grove.rb"
brew_prefix="$out/brew"
keg="$brew_prefix/Cellar/grove/$version"
mkdir -p "$keg" "$brew_prefix/opt" "$brew_prefix/share/ravel/skills"
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/candidate-keg.rb" "$out/grove.rb" "$out/stage/$top" "$keg" ||
  fail "Homebrew's install method of the candidate formula failed"
ln -s "../Cellar/grove/$version" "$brew_prefix/opt/grove"
actual_ids="$(find "$keg/share/ravel/skills" -mindepth 1 -maxdepth 1 -exec basename {} \; | LC_ALL=C sort)"
[[ "$actual_ids" == "$expected_ids" ]] ||
  fail "the keg holds the sources '${actual_ids//$'\n'/ }', not '${expected_ids//$'\n'/ }'"
while IFS= read -r id; do
  ln -s "../../../Cellar/grove/$version/share/ravel/skills/$id" "$brew_prefix/share/ravel/skills/$id"
done <<<"$actual_ids"
echo "candidate-keg: keg $keg, linked as $brew_prefix/opt/grove"

# 4. The resolutions. resolve RAVEL HOME prints the id, resourceDigest,
# originPath and comma-separated roots of each source, tab-separated, and
# keeps Ravel's whole answer in HOME/resolve.json.
resolve() {
  mkdir -p "$out/tmp"
  env -i HOME="$2" TMPDIR="$out/tmp" PATH=/usr/bin:/bin "$1" skills resolve --json >"$2/resolve.json" ||
    return 1
  python3 -I - "$2/resolve.json" <<'PY'
import json, os, sys
document = json.load(open(sys.argv[1]))
if document.get("status") != "resolved":
    sys.exit(f"the sources did not resolve: {document}")
for source in document["sources"]:
    roots = sorted(os.path.relpath(d["path"], source["snapshotRoot"])
                   for d in document["directories"] if d["source"] == source["id"])
    print("\t".join((source["id"], source["resourceDigest"], source["originPath"], ",".join(roots))))
PY
}
mkdir -p "$out/ravel/bin" "$out/ravel/share/ravel" "$out/home"
cp "$ravel_2_0" "$out/ravel/bin/ravel"
printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$brew_prefix/share/ravel/skills" \
  >"$out/ravel/share/ravel/installation.json"
resolve "$out/ravel/bin/ravel" "$out/home" | LC_ALL=C sort >"$out/keg.tsv" ||
  fail "Ravel 2.0 cannot resolve the keg's sources from $brew_prefix/share/ravel/skills"
[[ "$(cut -f1 "$out/keg.tsv")" == "$expected_ids" ]] ||
  fail "Ravel resolves the sources '$(cut -f1 "$out/keg.tsv" | tr '\n' ' ')', not '${expected_ids//$'\n'/ }'"
while IFS=$'\t' read -r id _ origin _; do
  [[ "$origin" == "$keg/share/ravel/skills/$id" ]] || fail "source $id lies outside the keg: $origin"
done <"$out/keg.tsv"

# The plain checkouts of the same commit: core in OUT/source, and each pack in
# the checkout that the build copied from. Ravel's digest covers the source's
# content, not its path, ID or roots.
mkdir -p "$out/reference-home/.config/ravel"
{
  printf 'grove-core\t%s\n' "$out/source/plugins/grove"
  while IFS=$'\t' read -r _ id _ path sha; do
    printf '%s\t%s/%s\n' "$id" "$(pack_checkout_dir "$out/source/target/external-packs" "$id" "$sha")" "$path"
  done <<<"$members"
} | python3 -I -c '
import json, os, sys
sources = []
for line in sys.stdin.read().splitlines():
    source_id, path = line.split("\t")
    roots = [root for root in ("skills", "craft") if os.path.isdir(os.path.join(path, root))]
    sources.append({"id": source_id, "type": "local", "path": path, "roots": roots})
print(json.dumps({"skills": {"schemaVersion": 1, "sources": sources}}))
' >"$out/reference-home/.config/ravel/settings.json"
resolve "$ravel_2_0" "$out/reference-home" | LC_ALL=C sort >"$out/reference.tsv" ||
  fail 'Ravel 2.0 cannot resolve the plain checkouts'
cut -f1,2 "$out/keg.tsv" >"$out/digests.tsv"
if ! diff -u <(cut -f1,2,4 "$out/reference.tsv") <(cut -f1,2,4 "$out/keg.tsv") >&2; then
  fail "a keg source's resourceDigest or roots differ from its plain checkout; see the diff above"
fi
echo "candidate-keg: each keg source's resourceDigest and roots equal those of its plain checkout"

# 5. The record of the candidate.
python3 -I - "$out" "$commit" "$version" "$target" "$archive" "$archive_sha" "$keg" "$ravel_2_0" \
  "${published:-}" <<'PY'
import hashlib, json, sys
out, commit, version, target, archive, archive_sha, keg, ravel, published = sys.argv[1:]
sources = []
for line in open(f"{out}/keg.tsv").read().splitlines():
    source_id, digest, origin, roots = line.split("\t")
    sources.append({"id": source_id, "resourceDigest": digest, "originPath": origin,
                    "roots": roots.split(",")})
record = {
    "schemaVersion": 1,
    "commit": commit,
    "publishedOn": published or None,
    "version": version,
    "target": target,
    "archive": {"path": archive, "sha256": archive_sha},
    "keg": keg,
    "opt": f"{out}/brew/opt/grove",
    "skillDirectory": f"{out}/brew/share/ravel/skills",
    "ravel": {"bin": ravel, "sha256": hashlib.sha256(open(ravel, "rb").read()).hexdigest()},
    "sources": sources,
}
open(f"{out}/candidate.json", "w").write(json.dumps(record, indent=2) + "\n")
PY
while IFS=$'\t' read -r id digest; do
  echo "candidate-keg: $id $digest"
done <"$out/digests.tsv"
if [[ -n "$accepted" ]]; then
  if ! diff -u "$accepted" "$out/digests.tsv" >&2; then
    fail "a digest differs from $accepted; see the diff above"
  fi
  echo "candidate-keg: each digest equals $accepted"
fi
echo "candidate-keg: wrote $out/candidate.json and $out/digests.tsv"

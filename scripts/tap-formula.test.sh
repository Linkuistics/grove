#!/usr/bin/env bash
# Exercise one skill formula of the Linkuistics tap in a private
# Homebrew-shaped prefix, without changing any real keg, HOME or tap.
#
#   tap-formula.test.sh FORMULA RAVEL_2_0_PREFIX RAVEL_FORMULA RAVEL_ARCHIVE REPO_URL SHA ID=PATH...
#
# FORMULA is the formula file, such as Formula/personal.rb of the tap. REPO_URL
# and SHA name the commit that the formula installs. Each ID=PATH names one
# Ravel source of the keg, share/ravel/skills/ID, and its source directory PATH
# in that commit. PATH . names the repository root. The roots of each source
# are those of `skills` and `craft` that its directory holds.
#
# 1. The script downloads the formula's URL and compares its sha256 with the
#    formula's. Homebrew's own install and test methods of the formula then run
#    in the private prefix (scripts/tap-formula.test.rb). The keg lies in
#    Cellar/<name>/<version>, an `opt` link names it, and a relative link in
#    the prefix's share/ravel/skills names each source, as `brew link` makes.
# 2. Each keg source holds the files of PATH in a plain checkout of SHA, each
#    with its Git mode.
# 3. Ravel 2.0 resolves each source through the prefix's share/ravel/skills,
#    with no settings. Its originPath lies in the keg, its directories are its
#    roots, and its resourceDigest equals that of the plain checkout.
# 4. When the keg root holds .claude-plugin/marketplace.json, `claude plugin
#    validate` accepts the keg. In an isolated Claude Code home, the documented
#    commands register the marketplace by its `opt` path and install each
#    plugin that it lists.
# 5. scripts/shared-directory-seam.sh links the formula's actual output with
#    Homebrew's own linker into private prefixes, beside Ravel's keg and the
#    seam's fixture formulae. Each source must keep the digest and roots of
#    its plain checkout. When FORMULA lies in a Git working copy, the seam
#    reads the formula's published version from that tap's origin/main.
#    RAVEL_FORMULA is Ravel's formula or its template, and RAVEL_ARCHIVE is
#    the recorded Ravel archive that it installs.
#
# "The keg layout" and "Hook adapters" of docs/specs/codex-skill-provisioning.md
# state the contract.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=scripts/pack-record.sh
source "$root/scripts/pack-record.sh"
fail() {
  echo "tap-formula: $*" >&2
  exit 1
}
(($# >= 7)) || fail 'usage: tap-formula.test.sh FORMULA RAVEL_2_0_PREFIX RAVEL_FORMULA RAVEL_ARCHIVE REPO_URL SHA ID=PATH...'
formula="$(cd "$(dirname "$1")" && pwd -P)/$(basename "$1")"
ravel_2_0="$(cd "$2" && pwd -P)/bin/ravel"
seam_ravel_formula="$3"
seam_ravel_archive="$4"
repo_url="$5"
sha="$6"
shift 6
[[ -f "$seam_ravel_formula" && -f "$seam_ravel_archive" ]] || fail 'the seam needs the Ravel formula and its archive'
[[ -f "$formula" ]] || fail "no formula file $formula"
[[ -x "$ravel_2_0" ]] || fail 'the Ravel 2.0 prefix needs bin/ravel'
[[ "$sha" =~ ^[0-9a-f]{40}$|^[0-9a-f]{64}$ ]] || fail "SHA must be a full commit ID, not $sha"
sources=()
for spec in "$@"; do
  [[ "$spec" =~ ^[a-z0-9][a-z0-9-]*=[^=]+$ ]] || fail "expected ID=PATH, not $spec"
  sources+=("$spec")
done

scratch="$(mktemp -d)"
# Ravel publishes each skill snapshot read-only.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT
scratch="$(cd "$scratch" && pwd -P)"
mkdir "$scratch/download" "$scratch/source" "$scratch/test" "$scratch/tmp"

# 1. The formula's own download, install and test.
brew_ruby() {
  HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/tap-formula.test.rb" "$@"
}
IFS=$'\t' read -r name version url expected_sha < <(brew_ruby source "$formula") ||
  fail "Homebrew cannot read $formula"
case "$url" in
  "${repo_url%.git}/archive/"*)
    [[ "$url" == "${repo_url%.git}/archive/$sha.tar.gz" ]] || fail "the formula's URL $url names no archive of $sha"
    ;;
esac
curl --fail --silent --show-error --location --output "$scratch/download/archive" "$url" ||
  fail "cannot download $url"
actual_sha="$(shasum -a 256 "$scratch/download/archive" | cut -d' ' -f1)"
[[ "$actual_sha" == "$expected_sha" ]] || fail "the archive's sha256 is $actual_sha, not the formula's $expected_sha"
tar -xf "$scratch/download/archive" -C "$scratch/source"
source_dir="$scratch/source"
# Homebrew stages a single top directory as the source root.
entries=("$source_dir"/*)
if ((${#entries[@]} == 1)) && [[ -d "${entries[0]}" ]]; then
  source_dir="${entries[0]}"
fi
brew_prefix="$scratch/brew"
keg="$brew_prefix/Cellar/$name/$version"
mkdir -p "$keg" "$brew_prefix/opt" "$brew_prefix/share/ravel/skills"
brew_ruby install "$formula" "$source_dir" "$keg" "$scratch/test" >"$scratch/caveats" ||
  fail "Homebrew's install or test method of $name failed"
echo "tap-formula: Homebrew installed and tested $name $version in a private prefix"
ln -s "../Cellar/$name/$version" "$brew_prefix/opt/$name"
for spec in "${sources[@]}"; do
  id="${spec%%=*}"
  [[ -d "$keg/share/ravel/skills/$id" ]] || fail "the keg holds no share/ravel/skills/$id"
  ln -s "../../../Cellar/$name/$version/share/ravel/skills/$id" "$brew_prefix/share/ravel/skills/$id"
done
actual_ids="$(find "$keg/share/ravel/skills" -mindepth 1 -maxdepth 1 -exec basename {} \; | LC_ALL=C sort)"
expected_ids="$(for spec in "${sources[@]}"; do echo "${spec%%=*}"; done | LC_ALL=C sort)"
[[ "$actual_ids" == "$expected_ids" ]] ||
  fail "the keg holds the sources '${actual_ids//$'\n'/ }', not '${expected_ids//$'\n'/ }'"

# 2. The plain checkout, and each keg source's files and modes.
pack_checkout "$scratch/checkout" "$repo_url" "$sha" >/dev/null || fail "cannot check out $sha from $repo_url"
keg_mode() {
  if [[ -L "$1" ]]; then
    echo 120000
  elif [[ -f "$1" && -x "$1" ]]; then
    echo 100755
  elif [[ -f "$1" ]]; then
    echo 100644
  else
    echo other
  fi
}
for spec in "${sources[@]}"; do
  id="${spec%%=*}" path="${spec#*=}"
  [[ -d "$scratch/checkout/$path" ]] || fail "the commit holds no directory $path"
  # PATH . names the repository root, whose files carry no prefix.
  prefix="$path/"
  [[ "$path" == . ]] && prefix=''
  expected="$(pack_git -C "$scratch/checkout" ls-tree -r --full-tree "$sha" -- "$path" |
    awk -v prefix="$prefix" -F '\t' '{
      split($1, f, " ")
      if (index($2, prefix) == 1) $2 = substr($2, length(prefix) + 1)
      print f[1] "\t" $2
    }' | LC_ALL=C sort)"
  [[ -n "$expected" ]] || fail "the commit holds no file under $path"
  # Both lists sort as whole lines, mode first.
  actual="$(cd "$keg/share/ravel/skills/$id" && find . \( -type f -o -type l \) | sed 's|^\./||' |
    while IFS= read -r entry; do printf '%s\t%s\n' "$(keg_mode "$entry")" "$entry"; done | LC_ALL=C sort)"
  if [[ "$actual" != "$expected" ]]; then
    diff -u <(echo "$expected") <(echo "$actual") >&2 || true
    fail "the keg's $id differs in its files or Git modes from $path at $sha; see the diff above"
  fi
  echo "tap-formula: the keg's $id holds the files of $path with their Git modes"
done

# 3. Ravel 2.0 resolutions. resolve RAVEL HOME prints the id, resourceDigest,
# originPath and the comma-separated roots of each source, tab-separated.
resolve() {
  env -i HOME="$2" TMPDIR="$scratch/tmp" PATH=/usr/bin:/bin "$1" skills resolve --json |
    python3 -I -c '
import json, os, sys
document = json.load(sys.stdin)
if document.get("status") != "resolved":
    sys.exit(f"the sources did not resolve: {document}")
for source in document["sources"]:
    roots = sorted(os.path.relpath(d["path"], source["snapshotRoot"])
                   for d in document["directories"] if d["source"] == source["id"])
    print("\t".join((source["id"], source["resourceDigest"], source["originPath"], ",".join(roots))))
'
}
mkdir -p "$scratch/reference-home/.config/ravel"
for spec in "${sources[@]}"; do
  printf '%s\t%s\n' "${spec%%=*}" "$scratch/checkout/${spec#*=}"
done | python3 -I -c '
import json, os, sys
sources = []
for line in sys.stdin.read().splitlines():
    source_id, path = line.split("\t")
    roots = [root for root in ("skills", "craft") if os.path.isdir(os.path.join(path, root))]
    sources.append({"id": source_id, "type": "local", "path": path, "roots": roots})
print(json.dumps({"skills": {"schemaVersion": 1, "sources": sources}}))
' >"$scratch/reference-home/.config/ravel/settings.json"
resolve "$ravel_2_0" "$scratch/reference-home" | LC_ALL=C sort >"$scratch/reference" ||
  fail 'Ravel 2.0 cannot resolve the plain checkout'
# The seam's accepted values: ID, digest and roots of each plain checkout.
cut -f1,2,4 "$scratch/reference" >"$scratch/accepted"
# A private copy of the 2.0 build, whose installation record names the
# prefix's share/ravel/skills. That HOME holds no settings.
mkdir -p "$scratch/keg-ravel/bin" "$scratch/keg-ravel/share/ravel" "$scratch/keg-home"
cp "$ravel_2_0" "$scratch/keg-ravel/bin/ravel"
printf '{"schemaVersion":1,"skillDirectory":"%s"}\n' "$brew_prefix/share/ravel/skills" \
  >"$scratch/keg-ravel/share/ravel/installation.json"
resolve "$scratch/keg-ravel/bin/ravel" "$scratch/keg-home" | LC_ALL=C sort >"$scratch/keg" ||
  fail "Ravel 2.0 cannot resolve the keg's sources from the prefix's share/ravel/skills"
[[ "$(cut -f1 "$scratch/keg")" == "$expected_ids" ]] ||
  fail "Ravel resolves the sources '$(cut -f1 "$scratch/keg" | tr '\n' ' ')', not '${expected_ids//$'\n'/ }'"
while IFS=$'\t' read -r id digest origin roots; do
  [[ "$origin" == "$keg/share/ravel/skills/$id" ]] || fail "source $id lies outside the keg: $origin"
  reference="$(awk -F '\t' -v id="$id" '$1 == id { print $2 "\t" $4 }' "$scratch/reference")"
  [[ "$digest"$'\t'"$roots" == "$reference" ]] ||
    fail "the keg's $id has digest $digest and roots $roots, but its plain checkout has ${reference//$'\t'/ and roots }"
  echo "tap-formula: Ravel 2.0 resolves the keg's $id with roots $roots and resourceDigest $digest, equal to its plain checkout"
done <"$scratch/keg"

# 4. The marketplace file of the keg's hook adapters, if it has one.
marketplace="$keg/.claude-plugin/marketplace.json"
if [[ -f "$marketplace" ]]; then
  command -v claude >/dev/null || fail 'claude is not on PATH; the keg marketplace check needs Claude Code'
  mkdir "$scratch/claude-home"
  claude_isolated() {
    env -u CLAUDE_CODE_PLUGIN_CACHE_DIR -u CLAUDE_CODE_PLUGIN_DIRS -u CLAUDE_CODE_PLUGIN_SEED_DIR \
      HOME="$scratch/claude-home" CLAUDE_CONFIG_DIR="$scratch/claude-home/.claude" claude "$@"
  }
  claude_isolated plugin validate "$keg" || fail "claude plugin validate refused the keg's marketplace file"
  marketplace_name="$(python3 -I -c 'import json, sys; print(json.load(open(sys.argv[1]))["name"])' "$marketplace")"
  claude_isolated plugin marketplace add "$brew_prefix/opt/$name" ||
    fail "claude plugin marketplace add refused $brew_prefix/opt/$name"
  while IFS= read -r plugin; do
    claude_isolated plugin install "$plugin@$marketplace_name" ||
      fail "claude plugin install refused $plugin@$marketplace_name"
    claude_isolated plugin list --json | python3 -I -c '
import json, sys
wanted = sys.argv[1]
plugins = json.load(sys.stdin)
if not any(p.get("id") == wanted and p.get("enabled") for p in plugins):
    sys.exit(f"{wanted} is not installed and enabled: {plugins}")
' "$plugin@$marketplace_name" || fail "$plugin@$marketplace_name is not installed after its install command"
    echo "tap-formula: the documented commands registered $marketplace_name and installed $plugin@$marketplace_name"
  done < <(python3 -I -c 'import json, sys; print("\n".join(p["name"] for p in json.load(open(sys.argv[1]))["plugins"]))' "$marketplace")
fi

# 5. The shared default directory seam, with the formula's own download.
seam_tap=()
if tap_root="$(git -C "$(dirname "$formula")" rev-parse --show-toplevel 2>/dev/null)"; then
  seam_tap=(--tap "$tap_root")
fi
bash "$root/scripts/shared-directory-seam.sh" --ravel-formula "$seam_ravel_formula" \
  --ravel-archive "$seam_ravel_archive" --subject "$name=$formula" --archive "$name=$scratch/download/archive" \
  --accepted "$name=$scratch/accepted" "${seam_tap[@]+"${seam_tap[@]}"}" ||
  fail 'the shared default directory seam failed; see its record above'
echo "tap-formula: $name $version passes"

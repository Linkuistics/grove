#!/usr/bin/env bash
# Exercise one skill formula of the Linkuistics tap in a private
# Homebrew-shaped prefix, without changing any real keg, HOME or tap.
#
#   tap-formula.test.sh FORMULA RAVEL_2_0_PREFIX RAVEL_FORMULA RAVEL_ARCHIVE REPO_URL SHA ID=PATH...
#
# FORMULA is the formula file, such as Formula/personal.rb of the tap. REPO_URL
# and SHA name the commit that the formula installs. Each ID=PATH names one
# Ravel source of the keg and its source directory PATH in that commit. PATH .
# names the repository root. The roots of each source are those of `skills`
# and `craft` that its directory holds. scripts/skill-formulae.json declares
# the sources and the layout of the formula:
#
# - projected: the source is a private payload, libexec/skill-sources/ID, and
#   share/ravel/skills/ID is a relative directory symlink to it.
# - direct: the source is a real directory, share/ravel/skills/ID. Only a
#   formula that is not yet projected keeps this layout.
#
# 1. The script downloads the formula's URL and compares its sha256 with the
#    formula's. Homebrew's own install and test methods of the formula then run
#    in the private prefix (scripts/tap-formula.test.rb). The keg lies in
#    Cellar/<name>/<full package version>, an `opt` link names it, and a
#    relative link in the prefix's share/ravel/skills names each source, as
#    `brew link` makes. The keg holds each source in its declared layout.
# 2. The payload of each source holds the files of PATH in a plain checkout of
#    SHA, each with its Git mode. Each regular file of the commit is a regular
#    file of the payload, and the payload holds no other type of entry.
# 3. Ravel 2.0 resolves each source through the prefix's share/ravel/skills,
#    with no settings. Its originPath is the payload in the keg, its
#    directories are its roots, and its resourceDigest equals that of the
#    plain checkout.
# 4. When the keg root holds .claude-plugin/marketplace.json, `claude plugin
#    validate` accepts the keg. In an isolated Claude Code home, the documented
#    commands register the marketplace by its `opt` path and install each
#    plugin that it lists.
# 5. When the keg holds a native-only plugin in libexec/plugins or a
#    marketplace file, its accepted subject is the keg of the formula on the
#    published main of the tap. The install and test methods of that formula
#    make that keg in a second private prefix. The files, permission modes,
#    links and contents of each native-only plugin and each marketplace file
#    equal those of the accepted keg. Hook declarations lie in those files,
#    and the script names each declaring file. A different payload needs the
#    capability trials of the plugin. "Skill formulae" of
#    docs/specs/owner-skill-delivery.md states that rule.
# 6. scripts/shared-directory-seam.sh links the formula's actual output with
#    Homebrew's own linker into private prefixes, beside Ravel's keg and the
#    seam's fixture formulae. Each source must keep the digest and roots of
#    its plain checkout. When FORMULA lies in a Git working copy, the seam
#    reads the formula's published version from that tap's origin/main. Each
#    other declared formula of the working copy also takes part, in its
#    declared layout, as `release:formula:test` runs the seam. A direct
#    formula takes part only in the failure control.
#    RAVEL_FORMULA is Ravel's formula or its template, and RAVEL_ARCHIVE is
#    the recorded Ravel archive that it installs.
#
# "The keg layout" and "Hook adapters" of docs/specs/codex-skill-provisioning.md
# state the contract. TAP_BASE overrides the published revision of the tap,
# which is origin/main as the last fetch of the tap recorded it.
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
# version is the full package version, with the revision of the formula.
IFS=$'\t' read -r name version url expected_sha < <(brew_ruby source "$formula") ||
  fail "Homebrew cannot read $formula"
# The declared layout and sources of the formula.
IFS=$'\t' read -r layout declared_ids < <(python3 -I -c '
import json, sys
formulae = json.load(open(sys.argv[1]))["formulae"]
if sys.argv[2] not in formulae:
    sys.exit(f"{sys.argv[2]} has no declaration in {sys.argv[1]}")
entry = formulae[sys.argv[2]]
print(entry["layout"] + "\t" + " ".join(sorted(entry["sources"])))
' "$root/scripts/skill-formulae.json" "$name") || fail "scripts/skill-formulae.json declares no layout for $name"
[[ "$layout" == projected || "$layout" == direct ]] || fail "$name has the unknown layout $layout"
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
expected_ids="$(for spec in "${sources[@]}"; do echo "${spec%%=*}"; done | LC_ALL=C sort)"
[[ "${expected_ids//$'\n'/ }" == "$declared_ids" ]] ||
  fail "SOURCES names '${expected_ids//$'\n'/ }', but scripts/skill-formulae.json declares '$declared_ids' for $name"
# payload ID: the canonical directory of source ID in the keg.
payload() {
  if [[ "$layout" == projected ]]; then
    echo "$keg/libexec/skill-sources/$1"
  else
    echo "$keg/share/ravel/skills/$1"
  fi
}
for spec in "${sources[@]}"; do
  id="${spec%%=*}"
  alias="$keg/share/ravel/skills/$id"
  if [[ "$layout" == projected ]]; then
    [[ -L "$alias" ]] || fail "the keg's share/ravel/skills/$id is not a symlink, so $name is not projected"
    [[ "$(readlink "$alias")" == "../../../libexec/skill-sources/$id" ]] ||
      fail "the keg's share/ravel/skills/$id names $(readlink "$alias"), not ../../../libexec/skill-sources/$id"
    [[ -d "$(payload "$id")" && ! -L "$(payload "$id")" ]] ||
      fail "the keg's libexec/skill-sources/$id is not a real directory"
    [[ "$(cd "$alias" && pwd -P)" == "$(payload "$id")" ]] ||
      fail "the keg's share/ravel/skills/$id leaves its private source"
    echo "tap-formula: the keg's share/ravel/skills/$id is a relative alias of its private payload libexec/skill-sources/$id"
  else
    [[ -d "$alias" && ! -L "$alias" ]] || fail "the keg's share/ravel/skills/$id is not a real directory, so $name is not direct"
    [[ ! -e "$keg/libexec/skill-sources/$id" ]] || fail "the direct keg holds libexec/skill-sources/$id"
    echo "tap-formula: the keg's share/ravel/skills/$id is a real directory, as the direct layout of $name declares"
  fi
  ln -s "../../../Cellar/$name/$version/share/ravel/skills/$id" "$brew_prefix/share/ravel/skills/$id"
done
actual_ids="$(find "$keg/share/ravel/skills" -mindepth 1 -maxdepth 1 -exec basename {} \; | LC_ALL=C sort)"
[[ "$actual_ids" == "$expected_ids" ]] ||
  fail "the keg holds the sources '${actual_ids//$'\n'/ }', not '${expected_ids//$'\n'/ }'"
if [[ "$layout" == projected ]]; then
  actual_ids="$(find "$keg/libexec/skill-sources" -mindepth 1 -maxdepth 1 -exec basename {} \; | LC_ALL=C sort)"
  [[ "$actual_ids" == "$expected_ids" ]] ||
    fail "the keg's libexec/skill-sources holds '${actual_ids//$'\n'/ }', not '${expected_ids//$'\n'/ }'"
fi

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
  # A payload entry that is no regular file, link or directory has no Git
  # mode. The comparison below gives a regular file of the commit only to a
  # regular file of the payload.
  others="$(cd "$(payload "$id")" && find . ! -type f ! -type l ! -type d)"
  [[ -z "$others" ]] || fail "the keg's $id holds entries that are no file, link or directory: ${others//$'\n'/ }"
  # Both lists sort as whole lines, mode first.
  actual="$(cd "$(payload "$id")" && find . \( -type f -o -type l \) | sed 's|^\./||' |
    while IFS= read -r entry; do printf '%s\t%s\n' "$(keg_mode "$entry")" "$entry"; done | LC_ALL=C sort)"
  if [[ "$actual" != "$expected" ]]; then
    diff -u <(echo "$expected") <(echo "$actual") >&2 || true
    fail "the keg's $id differs in its files or Git modes from $path at $sha; see the diff above"
  fi
  echo "tap-formula: the payload of the keg's $id holds the files of $path with their Git modes, and each regular file is regular"
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
  [[ "$origin" == "$(payload "$id")" ]] || fail "source $id has originPath $origin, not its payload $(payload "$id")"
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

# 5. The native-only payload against its accepted subject, the keg of the
# formula on the published main of the tap.
tap_root="$(git -C "$(dirname "$formula")" rev-parse --show-toplevel 2>/dev/null)" || tap_root=''
tap_base="${TAP_BASE:-origin/main}"
native=()
[[ ! -d "$keg/libexec/plugins" ]] || native+=(libexec/plugins)
for file in .claude-plugin/marketplace.json .agents/plugins/marketplace.json; do
  [[ ! -e "$keg/$file" ]] || native+=("$file")
done
# IFS starts with a newline, so the message joins the entries itself.
native_list="$(printf '%s ' "${native[@]+"${native[@]}"}")"
native_list="${native_list% }"
if ((${#native[@]})); then
  [[ -n "$tap_root" ]] ||
    fail "$name has the native-only payload '$native_list', but $formula lies in no tap with a published formula to accept it"
  mkdir "$scratch/accepted-formula" "$scratch/accepted-download" "$scratch/accepted-source" "$scratch/accepted-test"
  git -C "$tap_root" show "$tap_base:Formula/$name.rb" >"$scratch/accepted-formula/$name.rb" ||
    fail "the tap's $tap_base holds no Formula/$name.rb, the accepted subject of the native-only payload"
  IFS=$'\t' read -r _ accepted_version accepted_url accepted_sha < <(brew_ruby source "$scratch/accepted-formula/$name.rb") ||
    fail "Homebrew cannot read $tap_base:Formula/$name.rb"
  if [[ "$accepted_sha" == "$expected_sha" ]]; then
    cp "$scratch/download/archive" "$scratch/accepted-download/archive"
  else
    curl --fail --silent --show-error --location --output "$scratch/accepted-download/archive" "$accepted_url" ||
      fail "cannot download $accepted_url"
    actual_sha="$(shasum -a 256 "$scratch/accepted-download/archive" | cut -d' ' -f1)"
    [[ "$actual_sha" == "$accepted_sha" ]] || fail "the archive of $accepted_url has sha256 $actual_sha, not $accepted_sha"
  fi
  tar -xf "$scratch/accepted-download/archive" -C "$scratch/accepted-source"
  accepted_source="$scratch/accepted-source"
  entries=("$accepted_source"/*)
  if ((${#entries[@]} == 1)) && [[ -d "${entries[0]}" ]]; then
    accepted_source="${entries[0]}"
  fi
  accepted_keg="$scratch/accepted-brew/Cellar/$name/$accepted_version"
  mkdir -p "$accepted_keg"
  brew_ruby install "$scratch/accepted-formula/$name.rb" "$accepted_source" "$accepted_keg" "$scratch/accepted-test" \
    >/dev/null || fail "Homebrew's install or test method of $tap_base:Formula/$name.rb failed"
  echo "tap-formula: the accepted subject is $name $accepted_version of $tap_base:Formula/$name.rb, installed in a second private prefix"
  # native_inventory KEG: one line for each entry of the native-only payload,
  # with its type and its permission mode, link target or SHA-256.
  native_inventory() {
    local entry
    (cd "$1" && find "${native[@]}" -print) | LC_ALL=C sort | while IFS= read -r entry; do
      if [[ -L "$1/$entry" ]]; then
        printf '%s\tlink\t%s\n' "$entry" "$(readlink "$1/$entry")"
      elif [[ -d "$1/$entry" ]]; then
        printf '%s\tdirectory\t%s\n' "$entry" "$(stat -f '%Lp' "$1/$entry")"
      elif [[ -f "$1/$entry" ]]; then
        printf '%s\tfile\t%s\t%s\n' "$entry" "$(stat -f '%Lp' "$1/$entry")" "$(shasum -a 256 "$1/$entry" | cut -d' ' -f1)"
      else
        printf '%s\tother\n' "$entry"
      fi
    done
  }
  # A native-only entry of the accepted keg that the revision drops fails too.
  for file in libexec/plugins .claude-plugin/marketplace.json .agents/plugins/marketplace.json; do
    [[ ! -e "$accepted_keg/$file" || -e "$keg/$file" ]] || fail "the revised keg drops the accepted $file"
    [[ -e "$accepted_keg/$file" || ! -e "$keg/$file" ]] || fail "the revised keg adds $file, which the accepted keg lacks"
  done
  native_inventory "$keg" >"$scratch/native"
  native_inventory "$accepted_keg" >"$scratch/native-accepted"
  if ! cmp -s "$scratch/native" "$scratch/native-accepted"; then
    diff -u "$scratch/native-accepted" "$scratch/native" >&2 || true
    fail "the native-only payload of $name $version differs from its accepted subject $accepted_version; see the diff above. A changed payload needs its capability trials"
  fi
  # The files that declare hooks: SKILL.md frontmatter with hooks, a plugin's
  # hooks/hooks.json, and a manifest or marketplace entry with hooks.
  declaring="$(cd "$keg" && find "${native[@]}" -type f \( -name SKILL.md -o -name '*.json' \) -print |
    LC_ALL=C sort | python3 -I -c '
import json, re, sys
for name in sys.stdin.read().splitlines():
    text = open(name, encoding="utf-8").read()
    if name.endswith("SKILL.md"):
        match = re.match(r"---\n(.*?)\n---\n", text, re.S)
        if match and re.search(r"^hooks:", match.group(1), re.M):
            print(name)
    elif name.endswith("hooks/hooks.json"):
        print(name)
    else:
        document = json.loads(text)
        entries = [document] + (document.get("plugins") or [] if isinstance(document, dict) else [])
        if any(isinstance(entry, dict) and "hooks" in entry for entry in entries):
            print(name)
')"
  echo "tap-formula: the native-only payload of $name $version equals its accepted subject $accepted_version in files, modes, links and contents: $(wc -l <"$scratch/native" | tr -d ' ') entries of $native_list"
  declaring="${declaring:-none}"
  echo "tap-formula: the equal files that declare hooks: ${declaring//$'\n'/ }"
fi

# 6. The shared default directory seam, with the formula's own download.
seam_tap=()
[[ -z "$tap_root" ]] || seam_tap=(--tap "$tap_root" --tap-base "$tap_base" --tap-subjects)
bash "$root/scripts/shared-directory-seam.sh" --ravel-formula "$seam_ravel_formula" \
  --ravel-archive "$seam_ravel_archive" --subject "$name=$formula" --archive "$name=$scratch/download/archive" \
  --accepted "$name=$scratch/accepted" "${seam_tap[@]+"${seam_tap[@]}"}" ||
  fail 'the shared default directory seam failed; see its record above'
echo "tap-formula: $name $version passes"

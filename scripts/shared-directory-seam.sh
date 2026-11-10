#!/usr/bin/env bash
# The shared default directory seam: Homebrew's own linker links actual formula
# output into private prefixes, and Ravel resolves each prefix's shared
# default directory with no settings. No real keg, link, tap or HOME changes.
#
#   shared-directory-seam.sh --ravel-formula FILE [--ravel-archive FILE]
#       [--subject NAME=FORMULA]... [--archive NAME=FILE]... [--accepted NAME=FILE]...
#       [--tap DIR] [--tap-subjects] [--tap-base REV] [--record FILE]
#
# --ravel-formula  Ravel's formula, or its template scripts/templates/ravel.rb.tmpl
#                  of the Ravel repository. A template is rendered for the archive.
# --ravel-archive  The archive that the formula installs on this host. Without it,
#                  the seam downloads the formula's URL. The archive must be in a
#                  lane of scripts/ravel_target_inputs.py that records a build
#                  identity, and its bin/ravel must hold that identity.
# --subject        A formula under test. NAME is declared in scripts/skill-formulae.json.
# --archive        The archive of a formula under test. Without it, the seam
#                  downloads the formula's URL. The formula's sha256 must equal
#                  the archive's.
# --accepted       The accepted values of a formula under test: one line for each
#                  source, with its ID, resourceDigest and comma-separated roots,
#                  tab-separated, from a plain checkout. Without it, the seam
#                  takes them from the pristine private payloads.
# --tap            A tap working copy. The published formula of each subject is
#                  Formula/NAME.rb at REV of that tap, when its declaration
#                  names a published layout.
# --tap-subjects   Also make each declared formula of the tap's working copy,
#                  except an explicit subject, a formula under test.
# --tap-base       The published revision of the tap. The default is origin/main,
#                  as the tap's last fetch recorded it.
# --record         Write the run's JSON record here. The default is
#                  target/shared-directory-seam/<UTC time>.json.
#
# scripts/shared-directory-seam.rb runs the cases. "Shared default directory
# acceptance" of docs/specs/codex-skill-provisioning.md owns them. This seam
# needs Homebrew and a Ravel 2.0 build, so scripts/check.sh does not run it.
set -euo pipefail
IFS=$'\n\t'
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
fail() {
  echo "shared-directory-seam: $*" >&2
  exit 1
}
declarations="$root/scripts/skill-formulae.json"

ravel_formula=''
ravel_archive=''
tap=''
tap_subjects=0
tap_base='origin/main'
record=''
subjects=()
archives=()
accepted=()
while (($#)); do
  case "$1" in
    --ravel-formula) ravel_formula="${2:?--ravel-formula needs a file}" && shift 2 ;;
    --ravel-archive) ravel_archive="${2:?--ravel-archive needs a file}" && shift 2 ;;
    --subject) subjects+=("${2:?--subject needs NAME=FORMULA}") && shift 2 ;;
    --archive) archives+=("${2:?--archive needs NAME=FILE}") && shift 2 ;;
    --accepted) accepted+=("${2:?--accepted needs NAME=FILE}") && shift 2 ;;
    --tap) tap="${2:?--tap needs a directory}" && shift 2 ;;
    --tap-subjects) tap_subjects=1 && shift ;;
    --tap-base) tap_base="${2:?--tap-base needs a revision}" && shift 2 ;;
    --record) record="${2:?--record needs a file}" && shift 2 ;;
    -h | --help)
      sed -n '2,/^set -euo/p' "${BASH_SOURCE[0]}" | sed '$d' | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *) fail "unknown argument $1; see --help" ;;
  esac
done
[[ -n "$ravel_formula" ]] || fail 'the seam needs --ravel-formula; see --help'
[[ -f "$ravel_formula" ]] || fail "no Ravel formula $ravel_formula"
command -v brew >/dev/null || fail 'the seam needs Homebrew on PATH'
[[ -n "$record" ]] || record="$root/target/shared-directory-seam/$(date -u +%Y%m%dT%H%M%SZ).json"
mkdir -p "$(dirname "$record")"
record="$(cd "$(dirname "$record")" && pwd -P)/$(basename "$record")"

scratch="$(mktemp -d)"
# Ravel publishes each skill snapshot read-only.
trap 'chmod -R u+w "$scratch" 2>/dev/null; rm -rf "$scratch"' EXIT
scratch="$(cd "$scratch" && pwd -P)"
mkdir "$scratch/formulae" "$scratch/downloads" "$scratch/run"

absolute() {
  echo "$(cd "$(dirname "$1")" && pwd -P)/$(basename "$1")"
}
# value NAME LIST...: the FILE of the NAME=FILE entry of LIST, or nothing.
value() {
  local name="$1" entry
  shift
  for entry in "$@"; do
    [[ "${entry%%=*}" != "$name" ]] || {
      absolute "${entry#*=}"
      return
    }
  done
}
# formula_source FORMULA: its name, version, URL and sha256 on this host.
formula_source() {
  HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/tap-formula.test.rb" source "$1"
}
# archive_for FORMULA [ARCHIVE]: ARCHIVE, or a download of the formula's URL.
# Either way, its SHA-256 must equal the formula's sha256.
archive_for() {
  local formula="$1" archive="${2:-}" name version url sha actual
  IFS=$'\t' read -r name version url sha < <(formula_source "$formula") || fail "Homebrew cannot read $formula"
  if [[ -z "$archive" ]]; then
    archive="$scratch/downloads/$sha"
    if [[ ! -f "$archive" ]]; then
      curl --fail --silent --show-error --location --output "$archive.part" "$url" || fail "cannot download $url"
      mv "$archive.part" "$archive"
    fi
  fi
  actual="$(shasum -a 256 "$archive" | cut -d' ' -f1)"
  [[ "$actual" == "$sha" ]] || fail "$archive has SHA-256 $actual, not the sha256 $sha of $formula ($name $version)"
  echo "$archive"
}
# declared NAME FIELD: a field of NAME's declaration, or the refusal.
declared() {
  python3 -I - "$declarations" "$1" "$2" <<'PY'
import json, sys
declarations, name, field = sys.argv[1:]
formulae = json.load(open(declarations))["formulae"]
if name not in formulae:
    sys.exit(f"{name} has no declaration in {declarations}")
value = formulae[name][field]
print(" ".join(value) if isinstance(value, list) else ("" if value is None else value))
PY
}

# Ravel: render a template for its archive, then identify the archive's lane.
ravel_formula="$(absolute "$ravel_formula")"
[[ -z "$ravel_archive" ]] || ravel_archive="$(absolute "$ravel_archive")"
mkdir "$scratch/formulae/ravel"
if [[ "$ravel_formula" == *.tmpl ]]; then
  [[ -n "$ravel_archive" ]] || fail 'a Ravel template needs --ravel-archive'
  stem="$(basename "$ravel_archive" .tar.xz)"
  [[ "$stem" =~ ^ravel-v([0-9][^-]*)- ]] || fail "$ravel_archive is not named ravel-v<version>-<target>.tar.xz"
  ravel_sha="$(shasum -a 256 "$ravel_archive" | cut -d' ' -f1)"
  # Only this host's archive exists, so every sha256 placeholder takes its
  # digest, as scripts/release-formula.test.sh renders Grove's template.
  sed -e "s/@VERSION@/${BASH_REMATCH[1]}/g" -e "s/@SHA_[A-Z0-9_]*@/$ravel_sha/g" \
    "$ravel_formula" >"$scratch/formulae/ravel/ravel.rb"
  ! grep -n '@[A-Z_0-9]*@' "$scratch/formulae/ravel/ravel.rb" || fail "$ravel_formula has unresolved placeholders"
else
  cp "$ravel_formula" "$scratch/formulae/ravel/ravel.rb"
fi
ravel_archive="$(archive_for "$scratch/formulae/ravel/ravel.rb" "$ravel_archive")"
ravel_sha="$(shasum -a 256 "$ravel_archive" | cut -d' ' -f1)"
IFS=$'\t' read -r lane target source_commit identity < <(python3 -I - "$root/scripts" "$ravel_sha" <<'PY'
import sys
sys.path.insert(0, sys.argv[1])
import ravel_target_inputs
for lane in ravel_target_inputs.LANES.values():
    for target, pin in lane.pins.items():
        if pin == sys.argv[2] and lane.build_identity:
            print("\t".join((lane.runtime, target, lane.source_commit or "", lane.build_identity)))
            sys.exit(0)
sys.exit(f"no lane of scripts/ravel_target_inputs.py with a build identity records the Ravel archive {sys.argv[2]}")
PY
) || fail "the Ravel archive $ravel_archive is unrecorded"
ravel_member="$(tar -tf "$ravel_archive" | grep -E '^[^/]+/bin/ravel$')" || fail "$ravel_archive holds no bin/ravel"
# A file, not a pipe: grep -q exits at the first match, and pipefail would
# report tar's broken pipe.
tar -xOf "$ravel_archive" "$ravel_member" >"$scratch/ravel-executable" || fail "cannot read bin/ravel of $ravel_archive"
grep -aqF "$identity" "$scratch/ravel-executable" ||
  fail "the bin/ravel of $ravel_archive does not hold the build identity $identity of lane $lane"
echo "shared-directory-seam: Ravel archive $ravel_sha is lane $lane, $target, source $source_commit, build identity $identity"

# The formulae under test, and their published formulae.
if [[ -n "$tap" ]]; then
  tap="$(cd "$tap" && pwd -P)"
  git -C "$tap" rev-parse --verify --quiet "$tap_base^{commit}" >/dev/null || fail "the tap $tap has no revision $tap_base"
  tap_revision="$(git -C "$tap" rev-parse "$tap_base^{commit}")"
  if ((tap_subjects)); then
    while IFS= read -r name; do
      [[ "$name" != grove && -z "$(value "$name" "${subjects[@]+"${subjects[@]}"}")" ]] || continue
      [[ -f "$tap/Formula/$name.rb" ]] || fail "the tap $tap holds no Formula/$name.rb"
      subjects+=("$name=$tap/Formula/$name.rb")
    done < <(python3 -I -c 'import json, sys; print("\n".join(json.load(open(sys.argv[1]))["formulae"]))' "$declarations")
  fi
fi
entries="$scratch/formulae.tsv"
: >"$entries"
for subject in "${subjects[@]+"${subjects[@]}"}"; do
  name="${subject%%=*}"
  formula="$(absolute "${subject#*=}")"
  [[ -f "$formula" ]] || fail "no formula $formula for $name"
  layout="$(declared "$name" layout)" || fail "$name has no declaration"
  sources="$(declared "$name" sources)"
  mkdir -p "$scratch/formulae/subject/$name"
  cp "$formula" "$scratch/formulae/subject/$name/$name.rb"
  archive="$(archive_for "$formula" "$(value "$name" "${archives[@]+"${archives[@]}"}")")"
  printf 'subject\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n' "$name" "$scratch/formulae/subject/$name/$name.rb" "$archive" \
    "$layout" "$sources" "$(value "$name" "${accepted[@]+"${accepted[@]}"}")" "$formula" >>"$entries"
  published="$(declared "$name" published)"
  if [[ -n "$tap" && -n "$published" ]]; then
    mkdir -p "$scratch/formulae/published/$name"
    git -C "$tap" show "$tap_revision:Formula/$name.rb" >"$scratch/formulae/published/$name/$name.rb" ||
      fail "the tap's $tap_base holds no Formula/$name.rb"
    archive="$(archive_for "$scratch/formulae/published/$name/$name.rb")"
    printf 'published\t%s\t%s\t%s\t%s\t%s\t\t%s\n' "$name" "$scratch/formulae/published/$name/$name.rb" "$archive" \
      "$published" "$sources" "$tap_base:Formula/$name.rb" >>"$entries"
  fi
done

brew_revision="$(git -C "$(brew --repository)" rev-parse HEAD)"
python3 -I - "$entries" "$scratch" "$record" "$brew_revision" "$scratch/formulae/ravel/ravel.rb" "$ravel_formula" "$ravel_archive" \
  "$ravel_sha" "$lane" "$target" "$source_commit" "$identity" "${tap:-}" "${tap_revision:-}" \
  "$root/scripts/shared-directory-seam.rb" "$root/scripts/shared-directory-seam.sh" "$declarations" \
  >"$scratch/plan.json" <<'PY'
import json, sys
(entries, scratch, record, brew, formula, origin, archive, sha, lane, target, commit, identity, tap, tap_revision,
 *instrument) = sys.argv[1:]
formulae = []
for line in open(entries).read().splitlines():
    role, name, path, source, layout, sources, accepted, from_ = line.split("\t")
    formulae.append({"role": role, "name": name, "formula": path, "from": from_, "archive": source,
                     "layout": layout, "sources": sources.split(" "), "accepted": accepted or None})
print(json.dumps({
    "scratch": f"{scratch}/run", "record": record, "homebrewRevision": brew, "instrument": instrument,
    "tap": {"path": tap or None, "revision": tap_revision or None},
    "ravel": {"formula": formula, "from": origin, "archive": archive, "archiveSha256": sha, "lane": lane, "target": target,
              "sourceCommit": commit or None, "buildIdentity": identity},
    "formulae": formulae,
}, indent=2))
PY
HOMEBREW_NO_AUTO_UPDATE=1 brew ruby "$root/scripts/shared-directory-seam.rb" "$scratch/plan.json"

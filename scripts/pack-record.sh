#!/usr/bin/env bash
#
# Grove's pack record and the checkout of each member. SOURCED, never executed:
#
#   source "$(dirname "${BASH_SOURCE[0]}")/pack-record.sh"
#
# `plugins/packs.json` is Grove's record of the accepted pack commits.
# `docs/specs/codex-skill-provisioning.md`, "The pack record and the release",
# states it. The record is one JSON object. Each key is a source ID, and each
# member holds exactly `url`, `path` and `sha`:
#
#   {"research": {"url": "https://github.com/Linkuistics/ResearchSkills.git",
#                 "path": "plugins/research", "sha": "<full commit ID>"}}
#
# `scripts/check-packs.sh` checks each member from its checkout. The release
# build copies each source directory from the same checkout, so the keg holds
# the bytes that the check accepted. Both use the functions below, and neither
# makes a checkout of its own.

if [[ -n "${GROVE_PACK_RECORD_SOURCED:-}" ]]; then
  return 0
fi
GROVE_PACK_RECORD_SOURCED=1

# pack_record_members <record>: one tab-separated line for each member, in
# record order. A valid member gives `ok<TAB>id<TAB>url<TAB>path<TAB>sha`. A
# member that breaks the record's shape gives `bad<TAB>name<TAB>reason`, where
# name is the key in JSON form when the key breaks the grammar. A record that is
# missing, is not JSON, is not an object or repeats a key prints its error and
# returns 1. Python's JSON reader keeps the last of two equal keys, so a
# repeated key would otherwise hide a member.
#
# The source-ID grammar is Ravel's `id`. An ID is a lowercase ASCII token of 1
# to 64 characters. Single hyphens separate groups of letters and digits, and
# the first character is a letter. Ravel's docs/specs/skill-discovery.md,
# "Owner configuration", states it. `sha` is a full lowercase 40- or 64-hex
# object ID, as Ravel's `revision` is.
pack_record_members() {
  python3 -I - "$1" <<'PY'
import json, re, sys

ID = re.compile(r"[a-z][a-z0-9]*(-[a-z0-9]+)*\Z")
SHA = re.compile(r"([0-9a-f]{40}|[0-9a-f]{64})\Z")
FIELDS = ("url", "path", "sha")

path = sys.argv[1]


class RepeatedKey(ValueError):
    pass


def unique(pairs):
    keys = [key for key, _ in pairs]
    repeated = sorted({key for key in keys if keys.count(key) > 1})
    if repeated:
        raise RepeatedKey("a key repeats: " + ", ".join(json.dumps(key) for key in repeated))
    return dict(pairs)


try:
    with open(path) as handle:
        record = json.load(handle, object_pairs_hook=unique)
except OSError as error:
    sys.exit(f"error: cannot read the pack record {path}: {error.strerror}")
except RepeatedKey as error:
    sys.exit(f"error: the pack record {path} is ambiguous: {error}")
except ValueError as error:
    sys.exit(f"error: the pack record {path} is not JSON: {error}")
if not isinstance(record, dict):
    sys.exit(f"error: the pack record {path} is not a JSON object keyed by source ID")


def problem(key, member):
    if not ID.match(key) or len(key) > 64:
        return "the key is not a source ID: 1 to 64 lowercase letters and digits, single hyphens between groups, a letter first"
    if not isinstance(member, dict):
        return "the member is not an object with url, path and sha"
    unknown = sorted(set(member) - set(FIELDS))
    if unknown:
        return "the member has a field outside url, path and sha: " + ", ".join(unknown)
    for field in FIELDS:
        if not isinstance(member.get(field), str) or not member[field]:
            if field == "sha":
                return "the member pins no full commit in sha (absent)"
            return f"the member has no {field}"
        if any(ord(c) < 0x20 for c in member[field]):
            return f"the member's {field} holds a control character"
    if not SHA.match(member["sha"]):
        return f"the member pins no full commit in sha ({member['sha']})"
    parts = member["path"].split("/")
    if member["path"].startswith("/") or any(p in ("", ".", "..") for p in parts):
        return f"the member's path is not a normalised relative path ({member['path']})"
    return None


for key in record:
    member = record[key]
    reason = problem(key, member)
    if reason is None:
        print("\t".join(["ok", key, member["url"], member["path"], member["sha"]]))
    else:
        name = key if ID.match(key) and len(key) <= 64 else json.dumps(key)
        print("\t".join(["bad", name, reason]))
PY
}

# pack_checkout_dir <cache> <id> <sha>: where the checkout of a member lies.
pack_checkout_dir() {
  printf '%s/%s/%s\n' "$1" "$2" "$3"
}

# pack_git <arg>...: git with the global and system configuration off and no
# prompt, so the user's configuration cannot change a checkout.
pack_git() {
  GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1 GIT_TERMINAL_PROMPT=0 git "$@"
}

# pack_checkout <dir> <url> <sha>: a clean plain checkout of <sha> at <dir>.
# It reuses <dir> only when its HEAD is <sha> and `git status --porcelain
# --ignored` prints nothing. Otherwise it replaces <dir> by a fresh shallow
# fetch of <sha>. It reports what it did on stdout and returns 1 on failure.
pack_checkout() {
  local dir="$1" url="$2" sha="$3" head
  if [[ -d "${dir}/.git" ]] &&
    [[ "$(pack_git -C "${dir}" rev-parse HEAD 2>/dev/null)" == "${sha}" ]] &&
    [[ -z "$(pack_git -C "${dir}" status --porcelain --ignored)" ]]; then
    echo "  reusing the clean checkout of ${sha}"
    return 0
  fi
  rm -rf "${dir}"
  mkdir -p "${dir}"
  # A 64-hex ID names a SHA-256 repository, and only a SHA-256 repository can
  # fetch it.
  if ((${#sha} == 64)); then
    pack_git init --quiet --object-format=sha256 "${dir}"
  else
    pack_git init --quiet "${dir}"
  fi
  pack_git -C "${dir}" fetch --quiet --depth 1 "${url}" "${sha}" || return 1
  pack_git -C "${dir}" checkout --quiet --detach FETCH_HEAD || return 1
  head="$(pack_git -C "${dir}" rev-parse HEAD)"
  if [[ "${head}" != "${sha}" ]]; then
    echo "  error: the checkout is at ${head}, not ${sha}" >&2
    return 1
  fi
  echo "  checked out ${sha} from ${url}"
}

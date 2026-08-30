#!/usr/bin/env bash
#
# Install this repo's bundled skills into non-Claude-Code agent harnesses by
# symlinking each eligible skill directory into that harness's personal skills
# folder.
#
# Claude Code does NOT need this script. Install there via the marketplace:
#   /plugin marketplace add Linkuistics/grove
#   /plugin install linkuistics@linkuistics
#   /plugin install testanyware@linkuistics
#
# Both bundled plugins are scanned. Which harnesses a skill reaches is the
# skill's own declaration, not this script's list: every SKILL.md carries a
# `harnesses:` frontmatter key, and this script installs a skill only where that
# key allows it (see plugins/CONTEXT.md, "Harness eligibility"). A skill whose
# instructions are unfollowable off Claude Code — `guardrail`, whose whole
# mechanism is a Claude Code hook — declares `[claude-code]` and is skipped here.
#
# For the harnesses below, the SKILL.md format is shared but there is no
# package manager, so symlinks are the install mechanism. Because the targets
# are symlinks, a later `git pull` in this repo updates the content in place —
# no need to re-run this script unless skills are added, removed, or change
# their eligibility.
#
# A re-run reconciles rather than only adding: every symlink in a harness's
# skills directory that points into a checkout of this repo is this script's to
# manage, so one whose skill has been deleted, renamed, or made ineligible for
# that harness is removed rather than left dangling.
#
# Usage: ./plugins/install.sh [--force]

set -euo pipefail
IFS=$'\n\t'

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
plugins_dir="${repo_root}/plugins"

print_usage() {
  cat <<'EOF'
usage: ./plugins/install.sh [--force]

  --force      link from this tree even when it is not the repo's main
               checkout — every link then dangles once this tree is
               removed, so re-run from the main checkout to repair
  -h, --help   show this help
EOF
}

force=0
for arg in "$@"; do
  case "${arg}" in
    --force) force=1 ;;
    -h | --help)
      print_usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: ${arg}" >&2
      print_usage >&2
      exit 2
      ;;
  esac
done

if ! compgen -G "${plugins_dir}/*/skills/*/SKILL.md" >/dev/null; then
  echo "error: no bundled skills found under ${plugins_dir}/*/skills/" >&2
  exit 1
fi

# print_real_path <dir>: <dir> with symlinks resolved; nothing if unreadable.
# `realpath` is not assumed — it is GNU coreutils on Linux and a comparatively
# recent addition on macOS.
print_real_path() {
  (cd "$1" 2>/dev/null && pwd -P)
}

# print_main_repo <dir>: the main repo behind the working tree at <dir> — the
# checkout that a linked git worktree or a secondary jj workspace belongs to,
# and <dir> itself for a plain checkout. Prints nothing and returns non-zero
# when <dir> is under no probeable VCS (an unpacked tarball, or jj/git absent
# from PATH); that deliberately disables the guard below rather than blocking
# an install this script cannot judge.
#
# jj-first, mirroring the grove binary's `repo::vcs_of` (symmetric-vcs-rule):
# a `.jj/` decides even when a `.git` sits beside it, as in a colocated repo.
# That ordering is load-bearing here, not cosmetic — a secondary jj workspace
# of a colocated repo is not a git worktree at all, so the git probe reports
# "not a repository" and would leave exactly the case this guard exists for
# undetected.
#
# `--ignore-working-copy` keeps the probe read-only: every other jj invocation
# snapshots the working copy as a side effect, and an install-time check has no
# business mutating the tree it is auditing.
print_main_repo() {
  local dir="$1" common_dir
  if [[ -d "${dir}/.jj" ]]; then
    jj -R "${dir}" workspace root --name default --ignore-working-copy 2>/dev/null
    return
  fi
  # The marker must be at <dir> itself, exactly as `vcs_of` requires — `.git`
  # as a directory in a checkout, as a gitfile in a linked worktree. Probing
  # without that check would let `git rev-parse` walk *up* out of an unpacked
  # or vendored copy of this repo and report the enclosing repository as the
  # main checkout, refusing an install that is perfectly fine.
  [[ -e "${dir}/.git" ]] || return 1
  common_dir="$(git -C "${dir}" rev-parse --git-common-dir 2>/dev/null)" || return 1
  # git prints this relative to <dir> from a plain checkout (".git") and
  # absolute from a linked worktree; the main repo is its parent either way.
  [[ "${common_dir}" == /* ]] || common_dir="${dir}/${common_dir}"
  print_real_path "${common_dir}/.."
}

# The workspace guard. This script links from whichever tree it lives in and
# re-links every skill unconditionally, so running it from a secondary jj
# workspace or a linked git worktree silently re-points *all* installed skills
# at a tree that is usually ephemeral. Nothing surfaces the breakage: `ln -s`
# does not require its target to persist, the harnesses read SKILL.md lazily,
# and a dangling skill directory reads as "skill not installed" rather than as
# an error.
#
# It refuses rather than warns because the damage is silent and delayed — a
# warning among 48 `ok` lines is read once, weeks before the symptom. It takes
# `--force` rather than refusing outright because linking from a side tree is
# sometimes deliberate: testing an unmerged skill against a live harness.
main_repo="$(print_main_repo "${repo_root}" || true)"
if [[ -n "${main_repo}" && "$(print_real_path "${repo_root}")" != "$(print_real_path "${main_repo}")" ]]; then
  if ((force)); then
    echo "warn   linking from ${repo_root}" >&2
    echo "warn   this is not the main checkout (${main_repo}); every link below" >&2
    echo "warn   dangles once this tree is removed — re-run there to repair" >&2
  else
    cat >&2 <<EOF
error: refusing to install from a secondary working tree.

  this tree:  ${repo_root}
  main repo:  ${main_repo}

install.sh links skills from the tree it lives in, and re-links every skill
unconditionally — so installing from here would re-point *all* of them at this
tree. Nothing would report it: a symlink whose target later disappears reads as
"skill not installed", not as an error.

  install normally:  cd "${main_repo}" && ./plugins/install.sh
  link here anyway:  ./plugins/install.sh --force
                     (deliberate — e.g. testing an unmerged skill live; re-run
                      from the main repo afterwards to repair the links)
EOF
    exit 1
  fi
fi

# Personal skill directories for harnesses that follow the SKILL.md open
# standard, each tagged with the harness id a `harnesses:` key names. Encoded as
# `id:path` rather than an associative array so this stays bash 3.2 clean (the
# macOS system bash). Each is installed only if its parent harness directory
# exists.
harness_targets=(
  "codex:${HOME}/.codex/skills"
  "gemini:${HOME}/.gemini/skills"
  "pi:${HOME}/.pi/agent/skills"
)

# print_frontmatter_value <file> <key>: the raw value of <key> from the leading
# `---` block, or nothing when the key is absent. Deliberately a first-block-only
# scan: a body line that happens to start with the key must not be mistaken for
# metadata.
print_frontmatter_value() {
  awk -v key="$2" '
    /^---[[:space:]]*$/ { block++; if (block > 1) exit; next }
    block == 1 && index($0, key ":") == 1 {
      sub("^" key ":[[:space:]]*", "")
      print
      exit
    }
  ' "$1"
}

# print_harnesses <skill-dir>: the declared harness ids, one per line, or
# nothing when the skill declares none. The value is a YAML flow list —
# `[any]`, `[claude-code, codex]` — which keeps it a single line that both a
# harness's YAML parser and this awk-free split agree on.
print_harnesses() {
  local raw
  raw="$(print_frontmatter_value "$1/SKILL.md" harnesses)"
  [[ -n "${raw}" ]] || return 0
  raw="${raw#[}"
  raw="${raw%]}"
  local IFS=','
  local id
  for id in ${raw}; do
    # trim surrounding whitespace and any quoting
    id="${id#"${id%%[![:space:]]*}"}"
    id="${id%"${id##*[![:space:]]}"}"
    id="${id//\"/}"
    id="${id//\'/}"
    if [[ -n "${id}" ]]; then printf '%s\n' "${id}"; fi
  done
}

# is_eligible <skill-dir> <harness-id>: whether the skill declares itself
# installable on that harness. `any` is a claim about the skill — nothing in it
# depends on a particular harness's affordances — rather than an enumeration
# that goes stale the next time a harness is added here.
is_eligible() {
  local id
  while IFS= read -r id; do
    [[ "${id}" == "any" || "${id}" == "$2" ]] && return 0
  done < <(print_harnesses "$1")
  return 1
}

# Collect every bundled skill once: `<name>\t<absolute path>`, sorted by name so
# the report reads the same on every run and on every filesystem.
skill_records=()
while IFS= read -r skill_md; do
  skill_path="$(dirname "${skill_md}")"
  skill_records+=("$(basename "${skill_path}")	${skill_path}")
done < <(find "${plugins_dir}" -mindepth 4 -maxdepth 4 -name SKILL.md)
# Sorted by name — the leading field — so the report reads identically on every
# run, whatever order the filesystem hands `find` its entries in.
# shellcheck disable=SC2207  # records are newline-delimited by construction
IFS=$'\n' skill_records=($(printf '%s\n' "${skill_records[@]}" | sort))
IFS=$'\n\t'

# The plugin directory names this repo ships (`linkuistics`, `testanyware`).
# They are what makes a symlink target recognisable as pointing into a checkout
# of this repo — see is_ours.
plugin_names=()
for plugin_skills in "${plugins_dir}"/*/skills; do
  [[ -d "${plugin_skills}" ]] || continue
  plugin_names+=("$(basename "$(dirname "${plugin_skills}")")")
done

# is_ours <link>: whether a symlink in a harness's skills directory was placed
# by this script — from *any* checkout of this repo, not only this one, so a
# `--force` run from a side tree still reconciles links laid down by the main
# checkout.
#
# The test is the target's shape: `…/plugins/<plugin>/skills/<anything>` where
# <plugin> is a plugin directory this repo ships. Deliberately *not* also
# "<anything> is a skill we currently ship" — the links most in need of
# reclaiming are the ones whose skill was deleted or renamed, and that extra
# condition would disown exactly those. The plugin segment survives a skill's
# deletion, which is what makes it the durable half of the signature.
#
# A textual match on the stored target rather than a resolved one, so an
# already-dangling link is still recognised as ours.
is_ours() {
  local target plugin
  target="$(readlink "$1")"
  for plugin in "${plugin_names[@]}"; do
    case "${target}" in
      */plugins/"${plugin}"/skills/*) return 0 ;;
    esac
  done
  return 1
}

linked=0
unlinked=0
skipped=0
blocked=()
declared_none=()
personal=()

for record in "${skill_records[@]}"; do
  skill_path="${record#*	}"
  if [[ -z "$(print_harnesses "${skill_path}")" ]]; then
    declared_none+=("${record%%	*}")
  fi
  if [[ "$(print_frontmatter_value "${skill_path}/SKILL.md" assumes-personal-setup)" == "true" ]]; then
    personal+=("${record%%	*}")
  fi
done

for target in "${harness_targets[@]}"; do
  harness_id="${target%%:*}"
  target_root="${target#*:}"
  harness_home="$(dirname "${target_root}")"
  if [[ ! -d "${harness_home}" ]]; then
    echo "skip   ${harness_home}  (harness not installed)"
    continue
  fi
  mkdir -p "${target_root}"

  # Reconcile first: drop every link of ours that this pass will not re-create,
  # so a skill deleted, renamed, or newly ineligible for this harness leaves no
  # link behind. Left in place it would read as "skill installed" while
  # resolving to nothing, or as a live skill this harness must not have.
  for link in "${target_root}"/*; do
    [[ -L "${link}" ]] || continue
    is_ours "${link}" || continue
    name="$(basename "${link}")"
    keep=0
    for record in "${skill_records[@]}"; do
      if [[ "${record%%	*}" == "${name}" ]] && is_eligible "${record#*	}" "${harness_id}"; then
        keep=1
        break
      fi
    done
    if ((!keep)); then
      rm "${link}"
      echo "unlink ${harness_id}  ${name}  (no longer eligible here)"
      unlinked=$((unlinked + 1))
    fi
  done

  for record in "${skill_records[@]}"; do
    skill_name="${record%%	*}"
    skill_path="${record#*	}"
    if ! is_eligible "${skill_path}" "${harness_id}"; then
      # Say so, every run. A skipped skill is otherwise indistinguishable from a
      # broken install: nothing appears in the directory and nothing explains it.
      declared="$(print_harnesses "${skill_path}" | paste -sd, -)"
      echo "skip   ${harness_id}  ${skill_name}  (harnesses: ${declared:-<none declared>})"
      skipped=$((skipped + 1))
      continue
    fi
    link="${target_root}/${skill_name}"
    if [[ -L "${link}" ]]; then
      rm "${link}"
    elif [[ -e "${link}" ]]; then
      # Refuse rather than warn, for the reason the workspace guard above
      # refuses: the skill is not installed, nothing in the harness says so, and
      # a `warn` line among 48 `ok` lines is read once. The live case was real —
      # the `grove` binary used to provision its own methodology into
      # `~/.codex/skills/grove` and `~/.pi/agent/skills/grove`, which is exactly
      # a non-symlink at a path this script wants. No build writes those since
      # `delete-provisioning-k19`, so what remains is one left behind by an older
      # build, or a directory a user made. The run continues so every other skill
      # still installs, and the exit status and the closing report carry the
      # failure.
      echo "error  ${link} exists and is not a symlink — ${skill_name} not installed" >&2
      blocked+=("${link}")
      continue
    fi
    ln -s "${skill_path}" "${link}"
    linked=$((linked + 1))
  done
  echo "ok     ${target_root}"
done

if ((${#declared_none[@]})); then
  echo "note   no harnesses: key — installed nowhere but Claude Code:" >&2
  for skill_name in "${declared_none[@]}"; do
    echo "note     ${skill_name}  (declare one in its SKILL.md frontmatter)" >&2
  done
fi

if ((${#personal[@]})); then
  echo "note   assumes-personal-setup — names the author's own models, profiles"
  echo "note   or machine config; review before relying on it:"
  for skill_name in "${personal[@]}"; do
    echo "note     ${skill_name}"
  done
fi

echo "linked ${linked} skill symlink(s); removed ${unlinked}; skipped ${skipped}"

if ((${#blocked[@]})); then
  cat >&2 <<EOF

error: ${#blocked[@]} skill(s) not installed — a real file or directory sits at
       the path this script installs to:

$(printf '         %s\n' "${blocked[@]}")

Something else owns those paths, and a directory this script did not create is
not its to replace. The usual cause is a grove build older than
delete-provisioning-k19, which swept its own methodology into
~/.codex/skills/grove and ~/.pi/agent/skills/grove on every invocation; no
current build writes them.

  keep both:    let the other owner keep the path; the skill stays uninstalled
                for that harness
  hand over:    remove the path yourself, then re-run this script

Nothing was overwritten.
EOF
  exit 1
fi

#!/usr/bin/env bash
#
# The methodology's delivery assertion, over the shipped skill set.
#
# `docs/adr/behavioural-coverage-asserts-delivery.md` states the rule: a
# methodology is delivered only where a session actually reads it. The
# instrument used to be a Rust suite over the binary's embedded `content/`
# (`tests/rule_ownership.rs`, `tests/loaded_path_budgets.rs` and four others).
# Once the methodology ships as a plugin, two of the four things that walk
# covered no longer exist in the binary, so the assertion moves here — to a
# dependency-free shell runner over the files a harness installs.
#
# Three assertions, over `skills/`:
#
#   1. Every behavioural rule is present on the composed loaded path of every
#      kind that binds it.
#   2. No rule has two owners.
#   3. Every file a skill names by path exists.
#
# It asserts nothing about how many kinds there are. A kind exists iff a skill
# of that name exists, so a spine with no `grove-<kind>` skills beside it is a
# legitimate intermediate state and is reported rather than failed.
#
# A fourth, temporary assertion rides along and is named as temporary: while the
# binary still provisions its own `content/`, the spine and `content/` carry the
# same bytes for every file they share. It dies with provisioning at
# `delete-provisioning-k19`.
#
# Usage: ./plugins/grove/conformance.sh [--verbose] [--skills <dir>] [--rules <file>]
#
# `--skills` and `--rules` point the run at a skill set, or a manifest, other
# than this plugin's own. They are what `conformance.test.sh` uses to watch every assertion below come back
# **dirty** against a subject known to be wrong: an instrument that has only
# ever read clean is indistinguishable from one that is broken.
#
# Exit status: 0 all assertions hold, 1 an assertion failed, 2 bad usage.

set -euo pipefail
IFS=$'\n\t'

plugin_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
repo_root="$(cd "${plugin_dir}/../.." && pwd)"
skills_dir="${plugin_dir}/skills"
spine="${skills_dir}/grove"
manifest="${plugin_dir}/conformance/rules.tsv"
paraphrases="${plugin_dir}/conformance/removed-paraphrases.tsv"

verbose=0
while (($#)); do
  case "$1" in
    --verbose | -v) verbose=1 ;;
    --rules)
      shift
      [[ $# -gt 0 && -f "$1" ]] || {
        echo "error: --rules needs an existing manifest" >&2
        exit 2
      }
      manifest="$(cd "$(dirname "$1")" && pwd)/$(basename "$1")"
      ;;
    --skills)
      shift
      [[ $# -gt 0 ]] || {
        echo "error: --skills needs a directory" >&2
        exit 2
      }
      skills_dir="$(cd "$1" 2>/dev/null && pwd)" || {
        echo "error: --skills: no such directory: $1" >&2
        exit 2
      }
      spine="${skills_dir}/grove"
      ;;
    -h | --help)
      sed -n '3,36p' "${BASH_SOURCE[0]}" | sed 's/^# \{0,1\}//'
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      echo "usage: ./plugins/grove/conformance.sh [--verbose] [--skills <dir>] [--rules <file>]" >&2
      exit 2
      ;;
  esac
  shift
done

failures=0
notes=0

fail() {
  echo "FAIL   $*" >&2
  failures=$((failures + 1))
}

note() {
  echo "note   $*"
  notes=$((notes + 1))
}

detail() {
  ((verbose)) && echo "       $*"
  return 0
}

[[ -d "${spine}" ]] || {
  echo "error: no spine skill at ${spine}" >&2
  exit 1
}
[[ -f "${manifest}" ]] || {
  echo "error: no rule manifest at ${manifest}" >&2
  exit 1
}

# ---------------------------------------------------------------------------
# The shipped skill set
# ---------------------------------------------------------------------------

# The kinds are read off the directory names, never counted and never listed:
# `skills/grove-<kind>` is a kind, `skills/grove` is the spine.
shipped_kinds=()
for dir in "${skills_dir}"/grove-*; do
  [[ -d "${dir}" ]] || continue
  shipped_kinds+=("$(basename "${dir}")")
  shipped_kinds[${#shipped_kinds[@]} - 1]="${shipped_kinds[${#shipped_kinds[@]} - 1]#grove-}"
done

# print_skill_dir <kind>: where that kind's skill lives.
print_skill_dir() { printf '%s\n' "${skills_dir}/grove-$1"; }

# print_skill_files <dir>: every markdown file in a skill, skill-relative.
print_skill_files() {
  (cd "$1" && find . -name '*.md' -type f | sed 's|^\./||' | sort)
}

# ---------------------------------------------------------------------------
# Normalisation
# ---------------------------------------------------------------------------

# normalised: emphasis and code markers stripped, case folded, whitespace
# collapsed. The corpus wraps prose at 80 columns and marks up mid-phrase, so a
# raw substring test misses real occurrences — the failure mode that reads as a
# clean sweep.
normalised() {
  tr -d '*_`' | tr '[:upper:]' '[:lower:]' | tr -s '[:space:]' ' '
}

# print_normalised_file <path>
print_normalised_file() { normalised <"$1"; }

# ---------------------------------------------------------------------------
# Assertion 3 — every file a skill names by path exists
# ---------------------------------------------------------------------------
#
# **Enumerate, then classify.** Every backticked token ending in `.md` is
# extracted from every shipped skill file, and each one is classified into
# exactly one of four classes. A token that fits none of them fails: a pattern
# list is complete only as far as the list, so the classification is closed
# instead, and a corpus file with a new filename shape cannot slip past.
#
#   skill       skill-relative (`SKILL.md`, `TASK-FORMAT.md`, `references/x.md`).
#               Must exist in this skill or in the spine.
#   grammar     a filename *template* or fragment, not a file: anything carrying
#               `<`/`>`, and the bare extension or a suffix (`-a.md`).
#   tree        an artifact in the grove *working tree*, not in the skill set:
#               a brief, the glossary, a leaf filename.
#   repo        a path into the repository under review (`docs/adr/x.md`).
#               Whether it resolves is that repository's business.
#
# A citation inside an HTML comment is provenance — the upstream file an adapted
# passage came from — and never something a session is told to open, so comments
# are blanked before extraction rather than classified after it.
#
# classify_reference <token>: prints the class, or nothing when unclassifiable.
classify_reference() {
  local token="$1"
  # A template or a fragment, not a name: `NN-<kind>--<slug>-k<key>.md`, the
  # bare `.md`, the `-a.md` suffix a research output path is described by.
  case "${token}" in
    *[\<\>]*) printf 'grammar\n'; return ;;
    -* | .*) printf 'grammar\n'; return ;;
  esac
  case "${token}" in
    references/*.md) printf 'skill\n'; return ;;
    */*) printf 'repo\n'; return ;;
  esac
  case "${token}" in
    # Tree artifacts a session writes or reads under `.grove/` and at the repo
    # root — named by the corpus, never carried by a skill.
    BRIEF.md | CONTEXT.md | CONTEXT-MAP.md) printf 'tree\n'; return ;;
  esac
  # A leaf filename: position, optional outcome infix, kind, slug, key.
  if [[ "${token}" =~ ^[0-9][0-9]-.*-k[0-9]+\.md$ ]]; then
    printf 'tree\n'
    return
  fi
  case "${token}" in
    *.md) printf 'skill\n'; return ;;
  esac
}

# print_references <file>: every backticked `*.md` token in a file, one per line.
print_references() {
  # HTML comments are blanked first: a `.md` inside one is provenance — the
  # upstream file an adapted passage came from — and never something a session
  # is told to open. Then split on backticks and keep the odd-numbered fields,
  # the spans *inside* backticks. Two markers inside one noun phrase are common
  # in this corpus, so a greedy single-line regex would join them.
  awk '
    {
      line = $0
      while (1) {
        if (incomment) {
          close_at = index(line, "-->")
          if (close_at == 0) { line = ""; break }
          line = substr(line, close_at + 3)
          incomment = 0
        }
        open_at = index(line, "<!--")
        if (open_at == 0) break
        rest = substr(line, open_at + 4)
        line = substr(line, 1, open_at - 1)
        incomment = 1
        close_at = index(rest, "-->")
        if (close_at == 0) break
        line = line substr(rest, close_at + 3)
        incomment = 0
      }
      n = split(line, part, "`")
      for (i = 2; i <= n; i += 2) {
        if (part[i] ~ /\.md$/) print part[i]
      }
    }
  ' "$1" | sort -u
}

# check_references <skill-dir> <label>
check_references() {
  local dir="$1" label="$2" file token class
  while IFS= read -r file; do
    while IFS= read -r token; do
      [[ -n "${token}" ]] || continue
      class="$(classify_reference "${token}")"
      case "${class}" in
        skill)
          if [[ -e "${dir}/${token}" || -e "${spine}/${token}" ]]; then
            detail "ref ok   ${label}/${file} -> ${token}"
          else
            fail "${label}/${file} names \`${token}\`, which exists in neither ${label} nor the spine"
          fi
          ;;
        grammar | tree | repo)
          detail "ref ${class} ${label}/${file} -> ${token}"
          ;;
        *)
          fail "${label}/${file} names \`${token}\`, which this runner cannot classify — extend classify_reference rather than ignoring it"
          ;;
      esac
    done < <(print_references "${dir}/${file}")
  done < <(print_skill_files "${dir}")
}

# ---------------------------------------------------------------------------
# The composed loaded path
# ---------------------------------------------------------------------------
#
# A kind's composed loaded path is its own skill's `SKILL.md`, the spine's
# `SKILL.md`, and the transitive closure of every skill-relative reference those
# files reach. That is the pointer graph the corpus itself realises, read off the
# shipped bytes rather than replayed from a table — a path computed by a parallel
# notion of what a session reads drifts from the real one and then lies.
#
# Paths are printed as `<owner>` — the skill-relative filename — because that is
# the grain the inventory's `owner` column is written at. A file reached in the
# kind's own skill and one reached in the spine are the same rule to a session.

# print_loaded_path <kind>: the closure, one skill-relative path per line.
print_loaded_path() {
  local kind="$1" dir seen frontier path token class
  dir="$(print_skill_dir "${kind}")"
  seen=$'\n'
  frontier=("SKILL.md")
  seen="${seen}SKILL.md"$'\n'
  while ((${#frontier[@]})); do
    path="${frontier[0]}"
    frontier=("${frontier[@]:1}")
    for base in "${dir}" "${spine}"; do
      [[ -f "${base}/${path}" ]] || continue
      while IFS= read -r token; do
        [[ -n "${token}" ]] || continue
        class="$(classify_reference "${token}")"
        [[ "${class}" == "skill" ]] || continue
        [[ -e "${dir}/${token}" || -e "${spine}/${token}" ]] || continue
        case "${seen}" in
          *$'\n'"${token}"$'\n'*) continue ;;
        esac
        seen="${seen}${token}"$'\n'
        frontier+=("${token}")
      done < <(print_references "${base}/${path}")
    done
  done
  printf '%s' "${seen}" | sed '/^$/d' | sort
}

# The spine's own path, for the report and for the intermediate state.
print_spine_path() {
  local seen frontier path token class
  seen=$'\n'"SKILL.md"$'\n'
  frontier=("SKILL.md")
  while ((${#frontier[@]})); do
    path="${frontier[0]}"
    frontier=("${frontier[@]:1}")
    [[ -f "${spine}/${path}" ]] || continue
    while IFS= read -r token; do
      [[ -n "${token}" ]] || continue
      class="$(classify_reference "${token}")"
      [[ "${class}" == "skill" ]] || continue
      [[ -e "${spine}/${token}" ]] || continue
      case "${seen}" in
        *$'\n'"${token}"$'\n'*) continue ;;
      esac
      seen="${seen}${token}"$'\n'
      frontier+=("${token}")
    done < <(print_references "${spine}/${path}")
  done
  printf '%s' "${seen}" | sed '/^$/d' | sort
}

# ---------------------------------------------------------------------------
# The manifest
# ---------------------------------------------------------------------------

print_manifest() { command grep -v '^#' "${manifest}" | command grep -v '^[[:space:]]*$'; }

# owner_is_shipped <owner-list>: whether every file the row names is carried by
# the spine or by some kind skill.
owner_is_shipped() {
  local owners="$1" owner found dir
  local IFS=','
  for owner in ${owners}; do
    found=0
    [[ -f "${spine}/${owner}" ]] && found=1
    if ((!found)); then
      for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
        dir="$(print_skill_dir "${kind}")"
        [[ -f "${dir}/${owner}" ]] && found=1 && break
      done
    fi
    ((found)) || return 1
  done
  return 0
}

# print_bound_kinds <load>: the shipped kinds a load predicate binds, one per
# line. `static(K)` names a kind set; `on(t) @ F` binds wherever F is reached.
# An unrecognised spelling prints nothing and sets `unreadable`.
unreadable=""
print_bound_kinds() {
  local load="$1" set kind
  unreadable=""
  if [[ "${load}" == static\(*\) ]]; then
    set="${load#static(}"
    set="${set%)}"
    case "${set}" in
      19) printf '%s\n' ${shipped_kinds[@]+"${shipped_kinds[@]}"} ;;
      18) for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
        [[ "${kind}" == "finish" ]] || printf '%s\n' "${kind}"
      done ;;
      research) for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
        case "${kind}" in research-a | research-b) printf '%s\n' "${kind}" ;; esac
      done ;;
      review-\*) for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
        case "${kind}" in integrate-review-*) ;; review-*) printf '%s\n' "${kind}" ;; esac
      done ;;
      integrate-review-\*) for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
        case "${kind}" in integrate-review-*) printf '%s\n' "${kind}" ;; esac
      done ;;
      \{*\}) set="${set#\{}"
        set="${set%\}}"
        for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
          [[ "${kind}" == "${set}" ]] && printf '%s\n' "${kind}"
        done ;;
      *) unreadable="static(${set})" ;;
    esac
    return
  fi
  if [[ "${load}" == on\(* ]]; then
    local trigger_file="${load##*) @ }"
    trigger_file="${trigger_file#content/}"
    if [[ "${trigger_file}" == "${load}" || -z "${trigger_file}" ]]; then
      unreadable="${load}"
      return
    fi
    for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
      if print_loaded_path "${kind}" | command grep -qxF "${trigger_file}"; then
        printf '%s\n' "${kind}"
      fi
    done
    return
  fi
  unreadable="${load}"
}

# ---------------------------------------------------------------------------
# Run
# ---------------------------------------------------------------------------

echo "grove conformance — ${skills_dir}"

# -- Assertion 3, over every shipped skill ------------------------------------
check_references "${spine}" "grove"
for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
  check_references "$(print_skill_dir "${kind}")" "grove-${kind}"
done

# -- Assertion 2, the single-source sweep -------------------------------------
#
# The sweep is phrase-scoped, and that bounds what it proves: a *paraphrase* — a
# second current-state statement of the same rule in other words — reads clean.
# `removed-paraphrases.tsv` pins the three known ones; nothing here claims a
# fourth does not exist.

# print_sites <phrase>: every shipped skill file stating it, `<label>/<path>`.
# The spine's `SKILL.md` is the *condition register* — it states situations and
# paths, never procedures — so it is swept separately, from the other direction.
print_sites() {
  local needle file label dir
  needle="$(printf '%s' "$1" | normalised)"
  {
    while IFS= read -r file; do
      [[ "${file}" == "SKILL.md" ]] && continue
      printf '%s\t%s\n' "grove" "${file}"
    done < <(print_skill_files "${spine}")
    for kind in ${shipped_kinds[@]+"${shipped_kinds[@]}"}; do
      dir="$(print_skill_dir "${kind}")"
      while IFS= read -r file; do
        printf '%s\t%s\n' "grove-${kind}" "${file}"
      done < <(print_skill_files "${dir}")
    done
  } | while IFS=$'\t' read -r label file; do
    if [[ "${label}" == "grove" ]]; then dir="${spine}"; else dir="$(print_skill_dir "${label#grove-}")"; fi
    if print_normalised_file "${dir}/${file}" | command grep -qF -- "${needle}"; then
      printf '%s\n' "${file}"
    fi
  done | sort -u
}

pending=0
unpinned=0
checked_ownership=0
checked_path=0
skipped_path=0

while IFS=$'\t' read -r rule owner class load phrase; do
  if ! owner_is_shipped "${owner}"; then
    pending=$((pending + 1))
    detail "pending  ${rule} (owner ${owner} not shipped yet)"
    continue
  fi

  # -- 2. No rule has two owners --------------------------------------------
  if [[ -n "${phrase}" ]]; then
    sites="$(print_sites "${phrase}" | paste -sd, -)"
    if [[ "${sites}" != "${owner}" ]]; then
      fail "${rule}: stated in [${sites:-nowhere}], and ${owner} must be the only site — a missing owner is a rule deleted by the move that was supposed to home it; an extra site is a second owner"
    else
      checked_ownership=$((checked_ownership + 1))
    fi
    if print_normalised_file "${spine}/SKILL.md" | command grep -qF -- "$(printf '%s' "${phrase}" | normalised)"; then
      fail "${rule}: the spine's SKILL.md states it — a condition register names the situation and the file, never the procedure"
    fi
  else
    unpinned=$((unpinned + 1))
  fi

  # -- 1. Behavioural rules on the composed loaded path ----------------------
  case "${class}" in
    *B*)
      print_bound_kinds "${load}" >/dev/null
      if [[ -n "${unreadable}" ]]; then
        fail "${rule}: load predicate \`${unreadable}\` is neither static(K) nor on(t) @ F"
        continue
      fi
      bound="$(print_bound_kinds "${load}")"
      if [[ -z "${bound}" ]]; then
        skipped_path=$((skipped_path + 1))
        continue
      fi
      while IFS= read -r kind; do
        [[ -n "${kind}" ]] || continue
        path="$(print_loaded_path "${kind}")"
        local_ok=1
        for one in ${owner//,/ }; do
          printf '%s\n' "${path}" | command grep -qxF "${one}" || local_ok=0
        done
        if ((local_ok)); then
          checked_path=$((checked_path + 1))
        else
          fail "${rule}: ${owner} is not on grove-${kind}'s composed loaded path, and ${kind} binds it — a rule a session never reaches is not delivered"
        fi
      done <<<"${bound}"
      ;;
  esac
done < <(print_manifest)

# -- Removed paraphrases ------------------------------------------------------
while IFS=$'\t' read -r rule site phrase; do
  [[ "${rule}" == \#* || -z "${rule}" ]] && continue
  sites="$(print_sites "${phrase}" | paste -sd, -)"
  if [[ -n "${sites}" ]]; then
    fail "${rule}: the wording ${site} used to restate it is back, in [${sites}] — the owner states it, and this file points at the owner"
  fi
done < <(command grep -v '^#' "${paraphrases}" | command grep -v '^[[:space:]]*$')

# -- The temporary agreement with the binary's `content/` ---------------------
#
# Dies at `delete-provisioning-k19`, with `content/` itself.
if [[ "${skills_dir}" == "${plugin_dir}/skills" && -d "${repo_root}/content" ]]; then
  diverged=0
  shared=0
  while IFS= read -r file; do
    # `SKILL.md` is **split**, not moved: `content/`'s copy is the kind router
    # the live binary still provisions, and the spine's is the condition
    # register. They are two documents and are expected to differ.
    [[ "${file}" == "SKILL.md" ]] && continue
    [[ -f "${repo_root}/content/${file}" ]] || continue
    shared=$((shared + 1))
    if ! cmp -s "${spine}/${file}" "${repo_root}/content/${file}"; then
      diverged=$((diverged + 1))
      fail "content/${file} and the spine's copy have diverged — the spine is the source, and the two ship at once until delete-provisioning-k19"
    fi
  done < <(print_skill_files "${spine}")
  ((diverged)) || note "spine and content/ agree on ${shared} shared file(s) — temporary, until delete-provisioning-k19"
fi

# -- Report -------------------------------------------------------------------

if ((${#shipped_kinds[@]} == 0)); then
  note "no grove-<kind> skills are shipped yet, so no kind binds anything and the"
  note "per-kind half of assertion 1 has nothing to quantify over. The spine's own"
  note "loaded path is:"
  while IFS= read -r path; do note "  ${path}"; done < <(print_spine_path)
  note "the kind skills land at plugin-kind-skills-k17"
fi

echo "rules  ${checked_ownership} single-source checked, ${checked_path} loaded-path checked"
echo "       ${pending} pending (owner not shipped yet), ${unpinned} with no pinned wording,"
echo "       ${skipped_path} bound to no shipped kind"

if ((failures)); then
  echo "FAILED ${failures} assertion(s)" >&2
  exit 1
fi
echo "ok     every assertion holds over the shipped skill set"

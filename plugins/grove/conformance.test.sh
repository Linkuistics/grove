#!/usr/bin/env bash
#
# Controls over `conformance.sh`.
#
# A control that has never been seen to fail is not a control: an instrument
# that reads clean everywhere is indistinguishable from a broken one, which is
# the single property these assertions exist to rule out. So every case below
# builds a skill set that is **wrong in one named way** and requires the runner
# to come back dirty about that specific thing — plus one case that requires it
# to come back clean, so "always fails" is ruled out too.
#
# Dependency-free bash, following `plugins/install.test.sh`: a scratch directory
# per case, no network, no package manager, nothing written outside the
# temporary tree.
#
# Usage: ./plugins/grove/conformance.test.sh

set -euo pipefail
IFS=$'\n\t'

plugin_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
runner="${plugin_dir}/conformance.sh"
spine_source="${plugin_dir}/skills/grove"

scratch="$(mktemp -d "${TMPDIR:-/tmp}/grove-conformance.XXXXXX")"
trap 'rm -rf "${scratch}"' EXIT

passed=0
failed=0

ok() {
  echo "ok     $*"
  passed=$((passed + 1))
}

bad() {
  echo "FAIL   $*" >&2
  failed=$((failed + 1))
}

# new_skill_set <name>: a fresh copy of the spine under its own skills root.
# Prints the skills root.
new_skill_set() {
  local root="${scratch}/$1"
  mkdir -p "${root}"
  cp -R "${spine_source}" "${root}/grove"
  printf '%s\n' "${root}"
}

# add_kind <root> <kind>: a minimal `grove-<kind>` skill that directs a load of
# the spine, which is the shape `plugin-kind-skills-k17` ships.
add_kind() {
  local root="$1" kind="$2" dir
  dir="${root}/grove-${kind}"
  mkdir -p "${dir}"
  cat >"${dir}/SKILL.md" <<SKILL
---
name: grove-${kind}
description: The ${kind} kind.
harnesses: [any]
---

# grove-${kind}

Read the \`grove\` spine's \`SKILL.md\` now.
SKILL
}

# run_runner <root>: the runner's combined output; never aborts the test file.
run_runner() {
  bash "${runner}" --skills "$1" 2>&1 || true
}

# expect_clean <root> <label>
expect_clean() {
  local out
  out="$(run_runner "$1")"
  if printf '%s\n' "${out}" | command grep -q '^ok  '; then
    ok "$2"
  else
    bad "$2 — expected a clean run, got:"
    printf '%s\n' "${out}" | sed 's/^/       /' >&2
  fi
}

# expect_dirty <root> <needle> <label>
expect_dirty() {
  local out
  out="$(run_runner "$1")"
  if printf '%s\n' "${out}" | command grep -qF -- "$2"; then
    ok "$3"
  else
    bad "$3 — expected a failure mentioning \"$2\", got:"
    printf '%s\n' "${out}" | sed 's/^/       /' >&2
  fi
}

# -- The clean reading, so "always fails" is ruled out ------------------------

root="$(new_skill_set clean)"
expect_clean "${root}" "an untouched spine reads clean"

# -- Assertion 3: a named path that does not exist ---------------------------

root="$(new_skill_set dangling)"
# shellcheck disable=SC2016  # a literal backtick span, not a command substitution
printf '\nWhen in doubt, read `references/does-not-exist.md`.\n' >>"${root}/grove/SKILL.md"
expect_dirty "${root}" "references/does-not-exist.md" \
  "a skill naming a file that does not exist is caught"

# -- Assertion 2: a second owner ---------------------------------------------
#
# `review-budget`'s canonical wording, restated in a file that does not own it.
# This is the duplication the single-source sweep exists to find.

root="$(new_skill_set second-owner)"
printf '\nA picked plain producer may materialise at most one in-session reviewer across the whole picked leaf.\n' \
  >>"${root}/grove/references/decompose.md"
expect_dirty "${root}" "review-budget" \
  "a rule stated by a second file is caught"

# -- Assertion 2: a rule deleted by the move that was supposed to home it -----

root="$(new_skill_set deleted-rule)"
command grep -v 'in-session reviewer across' \
  "${root}/grove/references/execute.md" >"${root}/execute.tmp"
mv "${root}/execute.tmp" "${root}/grove/references/execute.md"
expect_dirty "${root}" "review-budget" \
  "a rule its owner no longer states is caught"

# -- Assertion 2: the condition register restating a procedure ---------------

root="$(new_skill_set register-states-procedure)"
printf '\nA picked plain producer may materialise at most one in-session reviewer across the whole picked leaf.\n' \
  >>"${root}/grove/SKILL.md"
expect_dirty "${root}" "condition register" \
  "the spine's SKILL.md stating a procedure is caught"

# -- Assertion 2: a removed paraphrase coming back ---------------------------

root="$(new_skill_set paraphrase-returns)"
printf '\nThe glossary is the Ubiquitous Language — read every session, appended inline.\n' \
  >>"${root}/grove/references/retire.md"
expect_dirty "${root}" "glossary-is-the-forcing-function" \
  "a removed paraphrase returning is caught"

# -- Assertion 1: a bound kind that cannot reach the rule --------------------
#
# With a kind skill present, a `SKILL.md`-triggered row binds it. `SKILL.md` is
# the only file that names `references/bootstrap.md`, so severing that one
# pointer puts the rules bootstrap.md owns off the kind's composed loaded path
# while leaving them stated — delivered nowhere, and invisible to a
# single-source sweep. Severing a pointer *several* files carry proves nothing:
# the closure simply reaches the file another way, which is the first fixture
# this control was written with and the reason it is written this way now.

root="$(new_skill_set unreachable)"
add_kind "${root}" impl
expect_clean "${root}" "a kind skill that reaches the spine reads clean"

root="$(new_skill_set unreachable-broken)"
add_kind "${root}" impl
command grep -v 'references/bootstrap.md' "${root}/grove/SKILL.md" >"${root}/skill.tmp"
mv "${root}/skill.tmp" "${root}/grove/SKILL.md"
expect_dirty "${root}" "composed loaded path" \
  "a behavioural rule no bound kind can reach is caught"

# -- Assertion 1: a load predicate in neither form ---------------------------

root="$(new_skill_set bad-predicate)"
printf 'invented-rule\tSKILL.md\tB\twhenever\t\n' >"${scratch}/bad-rules.tsv"
out="$(bash "${runner}" --skills "${root}" --rules "${scratch}/bad-rules.tsv" 2>&1 || true)"
if printf '%s\n' "${out}" | command grep -qF -- 'neither static(K) nor on(t) @ F'; then
  ok "an unreadable load predicate is caught"
else
  bad "an unreadable load predicate is caught — got:"
  printf '%s\n' "${out}" | sed 's/^/       /' >&2
fi

# -- Report ------------------------------------------------------------------

echo "controls ${passed} passed, ${failed} failed"
((failed == 0))

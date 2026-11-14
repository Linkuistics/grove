#!/usr/bin/env bash
#
# Controls over `conformance.sh`.
#
# A control that has never been seen to fail is not a control. An instrument
# that reads clean everywhere is indistinguishable from a broken one, and these
# controls exist to rule that out. Each control therefore reads its fixture
# clean first, makes one mutation, and requires the runner to fail about that
# specific thing.
#
# A clean reading asserts the exit status 0. A dirty reading asserts the exit
# status 1 and the intended diagnostic. A runner that prints the diagnostic and
# exits 0 therefore fails the control, and so does a usage error.
#
# Each fixture ships the owner of each row in each manifest that its run reads.
# A core fixture copies the spine and some kinds, and it reads a manifest of
# only those rows of core's manifest whose owner skills it ships. The external
# pack is a fixture too, never the real research pack. "Controls that must
# fail" in `docs/specs/external-pack-contract.md` lists the pack controls.
#
# Dependency-free bash: a scratch directory per case, no network, no package
# manager, nothing written outside the temporary tree.
#
# Usage: ./plugins/grove/conformance.test.sh

# The fixtures and the needles hold literal Markdown code spans.
# shellcheck disable=SC2016

set -euo pipefail
IFS=$'\n\t'

plugin_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
runner="${plugin_dir}/conformance.sh"
spine_source="${plugin_dir}/skills/grove"
core_manifest="${plugin_dir}/conformance/rules.tsv"

scratch="$(mktemp -d "${TMPDIR:-/tmp}/grove-conformance.XXXXXX")"
trap 'rm -rf "${scratch}"' EXIT

passed=0
failed=0
out=""
status=0

ok() {
  echo "ok     $*"
  passed=$((passed + 1))
}

bad() {
  echo "FAIL   $*" >&2
  printf '%s\n' "${out}" | sed 's/^/       /' >&2
  failed=$((failed + 1))
}

# run <argument>...: the runner's combined output in `out`, its exit status
# in `status`. It never stops the test file.
run() {
  status=0
  out="$(bash "${runner}" "$@" 2>&1)" || status=$?
}

# assert_clean <label>: the last run held every assertion.
assert_clean() {
  if ((status == 0)) && printf '%s\n' "${out}" | command grep -q '^ok  '; then
    ok "$1"
  else
    bad "$1 — expected exit status 0 and a clean run, got ${status}:"
  fi
}

# assert_dirty <needle> <label>: the last run failed an assertion, and its
# output names the needle.
assert_dirty() {
  if ((status == 1)) && printf '%s\n' "${out}" | command grep -qF -- "$1"; then
    ok "$2"
  else
    bad "$2 — expected exit status 1 and a failure that names \"$1\", got ${status}:"
  fi
}

# assert_output <needle> <label>: the last run's output names the needle.
assert_output() {
  if printf '%s\n' "${out}" | command grep -qF -- "$1"; then
    ok "$2"
  else
    bad "$2 — expected output that names \"$1\":"
  fi
}

# assert_usage <label>: the last run refused its arguments.
assert_usage() {
  if ((status == 2)); then
    ok "$1"
  else
    bad "$1 — expected exit status 2, got ${status}:"
  fi
}

# ---------------------------------------------------------------------------
# Core fixtures
# ---------------------------------------------------------------------------

# new_skill_set <name>: a fresh copy of the spine under its own skills root.
# Prints the skills root.
new_skill_set() {
  local root="${scratch}/$1/core"
  mkdir -p "${root}"
  cp -R "${spine_source}" "${root}/grove"
  printf '%s\n' "${root}"
}

# add_kind <root> <kind>: the real shipped `grove-<kind>` skill, copied in. A
# stub would not do. The manifest's kind-owned rows name that skill's own file
# as their owner, so a stub makes each of them fail for the wrong reason.
add_kind() {
  cp -R "${plugin_dir}/skills/grove-$2" "$1/grove-$2"
}

# print_fixture_manifest <root>: the path of a manifest that holds the rows of
# core's manifest whose owner skills the fixture ships. The filter works at the
# grain of the skill, so a deleted owner file inside a shipped skill still
# fails. It runs again before each run, because `add_kind` adds skills.
print_fixture_manifest() {
  local root="$1" skills
  skills="$(cd "${root}" && for dir in */; do printf '%s ' "${dir%/}"; done)"
  awk -F'\t' -v skills="${skills}" '
    BEGIN { n = split(skills, list, " "); for (k = 1; k <= n; k++) shipped[list[k]] = 1 }
    /^#/ || /^[ \t]*$/ { next }
    {
      keep = 1
      m = split($2, owners, ",")
      for (k = 1; k <= m; k++) {
        if (owners[k] == "${prompt}") continue
        skill = owners[k]
        sub(/\/.*/, "", skill)
        if (!(skill in shipped)) keep = 0
      }
      if (keep) print
    }
  ' "${core_manifest}" >"${root}.rules.tsv"
  printf '%s\n' "${root}.rules.tsv"
}

# run_core <root> <argument>...: the runner over a core fixture.
run_core() {
  local root="$1"
  shift
  run --skills "${root}" --rules "$(print_fixture_manifest "${root}")" "$@"
}

# expect_clean <root> <label>
expect_clean() {
  run_core "$1"
  assert_clean "$2"
}

# expect_dirty <root> <needle> <label>
expect_dirty() {
  run_core "$1"
  assert_dirty "$2" "$3"
}

# drop_lines <file> <fixed string>: the file without each line that holds it.
drop_lines() {
  command grep -vF -- "$2" "$1" >"$1.tmp" || true
  mv "$1.tmp" "$1"
}

# replace_text <file> <old> <new>: each occurrence of a fixed string replaced.
replace_text() {
  awk -v old="$2" -v new="$3" '
    {
      line = $0
      result = ""
      while ((at = index(line, old)) > 0) {
        result = result substr(line, 1, at - 1) new
        line = substr(line, at + length(old))
      }
      print result line
    }
  ' "$1" >"$1.tmp"
  mv "$1.tmp" "$1"
}

# -- Core alone, over the shipped plugin -------------------------------------

run
assert_clean "core alone reads clean over the shipped plugin"

# -- The clean readings of a spine-only fixture ------------------------------

root="$(new_skill_set clean)"
expect_clean "${root}" "an untouched spine reads clean"
assert_output "no grove-<kind> skill is in this run" \
  "a run without kinds says that no kind binds a row"

root="$(new_skill_set node-file-references)"
expect_clean "${root}" "node-file references: the fixture reads clean"
printf '\nRead `_BRIEF.md` and `_extract.md`; their shape is in `BRIEF-FORMAT.md`.\n' \
  >>"${root}/grove/SKILL.md"
expect_clean "${root}" "root and titled node files are working-tree references"

root="$(new_skill_set leaf-position-reference)"
expect_clean "${root}" "leaf positions: the fixture reads clean"
printf '\nRead `100-impl--extract-k101.md`.\n' >>"${root}/grove/SKILL.md"
expect_clean "${root}" "leaf positions may use more than two digits"

# -- Assertion 3: a named path that does not exist ---------------------------

root="$(new_skill_set missing-brief-format)"
expect_clean "${root}" "missing brief format: the fixture reads clean"
rm "${root}/grove/BRIEF-FORMAT.md"
expect_dirty "${root}" 'names `BRIEF-FORMAT.md`' \
  "the brief format remains a checked skill reference"

root="$(new_skill_set dangling)"
expect_clean "${root}" "dangling path: the fixture reads clean"
printf '\nWhen in doubt, read `references/does-not-exist.md`.\n' >>"${root}/grove/SKILL.md"
expect_dirty "${root}" 'missing file: grove/SKILL.md names `references/does-not-exist.md`' \
  "a skill naming a file that does not exist is caught"

# -- Assertion 2: a rule removed from its owner ------------------------------

root="$(new_skill_set missing-node-rule)"
expect_clean "${root}" "missing node rule: the fixture reads clean"
drop_lines "${root}/grove/BRIEF-FORMAT.md" 'The node file is mandatory; its body is freeform'
expect_dirty "${root}" "every-node-carries-a-brief" \
  "removing the mandatory node-file rule is caught"

root="$(new_skill_set missing-chain-refusal)"
expect_clean "${root}" "missing chain refusal: the fixture reads clean"
drop_lines "${root}/grove/references/bootstrap.md" 'It refuses malformed levels'
expect_dirty "${root}" "brief-chain-refuses-gaps" \
  "removing the brief-chain refusal instruction is caught"

root="$(new_skill_set deleted-rule)"
expect_clean "${root}" "deleted rule: the fixture reads clean"
drop_lines "${root}/grove/references/execute.md" 'Routine checks have no per-leaf reviewer quota'
expect_dirty "${root}" "review-budget: stated in [nowhere]" \
  "a rule its owner no longer states is caught"

# -- Assertion 2: a second owner ---------------------------------------------
#
# `review-budget`'s canonical wording, restated in a file that does not own it.
# This is the duplication that the single-source sweep exists to find.

root="$(new_skill_set second-owner)"
expect_clean "${root}" "second owner: the fixture reads clean"
printf '\nRoutine checks have no per-leaf reviewer quota.\n' \
  >>"${root}/grove/references/decompose.md"
expect_dirty "${root}" "review-budget: stated in [grove/references/decompose.md,grove/references/execute.md]" \
  "a rule stated by a second file is caught"

root="$(new_skill_set register-states-procedure)"
expect_clean "${root}" "condition register: the fixture reads clean"
printf '\nRoutine checks have no per-leaf reviewer quota.\n' \
  >>"${root}/grove/SKILL.md"
expect_dirty "${root}" "condition register" \
  "the spine's SKILL.md stating a procedure is caught"

root="$(new_skill_set paraphrase-returns)"
expect_clean "${root}" "removed paraphrase: the fixture reads clean"
printf '\nThe glossary is the Ubiquitous Language — read every session, appended inline.\n' \
  >>"${root}/grove/references/retire.md"
expect_dirty "${root}" "glossary-is-the-forcing-function" \
  "a removed paraphrase returning is caught"

# -- Assertion 1: a bound kind that cannot reach the rule --------------------
#
# With a kind skill present, a `SKILL.md`-triggered row binds it. `SKILL.md` is
# the only file that names `references/bootstrap.md`, so severing that one
# pointer puts the rules that bootstrap.md owns off the kind's composed loaded
# path, while the file still states them. Severing a pointer that several files
# carry proves nothing, because the closure reaches the file another way.

root="$(new_skill_set unreachable)"
add_kind "${root}" planning
expect_clean "${root}" "a kind skill that reaches the spine reads clean"
drop_lines "${root}/grove/SKILL.md" 'references/bootstrap.md'
expect_dirty "${root}" "is not on grove-planning's composed loaded path" \
  "a behavioural rule no bound kind can reach is caught"

# -- Assertion 1: the spine is on a kind's path only through its load --------

root="$(new_skill_set kind-without-load)"
add_kind "${root}" planning
expect_clean "${root}" "kind without load: the fixture reads clean"
replace_text "${root}/grove-planning/SKILL.md" '**Load the `grove` skill now.**' '**Read the spine now.**'
expect_dirty "${root}" "bootstrap-order: grove/SKILL.md is not on grove-planning's composed loaded path" \
  "a kind that no longer loads the spine is caught"

# -- Assertion 3: a supporting file resolves through its skill's loads -------
#
# `grove-interview/procedure.md` names `ADR-FORMAT.md` and loads no skill
# itself. It resolves in the spine because `grove-interview/SKILL.md` loads it,
# and a session reads the supporting file only after that `SKILL.md`. No core
# kind ships a supporting file, so the kind is a fixture.

root="$(new_skill_set inherited-load)"
mkdir -p "${root}/grove-interview"
cat >"${root}/grove-interview/SKILL.md" <<'EOF'
---
name: grove-interview
description: A fixture kind of the conformance controls, with a supporting file.
---

# interview

**Load the `grove` skill now** — it is the shared spine and holds everything
this kind does not own. `grove` is the bare skill name.

When the interview starts, read `procedure.md`.
EOF
cat >"${root}/grove-interview/procedure.md" <<'EOF'
# The interview procedure

When an answer settles a decision, test it against `ADR-FORMAT.md`.
EOF
expect_clean "${root}" "a supporting file resolves through its skill's load"
replace_text "${root}/grove-interview/SKILL.md" '**Load the `grove` skill now**' '**Read the spine now**'
expect_dirty "${root}" 'missing file: grove-interview/procedure.md names `ADR-FORMAT.md`' \
  "a supporting file loses the spine with its skill's load"

# -- Assertion 2: two kinds stating one kind-owned rule ----------------------
#
# The grain control. `finish-records-teardown-before-exit` is owned by
# `grove-finish/SKILL.md`. Its wording restated in `grove-planning/SKILL.md`
# gives it two owners. Both sites are a file called `SKILL.md`, so a sweep that
# reported sites skill-relative would collapse them to one site and read clean.
# Both kinds are core kinds, because core keeps them after every move.

root="$(new_skill_set two-kinds-one-rule)"
add_kind "${root}" finish
add_kind "${root}" planning
expect_clean "${root}" "two real kind skills side by side read clean"
printf '\nRecord the teardown before sending the exit signal.\n' \
  >>"${root}/grove-planning/SKILL.md"
expect_dirty "${root}" "finish-records-teardown-before-exit" \
  "one kind-owned rule stated by a second kind is caught"

# -- The finish ending: the record precedes the signal -----------------------

root="$(new_skill_set finish-ending)"
add_kind "${root}" finish
expect_clean "${root}" "the finish ending reads clean"
drop_lines "${root}/grove-finish/SKILL.md" 'Record the teardown before sending the exit signal'
expect_dirty "${root}" "finish-records-teardown-before-exit" \
  "removing the teardown-before-exit instruction is caught"

# -- The creator step: present, and stated once -------------------------------
#
# `finishing-session-names-its-evidence` and the node-close step that carries
# it are `references/retire.md`'s. `references/decompose.md` is where a session
# cuts the review, so it is the likeliest second owner.

root="$(new_skill_set creator-step-missing)"
expect_clean "${root}" "creator step: the fixture reads clean"
drop_lines "${root}/grove/references/retire.md" 'names its provider evidence on that producer'
expect_dirty "${root}" "finishing-session-names-its-evidence" \
  "a retire procedure without the creator step is caught"

root="$(new_skill_set creator-step-missing-at-node-close)"
expect_clean "${root}" "creator step at node close: the fixture reads clean"
drop_lines "${root}/grove/references/retire.md" 'producer you finished, so take'
expect_dirty "${root}" "node-close-four-steps" \
  "a node close without the creator step is caught"

root="$(new_skill_set creator-step-restated)"
expect_clean "${root}" "creator step restated: the fixture reads clean"
printf "\nThe session that finishes a producer names its provider evidence on that producer's reviews.\n" \
  >>"${root}/grove/references/decompose.md"
expect_dirty "${root}" "finishing-session-names-its-evidence" \
  "the creator step stated by a second file is caught"

# -- Load predicates ----------------------------------------------------------

root="$(new_skill_set bad-predicate)"
rules="${scratch}/bad-predicate/rules.tsv"
printf 'invented-rule\tgrove/SKILL.md\tB\tstatic(all)\t\n' >"${rules}"
run --skills "${root}" --rules "${rules}"
assert_clean "load predicate: the fixture reads clean"
printf 'invented-rule\tgrove/SKILL.md\tB\twhenever\t\n' >"${rules}"
run --skills "${root}" --rules "${rules}"
assert_dirty 'neither static(K) nor on(t) @ F' "an unreadable load predicate is caught"
printf 'invented-rule\tgrove/SKILL.md\tB\tstatic(22)\t\n' >"${rules}"
run --skills "${root}" --rules "${rules}"
assert_dirty 'load predicate `static(22)` is neither' "the dropped kind count static(22) is unreadable"

# -- The forms of a directed load --------------------------------------------
#
# Core alone is the aggregate of core only, so a directed load of a skill that
# core does not ship fails. Each case shows that one form gives an edge, or
# that one form gives none until its mutation.

root="$(new_skill_set load-across-lines)"
expect_clean "${root}" "load across lines: the fixture reads clean"
printf '\nThen load the\n`absent-wrapped` skill.\n' >>"${root}/grove/SKILL.md"
expect_dirty "${root}" 'unresolved load: grove/SKILL.md loads `absent-wrapped`' \
  "a directed load that wraps across a line break is an edge"

root="$(new_skill_set load-with-emphasis)"
expect_clean "${root}" "load with emphasis: the fixture reads clean"
printf '\n**Load the** `absent-emphasised` _skill_.\n' >>"${root}/grove/SKILL.md"
expect_dirty "${root}" 'unresolved load: grove/SKILL.md loads `absent-emphasised`' \
  "a directed load with emphasis inside the phrase is an edge"

root="$(new_skill_set load-in-comment)"
printf '\n<!--\nLoad the `absent-commented` skill.\n-->\n' >>"${root}/grove/SKILL.md"
expect_clean "${root}" "a directed load inside an HTML comment is no edge"
drop_lines "${root}/grove/SKILL.md" '<!--'
drop_lines "${root}/grove/SKILL.md" '-->'
expect_dirty "${root}" 'unresolved load: grove/SKILL.md loads `absent-commented`' \
  "the same load outside the comment is an edge"

root="$(new_skill_set load-in-quote)"
printf '\n> Load the `absent-quoted` skill.\n' >>"${root}/grove/SKILL.md"
expect_clean "${root}" "a directed load inside a block quote is no edge"
replace_text "${root}/grove/SKILL.md" '> Load the `absent-quoted`' 'Load the `absent-quoted`'
expect_dirty "${root}" 'unresolved load: grove/SKILL.md loads `absent-quoted`' \
  "the same load outside the block quote is an edge"

# ---------------------------------------------------------------------------
# The fixture external pack
# ---------------------------------------------------------------------------
#
# Two members of one family, `grove-survey-a` and `grove-survey-b`, open with
# the exact text of the research kinds, so "Then load the `research-family`
# skill" gives an edge. `grove-survey-c` opens with the exact text of the
# combine step: it loads `grove` and names `references/review.md`, a resource
# of core. Its two fenced examples each name a skill that no pack ships.
#
# A backtick fence holds three backticks, so it flips the parity of the split
# on backticks. Inside such a fence, the name of an example falls outside the
# code span, and the parity alone hides the load. Only the tilde fence makes
# the fence rule itself load-bearing, so each fence has its own control.

# write_member <file> <kind> <last paragraph>
write_member() {
  {
    printf -- '---\nname: grove-%s\ndescription: A fixture kind of the conformance controls.\n---\n\n# %s\n\n' "$2" "$2"
    cat <<'EOF'
**Load the `grove` skill now.** Then load the `research-family` skill. These
are bare skill names. If your harness requires a plugin namespace, use the
namespace under which each skill is actually installed. Do not assume one. If
you cannot find or read a skill named here, follow the launch prompt's rule for
a missing kind skill.

`research-a` and `research-b` are one family with one procedure, and the
`research-family` skill is it: the goal, what the survey owes, what the pair
owes each other, and which output path each half writes. This skill states none
of it a second time. Everything else — the constraints, the bootstrap, and the
execute, decompose, retire and commit procedures — is the spine's.

EOF
    printf '%s\n' "$3"
  } >"$1"
}

# new_pack_fixture <name>: a core fixture and a fixture pack beside it. Prints
# the fixture directory, which holds `core/` and `pack/`.
new_pack_fixture() {
  local fixture="${scratch}/$1" pack skills
  new_skill_set "$1" >/dev/null
  pack="${fixture}/pack"
  skills="${pack}/skills"
  mkdir -p "${skills}/research-family/references" "${skills}/grove-survey-a" \
    "${skills}/grove-survey-b" "${skills}/grove-survey-c" "${pack}/conformance"

  cat >"${skills}/research-family/SKILL.md" <<'EOF'
---
name: research-family
description: The procedure of the fixture survey pair. Use when grove-survey-a or grove-survey-b directs you here.
---

# research-family

Each half of the fixture pair writes its own survey before it reads the other
half. The rest of the pair procedure is in `references/pair.md`.
EOF
  cat >"${skills}/research-family/references/pair.md" <<'EOF'
# The pair

Neither half of the fixture pair copies a finding from the other half.
EOF

  write_member "${skills}/grove-survey-a/SKILL.md" survey-a "This half writes the first survey."
  write_member "${skills}/grove-survey-b/SKILL.md" survey-b "This half writes the second survey."

  cat >"${skills}/grove-survey-c/SKILL.md" <<'EOF'
---
name: grove-survey-c
description: The fixture combine step of the conformance controls.
---

# survey-c

**Load the `grove` skill now.** It is the shared spine and holds everything
this kind does not own: the constraints, the bootstrap, and the execute,
decompose, retire and commit procedures. `grove` is the bare skill name. If your
harness requires a plugin namespace, use the namespace under which `grove` is
actually installed. Do not assume one. If you cannot find or read the `grove`
skill, follow the launch prompt's rule for a missing kind skill. What follows is
`survey-c`'s, and is stated nowhere else.

The fixture combine step reads `references/review.md` before it compares the
two surveys.

An example of the phrase, which is not an edge:

```markdown
Load the `no-such-skill` skill.
```

A second example uses a tilde fence:

~~~text
Load the `no-such-tilde-skill` skill.
~~~

The fixture combine step states each disagreement once.
EOF

  printf '%s\n' \
    '# The fixture pack'"'"'s rules.' \
    "$(printf 'fixture-family-writes-first\tresearch-family/SKILL.md\tB\tstatic({survey-a,survey-b})\tEach half of the fixture pair writes its own survey before it reads the other half')" \
    "$(printf 'fixture-pair-copies-nothing\tresearch-family/references/pair.md\tB\tstatic({survey-a,survey-b})\tNeither half of the fixture pair copies a finding from the other half')" \
    "$(printf 'fixture-combine-states-once\tgrove-survey-c/SKILL.md\tB\tstatic({survey-c})\tThe fixture combine step states each disagreement once')" \
    >"${pack}/conformance/rules.tsv"
  printf '%s\n' "${fixture}"
}

# run_alone <fixture>: the fixture pack alone.
run_alone() { run --pack "$1/pack" --alone; }

# run_aggregate <fixture> <argument>...: core's fixture with the fixture pack.
run_aggregate() {
  local fixture="$1"
  shift
  run_core "${fixture}/core" --pack "${fixture}/pack" "$@"
}

# -- The clean readings of the fixture pack ----------------------------------

fixture="$(new_pack_fixture pack-clean)"
run_aggregate "${fixture}" --verbose
assert_clean "the fixture aggregate reads clean"
assert_output 'load     grove-survey-a/SKILL.md -> research-family' \
  "the research kinds' opening text gives an edge to the family skill"
assert_output 'ref ok   grove-survey-c/SKILL.md -> grove/references/review.md' \
  "the aggregate resolves a resource of core in the spine"
run_alone "${fixture}"
assert_clean "the fixture pack alone reads clean"
assert_output 'external resource: grove-survey-c/SKILL.md names `references/review.md`, which resolves in `grove`' \
  "the pack alone lists a resource of core as an external resource"
assert_output 'external edge: grove-survey-a/SKILL.md loads `grove`' \
  "the pack alone lists the load of the spine as an external edge"

# -- Controls that must fail --------------------------------------------------

fixture="$(new_pack_fixture pack-missing-own-resource)"
run_alone "${fixture}"
assert_clean "missing pack resource: the pack alone reads clean"
run_aggregate "${fixture}"
assert_clean "missing pack resource: the aggregate reads clean"
printf '\nThe pair also keeps `references/absent.md`.\n' >>"${fixture}/pack/skills/research-family/SKILL.md"
run_alone "${fixture}"
assert_dirty 'missing file: research-family/SKILL.md names `references/absent.md`' \
  "alone, a pack resource that the pack does not ship is a missing file"
run_aggregate "${fixture}"
assert_dirty 'missing file: research-family/SKILL.md names `references/absent.md`' \
  "in the aggregate, a pack resource that the pack does not ship is a missing file"

fixture="$(new_pack_fixture pack-missing-core-resource)"
run_aggregate "${fixture}"
assert_clean "missing core resource: the aggregate reads clean"
printf '\nIt also reads `references/absent-core.md`.\n' >>"${fixture}/pack/skills/grove-survey-c/SKILL.md"
run_aggregate "${fixture}"
assert_dirty 'missing file: grove-survey-c/SKILL.md names `references/absent-core.md`' \
  "in the aggregate, a resource that core does not ship is a missing file"
run_alone "${fixture}"
assert_clean "alone, the same resource reads clean"
assert_output 'external resource: grove-survey-c/SKILL.md names `references/absent-core.md`' \
  "alone, the same resource is listed as an external resource"

fixture="$(new_pack_fixture pack-member-without-family)"
run_alone "${fixture}"
assert_clean "member without family: the pack alone reads clean"
run_aggregate "${fixture}"
assert_clean "member without family: the aggregate reads clean"
replace_text "${fixture}/pack/skills/grove-survey-a/SKILL.md" ' Then load the `research-family` skill.' ''
run_alone "${fixture}"
assert_dirty "fixture-family-writes-first: research-family/SKILL.md is not on grove-survey-a's composed loaded path" \
  "alone, a member that no longer loads its family skill is caught"
run_aggregate "${fixture}"
assert_dirty "fixture-family-writes-first: research-family/SKILL.md is not on grove-survey-a's composed loaded path" \
  "in the aggregate, a member that no longer loads its family skill is caught"

fixture="$(new_pack_fixture pack-kind-without-spine)"
run_aggregate "${fixture}"
assert_clean "pack kind without spine: the aggregate reads clean"
replace_text "${fixture}/pack/skills/grove-survey-b/SKILL.md" '**Load the `grove` skill now.** ' ''
run_aggregate "${fixture}"
assert_dirty "bootstrap-order: grove/SKILL.md is not on grove-survey-b's composed loaded path" \
  "a pack kind that no longer loads the spine puts the static(all) rows off its path"

fixture="$(new_pack_fixture pack-unknown-load)"
run_aggregate "${fixture}"
assert_clean "unknown load: the aggregate reads clean"
printf '\nLoad the `absent-skill` skill.\n' >>"${fixture}/pack/skills/grove-survey-c/SKILL.md"
run_aggregate "${fixture}"
assert_dirty 'unresolved load: grove-survey-c/SKILL.md loads `absent-skill`' \
  "in the aggregate, a load of a skill that no pack ships is unresolved"
run_alone "${fixture}"
assert_clean "alone, the same load reads clean"
assert_output 'external edge: grove-survey-c/SKILL.md loads `absent-skill`' \
  "alone, the same load is listed as an external edge"

fixture="$(new_pack_fixture pack-unfenced-example)"
run_aggregate "${fixture}"
assert_clean "a backtick-fenced example of a directed load is no edge"
drop_lines "${fixture}/pack/skills/grove-survey-c/SKILL.md" '```'
run_aggregate "${fixture}"
assert_dirty 'unresolved load: grove-survey-c/SKILL.md loads `no-such-skill`' \
  "the example without its backtick fence is an unresolved load"

fixture="$(new_pack_fixture pack-unfenced-tilde-example)"
run_aggregate "${fixture}"
assert_clean "a tilde-fenced example of a directed load is no edge"
drop_lines "${fixture}/pack/skills/grove-survey-c/SKILL.md" '~~~'
run_aggregate "${fixture}"
assert_dirty 'unresolved load: grove-survey-c/SKILL.md loads `no-such-tilde-skill`' \
  "the example without its tilde fence is an unresolved load"

fixture="$(new_pack_fixture pack-states-core-phrase)"
run_aggregate "${fixture}"
assert_clean "pack states core phrase: the aggregate reads clean"
printf '\nRoutine checks have no per-leaf reviewer quota.\n' \
  >>"${fixture}/pack/skills/grove-survey-c/SKILL.md"
run_aggregate "${fixture}"
assert_dirty "review-budget: stated in [grove-survey-c/SKILL.md,grove/references/execute.md]" \
  "a pack file that states a pinned phrase of core is a second owner"

fixture="$(new_pack_fixture core-states-pack-phrase)"
run_aggregate "${fixture}"
assert_clean "core states pack phrase: the aggregate reads clean"
printf '\nThe fixture combine step states each disagreement once.\n' \
  >>"${fixture}/core/grove/references/decompose.md"
run_aggregate "${fixture}"
assert_dirty "fixture-combine-states-once: stated in [grove-survey-c/SKILL.md,grove/references/decompose.md]" \
  "a core file that states a pinned phrase of the pack is a second owner"

fixture="$(new_pack_fixture pack-duplicate-skill)"
second="${fixture}/second"
mkdir -p "${second}/skills/other-skill" "${second}/conformance"
printf -- '---\nname: other-skill\ndescription: A second fixture pack.\n---\n\nA skill of a second pack.\n' \
  >"${second}/skills/other-skill/SKILL.md"
printf '# A second fixture pack, with no rows.\n' >"${second}/conformance/rules.tsv"
run_aggregate "${fixture}" --pack "${second}"
assert_clean "duplicate skill: the aggregate of two packs reads clean"
mv "${second}/skills/other-skill" "${second}/skills/research-family"
run_aggregate "${fixture}" --pack "${second}"
assert_dirty 'duplicate skill: `research-family` is shipped by pack' \
  "two packs that ship one skill name are caught"

fixture="$(new_pack_fixture pack-duplicate-row)"
run_aggregate "${fixture}"
assert_clean "duplicate row: the aggregate reads clean"
printf 'review-budget\tresearch-family/SKILL.md\tS\tstatic(all)\t\n' >>"${fixture}/pack/conformance/rules.tsv"
run_aggregate "${fixture}"
assert_dirty 'duplicate row: rule `review-budget` is in the manifest of core and in the manifest of pack' \
  "two manifests that hold one rule ID are caught"

fixture="$(new_pack_fixture pack-absent-owner)"
run_alone "${fixture}"
assert_clean "absent owner: the pack alone reads clean"
run_aggregate "${fixture}"
assert_clean "absent owner: the aggregate reads clean"
printf 'fixture-absent-owner\tresearch-family/references/absent.md\tS\tstatic(all)\t\n' \
  >>"${fixture}/pack/conformance/rules.tsv"
run_alone "${fixture}"
assert_dirty 'unshipped owner: fixture-absent-owner names the owner `research-family/references/absent.md`' \
  "alone, a row whose owner file is absent is caught"
run_aggregate "${fixture}"
assert_dirty 'unshipped owner: fixture-absent-owner names the owner `research-family/references/absent.md`' \
  "in the aggregate, a row whose owner file is absent is caught"

fixture="$(new_pack_fixture pack-foreign-owner)"
run_alone "${fixture}"
assert_clean "foreign owner: the pack alone reads clean"
run_aggregate "${fixture}"
assert_clean "foreign owner: the aggregate reads clean"
printf 'fixture-core-owner\tgrove/references/execute.md\tS\tstatic(all)\t\n' \
  >>"${fixture}/pack/conformance/rules.tsv"
run_alone "${fixture}"
assert_dirty 'unshipped owner: fixture-core-owner names the owner `grove/references/execute.md`' \
  "alone, a row whose owner is in another pack is caught"
run_aggregate "${fixture}"
assert_dirty 'unshipped owner: fixture-core-owner names the owner `grove/references/execute.md`' \
  "in the aggregate, a row whose owner is in another pack is caught"

# -- Usage --------------------------------------------------------------------

run --alone
assert_usage "--alone without a pack is refused"
fixture="$(new_pack_fixture usage-two-alone)"
run --pack "${fixture}/pack" --pack "${fixture}/pack" --alone
assert_usage "--alone with two packs is refused"
run --pack "${fixture}/pack" --alone --skills "${fixture}/core"
assert_usage "--alone with core's --skills is refused"
rm "${fixture}/pack/conformance/rules.tsv"
run --pack "${fixture}/pack"
assert_usage "a pack without a manifest is refused"

# -- Report ------------------------------------------------------------------

echo "controls ${passed} passed, ${failed} failed"
((failed == 0))

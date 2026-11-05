# editorial-directory-trials-k207

## Goal

Prove the boundary that this move adds, under Ravel directory delivery. An
editorial kind loads the spine and the family skill `editorial-family` before
it acts. Prove it in real Claude Code and Codex sessions, with native copies
absent.

## Context

- `docs/specs/pack-membership.md`, the row "A family skill outside research"
  in "The acceptance of each move".
- `docs/common-loading-acceptance-record.md`: "Research recipes", "The runner"
  and the case section of `requirements-family-read`.
- `scripts/harness-trials/cases/requirements-family-read/` and
  requirements-directory-trials-k193 as the model of the case and its record.
- The removal commit and the published pack commits from
  editorial-removal-k206.

## Done when

A new case runs in both harnesses with two sources. `grove` comes from the
prepared Grove checkout at the removal commit. `writing` comes from a plain
Git checkout of the published pack commit through the case's `external`
object. Each run is from a directory outside the Grove working copy, the pack
working copy and the checkout. Each run uses the actual session and delegate
read permissions.

- A mandate for one editorial kind names the bare kind skill. The agent reads
  the kind skill from the `writing` source, the spine from the `grove` source
  and `editorial-family/SKILL.md` from the `writing` source, all before it
  acts.
- The case section in the acceptance record names the Ravel build, the Grove
  commit, the pack commit, each `resourceDigest` and the observations. A gap
  is a named gap with a disposition, never a pass.
- `docs/repository-moves-record.md` links the case section.
- `bash scripts/check.sh` passes after the case and record are added.

## Notes

The membership specification asks for this one read in each harness. It asks
for no native trial, delegate case, working-copy case, refusal or offline
launch. The research move proved a family skill under a native namespace, and
the staging trials run every source on both harnesses later.

The case needs a small planted grove whose live leaf is an editorial stage. A
`copy-edit` leaf after a retired `draft` leaf, over a short document with a
prose contract in its node brief, is one choice. `grove-llm pick` on a copy of
the tree must give the planted leaf and its kind. The observation is the order
of the reads before the first action. The stage does not need to finish.

A real run spends subscription usage. Do not remove or change the owner's
native installations. The sandbox home links only each harness's sign-in
store.

This is the node's last planned leaf. Its retirement closes
editorial-pack-k179 if no live leaf remains, by the four close steps.

## Decisions (running log)

- The case is `editorial-family-read`. Its sources are `grove` from the
  prepared checkout of the removal commit `31ec9117c9da` and `writing`
  through `external`, at the second pack commit
  `ae1649d181a5aa7950404797d4735c58831d542a`, the public `main`. Both pack
  commits have the tree `361070804328` for `plugins/writing`.
- The case routes only `copy-edit`. It declares
  `skills/editorial-family/SKILL.md` of `writing` in `requires`, because the
  task relies on the family skill and the case does not name it. The
  supplier check then refuses a checkout that lacks the file.
- The planted tree holds the document node `usage-note-k1` with the retired
  `draft` leaf `usage-note-k2` and the live `copy-edit` leaf
  `usage-note-k3`. The runner plants only `.grove/`, so the note's text is in
  the node brief, beside its prose contract. The text breaks the contract
  five ways: "recognize", a semicolon, "word total", "number of words" and
  the figure "lightning fast". The leaves name no skill.
- `grove-llm pick` on a jj copy of the tree gave `02-copy-edit--usage-note-k3.md`
  and the kind `copy-edit`. The brief chain was the root brief and
  `_usage-note.md`. The verbs ran with a cleared environment, so the live
  session's admission did not apply to the copy.
- The supplier check held the 7 required paths at `31ec9117c9da`. A control
  that misspelt the `requires` path refused with status 1, and the case file
  then had its earlier SHA-256 `8438a96158fa…` again.
- The instrument was the Ravel 2.0 build of `builds.json`, installed by the
  research recipe from `71f70fcd3f92` into a scratch prefix. Its `bin/ravel`
  SHA-256 is `26b487afb957…`, and the prefix has no `libexec`. The Ravel
  workspace stayed empty. `task trial:prepare` built Grove at
  `31ec9117c9da`, with the `grove` and `grove-llm` digests of the earlier
  trials.
- Claude Code run `editorial-family-read-20261105T080814Z`: pass. Call 1 read
  the spine from the grove snapshot. Call 3 read the kind skill from the
  writing snapshot. Call 4 read `editorial-family/SKILL.md` from the writing
  snapshot.
  The first command on the task came in call 8. The runner marks call 4 with
  an error. The result holds the whole family file, 5581 bytes, and then the
  zsh error of a later `echo =====`. Only the `ls` after that echo did not
  run.
- Codex run `editorial-family-read-20261105T080952Z`: pass. Calls 2, 4 and 5
  made the same three reads, and the first command on the task came in call
  8.
- Both runs: the `writing` digest `db96fa09905e…` and the tree
  `361070804328…` equal the moves record's values. The checkout was clean at
  its commit after the driver. The driver refused `art` as `config_refused`
  by design. Each session removed the five planted breaks of the contract.
- The acceptance record holds the section "Editorial directory trials", with
  its limits. No gap is open. The moves record links it from the
  WritingSkills section. The controlled-English checker reports 0 hard
  findings in both records, and no finding in the new text. The acceptance
  record keeps its 57 advisory findings.
- Gate: `bash scripts/check.sh` passed all 21 principal checks, 1344 Rust
  tests among them, with the harness trial runner controls. The seven changed
  files outside `.grove/` had the same digests before and after the run.
- No in-session reviewer was spent. The observations come from the runner's
  per-call record and the transcripts, and no code changed.
- Close of `editorial-pack-k179`: each item of its `Done when` holds. The
  first two leaves delivered the repository, the removal, the static proof
  and the record. This leaf delivered the directory trial in each harness.
  No review names a leaf of the node, and this session has no provider
  evidence, so no Creator line is written. The parent `repository-moves-k5`
  keeps live leaves, so the cascade stops there.

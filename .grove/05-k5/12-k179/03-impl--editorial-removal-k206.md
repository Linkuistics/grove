# editorial-removal-k206

## Goal

Remove the four editorial kinds and `references/editorial.md` from Grove.
Reconcile every Grove consumer in the same change, so that Grove stays green
with WritingSkills as the owner. Then publish the second pack commit, which
records the removal commit. This is the contraction step of the move.

## Context

- `docs/specs/external-pack-contract.md`: "The family procedure", "Rule
  placement", "The kind table", "Manifest changes", "The checker without a
  copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove tests that name moving kinds"
  and "The acceptance of each move", the row "A family skill outside research".
- The WritingSkills section of `docs/repository-moves-record.md`, from
  writing-pack-repository-k205: the first pack commit, its tree ID, its digest
  and the aggregate output before the removal.
- grove-research-removal-k158 and engineering-removal-k204, with "Removal of
  the engineering pack" in the moves record, as the models. The research
  removal also moved a family file out of the spine.

Consumers at planning. Enumerate the four kind names, `editorial.md`,
`editorial-family` and `static(editorial)` again with `--hidden` over the
whole tree, with positive and cross-tree controls, before you rely on this
list:

- The spine: `grove/SKILL.md` drops `references/editorial.md` from its
  family-file list, and its description drops "editorial" from the family
  files. The editorial row of the kind table in `TASK-FORMAT.md` names
  `WritingSkills`. The label tests take those labels from the external row.
- `plugins/grove/conformance/rules.tsv`: remove the 23 rows that the moving
  files own. Eleven have the owner `grove/references/editorial.md`, and twelve
  have the owner of a moving kind's `SKILL.md`. The pack's manifest holds all
  23, as writing-pack-repository-k205 records. The header comment drops the
  family set `editorial`.
- `plugins/grove/conformance.awk`: drop the named set `editorial` and its
  comment, as "Manifest changes" states. The pack's rows already use the
  explicit list.
- `plugin_fallback.rs`: the four editorial openings leave
  `MOVING_KIND_OPENINGS`. The list is then empty, and the test drops it, as
  the membership decision states. The test's positive control iterates that
  list, so it must take another subject. For example, the enumeration must
  find each token of `NOT_SKILL_NAMES` in the file that holds it. Watch the
  new control fail against a mutated token before you rely on it.
- The `grove` plugin descriptions: `plugins/grove/.claude-plugin/plugin.json`
  and the `grove` entry of `.claude-plugin/marketplace.json` drop "editorial"
  from their family files.
- Distribution: a `git-subdir` entry for `writing` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update the
  marketplace description, which counts and names its plugins.
  `task check:packs` reads that pin.
- Documents that state where an editorial file lives. They are `README.md`,
  `plugins/grove/README.md`, `CONTEXT.md`, `plugins/README.md`,
  `plugins/CONTEXT.md`, `docs/ARCHITECTURE.md` and `docs/USAGE.md`. In
  `plugins/grove/README.md`, check the fatness rule, the family files and the
  hybrid paragraph of the editorial stages. In `CONTEXT.md`, check the entries
  "Family reference file" and "Spine skill / kind skill", and the session kind
  entries. In `docs/ARCHITECTURE.md`, check the paragraph "Core ships
  `planning` …". `docs/USAGE.md` gains a `writing` source with the root
  `skills`.
- Specifications that name `references/editorial.md` in the present tense:
  `docs/specs/pass-series.md` and the lists of "Rule placement" in
  `docs/specs/external-pack-contract.md`. Classify each by the asset rule. A
  location claim about the current state changes. An account of an earlier
  move stays.
- History and evidence stay: `CHANGELOG.md`, the decision records' accounts,
  the move specifications, the acceptance record, the trial cases,
  `docs/ravel-caller-surface-inventory.tsv` and the two inventory files.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The editorial labels, leaf names, policy routes and `grove-llm` behaviour
  are unchanged. The label tests take those labels from the external row, and
  they pass.
- Core conformance passes alone, and the runner has no named set `editorial`.
  The aggregate in `task check:packs` passes with the pinned first pack commit.
  Record the counts of core alone and of the aggregate beside the counts before
  the move.
- In a scratch copy of the pinned pack, each editorial kind in turn stops
  loading `editorial-family`. The pack alone and the aggregate then fail, and
  each failure names that member. Record the readings. This is the static
  proof that the membership specification assigns to this boundary.
- `no_skill_file_writes_a_plugin_qualified_name` has no list of retained
  openings. Its new positive control reads dirty after a mutation. Record
  both readings.
- In an isolated home, `plugins/install.sh --pack <WritingSkills checkout>/plugins/writing`
  links the four kinds and `editorial-family`. The checkout is a plain checkout
  of the published pack commit. The owner's real links are unchanged.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote through jj.
  Then publish a second pack commit whose `GROVE_COMMIT` is the removal commit.
  Its `plugins/writing` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit, the check results and the asset
  classification. Its section "For discovery-rollout-k6" adds the native step
  `/plugin install writing@linkuistics`. It also records, for rollout's text
  to the paused groves, that the editorial kinds and their family file now
  ship in WritingSkills.

## Notes

The marketplace pins the first pack commit, as the earlier moves did. Both
pack commits deliver the same bytes in `plugins/writing`. This workstream
reaches `main` only in finish, so the owner's native marketplace does not
change here. skill-marketplace-decision-k201 may later decide that this
workstream needs no marketplace. Until then, the pin is the input of
`check:packs`.

No conformance control copies an editorial kind, and no trial case launches
one. The planning session found no other named read of a moving file. The
first gate run can still find one. Then change that read in this change. Use
the membership decision's rules for a named read.

After the removal, core ships four kinds: `finish`, `planning`,
`review-planning` and `integrate-review-planning`. `shipped_kinds` requires
more than three, so the walks keep their floor.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it. Each Grove commit must still pass
`bash scripts/check.sh`, and the second pack commit records the last one.

The paused editorial grove reads the moved kinds. Do not edit its tree or
signal its sessions.

## Decisions (running log)

- The positive control of `no_skill_file_writes_a_plugin_qualified_name` now
  pairs each `NOT_SKILL_NAMES` token with one shipped file that holds it:
  `ravel:review` in `configure-grove/SKILL.md` and `path:line` in
  `grove/references/decompose.md`. The control asserts that the enumeration
  finds each token in its file.
- The two lists under "Rule placement" in `external-pack-contract.md` stay.
  They are the account of the changes of grove-research-removal-k158.
  `pass-series.md` names the `editorial-family` skill in its two present-tense
  claims. The "Out of scope" item of the contract no longer says that the
  family stays in the spine.
- The fatness rule of `plugins/grove/README.md` keeps the branch "a family that
  core ships alone", without an example. No such family exists after this
  move, but the rule still covers a future one.

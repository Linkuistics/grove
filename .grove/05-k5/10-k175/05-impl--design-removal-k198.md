# design-removal-k198

## Goal

Remove the three design kinds and the four design craft skills from Grove.
Reconcile every Grove consumer in the same change, so that Grove stays green
with DesignSkills as the owner. Then publish the second pack commit, which
records the removal commit. This is the contraction step of the move.

## Context

- `docs/specs/external-pack-contract.md`: "Rule placement", "The kind table",
  "Manifest changes", "The checker without a copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove assets of moving craft skills",
  "The Grove tests that name moving kinds", "The trial cases that name moving
  skills" and "The acceptance of each move".
- The first published pack commit, its tree ID and the aggregate output before
  the removal, from design-pack-repository-k197.
- The refusal check without a launch, from trial-supplier-refusal-k196.
- requirements-removal-k192 as the model of a removal change.

Consumers at planning. Enumerate the names of the seven moving skills and
their resource files again, with positive and cross-tree controls, before you
rely on this list:

- The spine: the rows `design`, `review-design` and `integrate-review-design`
  of the kind table in `TASK-FORMAT.md` name `DesignSkills`. The citations of
  `decision-records` and `codebase-design` in `ADR-FORMAT.md`,
  `SPEC-FORMAT.md`, `references/grove.md` and `references/review.md` stay.
  They are bare names with a local fallback.
- `plugins/grove/conformance/rules.tsv`: remove the rows whose owner is in
  `grove-design/`. The prefix sets stay.
- `plugins/grove/conformance.test.sh`: the `two-kinds-one-rule` control stops
  adding the design kind. It takes a second core kind, or a fixture.
- `crates/grove/tests/plugin_fallback.rs`: the three design openings leave
  `MOVING_KIND_OPENINGS`. The `CITATIONS` row for `grove-design/SKILL.md`
  leaves, because the pack's manifest now pins it.
- `docs/adr/a-skill-states-what-binds-without-its-dependencies.md`: its last
  paragraph states that the registry covers core and that each pack pins its
  own citations.
- `crates/grove/tests/native_skill_delivery.rs` names `grove-design` in a
  temporary tree. Check that it is a fixture label, which stays.
- `plugins/linkuistics`: remove the four craft skills and their sections of
  `PROVENANCE.md`. Its manifest description and keywords, and its marketplace
  description, then name only what it still holds.
- Distribution: a `git-subdir` entry for `design` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update
  the marketplace description, which counts and names its plugins.
- Some documents state which pack ships a kind or a craft skill. They are
  `README.md`, `plugins/grove/README.md`, `CONTEXT.md` and `CONTEXT-MAP.md`.
  They are also `plugins/CONTEXT.md` and `docs/ARCHITECTURE.md`. `plugins/README.md` gains
  a `design` section and changes its install lists and `--pack`.
  `docs/USAGE.md` gains a `design` source with both roots on the directory
  route.
  `docs/ordinal-fs-tree/CLI.md` names `cli-tool-design`. Classify it by the
  asset rule.
- History and evidence stay: `CHANGELOG.md`, the decision records' accounts,
  the move specifications, the acceptance record, the trial cases, the
  formal-methods records, `docs/ravel-caller-surface-inventory.tsv` and the two
  inventory files.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The design labels, leaf names, policy routes and `grove-llm` behaviour are
  unchanged. The label tests take the design labels from the external row,
  and they pass.
- Core conformance passes alone. The aggregate in `task check:packs` passes
  with the pinned first pack commit. In that aggregate, the core rows of
  `review-*` and `integrate-review-*` bind the pack's two family members. The
  format files that the moved kinds name resolve in the spine. Record the
  counts of core alone and of the aggregate beside the counts before the move.
- The changed `two-kinds-one-rule` control reads clean, and it reads dirty
  after its mutation.
- The refusal check of k196 refuses a case that requires `decision-records`
  at the removal commit. The same case passes the check at the commit that its
  record names. Record both readings.
- In an isolated home, `plugins/install.sh --pack <DesignSkills checkout>/plugins/design`
  links the three kinds and the four craft skills. The checkout is a plain
  checkout of the published pack commit. The owner's real links are unchanged.
- `plugins/linkuistics` no longer holds the four craft skills. Its manifest,
  its marketplace entry and its provenance file agree with what it still
  holds. `claude plugin validate plugins/linkuistics` passes.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote. Then publish a
  second pack commit whose `GROVE_COMMIT` is the removal commit. Its
  `plugins/design` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit, the check results and the asset
  classification. Its section "For discovery-rollout-k6" adds the native step
  `/plugin install design@linkuistics`.

## Notes

The marketplace pins the first pack commit, as the research and requirements
moves did. Both pack commits deliver the same bytes. This workstream reaches
`main` only in finish, so the owner's native marketplace does not change here.

`simplify-project` and `authoring-conventions` stay in `plugins/linkuistics`
for later moves. They cite moved skills. Each citation must be a bare name and
not a path, so it stays correct across packs.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it. A natural seam is the craft removal from
`plugins/linkuistics` beside the kind removal. The second pack commit then
records the later of the two Grove commits.

## Decisions (running log)

- The removal takes out 14 tracked files and nothing else: `grove-design/`
  with `SKILL.md`, `assets/diagram-viewer.html` and
  `references/visual-design-discussion.md`, `grove-review-design/`,
  `grove-integrate-review-design/`, and the four craft directories of
  `plugins/linkuistics/skills/`. They are the pack's skill files, less its new
  licence copy. The directories held no untracked file.
- Core's manifest loses the six rows whose owner is in `grove-design/`. They
  held four pinned rows and two unpinned rows. The prefix sets stay. The
  manifest's example `{design}` stays, because it shows the syntax of a list
  and `design` is still a valid label.
- Core alone before the removal, at `bbd90b1d82e4`: 115 single-source checks,
  2062 loaded-path checks and 55 unpinned rows. After it: 111, 1704 and 53.
  The aggregate with the first pack commit before the removal failed on the 16
  duplicates that design-pack-repository-k197 recorded.
- The `two-kinds-one-rule` control takes `planning` as its second kind,
  because core keeps `planning` after every move. It reads clean and then
  dirty. A mutant that planted a sentence without the rule's wording failed
  that control alone, 97 passed and 1 failed.
- `plugin_fallback.rs`: the registry loses the `grove-design/SKILL.md` row,
  and its positive control now expects eleven citations across seven files.
  The doc comment states that the registry covers core and names the pack's
  pinned row. `MOVING_KIND_OPENINGS` loses the three design openings.
- `native_skill_delivery.rs` writes `checkout/plugins/grove/skills/grove-design`
  under a temporary home. It is a fixture label, and it stays.
- The dependency decision record's last paragraph states that the registry
  covers core and that each pack pins its own citations, with the reopening
  condition of pack-membership.
- The `design` row of the kind table names `DesignSkills`. The label tests
  read the external rows through `testing/support.rs`, so they take the three
  design labels from that row with no code change.
- `plugins/linkuistics`: the manifest description drops CLI design, deep
  modules, seams, decision records and formal-model routing. The keywords drop
  `cli-design`, `software-design`, `decision-records`, `adr`,
  `formal-methods`, `specification`, `alloy`, `quint` and `model-checking`.
  The marketplace entry drops the same subjects. `PROVENANCE.md` loses the
  sections of `codebase-design`, `decision-records` and
  `model-led-development`, and one paragraph names where they went. The
  `grove` manifest keeps its keyword `decision-records`, because the ADR format
  stays in the spine.
- The marketplace adds `design` after `requirements`, with a `git-subdir`
  source that pins `9d7c7c5368f6a22b541881e6c107da28e39627bd`. Its
  description is the pack manifest's. The marketplace description counts six
  plugins.
- Documents: `README.md` (the craft paragraph, the install list, a `design`
  paragraph and `--pack`), `plugins/README.md` (a `design` section with the
  craft table, the four rows leave the linkuistics table, the install lists,
  `--pack` and the pin exception), `plugins/grove/README.md` (the fatness
  counts and the install list), `CONTEXT.md` (the family file and session
  kind entries), `plugins/CONTEXT.md`, `docs/ARCHITECTURE.md` and
  `docs/USAGE.md` (a `design` source with the roots `skills` and `craft`).
  `CONTEXT-MAP.md` names the two craft skills by bare name and makes no
  location claim, so it stays.
- Asset classification of the Grove files that name a moved craft skill:
  - Test 1, Grove's own claim, stays: `docs/ordinal-fs-tree/CLI.md` applies
    `cli-tool-design` by bare name to Grove's own CLI. The spine's
    `ADR-FORMAT.md`, `SPEC-FORMAT.md`, `references/grove.md` and
    `references/review.md`, `CONTEXT.md`, `CONTEXT-MAP.md` and the two
    remaining linkuistics skills cite by bare name.
  - Test 1, the formal-methods records stay. `docs/formalism-findings.md` had
    one relative link into `model-led-development/references/keeping-a-log.md`.
    It is now the DesignSkills public URL, which returned HTTP status 200. The
    path mentions in its dated entries, in `results-of-formal-methods-trial.md`
    and in `evidence-outlives-the-instrument.md` are accounts of their time.
  - Test 3, evidence stays: the acceptance record, the trial cases and the
    `rival-records` fixture, the two inventory files with `inventory.py`, and
    `docs/ravel-caller-surface-inventory.tsv`.
  - History and design stay: `CHANGELOG.md`, the move specifications, the
    `membership-formats.mmd` view and the example name request in
    `owner-skill-delivery.md`.
- Stale-location sweep: an enumeration of the seven names and the seven
  resource file names with `--hidden` over the whole tree, excluding `.jj`,
  `.git`, `target` and `.grove`, then a path search for each name under
  `linkuistics/skills`, `grove/skills` and `linkuistics:`. Every hit is in a
  class above. A planted path in a hidden scratch directory was found, and a
  misspelt pattern found nothing. A Markdown-link search found only the
  formal-methods link, and its control found links to present skills.
- The aggregate with the pinned first pack commit `9d7c7c5368f6`: 116
  single-source checks, 2063 loaded-path checks and 55 unpinned rows. That is
  core alone before the move plus the pinned citation row, which moved from
  the Rust registry into the pack's manifest. The pack alone: 5, 7 and 2, with
  3 external edges and 2 external resources. The requirements aggregate is now
  112 and 2066, and the research aggregate is 111 and 2063.
- Binding proof: in a scratch copy of the pinned pack, `grove-review-design`
  and `grove-integrate-review-design` stopped loading `grove`. The aggregate
  then failed 25 assertions, and each names one of the two members: the three
  `static(review-*)` rows, the four `static(integrate-review-*)` rows, the
  spine rows and the two family files as missing files.
- Installer proof: in an isolated home with Codex, Gemini CLI and Pi
  directories, `install.sh --force --pack <plain checkout of 9d7c7c5368f6>/plugins/design`
  linked the three kinds from `skills/` and the four craft skills from
  `craft/` into each harness, 111 links in total. `--force` is needed because
  this workspace is a secondary jj workspace. The owner's real skill links had
  the same digest before and after.
- `claude plugin validate plugins/linkuistics` (Claude Code 2.1.318) passed
  with two warnings, no `version` and no icon. The marketplace validation
  passed with the two `version` warnings of the relative plugins.
- Gate: `bash scripts/check.sh` passed all 21 principal checks, 1345 Rust
  tests among them. `check:packs` passed `requirements`, `design` and
  `research` alone and in the aggregate, and `testanyware` passed the
  directory-only check. `GROVE_TEST_RAVEL` resolved to
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, Ravel 1.1.0, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  changed files outside `.grove/` had the same digests before and after the
  run.
- `.claude/settings.json` is the owner's local permission. It stays in the
  working copy and in no commit.

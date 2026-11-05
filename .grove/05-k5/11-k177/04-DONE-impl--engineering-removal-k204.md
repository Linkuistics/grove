# engineering-removal-k204

## Goal

Remove the six engineering kinds and the nine engineering craft skills from
Grove. Reconcile every Grove consumer in the same change, so that Grove stays
green with EngineeringSkills as the owner. Then publish the second pack
commit, which records the removal commit. This is the contraction step of the
move.

## Context

- `docs/specs/external-pack-contract.md`: "Rule placement", "The kind table",
  "Manifest changes", "The checker without a copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove assets of moving craft skills",
  "The Grove tests that name moving kinds", "The trial cases that name moving
  skills" and "The acceptance of each move".
- The EngineeringSkills section of `docs/repository-moves-record.md`. It holds
  the consumer classification and the scratch-removal failures of
  impl-consumer-retarget-k202. It also holds the first pack commit, its tree
  ID and the aggregate output before the removal, from
  engineering-pack-repository-k203.
- design-removal-k198 and "Removal of the design pack" in the moves record as
  the model of a removal change.

Consumers at planning. Enumerate the fifteen skill names and their resource
file names again, with positive and cross-tree controls, before you rely on
this list:

- The spine: the rows `impl` and `prototype` of the kind table in
  `TASK-FORMAT.md` name `EngineeringSkills`. The label tests take those
  labels from the external rows.
- `plugins/grove/conformance/rules.tsv`: remove the four rows whose owner is
  a moving kind. The prefix sets stay.
- `plugin_fallback.rs`: the six engineering openings leave
  `MOVING_KIND_OPENINGS`. The four editorial openings stay.
- The doubt assertions in `composition_guidance.rs` and
  `session_kind_guidance.rs`. Each test drops its read of
  `doubt-driven-development/SKILL.md` and keeps its assertions over Grove's
  own files. The pack's doubt check of k203 then owns the skill's half.
- `plugins/linkuistics`: remove the nine craft skills and their sections of
  `PROVENANCE.md`. Its manifest description and keywords, and its
  marketplace entry, then name only what it still holds.
- Distribution: a `git-subdir` entry for `engineering` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update
  the marketplace description, which counts and names its plugins.
  `task check:packs` reads that pin.
- Documents that state which pack ships a moving kind or craft skill:
  `README.md`, `plugins/grove/README.md`, `plugins/README.md`, `CONTEXT.md`,
  `CONTEXT-MAP.md`, `plugins/CONTEXT.md`, `docs/ARCHITECTURE.md` and
  `docs/USAGE.md`. `plugins/README.md` gains an `engineering` section with
  the craft table. `docs/USAGE.md` gains an `engineering` source with both
  roots on the directory route.
- The two walkthrough chapters of `docs/walkthroughs/grove-loop/` that name
  `grove-impl`. Classify them by the asset rule. A chapter that states where
  core ships the kind is a stale location claim.
- History and evidence stay: `CHANGELOG.md`, the decision records' accounts,
  the move specifications, the acceptance record, the trial cases, the
  formal-methods records, `docs/ravel-caller-surface-inventory.tsv` and the two
  inventory files.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The engineering labels, leaf names, policy routes and `grove-llm` behaviour
  are unchanged. The label tests take those labels from the external row, and
  they pass.
- Core conformance passes alone. The aggregate in `task check:packs` passes
  with the pinned first pack commit. In that aggregate, the core rows of
  `review-*` and `integrate-review-*` bind the pack's four family members.
  Record the counts of core alone and of the aggregate beside the counts
  before the move.
- In a scratch copy of the pinned pack, the four family members stop loading
  `grove`. The aggregate then fails, and each failure names a member. Record
  the reading.
- Grove's suite no longer reads `doubt-driven-development/SKILL.md`. Each
  changed doubt test keeps its assertions over Grove's files. The pack's doubt
  check passes against the removal commit.
- The refusal check of trial-supplier-refusal-k196 refuses a case that
  launches `impl` at the removal commit. The same case passes the check at
  the commit that its record names. Record both readings.
- In an isolated home, `plugins/install.sh --pack <EngineeringSkills checkout>/plugins/engineering`
  links the six kinds and the nine craft skills. The checkout is a plain
  checkout of the published pack commit. The owner's real links are
  unchanged.
- `plugins/linkuistics` no longer holds the nine craft skills. Its manifest,
  its marketplace entry and its provenance file agree with what it still
  holds. `claude plugin validate plugins/linkuistics` passes.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote. Then publish
  a second pack commit whose `GROVE_COMMIT` is the removal commit. Its
  `plugins/engineering` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit, the check results and the asset
  classification. Its section "For discovery-rollout-k6" adds the native step
  `/plugin install engineering@linkuistics`.

## Notes

The marketplace pins the first pack commit, as the earlier moves did. Both
pack commits deliver the same bytes in `plugins/engineering`. This workstream
reaches `main` only in finish, so the owner's native marketplace does not
change here. skill-marketplace-decision-k201 may later decide that this
workstream needs no marketplace. Until then, the pin is the input of
`check:packs`.

The trial cases that launch `impl` stay unchanged as records of their builds.
After this removal, most of them are refused at the Grove line's head. That
refusal is the designed reading, not a regression.

`simplify-project` leaves `plugins/linkuistics` here.
`authoring-conventions` and the writing craft stay for later moves. Each
citation of a moved skill in them must be a bare name and not a path.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it. A natural seam is the craft removal from
`plugins/linkuistics`, with the doubt assertions, beside the kind removal. The
second pack commit then records the later of the two Grove commits.

This is the node's last planned leaf. The base proofs need no trial, as the
membership decision states. Its retirement closes engineering-pack-k177 if no
live leaf remains, by the four close steps.

## Decisions (running log)

- Core alone before the removal, at `55dae6392644`: 111 single-source checks,
  1704 loaded-path checks and 53 unpinned rows. The aggregate with the first
  pack commit before the removal fails on the 14 duplicates that
  engineering-pack-repository-k203 recorded.
- The removal takes out 19 tracked files and nothing else. They are the six
  kind directories of `plugins/grove/skills/` with 7 files, and the nine craft
  directories of `plugins/linkuistics/skills/` with 12 files. The directories
  hold no untracked file. The only untracked file under either skill tree is
  an ignored `__pycache__` of `technical-communication`, which stays.
- `.claude/settings.json` is the owner's local permission. It stays in the
  working copy and in no commit, as in design-removal-k198.
- Core's manifest loses the four rows whose owner is `grove-impl/SKILL.md` or
  `grove-prototype/SKILL.md`. The prefix sets stay. No other conformance file
  names a moving owner.
- The rows `prototype` and `impl` of the kind table name `EngineeringSkills`.
  `MOVING_KIND_OPENINGS` loses the six engineering openings and keeps the four
  editorial openings.
- The doubt tests drop each read of `doubt-driven-development/SKILL.md`. Five
  tests keep their assertions over Grove's files. The test
  `doubt_guidance_yields_to_grove_without_changing_standalone_doubt` read only
  the skill, so it goes. Its Grove half is in
  `grove_guidance_replaces_the_old_in_session_review_loop`, which already held
  two of the three Grove phrases of the pack's assertion
  `yields_to_grove_review_allowance`. That test gains the third phrase,
  "outside that predicate, doubt keeps its standalone bounded cycles
  unchanged". The comments name the pack's doubt check as the owner of the
  skill's half.
- `plugins/linkuistics`: the manifest description drops the style guides,
  project simplification, adversarial verification and codebase-graph queries,
  and it calls the suite "practice skills". The keywords drop
  `coding-standards`, `style-guide`, the six language names,
  `project-simplification`, `repository-maintenance`, `code-review`,
  `codebase-memory`, `knowledge-graph` and `code-search`. The marketplace
  entry drops the same subjects. `PROVENANCE.md` loses the section of
  `doubt-driven-development`, and the paragraph of moved sections names
  EngineeringSkills. The `addyosmani-S*` row of the source-key table stays,
  because `authoring-conventions` uses those keys.
- The marketplace adds `engineering` after `design`, with a `git-subdir`
  source that pins `d98862f217492a44932c8fa93a006367da252a5d`. Its
  description is the pack manifest's. The marketplace description counts
  seven plugins.
- Documents: `README.md` adds `engineering@linkuistics` to the install list,
  a paragraph that names the craft skills, and `--pack`. `plugins/README.md`
  adds an `engineering` section with the craft table, the language-coverage
  paragraph and the `paths:` sentences. Those rows and sentences leave the
  linkuistics section, which is now "practice skills". It changes the install
  lists, `--pack` and the pin exception. `plugins/grove/README.md` changes
  the family members, the fatness counts (two fat, two thin) and the install
  list. Its licence-file paragraph states that no core kind now carries a file
  beside `SKILL.md`. `CONTEXT.md` changes the family file and session kind
  entries. `plugins/CONTEXT.md` changes the plugin entry, the `paths:` entry
  and the personal-setup entry. `docs/ARCHITECTURE.md` changes the external
  packs row and the kinds paragraph, which no longer says core ships most
  kinds. `docs/USAGE.md` adds an `engineering` source with the roots `skills`
  and `craft`.
- Two claims were already wrong before this leaf, and the change repairs
  both. `plugins/README.md` and `plugins/CONTEXT.md` said that only
  `doubt-driven-development` carries `assumes-personal-setup`, but
  `machine-environment` also carries it. Each now names `machine-environment`
  here and `doubt-driven-development` in EngineeringSkills. The architecture
  pointer to the formal-methods results named "the `linkuistics` skills". It
  now says "the craft skills", because those skills left `linkuistics`.
- These stay. `CONTEXT-MAP.md` names the doubt skill by role, with no
  location. `CONTEXT.md` keeps its dated account of the campaign's lessons.
  Chapter 3 of the walkthrough derives `grove-impl` from `impl`, and chapter
  18 shows the composed prompt. The `grove` manifest names no moved subject.
- Core alone after the removal: 109 single-source checks, 996 loaded-path
  checks and 51 unpinned rows. The control suite gave 98 passed.
- `task check:packs` passed `requirements`, `design`, `engineering` and
  `research` alone and in the aggregate, and `testanyware` passed the
  directory-only check. The engineering pack alone: 2, 4 and 2, with 6 external
  edges and 5 external resources. The aggregate of core and the first pack
  commit: 111, 1704 and 53, the figures of core alone before the removal. The
  other aggregates are now requirements 110 and 1358, design 114 and 1355, and
  research 109 and 1355.
- Binding proof: in a scratch copy of the pinned pack, the four family members
  wrote "Note the `grove` skill now" in place of the directed load. The
  aggregate then failed 50 assertions, and each names one of the four members.
  They are the 4 family files as missing files and the 7 spine rows of
  `grove/SKILL.md` for each member (28). The others are the 3
  `static(review-*)` rows for each review member (6) and the 4
  `static(integrate-review-*)` rows for each integration member (8). The two kinds that kept their load,
  `grove-impl` and `grove-prototype`, appear in no failure.
- Stale-location sweep. An enumeration of the fifteen names and the four
  resource file names with `--hidden` over the whole tree excluded `.jj`,
  `.git`, `target` and `.grove`. Searches for each name under
  `linkuistics/skills/`, `grove/skills/`, `linkuistics:` and any
  `skills/` or `craft/` segment followed. Two hits were stale and changed. The
  example dialogue of `plugins/CONTEXT.md` treated
  `linkuistics:coding-style-rust` as a valid reference, and it now uses
  `linkuistics:using-jujutsu`. The licence rule of
  `external-pack-contract.md` said "as `grove-impl` does in core", and it now
  says EngineeringSkills. Every other hit is in one class. The evidence is the
  acceptance record, the trial cases and the dual-Ravel record. The baseline
  is the two inventory files and `inventory.py`. The other classes are fixture
  labels in a temporary tree, history, and this workstream's record and design.
  The bare-name citations are in `authoring-conventions`, `guardrail` and
  `evidence-outlives-the-instrument.md`.
  A planted path in a hidden scratch directory was found, and a misspelt name
  found nothing. The first enumeration used `rg -E`, which is `--encoding`, and
  printed nothing. The rerun used `-e`.
- An earlier gate run stopped when the sweep found the two stale claims, so
  that no file changed under a running measurement.
- Installer proof: an isolated home held Codex, Gemini CLI and Pi
  directories. A plain clone of the public remote, at
  `d98862f217492a44932c8fa93a006367da252a5d`, was clean. There
  `install.sh --force --pack <clone>/plugins/engineering` linked the six kinds
  from `skills/` and the nine craft skills from `craft/` into each harness.
  The run made 90 links in total and skipped `guardrail` three times.
  The owner's real skill links had the same digest before and after,
  `40716ee1b429…` over 165 entries.
- `claude plugin validate plugins/linkuistics` (Claude Code 2.1.318) passed
  with two warnings, no `version` and no icon. The marketplace validation
  passed with the two `version` warnings of the relative plugins.
- The first full gate failed `cargo clippy` alone, with 20 of 21 checks
  passing. Dropping the doubt tuple left a loop over one element in
  `grove_and_doubt_state_one_mandate_based_ownership_predicate` and in
  `every_session_kind_is_named_by_the_provisioned_taxonomy`. Each loop is now
  direct statements over the one Grove surface, with the same assertions.
- Gate: `bash scripts/check.sh` passed all 21 principal checks, 1344 Rust
  tests among them. That is one fewer than the 1345 of earlier leaves, because
  the skill-only doubt test went. `GROVE_TEST_RAVEL` resolved to
  `/opt/homebrew/bin/ravel`, which is
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, Ravel 1.1.0, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  changed files outside `.grove/` had the same digests before and after the
  run.
- The removal commit is `f45c5a53796f2a5bf634fec340cb0d7aef63e184`. The leaf
  is retired in a later commit, because the second pack commit records the
  removal commit and this leaf records that pack commit.
- `jj git push --remote origin --bookmark gh-issue-25` moved the remote
  bookmark forward from `d65914d57015` to `f45c5a53796f`. `git ls-remote`
  shows it there.
- The supplier check refused `discovery`, an `impl` case, at the removal
  commit with status 1. It names `coding-style-bash` and the kind skill
  `grove-impl` as missing. At `3bfb096810db`, the commit that its record
  names, it held the case's 8 paths with status 0. The acceptance record's
  section "Readings at the removal" adds this reading.
- The second pack commit is `549607bbb2e13baedca02d8c2481d6ad65541a26`, a
  child of `d98862f21749`. It sets `GROVE_COMMIT` to `f45c5a53796f`. Its
  README replaces the paragraph that said the aggregate fails, and it adds one
  provenance sentence. The install paragraph about the coming Grove release
  stays, because it holds until finish. The tree of `plugins/engineering`
  stays `d8a891cd5c13e9866f7937537afc7979fb3ef0f7`.
- From a clean plain-Git checkout of `549607bbb2e1`, with the committed public
  `GROVE_REPOSITORY` and no override, `task check` passed. Grove came from
  GitHub. The doubt check gave 7 and 15, the pack alone 2 and 4, and the
  aggregate 111 and 1704.
- `jj git push --remote origin --bookmark main` in EngineeringSkills moved the
  public `main` forward from `d98862f21749` to `549607bbb2e1`. A clone of the
  public remote was at that commit. The Ravel 2.0 build resolved it with the
  digest `7e70b280e7a9…` of the first commit, and the snapshot equals
  `plugins/engineering`. That build prints `1.1.0` for `--version`, so its
  SHA-256 and build identity are the pin.
- `docs/repository-moves-record.md` records the removal in the sections
  "Removal of the engineering pack", "Readings of the removal of the
  engineering pack", "Asset classification of the engineering pack" and
  "Checks of the removal of the engineering pack". The native step
  `/plugin install engineering@linkuistics` is in the section "For
  discovery-rollout-k6".
- No in-session reviewer was spent. Executable controls with seen failures
  cover the family binding, the supplier refusal and the sweep. The moved
  bytes are unchanged, and the pack's doubt check owns the skill's half of
  each removed doubt assertion.
- Close of `engineering-pack-k177`: each item of its Done when holds, so the
  node closes. No review names this leaf or the node, and the environment has
  no `GROVE_PROVIDER_EVIDENCE`, so no Creator line is written. Three results
  go to the parent brief. They are the doubt check as the model for a
  composing craft skill, the `assumes-personal-setup` claims for the personal
  move, and the bare-name citations in `authoring-conventions` and
  `guardrail`. The ADR set needs no change, because no decision changed and
  the records cite the moved skills by bare name.

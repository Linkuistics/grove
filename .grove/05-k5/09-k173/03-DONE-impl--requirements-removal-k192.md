# requirements-removal-k192

## Goal

Remove the three requirements kinds from Grove and reconcile every Grove
consumer in the same change. Grove then stays green with RequirementsSkills as
the owner. Then publish the second pack commit, which records the removal
commit. This is the contraction step of the move.

## Context

- `docs/specs/external-pack-contract.md`: "Rule placement", "The kind table",
  "Manifest changes", "The checker without a copy" and "Distribution".
- `docs/specs/pack-membership.md`: "The Grove tests that name moving kinds"
  and "The acceptance of each move".
- The first published pack commit, its tree ID and the aggregate output before
  the removal, from requirements-pack-repository-k191.
- grove-research-removal-k158 as the model of a removal change.

Consumers at planning. Search again with positive and cross-tree controls
before you rely on the list:

- The spine: the `requirements` row of the kind table in `TASK-FORMAT.md`
  names `RequirementsSkills`. The sentence that Grove spells `requirements`
  for root scaffolding stays, because the label is unchanged.
- `plugins/grove/conformance/rules.tsv`: remove the rows that moved. The
  prefix sets `review-*` and `integrate-review-*` stay, and core keeps their
  rows.
- `plugins/grove/conformance.test.sh`: the `inherited-load` control reads
  `grove-requirements/grilling.md`. It takes a fixture kind with a supporting
  file, because no core kind keeps one.
- `crates/grove/tests/plugin_fallback.rs`: the three requirements openings
  leave `MOVING_KIND_OPENINGS`. Check the doc comment that names
  `grilling.md`.
- Distribution: a `git-subdir` entry for `requirements` in
  `.claude-plugin/marketplace.json` that pins the first pack commit. Update
  the marketplace description, which counts and names its plugins. Check
  `plugins/README.md` and the installer guidance for `--pack`.
- Documents: `plugins/grove/README.md` (the inventory and the kinds' own
  resources), `CONTEXT.md`, `plugins/grove/skills/grove/CONTEXT-FORMAT.md`,
  `docs/USAGE.md` (its link to `grilling.md` becomes a public URL),
  `docs/ARCHITECTURE.md` and `plugins/CONTEXT.md`. Each history record, such
  as `CHANGELOG.md` and the decision record's account of the old
  `references/requirements.md`, stays as history.
- `docs/evaluations/owner-skill-delivery/inventory.py` and
  `docs/owner-skill-inventory.tsv` are the baseline evidence. They stay.

## Done when

- No moving file remains in Grove. A search with positive and cross-tree
  controls shows no stale location claim.
- The three requirements labels and root scaffolding are unchanged. Leaf
  names, policy routes and `grove-llm` behaviour are also unchanged. The label tests take
  the requirements labels from the external row, and they pass.
- Core conformance passes alone. The aggregate in `task check:packs` passes
  with the pinned first pack commit. In that aggregate, the core rows of
  `review-*` and `integrate-review-*` bind the pack's two members. The format
  files that the moved text names resolve in the spine. Record the
  counts of core alone and of the aggregate beside the counts before the move.
- The changed `inherited-load` control reads clean on its fixture and dirty
  after its mutation.
- A documentation sentence that tells a user what a fresh grove's first leaf
  needs names RequirementsSkills, where such a sentence exists.
- `bash scripts/check.sh` passes on the Grove line. Record the removal commit
  and the Ravel build.
- Push the line to the bookmark `gh-issue-25` on Grove's remote. Then publish a
  second pack commit whose `GROVE_COMMIT` is the removal commit. Its
  `plugins/requirements` tree ID equals the first pack commit's. Its full
  `task check` passes from a clean checkout, with the committed public URL and
  no override.
- `docs/repository-moves-record.md` records the removal commit, both pack
  commits, the pushed Grove commit and the check results.

## Notes

The marketplace pins the first pack commit, as the research move did. Both
pack commits deliver the same bytes. This workstream reaches `main` only in
finish, so the owner's native marketplace does not change here. A native user
needs `/plugin install requirements@linkuistics` after finish. Add that step
to the moves record for discovery-rollout-k6.

Keep the documentation in the change that it describes. If the change is too
large for one session, decompose it.

## Decisions (running log)

- The removal takes out `grove-requirements/` with `SKILL.md` and
  `grilling.md`, `grove-review-requirements/` and
  `grove-integrate-review-requirements/`. It removes the ten manifest rows
  whose owner is in `grove-requirements/`. They are the ten rows of the pack's
  manifest. The manifest's example of a trigger now names
  `grove-finish/SKILL.md`. The prefix sets stay.
- Core alone before the removal, at `53d092fa0e5e`: 116 single-source checks,
  2424 loaded-path checks, 64 unpinned rows. After it: 115, 2062 and 55. The
  ten rows held one pinned row and nine unpinned rows.
- The aggregate with the pinned first pack commit `9f7278c2d46b`: 116 and
  2424, the core-alone figures before the move. The pack alone: 1
  single-source check, 10 loaded-path checks, 3 external edges and 6 external
  resources. The research aggregate is now 115 and 2421, which is 362 fewer
  loaded-path checks than before, the same drop as core alone.
- Binding proof: in a scratch copy of the pinned pack, the two family members
  stopped loading `grove`. The aggregate then failed all three
  `static(review-*)` rows for `review-requirements` and all four
  `static(integrate-review-*)` rows for `integrate-review-requirements`, and
  no other kind. Removing only the member's pointer to its family file left the
  run clean, because the spine's `SKILL.md` lists its family files. That
  mutation therefore is not a control.
- The `inherited-load` control now writes the fixture kind `grove-interview`
  with the supporting file `procedure.md`, which names `ADR-FORMAT.md`. Its
  first dirty reading came back clean, because the fixture's load sentence put
  the full stop inside the bold and the mutation matched nothing. With the
  moved skill's form, it reads clean and then dirty with the exact diagnostic.
- `MOVING_KIND_OPENINGS` loses the three requirements openings. The citation
  audit's comment keeps its history and adds that `grilling.md` moved out of
  the corpus.
- The `requirements` row of the kind table names `RequirementsSkills`.
- The marketplace adds `requirements` with a `git-subdir` source that pins
  `9f7278c2d46b3e7443355633afb8a745b37af7ac`. Its description is the pack
  manifest's. The marketplace description counts five plugins.
- The documents name RequirementsSkills where they state which pack ships a
  kind: `README.md`, `plugins/README.md` (a new `requirements` section, the
  install lists, `--pack` and the pin exception), `plugins/grove/README.md`,
  `CONTEXT.md`, `plugins/CONTEXT.md` and `docs/ARCHITECTURE.md`.
  `plugins/grove/README.md` also gained the missing `research` install line.
  `docs/USAGE.md` adds a `requirements` source to the directory route. It
  states after the start-up checks that a fresh grove's first leaf needs
  `grove-requirements` from RequirementsSkills. Its `grilling.md` link is now
  a public URL. The provenance comment of `CONTEXT-FORMAT.md` adds where
  `grilling.md` ships now. The frozen measurement table, `CHANGELOG.md`, the
  decision record, the two inventory files and the move specifications stay.
- Stale-location sweep: an enumeration of `grove-requirements`,
  `grove-review-requirements`, `grove-integrate-review-requirements` and
  `grilling.md` over the whole tree with `--hidden`, excluding `.jj`, `.git`,
  `target` and `.grove`. Each hit is new correct text, history, a move
  specification, the record or the inventory. A planted path in a hidden
  scratch directory was found, the specifications still held the class, and
  a mutated pattern found nothing.
- The controlled-English checker reported two hard findings in new text, a
  seven-sentence paragraph and "start" beside "launch". Both are corrected,
  and the changed documents report no hard finding.
- Gate: `bash scripts/check.sh` passed all 21 principal checks, 1345 Rust
  tests among them. `GROVE_TEST_RAVEL` resolved to
  `/opt/homebrew/Cellar/ravel/1.1.0/bin/ravel`, Ravel 1.1.0, SHA-256
  `c52f990b2059c8bbe8efba10a741d3a0311000ac1712d71e882638f54c3ac1ed`. The
  13 changed files outside `.grove/` had the same digests before and after the
  run.
- `.claude/settings.json` is the owner's local permission for the earlier
  publication. It stays in the working copy and in no commit.
- The removal commit is `302f85429e4b8f8ada02be594687155109dfbf6b`. The leaf
  is retired in a later commit, because the second pack commit records the
  removal commit and this leaf records that pack commit.
- `jj git push --remote origin --bookmark gh-issue-25` moved the remote
  bookmark forward from `6cdab3a98833` to `302f85429e4b`. `git ls-remote`
  shows it there.
- The second pack commit is `eb945d9b13efcf11a74c78936c802108d3f022c6`, a
  child of `9f7278c2d46b`. It sets `GROVE_COMMIT` to `302f85429e4b`. Its README
  replaces the paragraph that said the aggregate fails, and it adds one
  provenance sentence. The tree of `plugins/requirements` stays
  `f3cf8128fd206c48d73b061cc5ef6b536d4f9d61`.
- From a clean plain-Git checkout of `eb945d9b13ef`, with the committed public
  `GROVE_REPOSITORY` and no override, `task check` passed. Grove came from
  GitHub. `check:plugin` passed with its two warnings. The pack alone gave 1
  and 10, and the aggregate gave 116 and 2424.
- `jj git push --remote origin --bookmark main` in RequirementsSkills moved the
  public `main` forward from `9f7278c2d46b` to `eb945d9b13ef`. A clone of the
  public remote was at that commit, with the same tree ID, and its
  `task check:conformance` gave the same figures.
- `docs/repository-moves-record.md` records the removal in the sections
  "Removal", "Readings of the removal" and "Checks of the removal". The native
  install step `/plugin install requirements@linkuistics` is in its section
  "For discovery-rollout-k6".
- No in-session reviewer was spent. Executable controls with seen failures
  cover the changed control and the family binding, and the moved bytes are
  unchanged.

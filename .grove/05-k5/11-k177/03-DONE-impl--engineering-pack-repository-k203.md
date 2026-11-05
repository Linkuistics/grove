# engineering-pack-repository-k203

## Goal

Create and publish the EngineeringSkills repository. The six engineering kinds
go in `skills/`, and the nine engineering craft skills go in `craft/`. The pack
also takes the check of the doubt skill's composition with Grove. This is the
expand step of the move. Grove keeps its own copies until
engineering-removal-k204, so no Grove consumer changes here.

## Context

- `docs/specs/external-pack-contract.md`: "Layout", "References between
  skills", "Conformance", "Provenance and licences", "Distribution",
  "Checkout boundary", "The checker without a copy" and "Proven facts".
- `docs/specs/pack-membership.md`: "The Grove assets of moving craft skills"
  (the doubt assertions and the provenance file) and "The acceptance of each
  move". This move owes the base proofs only.
- DesignSkills at `~/Development/AgenticOrchestration/DesignSkills` and
  design-pack-repository-k197 as the model. Copy its `Taskfile.yml`,
  `.gitignore`, `LICENSE` and README shape, and change the source directory.
- `docs/specs/owner-skill-delivery.md`: the source `engineering` is
  EngineeringSkills, with the roots `skills` and `craft` in its plugin
  directory. The plugin directory is therefore `plugins/engineering`, and the
  native namespace is `engineering:`.
- The baseline of owner-ravel-skill-delivery-k37: `docs/owner-skill-inventory.tsv`
  and `docs/evaluations/owner-skill-delivery/inventory.py`.
- The EngineeringSkills section of `docs/repository-moves-record.md`, from
  impl-consumer-retarget-k202.
- `linkuistics:machine-environment` for the Taskfile and mise conventions of a
  new repository.

The moving files at planning. Search again before you rely on the list.

- `plugins/grove/skills/grove-impl/`: `SKILL.md` and
  `LICENSES/addyosmani-agent-skills.LICENSE`.
- `plugins/grove/skills/`: `grove-prototype`, `grove-review-impl`,
  `grove-integrate-review-impl`, `grove-review-prototype` and
  `grove-integrate-review-prototype`, each a `SKILL.md` alone.
- `plugins/linkuistics/skills/`: the six `coding-style-*` skills and
  `simplify-project`, each a `SKILL.md` alone.
- `plugins/linkuistics/skills/using-codebase-memory/`: `SKILL.md` and
  `references/reading-the-graph.md`.
- `plugins/linkuistics/skills/doubt-driven-development/`: `SKILL.md` and two
  files in `references/`.
- The four rows of `plugins/grove/conformance/rules.tsv` whose owner is a
  moving kind: three rows of `grove-impl/SKILL.md` and one of
  `grove-prototype/SKILL.md`.
- The doubt assertions: seven tests in `composition_guidance.rs` and one in
  `session_kind_guidance.rs` read `doubt-driven-development/SKILL.md`.

## Done when

- `~/Development/AgenticOrchestration/EngineeringSkills` exists as a colocated
  Git and Jujutsu repository, with the public remote
  `https://github.com/Linkuistics/EngineeringSkills`. Its source directory is
  `plugins/engineering`, and its Ravel roots are `["skills", "craft"]`.
- `plugins/engineering/skills/` holds the six kinds. `grove-impl` keeps its
  `LICENSES/` file inside its own skill.
- `plugins/engineering/craft/` holds the nine craft skills with their
  resources. Each keeps its bytes, or the record names the change.
- `plugins/engineering/PROVENANCE.md` holds the sections of
  `plugins/linkuistics/PROVENANCE.md` for the moving craft skills. Each
  relative link of a moved craft skill to `PROVENANCE.md` resolves inside the
  source directory. Grove keeps its copy of those sections until
  engineering-removal-k204.
- The six kinds open with the contract's bare-name text. The four family
  members keep their read of `references/review.md` or
  `references/integrate-review.md` after the load sentence.
- A citation of another skill in a moved file is a bare name, and it stays a
  citation. A mention of a Grove record that stays in Grove is a public URL.
- A moved skill that adapts a third-party passage holds that licence text in
  its own `LICENSES/`. Check `doubt-driven-development` and
  `simplify-project`. A skill without such a passage gets no `LICENSES/`.
- The record names each byte change against Grove's files and against each
  moved skill's baseline entry, as proof 5 of the membership specification
  requires.
- `plugins/engineering/.claude-plugin/plugin.json` sets the name
  `engineering`, the licence `Apache-2.0`, the dependencies `["grove"]` and
  `"skills": ["./craft/"]`, with no `version`. `claude plugin validate`
  accepts it, and the record names its warnings.
- `plugins/engineering/conformance/rules.tsv` holds the four rows that the
  moving kinds own, with their owners and loads unchanged.
- The pack has a check of the doubt skill's composition with Grove. It runs
  in the pack's `check` task. It asserts the skill's half of each of the
  eight doubt assertions. It reads Grove's matching phrases from the
  `GROVE_COMMIT` checkout that `grove:checkout` makes. Each assertion reads
  dirty against a mutated copy of the skill and against a mutated copy of
  Grove's phrase. Record those readings.
- The repository root holds `LICENSE`, `.gitignore` with `.cache/`, a
  Taskfile and a README. The README names the source directory, both roots,
  the kinds, the craft skills, the Ravel record, the install commands, the
  provenance and the tool requirements. It states that conformance covers
  only `skills/`. It states that `check:plugin` and the doubt check cover the
  craft skills.
- The Taskfile has the tasks and variables of DesignSkills, and the doubt
  check. `GROVE_COMMIT` names a Grove commit that still ships the moving files
  and that includes impl-consumer-retarget-k202. `GROVE_REPOSITORY` keeps the
  public URL.
- Before the pack's first publication, push this workstream's line to the
  bookmark `gh-issue-25` on Grove's remote through jj. The recorded Grove
  commit is an ancestor of the pushed bookmark.
- The pack-alone check, `check:plugin` and the doubt check pass. They run
  from a clean checkout of the first pack commit, with the committed public
  URL and no override. The pack-alone output lists the family files as external
  resources and the loads of `grove` as external edges. The aggregate fails
  only because each moving kind and each moved row is a duplicate. Record
  the outputs.
- `ravel skills resolve --json`, from the Ravel 2.0 build of
  `scripts/harness-trials/builds.json` in a scratch HOME, resolves a clean
  checkout as a `local` source with the roots `["skills", "craft"]`. It lists
  the six kinds and the nine craft skills. Record the `resourceDigest` and the
  tree ID of `plugins/engineering`.
- The first pack commit is on the public `main` through jj. It names the
  Grove commit and the source paths. The EngineeringSkills section of the
  moves record holds the source root, both roots, the commit, the tree ID and
  the digest. It holds the external resources, the external edges and the
  byte changes. It also holds the tool requirements: the language toolchains
  that each coding style names and `codebase-memory-mcp`.

## Notes

The root brief and the node brief authorise the public remote. Never push
Grove's `main` here. Run the pack's commands outside the Grove working copy.
Import no Git history.

The record of DesignSkills states that this session ran `gh repo create`
under an uncommitted `.claude/settings.json` of the Grove workspace. That
file is still uncommitted. Keep it out of every task commit. If the
permission classifier refuses the command, ask the owner to run it at the
prompt.

The doubt check is the first pack check that reads core's text. The
membership decision accepts the trade-off: Grove's suite stops reading the
skill, so a drift appears when the pack moves `GROVE_COMMIT` forward. Choose
the check's language by `linkuistics:machine-environment` and the pack's
Taskfile. Keep Grove's half of each assertion in Grove. Assert in the pack
only what the skill's text states, and read Grove's phrase only to compare.

`simplify-project` cites `codebase-design` and `decision-records` of
DesignSkills. `model-led-development` in DesignSkills cites
`doubt-driven-development`. Each is a citation across packs, not a directed
load, so the pack alone lists no edge for it.

The moving craft skills still exist in `plugins/linkuistics` until k204. In
the meantime, a native user meets a shared skill name, and the installer
refuses a scan of both plugins. That state lives only on this workstream's
line.

If this leaf proves too large, the doubt check is the seam. It can land in a
later leaf before engineering-removal-k204, but then a later pack commit
before the removal holds it. Record which pack commit the marketplace pins.

## Decisions (running log)

- The doubt assertions are seven tests, not eight. Six tests of
  `composition_guidance.rs` read `DOUBT_SKILL`: `no_provisioned_surface_instructs_a_deleted_composition_verb`,
  `guidance_cuts_the_integrate_step_where_pick_reaches_it_next`,
  `doubt_guidance_yields_to_grove_without_changing_standalone_doubt`,
  `every_handoff_passage_retires_before_it_commits`,
  `canonical_guidance_drops_receipt_and_diversity_era_review_routing` and
  `grove_and_doubt_state_one_mandate_based_ownership_predicate`. One test of
  `session_kind_guidance.rs` reads it:
  `every_session_kind_is_named_by_the_provisioned_taxonomy`. The pack's check
  has one assertion for each of the seven tests.
- The doubt check is Bash. The pack's Taskfile already needs only Bash and Git,
  and DesignSkills has no mise file. Bash therefore adds no toolchain.
- `GROVE_COMMIT` is `d65914d57015d88924edf5dc9d22df467ff82264`, the commit of
  impl-consumer-retarget-k202. It still ships every moving file. jj moves
  Grove's remote bookmark `gh-issue-25` from `84fba18c1f98` to it before the
  pack's publication.
- The doubt check is `scripts/doubt-composition.sh` at the pack's repository
  root, outside the source directory, so it changes no `resourceDigest`. Each
  assertion has the skill's half of one Grove test: the phrases that the skill
  holds, the phrases that it does not hold, or the order of its handoff. Each
  assertion also names the Grove phrase that the skill's half composes with,
  and it fails when the pinned checkout does not hold that phrase. The check
  collapses whitespace and compares case-sensitively, as the Rust tests do.
- `scripts/doubt-composition.test.sh` holds the controls. For each assertion,
  one mutation of a copy of the skill and one mutation of a copy of Grove's
  file must each fail that assertion alone. `check:doubt` runs the check, then
  the controls, so a control that stops failing breaks the pack's check.
- `doubt-driven-development` gains `LICENSES/addyosmani-agent-skills.LICENSE`,
  a copy of the file of `grove-impl`. Its provenance section states that it is
  adapted from `addyosmani/agent-skills` (MIT). `simplify-project` adapts no
  third-party passage and gets no `LICENSES/`.
- `measured-workstream.md` names `docs/candidate-lessons.md` as a bare path.
  The pack links it by its public URL, as it already links `docs/review-yield.md`.
- The doubt check landed in the first pack commit, so this leaf did not use
  the seam of the notes. The marketplace entry of engineering-removal-k204
  can pin the first pack commit `d98862f217492a44932c8fa93a006367da252a5d`.
- The record's two older synonym findings (`deleted` at the TestSubject
  readings and `corrects` in the consumer classification) changed to
  `removed` and `repairs`, so the record has no hard finding of the checker.

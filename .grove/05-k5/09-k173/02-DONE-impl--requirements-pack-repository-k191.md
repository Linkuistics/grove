# requirements-pack-repository-k191

## Goal

Create and publish the RequirementsSkills repository with the three
requirements kinds. This is the expand step of the move. Grove keeps its own
copies until requirements-removal-k192, so no Grove consumer changes here.

## Context

- `docs/specs/external-pack-contract.md`: "Layout", "References between
  skills", "Conformance", "Provenance and licences", "Distribution" and
  "Proven facts".
- `docs/specs/pack-membership.md`: "The ADR and spec formats" and "The
  acceptance of each move".
- ResearchSkills at `~/Development/AgenticOrchestration/ResearchSkills`. Copy
  its `Taskfile.yml`, `.gitignore`, `LICENSE` and README shape, and change the
  source directory.
- The moving files in Grove: `plugins/grove/skills/grove-requirements/`
  (`SKILL.md` and `grilling.md`), `grove-review-requirements/` and
  `grove-integrate-review-requirements/`.
- The rows that the moving files own in `plugins/grove/conformance/rules.tsv`.
  At planning, only `grove-requirements/SKILL.md` and `grilling.md` owned rows.
  Search again before you rely on that.
- The baseline of owner-ravel-skill-delivery-k37: `docs/owner-skill-inventory.tsv`
  gives each moving skill its digest and the source ID `requirements`.
- `linkuistics:machine-environment` for the Taskfile and mise conventions of a
  new repository.

## Done when

- `~/Development/AgenticOrchestration/RequirementsSkills` exists as a colocated
  Git and Jujutsu repository, with the public remote
  `https://github.com/Linkuistics/RequirementsSkills`. Its source directory is
  `plugins/requirements`, and its only Ravel root is `skills`. It has no
  `craft/` directory.
- The three kinds open with the contract's bare-name text. The two family
  members keep their read of `references/review.md` or
  `references/integrate-review.md` after the load sentence, as core's planning
  members do. They change no other text.
- `grove-requirements/LICENSES/mattpocock-skills.LICENSE` holds the licence
  text of the passages that the skill adapts and bundles. Its bytes equal the
  spine's copy. The skill's pointers to that file then resolve in the skill
  itself.
- The record names each byte change against Grove's files and each moved
  skill's baseline entry, as proof 5 of the membership specification requires.
- `plugins/requirements/.claude-plugin/plugin.json` sets the name
  `requirements`, the licence `Apache-2.0` and the dependencies `["grove"]`,
  with no `version`.
- `plugins/requirements/conformance/rules.tsv` holds every row that the moving
  files own, with their owners and loads unchanged.
- The repository root holds `LICENSE`, `.gitignore` with `.cache/`, a
  Taskfile and a README. The README names the source directory, the root, the
  kinds, the Ravel record, the install commands and the provenance. It states
  that the ADR, spec and glossary formats stay in core.
- The Taskfile has the tasks and variables of ResearchSkills.
  `GROVE_COMMIT` names a Grove commit that still ships the moving files, and
  `GROVE_REPOSITORY` keeps the public URL.
- Before the pack's first publication, push this workstream's line to the
  bookmark `gh-issue-25` on Grove's remote through jj. The recorded Grove
  commit is an ancestor of the pushed bookmark.
- From a clean checkout of the first pack commit, with the committed public
  URL and no override, the pack-alone check and `check:plugin` pass. The
  pack-alone output lists `references/review.md`,
  `references/integrate-review.md` and the format files that the moved text
  names as external resources. It lists the loads of `grove` as external
  edges. The aggregate fails only because each moving skill and each moved
  row is a duplicate. Record both outputs.
- `ravel skills resolve --json`, from the Ravel 2.0 build of the research
  proof in a scratch HOME, resolves a clean checkout as a `local` source.
  Record the `resourceDigest` and the tree ID of `plugins/requirements`.
- The first pack commit is on the public `main` through jj. It names the Grove
  commit and the source paths. `docs/repository-moves-record.md` gains a
  "RequirementsSkills" section with the source root, the commit, the tree ID,
  the digest, the external resources and the byte changes.

## Notes

The root brief and the node brief authorise the public remote. Never push
Grove's `main` here. Run the pack's commands outside the Grove working copy.
Import no Git history.

At planning, the spine was the only Grove skill that held
`mattpocock-skills.LICENSE`. The contract adds `LICENSES/` inside the skill
that adapts a passage, as `grove-impl` does. Only `grove-requirements` adapts
or bundles such passages among the moving skills. Check that again.

## Decisions (running log)

- The recorded Grove commit is `6cdab3a9883342e68fe51b3913d8c72ddb26e3bf`,
  the planning commit of this node. It still ships the moving files. jj moved
  the bookmark `gh-issue-25` forward from `2328c7f0d45a` to that commit and
  pushed it before the pack's first publication.
- The manifest keeps the ten moved rows in core's order and byte for byte.
  Three rows have the owner `grilling.md` and seven have the owner `SKILL.md`.
  No row of the two family members exists, so their manifest share is empty.
- The aggregate fails on 23 assertions: 3 duplicate skills, 10 duplicate rows
  and 10 unshipped owners. The runner maps a skill name that two packs ship to
  the first pack, which is core. Each pack row whose owner is in a duplicate
  skill therefore reads as unshipped. The duplicates cause all 23 failures, and
  each failure names a moving skill or a moved row.
- The licence pointer is inside an HTML comment, so the runner does not read
  it. `LICENSES/mattpocock-skills.LICENSE` inside the skill resolves it on
  every route without the spine.
- The permission classifier refused `gh repo create --public`. The owner
  chose to create the repository at the prompt, and then jj pushed `main`.
  The evidence of the record comes from a plain checkout fetched from the
  public remote, and it equals the earlier local reading.

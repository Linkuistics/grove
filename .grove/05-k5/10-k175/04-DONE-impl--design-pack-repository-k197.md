# design-pack-repository-k197

## Goal

Create and publish the DesignSkills repository with the three design kinds in
`skills/` and the four design craft skills in `craft/`. This is the expand
step of the move. Grove keeps its own copies until design-removal-k198, so no
Grove consumer changes here.

## Context

- `docs/specs/external-pack-contract.md`: "Layout" (the paragraph on a later
  pack with craft skills), "References between skills", "Conformance",
  "Provenance and licences", "Distribution", "Checkout boundary" and "Proven
  facts".
- `docs/specs/pack-membership.md`: "The ADR and spec formats", "The Grove
  assets of moving craft skills", "The Grove tests that name moving kinds" and
  the row "The first `craft/` root".
- `docs/adr/a-skill-states-what-binds-without-its-dependencies.md` and the
  dependency decision record that it names for citations across packs.
- RequirementsSkills at `~/Development/AgenticOrchestration/RequirementsSkills`
  and requirements-pack-repository-k191 as the model. Copy its
  `Taskfile.yml`, `.gitignore`, `LICENSE` and README shape, and change the
  source directory.
- `docs/specs/owner-skill-delivery.md`: the source `design` is DesignSkills,
  with the roots `skills` and `craft` in its plugin directory.
- The baseline of owner-ravel-skill-delivery-k37: `docs/owner-skill-inventory.tsv`
  gives each moving skill its digest and its source.
- The moving files at planning. Search again before you rely on the list.
  - `plugins/grove/skills/grove-design/`: `SKILL.md`,
    `assets/diagram-viewer.html` and `references/visual-design-discussion.md`.
  - `plugins/grove/skills/grove-review-design/SKILL.md` and
    `grove-integrate-review-design/SKILL.md`.
  - `plugins/linkuistics/skills/decision-records/SKILL.md`.
  - `plugins/linkuistics/skills/codebase-design/`: `SKILL.md` and
    `references/design-examples.md`.
  - `plugins/linkuistics/skills/cli-tool-design/`: `SKILL.md` and
    `references/auditing-and-refactoring.md`.
  - `plugins/linkuistics/skills/model-led-development/`: `SKILL.md` and three
    files in `references/`.
- The rows of `plugins/grove/conformance/rules.tsv` whose owner is a moving
  file. At planning, six rows had owners in `grove-design/`.
- The registry row of `crates/grove/tests/plugin_fallback.rs` for the citation
  of `codebase-design` in `grove-design/SKILL.md`.
- `linkuistics:machine-environment` for the Taskfile and mise conventions of a
  new repository.

## Done when

- `~/Development/AgenticOrchestration/DesignSkills` exists as a colocated Git
  and Jujutsu repository, with the public remote
  `https://github.com/Linkuistics/DesignSkills`. Its source directory is
  `plugins/design`, and its Ravel roots are `["skills", "craft"]`.
- `plugins/design/skills/` holds the three kinds. Each resource of
  `grove-design` stays inside that skill.
- `plugins/design/craft/` holds the four craft skills with their resources.
- `plugins/design/PROVENANCE.md` holds the sections of `plugins/linkuistics/PROVENANCE.md`
  for the four craft skills. Each relative link of a moved craft skill to
  `PROVENANCE.md` then resolves inside the source directory. Grove keeps its
  copy of those sections until design-removal-k198.
- The three kinds open with the contract's bare-name text. The two family
  members keep their read of `references/review.md` or
  `references/integrate-review.md` after the load sentence.
- `model-led-development` names `doubt-driven-development` by bare name. The
  citation stays a citation, as the dependency decision record requires.
- The moved craft skills cite Grove's formal-methods records by public URL.
  Those records stay in Grove.
- An adapted third-party passage in a moved skill has its licence text inside
  that skill's `LICENSES/`, as "Provenance and licences" directs. Check
  `decision-records` first. A skill without such a passage gets no
  `LICENSES/`.
- The record names each byte change against Grove's files and each moved
  skill's baseline entry, as proof 5 of the membership specification requires.
- `plugins/design/.claude-plugin/plugin.json` sets the name `design`, the
  licence `Apache-2.0`, the dependencies `["grove"]` and
  `"skills": ["./craft/"]`, with no `version`. `claude plugin validate`
  accepts it, and its warnings are recorded.
- `plugins/design/conformance/rules.tsv` holds every row that the moving files
  own, with their owners and loads unchanged. It also holds the registry row
  for the citation of `codebase-design` as a pinned row. The pack alone then
  checks that `grove-design/SKILL.md` states its fallback phrase.
- The repository root holds `LICENSE`, `.gitignore` with `.cache/`, a
  Taskfile and a README. The README names the source directory, both roots,
  the kinds, the craft skills, the Ravel record, the install commands and the
  provenance. It states that the ADR and spec formats stay in core and that the
  craft skills deepen them. It states that conformance covers only `skills/`
  and that `check:plugin` checks the craft skills.
- The Taskfile has the tasks and variables of RequirementsSkills.
  `GROVE_COMMIT` names a Grove commit that still ships the moving files, and
  `GROVE_REPOSITORY` keeps the public URL.
- Before the pack's first publication, push this workstream's line to the
  bookmark `gh-issue-25` on Grove's remote through jj. The recorded Grove
  commit is an ancestor of the pushed bookmark.
- From a clean checkout of the first pack commit, with the committed public
  URL and no override, the pack-alone check and `check:plugin` pass. The
  pack-alone output lists the format files and the family files that the moved
  kinds name as external resources. It lists the loads of `grove` as external
  edges. The aggregate fails only because each moving kind and each moved row
  is a duplicate. Record both outputs.
- `ravel skills resolve --json`, from the Ravel 2.0 build of the research
  proof in a scratch HOME, resolves a clean checkout as a `local` source with
  the roots `["skills", "craft"]`. It lists the three kinds and the four craft
  skills. Record the `resourceDigest` and the tree ID of `plugins/design`.
- The first pack commit is on the public `main` through jj. It names the Grove
  commit and the source paths. `docs/repository-moves-record.md` gains a
  "DesignSkills" section. It holds the source root, both roots, the commit,
  the tree ID and the digest. It also holds the external resources, the
  external edges and the byte changes.

## Notes

The root brief and the node brief authorise the public remote. Never push
Grove's `main` here. Run the pack's commands outside the Grove working copy.
Import no Git history.

The permission classifier refused `gh repo create --public` in k191. There,
the owner created the repository at the prompt, and then jj pushed `main`.
Ask the owner in the same way if the classifier refuses again.

`codebase-design` already names `doubt-driven-development` by bare name.
`model-led-development` used the form `linkuistics:doubt-driven-development`
in `SKILL.md` and in `references/instruments.md` at planning. Search again.

The four craft skills still exist in `plugins/linkuistics` until k198. In
the meantime, a native user meets a shared skill name. The installer also
refuses a scan of both plugins. That state lives only on this workstream's
line.

## Decisions (running log)

- The recorded Grove commit is `c800d311a541b02237f06b5a9b5ddba4c2993765`,
  the commit of trial-supplier-refusal-k196. It still ships the moving
  files. jj moved the bookmark `gh-issue-25` forward from `302f85429e4b` to
  that commit and pushed it before the pack's first publication.
- Grove's `grove-design` has the digest `7a610371ecb2c752`, not the baseline
  `beee6cbee6524784`. core-bare-references-k170 (`5a6bdd68a7d8`) changed its
  citation of `codebase-design` to a bare name on this line. The record names
  that change beside the pack's own changes. The other six Grove copies have
  their baseline digests.
- `decision-records` adapts the template, the examples and the when-to-write
  test of `mattpocock/skills` (MIT). It therefore gets
  `LICENSES/mattpocock-skills.LICENSE`, a copy of the spine's file. Its
  `SKILL.md` keeps its pointer to `PROVENANCE.md`, and the pack's provenance
  section names the licence file. `codebase-design` cites works and keeps an
  inherited attribution for an exercise that it no longer contains.
  `cli-tool-design` and `model-led-development` adapt nothing. Those three get
  no `LICENSES/`.
- The formal-methods records take `https://github.com/Linkuistics/grove/blob/main/docs/<file>`.
  Grove's public `main` (`238c3a839ab4`) holds both cited files, and each URL
  returned 200. The text keeps "in `Linkuistics/grove`" and links the path.
- `plugins/design/PROVENANCE.md` holds the three sections of Grove's file
  that cover the craft skills. `cli-tool-design` has no section. The
  `model-led-development` section said "this repository", which in the pack
  names Grove by name and URL. `codebase-design` is byte for byte.
- The pinned citation row is `design-cites-codebase-design-with-fallback`,
  owner `grove-design/SKILL.md`, class `B`, load `static({design})`, with the
  registry's phrase in its source case. The runner normalises case, markup and
  line breaks. Two controls failed on that row alone: a changed phrase in the
  owner, and a second site in `grove-review-design`.
- The moved kinds name no format file. The pack alone therefore lists only
  `references/review.md` and `references/integrate-review.md` as external
  resources, with the three loads of `grove` as external edges.
- The aggregate at the recorded commit fails on 16 assertions: 3 duplicate
  skills, 6 duplicate rows and 7 unshipped owners. The seventh unshipped owner
  is the pinned row, whose owner is in the duplicate `grove-design`. The
  duplicates cause all 16.
- `claude plugin validate` (Claude Code 2.1.318) passed with two warnings, no
  `version` and no icon, as in the requirements record.
- `gh repo create Linkuistics/DesignSkills --public` ran without a prompt. An
  uncommitted `.claude/settings.json` in this workspace allows
  `Bash(gh repo create *)`. This session did not write that file, and the
  task commit leaves it out. jj pushed `main` as
  `9d7c7c5368f6a22b541881e6c107da28e39627bd`.
- Grove's `bash scripts/check.sh` passed on this leaf's Grove change, with all
  21 principal checks. The change touches only the moves record and this task
  file.

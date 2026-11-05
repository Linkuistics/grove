# writing-pack-repository-k205

## Goal

Create and publish the WritingSkills repository with the four editorial kinds
and the family skill `editorial-family` in `skills/`. This is the expand step
of the move. Grove keeps its own copies until editorial-removal-k206, so no
Grove consumer changes here.

## Context

- `docs/specs/external-pack-contract.md`: "Layout", "The family procedure",
  "References between skills", "Rule placement", "Manifest changes",
  "Provenance and licences", "Distribution", "Checkout boundary", "The checker
  without a copy" and "Proven facts".
- `docs/specs/pack-membership.md`: "The acceptance of each move", the row "A
  family skill outside research".
- `docs/specs/owner-skill-delivery.md`: the source `writing` is WritingSkills.
  Its plugin directory is therefore `plugins/writing`, and the native namespace
  is `writing:`. This move ships the root `skills` alone.
  writing-craft-pack-k181 adds `craft` and the hooks plugin.
- ResearchSkills at `~/Development/AgenticOrchestration/ResearchSkills` is the
  model of a family skill. RequirementsSkills is the model of a pack with the
  root `skills` alone, for its `Taskfile.yml`, `.gitignore`, `LICENSE`,
  manifest and README. Copy them, and change the source directory.
- The baseline of owner-ravel-skill-delivery-k37: `docs/owner-skill-inventory.tsv`
  and `docs/evaluations/owner-skill-delivery/inventory.py`.
- `linkuistics:machine-environment` for the conventions of a new repository.

The moving files at planning. Search again before you rely on the list.

- `plugins/grove/skills/grove-draft/`, `grove-copy-edit/`, `grove-art/` and
  `grove-proof/`, each a `SKILL.md` alone.
- `plugins/grove/skills/grove/references/editorial.md`.
- The eleven rows of `plugins/grove/conformance/rules.tsv` whose owner is
  `grove/references/editorial.md`. Each has the load `static(editorial)`.

## Done when

- `~/Development/AgenticOrchestration/WritingSkills` exists as a colocated Git
  and Jujutsu repository, with the public remote
  `https://github.com/Linkuistics/WritingSkills`. Its source directory is
  `plugins/writing`, and its Ravel roots are `["skills"]`.
- `plugins/writing/skills/editorial-family/SKILL.md` holds the body of
  `references/editorial.md` with one recorded change, stated in the notes. It
  gains the frontmatter of the contract's "The family procedure", changed for
  the editorial family: the name `editorial-family`, a description that names
  the four kinds, and `harnesses: [any]`. It sets no
  `disable-model-invocation`.
- `plugins/writing/skills/` holds the four kinds. Each opens with the
  contract's text for `research-a`, changed for the editorial family. It
  loads `grove` and then `editorial-family`. It states the namespace rule and
  the rule for a missing kind skill. The next paragraph names the
  `editorial-family` skill where it now says "that file". No moved text names
  a spine path for the family file, or the alias `grove:grove`.
- A citation of another skill in a moved file is a bare name, and it stays a
  citation. A mention of a Grove record that stays in Grove is a public URL.
  No moved file adapts a third-party passage, or the record states which one
  does and holds its licence in that skill's `LICENSES/`.
- `plugins/writing/conformance/rules.tsv` holds the eleven rows with the
  owner `editorial-family/SKILL.md` and the load
  `static({draft,copy-edit,art,proof})`. Each row keeps its identifier, class
  and pinned phrase.
- `plugins/writing/.claude-plugin/plugin.json` sets the name `writing`, the
  licence `Apache-2.0` and the dependencies `["grove"]`, with no `version`.
  `claude plugin validate` accepts it, and the record names its warnings.
- The repository root holds `LICENSE`, `.gitignore` with `.cache/`, a
  Taskfile and a README. The README names the source directory, the root, the
  kinds, the family skill, the Ravel record, the install commands and the
  provenance. It states that writing-craft-pack-k181 adds the writing craft
  and the hooks plugin.
- The Taskfile has the tasks and variables of RequirementsSkills.
  `GROVE_COMMIT` names a Grove commit that still ships the moving files and
  is an ancestor of the remote bookmark `gh-issue-25`. If the bookmark does
  not hold that commit, push this workstream's line to it through jj. Do it
  before the pack's first publication.
- From a clean checkout of the first pack commit, with the committed public
  URL and no override, the pack-alone check and `check:plugin` pass. The
  pack-alone output lists four external edges to `grove` and no missing file.
  The aggregate fails only because each moving kind and each moved row is a
  duplicate. Record the outputs.
- `ravel skills resolve --json`, from the Ravel 2.0 build of
  `scripts/harness-trials/builds.json` in a scratch HOME, resolves a clean
  checkout as a `local` source with the roots `["skills"]`. It lists the four
  kinds and `editorial-family`. Record the `resourceDigest` and the tree ID of
  `plugins/writing`.
- The first pack commit is on the public `main` through jj. It names the Grove
  commit and the source paths. A new WritingSkills section of
  `docs/repository-moves-record.md` holds the source root, the root, the
  commit, the tree ID, the digest and the external edges. It also names each
  byte change against Grove's files and against the baseline entries, as
  proof 5 of the membership specification requires.

## Notes

**The one change to the family file.** A byte-for-byte copy with frontmatter
alone fails the pack-alone check. The planning session proved it with a
scratch pack and the runner of the Grove line. Line 75 of `editorial.md` names
`references/execute.md`. Under rule 3 of "References between skills", that
path resolves in the referring skill or in a skill that the same file loads.
`editorial-family` loads nothing, so the runner reports "missing file".

The research family file names no spine path, so the research move did not meet
this case.

The planning session decided the change. "no stage is `references/execute.md`'s"
becomes "no stage is the spine's". With that change alone the scratch pack
passed alone: 11 single-source checks, 44 loaded-path checks and 4 external
edges.

The reader has already loaded the spine, which sends an in-session
reviewer to `references/execute.md`. The planning session rejected two other
options. A directed load of `grove` in the family skill adds a dependency
sentence and the rule-5 sentence for one pointer. A change of rule 3 for family
skills changes a reviewed contract. If the move reveals another unresolved
path, apply the same test and record each change.

No pinned phrase of the eleven rows contains the changed words, so the rows
need no change for it.

The root brief and the node brief authorise the public remote. Never push
Grove's `main` here. Run the pack's commands outside the Grove working copy.
Import no Git history.

`.claude/settings.json` in the Grove working copy is
the owner's local permission. Keep it out of every task commit. If the
permission classifier refuses `gh repo create`, ask the owner to run it at the
prompt.

Until editorial-removal-k206, a native user meets shared skill names, and the
installer refuses a scan of both plugins. That state lives only on this
workstream's line.

## Decisions (running log)

- `GROVE_COMMIT` is `f45c5a53796f2a5bf634fec340cb0d7aef63e184`. It is the head
  of Grove's remote bookmark `gh-issue-25`, so no push is needed. The two later
  commits of this line change only `.grove/` and `docs/`, and `git diff` shows
  the four kind directories unchanged against the line head.
- The pack manifest holds 23 rows, not 11. Grove's manifest has 11 rows whose
  owner is `grove/references/editorial.md` and 12 rows whose owners are the four
  kind skills (`grove-art` 3, `grove-copy-edit` 2, `grove-draft` 3,
  `grove-proof` 4). The contract's "Manifest changes" moves every row that a
  moving file owns, and the research and requirements moves moved their
  kind-owned rows. The two pack commits must also have the same
  `plugins/writing`, so the kind rows belong in the first commit. If the rows
  stayed in Grove, the removal would leave 12 rows with absent owners, and the
  runner fails such a row. The 11 family rows take the owner
  `editorial-family/SKILL.md` and the load
  `static({draft,copy-edit,art,proof})`. The 12 kind rows keep their owner and
  their load, as the `combine-research` rows did. editorial-removal-k206 must
  therefore remove 23 rows from Grove's manifest.
- The family body has the one planned change on line 75 and no other. The
  sweep of the moved files found no other spine path, no Grove record link,
  no craft-skill citation and no adapted third-party passage. The kinds keep
  "the family file's rule" in their last-act sentences, because the glossary
  entry names the family skill as an external pack's family file.
- The moved text keeps its technical-communication findings, as the design
  pack's record did. The new openings, the frontmatter, the README and the
  record section have no hard finding.
- The first pack commit is `694140a18dc7`, tree `361070804328`, digest
  `db96fa09905e…`. The pack alone passes with 23, 56 and 4 external edges.
  The aggregate fails on 105 assertions, every one caused by a duplicate.
- The permission classifier refused `gh repo create`. The owner runs it.

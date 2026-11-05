# repository-moves-k5 — brief

## Goal

Complete the remaining domain, personal, TestAnyware and marketplace moves,
and relocate the four writing projects, using the proven external-pack contract.

## Done when

- Every accepted destination and membership in the root brief holds.
  Core retains planning, its review/integration entry skills and shared
  procedures. Each domain separates kind skills from craft skills.
- All remaining packs have complete resources and own repeatable conformance
  checks. The aggregate core/external checks and affected product checks pass.
- Personal source and hooks live in PersonalSkills. Machine configuration that
  points to them follows the current mise-managed process and maintained manual.
  Do not resurrect chezmoi or put personal sources under .config. Account for
  native hook behaviour separately from ordinary Markdown guidance.
- TestAnyware's own repository owns and checks TestAnyware's skill.
  The marketplace-only repository points to real source repositories, and
  installation guidance/distribution consumers agree with each completed move.
- Stylepack, TestSubject, TheGreatExplainer and Writegood move with their
  contents, unrelated edits, history and actual workspace registrations
  preserved. Reconcile affected references and check registrations at the new
  paths. The moves include no change of product behaviour.
- A durable record holds the owning-repository commits and the migration
  checks, enough for discovery-rollout-k6 to integrate and release independent
  products.
- Each pack repository is on its public remote. Each move gives
  rollout the published commit that a pin can name. A plain Git checkout of
  that commit works as a local source. The published line of this workstream
  holds the `grove-core` candidate, as order 2 of the owner-delivery cutover
  contract states.
- Reconcile each moved skill/resource/tool/hook with the baseline from
  owner-ravel-skill-delivery-k37. Preserve third-party and other installed
  skills outside this repository's packs. Give rollout exact source roots and
  revisions for the owner's full Ravel collection, including TestAnyware.
  Actual native-installation removal follows replacement acceptance in rollout.

## Decomposition

Start with planning from the research proof. Create a separate subtree for
each independently useful domain or project relocation. Combine only moves
that cannot leave a working product alone. Each move includes its callers,
resources, configuration/distribution references, checks and preservation
evidence.

Later migrations must not be represented merely by an aspirational cleanup
task. The next planner must materialise each precise remaining obligation.
Details of resource layout depend on the external research-pack proof.

repository-moves-k11 cut the children in this order. Its running log records
the inventory and the reasons.

1. `writing-projects-k165`: the four project relocations. They depend on no
   other move.
2. `core-bare-references-k170`: core names other skills by bare names before
   the craft skills leave the `linkuistics` plugin.
3. `pack-membership-k171`: the open membership questions, settled before the
   moves that need them.
4. `testanyware-skill-move-k172`: the TestAnyware skill in its own repository.
5. `requirements-pack-k173`: the first external member of the cross-pack
   review families.
6. `design-pack-k175`: the first `craft/` root.
7. `engineering-pack-k177`: after design, because a design craft skill cites
   an engineering one.
8. `editorial-pack-k179`: WritingSkills and the editorial family skill.
9. `writing-craft-pack-k181`: the writing craft and the native hooks plugin.
10. `personal-pack-k183`: the last craft move. It removes the empty
    `linkuistics` plugin.
11. `skill-marketplace-k185`: the marketplace, after every pack exists.
12. `moves-handoff-k187`: the per-source record for the owner cutover.

Each pack node starts with its own planning leaf. Each move appends its
results to `docs/repository-moves-record.md`.

## Pointers

The root's destination table is authoritative. Do not maintain a second table
here. Read each owning repository's instructions before its work becomes
concrete. Use the latest manual for machine configuration ownership.

The proven contract is `docs/specs/external-pack-contract.md`.
external-research-pack-k4 closed with its section "Proven facts". That
section holds the commits, the builds, the proven boundaries, the reusable
pack tooling and the differences from the decisions. Each later pack copies
the tooling of ResearchSkills and changes its source directory.

## Notes

Inspect each source repository's actual Git/Jujutsu/workspace arrangement
before relocation. Check destination collisions, external workspaces and path
references. Use supported workspace relocation/repair procedures. The
authorisation covers these moves and preservation. It does not cover
overwriting unrelated destinations or discarding changes. Do not signal other
sessions or edit their task trees.

`writing-projects-k165` closed after the four projects moved. Its results
are in the four project sections of `docs/repository-moves-record.md`.
Three of them matter to later work.

- The rollout text for paused groves needs one entry. The Ravel grove that
  owns `rhai-book-use-k74` must read TestSubject at its new path.
- Writegood has no remote, so it has no published commit for a pin.
- Some registrations and harness entries name earlier roots, and they stay
  unchanged. Stylepack has a prunable Git worktree registration. Writegood
  has Claude Code and Codex entries for `~/Development/Writegood`.

`requirements-pack-k173` closed after its directory and native trials passed.
Its results are in the RequirementsSkills section of
`docs/repository-moves-record.md`. Two of them matter to later work.

- The cases `requirements-family-read` and `requirements-native-namespace`
  are the models for a later pack whose kinds read a family file in the
  spine. `editorial-pack-k179` can copy them.
- The owner's native marketplace lists `requirements` only after finish.
  Rollout then installs `requirements@linkuistics`, as the section "For
  discovery-rollout-k6" of that record states.

`design-pack-k175` closed after its directory and native trials. Its results
are in the DesignSkills section of `docs/repository-moves-record.md`. Four of
them matter to later work.

- The installer's craft-only fixture is `make_craft_only_pack` in
  `install.test.sh`. `personal-pack-k183` consumes it.
- The craft trial models are `design-craft-discovery` for the directory
  route and `design-native-namespace` for the native route. The engineering
  and writing craft moves can copy them.
- The trial runner now accepts a native plugin's dependency as the supplier
  of a kind skill. A `requires` entry can name a native plugin. The runner's
  plugin inventory reads each root of the manifest's `skills` field.
- In one of two native runs, the agent did not load the craft skill. The
  acceptance record names this as a limit of the native route, not a defect
  of the pack.

`engineering-pack-k177` closed after its removal and its second pack commit.
Its results are in the EngineeringSkills section of
`docs/repository-moves-record.md`. Three of them matter to later work.

- A craft skill whose text composes with Grove's text keeps its half of each
  Grove test in the pack. `scripts/doubt-composition.sh` of EngineeringSkills
  is the model. It reads Grove's matching phrase from `GROVE_COMMIT`, and its
  controls mutate each side. `writing-craft-pack-k181` can copy it for any
  Grove test that reads a writing craft skill.
- `machine-environment` carries `assumes-personal-setup`, and the installer
  reports it. `plugins/README.md` and `plugins/CONTEXT.md` name it.
  `personal-pack-k183` repairs those two claims when the skill leaves.
- `authoring-conventions` and `guardrail` cite `doubt-driven-development` and
  `using-codebase-memory` by bare name. Those citations stay valid when the
  personal skills move.

`editorial-pack-k179` closed after its removal and its directory trials.
Its results are in the WritingSkills section of
`docs/repository-moves-record.md`. Three of them matter to later work.

- WritingSkills has only the root `skills`. `writing-craft-pack-k181` adds
  the `craft/` root and the hooks plugin in the same repository. Grove's
  marketplace pins the first pack commit `694140a18dc7` until that move.
- The section "For the paused editorial grove" of the moves record holds the
  fact that rollout's brief-update text states.
- The case `editorial-family-read` is the model for a kind that loads a
  family skill from its own pack.

During design-native-namespace-k200, the human inserted
`skill-marketplace-decision-k201` before `skill-marketplace-k185`. The human
is not sure that this workstream needs a skills marketplace. Do not start
marketplace work before that leaf records the human's decision.

`writing-craft-pack-k181` closed after its removal and its last pack commit.
Its results are in the WritingSkills section of
`docs/repository-moves-record.md`. Three of them matter to later work.

- Codex 0.166.0 lists a `local` marketplace source only inside the
  marketplace root. `skill-marketplace-k185` therefore cannot list the Codex
  hooks plugin from a marketplace-only repository through a `local` source.
  The section "For skill-marketplace-k185" of that record holds the reading.
- Grove's marketplace lists `technical-communication-hooks`, a hooks-only
  plugin, from the same pack commit as `writing`. `check:packs` gives it the
  directory-only check. A later pin change moves both entries together.
- `plugins/linkuistics` now holds `authoring-conventions`, `guardrail`,
  `machine-environment` and `using-jujutsu`, with no hooks file.
  `personal-pack-k183` removes those four. The installer's Pi extension case
  now uses a pack fixture and reads no Grove skill.

`personal-pack-k183` closed after its last pack commit and its removal. Its
results are in the PersonalSkills section of `docs/repository-moves-record.md`.
Four of them matter to later work.

- Grove now ships only the `grove` plugin. The marketplace keeps the name
  `linkuistics` and lists ten plugins. `personal` and `guardrail` come from
  PersonalSkills commit `1ff489d996a9`. The dependency of `personal` resolves
  only in a marketplace that also lists `guardrail`. `skill-marketplace-k185`
  must keep both entries together.
- The owner's native `linkuistics@linkuistics` comes from Grove commit
  `238c3a839ab4`. Grove's `main` lists that plugin until finish, and then it
  does not. Rollout installs `personal@linkuistics`, which also installs
  `guardrail`, and it removes `linkuistics@linkuistics`.
- The section "For moves-handoff-k187" of that record gives the source root
  and revision of the `personal` source and of the two native plugins.
- No `craft/` skill remains in Grove. The fixtures of `install.test.sh` keep
  the synthetic label `linkuistics`, and the record says why.

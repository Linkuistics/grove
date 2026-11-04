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

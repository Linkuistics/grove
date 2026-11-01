# ravel-pinned-delivery-k25 — brief

## Goal

Extend the working local route to exact pinned Git sources, completing the
generic source contract of Ravel before Grove adopts directory delivery. Rhai
native compatibility is independent of this feature.

## Done when

- Public resolution and launch acquire exactly the configured commit and
  subdir. They check the object format and the filesystem, and they use
  noninteractive bounded transport. They execute no hook, filter or submodule,
  and no inherited completion authority reaches them.
- Checked exact cache hits require neither network nor Git. Missing cold pins,
  corrupt cache, object and subdir errors and timeouts keep distinct remedies.
  No nearby revision, branch or source is substituted.
- Pin changes, concurrent publication, mixed required local and Git sources and
  retained readers satisfy the same descriptor, resource and confinement
  contract.
- The full discovery acceptance fixtures of Ravel pass against a reproducible
  matched independent build. So do the applicable repository and book checks,
  the installed layout and the Git transport dependency checks. Owning
  documentation and commits record the handoff.

## Decomposition

The child planner consumes actual local-delivery implementation. Git transport
and extraction must reach existing public readers in each working increment.
Private acquisition machinery alone does not close this node.

## Pointers

Use the reviewed `docs/specs/skill-discovery.md` of Ravel, with its exact-Git,
publication, failure and acceptance sections. Its reviewed form is commit
`822c0ed2f67d` on `skill-directory-delivery`, which
ravel-rhai-skill-delivery-k50 recorded. Use the owning commits and
recorded installed-pair inputs of the preceding local node. Use the repaired
contract from ravel-rhai-skill-delivery-k34, not the historical TypeScript and
schema-3 design. Follow the repository instructions of Ravel.

## Settled inputs

- The identity is that of the local stage: the combined Ravel 2.0.0, script and
  request schema 4 and worker protocol 1.
- Item 5 of the stage handoff in the Ravel evidence document lists the work of
  this stage. A cache miss needs Git on the `PATH` of the Ravel CLI. This
  stage decides whether the Homebrew formula declares Git.
- The installed and release preparation tasks are `task smoke`,
  `task install PREFIX=<prefix>`, `task release:test`, `task release:formula`,
  `task release:archives` and `task release:smoke`. The repository and book
  tasks are those of the local stage.

## Notes

Only source acquisition extends here. This stage introduces no skill catalogue,
Grove dependency, native installer or garbage collector. Final publication
belongs to discovery-rollout-k6. Use isolated temporary installs for
acceptance.

At entry the owning pinned implementation session rebases the recorded local
commits onto current Ravel main. It checks the integrated local path and
records the exact base. Keep all work on `skill-directory-delivery`, outside
main, through acceptance. Rollout owns final rebase, checks and landing of the
complete feature. Remove the temporary Git refusal of the local subset and its
fixture when the public Git path works. Neither may survive into publication.

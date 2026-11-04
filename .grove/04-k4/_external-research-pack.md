# external-research-pack-k4 — brief

## Goal

Deliver ResearchSkills as the first independent external pack, proving the
cross-repository loading contract before the remaining migrations.

## Done when

- Research kind skills and their family procedure live in the accepted
  colocated Git/Jujutsu ResearchSkills repository, with complete resources,
  provenance/licenses and their own repeatable checks.
- Agents can find the research kind skills, core and supporting resources through
  Ravel's supplied directories. Conformance passes for core and the external pack
  without requiring source files left behind in Grove to make it pass.
- ResearchSkills is published on its remote. A checkout of a published
  commit, which a tool outside Ravel makes, works as a local source. The
  proof uses a plain Git checkout and requires no particular checkout tool.
  A missing checkout or root refuses as `skill_source_missing` before any
  worker. Run each test from a directory outside the Grove working copy, the
  ResearchSkills working copy and the checkout. Use the actual session and
  delegate read permissions.
- Existing research labels and routing remain compatible. Marketplace/source
  references and any packaging/build consumers affected by this move are
  reconciled in the same working increment.
- A native Claude Code installation loads a moved kind under its actual plugin
  namespace from Grove's bare-name mandate. Directory loading also succeeds
  with native copies absent, under the delivery specification's acceptance cases.
- Applicable repository checks pass. Grove's full check uses the independent
  compatible Ravel build. The durable external-pack contract records the proven
  layout, resource, checkout and conformance boundary for later migration
  planners. The checkout boundary names the source directory and roots inside
  any checkout of the pack.

## Decomposition

The first child plans from the delivered common-skill-loading-k3 contract and
actual repository state. Grow design only for questions that proof still needs,
then focused migration and acceptance leaves. Do not migrate a second domain
until this subtree's working proof holds.

external-research-pack-k10 cut these children. The order follows the
dependencies, and each handoff leaves Grove green:

1. `external-pack-contract-k155` (design) settles the contract that every
   migration leaf depends on. Its leaf body lists the questions.
2. `pack-conformance-runner-k156` extends Grove's checker to a single pack
   and to an aggregate. Core alone gives the same result. This is the expand
   step.
3. `research-pack-repository-k157` creates and publishes ResearchSkills.
   Grove keeps its copies, so no consumer changes yet.
4. `grove-research-removal-k158` removes the files from Grove. It reconciles
   the spine, the manifest, the tests, the marketplace, the installer and the
   documents in one change. This is the move.
5. `trial-external-sources-k159` lets the trial runner take a source from an
   external checkout at a commit.
6. `research-directory-trials-k160` runs the directory proof in both harnesses
   with native copies absent.
7. `research-native-namespace-k161` runs the native Claude Code proof under the
   plugin's actual namespace.
8. `external-pack-proof-k162` records the proven contract and audits this
   node's Done when.

The pack creation and the removal are separate increments. The pack alone
changes no consumer. The removal needs a published pack commit. A step that
produces a load-bearing artifact can earn a lazy review chain.

## The contract artifact

The durable external-pack contract is the Grove specification
`docs/specs/external-pack-contract.md`. Grove owns it because Grove owns core,
the conformance checker and the rules for references between packs.
`external-pack-contract-k155` writes its decisions. `external-pack-proof-k162`
adds the proven facts. `owner-skill-delivery.md` "Ravel sources" and
`codex-skill-provisioning.md` "Resources and conformance" refer to it and do
not restate it.

## Evidence before repository-moves-k5

repository-moves-k5 starts only when this node has the evidence below. The
proof leaf names each commit and build identity:

- A published ResearchSkills commit whose own checks pass from a clean
  checkout.
- A Grove commit on this workstream's line with no research files. Core and
  aggregate conformance pass on it, and `bash scripts/check.sh` passes with
  the Ravel 2.0 build.
- Directory-delivery records in Claude Code and Codex from a plain checkout
  and from the working copy. The records include a delegate read and the
  `skill_source_missing` refusal for a missing checkout and a missing root.
- A native Claude Code record under the plugin's actual namespace.
- The contract with its proven facts and its reusable pack tooling. The
  tooling is the conformance invocation, the Taskfile tasks, the plugin
  manifest and the trial-runner fields.

## Pointers

The root brief owns repository destination and research membership. Preserve
rule ownership and declared restatement classes from the cited ADRs.
Keep reusable pack checks in their owning repository's Taskfile.

## Notes

This stage moves the research pair and the combiner. It does not commission
unrelated literature surveys. Update source distribution references as each
move lands. The full marketplace relocation remains in the later
repository-moves-k5 scope.

acceptance-completion-audit-k127 wrote the "Research recipes" section of
`docs/common-loading-acceptance-record.md`. It covers the install, the
settings, the policy, the sources and the real harness runs. The trial runner
takes each source from its prepared Grove checkout only. The proof of this
node adds a case field that names an external checkout and its commit.

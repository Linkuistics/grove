# external-pack-contract-k155

## Goal

Settle the external-pack contract that the research move needs before any
file moves. Record it in a new Grove specification,
`docs/specs/external-pack-contract.md`. Each later pack move reads this
specification.

## Context

- `docs/specs/codex-skill-provisioning.md`, "Resources and conformance" and
  "Launch and compatibility". These sections already state the general rules.
  A reference to another pack names its skill and resource. Each pack checks
  its own rules. Aggregate conformance checks the composed loaded paths across
  packs.
- `docs/specs/owner-skill-delivery.md`, "Ravel sources" and "Pack checkouts".
  "Ravel sources" gives the layout rules and says that the research proof
  settles the exact paths. "Pack checkouts" gives the checkout path and the
  `resourceDigest` identity.
- `docs/adr/corpus-rules-have-one-owner.md`, placement rule 2. This rule puts
  each family file "in the spine". The research move puts the research family
  file in another repository.
- `docs/common-loading-acceptance-record.md`, "Research recipes". These recipes
  give the settings shape that names a source directory and its roots.
- The consumer inventory that external-research-pack-k10 made. Its commit
  message names it. The facts that bind this design are below.

Facts about the current state, from that inventory:

- The moving files are `grove-research-a/SKILL.md`, `grove-research-b/SKILL.md`,
  `grove-combine-research/SKILL.md` and the spine's `references/research.md`.
  The two research skills tell the agent to load `grove:grove` and to read
  `references/research.md` in it. `research.md` names no other skill file.
- `plugins/grove/conformance.sh` reads one skills root. It expects the spine at
  `<root>/grove`. It resolves a skill's reference in that skill's directory or
  in the spine only. It hard-codes the `static(research)` load set as
  `research-a` and `research-b`. Eight rows of `conformance/rules.tsv` have a
  moving file as owner. A row whose owner file is absent counts as pending and
  does not fail.
- The spine `SKILL.md` lists `references/research.md` as a family file.
  `TASK-FORMAT.md` states "twenty-three" kinds.
  `the_guidance_counts_the_kind_set_correctly` compares that word with the
  number of `plugins/grove/skills/grove-*` directories.
- `.claude-plugin/marketplace.json` in Grove lists three plugins by relative
  root. `plugins/install.sh` enumerates `plugins/*/skills` of the Grove
  checkout only.
- `settings_for` in `scripts/harness_trial.py` builds every source path from the
  prepared Grove checkout.

## Done when

The specification settles each question below with a decision and its reason:

- **Layout.** The repository name and location: `ResearchSkills` under
  `~/Development/AgenticOrchestration/`, colocated Git and Jujutsu, on a public
  `Linkuistics` remote. State the source directory, its roots and the kind and
  craft directories. State where the Claude Code plugin root and manifest are,
  and the plugin name, which sets the native namespace. Never use a root that
  holds build output.
- **The family procedure.** Where `research.md` lives in the pack. State how
  the three kind skills reach it when relative references resolve against the
  skill that owns them. Keep one owner and no copy.
- **References to core.** How pack text names the spine and core resources.
  Pack text must work under directory delivery and under a native plugin
  namespace.
- **Rule placement.** Rework `corpus-rules-have-one-owner.md` and any other
  affected ADR through `ADR-FORMAT.md`. A family file then lives in its
  family's pack. State the change to the spine's family-file list, to the
  `README.md` fatness rule and to the glossary entry "Family reference file".
- **The kind count.** What `TASK-FORMAT.md` and its test claim when kind
  skills live in more than one pack.
- **Conformance.** What the pack checks alone. What the aggregate check reads,
  which is core plus the supplied external packs at exact revisions. How a pack
  repository obtains Grove's checker without a copy that can drift. How the
  research rows of the manifest move or stay. Which deliberate controls must
  fail.
- **Provenance and licences.** The pack's licence, any `LICENSES/` content
  and whether the moved files keep their Git history.
- **Distribution.** The marketplace entry for a plugin in another repository.
  The manual installer route for the pack. The Codex marketplace stays
  unchanged unless the design shows a reason.
- **Checkout boundary.** The source directory and roots inside any checkout of
  the pack. The `resourceDigest` that identifies an accepted checkout.
- **Proof shape.** The case field by which the trial runner names an external
  checkout and its commit. The trial record must name that commit.
- **Evidence for later moves.** A section that the proof leaf fills with
  proven facts. Later pack moves read it before repository-moves-k5 starts.

Also:

- Reconcile `owner-skill-delivery.md` "Ravel sources" and
  `codex-skill-provisioning.md` with the new specification. Each rule keeps
  one owner.
- Where a decision changes the work of a later leaf in this node, edit that
  leaf's body in this session.
- Run the documentation checks that apply.

## Notes

This contract governs every later pack move, so weigh a `review-design` leaf
under the lazy-review rule. Use the Claude Code plugin documentation, not
memory, for the marketplace source forms and namespaces. Do not change any
skill file or create the repository in this leaf.

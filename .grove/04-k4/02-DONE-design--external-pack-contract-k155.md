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
  proven facts. Later pack moves read it before the work of repository-moves-k5.

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

## Decisions (running log)

**Layout.** ResearchSkills keeps Grove's plugin layout. The source directory
in any checkout is `plugins/research`, which is also the Claude Code plugin
root. Its kind directory `skills/` is the only Ravel root. A later pack with
craft skills adds `craft/` beside `skills/`, lists it in the manifest's
`skills` paths and adds it as a second root. The repository root holds the
licence, the README, the Taskfile and tool configuration, outside the source
directory.

**Why a subdirectory source.** Ravel captures the whole source directory
except `.git` and `.jj`, and it ignores ignore files. A subdirectory keeps
the Taskfile, caches and README out of the `resourceDigest` and out of a
working-copy capture. The path `plugins/<plugin>/skills/` also matches the
installer's link reconciliation and the trial runner's `plugin` field.

**Plugin name.** The manifest name is `research`, so the native namespace is
`research:`. It equals the Ravel source ID of the owner design. The name
`grove-research` was rejected, because it reads as the name of a kind skill.

**The family procedure.** `references/research.md` becomes the `SKILL.md` of a
family skill named `research-family`, in the kind directory. Only frontmatter
is added. `research-a` and `research-b` load it by name after the spine.
`combine-research` does not load it today, so it keeps that. A family skill
never has the `grove-` prefix, because such a name would make a kind.

**Why a skill and not a resource.** Loading a skill by name works the same way
under a native namespace and under directory delivery, and it gives the agent
the skill's directory. A resource inside a member's skill would force the
other member to load that member's `SKILL.md`. A file outside every skill has
no stable path under a native plugin.

**References.** Pack text names another skill by its bare name in the form
"Load the `<name>` skill". It writes no plugin-qualified alias, for core or
for the pack. A skill-relative path resolves in its own skill first, then in
each skill that the same file loads. A kind whose load crosses a pack boundary
points at the launch prompt's rule for a missing kind skill.

**Rule placement.** A family file lives in the pack that ships the family's
kinds. In core it is in the spine's `references/`. In an external pack it is
the pack's family skill. An edge to a family skill is a directed load by name. The ADRs
`corpus-rules-have-one-owner`, `a-skill-states-what-binds-without-its-dependencies`
and `behavioural-coverage-asserts-delivery` change in this session.

**The kind count.** `TASK-FORMAT.md` stops counting the whole set. Its table
keeps every row and gains a pack column. The test checks the core direction
and the absence of a count. The external rows have no mechanical check, and
the specification states that trade-off.

**Conformance.** The runner takes `--pack <dir>` for each external pack and
`--alone` for one pack without core. Each pack keeps its own
`conformance/rules.tsv`. The eight research rows move to ResearchSkills.
Loaded paths become skill-qualified and follow directed loads. A row whose
owner is absent fails, and so does a duplicate skill name across packs.

**The checker without a copy.** A pack's Taskfile records a Grove repository
and commit. Its check makes a plain checkout of that commit and runs that
checkout's runner. Grove's check reads each external pack's `url`, `path` and
`sha` from its marketplace entry, while Grove hosts the marketplace.

**Provenance.** The licence is Apache-2.0, as in Grove. The plugin manifest
names it. No `LICENSES/` directory exists, because the moved text has no
third-party passage. History is not imported: the first commit and the README
name the Grove commit and the four source paths.

**Distribution.** Grove's marketplace lists `research` with a `git-subdir`
source pinned by `sha`. The research manifest declares a dependency on
`grove`. The manual installer gains `--pack <plugin-dir>`. The Codex
marketplace stays unchanged, because it lists only a hooks plugin.

**Checkout boundary.** In any checkout, the source directory is
`plugins/research` with the root `skills`. A checkout is accepted by its
`resourceDigest`, which covers only that directory. The committed tree ID of
that directory is also recorded.

**Proof shape.** A trial source record gains an optional `external` object
with `repository` and `commit`. The record names the commit, the checkout
path, the tree ID and the `resourceDigest`. A run option replaces one source
with a working copy and records its commit and state.

**Publication limit.** This workstream's Grove line is not on the remote. Until
finish publishes it, a pack check that needs the new runner obtains Grove from
the local repository. discovery-rollout-k6 advances the pack's record to a
published commit.

**Review.** The contract governs every later pack move, so it earns a
`review-design` leaf. The leaf goes before pack-conformance-runner-k156, and
this session spends no in-session reviewer.

**Duplicate rule IDs.** A moved row keeps its rule ID. The aggregate therefore
refuses one rule ID in two manifests. That check finds a row that stayed in core
after its move. During a move, the first pack commit's aggregate reports each
moving skill and each moved row as a duplicate, and the removal clears both.

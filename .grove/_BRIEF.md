# grove.gh-issue-25 — brief

## Goal

Deliver [issue #25](https://github.com/Linkuistics/grove/issues/25): separate the
skill corpus into domain repositories and provide common skill discovery through
Ravel. Consolidate the four named writing projects into
`~/Development/AgenticOrchestration/`.

The human's scope corrections in `plan-k1` and delivery-boundary corrections in
`grove-skill-delivery-k8` govern this brief. The stop
command is separate [issue #26](https://github.com/Linkuistics/grove/issues/26).
Review/integration simplification is separate
[issue #27](https://github.com/Linkuistics/grove/issues/27), which investigates a
skills-only approach first. Neither is a prerequisite for this work.

## Done when

- Ravel resolves configured local skill sources and automatically adds
  discovery instructions and absolute locations to launch prompts. A missing
  required source fails clearly. Ravel does not fetch, install or update a
  source.
- The same discovery contract works in real Claude Code and Codex sessions
  without native copies of the skills. Launched sessions and their delegates
  can read the required skill resources.
- The domain packs, craft skills, personal plugin, and TestAnyware skill are in
  the destinations below. Homebrew formulae install them, as "Skill
  installation" states. Research first proves the external-pack contract.
- The four named writing projects have moved with contents, working-copy changes,
  history, workspace registrations, and affected references preserved or
  reconciled.
- Grove's existing open-kind, tree, and launch behaviour remains compatible.
  Obsolete automatic provisioning is reconciled with the common Ravel delivery
  path. Packaging, conformance, resource references, and documentation are
  current.
- Core and each external pack pass conformance. Each affected repository passes
  its applicable checks. Grove's full `bash scripts/check.sh` passes on the
  integrated result.
- Durable decisions and contracts are recorded in their owning repositories.
  Complete the independent Ravel/Grove integration and releases through the
  repository procedures, preserving every running session's original ending
  command.
<!-- tc: depart synonym-rotation because "check" occurs only as a noun that names a check suite or a check step, and "verify" is the only verb for that action -->
- The owner's full installed skill collection, including TestAnyware, is
  supplied through Ravel's common insertion route. Inventory current sources,
  resources, hooks and plugin/tool dependencies. Preserve them through the
  repository moves. Verify replacement discovery and reads before removing
  superseded native installations. Record the actual owner cutover and any
  retained resources needed by active readers. A disposable trial alone does
  not satisfy this outcome.

These are whole-workstream outcomes. This requirements session records the
contract. Design and implementation remain ahead.

During common-skill-loading-k32 the human additionally directed this grove to
do Ravel's release work and coordinate both releases. The human also directed
this grove to remove current skill installations after setting up all the
owner's skills through Ravel, explicitly including TestAnyware. The new owner
setup uses common delivery. skill-marketplace-decision-k201 later made Ravel
the only route that delivers skills, for every user. The existing isolated
test cases remain required before the actual owner cutover. Ravel publication, owner activation and removal are
now deliverables, not handoffs left for the human to perform outside this grove.

<!-- tc: depart synonym-rotation because "confirmed" reports the human's agreement, not a check or a verification -->
During coordinated-ravel-release-k36 the human confirmed that this is the only
live workstream and that all others remain paused until completion. This grove
is the sole release and owner-cutover executor. It completes any unfinished
Rhai documentation or acceptance prerequisite needed for Ravel publication in
its own Ravel workspace. It neither waits for the other grove to resume nor
edits that grove's task tree. The paused workstream later consumes actual
release and activation receipts rather than repeating its old release tasks.

The coordinated release design is `docs/specs/coordinated-release.md`, subject
to its focused review before dependent work. Ravel 2.0.0 combines Rhai and
complete directory delivery, with a distinct script/request schema 4, and
publishes first through keg-only `ravel@2`. Rollout prepares and rehearses the
release-channel and promotion tooling, and releases/verifies the versioned
dependency.

Grove's driver-created finish retains its normal one confirmation, teardown
helper, main integration, full check and minor release. Its concrete
whole-sequence proposal must additionally include promotion of the same Ravel
archives to unversioned `ravel` after compatible Grove publication and installed
verification. The proposal must also include any final owner activation
explicitly assigned by the owner-delivery design. These steps occur before the
finish signal. Incomplete promotion or assigned activation is incomplete finish,
so never signal success. Preserve the original full 1.1.0 keg and ending command
through the final signal.

## Ownership boundaries

### Ravel

One owner configuration identifies local source directories.
Ravel captures configured sources and injects skill-directory paths into the
prompt before owner policy constructs the harness command. The agent finds and
reads applicable skills. Ravel does not resolve a skill named by Grove.

The human clarified this boundary during grove-skill-delivery-k8. This boundary
replaces the earlier generated skill-catalog design. Reconcile the owning Ravel
design before implementation. Every policy must not have to remember a separate
helper.

Preserve the caller's original task and exact ending commands intact in a
separate block. This changes Ravel's current no-injection contract. Mandatory
guidance loads first. Other skills and supporting resources load when needed.
Resolve relative resources against their own skill directory.

During ravel-git-acquire-k77 the human removed pinned Git sources from the
scope of Ravel. The process of installing skills and keeping them up to date is
not part of Ravel. Ravel reads local directories only. A mechanism outside
Ravel sets the revision of each pack checkout in those directories.

owner-pack-checkouts-k79 chose mise checkouts for the owner's sources.
skill-marketplace-decision-k201 replaced them with Homebrew. Homebrew installs
each source at a published commit, and mise's `[bootstrap.packages]` runs
Homebrew. Neither Ravel nor Grove fetches a source. homebrew-skill-delivery-k215
amends `docs/specs/owner-skill-delivery.md`, which states the commands, the
missing-source behaviour and the behaviour without a network.

Account for compaction, delegates, source/resource lifetime, duplicate native
copies, and actual launch permissions. The automatic contract covers launched
sessions and their delegated agents. Independently launched harness sessions can
use the same supplied directories. Automatically configuring them is outside
scope.

### Grove

Kinds remain open labels. Grove neither declares a kind set nor resolves the
skill locations Ravel adds to the prompt. Its ordinary mandate continues to
name the required kind skill.

Homebrew installs the skills and Ravel delivers them, as "Skill installation"
states. Grove's binary installs no skill and resolves no skill location.
Preserve kind labels, leaf filenames, handles, review/integration routes, and
workflow behaviour.

Do not introduce declaration-based admission, a kind registry, generic review
parameters, or the earlier proposed Grove discovery commands. Generic discovery
is Ravel's responsibility and is invisible to Grove's task loop.

Grove's current broad Codex snapshot provisioning, packaging, embedded resources,
and tests must be reconciled with the new delivery path and moved files. Keep
any core-only fallback on the common path and replace obsolete automatic
provisioning rather than maintaining a second competing mechanism. The native
skill routes retire: no remote marketplace lists a pack, and the manual
installer links no skill.

Planning ships with core as the existing replaceable default skill. Retain the
existing producer-specific review and integration entry skills for this move.
Their simplification belongs to #27.

## Repository destinations

All new domain skill repositories below live under
`~/Development/AgenticOrchestration/` and use colocated Git/Jujutsu. Each keeps
kind skills and standalone craft skills in separate directories.

| Destination | Ownership |
| --- | --- |
| Existing `grove` | Core spine, the tree and artifact formats (task, brief, glossary, ADR and spec), finish, and planning with its review/integration skills. Shared review/integration procedures, and Grove configuration guidance. |
| New `RequirementsSkills` | Requirements, its interview procedure, and its existing review/integration skills. |
| New `DesignSkills` | Design, existing review/integration skills, and design craft. The craft skills deepen the ADR and spec formats, which stay in the spine. |
| New `EngineeringSkills` | Impl/prototype and their existing review/integration skills, plus engineering craft. |
| New `ResearchSkills` | Research pair, combiner, and family procedure. First external-pack proof. |
| New `WritingSkills` | Editorial kinds and family procedure, writing/documentation craft, and the technical-communication always-on hooks plugins for Claude Code and Codex. |
| New `PersonalSkills` | Personal skills, and the native-only `guardrail` plugin with its user-invoked hook, in their own repository. |

Design craft includes decision records, codebase/CLI design, and model-led
development. Engineering craft includes coding styles, codebase-memory use,
project simplification, and doubt-driven development. Writing craft includes
technical communication and code walkthrough authoring. Personal includes
machine environment, Jujutsu use, guardrail, and authoring conventions.
pack-membership-k171 changed the core, DesignSkills, WritingSkills and
PersonalSkills rows with the human's approval. `docs/specs/pack-membership.md`
records the reasons.

Move the TestAnyware skill to the existing
`~/Development/AbstractingComputers/TestAnyware` repository. No
marketplace-only repository exists. skill-marketplace-decision-k201 removed it
from scope.

Move `Stylepack`, `TestSubject`, `TheGreatExplainer`, and `Writegood` from
`~/Development/AgenticWriting/` into `~/Development/AgenticOrchestration/`.
Inspect each repository's actual VCS/workspace arrangement, preserve its edits
and history, reconcile paths, and verify the resulting registrations. This
authorises relocation, not changes to those tools' product behaviour.

The personal repository owns its source. Machine settings that point to it
remain under the current mise setup. The maintained manual records retirement
of chezmoi. Do not recreate that source or put personal skill sources in
`.config`.

## Skill installation

skill-marketplace-decision-k201 settled installation with the human:

- Homebrew installs every Linkuistics skill source, the owner's machine
  included. mise only declares the formulae, in `[bootstrap.packages]`.
- Grove's formula carries core and the five domain packs, at the commits that
  Grove's pack check accepted. A change to a pack reaches users through a
  Grove patch release.
- The personal pack, superpowers and quint-llm-kit have formulae of their
  own. TestAnyware's formula carries `using-testanyware`.
- Ravel is the only route that delivers skills.
- Each harness registers the hook adapters from the kegs, once, by a
  documented command. Claude Code and Codex use local marketplace files, and
  Pi links its extension. `guardrail` exists only for Claude Code.

During homebrew-delivery-design-k216 the human wrote: "My goal is to have
nothing more to do beyond homebrew install/upgrade". The human chose "Ravel
default in 2.0.0". Each skill formula installs its sources at
`share/ravel/skills/<source-id>`, and Homebrew links them into
`$(brew --prefix)/share/ravel/skills/`. Ravel 2.0.0 reads that directory when
its settings name no sources. Codex repeats its hook registration after each
Grove upgrade, because Codex stores the versioned keg path.

homebrew-skill-delivery-k215 designs and delivers this installation. Two later
pieces of work are outside this grove. Linkuistics/Ravel#4 makes Ravel a
universal skills provider. Linkuistics/grove#28 moves Ravel, the packs, Grove
and Ravel-Strategies into one repository.

## Decomposition

The ordered subtrees deliver working increments. Do not implement the two
deferred issues as prerequisites or recreate the removed kind machinery.

1. `common-skill-loading-k3`: settle Ravel's generic discovery contract and
   Grove's delivery transition. Then plan and implement the common launch path
   while sources remain here. Public local-source and lifetime fixtures and
   real Claude Code/Codex implicit-discovery trials prove this handoff.
2. `external-research-pack-k4`: move research and prove external resource
   reachability, own/aggregate conformance and common local-source
   loading. Its durable external-pack contract informs all remaining moves.
3. `repository-moves-k5`: plan separate working increments for the remaining
   domains, personal source/hooks, TestAnyware skill, Homebrew installation and
   four writing projects. Each move owns its preservation checks, affected consumers
   and references. Detailed layout follows the research proof.
4. `discovery-rollout-k6`: reconcile the final installed split, documentation,
   configuration and paused-grove handoff text. Verify all repositories and
   coordinate independent releases. Ravel release work precedes Grove's
   driver-owned finish, which performs final integration and minor release.
   This grove also executes the coordinated Ravel release and owner skill
   cutover, including removal of superseded installations. Consume the early
   coordinated-release and owner-delivery designs. Preserve a working published
   Grove/Ravel pair at every step. Account for the current driver's fixed Ravel
   1.1.0 executable before changing native discovery or shared settings.

`repository-moves-k5` closed with `moves-handoff-k187`. The section "The
handoff to the owner cutover" of `docs/repository-moves-record.md` holds its
results for rollout. Five of them matter to later work.

- The `grove-core` candidate is `2a24d5e61f29`, and the tree ID of its
  `plugins/grove` is `961fddd04bb7`. The bookmark `gh-issue-25` holds it on
  the public remote. A change to `plugins/grove` or `plugins/packs.json`
  needs a new candidate keg. core-candidate-restaging-k300 renewed it. The
  earlier candidates were `7b113a28a966` and `ef8a79dd7621`.
- The section names the published commit, the tree ID and the
  `resourceDigest` of each of the ten sources. It also names each formula and
  each difference from the earlier sections.
- It maps each owner-pack line of the baseline to its source and root, or to
  its route. It states the route of each other class.
- It names the references in paused task trees that rollout's brief-update
  text must state, such as the TestSubject path in `rhai-book-use-k74`.
- Some registrations stay at earlier roots. Stylepack has a prunable Git
  worktree registration. Writegood has Claude Code and Codex entries for
  `~/Development/Writegood`, and it has no remote.

These are subtrees within this one coordinated workstream. Later planners grow
their current implementation detail from delivered evidence. A node's design
and planning leaves do not satisfy its behavioural Done when. The node's
sessions add the missing implementation/acceptance work before allowing the
node to close. Any earned producer review is cut lazily, placed before work
that needs the settled artifact. An actionable review's integration runs next.

Design must settle Ravel's source/configuration interface, directory and source
lifetime, duplicate handling, bootstrap with missing sources, compatibility
checks, and confinement/delegation. Preserve one owner per normative rule and
reachability across source locations. These are design questions within the
requirements, not a reason to repeat the interview.

## Agreed test seams

- Ravel's public launch/resolution boundary with temporary homes/workspaces and
  local source fixtures: sources, missing-source failure, readable resources,
  prepared prompts, and intact task/ending commands.
  Include session/delegate lifetime and confinement.
- Grove's existing tree/launch checks and conformance over core and every
  external pack. Preserve current kind labels and policy routing.
- Actual Claude Code and Codex sessions: a task activates a relevant craft skill
  without naming it, and reads it before the relevant action. Isolate absence
  of native copies. Do not remove the owner's real installations.
<!-- tc: depart synonym-rotation because "fixed" in the decomposition means unchanging, not repaired, so "corrected" is the only verb for a repair -->
- Migration checks: skill/resource preservation, VCS/workspace state, corrected
  references and formula sources, and affected repositories' existing checks.

skill-marketplace-decision-k201 added six seams for Homebrew installation:

- Grove's archive manifest, `task release:smoke` and `task release:formula:test`
  show that the keg holds core and the five packs at the accepted commits,
  with executable bits intact. Ravel resolves each source from the keg's paths
  and reports its `resourceDigest`.
- `check:packs` reads Grove's own record of pack commits. It checks each pack
  alone and in the aggregate with core.
- The Claude Code and Codex trials run through Ravel, with every source at a
  keg path and no native copy.
- The owner design's staging trials and rehearsal read the sources from kegs.
- In isolated homes, each harness registers its hook adapter from a keg. On
  Claude Code, the hooks run and no native skill appears. One measurement
  covers a hook plugin across `brew upgrade`.
- Inside finish, the new local marketplace and the installation of the writing
  hooks plugin change no plugin that finish loaded. They write no orphan
  marker.

The previously agreed stop-command tests move with #26. Review-parameter tests
are not required here.

## Pointers and constraints

- `plan-k1`: running decisions and evidence. Later corrections supersede earlier
  proposals retained in its conversation record.
- `skill-marketplace-decision-k201`: the decisions of "Skill installation", in
  the human's words, and the comparison of the routes.
- `docs/adr/corpus-rules-have-one-owner.md` and
  `docs/adr/restatement-declares-its-class.md`.
- `docs/specs/codex-skill-provisioning.md`: current delivery to replace.
- `docs/specs/harness-selection-and-execution.md` and Ravel's
  `docs/specs/session-execution.md`: caller boundary to revise.
- `CLAUDE.md`, `Taskfile.yml`, and `docs/RELEASING.md`: checks and release.

Keep one coordinated workstream. The human pauses other groves before shared
skill sources change. Do not signal their sessions or edit their task trees.
Give the paused editorial and design/decomposition groves brief-update text
for their later rebase. In that text, keep craft guidance separate from kind
charters.

Do not redesign kind guidance beyond changes required for reference resolution
and delivery. The Ravel-Strategies tool-integration mechanism remains outside
scope.

No implementation, repository relocation, or owner-configuration change has
occurred in this requirements session. Issues #26 and #27 were created and #25
was narrowed at the human's request. The human confirmed this revised scope,
destinations, and test seams with "agreed". Requirements are settled. The fresh
planning session should use this contract without repeating the interview.

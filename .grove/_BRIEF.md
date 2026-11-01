# grove.gh-issue-25 — brief

## Goal

Deliver [issue #25](https://github.com/Linkuistics/grove/issues/25): separate the
skill corpus into domain repositories and provide common skill discovery through
Ravel. Consolidate the four named writing projects into
`~/Development/AgenticOrchestration/`.

The human's latest scope corrections in `plan-k1` govern this brief. The stop
command is separate [issue #26](https://github.com/Linkuistics/grove/issues/26).
Review/integration simplification is separate
[issue #27](https://github.com/Linkuistics/grove/issues/27), which investigates a
skills-only approach first. Neither is a prerequisite for this work.

## Done when

- Ravel resolves configured local and pinned Git skill sources and automatically
  adds discovery instructions and absolute locations to launch prompts. Cached
  exact revisions work offline; unavailable required revisions fail clearly.
- The same discovery contract works in real Claude Code and Codex sessions
  without native copies of the skills. Launched sessions and their delegates
  can read the required skill resources.
- The domain packs, craft skills, personal plugin, and TestAnyware skill are in
  the destinations below, and the marketplace references the source repositories.
  Research first proves the external-pack contract.
- The four named writing projects have moved with contents, working-copy changes,
  history, workspace registrations, and affected references preserved or
  reconciled.
- Grove's existing open-kind, tree, and launch behavior remains compatible.
  Obsolete automatic provisioning is reconciled with the common Ravel delivery
  path; packaging, conformance, resource references, and documentation are current.
- Core and each external pack pass conformance. Each affected repository passes
  its applicable checks; Grove's full `bash scripts/check.sh` passes on the
  integrated result.
- Durable decisions and contracts are recorded in their owning repositories.
  Complete the independent Ravel/Grove integration and releases through the
  repository procedures, preserving every running session's original ending
  command.

These are whole-workstream outcomes. This requirements session records the
contract; design and implementation remain ahead.

## Ownership boundaries

### Ravel

One owner configuration identifies local sources and exact pinned Git revisions.
Ravel resolves the sources and generates an entry document containing skill
names, trigger descriptions, and absolute paths. It automatically adds a loading
instruction and the catalog location to the prompt before owner policy constructs
the harness command. Every policy must not have to remember a separate helper.

Preserve the caller's original task and exact ending commands intact in a
separate block. This changes Ravel's current no-injection contract. Mandatory
guidance loads first; other skills and supporting resources load when needed.
Resolve relative resources against their own skill directory.

After the first fetch, reuse exact cached Git revisions offline without network
access. Fetch missing revisions; if the required revision is unavailable, report
it clearly and never substitute another revision.

Account for compaction, delegates, source/resource lifetime, duplicate native
copies, and actual launch permissions. The automatic contract covers launched
sessions and their delegated agents. Independently started harness sessions can
read the same entry document; automatically configuring them is outside scope.

### Grove

Kinds remain open labels. Grove neither declares a kind set nor resolves the
skill locations Ravel adds to the prompt. Its ordinary mandate continues to
name the required kind skill. Preserve existing kind labels, leaf filenames,
handles, review/integration routes, and workflow behavior.

Do not introduce declaration-based admission, a kind registry, generic review
parameters, or the earlier proposed Grove discovery commands. Generic discovery
is Ravel's responsibility and is invisible to Grove's task loop.

Grove's current broad Codex snapshot provisioning, packaging, embedded resources,
and tests must be reconciled with the new delivery path and moved files. Keep
any core-only fallback on the common path and replace obsolete automatic
provisioning rather than maintaining a second competing mechanism. The marketplace
remains a distribution option for standalone harness use.

Planning ships with core as the existing replaceable default skill. Retain the
existing producer-specific review and integration entry skills for this move;
their simplification belongs to #27.

## Repository destinations

All new domain skill repositories below live under
`~/Development/AgenticOrchestration/` and use colocated Git/Jujutsu. Each keeps
kind skills and standalone craft skills in separate directories.

| Destination | Ownership |
| --- | --- |
| Existing `grove` | Core spine, tree formats, finish, planning and its review/integration skills, shared review/integration procedures, and Grove configuration guidance. |
| New `RequirementsSkills` | Requirements, its interview procedure, and its existing review/integration skills. |
| New `DesignSkills` | Design, ADR/spec formats, existing review/integration skills, and design craft. |
| New `EngineeringSkills` | Impl/prototype and their existing review/integration skills, plus engineering craft. |
| New `ResearchSkills` | Research pair, combiner, and family procedure; first external-pack proof. |
| New `WritingSkills` | Editorial kinds and family procedure, plus writing/documentation craft. |
| New `PersonalSkills` | Personal skills and always-on hooks in their own repository. |

Design craft includes decision records, codebase/CLI design, and model-led
development. Engineering craft includes coding styles, codebase-memory use,
project simplification, and doubt-driven development. Writing craft includes
technical communication and code walkthrough authoring. Personal includes
machine environment, Jujutsu use, guardrail, and authoring conventions.

Move the TestAnyware skill to the existing
`~/Development/AbstractingComputers/TestAnyware` repository. Create the
marketplace-only repository at `~/Development/Delivery/skill-marketplace`.

Move `Stylepack`, `TestSubject`, `TheGreatExplainer`, and `Writegood` from
`~/Development/AgenticWriting/` into `~/Development/AgenticOrchestration/`.
Inspect each repository's actual VCS/workspace arrangement, preserve its edits
and history, reconcile paths, and verify the resulting registrations. This
authorizes relocation, not changes to those tools' product behavior.

The personal repository owns its source; machine settings that point to it
remain under the current mise setup. The maintained manual records retirement
of chezmoi. Do not recreate that source or put personal skill sources in
`.config`.

## Decomposition

The ordered subtrees deliver working increments. Do not implement the two
deferred issues as prerequisites or recreate the removed kind machinery.

1. `common-skill-loading-k3`: settle Ravel's generic discovery contract and
   Grove's delivery transition, then plan and implement the common launch path
   while sources remain here. Public local/Git/offline/lifetime fixtures and
   real Claude Code/Codex implicit-discovery trials prove this handoff.
2. `external-research-pack-k4`: move research and prove external resource
   reachability, own/aggregate conformance and common local/pinned/offline
   loading. Its durable external-pack contract informs all remaining moves.
3. `repository-moves-k5`: plan separate working increments for the remaining
   domains, personal source/hooks, TestAnyware skill, marketplace and four
   writing projects. Each move owns its preservation checks, affected consumers
   and references; detailed layout follows the research proof.
4. `discovery-rollout-k6`: reconcile the final installed split, documentation,
   configuration and paused-grove handoff text; verify all repositories and
   coordinate independent releases. Ravel release work precedes Grove's
   driver-owned finish, which performs final integration and minor release.

These are subtrees within this one coordinated workstream. Later planners grow
their current implementation detail from delivered evidence. A node's design
and planning leaves do not satisfy its behavioral Done when: its sessions add
the missing implementation/acceptance work before allowing the node to close.
Any earned producer review is cut lazily, placed before work that needs the
settled artifact; an actionable review's integration runs next.

Design must settle Ravel's source/configuration interface, catalog and source
lifetime, duplicate handling, bootstrap with missing sources, compatibility
checks, and confinement/delegation. Preserve one owner per normative rule and
reachability across source locations. These are design questions within the
requirements, not a reason to repeat the interview.

## Agreed test seams

- Ravel's public launch/resolution boundary with temporary homes/workspaces and
  local Git fixtures: sources, exact revisions, offline reuse, missing-revision
  failure, readable resources, prepared prompts, and intact task/ending commands.
  Include session/delegate lifetime and confinement.
- Grove's existing tree/launch checks and conformance over core and every
  external pack. Preserve current kind labels and policy routing.
- Actual Claude Code and Codex sessions: a task activates a relevant craft skill
  without naming it, and reads it before the relevant action. Isolate absence
  of native copies; do not remove the owner's real installations.
- Migration checks: skill/resource preservation, VCS/workspace state, corrected
  references and marketplace sources, and affected repositories' existing checks.

The previously agreed stop-command tests move with #26; review-parameter tests
are not required here.

## Pointers and constraints

- `plan-k1`: running decisions and evidence. Later corrections supersede earlier
  proposals retained in its conversation record.
- `docs/adr/corpus-rules-have-one-owner.md` and
  `docs/adr/restatement-declares-its-class.md`.
- `docs/specs/codex-skill-provisioning.md`: current delivery to replace.
- `docs/specs/harness-selection-and-execution.md` and Ravel's
  `docs/specs/session-execution.md`: caller boundary to revise.
- `CLAUDE.md`, `Taskfile.yml`, and `docs/RELEASING.md`: checks and release.

Keep one coordinated workstream. The human pauses other groves before shared
skill sources change. Do not signal their sessions or edit their task trees.
Give the paused editorial and design/decomposition groves brief-update text
for their later rebase, with craft guidance kept separate from kind charters.

Do not redesign kind guidance beyond changes required for reference resolution
and delivery. The Ravel-Strategies tool-integration mechanism remains outside
scope.

No implementation, repository relocation, or owner-configuration change has
occurred in this requirements session. Issues #26 and #27 were created and #25
was narrowed at the human's request. The human confirmed this revised scope,
destinations, and test seams with "agreed". Requirements are settled; the fresh
planning session should use this contract without repeating the interview.

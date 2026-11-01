# plan-k1

## Goal

Settle the requirements for [issue #25](https://github.com/Linkuistics/grove/issues/25):
separate the skill corpus into domain repositories, provide common discovery
through Ravel, and consolidate the four named writing projects into
`AgenticOrchestration/`.

## Context

The workspace was scaffolded with an empty root brief and this empty bootstrap
leaf. Issue #25 supplied the initial mandate. The human's subsequent corrections
govern the current scope; #25 and the root brief now reflect those corrections.

Relevant existing contracts:

- `docs/specs/codex-skill-provisioning.md`: current embedded snapshot and managed
  Codex links, including offline operation and ownership-safe repair.
- `docs/adr/corpus-rules-have-one-owner.md` and
  `docs/adr/restatement-declares-its-class.md`: retain rule ownership and
  reachability while moving family procedures to packs.
- `CONTEXT.md`: current kind grammar, stable handles, driver lease, session epoch,
  skill delivery, and the prohibition on automatic task-tree migration.
- `CLAUDE.md`: integration and release procedure; the running session keeps the
  ending command from its original launch prompt.

## Done when

- The human has confirmed a shared understanding of scope and remaining choices.
- The accepted requirements, behavior examples, and test seams are recorded.
- The repository destinations and the supported harness levels are agreed.
- A subsequent design/planning session can decompose the work without repeating
  the requirements interview.
- The leaf is retired and sealed through jj only after agreement and the required
  handoff work.

## Notes

This is a multi-session workstream. Harness discovery, pinned-source caching,
pack boundaries, and repository relocation have interdependent answers, so the
one-question-at-a-time procedure in `grove-requirements/grilling.md` applies.
The human has now confirmed the revised shared understanding. Carry the settled
requirements into a fresh planning session; the remaining questions are design
work within that scope.

### Current scope after the human's corrections

Ravel owns source resolution, the discovery catalog, and automatic prompt
augmentation. Grove's task loop does not see the added skill locations. Kinds
remain open labels; do not add a kind registry, declaration-based admission,
Grove discovery commands, or a generic review-parameter protocol.

Establish the common loading path while the skills remain in this repository.
Reconcile Grove's existing provisioning, packaging, references, conformance,
and documentation with that path. Move research first to prove the external-pack
contract. Keep launch policy separate from skill configuration and preserve one
owner per normative rule. Any core-only fallback must use the common path.

The stop-at-loop-point command is now separate
[issue #26](https://github.com/Linkuistics/grove/issues/26). Parameterised skill
simplification is separate [issue #27](https://github.com/Linkuistics/grove/issues/27),
which investigates a skills-only approach first. Neither is a prerequisite.
Retain existing review/integration entry skills and move them with their producers.

Use one repository per domain at the agreed destinations below.
Do not edit either paused workspace; provide brief-update text for the human.
Do not change kind guidance beyond reference changes. Leave room for tool
declarations without designing the Ravel-Strategies tool mechanism.

### Design work to carry forward

- Resolution lifetime for sources across sessions and delegates, to be specified
  in design without weakening the agreed exact-revision offline behavior.
- Public resolver/configuration interfaces, compatibility checks, and
  source availability for fresh bootstrap and confined/delegated sessions.

## Decisions (running log)

The human asked for unified skill discovery across harnesses, suggesting that
Grove automatically tell each harness to read a shared instruction file, as
Herdr does. The reason is central to the product: using multiple harnesses and
agents is a raison d'etre for Grove. Treat a common discovery and activation
contract as the design objective; separate native deployment for each harness
is no longer the recommended baseline.

The human agreed to the proposed common entry document and discovery contract,
including automatic discovery for Grove-launched sessions and their delegated
agents. The same entry document is available for independently launched harness
sessions to read; automatic setup of those sessions is outside the initial
guarantee. Native skill installation is not a prerequisite. The proposed
discovery acceptance exercise uses Claude Code and Codex with native copies
absent, and asks for work that should trigger a skill without naming that skill.

The human then asked whether the common facility belongs in Ravel rather than
Grove, retaining the agreed user-facing behavior. The subsequent direction is
that Ravel itself should modify the prompt to pass skill locations. This settles
the ownership boundary: generic resolution and automatic prompt augmentation
belong to Ravel. The earlier recommendation to make each owner policy opt into a
helper is superseded. Grove retains its existing workflow behavior without
resolving Ravel's added skill locations.

The human agreed that offline use after the first fetch is sufficient for Git
skill sources. Cache each exact pinned revision and reuse it without network
access. Fetch only missing revisions. If the requested revision is unavailable
locally and cannot be fetched, report the missing revision clearly; never
silently substitute another revision. Ravel owns this generic source behavior.

The human agreed that `planning` ships with Grove core as a replaceable default.
Its responsibility is to turn work into Grove's task tree. Shipping the default
does not make it a closed or privileged runtime kind or require a declaration
mechanism for replacements.

The earlier discussion accepted generic review/integration kinds with a separate
producer parameter while retaining readable filenames. The later scope correction
supersedes that proposal: #27 will investigate simplification independently,
starting with the skills themselves. Current work preserves labels and routes.

The human distinguished requirements from design and asked for separate
destinations. They are happy to consolidate the proposed requirements/design
and writing skill destinations under `AgenticOrchestration/`. Personal skills
must live in their own Git/Jujutsu repository, rather than under `.config`.
This supersedes the combined requirements/design repository and the mise-owned
personal source in the first location proposal.

The human subsequently confirmed that the existing writing projects also move:
`Stylepack`, `TestSubject`, `TheGreatExplainer`, and `Writegood` relocate from
`~/Development/AgenticWriting/` to `~/Development/AgenticOrchestration/`.
Preserve their contents and Git/Jujutsu state, and reconcile affected path
references. This is part of the authorized workstream, not a request to change
their product behavior.

The human agreed to additional test boundaries covering Ravel's public
launch/resolution interface, Grove tree commands and resolved-pack conformance,
and migration checks preserving repository state and references. These supplement
the previously agreed real Claude Code/Codex implicit-discovery trial. The
consolidated root brief is the input to the final shared-understanding check and
subsequent planning. Stop-command tests were also discussed; they now belong to #26.

The human then removed the stop command from this work and asked for its own
issue. They clarified that Grove does not resolve skills, Ravel's prompt addition
is invisible to Grove, and kinds are not declared. The earlier issue draft's
declaration and validation machinery is therefore excluded. After asking whether
parameterised reviews were necessary here, the human requested a separate issue
and suggested the simplification could be entirely within the skills. Issues
#26 and #27 have been created, and #25 has been rewritten to the narrower scope.

The human confirmed the final scope with "agreed": Ravel owns discovery and
prompt augmentation; Grove keeps open kinds and existing review routes; #25
covers the skill repositories, four project moves, and necessary delivery changes;
#26 and #27 are independent follow-ups. The repository destinations and test seams
in the revised root brief are accepted. Requirements are settled and the fresh
planning handoff can now be created.

### Agreed discovery behavior

The common facility resolves owner-configured sources, reads metadata, and generates
one entry document containing the available names, trigger descriptions, and
absolute paths. Every launch receives an instruction to read that document.
Grove's original mandate names the assigned kind skill; Ravel's generic catalog
makes its location available without interpreting the workflow. Mandatory guidance
loads first; other skills load when their descriptions match the work. Supporting
files remain relative to their skill's own directory. A location list without descriptions
and a loading rule is insufficient for task-driven discovery.

This uses ordinary file reads and the prompt channel for every harness.
Native skill installation is not required for Grove-managed discovery.
The design must still account for duplicate native copies, readable paths under
the actual launch permissions, compaction, and delegated agents. A Markdown
loading contract does not itself register tools or native lifecycle hooks;
always-on prose guidance can be included in the mandatory set.

Evidence consulted:

- [Herdr guide](https://herdr.dev/guide/): the current instruction is to read
  `~/.config/herdr-app/AGENTS.md`.
- [Agent Skills integration guide](https://agentskills.io/client-implementation/adding-skills-support):
  discovery exposes name, description, and location; a file-reading model can
  activate a relevant skill by reading its instructions.
- [Official OpenAI customization guidance](https://learn.chatgpt.com/docs/customization/overview#skills):
  skill metadata supports discovery, with full instructions and resources loaded
  when needed. This supports the proposed pattern, but does not establish equal
  trigger reliability across all harnesses.
- The current `prompt::compose` and `session_command::lifecycle` / `start`
  functions provide a Grove-owned prompt passed to Ravel's `start-session`.
  Graph verification used the workspace's index generation
  `2026-11-01T06:38:35Z`; coverage reported no recorded issue for these files,
  and exact source snippets were read. This is bounded evidence for the launch
  entry point, not a claim that every adapter already preserves the proposal.

The agreed discovery acceptance exercise asks for a relevant craft task without
naming the skill, verifies that the expected skill file was read before the
relevant action, and repeats on Claude Code and Codex with Grove's native skill
copies absent.
The other agreed test seams are recorded below.

### Ravel ownership and prompt preparation

The generic source resolver, metadata catalog, and bootstrap document belong in
Ravel. Ravel automatically adds the skill-loading instruction and resolved
locations to the launch prompt from the configured sources. Owner policy does
not have to assemble that augmentation for each harness. Grove's ordinary prompt
continues to name the required kind skill; its tree and launch behavior do not
resolve the catalog or validate kinds against source declarations. Git-source
caching follows the agreed exact-revision offline behavior above.

Recommended design constraints: prepare the augmented prompt before policy
selection, then let the existing command construction pass that prepared prompt
to the harness. Keep the caller's original task and exact ending commands intact
in a clearly separated block. Add a generic bootstrap with the absolute path of
the generated discovery document; leave skill descriptions and paths in that
document. Keep the document and resolved resources readable for the consuming
session and its delegates. Preserve the original task separately if routing or
diagnostics need to distinguish it from the prepared prompt. The precise public
interface, configuration behavior with no skill sources, and source-resolution
lifetime remain design work.

This deliberately changes the current no-injection contract in both Ravel's
specification and Grove's caller documentation. Generic skill preparation applies
to Ravel callers; it carries no Grove workflow meaning. Delivery requires Ravel
changes and reconciliation of Grove's existing provisioning, packaging, references,
and checks. It does not require new skill resolution in Grove's loop. Account for
independent releases and the running driver's original launch contract.

Existing boundary evidence:

- Ravel's `docs/specs/session-execution.md`, especially "Request, context and
  selection", assigns prompts and task conventions to callers. The request
  prompt currently reaches policy unchanged; Ravel currently injects no
  instructions. Those clauses must change for the agreed behavior.
- Ravel's worker `src/main.ts::select` invokes `policy.select`, reports its
  failures, and serializes its result. A shared policy facility is therefore a
  possible extension point in the old proposal, but per-policy augmentation was
  superseded by the human's requirement for Ravel-owned preparation.
- Grove's `docs/specs/harness-selection-and-execution.md` and
  `docs/ARCHITECTURE.md` describe the matching caller boundary.

Ravel evidence was read from
`/Users/antony/Development/AgenticOrchestration/Ravel`. Its graph generation was
`2026-11-01T07:59:38Z`; the worker source had no recorded coverage issue. That
moderate index excluded `docs/`, so the relevant specification and ADR were
read directly. No Ravel or owner-configuration files have been changed.

The exact source of Ravel's `crates/ravel/src/run.rs::run` shows request
preparation followed by policy selection, then construction of the selected
program's argument vector. `crates/ravel/worker/src/main.ts::converse` freezes
the prepared request before context loading and selection. These support placing
augmentation before policy evaluation; no harness-specific argument rewriting
is needed. Both paths had no recorded coverage issue in the same generation.

### Review simplification deferred to #27

Keep existing kinds such as `review-design` and `integrate-review-impl`, their
entry skills, filenames, stable handles, and owner-policy routes for this work.
Move each producer's review/integration entry skills with that producer's pack.

Issue #27 asks whether shared procedures and producer-specific checks can reduce
duplication entirely within skill content. Thin entry skills are an acceptable
outcome. It must establish the actual duplication and loading requirements before
proposing any binary or API changes. The earlier generic-kind/producer-parameter
proposal is not an agreed runtime requirement of that issue.

### Agreed repository destinations

Paths below are relative to `~/Development/`. New skill repositories are
colocated Git/Jujutsu repositories, including the personal repository.

| Repository or source | Contents |
| --- | --- |
| Existing `AgenticOrchestration/grove` | Core spine, tree formats, finish, default planning and its review/integration skills, shared review/integration procedures, and Grove configuration guidance. |
| New `AgenticOrchestration/RequirementsSkills` | Requirements, its interview procedure, and existing review/integration skills. |
| New `AgenticOrchestration/DesignSkills` | Design, its formats and existing review/integration skills, and design craft. |
| New `AgenticOrchestration/EngineeringSkills` | Impl/prototype and their existing review/integration skills, plus engineering craft. |
| New `AgenticOrchestration/ResearchSkills` | Research pack; first external pack used to prove the contract. |
| New `AgenticOrchestration/WritingSkills` | Editorial pack and writing/documentation craft skills. |
| Existing `AbstractingComputers/TestAnyware` | `using-testanyware`. |
| New `Delivery/skill-marketplace` | Marketplace index pointing to source repositories. |
| New `AgenticOrchestration/PersonalSkills` | Personal plugin in its own Git/Jujutsu repository. |

Keep kind packs and standalone craft skills in separate directories within a
domain repository. The agreed design-craft group is `decision-records`,
`codebase-design`, `cli-tool-design`, and `model-led-development`. Engineering
craft includes `coding-style-*`, `using-codebase-memory`, `simplify-project`,
and `doubt-driven-development`. Writing craft includes
`technical-communication` and `writing-code-walkthroughs`. Personal contains
`machine-environment`, `using-jujutsu`, `guardrail`, `authoring-conventions`,
and the personal always-on hooks.

The machine's current Obsidian `Configuration/Machine Configuration.md` says
that mise tracks machine configuration in place and publishes its history to
`AntonyBlakey/mise-setup`. It records retirement of chezmoi on 2026-11-01.
The installed machine-environment skill still names chezmoi; do not recreate
a chezmoi source. The human's explicit choice governs personal skill ownership:
their source is the separate repository above, while any machine configuration
that points to it remains managed by the machine's existing process.

The revised names above were accepted in the final shared-understanding check.
The existing `Stylepack`, `TestSubject`, `TheGreatExplainer`, and
`Writegood` repositories also move from `AgenticWriting/` into
`AgenticOrchestration/`, as explicitly requested. Preserve working-copy changes,
history, and workspace registrations; reconcile affected path references and
verify each relocated repository. No destination repository or personal
configuration has been created or changed in this requirements session.

### Stop command deferred to #26

The separate issue preserves the previously discussed lifetime and test concerns:
target a live driver instance, let its current session retire and commit, report
a distinct stop outcome, and allow plain resume without an old request carrying
over. Machine-wide discovery, races, and old-driver rollout are that issue's
design work. Do not implement them as part of #25.

### Agreed test seams

Use public command boundaries with isolated temporary homes/workspaces and local
Git source fixtures for deterministic checks:

- Ravel's launch and skill-resolution interface: exact configured source
  selection, pinned revisions, cached offline use, missing-revision failure,
  absolute readable paths, automatic prompt augmentation, and intact caller task
  and ending commands. Include the session/delegate lifetime and confinement
  behavior in the public contract.
- Grove's existing tree/launch commands and resolved-pack conformance: preserve
  open kind labels, current policy routing, stable handles, and conformance for
  core and each pack outside this checkout.
- Migration checks: verify skill/resource preservation, independent pack checks,
  source and marketplace references, and preserved Git/Jujutsu state for the four
  relocated writing projects. Reconcile path references and run the appropriate
  existing checks in each affected repository.

The real Claude Code/Codex implicit-skill trial and these additional seams are
agreed. Run the repository's full required checks on the integrated
implementation before release.

# external-pack-contract-k163

**Reviews:** external-pack-contract-k155

## Goal

Read the external pack contract adversarially before the runner, the pack and
the removal build on it. Every later pack move inherits its decisions.

## Context

- `docs/specs/external-pack-contract.md`, the artifact under review. Its views
  are under `docs/design/external-pack-contract/`.
- The three reworked records: `docs/adr/corpus-rules-have-one-owner.md`,
  `docs/adr/a-skill-states-what-binds-without-its-dependencies.md` and
  `docs/adr/behavioural-coverage-asserts-delivery.md`. A preparatory commit
  converted each one to controlled English first, so compare the rework with
  that commit, not with the older text.
- The reconciled sections: `docs/specs/owner-skill-delivery.md`, "Ravel
  sources" and "Pack checkouts", and `docs/specs/codex-skill-provisioning.md`,
  "Resources and conformance".
- The producer's decision log in its task file. The commit of
  external-pack-contract-k155 names its handle.
- The current runner `plugins/grove/conformance.sh`, its manifest and the four
  moving files under `plugins/grove/skills/`.
- Ravel's `docs/specs/skill-discovery.md` in
  `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`, for
  capture, roots and the loading rules.
- The Claude Code plugin documentation, for the marketplace sources, the
  manifest `skills` paths, `dependencies` and namespaces. Read it, not memory.

## Done when

- Test the edge syntax. Can the runner read "Load the `<name>` skill" without a
  false edge or a missed edge in core's text and in the moved text? Does rule 3
  of "References between skills" keep the meaning of core's existing paths?
- Test the family skill. Look for a case where a skill fails and a resource or
  another arrangement works. Check the claims about the `grove-` prefix and
  `disable-model-invocation`.
- Test the conformance design. Does each control fail for the stated reason,
  and can a broken runner pass the clean readings? Does the change from a
  pending row to a failure break a check that should pass?
- Test the records. Is the marketplace `sha` a sound record for Grove's check?
  Is a network fetch inside `scripts/check.sh` acceptable? Does the pack's
  `GROVE_COMMIT` create a cycle of pins?
- Test the missing-skill sentence against the too-late test and against the
  launch prompt's actual wording.
- Check each reworked record and each reconciled section for a rule with two
  owners. Look also for a statement that the contract contradicts.
- Produce findings only. If a finding is actionable, cut an
  integrate-review-design step immediately before pack-conformance-runner-k156.

## Notes

The producer accepted one gap on purpose: no mechanical check covers the
external rows of `TASK-FORMAT.md`'s kind table. Challenge that trade-off if a
cheap check exists that keeps kinds open.

The Grove line of this workstream is not on the remote. The contract states the
consequence for a pack check before finish. Check that statement against the
later leaves.

## Findings

The reviewed change is `872faec9` (external-pack-contract-k155), against its
controlled-English preparatory parent `ec33e3bd`. These findings concern the
contract and its handoffs. No production, test or design file was changed by
this review.

### F1 — Define ownership for families that span packs (high)

The reworked placement rule has no result for a family whose kinds ship in
several packs. In `docs/adr/corpus-rules-have-one-owner.md:21-26`, rule 2 puts
the family file in "the pack that ships the family's kinds". Its rejected
option at lines 182-188 treats a family spread over several packs as a future
reopen condition. That condition is already part of this workstream's accepted
scope: `.grove/_BRIEF.md:150-153` keeps planning's review and integration in
core, moves the other review and integration entries to their domain packs,
and explicitly keeps the shared review and integration procedures in core.

Consequently the next migrations cannot obey both the root contract and the
new placement function. Moving `review.md` into each domain would duplicate
its rules; choosing one domain as its owner would contradict the stated core
ownership. The contract's "Rule placement" section inherits this problem.
Define the owner of a family shared across packs now, and reconcile the ADR,
the specification and the later-move guidance with that choice. The research
pair can still own its family procedure in ResearchSkills.

### F2 — Put publication of the checker pin in an executable order (high)

`docs/specs/external-pack-contract.md:499-505` says that the needed Grove
commits stay unpublished until finish, acknowledges that finish's rebase
changes their IDs, then assigns discovery-rollout-k6 the advance to a
published Grove commit. Rollout precedes finish. It cannot select a commit
that, under this rule, only becomes public after rollout has ended.

The affected checks are concrete. research-pack-repository-k157 pins the
runner commit. grove-research-removal-k158 then publishes a second pack
commit that pins the removal commit. Both need a local `GROVE_REPOSITORY`
override, so a fresh checkout using the committed public URL cannot run the
pack's advertised `task check`. external-pack-proof-k162 requires checks of
that published pack before the later migrations. The owner-delivery design
already permits a published candidate on the workstream's line
(`docs/specs/owner-skill-delivery.md:236-242`), which differs from the new
contract's blanket publication limit.

Choose one consistent publication policy. For example, publish and retain a
stable checker/core candidate before publishing a pack pin that needs it, or
assign the final pin repair and clean public-URL check explicitly to a step
after Grove publication. Name how that policy satisfies the earlier research
proof. The two-way pins do not themselves require a hash cycle: the pack's
source directory can stay identical while its Taskfile pin changes. The
missing part is the publication and verification order.

### F3 — Distinguish an external resource from a missing local resource (medium)

The pack-alone contract is inconsistent for a skill that reads a core
resource. `docs/specs/external-pack-contract.md:394-409` permits an unresolved
external directed load, forbids reading core in this mode, but still requires
every skill-relative path to resolve. Rule 3 at lines 235-238 can resolve a
path through a loaded skill only when that skill's files are available.

The accepted next moves supply a real counterexample. A moved
`grove-review-design/SKILL.md` loads `grove` and reads
`references/review.md` from it (current lines 9-10). The resource belongs to
core under the root brief. A DesignSkills-only run must either fail this
valid dependency or silently stop enforcing the promised resource check.
The current research text avoids this case, so a research-only fixture would
not reveal the gap.

Define how a pack identifies the owner of an external supporting resource
and what the alone report defers to aggregate conformance. Keep a missing
pack-local resource a failure. Add both outcomes to the contract's controls,
using a member that actually names a resource in core. Do not require a
second copy of core's resource to make the alone check pass.

### F4 — Migrate the control fixtures when absent owners become failures (medium)

`docs/specs/external-pack-contract.md:439-441` makes an absent owner fail, but
the existing clean fixtures intentionally contain only part of core.
`plugins/grove/conformance.test.sh:43-61` copies the spine and selected kinds
while keeping the entire default manifest. Its clean-spine case at lines
90-91 therefore omits owners such as `grove-impl/SKILL.md`. With the proposed
rule, that clean control fails even when the runner is correct.

The control helper also does not establish that a dirty run failed.
`expect_dirty` at lines 77-85 accepts any output containing its needle, and
`run_runner` discards the exit status. A runner that still prints the expected
diagnostics but incorrectly exits zero would satisfy these dirty controls
and the clean readings. The new contract's requirement to see each control
fail therefore needs more than this helper's current assertion.

Specify the fixture migration in the runner handoff: supply complete fixture
packs or manifests whose owners all ship, establish each fixture's clean
baseline, and assert the failing status and intended diagnostic after its
one mutation. Preserve the old behavioral cases when adding the external
ones. This keeps the absent-owner rule without making unrelated failures
stand in for the controls it is meant to protect.

## Inspection notes and limits

- The canonical edge wording matches core's current opening imperatives.
  The proposed pair opening instead says "Then load the `research-family`
  skill" at specification line 269. The implementation needs an explicit
  decision about case and Markdown normalization, with that exact opening
  as a clean fixture. An exact case-sensitive match of the stated form would
  miss the family edge. Comments, quotations and examples should not become
  required loads merely because they contain the words.
- The kind-table gap has a bounded alternative. An aggregate check can verify
  rows for a supplied, documented pack while accepting additional kinds that
  have no core table row. That would preserve open kinds. It would need a
  mapping from the table's repository names to supplied packs; the current
  runner has no such input. This is a smaller trade-off than the specification's
  claim that any aggregate assertion must reject third-party kinds, but it is
  not an additional blocking finding.
- The family skill gives the pair one named owner without giving it the
  `grove-` prefix that the current checker treats as a kind. Omitting
  `disable-model-invocation` is consistent with Claude Code's model-loading
  rules. The claim that native plugins offer no stable resource base is too
  broad: the official skills documentation describes `${CLAUDE_PLUGIN_ROOT}`.
  A resource arrangement would still need a separate portable rule for the
  manual route, so this fact alone does not invalidate the family-skill choice.
- The marketplace `git-subdir` source, its `sha`, additional `skills` paths,
  plugin namespaces and the bare `grove` dependency agree with the current
  official [marketplace reference](https://code.claude.com/docs/en/plugins/marketplace-reference),
  [manifest reference](https://code.claude.com/docs/en/plugins-reference) and
  [dependency documentation](https://code.claude.com/docs/en/plugins/dependencies).
  The [skills documentation](https://code.claude.com/docs/en/skills) supports
  the loading observations above. These are documentation checks, not native
  acceptance results. Fetching pinned content in `check:packs` is an explicit
  first-run network dependency, not a fetch by Ravel.
- The missing-dependency sentence has a target in the reviewed source:
  `crates/grove-loop/src/prompt.rs:113-130` supplies the missing-kind report,
  live-leaf and exact force-stop rule. Putting the extension in the loaded
  kind keeps it outside the too-late prompt boundary. This session's inherited
  Grove 22.8 launch predates that wording; it is not evidence for the new
  mandate. The native proof must use and record the intended compatible Grove
  build, and this session retains its original ending command.
- Inspection used Tier 2 graph evidence at generation
  `2026-11-04T02:19:35Z`, exact source snippets and direct document reads.
  Coverage reported partial parses for the four design `.mmd` files and
  `plugins/install.sh:188-447`; those ranges were read directly. The manifest
  TSV had no tracked freshness record and was read directly. Coverage is a
  best-effort signal, not a completeness proof. No test, build, lint or format
  command was run, as the review procedure requires.

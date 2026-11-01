# ravel-rhai-skill-delivery-k49

**Reviews:** ravel-rhai-skill-delivery-k34

## Goal

Read the revised Ravel directory contract adversarially before Grove's
adoption design and local planning consume it. Produce findings and no edits.

## Context

The artifact is Ravel commit `fc87cb41a422bf2623ce362ef689c38ddf98a506` on the
local bookmark `skill-directory-delivery`, in the jj workspace
`Ravel.skill-directory-delivery`. Read its diff against the channel contract
`2a77ffbd`. It adds `docs/specs/skill-discovery.md`, its evidence, two decision
records and the views in `docs/design/skill-discovery/`. It changes the
configuration and session specifications, `CONTEXT.md`, `Taskfile.yml` and
`ravel-supervises-the-session`.

Compare it with Ravel `main` `24a54523` and its source, not with memory. The
historical design `b93bb927` and the Grove reviews `ravel-skill-directories-k17`
and `-k18` supply the earlier reviewed intent. The consumers are
`docs/specs/coordinated-release.md` and `docs/specs/owner-skill-delivery.md` in
Grove.

## Done when

- Challenge the refusal of a schema-3 script with delivery disabled, against
  the schema-4 test of the coordinated release design. Weigh its cost for
  Ravel's examples, book and the owner's `config.rhai` conversion.
- Check the derivation of the evaluate-frame bound against `frame.rs`,
  `serve.rs`, `choice.rs` and `inputs.rs`. Look for a field that the formula
  misses or undercounts. Check that a refusal after the worker launch keeps
  the teardown and failure-precedence contract.
- Challenge the decision that harness runs during evaluation receive
  `RAVEL_SKILL_DIRECTORIES` and the skill read grants.
- Challenge the unchanged worker protocol, notice schema and failure schema.
  Challenge the refusal of a repeated member name in the whole settings file.
- Check that each rule has one owner across the skill discovery, configuration
  and session specifications. Check that no historical reviewed rule was lost.
- Check that each of the eight requirements of the owner-delivery design maps
  to a stated rule. Check the sandbox home, including the unconfirmed Linux
  Claude Code sign-in path.
- Check the stage handoff and the consumer handoff against the local, pinned
  and adoption leaves in this tree.
- Record the findings as the review procedure requires. Cut an integration leaf
  only if a finding needs action.

## Notes

Inspection only. Edit no Ravel or Grove artifact, and push nothing. Do not edit
the `Ravel.gh-issue-2` workspace or its task tree.

## Decisions (running log)

- Review the committed Ravel artifact `fc87cb41a422bf2623ce362ef689c38ddf98a506`
  against `2a77ffbd`, with source at `main` `24a54523`. Keep implementation
  and all post-fix checks with the integration and implementation leaves.
- The sandbox-home contract and the frame-bound control each need a finding.
  The schema transition, retained resources and inherited delegate authority
  remain visible costs rather than reasons to redesign this contract.
- Cut an `integrate-review-design` sibling before the adoption design consumes
  this review. Give it this handle, not a transcribed list of corrections.

## Findings

The locations below are in Ravel commit `fc87cb41`, in the
`Ravel.skill-directory-delivery` workspace. Both findings warrant integration.
Neither establishes a defect in implemented directory delivery: that feature
has not been implemented.

### 1. The sandbox leaves Claude Code's configuration override uncontrolled (medium)

Location: `docs/specs/configuration-evaluation.md:769-792`, especially the
Claude Code rows and the environment check at lines 789-792. Item 3 of the stage
handoff in `docs/skill-discovery-design-evidence.md` inherits this technique.

The recipe promises that the sandbox owns Claude Code's settings, plugins and
skills. It checks `CODEX_HOME`, but gives no corresponding rule for
`CLAUDE_CONFIG_DIR`. An unconfined harness run inherits ordinary caller
variables. `rhai_worker/harness.rs`, `execute`, scrubs `LAUNCH_CONTROLS`, whose
list in `environment.rs` does not include that Claude Code variable. The smoke
wrapper in `scripts/harness-smoke.sh`, `sandboxed`, does not remove it either.

Claude Code documents that `CLAUDE_CONFIG_DIR` selects its settings, history
and plugins. It also changes the credential-file location on Linux and the
Keychain entry on macOS. See the official
[environment reference](https://code.claude.com/docs/en/env-vars#variables)
and [credential management](https://code.claude.com/docs/en/authentication#credential-management),
read on 2026-11-02. These sources describe the external interface; no harness
trial ran in this review.

For example, a caller can inherit `CLAUDE_CONFIG_DIR` pointing at its real
`.claude` directory. Changing `HOME` then leaves Claude Code using that real
configuration. Every sandbox link check can pass while native plugins remain
available and a configuration write reaches the real directory. Such a run
cannot prove discovery with native copies absent. Linking the default Linux
credential file also need not supply the sign-in of the selected account.

The smallest useful correction is to control this override in the owning
sandbox contract, record the effective Claude Code configuration directory and
give an inherited real-directory value an explicit refusal or sandbox
replacement. Removing the override retains the default-path recipe; setting
a new override also needs an explicit credential-reuse rule. Require a
deliberately inherited override in the recipe's isolation controls before
actual trials rely on it. Keep the local stage's check of the installed Linux
credential path: the published default is not a measurement of that build.

Under `codebase-design`, this is ambient configuration crossing the trial's
state boundary. The trial wrapper is the enabling point and should own the
effective harness home. It needs no new product setting or source resolver.

### 2. The one-byte frame control assumes a tight bound that the formula does not give (low)

Location: `docs/specs/configuration-evaluation.md:434-436`, read with the
derivation at lines 405-411.

The specification requires a maximum-input request to fit and a control that
reduces the frame bound by one byte to fail. The proposed constant is a
conservative sum, with a whole MiB for the envelope and worst-case allowances
for each independent field. It is not the encoded size of a particular
request. The original prompt and prepared prompt also cannot independently
fill every allowance while preserving their required relationship.

Consequently, a request that fits with spare capacity can still fit after
38,541,312 becomes 38,541,311. That result says nothing about whether the
size check is active. Conversely, forcing that request to fill the constant
would mix this check with the separate, unbounded-kind case. The literal
control therefore cannot establish the claimed public-bound coverage.

Keep the conservative bound. State separately that worst-case bounded inputs
must fit, and that an overflow control sets the effective limit below the
actual encoded size of a known frame, or constructs a frame at a deliberately
reduced test limit and crosses it by one byte. Exercise removal of the check
at its internal enabling point. The CLI remains the observable seam; no
extra public flag is needed.

Under `codebase-design`, the seam is adequate, but the proposed observation
does not distinguish the correct implementation from a disabled check. This
finding concerns the acceptance contract, not a supported undercount in the
production formula.

## Other challenged decisions

**Schema 4 with delivery disabled.** The refusal is consistent with the
coordinated-release test. It prevents an unchanged schema-3 script from
receiving different prompt semantics after a settings edit. The preparation
ADR records the rejected compatibility option and its reopen condition.
Schema 3 is unreleased. The local-stage handoff assigns the migration of
examples, fixtures, book material and release notes; the owner conversion
targets 4. Keeping the refusal avoids a second request adapter, at that
migration cost.

**The frame derivation and failure path.** `choice::rhai_request` and
`serve::serve` show that the caller context appears in the request and again
as `selectContext`. `context::deliver` bounds the latter after adding measured
sources. The derivation accounts for those two encoded values, both prompts,
the entry source, task ID, parameter content and punctuation, and the
descriptor. The 19 MiB harness-reply calculation is smaller. I found no
supported missing bounded field in this comparison. The new encoded-size
check remains necessary for the unbounded request strings.

The oversize refusal occurs after worker launch because the deadline covers
the entry read. It must enter the existing conversation-refusal path in
`worker::supervise`, rather than return around that supervisor. That path
closes the channel, attempts bounded stop/reap and private-directory cleanup,
retains secondary failures and lets cancellation win. The completion spec's
"Worker teardown before admission" still owns those rules. Implementation
must retain that precedence for `malformed_input`; the review establishes no
successful reap or signal behavior of the unimplemented path.

**Harness runs during evaluation.** Giving these runs the descriptor variable
and the same skill reads is consistent with their existing managed-process
environment contract. It adds no caller-side helper. It does mean that passing
`originalPrompt` to a classifier omits the preamble without isolating that
classifier from skill resources. The specification states the environment
scope explicitly. Neither the reserved variable nor a read grant supplies
ending authority; each harness run already has its own result channel.

**Other schema numbers and repeated settings members.** Worker protocol 1 can
remain: `verify_hello` checks package, build and Rhai identities, and
`embed_crate_digest` hashes the crate sources. A changed request implementation
therefore changes the identity even without a protocol-number change. Context
schema 1 is unaffected. Notice and failure schema 2 are the first published
2.0 shapes, and the Grove consumer handoff says it parses neither document.
The stricter duplicate-member rule replaces the current JSON map's silent
value selection. Applying it to the whole settings file gives one parser one
rule; its compatibility cost is explicit and is not a reason to add a second
parser for `skills`.

**Ownership, records and historical intent.** The discovery spec owns sources,
loading and grants; configuration evaluation owns the request transport,
frames and sandbox recipe; session execution points to those owners. The
preparation and retention ADRs cover independent decisions, and the existing
supervision ADR carries delegate authority. I found no need for another ADR.
The inspected historical contract `b93bb927` retains its local freshness,
exact Git/offline, immutable publication, resource lifetime, mandatory order,
explicit-loader, protected-origin, compaction and delegation rules here. The
TypeScript freeze, host and protocol assignments have been replaced explicitly
by Rhai copies, measured reads and the reviewed schema decisions.

## Owner requirements and handoffs

The eight items in Grove's `docs/specs/owner-skill-delivery.md`, "Requirements
for the Ravel directory contract", map as follows:

| Item | Rule in the Ravel artifact | Review result |
| --- | --- | --- |
| 1. Ten sources and narrow Git subdirectories | `skill-discovery.md`, Settings, Exact Git acquisition and Bounds | Sixteen sources; full pins and `subdir` are explicit |
| 2. Pinned mandatory file and relative supporting reads | Settings, Sources and retained resources, The preamble | Source-relative mandatory paths and supporting reads remain inside the captured source |
| 3. Descriptor and roots visible to the script | Directory descriptor and Request and configuration script | `request.skills` exposes the descriptor path and each `snapshotRoot` |
| 4. Executable bits | Publication, concurrency and cleanup | Executable versus nonexecutable is preserved; this grants no execution capability |
| 5. Warm offline use without Git | Exact Git acquisition and offline reuse | Checked hits require neither Git nor a network operation |
| 6. User-invoked guidance | Agent discovery and native installations | The preamble carries the `disable-model-invocation` rule, and actual trials cover it |
| 7. Exact 1.1 settings refusal | Configuration readers | Unknown 2.0 keys refuse at `cli`, exit 2; `end-session` reads no settings |
| 8. Sign-in-only sandbox | Configuration evaluation, Sandbox home for real harness runs | Narrow links and separate `CODEX_HOME` are stated; finding 1 remains |

The evidence's stage handoff agrees with the local and pinned briefs and
planners: the local stage owns the public local route and temporary Git
refusal; the pinned stage removes that refusal; both rebase and record exact
commits at entry. The line remains unpublished through actual acceptance.
Rollout owns final integration and release. The adoption leaf consumes the
reviewed commit before local planning, and keeps the native 1.1 route and its
canonical ending executable separate. No change to those owners is proposed.

## Evidence and limits

- Read the complete discovery contract, the producer's diff against
  `2a77ffbd`, the changed configuration and session sections, the three ADRs,
  evidence and four diagram sources. Read the historical `b93bb927` contract
  and the k17/k18 review and integration. Read the Grove consumer contracts,
  brief chain and local, pinned and adoption handoffs.
- Used Verify-tier graph discovery in
  `Users-antony-Development-AgenticOrchestration-Ravel.gh-issue-2`, generation
  `2026-10-29T08:59:41Z`. The `serve` trace completed at depth one. Several
  edges are heuristic; direct source supplied the material conclusions.
  Coverage reported matching metadata and no recorded gap for the inspected
  source. Documentation and scripts are excluded there, so direct reads of
  the target artifact supplied them. No file or tree was edited in that
  workspace. The Ravel feature workspace itself is not an indexed project.
- `jj diff` from `24a54523` to `fc87cb41` reports no change under
  `crates/ravel/src`. The feature workspace is clean and its parent is the
  named artifact. Direct byte comparisons also matched all 13 inspected
  source files in that workspace to their counterparts in the indexed
  workspace: `choice`, `context`, `frame`, `inputs`, `worker`, `environment`,
  `confinement`, `limits`, `settings`, and the Rhai `serve`, `front`, `harness`
  and `engine` modules. These observations identify the inspected source;
  they do not establish product behavior.
- Grove graph generation `2026-11-01T21:50:42Z` reported matching metadata and
  no recorded gap for the checked consumer specifications, brief chain and
  producer/review task paths before this review was written. A clean coverage
  report is not proof of completeness.
- No test, build, lint, format, product launch or real harness trial ran. No
  production file, test, owner setting, installed skill or Ravel task tree was
  changed. The producer's recorded document checks remain its evidence, not
  checks rerun by this review.

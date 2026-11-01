# common-skill-loading-k31

**Reviews:** common-skill-loading-k9

## Goal

Review the staged common-loading decomposition before implementation depends on
its cross-repository handoffs. Find missing behaviour, unusable stage boundaries
or a scope that cannot fit the proposed focused sessions.

## Context

Read the producer's commit by its handle and the current common-loading brief
and new subtree briefs/tasks. Contracts are Grove's delivery specification and
evidence at adoption commit `cf3bf6ec`, and Ravel's directory specification and
evidence at integrated commit `b93bb927`. Earlier catalog design is historical.
The root owns requirements and exclusions. This review changes no product or
plan. Its artifact is findings under the review-planning procedure.

## Done when

- Check whether native decoupling, local delivery, pinned delivery, dual-major
  caller adoption and common acceptance each leave useful verifiable behaviour.
  Challenge the first writer-removal leaf's scope and any schema/reader split.
  Check whether the transient local-only build can fail required unsupported
  sources clearly without inventing public configuration or claiming completion.
- Trace every parent behavioural criterion and both specifications' acceptance
  obligations to an owning stage. Distinguish fixtures from actual harnesses,
  local/native lifecycle from confined read access, and unprompted recovery from
  a retained path. Check deferred detail has a precise planner and live work.
- Challenge isolated pre-publication build/settings/policy handoffs, retained
  reader resources and the research prerequisite. Check that publication ordering
  remains owned by rollout and cannot silently break the existing tap dependency.
- Check documentation/source-exact books, manual-cleanup guidance, existing
  open-tree behaviour, installed checks and owning-repository commits accompany
  their behaviour. Do not require moving packs or solving issues #26/#27 here.
- Record actionable findings with source/contract evidence and their practical
  consequence. Cut integration only if warranted, next before the first live
  implementation subtree. Its body names this review rather than copying findings.

## Notes

The planner used no competing in-session review. Current provider evidence was
absent. Do not invent a Creator line. Review the committed decomposition against
current source, and state any limits on inspection or measured mechanisms.

## Findings

Each finding names its location, the failure it allows, the practical
consequence, a repair and a severity. High means that a stage cannot meet
its brief as written, or that a required order has no real owner. Medium means
that a session can act on two readings, or that one session cannot hold the
scope. Low means a gap that a later planner can close without a tree change.

Findings 1 and 2 are high. They come from work outside this tree that the plan
does not mention. Findings 3 to 6 apply even after the integration repairs
findings 1 and 2.

### 1. The Ravel stages target a design that Ravel `main` superseded (high)

Location: the `ravel-local-delivery-k23` brief, Done when 2 and Notes.
`ravel-local-delivery-k24`, Done when 1 and 4. The `ravel-pinned-delivery-k25`
brief. The `grove-dual-ravel-k27` brief, Done when 1 and 2. The common-loading
brief, Notes, "Ravel 2/schema 3 remains the feature target".

Evidence: the integrated directory design `b93bb927` descends from the Ravel
v1.1.0 release commit `2095cbd879c9`. Ravel `main` is `24a54523`. It holds 107
later commits that are not in the design line, with 365 changed files. Those
commits are the `Ravel.gh-issue-2` workstream.

That workstream replaces the TypeScript configuration script with embedded
Rhai. It removes the Bun worker and the TypeScript SDK. It renames the entry
to `config.rhai` and the settings key `policyEnv` to `configEnv`.

Ravel `main` `docs/specs/configuration-evaluation.md` already defines request
`schemaVersion` 3 with `prompt` as "The caller's prompt, unchanged". It has no `originalPrompt` or `skills` field. Its hook is
`load_context`, not `loadContext`. Its worker handshake checks protocol 1.

`docs/specs/skill-discovery.md` exists only on the design line, not on `main`.
The Ravel leaf `.grove/21-impl--release-ravel-2-0-0-k37.md` on `main` releases
this Rhai change as Ravel 2.0.0.

The directory design claims Ravel 2.0, request schema 3, worker protocol 2, a
TypeScript SDK, `policy.ts` samples and `host.readText`. Ravel `main` assigns
each of those identities differently or removes them. The local stage tells its
sessions to "keep final configuration/API shapes intact" and to migrate a
"matching front/worker/SDK". The local planner names Ravel `typecheck` and
"SDK/sample migration". The dual-major stage admits "stable 2.x from 2.0.0" as
the directory route, and it asks for "schema-2/schema-3 SDK examples".

Failure: the local planner can follow the brief in two ways, and both fail. If
it builds on the design line, it extends a branch 107 commits behind `main`
that cannot integrate without a redesign. If it builds on `main`, it must
contradict the "final API shapes" rule. Grove's dual-major stage would then
admit Rhai Ravel 2.0.0, which has no directory delivery, as the directory route.

Consequence: no Ravel implementation session has a valid base. The tree
plans three stages and the Grove caller adoption on version and schema
identities that are no longer free.

Repair: before `ravel-local-delivery-k24` runs, cut a design repair of the
directory contract against Ravel `main` and its review. It must settle the
feature's version, for example a 2.x minor or 3.0. It must settle the request
schema number and fields, and the descriptor read through the Rhai host
capabilities. It must settle the place of `skills` in the settings file beside
the new Rhai keys, and the frame bounds of the Rhai worker.

Then repair the Grove delivery specification, its evidence and the stage
briefs that name "Ravel 2", "schema 3" or the SDK. The parent brief already has the rule: an
unclosed public decision earns a design leaf before dependent work.

### 2. Another workstream will advance the tap's unversioned `ravel` before Grove admits it (high)

Location: the `grove-dual-ravel-k27` brief, Notes. The common-loading brief,
Notes, "Publication must preserve compatible tap dependencies at every
intermediate state". `discovery-rollout-k6`, which the plan makes the owner
of publication order.

Evidence: Grove's `scripts/templates/grove.rb.tmpl` line 25 depends on
`linkuistics/taps/ravel` with no version bound. Its test asserts
`/^ravel 1\./`. It copies `sample/policy.ts` into the owner's configuration.
Grove's `crates/grove/src/dispatch/ravel.rs` admits only `ravel 1.x` from
1.1.0, and its remedy is `brew upgrade ravel`. The shipped caller assets
`assets/ravel-policy/grove.ts` and `sample/policy.ts` use the TypeScript SDK.

In `Ravel.gh-issue-2`, `.grove/21-impl--release-ravel-2-0-0-k37.md` runs
`task release:major`, which updates the tap formula. Its sibling
`22-impl--homebrew-upgrade-k38.md` makes `/opt/homebrew/bin/ravel` report
2.0.0. Ravel 2.0.0 refuses `policy.ts` as `config_format_retired`.

A search of this tree, Grove's specifications, ADRs and top-level documents
finds no reference to Rhai or to that workstream.

Failure: the plan puts Grove's admission of a second Ravel major after both
Ravel stages, and it puts the tap order in rollout. The Ravel workstream does
not read this tree. It can release Ravel 2.0.0 at any time. The design's
mitigations, Grove admission first or a versioned Ravel formula, are then
already too late.

Consequence: after that release, a fresh `brew install grove` or an upgrade
pairs Grove 22.8 with Ravel 2.0.0, and every launch refuses. The remedy that
Grove prints repeats the upgrade that caused the refusal. The owner's own
driver for this grove can refuse at its next restart after the 1.1.0 keg goes.
Grove's shipped sample policy cannot run under the new Ravel.

Repair: the integration needs a human decision, because two workstreams
share one release order. Give the human the options.

Grove can ship admission of Rhai Ravel 2.x with Rhai caller assets and
formula tests before the tap advances. Ravel 2.0.0 can publish under a versioned formula. Or the human can
hold the Ravel release. Then move the matching Grove compatibility work ahead
of directory delivery in this tree, because it no longer depends on that
feature. File the cross-repository item through the Ravel repository's own
channel. Do not edit that workstream's tree.

### 3. No stage says where the Ravel commits integrate (medium)

Location: the `ravel-local-delivery-k23` brief, Done when 4.
`ravel-pinned-delivery-k26`, Done when 5. The common-loading brief, Notes,
"leave discoverable commits".

Evidence: the design commits `b68af6e4` to `b93bb927` are on a side line from
v1.1.0. They are not on Ravel `main`. Ravel's release tasks run from its
default colocated workspace and release `main`. A second workspace,
`Ravel.gh-issue-2`, holds a live grove.

Failure: the stages require "owning commits" and "reproducible build inputs",
but they do not say which base the commits use, or when the commits land on
Ravel `main`. A local-only build that lands on `main` before the pinned stage
is incomplete can go into an unrelated Ravel release. A build that does not land
grows further from `main` at each stage.

Consequence: one of two failures follows. The release of an incomplete
discovery contract is the first. A late, large rebase across a moving `main`
is the second.

Repair: state in the local stage brief the base revision of each Ravel
stage. State whether each stage lands on Ravel `main` with its delivery off,
or stays on a named bookmark until rollout. Name who rebases the stages, and
when. The repair of finding 1 sets the base.

### 4. The first writer-removal leaf holds more than one focused session (medium)

Location: `grove-native-provisioning-k21`.

Evidence: the leaf removes `crates/grove/src/provision.rs` (401 lines) and
`crates/grove/build.rs` (88 lines). It replaces assertions in
`tests/skill_provisioning.rs` (335 lines) and `tests/lifecycle_cutover.rs`
(1109 lines). It also changes `check:skills`, which names
`--test skill_provisioning`. Provisioning wording occurs in about 40 files,
including `grove-llm` and `grove-loop` tests, `docs/ARCHITECTURE.md`,
`docs/RELEASING.md`, `plugins/README.md` and the walkthrough books.

The leaf also writes the manual cleanup procedure in `docs/USAGE.md`. It exercises
that procedure in disposable legacy layouts with seven classes of case.
Then it runs the full check.

Failure: the leaf states "Decompose if necessary", so a session that runs out
of room decomposes in the middle of the work. The cleanup procedure and its
exercise are documentation and test work that do not change the writer. The
delivery specification binds the procedure to "the increment that removes
provisioning". No release occurs between two leaves of `grove-native-delivery-k20`,
so a following sibling leaf in that node keeps that rule.

Consequence: one session must carry both the code removal and the cleanup
exercise. Either part can be left half-verified when the session stops.

Repair: cut the manual cleanup procedure and its disposable-layout
exercise as a sibling leaf directly after the writer removal, in the same
node. Keep in the writer-removal leaf the documentation that its removal makes
false. Keep the node Done when unchanged.

### 5. The native-route lifecycle proofs do not say fixture or actual harness (medium)

Location: `grove-native-provisioning-k21`, Done when 2.
`grove-skill-mandate-k22`, Done when 3.
`common-loading-acceptance-k30`, Done when 2.

Evidence: the delivery specification's "Offline and failures" row says that
"the agent reports and uses the prompt's exact force-stop command". That is
agent behaviour. Grove's lifecycle tests use a fake managed program. k22 asks
for a "missing/unreadable-guidance report plus force-stop" in "disposable
lifecycle sessions" and does not name the harness. The acceptance planner
lists "missing guidance" among the directory-route cases only.

Failure: a fixture that sends force-stop proves that Grove keeps the leaf live
and does not relaunch it. It does not prove that the new mandate wording makes
a real agent report and force-stop. The k22 wording lets a session credit the
fixture as both.

Consequence: the main scenario of the mandate change has no owner as an actual
observation. That scenario is a native-only owner whose harness lacks the kind
skill. A relaunch loop in a real unattended session would then be found only in
use.

Repair: k22 states that its lifecycle proof uses a fixture and proves
Grove's interpretation only. The acceptance planner adds, for each real
harness, a native-only case with injection disabled and the kind skill absent.
That case observes the report and the force-stop.

### 6. The temporary local-only build has no stated refusal for a Git source (low)

Location: `ravel-local-delivery-k24`, Notes.
The `ravel-local-delivery-k23` brief, Notes.

Evidence: the directory design admits `type` `local` or `git` and defines no
"unsupported source" outcome. The local planner must "account for the
transient local-only stage explicitly, without inventing another public
config".

Failure: a session can map a valid Git source to `settings_invalid`. That
diagnosis is false for a valid configuration. Or a session can add a stable
`skill_*` code that becomes public by accident.

Consequence: the local stage's fixtures can assert a refusal that the pinned
stage must then change. Or a transient code can reach a release.

Repair: the local planner records three rules. The local build refuses a
Git source before policy import with a remedy that names the unfinished pinned
stage. It uses no stable published code. The pinned stage removes that refusal
and its fixture. Finding 3 must keep that build out of a release.

## Coverage traced

Apart from findings 1, 2 and 5, the plan assigns each parent Done when to a
stage. It also assigns each row of both acceptance tables. Grove's rows go to
the native stage, the dual-major stage, acceptance, research or rollout.
Research gets native names after a move, and external and aggregate
conformance. Rollout gets each tap state.

Ravel's rows go to the local
and pinned planners. Real-harness rows, compaction, delegate authority and the
whole-preamble trial go to acceptance.

The plan keeps fixtures and LLM adherence apart, except for finding 5. It keeps native lifecycle and confined
read access apart. It keeps an unprompted recovery apart from a retained path.
Documentation, books, manual cleanup, open-tree behaviour, installed checks and
owning commits go with their behaviour. Issues #26 and #27 and the pack moves
stay out of scope.

The schema and reader rule holds as written. No stage separates a schema from
its only public reader. The pinned stage cannot close on private acquisition
code. The research prerequisite holds. Acceptance hands a recorded,
reproducible path to `external-research-pack-k4`.

## Limits

This review read the tree, Grove's delivery specification, evidence and
source, Ravel's design line `b93bb927`, Ravel `main` `24a54523` and the
`Ravel.gh-issue-2` task tree. It ran no build, test or check. It did not
measure session length. Finding 4 estimates scope from file sizes and the
number of files. The provisioning-wording count is from a text search, not an
inventory. This review did not ask the owner whether the `Ravel.gh-issue-2`
workstream still runs or waits.

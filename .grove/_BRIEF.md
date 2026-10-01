# Harness selection and execution — brief

## Goal

Build a separate executable that selects and executes an agent harness with the
appropriate model and effort. Grove supplies the selected task and session
prompt; the executable evaluates user-owned selection policy and is useful
outside Grove.

## Human requirements already established

- Keep the executable in a separate package in this repository for development,
  testing and initial deployment. Ship it with Grove's installation and release.
  The installation must evaluate TypeScript without a separately installed
  runtime; design chooses embedding or bundling. Preserve a boundary that allows
  eventual extraction to a separate repository.
- Pass the session kind as an explicit argument. The executable must not parse
  Grove filenames to discover it or depend on Grove for ordinary use.
- Pass the original prompt unchanged, plus the selected task file and stable
  task identity when one exists. Grove supplies its selected handle as caller
  data; the executable does not recover identity from a filename or prompt.
  Other callers can supply generic context and stable artifact identities.
- Support configured selection and dynamic per-invocation selection. Prefer
  TypeScript configuration so policy can perform arbitrary computation without
  growing a declarative policy language in Grove.
- Choose the harness, model and effort together, then execute the harness.
- For the first version, every review selected through the supplied review
  policy must use a different provider from the artifact's original creator.
  This includes explicit choices and every re-invocation (a retry).
  Use recorded execution provenance, or an explicit owner declaration of the
  original creator's provider labelled as declared. Missing both must stop review.
  Keep this simple: no separate model comparison or multi-author accounting is
  required for this increment.
  Direct-harness configurations remain outside that policy's enforcement.
- Encode that review rule in static configuration or TypeScript policy, rather
  than hard-coding it into the generic executable. Artifact-associated creator
  provenance must be discoverable by TypeScript when selecting a reviewer.
- Load personal policy by default. Repository TypeScript runs only when
  explicitly selected through personal configuration or a `--config` argument;
  cloning or entering a repository must not execute its policy code.
  The host must not implicitly load repository runtime configuration, preload
  hooks, environment files or shadowing modules when evaluating personal policy.
  An explicit relative path is an opt-in to the resolved location in that cwd;
  inspection identifies it. Trusted policy can explicitly import other code.
- Higher abstraction and/or higher consequences of error justify more effort.
  Consider uncertainty, downstream rework, reversibility and available checks.
- Start with useful static policy and a testable dynamic selection interface.
  A learned selector needs representative outcomes and calibration before it
  controls launches. General web findings inform priors; task-specific evidence
  must establish value across model/effort combinations.
- The agreed first release is static routing plus a programmable TypeScript
  selection interface, with execution and outcome records ready for evaluation.
  A working local LLM selector and its evaluation pilot are follow-up work, not
  required for this first release.
- Incomplete mappings stop the launch with an actionable diagnostic. The human
  rejected automatic fallback; the executable must not fill a missing or invalid
  selection by inventing a default or substituting another candidate.
- An explicit choice names one configured joint candidate. Policy sees it and
  can accept or refuse it; the executable rejects a different returned candidate.
  Partial model/effort overrides and automatic retries are not required.
  Grove owners supply such choices through personal command configuration;
  this adds no Grove flag, environment override or task-body launch metadata.
- Grove validates its configured command before authoring a task. When that
  command delegates, the new executable validates its policy at launch only.
  This accepted boundary applies to static and computed policy; document it in
  usage guidance and configure-grove. Tree mutation does not evaluate policy.
- The existing Linux floor is kept for glibc 2.17 and the baseline CPU, both
  executed on each Linux target. The worker's kernel floor is Bun's documented
  range, stated as documented rather than executed and rechecked at each Bun
  upgrade. The human accepted this trade-off in
  `harness-selection-and-execution-k7`: RHEL 7-era kernels are claimed, not
  tested.

## Done when

- An independently usable executable can inspect a proposed choice and execute
  a configured harness using static or computed policy, with actionable errors.
- Grove passes the selected kind, task, stable identity and prompt through a
  small generic seam; current direct-harness configurations continue to work.
- The interface supplies bounded context with documented volume/time limits and
  exposes artifact-associated original-creator provenance to configuration.
  A shipped example policy implements the provider rule, with explicit personal
  activation instructions and command-seam tests of that delivered artifact.
- Execution preserves terminal, working directory, cancellation, exit status and
  Grove completion authority correctly, including cancellation during selection.
  Selection helpers are not granted completion authority. Trusted policy is not
  sandboxed against a hostile local owner.
- Configuration ownership and trust, overrides, unavailable candidates and
  fallback behavior are explicit and tested at their external boundaries.
- Records support later evaluation of acceptance, missed defects, false findings,
  downstream repair, human work, latency and total usage, retaining unknown or
  unobserved outcomes explicitly. Launch success is not task acceptance. Any
  probability fields distinguish choices from calibrated success probabilities.
- Durable usage/design documentation and the configure-grove skill explain the
  resulting division of responsibility. Required checks pass and the completed
  work is ready for the repository's usual confirmed finish/release sequence.

## Acceptance cases

- A caller without Grove supplies a kind, prompt and sufficient generic task
  context and can inspect or run one configured joint harness/model/effort
  choice. A task file is optional. TypeScript can consume caller context without
  needing Grove filenames or a closed enumeration of kinds. Exercise this with
  no Grove binaries, configuration or task tree present. Non-Grove review callers
  can supply a generic reviewed-artifact/original-creator association.
- Static mapping and computed TypeScript policy can each select a configured
  candidate. Inspection explains the effective policy source, choice, reason,
  context sources, delivered context size, limits and expanded arguments without
  launching the harness. It must not promise that evaluating trusted TypeScript
  is free of side effects.
- An incomplete mapping, unreadable or invalid selected configuration, invalid
  selection result, unavailable selected executable, missing required context or
  failed context loader launches nothing and reports the missing or invalid input.
  No automatic default fills the gap. Context overflow is refused or represented
  explicitly to policy; it is never silently discarded.
  Explicit choice inputs are visible to policy and do not silently become some
  other candidate when they cannot be satisfied.
- A repository containing TypeScript policy causes no execution merely because
  the caller runs there. Personal policy or an explicit `--config` selection is
  the authority to load it; inspection identifies the selected source.
  Test explicit cwd-relative selection as admitted authority, and test that
  repository runtime configuration, preload hooks, environment files and module
  shadowing cannot affect personal policy through implicit host discovery.
- A review can discover the original creator's provider through artifact-associated
  provenance. Changing today's producer mapping does not change that record.
  For artifacts made before adoption or by direct harnesses, an owner can supply
  an explicit declaration, inspectably recorded as declared rather than
  execution-recorded. The supplied configuration chooses another provider;
  missing both forms is an incomplete mapping with a diagnostic explaining the
  declaration remedy, not permission to launch a same-provider review.
  Provider is an owner-declared candidate attribute for model origin; a gateway
  change does not create a different provider. It is never inferred from argv.
  The supplied Grove policy/context adapter recognises review relationships;
  missing or ambiguous associations refuse. The generic core has no Grove
  review-kind list or `Reviews` parser.
- Provenance and later outcome association survive retirement and reordering of
  the producing leaf across sessions in the same live grove and workspace.
  Grove supplies its stable handle; lookups use globally unique run identities,
  so handles need no namespace against other groves. Run and observation
  records live outside task bodies and retain their identities when `.grove/` is
  removed; a review task carries only its creator reference. Automatic
  cross-checkout discovery and post-teardown artifact lookup are not required in
  the first release.
- Grove passes the kind, task path and stable handle from its authoritative
  selection along with the original prompt. Spaces, quotes and shell punctuation
  in the prompt or paths remain data. Existing direct-harness templates remain
  valid. The adapter does not select another leaf or recover its kind from the
  prompt.
- With a valid Grove route to the executable but an incomplete delegated mapping,
  task authoring succeeds and launch refuses actionably. Test and document that
  Grove's pre-authoring guarantee covers only the configured command.
- The selected harness keeps the intended cwd, terminal, cancellation and exit
  behavior. Selection helpers receive no Grove completion authority; the final
  harness receives the completion channel. Test helper environments as well as
  final-harness delivery. The executable dispatches one fixed joint choice per
  invocation; it does not police later model changes inside the harness.
  Preserve Grove's existing wrapper-exec and foreground-job contract.
- Interrupting context loading or policy evaluation, or exceeding its documented
  finite time bound, launches no harness. Helpers remain subject to the enclosing
  job's cancellation. Selection consumes no interactive stdin; policy diagnostics
  cannot corrupt structured inspection/protocol output. Refusal has a documented
  non-zero exit result; final-harness exit behavior remains intact. Abrupt kill
  need not produce a terminal record.
- Before launch, persist a run identity, artifact/task association when supplied,
  selected candidate/provider and argv, policy/context versions and timestamp.
  Failure to write this required record stops launch with a diagnostic. Records
  distinguish a proposal, attempted handoff, observable launch failure and
  execution confirmed by external evidence; an attempt alone is not success.
  Provenance describes the configured launched choice, not verified backend
  identity. Exit, duration, usage and outcomes remain unknown unless observed.
  Supply a documented, tested way to write/import later acceptance, findings,
  repair, human work and other observations against the run, without treating
  absent measurements as zero. An outcome schema alone is insufficient; analytics
  and a local-selector benchmark are outside this release.
- Grove's existing release/install route delivers the new command on its
  supported targets. Usage documentation and configure-grove explain which
  configuration owns selection, how to inspect it, and how to remedy incomplete
  mappings. Reusable package checks join the repository's Taskfile workflow.
  Check archive/install contents and run a static/TypeScript fake-harness smoke
  test on each supported target, natively or under emulation; retain the existing
  Linux compatibility floor. Run the TypeScript case without a separately
  installed runtime. Review documentation against these acceptance cases.

## Agreed test seams

The human agreed these boundaries during requirements:

- Exercise the new command with temporary TypeScript policies and fake harness
  executables. Verify selection, generic context delivery, inspection,
  provenance, trust admission and refusal on incomplete mappings.
  Use the shipped example review policy, and cover required-record write failure,
  later outcome entry, stable association through task renames and explicit-choice
  mismatch. Inspection must expose measured context size and source authority.
- Extend Grove's existing launch-boundary integration tests. Verify unchanged
  prompts, selected kind/task context, terminal and cancellation behavior, exit
  status and completion authority reaching only the final harness. Retain
  direct-harness compatibility coverage.

These remain the two process seams. Release artifacts additionally need the
per-target delivery checks above; documentation acceptance is a review. Dedicated
integration with confined `grove run` routes is follow-up work; existing
standalone templates remain unchanged. Independent command use is in scope.

## Starting evidence and existing contracts

Read `docs/research/grove-model-effort-routing.md` for the dated model research,
local inference investigation and proposed interface. It is research and a
starting proposal, not an implemented API or a substitute for requirements.
The durable boundary decision is
`docs/adr/harness-selection-is-owned-by-policy.md`.
Read `docs/CONFIGURATION.md`, `docs/adr/complete-session-configuration.md`,
`docs/adr/a-kind-is-an-open-token.md`, `docs/adr/the-launched-child-is-a-job.md`,
and `docs/adr/untracked-configuration-delta.md` when defining the integration.
Use the repository Taskfile for reusable development workflows.

## Next design and planning work

The human confirmed the consolidated requirements and the next sessions: an
independent requirements review (`harness-selection-and-execution-k2`), its
integration (`harness-selection-and-execution-k4`), then design
(`harness-selection-and-execution-k3`). Design synthesizes
the running decisions in `harness-selection-and-execution-k1`, their review
dispositions in `harness-selection-and-execution-k4`, and this brief; it
does not repeat the interview. It owns the executable/package name, runtime and
delivery choice, policy/request/selection protocol, configuration precedence,
bounded generic context, simple provenance association and record format. It
chooses which producing invocation is associated as original creator when a leaf
is launched more than once, without adding multi-author accounting. It
must account for execution and completion authority before choosing a TypeScript
hosting strategy, and preserve the agreed process test seams.

The design produces the enduring area spec, reconciles the ADR set and schedules
review when needed. Planning then cuts independently useful increments against
that design. Do not pre-build implementation leaves before those decisions.

## Beyond the first release

The research records Jev-style decision models and open-source local alternatives
for a later selector pilot. The user has oMLX on an Apple M4 Max with 128 GiB
memory; availability is not evidence of routing quality. A working local selector,
calibration campaign and repository extraction are outside this grove's first
release scope.

## Design handoff

The area contract is `docs/specs/harness-selection-and-execution.md`, with the
policy-ownership ADR and `docs/adr/policy-evaluation-precedes-process-replacement.md`.
`docs/design/harness-selection-and-execution/` contains editable visual views and
the bounded native runtime/source evidence. The command name is
`harness-dispatch`; Rust owns admission/records/exec and an installed compiled Bun
worker evaluates the TypeScript entry. The spec labels the new behavior as design,
not implementation.

`harness-selection-and-execution-k5` reviews that design before
`harness-selection-and-execution-k6` plans independently useful increments. A
review with actionable findings must insert integration before planning. The
review should challenge the scope lifecycle, explicit creator registration,
startup trust, process semantics and delivery evidence, without reopening the
settled requirements interview. These design choices are delegated decisions,
not newly asserted human approvals.

`harness-selection-and-execution-k7` integrated that review. Its running log
holds every finding's disposition. The runtime, signal, inspection, delivery and
evidence repairs are in the spec, both ADRs and the runtime evidence. The review
showed the original-creator mechanism unreachable in unattended loops, and the
per-grove dispatch scope in conflict with
`docs/adr/one-live-driver-per-working-tree.md`. The human then questioned why
creators need registering and why a handle namespace is needed at all.

`harness-selection-and-execution-k8` redesigned that area. The human chose the
review-carried creator reference, their own last suggestion, refined to name the
producer's dispatch run rather than transcribe its provider. They confirmed the
methodology amendments it needs and amended the two acceptance sentences above.
Their other suggestions were compared and set aside. k8's running log and
`docs/adr/a-review-carries-its-creator-reference.md` record why: a handle
identifies no grove without stored state, and Grove cannot know a provider. The
review's original-creator requirement is unchanged: a different provider from the
original creator, recorded or explicitly declared, with missing both stopping
review.

`harness-selection-and-execution-k9` reviews the whole current design, including
k7's unreviewed repairs, before `harness-selection-and-execution-k6` plans. A
review with actionable findings inserts integration before planning.

`harness-selection-and-execution-k10` integrated that review within the chosen
mechanism; its running log holds the dispositions. A session finishes a producer
by retiring its leaf or closing its node, and it replaces the review's
`**Creator:**` line with its run, or removes the line when it has no run. A run
reference is the session's attestation: its provider is recorded, but its
association is not verified. The creator-reference ADR carries the amendment
text, including the node-close step, that ships with the implementation.

## Implementation plan

`harness-selection-and-execution-k6` cut the implementation from the reviewed
design, and its running log gives the reasons for this order.
`harness-selection-and-execution-k42` reviewed the plan before any increment
ran. `harness-selection-and-execution-k43` integrated that review, and its
running log holds each finding's disposition. Two obligations moved into
static dispatch: the whole-selection deadline, because static policy already
executes TypeScript at import, and the required handoff record, because every
run needs it. Node reviews moved out of their nodes.

Each entry below leaves `task check` green and Grove releasable, and adds
behavior its successors build on. Releasability is claimed at these increment
boundaries, not at a leaf boundary inside a node:

1. `grove-task-slots-k11`: Grove's optional `kind`, `task_file` and `task_id`
   lifecycle slots, useful to any wrapper.
2. `static-dispatch-k12`: a standalone `harness-dispatch` that inspects a static
   TypeScript `routes` policy through the compiled worker within the selection
   deadline, and runs it after committing the required handoff record.
3. `dispatch-delivery-k16`: the pair, with bundled SQLite, in every release
   archive and the Homebrew formula, smoke-tested on each target at the Linux
   floor.
4. `computed-policy-k20`: `select`, caller and loaded context, and the SDK reads,
   within the documented bounds.
5. `dispatch-records-k23`: later observations against recorded runs, and policy
   run lookup.
6. `evaluation-boundary-k27`: signal cancellation, signal-transparent handoff,
   and proof that ambient authority stays inert.
7. `grove-dispatch-k31`: Grove sessions launched through dispatch, at the launch
   boundary and the controlling PTY, with configure-grove guidance.
8. `review-policy-k35`: the shipped example review selector and the Grove
   adapter.
9. `creator-reference-k38`: the methodology amendment, its conformance rows and
   pins, and the Grove creator lifecycle cases.
10. `dispatch-documentation-k41`: the consolidated usage and spec current state,
    then the mandatory documentation-acceptance review.

Delivery is deliberately third. The supported-target floor is the design's
largest untested risk, and the worker ADR's reopen condition names it. Its first
cross-build already carries bundled SQLite. After delivery lands, the per-target
installed smoke task is a regression instrument. Any later leaf that changes the
worker, the installed layout or the native dependencies reruns it.
Delivery closed with `cpu-floor-k19`: the compiled worker met both Linux floors,
the C library and the CPU, so the ADR was not reopened. `task release` now
publishes nothing unless every archive passes that smoke test, so a leaf that
breaks a floor also blocks the release. The smoke needs an arm64 Docker and Zig
(`docs/RELEASING.md`).

Computed policy closed with `bounded-context-k22`. A caller's `--context`
document and a `loadContext` result are checked by one version-1 shape in
`src/context.rs`. `select` receives the delivered context, which is that value
with its measured sources attached as `measured`. Later leaves build on three
facts. First, a bound or refusal the worker records is reported even if the
policy catches its error, and a new host operation follows the same rule.
Second, the Grove adapter (`grove-review-adapter-k37`) reads its task file
through `host.readText`, so the file becomes a measured source whose digest
inspection and the run record show; the SDK already exports `ReviewedArtifact`
and `Creator`. Third, the run record's launch document fills
`reviewedArtifact` and `context`, and it carries all six bounds within
version 1. `adapter` stays `null` until k37 supplies it.

Dispatch records closed with `run-lookup-k26`. `record observe` and `record
show` carry later evidence against a run (`run-observations-k25`). Run lookup
is open to `loadContext` alone, under `inspect` and `run` alike, and the front
answers it from the record store. `host.run(runId)` returns `{ runId, status:
"found", recordedAt, kind, taskId, candidate: { id, provider, model, effort },
launchFailure }` or `{ runId, status: "missing" }`. A missing or empty store
answers missing. The delivered context carries every answer, in call order, as
`runs`, which a loader cannot supply. So the review selector
(`review-selector-k36`) has its loader call `host.run`, and its `select` reads
the creator's recorded provider from `context.runs`. A store that exists but
cannot be read, is corrupt or of another version, or holds a launch record this
release cannot read, refuses with exit 4 before any more policy code runs. The
selector never sees that case, and needs no remedy of its own for it. The front
fills the run record's `creator`, and inspection's, from the delivered context:
`{ reference, evidence, provider, lookup }`, with evidence `execution_recorded`
or `declared`. Neither the selector nor the adapter records it itself.

The evaluation boundary closed with `ambient-authority-k30`, and
`evaluation-boundary-k54` integrated its review. Later leaves build on four
facts. First, `environment::Grants` owns the worker's whole environment: the
base set, exact `--policy-env` grants, the front's own `environment::HOST`
setting `BUN_RUNTIME_TRANSPILER_CACHE_PATH=0`, and the excluded classes
`BUN_*`, `NODE_OPTIONS`, `NODE_PATH`, `NODE_PRESERVE_SYMLINKS`,
`NODE_CHANNEL_*`, `LD_*`, `DYLD_*` and `HARNESS_DISPATCH_*`. Inspection
reports `policyEnv`, names only, and runs record no grant. Grove's documented configurations grant nothing. Second,
`task dispatch:probes` builds the probe builds, each the shipped source with
one control removed, and `task check` builds them before `cargo test`. A probe
reports `probe-<name>-<source digest>`, which no front accepts. Tests drive a
probe directly through `tests/support/direct.rs`. Third, the fake harness now
records its environment's names (`Sandbox::harness_env`), so a lifecycle test
can show a completion value reaching only the final harness. Fourth, the
shipped worker reads no `package.json` at run time, so a package whose entry
`main` or `exports` declares does not load.

`package-entry-resolution-k52` decided that limitation stays for now, and the
spec's `#policy-authority` states it as the contract with its reason. Bun
resolves the compiled worker's own imports, and those of any module with no
file location, from the directory the worker started in, and walks up from
there. The front creates that directory under the caller's TMPDIR. With
package.json autoloading on, a `package.json` in TMPDIR took part in every
evaluation. With every switch off, a `node_modules` there already answers a
bare import from a `data:` or `blob:` module. Two leaves follow, ahead of the
documentation. `worker-directory-chain-k61` closes that chain, and
`package-json-autoloading-k62` then turns the switch on from the work k52
parked beside it. Later leaves build on two facts. `scripts/dispatch.sh`
states the shipped autoload switches once, as `SHIPPED_SWITCHES`, and
`switches_enabling` derives each probe's from them, so a probe differs from
the shipped worker by its one control; the `tsconfig` probe now turns on
tsconfig autoloading alone. And the README's paragraph beginning "One
exception is known" is k61's to remove, so `dispatch-documentation-k41`
consolidates around whatever k61 and k62 leave.

`worker-directory-chain-k61` closed that chain, with a repair the plan had not
listed. The worker still starts in the front's private empty directory, now
created owner-only, and moves to `/` before it registers or loads anything.
Bun resolves a module with no file location from the directory the process is
in, and reads bunfig, and its first VM's dotenv files, from the one it started
in. k61 concluded that no stated control was given up and did not ask the
human. That was wrong for one path, which the next paragraph states. k61's
running log has the reasoning and what was seen. Later leaves build on four
facts. First,
`task dispatch:probes` also builds `unmoved`, the shipped source without the
move, and `Probe::ALL` in `tests/support/direct.rs` lists every probe. It is
the firing configuration for anything above the worker's start directory, so
k62's `package.json` case drives it. Second, with the switch on in an
experimental build, the move left a TMPDIR `package.json` unread: an `imports`
map answered nothing, and one of 4 GiB did not stall. No test holds that yet;
k62's case is where it lands. Third, a policy's `process.cwd()` is `/`. The
README's *One exception* paragraph is gone, and the paragraph after its
`adapter` field says where a policy runs. Fourth, the comment above
`SHIPPED_SWITCHES` now says the reach is closed and why the switch is still
off, and k62 rewrites it with the switch. k62's parked patch carries a comment
in `tests/hostile.rs`, "Nor is it the front's private directory that keeps it
out", which is still true and no longer the whole reason.

`worker-directory-chain-k63` reviewed k61 and `worker-directory-chain-k64`
integrated the review. k64's running log holds each finding's disposition, and
*A VM started later* and *The root directory* in the runtime evidence hold
what was seen. Later leaves build on five more facts. First, the guarantee
stops short of `/`. `/node_modules` answers a module with no file location
through the shipped pair, and with package.json autoloading on `/package.json`
answers one too. Both were seen as root in a container, since the suite cannot
plant a file in `/`. So k62 says "no directory between where the worker
started and `/`" and never "above", and its TMPDIR `package.json` case proves
that much and no more. Second, Bun loads dotenv files again for each VM it
starts, from the directory the process is then in, and never rereads bunfig.
A native `Worker` a policy starts would read `/.env`, and the dotenv switch
alone keeps it out. The private start directory is a second control only for
what Bun reads as the process starts. The human accepted that single control
on 2026-10-01, so it is settled and not k62's to reopen. Third,
`hostile::a_dotenv_where_a_policy_starts_a_worker_stays_inert_and_fires_under_the_autoload_probe`
holds that, and was seen to fail with the shipped switch turned on. Its
helpers, `DOTENV_REPORTING_WORKER` and `dotenv_view_policy_moved_into`, serve
any later case that needs a VM started after a move. The tripwire test now
watches the VM that moved for dotenv, and that VM and a `Worker` for bunfig.
Fourth, `scripts/dispatch.sh` has a comment above `SHIPPED_SWITCHES` saying
why the dotenv switch is sometimes the only control. k62 rewrites the
package.json comment below it and keeps that one. k62's parked patch dry-runs
as it did before k64: `tests/authority.rs` fails its one hunk, and the other
three files apply. Fifth, Docker Desktop on the development host moved from
28.1.1 to 29.8.1 during k64, and its seccomp profile refuses the smoke test's
helper container a user namespace. The human chose to run that one container
with `--security-opt seccomp=unconfined`; `scripts/release-smoke.sh` gives the
reason at its `docker run`. `task release:smoke` passes on all three targets
that way, and a leaf that sees it fail there again looks at Docker first.

`package-json-autoloading-k62` turned the switch on. The shipped worker reads
`package.json` at run time, so an ordinary npm package loads, and the fourth
fact under the evaluation boundary above no longer holds. k62's running log
has the reasoning and what was seen, and *Package.json autoloading* in the
runtime evidence has the tables. Later leaves build on six facts. First, the
spec's `#policy-authority` states the contract in four paragraphs, from "Other
external imports" to the one before "Because tsconfig". The README's *Which
policy runs* gained three paragraphs, and its "Nothing ambient takes part
otherwise" passage changed. `dispatch-documentation-k41` consolidates around
those. Second, one limit was accepted without the human. An `imports` alias to
a registered specifier is a `node_modules` lookup: it refuses with no package
of the name, and loads a `node_modules/harness-dispatch` shadow beside one.
The spec states it at `#imports-alias`, the README says not to alias those
names, and `hostile::an_imports_alias_to_a_registered_specifier_is_a_package_lookup_and_never_the_embedded_module`
holds both sides. k62 judged it inside what an entry admits. The review below
is asked to attack that first, and a finding against it is the human's to
settle. Third, every probe derives its switches from the shipped set, so the
`tsconfig` and `autoload` probes now have package.json autoloading on as well.
Fourth, `direct::drive_within` in `tests/support/direct.rs` bounds the wait
for a frame and kills a worker that stays silent, and `Driven::stalled` says
so. A firing configuration that stalls a worker uses it. Fifth, the oversized
arm of
`hostile::a_package_json_above_the_workers_start_directory_stays_inert_and_fires_under_the_unmoved_probe`
holds about 4 GiB resident for five seconds on every run of the suite, which
is the cost of keeping its firing configuration in the test. Sixth, the
installed smoke test has a fourth case, `declared_package`, and
`task release:smoke` passed with it on all three targets.

`package-json-autoloading-k65` reviews k62 before
`dispatch-documentation-k41` runs. A review with actionable findings inserts
its integration ahead of k41.

`package-json-autoloading-k66` integrated that review, and its running log
holds both findings' dispositions. Later leaves build on four facts. First,
the fifth fact above no longer holds. The oversized file is out of the suite.
`hostile::a_package_json_that_never_yields_above_the_workers_start_directory_stays_unopened_and_stalls_the_unmoved_probe`
holds the claim instead, with a `package.json` that is a link to a FIFO, in
five seconds and no memory to speak of. Its firing arm asserts that the probe
opened the file and was still running and silent at its bound, so a probe
that dies passes neither. `watching_for_a_reader` in `tests/hostile.rs` says
whether any process opened a FIFO while a closure ran, and serves any later
case that must see a file opened or left alone. `support::mkfifo` makes the
FIFO. Second, the nearest-`package.json` rule is qualified wherever it is
stated. The nearest one that reads as a package is a module's own, named or
not. One that does not parse is passed over for the next above, where Node
refuses. The spec has it in the `#policy-authority` paragraph beginning "The
worker reads `package.json` at run time", and the README in the paragraph of
*Which policy runs* beginning "One difference from Node".
`authority::the_nearest_package_json_that_reads_as_one_is_a_modules_own_and_one_that_does_not_is_passed_over`
holds it, and `dispatch-documentation-k41` consolidates around those
paragraphs too. Third, the runtime evidence has both readings under *Which
file is the nearest* and *A `package.json` that never yields*, and keeps the
4 GiB table as k62's measurement. Fourth, no shipped source changed. The
worker is still build `721aab0848f6…`, so the installed smoke test was not
rerun, and k62's run of it stands for this worker.

Grove dispatch closed with `grove-dispatch-guidance-k34`. Later leaves build on
three facts. First, the Grove-side guidance lives in four places, and later
guidance joins them rather than starting a fifth.
`docs/CONFIGURATION.md#harness-dispatch` holds the whole account.
`docs/USAGE.md#if-a-dispatched-launch-refuses` holds the refused-launch
workflow. `plugins/grove/skills/configure-grove/references/dispatch.md` holds
the operator procedure. The dispatch README's "Called from Grove" section
points back to Grove. The review policy's activation (`review-policy-k35`) and
the missing-creator remedy (`creator-reference-k38`) belong beside the
incomplete-mapping remedy there. Second,
`the_documented_command_definition_for_dispatch_is_the_one_launched_here`, in
`crates/grove/tests/loop_driver.rs`, pins every `harness-dispatch run --kind
${kind}` command those surfaces and both help texts quote to
`dispatch_template`'s words. It allows only a literal `--choice ID` before the
prompt. A leaf that documents another form, such as `--config` or
`--policy-env` in the command, extends the test with a launched case for it.
Third, the documents activate `harness-dispatch/examples/grove-static`, which
enforces no provider rule. They advise copying it rather than importing it,
since an imported example changes with the installation.

The review policy closed with `grove-review-adapter-k37`. Later leaves build
on four facts. First, `harness-dispatch/grove` reads the supplied task file's
`**Reviews:**` and `**Creator:**` lines, and
`harness-dispatch/examples/grove-review` composes it with the selector over
the Grove static example's catalog and routes. `tests/grove.rs` drives it with
a fake producer that writes its own `**Creator:** run` line from
`HARNESS_DISPATCH_RUN_ID`. `creator-reference-k38`'s lifecycle cases can reuse
that producer. Second, a `loadContext` may return a refusal in `select`'s
shape, which the front reports as `policy_refused` at stage `context`; the
adapter refuses that way, so no `select` can drop its refusals. Third, the
worker reports the adapter's version when a policy imports it, and names by
module the embedded examples that bring it in (`bringsAdapter` in
`worker/src/main.ts`). A new example that composes the adapter joins that set,
and `tests/grove.rs` fails until it does. Fourth, the Grove-side guidance
activates the example in `docs/CONFIGURATION.md#harness-dispatch`, and gives
the missing-creator remedy under "When a dispatched launch refuses" and in
configure-grove's "Remedy a review that cannot name its creator".
`creator-reference-k38` has since made those documents, and the dispatch-side
statements `review-policy-k58` qualified, say who writes the line.

The creator reference closed with `creator-lifecycle-k40`. Later leaves build
on four facts. First, the finishing session's step is the Grove plugin's
`references/retire.md`, *Naming your run on what you finish*, and node-close
step 4 carries it. Its conformance row is `finishing-session-names-its-run`,
so no file of the spine or of a kind skill may restate its lead sentence. The
runner sweeps those skills and not `configure-grove`, whose *The creator line*
points to the step and says what a session leaves behind, as the spec asks of
it. Second, the owner's
account of the line is `docs/CONFIGURATION.md#a-reviews-creator-line`: the two
forms and their writers, the declaration, why a wrong but existing run passes
launch, and attaching findings as an observation. The usage guide's
review-composition section, configure-grove's *The creator line* and the
dispatch README's Grove review section carry shorter forms.
`dispatch-documentation-k41` consolidates against that section and does not
write a fifth. Those documents keep one owner-written run line: for a producer
a dispatched session finished without leaving its line. Third, Grove's
launch-boundary suite has a `Lifecycle` fixture, in
`crates/grove/tests/loop_driver.rs`. Its fake sessions follow that step under
the shipped review example, whose two wrapper programs are on the driver's
`PATH`. A later lifecycle case adds an arm to its `SESSION` procedure. Fourth,
that suite's `driver_command` scrubs `HARNESS_DISPATCH_RUN_ID` and
`HARNESS_DISPATCH_STATE_DIR`, so a direct fake harness has no run when the
suite itself runs under dispatch. A new fixture that launches a driver goes
through it.

`creator-reference-k59` reviewed the node and `creator-reference-k60`
integrated the review; k60's running log holds each finding's disposition.
Later leaves build on three more facts. First, a session that cuts a review
cuts it, and writes its `**Reviews:**` line, before it retires its leaf. The
step's last paragraph says so, and says why: the creator line goes under that
one, and a leaf cut inside a closed node reopens it. k60 rewrote that
paragraph after its one in-session review, and nothing has reviewed the
rewrite. The documentation-acceptance review that
`dispatch-documentation-k41` cuts reads it for a leaf that cuts its own
review, a node close and a multi-level close. Second, one fake session in the
`Lifecycle` cases breaks the step on purpose: the direct-finish case's first
attempt names its run while its leaf is live, to plant the stale line. Every
other fake follows it. Third,
`a_close_cascade_settles_every_live_review_of_each_producer_it_finishes_and_no_other`
is where the step's set is observed: a nested review, a review the closing
session cut for an inner node, two terminal reviews and a review of a producer
whose handle the finished one begins. A change to `name_run` or `finish` is
checked there.

Five nodes end with a `review-impl` of the node: `static-dispatch-k12`,
`dispatch-records-k23`, `evaluation-boundary-k27`, `review-policy-k35` and
`creator-reference-k38`. The leaf whose retirement closes a node cuts its review
as the node's root sibling, with `leaf-insert` ahead of the next increment. So
review still precedes dependent work, and the review names a finished producer.

Each increment is a node of this grove, not a separately created grove as the
planning rule literally asks. This grove charters one first release, and its
finish sequence releases once. No requirement asks for per-increment releases.
k42 found this deviation advisory, and k43 accepted it as a visible trade-off.

Obligations common to every implementation leaf:

- Implement against the spec; it is the contract. As behavior lands, narrow the
  spec's "not implemented" notice to state what is delivered, and never claim
  the whole feature early. Until its owning leaf lands, a form is explicitly
  refused, never accepted and ignored, and no stub methods are published.
- The two agreed process seams are the acceptance instruments. The command seam
  uses temporary policies and fake harnesses with no Grove. The other is Grove's
  existing launch boundary and PTY tests. Internal tests support them and never
  replace them. Map each spec acceptance row the leaf owns to a named test.
- A test that needs the compiled worker obtains it deterministically. If the
  worker is absent, the test fails; it never skips. Every hostile, limit and
  missing-source fixture has a positive control that has been seen to fire.
- `task check` passes, with clippy at deny and fmt clean. The walkthrough book
  for any touched crate stays source-exact. Reusable workflows go through the
  Taskfile. Invoke `grove-llm` directly, never through `cargo run`.
- Bun stays pinned at 1.4.2 and the worker builds only through Taskfile tasks.
  Keep the TypeScript worker, SDK, adapter and examples inside the package
  boundary, so that an extraction can move them without taking Grove.
- A decision records durably only if it clears the ADR bar. Otherwise the spec
  and the leaf's running log hold it.
- Every refusal names its input or source as well as its code, stage, message
  and remedy. `support::Run::refusal` asserts this for every JSON refusal a
  test reads. A refused `run` names its equivalent `inspect` invocation from
  its parsed inputs, so a new selection input joins
  `SelectionArgs::inspect_invocation` (`static-dispatch-k12`).
- A new embedded specifier joins the `embedded` table in
  `worker/src/main.ts`. Its source joins the digest set that `build.rs` and
  `scripts/dispatch.sh` both compute, and `dispatch.sh build` ships its
  declarations and readable source beside the worker. `documented_specifiers`
  in `tests/hostile.rs` lists it too, and fails while the table registers a
  specifier that list lacks. The static starter
  examples export `catalog`, `routes`, `CandidateId` and `policy`, so a later
  example can build on their routes.

# harness-selection-and-execution-k5

**Reviews:** harness-selection-and-execution-k3

## Goal

Independently assess the harness-dispatch design against the accepted requirements
and current Grove contracts before planning turns it into implementation work.

## Context

Read the producer's committed artifact via its stable handle. The enduring area
spec is `docs/specs/harness-selection-and-execution.md`; the boundary and runtime
decisions are `docs/adr/harness-selection-is-owned-by-policy.md` and
`docs/adr/policy-evaluation-precedes-process-replacement.md`. Visual sources and
bounded runtime/source evidence are under
`docs/design/harness-selection-and-execution/`. The root brief and k1/k4 decisions
remain the requirements authority; the design is not evidence of new human
approval or of implemented runtime guarantees.

## Done when

- Findings distinguish contract gaps, unnecessary obligations and accepted
  trade-offs, using source/runtime evidence where it bears on the conclusion.
- The whole acceptance surface is assessed, including standalone use, runtime
  delivery, authority, context limits, original creator, records and both process
  seams. The spec/ADR set is coherent and does not claim implementation results.
- If findings warrant integration, create the appropriate integration leaf
  immediately before the later planning sibling; its charter points to this
  review, rather than copying the findings as obligations.

## Notes

Specific producer doubts worth adversarial examination: whether the compiled
worker can exclude ambient config/module injection across supported targets;
whether the scope binding survives live-tree restarts without confusing inode
identity with durable evidence; whether creator registration remains simple and
actually satisfies the provider rule across retries; and whether cancellation,
same-job helpers and exec timing have an implementable contract. Test the design
against these doubts rather than accepting its stated remedies. Linux execution
and final process integration are explicitly unmeasured in this producer.

No extra in-session reviewer competed with this scheduled review. The configured
review-design route was inspected, not launched or modified.

## Decisions (running log)

Material findings remain, so integration is required. It was cut as
`harness-selection-and-execution-k7` by `leaf-insert` at the planning leaf
`harness-selection-and-execution-k6`, the first live sibling after this review,
so it runs before planning. Its body carries this review's handle, not the
findings.

## Findings

Read: the k3 commit `843d0dee` — the spec, both design ADRs, the `CONTEXT.md`
additions, the Taskfile entries and `docs/design/harness-selection-and-execution/`
— against the root brief, the k1/k4 running logs, the cited Grove ADRs,
`docs/ARCHITECTURE.md`, `docs/CONFIGURATION.md`,
`crates/grove-loop/src/{session_config,loop_driver}.rs`, the release scripts and
Homebrew template. Bun 1.4.2 documentation and resolver source were consulted
through Context7. Inspection only: no build, test, probe or launch was run.
Spec line numbers refer to `docs/specs/harness-selection-and-execution.md` at
`843d0dee`.

Severity: **high** — the design, built as written, defeats a requirement or
contradicts a standing decision without saying so; **medium** — a contract
planning needs is missing or an acceptance test cannot be falsified as written;
**low** — consistency and wording. Each finding is also classed as a *contract
gap*, an *unnecessary obligation* or an *ADR/evidence incoherence*.

### F1 (high) — Recorded creator provenance is unreachable; supplied-policy Grove reviews stop by default

Class: contract gap and unnecessary obligation. Affected: spec 298-327, 397-403,
416-424; runtime ADR paragraph 2; brief "Use recorded execution provenance" and
the design handoff's "chooses which producing invocation is associated as
original creator"; k4 F3 disposition.

`creator bind` accepts a run only when it belongs to S/A **and** an imported
observation carries `executionConfirmed: true` and a matching `producedArtifact`
(300-303, 421-423). Nothing in the design produces that observation: the Grove
integration adds slots only (429-451), and the final harness receives
`HARNESS_DISPATCH_RUN_ID` (176) with no procedure or automation that uses it.
Nor can an owner find R afterwards — `record show` needs the run ID (397),
`creator show` reports only registered creators (320), and there is no lookup of
runs by scope/artifact. The only reachable route is therefore `creator declare`,
for every reviewed artifact. That turns the remedy the requirements scoped to
pre-adoption and direct-harness artifacts into the normal path, and leaves
`execution_recorded` vestigial.

Concrete case: k3 runs through dispatch and retires; the driver picks k5; the
supplied policy finds no registration and refuses (exit 3); the unattended loop
stops at every review until a human declares a provider.

The brief delegated to design the choice of *which* producing invocation is the
original creator. The design hands that choice back to the owner at registration
time (308-311). The k4 F3 disposition defines provenance as "the configured
launched choice, not independently observed backend identity", so the pre-launch
record already is the recorded execution provenance the requirement names, while
still being labelled an attempt. Requiring external execution confirmation
exceeds that requirement, and the cost it imposes on unattended reviews is a
delegated design choice rather than an accepted trade-off. Integration should
either choose a deterministic rule over the artifact's own handoff records,
labelled as such (for example, a single provider across every associated attempt
is used and any disagreement refuses with the bind/declare remedy); specify the
automation that produces the observation together with a run lookup by
association; or put the per-review manual cost to the human with a
recommendation.

### F2 (high) — The dispatch-scope binding adds durable Grove state the ADR set says does not exist, and its tripwire has no provenance-preserving recovery

Class: ADR incoherence and contract gap. Affected: spec 276-296; `CONTEXT.md`
dispatch-scope entry; `docs/adr/one-live-driver-per-working-tree.md` 55-59 and
109-111; `docs/ARCHITECTURE.md` 561-562.

The one-live-driver record says the control-directory files "have meaning only
with live lock evidence; `.grove/` remains the only durable workflow state", and
names statistical freshness as "the explicit cost of keeping grove generation
out of durable workflow state". The architecture document repeats it. The spec
stores a per-grove UUID binding in that same control directory. The binding is
retained across sessions and driver restarts, rotated at root creation and
retired at finish. That is a durable grove generation, and losing it changes
launch behaviour, because reviews then find no creator. Neither design ADR
mentions it, and the standing record is not reworked (`ADR-FORMAT.md`: a changed
decision is edited in place). The choice meets all three tests for a record:
scope IDs are written into every run, so it is hard to reverse; a UUID under
`.jj/grove/` is surprising; and a real trade-off was made.

The tripwire compares stored device/inode identities and refuses delegated
launch on "unexpected replacement" (283-285). Its only remedy, `reset`, mints a
fresh UUID (292-294). Nothing re-adopts the existing scope after a benign
recreation. Any operation that deletes and restores `.grove/` with identical
contents, such as moving `@` to a commit without the tree and back or restoring
it after an accidental removal, gives a new inode and therefore a refusal. The
reset then orphans every creator registration in the live grove, so each later
review refuses until it is re-declared. The false negative is acknowledged
(286-288), but its consequence for the provider rule is not stated as an
accepted trade-off. The one-live-driver record (102-103) itself notes handle
reuse after finish, so a reused scope can match an older grove's creator for a
recurring slug and key. On the task's doubt: the design does not treat inode
identity as evidence identity. It does use inode change as a refusal gate whose
false positives cannot be recovered without losing provenance.

### F3 (medium) — Review recognition by configured kind list fails open for an unlisted review kind

Class: contract gap against the brief's "The supplied Grove policy/context
adapter recognises review relationships; missing or ambiguous associations
refuse". Affected: spec 332-336, 338-343; `docs/adr/a-kind-is-an-open-token.md`.

The example applies the provider rule only to "exact configured review-kind
entries"; other kinds use the owner's static table. Kinds are open tokens, and
the methodology adds one by shipping a skill. An owner who gives a new
`review-*` kind its required exact route but omits it from the review list gets
a static route. The adapter never reads the `**Reviews:**` line the leaf
carries, so a same-provider review launches without a diagnostic. The adapter
recognises a *kind*, so a relationship the task actually declares goes
unrecognised. The fail-closed form is to refuse a task that declares
`**Reviews:**` under a kind not configured as a review entry.

### F4 (medium) — Provider comparison over time is free-text inequality, so label drift or a declaration typo passes the rule

Class: contract gap. Affected: spec 109-110, 304-306, 349-353; policy ADR
paragraph 4; seam row 2 (509).

The creator's provider is a stored string, either snapshotted from the catalog
of its time or typed into `creator declare --provider P`. The supplied policy
requires today's candidate label to differ. If the owner relabels an origin
(`openai` to `OpenAI`), or a declaration is misspelt, every candidate compares as
different, including the same origin, and the review launches. The failure is
silent and unsafe. Seam row 2 tests a gateway disguise (same label, different
program) but not a label mismatch. The fail-closed form refuses when the
creator's provider is not a member of the current catalog's provider-origin set,
names the correction remedy and states any normalisation.

### F5 (medium) — The adapter's handle resolution needs operations and limits the SDK lacks, and the provider rule does not need it

Class: unnecessary obligation, under-specified. Affected: spec 193-208, 210-219,
338-347, 429-432.

Resolving the declared handle "to exactly one task header within the bounded
tree read" means enumerating the tree and reading headers. The SDK offers
`readText`, `readJson`, `originalCreator`, `diagnostic` and `signal`, with no
directory listing. The adapter would therefore need native file calls, which the
spec treats as unreported ambient reads (193-196), while also claiming that the
supplied adapter's delivered context is completely inspectable (207-208). "Under
the supplied root" has no input: the slots are kind, task file, task ID and
scope, so the root must be derived from Grove's path layout, and that derivation
is unstated. If header reads count as sources, a tree above 256 files refuses
every review, since the source limit is fixed; "bounded tree read" states no
bound. Lookup by (scope, handle) already refuses with the remedy when nothing is
registered. Header resolution adds protection only against a typo that happens
to match another registered artifact. Drop it and validate only the declaration
in the task file, or specify enumeration, the root input and the limits.

### F6 (medium) — How owner policy imports the SDK, the Grove adapter and the shipped examples is unspecified

Class: contract gap. Affected: spec 42-45, 146-153, 161-167, 443-446, 483-487;
brief "explicit personal activation instructions and command-seam tests of that
delivered artifact"; seam row 3 "package shadow".

The spec says the worker's own entry and SDK imports are embedded, and that
external imports start at the selected entry. It never says what specifier
`~/.config/harness-dispatch/policy.ts` uses to reach the SDK, adapter or example.
Bun's current resolver source resolves an external file's bare specifiers
through `node_modules` from the importing file's directory, and disables
auto-install in standalone executables. This was read from the main branch, not
the 1.4.2 tag. The candidates are an absolute installed path (a Homebrew
Cellar path changes on upgrade, and a portable archive can be anywhere), a
worker-registered virtual module, or an owner-maintained `node_modules`. Each
gives different upgrade behaviour, activation instructions and shadowing
exposure. Until one is chosen, "package shadow stays inert" has no defined
meaning. Neither version skew between an adapter "with its own version" and the
embedded SDK nor type-checking against the shipped types is addressed.

### F7 (medium) — The Linux floor is defined by glibc alone, and the stated instruments cannot observe the kernel dimension Bun adds

Class: contract gap in delivery acceptance. Affected: spec 473-481, 491-498;
runtime evidence "Primary runtime references"; `scripts/release-build.sh:24`
("RHEL 7-era").

Grove's floor today is glibc 2.17 for Rust binaries. The Bun worker adds a
kernel dependency, and Bun 1.4.2's own sources disagree about it: the README says
"the minimum is 5.1", while the installation document says it "runs on kernels
as old as 3.10 (RHEL 7) with graceful degradation". The evidence cites only
glibc. The spec admits checks "natively or under emulation" and requires "the
actual compatibility floor". A glibc-2.17 container or user-mode emulation runs
the old userland on the host's modern kernel, so a green result cannot establish
a kernel floor. The spec should name the floor's dimensions (glibc, kernel, CPU
baseline) and the instrument that observes each, or state a narrower floor for
the worker as a trade-off.

### F8 (medium) — The runtime evidence and the seam's positive controls cover fewer ambient-loading classes than claimed

Class: evidence incoherence. Affected: runtime ADR lines 21-23; runtime
evidence "Native probe"; spec 161-163, 510, 518-519.

The ADR says native probes "demonstrated both the dangerous ambient-loading
defaults and the controls needed to disable them". The positive control fired
for dotenv and the bunfig preload only. Bun 1.4.2's executable documentation
says tsconfig and package.json runtime loading are already disabled by default
in standalone builds, so the "otherwise identical default build" could not have
shown them dangerous, and the tsconfig fixture was never seen to fire. The probe
also ran the worker inside the hostile cwd. The design's primary control for
cwd-borne input, a private empty worker cwd, was not exercised. Seam row 3
requires the tsconfig and package-shadow fixtures to stay inert under positive
controls (518-519), but defines no configuration in which those fixtures take
effect. As written, those tests cannot fail. State the ADR claim at its measured
scope, and define a configuration in which each hostile class fires. Examples:
the same fixture beside an explicitly admitted `--config` entry, or a probe build
with that autoload enabled.

### F9 (medium) — Signal state at handoff is specified as "default" for three signals, not as the caller's inherited state

Class: contract gap. Affected: spec 239-262, 212, 219;
`docs/adr/the-launched-child-is-a-job.md` ("Only an ignored disposition survives
`execve`").

Before exec, Rust "restores default dispositions, and hands off with the
original signal mask" for INT, TERM and HUP (251-252). Three gaps follow:

- The Rust runtime ignores SIGPIPE at startup, and an ignored disposition
  survives exec, which is the exact class the job ADR names. The spec never says
  which SIGPIPE disposition the harness receives.
- "Default" is not "inherited". An independent caller that ignores HUP (`nohup`)
  or INT (a non-interactive background job) would hand the harness a changed
  disposition, and nothing says handlers are skipped for signals ignored at
  entry.
- Rust's `CommandExt::exec` resets the mask to empty and SIGPIPE to its default
  rather than to the caller's originals. "Original signal mask" therefore
  constrains which exec path is permissible. It should be stated as the
  observable contract: the whole inherited mask and every disposition.

Two further gaps are separate from signal state:

- Cancellation observed after the handoff record commits but before exec
  (247-253) has no specified record. Only an exec error is specified (259-262),
  so an attempt that never executed looks like a real handoff.
- The "invocation's pre-handoff deadline" (219) is undefined relative to the
  30-second selection bound (212).

On the task's doubt: the linearization point is sound and the contract is
implementable. These are underspecified edges at a seam the ADR set already
treats as load-bearing.

### F10 (low) — There is no path from a selected Grove task to the exact inspection of its delegated choice

Class: contract gap in documentation acceptance ("how to inspect it").
Affected: spec 56-57, 437-438, 443-451.

`grove config show` renders the new slots symbolically. `inspect` needs
`--kind`, `--task-file`, `--task-id`, the scope from `grove dispatch-scope show`
and a prompt. The prompt is required even though the worker never sees it. An
owner diagnosing a refused unattended launch must reconstruct every value by
hand.

### F11 (low) — Smaller interface and record inconsistencies

- Lines 269-270 list output-artifact, task, reviewed and no-artifact
  associations, but the CLI supplies only the task pair and the context's
  `reviewedArtifact`. No input names an output artifact distinct from the task.
- Static `routes` with `--choice` (131-133) leaves two questions open: whether a
  choice for an unrouted kind is accepted or refused as an incomplete mapping,
  and whether a static policy can refuse a choice at all.
- Because a policy has "exactly one of `routes` or `select`" (93-94), activating
  the review example turns the owner's whole static table into computed
  selection. Inspection then stops distinguishing static routes for non-review
  kinds.
- `CONTEXT.md` 929 says "Grove's adapter supplies" the dispatch scope. The spec
  has Grove itself populate `task_scope` (276-282), and uses "Grove adapter" for
  the TypeScript reader (39, 338).
- The evidence table counts "a pre-handoff refusal" as an observable launch
  failure (387), yet pre-selection refusals may have no run ID (394-395). Which
  refusals are recorded is unstated.
- The `--policy-env` exclusions (171-173) omit the caller's completion
  variables, and "Grove templates grant none" is a convention: an owner template
  can grant `GROVE_SIGNAL_FILE` to the worker. State that as owner
  responsibility, or give callers a generic way to reserve names.
- The policy ADR keeps requirement-era contract text that the spec now owns: the
  records semantics, trust rules and completion authority. That makes two
  sources for one contract, contrary to k3's "without duplicating contracts".
- The Grove adapter ships inside the independent package but parses Grove's task
  conventions. For the extraction the requirements preserve, it may belong on
  Grove's side of the dependency.

### Accepted trade-offs (no action)

These are visible and consistent with the requirements:

- Delegated policy is validated only at launch, as the human accepted in k4.
- The exec handoff has no post-launch supervisor, so exit and usage stay unknown.
- Trusted policy is not sandboxed. Policy-created children fall outside the job
  contract on timeout.
- There is no cross-checkout discovery.
- The design brings a second toolchain and a larger installation.
- `inspect` is not free of side effects.
- Prompts are stored in private records.
- A harness exit code can coincide with a pre-exec code.

Several parts hold up under adversarial reading:

- The seam heights are sound. The native boundary before evaluation is the only
  place runtime startup inputs can be removed, as the `BUN_OPTIONS` control
  shows.
- Creator lookup through the Rust store keeps one writer.
- Four Grove slots are the minimum integration.
- The acceptance table covers every brief acceptance case.
- The spec and evidence otherwise claim no implementation results.

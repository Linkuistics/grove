# grove.extract-harness-dispatch-to-top-level — brief

## Goal

Extract harness-dispatch into the top-level **Ravel** project with independent
packaging and deployment, while retaining Grove as a working consumer. Ravel owns the
controlled-session notice and the instructions for explicit force-stop and
continue endings.

## Done when

- The older Ravel revision and its local README edit are preserved under a named
  retired revision in the existing repository.
- Ravel independently builds, checks and installs the extracted launcher, its
  matching private policy worker, generic SDK, generic examples and
  notices. Its active documentation describes this tool; the older daemon
  design remains retired history.
- Ravel has its own release tasks, version lifecycle, supported-target archives
  and `linkuistics/taps/ravel` formula. The installed command is `ravel`.
- Grove invokes the independently installed command and its Homebrew formula
  depends on Ravel. Its integration uses only Ravel's public CLI and documented
  parameters. Ravel is caller-independent and contains no Grove-specific adapter,
  policy, task-tree conventions or embedded examples. There is no cross-project
  Rust-library dependency, no bundled Ravel release inside Grove, and no
  assumption that both releases have equal versions or share a Cellar directory.
- Ravel uses the existing harness-dispatch configuration format. Copy the
  owner's current configuration into `~/.config/ravel`, preserving their policy,
  settings and selections rather than installing a replacement sample.
- Ravel injects a controlled-session notice and instructions for continue and
  force-stop into the agent's delivered prompt. The caller's task instructions
  remain intact and the generic session-control instructions have one owner.
- The default exit means continue. An explicit force-stop ends the run and asks
  the caller to stop its sequence. Continue returns to the caller's normal
  decision. On Grove's side, that means advancing when work remains and finishing
  when its completion contract says so. Ravel retains ownership of termination
  and reaping, and implements no Grove loop decision.
- The agreed CLI, Grove integration and installed-layout checks pass, and the
  durable documentation in both projects reflects the executable boundary and
  session-control contract.

## Decomposition

Work in minimal, independently useful increments. A planning review checks
`ravel-extraction-k2`'s tree before implementation. The working handoffs are ordered
below; each leaf includes its tests and current-state documentation rather than
leaving those as a later horizontal phase.

| Leaf | Independently verifiable handoff | Requires |
|---|---|---|
| ravel-cli-k3 | Generic runnable front/worker, rename/report contract, isolated history snapshot and executable docs; retired revision pushed | Settled requirements |
| ravel-delivery-k4 | Release instruments, formula template and all supported unreleased archive checks pass | Runnable Ravel |
| ravel-publication-k11 | Independent release, pinned per-target archives and installed Homebrew formula verified | Prepared delivery |
| grove-ravel-caller-k5 | Both callers use PATH-resolved Ravel and its supported CLI/report; names, guards, caller assets and source-pinned docs migrate together | Installed Ravel delivery |
| grove-ravel-delivery-k6 | Archives/formula ship Grove/caller assets; separate Ravel dependency and live-driver preservation are tested | Migrated callers/assets |
| grove-ravel-docs-k12 | Remaining Grove prose/decisions are reconciled with executable docs now owned by Ravel | Green artifact cutover |
| grove-extraction-release-k13 | Extraction integrated/released independently while this grove and original driver remain live | Release-ready extraction |
| ravel-session-control-k7 | Generic notice and distinct continue/force-stop reports work and are released; extraction-era readers remain safe | Extraction release checkpoint |
| grove-session-control-k8 | Grove consumes both endings with its own continuation/teardown decision | Migrated Grove and updated installed Ravel |

The positional order is deliberate even where two source changes could run
independently: it avoids concurrent installation changes beneath the live driver.
The runnable copy is an expand step. Grove remains usable with the old delivery
until the independent Ravel installation is verified; only after caller migration
does the contract step remove the old package. Session-control behavior follows
as an additive Ravel increment and its Grove integration after the extraction
release checkpoint. The human chose to keep one grove and release part way
through it; this replaces the suggested separate-grove boundary. Do not add
speculative agent-server leaves.

## Pointers

- `plan-k1` records the human's requirements and decisions. The running log is
  the handoff; synthesize it rather than re-interviewing settled choices.
- `docs/specs/harness-selection-and-execution.md`
- `docs/adr/harness-selection-is-owned-by-policy.md`
- `docs/adr/dispatch-supervises-the-harness.md`

These describe the current co-located implementation. The human's requirements
in this brief supersede its shared-library and embedded Grove-adapter choices.

## Notes

The human requires minimal, independently useful increments and no rabbit holes
or excursions. Establish Ravel's independent delivery first. Preserve existing
architecture wherever possible, and handle the controlled-session behavior as
the next small increment. Retire the older Ravel project through the agreed
replacement route.

The project and repository are **Ravel** (`Linkuistics/Ravel`); the command,
formula and package-facing spelling are `ravel`. Reuse the existing checkout at
`/Users/antony/Development/Ravel`. Homebrew owns the runtime dependency from Grove
to Ravel; each project owns its internal runner implementation. Prefer preserving
the existing process/terminal logic locally to introducing another published
library, executable product, or supervision protocol.

Caller-specific integration belongs to the caller. Move any still-needed Grove
adapter, sample policy, examples and related tests to Grove's side; Ravel's
worker, SDK, defaults and injected instructions remain generic. Grove depends
only on documented CLI arguments and parameters, not an embedded adapter or
Ravel's private worker protocol. Keep existing policies, settings and run history
usable through the smallest explicit rename migration. k3 owns the public names
table and isolated store snapshot; Ravel never upgrades the live legacy store.
Late old-supervisor records stay readable through the retained legacy front and
are imported after that driver ends, rather than treating the initial snapshot
as continuous replication.

The configuration format stays as it is. Copy the actual contents of the owner's
current `~/.config/harness-dispatch` directory into `~/.config/ravel` as part of
the first runnable increment. Preserve policy choices, bounds and environment
grants. Make only necessary module-import relocation changes so a Grove-specific
personal policy can use its caller-owned adapter without embedding it in Ravel.
The source currently contains `policy.ts`; the destination did not exist when
requirements checked it. Recheck both at execution and preserve any later edits.
The current personal policy references the generic `harness-dispatch/sdk` module;
any corresponding import rename must preserve the policy's format and behavior.

Transfer the actual build/install/check/release recipes into Ravel's root
Taskfile, wrapping their existing scripts. Preserve the current supported
targets and bundled-runtime delivery; new platforms and runtime upgrades are
separate work. Validate each moved recipe before using it.

This grove is driven by installed Grove 22.1.0 and its original dispatch tool.
Preserve the launch prompt's ending command and channel through the live cutover;
source and installation changes must not strand a session before its final
signal or the next launch. The driver resolves its canonical dispatch executable
once and continues using it after a mid-grove upgrade. k6 tests retaining the
original complete keg with cleanup disabled and preserving the bare old ending
command through a non-overwriting compatibility link on the inherited PATH;
k13 applies that procedure. Preserve the original grove-llm admission/teardown
contract as well. Keep this transition installation until the original driver
has ended, including final finish's last signal. Ravel's documented legacy
exit-channel fallback is additional compatibility, not a change to the mandate.
Carry the existing cargo/test guards into the moved project, including both old
and Ravel control names, so tests cannot use live completion authority.

## Agreed test seams

The human accepted the existing seams; extend their observable cases rather than
introducing new testing infrastructure:

- Ravel's public CLI, real worker, temporary policies and fake agents: the
  controlled-session notice and both ending instructions reach the agent;
  default continue and explicit force-stop are distinguishable in the caller's
  result and the run record; nested and stale signals keep their isolation;
  generic callers work without Grove files, conventions or an installation.
- Grove's lifecycle and standalone launch suites through the installed Ravel
  executable: a continue advances, final completion finishes, and force-stop
  stops without advancing; cancellation and reaping preserve terminal cleanup.
- Archive and Homebrew installed-layout checks on the current supported targets:
  the independently versioned front/worker pair runs without Grove or a host
  TypeScript runtime; Grove discovers the dependency across separate installation
  locations. Retain the existing records and internal runner coverage.

## On the horizon

The human eventually wants an agent-server: a central place to manage agent
sessions with hierarchical names, let agents communicate, present all work in a
dashboard, and selectively interact with agents through a terminal-like system.
Preserve that direction without designing its protocol or session model as part
of this extraction.

## Cross-project handoffs

The live driver and this coordination tree stay in the original session
workspace. Ravel implementation and release leaves work in the reused Ravel
checkout, commit their project changes there and name those commits/installed
versions in their task handoff here, then return here to retire and commit their
leaf. Do not start another Grove driver from inside the current session. Recheck
checkout/config/tap state at each handoff; the planning inspection is not a lock.

Each working increment is a releasable boundary. k11 publishes Ravel's extracted
tool; k13 integrates/releases the extraction's Grove changes before k7 starts;
k7 independently releases Ravel's controlled-session increment. k13 performs
source-only jj integration with no live `.grove/` on main, preserves the session
tree/workspace/lease and leaves the remaining leaves live. It records neither
teardown nor a finish commit. Final Grove integration/release still belongs to
the existing driver-owned finish sequence after k8. Resume partial releases;
never repeat a version bump. Preserve the original launch/ending contract through
both Grove releases; installing new Grove does not replace the running driver.

Ravel's executable spec, generic ADRs and Bun runtime evidence move with k3 and
remain cited by k4's recipes. k5/k6 update source-pinned walkthroughs and gating
assertions with their source changes; k12 reconciles Grove's broader prose to its
caller contract. Per-target Grove installed-layout checks consume Ravel's
immutable published archives/digests, including QEMU targets without Homebrew.
Grove resolves Ravel once from PATH and checks the supported public CLI version
at runtime; formula dependency presence alone cannot establish that contract.

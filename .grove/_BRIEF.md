# grove.extract-harness-dispatch-to-top-level — brief

## Goal

Extract harness-dispatch into the independent **Ravel** project, with its own
packaging and deployment, while keeping Grove a working CLI consumer. At this
stage Ravel is a process manager and router.

## Current contract

The human narrowed the scope during ravel-generic-pair-k18. This contract replaces
older record-store, history-migration and prompt-injection requirements, including
those in completed planning/review notes.

- Exactly two public verbs: `ravel start-session` and `ravel end-session RESULT`.
- `start-session` routes through the owner's policy, launches the selected
  process, supervises it, terminates/reaps it when requested, and returns the
  opaque argument passed to `end-session`. The result belongs to the caller;
  Ravel attaches no continue/force-stop or task-tree meaning to it.
- `end-session` addresses its enclosing session through inherited process
  context. It takes no session identifier or explicit target path.
- The supplied prompt reaches policy/harness unchanged. The caller supplies
  instructions to call `end-session` and the meaning of the result.
- Ravel owns no persistent store, record/observation commands, historical lookup
  API, history snapshots/importers or catch-up work. Existing dispatch history
  stays with the old tool. Grove owns any provenance it still needs.
- Preserve routing choices, process/terminal/confinement behavior and the current
  pinned runtime. Reuse the existing configuration format and copy the owner's
  actual policy/settings to `~/.config/ravel`, with necessary import relocation.
- Testing is deterministic and economical: reusable Taskfile entry points,
  complete saved logs with concise summaries, focused checks during edits and
  one full relevant handoff check. Read failure details only when needed. Reuse
  real-worker/fake-agent and installed-layout seams; no model launches or new
  testing framework. Remove tests of discarded storage/prompt-injection features.

## Done when

- The old Ravel project and README edit remain at the named retired revision;
  its old implementation and local build/editor remnants are outside the active
  replacement tree.
- Ravel independently builds, checks and installs the 0.0.0 front/compiled worker,
  local runner, generic SDK/examples and required runtime notices. The active
  executable documentation describes the two-verb process boundary. Caller
  adapters, policy samples and task-tree conventions remain caller-owned.
- Ravel has independent release tasks, the existing three supported-target
  archives, a first 0.1.0 release and `linkuistics/taps/ravel`. No host Bun/Node
  or Grove installation is needed by its installed pair.
- Grove resolves the installed Ravel from PATH once, checks its supported public
  CLI version and uses only its documented arguments and returned result. Its
  formula depends on Ravel. No shared Rust library, bundled Ravel release or
  equal-version/shared-Cellar assumption crosses the project boundary.
- Grove supplies its own end-session instructions and interprets results to
  advance, finish or stop its sequence. Ravel implements no Grove loop decision.
- Routing, opaque-result round trips, nested/stale context isolation,
  cancellation/reaping, terminal cleanup, confinement and independent installed
  layouts pass. Source-pinned documentation changes with its source; remaining
  ownership prose is reconciled before release.

## Working handoffs

| Leaf | Handoff |
|---|---|
| ravel-cli-k3 (k14 → k15 → k16) | Retired old project, two-verb generic runnable pair, copied configuration and executable docs |
| ravel-delivery-k4 | Independent release recipes and all unreleased target/layout checks |
| ravel-publication-k11 | Published/installed independent Ravel release |
| grove-ravel-caller-k5 | Both callers use start-session/end-session and own their prompt/result/provenance conventions |
| grove-ravel-delivery-k6 | Grove archives/formula ship caller assets and depend on separate Ravel |
| grove-ravel-docs-k12 | Remaining prose/decisions describe the CLI boundary |
| grove-extraction-release-k13 | Extraction integrated/released while the original driver remains live |
| grove-session-control-k8 | Caller-owned continue/force-stop results and instructions |

The former Ravel-specific session-control leaf k7 is abandoned: Ravel injects
nothing and interprets no result token. k8 remains Grove work and requires k13,
not another Ravel feature/release. The agent-server remains a future direction;
do not add its protocol, daemon, dashboard or persistence now.

## Source and delivery

Reuse `/Users/antony/Development/Ravel` and Linkuistics/Ravel. Transfer the
applicable front/worker and local runner from `crates/harness-dispatch` and
`crates/keyed-launch`, removing persistence and caller semantics. Transfer the
actual build/install/check/release instruments, pinned Bun 1.4.2 dependencies,
hostile-probe controls and applicable executable-owned spec/ADRs/runtime evidence.
Reconcile those documents to the smaller contract; do not copy obsolete store,
record, inspect/init or prompt-injection requirements as active Ravel design.

Keep the actual personal policy choices, bounds and environment grants. k16
copies configuration only; it opens or migrates no personal history. Explicit
storage settings are obsolete and must not make the new tool access the old
store. Ravel's public names include `ravel/sdk`, `.ravel-choice`, and
`libexec/ravel/ravel-policy`; `--help` and `--version` are ordinary flags.

## Live-driver boundary

This grove is driven by installed Grove 22.1.0 and its original dispatch tool.
Every current session keeps its launch prompt's original ending binary/channel;
this session still ends with **`harness-dispatch exit`**. The new CLI contract
never rewrites a running session's mandate.

k6 prepares/tests and k13 applies preservation of the original complete keg,
with automatic cleanup disabled, and a non-overwriting compatibility link for
bare `harness-dispatch exit` on the inherited PATH. Preserve original grove-llm
admission/teardown too. Keep that installation through final finish's last signal.
Cargo and direct test helpers scrub original and new session authority controls.

Ravel leaves commit project changes in Ravel, record exact commits here, then
return here to retire/prune and seal their coordination changes. No second Grove
driver. Recheck checkout/config/tap state at each handoff. Resume partial releases
without repeating a version bump. k13 leaves the tree, lease and remaining work
live, with no teardown/finish commit; final integration/release belongs to the
existing driver-owned finish sequence.

## Pointers

- plan-k1 records original requirements; the current contract above incorporates
  the human's subsequent corrections.
- Ravel's `docs/session-cli.md` records the agreed process/router boundary.
- The source executable spec and ADRs describe the pre-extraction implementation;
  retain applicable process/runtime decisions and reconcile ownership in k3/k12.

## Configured Ravel handoff — ravel-cli-k3 closed

Ravel local main is `51b73cbffaa46882d7f9ebad15058e17dbd7bec4`, a sealed
docs-only descendant of runtime baseline
`b1060a176d5fa7c6c5bc96f41cacc18480e0fb65`, followed by an empty change.
k14 preserved the remotely retrievable retired project/owner README; k17/k21/
k22/k23 delivered the generic 0.0.0 pair, independent installation, local runner,
SDK/examples, executable docs and complete runtime/hostile/installed checks.

k16 copied the owner's sole policy.ts exclusively into the previously absent
`~/.config/ravel`. Its 24 routes, arguments, workspace grants and owner version
label are retained; only the SDK import and obsolete inspection comments changed.
No settings/helpers/choice imports required migration. Default bounds and empty
extra environment grants match; no history/store access or installed-front
replacement occurred. Source/destination SHA-256 and exact selected choices are
recorded in Ravel's `docs/owner-configuration.md` and k16's terminal handoff.

The strict public-SDK policy check, `task typecheck`, all 11 CLI tests and 81
fake-agent selections/3 refusals passed. Negative controls fired, and all 1,145
measured subjects retained matching individual digests, including the original
22.1.0 installed commands. k16 lists complete logs/instruments in Ravel's target/;
k23 retains the unchanged full runtime/installation evidence. No publication
occurred. The tap is clean, ravel-lite.rb remains intact and ravel.rb is absent.

k3's complete contract is met. k4 consumes this exact configured source handoff
for archive/target/release preparation; k11 publishes/installs it. The root stays
live for those steps and Grove integration. Keep the original driver and
`harness-dispatch exit` channel through final finish.

## Independent Ravel delivery handoff — ravel-delivery-k4 closed

Ravel local main is sealed at `64b1e6e7b92ebd3eac28c9765e6c705d40c3e95d`, followed by a
clean empty change. It combines k24's independent three-target archive/floor/
manifest and real formula instruments with k25's cut/notes/doctor/publish tasks,
cargo-release 1.1.2 config, refusal/order/lock/state tests and complete recovery.
All k4 preparation criteria are reconciled and met; k24/k25 are terminal.

Durable final evidence is Ravel's `docs/release-lifecycle-evidence.md`, with the
exact latest 0.0.0 local archive/formula digests, all target/native/QEMU/floor
outcomes and complete logs. The root tasks passed lifecycle, archive, formula
and every installed-layout route; all 134 runtime tests passed. No preparation
prerequisite/check is missing. All 14,183 individually frozen subjects matched.

No production version cut/tag/push/publication or permanent installation ran.
Remote rechecks still show no version tags; first minor remains 0.1.0. k11 now
consumes this exact source, rechecks partial state/tags/tap/config and runs the
actual independent release task. It must rebuild and rerun every tagged 0.1.0
target/floor/formula check before publication and provide immutable URLs/digests
and actual installed Homebrew evidence. Never publish the local 0.0.0 files or
repeat a partially completed bump; Ravel's release guide owns recovery.

Owner config and the original installed 22.1.0 Grove/dispatch bytes remain intact.
Tap main is e7416e57, clean, ravel-lite.rb retained and ravel.rb absent. The
original driver's `harness-dispatch exit` contract remains in force. The root
stays live for k11 and Grove integration/release; this is no finish/teardown.

## Public Ravel handoff — ravel-publication-k11 closed

Ravel **0.1.0** is publicly downloadable and installed through
`linkuistics/taps/ravel`. Release source/peeled tag is
`bbab823803bd2f2796f8bd9570f732dce900efa2`; sealed/pushed Ravel main
`3916a4a1d17fef925db7e9c730bd4c23d6df3ac8` adds
`docs/publication-evidence.md` and has a clean empty successor. Tap main is
clean and synchronized at `bd5aa37b721aaff4a72b01fb966785392812b6a8`, with
only Formula/ravel.rb added and ravel-lite retained. The owner explicitly
approved making Ravel's repository/history public.

k11's terminal handoff and Ravel's durable evidence supply **all three exact
version URLs and SHA-256 pins** for k6's independent Linux/QEMU inputs.
Authenticated and anonymous downloads equal the tested archives; the remote
formula, source tag and notes are verified. GitHub does not enforce a release
lock, so consumers must verify the supplied SHA-256 pins. All tagged
manifest/formula/native/QEMU/glibc/CPU-floor routes passed before publication.

`/opt/homebrew/bin/ravel` finds its matching worker under
`/opt/homebrew/Cellar/ravel/0.1.0`. Homebrew's public forced fetch and brew test
pass, as do 81 copied-owner-policy shell fake-agent sessions and three
refusals on a child PATH without Grove/Bun/Node. All installed files match
the archive; 134 runtime tests passed. Configuration, existing tap formulas
and the complete original 22.1.0 Grove/dispatch keg retain their bytes.

The partial main push and private-repository download refusal were recovered
at their documented boundaries, with no second bump or tag replacement.
No Ravel publication/install step remains. Grove integration/release and
the driver's original harness-dispatch exit channel remain live.

## Caller acceptance handoff — k46; grove-ravel-caller-k5 closed

k46 closes grove-ravel-contract-k29 and grove-ravel-caller-k5 after reconciling
current ADR/usage/standalone prose and repeating complete frozen acceptance.
`docs/ravel-caller-surface-audit.md` carries every k5/k29 clause and the current
per-file inventory's named exceptions. No caller acceptance gap remains.

Final task check passes all 14 checks, 1,723 workspace nextest cases, nine
doctests and six final/all books. Explicit installed fixtures/commands/review/
authority acceptance passes 116 executions, including all named ignored cases.
All 1,738 unique identities and 101 shared outcomes match individually across
final/k45/prelinked runs; all 17,069 frozen subjects and adjacent checkout state
pairs match. Complete logs/instruments are in target/k46; the audit and terminal
leaf record exact boundaries and coverage limitations.

Independent Ravel 0.1.0, pinned Bun 1.4.2, the complete original 22.1.0 keg,
owner configuration and adjacent Ravel/tap/default Grove remain preserved.
k6 retains delivery/asset/compiler work, k12 broader prose, and k13/k8 the
later cutover/result extensions. The root remains live; no runtime change,
installation, integration/release, new driver or teardown occurred. This
session's original harness-dispatch exit/admission contract remains intact.

## Testing runner

The human selected cargo nextest. Rust test tasks use nextest and preserve its
reporting UI; formatting/lint diagnostics may be saved with short summaries.
Use focused filters during edits and the full relevant suite at handoff. The
nextest runner replaces cargo test for runtime suites; retain doctest checks
when the source contains executable documentation examples.

## Complete Grove delivery handoff — grove-ravel-delivery-k6 closed

k47-k55 and final k51 acceptance meet every k6 preparation criterion.
docs/grove-delivery-contraction.md carries the complete criterion table,
retained fixture ownership, final three-target unreleased archive digests and
current old-name reconciliation. Fresh Grove 22.1.0/Ravel 0.1.0 separate-prefix
native/QEMU lifecycle/standalone/import/confinement checks, actual private
Homebrew install/test, release pipeline and fail-closed archive/ELF controls
pass. The full gate passes 18 checks, 1,378 workspace cases, nine doctests and
six source books; all 15 ignored installed cases pass. All 1,393 expected unique
runtime identities and 42 target/route/control outcomes match individually.
All 35,359 stable frozen subjects and adjacent checkout states match. The one
shared legacy database difference has a read-only record of another
BookScanBuilder session's ending/relaunch during measurement; it was not
restored or mutated here. Complete evidence is in target/k51.

Ravel/tap/default Grove remain clean at their existing handoffs; owner configs
and the complete original 22.1.0 installation retain their bytes. Fresh original
inventory equals docs/legacy-driver-inventory.json, and the actual old-pair
scratch upgrade/unlink rehearsal passes. k12 owns remaining broader ownership
prose; k13 applies the live preservation and performs extraction integration/
release, with rebuilt tagged acceptance. No delivery preparation work remains.

k13 must prepare a fresh `.jj/legacy-driver-22.1.0` plan using
docs/legacy-driver-transition.md, compare its complete inventory, source its
release-env.sh, apply/verify the absent owned stable-PATH dispatch/helper links
and record that plan's absolute path here. The target/k51 plan is scratch
evidence, not the applied plan. Preserve the original canonical keg, complete
backup, plan/links and cleanup exclusions through final finish's last signal.
Every later release/recovery shell and final finish re-sources release-env.sh.
Never repeat a partial version cut. The existing driver stays live and this
session still ends with original `harness-dispatch exit`; no publication,
permanent installation changes, new driver or teardown occurred in k6.

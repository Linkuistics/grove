# witnessed-activity-k13


## Goal

Cut concrete, session-sized implementation leaves for witnessed-activity-k12
against the landed idle-next-k11 observation and selection seams. Preserve the
node's complete product and process-control acceptance contract.



## Context

Read the node brief and current spec, then the predecessor's implementation and
any review/integration it earned. item-status-k4 chose this boundary because
namespace discovery, captures and typed rows will already exist here. Avoid
planning them a second time from design-era source pointers.

Reconfirm graph freshness and coverage. Starting symbols are DriverLease and
launch_configured_session in grove-loop, Workspace in jj-workspace,
run/supervise/watch in keyed-launch and Viewer::refresh_at/render in grove-tui.
Existing process controls are in `crates/grove-loop/tests/driver_lease.rs` and
`crates/keyed-launch/tests/launch.rs`; application tests are in
`crates/grove-tui/tests/browser.rs`. Use the actual interfaces after idle-next-k11.

## Done when

- Before cutting implementation, resolve an authorized Linux execution route
  under the node brief's procedure. If none is available, arrange the explicit
  human-run handoff early and assign its concrete command/revision, prerequisites,
  macOS result and Linux evidence to a named leaf. That leaf stays live until
  both platform results exist; do not defer discovery to node closure.
- Ordered implementation leaves carry precise acceptance scenarios, production
  consumers and existing seams. Each can be demonstrated or verified without
  waiting for a sibling and fits a focused session. Together they supply
  end-to-end witnessed RUNNING/NEXT.
  Individually tested protocol steps are allowed even when their product
  behavior arrives only with the last step; partial metadata stays Unavailable
  and every step carries its own tests and source-derived book updates.
- Every scenario in the node brief and spec has an owner, including Linux/macOS
  real process results, forced-reuse mutation controls, multi-viewer and handoff
  races, unconfirmed reap and close order, read-only observation, tree identity,
  selection/finish, presentation and documentation.
- Producers include their own behavior tests and current documentation. Tests
  and documentation ownership includes G6 reconciliation in
  docs/specs/user-guide-coverage.md as each behavior ships. Tests
  use the production typed observer. Runner API changes preserve domain-free
  vocabulary and ordinary callers; unused witness metadata is not an independent
  product increment.
- Review placement covers the implemented concurrency protocol and evidence
  before node closure. Cut review leaves after artifacts exist, leaving
  integration to reviewers with findings. State doubts precisely enough for
  the creating producer to commission its review.

## Notes

Do not reopen the integrated kernel design without contradictory evidence or
weaken the node exit condition to make a split fit. If inspection reveals a
smaller useful product increment, name its working handoff and preserve the
successor's inherited requirements.

## Decisions (running log)

- Linux execution is available through this machine's running Docker Desktop
  Linux VM. A disposable cached `rust:1.85` container reported Linux
  6.10.14-linuxkit aarch64, Rust/Cargo 1.85.1 and overlayfs. The route uses copied
  source and container-local fixture directories, never a macOS bind mount for
  locks. Its immutable image digest and execution prerequisites belong in the
  node brief. This is route evidence only; no future witness test has run.
- Keep one product increment: runner notifications, lease-owned publication,
  verified runtime observation, then viewer binding. The earlier protocol
  leaves have real production consumers and independent process/test seams,
  while unsupported or partial activity remains unavailable. A schema-only
  milestone would deliver no working behavior. Preserve the node's complete
  exit condition, including both platform results and adversarial review.
- Graph project matches this workspace; generation is 2026-09-15T11:44:22Z.
  Targeted search and launch call tracing completed without pagination. Coverage
  marks loop_driver and keyed-launch current, but the new loop observation
  files are not tracked by that generation and viewer/lease/workspace files
  have changed. Current source reads ground those seams. idle-next-k23 already
  repaired independent runtime summaries; its accepted F3/F4 limits create no
  new repair task here.
- The native Linux route also built grove-loop and passed the existing exact
  test `driver_lease::observation::tests::descriptor_flags_and_bounded_reads_are_enforced`
  (one passed, zero failed/ignored), under uid/gid 1000:1000. Runtime source is
  unchanged from revision `720f525547a3359d1a1443d2b3ec3ac01c0815de`;
  the copied tracked-source archive's SHA-256 is
  `fb7a4db86fafe053a39bf22286f69d3659f1a7259269d0a372ad48147f35849c`.
  This confirms the route and locked dependencies compile natively. It is no
  evidence for the unimplemented witness controls that k26 must run.
- One in-session adversarial planning reviewer checked acceptance ownership,
  intermediate compatibility, session scope and the Linux route. Its sole
  finding was low severity: plain `docker run` could follow a changed ambient
  daemon. Classified valid/actionable; the recipe now explicitly selects
  `--context desktop-linux`. This is a mechanical command correction with no
  further review needed. k27 still commissions the required complete-protocol
  review after implementation; no competing review is scheduled for this plan.
- Final validation: the four implementation handles resolve and their brief
  chain is the root plus witnessed-activity-k12. `bash scripts/check.sh` passed
  all eight principal checks, including workspace tests and six final book
  validations. SHA-256 checks confirm all 1,750 tracked files outside `.grove/`
  were unchanged across the run; no implementation or durable design changed.
  Retire this planner only. witnessed-activity-k12 retains four live children,
  its complete acceptance contract and the later review obligation, so no
  parent-chain node closes in this session.

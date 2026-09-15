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

- Ordered implementation leaves carry precise acceptance scenarios, production
  consumers and existing seams. Each can be demonstrated or verified without
  waiting for a sibling and fits a focused session. Together they supply
  end-to-end witnessed RUNNING/NEXT.
- Every scenario in the node brief and spec has an owner, including Linux/macOS
  real process results, forced-reuse mutation controls, multi-viewer and handoff
  races, unconfirmed reap and close order, read-only observation, tree identity,
  selection/finish, presentation and documentation.
- Producers include their own behavior tests and current documentation. Tests
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

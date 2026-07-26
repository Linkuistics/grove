# herdr-authority-patch-k9

**Kind:** work

## Goal

Land the two-hunk authority fix on the herdr fork and ship it, so that a
`grove`-labelled state report is **accepted** by the running herdr on a pane
whose session identity is owned by the harness's own integration. This is what
unblocks `report-plumbing-k8`: until it ships, the reporter has nothing that
will accept its reports.

Repo is `~/Development/herdr` (`AntonyBlakey/herdr`, `upstream` =
`ogulcancelik/herdr`). This leaf is **fork + tap only** — the upstream PR is
`herdr-upstream-pr-k10` and must not block this.

## Context

The route, the shape, and why every alternative was rejected are in
`01-DONE-herdr-authority-route-k7.md`'s `## Decisions (running log)`. Read it
before touching code; the reasoning is not reconstructible from the diff.

The principle the patch encodes, in one line: **a hook report that makes no
session-identity claim neither conflicts with, nor clears, the identity owner.**

Both hunks are in `src/terminal/state.rs`, in `set_hook_authority_at`. Line
numbers are against upstream HEAD as of 2026-07-26 and **will have moved** —
that file took +1281/-812 in the last three months. Find them by name, not by
number.

```rust
// ~line 633 — the owner gate
let owner_conflicts = session_ref.is_some()
    && self.current_session_owner_conflicts(&source, &agent_label);

// ~line 691 — the accept path
if session_ref.is_some() {
    self.persisted_agent_session = None;
}
```

The second hunk is **required, not cosmetic**. Line 691 currently clears
`persisted_agent_session` unconditionally on every accepted report, so without
it grove's first accepted report silently destroys herdr's session-resume for
that pane — the exact harm that disqualified the "win the race" and "uninstall
the integration" routes.

Do **not** add `("herdr:grove", "grove")` to `full_lifecycle_hook_authority`.
Verified: it routes the report into `route_full_lifecycle_hook_report`'s
allowlisted branch, which requires `process_present`
(`parse_agent_label("grove") == detected_agent`, false) and then bails on
`let Some(session_ref) = … else { return Ignore }`. Every report would be
dropped. The allowlist is a stricter path, not a fast lane.

## Done when

- Both hunks are on a fork branch, rebased onto current `upstream/master`.
- `cargo test` passes. Pay particular attention to the owner/takeover tests
  (`different_owner_*`, `foreground_agent_*`, `custom_session_report_*` — around
  `state.rs:4537-4790` on the pre-rebase tree). **This is the one unverified
  assumption in the route decision**: the reasoning says a report carrying no
  `session_ref` makes no identity claim and so cannot conflict, but that was
  argued from the source, not proven by running the suite. If a test disagrees,
  the test is evidence about the patch — reconcile before shipping, do not
  weaken the test to fit.
- A **new** test covers the fix directly: a pane whose session is owned by
  `herdr:claude`/`claude` accepts a `grove`-sourced state report carrying no
  `session_ref`, and the pane's `persisted_agent_session` **survives** it.
- Formula bumped in `linkuistics/taps` and installed; `herdr --version` and the
  Cellar path show the new build.
- Verified live from a pane: report `working` as `grove`, confirm it lands
  (`revision` advances, `agent_status` changes) *with the claude integration
  installed and owning the session* — the configuration that previously dropped
  it. Then report `blocked`, confirm it lands too (this is what dissolves the
  latching hazard), then `release-agent` and confirm the pane returns to screen
  detection with its claude session ref intact.

## Notes

**Versioning discipline** (the human's constraint, and the reason to keep the
diff at two hunks): track upstream closely and often, and keep our version
number **aligned with theirs plus a suffix of our own** — the existing
`0.7.5-uilayout.b9570aa` is the pattern to follow. Every upstream release means
a rebase, so diff size is a recurring cost, not a one-off.

The fork currently carries `ui-layout` (two unsubmitted `feat:` commits) on top
of upstream. Decide whether the authority fix rides that branch or gets its own
— its own is cleaner, since `herdr-upstream-pr-k10` needs a branch containing
*only* these two hunks to submit.

`herdr pane report-agent <PANE_ID> --source … --agent … --state …` — the
positional pane id comes **first**; flags before it fail with a bare
`unknown option: <value>`.

**Scope guard**: the reporter is `report-plumbing-k8`; the upstream PR and the
mis-detection issue are `herdr-upstream-pr-k10`. This leaf ships a patched
herdr and nothing else.

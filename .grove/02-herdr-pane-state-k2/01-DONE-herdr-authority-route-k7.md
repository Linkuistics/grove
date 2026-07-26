# herdr-authority-route-k7

**Kind:** planning

## Goal

Settle how grove gets a state report **accepted** by herdr for a pane whose
session identity is already owned by the harness's own herdr integration — the
gate measured in this node's brief. Everything `report-plumbing-k8` builds
depends on the answer, so this is a route decision, not an implementation.

## Context

Read the node `BRIEF.md` first — it carries the live measurements and the exact
`state.rs` line numbers. The short version: `current_session_owner_conflicts`
vetoes any report whose `(source, agent)` differs from the pane's session-identity
owner, and herdr's claude/codex integrations claim exactly that ownership at
every SessionStart while contributing nothing to state. No unforked report gets
through, and one that lands *before* the owner appears latches the pane at a
stale state forever.

*herdr-optional-ui* named this: the fork option was rejected "for now" and
pre-committed to reopen **"if that turns out to be false"**. It has. This leaf
spends that reopening.

The user owns a herdr fork already — `AntonyBlakey/herdr`, with
`ogulcancelik/herdr` as `upstream`, checked out at `~/Development/herdr` and
currently level with `v0.7.5` on the files that matter. That materially changes
the cost of the fork route versus what planning assumed.

## Done when

A route is chosen, with the human, and recorded where it belongs — an ADR if it
clears the when-to-write bar, otherwise the node brief. `report-plumbing-k8`
is then rewritten to match the chosen route, since its shape follows from it.

## Notes

**Grilling agenda.** Candidate routes, roughly cheapest-first. Each is a real
option; the recommendation is the fourth.

1. **Do nothing unforked; ship the reporter disabled by default.** Honest, no
   herdr change, no risk of latching. But it delivers nothing the root brief's
   "Done when" asks for, so it is really a decision to abandon this node and
   let `05-herdr-grove-plugin-k5` carry the whole integration.
2. **Win the ownership race.** Have the driver claim the pane's session identity
   as `grove`/`grove` *before* launching the first harness session. Measured to
   work on a fresh pane — and it then vetoes the harness's own SessionStart
   report, permanently. Rejected on sight unless the human wants it: it is
   hostile to another integration, it silently breaks herdr's claude-session
   resume for that pane, and it depends on the mis-detection bug persisting.
   Loses outright if the pane ran an agent before `grove do`.
3. **Ask the user to uninstall the harness integration** (`herdr integration
   uninstall claude`). Then nothing owns the session and grove's reports land —
   measured. Costs herdr's session-resume for that harness, and is configuration
   guidance rather than a design. Viable as a documented escape hatch, weak as
   the primary route.
4. **Fork + upstream PR** (recommended). Two shapes, and the grill should pick
   between them rather than assume:
   - *Narrow*: add `("grove", "grove")` to the `full_lifecycle_hook_authority`
     allowlist at `src/detect/mod.rs:283`. Smallest possible diff, matches the
     seven existing pairs, plausibly acceptable upstream. But it grants grove
     **full** authority, which also suppresses screen detection entirely
     (`should_ignore_detected_state_under_full_lifecycle_hook`) — decide whether
     that is wanted or merely tolerated.
   - *General*: fix the gate itself, so a session-identity owner does not veto a
     differently-sourced *state* report. Arguably the real bug — session identity
     and lifecycle state are separate concerns that herdr conflates here — and it
     helps every third-party reporter, not just grove. Larger diff, better
     upstream story, needs care not to regress the takeover tests around
     `state.rs:4519-4670`.

**Questions the grill has to answer, whichever route wins:**

- Does grove want full lifecycle authority (screen detection off) or only
  precedence over it? The mis-detection bug argues for off — herdr is evaluating
  codex's manifest against a claude TUI on every pane measured. But off means a
  grove bug shows as a frozen pane with no fallback.
- What clears grove's authority when the loop stops? Measured: `release_agent`
  works while grove is the effective label, but cannot dislodge a foreign
  session owner. A driver that reports must also un-report on exit, or every
  `grove do` leaves a latched pane behind.
- Fork-only, or upstream PR too? If upstream, the *general* shape is the one
  worth proposing; if fork-only, the *narrow* one is a two-line carry.
- Does the mis-detection itself need fixing (MCP servers inheriting the
  harness's foreground process group)? It is upstream of everything here and
  affects far more than grove — possibly its own leaf under the root, not this
  node's business.

**Scope guard**: this leaf decides a route and records it. Writing the reporter
is `report-plumbing-k8`; touching herdr's screen manifests or the plugin is
neither.

## Corrections to the agenda above (verified against upstream HEAD, 2026-07-26)

The brief's measurements were taken against `v0.7.5`. Upstream has since
rewritten `state.rs` (+1281/-812). Re-reading changed four premises.

- **The veto survives the rewrite.** `set_hook_authority_at` still evaluates
  `current_session_owner_conflicts` (`state.rs:1215`) and returns `None` at
  `state.rs:640`; the escape still bottoms out in
  `foreground_agent_confirms_session_owner`, which still demands
  `parse_agent_label(label) == Some(detected_agent)` — `None` for `grove`. No
  unforked route exists on current upstream either.
- **Upstream now names the distinction grove needs.** `detect/mod.rs:295`
  extracts `session_identity_only_integration()`, and `("herdr:hermes","hermes")`
  moved out of `full_lifecycle_hook_authority` into it (allowlist is now **six**
  pairs). The maintainer is actively separating identity from state.
- **Route 4-*narrow* is fatal, not merely blunt.** Allowlist membership does not
  bypass the owner gate — it routes the report into
  `route_full_lifecycle_hook_report`'s allowlisted branch, which requires
  `process_present` (`parse_agent_label("grove") == detected_agent`, false) and
  then `let Some(session_ref) = … else { return Ignore }`. grove sends no
  session_ref, so **every report is dropped** before the owner gate. Adding the
  entry is strictly worse than doing nothing.
- **The fork is already production infrastructure.** `/opt/homebrew/bin/herdr` →
  `linkuistics-herdr 0.7.5-uilayout.b9570aa`, built from the fork's `ui-layout`
  branch (two unsubmitted `feat:` commits) and shipped from `linkuistics/taps`,
  the same tap grove ships from. The `v0.7.5` tag in that checkout is a *local*
  tag on the branch head, not upstream's release.
- **Upstream contribution status, corrected.** `akbash` is herdr's own agent bot
  (`akbash@herdr.dev`; dog-breed naming alongside `kangal-bot`), not the user;
  the `akbash/*` branches on the fork are mirrored upstream branches. The user
  (`AntonyBlakey`) has **zero** upstream PRs and is **not** in
  `.github/APPROVED_CONTRIBUTORS`. Upstream landed `2debcec7 fix: restrict
  unapproved pull requests to bug fixes`: unapproved contributors may open only
  `fix:`/`fix(scope):` PRs of ≤20 files and ≤1000 lines; features need prior
  maintainer approval. `approve-merged-contributor.yml` auto-approves an author
  once a PR merges.

## Decisions (running log)

**Route: fork.** Patch herdr in `~/Development/herdr` and ship it through
`linkuistics/taps`. Chosen because the fork is already load-bearing daily
infrastructure, so the marginal cost is one commit on a branch already carried
plus a formula bump — not the new cost the agenda assumed. Attached discipline
(user's constraint): **track upstream closely and often, and keep our version
number aligned with theirs plus a suffix of our own** — the existing
`0.7.5-uilayout.b9570aa` is the pattern. That discipline is what puts downward
pressure on diff size, and it is why the shape below matters more than it would
for a fork left to drift.

**Shape: the general fix, in its minimal form.** Not because it has the better
upstream story (it does) but because *narrow does not work at all* — see the
correction above. Two hunks, one principle: **a hook report that makes no
session-identity claim neither conflicts with, nor clears, the identity owner.**

```rust
// state.rs:633
let owner_conflicts = session_ref.is_some()
    && self.current_session_owner_conflicts(&source, &agent_label);

// state.rs:691
if session_ref.is_some() {
    self.persisted_agent_session = None;
}
```

The second hunk is **required, not cosmetic**: line 691 currently clears
`persisted_agent_session` on every accepted report, so without it grove's first
accepted report silently destroys herdr's session-resume for that pane — the
exact objection that sank routes 2 and 3, re-entering through route 4.

grove's report traced clean through all six gates under this patch, and
*subsequent* reports land too, which dissolves the latching hazard the node
brief measured (grove can always correct its own state).

**Authority: precedence only, not full lifecycle.** grove's reported state wins
whenever a report has landed; herdr's screen detection stays live underneath as
a fallback. The two hunks already give this, so it costs nothing; suppressing
screen detection would need the allowlisted path taught to accept a
session-ref-less agent, a larger and feature-shaped diff. Accepted trade-off:
the fallback that fills grove's silent gaps is the *mis-detected* one today
(codex manifest against a claude TUI). Wrong, but self-correcting and visibly
moving — preferred over a frozen pane with no fallback, in a tool whose whole
purpose is telling the truth about pane state. The mis-detection is a separate
bug and is not laundered through this decision.

*Correction to the reasoning above, not the decision*: "self-corrects from the
screen on a grove crash" is **false**, and was stated in error while this was
being decided. `hook_authority_is_effective` short-circuits to `true` for any
label outside the authority allowlist, so grove's authority is permanently
effective; there is no TTL (`reported_at` is only ever compared for ordering),
and the clear-on-process-exit path (`state.rs:457-465`) requires
`parse_agent_label(authority.agent_label) == agent`, `None` for grove. A dead
grove pins the pane under *both* options, so that axis never differentiated
them. Precedence-only still wins on the grounds that held: smaller diff, no
feature-shaped work, and `fallback_state` keeps updating underneath for whenever
authority is released.

**Upstream: both tracks.** Carry the patch on the fork now, so
`report-plumbing-k8` is unblocked without waiting on anyone; open the upstream
PR on an independent, non-blocking track:

> `fix: separate session identity from lifecycle state in hook reports`

Two hunks in one file, far inside the ≤20-file / ≤1000-line ceiling for
unapproved contributors, and reproducible from the CLI (`herdr pane
report-agent`). If it merges, the carry drops to zero *and*
`approve-merged-contributor.yml` adds `AntonyBlakey` to the approved list —
which is also the cheapest door to getting the fork's existing `ui-layout`
feature work upstream. If it stalls or is declined, nothing in this grove is
blocked.

**Release: on catchable exits only.** The driver releases grove's authority on
clean relaunch-stop, on `complete --done`, and on SIGINT/SIGTERM. That last part
is **new work, not free**: `src/loop_driver.rs` installs no signal handler
today, so Ctrl-C kills the driver outright with no cleanup. SIGKILL, panic, OOM
and power loss stay uncovered — the pane pins at grove's last state and the user
recovers with `herdr pane release-agent`. Deliberately *not* adding a
staleness-TTL hunk upstream: it introduces a tunable constant and changes
behaviour for every existing custom reporter, which is a materially weaker
`fix:` framing to hang on a first PR from an unapproved contributor. Keeping
that PR at two hunks and one principle is worth more than closing the
SIGKILL hole.

**Mis-detection: a herdr issue, not a leaf.** It is a herdr bug affecting every
user, it is required by none of the root brief's "Done when", and the chosen fix
masks its visible symptom on grove panes — `effective_agent_label` returns
grove's label whenever its authority is effective, so a grove pane's sidebar row
reads `grove`, not the mis-detected `codex`. Its residual effect is only on
`fallback_state` quality in the gaps after a release. By `driving.md`'s triage
this is "not ours at all" → file it upstream and drop it. The reproduction is
already in hand (`wQ:p1`, `wJ:p1`, `wP:p1` — all running claude, all detected as
codex, all `agent_status: idle` while mid-turn), which is what herdr's issue
template demands. Filing rides along with the upstream-PR leaf, since both are
outward-facing work in the same repo; it gets no leaf of its own.

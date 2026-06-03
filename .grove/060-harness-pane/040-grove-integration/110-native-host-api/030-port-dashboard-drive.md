# 030-port-dashboard-drive

**Kind:** work

## Goal

Port the **v1 `App`/`RepoView` dashboard** to render through the native pane (020)
and add **native tab/pane/focus control by direct call**, so the real grove
dashboard runs natively in-process with **no proxy socket anywhere**. This is the
node's acceptance leaf — ADR-0021 seam points (2) and the consumer-facing half of
seam point (3) — and where the host-API contract gets documented for 120–150.

## Context

- **Port, don't rewrite.** The v1 dashboard rendering (`src/tui.rs`, the `App` and
  its ratatui views) and the `RepoView` data layer cross the ADR-0013 seam
  unchanged: ratatui above, `RepoView`/shell-out writes below. Only the
  *transport* changes — the v1 alt-screen/crossterm loop and the ADR-0016 socket
  frames are replaced by drawing into the native pane's buffer (020) and receiving
  events in-process. The `suspended()` alt-screen dance for `grove-llm` writes
  goes away (it only existed because the dashboard owned the tty).
- **Native driving (subsumes proxy seam + `zellij action`).** Add the host-API
  calls grove needs to manage layout *by direct in-process call*: create/close/
  focus **tab** and create/close/focus **pane**. These replace the
  `new-tab`/`go-to-tab`/`focus-pane-id` `zellij action` shell-outs (ADR-0015/040)
  and the dumb proxy/controller (`src/dash/`, ADR-0016) — all now dead code to
  remove or mark superseded. Keep the surface **minimal**: only what the v1
  dashboard exercises here; 120 (nav, tab open/switch) and 150 (working-set panes)
  will pull more as they need it.
- **`ClientOsApi`/`ServerOsApi` substitution (ADR-0021 seam point 2)** lands here
  to the extent the dashboard needs grove-specific OS/terminal behaviour; stock
  impls otherwise.
- **Document the seam.** Write the host-API contract at the framework↔grove crate
  boundary — the first cut of what an extracted `trellis` publishes, and the
  contract 120–150 build on. Prose + the public Rust signatures; not a GraphQL/
  network surface (out of scope, ADR-0020 §5).
- **Retire the superseded machinery.** With the native dashboard live, remove (or
  clearly tombstone) `zellij::launch`, `src/dash/` (proxy/controller/proto/decode),
  the `__dash-proxy`/`__dash-controller` CLI verbs, and the now-unused seam-frame
  codec. The shelved `harness-pane` and `grove-nav` crates stay as recorded
  fallback/history per their ADRs.

## Done when

- `grove tui` renders the **real v1 grove dashboard natively in-process** as a
  trellis pane — same master/detail behaviour as v1 (grove list, task tree,
  brief/inbox/leaf panes), no proxy socket, no `zellij action`, no WASM.
- grove creates/closes/focuses tabs and panes by **direct host-API call**.
- The host-API seam is **documented** at the framework↔grove boundary.
- Superseded transport (proxy seam, controller, `zellij action` launch) is removed
  or tombstoned; `cargo build`/`cargo test` green; grove core below the ADR-0013
  seam stays `ratatui`-free.
- This closes the **110 node** acceptance ("a host app renders a native ratatui
  surface as a trellis pane, receives input/events in-process, and drives
  pane/tab/focus by direct call — the v1 dashboard runs this way").

## Notes

- 110's node "Done when" is met when this leaf retires; on retire, promote the
  host-API contract from this leaf's notes into the documented seam (and a glossary
  entry / ADR if a durable decision surfaced), then re-check the parent chain.
- Within one repo only — cross-repo fleet is 070-fleet-view, which reuses this
  driving layer opened cross-repo.
- Keep the API discovered-by-consumer (constraint 4): if the v1 dashboard doesn't
  need a call, don't add it for hypothetical 120–150 use.

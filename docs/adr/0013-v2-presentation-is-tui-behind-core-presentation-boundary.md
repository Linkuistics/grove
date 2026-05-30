# v2 presentation is the Ratatui TUI, behind a core↔presentation boundary; web is deferred, not rejected

A web front-end (browser + `grove server` over the same harness backend) surfaced
as an alternative to extending the v1 Ratatui TUI for grove's v2 (multi-repo fleet
view + embedded harness). We **build v2 as the TUI**, and keep all grove logic —
the `RepoView`/`MultiRepoView` data layer, harness driving, and shell-out writes —
**below an explicit presentation-agnostic boundary**, so a future web server is a
*second presentation over the same core*, not a rewrite. Web is a deliberately
preserved option, not a rejected one.

## Status
accepted

## Why the TUI now
The comparison is recorded in full in
`docs/research/web-frontend-vs-tui-presentation.md`. In short: web's two primary
draws are real but narrower than they look. **Richness** lands in grove's
*dashboard chrome* (task trees, diffs, fleet tables), **not** the harness view —
which is terminal emulation either way. **Remote access** the harness backend
already has (any tmux/ssh/mosh path, or whatever D2 settles), so web adds remote
reach to the *dashboard*, not the harnesses. Against those localized wins, web
imposes decisive costs on four load-bearing grove values: walk-away-ability (one
binary vs server + browser + JS build + port), persistence simplicity (a new
long-lived server is where the prior-art failure surface concentrates),
dependency budget (forces `tokio`/async — the deferred concern 4/070 refactor — as
a *precondition*, not an addition), and near-term effort (the TUI extends v1; web
adds a whole parallel surface). The one clear web win on a primary driver — the
fleet view — is buildable in a TUI (k9s/gh-dash prove dense multi-entity TUIs
work), so it does not overturn the rest.

## The boundary is the durable output, and it is mechanism-agnostic
Recording "TUI now" alone would be unremarkable. The decision worth pinning is the
**boundary**: grove's core logic stays free of presentation types so the
presentation can be swapped or doubled. The v1 data layer (`src/repo_view.rs`) is
already presentation-agnostic — read-only, returns plain structs — and is the
proven example. The rule for v2: ratatui rendering lives *above* the seam; the
data layer, the harness-driving code, and the shell-out writes live *below* it and
never import `ratatui` types. Enforcement is by **module placement and review**,
not a speculative `GroveBackend` trait — the only v2 consumer is the TUI, so a
trait designed for a non-existent web consumer would almost certainly be
over-fit (grove constraint 4: lazy/optional). The seam is proven the day a second
presentation calls the same modules.

Crucially, **the harness-embedding mechanism (an owned tmux server vs an
in-process pty embedded in ratatui) is deliberately *not* decided by this ADR.**
That is decision D2, reopened during 040's grilling once grove's crash-resilience
was reaffirmed to come from the artifacts-over-state model (spine constraint 1),
not from tmux session persistence — which demoted tmux's headline advantage. D2 is
resolved empirically by the `050-spike-embed-pty-harness` leaf. Whichever
mechanism wins, it sits **below** this boundary; the presentation decision recorded
here stands either way.

## Considered options
1. **Build the TUI now, keep web behind the boundary (chosen).** Additive to v1,
   no async refactor, one binary; web stays a cheap-to-open second presentation
   because the core is already presentation-agnostic.
2. **Go web now (browser + `grove server` + WebSocket + xterm.js).** Rejected for
   v2: forces the async refactor as a precondition and adds a long-lived server,
   auth/TLS/port surface, and a JS build — large costs against grove's
   single-binary, walk-away ethos, for primary-driver wins that are localized to
   the dashboard chrome. Reconsider when a fleet/dashboard whose richness the TUI
   genuinely can't meet is proven *and* async is wanted for other reasons.

## Consequences
- 060 (harness pane) and 070 (fleet view) target the TUI on the existing sync
  stack. The concrete harness mechanism waits on the 050 spike (D2).
- 080 (async-revisit) narrows to a recorded finding: sync suffices for the TUI;
  *web ⇒ async* is the entry toll, paid only if/when web is chosen. (If the 050
  spike picks in-process pty, async reopens on different grounds — pty output
  juggling — which 050 will flag.)
- The "no `ratatui` types below the seam" rule is a standing review check.

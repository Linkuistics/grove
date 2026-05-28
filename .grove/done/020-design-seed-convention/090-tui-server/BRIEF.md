# 090-tui-server — brief

## Goal

Deliver v1 of the grove TUI navigator: a per-repo, read-oriented surface
that answers *"what groves exist in this repo, what is each grove's live
task tree, what is sitting in each inbox?"* — plus a single write action
(`grove inbox add` via shell-out) so dogfooding the seed-capture
convention is one keystroke away.

Multi-repo aggregation, embedded harness panes, and the tmux/zellij
multiplexer choice are explicitly deferred — captured as a seed at
`<repo>/.grove-meta/inboxes/tui-multi-repo-and-multiplexer/` during this
planning session (the dogfooding instance the parent brief called for).

## Decisions settled in grilling

| Concern | Decision | Rationale |
|---|---|---|
| Entry point | `grove tui` subcommand on the existing `grove` binary | Composes with the existing CLI crate. One install. Mirrors `grove status` as a read-oriented diagnostic. |
| Stack | Ratatui + crossterm, **sync** event loop | v1 has no concurrent IO that demands async. Tokio would be paid-for-nothing now and the wrong abstraction risk is real (we don't yet know what tmux events look like). The view code is identical sync-or-async, so migration later is cheap. |
| Filesystem-watch | `notify` crate, polled from the sync loop, 200ms debounce | Standard pattern; no async needed. |
| View shape | Two screens, master/detail | Grove list → grove detail; left pane is the task tree, right pane cycles through leaf content / inbox / `BRIEF.md`. Pre-select the current grove if launched from inside a worktree. |
| Navigation | Arrow keys + j/k; Enter drills; Esc/q pops; `/` filters; Tab cycles right-pane mode; `?` for help | Conventional Ratatui-app keybindings. |
| Write actions | `c` only — shells out to `grove inbox add`, prompting for target grove | One-shot CLI write, no pane needed. Walk-away-ability preserved (every mutation goes through a grove verb, no direct file edits). |
| Launch (`d`) | **Not in v1** | Launching a harness requires a multiplexer architecture we have not designed. Captured in the seed. |
| Factoring for future | Data layer behind a `RepoView`-style abstraction | So the future `MultiRepoView` is additive, not a rewrite. The factoring is the only concession v1 makes to the deferred concerns. |

## Done when

- `grove tui` subcommand exists, launchable from any worktree, and
  delivers the two-screen master/detail UX described above.
- Filesystem-watch refresh works against both `.grove/` and the relevant
  `.grove-meta/inboxes/<name>/` directories.
- `c` shells out to `grove inbox add` and returns to the TUI on
  completion.
- The data layer is wrapped behind a `RepoView` abstraction (name is
  judgement — see leaf 010); single-repo is one instantiation.
- All three child leaves are retired into `done/`; this node retires
  when none remain live.

## Decomposition

Three work leaves, ordered to keep each downstream leaf unblocked. The
order also matches natural review checkpoints — data first (testable
without UI), then the UI shell (testable without writes or fs-watch),
then the interactive bits.

- `010-data-layer.md` — scan repo for groves, count
  leaves/inboxes/retired, expose via a `RepoView`-style API. No UI; no
  Ratatui dep yet.
- `020-tui-shell-read-only.md` — Ratatui app skeleton, two screens,
  navigation, master/detail rendering. Consumes the data layer. No
  writes, no fs-watch — the TUI is launchable as a static-snapshot
  read-only navigator after this leaf.
- `030-writes-and-fs-watch.md` — `c` keybind shelling to
  `grove inbox add`, plus `notify`-based debounced refresh of the data
  layer. Final leaf; v1 ships at retirement.

## Pointers

- Seed for the deferred concerns:
  `.grove-meta/inboxes/tui-multi-repo-and-multiplexer/` on the
  `grove-meta` branch (use `grove inbox show
  tui-multi-repo-and-multiplexer` to view).
- Methodology constraint: `.claude/skills/grove/SKILL.md` constraint 6
  (walk-away-able) — the TUI must never edit grove state directly; every
  mutation goes through a `grove` verb.
- ADRs that define the state the TUI reads:
  `docs/adr/0002-grove-meta-branch-and-inbox-model.md`,
  `docs/adr/0003-cross-repo-inbox-handoff.md`,
  `docs/adr/0004-inbox-as-directory-of-observation-files.md`.
- The seed-capture prior-art (`docs/research/seed-capture-prior-art.md`)
  is **not** load-bearing here — no surveyed paradigm addresses TUI
  visualisation. The TUI design space is constrained only by
  walk-away-ability and process correctness.

## Notes

- **No PRD.** The grilling did not surface a human-facing decision worth
  a PRD; v1 is a small UX layer over already-documented state. If a
  child leaf surfaces such a moment (e.g. an unexpected interaction
  shape), it raises one.
- **No ADR.** None of the v1 decisions clear the ADR bar — they are
  reversible-ish (Ratatui to Cursive would be painful but not
  catastrophic; subcommand to separate binary is a `clap` reorg), or
  unsurprising (sync event loop, `notify` for fs-watch are defaults).
  The deferral of multiplexer choice *might* warrant an ADR when the
  future grove starts, but that's that grove's call.
- **Node name (`090-tui-server`) is inherited from the pre-decompose
  leaf.** The "server" suffix was speculative; v1 is decidedly not a
  server. Renaming the node now would churn cross-references for no
  payoff — the name retires with the node into `done/` once the three
  children ship. The future grove (per the seed) gets a name that
  reflects its actual scope.

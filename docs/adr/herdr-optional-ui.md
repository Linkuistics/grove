# herdr is an optional UI over grove's artifacts, never a dependency

grove **optimises for** [herdr](https://herdr.dev) — an agent multiplexer whose
sidebar rolls per-pane state up to tabs and workspaces — without ever requiring
it. Two mechanisms, deliberately separated:

- **Semantic state** (`idle` / `working` / `blocked`) is reported *by grove* over
  herdr's socket, addressed by the `HERDR_*` variables herdr places in the pane
  environment. Detection is skipped entirely when they are absent, and a refused
  or slow socket is a no-op, never a failed launch or a stalled loop.
- **Everything richer** — the task tree, the live leaf, progress — is rendered by
  a **herdr plugin that reads `.grove/` directly**. grove pushes it nothing and
  does not know it exists.

A grove with no herdr present behaves identically, minus the status surface.

## Why it binds

The split falls out of constraint 1: *the directory tree under `.grove/` is the
only state*. Because the tree on disk already **is** the status, a UI over it
needs no cooperation from grove at all — no status protocol, no push, no second
representation to drift. That is what makes "optimised-for" achievable rather
than aspirational: the plugin and the binary version independently, and neither
can break the other.

The one thing artifacts cannot supply is *semantic state at a moment in time* —
whether the agent is mid-turn, or has stopped and is waiting for a human. That is
knowledge grove holds and nothing on disk records, so it is the only thing grove
reports. Keeping the boundary exactly there is what stops the integration
metastasising into a dependency.

The alternative — herdr as grove's execution substrate, spawning panes per leaf
and sequencing through the socket API — would be more capable and would amend the
spine. Constraint 6 (*walk-away-able*) is stated about artifacts, but a loop that
assumes panes exist is a broken grove without herdr, not a plainer one. That is a
change to grove's contract, not a feature, and it is not this decision.

## Considered options

- **Push status to herdr as structured metadata** (`pane.report_metadata`
  tokens for the leaf, kind, progress). Rejected as the *primary* channel: it
  duplicates on the wire what `.grove/` already holds, adds a reporting
  obligation to every tree mutation, and drifts the moment a verb forgets to
  report. Still fine as a thin display-only garnish on the pane's sidebar row.
- **Fork herdr to make grove a first-class agent.** herdr's *full lifecycle
  authority* is a compiled-in allowlist of `(source, agent)` pairs, so joining
  it genuinely requires a fork or an upstream PR. This was rejected on the
  reading that an unrecognised agent label gives the same practical result
  unforked — and **reopened, because measurement showed it does not**. The label
  does bypass both gates that reading identified, but a third one,
  `current_session_owner_conflicts`, drops any report whose `(source, agent)`
  differs from the pane's session-identity owner — which the harness's own herdr
  integration claims at every SessionStart. There is no unforked way through it,
  and a report that lands before the owner appears latches the pane at a stale
  state. Measured live against herdr 0.7.5; the route is being settled now.
- **herdr as execution substrate** (panes per leaf, sequencing over the socket).
  Rejected as a spine change, not a feature — see above. Reopened only as a
  deliberate amendment to the constraints, with constraint 6 restated.
- **A grove plugin that reports state as well as rendering it.** Not possible:
  plugin v1 declares argv commands in a manifest, cannot register actions at
  runtime, and cannot add socket methods or join the authority allowlist. A
  plugin has exactly the socket access `grove-llm` already has, so routing state
  through one would add a hop and buy nothing.

## Consequences

- grove reports as agent **`grove`**, not as the harness it launched. This is
  honest — a `grove do` pane is a loop relaunching a sequence of sessions, and
  the harness may vary per leaf — and it sidesteps herdr's gate that silently
  drops a report whose label names a different *known* agent than the one it
  detected.
- The plugin's only contract is the `.grove/` directory scheme
  (*task-tree-scheme*), which is already published and stable. Changing that
  scheme is now also a plugin-compatibility question.
- No grove verb gains a reporting obligation; `leaf-add`, `leaf-retire` and the
  rest stay pure working-tree changes.
- Nothing here is load-bearing for the loop. If herdr, the socket, or the plugin
  is absent or broken, `grove do` is unaffected.

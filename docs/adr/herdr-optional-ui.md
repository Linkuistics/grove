# herdr is an optional UI over grove's artifacts, never a dependency

grove **optimises for** [herdr](https://herdr.dev) — an agent multiplexer whose
sidebar rolls per-pane state up to tabs and workspaces — without ever requiring
it. Two mechanisms, deliberately separated:

- **Semantic state** (`idle` / `working` / `blocked`) is reported *by grove* over
  herdr's socket, addressed by the `HERDR_*` variables herdr places in the pane
  environment. Detection is skipped entirely when they are absent, and a refused
  or slow socket is a no-op, never a failed launch or a stalled loop. Stock herdr
  **drops** these reports; landing them needs a two-hunk patch grove carries in a
  fork (see below).
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
- **Join herdr's authority allowlist.** Adding `("herdr:grove", "grove")` to the
  compiled-in `full_lifecycle_hook_authority` list is the obvious fork, and it
  **does not work**: allowlist membership is not a fast lane past the owner gate,
  it is a stricter path *before* it. An allowlisted report must satisfy
  `route_full_lifecycle_hook_report`, which demands the label parse to the
  detected agent (`grove` parses to nothing) and then requires a `session_ref`
  grove does not have — so every report would be dropped. Rejected as strictly
  worse than doing nothing. Reopen only if grove ever gains a resumable session
  identity of its own, which would be a different tool.
- **Report unforked, on an unrecognised label.** Rejected on measurement. An
  unrecognised label does bypass the agent-label gate and the screen-blocker
  override, but a third gate, `current_session_owner_conflicts`, drops any report
  whose `(source, agent)` differs from the pane's session-identity owner — which
  the harness's own herdr integration claims at every SessionStart. There is no
  unforked way through, on 0.7.5 or on current upstream, and a report that lands
  before the owner appears latches the pane at a stale state forever. Reopen if
  herdr ever accepts a state report from a source that owns no session identity.
- **Claim the pane's session identity first, or have the user uninstall the
  harness integration.** Both make grove's reports land, and both work by
  destroying herdr's session-resume for that harness — hostile to another
  integration in the first case, a configuration demand rather than a design in
  the second. Rejected. The uninstall remains usable as a documented escape
  hatch for anyone unwilling to run the patched build.
- **herdr as execution substrate** (panes per leaf, sequencing over the socket).
  Rejected as a spine change, not a feature — see above. Reopened only as a
  deliberate amendment to the constraints, with constraint 6 restated.
- **A grove plugin that reports state as well as rendering it.** Not possible:
  plugin v1 declares argv commands in a manifest, cannot register actions at
  runtime, and cannot add socket methods or join the authority allowlist. A
  plugin has exactly the socket access `grove-llm` already has, so routing state
  through one would add a hop and buy nothing.

## The patch, and what it costs

herdr conflates two separable concerns: who owns a pane's *session identity*
(for resume) and who reports its *lifecycle state*. `set_hook_authority_at` lets
the identity owner veto any differently-sourced state report, and clears the
identity record on every accepted one. grove carries a two-hunk fix encoding one
principle — **a hook report that makes no session-identity claim neither
conflicts with, nor clears, the identity owner** — gating both behaviours on
`session_ref.is_some()`. grove never sends a `session_ref`, so it reports state
without touching identity, and the harness's own session-resume survives intact.

The fork is shipped from `linkuistics/taps` alongside grove itself, versioned as
upstream's version plus a local suffix, and tracked closely against upstream —
the discipline that keeps a recurring rebase cheap, and the reason the patch is
held to two hunks rather than a more thorough refactor of the same seam. The
same patch goes upstream as a `fix:` PR; if it lands, the carry ends.

grove takes **precedence** over screen detection, not authority *instead of* it:
staying outside herdr's allowlist keeps `fallback_state` live underneath. The
known gap is that grove's authority has no expiry in herdr, so a driver killed
uncatchably (SIGKILL, panic, OOM) pins the pane at its last reported state until
`herdr pane release-agent`. Accepted rather than fixed with a staleness TTL,
which would change behaviour for every third-party reporter and weaken the
upstream PR's framing as a bug fix.

## Consequences

- grove reports as agent **`grove`**, not as the harness it launched. This is
  honest — a `grove do` pane is a loop relaunching a sequence of sessions, and
  the harness may vary per leaf — and it sidesteps herdr's gate that silently
  drops a report whose label names a different *known* agent than the one it
  detected.
- **The status surface, not the loop, is what depends on the patched build.**
  Under stock herdr the reports are dropped and the pane falls back to screen
  detection — the same outcome as no herdr at all, which is exactly what this
  ADR already promises. The optional-UI claim therefore survives the fork: it
  now reads *a grove with no herdr, or with stock herdr, behaves identically,
  minus the status surface.*
- grove must **release** its authority on exit, because herdr will never expire
  it. That obligation is real code in the loop driver, including signal
  handling it did not previously have.
- The plugin's only contract is the `.grove/` directory scheme
  (*task-tree-scheme*), which is already published and stable. Changing that
  scheme is now also a plugin-compatibility question.
- No grove verb gains a reporting obligation; `leaf-add`, `leaf-retire` and the
  rest stay pure working-tree changes.
- Nothing here is load-bearing for the loop. If herdr, the socket, or the plugin
  is absent or broken, `grove do` is unaffected.

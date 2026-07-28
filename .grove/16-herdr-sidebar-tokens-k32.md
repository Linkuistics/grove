# herdr-sidebar-tokens-k32

**Kind:** design

## Goal

Decide whether a `grove do` pane's **sidebar row** should carry the current leaf —
`pane.report_metadata` tokens such as `$grove_leaf` / `$grove_kind` — and, if so,
which side reports them.

## Context

- Raised as an open question inside **herdr-grove-plugin-k5**, which decided it does
  **not** belong there: the plugin renders a pane by *reading disk*, whereas tokens
  are a **push** over the socket. Different surface (the always-visible sidebar row
  vs. a summoned pane), different mechanism, so it is its own decision.
- Also not settled by **herdr-pane-state-k2**: that one carries `idle`/`working`/
  `blocked`, which is the knowledge *no artifact holds*. The live leaf is the exact
  opposite — `.grove/` already records it, so pushing it duplicates on the wire what
  is already on disk. ADR *herdr-optional-ui* pre-judged this as "still fine as a thin
  display-only garnish", not as the primary channel; this leaf decides whether the
  garnish is worth its cost.
- What the mechanism actually is, from herdr's own docs (verified 2026-07-28):
  `pane.report_metadata` is **display-only** — it cannot take lifecycle state from an
  integration, so none of the authority machinery applies. Tokens are per-resource
  patches: a string sets a key, JSON `null` clears it, ≤16 keys per report, ≤32
  retained per pane, values capped at 80 chars, optional per-key `ttl_ms`. **Token
  metadata is not restored after a server restart.**
- The cost that decides it: a token renders **only if the user has put `$name` into
  `[ui.sidebar.agents] rows` in their own `~/.config/herdr/config.toml`**. So this is
  not "grove reports and it appears" — it is a configuration demand on the user, of
  the kind ADR *herdr-optional-ui* rejected in a neighbouring case.

## Done when

The decision is recorded — as an amendment to *herdr-optional-ui* if it changes what
that ADR already says about metadata, otherwise as a note plus (if adopted) an `impl`
leaf. Either outcome is fine; a well-argued **no** retires the question for good and
is the cheaper answer.

## Notes

Three candidate reporters, with the obvious objection to each:

- **The loop driver**, beside its existing state report. Cheapest — it already peeks
  the leaf every iteration to route harness and model, so the value is in hand and
  costs no extra work. But it puts a *display* obligation into the loop, and the
  driver only knows the leaf at launch: nothing re-reports after a mid-session
  `leaf-add`, so the row goes stale within the session that grows the tree.
- **A grove verb** (`leaf-add` / `leaf-retire` / …). Rejected in advance by
  *herdr-optional-ui*'s consequence "No grove verb gains a reporting obligation" —
  reopening that is a change to the ADR, not an application of it.
- **The plugin**, via a `[[startup]]` hook or an `[[events]]` hook. Keeps the push
  out of grove entirely and is the only option that costs grove nothing — but plugin
  v1 hooks fire on *herdr* events, and no herdr event fires when a file under
  `.grove/` changes, so freshness would need a daemon the plugin has nowhere to run.

Worth weighing against a fourth option that needs no tokens at all: the pane is
already legible, and `terminal_title` (OSC 0/2) is a channel grove's *harness* writes
without anyone's configuration.

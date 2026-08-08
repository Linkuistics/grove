# remove-herdr-integration-k92

**Kind:** impl

## Goal

Remove Grove's Herdr-specific runtime integration and the bundled Herdr plugin.
Leave replacement status/reporting mechanics out of this grove unless they are
separately requested and scoped.

## Context

- The operator has explicitly decided not to keep investing in the Herdr path
  and intends to use a different mechanism later.
- The current surface includes `src/herdr.rs`, launch/driver/reporting hooks,
  session configuration, tests, documentation, packaging, and `herdr-plugin/`.
- Preserve the self-driving completion signal and other lifecycle behavior that
  is independent of Herdr.

## Done when

- Runtime and CLI behavior contain no Herdr-specific reporting, pane, socket,
  hook injection, agent-hint, or configuration path.
- `herdr-plugin/` and its packaging/documentation references are removed.
- Pending Herdr-only Grove work is pruned under this explicit scope decision;
  mixed work items and briefs retain their non-Herdr deliverables without Herdr
  acceptance criteria.
- Active product documentation and tests describe the remaining lifecycle
  without proposing or implementing a replacement status mechanism.
- `cargo fmt --check` and `cargo test --locked` pass.

## Notes

Treat historical terminal leaves as records, not active product documentation.

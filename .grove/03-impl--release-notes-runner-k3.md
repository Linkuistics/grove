# release-notes-runner-k3

## Goal

Make the standalone release-notes command usable with the user's actual personal
Grove policy and prove the complete invocation in its confinement boundary.

## Context

- `release-notes-k2` supplies the command, writing skill and documented runtime
  interface; read the resulting implementation before configuring it.
- `~/.config/grove/config.kdl` currently defines interactive Codex and Claude
  commands and profiles, with no release-notes route. Inspect the effective
  configuration again rather than assuming those details are unchanged.
- `docs/CONFIGURATION.md` records an earlier successful confined headless setup.
  Recheck actual installed CLI capabilities before reusing its version-specific
  arguments or paths.

## Done when

- A `release-notes` route resolves to a working headless command in personal
  configuration, preserving existing routes, profiles and unrelated settings.
  Harness, model and permission choices follow existing user preferences and
  remain personal policy.
- Required runtime reads are explicit and writable harness state stays beneath
  the standalone invocation directory. Credentials are not committed or exposed
  in documentation, test fixtures or transcripts.
- One live invocation through `task release:notes`, in a disposable jj fixture
  with a prior release and multiple unreleased changes, returns valid notes,
  incorporates existing Unreleased text and preserves historical sections and
  jj topology. Inspect the actual output and the visible runner result.
- The live invocation uses the configured route, the shipped writing
  instructions and Grove's mandatory confinement. It neither starts a Grove
  task tree nor signals the parent session's control channel.
- Any runtime helper or documentation changes needed for the working setup are
  durable and verified. Document how another user supplies equivalent personal
  policy without imposing this machine's model, paths or credentials.
- The root brief's done criteria hold across both implementation leaves; report
  any remaining specific gap as work rather than marking the grove finished.

## Notes

The human agreed to this live smoke test as the complement to deterministic
fake-runner integration tests. Keep it a focused invocation, not a paid test
suite. No release is cut or published by this leaf.

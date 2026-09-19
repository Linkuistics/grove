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

## Decisions (running log)

- The human rejected a wrapper under `~/.config/grove`: helper scripts stay with
  the shipped artifacts. It is `scripts/release-notes/codex-headless.sh`, staged
  as an `--input` like the skill, so personal policy names
  `/bin/bash codex-headless.sh …` with no checkout path and the sandbox needs no
  grant for it. Shipping it in the Homebrew archive was declined: the formula
  installs two binaries and nothing else.
- The human approved `--sandbox danger-full-access` inside Grove's confinement,
  as `docs/CONFIGURATION.md` already recorded for Codex 0.155.1.
- Personal policy gained command `release-notes-codex` (model `gpt-6-astra`,
  effort `medium`, the existing Codex preferences), binding `notes-writer` and a
  `release-notes` route in the shared `routes` profile. Grants come from
  `GROVE_RELEASE_RUNTIME_READ`: the credential file and four Caskroom files.
- Live smoke (one invocation, disposable fixture, three unreleased changes):
  the hand-written Unreleased entry was kept, two user-facing changes added, the
  internal tidy omitted, `v1.2.3` and the footer byte-identical, jj topology
  unchanged, no parent signal file written, no credential text in either
  transcript, invocation directory removed.

## Notes

The human agreed to this live smoke test as the complement to deterministic
fake-runner integration tests. Keep it a focused invocation, not a paid test
suite. No release is cut or published by this leaf.

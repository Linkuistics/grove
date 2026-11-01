# ravel-smoke-sandbox-home-k60

## Goal

Move Ravel's real-harness smoke tasks to the narrow sandbox home. Each harness
then sees only its own sign-in store from the real home, and fake-harness
controls prove each refusal. `common-loading-acceptance-k29` relies on this
home for its actual trials.

## Context

- Ravel `docs/specs/configuration-evaluation.md` at `822c0ed2`: "Sandbox home
  for real harness runs".
- Ravel `docs/skill-discovery-design-evidence.md`: "Harness observations" and
  the two facts that the local stage establishes.
- Ravel `scripts/harness-smoke.sh` (`link_store` and `sandboxed`) and
  `scripts/harness-smoke.test.sh`.

## Done when

- Before any script change, the session reads the installed Claude Code on
  Linux and records the sign-in path `.claude/.credentials.json`. It also
  checks the installed Claude Code against the rows of "Harness observations".
  The Ravel evidence document records both results with the versions.
- The sandbox home links exactly one path for each harness, as the table of
  the specification states. No link covers the whole `.claude` or `.codex`.
  Codex runs set `CODEX_HOME` to the sandbox `.codex`.
- A Claude Code run refuses before the harness launches when it inherits
  `CLAUDE_CONFIG_DIR`, `CLAUDE_CODE_PLUGIN_CACHE_DIR`,
  `CLAUDE_CODE_PLUGIN_DIRS` or `CLAUDE_CODE_PLUGIN_SEED_DIR`. A Codex run
  refuses an inherited `CODEX_HOME` in the real home. The run removes and sets
  none of these variables.
- Each run lists every link before the launch and records the effective
  configuration directory. After the run, it checks that each linked path is
  still a link.
- `task harness:smoke:test` runs a fake-harness control for each refusal. A
  decoy real directory proves the `CLAUDE_CONFIG_DIR` refusal.
- The configuration specification, the book and `CHANGELOG.md` agree with the
  scripts. `task harness:smoke:test` and `task check` pass. The opt-in real
  smoke tasks run only with the owner's consent, because they spend
  subscription usage. The bookmark advances by jj, and nobody pushes it.

## Notes

This leaf does not depend on the skill resolver. It follows the confinement
leaf only so that the delivery leaves stay contiguous. Do not change the
owner's real `.claude` or `.codex`.

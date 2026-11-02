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

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing. Ravel `main` is still
  `24a545236185`, an ancestor of the line, so no rebase was necessary. The
  line starts at k59's `e343587583eb`.
- Linux sign-in path, measured before any script change. Claude Code 2.1.316
  (installer, arm64 container from `ravel-native-linux:arm64`, binary sha256
  `55cdb8e1…`) builds the store path as `join(store dir, ".credentials.json")`.
  The store directory is `CLAUDE_SECURESTORAGE_CONFIG_DIR` when set, else the
  configuration directory. With a fake token, `auth status` reports signed in
  for a regular file, for a link from a sandbox `.claude/.credentials.json`, and
  for the file under `CLAUDE_CONFIG_DIR`. It reports signed out when
  `CLAUDE_CONFIG_DIR` names a directory without the file.
- The binary also holds a second store reader that opens the file with
  `O_NOFOLLOW` and maps `ELOOP` to `refused-symlink`. A gate selects it, and the
  minified code does not show which. `auth status` in 2.1.316 does not use it.
  The evidence records this as a risk for a later build, not as a measurement.
- macOS rows, Claude Code 2.1.316 (binary sha256 `64c327e9…`). Source read: the
  keychain service is `Claude Code-credentials`, with `-<8 hex of sha256>` of the
  directory when `CLAUDE_CONFIG_DIR` or `CLAUDE_SECURESTORAGE_CONFIG_DIR` is set.
  A keychain-with-plaintext-fallback store reads `.credentials.json` of the
  configuration directory when the keychain holds no item. The three plugin
  variables are read as the table states. Skills with `loadedFrom`
  `syncedSkills` exist, and the directory name is `synced`. `auth status` in a
  temporary home with the keychain link reports signed in. With
  `CLAUDE_CONFIG_DIR` added, it reports signed out. All five rows hold.
- Fifth variable. `CLAUDE_SECURESTORAGE_CONFIG_DIR` moves only the sign-in
  store, on both platforms, and not the configuration. An inherited value
  makes the run sign in from a store that the link list does not show. The
  smoke run refuses it with the four names, and the spec names it. This is
  conservative and costs the owner nothing.
- Codex 0.166.0: `login status` with `CODEX_HOME` at a sandbox `.codex` that
  holds only a link to the real `auth.json` reports signed in. It writes
  `tmp/` beside the link and leaves the link in place.
- Out of scope, recorded only: token variables such as
  `CLAUDE_CODE_OAUTH_TOKEN` and the host gateway's `CLAUDE_CODE_HOST_CREDS_FILE`
  are other sign-in routes. They move no configuration.
- Script order. The refusals come first, before any launch of the harness.
  The old script ran `--version` in the real home, so the version probe now
  runs in the sandbox, after the links. A Claude Code run refuses the five
  names when present, also with an empty value (`${!name+x}`, which bash 3.2
  supports). A Codex run refuses a `CODEX_HOME` whose physical path is the
  real home or below it. An empty value refuses, because it selects the
  default. A value outside the real home gives way to the sandbox `.codex`,
  and the run reports that. The run removes none of the names.
- Links. Each harness links one path: `Library/Keychains`,
  `.claude/.credentials.json` or `.codex/auth.json`. A missing store refuses
  with the sign-in remedy. The run lists every link with `find -type l` and
  fails unless the list is exactly the store. It prints the configuration
  directory. `check_links` runs after the version probe, after the sign-in
  check and after the session, also when the session failed.
- The real sign-in check of Claude Code compares the `configDirectory` of
  `auth status` with the sandbox `.claude`, by physical path. The first
  version compared strings and failed on macOS, because `TMPDIR` ends in `/`
  and Claude Code normalises `//`. The sandbox path now drops the trailing
  slash too. A mutation that compares with the sandbox home fired against the
  real Claude Code. No fake reaches this check, so it has no automatic
  control.
- Tests. Every run uses a fake real home, and a digest proves that no run
  changes it. The fakes record each launch, including `--version`, so each
  refusal proves that no launch happened. The controls cover the five Claude
  Code names, an empty `CLAUDE_CONFIG_DIR`, a decoy real directory that stays
  unchanged, `CODEX_HOME` in the real home, through an alias and empty, a
  value outside the real home, a replaced link for each harness and a home
  without a store. The guard that the link list holds only the store has no
  control, because no case makes a second link.
- Firing controls on the final script, each alone, restored from a copy, with
  both script digests the same before and after (`6808a801…` and
  `4906e569…`). M1 to M6: no refusal loop, and each name dropped. M7: no
  Codex refusal. M8: a lexical path, which fails at the first Codex case
  because macOS `TMPDIR` is under the alias `/var`. M16: lexical on both
  sides, which fails at the alias case alone. M9: empty is not the home. M10:
  a launch before the refusals. M11 and M12: link `Library` or `.codex`. M13:
  no `CODEX_HOME`. M14: no link check. M15: a write into the real home. Each
  failed its fixture.
- Linux. `harness-smoke.test.sh` passes in `ravel-native-linux:arm64` with
  the final copy, which proves the `.claude/.credentials.json` link. With
  the real Claude Code 2.1.316 and a fake token, the script lists the one
  link, passes the sign-in check and stops at a missing Ravel binary on
  purpose. An inherited `CLAUDE_CONFIG_DIR` refuses there too. No real call
  ran on either platform. The opt-in real tasks still need the owner's
  consent.
- Repair of an earlier step. k59 removed the `confine` argument of
  `skills::launch::prepare`, but the chapter 8 excerpt kept it, so
  `task docs:excerpts` failed at the entry base. This leaf removes the line
  from the excerpt. That keeps the line green, and it costs less than a leaf.
- Documentation. The spec names the fifth variable, the order of the checks,
  the `CODEX_HOME` rules and the `configDirectory` comparison. The evidence
  adds "Measured by the local stage". Chapter 4 gets a table of the three
  links, which also keeps its prose run under 450 words, and the stop
  conditions. `CHANGELOG.md` gets two items.
- Checks. `task harness:smoke:test` passed. `task check` passed with 715 of 715
  tests, and its fake smoke step passed. `docs:ste:test`, `docs:breaks:test`,
  `docs:excerpts:test`, `docs:excerpts`, `book:integrity` and
  `docs:book:build` passed. Strict `ste` and `breaks` found no problem in
  chapter 4. No reviewer: the executable controls cover each claim of the leaf.
- Output. The Ravel commit is `ba0f388ecb6373bd6df380c16d5c66b9d3b63dcc` on
  the local bookmark `skill-directory-delivery`, above k59's `e343587583eb` and
  main `24a545236185`. Nobody pushed it.

# owner-switch-rehearsal-k255

## Goal

Execute the prepared real plan once in a disposable copy of HOME, including
full restore. Retain a readable, content-identified result and its
comparison with the real inputs.

## Context

- The section "The rehearsed plan" of `docs/specs/owner-skill-delivery.md`
  states the rule. The switch consumes prepared values, not a passing token
  or a record of an earlier run.
- owner-switch-dry-run-k271 supplies the dry-run instrument and the
  comparison command. owner-switch-plan-k254 supplies the prepared plan.
- The plan's `accepted` holds real digests. The dry run therefore resolves
  with the accepted 2.0 build against a copy of the real kegs. The promoted
  `ravel` exists only inside finish, so the result declares the prefix
  substitution from `ravel@2`.
- The dry run runs the real `mac-check` with HOME set to the disposable
  HOME.
- owner-marketplace-freeze-k253 holds Claude Code's version. Sign-in reuse
  follows the staging contract of owner-staging-setup-k248.
- core-candidate-restaging-k300 found a limit of the native probe. In the
  `codex-native-bare` case, Codex searched HOME and read `grove-planning`
  from the frozen `grove@linkuistics` copy in Claude Code's plugin cache.
  The probe's `kindSkill` observation counts that copy as native. Only the
  `listing` observation failed, so the probe still failed. The section
  "Core candidate restaging" of `docs/owner-skill-delivery-evidence.md`
  holds the records.
- owner-switch-plan-k254 prepared the plan, SHA-256 `abb8ab18…`, in
  `~/.local/state/owner-switch/prepared`. Its `INSTRUCTIONS.md` names the
  comparison command with the result
  `~/.local/state/owner-switch/dry-run/result.json`. Retain the result
  there, or change the instructions and prepare again. The trial commands
  run a frozen staging runner in `prepared/staging`. The section "Owner
  switch plan" of `docs/owner-skill-delivery-evidence.md` names each digest.
- On 2026-11-11, 21 leftover tool processes and 11 that need review would
  stop the switch. They include the orphaned Ravel fixture PID 27230, which
  also makes the real `native-off` refuse. The section "Processes that stop
  the switch now" names them. The owner ends them before the dry run.

## Done when

- The dry run executes the ordinary writes, hook commands, probes, probe
  cleanup, final comparison and full restore. It also runs `native-off`
  without a switch backup. The real HOME is unchanged.
- The result's input table identifies each input by content, before and
  after the run. The hook commands ran with the held Claude Code version.
- The comparison command, run against the real prepared inputs, reports
  each row as equal or as a declared equal-content prefix substitution.
- The result holds per-probe outcomes, the restore comparison and its
  limits. It is committed, and it is also readable outside `.grove/`.
- The evidence document states when another dry run is necessary. A
  changed plan, source digest, program, helper, command input or tool
  version requires one.

## Notes

discovery-rollout-k12 records each accepted digest again with the published
Ravel archives. A differing digest first needs that source's staging trials
and revised acceptance, then another dry run.

The dry run is reproducible, so a new run after a change costs little.
Never edit a result by hand.

owner-switch-dry-run-k271 delivered the instrument as
`scripts/owner_switch_dry_run.py`, with `task owner:switch:dry-run` and
`task owner:switch:compare`. Its module docstring states the input table,
the copied layout and the declared prefixes. Declare the candidate keg and
`ravel@2` with `run --prefix grove-keg=DIR --prefix ravel=DIR`. Pass sign-in
files and the plugin cache with `--include` as the staging contract needs.
The comparison compares the row `tool:grove` by content and by
`grove --version`. A candidate keg that was built before the release bump
reports another version than the released keg. Inside finish, that row then
differs, so finish needs another dry run against the released keg, or a
candidate keg built at the release commit. The section "Owner switch dry
run" of `docs/owner-skill-delivery-evidence.md` records this limit.

## Decisions (running log)

- 2026-11-11: The first session stopped before the dry run. The process
  check `owner_switch.py processes` still refused. Its list held PID 27230
  and its `harness.sh` child 27248. It also held 15 Codex app-server
  processes of finished trials and two debug `grove` builds. Stale
  `uv run pytest` processes, an orphaned `ugrep` and the owner's own
  processes were also on it. The owner
  chose to restart the computer and then rerun Grove. Run the process
  check again before the dry run.
- 2026-11-11: The real Homebrew prefix cannot pass gates 4 and 6. Its
  `bin/ravel` is 1.1.0, and Grove 22.8.0 has no keg layout. The dry run
  therefore needs a disposable Homebrew prefix and a `brew` wrapper first
  on `--path`. The prefix holds a copy of the real kegs, the candidate keg
  as `Cellar/grove/<version>` with `opt/grove`, and the accepted 2.0 build
  as `Cellar/ravel@2/2.0.0` with its `share/ravel/installation.json`.
  `brew --prefix`, `--cellar` and `info --json=v2` must answer for that
  prefix. Gate 5 also runs `mise -C HOME bootstrap packages status --json`.
  `scripts/owner_switch_rehearsal.py` (`fixture_prefix`, `cmd_stub_brew`)
  and `docs/evaluations/owner-skill-delivery/settings_drafts.py` and
  `default_sources.py` hold the fixture forms to adapt. The candidate keg
  is `~/.local/state/owner-staging/candidate`. The 2.0 build is
  `~/.local/state/owner-staging/ravel-2.0`.
- 2026-11-11: After the restart, `owner_switch.py processes` reported
  "clear". Only this session's Grove, Ravel and Claude Code ancestors were
  on its list.
- 2026-11-11: The stand-in prefix is
  `~/.local/state/owner-switch-dry-run/prefix`, which
  `docs/evaluations/owner-skill-delivery/dry_run_prefix.py build` makes.
  Homebrew refuses a prefix in `/private/tmp`. It takes the repository's
  Cellar when the repository holds one, so `Homebrew/` holds links to the
  real `bin/brew` and `Library` only. Homebrew counts no link out of its
  Cellar as a keg, so each keg is an APFS clone. `mise` reads only the real
  `/opt/homebrew`. Each clone therefore keeps its real version, and the
  candidate is 22.8.0, as is the installed Grove. All ten sources resolved
  from the clones with their accepted digests.
- 2026-11-11: The rows of the Homebrew tools, the kegs and the sources
  name the stand-in prefix. The comparison therefore passes only with the
  same `--path`. Against `/opt/homebrew` those rows differ until finish
  makes its own dry run against the released prefix.
- 2026-11-11: `run --sign-in` links the two sign-in stores of the staging
  contract, `Library/Keychains` and `.codex/auth.json`. A copied login
  keychain would be a second, locked keychain.
- 2026-11-11: The first run, `run-20261111T075753Z`, did not pass. Its
  switch stopped at step 3 before any write. The baseline instrument reads
  `~/.codex/config.toml`, `~/.codex/hooks.json`,
  `~/.local/bin/codebase-memory-mcp` and `~/.quint-llm-kit/repo`, which the
  plan does not name. Each run therefore copies these with `--include`. It
  also copies `.claude.json`, `.codex/plugins`, `.codex/AGENTS.md` and
  `.codex/instructions.md`, which the real probes read. The source HOME
  stayed unchanged.
- 2026-11-11: The `undeclared` report counts the prepared program as
  declared. The plan names it as `$program`, and the probes import it.
- 2026-11-11: The second run, `run-20261111T080458Z`, passed step 3 and
  stopped at step 4 before any write. Homebrew 7 refused the untrusted tap,
  because its trust store `~/.homebrew/trust.json` was not copied. The real
  `~/.claude.json` also changed, and it changed again after the run. This
  session's Claude Code writes it, so no run copies `.claude.json`. The
  probes run without the owner's user-level MCP servers.
- 2026-11-11: The third run, `run-20261111T080944Z`, passed steps 3 and 4
  and stopped at step 5 before any write. `mise -C OUT/home` searches each
  parent folder. With OUT inside the real HOME, it read the real
  `~/.config/mise/config.local.toml` as an untrusted project file. Claude
  Code reads `.claude/skills` in each parent folder too. Each run's OUT
  therefore lies in `/private/tmp/owner-switch-dry-run`, outside the real
  HOME, mode 0700, on the same volume for the clones. The result is copied
  to `~/.local/state/owner-switch/dry-run/`.
- 2026-11-11: The fourth run, `run-20261111T081407Z`, stopped at step 5
  before any write. `mise` evaluates a cask under `sandbox-exec` with the
  first `ruby` on PATH. With the disposable HOME, it did not recognise the
  real `~/.local/share/mise/shims` as its own, chose the shim and was
  refused. Each run therefore starts with that folder removed from PATH.
  Only `ruby`, `node` and `npx` then resolve differently, to Homebrew's
  copies. No tool row changes.
- 2026-11-11: The fifth run, `run-20261111T081847Z`, passed native-off,
  attempt and the whole switch, with the receipt "switched". Restore wrote
  every entry back, and all 328 restore items were equal. Restore exited 4
  only because `codex-native` failed `listing`. Codex 0.167.0 names a
  linked skill after the plugin folder of its target, so the real
  `.agents/skills` links show as `grove:grove-planning`. The Codex listing
  pattern of `scripts/harness_trial.py` dropped every name with a colon.
  The real restore would fail the same way. This leaf repairs the pattern,
  with `CodexSkillListingTest`. After the switch, Codex listed no Grove,
  Linkuistics, TestAnyware or quint-llm-kit skill, so the switched probe
  hid nothing.
- 2026-11-11: The adapter is a plan input, so the plan is drafted again
  and prepared again, and the dry run runs again.
- 2026-11-11: `run` refuses an OUT inside HOME, with a refusal control.
- 2026-11-11: The sixth run, `run-20261111T083910Z`, passed every step,
  and all three native probes passed. Its per-probe outcomes still lacked
  the switch's probes. The instrument read them from the 20,000-character
  tail that the result keeps, and the real switch prints far more. The
  instrument now reads the whole output, with the control
  `dry-run-long-output`. A seventh run gives the result.
- 2026-11-11: The seventh run, `run-20261111T085127Z`, passed with all six
  probe outcomes. The comparison through the stand-in exited 0 with 163
  rows. The comparison with the real prefixes differs in 14 rows and the
  `retained` path, as expected before finish. Finish runs its own dry run
  after the release and the promotion. The result is retained in
  `~/.local/state/owner-switch/dry-run` and in `docs/owner-cutover/dry-run-*`.

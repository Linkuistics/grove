# staging-guidance-probe-k278

## Goal

Run the Claude Code trial that disables the plugin of its own guidance.
Then close this node's evidence and reconcile "Not measured".

## Context

- The node brief states the trial. "The final session and new launches"
  and "The switch inside finish" of `docs/specs/owner-skill-delivery.md`
  give its purpose: finish reads its guidance from absolute plugin-cache
  paths, and the switch disables that plugin.
- The staging HOME holds no superseded plugin. Install the guidance plugin
  only in a disposable copy, or remove it with Claude Code's own commands
  afterwards. The retained staging HOME must end with its original content.
- "Not measured" of `docs/owner-skill-delivery-evidence.md` lists open
  measurements.

## Done when

- A Claude Code session loads its guidance from a plugin and disables that
  plugin. It then launches a nested probe. After the probe, it reads its
  guidance again by absolute path.
- The evidence reconciles each item of "Not measured" that a discovery
  trial of this node measured.
- The evidence section of this node states each command that the operator
  reruns after a released source changes.

## Notes

A failure is a finding for the owner design or for a pack, not a reason to
weaken a case.

## Decisions (running log)

- The case runs in a disposable clone of the retained staging HOME, at
  `OUT/home`, made with `cp -c`. The retained HOME never holds the guidance
  plugin. Three Claude Code files of the staging HOME name its absolute
  path: `settings.json`, `installed_plugins.json` and
  `known_marketplaces.json`. The runner rewrites that path in the clone and
  refuses when another Claude Code file of the clone still names it. The
  clone drops `.cache/ravel`, so Ravel takes new snapshots there. The
  runner digests the plugin subjects of the retained HOME before and after.
- The guidance plugin is `grove@linkuistics`, from the GitHub marketplace
  `Linkuistics/grove` that the owner's registry names. Its `main` was
  `238c3a839ab4` on 2026-11-09, the owner's installed version. Claude Code's
  own `plugin marketplace add` and `plugin install` install it in the clone.
- The guidance is `grove:grove`, the spine that finish also loads, through
  the native Skill tool. After the disable, the agent loads
  `grove:configure-grove` through the Skill tool. The record keeps that
  result without a judgement, because it measures the running session's
  loaded set, an item of "Not measured".
- The nested probe is a `ravel start-session` of the accepted 2.0 build. The
  runner plans its whole argument vector and an allow-listed environment
  without `RAVEL_*` names, and the agent runs `staging_trial.py probe` on
  that plan. The verdict requires Ravel exit 0, the result `probe-ok` and a
  native listing without a superseded name.
- The reread is a read of the plugin-cache `SKILL.md` by its absolute path
  after the probe. Its result must hold the skill's `name:` line and first
  heading. The runner also requires the installed version directory
  unchanged without `.in_use`, with no `.orphaned_at`, still registered and
  disabled.
- `trials/guidance-claude-1` passed on its first run. The Skill tool of the
  running session loaded `grove:configure-grove` after the disable, so the
  session kept its loaded set. "Resource lifetime" of the owner design now
  cites the case for its two claims.
- Claude Code's `plugin marketplace add` cloned the GitHub shorthand through
  SSH. OpenSSH reads the account's real `~/.ssh`, so a staging HOME does not
  isolate SSH. The evidence records this as a finding and a limit.
- `bash scripts/check.sh` failed only `every_grove_name_in_the_tree_is_classified`
  for `GROVE_ROWS` in `scripts/owner_switch_rehearsal.py`. This leaf did not
  change that file, and classify-grove-rows-name-k279 owns the repair. The
  other 17 checks passed, `task owner:staging:trial-tests` among them.

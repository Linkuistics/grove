# inventory-guardrail-loaded-dir-k288

## Goal

Make the inventory's sole-skill rule for `guardrail@personal` read the
plugin directory that Claude Code loads, not only the plugin-cache copy.

## Context

- `inventory.py` accepts an enabled `guardrail@personal` only when the
  `personal` marketplace is registered from `/opt/homebrew/opt/personal`
  and the plugin's `skills/` holds only `guardrail`. It reads `skills/` of
  the `installPath` in `installed_plugins.json`, which is
  `~/.claude/plugins/cache/personal/guardrail/unknown`.
- staging-claude-native-trial-k285 measured Claude Code 2.1.322 in the
  staging HOME. Its init event loaded `guardrail@personal` from
  `/opt/homebrew/opt/personal/libexec/plugins/guardrail`, the directory
  that the marketplace's `marketplace.json` names. The guardrail hook ran
  from that directory. The cache copy had the same tree digest on
  2026-11-10, so the rule's verdict was right for that HOME.
- A later personal keg could add a skill to the loaded directory. Claude
  Code would then load it natively, but the rule would still read the older
  cache copy and accept the plugin. The exception would then be wider than
  "Concurrent writers" of `docs/specs/owner-skill-delivery.md` allows.
- The origin rule names the fixed path `/opt/homebrew/opt/personal`, so the
  fixture HOME of `controls.py` cannot supply that directory. Decide how a
  control reaches it before the change.

## Done when

- The rule refuses a loaded marketplace directory whose plugin holds a
  skill other than `guardrail`, and still refuses the same in the cache
  copy.
- `controls.py` has a case for each refusal, and a mutant without the new
  check fails a control.
- `task owner:staging:inventory` passes against the staging HOME, and its
  evidence records the new `inventory.py` digest.

## Decisions (running log)

- The rule resolves the loaded directory as Claude Code does. It reads
  `.claude-plugin/marketplace.json` under the registered origin and joins the
  `guardrail` entry's string `source` to that origin. It checks that directory
  and the cache copy with the same sole-skill test.
- The controls reach the fixed origin without a knob in the instrument. Each
  case runs a copy of the instrument in which the literal
  `/opt/homebrew/opt/personal` names that case's fixture keg. The shipped
  instrument therefore has no input that widens the origin.
- The source is the plugins reference of Claude Code,
  <https://code.claude.com/docs/en/plugins-reference>. Its section "How each
  key combines with its default location" says that a manifest `skills` key
  adds to the default `skills/` scan. Its section "Marketplace entries and
  the manifest" says that the `skills` of an entry append to it. The rule
  therefore also refuses a `skills` key in either plugin manifest or in the
  marketplace entry. It refuses a loaded directory outside the origin, a
  non-directory source and a marketplace with no `guardrail` entry.

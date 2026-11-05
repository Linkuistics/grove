# writing-craft-removal-k211

## Goal

Remove `technical-communication`, `writing-code-walkthroughs`, the hooks of
the `linkuistics` plugin and the moved assets from Grove. Reconcile every
Grove consumer in the same change, so that Grove stays green with
WritingSkills as the owner. Then publish the pack commit that records the
removal commit. This is the contraction step of the move.

## Context

- `docs/specs/pack-membership.md`: "The always-on hooks", "The Grove assets
  of moving craft skills" and "The acceptance of each move".
- `docs/specs/external-pack-contract.md`: "Distribution" and "The checker
  without a copy".
- The WritingSkills sections of `docs/repository-moves-record.md` from
  k208, k209 and k210: the moved assets, the plugin names and the commit
  that the trial accepted.
- engineering-removal-k204 and editorial-removal-k206, with their record
  sections, as the models.

Consumers at planning. Before you rely on this list, enumerate the tokens
again over the whole tree with `--hidden`. Use positive and cross-tree
controls. The tokens are both skill names, `tc-hook`, `tc_hook`,
`ste-lint`, `pi-extension`, `.agents/plugins` and each moved asset's path.

- `plugins/linkuistics/hooks/hooks.json` leaves. No hook in Grove names a
  moved script. `plugins/linkuistics/.claude-plugin/plugin.json` drops the
  walkthrough and writing words from its description and keywords.
- `scripts/check.sh` drops the checker self-test, the Pi load test and the
  `tc-hook` and Pi test paths of its shell lint.
- `plugins/install.test.sh` asserts the real Pi extension link. That is a
  named read of a moving file, so it takes a fixture.
- `.agents/plugins/marketplace.json` leaves, because the pack root holds the
  Codex marketplace file.
- `Taskfile.yml` drops the two `design:technical-communication` tasks.
- `plugins/linkuistics/PROVENANCE.md` drops the two moved sections.
- `.claude-plugin/marketplace.json` pins the accepted pack commit for
  `writing`, and it lists the Claude Code hooks plugin from the same
  commit. The dependency of the writing plugin resolves only when this
  marketplace lists the hooks plugin. The descriptions of the marketplace,
  of `writing` and of `linkuistics` change to match.
- The documents that state where the skills, the hooks or the moved records
  live: `plugins/README.md`, `plugins/CONTEXT.md`, `CONTEXT-MAP.md`,
  `docs/USAGE.md` and `docs/ARCHITECTURE.md` among them. A Grove file that
  stays and links a moved record links its public URL.
- The citation registry of `plugin_fallback.rs` keeps its two
  `technical-communication` rows. They read Grove's own files.

## Done when

- Grove holds neither skill, no `linkuistics` hooks file and no moved asset.
  The enumeration above is clean, and the session saw each control fail.
- The off file still turns off every always-on layer. The pack's checks
  show it for the hook core and Pi, and k210 showed it in the native route.
  No Grove record still claims to own that rule.
- `task check:packs` passes with the new pin, and `bash scripts/check.sh`
  passes on the Grove line.
- The pack's `GROVE_COMMIT` names the removal commit. That pack commit is
  on the public remote. Its `task check` passes from a clean checkout, with
  the public `GROVE_REPOSITORY` and no override.
- `docs/repository-moves-record.md` records both roots and the pack
  commit. It records the Git tree ID and the `resourceDigest` of
  `plugins/writing`, both from a clean checkout. It names the hooks plugin
  and the tool requirements: `python3` and the shell of `tc-hook`. It
  records the asset classification. It also gives skill-marketplace-k185
  one result. The Codex plugin stays listed in the pack root, unless Codex
  resolves a source outside it.

## Notes

The owner's native plugins stay at Grove commit `238c3a839ab4` from the
public `main`. Do not install, update or remove a plugin in the owner's
home. Rollout makes the native change.

`personal-pack-k183` removes the rest of the `linkuistics` plugin. Leave
its other four skills and their checks unchanged.

This is the node's last planned leaf. Its retirement closes
writing-craft-pack-k181 if no live leaf remains, by the four close steps.

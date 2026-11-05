# writing-craft-pack-plan-k182

## Goal

Cut the implementation leaves of writing-craft-pack-k181.

## Done when

- Each Done when of the parent brief belongs to a leaf with a concrete,
  verifiable outcome.
- Each handoff leaves Grove's `bash scripts/check.sh` green and the always-on
  layers working in this session's harness.
- The hooks-plugin trial has its own leaf.

## Decisions (running log)

- The node is one move with four leaves. The expand comes first, in two
  pack commits: the craft root with its assets and checks, and then the two
  hooks plugins. The native trial of the hooks plugin follows. The Grove
  removal and the last pack commit come last. Each pack commit leaves the
  product working, because Grove's marketplace still pins `694140a18dc7`,
  and the owner's native plugins stay unchanged.
- The trial comes before the removal. The owner design says to repair the
  layout if Claude Code lists the root `SKILL.md`. A repair before the
  removal changes only the pack. A repair after the removal would need one
  more pack commit and one more Grove pin change.
- The always-on layer of this session comes from the owner's plugin cache
  at Grove commit `238c3a839ab4`, from `linkuistics@linkuistics` on the
  public `main`. No change on this workstream's line reaches that cache.
  Each leaf therefore keeps the layer working if it does not install,
  update or remove a plugin in the owner's home. Each leaf states that
  limit.
- Planning finding: the brief said that executable bits stay. Grove's index
  holds the three scripts with mode `100644`, and the owner's plugin cache
  holds them without an executable bit. The hooks run `/bin/sh tc-hook`, and
  `SKILL.md` runs `python3 scripts/ste-lint.py`. The node brief now says
  that the Git modes stay as Grove records them. The working tree also holds
  `scripts/__pycache__`, which the capture must exclude.
- The second-plugin boundary is new, so this move owes its static proof and
  its trial. It owes no directory trial and no craft-root trial.
  design-pack-k175 proved the craft root, and the membership row asks for
  neither.
- No Grove test reads the bytes of either moving skill. The citation
  registry of `plugin_fallback.rs` reads Grove's own files, which cite
  `technical-communication` by bare name, so its two rows stay. The pack
  needs no composition check like `doubt-composition.sh`, unless the expand
  finds that the skill's text quotes a Grove phrase.
- The removal changes five named reads. Three are checks of
  `scripts/check.sh`: the `tc-hook` lint, the checker self-test and the Pi
  load test. The fourth is the real-corpus Pi assertion of
  `plugins/install.test.sh`. The fifth is the Codex marketplace
  `.agents/plugins/marketplace.json`.
- The parent's Done when maps to the leaves as follows. The craft root, the
  roots, the resources and the Git modes belong to k208. The assets that
  test 2 or 3 moves, with their checks, also belong to k208. The hooks
  plugins, the dependency, the static count and the Codex marketplace file
  belong to k209. The trial, the off file in the native route and any
  layout repair belong to k210. The removal, the Grove marketplace entries
  and the last pack commit belong to k211. The record of the roots, the
  commit, the tree ID, the digest, the hooks plugin and the tool
  requirements also belongs to k211.
- No in-session reviewer and no `review-planning` leaf. The subtree follows
  the pattern of four finished moves, and the membership specification had
  its review. The trial leaf answers the one open layout question with an
  observation.

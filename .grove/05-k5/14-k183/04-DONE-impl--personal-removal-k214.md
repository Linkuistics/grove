# personal-removal-k214

## Goal

Remove `plugins/linkuistics` from Grove: the four skills, the manifest, the
provenance file and the marketplace entry. Reconcile every Grove consumer in
the same change, so that Grove stays green with PersonalSkills as the owner.
This is the contraction step of the move.

## Context

- `docs/specs/pack-membership.md`: "The Grove assets of moving craft
  skills" and "The acceptance of each move".
- `docs/specs/external-pack-contract.md`: "Distribution" and "The checker
  without a copy".
- The PersonalSkills sections of `docs/repository-moves-record.md` from k212
  and k213: the moved files, the plugin names and the accepted commit.
- writing-craft-removal-k211 and its record section, as the model.

Consumers at planning. Before you rely on this list, enumerate the tokens
again over the whole tree with `--hidden`. Use positive and cross-tree
controls. The tokens are the four skill names, `plugins/linkuistics`,
`linkuistics@linkuistics`, `linkuistics:`, `guardrail-hook` and
`assumes-personal-setup`.

- `.claude-plugin/marketplace.json` drops the `linkuistics` entry. It lists
  both PersonalSkills plugins from the accepted commit. The dependency
  resolves only when this marketplace lists `guardrail`. The marketplace
  description and its plugin count change to match.
- The install commands `/plugin install linkuistics@linkuistics` in
  `README.md`, `plugins/README.md`, `plugins/grove/README.md`,
  `plugins/install.sh` and `scripts/templates/grove.rb.tmpl`.
- `plugins/README.md` and `plugins/CONTEXT.md`: the personal skill table,
  the `guardrail` text and the claim that `machine-environment` carries
  `assumes-personal-setup` here. engineering-pack-k177 left that claim for
  this move to correct.
- `docs/ARCHITECTURE.md` names `plugins/linkuistics/` as a plugin
  directory. `CONTEXT-MAP.md` names `using-jujutsu`.
- The comment of `plugins/install.sh` that names the shipped plugin
  directories. The fixtures of `plugins/install.test.sh` use the synthetic
  name `linkuistics`, and they read no real file. Decide whether to rename
  them, and record the result.
- `scripts/harness-trials/cases/*/case.json` name `plugins/linkuistics` as a
  source. They are evidence at their recorded commits, by test 3. The
  supplier check refuses them at the removal commit, as the earlier
  removals recorded. `scripts/harness-trials/sources/README.md` makes a
  present-tense claim about `plugins/linkuistics`, which is stale.
- The citation rows of `plugin_fallback.rs` and the assertion of
  `commit_guidance.rs` read Grove's own files, and they stay.

## Done when

- Grove holds no `plugins/linkuistics`. The enumeration above is clean, and
  the session saw each control fail.
- `task check:packs` reports the directory-only check for the `personal`
  source with the new pin. It reports `guardrail` by the same rule, or the
  record says why not. `bash scripts/check.sh` passes on the Grove line.
- `claude plugin validate .` accepts Grove's marketplace.
- The record names each consumer and its disposition. A Grove file that
  stays and links a moved file links its public URL.
- No personal source is under `.config`, and nothing recreates chezmoi. The
  record says that this move changes no machine setting. Rollout adds the
  mise entry of the checkout, by order 6 of the cutover contract.
- `docs/repository-moves-record.md` records the root `craft`, the pack
  commit, the Git tree ID and the `resourceDigest` of `plugins/personal`.
  Both come from a clean checkout of the published commit. The record names
  the `guardrail` plugin and its commit. It also names the tool
  requirements: `jj`, `mise`, `task` and the Obsidian manual.
- The record gives moves-handoff-k187 the exact source root and revision of
  the `personal` source and of the `guardrail` plugin.

## Notes

If a pack file claims delivery through Grove's `linkuistics` plugin, correct
it in a last pack commit, and pin that commit. Otherwise the pin stays at the
commit that k213 accepted. Planning found that the pack needs no
`GROVE_COMMIT`, so no pack commit has to record the removal commit.

`.claude/settings.json` is the owner's local permission. It stays in the
working copy and in no commit.

The owner's native plugins stay at Grove commit `238c3a839ab4` from the
public `main`. Do not install, update or remove a plugin in the owner's
home. Rollout makes the native change.

This is the node's last planned leaf. Its retirement closes
personal-pack-k183 if no live leaf remains, by the four close steps.

## Decisions (running log)

- The README of PersonalSkills says in the present tense that Grove's
  `linkuistics` plugin delivers `guardrail`. Two install passages state the
  removal as a future event. A last pack commit corrects these three
  passages, and Grove pins that commit. It changes no file under `plugins/`,
  so the tree IDs and the `resourceDigest` stay.
- The fixtures of `plugins/install.test.sh` keep the synthetic plugin label
  `linkuistics`. The installer scans every plugin directory, so a fixture
  with two plugin directories still tests that rule. A renamed label proves
  nothing more, by the fixture-label rule of `pack-membership.md`. Only the
  comment that calls the fixture the shape of this repository changes.
- `scripts/harness-trials/cases/*/case.json` stay unchanged as records of
  their builds, by "The trial cases that name moving skills".
- `docs/USAGE.md` stays unchanged. Its directory route never listed a
  `linkuistics` source, so it needs no `personal` entry for parity.
- `personal-pack-k183` closes with this leaf. Its `Done when` holds, and the
  parent brief receives four facts for later work. `repository-moves-k5`
  keeps live leaves, so the cascade stops there.

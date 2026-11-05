# design-native-namespace-k200

## Goal

Prove that a native Claude Code installation of the `design` plugin lists a
craft skill from `craft/` under the plugin's namespace. The manifest entry
`"skills": ["./craft/"]` adds that directory to the default `skills/` scan.

## Context

- `docs/specs/pack-membership.md`, the row "The first `craft/` root" in "The
  acceptance of each move".
- `docs/common-loading-acceptance-record.md`, "Requirements native namespace",
  as the model. The trial runner field `nativePlugins` installs plugins from
  the marketplace.
- The removal commit from design-removal-k198, whose marketplace lists
  `design` with a `git-subdir` source.

## Done when

- An isolated Claude Code HOME adds the marketplace from a plain checkout of
  this workstream's line at the removal commit. It installs the `design`
  plugin from that marketplace. Record whether the dependency installed the
  `grove` plugin. The `skills` key is absent from the Ravel settings, so only
  the native route supplies skills.
- Claude Code lists the four craft skills as `design:<name>` and the three
  kinds as `design:grove-<kind>`. It lists no bare copy of them. Record the
  listing.
- A Grove launch of a core kind whose task needs the craft skill of
  design-craft-trials-k199 loads that skill as `design:<name>` before the
  action that it governs.
- The record names the marketplace commit, the installed plugin revisions,
  the Ravel build, the Grove build that renders the mandate and the
  observations. A gap is a named gap with a disposition, never a pass.
- `docs/repository-moves-record.md` links the record section.
- The owner's real Claude Code installation is unchanged.

## Notes

Codex has no native skill loader in the measured version, so this proof is
for Claude Code only. A real run spends subscription usage.

Reuse the planted tree and task of design-craft-trials-k199 where they fit,
as requirements-native-namespace-k194 reused its sibling's tree.

This is the node's last planned leaf. Its retirement closes design-pack-k175
if no live leaf remains, by the four close steps. The close promotes the
facts that later craft moves need to the parent brief. They are the
installer's craft-only fixture for personal-pack-k183 and the case models for
a craft trial.

## Decisions (running log)

- The human asked for a stop before `skill-marketplace-k185`, because they
  are not sure that they want a skills marketplace. The stop is the
  `requirements` leaf `skill-marketplace-decision-k201`, inserted before that
  node. A requirements session resolves through a live human, so the loop
  stops there. The leaf prunes nothing without the human's confirmation.
- The case is `design-native-namespace`. It reuses the planted tree of
  `design-craft-discovery`, so the task and the craft skill are those of
  design-craft-trials-k199. It installs only `design@linkuistics`, so the
  record shows whether the dependency brings `grove`.
- The supplier check could not hold this case. The `impl` kind skill comes
  from the dependency `grove`, and `requires` accepted only a source. The
  runner now follows the `dependencies` of a native plugin's manifest for
  kind skills. A `requires` entry can name a native plugin. The new control
  `test_a_native_plugin_dependency_supplies_its_kind` fails against a runner
  without either change. The record's section "The runner" and the spec's
  supplier table state both changes.
- A plugin that names no supplier now refuses once, also when a declared
  skill and a kind skill both need it.
- The check holds the two required paths at `84fba18c1f98`. At
  `c800d311a541` it refuses, because that marketplace has no `design` entry.
- Run 1 loaded `design:cli-tool-design` before it wrote `notes.py`. The
  runner then exited 1, because its plugin inventory read only `skills/`.
  The inventory now also reads each root that the manifest's `skills` field
  adds. The control `test_a_plugin_with_a_craft_root_accounts_for_its_copies`
  fails against the old inventory.
- Run 2, with the repaired runner, exited 0. Its agent wrote `notes.py`
  without a load or read of the craft skill. The listing was the same in
  both runs: four `design:<name>` craft skills, three `design:grove-<kind>`
  kinds and no bare copy.
- The miss is a named gap of the native route, not a pass. Disposition: an
  accepted limit of the native route. The pack controls the listing, the
  namespace and the files, and both runs prove them. The owner's delivery
  uses the directory route, where the craft trials activated the skill in
  both harnesses. The case did not run again, because a selected pass would
  hide the miss.
- The owner's real `~/.claude` had the same digests before and after the runs.
  The task commit leaves out the uncommitted `.claude/settings.json`, which
  this session did not write.
- `bash scripts/check.sh` passed with all 21 principal checks, and
  `trial:controls` ran 77 tests.

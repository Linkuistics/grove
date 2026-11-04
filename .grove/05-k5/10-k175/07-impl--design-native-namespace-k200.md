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

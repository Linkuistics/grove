# research-native-namespace-k161

## Goal

Prove that a native Claude Code installation loads a moved research kind
under the plugin's actual namespace from Grove's bare-name mandate.

## Context

- `docs/specs/codex-skill-provisioning.md`, "Launch and compatibility" and its
  test seam "Native names after a pack move".
- `docs/specs/external-pack-contract.md`, the plugin name and the
  marketplace entry.
- `crates/grove-loop/tests/prompt.rs`,
  `the_prompt_names_no_fixed_plugin_namespace`. The current mandate names no
  plugin namespace.
- The `explicit-plugin` and `explicit-native-*` rows of
  `docs/common-loading-acceptance-record.md`, as models.

## Done when

- An isolated Claude Code HOME adds the marketplace from a plain checkout of
  this workstream's line at the removal commit. It installs the research
  plugin and the `grove` plugin from that marketplace. The `skills` key is
  absent from the Ravel settings, so only the native route supplies skills.
- A Grove `research-a` launch with the bare-name mandate loads the kind under
  its actual plugin namespace. The agent reads the family procedure and the
  spine before it acts.
- The record names the marketplace commit, the installed plugin revisions,
  the Ravel build and the observations. A gap is a named gap with a
  disposition, never a pass.
- The owner's real Claude Code installation is unchanged.

## Notes

Codex has no native skill loader in the measured version, so this proof is
for Claude Code only. A real run spends subscription usage.

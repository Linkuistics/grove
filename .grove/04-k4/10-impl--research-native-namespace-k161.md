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
  this workstream's line at the removal commit. It installs the `research`
  plugin from that marketplace. Record whether the dependency installed the
  `grove` plugin. The `skills` key is
  absent from the Ravel settings, so only the native route supplies skills.
- A Grove `research-a` launch with the bare-name mandate loads the kind as
  `research:grove-research-a`. The agent loads the spine and the
  `research-family` skill before it acts, under their actual namespaces.
- The record names the marketplace commit, the installed plugin revisions,
  the Ravel build and the observations. It also names the Grove build that
  renders the mandate. That build must state the missing-skill rule that the
  moved kinds cite. A gap is a named gap with a disposition, never a pass.
- The owner's real Claude Code installation is unchanged.

## Notes

Codex has no native skill loader in the measured version, so this proof is
for Claude Code only. A real run spends subscription usage.

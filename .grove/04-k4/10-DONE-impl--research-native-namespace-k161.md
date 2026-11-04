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

## Decisions (running log)

- The runner gains the Claude Code case field `nativePlugins`, a list of
  `<plugin>@<marketplace>` identifiers. The sandbox HOME adds the marketplace
  from the prepared plain Git checkout of Grove, then installs each plugin
  with `claude plugin install --json`. The record names the marketplace
  commit and each installed plugin's version, Git commit, dependency flag and
  skill files. A later pack move reuses the field. Codex refuses it.
- Skill files under `.claude/plugins` are plugin copies, not native copies of
  `.claude/skills`. The record lists them apart, and a run fails when they
  differ from the installed plugins' own skill files.
- A manual probe in a scratch HOME, before the change: the marketplace added
  from a checkout of `57414e00cd87` installs `research@linkuistics` "(+ 1
  dependency: grove)". `research` has the version `dc3a4812db3b-07869d54` and
  the Git commit `dc3a4812db3b`. `grove` has the version `57414e00cd87`,
  with `auto: true`. The checkout stays clean.
- The run `research-native-namespace-20261104T063648Z` passed on its first
  attempt. Calls 1 to 3 used the Skill tool for
  `research:grove-research-a`, `grove:grove` and `research:research-family`,
  before any other call. The acceptance record's "Research native namespace"
  section holds the subjects, the installed revisions and the observations.
- The dependency installed `grove`. A marketplace added from a directory
  loads its relative plugin `grove` in place from the checkout, and the
  `git-subdir` plugin `research` from the cache. The record names this as a
  limit and not as a gap.
- The runner classifies a read under a plugin's load path as `command`. The
  record names those reads by hand. No later step needs the class, so the
  runner keeps this classification.
- The owner's real `~/.claude` files and listing had the same digests before
  and after the run.

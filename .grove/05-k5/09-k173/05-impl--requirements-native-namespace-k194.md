# requirements-native-namespace-k194

## Goal

Prove that a native Claude Code installation loads a moved requirements
member under its plugin's actual namespace. Prove also that the session reads
the core family file under the `grove` plugin's namespace.

## Context

- `docs/specs/pack-membership.md`, the row "An external member of a
  cross-pack family" in "The acceptance of each move".
- `docs/common-loading-acceptance-record.md`, "Research native namespace", as
  the model. The trial runner field `nativePlugins` installs plugins from the
  marketplace.
- The removal commit from requirements-removal-k192, whose marketplace lists
  `requirements` with a `git-subdir` source.

## Done when

- An isolated Claude Code HOME adds the marketplace from a plain checkout of
  this workstream's line at the removal commit. It installs the
  `requirements` plugin from that marketplace. Record whether the dependency
  installed the `grove` plugin. The `skills` key is absent from the Ravel
  settings, so only the native route supplies skills.
- A Grove `review-requirements` launch with the bare-name mandate loads the
  kind as `requirements:grove-review-requirements`. The agent loads the spine
  under its actual namespace and reads `references/review.md` in it before it
  acts.
- The record names the marketplace commit, the installed plugin revisions,
  the Ravel build, the Grove build that renders the mandate and the
  observations. A gap is a named gap with a disposition, never a pass.
- `docs/repository-moves-record.md` links the record section.
- The owner's real Claude Code installation is unchanged.

## Notes

Codex has no native skill loader in the measured version, so this proof is
for Claude Code only. A real run spends subscription usage.

This is the node's last planned leaf. Its retirement closes
requirements-pack-k173 if no live leaf remains, by the four close steps.

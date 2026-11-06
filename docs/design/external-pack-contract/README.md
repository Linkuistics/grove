# External pack contract — views

The [external pack contract](../../specs/external-pack-contract.md) owns the
decisions. These views show the same design. external-pack-contract-k163
reviewed the contract before the runner work, and external-pack-contract-k164
applied its findings.

Run `task design:external-pack-contract` from the repository root, then open
[the viewer](http://127.0.0.1:8793/). `PORT=...` chooses another port. The
server exposes only this directory. The `.mmd` files and `diagrams.json` are the
editable sources. The viewer loads pinned Mermaid 12.0.0 from jsDelivr, so it
needs network access.

Each graph has one dark node labelled `Start here`, or one for each independent
run.
Numbered edges give the order of the steps. The views show files and checks,
not deployment containers.

| View | Contract illustrated |
| --- | --- |
| [One source directory, copied into the keg](http://127.0.0.1:8793/#diagram-layout) | Grove's release copies `plugins/research` into its keg, and Ravel reads the copy. The repository root holds what the pack does not deliver. |
| [Directed loads of the research kinds](http://127.0.0.1:8793/#diagram-edges) | Each research kind loads the spine across packs, and the pair loads its family skill. A missing loaded skill sends the session to the launch prompt's rule. |
| [Two checks, each against a pinned revision](http://127.0.0.1:8793/#diagram-conformance) | Grove checks against the pack commit of its record `plugins/packs.json`. The pack checks against a recorded Grove commit and runs that commit's runner. |
| [From a published commit to a session](http://127.0.0.1:8793/#diagram-distribution) | Grove's record, `check:packs`, the release archive, the keg and the Ravel source record. |
| [The format files stay in the spine](http://127.0.0.1:8793/#diagram-membership-formats) | The first [pack membership](../../specs/pack-membership.md) decision: every kind reaches the ADR and spec formats through the spine, and the design craft skills deepen them. |
| [WritingSkills owns the always-on hooks](http://127.0.0.1:8793/#diagram-membership-hooks) | Each hooks plugin runs the scripts of its skill. The ordinary writing plugin installs the Claude Code hooks plugin as its dependency. PersonalSkills keeps the native-only `guardrail` plugin. |
| [The asset rule, in order](http://127.0.0.1:8793/#diagram-membership-asset-rule) | The three ordered tests that decide where a Grove asset of a moving craft skill goes. |
| [The boundary that each move proves first](http://127.0.0.1:8793/#diagram-membership-boundaries) | The order of the moves, and the boundary proofs that each move owes beside the base proofs. |

## Current changes

homebrew-delivery-design-k216 applies the human's decisions of
skill-marketplace-decision-k201 to these views. Grove's record names each
pack's commit, and Grove's release copies each source directory into the keg.
No marketplace lists a pack, and no installer links a skill. See
[Distribution](../../specs/external-pack-contract.md#distribution) and
[The checker without a copy](../../specs/external-pack-contract.md#the-checker-without-a-copy).

- [From a published commit to a session](http://127.0.0.1:8793/#diagram-distribution) — Replaced the three routes with one: Grove's record, `check:packs`, the release archive, the keg and the Ravel source record.
- [One source directory, copied into the keg](http://127.0.0.1:8793/#diagram-layout) — The release copies the source directory into the keg, and Ravel reads the copy. The plugin cache and the installer links go.
- [Two checks, each against a pinned revision](http://127.0.0.1:8793/#diagram-conformance) — Grove's check reads `plugins/packs.json` in place of the marketplace.
- [The boundary that each move proves first](http://127.0.0.1:8793/#diagram-membership-boundaries) — The pruned marketplace step became homebrew-skill-delivery-k215, which proves the keg contents and the hook registration.

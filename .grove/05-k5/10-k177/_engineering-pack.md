# engineering-pack-k177 — brief

## Goal

Create `~/Development/AgenticOrchestration/EngineeringSkills`. Move the impl
and prototype chains into its kind directory. They are `grove-impl` with its
licence file, `grove-prototype`, and the review and integration kinds of both.
Move
the engineering craft into its craft directory: the six coding styles,
`using-codebase-memory`, `simplify-project` and `doubt-driven-development`.

## Done when

- EngineeringSkills has the layout that design-pack-k175 established, with
  the roots `["skills", "craft"]`.
- The moved kinds take the bare-name opening. `grove-impl` keeps its
  `LICENSES/` file inside its own skill, as the contract's "Provenance and
  licences" requires for an adapted passage.
- The craft skills keep their bytes and resources, or the record names each
  change. Their checks live in EngineeringSkills.
- Grove's Rust tests that embed `doubt-driven-development` follow the
  decision of pack-membership-k171. Grove's tests, scripts and trial cases
  that use `grove-impl` follow its rule for moved kinds. Core's checks then
  read no file of this pack from Grove's checkout.
- The kind rows move by the contract's move order, and both pack commits are
  on the public remote. Grove's marketplace pins the first one.
- The pack passes alone and in the aggregate. Grove's `bash scripts/check.sh`
  passes after the removal.
- `plugins/linkuistics` no longer holds the nine craft skills, and its
  manifest and description agree with what it still holds.
- `docs/repository-moves-record.md` records the source root, both roots, the
  commits, the tree ID, the `resourceDigest` and the tool requirements:
  the language toolchains of the coding styles and `codebase-memory-mcp`.

## Decomposition

The first child plans this node from the membership decisions and from the
evidence of the earlier pack moves.

## Notes

`impl` is the kind that Grove's own tests and trial cases use most. Plan the
retargeting of those consumers as its own step before the removal from
Grove. Each step then keeps Grove's checks green.

`using-codebase-memory` and the tool-managed `codebase-memory` skill have
different names. The owner design keeps both, so the move changes neither
name.

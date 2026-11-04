# skill-marketplace-k185 — brief

## Goal

Create the marketplace-only repository
`~/Development/Delivery/skill-marketplace`. It lists every Linkuistics pack
plugin from its own source repository. Grove stops hosting the marketplace
and keeps its own record of the pack revisions that it checks.

## Done when

- The repository is colocated Git and jj, holds only marketplace files and
  their checks, and is on a public remote.
- Its Claude Code marketplace lists each plugin with a `git-subdir` source
  that pins a published commit. The list covers core `grove`, `research`, the
  requirements, design, engineering and writing packs, the writing hooks
  plugin, the personal pack, the guardrail plugin and `testanyware`.
- Each listed plugin installs natively from the new marketplace in an
  isolated Claude Code home, as pack-membership-k171 requires. The ordinary
  writing plugin also installs the hooks plugin as its dependency.
- Its Codex marketplace lists the hooks plugin that pack-membership-k171
  assigns to it, or the record states where that marketplace lives instead.
- The repository checks its own pins: each pin exists on its remote, and each
  pack passes alone and in the aggregate with the pinned core.
- Grove keeps a record with the fields `url`, `path` and `sha` for each pack,
  and `task check:packs` reads it, as "The checker without a copy" states.
  `bash scripts/check.sh` passes.
- Grove's installation guidance, `docs/USAGE.md`, the README files and the
  manual installer's help name the new marketplace. Each completed move's
  entry agrees with that move's record.
- `docs/repository-moves-record.md` records the marketplace commit and each
  pin. moves-handoff-k187 compares them with the owner's pins.

## Decomposition

The first child plans this node after every pack move is complete. It
settles the questions below first, as a design leaf if they need one.

## On the horizon

- The owner's Claude Code and other native users registered
  `Linkuistics/grove` as the `linkuistics` marketplace, and it follows Grove's
  `main`. Finish moves `main`. Decide what Grove's `.claude-plugin/` holds
  after finish: a copy, a pointer in the release notes, or nothing. Each
  answer has a different effect on a native user's next update.
- The core entry needs a pin. Grove's new layout reaches a release only in
  finish. The owner design wants the marketplace and the owner's pins to
  name the same commits at the switch. Decide which commit the core entry
  pins before finish, and which step advances it after the release.
- Decide whether the marketplace keeps the name `linkuistics`. A new name
  needs each native user to add the marketplace again.

## Notes

Native users see the marketplace only through Grove's `main` until finish.
The intermediate states of the moves therefore never reach them. Only the
final state must be complete.

The owner design gives the switch the change of the owner's native plugins.
This node supplies the marketplace. It does not register it on the owner's
machine. moves-handoff-k187 records the registration step for
owner-ravel-skill-inventory-k38.

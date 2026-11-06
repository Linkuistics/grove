# grove-pack-record-k227

## Goal

Give Grove its own record of the accepted pack commits, and make
`task check:packs` read it in place of the marketplace.

## Context

- "The pack record and the release" of `docs/specs/codex-skill-provisioning.md`
  states the record.
- "The checker without a copy" of `docs/specs/external-pack-contract.md`
  states how `check:packs` reads it.
- `scripts/check-packs.sh` and `scripts/check-packs.test.sh` read
  `.claude-plugin/marketplace.json` today.

## Done when

- `plugins/packs.json` names the five domain packs, keyed by source ID:
  `requirements`, `design`, `engineering`, `research` and `writing`. Each
  member holds `url`, `path` and a full commit ID as `sha`. The commits are
  those that the marketplace pins now.
- `check:packs` reads only that record. It checks out each `sha` under
  `target/external-packs/<source-id>/<sha>`, under the existing isolation
  and reuse rule. It checks each pack alone and in the aggregate with core.
- TestAnyware, PersonalSkills and `technical-communication-hooks` leave
  Grove's check, because Grove's formula does not carry them. The
  directory-only check of a source without a kind skill leaves with them, as
  the external pack contract states.
- The controls of `scripts/check-packs.test.sh` read a fixture record. Each
  control has been seen to fail: a member without `sha`, a commit without its
  `path`, a key outside the source-ID grammar and a broken pack. The
  external pack contract's "Controls that must fail" lists the others.
- The Taskfile description of `check:packs` names the record.
- `bash scripts/check.sh` passes.

## Notes

The marketplace keeps its pack entries until
`native-marketplace-retirement-k235`. Native users see it only through
Grove's `main`, which moves in finish.

Expose the checkout of each member so that `grove-keg-sources-k229` can copy
from the same checkout.

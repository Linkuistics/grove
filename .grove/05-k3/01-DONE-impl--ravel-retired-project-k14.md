# ravel-retired-project-k14


## Goal

Preserve and remotely verify the complete old Ravel revision, including the
owner's README edit, before any active source replacement.



## Context

- Work in `/Users/antony/Development/Ravel`, using jj. The parent
  `ravel-cli-k3` retains the full runnable extraction contract.
- Rechecked `uwlrknks` / `93f202ba5438955f27090ae0444cc93e61fd8d15` over
  `main` / `2f452e51`; README.md is the only working-copy difference.
- The remote is `https://github.com/Linkuistics/Ravel.git`. Fetch before naming
  the preserved revision; inspect tags rather than deriving a version from prose.

## Done when

- Seal the owner's current snapshot with a useful description, retaining every
  file and README bytes. Create and push a named retired bookmark to origin.
  Verify the remote points at the exact preserved commit before handing off.
- Record the bookmark, full commit, README digest, retrieval commands and version
  decision in Ravel's ordinary `docs/retired-project.md`. Its documentation
  change descends from the preserved revision; advance local main by descendants.
  No old source replacement occurs in this child.
- Confirm the preserved tree equals the pre-session working-copy tree, the README
  digest matches, the documentation-only successor changes only its named file,
  and main retains its previous tip as an ancestor.
- Return here, name the Ravel commits in this leaf's handoff, retire this child
  and seal the coordination-tree commit. The parent stays open for k15 and k16.

## Notes

No product runtime behavior changes in this child, so product build/test runs
would not verify its deliverable. Use actual local/remote revision and content
checks. Ravel has no root Taskfile; k15 creates the reusable build/check/install
entry point from the transferred recipes.

## Decisions (running log)

**Preservation boundary.** The parent spans a generic runnable implementation
and a separate configuration/history migration. This first child satisfies the
mandatory pre-replacement history boundary alone. The named retired bookmark is
`retired/actor-daemon-2026-10-12`; no active sources, personal configuration,
record store, installation or live-session channel are changed here.

**Retired revision verified.** Sealed the owner's exact source tree as
`035edfa30f3bf40eb870a525f49ac98a3b5abbb1`, named and pushed
`retired/actor-daemon-2026-10-12`, and verified `git ls-remote` reports that full
commit for its origin ref. `jj diff` from the original `93f202ba` to this commit
is empty. The preserved README SHA-256 matches the pre-session working copy:
`9ce4d3e7d4774cad418c713bc4feec6f9ded6302ab77ca7eb0f02ecf3058600b`.

**Version baseline.** Fetch and remote tag enumeration found only
`structured-plan-migration` at `09fdb370893f775bc2f03ff2d84b93e76ddb4d25`.
The old README's mention of a subsumed v0.1.0 is not a released tag. Preserve the
parent's settled 0.0.0 development pair and first minor 0.1.0 decision.

**Ravel handoff.** Added `docs/retired-project.md` in descendant commit
`b0d757c39f7a1ccab39c1a40772cd016883f2563` and moved local main to it. Origin
main remains at `2f452e51`; only the retired bookmark was pushed. Ravel has a
clean, empty working-copy change above the documented main. The preserved
bookmark remains at the original source snapshot, without the new document.

**Verification.** Actual local/remote ref comparisons and SHA-256 checks passed;
the entire retired source tree equals the pre-session snapshot, and the
successor's diff adds only `docs/retired-project.md`. The previous main tip is
still an ancestor. Checked the installed jj help for the documented retrieval
command. Grove's `brief-chain` parsed the revised tree, and resolve finds k3 as
the node retaining its complete contract. No product code changed, so package,
runner and installed-layout suites remain obligations of k15, not claimed
results here. Tier 2 coverage found no recorded issue for Ravel's README; its
docs scope is excluded from the older fast graph, so the new document was read
directly. Root and parent closure are not due: k15/k16 and later leaves are live.

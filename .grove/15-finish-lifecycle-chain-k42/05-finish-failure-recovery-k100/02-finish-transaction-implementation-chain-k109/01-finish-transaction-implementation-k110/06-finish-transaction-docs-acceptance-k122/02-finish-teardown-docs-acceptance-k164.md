# finish-teardown-docs-acceptance-k164

**Kind:** impl

## Goal

Reconcile the shipped methodology, CLI help and diagnostics, and the durable
doc set with the implemented finish transaction, then run the complete
acceptance verification.

## Context

- Consume `finish-lost-result-retry-k163`: the rootless retry proof is part of
  the behavior these docs must describe.
- Edit the minimum coherent set in place. `docs/adr/task-tree-transactions-fail-closed.md`,
  `docs/specs/config-driven-sessions.md`, and `CONTEXT.md` were written by the
  contract slices and are believed accurate; verify them against the code rather
  than rewriting, and correct only what diverges.
- The known stale surface is `content/SKILL.md`'s Finish cycle: step 2 still
  says "Delete `.grove/` in one focused commit" with no mention of
  `grove-llm finish-commit <finish-handle>`, and its Resume paragraph still
  treats task-root absence as proof that deletion happened — the exact inference
  the transaction exists to refuse.
- `grove-llm finish-commit`'s help does not mention the transaction, the
  `Recovery pending` operator exits, or the rootless retry.
- `docs/ARCHITECTURE.md`'s in-session-finish-cycle section describes teardown as
  a plain delete-and-commit.

## Done when

- `content/SKILL.md`, `grove-llm` help and diagnostics, `docs/ARCHITECTURE.md`,
  and the finish-relevant parts of `docs/USAGE.md` / `docs/CONFIGURATION.md`
  describe the implemented transaction, its witness, its `Recovery pending`
  operator exits, and the attempt-bound retry.
- No stale unsafe teardown description remains — in particular, nothing tells a
  reader that an absent `.grove/` proves teardown succeeded.
- The spec's finish test-seam list matches the tests that exist; any bullet with
  no test is either covered or removed as a claim.
- Plain Git, native jj, colocated jj, driver restart, lost result, reused handle,
  and cleanup/recovery acceptance tests pass.
- `cargo fmt --check` and `cargo test --locked` pass from a clean verification
  run.

## Notes

Out of scope: the wider legacy-removal doc debt. `grove do`, `--harness` flags,
harness/model policy, `GROVE_REVIEW_HARNESS`, producer launch receipts, and the
`start.md` / `retire.md` prompts still appear across `content/SKILL.md`,
`docs/USAGE.md`, and `docs/CONFIGURATION.md`; those belong to
`legacy-launch-removal-k46`, `legacy-review-removal-chain-k62`,
`methodology-and-viewer-chain-k66`, and `durable-docs-reconciliation-chain-k70`.
Touch only what the finish transaction changed, and do not pre-empt those leaves.

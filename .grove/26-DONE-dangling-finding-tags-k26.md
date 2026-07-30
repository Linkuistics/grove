# dangling-finding-tags-k26

**Kind:** impl

## Goal

Nine `B<n>` tags (`B1`–`B9`) across ~25 comment sites in `src/` and `tests/`
cite a review's numbered findings and **resolve nowhere in the repo** — no
`.grove/` leaf, no `docs/` artifact, no source legend. Decide what they become,
and apply it.

## Context

Found by `nonsrc-position-citations-k25` while classifying all 208
numeric-bearing comment tokens. It is the **same failure family** as `k14`/`k22`/
`k24`/`k25` — a comment citing a referent that died with a brief — but a
**different claim's surface**: a `B<n>` is a *finding id*, not a `.grove/`
position, so every previous claim in that lineage is untouched by it and none of
them is falsified.

**The premise is measured, and it is the opposite of `k25`'s.** The tags **are**
recoverable: `B1`–`B10` are `branch-review-k14`'s numbered findings, readable at
`d65c033^:.grove/14-DONE-branch-review-k14.md` in **this repo's own** history
(`d65c033` is that grove's finish-cycle commit, "delete `.grove/` —
grove-harness-switch finished"). Verified by reading them, not by grep. So this
leaf's options are wider than `k25`'s codex-bridge decision, where recovery was
possible but the referent was in a *foreign* repo.

**Shape is not the discriminator — reachability is**, and this is why the tags
must be judged individually rather than swept. Three same-shaped series sit
beside `B<n>` in the same files and are **correct**:

- `T<n>` → Task numbers in `docs/superpowers/plans/2026-07-18-codex-pi-harness-switch.md` (live).
- `K3` → `docs/superpowers/specs/2026-07-18-codex-pi-harness-switch-design.md:34` (live).
- "runbook step 6" → that same spec's *Migration runbook* (live, and it says what the comment claims).

`F1`/`F2` are a fourth case and need checking rather than assuming: they point at
`chain-node-review-k10`'s findings, which are in **`.grove/` right now** and die at
this grove's finish cycle — so they are dangling-in-waiting, not yet dangling.

Sites, measured: `src/launch.rs` (B3, B4), `src/provision.rs` (B8×2, B9),
`src/loop_driver.rs` (B2, B5×3, B6, B7), `src/harness_stamp.rs` (B3),
`tests/launch.rs` (B3, B4×3), `tests/provision.rs` (B9×2),
`tests/loop_driver.rs` (B1×2, B2, B5×2, B6, B7×5).

## Done when

- Every `B<n>` in `src/` and `tests/` either dereferences a durable referent or
  no longer needs to be dereferenced. `F1`/`F2` decided on the same basis.
- The decision is recorded with its reasoning, and applied uniformly.
- Re-verified by grep over the whole repo — checked for having run, against a
  positive control **and** a cross-tree control (`k24`'s method: a broken
  instrument reads clean everywhere, so clean-here-plus-dirty-there is the
  evidence, not clean-here alone).
- `cargo test` passes.

## Notes

**Do not default to deleting the tags.** `k25` found the opposite of what its own
Context predicted — the codex-bridge pointer was called "load-bearing" and turned
out to be decorative, because the reason it cited was already stated three lines
below it. Check each `B<n>` the same way: read the recovered finding, then read
the comment, and see which one is actually carrying the sentence. A tag whose
finding is restated inline is free to drop; one that is the only record of *why*
a guard exists needs the finding's substance promoted, not its number preserved.

**`k25`'s applied fix is the worked precedent for the promote case.** Where a
citation was load-bearing it became an **ADR slug**, a **module path**, or a
`<slug>-k<key>` **handle plus where the commit log holds it** — never a path and
never a bare number. That last form is the one this leaf will most likely want:
ADR *task-tree-scheme* §5 exists precisely so a work item stays nameable after its
tree is deleted, and it is what made the AgentAnyware referent recoverable.

**Do not ship a lexical guard test**, inherited from `k24`/`k25` and for the same
reason: the discriminator is *semantic* (does the reader dereference it?), so any
regex either flags the correct `T<n>`/`K3` sites or narrows to a pattern list,
re-committing `k14`'s defect while wearing a test's authority.

CHANGELOG-free unless it changes behaviour — `retire-help-node-path-k20`'s rule,
now confirmed by `k21`–`k25`.

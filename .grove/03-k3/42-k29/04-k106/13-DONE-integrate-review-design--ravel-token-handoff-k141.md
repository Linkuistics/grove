# ravel-token-handoff-k141

**Integrates:** ravel-token-handoff-k140

## Goal

Triage the integrated token handoff design review and leave a coherent
contract before ravel-token-handoff-k133 implements it.

## Context

- Review ravel-token-handoff-k140 and its committed findings in the
  "Token handoff integrated design review" section of
  `docs/common-loading-acceptance-record.md`.
- The reviewed Ravel artifact is commit
  `9084e0dcfcd733a1185ca1bb26a312b53fb25549`, on the unpushed bookmark
  `skill-directory-delivery`, in `Ravel.skill-directory-delivery`. Follow
  that repository's VCS discipline.
- The owner chose the minted-token and socket route in
  ravel-token-handoff-k132. That route remains settled.

## Done when

- Each review finding has an evidence-based disposition. Apply supported
  corrections to their owning design artifacts and explain rejected findings.
- Record the resulting Ravel commit and the verification limits in Grove's
  acceptance record, for ravel-token-handoff-k133 to consume.

## Notes

This task owns design integration, not implementation. Read no credential and
make no model call. The opt-in credential check remains with
ravel-token-handoff-k133 and needs the human's approval.

## Decisions (running log)

**Triage evidence.** The triage reads F1 from review commit `4556a6329f1`. It
checks the finding against Ravel commit `9084e0dcfcd7`, whose workspace has an
empty working copy above it. It reads no credential and makes no model call.

**F1 is a real issue.** `platform_command` in
`crates/keyed-launch/src/confinement.rs` binds each writable root, runtime read
and read tree by path. Bubblewrap 0.11.0 resolves each bind source with
`realpath` in `resolve_symlinks_in_ops`, after the runner's last comparison. A
confined process that can write the parent of a grant of another launch can
replace that grant with a link in the interval. The same replacement works for
a writable root, and then the other launch can write the target directory. The
race is older than this feature, because it also reaches the settings file.

**A repair exists, with a cost.** Bubblewrap 0.10.0, published on 2024-08-14,
added `--bind-fd` and `--ro-bind-fd`. After each such bind, bubblewrap compares
the mount with the open descriptor and stops before the payload on a
difference. Release 0.13.1 keeps the check. Ubuntu 24.04 ships 0.9.0 and
Debian 12 ships 0.8.0, so this repair removes Linux confinement there. Ubuntu
26.04 ships 0.11.1 and Debian 13 ships 0.12.0.

**A question to the human.** The human asked why the session spends this
effort on containment, when the harnesses contain themselves. The session
asked whether to repair F1 or to accept it as a stated Linux limit. The leaf
stays live until the answer.

**The human's answer.** The human chose the third direction. The confined
Claude Code cases leave the acceptance contract, and the harness sandboxes
are enough. Only those cases needed the token handoff, so its design is
withdrawn. `ravel-token-handoff-k133` and `claude-confined-rerun-k134` are
pruned on that decision. Their two rows become explicit gaps.

**The withdrawal is additive.** The Ravel workspace refused a restore of the
earlier tree. `jj revert` made `bb0699dc0189`, which reverts `9084e0dcfcd7`,
and `7f4093832f33`, which reverts `7c5b6cdd3842`. The resulting tree equals
the tree of `0d60cb531240`, so the loader report work below stays.

**Two limits stay visible.** The book's runner chapter owns platform limits,
as `session-execution.md` states. Commit `d16976730248` adds the Keychain
limit to its confinement section. It adds the grant interval as a row of its
guarantees table, because a run of prose there would pass 450 words. The
interval is older than the design, and the human's decision leaves it
unrepaired. The bookmark `skill-directory-delivery` is at `d16976730248`.

**The build stays current.** After the build commit `e9cc79993b46`, only two
documents changed, and no crate embeds either. The runner checks the build by
digest and identity, so `builds.json` keeps its pins.

**A conversion first.** The record section of `ravel-token-handoff-k140` held
four semicolons, a long paragraph and a long instruction. A separate commit
before this task's commit converts it to controlled English, with no change of
meaning.

**Result.** The "Token handoff withdrawal" section of
`docs/common-loading-acceptance-record.md` holds the decision, each
disposition, the Ravel commits and the limits. The acceptance-handoff brief
names the decision for the remaining leaves.

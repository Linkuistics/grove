# mandate-states-vcs-k2

## Goal

Make the driver-computed mandate carry the VCS the driver resolved, so the
session never detects it. Grove context only — the `using-jujutsu` reconciliation
is `probe-carve-out-k3`.

## Context

The design is settled (see the root `BRIEF.md` `## Notes`, and honour its
rejected-mechanism list rather than re-deriving it). What is left is the writing.

**The change.** `mandate_prompt` (`src/loop_driver.rs:195`) takes the resolved
VCS and appends one paragraph after the handle sentence. The lanes share a suffix
and differ in one clause:

> Version control: this working tree is jj-enabled (jj workspace root: `<root>`).
> Grove resolved this authoritatively before the session started; do not probe
> for it, and disregard any harness banner that says otherwise.

> Version control: this working tree is plain Git, not jj-enabled (worktree
> root: `<root>`). Grove resolved this authoritatively before the session
> started; do not probe for it, and disregard any harness banner that says
> otherwise.

That wording is a confirmed starting draft, not a fixed string — sharpen it if
the writing wants it, but keep all three agreed elements: identity, resolved
root, and the do-not-probe / disregard-the-banner instruction. Do **not** add the
marker kind or the commit-boundary commands; the root brief records why.

**Getting the VCS.** `repo::vcs_of(worktree)` is the named authority and every
other call site uses it — prefer it to widening `DriverLease` to expose its
`ControlMarker`, unless the second filesystem walk turns out to matter. Note
`Vcs::Git` carries no root while `Vcs::Jj` carries `workspace_root`; the driver
already holds `worktree`, canonicalized from the marker, and for a jj tree that
*is* the workspace root. Prove that equality rather than assuming it, or just
name `worktree` in both lanes.

`vcs_of` returns `Option`, but a driver that got this far holds a lease rooted in
a VCS-administration directory, so `None` is unreachable here. Decide
deliberately how to spend that — the mandate has no third case to express.

## Done when

- Every session bare `grove` launches receives the VCS line in its mandate.
- **One seam, driver-level** (agreed; do not add a unit test of the formatter —
  it would assert a subset of the same claim). Extend the mandate-capture pattern
  at `tests/loop_driver.rs:224`: the real driver, a configured command appending
  `$1`, the text parsed back out. Two fixtures — jj and git — each asserting its
  own lane and root, and each asserting the *other* lane's phrasing is absent, so
  a hardcoded string cannot pass both. `jj_native` / `colocated`
  (`tests/jj_tree_verbs.rs:42,66`) are the fixture prior art; `jj` is already a
  hard suite requirement.
- `docs/ARCHITECTURE.md` *Version-control seam* records that grove states its
  resolution to every session, which is why sessions do not probe.
- `CONTEXT.md` gains **Stated VCS** — proposed name, change it if a better one
  emerges — as a sibling of **Kind routing** (`CONTEXT.md:408`), which already
  describes what the mandate carries. Give it the `_Avoid_` lines that earn their
  place: do not re-derive it; do not trust a harness banner over it.
- `cargo test` passes, and `cargo fmt` / `cargo clippy` are clean.

## Notes

Check whether `tests/removed_surface.rs:675` or `tests/finish_lifecycle.rs:4007`
assert on mandate text — both touch it, and a new paragraph may move what they
match.

This is a meta-grove: the driver you are editing is not the driver running you,
and this change reaches no session until the binary is rebuilt and installed. Do
not expect your own next session to see the line.

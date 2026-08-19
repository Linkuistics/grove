# local-config-kdl-k6

**Reviews:** local-config-kdl-k3

## Goal

Read the configuration-delta implementation adversarially, against
`docs/adr/untracked-configuration-delta.md` and what survived in
`docs/adr/complete-session-configuration.md`. Findings, not fixes — the paired
`integrate-review-impl` leaf, if this one finds anything, owns every change.

## Context

`local-config-kdl-k3` widened `SessionConfig::load` to take `DeltaRoots`,
resolved `.grove.kdl` over the personal file per kind, enforced the delta's
untrackedness through a new `repo::path_is_tracked`, corrected the spawn and
validation diagnostics to name the file each kind actually resolved from, and
reconciled `docs/CONFIGURATION.md`, `docs/ARCHITECTURE.md`,
`content/references/driver.md`, `content/references/decompose.md` and
`CONTEXT.md`. `cargo test`, `cargo fmt` and `cargo clippy` were green at the
commit; that proves the asserted cases hold, not that the right cases were
asserted.

**Five specific doubts**, roughly in order of how likely they are to be real.

1. **The stated gap: nothing asserts end-to-end that an overridden kind reaches
   a different program.** Requirement 5 fixed the boundary at
   `SessionConfig::load` deliberately, with that cost stated, and this leaf
   honoured it. So the whole chain from a `.grove.kdl` on disk to a different
   argv actually spawned is covered only by the seam plus two `load` call sites
   the suite never exercises together. Read `src/loop_driver.rs` for what could
   break without a test failing — a `resolved_source` computed from the wrong
   kind, a delta resolved at the pre-launch load but not the pre-mutation one,
   the two `DeltaRoots` fields swapped at the one construction site. Say whether
   the boundary should move, and if so name the minimum test rather than
   "add end-to-end coverage".
2. **Is the trackedness probe sound in the shapes the fixtures do not build?**
   `repo::path_is_tracked` answers from emptiness of stdout: `git ls-files -- <name>`
   and `jj file list --ignore-working-copy <name>`. The fixtures are a plain git
   checkout (committed and not), and jj-native and colocated trees (snapshotted
   and ignored). Untested: a **git linked worktree** and a **secondary jj
   workspace**, which are exactly the layouts where the two searched roots differ
   and where the probe's anchoring — the candidate's *own* directory, not the
   leased worktree — is load-bearing. Also worth attacking: a file name jj would
   read as a fileset expression, a `.grove.kdl` that is a symlink into the
   repository, and whether `git ls-files` can print for a path that is not
   actually tracked in the delta's own worktree.
3. **Does `None` from `vcs_of` deserve to answer "untracked"?** The probe
   returns `Ok(false)` when no marker is found, reasoned as "nothing owns it, so
   nothing tracks it, and a hostile repository has a marker by definition". That
   is a deliberate hole in a fail-closed posture. Try to construct a case where a
   candidate sits in a tree Grove's marker walk misses but a VCS still supplies —
   and if you cannot, say so, because the reasoning is then worth keeping rather
   than hardening.
4. **Is the candidate test the right one?** `find_delta` treats anything
   `symlink_metadata` can stat as *the* delta, so a directory or a broken symlink
   at the worktree root shadows a perfectly good delta at the repository root and
   fails the load. That is the fail-closed reading of "the first file found";
   check it against the ADR's wording and against what an operator would expect
   from a stray `.grove.kdl/` directory.
5. **Do the reconciled documents still agree with each other and with the
   code?** `docs/CONFIGURATION.md` gained a delta section and lost its "entirety
   of user configuration" claim; `CONTEXT.md` gained a **Configuration delta**
   entry and reworked **Grove configuration**; `content/references/driver.md`
   now says personal launch policy has two homes while still owning the
   `Nothing else routes a session` rule. Check for a claim one file makes that
   another now contradicts, and for anything the delta falsified that was missed
   — `docs/USAGE.md` and `README.md` were read and judged still true, which is a
   call worth a second reader.

## Done when

Each doubt above is answered — confirmed with a concrete failing case, or
dismissed with the reason it is not real — plus anything else the read surfaces.
Inspection only: read the committed change, the two ADRs and the source; do not
run test, build, lint or format commands, and do not edit code. If there are
findings worth acting on, the last act is to cut the `integrate-review-impl`
leaf carrying them verbatim; if there are none, create nothing and retire.

## Notes

The producer spent no in-session reviewer, so this is the first fresh context to
read any of it.

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

## Review

### Findings

1. **High — the Git trackedness check trusts an inherited alternate index.**
   `src/repo.rs:493` builds the security-sensitive `git ls-files` probe and
   `src/repo.rs:501` applies only `anchor_git_worktree_environment`; that helper
   removes `GIT_DIR` and `GIT_COMMON_DIR` and replaces `GIT_WORK_TREE`, but it
   does not remove `GIT_INDEX_FILE` (`src/repo.rs:350`). The repository's own
   internal-child contract already classifies `GIT_INDEX_FILE` as a repository
   selector and removes it in `scrub_internal_child_env`
   (`src/launch.rs:105-112`), but this new subprocess never calls that helper.
   Concrete failure: let the real checkout index track `.grove.kdl`, launch
   Grove with `GIT_INDEX_FILE` naming a valid empty alternate index, and
   `git ls-files -- .grove.kdl` exits successfully with empty stdout. Grove then
   accepts and executes the repository-tracked delta, defeating the seam's
   security guarantee. The same omission also passes the ambient Grove control
   environment to both new VCS subprocesses, contrary to the shared
   internal-child rule.
2. **Medium — delta discovery turns metadata failures into absence.**
   `src/session_config.rs:201-204` implements the first-candidate search with
   `symlink_metadata(candidate).is_ok()`. Every error, not only `NotFound`, is
   therefore treated as “no delta”: a permission or I/O error at the worktree
   candidate silently selects a repository-root delta, and the same error at
   the repository candidate silently falls back to the personal file. That
   contradicts both search precedence and the explicit rule that an unreadable
   delta fails closed (`docs/CONFIGURATION.md:150-157`). Discovery needs to
   distinguish `NotFound` from every other metadata error and return the latter
   with the candidate path.

### Five doubts resolved

1. **Keep the test boundary at `SessionConfig::load`.** The settled requirement
   explicitly accepts the missing process-level assertion, and the production
   wiring is small and internally consistent: one named `DeltaRoots`
   construction feeds both load points, the selected kind indexes both
   `source` and `expand`, and the expanded argv is passed unchanged to the
   launcher (`src/loop_driver.rs:106-168`). There is no new evidence sufficient
   to overturn that recorded trade-off. If the requirement is ever reopened,
   the minimum process test is one existing fake-command fixture with personal
   `impl` command A and delta `impl` command B, asserting B records the launch
   and A does not; a broader driver matrix would add no extra signal for this
   claim.
2. **The root anchoring itself is sound, but the Git environment is not.** A
   linked worktree candidate is probed from its own worktree and a main-root
   candidate from the main checkout; a secondary jj workspace and its default
   workspace are likewise probed from the candidate's own root. The fixed
   `.grove.kdl` argument is a valid jj path expression, and empty stdout is an
   adequate trackedness answer once the command is bound to the correct
   repository view. Finding 1 is the concrete counterexample: inherited
   `GIT_INDEX_FILE` changes that view. An untracked `.grove.kdl` symlink to a
   tracked payload does not independently breach the stated boundary: Git or jj
   rejects a tracked symlink at the candidate path, while creating an untracked
   link is an explicit local delegation to repository-controlled bytes, like
   configuring a tracked wrapper as word zero.
3. **`vcs_of == None` is not a production hole on the reviewed path.** Both
   roots passed by the driver came from the already-resolved worktree and main
   repository, so each has an owning marker. Reaching `None` requires the marker
   topology to be removed concurrently after lease acquisition, or calling the
   public load seam with non-production roots; neither supplies a repository
   that Grove can still identify as owning the candidate. The deliberate
   `Ok(false)` remains coherent.
4. **Directories and broken symlinks should shadow and fail.** Treating any
   successfully stated entry at the privileged name as the first candidate is
   the conservative interpretation: a stray `.grove.kdl/` or broken link gets a
   diagnostic instead of silently changing launch policy to the repository or
   personal source. Finding 2 is the defect around that decision: failure to
   *state* the entry is currently confused with confirmed absence.
5. **The reconciled repository documents agree with the implementation.**
   `docs/CONFIGURATION.md`, the architecture flow and module seam,
   `content/references/driver.md`, `content/references/decompose.md`, and the two
   glossary entries consistently describe first-found two-root lookup,
   whole-template replacement, mandatory personal completeness, and enforced
   untrackedness. README and `docs/USAGE.md` still truthfully state the required
   personal configuration and do not claim it is the only source. The older
   wording in the provisioned skill is the already-recorded build-pairing skew,
   not another source defect in this commit.

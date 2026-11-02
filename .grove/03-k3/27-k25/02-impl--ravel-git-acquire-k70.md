# ravel-git-acquire-k70

## Goal

Deliver online exact Git acquisition at both public seams, `skills resolve` and
`start-session`. Each resolution acquires the configured commit and `subdir`
through Git on `PATH`. The extracted tree then passes through the existing
manifest, check and publication path. Local sources keep their behaviour.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Settings", "Sources
  and retained resources", "Exact Git acquisition and offline reuse" without
  the cache hit, "Directory descriptor and loading contract" and "Failures".
- The running log of `ravel-pinned-delivery-k26` names the seams:
  `refuse_git` and `origin` in `skills::watched`, and the shared capture path.
- The fixtures of `ravel-skills-resolve-k54` and `ravel-skill-launch-k55` show
  the public seam style.

## Done when

- The session fetches Ravel. When `main` moved, it rebases the line, reconciles
  upstream changes and reruns the affected checks. It records the exact base.
- `refuse_git`, the code `skill_git_unsupported` and their fixture are gone.
  Settings refusals keep stage `cli` and come before every acquisition.
- Ravel acquires the commit in isolated temporary Git storage inside private
  staging. It checks the object format of the pin against the repository. It
  checks that the object is a commit, not a tag object that peels to one. It
  checks that `subdir` is a tree in that commit. It extracts only the
  configured full commit ID and never a branch tip, ancestor or other source.
- Every Git child runs noninteractively, with no terminal and null standard
  input. It runs no hook, filter, attribute conversion or submodule command. It
  receives no `RAVEL_EXIT_FILE`, session-result context or caller completion
  descriptor. Inherited `GIT_*` redirections, such as `GIT_DIR`, do not reach
  it. The ordinary noninteractive authentication of the owner still works.
- Extraction writes the tracked object contents directly, with no working-tree
  checkout. A gitlink refuses. A symlink obeys the chain rules of a local
  source. The executable bit follows the tree mode. A committed Git LFS pointer
  is captured as its literal bytes. Path validity and collisions are checked
  against the tree listing, not only against the written tree.
- The descriptor, the request and `skills resolve --json` carry the Git source
  record: `url`, `revision` and `subdir`, with no `originPath`.
- A missing Git executable, a failed transport, an unreachable commit and a
  missing `subdir` refuse as `skill_revision_unavailable`. Each refusal names
  the source and the required revision. A wrong object type has its own
  remedy. No refusal launches a worker or a managed process.
- Local Git fixtures use `file://` and an absolute path at both seams. They
  cover "Exact Git", "Git LFS object bytes" and "Relative resources" with
  gitlinks. They cover "Missing or corrupt revision" for the commit, `subdir`
  and object type. They cover "Directory delivery" with mixed local and Git
  sources. A failure of the Git source in a mixed set leaves no
  descriptor.
- The specification status, the book, `CONTEXT.md` and `CHANGELOG.md` describe
  online acquisition and no longer name the temporary refusal. The affected
  Ravel checks pass. The bookmark advances by jj, and nobody pushes it.

## Notes

Offline reuse and the acquisition key record are `ravel-git-offline-k71`. Each
resolution in this leaf may contact the remote. Transport deadlines,
cancellation, child reaping and diagnostic bounds are `ravel-git-bounds-k72`.
Keep internal seams for their fault injection.

The protected-overlap rules apply to a local repository path. A refusal in
this leaf must never suggest a revision that the owner did not configure. This
leaf carries the transport hygiene of every Git call, so it is a likely
candidate for a `review-impl`. Insert any review before
`ravel-pinned-handoff-k75`.

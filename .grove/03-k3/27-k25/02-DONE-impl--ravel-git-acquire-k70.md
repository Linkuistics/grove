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

## Decisions (running log)

**Base at entry.** On 2026-11-02, `jj git fetch` in
`Ravel.skill-directory-delivery` reported no change. `main` and `main@origin`
are both `24a545236185`. The bookmark `skill-directory-delivery` is at
`1d50dae5d29e`, directly above `main`, with an empty working-copy change. The
line needs no rebase.

**Observed Git behaviour.** Apple Git 2.54 at `/usr/bin/git` and Homebrew Git
2.56 behave the same in a scratch probe. Protocol 2 over `file://` fetches a
non-tip commit and a tag object by full ID. A glob refspec without a
destination is invalid. A sha1 pin against a sha256 repository fails with
`mismatched algorithms`. Git's `prompt.c` uses a set but empty `GIT_ASKPASS`
in place of `core.askPass` and `SSH_ASKPASS`, and runs nothing.

**Acquisition shape.** A private `git-` staging holds an empty template and
hooks directory, a bare repository in the object format of the pin and the
extracted tree. The direct fetch is `--depth=1` of the full ID. When it fails,
one fetch of `+refs/*:refs/fetched/*` follows, and a failure of that fetch is
the transport failure.

Ravel walks the trees itself with non-recursive `ls-tree -z -l`. It reads
each blob with `cat-file blob`. Thus every child has null standard input, and
no name with `/` can pass as a path. The extracted tree then goes through the unchanged `snapshot::capture`, which owns links,
reserved names, bounds and the manifest. Refusals name Git paths as
`<url>@<revision>/<subdir>/<path>`.

**Child hygiene.** Each Git child runs in a new session with null standard
input and every inherited descriptor above 2 closed at exec.

Its environment
is the caller's, minus every `GIT_*`, `RAVEL_*` and `HARNESS_DISPATCH_*`
name, plus `GIT_TERMINAL_PROMPT=0`, an empty `GIT_ASKPASS`,
`SSH_ASKPASS_REQUIRE=never`, `GIT_NO_REPLACE_OBJECTS=1` and `LC_ALL=C`.
Command-line configuration sets the hooks path to the empty directory and
turns off the file-system monitor, submodule recursion, automatic maintenance,
commit-graph writes and bundle URIs. It sets `credential.interactive=false` and
permits only the https, ssh and file protocols. Every command uses
`--git-dir`, so `safe.bareRepository=explicit` cannot refuse it. System and
global configuration stay, because they hold the owner's credential helpers.

**Left to later leaves.** The notice reports `acquired` by type until k71
adds `reused`. Diagnostics carry a short tail of Git's standard error, and k72
replaces it with the 256 KiB redacted rule. The deadline does not interrupt a
running child, and entry and size bounds before extraction are k72's.

**Refusal paths of Git sources.** Capture takes a shown root beside its I/O
root. `Walk` maps each refusal path below the extracted tree to
`<url>@<revision>/<subdir>/<path>`. Thus no refusal names a staging path that
cleanup removes. The fault hooks still receive real paths.

**Fixtures and controls.** `tests/skills_git.rs` builds repositories with
`hash-object`, `mktree` and `commit-tree`, so gitlinks, case collisions and
exact modes need no work tree. A spy `git` first on the private `PATH` records
each child. Ordinary Git with the same HOME runs the hooks, the template hooks
and the smudge filter. That proves those controls can fire.

Nine mutations of `git.rs` and `mod.rs` each failed their guarding test. They
removed the hooks path, the environment filter and null standard input. They
also removed the descriptor sweep, the fallback fetch and the type check.
The last three removed the gitlink refusal, the collision check and the shown
root. The new session has no control, because the test process has
no terminal to lose.

**Book layout.** The rules of a Git child are a table in the reference
chapter. A paragraph there made a new prose run longer than 450 words. The
two other long runs in that chapter predate this leaf.

**Checks.** `task check` passed on the final code: formatting, scripts, lint,
terminal recovery and 741 nextest cases, with 48 skipped. The 13 cases of
`skills_git` passed.

`task docs:excerpts`, `task docs:book:build` and
`task docs:examples` passed. `docs:ste` reports no problem in the reference
chapter or `CONTEXT.md`. The specification and the changelog have their
earlier counts. `task test:native-linux PLATFORM=arm64` reported `failed=0`,
and its `check` and `test` receipts hold the 13 Git cases. On Linux the cache
keeps both names of the case collision.

**Output.** Ravel commit `1898dc8603a0befd77f8b39f43e8b0abc7dc4da8` on the
unpushed bookmark `skill-directory-delivery`, with the parent `1d50dae5d29e`.
An empty working-copy change sits above it.

**Review earned.** The transport hygiene of every Git call rests on claims
that no compiler checks. The mutation controls cover only the safeguards that
this leaf wrote. `ravel-git-acquire-k76` is a `review-impl` before
`ravel-git-offline-k71`, so its findings land before the next leaf edits
`git.rs`. The launch has no `GROVE_PROVIDER_EVIDENCE`, so the review has no
Creator line.

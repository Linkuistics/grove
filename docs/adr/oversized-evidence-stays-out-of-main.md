# Oversized evidence stays out of main

Evidence with a blob above the publishing host's per-file limit stays out of
`main` and out of every pushed commit. The work that produced it reaches `main`
as a **curated change**. That change is one new commit on current `main`, and it
carries a named set of durable paths. The full evidence stays in the unpublished
history of the workspace that produced it. `scripts/check.sh` refuses a tracked
file above 50 MiB in the working-copy commit.

The first case is the documentation-guidance grove. Its evaluation tree under
`docs/evaluations/documentation-guidance/` held about 1 GB of cases, observer
evidence and fixtures. On 2026-10-07, 50 commits of its history each held a
`list-readings.json` blob above 100 MiB. `main` received the spec, the research,
the ADRs, the design figures, the comparison report and the record of the human
read. The cases, the harness and the observer evidence stayed behind.

## The trade-off

**The host's limit applies to every pushed blob, not to the tip.** GitHub
refuses a file larger than 100 MiB (104,857,600 bytes) and warns above 50 MiB
([about large files](https://docs.github.com/en/repositories/working-with-files/managing-large-files/about-large-files-on-github),
read 2026-10-06). The host reads each blob in every commit that a push sends, so
a smaller file at the tip does not help. A history that ever held an oversized
blob cannot be pushed as it stands.

**A curated change gives up the task history on `main`. A rewrite costs a chain
of sessions.** The grove's history had 448 commits. The earlier form of this
decision kept them all. It stored each oversized file as exact parts: ordered
files below 1,000,000 bytes, with a manifest of their SHA-256 digests. A
storage-only rewrite through jj was to change each old commit in the same way.

That rewrite needed the owner's authorisation and a rehearsal at full scale. On
2026-11-16 the human abandoned the evaluation harness that the evidence served,
and the rewrite with it. A curated change needs no rewrite. On `main`, the work
is one commit, and a citation of an old commit ID resolves only in the
producing workspace.

**The evidence is kept, but not on `main`.**
[Evidence outlives the instrument](evidence-outlives-the-instrument.md) keeps
evidence that something still cites for authority. The curated change carries
each document that the installed guidance cites: the survey, the reader evidence,
the comparison and the record of the human read. The evidence that stayed behind
supports the abandoned harness, and no carried rule depends on it.

**What it costs.** A carried document cannot link into the evidence that stayed
behind. Where a carried document names such evidence, it states that limit. A
reader who needs that evidence needs the producing workspace. The publication
check reads blob sizes through `git ls-tree` on jj's backing repository, because
jj's templates expose no blob size. It makes no commit, so the rule that commits
go through jj still holds.

## Considered options

- **Exact parts and a storage-only history rewrite.** This was the earlier
  decision. Rejected on 2026-11-16: the rewrite needed owner authorisation and
  a full rehearsal, and the harness whose evidence it kept was abandoned. Reopen
  when oversized evidence must reach `main` with its commit history.
- **Rebasing the whole history onto `main`.** Rejected: GitHub refuses the push.
- **Git LFS.** Rejected: jj runs no LFS filters, so a jj checkout would see
  pointer files. The oversized versions alone total 9.6 GB. Reopen if jj supports
  LFS and the volume fits the account's quota.
- **One compressed file.** Rejected: a compressed blob cannot be reviewed as text.
- **An external archive or release asset behind a digest.** Rejected: readers
  would need a network and a second store. Reopen for evidence that a reader of
  `main` must reach.

## What enforces it

`scripts/check.sh` runs `scripts/check-blob-sizes.sh tip`. That check fails when
a tracked file in the working-copy commit is above 50 MiB. `check.sh` also runs
the script's tests, which build scratch repositories for a small blob, a warning
and a refusal.

`task publication:history:check` reads every commit that a push
to `origin` would publish. It fails on a blob above 100 MiB and reports each blob
above 50 MiB. It is a task, not a part of `check.sh`. Neither check is a gate on
the server, so GitHub's own refusal stays the backstop.

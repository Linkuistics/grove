# `grove-meta` sync is local-first, opt-in remote, fetch-before-drain, push-best-effort

The `grove-meta` branch is **local-only by default** — `grove install` and `grove meta init` materialise the branch and worktree without configuring an upstream. Multi-machine users opt in explicitly via `grove meta remote add <url>` (which sets upstream tracking). When a remote is configured, every `grove start` / `grove continue` fetch + ff-merges before drain, and every `grove-llm inbox-add` / `grove-llm inbox-drain` pushes best-effort after commit (with one auto-retry on non-ff, safe under the directory-of-files shape). When no remote is configured, all of the above is a no-op. Intra-machine concurrency is serialised by git's own index lock with a bounded CLI retry.

## Status
superseded by ADR-0031 `0031-shed-machinery-keep-self-extension-core-and-methodology.md` — the `grove-meta` branch and its sync are deleted with the inbox subsystem.

## Why local-first, not auto-remote
The single-machine user is the common case. Materialising a branch and worktree at install time is a local-only operation; adding a remote-fetch step would introduce a network dependency to a workflow that is fundamentally about local capture-and-drain. The remote is a coordination convenience for multi-machine and multi-author use; it is not the source of truth. Treating the local `grove-meta` worktree as authoritative — with the remote as a sync target when one happens to be configured — preserves a working experience offline, on a fresh laptop, or in any of the network-degraded states the user already lives with.

The complement is also true: the multi-machine user is real (the grove project's own author works across multiple machines) and silent state divergence between them is intolerable. The opt-in remote covers exactly that case without inflicting it on users who don't need it.

## Why fetch-before-drain, with soft failure
The post-mortem survey at `docs/research/in-repo-issue-tracker-postmortems.md` documents repeated failures of "manual sync" workflows. The strongest signal is git-bug's own dogfood ticket [git-bug/git-bug#1221](https://github.com/git-bug/git-bug/issues/1221) — the project's most active non-maintainer contributor admits "most of my commenting and issue opening/closing occurs offline… periodically, i do need to go to github.com" and wants a cron job that runs `git bug bridge pull && git bug push` automatically. The lesson: tools that leave sync to user discipline lose. git-appraise inherited the same pathology ("[HN: I just write reviews to myself](https://news.ycombinator.com/item?id=37084575)"); Radicle ships an ergonomic `rad sync --fetch` shorthand and still warns that local state is silently behind ([guides/user](https://radicle.dev/guides/user)).

Sapling/jj's MetaLog ([sapling-scm.com/docs/dev/internals/metalog](https://sapling-scm.com/docs/dev/internals/metalog/)) is the post-mortem survey's preferred pattern: treat local state as a cache, rebuild deterministically from the source of truth on every read, and refuse to commit on merge failure. We adopt the cache stance at session boundaries: `grove start` and `grove continue` fetch `grove-meta` before drain so the session triages the latest known state.

**Soft failure** because the alternative is hostile. On a train, on a flight, behind a corporate proxy that rejects the SSH key — the local inbox is still useful. Refusing to drain because fetch failed would force the user to either hand-resolve the network problem or skip drain entirely (and then the bootstrap step that exists precisely to prevent forgotten observations becomes the thing they skip). We warn loudly on fetch failure and proceed with local state; the user knows their view may be stale and the warning is unmissable.

**Refuse on non-ff** because that case means a real divergence: a remote commit conflicts with an unpushed local one. Auto-merging would either pick a winner silently or invent a merge commit on a branch whose history is supposed to be linear-per-event. Refuse-and-instruct is the only safe response. This is structurally rare under the directory-of-files shape (ADR-0004) — disjoint write paths produce no textual conflicts — but it can still happen if a previous `grove-llm inbox-add`'s push was rejected and the user kept working without resolving.

## Why push-best-effort with one auto-retry on non-ff
Auto-pushing on every write couples write latency to network availability; if the push fails for any reason the local commit still stands and `grove meta sync` can publish it later. The retry-on-non-ff exists because the directory-of-files shape makes the retry safe: a non-ff rejection means another writer pushed before us, but their write was a *new file on a disjoint path*, not an edit to our file. Fetching their commit and replaying our push is structurally non-conflicting. We retry once and refuse loudly if it still fails (the third attempt would be the loop).

This is the cleanest of the post-mortem survey's recommendations: keep the write local and fast (manual semantics) but ship a scheduled/post-action sync verb out of the box so the user has a one-liner to enable. The verb is `grove meta sync`; cron-driven use is the canonical multi-machine deployment.

## Why the opt-in verb is `grove meta remote add|remove|list`
The dedicated `grove meta` namespace already exists for `grove meta init`. Sub-verbs `remote add|remove|list` mirror `git remote` exactly, so any user who knows git can predict the shape from the name. `add <url>` does three things in one transaction: `git remote add origin <url>`, `git fetch origin grove-meta`, and `git config branch.grove-meta.remote=origin` + `git config branch.grove-meta.merge=refs/heads/grove-meta`. Setting upstream tracking eliminates the failure mode that killed ticgit (orphan-branch refs sit outside the default refspec; `git pull` doesn't bring them — see the postmortem survey).

The alternative — exposing the remote as a `--remote <url>` flag on `grove meta init` — was rejected because `init` can run before a remote is known (a user installs grove on a repo, decides later to multi-machine) and flags don't compose with that later-decided case.

## Why intra-machine concurrency relies on git's own lock
Under the directory-of-files shape (ADR-0004), two concurrent `grove-llm inbox-add` invocations on the same machine write to disjoint filesystem paths — they cannot collide at the filesystem layer. The only contention point is git's `.git/index.lock` when both invocations stage+commit on the shared worktree. The CLI wraps a small retry loop (~2s total, exponential backoff) around the git-lock-contended path before reporting cleanly. Adding a grove-level `flock(2)` above git's lock would duplicate the same mechanism at a different layer for no behavioural difference. Stale-lock recovery is git's own standard mechanism — manual removal if a holding process crashed; we do not auto-remove `.git/index.lock`.

## What this ADR explicitly does not cover
- **Conflict resolution beyond non-ff.** A truly conflicting state (same file modified by two writers, possible only under the legacy single-file shape) is out of scope: ADR-0004's directory shape eliminates the case structurally.
- **Repo-path discovery for cross-repo capture.** Out of scope for v1, per ADR-0003.
- **Multi-remote topologies (mesh sync).** Single-upstream is the supported topology. The git-bug postmortem flags mesh as the least-exercised path even for projects that explicitly designed for it; we do not invent that complexity.
- **Authentication.** Whatever git auth is configured for the user globally applies to the `grove-meta` remote; grove does not own an auth subsystem.

## Walk-away check
The remote config lives in `.grove-meta/.git/config` in the standard git format. Push and fetch work via plain `git push` / `git pull` from inside the worktree (because upstream tracking is set). Delete the `grove` CLI: the worktree is still a normal git worktree on a normal remote-tracked branch. Cron a `cd .grove-meta && git pull --ff-only && git push` and you have grove's `meta sync` re-implemented in one line of shell.

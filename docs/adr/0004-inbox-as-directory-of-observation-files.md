# Each inbox is a directory of one-file-per-observation, not a single appended-to file

An [[Inbox]] is the directory `inboxes/<name>/` on the `grove-meta` branch, containing one markdown file per observation named `<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`. Capture writes a new file; drain deletes triaged files in one session-commit; the directory's existence (preserved by a `.gitkeep`) remains the "this grove is known" signal. This supersedes the single-file-per-grove shape (`inboxes/<name>.md`) described in ADR-0002.

## Status
superseded by ADR-0031 `0031-shed-machinery-keep-self-extension-core-and-methodology.md` — the inbox subsystem is deleted as part of shedding grove's machinery.

## Why this shape, in one paragraph
Under the single-file shape, two writers (parallel groves on one machine, or one grove on two machines) appending to `inboxes/<name>.md` produce a non-ff push on the second writer and a merge-conflict-prone reconcile. The directory-of-files shape places every capture on a disjoint filesystem path, so the same two writers produce two unrelated commits that fast-forward cleanly when either machine pulls. The convergence in the broader industry is decisive on this point — see the rationale section below.

## Why we are confident: the prior-art evidence
The decision is anchored in the in-repo-issue-tracker post-mortem survey at `docs/research/in-repo-issue-tracker-postmortems.md` (commissioned by leaf `050-research-in-repo-issue-trackers.md` as a deliberate de-risking step for this planning leaf). The survey identifies a **unidirectional industry migration**: every team that started with a single shared file and many appenders eventually moved to a directory of fragment files; no surveyed tool migrated in the reverse direction.

- **GitLab's "Great CHANGELOG Conflict Crisis"** (issue [gitlab-org/gitlab-foss#17826](https://gitlab.com/gitlab-org/gitlab-foss/-/issues/17826), [2018 blog post](https://about.gitlab.com/blog/2018/07/03/solving-gitlabs-changelog-conflict-crisis/)) — migrated from single-file `CHANGELOG.md` (with a brief `merge=union` stopgap, [commit 4377ba1c](https://gitlab.com/gitlab-org/gitlab-foss/-/commit/4377ba1c360cf6f4d15e3b5ad2a7ed7bc41f795e)) to `CHANGELOG/unreleased/<entry>.yml` with a release-time merger because rebases dominated review cycles at contributor scale.
- **towncrier** ([docs](https://towncrier.readthedocs.io/en/stable/index.html)) — born inside Twisted specifically because maintainers had tired of single-file changelog conflict resolution.
- **Changesets** ([rationale](https://github.com/changesets/changesets/issues/719)) — designed conflict-free from day one: "all the xxx-changeset.md files are distributed, so teams can easily collaborate on merge/rebase/cherry-pick without any worry about conflict."
- **changie**, **git-cliff** (with `--include-path`), **GitLab's `CHANGELOG/unreleased/`**, **logchange** — independent invention of the same shape across language ecosystems.

The `merge=union` band-aid is documented to silently corrupt on non-unique lines ([Kiselev — On reducing Changelog merge conflicts](https://medium.com/@nettsundere/on-reducing-changelog-merge-conflicts-1eb23552630b)) and is ignored by GitHub's web merge UI ([GitHub Community #9288](https://github.com/orgs/community/discussions/9288)) — i.e. it works locally but fails at the platform layer most teams actually use.

Two prior tools showed the same shape applied to issue-style records and got the walk-away properties right: **bugs-everywhere** (`.be/<dir-uuid>/bugs/<bug-uuid>/values`) and **artemis** (`.issues/<issue-id>/` as a Maildir). Both died from social/ecosystem causes, not from the data-shape choice.

## Why the entry name is `<timestamp>Z-<slug>-<content-hash-8>.md`
Three properties combine into one filename:
- **`<UTC-iso8601-seconds>Z` prefix** — `ls` sorts entries chronologically; the most recent capture is the last line.
- **`<slug>`** — mechanically derived from the first ~40 chars of the observation body (lowercase, `[a-z0-9-]`, collapse dashes). User-overridable with `--slug=`. The directory listing is self-documenting: `ls inboxes/racket-bugs/` shows what each entry is about, without needing the file open. The opaque-UUID alternative was rejected on this exact ground.
- **`<content-hash-8>`** — first 8 hex chars of SHA-256 over the observation body. Two purposes: (1) **idempotency** — capturing the same observation twice produces the same hash, so the CLI checks the target directory for any file ending in `-<hash>.md` before writing and surfaces "already captured at <path>" on hit; (2) **collision avoidance** — distinct content from two writers in the same second produces different hashes.

This shape is influenced by **bugs-everywhere's UUID-keyed directories** (good walk-away) and **Changesets' human-readable slugs** (good `ls` legibility), avoiding both the opacity of one and the chronological unsortability of the other.

## Why drain deletes triaged files, with `.gitkeep` keeping the directory alive
Under the single-file shape, the "known grove" signal was the file's existence at `inboxes/<name>.md`. Under the directory shape, the per-grove directory's existence carries that signal. A `.gitkeep` (or equivalent placeholder) keeps the directory in git after all observations are drained. Drain produces one commit per session — message names the triage disposition counts (e.g. `drain racket-bugs: 3 incorporated, 1 deferred, 1 rejected`) — rather than one commit per observation, to keep the `grove-meta` branch's history at the session granularity that matters for audit.

The Changesets/towncrier pattern is again the model: at consumption time, the fragment files are deleted and an aggregated artefact is produced. For grove there is no aggregated artefact — the triage outcome is consumed into the receiving grove's work itself — but the deletion-on-consumption mechanic is the same.

## Why we did not take the middle path of "single file + lock + pull-rebase"
The middle path keeps `inboxes/<name>.md` as a single file but requires `grove-llm inbox-add` (was `grove inbox add` before the ADR-0006 split) to fetch + rebase + append + push under a lock. It preserves the single-file data model at the cost of multiple new failure modes: rebase races, editor races on the worktree file, and the still-unresolved invisibility of `merge=union` to GitHub's web merge UI (i.e. any team workflow that uses the GitHub UI for merging gets the corrupting behaviour silently). The post-mortem survey notes no surveyed tool retained a single-file shape under multi-writer load.

## Walk-away check
Delete the `grove` CLI: `inboxes/<name>/` is still a directory of plain markdown files on a plain git branch. `find inboxes -name '*.md' | xargs cat` produces every observation ever captured; `git log inboxes/<name>/` shows the full per-observation history with author/date/message. The `<content-hash-8>` suffix is decorative — a reader can ignore it. The `.gitkeep` is a known git convention. No part of the shape requires `grove` to be installed to read.

## Consequences for adjacent decisions
- ADR-0002's storage-shape claim (one file per grove, originally written as `inboxes/<name>.md`) is superseded by this ADR's claim (`<repo>/.grove-meta/inboxes/<name>/<entry>.md`). ADR-0002's other arguments — why a branch and not a refs/* namespace, why CLI-mediated, why drain at bootstrap, why dedicated branch reserved broadly — are unchanged and remain authoritative.
- ADR-0003's cross-repo rule still applies: a cross-repo write to grove `Y` in repo B is a write to the appropriate file under `<repo-B>/.grove-meta/inboxes/Y/` — same gesture, new directory shape.
- The `Inbox`, `Seed`, and `Drain` glossary entries in `CONTEXT.md` are updated inline to reflect the new shape.

## Considered alternatives
The single-file shape (status quo), the single-file + lock + pull-rebase middle path, and a Changesets-style word-pool slug (`tasty-otters-jump.md`) without timestamp were considered and rejected for the reasons above. A pure-UUID entry name (`<uuid>.md`) was rejected on user feedback: directories with opaque entry names are impossible to manually process when something goes wrong. The hash-only variant (`<content-hash>.md`) was rejected because it loses the chronological sort that `<timestamp>-` provides.

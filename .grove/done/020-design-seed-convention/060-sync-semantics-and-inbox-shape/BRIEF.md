# 060-sync-semantics-and-inbox-shape — brief

## Goal

Implement the on-disk shape and sync semantics agreed in the 060 planning
grilling: inbox is a directory of one-file-per-observation; sync is
local-first with opt-in remote, fetch-before-drain, push-best-effort with
one auto-retry. The agreed model is recorded in two new ADRs:

- **ADR-0004** `docs/adr/0004-inbox-as-directory-of-observation-files.md` —
  shape.
- **ADR-0005** `docs/adr/0005-grove-meta-sync-semantics.md` — sync,
  remote-config verb, intra-machine concurrency.

ADR-0002 carries a supersession note pointing at both. The `CONTEXT.md`
glossary entries for `Inbox`, `Drain`, and `grove-meta branch` were
updated inline during the grilling.

## Done when

- `grove inbox add` writes per-observation files at
  `inboxes/<name>/<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`,
  enforces content-hash idempotency, and pushes best-effort with one
  auto-retry on non-ff when a remote is configured.
- The session bootstrap (`grove start` / `grove continue`) calls a CLI
  verb that fetches `grove-meta` (when remote is configured;
  warn-and-continue on failure) and then drains the inbox by
  triaging+deleting the observation files in one session-commit; that
  commit pushes best-effort with one auto-retry.
- `grove meta remote add|remove|list` exists, with `add <url>` setting
  upstream tracking on `grove-meta`.
- `grove meta sync` exists as the manual push-pending + pull-latest
  verb (the cron-friendly entry point).
- Intra-machine concurrency on `grove inbox add` relies on git's
  `.git/index.lock` with a bounded CLI retry (~2s exponential
  backoff); no grove-level lock.
- A `.gitkeep` (or equivalent placeholder) keeps the per-grove
  directory alive after drain so its existence is the "known grove"
  signal.
- Migration from any pre-existing single-file inbox
  (`inboxes/<name>.md`) to the new directory shape is handled
  gracefully by the CLI (one-time move on first encounter, attributed
  in commit message).

This node retires when its children retire.

## Decomposition

Three work leaves, ordered to minimise dependencies. None of them
depend on the rename leaf (`070-grove-meta-rename-and-init.md`)
landing first; they operate against the current branch name
(`grove-inboxes`) and the rename leaf sweeps later.

- `010-cli-shape-and-capture.md` — implement the directory shape and
  the modified `grove inbox add` (content-hash idempotency, push
  policy, git-lock retry). Includes the one-time migration from any
  existing single-file inbox.
- `020-cli-drain-as-verb-and-bootstrap-fetch.md` — promote drain from
  bootstrap prose to a CLI verb (`grove inbox drain` or
  `grove meta drain` — sub-decision in the leaf), wire
  fetch-before-drain into the bootstrap path, push the drain commit.
- `030-cli-meta-remote-and-sync.md` — introduce the `grove meta remote
  add|remove|list` sub-namespace and `grove meta sync`. Sets upstream
  tracking on `add`. Reuses the push/fetch primitives from leaves
  010/020 rather than re-implementing them.

## Decisions (the synthesis from the grilling)

These are the binding decisions the children implement. The full
running log lives in the original planning task file (retired into
`done/` alongside this BRIEF's creation); ADRs 0004 and 0005 are the
durable record. Children should cite the ADRs, not this brief.

- **Inbox shape**: `inboxes/<name>/<entry>.md` (directory per grove).
  ADR-0004.
- **Entry naming**: `<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`.
  Slug is mechanically derived from the first ~40 chars of the
  observation body (lowercase, `[a-z0-9-]`, collapse dashes); explicit
  `--slug=<override>` allowed. Content-hash is first 8 hex chars of
  SHA-256 over the observation body. ADR-0004.
- **Drain pull**: fetch + ff-merge before drain when remote is
  configured; warn-and-continue on fetch failure; refuse-and-instruct
  on non-ff. Skip silently when no remote configured. ADR-0005.
- **Capture push**: best-effort after commit when remote configured;
  warn-and-continue on network failure; on non-ff auto-fetch +
  ff-replay-push *once*, then refuse loudly. Skip when no remote.
  ADR-0005.
- **Drain push**: same policy as capture push. ADR-0005.
- **Post-drain state**: triaged files deleted; `.gitkeep` preserves
  the directory's existence as the "known grove" signal; one commit
  per drain session, message names disposition counts. ADR-0004.
- **Remote config verb**: `grove meta remote add|remove|list`. `add`
  sets upstream tracking. ADR-0005.
- **Intra-machine concurrency**: rely on git's `.git/index.lock`
  with a small CLI retry; no grove-level lock. ADR-0005.

## Binding principle for all three children

**Workflow lives in CLI verbs, not LLM prose.** User steer (2026-05-28):
"this should all be done via CLI commands, rather than the LLM running
the workflow each time." Every step decided in the grilling becomes the
body of a CLI subcommand; launcher prompts invoke the verb whose
contract guarantees the workflow rather than re-explaining it. This is
the leaf-080 audit principle promoted to a binding precondition of these
children's CLI changes.

## Interaction with leaf 070 (rename + meta init)

070's job is unchanged (rename `grove-inboxes` → `grove-meta` + add
`grove meta init`). It does not absorb shape/sync work. The children
of this node operate against the current branch name
(`grove-inboxes`) and the rename leaf sweeps later — the additional
path-edit churn is acceptable.

Sub-coordination: leaf `030-cli-meta-remote-and-sync.md` introduces
the `grove meta` subcommand parent if it doesn't exist yet (clap
parents are extensible). When 070 lands and adds `grove meta init`,
the parent already exists.

## Pointers

- ADR-0004 (shape): `docs/adr/0004-inbox-as-directory-of-observation-files.md`
- ADR-0005 (sync): `docs/adr/0005-grove-meta-sync-semantics.md`
- ADR-0002 (now superseded in part): `docs/adr/0002-grove-inboxes-branch-and-inbox-model.md`
- Post-mortem research that drove the decisions:
  `docs/research/in-repo-issue-tracker-postmortems.md` (its
  "Findings adopted" section is the bridge to the ADRs).
- Glossary: `CONTEXT.md` — `Inbox`, `Drain`, `grove-meta branch`.

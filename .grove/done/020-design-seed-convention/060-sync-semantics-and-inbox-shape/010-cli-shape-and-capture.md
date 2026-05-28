# 010-cli-shape-and-capture

**Kind:** work

## Goal

Implement the directory-of-files inbox shape and the modified
`grove inbox add` verb. After this leaf retires, capture writes a new
per-observation file at
`inboxes/<name>/<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`,
enforces content-hash idempotency, and pushes best-effort with one
auto-retry when a remote is configured.

## Scope

- **On-disk shape change.** `grove inbox add --to=<name>` now writes a
  new file under `inboxes/<name>/` rather than appending to
  `inboxes/<name>.md`. The directory is created on first capture for a
  given grove if it doesn't exist; a `.gitkeep` is added in the same
  commit so the directory survives subsequent drains.
- **Entry filename**: `<UTC-iso8601-seconds>Z-<slug>-<content-hash-8>.md`.
  Slug derivation: take the first ~40 chars of the observation body
  after stripping markdown headers/leading whitespace, lowercase,
  replace non-`[a-z0-9]` with `-`, collapse runs of `-`, strip
  leading/trailing `-`. If the result is empty, fall back to
  `untitled`. `--slug=<override>` flag bypasses the mechanical
  derivation.
- **Content-hash idempotency.** Hash the observation body (the full
  markdown content, not the slug-stripped version) with SHA-256; take
  the first 8 hex chars. Before writing, list the target directory
  for any file ending in `-<hash>.md`; on hit, print
  `already captured at <path>` to stderr, exit 0, no commit.
- **One-time legacy migration.** If `inboxes/<name>.md` exists (from a
  pre-shape-change install), move it to
  `inboxes/<name>/<UTC-iso8601-seconds-of-the-move>Z-legacy-<content-hash-8>.md`
  in a separate commit before adding the new entry, with a commit
  message naming the migration (`migrate inbox <name> to directory
  shape`). This runs lazily on first add to a legacy single-file
  inbox.
- **Push policy.** If `grove-meta` has an upstream configured (per
  ADR-0005), push after commit. On non-ff: `git fetch` + replay-push
  *once*; if still non-ff or other error, print remediation and exit
  non-zero. On other network failures: warn-and-continue.
- **Git-lock retry.** Wrap the stage+commit step in a retry loop that
  catches "Another git process seems to be running" / `.git/index.lock
  exists` errors with exponential backoff (e.g. 100ms, 200ms, 400ms,
  800ms, 1500ms — ~3s total over five tries) before reporting.
- **CLI surface** is single-shot, non-interactive: `grove inbox add
  --to=<name> [--slug=<slug>] [--body=<text> | --body-file=<path> |
  --body-stdin]`. No wizard.

## Out of scope

- Drain (covered by leaf 020).
- `grove meta` namespace verbs (covered by leaf 030).
- Branch rename (covered by leaf 070 at the parent level).
- The `--cross-repo` path discovery — ADR-0003 explicitly defers
  this; cross-repo writes still work whenever the writer can name the
  target repo's path manually.

## Done when

- `grove inbox add` writes per-observation files in the shape
  specified above and the existing `inboxes` tests are updated to
  cover the new shape.
- Capturing the same observation twice produces one file plus a
  stderr "already captured" notice on the second invocation.
- A pre-existing `inboxes/<name>.md` is migrated lazily on first add
  to a legacy single-file inbox, in a dedicated commit.
- Concurrent `grove inbox add` invocations on the same machine
  serialise via git's index lock with the retry loop; no observed
  data loss or duplicate writes in a manual stress test (e.g.
  `seq 1 10 | xargs -P 10 -I{} grove inbox add --to=test
  --body="entry {}"`).
- Push best-effort policy implemented; manual test with a configured
  remote shows pushes happening after capture commits, and a deliberately
  raced second writer triggers the one-shot auto-retry path.
- ADR-0004 referenced from the inbox-related code comments (one
  pointer line where the directory-shape logic lives, naming
  `0004-inbox-as-directory-of-observation-files.md`).

## Pointers

- ADR-0004: `docs/adr/0004-inbox-as-directory-of-observation-files.md`
- Existing inbox CLI: `src/inboxes.rs` (entry point for the changes
  here).
- Sibling parent BRIEF: `.grove/020-design-seed-convention/060-sync-semantics-and-inbox-shape/BRIEF.md`.

## Notes

- The push-after-commit behaviour is intentionally simple — there is no
  queue, no retry-on-the-next-invocation. If the push fails and the
  user keeps working offline, the commits accumulate locally and
  `grove meta sync` (leaf 030) is the catch-up path.
- The `.gitkeep` survives drains by design (see ADR-0004). Do not
  delete it during the directory-creation step.

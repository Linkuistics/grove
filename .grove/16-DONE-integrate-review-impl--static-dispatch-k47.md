# static-dispatch-k47

**Integrates:** static-dispatch-k46

## Goal

Triage the findings in `static-dispatch-k46` against the committed standalone
`harness-dispatch` implementation and its contract. Apply the valid findings,
record the disposition of every finding, and verify the resulting behavior.

## Context

The producer is the completed `static-dispatch-k12` node. The review is anchored
to producer tip `e0d5c67d5e69`; read its own retired leaf for findings and source
evidence rather than treating this charter as a prescribed fix list.

The implementation lives in `crates/harness-dispatch/`. Its area contract is
`docs/specs/harness-selection-and-execution.md`, with the policy-ownership and
pre-exec evaluation ADRs cited by that spec. Read the producer's recorded
verification evidence and the package's current tests when judging the review.

## Done when

Every review finding has a supported disposition. Valid defects are corrected
with meaningful verification appropriate to their behavior and the required
project checks, and the integration is retired and committed. Any durable
decision made during triage is reconciled with the documentation that owns it.

## Notes

This task is placed immediately after the review so its cited implementation
does not change before triage. Later leaves already own the remaining delivery,
computed-context and evaluation-boundary increments; keep their contracts in
view when deciding what belongs in this integration.

## Decisions (running log)

**Findings read from the review's commit.** `womusstwlsps` (`328835912f9b`)
adds `static-dispatch-k46`'s retired leaf with findings F1–F7; the working copy
still matches it. Each was graded against the committed source at that tip,
not against the review's prose.

**Triage: all seven are real issues in the artifact, fixed here.** Each was
re-derived from source before being accepted:

- F1 — `worker.rs` sends `entry.to_string_lossy()`, and the worker imports that
  string. Real. Probing Bun 1.4.2 found a sibling the review did not name: a
  `?` anywhere in the entry path is read as a query, even through a `file:`
  URL. `policy.ts?x` imports `policy.ts`, and `d?q/policy.ts` imports `d.ts`,
  so a different file's code runs. `#`, `%`, spaces and newlines import
  exactly, and `\` fails to load without substitution. The fix refuses both
  substituting classes at the authority stage, before a worker starts.
- F2 — the close-on-exec sweep stops at the soft `RLIMIT_NOFILE`, capped at
  65,536. Real: descriptor 100 under a soft limit of 64 reaches the worker. The
  fix marks every descriptor this process actually holds, listed before the fork.
  If that list cannot be read, the evaluation refuses; the rlimit bound is
  not a fallback. `ambient-authority-k30` does not own this control; the
  skeleton does.
- F3 — `create` makes the state directory recursively without syncing the
  parents that gain new entries. Real, by the fsync(2) contract. SQLite's EXTRA
  directory sync covers only the store's own directory. The fix syncs the
  parent of every directory the invocation found missing before it commits.
  A failed sync refuses with exit 4.
- F4 — NUL passes validation in literal arguments and in `model` and `effort`.
  A `program` containing NUL refuses only at resolution, as unexecutable (126).
  Real. The fix refuses NUL in every catalog string that can reach argv as
  `policy_invalid`, with its location, catalog-wide.
- F5 — `path.filter(|p| !p.is_empty())` treats an empty PATH as unset. Real, and
  against the spec's `execvp` wording. An unset PATH still refuses, because
  there is then no caller's PATH to search.
- F6 — the reproduction converts argv[0], every value and the cwd lossily.
  Real: a non-UTF-8 `--task-id` or `--choice` refusal advertises a different,
  valid inspection. The fix reports the reproduction as unavailable, naming
  the input, whenever any word or the directory is not UTF-8. A reversible
  byte encoding would need a new JSON shape, and `$'…'` quoting is not portable
  to every POSIX shell; neither is worth it for input that this path otherwise
  refuses.
- F7 — `main` applies the pre-parse `--json` scan to every failure after a
  successful parse. Real. The fix makes the parsed flag decide format once
  parsing succeeds. The raw scan remains only for a command line that did not
  parse and for `record observe`, whose words are never parsed.

**Verification: each fix has a command-seam test whose control fails without it.**
Every new test was run once against the fixed source. It was then run with
that fix mutated back to the reviewed behavior, and the source was restored.
The rlimit-bounded sweep leaks descriptor 100 under a soft limit of 64 (F2).
Without the `?` check, `--config policy.ts?x` evaluates and exits 0 (F1).
Without the directory sync, a new hierarchy under an unopenable parent
launches (F3). Without the NUL check, both the shape table and the run test
fail (F4). The old empty-PATH filter refuses `PATH=""` (F5). A lossy
reproduction is advertised again (F6), and the pre-parse scan renders
`--prompt --json` as JSON (F7). F6's test also shows its firing configuration
in the same run: the lossy `deep�` really selects a catalog candidate.
F1's non-UTF-8 case is a unit test of `import_specifier`: APFS refuses
non-UTF-8 names, so no command-seam fixture can exist on this host.

**Accepted trade-off (F3): a failed sync leaves its directories behind.**
Only directories the invocation found missing are synced. If a sync refuses,
the directories already created stay. A retry then finds them present and does
not sync them. Syncing every ancestor on every run would close that gap, but
it costs a full-cache flush per level on macOS for every launch. The failure
needs an unreadable parent or an I/O error, and the first attempt launched
nothing.

**Contract reconciled where it is owned.** The area spec's authority, program,
records and diagnostics sections now state the entry-path rule, the descriptor
listing, the first-use directory sync, the NUL rule, empty versus unset PATH,
the unavailable reproduction and parsed-flag format. The package README mirrors
each. `runtime-evidence.md` records the Bun `?` observation for recheck at the
next Bun upgrade. No refusal code was added, and no ADR-bar decision arose.
No walkthrough book covers this crate.

**Checks.** `bash scripts/check.sh` passed all twelve steps with exit 0.
Before the run, all edits were finished. The digest of the changed source,
tests and contract documents matched before and after the run. An earlier run
was stopped and discarded because two late edits landed during it. This macOS
host runs only the `/dev/fd` branch of the descriptor listing. The
`/proc/self/fd` branch first runs in `dispatch-delivery-k16`'s per-target
installed smoke. A failed listing there refuses with exit 5 before any worker
starts, so a broken branch cannot pass silently.

**Retirement closes no node.** This leaf sits at the grove root, and
`dispatch-delivery-k16` and its successors are still live.

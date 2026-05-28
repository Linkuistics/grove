# 010-shape-the-feature

**Kind:** planning

## Goal

Pin down the output-shape decisions for showing the three grove-methodology
versions (installed CLI / repo-materialised / worktree-materialised) and
their drift in `grove status`, `grove list`, and the TUI. Grow this tree
with whatever work leaves the agreement implies.

## Context

- `src/status.rs` — current `grove status`. Already prints repo per-harness
  versions with cross-harness drift warning; does not look inside worktrees.
- `src/list.rs` — current `grove list`. Bare names, one per line.
  Machine-parseable; output-shape changes have a compatibility cost.
- `src/version.rs` — current `grove version`. CLI version + per-harness repo
  versions. Has overlap with `grove status`; whether to consolidate is open.
- `src/version_md.rs` — `VERSION.md` format and accessor.
- `src/tui.rs` — no version handling today; new surface to design.
- Glossary `CONTEXT.md` — does not yet have an entry for the three-layer
  version model; add inline once we name the layers.

## Done when

- The three version layers have agreed names (used everywhere).
- Drift-detection rule is agreed (string equality of `VERSION.md` stamps?
  what about `VERSION.md` missing?).
- Output-shape decisions are made for each of: `grove status`, `grove list`,
  `grove version`, TUI.
- Tree is grown: implementation leaves under `010-shape-the-feature/`, or
  sibling `020-…`, `030-…` leaves.

## Decisions (running log)

- **2026-05-29 — "active grove" is not the topic.** The working title was
  misleading; the user clarified the scope is version visibility and drift
  across (installed CLI, repo, worktree). No "active grove" detection is
  in scope. Brief updated accordingly.
- **2026-05-29 — Layer names are `cli` / `repo` / `worktree`.** Short,
  evocative, match the directory each lives in. Added inline to
  `CONTEXT.md` as a single combined entry.
- **2026-05-29 — Drift rule: raw string-equality, both values shown on
  mismatch, no semver awareness.** Reversed an in-flight semver decision:
  the user prefers the tool stay out of interpretation and just show both
  stamps when they differ — the reader works out major/minor/patch. No
  `semver` crate dependency. Unknown (missing/malformed `VERSION.md`) is
  rendered `(unknown)`, not flagged as drift. Orphan worktrees (harness
  uninstalled in repo) show `repo=(none)` but are not warned unless
  cli/worktree itself drifts.
- **2026-05-29 — `grove status` output shape: option α (append per-row).**
  Keep the existing two-section layout; append `worktree=X.Y.Z` (and a
  trailing `⚠ repo=A.B.C` on mismatch) to each grove row. Smallest delta
  from current output; drift signal sits next to the value that drifted.
  The existing `harness=…` row column (from the harness stamp, written
  only in multi-harness repos) is dropped — `worktree=` lives at the same
  position and a `[harness]` prefix can be added on grove rows in
  multi-harness repos when needed. CLI version appears in the existing
  header line: `grove cli X.Y.Z, installs in <repo>:`.
- **2026-05-29 — `grove list` is removed wholesale.** `status` covers
  every use case `list` did. No migration flag on `status` (no
  `--names-only` etc.) — anyone scripting against `grove list` adapts to
  parse `grove status` or pin to an older grove version. Implementation:
  drop `src/list.rs`, the `Command::List` arm in `src/cli.rs:276`, and
  the doc line in `docs/grove.md:62`. The removal is a CLI breaking
  change and will want a CHANGELOG entry under a major-version bump (or
  an explicit minor-with-breaking-change note, per however the grove
  repo handles this; check `release.toml` / CHANGELOG conventions during
  implementation).
- **2026-05-29 — `grove version` is removed.** Same reasoning as `list`:
  its output is a subset of the new `grove status`. `grove --version`
  (clap auto-flag) still answers the CLI-only question. Implementation:
  drop `src/version.rs` and the `Command::Version` arm in `src/cli.rs`.
  Verify clap's `--version` works after removal. Same CHANGELOG /
  versioning treatment as the `list` removal.
- **2026-05-29 — TUI placement: option I (header bar).** cli + repo
  versions appear in a top-row header above the grove list; visible on
  both `Screen::GroveList` and `Screen::GroveDetail`. Drift styling
  (e.g. coloured header text) makes the signal hard to miss. Per-row
  `worktree=…` continues to slot into `grove_row` (`src/tui.rs:808`).
  `render` (`src/tui.rs:710`) gains a Rect for the header above the
  current body area.
- **2026-05-29 — Scope pivot: this grove also covers merging
  `install`/`update` into one verb.** Evidence collected during grilling:
  `cli.rs:14,16` define both verbs with the *same* `InstallArgs` struct;
  both call `install::run(&args, mode)`; the only behavioural difference
  is a pre-flight safety check (`install.rs:37-51`) — create-only vs
  update-only. The actual materialise work is identical. Brief expanded
  to a two-concern (visibility + surface cleanup) goal; merge shape is
  the next question to grill.
- **2026-05-29 — Merged-verb behaviour: option D (idempotent, verbose).**
  No safety flag. Behaviour: not installed → install; installed at same
  version → no-op; installed at different version → update. *Always*
  print the outcome (`installed @ X`, `already at X, no change`,
  `updated X → Y`) so the action is visible without being interactive.
  Implementation: collapse the `Mode` enum out of `src/install.rs`; the
  pre-flight check at lines 37-51 is removed and replaced by per-harness
  outcome tracking (read existing `VERSION.md` if present, compare to
  target version, decide install vs no-op vs update).
- **2026-05-29 — Merged verb is named `grove install`.** Keep the
  existing name; `grove update` is removed wholesale, no alias. Matches
  Homebrew / apt convention where re-running install over an existing
  install is the normal way to upgrade. Idempotency (decided above) makes
  the dual semantics safe; the always-on outcome line makes it visible.
  Same CHANGELOG / breaking-change treatment as the `list` / `version`
  removals.
- **2026-05-29 — Two ADRs: 0007 (status canonical / list+version
  removed) and 0008 (install idempotent / update merged in).** Skip ADRs
  for layer names (in glossary), drift rule (observable from code),
  output shape (observable from code), TUI placement (not architectural).
  Pair each ADR with the leaf that implements its decision so the
  rationale lands together with the code.

## Notes

Grilling discipline: one question at a time, recommended answer with
evidence, walk down the design tree (`grilling.md`). Update `CONTEXT.md`
inline as terms settle. ADR only when a decision is hard to reverse and a
real trade-off (`driving.md`).

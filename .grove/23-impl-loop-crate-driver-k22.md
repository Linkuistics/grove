# loop-crate-driver-k22

## Goal

Move the driver into `grove-loop` — the lease, the prompt composition, the
completion signal and the loop itself — and make `grove` a thin binary crate.
What remains of grove is then a launcher that owns a loop and a vocabulary, and
nothing else.

## Context

`docs/specs/module-decomposition.md`, decision 9 — `DriverLease`, `Mandate`,
`compose`, `run`, `LoopOutcome`.

The loop composes the other three modules: **`exists? → create or find next →
determine the command → run → finalise`**. It holds grove's vocabulary — kind,
handle, prompt composition — and the one-driver-per-workspace lease.

`minimalism-k1` records **what the loop physically cannot delegate**, and it is
the whole justification for this crate existing at all:

1. **relaunch** — a session cannot restart itself with fresh context;
2. **kill** — codex's Seatbelt denies a same-sandbox process signalling its own
   session;
3. **kind before launch** — the kind selects the **vendor**, and a session cannot
   re-route itself after starting;
4. **the bootstrap prompt** — a skill cannot tell you to load it.

Everything else is already skill-driven: every tree mutation is a verb a session
invokes, and `finish-commit` — the whole teardown — is already invoked by the
finish session's skill, not by the driver.

## Done when

- `src/driver_lease.rs`'s surviving lease half (~900 lines by `minimalism-k1`'s
  measurement, less after the deletions), `src/loop_driver.rs`, `src/prompt.rs`
  and `src/complete.rs` are in `grove-loop`.
- `DriverLease::{acquire, worktree_root, revalidate}` takes a `&Workspace` and
  gets its control directory from `Workspace::control_dir(namespace)` — the
  namespace being grove's, which is what `extract-jj-workspace-k9` made sayable.
- `run(workspace, lease, templates) -> Result<LoopOutcome, Error>` is the whole
  loop; `LoopOutcome` is `Finished` or `Stopped`.
- `src/main.rs` becomes its own thin binary crate under `crates/`, and the
  workspace root stops being a package. One workspace, one release version, one
  changelog — check `release.toml`, `docs/RELEASING.md` and
  `scripts/release-*.sh` against the new layout, because the release process
  assumed a root package.
- `src/` at the repository root is gone.
- The composed-loop test seam exists — **test seam 2**: the loop driving a fake
  harness binary end to end. Today's driver, completion and lease suites, much
  shrunk.
- `cargo test` and `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is the last structural move.

**Reinstall in this session**, per `grammar-separator-k15`'s sequence, and expect
the release scripts to need attention: the root package moving is exactly the
kind of change that breaks a cut silently and is only noticed at release time.
`cargo release patch` **without** `--execute` is a dry run and is the cheap check.

**The lease is the one thing here that is not a move.** Its derivation of a
control directory changes shape, and `docs/adr/one-live-driver-per-working-tree.md`
was reworked for that at `extract-jj-workspace-k9`. Re-read the reworked record
rather than the code's memory of the old one.

**Do not let the loop grow back.** The four things above are the whole of what it
may own. Anything a session can do, a session does — through verbs it invokes.

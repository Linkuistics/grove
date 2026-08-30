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

## Decisions (running log)

**The workspace root stops being a package, and `crates/grove` is the human's
binary.** `src/main.rs` and `src/cli.rs` moved there; `src/driver_lease.rs`,
`src/loop_driver.rs`, `src/prompt.rs` and `src/session_config.rs` moved into
`crates/grove-loop`; `src/` is gone. `crates/grove` has **no `[lib]`** — a
library on that package would give the binary something to reach into, which is
the property decision 1 made it a crate to keep.

**One release version is now a manifest fact, not a convention.**
`[workspace.package] version = "19.6.0"`, and every member takes
`version.workspace = true`. That is what makes `grove_loop::VERSION` answerable:
the prompt publishes grove's release version (decision 10) and the loop composes
the prompt, so the constant has to be readable from `crates/grove-loop`, which
would otherwise have answered `0.1.0`. Both binaries read the one constant.

**`run` takes a template *source*, not a loaded configuration.** The spec sketch
says `run(workspace, lease, templates: &Templates)`; a loaded snapshot handed in
once cannot express the two things the loop does today, and both are load-bearing
rather than incidental. The configuration is re-read **once per iteration**, so a
session that adds a kind to `config.kdl` is launched from the document as it
stands; and the just-in-time presence rule is asked against the **pre-transition**
load, which is the document as it stood before the tree was mutated
(`docs/adr/complete-session-configuration.md`). So the third argument is
`TemplateSource` — where the templates are read from — and `SessionConfig` stays
what it was, the per-iteration snapshot. The delta roots are not held in it: they
come from the workspace `run` is already given, so there is one derivation.

**`DriverLease::acquire` takes a resolved `&Workspace`, and `main_repo()` is
gone.** The lease used to resolve a path itself and hold both the worktree root
and the main repo. Its caller now has to resolve a workspace anyway — `run` takes
one — so a second resolution inside the lease could only disagree with the first.
`run` reads `workspace.main_repo()` directly. That also made `session_prompt`
infallible: it used to resolve a workspace it could prove was there, and carried
an unreachable error arm to do it.

**`grove_loop::driver` stays public, against its own prediction.** Its header
said the module would stop being public at this leaf. It did not:
`materialize_finish` and `transition_to_current` are the only way to put a tree
into the two states `tests/verbs.rs` has to test against — the pre-loop
transition, and the driver-reserved `finish` leaf `leaf-add` refuses to write.
Crate-private would have bought nothing the compiler can check and cost a second
copy of that file's jj fixture harness inside the crate. The header now says so
rather than predicting again.

**The shared test helpers live at `testing/support.rs`.** They were
`tests/support/mod.rs` of the root package, which no longer exists; they are
about the repository rather than any of the three packages that use them, so they
moved out of every package and each consumer keeps a `tests/support/mod.rs` shim
that names the one file by `#[path]`. A root `tests/` directory cargo silently
ignores would have been a trap.

**The two human-CLI surface assertions moved into the binary they are about.**
`crates/grove-llm/tests/help_surfaces.rs` and `removed_surface.rs` each held one
test over `grove::cli::Cli`; a binary-only package's clap model is reachable only
from inside it, so both are now unit tests in `crates/grove/src/cli.rs`. The
~25-line clap walker is duplicated there, and the duplication is stated at the
site: the alternative was a `[lib]` on `crates/grove` existing only so a test in
another package could import it.

## Why this leaf does not install anything

The brief requires each remaining leaf to **re-derive** whether it is a cutover
leaf rather than inherit the label, and the test is the matrix `k6` ran: *is
there a cell where the **installed** build meets the tree this leaf leaves and
fails?*

**There is no such cell, because this leaf leaves the tree byte-identical in
shape.** It adds, removes and renames no `.grove/` entry, moves no filename
grammar, and writes no witness. Every change is inside the Rust workspace —
which package a module lives in, which manifest carries the version, which
directory a test file sits in. The installed 19.6.0 binaries meet exactly the
tree they met before this session started, parse it with exactly the code they
already have, and drive it the same way. Nothing a session installs would reach
the running driver anyway, and there is nothing here it would need to.

The task note's *reinstall in this session* was about the second half of the same
sentence — *expect the release scripts to need attention*. That check was made,
and it found real breakage: `release.toml`'s `pre-release-replacements` resolves
its `file` against the **released package's** manifest directory, which moved
from the repository root to `crates/grove`, so the standing `file =
"CHANGELOG.md"` aborts the cut with *unable to find file
crates/grove/CHANGELOG.md*. It is now `../../CHANGELOG.md`.

**The dry run could not be taken in this working tree, and the substitute is
named rather than skipped.** `cargo release` drives git and this is a jj-native
secondary workspace with no `.git`, so it refuses here — *could not find
repository* — exactly as `release.toml` already says (*run it from the colocated
workspace*). The colocated workspace is parked on `main` and holds none of this
leaf's changes, so running it there would have dry-run the **old** layout and
proved nothing. So the check was taken against a git fixture holding *this*
tree: the whole working tree minus `target/`, `.jj/` and `.grove-worktrees/`,
copied out, `git init`, committed, `cargo release patch` (no `--execute`). It
reproduced the failure with the old path, and with the fix it reports

```
   Upgrading workspace to version 19.6.1
   Upgrading grove from 19.6.0 to 19.6.1 (inherited from workspace)
   … five more members, each (inherited from workspace)
   Replacing in ../../CHANGELOG.md
```

with the `## v19.6.1` heading landing under the standing `## Unreleased` and
above the accumulated entries, which is what the replacement is for. That also
confirms the workspace-inherited version: one field moves six packages, and
`release = false` on the five non-released members buys them no tag, no
changelog section and no publish — not a frozen version. Their manifests said
otherwise and now say this.

**Test seam 2 is `crates/grove/tests/loop_driver.rs` and `lifecycle_cutover.rs`,
and the shrinking already happened.** Both drive the **real** `grove` process
against an isolated `$HOME` carrying a complete `config.kdl` and a fake
configured command — the composed loop over a fake harness, end to end — and
they moved into the binary's own package because that is where
`CARGO_BIN_EXE_grove` names something. *Today's driver, completion and lease
suites, much shrunk* was written against the pre-decomposition suite and has
been paid across the run rather than at this leaf: `keyed-launch-run-k11` took
the spawn, the channel and the escalation into `crates/keyed-launch/tests`, and
`loop-crate-verbs-k21` took the verb and completion coverage into
`crates/grove-loop`. What is left here is what only a real driver process can be
asked — its own stderr, its session-epoch bookkeeping, its response to being
signalled, and its ownership of one foreground child — so nothing was cut for
the sake of the sentence.

# shared-skill-dir-clobber-review-k16

**Reviews:** shared-skill-dir-clobber-k13

## Goal

Try to **disprove** `docs/adr/one-build-owns-a-session.md`. Nothing is
implemented yet, so this is a review of an argument, not of code: read the ADR,
then check each load-bearing claim against `src/` and against what a configured
session actually does. A claim that survives is cheap; one that does not is
cheapest to kill now, before `one-build-owns-a-session-k17` builds on it.

## Why this was cut

The design rests on three empirical claims that were checked once each, by the
session that wanted them to be true, and it rejects the option a reader is most
likely to reach for. Both halves are worth a fresh context. It also changes what
an existing safety guard *measures*, which is the kind of change that looks
obviously right until the case it used to catch stops being caught.

## Specific things to attack

- **"The guard checks the binary that cannot disagree."** The ADR's sharpest
  claim: `loop_driver::checked_grove_llm` prefers `current_exe()`'s sibling and
  the caller discards the resolved path (`loop_driver.rs:103`), so the session
  runs whatever its own `PATH` gives. Verify both halves. Is there *any* path by
  which the driver's resolution reaches the session — an inherited variable, a
  wrapper convention, a harness that resolves tools differently? If the sibling
  can reach the session, the whole reversal is wrong.
- **The refusal's false-positive shape, which stalls the loop.** The new check
  resolves `grove-llm` in the *driver's* environment. A template that re-derives
  `PATH` — `bash -lc`, `ssh`, `docker run`, a wrapper script — makes the driver's
  measurement irrelevant, and a *refusal* on a mismatch it cannot see correctly
  is worse than the mismatch: bare `grove` launches nothing. The ADR argues this
  is the same assumption the version check already made. Test that: today's
  resolution prefers the **sibling**, which for an ordinary install exists and
  matches, so today's guard almost never consults `PATH` at all. Does moving to
  `PATH`-only convert a check that is silent-by-luck into one that fires? In
  particular: a human who invokes `/opt/homebrew/bin/grove` with a `PATH` that
  lacks `grove-llm` is admitted today and refused after this change.
- **Whether the driver should refuse at all**, or warn and let the session's own
  `grove-llm` be the only refusing surface. The ADR gives `grove-llm` a
  never-refuse rule and the driver a refusing one; argue whether that split is
  principled or merely inherited from where the code already was.
- **`build.rs` reproducing `include_dir!`'s file set.** The constant is only
  correct if the build script's traversal selects exactly the files
  `include_dir!` embeds — hidden files, symlinks, nested empty directories,
  anything ignored. The ADR's answer is one equality test. Is a test that fails
  *after* a divergence the right guard, or does one traversal need to be the
  source of the other? Check what `include_dir` 0.7 actually includes.
- **The 320 KB claim.** Measured as `grove` 2 283 056 bytes vs `grove-llm`
  1 963 824, with the content marker string present only in `grove`. Confirm the
  delta is the embed and not release-profile noise, and that adding
  `--content-hash` as a *constant* really keeps `grove-llm` free of it — a
  careless implementation that calls `provision::content_hash` instead would
  relink the embed and silently spend the saving.
- **`--content-hash` staying outside the agent grammar.** The ADR says a flag is
  invisible to `tests/provision.rs`. Verify against `exposed_verbs()` (walks
  `clap::Command` subcommands) *and* `scan_instructed_verbs` (scans content for
  `grove-llm <word>`). Does either see a flag? Does adding one to `llm_cli` force
  a subcommand-shaped surface in clap's derive?
- **What per-iteration re-verification actually closes.** A clobber lands at an
  arbitrary moment; the driver checks between sessions. Work out the fraction of
  the window it covers, and whether the remaining window makes the check
  ceremony. If the honest answer is "it makes the alternation visible", the ADR
  should say only that.
- **The rejected `PATH` shim.** The ADR rejects it partly because it would need
  `complete-session-configuration` amended. Is that reasoning circular — an ADR
  declining a fix because another ADR currently says otherwise, when ADRs here
  are edited in place by design? And is "install it (`cargo install --path .`)"
  a remedy a dogfooder will actually follow, or one that will be routed around?
- **Glossary grain.** The new **Build pairing** entry in `CONTEXT.md` is mostly
  mechanism, and the file's preamble says "Definitions only." `k14` raises the
  same objection about the **Embedded methodology** entry. Judge whether
  **Build pairing** and **Methodology identity** should collapse into one
  definition plus `_Avoid_` lines, with the three checks left to
  `docs/ARCHITECTURE.md`.
- **Dangling or wrong citations.** The reconciliation touched
  `docs/ARCHITECTURE.md`, `docs/USAGE.md`, `docs/specs/config-driven-sessions.md`,
  `docs/adr/supported-workspace-layouts.md`, `CONTEXT.md` and `CONTEXT-MAP.md`.
  Check the anchor `#the-boundary-is-a-build-not-a-commit` resolves, that the new
  ADR is registered in `CONTEXT-MAP.md`, and that nothing still describes the
  question as open.

## Out of scope

Do not implement anything — `one-build-owns-a-session-k17` owns that. Do not run
build, test, lint or format commands; this is an inspection of a decision and its
documents. `provisioned-skill-refresh-review-k14` owns the `k9` contract; where
the two overlap (the "only writer" property, the per-iteration argument), leave
`k9`'s wording to that review and judge only what this ADR added.

## Verification already done (re-check, don't redo)

Read, not run: `grove`/`grove-llm` both report `17.0.0` and `Cargo.toml` is
`17.0.0` while `content/` has moved a release ahead — the case a version
comparison cannot see. The installed `grove-llm` still exposes `leaf-add-chain`
and `leaf-promote-chain`, confirming the current pair is coherent v17.0.0 and
that this session is not itself an instance of the defect.

## Findings

### P1 — The fatal `PATH` preflight cannot identify the CLI an opaque configured session will run

`docs/adr/one-build-owns-a-session.md:23-40` says the driver resolves
`grove-llm` "the way the session will," then immediately concedes that a login
shell, `ssh`, container, or wrapper may re-derive `PATH` and make the result
wrong in either direction. Those wrappers are not an edge outside Grove's
launch contract: `docs/adr/complete-session-configuration.md:3-15,36-41` makes
wrappers and their environment part of the configuration owner's policy, and
`docs/specs/config-driven-sessions.md:152-159,170-187,209-212` says they remain
opaque by design. A valid wrapper can therefore have the correct CLI and still
be refused before it launches; the agent-side warning cannot "cover the far
side" of that false positive because the agent is never started.

The comparison also does widen the old guard's assumption, contrary to the ADR:
`src/loop_driver.rs:382-412` currently prefers `current_exe()`'s sibling and
consults `PATH` only when that file is absent, while
`run_configured_loop_with_lease` binds the result only to `_grove_llm` and
`launch_configured_session` independently executes config-expanded argv. No
checked path, variable, or wrapper convention reaches the child. The new check
would always treat the driver's `PATH` as representative where the old ordinary
install path did not.

Do not implement a fatal proxy check under the claim that it observes the
session. Either narrow the supported launch contract to environment-preserving
targets (and reconcile the complete-configuration ADR), or make the driver-side
probe advisory/best-effort and state that the actual CLI behind an opaque target
is observable only when it runs. Reconsider the rejected shim/expected-identity
options on their merits; citing the current no-hidden-environment decision is
not enough when the chosen refusal has the same wrapper limitation.

### P2 — `cargo install --path .` is not the promised one-command remedy

`docs/adr/one-build-owns-a-session.md:65-72,91-99` says installing the checkout
makes the machine coherent and is the single remedy the refusal should print.
In this session's actual environment, `/opt/homebrew/bin` is PATH entry 7 while
`~/.cargo/bin` is entry 26; both current Grove binaries resolve from Homebrew.
`cargo install --path .` would install the checkout pair under `~/.cargo/bin`
without replacing or outranking the Homebrew pair. Invoking the new
`~/.cargo/bin/grove` explicitly would still have its driver-side probe resolve
the old `/opt/homebrew/bin/grove-llm`, refuse, and prescribe the already-completed
install again.

State the real requirement — the intended pair must occupy the active PATH
resolution (or an explicitly supported equivalent) — and make diagnostics name
the mismatching resolved path. Do not claim that Cargo installation alone fixes
every grove on the machine.

### P2 — The config-session spec still specifies the old sibling/version guard

The producer's reconciliation missed
`docs/specs/config-driven-sessions.md:374-380`, which still requires sibling-first
resolution and a crate-version check, and the test-seam list at lines 1288-1289,
which still requires sibling/PATH and version-skew coverage. Both contradict the
new PATH-only methodology-identity design stated near the top of the same spec.
Reconcile these passages with whatever the P1 resolution leaves current.

### P3 — The proposed methodology identity is not literally the identity of the whole embedded tree

`src/provision.rs:172-189` hashes only file paths and bytes. `include_dir` 0.7.4
also embeds a `DirEntry::Dir` for every directory, including an empty one (see
the upstream [`expand_dir`](https://docs.rs/crate/include_dir_macros/0.7.4/source/src/lib.rs#433-465)); its sorted `read_dir` includes hidden entries, and
`Path::is_dir` / `Path::is_file` follow symlinks. The current `content/` happens
to contain neither symlinks nor empty directories, so the file set agrees today,
but adding or moving an empty directory changes the embed and extracted tree
without changing the stamp. A build-script traversal and the runtime traversal
can both omit it, so their proposed equality test still passes.

Either include typed directory paths in the identity or narrow the ADR and code
comments from "the embedded tree" / "which embed" to the file payload that is
semantically relevant, explicitly making empty directories non-identity. The
equality test remains the right guard against the two independent file
traversals drifting.

### P3 — The two glossary terms are distinct, but both definitions carry mechanism

`CONTEXT.md:29-36` defines **Methodology identity** and then names `build.rs` and
the CLI flag; lines 56-64 define **Build pairing** and then enumerate all three
checks. The terms should not collapse: one names the value and the other the
invariant it is meant to support. But the implementation inventory violates the
glossary's definition-only rule at lines 5-7 and duplicates
`docs/ARCHITECTURE.md`. Keep two short definitions and their useful `_Avoid_`
guards; leave the build-script, flag, and check sequence in the architecture and
ADR.

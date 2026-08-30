# Preservation baseline — grove v19.3.0

The observable contract of this repository **before** the modularity refactor,
measured rather than described. The workstream's *Preservation ledger* said what
must survive; this file is that ledger with the values filled in, so a later
phase can check a change against a recorded fact instead of against an
impression.

**The ledger itself, recorded here because its original home does not
outlive the workstream.** Preserve unless a change explicitly records an
approved exception:

- CLI verb names, arguments, help shape, structured/human output fields, and
  exit-status meanings.
- Configuration keys, environment overrides, defaults, and the current
  `session-kinds-v1` `.grove` format.
  **Excepted for the leaf filename grammar, from `grammar-separator-k15`.** A
  leaf's session kind and its slug are now separated by `--`
  (`NN-[DONE-|ABANDONED-]<kind>--<slug>-k<key>.md`), so every leaf filename
  captured below — and every refusal quoting one — records the spelling of
  v19.3.0 rather than one that must still hold. Node directory names, the
  outcome infixes, the terminal `-k<key>` and the handle are all unchanged. The
  exception is the approved kind this ledger allows for, argued in
  [`task-names-are-canonical`](adr/task-names-are-canonical.md) and
  [`docs/specs/module-decomposition.md`](specs/module-decomposition.md)
  decision 3; the measurements themselves stay as they were taken. The
  `.grove/FORMAT` witness that same entry names is also gone — its writers and
  readers went at `delete-migration-k6`, and this leaf's release removed the
  last file.
- Abstract outcomes across Git, native jj, and colocated jj workspaces.
  **Excepted for Git, from `drop-git-lane-k7`.** Grove now drives jj only and
  refuses a working tree with no `.jj/` before any mutation, so every Git row
  measured below records a lane that has been dropped rather than one that must
  still hold. The exception is the approved kind this ledger allows for, argued
  in [`jj-is-the-only-lane`](adr/jj-is-the-only-lane.md); the measurements
  themselves stay as they were taken.
- Methodology embedding/provisioning, package and binary names, release/install
  behaviour, MSRV 1.85, and the Linux glibc 2.17 compatibility target.
  **Excepted for embedding and provisioning entirely, from
  `delete-provisioning-k19`.** No build embeds a methodology, writes a skill
  directory, keeps a harness registry, answers `--content-hash`, or reports a
  build pairing; the methodology ships as the `grove` plugin, installed by a
  human. Section 4 below, and every provisioning line in the captured
  transcripts, therefore records a delivery path that has been **dropped** rather
  than one that must still hold. The measurements themselves stay as they were
  taken, and the argument is `docs/specs/module-decomposition.md`, decision 11.
- Fail-closed ownership: Grove never resets, merges, deletes, or rewrites work
  it cannot prove belongs to the current finish attempt.

**How to use it.** Every claim below is either a *captured* transcript — a real
command, its real streams, its real exit status — or a *read* fact with the file
it was read from. When a `documentation-k2` or `implementation-k3` leaf changes
something in this file's scope, it re-runs the capture and diffs. A difference is
either an approved exception, recorded here with the decision that approved it,
or a regression.

**What it is not.** Not a design, not a plan, not an account of how anything
works — `docs/ARCHITECTURE.md` is that, and `docs/adr/` records why. This file
holds values, versions and transcripts and nothing else.

**Lifetime.** It outlives the grove: `.grove/` is deleted by `finish`, and a
before-change ledger that vanishes with it would leave the refactor unable to
prove it preserved anything. Edit it only when a preservation exception is
approved, and say which decision approved it.

The measurement anchor is jj change `uwuvxpkowmpumtukrknzxqptvpklmlwp`, commit
`149994afa5f3a8d6fd04178b60754e085dfb960f`, measured 2026-08-24 on
macOS 26.5.2 (build 25F84), `arm64` (Darwin 25.5.0). Working copy empty over that
commit at capture time.

Referenced from `docs/formalism-findings.md`, *Experiment 2 — pre-registration*,
which is where the formal phase's hypotheses and measures live. This file carries
no hypotheses.

---

## 1. Toolchain, as measured

Recorded so that a failure appearing later can be attributed. **Every tool the
formal and build phases need is present**; nothing in this table is a stub or a
substitute.

| tool | version | how it is found |
|---|---|---|
| rustc | `1.98.0 (88d9e12ae 2026-08-18)` (Homebrew) | `PATH` |
| cargo | `1.98.0 (797e8a9bc 2026-08-05)` (Homebrew) | `PATH` |
| git | `2.55.0` | `PATH` |
| jj | `0.44.0` | `PATH` |
| Quint | `0.32.0` | `PATH` (also resolvable as `npx quint`) |
| Node | `v26.7.0` | `PATH` — Quint's runtime |
| Alloy | `6.2.0.202501090817` (git `794226d`) | `~/.local/share/alloy/org.alloytools.alloy.dist.jar`, SHA-256 `6b8c1cb5bc93bedfc7c61435c4e1ab6e688a242dc702a394628d9a9801edb78d` |
| Java (for Alloy) | Corretto `21.0.12.1+9-LTS` | `~/.local/share/jdk/amazon-corretto-21.jdk/Contents/Home/bin/java` |
| Alloy solver | SAT4J, Alloy's bundled pure-Java default | not overridden by `run-alloy.sh` |
| `grove` / `grove-llm` | `19.3.0`, `/opt/homebrew/bin/` | Homebrew-installed release build |

### The Java trap, recorded because it is live on this machine

`java` on `PATH` is **Corretto 16.0.1** — below Alloy 6's floor. `run-alloy.sh`
does not use it: it probes `$JAVA`, then `PATH`, then `~/.local/share/jdk/*`, and
takes the first candidate at major ≥ 17. That search is what makes the suite
green here.

This matters beyond convenience. Alloy reports "no instance found" for a run that
never started, so a JVM that fails to launch turns *every* `check` into a pass
and *every* `witness` into a failure — a dead tool reading as a broken model. The
runner therefore aborts on `Error|Exception|UnsupportedClassVersionError|LinkageError`
in the output rather than recording it as a result. **A repository runner built by
the formal phase must keep this property**; the brief's "fail if a tool silently
did no work" is this exact hazard, and this machine is a live instance of it.

## 2. Pre-existing suite state — all green

Anything failing later is therefore introduced, not inherited.

| suite | command | result | wall clock |
|---|---|---|---|
| Rust workspace | `cargo test --locked --workspace` | **exit 0** — 1210 passed, 0 failed, 0 ignored, across 58 test binaries (2 of them doc-test targets, both empty) | 2m 20s |
| Build | `cargo build --locked --all-targets` | **exit 0**, no warnings | 12s |
| Clippy | `cargo clippy --locked --all-targets` | **exit 0**, no warnings (workspace denies `clippy::all`) | 4s |
| Alloy | `docs/ordinal-fs-tree/models/run-alloy.sh` | **exit 0** — 20/20 commands pass (7 `check`, 13 `witness_*`) | 17s |
| Quint | `docs/ordinal-fs-tree/models/run-quint.sh` | **exit 0** — 148/148 claims hold across 8 instances | 3m 29s |

The Quint suite's cost is worth recording as a number the formal phase will have
to budget against: 3m 29s wall clock for **2527s of CPU** (≈1214% — it saturates
the machine). `rollback_fails` alone runs at six times the sample budget because
one of its witnesses lands in roughly 0.07% of traces.

## 3. Packages, binaries and release surface

**Workspace**, root package `grove` v19.3.0, one member `crates/ordinal-fs-tree`
v0.1.0. `resolver = "2"`, edition 2021 for both.

| name | kind | path |
|---|---|---|
| `grove` | binary | `src/main.rs` — the human command |
| `grove-llm` | binary | `src/bin/grove-llm.rs` — the session verbs |
| `grove` | library | `src/lib.rs` — shared by both binaries |
| `syllabus` | binary (member, `cli` feature, on by default) | `crates/ordinal-fs-tree/bin/syllabus.rs` |

- **MSRV 1.85**, both packages. Set by edition-2024 dependencies (`clap` 4.6,
  `clap_lex` 1.1, `assert_cmd` 2.2), and established by running toolchains rather
  than by any check in the build: `rustup run 1.85 cargo check --locked
  --all-targets` passes, `1.84` fails during dependency resolution. There is no
  CI in this repository.
- **Release targets** (`scripts/release-common.sh`, single source of truth):
  `aarch64-apple-darwin`, `aarch64-unknown-linux-gnu`, `x86_64-unknown-linux-gnu`.
- **Linux glibc floor 2.17** (`LINUX_GLIBC` in `scripts/release-build.sh`), via
  `cargo zigbuild --target <triple>.2.17`. The Darwin target builds natively.
- **Not a crates.io crate** — `release.toml` sets `publish = false`; distribution
  is three `.tar.xz` archives plus a rendered Homebrew formula.
  `crates/ordinal-fs-tree` is excluded from the cut (`release = false`), so it has
  no version bump, tag or changelog section of its own.
- **Release gate**: before archiving, both staged binaries are grepped for the
  marker `hierarchical, self-extending workstreams`, a phrase that exists only in
  `content/`. A binary missing the embed fails the release.

### Material dependencies

93 crates in the locked graph. Direct, `grove`:

`anyhow` 1.0.102 · `clap` 4.6.1 · `include_dir` 0.7.4 · `kdl` 4.7.1 ·
`libc` 0.2.186 · `ordinal-fs-tree` 0.1.0 (path) · `serde` 1.0.228 ·
`serde_json` 1.0.150 · `sha2` 0.10.9 · `shell-words` 1.1.1 · `tempfile` 3.27.0.
Dev: `assert_cmd` 2.0, `tempfile` 3.10. No build-dependencies — `build.rs` is a
`std`-only `rerun-if-changed` walk.

`ordinal-fs-tree` as grove takes it (`default-features = false`) imposes exactly
**one** transitive dependency: `libc`. That is a load-bearing claim, not a note —
`tests/library_dependency.rs` holds it against `cargo metadata`.

## 4. The embedded methodology, provisioning, and the launch blocker

> **Dropped, not preserved.** Every mechanism this section measures was deleted
> at `delete-provisioning-k19` under the ledger exception recorded above. What
> follows is the record of what 19.3.0 did, kept because the launch blocker below
> is part of this workstream's history.

`content/` — 29 files, 1740 lines — was compiled into **both** binaries with
`include_dir!`. `grove` extracted it verbatim into every installed harness's
personal skill directory; `grove-llm` hashed it to name which build it is.

The **methodology identity** is a SHA-256 over the embedded file payload:
files sorted by path, each contributing a little-endian `u64` length prefix and
bytes for its `content/`-relative path, then the same for its contents. Embedded
directories are excluded, so an empty directory is not part of a build's identity
(`one-build-owns-a-session`, a record retired with the mechanism at
`delete-provisioning-k19`). It was computed from the linked embed, not from a
constant recorded beside it — which is what made comparing it to a provisioned
directory worth doing.

### Baseline value, and the launch blocker resolved

The root brief records a skew: the CLI reported `10db…` while the installed
driving skill reported `8501…` when this tree was created. **It is repaired and
verified.** Three independent sources agree:

| source | value |
|---|---|
| `grove-llm --content-hash` (installed 19.3.0) | `10db034c77d5afe455998ad5ac58c969c66aaa0d0312077172bc1bf1bf96444b` |
| `~/.claude/skills/grove/.grove-content-hash` | `10db034c…` |
| `~/.codex/skills/grove/.grove-content-hash` | `10db034c…` |
| `~/.pi/agent/skills/grove/.grove-content-hash` | `10db034c…` |
| this working tree's `content/`, hashed by the algorithm above | `10db034c…` |

The remediation was re-provisioning, which is what bare `grove` does on every
invocation before it touches a working tree; the three skill directories carry an
mtime of 2026-08-24 17:50, after the tree was scaffolded at 17:09. **No further
action is required, and no later leaf need treat this as a blocker.**

The fourth row is the one worth keeping: the working tree's `content/` hashes to
the same value as the installed binary's embed, so this checkout and the binary
driving it are the same methodology. Any leaf that edits `content/` breaks that
agreement until a rebuilt `grove` re-provisions — which is a normal consequence,
not a fault, but it is the check to run when a session's skill and its CLI seem to
disagree.

## 5. The `.grove` tree format

`.grove/FORMAT` contains exactly `session-kinds-v1\n` (17 bytes). It is written
**last** when a root is scaffolded, via a same-directory rename from
`.FORMAT.tmp`, so no reader observes a torn marker.

`src/tree_format.rs` classifies three cases, and both diagnostics are captured
verbatim in §8:

| on disk | classification | exit |
|---|---|---|
| `FORMAT` holds `session-kinds-v1\n` | current | proceeds |
| `FORMAT` absent | **legacy** — "must be migrated before current tree operations can run" | 1 |
| `FORMAT` holds anything else | **unsupported** — names both found and required | 1 |

**A reserved witness outranks all three, and there are *three* reserved
classes, not two.** Any of

| reserved name under `.grove/` | what it witnesses |
|---|---|
| `MIGRATING-session-kinds/` | a session-kind migration interrupted mid-flight |
| `PREPARING-FINISH-<finish-handle>-<attempt-identity>/` | a finish transaction **built but not yet published** — the tree beside it still looks perfectly walkable, which is exactly why the refusal is by reserved prefix rather than by whether the tree looks intact |
| `FINISHING-<finish-handle>/` | a finish transaction **published**, holding the evacuated root entries |

makes every ordinary reader and mutator refuse, and that check runs *before*
format classification. `src/task_name.rs` classifies all three as
`Verdict::Reserved` and `src/tree_access.rs` holds the two finish prefixes
(`PREPARING_FINISH_PREFIX`, and the `FINISHING-` name), so the refusal is the
library's wherever the name sits. Captured in §8 H2: a tree with a migration
witness **and** no `FORMAT` reports the pending migration, not the legacy
format; and in §8 H3 for the preparing class. This ordering is what keeps an
evacuated finish tree from reading as a malformed grove
(`docs/adr/task-tree-transactions-fail-closed.md`).

**`PREPARING-FINISH-` was absent from this section on first pass**, which is a
recording error rather than a behaviour one: `src/finish_transaction.rs:554`
builds the name and `tests/finish_lifecycle.rs:1461` asserts an ordinary reader
refuses it. It is named here because `extract-task-tree-k24` and
`extract-finish-baseline-k26` will move the classifier and the prefix constants
apart, and a ledger listing two of three reserved classes would license
dropping one.

**Note on the legacy diagnostic, and the removal that was withdrawn.** The
legacy diagnostic instructs the operator to migrate. A planned breaking change
would have deleted migration, leaving that string pointing at a capability the
binary no longer had, and the leaf that owned the replacement wording —
`remove-migration-k22` — was to rewrite it as how to start a current-format
root. **That leaf never ran and the removal is not owed:** its phase was
abandoned on a cost-against-value judgement, so migration, the
`MIGRATING-session-kinds/` witness class, its refusal, and the driver-only
transition in `src/tree_lifecycle.rs` all stand as measured here. The
diagnostic therefore still names something the binary can do. There is **no
user-facing migrate verb** — migration happens inside bare `grove`'s driver
path only.

## 6. The observable command surface

### `grove` — the human binary

No subcommands and no flags beyond clap's own. Bare `grove` provisions every
installed harness's skill directory, acquires the working-tree driver lease, and
runs one configured foreground session per selected task until the agent stops
signalling (`src/cli.rs`, `src/launch.rs`).

```text
Grove: hierarchical workstream tool for AI agents

Usage: grove

Options:
  -h, --help     Print help
  -V, --version  Print version
```

### `grove-llm` — the session verbs

Thirteen subcommands plus the `--content-hash` metadata flag. The surface is:

| verb | positional | options | writes |
|---|---|---|---|
| `root-init` | `[SLUG]` (default `plan`) | — | working tree |
| `pick` | — | — | reads |
| `brief-chain` | `[LEAF_PATH]` | — | reads |
| `kind` | `[LEAF_PATH]` | — | reads |
| `resolve` | `<REFERENCE>` | — | reads |
| `leaf-add` | `<PARENT> <SLUG>` | `--kind` (default `impl`) | working tree |
| `leaf-add-pair` | `<PARENT> <STEM>` | — | working tree |
| `leaf-insert` | `<TARGET> <SLUG>` | `--kind` (default `impl`) | working tree |
| `leaf-decompose` | `<LEAF_PATH> <FIRST_CHILD_SLUG>` | `--kind` | working tree |
| `leaf-retire` | `<LEAF_PATH>` | — | working tree |
| `leaf-prune` | `<PATH>` | — | working tree |
| `finish-commit` | `<FINISH_HANDLE>` | — | working tree **and** VCS |
| `complete` | — | `--done`, `--signal-file` | signal file only |

`finish-commit` is the only verb that commits. Every other mutating verb is
working-tree-only.

**Nothing in the suite pins this table.** `tests/help_surfaces.rs` asserts a
weaker and different property — that no listed argument or subcommand renders as
a blank row, walked over clap's own `Command` model rather than the rendered
text. `tests/llm_cli.rs` checks a *representative sample* of four verbs
(`pick`, `root-init`, `leaf-add`, `complete`) and that six retired launcher verbs
stay absent. `tests/methodology.rs` cross-checks one direction only — the
embedded corpus instructs no verb the CLI lacks — plus that the verb surface is
flat. So a verb silently gaining an option, losing one, or changing its argument
arity passes today. This table is the record that would catch it.

### Addressing is not uniform, and the asymmetry is part of the contract

| takes a **reference** — `[n]`, `n`, `[n]-slug`, `<slug>-k<key>`, or a bare slug | takes a **path** only — absolute, or relative to `.grove/` |
|---|---|
| `resolve <REFERENCE>` | `brief-chain [LEAF_PATH]` |
| `leaf-add <PARENT> …` | `kind [LEAF_PATH]` |
| `leaf-insert <TARGET> …` | `leaf-decompose <LEAF_PATH> …` |
| | `leaf-retire <LEAF_PATH>` |
| | `leaf-prune <PATH>` |

A key handed to a path-taking verb is read as a path and fails with `ENOENT`,
naming `.grove/<key>` — captured in §8 D2. No help text states this as a rule;
it is inferable only from each verb's argument documentation. Recorded because a
crate extraction that regularises addressing would be changing the contract, not
tidying it.

### The nineteen session kinds

```text
requirements  design  planning  prototype  impl
review-requirements  review-design  review-planning  review-prototype  review-impl
integrate-review-requirements  integrate-review-design
integrate-review-planning  integrate-review-prototype  integrate-review-impl
research-a  research-b  combine-research
finish
```

`finish` is driver-reserved: `leaf-add --kind finish` and `leaf-insert --kind
finish` refuse it. An unrecognised `--kind` errors listing all nineteen in the
order above — captured verbatim in §8 D.

### Exit-status meanings

Both binaries return `anyhow::Result` from `main`, so:

| status | meaning | example |
|---|---|---|
| **0** | the verb did its job — **including** several "nothing matched" outcomes | `resolve` not-found, `resolve` ambiguous, `pick`/`kind`/`brief-chain` on a spent grove |
| **1** | operational refusal or error; a single `Error: …` block on stderr | rootless tree, wrong format, pending witness, already-retired leaf, reserved kind, bad slug |
| **2** | clap usage error — unknown flag, unknown subcommand, missing argument | `grove --nope`, `grove-llm nope` |

**Exit status is not the not-found channel.** `resolve` exits 0 whether it
resolved, found nothing, or found several; the caller reads stdout-emptiness.
Likewise `pick` on a spent grove exits 0 with empty stdout. A refactor that
"improves" either into a nonzero status breaks every caller that branches on
status — which is what the loop driver does.

**The 1/2 split is unasserted.** Enumerating every `code(` site under `tests/`
for the `grove` package finds exactly one, and it is a function *name*
(`no_withdrawn_tree_module_is_named_anywhere_in_grove_code`) — not an exit-code
assertion. The suite's 114 nonzero-exit assertions all read `!status.success()`,
which cannot tell a refusal from a usage error. Exact codes are asserted only in
`src/loop_driver.rs` (a child's chosen `Some(7)`) and captured in
`crates/ordinal-fs-tree/tests/driving_a_tree.rs`. So clap's `2` is contract by
observation, recorded here, and by nothing else.

### Output and error categories

| stream | carries |
|---|---|
| **stdout** | results only, one absolute path per line. `leaf-add-pair` prints exactly three; `leaf-decompose` two (brief, then first child); `root-init` three (brief, first leaf, then `FORMAT` **last**); `leaf-prune` one per newly-marked leaf. Nothing at all when the operation produced no result. |
| **stderr, exit 0** | information: the spent-grove note, `resolve`'s `note: … is retired (DONE)` / `… is abandoned (ABANDONED)`, `resolve`'s ambiguity list, `leaf-insert`'s renumber summary and cross-reference list, and the two-steps-remain guidance after `leaf-retire` / `leaf-prune`. |
| **stderr, exit ≥ 1** | one `Error: …` block, `anyhow`-formatted, with a `Caused by:` chain where there is one. |
| **structured output** | none. There is no `--json`, no machine-readable mode, and no output flag on any verb. Line-oriented stdout **is** the machine interface. |

A retired or abandoned match prints its path on stdout *and* its own stderr note,
so a resolved dead end never looks live. That pairing is the contract, not the
note alone.

## 7. Configuration, defaults and environment

**Personal file**: `~/.config/grove/config.kdl`. A flat set of **nineteen**
top-level KDL nodes, one per session kind, each with a single positional string
holding a complete command template. No properties, no child blocks. All nineteen
must appear **exactly once**; there are no defaults, families, profiles or
inheritance, and nothing is assembled from a precedence chain.

> **Excepted, from `keyed-launch-templates-k10`.** The *quantifier* is gone: a
> kind may appear at most once, and presence is checked per kind at the moment
> the kind is used rather than over a set of nineteen. Everything else in this
> section still binds — one complete template, read whole, out of one file — and
> the delta gains an explicit rule of its own: **a kind resolves only if the
> personal file declares it**. Every row below that reports `missing session
> kinds` or `unknown session kind` records behaviour that has been deliberately
> removed. See `docs/adr/complete-session-configuration.md`.

**Per-checkout delta**: `.grove.kdl`, same grammar, **any subset** of the
nineteen. Looked for at the worktree root first, then the main repository root;
**the first of the two that holds a file is the delta** and the other is not read.
The two are never merged with each other. Each declared kind replaces one whole
template. It must be **untracked**, and Grove enforces that — a tracked delta
names a program to execute. It sits *beside* `.grove/`, never inside it, because
`finish` deletes that directory wholesale.

**Template substitutions**: `${prompt}`, `${session_name}`, `${worktree}`,
`${repo}`. Details and the `#` rule: `docs/CONFIGURATION.md`.

**Environment.** Grove reads no configuration from the environment. What it does
own is a set of variables it **removes** from spawned children, listed in
`src/launch.rs`:

| set | members | why |
|---|---|---|
| loop control | `GROVE_SIGNAL_FILE`, `GROVE_HARNESS_PID`, `GROVE_CLAUDE_PID` | `GROVE_SIGNAL_FILE` is the driver's kill channel; whoever holds it can end the session, and an environment is inherited rather than addressed |
| internal test seams | `GROVE_TEST_FINISH_CLEANUP_FAIL_AT`, `…_PAUSE_AT`, `…_BARRIER`, `GROVE_TEST_FINISH_REBIND_EXIT_AT`, `…_FAIL_AT`, `GROVE_TEST_FOREIGN_FILESYSTEM` | deterministic failure controls must never leak from a developer shell into a configured session |
| repository context | `GIT_DIR`, `GIT_WORK_TREE`, `GIT_COMMON_DIR`, `GIT_INDEX_FILE` | process-global overrides; `current_dir` alone does not stop a Git-aware child following an inherited foreign repository |

Scrubbing is the **default** and granting the exception: the loop driver's own
session spawn scrubs too, then sets the one path it owns.

**Relocated, not changed, at `keyed-launch-run-k11`.** `src/launch.rs` was
absorbed into `src/loop_driver.rs` when the spawn and the escalation moved to
`crates/keyed-launch`; the loop-control list is now that module's
`LOOP_CONTROL_ENV`, handed to the runner as `Launch::scrub`, and the other two
sets went earlier with the machinery that read them. The members and the rule
below are unchanged, so this is a pointer correction rather than an exception —
every `src/launch.rs` in the rows that follow reads as `src/loop_driver.rs`.

**The three sets above are `src/launch.rs`'s scrub lists — they are not the
complete set of actionable internal test seams, and this ledger does not claim
they are.** `src/finish_transaction.rs` ships two more, read by
`finish_test_checkpoint`:

| seam | read at | in `src/launch.rs`'s scrub list? |
|---|---|---|
| `GROVE_TEST_FINISH_EXIT_AT` | `src/finish_transaction.rs:86` — `std::process::exit(86)` at the named checkpoint | **no** |
| `GROVE_TEST_FINISH_FAIL_AT` | `src/finish_transaction.rs:89` — `bail!` at the named checkpoint | **no** |

Both are live shipped failure controls that a spawned session therefore
inherits. Recorded as a **measured gap**, not as an approved exception: it is
the same class of leak `INTERNAL_TEST_SEAM_ENV` exists to close, and the six
listed there are the ones that were closed. Remediating it is product-code work
and belongs to `implementation-k3`, not to the formal phase; what belongs here
is the fact, so a later leaf does not read the six-member list as exhaustive and
"preserve" the gap by copying it.

`tests/env_hygiene.rs` holds a narrower claim than its name suggests — see §10.

`grove-llm complete --signal-file` defaults to `$GROVE_SIGNAL_FILE`; with that
absent the verb is a safe near-no-op that tells the operator to exit manually.

**Supported workspace layouts**: plain Git, native jj, and colocated jj — the
three shapes the finish transaction must behave symmetrically across
(`docs/adr/supported-workspace-layouts.md`). The tree these measurements were
taken in is a **jj-native secondary workspace** (`refactor-for-modularity`, one of
three in this repository), which is the layout with no `.git` of its own.

## 8. Captured transcripts

Every block below is a real run against a throwaway fixture. **A through I and
J** need nothing but `git init`-ing an empty repository and repeating the
commands in order. **K, L and M** need three further things, all of them
ordinary rig rather than special access, and each is named where it is used: an
isolated `$HOME` (so bare `grove` validates a configuration and provisions into
a fixture rather than the operator's home), a jj fixture for M's second and
third layouts, and — for K's refused case only — the shipped
`GROVE_TEST_FOREIGN_FILESYSTEM` seam, because a second filesystem is the one
operand this host cannot stage for real.

Absolute paths are rewritten to `<worktree>`, `<home>` and `<fixture>`; jj
author and email columns are elided; nothing else is edited.

### A — binary identity
#### grove --version
```console
$ grove --version
grove 19.3.0
exit: 0
```

#### grove-llm --version
```console
$ grove-llm --version
grove-llm 19.3.0
exit: 0
```

#### grove-llm --content-hash
```console
$ grove-llm --content-hash
10db034c77d5afe455998ad5ac58c969c66aaa0d0312077172bc1bf1bf96444b
exit: 0
```

#### unknown flag (clap usage error)
```console
$ grove --nope
--- stderr ---
error: unexpected argument '--nope' found

Usage: grove

For more information, try '--help'.
exit: 2
```

#### unknown verb (clap usage error)
```console
$ grove-llm nope
--- stderr ---
error: unrecognized subcommand 'nope'

  tip: a similar subcommand exists: 'complete'

Usage: grove-llm [OPTIONS] [COMMAND]

For more information, try '--help'.
exit: 2
```

### B — rootless working tree (no .grove/)
#### pick on a rootless tree
```console
$ grove-llm pick
--- stderr ---
Error: grove root not found: <worktree>/.grove
exit: 1
```

#### kind on a rootless tree
```console
$ grove-llm kind
--- stderr ---
Error: grove root not found: <worktree>/.grove
exit: 1
```

#### brief-chain on a rootless tree
```console
$ grove-llm brief-chain
--- stderr ---
Error: grove root not found: <worktree>/.grove
exit: 1
```

#### resolve on a rootless tree
```console
$ grove-llm resolve 1
--- stderr ---
Error: grove root not found: <worktree>/.grove
exit: 1
```

#### leaf-add on a rootless tree
```console
$ grove-llm leaf-add . thing
--- stderr ---
Error: grove root not found: <worktree>/.grove
exit: 1
```

### C — current-format initialization
#### root-init (fresh)
```console
$ grove-llm root-init baseline
<worktree>/.grove/BRIEF.md
<worktree>/.grove/01-requirements-baseline-k1.md
<worktree>/.grove/FORMAT
exit: 0
```

#### .grove/FORMAT contents
```console
$ cat .grove/FORMAT
session-kinds-v1
exit: 0
```

#### tree after root-init
```console
$ find .grove -type f
.grove/FORMAT
.grove/01-requirements-baseline-k1.md
.grove/BRIEF.md
exit: 0
```

#### root-init again (refuses)
```console
$ grove-llm root-init baseline
--- stderr ---
Error: grove root already exists: <worktree>/.grove
exit: 1
```

#### pick
```console
$ grove-llm pick
<worktree>/.grove/01-requirements-baseline-k1.md
exit: 0
```

#### kind
```console
$ grove-llm kind
requirements
exit: 0
```

#### brief-chain
```console
$ grove-llm brief-chain
<worktree>/.grove/BRIEF.md
exit: 0
```

### D — growing the tree
#### leaf-add (default kind impl)
```console
$ grove-llm leaf-add . second
<worktree>/.grove/02-impl-second-k2.md
exit: 0
```

#### leaf-add --kind design
```console
$ grove-llm leaf-add . third --kind design
<worktree>/.grove/03-design-third-k3.md
exit: 0
```

#### leaf-add --kind finish (driver-reserved, refused)
```console
$ grove-llm leaf-add . nope --kind finish
--- stderr ---
Error: `finish` is driver-reserved and cannot be created by `leaf-add`
exit: 1
```

#### leaf-add --kind bogus (lists the nineteen)
```console
$ grove-llm leaf-add . nope --kind bogus
--- stderr ---
Error: --kind must be one of `requirements`, `design`, `planning`, `prototype`, `impl`, `research-a`, `research-b`, `combine-research`, `finish`, `review-requirements`, `review-design`, `review-planning`, `review-prototype`, `review-impl`, `integrate-review-requirements`, `integrate-review-design`, `integrate-review-planning`, `integrate-review-prototype`, `integrate-review-impl`, got "bogus"
exit: 1
```

#### leaf-add bad slug
```console
$ grove-llm leaf-add . Bad_Slug
--- stderr ---
Error: slug "Bad_Slug": a slug holds lowercase ASCII letters, digits and dashes only
exit: 1
```

#### leaf-add-pair
```console
$ grove-llm leaf-add-pair . question
<worktree>/.grove/04-research-a-question-k4.md
<worktree>/.grove/05-research-b-question-k5.md
<worktree>/.grove/06-combine-research-question-k6.md
exit: 0
```

#### leaf-insert
```console
$ grove-llm leaf-insert 2 urgent
<worktree>/.grove/02-impl-urgent-k7.md
--- stderr ---
leaf-insert urgent: renumbered 5 siblings:
  02 -> 03  (03-impl-second-k2.md)
  03 -> 04  (04-design-third-k3.md)
  04 -> 05  (05-research-a-question-k4.md)
  05 -> 06  (06-research-b-question-k5.md)
  06 -> 07  (07-combine-research-question-k6.md)
cross-references to review (verb does not auto-rewrite):
exit: 0
```

#### leaf-decompose
```console
$ grove-llm leaf-decompose 3 first-child
--- stderr ---
Error: resolving path <worktree>/.grove/3

Caused by:
    No such file or directory (os error 2)
exit: 1
```

#### tree now
```console
$ find .grove -mindepth 1 -maxdepth 2
.grove/07-combine-research-question-k6.md
.grove/03-impl-second-k2.md
.grove/05-research-a-question-k4.md
.grove/02-impl-urgent-k7.md
.grove/06-research-b-question-k5.md
.grove/04-design-third-k3.md
.grove/FORMAT
.grove/01-requirements-baseline-k1.md
.grove/BRIEF.md
exit: 0
```

### D2 — decompose, retire, prune (path-addressed verbs)
#### leaf-decompose by key (paths only — key is read as a path)
```console
$ grove-llm leaf-decompose 3 first-child
--- stderr ---
Error: resolving path <worktree>/.grove/3

Caused by:
    No such file or directory (os error 2)
exit: 1
```

#### leaf-decompose by grove-root-relative path
```console
$ grove-llm leaf-decompose 03-impl-second-k2.md first-child
<worktree>/.grove/03-second-k2/BRIEF.md
<worktree>/.grove/03-second-k2/01-impl-first-child-k8.md
exit: 0
```

#### tree after decompose
```console
$ find .grove -mindepth 1 -maxdepth 2
.grove/07-combine-research-question-k6.md
.grove/05-research-a-question-k4.md
.grove/02-impl-urgent-k7.md
.grove/06-research-b-question-k5.md
.grove/04-design-third-k3.md
.grove/FORMAT
.grove/01-requirements-baseline-k1.md
.grove/BRIEF.md
.grove/03-second-k2
.grove/03-second-k2/01-impl-first-child-k8.md
.grove/03-second-k2/BRIEF.md
exit: 0
```

#### leaf-retire
```console
$ grove-llm leaf-retire 01-requirements-baseline-k1.md
<worktree>/.grove/01-DONE-requirements-baseline-k1.md
--- stderr ---
leaf-retire: two steps remain:
  1. commit this session's work, including this rename
  2. run `grove-llm complete` as your last action
exit: 0
```

#### leaf-retire again (refuses)
```console
$ grove-llm leaf-retire 01-DONE-requirements-baseline-k1.md
--- stderr ---
Error: leaf is already retired (DONE): 01-DONE-requirements-baseline-k1.md
exit: 1
```

#### leaf-retire a brief (refuses)
```console
$ grove-llm leaf-retire BRIEF.md
--- stderr ---
Error: cannot retire a brief (briefs are never done): BRIEF.md
exit: 1
```

#### pick after retiring the first leaf
```console
$ grove-llm pick
<worktree>/.grove/02-impl-urgent-k7.md
exit: 0
```

#### leaf-prune a node subtree
```console
$ grove-llm leaf-prune 04-design-third-k3.md
<worktree>/.grove/04-ABANDONED-design-third-k3.md
--- stderr ---
leaf-prune: two steps remain:
  1. commit this session's work, including this rename
  2. run `grove-llm complete` as your last action
exit: 0
```

#### leaf-prune the grove root (refuses)
```console
$ grove-llm leaf-prune .
--- stderr ---
Error: path <worktree> is not under grove root <worktree>/.grove
exit: 1
```

### E — resolve
#### resolve by bare key
```console
$ grove-llm resolve 2
<worktree>/.grove/03-second-k2
exit: 0
```

#### resolve by bracketed key
```console
$ grove-llm resolve [6]
<worktree>/.grove/07-combine-research-question-k6.md
exit: 0
```

#### resolve by handle
```console
$ grove-llm resolve question-k5
<worktree>/.grove/06-research-b-question-k5.md
exit: 0
```

#### resolve by slug (unique)
```console
$ grove-llm resolve urgent
<worktree>/.grove/02-impl-urgent-k7.md
exit: 0
```

#### resolve by slug (ambiguous)
```console
$ grove-llm resolve question
--- stderr ---
resolve: reference "question" is ambiguous; re-query by key:
  [4] <worktree>/.grove/05-research-a-question-k4.md
  [5] <worktree>/.grove/06-research-b-question-k5.md
  [6] <worktree>/.grove/07-combine-research-question-k6.md
exit: 0
```

#### resolve a retired leaf
```console
$ grove-llm resolve baseline
<worktree>/.grove/01-DONE-requirements-baseline-k1.md
--- stderr ---
note: referenced task is retired (DONE): <worktree>/.grove/01-DONE-requirements-baseline-k1.md
exit: 0
```

#### resolve an abandoned leaf
```console
$ grove-llm resolve third
<worktree>/.grove/04-ABANDONED-design-third-k3.md
--- stderr ---
note: referenced task is abandoned (ABANDONED): <worktree>/.grove/04-ABANDONED-design-third-k3.md
exit: 0
```

#### resolve not found
```console
$ grove-llm resolve nosuchthing
--- stderr ---
resolve: no entry matches reference "nosuchthing"
exit: 0
```

#### resolve a node (directory path)
```console
$ grove-llm resolve second-k2
<worktree>/.grove/03-second-k2
exit: 0
```

### F — legacy tree (no FORMAT witness)
#### pick
```console
$ grove-llm pick
--- stderr ---
Error: Grove tree format witness is missing at <worktree>/.grove/FORMAT; this is a legacy tree and must be migrated before current tree operations can run
exit: 1
```

#### kind
```console
$ grove-llm kind
--- stderr ---
Error: Grove tree format witness is missing at <worktree>/.grove/FORMAT; this is a legacy tree and must be migrated before current tree operations can run
exit: 1
```

#### leaf-add
```console
$ grove-llm leaf-add . x
--- stderr ---
Error: Grove tree format witness is missing at <worktree>/.grove/FORMAT; this is a legacy tree and must be migrated before current tree operations can run
exit: 1
```

#### root-init over a legacy tree
```console
$ grove-llm root-init x
--- stderr ---
Error: grove root already exists: <worktree>/.grove
exit: 1
```

### G — foreign / unsupported format
#### pick
```console
$ grove-llm pick
--- stderr ---
Error: unsupported Grove tree format: found "session-kinds-v2\n" in <worktree>/.grove/FORMAT; this binary requires "session-kinds-v1\n"
exit: 1
```

#### leaf-add
```console
$ grove-llm leaf-add . x
--- stderr ---
Error: unsupported Grove tree format: found "session-kinds-v2\n" in <worktree>/.grove/FORMAT; this binary requires "session-kinds-v1\n"
exit: 1
```

#### leaf-retire
```console
$ grove-llm leaf-retire 01-impl-t-k1.md
--- stderr ---
Error: unsupported Grove tree format: found "session-kinds-v2\n" in <worktree>/.grove/FORMAT; this binary requires "session-kinds-v1\n"
exit: 1
```

### H — reserved witness present: the *published* finish transaction (fail-closed)
#### pick
```console
$ grove-llm pick
--- stderr ---
Error: pending Grove finish transaction: "<worktree>/.grove/FINISHING-finish-k9". Recover it with the same finish-commit handle or rerun bare `grove`
exit: 1
```

#### leaf-add
```console
$ grove-llm leaf-add . x
--- stderr ---
Error: pending Grove finish transaction: "<worktree>/.grove/FINISHING-finish-k9". Recover it with the same finish-commit handle or rerun bare `grove`
exit: 1
```

### H2 — witness classification precedes format classification
#### pick (migration witness present AND no FORMAT)
```console
$ grove-llm pick
--- stderr ---
Error: pending Grove session-kind migration: "<worktree>/.grove/MIGRATING-session-kinds". To recover it, rerun bare `grove`
exit: 1
```

### H3 — the *preparing* finish witness (unpublished half of the same reservation)
A `PREPARING-FINISH-<finish-handle>-<attempt-identity>/` directory holds no
evacuated entry, so the tree beside it is still perfectly walkable. That is why
the refusal is by reserved prefix rather than by whether the tree looks intact.

#### pick
```console
$ grove-llm pick
--- stderr ---
Error: pending Grove finish transaction: "<worktree>/.grove/PREPARING-FINISH-finish-k2-11111111111111111111111111111111". Recover it with the same finish-commit handle or rerun bare `grove`
exit: 1
```

#### kind
```console
$ grove-llm kind
--- stderr ---
Error: pending Grove finish transaction: "<worktree>/.grove/PREPARING-FINISH-finish-k2-11111111111111111111111111111111". Recover it with the same finish-commit handle or rerun bare `grove`
exit: 1
```

#### resolve
```console
$ grove-llm resolve 1
--- stderr ---
Error: pending Grove finish transaction: "<worktree>/.grove/PREPARING-FINISH-finish-k2-11111111111111111111111111111111". Recover it with the same finish-commit handle or rerun bare `grove`
exit: 1
```

#### leaf-add
```console
$ grove-llm leaf-add . x
--- stderr ---
Error: pending Grove finish transaction: "<worktree>/.grove/PREPARING-FINISH-finish-k2-11111111111111111111111111111111". Recover it with the same finish-commit handle or rerun bare `grove`
exit: 1
```

### I — exhausted tree (rooted, every leaf terminal)
#### tree
```console
$ find .grove -type f
.grove/01-DONE-requirements-only-k1.md
.grove/FORMAT
.grove/BRIEF.md
exit: 0
```

#### pick
```console
$ grove-llm pick
--- stderr ---
grove spent: no live leaves; this grove is done
exit: 0
```

#### kind
```console
$ grove-llm kind
--- stderr ---
grove spent: no live leaves; this grove is done
exit: 0
```

#### brief-chain
```console
$ grove-llm brief-chain
--- stderr ---
grove spent: no live leaves; this grove is done
exit: 0
```


### J — malformed current tree (task-shaped leaf, no admissible kind)
The refusal `extract-task-tree-k24` will move. Both cases produce the same
diagnostic, which enumerates all nineteen kinds — so the diagnostic is also the
observable form of the closed kind set.

#### a task-shaped leaf with no kind segment
```console
$ grove-llm pick
--- stderr ---
Error: malformed Grove leaf "01-untyped-k1.md": expected NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md with session kind one of `requirements`, `design`, `planning`, `prototype`, `impl`, `research-a`, `research-b`, `combine-research`, `finish`, `review-requirements`, `review-design`, `review-planning`, `review-prototype`, `review-impl`, `integrate-review-requirements`, `integrate-review-design`, `integrate-review-planning`, `integrate-review-prototype`, `integrate-review-impl` (<worktree>/.grove/01-untyped-k1.md)
exit: 1
```

#### a task-shaped leaf naming an unknown kind
```console
$ grove-llm pick
--- stderr ---
Error: malformed Grove leaf "01-mystery-untyped-k1.md": expected NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md with session kind one of `requirements`, `design`, `planning`, `prototype`, `impl`, `research-a`, `research-b`, `combine-research`, `finish`, `review-requirements`, `review-design`, `review-planning`, `review-prototype`, `review-impl`, `integrate-review-requirements`, `integrate-review-design`, `integrate-review-planning`, `integrate-review-prototype`, `integrate-review-impl` (<worktree>/.grove/01-mystery-untyped-k1.md)
exit: 1
```

### K — the workspace-layout boundary, driven by bare `grove`
The seam `extract-workspace-k25` will move. The refused case is staged with the
one operand this host cannot supply for real — a second filesystem — through the
shipped `GROVE_TEST_FOREIGN_FILESYSTEM` seam, which makes every measurement at or
under the named directory report a distinct device (`src/repo.rs:212`). The
resolution, ordering, diagnostic and no-mutation guarantee are otherwise the real
ones.

#### refused: a cross-device linked Git worktree, with an empty `$HOME`
`$HOME` holds **no** configuration, so a driver that reached configuration
validation would fail with `Grove configuration is missing` (§8 L). It does not;
the layout refusal lands first, and nothing is created.

```console
$ grove
--- stderr ---
Error: unsupported workspace layout: Grove's teardown moves the whole task tree into the workspace-control directory in one atomic rename, which cannot cross a filesystem boundary, so this workspace could never finish
  working tree root:           <worktree>/linked (filesystem 16777232)
  workspace-control directory: <worktree>/main/.git/worktrees/linked/grove (filesystem 16777233)
  resolved from:               the `.git` file <worktree>/linked/.git, naming gitdir <worktree>/main/.git/worktrees/linked

Either place this working tree on the same filesystem as the repository its administration directory lives in, or drive Grove from a workspace whose administration directory is inside the working tree. Nothing was created or changed; repair the layout and rerun.
exit: 1
```

#### admitted: a plain Git checkout, one live `impl` leaf, stub session command
The stub prints its first argument and exits 0 without signalling, which is the
loop's clean stop. Absolute paths to the stub and to `$HOME` are rewritten.

```console
$ grove
grove: provisioned the codex skill at <home>/.codex/skills/grove
grove: launching impl with configured "<fixture>/stub.sh" — subject-k1
stub session ran: kind argument begins <**Load the `grove` skill now, and read its `refe>
grove: session ended without a completion signal — status exit status: 0, elapsed 0.501s; loop stopped.
exit: 0
```

### L — configuration diagnostics
Reachable in an isolated fixture: an isolated `$HOME` and an admitted layout are
the whole rig, and no session is launched in any of the three cases. `.grove/` is
absent throughout, so validation is reached before any tree work.

#### missing configuration
```console
$ grove
grove: provisioned the codex skill at <home>/.codex/skills/grove
--- stderr ---
Error: Grove configuration is missing at <home>/.config/grove/config.kdl; required session kinds: requirements, review-requirements, integrate-review-requirements, design, review-design, integrate-review-design, planning, review-planning, integrate-review-planning, prototype, review-prototype, integrate-review-prototype, impl, review-impl, integrate-review-impl, research-a, research-b, combine-research, finish
exit: 1
```

#### incomplete configuration (one kind declared of nineteen)
```console
$ grove
--- stderr ---
Error: invalid Grove configuration at <home>/.config/grove/config.kdl:
  - missing session kinds: requirements, review-requirements, integrate-review-requirements, design, review-design, integrate-review-design, planning, review-planning, integrate-review-planning, prototype, review-prototype, integrate-review-prototype, review-impl, integrate-review-impl, research-a, research-b, combine-research, finish
exit: 1
```

#### a template with no `${prompt}` (per-kind, with file position)
All nineteen are reported; two lines are shown and the remaining seventeen are
identical but for the kind and the line number.

```console
$ grove
--- stderr ---
Error: invalid Grove configuration at <home>/.config/grove/config.kdl:
  - <home>/.config/grove/config.kdl:1:1: session kind `requirements`: command template must contain `${prompt}` exactly once
  - <home>/.config/grove/config.kdl:2:1: session kind `review-requirements`: command template must contain `${prompt}` exactly once
exit: 1
```

### M — `finish-commit`, in all three supported layouts
Reproducible in an isolated fixture, contrary to this ledger's first pass. The
rig is an exhausted tree (one `DONE` leaf, committed), an isolated `$HOME`, and a
`finish` template pointing at a stub that reads the handle from the tree, calls
`grove-llm finish-commit`, and then signals. Every run below is a bare `grove`
drive from a clean fixture.

#### refusal: no live finish leaf, outside a driver session
```console
$ grove-llm finish-commit finish-k2
--- stderr ---
Error: the requested finish leaf is no longer live
exit: 1
```

#### success: plain Git
`finish-commit` is **silent** on success — the observable result is the exit
status and the commit. After it, bare `grove` finds a rootless tree and scaffolds
a fresh grove, which is why a `requirements` launch follows the finish.

```console
$ grove
grove: launching finish with configured "<fixture>/finish.sh" — finish-k2
stub finish session: handle <finish-k2>
finish-commit exit: 0
grove complete: signalled; the loop will start the next task.
grove: launching requirements with configured "<fixture>/stub.sh" — plan-k1
exit: 0
```
```console
$ git log --oneline -2
a1635cc finish-k2 (finish attempt a35acb50aea05e61bb82a41dd60a6c04): remove completed grove task tree
f208217 seed grove
exit: 0
```
```console
$ git show --stat HEAD
 .grove/01-DONE-impl-subject-k1.md | 1 -
 .grove/BRIEF.md                   | 1 -
 .grove/FORMAT                     | 1 -
 3 files changed, 3 deletions(-)
exit: 0
```

#### success: colocated jj
```console
$ jj log -r 'ancestors(@,3)'
@  xvpkovlr 2026-08-24 18:40:38 dffdb75a
│  (no description set)
○  wvmotzoq 2026-08-24 18:40:37 1e17e8a9
│  finish-k2 (finish attempt 62fe04dd0faf3b891ae390ca8e8237ea): remove completed grove task tree
○  sruwoysx 2026-08-24 18:40:28 005bca91
│  seed grove
◆  zzzzzzzz root() 00000000
exit: 0
```

#### success: native jj (no `.git` in the working tree)
```console
$ jj log -r 'ancestors(@,3)'
@  xxunryml 2026-08-24 18:40:48 9eaa571d
│  (no description set)
○  vwysruzz 2026-08-24 18:40:47 5b4e9dde
│  finish-k2 (finish attempt 1e20ea5b6e54248c0645dce288ea7357): remove completed grove task tree
○  mosrktxp 2026-08-24 18:40:47 76e87fe1
│  seed grove
◆  zzzzzzzz root() 00000000
exit: 0
```

The abstract outcome is identical across the three: one revision whose only
change is the deletion of `.grove/`, described `<handle> (finish attempt
<attempt-identity>): remove completed grove task tree`. The attempt identity
differs per run by construction; the commit subject shape does not.

---

## 9. Repository shape at baseline

The numbers the refactor moves. Recorded so "the crates are deeper now" can be
checked rather than asserted.

| | files | lines |
|---|---|---|
| `src/` (root package) | 36 | 31,423 |
| `tests/` (root package) | 42 | 27,996 |
| `crates/ordinal-fs-tree/{src,bin}` | 19 | 9,758 |
| `crates/ordinal-fs-tree/tests/` | 10 | 4,395 |
| `content/` (the embedded methodology) | 29 | 1,740 |
| `docs/**/*.md` | 28 | 11,232 |
| `docs/ordinal-fs-tree/models/structure.als` | 1 | 571 |
| `docs/ordinal-fs-tree/models/operations.qnt` | 1 | 1,349 |

### The ten largest units in `src/`

The concentration the target dependency direction is meant to break up.

| lines | file |
|---|---|
| 3,632 | `src/finish_transaction.rs` |
| 2,953 | `src/repo/finish_commit.rs` |
| 2,853 | `src/tree_lifecycle.rs` |
| 1,979 | `src/task_tree.rs` |
| 1,737 | `src/driver_lease.rs` |
| 1,634 | `src/finish_cleanup/auxiliary/tests.rs` |
| 1,534 | `src/tree_migrate.rs` |
| 1,497 | `src/tree_migration_transaction.rs` |
| 1,398 | `src/task_grow/tests.rs` |
| 1,257 | `src/finish_cleanup/auxiliary.rs` |

The finish/recovery concern alone is **10,366 production lines — 34% of `src/`** —
plus 6,701 lines of test. `docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`
— retired at `delete-finish-transaction-k8` with the layer it kept — carried that
breakdown and the four questions — two settled `keep`, two `defer`
pending a control neither model family runs yet; this row is here so the count is
anchored to a revision.

`src/tree_migrate.rs` (1,534) and `src/tree_migration_transaction.rs` (1,497) —
3,031 lines — were the approved deletion, and **the approval was withdrawn with
the phase that owned it**: both files stand. Nothing in this table is scheduled
to shrink by removal.

## 10. Where each preserved claim is checked today

Not every promise in the ledger above has a test behind it. Recorded so
the refactor knows which ones a green suite actually defends.

| preserved claim | checked by |
|---|---|
| CLI verb **names** — a four-verb sample, and six retired verbs asserted absent | `tests/llm_cli.rs`, `tests/removed_surface.rs` |
| every listed argument and subcommand **has a description** | `tests/help_surfaces.rs` |
| the corpus instructs no verb the CLI lacks; the verb surface is flat | `tests/methodology.rs` — `INSTRUCTED_VERBS` at `:1080` pins **eleven** verb *names*, the ones the embedded methodology actually instructs, against clap's own model of the command tree |
| **the full verb set, its arguments and their arity** | **nothing** — see §6. `--help` is generated from the same source it would be checked against. Note what *is* held: `tests/methodology.rs:1080` pins eleven instructed verb names, so this row is not "no verb name is checked" — it is that the remaining two of the **thirteen** verbs (`root-init`, `kind`), and every verb's options and arity, are unpinned |
| success vs failure | the per-verb suites (`pick.rs`, `resolve.rs`, `kind.rs`, `leaf_ops.rs`, `root_init.rs`, …), via `status.success()` |
| **the 1 vs 2 distinction** | **nothing** — no `tests/` assertion reads an exact code |
| the `session-kinds-v1` format and its refusals | `tests/session_kind_tree.rs`, `tests/migration_transition.rs` |
| reserved-witness refusal — **all three** classes (`MIGRATING-session-kinds`, `PREPARING-FINISH-`, `FINISHING-`) | `tests/session_kind_tree.rs:495` sweeps every tree verb against `MIGRATING-`; `tests/finish_lifecycle.rs:1424` and `:1461` hold ordinary-reader refusal of the published and the *preparing* finish witness, and `:1293` holds `finish-commit`'s own refusal of a witness collision. **Not** `tests/migration_transition.rs` (legacy-transition and unknown-format refusal) or `tests/migration_commit.rs` (migration commit and recovery mechanics) — neither asserts witness refusal, and both were cited here on first pass from their filenames |
| methodology rules stay reachable on each kind's loaded path | `tests/lifecycle_invariants.rs` — delivery only; a green run is not evidence a session obeyed (`docs/adr/behavioural-coverage-asserts-delivery.md`) |
| abstract outcomes across Git / native jj / colocated jj | `tests/jj_tree_verbs.rs`, `tests/workspace_layout.rs`, `tests/finish_lifecycle.rs`, `tests/repo.rs` |
| methodology embedding and provisioning | `tests/provision.rs`, `tests/methodology.rs`; and at release time by the `CONTENT_MARKER` grep on both staged binaries |
| the imposed dependency set is exactly `libc` | `tests/library_dependency.rs` |
| the **enumerated scrub lists** for spawned children | unit tests in `src/launch.rs`: `:133` for a configured session (loop control **and** the six internal test seams removed, repository context deliberately *retained*), `:172` for an internal child (loop control, repository context, and a foreign `GIT_INDEX_FILE` all removed) |
| launch **inheritance**, black-box | `tests/lifecycle_cutover.rs:190` — drives bare `grove` with stale `GROVE_SIGNAL_FILE`, `GROVE_HARNESS_PID` and `GROVE_CLAUDE_PID` in the environment and asserts the child sees the driver's own signal path, no legacy handles, and an unrelated ambient variable preserved; `tests/repo.rs` and `tests/finish_lifecycle.rs` exercise the internal-child side |
| the **test runner's own** signal channel cannot reach a live loop | `tests/env_hygiene.rs` — the `.cargo/config.toml` force-override and the shared scrub list, each asserted independently sufficient. It does **not** assert the spawned-child scrub set, which is what its name suggests and what it was cited for on first pass |
| **repository discovery** ignores ambient `GIT_DIR` / `GIT_WORK_TREE` / `GIT_COMMON_DIR` / `TMPDIR` | `tests/repo_environment.rs` — one test over `workspace_control` and `main_repo_of`. It says nothing about what a spawned child inherits |
| configuration grammar, the nineteen kinds, delta precedence | `tests/session_config.rs` |
| the advisory tree lock serialising cooperating processes | `tests/tree_lock.rs` — renamed with the module at `collapse-tree-access-k13`, which deleted Grove's own lock layer and left the store's the only one |
| **MSRV 1.85** | **nothing** — no CI, and an ordinary build never checks its own `rust-version`. Established by running `rustup run 1.85` / `1.84` by hand. |
| **release targets and the glibc 2.17 floor** | **nothing in the test suite** — asserted only by `scripts/release-doctor.sh` at release time |
| **package and binary names, install behaviour** | **partially** — both binaries are exercised throughout the suite, and the release gate greps each staged binary for the embed marker |
| **Homebrew formula rendering** | **nothing** — checked only by a real release cut |

**Five rows read *nothing*, and they are the ledger's weak points**: the full
verb surface, the 1-vs-2 exit split, MSRV, the release targets and glibc floor,
and the Homebrew formula rendering. A refactor can break any of them with a
green `cargo test`. (On first pass formula rendering was folded into the
*partially* row above while the prose still counted it among the five, so the
table and the sentence disagreed; it is its own row now.)

Two of the five — the verb surface and the exit split — are recorded as values
in §6 above precisely because nothing else holds them, and the format refusals'
exact wording is captured in §5 and §8 for the same reason, though that one is
not a *nothing* row: `tests/session_kind_tree.rs` holds the refusals themselves,
just not their strings.

## 11. What this ledger does not cover

Stated so a later reader does not mistake absence for a measured negative. The
first pass of this section listed four omissions and treated all four as out of
reach. Two of them — configuration diagnostics and `finish-commit` — were merely
inconvenient and are now captured in §8; a third, performance, was never in the
preservation contract at all and is reclassified below as an intentional
non-goal rather than a gap; only Linux execution was genuinely unreachable. A
fifth limitation the first pass omitted entirely, the release cut, is added.

What remains is sorted into **three** kinds, because they license different
things: a *measured transcript* can be diffed, a *read fact* can only be
re-read, and an *unmeasured* behaviour can be neither and must not be quoted as
a baseline.

### Measured here — captured in §8, diffable by a later phase

Listed because their earlier absence was recorded as a limitation and no longer
is. A `documentation-k2` or `implementation-k3` leaf that changes any of these
seams re-runs the capture and diffs.

| seam | §8 | the leaf that will move it |
|---|---|---|
| the *preparing* finish witness's refusal | H3 | `extract-finish-baseline-k26`, `extract-task-tree-k24` |
| malformed-current-tree refusal, and with it the observable closed kind set | J | `extract-task-tree-k24` |
| the workspace-layout boundary — one admitted layout driven, one cross-device refusal | K | `extract-workspace-k25` |
| configuration diagnostics — missing, incomplete, and a template failing substitution | L | `extract-workspace-k25`, and the application/runtime split |
| `finish-commit` success in plain Git, colocated jj and native jj, plus one refusal | M | `extract-finish-baseline-k26` |

### Read from source or tests, never executed here

True as far as the named file goes, and **not** a transcript. A change here
cannot be caught by re-running anything in §8; it has to be caught by re-reading
the same file.

- **§7's configuration grammar and precedence** — read from
  `docs/CONFIGURATION.md` and `src/session_config.rs`, held by
  `tests/session_config.rs`. §8 L now captures the *diagnostics*, which is a
  different claim from the grammar.
- **§7's spawned-child scrub sets, and the measured gap beside them** —
  `src/launch.rs`'s three lists, and the two `src/finish_transaction.rs` seams
  that are *not* in them. The gap is a read fact about shipped code, recorded so
  a later leaf does not preserve it by copying the six-member list as complete.
- **§10's whole mapping** — which test holds which claim. Every row was checked
  against what the named file asserts rather than what it is called, twice; that
  is a reading, and a row can rot without any capture changing.
- **MSRV 1.85** — established by running `rustup run 1.85` / `1.84` by hand at
  §3, not by anything in the build or the suite.
- **The release targets and the glibc 2.17 floor** — read from
  `scripts/release-common.sh` and `scripts/release-build.sh`.

### Genuinely unmeasured

- **Linux behaviour.** Everything here is `aarch64-apple-darwin`. The two Linux
  targets are cross-compiled and cannot execute on this host — which is why the
  release script asserts the embed marker on the staged artifacts rather than by
  running them. **Host-unreachable**, and it may stay that way; a later phase
  that needs it needs a Linux runner, not a better fixture.
- **The release cut: archive layout, Homebrew formula rendering, and install
  behaviour.** §10 records that formula rendering is checked only by a real cut,
  and a real cut tags and publishes — so it is not stageable from a throwaway
  fixture. **Host-unreachable here**, and it is the fifth *nothing* row in §10;
  the two statements are the same fact seen from either side, and the first pass
  of this section omitted it.
- **Performance.** No baseline beyond the suite wall-clock figures in §2, and
  none is wanted: performance is **not in the preservation contract**, so this is
  an intentional non-goal rather than a measurement that failed. A later phase
  must not read its absence as permission to regress, nor as an obligation to
  establish one.

### One naming note

**"Preservation baseline" is not a glossary term.** It names an artifact of this
workstream, not a concept in grove's domain, and `CONTEXT-FORMAT.md` keeps
process vocabulary out of `CONTEXT.md`. This file's own header is its definition.

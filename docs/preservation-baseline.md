# Preservation baseline — grove v19.3.0

The observable contract of this repository **before** the modularity refactor,
measured rather than described. The root brief's *Preservation ledger* says what
must survive; this file is that ledger with the values filled in, so a later
phase can check a change against a recorded fact instead of against an
impression.

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

`content/` — 29 files, 1740 lines — is compiled into **both** binaries with
`include_dir!`. `grove` extracts it verbatim into every installed harness's
personal skill directory; `grove-llm` hashes it to name which build it is.

The **methodology identity** is a SHA-256 over the embedded file payload:
files sorted by path, each contributing a little-endian `u64` length prefix and
bytes for its `content/`-relative path, then the same for its contents. Embedded
directories are excluded, so an empty directory is not part of a build's identity
(`docs/adr/one-build-owns-a-session.md`). It is computed from the linked embed,
not from a constant recorded beside it — which is what makes comparing it to a
provisioned directory worth doing.

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

**A reserved witness outranks all three.** A `MIGRATING-session-kinds/` or
`FINISHING-<handle>/` directory under `.grove/` makes every ordinary reader and
mutator refuse, and that check runs *before* format classification. Captured in
§8 H2: a tree with a migration witness **and** no `FORMAT` reports the pending
migration, not the legacy format. This ordering is what keeps an evacuated finish
tree from reading as a malformed grove
(`docs/adr/task-tree-transactions-fail-closed.md`).

**Note for `remove-migration-k22`.** The legacy diagnostic instructs the operator
to migrate. The approved breaking change deletes migration, so that string will
point at a capability the binary no longer has; the brief requires it be replaced
with how to start a current-format root. The `MIGRATING-session-kinds/` witness
class, its refusal, and the driver-only transition in `src/tree_lifecycle.rs` are
in the same scope. There is **no user-facing migrate verb** today — migration
happens inside bare `grove`'s driver path only.

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
session spawn scrubs too, then sets the one path it owns. `tests/env_hygiene.rs`
holds this.

`grove-llm complete --signal-file` defaults to `$GROVE_SIGNAL_FILE`; with that
absent the verb is a safe near-no-op that tells the operator to exit manually.

**Supported workspace layouts**: plain Git, native jj, and colocated jj — the
three shapes the finish transaction must behave symmetrically across
(`docs/adr/supported-workspace-layouts.md`). The tree these measurements were
taken in is a **jj-native secondary workspace** (`refactor-for-modularity`, one of
three in this repository), which is the layout with no `.git` of its own.

## 8. Captured transcripts

Every block below is a real run against a throwaway fixture, reproducible by
`git init`-ing an empty repository and repeating the commands in order. Absolute
paths are rewritten to `<worktree>`; nothing else is edited.

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

### H — reserved witness present (fail-closed)
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
plus 6,701 lines of test. `TODO.finish_process.md` carries that breakdown and the
four questions it raises; this row is here so the count is anchored to a revision.

`src/tree_migrate.rs` (1,534) and `src/tree_migration_transaction.rs` (1,497) —
3,031 lines — are the approved deletion. Nothing else in this table is scheduled
to shrink by removal rather than by moving.

## 10. Where each preserved claim is checked today

Not every promise in the root brief's ledger has a test behind it. Recorded so
the refactor knows which ones a green suite actually defends.

| preserved claim | checked by |
|---|---|
| CLI verb **names** — a four-verb sample, and six retired verbs asserted absent | `tests/llm_cli.rs`, `tests/removed_surface.rs` |
| every listed argument and subcommand **has a description** | `tests/help_surfaces.rs` |
| the corpus instructs no verb the CLI lacks; the verb surface is flat | `tests/methodology.rs` |
| **the full verb set, its arguments and their arity** | **nothing** — see §6. `--help` is generated from the same source it would be checked against |
| success vs failure | the per-verb suites (`pick.rs`, `resolve.rs`, `kind.rs`, `leaf_ops.rs`, `root_init.rs`, …), via `status.success()` |
| **the 1 vs 2 distinction** | **nothing** — no `tests/` assertion reads an exact code |
| the `session-kinds-v1` format and its refusals | `tests/session_kind_tree.rs`, `tests/migration_transition.rs` |
| reserved-witness refusal (`FINISHING-`, `MIGRATING-`) | `tests/session_kind_tree.rs`, `tests/migration_transition.rs`, `tests/finish_lifecycle.rs`, `tests/migration_commit.rs` |
| methodology rules stay reachable on each kind's loaded path | `tests/lifecycle_invariants.rs` — delivery only; a green run is not evidence a session obeyed (`docs/adr/behavioural-coverage-asserts-delivery.md`) |
| abstract outcomes across Git / native jj / colocated jj | `tests/jj_tree_verbs.rs`, `tests/workspace_layout.rs`, `tests/finish_lifecycle.rs`, `tests/repo.rs` |
| methodology embedding and provisioning | `tests/provision.rs`, `tests/methodology.rs`; and at release time by the `CONTENT_MARKER` grep on both staged binaries |
| the imposed dependency set is exactly `libc` | `tests/library_dependency.rs` |
| environment hygiene around spawned children | `tests/env_hygiene.rs`, `tests/repo_environment.rs` |
| configuration grammar, the nineteen kinds, delta precedence | `tests/session_config.rs` |
| the advisory tree lock serialising cooperating processes | `tests/tree_access.rs` |
| **MSRV 1.85** | **nothing** — no CI, and an ordinary build never checks its own `rust-version`. Established by running `rustup run 1.85` / `1.84` by hand. |
| **release targets and the glibc 2.17 floor** | **nothing in the test suite** — asserted only by `scripts/release-doctor.sh` at release time |
| **package and binary names, install behaviour** | **partially** — the binaries are exercised throughout, but the Homebrew formula rendering is checked only by a real cut |

**Five rows read *nothing*, and they are the ledger's weak points**: the full
verb surface, the 1-vs-2 exit split, MSRV, the release targets and glibc floor,
and the formula rendering. A refactor can break any of them with a green
`cargo test`. Three of the five — the verb surface, the exit split, and the
format refusals' exact wording — are recorded in §6, §8 and §5 above precisely
because nothing else holds them.

## 11. What this ledger does not cover

Stated so a later reader does not mistake absence for a measured negative.

- **Configuration diagnostics were not captured.** Config validation lives inside
  bare `grove`, which provisions, takes the working-tree driver lease and starts
  launching sessions; running it here would have contended with the live driver
  that launched this session. The grammar and precedence rules in §7 are read from
  `docs/CONFIGURATION.md` and `src/session_config.rs`, and are held by
  `tests/session_config.rs`. Capturing them needs a rig that can drive
  `grove` with a stub session command — which `tests/lifecycle_cutover.rs`
  already has, and which is the place to add it if a later leaf needs the
  transcripts.
- **`finish-commit` was not run.** It commits, and a captured run needs the three
  VCS layouts and a live driver-owned finish leaf. `tests/finish_lifecycle.rs`
  (4,144 lines) is the existing coverage; the finish/recovery scopes of the formal
  phase are where its behaviour gets stated precisely.
- **Linux behaviour was not measured.** Everything here is `aarch64-apple-darwin`.
  The two Linux targets are cross-compiled and cannot execute on this host, which
  is why the release script asserts the embed marker on the staged artifacts
  rather than by running them.
- **No performance baseline** beyond the suite wall-clock figures in §2.
- **"Preservation baseline" is not a glossary term.** It names an artifact of this
  workstream, not a concept in grove's domain, and `CONTEXT-FORMAT.md` keeps
  process vocabulary out of `CONTEXT.md`. This file's own header is its
  definition.

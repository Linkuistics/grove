# Codex/Pi Harness Switch Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make grove drive a month-long codex+gpt-5.6-sol vs pi+Kimi-K3 trial: a `pi` harness entry, codex profile launches, per-kind harness routing (`GROVE_REVIEW_HARNESS=pi`), per-harness model envs, harness-neutral PID handle, and multi-harness skill provisioning — plus the user-config, skills-repo, and release/migration work around it.

**Architecture:** grove already dispatches launches through a static `Harness` table (`src/harness.rs`) consumed by a stateless loop driver (`src/loop_driver.rs`). This plan extends the table (pi; codex `--profile`), adds two env-keyed routing seams resolved per picked leaf (kind→harness override, then harness-scoped model lookup), renames the PID handle to be harness-neutral, and turns single-target skill provisioning (`~/.claude/skills/grove`) into a sweep over every installed harness's skills dir.

**Tech Stack:** Rust (anyhow, clap, include_dir, tempfile; cargo test with fake-binary shell scripts via env seams), zsh env config, codex config.toml profiles, pi extensions (`pi install`).

**Spec:** `docs/superpowers/specs/2026-07-18-codex-pi-harness-switch-design.md` (approved). Read it for rationale; this plan is the how.

## Global Constraints

- Env naming scheme (exact): `GROVE_<KIND>_HARNESS` (kind ∈ PLANNING, RESEARCH, PROTOTYPE, WORK, REVIEW) selects the harness for that kind; `GROVE_<HARNESS>_<KIND>_MODEL` (harness ∈ CLAUDE, CODEX, PI) beats `GROVE_<KIND>_MODEL`.
- `GROVE_HARNESS_PID` replaces `GROVE_CLAUDE_PID`; the old name is still **exported** by the wrapper and **read** as a fallback for exactly one release.
- pi harness row (verified against installed pi CLI 2026-07-18): `exec_bin: "pi"`, no session-name flag (`name_args: &[]`), `model_args: &["--model"]` (`--model` supports `provider/id`), headless mode is `--print`/`-p`, positional prompt works.
- codex `model_args` becomes `&["--profile"]` — env values name codex **profiles**, not models. Breaking; CHANGELOG-documented.
- Provisioning may replace a `grove` skills entry ONLY when it is a symlink or a grove-provisioned dir (has `.grove-content-hash` stamp, or is absent/empty); anything else → bail with a message. Symlinks must be detected with `symlink_metadata` and removed as **files** BEFORE any `remove_dir_all` (deleting through the symlink would destroy the target's contents).
- Unknown harness names (in `--harness`, stamps, or `GROVE_<KIND>_HARNESS`) → hard error naming the input and listing known harnesses; the known-list is derived from `HARNESSES`, never hardcoded.
- Loop semantics unchanged: fresh session per task, signal-gated relaunch, non-signal exit stops (restart ≡ continuation). No new daemon/state.
- Tests touching env vars or `$HOME` must hold the file's `ENV_LOCK` mutex (see `tests/loop_driver.rs:19`); new test files that set process env need their own `static ENV_LOCK: Mutex<()>`.
- All commits on a feature branch `codex-pi-harness` off `main`; commit messages in repo style (`feat:`/`fix:`/`docs:` + why-focused body).

---

### Task 0: Branch

**Files:** none (git only)

- [x] **Step 1: Create the working branch**

```bash
cd /Users/antony/Development/grove
git checkout -b codex-pi-harness main
```

Expected: `Switched to a new branch 'codex-pi-harness'`.

---

### Task 1: Harness registry — pi entry, codex `--profile`, `skills_dir`, derived known-names

**Files:**
- Modify: `src/harness.rs`
- Modify: `tests/harness.rs`
- Modify: `tests/loop_driver.rs:494-589` (the codex argv test — flag flips to `--profile`)

**Interfaces:**
- Consumes: nothing new.
- Produces (later tasks rely on these exact items):
  - `Harness.skills_dir: &'static str` — home-relative global skills dir (`".claude/skills"`, `".codex/skills"`, `".pi/agent/skills"`). Task 6 joins `$HOME/<skills_dir>/grove`.
  - `pub fn known_names() -> String` — `"claude, codex, pi"`. Tasks 2 and 5 use it in error messages.
  - `by_name("pi")` returns the pi row.

- [x] **Step 1: Write the failing tests**

In `tests/harness.rs`, replace `registry_contains_claude_and_codex` and add two tests:

```rust
#[test]
fn registry_contains_claude_codex_and_pi() {
    assert!(by_name("claude").is_some());
    assert!(by_name("codex").is_some());
    assert!(by_name("pi").is_some());
    assert!(by_name("nonsense").is_none());
}

#[test]
fn harness_rows_carry_the_launch_and_skills_contract() {
    let claude = by_name("claude").unwrap();
    assert_eq!(claude.skills_dir, ".claude/skills");
    assert_eq!(claude.model_args, &["--model"]);

    // codex: profiles bind model + reasoning effort, so the model-per-task-kind
    // value names a *profile*.
    let codex = by_name("codex").unwrap();
    assert_eq!(codex.skills_dir, ".codex/skills");
    assert_eq!(codex.model_args, &["--profile"]);
    assert!(codex.name_args.is_empty());

    // pi (verified against pi --help): --model, no name flag, skills under
    // ~/.pi/agent/skills (structurally unlike the other two — hence a field).
    let pi = by_name("pi").unwrap();
    assert_eq!(pi.project_dir, ".pi");
    assert_eq!(pi.exec_bin, "pi");
    assert_eq!(pi.skills_dir, ".pi/agent/skills");
    assert_eq!(pi.model_args, &["--model"]);
    assert!(pi.name_args.is_empty());
}

#[test]
fn known_names_lists_every_registry_row() {
    assert_eq!(grove::harness::known_names(), "claude, codex, pi");
}
```

Add `detect` coverage for pi in the existing style:

```rust
#[test]
fn detect_finds_pi_when_dot_pi_present() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir_all(tmp.path().join(".pi")).unwrap();

    let detected = detect_in_repo(tmp.path());
    assert_eq!(detected.len(), 1);
    assert_eq!(detected[0].name, "pi");
}
```

- [x] **Step 2: Run to verify failure**

Run: `cargo test --test harness`
Expected: FAIL — `skills_dir` field does not exist, `known_names` not found, `by_name("pi")` is `None`.

- [x] **Step 3: Implement in `src/harness.rs`**

Add the field to the struct (after `project_dir`):

```rust
    /// Home-relative path of this harness's **global skills dir** — the
    /// provisioning target for the embedded methodology. A field (not derived
    /// from `project_dir`) because pi nests its skills under `agent/`.
    pub skills_dir: &'static str,
```

Set `skills_dir: ".claude/skills"` on the claude row and `".codex/skills"` on codex. On the codex row also change:

```rust
        // codex model-per-task-kind values name **profiles** (`--profile`), not
        // models: a profile binds model + reasoning effort together
        // (e.g. sol-xhigh / sol-high), which bare `-m` cannot express.
        model_args: &["--profile"],
```

Append the pi row after codex:

```rust
    Harness {
        name: "pi",
        // Opt-in detection marker; pi does not create repo-local `.pi/` dirs,
        // so explicit `--harness pi` + the stamp is the normal binding route.
        project_dir: ".pi",
        skills_dir: ".pi/agent/skills",
        exec_bin: "pi",
        // pi has no launch-time session-name flag (pi --help, checked
        // 2026-07-18); empty ⇒ the launch paths skip pre-naming.
        name_args: &[],
        // pi accepts `--model <pattern>` incl. "provider/id" ids, so it takes
        // part in model-per-task-kind on the same terms as claude.
        model_args: &["--model"],
    },
```

Add the derived known-list and use it in `select`:

```rust
/// The registry's names, comma-joined — the single source for "known:" error
/// text, so adding a harness never leaves a stale hardcoded list behind.
pub fn known_names() -> String {
    HARNESSES
        .iter()
        .map(|h| h.name)
        .collect::<Vec<_>>()
        .join(", ")
}
```

and in `select` replace the lookup error:

```rust
            let h = by_name(name).ok_or_else(|| {
                anyhow!("unknown harness: {name}. Known: {}", known_names())
            })?;
```

- [x] **Step 4: Update the codex argv integration test**

In `tests/loop_driver.rs`, test `codex_launches_with_no_name_flag_and_a_model_flag`: change the env value and both assertions —

```rust
    std::env::set_var("GROVE_WORK_MODEL", "sol-high");
```

and

```rust
    // ...and the profile *is* selected: codex model-per-task-kind values name
    // profiles (`--profile`), which bind model + reasoning effort.
    assert!(
        rows[1].contains("--profile sol-high"),
        "codex must honour model-per-task-kind via --profile (argv: {:?})",
        rows[1]
    );
```

(also update `.env("GROVE_WORK_MODEL", ...)`/`remove_var` value sites and the test's header comment: the second defect bullet now reads "codex participates via `--profile`, since profiles are the only way to bind reasoning effort to the launch").

- [x] **Step 5: Run the full suite**

Run: `cargo test`
Expected: PASS (all tests, including the untouched stamp/loop tests).

- [x] **Step 6: Commit**

```bash
git add src/harness.rs tests/harness.rs tests/loop_driver.rs
git commit -m "feat: add the pi harness and switch codex to profile launches

pi: --model/no-name-flag/skills under ~/.pi/agent/skills (verified against
pi --help). codex: model-per-task-kind values now name profiles, the only
launch-time binding of model + reasoning effort. known_names() derives the
error list from the registry so it can never go stale."
```

---

### Task 2: Persist explicit `--harness` choices in the stamp

**Files:**
- Modify: `src/harness_stamp.rs:46-59` (`maybe_stamp`)
- Modify: `src/launch.rs:31` (the one caller)
- Create: `tests/harness_stamp.rs`

**Interfaces:**
- Consumes: `harness::by_name`, `harness::known_names` (Task 1).
- Produces: `pub fn maybe_stamp(repo: &Path, name: &str, chosen: &'static Harness, explicit: bool) -> Result<()>` — Task 13's migration relies on `grove do --harness <h>` sticking.

- [ ] **Step 1: Write the failing tests**

Create `tests/harness_stamp.rs`:

```rust
// The stamp is how a grove *stays* bound to a harness. Bug being fixed: an
// explicit `--harness` in a single-harness repo wrote no stamp, so the next
// plain `grove do` silently fell back to the detected harness — the exact
// migration hazard for repos that carry a stray `.claude/` after the switch.

use grove::harness::by_name;
use grove::harness_stamp::{maybe_stamp, path, resolve_for_launch};
use std::fs;
use tempfile::TempDir;

#[test]
fn explicit_choice_is_stamped_even_in_a_single_harness_repo() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();

    let pi = by_name("pi").unwrap();
    maybe_stamp(repo.path(), "g", pi, true).unwrap();

    assert_eq!(
        fs::read_to_string(path(repo.path(), "g")).unwrap().trim(),
        "pi",
        "an explicit --harness must persist, or the next plain `grove do` \
         silently reverts to the detected harness"
    );
    // ...and the next launch resolves it from the stamp, not detection.
    let resolved = resolve_for_launch(repo.path(), "g", None).unwrap();
    assert_eq!(resolved.name, "pi");
}

#[test]
fn detected_choice_in_a_single_harness_repo_still_writes_no_stamp() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();

    let claude = by_name("claude").unwrap();
    maybe_stamp(repo.path(), "g", claude, false).unwrap();

    assert!(
        !path(repo.path(), "g").exists(),
        "auto-detected single-harness choice needs no disambiguation stamp"
    );
}

#[test]
fn multi_harness_repo_still_stamps_without_explicit() {
    let repo = TempDir::new().unwrap();
    fs::create_dir_all(repo.path().join(".claude")).unwrap();
    fs::create_dir_all(repo.path().join(".codex")).unwrap();

    let codex = by_name("codex").unwrap();
    maybe_stamp(repo.path(), "g", codex, false).unwrap();

    assert_eq!(
        fs::read_to_string(path(repo.path(), "g")).unwrap().trim(),
        "codex"
    );
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test --test harness_stamp`
Expected: FAIL to compile — `maybe_stamp` takes 3 arguments.

- [ ] **Step 3: Implement**

In `src/harness_stamp.rs` replace `maybe_stamp`:

```rust
/// Write `<repo>/.grove-stamps/<name>`. Two triggers:
///   * `explicit` — the user passed `--harness`; a deliberate binding must
///     survive into the next plain `grove do`, even in a repo where detection
///     would pick something else (e.g. a stray `.claude/` after a switch).
///   * multi-harness repo — disambiguation, as before.
/// A single-harness repo with no explicit flag stays stamp-free: detection is
/// already deterministic there.
pub fn maybe_stamp(
    repo: &Path,
    name: &str,
    chosen: &'static Harness,
    explicit: bool,
) -> Result<()> {
    let detected = harness::detect_in_repo(repo);
    if !explicit && detected.len() < 2 {
        return Ok(());
    }
    let stamp = path(repo, name);
    if let Some(parent) = stamp.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&stamp, format!("{}\n", chosen.name))?;
    Ok(())
}
```

Also update `resolve_for_launch`'s two lookup errors to use the derived list, e.g.:

```rust
        return harness::by_name(name).ok_or_else(|| {
            anyhow::anyhow!(
                "unknown harness: {}. Known: {}",
                name,
                harness::known_names()
            )
        });
```

(and the stamp-contents error likewise appends `harness::known_names()`).

In `src/launch.rs:31` update the caller:

```rust
    harness_stamp::maybe_stamp(&repo_path, &name, harness, args.harness.is_some())?;
```

- [ ] **Step 4: Run the full suite**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/harness_stamp.rs src/launch.rs tests/harness_stamp.rs
git commit -m "fix: persist explicit --harness choices in the stamp

Previously only multi-harness repos stamped, so grove do --harness codex in a
repo with just .claude/ evaporated on the next plain grove do. An explicit
choice is a binding; it now always writes. This is the migration mechanism
for the codex/pi switch: one explicit flag per grove, permanent after."
```

---

### Task 3: Harness-neutral PID handle (`GROVE_HARNESS_PID`)

**Files:**
- Modify: `src/loop_driver.rs:28,152-154,168` (doc comment + wrapper)
- Modify: `src/complete.rs:71-72,98,146-149` (opts doc, resolve, message)
- Modify: `src/llm_cli.rs:154,161,174` (verb + flag doc text)
- Modify: `tests/loop_driver.rs:66,112` (fake script + assertion)

**Interfaces:**
- Consumes: nothing new.
- Produces: sessions see `GROVE_HARNESS_PID` (and, this release only, `GROVE_CLAUDE_PID`); `complete::resolve_opts` reads `GROVE_HARNESS_PID` then falls back to `GROVE_CLAUDE_PID`. Task 7's content rewrite refers to `GROVE_HARNESS_PID`.

- [ ] **Step 1: Write the failing test**

`tests/complete.rs` exists; append (add `use std::sync::Mutex;` + a file-local `static ENV_LOCK: Mutex<()> = Mutex::new(());` if the file doesn't already have one):

```rust
#[test]
fn resolve_opts_prefers_harness_pid_and_falls_back_to_the_old_name() {
    let _g = ENV_LOCK.lock().unwrap();

    // New name wins when both are set.
    std::env::set_var("GROVE_HARNESS_PID", "111");
    std::env::set_var("GROVE_CLAUDE_PID", "222");
    let opts = grove::complete::resolve_opts(
        None,
        None,
        None,
        None,
        grove::complete::Disposition::Relaunch,
    );
    assert_eq!(opts.pid, Some(111));

    // Old name still resolves alone — one release of backward compatibility
    // for content/agents that captured the old handle.
    std::env::remove_var("GROVE_HARNESS_PID");
    let opts = grove::complete::resolve_opts(
        None,
        None,
        None,
        None,
        grove::complete::Disposition::Relaunch,
    );
    assert_eq!(opts.pid, Some(222));

    std::env::remove_var("GROVE_CLAUDE_PID");
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test --test complete resolve_opts_prefers_harness_pid`
Expected: FAIL — `opts.pid` is `None` for `GROVE_HARNESS_PID` (first assert) because only the old var is read.

- [ ] **Step 3: Implement**

`src/complete.rs:98`:

```rust
        pid: pid
            .or_else(|| env_parse("GROVE_HARNESS_PID"))
            // One release of fallback for the pre-rename handle.
            .or_else(|| env_parse("GROVE_CLAUDE_PID")),
```

`src/complete.rs:71-72` opts doc: `/// PID of the harness session to kill. Defaults to $GROVE_HARNESS_PID, exported by the loop driver (and inherited by the agent's Bash tool).`

`src/complete.rs:146-149` message: `"grove complete: no GROVE_HARNESS_PID — not running under the loop driver; exit this session manually."`

`src/loop_driver.rs:168` wrapper (exports both):

```rust
        .arg(r#"export GROVE_HARNESS_PID=$$ GROVE_CLAUDE_PID=$$; exec "$@""#)
```

`src/loop_driver.rs:151-155` doc comment: replace both `GROVE_CLAUDE_PID` mentions with `GROVE_HARNESS_PID` and "`claude`'s own PID" with "the harness session's own PID" (note the compat export: "`GROVE_CLAUDE_PID` is co-exported for one release"). Line 28 (shell sketch in the header comment): `export GROVE_HARNESS_PID=$$; exec "$harness_bin" "$@"`.

`src/llm_cli.rs`: line 154 "ends this `claude` session" → "ends this harness session"; lines 161 and 174 `GROVE_CLAUDE_PID` → `GROVE_HARNESS_PID`.

- [ ] **Step 4: Update the loop integration test to prove the new export**

`tests/loop_driver.rs:66` (fake script log line): change `"$GROVE_CLAUDE_PID"` → `"$GROVE_HARNESS_PID"`. Line 110-114 assertion message: "GROVE_HARNESS_PID must equal the session's own pid".

- [ ] **Step 5: Run the full suite**

Run: `cargo test`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/loop_driver.rs src/complete.rs src/llm_cli.rs tests/loop_driver.rs tests/complete.rs
git commit -m "feat: rename the session PID handle to GROVE_HARNESS_PID

GROVE_CLAUDE_* now means 'claude-harness-specific' in the per-harness env
scheme, so the PID handle must stop squatting on that namespace. The wrapper
co-exports the old name and complete reads it as a fallback for one release."
```

---

### Task 4: Per-harness model envs (`GROVE_<HARNESS>_<KIND>_MODEL`)

**Files:**
- Modify: `src/loop_driver.rs:193-246` (`select_model` internals + helpers)
- Modify: `tests/loop_driver.rs` (one new test)

**Interfaces:**
- Consumes: `Harness.name`.
- Produces (Task 5 reuses these exact private helpers):
  - `const KIND_SUFFIXES: [&str; 5] = ["PLANNING", "RESEARCH", "PROTOTYPE", "WORK", "REVIEW"];`
  - `fn env_suffix(kind: Kind) -> &'static str`
  - `fn model_for(harness: &Harness, kind: Kind) -> Option<String>` — specific-then-base lookup
  - `fn any_model_env(harness: &Harness) -> bool`

- [ ] **Step 1: Write the failing test**

Append to `tests/loop_driver.rs` (same shape as `loop_selects_model_by_kind`, two iterations):

```rust
// Per-harness model envs: GROVE_<HARNESS>_<KIND>_MODEL beats GROVE_<KIND>_MODEL.
// One shared kind env can't serve two harnesses at once (a codex profile name is
// garbage to pi and vice versa), so each harness gets a scoped override.
#[test]
fn per_harness_model_env_beats_the_base_var() {
    let _g = ENV_LOCK.lock().unwrap();
    let worktree_dir = TempDir::new().unwrap();
    let worktree = worktree_dir.path();
    assert!(
        std::process::Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(worktree)
            .status()
            .unwrap()
            .success(),
        "git init failed"
    );

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let counter = worktree.join("counter");
    let log = worktree.join("log");

    // Fake harness: run 1 (start/planning) materialises a work leaf + signal;
    // run 2 (continue/work) stops.
    let fake = worktree.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf '%s\n' "$*" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** work\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    std::env::set_var("GROVE_HARNESS_BIN", &fake);
    std::env::set_var("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"));
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);
    // Scoped override for the launching harness + a base var it must beat;
    // an override for a *different* harness that must be ignored.
    std::env::set_var("GROVE_CLAUDE_WORK_MODEL", "kimi-k3");
    std::env::set_var("GROVE_WORK_MODEL", "sonnet");
    std::env::set_var("GROVE_PI_PLANNING_MODEL", "must-not-leak");

    let result = loop_driver::run_loop(harness, worktree, worktree, "envgrove");

    std::env::remove_var("GROVE_HARNESS_BIN");
    std::env::remove_var("GROVE_LLM_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");
    std::env::remove_var("GROVE_CLAUDE_WORK_MODEL");
    std::env::remove_var("GROVE_WORK_MODEL");
    std::env::remove_var("GROVE_PI_PLANNING_MODEL");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<&str> = log.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    // Start/planning: no CLAUDE planning override and no base planning var ⇒
    // no --model; the pi-scoped var must not leak across harnesses.
    assert!(
        !rows[0].contains("--model"),
        "another harness's scoped var must not select a model (argv: {:?})",
        rows[0]
    );
    // Continue/work: the claude-scoped var beats the base var.
    assert!(
        rows[1].contains("--model kimi-k3") && !rows[1].contains("sonnet"),
        "GROVE_CLAUDE_WORK_MODEL must beat GROVE_WORK_MODEL (argv: {:?})",
        rows[1]
    );
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test --test loop_driver per_harness_model_env_beats_the_base_var`
Expected: FAIL — row 1 argv contains `--model sonnet` (base var used, scoped var ignored).

- [ ] **Step 3: Implement in `src/loop_driver.rs`**

Replace `select_model` (lines 209-241) and `env_model` stays as-is; add helpers:

```rust
/// Env-name suffixes for the five kinds, in taxonomy order. Shared by the
/// model lookup and (task-routing) the harness override.
const KIND_SUFFIXES: [&str; 5] = ["PLANNING", "RESEARCH", "PROTOTYPE", "WORK", "REVIEW"];

fn env_suffix(kind: Kind) -> &'static str {
    match kind {
        Kind::Planning => "PLANNING",
        Kind::Research => "RESEARCH",
        Kind::Prototype => "PROTOTYPE",
        Kind::Work => "WORK",
        Kind::Review => "REVIEW",
    }
}

/// The model value for a kind on a harness: the harness-scoped var
/// (`GROVE_<HARNESS>_<KIND>_MODEL`) beats the base (`GROVE_<KIND>_MODEL`).
/// Scoped because one shared value cannot serve two harnesses — a codex
/// profile name is garbage to pi and vice versa.
fn model_for(harness: &Harness, kind: Kind) -> Option<String> {
    let h = harness.name.to_uppercase();
    let s = env_suffix(kind);
    env_model(&format!("GROVE_{h}_{s}_MODEL")).or_else(|| env_model(&format!("GROVE_{s}_MODEL")))
}

/// Whether any model env var (scoped-to-this-harness or base) is set — the
/// gate that keeps the common unconfigured path a zero-subprocess launch.
fn any_model_env(harness: &Harness) -> bool {
    let h = harness.name.to_uppercase();
    KIND_SUFFIXES.iter().any(|s| {
        env_model(&format!("GROVE_{h}_{s}_MODEL")).is_some()
            || env_model(&format!("GROVE_{s}_MODEL")).is_some()
    })
}

fn select_model(harness: &Harness, worktree: &Path, verb: &str) -> Option<String> {
    // A harness with no model-flag template opts out entirely.
    if harness.model_args.is_empty() {
        return None;
    }
    // Nothing configured ⇒ nothing to select; skip the kind peek so the
    // common path stays a zero-subprocess, byte-for-byte-unchanged launch.
    if !any_model_env(harness) {
        return None;
    }
    let kind = if verb == "start" {
        Kind::Planning
    } else {
        resolve_kind(worktree)?
    };
    model_for(harness, kind)
}
```

Keep the existing doc comment on `select_model`, amending the lookup sentence to name the scoped-then-base order.

- [ ] **Step 4: Run the full suite**

Run: `cargo test`
Expected: PASS (existing base-var tests still pass — base lookup is the fallback).

- [ ] **Step 5: Commit**

```bash
git add src/loop_driver.rs tests/loop_driver.rs
git commit -m "feat: harness-scoped model env vars

GROVE_<HARNESS>_<KIND>_MODEL beats GROVE_<KIND>_MODEL. One shared kind var
cannot serve two harnesses at once — a codex profile name is garbage to pi —
and the trial runs both sides concurrently from one shell environment."
```

---

### Task 5: Per-kind harness routing (`GROVE_<KIND>_HARNESS`) + per-harness bin seam

**Files:**
- Modify: `src/loop_driver.rs` (`run_loop` body lines 109-122, `launch_session` line 164, new `resolve_launch`/`harness_override` helpers)
- Modify: `tests/loop_driver.rs` (two new tests)

**Interfaces:**
- Consumes: Task 4's `model_for`/`any_model_env`/`env_suffix`/`KIND_SUFFIXES`; Task 1's `harness::by_name`/`known_names`.
- Produces:
  - `GROVE_<KIND>_HARNESS` env contract (the user sets `GROVE_REVIEW_HARNESS=pi`).
  - Test seam: `GROVE_HARNESS_BIN_<NAME>` (e.g. `GROVE_HARNESS_BIN_PI`) beats `GROVE_HARNESS_BIN` beats `exec_bin`.
  - `fn resolve_launch(stamped: &'static Harness, worktree: &Path, verb: &str) -> Result<(&'static Harness, Option<String>)>`

- [ ] **Step 1: Write the failing reroute test**

Append to `tests/loop_driver.rs`:

```rust
// Per-kind harness routing: GROVE_REVIEW_HARNESS=pi must launch review leaves
// on pi even in a codex-stamped grove — the trial's "K3 reviews everywhere"
// invariant. Proven with two distinct fake binaries wired through the
// per-harness bin seam, so the argv log shows *which* harness ran each leaf.
#[test]
fn review_leaf_reroutes_to_the_review_harness() {
    let _g = ENV_LOCK.lock().unwrap();
    let worktree_dir = TempDir::new().unwrap();
    let worktree = worktree_dir.path();
    assert!(
        std::process::Command::new("git")
            .arg("init")
            .arg("-q")
            .current_dir(worktree)
            .status()
            .unwrap()
            .success(),
        "git init failed"
    );

    let skill_dir = worktree.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let counter = worktree.join("counter");
    let log = worktree.join("log");

    // Fake codex: tags rows "codex"; run 1 (start/planning) materialises a
    // *review* leaf + signal, so run 2 is a review continue.
    let fake_codex = worktree.join("fake-codex.sh");
    write_exec(
        &fake_codex,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf 'codex\t%s\n' "$*" >> "$GROVE_TEST_LOG"
if [ "$n" -eq 1 ]; then
  mkdir -p "$PWD/.grove"
  printf '# g — brief\n' > "$PWD/.grove/BRIEF.md"
  printf '# a-k1\n\n**Kind:** review\n' > "$PWD/.grove/01-a-k1.md"
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );
    // Fake pi: tags rows "pi"; never signals, so the loop stops after it.
    let fake_pi = worktree.join("fake-pi.sh");
    write_exec(
        &fake_pi,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
printf 'pi\t%s\n' "$*" >> "$GROVE_TEST_LOG"
exit 0
"#,
    );

    let harness = harness::by_name("codex").unwrap();

    std::env::set_var("GROVE_HARNESS_BIN_CODEX", &fake_codex);
    std::env::set_var("GROVE_HARNESS_BIN_PI", &fake_pi);
    std::env::set_var("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"));
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);
    std::env::set_var("GROVE_REVIEW_HARNESS", "pi");
    std::env::set_var("GROVE_CODEX_PLANNING_MODEL", "sol-xhigh");
    std::env::set_var("GROVE_PI_REVIEW_MODEL", "kimi-code/k3");

    let result = loop_driver::run_loop(harness, worktree, worktree, "reroutegrove");

    std::env::remove_var("GROVE_HARNESS_BIN_CODEX");
    std::env::remove_var("GROVE_HARNESS_BIN_PI");
    std::env::remove_var("GROVE_LLM_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");
    std::env::remove_var("GROVE_REVIEW_HARNESS");
    std::env::remove_var("GROVE_CODEX_PLANNING_MODEL");
    std::env::remove_var("GROVE_PI_REVIEW_MODEL");

    assert_eq!(result.unwrap(), LoopOutcome::Stopped);

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.splitn(2, '\t').collect())
        .collect();
    assert_eq!(rows.len(), 2, "loop should run twice (log: {log:?})");

    // Planning leaf: the stamped harness (codex) with its scoped profile.
    assert_eq!(rows[0][0], "codex", "planning stays on the stamped harness");
    assert!(
        rows[0][1].contains("--profile sol-xhigh"),
        "codex planning launches on its scoped profile (argv: {:?})",
        rows[0][1]
    );
    // Review leaf: rerouted to pi, with pi's scoped model — the launch flag
    // template must be the *post-override* harness's (--model, not --profile).
    assert_eq!(rows[1][0], "pi", "review must reroute to GROVE_REVIEW_HARNESS");
    assert!(
        rows[1][1].contains("--model kimi-code/k3"),
        "the rerouted review leaf resolves models against pi (argv: {:?})",
        rows[1][1]
    );
}

// An unknown override value must fail loudly at launch — a typo'd harness
// name that silently fell back to the stamped harness would run reviews on
// the wrong (and possibly self-reviewing) model for a whole trial.
#[test]
fn unknown_review_harness_fails_loudly() {
    let _g = ENV_LOCK.lock().unwrap();
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let worktree = repo_path.join("wt");
    fs::create_dir_all(&worktree).unwrap();

    let harness = harness::by_name("claude").unwrap();

    // Start path ⇒ kind is Planning by construction; route planning to a typo.
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_PLANNING_HARNESS", "lemur");

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "typogrove");

    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_PLANNING_HARNESS");

    let err = result.unwrap_err().to_string();
    assert!(
        err.contains("GROVE_PLANNING_HARNESS") && err.contains("lemur"),
        "the error must name the variable and the bad value (err: {err})"
    );
    assert!(
        err.contains("claude") && err.contains("codex") && err.contains("pi"),
        "the error must list the known harnesses (err: {err})"
    );
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test --test loop_driver review_leaf_reroutes unknown_review_harness`
Expected: FAIL — reroute test: both rows tagged from the default `exec_bin` path (or the launch errors because real `codex` isn't the fake); typo test: no error raised.

- [ ] **Step 3: Implement in `src/loop_driver.rs`**

Add helpers (near `model_for`):

```rust
/// The per-kind harness override: `GROVE_<KIND>_HARNESS` names the harness
/// that runs leaves of that kind, whatever the grove is stamped to (the
/// trial's "K3 reviews everywhere": GROVE_REVIEW_HARNESS=pi). Unknown names
/// fail loudly — a typo that silently fell back would misroute every review.
fn harness_override(kind: Kind) -> Result<Option<&'static Harness>> {
    let var = format!("GROVE_{}_HARNESS", env_suffix(kind));
    match std::env::var(&var) {
        Ok(name) if !name.is_empty() => {
            let h = crate::harness::by_name(&name).ok_or_else(|| {
                anyhow::anyhow!(
                    "{var}={name}: unknown harness. Known: {}",
                    crate::harness::known_names()
                )
            })?;
            Ok(Some(h))
        }
        _ => Ok(None),
    }
}

/// Whether any per-kind harness override is set — like `any_model_env`, the
/// gate that decides whether the kind peek is worth a subprocess.
fn any_harness_override_env() -> bool {
    KIND_SUFFIXES
        .iter()
        .any(|s| std::env::var(format!("GROVE_{s}_HARNESS")).is_ok_and(|v| !v.is_empty()))
}

/// Resolve where and on what the next session launches: peek the picked
/// leaf's kind (only when some routing env makes it matter), apply the
/// per-kind harness override, then resolve the model against the
/// *post-override* harness — so `GROVE_PI_REVIEW_MODEL` governs a review
/// rerouted to pi, not the stamped harness's vars.
fn resolve_launch(
    stamped: &'static Harness,
    worktree: &Path,
    verb: &str,
) -> Result<(&'static Harness, Option<String>)> {
    if !any_harness_override_env() && !any_model_env(stamped) {
        return Ok((stamped, None));
    }
    let kind = if verb == "start" {
        Some(Kind::Planning)
    } else {
        resolve_kind(worktree)
    };
    let Some(kind) = kind else {
        // No kind (empty grove / peek degraded): no basis to route or select.
        return Ok((stamped, None));
    };
    let launch = harness_override(kind)?.unwrap_or(stamped);
    let model = if launch.model_args.is_empty() {
        None
    } else {
        model_for(launch, kind)
    };
    Ok((launch, model))
}
```

Delete `select_model` (its body is now inside `resolve_launch`; keep `model_for`/`any_model_env`/`env_suffix` from Task 4). In `run_loop` replace lines 111-122:

```rust
        // Route the launch by the picked leaf's kind: per-kind harness
        // override first (GROVE_<KIND>_HARNESS), then model-per-task-kind
        // against whichever harness actually launches.
        let (launch_harness, model) = resolve_launch(harness, worktree, verb)?;

        launch_session(
            launch_harness,
            worktree,
            &session_name,
            &prompt,
            &signal_file,
            model.as_deref(),
        )?;
```

In `launch_session` replace line 164 with the per-harness seam:

```rust
    let bin = harness_bin(harness);
```

and add:

```rust
/// The binary to exec for a harness: `GROVE_HARNESS_BIN_<NAME>` (the
/// per-harness test seam — kind routing can launch two different harnesses in
/// one loop) beats the legacy global `GROVE_HARNESS_BIN`, beats `exec_bin`.
fn harness_bin(harness: &Harness) -> String {
    std::env::var(format!("GROVE_HARNESS_BIN_{}", harness.name.to_uppercase()))
        .or_else(|_| std::env::var("GROVE_HARNESS_BIN"))
        .unwrap_or_else(|_| harness.exec_bin.to_string())
}
```

- [ ] **Step 4: Run the full suite**

Run: `cargo test`
Expected: PASS — including all pre-existing loop tests (they use the global `GROVE_HARNESS_BIN` fallback and no override envs). If `unrecognised_kind_warns_the_operator_and_still_launches` runs the real `grove` binary, it inherits no override envs — confirm it still passes.

- [ ] **Step 5: Commit**

```bash
git add src/loop_driver.rs tests/loop_driver.rs
git commit -m "feat: per-kind harness routing

GROVE_<KIND>_HARNESS reroutes leaves of a kind to another harness at launch;
model resolution follows the post-override harness. This is the trial's 'K3
reviews everywhere' seam: GROVE_REVIEW_HARNESS=pi runs review leaves on the
Kimi sub even in codex-stamped groves. Unknown names fail loudly; the
GROVE_HARNESS_BIN_<NAME> seam lets tests fake two harnesses in one loop."
```

---

### Task 6: Multi-harness skill provisioning + per-harness prompt loading

**Files:**
- Modify: `src/provision.rs` (multi-target sweep, symlink-safe guard; `global_skill_dir`/`skill_dir_under_home` replaced)
- Modify: `src/launch.rs:19-31,70,83-94,104-131` (`do_grove` order, `load_prompt(harness, verb)`)
- Modify: `src/loop_driver.rs:109` (`load_prompt` call)
- Modify: `tests/provision.rs` (new sweep/guard tests)

**Interfaces:**
- Consumes: `Harness.skills_dir`, `Harness.project_dir`, `harness::HARNESSES` (Task 1).
- Produces:
  - `pub fn skill_dir_for(harness: &Harness) -> Result<PathBuf>` — `$GROVE_SKILL_DIR` override, else `$HOME/<skills_dir>/grove`. (Replaces `global_skill_dir()`.)
  - `pub fn provision_all(primary: &'static Harness) -> Result<()>` — primary's dir unconditionally + every harness whose `$HOME/<project_dir>` root exists.
  - `pub(crate) fn load_prompt(harness: &Harness, verb: &str) -> Result<String>` in `launch.rs`.
  - `pub fn provision_target(dest: &Path) -> Result<bool>` — guarded single-dir provisioning (symlink replacement + foreign-dir bail), used by tests.

- [ ] **Step 1: Write the failing tests**

Append to `tests/provision.rs` (add at top of file: `use grove::provision::provision_target;`, `use std::sync::Mutex;`, `static ENV_LOCK: Mutex<()> = Mutex::new(());`):

```rust
#[test]
fn provision_replaces_a_symlinked_grove_entry_with_a_real_dir() {
    let tmp = TempDir::new().unwrap();
    // Simulate today's layout: a real provisioned dir + a skills entry that
    // symlinks to it (the current ~/.codex/skills/grove and
    // ~/.pi/agent/skills/grove setups).
    let real = tmp.path().join("claude-skills/grove");
    fs::create_dir_all(&real).unwrap();
    provision_into(&real).unwrap();
    let linked = tmp.path().join("codex-skills/grove");
    fs::create_dir_all(linked.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&real, &linked).unwrap();

    let wrote = provision_target(&linked).unwrap();

    assert!(wrote, "a symlinked entry is replaced, not treated as warm");
    let meta = fs::symlink_metadata(&linked).unwrap();
    assert!(meta.is_dir(), "the symlink becomes a real directory");
    assert!(linked.join("SKILL.md").is_file());
    // CRITICAL: replacing the link must not have reached through it — the
    // original target keeps its content.
    assert!(
        real.join("SKILL.md").is_file(),
        "replacing the symlink must never delete through it"
    );
}

#[test]
fn provision_refuses_a_foreign_directory() {
    let tmp = TempDir::new().unwrap();
    let dest = tmp.path().join("skills/grove");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("precious.txt"), "user data, not ours").unwrap();

    let err = provision_target(&dest).unwrap_err().to_string();

    assert!(
        err.contains("precious") || err.contains("not a grove-provisioned"),
        "must refuse to clobber a dir grove didn't write (err: {err})"
    );
    assert!(
        dest.join("precious.txt").is_file(),
        "the foreign content must be untouched"
    );
}

#[test]
fn provision_all_sweeps_installed_harness_roots_and_honours_the_primary() {
    let _g = ENV_LOCK.lock().unwrap();
    let home = TempDir::new().unwrap();
    // Installed roots: claude and pi. codex is absent. pi is also the primary.
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    fs::create_dir_all(home.path().join(".pi")).unwrap();

    let old_home = std::env::var_os("HOME");
    std::env::set_var("HOME", home.path());
    std::env::remove_var("GROVE_SKILL_DIR");

    let pi = grove::harness::by_name("pi").unwrap();
    let result = grove::provision::provision_all(pi);

    match old_home {
        Some(h) => std::env::set_var("HOME", h),
        None => std::env::remove_var("HOME"),
    }
    result.unwrap();

    assert!(
        home.path().join(".claude/skills/grove/SKILL.md").is_file(),
        "installed claude root is provisioned"
    );
    assert!(
        home.path().join(".pi/agent/skills/grove/SKILL.md").is_file(),
        "the primary (pi) is provisioned — note agent/ nesting"
    );
    assert!(
        !home.path().join(".codex").exists(),
        "an absent harness root is skipped, not created"
    );
}
```

- [ ] **Step 2: Run to verify failure**

Run: `cargo test --test provision`
Expected: FAIL to compile — `provision_target`/`provision_all` don't exist.

- [ ] **Step 3: Implement in `src/provision.rs`**

Replace `provision_global_skill`, `global_skill_dir`, `skill_dir_under_home` with:

```rust
use crate::harness::{Harness, HARNESSES};

/// Provision the embedded methodology for every harness on this machine:
/// `primary` (the harness about to launch) unconditionally, plus every other
/// harness whose home root (`~/.claude`, `~/.codex`, `~/.pi`) exists. Logging
/// only when a target actually (re)writes. With `$GROVE_SKILL_DIR` set (the
/// test/dev seam) the sweep collapses to that single dir.
pub fn provision_all(primary: &'static Harness) -> Result<()> {
    if std::env::var_os("GROVE_SKILL_DIR").is_some() {
        let dest = skill_dir_for(primary)?; // the override wins inside
        if provision_target(&dest)? {
            eprintln!("grove: provisioned the skill at {}", dest.display());
        }
        return Ok(());
    }
    let home = home_dir()?;
    for h in HARNESSES {
        let installed = home.join(h.project_dir).is_dir();
        if h.name != primary.name && !installed {
            continue; // absent harness root: skip, never create
        }
        let dest = home.join(h.skills_dir).join("grove");
        if provision_target(&dest)? {
            eprintln!("grove: provisioned the {} skill at {}", h.name, dest.display());
        }
    }
    Ok(())
}

/// A harness's global skill dir: `$GROVE_SKILL_DIR` override (test/dev seam),
/// else `$HOME/<harness.skills_dir>/grove`.
pub fn skill_dir_for(harness: &Harness) -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os("GROVE_SKILL_DIR") {
        return Ok(PathBuf::from(dir));
    }
    Ok(home_dir()?.join(harness.skills_dir).join("grove"))
}

fn home_dir() -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .ok_or_else(|| anyhow::anyhow!("$HOME is not set; cannot locate the global skill dirs"))?;
    Ok(PathBuf::from(home))
}

/// Guarded single-target provisioning. Replaces `dest` only when it is ours
/// to replace: a symlink (today's cross-harness link farm — removed as a
/// *link*, never through it), a grove-provisioned dir (stamp present), or
/// absent/empty. Anything else is someone's real content: bail.
pub fn provision_target(dest: &Path) -> Result<bool> {
    if let Ok(meta) = std::fs::symlink_metadata(dest) {
        if meta.file_type().is_symlink() {
            // Order matters: remove the LINK first. remove_dir_all through a
            // symlink would delete the target's contents.
            std::fs::remove_file(dest)
                .with_context(|| format!("removing symlink {}", dest.display()))?;
        } else if meta.is_dir()
            && !dest.join(STAMP_FILE).exists()
            && std::fs::read_dir(dest)?.next().is_some()
        {
            anyhow::bail!(
                "refusing to overwrite {} — it exists but is not a \
                 grove-provisioned dir (no {} stamp); move it aside and re-run",
                dest.display(),
                STAMP_FILE
            );
        }
    }
    provision_into(dest)
}
```

(`provision_into`, `sync_to_stamp`, hashing: unchanged.) Update the inline unit test `skill_dir_is_under_dot_claude_skills_grove` to the new shape:

```rust
    #[test]
    fn skill_dirs_follow_each_harness_layout() {
        let _lock = ENV_LOCK_FOR_HOME.lock().unwrap();
        std::env::remove_var("GROVE_SKILL_DIR");
        std::env::set_var("HOME", "/home/x");
        assert_eq!(
            skill_dir_for(crate::harness::by_name("claude").unwrap()).unwrap(),
            Path::new("/home/x/.claude/skills/grove")
        );
        assert_eq!(
            skill_dir_for(crate::harness::by_name("pi").unwrap()).unwrap(),
            Path::new("/home/x/.pi/agent/skills/grove")
        );
    }
```

with a module-level `static ENV_LOCK_FOR_HOME: std::sync::Mutex<()> = std::sync::Mutex::new(());` (HOME is process-global; note this test leaves HOME set to `/home/x` — restore it from a captured value exactly as in the integration test, copy that save/restore pattern).

- [ ] **Step 4: Rewire the callers**

`src/launch.rs` — in `do_grove`, move harness resolution above provisioning and switch the call (replace lines 20-31):

```rust
    let worktree = repo::git_toplevel(&std::env::current_dir().context("getting cwd")?)?;
    let name = worktree_name(&worktree);

    let repo_path = repo::resolve(None)?;
    let harness = harness_stamp::resolve_for_launch(&repo_path, &name, args.harness.as_deref())?;
    harness_stamp::maybe_stamp(&repo_path, &name, harness, args.harness.is_some())?;

    // Provision the global skill from the embedded methodology for every
    // installed harness (and the launching one unconditionally), so the skill
    // any session reads can never drift from this binary.
    crate::provision::provision_all(harness)?;
```

`load_prompt` gains the harness (replace lines 88-94):

```rust
pub(crate) fn load_prompt(harness: &Harness, verb: &str) -> Result<String> {
    let prompt_path = crate::provision::skill_dir_for(harness)?
        .join("prompts")
        .join(format!("{}.md", verb));
    fs::read_to_string(&prompt_path)
        .with_context(|| format!("reading prompt {}", prompt_path.display()))
}
```

Callers: `launch.rs:70` → `load_prompt(harness, "retire")`; `loop_driver.rs:109` → `crate::launch::load_prompt(harness, verb)?` (the stamped harness — prompt content is identical across copies).

- [ ] **Step 5: Run the full suite**

Run: `cargo test`
Expected: PASS. The loop tests pass because `GROVE_SKILL_DIR` short-circuits `skill_dir_for`.

- [ ] **Step 6: Commit**

```bash
git add src/provision.rs src/launch.rs src/loop_driver.rs tests/provision.rs
git commit -m "feat: provision the skill for every installed harness

The embedded methodology now extracts to each harness's own skills dir
(~/.claude/skills, ~/.codex/skills, ~/.pi/agent/skills) — the launching
harness unconditionally, others when their root exists. Existing symlink
entries are replaced as links (never deleted through); foreign dirs are
refused. load_prompt reads the launching harness's copy."
```

---

### Task 7: Harness-neutral text — CLI help, verb docs, SKILL.md

**Files:**
- Modify: `src/cli.rs:4-31` (`MODEL_ENV_HELP`)
- Modify: `content/SKILL.md:82,246-248,273`
- Modify: `src/launch.rs:10-18` (do_grove doc comment), `src/loop_driver.rs:1-31` (module header)

**Interfaces:** none (text only). Per the clean-cutover-prose rule: describe the current scheme on its own terms — no "formerly claude" contrast.

- [ ] **Step 1: Rewrite `MODEL_ENV_HELP` in `src/cli.rs`**

```rust
const MODEL_ENV_HELP: &str = "\
Environment variables:
  GROVE_PLANNING_MODEL   Model for planning leaves (grilling / design).
  GROVE_RESEARCH_MODEL   Model for research leaves (produces docs/research/*.md).
  GROVE_PROTOTYPE_MODEL  Model for prototype leaves (a cheap throwaway artifact).
  GROVE_WORK_MODEL       Model for work leaves (code / docs / tests).
  GROVE_REVIEW_MODEL     Model for review leaves (fresh-context adversarial read).

  GROVE_<HARNESS>_<KIND>_MODEL   Harness-scoped override (CLAUDE / CODEX / PI),
                         e.g. GROVE_PI_REVIEW_MODEL. Beats the base var for
                         that harness. Use when two harnesses run concurrently
                         and need different values for the same kind.
  GROVE_<KIND>_HARNESS   Route leaves of one kind to another harness,
                         e.g. GROVE_REVIEW_HARNESS=pi runs every review leaf
                         on pi whatever the grove's own harness is.

The loop passes the value via the harness's launch flag at each task launch,
keyed on the picked leaf's kind: `--model` for claude and pi (pi accepts
provider/id patterns), `--profile` for codex (a codex profile binds model +
reasoning effort — define profiles in ~/.codex/config.toml). Unset ⇒ no flag:
the session inherits the harness's own default, so grove is a no-op until you
opt in and never clobbers a default you already set. Setting only some kinds
is fine — an unconfigured kind still inherits.

An in-session model switch outranks the launch flag for that one session
only; whether it persists into the next task depends on whether the next
launch passes a flag again (configured kind: yes, override gone; unconfigured
kind: the harness's own persistence rules apply).

Example:
  GROVE_CODEX_WORK_MODEL=sol-high GROVE_REVIEW_HARNESS=pi \\
  GROVE_PI_REVIEW_MODEL=kimi-code/k3 grove do";
```

- [ ] **Step 2: Rewrite the three SKILL.md passages**

`content/SKILL.md:82` — in the sentence starting "It is a thin, stateless **self-driving loop**:", replace

> launch one fresh foreground `claude` (owning the real TTY, so grilling / resize / Ctrl-C are all native)

with

> launch one fresh foreground harness session (owning the real TTY, so grilling / resize / Ctrl-C are all native)

`content/SKILL.md:246-248` — replace

> a detached killer that ends this `claude` after a short grace (so the call itself returns first). It reads its env handles from the loop driver (`GROVE_CLAUDE_PID`, `GROVE_SIGNAL_FILE`);

with

> a detached killer that ends this harness session after a short grace (so the call itself returns first). It reads its env handles from the loop driver (`GROVE_HARNESS_PID`, `GROVE_SIGNAL_FILE`);

`content/SKILL.md:273` — replace

> uses `$GROVE_CLAUDE_PID` — nothing about the working tree —

with

> uses `$GROVE_HARNESS_PID` — nothing about the working tree —

- [ ] **Step 3: Neutralise the two rustdoc headers**

`src/launch.rs:14` "one fresh foreground `claude` per task" → "one fresh foreground harness session per task". `src/loop_driver.rs:4` same phrase, same fix; in the shell sketch (lines 14-31) change `GROVE_CLAUDE_PID` → `GROVE_HARNESS_PID` and `exec claude "$@"` → `exec "$harness_bin" "$@"`.

- [ ] **Step 4: Verify no stragglers, run the suite**

Run: `grep -rn "GROVE_CLAUDE_PID" src/ content/ tests/`
Expected: exactly two functional survivors — the compat co-export in `src/loop_driver.rs` (wrapper string) and the fallback read in `src/complete.rs` (plus their compat comments and the compat test in `tests/complete.rs`). Anything else: fix it.

Run: `grep -rn "fresh foreground \`claude\`" src/ content/`
Expected: no matches.

Run: `cargo test`
Expected: PASS (content edits change the embed hash; provisioning re-extracts on next launch by design).

- [ ] **Step 5: Commit**

```bash
git add src/cli.rs src/launch.rs src/loop_driver.rs content/SKILL.md
git commit -m "docs: harness-neutral loop wording and routing-env help

The loop launches 'a harness session', not 'a claude'; the model-env help
documents the harness-scoped and kind-routing vars and each harness's launch
flag. SKILL.md refers to GROVE_HARNESS_PID."
```

---

### Task 8: CHANGELOG + full verification

**Files:**
- Modify: `CHANGELOG.md` (new `## v12.0.0` section above `## v11.0.0`)

- [ ] **Step 1: Write the changelog entry**

Insert at the top of `CHANGELOG.md` (below `# Changelog`):

```markdown
## v12.0.0

grove learns to route: a `pi` harness joins claude and codex, leaves can be
routed per **kind** to a different harness than the grove's own, and model
selection becomes harness-scoped — the shape needed to drive two harnesses
(two subscriptions) concurrently and still send every review to one reviewer.

### Breaking

- **codex launches use `--profile`, not `--model`.** A codex profile binds
  model + reasoning effort, which a bare model flag cannot; model-per-task-kind
  values for codex now name profiles you define in `~/.codex/config.toml`.
- **`GROVE_HARNESS_PID` replaces `GROVE_CLAUDE_PID`.** The loop wrapper still
  co-exports the old name and `grove-llm complete` still reads it as a
  fallback — for this release only.
- **Skill provisioning is multi-harness.** `grove do` extracts the embedded
  methodology into every installed harness's skills dir (`~/.claude/skills/grove`,
  `~/.codex/skills/grove`, `~/.pi/agent/skills/grove`), replacing symlinked
  `grove` entries with real dirs (links are removed as links, never followed).
  A `grove` entry that is neither a symlink nor grove-provisioned is refused.

### Added

- **`pi` harness** (`--harness pi`): launches `pi` with `--model` (pi accepts
  `provider/id` patterns), no session pre-naming (pi has no launch-time name
  flag), skills under `~/.pi/agent/skills`.
- **`GROVE_<KIND>_HARNESS`** — route leaves of one kind to another harness at
  launch (e.g. `GROVE_REVIEW_HARNESS=pi`). Model resolution follows the
  post-override harness. Unknown names fail loudly.
- **`GROVE_<HARNESS>_<KIND>_MODEL`** — harness-scoped model vars
  (e.g. `GROVE_PI_REVIEW_MODEL`) that beat the base `GROVE_<KIND>_MODEL`.

### Fixed

- **Explicit `--harness` now always persists** to `.grove-stamps/<name>`.
  Previously only multi-harness repos stamped, so an explicit choice in a
  repo with a single (different) harness dir silently reverted on the next
  plain `grove do`.
```

- [ ] **Step 2: Full verification**

Run: `cargo fmt --check && cargo clippy --all-targets -- -D warnings && cargo test`
Expected: all clean/PASS. Fix anything surfaced before committing.

- [ ] **Step 3: Commit**

```bash
git add CHANGELOG.md
git commit -m "docs: cut the v12.0.0 changelog entry"
```

---

### Task 9: Codex profiles (user config)

**Files:**
- Modify: `~/.codex/config.toml` (append; do NOT touch existing keys)

- [ ] **Step 1: Append the profile blocks**

```toml

[profiles.sol-xhigh]
model = "gpt-5.6-sol"
model_reasoning_effort = "xhigh"

[profiles.sol-high]
model = "gpt-5.6-sol"
model_reasoning_effort = "high"
```

- [ ] **Step 2: Smoke-test both profiles**

Run: `codex exec --profile sol-high "Reply with exactly: profile-ok" 2>&1 | tail -5`
Expected: output containing `profile-ok` (sub-billed, no API-key errors).
Run the same for `--profile sol-xhigh`.
If `codex exec` rejects `--profile`, check `codex exec --help` for the current flag spelling and record any deviation in the trial notes — the grove env values must match whatever codex accepts.

---

### Task 10: Pi ↔ Kimi provider (user config)

**Files:**
- Modify: pi settings via `pi install` (no manual file edits)

- [ ] **Step 1: Install the provider package**

Run: `pi install npm:pi-provider-kimi-code`
Expected: install success. (If the source spec differs, follow https://github.com/Leechael/pi-provider-kimi-code README — it documents the exact `pi install` source and required env vars.)

- [ ] **Step 2: Configure the Kimi Code API key**

Per the package README, export the key it expects (from kimi.com → Kimi Code API key) in `~/.zshenv`, e.g. `export KIMI_CODE_API_KEY=sk-...` — use the README's exact variable name.

- [ ] **Step 3: Discover the exact model id**

Run: `pi --list-models kimi`
Expected: the provider's K3 entry, e.g. `kimi-code/k3`. **Record the exact id — Task 11 uses it verbatim.**

- [ ] **Step 4: Live round-trip on the sub**

Run: `pi -p --model <recorded-id> "Reply with exactly: kimi-ok"`
Expected: `kimi-ok`, billed to the Kimi Code subscription (check the kimi.com usage dashboard shows the call). If the endpoint rejects pi, set the protocol env the README documents (`KIMI_CODE_PROTOCOL=anthropic`) and retry; if it still fails, STOP and report — the fallback (Kimi CLI as reviewer shell) is a design change requiring sign-off.

---

### Task 11: `~/.zshenv` rewrite (user config)

**Files:**
- Modify: `~/.zshenv:5-9` (the five `GROVE_*_MODEL` lines)

- [ ] **Step 1: Replace the grove model block**

Delete lines 5-9 (`GROVE_PLANNING_MODEL=fable` … `GROVE_REVIEW_MODEL=opus`) and insert, using the model id recorded in Task 10 step 3 wherever `kimi-code/k3` appears:

```sh
# grove model routing — codex+sol vs pi+K3 trial (2026-07)
export GROVE_CODEX_PLANNING_MODEL=sol-xhigh
export GROVE_CODEX_RESEARCH_MODEL=sol-xhigh
export GROVE_CODEX_PROTOTYPE_MODEL=sol-high
export GROVE_CODEX_WORK_MODEL=sol-high
export GROVE_PI_PLANNING_MODEL=kimi-code/k3
export GROVE_PI_RESEARCH_MODEL=kimi-code/k3
export GROVE_PI_PROTOTYPE_MODEL=kimi-code/k3
export GROVE_PI_WORK_MODEL=kimi-code/k3
export GROVE_PI_REVIEW_MODEL=kimi-code/k3
export GROVE_REVIEW_HARNESS=pi
```

- [ ] **Step 2: Verify in a fresh shell**

Run: `zsh -c 'env | grep GROVE_ | sort'`
Expected: exactly the ten vars above (plus any `KIMI_CODE_*` from Task 10); no `GROVE_PLANNING_MODEL=fable` survivors.

---

### Task 12: Skills repo — doubt-driven-development harness spawns

**Files:**
- Create: `~/Development/skills/plugins/linkuistics/skills/doubt-driven-development/references/harness-spawns.md`
- Modify: `~/Development/skills/plugins/linkuistics/skills/doubt-driven-development/SKILL.md` (two-line pointer after the "Platform" note near line 24)

Commit in `~/Development/skills` ONLY — never in `~/.claude/plugins/marketplaces/` (disposable mirror).

- [ ] **Step 1: Write the reference**

`references/harness-spawns.md`:

```markdown
# Materialising the reviewer, per harness

The discipline (ARTIFACT + CONTRACT, never the CLAIM) is harness-neutral;
only the spawn mechanics differ. Cross-model review is preferred: a different
model family reviewing the author's output catches failure modes
self-review cannot.

## From a codex session → K3 (Kimi Code subscription)

Spawn pi headless in the worktree; it can read files and run commands:

    pi -p --model kimi-code/k3 "<adversarial review prompt>"

pi persists the session, so a finding worth interrogating can be resumed
interactively afterwards (`pi --resume`).

## From a pi session → GPT-5.6-sol (OpenAI subscription)

The codex binary is the only sanctioned consumer of the OpenAI sub; spawn it
headless:

    codex exec --profile sol-xhigh "<adversarial review prompt>"

## From a Claude Code session

Use a fresh Task subagent (built-in), or either spawn above for a
cross-model read.

Model ids/profiles here match the grove trial config (~/.zshenv,
~/.codex/config.toml); update this file if those move.
```

- [ ] **Step 2: Add the SKILL.md pointer**

After the paragraph ending "The core that transfers is the *discipline*, not" (its sentence completes around line 25), append on a new paragraph:

```markdown
Harness-specific spawn commands (codex ↔ pi ↔ claude, cross-model):
see `references/harness-spawns.md`.
```

- [ ] **Step 3: Commit (in the skills repo)**

```bash
cd ~/Development/skills
git add plugins/linkuistics/skills/doubt-driven-development
git commit -m "feat(doubt-driven-development): per-harness reviewer spawn recipes

Cross-model spawns for the codex+sol / pi+K3 setup: each harness materialises
the fresh-context reviewer on the *other* side's model via sanctioned
headless invocations (pi -p, codex exec)."
```

---

### Task 13: Merge, release, stamp, verify live

**Files:** none new (release scripts + live verification)

- [ ] **Step 1: Merge the branch**

```bash
cd /Users/antony/Development/grove
git checkout main && git merge --no-ff codex-pi-harness
```

- [ ] **Step 2: Release**

Run in order: `scripts/release-doctor.sh`, then `scripts/release-build.sh`, then `scripts/release-publish.sh` (fix anything doctor flags first). Then `brew update && brew upgrade grove`.

- [ ] **Step 3: Verify the LIVE binary (not the build)**

```bash
grove --version                      # expect v12.0.0
grove do --help | grep -A2 GROVE_REVIEW_HARNESS   # expect the new help text
ls -la ~/.codex/skills/grove ~/.pi/agent/skills/grove  # after the next `grove do`: real dirs, not symlinks
```

- [ ] **Step 4: Stamp one grove per side and run a full cycle each**

In a codex-side worktree: `grove do --harness codex`; confirm `cat <repo>/.grove-stamps/<name>` → `codex`, the session opens on `--profile sol-xhigh` (planning). Drive one task to `grove-llm complete`; when a **review** leaf comes up, confirm the loop launches **pi** (K3) — this observes `GROVE_REVIEW_HARNESS` live. In a pi-side worktree: `grove do --harness pi`, same cycle, confirm K3 drives and the kimi.com dashboard registers usage.

- [ ] **Step 5: Cancel Anthropic; start the trial clock**

User action: cancel the Anthropic subscription. Note the trial end date (one month out, ~2026-08-18): compare sides on quality, quota pressure, wall-clock; then flip losing stamps and cancel the losing sub.

---

## Plan Self-Review (completed)

- **Spec coverage:** pi row + codex profiles (T1), stamp fix (T2), PID rename (T3), scoped model envs (T4), kind routing + loud errors (T5), multi-harness provisioning + guards (T6), neutral wording + env docs (T7), changelog/breaking (T8), codex profiles config (T9), pi/Kimi wiring + verify-at-implementation items (T10), zshenv (T11), skills repo reference (T12), release/stamp/live-verify/cancel runbook (T13). Error-handling section of the spec: unknown-override error (T5 test), quota-exhaustion behaviour needs no code (existing stop semantics — documented in spec), provisioning guards (T6 tests), PID fallback (T3 test).
- **Placeholder scan:** the only deliberate external unknowns are quarantined as verify steps with exact commands (T9 flag spelling, T10 model id/source spec) and an explicit STOP condition (T10 step 4).
- **Type consistency:** `maybe_stamp(..., explicit: bool)` matches T2's caller edit; `resolve_launch`/`model_for`/`env_suffix`/`KIND_SUFFIXES` defined T4, consumed T5; `skill_dir_for(&Harness)`/`provision_all(&'static Harness)`/`provision_target(&Path)` defined and consumed in T6; `load_prompt(harness, verb)` callers updated in T6; `harness_bin` consumes T5's seam names.

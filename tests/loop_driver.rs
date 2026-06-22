// Integration test for the self-driving loop (src/loop_driver.rs).
//
// Drives the *real* loop against a **fake `claude`** (a shell script wired in
// via the `GROVE_HARNESS_BIN` seam) so the mechanism is proven end-to-end with
// no real TUI: the PID handle reaches the child, relaunch is gated on the
// completion signal, and the start→continue prompt switch happens once `.grove/`
// exists.

use grove::harness;
use grove::loop_driver;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::sync::Mutex;
use tempfile::TempDir;

// The loop launch reads several process-global env vars (the harness-bin
// override + the fake's bookkeeping handles); serialize so cargo's parallel
// runner doesn't cross test wires.
static ENV_LOCK: Mutex<()> = Mutex::new(());

fn write_exec(path: &std::path::Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

#[test]
fn loop_relaunches_on_signal_and_stops_without_one() {
    let _g = ENV_LOCK.lock().unwrap();
    let repo = TempDir::new().unwrap();
    let repo_path = repo.path();

    // Prompts the loop's load_prompt must read — from the GLOBAL skill dir the
    // binary provisions (`$GROVE_SKILL_DIR`), NOT any repo-local mirror (the 9.3
    // repoint). Plant a stale mirror in the old `install_path` location to prove
    // the loop ignores it: a regression to the old read would log "STALE …".
    let skill_dir = repo_path.join("global-skill");
    let prompts = skill_dir.join("prompts");
    fs::create_dir_all(&prompts).unwrap();
    fs::write(prompts.join("start.md"), "START PROMPT").unwrap();
    fs::write(prompts.join("continue.md"), "CONTINUE PROMPT").unwrap();

    let stale = repo_path.join(".claude/skills/grove/prompts");
    fs::create_dir_all(&stale).unwrap();
    fs::write(stale.join("start.md"), "STALE START").unwrap();
    fs::write(stale.join("continue.md"), "STALE CONTINUE").unwrap();

    let worktree = repo_path.join(".grove-worktrees/loopgrove");
    fs::create_dir_all(&worktree).unwrap();

    let counter = repo_path.join("counter");
    let log = repo_path.join("log");

    // Fake claude: log <iter>\t<own-pid>\t<inherited-handle>\t<prompt>; create
    // `.grove/` after the first iteration so the loop switches start→continue;
    // fire the completion signal for the first two iterations, then stop.
    let fake = repo_path.join("fake-claude.sh");
    write_exec(
        &fake,
        r#"#!/bin/sh
n=$(cat "$GROVE_TEST_COUNTER" 2>/dev/null || echo 0)
n=$((n + 1))
echo "$n" > "$GROVE_TEST_COUNTER"
for a in "$@"; do prompt="$a"; done
printf '%s\t%s\t%s\t%s\n' "$n" "$$" "$GROVE_CLAUDE_PID" "$prompt" >> "$GROVE_TEST_LOG"
mkdir -p "$PWD/.grove"
if [ "$n" -lt 3 ]; then
  : > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );

    let harness = harness::by_name("claude").unwrap();

    std::env::set_var("GROVE_HARNESS_BIN", &fake);
    std::env::set_var("GROVE_SKILL_DIR", &skill_dir);
    std::env::set_var("GROVE_TEST_COUNTER", &counter);
    std::env::set_var("GROVE_TEST_LOG", &log);

    let result = loop_driver::run_loop(harness, repo_path, &worktree, "loopgrove");

    std::env::remove_var("GROVE_HARNESS_BIN");
    std::env::remove_var("GROVE_SKILL_DIR");
    std::env::remove_var("GROVE_TEST_COUNTER");
    std::env::remove_var("GROVE_TEST_LOG");

    result.unwrap();

    let log = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = log
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.split('\t').collect())
        .collect();

    assert_eq!(
        rows.len(),
        3,
        "loop should run 3 times then stop (log: {log:?})"
    );

    // The exec-PID trick: the handle the child sees equals the child's own pid.
    for row in &rows {
        assert_eq!(
            row[1], row[2],
            "GROVE_CLAUDE_PID must equal the session's own pid (row: {row:?})"
        );
    }

    // start→continue switch: first iteration has no `.grove/` (start), the rest
    // do (continue).
    assert_eq!(
        rows[0][3], "START PROMPT",
        "first iteration bootstraps via start"
    );
    assert_eq!(rows[1][3], "CONTINUE PROMPT", "second iteration continues");
    assert_eq!(rows[2][3], "CONTINUE PROMPT", "third iteration continues");
}

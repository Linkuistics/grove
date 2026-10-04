// The driver's lifecycle around a launch: what it refuses before one, what it
// scaffolds, and what it reports when a session ends. Every case that launches
// goes through the real `harness-dispatch` front and its compiled worker, under
// a personal policy in a temporary HOME; Grove reads no configuration.

mod support;

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

/// The ordinary fixture: a native jj workspace, which is the only kind of
/// working tree Grove drives (`docs/adr/jj-is-the-only-lane.md`).
fn init_worktree(path: &Path) {
    init_jj_worktree(path, false);
}

fn run_command(binary: &str, directory: &Path, arguments: &[&str]) {
    let output = Command::new(binary)
        .args(arguments)
        .current_dir(directory)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_jj_worktree(path: &Path, colocate: bool) {
    fs::create_dir_all(path).unwrap();
    let colocate = if colocate { "true" } else { "false" };
    run_command(
        "jj",
        path,
        &[
            "--config",
            &format!("git.colocate={colocate}"),
            "git",
            "init",
            "--quiet",
            ".",
        ],
    );
    run_command(
        "jj",
        path,
        &[
            "config",
            "set",
            "--workspace",
            "user.name",
            "\"Grove Test\"",
        ],
    );
    run_command(
        "jj",
        path,
        &[
            "config",
            "set",
            "--workspace",
            "user.email",
            "\"grove-test@example.com\"",
        ],
    );
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

/// Where a policy written by [`route_every_kind`] puts the mandate among the
/// program's arguments.
const PROMPT: &str = "${prompt}";

/// A personal dispatch policy under `home` that runs `program` for every kind,
/// with `args` as its arguments and the mandate in place of each [`PROMPT`].
fn route_every_kind(home: &Path, program: &Path, args: &[&str]) {
    let args: Vec<String> = args
        .iter()
        .map(|arg| match *arg {
            PROMPT => "request.prompt".to_owned(),
            literal => format!("{literal:?}"),
        })
        .collect();
    support::write_policy(
        home,
        &format!(
            r#"export const policy = {{
  schemaVersion: 2,
  version: "cutover-1",
  select: (request) => ({{
    status: "selected",
    program: {program:?},
    args: [{args}],
    provider: "fixture",
    model: "none",
    effort: "none",
    reason: "every kind runs the fixture's command",
  }}),
}};
"#,
            program = program.to_str().unwrap(),
            args = args.join(", "),
        ),
    );
}

/// The driver-authored sentence naming the leaf selected for one session.
///
/// It has **one home in this binary** for the same reason it has one home in the
/// driver: it is the whole of what the prompt says about the selected leaf — a
/// value, with every normative consequence of it left to the skill.
fn mandate_naming(handle: &str) -> String {
    format!("Grove mandate: the leaf selected for this session is `{handle}`")
}

fn run_grove(home: &Path, worktree: &Path) -> Output {
    // The sibling this `grove` launches every session through.
    support::harness_dispatch();
    Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(worktree)
        .env("HOME", home)
        .env_remove("CODEX_HOME")
        .output()
        .unwrap()
}

fn tree_snapshot(root: &Path) -> Vec<(String, Option<Vec<u8>>)> {
    fn walk(root: &Path, path: &Path, snapshot: &mut Vec<(String, Option<Vec<u8>>)>) {
        let mut entries = fs::read_dir(path)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.sort_by_key(fs::DirEntry::file_name);
        for entry in entries {
            let path = entry.path();
            let relative = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if entry.file_type().unwrap().is_dir() {
                snapshot.push((relative, None));
                walk(root, &path, snapshot);
            } else {
                snapshot.push((relative, Some(fs::read(path).unwrap())));
            }
        }
    }

    let mut snapshot = Vec::new();
    walk(root, root, &mut snapshot);
    snapshot
}

#[test]
fn duplicate_keys_stop_the_driver_before_launch_or_finish_allocation() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(grove.join("03-k7")).unwrap();
    fs::write(grove.join("_BRIEF.md"), "root").unwrap();
    fs::write(grove.join("01-impl--work-k1.md"), "work").unwrap();
    fs::write(grove.join("02-DONE-impl--old-k7.md"), "old").unwrap();
    fs::write(grove.join("03-k7/_branch.md"), "branch").unwrap();
    let log = fixture.path().join("launched");
    let fake = fixture.path().join("session.sh");
    write_executable(&fake, "#!/bin/sh\nprintf launched > \"$1\"\n");
    route_every_kind(&home, &fake, &[log.to_str().unwrap(), PROMPT]);
    let before = tree_snapshot(&grove);

    let output = run_grove(&home, &worktree);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("duplicate key k7"), "{stderr}");
    assert!(!log.exists(), "an ambiguous tree must not launch a session");
    assert_eq!(tree_snapshot(&grove), before);
}

#[test]
fn bare_grove_launches_the_selected_filename_kind_with_one_mandate_argument() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("personal home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let worktree = fixture.path().join("work tree with spaces");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# cutover — brief\n").unwrap();
    fs::write(
        grove.join("01-impl--selected-work-k7.md"),
        "# selected-work-k7\n",
    )
    .unwrap();

    let argv_log = fixture.path().join("exact argv.log");
    let fake_command = fixture.path().join("selected command.sh");
    write_executable(
        &fake_command,
        r#"#!/bin/sh
log=$1
shift
{
  printf 'cwd=<%s>\n' "$PWD"
  printf 'argc=<%s>\n' "$#"
  for argument do
    printf 'arg=<%s>\n' "$argument"
  done
  printf 'signal=<%s>\n' "${GROVE_SIGNAL_FILE-unset}"
  printf 'launch=<%s>\n' "$GROVE_LAUNCH_DIR"
  printf 'dispatch_exit=<%s>\n' "$HARNESS_DISPATCH_EXIT_FILE"
  printf 'legacy_harness_pid=<%s>\n' "${GROVE_HARNESS_PID-unset}"
  printf 'legacy_claude_pid=<%s>\n' "${GROVE_CLAUDE_PID-unset}"
  printf 'unrelated=<%s>\n' "${UNRELATED_AMBIENT-unset}"
  printf 'harness=<%s>\n' "${GROVE_HARNESS_BIN-unset}"
  printf 'model=<%s>\n' "${GROVE_IMPL_MODEL-unset}"
  printf 'skill=<%s>\n' "${GROVE_SKILL_DIR-unset}"
  printf 'llm=<%s>\n' "${GROVE_LLM_BIN-unset}"
} > "$log"
exit 0
"#,
    );
    route_every_kind(
        &home,
        &fake_command,
        &[argv_log.to_str().unwrap(), "--before", PROMPT, "--after"],
    );

    support::harness_dispatch();
    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(&worktree)
        .env_clear()
        .env("HOME", &home)
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("GROVE_SIGNAL_FILE", fixture.path().join("stale-signal"))
        .env("GROVE_LAUNCH_DIR", fixture.path().join("stale-launch"))
        .env(
            "HARNESS_DISPATCH_EXIT_FILE",
            fixture.path().join("stale-exit"),
        )
        .env("GROVE_HARNESS_PID", "stale-harness-pid")
        .env("GROVE_CLAUDE_PID", "stale-claude-pid")
        .env("UNRELATED_AMBIENT", "preserved")
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let log = fs::read_to_string(argv_log).unwrap();
    let canonical_worktree = worktree.canonicalize().unwrap();
    assert!(
        log.contains(&format!("cwd=<{}>\n", canonical_worktree.display())),
        "{log:?}"
    );
    assert!(log.contains("argc=<3>\narg=<--before>\n"), "{log:?}");
    // The prompt argument carries the guaranteed core, so it is asserted on the
    // **load instruction's first clause** rather than on a slice of methodology:
    // that clause is what reaches a session first, and the core's shape and
    // wording are pinned whole in `tests/prompt.rs`.
    assert!(
        log.contains("arg=<**Load the `grove-impl` skill now**"),
        "{log:?}"
    );
    assert!(log.contains(&mandate_naming("selected-work-k7")), "{log:?}");
    assert!(log.contains("\narg=<--after>\n"), "{log:?}");
    let signal = log
        .lines()
        .find_map(|line| line.strip_prefix("signal=<")?.strip_suffix('>'))
        .expect("the selected command did not record its signal path");
    assert_eq!(signal, "unset");
    let launch = log
        .lines()
        .find_map(|line| line.strip_prefix("launch=<")?.strip_suffix('>'))
        .unwrap();
    assert_ne!(
        launch,
        fixture.path().join("stale-launch").to_str().unwrap()
    );
    assert_eq!(
        Path::new(launch).parent().unwrap(),
        canonical_worktree.join(".jj/grove")
    );
    assert!(
        !Path::new(launch).exists(),
        "an own-exit launch directory was retained"
    );
    let exit = log
        .lines()
        .find_map(|line| line.strip_prefix("dispatch_exit=<")?.strip_suffix('>'))
        .unwrap();
    assert_ne!(exit, fixture.path().join("stale-exit").to_str().unwrap());
    assert_eq!(Path::new(exit).parent(), Some(Path::new(launch)));
    assert!(!fixture.path().join("stale-exit").exists());
    assert!(log.contains("legacy_harness_pid=<unset>\n"), "{log:?}");
    assert!(log.contains("legacy_claude_pid=<unset>\n"), "{log:?}");
    assert!(log.contains("unrelated=<preserved>\n"), "{log:?}");
    assert!(log.contains("harness=<unset>\n"), "{log:?}");
    assert!(log.contains("model=<unset>\n"), "{log:?}");
    assert!(log.contains("skill=<unset>\n"), "{log:?}");
    assert!(log.contains("llm=<unset>\n"), "{log:?}");
    // Bare startup provisions canonical `.agents/skills` for this Codex home.
    // A fresh legacy `.codex/skills` installation is deliberately not created.
    assert!(!home.join(".codex/skills/grove").exists());
}

fn assert_bare_grove_launches_a_session_in_a_jj_worktree(colocate: bool) {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join(if colocate {
        "colocated-jj"
    } else {
        "native-jj"
    });
    init_jj_worktree(&worktree, colocate);
    assert_eq!(worktree.join(".git").exists(), colocate);

    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# native-jj — brief\n").unwrap();
    fs::write(
        grove.join("01-impl--task-k1.md"),
        "# task-k1\n\n## Goal\nLaunch.\n",
    )
    .unwrap();

    let cwd_log = fixture.path().join("jj-cwd.log");
    let fake = fixture.path().join("record-jj-cwd.sh");
    write_executable(
        &fake,
        &format!("#!/bin/sh\npwd -P > {}\n", shell_quote(&cwd_log)),
    );
    route_every_kind(&home, &fake, &[PROMPT]);

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(cwd_log).unwrap().trim(),
        worktree.canonicalize().unwrap().to_str().unwrap()
    );
}

#[test]
fn bare_grove_launches_a_session_in_a_native_jj_worktree() {
    assert_bare_grove_launches_a_session_in_a_jj_worktree(false);
}

#[test]
fn bare_grove_launches_a_session_in_a_colocated_jj_worktree() {
    assert_bare_grove_launches_a_session_in_a_jj_worktree(true);
}

// The bare path acquires the workspace lease *before* it touches the tree or
// launches anything, so a control directory it cannot create has to be reported
// as itself rather than surfacing as whatever the next step would have
// complained about. Stated black-box and adversarially: the tree is legacy
// **and** no policy is installed, so both later steps have a loud failure ready
// — the run must still name the control directory, and migrate nothing.
#[test]
fn an_unwritable_control_directory_fails_before_tree_access_or_launch() {
    let fixture = TempDir::new().unwrap();
    // No dispatch policy at all: reaching the launch would report the missing
    // one instead.
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# unwritable — brief\n").unwrap();
    fs::write(grove.join("01-task-k1.md"), "# task-k1\n\n**Kind:** impl\n").unwrap();
    let before = tree_snapshot(&grove);

    let jj_directory = worktree.join(".jj");
    fs::set_permissions(&jj_directory, fs::Permissions::from_mode(0o500)).unwrap();
    let output = run_grove(&home, &worktree);
    fs::set_permissions(&jj_directory, fs::Permissions::from_mode(0o700)).unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(
        stderr.contains("control directory") && stderr.contains("is not usable"),
        "the failure must name the control directory it could not create: {stderr}"
    );
    assert!(
        !stderr.contains("malformed Grove") && !stderr.contains("harness-dispatch"),
        "neither the tree nor the launch may have been reached: {stderr}"
    );
    assert_eq!(
        tree_snapshot(&grove),
        before,
        "a lease that was never acquired must leave the legacy tree unmigrated"
    );
}

#[test]
fn fresh_grove_creates_and_launches_the_requirements_leaf() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("fresh worktree");
    init_worktree(&worktree);
    let log = fixture.path().join("fresh.log");
    let fake = fixture.path().join("fresh-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
exit 0
"#,
    );
    route_every_kind(&home, &fake, &[log.to_str().unwrap(), PROMPT]);

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(worktree
        .join(".grove/01-requirements--plan-k1.md")
        .is_file());
    let prompt = fs::read_to_string(log).unwrap();
    assert!(prompt.contains(&mandate_naming("plan-k1")), "{prompt}");
}

/// **A taskless root stops the driver with a sentence, and repairs nothing.**
///
/// It used to be completed: `root-init` created the root and its charter under
/// one lock and appended the first leaf under another, so a death between them
/// left exactly this, and bare `grove` finished the job. `collapse-tree-access-k13`
/// made the whole grove one store operation, which closes that window — so a
/// root holding a charter and no task is now something *else* emptied, and
/// principle 2 says an anomaly grove did not cause gets a message rather than
/// machinery.
#[test]
fn a_taskless_root_stops_the_driver_before_selection_and_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("taskless-root");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    let created = root_init(&worktree, "custom-plan");

    fs::remove_file(&created[1]).unwrap();
    let prompt_log = fixture.path().join("taskless.log");
    let fake = fixture.path().join("taskless-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
exit 0
"#,
    );
    route_every_kind(&home, &fake, &[prompt_log.to_str().unwrap(), PROMPT]);

    let output = run_grove(&home, &worktree);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(stderr.contains("holds no task"), "{stderr}");
    assert!(
        stderr.contains("jj undo"),
        "the refusal must name the fix: {stderr}"
    );
    assert!(
        !grove.join("01-requirements--plan-k1.md").exists(),
        "the driver repaired a tree it should have refused"
    );
    assert!(!prompt_log.exists(), "the driver launched a session");
}

// Each launch selects afresh. The first session retires its leaf, writes the
// next one under another kind, and replaces the owner's policy before it
// signals; the relaunch is selected from the policy as it then stands, for the
// kind the new leaf's filename names.
#[test]
fn relaunch_selects_afresh_from_the_policy_and_uses_the_new_filename_kind() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("reload-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# reload — brief\n").unwrap();
    fs::write(grove.join("01-impl--first-k1.md"), "# first-k1\n").unwrap();

    let log = fixture.path().join("reload.log");
    let next_policy = fixture.path().join("next-policy.ts");
    let active_policy = home.join(".config/harness-dispatch/policy.ts");
    let fake = fixture.path().join("reload-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
log=$1
marker=$2
next_policy=$3
active_policy=$4
prompt=$5
printf '%s|%s\n' "$marker" "$prompt" >> "$log"
if [ "$marker" = first-policy ]; then
  mv .grove/01-impl--first-k1.md .grove/01-DONE-impl--first-k1.md
  printf '# second-k2\n' > .grove/02-design--second-k2.md
  cp "$next_policy" "$active_policy"
  "$6" exit
fi
exit 0
"#,
    );
    let dispatch = support::harness_dispatch();
    let args = |marker: &'static str| {
        [
            log.to_str().unwrap(),
            marker,
            next_policy.to_str().unwrap(),
            active_policy.to_str().unwrap(),
            PROMPT,
            dispatch.to_str().unwrap(),
        ]
    };
    route_every_kind(&home, &fake, &args("second-policy"));
    fs::rename(&active_policy, &next_policy).unwrap();
    route_every_kind(&home, &fake, &args("first-policy"));

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rows = fs::read_to_string(log).unwrap();
    assert!(rows.contains("first-policy|"), "{rows}");
    assert!(rows.contains(&mandate_naming("first-k1")), "{rows}");
    assert!(rows.contains("second-policy|"), "{rows}");
    assert!(rows.contains(&mandate_naming("second-k2")), "{rows}");
}

#[test]
fn insertion_during_launch_does_not_change_the_session_mandate() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("insert-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# insertion — brief\n").unwrap();
    fs::write(grove.join("02-impl--selected-k7.md"), "# selected-k7\n").unwrap();

    let prompt_log = fixture.path().join("insert.log");
    let fake = fixture.path().join("insert-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
printf '# inserted-k8\n' > .grove/01-design--inserted-k8.md
exit 0
"#,
    );
    route_every_kind(&home, &fake, &[prompt_log.to_str().unwrap(), PROMPT]);

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let prompt = fs::read_to_string(prompt_log).unwrap();
    assert!(prompt.contains(&mandate_naming("selected-k7")), "{prompt}");
    assert!(!prompt.contains("inserted-k8"), "{prompt}");
    assert!(grove.join("01-design--inserted-k8.md").is_file());
}

// A program the policy selects that is not there launches nothing: dispatch
// refuses, naming it, and Grove reports the kind and the handle and stops with
// the leaf live and its channel gone. Rerunning under a corrected policy
// launches that leaf.
#[test]
fn an_unavailable_selected_program_stops_the_loop_with_the_leaf_live_and_no_channel_left() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("spawn-failure-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# failure — brief\n").unwrap();
    let leaf = grove.join("01-impl--still-live-k9.md");
    fs::write(&leaf, "# still-live-k9\n").unwrap();
    let missing = fixture.path().join("missing selected executable");
    route_every_kind(&home, &missing, &[PROMPT]);

    let output = run_grove(&home, &worktree);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(leaf.is_file());
    assert!(stderr.contains("status exit status: 127"), "{stderr}");
    assert!(
        stderr.contains("session kind `impl` for `still-live-k9` failed"),
        "{stderr}"
    );
    assert!(stderr.contains(missing.to_str().unwrap()), "{stderr}");
    let leaked_signal_channels = fs::read_dir(worktree.join(".jj/grove"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .filter(|name| name.to_string_lossy().starts_with("signal-"))
        .collect::<Vec<_>>();
    assert!(
        leaked_signal_channels.is_empty(),
        "the refused launch leaked signal channels: {leaked_signal_channels:?}"
    );

    let restart_marker = fixture.path().join("restart-launched");
    let restart = fixture.path().join("restart-command.sh");
    write_executable(
        &restart,
        &format!(
            "#!/bin/sh\nprintf restarted > {}\n",
            shell_quote(&restart_marker)
        ),
    );
    route_every_kind(&home, &restart, &[PROMPT]);

    let restarted = run_grove(&home, &worktree);

    assert!(
        restarted.status.success(),
        "restart failed: {}",
        String::from_utf8_lossy(&restarted.stderr)
    );
    assert!(restart_marker.is_file());
    assert!(leaf.is_file());
}

#[test]
fn nonsignalled_nonzero_exit_reports_status_elapsed_kind_and_handle() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("nonzero-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# nonzero — brief\n").unwrap();
    fs::write(grove.join("01-design--crashing-k4.md"), "# crashing-k4\n").unwrap();
    let fake = fixture.path().join("exit-23.sh");
    write_executable(&fake, "#!/bin/sh\nexit 23\n");
    route_every_kind(&home, &fake, &[PROMPT]);

    let output = run_grove(&home, &worktree);

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("status exit status: 23"), "{stderr}");
    assert!(stderr.contains("elapsed "), "{stderr}");
    assert!(
        stderr.contains("session kind `design` for `crashing-k4` failed"),
        "{stderr}"
    );
    assert!(stderr.contains("loop stopped"), "{stderr}");
}

// A session that fails for its own reasons may have moved the tree first. This
// one retires its leaf with the real verb and then exits 23 without signalling.
// The driver looks at no tree before it reports, so it says the leaf is live
// only of a refused launch, and a rerun goes on from the tree as it stands: to
// the next leaf.
#[test]
fn a_session_that_retires_its_leaf_and_then_fails_is_not_reported_as_leaving_it_live() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("retired-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# retired — brief\n").unwrap();
    fs::write(grove.join("01-impl--retired-k1.md"), "# retired-k1\n").unwrap();
    fs::write(grove.join("02-impl--next-k2.md"), "# next-k2\n").unwrap();
    let prompts = fixture.path().join("prompts.log");
    let fake = fixture.path().join("retire-then-fail.sh");
    write_executable(
        &fake,
        &format!(
            "#!/bin/sh\n\
             printf '%s\\n' \"$2\" >> \"$1\"\n\
             [ -e .grove/01-impl--retired-k1.md ] || exit 0\n\
             {grove_llm} leaf-retire .grove/01-impl--retired-k1.md > /dev/null || exit 91\n\
             exit 23\n",
            grove_llm = shell_quote(&support::grove_llm()),
        ),
    );
    route_every_kind(&home, &fake, &[prompts.to_str().unwrap(), PROMPT]);

    let output = run_grove(&home, &worktree);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(grove.join("01-DONE-impl--retired-k1.md").is_file());
    for said in [
        "status exit status: 23",
        "session kind `impl` for `retired-k1` failed",
        "rerun `grove` to continue",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    assert!(
        !stderr.contains("The leaf is still live"),
        "the driver read no tree, and this leaf is retired: {stderr}"
    );
    let first = fs::read_to_string(&prompts).unwrap();
    assert!(first.contains(&mandate_naming("retired-k1")), "{first}");
    assert!(!first.contains("next-k2"), "{first}");

    let rerun = run_grove(&home, &worktree);

    assert!(
        rerun.status.success(),
        "{}",
        String::from_utf8_lossy(&rerun.stderr)
    );
    let both = fs::read_to_string(&prompts).unwrap();
    assert!(both.contains(&mandate_naming("next-k2")), "{both}");
}

/// Commit subjects in `worktree`, newest first (`git log`'s own order).
fn git_subjects(worktree: &Path) -> Vec<String> {
    let output = Command::new("git")
        .args(["log", "--format=%s"])
        .current_dir(worktree)
        .output()
        .unwrap();
    // `git log` exits non-zero on a worktree with no commits yet. That is an
    // empty history rather than a failure, so it must not be read as one — the
    // baseline is taken before the run, when a fixture may legitimately have
    // seeded nothing.
    if !output.status.success() {
        return Vec::new();
    }
    String::from_utf8(output.stdout)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

/// Each **withdrawn** layout is refused by one bare `grove`, and the refusal is
/// the whole of what happens.
///
/// Both used to migrate. What replaces that coverage now their readers are gone
/// is the property that matters: such a tree is still **stopped on** rather than
/// falling through as a tree with nothing in it. The distinction is not
/// cosmetic — every name in these layouts is positioned but unkeyed, so the
/// grammar disclaims all of them, and a root that read as empty would have the
/// driver's finish sentinel written into it. So this asserts the refusal *and*
/// that the tree is byte-identical afterwards, which is the claim a silent
/// misclassification would break while an error message alone would not.
///
/// The refusal no longer names which withdrawn layout it met — that classifier
/// was migration's, and went with it (`delete-migration-k6`). What it names
/// instead is the grammar grove does read and the entries that are not in it,
/// which is what an operator needs either way (principle 2).
#[test]
fn a_withdrawn_layout_is_refused_without_touching_the_tree() {
    /// Worktree name, seeded files, and the entries the refusal must list.
    type Case = (
        &'static str,
        &'static [(&'static str, &'static str)],
        &'static [&'static str],
    );

    let cases: [Case; 2] = [
        (
            "nnn-slug-worktree",
            &[
                (
                    "done/010-groundwork.md",
                    "# 010-groundwork\n\n**Kind:** impl\n\n## Goal\nDone.\n",
                ),
                ("020-spec/_BRIEF.md", "# 020-spec — brief\n"),
                (
                    "020-spec/010-draft.md",
                    "# 010-draft\n\n**Kind:** design\n\n## Goal\nDraft.\n",
                ),
                ("030-ship.md", "# 030-ship\n\n## Goal\nShip.\n"),
            ],
            &["020-spec", "030-ship.md", "done"],
        ),
        (
            "v1-flat-worktree",
            &[
                (
                    "1-[1]-groundwork.DONE.md",
                    "# 1-[1]-groundwork\n\n**Kind:** impl\n\n## Goal\nDone.\n",
                ),
                ("2-[2]-spec._BRIEF.md", "# 2-[2]-spec — brief\n"),
                (
                    "2.1-[3]-draft.md",
                    "# 2.1-[3]-draft\n\n**Kind:** design\n\n## Goal\nDraft.\n",
                ),
            ],
            &["1-[1]-groundwork.DONE.md", "2.1-[3]-draft.md"],
        ),
    ];

    for (name, files, entries) in cases {
        let fixture = TempDir::new().unwrap();
        let home = fixture.path().join("home");
        fs::create_dir_all(home.join(".codex")).unwrap();
        let worktree = fixture.path().join(name);
        init_worktree(&worktree);
        let grove = worktree.join(".grove");
        fs::create_dir_all(&grove).unwrap();
        for (relative, body) in files {
            let path = grove.join(relative);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, body).unwrap();
        }
        run_command("jj", &worktree, &["commit", "-m", "seed legacy"]);

        let before = tree_snapshot(&grove);
        let seeded_subjects = git_subjects(&worktree);

        let launched = fixture.path().join("launched.log");
        let configured = fixture.path().join("configured.sh");
        write_executable(
            &configured,
            &format!(
                "#!/bin/sh\nprintf 'launched\\n' >> {}\nexit 0\n",
                shell_quote(&launched)
            ),
        );
        route_every_kind(&home, &configured, &[PROMPT]);

        let output = run_grove(&home, &worktree);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        assert!(
            !output.status.success(),
            "bare grove must stop on a {name} tree; stderr was {stderr}"
        );
        assert!(
            stderr.contains("malformed Grove") && stderr.contains("NN-k<key>"),
            "the refusal must name the grammar grove reads: {stderr}"
        );
        assert!(
            entries.iter().any(|entry| stderr.contains(entry)),
            "the refusal must name an offending entry: {stderr}"
        );
        assert!(
            !launched.exists(),
            "no session may be launched over a tree Grove refused to read"
        );
        assert_eq!(
            tree_snapshot(&grove),
            before,
            "a refused {name} tree is byte-identical afterwards"
        );
        assert_eq!(
            git_subjects(&worktree),
            seeded_subjects,
            "a refusal commits nothing"
        );
    }
}

/// The finish sentinel is a leaf grove writes itself, and no policy is asked
/// about its kind before it is written. A policy that refuses `finish` is met at
/// the launch that follows: the leaf is there, the refusal is dispatch's own,
/// and the loop stops with the leaf live.
#[test]
fn a_finish_leaf_is_written_though_the_policy_refuses_its_kind() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    support::write_policy(
        &home,
        r#"export const policy = {
  schemaVersion: 2,
  version: "cutover-1",
  select: (request) => ({
    status: "refused",
    code: "unrouted_kind",
    message: `no command for ${request.kind}`,
    remedy: "route the kind in the fixture policy",
  }),
};
"#,
    );
    let worktree = fixture.path().join("no-finish-route");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# no-finish-route — brief\n").unwrap();
    fs::write(
        grove.join("01-DONE-impl--finished-k1.md"),
        "# finished-k1\n",
    )
    .unwrap();

    let output = run_grove(&home, &worktree);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        grove.join("02-finish--finish-k2.md").is_file(),
        "the finish leaf is written before any policy is asked: {stderr}"
    );
    for said in [
        "refused (policy_refused, stage selection)",
        "  policy code: unrouted_kind",
        "session kind `finish` for `finish-k2` failed",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
}

#[test]
fn empty_current_tree_allocates_and_launches_one_resumable_finish_leaf() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("empty-current");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# empty-current — brief\n").unwrap();
    fs::write(
        grove.join("01-DONE-impl--finished-k1.md"),
        "# finished-k1\n",
    )
    .unwrap();
    let launch_log = fixture.path().join("resume.log");
    let configured = fixture.path().join("resume-command.sh");
    write_executable(
        &configured,
        r#"#!/bin/sh
printf '%s\n' "$2" > "$1"
exit 0
"#,
    );
    route_every_kind(&home, &configured, &[launch_log.to_str().unwrap(), PROMPT]);

    let finish_output = run_grove(&home, &worktree);

    assert!(
        finish_output.status.success(),
        "{}",
        String::from_utf8_lossy(&finish_output.stderr)
    );
    let finish = grove.join("02-finish--finish-k2.md");
    assert!(finish.is_file());
    let finish_body = fs::read_to_string(&finish).unwrap();
    assert!(finish_body.starts_with("# finish-k2\n"));
    assert!(finish_body.contains("grove-llm finish-commit finish-k2"));
    assert!(finish_body.contains("grove-llm complete --done"));
    assert!(!finish_body.contains("**Kind:**"));
    assert!(fs::read_to_string(&launch_log)
        .unwrap()
        .contains(&mandate_naming("finish-k2")));

    fs::write(grove.join("03-design--resumed-k3.md"), "# resumed-k3\n").unwrap();
    let resumed_output = run_grove(&home, &worktree);

    assert!(
        resumed_output.status.success(),
        "{}",
        String::from_utf8_lossy(&resumed_output.stderr)
    );
    assert!(fs::read_to_string(&launch_log)
        .unwrap()
        .contains(&mandate_naming("resumed-k3")));
    assert_eq!(
        fs::read_dir(&grove)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry
                .file_name()
                .to_string_lossy()
                .contains("finish--finish"))
            .count(),
        1
    );

    fs::rename(
        grove.join("03-design--resumed-k3.md"),
        grove.join("03-DONE-design--resumed-k3.md"),
    )
    .unwrap();
    let reused_output = run_grove(&home, &worktree);

    assert!(
        reused_output.status.success(),
        "{}",
        String::from_utf8_lossy(&reused_output.stderr)
    );
    assert!(fs::read_to_string(&launch_log)
        .unwrap()
        .contains(&mandate_naming("finish-k2")));
    assert_eq!(
        fs::read_dir(&grove)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|entry| entry
                .file_name()
                .to_string_lossy()
                .contains("finish--finish"))
            .count(),
        1
    );
}

#[test]
fn cli_metadata_exposes_view_and_writes_no_skill_directory() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let help = Command::new(env!("CARGO_BIN_EXE_grove"))
        .arg("--help")
        .env("HOME", &home)
        .output()
        .unwrap();
    let version = Command::new(env!("CARGO_BIN_EXE_grove"))
        .arg("--version")
        .env("HOME", &home)
        .output()
        .unwrap();
    let obsolete = Command::new(env!("CARGO_BIN_EXE_grove"))
        .arg("do")
        .env("HOME", &home)
        .output()
        .unwrap();

    assert!(help.status.success());
    let help = String::from_utf8(help.stdout).unwrap();
    assert!(help.contains("Usage: grove"), "{help}");
    assert!(help.contains("Commands:"), "{help}");
    assert!(help.contains("view"), "{help}");
    assert!(!help.contains("grove do"), "{help}");
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        format!("grove {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(!obsolete.status.success());
    // Metadata paths and refused verbs bypass provisioning, even with a Codex
    // marker. They create neither canonical skills nor a legacy installation.
    assert!(!home.join(".agents/skills").exists());
    assert!(!home.join(".codex/skills/grove").exists());
}

/// Scaffold a grove the way `grove-llm root-init` does: read the slug, take the
/// vacancy, then write.
///
/// The verb takes a `Vacancy` since `loop-crate-verbs-k21` — that is what makes
/// *cannot clobber a live grove* a fact about the types — so a fixture that
/// wants a fresh grove composes the two halves exactly as the CLI does.
fn root_init(worktree: &Path, slug: &str) -> Vec<std::path::PathBuf> {
    let slug = grove_loop::Slug::new(slug).expect("a fixture slug must be well-formed");
    let grove_loop::Writing::Vacancy(vacancy) =
        grove_loop::write(worktree).expect("opening a fresh worktree")
    else {
        panic!("{} already holds a grove", worktree.display());
    };
    let initialized =
        grove_loop::verbs::root_init(vacancy, &slug, &grove_loop::Kind::requirements())
            .expect("scaffolding a grove");
    vec![initialized.brief, initialized.first_leaf]
}

use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::{Command, Output};

use tempfile::TempDir;

const SESSION_KINDS: &[&str] = &[
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
];

fn init_git_worktree(path: &Path) {
    fs::create_dir_all(path).unwrap();
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(path)
        .status()
        .unwrap()
        .success());
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

fn write_complete_config(home: &Path, template: &str) {
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {template:?}\n"))
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();
}

fn run_grove(home: &Path, worktree: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(worktree)
        .env("HOME", home)
        .output()
        .unwrap()
}

fn path_with_front(directory: &Path) -> OsString {
    let mut paths = vec![directory.to_path_buf()];
    paths.extend(std::env::split_paths(
        &std::env::var_os("PATH").unwrap_or_default(),
    ));
    std::env::join_paths(paths).unwrap()
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
fn bare_grove_launches_the_selected_filename_kind_with_one_mandate_argument() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("personal home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let worktree = fixture.path().join("work tree with spaces");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# cutover — brief\n").unwrap();
    fs::write(
        grove.join("01-impl-selected-work-k7.md"),
        "# selected-work-k7\n",
    )
    .unwrap();

    let argv_log = fixture.path().join("exact argv.log");
    let fake_command = fixture.path().join("configured command.sh");
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
  printf 'signal=<%s>\n' "$GROVE_SIGNAL_FILE"
  printf 'target=<%s>\n' "${GROVE_SESSION_TARGET-unset}"
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
    let template = format!(
        "{} {} --before '${{prompt}}' --after",
        shell_quote(&fake_command),
        shell_quote(&argv_log)
    );
    write_complete_config(&home, &template);

    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(&worktree)
        .env_clear()
        .env("HOME", &home)
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("GROVE_SIGNAL_FILE", fixture.path().join("stale-signal"))
        .env("GROVE_HARNESS_PID", "stale-harness-pid")
        .env("GROVE_CLAUDE_PID", "stale-claude-pid")
        .env("GROVE_SESSION_TARGET", "stale-target-must-not-leak")
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
    assert!(
        log.contains("Continue this grove — use the grove skill"),
        "{log:?}"
    );
    assert!(log.contains("selected-work-k7"), "{log:?}");
    assert!(log.contains("do not call `grove-llm pick`"), "{log:?}");
    assert!(log.contains("\narg=<--after>\n"), "{log:?}");
    let signal = log
        .lines()
        .find_map(|line| line.strip_prefix("signal=<")?.strip_suffix('>'))
        .expect("configured command did not record its signal path");
    assert_ne!(
        signal,
        fixture.path().join("stale-signal").to_str().unwrap()
    );
    assert_eq!(
        Path::new(signal).parent().unwrap(),
        canonical_worktree.join(".git/grove")
    );
    assert!(log.contains("target=<unset>\n"), "{log:?}");
    assert!(log.contains("legacy_harness_pid=<unset>\n"), "{log:?}");
    assert!(log.contains("legacy_claude_pid=<unset>\n"), "{log:?}");
    assert!(log.contains("unrelated=<preserved>\n"), "{log:?}");
    assert!(log.contains("harness=<unset>\n"), "{log:?}");
    assert!(log.contains("model=<unset>\n"), "{log:?}");
    assert!(log.contains("skill=<unset>\n"), "{log:?}");
    assert!(log.contains("llm=<unset>\n"), "{log:?}");
    assert!(home.join(".codex/skills/grove/SKILL.md").is_file());
}

#[test]
fn linked_worktree_expands_scalars_through_literal_env_word_zero() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let repository = fixture.path().join("main-repository");
    init_git_worktree(&repository);
    run_command(
        "git",
        &repository,
        &[
            "-c",
            "user.email=t@example.com",
            "-c",
            "user.name=Test",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "seed",
        ],
    );
    let worktree = fixture.path().join("linked-worktree");
    run_command(
        "git",
        &repository,
        &["worktree", "add", "-q", worktree.to_str().unwrap()],
    );

    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# linked — brief\n").unwrap();
    fs::write(grove.join("01-impl-scalars-k7.md"), "# scalars-k7\n").unwrap();

    let argv_log = fixture.path().join("scalar-argv.log");
    let fake = fixture.path().join("record-scalars.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf 'mode=%s\nrepo=%s\nprompt=%s\nworktree=%s\nsession=%s\n' \
    "$MODE" "$1" "$2" "$4" "$5" > "$3"
"#,
    );
    let template = format!(
        "env MODE='$(printf shell-evaluated)' {} '${{repo}}' '${{prompt}}' {} '${{worktree}}' '${{session_name}}'",
        shell_quote(&fake),
        shell_quote(&argv_log)
    );
    write_complete_config(&home, &template);

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let argv = fs::read_to_string(argv_log).unwrap();
    assert!(argv.contains("mode=$(printf shell-evaluated)\n"), "{argv}");
    assert!(
        argv.contains(&format!(
            "repo={}\n",
            repository.canonicalize().unwrap().display()
        )),
        "{argv}"
    );
    assert!(
        argv.contains(&format!(
            "worktree={}\n",
            worktree.canonicalize().unwrap().display()
        )),
        "{argv}"
    );
    assert!(
        argv.contains("session=main-repository: linked-worktree grove\n"),
        "{argv}"
    );
    assert!(argv.contains("resolve and execute `scalars-k7`"), "{argv}");
}

fn assert_bare_grove_migrates_and_launches_a_jj_legacy_tree(colocate: bool) {
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
    fs::write(grove.join("BRIEF.md"), "# native-jj — brief\n").unwrap();
    fs::write(
        grove.join("01-legacy-k1.md"),
        "# legacy-k1\n\n**Kind:** impl\n\n## Goal\nLaunch after migration.\n",
    )
    .unwrap();

    let cwd_log = fixture.path().join("jj-cwd.log");
    let fake = fixture.path().join("record-jj-cwd.sh");
    write_executable(
        &fake,
        &format!("#!/bin/sh\npwd -P > {}\n", shell_quote(&cwd_log)),
    );
    write_complete_config(&home, &format!("{} '${{prompt}}'", shell_quote(&fake)));

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
    assert_eq!(
        fs::read_to_string(grove.join("FORMAT")).unwrap(),
        "session-kinds-v1\n"
    );
    assert!(!grove.join("01-legacy-k1.md").exists());
    let migrated = grove.join("01-impl-legacy-k1.md");
    assert!(migrated.is_file());
    assert!(!fs::read_to_string(migrated).unwrap().contains("**Kind:**"));
}

#[test]
fn bare_grove_migrates_and_launches_a_native_jj_legacy_tree() {
    assert_bare_grove_migrates_and_launches_a_jj_legacy_tree(false);
}

#[test]
fn bare_grove_migrates_and_launches_a_colocated_jj_legacy_tree() {
    assert_bare_grove_migrates_and_launches_a_jj_legacy_tree(true);
}

#[test]
fn invalid_config_cannot_create_a_fresh_grove() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.kdl"), "impl \"runner ${prompt}\"\n").unwrap();
    let worktree = fixture.path().join("rootless");
    init_git_worktree(&worktree);

    let output = run_grove(&home, &worktree);

    assert!(!output.status.success());
    assert!(!worktree.join(".grove").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing session kinds"), "{stderr}");
}

#[test]
fn invalid_config_leaves_legacy_current_empty_and_pending_trees_byte_identical() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    fs::write(config_dir.join("config.kdl"), "impl \"runner ${prompt}\"\n").unwrap();

    for state in ["legacy", "current", "empty", "pending"] {
        let worktree = fixture.path().join(format!("{state}-worktree"));
        init_git_worktree(&worktree);
        let grove = worktree.join(".grove");
        fs::create_dir_all(&grove).unwrap();
        fs::write(grove.join("BRIEF.md"), format!("# {state} — brief\n")).unwrap();
        match state {
            "legacy" => {
                fs::write(grove.join("01-task-k1.md"), "# task-k1\n\n**Kind:** impl\n").unwrap()
            }
            "current" => {
                fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
                fs::write(grove.join("01-impl-task-k1.md"), "# task-k1\n").unwrap();
            }
            "empty" => {
                fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
                fs::write(grove.join("01-DONE-impl-task-k1.md"), "# task-k1\n").unwrap();
            }
            "pending" => {
                fs::write(grove.join("01-task-k1.md"), "# task-k1\n\n**Kind:** impl\n").unwrap();
                let witness = grove.join("MIGRATING-session-kinds");
                fs::create_dir(&witness).unwrap();
                fs::write(witness.join("partial"), "must remain\n").unwrap();
            }
            _ => unreachable!(),
        }
        let before = tree_snapshot(&grove);

        let output = run_grove(&home, &worktree);

        assert!(
            !output.status.success(),
            "state {state} unexpectedly launched"
        );
        assert_eq!(tree_snapshot(&grove), before, "state {state} was mutated");
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            stderr.contains("missing session kinds"),
            "{state}: {stderr}"
        );
    }
}

#[test]
fn fresh_grove_creates_and_launches_the_requirements_leaf() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("fresh worktree");
    init_git_worktree(&worktree);
    let log = fixture.path().join("fresh.log");
    let fake = fixture.path().join("fresh-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
exit 0
"#,
    );
    let template = format!("{} {} '${{prompt}}'", shell_quote(&fake), shell_quote(&log));
    write_complete_config(&home, &template);

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(worktree.join(".grove/FORMAT")).unwrap(),
        "session-kinds-v1\n"
    );
    assert!(worktree.join(".grove/01-requirements-plan-k1.md").is_file());
    let prompt = fs::read_to_string(log).unwrap();
    assert!(prompt.contains("resolve and execute `plan-k1`"), "{prompt}");
}

#[test]
fn partial_fresh_scaffold_recovers_before_selection_and_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("partial-scaffold");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    grove::tree_lifecycle::root_init(&worktree, "custom-plan").unwrap();
    fs::remove_file(grove.join("FORMAT")).unwrap();
    let prompt_log = fixture.path().join("partial.log");
    let fake = fixture.path().join("partial-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
exit 0
"#,
    );
    write_complete_config(
        &home,
        &format!(
            "{} {} '${{prompt}}'",
            shell_quote(&fake),
            shell_quote(&prompt_log)
        ),
    );

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(grove.join("FORMAT")).unwrap(),
        "session-kinds-v1\n"
    );
    assert!(fs::read_to_string(prompt_log)
        .unwrap()
        .contains("resolve and execute `custom-plan-k1`"));
}

#[test]
fn relaunch_reloads_config_and_uses_the_new_filename_kind() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("reload-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# reload — brief\n").unwrap();
    fs::write(grove.join("01-impl-first-k1.md"), "# first-k1\n").unwrap();

    let log = fixture.path().join("reload.log");
    let next_config = fixture.path().join("next-config.kdl");
    let active_config = home.join(".config/grove/config.kdl");
    let fake = fixture.path().join("reload-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
log=$1
marker=$2
next_config=$3
active_config=$4
prompt=$5
printf '%s|%s\n' "$marker" "$prompt" >> "$log"
if [ "$marker" = first-template ]; then
  mv .grove/01-impl-first-k1.md .grove/01-DONE-impl-first-k1.md
  printf '# second-k2\n' > .grove/02-design-second-k2.md
  cp "$next_config" "$active_config"
  printf 'relaunch\n' > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );
    let first_template = format!(
        "{} {} first-template {} {} '${{prompt}}'",
        shell_quote(&fake),
        shell_quote(&log),
        shell_quote(&next_config),
        shell_quote(&active_config)
    );
    let second_template = format!(
        "{} {} second-template {} {} '${{prompt}}'",
        shell_quote(&fake),
        shell_quote(&log),
        shell_quote(&next_config),
        shell_quote(&active_config)
    );
    write_complete_config(&home, &first_template);
    let next_document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {second_template:?}\n"))
        .collect::<String>();
    fs::write(&next_config, next_document).unwrap();

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let rows = fs::read_to_string(log).unwrap();
    assert!(rows.contains("first-template|"), "{rows}");
    assert!(rows.contains("resolve and execute `first-k1`"), "{rows}");
    assert!(rows.contains("second-template|"), "{rows}");
    assert!(rows.contains("resolve and execute `second-k2`"), "{rows}");
}

#[test]
fn insertion_during_launch_does_not_change_the_session_mandate() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("insert-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# insertion — brief\n").unwrap();
    fs::write(grove.join("02-impl-selected-k7.md"), "# selected-k7\n").unwrap();

    let prompt_log = fixture.path().join("insert.log");
    let fake = fixture.path().join("insert-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
printf '# inserted-k8\n' > .grove/01-design-inserted-k8.md
exit 0
"#,
    );
    let template = format!(
        "{} {} '${{prompt}}'",
        shell_quote(&fake),
        shell_quote(&prompt_log)
    );
    write_complete_config(&home, &template);

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let prompt = fs::read_to_string(prompt_log).unwrap();
    assert!(
        prompt.contains("resolve and execute `selected-k7`"),
        "{prompt}"
    );
    assert!(!prompt.contains("inserted-k8"), "{prompt}");
    assert!(grove.join("01-design-inserted-k8.md").is_file());
}

#[test]
fn spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("spawn-failure-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# failure — brief\n").unwrap();
    let leaf = grove.join("01-impl-still-live-k9.md");
    fs::write(&leaf, "# still-live-k9\n").unwrap();
    let missing = fixture.path().join("missing configured executable");
    let template = format!("{} '${{prompt}}'", shell_quote(&missing));
    write_complete_config(&home, &template);

    let output = run_grove(&home, &worktree);

    assert!(!output.status.success());
    assert!(leaf.is_file());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("kind `impl`"), "{stderr}");
    assert!(stderr.contains(missing.to_str().unwrap()), "{stderr}");
    assert!(
        stderr.contains(home.join(".config/grove/config.kdl").to_str().unwrap()),
        "{stderr}"
    );
    let leaked_signal_channels = fs::read_dir(worktree.join(".git/grove"))
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name())
        .filter(|name| name.to_string_lossy().starts_with("signal-"))
        .collect::<Vec<_>>();
    assert!(
        leaked_signal_channels.is_empty(),
        "spawn failure leaked signal channels: {leaked_signal_channels:?}"
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
    write_complete_config(&home, &format!("{} '${{prompt}}'", shell_quote(&restart)));

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
fn nonsignalled_nonzero_exit_reports_status_elapsed_and_launch_identity() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("nonzero-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# nonzero — brief\n").unwrap();
    fs::write(grove.join("01-design-crashing-k4.md"), "# crashing-k4\n").unwrap();
    let fake = fixture.path().join("exit-23.sh");
    write_executable(&fake, "#!/bin/sh\nexit 23\n");
    let template = format!("{} '${{prompt}}'", shell_quote(&fake));
    write_complete_config(&home, &template);

    let output = run_grove(&home, &worktree);

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("status exit status: 23"), "{stderr}");
    assert!(stderr.contains("elapsed "), "{stderr}");
    assert!(stderr.contains("kind `design`"), "{stderr}");
    assert!(stderr.contains(fake.to_str().unwrap()), "{stderr}");
    assert!(
        stderr.contains(home.join(".config/grove/config.kdl").to_str().unwrap()),
        "{stderr}"
    );
}

#[test]
fn sibling_grove_llm_takes_precedence_over_path() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("sibling-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# sibling — brief\n").unwrap();
    fs::write(grove.join("01-impl-sibling-k3.md"), "# sibling-k3\n").unwrap();
    let launched = fixture.path().join("launched");
    let configured = fixture.path().join("configured.sh");
    write_executable(
        &configured,
        &format!("#!/bin/sh\nprintf launched > {}\n", shell_quote(&launched)),
    );
    write_complete_config(
        &home,
        &format!("{} '${{prompt}}'", shell_quote(&configured)),
    );
    let fake_bin = fixture.path().join("fake-bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let path_marker = fixture.path().join("path-helper-ran");
    write_executable(
        &fake_bin.join("grove-llm"),
        &format!(
            "#!/bin/sh\nprintf wrong > {}\nexit 99\n",
            shell_quote(&path_marker)
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", path_with_front(&fake_bin))
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(launched.is_file());
    assert!(!path_marker.exists());
}

#[test]
fn version_skew_from_path_fails_before_tree_creation() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("skew-worktree");
    init_git_worktree(&worktree);
    let isolated_bin = fixture.path().join("isolated-bin");
    fs::create_dir_all(&isolated_bin).unwrap();
    let copied_grove = isolated_bin.join("grove");
    fs::copy(env!("CARGO_BIN_EXE_grove"), &copied_grove).unwrap();
    let fake_bin = fixture.path().join("fake-path");
    fs::create_dir_all(&fake_bin).unwrap();
    write_executable(
        &fake_bin.join("grove-llm"),
        "#!/bin/sh\nprintf 'grove-llm 0.0.0\\n'\n",
    );

    let output = Command::new(copied_grove)
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", path_with_front(&fake_bin))
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(!worktree.join(".grove").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("version skew"), "{stderr}");
    assert!(stderr.contains("0.0.0"), "{stderr}");
}

#[test]
fn malformed_grove_llm_version_output_fails_before_tree_creation() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "'/definitely/missing' '${prompt}'");
    let worktree = fixture.path().join("malformed-helper-worktree");
    init_git_worktree(&worktree);
    let isolated_bin = fixture.path().join("isolated-bin");
    fs::create_dir_all(&isolated_bin).unwrap();
    let copied_grove = isolated_bin.join("grove");
    fs::copy(env!("CARGO_BIN_EXE_grove"), &copied_grove).unwrap();
    let fake_bin = fixture.path().join("fake-path");
    fs::create_dir_all(&fake_bin).unwrap();
    write_executable(
        &fake_bin.join("grove-llm"),
        &format!(
            "#!/bin/sh\nprintf 'not-grove-llm {}\\n'\n",
            env!("CARGO_PKG_VERSION")
        ),
    );

    let output = Command::new(copied_grove)
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", path_with_front(&fake_bin))
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(!worktree.join(".grove").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognised"), "{stderr}");
    assert!(stderr.contains("not-grove-llm"), "{stderr}");
}

#[test]
fn missing_grove_llm_fails_before_tree_creation() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    write_complete_config(&home, "'/definitely/missing' '${prompt}'");
    let worktree = fixture.path().join("missing-helper-worktree");
    init_git_worktree(&worktree);
    let isolated_bin = fixture.path().join("isolated-bin");
    fs::create_dir_all(&isolated_bin).unwrap();
    let copied_grove = isolated_bin.join("grove");
    fs::copy(env!("CARGO_BIN_EXE_grove"), &copied_grove).unwrap();
    let empty_path = fixture.path().join("empty-path");
    fs::create_dir(&empty_path).unwrap();
    let git = std::env::split_paths(&std::env::var_os("PATH").unwrap_or_default())
        .map(|directory| directory.join("git"))
        .find(|candidate| candidate.is_file())
        .expect("test runner PATH has no git executable");
    std::os::unix::fs::symlink(git, empty_path.join("git")).unwrap();

    let output = Command::new(copied_grove)
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", &empty_path)
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(!worktree.join(".grove").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("could not run `grove-llm --version`"),
        "{stderr}"
    );
}

#[test]
fn legacy_tree_is_migrated_committed_and_launched_by_filename_kind() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("legacy-worktree");
    init_git_worktree(&worktree);
    for (key, value) in [
        ("user.email", "test@example.com"),
        ("user.name", "Test User"),
    ] {
        assert!(Command::new("git")
            .args(["config", key, value])
            .current_dir(&worktree)
            .status()
            .unwrap()
            .success());
    }
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# legacy-worktree — brief\n").unwrap();
    fs::write(
        grove.join("01-legacy-task-k1.md"),
        "# legacy-task-k1\n\n**Kind:** prototype\n\n## Goal\nMigrate.\n",
    )
    .unwrap();
    let prompt_log = fixture.path().join("legacy-prompt.log");
    let fake = fixture.path().join("legacy-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
printf '%s' "$2" > "$1"
exit 0
"#,
    );
    write_complete_config(
        &home,
        &format!(
            "{} {} '${{prompt}}'",
            shell_quote(&fake),
            shell_quote(&prompt_log)
        ),
    );

    let output = run_grove(&home, &worktree);

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let migrated = grove.join("01-prototype-legacy-task-k1.md");
    assert!(migrated.is_file());
    assert!(!grove.join("01-legacy-task-k1.md").exists());
    assert!(!fs::read_to_string(migrated).unwrap().contains("**Kind:**"));
    assert_eq!(
        fs::read_to_string(grove.join("FORMAT")).unwrap(),
        "session-kinds-v1\n"
    );
    assert!(fs::read_to_string(prompt_log)
        .unwrap()
        .contains("resolve and execute `legacy-task-k1`"));
    let subject = Command::new("git")
        .args(["log", "-1", "--format=%s"])
        .current_dir(&worktree)
        .output()
        .unwrap();
    assert!(subject.status.success());
    assert_eq!(
        String::from_utf8(subject.stdout).unwrap(),
        "grove(legacy-worktree): migrate task tree to session-kind filenames\n"
    );
}

#[test]
fn config_is_reloaded_after_a_completed_legacy_transition_before_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("reload-after-transition");
    init_git_worktree(&worktree);
    for (key, value) in [
        ("user.email", "test@example.com"),
        ("user.name", "Test User"),
    ] {
        assert!(Command::new("git")
            .args(["config", key, value])
            .current_dir(&worktree)
            .status()
            .unwrap()
            .success());
    }
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(
        grove.join("BRIEF.md"),
        "# reload-after-transition — brief\n",
    )
    .unwrap();
    fs::write(grove.join("01-task-k1.md"), "# task-k1\n\n**Kind:** impl\n").unwrap();
    let launch_marker = fixture.path().join("must-not-launch");
    let configured = fixture.path().join("configured.sh");
    write_executable(
        &configured,
        &format!(
            "#!/bin/sh\nprintf launched > {}\n",
            shell_quote(&launch_marker)
        ),
    );
    write_complete_config(
        &home,
        &format!("{} '${{prompt}}'", shell_quote(&configured)),
    );
    let invalid = fixture.path().join("invalid-after-transition.kdl");
    fs::write(&invalid, "impl \"runner ${prompt}\"\n").unwrap();
    let active = home.join(".config/grove/config.kdl");
    let hook = worktree.join(".git/hooks/post-commit");
    write_executable(
        &hook,
        &format!(
            "#!/bin/sh\ncp {} {}\n",
            shell_quote(&invalid),
            shell_quote(&active)
        ),
    );

    let output = run_grove(&home, &worktree);

    assert!(!output.status.success());
    assert!(grove.join("01-impl-task-k1.md").is_file());
    assert!(!launch_marker.exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing session kinds"), "{stderr}");
}

#[test]
fn empty_current_tree_stops_without_launch_and_can_resume_later() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("empty-current");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# empty-current — brief\n").unwrap();
    fs::write(grove.join("01-DONE-impl-finished-k1.md"), "# finished-k1\n").unwrap();
    let launch_log = fixture.path().join("resume.log");
    let configured = fixture.path().join("resume-command.sh");
    write_executable(
        &configured,
        r#"#!/bin/sh
printf launched > "$1"
exit 0
"#,
    );
    write_complete_config(
        &home,
        &format!(
            "{} {} '${{prompt}}'",
            shell_quote(&configured),
            shell_quote(&launch_log)
        ),
    );

    let empty_output = run_grove(&home, &worktree);

    assert!(empty_output.status.success());
    assert!(!launch_log.exists());
    assert!(
        String::from_utf8_lossy(&empty_output.stderr).contains("no live leaves"),
        "{}",
        String::from_utf8_lossy(&empty_output.stderr)
    );

    fs::write(grove.join("02-design-resumed-k2.md"), "# resumed-k2\n").unwrap();
    let resumed_output = run_grove(&home, &worktree);

    assert!(
        resumed_output.status.success(),
        "{}",
        String::from_utf8_lossy(&resumed_output.stderr)
    );
    assert!(launch_log.is_file());
}

#[test]
fn cli_metadata_exposes_only_the_bare_entrypoint_without_provisioning() {
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
    assert!(!help.contains("Commands:"), "{help}");
    assert!(!help.contains("grove do"), "{help}");
    assert!(version.status.success());
    assert_eq!(
        String::from_utf8(version.stdout).unwrap(),
        format!("grove {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(!obsolete.status.success());
    assert!(!home.join(".codex/skills/grove").exists());
}

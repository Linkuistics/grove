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

fn configure_git_identity(worktree: &Path) {
    run_command(
        "git",
        worktree,
        &["config", "user.email", "test@example.com"],
    );
    run_command("git", worktree, &["config", "user.name", "Test User"]);
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

/// The driver-authored sentence naming the leaf selected for one session.
///
/// It has **one home in this binary** for the same reason it has one home in the
/// driver: it is the whole of what `${prompt}` says about the selected leaf — a
/// value, with every normative consequence of it left to the skill
/// (`docs/adr/skill-delivers-the-methodology.md`).
fn mandate_naming(handle: &str) -> String {
    format!("Grove mandate: the leaf selected for this session is `{handle}`")
}

fn run_grove(home: &Path, worktree: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(worktree)
        .env("HOME", home)
        .output()
        .unwrap()
}

/// The real `git`, resolved before a stand-in of the same name goes on PATH.
fn resolve_real_git() -> std::path::PathBuf {
    let output = Command::new("sh")
        .args(["-c", "command -v git"])
        .output()
        .unwrap();
    assert!(output.status.success(), "git is not on PATH");
    std::path::PathBuf::from(String::from_utf8(output.stdout).unwrap().trim())
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
        log.contains("arg=<**Load the `grove` skill now, and read its `references/impl.md`.**"),
        "{log:?}"
    );
    assert!(log.contains(&mandate_naming("selected-work-k7")), "{log:?}");
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
    assert!(argv.contains(&mandate_naming("scalars-k7")), "{argv}");
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

// The bare path acquires the workspace lease *before* it reads configuration or
// touches the tree, so a control directory it cannot create has to be reported
// as itself rather than surfacing as whatever the next step would have
// complained about. Stated black-box and adversarially: the config is missing
// **and** the tree is legacy, so both later steps have a loud failure ready —
// the run must still name the control directory, and migrate nothing.
#[test]
fn an_unwritable_control_directory_fails_before_configuration_or_tree_access() {
    let fixture = TempDir::new().unwrap();
    // No `~/.config/grove/config.kdl` at all: reaching configuration would name
    // every one of the nineteen kinds instead.
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# unwritable — brief\n").unwrap();
    fs::write(grove.join("01-task-k1.md"), "# task-k1\n\n**Kind:** impl\n").unwrap();
    let before = tree_snapshot(&grove);

    let git_directory = worktree.join(".git");
    fs::set_permissions(&git_directory, fs::Permissions::from_mode(0o500)).unwrap();
    let output = run_grove(&home, &worktree);
    fs::set_permissions(&git_directory, fs::Permissions::from_mode(0o700)).unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(
        stderr.contains("creating Grove control directory"),
        "the failure must name the control directory it could not create: {stderr}"
    );
    assert!(
        !stderr.contains("config.kdl") && !stderr.contains("missing session kinds"),
        "configuration must not have been reached: {stderr}"
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
    assert!(prompt.contains(&mandate_naming("plan-k1")), "{prompt}");
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
        .contains(&mandate_naming("custom-plan-k1")));
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
    assert!(rows.contains(&mandate_naming("first-k1")), "{rows}");
    assert!(rows.contains("second-template|"), "{rows}");
    assert!(rows.contains(&mandate_naming("second-k2")), "{rows}");
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
    assert!(prompt.contains(&mandate_naming("selected-k7")), "{prompt}");
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

/// One bare-`grove` run whose session-resolved `grove-llm` the fixture controls.
struct PairingRun {
    _fixture: TempDir,
    output: Output,
    worktree: std::path::PathBuf,
    launched: std::path::PathBuf,
    resolved: std::path::PathBuf,
}

impl PairingRun {
    fn stderr(&self) -> String {
        String::from_utf8_lossy(&self.output.stderr).into_owned()
    }

    /// Both halves of "report, never gate", asserted together because either
    /// alone is satisfiable by the behaviour this leaf removed: the old guard
    /// printed a diagnostic *and* refused, and a check that quietly passed would
    /// launch without one.
    fn assert_reported_and_launched(&self, expected: &[&str]) {
        let stderr = self.stderr();
        for fragment in expected {
            assert!(
                stderr.contains(fragment),
                "missing {fragment:?} in {stderr}"
            );
        }
        assert!(
            self.output.status.success(),
            "the pairing report must not fail the run: {stderr}"
        );
        assert!(
            self.launched.is_file(),
            "the configured session must still launch: {stderr}"
        );
        assert!(
            self.worktree.join(".grove").is_dir(),
            "the lifecycle transition must still run: {stderr}"
        );
    }
}

/// Drive bare `grove` once against an **isolated** `PATH` holding only `git` and
/// — when `grove_llm` is `Some` — a stand-in agent CLI at `fake-path/grove-llm`.
///
/// Isolated rather than merely front-loaded, because one of the cases below is
/// *no* `grove-llm` anywhere and a developer's own `PATH` would quietly satisfy
/// it. The configured command records that it ran and exits without signalling,
/// so the loop reports one no-signal stop and ends after a single iteration —
/// which is what lets every case here assert the launch as well as the report.
fn drive_with_resolved_grove_llm(label: &str, grove_llm: Option<&str>) -> PairingRun {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join(format!("{label}-worktree"));
    init_git_worktree(&worktree);

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

    // Only `git` — the lifecycle transition shells out to it — plus whatever
    // agent CLI this case wants found.
    let isolated_path = fixture.path().join("isolated-path");
    fs::create_dir(&isolated_path).unwrap();
    std::os::unix::fs::symlink(resolve_real_git(), isolated_path.join("git")).unwrap();
    let resolved = isolated_path.join("grove-llm");
    if let Some(body) = grove_llm {
        write_executable(&resolved, body);
    }

    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", &isolated_path)
        .output()
        .unwrap();

    PairingRun {
        _fixture: fixture,
        output,
        worktree,
        launched,
        resolved,
    }
}

/// A digest that is well-formed and is not this build's, so the driver has to
/// classify it as a *mismatch* rather than as unidentifiable.
const FOREIGN_IDENTITY: &str = "00112233445566778899aabbccddeeff00112233445566778899aabbccddeeff";

#[test]
fn a_mismatched_grove_llm_is_reported_and_the_launch_proceeds() {
    let run = drive_with_resolved_grove_llm(
        "mismatch",
        Some(&format!("#!/bin/sh\nprintf '{FOREIGN_IDENTITY}\\n'\n")),
    );

    run.assert_reported_and_launched(&[
        "build pairing mismatch",
        FOREIGN_IDENTITY,
        run.resolved.to_str().unwrap(),
        "PATH resolves first",
    ]);
    // The requirement, not a command: `cargo install --path .` is the remedy
    // only where `~/.cargo/bin` outranks every other prefix holding a
    // `grove-llm`, and where it does not, that install is already done and still
    // is not what a session reaches.
    assert!(
        !run.stderr().contains("cargo install"),
        "the diagnostic must state the requirement, not prescribe one command: {}",
        run.stderr()
    );
}

#[test]
fn an_unidentifiable_grove_llm_is_reported_and_the_launch_proceeds() {
    // A binary predating the flag: clap rejects the unknown argument and exits
    // non-zero. Unidentifiable, never mismatched — refusing here would make the
    // pair unupgradable from inside the loop.
    let too_old = drive_with_resolved_grove_llm(
        "too-old",
        Some("#!/bin/sh\necho 'error: unexpected argument' >&2\nexit 2\n"),
    );
    too_old.assert_reported_and_launched(&[
        "could not name its methodology",
        too_old.resolved.to_str().unwrap(),
        // The three knowable fields of this branch: a resolved path, this
        // build's identity, and why no answer came. There is deliberately no
        // fourth — a binary that could not name its methodology has none to
        // print (one-build-owns-a-session).
        grove::methodology::identity(),
    ]);

    // ...and one that answers, but with something that is not a digest. Free
    // text must never be *compared*, or a correctly paired machine gets told it
    // is mismatched.
    let garbled =
        drive_with_resolved_grove_llm("garbled", Some("#!/bin/sh\nprintf 'not-a-hash\\n'\n"));
    garbled.assert_reported_and_launched(&[
        "could not name its methodology",
        "not-a-hash",
        garbled.resolved.to_str().unwrap(),
    ]);
}

#[test]
fn a_missing_grove_llm_is_reported_and_the_launch_proceeds() {
    // The property deliberately given up here: this used to be a hard stop
    // before `.grove/` existed. The driver never invokes `grove-llm`, and a
    // container or `ssh` target that supplies its own is a supported shape in
    // which the driver's `PATH` is simply not the one that matters.
    let run = drive_with_resolved_grove_llm("missing", None);

    run.assert_reported_and_launched(&[
        "no `grove-llm` on this driver's PATH",
        "PATH resolves first",
        // Nothing resolved, so this branch has neither a path nor a peer
        // identity to name — and it is still actionable, because it names this
        // build's identity, the search performed, and the requirement. The
        // durable record promises exactly these and no more.
        grove::methodology::identity(),
    ]);
}

#[test]
fn the_checked_grove_llm_is_the_one_on_path_not_the_drivers_own_sibling() {
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

    // `grove` beside a `grove-llm` that agrees with it — the shape `cargo run`
    // produces, and the shape that made the motivating case invisible while the
    // sibling was preferred.
    let isolated_bin = fixture.path().join("isolated-bin");
    fs::create_dir_all(&isolated_bin).unwrap();
    let copied_grove = isolated_bin.join("grove");
    fs::copy(env!("CARGO_BIN_EXE_grove"), &copied_grove).unwrap();
    let sibling_marker = fixture.path().join("sibling-ran");
    write_executable(
        &isolated_bin.join("grove-llm"),
        &format!(
            "#!/bin/sh\nprintf ran > {}\nexec {} \"$@\"\n",
            shell_quote(&sibling_marker),
            shell_quote(Path::new(env!("CARGO_BIN_EXE_grove-llm"))),
        ),
    );
    let fake_bin = fixture.path().join("fake-bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let path_marker = fixture.path().join("path-helper-ran");
    let path_grove_llm = fake_bin.join("grove-llm");
    write_executable(
        &path_grove_llm,
        &format!(
            "#!/bin/sh\nprintf ran > {}\nprintf '{FOREIGN_IDENTITY}\\n'\n",
            shell_quote(&path_marker),
        ),
    );

    let output = Command::new(copied_grove)
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", path_with_front(&fake_bin))
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(launched.is_file(), "{stderr}");
    assert!(
        path_marker.exists(),
        "the `PATH` binary is the one a session resolves, so it is the one asked: {stderr}"
    );
    assert!(
        !sibling_marker.exists(),
        "the driver's own sibling agrees with it by construction and must not be \
         what gets checked: {stderr}"
    );
    assert!(
        stderr.contains(path_grove_llm.to_str().unwrap()),
        "the diagnostic must name the path it actually resolved: {stderr}"
    );
}

/// Bare `grove` is deliberately accepted from any directory inside the working
/// tree, but the configured session is spawned at the **root**. So a relative or
/// empty `PATH` entry — `PATH=:/usr/bin`, as every POSIX shell reads it — names
/// a different directory for the driver than for the session, and resolving it
/// in the driver's own cwd would report on a `grove-llm` no session can reach
/// while executing an unrelated repository-local helper to do it. The case is
/// exactly the one the docs claim is reliable: an inherited environment.
#[test]
fn a_relative_path_entry_resolves_from_the_worktree_the_session_is_spawned_in() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("nested-path-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# nested-path — brief\n").unwrap();
    fs::write(
        grove.join("01-impl-nested-path-k2.md"),
        "# nested-path-k2\n",
    )
    .unwrap();

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

    // Two same-named helpers, one at each end of the disagreement: the session's
    // cwd is the worktree root, the driver's is the directory `grove` was typed
    // in. Each records that it ran, so the assertion is which one was executed
    // and not merely which one was named.
    let root_marker = fixture.path().join("root-helper-ran");
    write_executable(
        &worktree.join("grove-llm"),
        &format!(
            "#!/bin/sh\nprintf ran > {}\nprintf '{FOREIGN_IDENTITY}\\n'\n",
            shell_quote(&root_marker),
        ),
    );
    let invocation_dir = worktree.join("subdir");
    fs::create_dir_all(&invocation_dir).unwrap();
    let nested_marker = fixture.path().join("nested-helper-ran");
    write_executable(
        &invocation_dir.join("grove-llm"),
        &format!(
            "#!/bin/sh\nprintf ran > {}\nprintf '{FOREIGN_IDENTITY}\\n'\n",
            shell_quote(&nested_marker),
        ),
    );

    // Only `git`, behind an empty leading entry — "the current directory", whose
    // whole point here is that the driver's and the session's differ.
    let isolated_path = fixture.path().join("isolated-path");
    fs::create_dir(&isolated_path).unwrap();
    std::os::unix::fs::symlink(resolve_real_git(), isolated_path.join("git")).unwrap();
    let search = OsString::from(format!(":{}", isolated_path.display()));

    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(&invocation_dir)
        .env("HOME", &home)
        .env("PATH", &search)
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(
        launched.is_file(),
        "the session must still launch: {stderr}"
    );
    assert!(
        root_marker.exists(),
        "the worktree-root helper is the one a session resolves, so it is the \
         one asked: {stderr}"
    );
    assert!(
        !nested_marker.exists(),
        "the helper under the driver's own cwd is unreachable by the session and \
         must never be run: {stderr}"
    );
    let reported = worktree.canonicalize().unwrap().join("grove-llm");
    assert!(
        stderr.contains(reported.to_str().unwrap()),
        "the diagnostic must name the path a session would resolve, got: {stderr}"
    );
}

/// The control that makes the three reports above mean something: a `PATH`
/// binary that *is* this build says nothing at all.
#[test]
fn a_paired_grove_llm_is_reported_as_nothing() {
    let run = drive_with_resolved_grove_llm(
        "paired",
        Some(&format!(
            "#!/bin/sh\nexec {} \"$@\"\n",
            shell_quote(Path::new(env!("CARGO_BIN_EXE_grove-llm")))
        )),
    );

    let stderr = run.stderr();
    assert!(run.output.status.success(), "{stderr}");
    assert!(run.launched.is_file(), "{stderr}");
    for absent in [
        "build pairing mismatch",
        "could not name its methodology",
        "no `grove-llm` on this driver's PATH",
    ] {
        assert!(
            !stderr.contains(absent),
            "a matching pair must be silent, got {absent:?} in {stderr}"
        );
    }
}

/// A skill directory is global while the driver lease is per working tree, so
/// another build can take one *while a loop runs*. The loop therefore re-verifies
/// each stamp before every launch and restores its own embed — the one artifact
/// Grove owns, and so the one place the pairing is repaired rather than reported
/// (one-build-owns-a-session).
///
/// The clobber has to land **between** iterations to be the case under test: the
/// start-of-run sweep already repairs anything that was wrong before `grove`
/// started, so a pre-broken directory would prove nothing about the loop.
#[test]
fn a_skill_directory_clobbered_mid_loop_is_restored_before_the_next_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("clobber-worktree");
    init_git_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# clobber — brief\n").unwrap();
    fs::write(grove.join("01-impl-first-k1.md"), "# first-k1\n").unwrap();
    fs::write(grove.join("02-impl-second-k2.md"), "# second-k2\n").unwrap();

    let skill_dir = home.join(".codex/skills/grove");
    let fake = fixture.path().join("clobber-command.sh");
    write_executable(
        &fake,
        r#"#!/bin/sh
skill_dir=$1
if [ ! -f .grove/01-DONE-impl-first-k1.md ]; then
  mv .grove/01-impl-first-k1.md .grove/01-DONE-impl-first-k1.md
  # Stand in for another build writing this shared directory.
  rm -f "$skill_dir/SKILL.md"
  printf 'another-build\n' > "$skill_dir/.grove-content-hash"
  printf 'relaunch\n' > "$GROVE_SIGNAL_FILE"
fi
exit 0
"#,
    );
    write_complete_config(
        &home,
        &format!(
            "{} {} '${{prompt}}'",
            shell_quote(&fake),
            shell_quote(&skill_dir)
        ),
    );

    let output = run_grove(&home, &worktree);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains(skill_dir.to_str().unwrap()),
        "the restore must name the directory it took back: {stderr}"
    );
    // **Exactly once**, across two iterations. The count is the assertion that
    // this is a re-*verification*: the first iteration ran against a directory
    // the start-of-run sweep had just written, and a stamp comparison that
    // disagreed with the writer's — a stray newline, a trimmed read — would
    // re-extract identical bytes every iteration for the whole life of a driver.
    assert_eq!(
        stderr.matches("restored the codex skill at").count(),
        1,
        "a warm stamp must be a no-op; only the clobbered iteration restores: {stderr}"
    );
    assert!(
        skill_dir.join("SKILL.md").is_file(),
        "this driver's embed must be back on disk: {stderr}"
    );
    assert_ne!(
        fs::read_to_string(skill_dir.join(".grove-content-hash")).unwrap(),
        "another-build\n",
        "the stamp must name this build again"
    );
}

#[test]
fn legacy_tree_is_migrated_committed_and_launched_by_filename_kind() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("legacy-worktree");
    init_git_worktree(&worktree);
    configure_git_identity(&worktree);
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
        .contains(&mandate_naming("legacy-task-k1")));
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

/// Everything one adopted pre-v2 tree must show after a single bare `grove`
/// invocation. The two grammars below differ only in how they are seeded, so the
/// assertions they share live here rather than being written twice.
struct AdoptedTree {
    /// Relative paths under `.grove/`, sorted — directories included, so a node
    /// that failed to materialize is visible rather than implied by its child.
    entries: Vec<String>,
    /// Task-file bodies by relative path, for the body-marker sweep.
    bodies: Vec<(String, String)>,
    /// The `.grove/FORMAT` witness's contents, absent if it was never written.
    format: Option<String>,
    /// Every commit subject added by the run, oldest first.
    new_subjects: Vec<String>,
    /// The full mandate prompt the configured command received.
    mandate: String,
    /// The session kind whose *configured template* was selected and executed,
    /// reported by the command itself rather than inferred from the driver's
    /// announcement — the two are separate readers of the selected leaf.
    executed_kind: String,
    /// The driver's own announcement, which names the routed session kind.
    stderr: String,
}

/// Drive the real bare `grove` process over an already-seeded legacy `worktree`
/// and collect what the adoption produced.
///
/// The configured command writes its mandate and exits without signalling, so
/// the loop stops after exactly one launch — one adoption, one routed session,
/// nothing racing a second iteration.
///
/// Unlike [`write_complete_config`], every kind gets a *distinguishable*
/// template: each carries its own kind as a literal argument, which the command
/// reports back. That is what makes the configuration lookup observable at all.
/// With one template shared across all nineteen kinds, a driver that looked up
/// the wrong kind would execute a byte-identical command and no assertion here
/// could tell — the mutation that established this is why the fixture is shaped
/// this way.
fn adopt_with_bare_grove(fixture: &Path, home: &Path, worktree: &Path) -> AdoptedTree {
    let grove = worktree.join(".grove");
    let seeded_subjects = git_subjects(worktree);

    let mandate_log = fixture.join("adopted-mandate.log");
    let configured = fixture.join("adopted-command.sh");
    write_executable(
        &configured,
        r#"#!/bin/sh
printf '%s\n%s' "$2" "$3" > "$1"
exit 0
"#,
    );
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let document = SESSION_KINDS
        .iter()
        .map(|kind| {
            let template = format!(
                "{} {} '{kind}' '${{prompt}}'",
                shell_quote(&configured),
                shell_quote(&mandate_log)
            );
            format!("{kind} {template:?}\n")
        })
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();

    let output = run_grove(home, worktree);
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "{stderr}");

    let mut entries = Vec::new();
    let mut bodies = Vec::new();
    let mut format = None;
    for (relative, contents) in tree_snapshot(&grove) {
        if let Some(contents) = contents {
            if relative.ends_with(".md") {
                bodies.push((
                    relative.clone(),
                    String::from_utf8(contents).expect("a task file is UTF-8"),
                ));
            } else if relative == "FORMAT" {
                format = Some(String::from_utf8(contents).expect("FORMAT is UTF-8"));
            }
        }
        entries.push(relative);
    }
    entries.sort();

    // `git log` is newest-first; the run's own commits are that prefix, reversed
    // back into the order they were made. Named rather than left to an unsigned
    // underflow, so a run that *lost* history says so.
    let subjects = git_subjects(worktree);
    let added = subjects
        .len()
        .checked_sub(seeded_subjects.len())
        .unwrap_or_else(|| panic!("adoption removed commits: {seeded_subjects:?} -> {subjects:?}"));
    let mut new_subjects = subjects[..added].to_vec();
    new_subjects.reverse();

    let logged = fs::read_to_string(&mandate_log).unwrap_or_default();
    let (executed_kind, mandate) = logged.split_once('\n').unwrap_or(("", ""));

    AdoptedTree {
        entries,
        bodies,
        format,
        new_subjects,
        mandate: mandate.to_string(),
        executed_kind: executed_kind.to_string(),
        stderr,
    }
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

impl AdoptedTree {
    /// The single migration commit this adoption is allowed to add. Naming the
    /// whole added-subject list rather than only `HEAD` is what makes this a
    /// *boundary* assertion: a transition that split its work across two commits,
    /// or swept the launched session's output into a second one, fails here.
    fn assert_one_migration_commit(&self, grove_name: &str) {
        assert_eq!(
            self.new_subjects,
            vec![format!(
                "grove({grove_name}): migrate task tree to session-kind filenames"
            )],
            "adoption must add exactly one migration commit"
        );
    }

    /// The current session-kind format, proven positively (the `FORMAT` witness)
    /// and negatively (no body routing markers, no transaction witness left
    /// behind by a fail-closed transition that completed).
    fn assert_reached_current_format(&self) {
        assert_eq!(
            self.format.as_deref(),
            Some("session-kinds-v1\n"),
            "the current-format witness must be written: {:?}",
            self.entries
        );
        assert!(
            !self
                .entries
                .iter()
                .any(|entry| entry.starts_with("MIGRATING-session-kinds")),
            "a completed transition must leave no migration witness: {:?}",
            self.entries
        );
        for (relative, body) in &self.bodies {
            for marker in ["**Kind:**", "**Harness:**", "**Producer launch:**"] {
                assert!(
                    !body.contains(marker),
                    "{relative} still carries the obsolete {marker} line"
                );
            }
        }
    }

    /// The first routed launch after adoption: the driver read the session kind
    /// strictly, from the *migrated* filename, and mandated the migrated handle.
    ///
    /// This is the assertion the tree shape alone cannot make. Renaming the files
    /// correctly and then refusing to route is exactly the failure this leaf was
    /// cut for, and it leaves a perfectly current-looking tree on disk.
    fn assert_routed_launch(&self, kind: &str, handle: &str) {
        assert_eq!(
            self.executed_kind, kind,
            "the first session after adoption must execute {kind}'s configured template"
        );
        assert!(
            self.stderr
                .contains(&format!("grove: launching {kind} with")),
            "the first session after adoption must route as {kind}: {}",
            self.stderr
        );
        assert!(
            self.mandate.contains(&mandate_naming(handle)),
            "the mandate must name the migrated handle {handle}: {:?}",
            self.mandate
        );
    }
}

/// The **v1-flat** grammar (`<dotted>-[<key>]-<slug>[.BRIEF|.DONE].md`) carried
/// all the way to a routed session by one bare `grove`.
///
/// Two things about this fixture are load-bearing rather than decorative. It
/// carries a **node brief** (`2-[2]-spec.BRIEF.md`), so the run has to build a
/// directory and place a child inside it — a flat one-leaf tree exercises the
/// rename and nothing else. And its permanent keys are **already assigned**, so
/// `draft-k3` in the mandate is the seeded key surviving adoption, not a counter
/// that happened to land on the same number.
///
/// The routed kind is `design`, deliberately not the `impl` a lost kind would
/// fall back to.
#[test]
fn a_v1_flat_tree_is_adopted_migrated_and_routed_by_its_migrated_filename() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("v1-flat-worktree");
    init_git_worktree(&worktree);
    configure_git_identity(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# v1-flat-worktree — brief\n").unwrap();
    fs::write(
        grove.join("1-[1]-groundwork.DONE.md"),
        "# 1-[1]-groundwork\n\n**Kind:** impl\n\n## Goal\nAlready done.\n",
    )
    .unwrap();
    fs::write(
        grove.join("2-[2]-spec.BRIEF.md"),
        "# 2-[2]-spec — brief\n\n## Goal\nSpec it.\n",
    )
    .unwrap();
    fs::write(
        grove.join("2.1-[3]-draft.md"),
        "# 2.1-[3]-draft\n\n**Kind:** design\n\n## Goal\nDraft.\n",
    )
    .unwrap();
    fs::write(
        grove.join("3-[4]-ship.md"),
        "# 3-[4]-ship\n\n**Kind:** impl\n\n## Goal\nShip.\n",
    )
    .unwrap();
    run_command("git", &worktree, &["add", "-A"]);
    run_command("git", &worktree, &["commit", "-q", "-m", "seed v1-flat"]);

    let adopted = adopt_with_bare_grove(fixture.path(), &home, &worktree);

    assert_eq!(
        adopted.entries,
        [
            "01-DONE-impl-groundwork-k1.md",
            "02-spec-k2",
            "02-spec-k2/01-design-draft-k3.md",
            "02-spec-k2/BRIEF.md",
            "03-impl-ship-k4.md",
            "BRIEF.md",
            "FORMAT",
        ]
    );
    adopted.assert_reached_current_format();
    adopted.assert_one_migration_commit("v1-flat-worktree");
    adopted.assert_routed_launch("design", "draft-k3");
}

/// The older **`NNN-slug/`** grammar is refused by one bare `grove`, and the
/// refusal is the whole of what happens.
///
/// It used to migrate, `done/` mirror and all. What replaces that coverage is the
/// property that matters now the reader is gone: the shape is still **classified**
/// rather than falling through as an unrecognisable tree. That distinction is not
/// cosmetic — a tree of this shape that classified as empty would be stamped with
/// a current-format witness, after which every entry in it is foreign and `pick`
/// reports a finished grove. So this asserts the refusal *and* that the tree is
/// byte-identical afterwards, which is the claim a silent misclassification would
/// break while an error message alone would not.
#[test]
fn an_nnn_slug_tree_is_refused_without_touching_the_tree() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("nnn-slug-worktree");
    init_git_worktree(&worktree);
    configure_git_identity(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(grove.join("020-spec")).unwrap();
    fs::create_dir_all(grove.join("done/020-spec")).unwrap();
    fs::write(grove.join("BRIEF.md"), "# nnn-slug-worktree — brief\n").unwrap();
    fs::write(
        grove.join("done/010-groundwork.md"),
        "# 010-groundwork\n\n**Kind:** impl\n\n## Goal\nAlready done.\n",
    )
    .unwrap();
    fs::write(
        grove.join("020-spec/BRIEF.md"),
        "# 020-spec — brief\n\n## Goal\nSpec it.\n",
    )
    .unwrap();
    fs::write(
        grove.join("done/020-spec/005-scoping.md"),
        "# 005-scoping\n\n**Kind:** research\n\n## Goal\nSurvey.\n",
    )
    .unwrap();
    fs::write(
        grove.join("020-spec/010-draft.md"),
        "# 010-draft\n\n**Kind:** design\n\n## Goal\nDraft.\n",
    )
    .unwrap();
    fs::write(
        grove.join("030-ship.md"),
        "# 030-ship\n\n**Kind:** impl\n\n## Goal\nShip.\n",
    )
    .unwrap();
    run_command("git", &worktree, &["add", "-A"]);
    run_command("git", &worktree, &["commit", "-q", "-m", "seed NNN-slug"]);

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
    write_complete_config(
        &home,
        &format!("{} '${{prompt}}'", shell_quote(&configured)),
    );

    let output = run_grove(&home, &worktree);
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

    assert!(
        !output.status.success(),
        "bare grove must stop on an unmigratable tree; stderr was {stderr}"
    );
    assert!(
        stderr.contains("no longer migrates") && stderr.contains("NNN-slug"),
        "the refusal must name the layout: {stderr}"
    );
    assert!(
        stderr.contains("020-spec") && stderr.contains("030-ship.md"),
        "the refusal must name the entries that classified the tree: {stderr}"
    );
    assert!(
        !launched.exists(),
        "no session may be launched over a tree Grove refused to bring current"
    );
    assert_eq!(
        tree_snapshot(&grove),
        before,
        "a refused tree is byte-identical afterwards"
    );
    assert!(
        !grove.join("FORMAT").exists(),
        "no current-format witness may be installed over a tree that was not migrated"
    );
    assert!(
        !grove.join("MIGRATING-session-kinds").exists(),
        "a refusal happens before the transaction witness exists"
    );
    assert_eq!(
        git_subjects(&worktree),
        seeded_subjects,
        "a refusal commits nothing"
    );
}

#[test]
fn config_is_reloaded_after_a_completed_legacy_transition_before_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("reload-after-transition");
    init_git_worktree(&worktree);
    configure_git_identity(&worktree);
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
    // Grove's migration commit runs with user hooks disabled, so the corruption
    // is injected from a `git` stand-in on PATH. It has to land between the
    // transition's commit and the launch, and Grove drives no other process in
    // that window.
    let fake_bin = fixture.path().join("fake-bin");
    fs::create_dir_all(&fake_bin).unwrap();
    let real_git = resolve_real_git();
    write_executable(
        &fake_bin.join("git"),
        &format!(
            "#!/bin/sh\nfor argument in \"$@\"; do\n    if [ \"$argument\" = commit ]; then\n        {git} \"$@\" || exit $?\n        cp {invalid} {active}\n        exit 0\n    fi\ndone\nexec {git} \"$@\"\n",
            git = shell_quote(&real_git),
            invalid = shell_quote(&invalid),
            active = shell_quote(&active),
        ),
    );

    let output = Command::new(env!("CARGO_BIN_EXE_grove"))
        .current_dir(&worktree)
        .env("HOME", &home)
        .env("PATH", path_with_front(&fake_bin))
        .output()
        .unwrap();

    assert!(!output.status.success());
    assert!(grove.join("01-impl-task-k1.md").is_file());
    assert!(!launch_marker.exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("missing session kinds"), "{stderr}");
}

#[test]
fn empty_current_tree_allocates_and_launches_one_resumable_finish_leaf() {
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
printf '%s\n' "$2" > "$1"
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

    let finish_output = run_grove(&home, &worktree);

    assert!(
        finish_output.status.success(),
        "{}",
        String::from_utf8_lossy(&finish_output.stderr)
    );
    let finish = grove.join("02-finish-finish-k2.md");
    assert!(finish.is_file());
    let finish_body = fs::read_to_string(&finish).unwrap();
    assert!(finish_body.starts_with("# finish-k2\n"));
    assert!(finish_body.contains("grove-llm finish-commit finish-k2"));
    assert!(finish_body.contains("grove-llm complete --done"));
    assert!(!finish_body.contains("**Kind:**"));
    assert!(fs::read_to_string(&launch_log)
        .unwrap()
        .contains(&mandate_naming("finish-k2")));

    fs::write(grove.join("03-design-resumed-k3.md"), "# resumed-k3\n").unwrap();
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
                .contains("finish-finish"))
            .count(),
        1
    );

    fs::rename(
        grove.join("03-design-resumed-k3.md"),
        grove.join("03-DONE-design-resumed-k3.md"),
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
                .contains("finish-finish"))
            .count(),
        1
    );
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

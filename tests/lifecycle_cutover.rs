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
/// value, with every normative consequence of it left to the skill.
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
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# cutover — brief\n").unwrap();
    fs::write(
        grove.join("01-impl--selected-work-k7.md"),
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
        log.contains("arg=<**Load the `grove-impl` skill now**"),
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
        canonical_worktree.join(".jj/grove")
    );
    assert!(log.contains("legacy_harness_pid=<unset>\n"), "{log:?}");
    assert!(log.contains("legacy_claude_pid=<unset>\n"), "{log:?}");
    assert!(log.contains("unrelated=<preserved>\n"), "{log:?}");
    assert!(log.contains("harness=<unset>\n"), "{log:?}");
    assert!(log.contains("model=<unset>\n"), "{log:?}");
    assert!(log.contains("skill=<unset>\n"), "{log:?}");
    assert!(log.contains("llm=<unset>\n"), "{log:?}");
    // The home carries a `.codex` marker — the exact condition the deleted
    // registry read as "sweep the embed into this harness". A launch writes no
    // skill directory since `delete-provisioning-k19`; the methodology is a
    // plugin a human installs, and grove's own launch path never touches it.
    assert!(!home.join(".codex/skills/grove").exists());
}

#[test]
fn a_secondary_workspace_expands_scalars_through_literal_env_word_zero() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();

    let repository = fixture.path().join("main-repository");
    init_worktree(&repository);
    let worktree = fixture.path().join("secondary-workspace");
    run_command(
        "jj",
        &repository,
        &["workspace", "add", "--quiet", worktree.to_str().unwrap()],
    );

    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# secondary — brief\n").unwrap();
    fs::write(grove.join("01-impl--scalars-k7.md"), "# scalars-k7\n").unwrap();

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
        argv.contains("session=main-repository: secondary-workspace grove\n"),
        "{argv}"
    );
    assert!(argv.contains(&mandate_naming("scalars-k7")), "{argv}");
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
    fs::write(grove.join("BRIEF.md"), "# native-jj — brief\n").unwrap();
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
}

#[test]
fn bare_grove_launches_a_session_in_a_native_jj_worktree() {
    assert_bare_grove_launches_a_session_in_a_jj_worktree(false);
}

#[test]
fn bare_grove_launches_a_session_in_a_colocated_jj_worktree() {
    assert_bare_grove_launches_a_session_in_a_jj_worktree(true);
}

#[test]
fn invalid_config_cannot_create_a_fresh_grove() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    // A template that violates a slot rule, not a document missing a kind.
    // Presence is per-kind and just-in-time now
    // (`docs/adr/complete-session-configuration.md`), so an absent key is no
    // longer what makes a document invalid — but *every* template rule is still
    // checked eagerly, over the whole document, before any tree mutation, and
    // that is the property this test defends.
    fs::write(config_dir.join("config.kdl"), "impl \"runner\"\n").unwrap();
    let worktree = fixture.path().join("rootless");
    init_worktree(&worktree);

    let output = run_grove(&home, &worktree);

    assert!(!output.status.success());
    assert!(!worktree.join(".grove").exists());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("must contain `${prompt}` exactly once"),
        "{stderr}"
    );
}

#[test]
fn invalid_config_leaves_current_empty_and_partial_trees_byte_identical() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    // As above: a malformed template rather than an absent key.
    fs::write(config_dir.join("config.kdl"), "impl \"runner\"\n").unwrap();

    for state in ["current", "empty", "partial"] {
        let worktree = fixture.path().join(format!("{state}-worktree"));
        init_worktree(&worktree);
        let grove = worktree.join(".grove");
        fs::create_dir_all(&grove).unwrap();
        fs::write(grove.join("BRIEF.md"), format!("# {state} — brief\n")).unwrap();
        match state {
            "current" => {
                fs::write(grove.join("01-impl--task-k1.md"), "# task-k1\n").unwrap();
            }
            "empty" => {
                fs::write(grove.join("01-DONE-impl--task-k1.md"), "# task-k1\n").unwrap();
            }
            // The charter alone: a taskless root, which the lifecycle
            // transition refuses (`collapse-tree-access-k13`). Configuration is
            // reached first either way, and that is the point — the tree is not
            // touched, whichever answer it would have got.
            "partial" => {}
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
            stderr.contains("must contain `${prompt}` exactly once"),
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
    // No `~/.config/grove/config.kdl` at all: reaching configuration would
    // report the missing file instead.
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# unwritable — brief\n").unwrap();
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
        !stderr.contains("config.kdl") && !stderr.contains("configuration is missing"),
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
    let template = format!("{} {} '${{prompt}}'", shell_quote(&fake), shell_quote(&log));
    write_complete_config(&home, &template);

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
    let created = grove::tree_lifecycle::root_init(&worktree, "custom-plan").unwrap();
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
    write_complete_config(
        &home,
        &format!(
            "{} {} '${{prompt}}'",
            shell_quote(&fake),
            shell_quote(&prompt_log)
        ),
    );

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

#[test]
fn relaunch_reloads_config_and_uses_the_new_filename_kind() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("reload-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# reload — brief\n").unwrap();
    fs::write(grove.join("01-impl--first-k1.md"), "# first-k1\n").unwrap();

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
  mv .grove/01-impl--first-k1.md .grove/01-DONE-impl--first-k1.md
  printf '# second-k2\n' > .grove/02-design--second-k2.md
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
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# insertion — brief\n").unwrap();
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
    assert!(grove.join("01-design--inserted-k8.md").is_file());
}

#[test]
fn spawn_failure_names_the_kind_executable_and_config_without_retiring_the_leaf() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let worktree = fixture.path().join("spawn-failure-worktree");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# failure — brief\n").unwrap();
    let leaf = grove.join("01-impl--still-live-k9.md");
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
    let leaked_signal_channels = fs::read_dir(worktree.join(".jj/grove"))
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
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# nonzero — brief\n").unwrap();
    fs::write(grove.join("01-design--crashing-k4.md"), "# crashing-k4\n").unwrap();
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
                ("020-spec/BRIEF.md", "# 020-spec — brief\n"),
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
                ("2-[2]-spec.BRIEF.md", "# 2-[2]-spec — brief\n"),
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
        write_complete_config(
            &home,
            &format!("{} '${{prompt}}'", shell_quote(&configured)),
        );

        let output = run_grove(&home, &worktree);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();

        assert!(
            !output.status.success(),
            "bare grove must stop on a {name} tree; stderr was {stderr}"
        );
        assert!(
            stderr.contains("holds no Grove entries")
                && stderr.contains("NN-<kind>--<slug>-k<key>"),
            "the refusal must name the grammar grove reads: {stderr}"
        );
        for entry in entries {
            assert!(
                stderr.contains(entry),
                "the refusal must name {entry:?}: {stderr}"
            );
        }
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

/// The finish sentinel is a leaf grove writes itself, so the just-in-time
/// presence rule binds it exactly as it binds `leaf-add`: a configuration with
/// no `finish` template refuses **before** the leaf is written, not at the
/// launch that would follow it
/// (`docs/adr/complete-session-configuration.md`). A tree left holding a leaf
/// whose kind cannot launch is the state the rule exists to prevent.
#[test]
fn a_finish_leaf_is_not_written_when_no_finish_template_resolves() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    // Valid, and silent about `finish`.
    fs::write(config_dir.join("config.kdl"), "impl \"true ${prompt}\"\n").unwrap();
    let worktree = fixture.path().join("no-finish-template");
    init_worktree(&worktree);
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# no-finish-template — brief\n").unwrap();
    fs::write(
        grove.join("01-DONE-impl--finished-k1.md"),
        "# finished-k1\n",
    )
    .unwrap();
    let before = tree_snapshot(&grove);

    let output = run_grove(&home, &worktree);

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(
        stderr.contains("key `finish` does not resolve"),
        "the refusal must name the kind and the file that should declare it: {stderr}"
    );
    assert_eq!(
        tree_snapshot(&grove),
        before,
        "no finish leaf may be written for a kind that cannot launch"
    );
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
    fs::write(grove.join("BRIEF.md"), "# empty-current — brief\n").unwrap();
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
fn cli_metadata_exposes_only_the_bare_entrypoint_and_writes_no_skill_directory() {
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
    // **The behavioural witness for `delete-provisioning-k19`.** The home above
    // carries a `.codex` marker, which is exactly what the deleted registry
    // treated as "this harness is installed, sweep the embed into it". No run of
    // the binary may create that directory any more — not the metadata paths
    // above, and not the refused verb, which used to be the sweep's entry point.
    assert!(!home.join(".codex/skills/grove").exists());
}

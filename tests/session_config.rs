use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use grove::session_config::{DeltaRoots, ExpansionContext, SessionConfig};
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

fn complete_document(template_for_requirements: &str) -> String {
    let mut document = String::new();
    for kind in SESSION_KINDS {
        let template = if *kind == "requirements" {
            template_for_requirements
        } else {
            "runner ${prompt}"
        };
        document.push_str(&format!("{kind} {template:?}\n"));
    }
    document
}

fn write_config(home: &Path, template_for_requirements: &str) {
    write_raw_config(home, &complete_document(template_for_requirements));
}

fn write_raw_config(home: &Path, document: &str) -> PathBuf {
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let path = config_dir.join("config.kdl");
    fs::write(&path, document).unwrap();
    path
}

/// Load with two delta roots that hold no `.grove.kdl`, which is every
/// pre-delta case: resolution must land exactly where it did before the search
/// existed. A bare temp directory is deliberate — with no candidate file the
/// trackedness probe never runs, so no VCS fixture is needed to assert it.
fn load(home: &Path) -> anyhow::Result<SessionConfig> {
    let empty = TempDir::new().unwrap();
    load_from(home, empty.path(), empty.path())
}

fn load_from(home: &Path, worktree: &Path, repository: &Path) -> anyhow::Result<SessionConfig> {
    SessionConfig::load(
        home,
        &DeltaRoots {
            worktree,
            repository,
        },
    )
}

fn load_error(home: &Path) -> String {
    load(home).err().unwrap().to_string()
}

fn load_error_from(home: &Path, worktree: &Path, repository: &Path) -> String {
    load_from(home, worktree, repository)
        .err()
        .unwrap()
        .to_string()
}

fn write_delta(root: &Path, document: &str) -> PathBuf {
    let path = root.join(".grove.kdl");
    fs::write(&path, document).unwrap();
    path
}

fn context<'a>(prompt: &'a str) -> ExpansionContext<'a> {
    ExpansionContext {
        prompt,
        session_name: "session",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repo"),
    }
}

#[test]
fn load_and_expand_preserve_argument_boundaries_and_prompt_position() {
    let home = TempDir::new().unwrap();
    write_config(
        home.path(),
        "env RUN_MODE=review wrapper --before '${prompt}' --tree '${worktree}' --after",
    );

    let config = load(home.path()).unwrap();
    let worktree = Path::new("/worktrees/config with spaces; touch nope");
    let repository = Path::new("/repos/grove");
    let context = ExpansionContext {
        prompt: "mandate; echo not-a-shell",
        session_name: "grove: config grove",
        worktree,
        repository,
    };

    let argv = config.expand("requirements", &context).unwrap();

    assert_eq!(
        argv,
        vec![
            OsString::from("env"),
            OsString::from("RUN_MODE=review"),
            OsString::from("wrapper"),
            OsString::from("--before"),
            OsString::from("mandate; echo not-a-shell"),
            OsString::from("--tree"),
            worktree.as_os_str().to_owned(),
            OsString::from("--after"),
        ]
    );
}

#[test]
fn raw_kdl_strings_are_valid_command_templates() {
    let home = TempDir::new().unwrap();
    let mut document = String::from(
        r##"requirements r#"runner --label "quoted value" ${prompt}"#
"##,
    );
    for kind in &SESSION_KINDS[1..] {
        document.push_str(&format!("{kind} {:?}\n", "runner ${prompt}"));
    }
    write_raw_config(home.path(), &document);

    let config = load(home.path()).unwrap();
    let context = ExpansionContext {
        prompt: "mandate",
        session_name: "session",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repo"),
    };

    assert_eq!(
        config.expand("requirements", &context).unwrap(),
        vec!["runner", "--label", "quoted value", "mandate"]
    );
}

#[test]
fn scalar_substitutions_each_expand_to_one_argument() {
    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${session_name} ${repo} ${prompt}");
    let config = load(home.path()).unwrap();
    let context = ExpansionContext {
        prompt: "mandate",
        session_name: "grove repo: config grove",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repos/main with spaces"),
    };

    let argv = config.expand("requirements", &context).unwrap();

    assert_eq!(
        argv,
        vec![
            OsString::from("runner"),
            OsString::from("grove repo: config grove"),
            OsString::from("/repos/main with spaces"),
            OsString::from("mandate"),
        ]
    );
}

#[test]
fn missing_file_names_its_path_and_every_required_kind() {
    let home = TempDir::new().unwrap();

    let error = load_error(home.path());

    assert!(error.contains(
        &home
            .path()
            .join(".config/grove/config.kdl")
            .display()
            .to_string()
    ));
    assert!(error.contains(&SESSION_KINDS.join(", ")));
}

#[test]
fn schema_and_template_failures_are_aggregated_with_source_locations() {
    let home = TempDir::new().unwrap();
    let path = write_raw_config(
        home.path(),
        concat!(
            "requirements \"runner\"\n",
            "requirements \"runner ${prompt}\"\n",
            "mystery \"runner ${prompt}\"\n",
            "design \"runner ${prompt}\" property=true { child; }\n",
            "planning 42\n",
            "prototype \"runner ${prompt}\" \"extra\"\n",
            "impl \"runner 'unterminated ${prompt}\"\n",
        ),
    );

    let error = load_error(home.path());
    let display_path = path.display().to_string();

    assert!(
        error.contains("missing session kinds: review-requirements, integrate-review-requirements, review-design"),
        "{error}"
    );
    assert!(error.contains(
        "review-impl, integrate-review-impl, research-a, research-b, combine-research, finish"
    ));
    assert!(error.contains(&format!("{display_path}:1:1")));
    assert!(error.contains(&format!("{display_path}:2:1")));
    assert!(error.contains("duplicate session kind `requirements`"));
    assert!(error.contains(&format!(
        "{display_path}:3:1: unknown session kind `mystery`"
    )));
    assert!(error.contains(&format!("{display_path}:4:1")));
    assert!(error.contains("properties and child blocks are not allowed"));
    assert!(error.contains(&format!("{display_path}:5:1")));
    assert!(error.contains("sole argument must be a string"));
    assert!(error.contains(&format!("{display_path}:6:1")));
    assert!(error.contains("exactly one positional argument"));
    assert!(error.contains(&format!("{display_path}:7:1")));
    assert!(error.contains("command template has unmatched quotes"));
    assert!(error.contains(&format!(
        "{display_path}:1:1: session kind `requirements`: command template must contain `${{prompt}}` exactly once"
    )));
}

#[test]
fn kdl_syntax_errors_name_the_source_location() {
    let home = TempDir::new().unwrap();
    let path = write_raw_config(
        home.path(),
        "requirements \"runner ${prompt}\"\ndesign 1.\n",
    );

    let error = load_error(home.path());

    assert!(error.contains(&format!("{}:2:", path.display())));
    assert!(error.contains("KDL syntax error"));
    assert!(error.contains("Expected valid value"));
}

#[test]
fn invalid_placeholder_forms_are_rejected() {
    let cases = [
        ("", "literal non-empty executable"),
        ("${prompt} runner", "word zero must be a literal executable"),
        ("runner", "must contain `${prompt}` exactly once"),
        (
            "runner ${prompt} ${prompt}",
            "must contain `${prompt}` exactly once",
        ),
        (
            "runner ${session_name} ${session_name} ${prompt}",
            "`${session_name}` may appear at most once",
        ),
        (
            "runner prefix${prompt}",
            "substitutions must occupy a complete shell word",
        ),
        (
            "runner ${unknown} ${prompt}",
            "unknown substitution `${unknown}`",
        ),
        (
            "runner ${a}${prompt}",
            "substitutions must occupy a complete shell word",
        ),
        (
            "runner ${prompt}${session_name}",
            "substitutions must occupy a complete shell word",
        ),
    ];

    for (template, expected) in cases {
        let home = TempDir::new().unwrap();
        write_config(home.path(), template);

        let error = load_error(home.path());

        assert!(
            error.contains(expected),
            "expected {expected:?} for template {template:?}, got:\n{error}"
        );
    }
}

#[test]
fn herdr_settings_is_not_a_supported_substitution() {
    let home = TempDir::new().unwrap();
    write_config(
        home.path(),
        "runner ${herdr_settings} --model opus ${prompt}",
    );

    let error = load_error(home.path());

    assert!(
        error.contains("unknown substitution `${herdr_settings}`"),
        "Herdr-specific launch policy must not remain in Grove configuration:\n{error}"
    );
}

#[test]
fn empty_word_zero_reports_one_diagnostic() {
    let home = TempDir::new().unwrap();
    write_config(home.path(), "'' runner ${prompt}");

    let error = load_error(home.path());

    assert_eq!(
        error.matches("word zero must be a literal").count(),
        1,
        "{error}"
    );
}

#[test]
fn shell_metacharacters_remain_literal_arguments() {
    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner '$(touch nope)' '*' '>' '${prompt}'");
    let config = load(home.path()).unwrap();
    let context = ExpansionContext {
        prompt: "mandate",
        session_name: "session",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repo"),
    };

    assert_eq!(
        config.expand("requirements", &context).unwrap(),
        vec!["runner", "$(touch nope)", "*", ">", "mandate"]
    );
}

#[test]
fn unquoted_shell_comment_introducers_are_rejected_instead_of_truncating_argv() {
    for template in [
        "runner ${prompt} --color #ff0000 --verbose",
        "runner ${prompt} #ff0000 --trailing",
    ] {
        let home = TempDir::new().unwrap();
        write_config(home.path(), template);

        let error = load_error(home.path());

        assert!(
            error.contains("`#` starts a comment in a command template"),
            "expected a comment diagnostic for template {template:?}, got:\n{error}"
        );
    }
}

#[test]
fn quoted_escaped_and_midword_hashes_remain_literal_arguments() {
    for (template, expected) in [
        ("runner ${prompt} --color '#ff0000'", "#ff0000"),
        ("runner ${prompt} --color \\#ff0000", "#ff0000"),
        ("runner ${prompt} --tag tag#1", "tag#1"),
    ] {
        let home = TempDir::new().unwrap();
        write_config(home.path(), template);
        let config = load(home.path()).unwrap();
        let context = ExpansionContext {
            prompt: "mandate",
            session_name: "session",
            worktree: Path::new("/worktree"),
            repository: Path::new("/repo"),
        };

        let argv = config.expand("requirements", &context).unwrap();

        assert_eq!(
            argv.last().unwrap(),
            &OsString::from(expected),
            "{template:?}"
        );
    }
}

#[test]
fn duplicate_unknown_nodes_report_every_declaration_location() {
    let home = TempDir::new().unwrap();
    let mut document = complete_document("runner ${prompt}");
    document.push_str("mystery \"runner ${prompt}\"\n");
    document.push_str("mystery \"other ${prompt}\"\n");
    let path = write_raw_config(home.path(), &document);

    let error = load_error(home.path());

    assert!(
        error.contains("duplicate session kind `mystery`"),
        "{error}"
    );
    assert!(
        error.contains(&format!("{}:20:1", path.display())),
        "{error}"
    );
    assert!(
        error.contains(&format!("{}:21:1", path.display())),
        "{error}"
    );
}

// ---------------------------------------------------------------------------
// The configuration delta (`docs/adr/untracked-configuration-delta.md`)
//
// Everything here is asserted at `SessionConfig::load`, the one seam the delta
// lives behind, and no test spawns a configured command. The trackedness cases
// need real VCS fixtures rather than a bare temp directory, because the property
// under test is what a VCS says about a path; that widens the fixture, not the
// boundary.

/// A git checkout with a `.grove.kdl` — committed (so git's index holds it) or
/// merely present.
fn git_checkout_with_delta(document: &str, commit: bool) -> TempDir {
    let tmp = TempDir::new().unwrap();
    run("git", tmp.path(), &["init", "-q", "."]);
    run(
        "git",
        tmp.path(),
        &["config", "user.email", "t@example.com"],
    );
    run("git", tmp.path(), &["config", "user.name", "Grove Test"]);
    run(
        "git",
        tmp.path(),
        &["config", "core.hooksPath", "/dev/null"],
    );
    write_delta(tmp.path(), document);
    if commit {
        run("git", tmp.path(), &["add", "-A"]);
        run("git", tmp.path(), &["commit", "-q", "-m", "delta"]);
    }
    tmp
}

/// A jj working tree with a `.grove.kdl`. `colocate` picks the second jj shape —
/// a `.git` beside the `.jj`, where jj-first is a choice rather than the only
/// option. `ignored` writes the ignore line the refusal names, which is what
/// keeps the file out of the working-copy commit across the snapshot below.
fn jj_tree_with_delta(document: &str, colocate: bool, ignored: bool) -> TempDir {
    let tmp = TempDir::new().unwrap();
    let colocation = if colocate {
        "git.colocate=true"
    } else {
        "git.colocate=false"
    };
    run_jj(
        tmp.path(),
        &["--config", colocation, "git", "init", "--quiet", "."],
    );
    if ignored {
        fs::write(tmp.path().join(".gitignore"), "/.grove.kdl\n").unwrap();
    }
    write_delta(tmp.path(), document);
    // jj snapshots the working copy on any ordinary command, which is exactly
    // the moment an unignored delta becomes tracked.
    run_jj(tmp.path(), &["status"]);
    tmp
}

/// Fixture commands must describe the tree they are pointed at and nothing else.
/// A `Command` inherits the parent's environment whether or not the parent meant
/// it to, and one case below deliberately runs under a hostile `GIT_INDEX_FILE`
/// — which would otherwise send this fixture's own `git add` to the wrong index.
fn run(bin: &str, dir: &Path, args: &[&str]) -> String {
    let out = std::process::Command::new(bin)
        .current_dir(dir)
        .args(args)
        .env_remove("GIT_DIR")
        .env_remove("GIT_WORK_TREE")
        .env_remove("GIT_COMMON_DIR")
        .env_remove("GIT_INDEX_FILE")
        .output()
        .unwrap_or_else(|error| panic!("running {bin} {args:?}: {error} (is {bin} installed?)"));
    assert!(
        out.status.success(),
        "{bin} {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8_lossy(&out.stdout).into_owned()
}

fn run_jj(dir: &Path, args: &[&str]) -> String {
    let mut full = vec![
        "--config",
        "user.name=Test",
        "--config",
        "user.email=t@example.com",
    ];
    full.extend_from_slice(args);
    run("jj", dir, &full)
}

#[test]
fn a_delta_overrides_only_the_kinds_it_declares() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    write_delta(worktree.path(), "impl \"other --model opus ${prompt}\"\n");

    let config = load_from(home.path(), worktree.path(), worktree.path()).unwrap();

    assert_eq!(
        config.expand("impl", &context("mandate")).unwrap(),
        vec!["other", "--model", "opus", "mandate"],
        "the declared kind must come from the delta"
    );
    assert_eq!(
        config.expand("design", &context("mandate")).unwrap(),
        vec!["runner", "mandate"],
        "an undeclared kind must fall through to the personal file untouched"
    );
}

#[test]
fn each_kind_reports_the_file_it_resolved_from() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let delta_path = write_delta(worktree.path(), "impl \"other ${prompt}\"\n");

    let config = load_from(home.path(), worktree.path(), worktree.path()).unwrap();

    assert_eq!(config.source("impl"), Some(delta_path.as_path()));
    assert_eq!(
        config.source("design"),
        Some(home.path().join(".config/grove/config.kdl").as_path())
    );
}

#[test]
fn the_worktree_delta_shadows_the_repository_root_and_the_loser_is_not_read() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    let repository = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    write_delta(worktree.path(), "impl \"chosen ${prompt}\"\n");
    // Unparseable, and never merged or even opened: a load that reads it fails.
    write_delta(repository.path(), "impl 1.\n");

    let config = load_from(home.path(), worktree.path(), repository.path()).unwrap();

    assert_eq!(
        config.expand("impl", &context("mandate")).unwrap(),
        vec!["chosen", "mandate"]
    );
}

#[test]
fn a_repository_root_delta_is_read_when_the_worktree_has_none() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    let repository = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    write_delta(repository.path(), "impl \"inherited ${prompt}\"\n");

    let config = load_from(home.path(), worktree.path(), repository.path()).unwrap();

    assert_eq!(
        config.expand("impl", &context("mandate")).unwrap(),
        vec!["inherited", "mandate"]
    );
}

#[test]
fn a_delta_relaxes_nothing_about_the_personal_file() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    let mut document = complete_document("runner ${prompt}");
    document = document.replace("impl \"runner ${prompt}\"\n", "");
    write_raw_config(home.path(), &document);
    write_delta(worktree.path(), "impl \"other ${prompt}\"\n");

    let error = load_error_from(home.path(), worktree.path(), worktree.path());

    assert!(
        error.contains("missing session kinds: impl"),
        "the personal file must still declare all nineteen, whatever a delta says:\n{error}"
    );
}

#[test]
fn delta_diagnostics_are_aggregated_against_the_deltas_own_path_and_location() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let delta_path = write_delta(
        worktree.path(),
        concat!(
            "mystery \"runner ${prompt}\"\n",
            "impl \"runner ${prompt}\"\n",
            "impl \"other ${prompt}\"\n",
            "design \"runner ${prompt}\" property=true { child; }\n",
            "planning \"runner ${prompt}\" \"extra\"\n",
            "finish \"runner\"\n",
        ),
    );

    let error = load_error_from(home.path(), worktree.path(), worktree.path());
    let display_path = delta_path.display().to_string();

    assert!(
        error.contains(&format!(
            "invalid Grove configuration delta at {display_path}"
        )),
        "{error}"
    );
    assert!(
        error.contains(&format!(
            "{display_path}:1:1: unknown session kind `mystery`"
        )),
        "{error}"
    );
    assert!(error.contains("duplicate session kind `impl`"), "{error}");
    assert!(error.contains(&format!("{display_path}:2:1")), "{error}");
    assert!(error.contains(&format!("{display_path}:3:1")), "{error}");
    assert!(
        error.contains("properties and child blocks are not allowed"),
        "{error}"
    );
    assert!(error.contains("exactly one positional argument"), "{error}");
    assert!(
        error.contains("must contain `${prompt}` exactly once"),
        "{error}"
    );
    assert!(
        !error.contains("missing session kinds"),
        "a delta is a partial; completeness is not its rule:\n{error}"
    );
}

#[test]
fn an_unparseable_delta_names_its_own_source_location() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let delta_path = write_delta(worktree.path(), "impl \"runner ${prompt}\"\ndesign 1.\n");

    let error = load_error_from(home.path(), worktree.path(), worktree.path());

    assert!(
        error.contains(&format!("{}:2:", delta_path.display())),
        "{error}"
    );
    assert!(error.contains("KDL syntax error"), "{error}");
}

#[test]
fn an_unreadable_delta_fails_closed() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    // A directory at the searched path: present, so it is *the* delta, and
    // unreadable, so the load fails rather than resolving to the personal file.
    fs::create_dir(worktree.path().join(".grove.kdl")).unwrap();

    let error = load_error_from(home.path(), worktree.path(), worktree.path());

    assert!(
        error.contains("failed to read the Grove configuration delta"),
        "{error}"
    );
}

#[test]
fn every_delta_template_rule_still_binds() {
    for (template, expected) in [
        ("runner", "must contain `${prompt}` exactly once"),
        ("${prompt} runner", "word zero must be a literal executable"),
        (
            "runner ${unknown} ${prompt}",
            "unknown substitution `${unknown}`",
        ),
        (
            "runner ${worktree} ${worktree} ${prompt}",
            "`${worktree}` may appear at most once",
        ),
        (
            "runner --color #ff0000 ${prompt}",
            "`#` starts a comment in a command template",
        ),
    ] {
        let home = TempDir::new().unwrap();
        let worktree = TempDir::new().unwrap();
        write_config(home.path(), "runner ${prompt}");
        write_delta(worktree.path(), &format!("impl {template:?}\n"));

        let error = load_error_from(home.path(), worktree.path(), worktree.path());

        assert!(
            error.contains(expected),
            "expected {expected:?} for delta template {template:?}, got:\n{error}"
        );
    }
}

#[test]
fn a_git_tracked_delta_is_refused_and_the_refusal_names_the_ignore_line() {
    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let tree = git_checkout_with_delta("impl \"other ${prompt}\"\n", true);

    let error = load_error_from(home.path(), tree.path(), tree.path());

    assert!(
        error.contains(&tree.path().join(".grove.kdl").display().to_string()),
        "{error}"
    );
    assert!(error.contains("tracked"), "{error}");
    assert!(error.contains("/.grove.kdl"), "{error}");
}

#[test]
fn a_git_untracked_delta_is_read() {
    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let tree = git_checkout_with_delta("impl \"other ${prompt}\"\n", false);

    let config = load_from(home.path(), tree.path(), tree.path()).unwrap();

    assert_eq!(
        config.expand("impl", &context("mandate")).unwrap(),
        vec!["other", "mandate"]
    );
}

#[test]
fn a_snapshotted_jj_delta_is_refused_in_both_jj_shapes() {
    for colocate in [false, true] {
        let home = TempDir::new().unwrap();
        write_config(home.path(), "runner ${prompt}");
        let tree = jj_tree_with_delta("impl \"other ${prompt}\"\n", colocate, false);

        let error = load_error_from(home.path(), tree.path(), tree.path());

        assert!(
            error.contains("tracked"),
            "an unignored delta is in the working-copy commit after one jj command \
             (colocate={colocate}):\n{error}"
        );
    }
}

#[test]
fn an_ignored_jj_delta_is_read_in_both_jj_shapes() {
    for colocate in [false, true] {
        let home = TempDir::new().unwrap();
        write_config(home.path(), "runner ${prompt}");
        let tree = jj_tree_with_delta("impl \"other ${prompt}\"\n", colocate, true);

        let config = load_from(home.path(), tree.path(), tree.path()).unwrap();

        assert_eq!(
            config.expand("impl", &context("mandate")).unwrap(),
            vec!["other", "mandate"],
            "colocate={colocate}"
        );
    }
}

#[test]
fn a_trackedness_probe_that_cannot_be_completed_fails_closed() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    // A worktree marker naming a gitdir that is not there: a Git tree by every
    // test Grove applies, and one no probe can answer about.
    fs::write(worktree.path().join(".git"), "gitdir: ./absent-gitdir\n").unwrap();
    write_delta(worktree.path(), "impl \"other ${prompt}\"\n");

    let error = load_error_from(home.path(), worktree.path(), worktree.path());

    assert!(
        error.contains("is tracked"),
        "an unanswerable probe must fail the load, not resolve to the personal file:\n{error}"
    );
}

// A process-global variable cannot be set in-process here: the sibling cases
// above drive real `git` and `jj` fixtures in parallel inside this one test
// binary. The body therefore re-runs in a child copy of the binary that was
// spawned with the variable already installed, and the parent only asserts the
// child passed.
const ISOLATED_AMBIENT_ENVIRONMENT: &str = "GROVE_TEST_ISOLATED_AMBIENT_ENVIRONMENT";

fn this_test_name() -> String {
    std::thread::current()
        .name()
        .expect("the Rust test harness names every test thread")
        .to_string()
}

fn running_in_the_prepared_child() -> bool {
    std::env::var_os(ISOLATED_AMBIENT_ENVIRONMENT).is_some()
}

fn rerun_this_test_with(variables: &[(&str, &Path)]) {
    let name = this_test_name();
    let mut command =
        std::process::Command::new(std::env::current_exe().expect("locating the unit-test binary"));
    command
        .args(["--exact", &name, "--nocapture"])
        .env(ISOLATED_AMBIENT_ENVIRONMENT, &name);
    for (key, value) in variables {
        command.env(key, value);
    }
    let output = command.output().expect("launching the isolated child test");
    assert!(
        output.status.success(),
        "isolated test {name} failed with {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

/// The trackedness probe answers about the repository Grove selected, not about
/// an index the process that launched Grove chose. `GIT_INDEX_FILE` selects an
/// index independently of the worktree, so anchoring the worktree does not
/// dislodge it: without scrubbing, `git ls-files` consults the inherited
/// alternate index, reports a committed delta as untracked, and Grove executes
/// the repository-controlled launch template the seam exists to refuse.
#[test]
fn a_tracked_delta_is_refused_under_an_inherited_alternate_git_index() {
    if !running_in_the_prepared_child() {
        let scratch = git_checkout_with_delta("impl \"other ${prompt}\"\n", true);
        let alternate = TempDir::new().unwrap();
        let index = alternate.path().join("alternate-index");
        // A *valid* empty index, not a missing file: the bypass must not turn on
        // git tolerating a broken path.
        let out = std::process::Command::new("git")
            .current_dir(scratch.path())
            .args(["read-tree", "--empty"])
            .env("GIT_INDEX_FILE", &index)
            .output()
            .unwrap();
        assert!(out.status.success(), "preparing the alternate index");
        assert!(index.is_file(), "the alternate index must exist");

        rerun_this_test_with(&[("GIT_INDEX_FILE", &index)]);
        return;
    }

    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let tree = git_checkout_with_delta("impl \"other ${prompt}\"\n", true);

    let error = load_from(home.path(), tree.path(), tree.path())
        .err()
        .expect(
            "a tracked delta must be refused whatever index the ambient environment names, \
             and the load accepted it",
        )
        .to_string();

    assert!(
        error.contains("tracked"),
        "an inherited alternate index must not make a tracked delta readable:\n{error}"
    );
}

/// Absence is `NotFound` and nothing else. A candidate whose state cannot be
/// established is neither present nor absent, and treating it as absent breaks
/// both halves of the search: at the worktree root it hands the decision to the
/// repository root, and at the repository root it hands it back to the personal
/// file.
#[cfg(unix)]
#[test]
fn a_candidate_grove_cannot_stat_fails_closed_instead_of_reading_the_next_one() {
    use std::os::unix::fs::PermissionsExt;

    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    let tmp = TempDir::new().unwrap();
    let worktree = tmp.path().join("worktree");
    let repository = tmp.path().join("repository");
    fs::create_dir(&worktree).unwrap();
    fs::create_dir(&repository).unwrap();
    // The delta that must not be reached: it is second in the search order, and
    // the first candidate did not answer "absent".
    write_delta(&repository, "impl \"repository ${prompt}\"\n");
    fs::set_permissions(&worktree, fs::Permissions::from_mode(0o000)).unwrap();

    let outcome = load_from(home.path(), &worktree, &repository);

    // Restored before asserting, so a failing assertion still leaves the
    // temporary directory removable.
    fs::set_permissions(&worktree, fs::Permissions::from_mode(0o755)).unwrap();

    let error = outcome
        .err()
        .expect("an unresolvable candidate must fail the load")
        .to_string();
    assert!(
        error.contains(&worktree.join(".grove.kdl").display().to_string()),
        "the refusal must name the candidate whose state is unknown:\n{error}"
    );
}

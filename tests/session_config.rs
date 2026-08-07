use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use grove::session_config::{ExpansionContext, SessionConfig};
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

fn load_error(home: &Path) -> String {
    SessionConfig::load(home).err().unwrap().to_string()
}

#[test]
fn load_and_expand_preserve_argument_boundaries_and_prompt_position() {
    let home = TempDir::new().unwrap();
    write_config(
        home.path(),
        "env HERDR_AGENT=claude wrapper --before '${prompt}' --tree '${worktree}' --after",
    );

    let config = SessionConfig::load(home.path()).unwrap();
    let worktree = Path::new("/worktrees/config with spaces; touch nope");
    let repository = Path::new("/repos/grove");
    let context = ExpansionContext {
        prompt: "mandate; echo not-a-shell",
        session_name: "grove: config grove",
        worktree,
        repository,
        herdr_settings: None,
    };

    let argv = config.expand("requirements", &context).unwrap();

    assert_eq!(
        argv,
        vec![
            OsString::from("env"),
            OsString::from("HERDR_AGENT=claude"),
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

    let config = SessionConfig::load(home.path()).unwrap();
    let context = ExpansionContext {
        prompt: "mandate",
        session_name: "session",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repo"),
        herdr_settings: None,
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
    let config = SessionConfig::load(home.path()).unwrap();
    let context = ExpansionContext {
        prompt: "mandate",
        session_name: "grove repo: config grove",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repos/main with spaces"),
        herdr_settings: None,
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
fn herdr_splice_expands_zero_or_two_arguments_without_consuming_following_words() {
    let home = TempDir::new().unwrap();
    write_config(
        home.path(),
        "runner ${herdr_settings} --model opus ${prompt} --after",
    );
    let config = SessionConfig::load(home.path()).unwrap();
    let mut context = ExpansionContext {
        prompt: "mandate",
        session_name: "session",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repo"),
        herdr_settings: None,
    };

    assert_eq!(
        config.expand("requirements", &context).unwrap(),
        vec!["runner", "--model", "opus", "mandate", "--after"]
    );

    context.herdr_settings = Some(["--settings", r#"{"hooks":{"Stop":[]}}"#]);
    assert_eq!(
        config.expand("requirements", &context).unwrap(),
        vec![
            "runner",
            "--settings",
            r#"{"hooks":{"Stop":[]}}"#,
            "--model",
            "opus",
            "mandate",
            "--after",
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
            "runner ${herdr_settings} ${herdr_settings} ${prompt}",
            "`${herdr_settings}` may appear at most once",
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
    let config = SessionConfig::load(home.path()).unwrap();
    let context = ExpansionContext {
        prompt: "mandate",
        session_name: "session",
        worktree: Path::new("/worktree"),
        repository: Path::new("/repo"),
        herdr_settings: None,
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
        let config = SessionConfig::load(home.path()).unwrap();
        let context = ExpansionContext {
            prompt: "mandate",
            session_name: "session",
            worktree: Path::new("/worktree"),
            repository: Path::new("/repo"),
            herdr_settings: None,
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

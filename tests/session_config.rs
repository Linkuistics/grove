//! What is genuinely grove's about its launch configuration.
//!
//! The template grammar, the slot rules, aggregate diagnostics and the
//! primary-declares rule live in `crates/keyed-launch` and are tested there,
//! without a session or a kind in sight. What is left here is the part that
//! could not move: grove's own **slot vocabulary**, the just-in-time presence
//! rule, and the configuration delta — where it is searched, which candidate
//! wins, and the refusal of a tracked one
//! (`docs/adr/untracked-configuration-delta.md`).

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};

use grove::session_config::{DeltaRoots, ExpansionContext, SessionConfig};
use tempfile::TempDir;

/// Kinds the fixtures below declare. Not *the* kind set — grove no longer holds
/// one — just enough distinct keys for an overlay to override one and leave
/// another alone.
const FIXTURE_KINDS: &[&str] = &["requirements", "design", "impl", "finish"];

fn complete_document(template_for_requirements: &str) -> String {
    let mut document = String::new();
    for kind in FIXTURE_KINDS {
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
    format!("{:#}", load(home).err().expect("expected a load failure"))
}

fn load_error_from(home: &Path, worktree: &Path, repository: &Path) -> String {
    format!(
        "{:#}",
        load_from(home, worktree, repository)
            .err()
            .expect("expected a load failure")
    )
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

// ---------------------------------------------------------------------------
// Grove's slot vocabulary
//
// The four names, and their cardinalities, are the whole of what grove tells the
// runner about its own domain. Everything else about a template is the runner's
// and is tested in `crates/keyed-launch/tests/templates.rs`.

/// Each of the four slots expands to exactly one argument, whatever it holds —
/// nothing re-splits a prompt, a session name or a path with spaces in it.
#[test]
fn each_grove_slot_expands_to_one_argument() {
    let home = TempDir::new().unwrap();
    write_config(
        home.path(),
        "env RUN_MODE=review wrapper --before ${prompt} --name ${session_name} \
         --tree ${worktree} --repo ${repo}",
    );

    let config = load(home.path()).unwrap();
    let worktree = Path::new("/worktrees/config with spaces; touch nope");
    let repository = Path::new("/repos/main with spaces");
    let argv = config
        .expand(
            "requirements",
            &ExpansionContext {
                prompt: "mandate; echo not-a-shell",
                session_name: "grove repo: config grove",
                worktree,
                repository,
            },
        )
        .unwrap()
        .words();

    assert_eq!(
        argv,
        vec![
            OsString::from("env"),
            OsString::from("RUN_MODE=review"),
            OsString::from("wrapper"),
            OsString::from("--before"),
            OsString::from("mandate; echo not-a-shell"),
            OsString::from("--name"),
            OsString::from("grove repo: config grove"),
            OsString::from("--tree"),
            worktree.as_os_str().to_owned(),
            OsString::from("--repo"),
            repository.as_os_str().to_owned(),
        ]
    );
}

/// `${prompt}` is required — a launch that does not carry the prompt launches a
/// session with no mandate — and the other three are optional. There is no
/// fifth.
#[test]
fn the_four_slots_are_the_vocabulary_and_prompt_is_the_required_one() {
    for (template, expected) in [
        ("runner", "must contain `${prompt}` exactly once"),
        (
            "runner ${prompt} ${prompt}",
            "must contain `${prompt}` exactly once",
        ),
        (
            "runner ${worktree} ${worktree} ${prompt}",
            "`${worktree}` may appear at most once",
        ),
        (
            "runner ${session_name} ${session_name} ${prompt}",
            "`${session_name}` may appear at most once",
        ),
        (
            "runner ${repo} ${repo} ${prompt}",
            "`${repo}` may appear at most once",
        ),
        (
            "runner ${settings} ${prompt}",
            "unknown substitution `${settings}`",
        ),
    ] {
        let home = TempDir::new().unwrap();
        write_config(home.path(), template);
        let error = load_error(home.path());
        assert!(
            error.contains(expected),
            "expected {expected:?} for template {template:?}, got:\n{error}"
        );
    }

    // And the three optional ones may be left out entirely.
    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${prompt}");
    assert_eq!(
        load(home.path())
            .unwrap()
            .expand("requirements", &context("mandate"))
            .unwrap()
            .words(),
        vec![OsString::from("runner"), OsString::from("mandate")]
    );
}

// ---------------------------------------------------------------------------
// Presence is per-kind and just-in-time
// (`docs/adr/complete-session-configuration.md`)

/// A document that declares some kinds and not others is **valid**. The question
/// grove asks is about the kind in hand, at the moment it commits to it.
#[test]
fn a_kind_the_file_does_not_declare_is_refused_only_when_it_is_used() {
    let home = TempDir::new().unwrap();
    write_raw_config(home.path(), "impl \"runner ${prompt}\"\n");

    let config = load(home.path()).expect("an incomplete document is not an invalid one");

    config.require("impl").expect("a declared kind resolves");
    let refusal = format!(
        "{:#}",
        config.require("design").expect_err("an undeclared kind")
    );
    assert!(
        refusal.contains("key `design` does not resolve"),
        "{refusal}"
    );
    assert!(
        refusal.contains(
            &home
                .path()
                .join(".config/grove/config.kdl")
                .display()
                .to_string()
        ),
        "the refusal must name the file that should declare it:\n{refusal}"
    );
    assert!(
        format!("{:#}", config.expand("design", &context("m")).unwrap_err())
            .contains("does not resolve"),
        "expansion asks the same question"
    );
}

/// The eager half survives whole: a malformed template for a kind this run will
/// never reach still fails the load.
#[test]
fn every_template_in_the_document_is_validated_however_few_kinds_it_declares() {
    let home = TempDir::new().unwrap();
    write_raw_config(
        home.path(),
        "impl \"runner ${prompt}\"\nnever-reached \"runner\"\n",
    );

    let error = load_error(home.path());

    assert!(
        error.contains(
            "key `never-reached`: command template must contain `${prompt}` exactly once"
        ),
        "{error}"
    );
}

/// A missing file is still a refusal naming its path — it just no longer recites
/// a set of kinds grove does not hold.
#[test]
fn a_missing_file_names_its_path() {
    let home = TempDir::new().unwrap();

    let error = load_error(home.path());

    assert!(error.contains("configuration is missing at"), "{error}");
    assert!(error.contains(
        &home
            .path()
            .join(".config/grove/config.kdl")
            .display()
            .to_string()
    ));
}

// ---------------------------------------------------------------------------
// The cross-crate seam (`docs/specs/module-decomposition.md`, test seam 3)

/// The runner's conformance kit, run against grove's own vocabulary.
///
/// This is what keeps *reusable outside grove* honest without a second
/// repository: the kit is written in `crates/keyed-launch` with no knowledge of
/// a session kind, and grove's configuration is held to it from here. A document
/// that passes below is one any consumer of the crate could load.
#[test]
fn a_grove_configuration_conforms_to_the_runners_own_kit() {
    let home = TempDir::new().unwrap();
    write_config(
        home.path(),
        "runner ${session_name} ${worktree} ${repo} ${prompt}",
    );

    let outcome = keyed_launch::conformance::check(
        &SessionConfig::path(home.path()),
        grove::session_config::vocabulary(),
    );

    assert!(outcome.passed(), "{}", outcome.failures.join("\n"));
}

/// And it fails the same document for the same reason grove's own load does —
/// one contract, checked from two sides.
#[test]
fn the_kit_and_grove_refuse_the_same_document() {
    let home = TempDir::new().unwrap();
    write_config(home.path(), "runner ${settings} ${prompt}");

    let outcome = keyed_launch::conformance::check(
        &SessionConfig::path(home.path()),
        grove::session_config::vocabulary(),
    );

    assert!(!outcome.passed());
    assert!(
        outcome.failures[0].contains("unknown substitution `${settings}`"),
        "{:?}",
        outcome.failures
    );
    assert!(
        load_error(home.path()).contains("unknown substitution `${settings}`"),
        "grove's own load must refuse it too"
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
        config.expand("impl", &context("mandate")).unwrap().words(),
        vec!["other", "--model", "opus", "mandate"],
        "the declared kind must come from the delta"
    );
    assert_eq!(
        config.expand("design", &context("mandate")).unwrap().words(),
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
        config.expand("impl", &context("mandate")).unwrap().words(),
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
        config.expand("impl", &context("mandate")).unwrap().words(),
        vec!["inherited", "mandate"]
    );
}

/// The per-kind restatement of what the all-nineteen rule bought: an untracked
/// file a project ships cannot introduce a program the operator never chose.
#[test]
fn a_kind_only_the_delta_declares_does_not_resolve() {
    let home = TempDir::new().unwrap();
    let worktree = TempDir::new().unwrap();
    let mut document = complete_document("runner ${prompt}");
    document = document.replace("impl \"runner ${prompt}\"\n", "");
    write_raw_config(home.path(), &document);
    let delta_path = write_delta(worktree.path(), "impl \"other ${prompt}\"\n");

    let config = load_from(home.path(), worktree.path(), worktree.path()).unwrap();

    assert_eq!(config.source("impl"), None);
    let refusal = format!("{:#}", config.require("impl").unwrap_err());
    assert!(
        refusal.contains(&delta_path.display().to_string()),
        "the refusal must name where the key *is* written, so the reader knows it \
         is in the wrong file rather than misspelled:\n{refusal}"
    );
    assert!(
        refusal.contains(
            &home
                .path()
                .join(".config/grove/config.kdl")
                .display()
                .to_string()
        ),
        "and the file that must declare it:\n{refusal}"
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
        error.contains(&format!("invalid configuration overlay at {display_path}")),
        "{error}"
    );
    assert!(error.contains("duplicate key `impl`"), "{error}");
    assert!(error.contains(&format!("{display_path}:1:1")), "{error}");
    assert!(error.contains(&format!("{display_path}:2:1")), "{error}");
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
        !error.contains("does not resolve"),
        "validation is about the document; resolution is a later question:\n{error}"
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
        error.contains("failed to read the configuration overlay"),
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
        assert!(
            error.contains(&tree.path().join(".grove.kdl").display().to_string()),
            "the refusal must name the delta it refused (colocate={colocate}):\n{error}"
        );
        assert!(
            error.contains("/.grove.kdl"),
            "the refusal must name the ignore line that fixes it \
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
            config.expand("impl", &context("mandate")).unwrap().words(),
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
    // A `.jj` directory with no repository inside it: a jj working tree by the
    // test Grove applies — the marker walk — and one no probe can answer about.
    fs::create_dir(worktree.path().join(".jj")).unwrap();
    write_delta(worktree.path(), "impl \"other ${prompt}\"\n");

    let error = load_error_from(home.path(), worktree.path(), worktree.path());

    assert!(
        error.contains("is tracked"),
        "an unanswerable probe must fail the load, not resolve to the personal file:\n{error}"
    );
}

// `a_tracked_delta_is_refused_under_an_inherited_alternate_git_index` used to sit
// here, with the isolated-child harness that let it install a process-global
// `GIT_INDEX_FILE`. Both went with the Git lane: the hazard was that
// `GIT_INDEX_FILE` selects an *index* independently of the worktree, so anchoring
// the worktree left `git ls-files` reading whatever index the launching process
// chose. jj has no index and no equivalent selector — its probe is pinned by
// `current_dir` and `--ignore-working-copy` — so there is no ambient variable
// left for a regression to reach through. `scrub_internal_child_env` still runs
// on the probe, and `tests/env_hygiene.rs` owns what it removes.

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

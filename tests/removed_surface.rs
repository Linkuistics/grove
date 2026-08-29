//! Surfaces the design removed, asserted as removed — by **enumeration**, not by
//! a list of things to look for.
//!
//! Two of them, sharing one method and nothing else: the **launch-policy
//! environment**, which the routing rework deleted, and the **tree algebra**,
//! which `sweep-k37` deleted once `ordinal-fs-tree` owned it (gh issue #13,
//! increment 2). The second lives at the bottom of the file and is argued
//! there; everything down to it is the first.
//!
//! The obvious way to write this file is a constant holding the sixteen
//! variables the design deleted, checked one by one. That version answers only
//! the question its author already thought to ask: it passes unchanged the day
//! someone reintroduces a seventeenth, and it passes unchanged the day the last
//! real occurrence disappears and the constant becomes decoration. Both failure
//! modes are silent, and both are the *normal* fate of a removal check that
//! outlives the removal.
//!
//! So the candidates come from the tree and the *classification* is what is
//! committed here. Every `GROVE_*` name occurring under `src/`, `tests/`,
//! `scripts/` and the manifests is enumerated, and each must fall into one of
//! the roles below. A name nobody classified fails with its file and line — so
//! a reintroduced variable is caught whether or not anyone remembered to add it
//! here. The check runs in both directions: [`ROLES`]'s exact-name entries must
//! still occur somewhere outside this file, so an entry whose subject is gone
//! fails too and the table cannot quietly become a fossil.
//!
//! Two roles are generative rather than listed — anything ending `_HARNESS` or
//! `_MODEL`, and anything under the `GROVE_HARNESS_BIN` prefix, is removed
//! launch policy by shape. That is what makes a *newly invented* routing
//! variable classify (and then fail containment) without being named.
//!
//! **Source enumeration is only half a removal claim.** A name can be absent
//! from `src/` and the behaviour still live, and a name can be present in `src/`
//! (in the scrub list) and read by nothing. So the other half drives the real
//! bare `grove` process with the whole legacy environment set to values that
//! would be catastrophic if read, and asserts the launch is byte-identical to a
//! clean run — in a Git worktree and in a jj workspace, since the two take
//! different paths through the lifecycle transition before they ever reach a
//! launch. Both halves carry a positive control, because a sweep that cannot
//! fail is worth nothing: the classifier is shown rejecting a reintroduced
//! variable, and the launch fixture is shown observing a configuration-driven
//! change.
//!
//! Out of the sweep's roots by ownership, not by oversight: `content/` (the
//! provisioned methodology) and `docs/` still describe routing variables in
//! prose. Those are documentation surfaces contracted by the methodology and
//! durable-docs work, and pulling them in here would mean either failing on
//! another task's declared scope or writing an exception that misstates who
//! owns them. A companion sweep over the documentation surfaces existed and was
//! deleted: policing which sentences may name a removed token froze the wording
//! of every guide, and the CHANGELOG already records what was removed and when.
//! The claim kept here is the behavioural one — the launch ignores the
//! environment — which no amount of prose drift can falsify.

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use clap::CommandFactory;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Roles

/// What a `GROVE_*` name is *for*. Classification is the committed judgement;
/// the names themselves come from the tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    /// The one capability the driver grants its foreground child: the path it
    /// watches to end the session. Production reads and writes it by design.
    LoopControlChannel,
    /// Deterministic failure/pause seams and fixture handshakes. Internal test
    /// controls, never supported process configuration — which is why
    /// production names them only to scrub them from a configured session.
    InternalTestSeam,
    /// Release scripting's own variables. Not Grove runtime configuration at
    /// all: these steer a shell script a human runs, not a session.
    ReleaseTooling,
    /// Launch policy the design removed. May be named where something scrubs or
    /// refutes it; may not be read.
    RemovedLaunchPolicy,
    /// A source identifier that merely starts with the same prefix — not an
    /// environment name at all. Enumerating lexically is what makes the sweep
    /// exhaustive, and the price of that is naming the collisions.
    NotAnEnvironmentName,
}

/// Exact-name classifications. Every entry must still occur outside this file
/// (see [`the_classification_table_carries_no_stale_entry`]), so this shrinks
/// on its own as the surfaces it names disappear.
const ROLES: &[(&str, Role)] = &[
    ("GROVE_SIGNAL_FILE", Role::LoopControlChannel),
    (
        "GROVE_DRIVER_LEASE_FORK_SENSITIVE_TEST",
        Role::InternalTestSeam,
    ),
    ("GROVE_TAP_DIR", Role::ReleaseTooling),
    ("GROVE_RELEASE_COMMON_SOURCED", Role::ReleaseTooling),
    // Retired pre-watcher handles: the driver used to publish its child's PID
    // instead of watching a path. Named only by the scrub list, because a
    // stale unrelated PID leaking into a nested grove is the same class of
    // mistake one notch quieter.
    ("GROVE_HARNESS_PID", Role::RemovedLaunchPolicy),
    ("GROVE_CLAUDE_PID", Role::RemovedLaunchPolicy),
    // `GROVE_SESSION_TARGET` — exported session-target metadata, deleted with
    // target comparison — is deliberately absent. It reached this table only
    // because the documentation prose sweep that used to live beside this file
    // had to spell the name in order to assert the docs merely refute it; with
    // that sweep gone the name occurs nowhere under these roots, and
    // [`the_classification_table_carries_no_stale_entry`] is what removed it.
    // Reintroducing the name anywhere in the tree fails classification, which is
    // the containment that mattered.
    ("GROVE_LLM_BIN", Role::RemovedLaunchPolicy),
    ("GROVE_SKILL_DIR", Role::RemovedLaunchPolicy),
    ("GROVE_KILL_GRACE", Role::RemovedLaunchPolicy),
    ("GROVE_KILL_GRACE_KILL", Role::RemovedLaunchPolicy),
    ("GROVE_SKILL", Role::NotAnEnvironmentName),
];

/// Classify by shape first, then by name. The shape rules are what let a
/// routing variable nobody has invented yet still classify — and then fail
/// [`no_removed_launch_policy_is_named_outside_the_scrub_module`], which is the
/// assertion that actually bites.
fn role_of(name: &str) -> Option<Role> {
    if name.starts_with("GROVE_TEST_") {
        return Some(Role::InternalTestSeam);
    }
    if name.starts_with("GROVE_HARNESS_BIN")
        || name.ends_with("_HARNESS")
        || name.ends_with("_MODEL")
    {
        return Some(Role::RemovedLaunchPolicy);
    }
    ROLES
        .iter()
        .find(|(known, _)| *known == name)
        .map(|(_, role)| *role)
}

// ---------------------------------------------------------------------------
// Enumeration

#[derive(Clone, Debug)]
struct Occurrence {
    name: String,
    file: PathBuf,
    line: usize,
}

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// This file's own path, relative to the manifest directory. The table lists
/// every name it classifies, so the staleness check has to look past itself or
/// every entry would vouch for itself.
fn this_file() -> PathBuf {
    PathBuf::from(file!())
}

/// The roots the sweep is responsible for: production code, the suite, release
/// scripting, and the manifests that carry ignore rules and build metadata.
fn roots() -> Vec<PathBuf> {
    let base = manifest_dir();
    [
        "src",
        "tests",
        "scripts",
        "build.rs",
        "Cargo.toml",
        "release.toml",
        ".gitignore",
        ".cargo/config.toml",
    ]
    .iter()
    .map(|relative| base.join(relative))
    .collect()
}

/// Every `GROVE_*` occurrence under `path`, recursively.
///
/// This file is skipped. It is the classifier, and its text mentions every name
/// it classifies plus the invented ones its positive control needs — so
/// including it would make the table vouch for itself in one direction and
/// report its own examples as findings in the other. The cost is that a real
/// use added *here* goes unenumerated, which is why nothing in this file uses
/// one.
fn collect(path: &Path, out: &mut Vec<Occurrence>) {
    if path.ends_with(this_file().file_name().unwrap())
        && path.parent().is_some_and(|p| p.ends_with("tests"))
    {
        return;
    }
    if path.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(path)
            .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
            .map(|entry| entry.unwrap().path())
            .collect();
        entries.sort();
        for entry in entries {
            collect(&entry, out);
        }
        return;
    }
    let Ok(text) = fs::read_to_string(path) else {
        return; // binary or unreadable: nothing lexical to classify
    };
    let relative = path.strip_prefix(manifest_dir()).unwrap_or(path);
    for (index, line) in text.lines().enumerate() {
        for name in names_in(line) {
            out.push(Occurrence {
                name,
                file: relative.to_path_buf(),
                line: index + 1,
            });
        }
    }
}

/// The names on one line.
///
/// Two lexical rules, each earned by a false positive the naive scan produced.
/// A match must start at a **non-identifier boundary**, or a fixture's own
/// `OWN_…` constant enumerates as a phantom name no classification could
/// honestly describe. And the bare prefix with nothing after it is dropped: it
/// is how prose writes a wildcard and how `format!` writes a generated name,
/// and in neither case is there a name to classify.
fn names_in(line: &str) -> Vec<String> {
    const PREFIX: &str = "GROVE_";
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(offset) = line[from..].find(PREFIX) {
        let start = from + offset;
        let preceded_by_identifier =
            start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
        let mut end = start + PREFIX.len();
        while end < bytes.len()
            && (bytes[end].is_ascii_uppercase()
                || bytes[end].is_ascii_digit()
                || bytes[end] == b'_')
        {
            end += 1;
        }
        if !preceded_by_identifier && end > start + PREFIX.len() {
            found.push(line[start..end].to_string());
        }
        from = end.max(start + PREFIX.len());
    }
    found
}

fn occurrences() -> Vec<Occurrence> {
    let mut out = Vec::new();
    for root in roots() {
        collect(&root, &mut out);
    }
    assert!(
        !out.is_empty(),
        "the sweep found no GROVE_* name anywhere — the walk is mis-scoped, and a \
         mis-scoped walk reports a clean tree for the wrong reason"
    );
    out
}

// ---------------------------------------------------------------------------
// The sweep

/// The whole point: a name nobody has classified fails, whether or not anyone
/// thought to list it. This is what a fixed pattern list cannot do.
#[test]
fn every_grove_name_in_the_tree_is_classified() {
    let unclassified: Vec<Occurrence> = occurrences()
        .into_iter()
        .filter(|occurrence| role_of(&occurrence.name).is_none())
        .collect();
    assert!(
        unclassified.is_empty(),
        "unclassified GROVE_* names — each is either a reintroduced launch \
         surface or a new internal one that must be given a role in ROLES:\n{}",
        render(&unclassified)
    );
}

/// Removed launch policy may be *named* in production only where production is
/// getting rid of it. Confining that to the one module that owns the scrub list
/// is what keeps "we scrub it" from drifting into "we read it somewhere else
/// too".
///
/// The module moved at `keyed-launch-run-k11`: `src/launch.rs` was absorbed
/// into the loop driver when the spawn and escalation went to the runner, and
/// `LOOP_CONTROL_ENV` — now the runner's `scrub` argument — went with it. The
/// rule is unchanged; only the file that may state the list is.
#[test]
fn no_removed_launch_policy_is_named_outside_the_scrub_module() {
    const SCRUB_MODULE: &str = "src/loop_driver.rs";
    let stray: Vec<Occurrence> = occurrences()
        .into_iter()
        .filter(|occurrence| {
            occurrence.file.starts_with("src")
                && occurrence.file != Path::new(SCRUB_MODULE)
                && role_of(&occurrence.name) == Some(Role::RemovedLaunchPolicy)
        })
        .collect();
    assert!(
        stray.is_empty(),
        "production names removed launch policy outside {SCRUB_MODULE}, whose one \
         job is removing it from a configured session:\n{}",
        render(&stray)
    );
}

/// The reverse direction. Without this the table only grows: an entry whose
/// last occurrence was deleted keeps classifying nothing, and the file slowly
/// becomes a record of surfaces that no longer exist.
#[test]
fn the_classification_table_carries_no_stale_entry() {
    let live: BTreeSet<String> = occurrences()
        .into_iter()
        .filter(|occurrence| occurrence.file != this_file())
        .map(|occurrence| occurrence.name)
        .collect();
    let stale: Vec<&str> = ROLES
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| !live.contains(*name))
        .collect();
    assert!(
        stale.is_empty(),
        "these ROLES entries classify nothing outside this file — the surface is \
         gone, so the entry should go with it: {stale:?}"
    );
}

/// The enumeration's own control, across roots: each root expected to carry
/// `GROVE_*` names must actually yield some. A walk that silently stops at one
/// subtree — a skipped directory, a moved layout — otherwise reports a clean
/// tree, which is the exact false negative this whole file exists to avoid.
///
/// (The *cross-tree* control is the pair of behavioural launches at the foot of
/// this file. "Tree" there means a working tree, Git or jj, as it does
/// everywhere else in this repo; this one is about the roots of one walk.)
#[test]
fn the_walk_reaches_every_root_that_should_carry_names() {
    let found = occurrences();
    for expected in ["src", "tests", "scripts", ".cargo/config.toml"] {
        assert!(
            found
                .iter()
                .any(|occurrence| occurrence.file.starts_with(expected)),
            "the walk reached no GROVE_* name under {expected:?} — it is either \
             mis-scoped or the surface moved without this control being updated"
        );
    }
}

/// Positive control. A classifier is only evidence if it can say no, so this
/// shows it rejecting a variable of a shape nobody listed — and accepting a
/// corpus of live names, so the rejection is discrimination rather than
/// suspicion of everything.
#[test]
fn the_classifier_rejects_a_reintroduced_variable_and_accepts_live_names() {
    assert_eq!(
        role_of("GROVE_PROTOTYPE_MODEL"),
        Some(Role::RemovedLaunchPolicy),
        "a routing variable for a kind nobody has wired yet must still classify \
         as removed policy, by shape"
    );
    assert_eq!(
        role_of("GROVE_DESIGN_HARNESS"),
        Some(Role::RemovedLaunchPolicy)
    );
    assert_eq!(
        role_of("GROVE_SOMETHING_INVENTED"),
        None,
        "a name of no known shape must be reported rather than waved through"
    );

    // ...and the same classifier, over the same shapes production really uses.
    assert_eq!(role_of("GROVE_SIGNAL_FILE"), Some(Role::LoopControlChannel));
    assert_eq!(
        role_of("GROVE_TEST_FINISH_CLEANUP_FAIL_AT"),
        Some(Role::InternalTestSeam)
    );
}

fn render(occurrences: &[Occurrence]) -> String {
    let mut grouped: BTreeMap<&str, Vec<String>> = BTreeMap::new();
    for occurrence in occurrences {
        grouped.entry(&occurrence.name).or_default().push(format!(
            "{}:{}",
            occurrence.file.display(),
            occurrence.line
        ));
    }
    grouped
        .into_iter()
        .map(|(name, sites)| format!("  {name} at {}", sites.join(", ")))
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// The command surface, enumerated from clap's own model

/// Stated as a closure property rather than as a list of rejected verbs: the
/// human CLI has *nothing* to select. That subsumes `do` / `migrate` / `retire`
/// / `--harness` / `--no-launch` without naming them, and it fails on the next
/// flag too — which a list of five rejected argument vectors would not.
#[test]
fn the_human_command_surface_has_nothing_left_to_select() {
    let command = grove::cli::Cli::command();
    let subcommands: Vec<&str> = command.get_subcommands().map(|s| s.get_name()).collect();
    assert!(
        subcommands.is_empty(),
        "bare `grove` is the whole human lifecycle; it has subcommands: {subcommands:?}"
    );
    let arguments: Vec<String> = command
        .get_arguments()
        .map(|argument| argument.get_id().to_string())
        .filter(|id| id != "help" && id != "version")
        .collect();
    assert!(
        arguments.is_empty(),
        "launch policy has one home and it is not the command line; `grove` \
         accepts: {arguments:?}"
    );
}

/// The agent CLI keeps its verbs, so the claim here is narrower: no removed
/// verb came back, and nothing in the whole subtree selects a harness. Walking
/// clap's model rather than scraping `--help` asks the question where it is a
/// fact rather than a rendering (the reasoning `tests/help_surfaces.rs` sets out
/// at length).
///
/// The removed verbs fall into two classes, and the table below records which,
/// because the *reason* is what a reader needs: a **lifecycle** verb was
/// absorbed into bare `grove` and exists elsewhere; a **composition constructor**
/// was deleted outright, and what replaced it is `leaf-add` called by the session
/// that needs the next step (flat-lazy-review). Reintroducing either is a
/// regression, but only one of them has somewhere else to look.
#[test]
fn the_agent_command_surface_exposes_no_removed_verb_or_harness_selector() {
    // Positionals are recorded as `<name>` and flags as `--name`: a verb's
    // operand is not a selector, and conflating the two would make "this verb
    // selects nothing" unstatable for any verb that takes an argument at all.
    fn walk(
        command: &clap::Command,
        path: &str,
        verbs: &mut Vec<String>,
        arguments: &mut Vec<String>,
    ) {
        for argument in command.get_arguments() {
            let id = argument.get_id();
            arguments.push(if argument.is_positional() {
                format!("{path} :: <{id}>")
            } else {
                format!("{path} :: --{id}")
            });
        }
        for sub in command.get_subcommands() {
            verbs.push(sub.get_name().to_string());
            walk(sub, &format!("{path} {}", sub.get_name()), verbs, arguments);
        }
    }
    let mut verbs = Vec::new();
    let mut arguments = Vec::new();
    walk(
        &grove::llm_cli::Cli::command(),
        "grove-llm",
        &mut verbs,
        &mut arguments,
    );

    const REMOVED_VERBS: &[(&str, &str)] = &[
        (
            "do",
            "lifecycle: bare `grove`'s business, removed from both binaries",
        ),
        (
            "migrate",
            "lifecycle: bare `grove`'s business, removed from both binaries",
        ),
        (
            "retire",
            "lifecycle: bare `grove`'s business, removed from both binaries",
        ),
        (
            "leaf-add-chain",
            "composition constructor: a chain's steps are cut one at a time with \
             `leaf-add`, by the session that needs the next one",
        ),
        (
            "leaf-promote-chain",
            "composition constructor: retrofitting a chain node around a picked \
             producer is now one `leaf-add`, so the verb and its `PROMOTING-*` \
             transaction went together",
        ),
    ];
    for (removed, why) in REMOVED_VERBS {
        assert!(
            !verbs.iter().any(|verb| verb == removed),
            "`{removed}` is a removed verb ({why}); grove-llm exposes: {verbs:?}"
        );
    }
    let selectors: Vec<&String> = arguments
        .iter()
        .filter(|argument| argument.ends_with("--harness") || argument.ends_with("--no-launch"))
        .collect();
    assert!(
        selectors.is_empty(),
        "no argument may select launch policy: {selectors:?}"
    );

    // `kind` is the one verb a driver used to *route* from, via
    // `--with-harness --json`. Stated as a closure property on that verb rather
    // than as two rejected flag names: it has nothing left to select, which also
    // fails on the next peek anyone reintroduces. Its leaf-path operand is a
    // positional and so is not in scope; `--help` is clap's own.
    let kind_flags: Vec<&String> = arguments
        .iter()
        .filter(|argument| argument.starts_with("grove-llm kind :: --"))
        .filter(|argument| !argument.ends_with("--help"))
        .collect();
    assert!(
        kind_flags.is_empty(),
        "`kind` reads one filename and prints one token; a launch reads its \
         route from configuration, not from a peek: {kind_flags:?}"
    );
}

// ---------------------------------------------------------------------------
// The behavioural half

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

/// The legacy environment to launch under, with a value per name chosen so that
/// *reading* it would be visible: a harness the configuration never names, a
/// model that would appear in argv, binaries that cannot run, a skill dir that
/// does not exist, graces that would change the watcher's timing.
///
/// Derived from two independent generators rather than typed out — the suite's
/// own kind × family × harness expansion, and this file's enumeration of what
/// the tree still mentions. Neither alone is complete: the expansion produces
/// names no file contains, and the enumeration finds names no expansion
/// predicts. The live control channel is excluded, since granting it is the
/// driver's job and setting it here would end the session under test.
fn removed_environment() -> Vec<(String, String)> {
    let mut names: BTreeSet<String> = support::grove_env_names().into_iter().collect();
    names.extend(
        occurrences()
            .into_iter()
            .filter(|occurrence| role_of(&occurrence.name) == Some(Role::RemovedLaunchPolicy))
            .map(|occurrence| occurrence.name),
    );
    names
        .into_iter()
        .filter(|name| role_of(name) == Some(Role::RemovedLaunchPolicy))
        .map(|name| {
            let value = if name.starts_with("GROVE_HARNESS_BIN") {
                "/definitely/not/a/harness"
            } else if name == "GROVE_LLM_BIN" {
                "/definitely/not/an/agent/cli"
            } else if name == "GROVE_SKILL_DIR" {
                "/definitely/not/a/skill/dir"
            } else if name.starts_with("GROVE_KILL_GRACE") {
                "900"
            } else if name.ends_with("_HARNESS") {
                "pi"
            } else if name.ends_with("_MODEL") {
                "routed-model"
            } else {
                "must-not-be-read"
            };
            (name, value.to_string())
        })
        .collect()
}

/// The two shapes a jj working tree comes in. Grove drives jj only
/// (`docs/adr/jj-is-the-only-lane.md`), so this is the cross-tree axis that
/// remains: a colocated workspace carries a `.git` beside its `.jj`, a native
/// one does not.
#[derive(Clone, Copy, Debug)]
enum Tree {
    Native,
    Colocated,
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

/// One fixture: a working tree of the given VCS holding a single live `impl`
/// leaf, an isolated home carrying a complete config, and a fake command that
/// records `$0` plus its whole argv and then exits so the loop ends.
struct Fixture {
    root: TempDir,
    home: PathBuf,
    worktree: PathBuf,
    argv_log: PathBuf,
}

impl Fixture {
    fn new(tree: Tree) -> Self {
        let root = TempDir::new().unwrap();
        let home = root.path().join("home");
        fs::create_dir_all(home.join(".codex")).unwrap();
        let worktree = root.path().join("worktree");
        fs::create_dir_all(&worktree).unwrap();
        let colocate = matches!(tree, Tree::Colocated);
        run_vcs(
            "jj",
            &worktree,
            &[
                "--config",
                &format!("git.colocate={colocate}"),
                "git",
                "init",
                "--quiet",
                ".",
            ],
        );
        assert_eq!(
            worktree.join(".git").exists(),
            colocate,
            "the fixture must be exactly the shape it names"
        );
        let grove = worktree.join(".grove");
        fs::create_dir_all(&grove).unwrap();
        fs::write(grove.join("BRIEF.md"), "# removed-surface — brief\n").unwrap();
        fs::write(grove.join("01-impl-subject-k1.md"), "# subject-k1\n").unwrap();

        let argv_log = root.path().join("argv-log");
        let configured = root.path().join("configured-command.sh");
        // The mandate prompt is multi-line, so newlines are folded to spaces:
        // one launch must be one comparable line.
        write_executable(
            &configured,
            &format!(
                "#!/bin/sh\n{{ printf '%s|%s' \"$0\" \"$*\" | tr '\\n' ' '; printf '\\n'; }} >> {}\nexit 0\n",
                shell_quote(&argv_log)
            ),
        );
        write_complete_config(
            &home,
            &format!(
                "{} --configured-flag '${{prompt}}'",
                shell_quote(&configured)
            ),
        );

        Self {
            root,
            home,
            worktree,
            argv_log,
        }
    }

    fn run(&self, extra_env: &[(String, String)]) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
        command.current_dir(&self.worktree).env("HOME", &self.home);
        // A subprocess inherits this process's *whole* ambient environment, and
        // this repo dogfoods Grove — so scrub the legacy surface first and let
        // each case put back exactly what it means to test. The scrub covers
        // the same union the routed run sets, or a developer's ambient value
        // would be present in *both* runs and the equality would hold for a
        // reason this test is not making.
        for name in support::grove_env_names()
            .into_iter()
            .chain(removed_environment().into_iter().map(|(name, _)| name))
        {
            command.env_remove(name);
        }
        for (name, value) in extra_env {
            command.env(name, value);
        }
        command.output().unwrap()
    }

    /// The `$0|argv` lines the fake recorded, one per launch. Runs share one
    /// fixture — the same temp paths, the same leaf, the same handle — so two
    /// lines from the same log compare directly, with nothing to normalise.
    fn launch_lines(&self) -> Vec<String> {
        fs::read_to_string(&self.argv_log)
            .unwrap_or_default()
            .lines()
            .map(str::to_string)
            .collect()
    }
}

fn run_vcs(binary: &str, dir: &Path, arguments: &[&str]) {
    let output = Command::new(binary)
        .current_dir(dir)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("running {binary} {arguments:?}: {error}"));
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_launch_is_unchanged_by_the_removed_environment(tree: Tree) {
    let fixture = Fixture::new(tree);

    let control = fixture.run(&[]);
    assert!(
        control.status.success(),
        "control run failed in a {tree:?} tree: {}",
        String::from_utf8_lossy(&control.stderr)
    );

    let environment = removed_environment();
    assert!(
        environment.len() > 20,
        "the generated legacy environment collapsed to {} names — the derivation \
         broke and this test is now asserting almost nothing",
        environment.len()
    );
    let routed = fixture.run(&environment);
    assert!(
        routed.status.success(),
        "a launch under the legacy routing surface must be unaffected, not merely \
         different: {}",
        String::from_utf8_lossy(&routed.stderr)
    );

    let lines = fixture.launch_lines();
    assert_eq!(
        lines.len(),
        2,
        "both runs must reach the configured command in a {tree:?} tree"
    );
    assert!(
        lines[0].contains("--configured-flag"),
        "the control run must reach the configured command: {:?}",
        lines[0]
    );
    assert_eq!(
        lines[0], lines[1],
        "the configured argv must be identical with and without the legacy \
         routing environment"
    );

    // Belt and braces on the equality: the tokens a *live* reader would have
    // introduced, named individually, so a failure says which axis survived
    // rather than only that something differs.
    for token in [
        "--model",
        "--profile",
        "--add-dir",
        "routed-model",
        "/definitely/not/a/harness",
    ] {
        assert!(
            !lines[1].contains(token),
            "{token:?} reached the configured argv: {:?}",
            lines[1]
        );
    }
    let stderr = String::from_utf8_lossy(&routed.stderr);
    assert!(
        !stderr.contains("sandbox"),
        "no sandbox pre-flight survives: {stderr}"
    );
}

#[test]
fn a_configured_launch_in_a_native_jj_tree_ignores_the_removed_environment() {
    assert_launch_is_unchanged_by_the_removed_environment(Tree::Native);
}

/// The cross-tree half. "The configured argv is the whole of launch policy" is a
/// claim about every workspace shape or about none, and a colocated tree is the
/// one carrying a second marker that a resolution could still be tempted by.
#[test]
fn a_configured_launch_in_a_colocated_jj_tree_ignores_the_removed_environment() {
    assert_launch_is_unchanged_by_the_removed_environment(Tree::Colocated);
}

/// The positive control for the behavioural half: the same fixture *does*
/// observe a changed launch, so "identical argv" is a finding rather than an
/// artefact of nothing being watched. The one input still allowed to steer a
/// launch is the configuration, and changing it changes the recorded argv.
#[test]
fn the_launch_fixture_still_observes_a_configuration_driven_change() {
    let fixture = Fixture::new(Tree::Native);
    assert!(fixture.run(&[]).status.success());

    write_complete_config(
        &fixture.home,
        &format!(
            "{} --a-different-flag '${{prompt}}'",
            shell_quote(&fixture.root.path().join("configured-command.sh"))
        ),
    );
    assert!(fixture.run(&[]).status.success());

    let lines = fixture.launch_lines();
    assert_eq!(lines.len(), 2);
    assert!(
        lines[1].contains("--a-different-flag"),
        "the configuration must still steer the launch: {:?}",
        lines[1]
    );
    assert_ne!(
        lines[0], lines[1],
        "the fixture must be able to observe a changed launch"
    );
}

// ---------------------------------------------------------------------------
// The withdrawn tree algebra
//
// `sweep-k37`'s claim, and it is the same *shape* of claim as the one above:
// something the design deleted is gone, and a clean grep is not what shows it.
// grove's tree algebra moved into `ordinal-fs-tree` one verb group at a time
// (gh issue #13, increment 2), and the four modules that carried it were
// deleted once their last caller had gone.
//
// The enumeration is what makes this more than a pattern list. Candidates are
// every module-shaped `tree_*` / `task_*` token occurring anywhere under `src/`
// and `tests/` — code and prose alike, because an essay arguing about a module
// that no longer exists is worse than no essay — and the **live** half of the
// classification comes off disk rather than from a constant, so a module added
// tomorrow classifies without anyone editing this file.

/// The modules deleted from the tree layer — the four `sweep-k37` took with the
/// withdrawn algebra, and the three `delete-migration-k6` took with migration and
/// the format witness.
///
/// Listed, unlike the live set, because absence cannot be enumerated: nothing on
/// disk can tell you the name of a file that is not there. What keeps the list
/// from becoming a fossil is the cross-tree control below — each of these must
/// still be discussed somewhere in `docs/` or the changelog, so an entry naming
/// a module grove never had fails here.
const WITHDRAWN_TREE_MODULES: &[&str] = &[
    "tree_id",
    "tree_read",
    "tree_grow",
    "tree_rename",
    "tree_format",
    "tree_migrate",
    "tree_migration_transaction",
];

/// Every module-shaped `tree_*` / `task_*` token on one line.
///
/// The same two lexical rules [`names_in`] earned: a match starts at a
/// non-identifier boundary, and it runs to the end of the identifier. A token
/// this yields is not necessarily a *module* — `task_template_body` is a
/// function — which is why the classification consults disk and this list, and
/// never assumes shape implies role.
fn module_tokens_in(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    for prefix in ["tree_", "task_"] {
        let mut from = 0;
        while let Some(offset) = line[from..].find(prefix) {
            let start = from + offset;
            let preceded_by_identifier =
                start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
            let mut end = start + prefix.len();
            while end < bytes.len()
                && (bytes[end].is_ascii_lowercase()
                    || bytes[end].is_ascii_digit()
                    || bytes[end] == b'_')
            {
                end += 1;
            }
            if !preceded_by_identifier && end > start + prefix.len() {
                found.push(line[start..end].trim_end_matches('_').to_string());
            }
            from = end.max(start + prefix.len());
        }
    }
    found
}

/// Every module-token occurrence under the given roots, recursively.
///
/// This file is skipped for [`collect`]'s reason: it names all four withdrawn
/// modules, so including it would make the table report itself.
fn module_occurrences(roots: &[PathBuf]) -> Vec<Occurrence> {
    fn walk(path: &Path, out: &mut Vec<Occurrence>) {
        if path.ends_with(this_file().file_name().unwrap())
            && path.parent().is_some_and(|p| p.ends_with("tests"))
        {
            return;
        }
        if path.is_dir() {
            let mut entries: Vec<_> = fs::read_dir(path)
                .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
                .map(|entry| entry.unwrap().path())
                .collect();
            entries.sort();
            for entry in entries {
                walk(&entry, out);
            }
            return;
        }
        let Ok(text) = fs::read_to_string(path) else {
            return;
        };
        let relative = path.strip_prefix(manifest_dir()).unwrap_or(path);
        for (index, line) in text.lines().enumerate() {
            for name in module_tokens_in(line) {
                out.push(Occurrence {
                    name,
                    file: relative.to_path_buf(),
                    line: index + 1,
                });
            }
        }
    }

    let mut out = Vec::new();
    for root in roots {
        walk(root, &mut out);
    }
    out
}

/// The modules `src/` actually holds, read off disk — a `foo.rs` file or a
/// `foo/` directory beside it.
fn live_modules() -> BTreeSet<String> {
    fs::read_dir(manifest_dir().join("src"))
        .expect("src/ must be readable")
        .map(|entry| entry.unwrap().path())
        .filter_map(|path| {
            let name = path.file_name()?.to_str()?.to_owned();
            if path.is_dir() {
                return Some(name);
            }
            name.strip_suffix(".rs").map(str::to_owned)
        })
        .collect()
}

/// The code roots the module sweep is responsible for: production and the suite.
/// `docs/` and `content/` are deliberately **out**, and they are the cross-tree
/// control below rather than an oversight — a withdrawn module is history, and
/// history is what a changelog and a decision record are for.
fn code_roots() -> Vec<PathBuf> {
    let base = manifest_dir();
    ["src", "tests"].iter().map(|r| base.join(r)).collect()
}

#[test]
fn no_withdrawn_tree_module_is_named_anywhere_in_grove_code() {
    let live = live_modules();
    let withdrawn: BTreeSet<&str> = WITHDRAWN_TREE_MODULES.iter().copied().collect();

    let mut findings = Vec::new();
    let mut classified = 0_usize;
    for occurrence in module_occurrences(&code_roots()) {
        if withdrawn.contains(occurrence.name.as_str()) {
            findings.push(occurrence.clone());
        }
        if live.contains(&occurrence.name) || withdrawn.contains(occurrence.name.as_str()) {
            classified += 1;
        }
    }

    assert!(
        classified >= live.len(),
        "the module sweep matched only {classified} module references across \
         {} live modules — the walk is mis-scoped, and a mis-scoped walk reports \
         a clean tree for the wrong reason",
        live.len()
    );
    assert!(
        findings.is_empty(),
        "these name a deleted tree module — the algebra is `ordinal-fs-tree`'s \
         now and migration is gone entirely, and an essay arguing about a module \
         that no longer exists is worse than no essay:\n{}",
        render(&findings)
    );
}

#[test]
fn every_module_file_on_disk_is_declared_and_every_declaration_has_a_file() {
    let live = live_modules();
    let text = fs::read_to_string(manifest_dir().join("src/lib.rs")).unwrap();
    let declared: BTreeSet<String> = text
        .lines()
        .filter_map(|line| {
            let rest = line.trim().strip_prefix("pub mod ").or_else(|| {
                line.trim()
                    .strip_prefix("pub(crate) mod ")
                    .or_else(|| line.trim().strip_prefix("mod "))
            })?;
            rest.strip_suffix(';').map(str::to_owned)
        })
        .collect();

    // `lib.rs` and `main.rs` are the crate roots, not modules of it.
    let expected: BTreeSet<String> = live
        .iter()
        .filter(|name| !matches!(name.as_str(), "lib" | "main" | "bin"))
        .cloned()
        .collect();

    assert_eq!(
        expected, declared,
        "a module file with no declaration is dead weight the compiler never \
         mentions, and a declaration with no file does not build — this is the \
         check that makes `live_modules` reading disk equivalent to reading \
         `lib.rs`, which is what lets the sweep above derive its live set from \
         either"
    );
}

#[test]
fn the_module_sweep_finds_a_withdrawn_module_and_still_finds_it_in_the_docs() {
    // Positive control: the instrument finds the class when the class is there.
    // Without this, a clean sweep and a broken tokeniser are the same reading.
    assert_eq!(
        module_tokens_in("        crate::tree_id::parse(&name)"),
        vec!["tree_id".to_owned()],
        "the tokeniser must find a withdrawn module in a line that carries one"
    );
    assert_eq!(
        module_tokens_in("use crate::task_name::TaskName;"),
        vec!["task_name".to_owned()],
        "and a live one"
    );
    // …and it must not invent one out of an identifier that merely ends that way.
    assert!(
        module_tokens_in("let subtask_grow = 1;").is_empty(),
        "a match must start at an identifier boundary"
    );

    // Cross-tree control: the same tokeniser over the surfaces where a withdrawn
    // module legitimately still lives. `docs/` and the changelog *describe* what
    // grove used to be, so every entry in the table must be found there — which
    // proves the instrument works on the class and keeps the table from becoming
    // a fossil naming modules grove never had.
    let base = manifest_dir();
    let history = module_occurrences(&[base.join("docs"), base.join("CHANGELOG.md")]);
    let found: BTreeSet<&str> = history.iter().map(|o| o.name.as_str()).collect();
    for withdrawn in WITHDRAWN_TREE_MODULES {
        assert!(
            found.contains(withdrawn),
            "{withdrawn} is named nowhere in docs/ or the changelog. Three \
             things look like this and only the last is benign: the sweep's \
             instrument is broken; the table names a module grove never had; or \
             the durable record of the deletion was tidied away, in which case \
             put it back rather than relaxing this — a removal nothing \
             remembers is a removal nothing can check."
        );
    }
}

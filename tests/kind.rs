// Fixture-driven tests for `grove-llm kind` on the current session-kind tree.
// `kind` separates one of the closed nineteen kinds from the leaf filename; it
// never consults historical `**Kind:**` or `**Harness:**` body metadata. It is a
// diagnostic and tree-interface verb — the loop driver selects and reads its own
// leaf in-process, so nothing here routes a launch. With no argument it reads
// `pick`'s next live leaf; on an empty grove it emits the standard "no live
// leaves" diagnostic on stderr and exits 0 (mirroring `brief-chain`). Unknown or
// malformed filename kinds refuse rather than degrading. Each test stands up a
// real git repo so `git rev-parse --show-toplevel` resolves to the fixture path.

mod support;

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Pcmd;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Pcmd::new("git")
        .arg("init")
        .arg(tmp.path())
        .status()
        .unwrap();
    Pcmd::new("git")
        .args(["-C"])
        .arg(tmp.path())
        .args(["commit", "--allow-empty", "-m", "init"])
        .status()
        .unwrap();
    let grove = tmp.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    tmp
}

/// Write a leaf with an inert historical `**Kind:**` line, creating parent dirs.
fn touch_leaf(dir: &Path, name: &str, kind_label: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join(name),
        format!("# stub\n\n**Kind:** {kind_label}\n\n## Goal\n").as_bytes(),
    )
    .unwrap();
}

/// Write a bare file (no `**Kind:**` line), creating parent dirs.
fn touch(dir: &Path, name: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join(name), b"# stub\n").unwrap();
}

/// Create a node directory, returning its path (for nesting children inside).
fn mknode(dir: &Path, name: &str) -> PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    p
}

/// Run the verb with its **whole** stderr attributable to the tree.
///
/// `HOME` is pointed at the fixture repo, which holds no harness root, because
/// `grove-llm` now compares its own methodology identity against every installed
/// skill directory's stamp and warns on disagreement
/// (one-build-owns-a-session). That check reads process-global state this verb
/// otherwise ignores, so against a developer's real home the stderr assertions
/// below would be assertions about *that machine's* installed skill — true on a
/// freshly installed pair and false the moment anyone dogfoods a checkout. An
/// absent root is skipped rather than created, so isolating the home silences it
/// by the ordinary rule rather than by an exception.
fn run(cwd: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(cwd)
        .env("HOME", cwd)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

#[test]
fn kind_of_an_impl_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "impl");

    let (stdout, _, ok) = run(tmp.path(), &["kind", ".grove/01-impl-build-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "impl\n");
}

#[test]
fn kind_of_a_planning_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-planning-design-k1.md", "impl");

    let (stdout, _, ok) = run(tmp.path(), &["kind", ".grove/01-planning-design-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "planning\n");
}

#[test]
fn every_one_of_the_nineteen_round_trips_through_the_verb() {
    // The verb is the loop driver's only view of a leaf's kind, so the whole set
    // has to survive the file → stdout round trip, hyphens and all — a single
    // lowercase token plus a newline, with nothing on stderr.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    for (i, label) in support::KIND_LABELS.iter().enumerate() {
        let name = format!("{:02}-{label}-a-k{}.md", i + 1, i + 1);
        touch_leaf(&grove, &name, "bogus");
        let (stdout, stderr, ok) = run(tmp.path(), &["kind", &format!(".grove/{name}")]);
        assert!(ok, "{label} failed: {stderr:?}");
        assert_eq!(stdout, format!("{label}\n"));
        assert!(stderr.is_empty(), "{label} warned unexpectedly: {stderr:?}");
    }
}

#[test]
fn a_retired_work_body_label_is_ignored() {
    // Historical body routing metadata cannot override the current filename.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "work");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-build-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "impl\n");
    assert!(
        stderr.is_empty(),
        "body metadata must be inert, got {stderr:?}"
    );
}

#[test]
fn no_arg_form_reads_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    let node = mknode(&grove, "01-node-k1");
    touch(&node, "BRIEF.md");
    // pick's next live leaf is the node's first child — a planning leaf.
    touch_leaf(&node, "01-planning-first-k2.md", "impl");

    let (stdout, _, ok) = run(tmp.path(), &["kind"]);
    assert!(ok);
    assert_eq!(stdout, "planning\n");
}

#[test]
fn empty_grove_prints_no_live_leaves_on_stderr_and_exits_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md"); // brief only — no live leaf

    let (stdout, stderr, ok) = run(tmp.path(), &["kind"]);
    assert!(ok, "empty grove must exit 0");
    assert!(stdout.is_empty(), "stdout must be empty, got {stdout:?}");
    assert!(
        stderr.contains("no live leaves"),
        "expected the standard diagnostic, got {stderr:?}"
    );
}

#[test]
fn a_body_with_no_kind_line_does_not_affect_the_filename_kind() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-broken-k1.md"); // `# stub` only — no `**Kind:**` line

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-broken-k1.md"]);
    assert!(ok, "the filename supplies the kind");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty(), "body metadata is ignored: {stderr:?}");
}

#[test]
fn a_garbled_body_kind_is_ignored() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-broken-k1.md", "sideways");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-broken-k1.md"]);
    assert!(ok, "body kind tokens are not parsed");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty(), "body metadata is ignored: {stderr:?}");
}

#[test]
fn a_family_name_written_in_the_body_is_ignored() {
    // `review` and `integrate-review` are routing *families*, not members of the
    // set. On a leaf they are unrecognised — a naive prefix match would quietly
    // pick one of the five `review-*` kinds and misroute the session.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    for (name, label) in [
        ("01-impl-a-k1.md", "review"),
        ("02-impl-b-k2.md", "integrate-review"),
    ] {
        touch_leaf(&grove, name, label);
        let (stdout, stderr, ok) = run(tmp.path(), &["kind", &format!(".grove/{name}")]);
        assert!(ok, "{label} is body text, not routing input");
        assert_eq!(stdout, "impl\n", "{label} must not match a review-* kind");
        assert!(stderr.is_empty(), "{label} must not warn: {stderr:?}");
    }
}

#[test]
fn a_typoed_body_kind_is_ignored_with_exit_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-broken-k1.md", "reserch");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-broken-k1.md"]);
    assert!(ok, "exit 0 even on a typo'd kind");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty(), "body metadata is ignored: {stderr:?}");
}

// ── The historical `**Harness:**` line ────────────────────────────────────
//
// `kind` once grew `--with-harness` and `--json` so the loop driver could peek
// a leaf's route before launching it. Routing is now one configuration lookup
// keyed on the filename kind, the driver selects in-process, and both flags are
// gone — so what is left to assert is that the body line they were invented to
// read is *inert*, on the one form that remains.

/// Write a leaf that declares both a kind and a harness, in the shape the
/// retired leaf template wrote them — the `**Harness:**` line immediately under
/// `**Kind:**`.
fn touch_leaf_with_harness(dir: &Path, name: &str, kind_label: &str, harness: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join(name),
        format!("# stub\n\n**Kind:** {kind_label}\n**Harness:** {harness}\n\n## Goal\n").as_bytes(),
    )
    .unwrap();
}

#[test]
fn the_kind_verb_never_reads_the_harness_line() {
    // A historical `**Harness:**` line is inert; the filename remains the only
    // source of task kind.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-research-a-survey-k1.md", "impl", "codx");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-research-a-survey-k1.md"]);
    assert!(ok, "the verb must not gate on the harness line");
    assert_eq!(stdout, "research-a\n");
    assert!(stderr.is_empty(), "and must not warn about it: {stderr:?}");
}

#[test]
fn an_unrecognised_body_harness_is_ignored() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-impl-survey-k1.md", "research", "codx");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-survey-k1.md"]);
    assert!(ok, "body harness metadata is not parsed: {stderr:?}");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty());
}

#[test]
fn a_malformed_or_annotated_body_harness_line_is_ignored() {
    // Even malformed or commented historical routing metadata is inert.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    for (name, body) in [
        (
            "01-impl-empty-k1.md",
            "# stub\n\n**Kind:** research\n**Harness:**\n",
        ),
        (
            "02-impl-annotated-k2.md",
            "# stub\n\n**Kind:** research\n**Harness:** codex   (the pair's second survey)\n",
        ),
    ] {
        fs::write(grove.join(name), body.as_bytes()).unwrap();

        let (stdout, stderr, ok) = run(tmp.path(), &["kind", &format!(".grove/{name}")]);
        assert!(
            ok,
            "{name}: body harness metadata is not parsed: {stderr:?}"
        );
        assert_eq!(stdout, "impl\n", "{name}");
        assert!(stderr.is_empty(), "{name}: {stderr:?}");
    }
}

#[test]
fn the_removed_routing_flags_are_rejected() {
    // Stated on the process rather than on clap's model, because the claim that
    // matters to a caller is that the *invocation* fails — a driver still
    // passing the old peek must break loudly rather than silently receive a
    // plain kind line it would misparse.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "impl");

    for flags in [
        vec!["kind", "--with-harness"],
        vec!["kind", "--json"],
        vec!["kind", "--with-harness", "--json"],
    ] {
        let (stdout, stderr, ok) = run(tmp.path(), &flags);
        assert!(!ok, "{flags:?} must not succeed, got {stdout:?}");
        assert!(
            stderr.contains("unexpected argument"),
            "{flags:?} must be rejected as an unknown argument: {stderr:?}"
        );
    }
}

#[test]
fn kind_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("kind"), "grove-llm --help missing kind: {s}");
}

#[test]
fn grove_help_does_not_list_kind() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("Print a leaf's task"),
        "grove --help leaked the kind verb from the LLM surface: {s}"
    );
}

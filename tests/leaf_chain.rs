// The composite grow verbs — `grove-llm leaf-add-chain` and `leaf-add-pair` —
// exercised through the real binary (`docs/specs/task-kind-taxonomy.md`,
// *Constructing a chain is one call*).
//
// The library owns the shape and its all-or-nothing contract (`src/tree_grow.rs`
// unit tests). What only the binary can show is the *command* contract, which is
// where the review's F1 landed:
//
//   * **stdout is the shape or it is nothing** — a failed run prints no path at
//     all, because the run was rolled back and stdout describing files that are
//     no longer there is worse than silence;
//   * the refusals **teach**, at the one moment a session is deciding — a
//     `--harness` on a chain names the policy var, a `--kind` on a pair names
//     the other verb;
//   * both verbs sit beside `leaf-add` in `--help`, the bootstrap-recovery
//     surface. That placement is the whole answer to *compose-task-chains-k29*'s
//     failure mode (five documented surfaces, zero chains): a verb nobody
//     reaches for is that failure with a compile step.

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// A real git worktree holding a `.grove/` with only its root brief. The verbs
/// write untracked files and never shell out to a VCS, but every `grove-llm`
/// verb resolves its grove root from a repo, so the fixture is a repo.
fn grove() -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .arg("-q")
        .arg(tmp.path())
        .status()
        .unwrap();
    let root = tmp.path().join(".grove");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("BRIEF.md"), "# root — brief\n").unwrap();
    tmp
}

fn run(worktree: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(worktree)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// The grove's entry names, lexically sorted.
fn tree(worktree: &Path) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(worktree.join(".grove"))
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

fn body(worktree: &Path, name: &str) -> String {
    fs::read_to_string(worktree.join(".grove").join(name)).unwrap()
}

#[test]
fn chain_prints_its_three_paths_in_position_order() {
    let t = grove();
    let (stdout, _, ok) = run(
        t.path(),
        &["leaf-add-chain", ".", "sync", "--kind", "design"],
    );
    assert!(ok, "chain should succeed: {stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "one absolute path per leaf: {stdout:?}");
    for (line, expected) in lines.iter().zip([
        "01-sync-k1.md",
        "02-sync-review-k2.md",
        "03-sync-integrate-k3.md",
    ]) {
        assert!(
            line.ends_with(expected) && line.starts_with('/'),
            "expected an absolute path ending {expected}, got {line:?}"
        );
    }
}

#[test]
fn pair_prints_its_three_paths_and_declares_both_vendors() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "leaf-add-pair",
            ".",
            "survey",
            "--harness-a",
            "claude",
            "--harness-b",
            "codex",
        ],
    );
    assert!(ok, "pair should succeed: {stdout} {stderr}");
    assert_eq!(stdout.lines().count(), 3, "{stdout:?}");
    assert!(body(t.path(), "01-survey-a-k1.md").contains("**Harness:** claude"));
    assert!(body(t.path(), "02-survey-b-k2.md").contains("**Harness:** codex"));
    assert!(!body(t.path(), "03-survey-combine-k3.md").contains("**Harness:**"));
}

#[test]
fn a_failed_run_prints_no_path_at_all() {
    // F1's stdout half. Printing as each leaf lands would let stdout describe a
    // mutation the command reports as failed — and here the run is rolled back,
    // so those paths do not exist by the time the caller reads them.
    //
    // The obstruction is a *directory* wearing the third leaf's name: the tree
    // reconciles parsed names against real filesystem kinds, so it is invisible
    // to position and key allocation and still blocks the write.
    let t = grove();
    fs::create_dir(t.path().join(".grove").join("03-sync-integrate-k3.md")).unwrap();

    let (stdout, stderr, ok) = run(
        t.path(),
        &["leaf-add-chain", ".", "sync", "--kind", "design"],
    );

    assert!(!ok, "the run must fail");
    assert_eq!(
        stdout, "",
        "not one path on stdout for a shape that was not created"
    );
    assert!(
        stderr.contains("nothing was created"),
        "the diagnostic says what the tree holds: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["03-sync-integrate-k3.md", "BRIEF.md"],
        "no live prefix of a chain left behind"
    );
}

#[test]
fn chain_refuses_a_harness_and_names_the_policy_var() {
    // Routing reviews is a policy, not a per-leaf fact. clap's bare "unexpected
    // argument" would refuse correctly and teach nothing, which is the whole
    // reason the flag is accepted-then-refused rather than absent.
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "leaf-add-chain",
            ".",
            "sync",
            "--kind",
            "design",
            "--harness",
            "codex",
        ],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("GROVE_REVIEW_HARNESS"),
        "the refusal names the mechanism that *does* express this: {stderr}"
    );
    assert_eq!(tree(t.path()), vec!["BRIEF.md"], "nothing created");
}

#[test]
fn pair_refuses_a_kind_and_names_the_chain_verb() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "leaf-add-pair",
            ".",
            "survey",
            "--harness-a",
            "claude",
            "--harness-b",
            "codex",
            "--kind",
            "design",
        ],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("leaf-add-chain"),
        "the refusals point at each other: {stderr}"
    );
    assert_eq!(tree(t.path()), vec!["BRIEF.md"], "nothing created");
}

#[test]
fn chain_requires_a_producer_kind_rather_than_defaulting_to_impl() {
    // `leaf-add`'s `impl` default would silently choose the producer — exactly
    // the wrong-but-well-formed kind this verb exists to stop a session picking
    // by accident.
    let t = grove();
    let (stdout, _, ok) = run(t.path(), &["leaf-add-chain", ".", "sync"]);
    assert!(!ok, "--kind is required");
    assert_eq!(stdout, "");
    assert_eq!(tree(t.path()), vec!["BRIEF.md"]);
}

#[test]
fn pair_requires_both_vendors_named() {
    let t = grove();
    for args in [
        vec!["leaf-add-pair", ".", "survey"],
        vec!["leaf-add-pair", ".", "survey", "--harness-a", "claude"],
        vec!["leaf-add-pair", ".", "survey", "--harness-b", "codex"],
    ] {
        let (stdout, _, ok) = run(t.path(), &args);
        assert!(!ok, "{args:?} should be refused");
        assert_eq!(stdout, "", "{args:?}");
    }
    assert_eq!(tree(t.path()), vec!["BRIEF.md"]);
}

#[test]
fn pair_refuses_one_vendor_named_twice() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "leaf-add-pair",
            ".",
            "survey",
            "--harness-a",
            "codex",
            "--harness-b",
            "codex",
        ],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("two vendors"),
        "the refusal names the property the pair exists for: {stderr}"
    );
    assert_eq!(tree(t.path()), vec!["BRIEF.md"], "nothing created");
}

#[test]
fn pair_refuses_an_unknown_vendor_before_writing() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "leaf-add-pair",
            ".",
            "survey",
            "--harness-a",
            "claude",
            "--harness-b",
            "gemeni",
        ],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(stderr.contains("gemeni"), "{stderr}");
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md"],
        "a typo in the *second* vendor must not leave the first survey on disk"
    );
}

#[test]
fn chain_refuses_research_and_sends_the_caller_to_the_pair_verb() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &["leaf-add-chain", ".", "survey", "--kind", "research"],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("leaf-add-pair"),
        "research has a shape; it is the other verb's: {stderr}"
    );
    assert_eq!(tree(t.path()), vec!["BRIEF.md"], "nothing created");
}

#[test]
fn both_verbs_are_listed_beside_leaf_add_in_help() {
    // The review's one conditional verdict: the design's answer to "how does a
    // session come to use it" is *placement*, and `--help` is the surface a
    // session re-orients on. A verb absent from it is k29's failure repeated.
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    for verb in ["leaf-add", "leaf-add-chain", "leaf-add-pair"] {
        assert!(
            s.contains(verb),
            "`{verb}` missing from grove-llm --help: {s}"
        );
    }
    let at = |v: &str| s.find(v).unwrap();
    assert!(
        at("leaf-add") < at("leaf-add-chain") && at("leaf-add-chain") < at("leaf-add-pair"),
        "the composite verbs sit immediately after the verb they compose: {s}"
    );
}

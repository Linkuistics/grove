// The composite grow verbs — `grove-llm leaf-add-chain` and `leaf-add-pair` —
// exercised through the real binary (`docs/ARCHITECTURE.md#task-kind-taxonomy`,
// *Constructing a chain is one call*).
//
// The library owns the shape and its all-or-nothing contract (`src/tree_grow.rs`
// unit tests). What only the binary can show is the *command* contract, which is
// where chain-construction-review-k39's F1 landed:
//
//   * **stdout is the shape or it is nothing** — four absolute paths, the chain
//     node's directory first so a caller can `leaf-add <node>` a late step
//     straight into it; and a failed run prints no path at all, because the run
//     was rolled back and stdout describing files that are no longer there is
//     worse than silence;
//   * routing metadata is absent — chain kind lives in filenames and the pair
//     has fixed research-a/research-b/combine-research kinds;
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
    fs::write(root.join("FORMAT"), "session-kinds-v1\n").unwrap();
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
fn chain_prints_its_node_then_its_three_steps() {
    let t = grove();
    let (stdout, _, ok) = run(
        t.path(),
        &["leaf-add-chain", ".", "sync", "--kind", "design"],
    );
    assert!(ok, "chain should succeed: {stdout}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(
        lines.len(),
        4,
        "the node directory then one path per step: {stdout:?}"
    );
    for (line, expected) in lines.iter().zip([
        "01-sync-chain-k1",
        "01-sync-chain-k1/01-design-sync-k2.md",
        "01-sync-chain-k1/02-review-design-sync-review-k3.md",
        "01-sync-chain-k1/03-integrate-review-design-sync-integrate-k4.md",
    ]) {
        assert!(
            line.ends_with(expected) && line.starts_with('/'),
            "expected an absolute path ending {expected}, got {line:?}"
        );
    }
}

#[test]
fn pair_prints_its_node_and_fixed_research_kinds() {
    let t = grove();
    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", ".", "survey"]);
    assert!(ok, "pair should succeed: {stdout} {stderr}");
    assert_eq!(stdout.lines().count(), 4, "{stdout:?}");
    let node = "01-survey-pair-k1";
    assert!(
        !t.path().join(".grove").join(node).join("BRIEF.md").exists(),
        "a chain node is brief-less by rule — the Retire cascade's discriminator"
    );
    for name in [
        format!("{node}/01-research-a-survey-a-k2.md"),
        format!("{node}/02-research-b-survey-b-k3.md"),
        format!("{node}/03-combine-research-survey-combine-k4.md"),
    ] {
        let contents = body(t.path(), &name);
        assert!(!contents.contains("**Harness:**"), "got {contents:?}");
        assert!(!contents.contains("**Kind:**"), "got {contents:?}");
    }
}

#[test]
fn the_printed_node_path_is_a_parent_leaf_add_accepts() {
    // Why the node's path leads stdout rather than being implied: the shape's
    // one durable affordance is `leaf-add <node> <stem>-late-step`, which lands a
    // step decided on afterwards *inside* the chain instead of behind every
    // unrelated live leaf. Piping the first line straight back in is the whole
    // point, and it only works because a chain node's missing `BRIEF.md` does not
    // disqualify it as a parent.
    let t = grove();
    let (stdout, _, ok) = run(
        t.path(),
        &["leaf-add-chain", ".", "sync", "--kind", "design"],
    );
    assert!(ok, "{stdout}");
    let node = stdout.lines().next().unwrap();

    let (added, stderr, ok) = run(t.path(), &["leaf-add", node, "sync-second-review"]);

    assert!(ok, "leaf-add into the chain node should succeed: {stderr}");
    assert!(
        added
            .trim()
            .ends_with("01-sync-chain-k1/04-impl-sync-second-review-k5.md"),
        "the late step lands inside the node, after its stem-mates: {added:?}"
    );
}

#[test]
fn a_failed_run_prints_no_path_at_all() {
    // chain-construction-review-k39 F1's stdout half. Printing as each path
    // lands would let stdout describe a mutation the command reports as failed —
    // and here the run is rolled back, so those paths do not exist by the time
    // the caller reads them.
    //
    // The obstruction is a *file* wearing the node's name: the tree reconciles
    // parsed names against real filesystem kinds, so it is invisible to position
    // and key allocation and still blocks the directory.
    let t = grove();
    fs::write(
        t.path().join(".grove").join("01-sync-chain-k1"),
        "not a node\n",
    )
    .unwrap();

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
        vec!["01-sync-chain-k1", "BRIEF.md", "FORMAT"],
        "no half-built chain node left behind"
    );
}

#[test]
fn chain_rejects_the_removed_harness_flag() {
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
        stderr.contains("unexpected argument '--harness'"),
        "the removed surface must be absent: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "nothing created"
    );
}

#[test]
fn pair_refuses_a_kind_because_its_kinds_are_fixed() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &["leaf-add-pair", ".", "survey", "--kind", "design"],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("unexpected argument '--kind'"),
        "the pair has no configurable kind: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "nothing created"
    );
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
    assert_eq!(tree(t.path()), vec!["BRIEF.md", "FORMAT"]);
}

#[test]
fn pair_help_exposes_no_routing_flags() {
    let t = grove();
    let (stdout, _, ok) = run(t.path(), &["leaf-add-pair", "--help"]);
    assert!(ok);
    assert!(!stdout.contains("harness"), "got {stdout:?}");
    assert!(!stdout.contains("--kind"), "got {stdout:?}");
}

#[test]
fn pair_rejects_removed_harness_flags() {
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
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("unexpected argument '--harness-a'"),
        "{stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "nothing created"
    );
}

#[test]
fn pair_requires_parent_and_stem() {
    let t = grove();
    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", "."]);
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(stderr.contains("<STEM>"), "{stderr}");
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "a malformed command must not leave a partial pair on disk"
    );
}

#[test]
fn chain_refuses_a_fixed_pair_step_as_a_producer() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &["leaf-add-chain", ".", "survey", "--kind", "research-a"],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("leaf-add-pair"),
        "a pair step cannot head a review chain: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "nothing created"
    );
}

#[test]
fn both_verbs_are_listed_beside_leaf_add_in_help() {
    // chain-construction-review-k39's one conditional verdict: the design's
    // answer to "how does a session come to use it" is *placement*, and `--help`
    // is the surface a session re-orients on. A verb absent from it is
    // compose-task-chains-k29's failure repeated.
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

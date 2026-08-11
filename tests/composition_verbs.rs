// How a grove composes several leaves into one artifact, exercised through the
// real binaries. Two shapes, and after flat-lazy-review they are built by two
// different mechanisms:
//
//   * a **review chain** is `leaf-add`, three times, spread across three
//     sessions — the producer cuts its review only if review is required, and
//     the review cuts its integration only if it found something. There is no
//     chain verb and no chain node; the steps are flat siblings and the only
//     thing that groups them is a shared stem. So what the binary can be held to
//     is exactly what `leaf-add` does: append at the parent's next free
//     position, with a body the *creating session* is free to write.
//   * a **research pair** is still one call, because its two producers must not
//     see each other's framing. The library owns the shape and its
//     all-or-nothing contract (`src/tree_grow.rs` unit tests); what only the
//     binary can show is the *command* contract — stdout is the shape or it is
//     nothing, and the verb sits beside `leaf-add` in `--help`, the surface a
//     session re-orients on when it drops context.

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

// ---------------------------------------------------------------------------
// The review chain: three ordinary `leaf-add`s, cut lazily

#[test]
fn a_review_chain_is_three_leaf_adds_landing_as_flat_siblings() {
    // The command-level shape of the whole workstream. Nothing here names a
    // chain: each call is the ordinary append a session makes as its last act,
    // and what makes the three a chain is the shared stem plus the convention
    // the sessions write into the bodies themselves.
    let t = grove();
    for (slug, kind, expected) in [
        ("sync", "design", "01-design-sync-k1.md"),
        (
            "sync-review",
            "review-design",
            "02-review-design-sync-review-k2.md",
        ),
        (
            "sync-integrate",
            "integrate-review-design",
            "03-integrate-review-design-sync-integrate-k3.md",
        ),
    ] {
        let (stdout, stderr, ok) = run(t.path(), &["leaf-add", ".", slug, "--kind", kind]);
        assert!(ok, "leaf-add {slug} failed: {stderr}");
        assert!(
            stdout.trim().ends_with(expected) && stdout.starts_with('/'),
            "expected an absolute path ending {expected}, got {stdout:?}"
        );
    }
    assert_eq!(
        tree(t.path()),
        vec![
            "01-design-sync-k1.md",
            "02-review-design-sync-review-k2.md",
            "03-integrate-review-design-sync-integrate-k3.md",
            "BRIEF.md",
            "FORMAT",
        ],
        "flat siblings — no chain node was created for them"
    );
}

#[test]
fn a_freshly_added_leaf_carries_an_empty_body_for_its_creator_to_write() {
    // Why laziness pays: the creating session writes the new leaf's body, so
    // the template must not pre-empt it with a rendered goal or a relationship
    // line it would have to edit around. It gets the stable handle and empty
    // sections, and nothing else.
    let t = grove();
    let (_, stderr, ok) = run(
        t.path(),
        &["leaf-add", ".", "sync-review", "--kind", "review-design"],
    );
    assert!(ok, "{stderr}");

    let contents = body(t.path(), "01-review-design-sync-review-k1.md");

    assert!(
        contents.starts_with("# sync-review-k1\n"),
        "the position-free handle heads the body: {contents:?}"
    );
    for absent in [
        "**Reviews:**",
        "**Integrates:**",
        "**Kind:**",
        "**Harness:**",
        "Adversarially review",
    ] {
        assert!(
            !contents.contains(absent),
            "the template must leave {absent:?} to the creating session: {contents:?}"
        );
    }
}

#[test]
fn there_is_no_chain_constructor_left_to_call() {
    // Both eager verbs are gone, and their absence has to be a *refusal* rather
    // than a silent success against some near-neighbour. A session carrying the
    // old habit gets clap's unrecognised-subcommand error and an untouched tree.
    for argv in [
        vec!["leaf-add-chain", ".", "sync", "--kind", "design"],
        vec!["leaf-promote-chain", "1"],
    ] {
        let t = grove();
        let (stdout, stderr, ok) = run(t.path(), &argv);
        assert!(!ok, "{argv:?} was accepted");
        assert_eq!(stdout, "", "{argv:?} printed a path");
        assert!(
            stderr.contains("unrecognized subcommand"),
            "{argv:?}: {stderr}"
        );
        assert_eq!(
            tree(t.path()),
            vec!["BRIEF.md", "FORMAT"],
            "{argv:?} touched the tree"
        );
    }
}

/// The release's central compatibility promise, pinned by the one shape that
/// can regress it silently: **a chain node left by the old constructor**.
///
/// Nothing was migrated, and nothing needed to be — a chain node was only ever
/// an ordinary node *directory* whose slug ended in `-chain`, and every reader
/// handles node directories generically. But that argument is only as good as
/// the readers, and after the deletion no source or test fixture contained a
/// `-chain-k` name at all: the whole current-shape suite would keep passing if
/// a future change made node parsing, `pick`'s descent, handle resolution or
/// `brief-chain` assume a charter that this node, uniquely, does not have.
///
/// So the fixture is deliberately the *awkward* legacy case — a brief-less node
/// holding a live producer and its two steps, in current filename format,
/// exactly as `leaf-add-chain` would have written it — and all three read paths
/// are exercised against it untouched.
#[test]
fn an_unmigrated_chain_node_still_picks_resolves_and_walks_its_brief_chain() {
    let t = grove();
    let node = t.path().join(".grove/01-sync-chain-k1");
    fs::create_dir_all(&node).unwrap();
    // No `BRIEF.md` — the property under test. The three steps carry the stem
    // suffixes and derived kinds the deleted constructor emitted.
    for (name, header) in [
        ("01-design-sync-k2.md", "sync-k2"),
        ("02-review-design-sync-review-k3.md", "sync-review-k3"),
        (
            "03-integrate-review-design-sync-integrate-k4.md",
            "sync-integrate-k4",
        ),
    ] {
        fs::write(node.join(name), format!("# {header}\n")).unwrap();
    }

    // `pick` descends the node in pre-order and reaches its first live leaf,
    // rather than skipping a directory it cannot charter.
    let (stdout, stderr, ok) = run(t.path(), &["pick"]);
    assert!(ok, "pick failed on a legacy chain node: {stderr}");
    assert!(
        stdout
            .trim()
            .ends_with("01-sync-chain-k1/01-design-sync-k2.md"),
        "pick must descend the unmigrated node, got {stdout:?}"
    );

    // The children resolve by their permanent handles, which the node never
    // renumbered and no migration rewrote.
    for (handle, expected) in [
        ("sync-k2", "01-sync-chain-k1/01-design-sync-k2.md"),
        (
            "sync-review-k3",
            "01-sync-chain-k1/02-review-design-sync-review-k3.md",
        ),
    ] {
        let (stdout, stderr, ok) = run(t.path(), &["resolve", handle]);
        assert!(ok, "resolve {handle} failed: {stderr}");
        assert!(
            stdout.trim().ends_with(expected),
            "resolve {handle} gave {stdout:?}"
        );
    }

    // `brief-chain` skips the level with no charter silently and yields the root
    // brief alone — which is exactly what the close of such a node reports:
    // there is no `Done when` to check and nothing to promote.
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "brief-chain",
            ".grove/01-sync-chain-k1/01-design-sync-k2.md",
        ],
    );
    assert!(ok, "brief-chain failed: {stderr}");
    let briefs: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        briefs.len(),
        1,
        "expected the root brief alone, got {briefs:?}"
    );
    assert!(briefs[0].ends_with(".grove/BRIEF.md"), "got {briefs:?}");

    // And none of it moved anything: no migration ran.
    let mut names: Vec<String> = fs::read_dir(&node)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(
        names,
        vec![
            "01-design-sync-k2.md",
            "02-review-design-sync-review-k3.md",
            "03-integrate-review-design-sync-integrate-k4.md",
        ],
        "the legacy node must be untouched — still brief-less, still `-chain`"
    );
}

// ---------------------------------------------------------------------------
// The research pair: still one call

#[test]
fn pair_prints_three_flat_siblings_with_fixed_research_kinds() {
    let t = grove();
    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", ".", "survey"]);
    assert!(ok, "pair should succeed: {stdout} {stderr}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "one path per step: {stdout:?}");
    for (line, expected) in lines.iter().zip([
        "01-research-a-survey-a-k1.md",
        "02-research-b-survey-b-k2.md",
        "03-combine-research-survey-combine-k3.md",
    ]) {
        assert!(
            line.ends_with(expected) && line.starts_with('/'),
            "expected an absolute path ending {expected}, got {line:?}"
        );
        let contents = body(t.path(), expected);
        assert!(!contents.contains("**Harness:**"), "got {contents:?}");
        assert!(!contents.contains("**Kind:**"), "got {contents:?}");
    }
}

#[test]
fn a_failed_run_prints_no_path_at_all() {
    // Printing as each path lands would let stdout describe a mutation the
    // command reports as failed — and the run is rolled back, so those paths do
    // not exist by the time the caller reads them.
    //
    // The obstruction is a *directory* wearing a leaf's name: task-shaped, but
    // not the species its `.md` suffix declares, so the read that precedes
    // allocation refuses it. The run therefore fails after validating and
    // before writing, which is the arm stdout silence has to survive.
    let t = grove();
    fs::create_dir(t.path().join(".grove").join("01-research-a-survey-a-k1.md")).unwrap();

    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", ".", "survey"]);

    assert!(!ok, "the run must fail");
    assert_eq!(
        stdout, "",
        "not one path on stdout for a shape that was not created"
    );
    assert!(
        stderr.contains("01-research-a-survey-a-k1.md"),
        "the diagnostic names the entry standing in the way: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["01-research-a-survey-a-k1.md", "BRIEF.md", "FORMAT"],
        "no half-built pair left behind"
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
fn the_surviving_composite_verb_sits_beside_leaf_add_in_help() {
    // compose-task-chains-k29's failure mode was five documented surfaces and
    // zero chains: a verb nobody reaches for is that failure with a compile
    // step. `--help` is the surface a session re-orients on, so the one verb a
    // session cannot express with `leaf-add` has to be visible there, next to
    // the verb it composes.
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    for verb in ["leaf-add", "leaf-add-pair"] {
        assert!(
            s.contains(verb),
            "`{verb}` missing from grove-llm --help: {s}"
        );
    }
    for gone in ["leaf-add-chain", "leaf-promote-chain"] {
        assert!(
            !s.contains(gone),
            "`{gone}` no longer exists and must not be advertised: {s}"
        );
    }
    let at = |v: &str| s.find(v).unwrap();
    assert!(
        at("leaf-add") < at("leaf-add-pair"),
        "the composite verb sits immediately after the verb it composes: {s}"
    );
}

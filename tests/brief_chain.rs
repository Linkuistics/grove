// Fixture-driven tests for `grove-llm brief-chain`. Exercises the three shapes
// the leaf's brief calls out explicitly (leaf at grove root, leaf two levels
// deep, leaf whose intermediate node has no BRIEF.md), plus the no-arg form
// composing with `pick`, the relative-path acceptance, and the audience-split
// assertion (verb on grove-llm, not on grove). Each test stands up a real git
// repo so `git rev-parse --show-toplevel` resolves to the fixture.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Pcmd;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Pcmd::new("git").arg("init").arg(tmp.path()).status().unwrap();
    Pcmd::new("git")
        .args(["-C"])
        .arg(tmp.path())
        .args(["commit", "--allow-empty", "-m", "init"])
        .status()
        .unwrap();
    tmp
}

fn touch(p: &Path) {
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(p, b"# stub\n").unwrap();
}

fn brief_chain(cwd: &Path, args: &[&str]) -> (String, String, bool) {
    let mut cmd = Command::cargo_bin("grove-llm").unwrap();
    cmd.current_dir(cwd).arg("brief-chain");
    for a in args {
        cmd.arg(a);
    }
    let out = cmd.output().unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

fn rel_lines(stdout: &str, root: &Path) -> Vec<PathBuf> {
    let root = root.canonicalize().unwrap();
    stdout
        .lines()
        .map(|line| {
            let abs = PathBuf::from(line).canonicalize().unwrap();
            abs.strip_prefix(&root).unwrap().to_path_buf()
        })
        .collect()
}

#[test]
fn leaf_at_grove_root_returns_only_root_brief() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("010-leaf.md"));

    let leaf = grove.join("010-leaf.md");
    let leaf_s = leaf.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = brief_chain(tmp.path(), &[&leaf_s]);
    assert!(ok, "stderr={stderr}");
    let rels = rel_lines(&stdout, tmp.path());
    assert_eq!(rels, vec![PathBuf::from(".grove/BRIEF.md")]);
}

#[test]
fn leaf_two_levels_deep_returns_root_middle_node_briefs() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("020-mid/BRIEF.md"));
    touch(&grove.join("020-mid/030-node/BRIEF.md"));
    touch(&grove.join("020-mid/030-node/040-leaf.md"));

    let leaf = grove.join("020-mid/030-node/040-leaf.md");
    let leaf_s = leaf.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = brief_chain(tmp.path(), &[&leaf_s]);
    assert!(ok, "stderr={stderr}");
    let rels = rel_lines(&stdout, tmp.path());
    assert_eq!(
        rels,
        vec![
            PathBuf::from(".grove/BRIEF.md"),
            PathBuf::from(".grove/020-mid/BRIEF.md"),
            PathBuf::from(".grove/020-mid/030-node/BRIEF.md"),
        ]
    );
}

#[test]
fn intermediate_directory_without_brief_is_skipped_silently() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    // Mid-level directory has no BRIEF.md — mid-decomposition transient state.
    touch(&grove.join("020-mid/030-node/BRIEF.md"));
    touch(&grove.join("020-mid/030-node/040-leaf.md"));

    let leaf = grove.join("020-mid/030-node/040-leaf.md");
    let leaf_s = leaf.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = brief_chain(tmp.path(), &[&leaf_s]);
    assert!(ok, "stderr={stderr}");
    let rels = rel_lines(&stdout, tmp.path());
    assert_eq!(
        rels,
        vec![
            PathBuf::from(".grove/BRIEF.md"),
            PathBuf::from(".grove/020-mid/030-node/BRIEF.md"),
        ]
    );
}

#[test]
fn no_arg_form_uses_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("020-mid/BRIEF.md"));
    touch(&grove.join("020-mid/030-leaf.md"));

    let (stdout, stderr, ok) = brief_chain(tmp.path(), &[]);
    assert!(ok, "stderr={stderr}");
    let rels = rel_lines(&stdout, tmp.path());
    assert_eq!(
        rels,
        vec![
            PathBuf::from(".grove/BRIEF.md"),
            PathBuf::from(".grove/020-mid/BRIEF.md"),
        ]
    );
}

#[test]
fn no_arg_form_on_fully_retired_grove_prints_diagnostic() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("done/010-retired.md"));

    let (stdout, stderr, ok) = brief_chain(tmp.path(), &[]);
    assert!(ok, "expected success exit; stderr={stderr}");
    assert!(stdout.trim().is_empty(), "expected empty stdout, got {stdout:?}");
    assert!(
        stderr.contains("no live leaves"),
        "expected diagnostic, got {stderr:?}"
    );
}

#[test]
fn accepts_grove_root_relative_leaf_path() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    touch(&grove.join("020-mid/BRIEF.md"));
    touch(&grove.join("020-mid/030-leaf.md"));

    // Pass `020-mid/030-leaf.md` (no `.grove/` prefix) — must be interpreted
    // relative to grove root.
    let (stdout, stderr, ok) = brief_chain(tmp.path(), &["020-mid/030-leaf.md"]);
    assert!(ok, "stderr={stderr}");
    let rels = rel_lines(&stdout, tmp.path());
    assert_eq!(
        rels,
        vec![
            PathBuf::from(".grove/BRIEF.md"),
            PathBuf::from(".grove/020-mid/BRIEF.md"),
        ]
    );
}

#[test]
fn missing_root_brief_yields_empty_chain() {
    // The grove root has no BRIEF.md yet (mid-bootstrap transient). The verb
    // must not error; it returns an empty chain.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("020-mid/BRIEF.md"));
    touch(&grove.join("020-mid/030-leaf.md"));

    let leaf = grove.join("020-mid/030-leaf.md");
    let leaf_s = leaf.to_string_lossy().into_owned();
    let (stdout, stderr, ok) = brief_chain(tmp.path(), &[&leaf_s]);
    assert!(ok, "stderr={stderr}");
    let rels = rel_lines(&stdout, tmp.path());
    assert_eq!(rels, vec![PathBuf::from(".grove/020-mid/BRIEF.md")]);
}

#[test]
fn errors_when_leaf_outside_grove_root() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove.join("BRIEF.md"));
    let stray = tmp.path().join("stray.md");
    touch(&stray);

    let stray_s = stray.to_string_lossy().into_owned();
    let (_, stderr, ok) = brief_chain(tmp.path(), &[&stray_s]);
    assert!(!ok, "expected error exit");
    assert!(
        stderr.contains("not under grove root"),
        "expected diagnostic, got {stderr:?}"
    );
}

#[test]
fn brief_chain_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        s.contains("brief-chain"),
        "grove-llm --help missing brief-chain: {s}"
    );
}

#[test]
fn grove_help_does_not_list_brief_chain() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("brief-chain"),
        "grove --help leaked brief-chain from the LLM surface: {s}"
    );
}

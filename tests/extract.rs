mod common;

use common::fixture_tarball;
use grove::extract::extract_content;
use std::fs;
use tempfile::TempDir;

#[test]
fn extracts_content_files_strips_root_and_content_prefix() {
    let tarball = fixture_tarball(
        "0.1.0",
        &[
            ("content/SKILL.md", b"# SKILL"),
            ("content/prompts/start.md", b"start prompt"),
            ("Cargo.toml", b"[package]"), // outside content/ — should be skipped
        ],
    );
    let dest = TempDir::new().unwrap();

    extract_content(&tarball, dest.path()).unwrap();

    assert_eq!(
        fs::read_to_string(dest.path().join("SKILL.md")).unwrap(),
        "# SKILL"
    );
    assert_eq!(
        fs::read_to_string(dest.path().join("prompts/start.md")).unwrap(),
        "start prompt"
    );
    assert!(!dest.path().join("Cargo.toml").exists());
}

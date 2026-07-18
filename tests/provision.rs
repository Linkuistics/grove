// Integration tests for the global-skill provisioning (src/provision.rs):
// the `grove` binary embeds `content/` and extracts it idempotently to the
// global personal skill dir. These drive the *real* embed via the public
// `provision_into`, against a throwaway tempdir, and prove the three contract
// scenarios named in the leaf brief: extract-fresh, extract-idempotent,
// extract-on-change (self-extension-core-and-methodology / task-tree-scheme, 070/010).

use grove::provision::provision_target;
use grove::provision::{provision_into, STAMP_FILE};
use std::fs;
use std::sync::Mutex;
use tempfile::TempDir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

#[test]
fn extract_fresh_writes_the_full_content_tree() {
    let dest = TempDir::new().unwrap();

    let extracted = provision_into(dest.path()).unwrap();

    assert!(extracted, "a fresh dir must be extracted into");
    // The skill entrypoint, a prompt under a subdir, and a top-level format
    // guide all travel — i.e. the *whole* tree, nested dirs included.
    assert!(dest.path().join("SKILL.md").is_file());
    assert!(dest.path().join("prompts/start.md").is_file());
    assert!(dest.path().join("prompts/continue.md").is_file());
    assert!(dest.path().join("driving.md").is_file());
    // A deeply-nested file (LICENSES/) confirms recursion to the leaves.
    assert!(dest.path().join("LICENSES").is_dir());
    // The stamp is written so the next launch can detect a warm dir.
    assert!(dest.path().join(STAMP_FILE).is_file());
}

#[test]
fn extract_is_idempotent_on_a_warm_dir() {
    let dest = TempDir::new().unwrap();

    assert!(provision_into(dest.path()).unwrap(), "first call extracts");
    let before = fs::read_to_string(dest.path().join("SKILL.md")).unwrap();

    assert!(
        !provision_into(dest.path()).unwrap(),
        "a warm dir with a matching stamp is a no-op"
    );
    let after = fs::read_to_string(dest.path().join("SKILL.md")).unwrap();
    assert_eq!(before, after, "warm no-op leaves content untouched");
}

#[test]
fn extract_re_extracts_and_clears_stale_files_when_the_stamp_mismatches() {
    let dest = TempDir::new().unwrap();
    provision_into(dest.path()).unwrap();

    // Simulate a dir produced by a *different* (older) embed: a stale stamp plus
    // a file the current embed does not contain.
    let stale = dest.path().join("STALE.md");
    fs::write(&stale, "left over from an older embed").unwrap();
    fs::write(
        dest.path().join(STAMP_FILE),
        "0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    let extracted = provision_into(dest.path()).unwrap();

    assert!(
        extracted,
        "a changed embed (mismatched stamp) must re-extract"
    );
    assert!(!stale.exists(), "files from the old embed must be cleared");
    assert!(
        dest.path().join("SKILL.md").is_file(),
        "the current embed is re-written"
    );
}

#[test]
fn provision_replaces_a_symlinked_grove_entry_with_a_real_dir() {
    let tmp = TempDir::new().unwrap();
    // Simulate today's layout: a real provisioned dir + a skills entry that
    // symlinks to it (the current ~/.codex/skills/grove and
    // ~/.pi/agent/skills/grove setups).
    let real = tmp.path().join("claude-skills/grove");
    fs::create_dir_all(&real).unwrap();
    provision_into(&real).unwrap();
    let linked = tmp.path().join("codex-skills/grove");
    fs::create_dir_all(linked.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&real, &linked).unwrap();

    let wrote = provision_target(&linked).unwrap();

    assert!(wrote, "a symlinked entry is replaced, not treated as warm");
    let meta = fs::symlink_metadata(&linked).unwrap();
    assert!(meta.is_dir(), "the symlink becomes a real directory");
    assert!(linked.join("SKILL.md").is_file());
    // CRITICAL: replacing the link must not have reached through it — the
    // original target keeps its content.
    assert!(
        real.join("SKILL.md").is_file(),
        "replacing the symlink must never delete through it"
    );
}

#[test]
fn provision_refuses_a_foreign_directory() {
    let tmp = TempDir::new().unwrap();
    let dest = tmp.path().join("skills/grove");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("precious.txt"), "user data, not ours").unwrap();

    let err = provision_target(&dest).unwrap_err().to_string();

    assert!(
        err.contains("precious") || err.contains("not a grove-provisioned"),
        "must refuse to clobber a dir grove didn't write (err: {err})"
    );
    assert!(
        dest.join("precious.txt").is_file(),
        "the foreign content must be untouched"
    );
}

#[test]
fn provision_all_sweeps_installed_harness_roots_and_honours_the_primary() {
    let _g = ENV_LOCK.lock().unwrap();
    let home = TempDir::new().unwrap();
    // Installed roots: claude and pi. codex is absent. pi is also the primary.
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    fs::create_dir_all(home.path().join(".pi")).unwrap();

    let old_home = std::env::var_os("HOME");
    std::env::set_var("HOME", home.path());
    std::env::remove_var("GROVE_SKILL_DIR");

    let pi = grove::harness::by_name("pi").unwrap();
    let result = grove::provision::provision_all(pi);

    match old_home {
        Some(h) => std::env::set_var("HOME", h),
        None => std::env::remove_var("HOME"),
    }
    result.unwrap();

    assert!(
        home.path().join(".claude/skills/grove/SKILL.md").is_file(),
        "installed claude root is provisioned"
    );
    assert!(
        home.path()
            .join(".pi/agent/skills/grove/SKILL.md")
            .is_file(),
        "the primary (pi) is provisioned — note agent/ nesting"
    );
    assert!(
        !home.path().join(".codex").exists(),
        "an absent harness root is skipped, not created"
    );
}

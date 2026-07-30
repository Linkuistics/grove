// Integration tests for the global-skill provisioning (src/provision.rs):
// the `grove` binary embeds `content/` and extracts it idempotently to the
// global personal skill dir. These drive the *real* embed via the public
// `provision_into`, against a throwaway tempdir, and prove the three contract
// scenarios named in the leaf brief: extract-fresh, extract-idempotent,
// extract-on-change (self-extension-core-and-methodology / task-tree-scheme).

mod support;

use grove::provision::provision_target;
use grove::provision::{provision_into, STAMP_FILE};
use std::fs;
use std::sync::Mutex;
use support::EnvGuard;
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

    // `real` already carries a matching stamp (from `provision_into` above),
    // so this is the property that's actually load-bearing here: a symlink
    // must be unconditionally replaced even when its *target* looks warm —
    // `sync_to_stamp`'s own stamp comparison would otherwise treat a
    // stamp-matching symlink as already-done and never replace it, which
    // would leave the cross-harness link farm in place forever.
    assert!(wrote, "a symlinked entry is replaced, not treated as warm");
    let meta = fs::symlink_metadata(&linked).unwrap();
    assert!(meta.is_dir(), "the symlink becomes a real directory");
    assert!(linked.join("SKILL.md").is_file());
    // Regression guard, not a live-risk check (branch-review-k14 T2): removing
    // the symlink must never affect `real`'s own content. This can't currently
    // be violated by *how* the symlink is removed — `std::fs::remove_dir_all`
    // documents (see its "TOCTOU race conditions" section, stable since 1.0.0)
    // that on every platform except Miri/QNX/Redox/VxWorks (none grove targets)
    // it `lstat`s a top-level symlink argument and unlinks rather than
    // recursing, exactly like `remove_file` — verified against this repo's
    // own std (rustc 1.97.1, `sys/fs/unix.rs::remove_dir_all_modern` and the
    // `sys/fs/common.rs` fallback both special-case it). So `rust-version`
    // (1.74) needs no bump for this: the guarantee predates it. Kept as
    // documentation of the invariant, not as a mutation-resistant check.
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

    // branch-review-k14 T8: the message interpolates `dest`, never the
    // offending file's own name — an `err.contains("precious")` disjunct here
    // would never be true and silently widen the assertion; assert only the
    // clause the message actually satisfies.
    assert!(
        err.contains("not a grove-provisioned"),
        "must refuse to clobber a dir grove didn't write (err: {err})"
    );
    assert!(
        dest.join("precious.txt").is_file(),
        "the foreign content must be untouched"
    );
}

#[test]
fn provision_all_sweeps_installed_harness_roots_and_honours_the_primary() {
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();
    // Installed roots: claude and pi. codex is absent. pi is also the primary.
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    fs::create_dir_all(home.path().join(".pi")).unwrap();

    // branch-review-k14 T7: restoring `HOME` while unconditionally *removing*
    // `GROVE_SKILL_DIR` (never restoring whatever it held before) left the
    // ambient env asymmetrically clobbered; `EnvGuard` restores both the same
    // way.
    let mut env = EnvGuard::new();
    env.set("HOME", home.path()).remove("GROVE_SKILL_DIR");

    let pi = grove::harness::by_name("pi").unwrap();
    let result = grove::provision::provision_all(pi);

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

#[test]
fn provision_all_does_not_let_a_foreign_secondary_harness_block_the_primary() {
    // branch-review-k14 B9: a foreign dir under an *unrelated* harness must
    // never abort the primary's own provisioning, or the whole `grove do`
    // launch.
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();

    // codex is installed but its skill dir is foreign content, not ours.
    fs::create_dir_all(home.path().join(".codex/skills/grove")).unwrap();
    fs::write(
        home.path().join(".codex/skills/grove/precious.txt"),
        "user data, not ours",
    )
    .unwrap();

    let mut env = EnvGuard::new();
    env.set("HOME", home.path()).remove("GROVE_SKILL_DIR");

    let pi = grove::harness::by_name("pi").unwrap();
    let result = grove::provision::provision_all(pi);

    result.unwrap();

    assert!(
        home.path()
            .join(".pi/agent/skills/grove/SKILL.md")
            .is_file(),
        "the primary (pi) must still be provisioned despite codex's foreign dir"
    );
    assert!(
        home.path()
            .join(".codex/skills/grove/precious.txt")
            .is_file(),
        "the foreign secondary dir must be left untouched, not overwritten"
    );
    assert!(
        !home.path().join(".codex/skills/grove/SKILL.md").exists(),
        "the foreign secondary dir must not have been provisioned into"
    );
}

#[test]
fn provision_all_still_bails_when_the_primarys_own_dir_is_foreign() {
    // The contract still requires bailing on a foreign dir for the harness
    // actually about to launch — branch-review-k14 B9 narrows the blast radius,
    // it does not remove the guard.
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();

    fs::create_dir_all(home.path().join(".pi/agent/skills/grove")).unwrap();
    fs::write(
        home.path().join(".pi/agent/skills/grove/precious.txt"),
        "user data, not ours",
    )
    .unwrap();

    let mut env = EnvGuard::new();
    env.set("HOME", home.path()).remove("GROVE_SKILL_DIR");

    let pi = grove::harness::by_name("pi").unwrap();
    let err = grove::provision::provision_all(pi).unwrap_err().to_string();

    assert!(
        err.contains("not a grove-provisioned"),
        "the primary's own foreign dir must still bail (err: {err})"
    );
}

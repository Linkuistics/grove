// Integration tests for the global-skill provisioning (src/provision.rs):
// the `grove` binary embeds `content/` and extracts it idempotently to the
// global personal skill dir. These drive the *real* embed via the public
// `provision_into`, against a throwaway tempdir, and prove the three contract
// scenarios named in the leaf brief: extract-fresh, extract-idempotent,
// extract-on-change (self-extension-core-and-methodology / task-tree-scheme).

mod support;

use grove::methodology::identity;
use grove::provision::provision_target;
use grove::provision::{provision_into, STAMP_FILE};
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use support::EnvGuard;
use tempfile::TempDir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// A phrase that exists only in `content/`, used to ask a *binary* whether it
/// carries the embed.
const CONTENT_MARKER: &str = "hierarchical, self-extending workstreams";

/// **Both binaries link the embed** — `grove` to extract it, `grove-llm` to
/// serve unit bytes to a session that followed a `defers=` id.
///
/// This claim **inverted** with `grove-llm methodology`. It used to assert the
/// embed's *absence* from `grove-llm`, which was the whole reason the
/// methodology identity was a compile-time constant: naming the identity had to
/// stay possible without linking `content/`. Once the agent-facing binary links
/// it anyway, the constant loses its reason to exist, and both binaries hash the
/// embed directly through one implementation — which is what removed the
/// build-script traversal and the equality test that kept two traversals in step
/// (`docs/adr/one-build-owns-a-session.md`).
///
/// Asserted on the linked artifacts because that is where the claim lives: no
/// amount of reading the source shows what the linker kept. There is no longer
/// an interesting half and a control half — both halves are the claim — so a
/// phrase that quietly left `content/` fails here twice rather than
/// manufacturing a clean absence.
#[test]
fn both_binaries_carry_the_embedded_methodology() {
    let carries = |binary: &str| {
        let bytes = fs::read(binary).unwrap();
        bytes
            .windows(CONTENT_MARKER.len())
            .any(|window| window == CONTENT_MARKER.as_bytes())
    };

    assert!(
        carries(env!("CARGO_BIN_EXE_grove")),
        "`grove` extracts the methodology, so it must carry it — if this fails, \
         {CONTENT_MARKER:?} has left content/ and the marker needs updating"
    );
    assert!(
        carries(env!("CARGO_BIN_EXE_grove-llm")),
        "`grove-llm methodology` serves unit bytes out of this binary's own \
         embed, so `grove-llm` must carry content/. If this fails the verb has \
         stopped reading the embed — or something has put the compile-time \
         identity constant back."
    );
}

/// The test above sees only what `cargo test` built — one profile, one host
/// target — while a release builds three targets in `--release`, two of them
/// cross-compiled and therefore unrunnable on the building machine. So the
/// release path scans each staged pair for the same phrase before archiving it
/// (`scripts/release-build.sh`, via `assert_methodology_pairing`), which is what
/// makes the invariant hold of the artifacts that actually ship rather than of
/// their test-profile cousins. That scan inverted alongside the test above, and
/// it inverted *here* rather than at the next release cut because it is a
/// release-path check: nothing in `cargo test` would have gone red.
///
/// Two scans mean two places to edit, and this pins them together. Drift is not
/// silent in either direction — a stale marker fails the *presence* half — but it
/// fails there for the wrong reason, mid-release, on a tagged tree.
#[test]
fn the_release_path_scans_for_the_same_marker() {
    let common =
        fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("scripts/release-common.sh"))
            .unwrap();

    assert!(
        common.contains(&format!("readonly CONTENT_MARKER='{CONTENT_MARKER}'")),
        "scripts/release-common.sh must scan the shipped artifacts for the same \
         phrase this test scans for ({CONTENT_MARKER:?}); update CONTENT_MARKER \
         in both places together"
    );
}

/// The flag and the stamp are two views of one value, and the pair check is only
/// as good as their agreement: the driver compares what a `grove-llm` *reports*
/// against what its paired `grove` *wrote*, so a divergence would make every
/// correctly paired machine look mismatched.
#[test]
fn the_reported_identity_is_the_one_provisioning_stamps() {
    let dest = TempDir::new().unwrap();
    provision_into(dest.path()).unwrap();
    let stamped = fs::read_to_string(dest.path().join(STAMP_FILE)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("--content-hash")
        .output()
        .unwrap();

    assert!(output.status.success());
    let reported = String::from_utf8(output.stdout).unwrap();
    assert_eq!(reported.trim(), stamped);
    assert_eq!(reported.trim(), identity());
}

/// The check inside the session — the only one a clobber landing *after* launch
/// can reach, and the one whose two operands are the ones that matter: the CLI
/// actually invoked, and the methodology actually on disk in front of it.
///
/// It warns and does nothing else. A refusal mid-task destroys more than the
/// mismatch does, and the session least able to absorb a hard stop is one
/// already holding uncommitted work — so the verb still answers, on stdout,
/// exit zero.
#[test]
fn a_foreign_stamp_warns_without_changing_what_a_verb_does() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let skill_dir = home.join(".claude/skills/grove");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join(STAMP_FILE), "another-build").unwrap();
    let worktree = git_worktree_with_one_leaf(fixture.path());

    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("pick")
        .current_dir(&worktree)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "a mismatch must not refuse: {stderr}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("01-impl-first-k1.md"),
        "the verb must still answer: {stderr}"
    );
    assert!(
        stderr.contains(skill_dir.to_str().unwrap())
            && stderr.contains("another-build")
            && stderr.contains(identity()),
        "the warning must name the directory and both identities: {stderr}"
    );
}

/// **Absence is not disagreement.** The claim on offer is "this directory
/// belongs to another build", and nothing can say that about a directory that
/// does not exist — an unprovisioned root, or a harness that is not installed,
/// is silent.
#[test]
fn an_unstamped_or_absent_skill_directory_is_silent() {
    let fixture = TempDir::new().unwrap();
    let worktree = git_worktree_with_one_leaf(fixture.path());

    let unstamped = fixture.path().join("unstamped-home");
    fs::create_dir_all(unstamped.join(".claude/skills/grove")).unwrap();
    let uninstalled = fixture.path().join("uninstalled-home");
    fs::create_dir_all(&uninstalled).unwrap();

    for home in [&unstamped, &uninstalled] {
        let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
            .arg("pick")
            .current_dir(&worktree)
            .env("HOME", home)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(
            stderr.is_empty(),
            "{} must produce no pairing warning, got: {stderr}",
            home.display()
        );
    }
}

/// A minimal current-format grove, enough for `pick` to have an answer.
fn git_worktree_with_one_leaf(fixture: &Path) -> std::path::PathBuf {
    let worktree = fixture.join("worktree");
    fs::create_dir_all(&worktree).unwrap();
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(&worktree)
        .status()
        .unwrap()
        .success());
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# worktree — brief\n").unwrap();
    fs::write(grove.join("01-impl-first-k1.md"), "# first-k1\n").unwrap();
    worktree
}

#[test]
fn extract_fresh_writes_the_full_content_tree() {
    let dest = TempDir::new().unwrap();

    let extracted = provision_into(dest.path()).unwrap();

    assert!(extracted, "a fresh dir must be extracted into");
    // The skill entrypoint, a kind's reference file and a top-level format guide
    // all travel — i.e. the *whole* tree, nested dirs included. `references/` is
    // asserted here rather than only in the routing checks because it is what
    // makes the extraction recursive over a directory that carries methodology,
    // which `LICENSES/` below cannot say.
    assert!(dest.path().join("SKILL.md").is_file());
    assert!(dest.path().join("references/impl.md").is_file());
    assert!(dest.path().join("driving.md").is_file());
    // Recursion to the leaves, asserted on a nested **file** rather than on the
    // directory holding it: extraction creates directories on its way down, so a
    // `LICENSES/` that exists proves only that the walk reached the name. It is
    // a nested file the unit grammar does not govern, so it says something
    // `references/` above cannot: the walk copies non-markdown leaves too.
    assert!(dest.path().join("LICENSES/openspec.LICENSE").is_file());
    // The stamp is written so the next launch can detect a warm dir.
    assert!(dest.path().join(STAMP_FILE).is_file());
}

// `exactly_one_launcher_is_embedded_and_provisioned` **died with the surface it
// enumerated**, and both of its halves were re-housed rather than dropped.
//
// It read `prompts/` back out of a fresh extraction and asserted one entry, so
// that a third launcher failed here rather than shipping unnoticed. There is no
// launcher now, and no launcher prose in the embed at all: `content/MANDATE.md`
// framed the mandate and went with it, and what the driver writes is the
// guaranteed core, in Rust, under the too-late test. The successor guard is
// stronger than the directory listing was and lives in `tests/methodology.rs` —
// every embedded file must declare at least one unit, and `EMBEDDED_UNITS` pins
// the whole id set, so a launcher file added anywhere under `content/` fails
// there until a human names its unit.
//
// Its second half compared the provisioned bytes with the ones the driver put in
// front of every mandate. An embedded file is read into a prompt again — the
// kind's signal file, inlined byte-exact as the core's last part — so that
// comparison has operands once more and lives in `tests/prompt.rs`, where the
// claim is that the ending *is* the embed's bytes rather than a Rust copy of
// them. Both paths still project the same embed: `provision_into` extracts it
// verbatim, and `prompt::compose` inlines one file of it unmodified.

// The two claims that used to sit here — the embedded methodology instructing no
// `grove-llm` verb the embedded CLI lacks, and the flat-verb-surface pin that
// makes that comparison mean what it claims — **moved to
// `tests/methodology.rs`**. Both are claims about the embed rather than about
// provisioning, they outlive the sweep this file tests, and their corpus
// improved in the move: they scan the embed itself rather than a provisioned
// extraction of it, which is what dissolved the recursive walk that used to
// gather it. `the_identity_flag_is_not_part_of_the_agent_verb_surface` went
// with them, because both of its operands did.

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
    // needs no bump for this, whatever it currently says: the guarantee
    // predates any floor Grove could claim. Kept as documentation of the
    // invariant, not as a mutation-resistant check.
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

// Provisioning has no primary any more: configured commands are opaque, so the
// driver cannot know which harness a session eventually reaches and does not
// guess. It refreshes every *installed* known root instead — presence of the
// home marker is the whole rule, and an absent root is skipped rather than
// created.
#[test]
fn provision_installed_sweeps_every_installed_harness_root() {
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    fs::create_dir_all(home.path().join(".pi")).unwrap();

    let mut env = EnvGuard::new();
    env.set("HOME", home.path());

    grove::provision::provision_installed().unwrap();

    assert!(
        home.path().join(".claude/skills/grove/SKILL.md").is_file(),
        "an installed claude root is provisioned"
    );
    assert!(
        home.path()
            .join(".pi/agent/skills/grove/SKILL.md")
            .is_file(),
        "an installed pi root is provisioned — note the agent/ nesting"
    );
    assert!(
        !home.path().join(".codex").exists(),
        "an absent harness root is skipped, not created"
    );
}

// There is no destination override to route around this. The foreign-directory
// guard is what stands between a user's own `skills/grove` and an extraction
// over the top of it, so provisioning refuses rather than proceeding — and the
// refusal reaches the driver, which never owns the working tree.
#[test]
fn provision_installed_refuses_a_foreign_skill_directory() {
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();
    fs::create_dir_all(home.path().join(".pi/agent/skills/grove")).unwrap();
    fs::write(
        home.path().join(".pi/agent/skills/grove/precious.txt"),
        "user data, not ours",
    )
    .unwrap();

    let mut env = EnvGuard::new();
    env.set("HOME", home.path());

    let err = grove::provision::provision_installed()
        .unwrap_err()
        .to_string();

    assert!(
        err.contains("not a grove-provisioned"),
        "a foreign dir must bail (err: {err})"
    );
    assert!(
        home.path()
            .join(".pi/agent/skills/grove/precious.txt")
            .is_file(),
        "the foreign content must be untouched"
    );
}

//! Provision the global grove skill from the methodology embedded in the binary.
//!
//! self-extension-core-and-methodology / task-tree-scheme: distribution collapses to one binary that carries grove's full
//! retained methodology and extracts it to the global personal skill dir
//! (`~/.claude/skills/grove/`) on `grove do`. Because the skill travels inside
//! the binary, it can never drift from it — this replaces the old fetch-tarball +
//! materialise-per-harness + `VERSION.md` model (whose deletion is leaf 090).
//!
//! Extraction is idempotent against a content-hash stamp: a warm launch is a
//! cheap no-op; a binary built from changed `content/` re-extracts. The hash —
//! not the crate version — is the stamp, so even an edit at the same version
//! (the dogfooding case) re-provisions.

use crate::harness::{Harness, HARNESSES};
use anyhow::{Context, Result};
use include_dir::{include_dir, Dir, DirEntry};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

/// grove's full retained methodology, embedded at compile time. `build.rs` emits
/// `rerun-if-changed` for the tree so an edit to any file re-embeds.
static CONTENT: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/content");

/// Hidden stamp file written inside the extracted skill dir, holding the content
/// hash of the embed that produced it. Compared on launch; a mismatch — or a
/// missing stamp (e.g. an empty or old materialised dir) — re-extracts. Hidden,
/// and outside the methodology's own files, so it never collides with content
/// and Claude Code ignores it.
pub const STAMP_FILE: &str = ".grove-content-hash";

/// Provision the embedded methodology for every harness on this machine:
/// `primary` (the harness about to launch) unconditionally, plus every other
/// harness whose home root (`~/.claude`, `~/.codex`, `~/.pi`) exists. Logging
/// only when a target actually (re)writes. With `$GROVE_SKILL_DIR` set (the
/// test/dev seam) the sweep collapses to that single dir.
pub fn provision_all(primary: &'static Harness) -> Result<()> {
    if std::env::var_os("GROVE_SKILL_DIR").is_some() {
        let dest = skill_dir_for(primary)?; // the override wins inside
        if provision_target(&dest)? {
            eprintln!("grove: provisioned the skill at {}", dest.display());
        }
        return Ok(());
    }
    let home = home_dir()?;
    for h in HARNESSES {
        let installed = home.join(h.project_dir).is_dir();
        if h.name != primary.name && !installed {
            continue; // absent harness root: skip, never create
        }
        let dest = home.join(h.skills_dir).join("grove");
        if provision_target(&dest)? {
            eprintln!(
                "grove: provisioned the {} skill at {}",
                h.name,
                dest.display()
            );
        }
    }
    Ok(())
}

/// A harness's global skill dir: `$GROVE_SKILL_DIR` override (test/dev seam),
/// else `$HOME/<harness.skills_dir>/grove`.
pub fn skill_dir_for(harness: &Harness) -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os("GROVE_SKILL_DIR") {
        return Ok(PathBuf::from(dir));
    }
    Ok(home_dir()?.join(harness.skills_dir).join("grove"))
}

fn home_dir() -> Result<PathBuf> {
    let home = std::env::var_os("HOME")
        .ok_or_else(|| anyhow::anyhow!("$HOME is not set; cannot locate the global skill dirs"))?;
    Ok(PathBuf::from(home))
}

/// Idempotently extract the embedded methodology into `dest`. Returns whether it
/// (re)wrote: `true` on a fresh or changed embed, `false` on a warm no-op.
pub fn provision_into(dest: &Path) -> Result<bool> {
    let want = content_hash(&CONTENT);
    sync_to_stamp(dest, &want, |d| {
        CONTENT
            .extract(d)
            .with_context(|| format!("extracting embedded content to {}", d.display()))
    })
}

/// Guarded single-target provisioning. Replaces `dest` only when it is ours
/// to replace: a symlink (today's cross-harness link farm — removed as a
/// *link*, never through it), a grove-provisioned dir (stamp present), or
/// absent/empty. Anything else is someone's real content: bail.
pub fn provision_target(dest: &Path) -> Result<bool> {
    if let Ok(meta) = std::fs::symlink_metadata(dest) {
        if meta.file_type().is_symlink() {
            // Order matters: remove the LINK first. remove_dir_all through a
            // symlink would delete the target's contents.
            std::fs::remove_file(dest)
                .with_context(|| format!("removing symlink {}", dest.display()))?;
        } else if meta.is_dir()
            && !dest.join(STAMP_FILE).exists()
            && std::fs::read_dir(dest)?.next().is_some()
        {
            anyhow::bail!(
                "refusing to overwrite {} — it exists but is not a \
                 grove-provisioned dir (no {} stamp); move it aside and re-run",
                dest.display(),
                STAMP_FILE
            );
        }
    }
    provision_into(dest)
}

/// Write `dest` via `write_files` only when its stamp differs from `want_hash`;
/// a matching stamp is a no-op (the warm launch). On a mismatch the dir is
/// cleared first, so files dropped by an older embed do not linger. Returns
/// whether it (re)wrote.
fn sync_to_stamp(
    dest: &Path,
    want_hash: &str,
    write_files: impl FnOnce(&Path) -> Result<()>,
) -> Result<bool> {
    let have = std::fs::read_to_string(dest.join(STAMP_FILE)).ok();
    if have.as_deref() == Some(want_hash) {
        return Ok(false);
    }
    if dest.exists() {
        std::fs::remove_dir_all(dest)
            .with_context(|| format!("clearing stale skill dir {}", dest.display()))?;
    }
    std::fs::create_dir_all(dest)
        .with_context(|| format!("creating skill dir {}", dest.display()))?;
    write_files(dest)?;
    std::fs::write(dest.join(STAMP_FILE), want_hash)
        .with_context(|| format!("writing skill stamp to {}", dest.display()))?;
    Ok(true)
}

/// A deterministic hash over the embedded tree — every file's relative path and
/// bytes, in sorted path order. Changes whenever any file is added, removed,
/// moved, or edited, so it faithfully stamps "which embed produced this dir".
fn content_hash(dir: &Dir) -> String {
    let mut files: Vec<(&Path, &[u8])> = Vec::new();
    collect_files(dir, &mut files);
    files.sort_by(|a, b| a.0.cmp(b.0));
    hash_files(&files)
}

fn collect_files<'a>(dir: &'a Dir, out: &mut Vec<(&'a Path, &'a [u8])>) {
    for entry in dir.entries() {
        match entry {
            DirEntry::Dir(d) => collect_files(d, out),
            DirEntry::File(f) => out.push((f.path(), f.contents())),
        }
    }
}

/// Hash a pre-sorted (path, bytes) list. Each field is length-prefixed so no
/// reshuffling of bytes across the path/content boundary can collide.
fn hash_files(files: &[(&Path, &[u8])]) -> String {
    let mut hasher = Sha256::new();
    for (path, bytes) in files {
        let p = path.to_string_lossy();
        hasher.update((p.len() as u64).to_le_bytes());
        hasher.update(p.as_bytes());
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(bytes);
    }
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    static ENV_LOCK_FOR_HOME: std::sync::Mutex<()> = std::sync::Mutex::new(());

    #[test]
    fn skill_dirs_follow_each_harness_layout() {
        let _lock = ENV_LOCK_FOR_HOME.lock().unwrap();
        let old_home = std::env::var_os("HOME");
        std::env::remove_var("GROVE_SKILL_DIR");
        std::env::set_var("HOME", "/home/x");
        assert_eq!(
            skill_dir_for(crate::harness::by_name("claude").unwrap()).unwrap(),
            Path::new("/home/x/.claude/skills/grove")
        );
        assert_eq!(
            skill_dir_for(crate::harness::by_name("pi").unwrap()).unwrap(),
            Path::new("/home/x/.pi/agent/skills/grove")
        );
        match old_home {
            Some(h) => std::env::set_var("HOME", h),
            None => std::env::remove_var("HOME"),
        }
    }

    #[test]
    fn content_hash_of_the_embed_is_stable_and_nonempty() {
        let h = content_hash(&CONTENT);
        assert!(!h.is_empty());
        assert_eq!(h, content_hash(&CONTENT), "hashing is deterministic");
    }

    #[test]
    fn hash_changes_when_a_file_is_edited() {
        let a: [(&Path, &[u8]); 1] = [(Path::new("a.md"), b"one")];
        let b: [(&Path, &[u8]); 1] = [(Path::new("a.md"), b"two")];
        assert_ne!(hash_files(&a), hash_files(&b));
    }

    #[test]
    fn hash_changes_when_a_file_is_added() {
        let a: [(&Path, &[u8]); 1] = [(Path::new("a.md"), b"x")];
        let b: [(&Path, &[u8]); 2] = [(Path::new("a.md"), b"x"), (Path::new("b.md"), b"y")];
        assert_ne!(hash_files(&a), hash_files(&b));
    }

    #[test]
    fn hash_is_not_confused_by_the_path_content_boundary() {
        // "ab" + "c" vs "a" + "bc": length-prefixing must keep these distinct.
        let a: [(&Path, &[u8]); 1] = [(Path::new("ab"), b"c")];
        let b: [(&Path, &[u8]); 1] = [(Path::new("a"), b"bc")];
        assert_ne!(hash_files(&a), hash_files(&b));
    }

    #[test]
    fn sync_writes_on_fresh_then_noops_when_the_hash_matches() {
        let dir = TempDir::new().unwrap();

        let wrote = sync_to_stamp(dir.path(), "hash1", |d| {
            std::fs::write(d.join("a.txt"), "A")?;
            Ok(())
        })
        .unwrap();
        assert!(wrote, "fresh dir is written");
        assert_eq!(
            std::fs::read_to_string(dir.path().join("a.txt")).unwrap(),
            "A"
        );

        // Same hash → no-op; the writer must not even run.
        let wrote = sync_to_stamp(dir.path(), "hash1", |_d| {
            panic!("writer must not run on a warm dir");
        })
        .unwrap();
        assert!(!wrote, "matching stamp is a no-op");
    }

    #[test]
    fn sync_re_extracts_and_clears_stale_files_when_the_hash_differs() {
        let dir = TempDir::new().unwrap();
        sync_to_stamp(dir.path(), "hash1", |d| {
            std::fs::write(d.join("old.txt"), "old")?;
            Ok(())
        })
        .unwrap();

        let wrote = sync_to_stamp(dir.path(), "hash2", |d| {
            std::fs::write(d.join("new.txt"), "new")?;
            Ok(())
        })
        .unwrap();
        assert!(wrote, "differing stamp re-extracts");
        assert!(!dir.path().join("old.txt").exists(), "stale file cleared");
        assert!(dir.path().join("new.txt").is_file(), "new embed written");
    }
}

//! Provision the global grove skill from the methodology embedded in the binary.
//!
//! ADR-0031/0034: distribution collapses to one binary that carries grove's full
//! retained methodology and extracts it to the global personal skill dir
//! (`~/.claude/skills/grove/`) on `grove do`. Because the skill travels inside
//! the binary, it can never drift from it — this replaces the old fetch-tarball +
//! materialise-per-harness + `VERSION.md` model (whose deletion is leaf 090).
//!
//! Extraction is idempotent against a content-hash stamp: a warm launch is a
//! cheap no-op; a binary built from changed `content/` re-extracts. The hash —
//! not the crate version — is the stamp, so even an edit at the same version
//! (the dogfooding case) re-provisions.

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

/// Provision the global skill into `~/.claude/skills/grove/` (idempotent),
/// logging only when it actually (re)writes. This is the `grove do` provisioning
/// point — the entry the human always hits (ADR-0034).
pub fn provision_global_skill() -> Result<()> {
    let dest = global_skill_dir()?;
    if provision_into(&dest)? {
        eprintln!("grove: provisioned the global skill at {}", dest.display());
    }
    Ok(())
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

/// The global personal skill dir Claude Code reads live. `$GROVE_SKILL_DIR`
/// overrides it (a test/dev seam, mirroring `GROVE_HARNESS_BIN`); otherwise it
/// is `$HOME/.claude/skills/grove`.
pub fn global_skill_dir() -> Result<PathBuf> {
    if let Some(dir) = std::env::var_os("GROVE_SKILL_DIR") {
        return Ok(PathBuf::from(dir));
    }
    let home = std::env::var_os("HOME").ok_or_else(|| {
        anyhow::anyhow!(
            "$HOME is not set; cannot locate the global skill dir (~/.claude/skills/grove)"
        )
    })?;
    Ok(skill_dir_under_home(Path::new(&home)))
}

fn skill_dir_under_home(home: &Path) -> PathBuf {
    home.join(".claude").join("skills").join("grove")
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

    #[test]
    fn skill_dir_is_under_dot_claude_skills_grove() {
        assert_eq!(
            skill_dir_under_home(Path::new("/home/x")),
            Path::new("/home/x/.claude/skills/grove")
        );
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

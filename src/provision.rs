//! Provision the global grove skill from the methodology embedded in the binary.
//!
//! self-extension-core-and-methodology / task-tree-scheme: distribution collapses to one binary that carries grove's full
//! retained methodology and extracts it to the global personal skill dir
//! (`~/.claude/skills/grove/`) on `grove do`. Because the skill travels inside
//! the binary, it can never drift from it — this replaces the old fetch-tarball +
//! materialise-per-harness + `VERSION.md` model, now deleted.
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

/// This build's **methodology identity** — the content hash of its embedded
/// `content/`, emitted by `build.rs` as a compile-time constant.
///
/// A `const` rather than a call to [`content_hash`] so that *naming* the
/// identity does not link the embed: only `grove` extracts content, and a
/// `grove-llm` that hashed its own tree at runtime would grow by the size of
/// `content/` for a value known at compile time (`tests/provision.rs` holds that
/// saving). `build.rs` computes it from the filesystem and [`content_hash`]
/// computes it from `include_dir`'s embed, so the two traversals can only be
/// kept in step by assertion — see
/// [`the_compile_time_identity_names_the_linked_embed`](tests::the_compile_time_identity_names_the_linked_embed).
///
/// **It covers the embedded file payload and not the embedded directory
/// structure.** `include_dir` embeds a `DirEntry::Dir` for every directory,
/// including an empty one, and `docs/adr/one-build-owns-a-session.md`
/// deliberately excludes those: a directory with no files carries no
/// methodology, and hashing typed directory paths would make `build.rs`
/// reproduce `include_dir`'s directory semantics as well as its file selection.
/// Neither traversal should start hashing directory entries; the accepted
/// consequence is that an empty directory is not part of a build's identity.
pub const METHODOLOGY_IDENTITY: &str = env!("GROVE_CONTENT_HASH");

/// The one launcher embedded into every config-driven session prompt.
pub fn continue_prompt() -> Result<&'static str> {
    let file = CONTENT
        .get_file("prompts/continue.md")
        .context("embedded Grove content is missing prompts/continue.md")?;
    std::str::from_utf8(file.contents()).context("embedded prompts/continue.md is not UTF-8")
}

/// Provision Grove into every installed harness root. Configured commands are
/// opaque, so the driver cannot infer which harness a session eventually
/// reaches — and does not try to: every known root that exists is refreshed, so
/// whichever one the configured command lands in already carries the current
/// methodology.
///
/// There is no dev override for the destination. The registry is the only
/// answer to "where", which is what keeps provisioning independent of launch
/// policy rather than a second, quieter way to configure one.
pub fn provision_installed() -> Result<()> {
    each_installed_skill_dir(|harness, destination| {
        if provision_target(destination)? {
            eprintln!(
                "grove: provisioned the {} skill at {}",
                harness.name,
                destination.display()
            );
        }
        Ok(())
    })
}

/// Re-verify, before every launch, that each installed skill directory still
/// carries *this* build's methodology, and restore the embed where another build
/// has taken one ([[Build pairing]] — `docs/adr/one-build-owns-a-session.md`).
///
/// The directories are global while the driver lease is per working tree, so
/// nothing serializes two builds writing one directory. A matching stamp is the
/// ordinary case and costs one small read per root; only a differing one
/// extracts. Re-*extracting* every iteration would be pure cost — a driver never
/// re-execs, so it carries one embed for its whole life and would write
/// identical bytes — which is why the question asked here is ownership rather
/// than freshness.
pub fn reverify_installed() -> Result<()> {
    each_installed_skill_dir(|harness, destination| {
        if stamp_of(destination).as_deref() == Some(METHODOLOGY_IDENTITY) {
            return Ok(());
        }
        provision_target(destination)?;
        eprintln!(
            "grove: restored the {} skill at {} — it did not carry this build's methodology ({METHODOLOGY_IDENTITY})",
            harness.name,
            destination.display()
        );
        Ok(())
    })
}

/// Warn — never refuse — when an installed skill directory is stamped with a
/// methodology other than this binary's.
///
/// This is the check whose two operands are the ones that matter: the CLI
/// actually invoked, and the methodology actually on disk in front of it. It is
/// also the only one a clobber landing *after* launch can reach, which is why it
/// runs on every verb rather than at a launch boundary. It never changes a
/// verb's exit status — Grove guides and does not gate on the agent surface, and
/// the session least able to absorb a hard stop is one already mid-task with
/// uncommitted work.
///
/// **Absence is not disagreement.** An unprovisioned or missing directory is
/// silent, as is a home this process cannot locate: the claim on offer is "this
/// directory belongs to another build", and nothing here can say that about a
/// directory that does not exist.
pub fn warn_on_foreign_skill_dirs() {
    let _ = each_installed_skill_dir(|_, destination| {
        let Some(stamp) = stamp_of(destination) else {
            return Ok(()); // absence is not disagreement
        };
        if stamp == METHODOLOGY_IDENTITY {
            return Ok(());
        }
        eprintln!(
            "grove-llm: {} carries methodology {}, but this grove-llm was built with {METHODOLOGY_IDENTITY} — one build owns a session",
            destination.display(),
            stamp.trim()
        );
        Ok(())
    });
}

/// Visit every *installed* harness's skill directory, in registry order.
///
/// Presence of the harness's home marker is the whole rule — an absent root is
/// skipped, never created — and the destination is passed rather than derived by
/// each caller, so the three things that ask about these directories (the sweep,
/// the per-launch re-verification, and the agent-side warning) cannot disagree
/// about which directories they mean.
fn each_installed_skill_dir(mut visit: impl FnMut(&Harness, &Path) -> Result<()>) -> Result<()> {
    let home = home_dir()?;
    for harness in HARNESSES {
        if !home.join(harness.project_dir).is_dir() {
            continue; // an absent root is skipped, never created
        }
        visit(harness, &skill_dir_in(&home, harness))?;
    }
    Ok(())
}

/// The methodology identity stamped on `dir`, or `None` when it carries no
/// stamp — absent, empty, or never provisioned by Grove.
///
/// Compared verbatim against [`METHODOLOGY_IDENTITY`], exactly as
/// [`sync_to_stamp`] compares before rewriting, so "this directory is mine"
/// means the same thing to the reader and to the writer.
fn stamp_of(dir: &Path) -> Option<String> {
    std::fs::read_to_string(dir.join(STAMP_FILE)).ok()
}

/// A harness's global skill dir under `home`: `<home>/<harness.skills_dir>/grove`.
///
/// The home is an argument rather than another `$HOME` read, so the layout rule
/// is a pure function a test can pin without writing a process-global that
/// [`home_dir`] — and `loop_driver`'s own config lookup — read in parallel.
fn skill_dir_in(home: &Path, harness: &Harness) -> PathBuf {
    home.join(harness.skills_dir).join("grove")
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
            // Unlink explicitly rather than falling through to
            // `provision_into`: `sync_to_stamp` treats a matching stamp as
            // warm-and-done, and a symlink's target (today's cross-harness
            // link farm) is almost always already stamped — so without this
            // the symlink would never be replaced. (`remove_dir_all` would
            // also be safe here: it `lstat`s a top-level symlink argument and
            // unlinks rather than recursing, on every platform grove ships
            // for — stable since 1.0.0, see its docs' TOCTOU section — so
            // this unlink isn't working around a symlink-following footgun,
            // just forcing replacement past the stamp shortcut above.)
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
/// a matching stamp is a no-op (the warm launch). Returns whether it (re)wrote.
///
/// Writes land in a sibling staging dir first, and only a `rename` ever
/// touches `dest` itself — so an interrupt (Ctrl-C, SIGTERM, ENOSPC) mid-write
/// leaves `dest` exactly as it was (absent, or the old warm dir), never
/// non-empty-without-a-stamp. That half-written state used to be
/// indistinguishable from a foreign directory to `provision_target`'s guard,
/// wedging every later `grove do` (branch-review-k14 B8) — a regression from
/// the pre-stamp path, which self-healed.
fn sync_to_stamp(
    dest: &Path,
    want_hash: &str,
    write_files: impl FnOnce(&Path) -> Result<()>,
) -> Result<bool> {
    let have = std::fs::read_to_string(dest.join(STAMP_FILE)).ok();
    if have.as_deref() == Some(want_hash) {
        return Ok(false);
    }
    let parent = dest
        .parent()
        .ok_or_else(|| anyhow::anyhow!("{} has no parent directory", dest.display()))?;
    std::fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
    let staging = tempfile::Builder::new()
        .prefix(".grove-provision-staging-")
        .tempdir_in(parent)
        .with_context(|| format!("creating a staging dir beside {}", dest.display()))?;
    write_files(staging.path())?;
    std::fs::write(staging.path().join(STAMP_FILE), want_hash)
        .with_context(|| format!("writing skill stamp to {}", staging.path().display()))?;

    if dest.exists() {
        std::fs::remove_dir_all(dest)
            .with_context(|| format!("clearing stale skill dir {}", dest.display()))?;
    }
    let staging_path = staging.keep();
    std::fs::rename(&staging_path, dest).with_context(|| {
        format!(
            "moving staged skill from {} into {}",
            staging_path.display(),
            dest.display()
        )
    })?;
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

    /// The row named `name`, looked up the way a test has to now that
    /// `harness::by_name` is gone: production *iterates* `HARNESSES`, so the
    /// registry is a slice and never had a selecting caller of its own
    /// (`dead-non-launch-exports-k166`).
    fn row(name: &str) -> &'static crate::harness::Harness {
        crate::harness::HARNESSES
            .iter()
            .find(|harness| harness.name == name)
            .unwrap_or_else(|| panic!("no {name} row in the provisioning registry"))
    }

    /// The layout rule alone. That the home comes from `$HOME` is the one thing
    /// this cannot show, and `tests/provision.rs` shows it instead: it points
    /// `HOME` at a temp dir under `lock_env` and asserts `provision_installed`
    /// wrote both of these paths beneath it. That binary is a separate process
    /// with its own env-lock discipline, which is why the same override there is
    /// not the hazard it was here.
    #[test]
    fn skill_dirs_follow_each_harness_layout() {
        assert_eq!(
            skill_dir_in(Path::new("/home/x"), row("claude")),
            Path::new("/home/x/.claude/skills/grove")
        );
        assert_eq!(
            skill_dir_in(Path::new("/home/x"), row("pi")),
            Path::new("/home/x/.pi/agent/skills/grove")
        );
    }

    /// The one thing that keeps two independent traversals of `content/` in
    /// step. `build.rs` walks the **filesystem** to emit
    /// [`METHODOLOGY_IDENTITY`]; [`content_hash`] walks `include_dir`'s **embed**
    /// to write the provisioning stamp. Nothing but this equality stops them
    /// diverging — and a divergence is silent in the worst direction: every
    /// build would report an identity no skill directory it wrote could ever
    /// match, so the pair check would cry mismatch on a correctly paired
    /// machine and the loop would reprovision on every iteration.
    ///
    /// It is also the guard on the payload/structure grain: teach either side
    /// to hash directory entries and this fails until both are taught.
    #[test]
    fn the_compile_time_identity_names_the_linked_embed() {
        assert_eq!(
            content_hash(&CONTENT),
            METHODOLOGY_IDENTITY,
            "`build.rs`'s traversal of content/ and `content_hash`'s traversal \
             of the embed disagree; they hash the same tree and must agree \
             byte for byte"
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
    fn interrupted_extraction_leaves_dest_untouched_and_the_next_call_self_heals() {
        let parent = TempDir::new().unwrap();
        let dest = parent.path().join("grove");

        // Simulate an interrupt mid-extract: the writer starts dropping files
        // into place, then fails before the stamp is ever written — exactly the
        // Ctrl-C / SIGTERM / ENOSPC case (branch-review-k14 B8). `dest` itself
        // must never be left non-empty-without-a-stamp: that state is
        // indistinguishable from a foreign directory to `provision_target`'s
        // guard, and wedges every later `grove do`.
        let result = sync_to_stamp(&dest, "hash1", |d| {
            std::fs::write(d.join("partial.txt"), "oops").unwrap();
            anyhow::bail!("simulated interrupt")
        });

        assert!(result.is_err(), "the simulated interrupt must propagate");
        assert!(
            !dest.exists(),
            "an interrupted extract must never leave dest in a partial state"
        );

        // The very next call must succeed cleanly — no foreign-dir bail, no
        // manual cleanup required (the pre-regression behaviour: self-heals).
        let wrote = sync_to_stamp(&dest, "hash1", |d| {
            std::fs::write(d.join("real.txt"), "content")?;
            Ok(())
        })
        .unwrap();
        assert!(wrote, "the next call re-extracts cleanly");
        assert!(dest.join("real.txt").is_file());
        assert!(dest.join(STAMP_FILE).is_file());
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

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
use crate::methodology;
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

/// Hidden stamp file written inside the extracted skill dir, holding the content
/// hash of the embed that produced it. Compared on launch; a mismatch — or a
/// missing stamp (e.g. an empty or old materialised dir) — re-extracts. Hidden,
/// and outside the methodology's own files, so it never collides with content
/// and Claude Code ignores it.
pub const STAMP_FILE: &str = ".grove-content-hash";

// The launcher reader that used to sit here is **gone rather than moved**. A
// mandate is composed from units now, so the driver reaches `content/` through
// `methodology::compose` and no caller wants a whole embedded file as a string —
// which leaves provisioning with one job, the sweep below.

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
        if stamp_of(destination).as_deref() == Some(methodology::identity()) {
            return Ok(());
        }
        provision_target(destination)?;
        eprintln!(
            "grove: restored the {} skill at {} — it did not carry this build's methodology ({})",
            harness.name,
            destination.display(),
            methodology::identity()
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
        if stamp == methodology::identity() {
            return Ok(());
        }
        eprintln!(
            "grove-llm: {} carries methodology {}, but this grove-llm was built with {} — one build owns a session",
            destination.display(),
            stamp.trim(),
            methodology::identity()
        );
        Ok(())
    });
}

/// Report — before every launch — that this machine has **no known harness root
/// at all**, so nothing was provisioned. A total failure that is otherwise
/// entirely silent.
///
/// It used to say *and a session's core points at a skill that is not there*.
/// Since `prompt-names-the-kind-k18` the core names a `grove-<kind>` skill from
/// the installed **plugin**, which this walk neither writes nor can see, so the
/// report is about provisioning alone until provisioning goes at
/// `delete-provisioning-k19`.
///
/// **It reports and never refuses**, on the line Grove's surface already draws:
/// it stops on what governs its own operation and reports what it can only
/// predict about a session's environment. Which harness an opaque configured
/// command reaches is firmly the latter — Grove executes that command directly
/// and cannot know what it is — so a refusal here would be a guess with a
/// launch riding on it.
///
/// **One installed root silences it, and that is deliberate.** Absence of a
/// destination is the only claim on offer: a machine with a known root has been
/// provisioned, and the weaker claim that remains — *we do not know whether your
/// harness reads it* — is one the driver has no standing to make. It would fire
/// on every correctly configured machine, every iteration, forever.
///
/// A home this process cannot locate is silent for the same reason
/// [`warn_on_foreign_skill_dirs`] is: nothing can be said about roots that
/// cannot be named. `reverify_installed` runs first on the same iteration and
/// fails loudly on that case anyway.
pub fn report_absent_skill_destination() {
    let Ok(home) = home_dir() else { return };
    if let Some(report) = absent_destination_report(&home) {
        eprintln!("{report}");
    }
}

/// The diagnostic for `home`, or `None` when any known root exists.
///
/// Split out as a pure function of the home so the rule is pinnable without
/// writing `$HOME`, which [`home_dir`] and `loop_driver`'s config lookup read in
/// parallel — the same reason [`skill_dir_in`] takes one.
fn absent_destination_report(home: &Path) -> Option<String> {
    if HARNESSES
        .iter()
        .any(|harness| home.join(harness.project_dir).is_dir())
    {
        return None;
    }
    let looked_for = HARNESSES
        .iter()
        .map(|harness| home.join(harness.project_dir).display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!(
        "grove: no known harness root exists, so nothing is provisioned and a session will \
         find no grove skill where its mandate points;\n       \
         looked for {looked_for} — the launch proceeds regardless."
    ))
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
/// Compared verbatim against [`methodology::identity`], exactly as
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
    sync_to_stamp(dest, methodology::identity(), |d| {
        methodology::embed()
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

    /// No known root → one diagnostic, naming every root it looked for by
    /// **absolute** path. A relative name would be unactionable: the whole point
    /// is telling the operator which directory to create.
    #[test]
    fn an_absent_destination_is_reported_and_names_every_root_by_absolute_path() {
        let home = TempDir::new().unwrap();

        let report = absent_destination_report(home.path())
            .expect("a home with no harness root has nothing provisioned into it");

        for harness in crate::harness::HARNESSES {
            let root = home.path().join(harness.project_dir);
            assert!(root.is_absolute());
            assert!(
                report.contains(root.to_str().unwrap()),
                "the report must name {}: {report}",
                root.display()
            );
        }
    }

    /// One installed root silences it — for *every* row, not just the first.
    /// Absence of a destination is the only claim on offer, and it cannot be
    /// made about a machine that has one.
    #[test]
    fn any_single_installed_root_silences_the_report() {
        for harness in crate::harness::HARNESSES {
            let home = TempDir::new().unwrap();
            std::fs::create_dir_all(home.path().join(harness.project_dir)).unwrap();

            assert_eq!(
                absent_destination_report(home.path()),
                None,
                "an installed {} root must silence the report",
                harness.name
            );
        }
    }

    /// A *file* at the marker path is not an installed harness, and the report
    /// must agree with the sweep about that — [`each_installed_skill_dir`] asks
    /// `is_dir`, so a stray file would otherwise be provisioned into nothing
    /// while the driver claimed a destination existed.
    #[test]
    fn a_file_at_a_marker_path_is_not_an_installed_root() {
        let home = TempDir::new().unwrap();
        std::fs::write(home.path().join(row("claude").project_dir), "not a dir").unwrap();

        assert!(
            absent_destination_report(home.path()).is_some(),
            "a file where a harness root would be is still no destination"
        );
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

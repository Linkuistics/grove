// Grove's directory-walking **grow** machinery, with **no production caller
// left**: the module is `#[cfg(test)]` and survives only as its own tests, until
// `sweep-k37` deletes it.
//
// The three grow *verbs* went to `src/task_grow.rs` in `growing-k33` — `leaf-add`,
// `leaf-add-pair` and `leaf-insert` are `ordinal-fs-tree`'s `append`,
// `append_many` and `insert` now. What kept this module alive after that was the
// lifecycle: `root_init` and `materialize_finish` allocated leaves while holding
// **grove's own** exclusive guard, and grove cannot nest its lock inside the
// library's — both `flock` the directory containing the tree root, and two open
// file descriptions on one directory do not share a lock. `lifecycle-k35`
// answered that by splitting the scaffold across the two guards rather than
// nesting them, which orphaned the path-walking allocator and `tree_id::next_key`
// with it.
//
// What is worth reading here is the **destination guard** and its tests:
// `refuse_occupied_destination` and `claim_destination` are why a dangling
// symlink at a planned name cannot become a write outside `.grove/`, and the
// library's interpreter now owns that hazard.
//
// **Position-free headers.** A task file's first-line `# …` header is the stable
// handle `# <slug>-k<key>` — the per-level position `NN` lives only in the
// filename, never in the body. The template itself is `task_grow`'s, so there is
// one spelling of it and not two.

use crate::leaf::Kind;
use crate::tree_id::{next_key, parse, validate_slug, Entry, Outcome};
use anyhow::{bail, Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Append a child leaf under the node directory `parent_dir` at the next gapless
/// per-level position, with a fresh permanent key. `parent_dir` is the grove root
/// (a root-level leaf) or a node directory under it (absolute, or relative to the
/// grove root). The new position is `max(direct-child position) + 1` (counting
/// leaves live AND `DONE`, and node directories, so a slot is never reused), the
/// new key is `max(key over the whole tree) + 1`. Working-tree only — no commit.
///
/// The kind is part of the filename; the body is the bare template — the stable
/// header and empty task sections — which the creating session then fills in.
///
/// **This verb is how a review chain is built now** (flat-lazy-review). A
/// producer's last act is `leaf-add <parent> <stem> --kind review-<producer>`
/// when review is required, and the review's last act is the matching
/// `integrate-review-<producer>` leaf when it has findings worth acting on —
/// every step slugged with the same bare stem, since the kind states its role.
/// The steps are flat siblings, so nothing here knows they compose one
/// artifact; the shared stem and the session-written `**Reviews:**` /
/// `**Integrates:**` lines are convention, not grammar.
pub(crate) fn leaf_add_unlocked(
    grove_root: &Path,
    parent_dir: &Path,
    slug: &str,
    kind: Kind,
) -> Result<PathBuf> {
    refuse_finish_kind(kind, "leaf-add")?;
    validate_slug(slug)?;
    let grove_abs = canonical_grove_root(grove_root)?;
    let parent_abs = resolve_parent_node(&grove_abs, parent_dir)?;

    let position = next_child_position(&parent_abs)?;
    let key = next_key(collect_all_names(&grove_abs)?)?;
    let entry = Entry::Leaf {
        position,
        kind,
        slug: slug.to_string(),
        key,
        outcome: Outcome::Live,
    };
    let path = parent_abs.join(entry.name());
    refuse_occupied_destination(&path, "")?;
    write_task_template(&path, slug, key)?;
    Ok(path)
}

// ---------------------------------------------------------------------------
// helpers

/// Validate the grove root exists and canonicalise it (so child paths resolve
/// against a real absolute root).
fn canonical_grove_root(grove_root: &Path) -> Result<PathBuf> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))
}

/// Resolve a `parent_dir` argument (the grove root, or a node directory under it,
/// absolute or grove-root-relative) to an absolute, validated node directory. The
/// grove root is always a valid parent (the root node); any other parent must be
/// an existing **directory whose name parses as a node**. That also rejects
/// pointing at a leaf *file* (a leaf has no children until decomposed), which
/// fails `is_dir` before the name is ever read.
///
/// **A `BRIEF.md` is not required, and it is not re-required now that every node
/// carries one again.** Every node grove *writes* has a charter — `leaf-decompose`
/// moves the decomposed leaf's body in as one, `root-init` scaffolds one — so
/// with chain nodes gone (flat-lazy-review) the two species collapse back to one.
/// That makes a charter something a well-formed tree *has*, not something this
/// verb *checks*: node-ness is a structural fact (a directory whose name parses
/// as a node), while a charter is content, and grove validates content nowhere
/// (constraint 3 — task files and briefs are freeform markdown). Reinstating the
/// guard would only refuse a hand-cut node whose charter has not been written
/// yet, which is a session grove has no business gating (constraint 5).
fn resolve_parent_node(grove_abs: &Path, parent_dir: &Path) -> Result<PathBuf> {
    let candidate = if parent_dir.is_absolute() {
        parent_dir.to_path_buf()
    } else {
        grove_abs.join(parent_dir)
    };
    // A missing path or a leaf *file* is not a node — both fail `is_dir`, giving
    // the same actionable "parent" diagnostic (rather than a raw canonicalise error).
    if !candidate.is_dir() {
        bail!(
            "parent is not a node directory (need a directory named NN-<slug>-k<key>): {}",
            candidate.display()
        );
    }
    let parent_abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving parent {}", candidate.display()))?;
    if !parent_abs.starts_with(grove_abs) {
        bail!(
            "parent {} is not under grove root {}",
            parent_abs.display(),
            grove_abs.display()
        );
    }
    if parent_abs == *grove_abs {
        return Ok(parent_abs); // the root node — always valid
    }
    let name = file_name(&parent_abs)?;
    if !matches!(parse(&name), Some(Entry::Node { .. })) {
        bail!(
            "parent is not a node directory (need a directory named NN-<slug>-k<key>): {}",
            parent_abs.display()
        );
    }
    Ok(parent_abs)
}

/// The next gapless per-level position in `dir`: `max(position) + 1` over its
/// direct children (leaves live and `DONE`, and node directories — a `DONE` slot
/// is never reused), or `1` for an empty node. Reconciles each name's parsed kind
/// with its real filesystem kind, so a mis-kinded foreign entry is not counted.
pub(crate) fn next_child_position(dir: &Path) -> Result<u32> {
    let max = read_children(dir)?
        .iter()
        .filter_map(|(e, _)| e.position())
        .max();
    Ok(max.map_or(1, |m| m + 1))
}

/// Read one directory's grove children — `(Entry, path)` for each name that parses
/// *and* whose real on-disk species matches, sorted by the per-level comparator.
/// The charter brief is included (callers filter it via `position()` returning
/// `None`).
///
/// **`tree_read::read_level` itself**, not a mirror of it: grow and read agreeing
/// on what a sibling is matters most where they *disagreed* — a task-shaped entry
/// the reader now refuses would otherwise stay invisible to `collect_all_names`,
/// and `next_key` would re-issue a permanent key that is still live inside it.
fn read_children(dir: &Path) -> Result<Vec<(Entry, PathBuf)>> {
    crate::tree_read::read_level(dir)
}

/// Recursively collect every grove entry's name in the tree (leaves live and
/// `DONE`, node directories, and briefs), descending node directories. Feeds
/// `next_key`, which filters to the keyed names — so this only needs every name.
pub(crate) fn collect_all_names(dir: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    collect_names_into(dir, &mut names)?;
    Ok(names)
}

fn collect_names_into(dir: &Path, out: &mut Vec<String>) -> Result<()> {
    for (entry, path) in read_children(dir)? {
        let descend = entry.is_node();
        if let Entry::Brief = entry {
            out.push("BRIEF.md".to_string());
        } else {
            out.push(file_name(&path)?);
        }
        if descend {
            collect_names_into(&path, out)?;
        }
    }
    Ok(())
}

/// The path's final component as an owned `String`.
fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .with_context(|| format!("path {} has no filename", path.display()))
}

/// Refuse a destination that is occupied by *anything*, and refuse a
/// destination whose occupancy cannot be determined.
///
/// **`Path::exists()` answers the wrong question twice.** It follows symlinks,
/// so a dangling one reports `false` — the destination reads as free while a
/// later `fs::write` follows the link and creates or truncates its target,
/// which may be anywhere on disk and outside `.grove/` entirely; a rollback
/// would then remove the link and leave that target behind. And it collapses
/// every other error (`EACCES`, `ELOOP`, a broken mount) into `false` too,
/// turning "I could not tell" into "go ahead".
///
/// `symlink_metadata` is the no-follow primitive: it stats the link itself, so
/// **only `NotFound` means free** and every other error is a refusal. `detail`
/// is appended to the message so each caller can say what the refusal means for
/// the tree it was midway through building.
///
/// This is a diagnostic, not the guarantee — see [`claim_destination`].
fn refuse_occupied_destination(path: &Path, detail: &str) -> Result<()> {
    match fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error)
            .with_context(|| format!("checking whether {} is free{detail}", path.display())),
        Ok(_) => bail!("destination already exists: {}{detail}", path.display()),
    }
}

/// Take a destination **atomically**, or fail: `O_CREAT | O_EXCL` in one
/// syscall ([`std::fs::OpenOptions::create_new`]).
///
/// This is what actually makes a grow verb collision-safe, and the up-front
/// sweep above is not. A sweep is a check at one instant and the write happens
/// at another; anything that occupies the destination in between — a writer
/// that ignored the tree lock, a symlink planted at the planned name — is
/// invisible to the sweep and fully visible to the write. `create_new` closes
/// that gap by construction: it fails with `AlreadyExists` on a regular file,
/// on a directory, and on a symlink **dangling or not** (`O_EXCL` refuses to
/// follow one), so it can neither truncate an entry Grove does not own nor
/// write through a link to a target outside `.grove/`.
///
/// The open file handle is the proof of ownership, and it is returned rather
/// than consumed here so the caller can record that ownership *before* any
/// bytes are written ([`add_run`]).
fn claim_destination(path: &Path) -> Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .with_context(|| format!("creating {}", path.display()))
}

/// Write a claimed leaf's template into the handle [`claim_destination`]
/// returned. The first-line header is the **position-free handle**
/// `# <slug>-k<key>` — the mutable per-level position lives only in the
/// filename, so a later renumber never rewrites this.
///
/// **One template, no parameters beyond the handle.** Session kind and harness
/// are launch-time configuration, and a step's relationship to its neighbours is
/// prose the *creating session* writes into the body afterwards
/// (`content/TASK-FORMAT.md`, the `**Reviews:**` / `**Integrates:**`
/// convention). That is the whole point of creating a review step late: the
/// session that cuts it knows the specific finding or uncovered case the step
/// exists for, which no constructor rendering a goal sentence from a handle
/// could.
fn fill_task_file(mut file: fs::File, path: &Path, slug: &str, key: u32) -> Result<()> {
    file.write_all(task_template_body(slug, key).as_bytes())
        .with_context(|| format!("writing {}", path.display()))
}

/// Claim a destination and fill it — the whole of creating one leaf.
///
/// The **created-but-not-filled** window between the two syscalls is real —
/// `write_all` can return `ENOSPC` or `EIO` with the file already on disk — and
/// it used to be armed by an injected fault, because the composite verb's
/// rollback was the thing that had to survive it. There is no run to unwind any
/// more: the one caller left creates exactly one leaf, so the residue is that
/// leaf, empty, at a valid name, and the seam that made the arm possible went
/// with the verb it served.
pub(crate) fn write_task_template(path: &Path, slug: &str, key: u32) -> Result<()> {
    let file = claim_destination(path)?;
    fill_task_file(file, path, slug, key)
}

/// The task-file template, which is `task_grow`'s and not this module's.
///
/// One spelling of the template, reached from both allocators: a leaf the
/// lifecycle scaffolds has to be byte-identical to one `leaf-add` cuts, and two
/// format strings that must agree are one that will not.
pub(crate) fn task_template_body(slug: &str, key: u32) -> String {
    crate::task_grow::task_template_body(slug, key)
}

fn refuse_finish_kind(kind: Kind, verb: &str) -> Result<()> {
    if kind == Kind::Finish {
        bail!("`finish` is driver-reserved and cannot be created by `{verb}`");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    //! What is left to test here is the **destination guard**, which is the part
    //! of this module the grow verbs did not take with them: the appending
    //! lifecycle verbs still write through it, and `Path::exists()` answers the
    //! wrong question in two ways that both end in a lost file.
    //!
    //! The verbs themselves are tested in `src/task_grow/tests.rs` now, against
    //! the library. What remains of `leaf_add_unlocked` is covered where its
    //! callers are — `tree_lifecycle`'s `root_init` and `materialize_finish` —
    //! plus the allocation check below, which is the one property no lifecycle
    //! test states directly.

    use super::*;
    use tempfile::TempDir;

    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        crate::tree_format::write_current_last(&root).unwrap();
        (tmp, root)
    }

    fn leaf_add(grove_root: &Path, parent_dir: &Path, slug: &str, kind: Kind) -> Result<PathBuf> {
        let guard = crate::tree_access::write(grove_root)?;
        leaf_add_unlocked(guard.root(), parent_dir, slug, kind)
    }

    fn name_of(path: &Path) -> String {
        path.file_name().unwrap().to_string_lossy().into_owned()
    }

    #[test]
    fn the_lifecycle_allocator_appends_at_the_next_slot_with_a_tree_wide_key() {
        // The two counters the lifecycle verbs depend on, in one fixture: the
        // position is the *level's* greatest plus one, and the key is the
        // *tree's*. A `DONE` sibling still holds its slot, and a deeper subtree
        // still raises the key.
        let (_t, g) = grove();
        fs::write(g.join("BRIEF.md"), b"# root\n").unwrap();
        fs::write(g.join("01-DONE-impl-a-k1.md"), b"# a-k1\n").unwrap();
        let node = g.join("02-design-k2");
        fs::create_dir(&node).unwrap();
        fs::write(node.join("BRIEF.md"), "# design-k2 — brief\n").unwrap();
        fs::write(node.join("01-impl-deep-k9.md"), b"# deep-k9\n").unwrap();

        let got = leaf_add(&g, &g, "next", Kind::Impl).unwrap();

        assert_eq!(name_of(&got), "03-impl-next-k10.md");
        assert_eq!(
            fs::read_to_string(&got).unwrap(),
            task_template_body("next", 10)
        );
    }

    #[test]
    fn the_lifecycle_allocator_refuses_the_drivers_own_kind() {
        let (_t, g) = grove();
        fs::write(g.join("BRIEF.md"), b"# root\n").unwrap();
        let err = leaf_add(&g, &g, "x", Kind::Finish).unwrap_err().to_string();
        assert!(err.contains("driver-reserved"), "got {err}");
    }

    #[test]
    fn a_dangling_symlink_occupies_a_destination_rather_than_reading_as_absent() {
        // `Path::exists()` follows symlinks and so reports a dangling one as
        // *absent* — the destination reads free, and a following `fs::write`
        // creates the link's target, which may be anywhere on disk and is here
        // deliberately outside `.grove/`.
        let (t, g) = grove();
        let outside = t.path().join("outside.txt");
        let dangling = g.join("01-impl-a-k1.md");
        std::os::unix::fs::symlink(&outside, &dangling).unwrap();

        let err = refuse_occupied_destination(&dangling, "")
            .unwrap_err()
            .to_string();

        assert!(
            err.contains("already exists"),
            "a dangling link is an occupied destination: {err}"
        );
        assert!(!outside.exists(), "the guard must not resolve the link");
    }

    #[test]
    fn a_claim_can_neither_clobber_nor_follow_whatever_occupies_its_destination() {
        // The guarantee the up-front check cannot give. A check is at one
        // instant and the write happens at another, so a writer that ignored the
        // tree lock can occupy a planned destination in between; what makes that
        // harmless is that the write itself refuses. Pinned against every
        // species that can sit at the name — including the two `fs::write` would
        // have silently accepted.
        let (t, g) = grove();
        let outside = t.path().join("outside.txt");

        let regular = g.join("occupied.md");
        fs::write(&regular, "pre-existing bytes").unwrap();
        let directory = g.join("a-directory");
        fs::create_dir(&directory).unwrap();
        let dangling = g.join("dangling.md");
        std::os::unix::fs::symlink(&outside, &dangling).unwrap();
        let live = g.join("live.md");
        std::os::unix::fs::symlink(&regular, &live).unwrap();

        for occupied in [&regular, &directory, &dangling, &live] {
            assert!(
                claim_destination(occupied).is_err(),
                "claiming {} must fail",
                occupied.display()
            );
        }
        assert_eq!(
            fs::read_to_string(&regular).unwrap(),
            "pre-existing bytes",
            "an entry Grove does not own must not be truncated"
        );
        assert!(
            !outside.exists(),
            "no claim may write through a link to {}",
            outside.display()
        );
    }

    #[test]
    fn an_undeterminable_destination_is_a_refusal_rather_than_a_green_light() {
        // The other half of `Path::exists()`'s wrong answer: it collapses every
        // I/O error into `false`, so "I could not tell" reads as "go ahead". An
        // unreadable parent is the portable form — `symlink_metadata` fails with
        // `EACCES` rather than `NotFound`, and only `NotFound` means free.
        let (_t, g) = grove();
        let sealed = g.join("sealed");
        fs::create_dir(&sealed).unwrap();
        let mut mode = fs::metadata(&sealed).unwrap().permissions();
        std::os::unix::fs::PermissionsExt::set_mode(&mut mode, 0o000);
        fs::set_permissions(&sealed, mode.clone()).unwrap();

        let refused = refuse_occupied_destination(&sealed.join("01-impl-a-k1.md"), "");

        std::os::unix::fs::PermissionsExt::set_mode(&mut mode, 0o755);
        fs::set_permissions(&sealed, mode).unwrap();
        let err = refused.unwrap_err().to_string();
        assert!(
            err.contains("is free"),
            "the refusal says what it was: {err}"
        );
    }
}

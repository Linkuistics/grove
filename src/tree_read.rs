// What is **left** of grove's path-walking reader after `reading-k31` moved the
// reading surface onto `ordinal-fs-tree` (gh issue #13, increment 2).
//
// `pick`, `select`, `brief-chain`, `kind` and `resolve` now live in
// `src/task_tree.rs` and read one snapshot under the library's shared lock.
// What stays here is exactly what the verbs that have **not** flipped yet still
// need, and it stays for one reason: grove's exclusive guard and the library's
// cannot be nested. Both `flock` the directory *containing* the tree root, and
// two open file descriptions on one directory do not share a lock, so a verb
// holding `tree_access::write` cannot call into the library's reader without
// deadlocking against itself. Each of these dies with the leaf that flips its
// last caller, and `sweep-k37` checks that they are gone.
//
//   * `select_unlocked` — `tree_lifecycle`, under the lifecycle write guard;
//   * `resolve_unlocked` — `llm_cli`'s `<parent>` / `<target>` arguments, under
//     the grow verbs' write guard;
//   * `read_level` — `tree_grow` and `tree_lifecycle`, the one strict level
//     reader they share.
//
// **This is prior art and never authority.** `tree_id`'s grammar is deliberately
// lenient where `task_name`'s is canonical, so the two readers disagree about a
// hand-typed `5-…`: this one accepts it and the library halts on it. Both are
// live at once by construction — that is what *expand → migrate → contract*
// buys — and the equivalence tests at the bottom of this module are what turns
// that overlap into evidence rather than risk.

use crate::leaf::Kind;
#[cfg(test)]
use crate::task_name::Outcome as TaskOutcome;
use crate::task_tree::SelectedLeaf;
#[cfg(test)]
use crate::task_tree::{handle_key, parse_ref, AmbiguousMatch, Ref, Resolution};
// Only the in-test guard assertion reaches it: these helpers are lock-neutral,
// and the guard is the caller's.
#[cfg(test)]
use crate::tree_access;
use crate::tree_id::{parse_current, sort_key, Entry, Outcome};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// The [`TaskOutcome`] `resolve` reports for a matched entry: a leaf's own
/// live/`DONE`/`ABANDONED` state, or `Live` for a node, which carries no
/// terminal state of its own.
///
/// Crossing from `tree_id`'s outcome to `task_name`'s here, rather than keeping
/// a second `Resolution` type alive, is what stops the two readers from
/// answering in two vocabularies while both are live.
#[cfg(test)]
fn entry_outcome(entry: &Entry) -> TaskOutcome {
    match entry {
        Entry::Leaf { outcome, .. } => match outcome {
            Outcome::Live => TaskOutcome::Live,
            Outcome::Done => TaskOutcome::Done,
            Outcome::Abandoned => TaskOutcome::Abandoned,
        },
        Entry::Node { .. } | Entry::Brief => TaskOutcome::Live,
    }
}

/// Select one live leaf and copy every launch fact while the caller's guard is
/// held. The lock-neutral half of what `task_tree::select` now does through the
/// library; `tree_lifecycle` calls it under the lifecycle write guard, which the
/// library's reader cannot be nested inside.
pub(crate) fn select_unlocked(grove_root: &Path) -> Result<Option<SelectedLeaf>> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    let mut live = Vec::new();
    collect_live_leaf_entries(grove_root, &mut live)?;
    let finish = live
        .iter()
        .filter(|(entry, _)| entry.kind() == Some(Kind::Finish))
        .collect::<Vec<_>>();
    if finish.len() > 1 {
        bail!(
            "multiple live `finish` leaves are malformed: {}",
            finish
                .iter()
                .map(|(_, path)| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    let selected = if let Some(selected) = live
        .iter()
        .find(|(entry, _)| entry.kind() != Some(Kind::Finish))
    {
        Some(selected)
    } else {
        finish.first().copied()
    };
    selected
        .map(|(entry, path)| {
            Ok(SelectedLeaf {
                path: path.clone(),
                handle: entry
                    .handle()
                    .context("selected live leaf has no stable handle")?,
                kind: entry
                    .kind()
                    .context("selected live leaf has no session kind")?,
            })
        })
        .transpose()
}

fn collect_live_leaf_entries(dir: &Path, live: &mut Vec<(Entry, PathBuf)>) -> Result<()> {
    for (entry, path) in read_level(dir)? {
        match &entry {
            Entry::Leaf {
                outcome: Outcome::Live,
                ..
            } => live.push((entry, path)),
            Entry::Node { .. } => collect_live_leaf_entries(&path, live)?,
            Entry::Brief
            | Entry::Leaf {
                outcome: Outcome::Done | Outcome::Abandoned,
                ..
            } => {}
        }
    }
    Ok(())
}

/// Resolve a reference against the whole directory tree, lock-neutrally.
///
/// **A test oracle now, and nothing else.** Its last production caller was the
/// grow verbs' `<parent>` / `<target>` resolution, which `growing-k33` moved onto
/// the snapshot — so this survives only to keep
/// [`both_readers_resolve_every_reference_form_identically`](tests::both_readers_resolve_every_reference_form_identically)
/// possible while both readers exist. That test is the one direct check the
/// flip's *pure refactor* premise gets, and it is cheaper to keep the
/// implementation it compares against than to lose the evidence a leaf early.
/// Both die together in `sweep-k37`.
#[cfg(test)]
pub(crate) fn resolve_unlocked(grove_root: &Path, reference: &str) -> Result<Resolution> {
    #[cfg(test)]
    tree_access::assert_guard_held(grove_root);

    let mut all = Vec::new();
    collect_all(grove_root, &mut all)?;

    // Keys are unique tree-wide → at most one match; a node resolves to its
    // directory path (the dir name carries the key).
    let find_by_key = |key: u32| -> Resolution {
        all.iter()
            .find(|(e, _)| e.key() == Some(key))
            .map_or(Resolution::NotFound, |(e, path)| Resolution::Found {
                path: path.clone(),
                outcome: entry_outcome(e),
            })
    };

    match parse_ref(reference)? {
        Ref::Key(key) => Ok(find_by_key(key)),
        Ref::Slug(slug) => {
            // The brief (key None) is excluded — it is unreferenceable.
            let matches: Vec<AmbiguousMatch> = all
                .iter()
                .filter_map(|(e, path)| {
                    let key = e.key()?;
                    (e.slug() == Some(slug.as_str())).then(|| AmbiguousMatch {
                        key,
                        path: path.clone(),
                        outcome: entry_outcome(e),
                    })
                })
                .collect();
            Ok(match matches.as_slice() {
                // No slug match: retry as the full `<slug>-k<key>` handle (task-tree-scheme
                // §5's canonical commit/prose form), reading its terminal
                // `-k<digits>` as the key. Slugs are matched first, so a real slug
                // that itself ends in `-k<digits>` still wins — the fallback only
                // fires when the slug matched nothing.
                [] => match handle_key(&slug) {
                    Some(key) => find_by_key(key),
                    None => Resolution::NotFound,
                },
                [m] => Resolution::Found {
                    path: m.path.clone(),
                    outcome: m.outcome,
                },
                _ => Resolution::Ambiguous(matches),
            })
        }
    }
}

/// Read one directory's grove entries, parsed and sorted by the per-level
/// comparator (the charter brief first, then by numeric position, foreign last).
/// Returns `(Entry, path)` for every child whose name parses **and** whose real
/// on-disk species agrees with the parse.
///
/// This is where the **species half** of the task-shaped strictness rule lands
/// (`tree_id`'s module header states it whole): `tree_id::parse` infers
/// leaf-vs-node from the `.md` suffix alone, so a *directory* named `…-k1.md` or a
/// *file* shaped like a node name is a task-shaped name whose entry is not the
/// species it declares — a **malformed tree**, refused here, not a foreign entry
/// skipped. Skipping is what made a whole live subtree vanish under a hand-typed
/// `01-DONE-node-k1/`, and the leaf-side rule exists to prevent exactly that.
/// `BRIEF.md` is outside the rule — it carries no position and no key, a node with
/// no charter is legal everywhere, and so nothing is lost by ignoring an oddity at
/// that name.
///
/// **The one such reader in the crate.** `tree_grow` and `tree_lifecycle` call it
/// rather than keeping their own copies, so grow, retire, prune, key allocation
/// and `pick` cannot disagree about what a sibling is — and, in particular, a
/// subtree `pick` refuses can never be a subtree `next_key` silently skips,
/// re-issuing a live permanent key.
pub(crate) fn read_level(dir: &Path) -> Result<Vec<(Entry, PathBuf)>> {
    let mut entries: Vec<(String, Entry, PathBuf)> = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(parsed) = parse_current(&name)
            .with_context(|| format!("reading current Grove entry {}", entry.path().display()))?
        else {
            continue;
        };
        // `file_type` does not follow symlinks, so a symlink is neither a regular
        // file nor a directory and fails every species test below — deliberately:
        // followed, it would hand `pick` a path resolving outside the tree.
        let file_type = entry.file_type().ok();
        if matches!(parsed, Entry::Brief) {
            if !file_type.is_some_and(|t| !t.is_dir()) {
                continue;
            }
        } else {
            let (declared, ok) = if parsed.is_node() {
                ("node directory", file_type.is_some_and(|t| t.is_dir()))
            } else {
                ("leaf", file_type.is_some_and(|t| t.is_file()))
            };
            if !ok {
                bail!(
                    "malformed Grove tree entry {}: the name is task-shaped and declares a \
                     {declared}, so the entry must be {}. Restore it to the species its name \
                     declares, or rename it out of the NN-<slug>-k<key> grammar if it is not \
                     Grove's.",
                    entry.path().display(),
                    if parsed.is_node() {
                        "a directory"
                    } else {
                        "a regular file"
                    }
                );
            }
        }
        entries.push((name, parsed, entry.path()));
    }
    entries.sort_by_key(|a| sort_key(&a.0));
    Ok(entries.into_iter().map(|(_, e, p)| (e, p)).collect())
}

/// Recursively collect every parsed entry in the tree — leaves (live and `DONE`),
/// node directories, and briefs — each with its absolute path, in pre-order.
/// The shared scan behind [`resolve_unlocked`]'s tree-wide key/slug search, and
/// so, like it, a test oracle.
#[cfg(test)]
fn collect_all(dir: &Path, out: &mut Vec<(Entry, PathBuf)>) -> Result<()> {
    for (entry, path) in read_level(dir)? {
        let descend = entry.is_node();
        out.push((entry, path.clone()));
        if descend {
            collect_all(&path, out)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task_tree;
    use tempfile::TempDir;

    /// A fixture exercising every shape both readers have to agree about: two
    /// levels, a charter at each, live / `DONE` / `ABANDONED` leaves, a node
    /// directory, an ambiguous slug, a handle-shaped slug, and a foreign file.
    fn fixture() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        crate::tree_format::write_current_last(&root).unwrap();
        let write = |dir: &Path, name: &str| {
            fs::write(dir.join(name), b"# stub\n").unwrap();
        };
        write(&root, "BRIEF.md");
        write(&root, "README.md");
        let node = root.join("01-design-k1");
        fs::create_dir_all(&node).unwrap();
        write(&node, "BRIEF.md");
        write(&node, "01-impl-add-k2.md");
        write(&node, "02-DONE-impl-remove-k3.md");
        write(&root, "02-ABANDONED-impl-add-k4.md");
        write(&root, "03-impl-build-k5.md");
        (tmp, root)
    }

    /// **The flip's premise, held directly.** Both readers are live at once
    /// during the migrate stage — the library's for the verbs that have flipped,
    /// this one for the verbs still under grove's own exclusive guard — and the
    /// claim `grove-flip-k28` makes is that the flip is a *pure refactor*. That
    /// claim is falsifiable only while both implementations exist, which is now.
    ///
    /// Every reference form, against a tree carrying each shape the grammar
    /// admits: key, decorative-slug key, bare slug, ambiguous slug, full handle,
    /// handle-shaped-but-unmatched, and the unreferenceable root brief.
    #[test]
    fn both_readers_resolve_every_reference_form_identically() {
        let (_tmp, root) = fixture();
        for reference in [
            "[1]",
            "1",
            "[2]-add",
            "2",
            "add",
            "build",
            "remove",
            "design-k1",
            "add-k4",
            "missing-k99",
            "nothing",
            "BRIEF",
            "5",
        ] {
            let flipped = task_tree::resolve(&root, reference).unwrap();
            let path_walking = {
                let guard = tree_access::read(&root).unwrap();
                resolve_unlocked(guard.root(), reference).unwrap()
            };
            assert_eq!(
                flipped, path_walking,
                "the two readers disagree about {reference:?}"
            );
        }
    }

    /// The same claim for selection, which is the one every launch depends on:
    /// walk order, the live/`DONE`/`ABANDONED` filter, and the `finish` rule.
    #[test]
    fn both_readers_select_the_same_live_leaf() {
        let (_tmp, root) = fixture();
        let flipped = task_tree::select(&root).unwrap();
        let path_walking = {
            let guard = tree_access::read(&root).unwrap();
            select_unlocked(guard.root()).unwrap()
        };
        assert_eq!(
            flipped.map(|selection| (selection.path, selection.handle, selection.kind)),
            path_walking.map(|selection| (selection.path, selection.handle, selection.kind)),
        );
    }
}

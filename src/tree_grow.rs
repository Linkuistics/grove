// The **grow verbs** (task-tree-scheme) — `leaf-add`, `leaf-insert`, and the
// composite `leaf-add-pair` (which emits its three steps as **flat siblings**;
// flat-lazy-review) — expressed against
// the real **directory tree**, built on the id model (`src/tree_id.rs`). Keeps
// task-tree-scheme's *semantics* (append a gapless child with a fresh permanent
// key; insert shifts the occupant + later siblings up by one) with the mechanics
// the filesystem's shape allows:
//
//   * the superseded v1-flat scheme baked the whole dotted path into every
//     filename, so inserting at `2.2` rewrote `2.2.1`→`2.3.1`… across the **whole
//     subtree** — O(subtree) filename + header rewrites.
//   * directories carry the hierarchy instead (a node is a *directory* holding its
//     children, and optionally a `BRIEF.md` charter — see `tree_id`'s header for
//     the two species), so a renumber is a single **rename of a directory** and the
//     subtree — child names *and* keys — rides along untouched. The shift is
//     O(siblings at one level), the "cascade collapse" task-tree-scheme celebrates.
//
// **Position-free headers.** A task file's first-line `# …` header is the *stable
// handle* `# <slug>-k<key>` — the per-level position `NN` lives only in the
// filename, never in the body. This is the faithful realization of
// task-tree-scheme §5 ("reference a work item by `<slug>-k<key>`, never by its
// position/path") and it makes the renumber a **pure rename with zero content
// rewrites**: shifting `05-mid-k14/`→`06-mid-k14/` changes one directory name and
// nothing else — the moved node's own `BRIEF.md` header (`# mid-k14 — brief`) and
// every descendant file stay byte-identical. `tree_migrate` carries the same rule
// backwards, rewriting v1 `# <dotted>-[<key>]-<slug>` headers down to the handle.

use crate::leaf::Kind;
use crate::tree_id::{next_key, next_keys, parse, validate_slug, Entry, Outcome};
use crate::tree_rename::rename_entry;
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
/// producer's last act is `leaf-add <parent> <stem>-review --kind
/// review-<producer>` when review is required, and the review's last act is the
/// matching `integrate-review-<producer>` leaf when it has findings worth acting
/// on. The steps are flat siblings, so nothing here knows they compose one
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

/// One leaf of the research pair: the slug the verb derived for it and its
/// kind. Private — the shape, not the step, is what a caller names
/// ([`leaf_add_pair_unlocked`]).
struct Step {
    slug: String,
    kind: Kind,
}

/// Append a whole **shape** — its steps as **flat siblings** at consecutive
/// per-level positions — under `parent_dir` as *one mutation*. The engine behind
/// [`leaf_add_pair_unlocked`], which is the only shape a constructor still emits.
///
/// **There is no node directory, and that is the design rather than an
/// omission** (flat-lazy-review). A node means *this work proved bigger than one
/// session*, and a composed shape has no such context; the hierarchy the node
/// bought was not worth its navigation cost, and every reader already handles a
/// run of flat siblings. So a shape's steps sit beside their neighbours, and the
/// only thing that groups them is the shared stem in their slugs — a convention
/// nothing parses, exactly as the review chain's `-review` / `-integrate` suffix
/// is.
///
/// **This is not `leaf_add_unlocked` in a loop, and the difference is the whole
/// point of a composite verb.** `leaf_add_unlocked` validates, allocates and
/// writes in one breath, so three calls give three chances to stop half-way —
/// and a live prefix of a pair looks exactly like a deliberately hand-cut
/// partial one, which is the wrong-but-well-formed residue the verb exists to
/// prevent. So:
///
/// * **every slug is validated, and the parent resolved, before the first
///   write** — a bad third slug refuses without creating the first two;
/// * **the positions and keys come from one snapshot**, so the steps land at the
///   parent's next free slots and their keys run consecutively;
/// * **every destination is checked free before anything is written**, which is
///   what makes the realistic collision an up-front refusal rather than a
///   rollback. Flat siblings have no enclosing directory to make one check cover
///   the shape, so the sweep is per-step — the cost the flat form pays back for
///   the navigation the node charged;
/// * **anything that still fails mid-write rolls the run back**, so the
///   observable outcome is the whole shape or none of it.
///
/// Working-tree only — no commit, exactly like [`leaf_add_unlocked`].
fn add_run(grove_root: &Path, parent_dir: &Path, steps: &[Step]) -> Result<Vec<PathBuf>> {
    for step in steps {
        validate_slug(&step.slug)?;
    }
    let grove_abs = canonical_grove_root(grove_root)?;
    let parent_abs = resolve_parent_node(&grove_abs, parent_dir)?;

    // One snapshot for the whole run: the steps take the parent's next free
    // slots and the tree-wide next keys. Re-reading per step would give the same
    // answer today — nothing else is writing — but deriving both once is what
    // makes contiguous positions and consecutive keys a property rather than a
    // race.
    //
    // **The whole key run is allocated here, before the first write**, rather
    // than derived per step from the first. Deriving them later is unchecked
    // arithmetic *after* the first file lands, which is precisely the window this
    // verb exists to keep closed. `next_keys` refuses an exhausted keyspace, so
    // that refusal lands here, with the tree untouched.
    let first_position = next_child_position(&parent_abs)?;
    let step_count = u32::try_from(steps.len()).expect("a shape has a handful of steps");
    let keys = next_keys(collect_all_names(&grove_abs)?, step_count)
        .context("allocating the shape's keys (nothing was created)")?;

    let planned: Vec<PathBuf> = steps
        .iter()
        .zip(keys.iter())
        .enumerate()
        .map(|(index, (step, key))| {
            parent_abs.join(
                Entry::Leaf {
                    position: first_position + index as u32,
                    kind: step.kind,
                    slug: step.slug.clone(),
                    key: *key,
                    outcome: Outcome::Live,
                }
                .name(),
            )
        })
        .collect();
    // Every destination, before the first write. The node shape got this for
    // free from one `create_dir` on a directory it had just proven absent;
    // flat siblings have to be swept, and the sweep must cover *all* of them —
    // checking only the first would let a squatter at the third turn a refusal
    // into a rollback.
    //
    // **The sweep is the diagnostic, not the guarantee.** It buys the good
    // refusal — "nothing was created", naming the obstruction, before any leaf
    // lands — and nothing more, because a writer that ignored the tree lock can
    // still occupy a destination in the gap between this loop and the write
    // below. What closes that gap is the atomic claim each write now takes
    // ([`claim_destination`]); this loop just makes the realistic collision
    // read as a refusal rather than a rollback.
    for path in &planned {
        refuse_occupied_destination(path, " (nothing was created)")?;
    }

    let mut created: Vec<PathBuf> = Vec::with_capacity(planned.len());
    for ((step, key), path) in steps.iter().zip(keys.iter()).zip(planned.iter()) {
        let file = match claim_destination(path) {
            Ok(file) => file,
            Err(error) => return Err(roll_back(error, &created)),
        };
        // **Ownership is recorded the instant the claim succeeds, before a
        // single byte is written.** Creating and filling are two syscalls, and
        // the second can fail (`ENOSPC`, `EIO`) with the file already on disk —
        // an empty, well-formed leaf at a valid name, which is exactly the
        // wrong-but-well-formed residue this verb exists to prevent. Pushing
        // after the fill would leave that one path outside `created` and so
        // outside the rollback, undoing every leaf *except* the one the failure
        // just made.
        created.push(path.clone());
        if let Err(error) = fill_task_file(file, path, &step.slug, *key) {
            return Err(roll_back(error, &created));
        }
    }
    Ok(created)
}

/// Undo a partially-written run and return the error the caller should see.
///
/// Every path in `created` was **atomically created by this run**
/// ([`claim_destination`]), which is what makes removing exactly those both
/// complete and safe by construction: not "proven absent a moment ago" — a
/// claim that would have gone stale in the gap — but an entry that did not
/// exist until this run's own `O_EXCL` create made it. So a rollback can never
/// eat a pre-existing entry, and never follows a link to something outside
/// `.grove/`. A removal that itself fails is *named*: a residue the operator
/// must know about is worse hidden than reported.
///
/// **Reported errors only.** This runs when control returns through the error
/// path; process death mid-run bypasses it and leaves a partial shape, which is
/// the promise `docs/ARCHITECTURE.md` makes and the stronger one it does not.
fn roll_back(cause: anyhow::Error, created: &[PathBuf]) -> anyhow::Error {
    let mut stranded = Vec::new();
    for path in created.iter().rev() {
        if fs::remove_file(path).is_err() {
            stranded.push(path.display().to_string());
        }
    }
    if stranded.is_empty() {
        cause.context("creating the shape failed; the leaves it had created were rolled back")
    } else {
        cause.context(format!(
            "creating the shape failed and rollback could not remove {} — \
             delete them by hand before retrying, or the retry will append a duplicate shape",
            stranded.join(", ")
        ))
    }
}

/// Append a whole **research pair** — `<stem>-a` / `<stem>-b` /
/// `<stem>-combine` as three flat siblings — under `parent_dir`, in one mutation
/// ([`add_run`]). Returns the three paths in position order.
///
/// The producers are `-a` / `-b` rather than a bare stem beside a `-second`,
/// because they are peers rather than a producer and a step.
///
/// The steps have fixed filename kinds `research-a`, `research-b`, and
/// `combine-research`. Their commands and any desired independence are launch
/// configuration, never metadata in these task bodies.
///
/// **The pair stays eager while the review chain went lazy** (flat-lazy-review).
/// Lazy creation is actively *wrong* here: a `research-b` cut by `research-a`'s
/// own session would inherit that session's framing and corpus, and the
/// independence of the two corpora is the entire reason a pair is run.
pub(crate) fn leaf_add_pair_unlocked(
    grove_root: &Path,
    parent_dir: &Path,
    stem: &str,
) -> Result<Vec<PathBuf>> {
    // The stem is validated in its own right, not merely through the slugs it
    // builds: `foo-` is a bad slug, but `foo--a` would pass, so validating only
    // the derived names would let a malformed stem through on all three.
    validate_slug(stem)?;
    add_run(
        grove_root,
        parent_dir,
        &[
            Step {
                slug: format!("{stem}-a"),
                kind: Kind::ResearchA,
            },
            Step {
                slug: format!("{stem}-b"),
                kind: Kind::ResearchB,
            },
            Step {
                slug: format!("{stem}-combine"),
                kind: Kind::CombineResearch,
            },
        ],
    )
}

/// One entry of a `leaf-insert` renumber: an existing sibling whose per-level
/// position shifted up by one. The key and slug (and, for a node, its whole
/// subtree) are invariant — only the `NN` in this one entry's own name changes —
/// so a `Renumber` records just the position move and the names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Renumber {
    pub(crate) old_position: u32,
    pub(crate) new_position: u32,
    pub(crate) old_name: String,
    pub(crate) new_name: String,
}

/// Insert a new leaf at the slot currently held by `target`, shifting `target`
/// and every later sibling in its directory up by one position. `target` is an
/// existing entry — a leaf file or a node directory (absolute, or relative to the
/// grove root). Each shift is a single rename whose source and destination differ
/// only in the leading `NN`; a node directory carries its whole subtree along.
/// Renames run **highest-position-first** so each destination is already vacated.
/// The new leaf gets a fresh key. (Inserting past the last sibling is
/// `leaf_add_unlocked`'s job — `target` must exist.) Working-tree only — no
/// commit; siblings grown this session are untracked and rename fine
/// ([`crate::tree_rename`]).
///
/// Returns the new leaf's path and the renumber log (ascending by new position);
/// pass the log to [`surface_cross_refs_unlocked`] to lint stray
/// position-prefixed refs.
pub(crate) fn leaf_insert_unlocked(
    grove_root: &Path,
    target: &Path,
    slug: &str,
    kind: Kind,
) -> Result<(PathBuf, Vec<Renumber>)> {
    refuse_finish_kind(kind, "leaf-insert")?;
    validate_slug(slug)?;
    let grove_abs = canonical_grove_root(grove_root)?;
    let target_abs = resolve_under_root(&grove_abs, target)?;

    let target_name = file_name(&target_abs)?;
    let target_entry = parse(&target_name)
        .with_context(|| format!("target is not a grove entry: {target_name}"))?;
    let pos = target_entry
        .position()
        .with_context(|| format!("cannot insert at the brief: {}", target_abs.display()))?;
    let parent_abs = target_abs
        .parent()
        .with_context(|| format!("target {} has no parent", target_abs.display()))?
        .to_path_buf();

    // The new leaf's fresh key is stable across the renumber (renames preserve
    // keys), so it is computed once over the pre-renumber tree.
    let new_key = next_key(collect_all_names(&grove_abs)?)?;

    // Every positioned sibling at or after the target's slot shifts up by one.
    let mut affected: Vec<(Entry, PathBuf)> = read_children(&parent_abs)?
        .into_iter()
        .filter(|(e, _)| e.position().is_some_and(|p| p >= pos))
        .collect();
    // Highest position first so each rename's destination is already vacated:
    // a sibling's destination differs from its source only in `NN`, where it is
    // larger, so it sorts later and was moved first (collision-free).
    affected.sort_by_key(|b| std::cmp::Reverse(b.0.position()));

    let mut renumbers = Vec::with_capacity(affected.len());
    for (entry, path) in &affected {
        let old_position = entry.position().expect("positioned by the filter");
        let new_position = old_position + 1;
        let old_name = file_name(path)?;
        let new_name = bumped(entry, new_position).name();
        rename_entry(&parent_abs, &old_name, &new_name)?;
        renumbers.push(Renumber {
            old_position,
            new_position,
            old_name,
            new_name,
        });
    }
    // Report the log ascending by new position (the renames ran highest-first).
    renumbers.sort_by_key(|r| r.new_position);

    let entry = Entry::Leaf {
        position: pos,
        kind,
        slug: slug.to_string(),
        key: new_key,
        outcome: Outcome::Live,
    };
    let path = parent_abs.join(entry.name());
    refuse_occupied_destination(
        &path,
        &format!(" after renumber (renumber log: {renumbers:?})"),
    )?;
    write_task_template(&path, slug, new_key)?;
    Ok((path, renumbers))
}

/// Surface stray **position-prefixed** cross-references left stale by a
/// `leaf-insert` renumber, as a lint on stderr — never an auto-rewrite
/// (task-tree-scheme §5: durable references should use the stable `<slug>-k<key>` handle, which a
/// renumber never changes, so the operator reviews each occurrence). Scans every
/// `.md` body in the tree (recursively) for the **old on-disk name** of each
/// renamed entry (`05-mid-k14`), emitting one `path:line: <old-name> (context)`
/// per hit. A stable `<slug>-k<key>` reference is *not* surfaced (it did not move);
/// only the position-prefixed form is stale. Empty renumber log ⇒ nothing to do.
pub(crate) fn surface_cross_refs_unlocked(
    grove_root: &Path,
    renumbers: &[Renumber],
    out: &mut impl std::io::Write,
) -> Result<()> {
    if renumbers.is_empty() {
        return Ok(());
    }
    // The stale tokens are the *old* position-prefixed names the renumber moved
    // (`02-mid-k3`), with any `.md` extension dropped so a path reference
    // `02-mid-k3/01-impl-x-k4.md` matches the directory token. The `-k<digits>` tail
    // makes these specific enough to scan as plain substrings.
    let stale: Vec<String> = renumbers
        .iter()
        .map(|r| stem(&r.old_name).to_string())
        .collect();

    let mut files = Vec::new();
    collect_md_files(grove_root, &mut files)?;
    files.sort();

    for path in &files {
        let body = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for (idx, line) in body.lines().enumerate() {
            for token in &stale {
                if line.contains(token) {
                    writeln!(
                        out,
                        "{}:{}: {} ({})",
                        path.display(),
                        idx + 1,
                        token,
                        line.trim()
                    )
                    .ok();
                }
            }
        }
    }
    Ok(())
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

/// Resolve a path argument (absolute, or relative to the grove root) to an
/// existing absolute path under the grove root.
fn resolve_under_root(grove_abs: &Path, path: &Path) -> Result<PathBuf> {
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        grove_abs.join(path)
    };
    let abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving path {}", candidate.display()))?;
    if !abs.starts_with(grove_abs) {
        bail!(
            "path {} is not under grove root {}",
            abs.display(),
            grove_abs.display()
        );
    }
    Ok(abs)
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

/// Recursively collect every `.md` file path under `dir` (every directory level's
/// briefs and leaves) — the bodies `surface_cross_refs` scans.
fn collect_md_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let path = entry.path();
        match entry.file_type() {
            Ok(t) if t.is_dir() => collect_md_files(&path, out)?,
            Ok(t) if t.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") => {
                out.push(path)
            }
            _ => {}
        }
    }
    Ok(())
}

/// Rebuild a positioned entry with a new per-level position; key and slug (and, for
/// a node, its whole subtree) are invariant.
fn bumped(entry: &Entry, new_position: u32) -> Entry {
    match entry {
        Entry::Leaf {
            kind,
            slug,
            key,
            outcome,
            ..
        } => Entry::Leaf {
            position: new_position,
            kind: *kind,
            slug: slug.clone(),
            key: *key,
            outcome: *outcome,
        },
        Entry::Node { slug, key, .. } => Entry::Node {
            position: new_position,
            slug: slug.clone(),
            key: *key,
        },
        Entry::Brief => Entry::Brief, // unreachable: briefs are never renumbered
    }
}

/// The path's final component as an owned `String`.
fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_string())
        .with_context(|| format!("path {} has no filename", path.display()))
}

/// Drop a trailing `.md` from a name (a leaf's on-disk name) so a directory-token
/// reference matches; a node directory's name has no extension and is returned as-is.
fn stem(name: &str) -> &str {
    name.strip_suffix(".md").unwrap_or(name)
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
    fail_after_claim(path)?;
    file.write_all(task_template_body(slug, key).as_bytes())
        .with_context(|| format!("writing {}", path.display()))
}

/// Claim a destination and fill it — the whole of creating one leaf, for the
/// callers that create exactly one and so have nothing to unwind.
pub(crate) fn write_task_template(path: &Path, slug: &str, key: u32) -> Result<()> {
    let file = claim_destination(path)?;
    fill_task_file(file, path, slug, key)
}

/// The deterministic seam for the **created-but-not-filled** window.
///
/// Splitting creation from writing makes that window explicit, and what lives
/// in it is a real failure mode — `write_all` returning `ENOSPC` or `EIO` with
/// the file already on disk. Neither is portably reproducible, and an
/// unreachable arm is an unverified one, so the fill fails on demand instead:
/// a test arms one filename, and the run reaches its rollback with that leaf
/// created and empty. Compiled out of the shipped binary entirely.
#[cfg(test)]
fn fail_after_claim(path: &Path) -> Result<()> {
    let armed = FAIL_AFTER_CLAIM.with(|slot| slot.borrow().clone());
    if let Some(name) = armed {
        if path.file_name().and_then(|n| n.to_str()) == Some(name.as_str()) {
            bail!("injected failure after creating {}", path.display());
        }
    }
    Ok(())
}

#[cfg(not(test))]
fn fail_after_claim(_path: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
thread_local! {
    /// The filename [`fail_after_claim`] must fail on, if any. Thread-local so
    /// arming it in one test cannot reach a test running beside it.
    static FAIL_AFTER_CLAIM: std::cell::RefCell<Option<String>> =
        const { std::cell::RefCell::new(None) };
}

pub(crate) fn task_template_body(slug: &str, key: u32) -> String {
    format!("# {slug}-k{key}\n\n\n## Goal\n\n\n\n## Context\n\n## Done when\n\n## Notes\n")
}

fn refuse_finish_kind(kind: Kind, verb: &str) -> Result<()> {
    if kind == Kind::Finish {
        bail!("`finish` is driver-reserved and cannot be created by `{verb}`");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_access;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    // ---- guard-taking helpers ------------------------------------------------
    //
    // Each verb below composes the exclusive tree guard with its lock-neutral
    // twin, which is what a test needs to call one verb standalone. These were
    // `pub fn` on the module until `dead-non-launch-exports-k166`, and that was
    // the defect: **production calls none of them and cannot**, because
    // `llm_cli` has to resolve the `<parent>`/`<target>` reference *inside* the
    // same guard that then mutates, while these take an already-resolved path.
    // As module API they read as a second, blessed way to grow the tree that
    // locks a narrower window than the operation actually needs — and being
    // `pub` in a `pub` module, `dead_code` could never say they had no callers.
    // As test helpers they are what they always were: scaffolding for driving
    // one verb in isolation.

    fn leaf_add(grove_root: &Path, parent_dir: &Path, slug: &str, kind: Kind) -> Result<PathBuf> {
        refuse_finish_kind(kind, "leaf-add")?;
        let guard = tree_access::write(grove_root)?;
        leaf_add_unlocked(guard.root(), parent_dir, slug, kind)
    }

    fn leaf_add_pair(grove_root: &Path, parent_dir: &Path, stem: &str) -> Result<Vec<PathBuf>> {
        let guard = tree_access::write(grove_root)?;
        leaf_add_pair_unlocked(guard.root(), parent_dir, stem)
    }

    fn leaf_insert(
        grove_root: &Path,
        target: &Path,
        slug: &str,
        kind: Kind,
    ) -> Result<(PathBuf, Vec<Renumber>)> {
        refuse_finish_kind(kind, "leaf-insert")?;
        let guard = tree_access::write(grove_root)?;
        leaf_insert_unlocked(guard.root(), target, slug, kind)
    }

    fn surface_cross_refs(
        grove_root: &Path,
        renumbers: &[Renumber],
        out: &mut impl std::io::Write,
    ) -> Result<()> {
        let guard = tree_access::read(grove_root)?;
        surface_cross_refs_unlocked(guard.root(), renumbers, out)
    }

    /// A fresh `.grove/` directory (no git — the grow verbs only write files).
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        crate::tree_format::write_current_last(&root).unwrap();
        (tmp, root)
    }

    #[test]
    fn leaf_add_rejects_finish_before_inspecting_the_tree() {
        let (_tmp, grove_root) = grove();
        let missing_root = grove_root.join("missing");

        let error = leaf_add(&missing_root, &missing_root, "stop", Kind::Finish).unwrap_err();

        assert_eq!(
            error.to_string(),
            "`finish` is driver-reserved and cannot be created by `leaf-add`"
        );
    }

    #[test]
    fn leaf_insert_rejects_finish_before_inspecting_the_tree() {
        let (_tmp, grove_root) = grove();
        let missing_root = grove_root.join("missing");

        let error = leaf_insert(
            &missing_root,
            &missing_root.join("01-impl-existing-k1.md"),
            "stop",
            Kind::Finish,
        )
        .unwrap_err();

        assert_eq!(
            error.to_string(),
            "`finish` is driver-reserved and cannot be created by `leaf-insert`"
        );
    }

    /// A `.grove/` inside a real git repo. Entries rename whether or not git is
    /// tracking them ([`crate::tree_rename`]); call [`stage_all`] when a test wants
    /// the *tracked* branch (the rename moves git's index entry too).
    fn git_grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_git(&repo, &["init", "-q"]);
        run_git(&repo, &["config", "user.email", "t@example.com"]);
        run_git(&repo, &["config", "user.name", "Test"]);
        let root = repo.join(".grove");
        fs::create_dir_all(&root).unwrap();
        crate::tree_format::write_current_last(&root).unwrap();
        (tmp, root)
    }

    fn run_git(repo: &Path, args: &[&str]) {
        let out = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(args)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    /// Stage everything under the grove, putting the entries in git's index — the
    /// state in which a rename goes through `git mv` and carries the index along.
    fn stage_all(root: &Path) {
        run_git(root.parent().unwrap(), &["add", "-A"]);
    }

    /// Write a leaf/brief stub file with a position-free `# <handle>` header.
    fn touch(dir: &Path, name: &str, header: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, format!("# {header}\n")).unwrap();
        p
    }

    /// Write a file with an explicit body (for realistic multi-line content).
    fn touch_body(dir: &Path, name: &str, content: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    /// Create a node directory with its `BRIEF.md`, returning the directory path.
    fn mknode(dir: &Path, name: &str, handle: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(&p).unwrap();
        fs::write(p.join("BRIEF.md"), format!("# {handle} — brief\n")).unwrap();
        p
    }

    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    fn names_of(paths: &[PathBuf]) -> Vec<String> {
        paths.iter().map(|p| name_of(p)).collect()
    }

    fn body(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// The directory's child names (files and subdirs), lexically sorted.
    fn list(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .filter(|name| name != "FORMAT")
            .collect();
        v.sort();
        v
    }

    /// The names git has **in its index** for the grove, lexically sorted. Distinct
    /// from [`list`] (what is on disk): a rename that went through `git mv` moves the
    /// index entry and shows here under the new name; a plain rename of an untracked
    /// file shows in neither index state.
    fn indexed(root: &Path) -> Vec<String> {
        let out = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["ls-files", "--", "."])
            .output()
            .unwrap();
        let mut v: Vec<String> = String::from_utf8_lossy(&out.stdout)
            .lines()
            .map(|s| s.to_string())
            .filter(|name| !name.ends_with("/FORMAT") && name != "FORMAT")
            .collect();
        v.sort();
        v
    }

    // ---- leaf-add -----------------------------------------------------------

    #[test]
    fn add_root_level_child_gets_position_01_and_first_key() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "survey", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "01-impl-survey-k1.md");
    }

    #[test]
    fn add_appends_gapless_after_existing_root_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        touch(&g, "02-impl-b-k2.md", "b-k2");
        let got = leaf_add(&g, &g, "c", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "03-impl-c-k3.md");
    }

    #[test]
    fn add_child_under_a_node_appends_after_existing_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "02-impl-y-k4.md");
        assert_eq!(name_of(got.parent().unwrap()), "02-build-k2");
    }

    #[test]
    fn add_first_child_under_a_childless_node() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        let got = leaf_add(&g, &node, "first", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "01-impl-first-k3.md");
    }

    #[test]
    fn add_assigns_fresh_key_as_max_over_whole_tree_plus_one() {
        // Keys are global, not per-node: the new key is max(key) + 1 across the
        // whole tree, including a deeper subtree's higher key.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "01-impl-deep-k7.md", "deep-k7"); // a high key in another subtree
        let build = mknode(&g, "02-build-k2", "build-k2");
        let got = leaf_add(&g, &build, "y", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "01-impl-y-k8.md");
    }

    #[test]
    fn add_counts_done_children_so_a_retired_slot_is_never_reused() {
        // A `DONE` child still occupies its position — the next child is 02, not 01.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-impl-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "02-impl-y-k4.md");
    }

    #[test]
    fn add_counts_abandoned_children_so_a_pruned_slot_is_never_reused() {
        // Symmetric with DONE: an `ABANDONED` child still occupies its position
        // and its key (pruning's key-reuse defect) — the next child is 02
        // and its key is 4, not a reused 3.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-ABANDONED-impl-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "02-impl-y-k4.md");
    }

    #[test]
    fn add_counts_node_dir_siblings_when_numbering() {
        // A sibling that is a node *directory* still occupies its position.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        mknode(&g, "01-design-k1", "design-k1");
        let got = leaf_add(&g, &g, "build", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "02-impl-build-k2.md");
    }

    #[test]
    fn add_writes_kind_in_filename_and_not_in_body() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "survey", Kind::Impl).unwrap();
        let text = body(&got);
        assert!(
            text.starts_with("# survey-k1\n"),
            "header is the position-free handle; got {text:?}"
        );
        assert_eq!(name_of(&got), "01-impl-survey-k1.md");
        assert!(!text.contains("**Kind:**"), "got {text:?}");
        assert!(!text.contains("**Harness:**"), "got {text:?}");
    }

    #[test]
    fn add_planning_kind_writes_planning_filename() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "design", Kind::Planning).unwrap();
        assert_eq!(name_of(&got), "01-planning-design-k1.md");
        assert!(!body(&got).contains("**Kind:**"));
    }

    #[test]
    fn add_errors_when_parent_node_is_missing() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let missing = g.join("09-nope-k9");
        let err = leaf_add(&g, &missing, "y", Kind::Impl).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_when_parent_is_a_leaf_file_not_a_node() {
        // A leaf is a *file* — you must decompose it into a node before adding under it.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = touch(&g, "02-impl-build-k2.md", "build-k2");
        let err = leaf_add(&g, &leaf, "y", Kind::Impl).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn a_review_chain_is_cut_one_flat_sibling_at_a_time() {
        // The whole lazy shape, end to end, through the verb that builds it.
        // The producer's last act appends its review; the review's last act
        // appends the integration. Each is an ordinary `leaf-add` at the
        // parent's next free position, so the three land contiguously as flat
        // siblings — no node directory, no constructor, and nothing in `tree_*`
        // that knows the three compose one artifact.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");

        let producer = leaf_add(&g, &g, "sync", Kind::Design).unwrap();
        let review = leaf_add(&g, &g, "sync-review", Kind::ReviewDesign).unwrap();
        let integrate = leaf_add(&g, &g, "sync-integrate", Kind::IntegrateReviewDesign).unwrap();

        assert_eq!(
            names_of(&[producer, review, integrate]),
            vec![
                "01-design-sync-k1.md",
                "02-review-design-sync-review-k2.md",
                "03-integrate-review-design-sync-integrate-k3.md",
            ],
            "contiguous flat siblings off one stem"
        );
        assert_eq!(
            list(&g),
            vec![
                "01-design-sync-k1.md",
                "02-review-design-sync-review-k2.md",
                "03-integrate-review-design-sync-integrate-k3.md",
                "BRIEF.md",
            ],
            "and nothing else — no node directory was created for them"
        );
    }

    #[test]
    fn a_review_step_cut_after_unrelated_work_still_appends_at_the_end() {
        // The cost the flat shape accepts, stated rather than defended against
        // (flat-lazy-review, *Known consequence, accepted*). A review decided on
        // once a later leaf already exists lands **after** that leaf, not beside
        // its producer — grove validates no cross-leaf grammar and contiguity
        // was always a convention; nothing here enforces one.
        //
        // For *this* hop the gap is also harmless, which is why `leaf-add` is
        // the right verb: the review re-derives everything from the producer's
        // commit and computes its own citations against the tree it finds. The
        // integrate hop is the one that cares — see the test below.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        leaf_add(&g, &g, "sync", Kind::Design).unwrap();
        leaf_add(&g, &g, "unrelated", Kind::Impl).unwrap();

        let review = leaf_add(&g, &g, "sync-review", Kind::ReviewDesign).unwrap();

        assert_eq!(name_of(&review), "03-review-design-sync-review-k3.md");
    }

    #[test]
    fn an_integration_cut_with_insert_lands_beside_the_review_it_integrates() {
        // The shape the methodology asks a review session to produce when the
        // review still has a live sibling after it. Nothing in `tree_*` knows
        // that an `integrate-review-*` leaf belongs next to its review — the
        // rule is guidance, and this pins what obeying it looks like, because
        // the alternative (`leaf-add`, which would put the integration at 04)
        // is equally well-formed and equally accepted here.
        //
        // Why it matters is not visible to any verb: the integration resolves
        // `path:line` citations its review already froze, so an unrelated leaf
        // running first can move them without erroring.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        leaf_add(&g, &g, "sync", Kind::Design).unwrap();
        leaf_add(&g, &g, "sync-review", Kind::ReviewDesign).unwrap();
        let unrelated = leaf_add(&g, &g, "unrelated", Kind::Impl).unwrap();

        let (integrate, renumbered) = leaf_insert(
            &g,
            &unrelated,
            "sync-integrate",
            Kind::IntegrateReviewDesign,
        )
        .unwrap();

        assert_eq!(
            name_of(&integrate),
            "03-integrate-review-design-sync-integrate-k4.md",
            "the integration takes the slot after its review"
        );
        assert_eq!(
            list(&g),
            vec![
                "01-design-sync-k1.md",
                "02-review-design-sync-review-k2.md",
                "03-integrate-review-design-sync-integrate-k4.md",
                "04-impl-unrelated-k3.md",
                "BRIEF.md",
            ],
            "and the unrelated leaf shifts down, keeping its own key"
        );
        assert_eq!(renumbered.len(), 1, "only the displaced sibling moved");
    }

    #[test]
    fn add_errors_when_parent_dir_name_is_not_a_node() {
        // Node-ness is the *name* plus being a directory, and nothing else — a
        // `BRIEF.md` is not part of the test. There is one node species now
        // (flat-lazy-review took the chain node with it), and a charterless one
        // is a hand-authored lapse the reader tolerates rather than a second
        // kind of node; either way the guard reads the name, so a bare `notes/`
        // is still refused.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let bare = g.join("notes");
        fs::create_dir_all(&bare).unwrap();
        let err = leaf_add(&g, &bare, "y", Kind::Impl).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_on_invalid_slug() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add(&g, &g, "BRIEF", Kind::Impl).is_err());
        assert!(leaf_add(&g, &g, "Bad Slug", Kind::Impl).is_err());
    }

    #[test]
    fn add_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = leaf_add(&missing, &missing, "y", Kind::Impl).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-add-pair ------------------------------------------------------
    //
    // The one surviving composite verb. Two properties carry it and each is
    // pinned by *mutation*, not by asserting the happy path made three things:
    //
    //   * **the steps are flat siblings at consecutive positions**, off one
    //     stem, with the pair's three fixed kinds (flat-lazy-review);
    //   * **one call, one mutation** — a run that fails leaves *no leaf at all*,
    //     so a live prefix of a pair never masquerades as a hand-cut partial
    //     one. That is what the pair still buys over three `leaf-add`s, and it
    //     is why the pair alone stayed eager: its two producers must not see
    //     each other's framing (`leaf_add_pair_unlocked`).

    #[test]
    fn pair_emits_three_flat_siblings_with_the_fixed_research_kinds() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let paths = leaf_add_pair(&g, &g, "sync-survey").unwrap();
        assert_eq!(
            names_of(&paths),
            vec![
                "01-research-a-sync-survey-a-k1.md",
                "02-research-b-sync-survey-b-k2.md",
                "03-combine-research-sync-survey-combine-k3.md",
            ],
            "three siblings at consecutive positions, three consecutive keys"
        );
        for step in &paths {
            // Compared by name, not by path: the verb canonicalises the grove
            // root, and on macOS that rewrites `/var` to `/private/var`.
            assert_eq!(
                name_of(step.parent().unwrap()),
                ".grove",
                "every step is a direct child of the parent — no node directory"
            );
        }
        assert_eq!(
            list(&g),
            vec![
                "01-research-a-sync-survey-a-k1.md",
                "02-research-b-sync-survey-b-k2.md",
                "03-combine-research-sync-survey-combine-k3.md",
                "BRIEF.md",
            ],
            "the run created its three leaves and nothing else"
        );
    }

    #[test]
    fn pair_appends_after_existing_siblings_and_under_a_node() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-impl-x-k3.md", "x-k3");
        let paths = leaf_add_pair(&g, &node, "api").unwrap();
        assert_eq!(
            names_of(&paths),
            vec![
                "02-research-a-api-a-k4.md",
                "03-research-b-api-b-k5.md",
                "04-combine-research-api-combine-k6.md",
            ],
            "the steps continue the parent's positions; the keys continue the whole tree"
        );
        assert_eq!(name_of(paths[0].parent().unwrap()), "02-build-k2");
    }

    #[test]
    fn pair_rejects_a_malformed_stem_in_its_own_right() {
        // A trailing dash is a bad slug, but `foo--a` is not, so validating only
        // the *derived* names would let a malformed stem through on all three.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add_pair(&g, &g, "foo-").is_err());
        assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
    }

    // ---- one call, one mutation --------------------------------------------

    #[test]
    fn a_run_that_cannot_read_its_parent_level_creates_nothing_at_all() {
        // The run's single point of refusal, reached before allocation: a
        // task-shaped entry whose on-disk species contradicts its name is a
        // malformed tree the reader refuses (`tree_read::read_level`), and the
        // pair's very first act is to read the parent level for its next free
        // position. A squatter that is *not* malformed is no obstruction at all
        // — numbering simply allocates past it — which is why this is the
        // reachable arm and the destination sweep in `add_run` is a guard
        // against a writer that never took the tree lock.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        fs::create_dir(g.join("01-research-a-survey-a-k1.md")).unwrap();

        let err = leaf_add_pair(&g, &g, "survey").unwrap_err().to_string();

        assert!(
            err.contains("01-research-a-survey-a-k1.md"),
            "the error names the entry standing in the way: {err}"
        );
        let mut files = list(&g);
        files.sort();
        assert_eq!(
            files,
            vec!["01-research-a-survey-a-k1.md", "BRIEF.md"],
            "only the squatter and the brief — no half-built pair left behind"
        );
    }

    #[test]
    fn an_unwritable_third_destination_refuses_the_whole_run_up_front() {
        // A hazard **specific to composite verbs**: the derived names are longer
        // than the stem the caller validated, and unequally so, so a third name
        // can cross `NAME_MAX` (255) while the first two clear it.
        //
        // This used to be the mid-write arm, because `Path::exists` reports
        // false for an over-long path and the sweep waved it through. It is now
        // an *up-front refusal*: `symlink_metadata` returns `ENAMETOOLONG`
        // rather than `NotFound`, and only `NotFound` means free — so "I could
        // not tell" stops the run before the first leaf instead of after the
        // second. The strictly better outcome of the same hazard; the mid-write
        // rollback arm is pinned deterministically by the armed seam above.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        // `NN-research-a-<stem>-a-k<key>.md` is stem+22 at a single-digit key;
        // `NN-combine-research-<stem>-combine-k<key>.md` is stem+34. At 233 the
        // run plans two 255-byte names and one 267-byte name: the first two fit,
        // the third does not.
        let stem = "a".repeat(233);

        let err = leaf_add_pair(&g, &g, &stem).unwrap_err().to_string();

        assert!(
            err.contains("nothing was created"),
            "the refusal says what the tree now holds: {err}"
        );
        assert_eq!(
            list(&g),
            vec!["BRIEF.md"],
            "not even the two leaves whose names would have fit"
        );
    }

    #[test]
    fn a_run_that_cannot_get_three_fresh_keys_creates_nothing_at_all() {
        // Key exhaustion is a *resolution* failure, so it belongs beside slug
        // validation and the destination sweep, before the first write —
        // deriving the keys per step from the first would be unchecked
        // arithmetic after a leaf had already landed.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-old-k4294967294.md", "old-k4294967294");

        let err = leaf_add_pair(&g, &g, "survey").unwrap_err().to_string();

        assert!(
            err.contains("nothing was created"),
            "the refusal says what the tree now holds: {err}"
        );
        assert_eq!(
            list(&g),
            vec!["01-impl-old-k4294967294.md", "BRIEF.md"],
            "not even the first leaf"
        );
    }

    #[test]
    fn a_fill_that_fails_after_its_claim_unwinds_the_leaf_it_had_just_created() {
        // The window the split between claim and fill makes explicit, and the
        // one `fs::write` hid: the file lands, the bytes do not. `write_all`
        // returning `ENOSPC`/`EIO` after a successful create is the real form of
        // this, and it is not portably reproducible — hence the armed seam.
        //
        // What is asserted is that the **failing** path is rolled back too, not
        // just the leaves before it. Recording ownership after the fill instead
        // of after the claim would leave exactly one empty, well-formed
        // `NN-combine-research-…` behind: the residue that reads as a
        // deliberately cut partial pair.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        FAIL_AFTER_CLAIM.with(|slot| {
            *slot.borrow_mut() = Some("03-combine-research-survey-combine-k3.md".to_string())
        });

        let err = leaf_add_pair(&g, &g, "survey").unwrap_err().to_string();

        FAIL_AFTER_CLAIM.with(|slot| *slot.borrow_mut() = None);
        assert!(
            err.contains("rolled back"),
            "the error says the run was undone: {err}"
        );
        assert_eq!(
            list(&g),
            vec!["BRIEF.md"],
            "the created-but-unwritten leaf must not survive either"
        );
    }

    #[test]
    fn a_dangling_symlink_occupies_a_destination_rather_than_reading_as_absent() {
        // `Path::exists()` follows symlinks and so reports a dangling one as
        // *absent* — the destination reads free, and a following `fs::write`
        // creates the link's target, which may be anywhere on disk and is here
        // deliberately outside `.grove/`. A rollback would then remove the link
        // and leave that target standing.
        //
        // Asserted against the guard directly rather than through `leaf-add`,
        // because a symlink already sitting at a *task-shaped* name never
        // reaches it: `read_level` refuses that whole level as a malformed tree
        // first. The guard's caller is the plant that arrives *after* the level
        // was read, which no test can stage — so what is pinned here is the
        // guard's own answer, and `claim_destination` below pins the write's.
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
        // The guarantee the up-front sweep cannot give. A sweep is a check at
        // one instant and the write happens at another, so a writer that ignored
        // the tree lock can occupy a planned destination in between; what makes
        // that harmless is that the write itself refuses. Pinned against every
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

    #[test]
    fn a_rolled_back_run_leaves_the_next_call_a_clean_slate() {
        // Why all-or-nothing is worth the machinery: a retry after a failure
        // must produce the shape, not a second copy of it under new positions
        // and keys. Pinned end to end — fail, clear the obstruction, retry.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let squatter = g.join("01-research-a-survey-a-k1.md");
        fs::create_dir(&squatter).unwrap();
        assert!(leaf_add_pair(&g, &g, "survey").is_err());
        fs::remove_dir(&squatter).unwrap();

        let paths = leaf_add_pair(&g, &g, "survey").unwrap();

        assert_eq!(
            names_of(&paths),
            vec![
                "01-research-a-survey-a-k1.md",
                "02-research-b-survey-b-k2.md",
                "03-combine-research-survey-combine-k3.md",
            ],
            "the retry got the positions and keys the failed run had planned"
        );
        assert_eq!(
            list(&g).len(),
            4,
            "exactly one pair plus the brief — no duplicate from the failed attempt"
        );
    }

    #[test]
    fn a_refused_run_does_not_consume_positions_or_keys() {
        // The allocation happens after validation, so a refusal costs nothing:
        // an ordinary `leaf-add` afterwards still gets 01/k1.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add_pair(&g, &g, "foo-").is_err());
        let got = leaf_add(&g, &g, "plain", Kind::Impl).unwrap();
        assert_eq!(name_of(&got), "01-impl-plain-k1.md");
    }

    #[test]
    fn a_shape_is_byte_identical_to_the_same_leaves_cut_by_hand() {
        // Constraint 6, and the claim the spec makes: a generated pair is the
        // same standard markdown/filesystem shape a human can cut and annotate.
        // With the template carrying no relationship lines and no rendered goal,
        // the equivalence is now total — the verb's only remaining contribution
        // is that the three land or none does.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let generated = leaf_add_pair(&g, &g, "survey").unwrap();
        let generated_bodies: Vec<String> = generated.iter().map(|p| body(p)).collect();
        for path in &generated {
            fs::remove_file(path).unwrap();
        }

        let by_hand = [
            leaf_add(&g, &g, "survey-a", Kind::ResearchA).unwrap(),
            leaf_add(&g, &g, "survey-b", Kind::ResearchB).unwrap(),
            leaf_add(&g, &g, "survey-combine", Kind::CombineResearch).unwrap(),
        ];

        assert_eq!(names_of(&generated), names_of(&by_hand));
        let hand_bodies: Vec<String> = by_hand.iter().map(|p| body(p)).collect();
        assert_eq!(generated_bodies, hand_bodies);
    }

    // ---- leaf-insert --------------------------------------------------------

    #[test]
    fn insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        touch(&g, "02-impl-b-k2.md", "b-k2");
        touch(&g, "03-impl-c-k3.md", "c-k3");
        stage_all(&g);
        let (path, _renums) =
            leaf_insert(&g, &g.join("02-impl-b-k2.md"), "new", Kind::Impl).unwrap();
        assert_eq!(name_of(&path), "02-impl-new-k4.md"); // fresh key, not a reused one
        let files = list(&g);
        assert!(
            files.contains(&"01-impl-a-k1.md".to_string()),
            "pos 1 < 2, unchanged"
        );
        assert!(
            files.contains(&"02-impl-new-k4.md".to_string()),
            "the inserted leaf"
        );
        assert!(
            files.contains(&"03-impl-b-k2.md".to_string()),
            "b: 02->03, key 2 kept"
        );
        assert!(
            files.contains(&"04-impl-c-k3.md".to_string()),
            "c: 03->04, key 3 kept"
        );
        assert!(
            !files.contains(&"02-impl-b-k2.md".to_string()),
            "old name gone"
        );
    }

    #[test]
    fn insert_cascades_a_sibling_node_subtree_riding_along_byte_identical() {
        // The headline (and the leaf's Notes requirement): inserting ahead of a
        // sibling *node* shifts only that node's own dir name — its BRIEF.md and
        // every grandchild stay byte-identical, the cascade collapse.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let mid = mknode(&g, "02-mid-k3", "mid-k3");
        let grandchild = touch_body(
            &mid,
            "01-impl-x-k4.md",
            "# x-k4\n\n**Kind:** impl\n\n## Goal\nstuff\n",
        );
        let grandchild_before = body(&grandchild);
        let brief_before = body(&mid.join("BRIEF.md"));
        stage_all(&g);
        let (path, renums) = leaf_insert(&g, &g.join("02-mid-k3"), "new", Kind::Impl).unwrap();
        assert_eq!(name_of(&path), "02-impl-new-k5.md");
        // The node shifted 02 -> 03; only its ancestor dir name changed.
        let shifted = g.join("03-mid-k3");
        assert!(shifted.is_dir(), "node dir shifted to 03-mid-k3");
        assert!(!g.join("02-mid-k3").exists(), "old node dir name gone");
        assert_eq!(
            body(&shifted.join("BRIEF.md")),
            brief_before,
            "the moved node's BRIEF.md is byte-identical"
        );
        assert_eq!(
            body(&shifted.join("01-impl-x-k4.md")),
            grandchild_before,
            "the grandchild is byte-identical — only the ancestor dir name moved"
        );
        assert_eq!(renums.len(), 1, "exactly one sibling (the node) shifted");
        assert_eq!(renums[0].old_position, 2);
        assert_eq!(renums[0].new_position, 3);
    }

    #[test]
    fn insert_writes_position_free_header_for_the_new_leaf() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        stage_all(&g);
        let (path, _r) = leaf_insert(&g, &g.join("01-impl-a-k1.md"), "head", Kind::Impl).unwrap();
        let text = body(&path);
        assert!(text.starts_with("# head-k2\n"), "got {text:?}");
        assert!(!text.contains("**Kind:**"), "got {text:?}");
    }

    #[test]
    fn insert_does_not_rewrite_any_existing_file_contents() {
        // Position-free headers ⇒ a shifted leaf's body is byte-identical too.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let b = touch_body(&g, "02-impl-b-k2.md", "# b-k2\n\nbody text\n");
        let b_before = body(&b);
        stage_all(&g);
        leaf_insert(&g, &g.join("02-impl-b-k2.md"), "new", Kind::Impl).unwrap();
        assert_eq!(
            body(&g.join("03-impl-b-k2.md")),
            b_before,
            "shifted leaf content byte-identical; only NN in the filename changed"
        );
    }

    #[test]
    fn insert_collision_free_for_a_dense_run_of_siblings() {
        // Stress the highest-first ordering: insert at the head of five siblings;
        // a wrong order would make a rename collide and lose a file.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        for i in 1..=5 {
            touch(
                &g,
                &format!("{i:02}-impl-s{i}-k{i}.md"),
                &format!("s{i}-k{i}"),
            );
        }
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &g.join("01-impl-s1-k1.md"), "head", Kind::Impl).unwrap();
        assert_eq!(name_of(&path), "01-impl-head-k6.md");
        assert_eq!(renums.len(), 5);
        let leaves: Vec<String> = list(&g).into_iter().filter(|n| n != "BRIEF.md").collect();
        assert_eq!(
            leaves,
            vec![
                "01-impl-head-k6.md",
                "02-impl-s1-k1.md",
                "03-impl-s2-k2.md",
                "04-impl-s3-k3.md",
                "05-impl-s4-k4.md",
                "06-impl-s5-k5.md",
            ],
            "all six leaves present, gapless 01..06 — no file lost to a collision"
        );
    }

    #[test]
    fn insert_returns_renumber_log_ascending_by_new_position() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        touch(&g, "02-impl-b-k2.md", "b-k2");
        touch(&g, "03-impl-c-k3.md", "c-k3");
        stage_all(&g);
        let (_path, renums) =
            leaf_insert(&g, &g.join("01-impl-a-k1.md"), "head", Kind::Impl).unwrap();
        let positions: Vec<(u32, u32)> = renums
            .iter()
            .map(|r| (r.old_position, r.new_position))
            .collect();
        assert_eq!(positions, vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn insert_inside_a_nested_node_shifts_only_that_levels_siblings() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "01-impl-a-k2.md", "a-k2");
        touch(&design, "02-impl-b-k3.md", "b-k3");
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &design.join("01-impl-a-k2.md"), "first", Kind::Impl).unwrap();
        assert_eq!(name_of(&path), "01-impl-first-k4.md");
        assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
        let children = list(&design);
        assert!(children.contains(&"01-impl-first-k4.md".to_string()));
        assert!(children.contains(&"02-impl-a-k2.md".to_string()));
        assert!(children.contains(&"03-impl-b-k3.md".to_string()));
        assert_eq!(renums.len(), 2);
    }

    #[test]
    fn insert_errors_on_invalid_slug() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        stage_all(&g);
        assert!(leaf_insert(&g, &g.join("01-impl-a-k1.md"), "BRIEF", Kind::Impl).is_err());
    }

    #[test]
    fn insert_errors_when_target_is_a_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        let err = leaf_insert(&g, &g.join("BRIEF.md"), "x", Kind::Impl).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn insert_errors_when_target_missing() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        assert!(leaf_insert(&g, &g.join("09-impl-nope-k9.md"), "x", Kind::Impl).is_err());
    }

    #[test]
    fn insert_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err =
            leaf_insert(&missing, &missing.join("01-impl-a-k1.md"), "x", Kind::Impl).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- insert over untracked entries (issue #3) ----------------------------
    //
    // The grow verbs are working-tree-only by design — `leaf_add_unlocked` writes an
    // *untracked* file and the enclosing task's commit folds it in. So the
    // ordinary rhythm of a planning session (grow several leaves, then realise
    // one must sequence earlier) hands `leaf_insert_unlocked` siblings that are not in
    // git's index. Renaming those is `fs::rename`'s job, not `git mv`'s.

    #[test]
    fn insert_ahead_of_an_untracked_sibling_added_this_session() {
        // Issue #3 verbatim: `leaf_add_unlocked` then `leaf_insert_unlocked` ahead of it,
        // with no `git add` in between — the ordinary planning-session sequence.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let release = leaf_add(&g, &g, "release", Kind::Impl).unwrap();
        assert_eq!(name_of(&release), "01-impl-release-k1.md");

        // No stage_all: the leaf is untracked, exactly as the grow verb left it.
        let (path, renums) = leaf_insert(&g, &release, "review", Kind::Impl).unwrap();

        assert_eq!(name_of(&path), "01-impl-review-k2.md");
        let files = list(&g);
        assert!(
            files.contains(&"02-impl-release-k1.md".to_string()),
            "the untracked sibling shifted 01->02, key preserved (files: {files:?})"
        );
        assert!(
            !files.contains(&"01-impl-release-k1.md".to_string()),
            "old name gone (files: {files:?})"
        );
        assert_eq!(renums.len(), 1, "one sibling shifted");
    }

    #[test]
    fn insert_renumbers_a_mix_of_tracked_and_untracked_siblings() {
        // The realistic mid-session tree: some leaves committed by earlier tasks,
        // some grown just now. One insert must renumber straight through both.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        touch(&g, "02-impl-b-k2.md", "b-k2");
        stage_all(&g); // a and b are tracked
        touch(&g, "03-impl-c-k3.md", "c-k3"); // c is not

        let (path, renums) =
            leaf_insert(&g, &g.join("01-impl-a-k1.md"), "new", Kind::Impl).unwrap();

        assert_eq!(name_of(&path), "01-impl-new-k4.md");
        let files = list(&g);
        for expected in ["02-impl-a-k1.md", "03-impl-b-k2.md", "04-impl-c-k3.md"] {
            assert!(
                files.contains(&expected.to_string()),
                "every sibling shifted up one, tracked or not: missing {expected} (files: {files:?})"
            );
        }
        assert_eq!(renums.len(), 3, "all three siblings shifted");
    }

    #[test]
    fn insert_moves_the_index_entry_for_a_tracked_sibling() {
        // The other half of the contract: a *tracked* entry still moves through
        // `git mv`, so the rename is staged and the operator's `git status` shows a
        // rename rather than a delete + an untracked file. Guards against
        // "simplify" ing the primitive down to a bare `fs::rename` everywhere.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl-a-k1.md", "a-k1");
        stage_all(&g);

        leaf_insert(&g, &g.join("01-impl-a-k1.md"), "new", Kind::Impl).unwrap();

        let idx = indexed(&g);
        assert!(
            idx.contains(&"02-impl-a-k1.md".to_string()),
            "the tracked sibling's index entry moved to the new name (index: {idx:?})"
        );
        assert!(
            !idx.contains(&"01-impl-a-k1.md".to_string()),
            "the old index entry is gone — a staged rename, not a stale entry (index: {idx:?})"
        );
    }

    // ---- surface_cross_refs (position-prefixed lint, not auto-rewrite) -------

    fn renum(old: u32, new: u32, old_name: &str, new_name: &str) -> Renumber {
        Renumber {
            old_position: old,
            new_position: new,
            old_name: old_name.to_string(),
            new_name: new_name.to_string(),
        }
    }
    fn surfaced(root: &Path, renumbers: &[Renumber]) -> String {
        let mut buf = Vec::new();
        surface_cross_refs(root, renumbers, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn surface_empty_renumbers_emits_nothing() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "01-impl-a-k1.md",
            "# a-k1\n\nrefers to 02-mid-k3 somewhere\n",
        );
        assert_eq!(surfaced(&g, &[]), "");
    }

    #[test]
    fn surface_reports_a_stale_position_prefixed_reference_in_a_body() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "01-impl-a-k1.md",
            "# a-k1\n\nthe design lives at 02-mid-k3/01-impl-x-k4.md\n",
        );
        let out = surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]);
        assert!(out.contains("01-impl-a-k1.md"), "names the file: {out:?}");
        assert!(out.contains("02-mid-k3"), "shows the stale name: {out:?}");
        assert!(
            out.contains(":3:"),
            "1-based line number of the body ref: {out:?}"
        );
    }

    #[test]
    fn surface_does_not_flag_the_stable_slug_key_handle() {
        // A `<slug>-k<key>` reference is stable across a renumber — never surfaced.
        // Only the position-prefixed `02-mid-k3` form is stale.
        let (_t, g) = grove();
        touch_body(
            &g,
            "01-impl-a-k1.md",
            "# a-k1\n\nsee mid-k3 for the design\n",
        );
        assert_eq!(surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]), "");
    }

    #[test]
    fn surface_reports_hits_recursively_across_nested_files() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "BRIEF.md",
            "# root — brief\n\nthe plan is at 02-mid-k3\n",
        );
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch_body(
            &design,
            "01-impl-a-k2.md",
            "# a-k2\n\nalso 02-mid-k3 here\n",
        );
        let out = surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]);
        assert!(
            out.contains("BRIEF.md") && out.contains("02-mid-k3"),
            "{out:?}"
        );
        assert!(
            out.contains("01-impl-a-k2.md"),
            "nested file surfaced: {out:?}"
        );
    }
}

// The **v2 grow verbs** (task-tree-scheme) — `leaf-add`, `leaf-insert`, and the
// composite `leaf-add-chain` / `leaf-add-pair` (which emit a **chain node**: a
// directory holding three steps and, by rule, no `BRIEF.md`) —
// re-expressed against the real **directory tree**, built on the 11.1 id model
// (`src/tree_id.rs`). Keeps task-tree-scheme's *semantics* (append a gapless child with a
// fresh permanent key; insert shifts the occupant + later siblings up by one) and
// changes the *mechanics* to the filesystem's shape:
//
//   * v1-flat baked the whole dotted path into every filename, so inserting at
//     `2.2` rewrote `2.2.1`→`2.3.1`… across the **whole subtree** — O(subtree)
//     filename + header rewrites.
//   * v2 carries the hierarchy in directories (a node is a *directory* holding
//     its children, and optionally a `BRIEF.md` charter — see `tree_id`'s header
//     for the two species), so a renumber is a single **rename of a directory**
//     and the subtree — child names *and* keys — rides along untouched. The shift
//     is O(siblings at one level), the "cascade collapse" task-tree-scheme celebrates.
//
// **Position-free headers (decided 11.3, with the post-flip grilling).** A task
// file's first-line `# …` header is the *stable handle* `# <slug>-k<key>` — the
// per-level position `NN` lives only in the filename, never in the body. This is
// the faithful realization of task-tree-scheme §5 ("reference a work item by `<slug>-k<key>`,
// never by its position/path") and it makes the renumber a **pure rename with
// zero content rewrites**: shifting `05-mid-k14/`→`06-mid-k14/` changes one
// directory name and nothing else — the moved node's own `BRIEF.md` header
// (`# mid-k14 — brief`) and every descendant file stay byte-identical. (Carried to
// 11.4's migration, which rewrites v1 `# <dotted>-[<key>]-<slug>` headers down to
// the position-free handle, and to 11.5's prose.)
//
// Built **isolated**, mirroring `tree_id` / `tree_read`: this module does NOT
// touch the live v1 verb path (`leaf_grow` and the `llm_cli` dispatch to it),
// which keeps speaking the flat scheme until the user-gated re-flip (11.6). So no
// leaf in this node can break the v1-flat grove that is driving itself.

use crate::harness::Harness;
use crate::leaf::Kind;
use crate::tree_id::{next_key, next_keys, parse, validate_slug, Entry, Outcome};
use crate::tree_rename::rename_entry;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Append a child leaf under the node directory `parent_dir` at the next gapless
/// per-level position, with a fresh permanent key. `parent_dir` is the grove root
/// (a root-level leaf) or a node directory under it (absolute, or relative to the
/// grove root). The new position is `max(direct-child position) + 1` (counting
/// leaves live AND `DONE`, and node directories, so a slot is never reused), the
/// new key is `max(key over the whole tree) + 1`. Working-tree only — no commit.
///
/// `harness` is the optional per-leaf routing declaration (`leaf-harness-k15`):
/// `Some` writes a `**Harness:**` line beside `**Kind:**`, `None` — the
/// overwhelmingly common case — writes no line at all.
pub fn leaf_add(
    grove_root: &Path,
    parent_dir: &Path,
    slug: &str,
    kind: Kind,
    harness: Option<&'static Harness>,
) -> Result<PathBuf> {
    validate_slug(slug)?;
    let grove_abs = canonical_grove_root(grove_root)?;
    let parent_abs = resolve_parent_node(&grove_abs, parent_dir)?;

    let position = next_child_position(&parent_abs)?;
    let key = next_key(collect_all_names(&grove_abs)?)?;
    let entry = Entry::Leaf {
        position,
        slug: slug.to_string(),
        key,
        outcome: Outcome::Live,
    };
    let path = parent_abs.join(entry.name());
    if path.exists() {
        bail!("destination already exists: {}", path.display());
    }
    write_template(&path, slug, key, kind, harness)?;
    Ok(path)
}

/// One leaf of a composite shape: the slug the verb derived for it, its kind,
/// and its optional per-leaf harness declaration. Private — the shape, not the
/// step, is what a caller names ([`leaf_add_chain`], [`leaf_add_pair`]).
struct Step {
    slug: String,
    kind: Kind,
    harness: Option<&'static Harness>,
}

/// Append a whole **shape** — a **chain node** directory holding its steps as
/// children — under `parent_dir` as *one mutation*. The shared engine behind
/// [`leaf_add_chain`] and [`leaf_add_pair`]; the difference between them is
/// entirely `node_slug` and which `Step`s they derive.
///
/// **The node carries no `BRIEF.md`, and that is the point rather than an
/// omission** (task-tree-scheme, *the charter distinguishes the two species*). A
/// decomposition node's charter says *this proved bigger than one session*; a
/// chain has no such context to write, and a stub written because a step demanded
/// it is exactly what constraint 4 forbids. Its absence is also the **discriminator
/// the Retire cascade reads** — a file test, never a parse of the `-chain` /
/// `-pair` token, which is ordinary slug text nothing keys on.
///
/// **This is not `leaf_add` in a loop, and the difference is the whole point of
/// a composite verb.** `leaf_add` validates, allocates and writes in one breath,
/// so four calls give four chances to stop half-way — and a live prefix of a
/// chain looks exactly like a deliberately hand-cut partial one, which is the
/// wrong-but-well-formed residue the verb exists to prevent. So:
///
/// * **every slug is validated, and the parent resolved, before the first
///   write** — a bad third slug refuses without creating the node;
/// * **the position and keys come from one snapshot**, so the node lands at the
///   parent's next free slot and its keys run consecutively from the node's own;
/// * **the destination is checked free before anything is written**, which is
///   what makes the realistic collision an up-front refusal rather than a
///   rollback. One check now covers the whole shape: the children live inside a
///   directory that does not exist, so none of them can;
/// * **anything that still fails mid-write rolls the run back**, so the
///   observable outcome is the whole shape or none of it.
///
/// Working-tree only — no commit, exactly like [`leaf_add`].
fn add_run(
    grove_root: &Path,
    parent_dir: &Path,
    node_slug: &str,
    steps: &[Step],
) -> Result<Vec<PathBuf>> {
    validate_slug(node_slug)?;
    for step in steps {
        validate_slug(&step.slug)?;
    }
    let grove_abs = canonical_grove_root(grove_root)?;
    let parent_abs = resolve_parent_node(&grove_abs, parent_dir)?;

    // One snapshot for the whole run: the node takes the parent's next free slot
    // and the tree-wide next key; its children start at `01` inside it with the
    // keys after the node's. Re-reading per step would give the same answer today
    // — nothing else is writing — but deriving both once is what makes contiguous
    // positions and consecutive keys a property rather than a race.
    //
    // **The whole key run is allocated here, before the first write**, rather than
    // derived per step from the node's. Deriving them later is unchecked
    // arithmetic *after* `create_dir` — the one window where a failure can leave a
    // node directory behind, which is precisely what this verb exists to prevent.
    // `next_keys` refuses an exhausted keyspace, so that refusal lands here, with
    // the tree untouched.
    let node_position = next_child_position(&parent_abs)?;
    let step_count = u32::try_from(steps.len()).expect("a shape has a handful of steps");
    let mut keys = next_keys(collect_all_names(&grove_abs)?, 1 + step_count)
        .context("allocating the shape's keys (nothing was created)")?
        .into_iter();
    let node_key = keys
        .next()
        .expect("a run of 1 + steps keys always yields the node's");
    let node_dir = parent_abs.join(
        Entry::Node {
            position: node_position,
            slug: node_slug.to_string(),
            key: node_key,
        }
        .name(),
    );
    if node_dir.exists() {
        bail!(
            "destination already exists: {} (nothing was created)",
            node_dir.display()
        );
    }

    fs::create_dir(&node_dir).with_context(|| format!("creating {}", node_dir.display()))?;

    let mut created = vec![node_dir.clone()];
    for (i, (step, key)) in steps.iter().zip(keys).enumerate() {
        let entry = Entry::Leaf {
            position: i as u32 + 1,
            slug: step.slug.clone(),
            key,
            outcome: Outcome::Live,
        };
        let path = node_dir.join(entry.name());
        if let Err(e) = write_template(&path, &step.slug, key, step.kind, step.harness) {
            return Err(roll_back(e, &node_dir));
        }
        created.push(path);
    }
    Ok(created)
}

/// Undo a partially-written run and return the error the caller should see.
/// Everything the run created is inside `node_dir`, which was proven absent
/// moments earlier, so one recursive remove is both complete and safe by
/// construction — it can never eat a pre-existing entry. A removal that itself
/// fails is *named*: a residue the operator must know about is worse hidden than
/// reported.
fn roll_back(cause: anyhow::Error, node_dir: &Path) -> anyhow::Error {
    if fs::remove_dir_all(node_dir).is_ok() {
        cause
            .context("creating the shape failed; the node directory it had created was rolled back")
    } else {
        cause.context(format!(
            "creating the shape failed and rollback could not remove {} — \
             delete it by hand before retrying, or the retry will append a duplicate shape",
            node_dir.display()
        ))
    }
}

/// Append a whole **review chain** — a `<stem>-chain` node holding `<stem>` /
/// `<stem>-review` / `<stem>-integrate` — under `parent_dir`, in one mutation
/// ([`add_run`]). Returns four paths, the node directory first.
///
/// `producer` is the kind of the leaf that heads the chain; the other two kinds
/// are **derived** from it ([`Kind::review_steps`]), which is what the verb is
/// for. A kind that heads no chain is refused, naming the shape it does have.
///
/// The children keep the stem (`<stem>-review`, not a bare `review`) and the node
/// takes the `-chain` token, both for the same reason: `resolve` matches a bare
/// slug **exactly** and `.grove/` dies at the finish cycle, so a handle has to
/// stay unique tree-wide *and* name its artifact once the tree that explained it
/// is gone (task-tree-scheme §5). Without the token the node's slug would collide
/// with its own first child's.
pub fn leaf_add_chain(
    grove_root: &Path,
    parent_dir: &Path,
    stem: &str,
    producer: Kind,
) -> Result<Vec<PathBuf>> {
    // The stem is validated in its own right, not merely through the slugs it
    // builds: `foo-` is a bad slug, but `foo--review` would pass, so validating
    // only the derived names would let a malformed stem through on two of three.
    validate_slug(stem)?;
    let (review, integrate) = producer.review_steps_or_refuse()?;
    add_run(
        grove_root,
        parent_dir,
        &format!("{stem}-chain"),
        &[
            Step {
                slug: stem.to_string(),
                kind: producer,
                harness: None,
            },
            Step {
                slug: format!("{stem}-review"),
                kind: review,
                harness: None,
            },
            Step {
                slug: format!("{stem}-integrate"),
                kind: integrate,
                harness: None,
            },
        ],
    )
}

/// Append a whole **research vendor pair** — a `<stem>-pair` node holding
/// `<stem>-a` / `<stem>-b` / `<stem>-combine` — under `parent_dir`, in one
/// mutation ([`add_run`]). Returns four paths, the node directory first.
///
/// The producers are `-a` / `-b` rather than a bare stem beside a `-second`,
/// because they are peers rather than a producer and a step.
///
/// **Both producers declare their harness, and the two must differ.** Declaring
/// only the second would leave the first resolving through leaf → kind → family
/// → stamp at *launch* time, which makes "these run on different vendors" a
/// forecast about the environment rather than a fact in the tree — and one that
/// silently decays when routing policy changes after construction. Two surveys
/// on one vendor is not a pair; it is three ordinary research leaves wearing a
/// pair's names, and the equal case is refused before anything is written.
///
/// The `combine-research` step is left undeclared: it is not a peer of the two
/// producers and routes by policy like every other kind. Adding a
/// `**Harness:**` line to it afterwards is a normal edit.
pub fn leaf_add_pair(
    grove_root: &Path,
    parent_dir: &Path,
    stem: &str,
    harness_a: &'static Harness,
    harness_b: &'static Harness,
) -> Result<Vec<PathBuf>> {
    validate_slug(stem)?;
    if harness_a.name == harness_b.name {
        bail!(
            "a vendor pair needs two vendors: --harness-a and --harness-b are both `{}`. \
             The pair exists to union two *different* corpora, and two surveys on one \
             harness is three ordinary research leaves, not a pair. Known harnesses: {}. \
             (Wanted three research leaves on one vendor? That is `leaf-add` three times.)",
            harness_a.name,
            crate::harness::known_names()
        );
    }
    add_run(
        grove_root,
        parent_dir,
        &format!("{stem}-pair"),
        &[
            Step {
                slug: format!("{stem}-a"),
                kind: Kind::Research,
                harness: Some(harness_a),
            },
            Step {
                slug: format!("{stem}-b"),
                kind: Kind::Research,
                harness: Some(harness_b),
            },
            Step {
                slug: format!("{stem}-combine"),
                kind: Kind::CombineResearch,
                harness: None,
            },
        ],
    )
}

/// One entry of a `leaf_insert` renumber: an existing sibling whose per-level
/// position shifted up by one. The key and slug (and, for a node, its whole
/// subtree) are invariant — only the `NN` in this one entry's own name changes —
/// so a `Renumber` records just the position move and the names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Renumber {
    pub old_position: u32,
    pub new_position: u32,
    pub old_name: String,
    pub new_name: String,
}

/// Insert a new leaf at the slot currently held by `target`, shifting `target`
/// and every later sibling in its directory up by one position. `target` is an
/// existing entry — a leaf file or a node directory (absolute, or relative to the
/// grove root). Each shift is a single rename whose source and destination differ
/// only in the leading `NN`; a node directory carries its whole subtree along.
/// Renames run **highest-position-first** so each destination is already vacated.
/// The new leaf gets a fresh key. (Inserting past the last sibling is `leaf_add`'s
/// job — `target` must exist.) Working-tree only — no commit; siblings grown this
/// session are untracked and rename fine ([`crate::tree_rename`]).
///
/// Returns the new leaf's path and the renumber log (ascending by new position);
/// pass the log to [`surface_cross_refs`] to lint stray position-prefixed refs.
pub fn leaf_insert(
    grove_root: &Path,
    target: &Path,
    slug: &str,
    kind: Kind,
    harness: Option<&'static Harness>,
) -> Result<(PathBuf, Vec<Renumber>)> {
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
        slug: slug.to_string(),
        key: new_key,
        outcome: Outcome::Live,
    };
    let path = parent_abs.join(entry.name());
    if path.exists() {
        bail!(
            "destination already exists after renumber: {} (renumber log: {:?})",
            path.display(),
            renumbers
        );
    }
    write_template(&path, slug, new_key, kind, harness)?;
    Ok((path, renumbers))
}

/// Surface stray **position-prefixed** cross-references left stale by a
/// `leaf_insert` renumber, as a lint on stderr — never an auto-rewrite (task-tree-scheme
/// §5: durable references should use the stable `<slug>-k<key>` handle, which a
/// renumber never changes, so the operator reviews each occurrence). Scans every
/// `.md` body in the tree (recursively) for the **old on-disk name** of each
/// renamed entry (`05-mid-k14`), emitting one `path:line: <old-name> (context)`
/// per hit. A stable `<slug>-k<key>` reference is *not* surfaced (it did not move);
/// only the position-prefixed form is stale. Empty renumber log ⇒ nothing to do.
pub fn surface_cross_refs(
    grove_root: &Path,
    renumbers: &[Renumber],
    out: &mut impl std::io::Write,
) -> Result<()> {
    if renumbers.is_empty() {
        return Ok(());
    }
    // The stale tokens are the *old* position-prefixed names the renumber moved
    // (`02-mid-k3`), with any `.md` extension dropped so a path reference
    // `02-mid-k3/01-x-k4.md` matches the directory token. The `-k<digits>` tail
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
/// **A `BRIEF.md` is not required, and used to be.** The guard was the v2
/// translation of v1's "no brief at that position" check, written when every node
/// was a decomposition node and so always carried a charter. A **chain node** is
/// brief-less by rule (task-tree-scheme), which makes the old guard refuse the one
/// case the design names explicitly: `leaf-add <chain-node> <stem>-review`, a step
/// decided on after its producer already ran, appending *inside* the node rather
/// than behind every unrelated live leaf. Node-ness is the name plus being a
/// directory; the charter distinguishes the two *species*, not node from non-node.
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
fn next_child_position(dir: &Path) -> Result<u32> {
    let max = read_children(dir)?
        .iter()
        .filter_map(|(e, _)| e.position())
        .max();
    Ok(max.map_or(1, |m| m + 1))
}

/// Read one directory's grove children — `(Entry, path)` for each name that parses
/// *and* whose real filesystem kind matches (a node is a directory; a leaf/brief is
/// a file), sorted by the per-level comparator. The charter brief is included
/// (callers filter it via `position()` returning `None`). Mirrors `tree_read`'s
/// reconciliation so grow and read agree on what is a sibling.
fn read_children(dir: &Path) -> Result<Vec<(Entry, PathBuf)>> {
    let mut entries: Vec<(String, Entry, PathBuf)> = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(parsed) = parse(&name) else { continue };
        let is_dir = match entry.file_type() {
            Ok(t) => t.is_dir(),
            Err(_) => continue,
        };
        let kind_ok = match parsed {
            Entry::Node { .. } => is_dir,
            Entry::Brief | Entry::Leaf { .. } => !is_dir,
        };
        if kind_ok {
            entries.push((name, parsed, entry.path()));
        }
    }
    entries.sort_by_key(|a| crate::tree_id::sort_key(&a.0));
    Ok(entries.into_iter().map(|(_, e, p)| (e, p)).collect())
}

/// Recursively collect every grove entry's name in the tree (leaves live and
/// `DONE`, node directories, and briefs), descending node directories. Feeds
/// `next_key`, which filters to the keyed names — so this only needs every name.
fn collect_all_names(dir: &Path) -> Result<Vec<String>> {
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
            slug, key, outcome, ..
        } => Entry::Leaf {
            position: new_position,
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

/// Write a freshly-created leaf's template. The first-line header is the
/// **position-free handle** `# <slug>-k<key>` (decided 11.3) — the mutable per-level
/// position lives only in the filename, so a later renumber never rewrites this.
///
/// A declared `harness` adds one `**Harness:**` line immediately under
/// `**Kind:**` — the two routing facts read as one metadata block, and
/// `tree_read` finds either by its own marker wherever it sits. An undeclared
/// harness writes **no line**, not an empty one: the peek treats an empty
/// `**Harness:**` line as an unfinished declaration and refuses to launch, so a
/// template that emitted one would break every leaf grove creates.
fn write_template(
    path: &Path,
    slug: &str,
    key: u32,
    kind: Kind,
    harness: Option<&'static Harness>,
) -> Result<()> {
    let kind_label = kind.label();
    let harness_line = match harness {
        Some(h) => format!("**Harness:** {}\n", h.name),
        None => String::new(),
    };
    let body = format!(
        "# {slug}-k{key}\n\n**Kind:** {kind_label}\n{harness_line}\n## Goal\n\n## Context\n\n## Done when\n\n## Notes\n",
    );
    fs::write(path, body.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    /// A fresh `.grove/` directory (no git — `leaf_add` only writes a file).
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
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

    fn body(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// The directory's child names (files and subdirs), lexically sorted.
    fn list(dir: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(dir)
            .unwrap()
            .filter_map(|e| e.ok())
            .map(|e| e.file_name().to_string_lossy().into_owned())
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
            .collect();
        v.sort();
        v
    }

    // ---- leaf-add -----------------------------------------------------------

    #[test]
    fn add_root_level_child_gets_position_01_and_first_key() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "survey", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-survey-k1.md");
    }

    #[test]
    fn add_appends_gapless_after_existing_root_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        let got = leaf_add(&g, &g, "c", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "03-c-k3.md");
    }

    #[test]
    fn add_child_under_a_node_appends_after_existing_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-y-k4.md");
        assert_eq!(name_of(got.parent().unwrap()), "02-build-k2");
    }

    #[test]
    fn add_first_child_under_a_childless_node() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        let got = leaf_add(&g, &node, "first", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-first-k3.md");
    }

    #[test]
    fn add_assigns_fresh_key_as_max_over_whole_tree_plus_one() {
        // Keys are global, not per-node: the new key is max(key) + 1 across the
        // whole tree, including a deeper subtree's higher key.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let design = mknode(&g, "01-design-k1", "design-k1");
        touch(&design, "01-deep-k7.md", "deep-k7"); // a high key in another subtree
        let build = mknode(&g, "02-build-k2", "build-k2");
        let got = leaf_add(&g, &build, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-y-k8.md");
    }

    #[test]
    fn add_counts_done_children_so_a_retired_slot_is_never_reused() {
        // A `DONE` child still occupies its position — the next child is 02, not 01.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-DONE-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-y-k4.md");
    }

    #[test]
    fn add_counts_abandoned_children_so_a_pruned_slot_is_never_reused() {
        // Symmetric with DONE: an `ABANDONED` child still occupies its position
        // and its key (ADR *pruning*'s key-reuse defect) — the next child is 02
        // and its key is 4, not a reused 3.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-ABANDONED-x-k3.md", "x-k3");
        let got = leaf_add(&g, &node, "y", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-y-k4.md");
    }

    #[test]
    fn add_counts_node_dir_siblings_when_numbering() {
        // A sibling that is a node *directory* still occupies its position.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        mknode(&g, "01-design-k1", "design-k1");
        let got = leaf_add(&g, &g, "build", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "02-build-k2.md");
    }

    #[test]
    fn add_writes_position_free_header_and_kind() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "survey", Kind::Impl, None).unwrap();
        let text = body(&got);
        assert!(
            text.starts_with("# survey-k1\n"),
            "header is the position-free handle; got {text:?}"
        );
        assert!(text.contains("**Kind:** impl"), "got {text:?}");
    }

    #[test]
    fn add_planning_kind_writes_planning_label() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let got = leaf_add(&g, &g, "design", Kind::Planning, None).unwrap();
        assert!(body(&got).contains("**Kind:** planning"));
    }

    #[test]
    fn add_errors_when_parent_node_is_missing() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let missing = g.join("09-nope-k9");
        let err = leaf_add(&g, &missing, "y", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_when_parent_is_a_leaf_file_not_a_node() {
        // A leaf is a *file* — you must decompose it into a node before adding under it.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let leaf = touch(&g, "02-build-k2.md", "build-k2");
        let err = leaf_add(&g, &leaf, "y", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_appends_a_late_step_into_a_brief_less_chain_node() {
        // The lazy-cut shape the node makes possible, and the one place a
        // brief-less node forced a change to a verb this leaf was told not to
        // touch: a review decided on *after* its producer ran is
        // `leaf-add <chain-node> <stem>-review`, landing immediately after its
        // stem-mates. The old parent guard required a `BRIEF.md` and refused it,
        // sending the step to the end of the tree behind every unrelated live leaf
        // — which is exactly what the node shape exists to stop.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let chain = &leaf_add_chain(&g, &g, "sync", Kind::Design).unwrap()[0];

        let got = leaf_add(&g, chain, "sync-second-review", Kind::ReviewDesign, None).unwrap();

        assert_eq!(name_of(&got), "04-sync-second-review-k5.md");
        assert_eq!(name_of(got.parent().unwrap()), "01-sync-chain-k1");
    }

    #[test]
    fn add_errors_when_parent_dir_name_is_not_a_node() {
        // Node-ness is the *name* plus being a directory. A charter is what tells
        // the two node species apart, not node from non-node, so the guard reads
        // the name — a bare `notes/` is still refused.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let bare = g.join("notes");
        fs::create_dir_all(&bare).unwrap();
        let err = leaf_add(&g, &bare, "y", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("parent"), "got {err}");
    }

    #[test]
    fn add_errors_on_invalid_slug() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add(&g, &g, "BRIEF", Kind::Impl, None).is_err());
        assert!(leaf_add(&g, &g, "Bad Slug", Kind::Impl, None).is_err());
    }

    #[test]
    fn add_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = leaf_add(&missing, &missing, "y", Kind::Impl, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-add-chain / leaf-add-pair -------------------------------------
    //
    // The composite verbs. Three properties carry the design and each is pinned
    // by *mutation*, not by asserting the happy path made four things:
    //
    //   * **the derivation** — the caller names one producer kind and the other
    //     two are derived, which is the wrong-but-well-formed error class the
    //     `--kind` write-gate cannot catch (ADR *cli-binary-split*, leg 2);
    //   * **the node carries no `BRIEF.md`** — the discriminator the Retire
    //     cascade reads, so a stray charter silently restores the confirmation
    //     the design removed (task-tree-scheme);
    //   * **one call, one mutation** — a run that fails leaves *no directory*, so
    //     a live prefix of a chain never masquerades as a hand-cut partial one.

    fn names_of(paths: &[PathBuf]) -> Vec<String> {
        paths.iter().map(|p| name_of(p)).collect()
    }

    fn claude() -> &'static Harness {
        crate::harness::by_name("claude").unwrap()
    }
    fn codex() -> &'static Harness {
        crate::harness::by_name("codex").unwrap()
    }

    #[test]
    fn chain_emits_a_node_directory_holding_its_three_steps_off_one_stem() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let paths = leaf_add_chain(&g, &g, "sync-design", Kind::Design).unwrap();
        assert_eq!(
            names_of(&paths),
            vec![
                "01-sync-design-chain-k1",
                "01-sync-design-k2.md",
                "02-sync-design-review-k3.md",
                "03-sync-design-integrate-k4.md",
            ],
            "the node first, then its children at 01..03 — four keys, not three"
        );
        assert!(paths[0].is_dir(), "the shape's head is a directory");
        for step in &paths[1..] {
            assert_eq!(
                name_of(step.parent().unwrap()),
                "01-sync-design-chain-k1",
                "every step is a child of the node"
            );
        }
    }

    #[test]
    fn a_chain_node_carries_no_brief_so_the_cascade_never_asks_about_it() {
        // The discriminator, and the argument that unblocked node-per-chain. The
        // Retire cascade's confirmation is asked of **brief-carrying** nodes, so a
        // charter here would silently restore the noise the design removed — and
        // it would be written because a step demanded it (constraint 4), by a verb
        // that knows only a stem. Asserted as a *file test*, which is the only
        // thing any reader is allowed to key on.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        for paths in [
            leaf_add_chain(&g, &g, "sync", Kind::Design).unwrap(),
            leaf_add_pair(&g, &g, "survey", claude(), codex()).unwrap(),
        ] {
            let node = &paths[0];
            assert!(
                !node.join("BRIEF.md").exists(),
                "{}: a chain node is brief-less by rule",
                name_of(node)
            );
            assert_eq!(
                list(node).len(),
                3,
                "{}: the node holds its three steps and nothing else",
                name_of(node)
            );
        }
    }

    #[test]
    fn chain_derives_the_review_and_integrate_kinds_from_the_producer() {
        // The reason the verb exists. A session hand-transcribing this from the
        // seventeen-row table can pair `review-impl` with a `design` producer —
        // a valid invocation that misroutes the whole chain's discipline.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let paths = leaf_add_chain(&g, &g, "sync-design", Kind::Design).unwrap();
        let kinds: Vec<String> = paths[1..]
            .iter()
            .map(|p| {
                body(p)
                    .lines()
                    .find_map(|l| l.strip_prefix("**Kind:** ").map(str::to_string))
                    .unwrap()
            })
            .collect();
        assert_eq!(
            kinds,
            vec!["design", "review-design", "integrate-review-design"],
            "all three kinds agree on one producer"
        );
    }

    #[test]
    fn chain_derives_every_producer_s_own_steps_not_one_hardcoded_family() {
        // Sweeps the whole producer domain, so a derivation that silently
        // collapsed to (say) the `impl` family would fail on four of five.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        for producer in Kind::ALL.iter().filter(|k| k.review_steps().is_some()) {
            let stem = format!("s-{}", producer.label());
            let paths = leaf_add_chain(&g, &g, &stem, *producer).unwrap();
            assert!(
                body(&paths[2]).contains(&format!("**Kind:** review-{}", producer.label())),
                "{}: review step must name its own producer",
                producer.label()
            );
            assert!(
                body(&paths[3])
                    .contains(&format!("**Kind:** integrate-review-{}", producer.label())),
                "{}: integrate step must name its own producer",
                producer.label()
            );
        }
    }

    #[test]
    fn chain_refuses_research_and_names_the_pair_verb() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let err = leaf_add_chain(&g, &g, "survey", Kind::Research)
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("leaf-add-pair"),
            "the refusal points at the shape research *does* have: {err}"
        );
        assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
    }

    #[test]
    fn chain_refuses_a_kind_that_is_already_a_chain_step() {
        // Passing the chain's *output* where its input goes. Distinct from the
        // research refusal, and it must not be answered by silently deriving a
        // `review-review-design`.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        for step in [Kind::ReviewDesign, Kind::IntegrateReviewImpl] {
            let err = leaf_add_chain(&g, &g, "x", step).unwrap_err().to_string();
            assert!(
                err.contains("already a review-chain") && err.contains("`design`"),
                "{}: refuse, and list the producers: {err}",
                step.label()
            );
        }
        assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
    }

    #[test]
    fn chain_appends_after_existing_siblings_and_under_a_node() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        let node = mknode(&g, "02-build-k2", "build-k2");
        touch(&node, "01-x-k3.md", "x-k3");
        let paths = leaf_add_chain(&g, &node, "api", Kind::Impl).unwrap();
        assert_eq!(
            names_of(&paths),
            vec![
                "02-api-chain-k4",
                "01-api-k5.md",
                "02-api-review-k6.md",
                "03-api-integrate-k7.md",
            ],
            "the node continues the parent's positions; the keys continue the whole tree, \
             and the steps restart at 01 inside the node"
        );
        assert_eq!(name_of(paths[0].parent().unwrap()), "02-build-k2");
    }

    #[test]
    fn pair_emits_two_declared_producers_and_an_undeclared_combine() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let paths = leaf_add_pair(&g, &g, "sync-survey", claude(), codex()).unwrap();
        assert_eq!(
            names_of(&paths),
            vec![
                "01-sync-survey-pair-k1",
                "01-sync-survey-a-k2.md",
                "02-sync-survey-b-k3.md",
                "03-sync-survey-combine-k4.md",
            ]
        );
        assert!(body(&paths[1]).contains("**Kind:** research"));
        assert!(body(&paths[2]).contains("**Kind:** research"));
        assert!(body(&paths[3]).contains("**Kind:** combine-research"));
    }

    #[test]
    fn pair_declares_both_producers_so_two_vendors_is_a_fact_in_the_tree() {
        // F2, and the reason `--harness-a` exists at all. Declaring only the
        // second leaves the first resolving through leaf → kind → family →
        // stamp at *launch* time — a forecast about routing policy, which can
        // change after the leaves are cut, so the pair could quietly become two
        // surveys on one vendor without a byte of the tree changing.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let paths = leaf_add_pair(&g, &g, "sync-survey", claude(), codex()).unwrap();
        assert!(
            body(&paths[1]).contains("**Harness:** claude"),
            "producer A carries its own declaration: {:?}",
            body(&paths[1])
        );
        assert!(
            body(&paths[2]).contains("**Harness:** codex"),
            "producer B carries its own declaration: {:?}",
            body(&paths[2])
        );
        assert!(
            !body(&paths[3]).contains("**Harness:**"),
            "the combine step is not a peer of the two producers — it routes by \
             policy like every other kind: {:?}",
            body(&paths[3])
        );
    }

    #[test]
    fn pair_refuses_two_producers_on_one_harness_before_writing_anything() {
        // The other half of F2: requiring a declaration is not enough if the two
        // declarations may be equal — that emits exactly the state the verb
        // claims to prevent, a pair that looks like one and is not.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let err = leaf_add_pair(&g, &g, "survey", codex(), codex())
            .unwrap_err()
            .to_string();
        assert!(
            err.contains("two vendors"),
            "the refusal names the property, not the flag: {err}"
        );
        assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
    }

    #[test]
    fn both_verbs_reject_a_malformed_stem_in_its_own_right() {
        // A trailing dash is a bad slug, but `foo--review` and `foo--a` are not,
        // so validating only the *derived* names would let a malformed stem
        // through on two leaves of three.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add_chain(&g, &g, "foo-", Kind::Design).is_err());
        assert!(leaf_add_pair(&g, &g, "foo-", claude(), codex()).is_err());
        assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
    }

    // ---- one call, one mutation (F1) ----------------------------------------

    /// Occupy a run's **node** destination with an entry the numbering does not
    /// see: a *file* carrying a node's name. `read_children` reconciles each
    /// parsed name against its real filesystem kind, so this is skipped when the
    /// position and keys are allocated — and `Path::exists` still finds it when
    /// the destination is checked. Exactly the gap between the parsed tree and
    /// the filesystem that the check exists to cover.
    fn squat_node_destination(g: &Path) -> PathBuf {
        let p = g.join("01-sync-chain-k1");
        fs::write(&p, "not a node\n").unwrap();
        p
    }

    #[test]
    fn a_run_whose_node_destination_is_taken_creates_nothing_at_all() {
        // The reason the destination is checked before the first write: the node
        // is the run's single point of refusal, so a collision costs an error and
        // nothing else. (Under the flat shape this had to sweep three names to get
        // the same property; containment collapsed it to one.)
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        squat_node_destination(&g);

        let err = leaf_add_chain(&g, &g, "sync", Kind::Design)
            .unwrap_err()
            .to_string();

        assert!(
            err.contains("nothing was created"),
            "the error says what the tree now holds: {err}"
        );
        let mut files = list(&g);
        files.sort();
        assert_eq!(
            files,
            vec!["01-sync-chain-k1", "BRIEF.md"],
            "only the squatter and the brief — no half-built node left behind"
        );
    }

    #[test]
    fn a_write_that_fails_mid_run_removes_the_node_directory_entirely() {
        // The residual the up-front check cannot see: a destination that does
        // not exist and still cannot be written. The cheapest deterministic one
        // is also a hazard **specific to composite verbs** — the derived names
        // are longer than the stem the caller validated, so a third name can
        // cross `NAME_MAX` (255) while the node and the first two steps clear it.
        // `Path::exists` reports false for an over-long path, so the check waves
        // it through and the failure lands mid-write, which is the arm under test.
        //
        // What is asserted is that **no directory survives** — the whole reason
        // the node shape makes rollback easier than the flat one did. A leftover
        // `NN-<stem>-chain-k<key>/` holding one or two steps is precisely the
        // wrong-but-well-formed residue, and it is worse than the flat version
        // was: it *looks* like a deliberately cut chain node.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        // The node is `NN-<stem>-chain-k<key>` (stem+12); the steps are
        // `NN-<stem>-k<key>.md` (stem+9) plus 7 for `-review` and 10 for
        // `-integrate`. At 238 the run plans a 250-byte directory and 247-, 254-
        // and 257-byte children: the node and two steps fit, the third does not.
        let stem = "a".repeat(238);

        let err = leaf_add_chain(&g, &g, &stem, Kind::Design)
            .unwrap_err()
            .to_string();

        assert!(
            err.contains("rolled back"),
            "the error says the run was undone: {err}"
        );
        assert_eq!(
            list(&g),
            vec!["BRIEF.md"],
            "the node directory and the two steps inside it must not survive the failure"
        );
    }

    #[test]
    fn a_run_that_cannot_get_four_fresh_keys_creates_nothing_at_all() {
        // The arm `chain-node-review-k10` broke the shape on: keys were derived
        // per step as `node_key + 1 + i`, *after* `create_dir`, so a tree near the
        // `u32` ceiling panicked mid-write (debug) or wrapped to `k0` (release),
        // leaving a live two-step node behind — the wrong-but-well-formed residue
        // that reads exactly like a deliberately cut partial chain.
        //
        // The property is not "the arithmetic is checked" but **where the refusal
        // lands**: key exhaustion is a *resolution* failure, so it belongs beside
        // slug validation and the destination check, before the first write. This
        // is the reproduction from the finding, asserted as a refusal.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-old-k4294967292.md", "old-k4294967292");

        let err = leaf_add_chain(&g, &g, "sync", Kind::Impl)
            .unwrap_err()
            .to_string();

        assert!(
            err.contains("nothing was created"),
            "the refusal says what the tree now holds: {err}"
        );
        assert_eq!(
            list(&g),
            vec!["01-old-k4294967292.md", "BRIEF.md"],
            "no node directory, not even an empty one"
        );
    }

    #[test]
    fn a_rolled_back_run_leaves_the_next_call_a_clean_slate() {
        // Why all-or-nothing is worth the machinery: a retry after a failure
        // must produce the shape, not a second copy of it under new positions
        // and keys. Pinned end to end — fail, clear the obstruction, retry.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let squatter = squat_node_destination(&g);
        assert!(leaf_add_chain(&g, &g, "sync", Kind::Design).is_err());
        fs::remove_file(&squatter).unwrap();

        let paths = leaf_add_chain(&g, &g, "sync", Kind::Design).unwrap();

        assert_eq!(
            names_of(&paths),
            vec![
                "01-sync-chain-k1",
                "01-sync-k2.md",
                "02-sync-review-k3.md",
                "03-sync-integrate-k4.md",
            ],
            "the retry got the position and keys the failed run had planned"
        );
        assert_eq!(
            list(&g),
            vec!["01-sync-chain-k1", "BRIEF.md"],
            "exactly one chain node — no duplicate from the failed attempt"
        );
    }

    #[test]
    fn a_refused_run_does_not_consume_positions_or_keys() {
        // The allocation happens after validation, so a refusal costs nothing:
        // an ordinary `leaf-add` afterwards still gets 01/k1.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        assert!(leaf_add_chain(&g, &g, "survey", Kind::Research).is_err());
        assert!(leaf_add_pair(&g, &g, "survey", codex(), codex()).is_err());
        let got = leaf_add(&g, &g, "plain", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&got), "01-plain-k1.md");
    }

    #[test]
    fn a_shape_is_byte_identical_to_the_same_node_and_leaves_cut_by_hand() {
        // Constraint 6, and the claim the spec makes: nothing about a generated
        // chain is distinguishable afterwards. If this ever diverges, the verbs
        // have started writing a *format* rather than a convention.
        //
        // "By hand" is now `mkdir` + three `leaf-add`s into the bare directory,
        // which is only possible because node-ness is the name plus being a
        // directory — a parent guard that still demanded a `BRIEF.md` would make
        // the hand-cut shape unreachable and this equivalence false.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        let generated = leaf_add_chain(&g, &g, "sync", Kind::Design).unwrap();
        let generated_bodies: Vec<String> = generated[1..].iter().map(|p| body(p)).collect();
        fs::remove_dir_all(&generated[0]).unwrap();

        let node = g.join("01-sync-chain-k1");
        fs::create_dir(&node).unwrap();
        let by_hand = [
            node.clone(),
            leaf_add(&g, &node, "sync", Kind::Design, None).unwrap(),
            leaf_add(&g, &node, "sync-review", Kind::ReviewDesign, None).unwrap(),
            leaf_add(
                &g,
                &node,
                "sync-integrate",
                Kind::IntegrateReviewDesign,
                None,
            )
            .unwrap(),
        ];
        assert_eq!(names_of(&generated), names_of(&by_hand));
        let hand_bodies: Vec<String> = by_hand[1..].iter().map(|p| body(p)).collect();
        assert_eq!(generated_bodies, hand_bodies);
        assert!(
            !node.join("BRIEF.md").exists(),
            "neither shape has a charter — that is what makes them the same node"
        );
    }

    // ---- leaf-insert --------------------------------------------------------

    #[test]
    fn insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        touch(&g, "03-c-k3.md", "c-k3");
        stage_all(&g);
        let (path, _renums) =
            leaf_insert(&g, &g.join("02-b-k2.md"), "new", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "02-new-k4.md"); // fresh key, not a reused one
        let files = list(&g);
        assert!(
            files.contains(&"01-a-k1.md".to_string()),
            "pos 1 < 2, unchanged"
        );
        assert!(
            files.contains(&"02-new-k4.md".to_string()),
            "the inserted leaf"
        );
        assert!(
            files.contains(&"03-b-k2.md".to_string()),
            "b: 02->03, key 2 kept"
        );
        assert!(
            files.contains(&"04-c-k3.md".to_string()),
            "c: 03->04, key 3 kept"
        );
        assert!(!files.contains(&"02-b-k2.md".to_string()), "old name gone");
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
            "01-x-k4.md",
            "# x-k4\n\n**Kind:** impl\n\n## Goal\nstuff\n",
        );
        let grandchild_before = body(&grandchild);
        let brief_before = body(&mid.join("BRIEF.md"));
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &g.join("02-mid-k3"), "new", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "02-new-k5.md");
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
            body(&shifted.join("01-x-k4.md")),
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
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);
        let (path, _r) = leaf_insert(&g, &g.join("01-a-k1.md"), "head", Kind::Impl, None).unwrap();
        let text = body(&path);
        assert!(text.starts_with("# head-k2\n"), "got {text:?}");
        assert!(text.contains("**Kind:** impl"), "got {text:?}");
    }

    #[test]
    fn insert_does_not_rewrite_any_existing_file_contents() {
        // Position-free headers ⇒ a shifted leaf's body is byte-identical too.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let b = touch_body(&g, "02-b-k2.md", "# b-k2\n\nbody text\n");
        let b_before = body(&b);
        stage_all(&g);
        leaf_insert(&g, &g.join("02-b-k2.md"), "new", Kind::Impl, None).unwrap();
        assert_eq!(
            body(&g.join("03-b-k2.md")),
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
            touch(&g, &format!("{i:02}-s{i}-k{i}.md"), &format!("s{i}-k{i}"));
        }
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &g.join("01-s1-k1.md"), "head", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "01-head-k6.md");
        assert_eq!(renums.len(), 5);
        let leaves: Vec<String> = list(&g).into_iter().filter(|n| n != "BRIEF.md").collect();
        assert_eq!(
            leaves,
            vec![
                "01-head-k6.md",
                "02-s1-k1.md",
                "03-s2-k2.md",
                "04-s3-k3.md",
                "05-s4-k4.md",
                "06-s5-k5.md",
            ],
            "all six leaves present, gapless 01..06 — no file lost to a collision"
        );
    }

    #[test]
    fn insert_returns_renumber_log_ascending_by_new_position() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        touch(&g, "03-c-k3.md", "c-k3");
        stage_all(&g);
        let (_path, renums) =
            leaf_insert(&g, &g.join("01-a-k1.md"), "head", Kind::Impl, None).unwrap();
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
        touch(&design, "01-a-k2.md", "a-k2");
        touch(&design, "02-b-k3.md", "b-k3");
        stage_all(&g);
        let (path, renums) =
            leaf_insert(&g, &design.join("01-a-k2.md"), "first", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&path), "01-first-k4.md");
        assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
        let children = list(&design);
        assert!(children.contains(&"01-first-k4.md".to_string()));
        assert!(children.contains(&"02-a-k2.md".to_string()));
        assert!(children.contains(&"03-b-k3.md".to_string()));
        assert_eq!(renums.len(), 2);
    }

    #[test]
    fn insert_errors_on_invalid_slug() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);
        assert!(leaf_insert(&g, &g.join("01-a-k1.md"), "BRIEF", Kind::Impl, None).is_err());
    }

    #[test]
    fn insert_errors_when_target_is_a_brief() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        let err = leaf_insert(&g, &g.join("BRIEF.md"), "x", Kind::Impl, None).unwrap_err();
        assert!(err.to_string().contains("brief"), "got {err}");
    }

    #[test]
    fn insert_errors_when_target_missing() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        stage_all(&g);
        assert!(leaf_insert(&g, &g.join("09-nope-k9.md"), "x", Kind::Impl, None).is_err());
    }

    #[test]
    fn insert_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err =
            leaf_insert(&missing, &missing.join("01-a-k1.md"), "x", Kind::Impl, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- insert over untracked entries (issue #3) ----------------------------
    //
    // The grow verbs are working-tree-only by design — `leaf_add` writes an
    // *untracked* file and the enclosing task's commit folds it in. So the
    // ordinary rhythm of a planning session (grow several leaves, then realise
    // one must sequence earlier) hands `leaf_insert` siblings that are not in
    // git's index. Renaming those is `fs::rename`'s job, not `git mv`'s.

    #[test]
    fn insert_ahead_of_an_untracked_sibling_added_this_session() {
        // Issue #3 verbatim: `leaf_add` then `leaf_insert` ahead of what it made,
        // with no `git add` in between — the ordinary planning-session sequence.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md", "root — brief");
        let release = leaf_add(&g, &g, "release", Kind::Impl, None).unwrap();
        assert_eq!(name_of(&release), "01-release-k1.md");

        // No stage_all: the leaf is untracked, exactly as `leaf_add` left it.
        let (path, renums) = leaf_insert(&g, &release, "review", Kind::Impl, None).unwrap();

        assert_eq!(name_of(&path), "01-review-k2.md");
        let files = list(&g);
        assert!(
            files.contains(&"02-release-k1.md".to_string()),
            "the untracked sibling shifted 01->02, key preserved (files: {files:?})"
        );
        assert!(
            !files.contains(&"01-release-k1.md".to_string()),
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
        touch(&g, "01-a-k1.md", "a-k1");
        touch(&g, "02-b-k2.md", "b-k2");
        stage_all(&g); // a and b are tracked
        touch(&g, "03-c-k3.md", "c-k3"); // c is not

        let (path, renums) =
            leaf_insert(&g, &g.join("01-a-k1.md"), "new", Kind::Impl, None).unwrap();

        assert_eq!(name_of(&path), "01-new-k4.md");
        let files = list(&g);
        for expected in ["02-a-k1.md", "03-b-k2.md", "04-c-k3.md"] {
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
        touch(&g, "01-a-k1.md", "a-k1");
        stage_all(&g);

        leaf_insert(&g, &g.join("01-a-k1.md"), "new", Kind::Impl, None).unwrap();

        let idx = indexed(&g);
        assert!(
            idx.contains(&"02-a-k1.md".to_string()),
            "the tracked sibling's index entry moved to the new name (index: {idx:?})"
        );
        assert!(
            !idx.contains(&"01-a-k1.md".to_string()),
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
            "01-a-k1.md",
            "# a-k1\n\nrefers to 02-mid-k3 somewhere\n",
        );
        assert_eq!(surfaced(&g, &[]), "");
    }

    #[test]
    fn surface_reports_a_stale_position_prefixed_reference_in_a_body() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "01-a-k1.md",
            "# a-k1\n\nthe design lives at 02-mid-k3/01-x-k4.md\n",
        );
        let out = surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]);
        assert!(out.contains("01-a-k1.md"), "names the file: {out:?}");
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
        touch_body(&g, "01-a-k1.md", "# a-k1\n\nsee mid-k3 for the design\n");
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
        touch_body(&design, "01-a-k2.md", "# a-k2\n\nalso 02-mid-k3 here\n");
        let out = surfaced(&g, &[renum(2, 3, "02-mid-k3", "03-mid-k3")]);
        assert!(
            out.contains("BRIEF.md") && out.contains("02-mid-k3"),
            "{out:?}"
        );
        assert!(out.contains("01-a-k2.md"), "nested file surfaced: {out:?}");
    }
}

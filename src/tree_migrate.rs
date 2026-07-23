// The **v2 migration** (task-tree-scheme): the one-time, in-place conversion of a task
// tree to the v2 **directory** scheme — `NN-<slug>-k<key>/` node dirs holding a
// `BRIEF.md` + numbered children, leaves `NN-[DONE-]<slug>-k<key>.md`. It reuses
// task-tree-scheme's migrate-on-adoption mechanism (the same one the v1 `migrate.rs`
// rides) but targets the directory shape, and it accepts **two** source formats:
//
//   * **v1-flat** (`<dotted>-[<key>]-<slug>[.BRIEF|.DONE].md`, the scheme this
//     grove flipped to at 070/040) — the primary case;
//   * the **old `NNN-slug/` + `done/`** directory tree — for any grove that never
//     opened since 070/040, converted straight to v2 (no v1-flat way-station).
//
// Per task-tree-scheme §5 / the 11.3 settled note, a v2 task file's first-line header is
// the **position-free handle** `# <slug>-k<key>` (`# … — brief` for a node), so the
// migration rewrites every old header (`# <dotted>-[<key>]-<slug>` or `# NNN-slug`)
// down to that handle while preserving any ` — brief` tail.
//
// **Structure** mirrors `migrate.rs`: a pure `plan` (reads the directory shape,
// mutates nothing — unit-testable without a git repo) and an impure `execute`
// (renames + header rewrites, **no commit** — a reviewable git change). Both
// source readers lower to one intermediate — `Logical` (a `LeafId`-shaped id + the
// source path + the old header token) — and a single `render` maps that to v2
// directory destinations. So the only format-specific code is the two readers; the
// v2 placement rules live in exactly one place.
//
// **Keys.** v1-flat already carries permanent keys → preserved verbatim. The old
// `NNN-slug` format has none → keys are assigned fresh in **DFS pre-order** (a
// node's brief takes its key before its children), starting at `1` — deterministic
// and re-runnable, so the fixtures below pin exact output.
//
// Built **isolated**, mirroring `tree_id` / `tree_read` / `tree_grow` /
// `tree_lifecycle`: this module is NOT wired into the live `grove do` adoption
// (`launch.rs`) or `grove migrate` (`cli.rs`) path — those still call the v1
// `migrate.rs` until the user-gated re-flip (11.6) swaps `migrate::` → this module
// and sweeps the v1 reader. So no leaf in this node can break the live grove. The
// only old-format readers it leans on are `leaf::split_prefix` (old `NNN-slug`) and
// `leaf_id::parse` (v1-flat) — both already the migration's one-time inputs.

use crate::cli::MigrateArgs;
use crate::leaf::split_prefix;
use crate::leaf_id::{self, LeafId};
use crate::repo;
use crate::tree_id::{self, Entry};
use crate::tree_rename::rename_entry;
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// What a migration attempt did. Both no-op outcomes are clean (not errors), so
/// adoption (11.6) can call `migrate` unconditionally on every `grove do`.
#[derive(Debug)]
pub enum Outcome {
    /// The tree was v1-flat or old-`NNN` and was converted; carries the move log.
    Migrated(Vec<Rename>),
    /// The tree is already v2-directories — nothing to do.
    AlreadyV2,
    /// No recognizable migratable tree (`.grove/` absent, or present but holding
    /// only the root brief / foreign files).
    NothingToMigrate,
}

/// One file moved by the migration: the old path and the new path, both relative
/// to the grove root (e.g. `5.1-[6]-id-model.DONE.md` →
/// `05-dotted-decimal-numbering-k5/01-DONE-id-model-k6.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rename {
    pub from_rel: PathBuf,
    pub to_rel: PathBuf,
}

/// The detected on-disk format of a `.grove/` tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    /// Already v2 — a keyed `…-k<key>.md` leaf or a `NN-<slug>-k<key>/` node dir.
    V2,
    /// v1-flat — a `<dotted>-[<key>]-<slug>` keyed file (the 070/040 scheme).
    V1Flat,
    /// Old — a `done/` dir, an `NNN-slug/` node dir, or an `NNN-slug.md` leaf.
    OldNnn,
    /// Neither — only the root brief and/or foreign files; nothing to migrate.
    Empty,
}

/// The computed migration: the detected format and, for a migratable tree, the
/// ordered list of moves (DFS pre-order). Pure — reads the directory shape but
/// mutates nothing — so the order/position/key mapping is unit-testable.
struct Plan {
    format: Format,
    moves: Vec<PlannedMove>,
}

/// A single planned move plus the data to rewrite its first-line header.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedMove {
    /// Old path relative to the grove root.
    from_rel: PathBuf,
    /// New v2 path relative to the grove root (may include node directories).
    to_rel: PathBuf,
    /// The old `# …` header token to match (the v1-flat stem
    /// `<dotted>-[<key>]-<slug>`, or the old `NNN-slug`). Rewritten only if the
    /// first line is `# <old_token>…`; any trailing text (e.g. ` — brief`) is kept.
    old_token: String,
    /// The position-free handle `<slug>-k<key>` replacing the old token.
    new_handle: String,
}

/// One entity lowered from either source format into the common shape the v2
/// renderer consumes: a `LeafId`-shaped id (position vector + permanent key + slug
/// + brief/done flags), the file's absolute source path, and the old header token.
struct Logical {
    id: LeafId,
    from_abs: PathBuf,
    old_token: String,
}

/// Migrate the `.grove/` tree under `worktree` to v2-directories in place (no
/// commit). Idempotent: a clean no-op on an already-v2 tree and on a
/// missing/foreign `.grove/`.
pub fn migrate(worktree: &Path) -> Result<Outcome> {
    let grove_root = worktree.join(".grove");
    if !grove_root.is_dir() {
        return Ok(Outcome::NothingToMigrate);
    }
    let plan = plan(&grove_root)?;
    match plan.format {
        Format::V2 => Ok(Outcome::AlreadyV2),
        Format::Empty => Ok(Outcome::NothingToMigrate),
        Format::V1Flat | Format::OldNnn => {
            let renames = execute(&grove_root, &plan.moves)?;
            cleanup_empty_dirs(&grove_root)?;
            Ok(Outcome::Migrated(renames))
        }
    }
}

/// Adoption-migrate (task-tree-scheme, wired live by 11.6): the step `grove do` runs
/// **before driving** so the loop only ever sees a v2 tree. It detects the format
/// via [`migrate`]; if the tree was migratable it converts it **and commits** the
/// conversion as one clear, reviewable commit, then driving proceeds v2.
/// A v2 / empty / absent `.grove/` is a clean no-op — no commit, no churn.
///
/// Idempotent and safe under `restart ≡ continuation` (self-driving-loop): the conversion
/// is re-runnable, and the commit makes the old→v2 boundary crisp, so a re-run
/// after a completed migrate sees a v2 tree ([`Outcome::AlreadyV2`]) and does
/// nothing.
pub fn migrate_on_adoption(worktree: &Path, name: &str) -> Result<Outcome> {
    let outcome = migrate(worktree)?;
    if let Outcome::Migrated(_) = &outcome {
        commit_migration(worktree, name)?;
    }
    Ok(outcome)
}

/// Commit the adoption migration as one isolated, self-describing commit,
/// scoped to `.grove/` so an unrelated dirty file elsewhere is never swept in.
///
/// In a **jj-enabled** tree (jj-first, colocated included) that is one
/// fileset-scoped `jj commit` — jj's snapshot picks up the renames and header
/// rewrites, no staging step exists, and in a colocated repo jj's auto-export
/// keeps the git view in step (where a git-made commit would be one jj must
/// import). In a git tree, staging via a plain `git add` is required because
/// the post-rename header rewrites left each renamed file
/// modified-after-staging (and an untracked v1 leaf, renamed without touching
/// the index, needs adding regardless).
fn commit_migration(worktree: &Path, name: &str) -> Result<()> {
    let msg = format!("grove({name}): migrate task tree to v2 directory scheme");
    if matches!(repo::vcs_of(worktree), Some(repo::Vcs::Jj { .. })) {
        jj(worktree, &["commit", "-m", &msg, ".grove"])?;
    } else {
        git(worktree, &["add", "-A", "--", ".grove"])?;
        git(worktree, &["commit", "-q", "-m", &msg])?;
    }
    Ok(())
}

/// Run `jj <args>` in `dir`, bailing with stderr on a non-zero exit.
fn jj(dir: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("jj")
        .current_dir(dir)
        .args(args)
        .output()
        .with_context(|| format!("running jj {}", args.join(" ")))?;
    if !out.status.success() {
        bail!(
            "jj {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// Run `git -C <dir> <args>`, bailing with stderr on a non-zero exit.
fn git(dir: &Path, args: &[&str]) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(dir)
        .args(args)
        .output()
        .with_context(|| format!("running git {}", args.join(" ")))?;
    if !out.status.success() {
        bail!(
            "git {} failed: {}",
            args.join(" "),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// The `grove migrate [path]` CLI handler (wired live by 11.6). `path` is the
/// worktree whose `.grove/` to migrate (default: the current worktree). Prints a
/// move summary.
pub fn run(args: &MigrateArgs) -> Result<()> {
    let worktree = match &args.path {
        Some(p) => p.clone(),
        None => repo::toplevel(&std::env::current_dir().context("getting cwd")?)?,
    };
    let grove_root = worktree.join(".grove");
    match migrate(&worktree)? {
        Outcome::Migrated(renames) => {
            println!(
                "migrated {} file{} to v2 directory scheme:",
                renames.len(),
                if renames.len() == 1 { "" } else { "s" }
            );
            for r in &renames {
                println!("  {} -> {}", r.from_rel.display(), r.to_rel.display());
            }
            println!("review the working-tree changes, then commit.");
        }
        Outcome::AlreadyV2 => {
            eprintln!("grove migrate: already v2 (directory scheme); nothing to do");
        }
        Outcome::NothingToMigrate => {
            if grove_root.is_dir() {
                eprintln!(
                    "grove migrate: no migratable tree in {}",
                    grove_root.display()
                );
            } else {
                eprintln!("grove migrate: no .grove/ at {}", grove_root.display());
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// planning (pure: reads the directory shape, mutates nothing)

/// Detect the format and, for a migratable tree, compute the ordered move list.
fn plan(grove_root: &Path) -> Result<Plan> {
    let format = detect(grove_root)?;
    let logicals = match format {
        Format::V1Flat => read_v1flat(grove_root)?,
        Format::OldNnn => read_old(grove_root)?,
        Format::V2 | Format::Empty => Vec::new(),
    };
    let moves = render(grove_root, &logicals)?;
    Ok(Plan { format, moves })
}

/// Classify a `.grove/` tree by scanning its top level (see [`Format`]). Precedence
/// is V2 > V1Flat > OldNnn: a v2 marker means the tree is already migrated (never
/// touched), then the v1-flat scheme this grove rode, then the old format for an
/// un-opened tree. A v1-flat tree is *flat* (its only directories are foreign), so
/// a directory is only ever a v2 node dir, an old `NNN-slug/` node, or `done/`.
fn detect(grove_root: &Path) -> Result<Format> {
    let mut has_v2 = false;
    let mut has_v1 = false;
    let mut has_old = false;
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "BRIEF.md" {
            continue; // the unkeyed root brief is format-neutral
        }
        let ft = entry.file_type()?;
        if ft.is_dir() {
            if matches!(tree_id::parse(&name), Some(Entry::Node { .. })) {
                has_v2 = true; // a v2 node directory
            } else if name == "done" || split_prefix(&name).is_some() {
                has_old = true; // a `done/` mirror or an old `NNN-slug/` node dir
            }
        } else if ft.is_file() {
            if matches!(tree_id::parse(&name), Some(Entry::Leaf { .. })) {
                has_v2 = true; // a v2 `…-k<key>.md` leaf
            } else if leaf_id::parse(&name).is_some() {
                has_v1 = true; // a v1-flat `<dotted>-[<key>]-<slug>` keyed file
            } else if name.strip_suffix(".md").and_then(split_prefix).is_some() {
                has_old = true; // an old `NNN-slug.md` leaf
            }
        }
    }
    Ok(if has_v2 {
        Format::V2
    } else if has_v1 {
        Format::V1Flat
    } else if has_old {
        Format::OldNnn
    } else {
        Format::Empty
    })
}

/// Read a **v1-flat** tree: it is flat, so a single `read_dir` of the grove root,
/// parsing each keyed filename into its `LeafId` (keys preserved). The root brief
/// and foreign files are skipped. Sorted by position vector (DFS pre-order) so the
/// rendered moves are deterministic.
fn read_v1flat(grove_root: &Path) -> Result<Vec<Logical>> {
    let mut logicals = Vec::new();
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue; // v1-flat is flat; any directory is foreign
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "BRIEF.md" {
            continue; // the unkeyed root brief stays put, never moved
        }
        let Some(id) = leaf_id::parse(&name) else {
            continue; // foreign file
        };
        let old_token = v1flat_token(&id);
        logicals.push(Logical {
            id,
            from_abs: entry.path(),
            old_token,
        });
    }
    logicals.sort_by(|a, b| a.id.position.cmp(&b.id.position));
    Ok(logicals)
}

/// The v1-flat first-line header token for an id: `<dotted-position>-[<key>]-<slug>`
/// (no marker — headers never carry `.BRIEF`/`.DONE`). Matched against the old
/// `# …` header so the rewrite can replace it with the position-free handle.
fn v1flat_token(id: &LeafId) -> String {
    let dotted = id
        .position
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".");
    format!(
        "{}-[{}]-{}",
        dotted,
        id.key.expect("a v1-flat keyed file always has a key"),
        id.slug
    )
}

/// Read an **old `NNN-slug/` + `done/`** tree into the common `Logical` shape:
/// build the unified forest (live + `done/` mirror merged by logical path), then
/// walk it DFS pre-order assigning each entity its position (1-based sibling index)
/// and a fresh key. Same forest reader as v1's `migrate.rs`, re-targeted to v2.
fn read_old(grove_root: &Path) -> Result<Vec<Logical>> {
    let forest = collect_level(grove_root, Path::new(""))?;
    let mut logicals = Vec::new();
    let mut next_key = 1u32;
    assign_old(&forest, &[], &mut next_key, &mut logicals);
    Ok(logicals)
}

/// Lower a list of [`Logical`] entities to v2 directory moves. First indexes every
/// node (a brief with a non-empty position) by its position vector to recover its
/// v2 directory name; then, for each entity, walks its ancestor positions through
/// that index to build the destination directory path, and renders the final
/// component (a node's `…/BRIEF.md`, or a leaf file). An entity whose ancestor has
/// no node brief is an orphan (a malformed tree) → a clean bail, never a panic.
fn render(grove_root: &Path, logicals: &[Logical]) -> Result<Vec<PlannedMove>> {
    // The v2 directory name for each node, keyed by its position vector.
    let mut node_dir: BTreeMap<Vec<u32>, String> = BTreeMap::new();
    for l in logicals {
        if l.id.is_brief && !l.id.position.is_empty() {
            node_dir.insert(l.id.position.clone(), node_dir_name(&l.id));
        }
    }

    let mut moves = Vec::with_capacity(logicals.len());
    for l in logicals {
        let pos = &l.id.position;
        if pos.is_empty() {
            continue; // the root brief is never moved (and is excluded upstream)
        }
        // The destination directory: the v2 dir name of each ancestor position
        // (prefixes of length 1..pos.len()).
        let mut dest = PathBuf::new();
        for depth in 1..pos.len() {
            let prefix = &pos[..depth];
            let dir = node_dir.get(prefix).with_context(|| {
                format!(
                    "orphaned entity {} — no node brief at ancestor position {:?}",
                    l.from_abs.display(),
                    prefix
                )
            })?;
            dest.push(dir);
        }

        let key = l.id.key.expect("a migratable entity is always keyed");
        let new_handle = format!("{}-k{}", l.id.slug, key);
        if l.id.is_brief {
            // A node: its directory (recorded above) holding the brief as BRIEF.md.
            let dir = node_dir
                .get(pos)
                .expect("this brief was registered into node_dir above");
            dest.push(dir);
            dest.push("BRIEF.md");
        } else {
            // A leaf file at its per-level position (the last dotted component).
            let per_level = *pos.last().expect("non-empty position");
            let leaf = Entry::Leaf {
                position: per_level,
                slug: l.id.slug.clone(),
                key,
                // A migrated v1 tree never contains an abandoned mark (that
                // concept postdates v1) — only `is_done` survives the crossing.
                // Qualified (not imported bare) because this module's own
                // `Outcome` (the migration result) already owns that name.
                outcome: if l.id.is_done {
                    tree_id::Outcome::Done
                } else {
                    tree_id::Outcome::Live
                },
            };
            dest.push(leaf.name());
        }

        let from_rel = l
            .from_abs
            .strip_prefix(grove_root)
            .with_context(|| {
                format!(
                    "{} is not under grove root {}",
                    l.from_abs.display(),
                    grove_root.display()
                )
            })?
            .to_path_buf();
        moves.push(PlannedMove {
            from_rel,
            to_rel: dest,
            old_token: l.old_token.clone(),
            new_handle,
        });
    }
    Ok(moves)
}

/// The v2 node directory name for a node id: `NN-<slug>-k<key>` at its per-level
/// position (the last dotted component).
fn node_dir_name(id: &LeafId) -> String {
    Entry::Node {
        position: *id.position.last().expect("a node has a non-empty position"),
        slug: id.slug.clone(),
        key: id.key.expect("a node is always keyed"),
    }
    .name()
}

// --- old `NNN-slug/` + `done/` forest reader (ported from v1 `migrate.rs`) ---

/// One node/leaf in the **unified** old tree (live and `done/` mirrors merged by
/// logical path). `nnn` is the old zero-padded numeric prefix (the level's sort
/// key); `slug` is the text after `NNN-`.
enum Entity {
    Leaf {
        nnn: u32,
        slug: String,
        done: bool,
        from_abs: PathBuf,
    },
    Node {
        nnn: u32,
        slug: String,
        brief_abs: Option<PathBuf>,
        children: Vec<Entity>,
    },
}

impl Entity {
    fn nnn(&self) -> u32 {
        match self {
            Entity::Leaf { nnn, .. } | Entity::Node { nnn, .. } => *nnn,
        }
    }
}

/// Collect one logical level of the unified old tree. `chain` is the path of
/// `NNN-slug` directory components from the grove root to this level (empty at the
/// root). The level's children are read from both its live directory and its
/// `done/` mirror and merged by `NNN-slug` name, so a partially-retired node's live
/// and retired children land in one subtree. Returns the level's entities by `NNN`.
fn collect_level(grove_root: &Path, chain: &Path) -> Result<Vec<Entity>> {
    let live_dir = if chain.as_os_str().is_empty() {
        grove_root.to_path_buf()
    } else {
        grove_root.join(chain)
    };
    let done_dir = grove_root.join("done").join(chain);

    struct LeafAcc {
        nnn: u32,
        slug: String,
        done: bool,
        from_abs: PathBuf,
    }
    struct NodeAcc {
        nnn: u32,
        slug: String,
    }
    let mut leaves: BTreeMap<String, LeafAcc> = BTreeMap::new();
    let mut nodes: BTreeMap<String, NodeAcc> = BTreeMap::new();

    for (dir, in_done) in [(live_dir.as_path(), false), (done_dir.as_path(), true)] {
        if !dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let ft = entry.file_type()?;
            if ft.is_dir() {
                // The top-level `done/` mirror is scanned via `done_dir`; it is
                // never descended as if it were a child node of the live root.
                if !in_done && name == "done" {
                    continue;
                }
                if let Some((nnn, slug)) = split_prefix(&name) {
                    nodes.entry(name.clone()).or_insert(NodeAcc {
                        nnn,
                        slug: slug.to_string(),
                    });
                }
            } else if ft.is_file() {
                if name == "BRIEF.md" {
                    continue; // a node's brief, attached to the node at this level
                }
                if let Some((nnn, slug)) = name.strip_suffix(".md").and_then(split_prefix) {
                    leaves.insert(
                        name.clone(),
                        LeafAcc {
                            nnn,
                            slug: slug.to_string(),
                            done: in_done,
                            from_abs: dir.join(&name),
                        },
                    );
                }
            }
        }
    }

    let mut entities: Vec<Entity> = Vec::new();
    for (_name, l) in leaves {
        entities.push(Entity::Leaf {
            nnn: l.nnn,
            slug: l.slug,
            done: l.done,
            from_abs: l.from_abs,
        });
    }
    for (dirname, n) in nodes {
        let child_chain = chain.join(&dirname);
        let children = collect_level(grove_root, &child_chain)?;
        // A partially-retired node keeps its brief live; a fully-retired node's
        // brief lives in the `done/` mirror. Prefer live, else done, else none.
        let live_brief = grove_root.join(&child_chain).join("BRIEF.md");
        let done_brief = grove_root.join("done").join(&child_chain).join("BRIEF.md");
        let brief_abs = if live_brief.is_file() {
            Some(live_brief)
        } else if done_brief.is_file() {
            Some(done_brief)
        } else {
            None
        };
        entities.push(Entity::Node {
            nnn: n.nnn,
            slug: n.slug,
            brief_abs,
            children,
        });
    }
    entities.sort_by_key(Entity::nnn);
    Ok(entities)
}

/// Walk the unified old tree in DFS pre-order, lowering each entity to a
/// [`Logical`] with its position (1-based sibling index appended to the parent's)
/// and the next fresh key (a node's brief takes its key before its children). A
/// brief-less node consumes a position slot but emits no `Logical` (no file to
/// move); its children then have no v2 directory to land in, which `render` reports
/// as an orphan — the same degenerate case v1 tolerated only because flat needed no
/// parent directory.
fn assign_old(entities: &[Entity], parent_pos: &[u32], next_key: &mut u32, out: &mut Vec<Logical>) {
    for (i, e) in entities.iter().enumerate() {
        let mut pos = parent_pos.to_vec();
        pos.push(i as u32 + 1);
        match e {
            Entity::Leaf {
                nnn,
                slug,
                done,
                from_abs,
            } => {
                let key = take_key(next_key);
                out.push(Logical {
                    id: LeafId {
                        position: pos,
                        key: Some(key),
                        slug: slug.clone(),
                        is_brief: false,
                        is_done: *done,
                    },
                    from_abs: from_abs.clone(),
                    old_token: format!("{nnn:03}-{slug}"),
                });
            }
            Entity::Node {
                nnn,
                slug,
                brief_abs,
                children,
            } => {
                if let Some(brief_abs) = brief_abs {
                    let key = take_key(next_key);
                    out.push(Logical {
                        id: LeafId {
                            position: pos.clone(),
                            key: Some(key),
                            slug: slug.clone(),
                            is_brief: true,
                            is_done: false,
                        },
                        from_abs: brief_abs.clone(),
                        old_token: format!("{nnn:03}-{slug}"),
                    });
                }
                assign_old(children, &pos, next_key, out);
            }
        }
    }
}

/// Consume and return the next permanent key.
fn take_key(next_key: &mut u32) -> u32 {
    let key = *next_key;
    *next_key += 1;
    key
}

// ---------------------------------------------------------------------------
// header rewrite

/// Rewrite a file's first-line `# …` header for the move. Returns the new header
/// line, or `None` to leave the file untouched. The first line must be
/// `# <old_token>…` with the token at a word boundary; the leading token is
/// replaced with `# <new_handle>` and any trailing text (e.g. ` — brief`) is
/// preserved. A hand-edited title that does not start with the old token — or where
/// the token is only a partial prefix of a longer one — is left alone.
fn rewrite_header_line(first_line: &str, old_token: &str, new_handle: &str) -> Option<String> {
    let after_hash = first_line.strip_prefix("# ")?;
    let tail = after_hash.strip_prefix(old_token)?;
    // Reject a partial match: the next char must not continue a slug (`[a-z0-9-]`),
    // or `010-plan` would mis-match `010-planted` and `…-v2` mis-match `…-v2x`.
    if tail
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return None;
    }
    Some(format!("# {new_handle}{tail}"))
}

/// Rewrite the first line of a moved file via [`rewrite_header_line`], preserving
/// the rest of the body byte-for-byte. A non-matching first line is left alone.
fn rewrite_header_file(path: &Path, old_token: &str, new_handle: &str) -> Result<()> {
    let body = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let Some(new_first) = rewrite_header_line(first, old_token, new_handle) else {
        return Ok(());
    };
    let mut out = String::with_capacity(body.len() + 16);
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    fs::write(path, out.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

// ---------------------------------------------------------------------------
// execution (impure: renames + header rewrites)

/// Apply each planned move: ensure the destination directory exists (v2 node dirs
/// are created on the fly), rename the file to its new v2 path, then rewrite its
/// first-line header. Aborts (leaving the partial change for the human to reset) if
/// a destination file already exists. No commit.
fn execute(grove_root: &Path, moves: &[PlannedMove]) -> Result<Vec<Rename>> {
    let mut renames = Vec::with_capacity(moves.len());
    for m in moves {
        let dst_abs = grove_root.join(&m.to_rel);
        if dst_abs.exists() {
            bail!(
                "destination already exists: {} (migration aborted)",
                dst_abs.display()
            );
        }
        if let Some(parent) = dst_abs.parent() {
            fs::create_dir_all(parent).with_context(|| format!("creating {}", parent.display()))?;
        }
        rename_entry(grove_root, &m.from_rel, &m.to_rel)?;
        rewrite_header_file(&dst_abs, &m.old_token, &m.new_handle)?;
        renames.push(Rename {
            from_rel: m.from_rel.clone(),
            to_rel: m.to_rel.clone(),
        });
    }
    Ok(renames)
}

/// Remove directories left empty by the migration (the old `NNN-slug/` node dirs
/// and the whole `done/` mirror), bottom-up. Only *empty* directories are removed,
/// so a directory still holding a foreign file — or a freshly-created v2 node dir
/// (which always holds its `BRIEF.md` + children) — survives. A v1-flat source has
/// no source directories, so this is a no-op there.
fn cleanup_empty_dirs(grove_root: &Path) -> Result<()> {
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            remove_if_empty(&entry.path())?;
        }
    }
    Ok(())
}

/// Depth-first: clean a directory's subdirectories, then remove the directory
/// itself if it is now empty.
fn remove_if_empty(dir: &Path) -> Result<()> {
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            remove_if_empty(&entry.path())?;
        }
    }
    if fs::read_dir(dir)?.next().is_none() {
        fs::remove_dir(dir).with_context(|| format!("removing empty dir {}", dir.display()))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lay down a set of grove-root-relative files (empty-ish bodies — `plan` reads
    /// only the directory structure, never file contents).
    fn build(grove_root: &Path, files: &[&str]) {
        for f in files {
            let p = grove_root.join(f);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, b"# stub\n").unwrap();
        }
    }

    fn grove() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    /// The plan's moves as `(from_rel, to_rel)` string pairs, in plan order.
    fn moves_of(plan: &Plan) -> Vec<(String, String)> {
        plan.moves
            .iter()
            .map(|m| {
                (
                    m.from_rel.to_string_lossy().into_owned(),
                    m.to_rel.to_string_lossy().into_owned(),
                )
            })
            .collect()
    }

    // ---- detect -------------------------------------------------------------

    #[test]
    fn detect_v2_by_keyed_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-plan-k1.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    #[test]
    fn detect_v2_by_done_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-DONE-plan-k1.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    #[test]
    fn detect_v2_by_node_directory() {
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "05-design-k5/BRIEF.md",
                "05-design-k5/01-x-k6.md",
            ],
        );
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    #[test]
    fn detect_v1flat_by_keyed_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V1Flat);
    }

    #[test]
    fn detect_v1flat_by_done_and_brief_markers() {
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "1-[1]-plan.DONE.md", "5-[5]-node.BRIEF.md"],
        );
        assert_eq!(detect(&g).unwrap(), Format::V1Flat);
    }

    #[test]
    fn detect_old_by_nnn_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::OldNnn);
    }

    #[test]
    fn detect_old_by_done_dir() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "done/010-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::OldNnn);
    }

    #[test]
    fn detect_old_by_node_dir() {
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "010-node/BRIEF.md", "010-node/010-leaf.md"],
        );
        assert_eq!(detect(&g).unwrap(), Format::OldNnn);
    }

    #[test]
    fn detect_empty_when_only_root_brief() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md"]);
        assert_eq!(detect(&g).unwrap(), Format::Empty);
    }

    #[test]
    fn detect_empty_when_only_foreign_files() {
        let (_t, g) = grove();
        build(&g, &["README.md", "notes.txt"]);
        assert_eq!(detect(&g).unwrap(), Format::Empty);
    }

    #[test]
    fn detect_v2_wins_over_other_markers() {
        // Precedence guard: a stray old marker never re-migrates an already-v2 tree.
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-plan-k1.md", "done/010-old.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    // ---- plan: v1-flat golden tree (this grove's exact shape) ---------------

    #[test]
    fn plan_v1flat_golden_tree_mirrors_this_grove() {
        // The `refactor-to-archon` grove's real v1-flat shape (snapshot): retired
        // root leaves, fully-retired nodes (5/6/7/9 — brief + all-DONE children),
        // and a partially-retired node (11 — brief + DONE children + live leaves).
        // Keys are preserved verbatim, including node 9's out-of-order keys 30..33
        // (decomposed later, after node 11's keys 23..29).
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "1-[1]-plan.DONE.md",
                "2-[2]-loop-substrate-spike.DONE.md",
                "3-[3]-substrate-decision.DONE.md",
                "4-[4]-substrate-wiring.DONE.md",
                "5-[5]-dotted-decimal-numbering.BRIEF.md",
                "5.1-[6]-id-model.DONE.md",
                "5.2-[7]-read-verbs.DONE.md",
                "5.3-[8]-grow-verbs-renumber.DONE.md",
                "5.4-[9]-lifecycle-verbs.DONE.md",
                "6-[10]-backwards-compat-migration.BRIEF.md",
                "6.1-[11]-verbs-live.DONE.md",
                "6.2-[12]-grove-migrate.DONE.md",
                "6.3-[13]-migrate-on-adoption.DONE.md",
                "7-[14]-global-skill-homebrew-distribution.BRIEF.md",
                "7.1-[15]-embed-skill-and-provision.DONE.md",
                "7.2-[16]-new-scheme-prose.DONE.md",
                "7.3-[17]-homebrew-sole-gesture.DONE.md",
                "7.4-[18]-install-and-flip.DONE.md",
                "7.5-[19]-remove-project-skill-mirrors.DONE.md",
                "8-[20]-shed-tui.DONE.md",
                "9-[21]-shed-inbox-and-install-machinery.BRIEF.md",
                "9.1-[30]-migrate-seeds.DONE.md",
                "9.2-[31]-shed-inbox-grove-meta.DONE.md",
                "9.3-[32]-shed-install-and-status.DONE.md",
                "9.4-[33]-sweep-dead-old-verb-modules.DONE.md",
                "10-[22]-complete-terminate-signal.DONE.md",
                "11-[23]-scheme-v2-directories.BRIEF.md",
                "11.1-[24]-id-grammar.DONE.md",
                "11.2-[25]-read-verbs.DONE.md",
                "11.3-[26]-grow-lifecycle-verbs.DONE.md",
                "11.4-[27]-migrate-v1-to-v2.md",
                "11.5-[28]-prose-and-commit-convention.md",
                "11.6-[29]-install-and-reflip-v2.md",
            ],
        );
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::V1Flat);
        assert_eq!(
            moves_of(&plan),
            vec![
                ("1-[1]-plan.DONE.md".into(), "01-DONE-plan-k1.md".into()),
                (
                    "2-[2]-loop-substrate-spike.DONE.md".into(),
                    "02-DONE-loop-substrate-spike-k2.md".into()
                ),
                (
                    "3-[3]-substrate-decision.DONE.md".into(),
                    "03-DONE-substrate-decision-k3.md".into()
                ),
                (
                    "4-[4]-substrate-wiring.DONE.md".into(),
                    "04-DONE-substrate-wiring-k4.md".into()
                ),
                (
                    "5-[5]-dotted-decimal-numbering.BRIEF.md".into(),
                    "05-dotted-decimal-numbering-k5/BRIEF.md".into()
                ),
                (
                    "5.1-[6]-id-model.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/01-DONE-id-model-k6.md".into()
                ),
                (
                    "5.2-[7]-read-verbs.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/02-DONE-read-verbs-k7.md".into()
                ),
                (
                    "5.3-[8]-grow-verbs-renumber.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/03-DONE-grow-verbs-renumber-k8.md".into()
                ),
                (
                    "5.4-[9]-lifecycle-verbs.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/04-DONE-lifecycle-verbs-k9.md".into()
                ),
                (
                    "6-[10]-backwards-compat-migration.BRIEF.md".into(),
                    "06-backwards-compat-migration-k10/BRIEF.md".into()
                ),
                (
                    "6.1-[11]-verbs-live.DONE.md".into(),
                    "06-backwards-compat-migration-k10/01-DONE-verbs-live-k11.md".into()
                ),
                (
                    "6.2-[12]-grove-migrate.DONE.md".into(),
                    "06-backwards-compat-migration-k10/02-DONE-grove-migrate-k12.md".into()
                ),
                (
                    "6.3-[13]-migrate-on-adoption.DONE.md".into(),
                    "06-backwards-compat-migration-k10/03-DONE-migrate-on-adoption-k13.md".into()
                ),
                (
                    "7-[14]-global-skill-homebrew-distribution.BRIEF.md".into(),
                    "07-global-skill-homebrew-distribution-k14/BRIEF.md".into()
                ),
                (
                    "7.1-[15]-embed-skill-and-provision.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/01-DONE-embed-skill-and-provision-k15.md".into()
                ),
                (
                    "7.2-[16]-new-scheme-prose.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/02-DONE-new-scheme-prose-k16.md".into()
                ),
                (
                    "7.3-[17]-homebrew-sole-gesture.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/03-DONE-homebrew-sole-gesture-k17.md".into()
                ),
                (
                    "7.4-[18]-install-and-flip.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/04-DONE-install-and-flip-k18.md".into()
                ),
                (
                    "7.5-[19]-remove-project-skill-mirrors.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/05-DONE-remove-project-skill-mirrors-k19.md".into()
                ),
                ("8-[20]-shed-tui.DONE.md".into(), "08-DONE-shed-tui-k20.md".into()),
                (
                    "9-[21]-shed-inbox-and-install-machinery.BRIEF.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/BRIEF.md".into()
                ),
                (
                    "9.1-[30]-migrate-seeds.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/01-DONE-migrate-seeds-k30.md".into()
                ),
                (
                    "9.2-[31]-shed-inbox-grove-meta.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/02-DONE-shed-inbox-grove-meta-k31.md".into()
                ),
                (
                    "9.3-[32]-shed-install-and-status.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/03-DONE-shed-install-and-status-k32.md".into()
                ),
                (
                    "9.4-[33]-sweep-dead-old-verb-modules.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/04-DONE-sweep-dead-old-verb-modules-k33.md".into()
                ),
                (
                    "10-[22]-complete-terminate-signal.DONE.md".into(),
                    "10-DONE-complete-terminate-signal-k22.md".into()
                ),
                (
                    "11-[23]-scheme-v2-directories.BRIEF.md".into(),
                    "11-scheme-v2-directories-k23/BRIEF.md".into()
                ),
                (
                    "11.1-[24]-id-grammar.DONE.md".into(),
                    "11-scheme-v2-directories-k23/01-DONE-id-grammar-k24.md".into()
                ),
                (
                    "11.2-[25]-read-verbs.DONE.md".into(),
                    "11-scheme-v2-directories-k23/02-DONE-read-verbs-k25.md".into()
                ),
                (
                    "11.3-[26]-grow-lifecycle-verbs.DONE.md".into(),
                    "11-scheme-v2-directories-k23/03-DONE-grow-lifecycle-verbs-k26.md".into()
                ),
                (
                    "11.4-[27]-migrate-v1-to-v2.md".into(),
                    "11-scheme-v2-directories-k23/04-migrate-v1-to-v2-k27.md".into()
                ),
                (
                    "11.5-[28]-prose-and-commit-convention.md".into(),
                    "11-scheme-v2-directories-k23/05-prose-and-commit-convention-k28.md".into()
                ),
                (
                    "11.6-[29]-install-and-reflip-v2.md".into(),
                    "11-scheme-v2-directories-k23/06-install-and-reflip-v2-k29.md".into()
                ),
            ]
        );
    }

    // ---- plan: v1-flat focused cases ----------------------------------------

    #[test]
    fn plan_v1flat_root_brief_stays_put() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md"]);
        let plan = plan(&g).unwrap();
        assert!(
            !plan
                .moves
                .iter()
                .any(|m| m.from_rel == Path::new("BRIEF.md")),
            "root BRIEF.md must not be moved"
        );
    }

    #[test]
    fn plan_v1flat_live_root_leaves_pad_positions_and_keep_keys() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "2-[2]-b.md", "10-[3]-c.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-a.md".into(), "01-a-k1.md".into()),
                ("2-[2]-b.md".into(), "02-b-k2.md".into()),
                ("10-[3]-c.md".into(), "10-c-k3.md".into()),
            ]
        );
    }

    #[test]
    fn plan_v1flat_done_leaf_gets_done_infix() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "2-[2]-b.DONE.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-a.md".into(), "01-a-k1.md".into()),
                ("2-[2]-b.DONE.md".into(), "02-DONE-b-k2.md".into()),
            ]
        );
    }

    #[test]
    fn plan_v1flat_node_brief_becomes_a_directory_and_is_never_done() {
        // A fully-retired node: brief + all-DONE children. The brief becomes a
        // directory (never a `DONE` marker — node done-ness is implicit).
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "1-[1]-node.BRIEF.md", "1.1-[2]-child.DONE.md"],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-node.BRIEF.md".into(), "01-node-k1/BRIEF.md".into()),
                (
                    "1.1-[2]-child.DONE.md".into(),
                    "01-node-k1/01-DONE-child-k2.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_v1flat_deeply_nested_node_nests_directories() {
        // A node within a node: directories nest, each carrying its own key.
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "2-[1]-outer.BRIEF.md",
                "2.3-[2]-inner.BRIEF.md",
                "2.3.1-[3]-deep.md",
            ],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("2-[1]-outer.BRIEF.md".into(), "02-outer-k1/BRIEF.md".into()),
                (
                    "2.3-[2]-inner.BRIEF.md".into(),
                    "02-outer-k1/03-inner-k2/BRIEF.md".into()
                ),
                (
                    "2.3.1-[3]-deep.md".into(),
                    "02-outer-k1/03-inner-k2/01-deep-k3.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_v1flat_double_digit_per_level_position_pads_to_two() {
        // Position 1.10 → per-level 10 → `10-…` (two digits, no truncation).
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-node.BRIEF.md", "1.10-[2]-c10.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-node.BRIEF.md".into(), "01-node-k1/BRIEF.md".into()),
                ("1.10-[2]-c10.md".into(), "01-node-k1/10-c10-k2.md".into()),
            ]
        );
    }

    #[test]
    fn plan_v1flat_foreign_files_are_not_moved() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "README.md", "notes.txt"]);
        let plan = plan(&g).unwrap();
        assert!(
            !plan.moves.iter().any(|m| {
                let f = m.from_rel.to_string_lossy();
                f.contains("README") || f.contains("notes")
            }),
            "foreign files must be left untouched"
        );
    }

    #[test]
    fn plan_already_v2_has_no_moves() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-a-k1.md", "02-DONE-b-k2.md"]);
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::V2);
        assert!(plan.moves.is_empty());
    }

    #[test]
    fn plan_empty_has_no_moves() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md"]);
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::Empty);
        assert!(plan.moves.is_empty());
    }

    // ---- plan: old `NNN-slug/` → v2 directly --------------------------------

    #[test]
    fn plan_old_nnn_golden_tree_converts_straight_to_v2() {
        // An un-opened old-format grove: retired root leaves, a fully-retired node
        // (brief in done/), a partially-retired node (brief live + one done child +
        // two live children), trailing live leaves. Keys assigned fresh DFS pre-order.
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "done/010-plan.md",
                "done/020-loop-substrate-spike.md",
                "done/030-substrate-decision.md",
                "done/040-substrate-wiring.md",
                "done/050-dotted-decimal-numbering/BRIEF.md",
                "done/050-dotted-decimal-numbering/010-id-model.md",
                "done/050-dotted-decimal-numbering/020-read-verbs.md",
                "done/050-dotted-decimal-numbering/030-grow-verbs-renumber.md",
                "done/050-dotted-decimal-numbering/040-lifecycle-verbs.md",
                "done/060-backwards-compat-migration/010-verbs-live.md",
                "060-backwards-compat-migration/BRIEF.md",
                "060-backwards-compat-migration/020-grove-migrate.md",
                "060-backwards-compat-migration/030-migrate-on-adoption.md",
                "070-global-skill-homebrew-distribution.md",
                "080-shed-tui.md",
                "090-shed-inbox-and-install-machinery.md",
                "100-complete-terminate-signal.md",
            ],
        );
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::OldNnn);
        assert_eq!(
            moves_of(&plan),
            vec![
                ("done/010-plan.md".into(), "01-DONE-plan-k1.md".into()),
                (
                    "done/020-loop-substrate-spike.md".into(),
                    "02-DONE-loop-substrate-spike-k2.md".into()
                ),
                (
                    "done/030-substrate-decision.md".into(),
                    "03-DONE-substrate-decision-k3.md".into()
                ),
                (
                    "done/040-substrate-wiring.md".into(),
                    "04-DONE-substrate-wiring-k4.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/BRIEF.md".into(),
                    "05-dotted-decimal-numbering-k5/BRIEF.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/010-id-model.md".into(),
                    "05-dotted-decimal-numbering-k5/01-DONE-id-model-k6.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/020-read-verbs.md".into(),
                    "05-dotted-decimal-numbering-k5/02-DONE-read-verbs-k7.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/030-grow-verbs-renumber.md".into(),
                    "05-dotted-decimal-numbering-k5/03-DONE-grow-verbs-renumber-k8.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/040-lifecycle-verbs.md".into(),
                    "05-dotted-decimal-numbering-k5/04-DONE-lifecycle-verbs-k9.md".into()
                ),
                (
                    "060-backwards-compat-migration/BRIEF.md".into(),
                    "06-backwards-compat-migration-k10/BRIEF.md".into()
                ),
                (
                    "done/060-backwards-compat-migration/010-verbs-live.md".into(),
                    "06-backwards-compat-migration-k10/01-DONE-verbs-live-k11.md".into()
                ),
                (
                    "060-backwards-compat-migration/020-grove-migrate.md".into(),
                    "06-backwards-compat-migration-k10/02-grove-migrate-k12.md".into()
                ),
                (
                    "060-backwards-compat-migration/030-migrate-on-adoption.md".into(),
                    "06-backwards-compat-migration-k10/03-migrate-on-adoption-k13.md".into()
                ),
                (
                    "070-global-skill-homebrew-distribution.md".into(),
                    "07-global-skill-homebrew-distribution-k14.md".into()
                ),
                ("080-shed-tui.md".into(), "08-shed-tui-k15.md".into()),
                (
                    "090-shed-inbox-and-install-machinery.md".into(),
                    "09-shed-inbox-and-install-machinery-k16.md".into()
                ),
                (
                    "100-complete-terminate-signal.md".into(),
                    "10-complete-terminate-signal-k17.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_old_partially_retired_node_merges_live_and_done_children() {
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "010-node/BRIEF.md",
                "010-node/020-live-child.md",
                "done/010-node/010-done-child.md",
            ],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("010-node/BRIEF.md".into(), "01-node-k1/BRIEF.md".into()),
                (
                    "done/010-node/010-done-child.md".into(),
                    "01-node-k1/01-DONE-done-child-k2.md".into()
                ),
                (
                    "010-node/020-live-child.md".into(),
                    "01-node-k1/02-live-child-k3.md".into()
                ),
            ]
        );
    }

    // ---- header rewrite -----------------------------------------------------

    #[test]
    fn rewrite_v1flat_leaf_header_to_position_free_handle() {
        assert_eq!(
            rewrite_header_line(
                "# 11.4-[27]-migrate-v1-to-v2",
                "11.4-[27]-migrate-v1-to-v2",
                "migrate-v1-to-v2-k27"
            ),
            Some("# migrate-v1-to-v2-k27".to_string())
        );
    }

    #[test]
    fn rewrite_v1flat_node_header_preserves_brief_tail() {
        assert_eq!(
            rewrite_header_line(
                "# 11-[23]-scheme-v2-directories — brief",
                "11-[23]-scheme-v2-directories",
                "scheme-v2-directories-k23"
            ),
            Some("# scheme-v2-directories-k23 — brief".to_string())
        );
    }

    #[test]
    fn rewrite_old_nnn_header_to_handle() {
        assert_eq!(
            rewrite_header_line("# 010-plan", "010-plan", "plan-k1"),
            Some("# plan-k1".to_string())
        );
    }

    #[test]
    fn rewrite_leaves_hand_edited_titles_alone() {
        assert_eq!(
            rewrite_header_line("# Some hand title", "1-[1]-plan", "plan-k1"),
            None
        );
        assert_eq!(
            rewrite_header_line("not a header", "1-[1]-plan", "plan-k1"),
            None
        );
    }

    #[test]
    fn rewrite_rejects_partial_token_match() {
        // The token must end at a word boundary, or `…-v2` would clobber `…-v2x`.
        assert_eq!(
            rewrite_header_line(
                "# 11.4-[27]-migrate-v1-to-v2x",
                "11.4-[27]-migrate-v1-to-v2",
                "migrate-v1-to-v2-k27"
            ),
            None
        );
    }

    // ---- execute (renames + header rewrite + cleanup) -----------------------

    /// A `.grove/` inside a real git repo, with `files` written and committed.
    fn git_grove(files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_git(&repo, &["init", "-q"]);
        run_git(&repo, &["config", "user.email", "t@example.com"]);
        run_git(&repo, &["config", "user.name", "Test"]);
        let root = repo.join(".grove");
        for (rel, body) in files {
            let p = root.join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, body).unwrap();
        }
        run_git(&repo, &["add", "-A"]);
        run_git(&repo, &["commit", "-q", "-m", "seed"]);
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

    fn read(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    #[test]
    fn execute_v1flat_moves_files_into_dirs_and_rewrites_headers() {
        let (_t, g) = git_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("1-[1]-plan.DONE.md", "# 1-[1]-plan\n\nbody\n"),
            (
                "5-[5]-node.BRIEF.md",
                "# 5-[5]-node — brief\n\nbrief body\n",
            ),
            ("5.1-[6]-child.md", "# 5.1-[6]-child\n\nchild body\n"),
        ]);
        let worktree = g.parent().unwrap();
        let outcome = migrate(worktree).unwrap();
        assert!(matches!(outcome, Outcome::Migrated(_)));

        // Root brief untouched; leaf retired-in-dir; node became a directory.
        assert_eq!(read(&g.join("BRIEF.md")), "# my-grove — brief\n");
        assert_eq!(
            read(&g.join("01-DONE-plan-k1.md")),
            "# plan-k1\n\nbody\n",
            "header rewritten to the position-free handle; body preserved"
        );
        assert!(g.join("05-node-k5").is_dir());
        assert_eq!(
            read(&g.join("05-node-k5/BRIEF.md")),
            "# node-k5 — brief\n\nbrief body\n"
        );
        assert_eq!(
            read(&g.join("05-node-k5/01-child-k6.md")),
            "# child-k6\n\nchild body\n"
        );
        // The old flat files are gone.
        assert!(!g.join("1-[1]-plan.DONE.md").exists());
        assert!(!g.join("5-[5]-node.BRIEF.md").exists());
        assert!(!g.join("5.1-[6]-child.md").exists());
    }

    #[test]
    fn execute_re_migrate_is_a_clean_no_op() {
        let (_t, g) = git_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("1-[1]-plan.md", "# 1-[1]-plan\n"),
        ]);
        let worktree = g.parent().unwrap();
        assert!(matches!(migrate(worktree).unwrap(), Outcome::Migrated(_)));
        // Second run sees a v2 tree → AlreadyV2, no churn.
        assert!(matches!(migrate(worktree).unwrap(), Outcome::AlreadyV2));
    }

    #[test]
    fn execute_old_nnn_cleans_up_empty_source_dirs() {
        let (_t, g) = git_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("done/010-old.md", "# 010-old\n"),
            ("020-node/BRIEF.md", "# 020-node — brief\n"),
            ("020-node/010-child.md", "# 010-child\n"),
        ]);
        let worktree = g.parent().unwrap();
        assert!(matches!(migrate(worktree).unwrap(), Outcome::Migrated(_)));
        // The `done/` mirror and the old node dir are removed once emptied.
        assert!(!g.join("done").exists(), "done/ swept");
        assert!(!g.join("020-node").exists(), "old node dir swept");
        // The v2 shape is in place.
        assert_eq!(read(&g.join("01-DONE-old-k1.md")), "# old-k1\n");
        assert!(g.join("02-node-k2").is_dir());
        assert_eq!(read(&g.join("02-node-k2/BRIEF.md")), "# node-k2 — brief\n");
        assert_eq!(read(&g.join("02-node-k2/01-child-k3.md")), "# child-k3\n");
    }

    #[test]
    fn migrate_on_adoption_commits_the_conversion() {
        let (_t, g) = git_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("1-[1]-plan.md", "# 1-[1]-plan\n"),
        ]);
        let worktree = g.parent().unwrap();
        assert!(matches!(
            migrate_on_adoption(worktree, "my-grove").unwrap(),
            Outcome::Migrated(_)
        ));
        // The conversion is committed (clean working tree) with a clear message.
        let status = Command::new("git")
            .arg("-C")
            .arg(worktree)
            .args(["status", "--porcelain"])
            .output()
            .unwrap();
        assert!(
            status.stdout.is_empty(),
            "working tree must be clean after adoption commit: {}",
            String::from_utf8_lossy(&status.stdout)
        );
        let log = Command::new("git")
            .arg("-C")
            .arg(worktree)
            .args(["log", "-1", "--format=%s"])
            .output()
            .unwrap();
        assert_eq!(
            String::from_utf8_lossy(&log.stdout).trim(),
            "grove(my-grove): migrate task tree to v2 directory scheme"
        );
        // A second adoption is a clean no-op (no new commit).
        assert!(matches!(
            migrate_on_adoption(worktree, "my-grove").unwrap(),
            Outcome::AlreadyV2
        ));
    }

    #[test]
    fn migrate_missing_grove_is_nothing_to_migrate() {
        let tmp = tempfile::TempDir::new().unwrap();
        assert!(matches!(
            migrate(tmp.path()).unwrap(),
            Outcome::NothingToMigrate
        ));
    }

    // ---- jj-enabled trees (jj-first) ----------------------------------------

    /// Run `bin` in `dir`, asserting success and returning stdout.
    fn run_out(bin: &str, dir: &Path, args: &[&str]) -> String {
        let out = Command::new(bin)
            .current_dir(dir)
            .args(args)
            .output()
            .unwrap_or_else(|e| panic!("running {bin} {args:?}: {e} (is {bin} installed?)"));
        assert!(
            out.status.success(),
            "{bin} {args:?} failed: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8_lossy(&out.stdout).into_owned()
    }

    fn run_jj(dir: &Path, args: &[&str]) -> String {
        let mut full = vec![
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
        ];
        full.extend_from_slice(args);
        run_out("jj", dir, &full)
    }

    /// A `.grove/` inside a jj-native repo (no `.git/`), seeded and committed.
    /// `git.colocate=false` is forced — the ambient jj config may default
    /// colocation on, which would sneak a `.git/` into this fixture.
    fn jj_grove(files: &[(&str, &str)]) -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::TempDir::new().unwrap();
        let repo = tmp.path().to_path_buf();
        run_jj(
            &repo,
            &[
                "--config",
                "git.colocate=false",
                "git",
                "init",
                "--quiet",
                ".",
            ],
        );
        let root = repo.join(".grove");
        for (rel, body) in files {
            let p = root.join(rel);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, body).unwrap();
        }
        run_jj(&repo, &["commit", "-m", "seed"]);
        (tmp, root)
    }

    #[test]
    fn migrate_on_adoption_commits_via_jj_in_a_jj_native_tree() {
        let (_t, g) = jj_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("1-[1]-plan.md", "# 1-[1]-plan\n"),
        ]);
        let worktree = g.parent().unwrap();
        assert!(matches!(
            migrate_on_adoption(worktree, "my-grove").unwrap(),
            Outcome::Migrated(_)
        ));
        // The conversion is a finalized jj commit (@'s parent) with the same
        // message the git path writes, and the v2 shape is on disk.
        let desc = run_jj(
            worktree,
            &["log", "--no-graph", "-r", "@-", "-T", "description"],
        );
        assert_eq!(
            desc.trim(),
            "grove(my-grove): migrate task tree to v2 directory scheme"
        );
        assert!(g.join("01-plan-k1.md").is_file());
        // A second adoption is a clean no-op.
        assert!(matches!(
            migrate_on_adoption(worktree, "my-grove").unwrap(),
            Outcome::AlreadyV2
        ));
    }

    #[test]
    fn migrate_on_adoption_jj_commit_is_scoped_to_grove() {
        // An unrelated dirty file must stay in the working copy, not be swept
        // into the migration commit — the fileset scoping the git path gets
        // from `git add -- .grove`.
        let (_t, g) = jj_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("1-[1]-plan.md", "# 1-[1]-plan\n"),
        ]);
        let worktree = g.parent().unwrap();
        fs::write(worktree.join("stray.txt"), "stray\n").unwrap();
        assert!(matches!(
            migrate_on_adoption(worktree, "my-grove").unwrap(),
            Outcome::Migrated(_)
        ));
        let committed = run_jj(worktree, &["diff", "-r", "@-", "--summary"]);
        assert!(
            !committed.contains("stray.txt"),
            "stray file swept into the migration commit: {committed:?}"
        );
        let working = run_jj(worktree, &["diff", "-r", "@", "--summary"]);
        assert!(
            working.contains("stray.txt"),
            "stray file must remain in the working copy: {working:?}"
        );
    }

    #[test]
    fn migrate_on_adoption_in_colocated_tree_commits_via_jj() {
        // jj-first: `.jj/` beside `.git/` picks the jj path. The commit must be
        // jj-authored — carrying jj's change-id header, so it is a first-class
        // change (`jj op undo`-able), not a git-made commit jj must import —
        // and jj's auto-export keeps the git view clean and in step.
        let (_t, g) = git_grove(&[
            ("BRIEF.md", "# my-grove — brief\n"),
            ("1-[1]-plan.md", "# 1-[1]-plan\n"),
        ]);
        let worktree = g.parent().unwrap();
        run_jj(worktree, &["git", "init", "--colocate", "--quiet", "."]);
        assert!(matches!(
            migrate_on_adoption(worktree, "my-grove").unwrap(),
            Outcome::Migrated(_)
        ));
        assert_eq!(
            run_out("git", worktree, &["log", "-1", "--format=%s"]).trim(),
            "grove(my-grove): migrate task tree to v2 directory scheme"
        );
        assert_eq!(
            run_out("git", worktree, &["status", "--porcelain"]),
            "",
            "jj's export must leave the git view clean"
        );
        let commit_obj = run_out("git", worktree, &["cat-file", "commit", "HEAD"]);
        assert!(
            commit_obj.contains("change-id "),
            "migration commit must be jj-authored (change-id header): {commit_obj:?}"
        );
    }
}

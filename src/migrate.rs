// `grove migrate` — the one-time, in-place conversion of an **old** `NNN-slug/`
// + `done/` directory tree to the **new** flat `<position>-[<key>]-<slug>` scheme
// (ADR-0033 / ADR-0034). Per ADR-0034 this is the *only* surviving reader of the
// old format: once a tree is migrated (here, or by `grove do`'s adoption step in
// 060/030), every tree the live verbs ever see is new-format, so the verbs read
// only the new format and this migration reads the old format exactly once.
//
// The conversion is a **reviewable git change** — `git mv`s plus first-line
// `# …` header rewrites, with **no commit** (the human reviews the diff, then
// commits; adoption in 030 wraps the commit). It preserves:
//   * **order** — the old depth-first, zero-padded-`NNN` walk maps to the new
//     dotted position vectors, so the new version-sort (`leaf_id::sort_key`)
//     reproduces the old `pick` walk;
//   * **done-ness** — a retired leaf under `done/` becomes a `.DONE` file at the
//     position it would have had live (ADR-0033 D3 in-place marker); a retired
//     *node* brief becomes a plain `.BRIEF` (briefs are never done — node
//     done-ness is implicit, D4);
//   * **brief-chain structure** — a node's brief still heads its own subtree
//     under the new position-prefix rule.
//
// **Keys** are assigned fresh in **DFS pre-order** (a node's brief takes its key
// before its children), starting at `1` — deterministic and re-runnable, so the
// fixtures below pin exact output.
//
// The old format it consumes (read-only): a node is a directory `NNN-slug/` with
// a `BRIEF.md` plus child leaves `NNN-slug.md`; a retired entity lives under a
// single top-level `done/` directory whose path **mirrors the entity's live
// path**; the root `BRIEF.md` is the unkeyed singleton. The
// subtle case is a **partially-retired node**: its `BRIEF.md` and some children
// are live while sibling children sit in `done/<node>/…` — migration merges the
// live and `done/` mirrors by logical path into one unified subtree.

use crate::cli::MigrateArgs;
use crate::leaf::split_prefix;
use crate::leaf_id::{self, LeafId};
use crate::repo;
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// What a migration attempt did. Both no-op outcomes are clean (not errors), so
/// adoption (060/030) can call `migrate` unconditionally on every `grove do`.
#[derive(Debug)]
pub enum Outcome {
    /// The tree was old-format and was converted; carries the rename log.
    Migrated(Vec<Rename>),
    /// The tree is already new-format — nothing to do.
    AlreadyNew,
    /// No recognizable old-format tree to migrate (`.grove/` absent, or present
    /// but holding only the root brief / foreign files).
    NothingToMigrate,
}

/// One file moved by the migration: the old path (relative to the grove root,
/// e.g. `done/050-x/010-y.md`) and the new flat filename (e.g.
/// `5.1-[6]-y.DONE.md`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rename {
    pub from_rel: PathBuf,
    pub to_name: String,
}

/// The detected on-disk format of a `.grove/` tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    /// Holds old `NNN-slug` leaves / node directories / a `done/` directory.
    Old,
    /// Holds at least one new-format keyed file (`…-[n]-….md`).
    New,
    /// Neither — only the root brief and/or foreign files; nothing to migrate.
    Empty,
}

/// The computed migration: the detected format and, for an old tree, the ordered
/// list of moves (DFS pre-order, which is also key order). Pure — reads the
/// directory structure but mutates nothing — so the order/key/position mapping is
/// unit-testable without a git repo.
struct Plan {
    format: Format,
    moves: Vec<PlannedMove>,
}

/// A single planned move plus the data needed to rewrite its first-line header.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedMove {
    /// Old path relative to the grove root.
    from_rel: PathBuf,
    /// New flat filename.
    to_name: String,
    /// The old `# …` header token to match, e.g. `010-plan` (the file/dir
    /// `NNN-slug`). The header is rewritten only if its first line is
    /// `# <old_token>…`; the trailing text (e.g. ` — brief`) is preserved.
    old_token: String,
    /// The new header stem replacing the old token, e.g. `1-[1]-plan`.
    new_stem: String,
}

/// One node/leaf in the **unified** old tree (live and `done/` mirrors merged by
/// logical path). `nnn` is the old zero-padded numeric prefix (the level's sort
/// key); `slug` is the text after `NNN-`.
enum Entity {
    /// A leaf `NNN-slug.md`, live or retired (`done`), with its absolute source.
    Leaf {
        nnn: u32,
        slug: String,
        done: bool,
        from_abs: PathBuf,
    },
    /// A node `NNN-slug/`, with its `BRIEF.md` (absent only in a degenerate
    /// brief-less node) and its merged children.
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

/// Migrate the `.grove/` tree under `worktree` old → new in place (no commit).
/// Idempotent: a clean no-op on an already-new tree and on a missing/foreign
/// `.grove/`.
pub fn migrate(worktree: &Path) -> Result<Outcome> {
    let grove_root = worktree.join(".grove");
    if !grove_root.is_dir() {
        return Ok(Outcome::NothingToMigrate);
    }
    let plan = plan(&grove_root)?;
    match plan.format {
        Format::New => Ok(Outcome::AlreadyNew),
        Format::Empty => Ok(Outcome::NothingToMigrate),
        Format::Old => {
            let renames = execute(&grove_root, &plan.moves)?;
            cleanup_empty_dirs(&grove_root)?;
            Ok(Outcome::Migrated(renames))
        }
    }
}

/// Adoption-migrate (060/030, ADR-0034): the step `grove do` runs **before
/// driving** so the loop only ever sees a new-format tree. It detects the format
/// via [`migrate`]; if the tree was old it converts it **and commits** the
/// conversion as one clear, reviewable commit, then driving proceeds new-format.
/// A new-format / empty / absent `.grove/` is a clean no-op — no commit, no churn
/// — so `grove do` calls this unconditionally on every adoption.
///
/// Idempotent and safe under `restart ≡ continuation` (ADR-0032): the conversion
/// itself is re-runnable, and the commit makes the old→new boundary crisp, so a
/// re-run after a completed migrate sees a new-format tree ([`Outcome::AlreadyNew`])
/// and does nothing. This is the **only** place adoption auto-commits to the grove
/// branch — every other commit the loop makes is the agent's own.
pub fn migrate_on_adoption(worktree: &Path, name: &str) -> Result<Outcome> {
    let outcome = migrate(worktree)?;
    if let Outcome::Migrated(_) = &outcome {
        commit_migration(worktree, name)?;
    }
    Ok(outcome)
}

/// Commit the adoption migration as one isolated, self-describing commit. Staging
/// is scoped to `.grove/` so an unrelated dirty file elsewhere in the worktree is
/// never swept in, and a plain `git add` is required because the post-`git mv`
/// header rewrites left each renamed file modified-after-staging.
fn commit_migration(worktree: &Path, name: &str) -> Result<()> {
    git(worktree, &["add", "-A", "--", ".grove"])?;
    let msg = format!("grove({name}): migrate task tree to dotted-decimal scheme");
    git(worktree, &["commit", "-q", "-m", &msg])?;
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

/// The `grove migrate [path]` CLI handler. `path` is the worktree whose `.grove/`
/// to migrate (default: the current worktree). Prints a rename summary.
pub fn run(args: &MigrateArgs) -> Result<()> {
    let worktree = match &args.path {
        Some(p) => p.clone(),
        None => repo::git_toplevel(&std::env::current_dir().context("getting cwd")?)?,
    };
    let grove_root = worktree.join(".grove");
    match migrate(&worktree)? {
        Outcome::Migrated(renames) => {
            println!(
                "migrated {} file{} old → new format:",
                renames.len(),
                if renames.len() == 1 { "" } else { "s" }
            );
            for r in &renames {
                println!("  {} -> {}", r.from_rel.display(), r.to_name);
            }
            println!("review the working-tree changes, then commit.");
        }
        Outcome::AlreadyNew => {
            eprintln!("grove migrate: already new-format; nothing to do");
        }
        Outcome::NothingToMigrate => {
            if grove_root.is_dir() {
                eprintln!(
                    "grove migrate: no old-format tree to migrate in {}",
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
// planning (pure: reads the directory structure, mutates nothing)

/// Detect the format and, for an old tree, compute the ordered move list.
fn plan(grove_root: &Path) -> Result<Plan> {
    let format = detect(grove_root)?;
    let moves = if format == Format::Old {
        let forest = build_forest(grove_root)?;
        let mut moves = Vec::new();
        let mut next_key = 1u32;
        assign(grove_root, &forest, &[], &mut next_key, &mut moves)?;
        moves
    } else {
        Vec::new()
    };
    Ok(Plan { format, moves })
}

/// Classify a `.grove/` tree by scanning its top level (see [`Format`]). Any
/// new-format keyed file wins (a tree is never migrated twice); else any
/// old-format marker (a `done/` dir, an `NNN-slug/` node dir, or an `NNN-slug.md`
/// leaf) means old; else there is nothing to migrate.
fn detect(grove_root: &Path) -> Result<Format> {
    let mut has_new = false;
    let mut has_old = false;
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let ft = entry.file_type()?;
        if ft.is_dir() {
            if name == "done" || split_prefix(&name).is_some() {
                has_old = true;
            }
        } else if ft.is_file() {
            if name == "BRIEF.md" {
                continue; // the unkeyed root brief is format-neutral
            }
            if leaf_id::parse(&name).is_some() {
                has_new = true; // a new-format keyed file (leaf or brief, live or .DONE)
            } else if name.strip_suffix(".md").and_then(split_prefix).is_some() {
                has_old = true; // an old `NNN-slug.md` leaf
            }
        }
    }
    Ok(if has_new {
        Format::New
    } else if has_old {
        Format::Old
    } else {
        Format::Empty
    })
}

/// Build the unified old tree: the live tree and the `done/` mirror merged by
/// logical path, root level first.
fn build_forest(grove_root: &Path) -> Result<Vec<Entity>> {
    collect_level(grove_root, Path::new(""))
}

/// Collect one logical level of the unified tree. `chain` is the path of
/// `NNN-slug` directory components from the grove root to this level (empty at
/// the root). The level's children are read from both its live directory
/// (`grove_root/<chain>`) and its `done/` mirror (`grove_root/done/<chain>`) and
/// merged by `NNN-slug` name, so a partially-retired node's live and retired
/// children land in one subtree. Returns the level's entities sorted by `NNN`.
fn collect_level(grove_root: &Path, chain: &Path) -> Result<Vec<Entity>> {
    let live_dir = if chain.as_os_str().is_empty() {
        grove_root.to_path_buf()
    } else {
        grove_root.join(chain)
    };
    let done_dir = grove_root.join("done").join(chain);

    // Per-level accumulators keyed by the full `NNN-slug` name, so a node present
    // in both the live tree and the `done/` mirror merges into one entry.
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
                // a non-`NNN` directory is foreign — ignored
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
                // a non-`NNN` / non-`.md` file is foreign — ignored
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

/// Walk the unified tree in DFS pre-order, assigning each entity its new dotted
/// position (the 1-based index among its sorted siblings, appended to the
/// parent's position) and the next fresh key (a node's brief takes its key before
/// its children). Emits one [`PlannedMove`] per file that exists; a brief-less
/// node consumes a position slot but no key (there is no file to name).
fn assign(
    grove_root: &Path,
    entities: &[Entity],
    parent_pos: &[u32],
    next_key: &mut u32,
    out: &mut Vec<PlannedMove>,
) -> Result<()> {
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
                let id = LeafId {
                    position: pos.clone(),
                    key: Some(key),
                    slug: slug.clone(),
                    is_brief: false,
                    is_done: *done,
                };
                out.push(make_move(grove_root, from_abs, &id, *nnn, slug)?);
            }
            Entity::Node {
                nnn,
                slug,
                brief_abs,
                children,
            } => {
                if let Some(brief_abs) = brief_abs {
                    let key = take_key(next_key);
                    let id = LeafId {
                        position: pos.clone(),
                        key: Some(key),
                        slug: slug.clone(),
                        is_brief: true,
                        is_done: false,
                    };
                    out.push(make_move(grove_root, brief_abs, &id, *nnn, slug)?);
                }
                assign(grove_root, children, &pos, next_key, out)?;
            }
        }
    }
    Ok(())
}

/// Consume and return the next permanent key.
fn take_key(next_key: &mut u32) -> u32 {
    let key = *next_key;
    *next_key += 1;
    key
}

/// Build a [`PlannedMove`] for one entity: its old grove-root-relative path, its
/// new flat filename, and the header-rewrite tokens (old `NNN-slug`, new
/// `<pos>-[<key>]-<slug>` stem).
fn make_move(
    grove_root: &Path,
    from_abs: &Path,
    id: &LeafId,
    nnn: u32,
    slug: &str,
) -> Result<PlannedMove> {
    let from_rel = from_abs
        .strip_prefix(grove_root)
        .with_context(|| {
            format!(
                "{} is not under grove root {}",
                from_abs.display(),
                grove_root.display()
            )
        })?
        .to_path_buf();
    Ok(PlannedMove {
        from_rel,
        to_name: id.filename(),
        old_token: format!("{:03}-{}", nnn, slug),
        new_stem: header_stem(id),
    })
}

/// The new first-line header stem for an id: `<dotted-position>-[<key>]-<slug>`
/// (no marker — headers never carry `.BRIEF`/`.DONE`).
fn header_stem(id: &LeafId) -> String {
    let dotted = id
        .position
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".");
    format!(
        "{}-[{}]-{}",
        dotted,
        id.key.expect("planned moves are always keyed"),
        id.slug
    )
}

/// Rewrite a file's first-line `# …` header for the move. Returns the new header
/// line, or `None` to leave the file untouched. The first line must be
/// `# <old_token>…` with the token at a word boundary; the leading token is
/// replaced with `new_stem` and any trailing text (e.g. ` — brief`) is preserved.
/// A hand-edited title that does not start with the old token — or where the
/// token is only a partial prefix of a longer one — is left alone (conservative).
fn rewrite_header_line(first_line: &str, old_token: &str, new_stem: &str) -> Option<String> {
    let after_hash = first_line.strip_prefix("# ")?;
    let tail = after_hash.strip_prefix(old_token)?;
    // Reject a partial match: the next char must not continue an `NNN-slug` token
    // (slug chars are `[a-z0-9-]`), or `010-plan` would mis-match `010-planted`.
    if tail
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return None;
    }
    Some(format!("# {new_stem}{tail}"))
}

// ---------------------------------------------------------------------------
// execution (impure: git mv + header rewrites)

/// Apply each planned move: `git mv` the file to its new flat name, then rewrite
/// its first-line header. Aborts (leaving the partial change for the human to
/// reset) if a destination already exists. No commit.
fn execute(grove_root: &Path, moves: &[PlannedMove]) -> Result<Vec<Rename>> {
    let mut renames = Vec::with_capacity(moves.len());
    for m in moves {
        let dst_abs = grove_root.join(&m.to_name);
        if dst_abs.exists() {
            bail!(
                "destination already exists: {} (migration aborted)",
                dst_abs.display()
            );
        }
        git_mv(grove_root, &m.from_rel, Path::new(&m.to_name))?;
        rewrite_header_file(&dst_abs, &m.old_token, &m.new_stem)?;
        renames.push(Rename {
            from_rel: m.from_rel.clone(),
            to_name: m.to_name.clone(),
        });
    }
    Ok(renames)
}

/// `git -C <grove_root> mv <from_rel> <to_rel>` — a tracked move so the human's
/// review and the enclosing commit fold it in cleanly.
fn git_mv(grove_root: &Path, from_rel: &Path, to_rel: &Path) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(grove_root)
        .arg("mv")
        .arg(from_rel)
        .arg(to_rel)
        .output()
        .with_context(|| format!("running git mv {} {}", from_rel.display(), to_rel.display()))?;
    if !out.status.success() {
        bail!(
            "git mv {} -> {} failed: {}",
            from_rel.display(),
            to_rel.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// Rewrite the first line of a moved file via [`rewrite_header_line`], preserving
/// the rest of the body byte-for-byte. A non-matching first line is left alone.
fn rewrite_header_file(path: &Path, old_token: &str, new_stem: &str) -> Result<()> {
    let body = fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let Some(new_first) = rewrite_header_line(first, old_token, new_stem) else {
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

/// Remove directories left empty by the migration (the old `NNN-slug/` node dirs
/// and the whole `done/` mirror), bottom-up. Only *empty* directories are
/// removed, so a directory still holding a foreign file survives — foreign
/// content is never disturbed. Git does not track directories, so a plain
/// `remove_dir` suffices (no `git rm`).
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

    /// Lay down a set of grove-root-relative files (empty bodies — `plan` reads
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

    /// The plan's moves as `(from_rel, to_name)` string pairs, in plan order.
    fn moves_of(plan: &Plan) -> Vec<(String, String)> {
        plan.moves
            .iter()
            .map(|m| (m.from_rel.to_string_lossy().into_owned(), m.to_name.clone()))
            .collect()
    }

    // ---- detect -------------------------------------------------------------

    #[test]
    fn detect_old_tree_by_nnn_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::Old);
    }

    #[test]
    fn detect_old_tree_by_done_dir() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "done/010-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::Old);
    }

    #[test]
    fn detect_old_tree_by_node_dir() {
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "010-node/BRIEF.md", "010-node/010-leaf.md"],
        );
        assert_eq!(detect(&g).unwrap(), Format::Old);
    }

    #[test]
    fn detect_new_tree_by_keyed_file() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::New);
    }

    #[test]
    fn detect_new_tree_with_done_marker_file() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-plan.DONE.md"]);
        assert_eq!(detect(&g).unwrap(), Format::New);
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

    // ---- plan: golden tree (mirrors this grove's real shape) ----------------

    #[test]
    fn plan_golden_tree_full_shape() {
        // The exact shape of the `refactor-to-archon` grove at 060/020: retired
        // root-level leaves, a fully-retired node (050, brief in done/), a
        // partially-retired node (060, brief live + one done child + two live
        // children), and trailing live leaves through 100.
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
        assert_eq!(plan.format, Format::Old);
        assert_eq!(
            moves_of(&plan),
            vec![
                ("done/010-plan.md".into(), "1-[1]-plan.DONE.md".into()),
                (
                    "done/020-loop-substrate-spike.md".into(),
                    "2-[2]-loop-substrate-spike.DONE.md".into()
                ),
                (
                    "done/030-substrate-decision.md".into(),
                    "3-[3]-substrate-decision.DONE.md".into()
                ),
                (
                    "done/040-substrate-wiring.md".into(),
                    "4-[4]-substrate-wiring.DONE.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/BRIEF.md".into(),
                    "5-[5]-dotted-decimal-numbering.BRIEF.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/010-id-model.md".into(),
                    "5.1-[6]-id-model.DONE.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/020-read-verbs.md".into(),
                    "5.2-[7]-read-verbs.DONE.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/030-grow-verbs-renumber.md".into(),
                    "5.3-[8]-grow-verbs-renumber.DONE.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/040-lifecycle-verbs.md".into(),
                    "5.4-[9]-lifecycle-verbs.DONE.md".into()
                ),
                (
                    "060-backwards-compat-migration/BRIEF.md".into(),
                    "6-[10]-backwards-compat-migration.BRIEF.md".into()
                ),
                (
                    "done/060-backwards-compat-migration/010-verbs-live.md".into(),
                    "6.1-[11]-verbs-live.DONE.md".into()
                ),
                (
                    "060-backwards-compat-migration/020-grove-migrate.md".into(),
                    "6.2-[12]-grove-migrate.md".into()
                ),
                (
                    "060-backwards-compat-migration/030-migrate-on-adoption.md".into(),
                    "6.3-[13]-migrate-on-adoption.md".into()
                ),
                (
                    "070-global-skill-homebrew-distribution.md".into(),
                    "7-[14]-global-skill-homebrew-distribution.md".into()
                ),
                ("080-shed-tui.md".into(), "8-[15]-shed-tui.md".into()),
                (
                    "090-shed-inbox-and-install-machinery.md".into(),
                    "9-[16]-shed-inbox-and-install-machinery.md".into()
                ),
                (
                    "100-complete-terminate-signal.md".into(),
                    "10-[17]-complete-terminate-signal.md".into()
                ),
            ]
        );
    }

    // ---- plan: focused cases ------------------------------------------------

    #[test]
    fn plan_root_brief_stays_unkeyed_and_is_not_moved() {
        // The root brief is the one unkeyed singleton; it is never in the plan.
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-a.md"]);
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
    fn plan_live_only_root_leaves_get_sequential_positions_and_keys() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-a.md", "020-b.md", "030-c.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("010-a.md".into(), "1-[1]-a.md".into()),
                ("020-b.md".into(), "2-[2]-b.md".into()),
                ("030-c.md".into(), "3-[3]-c.md".into()),
            ]
        );
    }

    #[test]
    fn plan_done_leaf_gets_done_marker_at_its_live_position() {
        // A retired root-level leaf keeps the position it would have had live.
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-a.md", "done/020-b.md", "030-c.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("010-a.md".into(), "1-[1]-a.md".into()),
                ("done/020-b.md".into(), "2-[2]-b.DONE.md".into()),
                ("030-c.md".into(), "3-[3]-c.md".into()),
            ]
        );
    }

    #[test]
    fn plan_node_brief_is_never_marked_done_even_when_fully_retired() {
        // A fully-retired node's brief lives in done/, but a brief is never
        // `.DONE` — node done-ness is implicit (all children `.DONE`).
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "done/010-node/BRIEF.md",
                "done/010-node/010-child.md",
            ],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                (
                    "done/010-node/BRIEF.md".into(),
                    "1-[1]-node.BRIEF.md".into()
                ),
                (
                    "done/010-node/010-child.md".into(),
                    "1.1-[2]-child.DONE.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_partially_retired_node_merges_live_and_done_children() {
        // Node 010 is live (brief + 020 live) with one retired child (010 in
        // done/) — the merged child order is 010, 020 → 1.1, 1.2.
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
                ("010-node/BRIEF.md".into(), "1-[1]-node.BRIEF.md".into()),
                (
                    "done/010-node/010-done-child.md".into(),
                    "1.1-[2]-done-child.DONE.md".into()
                ),
                (
                    "010-node/020-live-child.md".into(),
                    "1.2-[3]-live-child.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_orders_numerically_so_new_sort_reproduces_old_walk() {
        // A node with 10 children (010..100): the old zero-padded walk visits
        // them 010,020,…,100 → new positions 1.1..1.10. The new comparator must
        // sort 1.9 < 1.10 numerically (the exact case lexical sort fails).
        let (_t, g) = grove();
        let mut files = vec!["BRIEF.md".to_string(), "010-node/BRIEF.md".to_string()];
        for i in 1..=10 {
            files.push(format!("010-node/{:03}-c{}.md", i * 10, i));
        }
        let files_ref: Vec<&str> = files.iter().map(String::as_str).collect();
        build(&g, &files_ref);

        let plan = plan(&g).unwrap();
        // The 10th child must be position 1.10 with key 11 (brief is [1]).
        let tenth = plan
            .moves
            .iter()
            .find(|m| m.from_rel == Path::new("010-node/100-c10.md"))
            .expect("child c10 planned");
        assert_eq!(tenth.to_name, "1.10-[11]-c10.md");

        // The planned new names, sorted by the new comparator, reproduce the old
        // DFS pre-order (= plan order).
        let plan_order: Vec<&str> = plan.moves.iter().map(|m| m.to_name.as_str()).collect();
        let mut sorted = plan_order.clone();
        sorted.sort_by_key(|a| leaf_id::sort_key(a));
        assert_eq!(
            plan_order, sorted,
            "new version-sort must match old walk order"
        );
    }

    #[test]
    fn plan_foreign_files_are_not_moved() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-a.md", "README.md", "notes.txt"]);
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
    fn plan_already_new_tree_has_no_moves() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "2-[2]-b.DONE.md"]);
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::New);
        assert!(plan.moves.is_empty());
    }

    // ---- header rewrite -----------------------------------------------------

    #[test]
    fn rewrite_header_leaf_replaces_token_no_tail() {
        assert_eq!(
            rewrite_header_line("# 010-plan", "010-plan", "1-[1]-plan"),
            Some("# 1-[1]-plan".to_string())
        );
    }

    #[test]
    fn rewrite_header_brief_preserves_em_dash_brief_tail() {
        assert_eq!(
            rewrite_header_line(
                "# 060-backwards-compat-migration — brief",
                "060-backwards-compat-migration",
                "6-[10]-backwards-compat-migration"
            ),
            Some("# 6-[10]-backwards-compat-migration — brief".to_string())
        );
    }

    #[test]
    fn rewrite_header_leaves_hand_edited_titles_alone() {
        // A first line that does not begin with the old token is preserved.
        assert_eq!(
            rewrite_header_line("# Some hand title", "010-plan", "1-[1]-plan"),
            None
        );
        assert_eq!(
            rewrite_header_line("not a header", "010-plan", "1-[1]-plan"),
            None
        );
    }
}

// The **v2 read verbs** (task-tree-scheme) — `pick`, `brief-chain`, `resolve` —
// re-expressed against the real **directory tree**, built on the 11.1 id model
// (`src/tree_id.rs`). Keeps task-tree-scheme's *semantics* (first live leaf in DFS
// pre-order; ancestor briefs root→leaf; reference-by-permanent-key) and changes
// only the **walk**: in v1 the whole tree was encoded into flat filenames, so a
// verb was a single `read_dir` + version-sort over `.grove/`; in v2 the
// filesystem carries the hierarchy (a node is a *directory* holding its children,
// and optionally a `BRIEF.md` charter — a chain node carries none, which is why
// `brief-chain`'s tolerance of a missing level is load-bearing rather than
// incidental), so the same verbs become filesystem-shape walks:
//
//   * `pick`        — recursive depth-first pre-order over node dirs;
//   * `brief-chain` — the leaf's ancestor *directories* root→leaf, each `BRIEF.md`;
//   * `resolve`     — a recursive collect-the-tree, then match by key/slug.
//
// Built **isolated**, mirroring `tree_id`: this module does NOT touch the live v1
// verb path (`leaf_read` and the `llm_cli` dispatch to it), which keeps speaking
// the flat scheme until the user-gated re-flip (11.6 — mirrors 070/040) wires
// these verbs in and sweeps the v1 modules. So no leaf in this node can break the
// v1-flat grove that is driving itself. The `Resolution` types and
// `render_resolution` are deliberately re-defined here (not reused from
// `leaf_read`) to keep the v2 surface self-contained for that later swap.
//
// `resolve`'s **reference grammar** keeps v1's `[n]` / `n` / `[n]-slug` / bare-slug
// forms verbatim, and 11.5 adds exactly one form: the full `<slug>-k<key>` handle
// that task-tree-scheme §5 makes canonical for commits and prose. The deferred question
// 11.2 left for here ("should resolve also accept the handle, now that the handle
// is established?") is **resolved yes**: `handle_key` peels the handle's terminal
// `-k<key>`, and the slug branch falls back to it only when no bare slug matched —
// so every v1 reference still resolves identically (a literal slug ending in
// `-k<digits>` is matched as a slug first), and the §5 handle round-trips to a path.

use crate::harness::Harness;
use crate::leaf::Kind;
use crate::tree_id::{parse, sort_key, Entry, Outcome};
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// `pick`: a recursive depth-first **pre-order** walk over the directory tree,
/// returning the first **live leaf**. Within each directory the children are
/// visited in per-level order (the charter brief first — and skipped — then by
/// numeric position): a live leaf returns immediately; a node directory is
/// descended in place (pre-order, so a node at an earlier position is fully
/// explored before a later sibling leaf); a `DONE` leaf, an `ABANDONED` leaf
/// (ADR *pruning*), the brief, and foreign names are skipped. `Ok(None)` means no
/// live leaf anywhere — the loop's finish signal (the CLI renders it as empty
/// stdout + a "no live leaves" stderr diagnostic). Lenient on foreign/malformed
/// names (a stray `README.md` never jams the loop). Never reads file contents.
pub fn pick(grove_root: &Path) -> Result<Option<PathBuf>> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    pick_in(grove_root)
}

/// The recursive heart of [`pick`]: the first live leaf in pre-order under `dir`,
/// descending node directories as they are met.
fn pick_in(dir: &Path) -> Result<Option<PathBuf>> {
    for (entry, path) in read_level(dir)? {
        match entry {
            // The first live leaf in pre-order is the answer.
            Entry::Leaf {
                outcome: Outcome::Live,
                ..
            } => return Ok(Some(path)),
            // Descend a node in place; its first live leaf (if any) wins before
            // any later sibling at this level is considered.
            Entry::Node { .. } => {
                if let Some(found) = pick_in(&path)? {
                    return Ok(Some(found));
                }
            }
            // The charter brief and terminal (`DONE` / `ABANDONED`) leaves are
            // skipped — a grove whose only remaining leaves are abandoned reports
            // "no live leaves; this grove is done", correctly: the work is
            // settled, however it settled (ADR *pruning*).
            Entry::Brief
            | Entry::Leaf {
                outcome: Outcome::Done | Outcome::Abandoned,
                ..
            } => {}
        }
    }
    Ok(None)
}

/// `brief-chain`: the `BRIEF.md` of each of the leaf's **ancestor directories**,
/// from the grove root down to the leaf's containing directory, in root→leaf
/// order. The headline behavioural shift from v1: ancestor briefs are found by
/// **directory ascent** (the filesystem carries the hierarchy now), not by
/// id-prefix in a flat namespace. A directory level with no `BRIEF.md` is skipped
/// silently (some nodes are mid-decomposition); a leaf has no brief of its own,
/// so its own directory's brief is the deepest one collected. `leaf_path` is
/// absolute or relative to `grove_root`, and must resolve to a path under it.
pub fn brief_chain(grove_root: &Path, leaf_path: &Path) -> Result<Vec<PathBuf>> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let grove_root = grove_root
        .canonicalize()
        .with_context(|| format!("canonicalising grove root {}", grove_root.display()))?;

    let candidate = if leaf_path.is_absolute() {
        leaf_path.to_path_buf()
    } else {
        grove_root.join(leaf_path)
    };
    let leaf_abs = candidate
        .canonicalize()
        .with_context(|| format!("resolving leaf path {}", candidate.display()))?;
    if !leaf_abs.starts_with(&grove_root) {
        bail!(
            "leaf path {} is not under grove root {}",
            leaf_abs.display(),
            grove_root.display()
        );
    }
    if leaf_abs == grove_root {
        bail!(
            "leaf path {} is the grove root, not a leaf",
            leaf_abs.display()
        );
    }

    // The leaf's ancestor directories: its containing directory, then up to (and
    // including) the grove root. The leaf lives strictly under the root (checked
    // above), so the ascent always reaches it.
    let mut dirs = Vec::new();
    let mut cur = leaf_abs
        .parent()
        .with_context(|| format!("leaf path {} has no parent", leaf_abs.display()))?
        .to_path_buf();
    loop {
        dirs.push(cur.clone());
        if cur == grove_root {
            break;
        }
        match cur.parent() {
            Some(p) => cur = p.to_path_buf(),
            None => break,
        }
    }
    dirs.reverse(); // root→leaf

    let mut chain = Vec::new();
    for d in dirs {
        let brief = d.join("BRIEF.md");
        if brief.is_file() {
            chain.push(brief);
        }
    }
    Ok(chain)
}

/// `kind [<leaf>]`: the task's kind — one of the closed seventeen
/// (task-kind-taxonomy) — the loop driver keys model selection on
/// (model-per-task-kind). With `leaf_path = Some`, read that leaf's
/// `**Kind:**` line; with `None`, default to [`pick`]'s next live leaf and
/// return `Ok(None)` on an empty grove — the same "no live leaves" signal `pick`
/// gives (the CLI renders it as the standard stderr diagnostic, mirroring
/// `brief-chain`). `leaf_path` is absolute or relative to `grove_root`. Reading
/// goes through [`read_kind`], which **degrades** rather than errors on a
/// missing or unrecognised `**Kind:**` line (warns to stderr, treated as
/// `impl`) — so a hand-edited or foreign task file can never jam the
/// self-driving loop.
pub fn kind(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<Kind>> {
    match target_leaf(grove_root, leaf_path)? {
        Some(leaf) => read_kind(&leaf).map(Some),
        None => Ok(None),
    }
}

/// Both facts the loop driver needs before it can launch a session: the leaf's
/// kind, and the harness that leaf *declares* for itself if it declares one
/// (`leaf-harness-k15`). `Ok(None)` is the empty-grove signal, exactly as
/// [`kind`] gives it.
///
/// One function, and one leaf resolution, because the driver peeks **once**: the
/// peek is a subprocess on every iteration of the loop (`model-per-task-kind`,
/// *Consequences*), and a second verb call to read a line out of the same file
/// would double that cost for a declaration almost no leaf carries.
///
/// The two fields degrade differently on purpose, which is why this returns a
/// pair rather than one fallible whole: `read_kind` never fails on content, so a
/// garbled kind still yields a launch, while [`read_harness`] **refuses** — an
/// unrecognised harness name aborts the peek rather than resolving to anything.
pub fn launch_peek(
    grove_root: &Path,
    leaf_path: Option<&Path>,
) -> Result<Option<(Kind, Option<&'static Harness>)>> {
    let Some(leaf) = target_leaf(grove_root, leaf_path)? else {
        return Ok(None);
    };
    let kind = read_kind(&leaf)?;
    let harness = read_harness(&leaf)?;
    Ok(Some((kind, harness)))
}

/// The leaf a leaf-reading verb operates on: the given path (absolute, or
/// relative to `grove_root`), else [`pick`]'s next live leaf. `Ok(None)` is the
/// empty grove — the same "no live leaves" signal `pick` gives, which the CLI
/// renders as the standard stderr diagnostic.
fn target_leaf(grove_root: &Path, leaf_path: Option<&Path>) -> Result<Option<PathBuf>> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    Ok(match leaf_path {
        Some(p) if p.is_absolute() => Some(p.to_path_buf()),
        Some(p) => Some(grove_root.join(p)),
        None => pick(grove_root)?,
    })
}

/// Read a leaf task file's declared launch harness from its `**Harness:** <name>`
/// line (`content/TASK-FORMAT.md`) — the per-leaf routing axis, which exists for
/// the vendor pair a kind→harness function cannot express
/// (`docs/specs/task-kind-taxonomy.md`, *Routing*). Parsed exactly like
/// [`read_kind`]: first line carrying the marker, first whitespace token after
/// it, trailing commentary tolerated.
///
/// **No line is the answer `None`, not a failure.** Almost no leaf declares a
/// harness — it is two leaves of a research pair and nothing else — so the
/// undeclared path must stay free.
///
/// **Read refuses, where [`read_kind`] degrades.** A token that is not a known
/// harness — and an empty `**Harness:**` line, which is a declaration the author
/// did not finish — errors, naming the file and the registry. This is a
/// deliberate departure from the kind axis, on the difference between the two
/// mistakes: a wrong *discipline label* costs a warning and a model chosen for
/// the wrong bucket, while a wrong *harness* would run the leaf on a vendor the
/// tree explicitly said not to — the same silent misroute `loop_driver` already
/// bails on when the kind peek itself fails. Refusing to honour a declaration
/// grove cannot execute is not grove gating on *process* grounds (constraint 5),
/// which is what that constraint forbids.
pub(crate) fn read_harness(leaf_path: &Path) -> Result<Option<&'static Harness>> {
    let text = fs::read_to_string(leaf_path)
        .with_context(|| format!("reading task file {}", leaf_path.display()))?;
    for line in text.lines() {
        let Some(rest) = line.trim_start().strip_prefix("**Harness:**") else {
            continue;
        };
        let Some(token) = rest.split_whitespace().next() else {
            bail!(
                "task file {} has an empty `**Harness:**` line. Name a harness ({}) \
                 or delete the line to run this leaf on the grove's own harness.",
                leaf_path.display(),
                crate::harness::known_names()
            );
        };
        return crate::harness::by_name(token).map(Some).ok_or_else(|| {
            anyhow::anyhow!(
                "task file {} declares `**Harness:** {}`, which is not a known harness. \
                 Known: {}.",
                leaf_path.display(),
                token,
                crate::harness::known_names()
            )
        });
    }
    Ok(None)
}

/// Read a leaf task file's declared kind from its `**Kind:** <kind>` line
/// (`content/TASK-FORMAT.md`). Takes the first line that begins with the
/// `**Kind:**` marker and parses the first whitespace token after it through
/// [`Kind::parse_read`] — so trailing commentary (`**Kind:** impl   (or:
/// planning)`) is tolerated.
///
/// **Read degrades** (task-kind-taxonomy, the counterpart to [`Kind::parse`]'s
/// write-side gate): a missing `**Kind:**` line, an empty one, or a token
/// [`Kind::parse_read`] does not recognise — hand-edited, or written by a future
/// grove version — warns to stderr, naming the file, and is treated as
/// `Kind::Impl`. This function never errors on the content of the line; only
/// a genuine I/O failure (the file cannot be read at all) still returns `Err`.
/// Reading must degrade because the self-driving loop relaunches unattended: a
/// typo in a task file must never jam it (constraint 5). `leaf_decompose`
/// reuses this to inherit a leaf's kind, so the same degrade applies there.
///
/// `**Kind:** work` is **not** a degrade: [`Kind::parse_read`] resolves the
/// previous spelling to `impl` silently, so the six live groves carrying it read
/// correctly and print no warning. Warning there would fire on every task file
/// of every live grove and train the operator to ignore this diagnostic.
pub(crate) fn read_kind(leaf_path: &Path) -> Result<Kind> {
    let text = fs::read_to_string(leaf_path)
        .with_context(|| format!("reading task file {}", leaf_path.display()))?;
    for line in text.lines() {
        let Some(rest) = line.trim_start().strip_prefix("**Kind:**") else {
            continue;
        };
        let Some(token) = rest.split_whitespace().next() else {
            eprintln!(
                "grove: task file {} has an empty `**Kind:**` line; treating as `impl`",
                leaf_path.display()
            );
            return Ok(Kind::Impl);
        };
        return Ok(match Kind::parse_read(token) {
            Some(k) => k,
            None => {
                eprintln!(
                    "grove: task file {} has an unrecognised `**Kind:**` token {:?}; \
                     treating as `impl`",
                    leaf_path.display(),
                    token
                );
                Kind::Impl
            }
        });
    }
    eprintln!(
        "grove: task file {} has no `**Kind:**` line; treating as `impl`",
        leaf_path.display()
    );
    Ok(Kind::Impl)
}

/// The outcome of resolving a reference. The CLI maps this to stdout/stderr via
/// [`render_resolution`]; the split keeps the I/O contract unit-testable without
/// a live verb dispatch. (Re-defined here, not reused from `leaf_read`, to keep
/// the v2 surface self-contained for the 11.6 swap.)
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Exactly one entry matched. `outcome` is the matched leaf's own
    /// live/`DONE`/`ABANDONED` state (ADR *pruning*) — so an abandoned match
    /// is distinguishable from both a live and a `DONE` one, not folded into a
    /// single `retired` bit. A matched node reports `Outcome::Live`: a node
    /// carries no terminal state of its own, its done-ness is the absence of a
    /// live leaf in its subtree.
    Found { path: PathBuf, outcome: Outcome },
    /// No entry matched the reference (pick-style: not an error).
    NotFound,
    /// A bare-slug reference matched more than one entry. Each carries its
    /// permanent key so the caller re-queries by the unambiguous key.
    Ambiguous(Vec<AmbiguousMatch>),
}

/// One entry of a [`Resolution::Ambiguous`] result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmbiguousMatch {
    pub key: u32,
    pub path: PathBuf,
    pub outcome: Outcome,
}

/// The [`Outcome`] `resolve` reports for a matched entry: a leaf's own
/// live/`DONE`/`ABANDONED` state, or `Outcome::Live` for a node (a node has no
/// terminal state of its own — see [`Resolution::Found`]).
fn entry_outcome(e: &Entry) -> Outcome {
    match e {
        Entry::Leaf { outcome, .. } => *outcome,
        Entry::Node { .. } | Entry::Brief => Outcome::Live,
    }
}

/// `resolve <ref>`: turn a reference into the current path of the entity it
/// names, searching the **whole directory tree** — live leaves, `DONE` leaves,
/// and node directories alike.
///   * `[n]` / `n` → the unique entity whose permanent key is `n` (keys are
///     unique tree-wide → 0 or 1 match). The primary durable-reference path.
///     A node resolves to its **directory** path (the dir name carries the key);
///     append `/BRIEF.md` to read its charter.
///   * `[n]-slug` → same; the slug part is decorative (ignored).
///   * bare slug → resolve by slug: 0 ⇒ `NotFound`; 1 ⇒ `Found`; >1 ⇒
///     `Ambiguous` (every match with its key, so the caller re-queries by key).
///   * `<slug>-k<key>` → the full canonical handle (task-tree-scheme §5): same as `[n]`
///     via its terminal `-k<key>`, the slug decorative. Tried only after the bare
///     slug fails to match, so a literal slug ending in `-k<digits>` still wins.
///
/// The root brief (`BRIEF.md`, the one unkeyed singleton) is unreferenceable.
pub fn resolve(grove_root: &Path, reference: &str) -> Result<Resolution> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
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

/// A parsed reference: a permanent key, or a bare slug.
enum Ref {
    Key(u32),
    Slug(String),
}

/// Classify a `resolve` reference (held identical to v1 — see the module header).
/// `[n]` / `[n]-slug` and a bare integer `n` resolve by key; anything else is a
/// bare slug. A bracketed-but-malformed key (`[abc]`, `[4`) is a reference error
/// (distinct from a valid-but-unmatched reference, which is `NotFound`).
fn parse_ref(reference: &str) -> Result<Ref> {
    if let Some(rest) = reference.strip_prefix('[') {
        let close = rest
            .find(']')
            .with_context(|| format!("reference {reference:?}: unclosed '['"))?;
        let key: u32 = rest[..close]
            .parse()
            .with_context(|| format!("reference {reference:?}: '[…]' is not an integer key"))?;
        // Anything after `]` (e.g. `-slug`) is decorative.
        Ok(Ref::Key(key))
    } else if !reference.is_empty() && reference.bytes().all(|b| b.is_ascii_digit()) {
        let key: u32 = reference
            .parse()
            .with_context(|| format!("reference {reference:?}: not an integer key"))?;
        Ok(Ref::Key(key))
    } else {
        Ok(Ref::Slug(reference.to_string()))
    }
}

/// Peel the terminal `-k<digits>` of a full `<slug>-k<key>` handle (task-tree-scheme §5's
/// canonical commit/prose reference) into its key, or `None` when the reference is
/// not handle-shaped. Mirrors the filename grammar's "the key is the terminal
/// `-k<digits>`" rule (`src/tree_id.rs`), so a handle and a filename peel the key
/// identically — `migrate-v1-to-v2-k27` → `27`, `build` → `None`.
fn handle_key(reference: &str) -> Option<u32> {
    let digits_start = reference.len()
        - reference
            .bytes()
            .rev()
            .take_while(|b| b.is_ascii_digit())
            .count();
    if digits_start == reference.len() {
        return None; // no trailing digits → not a handle
    }
    reference[..digits_start].strip_suffix("-k")?;
    reference[digits_start..].parse().ok()
}

/// Render a [`Resolution`] to the `(stdout, stderr)` the `resolve` verb emits
/// once wired in (11.6). Kept pure and separate from the I/O so the exact
/// stdout/stderr contract is unit-testable while the live CLI dispatch is
/// untouched.
pub fn render_resolution(reference: &str, resolution: &Resolution) -> (String, String) {
    match resolution {
        Resolution::Found { path, outcome } => {
            let stdout = format!("{}\n", path.display());
            let stderr = match outcome {
                Outcome::Live => String::new(),
                Outcome::Done => format!(
                    "note: referenced task is retired (DONE): {}\n",
                    path.display()
                ),
                // The abandoned counterpart of the DONE note above (ADR
                // *pruning*): `resolve` must not let a pruned dead end look
                // live, the same failure mode the ADR exists to prevent.
                Outcome::Abandoned => format!(
                    "note: referenced task is abandoned (ABANDONED): {}\n",
                    path.display()
                ),
            };
            (stdout, stderr)
        }
        Resolution::NotFound => (
            String::new(),
            format!("resolve: no entry matches reference {reference:?}\n"),
        ),
        Resolution::Ambiguous(matches) => {
            let mut stderr =
                format!("resolve: reference {reference:?} is ambiguous; re-query by key:\n");
            for m in matches {
                let tag = match m.outcome {
                    Outcome::Live => "",
                    Outcome::Done => " (retired)",
                    Outcome::Abandoned => " (abandoned)",
                };
                stderr.push_str(&format!("  [{}] {}{}\n", m.key, m.path.display(), tag));
            }
            (String::new(), stderr)
        }
    }
}

/// Read one directory's grove entries, parsed and sorted by the per-level
/// comparator (the charter brief first, then by numeric position, foreign last).
/// Returns `(Entry, path)` for every child whose name parses **and** whose real
/// filesystem kind agrees with the parse — `tree_id::parse` infers leaf-vs-node
/// from the `.md` suffix alone, so a *directory* named `…-k1.md` (parses as a
/// leaf) or a *file* shaped like a node name is reconciled here as foreign and
/// dropped. This is the shared one-level read behind every walk in this module.
fn read_level(dir: &Path) -> Result<Vec<(Entry, PathBuf)>> {
    let mut entries: Vec<(String, Entry, PathBuf)> = Vec::new();
    for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let Some(parsed) = parse(&name) else { continue };
        let is_dir = match entry.file_type() {
            Ok(t) => t.is_dir(),
            // A symlink or an entry we cannot stat is treated as foreign.
            Err(_) => continue,
        };
        // A node is a directory; a brief and a leaf are files. A kind mismatch
        // (e.g. a directory named like a leaf) is foreign — never a task.
        let kind_ok = match parsed {
            Entry::Node { .. } => is_dir,
            Entry::Brief | Entry::Leaf { .. } => !is_dir,
        };
        if !kind_ok {
            continue;
        }
        entries.push((name, parsed, entry.path()));
    }
    entries.sort_by_key(|a| sort_key(&a.0));
    Ok(entries.into_iter().map(|(_, e, p)| (e, p)).collect())
}

/// Recursively collect every parsed entry in the tree — leaves (live and `DONE`),
/// node directories, and briefs — each with its absolute path, in pre-order.
/// The shared scan behind [`resolve`]'s tree-wide key/slug search.
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
    use tempfile::TempDir;

    /// Stand up a fresh `.grove/` directory and return `(tempdir, grove_root)`.
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    /// Write a stub file into `dir`, returning its absolute path.
    fn touch(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, b"# stub\n").unwrap();
        p
    }

    /// Create a node directory inside `dir`, returning its absolute path.
    fn mknode(dir: &Path, name: &str) -> PathBuf {
        let p = dir.join(name);
        fs::create_dir_all(&p).unwrap();
        p
    }

    /// The path's final component, for terse assertions.
    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    // ---- pick ---------------------------------------------------------------

    #[test]
    fn pick_returns_first_live_leaf_in_per_level_order() {
        let (_t, g) = grove();
        touch(&g, "02-b-k2.md");
        touch(&g, "01-a-k1.md");
        touch(&g, "10-c-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-a-k1.md");
    }

    #[test]
    fn pick_orders_numerically_not_lexically() {
        // 9 < 10 — the case a dumb lexical sort of unpadded positions fails. The
        // v2 grammar zero-pads to two digits, but the comparator is numeric so an
        // unpadded hand-typed name still orders right.
        let (_t, g) = grove();
        touch(&g, "10-b-k2.md");
        touch(&g, "2-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "2-a-k1.md");
    }

    #[test]
    fn pick_skips_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "01-DONE-a-k1.md");
        touch(&g, "02-b-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-b-k2.md");
    }

    #[test]
    fn pick_skips_abandoned_leaves() {
        // Symmetric with DONE (ADR *pruning*): an abandoned leaf is a terminal
        // state, skipped exactly like a retired one.
        let (_t, g) = grove();
        touch(&g, "01-ABANDONED-a-k1.md");
        touch(&g, "02-b-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-b-k2.md");
    }

    #[test]
    fn pick_descends_a_node_in_preorder() {
        // A node at an earlier position is fully explored before a later sibling
        // leaf: the node's first live child wins.
        let (_t, g) = grove();
        let node = mknode(&g, "01-design-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-child-k2.md");
        touch(&g, "02-later-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-child-k2.md");
    }

    #[test]
    fn pick_skips_briefs_and_returns_the_child_leaf() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let node = mknode(&g, "01-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-child-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-child-k2.md");
    }

    #[test]
    fn pick_falls_through_an_all_done_node_to_a_later_live_leaf() {
        // A node whose subtree is entirely retired yields no live leaf, so pick
        // moves on to the next sibling.
        let (_t, g) = grove();
        let node = mknode(&g, "01-done-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-DONE-child-k2.md");
        touch(&g, "02-live-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-live-k3.md");
    }

    #[test]
    fn pick_falls_through_a_pruned_node_to_a_later_live_leaf() {
        // A node whose only leaf was pruned yields no live leaf either — the
        // grove's two terminal leaf states (DONE, ABANDONED) behave identically
        // for the walk (ADR *pruning*).
        let (_t, g) = grove();
        let node = mknode(&g, "01-dead-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-ABANDONED-child-k2.md");
        touch(&g, "02-live-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-live-k3.md");
    }

    #[test]
    fn pick_descends_nested_nodes() {
        let (_t, g) = grove();
        let n1 = mknode(&g, "01-outer-k1");
        touch(&n1, "BRIEF.md");
        let n2 = mknode(&n1, "01-inner-k2");
        touch(&n2, "BRIEF.md");
        touch(&n2, "01-deep-k3.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-deep-k3.md");
    }

    #[test]
    fn pick_none_when_only_briefs_and_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let node = mknode(&g, "01-node-k1");
        touch(&node, "BRIEF.md");
        touch(&node, "01-DONE-child-k2.md");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_none_when_every_remaining_leaf_is_abandoned() {
        // A grove whose only remaining leaves are abandoned reports "no live
        // leaves" — correct: the work is settled, however it settled.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-ABANDONED-a-k1.md");
        let node = mknode(&g, "02-node-k2");
        touch(&node, "BRIEF.md");
        touch(&node, "01-DONE-b-k3.md");
        touch(&node, "02-ABANDONED-c-k4.md");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_none_on_empty_tree() {
        let (_t, g) = grove();
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_lenient_on_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        touch(&g, "01-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-a-k1.md");
    }

    #[test]
    fn pick_none_when_only_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_ignores_a_directory_shaped_like_a_leaf() {
        // A directory whose name parses as a leaf (`.md` suffix) is reconciled as
        // foreign — a leaf is a file. So it is neither returned nor descended.
        let (_t, g) = grove();
        mknode(&g, "01-trap-k1.md");
        touch(&g, "02-real-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-real-k2.md");
    }

    #[test]
    fn pick_ignores_a_file_shaped_like_a_node() {
        // A *file* whose name parses as a node directory (no `.md`) is foreign —
        // a node is a directory — so pick never tries to descend it.
        let (_t, g) = grove();
        touch(&g, "01-trap-k1");
        touch(&g, "02-real-k2.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "02-real-k2.md");
    }

    #[test]
    fn pick_ignores_a_legacy_done_directory() {
        // A stray `done/` directory (or any foreign dir) is not a node and holds
        // no live leaf reachable by the walk.
        let (_t, g) = grove();
        let legacy = mknode(&g, "done");
        touch(&legacy, "09-old-k9.md");
        touch(&g, "01-a-k1.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "01-a-k1.md");
    }

    #[test]
    fn pick_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = pick(&missing).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- brief-chain --------------------------------------------------------

    #[test]
    fn brief_chain_root_level_leaf_returns_only_root_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let leaf = touch(&g, "01-a-k1.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_two_levels_deep_root_then_each_ancestor_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "02-mid-k1");
        touch(&n1, "BRIEF.md");
        let n2 = mknode(&n1, "01-node-k2");
        touch(&n2, "BRIEF.md");
        let leaf = touch(&n2, "01-leaf-k3.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        // Each brief's parent dir distinguishes them; assert on the parent.
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "02-mid-k1", "01-node-k2"]
        );
        assert!(chain.iter().all(|p| name_of(p) == "BRIEF.md"));
    }

    #[test]
    fn brief_chain_only_includes_ancestors_not_sibling_subtrees() {
        // The directory ascent inherently excludes a sibling node's brief: a leaf
        // under `01-design` never sees `02-other`'s brief.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let design = mknode(&g, "01-design-k1");
        touch(&design, "BRIEF.md");
        let other = mknode(&g, "02-other-k3");
        touch(&other, "BRIEF.md");
        let leaf = touch(&design, "01-leaf-k2.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_intermediate_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        // No BRIEF.md in `02-mid` — a mid-decomposition transient.
        let n1 = mknode(&g, "02-mid-k1");
        let n2 = mknode(&n1, "01-node-k2");
        touch(&n2, "BRIEF.md");
        let leaf = touch(&n2, "01-leaf-k3.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-node-k2"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_root_brief() {
        let (_t, g) = grove();
        // No root BRIEF.md.
        let n1 = mknode(&g, "02-mid-k1");
        touch(&n1, "BRIEF.md");
        let leaf = touch(&n1, "01-leaf-k2.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec!["02-mid-k1"]
        );
    }

    #[test]
    fn brief_chain_resolves_chain_for_a_done_leaf() {
        // Normally called on a live leaf, but a `DONE` leaf still has ancestors.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-design-k1");
        touch(&n1, "BRIEF.md");
        let leaf = touch(&n1, "01-DONE-leaf-k2.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_accepts_grove_root_relative_leaf_path() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-design-k1");
        touch(&n1, "BRIEF.md");
        touch(&n1, "01-leaf-k2.md");
        let chain = brief_chain(&g, Path::new("01-design-k1/01-leaf-k2.md")).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-design-k1"]
        );
    }

    #[test]
    fn brief_chain_errors_when_leaf_outside_grove_root() {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        let stray = tmp.path().join("stray.md");
        fs::write(&stray, b"# stub\n").unwrap();
        let err = brief_chain(&g, &stray).unwrap_err();
        assert!(
            err.to_string().contains("not under grove root"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_given_the_grove_root_itself() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let err = brief_chain(&g, &g).unwrap_err();
        assert!(
            err.to_string().contains("grove root, not a leaf"),
            "got {err}"
        );
    }

    #[test]
    fn brief_chain_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = brief_chain(&missing, Path::new("01-a-k1.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- kind ---------------------------------------------------------------

    /// Write a leaf whose body carries `body_after_header` verbatim after the
    /// `# <slug>` header, returning its absolute path — so a test can set (or
    /// omit, or garble) the `**Kind:**` line.
    fn touch_body(dir: &Path, name: &str, body_after_header: &str) -> PathBuf {
        let p = dir.join(name);
        fs::write(&p, format!("# stub\n\n{body_after_header}").as_bytes()).unwrap();
        p
    }

    #[test]
    fn kind_reads_an_impl_leaf() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-a-k1.md", "**Kind:** impl\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_reads_a_planning_leaf() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-a-k1.md", "**Kind:** planning\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Planning));
    }

    #[test]
    fn kind_reads_every_one_of_the_seventeen() {
        // The whole set through the real file-reading path, not just
        // `Kind::parse` — the marker-line scan is what a hyphenated label
        // (`integrate-review-impl`) could plausibly trip over.
        let (_t, g) = grove();
        for (i, want) in Kind::ALL.into_iter().enumerate() {
            let name = format!("{:02}-a-k{}.md", i + 1, i + 1);
            let leaf = touch_body(&g, &name, &format!("**Kind:** {}\n", want.label()));
            assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(want));
        }
    }

    #[test]
    fn kind_reads_the_legacy_work_label_as_impl() {
        // The compatibility rule the rename stands on (task-kind-taxonomy): a
        // live grove's task files say `work` and must keep resolving, silently.
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-a-k1.md", "**Kind:** work\n\n## Goal\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_no_arg_defaults_to_picks_next_leaf() {
        let (_t, g) = grove();
        touch(&g, "01-DONE-old-k1.md"); // skipped by pick
        touch_body(&g, "02-live-k2.md", "**Kind:** planning\n");
        // No leaf arg ⇒ pick's next live leaf (02-live), whose kind is planning.
        assert_eq!(kind(&g, None).unwrap(), Some(Kind::Planning));
    }

    #[test]
    fn kind_none_on_empty_grove() {
        // No live leaves ⇒ Ok(None), the same signal pick gives (the CLI renders
        // the "no live leaves" diagnostic).
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        assert_eq!(kind(&g, None).unwrap(), None);
    }

    #[test]
    fn kind_accepts_a_grove_root_relative_path() {
        let (_t, g) = grove();
        let node = mknode(&g, "01-design-k1");
        touch(&node, "BRIEF.md");
        touch_body(&node, "01-leaf-k2.md", "**Kind:** impl\n");
        let got = kind(&g, Some(Path::new("01-design-k1/01-leaf-k2.md"))).unwrap();
        assert_eq!(got, Some(Kind::Impl));
    }

    #[test]
    fn kind_tolerates_trailing_commentary_on_the_kind_line() {
        // TASK-FORMAT's own example writes `**Kind:** impl   (or: planning)`.
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-a-k1.md", "**Kind:** impl          (or: planning)\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_degrades_to_impl_on_a_missing_kind_line() {
        // Read degrades (task-kind-taxonomy): a leaf with no `**Kind:**` line at
        // all — `touch` writes only `# stub` — is treated as `impl`, never an
        // error, so a hand-edited leaf can never jam the self-driving loop.
        let (_t, g) = grove();
        let leaf = touch(&g, "01-a-k1.md");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_degrades_to_impl_on_a_garbled_kind_token() {
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-a-k1.md", "**Kind:** bogus\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_degrades_on_a_family_name_written_as_a_kind() {
        // `review` and `integrate-review` are *families* (the routing axis), not
        // members of the set. Written on a leaf they are simply unrecognised —
        // the degrade path, not a silent match against the five `review-*` kinds.
        let (_t, g) = grove();
        let leaf = touch_body(&g, "01-a-k1.md", "**Kind:** review\n");
        assert_eq!(kind(&g, Some(&leaf)).unwrap(), Some(Kind::Impl));
    }

    #[test]
    fn kind_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = kind(&missing, None).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- resolve ------------------------------------------------------------

    /// A nested tree with two `add` slugs (one live leaf, one retired leaf in a
    /// different subtree) so bare-slug `add` is ambiguous, plus a node directory
    /// and a unique `build` leaf — to exercise key, slug, node, and `DONE`.
    ///
    /// ```text
    /// .grove/
    ///   BRIEF.md
    ///   01-design-k1/         node
    ///     BRIEF.md
    ///     01-add-k2.md        live leaf, slug "add"
    ///     02-remove-k3.md
    ///   02-add-k4.DONE? -> 02-DONE-add-k4.md   retired leaf, slug "add"
    ///   03-build-k5.md
    /// ```
    fn resolve_fixture() -> (TempDir, PathBuf) {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        let design = mknode(&g, "01-design-k1");
        touch(&design, "BRIEF.md");
        touch(&design, "01-add-k2.md");
        touch(&design, "02-remove-k3.md");
        touch(&g, "02-DONE-add-k4.md");
        touch(&g, "03-build-k5.md");
        (tmp, g)
    }

    #[test]
    fn resolve_by_bracket_key_finds_a_nested_leaf() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[2]").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-add-k2.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_by_bare_number_finds_a_done_leaf() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "4").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "02-DONE-add-k4.md");
                assert_eq!(outcome, Outcome::Done, "the key-4 task is DONE");
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_finds_a_pruned_leaf_by_key() {
        // ADR *pruning*: an abandoned leaf's key must stay resolvable — durable
        // cross-references to it (commit messages, ADRs, briefs) are precisely
        // what the ADR protects. And `resolve` must not let the match *look*
        // live: `outcome` must come back `Abandoned`, not just the right path —
        // this is the exact failure mode ADR *pruning* exists to prevent ("a
        // tree that hides its dead ends lies"), here in `resolve` rather than
        // the tree itself.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-ABANDONED-spike-k1.md");
        match resolve(&g, "[1]").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-ABANDONED-spike-k1.md");
                assert_eq!(outcome, Outcome::Abandoned);
            }
            other => panic!("expected Found, got {other:?}"),
        }
        // The full `<slug>-k<key>` handle resolves it too.
        match resolve(&g, "spike-k1").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-ABANDONED-spike-k1.md");
                assert_eq!(outcome, Outcome::Abandoned);
            }
            other => panic!("expected Found, got {other:?}"),
        }
        // And the CLI-facing render carries the same note a DONE match gets,
        // in its own wording.
        let (_out, err) = render_resolution("[1]", &resolve(&g, "[1]").unwrap());
        assert!(
            err.contains("abandoned") && err.contains("ABANDONED"),
            "got {err:?}"
        );
    }

    #[test]
    fn resolve_bracket_key_ignores_decorative_slug() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[5]-whatever").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "03-build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_key_resolves_a_node_to_its_directory() {
        // A node's identity rides in its directory name, so a key reference to a
        // node resolves to the directory path (append /BRIEF.md to read it).
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[1]").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "01-design-k1");
                assert!(path.is_dir());
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_key_not_found() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "[99]").unwrap(), Resolution::NotFound);
    }

    #[test]
    fn resolve_bare_slug_unique_across_dirs() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "03-build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_bare_slug_resolves_a_nested_unique_leaf() {
        // `remove` lives only inside the node directory — slug search recurses.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "remove").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "02-remove-k3.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_bare_slug_not_found() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "nope").unwrap(), Resolution::NotFound);
    }

    #[test]
    fn resolve_bare_slug_ambiguous_lists_every_match_by_key() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "add").unwrap() {
            Resolution::Ambiguous(matches) => {
                // Pre-order: the nested `01-design/01-add-k2` precedes the
                // root-level `02-DONE-add-k4`.
                assert_eq!(matches.len(), 2);
                assert_eq!(matches[0].key, 2);
                assert_eq!(name_of(&matches[0].path), "01-add-k2.md");
                assert_eq!(matches[0].outcome, Outcome::Live);
                assert_eq!(matches[1].key, 4);
                assert_eq!(name_of(&matches[1].path), "02-DONE-add-k4.md");
                assert_eq!(matches[1].outcome, Outcome::Done);
            }
            other => panic!("expected Ambiguous, got {other:?}"),
        }
    }

    #[test]
    fn resolve_root_brief_is_unreferenceable() {
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "").unwrap(), Resolution::NotFound);
    }

    #[test]
    fn resolve_malformed_bracket_ref_errors() {
        let (_t, g) = resolve_fixture();
        assert!(resolve(&g, "[abc]").is_err());
        assert!(resolve(&g, "[4").is_err());
    }

    #[test]
    fn resolve_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = resolve(&missing, "[1]").unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- resolve: the full `<slug>-k<key>` handle (task-tree-scheme §5) --------------

    #[test]
    fn resolve_by_full_slug_handle_finds_by_terminal_key() {
        // §5's canonical commit/prose handle is `<slug>-k<key>`; resolve accepts it
        // directly — the terminal `-k<key>` is read as the key, the slug decorative
        // — so the handle round-trips back to a path.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build-k5").unwrap() {
            Resolution::Found { path, outcome } => {
                assert_eq!(name_of(&path), "03-build-k5.md");
                assert_eq!(outcome, Outcome::Live);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_full_handle_disambiguates_what_a_bare_slug_could_not() {
        // The bare slug `add` is ambiguous (two matches); the full handle `add-k2`
        // names exactly the nested live leaf via its key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "add-k2").unwrap() {
            Resolution::Found { path, .. } => {
                assert_eq!(name_of(&path), "01-add-k2.md");
                assert_eq!(name_of(path.parent().unwrap()), "01-design-k1");
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_handle_of_a_node_resolves_to_its_directory() {
        // A node's handle (`design-k1`) resolves to the node directory, like its key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "design-k1").unwrap() {
            Resolution::Found { path, .. } => {
                assert_eq!(name_of(&path), "01-design-k1");
                assert!(path.is_dir());
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_prefers_a_real_slug_over_the_handle_fallback() {
        // A bare slug that itself ends in `-k<digits>` resolves as a slug first; the
        // key fallback fires only when the slug match is empty. So a slug `foo-k5`
        // (key 7) wins over a *different* entity that happens to hold key 5.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "01-foo-k5-k7.md"); // slug "foo-k5", key 7
        touch(&g, "02-other-k5.md"); // slug "other", key 5
        match resolve(&g, "foo-k5").unwrap() {
            Resolution::Found { path, .. } => {
                assert_eq!(
                    name_of(&path),
                    "01-foo-k5-k7.md",
                    "the real slug match must win over the key-5 handle fallback"
                );
            }
            other => panic!("expected Found (slug), got {other:?}"),
        }
    }

    #[test]
    fn resolve_handle_shaped_but_unmatched_is_not_found() {
        // A handle whose key matches nothing is NotFound (not an error), like any
        // unmatched reference.
        let (_t, g) = resolve_fixture();
        assert_eq!(resolve(&g, "ghost-k99").unwrap(), Resolution::NotFound);
    }

    // ---- render_resolution --------------------------------------------------

    #[test]
    fn render_found_prints_path_no_stderr() {
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/03-build-k5.md"),
            outcome: Outcome::Live,
        };
        let (out, err) = render_resolution("[5]", &r);
        assert_eq!(out, "/g/.grove/03-build-k5.md\n");
        assert!(err.is_empty(), "got {err:?}");
    }

    #[test]
    fn render_found_retired_notes_on_stderr_but_still_prints_path() {
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/02-DONE-add-k4.md"),
            outcome: Outcome::Done,
        };
        let (out, err) = render_resolution("4", &r);
        assert_eq!(out, "/g/.grove/02-DONE-add-k4.md\n");
        assert!(err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_found_abandoned_notes_on_stderr_but_still_prints_path() {
        // The abandoned counterpart of the DONE case above: a resolved
        // `ABANDONED` entry must get its own stderr note, not silence
        // (silence is what a live match gets) and not the DONE wording.
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/01-ABANDONED-spike-k1.md"),
            outcome: Outcome::Abandoned,
        };
        let (out, err) = render_resolution("1", &r);
        assert_eq!(out, "/g/.grove/01-ABANDONED-spike-k1.md\n");
        assert!(err.contains("abandoned"), "got {err:?}");
        assert!(!err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_not_found_empty_stdout_diagnostic_stderr() {
        let (out, err) = render_resolution("nope", &Resolution::NotFound);
        assert!(out.is_empty(), "got {out:?}");
        assert!(err.contains("no entry matches"), "got {err:?}");
        assert!(err.contains("nope"), "got {err:?}");
    }

    #[test]
    fn render_ambiguous_lists_keys_on_stderr_empty_stdout() {
        let r = Resolution::Ambiguous(vec![
            AmbiguousMatch {
                key: 2,
                path: PathBuf::from("/g/.grove/01-design-k1/01-add-k2.md"),
                outcome: Outcome::Live,
            },
            AmbiguousMatch {
                key: 4,
                path: PathBuf::from("/g/.grove/02-DONE-add-k4.md"),
                outcome: Outcome::Done,
            },
        ]);
        let (out, err) = render_resolution("add", &r);
        assert!(
            out.is_empty(),
            "stdout must be empty for ambiguous; got {out:?}"
        );
        assert!(err.contains("ambiguous"), "got {err:?}");
        assert!(err.contains("[2]"), "got {err:?}");
        assert!(err.contains("[4]"), "got {err:?}");
        assert!(err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_ambiguous_tags_an_abandoned_match() {
        let r = Resolution::Ambiguous(vec![AmbiguousMatch {
            key: 1,
            path: PathBuf::from("/g/.grove/01-ABANDONED-spike-k1.md"),
            outcome: Outcome::Abandoned,
        }]);
        let (_out, err) = render_resolution("spike", &r);
        assert!(
            err.contains("[1]") && err.contains("(abandoned)"),
            "got {err:?}"
        );
    }

    // ---- pick + brief-chain together ----------------------------------------

    #[test]
    fn pick_then_brief_chain_on_a_realistic_nested_tree() {
        // End-to-end: pick the first live leaf in a nested tree, then resolve its
        // ancestor brief chain — the loop's bootstrap path.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let n1 = mknode(&g, "01-scheme-k1");
        touch(&n1, "BRIEF.md");
        touch(&n1, "01-DONE-id-model-k2.md");
        let leaf = touch(&n1, "02-read-verbs-k3.md");
        touch(&g, "02-shed-tui-k4.md");

        let picked = pick(&g).unwrap().unwrap();
        assert_eq!(picked, leaf);

        let chain = brief_chain(&g, &picked).unwrap();
        assert_eq!(
            chain
                .iter()
                .map(|p| name_of(p.parent().unwrap()))
                .collect::<Vec<_>>(),
            vec![".grove", "01-scheme-k1"]
        );
    }
}

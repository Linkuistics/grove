// The new-format **read verbs** — `pick`, `brief-chain`, and the new `resolve` —
// built on the 010 id model (`src/leaf_id.rs`). Implements ADR-0033 and the 050
// BRIEF running log D2/D4/D7.
//
// The structural shift from the old directory-based verbs (`src/pick.rs`,
// `src/brief_chain.rs`): the new scheme is **flat** (every task is a file
// directly in `.grove/`, no node directories), so:
//   * `pick` is a single `read_dir` + version-sort + first-live-leaf filter,
//     not a recursive depth-first directory walk;
//   * `brief-chain` ascends the leaf's **position-prefix ids** collecting the
//     brief at each prefix, not the `BRIEF.md` of each ancestor *directory*;
//   * `resolve` is brand new — it turns a permanent-key reference (`[n]`/`n`) or
//     a bare slug into the current file path (the consumer side of D6's stable
//     reference producer).
//
// Per D9 this module is **new, isolated code**: it does NOT touch the live verb
// path (`src/pick.rs`, `src/brief_chain.rs`, `src/llm_cli.rs`), which keeps
// speaking the old `NNN-slug` format until leaf 060 wires the dual-format
// dispatch in. So no 050 commit can break this old-format grove driving itself.

use crate::leaf_id::{parse, sort_key, LeafId};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// `pick`: walk the flat `.grove/` list in version-sort order (010 comparator),
/// skip every `.BRIEF` and every `.DONE`, and return the first **live leaf**.
/// `Ok(None)` means the tree has no live leaves — the loop's finish signal (the
/// CLI renders that as empty stdout + a "no live leaves; this grove is done"
/// diagnostic on stderr when wired in 060). Lenient on foreign/malformed files
/// (D2): anything that does not parse is simply not a leaf, so a stray
/// `README.md` never jams the loop. Never reads file contents.
pub fn pick(grove_root: &Path) -> Result<Option<PathBuf>> {
    let files = scan(grove_root)?;
    Ok(files
        .into_iter()
        .find(|(id, _)| id.is_live_leaf())
        .map(|(_, path)| path))
}

/// `brief-chain`: for the leaf at `leaf_path`, collect the brief at each
/// **proper-prefix position** of the leaf's id, in root→leaf order. A leaf at
/// position `[a,b,c]` pulls the briefs at `[]` (root `BRIEF.md`), `[a]`, and
/// `[a,b]`. Briefs are found by **id-prefix** in the flat namespace, not by
/// directory ascent (the key behavioural shift). A missing brief at any level is
/// skipped silently (some nodes are mid-decomposition); briefs are never `.DONE`
/// (D4). `leaf_path` is absolute or relative to `grove_root`.
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

    let leaf_name = leaf_abs
        .file_name()
        .and_then(|n| n.to_str())
        .with_context(|| format!("leaf path {} has no filename", leaf_abs.display()))?;
    let leaf_id = parse(leaf_name)
        .with_context(|| format!("leaf filename is not a new-format id: {leaf_name}"))?;

    // Index every brief in the flat namespace by its position vector, then pull
    // the brief at each *proper prefix* of the leaf's position, root→leaf. A
    // leaf at `[a,b,c]` collects positions `[]`, `[a]`, `[a,b]` — never `[a,b,c]`
    // itself (a leaf has no brief). Missing levels are simply absent from the map.
    let briefs = briefs_by_position(&grove_root)?;
    let mut chain = Vec::new();
    for len in 0..leaf_id.position.len() {
        let prefix = leaf_id.position[..len].to_vec();
        if let Some(path) = briefs.get(&prefix) {
            chain.push(path.clone());
        }
    }
    Ok(chain)
}

/// The outcome of resolving a reference (D7). The CLI maps this to stdout/stderr
/// via [`render_resolution`]; the split is kept out of the core so it is unit-
/// testable without a live verb dispatch (D9).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Resolution {
    /// Exactly one file matched. `retired` is `true` for a `.DONE` leaf.
    Found { path: PathBuf, retired: bool },
    /// No file matched the reference (pick-style: not an error).
    NotFound,
    /// A bare-slug reference matched more than one file. Each entry carries its
    /// permanent `[key]` so the caller re-queries by the unambiguous key.
    Ambiguous(Vec<AmbiguousMatch>),
}

/// One entry of an [`Resolution::Ambiguous`] result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmbiguousMatch {
    pub key: u32,
    pub path: PathBuf,
    pub retired: bool,
}

/// `resolve <ref>` (D7): turn a reference into the current file path, searching
/// live **and** `.DONE` files.
///   * `[n]` / `n` → the unique file whose permanent key is `n` (keys are unique
///     tree-wide → 0 or 1 match). This is the primary durable-reference path.
///   * `[n]-slug` → same; the slug part is decorative (ignored).
///   * bare slug → resolve by slug: 0 ⇒ `NotFound`; 1 ⇒ `Found`; >1 ⇒
///     `Ambiguous` (every match with its key, so the caller re-queries by key).
///
/// The root brief (`BRIEF.md`, the one unkeyed singleton) is unreferenceable.
pub fn resolve(grove_root: &Path, reference: &str) -> Result<Resolution> {
    let files = scan(grove_root)?;
    match parse_ref(reference)? {
        Ref::Key(key) => {
            // Keys are unique tree-wide → at most one match; `scan` is sorted so
            // a malformed duplicate still breaks deterministically.
            Ok(files.iter().find(|(id, _)| id.key == Some(key)).map_or(
                Resolution::NotFound,
                |(id, path)| Resolution::Found {
                    path: path.clone(),
                    retired: id.is_done,
                },
            ))
        }
        Ref::Slug(slug) => {
            // The root brief (key None) is excluded — it is unreferenceable (D6).
            let matches: Vec<AmbiguousMatch> = files
                .iter()
                .filter_map(|(id, path)| {
                    let key = id.key?;
                    (id.slug == slug).then(|| AmbiguousMatch {
                        key,
                        path: path.clone(),
                        retired: id.is_done,
                    })
                })
                .collect();
            Ok(match matches.as_slice() {
                [] => Resolution::NotFound,
                [m] => Resolution::Found {
                    path: m.path.clone(),
                    retired: m.retired,
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

/// Classify a `resolve` reference. `[n]` / `[n]-slug` and a bare integer `n`
/// resolve by key; anything else is a bare slug. A bracketed-but-malformed key
/// (`[abc]`, `[4`) is a reference error (distinct from a valid-but-unmatched
/// reference, which is `NotFound`).
fn parse_ref(reference: &str) -> Result<Ref> {
    if let Some(rest) = reference.strip_prefix('[') {
        let close = rest
            .find(']')
            .with_context(|| format!("reference {reference:?}: unclosed '['"))?;
        let key: u32 = rest[..close]
            .parse()
            .with_context(|| format!("reference {reference:?}: '[…]' is not an integer key"))?;
        // Anything after `]` (e.g. `-slug`) is decorative (D7).
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

/// Render a [`Resolution`] to the `(stdout, stderr)` the `resolve` verb emits
/// once wired in (060). Kept pure and separate from the I/O so the exact
/// stdout/stderr contract is unit-testable while the live CLI dispatch stays
/// untouched (D9).
pub fn render_resolution(reference: &str, resolution: &Resolution) -> (String, String) {
    match resolution {
        Resolution::Found { path, retired } => {
            let stdout = format!("{}\n", path.display());
            let stderr = if *retired {
                format!(
                    "note: referenced task is retired (.DONE): {}\n",
                    path.display()
                )
            } else {
                String::new()
            };
            (stdout, stderr)
        }
        Resolution::NotFound => (
            String::new(),
            format!("resolve: no file matches reference {reference:?}\n"),
        ),
        Resolution::Ambiguous(matches) => {
            let mut stderr =
                format!("resolve: reference {reference:?} is ambiguous; re-query by key:\n");
            for m in matches {
                stderr.push_str(&format!(
                    "  [{}] {}{}\n",
                    m.key,
                    m.path.display(),
                    if m.retired { " (retired)" } else { "" }
                ));
            }
            (String::new(), stderr)
        }
    }
}

/// Read the flat `.grove/` directory and parse every regular file, returning the
/// `(LeafId, path)` of each that parses (foreign/malformed files excluded —
/// leniency, D2), **sorted by the 010 version-sort comparator** so the list is
/// DFS pre-order. The shared scan behind `pick` and `resolve`. Subdirectories
/// are skipped: the new scheme is flat, so a directory is never a task.
fn scan(grove_root: &Path) -> Result<Vec<(LeafId, PathBuf)>> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let mut entries: Vec<(String, LeafId, PathBuf)> = Vec::new();
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        match entry.file_type() {
            Ok(t) if t.is_file() => {}
            _ => continue,
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(id) = parse(&name) {
            entries.push((name, id, entry.path()));
        }
    }
    entries.sort_by(|a, b| sort_key(&a.0).cmp(&sort_key(&b.0)));
    Ok(entries
        .into_iter()
        .map(|(_, id, path)| (id, path))
        .collect())
}

/// Index every node/root brief in the flat namespace by its position vector
/// (root brief → `[]`). Used by `brief-chain` to find ancestor briefs by
/// id-prefix. Briefs are never `.DONE` (D4), so every entry is live.
fn briefs_by_position(grove_root: &Path) -> Result<HashMap<Vec<u32>, PathBuf>> {
    let mut map = HashMap::new();
    for (id, path) in scan(grove_root)? {
        if id.is_brief {
            map.insert(id.position, path);
        }
    }
    Ok(map)
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

    /// Write a stub file into the grove root, returning its absolute path.
    fn touch(root: &Path, name: &str) -> PathBuf {
        let p = root.join(name);
        fs::write(&p, b"# stub\n").unwrap();
        p
    }

    /// The picked path's filename, for terse assertions.
    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    // ---- pick ---------------------------------------------------------------

    #[test]
    fn pick_returns_first_live_leaf_in_version_sort_order() {
        let (_t, g) = grove();
        touch(&g, "2-[2]-b.md");
        touch(&g, "1-[1]-a.md");
        touch(&g, "10-[3]-c.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "1-[1]-a.md");
    }

    #[test]
    fn pick_orders_numerically_not_lexically() {
        // The exact case the old zero-padded lexical sort can't carry over.
        let (_t, g) = grove();
        touch(&g, "10-[2]-b.md");
        touch(&g, "2-[1]-a.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "2-[1]-a.md");
    }

    #[test]
    fn pick_skips_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "1-[1]-a.DONE.md");
        touch(&g, "2-[2]-b.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "2-[2]-b.md");
    }

    #[test]
    fn pick_skips_root_and_node_briefs_and_returns_the_child_leaf() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-node.BRIEF.md");
        touch(&g, "1.1-[2]-child.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "1.1-[2]-child.md");
    }

    #[test]
    fn pick_none_when_only_briefs_and_done_leaves() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-node.BRIEF.md");
        touch(&g, "1.1-[2]-child.DONE.md");
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
        touch(&g, "1-[1]-a.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "1-[1]-a.md");
    }

    #[test]
    fn pick_none_when_only_foreign_files() {
        let (_t, g) = grove();
        touch(&g, "README.md");
        touch(&g, "notes.txt");
        assert_eq!(pick(&g).unwrap(), None);
    }

    #[test]
    fn pick_ignores_subdirectories() {
        // The new scheme is flat: a stray directory is never a leaf, even a
        // legacy `done/` dir or one named like a leaf.
        let (_t, g) = grove();
        fs::create_dir_all(g.join("done")).unwrap();
        touch(&g.join("done"), "9-[9]-old.md");
        fs::create_dir_all(g.join("1-[8]-dir.md")).unwrap();
        touch(&g, "2-[1]-a.md");
        let got = pick(&g).unwrap().unwrap();
        assert_eq!(name_of(&got), "2-[1]-a.md");
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
        let leaf = touch(&g, "1-[1]-a.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_two_levels_deep_root_then_node_briefs() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[1]-mid.BRIEF.md");
        touch(&g, "2.1-[2]-node.BRIEF.md");
        let leaf = touch(&g, "2.1.1-[3]-leaf.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md", "2-[1]-mid.BRIEF.md", "2.1-[2]-node.BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_finds_ancestors_by_id_prefix_not_directory() {
        // The headline behavioural shift: briefs are found by position-prefix in
        // the flat namespace. An unrelated node brief at [2] must NOT appear in
        // the chain of a leaf at [1,1] (its prefixes are [] and [1] only).
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-design.BRIEF.md");
        touch(&g, "2-[3]-other.BRIEF.md"); // sibling subtree — not a prefix
        let leaf = touch(&g, "1.1-[2]-leaf.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md", "1-[1]-design.BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_intermediate_brief() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        // No brief at [2] — mid-decomposition transient.
        touch(&g, "2.1-[2]-node.BRIEF.md");
        let leaf = touch(&g, "2.1.1-[3]-leaf.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md", "2.1-[2]-node.BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_skips_missing_root_brief() {
        let (_t, g) = grove();
        touch(&g, "2-[1]-mid.BRIEF.md");
        let leaf = touch(&g, "2.1-[2]-leaf.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["2-[1]-mid.BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_resolves_chain_for_a_done_leaf() {
        // brief-chain is normally called on a live leaf, but a `.DONE` leaf path
        // still has a position and so still yields its chain (robustness).
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-design.BRIEF.md");
        let leaf = touch(&g, "1.1-[2]-leaf.DONE.md");
        let chain = brief_chain(&g, &leaf).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md", "1-[1]-design.BRIEF.md"]
        );
    }

    #[test]
    fn brief_chain_accepts_grove_root_relative_leaf_path() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-design.BRIEF.md");
        touch(&g, "1.1-[2]-leaf.md");
        let chain = brief_chain(&g, Path::new("1.1-[2]-leaf.md")).unwrap();
        assert_eq!(
            chain.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
            vec!["BRIEF.md", "1-[1]-design.BRIEF.md"]
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
    fn brief_chain_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = brief_chain(&missing, Path::new("1-[1]-a.md")).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- resolve ------------------------------------------------------------

    /// A tree with two `add` slugs (one live, one retired) so bare-slug `add`
    /// is ambiguous, plus a node brief and a unique `build` leaf.
    fn resolve_fixture() -> (TempDir, PathBuf) {
        let (tmp, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-design.BRIEF.md");
        touch(&g, "1.1-[2]-add.md");
        touch(&g, "1.2-[3]-remove.md");
        touch(&g, "2-[4]-add.DONE.md");
        touch(&g, "3-[5]-build.md");
        (tmp, g)
    }

    #[test]
    fn resolve_by_bracket_key() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[2]").unwrap() {
            Resolution::Found { path, retired } => {
                assert_eq!(name_of(&path), "1.1-[2]-add.md");
                assert!(!retired);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_by_bare_number() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "4").unwrap() {
            Resolution::Found { path, retired } => {
                assert_eq!(name_of(&path), "2-[4]-add.DONE.md");
                assert!(retired, "the [4] task is .DONE");
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_bracket_key_ignores_decorative_slug() {
        // The slug part is decorative — key 5 resolves even with a stale slug.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[5]-whatever").unwrap() {
            Resolution::Found { path, retired } => {
                assert_eq!(name_of(&path), "3-[5]-build.md");
                assert!(!retired);
            }
            other => panic!("expected Found, got {other:?}"),
        }
    }

    #[test]
    fn resolve_key_resolves_a_node_brief() {
        // Node briefs carry keys too (D6) and are resolvable by key.
        let (_t, g) = resolve_fixture();
        match resolve(&g, "[1]").unwrap() {
            Resolution::Found { path, retired } => {
                assert_eq!(name_of(&path), "1-[1]-design.BRIEF.md");
                assert!(!retired);
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
    fn resolve_bare_slug_unique() {
        let (_t, g) = resolve_fixture();
        match resolve(&g, "build").unwrap() {
            Resolution::Found { path, retired } => {
                assert_eq!(name_of(&path), "3-[5]-build.md");
                assert!(!retired);
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
                // Sorted by position: [1,1] before [2].
                assert_eq!(matches.len(), 2);
                assert_eq!(matches[0].key, 2);
                assert_eq!(name_of(&matches[0].path), "1.1-[2]-add.md");
                assert!(!matches[0].retired);
                assert_eq!(matches[1].key, 4);
                assert_eq!(name_of(&matches[1].path), "2-[4]-add.DONE.md");
                assert!(matches[1].retired);
            }
            other => panic!("expected Ambiguous, got {other:?}"),
        }
    }

    #[test]
    fn resolve_root_brief_is_unreferenceable() {
        // The root brief (`BRIEF.md`, key None) has no reference form.
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

    // ---- render_resolution --------------------------------------------------

    #[test]
    fn render_found_prints_path_no_stderr() {
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/3-[5]-build.md"),
            retired: false,
        };
        let (out, err) = render_resolution("[5]", &r);
        assert_eq!(out, "/g/.grove/3-[5]-build.md\n");
        assert!(err.is_empty(), "got {err:?}");
    }

    #[test]
    fn render_found_retired_notes_on_stderr_but_still_prints_path() {
        let r = Resolution::Found {
            path: PathBuf::from("/g/.grove/2-[4]-add.DONE.md"),
            retired: true,
        };
        let (out, err) = render_resolution("4", &r);
        assert_eq!(out, "/g/.grove/2-[4]-add.DONE.md\n");
        assert!(err.contains("retired"), "got {err:?}");
    }

    #[test]
    fn render_not_found_empty_stdout_diagnostic_stderr() {
        let (out, err) = render_resolution("nope", &Resolution::NotFound);
        assert!(out.is_empty(), "got {out:?}");
        assert!(err.contains("no file matches"), "got {err:?}");
        assert!(err.contains("nope"), "got {err:?}");
    }

    #[test]
    fn render_ambiguous_lists_keys_on_stderr_empty_stdout() {
        let r = Resolution::Ambiguous(vec![
            AmbiguousMatch {
                key: 2,
                path: PathBuf::from("/g/.grove/1.1-[2]-add.md"),
                retired: false,
            },
            AmbiguousMatch {
                key: 4,
                path: PathBuf::from("/g/.grove/2-[4]-add.DONE.md"),
                retired: true,
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
}

// The new-format **grow verbs** — `leaf-add` and `leaf-insert` — built on the 010
// id model (`src/leaf_id.rs`). Implements ADR-0033 and the 050 BRIEF running log
// D5 (verb surface + subtree-cascade renumber) and D6 (a fresh key on add/insert;
// keys + slugs are *preserved* through a renumber, only the dotted **position**
// moves).
//
// The structural shift from the old directory-based verbs (`src/leaf.rs`): the new
// scheme is **flat** (every task is a file directly in `.grove/`, no node
// directories), so a node is addressed by its dotted **id**, not a directory path:
//   * `leaf-add <parent-id> <slug>` appends a child at `<parent-id>.<next>` with a
//     gapless `<next>` (max child index + 1) — the old `--prefix` +10-gap escape
//     hatch is gone (the scheme renumbers instead of leaving gaps);
//   * `leaf-insert <target-id> <slug>` inserts at exactly `<target-id>`, shifting
//     the occupant and later siblings up by one — and that shift **cascades through
//     whole subtrees** (bumping `2.2`→`2.3` also rewrites `2.2.1`→`2.3.1`…). This
//     generalises the old single-directory "+10, highest-first" trick to a dotted
//     subtree; the renumber touches **only the position** (filenames + `# …`
//     headers), never the permanent `[key]` or the slug — the reference-stability
//     D6 exists to guarantee.
//
// Per D9 this module is **new, isolated code**: it does NOT touch the live verb
// path (`src/leaf.rs`, `src/pick.rs`, `src/llm_cli.rs`), which keeps speaking the
// old `NNN-slug` format until leaf 060 wires the dual-format dispatch in. So no
// 050 commit can break this old-format grove driving itself.

use crate::leaf::Kind;
use crate::leaf_id::{next_key, parse, parse_position, sort_key, validate_slug, LeafId};
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Append a child leaf under the node `parent_id` at the next gapless child index.
/// `parent_id` is a dotted position (`2.3`) or `.` for the root. The new leaf gets
/// the next child index (max existing direct-child index + 1, counting `.DONE`
/// children so slots are never reused) and a **fresh permanent key** (max `[n]`
/// over the whole tree + 1, D6). Working-tree only — no commit; no `git` call (a
/// fresh file is simply written, nothing moves).
pub fn leaf_add(grove_root: &Path, parent_id: &str, slug: &str, kind: Kind) -> Result<PathBuf> {
    validate_slug(slug)?;
    let entries = scan(grove_root)?;
    let parent = parse_parent_id(parent_id)?;

    // A non-root parent must exist as a *node* (a brief at that position). Root
    // (`.`) is always valid — it is the grove dir itself. This guards typos and
    // refuses adding a child under a bare leaf (decompose it first).
    if !parent.is_empty()
        && !entries
            .iter()
            .any(|(id, _)| id.is_brief && id.position == parent)
    {
        bail!("parent node {parent_id} not found (no brief at that position)");
    }

    // Next gapless child index = max direct-child index + 1 (or 1 if none). A
    // direct child has a position exactly one segment longer than the parent with
    // the parent as its prefix; `.DONE` children still count so slots are never
    // reused (D6).
    let next_index = entries
        .iter()
        .filter(|(id, _)| id.position.len() == parent.len() + 1 && id.position.starts_with(&parent))
        .map(|(id, _)| id.position[parent.len()])
        .max()
        .map_or(1, |m| m + 1);

    let mut position = parent;
    position.push(next_index);
    let id = LeafId {
        position,
        key: Some(next_key(entries.iter().map(|(_, n)| n.as_str()))),
        slug: slug.to_string(),
        is_brief: false,
        is_done: false,
    };

    let path = grove_root.join(id.filename());
    if path.exists() {
        bail!("destination already exists: {}", path.display());
    }
    write_template(&path, &id, kind)?;
    Ok(path)
}

/// One renamed file produced by `leaf_insert`'s renumber: an existing leaf or
/// node brief whose dotted **position** shifted. The `[key]` and slug are
/// invariant (D6) — only the position (in the filename *and* the `# …` header)
/// changes — so a `Renumber` records just the position move and the names.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Renumber {
    pub old_position: Vec<u32>,
    pub new_position: Vec<u32>,
    pub old_name: String,
    pub new_name: String,
}

/// Insert a new leaf at exactly `target_id`, shifting the current occupant and
/// every later sibling up by one. The shift **cascades through whole subtrees**:
/// for a target with `k` segments and last segment `pos`, every file whose
/// position has ≥ `k` segments, shares the target's first `k-1` segments (same
/// parent), and whose `k`-th segment is ≥ `pos` has that one segment bumped by 1
/// (`2.2`→`2.3` drags `2.2.1`→`2.3.1`, `2.2.2`→`2.3.2`, …). Renames run
/// **highest-position-first** so each `git mv`'s destination is already vacated,
/// rewriting **only the position** in filenames and `# …` headers — never the
/// `[key]` or slug (D6). The new leaf gets a **fresh key**. When nothing is at or
/// after `target_id`, this degenerates to an add at that exact position (empty
/// renumber log). Working-tree only — no commit.
///
/// Returns the new leaf's path and the renumber log (ascending by new position);
/// pass the log to [`surface_cross_refs`] to lint stray positional references.
pub fn leaf_insert(
    grove_root: &Path,
    target_id: &str,
    slug: &str,
    kind: Kind,
) -> Result<(PathBuf, Vec<Renumber>)> {
    validate_slug(slug)?;
    let entries = scan(grove_root)?;
    let target = parse_target_id(target_id)?;
    // The new leaf's fresh key is stable across the renumber (renames preserve
    // keys), so it is computed once over the pre-renumber names.
    let new_key = next_key(entries.iter().map(|(_, n)| n.as_str()));

    // `k` segments, last segment `pos`, parent = the first `k-1` segments. The
    // shift bumps segment `k-1` (zero-based) of every file in the target's parent
    // whose `k`-th segment is ≥ `pos` (and of every descendant of those — they
    // share the prefix, so the same segment moves).
    let k = target.len();
    let pos = target[k - 1];
    let parent = &target[..k - 1];

    let mut affected: Vec<&(LeafId, String)> = entries
        .iter()
        .filter(|(id, _)| {
            id.position.len() >= k && id.position[..k - 1] == *parent && id.position[k - 1] >= pos
        })
        .collect();
    // Highest position first so each `git mv`'s destination is already vacated:
    // a file's destination differs from its source only at segment `k-1`, where
    // it is larger, so it sorts earlier and was moved first (collision-free).
    affected.sort_by(|a, b| sort_key(&b.1).cmp(&sort_key(&a.1)));

    let mut renumbers = Vec::with_capacity(affected.len());
    for (id, old_name) in affected {
        let mut new_position = id.position.clone();
        new_position[k - 1] += 1;
        let new_id = LeafId {
            position: new_position.clone(),
            ..id.clone()
        };
        let new_name = new_id.filename();
        git_mv(grove_root, old_name, &new_name)?;
        rewrite_header_position(&grove_root.join(&new_name), &new_position)?;
        renumbers.push(Renumber {
            old_position: id.position.clone(),
            new_position,
            old_name: old_name.clone(),
            new_name,
        });
    }
    // Report the log ascending by new position (the renames ran highest-first).
    renumbers.sort_by(|a, b| a.new_position.cmp(&b.new_position));

    let id = LeafId {
        position: target,
        key: Some(new_key),
        slug: slug.to_string(),
        is_brief: false,
        is_done: false,
    };
    let path = grove_root.join(id.filename());
    if path.exists() {
        bail!(
            "destination already exists after renumber: {} (renumber log: {:?})",
            path.display(),
            renumbers
        );
    }
    write_template(&path, &id, kind)?;
    Ok((path, renumbers))
}

/// Surface stray **positional** cross-references left stale by a `leaf_insert`
/// renumber, as a lint on stderr — never an auto-rewrite (D6: durable references
/// should use the stable `[key]`, not the position, so the operator reviews each
/// occurrence and decides). Scans every `.md` file's body (the first-line header
/// is skipped — it is the file's own, already-correct identity) for a dotted
/// token equal to one of the **old** positions the renumber moved, emitting one
/// `path:line: <pos> (context)` per hit.
///
/// Deliberately a *light* lint (D6): only **multi-segment** positions (`2.2`,
/// `2.2.1`) are surfaced — bare single integers (`2`, `3`) are far too common in
/// prose to lint usefully — and a date/version that happens to parse as a moved
/// position is an accepted false positive (the operator reviews). Empty renumber
/// log ⇒ nothing to do.
pub fn surface_cross_refs(
    grove_root: &Path,
    renumbers: &[Renumber],
    out: &mut impl std::io::Write,
) -> Result<()> {
    if renumbers.is_empty() {
        return Ok(());
    }
    // The stale references are the *old* positions the renumber moved — and only
    // the multi-segment ones (single integers are too common in prose to lint).
    let stale: std::collections::HashSet<Vec<u32>> = renumbers
        .iter()
        .map(|r| r.old_position.clone())
        .filter(|p| p.len() >= 2)
        .collect();
    if stale.is_empty() {
        return Ok(());
    }

    let mut files: Vec<PathBuf> = std::fs::read_dir(grove_root)
        .with_context(|| format!("reading {}", grove_root.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("md"))
        .collect();
    files.sort();

    for path in &files {
        let body = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => continue,
        };
        // Skip line 1 (the `# …` header — the file's own, already-correct id);
        // only prose references in the body can be stale.
        for (idx, line) in body.lines().enumerate().skip(1) {
            for token in find_position_tokens(line) {
                if stale.contains(&token) {
                    writeln!(
                        out,
                        "{}:{}: {} ({})",
                        path.display(),
                        idx + 1,
                        join_position(&token),
                        line.trim()
                    )
                    .ok();
                }
            }
        }
    }
    Ok(())
}

/// Render a position vector as its dotted string (`[2,3,1]` → `2.3.1`).
fn join_position(position: &[u32]) -> String {
    position
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".")
}

// ---------------------------------------------------------------------------
// helpers

/// Read the flat `.grove/` directory and parse every regular file, returning the
/// `(LeafId, filename)` of each that parses (foreign/malformed files excluded —
/// leniency, D2), sorted by the 010 version-sort comparator so the list is DFS
/// pre-order. Subdirectories are never tasks in the flat scheme.
fn scan(grove_root: &Path) -> Result<Vec<(LeafId, String)>> {
    if !grove_root.is_dir() {
        bail!("grove root not found: {}", grove_root.display());
    }
    let mut entries: Vec<(LeafId, String)> = Vec::new();
    for entry in std::fs::read_dir(grove_root)
        .with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        match entry.file_type() {
            Ok(t) if t.is_file() => {}
            _ => continue,
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if let Some(id) = parse(&name) {
            entries.push((id, name));
        }
    }
    entries.sort_by(|a, b| sort_key(&a.1).cmp(&sort_key(&b.1)));
    Ok(entries)
}

/// Parse a `<parent-id>` positional: `.` is the root (empty position); anything
/// else is a dotted position whose every segment must be ≥ 1 (positions are
/// 1-based, so a `0` segment is malformed).
fn parse_parent_id(parent_id: &str) -> Result<Vec<u32>> {
    if parent_id == "." {
        return Ok(Vec::new());
    }
    let position =
        parse_position(parent_id).with_context(|| format!("invalid parent id {parent_id:?}"))?;
    require_positive(&position, parent_id)?;
    Ok(position)
}

/// Parse a `<target-id>` positional for `leaf_insert`: a concrete dotted position
/// (no `.` root form — you insert *at* a position), non-empty, every segment ≥ 1.
fn parse_target_id(target_id: &str) -> Result<Vec<u32>> {
    let position =
        parse_position(target_id).with_context(|| format!("invalid target id {target_id:?}"))?;
    require_positive(&position, target_id)?;
    Ok(position)
}

/// Reject a position with any `0` segment (positions are 1-based).
fn require_positive(position: &[u32], shown: &str) -> Result<()> {
    if position.contains(&0) {
        bail!("invalid id {shown:?}: position segments are 1-based (no `0`)");
    }
    Ok(())
}

/// `git mv <src> <dst>` within `grove_root`. Renames are tracked moves (parity
/// with the old verbs) so the enclosing task's commit folds them in cleanly.
fn git_mv(grove_root: &Path, src: &str, dst: &str) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(grove_root)
        .args(["mv", src, dst])
        .output()
        .with_context(|| format!("running git mv {src} {dst}"))?;
    if !out.status.success() {
        bail!(
            "git mv {src} -> {dst} failed: {}",
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// Rewrite the first-line `# …` header of a renamed file so its position matches
/// the new on-disk name. Conservative: a first line that is not a new-format
/// header is left untouched (see [`rewrite_header_line`]).
fn rewrite_header_position(path: &Path, new_position: &[u32]) -> Result<()> {
    let body =
        std::fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let new_first = match rewrite_header_line(first, new_position) {
        Some(s) => s,
        None => return Ok(()),
    };
    let mut out = String::with_capacity(body.len());
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    std::fs::write(path, out.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// Extract every dotted-decimal **positional token** from a line: a maximal run
/// of `[0-9.]` that is bounded by non-alphanumeric on both sides (so `v1.2` and
/// `2.2x` are not tokens), with trailing `.` trimmed (so `see 2.2.` matches),
/// parsed as an `int('.'int)*` position. Multi-segment dates split on `-`
/// (`2026-06-21` → three single-int tokens, all dropped by the caller's
/// multi-segment filter). Invalid runs (`2..3`) are skipped.
fn find_position_tokens(line: &str) -> Vec<Vec<u32>> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let is_start = bytes[i].is_ascii_digit()
            && (i == 0 || !(bytes[i - 1].is_ascii_alphanumeric() || bytes[i - 1] == b'.'));
        if !is_start {
            i += 1;
            continue;
        }
        let start = i;
        let mut j = i;
        while j < bytes.len() && (bytes[j].is_ascii_digit() || bytes[j] == b'.') {
            j += 1;
        }
        let followed_by_alnum = j < bytes.len() && bytes[j].is_ascii_alphanumeric();
        if !followed_by_alnum {
            if let Some(v) = parse_position(line[start..j].trim_end_matches('.')) {
                out.push(v);
            }
        }
        i = j; // j > start (we started on a digit), so this always advances
    }
    out
}

/// Rewrite **only the dotted position** of a `# <position>-[<key>]-<slug>…`
/// first-line header to `new_position`, preserving the `-[<key>]-<slug>` tail and
/// any ` — brief` suffix. Returns `None` (leave the line alone) for anything that
/// is not a new-format header — a header whose leading dotted run is not followed
/// by `-[` (a key). Conservative by design: the renumber never touches the key or
/// slug (D6), and a hand-edited or old-format title stays untouched.
fn rewrite_header_line(line: &str, new_position: &[u32]) -> Option<String> {
    let body = line.strip_prefix("# ")?;
    // The position is the leading dotted-int run, ending at the first `-`.
    let dash = body.find('-')?;
    parse_position(&body[..dash])?; // only rewrite if the prefix is a real position
    let tail = &body[dash..]; // starts with the `-` before `[<key>]`
    if !tail.starts_with("-[") {
        return None; // not a new-format `# <pos>-[<key>]-…` header — leave alone
    }
    Some(format!("# {}{tail}", join_position(new_position)))
}

/// Write the leaf template for a freshly-created leaf. The first-line header is
/// the filename stem (`# <position>-[<key>]-<slug>`, D5/D6) so header and name
/// agree by construction.
fn write_template(path: &Path, id: &LeafId, kind: Kind) -> Result<()> {
    let stem = id
        .filename()
        .strip_suffix(".md")
        .expect("filename always ends .md")
        .to_string();
    let kind_label = match kind {
        Kind::Work => "work",
        Kind::Planning => "planning",
    };
    let body = format!(
        "# {stem}\n\n**Kind:** {kind_label}\n\n## Goal\n\n## Context\n\n## Done when\n\n## Notes\n",
    );
    std::fs::write(path, body.as_bytes()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::process::Command;
    use tempfile::TempDir;

    /// A fresh `.grove/` directory (no git — `leaf_add` writes, never `git mv`s).
    fn grove() -> (TempDir, PathBuf) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    /// A `.grove/` inside a real git repo — `leaf_insert` renames via `git mv`,
    /// which needs tracked files (call [`stage_all`] before inserting).
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

    /// Stage everything under the grove so `git mv` sees tracked files.
    fn stage_all(root: &Path) {
        run_git(root.parent().unwrap(), &["add", "-A"]);
    }

    fn touch(root: &Path, name: &str) -> PathBuf {
        let p = root.join(name);
        fs::write(&p, format!("# {}\n", name.strip_suffix(".md").unwrap())).unwrap();
        p
    }

    /// Write a file with explicit body (for realistic `# … — brief` headers).
    fn touch_body(root: &Path, name: &str, content: &str) -> PathBuf {
        let p = root.join(name);
        fs::write(&p, content).unwrap();
        p
    }

    fn name_of(p: &Path) -> String {
        p.file_name().unwrap().to_string_lossy().into_owned()
    }

    fn body(p: &Path) -> String {
        fs::read_to_string(p).unwrap()
    }

    /// The grove's regular-file names, lexically sorted.
    fn list(root: &Path) -> Vec<String> {
        let mut v: Vec<String> = fs::read_dir(root)
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_file()).unwrap_or(false))
            .map(|e| e.file_name().to_string_lossy().into_owned())
            .collect();
        v.sort();
        v
    }

    // ---- leaf-add -----------------------------------------------------------

    #[test]
    fn add_root_level_child_gets_position_one_and_first_key() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let got = leaf_add(&g, ".", "survey", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "1-[1]-survey.md");
    }

    #[test]
    fn add_appends_gapless_after_existing_root_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-a.md");
        touch(&g, "2-[2]-b.md");
        let got = leaf_add(&g, ".", "c", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "3-[3]-c.md");
    }

    #[test]
    fn add_child_under_a_node_appends_after_existing_children() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[2]-build.BRIEF.md");
        touch(&g, "2.1-[3]-x.md");
        let got = leaf_add(&g, "2", "y", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "2.2-[4]-y.md");
    }

    #[test]
    fn add_first_child_under_a_childless_node() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[2]-build.BRIEF.md");
        let got = leaf_add(&g, "2", "first", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "2.1-[3]-first.md");
    }

    #[test]
    fn add_assigns_fresh_key_as_max_over_whole_tree_plus_one() {
        // Keys are global, not per-node: the new key is max([n]) + 1 across the
        // whole tree, including a deeper subtree's higher key.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-a.BRIEF.md");
        touch(&g, "1.1-[7]-deep.md"); // a high key in another subtree
        touch(&g, "2-[2]-build.BRIEF.md");
        let got = leaf_add(&g, "2", "y", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "2.1-[8]-y.md");
    }

    #[test]
    fn add_counts_done_children_so_a_retired_slot_is_never_reused() {
        // A `.DONE` child still occupies its index — the next child is 2, not 1.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[2]-build.BRIEF.md");
        touch(&g, "2.1-[3]-x.DONE.md");
        let got = leaf_add(&g, "2", "y", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "2.2-[4]-y.md");
    }

    #[test]
    fn add_ignores_a_deeper_descendant_when_counting_direct_children() {
        // A grandchild `2.1.1` must not be mistaken for a direct child of `2`.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[2]-build.BRIEF.md");
        touch(&g, "2.1-[3]-mid.BRIEF.md");
        touch(&g, "2.1.1-[4]-deep.md");
        let got = leaf_add(&g, "2", "y", Kind::Work).unwrap();
        assert_eq!(name_of(&got), "2.2-[5]-y.md");
    }

    #[test]
    fn add_writes_new_format_header_and_kind() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let got = leaf_add(&g, ".", "survey", Kind::Work).unwrap();
        let text = body(&got);
        assert!(
            text.starts_with("# 1-[1]-survey\n"),
            "header should be the filename stem; got {text:?}"
        );
        assert!(text.contains("**Kind:** work"), "got {text:?}");
    }

    #[test]
    fn add_planning_kind_writes_planning_label() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let got = leaf_add(&g, ".", "design", Kind::Planning).unwrap();
        assert!(body(&got).contains("**Kind:** planning"));
    }

    #[test]
    fn add_errors_when_parent_node_is_missing() {
        // No brief at [9] — adding a child to a non-existent node is a typo guard.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        let err = leaf_add(&g, "9", "y", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("parent node"), "got {err}");
    }

    #[test]
    fn add_errors_when_parent_is_a_leaf_not_a_node() {
        // `2` is a live leaf (no brief) — you must decompose it before adding under it.
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[2]-build.md");
        let err = leaf_add(&g, "2", "y", Kind::Work).unwrap_err();
        assert!(err.to_string().contains("parent node"), "got {err}");
    }

    #[test]
    fn add_errors_on_invalid_slug() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        assert!(leaf_add(&g, ".", "BRIEF", Kind::Work).is_err());
        assert!(leaf_add(&g, ".", "Bad Slug", Kind::Work).is_err());
    }

    #[test]
    fn add_errors_on_malformed_or_zero_parent_id() {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md");
        assert!(leaf_add(&g, "2..3", "y", Kind::Work).is_err());
        assert!(leaf_add(&g, "2.0", "y", Kind::Work).is_err()); // zero segment
    }

    #[test]
    fn add_errors_when_grove_root_absent() {
        let (_t, g) = grove();
        let missing = g.join("nope");
        let err = leaf_add(&missing, ".", "y", Kind::Work).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- leaf-insert --------------------------------------------------------

    #[test]
    fn insert_degenerate_no_siblings_to_shift_just_creates_at_position() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "2-[1]-build.BRIEF.md");
        touch(&g, "2.1-[2]-a.md");
        touch(&g, "2.2-[3]-b.md");
        stage_all(&g);
        let (path, renums) = leaf_insert(&g, "2.3", "c", Kind::Work).unwrap();
        assert_eq!(name_of(&path), "2.3-[4]-c.md");
        assert!(
            renums.is_empty(),
            "nothing at/after 2.3 to shift; got {renums:?}"
        );
    }

    #[test]
    fn insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-a.md");
        touch(&g, "2-[2]-b.md");
        touch(&g, "3-[3]-c.md");
        stage_all(&g);
        let (path, _renums) = leaf_insert(&g, "2", "new", Kind::Work).unwrap();
        assert_eq!(name_of(&path), "2-[4]-new.md"); // fresh key, not a reused one
        let files = list(&g);
        assert!(
            files.contains(&"1-[1]-a.md".to_string()),
            "pos 1 < 2, unchanged"
        );
        assert!(
            files.contains(&"2-[4]-new.md".to_string()),
            "the inserted leaf"
        );
        assert!(
            files.contains(&"3-[2]-b.md".to_string()),
            "b: 2->3, key [2] kept"
        );
        assert!(
            files.contains(&"4-[3]-c.md".to_string()),
            "c: 3->4, key [3] kept"
        );
        assert!(!files.contains(&"2-[2]-b.md".to_string()), "old name gone");
    }

    #[test]
    fn insert_cascades_through_whole_subtrees() {
        // The headline: bumping the node `2.2`→`2.3` drags its whole subtree.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch_body(&g, "2-[1]-build.BRIEF.md", "# 2-[1]-build — brief\n");
        touch(&g, "2.1-[2]-a.md");
        touch_body(&g, "2.2-[3]-mid.BRIEF.md", "# 2.2-[3]-mid — brief\n");
        touch(&g, "2.2.1-[4]-x.md");
        touch(&g, "2.2.2-[5]-y.md");
        touch(&g, "2.3-[6]-z.md");
        stage_all(&g);
        let (path, renums) = leaf_insert(&g, "2.2", "new", Kind::Work).unwrap();
        assert_eq!(name_of(&path), "2.2-[7]-new.md");
        let files = list(&g);
        assert!(
            files.contains(&"2.1-[2]-a.md".to_string()),
            "sibling at seg 1 < 2: unchanged"
        );
        assert!(
            files.contains(&"2.2-[7]-new.md".to_string()),
            "new leaf in the vacated slot"
        );
        // node `mid` and its children all dragged from 2.2.* to 2.3.*, keys+slugs kept:
        assert!(files.contains(&"2.3-[3]-mid.BRIEF.md".to_string()));
        assert!(files.contains(&"2.3.1-[4]-x.md".to_string()));
        assert!(files.contains(&"2.3.2-[5]-y.md".to_string()));
        // later sibling `z` shifted 2.3 -> 2.4, key [6] kept:
        assert!(files.contains(&"2.4-[6]-z.md".to_string()));
        for gone in [
            "2.2-[3]-mid.BRIEF.md",
            "2.2.1-[4]-x.md",
            "2.2.2-[5]-y.md",
            "2.3-[6]-z.md",
        ] {
            assert!(!files.contains(&gone.to_string()), "{gone} should be gone");
        }
        assert_eq!(renums.len(), 4, "mid + x + y + z moved");
    }

    #[test]
    fn insert_rewrites_position_in_headers_never_key_or_slug() {
        // The explicitly-flagged risk (030 Notes): a renumber must touch ONLY the
        // position — a header whose `[key]` moved would break D6's reference
        // stability.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch_body(&g, "2-[1]-build.BRIEF.md", "# 2-[1]-build — brief\n");
        touch_body(
            &g,
            "2.2-[3]-mid.BRIEF.md",
            "# 2.2-[3]-mid — brief\n\nbody\n",
        );
        touch_body(&g, "2.2.1-[4]-x.md", "# 2.2.1-[4]-x\n\nbody\n");
        stage_all(&g);
        leaf_insert(&g, "2.2", "new", Kind::Work).unwrap();
        assert_eq!(
            body(&g.join("2.3-[3]-mid.BRIEF.md"))
                .lines()
                .next()
                .unwrap(),
            "# 2.3-[3]-mid — brief",
            "brief: position bumped, key/slug/suffix intact"
        );
        assert_eq!(
            body(&g.join("2.3.1-[4]-x.md")).lines().next().unwrap(),
            "# 2.3.1-[4]-x",
            "child: position bumped, key/slug intact"
        );
    }

    #[test]
    fn insert_at_root_level_shifts_all_root_siblings() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        touch(&g, "1-[1]-a.md");
        touch(&g, "2-[2]-b.md");
        stage_all(&g);
        let (path, _r) = leaf_insert(&g, "1", "x", Kind::Work).unwrap();
        assert_eq!(name_of(&path), "1-[3]-x.md");
        let files = list(&g);
        assert!(files.contains(&"1-[3]-x.md".to_string()));
        assert!(files.contains(&"2-[1]-a.md".to_string()));
        assert!(files.contains(&"3-[2]-b.md".to_string()));
    }

    #[test]
    fn insert_collision_free_for_a_dense_run_of_siblings() {
        // Stress the highest-first ordering: insert at the head of five siblings;
        // a wrong order would make a `git mv` collide and lose a file.
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        for i in 1..=5 {
            touch(&g, &format!("{i}-[{i}]-s{i}.md"));
        }
        stage_all(&g);
        let (path, renums) = leaf_insert(&g, "1", "head", Kind::Work).unwrap();
        assert_eq!(name_of(&path), "1-[6]-head.md");
        assert_eq!(renums.len(), 5);
        let leaves: Vec<String> = list(&g)
            .into_iter()
            .filter(|n| !n.ends_with("BRIEF.md"))
            .collect();
        assert_eq!(
            leaves,
            vec![
                "1-[6]-head.md",
                "2-[1]-s1.md",
                "3-[2]-s2.md",
                "4-[3]-s3.md",
                "5-[4]-s4.md",
                "6-[5]-s5.md",
            ],
            "all six leaves present, gapless 1..6 — no file lost to a collision"
        );
    }

    #[test]
    fn insert_errors_on_invalid_inputs() {
        let (_t, g) = git_grove();
        touch(&g, "BRIEF.md");
        stage_all(&g);
        assert!(
            leaf_insert(&g, "1", "BRIEF", Kind::Work).is_err(),
            "reserved slug"
        );
        assert!(
            leaf_insert(&g, ".", "x", Kind::Work).is_err(),
            "target must be concrete"
        );
        assert!(
            leaf_insert(&g, "1..2", "x", Kind::Work).is_err(),
            "malformed id"
        );
        assert!(
            leaf_insert(&g, "0", "x", Kind::Work).is_err(),
            "zero segment"
        );
    }

    #[test]
    fn insert_errors_when_grove_root_absent() {
        let (_t, g) = git_grove();
        let missing = g.join("nope");
        let err = leaf_insert(&missing, "1", "x", Kind::Work).unwrap_err();
        assert!(
            err.to_string().contains("grove root not found"),
            "got {err}"
        );
    }

    // ---- rewrite_header_line (position-only, key/slug preserved) -------------

    #[test]
    fn header_rewrite_replaces_position_keeps_key_and_slug() {
        assert_eq!(
            rewrite_header_line("# 2.2-[3]-mid", &[2, 3]),
            Some("# 2.3-[3]-mid".to_string())
        );
    }

    #[test]
    fn header_rewrite_handles_multi_segment_position() {
        assert_eq!(
            rewrite_header_line("# 2.2.1-[4]-x", &[2, 3, 1]),
            Some("# 2.3.1-[4]-x".to_string())
        );
    }

    #[test]
    fn header_rewrite_preserves_brief_suffix() {
        assert_eq!(
            rewrite_header_line("# 2.2-[3]-verbs — brief", &[2, 3]),
            Some("# 2.3-[3]-verbs — brief".to_string())
        );
    }

    #[test]
    fn header_rewrite_preserves_a_slug_with_dashes() {
        assert_eq!(
            rewrite_header_line("# 1-[5]-multi-word-slug", &[2]),
            Some("# 2-[5]-multi-word-slug".to_string())
        );
    }

    #[test]
    fn header_rewrite_is_none_for_non_new_format_headers() {
        // No `[key]` after the position → not a new-format header; leave it alone.
        assert_eq!(rewrite_header_line("# 2.2-no-bracket", &[2, 3]), None);
        // Old-format `NNN-slug` header (no key) — untouched (robustness).
        assert_eq!(rewrite_header_line("# 010-old-format", &[2]), None);
        // Not a header at all.
        assert_eq!(rewrite_header_line("# Some Title", &[2]), None);
        assert_eq!(rewrite_header_line("Not a header", &[2]), None);
    }

    // ---- find_position_tokens (the lint's tokenizer) ------------------------

    #[test]
    fn tokens_extracts_dotted_positions() {
        assert_eq!(
            find_position_tokens("see 2.2 for details"),
            vec![vec![2, 2]]
        );
        assert_eq!(
            find_position_tokens("1.2.3 and 2.2"),
            vec![vec![1, 2, 3], vec![2, 2]]
        );
    }

    #[test]
    fn tokens_trims_a_trailing_sentence_dot() {
        // `see 2.2.` (end of sentence) still yields [2,2], not a reject.
        assert_eq!(
            find_position_tokens("renumbered from 2.2."),
            vec![vec![2, 2]]
        );
    }

    #[test]
    fn tokens_ignores_versions_and_identifiers() {
        // A leading letter (version `v1.2`) or a trailing letter (`2.2x`) → no token.
        assert!(find_position_tokens("upgraded to v1.2").is_empty());
        assert!(find_position_tokens("2.2x is not an id").is_empty());
    }

    #[test]
    fn tokens_splits_dates_into_single_ints_only() {
        // `2026-06-21` has no dots → three single-int tokens (dropped by the
        // caller's multi-segment filter), never a multi-segment position.
        assert_eq!(
            find_position_tokens("on 2026-06-21"),
            vec![vec![2026], vec![6], vec![21]]
        );
    }

    #[test]
    fn tokens_skips_malformed_runs() {
        assert!(find_position_tokens("2..3 is malformed").is_empty());
    }

    // ---- surface_cross_refs (positional lint, not auto-rewrite) -------------

    /// A renumber log entry, for driving the lint without a real insert.
    fn renum(old: &[u32], new: &[u32]) -> Renumber {
        Renumber {
            old_position: old.to_vec(),
            new_position: new.to_vec(),
            old_name: format!("{}-x.md", join(old)),
            new_name: format!("{}-x.md", join(new)),
        }
    }
    fn join(v: &[u32]) -> String {
        v.iter().map(u32::to_string).collect::<Vec<_>>().join(".")
    }
    fn surfaced(root: &Path, renumbers: &[Renumber]) -> String {
        let mut buf = Vec::new();
        surface_cross_refs(root, renumbers, &mut buf).unwrap();
        String::from_utf8(buf).unwrap()
    }

    #[test]
    fn surface_empty_renumbers_emits_nothing() {
        let (_t, g) = grove();
        touch_body(&g, "1-[1]-a.md", "# 1-[1]-a\n\nrefers to 2.2 somewhere\n");
        assert_eq!(surfaced(&g, &[]), "");
    }

    #[test]
    fn surface_reports_a_stray_positional_reference_in_a_body() {
        let (_t, g) = grove();
        touch_body(&g, "1-[1]-a.md", "# 1-[1]-a\n\nsee 2.2 for the design\n");
        let out = surfaced(&g, &[renum(&[2, 2], &[2, 3])]);
        assert!(out.contains("1-[1]-a.md"), "names the file: {out:?}");
        assert!(out.contains("2.2"), "shows the stale position: {out:?}");
        assert!(
            out.contains(":3:"),
            "1-based line number of the body ref: {out:?}"
        );
    }

    #[test]
    fn surface_skips_the_self_referential_header_line() {
        // The new leaf sits at the old position `2.2`, so its OWN header contains
        // `2.2` — that must not be linted (only bodies are scanned).
        let (_t, g) = grove();
        touch_body(&g, "2.2-[7]-new.md", "# 2.2-[7]-new\n\n## Goal\n");
        assert_eq!(surfaced(&g, &[renum(&[2, 2], &[2, 3])]), "");
    }

    #[test]
    fn surface_ignores_single_segment_positions_as_too_noisy() {
        // A root-level insert moves `[3]`→`[4]` and `[3,1]`→`[4,1]`. A body "see 3
        // and 3.1" surfaces only the multi-segment `3.1`.
        let (_t, g) = grove();
        touch_body(&g, "1-[1]-a.md", "# 1-[1]-a\n\nsee 3 and 3.1 here\n");
        let out = surfaced(&g, &[renum(&[3], &[4]), renum(&[3, 1], &[4, 1])]);
        assert!(out.contains("3.1"), "multi-segment ref surfaced: {out:?}");
        // The bare `3` line is reported only for `3.1`; `3` alone is not a hit.
        assert_eq!(
            out.lines().count(),
            1,
            "exactly one hit (3.1), not bare 3: {out:?}"
        );
    }

    #[test]
    fn surface_does_not_match_dates_or_versions_as_moved_positions() {
        let (_t, g) = grove();
        touch_body(
            &g,
            "1-[1]-a.md",
            "# 1-[1]-a\n\nreleased 2.0.1 on 2026-06-21\n",
        );
        // The renumber moved [2,2]; neither the date nor the version equals it.
        assert_eq!(surfaced(&g, &[renum(&[2, 2], &[2, 3])]), "");
    }

    #[test]
    fn surface_reports_hits_across_multiple_files_including_briefs() {
        let (_t, g) = grove();
        touch_body(&g, "BRIEF.md", "# root\n\nthe plan lives at 2.2\n");
        touch_body(&g, "1-[1]-a.md", "# 1-[1]-a\n\nalso points at 2.2.1\n");
        let out = surfaced(
            &g,
            &[renum(&[2, 2], &[2, 3]), renum(&[2, 2, 1], &[2, 3, 1])],
        );
        assert!(out.contains("BRIEF.md") && out.contains("2.2"), "{out:?}");
        assert!(
            out.contains("1-[1]-a.md") && out.contains("2.2.1"),
            "{out:?}"
        );
    }
}

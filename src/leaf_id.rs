// The new-format **id model**: the foundational parsed-filename type plus the
// pure functions every dotted-decimal verb (020 read / 030 grow / 040 lifecycle)
// consumes. Implements the grammar settled in task-tree-scheme and the 050 brief's
// running log D1/D2/D3/D6:
//
//     <position>-[<key>]-<slug>[.BRIEF|.DONE].md      (root brief: BRIEF.md)
//
// Three orthogonal parts: the dotted **position** vector (where it sits in DFS
// pre-order; the sole sort input; rewritten on renumber), the bracketed
// permanent **key** (stable identity, assigned once, never rewritten), and the
// human **slug**.
//
// Per D9 this module is **new, isolated code**: it does NOT touch the live verb
// path (`src/leaf.rs`, `src/pick.rs`), which keeps speaking the old `NNN-slug`
// format until leaf 060 wires the dual-format dispatch in. Pure functions only,
// dependency-free of the verb modules.

use anyhow::{bail, Result};

/// A filename parsed into its dotted-decimal parts. A *leaf* is `is_brief =
/// is_done = false`; a *node brief* has `is_brief = true`; a *retired leaf* has
/// `is_done = true` (D4: briefs are never done, so the two flags never coincide).
/// The root brief `BRIEF.md` is the one unkeyed singleton: empty `position`,
/// `key = None`, empty `slug`, `is_brief = true`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LeafId {
    /// The dotted position vector, e.g. `2.3.1` → `[2, 3, 1]`. Empty for the
    /// root brief. The *only* input to the sort comparator (D2).
    pub position: Vec<u32>,
    /// The permanent identity key `[n]`. `None` only for the root brief.
    pub key: Option<u32>,
    /// The human slug. Empty for the root brief.
    pub slug: String,
    /// `true` for a node brief (`.BRIEF`) and the root brief.
    pub is_brief: bool,
    /// `true` for a retired leaf (`.DONE`). Never `true` together with
    /// `is_brief` (D4).
    pub is_done: bool,
}

impl LeafId {
    /// Render this id back to its on-disk filename. Inverse of [`parse`] for
    /// any well-formed id (round-trip: parts → filename → parse → same parts).
    pub fn filename(&self) -> String {
        // The root brief is the one unkeyed, position-less singleton.
        if self.position.is_empty() && self.is_brief {
            return "BRIEF.md".to_string();
        }
        let mut s = self
            .position
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join(".");
        if let Some(k) = self.key {
            s.push_str(&format!("-[{}]", k));
        }
        s.push('-');
        s.push_str(&self.slug);
        // is_brief and is_done never coincide (D4), so order is immaterial.
        if self.is_brief {
            s.push_str(".BRIEF");
        } else if self.is_done {
            s.push_str(".DONE");
        }
        s.push_str(".md");
        s
    }

    /// A *live leaf* is the only thing `pick` returns: not a brief, not retired.
    pub fn is_live_leaf(&self) -> bool {
        !self.is_brief && !self.is_done
    }
}

/// Parse a bare filename (not a path) into a [`LeafId`]. **Lenient** (D2): a
/// name that does not match the grammar is "not a leaf/brief" and yields
/// `None` — never a panic or error, so a stray `README.md` or half-written
/// file never jams the loop.
pub fn parse(name: &str) -> Option<LeafId> {
    // The root brief is the one unkeyed singleton (D6).
    if name == "BRIEF.md" {
        return Some(LeafId {
            position: Vec::new(),
            key: None,
            slug: String::new(),
            is_brief: true,
            is_done: false,
        });
    }

    let stem = name.strip_suffix(".md")?;

    // Peel at most one marker (D4: .BRIEF and .DONE never coincide on one file).
    let (stem, is_brief, is_done) = if let Some(s) = stem.strip_suffix(".BRIEF") {
        (s, true, false)
    } else if let Some(s) = stem.strip_suffix(".DONE") {
        (s, false, true)
    } else {
        (stem, false, false)
    };

    // The position is the leading `int('.'int)*` run with no dashes, so the
    // first `-` is its unambiguous end.
    let dash = stem.find('-')?;
    let position = parse_position(&stem[..dash])?;
    let rest = &stem[dash + 1..];

    // The key: `[<digits>]`, the first `]` closing it (the slug forbids `]`).
    let rest = rest.strip_prefix('[')?;
    let close = rest.find(']')?;
    let key_str = &rest[..close];
    if key_str.is_empty() || !key_str.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    let key: u32 = key_str.parse().ok()?;

    // The slug: `-<slug>`, validated under the new grammar.
    let slug = rest[close + 1..].strip_prefix('-')?;
    if validate_slug(slug).is_err() {
        return None;
    }

    Some(LeafId {
        position,
        key: Some(key),
        slug: slug.to_string(),
        is_brief,
        is_done,
    })
}

/// Parse a dotted-decimal position string (`2.3.1`) into its integer vector.
/// Strict: each `.`-separated part must be non-empty digits (so `2..3`, `1.`,
/// and a leading/trailing `.` all reject) and an empty string rejects. Public so
/// the grow verbs (030) can parse a bare `<parent-id>`/`<target-id>` positional
/// and the position prefix of a `# …` header (the id model is the foundation
/// every verb consumes).
pub fn parse_position(s: &str) -> Option<Vec<u32>> {
    if s.is_empty() {
        return None;
    }
    let mut v = Vec::new();
    for part in s.split('.') {
        if part.is_empty() || !part.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        v.push(part.parse().ok()?);
    }
    Some(v)
}

/// The total-order sort key for a filename. Orders by the **position vector
/// only** (D2): element-wise integer compare, shorter-prefix-first, so the flat
/// sorted list is the DFS pre-order. Foreign / malformed names are "not a leaf"
/// and sort **last**; a same-position collision (a malformed tree) breaks
/// deterministically by the full filename.
pub fn sort_key(name: &str) -> (bool, Vec<u32>, String) {
    // Tuple Ord does the work: `false < true` puts parseable names first;
    // `Vec<u32>` compares element-wise with shorter-prefix-first (D2); the
    // filename is the deterministic tie-break for a same-position collision and
    // the order among foreign names.
    match parse(name) {
        Some(id) => (false, id.position, name.to_string()),
        None => (true, Vec::new(), name.to_string()),
    }
}

/// The next permanent key for a tree: `max([n] over all files, live AND .DONE)
/// + 1`, or `1` for an empty tree. `.DONE` files stay in the tree (D3/D6) so the
///   max is always visible and keys are never reused — there is no counter file.
pub fn next_key<I, S>(names: I) -> u32
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    names
        .into_iter()
        .filter_map(|n| parse(n.as_ref()).and_then(|id| id.key))
        .max()
        .map_or(1, |m| m + 1)
}

/// Validate a slug under the new grammar (D6): non-empty, lowercase ASCII +
/// digits + `-`, no leading/trailing `-`, and not the reserved words `BRIEF` /
/// `DONE`. The character set already excludes `.`, `[`, and `]`.
pub fn validate_slug(slug: &str) -> Result<()> {
    if slug.is_empty() {
        bail!("slug must not be empty");
    }
    if slug == "BRIEF" || slug == "DONE" {
        bail!("slug {:?} is reserved", slug);
    }
    if slug.starts_with('-') || slug.ends_with('-') {
        bail!("slug must not start or end with a dash: {:?}", slug);
    }
    for c in slug.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            bail!(
                "slug must contain only lowercase ASCII letters, digits, and dashes (no `.`/`[`/`]`): {:?}",
                slug
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(position: Vec<u32>, key: u32, slug: &str) -> LeafId {
        LeafId {
            position,
            key: Some(key),
            slug: slug.to_string(),
            is_brief: false,
            is_done: false,
        }
    }

    // ---- parse: well-formed -------------------------------------------------

    #[test]
    fn parse_full_leaf() {
        assert_eq!(
            parse("2.3.1-[4]-add.md"),
            Some(leaf(vec![2, 3, 1], 4, "add"))
        );
    }

    #[test]
    fn parse_root_level_leaf() {
        assert_eq!(parse("1-[1]-survey.md"), Some(leaf(vec![1], 1, "survey")));
    }

    #[test]
    fn parse_node_brief() {
        assert_eq!(
            parse("2.3-[2]-verbs.BRIEF.md"),
            Some(LeafId {
                position: vec![2, 3],
                key: Some(2),
                slug: "verbs".to_string(),
                is_brief: true,
                is_done: false,
            })
        );
    }

    #[test]
    fn parse_done_leaf() {
        assert_eq!(
            parse("2.3.1-[4]-add.DONE.md"),
            Some(LeafId {
                position: vec![2, 3, 1],
                key: Some(4),
                slug: "add".to_string(),
                is_brief: false,
                is_done: true,
            })
        );
    }

    #[test]
    fn parse_root_brief_is_unkeyed() {
        assert_eq!(
            parse("BRIEF.md"),
            Some(LeafId {
                position: Vec::new(),
                key: None,
                slug: String::new(),
                is_brief: true,
                is_done: false,
            })
        );
    }

    #[test]
    fn parse_multidigit_position_and_key() {
        assert_eq!(
            parse("2.10-[12]-foo.md"),
            Some(leaf(vec![2, 10], 12, "foo"))
        );
    }

    #[test]
    fn parse_slug_may_contain_dashes() {
        assert_eq!(
            parse("1-[1]-multi-word-slug.md"),
            Some(leaf(vec![1], 1, "multi-word-slug"))
        );
    }

    // ---- parse: lenient (None) ----------------------------------------------

    #[test]
    fn parse_old_format_is_not_new_format() {
        // Old `NNN-slug` has no `[key]` — must not parse as new-format.
        assert_eq!(parse("010-id-model.md"), None);
    }

    #[test]
    fn parse_foreign_files_are_none() {
        assert_eq!(parse("README.md"), None);
        assert_eq!(parse("notes.txt"), None);
        assert_eq!(parse(".gitkeep"), None);
    }

    #[test]
    fn parse_keyless_positioned_is_none() {
        assert_eq!(parse("2.3-add.md"), None);
    }

    #[test]
    fn parse_empty_or_nondigit_key_is_none() {
        assert_eq!(parse("1-[]-x.md"), None);
        assert_eq!(parse("1-[a]-x.md"), None);
        assert_eq!(parse("1-[1a]-x.md"), None);
    }

    #[test]
    fn parse_malformed_position_is_none() {
        assert_eq!(parse("1..2-[1]-x.md"), None);
        assert_eq!(parse("-[1]-x.md"), None);
        assert_eq!(parse("1.-[1]-x.md"), None);
    }

    #[test]
    fn parse_slug_with_dot_or_bracket_is_none() {
        assert_eq!(parse("1-[1]-a.b.md"), None);
        assert_eq!(parse("1-[1]-a]b.md"), None);
    }

    #[test]
    fn parse_double_marker_is_none() {
        // Only one marker is peeled (D4); a second leaks into the slug → invalid.
        assert_eq!(parse("1-[1]-x.BRIEF.DONE.md"), None);
    }

    // ---- comparator ---------------------------------------------------------

    #[test]
    fn compare_numeric_not_lexical() {
        // [2,9] < [2,10] — the exact case the old zero-padded lexical sort fails.
        assert!(sort_key("2.9-[1]-a.md") < sort_key("2.10-[2]-b.md"));
    }

    #[test]
    fn compare_shorter_prefix_first() {
        // A node's brief [2,2] heads its own subtree [2,2,1] (DFS pre-order).
        assert!(sort_key("2.2-[1]-a.BRIEF.md") < sort_key("2.2.1-[2]-b.md"));
    }

    #[test]
    fn compare_root_brief_sorts_first() {
        // [] < [1] — the root brief leads everything.
        assert!(sort_key("BRIEF.md") < sort_key("1-[1]-a.md"));
    }

    #[test]
    fn compare_foreign_sorts_last() {
        assert!(sort_key("1-[1]-a.md") < sort_key("README.md"));
        assert!(sort_key("9.9-[9]-z.md") < sort_key("README.md"));
    }

    #[test]
    fn compare_same_position_breaks_by_filename() {
        // A malformed same-position collision still totally orders, by filename.
        assert!(sort_key("1-[1]-a.md") < sort_key("1-[2]-b.md"));
    }

    #[test]
    fn compare_two_foreign_break_by_filename() {
        assert!(sort_key("aaa.txt") < sort_key("bbb.txt"));
    }

    // ---- next_key -----------------------------------------------------------

    #[test]
    fn next_key_empty_tree_is_one() {
        let none: Vec<&str> = Vec::new();
        assert_eq!(next_key(none), 1);
    }

    #[test]
    fn next_key_is_max_plus_one() {
        assert_eq!(next_key(["1-[1]-a.md", "2-[2]-b.md"]), 3);
    }

    #[test]
    fn next_key_counts_retired_done_files_so_keys_are_never_reused() {
        // The .DONE file carries the max key; it must still bump next_key.
        assert_eq!(next_key(["1-[3]-a.DONE.md", "2-[2]-b.md"]), 4);
    }

    #[test]
    fn next_key_counts_node_briefs() {
        assert_eq!(next_key(["1-[2]-a.BRIEF.md", "1.1-[3]-b.md"]), 4);
    }

    #[test]
    fn next_key_ignores_foreign_and_root_brief() {
        assert_eq!(next_key(["README.md", "BRIEF.md", "1-[1]-a.md"]), 2);
    }

    // ---- round-trip ---------------------------------------------------------

    #[test]
    fn round_trip_leaf() {
        let id = leaf(vec![2, 3, 1], 4, "add");
        assert_eq!(id.filename(), "2.3.1-[4]-add.md");
        assert_eq!(parse(&id.filename()), Some(id));
    }

    #[test]
    fn round_trip_node_brief() {
        let id = LeafId {
            position: vec![2, 3],
            key: Some(2),
            slug: "verbs".to_string(),
            is_brief: true,
            is_done: false,
        };
        assert_eq!(id.filename(), "2.3-[2]-verbs.BRIEF.md");
        assert_eq!(parse(&id.filename()), Some(id));
    }

    #[test]
    fn round_trip_done_leaf() {
        let id = LeafId {
            position: vec![2, 3, 1],
            key: Some(4),
            slug: "add".to_string(),
            is_brief: false,
            is_done: true,
        };
        assert_eq!(id.filename(), "2.3.1-[4]-add.DONE.md");
        assert_eq!(parse(&id.filename()), Some(id));
    }

    #[test]
    fn round_trip_root_brief() {
        let id = LeafId {
            position: Vec::new(),
            key: None,
            slug: String::new(),
            is_brief: true,
            is_done: false,
        };
        assert_eq!(id.filename(), "BRIEF.md");
        assert_eq!(parse(&id.filename()), Some(id));
    }

    // ---- is_live_leaf -------------------------------------------------------

    #[test]
    fn is_live_leaf_true_only_for_live_leaves() {
        assert!(parse("1-[1]-a.md").unwrap().is_live_leaf());
        assert!(!parse("1-[1]-a.BRIEF.md").unwrap().is_live_leaf());
        assert!(!parse("1-[1]-a.DONE.md").unwrap().is_live_leaf());
        assert!(!parse("BRIEF.md").unwrap().is_live_leaf());
    }

    // ---- validate_slug ------------------------------------------------------

    #[test]
    fn validate_slug_accepts_lowercase_digits_dashes() {
        assert!(validate_slug("add").is_ok());
        assert!(validate_slug("multi-word-slug").is_ok());
        assert!(validate_slug("a1").is_ok());
    }

    #[test]
    fn validate_slug_rejects_reserved_words() {
        assert!(validate_slug("BRIEF").is_err());
        assert!(validate_slug("DONE").is_err());
    }

    #[test]
    fn validate_slug_rejects_bad_chars_and_shape() {
        assert!(validate_slug("").is_err());
        assert!(validate_slug("Bad").is_err());
        assert!(validate_slug("with space").is_err());
        assert!(validate_slug("has.dot").is_err());
        assert!(validate_slug("has[bracket]").is_err());
        assert!(validate_slug("-leading").is_err());
        assert!(validate_slug("trailing-").is_err());
    }
}

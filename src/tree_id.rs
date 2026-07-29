// The **v2 id model** (task-tree-scheme): the parsed tree-entry-name type plus the pure
// functions every directory-based verb (11.2 read / 11.3 grow+lifecycle / 11.4
// migrate) consumes. Replaces `leaf_id`'s flat dotted-decimal-*vector* model.
//
// In v2 the task tree is real **directories** — the filesystem carries the
// hierarchy, so a name encodes only its *per-level* position, not a global path:
//
//     leaf       NN-[DONE-|ABANDONED-]<slug>-k<key>.md
//     node dir   NN-<slug>-k<key>             (a directory holding BRIEF.md + children)
//     brief      BRIEF.md                     (the containing node's charter)
//
// Three orthogonal parts: the per-level **position** `NN` (a 2-digit zero-padded
// decimal; the sole sort input *within one directory*; rewritten on renumber), the
// permanent **key** (stable identity, assigned once, never rewritten, always the
// terminal `-k<key>` token), and the human **slug**. A leaf's **outcome** — live,
// `DONE`, or `ABANDONED` (ADR *pruning*) — is an infix right after the position,
// the two marks mutually exclusive by construction (`Outcome`). A node is never
// marked either way — its done-ness is the absence of a live leaf in its subtree.
//
// The `-k<key>` delimiter (resolved at 11.1, amending task-tree-scheme's `[<key>]`):
// brackets are shell-glob metacharacters; `-k<key>` is glob-safe, and it stays
// unambiguous because the key is mandatory and always rendered last — parse takes
// the terminal `-k<digits>`, so `05-task-k9-k3.md` is slug `task-k9`, key `3`.
//
// Built **isolated**: it does NOT touch the live v1 verb path (`leaf_id` /
// `leaf_read` / `leaf_grow` / `leaf_lifecycle` / `migrate`), which keeps speaking
// the flat scheme until 11.2+ rewrite the verbs onto this model and the v1 modules
// are swept. Pure functions only, dependency-free of the verb modules.

use anyhow::{bail, Result};

/// A leaf's outcome (ADR *pruning*): live, retired (`DONE`), or abandoned
/// (`ABANDONED`) — mutually exclusive by construction (an enum, not two
/// independent bools, so the impossible fourth state is unrepresentable). A node
/// directory never carries an outcome at all; its done-ness is the absence of a
/// live leaf in its subtree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    /// Not yet retired or abandoned — what `pick` returns.
    Live,
    /// Work completed — the `DONE` infix.
    Done,
    /// Work rejected, closed, not going to happen — the `ABANDONED` infix
    /// (ADR *pruning*). The *why* lives in the ADR set, not the filename.
    Abandoned,
}

/// A parsed tree-entry name — the three on-disk shapes a node directory holds:
/// its own `BRIEF.md` charter, leaf-file children, and node-directory children.
///
/// Leaf-vs-node is inferred from the `.md` suffix (a leaf is a file ending in
/// `.md`; a node is a bare directory name). A verb that already has the real
/// `file_type` should treat a kind mismatch (e.g. a *directory* named `x.md`) as
/// foreign — that reconciliation is the verb's job, not this pure model's.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entry {
    /// `BRIEF.md` — the containing node's charter. Position-less and unkeyed; it
    /// heads its directory. Every node directory (including the root `.grove/`)
    /// holds exactly one.
    Brief,
    /// `NN-[DONE-|ABANDONED-]<slug>-k<key>.md` — a leaf child (a unit of work).
    Leaf {
        position: u32,
        slug: String,
        key: u32,
        outcome: Outcome,
    },
    /// `NN-<slug>-k<key>` — a child node directory (rendered without a trailing
    /// slash, matching `read_dir`'s `file_name`).
    Node {
        position: u32,
        slug: String,
        key: u32,
    },
}

impl Entry {
    /// The per-level position, or `None` for the root/charter brief.
    pub fn position(&self) -> Option<u32> {
        match self {
            Entry::Brief => None,
            Entry::Leaf { position, .. } | Entry::Node { position, .. } => Some(*position),
        }
    }

    /// The permanent identity key, or `None` for the brief.
    pub fn key(&self) -> Option<u32> {
        match self {
            Entry::Brief => None,
            Entry::Leaf { key, .. } | Entry::Node { key, .. } => Some(*key),
        }
    }

    /// The human slug, or `None` for the brief.
    pub fn slug(&self) -> Option<&str> {
        match self {
            Entry::Brief => None,
            Entry::Leaf { slug, .. } | Entry::Node { slug, .. } => Some(slug),
        }
    }

    /// `true` only for a retired (`DONE`) leaf.
    pub fn is_done(&self) -> bool {
        matches!(
            self,
            Entry::Leaf {
                outcome: Outcome::Done,
                ..
            }
        )
    }

    /// `true` only for an abandoned (`ABANDONED`) leaf (ADR *pruning*).
    pub fn is_abandoned(&self) -> bool {
        matches!(
            self,
            Entry::Leaf {
                outcome: Outcome::Abandoned,
                ..
            }
        )
    }

    /// `true` for the `BRIEF.md` charter.
    pub fn is_brief(&self) -> bool {
        matches!(self, Entry::Brief)
    }

    /// `true` for a child node directory.
    pub fn is_node(&self) -> bool {
        matches!(self, Entry::Node { .. })
    }

    /// A *live leaf* is the only thing `pick` returns: a leaf that is neither
    /// retired nor abandoned.
    pub fn is_live_leaf(&self) -> bool {
        matches!(
            self,
            Entry::Leaf {
                outcome: Outcome::Live,
                ..
            }
        )
    }

    /// Render this entry back to its on-disk name. Inverse of [`parse`] for any
    /// well-formed entry (round-trip: parts → name → parse → same parts). A node
    /// renders without a trailing slash; the position is zero-padded to 2 digits.
    pub fn name(&self) -> String {
        match self {
            Entry::Brief => "BRIEF.md".to_string(),
            Entry::Leaf {
                position,
                slug,
                key,
                outcome,
            } => {
                let infix = match outcome {
                    Outcome::Live => "",
                    Outcome::Done => "DONE-",
                    Outcome::Abandoned => "ABANDONED-",
                };
                format!("{:02}-{}{}-k{}.md", position, infix, slug, key)
            }
            Entry::Node {
                position,
                slug,
                key,
            } => format!("{:02}-{}-k{}", position, slug, key),
        }
    }

    /// The **position-free stable handle** `<slug>-k<key>` — how a work item is
    /// named in a commit message, in prose, and in any durable cross-reference
    /// (task-tree-scheme, *Reference a work item by its stable handle*).
    ///
    /// The counterpart of [`Entry::name`], and the reason both live here: `name`
    /// renders everything that *moves* (the per-level position, the outcome
    /// infix, the `.md` suffix), `handle` renders the part that never does. A
    /// caller that wants to be readable after a renumber wants this one. It is
    /// also, byte for byte, the `# <slug>-k<key>` header the grow verbs write
    /// into the file itself.
    ///
    /// `None` for the brief — the one entry with neither slug nor key, because a
    /// brief is context rather than a work item.
    pub fn handle(&self) -> Option<String> {
        match self {
            Entry::Brief => None,
            Entry::Leaf { slug, key, .. } | Entry::Node { slug, key, .. } => {
                Some(format!("{slug}-k{key}"))
            }
        }
    }
}

/// Parse a bare name (not a path) into an [`Entry`]. **Lenient**: a name that does
/// not match the grammar is "not a grove entry" and yields `None` — never a panic
/// or error, so a stray `README.md` or half-written file never jams the loop.
///
/// Leaf-vs-node is decided by the `.md` suffix; a trailing `/` on a node name (as
/// a path argument might carry) is tolerated.
pub fn parse(name: &str) -> Option<Entry> {
    // The charter brief is the one unkeyed, position-less singleton.
    if name == "BRIEF.md" {
        return Some(Entry::Brief);
    }

    if let Some(stem) = name.strip_suffix(".md") {
        // A leaf file: the `DONE`/`ABANDONED` outcome infix is permitted here.
        let (position, outcome, slug, key) = parse_parts(stem, true)?;
        Some(Entry::Leaf {
            position,
            slug,
            key,
            outcome,
        })
    } else {
        // A node directory: no extension, and never an outcome infix.
        let stem = name.strip_suffix('/').unwrap_or(name);
        let (position, _outcome, slug, key) = parse_parts(stem, false)?;
        Some(Entry::Node {
            position,
            slug,
            key,
        })
    }
}

/// Parse the inner `NN-[DONE-|ABANDONED-]<slug>-k<key>` of a stem (a leaf's part
/// before `.md`, or a node's bare directory name). `allow_outcome_infix` gates the
/// optional outcome infix — leaves only, since a node is never marked done or
/// abandoned. Returns `(position, outcome, slug, key)`, or `None` for any
/// non-well-formed shape.
fn parse_parts(stem: &str, allow_outcome_infix: bool) -> Option<(u32, Outcome, String, u32)> {
    // The position is the leading digit run, ended by the first `-` (the position
    // is pure digits, so the first dash is its unambiguous boundary).
    let dash = stem.find('-')?;
    let position = parse_position(&stem[..dash])?;
    let mut rest = &stem[dash + 1..];

    // The optional outcome infix sits immediately after the position, and the two
    // marks are mutually exclusive by construction (`strip_prefix` on `rest` — a
    // leaf can carry at most one). Both tokens are uppercase and slugs are
    // lowercase-only, so neither ever collides with a slug.
    let outcome = if let Some(r) = rest.strip_prefix("DONE-") {
        if !allow_outcome_infix {
            return None; // a node directory may not carry an outcome infix
        }
        rest = r;
        Outcome::Done
    } else if let Some(r) = rest.strip_prefix("ABANDONED-") {
        if !allow_outcome_infix {
            return None;
        }
        rest = r;
        Outcome::Abandoned
    } else {
        Outcome::Live
    };

    // The key is the terminal `-k<digits>`: take the trailing digit run, which
    // must be immediately preceded by `-k`. The key is mandatory and always
    // rendered last, so this is unambiguous even when the slug itself contains
    // `-k<digits>` (e.g. `task-k9-k3` → slug `task-k9`, key `3`).
    let digits_start = rest.len()
        - rest
            .bytes()
            .rev()
            .take_while(|b| b.is_ascii_digit())
            .count();
    if digits_start == rest.len() {
        return None; // no trailing key digits
    }
    let key: u32 = rest[digits_start..].parse().ok()?;
    let slug = rest[..digits_start].strip_suffix("-k")?;
    validate_slug(slug).ok()?;

    Some((position, outcome, slug.to_string(), key))
}

/// Parse a per-level position string (`05`) into its integer. **Lenient on
/// padding**: a hand-typed `5` parses to `5` (rendering re-pads to `05`); only
/// non-digit or empty input rejects. Public so the grow/lifecycle verbs (11.3)
/// can validate a bare position argument or the `NN` prefix of a `# …` header.
pub fn parse_position(s: &str) -> Option<u32> {
    if s.is_empty() || !s.bytes().all(|b| b.is_ascii_digit()) {
        return None;
    }
    s.parse().ok()
}

/// The **per-level** sort key for a name. Orders the entries *within one
/// directory*: the charter brief first, then leaves and node dirs by numeric
/// position, then foreign names last; the full name breaks ties deterministically.
/// The *global* tree order is the directory walk (11.2's `pick`), so this is a
/// per-level comparator only — not a global vector like v1's.
pub fn sort_key(name: &str) -> (u8, u32, String) {
    // The leading rank groups: 0 = the charter (heads its dir), 1 = positioned
    // children (ordered by numeric position — `9 < 10`, the case lexical sort
    // fails), 2 = foreign. The name is the deterministic final tie-break.
    match parse(name) {
        Some(Entry::Brief) => (0, 0, name.to_string()),
        Some(entry) => (1, entry.position().unwrap_or(0), name.to_string()),
        None => (2, 0, name.to_string()),
    }
}

/// The next permanent key for a tree: `max(k over all names, live AND DONE AND
/// node dirs) + 1`, or `1` for an empty tree. Keys live in the names (no counter
/// file) and `DONE` leaves stay in the tree, so the max is always visible and keys
/// are never reused. The verb flattens the whole directory tree into the iterator.
pub fn next_key<I, S>(names: I) -> u32
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    names
        .into_iter()
        .filter_map(|n| parse(n.as_ref()).and_then(|e| e.key()))
        .max()
        .map_or(1, |m| m + 1)
}

/// Validate a slug: non-empty, lowercase ASCII + digits + `-`, no leading/trailing
/// `-`, and not the reserved words `BRIEF` / `DONE` / `ABANDONED`. The character
/// set already excludes everything that could blur a name boundary (`.`, `/`,
/// `[`, `]`), and being lowercase keeps it clear of the uppercase outcome infixes.
pub fn validate_slug(slug: &str) -> Result<()> {
    if slug.is_empty() {
        bail!("slug must not be empty");
    }
    if slug == "BRIEF" || slug == "DONE" || slug == "ABANDONED" {
        bail!("slug {:?} is reserved", slug);
    }
    if slug.starts_with('-') || slug.ends_with('-') {
        bail!("slug must not start or end with a dash: {:?}", slug);
    }
    for c in slug.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            bail!(
                "slug must contain only lowercase ASCII letters, digits, and dashes: {:?}",
                slug
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn leaf(position: u32, slug: &str, key: u32) -> Entry {
        Entry::Leaf {
            position,
            slug: slug.to_string(),
            key,
            outcome: Outcome::Live,
        }
    }

    fn done_leaf(position: u32, slug: &str, key: u32) -> Entry {
        Entry::Leaf {
            position,
            slug: slug.to_string(),
            key,
            outcome: Outcome::Done,
        }
    }

    fn abandoned_leaf(position: u32, slug: &str, key: u32) -> Entry {
        Entry::Leaf {
            position,
            slug: slug.to_string(),
            key,
            outcome: Outcome::Abandoned,
        }
    }

    fn node(position: u32, slug: &str, key: u32) -> Entry {
        Entry::Node {
            position,
            slug: slug.to_string(),
            key,
        }
    }

    // ---- parse: well-formed -------------------------------------------------

    #[test]
    fn parse_leaf_file() {
        assert_eq!(parse("05-add-k4.md"), Some(leaf(5, "add", 4)));
    }

    #[test]
    fn parse_node_directory() {
        assert_eq!(parse("05-verbs-k2"), Some(node(5, "verbs", 2)));
    }

    #[test]
    fn parse_node_directory_tolerates_trailing_slash() {
        // A path argument may carry the trailing `/`; `read_dir` names do not.
        assert_eq!(parse("05-verbs-k2/"), Some(node(5, "verbs", 2)));
    }

    #[test]
    fn parse_done_leaf() {
        assert_eq!(parse("05-DONE-add-k4.md"), Some(done_leaf(5, "add", 4)));
    }

    #[test]
    fn parse_abandoned_leaf() {
        assert_eq!(
            parse("05-ABANDONED-add-k4.md"),
            Some(abandoned_leaf(5, "add", 4))
        );
    }

    #[test]
    fn parse_root_brief() {
        assert_eq!(parse("BRIEF.md"), Some(Entry::Brief));
    }

    #[test]
    fn parse_lenient_on_unpadded_position() {
        // A hand-typed single-digit position still parses (rendering re-pads).
        assert_eq!(parse("1-survey-k1.md"), Some(leaf(1, "survey", 1)));
    }

    #[test]
    fn parse_multidigit_position_and_key() {
        assert_eq!(parse("12-foo-k123.md"), Some(leaf(12, "foo", 123)));
    }

    #[test]
    fn parse_slug_may_contain_dashes() {
        assert_eq!(
            parse("01-multi-word-slug-k1.md"),
            Some(leaf(1, "multi-word-slug", 1))
        );
    }

    #[test]
    fn parse_slug_containing_k_and_digits_round_trips() {
        // The crux of the `-k<key>` grammar: the key is the *terminal* `-k<digits>`,
        // so a slug that itself ends in `-k<digits>` is unambiguous.
        assert_eq!(parse("05-task-k9-k3.md"), Some(leaf(5, "task-k9", 3)));
    }

    #[test]
    fn parse_slug_ending_in_k_letter() {
        assert_eq!(parse("05-xk-k3.md"), Some(leaf(5, "xk", 3)));
    }

    #[test]
    fn parse_slug_ending_in_digit() {
        assert_eq!(parse("05-abc5-k3.md"), Some(leaf(5, "abc5", 3)));
    }

    // ---- parse: kind is decided by the `.md` suffix -------------------------

    #[test]
    fn parse_md_suffix_is_leaf_bare_name_is_node() {
        assert!(matches!(parse("05-foo-k5.md"), Some(Entry::Leaf { .. })));
        assert!(matches!(parse("05-foo-k5"), Some(Entry::Node { .. })));
    }

    // ---- parse: lenient (None) ----------------------------------------------

    #[test]
    fn parse_old_nnn_slug_format_is_not_v2() {
        // Old `NNN-slug` has no `-k<key>` — must not parse as v2.
        assert_eq!(parse("010-id-model.md"), None);
    }

    #[test]
    fn parse_v1_flat_format_is_not_v2() {
        // v1-flat `<dotted>-[<key>]-<slug>` has dots and brackets — rejected.
        assert_eq!(parse("2.3-[4]-add.md"), None);
    }

    #[test]
    fn parse_foreign_files_are_none() {
        assert_eq!(parse("README.md"), None);
        assert_eq!(parse("notes.txt"), None);
        assert_eq!(parse(".gitkeep"), None);
    }

    #[test]
    fn parse_keyless_positioned_is_none() {
        assert_eq!(parse("05-add.md"), None);
        assert_eq!(parse("05-add"), None);
    }

    #[test]
    fn parse_empty_or_nondigit_key_is_none() {
        assert_eq!(parse("05-x-k.md"), None);
        assert_eq!(parse("05-x-ka.md"), None);
        // No `-k` delimiter before the trailing digits.
        assert_eq!(parse("05-x5.md"), None);
    }

    #[test]
    fn parse_malformed_position_is_none() {
        assert_eq!(parse("1.2-x-k1.md"), None);
        assert_eq!(parse("-x-k1.md"), None);
        assert_eq!(parse("x-foo-k1.md"), None);
    }

    #[test]
    fn parse_node_with_done_infix_is_none() {
        // A node directory is never marked done — done-ness is implicit.
        assert_eq!(parse("05-DONE-x-k1"), None);
    }

    #[test]
    fn parse_node_with_abandoned_infix_is_none() {
        // Symmetric with DONE: a node directory is never marked abandoned either —
        // its state is "no live leaf in the subtree", however that came about.
        assert_eq!(parse("05-ABANDONED-x-k1"), None);
    }

    #[test]
    fn parse_slug_with_dot_or_bracket_is_none() {
        assert_eq!(parse("01-a.b-k1.md"), None);
        assert_eq!(parse("01-a[b-k1.md"), None);
    }

    #[test]
    fn parse_double_done_infix_is_none() {
        // Only one `DONE-` is peeled; a second leaks into the slug (uppercase),
        // which `validate_slug` rejects.
        assert_eq!(parse("05-DONE-DONE-x-k1.md"), None);
    }

    #[test]
    fn parse_double_abandoned_infix_is_none() {
        assert_eq!(parse("05-ABANDONED-ABANDONED-x-k1.md"), None);
    }

    #[test]
    fn parse_done_and_abandoned_infixes_do_not_combine() {
        // The two marks are mutually exclusive by construction: only the first
        // (`DONE-`) is peeled as the outcome infix, so `ABANDONED-` leaks into the
        // slug (uppercase), which `validate_slug` rejects.
        assert_eq!(parse("05-DONE-ABANDONED-x-k1.md"), None);
        assert_eq!(parse("05-ABANDONED-DONE-x-k1.md"), None);
    }

    // ---- comparator (per-level) ---------------------------------------------

    #[test]
    fn compare_numeric_not_lexical() {
        // 9 < 10 — the exact case a dumb lexical sort of unpadded positions fails.
        assert!(sort_key("9-a-k1.md") < sort_key("10-b-k2.md"));
    }

    #[test]
    fn compare_brief_sorts_first() {
        assert!(sort_key("BRIEF.md") < sort_key("01-a-k1.md"));
    }

    #[test]
    fn compare_node_and_leaf_order_by_position() {
        // Mixed children (a node dir and a leaf file) order purely by position.
        assert!(sort_key("01-a-k1") < sort_key("02-b-k2.md"));
        assert!(sort_key("01-a-k1.md") < sort_key("02-b-k2"));
    }

    #[test]
    fn compare_foreign_sorts_last() {
        assert!(sort_key("01-a-k1.md") < sort_key("README.md"));
        assert!(sort_key("99-z-k9") < sort_key("README.md"));
    }

    #[test]
    fn compare_same_position_breaks_by_name() {
        // A malformed same-position collision still totally orders, by name.
        assert!(sort_key("01-a-k1.md") < sort_key("01-b-k2.md"));
    }

    #[test]
    fn compare_two_foreign_break_by_name() {
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
        assert_eq!(next_key(["01-a-k1.md", "02-b-k2.md"]), 3);
    }

    #[test]
    fn next_key_counts_retired_done_leaves() {
        // The `DONE` leaf carries the max key; it must still bump next_key.
        assert_eq!(next_key(["01-DONE-a-k3.md", "02-b-k2.md"]), 4);
    }

    #[test]
    fn next_key_counts_abandoned_leaves() {
        // The whole point of ADR *pruning*: a pruned leaf's key stays visible in
        // the tree, so a future `leaf-add` never re-issues it. `next_key` already
        // maxes over every parsed name regardless of outcome — asserted here so a
        // future refactor cannot quietly regress the property the ADR rests on.
        assert_eq!(next_key(["01-ABANDONED-a-k3.md", "02-b-k2.md"]), 4);
    }

    #[test]
    fn next_key_counts_node_directories() {
        // A node lives as a bare directory name — its key still counts.
        assert_eq!(next_key(["05-node-k3", "01-b-k2.md"]), 4);
    }

    #[test]
    fn next_key_ignores_foreign_and_brief() {
        assert_eq!(next_key(["README.md", "BRIEF.md", "01-a-k1.md"]), 2);
    }

    // ---- round-trip ---------------------------------------------------------

    #[test]
    fn round_trip_leaf() {
        let e = leaf(5, "add", 4);
        assert_eq!(e.name(), "05-add-k4.md");
        assert_eq!(parse(&e.name()), Some(e));
    }

    #[test]
    fn round_trip_done_leaf() {
        let e = done_leaf(5, "add", 4);
        assert_eq!(e.name(), "05-DONE-add-k4.md");
        assert_eq!(parse(&e.name()), Some(e));
    }

    #[test]
    fn round_trip_abandoned_leaf() {
        let e = abandoned_leaf(5, "add", 4);
        assert_eq!(e.name(), "05-ABANDONED-add-k4.md");
        assert_eq!(parse(&e.name()), Some(e));
    }

    #[test]
    fn round_trip_node() {
        let e = node(5, "verbs", 2);
        assert_eq!(e.name(), "05-verbs-k2");
        assert_eq!(parse(&e.name()), Some(e));
    }

    #[test]
    fn round_trip_root_brief() {
        let e = Entry::Brief;
        assert_eq!(e.name(), "BRIEF.md");
        assert_eq!(parse(&e.name()), Some(e));
    }

    #[test]
    fn round_trip_slug_containing_k_segment() {
        // Proves render→parse survives a slug that mimics the key delimiter.
        let e = leaf(5, "task-k9", 3);
        assert_eq!(e.name(), "05-task-k9-k3.md");
        assert_eq!(parse(&e.name()), Some(e));
    }

    // ---- render specifics ---------------------------------------------------

    #[test]
    fn name_pads_position_to_two_digits() {
        assert_eq!(leaf(5, "x", 1).name(), "05-x-k1.md");
        assert_eq!(node(5, "x", 1).name(), "05-x-k1");
    }

    #[test]
    fn name_keeps_three_digit_position_unclamped() {
        // Past 99 the FS lexical sort degrades (the documented ~99/level limit),
        // but rendering must not truncate — grove's own comparator stays numeric.
        assert_eq!(leaf(100, "x", 1).name(), "100-x-k1.md");
    }

    // ---- handle -------------------------------------------------------------

    #[test]
    fn handle_drops_the_position_that_name_renders() {
        // The whole point of §5: the same work item at two positions is one
        // handle, so a reference written before a renumber still reads after it.
        assert_eq!(leaf(5, "add", 4).handle().as_deref(), Some("add-k4"));
        assert_eq!(leaf(9, "add", 4).handle().as_deref(), Some("add-k4"));
        assert_eq!(node(5, "verbs", 2).handle().as_deref(), Some("verbs-k2"));
    }

    #[test]
    fn handle_survives_the_outcome_infix() {
        // Retiring renames the file; a commit message that named the leaf must
        // not have named something that no longer exists.
        assert_eq!(done_leaf(5, "add", 4).handle().as_deref(), Some("add-k4"));
        assert_eq!(
            abandoned_leaf(5, "add", 4).handle().as_deref(),
            Some("add-k4")
        );
    }

    #[test]
    fn handle_is_none_for_the_brief() {
        // The one entry with neither slug nor key: a brief is context, not a
        // work item, so there is nothing to name.
        assert_eq!(Entry::Brief.handle(), None);
    }

    // ---- kind predicates ----------------------------------------------------

    #[test]
    fn is_live_leaf_true_only_for_live_leaves() {
        assert!(parse("01-a-k1.md").unwrap().is_live_leaf());
        assert!(!parse("01-DONE-a-k1.md").unwrap().is_live_leaf());
        assert!(!parse("01-ABANDONED-a-k1.md").unwrap().is_live_leaf());
        assert!(!parse("01-a-k1").unwrap().is_live_leaf()); // node dir
        assert!(!parse("BRIEF.md").unwrap().is_live_leaf());
    }

    #[test]
    fn kind_predicates_classify_each_shape() {
        assert!(parse("BRIEF.md").unwrap().is_brief());
        assert!(parse("01-a-k1").unwrap().is_node());
        assert!(parse("01-DONE-a-k1.md").unwrap().is_done());
        assert!(!parse("01-a-k1.md").unwrap().is_done());
    }

    #[test]
    fn is_abandoned_true_only_for_abandoned_leaves() {
        assert!(parse("01-ABANDONED-a-k1.md").unwrap().is_abandoned());
        assert!(!parse("01-DONE-a-k1.md").unwrap().is_abandoned());
        assert!(!parse("01-a-k1.md").unwrap().is_abandoned());
        assert!(!parse("01-a-k1").unwrap().is_abandoned()); // node dir
        assert!(!parse("BRIEF.md").unwrap().is_abandoned());
    }

    #[test]
    fn done_and_abandoned_are_mutually_exclusive() {
        // The outcome is a single enum, not two independent bools — the
        // impossible fourth state (both marks at once) is unrepresentable, so
        // this is really just confirming the accessors agree with each other.
        let done = parse("01-DONE-a-k1.md").unwrap();
        assert!(done.is_done() && !done.is_abandoned());
        let abandoned = parse("01-ABANDONED-a-k1.md").unwrap();
        assert!(abandoned.is_abandoned() && !abandoned.is_done());
    }

    #[test]
    fn accessors_return_parts_or_none_for_brief() {
        let l = parse("07-foo-bar-k9.md").unwrap();
        assert_eq!(l.position(), Some(7));
        assert_eq!(l.key(), Some(9));
        assert_eq!(l.slug(), Some("foo-bar"));

        let b = Entry::Brief;
        assert_eq!(b.position(), None);
        assert_eq!(b.key(), None);
        assert_eq!(b.slug(), None);
    }

    // ---- validate_slug ------------------------------------------------------

    #[test]
    fn validate_slug_accepts_lowercase_digits_dashes() {
        assert!(validate_slug("add").is_ok());
        assert!(validate_slug("multi-word-slug").is_ok());
        assert!(validate_slug("a1").is_ok());
        assert!(validate_slug("task-k9").is_ok()); // a `k<digit>` segment is fine mid-slug
    }

    #[test]
    fn validate_slug_rejects_reserved_words() {
        assert!(validate_slug("BRIEF").is_err());
        assert!(validate_slug("DONE").is_err());
        assert!(validate_slug("ABANDONED").is_err());
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

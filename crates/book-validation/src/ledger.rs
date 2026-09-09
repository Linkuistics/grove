use std::collections::BTreeSet;

use crate::manifest::{Manifest, Page, SourceRoot};
use crate::parser::{Child, Fragment, FragmentBody, ParsedBook, Root};
use crate::{BookSnapshot, Diagnostic, Location, Scope};

#[derive(Clone)]
struct LedgerRow {
    raw: String,
    location: Location,
}

struct LedgerTable {
    location: Location,
    header: Option<String>,
    separator: Option<String>,
    rows: Vec<LedgerRow>,
}

/// Where a ledger table's header sits relative to its H2 heading.
#[derive(Clone, Copy, Eq, PartialEq)]
enum LeadIn {
    /// The four fixed tables: one blank line and then the header, with a
    /// paragraph there reported as `F009` (*Source and ownership ledger*).
    Forbidden,
    /// The owned-source totals table, which is a figure and so carries an
    /// adjacent statement of its role between the heading and the header
    /// (*Figures*). The lead-in's prose is editorial and unchecked — the
    /// validator inspects no figure's role statement — but the rows below it
    /// are reconciled against the manifest like any other derived index.
    Permitted,
}

pub(crate) fn check(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest = &snapshot.manifest;
    let index_path = manifest.source_index_path();
    let source_index = snapshot
        .book_files
        .get(&index_path)
        .and_then(|bytes| std::str::from_utf8(bytes).ok());

    let source_tables = required_table(
        &index_path,
        source_index,
        "Source roots",
        LeadIn::Forbidden,
        diagnostics,
    );
    let ownership_tables = required_table(
        &index_path,
        source_index,
        "Ownership blocks",
        LeadIn::Forbidden,
        diagnostics,
    );
    let fragment_tables = required_table(
        &index_path,
        source_index,
        "Fragment index",
        LeadIn::Forbidden,
        diagnostics,
    );
    let early_tables = required_table(
        &index_path,
        source_index,
        "Early uses",
        LeadIn::Forbidden,
        diagnostics,
    );
    let totals_tables = required_table(
        &index_path,
        source_index,
        "Owned source totals",
        LeadIn::Permitted,
        diagnostics,
    );

    if let Some(table) = source_tables {
        check_exact_table(
            &table,
            "| Root ID | Source path | Lines |\n",
            "|---|---|---|\n",
            &declared_source_rows(manifest),
            "Source roots ledger disagrees with the declared corpus",
            diagnostics,
        );
        check_exact_rows(
            &table,
            &directive_source_rows(&index_path, parsed),
            "Source roots ledger disagrees with source-root directives",
            diagnostics,
        );
    }

    if let Some(table) = ownership_tables {
        check_exact_table(
            &table,
            "| Block ID | Root ID | Owner | Source lines | Count | State |\n",
            "|---|---|---|---|---|---|\n",
            &declared_ownership_rows(manifest, scope),
            "Ownership blocks ledger disagrees with the declared ownership contract",
            diagnostics,
        );
        check_exact_rows(
            &table,
            &directive_ownership_rows(&index_path, parsed),
            "Ownership blocks ledger disagrees with root directives",
            diagnostics,
        );
    }

    if let Some(table) = fragment_tables {
        check_exact_table(
            &table,
            "| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |\n",
            "|---|---|---|---|---|---|---|---|\n",
            &directive_fragment_rows(manifest, &index_path, parsed),
            "Fragment index disagrees with fragment directives",
            diagnostics,
        );
    }

    let early_rows: Vec<String> = early_tables
        .as_ref()
        .map(|table| table.rows.iter().map(|row| row.raw.clone()).collect())
        .unwrap_or_default();
    if let Some(table) = early_tables {
        check_early_uses(snapshot, &table, scope, diagnostics);
    }

    if let Some(table) = totals_tables {
        check_exact_table(
            &table,
            "| Slice | Page | Owned lines |\n",
            "|---|---|---:|\n",
            &declared_totals_rows(manifest),
            "Owned source totals disagree with the declared ownership contract",
            diagnostics,
        );
    }

    check_root_locations(manifest, &index_path, source_index, parsed, diagnostics);
    check_fragment_locations(manifest, parsed, diagnostics);
    check_rollups(manifest, &early_rows, parsed, diagnostics);
}

fn required_table(
    index_path: &str,
    source_index: Option<&str>,
    heading: &str,
    lead_in: LeadIn,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<LedgerTable> {
    let tables = source_index.map_or_else(Vec::new, |text| {
        find_tables(index_path, text, heading, lead_in)
    });
    if tables.len() == 1 {
        return tables.into_iter().next();
    }
    diagnostics.push(f009(
        format!(
            "mandatory `{heading}` ledger table occurs {} times; expected exactly once",
            tables.len()
        ),
        Location {
            path: index_path.into(),
            byte: 0,
            line: 1,
            column: 1,
        },
    ));
    None
}

fn find_tables(index_path: &str, text: &str, heading: &str, lead_in: LeadIn) -> Vec<LedgerTable> {
    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut offsets = Vec::with_capacity(lines.len());
    let mut offset = 0;
    for line in &lines {
        offsets.push(offset);
        offset += line.len();
    }
    let marker = format!("## {heading}\n");
    let mut tables = Vec::new();
    let mut fence = None;
    for (index, line) in lines.iter().enumerate() {
        let bare = line.strip_suffix('\n').unwrap_or(line);
        if let Some(delimiter) = &fence {
            if bare == delimiter {
                fence = None;
            }
            continue;
        }
        if let Some(delimiter) = fence_opener(bare) {
            fence = Some(delimiter);
            continue;
        }
        if *line != marker {
            continue;
        }
        let mut cursor = index + 1;
        let blank = lines.get(cursor).copied() == Some("\n");
        if blank {
            cursor += 1;
        }
        if lead_in == LeadIn::Permitted {
            // Walk the role statement. It ends at the table's own first row;
            // a new heading or a fence means there is no table under this
            // one, and leaving the cursor there reports that as `F009`
            // through the header the caller then fails to match.
            while let Some(line) = lines.get(cursor) {
                let bare = line.strip_suffix('\n').unwrap_or(line);
                if bare.starts_with('|') || bare.starts_with("## ") || fence_opener(bare).is_some()
                {
                    break;
                }
                cursor += 1;
            }
        }
        let header = lines.get(cursor).map(|line| (*line).to_owned());
        cursor += usize::from(header.is_some());
        let separator = lines.get(cursor).map(|line| (*line).to_owned());
        cursor += usize::from(separator.is_some());
        let mut rows = Vec::new();
        while let Some(line) = lines.get(cursor).filter(|line| line.starts_with('|')) {
            rows.push(LedgerRow {
                raw: (*line).to_owned(),
                location: line_location(index_path, text, offsets[cursor]),
            });
            cursor += 1;
        }
        let mut table = LedgerTable {
            location: line_location(index_path, text, offsets[index]),
            header,
            separator,
            rows,
        };
        if !blank && lead_in == LeadIn::Forbidden {
            table.header = None;
        }
        tables.push(table);
    }
    tables
}

fn fence_opener(line: &str) -> Option<String> {
    let marker = line.as_bytes().first().copied()?;
    if !matches!(marker, b'`' | b'~') {
        return None;
    }
    let count = line.bytes().take_while(|byte| *byte == marker).count();
    if count < 3 {
        return None;
    }
    let info = &line[count..];
    if !info.is_empty()
        && (!info
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
            || !info.as_bytes()[0].is_ascii_alphanumeric())
    {
        return None;
    }
    Some((marker as char).to_string().repeat(count))
}

fn check_exact_table(
    table: &LedgerTable,
    header: &str,
    separator: &str,
    rows: &[String],
    message: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let valid_shape = table.header.as_deref() == Some(header)
        && table.separator.as_deref() == Some(separator)
        && table
            .rows
            .iter()
            .all(|row| strict_cells(&row.raw).is_some());
    if !valid_shape || table.rows.iter().map(|row| &row.raw).ne(rows.iter()) {
        diagnostics.push(f009(message, first_difference(table, rows)));
    }
}

fn check_exact_rows(
    table: &LedgerTable,
    rows: &[String],
    message: &str,
    diagnostics: &mut Vec<Diagnostic>,
) {
    if table.rows.iter().map(|row| &row.raw).ne(rows.iter()) {
        diagnostics.push(f009(message, first_difference(table, rows)));
    }
}

fn first_difference(table: &LedgerTable, expected: &[String]) -> Location {
    table
        .rows
        .iter()
        .zip(expected)
        .find_map(|(actual, expected)| (actual.raw != *expected).then(|| actual.location.clone()))
        .or_else(|| {
            table
                .rows
                .get(expected.len())
                .map(|row| row.location.clone())
        })
        .unwrap_or_else(|| table.location.clone())
}

fn declared_source_rows(manifest: &Manifest) -> Vec<String> {
    manifest
        .roots()
        .iter()
        .map(|root| {
            format!(
                "| `{}` | `{}` | {} |\n",
                root.id(),
                root.path(),
                grouped(root.lines())
            )
        })
        .collect()
}

fn directive_source_rows(index_path: &str, parsed: &ParsedBook) -> Vec<String> {
    roots_in_source_index(index_path, parsed)
        .into_iter()
        .map(|root| {
            format!(
                "| `{}` | `{}` | {} |\n",
                root.id,
                root.source,
                grouped(root.range.last - root.range.first + 1)
            )
        })
        .collect()
}

/// The owned-source totals table, derived from the manifest alone.
///
/// One row per chapter in manifest order, crediting each source line once to
/// the slice whose top-level block owns it, then a total row naming the root
/// count and the corpus line count. Both columns are safe to derive because
/// the schema already refuses a chapter with no slice and a block whose owner
/// is no chapter's slice, so every owned line lands in exactly one row and the
/// column sums to the corpus.
///
/// State plays no part: a line is owned by its slice from the manifest onward,
/// whether or not the owning chapter has resolved its defer yet, so a scoped
/// run reconciles this table exactly as a final one does.
fn declared_totals_rows(manifest: &Manifest) -> Vec<String> {
    let mut rows: Vec<String> = manifest
        .chapters()
        .map(|page| {
            let slice = page.slice().unwrap_or_default();
            let owned: usize = manifest
                .blocks()
                .iter()
                .filter(|block| block.owner() == slice)
                .map(|block| block.last() - block.first() + 1)
                .sum();
            format!("| `{slice}` | `{}` | {} |\n", page.file(), grouped(owned))
        })
        .collect();
    let corpus: usize = manifest.roots().iter().map(SourceRoot::lines).sum();
    let roots = manifest.roots().len();
    rows.push(format!(
        "| **Total** | {roots} source root{} | **{}** |\n",
        if roots == 1 { "" } else { "s" },
        grouped(corpus)
    ));
    rows
}

fn declared_ownership_rows(manifest: &Manifest, scope: &Scope) -> Vec<String> {
    manifest
        .blocks()
        .iter()
        .map(|block| {
            ownership_row(
                block.id(),
                block.root(),
                block.owner(),
                block.first(),
                block.last(),
                if owner_is_complete(manifest, scope, block.owner()) {
                    "resolved"
                } else {
                    "deferred"
                },
            )
        })
        .collect()
}

fn directive_ownership_rows(index_path: &str, parsed: &ParsedBook) -> Vec<String> {
    let mut rows = Vec::new();
    for root in roots_in_source_index(index_path, parsed) {
        for child in &root.children {
            match child {
                Child::Defer {
                    id, owner, range, ..
                } => rows.push(ownership_row(
                    id,
                    &root.id,
                    owner,
                    range.first,
                    range.last,
                    "deferred",
                )),
                Child::Insert { id, .. } => {
                    let Some(fragment) = unique_fragment(parsed, id) else {
                        rows.push(format!(
                            "| `{id}` | `{}` | `—` | `—` | 0 | `resolved` |\n",
                            root.id
                        ));
                        continue;
                    };
                    rows.push(ownership_row(
                        id,
                        &root.id,
                        &fragment.owner,
                        fragment.range.first,
                        fragment.range.last,
                        "resolved",
                    ));
                }
            }
        }
    }
    rows
}

fn ownership_row(
    id: &str,
    root: &str,
    owner: &str,
    first: usize,
    last: usize,
    state: &str,
) -> String {
    format!(
        "| `{id}` | `{root}` | `{owner}` | `{first}-{last}` | {} | `{state}` |\n",
        grouped(last - first + 1)
    )
}

fn directive_fragment_rows(
    manifest: &Manifest,
    index_path: &str,
    parsed: &ParsedBook,
) -> Vec<String> {
    let roots = roots_in_source_index(index_path, parsed);
    let mut rows = Vec::new();
    for root in roots {
        rows.push(root_fragment_row(manifest, root));
        let mut fragments: Vec<&Fragment> = parsed
            .fragments
            .values()
            .flatten()
            .filter(|fragment| root_for_fragment(parsed, fragment) == Some(root.id.as_str()))
            .collect();
        fragments.sort_by(|left, right| {
            left.range
                .first
                .cmp(&right.range.first)
                .then_with(|| left.range.last.cmp(&right.range.last))
                .then_with(|| left.id.cmp(&right.id))
                .then_with(|| left.location.cmp(&right.location))
        });
        rows.extend(fragments.into_iter().map(|fragment| {
            let kind = match fragment.body {
                FragmentBody::Literal(_) => "literal",
                FragmentBody::Composite(_) => "composite",
            };
            let children = fragment_children(fragment);
            format!(
                "| `{}` | `{}` | `{}` | `{kind}` | `{}` | `{}-{}` | `{}` | {} |\n",
                fragment.id,
                page_id_for_path(manifest, &fragment.location.path).unwrap_or("—"),
                root.id,
                fragment.owner,
                fragment.range.first,
                fragment.range.last,
                fragment.parent,
                children
            )
        }));
    }
    rows
}

fn root_fragment_row(manifest: &Manifest, root: &Root) -> String {
    let children = child_ids(&root.children);
    format!(
        "| `{}` | `{}` | `{}` | `root` | `—` | `{}-{}` | `—` | {} |\n",
        root.id,
        manifest.source_index_id(),
        root.id,
        root.range.first,
        root.range.last,
        children
    )
}

fn fragment_children(fragment: &Fragment) -> String {
    match &fragment.body {
        FragmentBody::Literal(_) => "`—`".into(),
        FragmentBody::Composite(children) => child_ids(children),
    }
}

fn child_ids(children: &[Child]) -> String {
    if children.is_empty() {
        return "`—`".into();
    }
    children
        .iter()
        .map(|child| match child {
            Child::Insert { id, .. } | Child::Defer { id, .. } => format!("`{id}`"),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn check_early_uses(
    snapshot: &BookSnapshot,
    table: &LedgerTable,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest = &snapshot.manifest;
    let header = "| Symbol family | First use | Owner | Minimum local statement | Status |\n";
    let separator = "|---|---|---|---|---|\n";
    if table.header.as_deref() != Some(header) || table.separator.as_deref() != Some(separator) {
        diagnostics.push(f009(
            "Early uses ledger has a malformed header or separator",
            table.location.clone(),
        ));
        return;
    }

    let required: Vec<String> = manifest
        .early_uses()
        .iter()
        .map(|entry| {
            let status = if owner_is_complete(manifest, scope, entry.owner()) {
                "explained"
            } else {
                "pending"
            };
            format!(
                "| {} | `{}` | `{}` | {} | `{status}` |\n",
                entry.symbols(),
                entry.first_use(),
                entry.owner(),
                entry.statement()
            )
        })
        .collect();
    if required
        .iter()
        .any(|expected| table.rows.iter().filter(|row| row.raw == *expected).count() != 1)
    {
        diagnostics.push(f009(
            "Early uses ledger is missing or contradicts a mandatory row",
            table.location.clone(),
        ));
    }

    let mut keys = Vec::new();
    let mut seen = BTreeSet::new();
    for row in &table.rows {
        let Some(cells) = strict_cells(&row.raw) else {
            diagnostics.push(f009(
                "Early uses ledger contains a malformed row",
                row.location.clone(),
            ));
            continue;
        };
        if cells.len() != 5 || !valid_symbol_family(cells[0]) {
            diagnostics.push(f009(
                "Early uses ledger row has invalid columns or symbol syntax",
                row.location.clone(),
            ));
            continue;
        }
        let Some(first_use) = unquote(cells[1]) else {
            diagnostics.push(f009(
                "Early use location must be wrapped in backticks",
                row.location.clone(),
            ));
            continue;
        };
        let Some(owner) = unquote(cells[2]) else {
            diagnostics.push(f009(
                "Early use owner must be wrapped in backticks",
                row.location.clone(),
            ));
            continue;
        };
        let Some(status) = unquote(cells[4]) else {
            diagnostics.push(f009(
                "Early use status must be wrapped in backticks",
                row.location.clone(),
            ));
            continue;
        };
        let Some((filename, anchor)) = first_use.split_once('#') else {
            diagnostics.push(f009(
                "Early use location must contain a page and explicit anchor",
                row.location.clone(),
            ));
            continue;
        };
        let Some(first_number) = chapter_number(manifest, filename) else {
            diagnostics.push(f009(
                "Early use location names a noncanonical numbered page",
                row.location.clone(),
            ));
            continue;
        };
        let Some(owner_order) = manifest.slice_order(owner) else {
            diagnostics.push(f009(
                "Early use owner is not a canonical slice",
                row.location.clone(),
            ));
            continue;
        };
        // The manifest is complete from the start, so a mandatory row may name
        // a first use on a chapter the prefix has not reached. Its page is not
        // in the snapshot, and Markdown validation independently forbids it
        // from being — so outside the prefix the anchor is a promise, checked
        // for structure here and against bytes at the scope that has them
        // (`docs/specs/walkthrough-books.md`, *Early-use ledger*).
        let in_prefix = chapter_in_prefix(scope, first_number);
        let page_path = manifest.path(filename);
        let anchor_line = format!("<a id=\"{anchor}\"></a>\n");
        let anchor_byte = in_prefix
            .then(|| {
                snapshot
                    .book_files
                    .get(&page_path)
                    .and_then(|bytes| std::str::from_utf8(bytes).ok())
                    .and_then(|text| text.find(&anchor_line))
            })
            .flatten();
        let expected_status = if owner_is_complete(manifest, scope, owner) {
            "explained"
        } else {
            "pending"
        };
        // `first_number` counts chapters from one and `owner_order` from zero,
        // so this is "the first use is not on a page strictly before the page
        // that explains it" — which is the whole point of an early-use row.
        if first_number > owner_order
            || (in_prefix && anchor_byte.is_none())
            || status != expected_status
            || cells[3].is_empty()
        {
            diagnostics.push(f009(
                "Early use row disagrees with page, owner, anchor, or scope state",
                row.location.clone(),
            ));
        }
        let key = EarlyUseKey {
            chapter: first_number,
            anchor: anchor_byte,
            owner: owner_order,
            symbols: cells[0].to_owned(),
        };
        if !seen.insert(key.clone()) {
            diagnostics.push(f009("Early use row is duplicated", row.location.clone()));
        }
        keys.push((key, row.location.clone()));
    }
    if keys.windows(2).any(|pair| pair[0].0.follows(&pair[1].0)) {
        diagnostics.push(f009(
            "Early uses ledger rows are not in canonical order",
            table.location.clone(),
        ));
    }
}

/// The canonical sort position of one early-use ledger row.
///
/// `anchor` is the byte offset of the row's explicit anchor in its first-use
/// page, and is `None` when that page is outside the proved prefix: the page
/// does not exist yet, so its anchor occurrence is unobservable rather than
/// wrong.
#[derive(Clone, Eq, Ord, PartialEq, PartialOrd)]
struct EarlyUseKey {
    chapter: usize,
    anchor: Option<usize>,
    owner: usize,
    symbols: String,
}

impl EarlyUseKey {
    /// Whether this row sorts *after* the row written below it.
    ///
    /// Two rows sharing an out-of-prefix first-use page are unordered here:
    /// their canonical order is anchor occurrence in a page nobody can read, so
    /// any order the prefix imposed would be one the final ledger might have to
    /// contradict. Sharing a page means sharing a prefix state, so a pair is
    /// never half-known.
    fn follows(&self, next: &Self) -> bool {
        match self.chapter.cmp(&next.chapter) {
            std::cmp::Ordering::Greater => true,
            std::cmp::Ordering::Less => false,
            std::cmp::Ordering::Equal => self.anchor.is_some() && self > next,
        }
    }
}

/// Whether the chapter at 1-based position `number` is inside the proved
/// prefix. [`crate::manifest::ScopedSlice::chapter_index`] counts from zero.
fn chapter_in_prefix(scope: &Scope, number: usize) -> bool {
    match scope {
        Scope::Final => true,
        Scope::Through(slice) => number <= slice.chapter_index() + 1,
    }
}

fn strict_cells(line: &str) -> Option<Vec<&str>> {
    let body = line.strip_prefix("| ")?.strip_suffix(" |\n")?;
    let cells: Vec<&str> = body.split(" | ").collect();
    if cells
        .iter()
        .any(|cell| cell.contains('|') || cell.contains('\\'))
    {
        return None;
    }
    Some(cells)
}

fn valid_symbol_family(cell: &str) -> bool {
    let mut symbols = cell.split(", ");
    let mut count = 0;
    for symbol in &mut symbols {
        let Some(value) = unquote(symbol) else {
            return false;
        };
        if value.is_empty() {
            return false;
        }
        count += 1;
    }
    count > 0
}

fn unquote(cell: &str) -> Option<&str> {
    let value = cell.strip_prefix('`')?.strip_suffix('`')?;
    (!value.contains('`')).then_some(value)
}

fn check_root_locations(
    manifest: &Manifest,
    index_path: &str,
    source_index: Option<&str>,
    parsed: &ParsedBook,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let roots = roots_in_source_index(index_path, parsed);
    let observed: Vec<&str> = roots.iter().map(|root| root.id.as_str()).collect();
    let expected: Vec<&str> = manifest.roots().iter().map(|root| root.id()).collect();
    let outside = parsed
        .roots
        .values()
        .flatten()
        .find(|root| root.location.path != index_path);
    let section = source_index.and_then(|text| {
        Some((
            text.find("## Source roots\n")?,
            text.find("\n## Ownership blocks\n")?,
        ))
    });
    let outside_section = section.and_then(|(start, end)| {
        roots
            .iter()
            .find(|root| root.location.byte <= start || root.location.byte >= end)
            .copied()
    });
    if observed != expected || outside.is_some() || outside_section.is_some() {
        let location = outside
            .map(|root| root.location.clone())
            .or_else(|| outside_section.map(|root| root.location.clone()))
            .unwrap_or_else(|| Location {
                path: index_path.into(),
                byte: 0,
                line: 1,
                column: 1,
            });
        diagnostics.push(f009(
            format!(
                "source roots must occur only in `{}` and in fixed order",
                index_path.rsplit('/').next().unwrap_or(index_path)
            ),
            location,
        ));
    }
}

fn check_fragment_locations(
    manifest: &Manifest,
    parsed: &ParsedBook,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for fragment in parsed.fragments.values().flatten() {
        let Some(page) = manifest.page_of_slice(&fragment.owner) else {
            continue;
        };
        let required = manifest.path(page.file());
        if fragment.location.path != required {
            diagnostics.push(Diagnostic::new(
                "F010",
                "inventory",
                format!(
                    "fragment `{}` appears in `{}`; owner `{}` requires `{required}`",
                    fragment.id, fragment.location.path, fragment.owner
                ),
                fragment.location.clone(),
                Some(&fragment.id),
                root_for_fragment(parsed, fragment),
            ));
        }
    }
}

fn roots_in_source_index<'a>(index_path: &str, parsed: &'a ParsedBook) -> Vec<&'a Root> {
    let mut roots: Vec<&Root> = parsed
        .roots
        .values()
        .flatten()
        .filter(|root| root.location.path == index_path)
        .collect();
    roots.sort_by_key(|root| root.location.byte);
    roots
}

fn unique_fragment<'a>(parsed: &'a ParsedBook, id: &str) -> Option<&'a Fragment> {
    let fragments = parsed.fragments.get(id)?;
    (fragments.len() == 1).then(|| &fragments[0])
}

fn root_for_fragment<'a>(parsed: &'a ParsedBook, fragment: &'a Fragment) -> Option<&'a str> {
    let mut parent = fragment.parent.as_str();
    let mut seen = BTreeSet::new();
    while seen.insert(parent) {
        if parsed
            .roots
            .get(parent)
            .is_some_and(|roots| roots.len() == 1)
        {
            return Some(parent);
        }
        parent = unique_fragment(parsed, parent)?.parent.as_str();
    }
    None
}

fn owner_is_complete(manifest: &Manifest, scope: &Scope, owner: &str) -> bool {
    match scope {
        Scope::Final => true,
        Scope::Through(slice) => manifest
            .slice_order(owner)
            .is_some_and(|owner| owner <= slice.chapter_index()),
    }
}

/// The chapter page identifier a fragment definition sits on.
///
/// Chapters only: a fragment defined on the contents or on either lookup page
/// has no owning chapter, and the fragment-index row records `—` for it. The
/// compiled `PAGE_BY_OWNER` table this replaces held the chapters and nothing
/// else, and widening it to every page would silently change the expected
/// `F009` row for any book that defines a non-root fragment off a chapter.
fn page_id_for_path<'a>(manifest: &'a Manifest, path: &str) -> Option<&'a str> {
    manifest
        .chapters()
        .find(|page| manifest.path(page.file()) == path)
        .map(Page::id)
}

/// The 1-based position of `filename` among the book's chapters.
fn chapter_number(manifest: &Manifest, filename: &str) -> Option<usize> {
    manifest
        .chapters()
        .position(|page| page.file() == filename)
        .map(|index| index + 1)
}

fn grouped(value: usize) -> String {
    if value >= 1_000 {
        format!("{},{:03}", value / 1_000, value % 1_000)
    } else {
        value.to_string()
    }
}

fn line_location(index_path: &str, text: &str, byte: usize) -> Location {
    let prefix = &text[..byte];
    Location {
        path: index_path.into(),
        byte,
        line: prefix
            .bytes()
            .filter(|candidate| *candidate == b'\n')
            .count()
            + 1,
        column: 1,
    }
}

fn f009(message: impl Into<String>, location: Location) -> Diagnostic {
    Diagnostic::new("F009", "inventory", message, location, None, None)
}

/// The three bolded lead-ins a book uses for the ledger accounts it rolls up in
/// its assembly chapter. Inside a section anchored `the-closed-ledgers` each of
/// these paragraphs must be marked, so that deleting a `rollup` directive is a
/// finding rather than a silent loss of cover.
const LEDGER_ACCOUNTS: [&str; 3] = ["**Ownership.**", "**Early use.**", "**Owned source.**"];

const CLOSED_LEDGERS_ANCHOR: &str = "<a id=\"the-closed-ledgers\"></a>\n";

fn check_rollups(
    manifest: &Manifest,
    early_rows: &[String],
    parsed: &ParsedBook,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for document in parsed.documents.values() {
        for rollup in &document.rollups {
            if rollup.paragraph.is_empty() {
                diagnostics.push(f011(
                    format!(
                        "roll-up `{}` is not followed by a paragraph to check",
                        rollup.quantity
                    ),
                    rollup.location.clone(),
                ));
                continue;
            }
            let value = match rollup_value(
                manifest,
                early_rows,
                &rollup.quantity,
                rollup.argument.as_deref(),
            ) {
                Ok(value) => value,
                Err(reason) => {
                    diagnostics.push(f011(
                        format!("roll-up `{}` {reason}", rollup.quantity),
                        rollup.location.clone(),
                    ));
                    continue;
                }
            };
            // Prose is hard-wrapped, so a multi-token figure like the
            // owned-source sequence straddles line breaks. Matching happens on
            // a whitespace-normalised copy of the paragraph for that reason.
            let paragraph = unwrapped(&document.text[rollup.paragraph.clone()]);
            if !states_number(&paragraph, &value) {
                diagnostics.push(f011(
                    format!(
                        "roll-up `{}` derives {value}, which the paragraph below does not state",
                        rollup.quantity
                    ),
                    rollup.location.clone(),
                ));
            }
        }
        check_mandatory_rollups(document, diagnostics);
    }
}

/// Every ledger-account paragraph inside the closed-ledgers section carries at
/// least one mark. The section runs from its explicit anchor to the next one,
/// which is the same block structure `M102` requires of every page.
fn check_mandatory_rollups(
    document: &crate::parser::ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(start) = document.text.find(CLOSED_LEDGERS_ANCHOR) else {
        return;
    };
    let body_start = start + CLOSED_LEDGERS_ANCHOR.len();
    let end = document.text[body_start..]
        .find("\n<a id=\"")
        .map_or(document.text.len(), |offset| body_start + offset + 1);
    let mut offset = body_start;
    for line in document.text[body_start..end].split_inclusive('\n') {
        if LEDGER_ACCOUNTS
            .iter()
            .any(|lead_in| line.starts_with(lead_in))
            && !document
                .rollups
                .iter()
                .any(|rollup| rollup.paragraph.contains(&offset))
        {
            let lead_in = line
                .split_once("**")
                .and_then(|(_, rest)| rest.split_once("**"));
            diagnostics.push(f011(
                format!(
                    "ledger account `{}` in the closed-ledgers section states a roll-up no `rollup` directive marks",
                    lead_in.map_or("", |(name, _)| name)
                ),
                line_location(&document.path, &document.text, offset),
            ));
        }
        offset += line.len();
    }
}

/// The derived value of one roll-up quantity, in the ledgers' own digit form.
///
/// The first four quantities read the manifest, which `F009` has already proved
/// the source-root, ownership and totals tables equal byte for byte. The
/// early-use quantities read the ledger table instead, because the manifest's
/// `[[early-use]]` rows are a floor rather than the set
/// (`docs/specs/walkthrough-books.md`, *Early-use ledger*).
fn rollup_value(
    manifest: &Manifest,
    early_rows: &[String],
    quantity: &str,
    argument: Option<&str>,
) -> Result<String, String> {
    let named = |what: &str| -> Result<&str, String> {
        argument.ok_or_else(|| format!("requires an `of=\"…\"` {what}"))
    };
    let bare = || -> Result<(), String> {
        match argument {
            None => Ok(()),
            Some(_) => Err("takes no `of=\"…\"` argument".to_owned()),
        }
    };
    let first_uses = || -> Vec<String> {
        early_rows
            .iter()
            .filter_map(|row| {
                let cells = strict_cells(row)?;
                unquote(cells.get(1)?).map(str::to_owned)
            })
            .collect()
    };

    let count = match quantity {
        "source-roots" => {
            bare()?;
            manifest.roots().len()
        }
        "ownership-blocks" => {
            bare()?;
            manifest.blocks().len()
        }
        "ownership-blocks-owned-by" | "ownership-blocks-not-owned-by" => {
            let slice = named("slice")?;
            if manifest.slice_order(slice).is_none() {
                return Err(format!(
                    "names `{slice}`, which is not a slice of this book"
                ));
            }
            let owned = manifest
                .blocks()
                .iter()
                .filter(|block| block.owner() == slice)
                .count();
            if quantity.contains("not-owned-by") {
                manifest.blocks().len() - owned
            } else {
                owned
            }
        }
        "early-use-rows" => {
            bare()?;
            early_rows.len()
        }
        "early-use-rows-declared" => {
            bare()?;
            manifest.early_uses().len()
        }
        "early-use-rows-at" | "early-use-rows-not-at" => {
            let first_use = named("first-use location")?;
            let uses = first_uses();
            let at = uses.iter().filter(|use_| *use_ == first_use).count();
            if at == 0 {
                return Err(format!(
                    "names `{first_use}`, which no early-use row gives as its first use"
                ));
            }
            if quantity.contains("not-at") {
                uses.len() - at
            } else {
                at
            }
        }
        "chapters" => {
            bare()?;
            manifest.chapter_count()
        }
        "source-owning-chapters" => {
            bare()?;
            manifest
                .chapters()
                .filter(|page| owned_lines(manifest, page.slice().unwrap_or_default()) > 0)
                .count()
        }
        "owned-lines-total" => {
            bare()?;
            manifest.roots().iter().map(SourceRoot::lines).sum()
        }
        "owned-lines-sequence" => {
            bare()?;
            let addends: Vec<String> = manifest
                .chapters()
                .map(|page| owned_lines(manifest, page.slice().unwrap_or_default()))
                .filter(|owned| *owned > 0)
                .map(grouped)
                .collect();
            if addends.is_empty() {
                return Err("derives no addends, because no chapter owns source".to_owned());
            }
            let total: usize = manifest.roots().iter().map(SourceRoot::lines).sum();
            return Ok(format!("{} = {}", addends.join(" + "), grouped(total)));
        }
        _ => return Err("is not a quantity the ledgers derive".to_owned()),
    };
    Ok(grouped(count))
}

fn owned_lines(manifest: &Manifest, slice: &str) -> usize {
    manifest
        .blocks()
        .iter()
        .filter(|block| block.owner() == slice)
        .map(|block| block.last() - block.first() + 1)
        .sum()
}

/// Whether `value` occurs in `text` as a figure rather than inside a longer one.
/// `13` must not be satisfied by `130`, by `2,013`, or by `k13`; the boundary is
/// therefore any byte that could continue a number or a word.
fn unwrapped(text: &str) -> String {
    text.split_whitespace().collect::<Vec<&str>>().join(" ")
}

fn states_number(text: &str, value: &str) -> bool {
    let bytes = text.as_bytes();
    let mut from = 0;
    while let Some(offset) = text[from..].find(value) {
        let start = from + offset;
        let end = start + value.len();
        let before_ok = start == 0 || !continues_figure(bytes, start - 1, Side::Before);
        let after_ok = end == bytes.len() || !continues_figure(bytes, end, Side::After);
        if before_ok && after_ok {
            return true;
        }
        from = start + 1;
    }
    false
}

enum Side {
    Before,
    After,
}

/// An alphanumeric always continues the figure. A comma or a full stop continues
/// one only when a digit sits on its far side, so `1,017,` and `1,017.` ending a
/// clause are the figure while `21,017` and `1,017.5` are not.
fn continues_figure(bytes: &[u8], at: usize, side: Side) -> bool {
    let byte = bytes[at];
    if byte.is_ascii_alphanumeric() {
        return true;
    }
    if byte != b'.' && byte != b',' {
        return false;
    }
    let far = match side {
        Side::Before => at.checked_sub(1),
        Side::After => Some(at + 1),
    };
    far.and_then(|index| bytes.get(index))
        .is_some_and(u8::is_ascii_digit)
}

fn f011(message: impl Into<String>, location: Location) -> Diagnostic {
    Diagnostic::new("F011", "inventory", message, location, None, None)
}

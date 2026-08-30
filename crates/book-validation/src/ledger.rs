use std::collections::BTreeSet;

use crate::parser::{Child, Fragment, FragmentBody, ParsedBook, Root};
use crate::validator::{BLOCKS, ROOTS, SLICE_ORDER};
use crate::{BookSnapshot, Diagnostic, Location, Scope};

const SOURCE_INDEX: &str = "docs/ordinal-fs-tree/book/source-index.md";

const PAGE_BY_OWNER: &[(&str, &str, &str, usize)] = &[
    ("orientation-k11", "orientation", "01-orientation.md", 1),
    ("name-seam-k12", "name-seam", "02-name-seam.md", 2),
    (
        "reference-domain-k13",
        "reference-domain",
        "03-reference-domain.md",
        3,
    ),
    ("read-path-k14", "read-path", "04-read-path.md", 4),
    (
        "mutation-algebra-k15",
        "mutation-algebra",
        "05-mutation-algebra.md",
        5,
    ),
    (
        "filesystem-interpreter-k16",
        "filesystem-interpreter",
        "06-filesystem-interpreter.md",
        6,
    ),
    ("syllabus-cli-k17", "syllabus-cli", "07-syllabus-cli.md", 7),
    (
        "book-assembly-k18",
        "invariants-and-trade-offs",
        "08-invariants-and-trade-offs.md",
        8,
    ),
];

const EARLY_USES: &[(&str, &str, &str, &str)] = &[
    (
        "`Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName`",
        "01-orientation.md#working-vocabulary",
        "name-seam-k12",
        "Ordinal is mutable sibling position, key is stable tree identity, observed file kind is not followed, verdict separates foreign, accepted, and refused names, species controls file versus directory shape, and EntryName is the consumer parsing and composition seam.",
    ),
    (
        "`manifest-cli-binary`",
        "01-orientation.md#package-contract",
        "syllabus-cli-k17",
        "The binary declaration is CLI-owned and deferred; it maps the demonstration executable to its external consumer source and requires the CLI feature.",
    ),
    (
        "`manifest-cli-feature`",
        "01-orientation.md#package-contract",
        "syllabus-cli-k17",
        "The optional parser dependency is activated by a later CLI-owned feature range, enabled by default while library consumers may disable default features.",
    ),
    (
        "`Label`, `Status`, `reference::Parts`, `SyllabusName`",
        "01-orientation.md#insert-tour",
        "reference-domain-k13",
        "These values are the syllabus consumer's vocabulary and seam implementation, not library defaults.",
    ),
    (
        "`Snapshot`, `Entry`, `ReadGuard`",
        "01-orientation.md#insert-tour",
        "read-path-k14",
        "A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot.",
    ),
    (
        "`Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report`",
        "01-orientation.md#insert-tour",
        "mutation-algebra-k15",
        "Target names the root or a stable key, new entry carries opaque parts and optional bytes, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders.",
    ),
    (
        "`WriteGuard`, `Error`, `apply::Faults`, `apply::Run`",
        "01-orientation.md#insert-tour",
        "filesystem-interpreter-k16",
        "A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state.",
    ),
    (
        "`Cli`, `Verb`, `Streams`, `Failure`",
        "01-orientation.md#insert-tour",
        "syllabus-cli-k17",
        "Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category.",
    ),
];

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

pub(crate) fn check(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let source_index = snapshot
        .book_files
        .get(SOURCE_INDEX)
        .and_then(|bytes| std::str::from_utf8(bytes).ok());

    let source_tables = required_table(source_index, "Source roots", diagnostics);
    let ownership_tables = required_table(source_index, "Ownership blocks", diagnostics);
    let fragment_tables = required_table(source_index, "Fragment index", diagnostics);
    let early_tables = required_table(source_index, "Early uses", diagnostics);

    if let Some(table) = source_tables {
        check_exact_table(
            &table,
            "| Root ID | Source path | Lines |\n",
            "|---|---|---|\n",
            &fixed_source_rows(),
            "Source roots ledger disagrees with the fixed corpus",
            diagnostics,
        );
        check_exact_rows(
            &table,
            &directive_source_rows(parsed),
            "Source roots ledger disagrees with source-root directives",
            diagnostics,
        );
    }

    if let Some(table) = ownership_tables {
        check_exact_table(
            &table,
            "| Block ID | Root ID | Owner | Source lines | Count | State |\n",
            "|---|---|---|---|---|---|\n",
            &fixed_ownership_rows(scope),
            "Ownership blocks ledger disagrees with the fixed ownership contract",
            diagnostics,
        );
        check_exact_rows(
            &table,
            &directive_ownership_rows(parsed),
            "Ownership blocks ledger disagrees with root directives",
            diagnostics,
        );
    }

    if let Some(table) = fragment_tables {
        check_exact_table(
            &table,
            "| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |\n",
            "|---|---|---|---|---|---|---|---|\n",
            &directive_fragment_rows(parsed),
            "Fragment index disagrees with fragment directives",
            diagnostics,
        );
    }

    if let Some(table) = early_tables {
        check_early_uses(snapshot, &table, scope, diagnostics);
    }

    check_root_locations(source_index, parsed, diagnostics);
    check_fragment_locations(parsed, diagnostics);
}

fn required_table(
    source_index: Option<&str>,
    heading: &str,
    diagnostics: &mut Vec<Diagnostic>,
) -> Option<LedgerTable> {
    let tables = source_index.map_or_else(Vec::new, |text| find_tables(text, heading));
    if tables.len() == 1 {
        return tables.into_iter().next();
    }
    diagnostics.push(f009(
        format!(
            "mandatory `{heading}` ledger table occurs {} times; expected exactly once",
            tables.len()
        ),
        Location {
            path: SOURCE_INDEX.into(),
            byte: 0,
            line: 1,
            column: 1,
        },
    ));
    None
}

fn find_tables(text: &str, heading: &str) -> Vec<LedgerTable> {
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
        let header = lines.get(cursor).map(|line| (*line).to_owned());
        cursor += usize::from(header.is_some());
        let separator = lines.get(cursor).map(|line| (*line).to_owned());
        cursor += usize::from(separator.is_some());
        let mut rows = Vec::new();
        while let Some(line) = lines.get(cursor).filter(|line| line.starts_with('|')) {
            rows.push(LedgerRow {
                raw: (*line).to_owned(),
                location: line_location(text, offsets[cursor]),
            });
            cursor += 1;
        }
        let mut table = LedgerTable {
            location: line_location(text, offsets[index]),
            header,
            separator,
            rows,
        };
        if !blank {
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

fn fixed_source_rows() -> Vec<String> {
    ROOTS
        .iter()
        .map(|(id, source, lines)| format!("| `{id}` | `{source}` | {} |\n", grouped(*lines)))
        .collect()
}

fn directive_source_rows(parsed: &ParsedBook) -> Vec<String> {
    roots_in_source_index(parsed)
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

fn fixed_ownership_rows(scope: &Scope) -> Vec<String> {
    BLOCKS
        .iter()
        .map(|block| {
            ownership_row(
                block.id,
                block.root,
                block.owner,
                block.first,
                block.last,
                if owner_is_complete(scope, block.owner) {
                    "resolved"
                } else {
                    "deferred"
                },
            )
        })
        .collect()
}

fn directive_ownership_rows(parsed: &ParsedBook) -> Vec<String> {
    let mut rows = Vec::new();
    for root in roots_in_source_index(parsed) {
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

fn directive_fragment_rows(parsed: &ParsedBook) -> Vec<String> {
    let roots = roots_in_source_index(parsed);
    let mut rows = Vec::new();
    for root in roots {
        rows.push(root_fragment_row(root));
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
                page_id_for_path(&fragment.location.path).unwrap_or("—"),
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

fn root_fragment_row(root: &Root) -> String {
    let children = child_ids(&root.children);
    format!(
        "| `{}` | `source-index` | `{}` | `root` | `—` | `{}-{}` | `—` | {} |\n",
        root.id, root.id, root.range.first, root.range.last, children
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
    let header = "| Symbol family | First use | Owner | Minimum local statement | Status |\n";
    let separator = "|---|---|---|---|---|\n";
    if table.header.as_deref() != Some(header) || table.separator.as_deref() != Some(separator) {
        diagnostics.push(f009(
            "Early uses ledger has a malformed header or separator",
            table.location.clone(),
        ));
        return;
    }

    let required: Vec<String> = EARLY_USES
        .iter()
        .map(|(symbols, first_use, owner, statement)| {
            let status = if owner_is_complete(scope, owner) {
                "explained"
            } else {
                "pending"
            };
            format!("| {symbols} | `{first_use}` | `{owner}` | {statement} | `{status}` |\n")
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
        let Some(first_order) = page_order_for_filename(filename) else {
            diagnostics.push(f009(
                "Early use location names a noncanonical numbered page",
                row.location.clone(),
            ));
            continue;
        };
        let Some(owner_order) = slice_order(owner) else {
            diagnostics.push(f009(
                "Early use owner is not a canonical slice",
                row.location.clone(),
            ));
            continue;
        };
        let page_path = format!("docs/ordinal-fs-tree/book/{filename}");
        let anchor_line = format!("<a id=\"{anchor}\"></a>\n");
        let anchor_byte = snapshot
            .book_files
            .get(&page_path)
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .and_then(|text| text.find(&anchor_line));
        let expected_status = if owner_is_complete(scope, owner) {
            "explained"
        } else {
            "pending"
        };
        if first_order > owner_order
            || anchor_byte.is_none()
            || status != expected_status
            || cells[3].is_empty()
        {
            diagnostics.push(f009(
                "Early use row disagrees with page, owner, anchor, or scope state",
                row.location.clone(),
            ));
        }
        let key = (
            first_order,
            anchor_byte.unwrap_or(usize::MAX),
            owner_order,
            cells[0].to_owned(),
        );
        if !seen.insert(key.clone()) {
            diagnostics.push(f009("Early use row is duplicated", row.location.clone()));
        }
        keys.push((key, row.location.clone()));
    }
    if keys.windows(2).any(|pair| pair[0].0 > pair[1].0) {
        diagnostics.push(f009(
            "Early uses ledger rows are not in canonical order",
            table.location.clone(),
        ));
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
    source_index: Option<&str>,
    parsed: &ParsedBook,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let roots = roots_in_source_index(parsed);
    let observed: Vec<&str> = roots.iter().map(|root| root.id.as_str()).collect();
    let expected: Vec<&str> = ROOTS.iter().map(|(id, _, _)| *id).collect();
    let outside = parsed
        .roots
        .values()
        .flatten()
        .find(|root| root.location.path != SOURCE_INDEX);
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
                path: SOURCE_INDEX.into(),
                byte: 0,
                line: 1,
                column: 1,
            });
        diagnostics.push(f009(
            "source roots must occur only in source-index.md and in fixed order",
            location,
        ));
    }
}

fn check_fragment_locations(parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
    for fragment in parsed.fragments.values().flatten() {
        let Some((_, _, filename, _)) = PAGE_BY_OWNER
            .iter()
            .find(|(owner, _, _, _)| *owner == fragment.owner)
        else {
            continue;
        };
        let required = format!("docs/ordinal-fs-tree/book/{filename}");
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

fn roots_in_source_index(parsed: &ParsedBook) -> Vec<&Root> {
    let mut roots: Vec<&Root> = parsed
        .roots
        .values()
        .flatten()
        .filter(|root| root.location.path == SOURCE_INDEX)
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

fn owner_is_complete(scope: &Scope, owner: &str) -> bool {
    match scope {
        Scope::Final => true,
        Scope::Through(slice) => slice_order(owner)
            .zip(slice_order(slice.as_str()))
            .is_some_and(|(owner, through)| owner <= through),
    }
}

fn slice_order(slice: &str) -> Option<usize> {
    SLICE_ORDER.iter().position(|candidate| *candidate == slice)
}

fn page_id_for_path(path: &str) -> Option<&'static str> {
    PAGE_BY_OWNER.iter().find_map(|(_, id, filename, _)| {
        (path == format!("docs/ordinal-fs-tree/book/{filename}")).then_some(*id)
    })
}

fn page_order_for_filename(filename: &str) -> Option<usize> {
    PAGE_BY_OWNER
        .iter()
        .find_map(|(_, _, candidate, order)| (*candidate == filename).then_some(*order))
}

fn grouped(value: usize) -> String {
    if value >= 1_000 {
        format!("{},{:03}", value / 1_000, value % 1_000)
    } else {
        value.to_string()
    }
}

fn line_location(text: &str, byte: usize) -> Location {
    let prefix = &text[..byte];
    Location {
        path: SOURCE_INDEX.into(),
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

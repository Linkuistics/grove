use std::collections::BTreeSet;
use std::ops::Range;

use crate::parser::{ParsedBook, ParsedDocument};
use crate::{BookSnapshot, Diagnostic, Location, Scope};

const ROOT: &str = "docs/ordinal-fs-tree/book/";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownLink {
    pub label: String,
    pub destination: String,
    pub byte: usize,
    pub line: usize,
    pub column: usize,
    pub valid_syntax: bool,
}

struct Page {
    file: &'static str,
    id: &'static str,
    title: &'static str,
    identity: &'static str,
}

const CHAPTERS: &[Page] = &[
    Page { file: "01-orientation.md", id: "orientation", title: "Orientation", identity: "<!-- book-page id=\"orientation\" slice=\"orientation-k11\" order=\"1\" -->" },
    Page { file: "02-name-seam.md", id: "name-seam", title: "Name seam", identity: "<!-- book-page id=\"name-seam\" slice=\"name-seam-k12\" order=\"2\" -->" },
    Page { file: "03-reference-domain.md", id: "reference-domain", title: "Reference domain", identity: "<!-- book-page id=\"reference-domain\" slice=\"reference-domain-k13\" order=\"3\" -->" },
    Page { file: "04-read-path.md", id: "read-path", title: "Read path", identity: "<!-- book-page id=\"read-path\" slice=\"read-path-k14\" order=\"4\" -->" },
    Page { file: "05-mutation-algebra.md", id: "mutation-algebra", title: "Mutation algebra", identity: "<!-- book-page id=\"mutation-algebra\" slice=\"mutation-algebra-k15\" order=\"5\" -->" },
    Page { file: "06-filesystem-interpreter.md", id: "filesystem-interpreter", title: "Filesystem interpreter", identity: "<!-- book-page id=\"filesystem-interpreter\" slice=\"filesystem-interpreter-k16\" order=\"6\" -->" },
    Page { file: "07-syllabus-cli.md", id: "syllabus-cli", title: "Syllabus CLI", identity: "<!-- book-page id=\"syllabus-cli\" slice=\"syllabus-cli-k17\" order=\"7\" -->" },
    Page { file: "08-invariants-and-trade-offs.md", id: "invariants-and-trade-offs", title: "Invariants and trade-offs", identity: "<!-- book-page id=\"invariants-and-trade-offs\" slice=\"book-assembly-k18\" order=\"8\" -->" },
];

const FIXED: &[Page] = &[
    Page {
        file: "README.md",
        id: "contents",
        title: "Ordinal filesystem tree",
        identity: "<!-- book-page id=\"contents\" role=\"contents\" -->",
    },
    Page {
        file: "concept-index.md",
        id: "concept-index",
        title: "Concept index",
        identity: "<!-- book-page id=\"concept-index\" role=\"lookup\" -->",
    },
    Page {
        file: "source-index.md",
        id: "source-index",
        title: "Source index",
        identity: "<!-- book-page id=\"source-index\" role=\"lookup\" -->",
    },
];

pub(crate) fn check(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let chapter_count = match scope {
        Scope::Final => CHAPTERS.len(),
        Scope::Through(slice) => crate::validator::SLICE_ORDER
            .iter()
            .position(|candidate| *candidate == slice.as_str())
            .map_or(0, |index| index + 1),
    };
    let expected: BTreeSet<String> = FIXED
        .iter()
        .chain(CHAPTERS[..chapter_count].iter())
        .map(|page| format!("{ROOT}{}", page.file))
        .collect();
    let actual = snapshot.book_entries.clone();
    for path in expected.difference(&actual) {
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!("required book page `{path}` is missing"),
            command_location(path),
        ));
    }
    for path in actual.difference(&expected) {
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!("book page `{path}` is outside the canonical scope inventory"),
            command_location(path),
        ));
    }
    for path in expected.intersection(&snapshot.non_regular_book_entries) {
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!("required book page `{path}` is not a regular file"),
            command_location(path),
        ));
    }

    for page in FIXED.iter().chain(CHAPTERS[..chapter_count].iter()) {
        let path = format!("{ROOT}{}", page.file);
        let Some(document) = parsed.documents.get(&path) else {
            continue;
        };
        check_identity(page, document, diagnostics);
        check_headings(page, document, diagnostics);
        check_fences(document, diagnostics);
        check_fragment_introductions(parsed, document, diagnostics);
        check_links(snapshot, parsed, document, diagnostics);
    }
    for (index, page) in CHAPTERS[..chapter_count].iter().enumerate() {
        let path = format!("{ROOT}{}", page.file);
        if let Some(document) = parsed.documents.get(&path) {
            check_navigation(index, chapter_count, document, diagnostics);
        }
    }
    check_contents(parsed, chapter_count, diagnostics);
}

fn check_identity(page: &Page, document: &ParsedDocument, diagnostics: &mut Vec<Diagnostic>) {
    let matching: Vec<_> = document
        .page_directives
        .iter()
        .filter(|directive| directive.raw == page.identity)
        .collect();
    if document.page_directives.len() != 1 || matching.len() != 1 {
        let location = document.page_directives.first().map_or_else(
            || command_location(&format!("{ROOT}{}", page.file)),
            |value| value.location.clone(),
        );
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!(
                "page `{}` must declare exactly one canonical identity `{}`",
                page.file, page.id
            ),
            location,
        ));
    }
    let mut lines = document.text.split_inclusive('\n');
    let first = lines.next().unwrap_or_default().trim_end_matches('\n');
    let second = lines.next().unwrap_or_default().trim_end_matches('\n');
    if first != format!("# {}", page.title) || second != page.identity {
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!(
                "page `{}` must begin with its canonical H1 and identity",
                page.file
            ),
            Location {
                path: format!("{ROOT}{}", page.file),
                byte: 0,
                line: 1,
                column: 1,
            },
        ));
    }
}

fn check_headings(page: &Page, document: &ParsedDocument, diagnostics: &mut Vec<Diagnostic>) {
    let mut previous = 0;
    let mut h1_count = 0;
    let mut anchors = BTreeSet::new();
    let lines = visible_lines(document);
    for (index, (byte, line)) in lines.iter().enumerate() {
        if let Some(level) = heading_level(line) {
            if level == 1 {
                h1_count += 1;
            }
            if previous != 0 && level > previous + 1 {
                diagnostics.push(markdown_diagnostic(
                    "M102",
                    "heading level skips over an intermediate level",
                    line_location(&format!("{ROOT}{}", page.file), &document.text, *byte),
                ));
            }
            previous = level;
        }
        if let Some(anchor) = explicit_anchor(line) {
            if !valid_anchor(anchor) || !anchors.insert(anchor.to_owned()) {
                diagnostics.push(markdown_diagnostic(
                    "M102",
                    format!("explicit anchor `{anchor}` is invalid or duplicated"),
                    line_location(&format!("{ROOT}{}", page.file), &document.text, *byte),
                ));
            }
            if lines
                .get(index + 1)
                .and_then(|(_, next)| heading_level(next))
                .is_none()
            {
                diagnostics.push(markdown_diagnostic(
                    "M102",
                    format!("explicit anchor `{anchor}` must immediately precede a heading"),
                    line_location(&format!("{ROOT}{}", page.file), &document.text, *byte),
                ));
            }
        }
    }
    if h1_count != 1 {
        diagnostics.push(markdown_diagnostic(
            "M102",
            format!(
                "page `{}` has {h1_count} H1 headings; expected one",
                page.file
            ),
            command_location(&format!("{ROOT}{}", page.file)),
        ));
    }
}

fn check_navigation(
    index: usize,
    count: usize,
    document: &ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expected = navigation(index, count);
    let lines: Vec<&str> = document.text.lines().collect();
    let top = lines.get(2).copied().unwrap_or_default();
    let bottom = lines
        .iter()
        .rev()
        .find(|line| !line.is_empty())
        .copied()
        .unwrap_or_default();
    if top != expected || bottom != expected {
        diagnostics.push(markdown_diagnostic(
            "M103",
            format!("numbered page navigation must be `{expected}` at top and bottom"),
            line_location(&document.path, &document.text, 0),
        ));
    }
}

fn navigation(index: usize, count: usize) -> String {
    let mut parts = Vec::new();
    if index > 0 {
        parts.push(format!(
            "[Previous: {}]({})",
            CHAPTERS[index - 1].title,
            CHAPTERS[index - 1].file
        ));
    }
    parts.push("[Contents](README.md)".into());
    if index + 1 < count {
        parts.push(format!(
            "[Next: {}]({})",
            CHAPTERS[index + 1].title,
            CHAPTERS[index + 1].file
        ));
    }
    parts.join(" | ")
}

fn check_fences(document: &ParsedDocument, diagnostics: &mut Vec<Diagnostic>) {
    for fence in &document.ordinary_fences {
        if matches!(fence.info.as_str(), "rust" | "toml") {
            diagnostics.push(markdown_diagnostic(
                "M104",
                format!(
                    "ordinary `{}` fence contains untracked production source",
                    fence.info
                ),
                fence.location.clone(),
            ));
        }
    }
}

fn check_fragment_introductions(
    parsed: &ParsedBook,
    document: &ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for fragment in parsed.fragments.values().flatten().filter(|fragment| {
        fragment.location.path == document.path
            && matches!(fragment.body, crate::parser::FragmentBody::Literal(_))
    }) {
        let prefix = &document.text[..fragment.location.byte];
        let previous = prefix.lines().rev().find(|line| !line.trim().is_empty());
        if !previous.is_some_and(paragraph_line) {
            diagnostics.push(markdown_diagnostic(
                "M105",
                format!(
                    "literal fragment `{}` must have a prose paragraph as its nearest preceding nonblank block",
                    fragment.id
                ),
                fragment.location.clone(),
            ));
        }
    }
}

fn paragraph_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    !trimmed.is_empty()
        && heading_level(trimmed).is_none()
        && !trimmed.starts_with("<!--")
        && !trimmed.starts_with('|')
        && !trimmed.starts_with('>')
        && !["- ", "* ", "+ "]
            .iter()
            .any(|prefix| trimmed.starts_with(prefix))
        && !trimmed
            .split_once(". ")
            .is_some_and(|(prefix, _)| prefix.bytes().all(|byte| byte.is_ascii_digit()))
        && !trimmed.starts_with("```")
        && !trimmed.starts_with("~~~")
}

fn check_links(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    document: &ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for link in scan_links(&document.text, &document.opaque_ranges) {
        let location = Location {
            path: document.path.clone(),
            byte: link.byte,
            line: link.line,
            column: link.column,
        };
        if !link.valid_syntax || matches!(link.label.as_str(), "here" | "this" | "more") {
            diagnostics.push(markdown_diagnostic(
                "M201",
                format!(
                    "link `{}` is outside the accepted Markdown link subset",
                    link.destination
                ),
                location,
            ));
            continue;
        }
        if external_destination(&link.destination) {
            if !valid_external_destination(&link.destination) {
                diagnostics.push(markdown_diagnostic(
                    "M201",
                    format!("external URL `{}` is malformed", link.destination),
                    location,
                ));
            }
            continue;
        }
        let Some((target, anchor)) = resolve_local(&location.path, &link.destination) else {
            diagnostics.push(markdown_diagnostic(
                "M201",
                format!("link `{}` escapes the repository scope", link.destination),
                location,
            ));
            continue;
        };
        let target_bytes = snapshot
            .book_files
            .get(&target)
            .or_else(|| snapshot.source_files.get(&target));
        let Some(target_bytes) = target_bytes else {
            diagnostics.push(markdown_diagnostic(
                "M201",
                format!(
                    "link `{}` resolves to missing repository file `{target}`",
                    link.destination
                ),
                location,
            ));
            continue;
        };
        if let Some(anchor) = anchor {
            let has_anchor = std::str::from_utf8(target_bytes).ok().is_some_and(|text| {
                parsed
                    .documents
                    .get(&target)
                    .is_some_and(|target_document| {
                        visible_explicit_anchor(text, &target_document.opaque_ranges, anchor)
                    })
            });
            if !target.ends_with(".md") || !valid_anchor(anchor) || !has_anchor {
                diagnostics.push(markdown_diagnostic(
                    "M201",
                    format!(
                        "link `{}` names no explicit Markdown anchor in `{target}`",
                        link.destination
                    ),
                    location,
                ));
            }
        }
    }
}

fn check_contents(parsed: &ParsedBook, chapter_count: usize, diagnostics: &mut Vec<Diagnostic>) {
    let readme_path = format!("{ROOT}README.md");
    if let Some(readme) = parsed.documents.get(&readme_path) {
        let destinations: Vec<String> = scan_links(&readme.text, &readme.opaque_ranges)
            .into_iter()
            .filter(|link| link.valid_syntax)
            .map(|link| link.destination)
            .collect();
        let expected_chapters: Vec<&str> = CHAPTERS[..chapter_count]
            .iter()
            .map(|page| page.file)
            .collect();
        let chapter_destinations: Vec<&str> = destinations
            .iter()
            .map(String::as_str)
            .filter(|destination| CHAPTERS.iter().any(|page| page.file == *destination))
            .collect();
        if chapter_destinations != expected_chapters {
            diagnostics.push(markdown_diagnostic(
                "M103",
                "README contents must link each in-scope chapter exactly once in canonical order",
                command_location(&readme_path),
            ));
        }
        for file in ["concept-index.md", "source-index.md"] {
            if destinations
                .iter()
                .filter(|value| value.as_str() == file)
                .count()
                != 1
            {
                diagnostics.push(markdown_diagnostic(
                    "M103",
                    format!("README contents must link canonical page `{file}` exactly once"),
                    command_location(&readme_path),
                ));
            }
        }
    }
    for file in ["concept-index.md", "source-index.md"] {
        let path = format!("{ROOT}{file}");
        if let Some(document) = parsed.documents.get(&path) {
            let has_contents = scan_links(&document.text, &document.opaque_ranges)
                .iter()
                .any(|link| link.valid_syntax && link.destination == "README.md");
            if !has_contents {
                diagnostics.push(markdown_diagnostic(
                    "M103",
                    format!("lookup page `{file}` must link back to README.md"),
                    command_location(&path),
                ));
            }
        }
    }
}

fn visible_explicit_anchor(text: &str, opaque_ranges: &[Range<usize>], anchor: &str) -> bool {
    let mut byte = 0;
    for raw in text.split_inclusive('\n') {
        if !opaque_ranges.iter().any(|range| range.contains(&byte))
            && explicit_anchor(raw.trim_end_matches('\n')) == Some(anchor)
        {
            return true;
        }
        byte += raw.len();
    }
    false
}

pub fn scan_markdown_links(markdown: &str) -> Vec<MarkdownLink> {
    let path = "__standalone_markdown__.md";
    let mut snapshot = BookSnapshot::default();
    snapshot
        .book_files
        .insert(path.to_owned(), markdown.as_bytes().to_vec());
    let parsed = crate::parser::parse(&snapshot);
    let opaque_ranges = parsed
        .documents
        .get(path)
        .map_or(&[][..], |document| document.opaque_ranges.as_slice());
    scan_links(markdown, opaque_ranges)
}

fn scan_links(markdown: &str, opaque_ranges: &[Range<usize>]) -> Vec<MarkdownLink> {
    let bytes = markdown.as_bytes();
    let mut links = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        if let Some(range) = opaque_ranges.iter().find(|range| range.contains(&index)) {
            index = range.end;
            continue;
        }
        if bytes[index] == b'`' && !is_escaped(bytes, index) {
            let count = bytes[index..]
                .iter()
                .take_while(|byte| **byte == b'`')
                .count();
            if let Some(after_close) = exact_code_span_end(bytes, index + count, count) {
                index = after_close;
                continue;
            }
            index += count;
            continue;
        }
        let label_start = if bytes[index] == b'!'
            && bytes.get(index + 1) == Some(&b'[')
            && !is_escaped(bytes, index)
        {
            index + 2
        } else if bytes[index] == b'[' && !is_escaped(bytes, index) {
            index + 1
        } else {
            index += 1;
            continue;
        };
        let Some(relative_separator) = markdown[label_start..].find("](") else {
            index = label_start;
            continue;
        };
        let separator = label_start + relative_separator;
        if markdown[index..separator].contains('\n') {
            index = label_start;
            continue;
        }
        let destination_start = separator + 2;
        let close = markdown[destination_start..]
            .find(')')
            .map(|relative| destination_start + relative);
        let destination_end = close.unwrap_or_else(|| {
            markdown[destination_start..]
                .find('\n')
                .map_or(markdown.len(), |relative| destination_start + relative)
        });
        let label = &markdown[label_start..separator];
        let raw_destination = &markdown[destination_start..destination_end];
        let destination = raw_destination
            .split(char::is_whitespace)
            .next()
            .unwrap_or_default();
        let valid_syntax = close.is_some()
            && !label.is_empty()
            && !label.contains('[')
            && !label.contains(']')
            && !destination.is_empty()
            && destination == raw_destination
            && !destination.contains('\\');
        let prefix = &markdown[..index];
        links.push(MarkdownLink {
            label: label.into(),
            destination: destination.into(),
            byte: index,
            line: prefix.bytes().filter(|byte| *byte == b'\n').count() + 1,
            column: prefix
                .rfind('\n')
                .map_or(prefix.len() + 1, |newline| prefix.len() - newline),
            valid_syntax,
        });
        index = close.map_or(destination_end, |value| value + 1);
    }
    links
}

fn is_escaped(bytes: &[u8], index: usize) -> bool {
    let slash_count = bytes[..index]
        .iter()
        .rev()
        .take_while(|byte| **byte == b'\\')
        .count();
    slash_count % 2 == 1
}

fn exact_code_span_end(bytes: &[u8], mut index: usize, opening_count: usize) -> Option<usize> {
    while index < bytes.len() {
        if bytes[index] != b'`' || is_escaped(bytes, index) {
            index += 1;
            continue;
        }
        let count = bytes[index..]
            .iter()
            .take_while(|byte| **byte == b'`')
            .count();
        if count == opening_count {
            return Some(index + count);
        }
        index += count;
    }
    None
}

pub(crate) fn external_destination(destination: &str) -> bool {
    destination.starts_with("http://")
        || destination.starts_with("https://")
        || destination.starts_with("mailto:")
}

fn valid_external_destination(destination: &str) -> bool {
    if destination
        .bytes()
        .any(|byte| byte.is_ascii_control() || byte == b' ')
    {
        return false;
    }
    if let Some(address) = destination.strip_prefix("mailto:") {
        let mut parts = address.split('@');
        let (Some(local), Some(domain), None) = (parts.next(), parts.next(), parts.next()) else {
            return false;
        };
        return valid_mail_local(local) && domain.contains('.') && valid_hostname(domain);
    }
    let authority = destination
        .strip_prefix("https://")
        .or_else(|| destination.strip_prefix("http://"));
    authority.is_some_and(|tail| {
        let authority = tail.split(['/', '?', '#']).next().unwrap_or_default();
        valid_http_authority(authority)
    })
}

fn valid_mail_local(local: &str) -> bool {
    !local.is_empty()
        && !local.starts_with('.')
        && !local.ends_with('.')
        && !local.contains("..")
        && local
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'+' | b'-'))
}

fn valid_http_authority(authority: &str) -> bool {
    if authority.is_empty() || authority.contains('@') {
        return false;
    }
    if let Some(rest) = authority.strip_prefix('[') {
        let Some((address, suffix)) = rest.split_once(']') else {
            return false;
        };
        return address.parse::<std::net::Ipv6Addr>().is_ok() && valid_port_suffix(suffix);
    }
    if authority.matches(':').count() > 1 {
        return false;
    }
    let (host, port) = authority
        .rsplit_once(':')
        .map_or((authority, None), |(host, port)| (host, Some(port)));
    valid_hostname(host) && port.is_none_or(valid_port)
}

fn valid_port_suffix(suffix: &str) -> bool {
    suffix.is_empty() || suffix.strip_prefix(':').is_some_and(valid_port)
}

fn valid_port(port: &str) -> bool {
    !port.is_empty()
        && port.bytes().all(|byte| byte.is_ascii_digit())
        && port.parse::<u16>().is_ok()
}

fn valid_hostname(host: &str) -> bool {
    !host.is_empty()
        && host.split('.').all(|label| {
            !label.is_empty()
                && !label.starts_with('-')
                && !label.ends_with('-')
                && label
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}

pub(crate) fn resolve_local<'a>(
    source: &str,
    destination: &'a str,
) -> Option<(String, Option<&'a str>)> {
    let (path, anchor) = destination
        .split_once('#')
        .map_or((destination, None), |(path, anchor)| (path, Some(anchor)));
    if path.starts_with('/') || path.contains(':') || anchor == Some("") {
        return None;
    }
    let mut components: Vec<&str> = source.split('/').collect();
    components.pop();
    if !path.is_empty() {
        for component in path.split('/') {
            match component {
                "" => return None,
                "." => {}
                ".." => {
                    components.pop()?;
                }
                value => components.push(value),
            }
        }
    }
    Some((components.join("/"), anchor))
}

fn visible_lines(document: &ParsedDocument) -> Vec<(usize, &str)> {
    let mut result = Vec::new();
    let mut byte = 0;
    for raw in document.text.split_inclusive('\n') {
        if !document
            .opaque_ranges
            .iter()
            .any(|range| range.contains(&byte))
        {
            result.push((byte, raw.trim_end_matches('\n')));
        }
        byte += raw.len();
    }
    result
}

fn heading_level(line: &str) -> Option<usize> {
    let level = line.bytes().take_while(|byte| *byte == b'#').count();
    (level > 0 && level <= 6 && line.as_bytes().get(level) == Some(&b' ')).then_some(level)
}

fn explicit_anchor(line: &str) -> Option<&str> {
    line.strip_prefix("<a id=\"")?.strip_suffix("\"></a>")
}

fn valid_anchor(anchor: &str) -> bool {
    let Some(first) = anchor.bytes().next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && !anchor.contains("--")
        && !anchor.ends_with('-')
        && anchor
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn markdown_diagnostic(code: &str, message: impl Into<String>, location: Location) -> Diagnostic {
    Diagnostic::new(code, "markdown", message, location, None, None)
}

fn command_location(path: &str) -> Location {
    Location {
        path: path.into(),
        byte: 0,
        line: 0,
        column: 0,
    }
}

fn line_location(path: &str, text: &str, byte: usize) -> Location {
    let prefix = &text[..byte.min(text.len())];
    Location {
        path: path.into(),
        byte,
        line: prefix
            .bytes()
            .filter(|candidate| *candidate == b'\n')
            .count()
            + 1,
        column: prefix
            .rfind('\n')
            .map_or(prefix.len() + 1, |newline| prefix.len() - newline),
    }
}

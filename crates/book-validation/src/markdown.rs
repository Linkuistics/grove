use std::collections::BTreeSet;
use std::ops::Range;

use crate::manifest::{Manifest, Page, Role};
use crate::parser::{ParsedBook, ParsedDocument};
use crate::{BookSnapshot, Diagnostic, Location, Scope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MarkdownLink {
    pub label: String,
    pub destination: String,
    pub byte: usize,
    pub line: usize,
    pub column: usize,
    pub valid_syntax: bool,
}

pub(crate) fn check(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest = &snapshot.manifest;
    let chapter_count = match scope {
        Scope::Final => manifest.chapter_count(),
        Scope::Through(slice) => slice.chapter_index() + 1,
    };
    let in_scope: Vec<&Page> = pages_in_scope(manifest, chapter_count).collect();
    let mut expected: BTreeSet<String> = in_scope
        .iter()
        .map(|page| manifest.path(page.file()))
        .collect();
    expected.insert(manifest.manifest_path());
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

    for page in &in_scope {
        let path = manifest.path(page.file());
        let Some(document) = parsed.documents.get(&path) else {
            continue;
        };
        check_identity(manifest, page, document, diagnostics);
        check_headings(manifest, page, document, diagnostics);
        check_fences(document, diagnostics);
        check_fragment_introductions(parsed, document, diagnostics);
        check_links(snapshot, parsed, document, diagnostics);
    }
    for (index, page) in manifest.chapters().take(chapter_count).enumerate() {
        let path = manifest.path(page.file());
        if let Some(document) = parsed.documents.get(&path) {
            check_navigation(manifest, index, chapter_count, document, diagnostics);
        }
    }
    check_contents(manifest, parsed, chapter_count, diagnostics);
    check_guide_citation(snapshot, parsed, diagnostics);
    check_declared_anchors(snapshot, diagnostics);
}

/// The contents page, the chapters of the requested prefix, and both lookup
/// pages. Both indexes exist from the first slice and grow with the prefix.
fn pages_in_scope(manifest: &Manifest, chapter_count: usize) -> impl Iterator<Item = &Page> {
    manifest
        .pages()
        .iter()
        .filter(move |page| match page.role() {
            Role::Contents | Role::Lookup => true,
            Role::Chapter => manifest
                .slice_order(page.slice().unwrap_or_default())
                .is_some_and(|order| order < chapter_count),
        })
}

fn check_identity(
    manifest: &Manifest,
    page: &Page,
    document: &ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let identity = page.identity();
    let matching: Vec<_> = document
        .page_directives
        .iter()
        .filter(|directive| directive.raw == identity)
        .collect();
    if document.page_directives.len() != 1 || matching.len() != 1 {
        let location = document.page_directives.first().map_or_else(
            || command_location(&manifest.path(page.file())),
            |value| value.location.clone(),
        );
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!(
                "page `{}` must declare exactly one canonical identity `{}`",
                page.file(),
                page.id()
            ),
            location,
        ));
    }
    let mut lines = document.text.split_inclusive('\n');
    let first = lines.next().unwrap_or_default().trim_end_matches('\n');
    let second = lines.next().unwrap_or_default().trim_end_matches('\n');
    if first != format!("# {}", page.title()) || second != identity {
        diagnostics.push(markdown_diagnostic(
            "M101",
            format!(
                "page `{}` must begin with its canonical H1 and identity",
                page.file()
            ),
            Location {
                path: manifest.path(page.file()),
                byte: 0,
                line: 1,
                column: 1,
            },
        ));
    }
}

fn check_headings(
    manifest: &Manifest,
    page: &Page,
    document: &ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let path = manifest.path(page.file());
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
                    line_location(&path, &document.text, *byte),
                ));
            }
            previous = level;
        }
        if let Some(anchor) = explicit_anchor(line) {
            if !valid_anchor(anchor) || !anchors.insert(anchor.to_owned()) {
                diagnostics.push(markdown_diagnostic(
                    "M102",
                    format!("explicit anchor `{anchor}` is invalid or duplicated"),
                    line_location(&path, &document.text, *byte),
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
                    line_location(&path, &document.text, *byte),
                ));
            }
        }
    }
    if h1_count != 1 {
        diagnostics.push(markdown_diagnostic(
            "M102",
            format!(
                "page `{}` has {h1_count} H1 headings; expected one",
                page.file()
            ),
            command_location(&path),
        ));
    }
}

fn check_navigation(
    manifest: &Manifest,
    index: usize,
    count: usize,
    document: &ParsedDocument,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let expected = navigation(manifest, index, count);
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

fn navigation(manifest: &Manifest, index: usize, count: usize) -> String {
    let mut parts = Vec::new();
    if let Some(previous) = index
        .checked_sub(1)
        .and_then(|index| manifest.chapter(index))
    {
        parts.push(format!(
            "[Previous: {}]({})",
            previous.title(),
            previous.file()
        ));
    }
    let contents = manifest
        .contents_page()
        .map_or_else(String::new, |page| format!("[Contents]({})", page.file()));
    parts.push(contents);
    if let Some(next) = manifest.chapter(index + 1).filter(|_| index + 1 < count) {
        parts.push(format!("[Next: {}]({})", next.title(), next.file()));
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
        // An empty anchor is rejected, like any other unresolvable destination —
        // but "escapes the repository scope" describes a `../..` climb, and says
        // nothing to an author who wrote a trailing `#`. Now that an empty *path*
        // is legal, the two failures are easy to confuse, so this one names its
        // own remedy.
        if link.destination.ends_with('#') {
            diagnostics.push(markdown_diagnostic(
                "M201",
                format!(
                    "link `{}` names an empty anchor; drop the trailing `#`, or name the explicit anchor it should reach",
                    link.destination
                ),
                location,
            ));
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
            .or_else(|| snapshot.source_files.get(&target))
            .or_else(|| snapshot.outbound_files.get(&target));
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
            // A citation into a declared outbound document is checked against
            // the *declaration*: the anchor must be one the manifest reserves.
            // Whether the target carries it is reported once, against the
            // manifest, by `check_declared_anchors` — a book must not be able
            // to reserve an anchor nothing guarantees, cited or not.
            if let Some(document) = snapshot
                .manifest
                .outbound_documents()
                .find(|document| document.path() == target)
            {
                if !document.anchors().iter().any(|declared| declared == anchor) {
                    diagnostics.push(markdown_diagnostic(
                        "M201",
                        format!(
                            "link `{}` names anchor `{anchor}`, which this book's manifest does not declare for `{target}`",
                            link.destination
                        ),
                        location,
                    ));
                }
                continue;
            }
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

fn check_contents(
    manifest: &Manifest,
    parsed: &ParsedBook,
    chapter_count: usize,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let Some(contents_page) = manifest.contents_page() else {
        return;
    };
    let readme_path = manifest.path(contents_page.file());
    if let Some(readme) = parsed.documents.get(&readme_path) {
        let destinations: Vec<String> = scan_links(&readme.text, &readme.opaque_ranges)
            .into_iter()
            .filter(|link| link.valid_syntax)
            .map(|link| link.destination)
            .collect();
        let expected_chapters: Vec<&str> = manifest
            .chapters()
            .take(chapter_count)
            .map(Page::file)
            .collect();
        let chapter_destinations: Vec<&str> = destinations
            .iter()
            .map(String::as_str)
            .filter(|destination| manifest.chapters().any(|page| page.file() == *destination))
            .collect();
        if chapter_destinations != expected_chapters {
            diagnostics.push(markdown_diagnostic(
                "M103",
                "README contents must link each in-scope chapter exactly once in canonical order",
                command_location(&readme_path),
            ));
        }
        for page in manifest.lookup_pages() {
            if destinations
                .iter()
                .filter(|value| value.as_str() == page.file())
                .count()
                != 1
            {
                diagnostics.push(markdown_diagnostic(
                    "M103",
                    format!(
                        "README contents must link canonical page `{}` exactly once",
                        page.file()
                    ),
                    command_location(&readme_path),
                ));
            }
        }
    }
    for page in manifest.lookup_pages() {
        let path = manifest.path(page.file());
        if let Some(document) = parsed.documents.get(&path) {
            let has_contents = scan_links(&document.text, &document.opaque_ranges)
                .iter()
                .any(|link| link.valid_syntax && link.destination == contents_page.file());
            if !has_contents {
                diagnostics.push(markdown_diagnostic(
                    "M103",
                    format!(
                        "lookup page `{}` must link back to {}",
                        page.file(),
                        contents_page.file()
                    ),
                    command_location(&path),
                ));
            }
        }
    }
}

/// Rule 1 of *Outbound links*: a book declaring a `[guide]` path cites one of
/// its declared guide anchors from its `README.md` reader contract.
///
/// A book declaring `omitted` is checked by omission and not here: it declares
/// no guide path, so the guide is in no book's permitted-target set and a
/// `README.md` citing it fails as an unresolvable link.
fn check_guide_citation(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let manifest = &snapshot.manifest;
    let (Some(guide), Some(contents_page)) =
        (manifest.guide().declared(), manifest.contents_page())
    else {
        return;
    };
    let readme_path = manifest.path(contents_page.file());
    let Some(readme) = parsed.documents.get(&readme_path) else {
        return;
    };
    let mut cites_an_anchor = false;
    for link in scan_links(&readme.text, &readme.opaque_ranges)
        .iter()
        .filter(|link| link.valid_syntax)
    {
        let Some((target, anchor)) = resolve_local(&readme_path, &link.destination) else {
            continue;
        };
        if target != guide.path() {
            continue;
        }
        if anchor.is_some() {
            // Whether the named anchor is one the manifest declares is
            // `check_links`' finding, so a citation is counted here and judged
            // there.
            cites_an_anchor = true;
        } else {
            diagnostics.push(markdown_diagnostic(
                "M201",
                format!(
                    "guide link `{}` in `{}` carries no anchor; a book declaring a guide cites one of its declared anchors",
                    link.destination,
                    contents_page.file()
                ),
                Location {
                    path: readme_path.clone(),
                    byte: link.byte,
                    line: link.line,
                    column: link.column,
                },
            ));
        }
    }
    if !cites_an_anchor {
        diagnostics.push(markdown_diagnostic(
            "M201",
            format!(
                "`{}` must cite one of the guide anchors declared in the manifest for `{}`",
                contents_page.file(),
                guide.path()
            ),
            command_location(&readme_path),
        ));
    }
}

/// Rule 3 of *Outbound links*: every anchor the manifest declares exists in its
/// target document, reported once per missing anchor **whether or not any page
/// cites it**.
///
/// Against the manifest, because the manifest is what reserved it. A check
/// driven by citations would let a book reserve an anchor nothing guarantees,
/// and a guide edit that removed an anchor would fail later and elsewhere
/// instead of failing the books that reserved it — which is the property the
/// guide-first ordering was for.
fn check_declared_anchors(snapshot: &BookSnapshot, diagnostics: &mut Vec<Diagnostic>) {
    let manifest = &snapshot.manifest;
    for document in manifest.outbound_documents() {
        let present = snapshot
            .outbound_files
            .get(document.path())
            .and_then(|bytes| std::str::from_utf8(bytes).ok())
            .map(explicit_anchors)
            .unwrap_or_default();
        for anchor in document.anchors() {
            if !present.contains(anchor.as_str()) {
                diagnostics.push(markdown_diagnostic(
                    "M201",
                    format!(
                        "the manifest reserves anchor `{anchor}` from `{}`, which carries no such explicit anchor",
                        document.path()
                    ),
                    command_location(&manifest.manifest_path()),
                ));
            }
        }
    }
}

/// The explicit anchors a document outside the book carries: an
/// `<a id="…"></a>` line, outside any fenced code block, immediately preceding
/// a heading.
///
/// Requiring the explicit form is the whole mechanism. A renderer-generated
/// heading slug changes silently when the heading is retitled; an explicit
/// anchor does not, and an anchor line that precedes no heading is a label for
/// nothing.
///
/// This is a scan rather than a book parse: an outbound document is not a book
/// page, and running the book lexer over it would report the book's own
/// directive and encoding rules against a document that never agreed to them.
fn explicit_anchors(text: &str) -> BTreeSet<&str> {
    let lines: Vec<&str> = text.lines().collect();
    let mut anchors = BTreeSet::new();
    let mut fence: Option<usize> = None;
    for (index, line) in lines.iter().enumerate() {
        let backticks = line.bytes().take_while(|byte| *byte == b'`').count();
        match fence {
            Some(opened) => {
                if backticks >= opened && line.trim_end().len() == backticks {
                    fence = None;
                }
                continue;
            }
            None => {
                if backticks >= 3 {
                    fence = Some(backticks);
                    continue;
                }
            }
        }
        if let Some(anchor) = explicit_anchor(line) {
            if lines
                .get(index + 1)
                .and_then(|next| heading_level(next))
                .is_some()
            {
                anchors.insert(anchor);
            }
        }
    }
    anchors
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
        // `find` above scans raw bytes, so it reaches into fenced code and code
        // spans the parser already marked opaque. Only the cursor was checked
        // against those ranges; a separator inside one must not pair either, or
        // a stray bracket in the prose above a fence captures a link inside it.
        if opaque_ranges.iter().any(|range| range.contains(&separator)) {
            index = label_start;
            continue;
        }
        // A label may be hard-wrapped — that is this repository's house style,
        // and excluding every wrapped label made the common case the invisible
        // one. What the guard is actually for is the pairing hazard: a `[` left
        // open in one paragraph finding a `](` in the next. A blank line is
        // exactly where that stops being possible, because a link's label is
        // inline content and CommonMark 0.31.2 §4.8 has it that "paragraphs can
        // contain multiple lines, but no blank lines"
        // (https://spec.commonmark.org/0.31.2/#paragraphs). Confirmed against a
        // live CommonMark implementation rather than inferred: cmark-gfm, via
        // GitHub's /markdown API, renders `[label\nwrapped](url)` as one anchor
        // and leaves `[ bracket\n\nlater](url)` as two paragraphs of plain text.
        if crosses_blank_line(&markdown[index..separator]) {
            index = label_start;
            continue;
        }
        // Brackets pair innermost-first. Scanning from the *outermost* `[` to
        // the *first* `](` was containable while a label could not cross a line;
        // once it can, one unmatched `[` in prose — `[[Glossary term]]`, a bare
        // `[1]`, a bracket opening a block quote — swallows every line up to the
        // next real link and reports that link's destination against the stray
        // bracket's line. Restarting at the innermost bracket is what CommonMark
        // does and what keeps the genuine link visible at its own location.
        if let Some(inner) = last_unescaped_bracket(markdown, label_start, separator) {
            index = inner;
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

/// Whether a span from an opening bracket to its `](` crosses a paragraph
/// break. Only the interior lines can be one: the first element is the tail of
/// the line the bracket opened on and the last runs up to the separator, so
/// neither is a whole line and a label closing at column 1 is not a break.
fn crosses_blank_line(span: &str) -> bool {
    let mut lines = span.split('\n').peekable();
    lines.next();
    while let Some(line) = lines.next() {
        if lines.peek().is_some() && blank_line(line) {
            return true;
        }
    }
    false
}

/// CommonMark's blank line: "a line containing no characters, or a line
/// containing only spaces (U+0020) or tabs (U+0009)"
/// (https://spec.commonmark.org/0.31.2/#blank-line). Deliberately **not**
/// `str::trim`, which strips every Unicode `White_Space` character — a line
/// holding only a non-breaking space is not blank, and treating it as one drops
/// a real link without a word. The carriage return is allowed because the split
/// above is on `\n` alone, so a CRLF blank line arrives here as a lone `\r`.
fn blank_line(line: &str) -> bool {
    line.bytes()
        .all(|byte| matches!(byte, b' ' | b'\t' | b'\r'))
}

/// The last unescaped `[` strictly inside a candidate label, which is the
/// bracket CommonMark would have paired with the separator.
fn last_unescaped_bracket(markdown: &str, label_start: usize, separator: usize) -> Option<usize> {
    markdown[label_start..separator]
        .bytes()
        .enumerate()
        .rev()
        .map(|(offset, byte)| (label_start + offset, byte))
        .find(|(index, byte)| *byte == b'[' && !is_escaped(markdown.as_bytes(), *index))
        .map(|(index, _)| index)
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
    // An empty path is a bare `#anchor`: a link into the page that carries it.
    // Popping the filename and returning the *directory* — which is in none of
    // the file maps — failed any such link the scanner could see, as `resolves
    // to missing repository file`. Which was none of the ones in the books,
    // because a wrapped label hid them all; the two defects concealed each other.
    if path.is_empty() {
        return Some((source.to_owned(), anchor));
    }
    let mut components: Vec<&str> = source.split('/').collect();
    components.pop();
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

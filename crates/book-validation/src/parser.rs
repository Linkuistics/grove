use std::collections::BTreeMap;
use std::ops::Range;

use crate::{BookSnapshot, Diagnostic, Location};

#[derive(Clone, Debug)]
pub(crate) struct ParsedBook {
    pub(crate) roots: BTreeMap<String, Vec<Root>>,
    pub(crate) fragments: BTreeMap<String, Vec<Fragment>>,
    pub(crate) documents: BTreeMap<String, ParsedDocument>,
    pub(crate) diagnostics: Vec<Diagnostic>,
}

#[derive(Clone, Debug)]
pub(crate) struct ParsedDocument {
    pub(crate) path: String,
    pub(crate) text: String,
    pub(crate) page_directives: Vec<PageDirective>,
    pub(crate) opaque_ranges: Vec<Range<usize>>,
    pub(crate) ordinary_fences: Vec<OrdinaryFence>,
}

#[derive(Clone, Debug)]
pub(crate) struct PageDirective {
    pub(crate) raw: String,
    pub(crate) location: Location,
}

#[derive(Clone, Debug)]
pub(crate) struct OrdinaryFence {
    pub(crate) info: String,
    pub(crate) location: Location,
}

#[derive(Clone, Debug)]
pub(crate) struct Root {
    pub(crate) id: String,
    pub(crate) source: String,
    pub(crate) range: LineRange,
    pub(crate) children: Vec<Child>,
    pub(crate) location: Location,
}

#[derive(Clone, Debug)]
pub(crate) struct Fragment {
    pub(crate) id: String,
    pub(crate) owner: String,
    pub(crate) source: String,
    pub(crate) range: LineRange,
    pub(crate) parent: String,
    pub(crate) body: FragmentBody,
    pub(crate) location: Location,
}

#[derive(Clone, Debug)]
pub(crate) enum FragmentBody {
    Literal(Vec<u8>),
    Composite(Vec<Child>),
}

#[derive(Clone, Debug)]
pub(crate) enum Child {
    Insert {
        id: String,
        location: Location,
    },
    Defer {
        id: String,
        owner: String,
        range: LineRange,
        location: Location,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct LineRange {
    pub(crate) first: usize,
    pub(crate) last: usize,
}

enum Active {
    Root(Root),
    Composite(Fragment),
}

pub(crate) fn parse(snapshot: &BookSnapshot) -> ParsedBook {
    let mut parsed = ParsedBook {
        roots: BTreeMap::new(),
        fragments: BTreeMap::new(),
        documents: BTreeMap::new(),
        diagnostics: Vec::new(),
    };
    for (path, bytes) in &snapshot.book_files {
        parse_file(path, bytes, &mut parsed);
    }
    parsed
}

fn parse_file(path: &str, bytes: &[u8], parsed: &mut ParsedBook) {
    let recovered;
    let text = match std::str::from_utf8(bytes) {
        Ok(text) => text,
        Err(error) => {
            parsed.diagnostics.push(Diagnostic::new(
                "P003",
                "parse",
                "book file is not valid UTF-8",
                byte_location(path, bytes, error.valid_up_to()),
                None,
                None,
            ));
            recovered = recover_after_invalid_lines(bytes);
            &recovered
        }
    };
    if let Some(byte) = bytes.iter().position(|byte| *byte == b'\r') {
        parsed.diagnostics.push(Diagnostic::new(
            "P003",
            "parse",
            "book file contains a CR byte",
            location(path, text, byte),
            None,
            None,
        ));
    }
    if !bytes.ends_with(b"\n") {
        parsed.diagnostics.push(Diagnostic::new(
            "P003",
            "parse",
            "book file is missing its final LF",
            location(path, text, bytes.len()),
            None,
            None,
        ));
    }

    let lines: Vec<&str> = text.split_inclusive('\n').collect();
    let mut offsets = Vec::with_capacity(lines.len());
    let mut offset = 0;
    for line in &lines {
        offsets.push(offset);
        offset += line.len();
    }

    let mut document = ParsedDocument {
        path: path.into(),
        text: text.to_owned(),
        page_directives: Vec::new(),
        opaque_ranges: Vec::new(),
        ordinary_fences: Vec::new(),
    };
    let mut active: Option<Active> = None;
    let mut ordinary_fence: Option<(String, String, Location, usize)> = None;
    let mut index = 0;
    while index < lines.len() {
        let raw = lines[index];
        let line = raw.strip_suffix('\n').unwrap_or(raw);
        let here = Location {
            path: path.into(),
            byte: offsets[index],
            line: index + 1,
            column: 1,
        };

        if let Some((delimiter, _, _, _)) = &ordinary_fence {
            if line == delimiter {
                let (_, info, location, start) = ordinary_fence.take().unwrap();
                document
                    .opaque_ranges
                    .push(start..offsets[index] + raw.len());
                document
                    .ordinary_fences
                    .push(OrdinaryFence { info, location });
            }
            index += 1;
            continue;
        }
        if active.is_none() {
            match fence_start(line) {
                Some(FenceStart::Ordinary { delimiter, info }) => {
                    ordinary_fence = Some((delimiter, info, here, offsets[index]));
                    index += 1;
                    continue;
                }
                Some(FenceStart::ReservedFour) => {
                    invalid_context(
                        parsed,
                        here,
                        "four-backtick fence is reserved for literal fragments",
                    );
                    index += 1;
                    continue;
                }
                Some(FenceStart::Invalid) => {
                    invalid_context(parsed, here, "ordinary fence opener is malformed");
                    index += 1;
                    continue;
                }
                None => {}
            }
        }
        if valid_book_page(line) {
            if active.is_some() {
                invalid_context(parsed, here, "book-page directive is inside a construct");
            } else {
                document.page_directives.push(PageDirective {
                    raw: line.into(),
                    location: here,
                });
            }
            index += 1;
            continue;
        }

        if let Some((id, source, range)) = parse_root(line) {
            if active.is_none() {
                active = Some(Active::Root(Root {
                    id,
                    source,
                    range,
                    children: Vec::new(),
                    location: here,
                }));
            } else {
                invalid_context(parsed, here, "source root cannot nest");
            }
            index += 1;
            continue;
        }
        if let Some((id, owner, source, range, parent)) = parse_fragment(line) {
            if active.is_some() {
                invalid_context(parsed, here, "fragment cannot nest");
                index += 1;
                continue;
            }
            let mut fragment = Fragment {
                id,
                owner,
                source,
                range,
                parent,
                body: FragmentBody::Composite(Vec::new()),
                location: here.clone(),
            };
            if lines
                .get(index + 1)
                .is_some_and(|line| *line == "````rust\n" || *line == "````toml\n")
            {
                let expected = if fragment.source.ends_with(".toml") {
                    "````toml\n"
                } else {
                    "````rust\n"
                };
                if lines[index + 1] != expected {
                    invalid_context(
                        parsed,
                        here.clone(),
                        "literal fence language does not match its source path",
                    );
                }
                let body_start = index + 2;
                let mut close = body_start;
                while close < lines.len() && lines[close] != "````\n" {
                    close += 1;
                }
                if close == lines.len()
                    || lines.get(close + 1).copied() != Some("<!-- /fragment -->\n")
                {
                    invalid_context(parsed, here, "literal fragment is not closed exactly");
                    index = if close < lines.len() {
                        close + 1
                    } else {
                        index + 1
                    };
                    continue;
                }
                fragment.body =
                    FragmentBody::Literal(lines[body_start..close].concat().into_bytes());
                if body_start < close {
                    document
                        .opaque_ranges
                        .push(offsets[body_start]..offsets[close]);
                }
                parsed
                    .fragments
                    .entry(fragment.id.clone())
                    .or_default()
                    .push(fragment);
                index = close + 2;
                continue;
            }
            active = Some(Active::Composite(fragment));
            index += 1;
            continue;
        }
        if let Some(id) = parse_insert(line) {
            let child = Child::Insert { id, location: here };
            push_child(parsed, active.as_mut(), child);
            index += 1;
            continue;
        }
        if let Some((id, owner, range)) = parse_defer(line) {
            let child = Child::Defer {
                id,
                owner,
                range,
                location: here,
            };
            push_child(parsed, active.as_mut(), child);
            index += 1;
            continue;
        }
        if line == "<!-- /source-root -->" {
            match active.take() {
                Some(Active::Root(root)) => {
                    parsed.roots.entry(root.id.clone()).or_default().push(root)
                }
                other => {
                    active = other;
                    invalid_context(parsed, here, "source-root close does not match context");
                }
            }
            index += 1;
            continue;
        }
        if line == "<!-- /fragment -->" {
            match active.take() {
                Some(Active::Composite(fragment)) => parsed
                    .fragments
                    .entry(fragment.id.clone())
                    .or_default()
                    .push(fragment),
                other => {
                    active = other;
                    invalid_context(parsed, here, "fragment close does not match context");
                }
            }
            index += 1;
            continue;
        }
        if reserved_prefix(line) {
            parsed.diagnostics.push(Diagnostic::new(
                "P001",
                "parse",
                "malformed reserved directive",
                here,
                None,
                None,
            ));
        } else if active.is_some() {
            invalid_context(parsed, here, "non-directive content inside composite");
        }
        index += 1;
    }

    if let Some((_, info, location, start)) = ordinary_fence {
        document.opaque_ranges.push(start..text.len());
        document.ordinary_fences.push(OrdinaryFence {
            info,
            location: location.clone(),
        });
        invalid_context(
            parsed,
            location,
            "ordinary fence is unclosed at end of file",
        );
    }

    if let Some(active) = active {
        let (location, id) = match active {
            Active::Root(root) => (root.location, root.id),
            Active::Composite(fragment) => (fragment.location, fragment.id),
        };
        parsed.diagnostics.push(Diagnostic::new(
            "P002",
            "parse",
            "construct is unclosed at end of file",
            location,
            Some(&id),
            None,
        ));
    }
    document.opaque_ranges.sort_by_key(|range| range.start);
    parsed.documents.insert(path.into(), document);
}

fn push_child(parsed: &mut ParsedBook, active: Option<&mut Active>, child: Child) {
    match active {
        Some(Active::Root(root)) => root.children.push(child),
        Some(Active::Composite(fragment)) => {
            if let FragmentBody::Composite(children) = &mut fragment.body {
                children.push(child);
            }
        }
        None => invalid_context(
            parsed,
            child.location().clone(),
            "child directive is outside a root or fragment",
        ),
    }
}

impl Child {
    pub(crate) fn location(&self) -> &Location {
        match self {
            Self::Insert { location, .. } | Self::Defer { location, .. } => location,
        }
    }
}

fn parse_root(line: &str) -> Option<(String, String, LineRange)> {
    let body = line
        .strip_prefix("<!-- source-root «")?
        .strip_suffix(" -->")?;
    let (id, tail) = body.split_once("» source=\"")?;
    if !valid_id(id) {
        return None;
    }
    let (source, range) = tail.split_once("\" lines=\"")?;
    Some((
        id.into(),
        source.into(),
        parse_range(range.strip_suffix('"')?)?,
    ))
}

fn parse_fragment(line: &str) -> Option<(String, String, String, LineRange, String)> {
    let body = line.strip_prefix("<!-- fragment «")?.strip_suffix(" -->")?;
    let (id, tail) = body.split_once("» owner=\"")?;
    let (owner, tail) = tail.split_once("\" source=\"")?;
    let (source, tail) = tail.split_once("\" lines=\"")?;
    let (range, parent) = tail.split_once("\" parent=\"")?;
    if !valid_id(id) || !valid_id(parent.strip_suffix('"')?) || !valid_slice(owner) {
        return None;
    }
    Some((
        id.into(),
        owner.into(),
        source.into(),
        parse_range(range)?,
        parent.strip_suffix('"')?.into(),
    ))
}

fn parse_insert(line: &str) -> Option<String> {
    let id = line.strip_prefix("<!-- insert «")?.strip_suffix("» -->")?;
    valid_id(id).then(|| id.into())
}

fn parse_defer(line: &str) -> Option<(String, String, LineRange)> {
    let body = line.strip_prefix("<!-- defer «")?.strip_suffix(" -->")?;
    let (id, tail) = body.split_once("» owner=\"")?;
    let (owner, range) = tail.split_once("\" lines=\"")?;
    if !valid_id(id) || !valid_slice(owner) {
        return None;
    }
    Some((
        id.into(),
        owner.into(),
        parse_range(range.strip_suffix('"')?)?,
    ))
}

fn parse_range(text: &str) -> Option<LineRange> {
    let (first, last) = text.split_once('-')?;
    let first_text = first;
    let last_text = last;
    let first: usize = first.parse().ok()?;
    let last: usize = last.parse().ok()?;
    if first.to_string() != first_text || last.to_string() != last_text {
        return None;
    }
    (first > 0 && first <= last).then_some(LineRange { first, last })
}

enum FenceStart {
    Ordinary { delimiter: String, info: String },
    ReservedFour,
    Invalid,
}

fn fence_start(line: &str) -> Option<FenceStart> {
    let marker = line.as_bytes().first().copied()?;
    if !matches!(marker, b'`' | b'~') {
        return None;
    }
    let count = line.bytes().take_while(|byte| *byte == marker).count();
    if count < 3 {
        return None;
    }
    if count == 4 && marker == b'`' {
        return Some(FenceStart::ReservedFour);
    }
    if count != 3 && count < 5 {
        return Some(FenceStart::Invalid);
    }
    let info = &line[count..];
    if !info.is_empty()
        && (!info
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
            || !info.as_bytes()[0].is_ascii_alphanumeric())
    {
        return Some(FenceStart::Invalid);
    }
    Some(FenceStart::Ordinary {
        delimiter: (marker as char).to_string().repeat(count),
        info: info.into(),
    })
}

fn valid_id(id: &str) -> bool {
    let Some(first) = id.as_bytes().first() else {
        return false;
    };
    first.is_ascii_lowercase()
        && !id.contains("--")
        && !id.ends_with('-')
        && id
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn valid_book_page(line: &str) -> bool {
    [
        "<!-- book-page id=\"contents\" role=\"contents\" -->",
        "<!-- book-page id=\"concept-index\" role=\"lookup\" -->",
        "<!-- book-page id=\"source-index\" role=\"lookup\" -->",
        "<!-- book-page id=\"orientation\" slice=\"orientation-k11\" order=\"1\" -->",
        "<!-- book-page id=\"name-seam\" slice=\"name-seam-k12\" order=\"2\" -->",
        "<!-- book-page id=\"reference-domain\" slice=\"reference-domain-k13\" order=\"3\" -->",
        "<!-- book-page id=\"read-path\" slice=\"read-path-k14\" order=\"4\" -->",
        "<!-- book-page id=\"mutation-algebra\" slice=\"mutation-algebra-k15\" order=\"5\" -->",
        "<!-- book-page id=\"filesystem-interpreter\" slice=\"filesystem-interpreter-k16\" order=\"6\" -->",
        "<!-- book-page id=\"syllabus-cli\" slice=\"syllabus-cli-k17\" order=\"7\" -->",
        "<!-- book-page id=\"invariants-and-trade-offs\" slice=\"book-assembly-k18\" order=\"8\" -->",
    ]
    .contains(&line)
}

fn valid_slice(slice: &str) -> bool {
    [
        "orientation-k11",
        "name-seam-k12",
        "reference-domain-k13",
        "read-path-k14",
        "mutation-algebra-k15",
        "filesystem-interpreter-k16",
        "syllabus-cli-k17",
        "book-assembly-k18",
    ]
    .contains(&slice)
}

fn reserved_prefix(line: &str) -> bool {
    [
        "<!-- book-page",
        "<!-- source-root",
        "<!-- fragment",
        "<!-- insert",
        "<!-- defer",
        "<!-- /source-root",
        "<!-- /fragment",
    ]
    .iter()
    .any(|prefix| line.starts_with(prefix))
}

fn invalid_context(parsed: &mut ParsedBook, location: Location, message: &str) {
    parsed.diagnostics.push(Diagnostic::new(
        "P002", "parse", message, location, None, None,
    ));
}

fn location(path: &str, text: &str, byte: usize) -> Location {
    let prefix = &text[..byte.min(text.len())];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let column = prefix
        .rfind('\n')
        .map_or(prefix.len() + 1, |newline| prefix.len() - newline);
    Location {
        path: path.into(),
        byte,
        line,
        column,
    }
}

fn byte_location(path: &str, bytes: &[u8], byte: usize) -> Location {
    let prefix = &bytes[..byte.min(bytes.len())];
    let line = prefix
        .iter()
        .filter(|candidate| **candidate == b'\n')
        .count()
        + 1;
    let column = prefix
        .iter()
        .rposition(|candidate| *candidate == b'\n')
        .map_or(prefix.len() + 1, |newline| prefix.len() - newline);
    Location {
        path: path.into(),
        byte,
        line,
        column,
    }
}

fn recover_after_invalid_lines(bytes: &[u8]) -> String {
    let mut recovered = Vec::with_capacity(bytes.len());
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        if std::str::from_utf8(line).is_ok() {
            recovered.extend_from_slice(line);
        } else {
            recovered.extend(
                line.iter()
                    .map(|byte| if *byte == b'\n' { b'\n' } else { b' ' }),
            );
        }
    }
    String::from_utf8(recovered).expect("invalid lines were replaced with ASCII spaces")
}

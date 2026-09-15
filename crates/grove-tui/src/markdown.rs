//! Inert Markdown layout. Source offsets stay attached through terminal reflow.
use std::{collections::HashMap, ops::Range};

use pulldown_cmark::{Alignment, Event, Options, Parser, Tag, TagEnd};
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

use crate::observation::safe_text;

/// Byte position plus a rendered byte offset inside a transformed parser event.
/// The second coordinate distinguishes wrapped rows of normalized inline code
/// and generated link destinations without inventing source byte positions.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Anchor {
    byte: usize,
    within: usize,
}

impl Anchor {
    /// Follow an unchanged source line, retaining its byte/renderer coordinates.
    /// Common prefixes/suffixes disambiguate unchanged runs. Inside an edited
    /// region, prefer matching neighbours, then proximity, then source order.
    /// This is linear in source size, including entirely repetitive documents;
    /// it deliberately does not construct a quadratic whole-document diff.
    pub fn remap(self, old: &str, new: &str) -> Self {
        if old == new {
            return self;
        }
        fn lines(source: &str) -> Vec<(usize, &str)> {
            let mut offset = 0;
            source
                .split_inclusive('\n')
                .map(move |line| {
                    let start = offset;
                    offset += line.len();
                    (start, line)
                })
                .collect::<Vec<_>>()
        }
        let old_lines = lines(old);
        let new_lines = lines(new);
        if old_lines.is_empty() || new_lines.is_empty() {
            return Self::default();
        }
        let target = old_lines
            .partition_point(|(start, _)| *start <= self.byte)
            .saturating_sub(1);
        let prefix = old_lines
            .iter()
            .zip(&new_lines)
            .take_while(|(a, b)| a.1 == b.1)
            .count();
        let suffix = old_lines[prefix..]
            .iter()
            .rev()
            .zip(new_lines[prefix..].iter().rev())
            .take_while(|(a, b)| a.1 == b.1)
            .count();
        let mut occurrences: HashMap<&str, Vec<usize>> = HashMap::new();
        // Prefix/suffix occurrences already belong to preserved old lines;
        // deleted duplicates must not reuse them or repeatedly scan them.
        for (index, (_, text)) in new_lines
            .iter()
            .enumerate()
            .take(new_lines.len() - suffix)
            .skip(prefix)
        {
            occurrences.entry(text).or_default().push(index);
        }
        let locate = |index: usize| -> Option<usize> {
            if index < prefix {
                return Some(index);
            }
            if index >= old_lines.len() - suffix {
                return Some(new_lines.len() - (old_lines.len() - index));
            }
            let candidates = occurrences.get(old_lines[index].1)?;
            let after: Vec<_> = old_lines[index..].iter().map(|line| line.1).collect();
            let before: Vec<_> = old_lines[..index].iter().rev().map(|line| line.1).collect();
            let forward: Vec<_> = new_lines.iter().map(|line| line.1).collect();
            let backward: Vec<_> = forward.iter().rev().copied().collect();
            let right = prefix_matches(&after, &forward);
            let left = prefix_matches(&before, &backward);
            candidates.iter().copied().min_by_key(|&candidate| {
                let context = right[candidate]
                    + if candidate == 0 {
                        0
                    } else {
                        left[new_lines.len() - candidate]
                    };
                (
                    std::cmp::Reverse(context),
                    candidate.abs_diff(index),
                    candidate,
                )
            })
        };
        // The following line wins equidistant fallback ties. Preserve an
        // intra-line offset only when the original line itself survived.
        for distance in 0..old_lines.len() {
            for index in [
                target
                    .checked_add(distance)
                    .filter(|i| *i < old_lines.len()),
                target.checked_sub(distance),
            ]
            .into_iter()
            .flatten()
            {
                if let Some(mapped) = locate(index) {
                    return Self {
                        byte: new_lines[mapped].0
                            + if index == target {
                                self.byte
                                    .saturating_sub(old_lines[target].0)
                                    .min(new_lines[mapped].1.len())
                            } else {
                                0
                            },
                        within: if index == target { self.within } else { 0 },
                    };
                }
            }
        }
        // No unchanged line exists. Keep the nearest source byte, on a UTF-8
        // boundary; layout's row lookup and viewport clamp finish the fallback.
        let mut byte = self.byte.min(new.len());
        while !new.is_char_boundary(byte) {
            byte -= 1;
        }
        Self { byte, within: 0 }
    }
}

/// Longest pattern prefix at every text position, using a Z window: comparisons
/// inside the known matching window are reused instead of rescanning each run.
fn prefix_matches(pattern: &[&str], text: &[&str]) -> Vec<usize> {
    let sequence: Vec<_> = pattern
        .iter()
        .copied()
        .map(Some)
        .chain(std::iter::once(None))
        .chain(text.iter().copied().map(Some))
        .collect();
    let mut matches = vec![0; sequence.len()];
    let (mut left, mut right) = (0, 0);
    for index in 1..sequence.len() {
        if index < right {
            matches[index] = matches[index - left].min(right - index);
        }
        while index + matches[index] < sequence.len()
            && sequence[matches[index]] == sequence[index + matches[index]]
        {
            matches[index] += 1;
        }
        if index + matches[index] > right {
            left = index;
            right = index + matches[index];
        }
    }
    matches[pattern.len() + 1..].to_vec()
}

pub(crate) struct SourceLine {
    pub line: Line<'static>,
    pub source: Range<usize>,
    within: usize,
    pub wide: bool,
}

#[derive(Default)]
pub(crate) struct Document {
    pub lines: Vec<SourceLine>,
}

impl Document {
    pub fn layout(source: &str, width: usize) -> Self {
        let mut renderer = Renderer::new(source, width.max(1));
        // OffsetIter returns ranges in the original input, including transformed
        // entities. Sanitize *after* parsing so decoded controls are inert too.
        // https://docs.rs/pulldown-cmark/0.13.0/pulldown_cmark/struct.Parser.html#method.into_offset_iter
        for (event, range) in Parser::new_ext(source, Options::ENABLE_TABLES).into_offset_iter() {
            renderer.event(event, range);
        }
        renderer.flush();
        while renderer
            .lines
            .last()
            .is_some_and(|line| line.line.width() == 0)
        {
            renderer.lines.pop();
        }
        Self {
            lines: renderer.lines,
        }
    }

    pub fn anchor(&self, row: usize) -> Anchor {
        self.lines
            .get(row)
            .map_or(Anchor::default(), |line| Anchor {
                byte: line.source.start,
                within: line.within,
            })
    }

    pub fn row_for(&self, anchor: Anchor) -> usize {
        // Equal anchors prefer the first row (e.g. a table header and its rule).
        // Otherwise pick the preceding start after reflow, not the first broad
        // event range containing the source byte.
        (0..self.lines.len())
            .find(|&row| self.anchor(row) == anchor)
            .or_else(|| (0..self.lines.len()).rfind(|&row| self.anchor(row) < anchor))
            .unwrap_or(0)
    }

    pub fn width(&self) -> usize {
        self.lines
            .iter()
            .filter(|line| line.wide)
            .map(|line| line.line.width())
            .max()
            .unwrap_or(0)
    }
}

#[derive(Clone)]
struct Glyph {
    text: String,
    style: Style,
    source: Range<usize>,
    within: usize,
    width: usize,
}

impl Glyph {
    fn space(&self) -> bool {
        self.text.chars().all(char::is_whitespace)
    }
}

struct Item {
    marker: String,
    first: bool,
}

#[derive(Default)]
struct Table {
    align: Vec<Alignment>,
    rows: Vec<Vec<Vec<Glyph>>>,
    row: Vec<Vec<Glyph>>,
}

struct Renderer<'a> {
    source: &'a str,
    width: usize,
    lines: Vec<SourceLine>,
    current: Vec<Glyph>,
    style: Style,
    styles: Vec<Style>,
    lists: Vec<Option<u64>>,
    links: Vec<String>,
    items: Vec<Item>,
    quotes: usize,
    code: bool,
    table: Option<Table>,
}

impl<'a> Renderer<'a> {
    fn new(source: &'a str, width: usize) -> Self {
        Self {
            source,
            width,
            lines: Vec::new(),
            current: Vec::new(),
            style: Style::default(),
            styles: Vec::new(),
            lists: Vec::new(),
            links: Vec::new(),
            items: Vec::new(),
            quotes: 0,
            code: false,
            table: None,
        }
    }

    fn text(&mut self, text: &str, range: Range<usize>) {
        let exact = self.source.get(range.clone()) == Some(text);
        let mut offset = 0;
        // Grapheme iteration and widths come from the same terminal text engine.
        // https://docs.rs/ratatui/0.29.0/ratatui/text/struct.Line.html#method.styled_graphemes
        for part in text.split_inclusive('\n') {
            let body = part.strip_suffix('\n').unwrap_or(part);
            let line = Line::raw(body);
            for grapheme in line.styled_graphemes(self.style) {
                let source = if exact {
                    range.start + offset..range.start + offset + grapheme.symbol.len()
                } else {
                    range.clone()
                };
                let within = if exact { 0 } else { offset };
                offset += grapheme.symbol.len();
                let clean = safe_text(grapheme.symbol);
                self.current.push(Glyph {
                    width: Line::raw(clean.as_str()).width(),
                    text: clean,
                    style: self.style,
                    source,
                    within,
                });
            }
            if part.ends_with('\n') {
                let newline = if exact {
                    range.start + offset..range.start + offset + 1
                } else {
                    range.clone()
                };
                offset += 1;
                self.flush_line(newline);
            }
        }
    }

    fn prefix(&self) -> String {
        let mut prefix = "│ ".repeat(self.quotes);
        for item in &self.items {
            if item.first {
                prefix.push_str(&item.marker);
            } else {
                prefix.push_str(&" ".repeat(Line::raw(item.marker.as_str()).width()));
            }
        }
        prefix
    }

    fn emit(&mut self, glyphs: &[Glyph], fallback: Range<usize>, wide: bool) {
        let source = glyphs.first().map_or(fallback.clone(), |g| {
            g.source.start..glyphs.last().map_or(g.source.end, |g| g.source.end)
        });
        let mut spans = vec![Span::raw(self.prefix())];
        for glyph in glyphs {
            // Coalesce adjacent styles; the resulting layout is compact even
            // though wrapping and source mapping work at grapheme boundaries.
            if let Some(span) = spans.last_mut().filter(|span| span.style == glyph.style) {
                span.content.to_mut().push_str(&glyph.text);
            } else {
                spans.push(Span::styled(glyph.text.clone(), glyph.style));
            }
        }
        self.lines.push(SourceLine {
            line: Line::from(spans),
            source,
            within: glyphs.first().map_or(0, |g| g.within),
            wide,
        });
        for item in &mut self.items {
            item.first = false;
        }
    }

    fn flush_line(&mut self, fallback: Range<usize>) {
        let glyphs = std::mem::take(&mut self.current);
        if self.code || glyphs.is_empty() {
            self.emit(&glyphs, fallback, self.code);
            return;
        }
        let mut start = 0;
        while start < glyphs.len() {
            let available = self
                .width
                .saturating_sub(Line::raw(self.prefix()).width())
                .max(1);
            let mut end = start;
            let mut columns = 0;
            let mut boundary = None;
            while end < glyphs.len() && (columns + glyphs[end].width <= available || end == start) {
                columns += glyphs[end].width;
                if glyphs[end].space() {
                    boundary = Some(end);
                }
                end += 1;
            }
            if end < glyphs.len() && !glyphs[end].space() {
                if let Some(space) = boundary.filter(|&space| space > start) {
                    end = space;
                }
            }
            self.emit(&glyphs[start..end], fallback.clone(), false);
            start = end;
            while start < glyphs.len() && glyphs[start].space() {
                start += 1;
            }
        }
    }

    fn flush(&mut self) {
        if !self.current.is_empty() {
            let range = self.current[0].source.clone();
            self.flush_line(range);
        }
    }

    fn gap(&mut self, range: Range<usize>) {
        self.flush();
        if self.lines.last().is_some_and(|line| line.line.width() > 0) {
            self.lines.push(SourceLine {
                line: Line::default(),
                source: range.end..range.end,
                within: 0,
                wide: false,
            });
        }
    }

    fn event(&mut self, event: Event<'_>, range: Range<usize>) {
        match event {
            Event::Start(tag) => {
                self.styles.push(self.style);
                match tag {
                    Tag::Paragraph => self.flush(),
                    Tag::Heading { level, .. } => {
                        self.flush();
                        self.style = self.style.fg(Color::Cyan).add_modifier(Modifier::BOLD);
                        self.text(
                            &format!("{} ", "#".repeat(level as usize)),
                            range.start..range.start,
                        );
                    }
                    Tag::Emphasis => self.style = self.style.add_modifier(Modifier::ITALIC),
                    Tag::Strong => self.style = self.style.add_modifier(Modifier::BOLD),
                    Tag::Link { dest_url, .. } => {
                        self.links.push(dest_url.into_string());
                        self.style = self.style.add_modifier(Modifier::UNDERLINED);
                    }
                    Tag::BlockQuote(_) => {
                        self.flush();
                        self.quotes += 1;
                    }
                    Tag::List(start) => {
                        self.flush();
                        self.lists.push(start);
                    }
                    Tag::Item => {
                        self.flush();
                        let marker = match self.lists.last_mut() {
                            Some(Some(n)) => {
                                let marker = format!("{n}. ");
                                *n = n.saturating_add(1);
                                marker
                            }
                            _ => "• ".into(),
                        };
                        self.items.push(Item {
                            marker,
                            first: true,
                        });
                    }
                    Tag::CodeBlock(_) => {
                        self.flush();
                        self.code = true;
                        self.style = self.style.fg(Color::Yellow);
                    }
                    Tag::HtmlBlock => self.flush(),
                    Tag::Table(align) => {
                        self.flush();
                        self.table = Some(Table {
                            align,
                            ..Table::default()
                        });
                    }
                    Tag::TableHead => self.style = self.style.add_modifier(Modifier::BOLD),
                    _ => {}
                }
            }
            Event::End(tag) => {
                match tag {
                    TagEnd::Paragraph | TagEnd::Heading(_) | TagEnd::HtmlBlock => self.gap(range),
                    TagEnd::CodeBlock => {
                        self.flush();
                        self.code = false;
                        self.gap(range);
                    }
                    TagEnd::BlockQuote(_) => {
                        self.flush();
                        self.quotes = self.quotes.saturating_sub(1);
                        self.gap(range);
                    }
                    TagEnd::Item => {
                        self.flush();
                        self.items.pop();
                    }
                    TagEnd::List(_) => {
                        self.flush();
                        self.lists.pop();
                        if self.lists.is_empty() {
                            self.gap(range);
                        }
                    }
                    TagEnd::Link => {
                        if let Some(destination) = self.links.pop() {
                            self.text(&format!(" ({destination})"), range.end..range.end);
                        }
                    }
                    TagEnd::TableCell => {
                        if let Some(table) = &mut self.table {
                            table.row.push(std::mem::take(&mut self.current));
                        }
                    }
                    TagEnd::TableHead | TagEnd::TableRow => {
                        if let Some(table) = &mut self.table {
                            table.rows.push(std::mem::take(&mut table.row));
                        }
                    }
                    TagEnd::Table => {
                        self.finish_table(range.clone());
                        self.gap(range);
                    }
                    _ => {}
                }
                self.style = self.styles.pop().unwrap_or_default();
            }
            Event::Text(text) | Event::Html(text) | Event::InlineHtml(text) => {
                self.text(&text, range)
            }
            Event::Code(text) => {
                let style = self.style;
                self.style = self.style.fg(Color::Yellow);
                self.text(&text, range);
                self.style = style;
            }
            Event::SoftBreak => self.text(" ", range),
            Event::HardBreak => self.flush_line(range),
            Event::Rule => {
                self.flush();
                self.text(&"─".repeat(self.width), range.clone());
                self.flush();
                self.gap(range);
            }
            _ => {}
        }
    }

    fn finish_table(&mut self, range: Range<usize>) {
        let Some(table) = self.table.take() else {
            return;
        };
        let widths: Vec<usize> = (0..table.align.len())
            .map(|col| {
                table
                    .rows
                    .iter()
                    .filter_map(|row| row.get(col))
                    .map(|cell| cell.iter().map(|g| g.width).sum())
                    .max()
                    .unwrap_or(0)
                    .max(3)
            })
            .collect();
        for (index, row) in table.rows.into_iter().enumerate() {
            let row_range = row
                .iter()
                .flatten()
                .next()
                .map_or(range.clone(), |g| g.source.clone());
            let mut cells = Vec::new();
            let decoration = |text: String| Glyph {
                width: Line::raw(text.as_str()).width(),
                text,
                style: Style::default(),
                source: row_range.clone(),
                within: 0,
            };
            cells.push(decoration("│ ".into()));
            for (col, cell) in row.into_iter().enumerate() {
                let width: usize = cell.iter().map(|g| g.width).sum();
                let padding = widths
                    .get(col)
                    .copied()
                    .unwrap_or(width)
                    .saturating_sub(width);
                let left = match table.align.get(col) {
                    Some(Alignment::Right) => padding,
                    Some(Alignment::Center) => padding / 2,
                    _ => 0,
                };
                cells.push(decoration(" ".repeat(left)));
                cells.extend(cell);
                cells.push(decoration(format!("{} │ ", " ".repeat(padding - left))));
            }
            // Table decorations map to the row, not to a preceding cell.
            let end = cells
                .iter()
                .map(|g| g.source.end)
                .max()
                .unwrap_or(row_range.end);
            self.emit(&cells, row_range.clone(), true);
            if let Some(line) = self.lines.last_mut() {
                line.source = row_range.start..end;
            }
            if index == 0 {
                let separator = format!(
                    "├─{}─┤",
                    widths
                        .iter()
                        .map(|w| "─".repeat(*w))
                        .collect::<Vec<_>>()
                        .join("─┼─")
                );
                self.emit(&[decoration(separator)], row_range.clone(), true);
            }
        }
    }
}

/// Clip styled terminal columns, preserving graphemes and styles. Only wide
/// blocks follow the horizontal offset; reflowed prose stays at the left edge.
pub(crate) fn clip(line: &SourceLine, offset: usize, width: usize) -> Line<'static> {
    let offset = if line.wide { offset } else { 0 };
    let mut column: usize = 0;
    let mut spans = Vec::new();
    for grapheme in line.line.styled_graphemes(Style::default()) {
        let start = column;
        column += Line::raw(grapheme.symbol).width();
        if column <= offset {
            continue;
        }
        if start >= offset.saturating_add(width) {
            break;
        }
        let text = if start < offset || column > offset.saturating_add(width) {
            " ".repeat(column.min(offset.saturating_add(width)) - start.max(offset))
        } else {
            grapheme.symbol.to_owned()
        };
        spans.push(Span::styled(text, grapheme.style));
    }
    Line::from(spans)
}

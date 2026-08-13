//! The unit-marker reader for the embedded methodology — **one implementation,
//! compiled twice**.
//!
//! `build.rs` gates `content/` on this grammar before the embed is built, and
//! `methodology::units` reads the same grammar back out of the linked embed at
//! runtime. Those are two *traversals* — a filesystem walk and an `include_dir`
//! walk — but deliberately one *parser*: this file is `#[path]`-included into
//! the build script, so a rule learned here cannot be half-learned there. The
//! repository has paid for the alternative once already; the hash the build
//! script used to compute needed a standing equality test purely because two
//! readers of one tree had been written twice
//! (`docs/specs/mandate-delivered-methodology.md`, *A malformed embed fails the
//! build*).
//!
//! The grammar, in full:
//!
//! ```text
//! <!-- file: order=<position> -->
//!
//! <!-- unit: <id> kinds=<scope> class=triggering -->
//! <!-- unit: <id> kinds=<scope> class=triggering defers=<target> -->
//! <!-- unit: <id> class=procedural -->
//! <!-- unit: <id> class=procedural defers=<target> -->
//! ```
//!
//! Units **partition** a file's body past its file directive: every such byte
//! belongs to exactly one
//! unit, a unit runs from its marker line to the byte before the next marker or
//! to end of file, and there is no nesting, no gap and no close marker. *Body*
//! is everything after the optional leading `---`-delimited preamble, which is
//! the file's one unread region.
//!
//! The **file directive** is the body's first line and the one line no unit
//! covers, which is what keeps partition true of everything after it. It is the
//! same device and the same recogniser as a unit marker — an unindented whole
//! line at neutral fence state — so `content/` gains no metadata language for
//! ordering (`docs/specs/mandate-delivered-methodology.md`, *The file's mandate
//! order is a comment directive*). It carries the file's position in the
//! composed mandate, it is **required**, and it is the one thing here the build
//! gate needs that a *unit* does not: composition order must be total.
//!
//! Partition is the load-bearing choice rather than tidiness: it makes
//! unclassified prose impossible, and it converts this parser's own worst
//! failure — going blind to some marker shape — into a *visible* one, because
//! the preceding unit absorbs the text instead of a unit silently vanishing.

use crate::leaf::Kind;
use std::fmt;

/// The unindented prefix that makes a line a marker *candidate*. A line that
/// starts with it and then fails to parse is an error rather than prose: a typo
/// in a marker must not quietly become part of the unit above it.
const MARKER_PREFIX: &str = "<!-- unit:";
/// The unindented prefix of the **file** directive, recognised exactly as a
/// marker is and sharing its terminator. One namespace of comment directives,
/// two keywords: a line can be a unit marker or a file directive, never both.
const FILE_PREFIX: &str = "<!-- file:";
const MARKER_SUFFIX: &str = "-->";
/// The file directive's one attribute — the file's position in the composed
/// mandate. Sole for now, and spelled `name=value` anyway so that the directive
/// is the *shape* a marker is rather than a bare number needing a second reading
/// rule the day anything else is ordered per file.
const ORDER_ATTRIBUTE: &str = "order";
/// The delimiter of the leading opaque preamble — `content/SKILL.md`'s YAML
/// frontmatter today, and whatever else ever opens a file with one.
///
/// This exact spelling is **reserved** as a first line, because nothing else can
/// distinguish it from a Markdown thematic break. The reservation costs an author
/// nothing: a body must begin with a unit marker, so a leading thematic break is
/// already an error in every other spelling (`***`, `___`, `----`, `- - -`), and
/// the one spelling that would have been swallowed instead is this one.
const PREAMBLE_DELIMITER: &str = "---";

/// A unit's delivery class: does this unit ship in a mandate, or is it served on
/// demand by `grove-llm methodology`?
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Triggering,
    Procedural,
}

impl Class {
    /// The spelling in a marker and in the `grove-llm methodology` listing.
    pub fn label(self) -> &'static str {
        match self {
            Class::Triggering => "triggering",
            Class::Procedural => "procedural",
        }
    }

    fn from_label(value: &str) -> Option<Class> {
        match value {
            "triggering" => Some(Class::Triggering),
            "procedural" => Some(Class::Procedural),
            _ => None,
        }
    }
}

/// Which session kinds a triggering unit's scope admits.
///
/// `*` or an explicit list, and nothing else — no family shorthand and no
/// negation, because a shorthand would silently absorb a kind added later, which
/// is the failure `docs/adr/complete-session-configuration.md` designed against.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Scope {
    All,
    Kinds(Vec<Kind>),
}

impl Scope {
    /// Does a mandate for `kind` carry a unit with this scope?
    ///
    /// The composer's whole selection rule, and it is a method here rather than
    /// a `match` there because the two spellings of a scope are this type's
    /// business: `*` and an explicit list are the only ones the grammar admits,
    /// so a third would have to be added beside this function and could not be
    /// forgotten in it.
    pub fn admits(&self, kind: Kind) -> bool {
        match self {
            Scope::All => true,
            Scope::Kinds(kinds) => kinds.contains(&kind),
        }
    }

    /// The scope as the listing writes it: `*`, or the kind labels in the order
    /// the marker gave them.
    pub fn render(&self) -> String {
        match self {
            Scope::All => "*".to_string(),
            Scope::Kinds(kinds) => kinds
                .iter()
                .map(|kind| kind.label())
                .collect::<Vec<_>>()
                .join(" "),
        }
    }
}

/// One marked span of one embedded file.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Unit {
    /// Kebab-case, and unique across the whole embed — which one file's text
    /// cannot decide, so it is checked in [`super::whole_embed`] rather than
    /// here.
    pub id: String,
    pub class: Class,
    /// `Some` exactly when the unit is triggering — a procedural unit ships to
    /// no mandate, so a scope on one would be a lie rather than an ignored field.
    pub scope: Option<Scope>,
    /// The ids of the procedural units whose bodies complete this one. Empty
    /// means *complete as delivered*, which is how a session knows not to go
    /// looking for more.
    pub defers: Vec<String>,
    /// The unit's source file, `content/`-relative.
    pub file: String,
    /// The file's position in the composed mandate, from its file directive.
    ///
    /// Carried per unit rather than per file, exactly as [`Unit::file`] is, so
    /// that composition is a pure function of a unit *slice*: the composer sorts
    /// by `(file_order, offset)` and needs no second structure to consult.
    /// Duplicated across a file's units by construction — one directive sets it
    /// for all of them — and unique **across** the embed, which one file's text
    /// cannot decide, so it is checked in [`super::whole_embed`] beside id
    /// uniqueness.
    pub file_order: u32,
    /// 1-based line of the marker.
    pub line: usize,
    /// Byte offset of the marker within the file — the machine half of the
    /// coordinate pair, carried so a *whole-embed* failure can be reported at
    /// the same `file:line:offset` a per-file one is
    /// ([`super::whole_embed`]). Nothing can recover it from a [`Unit`]
    /// afterwards: a unit's source is a copy, so searching for it would find
    /// the first identical span rather than this one.
    pub offset: usize,
    /// The unit's source bytes, **marker line included** — so a slice carries
    /// its own id and its own deferral, and addressing costs no driver-authored
    /// prose.
    pub source: String,
}

/// What is wrong, without where it is wrong. Split from [`ParseError`] so a test
/// can name the class it provoked without matching on a rendered string.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Fault {
    /// A marker-shaped line this grammar cannot read. Carries the specific
    /// reason, which is the half a contributor acts on.
    UnparseableMarker(String),
    UnknownAttribute(String),
    AttributesOutOfOrder {
        attribute: String,
        after: String,
    },
    MissingClass,
    /// `kinds` is required on a triggering unit.
    MissingKinds,
    /// ...and forbidden on a procedural one.
    KindsOnProcedural,
    /// A `kinds=` member outside the closed nineteen.
    UnknownKind(String),
    NoUnitDeclared,
    BodyBeforeFirstMarker,
    /// A `<!-- file: … -->` line this grammar cannot read. Carries the specific
    /// reason, exactly as [`Fault::UnparseableMarker`] does — and for the same
    /// reason: a typo in a directive must not quietly become prose.
    UnparseableFileDirective(String),
    /// The file declares no composition position. Required rather than optional
    /// because the mandate's order must be **total**: one file without a
    /// position is one pair of files with no order between them, which is the
    /// single property the directive exists to supply.
    MissingFileOrder,
    /// A directive line that is not the body's first one — including a second
    /// directive, which is the same fault seen twice. A file carries exactly one
    /// position, and a directive anywhere else would sit *inside* a unit and
    /// ship into a mandate as prose.
    MisplacedFileOrder,
    /// Reported at the line the fence opened on: a fence opened and never closed
    /// absorbs every later marker into one giant unit while violating no other
    /// rule, so the file would otherwise parse clean and stop being classified.
    UnterminatedFence,
    /// The same hole in the second place a delimiter can run away with a file: a
    /// leading `---` with no close swallows the document as unread preamble.
    UnterminatedPreamble,
    /// The one shape in which an over-long leading block could hide *classified*
    /// bytes. The preamble is unread by design, so an author who spelled a
    /// thematic break `---` on line 1 loses whatever the block then swallows;
    /// making a swallowed marker an error is what keeps that loss out of the
    /// partition claim.
    MarkerInPreamble,
    /// A unit's source is spliced straight into a `grove-llm methodology` fetch,
    /// so the last unit of a file that does not end in a newline would run its
    /// prose into the *next* fetched unit's marker and cost that marker its line.
    MissingFinalNewline,
    /// The listing writes one tab-separated line per unit and the last field is
    /// the unit's file. That grammar needs no escaping rule only while no field
    /// can carry a delimiter — a property of the *data*, so it is enforced here
    /// rather than assumed of today's tree.
    UnlistablePath(char),
}

impl fmt::Display for Fault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fault::UnparseableMarker(why) => write!(f, "unreadable unit marker: {why}"),
            Fault::UnknownAttribute(name) => write!(
                f,
                "unknown marker attribute `{name}`; the marker takes `kinds`, `class` and `defers`"
            ),
            Fault::AttributesOutOfOrder { attribute, after } => write!(
                f,
                "marker attribute `{attribute}` appears after `{after}`; the fixed order is \
                 id, kinds, class, defers"
            ),
            Fault::MissingClass => write!(f, "the marker declares no `class`"),
            Fault::MissingKinds => write!(
                f,
                "`kinds` is required on a `class=triggering` unit — a triggering unit that \
                 named no scope would reach no mandate"
            ),
            Fault::KindsOnProcedural => write!(
                f,
                "`kinds` is forbidden on a `class=procedural` unit — a procedural unit ships \
                 to no mandate, so a scope there would be a lie"
            ),
            Fault::UnknownKind(label) => write!(
                f,
                "`kinds` member `{label}` is not one of the nineteen session kinds; \
                 write `*` for every kind, or a quoted space-separated list"
            ),
            Fault::NoUnitDeclared => write!(
                f,
                "the file declares no unit; every embedded markdown file is fully classified"
            ),
            Fault::BodyBeforeFirstMarker => write!(
                f,
                "body text precedes the first unit marker; units partition everything after \
                 the file directive, so the first marker follows it immediately"
            ),
            Fault::UnparseableFileDirective(why) => {
                write!(f, "unreadable file directive: {why}")
            }
            Fault::MissingFileOrder => write!(
                f,
                "the file declares no mandate position; write `<!-- file: order=<n> -->` as the \
                 body's first line, since the composition order must be total"
            ),
            Fault::MisplacedFileOrder => write!(
                f,
                "this file directive is not the body's first line; a file carries exactly one, \
                 written immediately before its first unit marker — anywhere else it sits inside \
                 a unit and ships into a mandate as prose"
            ),
            Fault::UnterminatedFence => write!(
                f,
                "this fenced block is never closed, so every later marker is swallowed by it"
            ),
            Fault::UnterminatedPreamble => write!(
                f,
                "this leading `---` block is never closed, so the whole file reads as unread \
                 preamble; a first line of `---` always opens one, so write a leading thematic \
                 break as `***`"
            ),
            Fault::MarkerInPreamble => write!(
                f,
                "this unit marker is inside the leading `---` block, which is unread — a first \
                 line of `---` always opens one, so write a leading thematic break as `***`"
            ),
            Fault::MissingFinalNewline => write!(
                f,
                "the file does not end in a newline; a unit's bytes are served verbatim, so the \
                 last unit here would run into the next unit fetched beside it"
            ),
            Fault::UnlistablePath(character) => write!(
                f,
                "this file's path holds U+{:04X}, which no listing row can carry; `grove-llm \
                 methodology` writes one tab-separated line per unit",
                *character as u32
            ),
        }
    }
}

/// A [`Fault`] located in a file: line for a human, byte offset for a machine
/// and for a fence or preamble whose damage is measured in bytes.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParseError {
    pub file: String,
    pub line: usize,
    pub offset: usize,
    pub fault: Fault,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}:{}: {}",
            self.file, self.line, self.offset, self.fault
        )
    }
}

impl std::error::Error for ParseError {}

/// Read one embedded markdown file into its units.
///
/// `file` is the `content/`-relative path, carried into every [`Unit`] and every
/// [`ParseError`] — the parser never *reads* the filesystem, so the caller owns
/// what the path is called. What the parser does own is whether that name can
/// survive the surfaces a unit is carried on, which is why an unlistable path is
/// rejected here: this is the one function both traversals share, so a rule
/// enforced anywhere else would hold on only one of them.
pub fn parse_units(file: &str, text: &str) -> Result<Vec<Unit>, ParseError> {
    let fault_at = |line: usize, offset: usize, fault: Fault| ParseError {
        file: file.to_string(),
        line,
        offset,
        fault,
    };

    if let Some(character) = file.chars().find(|character| character.is_control()) {
        return Err(fault_at(1, 0, Fault::UnlistablePath(character)));
    }
    // An empty file has no missing newline to report; it declares no unit, which
    // is the error the tail of this function reports.
    if !text.is_empty() && !text.ends_with('\n') {
        let last_line = text.split_inclusive('\n').count();
        return Err(fault_at(last_line, text.len(), Fault::MissingFinalNewline));
    }

    let (body_start, body_line) = preamble_end(text).map_err(|fault| match fault {
        PreambleFault::Unterminated => fault_at(1, 0, Fault::UnterminatedPreamble),
        PreambleFault::MarkerInside { line, offset } => {
            fault_at(line, offset, Fault::MarkerInPreamble)
        }
    })?;

    // The file directive is read off the body's first line and consumed, so
    // everything the unit loop then sees is a region units partition exactly. A
    // directive is impossible to be inside a fence here — no line precedes this
    // one to have opened it — which is why it needs no fence state.
    let first = text[body_start..]
        .split_inclusive('\n')
        .next()
        .unwrap_or("");
    let (order, units_start, units_line) = if first.trim_end().starts_with(FILE_PREFIX) {
        let order = parse_file_directive(first.trim_end())
            .map_err(|fault| fault_at(body_line, body_start, fault))?;
        (Some(order), body_start + first.len(), body_line + 1)
    } else {
        (None, body_start, body_line)
    };

    let mut units: Vec<Unit> = Vec::new();
    let mut open: Option<(Unit, usize)> = None;
    let mut fence: Option<Fence> = None;
    let mut offset = units_start;

    for (line_number, raw) in (units_line..).zip(text[units_start..].split_inclusive('\n')) {
        let line = raw.trim_end();
        if let Some(state) = &fence {
            if state.is_closed_by(line) {
                fence = None;
            }
        } else if let Some(state) = Fence::opened_by(line, line_number, offset) {
            fence = Some(state);
        } else if line.starts_with(FILE_PREFIX) {
            // Reported before the directive is even read: where it sits is
            // wrong whatever it says, and a second complaint about its
            // attributes would send a contributor to fix the wrong half.
            return Err(fault_at(line_number, offset, Fault::MisplacedFileOrder));
        } else if line.starts_with(MARKER_PREFIX) {
            match open.take() {
                Some((unit, start)) => units.push(unit.with_source(&text[start..offset])),
                None if offset > units_start => {
                    return Err(fault_at(
                        units_line,
                        units_start,
                        Fault::BodyBeforeFirstMarker,
                    ))
                }
                None => {}
            }
            let unit = parse_marker(file, line, line_number, offset)?;
            open = Some((unit, offset));
        }
        offset += raw.len();
    }

    // A runaway fence outranks the emptiness it may itself have caused: the
    // delimiter is the fact a contributor can act on.
    if let Some(state) = fence {
        return Err(fault_at(state.line, state.offset, Fault::UnterminatedFence));
    }
    if let Some((unit, start)) = open {
        units.push(unit.with_source(&text[start..]));
    }
    // Emptiness outranks a missing position for the same reason: a file with no
    // units is not yet a file whose place in the mandate means anything, and an
    // empty file would otherwise be reported as an ordering mistake.
    if units.is_empty() {
        return Err(fault_at(units_line, units_start, Fault::NoUnitDeclared));
    }
    let Some(order) = order else {
        return Err(fault_at(body_line, body_start, Fault::MissingFileOrder));
    };
    for unit in &mut units {
        unit.file_order = order;
    }
    Ok(units)
}

/// Read `<!-- file: order=<n> -->`, or say why it could not be read.
///
/// Deliberately strict about arity: exactly one attribute, spelled `order`,
/// whose value is a non-negative integer. An unknown attribute is refused rather
/// than ignored, on the marker grammar's own rule — a field the reader skips is
/// a field a contributor believes is doing something.
fn parse_file_directive(line: &str) -> Result<u32, Fault> {
    let unreadable = |why: String| Fault::UnparseableFileDirective(why);
    let stripped = line
        .strip_suffix(MARKER_SUFFIX)
        .ok_or_else(|| unreadable("the directive line does not end with `-->`".to_string()))?;
    let inner = stripped[FILE_PREFIX.len()..].trim();

    let tokens = tokenize(inner).map_err(unreadable)?;
    let [token] = tokens.as_slice() else {
        return Err(unreadable(format!(
            "expected exactly one attribute, `{ORDER_ATTRIBUTE}=<n>`, got {} in `{inner}`",
            tokens.len()
        )));
    };
    let Some((name, value)) = token.split_once('=') else {
        return Err(unreadable(format!(
            "expected `{ORDER_ATTRIBUTE}=<n>`, got `{token}`"
        )));
    };
    if name != ORDER_ATTRIBUTE {
        return Err(unreadable(format!(
            "unknown file-directive attribute `{name}`; the directive takes `{ORDER_ATTRIBUTE}`"
        )));
    }
    value.parse::<u32>().map_err(|_| {
        unreadable(format!(
            "`{ORDER_ATTRIBUTE}` must be a non-negative integer, got `{value}`"
        ))
    })
}

impl Unit {
    fn with_source(mut self, source: &str) -> Unit {
        self.source = source.to_string();
        self
    }
}

/// Why a leading `---` block could not be skipped.
enum PreambleFault {
    Unterminated,
    MarkerInside { line: usize, offset: usize },
}

/// Where the body begins: `(byte offset, 1-based line)`.
///
/// The block is **optional and unread**. Nothing in the embed depends on it, so
/// `content/SKILL.md`'s YAML keeps discovering the provisioned skill for as long
/// as anything provisions it, and may be deleted the day nothing does, with no
/// parser change either way. Making the rule about *a leading delimited block*
/// rather than about that one file is the point: a gate whose value is having no
/// exceptions cannot afford a filename-keyed one.
///
/// A first line of `---` is a valid Markdown thematic break as well as a
/// frontmatter opener, and nothing in the bytes tells the two apart. The
/// collision is resolved by **reserving** the spelling rather than by guessing:
/// a leading `---` always opens a block. That takes nothing from an author,
/// because a body must begin with a unit marker — so a leading thematic break is
/// `BodyBeforeFirstMarker` in every *other* spelling — and it leaves exactly one
/// residue, an author who writes `---` on line 1 and gets a longer unread region
/// than they meant. [`Fault::MarkerInPreamble`] takes the load-bearing half of
/// that residue out: whatever an over-long block swallows, it cannot be a
/// classified unit.
fn preamble_end(text: &str) -> Result<(usize, usize), PreambleFault> {
    let mut lines = text.split_inclusive('\n');
    // An empty file has no preamble to be unterminated; it declares no unit,
    // which is the error the caller reports.
    let Some(first) = lines.next() else {
        return Ok((0, 1));
    };
    if first.trim_end() != PREAMBLE_DELIMITER {
        return Ok((0, 1));
    }
    let mut offset = first.len();
    for (line_number, raw) in (2..).zip(lines) {
        let line = raw.trim_end();
        if line == PREAMBLE_DELIMITER {
            return Ok((offset + raw.len(), line_number + 1));
        }
        if line.starts_with(MARKER_PREFIX) {
            return Err(PreambleFault::MarkerInside {
                line: line_number,
                offset,
            });
        }
        offset += raw.len();
    }
    Err(PreambleFault::Unterminated)
}

/// An open fenced block. Markers are recognised only at neutral fence state, so
/// no unit can begin or end inside a fence and every unit's fenced blocks are
/// balanced within it.
///
/// The recogniser is [CommonMark 0.31.2 §4.5][spec] applied to the document's
/// lines, and the exactness is load-bearing in **both** directions. Accept a
/// fence CommonMark would not (an over-indented opener) and the parser swallows
/// later real markers into the preceding unit; accept a *close* CommonMark would
/// not (an over-indented closer) and it returns to neutral early and promotes an
/// example marker — text a reader sees as code — into a unit boundary. Only the
/// second direction is silent, and it is the one a loose `trim()` enables.
///
/// What is deliberately *not* modelled is container context: a fence nested in a
/// list item or a block quote is read at its own indentation, with no memory of
/// the container's. The residue is one-directional and visible — a fence indented
/// one to three columns inside a list keeps swallowing after CommonMark would
/// have closed it with the list item, which moves the pinned id set rather than
/// inventing a unit.
///
/// [spec]: https://spec.commonmark.org/0.31.2/#fenced-code-blocks
struct Fence {
    character: char,
    length: usize,
    line: usize,
    offset: usize,
}

impl Fence {
    /// Up to three columns of indentation, three or more of one fence character,
    /// then an info string — which may hold no backtick on a backtick fence,
    /// since otherwise every paragraph opening with an inline code span would
    /// open a block.
    fn opened_by(line: &str, line_number: usize, offset: usize) -> Option<Fence> {
        let rest = fence_indent(line)?;
        let character = rest.chars().next().filter(|c| *c == '`' || *c == '~')?;
        let length = rest.chars().take_while(|c| *c == character).count();
        if length < 3 || (character == '`' && rest[length..].contains('`')) {
            return None;
        }
        Some(Fence {
            character,
            length,
            line: line_number,
            offset,
        })
    }

    /// At least as many of the same character, indented no further than an
    /// opener, with nothing after the run but spaces and tabs — a close carries
    /// no info string.
    fn is_closed_by(&self, line: &str) -> bool {
        let Some(rest) = fence_indent(line) else {
            return false;
        };
        let run = rest.chars().take_while(|c| *c == self.character).count();
        run >= self.length
            && rest[run..]
                .chars()
                .all(|character| character == ' ' || character == '\t')
    }
}

/// The line past its indentation, or `None` when it is indented too far to be a
/// fence line at all.
///
/// CommonMark allows a fence up to three columns of indentation; a fourth makes
/// an indented code block instead. A tab advances to the next four-column stop,
/// so a tab anywhere in the indentation is already past the bound — which falls
/// out of counting spaces alone and leaving the tab in `rest`, where it is not a
/// fence character.
fn fence_indent(line: &str) -> Option<&str> {
    let indent = line.len() - line.trim_start_matches(' ').len();
    (indent <= 3).then(|| &line[indent..])
}

fn parse_marker(
    file: &str,
    line: &str,
    line_number: usize,
    offset: usize,
) -> Result<Unit, ParseError> {
    let fault = |fault: Fault| ParseError {
        file: file.to_string(),
        line: line_number,
        offset,
        fault,
    };
    let unreadable = |why: String| fault(Fault::UnparseableMarker(why));

    // The caller guarantees the prefix, and the prefix cannot overlap the
    // suffix, so the slice below is in bounds for every line that reaches here.
    let stripped = line
        .strip_suffix(MARKER_SUFFIX)
        .ok_or_else(|| unreadable("the marker line does not end with `-->`".to_string()))?;
    let inner = stripped[MARKER_PREFIX.len()..].trim();

    let mut tokens = tokenize(inner).map_err(&unreadable)?.into_iter();
    let id = tokens
        .next()
        .ok_or_else(|| unreadable("the marker declares no unit id".to_string()))?;
    if !is_kebab_case(&id) {
        return Err(unreadable(format!(
            "unit id `{id}` is not kebab-case (lowercase letters, digits and single dashes)"
        )));
    }

    let mut scope: Option<Scope> = None;
    let mut class: Option<Class> = None;
    let mut defers: Vec<String> = Vec::new();
    let mut previous = ("id".to_string(), 0usize);

    for token in tokens {
        let Some((name, value)) = token.split_once('=') else {
            return Err(unreadable(format!(
                "expected `<attribute>=<value>`, got `{token}`"
            )));
        };
        let rank = match name {
            "kinds" => 1,
            "class" => 2,
            "defers" => 3,
            _ => return Err(fault(Fault::UnknownAttribute(name.to_string()))),
        };
        if rank <= previous.1 {
            return Err(fault(Fault::AttributesOutOfOrder {
                attribute: name.to_string(),
                after: previous.0,
            }));
        }
        previous = (name.to_string(), rank);
        match name {
            "kinds" => scope = Some(parse_scope(value).map_err(&fault)?),
            "class" => {
                class = Some(Class::from_label(value).ok_or_else(|| {
                    unreadable(format!(
                        "`class` must be `triggering` or `procedural`, got `{value}`"
                    ))
                })?)
            }
            _ => defers = parse_id_list("defers", value).map_err(&unreadable)?,
        }
    }

    let class = class.ok_or_else(|| fault(Fault::MissingClass))?;
    match (class, &scope) {
        (Class::Procedural, Some(_)) => return Err(fault(Fault::KindsOnProcedural)),
        (Class::Triggering, None) => return Err(fault(Fault::MissingKinds)),
        _ => {}
    }

    Ok(Unit {
        id,
        class,
        scope,
        defers,
        file: file.to_string(),
        // Filled in by `parse_units` once the whole file has parsed. A marker
        // says nothing about its file's position, and a placeholder that
        // escaped would be a silent zero rather than a visible error — which is
        // why the caller sets it on every unit unconditionally.
        file_order: 0,
        line: line_number,
        offset,
        source: String::new(),
    })
}

fn parse_scope(value: &str) -> Result<Scope, Fault> {
    if value == "*" {
        return Ok(Scope::All);
    }
    let mut kinds = Vec::new();
    for member in value.split_whitespace() {
        let kind =
            Kind::from_label(member).ok_or_else(|| Fault::UnknownKind(member.to_string()))?;
        kinds.push(kind);
    }
    if kinds.is_empty() {
        return Err(Fault::UnparseableMarker("`kinds` is empty".to_string()));
    }
    Ok(Scope::Kinds(kinds))
}

fn parse_id_list(attribute: &str, value: &str) -> Result<Vec<String>, String> {
    let ids: Vec<String> = value.split_whitespace().map(str::to_string).collect();
    if ids.is_empty() {
        return Err(format!("`{attribute}` is empty"));
    }
    for id in &ids {
        if !is_kebab_case(id) {
            return Err(format!("`{attribute}` member `{id}` is not kebab-case"));
        }
    }
    Ok(ids)
}

/// Split a marker's inner text into `id` and `key=value` tokens, honouring the
/// double quotes a multi-member `kinds=` or `defers=` needs.
///
/// An *unquoted* space therefore ends a token, which is what makes
/// `kinds=impl design` an error rather than a silently truncated scope: the
/// second word arrives as a token with no `=` in it.
fn tokenize(inner: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut started = false;
    let mut quoted = false;
    for character in inner.chars() {
        if character == '"' {
            quoted = !quoted;
            started = true;
        } else if character.is_whitespace() && !quoted {
            if started {
                tokens.push(std::mem::take(&mut current));
                started = false;
            }
        } else {
            current.push(character);
            started = true;
        }
    }
    if quoted {
        return Err("a quoted value is never closed".to_string());
    }
    if started {
        tokens.push(current);
    }
    Ok(tokens)
}

fn is_kebab_case(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
        && value
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every fixture below opens with a **file directive**, because every
    /// embedded file does and this reader's contract is over one of those.
    ///
    /// It is written into each fixture rather than prepended by these helpers.
    /// Several assertions here are about exact line and byte coordinates, and a
    /// helper that silently shifted them would leave those assertions saying
    /// something other than what they read.
    fn units(text: &str) -> Vec<Unit> {
        parse_units("FIXTURE.md", text).expect("fixture must parse")
    }

    fn fault(text: &str) -> Fault {
        parse_units("FIXTURE.md", text)
            .expect_err("fixture must be rejected")
            .fault
    }

    // -- The shapes that decide the reading rule ----------------------------
    //
    // Accepted *and* ignored alike. The repository's instructed-verb scanner is
    // the precedent, and its two live holes are the reason: a reader is only as
    // good as the shapes it is pinned on, and a fixture set that never carries
    // the interesting shape goes green on a parser that rejects the real embed.

    #[test]
    fn a_file_with_no_leading_block_starts_its_body_at_the_directive() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             body\n",
        );
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].id, "alpha");
        assert_eq!(
            (parsed[0].line, parsed[0].offset),
            (2, 23),
            "a unit is located past the file directive, in both coordinates"
        );
        assert_eq!(parsed[0].file_order, 1);
        assert_eq!(
            parsed[0].source, "<!-- unit: alpha kinds=* class=triggering -->\nbody\n",
            "the marker line is part of the unit's source, and the directive is not"
        );
    }

    /// The shape that keeps `content/SKILL.md` parseable while it is still
    /// provisioned: its YAML frontmatter is what every harness reads to discover
    /// the skill, and without this rule the per-file gate would reject the real
    /// embed on the day it landed. The directive sits **after** the block, in
    /// the body, because it is read and the block is not.
    #[test]
    fn a_leading_delimited_block_is_skipped_and_belongs_to_no_unit() {
        let parsed = units(
            "---\n\
             name: grove\n\
             ---\n\
             <!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             body\n",
        );
        assert_eq!(parsed.len(), 1);
        assert_eq!(
            (parsed[0].line, parsed[0].offset),
            (5, 43),
            "a unit is located past the unread preamble and the directive"
        );
        assert!(
            !parsed[0].source.contains("name: grove"),
            "the preamble belongs to no unit: {:?}",
            parsed[0].source
        );
        assert!(parsed[0].source.starts_with("<!-- unit: alpha"));
    }

    #[test]
    fn an_unterminated_leading_block_is_rejected_on_the_line_it_opened() {
        let error = parse_units("FIXTURE.md", "---\nname: grove\n# heading\n").unwrap_err();
        assert_eq!(error.fault, Fault::UnterminatedPreamble);
        assert_eq!((error.line, error.offset), (1, 0));
    }

    #[test]
    fn a_marker_inside_a_balanced_fence_declares_no_unit() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ```text\n\
             <!-- unit: example kinds=* class=triggering -->\n\
             ```\n\
             after\n",
        );
        assert_eq!(
            parsed.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
            ["alpha"],
            "an example marker is prose, and its bytes belong to the unit containing the fence"
        );
        assert!(parsed[0].source.contains("<!-- unit: example"));
    }

    #[test]
    fn a_longer_fence_is_not_closed_by_a_shorter_run() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ````\n\
             ```\n\
             ````\n\
             <!-- unit: beta kinds=* class=triggering -->\n",
        );
        assert_eq!(
            parsed.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
            ["alpha", "beta"]
        );
    }

    /// CommonMark allows a fence up to three columns of indentation, and the
    /// bound binds in the accepting direction too — a fence one column in still
    /// makes the marker it contains an example.
    #[test]
    fn a_fence_indented_within_three_columns_still_opens() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             \x20  ```text\n\
             <!-- unit: example kinds=* class=triggering -->\n\
             \x20  ```\n",
        );
        assert_eq!(
            parsed.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
            ["alpha"]
        );
    }

    /// The **swallowing** direction of a loose fence rule. Four columns is an
    /// indented code block, not a fence, so the marker after it is real; reading
    /// it as a fence would absorb `beta` into `alpha` and lose a unit.
    #[test]
    fn a_fence_indented_past_three_columns_is_not_a_fence() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             \x20   ```\n\
             <!-- unit: beta kinds=* class=triggering -->\n\
             \x20   ```\n",
        );
        assert_eq!(
            parsed.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
            ["alpha", "beta"],
            "a four-column ````` is an indented code block; the marker below it is a boundary"
        );
    }

    /// A backtick fence's info string may hold no backtick, or every paragraph
    /// opening with an inline code span would open a block and swallow the rest
    /// of the file.
    #[test]
    fn a_backtick_info_string_containing_a_backtick_opens_nothing() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ```x``` is a code span, not a fence\n\
             <!-- unit: beta kinds=* class=triggering -->\n",
        );
        assert_eq!(
            parsed.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
            ["alpha", "beta"]
        );
    }

    #[test]
    fn an_indented_marker_shaped_line_declares_no_unit() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             \x20 <!-- unit: beta kinds=* class=triggering -->\n",
        );
        assert_eq!(
            parsed.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
            ["alpha"]
        );
    }

    /// Partition is stated over the region the directive leaves behind, which is
    /// the whole of what the directive costs: **one** body line belongs to no
    /// unit, it is the first, and it is the only one.
    #[test]
    fn units_partition_everything_after_the_directive() {
        let partitioned = "<!-- unit: alpha kinds=* class=triggering -->\none\n\
                           <!-- unit: beta class=procedural -->\ntwo\n";
        let parsed = units(&format!("<!-- file: order=4 -->\n{partitioned}"));
        assert_eq!(
            parsed.iter().map(|u| u.source.as_str()).collect::<String>(),
            partitioned,
            "concatenating every unit's source must reproduce the body past the directive exactly"
        );
        assert!(
            parsed.iter().all(|unit| unit.file_order == 4),
            "one directive sets the position of every unit in its file"
        );
    }

    #[test]
    fn a_multi_member_scope_and_deferral_are_quoted_and_ordered() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=\"impl design\" class=triggering defers=\"beta gamma\" -->\n\
             body\n",
        );
        assert_eq!(
            parsed[0].scope,
            Some(Scope::Kinds(vec![Kind::Impl, Kind::Design]))
        );
        assert_eq!(parsed[0].scope.as_ref().unwrap().render(), "impl design");
        assert_eq!(parsed[0].defers, ["beta", "gamma"]);
    }

    #[test]
    fn a_procedural_unit_carries_no_scope_and_may_still_defer() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha class=procedural defers=beta -->\n\
             body\n",
        );
        assert_eq!(parsed[0].class, Class::Procedural);
        assert_eq!(parsed[0].scope, None);
        assert_eq!(parsed[0].defers, ["beta"]);
    }

    /// A scope admits `*` universally and a list by membership, and nothing
    /// else. Both directions are asserted, because a predicate that answered
    /// `true` everywhere would satisfy the first half alone.
    #[test]
    fn a_scope_admits_every_kind_or_exactly_its_members() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: wide kinds=* class=triggering -->\nbody\n\
             <!-- unit: narrow kinds=\"impl finish\" class=triggering -->\nbody\n",
        );
        let scope_of = |id: &str| {
            parsed
                .iter()
                .find(|unit| unit.id == id)
                .and_then(|unit| unit.scope.clone())
                .expect("a triggering unit carries a scope")
        };

        assert!(Kind::ALL
            .into_iter()
            .all(|kind| scope_of("wide").admits(kind)));
        let narrow = scope_of("narrow");
        assert!(narrow.admits(Kind::Impl) && narrow.admits(Kind::Finish));
        assert!(
            Kind::ALL
                .into_iter()
                .filter(|kind| !matches!(kind, Kind::Impl | Kind::Finish))
                .all(|kind| !narrow.admits(kind)),
            "a narrowed scope must be absent from the other seventeen; that absence \
             is half of what the completeness invariant asserts"
        );
    }

    // -- The file directive's own shapes ------------------------------------

    #[test]
    fn a_file_declaring_no_mandate_position_is_rejected() {
        let error = parse_units(
            "FIXTURE.md",
            "<!-- unit: alpha kinds=* class=triggering -->\nbody\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::MissingFileOrder);
        assert_eq!(
            (error.line, error.offset),
            (1, 0),
            "reported where the directive should have been — the body's first line"
        );
    }

    /// A file that carries a position but declares no unit fails as the empty
    /// file it is. The order is deliberate: a position is a fact *about* a file
    /// of units, so complaining about it first would name the wrong repair.
    #[test]
    fn an_ordered_file_declaring_no_unit_still_fails_as_empty() {
        assert_eq!(fault("<!-- file: order=1 -->\n"), Fault::NoUnitDeclared);
    }

    /// A second directive is the same fault as a misplaced first one, and that
    /// is not a shortcut: a file carries exactly one position, and every
    /// directive but the body's first line sits *inside* a unit, where it would
    /// ship into a mandate as prose.
    #[test]
    fn a_second_file_directive_is_rejected_wherever_it_sits() {
        let error = parse_units(
            "FIXTURE.md",
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             <!-- file: order=2 -->\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::MisplacedFileOrder);
        assert_eq!(error.line, 3);

        assert_eq!(
            fault(
                "<!-- unit: alpha kinds=* class=triggering -->\n\
                 <!-- file: order=1 -->\n",
            ),
            Fault::MisplacedFileOrder,
            "a directive below the first marker is misplaced even when it is the only one"
        );
    }

    /// The **asymmetry with fences, deliberately the same as the markers'.**
    /// Strictness only ever *withholds* directive-hood, so an indented directive
    /// is not one — and the residue lands in the visible direction: as the body's
    /// first line it costs the file its position, and anywhere else it is inert
    /// prose inside a unit whose file already has one.
    #[test]
    fn an_indented_file_directive_declares_no_position() {
        assert_eq!(
            fault(
                "\x20<!-- file: order=1 -->\n\
                 <!-- unit: alpha kinds=* class=triggering -->\n",
            ),
            Fault::BodyBeforeFirstMarker,
            "one column in, it is text before the first marker rather than a position"
        );

        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             \x20<!-- file: order=2 -->\n",
        );
        assert_eq!(parsed[0].file_order, 1, "and inert once the file has one");
    }

    /// The methodology documents itself, so an example directive inside a fence
    /// is certain to appear — and it must be prose, exactly as an example
    /// marker is.
    #[test]
    fn a_file_directive_inside_a_balanced_fence_declares_no_position() {
        let parsed = units(
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ```text\n\
             <!-- file: order=9 -->\n\
             ```\n",
        );
        assert_eq!(parsed[0].file_order, 1);
        assert!(
            parsed[0].source.contains("<!-- file: order=9 -->"),
            "the example's bytes belong to the unit containing the fence"
        );
    }

    #[test]
    fn an_unreadable_file_directive_is_rejected_rather_than_read_as_prose() {
        let unreadable = |text: &str| {
            assert!(
                matches!(fault(text), Fault::UnparseableFileDirective(_)),
                "expected an unreadable directive from {text:?}, got {:?}",
                fault(text)
            );
        };
        unreadable("<!-- file: order=1\n");
        unreadable("<!-- file: -->\n");
        unreadable("<!-- file: position=1 -->\n");
        unreadable("<!-- file: order=first -->\n");
        unreadable("<!-- file: order=-1 -->\n");
        unreadable("<!-- file: order=1 order=2 -->\n");
    }

    // -- The malformed cases, each asserted to fail -------------------------

    #[test]
    fn an_unparseable_marker_is_rejected_rather_than_read_as_prose() {
        let unreadable = |text: &str| {
            assert!(
                matches!(
                    fault(&format!("<!-- file: order=1 -->\n{text}")),
                    Fault::UnparseableMarker(_)
                ),
                "expected an unreadable marker from {text:?}"
            );
        };
        unreadable("<!-- unit: alpha kinds=* class=triggering\n");
        unreadable("<!-- unit: -->\n");
        unreadable("<!-- unit: Alpha kinds=* class=triggering -->\n");
        unreadable("<!-- unit: alpha kinds=\"impl class=triggering -->\n");
        // An unquoted multi-member scope must fail rather than silently truncate.
        unreadable("<!-- unit: alpha kinds=impl design class=triggering -->\n");
    }

    #[test]
    fn an_unknown_attribute_is_rejected() {
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha scope=* class=triggering -->\n",
            ),
            Fault::UnknownAttribute("scope".to_string())
        );
    }

    #[test]
    fn attributes_out_of_the_fixed_order_are_rejected() {
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha class=triggering kinds=* -->\n",
            ),
            Fault::AttributesOutOfOrder {
                attribute: "kinds".to_string(),
                after: "class".to_string(),
            }
        );
        assert!(
            matches!(
                fault(
                    "<!-- file: order=1 -->\n\
                     <!-- unit: alpha kinds=* kinds=* class=triggering -->\n",
                ),
                Fault::AttributesOutOfOrder { .. }
            ),
            "a repeated attribute is out of a strictly increasing order"
        );
    }

    #[test]
    fn a_missing_class_is_rejected() {
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha kinds=* -->\n",
            ),
            Fault::MissingClass
        );
    }

    #[test]
    fn the_scope_rule_binds_both_ways() {
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha kinds=* class=procedural -->\n",
            ),
            Fault::KindsOnProcedural
        );
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha class=triggering -->\n",
            ),
            Fault::MissingKinds
        );
    }

    #[test]
    fn a_scope_member_outside_the_closed_nineteen_is_rejected() {
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha kinds=producers class=triggering -->\n",
            ),
            Fault::UnknownKind("producers".to_string()),
            "no family shorthand: it would silently absorb a kind added later"
        );
        assert_eq!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: alpha kinds=work class=triggering -->\n",
            ),
            Fault::UnknownKind("work".to_string()),
            "the read-side `work` alias is a task-filename concession, not a scope"
        );
    }

    #[test]
    fn a_file_declaring_no_unit_is_rejected() {
        assert_eq!(fault("# heading\n\nprose\n"), Fault::NoUnitDeclared);
        assert_eq!(fault(""), Fault::NoUnitDeclared);
    }

    #[test]
    fn body_text_before_the_first_marker_is_rejected_with_the_bodys_offset() {
        let error = parse_units(
            "FIXTURE.md",
            "---\nname: x\n---\n\
             <!-- file: order=1 -->\n\
             prose\n\
             <!-- unit: a kinds=* class=triggering -->\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::BodyBeforeFirstMarker);
        assert_eq!(
            (error.line, error.offset),
            (5, 39),
            "the offset names where the units begin, past the unread preamble and the directive"
        );
    }

    #[test]
    fn an_unterminated_fence_is_rejected_on_the_line_it_opened() {
        let error = parse_units(
            "FIXTURE.md",
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ```\n\
             swallowed\n\
             <!-- unit: beta kinds=* class=triggering -->\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::UnterminatedFence);
        assert_eq!(error.line, 3);
    }

    /// The **silent** direction of a loose fence rule, and the reason the close
    /// is as exact as the open. A four-column run is content, so the fence is
    /// still open at end of file; reading it as a close would return to neutral
    /// inside a block a reader sees as code and promote `example` to a unit.
    #[test]
    fn an_over_indented_run_does_not_close_a_fence() {
        let error = parse_units(
            "FIXTURE.md",
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ```\n\
             <!-- unit: example kinds=* class=triggering -->\n\
             \x20   ```\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::UnterminatedFence);
        assert_eq!(error.line, 3);
    }

    /// A close carries no info string, so a second opener-shaped line does not
    /// end the block it looks like it belongs to.
    #[test]
    fn a_run_carrying_an_info_string_does_not_close_a_fence() {
        let error = parse_units(
            "FIXTURE.md",
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             ```\n\
             ```text\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::UnterminatedFence);
    }

    /// The collision `---` cannot resolve on its own: read as a thematic break
    /// this is body, read as a preamble opener it hides `hidden` and its prose.
    /// The spelling is reserved for the preamble, and a marker caught inside one
    /// is what keeps the reservation from costing a classified unit.
    #[test]
    fn a_marker_inside_the_leading_block_is_rejected() {
        let error = parse_units(
            "FIXTURE.md",
            "---\n\
             <!-- unit: hidden kinds=* class=triggering -->\n\
             prose\n\
             ---\n\
             <!-- file: order=1 -->\n\
             <!-- unit: visible kinds=* class=triggering -->\n",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::MarkerInPreamble);
        assert_eq!(error.line, 2, "named on the marker the block would swallow");
    }

    /// The last unit of a file is served through end of file, and a fetch
    /// concatenates units with nothing between them — so without a final newline
    /// the next unit's marker stops being a whole line.
    #[test]
    fn a_file_that_does_not_end_in_a_newline_is_rejected() {
        let error = parse_units(
            "FIXTURE.md",
            "<!-- file: order=1 -->\n\
             <!-- unit: alpha kinds=* class=triggering -->\n\
             body",
        )
        .unwrap_err();
        assert_eq!(error.fault, Fault::MissingFinalNewline);
        assert_eq!((error.line, error.offset), (3, 73));
    }

    /// The listing's five fields need no escaping rule only while no field can
    /// carry a delimiter. Four are closed sets; the fifth is a filename, which is
    /// data rather than a grammar until the build makes it one.
    #[test]
    fn a_path_no_listing_row_could_carry_is_rejected() {
        let file = "<!-- file: order=1 -->\n<!-- unit: alpha kinds=* class=triggering -->\n";
        assert_eq!(
            parse_units("two\tfields.md", file).unwrap_err().fault,
            Fault::UnlistablePath('\t')
        );
        assert_eq!(
            parse_units("two\nrows.md", file).unwrap_err().fault,
            Fault::UnlistablePath('\n')
        );
        assert!(
            parse_units("prompts/continue.md", file).is_ok(),
            "an ordinary nested path is untouched"
        );
    }

    // -- The classifier must be able to fail --------------------------------
    //
    // A well-formed pattern matching nothing reads exactly like a clean
    // repository, so the pair is asserted together: the same reader accepts a
    // synthetic well-formed marker and rejects a synthetic malformed one.

    #[test]
    fn the_reader_separates_a_well_formed_marker_from_a_malformed_one() {
        assert_eq!(
            units(
                "<!-- file: order=1 -->\n\
                 <!-- unit: well-formed kinds=* class=triggering -->\n",
            )[0]
            .id,
            "well-formed"
        );
        assert!(matches!(
            fault(
                "<!-- file: order=1 -->\n\
                 <!-- unit: mal formed kinds=* class=triggering -->\n",
            ),
            Fault::UnparseableMarker(_)
        ));
    }

    #[test]
    fn an_error_names_the_file_the_line_and_the_offset() {
        let rendered = parse_units(
            "SKILL.md",
            "<!-- file: order=1 -->\n<!-- unit: alpha kinds=* -->\n",
        )
        .unwrap_err()
        .to_string();
        assert!(
            rendered.starts_with("SKILL.md:2:23: "),
            "a build error must be openable: {rendered}"
        );
    }
}

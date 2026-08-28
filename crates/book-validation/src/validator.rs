use std::collections::{BTreeMap, BTreeSet};

use crate::parser::{self, Child, Fragment, FragmentBody, LineRange, ParsedBook, Root};
use crate::{BookSnapshot, Coverage, Diagnostic, ReportStatus, Request, Scope, ValidationReport};

const SLICE_ORDER: &[&str] = &[
    "orientation-k11",
    "name-seam-k12",
    "reference-domain-k13",
    "read-path-k14",
    "mutation-algebra-k15",
    "filesystem-interpreter-k16",
    "syllabus-cli-k17",
    "book-assembly-k18",
];

const ROOTS: &[(&str, &str, usize)] = &[
    (
        "source-crate-manifest",
        "crates/ordinal-fs-tree/Cargo.toml",
        116,
    ),
    (
        "source-syllabus-cli",
        "crates/ordinal-fs-tree/bin/syllabus.rs",
        1_128,
    ),
    ("source-library", "crates/ordinal-fs-tree/src/lib.rs", 94),
    (
        "source-conformance",
        "crates/ordinal-fs-tree/src/conformance.rs",
        636,
    ),
    ("source-error", "crates/ordinal-fs-tree/src/error.rs", 342),
    ("source-name", "crates/ordinal-fs-tree/src/name.rs", 700),
    (
        "source-operations",
        "crates/ordinal-fs-tree/src/ops.rs",
        543,
    ),
    ("source-plan", "crates/ordinal-fs-tree/src/plan.rs", 568),
    (
        "source-reference",
        "crates/ordinal-fs-tree/src/reference.rs",
        555,
    ),
    ("source-report", "crates/ordinal-fs-tree/src/report.rs", 152),
    (
        "source-snapshot",
        "crates/ordinal-fs-tree/src/snapshot.rs",
        650,
    ),
    (
        "source-filesystem-module",
        "crates/ordinal-fs-tree/src/fs/mod.rs",
        393,
    ),
    (
        "source-filesystem-read",
        "crates/ordinal-fs-tree/src/fs/read.rs",
        179,
    ),
    (
        "source-filesystem-apply",
        "crates/ordinal-fs-tree/src/fs/apply.rs",
        471,
    ),
    (
        "source-filesystem-lock",
        "crates/ordinal-fs-tree/src/fs/lock.rs",
        91,
    ),
];

#[derive(Clone, Copy)]
struct Block {
    id: &'static str,
    root: &'static str,
    owner: &'static str,
    first: usize,
    last: usize,
}

const BLOCKS: &[Block] = &[
    block(
        "manifest-package-and-library-dependency",
        "source-crate-manifest",
        "orientation-k11",
        1,
        42,
    ),
    block(
        "manifest-cli-feature",
        "source-crate-manifest",
        "syllabus-cli-k17",
        43,
        45,
    ),
    block(
        "manifest-library-cli-boundary",
        "source-crate-manifest",
        "orientation-k11",
        46,
        61,
    ),
    block(
        "manifest-cli-binary",
        "source-crate-manifest",
        "syllabus-cli-k17",
        62,
        65,
    ),
    block(
        "manifest-development-and-release",
        "source-crate-manifest",
        "orientation-k11",
        66,
        116,
    ),
    block(
        "syllabus-cli-source",
        "source-syllabus-cli",
        "syllabus-cli-k17",
        1,
        1_128,
    ),
    block(
        "library-crate-surface",
        "source-library",
        "orientation-k11",
        1,
        94,
    ),
    block(
        "reference-conformance-source",
        "source-conformance",
        "reference-domain-k13",
        1,
        636,
    ),
    block(
        "filesystem-error-source",
        "source-error",
        "filesystem-interpreter-k16",
        1,
        342,
    ),
    block("name-seam-source", "source-name", "name-seam-k12", 1, 700),
    block(
        "mutation-operations-source",
        "source-operations",
        "mutation-algebra-k15",
        1,
        543,
    ),
    block(
        "mutation-plan-source",
        "source-plan",
        "mutation-algebra-k15",
        1,
        568,
    ),
    block(
        "reference-domain-source",
        "source-reference",
        "reference-domain-k13",
        1,
        555,
    ),
    block(
        "mutation-report-source",
        "source-report",
        "mutation-algebra-k15",
        1,
        152,
    ),
    block(
        "read-snapshot-source",
        "source-snapshot",
        "read-path-k14",
        1,
        650,
    ),
    block(
        "filesystem-read-opening",
        "source-filesystem-module",
        "read-path-k14",
        1,
        86,
    ),
    block(
        "filesystem-write-acquire",
        "source-filesystem-module",
        "filesystem-interpreter-k16",
        87,
        105,
    ),
    block(
        "filesystem-read-acquire-and-guard",
        "source-filesystem-module",
        "read-path-k14",
        106,
        131,
    ),
    block(
        "filesystem-write-guard",
        "source-filesystem-module",
        "filesystem-interpreter-k16",
        132,
        154,
    ),
    block(
        "filesystem-read-guard-api",
        "source-filesystem-module",
        "read-path-k14",
        155,
        168,
    ),
    block(
        "filesystem-write-guard-api",
        "source-filesystem-module",
        "filesystem-interpreter-k16",
        169,
        378,
    ),
    block(
        "filesystem-read-deref",
        "source-filesystem-module",
        "read-path-k14",
        379,
        386,
    ),
    block(
        "filesystem-write-deref",
        "source-filesystem-module",
        "filesystem-interpreter-k16",
        387,
        393,
    ),
    block(
        "read-filesystem-source",
        "source-filesystem-read",
        "read-path-k14",
        1,
        179,
    ),
    block(
        "filesystem-interpreter-source",
        "source-filesystem-apply",
        "filesystem-interpreter-k16",
        1,
        471,
    ),
    block(
        "filesystem-lock-source",
        "source-filesystem-lock",
        "filesystem-interpreter-k16",
        1,
        91,
    ),
];

const fn block(
    id: &'static str,
    root: &'static str,
    owner: &'static str,
    first: usize,
    last: usize,
) -> Block {
    Block {
        id,
        root,
        owner,
        first,
        last,
    }
}

pub fn validate(snapshot: &BookSnapshot, request: Request) -> ValidationReport {
    let crate::Check::Fragments = request.check;
    let parsed = parser::parse(snapshot);
    let mut diagnostics = parsed.diagnostics.clone();
    if let Scope::Through(slice) = &request.scope {
        if !SLICE_ORDER[..SLICE_ORDER.len() - 1].contains(&slice.as_str()) {
            diagnostics.push(Diagnostic::new(
                "U001",
                "parse",
                format!("unknown scoped slice `{slice}`"),
                crate::Location {
                    path: "<command>".into(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                None,
            ));
        }
    }
    check_inventory(snapshot, &parsed, &mut diagnostics);
    check_identities(&parsed, &mut diagnostics);
    check_references(&parsed, &request.scope, &mut diagnostics);
    check_ownership(&parsed, &request.scope, &mut diagnostics);
    check_graph(&parsed, &mut diagnostics);
    check_coverage(&parsed, &mut diagnostics);
    check_bytes(snapshot, &parsed, &mut diagnostics);
    diagnostics.sort_by(|left, right| {
        phase_order(&left.phase)
            .cmp(&phase_order(&right.phase))
            .then_with(|| left.primary.cmp(&right.primary))
            .then_with(|| left.code.cmp(&right.code))
            .then_with(|| left.fragment_id.cmp(&right.fragment_id))
            .then_with(|| left.message.cmp(&right.message))
    });
    diagnostics.dedup();
    let coverage = coverage(&parsed, matches!(request.scope, Scope::Final));
    let valid = diagnostics.is_empty();
    ValidationReport {
        schema: 1,
        status: if valid {
            ReportStatus::Valid
        } else {
            ReportStatus::Findings
        },
        valid,
        scope: request.scope,
        coverage,
        diagnostics,
    }
}

fn check_inventory(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for (id, source, lines) in ROOTS {
        match snapshot.source_files.get(*source) {
            Some(bytes) if source_line_count(bytes) == *lines => {}
            Some(bytes) => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!(
                    "authoritative source `{source}` has {} lines; fixed inventory requires {lines}",
                    source_line_count(bytes)
                ),
                crate::Location {
                    path: (*source).into(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                Some(id),
            )),
            None => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("required authoritative source `{source}` is missing"),
                crate::Location {
                    path: (*source).into(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                Some(id),
            )),
        }
        match parsed.roots.get(*id) {
            Some(roots)
                if roots.len() == 1
                    && roots[0].source == *source
                    && roots[0].range
                        == (LineRange {
                            first: 1,
                            last: *lines,
                        }) => {}
            Some(roots) => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("source root `{id}` is duplicated or disagrees with the fixed inventory"),
                roots[0].location.clone(),
                None,
                Some(id),
            )),
            None => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("required source root `{id}` is missing"),
                crate::Location {
                    path: "docs/ordinal-fs-tree/book/source-index.md".into(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                Some(id),
            )),
        }
    }
    for (id, roots) in &parsed.roots {
        if !ROOTS.iter().any(|(expected, _, _)| expected == id) {
            diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("source root `{id}` is not in the fixed inventory"),
                roots[0].location.clone(),
                None,
                Some(id),
            ));
        }
    }
    for path in snapshot.source_files.keys() {
        if !ROOTS.iter().any(|(_, expected, _)| expected == path) {
            diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("authoritative source `{path}` is outside the fixed corpus"),
                crate::Location {
                    path: path.clone(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                None,
            ));
        }
    }
}

fn check_identities(parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
    let mut locations: BTreeMap<&str, Vec<_>> = BTreeMap::new();
    for roots in parsed.roots.values() {
        for root in roots {
            locations.entry(&root.id).or_default().push(&root.location);
        }
    }
    for fragments in parsed.fragments.values() {
        for fragment in fragments {
            locations
                .entry(&fragment.id)
                .or_default()
                .push(&fragment.location);
        }
    }
    for (id, occurrences) in locations {
        if occurrences.len() > 1 {
            diagnostics.push(Diagnostic::new(
                "F001",
                "identity",
                format!("fragment or source-root ID `{id}` is duplicated"),
                occurrences[0].clone(),
                Some(id),
                None,
            ));
        }
    }
}

fn check_references(parsed: &ParsedBook, scope: &Scope, diagnostics: &mut Vec<Diagnostic>) {
    let through = match scope {
        Scope::Through(slice) => SLICE_ORDER.iter().position(|candidate| *candidate == slice),
        Scope::Final => Some(SLICE_ORDER.len()),
    };
    for (parent, root_id, children) in all_children(parsed) {
        for child in children {
            match child {
                Child::Insert { id, location } => {
                    if parsed.fragments.get(id).map_or(0, Vec::len) != 1 {
                        diagnostics.push(Diagnostic::new(
                            "F002",
                            "reference",
                            format!(
                                "insert target `{id}` is unresolved or ambiguous in `{parent}`"
                            ),
                            location.clone(),
                            Some(id),
                            root_id,
                        ));
                    }
                }
                Child::Defer {
                    id,
                    owner,
                    location,
                    ..
                } => {
                    let owner_order = SLICE_ORDER.iter().position(|candidate| *candidate == owner);
                    let invalid = matches!(scope, Scope::Final)
                        || owner_order.is_none()
                        || through
                            .zip(owner_order)
                            .is_some_and(|(now, owner)| owner <= now);
                    if invalid {
                        diagnostics.push(Diagnostic::new(
                            "F003",
                            "reference",
                            format!("deferred fragment `{id}` is not owned by a later slice"),
                            location.clone(),
                            Some(id),
                            root_id,
                        ));
                    }
                    if parsed.fragments.contains_key(id) {
                        diagnostics.push(Diagnostic::new(
                            "F003",
                            "reference",
                            format!(
                                "deferred fragment `{id}` is defined before its parent replaces the defer"
                            ),
                            location.clone(),
                            Some(id),
                            root_id,
                        ));
                    }
                }
            }
        }
    }
}

fn check_ownership(parsed: &ParsedBook, scope: &Scope, diagnostics: &mut Vec<Diagnostic>) {
    let through = match scope {
        Scope::Through(slice) => SLICE_ORDER.iter().position(|candidate| *candidate == slice),
        Scope::Final => Some(SLICE_ORDER.len()),
    };
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        let root = &roots[0];
        let expected: Vec<&Block> = BLOCKS
            .iter()
            .filter(|block| block.root == root.id)
            .collect();
        for (position, child) in root.children.iter().enumerate() {
            let (id, owner, range, is_defer, location) = match child {
                Child::Defer {
                    id,
                    owner,
                    range,
                    location,
                } => (id.as_str(), owner.as_str(), *range, true, location),
                Child::Insert { id, location } => {
                    let Some(fragment) = parsed
                        .fragments
                        .get(id)
                        .filter(|items| items.len() == 1)
                        .map(|items| &items[0])
                    else {
                        continue;
                    };
                    (
                        id.as_str(),
                        fragment.owner.as_str(),
                        fragment.range,
                        false,
                        location,
                    )
                }
            };
            let Some(block) = expected.get(position).copied() else {
                diagnostics.push(Diagnostic::new(
                    "F003",
                    "reference",
                    format!("`{id}` is not a declared ownership block of `{}`", root.id),
                    location.clone(),
                    Some(id),
                    Some(&root.id),
                ));
                continue;
            };
            let owner_order = SLICE_ORDER
                .iter()
                .position(|candidate| *candidate == block.owner);
            let should_defer = !matches!(scope, Scope::Final)
                && through
                    .zip(owner_order)
                    .is_some_and(|(now, owner)| owner > now);
            if id != block.id
                || owner != block.owner
                || range
                    != (LineRange {
                        first: block.first,
                        last: block.last,
                    })
                || is_defer != should_defer
            {
                diagnostics.push(Diagnostic::new(
                    "F003",
                    "reference",
                    format!(
                        "ownership block `{id}` does not match the required state for `{}`",
                        root.id
                    ),
                    location.clone(),
                    Some(id),
                    Some(&root.id),
                ));
            }
        }
        if root.children.len() != expected.len() {
            diagnostics.push(Diagnostic::new(
                "F003",
                "reference",
                format!(
                    "source root `{}` does not contain its exact ownership block set",
                    root.id
                ),
                root.location.clone(),
                None,
                Some(&root.id),
            ));
        }
    }
}

fn check_graph(parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
    let unique: BTreeMap<&str, &Fragment> = parsed
        .fragments
        .iter()
        .filter_map(|(id, fragments)| {
            (fragments.len() == 1).then_some((id.as_str(), &fragments[0]))
        })
        .collect();
    let mut reached: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut reported_cycles = BTreeSet::new();
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        let root = &roots[0];
        let mut stack = Vec::new();
        for child in &root.children {
            if let Child::Insert { id, .. } = child {
                visit(
                    id,
                    &unique,
                    &mut stack,
                    &mut reached,
                    &mut reported_cycles,
                    diagnostics,
                    root,
                );
            }
        }
    }
    for (id, fragments) in &parsed.fragments {
        if fragments.len() == 1 {
            let fragment = &fragments[0];
            let observed = reached.get(id);
            let expected = root_for(parsed, fragment);
            let wrong = observed.is_none_or(BTreeSet::is_empty)
                || observed.is_some_and(|roots| {
                    roots.len() != 1 || expected.is_none_or(|root| !roots.contains(root))
                });
            if wrong {
                diagnostics.push(Diagnostic::new(
                    "F005",
                    "graph",
                    format!("fragment `{id}` is unreachable or reached from the wrong source root"),
                    fragment.location.clone(),
                    Some(id),
                    expected,
                ));
            }
        }
    }
}

fn check_coverage(parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        check_child_partition(
            parsed,
            &roots[0].id,
            &roots[0].source,
            roots[0].range,
            &roots[0].children,
            None,
            &roots[0].location,
            diagnostics,
        );
    }
    for fragments in parsed
        .fragments
        .values()
        .filter(|fragments| fragments.len() == 1)
    {
        let fragment = &fragments[0];
        if let FragmentBody::Composite(children) = &fragment.body {
            check_child_partition(
                parsed,
                &fragment.id,
                &fragment.source,
                fragment.range,
                children,
                Some(&fragment.owner),
                &fragment.location,
                diagnostics,
            );
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn check_child_partition(
    parsed: &ParsedBook,
    parent: &str,
    source: &str,
    range: LineRange,
    children: &[Child],
    parent_owner: Option<&str>,
    parent_location: &crate::Location,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let mut next = range.first;
    for child in children {
        let (id, child_range, child_source, child_owner, location) = match child {
            Child::Defer {
                id,
                owner,
                range,
                location,
            } => (id.as_str(), *range, source, owner.as_str(), location),
            Child::Insert { id, location } => {
                let Some(fragment) = parsed
                    .fragments
                    .get(id)
                    .filter(|items| items.len() == 1)
                    .map(|items| &items[0])
                else {
                    continue;
                };
                if fragment.parent != parent {
                    diagnostics.push(Diagnostic::new(
                        "F007",
                        "coverage",
                        format!(
                            "fragment `{id}` declares parent `{}` but is inserted by `{parent}`",
                            fragment.parent
                        ),
                        location.clone(),
                        Some(id),
                        None,
                    ));
                }
                (
                    id.as_str(),
                    fragment.range,
                    fragment.source.as_str(),
                    fragment.owner.as_str(),
                    location,
                )
            }
        };
        let owner_mismatch = parent_owner.is_some_and(|owner| owner != child_owner);
        if child_range.first != next
            || child_range.last > range.last
            || child_source != source
            || owner_mismatch
        {
            diagnostics.push(Diagnostic::new(
                "F007",
                "coverage",
                format!("child `{id}` does not form the required ordered source partition of `{parent}`"),
                location.clone(),
                Some(id),
                None,
            ));
        }
        next = child_range.last.saturating_add(1);
    }
    if next != range.last.saturating_add(1) {
        diagnostics.push(Diagnostic::new(
            "F007",
            "coverage",
            format!(
                "children of `{parent}` do not cover lines {}-{}",
                range.first, range.last
            ),
            parent_location.clone(),
            Some(parent),
            None,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn visit<'a>(
    id: &'a str,
    unique: &BTreeMap<&'a str, &'a Fragment>,
    stack: &mut Vec<&'a str>,
    reached: &mut BTreeMap<String, BTreeSet<String>>,
    reported_cycles: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
    root: &Root,
) {
    let Some(fragment) = unique.get(id) else {
        return;
    };
    if let Some(start) = stack.iter().position(|candidate| *candidate == id) {
        let mut witness = stack[start..].to_vec();
        if let Some((rotation, _)) = witness.iter().enumerate().min_by_key(|(_, item)| *item) {
            witness.rotate_left(rotation);
        }
        witness.push(witness[0]);
        let key = witness.join(" -> ");
        if reported_cycles.insert(key.clone()) {
            diagnostics.push(Diagnostic::new(
                "F004",
                "graph",
                format!("fragment cycle: {key}"),
                fragment.location.clone(),
                Some(id),
                Some(&root.id),
            ));
        }
        return;
    }
    reached
        .entry(id.into())
        .or_default()
        .insert(root.id.clone());
    stack.push(id);
    if let FragmentBody::Composite(children) = &fragment.body {
        for child in children {
            if let Child::Insert { id, .. } = child {
                visit(
                    id,
                    unique,
                    stack,
                    reached,
                    reported_cycles,
                    diagnostics,
                    root,
                );
            }
        }
    }
    stack.pop();
}

fn check_bytes(snapshot: &BookSnapshot, parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
    for fragments in parsed
        .fragments
        .values()
        .filter(|fragments| fragments.len() == 1)
    {
        let fragment = &fragments[0];
        if let FragmentBody::Literal(actual) = &fragment.body {
            let expected = snapshot
                .source_files
                .get(&fragment.source)
                .and_then(|bytes| source_range(bytes, fragment.range));
            if expected.as_deref() != Some(actual.as_slice()) {
                diagnostics.push(Diagnostic::new(
                    "F008",
                    "bytes",
                    "literal bytes differ from authoritative source range",
                    fragment.location.clone(),
                    Some(&fragment.id),
                    root_for(parsed, fragment),
                ));
            }
        }
    }
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        let root = &roots[0];
        let mut visiting = BTreeSet::new();
        if let Some(actual) = expand_children(&root.children, parsed, &mut visiting) {
            let expected = snapshot
                .source_files
                .get(&root.source)
                .and_then(|bytes| source_range(bytes, root.range));
            let has_defer = root
                .children
                .iter()
                .any(|child| matches!(child, Child::Defer { .. }));
            if !has_defer && expected.as_deref() != Some(actual.as_slice()) {
                diagnostics.push(Diagnostic::new(
                    "F008",
                    "bytes",
                    "expanded bytes differ from authoritative source",
                    root.location.clone(),
                    None,
                    Some(&root.id),
                ));
            }
        }
    }
}

fn expand_children(
    children: &[Child],
    parsed: &ParsedBook,
    visiting: &mut BTreeSet<String>,
) -> Option<Vec<u8>> {
    let mut output = Vec::new();
    for child in children {
        match child {
            Child::Defer { .. } => {}
            Child::Insert { id, .. } => {
                if !visiting.insert(id.clone()) {
                    return None;
                }
                let fragments = parsed.fragments.get(id)?;
                if fragments.len() != 1 {
                    return None;
                }
                match &fragments[0].body {
                    FragmentBody::Literal(bytes) => output.extend_from_slice(bytes),
                    FragmentBody::Composite(children) => {
                        output.extend(expand_children(children, parsed, visiting)?)
                    }
                }
                visiting.remove(id);
            }
        }
    }
    Some(output)
}

fn source_range(source: &[u8], range: LineRange) -> Option<Vec<u8>> {
    let starts = std::iter::once(0)
        .chain(
            source
                .iter()
                .enumerate()
                .filter_map(|(index, byte)| (*byte == b'\n').then_some(index + 1)),
        )
        .collect::<Vec<_>>();
    let start = *starts.get(range.first - 1)?;
    let end = if range.last < starts.len() {
        starts[range.last]
    } else if range.last == starts.len() && source.ends_with(b"\n") {
        source.len()
    } else {
        return None;
    };
    Some(source[start..end].to_vec())
}

fn source_line_count(source: &[u8]) -> usize {
    source.iter().filter(|byte| **byte == b'\n').count()
        + usize::from(!source.is_empty() && !source.ends_with(b"\n"))
}

fn all_children(parsed: &ParsedBook) -> Vec<(&str, Option<&str>, &[Child])> {
    let mut result = Vec::new();
    for roots in parsed.roots.values() {
        for root in roots {
            result.push((
                root.id.as_str(),
                Some(root.id.as_str()),
                root.children.as_slice(),
            ));
        }
    }
    for fragments in parsed.fragments.values() {
        for fragment in fragments {
            if let FragmentBody::Composite(children) = &fragment.body {
                result.push((fragment.id.as_str(), None, children.as_slice()));
            }
        }
    }
    result
}

fn root_for<'a>(parsed: &'a ParsedBook, fragment: &'a Fragment) -> Option<&'a str> {
    let mut parent = fragment.parent.as_str();
    let mut seen = BTreeSet::new();
    while seen.insert(parent) {
        if parsed.roots.contains_key(parent) {
            return Some(parent);
        }
        parent = parsed.fragments.get(parent)?.first()?.parent.as_str();
    }
    None
}

fn coverage(parsed: &ParsedBook, final_: bool) -> Coverage {
    let mut resolved_lines = 0;
    let mut deferred_lines = 0;
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        for child in &roots[0].children {
            match child {
                Child::Insert { id, .. } => {
                    if let Some(fragment) = parsed
                        .fragments
                        .get(id)
                        .filter(|items| items.len() == 1)
                        .and_then(|items| items.first())
                    {
                        resolved_lines += fragment.range.last - fragment.range.first + 1;
                    }
                }
                Child::Defer { range, .. } => deferred_lines += range.last - range.first + 1,
            }
        }
    }
    Coverage {
        files: parsed
            .roots
            .values()
            .filter(|roots| roots.len() == 1)
            .count(),
        resolved_lines,
        deferred_lines,
        final_,
    }
}

fn phase_order(phase: &str) -> usize {
    [
        "parse",
        "inventory",
        "identity",
        "reference",
        "graph",
        "coverage",
        "bytes",
        "markdown",
    ]
    .iter()
    .position(|candidate| *candidate == phase)
    .unwrap_or(usize::MAX)
}

use std::collections::{BTreeMap, BTreeSet};

use crate::manifest::{Manifest, OwnershipBlock};
use crate::parser::{self, Child, Fragment, FragmentBody, LineRange, ParsedBook, Root};
use crate::{
    BookSnapshot, Coverage, Diagnostic, RelatedLocation, ReportStatus, Request, Scope,
    SourceLocation, ValidationReport,
};

pub fn validate(snapshot: &BookSnapshot, request: Request) -> ValidationReport {
    let parsed = parser::parse(snapshot);
    let mut diagnostics = parsed.diagnostics.clone();
    if matches!(request.check, crate::Check::Fragments | crate::Check::All) {
        check_inventory(snapshot, &parsed, &mut diagnostics);
        check_identities(&snapshot.manifest, &parsed, &mut diagnostics);
        check_references(
            &snapshot.manifest,
            &parsed,
            &request.scope,
            &mut diagnostics,
        );
        check_ownership(
            &snapshot.manifest,
            &parsed,
            &request.scope,
            &mut diagnostics,
        );
        crate::ledger::check(snapshot, &parsed, &request.scope, &mut diagnostics);
        check_graph(&parsed, &mut diagnostics);
        check_coverage(&parsed, &mut diagnostics);
        let invalid_roots = structurally_invalid_roots(&parsed, &diagnostics);
        check_bytes(snapshot, &parsed, &invalid_roots, &mut diagnostics);
    }
    if matches!(request.check, crate::Check::Markdown | crate::Check::All) {
        crate::markdown::check(snapshot, &parsed, &request.scope, &mut diagnostics);
    }
    diagnostics.sort_by(|left, right| compare_diagnostics(&snapshot.manifest, left, right));
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
    let manifest = &snapshot.manifest;
    for root in manifest.roots() {
        let (id, source, lines) = (root.id(), root.path(), root.lines());
        match snapshot.source_files.get(source) {
            Some(bytes) if source_line_count(bytes) == lines => {}
            Some(bytes) => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!(
                    "authoritative source `{source}` has {} lines; the manifest declares {lines}",
                    source_line_count(bytes)
                ),
                crate::Location {
                    path: source.into(),
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
                    path: source.into(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                Some(id),
            )),
        }
        match parsed.roots.get(id) {
            Some(roots)
                if roots.len() == 1
                    && roots[0].source == source
                    && roots[0].range
                        == (LineRange {
                            first: 1,
                            last: lines,
                        }) => {}
            Some(roots) => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!(
                    "source root `{id}` is duplicated or disagrees with the declared inventory"
                ),
                roots[0].location.clone(),
                None,
                Some(id),
            )),
            None => diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("required source root `{id}` is missing"),
                crate::Location {
                    path: manifest.source_index_path(),
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
        if manifest.root_order(id).is_none() {
            diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("source root `{id}` is not in the declared inventory"),
                roots[0].location.clone(),
                None,
                Some(id),
            ));
        }
    }
    for path in snapshot.source_files.keys() {
        if !manifest.roots().iter().any(|root| root.path() == path) {
            diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!("authoritative source `{path}` is outside the declared corpus"),
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
    check_derived_corpus(snapshot, diagnostics);
}

/// The declared roots against the set the corpus rule derives from the real
/// directory.
///
/// **This is the book's only external witness, and it is the reason the
/// compiled-in corpus could be deleted.** Every other check in this file reads
/// the manifest and the pages — two artifacts the same author wrote in the same
/// commit — so a file left out of both is invisible to all of them. The derived
/// set is produced by walking the crate, so a production file added to the
/// source and forgotten by the book is a finding rather than a silence.
///
/// It is not a complete witness on its own: derivation proves the declared
/// patterns matched, never that the author declared the right ones, and it
/// cannot judge an exception. The manifest schema fixes the pattern floor to
/// `[book].subject` and closes the exception classes, and a repository test
/// compares every book's exceptions against the specification's inventory.
/// Those three together are the control; this function is one of them.
fn check_derived_corpus(snapshot: &BookSnapshot, diagnostics: &mut Vec<Diagnostic>) {
    let manifest = &snapshot.manifest;
    let declared: BTreeSet<&str> = manifest.roots().iter().map(|root| root.path()).collect();
    for path in &snapshot.derived_corpus {
        if !declared.contains(path.as_str()) {
            diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!(
                    "`{path}` is reached by the corpus rule of book `{}` and is declared by no `[[root]]`",
                    manifest.book_id()
                ),
                crate::Location {
                    path: manifest.manifest_path(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                None,
            ));
        }
    }
    for root in manifest.roots() {
        if !snapshot.derived_corpus.contains(root.path()) {
            diagnostics.push(Diagnostic::new(
                "F006",
                "inventory",
                format!(
                    "declared root `{}` at `{}` is reached by no `[corpus] include` pattern and accounted for by no `[[corpus.add]]`",
                    root.id(),
                    root.path()
                ),
                crate::Location {
                    path: manifest.manifest_path(),
                    byte: 0,
                    line: 1,
                    column: 1,
                },
                None,
                Some(root.id()),
            ));
        }
    }
}

fn check_identities(manifest: &Manifest, parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
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
    for (id, mut occurrences) in locations {
        if occurrences.len() > 1 {
            occurrences.sort_by(|left, right| compare_locations(manifest, left, right));
            let root_id = parsed.roots.contains_key(id).then_some(id);
            let related = occurrences[1..]
                .iter()
                .map(|location| RelatedLocation::from_location(location, "duplicate occurrence"))
                .collect();
            diagnostics.push(
                Diagnostic::new(
                    "F001",
                    "identity",
                    format!("fragment or source-root ID `{id}` is duplicated"),
                    occurrences[0].clone(),
                    Some(id),
                    root_id,
                )
                .with_related(related),
            );
        }
    }
}

/// The last chapter position the requested scope covers, in the book's own
/// slice order. `Final` covers every chapter, so it sits one past the last.
fn prefix_end(manifest: &Manifest, scope: &Scope) -> Option<usize> {
    match scope {
        Scope::Through(slice) => Some(slice.chapter_index()),
        Scope::Final => Some(manifest.chapter_count()),
    }
}

fn check_references(
    manifest: &Manifest,
    parsed: &ParsedBook,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let through = prefix_end(manifest, scope);
    let declared_roots = declared_roots(parsed);
    for (parent, root_id, children) in all_children(parsed, &declared_roots) {
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
                    let owner_order = manifest.slice_order(owner);
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

fn check_ownership(
    manifest: &Manifest,
    parsed: &ParsedBook,
    scope: &Scope,
    diagnostics: &mut Vec<Diagnostic>,
) {
    let through = prefix_end(manifest, scope);
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        let root = &roots[0];
        let expected: Vec<&OwnershipBlock> = manifest.blocks_of(&root.id).collect();
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
            let owner_order = manifest.slice_order(block.owner());
            let should_defer = !matches!(scope, Scope::Final)
                && through
                    .zip(owner_order)
                    .is_some_and(|(now, owner)| owner > now);
            if id != block.id()
                || owner != block.owner()
                || range
                    != (LineRange {
                        first: block.first(),
                        last: block.last(),
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
        let mut completed = BTreeSet::new();
        for child in &root.children {
            if let Child::Insert { id, location } = child {
                visit_iterative(
                    id,
                    location,
                    &unique,
                    &mut completed,
                    &mut reached,
                    &mut reported_cycles,
                    diagnostics,
                    root,
                );
            }
        }
    }
    let declared_roots = declared_roots(parsed);
    for (id, fragments) in &parsed.fragments {
        if fragments.len() == 1 {
            let fragment = &fragments[0];
            let observed = reached.get(id);
            let expected = declared_roots.get(id).and_then(|root| root.as_deref());
            let wrong = observed.is_none_or(BTreeSet::is_empty)
                || observed.is_some_and(|roots| {
                    roots.len() != 1 || expected.is_none_or(|root| !roots.contains(root))
                });
            if wrong {
                let observed_text = observed.map_or_else(
                    || "<none>".into(),
                    |roots| roots.iter().cloned().collect::<Vec<_>>().join(", "),
                );
                let expected_text = expected.unwrap_or("<none>");
                let mut related = Vec::new();
                if let Some(observed) = observed {
                    for root_id in observed {
                        if let Some(root) = parsed
                            .roots
                            .get(root_id)
                            .filter(|items| items.len() == 1)
                            .and_then(|items| items.first())
                        {
                            if let Some(location) =
                                root.children.iter().find_map(|child| match child {
                                    Child::Insert {
                                        id: child_id,
                                        location,
                                    } if child_id == id => Some(location),
                                    _ => None,
                                })
                            {
                                related.push(RelatedLocation::from_location(
                                    location,
                                    format!("observed from {root_id}"),
                                ));
                            }
                        }
                    }
                }
                related.sort();
                diagnostics.push(
                    Diagnostic::new(
                        "F005",
                        "graph",
                        format!(
                            "fragment `{id}` reachability differs: expected `{expected_text}`, observed `{observed_text}`"
                        ),
                        fragment.location.clone(),
                        Some(id),
                        expected,
                    )
                    .with_related(related),
                );
            }
        }
    }
}

fn check_coverage(parsed: &ParsedBook, diagnostics: &mut Vec<Diagnostic>) {
    let declared_roots = declared_roots(parsed);
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        check_child_partition(
            parsed,
            &roots[0].id,
            &roots[0].source,
            roots[0].range,
            &roots[0].children,
            None,
            Some(&roots[0].id),
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
                declared_roots
                    .get(&fragment.id)
                    .and_then(|root| root.as_deref()),
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
    root_id: Option<&str>,
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
                        root_id,
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
                root_id,
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
            root_id,
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn visit_iterative<'a>(
    start: &'a str,
    start_location: &'a crate::Location,
    unique: &BTreeMap<&'a str, &'a Fragment>,
    completed: &mut BTreeSet<&'a str>,
    reached: &mut BTreeMap<String, BTreeSet<String>>,
    reported_cycles: &mut BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
    root: &Root,
) {
    let Some(fragment) = unique.get(start) else {
        return;
    };
    let mut frames = vec![(start, fragment, 0_usize, start_location)];
    let mut active = BTreeMap::from([(start, 0_usize)]);
    reached
        .entry(start.into())
        .or_default()
        .insert(root.id.clone());

    while let Some((id, fragment, next_child, _)) = frames.last_mut() {
        let FragmentBody::Composite(children) = &fragment.body else {
            completed.insert(id);
            active.remove(id);
            frames.pop();
            continue;
        };
        let Some(child) = children.get(*next_child) else {
            completed.insert(id);
            active.remove(id);
            frames.pop();
            continue;
        };
        *next_child += 1;
        let Child::Insert {
            id: child_id,
            location,
        } = child
        else {
            continue;
        };
        let Some(child_fragment) = unique.get(child_id.as_str()) else {
            continue;
        };
        reached
            .entry(child_id.clone())
            .or_default()
            .insert(root.id.clone());
        if let Some(cycle_start) = active.get(child_id.as_str()).copied() {
            let mut witness: Vec<&str> = frames[cycle_start..]
                .iter()
                .map(|(id, _, _, _)| *id)
                .collect();
            let mut edges: Vec<&crate::Location> = frames[cycle_start + 1..]
                .iter()
                .map(|(_, _, _, incoming)| *incoming)
                .chain(std::iter::once(location))
                .collect();
            let rotation = witness
                .iter()
                .enumerate()
                .min_by_key(|(_, candidate)| *candidate)
                .map_or(0, |(index, _)| index);
            witness.rotate_left(rotation);
            edges.rotate_left(rotation);
            let mut complete_witness = witness.clone();
            complete_witness.push(witness[0]);
            let key = complete_witness.join(" -> ");
            if reported_cycles.insert(key.clone()) {
                let related = edges
                    .iter()
                    .enumerate()
                    .map(|(index, location)| {
                        RelatedLocation::from_location(
                            location,
                            format!(
                                "cycle edge {} -> {}",
                                witness[index],
                                witness[(index + 1) % witness.len()]
                            ),
                        )
                    })
                    .collect();
                diagnostics.push(
                    Diagnostic::new(
                        "F004",
                        "graph",
                        format!("fragment cycle: {key}"),
                        edges[0].clone(),
                        Some(child_id),
                        Some(&root.id),
                    )
                    .with_related(related),
                );
            }
            continue;
        }
        if completed.contains(child_id.as_str()) {
            continue;
        }
        active.insert(child_id, frames.len());
        frames.push((child_id, child_fragment, 0, location));
    }
}

fn check_bytes(
    snapshot: &BookSnapshot,
    parsed: &ParsedBook,
    invalid_roots: &BTreeSet<String>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for roots in parsed.roots.values().filter(|roots| roots.len() == 1) {
        let root = &roots[0];
        if invalid_roots.contains(&root.id) {
            continue;
        }
        let expected = snapshot
            .source_files
            .get(&root.source)
            .and_then(|bytes| source_range(bytes, root.range));
        let source_bytes = snapshot.source_files.get(&root.source);
        if source_bytes.is_none() {
            continue;
        }
        let has_defer = root
            .children
            .iter()
            .any(|child| matches!(child, Child::Defer { .. }));
        if has_defer {
            for fragment in reachable_literals(&root.children, parsed) {
                let FragmentBody::Literal(actual) = &fragment.body else {
                    continue;
                };
                let expected = source_bytes.and_then(|bytes| source_range(bytes, fragment.range));
                if expected.as_deref() != Some(actual.as_slice()) {
                    diagnostics.push(byte_diagnostic(
                        parsed,
                        root,
                        fragment,
                        source_bytes.map(Vec::as_slice),
                        expected.as_deref().unwrap_or_default(),
                        actual,
                        fragment.range,
                    ));
                    break;
                }
            }
            continue;
        }
        let limit = expected
            .as_ref()
            .map_or(1, |bytes| bytes.len().saturating_add(1));
        if let Some(actual) = expand_children(&root.children, parsed, limit) {
            let expected = expected.unwrap_or_default();
            if expected != actual {
                let mismatch = first_mismatch(&expected, &actual);
                if let Some(fragment) = emitting_fragment_at(&root.children, parsed, mismatch) {
                    diagnostics.push(byte_diagnostic(
                        parsed,
                        root,
                        fragment,
                        source_bytes.map(Vec::as_slice),
                        &expected,
                        &actual,
                        root.range,
                    ));
                }
            }
        }
    }
}

fn byte_diagnostic(
    parsed: &ParsedBook,
    root: &Root,
    fragment: &Fragment,
    source_bytes: Option<&[u8]>,
    expected: &[u8],
    actual: &[u8],
    compared_range: LineRange,
) -> Diagnostic {
    let mismatch = first_mismatch(expected, actual);
    let source_offset = source_bytes
        .and_then(|bytes| source_range_bounds(bytes, compared_range))
        .map_or(mismatch, |(start, _)| start.saturating_add(mismatch));
    let source_line = source_bytes.map_or(compared_range.first, |bytes| {
        source_line_at(bytes, source_offset)
    });
    let path = fragment_path(parsed, fragment);
    Diagnostic::new(
        "F008",
        "bytes",
        format!(
            "expanded bytes differ from authoritative source at source byte {source_offset}, line {source_line}: expected {}, actual {}; emitting fragment `{}`; owner `{}`; path {}",
            byte_evidence(expected.get(mismatch)),
            byte_evidence(actual.get(mismatch)),
            fragment.id,
            fragment.owner,
            path.join(" -> ")
        ),
        fragment.location.clone(),
        Some(&fragment.id),
        Some(&root.id),
    )
    .with_source(SourceLocation {
        path: root.source.clone(),
        byte: source_offset,
        line: source_line,
    })
    .with_related(vec![RelatedLocation::from_location(
        &root.location,
        "root insertion",
    )])
}

fn reachable_literals<'a>(children: &'a [Child], parsed: &'a ParsedBook) -> Vec<&'a Fragment> {
    let mut literals = Vec::new();
    let mut stack: Vec<&Child> = children.iter().rev().collect();
    while let Some(child) = stack.pop() {
        let Child::Insert { id, .. } = child else {
            continue;
        };
        let Some(fragment) = parsed
            .fragments
            .get(id)
            .filter(|items| items.len() == 1)
            .and_then(|items| items.first())
        else {
            continue;
        };
        match &fragment.body {
            FragmentBody::Literal(_) => literals.push(fragment),
            FragmentBody::Composite(children) => stack.extend(children.iter().rev()),
        }
    }
    literals
}

fn emitting_fragment_at<'a>(
    children: &'a [Child],
    parsed: &'a ParsedBook,
    target: usize,
) -> Option<&'a Fragment> {
    let literals = reachable_literals(children, parsed);
    let mut offset = 0_usize;
    for fragment in &literals {
        let FragmentBody::Literal(bytes) = &fragment.body else {
            continue;
        };
        if target < offset.saturating_add(bytes.len()) {
            return Some(fragment);
        }
        offset = offset.saturating_add(bytes.len());
    }
    literals.last().copied()
}

fn expand_children(children: &[Child], parsed: &ParsedBook, limit: usize) -> Option<Vec<u8>> {
    let mut output = Vec::new();
    let mut frames = vec![(children, 0_usize, None::<&str>)];
    let mut visiting = BTreeSet::new();
    while let Some((children, next_child, fragment_id)) = frames.last_mut() {
        let Some(child) = children.get(*next_child) else {
            if let Some(id) = fragment_id {
                visiting.remove(id);
            }
            frames.pop();
            continue;
        };
        *next_child += 1;
        let Child::Insert { id, .. } = child else {
            continue;
        };
        if !visiting.insert(id.as_str()) {
            return None;
        }
        let fragment = parsed.fragments.get(id)?.first()?;
        if parsed.fragments.get(id)?.len() != 1 {
            return None;
        }
        match &fragment.body {
            FragmentBody::Literal(bytes) => {
                let remaining = limit.saturating_sub(output.len());
                output.extend_from_slice(&bytes[..bytes.len().min(remaining)]);
                visiting.remove(id.as_str());
                if output.len() >= limit {
                    break;
                }
            }
            FragmentBody::Composite(children) => {
                frames.push((children, 0, Some(id.as_str())));
            }
        }
    }
    Some(output)
}

fn source_range(source: &[u8], range: LineRange) -> Option<Vec<u8>> {
    let (start, end) = source_range_bounds(source, range)?;
    Some(source[start..end].to_vec())
}

fn source_range_bounds(source: &[u8], range: LineRange) -> Option<(usize, usize)> {
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
    Some((start, end))
}

fn first_mismatch(expected: &[u8], actual: &[u8]) -> usize {
    expected
        .iter()
        .zip(actual)
        .position(|(expected, actual)| expected != actual)
        .unwrap_or_else(|| expected.len().min(actual.len()))
}

fn byte_evidence(byte: Option<&u8>) -> String {
    byte.map_or_else(|| "EOF".into(), |byte| format!("0x{byte:02x}"))
}

fn source_line_at(source: &[u8], byte: usize) -> usize {
    source[..byte.min(source.len())]
        .iter()
        .filter(|candidate| **candidate == b'\n')
        .count()
        + 1
}

fn fragment_path<'a>(parsed: &'a ParsedBook, fragment: &'a Fragment) -> Vec<&'a str> {
    let mut reversed = vec![fragment.id.as_str()];
    let mut parent = fragment.parent.as_str();
    let mut seen = BTreeSet::new();
    while seen.insert(parent) {
        reversed.push(parent);
        if parsed.roots.contains_key(parent) {
            break;
        }
        let Some(next) = parsed
            .fragments
            .get(parent)
            .filter(|items| items.len() == 1)
            .and_then(|items| items.first())
        else {
            break;
        };
        parent = next.parent.as_str();
    }
    reversed.reverse();
    reversed
}

fn source_line_count(source: &[u8]) -> usize {
    source.iter().filter(|byte| **byte == b'\n').count()
        + usize::from(!source.is_empty() && !source.ends_with(b"\n"))
}

fn all_children<'a>(
    parsed: &'a ParsedBook,
    declared_roots: &'a BTreeMap<String, Option<String>>,
) -> Vec<(&'a str, Option<&'a str>, &'a [Child])> {
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
                result.push((
                    fragment.id.as_str(),
                    declared_roots
                        .get(&fragment.id)
                        .and_then(|root| root.as_deref()),
                    children.as_slice(),
                ));
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

fn declared_roots(parsed: &ParsedBook) -> BTreeMap<String, Option<String>> {
    let mut resolved: BTreeMap<String, Option<String>> = BTreeMap::new();
    for start in parsed.fragments.keys() {
        if resolved.contains_key(start) {
            continue;
        }
        let mut trail = Vec::new();
        let mut positions = BTreeMap::new();
        let mut current = start.as_str();
        let outcome = loop {
            if let Some(root) = parsed.roots.get(current).filter(|items| items.len() == 1) {
                break Some(root[0].id.clone());
            }
            if let Some(root) = resolved.get(current) {
                break root.clone();
            }
            if positions.insert(current.to_owned(), trail.len()).is_some() {
                break None;
            }
            trail.push(current.to_owned());
            let Some(fragment) = parsed
                .fragments
                .get(current)
                .filter(|items| items.len() == 1)
                .and_then(|items| items.first())
            else {
                break None;
            };
            current = fragment.parent.as_str();
        };
        for id in trail {
            resolved.insert(id, outcome.clone());
        }
    }
    resolved
}

fn structurally_invalid_roots(parsed: &ParsedBook, diagnostics: &[Diagnostic]) -> BTreeSet<String> {
    diagnostics
        .iter()
        .filter(|diagnostic| {
            matches!(
                diagnostic.phase.as_str(),
                "parse" | "identity" | "reference" | "graph" | "coverage"
            ) || diagnostic.code == "F006"
        })
        .filter_map(|diagnostic| {
            diagnostic.root_id.clone().or_else(|| {
                let id = diagnostic.fragment_id.as_deref()?;
                if parsed.roots.contains_key(id) {
                    Some(id.to_owned())
                } else {
                    parsed
                        .fragments
                        .get(id)
                        .filter(|items| items.len() == 1)
                        .and_then(|items| items.first())
                        .and_then(|fragment| root_for(parsed, fragment))
                        .map(str::to_owned)
                }
            })
        })
        .collect()
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

fn compare_diagnostics(
    manifest: &Manifest,
    left: &Diagnostic,
    right: &Diagnostic,
) -> std::cmp::Ordering {
    phase_order(&left.phase)
        .cmp(&phase_order(&right.phase))
        .then_with(|| {
            nullable(root_index(manifest, left)).cmp(&nullable(root_index(manifest, right)))
        })
        .then_with(|| {
            nullable(left.source.as_ref().map(|source| source.byte))
                .cmp(&nullable(right.source.as_ref().map(|source| source.byte)))
        })
        .then_with(|| compare_locations(manifest, &left.primary, &right.primary))
        .then_with(|| left.code.cmp(&right.code))
        .then_with(|| compare_optional_text(&left.fragment_id, &right.fragment_id))
        .then_with(|| compare_optional_text(&left.root_id, &right.root_id))
        .then_with(|| left.related.cmp(&right.related))
        .then_with(|| left.message.cmp(&right.message))
}

fn root_index(manifest: &Manifest, diagnostic: &Diagnostic) -> Option<usize> {
    diagnostic
        .root_id
        .as_deref()
        .and_then(|id| manifest.root_order(id))
}

fn nullable<T: Ord>(value: Option<T>) -> (bool, Option<T>) {
    (value.is_none(), value)
}

fn compare_optional_text(left: &Option<String>, right: &Option<String>) -> std::cmp::Ordering {
    nullable(left.as_deref()).cmp(&nullable(right.as_deref()))
}

fn compare_locations(
    manifest: &Manifest,
    left: &crate::Location,
    right: &crate::Location,
) -> std::cmp::Ordering {
    page_key(manifest, &left.path)
        .cmp(&page_key(manifest, &right.path))
        .then_with(|| left.byte.cmp(&right.byte))
}

/// A book page sorts by its position in the manifest's page inventory; every
/// other path sorts after them all, lexically.
fn page_key<'a>(manifest: &Manifest, path: &'a str) -> (bool, usize, &'a str) {
    // This runs inside a sort comparator, so it compares the path's tail
    // against each page's file name rather than building a joined path per
    // page per comparison.
    path.strip_prefix(manifest.book_root())
        .and_then(|tail| tail.strip_prefix('/'))
        .and_then(|file| manifest.pages().iter().position(|page| page.file() == file))
        .map_or((true, usize::MAX, path), |index| (false, index, ""))
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::compare_diagnostics;
    use crate::manifest::Manifest;
    use crate::{Diagnostic, Location, RelatedLocation, SourceLocation};

    const BOOK_ROOT: &str = "docs/walkthroughs/ordinal-fs-tree";

    fn manifest() -> Manifest {
        Manifest::load(
            BOOK_ROOT,
            include_str!("../../../docs/walkthroughs/ordinal-fs-tree/walkthrough.toml"),
        )
        .expect("the relocated book's manifest is schema-valid")
    }

    /// A synthetic diagnostic whose book-specific parts are read off the
    /// manifest rather than spelled here.
    ///
    /// Naming a root id or a source path in this file would put back, in a
    /// test, exactly the compiled-in knowledge of one book that
    /// `validator-fragments-k22` removed from the production path: the
    /// comparator sorts by *position in the declared root order*, so the fixture
    /// only has to be two roots apart, not two particular roots.
    fn diagnostic(manifest: &Manifest) -> Diagnostic {
        let later_root = manifest.roots()[1].id();
        let page = manifest.path(
            manifest
                .lookup_pages()
                .last()
                .expect("a lookup page")
                .file(),
        );
        Diagnostic {
            code: "F007".into(),
            phase: "coverage".into(),
            message: "message".into(),
            primary: Location {
                path: page.clone(),
                byte: 20,
                line: 2,
                column: 1,
            },
            fragment_id: Some("fragment-b".into()),
            root_id: Some(later_root.to_owned()),
            source: Some(SourceLocation {
                path: manifest.roots()[1].path().to_owned(),
                byte: 20,
                line: 2,
            }),
            related: vec![RelatedLocation {
                path: page,
                byte: 30,
                line: 3,
                column: 1,
                label: "related-b".into(),
            }],
            remedy: Some("remedy".into()),
        }
    }

    #[test]
    fn diagnostic_comparison_uses_every_total_order_key_with_nulls_last() {
        let manifest = manifest();
        let base = diagnostic(&manifest);

        let mut earlier = base.clone();
        earlier.phase = "graph".into();
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );

        let mut earlier = base.clone();
        earlier.root_id = Some(manifest.roots()[0].id().to_owned());
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );
        let mut later = base.clone();
        later.root_id = None;
        assert_eq!(
            compare_diagnostics(&manifest, &base, &later),
            Ordering::Less
        );

        let mut earlier = base.clone();
        earlier.source.as_mut().unwrap().byte = 10;
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );
        let mut later = base.clone();
        later.source = None;
        assert_eq!(
            compare_diagnostics(&manifest, &base, &later),
            Ordering::Less
        );

        let mut earlier = base.clone();
        earlier.primary.path = manifest.path(
            manifest
                .chapters()
                .next()
                .expect("the book has a first chapter")
                .file(),
        );
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );
        let mut earlier = base.clone();
        earlier.primary.byte = 10;
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );

        let mut earlier = base.clone();
        earlier.code = "F006".into();
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );
        let mut earlier = base.clone();
        earlier.fragment_id = Some("fragment-a".into());
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );
        let mut later = base.clone();
        later.fragment_id = None;
        assert_eq!(
            compare_diagnostics(&manifest, &base, &later),
            Ordering::Less
        );

        let mut earlier = base.clone();
        earlier.related[0].label = "related-a".into();
        assert_eq!(
            compare_diagnostics(&manifest, &earlier, &base),
            Ordering::Less
        );
        let mut later = base.clone();
        later.message = "z-message".into();
        assert_eq!(
            compare_diagnostics(&manifest, &base, &later),
            Ordering::Less
        );
    }
}

// The **migration** (task-tree-scheme): the one-time, in-place conversion of a task
// tree to the v2 **directory** scheme — `NN-<slug>-k<key>/` node dirs holding a
// `BRIEF.md` + numbered children, leaves `NN-[DONE-]<slug>-k<key>.md`. It runs on
// adoption, from the bare driver's lifecycle transition, and it accepts **two**
// superseded source formats:
//
//   * **v1-flat** (`<dotted>-[<key>]-<slug>[.BRIEF|.DONE].md`) — the primary case;
//   * the **older `NNN-slug/` + `done/`** directory tree — for any grove that never
//     opened during the v1-flat generation, converted straight across with no
//     way-station.
//
// Per task-tree-scheme §5 a task file's first-line header is the **position-free
// handle** `# <slug>-k<key>` (`# … — brief` for a node), so the migration rewrites
// every old header (`# <dotted>-[<key>]-<slug>` or `# NNN-slug`) down to that handle
// while preserving any ` — brief` tail.
//
// **Structure: planning only.** Everything here is pure — it reads the directory
// shape and source bytes and mutates nothing, which is what makes the whole
// order/position/key mapping unit-testable without a repository. Both source
// readers lower to one intermediate — `Logical` (a `LeafId`-shaped id + the
// source path + the old header token) — and a single `render` maps that to
// directory destinations. So the only format-specific code is the two readers;
// the placement rules live in exactly one place.
//
// **Keys.** v1-flat already carries permanent keys → preserved verbatim. The
// `NNN-slug` format has none → keys are assigned fresh in **DFS pre-order** (a
// node's brief takes its key before its children), starting at `1` — deterministic
// and re-runnable, so the fixtures below pin exact output.
//
// `plan_current` is the single authoritative entry point: it lowers a legacy tree
// of either shape *directly* into the current filename-kind grammar, and
// `tree_migration_transaction` is the only mutation owner for that plan — it
// applies the files, removes emptied source directories, and commits through the
// repository seam under one recoverable witness. The separate in-place engine
// this module used to carry (`migrate` / `migrate_on_adoption` and their
// executor) died with the human `grove migrate` verb and the adoption hook of
// `grove do`: the driver calls `tree_lifecycle::transition_to_current` exactly
// once instead of sequencing two migration APIs.

use crate::leaf::{split_prefix, Kind};
use crate::leaf_id::{self, LeafId};
use crate::tree_id::{self, Entry};
use anyhow::{bail, Context, Result};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The detected on-disk format of a `.grove/` tree.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    /// Already v2 — a keyed `…-k<key>.md` leaf or a `NN-<slug>-k<key>/` node dir.
    V2,
    /// v1-flat — a `<dotted>-[<key>]-<slug>` keyed file.
    V1Flat,
    /// Old — a `done/` dir, an `NNN-slug/` node dir, or an `NNN-slug.md` leaf.
    OldNnn,
    /// Neither — only the root brief and/or foreign files; nothing to migrate.
    Empty,
}

/// The computed migration: the detected format and, for a migratable tree, the
/// ordered list of moves (DFS pre-order). Pure — reads the directory shape but
/// mutates nothing — so the order/position/key mapping is unit-testable.
struct Plan {
    format: Format,
    moves: Vec<PlannedMove>,
}

/// The complete deterministic file plan consumed by the session-kind migration
/// transaction. Planning reads source bytes but never mutates the tree.
pub(crate) struct CurrentPlan {
    pub(crate) files: Vec<PlannedFile>,
}

/// One deterministic task-file transformation in the current-format plan.
pub(crate) struct PlannedFile {
    pub(crate) from_rel: PathBuf,
    pub(crate) to_rel: PathBuf,
    pub(crate) body: Vec<u8>,
}

/// A single planned move plus the data to rewrite its first-line header.
#[derive(Clone, Debug, PartialEq, Eq)]
struct PlannedMove {
    /// Old path relative to the grove root.
    from_rel: PathBuf,
    /// New v2 path relative to the grove root (may include node directories).
    to_rel: PathBuf,
    /// The old `# …` header token to match (the v1-flat stem
    /// `<dotted>-[<key>]-<slug>`, or the old `NNN-slug`). Rewritten only if the
    /// first line is `# <old_token>…`; any trailing text (e.g. ` — brief`) is kept.
    old_token: String,
    /// The position-free handle `<slug>-k<key>` replacing the old token.
    new_handle: String,
}

/// One entity lowered from either source format into the common shape the v2
/// renderer consumes: a `LeafId`-shaped id (position vector + permanent key + slug
/// + brief/done flags), the file's absolute source path, and the old header token.
struct Logical {
    id: LeafId,
    from_abs: PathBuf,
    old_token: String,
}

// ---------------------------------------------------------------------------
// planning (pure: reads the directory shape, mutates nothing)

/// Detect the format and, for a migratable tree, compute the ordered move list.
fn plan(grove_root: &Path) -> Result<Plan> {
    let format = detect(grove_root)?;
    let logicals = match format {
        Format::V1Flat => read_v1flat(grove_root)?,
        Format::OldNnn => read_old(grove_root)?,
        Format::V2 | Format::Empty => Vec::new(),
    };
    let moves = render(grove_root, &logicals)?;
    Ok(Plan { format, moves })
}

/// Plan the complete legacy tree directly into the current filename-kind
/// grammar. This is deliberately not wired to the legacy in-place executor:
/// `session-kind-transaction-k94` is the first mutation owner for this plan.
pub(crate) fn plan_current(grove_root: &Path) -> Result<CurrentPlan> {
    if grove_root.join("FORMAT").exists() {
        crate::tree_format::require_current(grove_root)?;
        return Ok(CurrentPlan { files: Vec::new() });
    }

    require_unambiguous_legacy_layout(grove_root)?;
    let legacy_plan = plan(grove_root)?;
    let mut files = Vec::new();
    match legacy_plan.format {
        Format::V2 => {
            plan_legacy_v2_node(grove_root, Path::new(""), &mut files)?;
        }
        Format::V1Flat | Format::OldNnn => {
            plan_pre_v2_current(grove_root, &legacy_plan.moves, &mut files)?;
        }
        Format::Empty => {}
    }
    validate_current_plan(grove_root, &files)?;
    files.sort_by(|left, right| left.from_rel.cmp(&right.from_rel));
    Ok(CurrentPlan { files })
}

fn require_unambiguous_legacy_layout(grove_root: &Path) -> Result<()> {
    let mut v2 = Vec::new();
    let mut v1 = Vec::new();
    let mut old = Vec::new();
    collect_legacy_layout_markers(grove_root, Path::new(""), &mut v2, &mut v1, &mut old)?;

    let format_count =
        usize::from(!v2.is_empty()) + usize::from(!v1.is_empty()) + usize::from(!old.is_empty());
    if format_count > 1 {
        let mut markers = v2.into_iter().chain(v1).chain(old).collect::<Vec<_>>();
        markers.sort();
        bail!(
            "ambiguous legacy tree layout in {}: {}",
            grove_root.display(),
            markers
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    Ok(())
}

fn collect_legacy_layout_markers(
    directory: &Path,
    relative: &Path,
    v2: &mut Vec<PathBuf>,
    v1: &mut Vec<PathBuf>,
    old: &mut Vec<PathBuf>,
) -> Result<()> {
    for entry in
        fs::read_dir(directory).with_context(|| format!("reading {}", directory.display()))?
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "BRIEF.md" {
            continue;
        }
        let path = relative.join(&name);
        let file_type = entry.file_type()?;
        let mut recurse = false;
        if file_type.is_dir() {
            if matches!(tree_id::parse(&name), Some(Entry::Node { .. })) {
                v2.push(path.clone());
                recurse = true;
            } else if name == "done" || split_prefix(&name).is_some() {
                old.push(path.clone());
                recurse = true;
            }
        } else if file_type.is_file() {
            if parse_legacy_v2_leaf(&name) {
                v2.push(path.clone());
            } else if leaf_id::parse(&name).is_some() {
                v1.push(path.clone());
            } else if split_prefix(&name).is_some() {
                old.push(path.clone());
            }
        }
        if recurse {
            collect_legacy_layout_markers(&entry.path(), &path, v2, v1, old)?;
        }
    }
    Ok(())
}

fn validate_current_plan(grove_root: &Path, files: &[PlannedFile]) -> Result<()> {
    let mut destinations = BTreeMap::new();
    for file in files {
        if let Some(previous_source) = destinations.insert(&file.to_rel, &file.from_rel) {
            bail!(
                "duplicate migration destination {} from {} and {}",
                file.to_rel.display(),
                previous_source.display(),
                file.from_rel.display()
            );
        }
        if file.from_rel == file.to_rel {
            continue;
        }

        let destination = grove_root.join(&file.to_rel);
        match fs::symlink_metadata(&destination) {
            Ok(_) => bail!(
                "migration destination collision: {} from {} already exists",
                file.to_rel.display(),
                file.from_rel.display()
            ),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => {
                return Err(error)
                    .with_context(|| format!("checking destination {}", destination.display()));
            }
        }
    }
    Ok(())
}

fn plan_pre_v2_current(
    grove_root: &Path,
    moves: &[PlannedMove],
    files: &mut Vec<PlannedFile>,
) -> Result<()> {
    for planned in moves {
        let source_path = grove_root.join(&planned.from_rel);
        let source = fs::read_to_string(&source_path)
            .with_context(|| format!("reading {}", source_path.display()))?;
        let source = rewrite_body_header(&source, &planned.old_token, &planned.new_handle);
        let (legacy_kind, body) = rewrite_legacy_metadata(&source_path, &source)?;

        let to_rel =
            if planned.to_rel.file_name().and_then(|name| name.to_str()) == Some("BRIEF.md") {
                planned.to_rel.clone()
            } else {
                let file_name = planned
                    .to_rel
                    .file_name()
                    .and_then(|name| name.to_str())
                    .context("legacy plan produced a non-UTF-8 destination")?;
                let legacy = parse_legacy_v2_leaf_parts(file_name).with_context(|| {
                    format!(
                        "legacy plan produced invalid task destination {}",
                        planned.to_rel.display()
                    )
                })?;
                let current_name = Entry::Leaf {
                    position: legacy.position,
                    kind: resolve_standalone_kind(legacy_kind),
                    slug: legacy.slug,
                    key: legacy.key,
                    outcome: legacy.outcome,
                }
                .name();
                planned
                    .to_rel
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .join(current_name)
            };

        files.push(PlannedFile {
            from_rel: planned.from_rel.clone(),
            to_rel,
            body: body.into_bytes(),
        });
    }
    Ok(())
}

fn rewrite_body_header(body: &str, old_token: &str, new_handle: &str) -> String {
    let (first, rest) = body.split_once('\n').unwrap_or((body, ""));
    let Some(new_first) = rewrite_header_line(first, old_token, new_handle) else {
        return body.to_string();
    };
    if body.contains('\n') {
        format!("{new_first}\n{rest}")
    } else {
        new_first
    }
}

fn plan_legacy_v2_node(node: &Path, node_rel: &Path, files: &mut Vec<PlannedFile>) -> Result<()> {
    let mut entries = fs::read_dir(node)
        .with_context(|| format!("reading {}", node.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| entry.file_name());

    let mut has_brief = false;
    let mut child_nodes = Vec::new();
    let mut leaves = Vec::new();

    for entry in entries {
        let name = entry.file_name().to_string_lossy().into_owned();
        let from_rel = node_rel.join(&name);
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            if let Some(Entry::Node { position, .. }) = tree_id::parse(&name) {
                let diagnostic_rel = if entry.path().join("BRIEF.md").is_file() {
                    from_rel.join("BRIEF.md")
                } else {
                    from_rel.clone()
                };
                child_nodes.push((position, entry.path(), from_rel, diagnostic_rel));
            }
            continue;
        }
        if !file_type.is_file() {
            continue;
        }
        if name == "BRIEF.md" {
            has_brief = true;
            if node_rel.as_os_str().is_empty() {
                continue;
            }
            let source = fs::read_to_string(entry.path())
                .with_context(|| format!("reading {}", entry.path().display()))?;
            let (_kind, body) = rewrite_legacy_metadata(&entry.path(), &source)?;
            files.push(PlannedFile {
                from_rel: from_rel.clone(),
                to_rel: from_rel,
                body: body.into_bytes(),
            });
            continue;
        }
        let Some(legacy) = parse_legacy_v2_leaf_parts(&name) else {
            continue;
        };
        let source = fs::read_to_string(entry.path())
            .with_context(|| format!("reading {}", entry.path().display()))?;
        let (kind, body) = rewrite_legacy_metadata(&entry.path(), &source)?;
        leaves.push((
            from_rel,
            legacy,
            kind.unwrap_or(LegacyKind::Known(Kind::Impl)),
            body,
        ));
    }

    leaves.sort_by_key(|(_, legacy, _, _)| legacy.position);
    let mut task_children = leaves
        .iter()
        .map(|(path, legacy, kind, _)| (legacy.position, path.clone(), Some(*kind)))
        .collect::<Vec<_>>();
    task_children.extend(
        child_nodes
            .iter()
            .map(|(position, _, _, diagnostic_rel)| (*position, diagnostic_rel.clone(), None)),
    );
    task_children.sort_by(|left, right| (left.0, &left.1).cmp(&(right.0, &right.1)));
    let mut research_count = 0;
    let mut vendor_candidate = false;
    for (_, _, kind, _) in &leaves {
        match kind {
            LegacyKind::Research => research_count += 1,
            LegacyKind::Known(Kind::CombineResearch) if research_count >= 2 => {
                vendor_candidate = true;
                break;
            }
            LegacyKind::Known(_) => {}
        }
    }
    vendor_candidate &= !has_brief;
    let vendor_pair = vendor_candidate
        && child_nodes.is_empty()
        && leaves.len() == 3
        && matches!(leaves[0].2, LegacyKind::Research)
        && matches!(leaves[1].2, LegacyKind::Research)
        && matches!(leaves[2].2, LegacyKind::Known(Kind::CombineResearch));
    if vendor_candidate && !vendor_pair {
        bail!(
            "ambiguous legacy vendor pair in {}: {}",
            node.display(),
            task_children
                .iter()
                .map(|(_, path, _)| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }

    for (index, (from_rel, legacy, legacy_kind, body)) in leaves.into_iter().enumerate() {
        let kind = match (vendor_pair, index, legacy_kind) {
            (true, 0, LegacyKind::Research) => Kind::ResearchA,
            (true, 1, LegacyKind::Research) => Kind::ResearchB,
            (_, _, LegacyKind::Research) => Kind::ResearchA,
            (_, _, LegacyKind::Known(kind)) => kind,
        };
        let to_name = Entry::Leaf {
            position: legacy.position,
            kind,
            slug: legacy.slug,
            key: legacy.key,
            outcome: legacy.outcome,
        }
        .name();
        files.push(PlannedFile {
            from_rel,
            to_rel: node_rel.join(to_name),
            body: body.into_bytes(),
        });
    }

    for (_, path, rel, _) in child_nodes {
        plan_legacy_v2_node(&path, &rel, files)?;
    }
    Ok(())
}

struct LegacyV2Leaf {
    position: u32,
    slug: String,
    key: u32,
    outcome: tree_id::Outcome,
}

fn parse_legacy_v2_leaf_parts(name: &str) -> Option<LegacyV2Leaf> {
    let stem = name.strip_suffix(".md")?;
    let (position, rest) = stem.split_once('-')?;
    if position.is_empty() || !position.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let position = position.parse().ok()?;
    let (outcome, rest) = if let Some(rest) = rest.strip_prefix("DONE-") {
        (tree_id::Outcome::Done, rest)
    } else if let Some(rest) = rest.strip_prefix("ABANDONED-") {
        (tree_id::Outcome::Abandoned, rest)
    } else {
        (tree_id::Outcome::Live, rest)
    };
    let (slug, key) = rest.rsplit_once("-k")?;
    if slug.is_empty() || key.is_empty() || tree_id::validate_slug(slug).is_err() {
        return None;
    }
    Some(LegacyV2Leaf {
        position,
        slug: slug.to_string(),
        key: key.parse().ok()?,
        outcome,
    })
}

#[derive(Clone, Copy)]
enum LegacyKind {
    Known(Kind),
    Research,
}

#[derive(Clone, Copy)]
struct MarkdownFence {
    marker: char,
    length: usize,
}

fn resolve_standalone_kind(kind: Option<LegacyKind>) -> Kind {
    match kind.unwrap_or(LegacyKind::Known(Kind::Impl)) {
        LegacyKind::Known(kind) => kind,
        LegacyKind::Research => Kind::ResearchA,
    }
}

fn rewrite_legacy_metadata(path: &Path, body: &str) -> Result<(Option<LegacyKind>, String)> {
    let mut kind = None;
    let mut rewritten = String::with_capacity(body.len());
    let mut open_fence = None;
    for line in body.split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']);
        if let Some(fence) = open_fence {
            rewritten.push_str(line);
            if closes_markdown_fence(content, fence) {
                open_fence = None;
            }
            continue;
        }
        if let Some(fence) = markdown_fence_start(content) {
            open_fence = Some(fence);
            rewritten.push_str(line);
            continue;
        }

        let up_to_three_space_indented = strip_up_to_three_spaces(content);
        if up_to_three_space_indented != content {
            if let Some(marker) = legacy_routing_marker(up_to_three_space_indented) {
                bail!(
                    "indented legacy routing marker {marker:?} in {}; place task metadata at column zero or preserve examples in a fenced code block",
                    path.display()
                );
            }
        }
        if let Some(value) = content.strip_prefix("**Kind:**") {
            if kind.is_some() {
                bail!("repeated legacy kind marker in {}", path.display());
            }
            let value = value.trim();
            if value.is_empty() {
                bail!("empty legacy kind marker in {}", path.display());
            }
            kind =
                Some(parse_legacy_kind(value).with_context(|| {
                    format!("unknown legacy kind {value:?} in {}", path.display())
                })?);
        } else if content.starts_with("**Harness:**") || content.starts_with("**Producer launch:**")
        {
            continue;
        } else {
            rewritten.push_str(line);
        }
    }
    Ok((kind, rewritten))
}

/// Distinguish an accepted legacy-v2 leaf from interrupted root-init state when
/// its kind-free slug happens to begin with the current `requirements-` prefix.
pub(crate) fn has_explicit_legacy_kind(path: &Path) -> Result<bool> {
    let body = fs::read_to_string(path)
        .with_context(|| format!("reading legacy task candidate {}", path.display()))?;
    Ok(rewrite_legacy_metadata(path, &body)?.0.is_some())
}

fn legacy_routing_marker(line: &str) -> Option<&'static str> {
    ["**Kind:**", "**Harness:**", "**Producer launch:**"]
        .into_iter()
        .find(|marker| line.starts_with(marker))
}

fn strip_up_to_three_spaces(line: &str) -> &str {
    line.strip_prefix("   ")
        .or_else(|| line.strip_prefix("  "))
        .or_else(|| line.strip_prefix(' '))
        .unwrap_or(line)
}

fn markdown_fence_start(line: &str) -> Option<MarkdownFence> {
    let line = strip_up_to_three_spaces(line);
    let marker = line
        .chars()
        .next()
        .filter(|marker| matches!(marker, '`' | '~'))?;
    let length = line
        .chars()
        .take_while(|character| *character == marker)
        .count();
    if length < 3 || (marker == '`' && line[length..].contains('`')) {
        return None;
    }
    Some(MarkdownFence { marker, length })
}

fn closes_markdown_fence(line: &str, fence: MarkdownFence) -> bool {
    let line = strip_up_to_three_spaces(line);
    let length = line
        .chars()
        .take_while(|character| *character == fence.marker)
        .count();
    length >= fence.length && line[length..].trim().is_empty()
}

fn parse_legacy_kind(value: &str) -> Option<LegacyKind> {
    match value {
        "research" => Some(LegacyKind::Research),
        "review-work" => Some(LegacyKind::Known(Kind::ReviewImpl)),
        "integrate-review-work" => Some(LegacyKind::Known(Kind::IntegrateReviewImpl)),
        "finish" => None,
        _ => Kind::parse_read(value).map(LegacyKind::Known),
    }
}

/// Classify a `.grove/` tree by scanning its top level (see [`Format`]). Precedence
/// is V2 > V1Flat > OldNnn: a v2 marker means the tree is already migrated (never
/// touched), then the v1-flat scheme this grove rode, then the old format for an
/// un-opened tree. A v1-flat tree is *flat* (its only directories are foreign), so
/// a directory is only ever a v2 node dir, an old `NNN-slug/` node, or `done/`.
fn detect(grove_root: &Path) -> Result<Format> {
    if grove_root.join("FORMAT").exists() {
        crate::tree_format::require_current(grove_root)?;
        return Ok(Format::V2);
    }
    let mut has_v2 = false;
    let mut has_v1 = false;
    let mut has_old = false;
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "BRIEF.md" {
            continue; // the unkeyed root brief is format-neutral
        }
        let ft = entry.file_type()?;
        if ft.is_dir() {
            if matches!(tree_id::parse(&name), Some(Entry::Node { .. })) {
                has_v2 = true; // a v2 node directory
            } else if name == "done" || split_prefix(&name).is_some() {
                has_old = true; // a `done/` mirror or an old `NNN-slug/` node dir
            }
        } else if ft.is_file() {
            if parse_legacy_v2_leaf(&name) {
                has_v2 = true; // a legacy v2 `…-k<key>.md` leaf
            } else if leaf_id::parse(&name).is_some() {
                has_v1 = true; // a v1-flat `<dotted>-[<key>]-<slug>` keyed file
            } else if name.strip_suffix(".md").and_then(split_prefix).is_some() {
                has_old = true; // an old `NNN-slug.md` leaf
            }
        }
    }
    Ok(if has_v2 {
        Format::V2
    } else if has_v1 {
        Format::V1Flat
    } else if has_old {
        Format::OldNnn
    } else {
        Format::Empty
    })
}

/// Recognise the kind-free leaf spelling produced by the legacy v2 adapter.
/// Without the positive format witness, even a slug beginning with a current
/// kind label remains legacy input.
fn parse_legacy_v2_leaf(name: &str) -> bool {
    parse_legacy_v2_leaf_parts(name).is_some()
}

/// Read a **v1-flat** tree: it is flat, so a single `read_dir` of the grove root,
/// parsing each keyed filename into its `LeafId` (keys preserved). The root brief
/// and foreign files are skipped. Sorted by position vector (DFS pre-order) so the
/// rendered moves are deterministic.
fn read_v1flat(grove_root: &Path) -> Result<Vec<Logical>> {
    let mut logicals = Vec::new();
    for entry in
        fs::read_dir(grove_root).with_context(|| format!("reading {}", grove_root.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue; // v1-flat is flat; any directory is foreign
        }
        let name = entry.file_name().to_string_lossy().into_owned();
        if name == "BRIEF.md" {
            continue; // the unkeyed root brief stays put, never moved
        }
        let Some(id) = leaf_id::parse(&name) else {
            continue; // foreign file
        };
        let old_token = v1flat_token(&id);
        logicals.push(Logical {
            id,
            from_abs: entry.path(),
            old_token,
        });
    }
    logicals.sort_by(|a, b| a.id.position.cmp(&b.id.position));
    Ok(logicals)
}

/// The v1-flat first-line header token for an id: `<dotted-position>-[<key>]-<slug>`
/// (no marker — headers never carry `.BRIEF`/`.DONE`). Matched against the old
/// `# …` header so the rewrite can replace it with the position-free handle.
fn v1flat_token(id: &LeafId) -> String {
    let dotted = id
        .position
        .iter()
        .map(u32::to_string)
        .collect::<Vec<_>>()
        .join(".");
    format!(
        "{}-[{}]-{}",
        dotted,
        id.key.expect("a v1-flat keyed file always has a key"),
        id.slug
    )
}

/// Read an **old `NNN-slug/` + `done/`** tree into the common `Logical` shape:
/// build the unified forest (live + `done/` mirror merged by logical path), then
/// walk it DFS pre-order assigning each entity its position (1-based sibling index)
/// and a fresh key.
fn read_old(grove_root: &Path) -> Result<Vec<Logical>> {
    let forest = collect_level(grove_root, Path::new(""))?;
    let mut logicals = Vec::new();
    let mut next_key = 1u32;
    assign_old(&forest, &[], &mut next_key, &mut logicals);
    Ok(logicals)
}

/// Lower a list of [`Logical`] entities to v2 directory moves. First indexes every
/// node (a brief with a non-empty position) by its position vector to recover its
/// v2 directory name; then, for each entity, walks its ancestor positions through
/// that index to build the destination directory path, and renders the final
/// component (a node's `…/BRIEF.md`, or a leaf file). An entity whose ancestor has
/// no node brief is an orphan (a malformed tree) → a clean bail, never a panic.
fn render(grove_root: &Path, logicals: &[Logical]) -> Result<Vec<PlannedMove>> {
    // The v2 directory name for each node, keyed by its position vector.
    let mut node_dir: BTreeMap<Vec<u32>, String> = BTreeMap::new();
    for l in logicals {
        if l.id.is_brief && !l.id.position.is_empty() {
            node_dir.insert(l.id.position.clone(), node_dir_name(&l.id));
        }
    }

    let mut moves = Vec::with_capacity(logicals.len());
    for l in logicals {
        let pos = &l.id.position;
        if pos.is_empty() {
            continue; // the root brief is never moved (and is excluded upstream)
        }
        // The destination directory: the v2 dir name of each ancestor position
        // (prefixes of length 1..pos.len()).
        let mut dest = PathBuf::new();
        for depth in 1..pos.len() {
            let prefix = &pos[..depth];
            let dir = node_dir.get(prefix).with_context(|| {
                format!(
                    "orphaned entity {} — no node brief at ancestor position {:?}",
                    l.from_abs.display(),
                    prefix
                )
            })?;
            dest.push(dir);
        }

        let key = l.id.key.expect("a migratable entity is always keyed");
        let new_handle = format!("{}-k{}", l.id.slug, key);
        if l.id.is_brief {
            // A node: its directory (recorded above) holding the brief as BRIEF.md.
            let dir = node_dir
                .get(pos)
                .expect("this brief was registered into node_dir above");
            dest.push(dir);
            dest.push("BRIEF.md");
        } else {
            // A leaf file at its per-level position (the last dotted component).
            let per_level = *pos.last().expect("non-empty position");
            // This bounded adapter deliberately renders the pre-session-kind
            // v2 name. session-kind-migration-k27 owns body-marker mapping and
            // installation of the current FORMAT witness.
            let outcome = if l.id.is_done { "DONE-" } else { "" };
            dest.push(format!("{per_level:02}-{outcome}{}-k{key}.md", l.id.slug));
        }

        let from_rel = l
            .from_abs
            .strip_prefix(grove_root)
            .with_context(|| {
                format!(
                    "{} is not under grove root {}",
                    l.from_abs.display(),
                    grove_root.display()
                )
            })?
            .to_path_buf();
        moves.push(PlannedMove {
            from_rel,
            to_rel: dest,
            old_token: l.old_token.clone(),
            new_handle,
        });
    }
    Ok(moves)
}

/// The v2 node directory name for a node id: `NN-<slug>-k<key>` at its per-level
/// position (the last dotted component).
fn node_dir_name(id: &LeafId) -> String {
    Entry::Node {
        position: *id.position.last().expect("a node has a non-empty position"),
        slug: id.slug.clone(),
        key: id.key.expect("a node is always keyed"),
    }
    .name()
}

// --- old `NNN-slug/` + `done/` forest reader ---------------------------------

/// One node/leaf in the **unified** old tree (live and `done/` mirrors merged by
/// logical path). `nnn` is the old zero-padded numeric prefix (the level's sort
/// key); `slug` is the text after `NNN-`.
enum Entity {
    Leaf {
        nnn: u32,
        slug: String,
        done: bool,
        from_abs: PathBuf,
    },
    Node {
        nnn: u32,
        slug: String,
        brief_abs: Option<PathBuf>,
        children: Vec<Entity>,
    },
}

impl Entity {
    fn nnn(&self) -> u32 {
        match self {
            Entity::Leaf { nnn, .. } | Entity::Node { nnn, .. } => *nnn,
        }
    }
}

/// Collect one logical level of the unified old tree. `chain` is the path of
/// `NNN-slug` directory components from the grove root to this level (empty at the
/// root). The level's children are read from both its live directory and its
/// `done/` mirror and merged by `NNN-slug` name, so a partially-retired node's live
/// and retired children land in one subtree. Returns the level's entities by `NNN`.
fn collect_level(grove_root: &Path, chain: &Path) -> Result<Vec<Entity>> {
    let live_dir = if chain.as_os_str().is_empty() {
        grove_root.to_path_buf()
    } else {
        grove_root.join(chain)
    };
    let done_dir = grove_root.join("done").join(chain);

    struct LeafAcc {
        nnn: u32,
        slug: String,
        done: bool,
        from_abs: PathBuf,
    }
    struct NodeAcc {
        nnn: u32,
        slug: String,
    }
    let mut leaves: BTreeMap<String, LeafAcc> = BTreeMap::new();
    let mut nodes: BTreeMap<String, NodeAcc> = BTreeMap::new();

    for (dir, in_done) in [(live_dir.as_path(), false), (done_dir.as_path(), true)] {
        if !dir.is_dir() {
            continue;
        }
        for entry in fs::read_dir(dir).with_context(|| format!("reading {}", dir.display()))? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().into_owned();
            let ft = entry.file_type()?;
            if ft.is_dir() {
                // The top-level `done/` mirror is scanned via `done_dir`; it is
                // never descended as if it were a child node of the live root.
                if !in_done && name == "done" {
                    continue;
                }
                if let Some((nnn, slug)) = split_prefix(&name) {
                    nodes.entry(name.clone()).or_insert(NodeAcc {
                        nnn,
                        slug: slug.to_string(),
                    });
                }
            } else if ft.is_file() {
                if name == "BRIEF.md" {
                    continue; // a node's brief, attached to the node at this level
                }
                if let Some((nnn, slug)) = name.strip_suffix(".md").and_then(split_prefix) {
                    leaves.insert(
                        name.clone(),
                        LeafAcc {
                            nnn,
                            slug: slug.to_string(),
                            done: in_done,
                            from_abs: dir.join(&name),
                        },
                    );
                }
            }
        }
    }

    let mut entities: Vec<Entity> = Vec::new();
    for (_name, l) in leaves {
        entities.push(Entity::Leaf {
            nnn: l.nnn,
            slug: l.slug,
            done: l.done,
            from_abs: l.from_abs,
        });
    }
    for (dirname, n) in nodes {
        let child_chain = chain.join(&dirname);
        let children = collect_level(grove_root, &child_chain)?;
        // A partially-retired node keeps its brief live; a fully-retired node's
        // brief lives in the `done/` mirror. Prefer live, else done, else none.
        let live_brief = grove_root.join(&child_chain).join("BRIEF.md");
        let done_brief = grove_root.join("done").join(&child_chain).join("BRIEF.md");
        let brief_abs = if live_brief.is_file() {
            Some(live_brief)
        } else if done_brief.is_file() {
            Some(done_brief)
        } else {
            None
        };
        entities.push(Entity::Node {
            nnn: n.nnn,
            slug: n.slug,
            brief_abs,
            children,
        });
    }
    entities.sort_by_key(Entity::nnn);
    Ok(entities)
}

/// Walk the unified old tree in DFS pre-order, lowering each entity to a
/// [`Logical`] with its position (1-based sibling index appended to the parent's)
/// and the next fresh key (a node's brief takes its key before its children). A
/// brief-less node consumes a position slot but emits no `Logical` (no file to
/// move); its children then have no v2 directory to land in, which `render` reports
/// as an orphan — the same degenerate case v1 tolerated only because flat needed no
/// parent directory.
fn assign_old(entities: &[Entity], parent_pos: &[u32], next_key: &mut u32, out: &mut Vec<Logical>) {
    for (i, e) in entities.iter().enumerate() {
        let mut pos = parent_pos.to_vec();
        pos.push(i as u32 + 1);
        match e {
            Entity::Leaf {
                nnn,
                slug,
                done,
                from_abs,
            } => {
                let key = take_key(next_key);
                out.push(Logical {
                    id: LeafId {
                        position: pos,
                        key: Some(key),
                        slug: slug.clone(),
                        is_brief: false,
                        is_done: *done,
                    },
                    from_abs: from_abs.clone(),
                    old_token: format!("{nnn:03}-{slug}"),
                });
            }
            Entity::Node {
                nnn,
                slug,
                brief_abs,
                children,
            } => {
                if let Some(brief_abs) = brief_abs {
                    let key = take_key(next_key);
                    out.push(Logical {
                        id: LeafId {
                            position: pos.clone(),
                            key: Some(key),
                            slug: slug.clone(),
                            is_brief: true,
                            is_done: false,
                        },
                        from_abs: brief_abs.clone(),
                        old_token: format!("{nnn:03}-{slug}"),
                    });
                }
                assign_old(children, &pos, next_key, out);
            }
        }
    }
}

/// Consume and return the next permanent key.
fn take_key(next_key: &mut u32) -> u32 {
    let key = *next_key;
    *next_key += 1;
    key
}

// ---------------------------------------------------------------------------
// header rewrite

/// Rewrite a file's first-line `# …` header for the move. Returns the new header
/// line, or `None` to leave the file untouched. The first line must be
/// `# <old_token>…` with the token at a word boundary; the leading token is
/// replaced with `# <new_handle>` and any trailing text (e.g. ` — brief`) is
/// preserved. A hand-edited title that does not start with the old token — or where
/// the token is only a partial prefix of a longer one — is left alone.
fn rewrite_header_line(first_line: &str, old_token: &str, new_handle: &str) -> Option<String> {
    let after_hash = first_line.strip_prefix("# ")?;
    let tail = after_hash.strip_prefix(old_token)?;
    // Reject a partial match: the next char must not continue a slug (`[a-z0-9-]`),
    // or `010-plan` would mis-match `010-planted` and `…-v2` mis-match `…-v2x`.
    if tail
        .chars()
        .next()
        .is_some_and(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return None;
    }
    Some(format!("# {new_handle}{tail}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Lay down a set of grove-root-relative files (empty-ish bodies — `plan` reads
    /// only the directory structure, never file contents).
    fn build(grove_root: &Path, files: &[&str]) {
        for f in files {
            let p = grove_root.join(f);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(&p, b"# stub\n").unwrap();
        }
    }

    fn grove() -> (tempfile::TempDir, PathBuf) {
        let tmp = tempfile::TempDir::new().unwrap();
        let root = tmp.path().join(".grove");
        fs::create_dir_all(&root).unwrap();
        (tmp, root)
    }

    /// The plan's moves as `(from_rel, to_rel)` string pairs, in plan order.
    fn moves_of(plan: &Plan) -> Vec<(String, String)> {
        plan.moves
            .iter()
            .map(|m| {
                (
                    m.from_rel.to_string_lossy().into_owned(),
                    m.to_rel.to_string_lossy().into_owned(),
                )
            })
            .collect()
    }

    fn write(grove_root: &Path, path: &str, body: &str) {
        let path = grove_root.join(path);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body.as_bytes()).unwrap();
    }

    // ---- detect -------------------------------------------------------------

    #[test]
    fn detect_v2_by_keyed_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-plan-k1.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    #[test]
    fn detect_v2_by_done_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-DONE-plan-k1.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    #[test]
    fn detect_v2_by_node_directory() {
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "05-design-k5/BRIEF.md",
                "05-design-k5/01-x-k6.md",
            ],
        );
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    #[test]
    fn detect_v1flat_by_keyed_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V1Flat);
    }

    #[test]
    fn detect_v1flat_by_done_and_brief_markers() {
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "1-[1]-plan.DONE.md", "5-[5]-node.BRIEF.md"],
        );
        assert_eq!(detect(&g).unwrap(), Format::V1Flat);
    }

    #[test]
    fn detect_old_by_nnn_leaf() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "010-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::OldNnn);
    }

    #[test]
    fn detect_old_by_done_dir() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "done/010-plan.md"]);
        assert_eq!(detect(&g).unwrap(), Format::OldNnn);
    }

    #[test]
    fn detect_old_by_node_dir() {
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "010-node/BRIEF.md", "010-node/010-leaf.md"],
        );
        assert_eq!(detect(&g).unwrap(), Format::OldNnn);
    }

    #[test]
    fn detect_empty_when_only_root_brief() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md"]);
        assert_eq!(detect(&g).unwrap(), Format::Empty);
    }

    #[test]
    fn detect_empty_when_only_foreign_files() {
        let (_t, g) = grove();
        build(&g, &["README.md", "notes.txt"]);
        assert_eq!(detect(&g).unwrap(), Format::Empty);
    }

    #[test]
    fn detect_v2_wins_over_other_markers() {
        // Precedence guard: a stray old marker never re-migrates an already-v2 tree.
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-plan-k1.md", "done/010-old.md"]);
        assert_eq!(detect(&g).unwrap(), Format::V2);
    }

    // The pending-transaction refusal that used to be asserted here went with
    // the engine that reached it. The property itself is unchanged and is
    // pinned where it lives — `tree_access`'s own tests own the diagnostic, and
    // `tree_migration_transaction` owns the refusal on the live path.

    #[test]
    fn current_plan_maps_a_legacy_v2_kind_and_removes_only_obsolete_metadata() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(
            &g,
            "01-review-design-notes-k1.md",
            "# review-design-notes-k1\n\n**Kind:** work\n**Harness:** codex\n\
             **Reviews:** earlier-k9\n\
             **Integrates:** combined-k8\n\
             **Producer launch:** {\"producer\":\"earlier-k9\"}\n\n## Goal\nKeep me.\n",
        );
        write(&g, "notes.txt", "foreign\n");

        let plan = plan_current(&g).unwrap();

        assert_eq!(plan.files.len(), 1);
        let file = &plan.files[0];
        assert_eq!(file.from_rel, Path::new("01-review-design-notes-k1.md"));
        assert_eq!(file.to_rel, Path::new("01-impl-review-design-notes-k1.md"));
        assert_eq!(
            file.body,
            b"# review-design-notes-k1\n\n**Reviews:** earlier-k9\n**Integrates:** combined-k8\n\n## Goal\nKeep me.\n"
        );
        assert!(g.join("01-review-design-notes-k1.md").is_file());
        assert!(!g.join("01-impl-review-design-notes-k1.md").exists());
        assert_eq!(
            fs::read_to_string(g.join("notes.txt")).unwrap(),
            "foreign\n"
        );
    }

    #[test]
    fn current_plan_rejects_an_indented_legacy_routing_marker() {
        let (_t, grove_root) = grove();
        write(&grove_root, "BRIEF.md", "# demo — brief\n");
        write(
            &grove_root,
            "01-task-k1.md",
            "# task-k1\n\n  **Kind:** design\n\n## Goal\nKeep me.\n",
        );

        let Err(error) = plan_current(&grove_root) else {
            panic!("indented routing marker was accepted");
        };

        let diagnostic = format!("{error:#}");
        assert!(diagnostic.contains("01-task-k1.md"), "{diagnostic}");
        assert!(diagnostic.contains("indented"), "{diagnostic}");
        assert!(diagnostic.contains("**Kind:**"), "{diagnostic}");
    }

    #[test]
    fn current_plan_preserves_legacy_routing_markers_in_fenced_examples() {
        let (_t, grove_root) = grove();
        write(&grove_root, "BRIEF.md", "# demo — brief\n");
        let body = "# task-k1\n\n```markdown\n**Kind:** design\n**Harness:** codex\n\
                    **Producer launch:** {\"producer\":\"example-k9\"}\n```\n";
        write(&grove_root, "01-task-k1.md", body);

        let plan = plan_current(&grove_root).unwrap();

        assert_eq!(plan.files.len(), 1);
        assert_eq!(plan.files[0].to_rel, Path::new("01-impl-task-k1.md"));
        assert_eq!(plan.files[0].body, body.as_bytes());
    }

    #[test]
    fn current_plan_does_not_treat_an_invalid_backtick_opener_as_a_fence() {
        let (_t, grove_root) = grove();
        write(&grove_root, "BRIEF.md", "# demo — brief\n");
        write(
            &grove_root,
            "01-task-k1.md",
            "# task-k1\n\n```bad`info\n**Kind:** design\n```\n",
        );

        let plan = plan_current(&grove_root).unwrap();

        assert_eq!(plan.files.len(), 1);
        assert_eq!(plan.files[0].to_rel, Path::new("01-design-task-k1.md"));
        assert!(!plan.files[0]
            .body
            .windows(b"**Kind:**".len())
            .any(|window| window == b"**Kind:**"));
    }

    #[test]
    fn current_plan_walks_legacy_v2_nodes_and_preserves_outcomes() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(
            &g,
            "01-feature-k10/BRIEF.md",
            "# feature-k10 — brief\n\n**Kind:** design\n## Goal\nShip.\n",
        );
        write(
            &g,
            "01-feature-k10/01-DONE-build-k11.md",
            "# build-k11\n\n## Goal\nBuilt.\n",
        );
        write(
            &g,
            "01-feature-k10/02-ABANDONED-spike-k12.md",
            "# spike-k12\n\n**Kind:** prototype\n## Goal\nProbe.\n",
        );

        let plan = plan_current(&g).unwrap();

        assert_eq!(
            plan.files
                .iter()
                .map(|file| (
                    file.from_rel.to_string_lossy().into_owned(),
                    file.to_rel.to_string_lossy().into_owned(),
                    String::from_utf8(file.body.clone()).unwrap(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "01-feature-k10/01-DONE-build-k11.md".into(),
                    "01-feature-k10/01-DONE-impl-build-k11.md".into(),
                    "# build-k11\n\n## Goal\nBuilt.\n".into(),
                ),
                (
                    "01-feature-k10/02-ABANDONED-spike-k12.md".into(),
                    "01-feature-k10/02-ABANDONED-prototype-spike-k12.md".into(),
                    "# spike-k12\n\n## Goal\nProbe.\n".into(),
                ),
                (
                    "01-feature-k10/BRIEF.md".into(),
                    "01-feature-k10/BRIEF.md".into(),
                    "# feature-k10 — brief\n\n## Goal\nShip.\n".into(),
                ),
            ]
        );
    }

    #[test]
    fn current_plan_maps_every_accepted_legacy_kind_spelling() {
        let cases = [
            ("requirements", "requirements"),
            ("design", "design"),
            ("planning", "planning"),
            ("prototype", "prototype"),
            ("impl", "impl"),
            ("work", "impl"),
            ("review-requirements", "review-requirements"),
            ("review-design", "review-design"),
            ("review-planning", "review-planning"),
            ("review-prototype", "review-prototype"),
            ("review-impl", "review-impl"),
            ("review-work", "review-impl"),
            (
                "integrate-review-requirements",
                "integrate-review-requirements",
            ),
            ("integrate-review-design", "integrate-review-design"),
            ("integrate-review-planning", "integrate-review-planning"),
            ("integrate-review-prototype", "integrate-review-prototype"),
            ("integrate-review-impl", "integrate-review-impl"),
            ("integrate-review-work", "integrate-review-impl"),
            ("research", "research-a"),
            ("research-a", "research-a"),
            ("research-b", "research-b"),
            ("combine-research", "combine-research"),
        ];
        for (index, (legacy, current)) in cases.iter().enumerate() {
            let (_t, g) = grove();
            write(&g, "BRIEF.md", "# demo — brief\n");
            let position = index + 1;
            write(
                &g,
                &format!("{position:02}-task-k{position}.md"),
                &format!("# task-k{position}\n\n**Kind:** {legacy}\n"),
            );

            let plan = plan_current(&g)
                .unwrap_or_else(|error| panic!("legacy kind {legacy:?} was rejected: {error:#}"));

            assert_eq!(
                plan.files[0].to_rel,
                PathBuf::from(format!("{position:02}-{current}-task-k{position}.md")),
                "legacy kind {legacy:?}"
            );
        }
    }

    #[test]
    fn current_plan_rejects_invalid_legacy_kind_markers_with_the_source_path() {
        let cases = [
            ("**Kind:**\n", "empty legacy kind marker"),
            (
                "**Kind:** impl\n**Kind:** design\n",
                "repeated legacy kind marker",
            ),
            ("**Kind:** sideways\n", "unknown legacy kind"),
            ("**Kind:** finish\n", "unknown legacy kind"),
        ];
        for (body, expected) in cases {
            let (_t, g) = grove();
            write(&g, "BRIEF.md", "# demo — brief\n");
            write(
                &g,
                "01-task-k1.md",
                &format!("# task-k1\n\n{body}## Goal\n"),
            );

            let error = match plan_current(&g) {
                Ok(_) => panic!("invalid marker was accepted: {body:?}"),
                Err(error) => error,
            };
            let error = format!("{error:#}");

            assert!(error.contains(expected), "unexpected error: {error}");
            assert!(error.contains("01-task-k1.md"), "missing path: {error}");
        }
    }

    #[test]
    fn current_plan_assigns_sides_only_to_an_unambiguous_legacy_vendor_pair() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(
            &g,
            "01-vendor-k10/01-DONE-first-k11.md",
            "# first-k11\n\n**Kind:** research\n",
        );
        write(
            &g,
            "01-vendor-k10/02-second-k12.md",
            "# second-k12\n\n**Kind:** research\n",
        );
        write(
            &g,
            "01-vendor-k10/03-ABANDONED-combine-k13.md",
            "# combine-k13\n\n**Kind:** combine-research\n",
        );

        let plan = plan_current(&g).unwrap();

        assert_eq!(
            plan.files
                .iter()
                .map(|file| file.to_rel.to_string_lossy().into_owned())
                .collect::<Vec<_>>(),
            vec![
                "01-vendor-k10/01-DONE-research-a-first-k11.md",
                "01-vendor-k10/02-research-b-second-k12.md",
                "01-vendor-k10/03-ABANDONED-combine-research-combine-k13.md",
            ]
        );
    }

    #[test]
    fn current_plan_rejects_legacy_vendor_pairs_with_extra_task_structure() {
        for extra in ["04-extra-k14.md", "00-extra-k14/BRIEF.md"] {
            let (_t, g) = grove();
            write(&g, "BRIEF.md", "# demo — brief\n");
            write(
                &g,
                "01-vendor-k10/01-first-k11.md",
                "# first-k11\n\n**Kind:** research\n",
            );
            write(
                &g,
                "01-vendor-k10/02-second-k12.md",
                "# second-k12\n\n**Kind:** research\n",
            );
            write(
                &g,
                "01-vendor-k10/03-combine-k13.md",
                "# combine-k13\n\n**Kind:** combine-research\n",
            );
            write(&g, &format!("01-vendor-k10/{extra}"), "# extra-k14\n");

            let error = match plan_current(&g) {
                Ok(_) => panic!("ambiguous vendor pair with {extra} was accepted"),
                Err(error) => format!("{error:#}"),
            };

            assert!(error.contains("ambiguous legacy vendor pair"), "{error}");
            for path in [
                "01-first-k11.md",
                "02-second-k12.md",
                "03-combine-k13.md",
                extra,
            ] {
                assert!(error.contains(path), "missing {path:?} in: {error}");
            }
        }
    }

    #[test]
    fn current_plan_maps_v1_flat_directly_to_kind_filenames() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(
            &g,
            "1-[7]-feature.DONE.md",
            "# 1-[7]-feature\n\n**Kind:** work\n**Harness:** codex\n\
             **Reviews:** design-k6\n\n## Goal\nShip.\n",
        );

        let plan = plan_current(&g).unwrap();

        assert_eq!(plan.files.len(), 1);
        assert_eq!(plan.files[0].from_rel, Path::new("1-[7]-feature.DONE.md"));
        assert_eq!(
            plan.files[0].to_rel,
            Path::new("01-DONE-impl-feature-k7.md")
        );
        assert_eq!(
            plan.files[0].body,
            b"# feature-k7\n\n**Reviews:** design-k6\n\n## Goal\nShip.\n"
        );
    }

    #[test]
    fn current_plan_maps_old_nnn_directly_to_kind_filenames() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(
            &g,
            "010-node/BRIEF.md",
            "# 010-node — brief\n\n**Kind:** design\n## Goal\nGroup.\n",
        );
        write(
            &g,
            "010-node/020-task.md",
            "# 020-task\n\n**Producer launch:** stale\n## Goal\nBuild.\n",
        );

        let plan = plan_current(&g).unwrap();

        assert_eq!(
            plan.files
                .iter()
                .map(|file| (
                    file.from_rel.to_string_lossy().into_owned(),
                    file.to_rel.to_string_lossy().into_owned(),
                    String::from_utf8(file.body.clone()).unwrap(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "010-node/020-task.md".into(),
                    "01-node-k1/01-impl-task-k2.md".into(),
                    "# task-k2\n\n## Goal\nBuild.\n".into(),
                ),
                (
                    "010-node/BRIEF.md".into(),
                    "01-node-k1/BRIEF.md".into(),
                    "# node-k1 — brief\n\n## Goal\nGroup.\n".into(),
                ),
            ]
        );
    }

    #[test]
    fn current_plan_is_a_no_op_for_the_known_format_and_rejects_unknown_witnesses() {
        let (_t, g) = grove();
        write(&g, "FORMAT", "session-kinds-v1\n");
        write(&g, "BRIEF.md", "# current — brief\n");
        write(&g, "01-impl-task-k1.md", "# task-k1\n");

        assert!(plan_current(&g).unwrap().files.is_empty());

        write(&g, "FORMAT", "session-kinds-v99\n");
        let error = match plan_current(&g) {
            Ok(_) => panic!("unknown format witness was accepted"),
            Err(error) => format!("{error:#}"),
        };
        assert!(error.contains("unsupported Grove tree format"), "{error}");
        assert!(error.contains("FORMAT"), "{error}");
    }

    #[test]
    fn current_plan_rejects_an_existing_destination_before_mutation() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(&g, "01-task-k1.md", "# task-k1\n");
        write(
            &g,
            "01-impl-task-k1.md/foreign.txt",
            "must remain untouched\n",
        );

        let error = match plan_current(&g) {
            Ok(_) => panic!("existing destination was accepted"),
            Err(error) => format!("{error:#}"),
        };

        assert!(error.contains("migration destination collision"), "{error}");
        assert!(error.contains("01-task-k1.md"), "{error}");
        assert!(error.contains("01-impl-task-k1.md"), "{error}");
        assert!(g.join("01-task-k1.md").is_file());
        assert!(g.join("01-impl-task-k1.md/foreign.txt").is_file());
    }

    #[test]
    fn current_plan_rejects_mixed_legacy_grammars_with_the_marker_paths() {
        for (left, right) in [
            ("01-task-k1.md", "1-[2]-other.md"),
            ("1-[1]-task.md", "010-other.md"),
            ("01-task-k1.md", "010-other.md"),
            ("01-node-k1/01-task-k2.md", "01-node-k1/010-other.md"),
        ] {
            let (_t, g) = grove();
            write(&g, "BRIEF.md", "# demo — brief\n");
            write(&g, left, "# task\n");
            write(&g, right, "# other\n");

            let error = match plan_current(&g) {
                Ok(_) => panic!("mixed legacy grammars were accepted: {left}, {right}"),
                Err(error) => format!("{error:#}"),
            };

            assert!(error.contains("ambiguous legacy tree layout"), "{error}");
            assert!(error.contains(left), "missing {left:?} in: {error}");
            assert!(error.contains(right), "missing {right:?} in: {error}");
        }
    }

    // ---- plan: v1-flat golden tree (this grove's exact shape) ---------------

    #[test]
    fn plan_v1flat_golden_tree_mirrors_this_grove() {
        // The `refactor-to-archon` grove's real v1-flat shape (snapshot): retired
        // root leaves, fully-retired nodes (5/6/7/9 — brief + all-DONE children),
        // and a partially-retired node (11 — brief + DONE children + live leaves).
        // Keys are preserved verbatim, including node 9's out-of-order keys 30..33
        // (decomposed later, after node 11's keys 23..29).
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "1-[1]-plan.DONE.md",
                "2-[2]-loop-substrate-spike.DONE.md",
                "3-[3]-substrate-decision.DONE.md",
                "4-[4]-substrate-wiring.DONE.md",
                "5-[5]-dotted-decimal-numbering.BRIEF.md",
                "5.1-[6]-id-model.DONE.md",
                "5.2-[7]-read-verbs.DONE.md",
                "5.3-[8]-grow-verbs-renumber.DONE.md",
                "5.4-[9]-lifecycle-verbs.DONE.md",
                "6-[10]-backwards-compat-migration.BRIEF.md",
                "6.1-[11]-verbs-live.DONE.md",
                "6.2-[12]-grove-migrate.DONE.md",
                "6.3-[13]-migrate-on-adoption.DONE.md",
                "7-[14]-global-skill-homebrew-distribution.BRIEF.md",
                "7.1-[15]-embed-skill-and-provision.DONE.md",
                "7.2-[16]-new-scheme-prose.DONE.md",
                "7.3-[17]-homebrew-sole-gesture.DONE.md",
                "7.4-[18]-install-and-flip.DONE.md",
                "7.5-[19]-remove-project-skill-mirrors.DONE.md",
                "8-[20]-shed-tui.DONE.md",
                "9-[21]-shed-inbox-and-install-machinery.BRIEF.md",
                "9.1-[30]-migrate-seeds.DONE.md",
                "9.2-[31]-shed-inbox-grove-meta.DONE.md",
                "9.3-[32]-shed-install-and-status.DONE.md",
                "9.4-[33]-sweep-dead-old-verb-modules.DONE.md",
                "10-[22]-complete-terminate-signal.DONE.md",
                "11-[23]-scheme-v2-directories.BRIEF.md",
                "11.1-[24]-id-grammar.DONE.md",
                "11.2-[25]-read-verbs.DONE.md",
                "11.3-[26]-grow-lifecycle-verbs.DONE.md",
                "11.4-[27]-migrate-v1-to-v2.md",
                "11.5-[28]-prose-and-commit-convention.md",
                "11.6-[29]-install-and-reflip-v2.md",
            ],
        );
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::V1Flat);
        assert_eq!(
            moves_of(&plan),
            vec![
                ("1-[1]-plan.DONE.md".into(), "01-DONE-plan-k1.md".into()),
                (
                    "2-[2]-loop-substrate-spike.DONE.md".into(),
                    "02-DONE-loop-substrate-spike-k2.md".into()
                ),
                (
                    "3-[3]-substrate-decision.DONE.md".into(),
                    "03-DONE-substrate-decision-k3.md".into()
                ),
                (
                    "4-[4]-substrate-wiring.DONE.md".into(),
                    "04-DONE-substrate-wiring-k4.md".into()
                ),
                (
                    "5-[5]-dotted-decimal-numbering.BRIEF.md".into(),
                    "05-dotted-decimal-numbering-k5/BRIEF.md".into()
                ),
                (
                    "5.1-[6]-id-model.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/01-DONE-id-model-k6.md".into()
                ),
                (
                    "5.2-[7]-read-verbs.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/02-DONE-read-verbs-k7.md".into()
                ),
                (
                    "5.3-[8]-grow-verbs-renumber.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/03-DONE-grow-verbs-renumber-k8.md".into()
                ),
                (
                    "5.4-[9]-lifecycle-verbs.DONE.md".into(),
                    "05-dotted-decimal-numbering-k5/04-DONE-lifecycle-verbs-k9.md".into()
                ),
                (
                    "6-[10]-backwards-compat-migration.BRIEF.md".into(),
                    "06-backwards-compat-migration-k10/BRIEF.md".into()
                ),
                (
                    "6.1-[11]-verbs-live.DONE.md".into(),
                    "06-backwards-compat-migration-k10/01-DONE-verbs-live-k11.md".into()
                ),
                (
                    "6.2-[12]-grove-migrate.DONE.md".into(),
                    "06-backwards-compat-migration-k10/02-DONE-grove-migrate-k12.md".into()
                ),
                (
                    "6.3-[13]-migrate-on-adoption.DONE.md".into(),
                    "06-backwards-compat-migration-k10/03-DONE-migrate-on-adoption-k13.md".into()
                ),
                (
                    "7-[14]-global-skill-homebrew-distribution.BRIEF.md".into(),
                    "07-global-skill-homebrew-distribution-k14/BRIEF.md".into()
                ),
                (
                    "7.1-[15]-embed-skill-and-provision.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/01-DONE-embed-skill-and-provision-k15.md".into()
                ),
                (
                    "7.2-[16]-new-scheme-prose.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/02-DONE-new-scheme-prose-k16.md".into()
                ),
                (
                    "7.3-[17]-homebrew-sole-gesture.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/03-DONE-homebrew-sole-gesture-k17.md".into()
                ),
                (
                    "7.4-[18]-install-and-flip.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/04-DONE-install-and-flip-k18.md".into()
                ),
                (
                    "7.5-[19]-remove-project-skill-mirrors.DONE.md".into(),
                    "07-global-skill-homebrew-distribution-k14/05-DONE-remove-project-skill-mirrors-k19.md".into()
                ),
                ("8-[20]-shed-tui.DONE.md".into(), "08-DONE-shed-tui-k20.md".into()),
                (
                    "9-[21]-shed-inbox-and-install-machinery.BRIEF.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/BRIEF.md".into()
                ),
                (
                    "9.1-[30]-migrate-seeds.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/01-DONE-migrate-seeds-k30.md".into()
                ),
                (
                    "9.2-[31]-shed-inbox-grove-meta.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/02-DONE-shed-inbox-grove-meta-k31.md".into()
                ),
                (
                    "9.3-[32]-shed-install-and-status.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/03-DONE-shed-install-and-status-k32.md".into()
                ),
                (
                    "9.4-[33]-sweep-dead-old-verb-modules.DONE.md".into(),
                    "09-shed-inbox-and-install-machinery-k21/04-DONE-sweep-dead-old-verb-modules-k33.md".into()
                ),
                (
                    "10-[22]-complete-terminate-signal.DONE.md".into(),
                    "10-DONE-complete-terminate-signal-k22.md".into()
                ),
                (
                    "11-[23]-scheme-v2-directories.BRIEF.md".into(),
                    "11-scheme-v2-directories-k23/BRIEF.md".into()
                ),
                (
                    "11.1-[24]-id-grammar.DONE.md".into(),
                    "11-scheme-v2-directories-k23/01-DONE-id-grammar-k24.md".into()
                ),
                (
                    "11.2-[25]-read-verbs.DONE.md".into(),
                    "11-scheme-v2-directories-k23/02-DONE-read-verbs-k25.md".into()
                ),
                (
                    "11.3-[26]-grow-lifecycle-verbs.DONE.md".into(),
                    "11-scheme-v2-directories-k23/03-DONE-grow-lifecycle-verbs-k26.md".into()
                ),
                (
                    "11.4-[27]-migrate-v1-to-v2.md".into(),
                    "11-scheme-v2-directories-k23/04-migrate-v1-to-v2-k27.md".into()
                ),
                (
                    "11.5-[28]-prose-and-commit-convention.md".into(),
                    "11-scheme-v2-directories-k23/05-prose-and-commit-convention-k28.md".into()
                ),
                (
                    "11.6-[29]-install-and-reflip-v2.md".into(),
                    "11-scheme-v2-directories-k23/06-install-and-reflip-v2-k29.md".into()
                ),
            ]
        );
    }

    // ---- plan: v1-flat focused cases ----------------------------------------

    #[test]
    fn plan_v1flat_root_brief_stays_put() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md"]);
        let plan = plan(&g).unwrap();
        assert!(
            !plan
                .moves
                .iter()
                .any(|m| m.from_rel == Path::new("BRIEF.md")),
            "root BRIEF.md must not be moved"
        );
    }

    #[test]
    fn plan_v1flat_live_root_leaves_pad_positions_and_keep_keys() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "2-[2]-b.md", "10-[3]-c.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-a.md".into(), "01-a-k1.md".into()),
                ("2-[2]-b.md".into(), "02-b-k2.md".into()),
                ("10-[3]-c.md".into(), "10-c-k3.md".into()),
            ]
        );
    }

    #[test]
    fn plan_v1flat_done_leaf_gets_done_infix() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "2-[2]-b.DONE.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-a.md".into(), "01-a-k1.md".into()),
                ("2-[2]-b.DONE.md".into(), "02-DONE-b-k2.md".into()),
            ]
        );
    }

    #[test]
    fn plan_v1flat_node_brief_becomes_a_directory_and_is_never_done() {
        // A fully-retired node: brief + all-DONE children. The brief becomes a
        // directory (never a `DONE` marker — node done-ness is implicit).
        let (_t, g) = grove();
        build(
            &g,
            &["BRIEF.md", "1-[1]-node.BRIEF.md", "1.1-[2]-child.DONE.md"],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-node.BRIEF.md".into(), "01-node-k1/BRIEF.md".into()),
                (
                    "1.1-[2]-child.DONE.md".into(),
                    "01-node-k1/01-DONE-child-k2.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_v1flat_deeply_nested_node_nests_directories() {
        // A node within a node: directories nest, each carrying its own key.
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "2-[1]-outer.BRIEF.md",
                "2.3-[2]-inner.BRIEF.md",
                "2.3.1-[3]-deep.md",
            ],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("2-[1]-outer.BRIEF.md".into(), "02-outer-k1/BRIEF.md".into()),
                (
                    "2.3-[2]-inner.BRIEF.md".into(),
                    "02-outer-k1/03-inner-k2/BRIEF.md".into()
                ),
                (
                    "2.3.1-[3]-deep.md".into(),
                    "02-outer-k1/03-inner-k2/01-deep-k3.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_v1flat_double_digit_per_level_position_pads_to_two() {
        // Position 1.10 → per-level 10 → `10-…` (two digits, no truncation).
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-node.BRIEF.md", "1.10-[2]-c10.md"]);
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("1-[1]-node.BRIEF.md".into(), "01-node-k1/BRIEF.md".into()),
                ("1.10-[2]-c10.md".into(), "01-node-k1/10-c10-k2.md".into()),
            ]
        );
    }

    #[test]
    fn plan_v1flat_foreign_files_are_not_moved() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "1-[1]-a.md", "README.md", "notes.txt"]);
        let plan = plan(&g).unwrap();
        assert!(
            !plan.moves.iter().any(|m| {
                let f = m.from_rel.to_string_lossy();
                f.contains("README") || f.contains("notes")
            }),
            "foreign files must be left untouched"
        );
    }

    #[test]
    fn plan_already_v2_has_no_moves() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md", "01-a-k1.md", "02-DONE-b-k2.md"]);
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::V2);
        assert!(plan.moves.is_empty());
    }

    #[test]
    fn plan_empty_has_no_moves() {
        let (_t, g) = grove();
        build(&g, &["BRIEF.md"]);
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::Empty);
        assert!(plan.moves.is_empty());
    }

    // ---- plan: old `NNN-slug/` → v2 directly --------------------------------

    #[test]
    fn plan_old_nnn_golden_tree_converts_straight_to_v2() {
        // An un-opened old-format grove: retired root leaves, a fully-retired node
        // (brief in done/), a partially-retired node (brief live + one done child +
        // two live children), trailing live leaves. Keys assigned fresh DFS pre-order.
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "done/010-plan.md",
                "done/020-loop-substrate-spike.md",
                "done/030-substrate-decision.md",
                "done/040-substrate-wiring.md",
                "done/050-dotted-decimal-numbering/BRIEF.md",
                "done/050-dotted-decimal-numbering/010-id-model.md",
                "done/050-dotted-decimal-numbering/020-read-verbs.md",
                "done/050-dotted-decimal-numbering/030-grow-verbs-renumber.md",
                "done/050-dotted-decimal-numbering/040-lifecycle-verbs.md",
                "done/060-backwards-compat-migration/010-verbs-live.md",
                "060-backwards-compat-migration/BRIEF.md",
                "060-backwards-compat-migration/020-grove-migrate.md",
                "060-backwards-compat-migration/030-migrate-on-adoption.md",
                "070-global-skill-homebrew-distribution.md",
                "080-shed-tui.md",
                "090-shed-inbox-and-install-machinery.md",
                "100-complete-terminate-signal.md",
            ],
        );
        let plan = plan(&g).unwrap();
        assert_eq!(plan.format, Format::OldNnn);
        assert_eq!(
            moves_of(&plan),
            vec![
                ("done/010-plan.md".into(), "01-DONE-plan-k1.md".into()),
                (
                    "done/020-loop-substrate-spike.md".into(),
                    "02-DONE-loop-substrate-spike-k2.md".into()
                ),
                (
                    "done/030-substrate-decision.md".into(),
                    "03-DONE-substrate-decision-k3.md".into()
                ),
                (
                    "done/040-substrate-wiring.md".into(),
                    "04-DONE-substrate-wiring-k4.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/BRIEF.md".into(),
                    "05-dotted-decimal-numbering-k5/BRIEF.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/010-id-model.md".into(),
                    "05-dotted-decimal-numbering-k5/01-DONE-id-model-k6.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/020-read-verbs.md".into(),
                    "05-dotted-decimal-numbering-k5/02-DONE-read-verbs-k7.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/030-grow-verbs-renumber.md".into(),
                    "05-dotted-decimal-numbering-k5/03-DONE-grow-verbs-renumber-k8.md".into()
                ),
                (
                    "done/050-dotted-decimal-numbering/040-lifecycle-verbs.md".into(),
                    "05-dotted-decimal-numbering-k5/04-DONE-lifecycle-verbs-k9.md".into()
                ),
                (
                    "060-backwards-compat-migration/BRIEF.md".into(),
                    "06-backwards-compat-migration-k10/BRIEF.md".into()
                ),
                (
                    "done/060-backwards-compat-migration/010-verbs-live.md".into(),
                    "06-backwards-compat-migration-k10/01-DONE-verbs-live-k11.md".into()
                ),
                (
                    "060-backwards-compat-migration/020-grove-migrate.md".into(),
                    "06-backwards-compat-migration-k10/02-grove-migrate-k12.md".into()
                ),
                (
                    "060-backwards-compat-migration/030-migrate-on-adoption.md".into(),
                    "06-backwards-compat-migration-k10/03-migrate-on-adoption-k13.md".into()
                ),
                (
                    "070-global-skill-homebrew-distribution.md".into(),
                    "07-global-skill-homebrew-distribution-k14.md".into()
                ),
                ("080-shed-tui.md".into(), "08-shed-tui-k15.md".into()),
                (
                    "090-shed-inbox-and-install-machinery.md".into(),
                    "09-shed-inbox-and-install-machinery-k16.md".into()
                ),
                (
                    "100-complete-terminate-signal.md".into(),
                    "10-complete-terminate-signal-k17.md".into()
                ),
            ]
        );
    }

    #[test]
    fn plan_old_partially_retired_node_merges_live_and_done_children() {
        let (_t, g) = grove();
        build(
            &g,
            &[
                "BRIEF.md",
                "010-node/BRIEF.md",
                "010-node/020-live-child.md",
                "done/010-node/010-done-child.md",
            ],
        );
        assert_eq!(
            moves_of(&plan(&g).unwrap()),
            vec![
                ("010-node/BRIEF.md".into(), "01-node-k1/BRIEF.md".into()),
                (
                    "done/010-node/010-done-child.md".into(),
                    "01-node-k1/01-DONE-done-child-k2.md".into()
                ),
                (
                    "010-node/020-live-child.md".into(),
                    "01-node-k1/02-live-child-k3.md".into()
                ),
            ]
        );
    }

    // ---- header rewrite -----------------------------------------------------

    #[test]
    fn rewrite_v1flat_leaf_header_to_position_free_handle() {
        assert_eq!(
            rewrite_header_line(
                "# 11.4-[27]-migrate-v1-to-v2",
                "11.4-[27]-migrate-v1-to-v2",
                "migrate-v1-to-v2-k27"
            ),
            Some("# migrate-v1-to-v2-k27".to_string())
        );
    }

    #[test]
    fn rewrite_v1flat_node_header_preserves_brief_tail() {
        assert_eq!(
            rewrite_header_line(
                "# 11-[23]-scheme-v2-directories — brief",
                "11-[23]-scheme-v2-directories",
                "scheme-v2-directories-k23"
            ),
            Some("# scheme-v2-directories-k23 — brief".to_string())
        );
    }

    #[test]
    fn rewrite_old_nnn_header_to_handle() {
        assert_eq!(
            rewrite_header_line("# 010-plan", "010-plan", "plan-k1"),
            Some("# plan-k1".to_string())
        );
    }

    #[test]
    fn rewrite_leaves_hand_edited_titles_alone() {
        assert_eq!(
            rewrite_header_line("# Some hand title", "1-[1]-plan", "plan-k1"),
            None
        );
        assert_eq!(
            rewrite_header_line("not a header", "1-[1]-plan", "plan-k1"),
            None
        );
    }

    #[test]
    fn rewrite_rejects_partial_token_match() {
        // The token must end at a word boundary, or `…-v2` would clobber `…-v2x`.
        assert_eq!(
            rewrite_header_line(
                "# 11.4-[27]-migrate-v1-to-v2x",
                "11.4-[27]-migrate-v1-to-v2",
                "migrate-v1-to-v2-k27"
            ),
            None
        );
    }
}

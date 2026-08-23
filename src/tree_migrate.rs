// The **migration** (task-tree-scheme): the one-time conversion of a task tree to
// the current grammar — `NN-<slug>-k<key>/` node dirs holding a `BRIEF.md` +
// numbered children, leaves `NN-[DONE-]<session-kind>-<slug>-k<key>.md`. It runs
// on adoption, from the bare driver's lifecycle transition.
//
// **One superseded source format is still converted**: a tree already in v2
// directories whose leaves predate filename kinds. Its conversion is a rename
// *inside each directory* — each leaf gains the kind its body's `**Kind:**`
// marker declared — plus the removal of that marker and its siblings from the
// body. Nothing is relocated, so no directory is created or removed.
//
// **Two earlier layouts are recognised and refused**, and the recognition is the
// load-bearing half of having withdrawn them:
//
//   * the original **`NNN-slug/` + `done/`** tree, whose reader was a unified
//     forest walk — the `done/` mirror merged into the live tree by logical path,
//     then fresh keys assigned in DFS pre-order, because that format carries none;
//   * **v1-flat** (`<dotted>-[<key>]-<slug>[.BRIEF|.DONE].md`), whose reader
//     lowered flat keyed files into node directories and rewrote every
//     `# <dotted>-[<key>]-<slug>` header down to the position-free handle.
//
// Both readers are gone. Deleting their *detection* with them would let such a
// tree classify as `Empty`, whereupon migration writes a `FORMAT` witness over a
// tree whose every entry is then foreign and `pick` reports a finished grove. So
// each shape still classifies, and refuses before any mutation, naming the paths
// it found. What is left of the two readers is a private name matcher each, under
// *legacy recognition* below, which read nothing.
//
// **Two grammars meet here, and they are deliberately not one.** Migration's
// *input* is a tree written before the live grammar existed, so every recogniser
// in this module is frozen — a private matcher, checked against no live rule (see
// `is_legacy_slug`). Its *output* is a current-format tree, so the one place a
// name is rendered goes through `task_name::TaskName` and nothing else. The two
// obligations point opposite ways: a recogniser must never narrow, because its
// false negative classifies a real workstream `Empty`; a renderer must always
// track, because its output is read by every verb. Sharing one rule between them
// would satisfy either by breaking the other, so a test holds them together
// instead — `frozen_legacy_slug_rule_still_agrees_with_the_live_grammar`.
//
// One consequence is worth stating where it is decided: a legacy **node
// directory** is the one entry migration never renames, so a hand-typed `5-` on a
// directory passes through and is then refused by the live grammar, which names
// the `mv` that fixes it. Recognising it leniently and letting it halt loudly
// beats not recognising it at all, which loses the classification and with it the
// subtree.
//
// **Structure: planning only.** Everything here is pure — it reads the directory
// shape and source bytes and mutates nothing, which is what makes the mapping
// unit-testable without a repository.
//
// `plan_current` is the single authoritative entry point, and
// `tree_migration_transaction` is the only mutation owner for its plan — it
// applies the files and commits through the repository seam under one recoverable
// witness. The separate in-place engine this module used to carry (`migrate` /
// `migrate_on_adoption` and their executor) died with the human `grove migrate`
// verb and the adoption hook of `grove do`: the driver calls
// `tree_lifecycle::transition_to_current` exactly once instead of sequencing two
// migration APIs.
use crate::leaf::Kind;
use crate::task_name::{Outcome, Parts, Slug, TaskName};
use anyhow::{bail, Context, Result};
use ordinal_fs_tree::{Key, Ordinal};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

/// The detected on-disk format of a `.grove/` tree.
///
/// Two of the four are **classified but refused**. Keeping a refused format in
/// this enum rather than deleting it with its reader is the load-bearing part of
/// having withdrawn it: a legacy tree Grove cannot classify falls through to
/// [`Format::Empty`], and migration then stamps `FORMAT` over a tree whose every
/// entry is thereafter foreign — picking reports a finished grove and the
/// workstream is silently gone. Recognition costs a name matcher; the alternative
/// costs the work.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Format {
    /// Already v2 directories. Migratable when its leaves predate filename kinds:
    /// each leaf gains its kind segment **in place** and its body loses the
    /// obsolete markers. This is the one format still converted.
    V2,
    /// v1-flat — a `<dotted>-[<key>]-<slug>` keyed file. Classified, refused.
    V1Flat,
    /// The original layout — a `done/` dir, an `NNN-slug/` node dir, or an
    /// `NNN-slug.md` leaf. Classified, refused.
    OldNnn,
    /// Neither — only the root brief and/or foreign files; nothing to migrate.
    Empty,
}

/// The complete deterministic file plan consumed by the session-kind migration
/// transaction. Planning reads source bytes but never mutates the tree.
pub(crate) struct CurrentPlan {
    pub(crate) files: Vec<PlannedFile>,
}

/// One deterministic task-file transformation in the current-format plan.
///
/// `from_rel` and `to_rel` always share a parent directory. That is not a
/// coincidence to rely on loosely — it is what is left of migration now that the
/// two formats needing *relocation* are refused, and the migration transaction
/// leans on it (see its note where the directory sweeps used to be).
pub(crate) struct PlannedFile {
    pub(crate) from_rel: PathBuf,
    pub(crate) to_rel: PathBuf,
    pub(crate) body: Vec<u8>,
}

// ---------------------------------------------------------------------------
// planning (pure: reads the directory shape, mutates nothing)

/// Plan the legacy tree directly into the current filename-kind grammar, or
/// refuse it. `tree_migration_transaction` is the only mutation owner for the
/// result.
pub(crate) fn plan_current(grove_root: &Path) -> Result<CurrentPlan> {
    if grove_root.join("FORMAT").exists() {
        crate::tree_format::require_current(grove_root)?;
        return Ok(CurrentPlan { files: Vec::new() });
    }

    require_supported_legacy_layout(grove_root)?;
    let mut files = Vec::new();
    match detect(grove_root)? {
        Format::V2 => {
            plan_legacy_v2_node(grove_root, Path::new(""), &mut files)?;
        }
        // Unreachable: `require_supported_legacy_layout` above refuses both of
        // these with their full path lists. Spelled out rather than folded into a
        // catch-all so that adding a format to `Format` is a compile error here,
        // which is the whole reason the enum is matched exhaustively.
        format @ (Format::V1Flat | Format::OldNnn) => {
            bail!("{}", withdrawn_layout_message(grove_root, format, &[]))
        }
        Format::Empty => {}
    }
    validate_current_plan(grove_root, &files)?;
    files.sort_by(|left, right| left.from_rel.cmp(&right.from_rel));
    Ok(CurrentPlan { files })
}

/// Admit a legacy tree for migration, or refuse it before anything mutates.
///
/// Two refusals, and the order between them is deliberate. **Ambiguity first**: a
/// tree carrying markers of more than one format is refused whatever those
/// formats are, because the diagnostic a reader needs there is the full marker
/// list, not a verdict on one shape. Only an unambiguous tree reaches the second,
/// narrower refusal, which names the one withdrawn layout it is in.
fn require_supported_legacy_layout(grove_root: &Path) -> Result<()> {
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
            join_paths(&markers)
        );
    }
    for (format, mut markers) in [(Format::V1Flat, v1), (Format::OldNnn, old)] {
        if !markers.is_empty() {
            markers.sort();
            bail!("{}", withdrawn_layout_message(grove_root, format, &markers));
        }
    }
    Ok(())
}

/// The refusal for a layout Grove classifies but no longer converts.
///
/// It names the paths when the caller has them, because "this tree is the wrong
/// shape" is unactionable without knowing which entries said so — the same
/// standard the ambiguity refusal above and the unknown-kind refusal below hold
/// themselves to. It also says what *is* still migrated, so the message
/// distinguishes "Grove cannot migrate any more" from "Grove cannot migrate
/// this".
fn withdrawn_layout_message(grove_root: &Path, format: Format, markers: &[PathBuf]) -> String {
    let layout = match format {
        Format::V1Flat => "the v1-flat `<position>-[<key>]-<slug>` layout",
        Format::OldNnn => "the original `NNN-slug/` + `done/` layout",
        Format::V2 | Format::Empty => "a migratable layout",
    };
    let mut message = format!(
        "the task tree in {} is in {layout}, which Grove no longer migrates",
        grove_root.display()
    );
    if !markers.is_empty() {
        message.push_str(&format!(
            "\n  entries in that layout: {}",
            join_paths(markers)
        ));
    }
    message.push_str(
        "\n  nothing has been changed. Trees already in v2 directories still migrate \
         automatically when their leaves predate filename kinds.",
    );
    message
}

fn join_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

// ---------------------------------------------------------------------------
// legacy recognition
//
// Both withdrawn layouts are recognised here and read nowhere. These are the
// whole of what is left of two readers, and they exist so a tree in either shape
// is *named* rather than mistaken for one with no work in it — see [`Format`].
// They are deliberately as accepting as the parsers they replace: a false
// negative loses a workstream, while a false positive costs a refusal an operator
// can see and argue with.

/// Does `name` spell a leaf or node in the original **`NNN-slug`** layout —
/// exactly three ASCII digits, a dash, then a non-empty remainder?
///
/// Applies to a bare directory name (`020-spec`) and, with the `.md` stripped, to
/// a leaf. The layout's other marker, a `done/` directory, is matched by name at
/// the call sites.
fn is_nnn_slug_name(name: &str) -> bool {
    let Some((head, tail)) = name.split_at_checked(3) else {
        return false;
    };
    head.bytes().all(|byte| byte.is_ascii_digit()) && tail.starts_with('-') && tail.len() > 1
}

/// Does `name` spell a file in the **v1-flat** layout —
/// `<dotted-position>-[<key>]-<slug>[.BRIEF|.DONE].md`?
///
/// The root `BRIEF.md` is excluded: it is unkeyed and position-less, legal in
/// every layout, and so says nothing about which one this is. Callers skip it
/// before asking, and rejecting it here keeps the answer true standalone.
fn is_v1_flat_name(name: &str) -> bool {
    if name == "BRIEF.md" {
        return false;
    }
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    // At most one marker; `.BRIEF` and `.DONE` never coincide on one file.
    let stem = stem
        .strip_suffix(".BRIEF")
        .or_else(|| stem.strip_suffix(".DONE"))
        .unwrap_or(stem);

    // The position is the leading `int('.'int)*` run, so the first `-` ends it.
    let Some(dash) = stem.find('-') else {
        return false;
    };
    let (position, rest) = (&stem[..dash], &stem[dash + 1..]);
    if position.is_empty()
        || position
            .split('.')
            .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return false;
    }

    // The key `[<digits>]`, then `-<slug>`, checked against this module's own
    // frozen shape rule rather than against the live grammar — see
    // [`is_withdrawn_layout_slug`].
    let Some(rest) = rest.strip_prefix('[') else {
        return false;
    };
    let Some(close) = rest.find(']') else {
        return false;
    };
    let key = &rest[..close];
    if key.is_empty() || !key.bytes().all(|byte| byte.is_ascii_digit()) {
        return false;
    }
    rest[close + 1..]
        .strip_prefix('-')
        .is_some_and(is_legacy_slug)
}

/// The slug shape a **legacy** name had to satisfy, frozen here.
///
/// It is a copy of the rule [`Slug::new`](crate::task_name::Slug::new) applies
/// today, and copying it is the point rather than an oversight. Every caller is
/// a *recogniser*, asked only of trees that were written before the live grammar
/// existed, and a recogniser's false negative is a workstream classified
/// [`Format::Empty`] and stamped `FORMAT` — the loss [`Format`]'s own doc comment
/// is about. Sharing the live validator would put that outcome one tightening of
/// an unrelated grammar away, and the tightening would look harmless. The live
/// grammar is free to move; this is a statement about what was already written.
///
/// Rendering is the other side of it and does **not** use this: a name migration
/// *writes* goes through [`TaskName`], so the live grammar is the only authority
/// on output. `frozen_legacy_slug_rule_still_agrees_with_the_live_grammar` holds
/// the two together and is what fails on the day they part.
fn is_legacy_slug(slug: &str) -> bool {
    !slug.is_empty()
        && !matches!(slug, "BRIEF" | "DONE" | "ABANDONED")
        && !slug.starts_with('-')
        && !slug.ends_with('-')
        && slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
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
            if legacy_v2_node_position(&name).is_some() {
                v2.push(path.clone());
                recurse = true;
            } else if name == "done" || is_nnn_slug_name(&name) {
                old.push(path.clone());
                recurse = true;
            }
        } else if file_type.is_file() {
            if parse_legacy_v2_leaf(&name) {
                v2.push(path.clone());
            } else if is_v1_flat_name(&name) {
                v1.push(path.clone());
            } else if is_nnn_slug_name(&name) {
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
            if let Some(position) = legacy_v2_node_position(&name) {
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
        // The one place migration *renders* a name, and it renders it through the
        // live grammar rather than through a spelling of its own: what this plan
        // writes is a current-format tree, so the type every other verb reads
        // through is the only authority on how it is spelled. Note what that
        // buys on the input side — the position is re-rendered from the parsed
        // integer, so a legacy `5-` leaf lands canonically padded as `05-`.
        //
        // Fallible because `Slug` is validated on construction and this slug came
        // off disk through `is_legacy_slug`, which is frozen and could one day
        // admit what the live grammar refuses. Unreachable today, and
        // `frozen_legacy_slug_rule_still_agrees_with_the_live_grammar` is what
        // says so; if it ever fires, the operator is told which file rather than
        // handed a name nothing can read back.
        let slug = Slug::new(&legacy.slug)
            .map_err(|error| anyhow::anyhow!("cannot migrate {}: {error}", from_rel.display()))?;
        let to_name = TaskName::Positioned {
            ordinal: Ordinal::new(legacy.position),
            key: Key::new(legacy.key),
            parts: Parts::leaf(legacy.outcome, kind, slug),
        }
        .to_string();
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
    outcome: Outcome,
}

fn parse_legacy_v2_leaf_parts(name: &str) -> Option<LegacyV2Leaf> {
    let stem = name.strip_suffix(".md")?;
    let (position, rest) = stem.split_once('-')?;
    if position.is_empty() || !position.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    let position = position.parse().ok()?;
    let (outcome, rest) = if let Some(rest) = rest.strip_prefix("DONE-") {
        (Outcome::Done, rest)
    } else if let Some(rest) = rest.strip_prefix("ABANDONED-") {
        (Outcome::Abandoned, rest)
    } else {
        (Outcome::Live, rest)
    };
    let (slug, key) = rest.rsplit_once("-k")?;
    if key.is_empty() || !is_legacy_slug(slug) {
        return None;
    }
    Some(LegacyV2Leaf {
        position,
        slug: slug.to_string(),
        key: key.parse().ok()?,
        outcome,
    })
}

/// The position a **legacy v2 node directory** name carries, or `None` if the
/// name is not one.
///
/// Recognition only: a node directory is the one entry migration never renames,
/// so nothing here reaches [`TaskName`]. It is deliberately **lenient on the
/// position** — `5-design-k5` is recognised exactly as `05-design-k5` is —
/// because that is what the reader it replaces accepted, and because the
/// alternative is the [`Format::Empty`] misclassification a false negative
/// costs. A lenient spelling therefore survives migration unchanged and is then
/// refused by the live grammar, which names the canonical spelling and the one
/// `mv` that fixes it (`docs/adr/task-names-are-canonical.md`) — loud, local,
/// and recoverable, where losing the tree's classification is none of the three.
fn legacy_v2_node_position(name: &str) -> Option<u32> {
    // A `.md` suffix declares a leaf, whatever is on disk. This guard and the
    // outcome-infix one below are **redundant** rather than load-bearing — a
    // `.md` tail leaves the key non-numeric, and an uppercase infix leaves the
    // slug outside the character set — and both survived mutation against the
    // differential corpus for exactly that reason. They are kept because they
    // state the two rules a reader would otherwise have to derive, and because a
    // later change to the key or slug rule would make them load-bearing without
    // announcing it.
    if name.ends_with(".md") {
        return None;
    }
    // A trailing `/` is tolerated for exactness with the reader this replaces;
    // every caller here feeds it a `read_dir` file name, which carries none.
    let stem = name.strip_suffix('/').unwrap_or(name);
    let dash = stem.find('-')?;
    let (position, rest) = (&stem[..dash], &stem[dash + 1..]);
    if position.is_empty() || !position.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    // A node directory never carries an outcome infix — its done-ness is the
    // absence of a live leaf beneath it. Redundant today; see above.
    if rest.starts_with("DONE-") || rest.starts_with("ABANDONED-") {
        return None;
    }
    let (slug, key) = rest.rsplit_once("-k")?;
    if key.is_empty() || !key.bytes().all(|byte| byte.is_ascii_digit()) || !is_legacy_slug(slug) {
        return None;
    }
    position.parse().ok()
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
/// touched), then the v1-flat scheme this grove rode, then the original layout —
/// classified last and only so it can be refused by name. A v1-flat tree is *flat*
/// (its only directories are foreign), so a directory is only ever a v2 node dir,
/// an old `NNN-slug/` node, or `done/`.
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
            if legacy_v2_node_position(&name).is_some() {
                has_v2 = true; // a v2 node directory
            } else if name == "done" || is_nnn_slug_name(&name) {
                has_old = true; // a `done/` mirror or an old `NNN-slug/` node dir
            }
        } else if ft.is_file() {
            if parse_legacy_v2_leaf(&name) {
                has_v2 = true; // a legacy v2 `…-k<key>.md` leaf
            } else if is_v1_flat_name(&name) {
                has_v1 = true; // a v1-flat `<dotted>-[<key>]-<slug>` keyed file
            } else if name.strip_suffix(".md").is_some_and(is_nnn_slug_name) {
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

    /// Each **withdrawn** layout is refused by name, and the refusal names every
    /// entry that put the tree in that class.
    ///
    /// Both cases run through one table because the property is the same and the
    /// failure mode of writing them separately is that only one keeps its
    /// entry-naming assertion. Naming the entries is the difference between an
    /// actionable message and a bare verdict: an operator has to find the tree's
    /// legacy entries to do anything about it, and a single stray file is enough
    /// to classify a whole tree.
    #[test]
    fn current_plan_refuses_each_withdrawn_layout_by_name() {
        /// Files to plant, the layout the refusal must name, the entries it
        /// must list.
        type Case = (
            &'static [(&'static str, &'static str)],
            &'static str,
            &'static [&'static str],
        );

        let cases: [Case; 2] = [
            (
                &[
                    ("010-node/BRIEF.md", "# 010-node — brief\n"),
                    ("010-node/020-task.md", "# 020-task\n"),
                ],
                "NNN-slug",
                &["010-node", "020-task.md"],
            ),
            (
                &[
                    ("1-[7]-feature.DONE.md", "# 1-[7]-feature\n"),
                    ("2-[8]-next.md", "# 2-[8]-next\n"),
                ],
                "v1-flat",
                &["1-[7]-feature.DONE.md", "2-[8]-next.md"],
            ),
        ];

        for (files, layout, entries) in cases {
            let (_t, g) = grove();
            write(&g, "BRIEF.md", "# demo — brief\n");
            for (path, body) in files {
                write(&g, path, body);
            }

            let error = match plan_current(&g) {
                Ok(_) => panic!("the {layout} layout must be refused"),
                Err(error) => format!("{error:#}"),
            };

            assert!(
                error.contains("no longer migrates") && error.contains(layout),
                "the refusal must name the layout it withdrew: {error}"
            );
            for entry in entries {
                assert!(
                    error.contains(entry),
                    "the refusal must name {entry:?}: {error}"
                );
            }
            assert!(
                error.contains("still migrate"),
                "the refusal must distinguish itself from a total loss of \
                 migration: {error}"
            );
        }
    }

    /// The refusal writes nothing — the tree is exactly as the operator left it,
    /// which is what makes a re-run after a manual conversion safe, and what
    /// separates this from the `Empty` misclassification the variant survives to
    /// prevent.
    #[test]
    fn refusing_a_withdrawn_layout_writes_nothing() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(&g, "done/010-plan.md", "# 010-plan\n");

        let before = tree_snapshot(&g);
        assert!(plan_current(&g).is_err());

        assert_eq!(tree_snapshot(&g), before);
        assert!(!g.join("FORMAT").exists(), "no format witness is installed");
    }

    /// Every file under `grove_root` with its bytes, sorted — enough to catch a
    /// stray witness, a rewritten body, or a moved entry.
    fn tree_snapshot(grove_root: &Path) -> Vec<(String, Vec<u8>)> {
        fn walk(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
            for entry in fs::read_dir(dir).unwrap() {
                let entry = entry.unwrap();
                let path = entry.path();
                if entry.file_type().unwrap().is_dir() {
                    walk(root, &path, out);
                } else {
                    let relative = path.strip_prefix(root).unwrap().display().to_string();
                    out.push((relative, fs::read(&path).unwrap()));
                }
            }
        }
        let mut out = Vec::new();
        walk(grove_root, grove_root, &mut out);
        out.sort();
        out
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

    // ---- the grammar seam --------------------------------------------------
    //
    // `migration-k36` cut this module off `tree_id`, and the cut runs along a
    // line the module did not have before: **recognition** is frozen here and
    // **rendering** is the live grammar's. The tests below are what hold that
    // line, and each fails in a different direction.

    /// The two slug rules agree today, and this is what says so.
    ///
    /// [`is_legacy_slug`] is a deliberate copy of [`Slug::new`]'s rule rather
    /// than a call to it, for the reason its doc comment gives. A copy that
    /// nothing compares is a copy that drifts, so the comparison is here — over
    /// a corpus that exercises every clause of both, in both directions. It is
    /// **not** a demand that the live grammar stay put: the live grammar may
    /// tighten, and when it does this test fails and someone decides what a
    /// legacy tree spelled that way should do, which is exactly the decision
    /// that must not be made silently.
    #[test]
    fn frozen_legacy_slug_rule_still_agrees_with_the_live_grammar() {
        for slug in [
            "task",
            "add-leaf",
            "k9",
            "a",
            "task-k9",
            "grove-flip-2",
            "",
            "-task",
            "task-",
            "BRIEF",
            "DONE",
            "ABANDONED",
            "Task",
            "task.md",
            "task/child",
            "task_leaf",
            "tâche",
            "task ",
        ] {
            assert_eq!(
                is_legacy_slug(slug),
                Slug::new(slug).is_ok(),
                "the frozen legacy slug rule and the live grammar disagree on \
                 {slug:?}. Neither is automatically wrong — decide what a legacy \
                 tree spelling it that way should do, then move whichever rule \
                 that decision names."
            );
        }
    }

    /// A legacy leaf's position is re-rendered, not passed through.
    ///
    /// The old renderer padded `{:02}` unconditionally and so does [`TaskName`],
    /// which is why this leaf's cut could be a pure refactor at all. Asserted
    /// rather than assumed, because it is the only reason a lenient *leaf*
    /// spelling in a legacy tree cannot survive migration into a tree the live
    /// grammar then refuses.
    #[test]
    fn a_lenient_legacy_leaf_position_is_migrated_to_the_canonical_spelling() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(&g, "5-task-k1.md", "# task-k1\n\n**Kind:** impl\n");

        let plan = plan_current(&g).unwrap();

        assert_eq!(plan.files.len(), 1);
        assert_eq!(plan.files[0].from_rel, Path::new("5-task-k1.md"));
        assert_eq!(plan.files[0].to_rel, Path::new("05-impl-task-k1.md"));
    }

    /// A lenient *node* spelling is recognised and passed through, and then
    /// refused by the live grammar with the `mv` that fixes it.
    ///
    /// Both halves matter and neither is obvious. Migration never renames a
    /// directory — a plan's source and destination share a parent, which the
    /// transaction leans on — so the leniency cannot be normalised away here.
    /// What it must not do instead is go *unrecognised*: that classifies the
    /// tree [`Format::Empty`], stamps `FORMAT` over it, and loses the subtree
    /// under a name every later reader then calls foreign. So it is recognised,
    /// the leaves beneath it migrate, and the one non-canonical name left halts
    /// the tree loudly, naming its own repair
    /// (`docs/adr/task-names-are-canonical.md`).
    #[test]
    fn a_lenient_legacy_node_is_recognised_passed_through_and_then_refused_by_name() {
        let (_t, g) = grove();
        write(&g, "BRIEF.md", "# demo — brief\n");
        write(&g, "5-feature-k10/BRIEF.md", "# feature-k10 — brief\n");
        write(
            &g,
            "5-feature-k10/1-build-k11.md",
            "# build-k11\n\n**Kind:** impl\n",
        );

        assert_eq!(detect(&g).unwrap(), Format::V2);
        let plan = plan_current(&g).unwrap();
        assert_eq!(
            plan.files
                .iter()
                .map(|file| (
                    file.from_rel.to_string_lossy().into_owned(),
                    file.to_rel.to_string_lossy().into_owned(),
                ))
                .collect::<Vec<_>>(),
            vec![
                (
                    "5-feature-k10/1-build-k11.md".to_string(),
                    // The leaf is re-rendered canonically; the directory it sits
                    // in keeps the spelling it had.
                    "5-feature-k10/01-impl-build-k11.md".to_string(),
                ),
                (
                    "5-feature-k10/BRIEF.md".to_string(),
                    "5-feature-k10/BRIEF.md".to_string(),
                ),
            ],
        );

        // The directory keeps its lenient spelling, and the live grammar refuses
        // it by name — the loud half of the trade this test exists to pin.
        let verdict = <TaskName as ordinal_fs_tree::EntryName>::parse(
            "5-feature-k10",
            ordinal_fs_tree::Found::Dir,
        );
        let ordinal_fs_tree::Verdict::Malformed(error) = verdict else {
            panic!("the live grammar accepted a lenient node spelling");
        };
        assert!(
            format!("{error}").contains("05-feature-k10"),
            "the refusal must name the canonical spelling: {error}"
        );
    }

    /// The frozen matchers admit exactly what the withdrawn reader admitted,
    /// over a corpus neither of them was written against.
    ///
    /// This is the claim with teeth in `migration-k36` and the one the compiler
    /// cannot reach: `legacy_v2_node_position` and [`is_legacy_slug`] are
    /// hand-derived from `tree_id`, and this node's own brief warns that a
    /// transcription of that module is a bug. So they are not argued against it,
    /// they are **run** against it — the withdrawn reader is still compiled, so
    /// both sides are live at once and a cross product settles it. The corpus is
    /// generated rather than listed, because a listed corpus is a second copy of
    /// the transcriber's own idea of what matters; this one crosses every
    /// position form, infix, slug, key form and suffix that either grammar
    /// distinguishes, including combinations neither author would think to write.
    ///
    /// It dies with `sweep-k37`, and the evidence has to be spent before then —
    /// which is why it is here rather than deferred to the leaf that deletes the
    /// other side of it.
    #[test]
    fn the_frozen_matchers_admit_exactly_what_the_withdrawn_reader_did() {
        // Slug shapes: this module's frozen rule against the reader's validator.
        let slugs = [
            "task",
            "a",
            "k9",
            "task-k9",
            "a-1",
            "1",
            "-a",
            "a-",
            "a--b",
            "",
            "BRIEF",
            "DONE",
            "ABANDONED",
            "brief",
            "Task",
            "TASK",
            "a.b",
            "a/b",
            "a_b",
            "a b",
            "tâche",
            "a-k",
            "--",
            "-",
            "k",
            "0",
        ];
        for slug in slugs {
            assert_eq!(
                is_legacy_slug(slug),
                crate::tree_id::validate_slug(slug).is_ok(),
                "the frozen slug rule and the withdrawn reader disagree on {slug:?}"
            );
        }

        // Node-directory names: the cross product, against the reader's own
        // node arm. `parse` decides species by the `.md` suffix, so a name
        // wearing one is a leaf to it and must be `None` here whatever else it
        // spells — which is why the suffixes are part of the product.
        let mut checked = 0_usize;
        let mut admitted = 0_usize;
        for position in ["01", "5", "005", "100", "0", "", "1a", "a", "1-2"] {
            for infix in ["", "DONE-", "ABANDONED-", "done-"] {
                for slug in slugs {
                    for key in ["-k1", "-k0", "-k99", "-k", "-kx", "-k1x", "", "-1"] {
                        for suffix in ["", ".md", "/"] {
                            let name = format!("{position}-{infix}{slug}{key}{suffix}");
                            let expected = match crate::tree_id::parse(&name) {
                                Some(crate::tree_id::Entry::Node { position, .. }) => {
                                    Some(position)
                                }
                                _ => None,
                            };
                            assert_eq!(
                                legacy_v2_node_position(&name),
                                expected,
                                "the frozen node matcher and the withdrawn reader \
                                 disagree on {name:?}"
                            );
                            checked += 1;
                            admitted += usize::from(expected.is_some());
                        }
                    }
                }
            }
        }

        // The control, because agreement between two functions that both answer
        // `None` to everything is not evidence. A structural floor rather than an
        // exact count: the product's shape may grow, and a count would then be a
        // second thing to maintain that says nothing more than this does.
        assert!(checked > 1_000, "the corpus collapsed to {checked} names");
        assert!(
            admitted > 20,
            "only {admitted} of {checked} names were admitted as nodes — the \
             corpus no longer exercises the accepting path, so agreement on it \
             means nothing"
        );
    }

    /// The renderer changed hands and wrote the same bytes, held by experiment.
    ///
    /// This leaf's *Done when* is byte-identical migration output, and this is
    /// the one place in the increment where that can be **measured** rather than
    /// argued: the withdrawn renderer is still compiled, so both are live at once
    /// and can be driven over one corpus. It is `reading-k31`'s equivalence-test
    /// finding applied to a renderer instead of a reader, and like that one it
    /// dies with `sweep-k37` — which is the point at which the claim has stopped
    /// being about a change and started being about history.
    #[test]
    fn both_renderers_spell_every_migrated_leaf_identically() {
        for (position, kind, slug, key, outcome) in [
            (1_u32, Kind::Impl, "task", 1_u32, Outcome::Live),
            (5, Kind::Design, "add-leaf", 42, Outcome::Done),
            (99, Kind::ResearchA, "a", 7, Outcome::Abandoned),
            (100, Kind::CombineResearch, "task-k9", 1234, Outcome::Live),
            (0, Kind::Finish, "grove-flip-2", 0, Outcome::Done),
            (
                12,
                Kind::IntegrateReviewImpl,
                "x0",
                u32::MAX,
                Outcome::Abandoned,
            ),
        ] {
            let withdrawn = crate::tree_id::Entry::Leaf {
                position,
                kind,
                slug: slug.to_string(),
                key,
                outcome: match outcome {
                    Outcome::Live => crate::tree_id::Outcome::Live,
                    Outcome::Done => crate::tree_id::Outcome::Done,
                    Outcome::Abandoned => crate::tree_id::Outcome::Abandoned,
                },
            }
            .name();
            let live = TaskName::Positioned {
                ordinal: Ordinal::new(position),
                key: Key::new(key),
                parts: Parts::leaf(outcome, kind, Slug::new(slug).unwrap()),
            }
            .to_string();
            assert_eq!(withdrawn, live);
        }
    }

    /// The node recogniser's own controls: what it must *not* admit.
    ///
    /// A `.md` suffix declares a leaf whatever is on disk, and a directory never
    /// wears an outcome infix — its done-ness is the absence of a live leaf
    /// beneath it. Both were the withdrawn reader's rules and both are load
    /// bearing: admitting either would plan a rename for something that is not a
    /// node, inside a directory walk.
    #[test]
    fn the_node_recogniser_admits_only_what_the_reader_it_replaces_did() {
        for (name, expected) in [
            ("01-feature-k10", Some(1)),
            ("5-feature-k10", Some(5)),
            ("100-feature-k10", Some(100)),
            ("01-feature-k9-k10", Some(1)),
            ("01-feature-k10/", Some(1)),
            ("01-feature-k10.md", None),
            ("01-DONE-feature-k10", None),
            ("01-ABANDONED-feature-k10", None),
            ("feature-k10", None),
            ("01-feature", None),
            ("01-feature-k", None),
            ("01-feature-kx", None),
            ("01--k10", None),
            ("BRIEF.md", None),
            ("done", None),
        ] {
            assert_eq!(legacy_v2_node_position(name), expected, "{name:?}");
        }
    }
}

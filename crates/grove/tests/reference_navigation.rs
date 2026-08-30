use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

mod support;

/// The user-facing documentation surface: the files a reader reaches without
/// opening `src/`, and the only ones this check walks.
///
/// Enumerated rather than globbed, because the claim is about a *surface* a
/// person navigates, not about every Markdown file in the tree. Architecture,
/// decision records, specs, and the provisioned methodology are separately
/// owned; a repo-wide sweep belongs to whoever owns all of them at once.
/// Each entry must exist and must actually contain a relative link, so the
/// check cannot pass by finding nothing to do.
const USER_DOCS: [&str; 5] = [
    "README.md",
    "CHANGELOG.md",
    "docs/USAGE.md",
    "docs/CONFIGURATION.md",
    "docs/RELEASING.md",
];

#[derive(Clone, Copy)]
struct Fence {
    marker: char,
    length: usize,
}

fn github_heading_anchor(heading: &str) -> String {
    let mut anchor = String::new();
    for character in heading.chars() {
        match character {
            ' ' => anchor.push('-'),
            character if character.is_alphanumeric() || matches!(character, '-' | '_') => {
                anchor.extend(character.to_lowercase());
            }
            _ => {}
        }
    }
    anchor
}

fn up_to_three_space_indented(line: &str) -> &str {
    line.strip_prefix("   ")
        .or_else(|| line.strip_prefix("  "))
        .or_else(|| line.strip_prefix(' '))
        .unwrap_or(line)
}

fn fence_start(line: &str) -> Option<Fence> {
    let line = up_to_three_space_indented(line);
    let marker = line
        .chars()
        .next()
        .filter(|marker| matches!(marker, '`' | '~'))?;
    let length = line
        .chars()
        .take_while(|character| *character == marker)
        .count();
    (length >= 3).then_some(Fence { marker, length })
}

fn closes_fence(line: &str, fence: Fence) -> bool {
    let line = up_to_three_space_indented(line);
    let length = line
        .chars()
        .take_while(|character| *character == fence.marker)
        .count();
    length >= fence.length && line[length..].trim().is_empty()
}

fn atx_heading(line: &str) -> Option<&str> {
    let line = up_to_three_space_indented(line);
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    if !(1..=6).contains(&level) {
        return None;
    }
    line[level..].strip_prefix(' ')
}

/// Every generated heading anchor in a document, in document order.
fn markdown_headings(markdown: &str) -> Vec<String> {
    let mut open_fence = None;
    let mut used_anchors = HashSet::new();
    let mut next_suffix_by_base = HashMap::new();
    let mut headings = Vec::new();

    for line in markdown.lines() {
        if open_fence.is_some_and(|fence| closes_fence(line, fence)) {
            open_fence = None;
            continue;
        }
        if open_fence.is_some() {
            continue;
        }
        if let Some(fence) = fence_start(line) {
            open_fence = Some(fence);
            continue;
        }
        let Some(heading) = atx_heading(line) else {
            continue;
        };

        let base_anchor = github_heading_anchor(heading);
        let mut anchor = base_anchor.clone();
        let next_suffix = next_suffix_by_base.entry(base_anchor.clone()).or_insert(1);
        while used_anchors.contains(&anchor) {
            anchor = format!("{base_anchor}-{next_suffix}");
            *next_suffix += 1;
        }
        used_anchors.insert(anchor.clone());
        headings.push(anchor);
    }

    headings
}

/// Every explicit `<a id="…"></a>` anchor in a document.
///
/// `docs/ARCHITECTURE.md` keeps the former decision-record slugs as explicit
/// anchors precisely so a retitled section does not break the citations that
/// name them, so resolving only generated heading anchors would reject exactly
/// the links that were designed to be stable.
fn explicit_anchors(markdown: &str) -> HashSet<String> {
    let mut anchors = HashSet::new();
    for fragment in markdown.split("<a id=").skip(1) {
        let Some(rest) = fragment.strip_prefix('"') else {
            continue;
        };
        if let Some((anchor, _)) = rest.split_once('"') {
            anchors.insert(anchor.to_owned());
        }
    }
    anchors
}

/// Relative link targets and their line numbers, skipping fenced blocks so a
/// worked example cannot be mistaken for a real reference. Absolute URLs and
/// bare fragments are out of scope: the first is not this repository's to
/// verify, and the second resolves within the rendering page.
fn relative_link_targets(markdown: &str) -> Vec<(String, usize)> {
    let mut open_fence = None;
    let mut targets = Vec::new();

    for (line_index, line) in markdown.lines().enumerate() {
        if open_fence.is_some_and(|fence| closes_fence(line, fence)) {
            open_fence = None;
            continue;
        }
        if open_fence.is_some() {
            continue;
        }
        if let Some(fence) = fence_start(line) {
            open_fence = Some(fence);
            continue;
        }

        for candidate in line.split("](").skip(1) {
            let Some((target, _)) = candidate.split_once(')') else {
                continue;
            };
            // A Markdown link may carry a title after the destination.
            let target = target.split_whitespace().next().unwrap_or_default();
            if target.is_empty()
                || target.starts_with('#')
                || target.contains("://")
                || target.starts_with("mailto:")
            {
                continue;
            }
            targets.push((target.to_owned(), line_index + 1));
        }
    }

    targets
}

/// Resolve one relative link against the repository, returning the reason it
/// does not resolve. Kept total and pure so the check can be shown failing.
fn unresolved_reason(repository_root: &Path, source: &str, target: &str) -> Option<String> {
    let (path_part, fragment) = match target.split_once('#') {
        Some((path_part, fragment)) => (path_part, Some(fragment)),
        None => (target, None),
    };

    let source_directory = Path::new(source).parent().unwrap_or(Path::new(""));
    let resolved: PathBuf = if path_part.is_empty() {
        repository_root.join(source)
    } else {
        repository_root.join(source_directory).join(path_part)
    };

    if !resolved.exists() {
        return Some(format!("{} does not exist", resolved.display()));
    }

    let fragment = fragment?;
    if fragment.is_empty() {
        return None;
    }
    if resolved
        .extension()
        .is_none_or(|extension| extension != "md")
    {
        return Some(format!(
            "fragment `#{fragment}` names a non-Markdown target {}",
            resolved.display()
        ));
    }

    let markdown = std::fs::read_to_string(&resolved).ok()?;
    let resolves = markdown_headings(&markdown)
        .iter()
        .any(|anchor| anchor == fragment)
        || explicit_anchors(&markdown).contains(fragment);
    (!resolves).then(|| {
        format!(
            "fragment `#{fragment}` matches no heading or explicit anchor in {}",
            resolved.display()
        )
    })
}

/// The repository root — the surface these documents live on. Walked up to
/// rather than read from `CARGO_MANIFEST_DIR`, which names `crates/grove` since
/// `loop-crate-driver-k22` moved these tests off a root package that no longer
/// exists.
fn repository_root() -> PathBuf {
    support::repo_root()
}

#[test]
fn user_documentation_references_resolve() {
    let root = repository_root();

    for document in USER_DOCS {
        let path = root.join(document);
        let markdown = std::fs::read_to_string(&path)
            .unwrap_or_else(|error| panic!("{document} must be readable: {error}"));
        let targets = relative_link_targets(&markdown);
        assert!(
            !targets.is_empty(),
            "{document} must carry at least one relative reference for this check to mean anything"
        );

        for (target, line_number) in targets {
            if let Some(reason) = unresolved_reason(&root, document, &target) {
                panic!(
                    "{document}:{line_number}: reference `{target}` does not resolve — {reason}"
                );
            }
        }
    }
}

#[test]
fn user_documentation_reference_check_rejects_dangling_targets() {
    let root = repository_root();

    assert!(
        unresolved_reason(&root, "docs/USAGE.md", "CONFIGURATION.md").is_none(),
        "a real sibling document must resolve"
    );
    assert!(
        unresolved_reason(&root, "docs/USAGE.md", "ARCHITECTURE.md#task-kind-taxonomy").is_none(),
        "an explicit `<a id=…>` anchor must resolve"
    );
    assert!(
        unresolved_reason(&root, "docs/USAGE.md", "ARCHITECTURE.md#runtime-flow").is_none(),
        "a generated heading anchor must resolve"
    );
    assert!(
        unresolved_reason(&root, "docs/USAGE.md", "NO-SUCH-DOCUMENT.md").is_some(),
        "a missing file must be reported"
    );
    assert!(
        unresolved_reason(&root, "docs/USAGE.md", "ARCHITECTURE.md#no-such-anchor").is_some(),
        "a missing fragment must be reported"
    );
}

#[test]
fn relative_link_scan_ignores_fenced_examples_and_absolute_urls() {
    let markdown = concat!(
        "See [real](docs/USAGE.md).\n",
        "```text\n",
        "[fenced](docs/NEVER.md)\n",
        "```\n",
        "And [remote](https://example.invalid/x) and [local](#section).\n",
    );

    assert_eq!(
        relative_link_targets(markdown),
        [("docs/USAGE.md".to_owned(), 1)]
    );
}

#[test]
fn github_heading_namespace_includes_subsections_fences_and_deduplication() {
    let headings = markdown_headings(
        "# Guide\n## Same\n~~~markdown\n## Hidden example\n~~~~\n### Same\n  ````text\n## Also hidden\n  `````\n## Same-1\n   ### WDYT?\n####### Not a heading",
    );

    assert_eq!(headings, ["guide", "same", "same-1", "same-1-1", "wdyt"]);
}

// ---------------------------------------------------------------------------
// The repository-wide sweep
//
// [`USER_DOCS`] above enumerates *a surface* — the files a reader reaches
// without opening `src/`. Everything else was left to "whoever owns all of them
// at once", which is what follows: the same resolver, run over every Markdown
// document the repository ships, plus the citations that name a durable record
// by its *kind* rather than by a link.

/// Directories the repository-wide sweep does not enter.
///
/// `.grove/` is the live task tree: process state the finish cycle deletes, and
/// its briefs cite work items by handles that resolve only inside it. The rest
/// is build output and VCS administration.
const UNSWEPT_DIRECTORIES: [&str; 5] = [".grove", ".git", ".jj", "target", ".grove-worktrees"];

/// History, which keeps the past by contract: a CHANGELOG entry describes the
/// system as it stood when it was written, so a record it names by a kind that
/// record has since lost is accurate, not stale.
const HISTORY: &str = "CHANGELOG.md";

/// This file, excluded from the citation scan below because it quotes the
/// mislabelled form it exists to reject — in its own explanation and in its
/// fixtures. Named as a constant so the exclusion is one visible decision
/// rather than a silent filter.
const THIS_FILE: &str = "crates/grove/tests/reference_navigation.rs";

fn collect_files(directory: &Path, prefix: &str, extensions: &[&str], into: &mut Vec<String>) {
    let entries = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} must be readable: {error}", directory.display()));

    for entry in entries {
        let entry = entry.expect("directory entries must be readable");
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };

        if entry
            .file_type()
            .expect("entry type must be readable")
            .is_dir()
        {
            if !UNSWEPT_DIRECTORIES.contains(&name.as_str()) {
                collect_files(&entry.path(), &relative, extensions, into);
            }
        } else if extensions.iter().any(|extension| name.ends_with(extension)) {
            into.push(relative);
        }
    }
}

fn repository_files(extensions: &[&str]) -> Vec<String> {
    let mut paths = Vec::new();
    collect_files(&repository_root(), "", extensions, &mut paths);
    paths.sort();
    paths
}

#[test]
fn every_repository_markdown_reference_resolves() {
    let root = repository_root();
    let documents = repository_files(&[".md"]);

    // The enumeration must reach past the named user surface, or this test is
    // just `user_documentation_references_resolve` under a wider name.
    for expected in [
        "CONTEXT.md",
        "docs/ARCHITECTURE.md",
        "plugins/grove/skills/grove/SKILL.md",
    ] {
        assert!(
            documents.iter().any(|document| document == expected),
            "the repository-wide sweep must reach {expected}: {documents:?}"
        );
    }
    assert!(
        documents
            .iter()
            .any(|document| document.starts_with("docs/adr/")),
        "the sweep must descend into the decision-record set"
    );
    assert!(
        documents
            .iter()
            .any(|document| document.starts_with("plugins/")),
        "the sweep must reach the second bounded context"
    );

    let mut unresolved = Vec::new();
    for document in &documents {
        let markdown = std::fs::read_to_string(root.join(document))
            .unwrap_or_else(|error| panic!("{document} must be readable: {error}"));
        for (target, line_number) in relative_link_targets(&markdown) {
            if let Some(reason) = unresolved_reason(&root, document, &target) {
                unresolved.push(format!("{document}:{line_number}: `{target}` — {reason}"));
            }
        }
    }

    assert!(
        unresolved.is_empty(),
        "repository references that do not resolve:\n  {}",
        unresolved.join("\n  ")
    );
}

// ---------------------------------------------------------------------------
// Citations that name a record by its kind
//
// A relative link fails loudly when its target moves. A citation like
// "ADR *pruning*" fails silently: it names a slug, asserts where that slug
// lives, and keeps reading fine long after the record stopped being an ADR.
// That is exactly what happened here — the former decision-record slugs are now
// explicit anchors in `docs/ARCHITECTURE.md` (see its *Documentation ownership*
// section), so a citation still calling one an ADR sends a reader to a
// `docs/adr/<slug>.md` that does not exist.
//
// The repository's own convention for those slugs is the bare parenthetical —
// `(task-kind-taxonomy)`, used throughout `src/` — so this check does not
// forbid citing them. It forbids **mislabelling the kind**: say "ADR" only of a
// file under `docs/adr/`.

fn slugs_in(directory: &str) -> HashSet<String> {
    let path = repository_root().join(directory);
    std::fs::read_dir(&path)
        .unwrap_or_else(|error| panic!("{directory} must be readable: {error}"))
        .filter_map(|entry| {
            let name = entry.ok()?.file_name().to_string_lossy().into_owned();
            name.strip_suffix(".md").map(str::to_owned)
        })
        .collect()
}

/// Every slug `docs/ARCHITECTURE.md` resolves — generated heading anchors and
/// the explicit `<a id="…">` anchors that carry the former record slugs.
fn architecture_slugs() -> HashSet<String> {
    let markdown = std::fs::read_to_string(repository_root().join("docs/ARCHITECTURE.md"))
        .expect("docs/ARCHITECTURE.md must be readable");
    let mut slugs = explicit_anchors(&markdown);
    slugs.extend(markdown_headings(&markdown));
    slugs
}

/// Strip a Rust comment marker so a citation reads the same in prose and in a
/// doc comment.
fn uncommented(line: &str) -> &str {
    let line = line.trim_start();
    for marker in ["///", "//!", "//"] {
        if let Some(rest) = line.strip_prefix(marker) {
            return rest.trim_start();
        }
    }
    line
}

/// The slug of a citation starting at `text`, if `text` opens with one wrapped
/// in `*…*` or backticks.
///
/// The wrapper is the first of two discriminators separating a citation from
/// prose: "the ADR when-to-write bar" carries a slug-shaped word and names no
/// record, and requiring the wrapper drops it. See [`names_a_record_slug`] for
/// the second, which the wrapper alone cannot supply.
fn wrapped_slug(text: &str) -> Option<&str> {
    let (wrapper, rest) = text
        .strip_prefix('*')
        .map(|rest| ('*', rest))
        .or_else(|| text.strip_prefix('`').map(|rest| ('`', rest)))?;
    let (slug, _) = rest.split_once(wrapper)?;
    let valid = !slug.is_empty()
        && slug.chars().all(|character| {
            character.is_ascii_lowercase() || matches!(character, '0'..='9' | '-')
        });
    valid.then_some(slug)
}

/// Whether a wrapped word after "ADR" is a citation rather than emphasis.
///
/// The wrapper is not enough on its own: `ADR *and*` — the shape real corpus
/// prose takes wherever "and" is emphasised right after "ADR" — is
/// syntactically identical to `ADR *pruning*`. A slug earns the reading if it
/// is **hyphenated** (no English word emphasised after "ADR" is) or if it
/// **names a record this repository actually has**, which is what makes the
/// single-word `pruning` a citation. A hyphenated slug that resolves nowhere
/// stays a citation on purpose: that is a dangling reference, and reporting it
/// is the point.
fn names_a_record_slug(slug: &str, known: &HashSet<String>) -> bool {
    slug.contains('-') || known.contains(slug)
}

/// Every `ADR <slug>` citation in a document, as (slug, line number).
///
/// A citation may wrap onto the next line — in prose and in a doc comment
/// alike — so a trailing "ADR" takes its slug from the line below.
fn adr_citations(text: &str) -> Vec<(String, usize)> {
    let lines: Vec<&str> = text.lines().collect();
    let mut citations = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        for (offset, _) in line.match_indices("ADR") {
            let before_is_word = line[..offset]
                .chars()
                .next_back()
                .is_some_and(|character| character.is_alphanumeric());
            let after = &line[offset + "ADR".len()..];
            if before_is_word || after.starts_with(|character: char| character.is_alphanumeric()) {
                continue;
            }

            let trimmed = after.trim_start();
            let slug = if trimmed.is_empty() {
                lines
                    .get(index + 1)
                    .and_then(|next| wrapped_slug(uncommented(next)))
            } else if after.starts_with(char::is_whitespace) {
                wrapped_slug(trimmed)
            } else {
                None
            };

            if let Some(slug) = slug {
                citations.push((slug.to_owned(), index + 1));
            }
        }
    }

    citations
}

/// Why a citation is wrong, or `None` when it names a real decision record.
/// Total and pure, so the control below can show it failing.
fn miscited_reason(
    slug: &str,
    adrs: &HashSet<String>,
    specs: &HashSet<String>,
    architecture: &HashSet<String>,
) -> Option<String> {
    if adrs.contains(slug) {
        return None;
    }
    if specs.contains(slug) {
        return Some(format!(
            "`{slug}` is a spec — cite it as docs/specs/{slug}.md"
        ));
    }
    if architecture.contains(slug) {
        return Some(format!(
            "`{slug}` is a docs/ARCHITECTURE.md anchor, not a decision record — \
             drop the ADR qualifier and cite the bare slug"
        ));
    }
    Some(format!("`{slug}` names no record in this repository"))
}

#[test]
fn every_adr_citation_names_a_decision_record() {
    let root = repository_root();
    let (adrs, specs, architecture) = (
        slugs_in("docs/adr"),
        slugs_in("docs/specs"),
        architecture_slugs(),
    );

    let known: HashSet<String> = adrs
        .iter()
        .chain(specs.iter())
        .chain(architecture.iter())
        .cloned()
        .collect();

    let mut sources = repository_files(&[".md", ".rs"]);
    // History keeps the past, and this file quotes the exact defect it checks
    // for — both would fail a check that could not name its own exceptions.
    sources.retain(|path| path != HISTORY && path != THIS_FILE);

    let mut cited = 0;
    let mut miscited = Vec::new();
    for source in &sources {
        let text = std::fs::read_to_string(root.join(source))
            .unwrap_or_else(|error| panic!("{source} must be readable: {error}"));
        for (slug, line_number) in adr_citations(&text) {
            if !names_a_record_slug(&slug, &known) {
                continue;
            }
            cited += 1;
            if let Some(reason) = miscited_reason(&slug, &adrs, &specs, &architecture) {
                miscited.push(format!("{source}:{line_number}: {reason}"));
            }
        }
    }

    assert!(
        cited > 0,
        "no ADR citation was found at all — the scan is not reading these surfaces"
    );
    assert!(
        miscited.is_empty(),
        "citations naming a record by a kind it does not have:\n  {}",
        miscited.join("\n  ")
    );
}

#[test]
fn the_citation_check_distinguishes_a_record_kind_from_prose() {
    let (adrs, specs, architecture) = (
        slugs_in("docs/adr"),
        slugs_in("docs/specs"),
        architecture_slugs(),
    );

    // A real decision record passes.
    assert!(
        miscited_reason("jj-is-the-only-lane", &adrs, &specs, &architecture).is_none(),
        "a file under docs/adr/ must be citable as an ADR"
    );
    // Positive control: the exact defect this check exists for.
    assert!(
        miscited_reason("pruning", &adrs, &specs, &architecture)
            .is_some_and(|reason| reason.contains("ARCHITECTURE")),
        "an architecture anchor cited as an ADR must be reported"
    );
    assert!(
        miscited_reason("doubt-grove-review-mechanics", &adrs, &specs, &architecture)
            .is_some_and(|reason| reason.contains("spec")),
        "a spec cited as an ADR must be reported"
    );
    assert!(
        miscited_reason("no-such-record", &adrs, &specs, &architecture).is_some(),
        "a citation that resolves nowhere must be reported"
    );

    // Two discriminators separate a citation from prose about ADRs: the
    // wrapper, and then whether the wrapped word names a record at all.
    let known: HashSet<String> = adrs
        .iter()
        .chain(specs.iter())
        .chain(architecture.iter())
        .cloned()
        .collect();
    assert_eq!(
        adr_citations("See ADR *pruning* and the ADR when-to-write bar.\n"),
        [("pruning".to_owned(), 1)],
        "the unwrapped `when-to-write` must not read as a citation"
    );
    assert!(
        names_a_record_slug("pruning", &known),
        "a single-word slug that names a real record is a citation"
    );
    assert!(
        names_a_record_slug("no-such-record", &known),
        "a hyphenated slug is a citation even when it resolves nowhere — that is \
         the dangling case this must report, not skip"
    );
    assert!(
        !names_a_record_slug("and", &known),
        "emphasis after the letters ADR must not read as a citation"
    );
    // A citation wrapping onto the next line, in prose and in a doc comment.
    assert_eq!(
        adr_citations("cleared by ADR\n*cli-binary-split*.\n"),
        [("cli-binary-split".to_owned(), 1)]
    );
    assert_eq!(
        adr_citations("/// the note above (ADR\n/// `pruning`): resolve must not\n"),
        [("pruning".to_owned(), 1)]
    );
    // "ADR set" and "ADRs" are prose, not citations.
    assert!(adr_citations("the ADR set holds the why; ADRs are minimal.\n").is_empty());
}

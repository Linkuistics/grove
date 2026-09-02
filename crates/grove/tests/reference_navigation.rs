use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

mod support;

/// The user-facing documentation surface: the files a reader reaches without
/// opening `src/`, and the only ones this check walks.
///
/// The guides are enumerated rather than globbed, because the claim is about a
/// *surface* a person navigates, not about every Markdown file in the tree.
/// Architecture, decision records, specs, and the provisioned methodology are
/// separately owned; a repo-wide sweep belongs to whoever owns all of them at
/// once, and is [`every_repository_markdown_reference_resolves`] below.
///
/// The code walkthroughs are the one part of the surface that is not a list.
/// A book is navigated exactly as a guide is — a contents page, chapters, two
/// lookup indexes — so it belongs here; but naming each one would cost an edit
/// per book, and decision 8 of `plan-k1` requires the membership to be held by
/// a machine rather than by the next author's memory. So the guides are named
/// and the books are discovered. That is an addition to this list's membership
/// rule, not the abandonment of it: `docs/walkthroughs/` is a reader's surface
/// in exactly the way `docs/adr/` is not.
const NAMED_GUIDES: [&str; 5] = [
    "README.md",
    "CHANGELOG.md",
    "docs/USAGE.md",
    "docs/CONFIGURATION.md",
    "docs/RELEASING.md",
];

/// Where the code walkthroughs live. One directory here is one book — the same
/// rule `scripts/check.sh` and `crates/grove/tests/corpus_exception_inventory.rs`
/// already discover by, so all three agree on what a book root is without
/// sharing a list of them.
const BOOKS: &str = "docs/walkthroughs";

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
    book_validation::scan_markdown_links(markdown)
        .into_iter()
        .filter(|link| {
            !link.destination.is_empty()
                && !link.destination.starts_with('#')
                && !link.destination.contains("://")
                && !link.destination.starts_with("mailto:")
        })
        .map(|link| (link.destination, link.line))
        .collect()
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

/// One member of the curated surface. A named guide is a single document; a
/// book is every page in its directory, which a reader moves through as one
/// document by way of its navigation lines.
struct SurfaceEntry {
    label: String,
    documents: Vec<String>,
}

/// The book roots under [`BOOKS`], repository-relative and sorted. A sixth book
/// joins this surface by existing, which is the whole of decision 8's
/// "machine-held": there is no list here to forget to add it to.
fn book_roots(root: &Path) -> Vec<String> {
    let mut roots: Vec<String> = std::fs::read_dir(root.join(BOOKS))
        .unwrap_or_else(|error| panic!("{BOOKS} must be readable: {error}"))
        .map(|entry| entry.expect("book directory entries must be readable"))
        .filter(|entry| {
            entry
                .file_type()
                .expect("entry type must be readable")
                .is_dir()
        })
        .map(|entry| format!("{BOOKS}/{}", entry.file_name().to_string_lossy()))
        .collect();
    roots.sort();
    roots
}

/// The Markdown pages of one book, sorted. Shallow, because a book is a flat
/// directory of pages beside its manifest.
fn book_pages(root: &Path, book: &str) -> Vec<String> {
    let mut pages: Vec<String> = std::fs::read_dir(root.join(book))
        .unwrap_or_else(|error| panic!("{book} must be readable: {error}"))
        .map(|entry| entry.expect("book page entries must be readable"))
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md"))
        .map(|name| format!("{book}/{name}"))
        .collect();
    pages.sort();
    pages
}

/// The curated surface: the named guides, then every book root by discovery.
fn user_documentation(root: &Path) -> Vec<SurfaceEntry> {
    let mut surface: Vec<SurfaceEntry> = NAMED_GUIDES
        .into_iter()
        .map(|guide| SurfaceEntry {
            label: guide.to_owned(),
            documents: vec![guide.to_owned()],
        })
        .collect();

    let books = book_roots(root);
    assert!(
        !books.is_empty(),
        "no book root found under {BOOKS} — a surface that discovers nothing passes for any \
         set of books"
    );
    for book in books {
        let documents = book_pages(root, &book);
        assert!(
            !documents.is_empty(),
            "{book} is a book root with no Markdown page"
        );
        surface.push(SurfaceEntry {
            label: book,
            documents,
        });
    }

    surface
}

#[test]
fn user_documentation_references_resolve() {
    let root = repository_root();

    for entry in user_documentation(&root) {
        // Counted per surface *entry*, not per file. A guide with no relative
        // link is a guide this check would pass over silently, and that is what
        // the count exists to reject; a single chapter of a book may honestly
        // carry none, while a book that carries none is not navigable at all.
        let mut references = 0;

        for document in &entry.documents {
            let markdown = std::fs::read_to_string(root.join(document))
                .unwrap_or_else(|error| panic!("{document} must be readable: {error}"));
            for (target, line_number) in relative_link_targets(&markdown) {
                references += 1;
                if let Some(reason) = unresolved_reason(&root, document, &target) {
                    panic!(
                        "{document}:{line_number}: reference `{target}` does not resolve — {reason}"
                    );
                }
            }
        }

        assert!(
            references > 0,
            "{} must carry at least one relative reference for this check to mean anything",
            entry.label
        );
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
        "Inline `[illustrative](relative/path)` is not a repository reference.\n",
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
fn relative_link_scan_keeps_the_destination_of_titled_links() {
    assert_eq!(
        relative_link_targets(r#"See [usage](docs/USAGE.md "guide")."#),
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
// [`user_documentation`] above builds *a surface* — the files a reader reaches
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

// ---------------------------------------------------------------------------
// Architecture anchors cited from a source file
//
// The check above reads `.rs` for one shape only — a record named by its
// *kind*. The other shape a source file uses is the anchor link itself,
// `docs/ARCHITECTURE.md#tree-access-lock`, written repository-relative in a
// doc comment beside the code the anchor explains. Nothing resolved it: the
// repository-wide sweep is Markdown-only, so an anchor could be renamed out
// from under every citation in `src/` and the suite would stay green.
//
// The Markdown half of "wherever it appears" is already
// [`every_repository_markdown_reference_resolves`], which resolves these links
// as links. This is the other surface, and the two do not overlap: a Rust file
// carries the citation as text rather than as a Markdown destination, so no
// link scanner reaches it and no citation is reported twice.
//
// What is *not* checked here is the bare parenthetical — `(task-kind-taxonomy)`
// — which is the repository's own compact form and carries no path to break.
// This check is about the form that asserts where an anchor lives.

/// The one path form this check resolves: the repository-relative citation
/// every Rust source here uses.
///
/// A source file is not rendered, so a directory-relative form like
/// `../ARCHITECTURE.md#…` has no directory to resolve against, and none appears
/// in `src/`. Where one does appear — inside `book-validation`'s synthetic
/// Markdown fixtures, whose anchors are made up — this prefix does not match
/// it. That is a consequence of how those fixtures are spelled today rather
/// than a guarantee about fixtures: a fixture written repository-relative would
/// be read as a citation, and the fix then is to spell the fixture the way a
/// fixture Markdown file actually sits, not to widen this constant.
const ARCHITECTURE_CITATION: &str = "docs/ARCHITECTURE.md#";

/// Every `docs/ARCHITECTURE.md#<anchor>` citation in a source file, as
/// (anchor, line number).
///
/// The anchor runs to the first character an anchor cannot contain, so the
/// closing backtick, bracket or sentence-ending full stop that follows a
/// citation in prose is not read as part of it.
fn architecture_anchor_citations(text: &str) -> Vec<(String, usize)> {
    let mut citations = Vec::new();

    for (index, line) in text.lines().enumerate() {
        for (offset, _) in line.match_indices(ARCHITECTURE_CITATION) {
            let anchor: String = line[offset + ARCHITECTURE_CITATION.len()..]
                .chars()
                .take_while(|character| {
                    character.is_alphanumeric() || matches!(character, '-' | '_')
                })
                .collect();
            // `docs/ARCHITECTURE.md#<anchor>` is how this repository writes the
            // *shape* of a citation when it is describing the convention rather
            // than using it. Metasyntax names no anchor, so there is nothing to
            // resolve.
            if !anchor.is_empty() {
                citations.push((anchor, index + 1));
            }
        }
    }

    citations
}

/// Why a cited anchor does not resolve, or `None` when it does. Total and pure,
/// so the control below can show it failing.
fn unresolved_architecture_anchor(anchor: &str, architecture: &HashSet<String>) -> Option<String> {
    (!architecture.contains(anchor))
        .then(|| format!("`#{anchor}` matches no heading or explicit anchor in {ARCHITECTURE}"))
}

#[test]
fn every_architecture_anchor_citation_in_a_source_resolves() {
    let root = repository_root();
    let architecture = architecture_slugs();

    // This file is scanned like any other, unlike the ADR citation scan above,
    // which must exclude it. Nothing here needs excluding: every citation the
    // fixtures below carry names an anchor `docs/ARCHITECTURE.md` really has,
    // and the dangling case is exercised through
    // [`unresolved_architecture_anchor`] with a bare anchor and no path in
    // front of it — chosen precisely so this file stays inside the sweep. A
    // fixture that needs a citation resolving nowhere is written the same way.
    let sources = repository_files(&[".rs"]);

    // The extension list is the only narrowing this sweep has, and a sweep that
    // stopped reaching the sources that carry these citations would read
    // exactly like a clean tree. Name the two that carry the most.
    for expected in [
        "crates/grove-loop/src/tree_lifecycle.rs",
        "crates/grove-loop/src/task_tree.rs",
    ] {
        assert!(
            sources.iter().any(|source| source == expected),
            "the source sweep must reach {expected}"
        );
    }

    let mut cited = 0;
    let mut unresolved = Vec::new();
    for source in &sources {
        let text = std::fs::read_to_string(root.join(source))
            .unwrap_or_else(|error| panic!("{source} must be readable: {error}"));
        for (anchor, line_number) in architecture_anchor_citations(&text) {
            cited += 1;
            if let Some(reason) = unresolved_architecture_anchor(&anchor, &architecture) {
                unresolved.push(format!("{source}:{line_number}: {reason}"));
            }
        }
    }

    assert!(
        cited > 0,
        "no architecture anchor citation was found at all — the scan is not reading these sources"
    );
    assert!(
        unresolved.is_empty(),
        "architecture anchors cited from a source that resolve nowhere:\n  {}",
        unresolved.join("\n  ")
    );
}

#[test]
fn the_architecture_anchor_check_reads_a_citation_and_rejects_a_dangling_one() {
    let architecture = architecture_slugs();

    // Both anchor namespaces the document publishes: an explicit `<a id="…">`
    // that survives a retitle, and one generated from a heading that does not.
    assert!(
        unresolved_architecture_anchor("task-kind-taxonomy", &architecture).is_none(),
        "an explicit `<a id=…>` anchor must resolve"
    );
    assert!(
        unresolved_architecture_anchor("runtime-flow", &architecture).is_none(),
        "a generated heading anchor must resolve"
    );
    // Positive control: the defect this check exists for.
    assert!(
        unresolved_architecture_anchor("no-such-anchor", &architecture).is_some(),
        "an anchor the document does not carry must be reported"
    );

    // The citation is read out of a doc comment, and stops where the anchor
    // does — not at the backtick, bracket or full stop that follows it.
    assert_eq!(
        architecture_anchor_citations(concat!(
            "/// clause 2 (`docs/ARCHITECTURE.md#library-refusals`).\n",
            "//! see docs/ARCHITECTURE.md#tree-access-lock.\n",
            "// [the lock](docs/ARCHITECTURE.md#tree-access-lock)\n",
        )),
        [
            ("library-refusals".to_owned(), 1),
            ("tree-access-lock".to_owned(), 2),
            ("tree-access-lock".to_owned(), 3),
        ]
    );
    // The anchor is read with the same character class the heading namespace is
    // built from, so a namespace this document could publish is never one the
    // scanner is incapable of spelling. Truncating at the first character an
    // anchor cannot contain is what stops the trailing full stop above; a `.`
    // is therefore never part of an anchor here, which the explicit-anchor
    // convention in *Documentation ownership* already holds to.
    //
    // The anchor is interpolated rather than written out, because this file is
    // itself inside the sweep and `docs/ARCHITECTURE.md` publishes no non-ASCII
    // anchor to cite: a literal here would be a real citation resolving
    // nowhere. `{` is not an anchor character, so the `{anchor}` left in the
    // format string names nothing and the sweep reads past it — the same rule
    // that lets this repository write the shape `docs/ARCHITECTURE.md#<anchor>`
    // in prose.
    let anchor = "café-notes";
    assert_eq!(
        architecture_anchor_citations(&format!("/// see docs/ARCHITECTURE.md#{anchor}.\n")),
        [(anchor.to_owned(), 1)]
    );
    // Two citations on one line are two citations.
    assert_eq!(
        architecture_anchor_citations(
            "/// docs/ARCHITECTURE.md#pruning and docs/ARCHITECTURE.md#no-migration\n"
        ),
        [("pruning".to_owned(), 1), ("no-migration".to_owned(), 1)]
    );
    // Neither the bare parenthetical the repository uses everywhere, nor a
    // citation of the document with no anchor, nor the metasyntax for the
    // shape itself, names an anchor to resolve.
    assert!(architecture_anchor_citations(concat!(
        "/// that rule names (task-kind-taxonomy).\n",
        "/// stated in docs/ARCHITECTURE.md, which owns it.\n",
        "/// written docs/ARCHITECTURE.md#<anchor> when describing the form.\n",
    ))
    .is_empty());
}

// ---------------------------------------------------------------------------
// The documentation-ownership table
//
// Decision 8 of `plan-k1` settled two obligations for books: they join the
// curated user surface above, and each earns a row in `docs/ARCHITECTURE.md`'s
// *Documentation ownership* table. [`user_documentation`] holds the first by
// discovery; this holds the second.
//
// It is deliberately a check about a document the book does not own. Everything
// else that gates a book — `book-check`, the manifest, the fragment ledger — is
// the book's own account of itself, and a book can be complete, reconstructing,
// and green while no document in the repository says what subject it is
// canonical for. The table is where that is said, and a row is what makes a new
// book joining `docs/` a decision somebody recorded rather than a directory
// that appeared.

const ARCHITECTURE: &str = "docs/ARCHITECTURE.md";
const OWNERSHIP_HEADING: &str = "## Documentation ownership";

/// Resolve a link destination written inside `directory` to a
/// repository-relative path, textually.
///
/// Textual on purpose: what is read here is what the table *claims* is
/// canonical, and whether that claim resolves to a real file is already
/// [`every_repository_markdown_reference_resolves`]' job. Two checks reporting
/// the same broken link would only make the second one's message worse.
fn repository_relative(directory: &str, destination: &str) -> String {
    let mut segments: Vec<&str> = Vec::new();
    for segment in directory.split('/').chain(destination.split('/')) {
        match segment {
            "" | "." => {}
            ".." => {
                segments.pop();
            }
            segment => segments.push(segment),
        }
    }
    segments.join("/")
}

/// Every repository-relative path the *Documentation ownership* table names as
/// a canonical source.
///
/// Bounded by the next `##` heading and taken from table rows only, so neither
/// the prose under the table — which cites `docs/adr/` and the specification
/// sets — nor a later section can be mistaken for a row.
fn documentation_ownership_targets(architecture: &str) -> Vec<String> {
    let after = architecture
        .split_once(OWNERSHIP_HEADING)
        .unwrap_or_else(|| panic!("{ARCHITECTURE} must carry `{OWNERSHIP_HEADING}`"))
        .1;
    let section = after
        .split_once("\n## ")
        .map_or(after, |(section, _)| section);

    let mut targets = Vec::new();
    for row in section
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
    {
        for link in book_validation::scan_markdown_links(row) {
            let destination = link.destination;
            if destination.starts_with('#')
                || destination.contains("://")
                || destination.starts_with("mailto:")
            {
                continue;
            }
            let path = destination.split('#').next().unwrap_or_default();
            if !path.is_empty() {
                targets.push(repository_relative("docs", path));
            }
        }
    }
    targets
}

/// The book roots the table declares nothing canonical for. Total and pure, so
/// the check below can be shown failing without a scratch book on disk.
///
/// A row naming a page *inside* the book counts, because the canonical source
/// for a book is its contents page rather than its directory — but only a page
/// inside it: the trailing separator is what stops `…/keyed-launch-notes` from
/// answering for `…/keyed-launch`.
fn book_roots_without_ownership_row(book_roots: &[String], targets: &[String]) -> Vec<String> {
    book_roots
        .iter()
        .filter(|book| {
            let inside = format!("{book}/");
            !targets
                .iter()
                .any(|target| target == *book || target.starts_with(&inside))
        })
        .cloned()
        .collect()
}

#[test]
fn every_book_root_has_a_documentation_ownership_row() {
    let root = repository_root();
    let architecture = std::fs::read_to_string(root.join(ARCHITECTURE))
        .unwrap_or_else(|error| panic!("{ARCHITECTURE} must be readable: {error}"));

    let targets = documentation_ownership_targets(&architecture);
    // The parse must reach the real table, or an empty target set would report
    // every book as unowned — a check that fails for the wrong reason is no
    // better than one that passes for the wrong reason.
    assert!(
        targets.iter().any(|target| target == "docs/USAGE.md"),
        "the ownership table was not read: its row for the human workflow guide is missing from \
         {targets:?}"
    );

    let books = book_roots(&root);
    assert!(
        !books.is_empty(),
        "no book root found under {BOOKS} — a table compared against nothing has a row for \
         every book"
    );

    let missing = book_roots_without_ownership_row(&books, &targets);
    assert!(
        missing.is_empty(),
        "book roots with no row in {ARCHITECTURE}'s *Documentation ownership* table: {missing:?}"
    );
}

#[test]
fn the_ownership_row_check_reports_a_book_with_no_row() {
    let books = [
        "docs/walkthroughs/declared".to_owned(),
        "docs/walkthroughs/undeclared".to_owned(),
    ];
    let targets = [
        "docs/USAGE.md".to_owned(),
        "docs/walkthroughs/declared/README.md".to_owned(),
    ];
    assert_eq!(
        book_roots_without_ownership_row(&books, &targets),
        ["docs/walkthroughs/undeclared"],
        "a book root the table names nothing for must be reported"
    );

    // A shared prefix is not a row.
    assert_eq!(
        book_roots_without_ownership_row(
            &books[..1],
            &["docs/walkthroughs/declared-elsewhere/README.md".to_owned()]
        ),
        ["docs/walkthroughs/declared"]
    );

    // The row for a book is written relative to `docs/`, and the guides above
    // it reach back out of the directory; both must land on the same surface
    // the book roots are named on.
    assert_eq!(repository_relative("docs", "../README.md"), "README.md");
    assert_eq!(
        repository_relative("docs", "walkthroughs/ordinal-fs-tree/README.md"),
        "docs/walkthroughs/ordinal-fs-tree/README.md"
    );

    // Only rows are read: a link in the prose under the table is not a
    // canonical-source declaration.
    assert_eq!(
        documentation_ownership_targets(concat!(
            "## Documentation ownership\n",
            "| Subject | Canonical source |\n",
            "|---|---|\n",
            "| A book | [`contents`](walkthroughs/a-book/README.md) |\n",
            "\nSee [prose](NOT-A-ROW.md).\n",
            "\n## Repository products\n",
            "| Product | Source |\n",
            "| Grove | [`later`](LATER.md) |\n",
        )),
        ["docs/walkthroughs/a-book/README.md"]
    );
}

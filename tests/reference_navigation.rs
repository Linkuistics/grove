use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

const DRIVING: &str = include_str!("../content/driving.md");

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

struct MarkdownHeading {
    anchor: String,
    base_anchor: String,
    line_number: usize,
}

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

fn markdown_headings(markdown: &str) -> Vec<MarkdownHeading> {
    let mut open_fence = None;
    let mut used_anchors = HashSet::new();
    let mut next_suffix_by_base = HashMap::new();
    let mut headings = Vec::new();

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
        headings.push(MarkdownHeading {
            anchor,
            base_anchor,
            line_number: line_index + 1,
        });
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
        .any(|heading| heading.anchor == fragment)
        || explicit_anchors(&markdown).contains(fragment);
    (!resolves).then(|| {
        format!(
            "fragment `#{fragment}` matches no heading or explicit anchor in {}",
            resolved.display()
        )
    })
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
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

fn guide_navigation_lines(markdown: &str) -> impl Iterator<Item = &str> {
    let (_, after_navigation_heading) = markdown
        .split_once("## In this guide\n")
        .expect("content/driving.md must have an In this guide section");
    after_navigation_heading
        .lines()
        .take_while(|line| atx_heading(line).is_none())
}

fn is_nested_list_item(line: &str) -> bool {
    let trimmed = line.trim_start();
    if line.len() == trimmed.len() {
        return false;
    }

    let marker_has_valid_tail =
        |tail: &str| tail.is_empty() || tail.starts_with(char::is_whitespace);
    if trimmed
        .strip_prefix(['-', '*', '+'])
        .is_some_and(marker_has_valid_tail)
    {
        return true;
    }

    let digit_count = trimmed
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .count();
    digit_count > 0
        && trimmed[digit_count..]
            .strip_prefix(['.', ')'])
            .is_some_and(marker_has_valid_tail)
}

fn navigation_anchors() -> Vec<String> {
    let mut anchors = Vec::new();
    for line in guide_navigation_lines(DRIVING) {
        assert!(
            !is_nested_list_item(line),
            "content/driving.md navigation must stay one level deep: {line:?}"
        );

        for link in line.split("](#").skip(1) {
            let (anchor, _) = link
                .split_once(')')
                .expect("each navigation anchor link must end after its target");
            anchors.push(anchor.to_owned());
        }
    }
    anchors
}

fn is_ambiguous_unsuffixed_target(
    navigation_anchor: &str,
    heading: &MarkdownHeading,
    base_anchor_count: usize,
) -> bool {
    base_anchor_count > 1 && navigation_anchor == heading.base_anchor
}

#[test]
fn navigation_structure_distinguishes_wrapping_from_nesting() {
    assert!(!is_nested_list_item("  or [another entry](#entry)"));
    assert!(!is_nested_list_item("- **Group:** [entry](#entry)"));
    assert!(is_nested_list_item("  - [nested entry](#nested-entry)"));
    assert!(is_nested_list_item("  -\t[nested entry](#nested-entry)"));
    assert!(is_nested_list_item("  -"));
    assert!(is_nested_list_item("  1. [nested entry](#nested-entry)"));
    assert!(is_nested_list_item("  1) [nested entry](#nested-entry)"));
}

#[test]
fn github_heading_namespace_includes_subsections_fences_and_deduplication() {
    let headings = markdown_headings(
        "# Guide\n## Same\n~~~markdown\n## Hidden example\n~~~~\n### Same\n  ````text\n## Also hidden\n  `````\n## Same-1\n   ### WDYT?\n####### Not a heading",
    );

    assert_eq!(
        headings
            .iter()
            .map(|heading| heading.anchor.as_str())
            .collect::<Vec<_>>(),
        ["guide", "same", "same-1", "same-1-1", "wdyt"]
    );
}

#[test]
fn navigation_scan_covers_loose_lists_until_the_next_section() {
    let markdown = "## In this guide\n\n- [first](#first)\n\n  - [nested](#nested)\n\n## First";
    assert_eq!(
        guide_navigation_lines(markdown).collect::<Vec<_>>(),
        ["", "- [first](#first)", "", "  - [nested](#nested)", ""]
    );
}

#[test]
fn collided_targets_require_an_explicit_deduplicated_anchor() {
    let headings = markdown_headings("## Target\n### Target");
    assert!(is_ambiguous_unsuffixed_target("target", &headings[0], 2));
    assert!(!is_ambiguous_unsuffixed_target("target-1", &headings[1], 2));
}

#[test]
fn driving_reference_navigation_resolves_in_document_order() {
    let headings = markdown_headings(DRIVING);
    let base_anchor_counts = headings.iter().fold(HashMap::new(), |mut counts, heading| {
        *counts.entry(&heading.base_anchor).or_insert(0) += 1;
        counts
    });
    let navigation_anchors = navigation_anchors();
    assert!(
        !navigation_anchors.is_empty(),
        "content/driving.md navigation must contain anchor links"
    );

    let mut previous_line = 0;
    for navigation_anchor in navigation_anchors {
        let heading = headings
            .iter()
            .find(|heading| heading.anchor == navigation_anchor)
            .unwrap_or_else(|| {
                panic!(
                    "content/driving.md navigation target {navigation_anchor:?} must resolve to a heading"
                )
            });
        assert!(
            !is_ambiguous_unsuffixed_target(
                &navigation_anchor,
                heading,
                base_anchor_counts[&heading.base_anchor],
            ),
            "content/driving.md navigation target {navigation_anchor:?} is an ambiguous unsuffixed anchor shared by multiple headings"
        );
        assert!(
            heading.line_number > previous_line,
            "content/driving.md navigation target {navigation_anchor:?} must name a later heading"
        );
        previous_line = heading.line_number;
    }
}

#[test]
fn driving_reference_navigation_includes_advertised_entry_points() {
    let navigation_anchors = navigation_anchors();
    for advertised_anchor in [
        "how-to-write-a-research-leaf-brief",
        "verifying-a-claim-about-the-repo-itself",
    ] {
        assert!(
            navigation_anchors.iter().any(|anchor| anchor == advertised_anchor),
            "content/driving.md navigation must include advertised entry point {advertised_anchor:?}"
        );
    }
}

use std::collections::{HashMap, HashSet};

const DRIVING: &str = include_str!("../content/driving.md");

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

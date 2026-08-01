use std::collections::HashSet;

use assert_cmd::Command;

const GROVE_SKILL: &str = include_str!("../content/SKILL.md");
const DRIVING: &str = include_str!("../content/driving.md");
const TASK_FORMAT: &str = include_str!("../content/TASK-FORMAT.md");
const DOUBT_SKILL: &str =
    include_str!("../plugins/linkuistics/skills/doubt-driven-development/SKILL.md");
const ARCHITECTURE: &str = include_str!("../docs/ARCHITECTURE.md");
const USAGE: &str = include_str!("../docs/USAGE.md");
const CONFIGURATION: &str = include_str!("../docs/CONFIGURATION.md");

fn assert_contains(surface: &str, text: &str, expected: &str) {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let expected = expected.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        text.contains(&expected),
        "{surface} must contain {expected:?}"
    );
}

fn assert_absent(surface: &str, text: &str, rejected: &str) {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let rejected = rejected.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(
        !text.contains(&rejected),
        "{surface} still contains obsolete guidance {rejected:?}"
    );
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

fn top_level_heading_anchors(markdown: &str) -> Vec<String> {
    let mut inside_fence = false;
    markdown
        .lines()
        .filter_map(|line| {
            if line.starts_with("```") {
                inside_fence = !inside_fence;
                return None;
            }
            if inside_fence {
                return None;
            }

            line.strip_prefix("## ")
                .filter(|heading| *heading != "In this guide")
                .map(github_heading_anchor)
        })
        .collect()
}

#[test]
fn driving_navigation_lint_models_github_anchor_edges() {
    assert_eq!(
        top_level_heading_anchors(
            "## Same\n```markdown\n## Hidden example\n```\n## Same\n## The seam — and `code/path`"
        ),
        ["same", "same", "the-seam--and-codepath"]
    );
}

#[test]
fn driving_reference_navigation_targets_top_level_sections() {
    let (_, after_navigation_heading) = DRIVING
        .split_once("## In this guide\n")
        .expect("content/driving.md must have an In this guide section");
    let navigation = after_navigation_heading
        .lines()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty());
    let mut navigation_anchors = Vec::new();

    for line in navigation {
        assert!(
            line.starts_with("- ") || line.starts_with("  [") || line.starts_with("  and ["),
            "content/driving.md navigation must stay one level deep: {line:?}"
        );

        for link in line.split("](#").skip(1) {
            let (anchor, _) = link
                .split_once(')')
                .expect("each navigation anchor link must end after its target");
            navigation_anchors.push(anchor.to_owned());
        }
    }

    let section_anchors = top_level_heading_anchors(DRIVING);
    let unique_section_anchors = section_anchors.iter().collect::<HashSet<_>>();
    assert_eq!(
        unique_section_anchors.len(),
        section_anchors.len(),
        "content/driving.md top-level headings must have unique GitHub anchors"
    );
    assert!(
        !navigation_anchors.is_empty() && navigation_anchors.len() < section_anchors.len(),
        "content/driving.md navigation must be selective rather than repeat the section outline"
    );

    let mut next_section = 0;
    for navigation_anchor in navigation_anchors {
        let relative_position = section_anchors[next_section..]
            .iter()
            .position(|section_anchor| section_anchor == &navigation_anchor)
            .unwrap_or_else(|| {
                panic!(
                    "content/driving.md navigation target {navigation_anchor:?} must name a later top-level section"
                )
            });
        next_section += relative_position + 1;
    }
}

#[test]
fn grove_guidance_replaces_the_old_in_session_review_loop() {
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/driving.md", DRIVING),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
    ] {
        assert_contains(surface, text, "whole picked leaf");
        assert_contains(surface, text, "leaf-promote-chain");
    }

    for (surface, text, integration_rule, research_rule) in [
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "substantial redesign inside the owning chain node",
            "every research-pair leaf spawn none",
        ),
        (
            "content/driving.md",
            DRIVING,
            "new producer review chain inside the owning chain node",
            "The two `research` producers and `combine-research` use their two-corpus and adversarial-combine disciplines instead",
        ),
        (
            "content/TASK-FORMAT.md",
            TASK_FORMAT,
            "`integrate-review-*` | at most one narrow reviewer | add a new producer review chain inside the owning chain node",
            "`research` / `combine-research` | none; the pair and combiner own breadth and doubt",
        ),
    ] {
        assert_contains(surface, text, integration_rule);
        assert_contains(surface, text, research_rule);
    }

    assert_absent("content/driving.md", DRIVING, "after three rounds");
    assert_absent(
        "content/driving.md",
        DRIVING,
        "big enough that a subagent cannot hold it",
    );
    assert_absent(
        "content/driving.md",
        DRIVING,
        "Either retrofit is the one case you still transcribe the kinds by hand",
    );
    assert_absent(
        "content/TASK-FORMAT.md",
        TASK_FORMAT,
        "there is deliberately no retrofit verb",
    );
}

#[test]
fn doubt_guidance_yields_to_grove_without_changing_standalone_doubt() {
    for expected in [
        "## Composition with Grove",
        "Bootstrap",
        "grove-llm pick",
        "whole picked leaf",
        "leaf-promote-chain",
        "Substantial redesign becomes a new producer review chain inside the owning chain node",
        "`research` and `combine-research` invoke none",
        "Outside that predicate",
    ] {
        assert_contains("doubt-driven-development/SKILL.md", DOUBT_SKILL, expected);
    }

    assert_contains("doubt-driven-development/SKILL.md", DOUBT_SKILL, "3 cycles");
}

#[test]
fn project_guides_name_the_implemented_composition_seams() {
    for expected in [
        "Tree access lock",
        "PROMOTING-",
        "**Reviews:**",
        "**Integrates:**",
        "Producer launch",
        "unless both",
    ] {
        assert_contains("docs/ARCHITECTURE.md", ARCHITECTURE, expected);
    }

    for expected in [
        "one in-session",
        "leaf-promote-chain",
        "reviewable boundary",
    ] {
        assert_contains("docs/USAGE.md", USAGE, expected);
    }

    for expected in [
        "Review target diversity",
        "unless both",
        "advisory",
        "GROVE_SESSION_TARGET",
    ] {
        assert_contains("docs/CONFIGURATION.md", CONFIGURATION, expected);
    }
}

#[test]
fn llm_help_exposes_the_structured_peek_promotion_and_receipt_handoff() {
    let kind = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["kind", "--help"])
        .output()
        .unwrap();
    let kind = String::from_utf8_lossy(&kind.stdout);
    for expected in ["--json", "stable", "handle", "harness"] {
        assert_contains("grove-llm kind --help", &kind, expected);
    }

    let promotion = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["leaf-promote-chain", "--help"])
        .output()
        .unwrap();
    let promotion = String::from_utf8_lossy(&promotion.stdout);
    for expected in ["currently picked plain producer", "PROMOTING-", "--json"] {
        assert_contains("grove-llm leaf-promote-chain --help", &promotion, expected);
    }

    let retirement = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["leaf-retire", "--help"])
        .output()
        .unwrap();
    let retirement = String::from_utf8_lossy(&retirement.stdout);
    assert_contains(
        "grove-llm leaf-retire --help",
        &retirement,
        "Producer launch",
    );
    assert_contains("grove-llm leaf-retire --help", &retirement, "best-effort");
}

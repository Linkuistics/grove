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

#[test]
fn grove_guidance_replaces_the_old_in_session_review_loop() {
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/driving.md", DRIVING),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
    ] {
        assert_contains(surface, text, "whole picked leaf");
        assert_contains(surface, text, "leaf-promote-chain");
        assert_contains(surface, text, "integrate-review-*");
        assert_contains(surface, text, "research");
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
        "integrate-review-*",
        "research",
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

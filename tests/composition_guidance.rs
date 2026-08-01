use assert_cmd::Command;

const GROVE_SKILL: &str = include_str!("../content/SKILL.md");
const CONTEXT: &str = include_str!("../CONTEXT.md");
const DRIVING: &str = include_str!("../content/driving.md");
const TASK_FORMAT: &str = include_str!("../content/TASK-FORMAT.md");
const DOUBT_SKILL: &str =
    include_str!("../plugins/linkuistics/skills/doubt-driven-development/SKILL.md");
const ARCHITECTURE: &str = include_str!("../docs/ARCHITECTURE.md");
const USAGE: &str = include_str!("../docs/USAGE.md");
const CONFIGURATION: &str = include_str!("../docs/CONFIGURATION.md");
const SPEC: &str = include_str!("../docs/specs/doubt-grove-review-mechanics.md");

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
    let root = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let root = String::from_utf8_lossy(&root.stdout);
    for expected in ["Producer launch", "source session", "generation"] {
        assert_contains("grove-llm --help", &root, expected);
    }

    let kind = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["kind", "--help"])
        .output()
        .unwrap();
    let kind = String::from_utf8_lossy(&kind.stdout);
    for expected in [
        "--json",
        "stable",
        "handle",
        "harness",
        "review",
        "producer-target",
    ] {
        assert_contains("grove-llm kind --help", &kind, expected);
    }

    let promotion = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["leaf-promote-chain", "--help"])
        .output()
        .unwrap();
    let promotion = String::from_utf8_lossy(&promotion.stdout);
    for expected in [
        "currently picked plain producer",
        "PROMOTING-",
        "--json",
        "source session",
        "generation",
    ] {
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

#[test]
fn canonical_guidance_explains_decomposed_receipts_and_pruning_scope() {
    for (surface, text, expected) in [
        ("CONTEXT.md", CONTEXT, "factual source session"),
        ("CONTEXT.md", CONTEXT, "producer generation"),
        ("content/SKILL.md", GROVE_SKILL, "factual source session"),
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "prune the enclosing review-chain node",
        ),
        ("content/driving.md", DRIVING, "producer generation"),
        ("content/TASK-FORMAT.md", TASK_FORMAT, "source session"),
        (
            "doubt-driven-development/SKILL.md",
            DOUBT_SKILL,
            "source session",
        ),
        ("docs/ARCHITECTURE.md", ARCHITECTURE, "producer-target"),
        ("docs/ARCHITECTURE.md", ARCHITECTURE, "no second tree read"),
        ("docs/USAGE.md", USAGE, "producer generation"),
        (
            "docs/USAGE.md",
            USAGE,
            "prune the enclosing review-chain node",
        ),
        ("docs/CONFIGURATION.md", CONFIGURATION, "producer-target"),
        ("docs/CONFIGURATION.md", CONFIGURATION, "factual pick"),
        (
            "receipt mechanics spec",
            SPEC,
            "legacy node receipts are uncheckable",
        ),
    ] {
        assert_contains(surface, text, expected);
    }
}

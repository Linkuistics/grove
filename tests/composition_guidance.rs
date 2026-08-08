use assert_cmd::Command;

const GROVE_SKILL: &str = include_str!("../content/SKILL.md");
const CONTEXT: &str = include_str!("../CONTEXT.md");
const DRIVING: &str = include_str!("../content/driving.md");
const TASK_FORMAT: &str = include_str!("../content/TASK-FORMAT.md");
const DOUBT_SKILL: &str =
    include_str!("../plugins/linkuistics/skills/doubt-driven-development/SKILL.md");
const ARCHITECTURE: &str = include_str!("../docs/ARCHITECTURE.md");
const USAGE: &str = include_str!("../docs/USAGE.md");
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

    for (surface, text, integration_rule) in [
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "substantial redesign inside the owning chain node",
        ),
        (
            "content/driving.md",
            DRIVING,
            "new producer review chain inside the owning chain node",
        ),
        (
            "content/TASK-FORMAT.md",
            TASK_FORMAT,
            "`integrate-review-*` | at most one narrow reviewer | add a new producer review chain inside the owning chain node",
        ),
    ] {
        assert_contains(surface, text, integration_rule);
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
fn planning_guidance_prefers_dependency_ordered_working_increments() {
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/driving.md", DRIVING),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
    ] {
        assert_contains(
            surface,
            text,
            "smallest independently useful working increments",
        );
        assert_contains(surface, text, "order them by dependency");
        assert_contains(surface, text, "separate grove");
        assert_contains(
            surface,
            text,
            "cannot independently leave the product working",
        );
    }
}

#[test]
fn doubt_guidance_yields_to_grove_without_changing_standalone_doubt() {
    for expected in [
        "## Composition with Grove",
        "Bootstrap",
        "whole picked leaf",
        "leaf-promote-chain",
        "Substantial redesign becomes a new producer review chain inside the owning chain node",
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
}

#[test]
fn llm_help_exposes_the_promotion_recovery_contract() {
    let promotion = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["leaf-promote-chain", "--help"])
        .output()
        .unwrap();
    let promotion = String::from_utf8_lossy(&promotion.stdout);
    assert_contains(
        "grove-llm leaf-promote-chain --help",
        &promotion,
        "PROMOTING-",
    );
}

#[test]
fn canonical_guidance_preserves_composition_relationships_and_pruning_scope() {
    for (surface, text, expected) in [
        (
            "CONTEXT.md",
            CONTEXT,
            "preserving `Reviews` and `Integrates`",
        ),
        (
            "CONTEXT.md",
            CONTEXT,
            "Bootstraps the resolved leaf, and does not pick again",
        ),
        (
            "CONTEXT.md",
            CONTEXT,
            "`leaf-promote-chain` likewise trusts the named live producer after its structural gates and never recomputes pick",
        ),
        ("review mechanics spec", SPEC, "**Reviews:**"),
        ("review mechanics spec", SPEC, "**Integrates:**"),
        (
            "review mechanics spec",
            SPEC,
            "`research-a`, `research-b`, or `combine-research` | None; the pair supplies independent corpora and the combiner supplies the adversarial move",
        ),
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "prune the enclosing review-chain node",
        ),
        ("docs/ARCHITECTURE.md", ARCHITECTURE, "no second tree read"),
        (
            "docs/USAGE.md",
            USAGE,
            "prune the enclosing review-chain node",
        ),
    ] {
        assert_contains(surface, text, expected);
    }
}

use assert_cmd::Command;

const GROVE_SKILL: &str = include_str!("../content/SKILL.md");
const CONTEXT: &str = include_str!("../CONTEXT.md");
const DRIVING: &str = include_str!("../content/driving.md");
const TASK_FORMAT: &str = include_str!("../content/TASK-FORMAT.md");
const BRIEF_FORMAT: &str = include_str!("../content/BRIEF-FORMAT.md");
const DOUBT_SKILL: &str =
    include_str!("../plugins/linkuistics/skills/doubt-driven-development/SKILL.md");
const ARCHITECTURE: &str = include_str!("../docs/ARCHITECTURE.md");
const USAGE: &str = include_str!("../docs/USAGE.md");
const SPEC: &str = include_str!("../docs/specs/doubt-grove-review-mechanics.md");
const CHANGELOG: &str = include_str!("../CHANGELOG.md");

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
        // Escalation is one ordinary append now, so the surfaces must name the
        // verb *and* the kind to spell — a session that reads only "cut a review
        // leaf" has to guess `--kind`, which is the one wrong-but-well-formed
        // mistake nothing downstream catches.
        assert_contains(surface, text, "review-<producer>");
    }

    for (surface, text, integration_rule) in [
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "substantial redesign as a new producer review chain beside the leaf it is integrating",
        ),
        (
            "content/driving.md",
            DRIVING,
            "new producer review chain beside the leaf being integrated",
        ),
        (
            "content/TASK-FORMAT.md",
            TASK_FORMAT,
            "`integrate-review-*` | at most one narrow reviewer | add a new producer review chain beside the leaf being integrated",
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

/// The two eager constructors are gone, so no provisioned surface may still
/// instruct one. This is the meta-grove hazard stated as a test: whatever
/// `content/` says at this commit is what drives the *next* session, and a
/// surface naming a verb the binary refuses sends that session to a dead end it
/// cannot diagnose from prose.
///
/// The verb names are banned as **tokens**, because there is nothing left for
/// them to be reworded into — banning the token bans the concept. Each surface
/// also carries the positive rule that replaced them, so deleting a paragraph
/// wholesale fails rather than passing vacuously.
#[test]
fn no_provisioned_surface_instructs_a_deleted_composition_verb() {
    // Scoped to the surfaces that *drive a session* — the provisioned
    // methodology, the human guide, and the doubt skill. `CONTEXT.md` and the
    // review-mechanics spec are deliberately out: a glossary `_Avoid_` line and
    // a spec's removed-surface test seam exist precisely to name a deleted verb
    // and say it is gone, and banning the token there would ban the record along
    // with the instruction.
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/driving.md", DRIVING),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
        ("content/BRIEF-FORMAT.md", BRIEF_FORMAT),
        ("doubt-driven-development/SKILL.md", DOUBT_SKILL),
        ("docs/ARCHITECTURE.md", ARCHITECTURE),
        ("docs/USAGE.md", USAGE),
    ] {
        for rejected in ["leaf-add-chain", "leaf-promote-chain"] {
            assert_absent(surface, text, rejected);
        }
    }

    for (surface, text, replacement) in [
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "grove-llm leaf-add <parent> <stem>-review --kind review-<producer>",
        ),
        (
            "content/TASK-FORMAT.md",
            TASK_FORMAT,
            "each session cuts the next step",
        ),
        (
            "content/BRIEF-FORMAT.md",
            BRIEF_FORMAT,
            "There is one node species, and it always has one",
        ),
        (
            "CONTEXT.md",
            CONTEXT,
            "an eager chain constructor (`leaf-add-chain`) or a retrofit verb",
        ),
    ] {
        assert_contains(surface, text, replacement);
    }
}

/// A review that finds nothing must create nothing — the empty session this
/// workstream exists to remove — and the surfaces have to say so, because the
/// rule lives nowhere else: no verb enforces it and no filename records it.
#[test]
fn guidance_states_that_each_step_is_created_only_when_required() {
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
    ] {
        assert_contains(surface, text, "only if it has findings worth acting on");
        assert_contains(
            surface,
            text,
            "A review that finds nothing creates nothing and simply retires",
        );
    }

    // …and the counterpart: the pair is eager, and the reason is stated rather
    // than left as an inconsistency for a later session to "fix".
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
        ("docs/ARCHITECTURE.md", ARCHITECTURE),
    ] {
        assert_contains(surface, text, "inherit");
        assert_contains(surface, text, "independence the pair is run for");
    }
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
        "--kind review-<producer>",
        "Substantial redesign becomes a new producer review chain beside the leaf being integrated",
        "Outside that predicate",
    ] {
        assert_contains("doubt-driven-development/SKILL.md", DOUBT_SKILL, expected);
    }

    assert_contains("doubt-driven-development/SKILL.md", DOUBT_SKILL, "3 cycles");
}

/// Normalized text between two anchors, so an ordering check sees only the
/// passage that makes the claim and not an unrelated earlier mention.
fn passage(surface: &str, text: &str, start: &str, end: &str) -> String {
    let text = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let from = text
        .find(start)
        .unwrap_or_else(|| panic!("{surface} no longer contains the passage opening {start:?}"));
    let len = text[from..]
        .find(end)
        .unwrap_or_else(|| panic!("{surface} no longer contains the passage close {end:?}"));
    text[from..from + len].to_string()
}

/// The handoff is a *sequence*, so it is pinned as one: **retirement precedes
/// the commit**, on every surface that states the handoff.
///
/// Order is what rots here, not phrasing — the four surfaces word it four ways
/// and three of them had it backwards — so this asserts positions rather than
/// quoting a sentence any of them could legitimately reword. The cost of the
/// inversion is asymmetric and silent: under jj, committing first seals the
/// artifact and pushes the producer's `DONE` rename into the *next* task's
/// working-copy commit, and unpicking that afterwards means an operation-log
/// rewind. Under Git it merely leaves the rename uncommitted or forces a second
/// commit.
#[test]
fn every_handoff_passage_retires_before_it_commits() {
    for (surface, text, start, end) in [
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "When a picked producer needs fresh review",
            "The tree is a real",
        ),
        (
            "docs/USAGE.md",
            USAGE,
            "The producer finishes only to a coherent",
            "Grove then launches",
        ),
        (
            "doubt-driven-development/SKILL.md",
            DOUBT_SKILL,
            "After cutting that leaf",
            "Grove owns the escalated route",
        ),
        (
            "review mechanics spec",
            SPEC,
            "## Producer handoff",
            "## Test seams",
        ),
    ] {
        let passage = passage(surface, text, start, end);
        let retire = passage
            .find("retir")
            .unwrap_or_else(|| panic!("{surface}: the handoff passage never retires"));
        let commit = passage
            .find("commit")
            .unwrap_or_else(|| panic!("{surface}: the handoff passage never commits"));
        assert!(
            retire < commit,
            "{surface}: the handoff must retire before it commits, so the `DONE` \
             rename lands inside the task's own commit — got {passage:?}"
        );
    }
}

/// A grow verb unwinds on a *reported error*; it does not survive process death,
/// and no current-state document may say otherwise.
///
/// The overreach is specific and was live: "Nothing survives an interruption"
/// claimed a guarantee `add_run` cannot make — its rollback runs only when
/// control returns through the `Err` branch — and contradicted the architecture's
/// own immediately preceding statement that finish teardown and the session-kind
/// migration are the only operations promising interruption recovery. A reader
/// who believes the strong claim stops looking for the partial shape a `SIGKILL`
/// really can leave.
#[test]
fn the_grow_verbs_promise_error_rollback_and_not_interruption_recovery() {
    for (surface, text) in [
        ("docs/ARCHITECTURE.md", ARCHITECTURE),
        ("review mechanics spec", SPEC),
    ] {
        assert_absent(surface, text, "Nothing survives an interruption");
        assert_absent(surface, text, "either wrote the leaf or did not");
        assert_contains(surface, text, "process-interruption recovery");
    }

    assert_contains(
        "docs/ARCHITECTURE.md",
        ARCHITECTURE,
        "all-or-nothing **on a reported error**",
    );
    assert_contains(
        "docs/ARCHITECTURE.md",
        ARCHITECTURE,
        "**Process death mid-run is not recovered**",
    );
    assert_contains(
        "review mechanics spec",
        SPEC,
        "Process death is a different question and is deliberately **not** covered",
    );
}

/// Upgrade guidance for a stranded `PROMOTING-*` must be **phase-correct**. The
/// old transaction wrote its witness and generated the chain's steps before
/// moving the producer, and could stage a Git index entry for a child that never
/// landed — so "move the producer copy back by hand" is simply false for the
/// first state and incomplete for the last. Both surfaces must name the three
/// shapes and prefer recovery under the old binary.
#[test]
fn stranded_promotion_recovery_guidance_distinguishes_the_phases() {
    for (surface, text) in [("CHANGELOG.md", CHANGELOG), ("review mechanics spec", SPEC)] {
        assert_contains(surface, text, "old binary");
        assert_contains(surface, text, "git rm --cached");
        assert_contains(surface, text, "generated steps");
    }
}

#[test]
fn project_guides_name_the_implemented_composition_seams() {
    for expected in [
        "Tree access lock",
        "FINISHING-",
        "**Reviews:**",
        "**Integrates:**",
    ] {
        assert_contains("docs/ARCHITECTURE.md", ARCHITECTURE, expected);
    }

    for expected in [
        "one in-session",
        "grove-llm leaf-add <parent> <stem>-review --kind review-<producer>",
        "reviewable boundary",
    ] {
        assert_contains("docs/USAGE.md", USAGE, expected);
    }
}

/// `--help` is the surface a session re-orients on after dropping context, so
/// the verb that now builds a review chain has to teach the lazy contract there
/// and not only in the methodology. A session that reaches `leaf-add --help`
/// holding "I should cut a review step" must find which kind to name.
#[test]
fn llm_help_teaches_the_lazy_review_chain_on_the_verb_that_builds_it() {
    let help = Command::cargo_bin("grove-llm")
        .unwrap()
        .args(["leaf-add", "--help"])
        .output()
        .unwrap();
    let help = String::from_utf8_lossy(&help.stdout);
    for expected in [
        "review chain",
        "--kind review-<producer>",
        "flat",
        "if it found something worth acting on",
    ] {
        assert_contains("grove-llm leaf-add --help", &help, expected);
    }
}

/// Receipts and the diversity warning were a real contract once: retirement
/// rewrote its producer's linked review with a `**Producer launch:**` line, and
/// review launch compared that record against the scheduled target. Both are
/// gone from `src/` — `**Producer launch:**` survives only in `tree_migrate`, as
/// a line to *delete* from a legacy body. Prose is therefore the only place the
/// contract can still exist, and prose that describes it sends a session looking
/// for a marker no verb emits and waiting on a warning no launch raises.
///
/// **The negatives are terms, not sentences.** A sentence-shaped `assert_absent`
/// passes the moment someone rewords it while keeping the idea, which is the
/// failure mode that matters here — the ideas are what rot, not the phrasing.
/// These five have no current referent left to be reworded into, so banning the
/// token bans the concept. The bare word `receipt` is deliberately *not* on the
/// list: the finish cycle uses it in an unrelated sense (a durable completion
/// marker) and `content/SKILL.md` says so legitimately.
///
/// Each surface also carries the positive rule that replaced its negative, so
/// deleting a paragraph wholesale fails rather than passing vacuously.
#[test]
fn canonical_guidance_drops_receipt_and_diversity_era_review_routing() {
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/driving.md", DRIVING),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
        ("doubt-driven-development/SKILL.md", DOUBT_SKILL),
    ] {
        for rejected in [
            "Producer launch",
            "producer generation",
            "handoff receipt",
            "diversity warning",
            "routing peek",
        ] {
            assert_absent(surface, text, rejected);
        }
    }

    for (surface, text, replacement) in [
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "grove records no producer target, compares none, and warns about none",
        ),
        (
            "content/SKILL.md",
            GROVE_SKILL,
            "Retiring it is the filename `DONE` transition and nothing else",
        ),
        (
            "content/driving.md",
            DRIVING,
            "it compares nothing, records nothing about how the producer ran, and raises no notice either way",
        ),
        (
            "content/TASK-FORMAT.md",
            TASK_FORMAT,
            "never a note the producer left behind about how its own session ran",
        ),
        (
            "doubt-driven-development/SKILL.md",
            DOUBT_SKILL,
            "Grove compares no targets and raises no notice, so there is none to wait for or consume",
        ),
    ] {
        assert_contains(surface, text, replacement);
    }
}

/// Done-when: Grove and doubt state the *same* ownership predicate. Both halves
/// matter and they fail differently — a surface that dropped `mandate` would
/// describe an allowance with no trigger, and one that dropped `Bootstrap` would
/// let a session claim ownership from checkout state, which is exactly what ADR
/// *grove-owns-escalated-review* rejects.
///
/// `Bootstrap-and-pick` is banned because it names the *superseded* predicate:
/// the session no longer picks, so a surface still spelling it that way tells a
/// session to prove ownership by re-walking the tree.
#[test]
fn grove_and_doubt_state_one_mandate_based_ownership_predicate() {
    for (surface, text) in [
        ("content/SKILL.md", GROVE_SKILL),
        ("content/driving.md", DRIVING),
        ("content/TASK-FORMAT.md", TASK_FORMAT),
        ("doubt-driven-development/SKILL.md", DOUBT_SKILL),
    ] {
        assert_contains(surface, text, "mandate");
        assert_contains(surface, text, "Bootstrap");
        assert_absent(surface, text, "Bootstrap-and-pick");
    }
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
            "Bootstraps the resolved leaf, and does not pick again",
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
            "prune each of its live steps",
        ),
        ("docs/ARCHITECTURE.md", ARCHITECTURE, "no second tree read"),
        ("docs/USAGE.md", USAGE, "prune each of its live steps"),
    ] {
        assert_contains(surface, text, expected);
    }
}

const GROVE_SKILL: &str = include_str!("../content/SKILL.md");

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The Commit step alone, so a rule proved here cannot be satisfied by prose
/// somewhere else in the skill.
fn commit_step() -> String {
    let (_, from_commit) = GROVE_SKILL
        .split_once("\n**Commit.**")
        .expect("content/SKILL.md must have a Commit step");
    let (step, _) = from_commit
        .split_once("\n**Retire.**")
        .expect("content/SKILL.md's Commit step must be followed by the Retire step");
    normalized(step)
}

#[test]
fn commit_step_scopes_one_commit_to_the_whole_task() {
    assert!(
        commit_step().contains(&normalized(
            "the artifact, whatever the grow verbs wrote, and the `DONE` rename that retires the leaf"
        )),
        "the Commit step must scope the one focused commit to the whole task, \
         or the DONE rename lands in the next task's commit"
    );
}

#[test]
fn commit_step_states_the_boundary_for_both_git_and_jj() {
    let step = commit_step();

    for (vcs, expected) in [
        ("git", "In **git** the working tree is not history"),
        ("git", "one `git commit`, taken once the rename has landed"),
        ("jj", "In **jj** the working copy *is* a commit"),
        (
            "jj",
            "`jj describe -m` records the task but leaves that change open",
        ),
        (
            "jj",
            "`jj new` after describing, once the rename has landed",
        ),
        ("jj", "so the next session opens on its own empty change"),
    ] {
        assert!(
            step.contains(&normalized(expected)),
            "the Commit step must say what a {vcs} session leaves behind: {expected:?}"
        );
    }
}

#[test]
fn commit_step_cites_the_jj_lane_rather_than_teaching_it() {
    let step = commit_step();

    assert!(
        step.contains("linkuistics:using-jujutsu"),
        "the Commit step must cite the jj skill that owns the new/describe/new lane"
    );
    for tutorial_verb in [
        "jj rebase",
        "jj squash",
        "jj bookmark",
        "jj git push",
        "jj edit",
    ] {
        assert!(
            !step.contains(tutorial_verb),
            "the Commit step must stay a boundary rule rather than a jj tutorial: {tutorial_verb:?}"
        );
    }
}

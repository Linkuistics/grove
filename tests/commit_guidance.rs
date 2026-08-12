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
        .split_once("\n**Signal.**")
        .expect("content/SKILL.md's Commit step must be followed by the Signal step");
    normalized(step)
}

/// Byte offsets of the loop's procedural step headings, in the order the page
/// prints them.
fn step_offset(heading: &str) -> usize {
    GROVE_SKILL
        .find(heading)
        .unwrap_or_else(|| panic!("content/SKILL.md must have a {heading:?} step"))
}

#[test]
fn retire_precedes_commit_so_the_commit_can_contain_what_retire_writes() {
    // The Commit step scopes one commit to the `DONE` rename and everything the
    // parent-chain close writes, and requires a closed node's handle in the
    // message. All three are facts Retire establishes, so a reader following the
    // page top-to-bottom must reach Retire first — otherwise a jj `jj new` seal
    // pushes every one of them into the next task's change.
    assert!(
        step_offset("\n**Retire.**") < step_offset("\n**Commit.**"),
        "content/SKILL.md must place the Retire step before the Commit step"
    );
    assert!(
        step_offset("\n**Commit.**") < step_offset("\n**Signal.**"),
        "content/SKILL.md must place the Commit step before the Signal step"
    );
}

#[test]
fn the_commit_step_closes_after_the_parent_chain_cascade() {
    // The loop overview diagram carried this ordering a second time, as a
    // picture; mandate delivery ships bytes, not renderings, so it went and the
    // prose is now the only statement of it. Pin the prose, not a redrawing:
    // what must survive is that the commit contains what the cascade wrote, and
    // names the nodes the cascade closed.
    let step = commit_step();

    assert!(
        step.contains(&normalized(
            "together with anything the cascade above promoted or added"
        )),
        "the Commit step must scope the commit to what the parent-chain cascade wrote"
    );
    assert!(
        step.contains(&normalized(
            "the message cannot name a node you have not yet closed"
        )),
        "the Commit step must state why the cascade has to settle before the boundary closes"
    );
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

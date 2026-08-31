const GROVE_SKILL: &str = include_str!("../../../plugins/grove/skills/grove/SKILL.md");
/// The Commit step's procedure, which the trigger sentence on the loop page
/// defers to — **including the commit's scope**, which used to be proved against
/// `SKILL.md` and no longer can be.
///
/// The condition register keeps the situation (*the leaf is retired*) and the
/// path; `one-focused-commit` and `name-by-handle` are this file's
/// (`plugins/grove/conformance/rules.tsv`). Left pointed at `SKILL.md`, the three
/// claims below would have asserted that the router had re-grown a procedure.
const COMMIT_REFERENCE: &str =
    include_str!("../../../plugins/grove/skills/grove/references/commit.md");

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The scope section alone, so a rule proved here cannot be satisfied by prose
/// elsewhere in the reference.
///
/// Bounded below by the boundary section, which is what follows it. Naming the
/// neighbour is deliberate: a section moved out from under this slice fails
/// every claim below by name rather than quietly widening the span they are
/// proved over.
fn commit_scope() -> String {
    let (_, from_scope) = COMMIT_REFERENCE
        .split_once("## What one commit contains")
        .expect("the Commit reference must say what one commit contains");
    let (section, _) = from_scope
        .split_once("\n## Where the boundary falls")
        .expect("the scope section must be followed by the boundary section");
    normalized(section)
}

/// Byte offsets of two claims in the spine's `SKILL.md`, so their **order** on
/// the page can be asserted.
fn skill_offset(phrase: &str) -> usize {
    GROVE_SKILL
        .find(phrase)
        .unwrap_or_else(|| panic!("the spine's SKILL.md must carry {phrase:?}"))
}

/// **Retire before Commit, still asserted on the page a session reads first.**
///
/// The loop's steps are no longer paragraphs in `SKILL.md` — they are trigger
/// sentences naming the reference that carries each procedure — so the ordering
/// is asserted over the two sentences rather than over two headings. It survives
/// the rewrite because the ordering is the rule: the commit contains the `DONE`
/// rename and everything the parent-chain close writes, all of it written by
/// Retire, so a `jj new` seal taken first pushes every one of them into the next
/// task's change.
#[test]
fn retire_precedes_commit_so_the_commit_can_contain_what_retire_writes() {
    assert!(
        skill_offset("When the work is done, retire the leaf")
            < skill_offset("When the leaf is retired, commit it"),
        "the spine's SKILL.md must reach the retire condition before the commit one"
    );
}

/// **The Signal step is not on this page**, and that is the one thing this file
/// still has to say about it.
///
/// Commit-before-Signal used to be a byte ordering inside the corpus's
/// `SKILL.md`. It is now an ordering inside `${prompt}` — grove states its own
/// signalling contract last, after the load instruction and the driver's facts —
/// and `tests/prompt.rs` is where that is proved. What is left here is the local
/// half: a Signal step written back into the spine would restore the ordering
/// these offsets used to check while defeating the placement, so a second copy
/// has to fail somewhere, and this is the file that would stop noticing.
///
/// The corpus half of this pair went at `delete-provisioning-k19`, which deleted
/// `content/SIGNAL.md` along with the embed that carried it; the contract is
/// driver-authored prose now and has no file in any skill set.
#[test]
fn the_signal_step_lives_in_the_prompt_rather_than_on_the_loop_page() {
    assert!(
        !GROVE_SKILL.contains("\n**Signal.**"),
        "the spine's SKILL.md must not carry a Signal step — the launch prompt states \
         the signalling contract last, and a copy here would arrive a whole \
         mandate early"
    );
}

#[test]
fn the_commit_step_closes_after_the_parent_chain_cascade() {
    // The loop overview diagram carried this ordering a second time, as a
    // picture; mandate delivery ships bytes, not renderings, so it went and the
    // prose is now the only statement of it. Pin the prose, not a redrawing:
    // what must survive is that the commit contains what the cascade wrote, and
    // names the nodes the cascade closed.
    let step = commit_scope();

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
        commit_scope().contains(&normalized(
            "the artifact, whatever the grow verbs wrote, and the `DONE` rename that retires the leaf"
        )),
        "the Commit step must scope the one focused commit to the whole task, \
         or the DONE rename lands in the next task's commit"
    );
}

#[test]
fn commit_step_states_the_boundary_a_session_actually_reaches() {
    let step = normalized(COMMIT_REFERENCE);

    for expected in [
        "in jj the working copy *is* a commit",
        "`jj describe -m` records the task but leaves that change open",
        "`jj new` after describing, once the rename has landed",
        "so the next session opens on its own empty change",
        "Nothing needs staging first",
    ] {
        assert!(
            step.contains(&normalized(expected)),
            "the Commit step must say what a session leaves behind: {expected:?}"
        );
    }
}

/// Grove refuses a working tree that is not jj-enabled
/// (`docs/adr/jj-is-the-only-lane.md`), so a Commit step teaching a Git
/// boundary would be teaching one no session can reach — and a session that
/// followed it in a colocated tree would commit behind jj's operation log.
#[test]
fn commit_step_teaches_no_boundary_grove_refuses_to_produce() {
    let step = normalized(COMMIT_REFERENCE);

    for absent in ["git commit", "git add", "git status", "git mv"] {
        assert!(
            !step.contains(absent),
            "the Commit step must not teach a lane Grove refuses: {absent:?}"
        );
    }
}

#[test]
fn commit_step_cites_the_jj_lane_rather_than_teaching_it() {
    let step = normalized(COMMIT_REFERENCE);

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

const GROVE_SKILL: &str = include_str!("../content/SKILL.md");
/// The Commit step's procedure, which the trigger sentence on the loop page
/// defers to — **including the commit's scope**, which used to be proved against
/// `SKILL.md` and no longer can be.
///
/// The condition register keeps the situation (*the leaf is retired*) and the
/// path; `one-focused-commit` and `name-by-handle` are this file's
/// (`docs/specs/corpus-rule-ownership.md`). Left pointed at `SKILL.md`, the three
/// claims below would have asserted that the router had re-grown a procedure.
const COMMIT_REFERENCE: &str = include_str!("../content/references/commit.md");
// Deliberately not prefixed the way `GROVE_SKILL` above is. The loop control
// channel is an environment variable whose name begins the same way and ends in
// `_FILE`, so a test constant one token short of it reads as that variable to
// everyone but its author — and `tests/removed_surface.rs` would have to carry a
// classification entry saying it is not one.
const SIGNAL_PAGE: &str = include_str!("../content/SIGNAL.md");

fn normalized(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The scope section alone, so a rule proved here cannot be satisfied by prose
/// elsewhere in the reference.
///
/// Bounded below by the git/jj section, which is what follows it. Naming the
/// neighbour is deliberate: a section moved out from under this slice fails
/// every claim below by name rather than quietly widening the span they are
/// proved over.
fn commit_scope() -> String {
    let (_, from_scope) = COMMIT_REFERENCE
        .split_once("## What one commit contains")
        .expect("content/references/commit.md must say what one commit contains");
    let (section, _) = from_scope
        .split_once("\n## Where the boundary falls")
        .expect("the scope section must be followed by the git/jj boundary section");
    normalized(section)
}

/// Byte offsets of two claims in `content/SKILL.md`, so their **order** on the
/// page can be asserted.
fn skill_offset(phrase: &str) -> usize {
    GROVE_SKILL
        .find(phrase)
        .unwrap_or_else(|| panic!("content/SKILL.md must carry {phrase:?}"))
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
        "content/SKILL.md must reach the retire condition before the commit one"
    );
}

/// **The Signal step is not on this page**, and that is the one thing this file
/// still has to say about it.
///
/// Commit-before-Signal used to be a byte ordering inside `content/SKILL.md`.
/// It is now a *composition* ordering in `${prompt}` — the kind's signal file is
/// the last of the three parts, so the instruction lands last in what a session
/// reads — and `tests/prompt.rs::the_ending_is_the_embedded_signal_files_bytes`
/// is where that is proved. What is left here is the local half: a Signal step written
/// back into `SKILL.md` would restore the ordering these offsets used to check
/// while defeating the placement, so a second copy has to fail somewhere, and
/// this is the file that would stop noticing.
#[test]
fn the_signal_step_lives_in_its_own_file_rather_than_on_the_loop_page() {
    assert!(
        !GROVE_SKILL.contains("\n**Signal.**"),
        "content/SKILL.md must not carry a Signal step — it composes last from \
         content/SIGNAL.md, and a copy here would arrive a whole mandate early"
    );
    // Anchored on the *first line* rather than on a preceding newline: with the
    // unit markers deleted, `content/SIGNAL.md` opens on its own step and has no
    // line before it to match against.
    assert!(
        SIGNAL_PAGE.starts_with("**Signal.**"),
        "content/SIGNAL.md must carry the Signal step, and open on it"
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
fn commit_step_states_the_boundary_for_both_git_and_jj() {
    let step = normalized(COMMIT_REFERENCE);

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

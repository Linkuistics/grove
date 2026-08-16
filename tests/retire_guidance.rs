const GROVE_SKILL: &str = include_str!("../content/SKILL.md");
/// The Retire step's procedures — harvesting, pruning, and the node close's four
/// steps — which the conditions on the loop page defer to. That pruning is HITL
/// is a condition and stays in `SKILL.md`; the verb it gates is a procedure.
const RETIRE_REFERENCE: &str = include_str!("../content/references/retire.md");
/// The `finish`-narrowed units, which now sit in that kind's reference file
/// rather than at the tail of `SKILL.md`.
const FINISH_REFERENCE: &str = include_str!("../content/references/finish.md");

#[test]
fn node_close_matches_the_confirmation_boundary() {
    let skill = GROVE_SKILL.split_whitespace().collect::<Vec<_>>().join(" ");
    let retire = RETIRE_REFERENCE
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let finish = FINISH_REFERENCE
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // The loop overview diagram stated this a second time as a picture, and went
    // with the rest of the file-reader narrative; the prose below is now the only
    // statement of it.
    assert!(
        skill.contains("The close asks the human nothing"),
        "a node close must stay unconfirmed — it infers done-ness rather than deciding it"
    );
    assert!(
        skill.contains("Instead the session **verifies and reports**"),
        "a node close must describe verify-and-report closure"
    );
    // With chain nodes gone there is one node species, so the close must not
    // reintroduce a per-species branch: a close that skips its `Done when`
    // rollup because a node "is the other kind" silently drops a real check.
    assert!(
        !skill.contains("Brief-less node"),
        "the node close must not restore the two-species discriminator"
    );
    assert!(
        skill.contains("an agent never prunes on its own"),
        "pruning must stay HITL on the page every session reads"
    );
    assert!(
        retire.contains("Only on explicit human confirmation, run"),
        "the prune verb must remain gated on human confirmation"
    );
    assert!(
        finish.contains("waits for explicit human confirmation before any teardown"),
        "the complete finish cycle must remain human-confirmed"
    );
}

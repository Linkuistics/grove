const GROVE_SKILL: &str = include_str!("../plugins/grove/skills/grove/SKILL.md");
/// The Retire step's procedures — harvesting, pruning, and the node close's four
/// steps — which the conditions on the loop page defer to. That pruning is HITL
/// is a condition and stays in `SKILL.md`; the verb it gates is a procedure.
const RETIRE_REFERENCE: &str = include_str!("../plugins/grove/skills/grove/references/retire.md");
/// The `finish` kind's own skill. Its narrowed rules sat at the tail of
/// `SKILL.md`, then in a kind reference file; since `plugin-kind-skills-k17`
/// they are one shipped skill, and `delete-provisioning-k19` deleted the corpus
/// copy this used to read.
const FINISH_SKILL: &str = include_str!("../plugins/grove/skills/grove-finish/SKILL.md");

#[test]
fn node_close_matches_the_confirmation_boundary() {
    let skill = GROVE_SKILL.split_whitespace().collect::<Vec<_>>().join(" ");
    let retire = RETIRE_REFERENCE
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let finish = FINISH_SKILL
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    // The loop overview diagram stated this a second time as a picture, and went
    // with the rest of the file-reader narrative; the prose below is now the only
    // statement of it. It moved to the Retire reference with the rest of the
    // node-close procedure: `SKILL.md` keeps the situation — *a node has no live
    // leaf left* — and this file says what the close is.
    assert!(
        retire.contains("The close asks the human nothing"),
        "a node close must stay unconfirmed — it infers done-ness rather than deciding it"
    );
    assert!(
        skill.contains("When a node has no live leaf left"),
        "and the page every session reads must still send it here when a node empties"
    );
    // *That* it closes without a gate is the condition, so it stays above; *how*
    // the closing session discharges it — check the `Done when`, then report the
    // close in the commit message — is the procedure, and it is asserted against
    // the file that owns it rather than against the skill page that routes there.
    assert!(
        retire.contains("**Check** the node's brief `Done when`"),
        "a node close must verify the charter it is closing"
    );
    assert!(
        retire.contains("**report** the close by naming the node's"),
        "a node close must report itself — the human reviews it after the fact"
    );
    // With chain nodes gone there is one node species, so the close must not
    // reintroduce a per-species branch: a close that skips its `Done when`
    // rollup because a node "is the other kind" silently drops a real check.
    assert!(
        !skill.contains("Brief-less node") && !retire.contains("Brief-less node"),
        "the node close must not restore the two-species discriminator"
    );
    assert!(
        retire.contains("agent never prunes on its own"),
        "pruning must stay HITL where the prune verb is"
    );
    assert!(
        skill.contains("When this leaf's path looks decided against, stop and ask"),
        "and the page every session reads must stop it before it reaches for the verb"
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

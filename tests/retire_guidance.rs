const GROVE_SKILL: &str = include_str!("../content/SKILL.md");

#[test]
fn retire_flowchart_matches_the_confirmation_boundary() {
    let skill = GROVE_SKILL.split_whitespace().collect::<Vec<_>>().join(" ");

    assert!(
        skill.contains("ret[\"Close the node: verify Done when; promote brief up; report close\"]"),
        "the Retire flowchart must describe verify-and-report node closure"
    );
    // With chain nodes gone there is one node species, so the flowchart must not
    // reintroduce a per-species branch: a close that skips its `Done when`
    // rollup because a node "is the other kind" silently drops a real check.
    assert!(
        !skill.contains("Brief-less node"),
        "the Retire flowchart must not restore the two-species discriminator"
    );
    assert!(
        !skill.contains("ret[\"Ask user;"),
        "the Retire flowchart must not restore a routine node-close confirmation"
    );
    assert!(
        skill.contains("Only on explicit human confirmation, run"),
        "pruning must remain human-confirmed"
    );
    assert!(
        skill.contains("waits for explicit human confirmation before any teardown"),
        "the complete finish cycle must remain human-confirmed"
    );
}

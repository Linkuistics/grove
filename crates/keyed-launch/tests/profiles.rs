use std::ffi::OsStr;
use std::fs;

use keyed_launch::{
    conformance, AssignmentValue, Catalog, CompiledWord, Requirement, Selection, Setting, Slot,
    SlotRule, Templates, Vocabulary,
};
use tempfile::TempDir;

fn vocabulary() -> Vocabulary<'static> {
    Vocabulary {
        slots: &[SlotRule {
            name: "payload",
            requirement: Requirement::ExactlyOnce,
        }],
    }
}
fn selection(names: &[&str]) -> Selection {
    Selection {
        profiles: names.iter().map(|s| (*s).into()).collect(),
        origin: None,
    }
}
fn resolve(
    text: &str,
    overlay: Option<&str>,
    names: &[&str],
) -> Result<Templates, keyed_launch::ConfigError> {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("primary.kdl");
    let o = dir.path().join("local.kdl");
    fs::write(&p, text).unwrap();
    if let Some(text) = overlay {
        fs::write(&o, text).unwrap();
    }
    Catalog::load(&p, overlay.map(|_| o.as_path()), vocabulary())?.resolve(&selection(names))
}
fn words(t: &Templates, key: &str) -> Vec<String> {
    t.expand(
        key,
        &[Slot {
            name: "payload",
            value: OsStr::new("native value"),
        }],
    )
    .unwrap()
    .words()
    .iter()
    .map(|w| w.to_str().unwrap().to_owned())
    .collect()
}
const POLICY: &str = r#"config {
    command "a" "agent-a effort=${param.effort} ${payload}" { param "effort" "medium"; }
    command "b" "agent-b effort=${param.effort} ${payload}" { param "effort" "medium"; }
    profile "routes" { route "opaque" "lead"; route "review" "reviewer"; }
    profile "a-led" { bind "lead" "a"; bind "reviewer" "b"; }
    profile "b-led" { bind "lead" "b"; bind "reviewer" "a"; }
    profile "daily" { include "routes" "a-led"; }
    profile "high" { values "a" { param "effort" "high"; }; }
    profile "exception" { route "opaque" { param "effort" "low"; }; }
    profile "remove" { route "opaque" { unset "effort"; }; }
    profile "broken" { bind "lead" "missing"; }
}"#;

#[test]
fn selected_building_blocks_compose_and_preserve_specificity() {
    let t = resolve(POLICY, None, &["daily", "high"]).unwrap();
    assert_eq!(
        words(&t, "opaque"),
        ["agent-a", "effort=high", "native value"]
    );
    assert_eq!(words(&t, "review")[0], "agent-b");
    assert_eq!(
        words(&resolve(POLICY, None, &["daily"]).unwrap(), "opaque")[1],
        "effort=medium"
    );
    let t = resolve(POLICY, None, &["exception", "routes", "a-led", "high"]).unwrap();
    assert_eq!(words(&t, "opaque")[1], "effort=low");
    let t = resolve(
        POLICY,
        None,
        &["daily", "exception", "high", "remove", "b-led"],
    )
    .unwrap();
    assert_eq!(words(&t, "opaque")[0], "agent-b");
    assert_eq!(words(&t, "opaque")[1], "effort=medium");
    assert_eq!(words(&t, "review")[0], "agent-a");
    assert!(t
        .inspect()
        .histories
        .iter()
        .flat_map(|h| &h.assignments)
        .any(|a| a.value == AssignmentValue::Unset));
    assert!(resolve(POLICY, None, &["broken", "daily"]).is_ok());
    let error = resolve(POLICY, None, &["daily", "broken"]).err().unwrap();
    assert_eq!(error.diagnostics()[0].category, "unknown_reference");
    assert_eq!(error.diagnostics()[0].occurrence_chain[0].profile, "broken");
}

#[test]
fn diamonds_and_repeated_selections_reapply_with_distinct_origins() {
    let text = r#"config {
        command "a" "runner ${param.p} ${payload}" { param "p" "default"; }
        bind "lead" "a"
        route "opaque" "lead"
        profile "base" { values "a" { param "p" "base"; }; }
        profile "left" { values "a" { param "p" "left"; }; include "base"; }
        profile "right" { include "base"; }
    }"#;
    let t = resolve(text, None, &["left", "right", "base"]).unwrap();
    assert_eq!(words(&t, "opaque")[1], "base");
    let v = t.inspect();
    assert_eq!(
        v.profile_occurrences
            .iter()
            .map(|o| (o.profile.as_str(), o.parent, o.selection_index))
            .collect::<Vec<_>>(),
        [
            ("left", None, 0),
            ("base", Some(0), 0),
            ("right", None, 1),
            ("base", Some(2), 1),
            ("base", None, 2)
        ]
    );
    assert!(v.profile_occurrences[0].via.is_none());
    assert!(v.profile_occurrences[1].via.is_some());
    let h = v.histories.iter().find(|h| matches!(&h.setting, Setting::CommandParameter { parameter, .. } if parameter == "p")).unwrap();
    assert_eq!(
        h.assignments
            .iter()
            .map(|a| v.origins[a.origin].occurrence)
            .collect::<Vec<_>>(),
        [Some(1), Some(0), Some(3), Some(4)]
    );
    assert!(h.assignments.windows(2).all(|a| a[0].order < a[1].order));
    assert_eq!(
        v.origins[h.assignments[0].origin].span,
        v.origins[h.assignments[2].origin].span
    );
    let winning = h.assignments.last().unwrap().origin;
    assert!(v.commands[0].words[1].origins.contains(&winning));
    assert!(!v.commands[0].words[1]
        .origins
        .contains(&h.assignments[0].origin));
}

#[test]
fn include_errors_show_closed_chains_and_do_not_disappear_under_overrides() {
    let text = r#"config {
        profile "a" { include "b"; }
        profile "b" { include "a"; }
        profile "unknown" { include "absent"; }
        profile "valid" {}
    }"#;
    let e = resolve(text, None, &["a", "unknown", "valid"])
        .err()
        .unwrap();
    assert_eq!(e.diagnostics().len(), 2);
    let cycle = e
        .diagnostics()
        .iter()
        .find(|d| d.category == "include_cycle")
        .unwrap();
    assert_eq!(
        cycle
            .occurrence_chain
            .iter()
            .map(|o| o.profile.as_str())
            .collect::<Vec<_>>(),
        ["a", "b", "a"]
    );
    assert!(cycle.message.contains("a -> b -> a"));
    assert_eq!(cycle.related.len(), 2);
    assert!(cycle.occurrence_chain[0].via.is_none());
    assert!(cycle.occurrence_chain[1..].iter().all(|o| o.via.is_some()));
    let unknown = e
        .diagnostics()
        .iter()
        .find(|d| d.category == "unknown_profile")
        .unwrap();
    assert_eq!(unknown.occurrence_chain.len(), 2);
    assert_eq!(unknown.occurrence_chain[1].profile, "absent");
    assert!(resolve(text, None, &["valid"]).is_ok());
}

#[test]
fn personal_authority_is_checked_after_profiles_and_before_local_targets() {
    let text = r#"config {
        command "a" "runner ${param.p} ${payload}" { param "p"; }
        bind "lead" "a"
        profile "patch" { route "opaque" { param "p" "personal"; }; }
        profile "target" { route "opaque" "lead"; }
        profile "nested" { include "patch"; }
    }"#;
    let local = r#"config { route "opaque" "lead" { param "p" "local"; }; }"#;
    let e = resolve(text, Some(local), &["nested"]).err().unwrap();
    let d = e
        .diagnostics()
        .iter()
        .find(|d| d.category == "missing_target")
        .unwrap();
    assert_eq!(d.key.as_deref(), Some("opaque"));
    assert_eq!(
        d.occurrence_chain
            .iter()
            .map(|o| o.profile.as_str())
            .collect::<Vec<_>>(),
        ["nested", "patch"]
    );
    assert_eq!(
        words(
            &resolve(text, Some(local), &["patch", "target"]).unwrap(),
            "opaque"
        )[1],
        "local"
    );
    assert_eq!(
        words(&resolve(text, Some(local), &["target"]).unwrap(), "opaque")[1],
        "local"
    );
    let inactive = resolve(text, Some(local), &[]).unwrap();
    assert!(inactive.require("opaque").is_err());
    assert_eq!(inactive.inspect().non_admitted_keys[0].key, "opaque");
    let e = resolve(text, None, &["target"]).err().unwrap();
    assert_eq!(e.diagnostics()[0].category, "missing_parameter");
    assert_eq!(e.diagnostics()[0].occurrence_chain[0].profile, "target");
}

#[test]
fn selected_snapshots_and_conformance_remain_independent_of_files_and_catalog() {
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("primary.kdl");
    fs::write(&p, POLICY).unwrap();
    let catalog = Catalog::load(&p, None, vocabulary()).unwrap();
    fs::remove_file(&p).unwrap();
    let selected = selection(&["daily", "high"]);
    assert!(conformance::check(&catalog, &selected).passed());
    assert!(!conformance::check(&catalog, &selection(&["broken"])).passed());
    assert!(!conformance::check(&catalog, &Selection::default()).passed());
    let snapshot = catalog.resolve(&selected).unwrap();
    assert_eq!(
        snapshot.inspect(),
        catalog.resolve(&selected).unwrap().inspect()
    );
    drop(catalog);
    let from_view: Vec<_> = snapshot
        .inspect()
        .commands
        .iter()
        .find(|c| c.key == "opaque")
        .unwrap()
        .words
        .iter()
        .map(|w| match &w.word {
            CompiledWord::Literal(s) => s.clone(),
            CompiledWord::Slot(_) => "native value".into(),
        })
        .collect();
    assert_eq!(from_view, words(&snapshot, "opaque"));
}

#[test]
fn surviving_errors_validate_even_without_routes_and_removed_names_are_harmless() {
    let text = r#"config {
        command "a" "runner ${param.p} ${payload}" { param "p"; }
        command "bad" "'unfinished"
        profile "unused-binding" { bind "x" "bad"; }
        profile "unused-values" { values "a" { param "unknown" "x"; }; }
        profile "repair" { values "a" { unset "unknown"; }; bind "x" "a"; }
        profile "route" { route "opaque" "x"; }
    }"#;
    let e = resolve(text, None, &["unused-binding", "unused-values"])
        .err()
        .unwrap();
    assert_eq!(e.diagnostics().len(), 2);
    assert!(e
        .diagnostics()
        .iter()
        .all(|d| !d.occurrence_chain.is_empty()));
    assert!(resolve(text, None, &["unused-binding", "unused-values", "repair"]).is_ok());
    let e = resolve(text, None, &["repair", "route"]).err().unwrap();
    assert_eq!(e.diagnostics()[0].category, "missing_parameter");
    assert_eq!(e.diagnostics()[0].occurrence_chain[0].profile, "route");
}

#[test]
fn repeated_includes_and_captured_selection_origins_are_preserved() {
    let text = r#"config {
        command "a" "runner ${payload}"
        profile "base" { bind "lead" "a"; route "opaque" "lead"; }
        profile "twice" { include "base" "base"; }
        select "twice" "twice"
    }"#;
    let dir = TempDir::new().unwrap();
    let p = dir.path().join("p.kdl");
    fs::write(&p, text).unwrap();
    let c = Catalog::load(&p, None, vocabulary()).unwrap();
    let t = c.resolve(c.primary_selection().unwrap()).unwrap();
    assert_eq!(t.inspect().profile_occurrences.len(), 6);
    assert_eq!(
        t.inspect().profile_occurrences[3].via,
        c.primary_selection().unwrap().origin
    );
    let history = t
        .inspect()
        .histories
        .iter()
        .find(|h| matches!(h.setting, Setting::RouteTarget { .. }))
        .unwrap();
    assert_eq!(history.assignments.len(), 4);
    assert_eq!(
        t.inspect().origins[t.inspect().commands[0].origins[0]].occurrence,
        Some(5)
    );
}

//! These fixtures catch omitted reader/pre-effect validation independently of
//! the conformance kit. Node-part policies are outside the bounded models.
use std::{fmt, fs, path::Path};

use ordinal_fs_tree::fs::{Reading, Writing};
use ordinal_fs_tree::reference::{Label, Parts, Status, SyllabusName};
use ordinal_fs_tree::{
    EntryName, EntryNameExt, Error, Found, Key, NameView, NewEntry, Ordinal, PositionedSpecies,
    Target, Verdict,
};

#[derive(Clone, Debug)]
enum Name<const POLICY: u8> {
    Positioned(SyllabusName),
    Own(&'static str),
}
type Required = Name<1>;
type Permissive = Name<0>;

#[derive(Debug)]
struct Grammar(String);
impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for Grammar {}
impl<const POLICY: u8> fmt::Display for Name<POLICY> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Positioned(n) => n.fmt(f),
            Self::Own(s) => f.write_str(s),
        }
    }
}
impl<const POLICY: u8> EntryName for Name<POLICY> {
    type Parts = Parts;
    type Err = Grammar;
    fn parse(name: &str, found: Found) -> Verdict<Self, Grammar> {
        let own = match name {
            "_ROOT" => Some("_ROOT"),
            "_NODE" => Some("_NODE"),
            "_SPECIAL" => Some("_SPECIAL"),
            _ => None,
        };
        if let Some(own) = own {
            return if found == Found::File {
                Verdict::Entry(Self::Own(own))
            } else {
                Verdict::Malformed(Grammar("own file must be regular".into()))
            };
        }
        match SyllabusName::parse(name, found) {
            Verdict::Entry(n) if n.triple().is_some() => Verdict::Entry(Self::Positioned(n)),
            Verdict::Malformed(e) => Verdict::Malformed(Grammar(e.to_string())),
            Verdict::Reserved(e) => Verdict::Reserved(Grammar(e.to_string())),
            _ => Verdict::Foreign,
        }
    }
    fn compose(o: Ordinal, k: Key, p: Parts) -> Self {
        Self::Positioned(SyllabusName::compose(o, k, p))
    }
    fn view(&self) -> NameView<'_, Parts> {
        match self {
            Self::Positioned(n) => n.view(),
            Self::Own(_) => NameView::Distinguished,
        }
    }
    fn positioned_species(p: &Parts) -> PositionedSpecies {
        SyllabusName::positioned_species(p)
    }
    fn validate_distinguished(node: Option<&Self>, children: &[Self]) -> Result<(), Grammar> {
        if POLICY == 0 {
            return Ok(());
        }
        if POLICY == 2 {
            return if children
                .first()
                .is_none_or(|name| name.to_string() == "_ROOT")
            {
                Ok(())
            } else {
                Err(Grammar("root must be first".into()))
            };
        }
        let expected = match node {
            None => "_ROOT",
            Some(n) if n.triple().unwrap().parts.label().as_str() == "special" => "_SPECIAL",
            Some(n)
                if n.triple().unwrap().parts.label().as_str() == "ordinal"
                    && n.triple().unwrap().ordinal != Ordinal::FIRST =>
            {
                "_SPECIAL"
            }
            Some(_) => "_NODE",
        };
        if children.len() == 1 && children[0].to_string() == expected {
            Ok(())
        } else {
            Err(Grammar(format!(
                "expected exactly one {expected}; found {}",
                children.len()
            )))
        }
    }
}
fn leaf() -> NewEntry<Parts> {
    NewEntry::new(
        Parts::lesson(Status::Draft, Label::new("lesson").unwrap()),
        b"leaf bytes".to_vec(),
    )
}
fn node(label: &str) -> Parts {
    Parts::module(Label::new(label).unwrap())
}
fn open(root: &Path) -> ordinal_fs_tree::fs::WriteGuard<Required> {
    match ordinal_fs_tree::fs::write(root).unwrap() {
        Writing::Tree(g) => g,
        Writing::Vacancy(_) => panic!("missing tree"),
    }
}
fn init(
    root: &Path,
    own: Option<Required>,
    entries: Vec<NewEntry<Parts>>,
) -> Result<ordinal_fs_tree::Report<Required>, Error<Required>> {
    match ordinal_fs_tree::fs::write(root).unwrap() {
        Writing::Vacancy(v) => v.initialize(own.map(|n| (n, b"root bytes".to_vec())), entries),
        Writing::Tree(_) => panic!("existing tree"),
    }
}
fn inventory(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn visit(root: &Path, at: &Path, out: &mut Vec<(String, Vec<u8>)>) {
        for entry in fs::read_dir(at).unwrap() {
            let path = entry.unwrap().path();
            out.push((
                path.strip_prefix(root).unwrap().display().to_string(),
                if path.is_dir() {
                    vec![]
                } else {
                    fs::read(&path).unwrap()
                },
            ));
            if path.is_dir() {
                visit(root, &path, out);
            }
        }
    }
    let mut out = vec![];
    visit(root, root, &mut out);
    out.sort();
    out
}

#[test]
fn readers_reject_missing_misplaced_and_competing_names_before_any_search() {
    for exclusive in [false, true] {
        for names in [vec![], vec!["_ROOT"], vec!["_NODE", "_SPECIAL"]] {
            let temp = tempfile::tempdir().unwrap();
            let root = temp.path().join("tree");
            fs::create_dir(&root).unwrap();
            fs::write(root.join("_ROOT"), "root").unwrap();
            fs::write(root.join("01-draft-lesson-i1.md"), "early match").unwrap();
            let later = root.join("02-topic-i2");
            fs::create_dir(&later).unwrap();
            for name in names {
                fs::write(later.join(name), "own").unwrap();
            }
            let result = if exclusive {
                ordinal_fs_tree::fs::write::<Required>(&root).map(|_| ())
            } else {
                ordinal_fs_tree::fs::read::<Required>(&root).map(|_| ())
            };
            assert!(
                result.is_err(),
                "later malformed level must prevent exposing an early match"
            );
            let error = result.unwrap_err();
            assert!(error.to_string().contains("_NODE"));
            assert!(format!("{error:?}").contains(later.to_str().unwrap()));
            assert!(std::error::Error::source(&error).is_some());
        }
    }
}

#[test]
fn permissive_domain_cannot_expose_competing_names() {
    let temp = tempfile::tempdir().unwrap();
    for name in ["_ROOT", "_NODE", "_SPECIAL"] {
        fs::write(temp.path().join(name), "bytes").unwrap();
    }
    for exclusive in [false, true] {
        let result = if exclusive {
            ordinal_fs_tree::fs::write::<Permissive>(temp.path()).map(|_| ())
        } else {
            ordinal_fs_tree::fs::read::<Permissive>(temp.path()).map(|_| ())
        };
        let error = result.expect_err("library cardinality rule must be independent");
        for name in ["_ROOT", "_NODE", "_SPECIAL"] {
            assert!(error.to_string().contains(name));
        }
    }
}

#[test]
fn initialization_checks_root_and_every_initial_node_before_creating_root() {
    for (own, entries) in [
        (None, vec![]),
        (Some(Required::Own("_NODE")), vec![]),
        (
            Some(Required::Own("_ROOT")),
            vec![leaf(), NewEntry::empty(node("topic"))],
        ),
    ] {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("tree");
        assert!(init(&root, own, entries).is_err());
        assert!(!root.exists(), "invalid initialization must leave no root");
    }
}

#[test]
fn all_node_constructors_and_part_rewrites_refuse_without_effects() {
    for operation in 0..7 {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("tree");
        init(&root, Some(Required::Own("_ROOT")), vec![leaf()]).unwrap();
        if operation == 6 {
            open(&root)
                .promote(Key::new(1), node("topic"), Required::Own("_NODE"), None)
                .unwrap();
        }
        let before = inventory(&root);
        let guard = open(&root);
        let result = match operation {
            0 => guard.append(Target::Root, NewEntry::empty(node("topic"))),
            1 => guard.append_many(Target::Root, vec![leaf(), NewEntry::empty(node("topic"))]),
            2 => guard.insert(Target::Root, Ordinal::FIRST, NewEntry::empty(node("topic"))),
            3 => guard.promote(Key::new(1), node("topic"), Required::Own("_ROOT"), None),
            4 => guard.promote(
                Key::new(1),
                node("topic"),
                Required::Own("_NODE"),
                Some(NewEntry::empty(node("child"))),
            ),
            5 => guard.promote(Key::new(1), node("special"), Required::Own("_NODE"), None),
            6 => guard.rewrite(Key::new(1), node("special")),
            _ => unreachable!(),
        };
        assert!(
            result.is_err(),
            "operation {operation} must refuse an invalid final level"
        );
        assert_eq!(
            inventory(&root),
            before,
            "operation {operation} changed tree on refusal"
        );
    }
}

#[test]
fn different_root_and_node_names_initialize_promote_and_preserve_bytes() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("tree");
    init(&root, Some(Required::Own("_ROOT")), vec![leaf()]).unwrap();
    open(&root)
        .promote(
            Key::new(1),
            node("special"),
            Required::Own("_SPECIAL"),
            Some(leaf()),
        )
        .unwrap();
    open(&root).append(Target::Root, leaf()).unwrap();
    open(&root)
        .append_many(Target::Key(Key::new(1)), vec![leaf(), leaf()])
        .unwrap();
    open(&root)
        .insert(Target::Root, Ordinal::FIRST, leaf())
        .unwrap();
    let Reading::Tree(tree) = ordinal_fs_tree::fs::read::<Required>(&root).unwrap() else {
        panic!("missing")
    };
    assert_eq!(
        tree.snapshot()
            .root()
            .distinguished()
            .unwrap()
            .name()
            .to_string(),
        "_ROOT"
    );
    assert_eq!(
        fs::read(root.join("02-special-i1/_SPECIAL")).unwrap(),
        b"leaf bytes"
    );
    assert_eq!(fs::read(root.join("_ROOT")).unwrap(), b"root bytes");
}

#[test]
fn kit_checks_fixture_verdicts_instead_of_deriving_expectations() {
    use ordinal_fs_tree::conformance::{check, LevelSample, Obligation};
    let sample = LevelSample::<Required> {
        node: None,
        distinguished: vec![],
        accepted: true,
    };
    let report = check(&[], &[], &[], &[sample]);
    assert!(
        report.violations().any(|finding| matches!(
            finding,
            ordinal_fs_tree::conformance::Finding::Violated {
                obligation: Obligation::LevelValidationMatchesSamples,
                ..
            }
        )),
        "the deliberately wrong fixture must fail: {report}"
    );
}

#[test]
fn shifting_a_node_rechecks_its_final_name_before_effects() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().join("tree");
    init(&root, Some(Required::Own("_ROOT")), vec![leaf()]).unwrap();
    open(&root)
        .promote(Key::new(1), node("ordinal"), Required::Own("_NODE"), None)
        .unwrap();
    let before = inventory(&root);
    let error = open(&root)
        .insert(Target::Root, Ordinal::FIRST, leaf())
        .unwrap_err();
    let Error::InvalidLevel { path, .. } = error else {
        panic!("wrong error: {error:?}")
    };
    assert_eq!(path, root.join("02-ordinal-i1"));
    assert_eq!(inventory(&root), before);
}

#[test]
fn independent_level_fixtures_cover_missing_competing_misplaced_and_node_parts() {
    use ordinal_fs_tree::conformance::{check, LevelSample};
    let ordinary = Required::compose(Ordinal::FIRST, Key::new(1), node("topic"));
    let special = Required::compose(Ordinal::FIRST, Key::new(2), node("special"));
    let mut levels = vec![];
    for (context, own, wrong) in [
        (None, "_ROOT", "_NODE"),
        (Some(ordinary), "_NODE", "_ROOT"),
        (Some(special), "_SPECIAL", "_NODE"),
    ] {
        for (names, accepted) in [
            (vec![], false),
            (vec![own], true),
            (vec![wrong], false),
            (vec![own, wrong], false),
            (vec![wrong, own], false),
        ] {
            levels.push(LevelSample {
                node: context.clone(),
                distinguished: names.into_iter().map(Required::Own).collect(),
                accepted,
            });
        }
    }
    check::<Required>(
        &[
            ("_ROOT", Found::File),
            ("01-topic-i1", Found::Dir),
            ("01-draft-lesson-i2.md", Found::File),
        ],
        &[
            (Ordinal::FIRST, Key::new(1), node("topic")),
            (
                Ordinal::FIRST,
                Key::new(2),
                Parts::lesson(Status::Draft, Label::new("lesson").unwrap()),
            ),
        ],
        &[
            Required::Own("_ROOT"),
            Required::Own("_NODE"),
            Required::Own("_SPECIAL"),
        ],
        &levels,
    )
    .assert_conforming();
}

#[test]
fn kit_exposes_order_dependent_level_validation() {
    use ordinal_fs_tree::conformance::{check, Finding, LevelSample, Obligation};
    let report = check::<Name<2>>(
        &[],
        &[],
        &[],
        &[LevelSample {
            node: None,
            distinguished: vec![
                Name::Own("_ROOT"),
                Name::Own("_NODE"),
                Name::Own("_SPECIAL"),
            ],
            accepted: true,
        }],
    );
    assert!(
        report.violations().any(|finding| matches!(
            finding,
            Finding::Violated {
                obligation: Obligation::LevelValidationMatchesSamples,
                ..
            }
        )),
        "{report}"
    );
}

use std::{fs, path::Path};

use grove_loop::{Reading, Selection};
use ordinal_fs_tree::Key;

fn fixture(names: &[&str]) -> tempfile::TempDir {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("_BRIEF.md"), "root").unwrap();
    for name in names {
        let path = root.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, "body").unwrap();
    }
    work
}

fn select(work: &Path, excluded: Option<u32>) -> Result<Option<Selection>, grove_loop::Error> {
    let Reading::Tree(tree) = grove_loop::read(work)? else {
        panic!("fixture has a tree");
    };
    grove_loop::select_snapshot(tree.root(), tree.snapshot(), excluded.map(Key::new))
}

#[test]
fn exclusion_filters_candidates_without_changing_depth_first_order() {
    let work = fixture(&[
        "01-finish--finish-k1.md",
        "02-k2/_branch.md",
        "02-k2/01-impl--nested-k3.md",
        "02-k2/02-DONE-impl--done-k4.md",
        "03-impl--later-k5.md",
        "04-ABANDONED-impl--abandoned-k6.md",
    ]);
    for (excluded, path, handle) in [
        (None, "02-k2/01-impl--nested-k3.md", "nested-k3"),
        (Some(1), "02-k2/01-impl--nested-k3.md", "nested-k3"),
        (Some(2), "02-k2/01-impl--nested-k3.md", "nested-k3"),
        (Some(3), "03-impl--later-k5.md", "later-k5"),
        (Some(4), "02-k2/01-impl--nested-k3.md", "nested-k3"),
        (Some(5), "02-k2/01-impl--nested-k3.md", "nested-k3"),
        (Some(6), "02-k2/01-impl--nested-k3.md", "nested-k3"),
        (Some(99), "02-k2/01-impl--nested-k3.md", "nested-k3"),
    ] {
        let selection = select(work.path(), excluded).unwrap().unwrap();
        assert_eq!(
            selection.path,
            work.path().join(".grove").join(path),
            "{excluded:?}"
        );
        assert_eq!(selection.handle.to_string(), handle);
        assert_eq!(selection.kind.label(), "impl");
    }
}

#[test]
fn excluding_the_last_ordinary_leaf_selects_the_finish_remainder() {
    let work = fixture(&["01-finish--finish-k1.md", "02-impl--work-k2.md"]);
    let selection = select(work.path(), Some(2)).unwrap().unwrap();
    assert_eq!(selection.handle.to_string(), "finish-k1");
    assert_eq!(selection.kind.label(), "finish");
    assert_eq!(
        selection.path,
        work.path().join(".grove/01-finish--finish-k1.md")
    );
}

#[test]
fn no_candidates_returns_none_without_creating_a_finish() {
    for (names, excluded) in [
        (vec![], None),
        (vec!["01-DONE-impl--done-k1.md", "02-k2/_empty.md"], None),
        (vec!["01-impl--work-k1.md"], Some(1)),
        (vec!["01-finish--finish-k1.md"], Some(1)),
    ] {
        let work = fixture(&names);
        let before = fs::read_dir(work.path().join(".grove")).unwrap().count();
        assert_eq!(select(work.path(), excluded).unwrap(), None);
        assert_eq!(
            fs::read_dir(work.path().join(".grove")).unwrap().count(),
            before
        );
        assert!(!work.path().join(".jj").exists());
    }
    let work = fixture(&["01-finish--finish-k1.md"]);
    assert_eq!(
        select(work.path(), None)
            .unwrap()
            .unwrap()
            .handle
            .to_string(),
        "finish-k1"
    );
}

#[test]
fn duplicate_keys_are_refused_before_exclusion_in_every_item_class() {
    for names in [
        vec!["01-impl--work-k7.md", "02-impl--other-k7.md"],
        vec!["01-DONE-impl--done-k7.md", "02-ABANDONED-impl--old-k7.md"],
        vec!["01-k7/_branch.md", "01-k7/01-impl--child-k7.md"],
        vec!["01-k7/_branch.md", "02-k7/_other.md"],
    ] {
        let work = fixture(&names);
        for excluded in [None, Some(7), Some(99)] {
            let error = select(work.path(), excluded).unwrap_err().to_string();
            assert!(error.contains("duplicate key k7"), "{error}");
            assert!(
                error.contains("permanent"),
                "repair advice missing: {error}"
            );
        }
    }
}

#[test]
fn multiple_live_finishes_are_refused_before_exclusion() {
    let work = fixture(&[
        "01-impl--work-k3.md",
        "02-finish--finish-k1.md",
        "03-finish--again-k2.md",
    ]);
    for excluded in [None, Some(1), Some(2), Some(3)] {
        let error = select(work.path(), excluded).unwrap_err().to_string();
        assert!(error.contains("multiple live `finish` leaves"), "{error}");
    }
}

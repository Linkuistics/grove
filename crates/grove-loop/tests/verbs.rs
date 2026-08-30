//! **Test seam 1: `grove-loop`'s public interface, exercised without the other
//! four modules** (`docs/specs/module-decomposition.md`, *Test seams*).
//!
//! Nothing here reaches a module of the crate, spawns a binary, or asks a
//! configuration file anything. It opens a worktree, calls verbs, and looks at
//! the directory afterwards — which is the whole of what a consumer can do, and
//! therefore the whole of what this crate promises.
//!
//! The crate's own `#[cfg(test)]` modules are not this seam and do not replace
//! it: they exercise `task_tree`, `task_grow` and `tree_lifecycle` from inside,
//! where a signature can be convenient rather than public. What they cannot see
//! is whether the *surface* composes — whether the arms `read` and `write`
//! answer with can be taken apart, whether a verb that says it takes a `&Tree`
//! can be reached with one, and whether the types refuse what the prose says
//! they refuse.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use grove_loop::verbs::{self, Resolution, Signalled};
use grove_loop::{Handle, Kind, Reading, Reference, Slug, Sought, TreeWrite, Writing};
use tempfile::TempDir;

/// A jj worktree with no grove in it.
///
/// jj rather than a bare directory because two verbs reach the VCS seam and
/// every fixture should meet the same worktree they do; the store itself neither
/// knows nor asks.
fn worktree() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().expect("a temporary directory");
    let root = tmp.path().join("worktree");
    fs::create_dir_all(&root).expect("creating the worktree");
    jj(&root, &["git", "init", "--colocate"]);
    jj(&root, &["config", "set", "--repo", "user.name", "fixture"]);
    jj(
        &root,
        &[
            "config",
            "set",
            "--repo",
            "user.email",
            "fixture@example.invalid",
        ],
    );
    (tmp, root)
}

fn jj(root: &Path, arguments: &[&str]) {
    let output = Command::new("jj")
        .args(arguments)
        .current_dir(root)
        .output()
        .unwrap_or_else(|error| panic!("running jj {arguments:?}: {error}"));
    assert!(
        output.status.success(),
        "jj {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn slug(text: &str) -> Slug {
    Slug::new(text).expect("a fixture slug must be well-formed")
}

fn kind(label: &str) -> Kind {
    Kind::new(label).expect("a fixture kind must be well-formed")
}

fn reference(text: &str) -> Reference {
    Reference::parse(text).expect("a fixture reference must parse")
}

/// Scaffold a grove through the public surface, as its one caller does.
fn scaffold(root: &Path) -> verbs::Initialized {
    let Writing::Vacancy(vacancy) = grove_loop::write(root).expect("opening a fresh worktree")
    else {
        panic!("a fresh worktree must open as a vacancy");
    };
    verbs::root_init(vacancy, &slug("plan"), &Kind::requirements()).expect("scaffolding")
}

fn writable(root: &Path) -> TreeWrite {
    match grove_loop::write(root).expect("opening for writing") {
        Writing::Tree(tree) => tree,
        Writing::Vacancy(_) => panic!("expected a live grove"),
    }
}

fn name_of(path: &Path) -> String {
    path.file_name()
        .expect("a path with a filename")
        .to_string_lossy()
        .into_owned()
}

fn listing(root: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(root.join(".grove"))
        .expect("the grove root is readable")
        .map(|entry| entry.expect("a readable entry").file_name())
        .map(|name| name.to_string_lossy().into_owned())
        .collect();
    names.sort();
    names
}

// ---- the opening ------------------------------------------------------------

/// **The two arms are the whole of grove's refusal to clobber or to imagine a
/// tree**, and neither is a check inside a verb.
#[test]
fn opening_answers_a_vacancy_before_a_grove_and_a_tree_after_it() {
    let (_tmp, root) = worktree();

    assert!(
        matches!(grove_loop::read(&root).unwrap(), Reading::Vacant),
        "a worktree with no grove reads as vacant"
    );
    assert!(matches!(
        grove_loop::write(&root).unwrap(),
        Writing::Vacancy(_)
    ));

    scaffold(&root);

    assert!(matches!(grove_loop::read(&root).unwrap(), Reading::Tree(_)));
    assert!(matches!(
        grove_loop::write(&root).unwrap(),
        Writing::Tree(_)
    ));
}

/// The opening takes a **worktree**, and joins `.grove` itself — so no consumer
/// can spell the grove root a second way.
#[test]
fn the_opening_joins_the_grove_root_itself() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);

    assert_eq!(initialized.brief, root.join(".grove").join("BRIEF.md"));
    assert_eq!(writable(&root).root(), root.join(".grove"));
}

/// `root_init` consumes a `Vacancy`, and a live grove yields none — so there is
/// no call to make, rather than a call that is refused.
#[test]
fn a_live_grove_offers_no_way_to_scaffold_over_it() {
    let (_tmp, root) = worktree();
    scaffold(&root);

    let Writing::Tree(_) = grove_loop::write(&root).unwrap() else {
        panic!("a live grove must not open as a vacancy");
    };
}

// ---- reading ----------------------------------------------------------------

#[test]
fn root_init_writes_the_charter_and_one_first_leaf_that_pick_answers() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);

    assert_eq!(name_of(&initialized.brief), "BRIEF.md");
    assert_eq!(
        name_of(&initialized.first_leaf),
        "01-requirements--plan-k1.md"
    );
    assert_eq!(
        listing(&root),
        vec!["01-requirements--plan-k1.md", "BRIEF.md"]
    );

    let Reading::Tree(tree) = grove_loop::read(&root).unwrap() else {
        panic!("a scaffolded grove reads as a tree");
    };
    let Sought::Match(selection) = verbs::pick(&tree).unwrap() else {
        panic!("a fresh grove is never mistaken for a finished one");
    };
    assert_eq!(selection.path, initialized.first_leaf);
    assert_eq!(selection.handle.to_string(), "plan-k1");
    assert_eq!(selection.kind, Kind::requirements());
}

/// A grove with no live leaf answers `Sought::Nothing` — the finish trigger, in
/// the store's word rather than an option of the loop's own invention.
#[test]
fn a_grove_with_no_live_leaf_is_a_search_that_matched_nothing() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    verbs::leaf_retire(&writable(&root), &initialized.first_leaf).unwrap();

    let Reading::Tree(tree) = grove_loop::read(&root).unwrap() else {
        panic!("still a tree");
    };
    assert_eq!(verbs::pick(&tree).unwrap(), Sought::Nothing);
    assert_eq!(verbs::kind(&tree, None).unwrap(), Sought::Nothing);
}

#[test]
fn kind_and_brief_chain_read_the_leaf_they_are_given() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    let Reading::Tree(tree) = grove_loop::read(&root).unwrap() else {
        panic!("a tree");
    };

    assert_eq!(
        verbs::kind(&tree, Some(&initialized.first_leaf)).unwrap(),
        Sought::Match(Kind::requirements())
    );
    assert_eq!(
        verbs::brief_chain(&tree, &initialized.first_leaf).unwrap(),
        vec![initialized.brief]
    );
}

// ---- resolve ----------------------------------------------------------------

#[test]
fn resolve_covers_the_root_a_key_a_handle_and_a_path() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    let leaf = initialized.first_leaf.clone();
    let Reading::Tree(tree) = grove_loop::read(&root).unwrap() else {
        panic!("a tree");
    };

    assert_eq!(
        verbs::resolve(&tree, &reference(".")).unwrap(),
        Sought::Match(Resolution::Root),
        "`.` is the root, which is not an entry"
    );
    // A **path** is `Reference`'s fourth form and `leaf_add` / `leaf_insert`
    // take it; `resolve` deliberately does not, because a path is already the
    // answer `resolve` exists to produce.
    for spelling in ["1", "[1]", "plan-k1"] {
        match verbs::resolve(&tree, &reference(spelling)).unwrap() {
            Sought::Match(Resolution::Entry(entry)) => {
                assert_eq!(entry.path, leaf, "{spelling:?}");
                assert_eq!(entry.handle.to_string(), "plan-k1", "{spelling:?}");
                assert_eq!(entry.kind, Some(Kind::requirements()), "{spelling:?}");
            }
            other => panic!("{spelling:?}: expected one entry, got {other:?}"),
        }
    }
    assert_eq!(
        verbs::resolve(&tree, &reference("ghost")).unwrap(),
        Sought::Nothing
    );
}

/// **Ambiguity is an answer, not an error** — and each match carries the handle
/// the caller re-asks with.
#[test]
fn resolve_answers_ambiguity_with_every_match() {
    let (_tmp, root) = worktree();
    scaffold(&root);
    verbs::leaf_add(
        &writable(&root),
        &Reference::root(),
        &slug("twin"),
        &[kind("impl"), kind("design")],
    )
    .unwrap();

    let Reading::Tree(tree) = grove_loop::read(&root).unwrap() else {
        panic!("a tree");
    };
    match verbs::resolve(&tree, &reference("twin")).unwrap() {
        Sought::Match(Resolution::Ambiguous(matches)) => {
            let handles: Vec<String> = matches.iter().map(|m| m.handle.to_string()).collect();
            assert_eq!(handles, vec!["twin-k2", "twin-k3"]);
        }
        other => panic!("expected an ambiguous match, got {other:?}"),
    }
}

#[test]
fn an_empty_reference_names_nothing() {
    assert!(Reference::parse("").is_err());
    assert!(Reference::parse("   ").is_err());
}

// ---- writing ----------------------------------------------------------------

/// One `leaf_add` per kind, in order, as **one** unit — and every verb returns
/// the paths it wrote, because its caller writes the commit message by hand.
#[test]
fn leaf_add_lands_one_leaf_per_kind_at_consecutive_positions_and_keys() {
    let (_tmp, root) = worktree();
    scaffold(&root);

    let written = verbs::leaf_add(
        &writable(&root),
        &Reference::root(),
        &slug("survey"),
        &[
            kind("research-a"),
            kind("research-b"),
            kind("combine-research"),
        ],
    )
    .unwrap();

    assert_eq!(
        written.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
        vec![
            "02-research-a--survey-k2.md",
            "03-research-b--survey-k3.md",
            "04-combine-research--survey-k4.md",
        ]
    );
}

#[test]
fn leaf_insert_takes_the_slot_and_reports_every_sibling_it_shifted() {
    let (_tmp, root) = worktree();
    scaffold(&root);
    verbs::leaf_add(
        &writable(&root),
        &Reference::root(),
        &slug("later"),
        &[kind("impl")],
    )
    .unwrap();

    let inserted = verbs::leaf_insert(
        &writable(&root),
        &reference("2"),
        &slug("earlier"),
        &kind("impl"),
    )
    .unwrap();

    assert_eq!(name_of(&inserted.path), "02-impl--earlier-k3.md");
    assert_eq!(inserted.renumbered.len(), 1);
    let shifted = &inserted.renumbered[0];
    assert_eq!(shifted.from_position, 2);
    assert_eq!(shifted.to_position, 3);
    assert_eq!(shifted.from_name(), "02-impl--later-k2.md");
    assert_eq!(shifted.to_name(), "03-impl--later-k2.md");
}

#[test]
fn leaf_decompose_turns_a_leaf_into_a_node_with_one_first_child() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);

    let decomposed = verbs::leaf_decompose(
        &writable(&root),
        &initialized.first_leaf,
        &slug("first"),
        Some(&kind("impl")),
    )
    .unwrap();

    assert_eq!(name_of(&decomposed.brief), "BRIEF.md");
    assert_eq!(name_of(&decomposed.first_child), "01-impl--first-k2.md");
    // The key is preserved: the entity that was the leaf became the node.
    assert_eq!(
        name_of(decomposed.brief.parent().unwrap()),
        "01-plan-k1",
        "the node keeps the leaf's ordinal and key"
    );
}

#[test]
fn leaf_retire_marks_in_place_and_leaves_the_bytes_alone() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    let before = fs::read_to_string(&initialized.first_leaf).unwrap();

    let marked = verbs::leaf_retire(&writable(&root), &initialized.first_leaf).unwrap();

    assert_eq!(name_of(&marked), "01-DONE-requirements--plan-k1.md");
    assert_eq!(fs::read_to_string(&marked).unwrap(), before);
}

#[test]
fn leaf_prune_marks_every_live_leaf_and_names_the_done_ones_it_left() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    let decomposed = verbs::leaf_decompose(
        &writable(&root),
        &initialized.first_leaf,
        &slug("first"),
        Some(&kind("impl")),
    )
    .unwrap();
    let node = decomposed.brief.parent().unwrap().to_path_buf();
    verbs::leaf_add(
        &writable(&root),
        &reference("plan-k1"),
        &slug("second"),
        &[kind("impl")],
    )
    .unwrap();
    verbs::leaf_retire(&writable(&root), &decomposed.first_child).unwrap();

    let pruned = verbs::leaf_prune(&writable(&root), &node).unwrap();

    assert_eq!(
        pruned.marked.iter().map(|p| name_of(p)).collect::<Vec<_>>(),
        vec!["02-ABANDONED-impl--second-k3.md"]
    );
    assert_eq!(
        pruned
            .left_done
            .iter()
            .map(|p| name_of(p))
            .collect::<Vec<_>>(),
        vec!["01-DONE-impl--first-k2.md"],
        "abandoning work that was finished would misreport it"
    );
}

/// `finish` is the driver's own kind and no operator verb may write it.
#[test]
fn the_growing_verbs_refuse_the_drivers_reserved_kind() {
    let (_tmp, root) = worktree();
    scaffold(&root);

    let error = verbs::leaf_add(
        &writable(&root),
        &Reference::root(),
        &slug("x"),
        &[kind("finish")],
    )
    .unwrap_err()
    .to_string();

    assert!(error.contains("driver-reserved"), "got {error}");
    assert_eq!(
        listing(&root),
        vec!["01-requirements--plan-k1.md", "BRIEF.md"],
        "a refusal leaves the tree byte-identical"
    );
}

// ---- the two that reach outward ---------------------------------------------

/// `finish_commit` reaches the VCS seam: it revalidates, deletes through the
/// store, and takes one commit.
#[test]
fn finish_commit_tears_the_tree_down_and_commits_it() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    verbs::leaf_retire(&writable(&root), &initialized.first_leaf).unwrap();
    // The finish sentinel is the driver's to write, and this is the seam it
    // writes it through.
    let selection = grove_loop::driver::materialize_finish(&root).unwrap();
    assert_eq!(selection.kind, Kind::finish());
    jj(&root, &["commit", "-m", "fixture"]);

    let workspace = grove_loop::Workspace::resolve(&root).unwrap();
    let commit = verbs::finish_commit(&workspace, &selection.handle).unwrap();

    assert!(
        !commit.change_id.is_empty(),
        "the teardown names its commit"
    );
    assert!(!root.join(".grove").exists(), "the tree is gone");
}

/// A handle that is not the live finish leaf's is refused, and nothing is
/// deleted.
#[test]
fn finish_commit_refuses_a_handle_that_is_not_the_live_finish_leaf() {
    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    verbs::leaf_retire(&writable(&root), &initialized.first_leaf).unwrap();
    grove_loop::driver::materialize_finish(&root).unwrap();
    jj(&root, &["commit", "-m", "fixture"]);

    let workspace = grove_loop::Workspace::resolve(&root).unwrap();
    let error = verbs::finish_commit(&workspace, &Handle::parse("other-k9").unwrap())
        .unwrap_err()
        .to_string();

    assert!(error.contains("does not match"), "got {error}");
    assert!(root.join(".grove").exists(), "nothing was torn down");
}

/// `complete` reaches the runner's channel — and outside a loop it is a no-op
/// that says so.
#[test]
fn complete_writes_the_flag_it_was_given_a_channel_for() {
    let tmp = TempDir::new().unwrap();
    let channel = tmp.path().join("signal");

    assert_eq!(
        verbs::complete(Some(&channel), false).unwrap(),
        Signalled::Wrote(channel.clone())
    );
    assert!(channel.exists());

    let done = tmp.path().join("done");
    assert_eq!(
        verbs::complete(Some(&done), true).unwrap(),
        Signalled::Wrote(done.clone())
    );
    assert!(
        fs::read_to_string(&done).unwrap().contains("done"),
        "`--done` must reach the channel as a finish disposition"
    );
}

/// **The channel is answerable before the write**, because the caller admits its
/// session against the channel it is about to signal.
#[test]
fn the_channel_can_be_asked_for_before_it_is_written() {
    let path = PathBuf::from("/tmp/explicit.signal");
    assert_eq!(verbs::signal_channel(Some(&path)), Some(path));
    // The meta-grove's own guard force-clears the variable to the empty string,
    // which is *no* loop context rather than a degenerate path.
    assert_eq!(verbs::signal_channel(None), None);
}

/// **A prune that stops partway says what it already marked, and that rerunning
/// finishes it.**
///
/// `docs/adr/bulk-marks-are-not-atomic.md` accepts *N* rewrites under *N* guards
/// on one argument — the marks are the state, so re-running converges — and that
/// argument is only available to an operator who can see the residue. A bare
/// store refusal shows none of it.
///
/// The mid-run failure is induced by a read-only directory at the **second**
/// position, so the first mark lands and the second cannot. Marks run in
/// pre-order, which is what puts them in that order.
#[cfg(unix)]
#[test]
fn a_prune_that_stops_partway_names_what_it_already_marked() {
    use std::os::unix::fs::PermissionsExt;

    let (_tmp, root) = worktree();
    let initialized = scaffold(&root);
    let outer = verbs::leaf_decompose(
        &writable(&root),
        &initialized.first_leaf,
        &slug("alpha"),
        Some(&kind("impl")),
    )
    .unwrap();
    let node = outer.brief.parent().unwrap().to_path_buf();
    let beta = verbs::leaf_add(
        &writable(&root),
        &reference("plan-k1"),
        &slug("beta"),
        &[kind("impl")],
    )
    .unwrap();
    let inner = verbs::leaf_decompose(
        &writable(&root),
        &beta[0],
        &slug("deep"),
        Some(&kind("impl")),
    )
    .unwrap();
    let inner_node = inner.brief.parent().unwrap().to_path_buf();

    let mode = fs::metadata(&inner_node).unwrap().permissions();
    fs::set_permissions(&inner_node, fs::Permissions::from_mode(0o500)).unwrap();

    let refusal = verbs::leaf_prune(&writable(&root), &node)
        .unwrap_err()
        .to_string();

    fs::set_permissions(&inner_node, mode).unwrap();

    assert!(
        refusal.contains("stopped partway"),
        "the refusal must say it stopped partway: {refusal}"
    );
    assert!(
        refusal.contains("01-ABANDONED-impl--alpha-k2.md"),
        "the refusal must name the leaf it already marked: {refusal}"
    );
    assert!(
        refusal.contains("rerun"),
        "the refusal must say what finishes it: {refusal}"
    );
}

/// A second verb on one `TreeWrite` reopens, and sees what the first left.
///
/// The wrapper holds the guard it was made with only until the first verb spends
/// it; everything after that is a fresh opening. What must not change is the
/// answer: the second verb acts on the tree the first one left, not on the
/// snapshot the wrapper was made from.
#[test]
fn a_second_verb_on_one_opening_acts_on_the_tree_the_first_left() {
    let (_tmp, root) = worktree();
    scaffold(&root);
    let tree = writable(&root);

    verbs::leaf_add(&tree, &Reference::root(), &slug("one"), &[kind("impl")]).unwrap();
    let second = verbs::leaf_add(&tree, &Reference::root(), &slug("two"), &[kind("impl")]).unwrap();

    assert_eq!(
        name_of(&second[0]),
        "03-impl--two-k3.md",
        "the second add took the position and key the first one left, not the one \
         the wrapper's original snapshot would have given it"
    );
}

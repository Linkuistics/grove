//! The grow verbs' own tests, carried across from the path-walking appender
//! when `growing-k33` moved the verbs onto the library.
//!
//! Most are the same fixtures asserting the same outcomes: what a verb does to a
//! tree is what this stage promises not to change. Three groups could not come
//! across unchanged, and each is a finding rather than an adjustment —
//! `.grove/07-grove-flip-k28/BRIEF.md` records them:
//!
//!   * the **repository assertion** inverted, because the shift is now
//!     `rename(2)` on every lane (`docs/adr/grove-does-not-stage-its-own-renames.md`);
//!   * three **destination-machinery** tests stayed behind with the appender's
//!     own helpers, which they actually exercised, and died with it in
//!     `sweep-k37`; the injected post-claim failure went earlier, with the
//!     rollback it pinned;
//!   * the **refusals** for a parent that is not in the tree are now the
//!     resolver's rather than the appender's, and say something different.

use super::*;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// A [`Kind`] for a test that needs one, by its label.
///
/// A kind is an **open token** since `open-kind-k20`, so a test names the token
/// it means rather than a variant, and an invalid one is a test bug that panics
/// here rather than a compile error somewhere else.
fn a_kind(label: &str) -> Kind {
    Kind::new(label).expect("a test kind must be well-formed")
}

/// A [`Slug`] for a test that needs one. The verbs take the validated type since
/// `loop-crate-verbs-k21` — one type owns the name (principle 3), so validation
/// happens where the text is read and not once per verb.
fn a_slug(text: &str) -> Slug {
    Slug::new(text).expect("a test slug must be well-formed")
}

/// The exclusive guard a write verb now takes, opened from a grove root.
///
/// The verbs stopped opening the tree themselves at `loop-crate-verbs-k21`: the
/// lock a verb needs is visible in its signature, and the caller holds it. Every
/// fixture below still names a root, so this is where the two meet.
fn guard(grove_root: &Path) -> Guard {
    open(grove_root).expect("opening the tree for a write verb")
}

/// The same, as a `Result`, for the fixtures whose subject *is* the opening.
fn open(grove_root: &Path) -> Result<Guard> {
    task_tree::write(grove_root)
}

/// `leaf-add` with **one** kind, which is what most fixtures below want: the
/// list form is the research pair's, and a one-kind list is the ordinary add.
fn add_one(root: &Path, parent: &str, slug: &str, kind: &str) -> Result<PathBuf> {
    add(root, parent, slug, &[a_kind(kind)]).map(|mut paths| paths.remove(0))
}

/// `leaf-add` as its own CLI drives it: read the slug, open the tree, call the
/// verb — **in that order**, which is what the fixtures about a refused slug and
/// an absent root are asserting. The verb takes a validated [`Slug`] and an open
/// guard since `loop-crate-verbs-k21`, so both refusals happen at the boundary
/// that owns them rather than inside it.
fn add(root: &Path, parent: &str, slug: &str, kinds: &[Kind]) -> Result<Vec<PathBuf>> {
    let slug = Slug::new(slug).map_err(|error| anyhow::anyhow!("slug {slug:?}: {error}"))?;
    leaf_add(open(root)?, parent, &slug, kinds)
}

/// The research pair, as `leaf-add` now spells it: three kinds in one call.
///
/// The pair tests below were `leaf-add-pair`'s own and are kept verbatim through
/// this helper, because the guarantee they check — one unit, contiguous
/// ordinals, consecutive keys, nothing on a refusal — is exactly what
/// generalising the verb had to preserve (`open-kind-k20`). The three tokens are
/// the *methodology's*, spelled here in a test rather than in the machinery.
fn add_pair(root: &Path, parent: &str, stem: &str) -> Result<Vec<PathBuf>> {
    add(
        root,
        parent,
        stem,
        &[
            a_kind("research-a"),
            a_kind("research-b"),
            a_kind("combine-research"),
        ],
    )
}

/// A fresh `.grove/` directory (no repository — the grow verbs write files and rename
/// them, and neither needs a repository).
fn grove() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join(".grove");
    fs::create_dir_all(&root).unwrap();
    (tmp, root)
}

/// A `.grove/` inside a real jj repo — the only kind of working tree Grove
/// drives (`docs/adr/jj-is-the-only-lane.md`). The **instrument** rather than a
/// prerequisite: entries rename whether or not the repository knows them, and the
/// repository is what lets a test observe what the index did — which, since the
/// flip, is nothing.
fn jj_grove() -> (TempDir, PathBuf) {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path().to_path_buf();
    run_jj(
        &repo,
        &["--config", "git.colocate=false", "git", "init", "."],
    );
    let root = repo.join(".grove");
    fs::create_dir_all(&root).unwrap();
    (tmp, root)
}

fn run_jj(repo: &Path, args: &[&str]) {
    let out = Command::new("jj")
        .current_dir(repo)
        .args([
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
        ])
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "jj {args:?} failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
}

/// Commit everything under the grove, putting the entries in the revision the
/// working copy sits on.
fn commit_all(root: &Path) {
    run_jj(root.parent().unwrap(), &["commit", "-m", "fixture"]);
}

/// Write a leaf/brief stub file with a position-free `# <handle>` header.
fn touch(dir: &Path, name: &str, header: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, format!("# {header}\n")).unwrap();
    path
}

/// Write a file with an explicit body (for realistic multi-line content).
fn touch_body(dir: &Path, name: &str, content: &str) -> PathBuf {
    let path = dir.join(name);
    fs::write(&path, content).unwrap();
    path
}

/// Create a node directory with its `BRIEF.md`, returning the directory path.
fn mknode(dir: &Path, name: &str, handle: &str) -> PathBuf {
    let path = dir.join(name);
    fs::create_dir_all(&path).unwrap();
    fs::write(path.join("BRIEF.md"), format!("# {handle} — brief\n")).unwrap();
    path
}

fn name_of(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().into_owned()
}

fn names_of(paths: &[PathBuf]) -> Vec<String> {
    paths.iter().map(|path| name_of(path)).collect()
}

fn body(path: &Path) -> String {
    fs::read_to_string(path).unwrap()
}

/// A path spelled as the argument a verb takes.
fn at(path: &Path) -> &str {
    path.to_str().unwrap()
}

/// The directory's child names (files and subdirs), lexically sorted.
fn list(dir: &Path) -> Vec<String> {
    let mut names: Vec<String> = fs::read_dir(dir)
        .unwrap()
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name != "FORMAT")
        .collect();
    names.sort();
    names
}

/// The names the **committed revision** holds for the grove, lexically sorted.
/// Distinct from [`list`] (what is on disk).
fn committed(root: &Path) -> Vec<String> {
    let out = Command::new("jj")
        .current_dir(root)
        .args(["--ignore-working-copy", "file", "list", "-r", "@-", "."])
        .output()
        .unwrap();
    let mut names: Vec<String> = String::from_utf8_lossy(&out.stdout)
        .lines()
        .map(ToString::to_string)
        .filter(|name| !name.ends_with("/FORMAT") && name != "FORMAT")
        .collect();
    names.sort();
    names
}

// ---- leaf-add ---------------------------------------------------------------

#[test]
fn add_root_level_child_gets_position_01_and_first_key() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let got = add_one(&g, ".", "survey", "impl").unwrap();
    assert_eq!(name_of(&got), "01-impl--survey-k1.md");
}

#[test]
fn add_appends_gapless_after_existing_root_children() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    touch(&g, "02-impl--b-k2.md", "b-k2");
    let got = add_one(&g, ".", "c", "impl").unwrap();
    assert_eq!(name_of(&got), "03-impl--c-k3.md");
}

#[test]
fn add_child_under_a_node_appends_after_existing_children() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let node = mknode(&g, "02-build-k2", "build-k2");
    touch(&node, "01-impl--x-k3.md", "x-k3");
    let got = add_one(&g, at(&node), "y", "impl").unwrap();
    assert_eq!(name_of(&got), "02-impl--y-k4.md");
    assert_eq!(name_of(got.parent().unwrap()), "02-build-k2");
}

#[test]
fn add_first_child_under_a_childless_node() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let node = mknode(&g, "02-build-k2", "build-k2");
    let got = add_one(&g, at(&node), "first", "impl").unwrap();
    assert_eq!(name_of(&got), "01-impl--first-k3.md");
}

#[test]
fn add_addresses_its_parent_by_key_handle_and_slug_alike() {
    // The reference grammar, resolved against the guard's own snapshot rather
    // than against a directory listing under grove's guard. Every form has to
    // reach the same node.
    for parent in ["2", "[2]", "[2]-build", "build-k2", "build"] {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        mknode(&g, "02-build-k2", "build-k2");
        let got = add_one(&g, parent, "y", "impl").unwrap();
        assert_eq!(
            name_of(got.parent().unwrap()),
            "02-build-k2",
            "{parent:?} must name the node"
        );
    }
}

#[test]
fn add_refuses_an_ambiguous_parent_slug_and_lists_the_keys() {
    // The one outcome grove's reference grammar has and a key does not, so the
    // library has no counterpart for it and grove must say it itself.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    mknode(&g, "01-build-k1", "build-k1");
    mknode(&g, "02-build-k2", "build-k2");
    let err = add_one(&g, "build", "y", "impl").unwrap_err().to_string();
    assert!(err.contains("ambiguous"), "got {err}");
    assert!(err.contains("[1]") && err.contains("[2]"), "got {err}");
    assert_eq!(
        list(&g),
        vec!["01-build-k1", "02-build-k2", "BRIEF.md"],
        "nothing was created"
    );
}

#[test]
fn add_assigns_fresh_key_as_max_over_whole_tree_plus_one() {
    // Keys are global, not per-node: the new key is max(key) + 1 across the
    // whole tree, including a deeper subtree's higher key.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let design = mknode(&g, "01-design-k1", "design-k1");
    touch(&design, "01-impl--deep-k7.md", "deep-k7"); // a high key in another subtree
    let build = mknode(&g, "02-build-k2", "build-k2");
    let got = add_one(&g, at(&build), "y", "impl").unwrap();
    assert_eq!(name_of(&got), "01-impl--y-k8.md");
}

#[test]
fn add_counts_done_children_so_a_retired_slot_is_never_reused() {
    // A `DONE` child still occupies its position — the next child is 02, not 01.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let node = mknode(&g, "02-build-k2", "build-k2");
    touch(&node, "01-DONE-impl--x-k3.md", "x-k3");
    let got = add_one(&g, at(&node), "y", "impl").unwrap();
    assert_eq!(name_of(&got), "02-impl--y-k4.md");
}

#[test]
fn add_counts_abandoned_children_so_a_pruned_slot_is_never_reused() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let node = mknode(&g, "02-build-k2", "build-k2");
    touch(&node, "01-ABANDONED-impl--x-k3.md", "x-k3");
    let got = add_one(&g, at(&node), "y", "impl").unwrap();
    assert_eq!(name_of(&got), "02-impl--y-k4.md");
}

#[test]
fn add_counts_node_dir_siblings_when_numbering() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    mknode(&g, "01-design-k1", "design-k1");
    let got = add_one(&g, ".", "build", "impl").unwrap();
    assert_eq!(name_of(&got), "02-impl--build-k2.md");
}

#[test]
fn add_preserves_a_gap_a_hand_edit_left_rather_than_filling_it() {
    // The library appends at `max + 1` and never at `count + 1`: density is
    // preserved by every operation and established by none, so a level a hand
    // edit left gapped keeps its gap. Grove's own appender counted the same way,
    // and this pins that the two agree — a fill would collide the moment the
    // missing ordinal came back.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    touch(&g, "05-impl--b-k2.md", "b-k2");
    let got = add_one(&g, ".", "c", "impl").unwrap();
    assert_eq!(name_of(&got), "06-impl--c-k3.md");
}

#[test]
fn add_writes_kind_in_filename_and_not_in_body() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let got = add_one(&g, ".", "survey", "impl").unwrap();
    let text = body(&got);
    assert!(
        text.starts_with("# survey-k1\n"),
        "header is the position-free handle; got {text:?}"
    );
    assert_eq!(name_of(&got), "01-impl--survey-k1.md");
    assert!(!text.contains("**Kind:**"), "got {text:?}");
    assert!(!text.contains("**Harness:**"), "got {text:?}");
}

#[test]
fn every_grow_verb_writes_a_handle_that_matches_its_own_filename() {
    // **The check the prediction earns.** The library allocates the key and
    // grove renders it into the body *before* the library has composed the name
    // (`task_tree::next_key`), so a header contradicting its own filename is the
    // failure mode this arrangement risks — silently, and forever. Asserted over
    // all three verbs at once, and read back off the tree rather than off the
    // paths the verbs returned, because an insert moves what an append left.
    use crate::task_name::TaskName;
    use ordinal_fs_tree::{EntryName, Found, Verdict};

    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let node = mknode(&g, "01-design-k1", "design-k1");
    touch(&node, "01-impl--deep-k9.md", "deep-k9");
    touch(&g, "02-impl--a-k2.md", "a-k2");

    add_one(&g, ".", "added", "impl").unwrap();
    add_pair(&g, at(&node), "surveyed").unwrap();
    leaf_insert(guard(&g), "2", &a_slug("inserted"), &a_kind("impl")).unwrap();

    let mut checked = 0;
    for directory in [&g, &node] {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            let name = name_of(&path);
            let Verdict::Entry(TaskName::Positioned { key, parts, .. }) =
                TaskName::parse(&name, Found::File)
            else {
                continue; // the charter, and the format witness
            };
            assert!(
                body(&path).starts_with(&format!("# {}-k{}\n", parts.slug(), key.get())),
                "the handle in {name} must carry the key its filename does: {:?}",
                body(&path).lines().next()
            );
            checked += 1;
        }
    }
    assert_eq!(
        checked, 7,
        "three at the root and four in the node — every leaf was read back"
    );
}

#[test]
fn add_planning_kind_writes_planning_filename() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let got = add_one(&g, ".", "design", "planning").unwrap();
    assert_eq!(name_of(&got), "01-planning--design-k1.md");
    assert!(!body(&got).contains("**Kind:**"));
}

#[test]
fn add_refuses_a_parent_that_names_nothing_in_the_tree() {
    // **Changed message, and the change is the resolver taking the question
    // over.** A path that does not exist is no longer *not a node directory*
    // reported by the appender; it is a reference that matched neither namespace,
    // reported before any operation is planned (clause 1).
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let missing = g.join("09-nope-k9");
    let err = add_one(&g, at(&missing), "y", "impl")
        .unwrap_err()
        .to_string();
    assert!(err.contains("no entry matches"), "got {err}");
}

#[test]
fn add_errors_when_parent_is_a_leaf_file_not_a_node() {
    // A leaf is a *file* — you must decompose it into a node before adding under
    // it. Grove's own refusal, in front of the library's `TargetNotNode`, and
    // reading the library's own predicate off the snapshot: a node is an entry
    // whose contents are `Some`.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let leaf = touch(&g, "02-impl--build-k2.md", "build-k2");
    let err = add_one(&g, at(&leaf), "y", "impl").unwrap_err().to_string();
    assert!(err.contains("parent is not a node directory"), "got {err}");
}

#[test]
fn add_errors_when_the_parent_is_the_charter_brief() {
    // The reason grove must keep its own check in front of `TargetNotNode`: a
    // `BRIEF.md` is an entry with no key at all, so it could not be handed to
    // the library as a target however the refusal were worded.
    let (_t, g) = grove();
    let brief = touch(&g, "BRIEF.md", "root — brief");
    let err = add_one(&g, at(&brief), "y", "impl")
        .unwrap_err()
        .to_string();
    assert!(err.contains("parent is not a node directory"), "got {err}");
}

#[test]
fn a_review_chain_is_cut_one_flat_sibling_at_a_time() {
    // The whole lazy shape, end to end, through the verb that builds it. The
    // producer's last act appends its review; the review's last act appends the
    // integration. Each is an ordinary `leaf-add` at the parent's next free
    // position, so the three land contiguously as flat siblings — no node
    // directory, no constructor, and nothing that knows the three compose one
    // artifact.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");

    let producer = add_one(&g, ".", "sync", "design").unwrap();
    let review = add_one(&g, ".", "sync", "review-design").unwrap();
    let integrate = add_one(&g, ".", "sync", "integrate-review-design").unwrap();

    assert_eq!(
        names_of(&[producer, review, integrate]),
        vec![
            "01-design--sync-k1.md",
            "02-review-design--sync-k2.md",
            "03-integrate-review-design--sync-k3.md",
        ],
        "contiguous flat siblings off one stem"
    );
    assert_eq!(
        list(&g),
        vec![
            "01-design--sync-k1.md",
            "02-review-design--sync-k2.md",
            "03-integrate-review-design--sync-k3.md",
            "BRIEF.md",
        ],
        "and nothing else — no node directory was created for them"
    );
}

#[test]
fn a_review_step_cut_after_unrelated_work_still_appends_at_the_end() {
    // The cost the flat shape accepts, stated rather than defended against
    // (flat-lazy-review, *Known consequence, accepted*). A review decided on
    // once a later leaf already exists lands **after** that leaf, not beside its
    // producer — grove validates no cross-leaf grammar and contiguity was always
    // a convention.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    add_one(&g, ".", "sync", "design").unwrap();
    add_one(&g, ".", "unrelated", "impl").unwrap();

    let review = add_one(&g, ".", "sync", "review-design").unwrap();

    assert_eq!(name_of(&review), "03-review-design--sync-k3.md");
}

#[test]
fn an_integration_cut_with_insert_lands_beside_the_review_it_integrates() {
    // The renumber the methodology's easy case produces: one blocking leaf
    // directly after the review. Nothing here knows that an `integrate-review-*`
    // leaf belongs next to its review — the rule is guidance, and this pins the
    // *mechanics* of obeying it, because the alternative (`leaf-add`, which
    // would put the integration at 04) is equally well-formed.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    add_one(&g, ".", "sync", "design").unwrap();
    add_one(&g, ".", "sync", "review-design").unwrap();
    let unrelated = add_one(&g, ".", "unrelated", "impl").unwrap();

    let inserted = leaf_insert(
        guard(&g),
        at(&unrelated),
        &a_slug("sync"),
        &a_kind("integrate-review-design"),
    )
    .unwrap();

    assert_eq!(
        name_of(&inserted.path),
        "03-integrate-review-design--sync-k4.md",
        "the integration takes the slot after its review"
    );
    assert_eq!(
        list(&g),
        vec![
            "01-design--sync-k1.md",
            "02-review-design--sync-k2.md",
            "03-integrate-review-design--sync-k4.md",
            "04-impl--unrelated-k3.md",
            "BRIEF.md",
        ],
        "and the unrelated leaf shifts down, keeping its own key"
    );
    assert_eq!(
        inserted.renumbered.len(),
        1,
        "only the displaced sibling moved"
    );
}

#[test]
fn add_refuses_a_parent_that_is_not_a_grove_entry_at_all() {
    // **Changed message, same refusal.** A bare `notes/` was once *not a node
    // directory* because its name did not parse; it is now a path under the root
    // that names no entry of the tree, which is what is actually wrong with it —
    // the grammar disclaims the name, so no walk ever reaches it.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let bare = g.join("notes");
    fs::create_dir_all(&bare).unwrap();
    let err = add_one(&g, at(&bare), "y", "impl").unwrap_err().to_string();
    assert!(
        err.contains("not a Grove leaf or node directory"),
        "got {err}"
    );
}

#[test]
fn add_errors_on_invalid_slug() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    assert!(add_one(&g, ".", "BRIEF", "impl").is_err());
    assert!(add_one(&g, ".", "Bad Slug", "impl").is_err());
}

#[test]
fn add_errors_when_grove_root_absent() {
    let (_t, g) = grove();
    let missing = g.join("nope");
    let err = add_one(&missing, ".", "y", "impl").unwrap_err();
    assert!(
        err.to_string().contains("grove root not found"),
        "got {err}"
    );
}

/// The driver's own kind, refused at the verb's boundary before anything is
/// planned — the counterpart of [`insert_rejects_finish`], and its header
/// carries why the *before inspecting the tree* phrasing went.
#[test]
fn add_rejects_finish() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let err = add_one(&g, ".", "x", "finish").unwrap_err().to_string();
    assert!(err.contains("driver-reserved"), "got {err}");
    assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
}

/// The driver's own kind is refused whatever the tree holds — here, before the
/// target `1` is even looked for.
///
/// It used to be phrased *before inspecting the tree*, against a missing root.
/// That premise is gone: since `loop-crate-verbs-k21` the caller opens the tree
/// and hands the verb a guard, so a missing root has no `leaf-insert` to reach —
/// [`crate::write`] answers a vacancy, and the vacancy offers only `root-init`.
/// What is left to assert is the refusal itself, and that nothing was created.
#[test]
fn insert_rejects_finish() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    commit_all(&g);
    let err = leaf_insert(guard(&g), "1", &a_slug("x"), &a_kind("finish"))
        .unwrap_err()
        .to_string();
    assert!(err.contains("driver-reserved"), "got {err}");
    assert_eq!(list(&g), vec!["01-impl--a-k1.md", "BRIEF.md"]);
}

// ---- leaf-add-pair ----------------------------------------------------------
//
// The one surviving composite verb. Two properties carry it:
//
//   * **the steps are flat siblings at consecutive positions**, off one stem,
//     with the pair's three fixed kinds (flat-lazy-review);
//   * **one call, one mutation** — a run that fails leaves *no leaf at all*, so
//     a live prefix of a pair never masquerades as a hand-cut partial one.
//
// The second is now `append_many`'s, not grove's: one snapshot answers every
// ordinal and key, and the interpreter unwinds its own effects. So the tests
// below assert the outcome and no longer the machinery, which is the point of
// the flip.

#[test]
fn pair_emits_three_flat_siblings_with_the_fixed_research_kinds() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let paths = add_pair(&g, ".", "sync-survey").unwrap();
    assert_eq!(
        names_of(&paths),
        vec![
            "01-research-a--sync-survey-k1.md",
            "02-research-b--sync-survey-k2.md",
            "03-combine-research--sync-survey-k3.md",
        ],
        "three siblings at consecutive positions, three consecutive keys — \
         one bare stem throughout, with only the kind field telling them apart"
    );
    for step in &paths {
        assert_eq!(
            name_of(step.parent().unwrap()),
            ".grove",
            "every step is a direct child of the parent — no node directory"
        );
    }
    assert_eq!(
        list(&g),
        vec![
            "01-research-a--sync-survey-k1.md",
            "02-research-b--sync-survey-k2.md",
            "03-combine-research--sync-survey-k3.md",
            "BRIEF.md",
        ],
        "the run created its three leaves and nothing else"
    );
}

#[test]
fn pair_appends_after_existing_siblings_and_under_a_node() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    let node = mknode(&g, "02-build-k2", "build-k2");
    touch(&node, "01-impl--x-k3.md", "x-k3");
    let paths = add_pair(&g, at(&node), "api").unwrap();
    assert_eq!(
        names_of(&paths),
        vec![
            "02-research-a--api-k4.md",
            "03-research-b--api-k5.md",
            "04-combine-research--api-k6.md",
        ],
        "the steps continue the parent's positions; the keys continue the whole tree"
    );
    assert_eq!(name_of(paths[0].parent().unwrap()), "02-build-k2");
}

#[test]
fn pair_rejects_a_malformed_stem_in_its_own_right() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    assert!(add_pair(&g, ".", "foo-").is_err());
    assert_eq!(list(&g), vec!["BRIEF.md"], "nothing was created");
}

// ---- one call, one mutation -------------------------------------------------

#[test]
fn a_run_that_cannot_read_its_parent_level_creates_nothing_at_all() {
    // A task-shaped entry whose on-disk species contradicts its name is a
    // malformed tree, and the *read* refuses it — so the guard the pair takes
    // never yields a snapshot and nothing is planned, let alone written. The
    // wording is now the domain grammar's own `SpeciesMismatch`, which is
    // `refusals-k30`'s *write no second wording* arriving as a message assertion.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    fs::create_dir(g.join("01-research-a--survey-k1.md")).unwrap();

    let err = add_pair(&g, ".", "survey").unwrap_err().to_string();

    assert!(
        err.contains("01-research-a--survey-k1.md"),
        "the error names the entry standing in the way: {err}"
    );
    assert_eq!(
        list(&g),
        vec!["01-research-a--survey-k1.md", "BRIEF.md"],
        "only the squatter and the brief — no half-built pair left behind"
    );
}

#[test]
fn an_unwritable_third_destination_refuses_the_whole_run() {
    // A hazard **specific to composite verbs**: the derived names are longer
    // than the stem the caller validated, and unequally so, so a third name can
    // cross `NAME_MAX` (255) while the first two clear it.
    //
    // Grove no longer sweeps destinations up front, so this is no longer an
    // up-front refusal — it is the interpreter's rollback, which is the arm the
    // sweep existed to keep the operator out of. The observable outcome is what
    // the test is for and it is unchanged: not one of the three survives.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    // `NN-research-a-<stem>-k<key>.md` is stem+20 at a single-digit key;
    // `NN-combine-research-<stem>-k<key>.md` is stem+26. At 235 the run plans
    // two 255-byte names and one 261-byte name: the first two fit, the third
    // does not.
    let stem = "a".repeat(235);

    assert!(add_pair(&g, ".", &stem).is_err());

    assert_eq!(
        list(&g),
        vec!["BRIEF.md"],
        "not even the two leaves whose names would have fit"
    );
}

#[test]
fn a_run_that_cannot_get_three_fresh_keys_creates_nothing_at_all() {
    // Key exhaustion changed hands: grove's own allocator refused an exhausted
    // keyspace up front, and `Refusal::KeysExhausted` now does — planned from
    // the same snapshot, before any effect is built, so it is still true that
    // nothing lands. The message is the library's, printed unchanged.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--old-k4294967294.md", "old-k4294967294");

    let err = add_pair(&g, ".", "survey").unwrap_err().to_string();

    assert!(
        err.contains("keys") || err.contains("key"),
        "the refusal is about the keyspace: {err}"
    );
    assert_eq!(
        list(&g),
        vec!["01-impl--old-k4294967294.md", "BRIEF.md"],
        "not even the first leaf"
    );
}

#[test]
fn a_level_at_the_last_ordinal_refuses_rather_than_wrapping() {
    // The ordinal's own exhaustion, which grove's appender never checked at all:
    // `next_child_position` added one to a `u32` unguarded. The library refuses.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "4294967295-impl--last-k1.md", "last-k1");

    assert!(add_one(&g, ".", "next", "impl").is_err());

    assert_eq!(
        list(&g),
        vec!["4294967295-impl--last-k1.md", "BRIEF.md"],
        "nothing was created"
    );
}

#[test]
fn a_failed_run_leaves_the_next_call_a_clean_slate() {
    // Why all-or-nothing is worth having: a retry after a failure must produce
    // the shape, not a second copy of it under new positions and keys.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let squatter = g.join("01-research-a--survey-k1.md");
    fs::create_dir(&squatter).unwrap();
    assert!(add_pair(&g, ".", "survey").is_err());
    fs::remove_dir(&squatter).unwrap();

    let paths = add_pair(&g, ".", "survey").unwrap();

    assert_eq!(
        names_of(&paths),
        vec![
            "01-research-a--survey-k1.md",
            "02-research-b--survey-k2.md",
            "03-combine-research--survey-k3.md",
        ],
        "the retry got the positions and keys the failed run had planned"
    );
    assert_eq!(
        list(&g).len(),
        4,
        "exactly one pair plus the brief — no duplicate from the failed attempt"
    );
}

#[test]
fn a_refused_run_does_not_consume_positions_or_keys() {
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    assert!(add_pair(&g, ".", "foo-").is_err());
    let got = add_one(&g, ".", "plain", "impl").unwrap();
    assert_eq!(name_of(&got), "01-impl--plain-k1.md");
}

#[test]
fn a_shape_is_byte_identical_to_the_same_leaves_cut_by_hand() {
    // Constraint 6: a generated pair is the same standard markdown/filesystem
    // shape a human can cut and annotate. The verb's only contribution is that
    // the three land or none does.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let generated = add_pair(&g, ".", "survey").unwrap();
    let generated_bodies: Vec<String> = generated.iter().map(|path| body(path)).collect();
    for path in &generated {
        fs::remove_file(path).unwrap();
    }

    let by_hand = [
        add_one(&g, ".", "survey", "research-a").unwrap(),
        add_one(&g, ".", "survey", "research-b").unwrap(),
        add_one(&g, ".", "survey", "combine-research").unwrap(),
    ];

    assert_eq!(names_of(&generated), names_of(&by_hand));
    let hand_bodies: Vec<String> = by_hand.iter().map(|path| body(path)).collect();
    assert_eq!(generated_bodies, hand_bodies);
}

// ---- leaf-insert ------------------------------------------------------------

#[test]
fn insert_at_occupied_position_shifts_occupant_and_later_siblings_keys_preserved() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    touch(&g, "02-impl--b-k2.md", "b-k2");
    touch(&g, "03-impl--c-k3.md", "c-k3");
    commit_all(&g);
    let inserted = leaf_insert(
        guard(&g),
        at(&g.join("02-impl--b-k2.md")),
        &a_slug("new"),
        &a_kind("impl"),
    )
    .unwrap();
    assert_eq!(name_of(&inserted.path), "02-impl--new-k4.md"); // fresh key, not a reused one
    assert_eq!(
        list(&g),
        vec![
            "01-impl--a-k1.md",
            "02-impl--new-k4.md",
            "03-impl--b-k2.md",
            "04-impl--c-k3.md",
            "BRIEF.md",
        ]
    );
}

#[test]
fn insert_cascades_a_sibling_node_subtree_riding_along_byte_identical() {
    // The headline: inserting ahead of a sibling *node* shifts only that node's
    // own directory name — its `BRIEF.md` and every grandchild stay
    // byte-identical, name *and* key, because a shift is one rename of one
    // directory.
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    let mid = mknode(&g, "02-mid-k3", "mid-k3");
    let grandchild = touch_body(
        &mid,
        "01-impl--x-k4.md",
        "# x-k4\n\n**Kind:** impl\n\n## Goal\nstuff\n",
    );
    let grandchild_before = body(&grandchild);
    let brief_before = body(&mid.join("BRIEF.md"));
    commit_all(&g);
    let inserted = leaf_insert(
        guard(&g),
        at(&g.join("02-mid-k3")),
        &a_slug("new"),
        &a_kind("impl"),
    )
    .unwrap();
    assert_eq!(name_of(&inserted.path), "02-impl--new-k5.md");
    let shifted = g.join("03-mid-k3");
    assert!(shifted.is_dir(), "node dir shifted to 03-mid-k3");
    assert!(!g.join("02-mid-k3").exists(), "old node dir name gone");
    assert_eq!(
        body(&shifted.join("BRIEF.md")),
        brief_before,
        "the moved node's BRIEF.md is byte-identical"
    );
    assert_eq!(
        body(&shifted.join("01-impl--x-k4.md")),
        grandchild_before,
        "the grandchild is byte-identical — only the ancestor dir name moved"
    );
    assert_eq!(
        inserted.renumbered.len(),
        1,
        "exactly one sibling (the node) shifted"
    );
    assert_eq!(inserted.renumbered[0].from_position, 2);
    assert_eq!(inserted.renumbered[0].to_position, 3);
}

#[test]
fn insert_writes_position_free_header_for_the_new_leaf() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    commit_all(&g);
    let inserted = leaf_insert(
        guard(&g),
        at(&g.join("01-impl--a-k1.md")),
        &a_slug("head"),
        &a_kind("impl"),
    )
    .unwrap();
    let text = body(&inserted.path);
    assert!(text.starts_with("# head-k2\n"), "got {text:?}");
    assert!(!text.contains("**Kind:**"), "got {text:?}");
}

#[test]
fn insert_does_not_rewrite_any_existing_file_contents() {
    // The property `leaf-insert`'s help text promises, and the one a reader will
    // check. It is structural rather than careful: a shift is
    // `compose(new_ordinal, key, parts)` and a rename, so there is no code path
    // through which a body could be touched. Pinned over every body in the tree
    // — the brief, a leaf, a node's charter and a grandchild — because the
    // claim is about all of them and not about the one that moved.
    let (_t, g) = jj_grove();
    let root_brief = touch_body(&g, "BRIEF.md", "# root — brief\n\nsee 02-impl--b-k2\n");
    let a = touch_body(&g, "01-impl--a-k1.md", "# a-k1\n\nfirst\n");
    let b = touch_body(&g, "02-impl--b-k2.md", "# b-k2\n\nbody text\n");
    let node = mknode(&g, "03-mid-k3", "mid-k3");
    let child = touch_body(&node, "01-impl--x-k4.md", "# x-k4\n\ndeep\n");
    let before: Vec<String> = [&root_brief, &a, &b, &node.join("BRIEF.md"), &child]
        .iter()
        .map(|path| body(path))
        .collect();
    commit_all(&g);

    leaf_insert(guard(&g), at(&b), &a_slug("new"), &a_kind("impl")).unwrap();

    let after: Vec<String> = [
        &g.join("BRIEF.md"),
        &g.join("01-impl--a-k1.md"),
        &g.join("03-impl--b-k2.md"),
        &g.join("04-mid-k3").join("BRIEF.md"),
        &g.join("04-mid-k3").join("01-impl--x-k4.md"),
    ]
    .iter()
    .map(|path| body(path))
    .collect();
    assert_eq!(
        before, after,
        "a renumber rewrites zero file contents; only the `NN` in a name moved"
    );
}

#[test]
fn insert_collision_free_for_a_dense_run_of_siblings() {
    // Stress the highest-first ordering: insert at the head of five siblings; a
    // wrong order would pass through a state carrying a duplicate ordinal.
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    for i in 1..=5 {
        touch(
            &g,
            &format!("{i:02}-impl--s{i}-k{i}.md"),
            &format!("s{i}-k{i}"),
        );
    }
    commit_all(&g);
    let inserted = leaf_insert(
        guard(&g),
        at(&g.join("01-impl--s1-k1.md")),
        &a_slug("head"),
        &a_kind("impl"),
    )
    .unwrap();
    assert_eq!(name_of(&inserted.path), "01-impl--head-k6.md");
    assert_eq!(inserted.renumbered.len(), 5);
    let leaves: Vec<String> = list(&g).into_iter().filter(|n| n != "BRIEF.md").collect();
    assert_eq!(
        leaves,
        vec![
            "01-impl--head-k6.md",
            "02-impl--s1-k1.md",
            "03-impl--s2-k2.md",
            "04-impl--s3-k3.md",
            "05-impl--s4-k4.md",
            "06-impl--s5-k5.md",
        ],
        "all six leaves present, gapless 01..06 — no file lost to a collision"
    );
}

#[test]
fn insert_shifts_highest_ordinal_first_and_reports_the_log_ascending() {
    // Two orders, and they are deliberately different. The library renames
    // highest-first — which is what keeps ordinals distinct at every
    // intermediate state, so a process killed mid-apply leaves a merely *gapped*
    // level — and `Report::renamed` is in that order. The log an operator reads
    // is the level's own, ascending, so the summary reads like the directory.
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    touch(&g, "02-impl--b-k2.md", "b-k2");
    touch(&g, "03-impl--c-k3.md", "c-k3");
    commit_all(&g);
    let inserted = leaf_insert(
        guard(&g),
        at(&g.join("01-impl--a-k1.md")),
        &a_slug("head"),
        &a_kind("impl"),
    )
    .unwrap();
    let positions: Vec<(u32, u32)> = inserted
        .renumbered
        .iter()
        .map(|renumber| (renumber.from_position, renumber.to_position))
        .collect();
    assert_eq!(positions, vec![(1, 2), (2, 3), (3, 4)]);
    assert_eq!(
        inserted.renumbered[0].from_name(),
        "01-impl--a-k1.md",
        "the log carries the name each entry left behind, which is what the lint scans for"
    );
    assert_eq!(inserted.renumbered[0].to_name(), "02-impl--a-k1.md");
}

#[test]
fn insert_inside_a_nested_node_shifts_only_that_levels_siblings() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    let design = mknode(&g, "01-design-k1", "design-k1");
    touch(&design, "01-impl--a-k2.md", "a-k2");
    touch(&design, "02-impl--b-k3.md", "b-k3");
    commit_all(&g);
    let inserted = leaf_insert(
        guard(&g),
        at(&design.join("01-impl--a-k2.md")),
        &a_slug("first"),
        &a_kind("impl"),
    )
    .unwrap();
    assert_eq!(name_of(&inserted.path), "01-impl--first-k4.md");
    assert_eq!(name_of(inserted.path.parent().unwrap()), "01-design-k1");
    let children = list(&design);
    assert!(children.contains(&"01-impl--first-k4.md".to_string()));
    assert!(children.contains(&"02-impl--a-k2.md".to_string()));
    assert!(children.contains(&"03-impl--b-k3.md".to_string()));
    assert_eq!(inserted.renumbered.len(), 2);
}

#[test]
fn insert_addresses_its_target_by_key_handle_and_slug_alike() {
    for target in ["2", "[2]", "[2]-b", "b-k2", "b"] {
        let (_t, g) = grove();
        touch(&g, "BRIEF.md", "root — brief");
        touch(&g, "01-impl--a-k1.md", "a-k1");
        touch(&g, "02-impl--b-k2.md", "b-k2");
        let inserted = leaf_insert(guard(&g), target, &a_slug("new"), &a_kind("impl")).unwrap();
        assert_eq!(
            name_of(&inserted.path),
            "02-impl--new-k3.md",
            "{target:?} must name the slot b holds"
        );
    }
}

#[test]
fn insert_refuses_a_target_whose_key_names_two_entries() {
    // `marking-k32`'s finding applied to an ordinal rather than to a key. A hand
    // edit or a failed rollback can put two entries under one key, and then *the
    // entry the operator named* is not a thing the tree can answer for — which
    // for this verb decides which slot the insert takes.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    touch(&g, "02-impl--b-k1.md", "b-k1");

    let err = leaf_insert(guard(&g), "1", &a_slug("new"), &a_kind("impl"))
        .unwrap_err()
        .to_string();

    assert!(err.contains("carry key 1"), "got {err}");
    assert_eq!(
        list(&g),
        vec!["01-impl--a-k1.md", "02-impl--b-k1.md", "BRIEF.md"],
        "nothing moved"
    );
}

/// An invalid slug never reaches the verb any more, and that is the point.
///
/// `leaf-insert` used to take `&str` and validate it first; since
/// `loop-crate-verbs-k21` it takes a [`Slug`], so the refusal happens where the
/// text is read — one type owns the name (principle 3) — and there is no way to
/// call the verb with a slug that is not one. What is left to check is that the
/// owner still refuses this token, which the tree reserves for its charter.
#[test]
fn insert_cannot_be_reached_with_an_invalid_slug() {
    assert!(Slug::new("BRIEF").is_err());
}

#[test]
fn insert_errors_when_target_is_a_brief() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    commit_all(&g);
    let err = leaf_insert(
        guard(&g),
        at(&g.join("BRIEF.md")),
        &a_slug("x"),
        &a_kind("impl"),
    )
    .unwrap_err();
    assert!(err.to_string().contains("brief"), "got {err}");
}

#[test]
fn insert_errors_when_the_target_is_the_grove_root() {
    // The root is a level and not an entry, so it holds no slot to take. Grove's
    // own refusal: the library's `insert` would have been handed an ordinal it
    // could not have derived.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    let err = leaf_insert(guard(&g), ".", &a_slug("x"), &a_kind("impl"))
        .unwrap_err()
        .to_string();
    assert!(err.contains("cannot insert at the grove root"), "got {err}");
}

#[test]
fn insert_errors_when_target_missing() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    commit_all(&g);
    assert!(leaf_insert(
        guard(&g),
        at(&g.join("09-impl--nope-k9.md")),
        &a_slug("x"),
        &a_kind("impl")
    )
    .is_err());
}

#[test]
fn insert_errors_when_grove_root_absent() {
    let (_t, g) = jj_grove();
    let missing = g.join("nope");
    let err = open(&missing)
        .err()
        .expect("an absent root has no tree to open");
    assert!(
        err.to_string().contains("grove root not found"),
        "got {err}"
    );
}

// ---- insert over untracked entries (issue #3) -------------------------------
//
// The grow verbs are working-tree-only by design — an added leaf is *uncommitted*
// and the enclosing task's commit folds it in. So the ordinary rhythm of a
// planning session (grow several leaves, then realise one must sequence earlier)
// hands `leaf_insert` siblings the committed revision has never seen. Since the
// flip the distinction has stopped mattering to the verb — a rename is a rename —
// and these tests are what says so.

#[test]
fn insert_ahead_of_an_untracked_sibling_added_this_session() {
    // Issue #3 verbatim: `leaf-add` then `leaf-insert` ahead of it, with no
    // commit in between.
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    let release = add_one(&g, ".", "release", "impl").unwrap();
    assert_eq!(name_of(&release), "01-impl--release-k1.md");

    // No commit: the leaf is uncommitted, exactly as the grow verb left it.
    let inserted =
        leaf_insert(guard(&g), at(&release), &a_slug("review"), &a_kind("impl")).unwrap();

    assert_eq!(name_of(&inserted.path), "01-impl--review-k2.md");
    let files = list(&g);
    assert!(
        files.contains(&"02-impl--release-k1.md".to_string()),
        "the uncommitted sibling shifted 01->02, key preserved (files: {files:?})"
    );
    assert!(
        !files.contains(&"01-impl--release-k1.md".to_string()),
        "old name gone (files: {files:?})"
    );
    assert_eq!(inserted.renumbered.len(), 1, "one sibling shifted");
}

#[test]
fn insert_renumbers_a_mix_of_committed_and_uncommitted_siblings() {
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    touch(&g, "02-impl--b-k2.md", "b-k2");
    commit_all(&g); // a and b are committed
    touch(&g, "03-impl--c-k3.md", "c-k3"); // c is not

    let inserted = leaf_insert(
        guard(&g),
        at(&g.join("01-impl--a-k1.md")),
        &a_slug("new"),
        &a_kind("impl"),
    )
    .unwrap();

    assert_eq!(name_of(&inserted.path), "01-impl--new-k4.md");
    let files = list(&g);
    for expected in ["02-impl--a-k1.md", "03-impl--b-k2.md", "04-impl--c-k3.md"] {
        assert!(
            files.contains(&expected.to_string()),
            "every sibling shifted up one, committed or not: missing {expected} (files: {files:?})"
        );
    }
    assert_eq!(inserted.renumbered.len(), 3, "all three siblings shifted");
}

#[test]
fn insert_records_nothing_in_the_committed_revision() {
    // **Inverted, and it is question 1 arriving at the last verbs that answered
    // it the other way.** A shift used to be a version-control-aware move, so it
    // reached the repository; it is `rename(2)` now, because that is what
    // `ordinal-fs-tree` does and grove does not reassemble the deleted primitive
    // one layer up (`docs/adr/grove-does-not-stage-its-own-renames.md`). The
    // verb moves the working copy and records nothing: the revision it sits on
    // still holds the old name, and the enclosing task's own commit is what
    // folds the shift in.
    let (_t, g) = jj_grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    commit_all(&g);

    leaf_insert(
        guard(&g),
        at(&g.join("01-impl--a-k1.md")),
        &a_slug("new"),
        &a_kind("impl"),
    )
    .unwrap();

    let recorded = committed(&g);
    assert!(
        recorded.contains(&"01-impl--a-k1.md".to_string()),
        "the committed revision still carries the old name — the verb recorded \
         nothing (revision: {recorded:?})"
    );
    assert!(
        !recorded.contains(&"02-impl--a-k1.md".to_string()),
        "and it recorded no new one either (revision: {recorded:?})"
    );
    assert!(
        g.join("02-impl--a-k1.md").is_file(),
        "while on disk the sibling did shift"
    );
}

// ---- surface_cross_refs (position-prefixed lint, not auto-rewrite) ----------

/// A renumber as the verb reports one: two paths, and the positions a caller
/// prints. The fixtures below name the two filenames, so the root they sit under
/// is joined here.
fn renum(root: &Path, old: u32, new: u32, old_name: &str, new_name: &str) -> Renumber {
    Renumber {
        from: root.join(old_name),
        to: root.join(new_name),
        from_position: old,
        to_position: new,
    }
}

fn surfaced(root: &Path, renumbered: &[Renumber]) -> String {
    let mut buffer = Vec::new();
    surface_cross_refs(guard(root), renumbered, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

#[test]
fn surface_empty_renumbers_emits_nothing() {
    let (_t, g) = grove();
    touch_body(
        &g,
        "01-impl--a-k1.md",
        "# a-k1\n\nrefers to 02-mid-k3 somewhere\n",
    );
    assert_eq!(surfaced(&g, &[]), "");
}

#[test]
fn surface_reports_a_stale_position_prefixed_reference_in_a_body() {
    let (_t, g) = grove();
    touch_body(
        &g,
        "01-impl--a-k1.md",
        "# a-k1\n\nthe design lives at 02-mid-k3/01-impl--x-k4.md\n",
    );
    let out = surfaced(&g, &[renum(&g, 2, 3, "02-mid-k3", "03-mid-k3")]);
    assert!(out.contains("01-impl--a-k1.md"), "names the file: {out:?}");
    assert!(out.contains("02-mid-k3"), "shows the stale name: {out:?}");
    assert!(
        out.contains(":3:"),
        "1-based line number of the body ref: {out:?}"
    );
}

#[test]
fn surface_does_not_flag_the_stable_slug_key_handle() {
    // A `<slug>-k<key>` reference is stable across a renumber — never surfaced.
    let (_t, g) = grove();
    touch_body(
        &g,
        "01-impl--a-k1.md",
        "# a-k1\n\nsee mid-k3 for the design\n",
    );
    assert_eq!(
        surfaced(&g, &[renum(&g, 2, 3, "02-mid-k3", "03-mid-k3")]),
        ""
    );
}

#[test]
fn surface_reports_hits_recursively_across_nested_files() {
    let (_t, g) = grove();
    touch_body(
        &g,
        "BRIEF.md",
        "# root — brief\n\nthe plan is at 02-mid-k3\n",
    );
    let design = mknode(&g, "01-design-k1", "design-k1");
    touch_body(
        &design,
        "01-impl--a-k2.md",
        "# a-k2\n\nalso 02-mid-k3 here\n",
    );
    let out = surfaced(&g, &[renum(&g, 2, 3, "02-mid-k3", "03-mid-k3")]);
    assert!(
        out.contains("BRIEF.md") && out.contains("02-mid-k3"),
        "{out:?}"
    );
    assert!(
        out.contains("01-impl--a-k2.md"),
        "nested file surfaced: {out:?}"
    );
}

#[test]
fn surface_scans_the_tree_and_not_the_directory() {
    // **A narrowing, recorded because it is one.** The lint used to walk
    // `.grove/` for every `.md` file; it walks the snapshot now, so what it
    // scans is every leaf and every charter — the same set every other verb
    // calls the tree — and a foreign file dropped in by hand is no longer
    // scanned. Grove writes no such file, and the alternative is a second,
    // wider notion of *what is in the tree* than the reader has.
    let (_t, g) = grove();
    touch_body(
        &g,
        "BRIEF.md",
        "# root — brief\n\nthe plan is at 02-mid-k3\n",
    );
    touch_body(&g, "NOTES.md", "# notes\n\nalso 02-mid-k3 here\n");
    let out = surfaced(&g, &[renum(&g, 2, 3, "02-mid-k3", "03-mid-k3")]);
    assert!(
        out.contains("BRIEF.md"),
        "the charter is in the tree: {out:?}"
    );
    assert!(!out.contains("NOTES.md"), "a foreign file is not: {out:?}");
}

#[test]
fn surface_holds_the_tree_while_it_reports() {
    // The lint's own guard: a hit names a path, and a path anything else could
    // rename between the read and the write is a path the operator cannot open.
    let (_t, g) = grove();
    touch_body(
        &g,
        "BRIEF.md",
        "# root — brief\n\nthe plan is at 02-mid-k3\n",
    );
    let mut probe = ProbingWriter {
        directory: g.parent().unwrap().to_path_buf(),
        bytes: Vec::new(),
    };
    surface_cross_refs(
        guard(&g),
        &[renum(&g, 2, 3, "02-mid-k3", "03-mid-k3")],
        &mut probe,
    )
    .unwrap();
    assert!(
        !probe.bytes.is_empty(),
        "the fixture must produce a hit for the probe to fire on"
    );
}

/// A writer that fails the test if the tree is not exclusively locked at the
/// moment a byte of lint output is written.
struct ProbingWriter {
    directory: PathBuf,
    bytes: Vec<u8>,
}

impl Write for ProbingWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        use std::os::fd::AsRawFd;
        let directory = std::fs::File::open(&self.directory)?;
        let taken =
            unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) } == 0;
        assert!(
            !taken,
            "cross-reference output was written without the tree held"
        );
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

// ---- the reachability table, transcribed ------------------------------------
//
// `docs/ARCHITECTURE.md#library-refusals` predicts which `Refusal` variants
// these three verbs can put in front of an operator, and the table's own
// guarantee is that each migrate leaf writes its rows into a suite and finds
// them wrong if they are. `marking-k32` found one row wrong that way. These are
// the grow verbs' rows, and two of them are wrong.
//
// | verb | predicted |
// |---|---|
// | `leaf-add` | `TargetNotNode`; `DestinationOccupied`, `KeysExhausted`, `OrdinalsExhausted` |
// | `leaf-add-pair` | the same four |
// | `leaf-insert` | `DestinationOccupied`, `KeysExhausted`, `OrdinalsExhausted` |
//
// `KeysExhausted` and `OrdinalsExhausted` are reached, and their tests are above
// (`a_run_that_cannot_get_three_fresh_keys_creates_nothing_at_all` and
// `a_level_at_the_last_ordinal_refuses_rather_than_wrapping`). The two below are
// not, and each is a correction rather than a gap.

#[test]
fn target_not_node_is_unreachable_because_clause_two_makes_it_so() {
    // **The row said *yes* and named its own contradiction in the next clause:
    // *Grove keeps its own check in front of it*.** Both are true of the design
    // and only one can be true of an operator, and it is grove's message they
    // see. That is not an accident to be tidied away either: a `BRIEF.md` is an
    // entry carrying no key at all, so it cannot be handed to the library as a
    // target however the refusal is worded — grove *must* classify, and once it
    // has, the library's species refusal sits behind a check grove needed
    // anyway. Every parent argument that is an entry and not a node, then.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--a-k1.md", "a-k1");
    let node = mknode(&g, "02-build-k2", "build-k2");
    touch(&node, "01-DONE-impl--x-k3.md", "x-k3");

    for parent in [
        at(&g.join("BRIEF.md")),
        at(&g.join("01-impl--a-k1.md")),
        "1",
        at(&node.join("01-DONE-impl--x-k3.md")),
        "3",
    ] {
        for err in [
            add_one(&g, parent, "y", "impl").unwrap_err().to_string(),
            add_pair(&g, parent, "y").unwrap_err().to_string(),
        ] {
            assert!(
                err.contains("parent is not a node directory"),
                "{parent:?} must reach grove's own refusal, not the library's: {err}"
            );
            assert!(
                !err.contains("promote"),
                "the library's `TargetNotNode` says *promote it first*, which grove \
                 has no verb for — that is the collision, and it must not reach an \
                 operator: {err}"
            );
        }
    }
}

#[test]
fn destination_occupied_is_unreachable_from_a_shift_however_the_tree_was_edited() {
    // **The row said *yes from the grow verbs, on a hand-edited tree: a copied
    // leaf duplicating a key*, and composing that tree is what shows it is
    // not.** The reasoning is the same shape as `marking-k32`'s: a destination
    // is occupied only if some *other* entry already carries exactly the name an
    // effect composes, and neither verb can produce one.
    //
    //   * an **append** composes its name with `max + 1` over the whole tree, so
    //     no entry in the snapshot can carry it, whatever a hand edit did;
    //   * a **shift** composes `(ordinal + 1, key, parts)`, and the only entry
    //     that could already carry that name is the sibling one ordinal higher —
    //     which is itself a mover, and has already vacated, because the renames
    //     run highest-first and the plan is folded through the snapshot in that
    //     order. This is the second thing highest-first buys, after the
    //     intermediate state, and the library's own note says as much in
    //     passing: *lowest-first is refused only where a hand edit already
    //     duplicated a key and its parts at adjacent ordinals*. The design
    //     document's table did not carry the consequence.
    //
    // The fixture is `operations.qnt`'s `corrupted` instance in grove's own
    // grammar — two entries sharing a key *and* their parts at adjacent
    // ordinals — with the insert aimed at an unrelated third entry, so grove's
    // duplicate-key guard passes and the shift is what has to survive.
    let (_t, g) = grove();
    touch(&g, "BRIEF.md", "root — brief");
    touch(&g, "01-impl--target-k9.md", "target-k9");
    touch(&g, "02-impl--twin-k5.md", "twin-k5");
    touch(&g, "03-impl--twin-k5.md", "twin-k5");

    let inserted = leaf_insert(guard(&g), "9", &a_slug("new"), &a_kind("impl")).unwrap();

    assert_eq!(name_of(&inserted.path), "01-impl--new-k10.md");
    assert_eq!(
        list(&g),
        vec![
            "01-impl--new-k10.md",
            "02-impl--target-k9.md",
            "03-impl--twin-k5.md",
            "04-impl--twin-k5.md",
            "BRIEF.md",
        ],
        "the duplicated pair shifted past each other without a collision"
    );
}

// ---- one command, one observation of the tree ------------------------------
//
// Carried across from `llm_cli`'s own test module at `loop-crate-verbs-k21`.
// They belong here rather than with the CLI for the reason they always did:
// their subject is `task_tree`'s read counter, which is crate-private, and what
// they assert is a property of the **verbs** — one command reads the tree once —
// not of the argument parsing in front of them.

fn grove_with_node() -> (TempDir, PathBuf) {
    let worktree = TempDir::new().unwrap();
    let grove_root = worktree.path().join(".grove");
    let paths = {
        let task_tree::Opening::Vacancy(vacancy) =
            task_tree::write_or_vacancy(&grove_root).unwrap()
        else {
            panic!("a fresh worktree holds no grove");
        };
        crate::tree_lifecycle::root_init(vacancy, &a_slug("plan"), &Kind::requirements()).unwrap()
    };
    crate::tree_lifecycle::leaf_decompose(
        guard(&grove_root),
        &paths[1],
        &a_slug("first"),
        Some(a_kind("impl")),
    )
    .unwrap();
    (worktree, grove_root)
}

fn assert_one_acquisition(operation: impl FnOnce(&Path)) {
    let (_worktree, grove_root) = grove_with_node();
    task_tree::reset_read_count();

    operation(&grove_root);

    // **One counter, where there used to be two summed.** Grove holds no
    // guard of its own since `collapse-tree-access-k13`, so every
    // observation of the tree is the store's and `task_tree` counts all of
    // them. The property is unchanged: one command reads the tree once, so a
    // verb that selected a leaf and then re-read the tree to act on it could
    // not slip through.
    assert_eq!(
        task_tree::read_count(),
        1,
        "one command must observe the tree exactly once"
    );
}

#[test]
fn reference_taking_commands_acquire_the_tree_lock_once() {
    assert_one_acquisition(|grove_root| {
        add_one(grove_root, "1", "later", "impl").unwrap();
    });
    assert_one_acquisition(|grove_root| {
        add_pair(grove_root, "1", "survey").unwrap();
    });
    assert_one_acquisition(|grove_root| {
        let tree = task_tree::tests::read(grove_root).unwrap();
        let leaf = task_tree::pick_in(&tree).unwrap().unwrap();
        task_tree::brief_chain(&tree, &leaf).unwrap();
    });
}

struct ExclusiveLockAssertingWriter {
    worktree: PathBuf,
    bytes: Vec<u8>,
}

impl std::io::Write for ExclusiveLockAssertingWriter {
    fn write(&mut self, buffer: &[u8]) -> std::io::Result<usize> {
        use std::os::fd::AsRawFd;
        let directory = std::fs::File::open(&self.worktree)?;
        let result = unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_SH | libc::LOCK_NB) };
        assert_ne!(
            result, 0,
            "cross-reference output was written after leaf-insert released its exclusive lock"
        );
        self.bytes.extend_from_slice(buffer);
        Ok(buffer.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn leaf_insert_lints_cross_references_under_an_exclusive_lock_of_its_own() {
    // **Two observations, and the second is the lint's.** A mutation consumes
    // its guard, so the tree the shift *left* — which is the tree a stale
    // reference has to be read out of, since a shifted node took its whole
    // subtree's paths with it — can only be seen through a second guard. What
    // the property was ever about is that the output is written while the tree
    // is **held**: a hit printed after the lock went would name a path anything
    // else could already have renamed.
    let (worktree, grove_root) = grove_with_node();
    let brief = grove_root.join("01-plan-k1").join("BRIEF.md");
    let body = fs::read_to_string(&brief).unwrap() + "stale path: 01-impl--first-k2\n";
    fs::write(&brief, body).unwrap();
    let mut output = ExclusiveLockAssertingWriter {
        worktree: worktree.path().to_path_buf(),
        bytes: Vec::new(),
    };
    task_tree::reset_read_count();

    let inserted =
        leaf_insert(guard(&grove_root), "2", &a_slug("earlier"), &a_kind("impl")).unwrap();
    surface_cross_refs(guard(&grove_root), &inserted.renumbered, &mut output).unwrap();

    assert!(
        String::from_utf8_lossy(&output.bytes).contains("BRIEF.md:"),
        "fixture must exercise a cross-reference hit: {}",
        String::from_utf8_lossy(&output.bytes)
    );
    assert_eq!(
        task_tree::read_count(),
        2,
        "the insert, then the lint's own guard over the tree it left"
    );
}

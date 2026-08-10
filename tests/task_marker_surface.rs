//! The task-body **marker** surface, asserted by **enumeration over what Grove
//! writes** — not by a list of retired markers to look for.
//!
//! The design left task bodies with exactly two pieces of metadata,
//! `**Reviews:**` and `**Integrates:**`, because those describe artifact
//! composition rather than launch policy. `**Kind:**`, `**Harness:**` and
//! `**Producer launch:**` went the other way, and after
//! `review-receipt-removal-k84` the *format* went with the writer: nothing in
//! Grove parses a task body except the composition lookup in
//! `src/task_relationship.rs` and migration's stripper.
//!
//! The obvious way to hold that line is `!body.contains("**Producer launch:**")`
//! at each generation site — and the suite had several. That assertion answers
//! only the question its author thought to ask: it passes forever once the
//! marker is gone, it passes unchanged the day a *different* marker is
//! reintroduced, and it names a string that now exists only inside the code that
//! deletes it. So the claim is stated the other way round. Every `**…:**`
//! occurrence in every body Grove **generates** is enumerated, and each must be a
//! declared composition relationship *on the step that should carry it*. A marker
//! nobody has invented yet fails here, with its file and the line it appeared on.
//!
//! Two things this file deliberately does **not** do:
//!
//! * **It does not sweep `src/` lexically.** Marker text appears throughout
//!   production *prose* — module headers explaining what was removed and why —
//!   and a lexical scan cannot tell a doc comment from a live string literal.
//!   Classifying prose would either fail on honest documentation or need an
//!   exception per comment, and neither is evidence. What Grove *writes* is the
//!   observable fact, and it is what a reintroduced writer would change.
//! * **It does not restate the reader half.** That a body marker is not metadata
//!   is already stated where it can fail: `filename_kind_is_unambiguous_and_body_routing_metadata_is_ignored`
//!   (`tests/session_kind_tree.rs`) drives a leaf whose body says `**Kind:** impl`
//!   and gets the *filename's* kind back, and the `task_relationship` unit tests
//!   show an undeclared marker reading as prose.
//!
//! **Cross-tree, because generation is not the only thing between the verb and
//! the bytes.** `leaf-promote-chain` moves the producer through the VCS, and a
//! Git worktree holding a *tracked* producer takes a different path to do it
//! than a jj-native workspace — the trackedness is load-bearing, and `fixture`
//! says why. A claim about the bodies a promotion leaves behind is a claim about
//! both trees or about neither.
//!
//! **Both controls, because a sweep that cannot fail is worth nothing.** The
//! enumeration is shown finding the markers that *are* there, at the exact steps
//! that carry them; and the classifier is shown rejecting a body with one extra
//! marker injected into it.

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Process;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Roles

/// What a `**…:**` body marker *is*. The composition relationships are the whole
/// declared set; anything else in a generated body is a finding, whether it is a
/// marker the design removed or one nobody has thought of.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Role {
    /// A review names the producer it reviews.
    Reviews,
    /// An integration names the review whose findings it applies.
    Integrates,
}

fn role_of(marker: &str) -> Option<Role> {
    match marker {
        "**Reviews:**" => Some(Role::Reviews),
        "**Integrates:**" => Some(Role::Integrates),
        _ => None,
    }
}

// ---------------------------------------------------------------------------
// Enumeration

#[derive(Clone, Debug, PartialEq, Eq)]
struct Occurrence {
    /// The generated file, relative to the working tree.
    file: String,
    line: usize,
    marker: String,
    /// The rest of the line: the handle a relationship declares.
    value: String,
}

/// Every `**…:**` marker on one line, with what follows it.
///
/// The marker must start at column zero after optional Markdown-legal
/// indentation, because that is the only position Grove writes one and the only
/// position anything reads one — bold text mid-sentence is prose, and a body
/// that discusses a relationship is not declaring one.
fn markers_in(line: &str) -> Option<(String, String)> {
    let line = line.trim_start();
    let rest = line.strip_prefix("**")?;
    let (name, tail) = rest.split_once(":**")?;
    if name.is_empty() || name.contains('*') {
        return None;
    }
    Some((format!("**{name}:**"), tail.trim().to_string()))
}

/// Every marker in every `.md` file under `.grove/`, in walk order.
fn occurrences(worktree: &Path) -> Vec<Occurrence> {
    fn walk(dir: &Path, grove: &Path, out: &mut Vec<Occurrence>) {
        let mut entries: Vec<PathBuf> = fs::read_dir(dir)
            .unwrap_or_else(|error| panic!("reading {}: {error}", dir.display()))
            .map(|entry| entry.unwrap().path())
            .collect();
        entries.sort();
        for entry in entries {
            if entry.is_dir() {
                walk(&entry, grove, out);
                continue;
            }
            if !entry.extension().is_some_and(|extension| extension == "md") {
                continue;
            }
            let file = entry
                .strip_prefix(grove)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let body = fs::read_to_string(&entry).unwrap();
            for (index, line) in body.lines().enumerate() {
                if let Some((marker, value)) = markers_in(line) {
                    out.push(Occurrence {
                        file: file.clone(),
                        line: index + 1,
                        marker,
                        value,
                    });
                }
            }
        }
    }
    let mut out = Vec::new();
    walk(&worktree.join(".grove"), &worktree.join(".grove"), &mut out);
    out
}

fn render(occurrences: &[&Occurrence]) -> String {
    occurrences
        .iter()
        .map(|occurrence| {
            format!(
                "  {} at {}:{}",
                occurrence.marker, occurrence.file, occurrence.line
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

// ---------------------------------------------------------------------------
// Fixtures

#[derive(Clone, Copy, Debug)]
enum Tree {
    Git,
    Jj,
}

fn vcs(binary: &str, dir: &Path, arguments: &[&str]) -> String {
    let output = Process::new(binary)
        .current_dir(dir)
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("running {binary} {arguments:?}: {error}"));
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).into_owned()
}

/// A working tree of the given VCS holding a current-format `.grove/` with one
/// live plain producer, ready to promote.
///
/// **The Git producer is tracked, and that is the whole difference between the
/// two fixtures.** `leaf-promote-chain` picks its move by trackedness, not by
/// VCS: an *untracked* entry moves with `fs::rename` (`src/tree_rename.rs`) —
/// exactly the operation a jj tree performs — and its index preparation is
/// skipped for want of an index entry. A Git fixture that only writes the files
/// therefore runs the jj half's path a second time under another name, and the
/// cross-tree claim above rests on nothing. Staging the producer is what puts
/// `git mv` plus the staged-path rewrite under the sweep; the assertion below
/// keeps it that way.
fn fixture(tree: Tree) -> TempDir {
    let tmp = TempDir::new().unwrap();
    match tree {
        Tree::Git => {
            vcs("git", tmp.path(), &["init", "-q", "."]);
        }
        Tree::Jj => {
            vcs(
                "jj",
                tmp.path(),
                &[
                    "--config",
                    "user.name=Test",
                    "--config",
                    "user.email=t@example.com",
                    "--config",
                    "git.colocate=false",
                    "git",
                    "init",
                    "--quiet",
                    ".",
                ],
            );
            assert!(
                !tmp.path().join(".git").exists(),
                "the jj fixture must have no .git/, or a git fallback would pass unnoticed"
            );
        }
    }
    let root = tmp.path().join(".grove");
    fs::create_dir(&root).unwrap();
    fs::write(root.join("BRIEF.md"), "# root — brief\n").unwrap();
    fs::write(root.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(root.join("01-design-sync-k1.md"), "# sync-k1\n").unwrap();
    if let Tree::Git = tree {
        vcs("git", tmp.path(), &["add", "--", ".grove"]);
        assert_eq!(
            vcs(
                "git",
                tmp.path(),
                &["ls-files", "--", ".grove/01-design-sync-k1.md"],
            )
            .trim(),
            ".grove/01-design-sync-k1.md",
            "the Git fixture's producer must be tracked, or promotion takes the \
             untracked plain-rename path and this half stops differing from the \
             jj half"
        );
    }
    tmp
}

fn llm(worktree: &Path, args: &[&str]) -> (String, String, bool) {
    let output = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(worktree)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

/// Every verb that writes a task body, run once, so the sweep sees the whole
/// generated surface rather than one shape's worth of it.
fn generate_every_shape(worktree: &Path) {
    for args in [
        &["leaf-add", ".", "plain"][..],
        &["leaf-add-chain", ".", "built", "--kind", "impl"][..],
        &["leaf-add-pair", ".", "survey"][..],
        // By its stable handle, as a session mandated to it would.
        &["leaf-promote-chain", "sync-k1"][..],
    ] {
        let (_, stderr, ok) = llm(worktree, args);
        assert!(ok, "{args:?} failed: {stderr}");
    }
}

// ---------------------------------------------------------------------------
// The sweep

/// The whole point: a marker nobody classified fails, whether or not anyone
/// thought to list it — and it fails *at the step that carries it*, so a review's
/// relationship appearing on a producer is a finding too.
fn assert_only_declared_markers_are_written(tree: Tree) {
    let tmp = fixture(tree);
    generate_every_shape(tmp.path());

    let found = occurrences(tmp.path());
    let undeclared: Vec<&Occurrence> = found
        .iter()
        .filter(|occurrence| role_of(&occurrence.marker).is_none())
        .collect();
    assert!(
        undeclared.is_empty(),
        "Grove wrote body markers it does not declare in a {tree:?} tree — each is \
         either a reintroduced metadata surface or a new one that must earn a \
         role:\n{}",
        render(&undeclared)
    );

    // The relationship a step declares has to match the step it *is*. Without
    // this the sweep would accept a producer that declares it reviews itself,
    // which is a well-formed marker in the wrong place.
    for occurrence in &found {
        let expected =
            if occurrence.file.contains("-review-") && !occurrence.file.contains("-integrate-") {
                Role::Reviews
            } else if occurrence.file.contains("-integrate-") {
                Role::Integrates
            } else {
                panic!(
                    "a marker on a step that carries no relationship: {} at {}:{}",
                    occurrence.marker, occurrence.file, occurrence.line
                )
            };
        assert_eq!(
            role_of(&occurrence.marker),
            Some(expected),
            "{} declares the wrong relationship for its step ({}:{})",
            occurrence.marker,
            occurrence.file,
            occurrence.line
        );
    }
}

#[test]
fn a_git_tree_receives_only_declared_body_markers() {
    assert_only_declared_markers_are_written(Tree::Git);
}

/// The cross-tree half. `leaf-promote-chain` writes the chain's other two steps
/// into its transaction, *then* moves the producer through the VCS — and a
/// jj-native workspace takes a different path to do that than the tracked Git
/// worktree above: a plain rename against a tree with no index, rather than
/// `git mv` followed by rewriting the staged path to its final spelling.
#[test]
fn a_jj_tree_receives_only_declared_body_markers() {
    assert_only_declared_markers_are_written(Tree::Jj);
}

/// The enumeration's own control. A sweep over an empty result set reports a
/// clean tree for the wrong reason, so this pins what the walk must find: both
/// relationships, on the exact steps of both shapes, naming the exact handles.
#[test]
fn the_walk_finds_every_relationship_the_two_chain_shapes_declare() {
    let tmp = fixture(Tree::Git);
    generate_every_shape(tmp.path());

    let found: Vec<(String, String, String)> = occurrences(tmp.path())
        .into_iter()
        .map(|occurrence| (occurrence.file, occurrence.marker, occurrence.value))
        .collect();

    assert_eq!(
        found,
        vec![
            // The promoted chain: the producer kept its position, so its node
            // heads the tree even though its keys were allocated last.
            (
                "01-sync-chain-k11/02-review-design-sync-review-k12.md".to_string(),
                "**Reviews:**".to_string(),
                "sync-k1".to_string(),
            ),
            (
                "01-sync-chain-k11/03-integrate-review-design-sync-integrate-k13.md".to_string(),
                "**Integrates:**".to_string(),
                "sync-review-k12".to_string(),
            ),
            // The constructed chain.
            (
                "03-built-chain-k3/02-review-impl-built-review-k5.md".to_string(),
                "**Reviews:**".to_string(),
                "built-k4".to_string(),
            ),
            (
                "03-built-chain-k3/03-integrate-review-impl-built-integrate-k6.md".to_string(),
                "**Integrates:**".to_string(),
                "built-review-k5".to_string(),
            ),
        ],
        "the research pair and the plain leaf declare nothing, and both chain \
         shapes declare exactly one relationship per step"
    );
}

/// The two verbs that build a review chain must write the *same* declaration for
/// the same relationship. They are separate code paths — `leaf-add-chain`
/// constructs a shape, `leaf-promote-chain` reconstructs one around an existing
/// producer — and they spelled the format out independently until it was given
/// one home. Two literals that agree today are not the same thing as one
/// definition, and only this notices when they stop agreeing.
#[test]
fn constructed_and_promoted_chains_declare_their_relationships_identically() {
    let tmp = fixture(Tree::Git);
    generate_every_shape(tmp.path());

    let by_role = |role: Role| -> Vec<String> {
        occurrences(tmp.path())
            .into_iter()
            .filter(|occurrence| role_of(&occurrence.marker) == Some(role))
            // The handle differs per shape, so compare the rendering with the
            // handle removed: what is under test is the marker and its spacing.
            .map(|occurrence| occurrence.marker)
            .collect()
    };

    for role in [Role::Reviews, Role::Integrates] {
        let rendered = by_role(role);
        assert_eq!(
            rendered.len(),
            2,
            "both shapes must declare {role:?} exactly once: {rendered:?}"
        );
        assert_eq!(
            rendered[0], rendered[1],
            "the constructed and promoted chains disagree on how {role:?} is written"
        );
    }

    // The goal sentence travels with the declaration, and it was the other half
    // of the duplication. Compared across shapes with the handles substituted
    // out, since only the handles legitimately differ.
    let goal = |path: &str, handle: &str| -> String {
        fs::read_to_string(tmp.path().join(".grove").join(path))
            .unwrap()
            .replace(handle, "<handle>")
    };
    let promoted_review = goal(
        "01-sync-chain-k11/02-review-design-sync-review-k12.md",
        "sync-k1",
    );
    let constructed_review = goal(
        "03-built-chain-k3/02-review-impl-built-review-k5.md",
        "built-k4",
    );
    assert!(
        promoted_review.contains("Adversarially review `<handle>`"),
        "the review's goal names its producer: {promoted_review}"
    );
    assert_eq!(
        promoted_review.replace("sync-review-k12", "<own-handle>"),
        constructed_review.replace("built-review-k5", "<own-handle>"),
        "a promoted review step and a constructed one are the same document"
    );
}

/// The classifier's positive control. It is only evidence if it can say no, so
/// here it is shown rejecting exactly what it exists to catch: one extra marker
/// in an otherwise clean generated body.
#[test]
fn the_sweep_rejects_a_reintroduced_marker_in_a_generated_body() {
    let tmp = fixture(Tree::Git);
    generate_every_shape(tmp.path());
    assert!(
        occurrences(tmp.path())
            .iter()
            .all(|occurrence| role_of(&occurrence.marker).is_some()),
        "the fixture must start clean, or the rejection below proves nothing"
    );

    let review = tmp
        .path()
        .join(".grove/01-sync-chain-k11/02-review-design-sync-review-k12.md");
    let body = fs::read_to_string(&review).unwrap();
    fs::write(
        &review,
        format!("{body}**Producer launch:** {{\"producer\":\"sync-k1\"}}\n"),
    )
    .unwrap();

    let undeclared: Vec<Occurrence> = occurrences(tmp.path())
        .into_iter()
        .filter(|occurrence| role_of(&occurrence.marker).is_none())
        .collect();
    assert_eq!(
        undeclared.len(),
        1,
        "the sweep must report the reintroduced marker and nothing else: {undeclared:?}"
    );
    assert_eq!(undeclared[0].marker, "**Producer launch:**");

    // ...and the same classifier over the shapes production really writes, so
    // the rejection is discrimination rather than suspicion of everything.
    assert_eq!(role_of("**Reviews:**"), Some(Role::Reviews));
    assert_eq!(role_of("**Integrates:**"), Some(Role::Integrates));
    assert_eq!(role_of("**Kind:**"), None);
    assert_eq!(role_of("**Harness:**"), None);
    assert_eq!(
        role_of("**Invented:**"),
        None,
        "a marker of no known name must be reported rather than waved through"
    );
}

/// The lexer's own control. `markers_in` decides what counts as a declaration
/// site, so its two rules are stated where they can fail: a marker is read at
/// the start of a line, and bold text elsewhere in a sentence is prose.
#[test]
fn only_a_line_initial_bold_field_reads_as_a_marker() {
    assert_eq!(
        markers_in("**Reviews:** build-k1"),
        Some(("**Reviews:**".to_string(), "build-k1".to_string()))
    );
    assert_eq!(
        markers_in("  **Reviews:** build-k1"),
        Some(("**Reviews:**".to_string(), "build-k1".to_string())),
        "Markdown-legal indentation still declares"
    );
    assert_eq!(
        markers_in("**Reviews:**"),
        Some(("**Reviews:**".to_string(), String::new())),
        "an empty declaration is still a marker occurrence — it is the sweep's \
         business to see it, not to judge it"
    );

    for prose in [
        "The review carries **Reviews:** and the integration carries the other.",
        "## Goal",
        "**bold** text with a colon: not a marker",
        "",
    ] {
        assert_eq!(markers_in(prose), None, "read {prose:?} as a marker");
    }
}

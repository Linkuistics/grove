// How a grove composes several leaves into one artifact, exercised through the
// real binaries. Two shapes, and after flat-lazy-review they are built by two
// different mechanisms:
//
//   * a **review chain** is `leaf-add`, three times, spread across three
//     sessions — the producer cuts its review only if review is required, and
//     the review cuts its integration only if it found something. There is no
//     chain verb and no chain node; the steps are flat siblings and the only
//     thing that groups them is a shared stem — which every step carries whole,
//     because the kind field already states its role and the slug does not
//     restate it. So what the binary can be held to is exactly what `leaf-add`
//     does: append at the parent's next free position, with a body the
//     *creating session* is free to write.
//   * a **research pair** is still one call, because its two producers must not
//     see each other's framing. The library owns the shape and its
//     all-or-nothing contract (`append_many`'s own interpreter); what only the
//     binary can show is the *command* contract — stdout is the shape or it is
//     nothing, and the verb sits beside `leaf-add` in `--help`, the surface a
//     session re-orients on when it drops context.

use assert_cmd::Command;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// A real git worktree holding a `.grove/` with only its root brief. The verbs
/// write untracked files and never shell out to a VCS, but every `grove-llm`
/// verb resolves its grove root from a repo, so the fixture is a repo.
fn grove() -> TempDir {
    let tmp = TempDir::new().unwrap();
    std::process::Command::new("git")
        .arg("init")
        .arg("-q")
        .arg(tmp.path())
        .status()
        .unwrap();
    let root = tmp.path().join(".grove");
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("BRIEF.md"), "# root — brief\n").unwrap();
    fs::write(root.join("FORMAT"), "session-kinds-v1\n").unwrap();
    tmp
}

fn run(worktree: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(worktree)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

/// The grove's entry names, lexically sorted.
fn tree(worktree: &Path) -> Vec<String> {
    let mut v: Vec<String> = fs::read_dir(worktree.join(".grove"))
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    v.sort();
    v
}

fn body(worktree: &Path, name: &str) -> String {
    fs::read_to_string(worktree.join(".grove").join(name)).unwrap()
}

// ---------------------------------------------------------------------------
// The review chain: three ordinary `leaf-add`s, cut lazily

#[test]
fn a_review_chain_is_three_leaf_adds_landing_as_flat_siblings() {
    // The command-level shape of the whole workstream. Nothing here names a
    // chain: each call is the ordinary append a session makes as its last act,
    // and what makes the three a chain is the shared stem plus the convention
    // the sessions write into the bodies themselves.
    let t = grove();
    for (slug, kind, expected) in [
        ("sync", "design", "01-design-sync-k1.md"),
        ("sync", "review-design", "02-review-design-sync-k2.md"),
        (
            "sync",
            "integrate-review-design",
            "03-integrate-review-design-sync-k3.md",
        ),
    ] {
        let (stdout, stderr, ok) = run(t.path(), &["leaf-add", ".", slug, "--kind", kind]);
        assert!(ok, "leaf-add {slug} failed: {stderr}");
        assert!(
            stdout.trim().ends_with(expected) && stdout.starts_with('/'),
            "expected an absolute path ending {expected}, got {stdout:?}"
        );
    }
    assert_eq!(
        tree(t.path()),
        vec![
            "01-design-sync-k1.md",
            "02-review-design-sync-k2.md",
            "03-integrate-review-design-sync-k3.md",
            "BRIEF.md",
            "FORMAT",
        ],
        "flat siblings — no chain node was created for them"
    );
}

#[test]
fn a_freshly_added_leaf_carries_an_empty_body_for_its_creator_to_write() {
    // Why laziness pays: the creating session writes the new leaf's body, so
    // the template must not pre-empt it with a rendered goal or a relationship
    // line it would have to edit around. It gets the stable handle and empty
    // sections, and nothing else.
    let t = grove();
    let (_, stderr, ok) = run(
        t.path(),
        &["leaf-add", ".", "sync", "--kind", "review-design"],
    );
    assert!(ok, "{stderr}");

    let contents = body(t.path(), "01-review-design-sync-k1.md");

    assert!(
        contents.starts_with("# sync-k1\n"),
        "the position-free handle heads the body: {contents:?}"
    );
    for absent in [
        "**Reviews:**",
        "**Integrates:**",
        "**Kind:**",
        "**Harness:**",
        "Adversarially review",
    ] {
        assert!(
            !contents.contains(absent),
            "the template must leave {absent:?} to the creating session: {contents:?}"
        );
    }
}

#[test]
fn there_is_no_chain_constructor_left_to_call() {
    // Both eager verbs are gone, and their absence has to be a *refusal* rather
    // than a silent success against some near-neighbour. A session carrying the
    // old habit gets clap's unrecognised-subcommand error and an untouched tree.
    for argv in [
        vec!["leaf-add-chain", ".", "sync", "--kind", "design"],
        vec!["leaf-promote-chain", "1"],
    ] {
        let t = grove();
        let (stdout, stderr, ok) = run(t.path(), &argv);
        assert!(!ok, "{argv:?} was accepted");
        assert_eq!(stdout, "", "{argv:?} printed a path");
        assert!(
            stderr.contains("unrecognized subcommand"),
            "{argv:?}: {stderr}"
        );
        assert_eq!(
            tree(t.path()),
            vec!["BRIEF.md", "FORMAT"],
            "{argv:?} touched the tree"
        );
    }
}

// ---------------------------------------------------------------------------
// Where the integrate step lands — pinned by asking `pick`, not by name
//
// The methodology says an integration is cut where `pick` reaches it next, and
// names the condition: `leaf-insert` at the first sibling entry after the review
// whose subtree still holds live work. No verb enforces it, so what these tests
// hold is the *rule*, in the only terms that make it a scheduling claim — what
// the walk selects. A filename-adjacency assertion cannot distinguish a rule that
// works from one that merely looks tidy, which is how the superseded
// "first live leaf after it" wording survived: it is indistinguishable from the
// correct rule until a later sibling is a **node**.
//
// Each shape therefore asserts both selections: what runs next after the cut, and
// what the *other* verb would have handed the session instead.

/// The grove-root-relative path `pick` selects, or `None` for a finished grove.
fn picked(worktree: &Path) -> Option<String> {
    let (stdout, stderr, ok) = run(worktree, &["pick"]);
    assert!(ok, "pick failed: {stderr}");
    let line = stdout.trim();
    if line.is_empty() {
        return None;
    }
    // Canonicalised, because a temp dir reaches the binary through `/var` and
    // comes back through `/private/var` on macOS.
    let root = worktree.join(".grove").canonicalize().unwrap();
    Some(
        Path::new(line)
            .canonicalize()
            .unwrap_or_else(|e| panic!("pick returned unresolvable {line:?}: {e}"))
            .strip_prefix(&root)
            .unwrap_or_else(|_| panic!("pick returned {line:?}, not a path under {root:?}"))
            .to_string_lossy()
            .into_owned(),
    )
}

fn retire(worktree: &Path, rel: &str) {
    let (_, stderr, ok) = run(worktree, &["leaf-retire", &format!(".grove/{rel}")]);
    assert!(ok, "leaf-retire {rel} failed: {stderr}");
}

fn add(worktree: &Path, parent: &str, slug: &str, kind: &str) {
    let (_, stderr, ok) = run(worktree, &["leaf-add", parent, slug, "--kind", kind]);
    assert!(ok, "leaf-add {slug} failed: {stderr}");
}

fn insert(worktree: &Path, target: &str, slug: &str, kind: &str) {
    let (_, stderr, ok) = run(
        worktree,
        &[
            "leaf-insert",
            &format!(".grove/{target}"),
            slug,
            "--kind",
            kind,
        ],
    );
    assert!(ok, "leaf-insert at {target} failed: {stderr}");
}

fn decompose(worktree: &Path, leaf: &str, child: &str) {
    let (_, stderr, ok) = run(
        worktree,
        &["leaf-decompose", &format!(".grove/{leaf}"), child],
    );
    assert!(ok, "leaf-decompose {leaf} failed: {stderr}");
}

/// A retired producer and review with one ordinary live leaf behind them — the
/// easy shape, and the only one the superseded wording also got right.
fn chain_with_a_live_sibling() -> TempDir {
    let t = grove();
    add(t.path(), ".", "sync", "design");
    add(t.path(), ".", "sync", "review-design");
    add(t.path(), ".", "unrelated", "impl");
    retire(t.path(), "01-design-sync-k1.md");
    retire(t.path(), "02-review-design-sync-k2.md");
    t
}

#[test]
fn insert_before_a_later_live_leaf_makes_the_integration_run_next() {
    let t = chain_with_a_live_sibling();
    assert_eq!(
        picked(t.path()).as_deref(),
        Some("03-impl-unrelated-k3.md"),
        "the unrelated leaf is what stands between the review and its integration"
    );

    insert(
        t.path(),
        "03-impl-unrelated-k3.md",
        "sync",
        "integrate-review-design",
    );

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("03-integrate-review-design-sync-k4.md"),
        "the integration takes the blocking sibling's slot and runs next"
    );

    // The contrast, on an identical tree: `leaf-add` appends at the parent's
    // *end*, so the unrelated leaf runs first and edits whatever it likes in the
    // files the findings cite before the integration ever opens them.
    let wrong = chain_with_a_live_sibling();
    add(wrong.path(), ".", "sync", "integrate-review-design");
    assert_eq!(
        picked(wrong.path()).as_deref(),
        Some("03-impl-unrelated-k3.md"),
        "appending puts the integration behind the leaf it needed to precede"
    );
}

#[test]
fn a_later_sibling_node_blocks_and_is_itself_the_insert_target() {
    // The shape the superseded wording gets wrong. `pick` descends a node
    // directory in place, so a live leaf *inside* a later sibling node runs
    // before anything appended after that node — the node blocks exactly as a
    // live leaf would, and the node is the entry to insert before.
    let t = grove();
    add(t.path(), ".", "sync", "design");
    add(t.path(), ".", "sync", "review-design");
    add(t.path(), ".", "follow-up", "impl");
    decompose(t.path(), "03-impl-follow-up-k3.md", "detail");
    retire(t.path(), "01-design-sync-k1.md");
    retire(t.path(), "02-review-design-sync-k2.md");

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("03-follow-up-k3/01-impl-detail-k4.md"),
        "a live descendant of a later sibling node is what runs next"
    );

    insert(
        t.path(),
        "03-follow-up-k3",
        "sync",
        "integrate-review-design",
    );

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("03-integrate-review-design-sync-k5.md"),
        "inserting before the node directory puts the integration ahead of its whole subtree"
    );
    assert!(
        t.path().join(".grove/04-follow-up-k3").is_dir(),
        "the node shifted down one position with its subtree intact"
    );
}

#[test]
fn targeting_the_blocking_nodes_descendant_inserts_at_the_wrong_level() {
    // Why the rule says *entry* and names the node as the target. Aiming at the
    // live leaf that `pick` would have selected is the natural mistake, and it is
    // silent: the integration still runs next, so nothing about the selection
    // reveals that the leaf landed a level down, inside a node whose brief
    // charters other work and whose close will now roll it up.
    let t = grove();
    add(t.path(), ".", "sync", "design");
    add(t.path(), ".", "sync", "review-design");
    add(t.path(), ".", "follow-up", "impl");
    decompose(t.path(), "03-impl-follow-up-k3.md", "detail");
    retire(t.path(), "01-design-sync-k1.md");
    retire(t.path(), "02-review-design-sync-k2.md");

    insert(
        t.path(),
        "03-follow-up-k3/01-impl-detail-k4.md",
        "sync",
        "integrate-review-design",
    );

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("03-follow-up-k3/01-integrate-review-design-sync-k5.md"),
        "it runs next either way — the defect is the level, not the order"
    );
    assert!(
        !t.path()
            .join(".grove/03-integrate-review-design-sync-k5.md")
            .exists(),
        "the integration never reached the review's own directory"
    );
}

#[test]
fn terminal_entries_between_the_steps_do_not_block_an_append() {
    // Terminal entries are exempt because `pick` never stops at one: a `DONE`
    // leaf, and a node whose whole subtree is terminal. A rule that counted them
    // would force an insert that buys nothing and renumbers live siblings for it.
    let t = grove();
    add(t.path(), ".", "sync", "design");
    add(t.path(), ".", "sync", "review-design");
    add(t.path(), ".", "old", "impl");
    add(t.path(), ".", "stale", "impl");
    decompose(t.path(), "04-impl-stale-k4.md", "gone");
    for rel in [
        "01-design-sync-k1.md",
        "02-review-design-sync-k2.md",
        "03-impl-old-k3.md",
        "04-stale-k4/01-impl-gone-k5.md",
    ] {
        retire(t.path(), rel);
    }
    assert_eq!(
        picked(t.path()),
        None,
        "nothing after the review is live, so nothing blocks"
    );

    add(t.path(), ".", "sync", "integrate-review-design");

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("05-integrate-review-design-sync-k6.md"),
        "an append at the parent's end still runs next"
    );
}

#[test]
fn live_work_in_a_later_outer_node_cannot_get_in_front_of_an_append() {
    // Why the condition is directory-local. The review sits inside a node, and a
    // later sibling *of that node* holds live work — but pre-order finishes the
    // review's own directory, including the leaf just appended to its end, before
    // it visits any later sibling of an ancestor. So there is nothing to defend
    // against and `leaf-add` is correct.
    let t = grove();
    add(t.path(), ".", "inner", "design");
    decompose(t.path(), "01-design-inner-k1.md", "sync");
    add(t.path(), "inner-k1", "sync", "review-design");
    add(t.path(), ".", "outer", "impl");
    decompose(t.path(), "02-impl-outer-k4.md", "later");
    retire(t.path(), "01-inner-k1/01-design-sync-k2.md");
    retire(t.path(), "01-inner-k1/02-review-design-sync-k3.md");
    assert_eq!(
        picked(t.path()).as_deref(),
        Some("02-outer-k4/01-impl-later-k5.md"),
        "the only live work is in the outer sibling node"
    );

    add(t.path(), "inner-k1", "sync", "integrate-review-design");

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("01-inner-k1/03-integrate-review-design-sync-k6.md"),
        "the appended integration still precedes the whole later sibling node"
    );
}

#[test]
fn the_finish_sentinel_does_not_block_an_append() {
    // The `finish` leaf is skipped while any ordinary leaf is live, so it is not
    // eligible work and cannot block — which is why the rule says *live work*
    // rather than *a live leaf*. It is written by hand because `leaf-add` refuses
    // the driver-reserved kind, which is exactly the situation being modelled:
    // the driver appended it, and ordinary work was inserted ahead of it since.
    let t = grove();
    add(t.path(), ".", "sync", "design");
    add(t.path(), ".", "sync", "review-design");
    fs::write(
        t.path().join(".grove/03-finish-finish-k3.md"),
        "# finish-k3\n",
    )
    .unwrap();
    retire(t.path(), "01-design-sync-k1.md");
    retire(t.path(), "02-review-design-sync-k2.md");

    add(t.path(), ".", "sync", "integrate-review-design");

    assert_eq!(
        picked(t.path()).as_deref(),
        Some("04-integrate-review-design-sync-k4.md"),
        "the integration appended behind the sentinel still runs before it"
    );
}

/// The release's central compatibility promise, pinned by the one shape that
/// can regress it silently: **a chain node left by the old constructor**.
///
/// Nothing was migrated, and nothing needed to be — a chain node was only ever
/// an ordinary node *directory* whose slug ended in `-chain`, and every reader
/// handles node directories generically. But that argument is only as good as
/// the readers, and after the deletion no source or test fixture contained a
/// `-chain-k` name at all: the whole current-shape suite would keep passing if
/// a future change made node parsing, `pick`'s descent, handle resolution or
/// `brief-chain` assume a charter that this node, uniquely, does not have.
///
/// So the fixture is deliberately the *awkward* legacy case — a brief-less node
/// holding a live producer and its two steps, in current filename format,
/// exactly as `leaf-add-chain` would have written it — and all three read paths
/// are exercised against it untouched.
#[test]
fn an_unmigrated_chain_node_still_picks_resolves_and_walks_its_brief_chain() {
    let t = grove();
    let node = t.path().join(".grove/01-sync-chain-k1");
    fs::create_dir_all(&node).unwrap();
    // No `BRIEF.md` — the property under test. The three steps carry the derived
    // kinds *and* the `-review` / `-integrate` stem suffixes the deleted
    // constructor emitted — a spelling the methodology no longer teaches and that
    // remains a perfectly legal filename, since the suffix was always convention
    // rather than grammar. Nothing was migrated for that change either, so this
    // fixture now guards both compatibility claims at once.
    for (name, header) in [
        ("01-design-sync-k2.md", "sync-k2"),
        ("02-review-design-sync-review-k3.md", "sync-review-k3"),
        (
            "03-integrate-review-design-sync-integrate-k4.md",
            "sync-integrate-k4",
        ),
    ] {
        fs::write(node.join(name), format!("# {header}\n")).unwrap();
    }

    // `pick` descends the node in pre-order and reaches its first live leaf,
    // rather than skipping a directory it cannot charter.
    let (stdout, stderr, ok) = run(t.path(), &["pick"]);
    assert!(ok, "pick failed on a legacy chain node: {stderr}");
    assert!(
        stdout
            .trim()
            .ends_with("01-sync-chain-k1/01-design-sync-k2.md"),
        "pick must descend the unmigrated node, got {stdout:?}"
    );

    // The children resolve by their permanent handles, which the node never
    // renumbered and no migration rewrote.
    for (handle, expected) in [
        ("sync-k2", "01-sync-chain-k1/01-design-sync-k2.md"),
        (
            "sync-review-k3",
            "01-sync-chain-k1/02-review-design-sync-review-k3.md",
        ),
    ] {
        let (stdout, stderr, ok) = run(t.path(), &["resolve", handle]);
        assert!(ok, "resolve {handle} failed: {stderr}");
        assert!(
            stdout.trim().ends_with(expected),
            "resolve {handle} gave {stdout:?}"
        );
    }

    // `brief-chain` skips the level with no charter silently and yields the root
    // brief alone — which is exactly what the close of such a node reports:
    // there is no `Done when` to check and nothing to promote.
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "brief-chain",
            ".grove/01-sync-chain-k1/01-design-sync-k2.md",
        ],
    );
    assert!(ok, "brief-chain failed: {stderr}");
    let briefs: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();
    assert_eq!(
        briefs.len(),
        1,
        "expected the root brief alone, got {briefs:?}"
    );
    assert!(briefs[0].ends_with(".grove/BRIEF.md"), "got {briefs:?}");

    // And none of it moved anything: no migration ran.
    let mut names: Vec<String> = fs::read_dir(&node)
        .unwrap()
        .filter_map(|e| e.ok())
        .map(|e| e.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    assert_eq!(
        names,
        vec![
            "01-design-sync-k2.md",
            "02-review-design-sync-review-k3.md",
            "03-integrate-review-design-sync-integrate-k4.md",
        ],
        "the legacy node must be untouched — still brief-less, still `-chain`"
    );
}

// ---------------------------------------------------------------------------
// The research pair: still one call

#[test]
fn pair_prints_three_flat_siblings_with_fixed_research_kinds() {
    let t = grove();
    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", ".", "survey"]);
    assert!(ok, "pair should succeed: {stdout} {stderr}");
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines.len(), 3, "one path per step: {stdout:?}");
    for (line, expected) in lines.iter().zip([
        "01-research-a-survey-k1.md",
        "02-research-b-survey-k2.md",
        "03-combine-research-survey-k3.md",
    ]) {
        assert!(
            line.ends_with(expected) && line.starts_with('/'),
            "expected an absolute path ending {expected}, got {line:?}"
        );
        let contents = body(t.path(), expected);
        assert!(!contents.contains("**Harness:**"), "got {contents:?}");
        assert!(!contents.contains("**Kind:**"), "got {contents:?}");
    }
}

/// The pair's half of the compatibility promise, which the legacy *chain* node
/// fixture above cannot make.
///
/// `content/TASK-FORMAT.md` and `task_grow::leaf_add_pair` both promise that both
/// slug spellings stay legal and no existing tree is invalidated, and the review
/// chain's half is guarded above. The pair's is a genuinely different parse and
/// was left uncovered: the moment the generator stopped emitting `-a` / `-b` /
/// `-combine`, no source or test fixture contained an old pair name at all, so the
/// whole current-shape suite would keep passing if a future change broke reading
/// them.
///
/// What makes it a *different* case is where the kind ends and the slug begins.
/// In `01-research-a-survey-a-k1.md` the kind label is `research-a` and the slug
/// is `survey-a` — a slug that itself ends in the step marker, and a boundary only
/// [`Kind::split_filename_prefix`]'s longest-label match puts in the right place.
/// A shorter or first-match split would silently mis-read all three names, and the
/// handle it derived (`survey-k1` rather than `survey-a-k1`) would resolve to
/// nothing.
///
/// So the three names go in exactly as the deleted generator wrote them, and are
/// read three ways without being touched. No migration and no generator fallback:
/// the current expectations above stay the bare stem.
#[test]
fn unmigrated_research_pair_filenames_still_pick_resolve_and_report_their_kinds() {
    let t = grove();
    let legacy = [
        ("01-research-a-survey-a-k1.md", "survey-a-k1", "research-a"),
        ("02-research-b-survey-b-k2.md", "survey-b-k2", "research-b"),
        (
            "03-combine-research-survey-combine-k3.md",
            "survey-combine-k3",
            "combine-research",
        ),
    ];
    for (name, handle, _) in legacy {
        fs::write(
            t.path().join(".grove").join(name),
            format!("# {handle}\n\n## Goal\n"),
        )
        .unwrap();
    }

    // `pick` reads them as ordinary well-formed live leaves and returns the first
    // in position order, rather than skipping names it cannot parse.
    let (stdout, stderr, ok) = run(t.path(), &["pick"]);
    assert!(ok, "pick failed on legacy pair names: {stderr}");
    assert!(
        stdout.trim().ends_with("01-research-a-survey-a-k1.md"),
        "pick must reach the legacy `research-a` leaf, got {stdout:?}"
    );

    for (name, handle, kind) in legacy {
        // The handle carries the old slug, and resolution finds it: the terminal
        // `-k<key>` is the key and the `-a` / `-b` / `-combine` inside the slug is
        // decorative, exactly as for any other slug.
        let (stdout, stderr, ok) = run(t.path(), &["resolve", handle]);
        assert!(ok, "resolve {handle} failed: {stderr}");
        assert!(
            stdout.trim().ends_with(name),
            "resolve {handle} gave {stdout:?}"
        );

        // …and the kind/slug boundary lands where the longest label puts it, which
        // is the assertion the chain fixture has no way to make.
        let (stdout, stderr, ok) = run(t.path(), &["kind", name]);
        assert!(ok, "kind {name} failed: {stderr}");
        assert_eq!(stdout.trim(), kind, "kind {name} gave {stdout:?}");
    }

    // Nothing moved: reading a legacy name never rewrites it.
    assert_eq!(
        tree(t.path()),
        vec![
            "01-research-a-survey-a-k1.md",
            "02-research-b-survey-b-k2.md",
            "03-combine-research-survey-combine-k3.md",
            "BRIEF.md",
            "FORMAT",
        ],
        "the legacy pair names must be untouched"
    );
}

#[test]
fn a_failed_run_prints_no_path_at_all() {
    // Printing as each path lands would let stdout describe a mutation the
    // command reports as failed — and the run is rolled back, so those paths do
    // not exist by the time the caller reads them.
    //
    // The obstruction is a *directory* wearing a leaf's name: task-shaped, but
    // not the species its `.md` suffix declares, so the read that precedes
    // allocation refuses it. The run therefore fails after validating and
    // before writing, which is the arm stdout silence has to survive.
    let t = grove();
    fs::create_dir(t.path().join(".grove").join("01-research-a-survey-k1.md")).unwrap();

    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", ".", "survey"]);

    assert!(!ok, "the run must fail");
    assert_eq!(
        stdout, "",
        "not one path on stdout for a shape that was not created"
    );
    assert!(
        stderr.contains("01-research-a-survey-k1.md"),
        "the diagnostic names the entry standing in the way: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["01-research-a-survey-k1.md", "BRIEF.md", "FORMAT"],
        "no half-built pair left behind"
    );
}

#[test]
fn pair_refuses_a_kind_because_its_kinds_are_fixed() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &["leaf-add-pair", ".", "survey", "--kind", "design"],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("unexpected argument '--kind'"),
        "the pair has no configurable kind: {stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "nothing created"
    );
}

#[test]
fn pair_help_exposes_no_routing_flags() {
    let t = grove();
    let (stdout, _, ok) = run(t.path(), &["leaf-add-pair", "--help"]);
    assert!(ok);
    assert!(!stdout.contains("harness"), "got {stdout:?}");
    assert!(!stdout.contains("--kind"), "got {stdout:?}");
}

#[test]
fn pair_rejects_removed_harness_flags() {
    let t = grove();
    let (stdout, stderr, ok) = run(
        t.path(),
        &[
            "leaf-add-pair",
            ".",
            "survey",
            "--harness-a",
            "claude",
            "--harness-b",
            "codex",
        ],
    );
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(
        stderr.contains("unexpected argument '--harness-a'"),
        "{stderr}"
    );
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "nothing created"
    );
}

#[test]
fn pair_requires_parent_and_stem() {
    let t = grove();
    let (stdout, stderr, ok) = run(t.path(), &["leaf-add-pair", "."]);
    assert!(!ok);
    assert_eq!(stdout, "");
    assert!(stderr.contains("<STEM>"), "{stderr}");
    assert_eq!(
        tree(t.path()),
        vec!["BRIEF.md", "FORMAT"],
        "a malformed command must not leave a partial pair on disk"
    );
}

#[test]
fn the_surviving_composite_verb_sits_beside_leaf_add_in_help() {
    // compose-task-chains-k29's failure mode was five documented surfaces and
    // zero chains: a verb nobody reaches for is that failure with a compile
    // step. `--help` is the surface a session re-orients on, so the one verb a
    // session cannot express with `leaf-add` has to be visible there, next to
    // the verb it composes.
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    for verb in ["leaf-add", "leaf-add-pair"] {
        assert!(
            s.contains(verb),
            "`{verb}` missing from grove-llm --help: {s}"
        );
    }
    for gone in ["leaf-add-chain", "leaf-promote-chain"] {
        assert!(
            !s.contains(gone),
            "`{gone}` no longer exists and must not be advertised: {s}"
        );
    }
    let at = |v: &str| s.find(v).unwrap();
    assert!(
        at("leaf-add") < at("leaf-add-pair"),
        "the composite verb sits immediately after the verb it composes: {s}"
    );
}

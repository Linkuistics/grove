//! The `syllabus` binary, driven from outside: a real process, real argv, real
//! streams, real exit codes, and a real directory.
//!
//! **Contract tests, not unit tests**, and that is the point of the leaf rather
//! than a stylistic preference. Every test before this one in this crate was
//! written by someone who had read `docs/ordinal-fs-tree/ARCHITECTURE.md`; a CLI
//! forces a real `Display`, real error text and a real domain implementation
//! through the same surface, so awkwardness that shows up here is evidence about
//! the seam.
//!
//! Every test names the model claim it discharges, or says it has none. **Most
//! say they have none, and that is the honest reading rather than a gap**:
//! neither `structure.als` nor `operations.qnt` holds strings, arguments,
//! streams or exit codes, so the routing rule in `docs/formalism-findings.md`
//! entry 009 predicted almost no coverage here before the leaf started. What the
//! few claim-naming tests below check is that a *modelled* outcome survives the
//! trip out through argv and back through stdout — not the outcome itself, which
//! the on-disk suites already discharge.
//!
//! The `#![cfg]` is what makes `--no-default-features` build: without it the
//! `env!` below is unresolvable when the binary is not built.
#![cfg(feature = "cli")]

use std::fs;
use std::path::{Path, PathBuf};

use assert_cmd::Command;
use tempfile::TempDir;

// ---------------------------------------------------------------------------
// Driving the binary
// ---------------------------------------------------------------------------

/// One invocation's whole observable result.
struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

impl Run {
    /// stdout parsed the way the help text tells a caller to parse it: one
    /// record per line, **split on the first tab**, because a caller-supplied
    /// `--root` may contain one.
    fn records(&self) -> Vec<(&str, &str)> {
        self.stdout
            .lines()
            .map(|line| {
                line.split_once('\t')
                    .expect("every stdout line is `<key>` TAB `<path>`")
            })
            .collect()
    }

    fn targets(&self) -> Vec<&str> {
        self.records().iter().map(|(target, _)| *target).collect()
    }

    fn paths(&self) -> Vec<&str> {
        self.records().iter().map(|(_, path)| *path).collect()
    }
}

fn syllabus(root: &Path, args: &[&str]) -> Run {
    let output = Command::new(env!("CARGO_BIN_EXE_syllabus"))
        .arg("--root")
        .arg(root)
        .args(args)
        .output()
        .expect("running the syllabus binary");
    Run {
        code: output.status.code().expect("the binary exited normally"),
        stdout: String::from_utf8(output.stdout).expect("stdout is UTF-8"),
        stderr: String::from_utf8(output.stderr).expect("stderr is UTF-8"),
    }
}

/// Run and require success, so a test that fails for an unrelated reason says so
/// with the tool's own message rather than with an empty-output mismatch.
fn ok(root: &Path, args: &[&str]) -> Run {
    let run = syllabus(root, args);
    assert_eq!(run.code, 0, "`syllabus {args:?}` failed: {}", run.stderr);
    run
}

/// An empty directory *is* an empty tree — there is no `init` and nothing to
/// create but the directory.
fn empty_tree() -> (TempDir, PathBuf) {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("syllabus");
    fs::create_dir(&root).expect("creating the tree root");
    (temporary, root)
}

/// A small course: two modules, a lesson beside them, two lessons inside the
/// first module. Keys 1–5, built by the tool itself.
fn a_course() -> (TempDir, PathBuf) {
    let (temporary, root) = empty_tree();
    ok(&root, &["module-add", ".", "linear-algebra", "calculus"]);
    ok(&root, &["lesson-add", ".", "orientation"]);
    ok(&root, &["lesson-add", "1", "vectors", "matrices"]);
    (temporary, root)
}

// ---------------------------------------------------------------------------
// The shape of stdout
// ---------------------------------------------------------------------------

/// No model claim: neither model holds strings or streams.
///
/// The key column exists so that output round-trips into the next call. A caller
/// that could only read paths would have to re-implement the domain's grammar to
/// recover a key, which is the one thing this library exists to prevent.
#[test]
fn every_key_stdout_prints_names_the_entry_that_line_is_about() {
    let (_temporary, root) = a_course();

    for (target, path) in ok(&root, &["list"]).records() {
        let shown = ok(&root, &["show", target]);
        assert_eq!(
            shown.paths(),
            vec![path],
            "`show {target}` did not name the line `list` printed it on"
        );
    }
}

/// No model claim.
///
/// Paths are built from the caller's own spelling of the root and nothing is
/// canonicalised, which is the library's own property surfaced. Driven with two
/// spellings of one tree, because a single spelling cannot tell a preserved path
/// from a canonicalised one that happens to match.
#[test]
fn a_path_comes_back_spelled_the_way_the_root_went_in() {
    let (_temporary, root) = a_course();
    let direct = ok(&root, &["show", "1"]);
    assert_eq!(
        direct.paths(),
        vec![root
            .join("01-linear-algebra-i1")
            .display()
            .to_string()
            .as_str()]
    );

    // The same tree, named through its parent with a `..` in the middle: a
    // spelling the kernel resolves to the same inode and no string function
    // would produce.
    let indirect = root.join("..").join("syllabus");
    let run = ok(&indirect, &["show", "1"]);
    assert_eq!(
        run.paths(),
        vec![indirect
            .join("01-linear-algebra-i1")
            .display()
            .to_string()
            .as_str()]
    );
}

/// No model claim.
///
/// The parsing rule is *split on the first tab*, and this is the case that makes
/// it a rule rather than a convention: a root the caller spelled with a tab in it
/// puts a second tab on every line.
#[test]
fn a_root_holding_a_tab_still_yields_one_record_per_line() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("a\tcourse");
    fs::create_dir(&root).expect("creating a tree root whose name holds a tab");
    ok(&root, &["module-add", ".", "linear-algebra"]);

    let run = ok(&root, &["list"]);
    assert_eq!(run.targets(), vec!["1"]);
    assert!(
        run.paths()[0].ends_with("a\tcourse/01-linear-algebra-i1"),
        "the path after the first tab was {:?}",
        run.paths()[0]
    );
}

/// No model claim.
///
/// A distinguished child carries no key of its own and no operation can name
/// one, so its line names the **level whose content it is** — which is the handle
/// a caller reading `overview-chain` or `list` actually needs.
#[test]
fn an_overview_is_listed_against_the_level_whose_content_it_is() {
    let (_temporary, root) = a_course();
    fs::write(root.join("OVERVIEW.md"), "the course").expect("writing the root's own content");
    fs::write(
        root.join("01-linear-algebra-i1").join("OVERVIEW.md"),
        "the module",
    )
    .expect("writing a module's own content");

    let run = ok(&root, &["list"]);
    let overviews: Vec<_> = run
        .records()
        .into_iter()
        .filter(|(_, path)| path.ends_with("OVERVIEW.md"))
        .collect();

    assert_eq!(overviews.len(), 2);
    assert_eq!(overviews[0].0, ".", "the root's own content names the root");
    assert_eq!(
        overviews[1].0, "1",
        "a module's own content names the module"
    );
}

// ---------------------------------------------------------------------------
// Reading
// ---------------------------------------------------------------------------

/// No model claim: walk *order* is unmodelled — `operations.qnt`'s handoff block
/// says so — so this holds the order against `ARCHITECTURE.md`'s prose and
/// nothing more.
#[test]
fn list_walks_depth_first_with_a_node_explored_before_its_next_sibling() {
    let (_temporary, root) = a_course();
    assert_eq!(
        ok(&root, &["list"]).targets(),
        vec!["1", "4", "5", "2", "3"]
    );
}

/// No model claim.
///
/// `--first` sends the predicate to `find`, which short-circuits, and the
/// difference from a filtered walk is observable only in how many records come
/// back.
#[test]
fn first_stops_at_one_match_where_a_filtered_walk_returns_every_one() {
    let (_temporary, root) = a_course();
    assert_eq!(
        ok(&root, &["list", "--status", "draft"]).targets(),
        vec!["4", "5", "3"]
    );
    assert_eq!(
        ok(&root, &["list", "--status", "draft", "--first"]).targets(),
        vec!["4"]
    );
}

/// No model claim.
///
/// A distinguished child carries no parts, so it matches no filter and is
/// dropped whenever one is given — and `--under` is the CLI's own filter over
/// `ancestors()`, not a subtree walk the library offers.
#[test]
fn under_keeps_descendants_and_a_filter_drops_the_overview() {
    let (_temporary, root) = a_course();
    fs::write(
        root.join("01-linear-algebra-i1").join("OVERVIEW.md"),
        "the module",
    )
    .expect("writing a module's own content");

    // `--under` alone keeps the module's own content and not the module itself.
    assert_eq!(
        ok(&root, &["list", "--under", "1"]).targets(),
        vec!["1", "4", "5"]
    );
    // Any predicate drops it, because it has no parts to match.
    assert_eq!(
        ok(&root, &["list", "--under", "1", "--status", "draft"]).targets(),
        vec!["4", "5"]
    );
    assert_eq!(
        ok(&root, &["list", "--label", "vectors"]).targets(),
        vec!["4"]
    );
}

/// No model claim.
///
/// The ancestor chain ends at the tree root, which is a level and **not** an
/// entry: it has no ordinal, no key and no parts, so its record is `.` and the
/// root's own spelling.
#[test]
fn ancestors_ends_at_the_root_which_is_a_level_and_not_an_entry() {
    let (_temporary, root) = a_course();
    let run = ok(&root, &["ancestors", "4"]);
    assert_eq!(run.targets(), vec![".", "1"]);
    assert_eq!(run.paths()[0], root.display().to_string());
}

/// No model claim.
///
/// `overview-chain` walks the target's **ancestors**, so a module's own OVERVIEW
/// is not in its own chain — the one thing about this verb a reader guesses
/// wrong, which is why the help text says it and why this test exists.
#[test]
fn an_overview_chain_holds_the_ancestors_content_and_not_the_targets_own() {
    let (_temporary, root) = a_course();
    fs::write(root.join("OVERVIEW.md"), "the course").expect("the root's own content");
    let module = root.join("01-linear-algebra-i1");
    fs::write(module.join("OVERVIEW.md"), "the module").expect("the module's own content");

    // From a lesson inside the module: both levels above it, root-first.
    let from_lesson = ok(&root, &["overview-chain", "4"]);
    assert_eq!(from_lesson.targets(), vec![".", "1"]);

    // From the module itself: only the root's, not its own.
    let from_module = ok(&root, &["overview-chain", "1"]);
    assert_eq!(from_module.targets(), vec!["."]);
    assert_eq!(
        from_module.paths(),
        vec![root.join("OVERVIEW.md").display().to_string().as_str()]
    );
}

/// No model claim.
///
/// A read verb whose result is empty says which emptiness it was, on stderr, and
/// exits 0: an empty tree is a tree.
#[test]
fn an_empty_result_is_a_success_that_says_which_emptiness_it_was() {
    let (_temporary, empty) = empty_tree();
    let nothing_there = ok(&empty, &["list"]);
    assert!(nothing_there.stdout.is_empty());
    assert!(
        nothing_there.stderr.contains("holds no entries"),
        "{}",
        nothing_there.stderr
    );

    let (_temporary, root) = a_course();
    let excluded = ok(&root, &["list", "--label", "nothing-is-called-this"]);
    assert!(excluded.stdout.is_empty());
    assert!(
        excluded.stderr.contains("filters excluded"),
        "{}",
        excluded.stderr
    );
}

// ---------------------------------------------------------------------------
// Mutating
// ---------------------------------------------------------------------------

/// Discharges `wit_appendManySucceeded` — through argv and back out of stdout.
///
/// Both `add` verbs are variadic and both call `append_many`, including for a
/// single label, so what this exercises is the property only a run has:
/// consecutive ordinals with consecutive keys, planned from one snapshot.
#[test]
fn an_add_run_lands_at_consecutive_ordinals_with_consecutive_keys() {
    let (_temporary, root) = empty_tree();
    let run = ok(&root, &["lesson-add", ".", "one", "two", "three"]);

    assert_eq!(run.targets(), vec!["1", "2", "3"]);
    for (n, (_, path)) in run.records().iter().enumerate() {
        let expected = format!("0{}-draft-", n + 1);
        assert!(
            path.contains(&expected),
            "{path} is not at ordinal {}",
            n + 1
        );
    }
}

/// No model claim: this is about which stream carries what.
///
/// A mutation prints created-if-any on stdout and everything else on stderr —
/// the siblings a shift moves are the price of the subject rather than the
/// subject.
#[test]
fn an_insert_prints_the_new_entry_and_puts_the_shifted_siblings_on_stderr() {
    let (_temporary, root) = a_course();
    let run = ok(&root, &["lesson-insert", "1", "1", "prerequisites"]);

    assert_eq!(run.targets(), vec!["6"]);
    assert_eq!(run.stdout.lines().count(), 1);
    assert!(run.stderr.contains("lesson-insert: 3 effects"));
    assert!(!run.stdout.contains("vectors"));
    assert!(run.stderr.contains("vectors"));
}

/// No model claim here — the shift order is `inv_ordinalsDistinctThroughout`'s
/// and `tests/inserting_on_disk.rs` discharges it. What this holds is that the
/// order stays **observable to an operator**, which is why it is a property of a
/// value (`Report::paths()`) rather than of a loop's direction.
#[test]
fn the_landing_trace_shows_the_shift_running_highest_ordinal_first() {
    let (_temporary, root) = a_course();
    let run = ok(&root, &["lesson-insert", "1", "1", "prerequisites"]);

    let trace: Vec<&str> = run.stderr.lines().skip(1).collect();
    assert_eq!(trace.len(), 3);
    assert!(trace[0].starts_with("  renamed") && trace[0].contains("matrices"));
    assert!(trace[1].starts_with("  renamed") && trace[1].contains("vectors"));
    assert!(trace[2].starts_with("  created") && trace[2].contains("prerequisites"));
}

/// Discharges `inv_promoteKeepsIdentity` and `wit_promoteWithChild`, through the
/// binary: the node keeps the leaf's **own** ordinal and its **own** key, so a
/// caller holding the key still resolves it.
///
/// The bytes moving verbatim is the library's own proposition and is unmodelled
/// by construction — content is outside both models — so it is checked here
/// against content this CLI never wrote, which is exactly what having no `add`
/// with content buys.
#[test]
fn a_promotion_keeps_the_lessons_key_and_moves_its_bytes_into_the_overview() {
    let (_temporary, root) = a_course();
    let lesson = root.join("03-draft-orientation-i3.md");
    fs::write(&lesson, "# Orientation\n").expect("writing a lesson's bytes by hand");

    let run = ok(
        &root,
        &["promote", "3", "orientation", "--first-lesson", "welcome"],
    );

    // stdout is the new module, then the first lesson. The promoted file, now
    // the module's own content, is a consequence and is on stderr.
    assert_eq!(run.targets(), vec!["3", "6"]);
    let module = root.join("03-orientation-i3");
    assert_eq!(run.paths()[0], module.display().to_string());
    assert_eq!(
        fs::read_to_string(module.join("OVERVIEW.md")).expect("the module's own content"),
        "# Orientation\n"
    );
    assert!(!lesson.exists());

    // The key still resolves, which is the whole reason the key exists.
    assert_eq!(
        ok(&root, &["show", "3"]).paths(),
        vec![module.display().to_string().as_str()]
    );
}

/// Discharges `wit_rewriteToSameParts` — held up by two mechanisms rather than
/// one, and this is the second: the algebra declines to refuse it and the
/// interpreter declines to perform it, and the report still names the entry.
#[test]
fn publishing_a_published_lesson_succeeds_and_changes_nothing() {
    let (_temporary, root) = a_course();
    let once = ok(&root, &["publish", "4"]);
    let published = root
        .join("01-linear-algebra-i1")
        .join("01-published-vectors-i4.md");
    assert_eq!(once.paths(), vec![published.display().to_string().as_str()]);

    let again = ok(&root, &["publish", "4"]);
    assert_eq!(
        again.paths(),
        vec![published.display().to_string().as_str()]
    );
    assert!(published.exists());
    assert_eq!(
        ok(&root, &["list"]).targets(),
        vec!["1", "4", "5", "2", "3"]
    );
}

/// No model claim.
///
/// A rewrite keeps the entry's ordinal, its key and its species; `relabel` keeps
/// the variant it read, which is why `RewriteSpeciesChange` is unreachable from
/// this verb — and it keeps a lesson's status, because a second place a status
/// could be set is a second place it can be wrong.
#[test]
fn relabelling_keeps_the_place_the_key_the_kind_and_the_status() {
    let (_temporary, root) = a_course();
    ok(&root, &["publish", "5"]);

    let lesson = ok(&root, &["relabel", "5", "matrix-algebra"]);
    assert_eq!(lesson.targets(), vec!["5"]);
    assert!(lesson.paths()[0].ends_with("02-published-matrix-algebra-i5.md"));

    let module = ok(&root, &["relabel", "1", "linear-algebra-i"]);
    assert_eq!(module.targets(), vec!["1"]);
    assert!(module.paths()[0].ends_with("01-linear-algebra-i-i1"));
    // The subtree came with the directory, untouched.
    assert_eq!(
        ok(&root, &["list"]).targets(),
        vec!["1", "4", "5", "2", "3"]
    );
}

/// No model claim.
///
/// `--quiet` suppresses the advisory stream and nothing else. The mutation still
/// happens and stdout still carries the answer.
#[test]
fn quiet_silences_the_trace_and_leaves_stdout_alone() {
    let (_temporary, root) = a_course();
    let run = ok(
        &root,
        &["--quiet", "lesson-insert", "1", "1", "prerequisites"],
    );
    assert_eq!(run.targets(), vec!["6"]);
    assert!(run.stderr.is_empty(), "{}", run.stderr);
}

// ---------------------------------------------------------------------------
// Refusals, and the exit code each one earns
// ---------------------------------------------------------------------------

/// Discharges `wit_refusedTargetMissing`. The CLI **constructs** this one for
/// the read verbs, because `by_key` answers with an `Option` and re-wording one
/// condition is where `docs/formalism-findings.md` entry 017 found drift landing.
#[test]
fn a_key_that_names_nothing_is_exit_three_from_every_verb_that_takes_one() {
    let (_temporary, root) = a_course();
    for args in [
        vec!["show", "99"],
        vec!["ancestors", "99"],
        vec!["overview-chain", "99"],
        vec!["list", "--under", "99"],
        vec!["publish", "99"],
        vec!["unpublish", "99"],
        vec!["relabel", "99", "x"],
        vec!["promote", "99", "x"],
        vec!["lesson-add", "99", "x"],
        vec!["module-insert", "99", "1", "x"],
    ] {
        let run = syllabus(&root, &args);
        assert_eq!(run.code, 3, "`syllabus {args:?}` exited {}", run.code);
        assert!(run
            .stderr
            .starts_with("syllabus: no entry in this tree has key 99"));
        assert!(run.stdout.is_empty());
    }
}

/// Discharges `wit_refusedTargetNotNode`.
#[test]
fn adding_into_a_lesson_is_refused_with_the_remedy_named() {
    let (_temporary, root) = a_course();
    let run = syllabus(&root, &["lesson-add", "4", "sections"]);
    assert_eq!(run.code, 4);
    assert!(
        run.stderr.contains("promote it first, or name a node"),
        "{}",
        run.stderr
    );
}

/// Discharges `wit_insertPastTheEnd` and `wit_insertIntoAGap` — the two
/// situations behind one refusal, which is how the level's occupied span comes to
/// be carried. This is also the discovery loop the ordinal argument buys: an
/// operator who guesses is told the least and greatest ordinals the level holds.
#[test]
fn an_insert_at_an_unoccupied_ordinal_names_the_span_the_level_occupies() {
    let (_temporary, root) = a_course();

    let past_the_end = syllabus(&root, &["lesson-insert", "1", "9", "late"]);
    assert_eq!(past_the_end.code, 4);
    assert!(
        past_the_end.stderr.contains("past the last sibling"),
        "{}",
        past_the_end.stderr
    );

    // A gap, and a hole below the first, both reachable only on a hand-edited
    // level — no operation this tool offers can make one.
    let module = root.join("01-linear-algebra-i1");
    fs::rename(
        module.join("01-draft-vectors-i4.md"),
        module.join("04-draft-vectors-i4.md"),
    )
    .expect("hand-editing a hole into a level");

    let gap = syllabus(&root, &["lesson-insert", "1", "3", "middle"]);
    assert_eq!(gap.code, 4);
    assert!(gap.stderr.contains("gap"), "{}", gap.stderr);

    let below = syllabus(&root, &["lesson-insert", "1", "1", "early"]);
    assert_eq!(below.code, 4);
    assert!(
        below.stderr.contains("lowest this level occupies"),
        "{}",
        below.stderr
    );
}

/// Discharges `wit_refusedPromoteNotLeaf`.
#[test]
fn promoting_a_module_is_refused() {
    let (_temporary, root) = a_course();
    let run = syllabus(&root, &["promote", "1", "algebra"]);
    assert_eq!(run.code, 4);
    assert!(run.stderr.starts_with("syllabus: "), "{}", run.stderr);
}

/// Discharges `wit_refusedDestinationOccupied` — reachable only on a tree
/// hand-edited to duplicate a key, because a tree this library built cannot
/// hold one.
#[test]
fn a_duplicated_key_refuses_the_operation_it_collides_with() {
    let (_temporary, root) = a_course();
    let module = root.join("01-linear-algebra-i1");
    // A twin of key 4 sitting on the very name publishing key 4 would take. The
    // draft spelling sorts first, so `by_key` answers with it — the documented
    // tie-break — and its destination is the twin.
    fs::write(
        module.join("01-published-vectors-i4.md"),
        "a hand-made twin",
    )
    .expect("hand-editing a duplicate key into the tree");

    let run = syllabus(&root, &["publish", "4"]);
    assert_eq!(run.code, 4);
    assert!(run.stderr.contains("already taken"), "{}", run.stderr);
}

/// Discharges no claim — an integer is unbounded in both models, so neither can
/// pose exhaustion at all. A `Key` is a `u32`, and a hand-written name carrying
/// the maximum makes every allocation after it impossible.
#[test]
fn a_hand_written_maximum_key_exhausts_allocation_rather_than_wrapping() {
    let (_temporary, root) = empty_tree();
    fs::write(
        root.join(format!("01-draft-edge-i{}.md", u32::MAX)),
        "at the top",
    )
    .expect("hand-writing a name that carries the greatest key there is");

    let run = syllabus(&root, &["lesson-add", ".", "next"]);
    assert_eq!(run.code, 4);
    assert!(run.stderr.starts_with("syllabus: "), "{}", run.stderr);
}

/// No model claim: both models hold no strings, so a grammar refusal is
/// unreachable in either. This is the parse trichotomy's whole point — a name
/// this domain recognises and cannot read halts the tree rather than being
/// skipped, because a skipped **directory** takes its subtree with it.
#[test]
fn a_name_this_domain_owns_and_cannot_read_halts_the_tree_at_exit_five() {
    let (_temporary, root) = a_course();
    // Non-canonical: the grammar writes `05-`, and accepting `5-` would make two
    // filenames one entry.
    fs::write(root.join("5-draft-hand-typed-i9.md"), "").expect("a lenient spelling");

    let run = syllabus(&root, &["list"]);
    assert_eq!(run.code, 5);
    assert!(
        run.stderr
            .contains("Rename it to `05-draft-hand-typed-i9.md`"),
        "{}",
        run.stderr
    );
    assert!(run.stdout.is_empty(), "a halted read prints no records");
}

/// No model claim.
///
/// A reserved name is the domain's and is deliberately not an entry, so it halts
/// the same way — and the advice is the **domain's own**, which is the whole
/// reason `Error::Malformed` and `Error::Reserved` carry `EntryName::Err`.
#[test]
fn a_reserved_name_halts_with_the_domains_own_recovery_advice() {
    let (_temporary, root) = a_course();
    fs::write(root.join("PUBLISHING"), "").expect("the witness of an interrupted run");

    let run = syllabus(&root, &["list"]);
    assert_eq!(run.code, 5);
    assert!(
        run.stderr.contains("delete the file to release the tree"),
        "{}",
        run.stderr
    );
}

/// No model claim. A foreign name is skipped **silently**, which is safe
/// precisely because the domain disclaimed it — the counterpart to the test
/// above, and the pair is what makes the trichotomy visible from outside.
#[test]
fn a_name_this_domain_disclaims_is_skipped_without_comment() {
    let (_temporary, root) = a_course();
    fs::write(root.join("README.md"), "notes for a human").expect("a foreign file");
    fs::create_dir(root.join(".git")).expect("a foreign directory");

    assert_eq!(
        ok(&root, &["list"]).targets(),
        vec!["1", "4", "5", "2", "3"]
    );
}

/// No model claim.
///
/// The environment refusing is not the tree refusing, and the two want different
/// answers from a caller: exit 1 says fix the path or the permissions.
#[test]
fn a_root_that_is_not_there_is_exit_one() {
    let (temporary, _root) = empty_tree();
    let run = syllabus(&temporary.path().join("no-such-tree"), &["list"]);
    assert_eq!(run.code, 1);
    assert!(run.stderr.starts_with("syllabus: "), "{}", run.stderr);
}

/// No model claim.
///
/// Usage is exit 2, which is clap's own default for a parse failure — inherited
/// rather than chosen, so that a hand-written usage error and clap's agree.
#[test]
fn bad_arguments_are_exit_two_and_say_what_a_good_one_looks_like() {
    let (_temporary, root) = a_course();

    let label = syllabus(&root, &["lesson-add", ".", "Not A Label"]);
    assert_eq!(label.code, 2);
    assert!(
        label.stderr.contains("starts with a letter"),
        "{}",
        label.stderr
    );

    let status = syllabus(&root, &["list", "--status", "somewhen"]);
    assert_eq!(status.code, 2);
    assert!(
        status.stderr.contains("`draft` or `published`"),
        "{}",
        status.stderr
    );

    let key = syllabus(&root, &["show", "the-first-one"]);
    assert_eq!(key.code, 2);
    assert!(
        key.stderr.contains("decimal a name carries after `i`"),
        "{}",
        key.stderr
    );

    let unknown = syllabus(&root, &["remove", "4"]);
    assert_eq!(unknown.code, 2);
}

/// No model claim.
///
/// Publishing a module is refused by the **CLI**, before the library sees it:
/// modules carry no publication status here, so there are no parts to compose
/// and nothing ever reaches `rewrite`. It is the CLI's own message, and it names
/// the verb the operator probably wanted.
#[test]
fn publishing_a_module_is_the_clis_own_refusal_and_points_at_relabel() {
    let (_temporary, root) = a_course();
    let run = syllabus(&root, &["publish", "1"]);
    assert_eq!(run.code, 4);
    assert!(
        run.stderr.contains("syllabus relabel 1 <label>"),
        "{}",
        run.stderr
    );
    // Nothing moved.
    assert!(ok(&root, &["show", "1"]).paths()[0].ends_with("01-linear-algebra-i1"));
}

// ---------------------------------------------------------------------------
// Help text
// ---------------------------------------------------------------------------

/// The twelve verbs, named once so the two help tests cannot drift apart.
const VERBS: [&str; 12] = [
    "list",
    "show",
    "ancestors",
    "overview-chain",
    "lesson-add",
    "module-add",
    "lesson-insert",
    "module-insert",
    "promote",
    "relabel",
    "publish",
    "unpublish",
];

/// No model claim.
///
/// An agent's whole experience of a tool is dominated by `--help` and by what it
/// sees when something goes wrong. The flat verb list is what lets a caller that
/// has lost its bearings recover in one call, and the exit-code table is what
/// lets it branch without parsing prose.
#[test]
fn one_help_call_enumerates_every_verb_and_the_exit_codes() {
    let (_temporary, root) = a_course();
    let help = ok(&root, &["--help"]);

    for verb in VERBS {
        assert!(
            help.stdout.contains(verb),
            "`--help` never mentions `{verb}`"
        );
    }
    for code in 0..=7 {
        assert!(
            help.stdout.contains(&format!("\n  {code}  ")),
            "`--help` documents no exit code {code}"
        );
    }
    assert!(help.stdout.contains("EXAMPLES"));
    assert!(
        help.stdout.contains("THERE IS NO REMOVAL"),
        "the help text must say why nothing is deleted"
    );
    assert!(
        help.stdout.contains("BLOCKS"),
        "the help text must say the verbs block on a lock"
    );
}

/// No model claim.
///
/// Two or three real examples per verb is the item an LLM caller pattern-matches
/// on hardest, so their absence is a defect rather than a polish item.
#[test]
fn every_verbs_help_carries_examples_and_a_see_also() {
    let (_temporary, root) = a_course();
    for verb in VERBS {
        let help = ok(&root, &[verb, "--help"]);
        assert!(
            help.stdout.contains("EXAMPLES"),
            "`{verb} --help` carries no examples"
        );
        assert!(
            help.stdout.contains("SEE ALSO"),
            "`{verb} --help` carries no see-also"
        );
        assert!(
            help.stdout.matches("  syllabus ").count() >= 2,
            "`{verb} --help` carries fewer than two example invocations"
        );
    }
}

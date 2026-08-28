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

#[cfg(unix)]
use std::ffi::OsString;
use std::fs;
#[cfg(unix)]
use std::os::unix::ffi::OsStringExt;
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
    /// stdout parsed through the first half of the documented rule: one record
    /// per line, split on the first tab. Tests with escaped platform bytes
    /// percent-decode the path separately.
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

/// An empty directory *is* an empty tree: `init` is what creates a tree from
/// nothing, and a directory that is already there is a tree holding no entries.
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
/// The parsing rule is *split on the first tab, then percent-decode the path*.
/// A tab in the caller's spelling is record structure unless the path encoder
/// escapes it.
#[test]
fn a_root_holding_a_tab_still_yields_one_record_per_line() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("a\tcourse");
    fs::create_dir(&root).expect("creating a tree root whose name holds a tab");
    ok(&root, &["module-add", ".", "linear-algebra"]);

    let run = ok(&root, &["list"]);
    assert_eq!(run.targets(), vec!["1"]);
    assert!(
        run.paths()[0].ends_with("a%09course/01-linear-algebra-i1"),
        "the path after the first tab was {:?}",
        run.paths()[0]
    );
}

/// No model claim: neither model holds strings or streams.
///
/// A record must remain one UTF-8 physical line and preserve delimiters. The
/// in-file stream tests cover a non-UTF-8 byte without depending on whether the
/// host filesystem or its sandbox admits such a name.
#[cfg(unix)]
#[test]
fn record_paths_round_trip_delimiters() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary
        .path()
        .join(OsString::from_vec(b"a\n%course".to_vec()));
    fs::create_dir(&root).expect("creating a tree root containing delimiters");
    ok(&root, &["module-add", ".", "linear-algebra"]);

    let run = ok(&root, &["list"]);
    assert_eq!(run.stdout.lines().count(), 1, "{:?}", run.stdout);
    let encoded = run.paths()[0];
    assert!(
        encoded.contains("a%0A%25course/01-linear-algebra-i1"),
        "the path was not bytewise percent-encoded: {encoded:?}"
    );
    assert_eq!(decode_unix_path(encoded), root.join("01-linear-algebra-i1"));
}

#[cfg(unix)]
fn decode_unix_path(encoded: &str) -> PathBuf {
    let mut bytes = Vec::with_capacity(encoded.len());
    let mut index = 0;
    while index < encoded.len() {
        if encoded.as_bytes()[index] == b'%' {
            let end = index + 3;
            let octet = u8::from_str_radix(&encoded[index + 1..end], 16)
                .expect("a percent escape contains two hexadecimal digits");
            bytes.push(octet);
            index = end;
        } else {
            bytes.push(encoded.as_bytes()[index]);
            index += 1;
        }
    }
    PathBuf::from(OsString::from_vec(bytes))
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
/// `--first` sends the predicate to `seek`, which short-circuits, and the
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

/// No model claim: `Sought` is a type-level distinction neither model has.
///
/// The `Nothing` half of a `seek`, at the CLI seam, and the point is what it is
/// **not**. `show 99` and `list --first` over a predicate nothing matches are
/// the same library answer — a search that completed and matched nothing — and
/// the CLI chooses opposite policies over it: exit 3 with a refusal there, exit
/// 0 with an empty listing and a note here. Neither is the library's call, which
/// is precisely why the library refuses to make it by answering `Refusal`.
#[test]
fn first_over_a_predicate_nothing_matches_is_an_empty_listing() {
    let (_temporary, root) = a_course();
    let run = ok(&root, &["list", "--label", "topology", "--first"]);
    assert_eq!(run.stdout, "", "nothing matched, so no record is printed");
    assert!(
        run.stderr.contains("no entry matched"),
        "an empty listing says why it is empty: {}",
        run.stderr
    );
    // The same predicate over a full walk, for the same reason and to the same
    // effect: `--first` changes how much work is done, not what nothing means.
    assert_eq!(ok(&root, &["list", "--label", "topology"]).stdout, "");
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
/// the read verbs, because `by_key` answers with a `Sought` — which is not a
/// refusal — and re-wording one condition is where `docs/formalism-findings.md`
/// entry 017 found drift landing. That the CLI treats a search matching nothing
/// as a failure is *its* policy: `list --first` takes the same answer and prints
/// an empty listing, which is [`first_over_a_predicate_nothing_matches_is_an_empty_listing`].
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
/// **A root that is not there stopped being an environment failure at
/// `open-shape-k25`.** It used to be exit 1 — the library could not stat the
/// root, so the CLI saw an I/O error and said *fix the path or the permissions*.
/// Now the library answers with a vacancy, which is not a failure at all: the
/// tree is simply absent, and the honest answer names the verb that creates one.
/// Exit 4 is *refused: nothing changed, and the message names the remedy*, which
/// is exactly what this is.
#[test]
fn a_root_that_is_not_there_is_refused_with_the_verb_that_would_create_it() {
    let (temporary, _root) = empty_tree();
    let run = syllabus(&temporary.path().join("no-such-tree"), &["list"]);
    assert_eq!(run.code, 4, "{}", run.stderr);
    assert!(run.stderr.starts_with("syllabus: "), "{}", run.stderr);
    assert!(
        run.stderr.contains("no tree at") && run.stderr.contains("init"),
        "the refusal names the remedy: {}",
        run.stderr
    );
}

/// No model claim: a root that is not a directory is not a tree either model can
/// hold.
///
/// The third answer to *is there a tree here*, and the reason it is an error
/// rather than a variant: the library will not move aside something it did not
/// put there. Exit 5 — a human fixes it, and no retry helps.
#[test]
fn a_root_that_is_a_regular_file_is_neither_a_tree_nor_a_vacancy() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("not-a-tree");
    fs::write(&root, "a file wearing the root's name").expect("a fixture");

    let listed = syllabus(&root, &["list"]);
    assert_eq!(listed.code, 5, "{}", listed.stderr);
    assert!(
        listed.stderr.contains("is a regular file"),
        "the message says what it found: {}",
        listed.stderr
    );

    // And `init` does not clear it away either — the one verb that creates a
    // tree still refuses a root that is occupied.
    let initialized = syllabus(&root, &["init"]);
    assert_eq!(initialized.code, 5, "{}", initialized.stderr);
}

/// No model claim: root creation is `initialize`, whose transition
/// `operations.qnt` gains as `Initialize` — this is the CLI's view of it.
///
/// One command turns nothing into a tree with an OVERVIEW and two lessons, and
/// the tree then reads back through the ordinary verbs.
#[test]
fn init_creates_the_tree_its_overview_and_its_first_lessons() {
    let temporary = TempDir::new().expect("a temporary directory");
    let root = temporary.path().join("course");

    let created = ok(
        &root,
        &[
            "init",
            "--overview",
            "An introduction.",
            "orientation",
            "vectors",
        ],
    );
    assert_eq!(
        created.targets(),
        vec![".", "1", "2"],
        "the OVERVIEW is reported against the level whose content it is, then \
         the lessons at consecutive keys: {}",
        created.stdout
    );
    assert_eq!(
        fs::read_to_string(root.join("OVERVIEW.md")).expect("the root's own content"),
        "An introduction."
    );
    assert_eq!(
        ok(&root, &["list"]).targets(),
        vec![".", "1", "2"],
        "and the tree reads back through the ordinary verbs"
    );
}

/// No model claim.
///
/// `--overview` omitted is a root with no OVERVIEW at all, which is a different
/// tree from one whose OVERVIEW is empty. Both are reachable, and neither is a
/// default the CLI picks.
#[test]
fn an_omitted_overview_and_an_empty_one_are_different_trees() {
    let temporary = TempDir::new().expect("a temporary directory");

    let bare = temporary.path().join("bare");
    ok(&bare, &["init"]);
    assert!(!bare.join("OVERVIEW.md").exists(), "no OVERVIEW at all");

    let empty = temporary.path().join("empty");
    ok(&empty, &["init", "--overview", ""]);
    assert_eq!(
        fs::read_to_string(empty.join("OVERVIEW.md")).expect("an OVERVIEW"),
        ""
    );
}

/// No model claim.
///
/// `init` is refused over a live tree — and it is refused by the *CLI*, because
/// the library is not asked: `initialize` lives on `Vacancy`, so the call the
/// tree arm would make does not exist to be made. Not idempotent, deliberately:
/// the call that thinks it is creating a course and the call that finds one
/// already there want different answers.
#[test]
fn init_over_a_live_tree_is_refused_and_changes_nothing() {
    let (_temporary, root) = a_course();
    let before = ok(&root, &["list"]).stdout.clone();

    let run = syllabus(&root, &["init"]);
    assert_eq!(run.code, 4, "{}", run.stderr);
    assert!(
        run.stderr.contains("already a tree"),
        "the refusal says what it found: {}",
        run.stderr
    );
    assert_eq!(
        ok(&root, &["list"]).stdout,
        before,
        "a refusal changes nothing"
    );
}

/// No model claim.
///
/// **`--yes` is the confirmation, because a prompt is not answerable here.** The
/// binary's consumers are contract tests and scripts, so an interactive
/// `[y/N]` would make the one destructive verb undrivable by everything that
/// actually drives it. A flag is the same confirmation in a form both an
/// operator and a script can give, and the refusal without it says the whole
/// command to re-run.
#[test]
fn delete_without_yes_is_refused_and_removes_nothing() {
    let (_temporary, root) = a_course();
    let before = ok(&root, &["list"]).stdout.clone();

    let run = syllabus(&root, &["delete"]);
    assert_eq!(run.code, 4, "{}", run.stderr);
    assert!(
        run.stderr.contains("delete --yes"),
        "the refusal names the command to re-run: {}",
        run.stderr
    );
    assert_eq!(ok(&root, &["list"]).stdout, before, "nothing was removed");
}

/// No model claim.
///
/// **stdout is the root; the entries are the trace.** Deletion's subject is the
/// root — one record, keyed `.`, exactly as the key column's rule says a level
/// is named — and everything beneath it is the consequence, which is the same
/// split `lesson-insert` makes between the entry it created and the siblings it
/// shifted. The order the entries went is stderr's, in the order they went.
#[test]
fn delete_prints_the_root_on_stdout_and_what_went_on_stderr() {
    let (_temporary, root) = a_course();
    let run = ok(&root, &["delete", "--yes"]);

    assert_eq!(run.records().len(), 1, "one record: {}", run.stdout);
    assert_eq!(run.targets(), vec!["."]);
    assert_eq!(run.paths(), vec![root.to_str().expect("a UTF-8 root")]);
    assert!(
        run.stderr.contains("in the order they went"),
        "the trace says what it is: {}",
        run.stderr
    );
    assert!(
        run.stderr
            .contains("01-linear-algebra-i1/01-draft-vectors-i4.md"),
        "a lesson two levels down is in the trace: {}",
        run.stderr
    );
    assert!(!root.exists(), "the root is gone");
}

/// No model claim.
///
/// **A leading `..` cancels no name, so it is an ordinary spelling and is
/// accepted.** That is the other side of the rule that refuses
/// `syllabus/topic/..`, and it matters because `../course` is what an operator
/// standing one directory below actually types. It needs a working directory to
/// be relative *to*, which is why it is a contract test here rather than a
/// library one.
#[test]
fn a_root_spelled_with_a_leading_parent_is_deleted() {
    let (temporary, root) = a_course();
    let beside = temporary.path().join("beside");
    fs::create_dir(&beside).expect("a directory to stand in");

    let output = Command::new(env!("CARGO_BIN_EXE_syllabus"))
        .current_dir(&beside)
        .args(["--root", "../syllabus", "delete", "--yes"])
        .output()
        .expect("running the syllabus binary");

    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        ".\t../syllabus\n",
        "the root, keyed `.`, in the caller's own spelling"
    );
    assert!(!root.exists(), "and the tree is gone");
}

/// No model claim.
///
/// **A root spelled through a symbolic link is refused, and only by `delete`.**
/// Every other verb accepts the spelling — the library goes out of its way to
/// make two spellings of one tree take one lock — because they use the root as
/// the directory things are in. A deletion acts on the root itself, where a link
/// and what it names are two objects and only one is the tree.
#[test]
fn deleting_through_a_symbolic_link_is_refused_where_reading_through_it_is_not() {
    let (temporary, root) = a_course();
    let link = temporary.path().join("elsewhere");
    std::os::unix::fs::symlink(&root, &link).expect("a link naming the tree");

    // Reading through it works, which is what makes the refusal below a
    // statement about deletion rather than about the spelling.
    assert_eq!(ok(&link, &["list"]).records().len(), 5);

    let run = syllabus(&link, &["delete", "--yes"]);
    assert_eq!(run.code, 4, "{}", run.stderr);
    assert!(
        run.stderr.contains("is a symbolic link"),
        "the refusal names the spelling: {}",
        run.stderr
    );
    assert!(root.is_dir(), "the tree is untouched");
    assert!(link.symlink_metadata().is_ok(), "and so is the link");
}

/// No model claim.
///
/// A second `delete` meets a **vacancy**, which every verb but `init` refuses in
/// one place and with one sentence. So a deletion is not idempotent in the
/// direction a caller might hope: the tree it was asked about is not there, and
/// saying nothing about that would make a mistyped `--root` indistinguishable
/// from a job already done.
#[test]
fn a_second_delete_finds_no_tree_and_says_so() {
    let (_temporary, root) = a_course();
    ok(&root, &["delete", "--yes"]);

    let run = syllabus(&root, &["delete", "--yes"]);
    assert_eq!(run.code, 4, "{}", run.stderr);
    assert!(
        run.stderr.contains("there is no tree at"),
        "the same sentence every other verb gives for a vacancy: {}",
        run.stderr
    );
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

/// The fourteen verbs, named once so the two help tests cannot drift apart.
const VERBS: [&str; 14] = [
    "init",
    "delete",
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
        help.stdout.contains("percent-decode `%HH`"),
        "the record decoder is absent from top-level help"
    );
    assert!(
        help.stdout.contains("REMOVAL: THE WHOLE TREE OR NOTHING"),
        "the help text must separate removing an entry, which is refused, from \
         deleting the root, which is the one destructive verb"
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

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;

mod support;

fn init_repo() -> TempDir {
    let temporary_directory = TempDir::new().unwrap();
    support::init_jj_repo(temporary_directory.path());
    temporary_directory
}

fn current_grove(repository: &Path) -> PathBuf {
    let grove = repository.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    grove
}

fn write_leaf(grove: &Path, name: &str, body: &str) -> PathBuf {
    let path = grove.join(name);
    fs::write(&path, body).unwrap();
    path
}

fn grove_llm(repository: &Path, arguments: &[&str]) -> std::process::Output {
    Command::cargo_bin("grove-llm")
        .unwrap()
        .env("HOME", support::fixture_home())
        .current_dir(repository)
        .args(arguments)
        .output()
        .unwrap()
}

fn stdout(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}

#[test]
fn filename_kind_is_unambiguous_and_body_routing_metadata_is_ignored() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    let leaf = write_leaf(
        &grove,
        "01-integrate-review-requirements--review-notes-k7.md",
        "# review-notes-k7\n\n**Kind:** impl\n**Harness:** codex\n",
    );

    let output = grove_llm(repository.path(), &["kind", leaf.to_str().unwrap()]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output), "integrate-review-requirements\n");
}

/// The kind set maintains a non-prefix invariant — no label plus `-` prefixes
/// another (`docs/ARCHITECTURE.md`, *task-tree-scheme*).
/// `src/leaf.rs` pins that over the *set*; what the set cannot pin is that the
/// parser honours the `-` boundary. Without it `01-designer-notes-k1.md` reads
/// as kind `design` plus slug `er-notes` and launches a session no human wrote —
/// silently, because both halves are individually well-formed.
///
/// Stated in both directions, because one alone is satisfiable by a parser that
/// is simply wrong: refusing everything passes the first loop, and matching any
/// prefix passes the second.
#[test]
fn the_kind_label_boundary_is_exact_in_both_directions() {
    // A token that merely *starts with* a kind label is not that kind.
    for name in [
        "01-designer-notes-k1.md",
        "01-implementation-k1.md",
        "01-prototypes-spike-k1.md",
        // `review` and `integrate-review` are routing families, not members.
        "01-review-notes-k1.md",
        "01-integrate-review-notes-k1.md",
        // Standalone legacy `research` becomes `research-a` at migration; on a
        // current tree the bare spelling is not a kind.
        "01-research-survey-k1.md",
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        write_leaf(&grove, name, "# notes-k1\n");

        let output = grove_llm(repository.path(), &["pick"]);

        assert!(!output.status.success(), "{name} was accepted as a leaf");
        assert!(stderr(&output).contains(name), "{}", stderr(&output));
    }

    // ...and a *slug* is free to begin with a kind label. All four coexist in
    // one tree, so a strictness failure on any single name fails the read.
    let repository = init_repo();
    let grove = current_grove(repository.path());
    let cases = [
        (
            "01-impl--review-design-notes-k1.md",
            "impl",
            "review-design-notes-k1",
        ),
        ("02-review-impl--design-k2.md", "review-impl", "design-k2"),
        (
            "03-integrate-review-impl--impl-k3.md",
            "integrate-review-impl",
            "impl-k3",
        ),
        ("04-design--research-a-k4.md", "design", "research-a-k4"),
    ];
    for (name, _, _) in cases {
        write_leaf(&grove, name, "# leaf\n");
    }

    for (name, kind, handle) in cases {
        let path = grove.join(name);
        let output = grove_llm(repository.path(), &["kind", path.to_str().unwrap()]);
        assert!(output.status.success(), "{name}: {}", stderr(&output));
        assert_eq!(stdout(&output), format!("{kind}\n"), "{name}");

        // The handle is what survives the tree, so the split has to leave the
        // *slug* right too — not merely name a kind that happens to parse.
        let output = grove_llm(repository.path(), &["resolve", handle]);
        assert!(output.status.success(), "{handle}: {}", stderr(&output));
        assert!(stdout(&output).contains(name), "{}", stdout(&output));
    }
}

#[test]
fn current_tree_refuses_a_task_shaped_leaf_with_no_separator() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-untyped-k1.md", "# untyped-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    // A leaf with no `--` is refused on its *shape*, before any question about
    // which kind it names — so the advice is the grammar and not the kind set
    // (`grammar-separator-k15`). Every name the old grammar wrote lands here,
    // which is why the refusal has to carry the canonical form.
    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("01-untyped-k1.md"), "{error}");
    assert!(
        error.contains("NN-[DONE-|ABANDONED-]<session-kind>--<slug>-k<key>.md"),
        "{error}"
    );
}

#[test]
fn current_tree_reads_a_task_shaped_leaf_whose_kind_no_skill_declares() {
    // **The scenario `open-kind-k20` inverted.** A kind grove has never heard of
    // used to halt the read with a refusal listing all nineteen labels; grove
    // holds no set now, so the name is well-formed, `pick` returns it, and the
    // failure — if there is one — belongs to the session that could not load a
    // `grove-mystery` skill. That is the spec's own scenario: *an unknown kind
    // reaches the loop* → the tree parses and the launch proceeds.
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-mystery--untyped-k1.md", "# untyped-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("01-mystery--untyped-k1.md"));

    let output = grove_llm(
        repository.path(),
        &["kind", ".grove/01-mystery--untyped-k1.md"],
    );
    assert!(output.status.success(), "{}", stderr(&output));
    assert_eq!(stdout(&output).trim(), "mystery");
}

#[test]
fn current_tree_refuses_a_task_shaped_leaf_whose_kind_is_not_a_token() {
    // What is left to refuse: a token that cannot be written into a name and
    // read back. The refusal names the file, the token, and the character.
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-My_Kind--untyped-k1.md", "# untyped-k1\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    let error = stderr(&output);
    assert!(error.contains("01-My_Kind--untyped-k1.md"), "{error}");
    assert!(error.contains("\"My_Kind\""), "{error}");
    assert!(error.contains("'M'"), "{error}");
}

#[test]
fn pick_skips_finish_while_non_finish_work_is_live() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish--finish-k1.md", "# finish-k1\n");
    write_leaf(&grove, "02-impl--work-k2.md", "# work-k2\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("02-impl--work-k2.md"));
}

#[test]
fn pick_selects_finish_when_it_is_the_only_live_work() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish--finish-k1.md", "# finish-k1\n");
    write_leaf(&grove, "02-DONE-impl--finished-k2.md", "# finished-k2\n");

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("01-finish--finish-k1.md"));
}

#[test]
fn pick_refuses_duplicate_live_finish_leaves() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish--finish-k1.md", "# finish-k1\n");
    write_leaf(
        &grove,
        "02-finish--finish-again-k2.md",
        "# finish-again-k2\n",
    );

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(!output.status.success());
    assert!(stderr(&output).contains("multiple live `finish` leaves"));
}

#[test]
fn terminal_infixes_preserve_filename_kind_and_stable_resolution() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    let done = write_leaf(
        &grove,
        "01-DONE-review-impl--reviewed-k7.md",
        "# reviewed-k7\n",
    );
    let abandoned = write_leaf(
        &grove,
        "02-ABANDONED-integrate-review-design--integrated-k8.md",
        "# integrated-k8\n",
    );

    for (path, expected) in [
        (&done, "review-impl\n"),
        (&abandoned, "integrate-review-design\n"),
    ] {
        let output = grove_llm(repository.path(), &["kind", path.to_str().unwrap()]);
        assert!(output.status.success(), "{}", stderr(&output));
        assert_eq!(stdout(&output), expected);
    }

    let output = grove_llm(repository.path(), &["resolve", "reviewed-k7"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("01-DONE-review-impl--reviewed-k7.md"));

    let output = grove_llm(repository.path(), &["resolve", "[8]"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(stdout(&output).contains("02-ABANDONED-integrate-review-design--integrated-k8.md"));
}

#[test]
fn malformed_terminal_task_names_fail_as_strictly_as_live_names() {
    for name in [
        "01-DONE-untyped-k1.md",
        "01-ABANDONED-mystery-untyped-k1.md",
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        write_leaf(&grove, name, "# untyped-k1\n");

        let output = grove_llm(repository.path(), &["pick"]);

        assert!(!output.status.success(), "{name} was accepted");
        assert!(stderr(&output).contains(name), "{}", stderr(&output));
    }
}

/// Strictness is a rule about **task-shaped names**, not about Markdown, and the
/// directory half is the one with teeth: a leaf skipped costs one task, a node
/// directory skipped costs its whole subtree. Grove never writes any of these —
/// `leaf-retire` and `leaf-prune` refuse a node operand outright — so each is
/// reachable only by hand, which is exactly the mistake the design predicts,
/// because "a node is never marked done" is a rule a human has to *know* rather
/// than one the filename grammar makes unstateable.
///
/// Each fixture hides a **live** leaf behind the malformed entry. The old answer
/// was silence: `pick` printed "no live leaves; this grove is done" and the driver's
/// next move was to allocate a finish leaf and propose teardown.
#[test]
fn a_task_shaped_entry_of_the_wrong_species_is_malformed_not_foreign() {
    // (entry name, whether it is a directory, the phrase naming what went wrong).
    // The phrases are the **domain's** since `reading-k31` put the reading verbs
    // on `ordinal-fs-tree`: the refusal is `task_name`'s own `TaskNameError`,
    // carried verbatim by the library's halt, rather than a sentence grove's
    // own withdrawn level reader wrote. Same conditions, same recovery advice,
    // different words — which is a finding recorded in the node brief and not an
    // adjustment made quietly here.
    for (name, directory, expected) in [
        // An outcome infix on a node name: parses as neither species.
        ("01-DONE-node-k1", true, "never marked DONE or ABANDONED"),
        // A leaf's name on a directory: parses as a leaf, is not one.
        ("01-impl--decoy-k1.md", true, "names a leaf"),
        // A node's name on a regular file: parses as a node, is not one.
        ("01-decoy-k1", false, "names a node"),
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        let entry = grove.join(name);
        if directory {
            fs::create_dir_all(&entry).unwrap();
            fs::write(entry.join("01-impl--hidden-k2.md"), "# hidden-k2\n").unwrap();
        } else {
            fs::write(&entry, "# not a node\n").unwrap();
        }

        for verb in [vec!["pick"], vec!["resolve", "hidden-k2"]] {
            let output = grove_llm(repository.path(), &verb);
            let diagnostic = stderr(&output);

            assert!(
                !output.status.success(),
                "{name}: `{}` accepted a malformed tree: {}",
                verb.join(" "),
                stdout(&output)
            );
            assert!(
                stdout(&output).is_empty(),
                "{name}: `{}` printed a path for a tree it refused: {}",
                verb.join(" "),
                stdout(&output)
            );
            assert!(
                diagnostic.contains(name),
                "{name}: `{}` did not name the entry: {diagnostic}",
                verb.join(" ")
            );
            assert!(
                diagnostic.contains(expected),
                "{name}: `{}` did not say what is wrong: {diagnostic}",
                verb.join(" ")
            );
        }
    }
}

/// The other half of the same rule, and the reason it can be stated as one: a name
/// that is *not* task-shaped stays foreign and ignored, whatever its species. The
/// strictness rule buys nothing if it also refuses a `notes/` directory someone
/// keeps beside their tasks.
#[test]
fn entries_outside_the_task_shaped_grammar_stay_foreign_at_either_species() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    fs::create_dir_all(grove.join("notes")).unwrap();
    fs::create_dir_all(grove.join("done")).unwrap();
    fs::write(grove.join("README.md"), "not a task\n").unwrap();
    // Positioned but unkeyed, and keyed but unpositioned: neither is task-shaped.
    fs::write(grove.join("01-notes.md"), "not a task\n").unwrap();
    fs::create_dir_all(grove.join("scratch-k9")).unwrap();
    let node = grove.join("01-real-k1");
    fs::create_dir_all(&node).unwrap();
    fs::write(node.join("01-impl--work-k2.md"), "# work-k2\n").unwrap();

    let output = grove_llm(repository.path(), &["pick"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("01-real-k1/01-impl--work-k2.md"),
        "{}",
        stdout(&output)
    );

    let output = grove_llm(repository.path(), &["resolve", "work-k2"]);
    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("01-real-k1/01-impl--work-k2.md"),
        "{}",
        stdout(&output)
    );
}

#[test]
fn non_finish_work_can_be_inserted_before_a_reserved_finish_leaf() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish--finish-k1.md", "# finish-k1\n");

    let output = grove_llm(
        repository.path(),
        &["leaf-insert", "finish-k1", "late-work", "--kind", "impl"],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(grove.join("01-impl--late-work-k2.md").exists());
    assert!(grove.join("02-finish--finish-k1.md").exists());
}

/// Finish is *reserved*, not *blocking*, and the design states both halves:
/// `leaf-insert` sequences work ahead of it, and ordinary `leaf-add` may also
/// append later work because finish selection cannot starve it
/// (`docs/ARCHITECTURE.md`, *task-tree-scheme*). The appended shape is the one that
/// bites — the finish leaf keeps the *earlier* position, so nothing but the skip
/// rule stops teardown being proposed while live work sits behind it.
#[test]
fn work_appended_behind_a_reserved_finish_leaf_is_still_selected() {
    let repository = init_repo();
    let grove = current_grove(repository.path());
    write_leaf(&grove, "01-finish--finish-k1.md", "# finish-k1\n");

    let output = grove_llm(
        repository.path(),
        &["leaf-add", ".", "late-work", "--kind", "impl"],
    );

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        grove.join("02-impl--late-work-k2.md").exists(),
        "append must land behind the finish leaf"
    );
    assert!(
        grove.join("01-finish--finish-k1.md").exists(),
        "appending must not renumber the finish leaf"
    );

    let output = grove_llm(repository.path(), &["pick"]);

    assert!(output.status.success(), "{}", stderr(&output));
    assert!(
        stdout(&output).contains("02-impl--late-work-k2.md"),
        "the later live leaf must outrank the earlier finish sentinel: {}",
        stdout(&output)
    );
}

#[test]
fn every_agent_side_mutation_refuses_the_driver_reserved_finish_kind() {
    let add_repository = init_repo();
    current_grove(add_repository.path());
    assert_finish_refusal(grove_llm(
        add_repository.path(),
        &["leaf-add", ".", "reserved", "--kind", "finish"],
    ));

    let insert_repository = init_repo();
    let insert_grove = current_grove(insert_repository.path());
    write_leaf(&insert_grove, "01-impl--target-k1.md", "# target-k1\n");
    assert_finish_refusal(grove_llm(
        insert_repository.path(),
        &["leaf-insert", "target-k1", "reserved", "--kind", "finish"],
    ));

    for verb in [
        vec!["leaf-decompose", ".grove/01-finish--finish-k1.md", "child"],
        vec!["leaf-retire", ".grove/01-finish--finish-k1.md"],
        vec!["leaf-prune", ".grove/01-finish--finish-k1.md"],
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        write_leaf(&grove, "01-finish--finish-k1.md", "# finish-k1\n");
        assert_finish_refusal(grove_llm(repository.path(), &verb));
        assert!(grove.join("01-finish--finish-k1.md").exists());
    }
}

/// A leftover `FINISHING-*` directory is **foreign**, not a held tree.
///
/// It used to be `Verdict::Reserved` — a witness the finish transaction wrote,
/// and every reader and mutator refused while one existed
/// (`docs/adr/task-tree-transactions-fail-closed.md`). There is no transaction
/// and no witness now (`delete-finish-transaction-k8`), so nothing grove writes
/// can produce that name; one on disk is a stray from an older build, and a
/// stray is a name every reader skips. This is the same answer `delete-migration-k6`
/// reached for a stray `.grove/FORMAT`, and it is asserted rather than assumed
/// because the behaviour change is deliberate.
#[test]
fn a_leftover_finish_witness_is_a_foreign_entry_every_verb_walks_past() {
    for arguments in [
        vec!["pick"],
        vec!["kind"],
        vec!["resolve", "task-k1"],
        vec!["brief-chain", ".grove/01-impl--task-k1.md"],
    ] {
        let repository = init_repo();
        let grove = current_grove(repository.path());
        fs::create_dir_all(grove.join("FINISHING-finish-k2")).unwrap();
        fs::write(grove.join("BRIEF.md"), "# demo — brief\n").unwrap();
        write_leaf(&grove, "01-impl--task-k1.md", "# task-k1\n");

        let output = grove_llm(repository.path(), &arguments);

        assert!(
            output.status.success(),
            "{arguments:?} refused a stray witness: {}",
            stderr(&output)
        );
    }
}

fn assert_finish_refusal(output: std::process::Output) {
    assert!(!output.status.success(), "finish mutation succeeded");
    let error = stderr(&output);
    assert!(error.contains("finish"), "{error}");
    assert!(error.contains("driver-reserved"), "{error}");
}

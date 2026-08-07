// Fixture-driven tests for `grove-llm kind` on the **v2 directory scheme**
// (task-tree-scheme). `kind` prints a leaf's task kind — one of the closed
// seventeen (ADR `task-kind-taxonomy`; membership in
// `docs/ARCHITECTURE.md#task-kind-taxonomy`) — read from its `**Kind:**` line through
// `leaf::Kind::parse_read` (the read-side counterpart of the `--kind` write
// gate). It is the primitive the self-driving loop uses to choose each
// session's launch harness and model by the picked leaf's kind
// (model-per-task-kind). With no argument it reads `pick`'s next live leaf; on
// an empty grove it emits the standard "no live leaves" diagnostic on stderr
// and exits 0 (mirroring `brief-chain`). Reading **degrades**: a missing or
// unrecognised `**Kind:**` line warns on stderr and is treated as `impl` rather
// than erroring (write still gates — see `tests/leaf.rs`'s invalid-`--kind`
// coverage). The one thing that is neither a match nor a degrade is the retired
// spelling `work`, which resolves to `impl` *silently*. Each test stands up a
// real git repo so `git rev-parse --show-toplevel` resolves to the fixture path.

mod support;

use assert_cmd::Command;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as Pcmd;
use tempfile::TempDir;

fn init_repo() -> TempDir {
    let tmp = TempDir::new().unwrap();
    Pcmd::new("git")
        .arg("init")
        .arg(tmp.path())
        .status()
        .unwrap();
    Pcmd::new("git")
        .args(["-C"])
        .arg(tmp.path())
        .args(["commit", "--allow-empty", "-m", "init"])
        .status()
        .unwrap();
    let grove = tmp.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), b"session-kinds-v1\n").unwrap();
    tmp
}

/// Write a leaf task file with the given `**Kind:**` label, creating parent dirs.
fn touch_leaf(dir: &Path, name: &str, kind_label: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join(name),
        format!("# stub\n\n**Kind:** {kind_label}\n\n## Goal\n").as_bytes(),
    )
    .unwrap();
}

/// Write a bare file (no `**Kind:**` line), creating parent dirs.
fn touch(dir: &Path, name: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(dir.join(name), b"# stub\n").unwrap();
}

/// Create a node directory, returning its path (for nesting children inside).
fn mknode(dir: &Path, name: &str) -> PathBuf {
    let p = dir.join(name);
    fs::create_dir_all(&p).unwrap();
    p
}

fn run(cwd: &Path, args: &[&str]) -> (String, String, bool) {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(cwd)
        .args(args)
        .output()
        .unwrap();
    (
        String::from_utf8_lossy(&out.stdout).into_owned(),
        String::from_utf8_lossy(&out.stderr).into_owned(),
        out.status.success(),
    )
}

#[test]
fn kind_of_an_impl_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "impl");

    let (stdout, _, ok) = run(tmp.path(), &["kind", ".grove/01-impl-build-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "impl\n");
}

#[test]
fn kind_of_a_planning_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-planning-design-k1.md", "impl");

    let (stdout, _, ok) = run(tmp.path(), &["kind", ".grove/01-planning-design-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "planning\n");
}

#[test]
fn every_one_of_the_nineteen_round_trips_through_the_verb() {
    // The verb is the loop driver's only view of a leaf's kind, so the whole set
    // has to survive the file → stdout round trip, hyphens and all — a single
    // lowercase token plus a newline, with nothing on stderr.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    for (i, label) in support::KIND_LABELS.iter().enumerate() {
        let name = format!("{:02}-{label}-a-k{}.md", i + 1, i + 1);
        touch_leaf(&grove, &name, "bogus");
        let (stdout, stderr, ok) = run(tmp.path(), &["kind", &format!(".grove/{name}")]);
        assert!(ok, "{label} failed: {stderr:?}");
        assert_eq!(stdout, format!("{label}\n"));
        assert!(stderr.is_empty(), "{label} warned unexpectedly: {stderr:?}");
    }
}

#[test]
fn the_retired_work_label_reads_as_impl_without_a_warning() {
    // The compatibility rule the `work` → `impl` rename stands on
    // (task-kind-taxonomy). Silence is half the contract: every task file of
    // every live grove says `work`, so warning here would fire constantly and
    // teach the operator to ignore the diagnostic that matters.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "work");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-build-k1.md"]);
    assert!(ok);
    assert_eq!(stdout, "impl\n");
    assert!(
        stderr.is_empty(),
        "the previous spelling is not a degrade — no warning, got {stderr:?}"
    );
}

#[test]
fn no_arg_form_reads_picks_next_leaf() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");
    let node = mknode(&grove, "01-node-k1");
    touch(&node, "BRIEF.md");
    // pick's next live leaf is the node's first child — a planning leaf.
    touch_leaf(&node, "01-planning-first-k2.md", "impl");

    let (stdout, _, ok) = run(tmp.path(), &["kind"]);
    assert!(ok);
    assert_eq!(stdout, "planning\n");
}

#[test]
fn empty_grove_prints_no_live_leaves_on_stderr_and_exits_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md"); // brief only — no live leaf

    let (stdout, stderr, ok) = run(tmp.path(), &["kind"]);
    assert!(ok, "empty grove must exit 0");
    assert!(stdout.is_empty(), "stdout must be empty, got {stdout:?}");
    assert!(
        stderr.contains("no live leaves"),
        "expected the standard diagnostic, got {stderr:?}"
    );
}

#[test]
fn a_body_with_no_kind_line_does_not_affect_the_filename_kind() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "01-impl-broken-k1.md"); // `# stub` only — no `**Kind:**` line

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-broken-k1.md"]);
    assert!(ok, "the filename supplies the kind");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty(), "body metadata is ignored: {stderr:?}");
}

#[test]
fn a_garbled_body_kind_is_ignored() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-broken-k1.md", "sideways");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-broken-k1.md"]);
    assert!(ok, "body kind tokens are not parsed");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty(), "body metadata is ignored: {stderr:?}");
}

#[test]
fn a_family_name_written_in_the_body_is_ignored() {
    // `review` and `integrate-review` are routing *families*, not members of the
    // set. On a leaf they are unrecognised — a naive prefix match would quietly
    // pick one of the five `review-*` kinds and misroute the session.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    for (name, label) in [
        ("01-impl-a-k1.md", "review"),
        ("02-impl-b-k2.md", "integrate-review"),
    ] {
        touch_leaf(&grove, name, label);
        let (stdout, stderr, ok) = run(tmp.path(), &["kind", &format!(".grove/{name}")]);
        assert!(ok, "{label} is body text, not routing input");
        assert_eq!(stdout, "impl\n", "{label} must not match a review-* kind");
        assert!(stderr.is_empty(), "{label} must not warn: {stderr:?}");
    }
}

#[test]
fn a_typoed_body_kind_is_ignored_with_exit_zero() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-broken-k1.md", "reserch");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-impl-broken-k1.md"]);
    assert!(ok, "exit 0 even on a typo'd kind");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty(), "body metadata is ignored: {stderr:?}");
}

// ── `--with-harness`: the loop driver's peek (leaf-harness-k15) ──────────
//
// The second routing axis rides the *same* verb call, on a second line, so the
// driver resolves both launch facts in one subprocess. Everything below is the
// contract that flag carries — including the two ways it deliberately differs
// from the kind axis: undeclared is the common answer, and a bad declaration
// **refuses** where a bad kind degrades.

/// Write a leaf that declares both a kind and a harness, in the shape the leaf
/// template writes them — the `**Harness:**` line immediately under `**Kind:**`.
fn touch_leaf_with_harness(dir: &Path, name: &str, kind_label: &str, harness: &str) {
    fs::create_dir_all(dir).unwrap();
    fs::write(
        dir.join(name),
        format!("# stub\n\n**Kind:** {kind_label}\n**Harness:** {harness}\n\n## Goal\n").as_bytes(),
    )
    .unwrap();
}

#[test]
fn with_harness_compatibility_flag_does_not_read_body_metadata() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-research-a-survey-k1.md", "impl", "codex");

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &[
            "kind",
            "--with-harness",
            ".grove/01-research-a-survey-k1.md",
        ],
    );
    assert!(ok, "a declared harness is not an error: {stderr:?}");
    assert_eq!(stdout, "research-a\n", "only the filename kind is returned");
}

#[test]
fn with_harness_adds_nothing_when_the_leaf_declares_none() {
    // The zero-cost path, and the reason the driver can pass this flag
    // unconditionally: for a leaf with no `**Harness:**` line the output is
    // byte-identical to the plain form, so nothing downstream has to branch on
    // whether the flag was passed.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "impl");

    let (plain, _, _) = run(tmp.path(), &["kind", ".grove/01-impl-build-k1.md"]);
    let (with_flag, stderr, ok) = run(
        tmp.path(),
        &["kind", "--with-harness", ".grove/01-impl-build-k1.md"],
    );
    assert!(ok, "{stderr:?}");
    assert_eq!(with_flag, plain);
    assert_eq!(with_flag, "impl\n");
}

#[test]
fn the_plain_form_never_reads_the_harness_line() {
    // `kind` prints the kind. A leaf whose `**Harness:**` line is garbage still
    // has a perfectly good kind, and asking for it must not fail — the refusal
    // belongs to the *launch* path, which is the only caller that would act on
    // the declaration.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-research-a-survey-k1.md", "impl", "codx");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", ".grove/01-research-a-survey-k1.md"]);
    assert!(ok, "the plain form must not gate on the harness line");
    assert_eq!(stdout, "research-a\n");
    assert!(stderr.is_empty(), "and must not warn about it: {stderr:?}");
}

#[test]
fn an_unrecognised_body_harness_is_ignored() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-impl-survey-k1.md", "research", "codx");

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["kind", "--with-harness", ".grove/01-impl-survey-k1.md"],
    );
    assert!(ok, "body harness metadata is not parsed: {stderr:?}");
    assert_eq!(stdout, "impl\n");
    assert!(stderr.is_empty());
}

#[test]
fn an_empty_body_harness_line_is_ignored() {
    // A `**Harness:**` with nothing after it is a declaration its author did not
    // finish, not the absence of one. Reading it as undeclared would silently
    // run the leaf on the stamp — the same failure as an unknown name, arrived
    // at by a different typo.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(
        grove.join("01-impl-survey-k1.md"),
        b"# stub\n\n**Kind:** research\n**Harness:**\n",
    )
    .unwrap();

    let (_, stderr, ok) = run(
        tmp.path(),
        &["kind", "--with-harness", ".grove/01-impl-survey-k1.md"],
    );
    assert!(ok, "body harness metadata is not parsed: {stderr:?}");
    assert!(stderr.is_empty());
}

#[test]
fn with_harness_ignores_trailing_body_commentary() {
    // Same parse as `**Kind:**` — first whitespace token, rest ignored — so a
    // leaf that explains *why* it names a vendor still routes.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(
        grove.join("01-impl-survey-k1.md"),
        b"# stub\n\n**Kind:** research\n**Harness:** codex   (the pair's second survey)\n",
    )
    .unwrap();

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["kind", "--with-harness", ".grove/01-impl-survey-k1.md"],
    );
    assert!(ok, "{stderr:?}");
    assert_eq!(stdout, "impl\n");
}

#[test]
fn with_harness_on_an_empty_grove_is_still_the_no_live_leaves_signal() {
    // The finish-cycle iteration: empty stdout is what tells the driver there is
    // no leaf to route, and the flag must not turn that into an error or a
    // stray blank line the driver would parse as a harness.
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", "--with-harness"]);
    assert!(ok, "empty grove must exit 0");
    assert!(stdout.is_empty(), "stdout must be empty, got {stdout:?}");
    assert!(stderr.contains("no live leaves"), "got {stderr:?}");
}

// ── `--json`: the loop driver's retained routing peek ─────────────────────

#[test]
fn json_compatibility_shape_uses_filename_kind_and_no_declared_harness() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-research-a-survey-k7.md", "impl", "codex");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", "--with-harness", "--json"]);

    assert!(ok, "the structured peek failed: {stderr:?}");
    let path = grove
        .join("01-research-a-survey-k7.md")
        .canonicalize()
        .unwrap();
    assert_eq!(
        stdout,
        format!(
            "{{\"path\":\"{}\",\"handle\":\"survey-k7\",\"kind\":\"research-a\",\"harness\":null,\"review\":null}}\n",
            grove::json::escape(&path.display().to_string())
        )
    );
}

#[test]
fn json_represents_an_undeclared_harness_as_null() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf(&grove, "01-impl-build-k1.md", "impl");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", "--with-harness", "--json"]);

    assert!(ok, "the structured peek failed: {stderr:?}");
    assert!(
        stdout.contains("\"handle\":\"build-k1\"")
            && stdout.contains("\"kind\":\"impl\"")
            && stdout.contains("\"harness\":null")
            && stdout.contains("\"review\":null"),
        "the JSON object must carry every launch fact: {stdout:?}"
    );
}

fn build_finished_producer_with_review(repo: &Path, receipt: &str) -> PathBuf {
    let chain = repo.join(".grove/01-build-chain-k10");
    let producer = chain.join("01-build-k1");
    touch(&producer, "BRIEF.md");
    touch_leaf(&producer, "01-DONE-impl-author-k8.md", "impl");
    touch_leaf(
        &producer,
        "02-DONE-integrate-review-impl-finish-k9.md",
        "integrate-review-impl",
    );
    let review = chain.join("02-review-impl-build-review-k2.md");
    fs::write(
        &review,
        format!(
            "# build-review-k2\n\n**Kind:** review-impl\n**Reviews:** build-k1\n**Producer launch:** {receipt}\n"
        ),
    )
    .unwrap();
    touch_leaf(
        &chain,
        "03-integrate-review-impl-build-integrate-k3.md",
        "integrate-review-impl",
    );
    review
}

#[test]
fn json_does_not_read_decomposed_producer_receipts() {
    let tmp = init_repo();
    let review = build_finished_producer_with_review(
        tmp.path(),
        r#"{"producer":"build-k1","session":"finish-k9","generation":"k9","harness":"claude","model":"opus"}"#,
    );

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", "--with-harness", "--json"]);

    assert!(ok, "the structured peek failed: {stderr:?}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        value["path"],
        review.canonicalize().unwrap().display().to_string()
    );
    assert_eq!(value["review"], serde_json::Value::Null);
}

#[test]
fn json_does_not_read_a_legacy_direct_leaf_receipt() {
    let tmp = init_repo();
    let chain = tmp.path().join(".grove/01-build-chain-k4");
    touch_leaf(&chain, "01-DONE-impl-build-k1.md", "impl");
    let review = chain.join("02-review-impl-build-review-k2.md");
    fs::write(
        &review,
        "# build-review-k2\n\n**Kind:** review-impl\n**Reviews:** build-k1\n\
         **Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\",\"model\":\"opus\"}\n",
    )
    .unwrap();

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["kind", "--with-harness", "--json", review.to_str().unwrap()],
    );

    assert!(ok, "legacy direct evidence failed: {stderr}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(value["review"], serde_json::Value::Null);
}

#[test]
fn json_ignores_legacy_node_and_stale_generation_receipts() {
    for receipt in [
        (r#"{"producer":"build-k1","harness":"claude","model":"opus"}"#,),
        (
            r#"{"producer":"build-k1","session":"finish-k9","generation":"k8","harness":"claude","model":"opus"}"#,
        ),
        (
            r#"{"producer":"build-k1","session":"missing-k77","generation":"k9","harness":"claude","model":"opus"}"#,
        ),
    ] {
        let receipt = receipt.0;
        let tmp = init_repo();
        let review = build_finished_producer_with_review(tmp.path(), receipt);

        let (stdout, stderr, ok) = run(
            tmp.path(),
            &["kind", "--with-harness", "--json", review.to_str().unwrap()],
        );

        assert!(ok, "structured peek failed: {stderr:?}");
        let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
        assert_eq!(
            value["review"],
            serde_json::Value::Null,
            "receipt {receipt}"
        );
    }
}

#[test]
fn json_ignores_a_source_session_outside_the_reviewed_producer() {
    let tmp = init_repo();
    let review = build_finished_producer_with_review(
        tmp.path(),
        r#"{"producer":"build-k1","session":"outside-k7","generation":"k9","harness":"claude","model":"opus"}"#,
    );
    touch_leaf(
        &tmp.path().join(".grove/01-build-chain-k10"),
        "04-DONE-impl-outside-k7.md",
        "impl",
    );

    let (stdout, stderr, ok) = run(
        tmp.path(),
        &["kind", "--with-harness", "--json", review.to_str().unwrap()],
    );

    assert!(ok, "source validation failed: {stderr}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(value["review"], serde_json::Value::Null);
}

#[test]
fn json_uses_literal_null_for_an_empty_grove() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch(&grove, "BRIEF.md");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", "--with-harness", "--json"]);

    assert!(ok, "an empty grove is not a structured-peek error");
    assert_eq!(stdout, "null\n");
    assert!(stderr.contains("no live leaves"), "got {stderr:?}");
}

#[test]
fn json_ignores_an_invalid_body_harness() {
    let tmp = init_repo();
    let grove = tmp.path().join(".grove");
    touch_leaf_with_harness(&grove, "01-impl-survey-k1.md", "research", "codx");

    let (stdout, stderr, ok) = run(tmp.path(), &["kind", "--with-harness", "--json"]);

    assert!(ok, "body harness metadata is not parsed: {stderr:?}");
    let value: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(value["harness"], serde_json::Value::Null);
    assert_eq!(value["kind"], "impl");
}

#[test]
fn kind_listed_in_grove_llm_help() {
    let out = Command::cargo_bin("grove-llm")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(s.contains("kind"), "grove-llm --help missing kind: {s}");
}

#[test]
fn grove_help_does_not_list_kind() {
    let out = Command::cargo_bin("grove")
        .unwrap()
        .arg("--help")
        .output()
        .unwrap();
    let s = String::from_utf8_lossy(&out.stdout);
    assert!(
        !s.contains("Print a leaf's task"),
        "grove --help leaked the kind verb from the LLM surface: {s}"
    );
}

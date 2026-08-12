//! `grove-llm methodology` over the real embed — the observable end of making
//! `content/` addressable.
//!
//! The parser's own edge cases live beside it in `src/methodology/parse.rs`,
//! against fixture strings; what is asserted here is what a human or a session
//! can see from outside the binary. Two things need a process rather than a
//! module call: the verb's **output contracts** (byte-exact fetch, a listing
//! that is a grammar rather than a layout, a non-zero exit on an unknown id),
//! and the **environment** it must answer in, which is the one a tree-resolving
//! verb is refused in.

use grove::methodology::{self, Unit};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::{Command, Output};
use tempfile::TempDir;

fn methodology_verb(arguments: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("methodology")
        .args(arguments)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap()
}

fn stdout_of(output: &Output) -> &str {
    assert!(
        output.status.success(),
        "the verb failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    std::str::from_utf8(&output.stdout).expect("unit bytes are UTF-8")
}

/// The claim the whole build gate rests on: every embedded markdown file is
/// read, and every one of them declares at least one unit.
///
/// The gate and this reader share a parser but not a traversal — one walks
/// `content/` on disk, the other walks `include_dir`'s embed — so a file the
/// gate checked and this enumeration missed would be classified by nobody. The
/// corpus is therefore taken from the **directory**, which is the side the
/// runtime reader does not use; `build.rs` re-embeds on any edit under
/// `content/`, so the two are the same tree at test time.
///
/// Stated as file *coverage* rather than as a pinned id set: the complete set of
/// ids is a different claim and lands with the whole-embed checks.
#[test]
fn every_embedded_markdown_file_is_classified() {
    let units = methodology::units().expect("the real embed must parse");

    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let mut on_disk = BTreeSet::new();
    collect_markdown_paths(&content, &content, &mut on_disk);
    let classified: BTreeSet<String> = units.iter().map(|unit| unit.file.clone()).collect();

    assert_eq!(
        classified, on_disk,
        "every markdown file under content/ must declare at least one unit, and \
         the listing must not name a file the embed does not carry"
    );
    assert!(
        !on_disk.is_empty(),
        "an empty corpus would satisfy the equality above without asserting anything"
    );
}

fn collect_markdown_paths(root: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    for entry in fs::read_dir(dir).expect("content/ must be readable") {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_markdown_paths(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).unwrap();
            out.insert(relative.to_string_lossy().into_owned());
        }
    }
}

/// **Marking `content/SKILL.md` must not stop it being a skill.**
///
/// Its leading `---` block is what every harness reads to discover the
/// provisioned skill — `name:` and `description:` are the entry a session sees
/// in its skill list — and it has to keep working unmodified for as long as
/// anything is provisioned. The parser skips the block uninterpreted, so the
/// only way to break this is to edit the file; asserting it on the **extracted**
/// copy is what makes the claim about what a harness actually reads rather than
/// about what the parser did.
#[test]
fn the_skill_frontmatter_survives_marking_and_provisioning() {
    let dest = TempDir::new().unwrap();
    grove::provision::provision_into(dest.path()).unwrap();

    let skill = fs::read_to_string(dest.path().join("SKILL.md")).unwrap();
    let mut lines = skill.lines();
    assert_eq!(
        lines.next(),
        Some("---"),
        "the provisioned SKILL.md must still open with its frontmatter block"
    );
    let front: Vec<&str> = lines.by_ref().take_while(|line| *line != "---").collect();
    assert!(
        front.iter().any(|line| line.starts_with("name:"))
            && front.iter().any(|line| line.starts_with("description:")),
        "the two fields a harness discovers the skill by must survive: {front:?}"
    );
    assert!(
        lines
            .next()
            .is_some_and(|line| line.starts_with("<!-- unit:")),
        "and the body must begin with a unit marker — the block belongs to no unit"
    );
}

/// **The listing is data, so it is asserted as a grammar rather than as a golden
/// string.** A row recovered from prose is a row recovered wrongly, and a golden
/// string would fail on every classification edit while saying nothing about
/// whether the rows are still parseable.
#[test]
fn the_listing_is_five_tab_separated_fields_per_unit() {
    let output = methodology_verb(&[]);
    let listing = stdout_of(&output);
    let units = methodology::units().unwrap();

    let rows: Vec<&str> = listing.lines().collect();
    assert_eq!(
        rows.len(),
        units.len(),
        "one row per unit, and nothing else on stdout: {listing:?}"
    );
    assert!(!rows.is_empty(), "an empty listing asserts nothing below");

    for (row, unit) in rows.iter().zip(&units) {
        let fields: Vec<&str> = row.split('\t').collect();
        assert_eq!(fields.len(), 5, "five fields per row, got {row:?}");
        assert_eq!(fields[0], unit.id);
        assert_eq!(fields[1], unit.class.label());
        assert_eq!(
            fields[2],
            unit.scope
                .as_ref()
                .map(grove::methodology::Scope::render)
                .unwrap_or_else(|| "-".to_string()),
            "a procedural unit has no scope, and the listing may not promise a \
             field it cannot supply for every row"
        );
        assert_eq!(
            fields[3],
            if unit.defers.is_empty() {
                "-".to_string()
            } else {
                unit.defers.join(" ")
            },
            "`-` where a unit defers to nothing — its absence is meaningful"
        );
        assert_eq!(fields[4], unit.file);
        assert!(
            fields.iter().all(|field| !field.is_empty()),
            "no field may be empty; the placeholder is `-`: {row:?}"
        );
    }
}

/// **The round trip is the point**: an inventory a caller cannot feed back into
/// the verb is prose. Asserted for every id at once rather than for a sample,
/// because the failure it guards against — a listing that renders an id
/// differently from the way the fetch accepts it — would show up on exactly one
/// awkward id.
#[test]
fn every_listed_id_round_trips_as_a_fetch_argument() {
    let listing = methodology_verb(&[]);
    let ids: Vec<String> = stdout_of(&listing)
        .lines()
        .map(|row| row.split('\t').next().unwrap().to_string())
        .collect();
    assert!(!ids.is_empty());

    for id in &ids {
        let fetched = methodology_verb(&[id]);
        assert!(
            fetched.status.success(),
            "a listed id must be a fetch argument unchanged; `{id}` was refused: {}",
            String::from_utf8_lossy(&fetched.stderr)
        );
        assert!(!fetched.stdout.is_empty(), "`{id}` fetched no bytes");
    }
}

/// Bytes, in the order given, framed by nothing. The output *is* the
/// methodology, so any decoration would be driver-authored prose arriving
/// through a second door — and the marker line travels with it, which is what
/// makes a slice self-addressing.
#[test]
fn a_fetch_writes_source_bytes_in_the_order_given() {
    let units = methodology::units().unwrap();
    assert!(
        units.len() >= 2,
        "the ordering half of this claim needs two units"
    );
    let first: &Unit = &units[0];
    let last: &Unit = units.last().unwrap();

    let output = methodology_verb(&[&last.id, &first.id]);
    assert_eq!(
        stdout_of(&output),
        format!("{}{}", last.source, first.source),
        "the fetch concatenates the named units' source bytes in argument order, \
         with no separator, header or trailing framing"
    );
    assert!(
        first.source.starts_with("<!-- unit:"),
        "a unit's source carries its own marker line"
    );
}

/// **The framing contract rests on an embed invariant rather than on luck.**
///
/// A fetch concatenates unit sources with nothing between them, so a unit whose
/// bytes did not end in a newline would run its last prose line into the *next*
/// unit's marker — costing that marker its line and the output the
/// self-addressing shape the verb exists to deliver. "Verbatim and framed by
/// nothing" and a useful multi-fetch cannot both hold for such a file, so the
/// build rejects one; what is asserted here is what that buys, over the real
/// embed and through the verb rather than through the parser that enforces it.
#[test]
fn a_multi_unit_fetch_keeps_every_marker_on_its_own_line() {
    let units = methodology::units().unwrap();
    for unit in &units {
        assert!(
            unit.source.ends_with('\n'),
            "`{}` ends mid-line, so anything fetched after it loses its marker",
            unit.id
        );
    }

    let ids: Vec<&str> = units.iter().map(|unit| unit.id.as_str()).collect();
    let output = methodology_verb(&ids);
    let fetched = stdout_of(&output);

    for unit in &units {
        let marker = unit
            .source
            .lines()
            .next()
            .expect("a unit carries its marker");
        assert!(
            fetched.lines().any(|line| line == marker),
            "`{}`'s marker is no longer a whole line of the fetch: {marker:?}",
            unit.id
        );
    }
}

/// A caller's mistake, not a contributor's. A bad `defers=` *inside* the embed
/// fails the build, because the build that produced it can see it; a bad
/// argument is visible only when the call is made, so it is an ordinary runtime
/// error that names the id and points at the listing.
#[test]
fn an_unknown_id_exits_non_zero_naming_it_and_pointing_at_the_listing() {
    let output = methodology_verb(&["no-such-unit"]);

    assert!(!output.status.success(), "an unknown id must exit non-zero");
    assert!(
        output.stdout.is_empty(),
        "nothing may be written before the whole fetch is known to resolve"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("no-such-unit") && stderr.contains("grove-llm methodology"),
        "the message must name the id and direct the caller to the listing: {stderr}"
    );
}

/// A partial fetch must not have already written the units it could serve. The
/// bytes go straight into whatever the caller is reading, so a truncated fetch
/// and a complete one are indistinguishable to it.
#[test]
fn a_fetch_that_cannot_resolve_every_id_writes_nothing() {
    let units = methodology::units().unwrap();
    let known = &units[0].id;

    let output = methodology_verb(&[known, "no-such-unit"]);

    assert!(!output.status.success());
    assert!(
        output.stdout.is_empty(),
        "the resolvable id must not have been served ahead of the failure: {:?}",
        String::from_utf8_lossy(&output.stdout)
    );
}

/// **The environment the successor grove's sessions fetch from.**
///
/// `grove-llm methodology` reads only this binary's own embed and touches no
/// tree, so it is dispatched ahead of the session-epoch guard — the same
/// exemption `--content-hash` already carries. Withholding it would refuse a
/// *lookup* in exactly the environments a session follows a `defers=` id from: a
/// non-repository directory, or one holding a signal path from a dead launch.
/// That is a split-brain inside a single rule.
///
/// **The control is what makes this able to fail.** `GROVE_SIGNAL_FILE` set to a
/// path under a directory with no epoch record is what makes admission resolve a
/// working tree and refuse; the same invocation with the variable unset proves
/// nothing, because admission returns immediately with no ambient context. So
/// `pick` is run in the identical environment and asserted refused: if it ever
/// stops being, this test has stopped testing anything.
#[test]
fn the_verb_answers_from_a_non_repository_directory_under_a_stale_signal_path() {
    let outside = TempDir::new().unwrap();
    let control = outside.path().join("control");
    fs::create_dir_all(&control).unwrap();
    let signal_file = control.join("signal");

    let run = |verb: &str| {
        Command::new(env!("CARGO_BIN_EXE_grove-llm"))
            .arg(verb)
            .current_dir(outside.path())
            .env("GROVE_SIGNAL_FILE", &signal_file)
            .output()
            .unwrap()
    };

    let refused = run("pick");
    assert!(
        !refused.status.success(),
        "control: a tree-resolving verb must be refused here, or the environment \
         below is not the hostile one this test claims to exercise"
    );

    let served = run("methodology");
    assert!(
        served.status.success(),
        "`methodology` must answer where a tree-resolving verb cannot: {}",
        String::from_utf8_lossy(&served.stderr)
    );
    assert!(
        !served.stdout.is_empty(),
        "the listing must be produced, not merely exit zero"
    );
}

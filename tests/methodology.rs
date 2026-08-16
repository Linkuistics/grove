//! The **real embed**, and `grove-llm methodology` over it — the observable end
//! of making `content/` addressable.
//!
//! Both readers' edge cases live beside them in `src/methodology/`, against
//! fixture strings: `parse.rs` for what one file decides, `whole_embed.rs` for
//! what only the assembled set does. What is asserted *here* is what the real
//! corpus is, and what a human or a session can see from outside the binary.
//!
//! Three species live here, and each needs this door rather than a module test:
//!
//! * **Claims about the embed itself** — every markdown file classified, the
//!   complete unit set pinned, and the methodology instructing no `grove-llm`
//!   verb the CLI lacks. Fixtures cannot make these; only the shipped corpus
//!   can.
//! * **The verb's output contracts** — a byte-exact fetch, a listing that is a
//!   grammar rather than a layout, a non-zero exit on an unknown id — which need
//!   a process.
//! * **The environment it must answer in**, which is the one a tree-resolving
//!   verb is refused in.

use clap::CommandFactory;
use grove::leaf::Kind;
use grove::methodology::{self, Class, Unit};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
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
/// Stated as file *coverage* rather than as a pinned id set — that is the
/// separate, stronger claim [`the_embedded_unit_set_is_pinned_complete`] makes,
/// and the two fail differently: this one names a file nobody classified, that
/// one names a unit nobody confirmed.
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

/// **`content/`'s mandate positions are contiguous from 1** — a convention about
/// the shipped corpus, and deliberately not a rule of the grammar.
///
/// The build gate settles *totality* and stops there: the composer sorts by the
/// position rather than indexing on it, so a gap breaks nothing, and a
/// contiguity check in the gate would fail a contributor over a property no
/// consumer reads — and would make *deleting* a file renumber every later one.
/// What density buys is a reader's. Across ten files a position that *is* the
/// order is read at a glance; one that has drifted into arbitrary integers is
/// not (`docs/specs/mandate-delivered-methodology.md`, *The file's mandate order
/// is a comment directive*).
///
/// So it is pinned here, where this file's other claims about the real corpus
/// live, and the split is the point: a fixture in `whole_embed.rs` stays free to
/// use a gap, while the corpus a human reads may not drift into one.
#[test]
fn the_corpus_mandate_positions_are_contiguous_from_one() {
    let units = methodology::units().expect("the real embed must parse");

    // Keyed by position: the whole-embed gate `units()` re-runs has already
    // rejected two files sharing one, so this cannot silently collapse a pair.
    let by_position: BTreeMap<u32, &str> = units
        .iter()
        .map(|unit| (unit.file_order, unit.file.as_str()))
        .collect();

    let actual: Vec<u32> = by_position.keys().copied().collect();
    let expected: Vec<u32> = (1..=by_position.len() as u32).collect();
    let files: Vec<&str> = by_position.values().copied().collect();

    assert_eq!(
        actual,
        expected,
        "content/'s mandate positions are {actual:?} across {} files, in the \
         order {files:?}. The build gate only requires positions to differ, so \
         this is not a malformation — it is the readability convention that a \
         file's position is its place in the reading order. Renumber the \
         `<!-- file: order=<n> -->` directives to 1..={}, or give the convention \
         up in the spec and here together.",
        by_position.len(),
        by_position.len(),
    );
    assert!(
        !actual.is_empty(),
        "an empty corpus would satisfy the equality above without asserting anything"
    );
}

/// **Constraint 7 made checkable**, and the measure is the whole claim.
///
/// "One page of rules" is unmeasurable as written, so the spec fixes a measure a
/// reader can recompute: the lines from a section's heading to the next heading
/// of the same level, blank lines included
/// (`docs/specs/skill-delivered-methodology.md`, *Scenario: the loop section fits
/// a page*). 100 is the **alarm** on a narrative the rewrite estimates at ~80,
/// not a budget the prose was fitted to — a loop section that reaches it has
/// grown a third of a page since the split, which is the point at which growth
/// has become visible.
///
/// It establishes nothing semantic. That `SKILL.md` states conditions and no
/// procedure has no classifier once the unit markers are deleted, and is a review
/// obligation with its own scenario; a line count passing says nothing about it.
#[test]
fn the_loop_section_of_the_skill_fits_a_page() {
    const LIMIT: usize = 100;
    let skill = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("content/SKILL.md"))
        .expect("content/SKILL.md must be readable");

    let measured =
        section_lines(&skill, "## The loop").expect("content/SKILL.md must have a loop section");
    assert!(
        measured <= LIMIT,
        "content/SKILL.md's loop section is {measured} lines against an alarm of \
         {LIMIT}. Move a procedure into the reference file its condition names \
         rather than trimming the alarm."
    );

    // The control. A measure that cannot fail is worth nothing, and this one
    // would silently pass on any heading it failed to find if it returned zero.
    let oversized = format!("## The loop\n{}## Artifacts\n", "x\n".repeat(LIMIT));
    assert_eq!(section_lines(&oversized, "## The loop"), Some(LIMIT + 1));
    assert_eq!(section_lines("# nothing here\n", "## The loop"), None);
}

/// Lines from `heading` to the line before the next heading of the same level,
/// or to end of text. `None` when the heading is absent, so a renamed section
/// fails rather than measuring zero.
fn section_lines(text: &str, heading: &str) -> Option<usize> {
    let level = heading
        .chars()
        .take_while(|character| *character == '#')
        .count();
    let opener = format!("{} ", "#".repeat(level));
    let mut lines = text.lines().skip_while(|line| *line != heading).peekable();
    lines.peek()?;
    Some(
        1 + lines
            .skip(1)
            .take_while(|line| !line.starts_with(&opener))
            .count(),
    )
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

/// Every unit the embed declares, named in full.
///
/// **Pinned complete rather than floored**, which is the same argument
/// [`INSTRUCTED_VERBS`] makes below and for the same reason: a floor bounds only
/// how much a classification may quietly *lose*, and that slack is enough to
/// defeat a universal claim. The cost is a list to maintain, and it buys exactly
/// what it charges for — a new unit fails here until someone names it, which is
/// the moment the classification decision is meant to be confirmed by a human.
///
/// The classification pass is **in progress**, in reviewed batches, and each
/// batch updates this constant deliberately. A `pending-` id is not a
/// classification: it is a coverage placeholder over a region no batch has
/// carved yet, always `class=triggering kinds=*` and never carrying a `defers=`.
/// The pass is finished when no `pending-` id remains here or in `content/`.
///
/// Ids are **prefixed by the file each was first carved from** — `skill-` for
/// `content/SKILL.md`, `task-` for `content/TASK-FORMAT.md`, and so on — which
/// is what makes embed-wide id uniqueness hold without coordination between
/// batches. The prefix is **not** a claim about where a unit lives now: the
/// rewrite into a progressive-disclosure skill moves conditions onto the loop
/// page and procedures into `content/references/`, and an id that travelled
/// keeps its spelling so the composition golden's diff stays a relocation
/// rather than a rename.
const EMBEDDED_UNITS: [&str; 163] = [
    "adr-placement-note",
    "adr-where-adrs-live",
    "adr-why-the-set-stays-minimal",
    "brief-briefs-inherit",
    "brief-durable-content",
    "brief-every-node-carries-one",
    "brief-suggested-shape",
    "brief-the-node-briefing",
    "context-rules",
    "context-single-vs-multi-repos",
    "context-structure",
    "driving-ask-about-the-trade-off",
    "driving-ask-for-pushback",
    "driving-ask-wdyt",
    "driving-cite-framework-decisions-to-the-source",
    "driving-dont-merge-questions",
    "driving-doubting-inside-a-picked-leaf",
    "driving-externalizing-surfaced-work",
    "driving-find-working-increments",
    "driving-how-to-write-a-research-leaf-brief",
    "driving-no-pre-baked-grilling",
    "driving-no-session-summary",
    "driving-prune-reorder-or-file-an-issue",
    "driving-record-decisions-inline",
    "driving-recording-fog",
    "driving-review-chain-habits",
    "driving-reworking-adrs-and-briefs",
    "driving-running-the-vendor-pair",
    "driving-signs-of-a-research-leaf",
    "driving-the-combine-step",
    "driving-the-findings-adopted-bridge",
    "driving-the-fog-or-ticket-test",
    "driving-the-review-chain",
    "driving-turning-a-sweep-into-evidence",
    "driving-what-a-good-child-leaf-looks-like",
    "driving-when-a-leafs-place-is-in-doubt",
    "driving-when-asserting-a-repo-wide-claim",
    "driving-when-code-depends-on-a-framework-version",
    "driving-when-not-to-start-a-grove",
    "driving-when-to-commission-prior-art-research",
    "driving-when-to-invoke-grilling",
    "driving-when-to-retire-research-into-adrs",
    "grilling-agree-the-test-seams",
    "grilling-challenge-the-glossary",
    "grilling-cross-reference-with-code",
    "grilling-discuss-concrete-scenarios",
    "grilling-domain-awareness",
    "grilling-interrogate",
    "grilling-offer-adrs-sparingly",
    "grilling-sharpen-fuzzy-language",
    "grilling-update-context-inline",
    "mandate-framing",
    "skill-adrs-and-specs",
    "skill-artifacts",
    "skill-bare-grove-dispatch",
    "skill-bare-stem-rule",
    "skill-bootstrap",
    "skill-briefs-vs-glossary",
    "skill-chain-gap-asymmetry",
    "skill-commit",
    "skill-commit-boundary-in-git-and-jj",
    "skill-cut-the-next-step",
    "skill-cutting-a-review-leaf",
    "skill-decompose",
    "skill-deriving-the-session-name",
    "skill-directory-tree-and-grow-verbs",
    "skill-dispatch-and-migration",
    "skill-do-not-pick-again",
    "skill-execute",
    "skill-finish",
    "skill-finish-cycle",
    "skill-finish-endings",
    "skill-finish-no-signal-stop",
    "skill-finish-nothing-after",
    "skill-finish-resume",
    "skill-finish-steps",
    "skill-glossary-is-load-bearing",
    "skill-how-the-pick-walks",
    "skill-integration-placement",
    "skill-kind-on-the-tree-verbs",
    "skill-leaf-prune-mechanics",
    "skill-leaf-retire-mechanics",
    "skill-linkuistics-prerequisite",
    "skill-no-exception-to-check",
    "skill-node-close-cascade",
    "skill-node-close-steps",
    "skill-one-configuration",
    "skill-pick",
    "skill-pruning-is-hitl",
    "skill-retire",
    "skill-retirement-touches-one-filename",
    "skill-review-ownership",
    "skill-self-driving-loop",
    "skill-session-name",
    "skill-signal",
    "skill-specs",
    "skill-spine-constraints",
    "skill-starting-a-new-grove",
    "skill-stated-vcs-is-definitive",
    "skill-the-loop-is-stateless",
    "skill-the-record-sets",
    "skill-the-spine-in-full",
    "skill-two-triggers-two-verbs",
    "skill-what-a-grove-is",
    "skill-what-bootstrap-reads",
    "skill-what-each-kind-produces",
    "skill-what-the-configuration-carries",
    "skill-what-the-plugin-carries",
    "skill-what-the-scaffold-creates",
    "skill-which-hop-a-gap-costs",
    "skill-why-a-second-walk-disagrees",
    "skill-why-the-glossary-holds",
    "skill-why-the-handle-outlives-the-path",
    "skill-why-the-stem-is-bare",
    "skill-why-there-is-no-exception",
    "skill-working-tree",
    "spec-behavioural-not-procedural",
    "spec-set-is-current-state",
    "spec-speak-the-projects-language",
    "spec-suggested-shape",
    "spec-synthesise-never-re-interview",
    "spec-test-seams",
    "spec-when-a-spec-is-written",
    "task-bare-stem-reasoning",
    "task-bootstrap-leaf-is-requirements",
    "task-chain-contiguity",
    "task-combine-research",
    "task-declare-the-relationship",
    "task-decompose-inherits-kind",
    "task-deliverable-design",
    "task-deliverable-planning",
    "task-deliverable-requirements",
    "task-deliverable-split-not-a-gate",
    "task-finish-session",
    "task-grammar-is-five-fields",
    "task-hitl-afk",
    "task-in-session-doubt-budget",
    "task-integrate-review-kinds",
    "task-kind-in-the-filename",
    "task-leaf-filename",
    "task-leaf-never-names-a-harness",
    "task-name-reading-is-strict",
    "task-nineteen-kinds",
    "task-no-node-for-a-shape",
    "task-nothing-in-a-body-is-metadata",
    "task-producer-design",
    "task-producer-impl",
    "task-producer-planning",
    "task-producer-prototype",
    "task-producer-requirements",
    "task-research-pair",
    "task-research-write-paths",
    "task-review-chain-mechanics",
    "task-review-kinds",
    "task-suggested-shape",
    "task-the-doubt-budget-table",
    "task-the-kind-table",
    "task-three-design-kinds",
    "task-too-big-is-planning",
    "task-two-shapes",
    "task-vendor-pair-mechanics",
    "task-what-each-field-does",
    "task-work-is-not-a-kind",
];

/// The positive control over the whole embed, failing in **both** directions: a
/// unit lost since the last review, and a unit gained without one.
///
/// It is deliberately a set equality rather than two one-sided assertions.
/// Losing a unit is the silent direction — the gate still passes, the verb still
/// lists, and the only evidence is an id that stopped existing — while gaining
/// one is the direction that must stop and ask.
#[test]
fn the_embedded_unit_set_is_pinned_complete() {
    let declared: BTreeSet<String> = methodology::units()
        .expect("the real embed must parse and check")
        .into_iter()
        .map(|unit| unit.id)
        .collect();

    let pinned: BTreeSet<String> = EMBEDDED_UNITS.iter().map(|id| id.to_string()).collect();
    assert_eq!(
        declared, pinned,
        "the embed's unit set moved. Ids declared but not listed in \
         `EMBEDDED_UNITS` are new classification decisions — confirm each, then \
         add it. Ids listed but no longer declared are either deliberate \
         removals (drop them here) or a unit that has quietly stopped being \
         parsed."
    );
}

/// **The whole-embed gate holds over the corpus that actually ships**, asserted
/// through the door a consumer uses rather than through the build script that
/// also runs it.
///
/// The gate walks `content/` **on disk**; this walks the **linked** embed, and
/// nothing but `build.rs`'s per-file change tracking makes those the same tree.
/// So a green build is not on its own evidence that the binary a session runs
/// carries a consistent embed — this is.
///
/// The claim is made by *calling* the seam rather than by re-deriving it here.
/// A test that recomputed id uniqueness and deferral resolution would be the
/// second implementation the whole design is arranged to avoid, and it would go
/// green on its own bugs.
#[test]
fn the_linked_embed_passes_the_whole_embed_gate() {
    methodology::units().expect(
        "the linked embed must satisfy the whole-embed checks: ids unique across \
         the embed, every `defers=` naming a declared `class=procedural` unit, \
         and every procedural unit reachable from a triggering one",
    );
}

// -- Composition, over the corpus that actually ships -----------------------
//
// The composer's *semantics* are pinned beside it in `src/methodology.rs`,
// against fixtures: selection, byte-exactness, order, and the empty selection.
// What needs this door instead is everything only the real embed can say — that
// the completeness invariant holds over the shipped corpus, what each kind
// actually receives, and whether any of it has grown past the classification
// alarm.

/// **The completeness invariant, over the embed that ships.**
///
/// Two claims, and the second is the one a `contains` sweep alone cannot make.
/// Membership says every unit that should be in this mandate is; the **length**
/// says nothing else is — no driver-authored introduction, no repeated slice,
/// no separator beyond the single blank line between adjacent pairs. Together
/// they pin the projection exactly, without this test re-deriving the selection
/// it is checking in a second implementation that could go green on its own bug.
#[test]
fn every_triggering_unit_reaches_exactly_the_kinds_its_scope_admits() {
    let units = methodology::units().expect("the real embed must parse and check");

    for kind in Kind::ALL {
        let mandate = methodology::compose(&units, kind);
        let mut expected_bytes = 0usize;
        let mut selected = 0usize;

        for unit in &units {
            let admitted = unit.class == Class::Triggering
                && unit.scope.as_ref().is_some_and(|scope| scope.admits(kind));
            assert_eq!(
                mandate.contains(unit.source.as_str()),
                admitted,
                "`{}` ({}, scope {}) is {} the `{}` mandate and should be {}",
                unit.id,
                unit.class.label(),
                unit.scope
                    .as_ref()
                    .map(grove::methodology::Scope::render)
                    .unwrap_or_else(|| "-".to_string()),
                if admitted {
                    "absent from"
                } else {
                    "present in"
                },
                kind.label(),
                if admitted { "present" } else { "absent" },
            );
            if admitted {
                expected_bytes += unit.source.len();
                selected += 1;
            }
        }

        assert!(
            selected > 0,
            "no unit reaches the `{}` mandate, which would satisfy every \
             membership claim above without asserting anything",
            kind.label()
        );
        // Exact given the guard above; `saturating_sub` rather than `- 1` so
        // that a future edit reordering the two fails on the claim rather than
        // on an unsigned overflow that says nothing about the mandate.
        let separators = selected.saturating_sub(1);
        assert_eq!(
            mandate.len(),
            expected_bytes + separators,
            "the `{}` mandate is not exactly its {selected} slices and the \
             {separators} blank lines joining them — something else is in it, or \
             a slice appears twice",
            kind.label(),
        );
    }
}

/// The framing unit leads every mandate, and it does so **without the composer
/// knowing which unit it is** — the file ordering places it, which is the
/// property that keeps composition free of any per-unit special case.
///
/// Pinned by **unit id rather than by filename**, and the rename that has since
/// landed is what that bought: `content/prompts/continue.md` became
/// `content/MANDATE.md` and this assertion did not move, because the unit's
/// position is declared by its file's `<!-- file: order= -->` and not by where the
/// file sits. A rename that had *also* moved the position would have failed here,
/// which is exactly the accident worth catching.
#[test]
fn the_framing_unit_leads_every_composed_mandate() {
    let units = methodology::units().unwrap();
    let framing = units
        .iter()
        .find(|unit| unit.id == "mandate-framing")
        .expect("the framing unit must exist");

    for kind in Kind::ALL {
        assert!(
            methodology::compose(&units, kind).starts_with(framing.source.as_str()),
            "the `{}` mandate does not open with the framing unit; a session \
             would meet a wall of sliced methodology with nothing saying what it is",
            kind.label()
        );
    }
}

/// **The classification alarm, framed honestly as what it is.**
///
/// Not a limit on argv — `ARG_MAX` is 1 MiB and this is about 6% of it. A
/// composed mandate approaching the whole of `SKILL.md` means procedural bodies
/// have been classified as triggering, and that is worth failing on long before
/// anything is at risk (`docs/specs/mandate-delivered-methodology.md`, *Mandate
/// size is a fact, not a design constraint*).
///
/// What is counted is exactly the composer's return: the driver's runtime facts
/// are excluded, because they vary per session with the selected handle and are
/// the two things in a mandate that cannot be misclassified.
#[test]
fn every_kinds_mandate_stays_under_the_classification_alarm() {
    const ALARM: usize = 64 * 1024;
    let units = methodology::units().unwrap();

    for kind in Kind::ALL {
        let size = methodology::compose(&units, kind).len();
        assert!(
            size <= ALARM,
            "the `{}` mandate is {size} bytes, past the {ALARM}-byte alarm. This \
             is a classification signal, not an argv limit: something the size of \
             a whole document has been marked `class=triggering` when its body \
             belongs behind a `defers=`.",
            kind.label()
        );
    }
}

/// **The golden, and it holds unit ids rather than bytes.** That is a
/// deliberate narrowing of "snapshot", and the reason is what a golden is *for*:
/// saying loudly that something moved.
///
/// A byte-level golden of nineteen ~48 kB mandates would move on **every prose
/// edit under `content/`** — which, in this repository, is most commits. A
/// golden regenerated every session is a golden nobody reads, and it would bury
/// the signal it exists to carry. What cannot be seen anywhere else is
/// *composition* drift, and this holds all four of its shapes: a unit gained or
/// lost, a scope widened or narrowed, a file reordered, and a unit moved within
/// its file. Prose edits touch none of them.
///
/// The ids are recovered from the composer's **own output** rather than
/// re-derived, so this cannot go green on a selection bug it shares.
/// [`every_triggering_unit_reaches_exactly_the_kinds_its_scope_admits`] is what
/// says the bytes between those ids are the units' own.
#[test]
fn the_per_kind_composition_matches_its_golden() {
    let units = methodology::units().unwrap();
    let mut rendered = String::from(GOLDEN_HEADER);
    for kind in Kind::ALL {
        for id in composed_ids(&units, kind) {
            writeln!(rendered, "{}\t{id}", kind.label()).unwrap();
        }
    }

    let path = golden_path();
    if std::env::var_os("GROVE_TEST_UPDATE_GOLDENS").is_some() {
        fs::write(&path, &rendered).expect("the golden must be writable");
        return;
    }

    let recorded = fs::read_to_string(&path).unwrap_or_default();
    assert_eq!(
        rendered, recorded,
        "the per-kind composition moved. Read the diff — a row is one unit \
         reaching one kind — and confirm each change is one you meant: a unit \
         added or removed, a `kinds=` scope widened or narrowed, a file's \
         `<!-- file: order= -->` changed, or a unit moved within its file. Then \
         regenerate with `GROVE_TEST_UPDATE_GOLDENS=1 cargo test --test methodology`."
    );
}

const GOLDEN_HEADER: &str = "\
# The ordered unit ids each session kind's composed mandate carries.
#
# One row per (kind, unit), in mandate order: file position, then position
# within the file. Regenerate after confirming a change with
# `GROVE_TEST_UPDATE_GOLDENS=1 cargo test --test methodology`.
";

fn golden_path() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens/composed-mandates.tsv")
}

/// The unit ids of a composed mandate, in the order the composer emitted them.
///
/// Located by **searching the output for each unit's own source bytes** rather
/// than by scanning the text for marker-shaped lines. A slice's body may itself
/// contain a marker-shaped line — the methodology documents its own grammar —
/// and a scanner would report those as units. A source's whole bytes cannot be
/// mistaken that way, and its position in the output is the order asked for.
fn composed_ids(units: &[Unit], kind: Kind) -> Vec<String> {
    let mandate = methodology::compose(units, kind);
    let mut located: Vec<(usize, String)> = units
        .iter()
        .filter_map(|unit| {
            mandate
                .find(unit.source.as_str())
                .map(|at| (at, unit.id.clone()))
        })
        .collect();
    located.sort();
    located.into_iter().map(|(_, id)| id).collect()
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
    assert_eq!(
        (
            lines.next(),
            lines.next().map(|line| line.starts_with("<!-- unit:"))
        ),
        (Some("<!-- file: order=2 -->"), Some(true)),
        "and the body must begin with the file directive and then a unit \
         marker — the block belongs to no unit, and the position it precedes is \
         the one `content/MANDATE.md` keeps at the rename"
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

// -- The embed instructs no verb the CLI lacks ------------------------------
//
// **Relocated from `tests/provision.rs`, and improved by the move.** Both checks
// below are claims about the *embed*, not about provisioning — the architecture
// already names the first as the enforceable half of the build boundary — and
// the grove that retires provisioning deletes their old home. They used to scan
// a **provisioned extraction** of the embed, which meant spending a tempdir and
// a sweep to obtain bytes the binary already carries. They now scan the embed
// itself, through `methodology::markdown_files`.

/// **The methodology and the CLI it instructs ship as one artifact**, and this
/// is the invariant that makes the build boundary safe rather than merely real.
///
/// `content/` is embedded with `include_dir!`, so a session reads the
/// methodology its own `grove` binary was **built** with — never the one
/// committed in some working tree (`docs/ARCHITECTURE.md` §Embedded
/// methodology). The tempting reading of that is a defect: a grove *of grove*
/// commits `content/SKILL.md` and the next session still reads the old one. It
/// is a feature, because **any** skew between the skill and the CLI it
/// instructs is unsafe, in both directions — a newer skill names verbs added
/// since the binary, an older one names verbs *removed* since the skill. This
/// repository holds the second case's ingredients: the `v17.0.0` skill
/// instructs `leaf-add-chain`, removed after that tag, so that skill beside any
/// later binary is broken. So there is no safe direction to drift in, and one
/// embed per build is what keeps the drift at zero.
///
/// No test can see a future build, so "the installed skill is current" is not
/// the statable claim. **This one is**: the embed is internally consistent —
/// every `grove-llm` verb the embedded methodology instructs is a verb the
/// embedded CLI exposes. A binary that ships therefore cannot hand a session a
/// verb it lacks, whatever the working tree it was built from happened to say.
///
/// Mentions come from the corpus rather than from a list, so a verb invented in
/// prose — or one deleted from the CLI while an instruction survives — fails
/// here without anyone having remembered to add it. The reading rule lives on
/// [`scan_instructed_verbs`] and the grain both sides are compared at lives on
/// [`exposed_verbs`]; each is pinned by its own control below, because a claim
/// this universal is only as good as the reader that gathers its corpus.
#[test]
fn the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks() {
    let exposed = exposed_verbs();
    let mut instructed = Vec::new();
    for (path, text) in methodology::markdown_files().expect("the real embed must be readable") {
        scan_instructed_verbs(&path, text, &mut instructed);
    }

    let unknown: Vec<&(String, usize, String)> = instructed
        .iter()
        .filter(|(_, _, verb)| !exposed.contains(verb))
        .collect();
    assert!(
        unknown.is_empty(),
        "the embedded methodology instructs {} verb(s) the embedded `grove-llm` \
         does not expose — a shipped binary would hand a session a call that \
         cannot succeed. Exposed: {exposed:?}. Offending mentions: {unknown:?}",
        unknown.len()
    );

    // Positive control, pinned complete rather than floored. A scan that quietly
    // matches nothing satisfies the assertion above forever, and a *floor* only
    // bounds how much it may quietly lose: `>= 8` against a corpus of eleven let
    // any three verbs disappear unremarked. That slack is enough to defeat the
    // universal claim — move three invocations to a Markdown shape the reader
    // misses, then drop one of those verbs from the CLI, and a shipped binary
    // instructs a call it cannot serve with this test still green. So the
    // complete set is the claim, and losing *any* instructed verb fails here.
    //
    // The cost is a list to maintain, and it buys the moment it charges for: a
    // genuinely new instruction fails until someone names it here, which is
    // exactly when to confirm the CLI really exposes it.
    let distinct: BTreeSet<&str> = instructed
        .iter()
        .map(|(_, _, verb)| verb.as_str())
        .collect();
    assert_eq!(
        distinct,
        BTreeSet::from(INSTRUCTED_VERBS),
        "the embedded methodology's instructed-verb set moved. Verbs found but \
         not listed in `INSTRUCTED_VERBS` are new instructions — check each is a \
         real verb, then add it. Verbs listed but no longer found are either \
         deletions (drop them here) or, if the methodology still instructs them, \
         a reader that has gone blind to a Markdown shape."
    );

    // ...and the classifier must be able to fail. Shown on the exact shape this
    // whole check came from: a removed constructor still instructed in prose.
    let mut synthetic = Vec::new();
    scan_instructed_verbs(
        "REGRESSION.md",
        "Append the whole chain with `grove-llm leaf-add-chain <parent> <stem>`.",
        &mut synthetic,
    );
    assert_eq!(
        synthetic
            .iter()
            .map(|(_, _, verb)| verb.as_str())
            .collect::<Vec<_>>(),
        ["leaf-add-chain"],
        "the scanner must see a removed constructor when one is instructed"
    );
    assert!(
        !exposed.contains("leaf-add-chain"),
        "`leaf-add-chain` is removed; if it is exposed again the positive \
         control above stops being one"
    );

    // The reading rule, pinned on every shape that decides it — the accepted
    // ones as much as the rejected, because the pin above can only be as
    // complete as the reader gathering its corpus. Two of these were live holes
    // rather than hypotheticals: the corpus really does wrap an invocation, and
    // the first version of this scan — line by line — dropped it without a word;
    // and the adjacent-code-span form was invisible for the same fail-open
    // reason until it was pinned here.
    let mut shapes = Vec::new();
    scan_instructed_verbs(
        "SHAPES.md",
        "Prefer `grove-llm\nresolve <ref>` for a durable reference.\n\
         The `grove-llm` binary is agent-facing.\n\
         Run `grove-llm --version` to check.\n\
         Then: grove-llm leaf-retire <leaf-path>\n\
         Run `grove-llm` `leaf-prune <path>` once a human confirms.\n\
         Or `grove-llm  pick`, spaced twice.\n\
         | `grove-llm brief-chain` | walk the ancestors |\n\
         A block ending in grove-llm\n\n\
         paragraph text follows.\n",
        &mut shapes,
    );
    assert_eq!(
        shapes
            .iter()
            .map(|(_, line, verb)| (*line, verb.as_str()))
            .collect::<Vec<_>>(),
        [
            (1, "resolve"),
            (5, "leaf-retire"),
            (6, "leaf-prune"),
            (7, "pick"),
            (8, "brief-chain"),
        ],
        "a wrapped invocation counts and reports the line it starts on, as do a \
         reopened code span, doubled spaces and a table cell; bare `grove-llm` \
         in prose, a flag, and a blank-line gap all do not"
    );
}

/// Every `grove-llm` verb the embedded methodology instructs, named in full so
/// the corpus is pinned rather than merely floored — see the completeness
/// control in [`the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks`].
///
/// **Twelve**, and `methodology` is the one that just joined — on the day
/// `content/MANDATE.md` landed, which is exactly where the eleven-verb note
/// predicted it. The framing unit tells a session that a marker naming
/// `defers=<id>` has a body `grove-llm methodology <id>` will serve, so the verb
/// stopped being a surface nothing in `content/` reaches and became the one the
/// deferral half of the design rests on: withholding a procedural body is only
/// safe while the session can fetch it, and this is the check that the fetch it is
/// pointed at exists in the CLI beside it.
const INSTRUCTED_VERBS: [&str; 12] = [
    "brief-chain",
    "complete",
    "finish-commit",
    "leaf-add",
    "leaf-add-pair",
    "leaf-decompose",
    "leaf-insert",
    "leaf-prune",
    "leaf-retire",
    "methodology",
    "pick",
    "resolve",
];

/// Every `grove-llm` verb the CLI exposes, read from clap's model rather than
/// from `--help` — the question is a fact about the command tree, not about a
/// rendering (the reasoning `tests/help_surfaces.rs` sets out at length).
///
/// **At the grain the instruction scanner reads**, which is the whole
/// correctness of the comparison. [`scan_instructed_verbs`] collects the *first*
/// word after `grove-llm`, so what has to be on this side is the set of first
/// words that begin an invocable command. A recursive walk that flattened every
/// nested subcommand name into one set would answer a different question: expose
/// `grove-llm admin repair` and the flattened set contains `repair`, so the
/// invalid instruction `grove-llm repair` passes as known. Hence top-level names
/// only — and [`the_grove_llm_verb_surface_is_flat`], which fails the day
/// nesting appears, since the scanner must learn full command paths before
/// either side means what this test says it means.
fn exposed_verbs() -> BTreeSet<String> {
    grove::llm_cli::Cli::command()
        .get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .collect()
}

/// `--content-hash` is a **flag**, and this is what that buys: it stays out of
/// the agent grammar. `exposed_verbs` reads subcommands, so a verb would have to
/// be instructed somewhere or fail the completeness control in
/// [`the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks`] — and
/// instructing it would be a lie, because no session ever calls it.
///
/// It followed the two helpers here from `tests/provision.rs`: its subject is
/// the agent verb surface and the instruction scanner, both of which now live
/// beside it, and the identity it is about outlives provisioning anyway.
#[test]
fn the_identity_flag_is_not_part_of_the_agent_verb_surface() {
    assert!(
        !exposed_verbs().contains("content-hash"),
        "`--content-hash` must stay a flag: it is metadata the driver reads from \
         outside any session, not something a session invokes"
    );
    let mut instructed = Vec::new();
    scan_instructed_verbs(
        "FLAG.md",
        "The driver asks `grove-llm --content-hash` for the build's identity.",
        &mut instructed,
    );
    assert!(
        instructed.is_empty(),
        "a flag mention must not read as an instructed verb: {instructed:?}"
    );
}

/// The grain assumption [`exposed_verbs`] rests on, asserted rather than
/// commented: `grove-llm` is a flat list of verbs, so one scanned word after the
/// binary names a whole invocable command.
///
/// Nesting does not break the CLI — it breaks the *comparison*, silently and in
/// the fail-open direction. This is the check that makes the failure loud, and
/// it names the two ways out rather than just the fact.
#[test]
fn the_grove_llm_verb_surface_is_flat() {
    let nested: Vec<String> = grove::llm_cli::Cli::command()
        .get_subcommands()
        .filter(|sub| sub.has_subcommands())
        .map(|sub| sub.get_name().to_string())
        .collect();

    assert!(
        nested.is_empty(),
        "`grove-llm` now nests subcommands under {nested:?}, so a verb name is no \
         longer a whole invocable command. Teach `scan_instructed_verbs` to read \
         full command paths and compare those, or keep the CLI flat — comparing \
         bare names across the two grains lets an invalid instruction pass."
    );
}

type InstructedVerbs = Vec<(String, usize, String)>;

/// `grove-llm`, a separator [`instruction_separator`] accepts, then a lowercase
/// token — wherever it occurs: inline in backticks, in a fenced or indented code
/// block, in a table cell, or in a mermaid node label, all of which the corpus
/// uses.
///
/// **Scanned over the whole file rather than line by line**, because prose wraps:
/// `content/SKILL.md` splits at least one invocation across a line break, and a
/// line-based scan drops it silently — the exact failure mode that makes a
/// removal check worthless.
///
/// It is also why the corpus is a **file** rather than a unit. Units partition a
/// file's body *past its directive*, so scanning them one at a time would put a
/// boundary through the middle of a wrapped invocation — and a marker line always
/// follows such a boundary, so the loss would land in the fail-open direction. A
/// file also carries the two regions no unit covers, which a scan of units would
/// skip; neither can hold an instruction today, and a corpus that cannot lose one
/// tomorrow is the cheaper claim.
///
/// A leading `-` is deliberately not accepted as the token's first character, so
/// a future `grove-llm --version` mention reads as a flag rather than as a verb
/// named `--version`.
fn scan_instructed_verbs(file: &str, text: &str, out: &mut InstructedVerbs) {
    const MARKER: &str = "grove-llm";
    let mut offset = 0;
    while let Some(at) = text[offset..].find(MARKER) {
        let start = offset + at;
        offset = start + MARKER.len();
        let after = &text[offset..];
        let Some(separator) = instruction_separator(after) else {
            continue;
        };
        let rest = &after[separator..];
        if !rest.starts_with(|character: char| character.is_ascii_lowercase()) {
            continue;
        }
        let verb: String = rest
            .chars()
            .take_while(|character| character.is_ascii_lowercase() || *character == '-')
            .collect();
        out.push((
            file.to_string(),
            text[..start].matches('\n').count() + 1,
            verb,
        ));
    }
}

/// The byte length of the run separating a `grove-llm` mention from the token
/// after it, or `None` when the mention does not instruct a call at all.
///
/// Two separators are accepted, and the second is why this is a function rather
/// than a `split`:
///
/// * **whitespace** — `grove-llm leaf-add`, in prose, a fenced or indented code
///   block, a mermaid label or a table cell. One line break may fall inside it,
///   because the corpus wraps invocations; a *blank* line may not, so a bare
///   `grove-llm` ending a code block cannot swallow the first word of the
///   paragraph after it.
/// * **a reopened code span** — `` `grove-llm` `leaf-add` ``, the ordinary
///   Markdown way to write a command whose verb carries its own emphasis. This
///   shape was invisible while the rule demanded whitespace *immediately* after
///   the marker: the closing backtick made the separator empty, and an empty
///   separator is skipped — a valid instruction dropped in the fail-open
///   direction, exactly what a removal check cannot afford.
///
/// The reopening is the discriminator, not decoration. After a closed span,
/// plain prose naming the binary (``the `grove-llm` binary is agent-facing``)
/// must not read as an invocation; admitting it would collect `binary` as a
/// verb. So a span that closed has to open again for the verb, while an
/// unquoted mention may be followed by either.
fn instruction_separator(after: &str) -> Option<usize> {
    let closing = usize::from(after.starts_with('`'));
    let spaced = &after[closing..];
    let spacing: &str = spaced
        .split_terminator(|character: char| !character.is_whitespace())
        .next()
        .unwrap_or("");
    if spacing.is_empty() || spacing.matches('\n').count() > 1 {
        return None;
    }
    let opening = usize::from(spaced[spacing.len()..].starts_with('`'));
    if closing > opening {
        return None;
    }
    Some(closing + spacing.len() + opening)
}

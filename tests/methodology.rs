//! Claims about the **real corpus** — what `content/` is, and what a session
//! reading the provisioned skill will find there.
//!
//! Every claim here needs the shipped corpus rather than a fixture, and that is
//! the whole membership rule for this file. There is nothing else to assert
//! *about* the embed: it is plain markdown, carried whole, with no grammar over
//! it and no verb serving it by the piece. What used to live here — a unit
//! listing's field grammar, a byte-exact fetch, an unknown-id exit code, the
//! environment a lookup had to answer in — went with the machinery those were
//! contracts of.
//!
//! Two of the survivors are the checks the **build gate** used to make, moved to
//! where a claim about the corpus's content belongs: every kind's reference file
//! exists and is reachable from the routing table, and `SKILL.md` stays inside
//! its budget. The other two are the pair that make the build boundary safe —
//! the methodology instructs no `grove-llm` verb the embedded CLI lacks, and the
//! flat verb surface that makes that comparison mean what it says.

use clap::CommandFactory;
use grove::methodology;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// **The linked embed and `content/` on disk are the same tree, byte for byte.**
///
/// `include_dir!` reads `content/` at compile time and, on stable Rust, does not
/// register those reads with Cargo's change tracker — `build.rs`'s per-file
/// `rerun-if-changed` walk is the only thing that makes an edit, an addition or
/// a removal reach the binary. That walk is now the **whole** of the build
/// script, so nothing else would notice it going wrong; this is what does.
///
/// The two sides are gathered by different traversals on purpose. `on_disk`
/// walks the directory with `fs::read_dir`; `embedded` walks `include_dir`'s
/// linked tree. A stale embed shows up as a difference between them, which a
/// single traversal compared against itself could never produce.
///
/// **The comparison is on contents, not on paths**, and the distinction is the
/// whole check rather than a refinement of it. The regression this test names
/// itself the sole guard against — the per-file walk missing an *edit* — changes
/// no filename at all, so a path-set equality reports success on exactly the
/// case it exists to catch: `cargo test` can run a stale test binary, compare
/// the same ten-odd paths against themselves, and pass while the binary still
/// embeds old prose and the installed skill lags `content/`. Adds and removes
/// are the easy half; they move the directory mtime, which even a bare
/// `rerun-if-changed=content` would have caught.
///
/// Disagreements are reported per path by [`disagreements`] rather than by
/// `assert_eq!` on the two maps, because the corpus is tens of kilobytes and an
/// equality failure would print all of it twice to say one file moved.
#[test]
fn the_linked_embed_carries_every_markdown_file_on_disk() {
    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let mut on_disk = BTreeMap::new();
    collect_markdown_files(&content, &content, &mut on_disk);

    let embedded: BTreeMap<String, String> = methodology::markdown_files()
        .expect("the real embed must be readable")
        .into_iter()
        .map(|(path, text)| (path, text.to_owned()))
        .collect();

    assert_eq!(
        disagreements(&embedded, &on_disk),
        Vec::<String>::new(),
        "the linked embed and content/ on disk disagree. Differing contents at \
         the same path mean `build.rs`'s change tracking missed an edit and the \
         binary is carrying a stale corpus, as does a file on disk and not in \
         the embed; a file in the embed and not on disk means a deletion has \
         not been rebuilt. Rebuild, and if that clears it, `build.rs`'s walk is \
         what to look at."
    );
    assert!(
        !on_disk.is_empty(),
        "an empty corpus would satisfy the comparison above without asserting anything"
    );

    // The controls. A comparison that cannot fail is worth nothing, and the
    // failure this one has to be able to state is precisely the one a path-set
    // equality could not: same path, different bytes.
    let (edited_path, edited_text) = embedded
        .iter()
        .next()
        .map(|(path, text)| (path.clone(), format!("{text}\nan edit the walk missed\n")))
        .expect("the real embed is not empty");
    let stale: BTreeMap<String, String> = embedded
        .iter()
        .map(|(path, text)| (path.clone(), text.clone()))
        .chain([(edited_path.clone(), edited_text)])
        .collect();
    let reported = disagreements(&stale, &on_disk);
    assert_eq!(
        reported.len(),
        1,
        "a single same-path content mismatch must be reported once, naming that \
         file and nothing else: {reported:?}"
    );
    assert!(
        reported[0].starts_with(&edited_path),
        "the report must name the file whose contents moved: {reported:?}"
    );

    // ...and the two path-shaped failures the old equality did catch, kept.
    let mut removed = embedded.clone();
    removed.remove(&edited_path);
    assert_eq!(disagreements(&removed, &on_disk).len(), 1);
    assert_eq!(disagreements(&embedded, &removed).len(), 1);
}

/// Every path where the two sides disagree, one line each, saying which side it
/// is on and what that means.
///
/// A path in both with differing contents is the stale-embed case; a path in
/// only one is an add or a delete the build did not carry. Reported as text
/// rather than as a diff: the question is *which file*, and the answer to a
/// failure here is always to rebuild and then look at `build.rs`.
fn disagreements(
    embedded: &BTreeMap<String, String>,
    on_disk: &BTreeMap<String, String>,
) -> Vec<String> {
    let mut out = Vec::new();
    for (path, text) in embedded {
        match on_disk.get(path) {
            Some(disk) if disk == text => {}
            Some(disk) => out.push(format!(
                "{path}: contents differ — the embed carries {} bytes, disk has {} \
                 (a stale embed: the edit did not reach the binary)",
                text.len(),
                disk.len()
            )),
            None => out.push(format!(
                "{path}: in the embed, not on disk (a deletion that has not been rebuilt)"
            )),
        }
    }
    for path in on_disk.keys() {
        if !embedded.contains_key(path) {
            out.push(format!(
                "{path}: on disk, not in the embed (`build.rs`'s change tracking missed it)"
            ));
        }
    }
    out
}

/// **The routing table is the first screen, and every row it prints resolves.**
///
/// The opening routes rather than introduces: a session that arrived by
/// description match rather than by a mandate still lands in its kind's
/// reference file (`docs/ARCHITECTURE.md`, *The corpus's shape*). A row naming a
/// file that is not there sends exactly that session nowhere, and prose cannot
/// notice.
///
/// Ten rows for nineteen kinds, because the corpus already treats each family as
/// one unit — the five `review-*` kinds share a file, as do the five
/// `integrate-review-*` and the two research producers. The **map** the driver
/// will own is an exhaustive `match` over the kind enum and is checked where it
/// is written; this checks the table a human reads.
///
/// **The pairing is what is asserted, not the path set.** Collecting the
/// right-hand paths alone would discard the kind column and collapse duplicate
/// rows, so a table with a kind missing, a kind twice, an extra row, or a path
/// against the wrong kind would still pass while sending a session to the wrong
/// discipline — which is the entire failure this test exists to catch. The rows
/// are therefore compared as an ordered sequence of `(kind label, path)`.
#[test]
fn the_skills_routing_table_names_a_reference_file_that_exists() {
    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let skill =
        fs::read_to_string(content.join("SKILL.md")).expect("content/SKILL.md must be readable");

    // A row is `| <kind label> | `references/<file>` |`; the header and its
    // `|---|---|` rule carry no backticked path and drop out here.
    let routed: Vec<(String, String)> = skill
        .lines()
        .skip_while(|line| !line.starts_with("## Read your kind's reference file first"))
        .take_while(|line| !line.starts_with("## The spine"))
        .filter(|line| line.starts_with('|'))
        .filter_map(|line| {
            let mut cells = line.trim_matches('|').split('|').map(str::trim);
            let kind = cells.next()?;
            let path = cells
                .next()?
                .strip_prefix("`references/")?
                .strip_suffix('`')?;
            Some((kind.to_owned(), path.to_owned()))
        })
        .collect();

    let expected: Vec<(String, String)> = [
        ("`requirements`", "requirements.md"),
        ("`design`", "design.md"),
        ("`planning`", "planning.md"),
        ("`prototype`", "prototype.md"),
        ("`impl`", "impl.md"),
        ("the five `review-*` kinds", "review.md"),
        ("the five `integrate-review-*` kinds", "integrate-review.md"),
        ("`research-a`, `research-b`", "research.md"),
        ("`combine-research`", "combine-research.md"),
        ("`finish`", "finish.md"),
    ]
    .into_iter()
    .map(|(kind, path)| (kind.to_owned(), path.to_owned()))
    .collect();

    assert_eq!(
        routed, expected,
        "content/SKILL.md's routing table must carry exactly these ten \
         kind→reference-file rows, in this order — a session that arrived \
         without a mandate reads the left column to find the right one"
    );
    for (kind, path) in &routed {
        assert!(
            content.join("references").join(path).is_file(),
            "content/SKILL.md routes {kind} to references/{path}, which does not exist"
        );
    }
}

/// **The house progressive-disclosure ceiling, on the body rather than the file.**
///
/// The measure is `SKILL.md`'s *body* — the frontmatter is a routing header a
/// harness reads, not guidance a session reads, and counting it would let a
/// description rewrite eat into the ceiling (`docs/ARCHITECTURE.md`, *The
/// corpus's shape*).
///
/// 500 is a **ceiling**, not the rewrite's target: the corpus rewrite estimates
/// ~200 lines, so a body approaching this number has re-grown procedure that the
/// split moved out. Like the loop-section alarm below, it establishes nothing
/// semantic — *no procedure in `SKILL.md`* has no classifier now that the unit
/// markers are deleted, and is a review obligation. **This passing is not
/// evidence for it.**
///
/// It is also, with the routing check above, what the deleted **build gate**
/// bought: a corpus malformation caught by the suite instead of by `cargo
/// build`. That is a weaker instrument by design — a contributor sees it at
/// `cargo test` rather than at compile — and it is the right one for a claim
/// about prose, which was never what a grammar gate could decide.
#[test]
fn the_skill_body_fits_the_progressive_disclosure_ceiling() {
    const LIMIT: usize = 500;
    let skill = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("content/SKILL.md"))
        .expect("content/SKILL.md must be readable");

    let measured = body_lines(&skill).expect("content/SKILL.md must have YAML frontmatter");
    assert!(
        measured <= LIMIT,
        "content/SKILL.md's body is {measured} lines against a ceiling of {LIMIT}. \
         Move a procedure into the reference file its condition names rather than \
         raising the ceiling."
    );

    // The control. A measure that cannot fail is worth nothing, and one that
    // silently returned zero on an unrecognised header would pass forever.
    let oversized = format!("---\nname: x\n---\n{}", "x\n".repeat(LIMIT));
    assert_eq!(body_lines(&oversized), Some(LIMIT));
    assert_eq!(body_lines("no frontmatter here\n"), None);
}

/// **The condition register's budget, asserted as four exact claims.**
///
/// `content/SKILL.md` is on every session's static loaded path, so it is the
/// single largest lever on what a session pays to read
/// (`docs/specs/corpus-rule-ownership.md`, *What `SKILL.md` can hold,
/// arithmetically*). The design states the budget as a **ceiling of 900 words**
/// plus three exact counts, and deliberately gives it **no floor**: a floor could
/// only be derived from a measurement nobody had taken, and it would be
/// discharged by writing words to reach it.
///
/// The exact counts are what a word count cannot do. A silently dropped
/// `trigger` sentence is a rule that stops being reachable from this file — the
/// failure the whole ownership design exists to prevent — and it makes the file
/// *smaller*, so a ceiling alone reports success. `=26` rather than `<=26` for
/// the same reason, and the eight `own` rows are checked by name so a failure
/// says **which** row left rather than that the total moved.
///
/// **A count is not an identity, and the count alone was the hole.** Twenty-six
/// short `When` bullets satisfy `=26` whether or not they are the twenty-six
/// canonical situations, so a rewrite that replaced one sentence with a second
/// copy of another stayed green while its rule became unreachable — which is the
/// same failure as deleting the sentence, minus the arithmetic that would have
/// caught it. So the set is matched **row by row** against
/// [`CANONICAL_TRIGGERS`]: every row claims exactly one sentence and every
/// sentence is claimed by exactly one row. The rows identify a sentence by its
/// owner path plus a situation word with alternatives, never by its wording, so
/// the design's licence to reword within the trigger grammar survives.
///
/// The rewrite that landed this measured **796 words**, 26 triggers, longest 16.
/// The spec projected 500–620 on the drafted parts; the difference is the intro,
/// the headings, the routing table's cells and the eighth `own` body, none of
/// which had been written when that projection was made. It is recorded here as a
/// measurement, not as a target — the ceiling is what binds.
#[test]
fn the_skill_holds_its_condition_register_budget() {
    const WORD_CEILING: usize = 900;
    const TRIGGERS: usize = 26;
    const TRIGGER_WORDS: usize = 25;

    let skill = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("content/SKILL.md"))
        .expect("content/SKILL.md must be readable");
    let body = body_text(&skill).expect("content/SKILL.md must have YAML frontmatter");

    let words = body.split_whitespace().count();
    assert!(
        words <= WORD_CEILING,
        "content/SKILL.md's body is {words} words against a ceiling of \
         {WORD_CEILING}. Route a procedure to the reference file its condition \
         names rather than raising the ceiling."
    );

    let triggers = trigger_sentences(body);
    assert_eq!(
        triggers.len(),
        TRIGGERS,
        "content/SKILL.md must carry exactly the {TRIGGERS} canonical trigger \
         sentences of docs/specs/corpus-rule-ownership.md, *The trigger \
         sentences* — they are this file's whole edge list, so a missing one \
         leaves its rule unreachable and an extra one is a rule stated twice. \
         Found: {triggers:#?}"
    );
    let overlong: Vec<(usize, usize)> = triggers
        .iter()
        .enumerate()
        .map(|(index, sentence)| (index + 1, sentence.split_whitespace().count()))
        .filter(|(_, count)| *count > TRIGGER_WORDS)
        .collect();
    assert!(
        overlong.is_empty(),
        "a trigger is one sentence of at most {TRIGGER_WORDS} words — the \
         situation, a single-clause obligation, and the owner file's path. These \
         (1-based index, words) have outgrown that and are becoming procedures: \
         {overlong:?}"
    );

    let audit = audit_triggers(&triggers);
    assert!(
        audit.is_clean(),
        "content/SKILL.md's trigger set must be exactly the 26 canonical rows of \
         docs/specs/corpus-rule-ownership.md, *The trigger sentences* — one \
         sentence per situation, each naming its owner's path. A row with no \
         sentence is a rule no session can reach; a row claiming two is a \
         situation stated twice; a sentence no row claims is an edge the design \
         does not have. {audit}"
    );

    let absent: Vec<String> = OWN_ROWS
        .iter()
        .flat_map(|row| {
            unmet_beats(body, row)
                .into_iter()
                .map(|beat| format!("{}: {beat}", row.rule))
        })
        .collect();
    assert!(
        absent.is_empty(),
        "content/SKILL.md is the canonical source for eight rules — the ones \
         whose whole content is their trigger, so no procedure remains to defer. \
         Each of these has lost wording or structure it is supposed to carry. \
         Any listed alternative satisfies its beat, so reword freely; what is \
         not free is dropping a beat, or — where the row is ordered — moving \
         one: {absent:#?}"
    );

    // The controls. Each measure has to be able to fail, and the three that read
    // structure have to fail on the shape they would otherwise lose silently.
    let synthetic = "- When a wrapped bullet runs on, it\n  still counts once.\n\
                     - Retire the leaf as `references/retire.md` directs.\n\
                     \nWhen a paragraph opens this way it is not a bullet.\n";
    let control = trigger_sentences(synthetic);
    assert_eq!(
        control.len(),
        1,
        "only a list item opening `When` is a trigger, and a wrapped one is one \
         sentence: {control:?}"
    );
    assert_eq!(control[0].split_whitespace().count(), 10);

    // Substitution, not deletion: sentence 18 is replaced by a second copy of
    // sentence 19, so the count stays 26 and the file stays under the ceiling.
    // This is the shape the old count-only assertion could not see.
    let substituted: Vec<String> = triggers
        .iter()
        .map(|sentence| {
            if sentence.contains("durable artifact") {
                triggers
                    .iter()
                    .find(|other| other.contains("`linkuistics` plugin"))
                    .expect("sentence 19 is in the corpus")
                    .clone()
            } else {
                sentence.clone()
            }
        })
        .collect();
    assert_eq!(substituted.len(), TRIGGERS);
    let substituted = audit_triggers(&substituted);
    assert_eq!(substituted.missing, vec!["durable-artifact-set"]);
    assert_eq!(substituted.duplicated, vec!["plugin-prerequisite"]);

    // Reordering Bootstrap's reads keeps every phrase in the body and breaks the
    // rule, which is exactly what an unordered presence check cannot report.
    let reordered = body.replace(
        "the glossary, the ADRs the briefs cite,\nthe `BRIEF.md` chain root→leaf, and the task file.",
        "the task file, the glossary, the ADRs\nthe briefs cite, and the `BRIEF.md` chain root→leaf.",
    );
    assert_ne!(
        reordered, body,
        "the Bootstrap section must be worded as the control assumes"
    );
    let bootstrap = OWN_ROWS
        .iter()
        .find(|row| row.rule == "bootstrap-order")
        .expect("bootstrap-order is an own row");
    assert_eq!(unmet_beats(&reordered, bootstrap).len(), 1);

    // Dropping the routing row's before-work clause leaves the table, the
    // instruction over it, and the no-selection clause all intact.
    let untimed = body.replace(", before you act", "");
    assert_ne!(
        untimed, body,
        "the routing instruction must carry a timing clause"
    );
    let routing = OWN_ROWS
        .iter()
        .find(|row| row.rule == "kind-routing-table")
        .expect("kind-routing-table is an own row");
    assert_eq!(unmet_beats(&untimed, routing).len(), 1);

    assert!(body_text("no frontmatter here\n").is_none());
}

/// The 26 canonical trigger rows of `docs/specs/corpus-rule-ownership.md`,
/// *The trigger sentences* — the whole edge list out of `content/SKILL.md`.
///
/// A row identifies its sentence by two things a rewording cannot move: the
/// **owner path**, which is the edge itself, and one **situation word** from
/// `situation`, any of which will do. Nothing here pins a sentence's wording,
/// because the design licenses `SKILL.md` to reword within the trigger grammar;
/// what it forbids is changing the set, and that is what a row failing to claim
/// exactly one sentence reports.
///
/// Five sentences carry two rows each. Both rows name the same owner and the
/// same situation, so each claims that one sentence and the bijection holds at
/// the level of sentences rather than rows.
const CANONICAL_TRIGGERS: [TriggerRow; 31] = [
    row("pick-walk-order", "references/driver.md", &["launched"]),
    row("one-configuration", "references/driver.md", &["launched"]),
    row(
        "stale-launch-stops",
        "references/bootstrap.md",
        &["terminal"],
    ),
    row("review-budget", "references/execute.md", &["reviewer"]),
    row(
        "verify-repo-claims-with-controls",
        "references/execute.md",
        &["repo-wide"],
    ),
    row(
        "decisions-land-as-they-settle",
        "references/execute.md",
        &["decision"],
    ),
    row(
        "escalation-names-the-tradeoff",
        "references/execute.md",
        &["human"],
    ),
    row(
        "externalize-by-default",
        "references/decompose.md",
        &["does not serve"],
    ),
    row(
        "bigger-than-brief-decomposes",
        "references/decompose.md",
        &["bigger"],
    ),
    row(
        "integration-placement",
        "references/decompose.md",
        &["integration"],
    ),
    row("fog-or-ticket", "references/decompose.md", &["foresee"]),
    row(
        "prior-art-research-is-its-own-leaf",
        "references/decompose.md",
        &["lessons"],
    ),
    row(
        "review-chain-when-load-bearing",
        "references/decompose.md",
        &["may need review"],
    ),
    row(
        "vendor-pair-when-load-bearing",
        "references/decompose.md",
        &["surveys"],
    ),
    row(
        "retire-before-commit",
        "references/retire.md",
        &["work is done"],
    ),
    row(
        "retirement-is-filename-only",
        "references/retire.md",
        &["work is done"],
    ),
    row(
        "pruning-is-hitl",
        "references/retire.md",
        &["decided against"],
    ),
    row(
        "triage-picks-the-verb",
        "references/retire.md",
        &["in doubt"],
    ),
    row("no-fourth-status", "references/retire.md", &["in doubt"]),
    row("node-close-is-implicit", "references/retire.md", &["node"]),
    row("cascade-is-silent", "references/retire.md", &["node"]),
    row(
        "finish-is-the-drivers-to-discover",
        "references/retire.md",
        &["finishing"],
    ),
    row("one-focused-commit", "references/commit.md", &["retired"]),
    row("name-by-handle", "references/commit.md", &["retired"]),
    row(
        "durable-artifact-set",
        "references/grove.md",
        &["durable artifact"],
    ),
    row("plugin-prerequisite", "references/grove.md", &["plugin"]),
    row("adr-when-to-write", "ADR-FORMAT.md", &["considering"]),
    row(
        "adr-set-is-minimum-coherent",
        "ADR-FORMAT.md",
        &["recorded decision"],
    ),
    row(
        "spec-at-an-agreement-point",
        "SPEC-FORMAT.md",
        &["agreement point"],
    ),
    row(
        "spec-set-is-current-state",
        "SPEC-FORMAT.md",
        &["changing a spec"],
    ),
    row(
        "glossary-is-the-forcing-function",
        "CONTEXT-FORMAT.md",
        &["term"],
    ),
];

/// One canonical trigger row. `situation` is an any-of list, so a rewording has
/// somewhere to land without the row losing its grip on the sentence.
struct TriggerRow {
    rule: &'static str,
    owner: &'static str,
    situation: &'static [&'static str],
}

const fn row(
    rule: &'static str,
    owner: &'static str,
    situation: &'static [&'static str],
) -> TriggerRow {
    TriggerRow {
        rule,
        owner,
        situation,
    }
}

/// What the canonical set and the corpus's sentences disagree about.
#[derive(Debug, Default)]
struct TriggerAudit {
    /// Rows no sentence claims — each one a rule `SKILL.md` no longer reaches.
    missing: Vec<&'static str>,
    /// Rows claiming more than one sentence — a situation stated twice.
    duplicated: Vec<&'static str>,
    /// Sentences no row claims — an edge the design does not have.
    unrecognised: Vec<String>,
}

impl TriggerAudit {
    fn is_clean(&self) -> bool {
        self.missing.is_empty() && self.duplicated.is_empty() && self.unrecognised.is_empty()
    }
}

impl std::fmt::Display for TriggerAudit {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "rules with no trigger sentence: {:?}; rules whose situation is \
             stated twice: {:?}; sentences matching no canonical row: {:#?}",
            self.missing, self.duplicated, self.unrecognised
        )
    }
}

/// Match every canonical row against `triggers`, both ways.
///
/// Both directions matter and they fail on different edits. Row → sentence
/// catches a rule that lost its edge; sentence → row catches a bullet that
/// gained one the inventory does not license. Substituting one canonical
/// sentence for another trips both.
fn audit_triggers(triggers: &[String]) -> TriggerAudit {
    let mut audit = TriggerAudit::default();
    let mut claimed = vec![false; triggers.len()];
    for row in &CANONICAL_TRIGGERS {
        let hits: Vec<usize> = triggers
            .iter()
            .enumerate()
            .filter(|(_, sentence)| {
                sentence.contains(row.owner)
                    && row.situation.iter().any(|word| sentence.contains(word))
            })
            .map(|(index, _)| index)
            .collect();
        match hits.len() {
            0 => audit.missing.push(row.rule),
            1 => claimed[hits[0]] = true,
            _ => {
                audit.duplicated.push(row.rule);
                for hit in hits {
                    claimed[hit] = true;
                }
            }
        }
    }
    audit.unrecognised = triggers
        .iter()
        .zip(&claimed)
        .filter(|(_, claimed)| !**claimed)
        .map(|(sentence, _)| sentence.clone())
        .collect();
    audit
}

/// The eight rules `content/SKILL.md` **owns** — `Occasion = orientation`, whole
/// content is the trigger, nothing left to defer — each with wording that has to
/// survive a rewrite (`docs/specs/corpus-rule-ownership.md`, *the condition
/// register*).
///
/// A row is a list of **beats**, and each beat is an any-of list of wordings:
/// the claim is that the beat is *there*, not that it is said to the letter, so
/// a rewrite is free to re-say it and an alternative can be added when it does.
/// What a rewrite is not free to do is drop a beat.
///
/// **Two rows are ordered, and that is the finding this shape answers.** A
/// flattened, unordered presence check reads the body as a bag of phrases, so it
/// passes on a rewrite that keeps every word and destroys the rule: Bootstrap
/// reading the task file before the glossary and the cited ADRs — which is a
/// session taking unqualified task prose as its mandate — or the spine's
/// constraints renumbered, which silently repoints every "constraint 4" citation
/// in the corpus. Neither is a wording change, so neither belongs to the licence
/// to reword; both are the structure that makes the row a rule.
///
/// `kind-routing-table` is unordered and gains its two missing clauses instead.
/// The table alone is a listing; what makes it a rule is *before you act* (a
/// session that starts work first has applied no kind discipline to it) and
/// *select nothing here* (a session that chooses re-derives a fact the driver
/// already resolved). The rows of the table itself are pinned by
/// [`the_skills_routing_table_names_a_reference_file_that_exists`].
const OWN_ROWS: [OwnRow; 8] = [
    OwnRow {
        rule: "kind-routing-table",
        ordered: false,
        beats: &[
            &[
                "open the one row your kind names",
                "read the one row your kind names",
            ],
            &[
                "before you act",
                "before you begin",
                "before doing anything",
            ],
            &[
                "select nothing here",
                "nothing for you to select",
                "you make no selection",
            ],
        ],
    },
    OwnRow {
        rule: "spine-seven-constraints",
        ordered: true,
        beats: &[
            &["Artifacts, not state."],
            &["Read, don't run."],
            &["Suggested shape, not enforced schema."],
            &["Lazy and optional."],
            &["just-in-time, not few"],
            &["grove guides, it does not gate."],
            &["Walk-away-able."],
            &["One page of rules."],
        ],
    },
    OwnRow {
        rule: "one-task-is-one-session",
        ordered: false,
        beats: &[&["One task is one session."]],
    },
    OwnRow {
        rule: "bootstrap-order",
        ordered: true,
        beats: &[
            &["Resolve the mandated handle", "Resolve the handle"],
            &["the glossary"],
            &["the ADRs the briefs cite", "the cited ADRs"],
            &["`BRIEF.md` chain root→leaf", "brief chain root→leaf"],
            &["the task file"],
            &["Nothing else by reflex", "nothing else by reflex"],
        ],
    },
    OwnRow {
        rule: "mandate-is-authoritative",
        ordered: false,
        beats: &[&["mandate is authoritative"]],
    },
    OwnRow {
        rule: "no-second-pick",
        ordered: false,
        beats: &[&["Do not pick again"], &["the mandate wins"]],
    },
    OwnRow {
        rule: "stated-vcs-is-definitive",
        ordered: false,
        beats: &[
            &["stated VCS is definitive"],
            &["Do not re-derive"],
            &["disregard a harness banner"],
        ],
    },
    OwnRow {
        rule: "hitl-afk-mark-predicts",
        ordered: false,
        beats: &[
            &["mark predicts, it does not permit"],
            &["any kind may stop and ask"],
        ],
    },
];

/// One `own` row: its rule, whether its beats have to appear in the body in the
/// order given, and the beats.
struct OwnRow {
    rule: &'static str,
    ordered: bool,
    beats: &'static [&'static [&'static str]],
}

/// Which of `row`'s beats `text` does not satisfy, one report per beat.
///
/// Matching is on whitespace-collapsed text, because the corpus wraps and a
/// needle that missed a wrapped match would report a present beat as absent.
///
/// For an ordered row the search carries a cursor: each beat must occur *after*
/// the match that satisfied the previous one, so the assertion is that some
/// in-order occurrence exists. That is the permissive reading on purpose — a
/// phrase repeated elsewhere in the file does not fail the row — while the edit
/// it exists to catch, a list whose items have been moved, has one occurrence
/// each and fails.
fn unmet_beats(text: &str, row: &OwnRow) -> Vec<String> {
    let flattened = text.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut unmet = Vec::new();
    let mut cursor = 0;
    for (index, beat) in row.beats.iter().enumerate() {
        let found = beat
            .iter()
            .filter_map(|wording| {
                flattened[cursor..]
                    .find(wording)
                    .map(|at| cursor + at + wording.len())
            })
            .min();
        match found {
            Some(end) if row.ordered => cursor = end,
            Some(_) => {}
            None if row.ordered => unmet.push(format!(
                "beat {} is absent, or is out of order — none of {beat:?} occurs \
                 after the beat before it",
                index + 1
            )),
            None => unmet.push(format!("beat {}: none of {beat:?}", index + 1)),
        }
    }
    unmet
}

/// Every `trigger` sentence in the body: a list item opening `When`, with its
/// continuation lines rejoined.
///
/// **Rejoining is the whole reader**, and it fails in the direction that matters
/// if it is left out: the sentences wrap at the file's column, so a line-based
/// count would see the right number of items while measuring each at a fraction
/// of its length, and the per-sentence ceiling would pass on a sentence that had
/// grown into a paragraph.
fn trigger_sentences(body: &str) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut open = false;
    for line in body.lines() {
        if let Some(item) = line.strip_prefix("- ") {
            open = item.starts_with("When ");
            if open {
                out.push(item.trim().to_owned());
            }
        } else if open && line.starts_with("  ") && !line.trim().is_empty() {
            let sentence = out.last_mut().expect("an open item was pushed");
            sentence.push(' ');
            sentence.push_str(line.trim());
        } else {
            open = false;
        }
    }
    out
}

/// The body of a frontmatter document — everything after the closing `---`.
/// `None` when the document does not open with frontmatter, so a stripped header
/// fails rather than measuring the whole file as prose.
fn body_text(text: &str) -> Option<&str> {
    let rest = text.strip_prefix("---\n")?;
    let end = rest.find("\n---\n")?;
    Some(&rest[end + "\n---\n".len()..])
}

/// Lines after the closing `---` of YAML frontmatter. `None` when the document
/// does not open with frontmatter, so a stripped header fails rather than
/// measuring the whole file.
fn body_lines(text: &str) -> Option<usize> {
    let mut lines = text.lines();
    (lines.next()? == "---").then_some(())?;
    let mut lines = lines.skip_while(|line| *line != "---");
    lines.next()?;
    Some(lines.count())
}

/// **Constraint 7 made checkable**, and the measure is the whole claim.
///
/// "One page of rules" is unmeasurable as written, so the design fixes a measure
/// a reader can recompute: the lines from a section's heading to the next heading
/// of the same level, blank lines included (`docs/ARCHITECTURE.md`, *The corpus's
/// shape*). 100 is the **alarm** on a narrative the rewrite estimates at ~80,
/// not a budget the prose was fitted to — a loop section that reaches it has
/// grown a third of a page since the split, which is the point at which growth
/// has become visible.
///
/// It establishes nothing semantic. That `SKILL.md` states conditions and no
/// procedure has no classifier now that the unit markers are deleted, and is a
/// review obligation; a line count passing says nothing about it.
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

/// Every Markdown file under `dir`, keyed by its path relative to `root` and
/// carrying its bytes — the disk side of the embed comparison, gathered by
/// `fs::read_dir` so that it shares no code with `include_dir`'s walk.
fn collect_markdown_files(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    for entry in fs::read_dir(dir).expect("content/ must be readable") {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_markdown_files(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).unwrap();
            let text = fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{} must be readable: {error}", path.display()));
            out.insert(relative.to_string_lossy().into_owned(), text);
        }
    }
}

/// **`content/SKILL.md` must reach a harness as a skill.**
///
/// Its leading `---` block is what every harness reads to discover the
/// provisioned skill — `name:` and `description:` are the entry a session sees
/// in its skill list. Provisioning copies the embed verbatim, so the only way to
/// break this is to edit the file; asserting it on the **extracted** copy is
/// what makes the claim about what a harness actually reads rather than about
/// what the embed happens to hold.
///
/// The old third limb — that the body then opened on a file directive and a unit
/// marker — went with the markers. Nothing stands in for it, because there is
/// nothing left for the frontmatter to be distinguished *from*: the body is
/// prose from its first byte.
#[test]
fn the_skill_frontmatter_survives_provisioning() {
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
/// **Eleven**, and `methodology` is the one that left: it was the twelfth for as
/// long as `content/MANDATE.md` told a session that a `defers=<id>` marker had a
/// body `grove-llm methodology <id>` would serve. The corpus stopped reaching for
/// it when it began routing by **path** — a condition names the reference file
/// carrying its procedure, and a session opens it — and the verb itself is now
/// off the CLI with the rest of the mandate machinery.
///
/// This check is **more** load-bearing for that, not less: the skill is once
/// again the only thing teaching a session which verbs exist, so a verb it names
/// wrongly is a call a session cannot make and has no second source to correct
/// it from.
const INSTRUCTED_VERBS: [&str; 11] = [
    "brief-chain",
    "complete",
    "finish-commit",
    "leaf-add",
    "leaf-add-pair",
    "leaf-decompose",
    "leaf-insert",
    "leaf-prune",
    "leaf-retire",
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

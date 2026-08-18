//! **What a session actually pays to read, budgeted per kind.**
//!
//! A **loaded path** is per-kind and has two halves. The *static* half is what
//! every session of that kind reads unconditionally: the guaranteed core in
//! `${prompt}`, the provisioned `content/SKILL.md`, and that kind's reference
//! file. The *reachable* half adds every file a condition on that path can send
//! it to — the transitive closure of the pointer graph
//! `docs/specs/corpus-rule-ownership.md` records in its **load** column.
//!
//! Nineteen kinds share ten reference files, so the budget is a **table, not a
//! number**, and this file owns the inventory's **load** column as
//! `tests/rule_ownership.rs` owns its **owner** column.
//!
//! The two read the same tables by different means and are **not** a shared
//! reader: that file sweeps normalised text for a rule id under a heading, this
//! one parses rows into a typed record. The duplication is deliberate — a rule's
//! wording and a rule's load predicate are different questions at different
//! grains — but it is duplication, and the two could in principle disagree about
//! what a row is. What stops that mattering is that both are pinned to the same
//! spec by name: a row this reader loses fails the count control below, and a
//! rule that file loses fails its own.
//!
//! **The measurement runs through the runtime, not beside it.**
//! [`grove::prompt::compose`] composes the real core — the real template, the
//! real per-kind reference mapping, the embed's own signal bytes — and the
//! reference file is [`grove::prompt::reference_file`]'s answer, not a second
//! table. A budget computed by a parallel notion of what a session reads drifts
//! from the real one and then lies, which is the failure mode this file exists
//! to avoid rather than to reproduce. The only fixtures are the three
//! **launch-varying** values `compose` takes — handle, stated VCS, provisioned
//! locations — held fixed so the measurement is reproducible on any machine.
//!
//! **The budget is in words, and that is a decision rather than an omission.**
//! Tokens are what a session pays and words are what a `cargo test` can count:
//! a token figure is model-specific and reproducible only with a vendored
//! tokenizer and vocabulary, and a budget that needs a network call or a
//! multi-megabyte dependency is one that stops running. Words track tokens
//! monotonically across prose in one voice, which is all a *growth* alarm needs.
//! What a word count cannot price is a corpus that shifts register — a page of
//! tables or code costs far more tokens per word than prose does — so the
//! reading is "this path grew", never "this path costs N tokens".
//!
//! **This file is a shape measure, and shape measures are what this grove
//! replaced the 500-line ceiling to stop over-claiming.** It says a path is
//! small. It says nothing about whether the path still carries the rules — that
//! is `tests/lifecycle_invariants.rs`'s behavioural coverage, and
//! `tests/rule_ownership.rs`'s single-source sweep. Neither is evidence for the
//! other, and a green budget is not evidence that a session obeys anything.

use grove::leaf::Kind;
use grove::{methodology, prompt};
use std::collections::{BTreeMap, BTreeSet};

/// The inventory, read from the design it is an assertion over.
const SPEC: &str = include_str!("../docs/specs/corpus-rule-ownership.md");

// -- Measuring one kind's loaded path -----------------------------------------

/// The launch-varying values [`grove::prompt::compose`] takes, held fixed.
///
/// A real launch's handle, VCS statement and provisioned-directory list vary by
/// tree and by machine, so measuring the composed core against a live launch
/// would make the budget machine-dependent. These are representative rather than
/// minimal — three locations, because a machine with Claude Code, Codex and Pi
/// installed is the common case and the widest of them, so the core is measured
/// at its ordinary size rather than at its floor.
const FIXTURE_HANDLE: &str = "loaded-path-budgets-k10";
const FIXTURE_VCS: &str = "this working tree is jj-enabled (jj workspace root: `/w`)";
const FIXTURE_LOCATIONS: [&str; 3] = [
    "/home/u/.claude/skills/grove",
    "/home/u/.codex/skills/grove",
    "/home/u/.pi/agent/skills/grove",
];

/// Whitespace-separated tokens. The same measure `wc -w` makes, so a contributor
/// can recompute a failure at a shell without this test.
fn words(text: &str) -> usize {
    text.split_whitespace().count()
}

/// A frontmatter document's body — everything after the closing `---` — or
/// `None` when the document does not open with frontmatter.
///
/// **`None` rather than the whole text**, because the caller that needs this is
/// measuring `content/SKILL.md`, whose header is a routing entry a harness reads
/// rather than guidance a session reads: counting it would let a `description:`
/// rewrite eat into the budget, and *silently falling back to the whole file* on
/// a stripped header is the fail-open direction — the measurement rises by the
/// header's 71 words and every budget absorbs it. The helper this replaced
/// returned `None` for exactly that reason, and it is kept.
fn body(text: &str) -> Option<&str> {
    text.strip_prefix("---\n")?
        .split_once("\n---\n")
        .map(|(_, after)| after)
}

/// The embedded corpus as path→text, embed-relative.
fn corpus() -> BTreeMap<String, &'static str> {
    methodology::markdown_files()
        .expect("the real embed must be readable")
        .into_iter()
        .collect()
}

/// One file's contribution to a path.
///
/// **Only `content/SKILL.md` is measured as a body.** It is the corpus's one
/// frontmatter document, and the discrimination is by path rather than by
/// sniffing a leading `---`, so a reference file that opened on a horizontal
/// rule could not silently lose its head.
fn file_words(corpus: &BTreeMap<String, &'static str>, path: &str) -> usize {
    let text = corpus
        .get(path)
        .unwrap_or_else(|| panic!("the embed must carry content/{path}"));
    if path != SKILL {
        return words(text);
    }
    words(body(text).unwrap_or_else(|| {
        panic!("content/{SKILL} must open with the YAML frontmatter a harness reads")
    }))
}

/// One kind's measured loaded path, both halves, with the parts that make it up.
struct Measured {
    /// `${prompt}` in full, composed by the runtime — load instruction, runtime
    /// facts, and this kind's signal file inlined verbatim.
    core: usize,
    /// `content/SKILL.md`'s body.
    skill: usize,
    /// `prompt::reference_file(kind)`.
    reference: usize,
    /// Every file reachable from the static path by a cross-file edge, and not
    /// already on it.
    reached: BTreeMap<String, usize>,
}

impl Measured {
    /// What every session of this kind reads unconditionally.
    fn static_words(&self) -> usize {
        self.core + self.skill + self.reference
    }

    /// The static path plus everything a condition on it can open — the worst
    /// case, if every condition fires in one session.
    fn reachable_words(&self) -> usize {
        self.static_words() + self.reached.values().sum::<usize>()
    }

    /// The breakdown a failure prints, so a contributor sees *which* part grew.
    fn report(&self, kind: Kind) -> String {
        let mut reached: Vec<&String> = self.reached.keys().collect();
        reached.sort_by_key(|path| std::cmp::Reverse(self.reached[*path]));
        let listed: Vec<String> = reached
            .iter()
            .map(|path| format!("      {:<32} {:>5}", path, self.reached[*path]))
            .collect();
        format!(
            "{}:\n      {:<32} {:>5}\n      {:<32} {:>5}\n      {:<32} {:>5}\n\
             \x20     {:<32} {:>5}\n{}\n      {:<32} {:>5}",
            kind.label(),
            "guaranteed core",
            self.core,
            "SKILL.md (body)",
            self.skill,
            prompt::reference_file(kind),
            self.reference,
            "= static",
            self.static_words(),
            listed.join("\n"),
            "= reachable",
            self.reachable_words(),
        )
    }
}

/// Measure `kind`'s loaded path against the real corpus and the real composition.
fn measure(kind: Kind, corpus: &BTreeMap<String, &'static str>, graph: &Graph) -> Measured {
    let core = prompt::compose(kind, FIXTURE_HANDLE, FIXTURE_VCS, &FIXTURE_LOCATIONS)
        .expect("the real embed must compose every kind's prompt");
    let reference = prompt::reference_file(kind);

    // The seed is the *whole* static path, signal file included — the same set
    // `static_files` computes. Seeding only `SKILL.md` and the reference file
    // would silently drop an edge out of a signal file, and would double-count
    // one into it: the signal file's words are already inside `core`.
    let seed = static_files(kind, corpus);
    let reached = graph
        .closure(&seed)
        .into_iter()
        .filter(|path| !seed.contains(path))
        .map(|path| {
            let count = file_words(corpus, &path);
            (path, count)
        })
        .collect();

    Measured {
        core: words(&core),
        skill: file_words(corpus, SKILL),
        reference: file_words(corpus, reference),
        reached,
    }
}

// -- The budget table ---------------------------------------------------------

/// **The per-kind budget, and the headroom is a stated choice.**
///
/// Each ceiling is the measurement taken when this leaf landed, plus **10%**,
/// rounded up to the next 25 words. A budget fitted to today's bytes fails on
/// the next legitimate sentence; one set at twice the measurement measures
/// nothing. Ten percent of a ~1,200-word static path is ~120 words — two or
/// three paragraphs of genuine new guidance, which is a real increment's worth —
/// while any of the growth this grove removed would blow it several times over.
/// A ceiling may then stand until the corpus has moved ~15% the other way; the
/// band's width, and why it has to have one, are on
/// [`widest_credible_ceiling`].
///
/// **A rewrite that needs more room raises the number and says why**, in the
/// same commit. That is the point of a table rather than a formula: the raise is
/// visible per kind, and a `SKILL.md` edit shows up in all nineteen rows at once
/// because `SKILL.md` is on all nineteen static paths.
///
/// The `reachable` column barely varies, and that is a finding rather than a
/// defect in the measure: ten of the pointer graph's fourteen edges leave
/// `SKILL.md`, so almost the whole conditional corpus hangs off every kind's
/// path. What distinguishes the rows is the kind reference, the signal file, and
/// `grilling.md` — reachable only from `references/requirements.md`.
struct Budget {
    kind: Kind,
    static_words: usize,
    reachable_words: usize,
}

const BUDGETS: &[Budget] = &[
    // requirements: measured 1562 static, 12533 reachable
    Budget {
        kind: Kind::Requirements,
        static_words: 1725,
        reachable_words: 13800,
    },
    // design: measured 1149 static, 11741 reachable
    Budget {
        kind: Kind::Design,
        static_words: 1275,
        reachable_words: 12925,
    },
    // planning: measured 1317 static, 11909 reachable
    Budget {
        kind: Kind::Planning,
        static_words: 1450,
        reachable_words: 13100,
    },
    // prototype: measured 1169 static, 11761 reachable
    Budget {
        kind: Kind::Prototype,
        static_words: 1300,
        reachable_words: 12950,
    },
    // impl: measured 1334 static, 11926 reachable
    Budget {
        kind: Kind::Impl,
        static_words: 1475,
        reachable_words: 13125,
    },
    // research-a: measured 1383 static, 11975 reachable
    Budget {
        kind: Kind::ResearchA,
        static_words: 1525,
        reachable_words: 13175,
    },
    // research-b: measured 1383 static, 11975 reachable
    Budget {
        kind: Kind::ResearchB,
        static_words: 1525,
        reachable_words: 13175,
    },
    // combine-research: measured 1225 static, 11817 reachable
    Budget {
        kind: Kind::CombineResearch,
        static_words: 1350,
        reachable_words: 13000,
    },
    // finish: measured 1938 static, 12530 reachable
    Budget {
        kind: Kind::Finish,
        static_words: 2150,
        reachable_words: 13800,
    },
    // review-requirements: measured 1260 static, 11852 reachable
    Budget {
        kind: Kind::ReviewRequirements,
        static_words: 1400,
        reachable_words: 13050,
    },
    // review-design: measured 1260 static, 11852 reachable
    Budget {
        kind: Kind::ReviewDesign,
        static_words: 1400,
        reachable_words: 13050,
    },
    // review-planning: measured 1260 static, 11852 reachable
    Budget {
        kind: Kind::ReviewPlanning,
        static_words: 1400,
        reachable_words: 13050,
    },
    // review-prototype: measured 1260 static, 11852 reachable
    Budget {
        kind: Kind::ReviewPrototype,
        static_words: 1400,
        reachable_words: 13050,
    },
    // review-impl: measured 1260 static, 11852 reachable
    Budget {
        kind: Kind::ReviewImpl,
        static_words: 1400,
        reachable_words: 13050,
    },
    // integrate-review-requirements: measured 1280 static, 11872 reachable
    Budget {
        kind: Kind::IntegrateReviewRequirements,
        static_words: 1425,
        reachable_words: 13075,
    },
    // integrate-review-design: measured 1280 static, 11872 reachable
    Budget {
        kind: Kind::IntegrateReviewDesign,
        static_words: 1425,
        reachable_words: 13075,
    },
    // integrate-review-planning: measured 1280 static, 11872 reachable
    Budget {
        kind: Kind::IntegrateReviewPlanning,
        static_words: 1425,
        reachable_words: 13075,
    },
    // integrate-review-prototype: measured 1280 static, 11872 reachable
    Budget {
        kind: Kind::IntegrateReviewPrototype,
        static_words: 1425,
        reachable_words: 13075,
    },
    // integrate-review-impl: measured 1280 static, 11872 reachable
    Budget {
        kind: Kind::IntegrateReviewImpl,
        static_words: 1425,
        reachable_words: 13075,
    },
];

// -- The pointer graph, read from the inventory's load column -----------------

const SKILL: &str = "SKILL.md";

/// One inventory row, at the grain the load column is asserted at.
struct Row {
    rule: String,
    /// The paths its grouping heading names. Almost always one; the
    /// `references/finish.md` group names two, and
    /// [`a_group_naming_two_owners_carries_only_static_rows`] is why that costs
    /// nothing here.
    owners: Vec<String>,
    mirror: String,
    load: Load,
}

/// The load column's two forms, per *Load predicate notation*.
enum Load {
    /// `static(K)` — on the static path of every kind in `K`.
    Static { kinds: Vec<Kind>, source: String },
    /// `on(t) @ F` — conditional, reached from a sentence in `F`.
    On { file: String },
}

/// The cross-file pointer graph: `source → targets`.
///
/// Built from the **cross-file** conditional rows only. A row whose `@` file is
/// its own owner records an in-file condition — which part of an already-open
/// file applies — and moves the session nowhere; reading those as edges makes
/// each a self-loop and condemns 45 of the 92 conditional rows.
struct Graph {
    edges: BTreeSet<(String, String)>,
}

impl Graph {
    fn targets_of<'a>(&'a self, source: &'a str) -> impl Iterator<Item = &'a String> + 'a {
        self.edges
            .iter()
            .filter(move |(from, _)| from == source)
            .map(|(_, to)| to)
    }

    /// Every file reachable from `seed`, `seed` included.
    fn closure(&self, seed: &BTreeSet<String>) -> BTreeSet<String> {
        let mut seen = seed.clone();
        let mut frontier: Vec<String> = seed.iter().cloned().collect();
        while let Some(path) = frontier.pop() {
            for target in self.targets_of(&path) {
                if seen.insert(target.clone()) {
                    frontier.push(target.clone());
                }
            }
        }
        seen
    }
}

// -- Reading the inventory ----------------------------------------------------

/// The inventory's rows, in document order, with the grouping heading each one
/// sits under carried down as its owner.
///
/// Scoped to *The inventory* section: the spec carries other tables — the
/// canonical trigger sentences, the deferral policy, the relocation table — and
/// a reader that swept the whole document would measure a graph over them too.
///
/// A malformed row is **kept** rather than skipped, as [`Row::rule`] `""` with no
/// owners, so [`the_inventory_reader_sees_every_row_and_rejects_a_malformed_one`]
/// can fail on it. Silently dropping a row is the fail-open direction: every
/// assertion below is over the rows the reader returns, so a reader that drops
/// half the inventory passes all of them.
fn inventory() -> Vec<Row> {
    let section = SPEC
        .split_once("\n### The inventory\n")
        .expect("the spec must carry an inventory section")
        .1
        .split_once("\n### The two contradictions")
        .expect("the inventory section must end at the contradictions")
        .0;

    let mut owners: Vec<String> = Vec::new();
    let mut rows = Vec::new();
    for line in section.lines() {
        if line.starts_with("####") {
            // **Cleared, not inherited.** A heading whose path this reader cannot
            // see would otherwise leave the previous group's owner standing, and
            // the rows under it would be *mis-attributed* rather than dropped —
            // a green sweep asserting the wrong file. Cleared, they parse with no
            // owner and fail the reader control by name. The two path-less
            // headings in the inventory (*The ten kind references*, *The format
            // files*) carry no table of their own, so clearing costs nothing.
            owners = heading_paths(line);
            continue;
        }
        let Some(cells) = table_cells(line) else {
            continue;
        };
        rows.push(read_row(&cells, &owners));
    }
    rows
}

/// The corpus paths a grouping heading names, embed-relative.
///
/// Headings spell a path either way — `` `content/references/grove.md` `` under
/// the loop-step groups, `` `references/requirements.md` `` under the kind
/// references — so the `content/` prefix is stripped and the two spellings meet.
fn heading_paths(line: &str) -> Vec<String> {
    line.split('`')
        .skip(1)
        .step_by(2)
        .filter(|span| span.ends_with(".md"))
        .map(|span| span.trim_start_matches("content/").to_owned())
        .collect()
}

/// A table row's cells, or `None` for a line that is not one — the header, its
/// `|---|` rule, and every line of prose between the tables.
fn table_cells(line: &str) -> Option<Vec<String>> {
    let trimmed = line.trim();
    if !trimmed.starts_with('|') {
        return None;
    }
    let cells: Vec<String> = trimmed
        .trim_matches('|')
        .split('|')
        .map(|cell| cell.trim().to_owned())
        .collect();
    let first = cells.first()?;
    if first == "rule" || first.starts_with("---") {
        return None;
    }
    Some(cells)
}

/// One row. Five columns per *Every row carries five columns, in every table*;
/// anything else is malformed and is reported rather than dropped.
fn read_row(cells: &[String], owners: &[String]) -> Row {
    if cells.len() != 5 {
        return Row {
            rule: String::new(),
            owners: Vec::new(),
            mirror: cells.join(" | "),
            load: Load::On {
                file: String::new(),
            },
        };
    }
    Row {
        rule: backticked(&cells[0]).unwrap_or_default(),
        owners: owners.to_vec(),
        mirror: cells[2].clone(),
        load: read_load(&cells[3]),
    }
}

/// The first backticked span of a cell — a row's rule id, a load predicate's
/// body.
fn backticked(cell: &str) -> Option<String> {
    cell.split('`').nth(1).map(str::to_owned)
}

/// The load column, per *Load predicate notation*. An unreadable value yields an
/// empty `On` file, which fails the reader control rather than passing quietly.
fn read_load(cell: &str) -> Load {
    let Some(predicate) = backticked(cell) else {
        return Load::On {
            file: String::new(),
        };
    };
    if let Some(kinds) = predicate
        .strip_prefix("static(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        return Load::Static {
            kinds: read_kind_set(kinds),
            source: kinds.to_owned(),
        };
    }
    let file = predicate
        .split_once(" @ ")
        .map(|(_, file)| file.trim_start_matches("content/").to_owned())
        .unwrap_or_default();
    Load::On { file }
}

/// The kind set a `static(K)` predicate names.
///
/// The six spellings are the closed list in *Load predicate notation* — stated
/// there rather than only here, because a notation whose sole definition is its
/// resolver cannot be checked against anything. An unrecognised spelling yields
/// the empty set, which fails
/// [`every_static_row_names_a_kind_set_the_notation_defines`] rather than
/// asserting the row against nothing.
fn read_kind_set(source: &str) -> Vec<Kind> {
    let all = || Kind::ALL.to_vec();
    match source {
        "19" => all(),
        "18" => all().into_iter().filter(|k| *k != Kind::Finish).collect(),
        "research" => vec![Kind::ResearchA, Kind::ResearchB],
        "review-*" => family(all(), "review-"),
        "integrate-review-*" => family(all(), "integrate-review-"),
        _ => source
            .strip_prefix('{')
            .and_then(|rest| rest.strip_suffix('}'))
            .and_then(|label| all().into_iter().find(|kind| kind.label() == label))
            .into_iter()
            .collect(),
    }
}

/// The kinds whose label opens with `prefix`, and — for `review-` — not with a
/// longer prefix that would swallow the integration family.
fn family(kinds: Vec<Kind>, prefix: &str) -> Vec<Kind> {
    kinds
        .into_iter()
        .filter(|kind| {
            kind.label().starts_with(prefix)
                && (prefix != "review-" || !kind.label().starts_with("integrate-"))
        })
        .collect()
}

/// The cross-file pointer graph the inventory's conditional rows realise.
fn graph(rows: &[Row]) -> Graph {
    Graph {
        edges: rows
            .iter()
            .filter_map(|row| match &row.load {
                Load::On { file } if !row.owners.contains(file) => Some((file, &row.owners)),
                _ => None,
            })
            .flat_map(|(file, owners)| {
                owners
                    .iter()
                    .map(move |owner| (file.clone(), owner.clone()))
            })
            .collect(),
    }
}

/// Every file that is static for at least one kind: `SKILL.md`, the ten
/// reference files, and the two signal files.
///
/// The signal paths are recovered **through the runtime seam** rather than
/// named here: [`grove::prompt::ending_of`] returns the bytes a kind's prompt
/// inlines, and the embed path carrying those bytes is that kind's ending file.
/// `src/prompt.rs` is out of this workstream's scope, so the path is derived
/// from what it already exposes instead of by widening it — and the derivation
/// fails loudly if the two signal files ever stop being distinguishable.
fn static_files(kind: Kind, corpus: &BTreeMap<String, &'static str>) -> BTreeSet<String> {
    let ending = prompt::ending_of(kind).expect("every kind has an inlined ending");
    let carriers: Vec<&String> = corpus
        .iter()
        .filter(|(_, text)| **text == ending)
        .map(|(path, _)| path)
        .collect();
    assert_eq!(
        carriers.len(),
        1,
        "{}'s inlined ending must be carried by exactly one embedded file, not {:?} — \
         two files with identical bytes make the ending's provenance unrecoverable",
        kind.label(),
        carriers,
    );

    [
        SKILL.to_owned(),
        prompt::reference_file(kind).to_owned(),
        carriers[0].clone(),
    ]
    .into_iter()
    .collect()
}

// -- The budgets --------------------------------------------------------------

/// The headroom a ceiling is **set** at: measurement + 10%, rounded up to the
/// next 25 words. This is what the table's numbers were computed from, and every
/// failure below prints it, so re-fitting a row is reading a number rather than
/// recomputing one.
fn ceiling_over(measured: usize) -> usize {
    (measured + measured / 10).div_ceil(25) * 25
}

/// The headroom a ceiling is **allowed** to keep: measurement + 25%.
///
/// **The two numbers are deliberately different, and a single number was a
/// defect.** With the band's edge at the ceiling, every budget sat exactly on its
/// own limit, so any shrink at all failed — deleting one fifteen-word bullet from
/// `references/decompose.md` turned fourteen rows red, on a reachable path of
/// ~12,000 words. Worse, it was **self-defeating**: the over-budget failure
/// above tells a contributor to move a procedure out of a static file, and doing
/// so shrinks the measurement and trips this check for the other eighteen kinds.
///
/// So a ceiling is set at +10% and may stand until the corpus has moved ~15% the
/// other way. That keeps both readings honest — a path may grow by a couple of
/// paragraphs before failing, and a ceiling may not drift into measuring nothing
/// — without making ordinary editing a re-fitting exercise across 38 numbers.
fn widest_credible_ceiling(measured: usize) -> usize {
    (measured + measured / 4).div_ceil(25) * 25
}

/// `kind`'s row, or a failure naming the kind — a table missing a row would
/// otherwise budget eighteen paths and leave one unmeasured.
fn budget(kind: Kind) -> &'static Budget {
    BUDGETS
        .iter()
        .find(|row| row.kind == kind)
        .unwrap_or_else(|| panic!("BUDGETS must carry a row for {}", kind.label()))
}

/// **Every kind's static loaded path fits its budget.**
///
/// The static path is what a session of that kind pays *unconditionally*, and it
/// is the number this grove set out to shrink: `content/SKILL.md` alone was
/// 3,152 words at the start of it.
///
/// Every kind is measured before anything is asserted, so one over-budget path
/// does not hide the other eighteen, and the failure prints the breakdown —
/// core, `SKILL.md`, reference file — because *which* part grew is the whole
/// actionable content of the failure.
#[test]
fn every_kinds_static_loaded_path_fits_its_budget() {
    let corpus = corpus();
    let graph = graph(&inventory());

    let over: Vec<String> = Kind::ALL
        .into_iter()
        .map(|kind| (kind, measure(kind, &corpus, &graph)))
        .filter(|(kind, measured)| measured.static_words() > budget(*kind).static_words)
        .map(|(kind, measured)| {
            format!(
                "  {} static is {} words against a budget of {} (a ceiling fitted to \
                 this measurement would be {})\n{}",
                kind.label(),
                measured.static_words(),
                budget(kind).static_words,
                ceiling_over(measured.static_words()),
                measured.report(kind),
            )
        })
        .collect();

    assert!(
        over.is_empty(),
        "static loaded paths over budget:\n{}\n\nMove a procedure into the reference file \
         its condition names, or raise the budget in this file and say why in the same \
         commit. Raising it is a design change: `content/SKILL.md` is on all nineteen \
         static paths, so a change there moves every row at once.",
        over.join("\n"),
    );
}

/// **Every kind's reachable loaded path fits its budget** — the worst case, if
/// every condition on the static path fires in one session.
///
/// This is the half a static budget cannot see: a rule moved out of `SKILL.md`
/// into a conditionally-loaded file leaves the static path smaller while the
/// corpus a session can be sent into is unchanged. Ten of the fourteen edges
/// leave `SKILL.md`, so this number barely varies by kind — which is why it is
/// reported per kind rather than trusted to.
#[test]
fn every_kinds_reachable_loaded_path_fits_its_budget() {
    let corpus = corpus();
    let graph = graph(&inventory());

    let over: Vec<String> = Kind::ALL
        .into_iter()
        .map(|kind| (kind, measure(kind, &corpus, &graph)))
        .filter(|(kind, measured)| measured.reachable_words() > budget(*kind).reachable_words)
        .map(|(kind, measured)| {
            format!(
                "  {} reachable is {} words against a budget of {} (a ceiling fitted to \
                 this measurement would be {})\n{}",
                kind.label(),
                measured.reachable_words(),
                budget(kind).reachable_words,
                ceiling_over(measured.reachable_words()),
                measured.report(kind),
            )
        })
        .collect();

    assert!(
        over.is_empty(),
        "reachable loaded paths over budget:\n{}\n\nA file grew, or a new edge out of a \
         static file pulled another one onto the path.",
        over.join("\n"),
    );
}

/// **The control: every budget can fail, and each one is still an alarm.**
///
/// A ceiling raised out of reach passes forever and measures nothing, which is
/// the failure mode the 500-line prose ceiling this file replaced had — 500 over
/// a body the rewrite estimated at ~200 lines. So each ceiling is asserted from
/// **both** sides: the tests above hold the corpus under it, and this one holds
/// it within [`widest_credible_ceiling`] of what the corpus actually measures.
///
/// The consequence is deliberate and cuts both ways. A path that grows past its
/// ceiling fails above; a corpus that shrinks by more than ~15% fails here, until
/// the table is re-fitted. That threshold is the width of the band rather than an
/// independent judgement — see [`widest_credible_ceiling`] for why a band with no
/// width made the two directions contradict each other.
#[test]
fn every_budget_stays_within_headroom_of_the_measurement() {
    let corpus = corpus();
    let graph = graph(&inventory());

    let slack: Vec<String> = Kind::ALL
        .into_iter()
        .flat_map(|kind| {
            let measured = measure(kind, &corpus, &graph);
            let row = budget(kind);
            [
                ("static", measured.static_words(), row.static_words),
                ("reachable", measured.reachable_words(), row.reachable_words),
            ]
            .into_iter()
            .filter(|(_, is, allowed)| *allowed > widest_credible_ceiling(*is))
            .map(move |(half, is, allowed)| {
                format!(
                    "  {} {half}: budget {allowed} is beyond {}, the widest credible \
                     ceiling over a measurement of {is} — re-fit it to {}",
                    kind.label(),
                    widest_credible_ceiling(is),
                    ceiling_over(is),
                )
            })
        })
        .collect();

    assert!(
        slack.is_empty(),
        "budgets with more headroom than the design allows:\n{}\n\nA budget is set at \
         measurement + 10% and may stand until the measurement has fallen ~15% below it. \
         Re-fit these rows to what the corpus now measures — a ceiling nothing approaches \
         is not an alarm.",
        slack.join("\n"),
    );
}

// -- The load column, checked against the runtime and against itself ----------

/// **`static(K)` is checked against the runtime that composes the path.**
///
/// Only three things can be static, per *Load predicate notation*: the
/// guaranteed core — whose one embedded part is the kind's signal file —
/// `SKILL.md`, and `reference_file(k)`. A row claiming `static(K)` whose owner
/// is none of those for some `k ∈ K` is claiming a session reads a file
/// unconditionally that the runtime never puts in front of it.
///
/// The superseded inventory labelled the loop-step rows `always(19)`, which
/// `src/prompt.rs` contradicts outright: `references/decompose.md` is on no
/// kind's static path at all. This check is what would not have survived that.
#[test]
fn every_static_row_is_owned_by_a_file_the_runtime_puts_on_that_kinds_path() {
    let corpus = corpus();
    let statics: BTreeMap<&str, BTreeSet<String>> = Kind::ALL
        .into_iter()
        .map(|kind| (kind.label(), static_files(kind, &corpus)))
        .collect();

    for row in inventory() {
        let Load::Static { kinds, source } = &row.load else {
            continue;
        };
        for kind in kinds {
            let on_path = &statics[kind.label()];
            assert!(
                row.owners.iter().any(|owner| on_path.contains(owner)),
                "{}: the row claims `static({source})`, but its owner {:?} is not on \
                 {}'s static path {:?}. Either the rule belongs to a file the runtime \
                 loads for that kind, or the predicate is conditional.",
                row.rule,
                row.owners,
                kind.label(),
                on_path,
            );
        }
    }
}

/// Every `static(K)` names a kind set *Load predicate notation* defines.
///
/// An unrecognised spelling reads as the empty set, and an empty set makes the
/// check above vacuous for that row — the fail-open direction, and the reason
/// this is a test rather than a `_ => vec![]` left to speak for itself.
#[test]
fn every_static_row_names_a_kind_set_the_notation_defines() {
    for row in inventory() {
        let Load::Static { kinds, source } = &row.load else {
            continue;
        };
        assert!(
            !kinds.is_empty(),
            "{}: `static({source})` names no kind this reader recognises, so the row is \
             asserted against nothing. Add the spelling to `read_kind_set` or fix the row.",
            row.rule,
        );
    }
}

/// **Reachability is an edge, and the edge is what is asserted — across files
/// only.**
///
/// For a conditional row `on(t) @ F` whose owner `O` is a different file, the
/// claim is that a sentence *in `F`* sends a session to `O`. A file-loadability
/// check cannot express that: it passes for a row whose `F` exists and is silent
/// about `O`, which is exactly how `content/references/driver.md` came to own
/// seven rules with nothing anywhere pointing at it.
///
/// Naming is checked as **the owner's path appearing in `F`'s text**, which is
/// what a session follows. It is a necessary condition rather than a sufficient
/// one — `F` could name the path in a sentence about something else — and the
/// sufficient half is the trigger-sentence audit in `tests/methodology.rs`,
/// which pins each sentence to its situation.
#[test]
fn every_cross_file_row_is_realised_by_a_sentence_naming_the_owners_path() {
    let corpus = corpus();

    for row in inventory() {
        let Load::On { file } = &row.load else {
            continue;
        };
        if row.owners.contains(file) {
            continue;
        }
        let text = corpus
            .get(file.as_str())
            .unwrap_or_else(|| panic!("{}: `@ {file}` names no embedded file", row.rule));
        for owner in &row.owners {
            assert!(
                text.contains(owner.as_str()),
                "{}: content/{file} must name `{owner}` — the row claims a session is sent \
                 there from it, and a rule nothing points at is present in content/ and \
                 deleted in effect.",
                row.rule,
            );
        }
    }
}

/// **Every reached file terminates at a static path, and no chain revisits a
/// file.**
///
/// **The two halves are not the same claim, and treating them as one is the
/// mistake this comment used to make.** Walking an edge backwards in a finite
/// acyclic graph ends at a node with no incoming edge — which need not be
/// *static*. So termination-at-static needs its own assertion, and it is made by
/// checking every file the corpus involves at all: not just rule owners, but
/// both ends of every edge. An edge **source** that owns no rows would otherwise
/// escape entirely, and a component of such files could sit unreachable from any
/// static path with every owner in it dutifully carrying an incoming edge.
///
/// That is spec assertion (1) — *`F` is static or is the owner of a reachable
/// row* — which the owner-only reading left unimplemented.
///
/// The incoming-edge half is what makes the in-file carve-out safe. **What is
/// reached is a file, not a rule**: one edge into `references/decompose.md` makes
/// every in-file condition there available, and a file with no edge into it fails
/// however many reflexive rows it carries.
#[test]
fn every_reached_file_terminates_at_a_static_path_without_a_cycle() {
    let corpus = corpus();
    let rows = inventory();
    let pointers = graph(&rows);

    let static_anywhere: BTreeSet<String> = Kind::ALL
        .into_iter()
        .flat_map(|kind| static_files(kind, &corpus))
        .collect();

    // No cycles: a file must not be reachable from its own targets. `closure`
    // terminates by construction, so this is the whole acyclicity check.
    let nodes: BTreeSet<&String> = pointers
        .edges
        .iter()
        .flat_map(|(from, to)| [from, to])
        .collect();
    for node in nodes {
        let onward = pointers.closure(&pointers.targets_of(node).cloned().collect());
        assert!(
            !onward.contains(node),
            "the pointer graph has a cycle through content/{node} — a session sent back to \
             a file it came from has no chain that terminates at a static path",
        );
    }

    // Every file the corpus involves at all — an owner of a rule, or either end
    // of an edge. Checking owners alone would miss a *source* that is itself
    // unreached: the chain out of it terminates nowhere, and every rule it sends
    // a session to is reachable only from a file nothing opens.
    let involved: BTreeSet<&String> = rows
        .iter()
        .flat_map(|row| row.owners.iter())
        .chain(pointers.edges.iter().flat_map(|(from, to)| [from, to]))
        .collect();
    let targets: BTreeSet<&String> = pointers.edges.iter().map(|(_, to)| to).collect();
    let orphans: Vec<&&String> = involved
        .iter()
        .filter(|path| !static_anywhere.contains(**path) && !targets.contains(*path))
        .collect();

    assert!(
        orphans.is_empty(),
        "these files own or point at rules and are neither static for any kind nor named \
         by any sentence: {orphans:?}. Every rule they carry or lead to is present in \
         content/ and deleted in effect — the failure `content/driving.md` used to be.",
    );
}

/// **The edge set the design records, recomputed rather than trusted.**
///
/// The spec states fourteen edges realised by 47 rows and names all fourteen.
/// They are pinned here as a set because they *are* the corpus's shape: ten out
/// of `SKILL.md`, and four that keep `SKILL.md` a page by hanging the format
/// files off the steps that need them.
///
/// A new edge is a design change — it puts another file on every reachable path
/// downstream of its source — so it lands here and in the spec together, or the
/// two have drifted.
#[test]
fn the_pointer_graph_is_the_edge_set_the_design_records() {
    let expected: BTreeSet<(String, String)> = [
        ("SKILL.md", "references/grove.md"),
        ("SKILL.md", "references/driver.md"),
        ("SKILL.md", "references/bootstrap.md"),
        ("SKILL.md", "references/execute.md"),
        ("SKILL.md", "references/decompose.md"),
        ("SKILL.md", "references/retire.md"),
        ("SKILL.md", "references/commit.md"),
        ("SKILL.md", "ADR-FORMAT.md"),
        ("SKILL.md", "SPEC-FORMAT.md"),
        ("SKILL.md", "CONTEXT-FORMAT.md"),
        ("references/requirements.md", "grilling.md"),
        ("references/decompose.md", "TASK-FORMAT.md"),
        ("references/decompose.md", "BRIEF-FORMAT.md"),
        ("references/retire.md", "BRIEF-FORMAT.md"),
    ]
    .into_iter()
    .map(|(from, to)| (from.to_owned(), to.to_owned()))
    .collect();

    assert_eq!(
        graph(&inventory()).edges,
        expected,
        "the inventory's cross-file rows realise a different pointer graph than the design \
         records (docs/specs/corpus-rule-ownership.md, *Load predicate notation*)",
    );
}

/// **A row reached from the condition register declares the `trigger` class.**
///
/// The sentence realising an edge out of `content/SKILL.md` *is* a trigger, so a
/// row claiming `@ SKILL.md` while declaring `none` says the file both points at
/// the rule and says nothing about it. That pairing is what left
/// `references/driver.md` with no incoming sentence at all.
#[test]
fn a_row_reached_from_the_condition_register_declares_the_trigger_class() {
    for row in inventory() {
        let Load::On { file } = &row.load else {
            continue;
        };
        if file != SKILL {
            continue;
        }
        assert!(
            row.mirror.contains("trigger"),
            "{}: the row is reached from content/SKILL.md, so its class is `trigger` — it \
             declares {:?} (docs/adr/restatement-declares-its-class.md)",
            row.rule,
            row.mirror,
        );
    }
}

/// **Every `trigger` row names a sentence in the canonical set of 26.**
///
/// The set is the reachability graph's edge list out of `content/SKILL.md`, so a
/// row citing a sentence number outside it is citing an edge the design does not
/// carry. The numbers come from the spec's own *The trigger sentences* table
/// rather than from a constant here, which is what makes the two move together.
#[test]
fn every_trigger_row_names_a_sentence_in_the_canonical_set() {
    let canonical = canonical_trigger_numbers();
    assert_eq!(
        canonical,
        (1..=26).collect::<BTreeSet<usize>>(),
        "docs/specs/corpus-rule-ownership.md must number its canonical trigger sentences \
         1 to 26 — the count is what tests/methodology.rs asserts the corpus against",
    );

    let mut cited = BTreeSet::new();
    for row in inventory() {
        let numbers = cited_sentences(&row.mirror);
        assert!(
            !row.mirror.contains("trigger") || !numbers.is_empty(),
            "{}: the row declares the `trigger` class and cites no sentence. Its partner \
             row may still claim the shared sentence, so the bijection below stays green \
             while this row's edge has become unnamed — which is the whole failure the \
             canonical set exists to prevent.",
            row.rule,
        );
        for number in numbers {
            assert!(
                canonical.contains(&number),
                "{}: the row cites trigger sentence {number}, which the canonical set does \
                 not carry",
                row.rule,
            );
            // Only a `trigger` row discharges coverage. A `none` row that happens
            // to mention a sentence number must not stand in for the edge nobody
            // declared.
            if row.mirror.contains("trigger") {
                cited.insert(number);
            }
        }
    }
    assert_eq!(
        cited, canonical,
        "every canonical trigger sentence must be claimed by at least one row — an \
         unclaimed sentence is a `SKILL.md` edge no rule needs",
    );
}

/// The sentence numbers of the spec's *The trigger sentences* table.
fn canonical_trigger_numbers() -> BTreeSet<usize> {
    let table = SPEC
        .split_once("\n#### The trigger sentences\n")
        .expect("the spec must carry the canonical trigger sentences")
        .1
        .split_once("\nTotal ")
        .expect("the trigger table must end at its total")
        .0;
    table
        .lines()
        .filter_map(table_cells)
        .filter_map(|cells| cells.first()?.parse().ok())
        .collect()
}

/// The trigger sentence numbers a mirror cell cites — `trigger (sentence 12)`
/// and `trigger (shares sentence 1)` alike.
///
/// The plural is accepted too. `sentences 3 and 4` does not contain `sentence `
/// with the trailing space, so a singular-only split would read a plural citation
/// as *no* citation — silently, and in the direction that makes a row's edge
/// vanish. It collects only the **first** number of such a cell, which is enough
/// for the two checks that read this: no row cites a plural today, and one that
/// did would be a design change to the sharing rule.
fn cited_sentences(mirror: &str) -> Vec<usize> {
    let singular = mirror.replace("sentences ", "sentence ");
    singular
        .split("sentence ")
        .skip(1)
        .filter_map(|rest| {
            let digits: String = rest.chars().take_while(char::is_ascii_digit).collect();
            digits.parse().ok()
        })
        .collect()
}

// -- Controls on the reader ---------------------------------------------------

/// **Positive control on the reader.** Every assertion above is over the rows
/// this reader returns, so a reader that silently returned nothing — or left
/// half the inventory ownerless — would pass all of them.
///
/// The failure a heading actually produces is **no owner**, not a dropped row:
/// [`inventory`] clears the owner on a heading it cannot read rather than letting
/// the previous group's stand, so the rows survive with nothing attributed to
/// them and fail the per-row check below by name.
///
/// The counts are the design's own, recomputed here rather than quoted: 139
/// rows, of which 47 are static and 92 conditional, and the conditional set
/// partitions into 45 in-file rows and 47 cross-file ones. They are asserted as
/// a floor and a partition rather than as exact totals, because a legitimate new
/// rule moves them — what may not move silently is the *shape*: every row has a
/// rule id and an owner, every load predicate reads, and the partition is
/// exhaustive.
#[test]
fn the_inventory_reader_sees_every_row_and_reads_every_predicate() {
    let rows = inventory();

    assert!(
        rows.len() >= 139,
        "the reader found {} inventory rows; the design records 139, so a table shape it \
         cannot parse has dropped rows silently",
        rows.len(),
    );
    for row in &rows {
        assert!(
            !row.rule.is_empty() && !row.owners.is_empty(),
            "a row parsed with no rule id or no owner: {:?} / {:?} / {}",
            row.rule,
            row.owners,
            row.mirror,
        );
        match &row.load {
            Load::Static { source, .. } => assert!(!source.is_empty()),
            Load::On { file } => assert!(
                !file.is_empty(),
                "{}: its load predicate is neither `static(...)` nor `on(...) @ <file>`",
                row.rule,
            ),
        }
    }

    let conditional = rows
        .iter()
        .filter(|row| matches!(row.load, Load::On { .. }))
        .count();
    let in_file = rows
        .iter()
        .filter(|row| match &row.load {
            Load::On { file } => row.owners.contains(file),
            Load::Static { .. } => false,
        })
        .count();
    let cross_file = conditional - in_file;
    assert!(
        in_file >= 40 && cross_file >= 40,
        "the conditional rows partition {in_file} in-file / {cross_file} cross-file; the \
         design records 45 and 47. A partition that has collapsed one way means a heading \
         or an `@` file stopped being recognised — and reading the in-file rows as edges \
         would make the graph uniformly cyclic",
    );
}

/// **A malformed row is reported, not skipped.** The reader keeps a row whose
/// table shape it cannot read, with an empty rule id, so the control above fails
/// on it — the alternative, a `continue`, is how a five-column schema quietly
/// becomes a suggestion.
#[test]
fn the_reader_keeps_a_malformed_row_rather_than_dropping_it() {
    let malformed = read_row(
        &["`some-rule`".to_owned(), "19 · context".to_owned()],
        &["SKILL.md".to_owned()],
    );
    assert!(
        malformed.rule.is_empty() && malformed.owners.is_empty(),
        "a row with the wrong number of columns must parse as unreadable",
    );

    let well_formed = read_row(
        &[
            "`some-rule` — what it says".to_owned(),
            "19 · context".to_owned(),
            "`trigger` (sentence 3)".to_owned(),
            "`on(a thing happens) @ SKILL.md`".to_owned(),
            "B".to_owned(),
        ],
        &["references/execute.md".to_owned()],
    );
    assert_eq!(well_formed.rule, "some-rule");
    assert_eq!(well_formed.owners, ["references/execute.md"]);
    assert_eq!(cited_sentences(&well_formed.mirror), [3]);
    match well_formed.load {
        Load::On { file } => assert_eq!(file, SKILL),
        Load::Static { .. } => panic!("an `on(...)` predicate must not read as static"),
    }
}

/// **A group heading naming two owners carries only static rows.**
///
/// `content/references/finish.md` and `content/SIGNAL-FINISH.md` share one
/// heading, and both are static for `finish`, so every row under it is
/// unambiguous. A *conditional* row there would be — the graph would need to know
/// which of the two a sentence points at — so the ambiguity is refused at the
/// schema rather than resolved by a guess.
#[test]
fn a_group_naming_two_owners_carries_only_static_rows() {
    for row in inventory() {
        if row.owners.len() < 2 {
            continue;
        }
        assert!(
            matches!(row.load, Load::Static { .. }),
            "{}: its heading names {:?}, so a conditional predicate leaves the edge's \
             target ambiguous. Give the rule a heading of its own.",
            row.rule,
            row.owners,
        );
    }
}

/// **Control on the kind-set reader.** A spelling that reads as a *wrong but
/// non-empty* set is the fail-open case
/// [`every_static_row_names_a_kind_set_the_notation_defines`] cannot see: a
/// `static(review-*)` that resolved to three of the five kinds would still be
/// checked, just not against the two it dropped.
///
/// The families are pinned by size and by membership, and `18` by which kind it
/// excludes — the one whose ending file is the other one.
#[test]
fn the_kind_set_reader_resolves_every_spelling_the_notation_uses() {
    assert_eq!(read_kind_set("19").len(), 19);

    let eighteen = read_kind_set("18");
    assert_eq!(eighteen.len(), 18);
    assert!(!eighteen.contains(&Kind::Finish));

    assert_eq!(
        read_kind_set("research"),
        [Kind::ResearchA, Kind::ResearchB]
    );
    assert_eq!(read_kind_set("{impl}"), [Kind::Impl]);
    assert_eq!(read_kind_set("{combine-research}"), [Kind::CombineResearch]);

    for (spelling, file) in [
        ("review-*", "references/review.md"),
        ("integrate-review-*", "references/integrate-review.md"),
    ] {
        let family = read_kind_set(spelling);
        assert_eq!(
            family.len(),
            5,
            "`static({spelling})` must name all five kinds of the family, not {family:?}",
        );
        for kind in family {
            assert_eq!(prompt::reference_file(kind), file);
        }
    }

    assert!(
        read_kind_set("{no-such-kind}").is_empty() && read_kind_set("all").is_empty(),
        "an undefined spelling must read as the empty set, so the row fails loudly rather \
         than being asserted against a set someone guessed",
    );
}

/// **Control on the frontmatter stripper.** The helper this replaced carried one,
/// on the reasoning that a measure returning something plausible for an
/// unrecognised header "would pass forever" — and that reasoning is unchanged.
///
/// Stripping `content/SKILL.md`'s frontmatter adds its 71 header words to the
/// measurement, which every budget has the headroom to absorb: `design` static
/// would go 1,149 → 1,220 against a ceiling of 1,275, and stay green. So the
/// header's absence has to fail as an error rather than as a number.
#[test]
fn the_frontmatter_stripper_fails_rather_than_measuring_the_header() {
    assert_eq!(body("---\nname: x\n---\nbody text\n"), Some("body text\n"));
    assert_eq!(body("no frontmatter here\n"), None);
    assert_eq!(body("---\nunterminated\n"), None);

    let corpus = corpus();
    assert!(
        body(corpus[SKILL]).is_some(),
        "content/{SKILL} must open with frontmatter — the measurement depends on it",
    );
    for (path, text) in &corpus {
        if path != SKILL {
            assert!(
                body(text).is_none(),
                "content/{path} opens with frontmatter, so `file_words` measures its \
                 header as prose. Only {SKILL} is a frontmatter document.",
            );
        }
    }
}

/// **Control on the trigger-number reader.** Every assertion over the canonical
/// set is over what [`cited_sentences`] extracts, so a reader that found no
/// numbers would leave every row uncited and the bijection vacuous — except that
/// the bijection is asserted in both directions, which is what turns this from a
/// hope into a check. Pinned here anyway, because the two mirror spellings are
/// easy to break with a rewording.
#[test]
fn the_trigger_number_reader_sees_both_mirror_spellings() {
    assert_eq!(cited_sentences("`trigger` (sentence 18)"), [18]);
    assert_eq!(cited_sentences("`trigger` (shares sentence 1)"), [1]);
    assert_eq!(cited_sentences("`none`"), Vec::<usize>::new());
    assert_eq!(
        cited_sentences("**none — byte-frozen and inlined into `${prompt}`**"),
        Vec::<usize>::new()
    );
}

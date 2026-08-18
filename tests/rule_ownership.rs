//! **One owner per rule, asserted over the corpus a session actually opens.**
//!
//! `docs/specs/corpus-rule-ownership.md` files every normative rule by *when a
//! session meets it*, and the property that falls out is single-source: a rule is
//! stated in one **procedure register** — one reference or format file — and
//! nowhere else. `content/SKILL.md` is the **condition register** and states
//! situations rather than procedures, so it is swept too, from the other
//! direction: none of the phrases below may appear there.
//!
//! Scope is every markdown file under `content/` **except `driving.md`**, and the
//! exclusion is derived rather than chosen. Only `SKILL.md` and
//! `reference_file(kind)` are ever on a static path, so a corpus file is reachable
//! only if some file names it — and no file under `content/` names `driving.md`
//! any more. A rule surviving only there is already deleted in effect, which is
//! the state the relocation table in the spec exists to end.
//! [`the_excluded_file_is_unreachable_and_the_exclusion_is_not_vacuous`] holds
//! both halves, and `corpus-split-k6` deletes it along with the file.
//!
//! **The sweeps are phrase-scoped, which is what makes them green now.** Each row
//! names its rule's canonical wording, normalised — emphasis and code markers
//! stripped, whitespace collapsed — because an unnormalised matcher silently
//! misses a wrapped or bolded occurrence, which is the fail-open direction. A
//! *differently worded* second statement in a file whose split belongs to a later
//! leaf is therefore out of a row's reach unless the row declares it: the three
//! `transient` sites below are exactly those, each one `corpus-split-k6`'s to
//! remove, and each asserted still present so the declaration cannot outlive the
//! duplication it excuses.
//!
//! Three controls stand over the matcher, and they are the ones
//! `docs/specs/corpus-rule-ownership.md` names for an **S** row: a positive
//! control that a phrase known present is found, a normalisation control that
//! emphasis and wrapping do not hide a match, and a cross-tree control that the
//! sweep still finds the class where it legitimately lives.

use grove::methodology;
use std::collections::BTreeMap;

/// The condition register. It carries situations and paths, never a procedure, so
/// every phrase below must be absent from it.
const CONDITION_REGISTER: &str = "SKILL.md";

/// The one corpus file no other corpus file names, and therefore the one that is
/// on no session's loaded path. Out of scope for that reason and no other.
const UNREACHED: &str = "driving.md";

/// Emphasis and code markers stripped, whitespace collapsed. The corpus wraps
/// prose at 80 columns and marks up mid-phrase, so a raw `contains` misses real
/// occurrences — the failure mode that reads as a clean sweep.
fn normalised(text: &str) -> String {
    let stripped: String = text
        .chars()
        .filter(|character| !matches!(character, '*' | '_' | '`'))
        .flat_map(char::to_lowercase)
        .collect();
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The embedded corpus, normalised, as path→text.
fn corpus() -> BTreeMap<String, String> {
    methodology::markdown_files()
        .expect("the real embed must be readable")
        .into_iter()
        .map(|(path, text)| (path, normalised(text)))
        .collect()
}

/// One rule this leaf homed, and where it is allowed to be stated.
struct Homed {
    /// The inventory's rule id. Never written into `content/` — see
    /// [`no_inventory_rule_id_is_written_into_the_corpus`].
    rule: &'static str,
    /// The file `docs/specs/corpus-rule-ownership.md` gives the rule.
    owner: &'static str,
    /// The rule's canonical wording, distinctive enough to identify it.
    phrase: &'static str,
    /// Files that state the rule in *these words* and are a later leaf's to
    /// strip. Asserted present, so a stale entry fails rather than lingering.
    transient: &'static [&'static str],
}

/// Every rule `loop-step-references-k11` landed in one of the seven loop-step
/// references, with the wording that identifies it.
const HOMED: &[Homed] = &[
    // -- content/references/execute.md ------------------------------------
    Homed {
        rule: "review-budget",
        owner: "references/execute.md",
        phrase: "at most one in-session reviewer across the whole picked leaf",
        transient: &[],
    },
    Homed {
        rule: "review-budget-predicate",
        owner: "references/execute.md",
        phrase: "adopted that mandate by running Bootstrap",
        transient: &[],
    },
    Homed {
        rule: "review-budget-by-kind",
        owner: "references/execute.md",
        phrase: "spend **none** — the pair supplies independent corpora",
        transient: &[],
    },
    Homed {
        rule: "doubt-pass-procedure",
        owner: "references/execute.md",
        phrase: "stripping the conclusion",
        transient: &[],
    },
    Homed {
        rule: "escalated-review-routes-through-config",
        owner: "references/execute.md",
        phrase: "Once review is escalated to the tree, grove owns the route",
        transient: &[],
    },
    Homed {
        rule: "verify-repo-claims-with-controls",
        owner: "references/execute.md",
        phrase: "clean-here plus dirty-there cannot be produced by a broken instrument",
        transient: &[],
    },
    Homed {
        rule: "enumerate-then-classify",
        owner: "references/execute.md",
        phrase: "Enumerate, then classify. Do not sweep a pattern list",
        transient: &[],
    },
    Homed {
        rule: "sweep-scope-is-the-claim",
        owner: "references/execute.md",
        phrase: "The sweep's scope is the claim's scope",
        transient: &[],
    },
    Homed {
        rule: "no-self-invalidating-count",
        owner: "references/execute.md",
        phrase: "Never document a claim with a count of itself",
        transient: &[],
    },
    Homed {
        rule: "check-the-rescued-clause",
        owner: "references/execute.md",
        phrase: "A clause rescued by its neighbour is not correct",
        transient: &[],
    },
    Homed {
        rule: "decisions-land-as-they-settle",
        owner: "references/execute.md",
        phrase: "Never reconstruct a session's decisions at the end of it",
        transient: &[],
    },
    Homed {
        rule: "escalation-names-the-tradeoff",
        owner: "references/execute.md",
        phrase: "name the trade-off you want input on",
        transient: &[],
    },
    // -- content/references/decompose.md ----------------------------------
    Homed {
        rule: "externalize-by-default",
        owner: "references/decompose.md",
        phrase: "the bar is *\"fits this session,\"* not \"I can finish it.\"",
        transient: &[],
    },
    Homed {
        rule: "bigger-than-brief-decomposes",
        owner: "references/decompose.md",
        phrase: "doing **only the first child** this session",
        transient: &[],
    },
    Homed {
        rule: "vertical-slice",
        owner: "references/decompose.md",
        phrase: "A good child leaf is a **vertical slice**",
        transient: &[],
    },
    Homed {
        rule: "wide-refactor-expand-contract",
        owner: "references/decompose.md",
        phrase: "Sequence it **expand → migrate → contract** instead, one leaf per stage",
        transient: &[],
    },
    Homed {
        rule: "fog-or-ticket",
        owner: "references/decompose.md",
        phrase: "a speculative leaf is the wrong shape for fog",
        transient: &[],
    },
    Homed {
        rule: "prior-art-research-is-its-own-leaf",
        owner: "references/decompose.md",
        phrase: "a research leaf of its own, cut ahead of the leaf that needs it",
        transient: &[],
    },
    Homed {
        rule: "research-brief-names-downstream-questions",
        owner: "references/decompose.md",
        phrase: "The research leaf's brief names the downstream questions",
        transient: &[],
    },
    Homed {
        rule: "review-chain-when-load-bearing",
        owner: "references/decompose.md",
        phrase: "artifact size and vendor preference are not the test",
        transient: &[],
    },
    Homed {
        rule: "vendor-pair-when-load-bearing",
        owner: "references/decompose.md",
        phrase: "is earned by a question load-bearing enough to pay for two corpora",
        transient: &[],
    },
    Homed {
        rule: "chain-is-lazy",
        owner: "references/decompose.md",
        phrase: "only if it has findings worth acting on",
        // `TASK-FORMAT.md` states the composition shapes as policy; that section
        // is `corpus-split-k6`'s to shed.
        transient: &["TASK-FORMAT.md"],
    },
    Homed {
        rule: "pair-is-eager",
        owner: "references/decompose.md",
        phrase: "it lands in one call or not at all",
        transient: &[],
    },
    Homed {
        rule: "creating-session-writes-the-body",
        owner: "references/decompose.md",
        phrase: "The creating session writes the new leaf's body",
        transient: &["TASK-FORMAT.md"],
    },
    Homed {
        rule: "name-step-kind-off-the-producer",
        owner: "references/decompose.md",
        phrase: "Name a chain step's kind off the producer that actually ran",
        transient: &[],
    },
    Homed {
        rule: "steps-share-the-producers-stem",
        owner: "references/decompose.md",
        phrase: "Give every step of a composed shape the producer's bare stem as its whole slug",
        transient: &[],
    },
    Homed {
        rule: "diversity-is-the-configs",
        owner: "references/decompose.md",
        phrase: "Diversity is the configuration's, not the tree's",
        transient: &[],
    },
    Homed {
        rule: "integration-placement",
        owner: "references/decompose.md",
        phrase: "the first sibling entry after the review whose subtree still holds live work",
        transient: &["TASK-FORMAT.md"],
    },
    Homed {
        rule: "no-adjacency-exception",
        owner: "references/decompose.md",
        phrase: "A session that departs anyway owns the drift",
        transient: &[],
    },
    // -- content/references/retire.md -------------------------------------
    Homed {
        rule: "retirement-is-filename-only",
        owner: "references/retire.md",
        phrase: "Retirement touches one filename and nothing else",
        transient: &[],
    },
    Homed {
        rule: "finish-is-the-drivers-to-discover",
        owner: "references/retire.md",
        phrase: "You do not discover that a grove is finished",
        transient: &[],
    },
    Homed {
        rule: "triage-picks-the-verb",
        owner: "references/retire.md",
        phrase: "name which of the three sentences is actually true before reaching for the CLI",
        transient: &[],
    },
    Homed {
        rule: "no-fourth-status",
        owner: "references/retire.md",
        phrase: "There is no fourth state",
        transient: &[],
    },
    Homed {
        rule: "prune-scopes-to-the-whole-path",
        owner: "references/retire.md",
        phrase: "prune each of its live steps",
        transient: &[],
    },
    Homed {
        rule: "node-close-is-implicit",
        owner: "references/retire.md",
        phrase: "The close asks the human nothing",
        transient: &[],
    },
    // -- content/references/commit.md -------------------------------------
    Homed {
        rule: "one-focused-commit",
        owner: "references/commit.md",
        phrase: "One task is **one focused commit**",
        transient: &[],
    },
    Homed {
        rule: "name-by-handle",
        owner: "references/commit.md",
        phrase: "Name each node the cascade closed the same way",
        transient: &[],
    },
    Homed {
        rule: "no-kind-prefix-in-commit-subject",
        owner: "references/commit.md",
        phrase: "Do not compensate for the bare stem with a subject convention",
        transient: &[],
    },
    Homed {
        rule: "jj-seal",
        owner: "references/commit.md",
        phrase: "`jj new` after describing, once the rename has landed",
        transient: &[],
    },
    // -- content/references/grove.md --------------------------------------
    Homed {
        rule: "durable-artifact-set",
        owner: "references/grove.md",
        phrase: "The first three **outlive the grove**",
        transient: &[],
    },
    Homed {
        rule: "plugin-prerequisite",
        owner: "references/grove.md",
        phrase: "each citation states what binds in the plugin's absence",
        transient: &[],
    },
    Homed {
        rule: "build-boundary-is-the-binary",
        owner: "references/grove.md",
        phrase: "changes nothing any session receives until the binary is rebuilt and installed",
        transient: &[],
    },
    // -- content/references/driver.md -------------------------------------
    Homed {
        rule: "pick-walk-order",
        owner: "references/driver.md",
        phrase: "first live leaf in a depth-first **pre-order** walk",
        transient: &[],
    },
    Homed {
        rule: "one-configuration",
        owner: "references/driver.md",
        phrase: "**Nothing else routes a session**",
        transient: &[],
    },
    Homed {
        rule: "restart-equals-continuation",
        owner: "references/driver.md",
        phrase: "**restart ≡ continuation**",
        transient: &[],
    },
    Homed {
        rule: "scaffold-is-the-drivers",
        owner: "references/driver.md",
        phrase: "a session **never scaffolds the tree itself**",
        transient: &[],
    },
    Homed {
        rule: "session-name-suggested-once",
        owner: "references/driver.md",
        phrase: "suggest `/rename",
        transient: &[],
    },
    Homed {
        rule: "migration-is-the-drivers",
        owner: "references/driver.md",
        phrase: "Neither conversion is yours to perform by hand",
        transient: &[],
    },
    // -- content/references/bootstrap.md ----------------------------------
    Homed {
        rule: "stale-launch-stops",
        owner: "references/bootstrap.md",
        phrase: "is a stale or hand-edited launch — not work to redo",
        transient: &[],
    },
    Homed {
        rule: "brief-chain-tolerates-gaps",
        owner: "references/bootstrap.md",
        phrase: "A level with no brief is skipped silently",
        transient: &[],
    },
];

/// Every swept file stating `phrase`, in path order. `SKILL.md` and the
/// unreachable file are out of the sweep; each has its own check.
fn sites(corpus: &BTreeMap<String, String>, phrase: &str) -> Vec<String> {
    let needle = normalised(phrase);
    corpus
        .iter()
        .filter(|(path, _)| path.as_str() != CONDITION_REGISTER && path.as_str() != UNREACHED)
        .filter(|(_, text)| text.contains(&needle))
        .map(|(path, _)| path.clone())
        .collect()
}

/// **The single-source claim itself**: each rule is stated by its owner, and by
/// no other procedure register except the transient sites it declares.
#[test]
fn every_rule_homed_here_is_stated_by_its_owner_and_by_no_other_procedure_register() {
    let corpus = corpus();

    for homed in HOMED {
        let mut expected = vec![homed.owner.to_owned()];
        expected.extend(homed.transient.iter().map(|path| (*path).to_owned()));
        expected.sort();

        assert_eq!(
            sites(&corpus, homed.phrase),
            expected,
            "{}: content/{} must state it, and no other procedure register may — declared \
             transient sites {:?}. A missing owner is a rule deleted by the rewrite that was \
             supposed to home it; an extra site is the duplication \
             docs/specs/corpus-rule-ownership.md exists to remove.",
            homed.rule,
            homed.owner,
            homed.transient,
        );
    }
}

/// **The condition register states no procedure.** `SKILL.md` carries the
/// situation and the owner's path; a rule's own wording appearing there is the
/// router re-growing the body it deferred.
#[test]
fn the_condition_register_states_none_of_these_procedures() {
    let corpus = corpus();
    let skill = corpus
        .get(CONDITION_REGISTER)
        .expect("the corpus carries content/SKILL.md");

    for homed in HOMED {
        assert!(
            !skill.contains(&normalised(homed.phrase)),
            "{}: content/SKILL.md must not state it — a trigger names the situation and the \
             file, never the procedure (docs/adr/restatement-declares-its-class.md)",
            homed.rule,
        );
    }
}

/// **The excluded file is excluded because nothing reaches it, and the exclusion
/// is doing real work.**
///
/// Both halves are asserted because either alone is worthless. Without the first,
/// the exclusion is a hand-waved convenience that could hide a live duplicate;
/// without the second, it could be excluding an empty file and every row above
/// would be passing for the wrong reason. `corpus-split-k6` deletes
/// `content/driving.md`, and this test goes with it.
#[test]
fn the_excluded_file_is_unreachable_and_the_exclusion_is_not_vacuous() {
    let corpus = corpus();
    assert!(
        corpus.contains_key(UNREACHED),
        "content/{UNREACHED} is gone, so this control and the exclusion in `sites` have both \
         outlived their reason — delete them"
    );

    for (path, text) in &corpus {
        if path == UNREACHED {
            continue;
        }
        assert!(
            !text.contains(UNREACHED),
            "content/{path} names content/{UNREACHED}, which puts it back on a loaded path — \
             the sweep above would then be excluding a file a session really opens"
        );
    }

    let duplicated = HOMED
        .iter()
        .filter(|homed| corpus[UNREACHED].contains(&normalised(homed.phrase)))
        .count();
    assert!(
        duplicated > 0,
        "content/{UNREACHED} states none of these rules, so excluding it proves nothing — \
         the sweep should be widened to it rather than left with a dead exclusion"
    );
}

// -- The three controls on the matcher ----------------------------------------

/// **Positive control**: the matcher finds a phrase known to be present, in the
/// file it is known to be in. A sweep that found nothing anywhere would satisfy
/// every absence check above and prove nothing.
#[test]
fn the_sweep_finds_a_phrase_known_to_be_present() {
    let corpus = corpus();
    assert_eq!(
        sites(&corpus, "Harvesting a done leaf"),
        vec!["references/retire.md".to_owned()],
        "the matcher must find a heading known to be in content/references/retire.md"
    );
}

/// **Normalisation control**: emphasis, code markers and a line wrap must not
/// hide a match, and normalising must not make unrelated prose match.
///
/// All three shapes are live in the corpus — it bolds mid-phrase, wraps at 80
/// columns, and writes `` `review-*` leaf `` with two markers inside one noun
/// phrase — so an unnormalised matcher reads clean while the rule is right there.
#[test]
fn the_sweep_normalises_emphasis_markers_and_line_wrapping() {
    let wrapped: BTreeMap<String, String> = [(
        "WRAPPED.md".to_owned(),
        normalised("Retire\n*before* you `commit`, so the **rename** lands in it."),
    )]
    .into_iter()
    .collect();
    assert_eq!(
        sites(&wrapped, "retire before you commit"),
        vec!["WRAPPED.md".to_owned()],
        "the matcher must see through emphasis, code markers and a line break"
    );

    let unrelated: BTreeMap<String, String> = [(
        "ABSENT.md".to_owned(),
        normalised("Retire the leaf, then commit it.\n"),
    )]
    .into_iter()
    .collect();
    assert!(
        sites(&unrelated, "retire before you commit").is_empty(),
        "normalising must not make unrelated prose match"
    );
}

/// **Cross-tree control**: the same matcher, pointed outside `content/`, still
/// finds the class where it legitimately lives — and finds it filed under the
/// *same owner*.
///
/// `docs/specs/corpus-rule-ownership.md` groups its inventory by canonical
/// source, so a rule id appearing under its owner's heading is the inventory
/// agreeing with the corpus this file just swept. That is what makes the sweep
/// evidence rather than a tautology over wording somebody chose twice: a rule
/// rehomed in `content/` without its row moving fails here, and so does a
/// renamed or deleted row, which would otherwise leave every assertion above
/// enforcing a rule the design no longer files that way.
#[test]
fn the_inventory_files_each_rule_under_the_owner_the_corpus_states_it_in() {
    let spec = normalised(include_str!("../docs/specs/corpus-rule-ownership.md"));

    for homed in HOMED {
        assert!(
            spec.contains(homed.rule),
            "{}: docs/specs/corpus-rule-ownership.md must carry the row this file claims to \
             enforce — a renamed row leaves the sweep enforcing nothing",
            homed.rule,
        );
        let group = group_for(&spec, homed.owner);
        assert!(
            group.contains(homed.rule),
            "{}: the inventory files it somewhere other than content/{}, which is where the \
             corpus states it. One of the two moved without the other",
            homed.rule,
            homed.owner,
        );
    }
}

/// The inventory rows grouped under `owner`'s heading, normalised. The inventory
/// is grouped by canonical source, so the heading is the owner's own path.
fn group_for(spec: &str, owner: &str) -> String {
    let heading = normalised(&format!("#### `content/{owner}`"));
    let from = spec
        .find(&heading)
        .unwrap_or_else(|| panic!("the inventory must group rows under content/{owner}"))
        + heading.len();
    let rest = &spec[from..];
    match rest.find("#### content/") {
        Some(next) => rest[..next].to_owned(),
        None => rest.to_owned(),
    }
}

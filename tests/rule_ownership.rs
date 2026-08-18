//! **One owner per rule, asserted over the corpus a session actually opens.**
//!
//! `docs/specs/corpus-rule-ownership.md` files every normative rule by *when a
//! session meets it*, and the property that falls out is single-source: a rule is
//! stated in one **procedure register** — one reference or format file — and
//! nowhere else. `content/SKILL.md` is the **condition register** and states
//! situations rather than procedures, so it is swept too, from the other
//! direction: none of the phrases below may appear there.
//!
//! Scope is every markdown file under `content/`. There is no longer an
//! excluded one: `driving.md` was the file no other corpus file named, so
//! nothing reached it and a rule surviving only there was already deleted in
//! effect. `corpus-split-k6` moved its surviving rules to the owners below and
//! deleted the file, which is what the relocation table in the spec existed to
//! reach.
//!
//! **The sweeps are phrase-scoped, and that bounds what they prove.** Each row
//! names its rule's canonical wording, normalised — emphasis and code markers
//! stripped, whitespace collapsed — because an unnormalised matcher silently
//! misses a wrapped or bolded occurrence, which is the fail-open direction.
//!
//! What a phrase sweep cannot see is a **paraphrase**: a second current-state
//! statement of the same rule, in other words, reads clean. While the rewrites
//! were in flight, each known duplicate carried a declaration of its own — the
//! same wording, or a second phrase pinning a paraphrase — so that none could be
//! removed silently and no declaration could outlive the duplication it excused.
//! Every one of them has now been removed with the section that carried it, so
//! the rows below claim single-source outright. **That is still not a
//! completeness claim.** Nothing here can prove no *undeclared* paraphrase
//! exists anywhere in the corpus — the blind spot
//! [`the_canonical_phrase_alone_is_blind_to_a_paraphrase`] exhibits rather than
//! papers over.
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
}

/// Every rule `loop-step-references-k11` landed in one of the seven loop-step
/// references, with the wording that identifies it.
const HOMED: &[Homed] = &[
    // -- content/references/execute.md ------------------------------------
    Homed {
        rule: "review-budget",
        owner: "references/execute.md",
        phrase: "at most one in-session reviewer across the whole picked leaf",
    },
    Homed {
        rule: "review-budget-predicate",
        owner: "references/execute.md",
        phrase: "adopted that mandate by running Bootstrap",
    },
    Homed {
        rule: "review-budget-by-kind",
        owner: "references/execute.md",
        phrase: "spend **none** — the pair supplies independent corpora",
    },
    Homed {
        rule: "doubt-pass-procedure",
        owner: "references/execute.md",
        phrase: "stripping the conclusion",
    },
    Homed {
        rule: "escalated-review-routes-through-config",
        owner: "references/execute.md",
        phrase: "Once review is escalated to the tree, grove owns the route",
    },
    Homed {
        rule: "verify-repo-claims-with-controls",
        owner: "references/execute.md",
        phrase: "clean-here plus dirty-there cannot be produced by a broken instrument",
    },
    Homed {
        rule: "enumerate-then-classify",
        owner: "references/execute.md",
        phrase: "Enumerate, then classify. Do not sweep a pattern list",
    },
    Homed {
        rule: "sweep-scope-is-the-claim",
        owner: "references/execute.md",
        phrase: "The sweep's scope is the claim's scope",
    },
    Homed {
        rule: "no-self-invalidating-count",
        owner: "references/execute.md",
        phrase: "Never document a claim with a count of itself",
    },
    Homed {
        rule: "check-the-rescued-clause",
        owner: "references/execute.md",
        phrase: "A clause rescued by its neighbour is not correct",
    },
    Homed {
        rule: "decisions-land-as-they-settle",
        owner: "references/execute.md",
        phrase: "Never reconstruct a session's decisions at the end of it",
    },
    Homed {
        rule: "escalation-names-the-tradeoff",
        owner: "references/execute.md",
        phrase: "name the trade-off you want input on",
    },
    // -- content/references/decompose.md ----------------------------------
    Homed {
        rule: "externalize-by-default",
        owner: "references/decompose.md",
        phrase: "the bar is *\"fits this session,\"* not \"I can finish it.\"",
    },
    Homed {
        rule: "bigger-than-brief-decomposes",
        owner: "references/decompose.md",
        phrase: "doing **only the first child** this session",
    },
    Homed {
        rule: "vertical-slice",
        owner: "references/decompose.md",
        phrase: "A good child leaf is a **vertical slice**",
    },
    Homed {
        rule: "wide-refactor-expand-contract",
        owner: "references/decompose.md",
        phrase: "Sequence it **expand → migrate → contract** instead, one leaf per stage",
    },
    Homed {
        rule: "fog-or-ticket",
        owner: "references/decompose.md",
        phrase: "a speculative leaf is the wrong shape for fog",
    },
    Homed {
        rule: "prior-art-research-is-its-own-leaf",
        owner: "references/decompose.md",
        phrase: "a research leaf of its own, cut ahead of the leaf that needs it",
    },
    Homed {
        rule: "research-brief-names-downstream-questions",
        owner: "references/decompose.md",
        phrase: "The research leaf's brief names the downstream questions",
    },
    Homed {
        rule: "review-chain-when-load-bearing",
        owner: "references/decompose.md",
        phrase: "artifact size and vendor preference are not the test",
    },
    Homed {
        rule: "vendor-pair-when-load-bearing",
        owner: "references/decompose.md",
        phrase: "is earned by a question load-bearing enough to pay for two corpora",
    },
    Homed {
        rule: "chain-is-lazy",
        owner: "references/decompose.md",
        phrase: "only if it has findings worth acting on",
    },
    Homed {
        rule: "pair-is-eager",
        owner: "references/decompose.md",
        phrase: "it lands in one call or not at all",
    },
    Homed {
        rule: "creating-session-writes-the-body",
        owner: "references/decompose.md",
        phrase: "The creating session writes the new leaf's body",
    },
    Homed {
        rule: "name-step-kind-off-the-producer",
        owner: "references/decompose.md",
        phrase: "Name a chain step's kind off the producer that actually ran",
    },
    Homed {
        rule: "steps-share-the-producers-stem",
        owner: "references/decompose.md",
        phrase: "Give every step of a composed shape the producer's bare stem as its whole slug",
    },
    Homed {
        rule: "diversity-is-the-configs",
        owner: "references/decompose.md",
        phrase: "Diversity is the configuration's, not the tree's",
    },
    Homed {
        rule: "integration-placement",
        owner: "references/decompose.md",
        phrase: "the first sibling entry after the review whose subtree still holds live work",
    },
    Homed {
        rule: "no-adjacency-exception",
        owner: "references/decompose.md",
        phrase: "A session that departs anyway owns the drift",
    },
    // -- content/references/retire.md -------------------------------------
    Homed {
        rule: "retirement-is-filename-only",
        owner: "references/retire.md",
        phrase: "Retirement touches one filename and nothing else",
    },
    Homed {
        rule: "finish-is-the-drivers-to-discover",
        owner: "references/retire.md",
        phrase: "You do not discover that a grove is finished",
    },
    Homed {
        rule: "triage-picks-the-verb",
        owner: "references/retire.md",
        phrase: "name which of the three sentences is actually true before reaching for the CLI",
    },
    Homed {
        rule: "no-fourth-status",
        owner: "references/retire.md",
        phrase: "There is no fourth state",
    },
    Homed {
        rule: "prune-scopes-to-the-whole-path",
        owner: "references/retire.md",
        phrase: "prune each of its live steps",
    },
    Homed {
        rule: "node-close-is-implicit",
        owner: "references/retire.md",
        phrase: "The close asks the human nothing",
    },
    // -- content/references/commit.md -------------------------------------
    Homed {
        rule: "one-focused-commit",
        owner: "references/commit.md",
        phrase: "One task is **one focused commit**",
    },
    Homed {
        rule: "name-by-handle",
        owner: "references/commit.md",
        phrase: "Name each node the cascade closed the same way",
    },
    Homed {
        rule: "no-kind-prefix-in-commit-subject",
        owner: "references/commit.md",
        phrase: "Do not compensate for the bare stem with a subject convention",
    },
    Homed {
        rule: "jj-seal",
        owner: "references/commit.md",
        phrase: "`jj new` after describing, once the rename has landed",
    },
    // -- content/references/grove.md --------------------------------------
    Homed {
        rule: "durable-artifact-set",
        owner: "references/grove.md",
        phrase: "The first three **outlive the grove**",
    },
    Homed {
        rule: "plugin-prerequisite",
        owner: "references/grove.md",
        phrase: "each citation states what binds in the plugin's absence",
    },
    Homed {
        rule: "build-boundary-is-the-binary",
        owner: "references/grove.md",
        phrase: "changes nothing any session receives until the binary is rebuilt and installed",
    },
    // -- content/references/driver.md -------------------------------------
    Homed {
        rule: "pick-walk-order",
        owner: "references/driver.md",
        phrase: "first live leaf in a depth-first **pre-order** walk",
    },
    Homed {
        rule: "one-configuration",
        owner: "references/driver.md",
        phrase: "**Nothing else routes a session**",
    },
    Homed {
        rule: "restart-equals-continuation",
        owner: "references/driver.md",
        phrase: "**restart ≡ continuation**",
    },
    Homed {
        rule: "scaffold-is-the-drivers",
        owner: "references/driver.md",
        phrase: "a session **never scaffolds the tree itself**",
    },
    Homed {
        rule: "session-name-suggested-once",
        owner: "references/driver.md",
        phrase: "suggest `/rename",
    },
    Homed {
        rule: "migration-is-the-drivers",
        owner: "references/driver.md",
        phrase: "Neither conversion is yours to perform by hand",
    },
    // -- content/references/bootstrap.md ----------------------------------
    Homed {
        rule: "stale-launch-stops",
        owner: "references/bootstrap.md",
        phrase: "is a stale or hand-edited launch — not work to redo",
    },
    Homed {
        rule: "brief-chain-tolerates-gaps",
        owner: "references/bootstrap.md",
        phrase: "A level with no brief is skipped silently",
    },
    // -- content/TASK-FORMAT.md -------------------------------------------
    Homed {
        rule: "leaf-name-grammar",
        owner: "TASK-FORMAT.md",
        phrase: "Five fields, all parsed",
    },
    Homed {
        rule: "convention-not-grammar",
        owner: "TASK-FORMAT.md",
        phrase: "What is convention rather than grammar",
    },
    Homed {
        rule: "body-carries-no-launch-metadata",
        owner: "TASK-FORMAT.md",
        phrase: "The body carries no launch metadata at all",
    },
    Homed {
        rule: "suggested-body-shape",
        owner: "TASK-FORMAT.md",
        // The running log's *section* is grammar and lives here; *when* to
        // append to it is `references/execute.md`'s, which is why the phrase
        // pinned is the section's introduction and not its name.
        phrase: "A sixth section appears when a session is keeping one",
    },
    // -- content/ADR-FORMAT.md --------------------------------------------
    Homed {
        rule: "adr-when-to-write",
        owner: "ADR-FORMAT.md",
        phrase: "All three, not any one",
    },
    Homed {
        rule: "adr-set-is-minimum-coherent",
        owner: "ADR-FORMAT.md",
        phrase: "never appends a superseding record",
    },
    Homed {
        rule: "research-to-adr-bridge",
        owner: "ADR-FORMAT.md",
        phrase: "bridge pointing both ways",
    },
    // -- content/SPEC-FORMAT.md -------------------------------------------
    Homed {
        rule: "spec-set-is-current-state",
        owner: "SPEC-FORMAT.md",
        phrase: "never append a superseding spec",
    },
    // -- content/CONTEXT-FORMAT.md ----------------------------------------
    Homed {
        rule: "glossary-is-the-forcing-function",
        owner: "CONTEXT-FORMAT.md",
        phrase: "Never batch a session's terms for the end",
    },
    Homed {
        rule: "challenge-and-sharpen-terms",
        owner: "CONTEXT-FORMAT.md",
        phrase: "Propose a precise canonical term for a fuzzy one",
    },
    // -- content/grilling.md ----------------------------------------------
    Homed {
        rule: "probe-with-concrete-scenarios",
        owner: "grilling.md",
        phrase: "stress-test them with specific scenarios",
    },
];

/// Every swept file stating `phrase`, in path order. `SKILL.md` is out of the
/// sweep and has its own check.
fn sites(corpus: &BTreeMap<String, String>, phrase: &str) -> Vec<String> {
    let needle = normalised(phrase);
    corpus
        .iter()
        .filter(|(path, _)| path.as_str() != CONDITION_REGISTER)
        .filter(|(_, text)| text.contains(&needle))
        .map(|(path, _)| path.clone())
        .collect()
}

/// **The single-source claim itself**: each rule is stated by its owner, and by
/// no other procedure register at all.
#[test]
fn every_rule_homed_here_is_stated_by_its_owner_and_by_no_other_procedure_register() {
    let corpus = corpus();

    for homed in HOMED {
        assert_eq!(
            sites(&corpus, homed.phrase),
            vec![homed.owner.to_owned()],
            "{}: content/{} must state it, and no other procedure register may. A missing \
             owner is a rule deleted by the rewrite that was supposed to home it; an extra \
             site is the duplication docs/specs/corpus-rule-ownership.md exists to remove.",
            homed.rule,
            homed.owner,
        );
    }
}

/// **The standing control on what the sweep cannot see.** A paraphrase is
/// invisible to the canonical phrase, so a clean sweep is evidence of
/// single-source only for the *wording* each row names.
///
/// This is the instrument's blind spot exhibited rather than papered over, and it
/// is why the rows above are read as "no file states this rule in these words"
/// rather than as "no file states this rule". Pointed at a corpus where a
/// non-owner restates a rule in other words, the sweep returns the owner alone —
/// a clean result, indistinguishable from single-source — while the second
/// wording finds the duplicate. The fixture is the corpus as it stood before
/// `corpus-split-k6` shed `TASK-FORMAT.md`'s doubt-budget table, which is the
/// real case this blindness was found on.
#[test]
fn the_canonical_phrase_alone_is_blind_to_a_paraphrase() {
    let canonical = "at most one in-session reviewer across the whole picked leaf";
    let paraphrase = "at most one; every independently materialised reviewer counts";

    let corpus: BTreeMap<String, String> = [
        (
            "references/execute.md".to_owned(),
            normalised(&format!(
                "A picked plain producer may materialise {canonical}."
            )),
        ),
        (
            "TASK-FORMAT.md".to_owned(),
            normalised(&format!("| plain producer | {paraphrase} |")),
        ),
    ]
    .into_iter()
    .collect();

    assert_eq!(
        sites(&corpus, canonical),
        vec!["references/execute.md".to_owned()],
        "the canonical-phrase sweep must read clean over the paraphrase — the blind spot \
         this control exists to exhibit"
    );
    assert_eq!(
        sites(&corpus, paraphrase),
        vec!["TASK-FORMAT.md".to_owned()],
        "and only the second wording finds the duplicate the canonical sweep missed"
    );
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

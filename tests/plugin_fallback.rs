//! **Grove does not depend on the `linkuistics` plugin silently.**
//!
//! Grove requires the plugin and does not provision it — the `grove` binary
//! sweeps its own methodology and nothing else — so a checkout without it reads
//! every `linkuistics:` citation as a pointer at a skill that is not there. The
//! rule `plugin-prerequisite` (`docs/specs/corpus-rule-ownership.md`, *Plugin
//! deferral policy*) closes that by requiring each citation to state **what binds
//! in the plugin's absence**, and the generating question is per citation: does
//! the absence change *what* a session writes, or only *how well*? Absence that
//! changes *what* is owned locally; absence that changes *how well* is deferred,
//! and the deferring sentence says so.
//!
//! **The registry below is the audit, and the sweep is what keeps it honest.**
//! [`CITATIONS`] names every citation site with the wording that discharges it;
//! [`the_citation_registry_is_exhaustive_over_the_corpus`] sweeps the embedded
//! corpus for `linkuistics:<skill>` and asserts the found set is exactly the
//! registry's. A citation added later without a binding sentence therefore fails
//! here rather than reaching a session — which is the fail-closed direction, and
//! the one a per-row presence check alone would miss.
//!
//! Three controls stand over the sweep, matching what the spec asks of an **S**
//! row: a positive control that a known citation is found, a normalisation
//! control that markup and 80-column wrapping do not hide one, and a negative
//! control that an unregistered citation in a fabricated corpus is caught.
//!
//! [`an_ordinary_task_completes_without_reading_a_plugin_skill`] is the
//! acceptance test the brief names: bootstrap → produce → retire → commit →
//! signal, with the grove-local statement each step needs asserted present in the
//! file that step opens.

use grove::methodology;
use std::collections::{BTreeMap, BTreeSet};

/// Emphasis and code markers stripped, whitespace collapsed — the same
/// normalisation `tests/rule_ownership.rs` uses, and for the same reason: the
/// corpus wraps at 80 columns and marks up mid-phrase, so a raw `contains`
/// misses real occurrences and reads as a clean sweep.
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

/// One citation of a plugin skill, and the wording that discharges it.
struct Citation {
    /// The corpus file carrying the citation, relative to `content/`.
    file: &'static str,
    /// The plugin skill cited, without the `linkuistics:` prefix.
    skill: &'static str,
    /// The local sentence stating what binds without the plugin. Its presence is
    /// what makes the citation attribution rather than a dependency.
    binds: &'static str,
}

/// **The audit, re-derived against the corpus as it now stands** rather than
/// carried over from the design's table. Four of the design's fourteen rows —
/// `references/execute.md`, `references/retire.md` and `grilling.md` twice —
/// discharged themselves when `loop-step-references-k11` and `corpus-split-k6`
/// repointed those citations at `ADR-FORMAT.md` and `references/requirements.md`;
/// the files no longer name the plugin at all, which is why they have no row.
///
/// `references/grove.md` is the **hub**: it states what binds for all three
/// skills in one place, and `SKILL.md`'s trigger sentence 19 routes every
/// citation there ([`the_hub_is_reachable_from_the_condition_register`]). That is
/// why no other file mirrors the seam gloss — a second statement of it would be
/// the duplication `docs/specs/corpus-rule-ownership.md` exists to remove.
const CITATIONS: &[Citation] = &[
    Citation {
        file: "ADR-FORMAT.md",
        skill: "decision-records",
        binds: "what binds without it is stated here",
    },
    Citation {
        file: "SPEC-FORMAT.md",
        skill: "codebase-design",
        binds: "those three rules are grove's own and bind without the plugin",
    },
    Citation {
        file: "references/commit.md",
        skill: "using-jujutsu",
        binds: "the boundary above is the whole of what a grove session needs, \
                and it binds on its own",
    },
    Citation {
        file: "references/grove.md",
        skill: "decision-records",
        binds: "what binds without it is adr-format.md",
    },
    Citation {
        file: "references/grove.md",
        skill: "codebase-design",
        binds: "are in spec-format.md and bind without the skill",
    },
    Citation {
        file: "references/grove.md",
        skill: "using-jujutsu",
        binds: "what binds without it is references/commit.md",
    },
];

/// Every `linkuistics:<skill>` citation in `text`, as skill names. Scanned over
/// the **normalised** text so that markup and a wrap between the marker and the
/// skill name cannot hide one — the fail-open direction this sweep exists to
/// close.
fn cited_skills(text: &str) -> BTreeSet<String> {
    const MARKER: &str = "linkuistics:";
    let mut found = BTreeSet::new();
    let mut rest = text;
    while let Some(offset) = rest.find(MARKER) {
        rest = &rest[offset + MARKER.len()..];
        let name: String = rest
            .chars()
            .take_while(|character| character.is_ascii_alphanumeric() || *character == '-')
            .collect();
        if !name.is_empty() {
            found.insert(name);
        }
    }
    found
}

/// Every `(file, skill)` citation pair the corpus actually carries.
fn citation_pairs(corpus: &BTreeMap<String, String>) -> BTreeSet<(String, String)> {
    corpus
        .iter()
        .flat_map(|(path, text)| {
            cited_skills(text)
                .into_iter()
                .map(move |skill| (path.clone(), skill))
        })
        .collect()
}

/// **The discharge test.** Every deferring sentence states what binds without the
/// plugin; a citation that leaves that unstated is the silent dependency,
/// whichever way the row was decided.
#[test]
fn every_plugin_citation_states_what_binds_without_it() {
    let corpus = corpus();

    for citation in CITATIONS {
        let text = corpus
            .get(citation.file)
            .unwrap_or_else(|| panic!("content/{} must be in the embed", citation.file));

        assert!(
            text.contains(&format!("linkuistics:{}", citation.skill)),
            "content/{} must still cite linkuistics:{} — a row whose citation is gone is a \
             stale audit, not a discharged one",
            citation.file,
            citation.skill,
        );
        assert!(
            text.contains(&normalised(citation.binds)),
            "content/{} cites linkuistics:{} without stating what binds in its absence. \
             Expected: {:?}",
            citation.file,
            citation.skill,
            citation.binds,
        );
    }
}

/// **The fail-closed half.** A per-row presence check cannot see a *new*
/// citation, so the registry is asserted exhaustive: the pairs the corpus carries
/// are exactly the pairs audited above. Adding a citation without writing its
/// binding sentence fails here.
#[test]
fn the_citation_registry_is_exhaustive_over_the_corpus() {
    let found = citation_pairs(&corpus());
    let audited: BTreeSet<(String, String)> = CITATIONS
        .iter()
        .map(|citation| (citation.file.to_owned(), citation.skill.to_owned()))
        .collect();

    assert_eq!(
        found, audited,
        "every plugin citation must be audited in CITATIONS. An extra pair is an \
         unaudited deferral reaching sessions; a missing one is a row the corpus no \
         longer carries."
    );

    // Positive control: the sweep finds something, so an empty-corpus or
    // broken-marker failure cannot pass as a clean audit.
    assert_eq!(
        found.len(),
        6,
        "the corpus carries six audited citations across four files: {found:?}"
    );
}

/// Negative control on the exhaustiveness sweep: pointed at a corpus carrying an
/// unregistered citation, it reports the pair rather than reading clean.
#[test]
fn an_unregistered_citation_is_caught() {
    let fabricated: BTreeMap<String, String> = [(
        "references/impl.md".to_owned(),
        normalised("Judge the seam with the `linkuistics:codebase-design` skill."),
    )]
    .into_iter()
    .collect();

    assert_eq!(
        citation_pairs(&fabricated),
        [(
            "references/impl.md".to_owned(),
            "codebase-design".to_owned()
        )]
        .into_iter()
        .collect::<BTreeSet<_>>(),
        "an unaudited citation must be visible to the sweep, or the registry proves nothing"
    );
}

/// Normalisation control: a citation split across an 80-column wrap and wrapped
/// in code markers is still found. An unnormalised matcher misses both, and
/// misses them *silently*.
#[test]
fn the_sweep_survives_markup_and_wrapping() {
    let wrapped = normalised(
        "The lane itself belongs to\n`linkuistics:using-jujutsu`; grove states where it falls.",
    );

    assert!(
        cited_skills(&wrapped).contains("using-jujutsu"),
        "markup and wrapping must not hide a citation: {wrapped:?}"
    );
}

/// The hub only discharges the citations that route to it, so both ends are
/// asserted: `SKILL.md` carries the condition, and `references/grove.md` carries
/// the statement it names.
#[test]
fn the_hub_is_reachable_from_the_condition_register() {
    let corpus = corpus();
    let register = corpus.get("SKILL.md").expect("SKILL.md is in the embed");
    let hub = corpus
        .get("references/grove.md")
        .expect("references/grove.md is in the embed");

    assert!(
        register.contains(&normalised(
            "When a rule cites the `linkuistics` plugin, read what binds without it in \
             `references/grove.md`."
        )),
        "the condition register must route a plugin citation to the hub"
    );
    assert!(
        hub.contains(&normalised(
            "each citation states what binds in the plugin's absence"
        )),
        "the hub must carry `plugin-prerequisite` itself"
    );
    assert!(
        hub.contains(&normalised("grove requires it and does not provision it")),
        "the hub must state the prerequisite it is standing in for"
    );
}

/// **The acceptance test.** A session in a checkout without the plugin runs an
/// ordinary task end to end — bootstrap, produce, retire, commit, signal —
/// reading no plugin skill. Each row is the grove-local statement one step needs,
/// asserted present in the file that step opens.
///
/// Commit is the row whose absence is unrecoverable: a session that commits with
/// git in a jj tree bypasses the operation log, and a session that cannot commit
/// cannot end. So the boundary itself is asserted, not just the citation.
#[test]
fn an_ordinary_task_completes_without_reading_a_plugin_skill() {
    let corpus = corpus();
    let text = |path: &str| {
        corpus
            .get(path)
            .unwrap_or_else(|| panic!("content/{path} must be in the embed"))
            .clone()
    };

    for (step, file, statement) in [
        // Bootstrap — the read order is grove's own and names no skill.
        (
            "bootstrap",
            "references/bootstrap.md",
            "grove-llm brief-chain",
        ),
        // Produce — the ADR when-to-write test is stated locally, in AND form.
        (
            "produce (adr)",
            "ADR-FORMAT.md",
            "write one only when all three hold",
        ),
        // Produce — the operative seam rules are grove's own.
        (
            "produce (seams)",
            "SPEC-FORMAT.md",
            "prefer existing seams to new ones",
        ),
        // Retire — never append a superseding record.
        (
            "retire",
            "references/retire.md",
            "adr-format.md carries how a set is reworked",
        ),
        // Commit — complete without the plugin.
        (
            "commit (jj)",
            "references/commit.md",
            "jj new after describing, once the rename has landed",
        ),
        (
            "commit (sufficiency)",
            "references/commit.md",
            "a checkout without the plugin commits correctly from this file alone",
        ),
    ] {
        assert!(
            text(file).contains(&normalised(statement)),
            "{step}: content/{file} must bind on its own without the plugin. Expected: \
             {statement:?}"
        );
    }

    // The steps that must name no skill at all: reading one there would be a
    // dependency no binding sentence could discharge, because the session is
    // past the point of choosing.
    for path in [
        "references/bootstrap.md",
        "references/retire.md",
        "SIGNAL.md",
        "SIGNAL-FINISH.md",
    ] {
        assert!(
            cited_skills(&text(path)).is_empty(),
            "content/{path} must cite no plugin skill: {:?}",
            cited_skills(&text(path))
        );
    }
}

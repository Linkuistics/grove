//! **Grove does not depend on a craft skill silently.**
//!
//! Grove's methodology cites craft skills that it does not ship, and an
//! installation can omit any of them. A machine without one reads its citation
//! as a pointer at a skill that is not there. The rule `plugin-prerequisite`
//! (`plugins/grove/conformance/rules.tsv`) closes that by requiring each citation
//! to state **what binds in the skill's absence**, and the generating question
//! is per citation: does the absence change *what* a session writes, or only *how
//! well*? Absence that changes *what* is owned locally; absence that changes *how
//! well* is deferred, and the deferring sentence says so.
//!
//! **The subject moved at `delete-provisioning-k19`, and the claims did not.**
//! This swept the corpus the binary embedded; the same corpus is now the shipped
//! `grove` plugin, and the sweep reads it off disk. Nothing about the audit
//! changes, because a citation is a citation wherever it is delivered from.
//!
//! **A citation names its skill bare, as "the `<name>` skill".** Until
//! `core-bare-references-k170` a citation wrote the plugin alias
//! `linkuistics:<name>`, and the sweep found citations by that marker. The craft
//! skills then leave that plugin for three packs, so rule 2 of "References
//! between skills" in `docs/specs/external-pack-contract.md` removed the alias.
//! The sweep now finds each code span followed by the word "skill" and keeps
//! the names that core does not ship.
//!
//! **The registry below is the audit, and the sweep is what keeps it honest.**
//! [`CITATIONS`] names every citation site with the wording that discharges it;
//! [`the_citation_registry_is_exhaustive_over_the_corpus`] sweeps the shipped
//! skill set and asserts the found set is exactly the registry's. A citation
//! added later without a binding sentence therefore fails here rather than
//! reaching a session — which is the fail-closed direction, and the one a
//! per-row presence check alone would miss.
//!
//! [`no_skill_file_writes_a_plugin_qualified_name`] keeps the alias out. It
//! enumerates every `<word>:<word>` token in the shipped markdown and classifies
//! each one, so a qualified name under any plugin fails, not only a known one.
//!
//! Controls stand over both sweeps: a positive control that the known citations
//! and the retained openings are found, a normalisation control that markup and
//! 80-column wrapping do not hide one, and negative controls that an
//! unregistered citation and a qualified name in a fabricated corpus are caught.
//!
//! [`an_ordinary_task_completes_without_reading_a_plugin_skill`] is the
//! acceptance test the brief names: bootstrap → produce → retire → commit →
//! signal, with the grove-local statement each step needs asserted present in the
//! file that step opens.

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

/// Emphasis and code markers stripped, whitespace collapsed — the same
/// normalisation `plugins/grove/conformance.sh` uses, and for the same reason: the
/// corpus wraps at 80 columns and marks up mid-phrase, so a raw `contains`
/// misses real occurrences and reads as a clean sweep.
fn normalised(text: &str) -> String {
    marked(text).replace('`', "")
}

/// [`normalised`], except that code markers stay. A bare citation is "the
/// `<name>` skill", and only the code span tells a skill name from prose.
fn marked(text: &str) -> String {
    let stripped: String = text
        .chars()
        .filter(|character| !matches!(character, '*' | '_'))
        .flat_map(char::to_lowercase)
        .collect();
    stripped.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn skills_root() -> PathBuf {
    support::repo_root().join("plugins/grove/skills")
}

/// The shipped skill set, [`marked`], as `skills/`-relative path→text.
fn corpus() -> BTreeMap<String, String> {
    let root = skills_root();
    let mut out = BTreeMap::new();
    collect(&root, &root, &mut out);
    assert!(
        !out.is_empty(),
        "no markdown under {} — a mis-scoped walk audits nothing",
        root.display()
    );
    out
}

/// The skills that core ships: one directory each under `plugins/grove/skills/`.
/// A name in this set is a load or a mention inside core, not a citation.
fn shipped_skills() -> BTreeSet<String> {
    std::fs::read_dir(skills_root())
        .expect("the shipped skill set must be readable")
        .map(|entry| entry.expect("a readable directory entry").path())
        .filter(|path| path.join("SKILL.md").is_file())
        .map(|path| {
            path.file_name()
                .expect("a skill directory has a name")
                .to_string_lossy()
                .into_owned()
        })
        .collect()
}

fn collect(root: &Path, dir: &PathBuf, out: &mut BTreeMap<String, String>) {
    for entry in std::fs::read_dir(dir).expect("the shipped skill set must be readable") {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            collect(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).expect("a path under the root");
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{} must be readable: {error}", path.display()));
            out.insert(relative.to_string_lossy().into_owned(), marked(&text));
        }
    }
}

/// One citation of a plugin skill, and the wording that discharges it.
struct Citation {
    /// The file carrying the citation, relative to `plugins/grove/skills/`.
    file: &'static str,
    /// The craft skill cited, by its bare name.
    skill: &'static str,
    /// The local sentence stating what binds without the skill. Its presence is
    /// what makes the citation attribution rather than a dependency.
    binds: &'static str,
}

/// **The audit, re-derived against the corpus as it now stands** rather than
/// carried over from the design's table. Four of the design's fourteen rows —
/// `references/execute.md`, `references/retire.md` and `grilling.md` twice —
/// discharged themselves when `loop-step-references-k11` and `corpus-split-k6`
/// repointed those citations at `ADR-FORMAT.md` and `references/requirements.md`;
/// the files no longer name the plugin at all, which is why they have no row.
/// `grilling.md` has since moved to the RequirementsSkills pack, outside this
/// corpus.
///
/// The registry covers core alone. `grove-design/SKILL.md` moved to the
/// DesignSkills pack with its citation of `codebase-design`, and that pack's
/// manifest pins the citation's binding sentence as the row
/// `design-cites-codebase-design-with-fallback`. Each pack pins its own
/// citations, and no sweep here reads a pack.
///
/// `grove/references/grove.md` is the **hub**: it states what binds for every
/// cited skill in one place, and the spine's `SKILL.md` routes every
/// citation there ([`the_hub_is_reachable_from_the_condition_register`]). That is
/// why no other file mirrors the seam gloss — a second statement of it would be
/// the duplication the ownership inventory exists to remove.
const CITATIONS: &[Citation] = &[
    Citation {
        file: "grove/ADR-FORMAT.md",
        skill: "decision-records",
        binds: "what binds without it is stated here",
    },
    Citation {
        file: "grove/SPEC-FORMAT.md",
        skill: "codebase-design",
        binds: "those three rules are grove's own and bind without the `codebase-design` skill",
    },
    Citation {
        file: "grove/references/commit.md",
        skill: "using-jujutsu",
        binds: "the boundary above is the whole of what a grove session needs, \
                and it binds on its own",
    },
    Citation {
        file: "grove/references/execute.md",
        skill: "machine-environment",
        binds: "the local task and environment rules below bind when the skill is absent",
    },
    Citation {
        file: "grove/references/grove.md",
        skill: "decision-records",
        binds: "what binds without it is adr-format.md",
    },
    Citation {
        file: "grove/references/grove.md",
        skill: "codebase-design",
        binds: "are in spec-format.md and bind without the skill",
    },
    Citation {
        file: "grove/references/grove.md",
        skill: "using-jujutsu",
        binds: "what binds without it is references/commit.md",
    },
    Citation {
        file: "grove/references/grove.md",
        skill: "technical-communication",
        binds: "that section says what to explain and in what order, and it stands on its own",
    },
    Citation {
        file: "grove/references/grove.md",
        skill: "machine-environment",
        binds: "what binds without it is references/execute.md",
    },
    Citation {
        file: "grove/SKILL.md",
        skill: "technical-communication",
        binds: "without the skill, this section binds as it stands",
    },
    Citation {
        file: "grove/references/review.md",
        skill: "codebase-design",
        binds: "grove's procedure and artifact requirements bind when the skill is absent",
    },
];

/// Every craft skill that `text` cites, as bare names. A citation is a code span
/// that holds one skill name and is followed by the word "skill", as in "the
/// `codebase-design` skill". A name in `shipped` is core's own and is no
/// citation. The scan reads [`marked`] text, so that markup and a wrap between
/// the span and the word cannot hide one — the fail-open direction this sweep
/// exists to close.
fn cited_skills(text: &str, shipped: &BTreeSet<String>) -> BTreeSet<String> {
    let mut found = BTreeSet::new();
    let mut spans = text.split('`');
    // The pieces alternate: outside a span, inside a span, outside, and so on.
    spans.next();
    while let (Some(inside), Some(after)) = (spans.next(), spans.next()) {
        let is_name = !inside.is_empty()
            && inside.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
            });
        if is_name && after.starts_with(" skill") && !shipped.contains(inside) {
            found.insert(inside.to_owned());
        }
    }
    found
}

/// Every `(file, skill)` citation pair the corpus actually carries.
fn citation_pairs(
    corpus: &BTreeMap<String, String>,
    shipped: &BTreeSet<String>,
) -> BTreeSet<(String, String)> {
    corpus
        .iter()
        .flat_map(|(path, text)| {
            cited_skills(text, shipped)
                .into_iter()
                .map(move |skill| (path.clone(), skill))
        })
        .collect()
}

/// **The discharge test.** Every deferring sentence states what binds without the
/// skill; a citation that leaves that unstated is the silent dependency,
/// whichever way the row was decided.
#[test]
fn every_plugin_citation_states_what_binds_without_it() {
    let corpus = corpus();

    for citation in CITATIONS {
        let text = corpus
            .get(citation.file)
            .unwrap_or_else(|| panic!("{} must be a shipped skill file", citation.file));

        assert!(
            text.contains(&format!("`{}` skill", citation.skill)),
            "{} must still cite the `{}` skill — a row whose citation is gone is a \
             stale audit, not a discharged one",
            citation.file,
            citation.skill,
        );
        assert!(
            normalised(text).contains(&normalised(citation.binds)),
            "{} cites the `{}` skill without stating what binds in its absence. \
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
    let found = citation_pairs(&corpus(), &shipped_skills());
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
        11,
        "the shipped skill set carries eleven audited citations across seven files: {found:?}"
    );
}

/// Negative control on the exhaustiveness sweep: pointed at a corpus carrying an
/// unregistered citation, it reports the pair rather than reading clean.
#[test]
fn an_unregistered_citation_is_caught() {
    let fabricated: BTreeMap<String, String> = [(
        "grove-impl/SKILL.md".to_owned(),
        marked("Judge the seam with the `codebase-design` skill. Load the `grove` skill."),
    )]
    .into_iter()
    .collect();

    assert_eq!(
        citation_pairs(&fabricated, &shipped_skills()),
        [(
            "grove-impl/SKILL.md".to_owned(),
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
    let wrapped = marked(
        "The lane itself belongs to the\n**`using-jujutsu`**\nskill; grove states where it falls.",
    );

    assert!(
        cited_skills(&wrapped, &shipped_skills()).contains("using-jujutsu"),
        "markup and wrapping must not hide a citation: {wrapped:?}"
    );
}

/// The hub only discharges the citations that route to it, so both ends are
/// asserted: `SKILL.md` carries the condition, and `references/grove.md` carries
/// the statement it names.
#[test]
fn the_hub_is_reachable_from_the_condition_register() {
    let corpus = corpus();
    let register = corpus
        .get("grove/SKILL.md")
        .expect("the spine ships a SKILL.md");
    let hub = corpus
        .get("grove/references/grove.md")
        .expect("the spine ships references/grove.md");

    assert!(
        normalised(register).contains(&normalised(
            "When a rule cites a craft skill, read what binds without it in \
             `references/grove.md`."
        )),
        "the condition register must route a craft-skill citation to the hub"
    );
    assert!(
        normalised(hub).contains(&normalised(
            "each citation therefore states what binds in the skill's absence"
        )),
        "the hub must carry `plugin-prerequisite` itself"
    );
    assert!(
        normalised(hub).contains(&normalised(
            "grove does not ship these skills, and an installation can omit any of them"
        )),
        "the hub must state the absence it is standing in for"
    );
    // A later pack move would make a named plugin or pack false, so the hub
    // names none.
    for stale in ["linkuistics", "plugin", "pack"] {
        let section = &normalised(hub)[normalised(hub)
            .find("what the cited craft skills carry")
            .expect("the hub keeps its craft-skill section")..];
        assert!(
            !section.contains(stale),
            "the hub's craft-skill section must name no plugin or pack: {stale:?}"
        );
    }
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
    let shipped = shipped_skills();
    let text = |path: &str| {
        corpus
            .get(path)
            .unwrap_or_else(|| panic!("{path} must be a shipped skill file"))
            .clone()
    };

    for (step, file, statement) in [
        // Bootstrap — the read order is grove's own and names no skill.
        (
            "bootstrap",
            "grove/references/bootstrap.md",
            "grove-llm brief-chain",
        ),
        // Produce — the ADR when-to-write test is stated locally, in AND form.
        (
            "produce (adr)",
            "grove/ADR-FORMAT.md",
            "write one only when all three hold",
        ),
        // Produce — the operative seam rules are grove's own.
        (
            "produce (seams)",
            "grove/SPEC-FORMAT.md",
            "prefer existing seams to new ones",
        ),
        // Retire — never append a superseding record.
        (
            "retire",
            "grove/references/retire.md",
            "adr-format.md carries how a set is reworked",
        ),
        // Commit — complete without the plugin.
        (
            "commit (jj)",
            "grove/references/commit.md",
            "jj new after describing, once the rename has landed",
        ),
        (
            "commit (sufficiency)",
            "grove/references/commit.md",
            "a checkout without that skill commits correctly from this file alone",
        ),
    ] {
        assert!(
            normalised(&text(file)).contains(&normalised(statement)),
            "{step}: {file} must bind on its own without the plugin. Expected: \
             {statement:?}"
        );
    }

    // The steps that must name no skill at all: reading one there would be a
    // dependency no binding sentence could discharge, because the session is
    // past the point of choosing.
    // The two corpus signal files this list used to end on are gone: grove states
    // its own signalling contract in `${prompt}` now, and `finish`'s three
    // endings are inline in its skill, which is why that skill takes their place
    // here.
    for path in [
        "grove/references/bootstrap.md",
        "grove/references/retire.md",
        "grove-finish/SKILL.md",
    ] {
        assert!(
            cited_skills(&text(path), &shipped).is_empty(),
            "{path} must cite no craft skill: {:?}",
            cited_skills(&text(path), &shipped)
        );
    }
}

/// The kinds that leave core in a later move. Each keeps its `grove:grove`
/// opening until its own move changes those bytes, as the external pack
/// contract assigns. No other file may write a plugin-qualified name.
const MOVING_KIND_OPENINGS: &[&str] = &[
    "grove-art/SKILL.md",
    "grove-copy-edit/SKILL.md",
    "grove-draft/SKILL.md",
    "grove-impl/SKILL.md",
    "grove-integrate-review-impl/SKILL.md",
    "grove-integrate-review-prototype/SKILL.md",
    "grove-proof/SKILL.md",
    "grove-prototype/SKILL.md",
    "grove-review-impl/SKILL.md",
    "grove-review-prototype/SKILL.md",
];

/// Colon-joined tokens that name no skill: the Taskfile target in
/// `configure-grove`, and the `path:line` coordinate form of a finding.
const NOT_SKILL_NAMES: &[&str] = &["ravel:review", "path:line"];

/// Every `<word>:<word>` token in `text`, where a word is lower-case letters,
/// digits and hyphens. A URL scheme is followed by `/`, so it yields no token.
fn colon_tokens(text: &str) -> Vec<String> {
    let is_word = |character: char| {
        character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
    };
    let characters: Vec<char> = text.chars().collect();
    let mut tokens = Vec::new();
    for (index, character) in characters.iter().enumerate() {
        if *character != ':' {
            continue;
        }
        let mut start = index;
        while start > 0 && is_word(characters[start - 1]) {
            start -= 1;
        }
        let mut end = index + 1;
        while end < characters.len() && is_word(characters[end]) {
            end += 1;
        }
        if start < index && index + 1 < end {
            tokens.push(characters[start..end].iter().collect());
        }
    }
    tokens
}

/// The tokens of `corpus` that are neither a retained opening nor a known
/// non-skill token, as `(file, token)` pairs.
fn unclassified_tokens(corpus: &BTreeMap<String, String>) -> BTreeSet<(String, String)> {
    corpus
        .iter()
        .flat_map(|(path, text)| {
            colon_tokens(text).into_iter().filter_map(move |token| {
                let retained =
                    token == "grove:grove" && MOVING_KIND_OPENINGS.contains(&path.as_str());
                let not_a_skill = NOT_SKILL_NAMES.contains(&token.as_str());
                (!retained && !not_a_skill).then(|| (path.clone(), token))
            })
        })
        .collect()
}

/// **Enumerate, then classify.** Rule 2 of "References between skills" bars a
/// plugin-qualified alias from pack text. A sweep for known prefixes misses an
/// alias under a new plugin, so every colon-joined token is classified instead.
#[test]
fn no_skill_file_writes_a_plugin_qualified_name() {
    let corpus = corpus();

    assert_eq!(
        unclassified_tokens(&corpus),
        BTreeSet::new(),
        "a shipped skill file writes a plugin-qualified name or an unclassified \
         `<word>:<word>` token. Name the skill bare, or classify the token here."
    );

    // Positive control: the enumeration finds each retained opening, so a broken
    // scanner cannot read as a clean corpus.
    for path in MOVING_KIND_OPENINGS {
        let text = corpus
            .get(*path)
            .unwrap_or_else(|| panic!("{path} must be a shipped skill file"));
        assert!(
            colon_tokens(text).contains(&"grove:grove".to_owned()),
            "{path} is listed as a retained opening and must still hold one; a moved \
             kind leaves this list in the same change"
        );
    }

    // The kinds that stay in core open with the bare name.
    for path in [
        "grove-planning/SKILL.md",
        "grove-review-planning/SKILL.md",
        "grove-integrate-review-planning/SKILL.md",
        "grove-finish/SKILL.md",
    ] {
        let text = normalised(
            corpus
                .get(path)
                .unwrap_or_else(|| panic!("{path} must be a shipped skill file")),
        );
        assert!(
            text.contains(&normalised(
                "grove is the bare skill name. If your harness requires a plugin \
                 namespace, use the namespace under which grove is actually installed. \
                 Do not assume one. If you cannot find or read the grove skill, follow \
                 the launch prompt's rule for a missing kind skill."
            )),
            "{path} must open with the contract's bare-name text"
        );
    }
}

/// Negative control on the enumeration: a qualified name under a plugin that no
/// list knows is caught, and so is the spine's alias outside a retained opening.
#[test]
fn a_qualified_name_is_caught() {
    let fabricated: BTreeMap<String, String> = [
        (
            "grove/SKILL.md".to_owned(),
            marked("Use the `newpack:codebase-design` skill, and see https://example.com/a."),
        ),
        (
            "grove-planning/SKILL.md".to_owned(),
            marked("Load the `grove` skill now, that is `grove:grove`."),
        ),
    ]
    .into_iter()
    .collect();

    assert_eq!(
        unclassified_tokens(&fabricated),
        [
            (
                "grove-planning/SKILL.md".to_owned(),
                "grove:grove".to_owned()
            ),
            (
                "grove/SKILL.md".to_owned(),
                "newpack:codebase-design".to_owned()
            ),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>(),
        "the enumeration must report each qualified name and no URL"
    );
}

//! No documentation surface still describes the removed launch-policy system as
//! current — asserted by **enumeration**, not by a list of phrases to look for.
//!
//! The redesign deleted a whole vocabulary: lifecycle subcommands (`grove do`,
//! `grove migrate`, `grove retire`), routing environment, task-body `**Kind:**`
//! / `**Harness:**` / `**Producer launch:**` fields, the harness stamp, and the
//! review-diversity warning. Prose is where those survive longest, because
//! nothing compiles it. The obvious check — grep for each phrase and assert
//! zero — cannot be written at all here: the surviving occurrences are
//! *deliberate*. A spec that says "`GROVE_SESSION_TARGET` has no successor" and
//! a usage guide that says "Grove emits no diversity warnings" both have to name
//! the thing they deny.
//!
//! So the candidates come from the tree and the **classification** is what is
//! committed. Every occurrence of every token in [`LEGACY_TOKENS`] across the
//! documentation trees must fall inside a [`REFUTATIONS`] quotation — a
//! committed judgement that *this sentence names the token in order to record
//! its removal*. The unit is the **occurrence**, not the file: a surface that
//! already denies a token stays answerable for a second, affirmative sentence
//! about that same token, which a file-wide exemption waved through. An
//! occurrence nobody quoted fails with its file and line, so a reintroduced
//! affirmative claim is caught whether or not anyone remembered to add it here.
//! The check runs in both directions: every quotation must still occur and must
//! still name a legacy token, so an entry whose subject is gone fails too and
//! the table cannot quietly become a fossil.
//!
//! Controls, because a sweep that cannot fail is worth nothing. The **positive
//! control** drives the same classifier over synthetic documents and shows an
//! affirmative legacy claim rejected — including the case a file-wide exemption
//! used to hide, where the denial and the affirmation name the same token in
//! the same file. The **cross-tree control**
//! asserts the walk actually reached every documentation tree it claims to
//! cover — repository root, `docs/`, `content/` and `plugins/` are four
//! separately owned trees, and a sweep that silently walked one of them would
//! pass while checking almost nothing. A third pins the matcher against the
//! false negative that a wrapped paragraph produces.
//!
//! Out of the roots by ownership, not by oversight:
//!
//! - `CHANGELOG.md` keeps the past by contract. Its entries described the
//!   system as it stood when they were written, and rewriting them would
//!   destroy the only record of the removal.
//! - `.grove/` is process state the finish cycle deletes.
//! - `src/` and `tests/` name these fields because the **migration path has to
//!   recognise them to strip them** — an occurrence there is machinery, not a
//!   claim. `tests/removed_surface.rs` owns the environment-variable half of
//!   that tree, by its own enumeration.

use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Scope

/// A separately owned documentation tree. `recursive` is false for the
/// repository root so the walk cannot wander into `target/`, `.jj/` or
/// `.grove/` — the root's own Markdown is the surface, not everything beneath
/// it.
struct DocumentationTree {
    name: &'static str,
    root: &'static str,
    recursive: bool,
}

const DOCUMENTATION_TREES: &[DocumentationTree] = &[
    DocumentationTree {
        name: "repository root",
        root: "",
        recursive: false,
    },
    DocumentationTree {
        name: "docs",
        root: "docs",
        recursive: true,
    },
    DocumentationTree {
        name: "content",
        root: "content",
        recursive: true,
    },
    DocumentationTree {
        name: "plugins",
        root: "plugins",
        recursive: true,
    },
];

/// History, excluded by contract rather than by exception. Named here so the
/// exclusion is a decision a reader can see and argue with.
const HISTORY: &str = "CHANGELOG.md";

// ---------------------------------------------------------------------------
// Candidates

/// A name the redesign removed, paired with what it used to be. The pairing is
/// documentation for whoever reads a failure: the token alone does not say why
/// naming it as current would be wrong.
const LEGACY_TOKENS: &[(&str, &str)] = &[
    ("grove do", "the removed lifecycle subcommand"),
    ("grove migrate", "the removed one-off migration subcommand"),
    ("grove retire", "the removed one-off retire subcommand"),
    ("--no-launch", "the removed dry-run lifecycle flag"),
    ("--harness", "the removed grow-verb harness flag"),
    ("GROVE_SESSION_TARGET", "removed exported target metadata"),
    ("GROVE_LLM_BIN", "the removed helper-binary override"),
    ("GROVE_KILL_GRACE", "removed grace-period configuration"),
    ("GROVE_HARNESS_PID", "a retired pre-watcher handle"),
    ("GROVE_CLAUDE_PID", "a retired pre-watcher handle"),
    ("GROVE_SKILL_DIR", "the removed skill-directory override"),
    ("**Kind:**", "the removed task-body kind field"),
    ("**Harness:**", "the removed task-body harness field"),
    ("**Producer launch:**", "the removed review-target receipt"),
    ("start.md", "a launcher prompt that no longer exists"),
    ("retire.md", "a launcher prompt that no longer exists"),
    (
        ".grove-stamps",
        "the removed repository-local stamp directory",
    ),
    ("harness stamp", "removed repository-local harness binding"),
    // Plural because the boundary rule below is exact at both edges, and every
    // surviving refutation denies the whole class ("emits no diversity
    // warnings"). A singular reintroduction would slip; a token cannot be both
    // whole-word and open-ended, and the class is what the surfaces talk about.
    ("diversity warnings", "the removed review-target comparison"),
    (
        "primary harness",
        "the removed grove-wide harness selection",
    ),
    ("seventeen", "the superseded session-kind count"),
];

/// Where a legacy token may still be named, quoted in the surface's own words.
///
/// Every entry is one sentence that names one or more legacy tokens in order to
/// record their **removal** — as a denial ("emits no diversity warnings"), as a
/// removal list, or as the motivating problem a design solved. One removal list
/// legitimately answers for several tokens at once, which is why the table is
/// shorter than the set of occurrences it classifies.
///
/// Quotations are matched against the same flattened text the sweep searches, so
/// reflowing a paragraph or moving it within its file changes nothing. Rewording
/// the denial itself is what breaks an entry — exactly when a reader should look
/// again.
///
/// Quoting is also what makes the classification **occurrence-level**. Keyed by
/// file and token, one blessed denial exempted every later occurrence of that
/// token in that file: `docs/USAGE.md` denies diversity warnings once, and a
/// second sentence asserting them was invisible while the stale-entry check
/// still passed on the strength of the legitimate denial.
///
/// What this cannot decide is **tense**. A quoted sentence may itself slip into
/// the present about a removed surface — `docs/specs/config-driven-sessions.md`
/// did, opening its problem statement with "Grove currently reconstructs…" long
/// after it stopped doing so. Distinguishing that from a legitimate naming needs
/// a reader, not a matcher; the table's job is to force one to have looked.
const REFUTATIONS: &[(&str, &str)] = &[
    (
        "CONTEXT.md",
        "_Avoid_: naming `start.md` or `retire.md` — those launcher prompts \
         disappeared with their lifecycle verbs",
    ),
    (
        "CONTEXT.md",
        "no longer lives in a `**Kind:**` task-file field",
    ),
    (
        "CONTEXT.md",
        "removes the obsolete `**Kind:**`, `**Harness:**`, and \
         `**Producer launch:**` lines",
    ),
    (
        "CONTEXT.md",
        "_Avoid_: giving either research leaf a `**Harness:**` declaration",
    ),
    ("CONTEXT.md", "there is no `GROVE_LLM_BIN` override"),
    (
        "CONTEXT.md",
        "\"primary harness\" — harness selection is a property of each session kind",
    ),
    (
        "CONTEXT.md",
        "describing environment variables, a harness stamp, `--harness`, or a \
         leaf-level `**Harness:**` declaration as configuration fallbacks",
    ),
    (
        "content/SKILL.md",
        "There is no `start.md`, `retire.md` or `finish.md`",
    ),
    (
        "docs/ARCHITECTURE.md",
        "mapping each legacy body's `**Kind:**` to a filename kind",
    ),
    (
        "docs/ARCHITECTURE.md",
        "It strips every `**Kind:**`, `**Harness:**`, and \
         `**Producer launch:**` line",
    ),
    (
        "docs/USAGE.md",
        "it records no launch receipts and emits no diversity warnings",
    ),
    (
        "docs/adr/complete-session-configuration.md",
        "**Keep a primary harness and layer kind/family overrides over it.** \
         Rejected",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "a repository marker and local stamp selected a primary harness",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "the retired `GROVE_HARNESS_PID` / `GROVE_CLAUDE_PID` handles",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "Task bodies no longer carry `**Kind:**`, `**Harness:**`, or \
         `**Producer launch:**`",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "structured routing peeks, `GROVE_SESSION_TARGET`, producer target \
         receipts, and diversity warnings have no role in this flow and are \
         removed",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "lifecycle flags such as `--harness` and `--no-launch` are removed",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "one known `**Kind:**` value supplies the filename kind",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "Migration removes every `**Kind:**`, `**Harness:**`, and \
         `**Producer launch:**` line",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "harness detection for launching, the harness stamp, `.grove-stamps/`, \
         and per-invocation harness flags",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "leaf `**Harness:**` metadata and grow-verb harness flags",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "`grove do`, `grove migrate`, `grove retire`, and dry-run routing output",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "structured harness-routing peeks, target receipts, \
         `GROVE_SESSION_TARGET`, and review diversity warnings",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "rather than supported `GROVE_LLM_BIN` or `GROVE_KILL_GRACE*` process \
         configuration",
    ),
    (
        "docs/specs/config-driven-sessions.md",
        "`start.md` and `retire.md` disappear with their human lifecycle verbs",
    ),
    (
        "docs/specs/doubt-grove-review-mechanics.md",
        "They carry no `**Kind:**`, `**Harness:**`, or producer target metadata",
    ),
];

// ---------------------------------------------------------------------------
// Matching

fn is_word_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// The document with every whitespace run collapsed to one space, alongside
/// the original line number of each byte of the result.
///
/// Prose wraps, and a multi-word token wraps with it. Matching line by line
/// silently misses `primary\nharness` — which is not hypothetical: reflowing
/// one paragraph of `docs/specs/config-driven-sessions.md` moved a token across
/// a line break, and a line-local matcher reported the claim as *gone* rather
/// than as still present. A false negative in a removal sweep is the expensive
/// direction, so the haystack is flattened first and the line number recovered
/// from the match offset.
fn flattened(text: &str) -> (String, Vec<usize>) {
    let mut flat = String::with_capacity(text.len());
    let mut line_of_byte = Vec::with_capacity(text.len());
    let mut line = 1;
    let mut in_whitespace = false;

    for character in text.chars() {
        if character.is_whitespace() {
            if !in_whitespace {
                flat.push(' ');
                line_of_byte.push(line);
                in_whitespace = true;
            }
            if character == '\n' {
                line += 1;
            }
            continue;
        }
        in_whitespace = false;
        let before = flat.len();
        flat.push(character);
        line_of_byte.resize(flat.len(), line);
        debug_assert!(flat.len() > before);
    }

    (flat, line_of_byte)
}

/// One whole-needle occurrence in flattened space: where it starts, and the
/// original line its first byte came from.
struct Occurrence {
    start: usize,
    line: usize,
}

/// Whole-needle occurrences in an already-flattened haystack.
///
/// The boundary rule applies only where the needle's own edge is a word
/// character, which is what separates `grove do` from "grove does" while still
/// letting `**Kind:**` and `.grove-stamps` match at their punctuation edges. A
/// hyphen is deliberately *not* a word byte, so `--harness-a` is an occurrence
/// of `--harness`: a resurrected harness flag is exactly what this must catch.
///
/// Quotations run through the same matcher as tokens, so a [`REFUTATIONS`]
/// entry is located by exactly the rule that finds what it classifies.
fn occurrences_in(flat: &str, line_of_byte: &[usize], needle: &str) -> Vec<Occurrence> {
    let (haystack, needle) = (flat.as_bytes(), needle.as_bytes());
    let leading_boundary = needle.first().is_some_and(|byte| is_word_byte(*byte));
    let trailing_boundary = needle.last().is_some_and(|byte| is_word_byte(*byte));

    let mut found = Vec::new();
    for start in 0..haystack
        .len()
        .saturating_sub(needle.len().saturating_sub(1))
    {
        if &haystack[start..start + needle.len()] != needle {
            continue;
        }
        if leading_boundary && start > 0 && is_word_byte(haystack[start - 1]) {
            continue;
        }
        let after = start + needle.len();
        if trailing_boundary && haystack.get(after).is_some_and(|byte| is_word_byte(*byte)) {
            continue;
        }
        found.push(Occurrence {
            start,
            line: line_of_byte[start],
        });
    }
    found
}

/// Whole-needle occurrences of `needle` in `text`, as 1-based line numbers.
fn occurrence_lines(text: &str, needle: &str) -> Vec<usize> {
    let (flat, line_of_byte) = flattened(text);
    occurrences_in(&flat, &line_of_byte, needle)
        .into_iter()
        .map(|found| found.line)
        .collect()
}

/// Every legacy occurrence in one document that falls outside every
/// [`REFUTATIONS`] quotation committed for that document. Kept pure and
/// path-parameterised so the controls can drive the real classifier over a
/// synthetic document.
fn unclassified_occurrences(path: &str, text: &str) -> Vec<String> {
    let (flat, line_of_byte) = flattened(text);

    // The spans this document's committed quotations occupy. An occurrence is
    // classified by *containment*, so the sentence that records a removal
    // answers for the tokens inside it and for nothing else in the file.
    let refuted: Vec<(usize, usize)> = REFUTATIONS
        .iter()
        .filter(|(entry_path, _)| *entry_path == path)
        .flat_map(|(_, quotation)| {
            occurrences_in(&flat, &line_of_byte, quotation)
                .into_iter()
                .map(|found| (found.start, found.start + quotation.len()))
        })
        .collect();

    let mut findings = Vec::new();
    for (token, subject) in LEGACY_TOKENS {
        for found in occurrences_in(&flat, &line_of_byte, token) {
            let end = found.start + token.len();
            if refuted
                .iter()
                .any(|(from, to)| *from <= found.start && end <= *to)
            {
                continue;
            }
            findings.push(format!("{path}:{}: `{token}` — {subject}", found.line));
        }
    }
    findings
}

// ---------------------------------------------------------------------------
// Walking

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn collect_markdown(directory: &Path, prefix: &str, recursive: bool, into: &mut Vec<String>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{} must be readable: {error}", directory.display()));

    for entry in entries {
        let entry = entry.expect("directory entries must be readable");
        let name = entry.file_name().to_string_lossy().into_owned();
        let relative = if prefix.is_empty() {
            name.clone()
        } else {
            format!("{prefix}/{name}")
        };
        let file_type = entry.file_type().expect("entry type must be readable");

        if file_type.is_dir() {
            if recursive {
                collect_markdown(&entry.path(), &relative, recursive, into);
            }
        } else if name.ends_with(".md") && relative != HISTORY {
            into.push(relative);
        }
    }
}

/// Repo-relative Markdown paths of one documentation tree, sorted so a failure
/// message is stable.
fn markdown_of(tree: &DocumentationTree) -> Vec<String> {
    let mut paths = Vec::new();
    collect_markdown(
        &repository_root().join(tree.root),
        tree.root,
        tree.recursive,
        &mut paths,
    );
    paths.sort();
    paths
}

// ---------------------------------------------------------------------------
// The sweep

#[test]
fn no_documentation_surface_states_a_removed_launch_policy_as_current() {
    let root = repository_root();
    let mut findings = Vec::new();

    for tree in DOCUMENTATION_TREES {
        for path in markdown_of(tree) {
            let text = fs::read_to_string(root.join(&path))
                .unwrap_or_else(|error| panic!("{path} must be readable: {error}"));
            findings.extend(unclassified_occurrences(&path, &text));
        }
    }

    assert!(
        findings.is_empty(),
        "unclassified legacy claims — either the surface is stale, or the \
         sentence that records the removal belongs in REFUTATIONS, quoted:\n  {}",
        findings.join("\n  ")
    );
}

#[test]
fn the_refutation_table_carries_no_stale_entry() {
    let root = repository_root();
    let mut stale = Vec::new();

    for (path, quotation) in REFUTATIONS {
        // A quotation naming no legacy token classifies nothing, whether it was
        // authored that way or survived the token it once answered for.
        let named: Vec<&str> = LEGACY_TOKENS
            .iter()
            .map(|(token, _)| *token)
            .filter(|token| !occurrence_lines(quotation, token).is_empty())
            .collect();
        if named.is_empty() {
            stale.push(format!("{path} entry names no legacy token: {quotation:?}"));
        }

        let Ok(text) = fs::read_to_string(root.join(path)) else {
            stale.push(format!("{path} no longer exists (entry {quotation:?})"));
            continue;
        };
        if occurrence_lines(&text, quotation).is_empty() {
            stale.push(format!("{path} no longer reads {quotation:?}"));
        }
    }

    assert!(
        stale.is_empty(),
        "REFUTATIONS entries whose subject is gone — requote the surface, or \
         delete the entry rather than leaving the table decorative:\n  {}",
        stale.join("\n  ")
    );
}

#[test]
fn every_legacy_token_is_still_worth_sweeping_for() {
    let root = repository_root();
    let classified: Vec<&str> = LEGACY_TOKENS
        .iter()
        .map(|(token, _)| *token)
        .filter(|token| {
            REFUTATIONS
                .iter()
                .any(|(_, quotation)| !occurrence_lines(quotation, token).is_empty())
        })
        .collect();

    // A token nothing refutes anywhere is not evidence of a problem — several
    // are here purely as tripwires for a name that could come back. But a token
    // that no *classification* and no *source* mentions is a word this repo has
    // no relationship with, and the entry says nothing.
    let mut unmoored = Vec::new();
    for (token, _) in LEGACY_TOKENS {
        if classified.contains(token) {
            continue;
        }
        let named_in_history = fs::read_to_string(root.join(HISTORY))
            .map(|text| !occurrence_lines(&text, token).is_empty())
            .unwrap_or(false);
        if !named_in_history {
            unmoored.push(*token);
        }
    }

    assert!(
        unmoored.is_empty(),
        "LEGACY_TOKENS entries that no refutation and no history entry names — \
         the removal they guard never happened here: {unmoored:?}"
    );
}

// ---------------------------------------------------------------------------
// Controls

#[test]
fn the_sweep_rejects_a_reintroduced_legacy_claim() {
    // Positive control on the real classifier: an affirmative claim in a file
    // that refutes nothing must be reported, with its line.
    let reintroduced = unclassified_occurrences(
        "docs/USAGE.md",
        "# Usage\n\nRun `grove do` in the working tree to drive the loop.\n",
    );
    assert_eq!(
        reintroduced.len(),
        1,
        "a reintroduced lifecycle subcommand must be reported: {reintroduced:?}"
    );
    assert!(
        reintroduced[0].starts_with("docs/USAGE.md:3: `grove do`"),
        "the report must name the file and line: {}",
        reintroduced[0]
    );

    // And the classification must actually exempt: a surface's own committed
    // removal record covers every legacy token inside it — three, here.
    assert!(
        unclassified_occurrences(
            "docs/specs/config-driven-sessions.md",
            "This design removes these launch-policy surfaces:\n\n\
             - `grove do`, `grove migrate`, `grove retire`, and dry-run routing\n\
             output.\n",
        )
        .is_empty(),
        "a quoted removal record must not be reported"
    );

    // The boundary rule is what keeps the control honest: prose about grove
    // must not trip the subcommand token.
    assert!(
        unclassified_occurrences("docs/USAGE.md", "grove does not route models.\n").is_empty(),
        "`grove do` must not match inside \"grove does\""
    );
}

#[test]
fn a_denial_does_not_exempt_the_rest_of_its_own_file() {
    // The blind spot a file-and-token classification had, as a control on the
    // pairing that produced it: `docs/USAGE.md` is classified for `diversity
    // warnings` *because it denies them*. A second, affirmative sentence about
    // that same token in that same file must still be reported — and the
    // stale-entry check could never catch it, because the legitimate denial it
    // looks for is still right there.
    let alongside = unclassified_occurrences(
        "docs/USAGE.md",
        "Grove executes opaque command strings, so it records no launch\n\
         receipts and emits no diversity warnings.\n\n\
         Grove emits diversity warnings when a review reuses its producer's model.\n",
    );

    assert_eq!(
        alongside.len(),
        1,
        "an affirmative claim beside a legitimate denial must be reported: {alongside:?}"
    );
    assert!(
        alongside[0].starts_with("docs/USAGE.md:4: `diversity warnings`"),
        "the report must name the affirmative sentence, not the denial: {}",
        alongside[0]
    );
}

#[test]
fn a_claim_wrapped_across_a_line_break_is_still_an_occurrence() {
    // The false negative this sweep would otherwise have: a reflowed paragraph
    // splits a two-word token, and a line-local matcher reports the claim as
    // gone. The line reported is where the token *starts*.
    assert_eq!(
        occurrence_lines(
            "the stamp selects a primary\nharness for the tree\n",
            "primary harness"
        ),
        [1]
    );
    assert_eq!(
        occurrence_lines("first line\nthen run `grove\ndo` in the tree\n", "grove do"),
        [2]
    );
    // Flattening must not invent a boundary: "grove does" stays unmatched even
    // when the wrap lands mid-word.
    assert!(occurrence_lines("grove\ndoes not route models\n", "grove do").is_empty());
}

#[test]
fn the_sweep_reaches_every_documentation_tree() {
    // Cross-tree control. Each tree is separately owned, and a walk that
    // silently covered one of them would pass while checking almost nothing.
    for tree in DOCUMENTATION_TREES {
        let paths = markdown_of(tree);
        assert!(
            !paths.is_empty(),
            "the {} tree contributed no Markdown — the sweep is not reaching it",
            tree.name
        );
    }

    let root_tree = markdown_of(&DOCUMENTATION_TREES[0]);
    assert!(
        root_tree.iter().any(|path| path == "CONTEXT.md"),
        "the root walk must reach the glossary: {root_tree:?}"
    );
    assert!(
        !root_tree.iter().any(|path| path == HISTORY),
        "history must stay out of the sweep: {root_tree:?}"
    );
    assert!(
        markdown_of(&DOCUMENTATION_TREES[1])
            .iter()
            .any(|path| path.starts_with("docs/adr/")),
        "the docs walk must descend into the decision-record set"
    );
    assert!(
        markdown_of(&DOCUMENTATION_TREES[3])
            .iter()
            .any(|path| path.contains("/skills/")),
        "the plugins walk must descend into the skill set"
    );
}

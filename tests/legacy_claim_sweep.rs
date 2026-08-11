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
//! documentation trees must be covered by a [`REFUTATIONS`] entry — a committed
//! judgement that *this file names this token in order to record its removal*.
//! An occurrence nobody classified fails with its file and line, so a
//! reintroduced affirmative claim is caught whether or not anyone remembered to
//! add it here. The check runs in both directions: every [`REFUTATIONS`] entry
//! must still occur, so an entry whose subject is gone fails too and the table
//! cannot quietly become a fossil.
//!
//! Controls, because a sweep that cannot fail is worth nothing. The **positive
//! control** drives the same classifier over a synthetic document carrying an
//! affirmative legacy claim and shows it rejected. The **cross-tree control**
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

/// Where a legacy token may still be named, and implicitly why: every entry is
/// a surface that names the token in order to record its **removal** — as a
/// denial ("emits no diversity warnings"), as a removal list, or as the
/// motivating problem a design solved. Keyed by file and token rather than by
/// line, so ordinary editing does not churn the table while a genuine
/// reintroduction still fails.
///
/// What this cannot decide is **tense**. A file classified here may still slip
/// into the present about a removed surface — `docs/specs/config-driven-sessions.md`
/// did, opening its problem statement with "Grove currently reconstructs…" long
/// after it stopped doing so. Distinguishing that from a legitimate naming needs
/// a reader, not a matcher; the table's job is to force one to have looked.
const REFUTATIONS: &[(&str, &str)] = &[
    ("CONTEXT.md", "**Harness:**"),
    ("CONTEXT.md", "**Kind:**"),
    ("CONTEXT.md", "**Producer launch:**"),
    ("CONTEXT.md", "--harness"),
    ("CONTEXT.md", "GROVE_LLM_BIN"),
    ("CONTEXT.md", "harness stamp"),
    ("CONTEXT.md", "primary harness"),
    ("CONTEXT.md", "retire.md"),
    ("CONTEXT.md", "start.md"),
    ("content/SKILL.md", "retire.md"),
    ("content/SKILL.md", "start.md"),
    ("docs/ARCHITECTURE.md", "**Harness:**"),
    ("docs/ARCHITECTURE.md", "**Kind:**"),
    ("docs/ARCHITECTURE.md", "**Producer launch:**"),
    ("docs/USAGE.md", "diversity warnings"),
    (
        "docs/adr/complete-session-configuration.md",
        "primary harness",
    ),
    ("docs/specs/config-driven-sessions.md", "**Harness:**"),
    ("docs/specs/config-driven-sessions.md", "**Kind:**"),
    (
        "docs/specs/config-driven-sessions.md",
        "**Producer launch:**",
    ),
    ("docs/specs/config-driven-sessions.md", "--harness"),
    ("docs/specs/config-driven-sessions.md", "--no-launch"),
    ("docs/specs/config-driven-sessions.md", ".grove-stamps"),
    ("docs/specs/config-driven-sessions.md", "GROVE_CLAUDE_PID"),
    ("docs/specs/config-driven-sessions.md", "GROVE_HARNESS_PID"),
    ("docs/specs/config-driven-sessions.md", "GROVE_KILL_GRACE"),
    ("docs/specs/config-driven-sessions.md", "GROVE_LLM_BIN"),
    (
        "docs/specs/config-driven-sessions.md",
        "GROVE_SESSION_TARGET",
    ),
    ("docs/specs/config-driven-sessions.md", "diversity warnings"),
    ("docs/specs/config-driven-sessions.md", "grove do"),
    ("docs/specs/config-driven-sessions.md", "grove migrate"),
    ("docs/specs/config-driven-sessions.md", "grove retire"),
    ("docs/specs/config-driven-sessions.md", "harness stamp"),
    ("docs/specs/config-driven-sessions.md", "primary harness"),
    ("docs/specs/config-driven-sessions.md", "retire.md"),
    ("docs/specs/config-driven-sessions.md", "start.md"),
    ("docs/specs/doubt-grove-review-mechanics.md", "**Harness:**"),
    ("docs/specs/doubt-grove-review-mechanics.md", "**Kind:**"),
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

/// Whole-token occurrences, as 1-based line numbers.
///
/// The boundary rule applies only where the token's own edge is a word
/// character, which is what separates `grove do` from "grove does" while still
/// letting `**Kind:**` and `.grove-stamps` match at their punctuation edges. A
/// hyphen is deliberately *not* a word byte, so `--harness-a` is an occurrence
/// of `--harness`: a resurrected harness flag is exactly what this must catch.
fn occurrence_lines(text: &str, token: &str) -> Vec<usize> {
    let (flat, line_of_byte) = flattened(text);
    let (haystack, needle) = (flat.as_bytes(), token.as_bytes());
    let leading_boundary = needle.first().is_some_and(|byte| is_word_byte(*byte));
    let trailing_boundary = needle.last().is_some_and(|byte| is_word_byte(*byte));

    let mut lines = Vec::new();
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
        lines.push(line_of_byte[start]);
    }
    lines
}

/// Every legacy occurrence in one document that no [`REFUTATIONS`] entry
/// covers. Kept pure and path-parameterised so the positive control can drive
/// the real classifier over a synthetic document.
fn unclassified_occurrences(path: &str, text: &str) -> Vec<String> {
    let mut findings = Vec::new();
    for (token, subject) in LEGACY_TOKENS {
        if REFUTATIONS.contains(&(path, token)) {
            continue;
        }
        for line in occurrence_lines(text, token) {
            findings.push(format!("{path}:{line}: `{token}` — {subject}"));
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
        "unclassified legacy claims — either the surface is stale, or it refutes \
         the token and belongs in REFUTATIONS:\n  {}",
        findings.join("\n  ")
    );
}

#[test]
fn the_refutation_table_carries_no_stale_entry() {
    let root = repository_root();
    let mut stale = Vec::new();

    for (path, token) in REFUTATIONS {
        let Ok(text) = fs::read_to_string(root.join(path)) else {
            stale.push(format!("{path} no longer exists (entry for `{token}`)"));
            continue;
        };
        if occurrence_lines(&text, token).is_empty() {
            stale.push(format!("{path} no longer names `{token}`"));
        }
    }

    assert!(
        stale.is_empty(),
        "REFUTATIONS entries whose subject is gone — delete them rather than \
         leaving the table decorative:\n  {}",
        stale.join("\n  ")
    );
}

#[test]
fn every_legacy_token_is_still_worth_sweeping_for() {
    let root = repository_root();
    let classified: Vec<&str> = REFUTATIONS.iter().map(|(_, token)| *token).collect();

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

    // And the classification must actually exempt: the same token in the file
    // whose entry covers it stays silent.
    assert!(
        unclassified_occurrences(
            "docs/specs/config-driven-sessions.md",
            "`grove do` has no successor.\n",
        )
        .is_empty(),
        "a classified surface must not be reported"
    );

    // The boundary rule is what keeps the control honest: prose about grove
    // must not trip the subcommand token.
    assert!(
        unclassified_occurrences("docs/USAGE.md", "grove does not route models.\n").is_empty(),
        "`grove do` must not match inside \"grove does\""
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

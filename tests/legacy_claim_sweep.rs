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
//! committed. Every occurrence of every token in [`LEGACY_TOKENS`], and every
//! `GROVE_*` environment name the prose spells out, must fall inside a
//! [`REFUTATIONS`] quotation — a
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
//! **Two candidate sources, because the removed surface has two shapes.** A
//! closed vocabulary — `grove do`, `**Kind:**`, `start.md` — is listable, and
//! [`LEGACY_TOKENS`] lists it. The routing environment is not: `GROVE_<KIND>_MODEL`
//! alone expands over nineteen kinds, and `GROVE_<KIND|FAMILY>_HARNESS` over every
//! harness anyone configures. A list can only name the variables someone already
//! thought of, and the one that comes back is the one nobody listed — so those
//! candidates are **enumerated from the prose** by prefix, and everything not in
//! [`LIVE_ENVIRONMENT`] is removed launch policy needing a refutation. This is the
//! same shape-over-list argument `tests/removed_surface.rs` makes for `src/`,
//! `tests/` and `scripts/`, and `tests/session_kind_guidance.rs` for `content/`;
//! before it was made here, `docs/CONFIGURATION.md` could say
//! "set `GROVE_IMPL_MODEL=sonnet`" and every sweep in the repository stayed green.
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

/// Path prefixes that make a legacy *basename* a live file instead.
///
/// The removed launcher prompts were `content/prompts/<name>.md`, and the
/// corpus now carries one reference file per loop step — `retire.md` among
/// them. Banning the bare basename would ban a path the methodology legitimately
/// names, so an occurrence directly preceded by one of these prefixes is the
/// live file. This is a **discriminator rather than an exemption**: a stale
/// claim about the deleted prompt cannot spell it this way, so the bare token
/// stays banned everywhere else.
const LIVE_PATH_PREFIXES: &[&str] = &["references/"];

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
    // The third and last one. It outlived the other two because one bare command
    // still needed *something* in `${prompt}`, and it went the moment the driver
    // composed that prompt from marked units instead — so the class this pair
    // opened is now closed, and leaving the survivor off the list would have made
    // it "the launcher prompts we removed, except the one we removed last".
    (
        "continue.md",
        "the last launcher prompt, replaced by a composed mandate",
    ),
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

/// The `GROVE_*` names a documentation surface may state as current, each with
/// the role that makes it legitimate. Everything else carrying that prefix is
/// launch policy the redesign removed, and may be named only inside a
/// [`REFUTATIONS`] quotation.
///
/// Both survivors are here because *stating* them is right, not merely
/// tolerated — which is what the Done-when behind this sweep asks for: the
/// legitimate loop-control reference stays explicit rather than being waved
/// through by a silent exception.
const LIVE_ENVIRONMENT: &[(&str, &str)] = &[
    (
        "GROVE_SIGNAL_FILE",
        "the loop's internal completion channel — a capability the driver grants \
         its own foreground child, never a user setting",
    ),
    (
        "GROVE_TAP_DIR",
        "release scripting's own variable, steering a shell script a human runs \
         rather than a session",
    ),
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
    // Requoted at the cutover, when the three tokens collapsed into one denial.
    // `continue.md` used to need a second entry of its own — the glossary said
    // the launcher "does not come back *under that name*", which denied the file
    // and not the surface — and there is no launcher of any name now: the driver
    // authors the guaranteed core in Rust, under the too-late test. One sentence
    // denies all three, so one entry does.
    (
        "CONTEXT.md",
        "_Avoid_: naming `start.md`, `retire.md` or `continue.md` — those \
         launcher prompts disappeared with their lifecycle verbs",
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
        "`start.md`, `retire.md` and `continue.md` disappeared with the human \
         lifecycle verbs",
    ),
    (
        "docs/specs/doubt-grove-review-mechanics.md",
        "A leaf carries no `**Kind:**`, `**Harness:**`, or producer target metadata",
    ),
    // The micro-test's control arm *is* the removed launcher, recovered from VCS
    // at the revision before it was deleted. Naming it there records what was
    // measured, not a surface that still exists.
    (
        "docs/research/wording-micro-test.md",
        "the pre-mandate launcher, recovered from VCS rather than reconstructed: \
         `content/prompts/continue.md`",
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

/// Every spelled-out `GROVE_*` name in an already-flattened haystack, as
/// (name, start offset, original line).
///
/// Two lexical rules, matching the enumeration `tests/removed_surface.rs` runs
/// over the source trees, so the same name classifies the same way on both
/// sides of the repository. A match must start at a **non-identifier
/// boundary**, or a longer name would also enumerate as its own suffix. And the
/// bare prefix with nothing spellable after it is dropped: `GROVE_*`,
/// `GROVE_<KIND|FAMILY>_HARNESS` and `GROVE_*_MODEL` are how prose writes a
/// *family*, and a schematic name is an argument about a class rather than a
/// variable a reader could export. Dropping them is what leaves the concrete
/// reintroduction — the line someone would copy and paste — as the thing this
/// sweep is looking at.
fn environment_names_in(flat: &str, line_of_byte: &[usize]) -> Vec<(String, usize, usize)> {
    const PREFIX: &str = "GROVE_";
    let bytes = flat.as_bytes();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(offset) = flat[from..].find(PREFIX) {
        let start = from + offset;
        let preceded_by_identifier = start > 0 && is_word_byte(bytes[start - 1]);
        let mut end = start + PREFIX.len();
        while end < bytes.len()
            && (bytes[end].is_ascii_uppercase()
                || bytes[end].is_ascii_digit()
                || bytes[end] == b'_')
        {
            end += 1;
        }
        if !preceded_by_identifier && end > start + PREFIX.len() {
            found.push((flat[start..end].to_owned(), start, line_of_byte[start]));
        }
        from = end.max(start + PREFIX.len());
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

    let is_refuted = |start: usize, end: usize| {
        refuted
            .iter()
            .any(|(from, to)| *from <= start && end <= *to)
    };

    let mut findings = Vec::new();
    for (token, subject) in LEGACY_TOKENS {
        for found in occurrences_in(&flat, &line_of_byte, token) {
            let end = found.start + token.len();
            if is_refuted(found.start, end) {
                continue;
            }
            if LIVE_PATH_PREFIXES
                .iter()
                .any(|prefix| flat[..found.start].ends_with(prefix))
            {
                continue;
            }
            findings.push(format!("{path}:{}: `{token}` — {subject}", found.line));
        }
    }

    // The enumerated half. Same containment rule, so one removal sentence
    // answers for the variables inside it exactly as it does for the listed
    // tokens, and one failure message reports both.
    for (name, start, line) in environment_names_in(&flat, &line_of_byte) {
        if LIVE_ENVIRONMENT.iter().any(|(live, _)| *live == name) {
            continue;
        }
        if is_refuted(start, start + name.len()) {
            continue;
        }
        findings.push(format!(
            "{path}:{line}: `{name}` — an environment variable no launch reads; \
             configuration is the only launch policy"
        ));
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

/// Every document the sweep is responsible for, as (repo-relative path, text).
fn swept_documents() -> Vec<(String, String)> {
    let root = repository_root();
    DOCUMENTATION_TREES
        .iter()
        .flat_map(markdown_of)
        .map(|path| {
            let text = fs::read_to_string(root.join(&path))
                .unwrap_or_else(|error| panic!("{path} must be readable: {error}"));
            (path, text)
        })
        .collect()
}

#[test]
fn no_documentation_surface_states_a_removed_launch_policy_as_current() {
    let mut findings = Vec::new();
    for (path, text) in swept_documents() {
        findings.extend(unclassified_occurrences(&path, &text));
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

/// The reverse direction for the enumerated half, matching
/// [`the_refutation_table_carries_no_stale_entry`]. An allow-list entry whose
/// last occurrence was deleted exempts nothing, and a permission granted to a
/// name no surface uses is how a sweep quietly reacquires the blind spot it was
/// written to close.
#[test]
fn the_live_environment_table_carries_no_stale_entry() {
    let documents = swept_documents();
    let stale: Vec<&str> = LIVE_ENVIRONMENT
        .iter()
        .map(|(name, _)| *name)
        .filter(|name| {
            !documents.iter().any(|(_, text)| {
                let (flat, line_of_byte) = flattened(text);
                environment_names_in(&flat, &line_of_byte)
                    .iter()
                    .any(|(found, _, _)| found == name)
            })
        })
        .collect();

    assert!(
        stale.is_empty(),
        "these LIVE_ENVIRONMENT entries permit a name no swept surface uses — \
         the exemption outlived its subject and now only widens the sweep's \
         blind spot: {stale:?}"
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
fn the_sweep_rejects_a_routing_variable_no_list_could_have_named() {
    // The exact mutation that passed every sweep in this repository before the
    // enumerated half existed: three affirmative routing claims in the user's
    // configuration guide. None is in LEGACY_TOKENS, and none could sensibly
    // be — they are three members of two open families.
    let reintroduced = unclassified_occurrences(
        "docs/CONFIGURATION.md",
        "Set `GROVE_IMPL_MODEL=sonnet` and `GROVE_REVIEW_HARNESS=codex` to route\n\
         implementation and review sessions, and `GROVE_HARNESS_BIN_CODEX` to point\n\
         at the binary.\n",
    );
    assert_eq!(
        reintroduced.len(),
        3,
        "every spelled-out routing variable must be reported: {reintroduced:?}"
    );
    assert!(
        reintroduced[0].starts_with("docs/CONFIGURATION.md:1: `GROVE_IMPL_MODEL`"),
        "the report must name the file, line and variable: {}",
        reintroduced[0]
    );

    // The permission is a permission, not a prefix: the loop-control channel is
    // stated as current all over the documentation and must stay silent, while a
    // *different* name extending it does not inherit that. The neighbour ends
    // `_HARNESS` on purpose — this file is itself swept by
    // `tests/removed_surface.rs`, which enumerates the same prefix under
    // `tests/` and reports any name matching none of its shape rules. A control
    // invented for its illustrative value alone — one ending `_PATH`, say —
    // fails that sweep from inside the assertion *and* from inside a comment
    // explaining it, since a lexical sweep cannot tell the two apart and should
    // not try. The fix is not an exemption here; an exemption is the blind spot
    // this file just closed. It is a control both sweeps already classify.
    assert!(
        unclassified_occurrences(
            "docs/CONFIGURATION.md",
            "it clears any inherited Grove loop-control variables and grants this \
             launch's fresh `GROVE_SIGNAL_FILE`.\n",
        )
        .is_empty(),
        "the live control channel must not be reported"
    );
    assert_eq!(
        unclassified_occurrences(
            "docs/CONFIGURATION.md",
            "export `GROVE_SIGNAL_FILE_HARNESS`\n"
        )
        .len(),
        1,
        "a neighbouring name must not inherit the channel's permission"
    );

    // And a removal sentence answers for the variables inside it, exactly as it
    // does for the listed tokens — the containment rule is shared, not copied.
    assert!(
        unclassified_occurrences(
            "docs/specs/config-driven-sessions.md",
            "Test suites inject tool paths, clocks, and kill-grace durations through\n\
             internal module seams rather than supported `GROVE_LLM_BIN` or\n\
             `GROVE_KILL_GRACE*` process configuration.\n",
        )
        .is_empty(),
        "a quoted removal record must answer for the variables it names"
    );
}

#[test]
fn a_schematic_family_name_is_prose_rather_than_a_candidate() {
    // `GROVE_*` and `GROVE_<KIND>_MODEL` are how a surface argues about a
    // *class* — including in the sentences that record the class's removal.
    // Enumerating them would report the denial as the offence, so the prefix
    // scan stops where the name stops being spellable.
    let (flat, lines) = flattened(
        "no user-settable `GROVE_*` environment configuration, and all\n\
         `GROVE_<KIND|FAMILY>_HARNESS` and `GROVE_*_MODEL` routing variables\n\
         are removed; `GROVE_IMPL_MODEL` is gone with them.\n",
    );
    let names: Vec<String> = environment_names_in(&flat, &lines)
        .into_iter()
        .map(|(name, _, _)| name)
        .collect();
    assert_eq!(
        names,
        ["GROVE_IMPL_MODEL"],
        "only the spelled-out variable is a candidate"
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

/// **The control on [`LIVE_PATH_PREFIXES`].** A discriminator that swallowed the
/// bare token would silence the claim it exists to catch, so the same token is
/// shown reported in one spelling and passed over in the other.
#[test]
fn a_live_reference_path_is_not_a_removed_launcher_prompt() {
    let bare = unclassified_occurrences("fixture.md", "the retire.md launcher prompt\n");
    assert_eq!(
        bare.len(),
        1,
        "a bare launcher-prompt basename must still be reported: {bare:?}"
    );

    let live = unclassified_occurrences("fixture.md", "see `references/retire.md`\n");
    assert!(
        live.is_empty(),
        "a live reference path must not read as the removed prompt: {live:?}"
    );
}

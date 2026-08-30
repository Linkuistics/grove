//! **The methodology instructs no `grove-llm` verb the CLI lacks** — asserted
//! over the skill set this repository ships, against clap's own command model.
//!
//! **Re-homed here at `delete-provisioning-k19`**, from `tests/methodology.rs`,
//! which was a file of claims about the *embed* and went with it. This claim is
//! not one of those: its subject is the methodology's **prose** and the CLI's
//! **grammar**, and both survive the delivery move. What changed is where the
//! prose is read from — `plugins/grove/skills/`, on disk, rather than
//! `include_dir!`'s linked copy — and what the agreement is worth, which the
//! test's own doc comment states rather than assumes.
//!
//! The other half of the old file's subject — that every kind's reference file
//! exists and that each skill's frontmatter is well formed — is
//! `plugins/grove/conformance.sh`'s, which asserts it over the same files a
//! harness installs.

mod support;

use clap::CommandFactory;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

/// Every Markdown file in the shipped skill set, as `(skills/-relative path,
/// text)`, sorted by path.
///
/// The whole plugin rather than the spine alone: a kind skill instructs verbs
/// too, and the claim is about what *any* session can be told. Sorted so a
/// failure names the same file on every machine.
fn shipped_markdown() -> Vec<(String, String)> {
    let root = support::repo_root().join("plugins/grove/skills");
    let mut files = Vec::new();
    collect_markdown(&root, &root, &mut files);
    assert!(
        !files.is_empty(),
        "no markdown found under {} — a mis-scoped walk asserts nothing",
        root.display()
    );
    files.sort_by(|a, b| a.0.cmp(&b.0));
    files
}

fn collect_markdown(root: &Path, dir: &PathBuf, out: &mut Vec<(String, String)>) {
    for entry in std::fs::read_dir(dir).expect("the shipped skill set must be readable") {
        let path = entry.expect("a readable directory entry").path();
        if path.is_dir() {
            collect_markdown(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).expect("a path under the root");
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("{} must be readable: {error}", path.display()));
            out.push((relative.to_string_lossy().into_owned(), text));
        }
    }
}

/// **The methodology instructs no `grove-llm` verb the CLI lacks.**
///
/// **Any** skew between the skill set and the CLI it instructs is unsafe, in
/// both directions — a newer skill names verbs added since the binary, an older
/// one names verbs *removed* since the skill. This repository holds the second
/// case's ingredients: the `v17.0.0` methodology instructs `leaf-add-chain`,
/// removed after that tag, so that skill beside any later binary is broken.
///
/// **The two used to ship as one artifact, and no longer do.** `content/` was
/// embedded in the binary with `include_dir!`, which made this claim a property
/// of a single build and drove the drift to zero by construction.
/// `delete-provisioning-k19` deleted the embed: the methodology installs as a
/// plugin, the binary installs through Homebrew, and the two now have separate
/// lifetimes. So the guarantee weakened from *a shipped binary cannot hand a
/// session a verb it lacks* to *this checkout's skill set and this checkout's
/// CLI agree*, which is what a working tree can actually assert. What pays for
/// the difference is that both are versioned in this one repository and land in
/// one commit; what does not is a user who upgrades one and not the other, and
/// that residue is `plugins/grove/README.md`'s to state rather than this file's
/// to pretend away.
///
/// No test can see a future build, so "the installed skill is current" is not
/// the statable claim. **This one is**: every `grove-llm` verb the shipped
/// methodology instructs is a verb this CLI exposes.
///
/// Mentions come from the corpus rather than from a list, so a verb invented in
/// prose — or one deleted from the CLI while an instruction survives — fails
/// here without anyone having remembered to add it. The reading rule lives on
/// [`scan_instructed_verbs`] and the grain both sides are compared at lives on
/// [`exposed_verbs`]; each is pinned by its own control below, because a claim
/// this universal is only as good as the reader that gathers its corpus.
#[test]
fn the_shipped_methodology_instructs_no_verb_the_cli_lacks() {
    let exposed = exposed_verbs();
    let mut instructed = Vec::new();
    for (path, text) in shipped_markdown() {
        scan_instructed_verbs(&path, &text, &mut instructed);
    }

    let unknown: Vec<&(String, usize, String)> = instructed
        .iter()
        .filter(|(_, _, verb)| !exposed.contains(verb))
        .collect();
    assert!(
        unknown.is_empty(),
        "the shipped methodology instructs {} verb(s) `grove-llm` does not \
         expose — a session following it would make a call that cannot \
         succeed. Exposed: {exposed:?}. Offending mentions: {unknown:?}",
        unknown.len()
    );

    // Positive control, pinned complete rather than floored. A scan that quietly
    // matches nothing satisfies the assertion above forever, and a *floor* only
    // bounds how much it may quietly lose: `>= 8` against a corpus of ten let
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
        "the shipped methodology's instructed-verb set moved. Verbs found but \
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

/// Every `grove-llm` verb the shipped methodology instructs, named in full so
/// the corpus is pinned rather than merely floored — see the completeness
/// control in [`the_shipped_methodology_instructs_no_verb_the_cli_lacks`].
///
/// The skill set is the only thing teaching a session which verbs exist, so a
/// verb it names wrongly is a call a session cannot make and has no second
/// source to correct it from. That was already true while the corpus was
/// embedded; it is more so now that the two ship separately.
const INSTRUCTED_VERBS: [&str; 10] = [
    "brief-chain",
    "complete",
    "finish-commit",
    "leaf-add",
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
    grove_llm::cli::Cli::command()
        .get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .collect()
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
    let nested: Vec<String> = grove_llm::cli::Cli::command()
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
/// the corpus splits at least one invocation across a line break, and a
/// line-based scan drops it silently — the exact failure mode that makes a
/// removal check worthless.
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

//! The provisioned methodology's **session-kind** surface, asserted against the
//! two things that cannot drift from the shipped behaviour: `Kind::ALL` and the
//! real `grove-llm` command model.
//!
//! The obvious way to write this file is a list of retired spellings —
//! `**Kind:**`, `**Harness:**`, `--harness-a`, `seventeen` — checked one by one.
//! That version answers only the question its author thought to ask. It passes
//! forever once those strings are gone, it passes unchanged the day a *twentieth*
//! kind ships undocumented, and it passes unchanged the day an example is written
//! with a filename shape the parser rejects. All three failure modes are silent.
//!
//! So each claim here is generated from a source that moves when the product
//! does:
//!
//! * The **set** comes from [`Kind::ALL`], so a kind added to the enum fails
//!   until `content/TASK-FORMAT.md` names it, and the set's *size word* fails
//!   until the prose is recounted.
//! * The **filename grammar** is enumerated out of the guidance itself and each
//!   example is classified, so an example written in the pre-session-kind shape
//!   fails whether or not anyone thought to look for it.
//! * The **flags** come from `grove-llm`'s clap model, so a flag the docs invent
//!   — or one the CLI drops — fails from either side.
//!
//! Two limits, stated rather than papered over. A filename whose *slug* begins
//! with a kind word (`01-design-notes-k4.md`) is indistinguishable from a kinded
//! name by inspection; that ambiguity is in the grammar itself, which is why
//! `.grove/FORMAT` and not a prefix test decides a real tree's format. And the
//! flag sweep only judges a line that names a hyphenated `grove-llm` verb, so a
//! flag discussed in bare prose is out of scope — the check under-covers rather
//! than lying about coverage.
//!
//! Every sweep carries both controls, because a sweep that cannot fail is worth
//! nothing: each classifier is shown rejecting the shape it exists to reject and
//! accepting the one it exists to allow.

use clap::CommandFactory;
use grove::leaf::Kind;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// The surfaces

const TASK_FORMAT: &str = include_str!("../content/TASK-FORMAT.md");
const SKILL: &str = include_str!("../content/SKILL.md");
const DRIVING: &str = include_str!("../content/driving.md");
const DOUBT_SKILL: &str =
    include_str!("../plugins/linkuistics/skills/doubt-driven-development/SKILL.md");

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Every `.md` file the binary provisions, read from disk rather than listed, so
/// a reference file added to `content/` joins the sweep by existing.
fn provisioned_markdown() -> Vec<(PathBuf, String)> {
    fn walk(path: &Path, out: &mut Vec<(PathBuf, String)>) {
        if path.is_dir() {
            let mut entries: Vec<PathBuf> = fs::read_dir(path)
                .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()))
                .map(|entry| entry.unwrap().path())
                .collect();
            entries.sort();
            for entry in entries {
                walk(&entry, out);
            }
            return;
        }
        if path.extension().is_some_and(|extension| extension == "md") {
            let text = fs::read_to_string(path)
                .unwrap_or_else(|error| panic!("reading {}: {error}", path.display()));
            let relative = path.strip_prefix(manifest_dir()).unwrap_or(path);
            out.push((relative.to_path_buf(), text));
        }
    }

    let mut out = Vec::new();
    walk(&manifest_dir().join("content"), &mut out);
    assert!(
        out.len() > 3,
        "the walk found {} provisioned markdown files — a mis-scoped walk reports \
         a clean surface for the wrong reason",
        out.len()
    );
    out
}

// ---------------------------------------------------------------------------
// The set, and the word that counts it

/// English cardinals, only as far as a plausible kind set reaches. The size word
/// is derived rather than typed into the assertion, so recounting the taxonomy is
/// what changes it.
const CARDINALS: [(usize, &str); 11] = [
    (15, "fifteen"),
    (16, "sixteen"),
    (17, "seventeen"),
    (18, "eighteen"),
    (19, "nineteen"),
    (20, "twenty"),
    (21, "twenty-one"),
    (22, "twenty-two"),
    (23, "twenty-three"),
    (24, "twenty-four"),
    (25, "twenty-five"),
];

fn cardinal(count: usize) -> &'static str {
    CARDINALS
        .iter()
        .find(|(value, _)| *value == count)
        .map(|(_, word)| *word)
        .unwrap_or_else(|| {
            panic!(
                "the kind set has {count} members and CARDINALS stops short of it — \
                 extend the table so the size word stays derived"
            )
        })
}

/// The taxonomy's home names every kind, and the guidance that summarises it
/// names the four whose spelling the design changed. A twentieth kind fails here
/// the moment it reaches the enum.
#[test]
fn every_session_kind_is_named_by_the_provisioned_taxonomy() {
    let missing: Vec<&str> = Kind::ALL
        .iter()
        .map(|kind| kind.label())
        .filter(|label| !TASK_FORMAT.contains(*label))
        .collect();
    assert!(
        missing.is_empty(),
        "content/TASK-FORMAT.md is the taxonomy's home and does not name {missing:?}"
    );

    // The kinds a summarising surface must spell out rather than derive: the
    // three that replaced one `research` kind plus one `**Harness:**` line, and
    // the driver-reserved one no session may create. The producers and their two
    // step families are stated generatively in prose ("each with its own
    // `review-` and `integrate-review-` step"), so requiring all nineteen of a
    // summary would demand a table it deliberately does not carry.
    for (surface, text, expected) in [
        (
            "content/SKILL.md",
            SKILL,
            ["research-a", "research-b", "combine-research", "finish"].as_slice(),
        ),
        (
            "content/driving.md",
            DRIVING,
            ["research-a", "research-b", "combine-research"].as_slice(),
        ),
        (
            "doubt-driven-development/SKILL.md",
            DOUBT_SKILL,
            ["research-a", "research-b", "combine-research"].as_slice(),
        ),
    ] {
        let missing: Vec<&&str> = expected
            .iter()
            .filter(|label| !text.contains(**label))
            .collect();
        assert!(
            missing.is_empty(),
            "{surface} must name {missing:?} — the vendor pair's two kinds are \
             what replaced a per-leaf harness declaration"
        );
    }
}

/// The prose states the set's size in words, and the word is the enum's. A kind
/// added without recounting fails, which is the half a per-label check misses.
#[test]
fn the_guidance_counts_the_kind_set_correctly() {
    let word = cardinal(Kind::ALL.len());
    for (surface, text) in [
        ("content/TASK-FORMAT.md", TASK_FORMAT),
        ("content/SKILL.md", SKILL),
    ] {
        assert!(
            text.contains(word),
            "{surface} must state the set's size as {word:?} — the taxonomy has \
             {} members",
            Kind::ALL.len()
        );
    }

    // The other direction: no *other* cardinal may be used to count kinds. Stated
    // against the two phrasings the guidance actually uses for a set size, so
    // "the five producers" and "all five `review-*` kinds" — real counts of real
    // subsets — do not read as stale claims about the whole set.
    for (surface, text) in [
        ("content/TASK-FORMAT.md", TASK_FORMAT),
        ("content/SKILL.md", SKILL),
        ("content/driving.md", DRIVING),
    ] {
        let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
        for (count, stale) in CARDINALS {
            if count == Kind::ALL.len() {
                continue;
            }
            for phrasing in [
                format!("the {stale} kinds"),
                format!("one of the {stale}"),
                format!("set of **{stale}**"),
                format!("listing all {stale}"),
            ] {
                assert!(
                    !collapsed.contains(&phrasing),
                    "{surface} still counts the kind set as {stale:?} ({phrasing:?}); \
                     it has {} members",
                    Kind::ALL.len()
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// The filename grammar, enumerated out of the guidance

/// What an example leaf filename in the guidance turned out to be.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Example {
    /// `NN-[DONE-]<kind>-<slug>-k<key>.md` — the current grammar.
    Kinded,
    /// No leading token is a kind: the pre-session-kind grammar, or a slug where
    /// a kind belongs.
    MissingKind,
    /// A kind is present but consumed the whole name, leaving no slug — which is
    /// exactly what a pre-session-kind example looks like when its slug happens
    /// to be a kind word (`02-impl-k3.md`).
    KindWithoutSlug,
}

/// The kind labels plus the placeholders the guidance writes in a grammar
/// sketch, longest first so `review-design` cannot be shadowed by `design`.
fn kind_tokens() -> Vec<String> {
    let mut tokens: Vec<String> = Kind::ALL
        .iter()
        .map(|kind| kind.label().to_owned())
        .chain(["<session-kind>".to_owned(), "<kind>".to_owned()])
        .collect();
    tokens.sort_by_key(|token| std::cmp::Reverse(token.len()));
    tokens
}

/// Classify one candidate filename. Position, then an optional outcome infix,
/// then a kind, then a non-empty slug ending in a key.
fn classify_example(name: &str) -> Example {
    let after_position = name
        .strip_prefix("NN-")
        .or_else(|| {
            let (head, tail) = name.split_at(name.len().min(3));
            let digits = head.len() == 3
                && head.as_bytes()[0].is_ascii_digit()
                && head.as_bytes()[1].is_ascii_digit()
                && head.as_bytes()[2] == b'-';
            digits.then_some(tail)
        })
        .expect("candidates are collected with a position prefix");

    let after_outcome = ["[DONE-|ABANDONED-]", "DONE-", "ABANDONED-"]
        .iter()
        .find_map(|infix| after_position.strip_prefix(infix))
        .unwrap_or(after_position);

    let Some(after_kind) = kind_tokens().iter().find_map(|token| {
        after_outcome
            .strip_prefix(token.as_str())
            .and_then(|rest| rest.strip_prefix('-'))
    }) else {
        return Example::MissingKind;
    };

    let body = after_kind.trim_end_matches(".md");
    let Some((slug, _key)) = body.rsplit_once("-k") else {
        return Example::KindWithoutSlug;
    };
    if slug.is_empty() {
        return Example::KindWithoutSlug;
    }
    Example::Kinded
}

/// Every candidate leaf filename on one line: a position prefix through a
/// terminating `.md`. Anything without `.md` is not a filename — that is what
/// keeps a node directory (`NN-<slug>-k<key>/`) and a bare position reference
/// (`01-…`) out of the sweep instead of needing an exception each.
fn candidates_in(line: &str) -> Vec<String> {
    fn token_char(byte: u8) -> bool {
        byte.is_ascii_alphanumeric()
            || matches!(byte, b'-' | b'_' | b'<' | b'>' | b'[' | b']' | b'|' | b'.')
    }

    let bytes = line.as_bytes();
    let mut found = Vec::new();
    let mut index = 0;
    while index < bytes.len() {
        let boundary = index == 0 || !bytes[index - 1].is_ascii_alphanumeric();
        let position = bytes[index..].starts_with(b"NN-")
            || (index + 3 <= bytes.len()
                && bytes[index].is_ascii_digit()
                && bytes[index + 1].is_ascii_digit()
                && bytes[index + 2] == b'-');
        if !(boundary && position) {
            index += 1;
            continue;
        }
        let mut end = index;
        while end < bytes.len() && token_char(bytes[end]) {
            end += 1;
        }
        let token = &line[index..end];
        if let Some(cut) = token.find(".md") {
            found.push(token[..cut + 3].to_owned());
        }
        index = end.max(index + 1);
    }
    found
}

#[test]
fn every_leaf_filename_example_in_the_methodology_carries_a_session_kind() {
    let mut examples = 0;
    let mut findings = Vec::new();
    for (path, text) in provisioned_markdown() {
        for (offset, line) in text.lines().enumerate() {
            for candidate in candidates_in(line) {
                examples += 1;
                let role = classify_example(&candidate);
                if role != Example::Kinded {
                    findings.push(format!(
                        "{}:{}: {candidate} ({role:?})",
                        path.display(),
                        offset + 1
                    ));
                }
            }
        }
    }
    assert!(
        examples >= 8,
        "the sweep found only {examples} filename examples — the provisioned \
         guidance carries more than that, so the scan is mis-scoped"
    );
    assert!(
        findings.is_empty(),
        "these filename examples do not carry a session kind:\n  {}",
        findings.join("\n  ")
    );
}

#[test]
fn the_filename_classifier_separates_the_current_grammar_from_its_predecessor() {
    // The shape the design replaced, in all three of the forms it took.
    assert_eq!(classify_example("01-plan-k1.md"), Example::MissingKind);
    assert_eq!(classify_example("01-DONE-spec-k2.md"), Example::MissingKind);
    assert_eq!(classify_example("02-impl-k3.md"), Example::KindWithoutSlug);

    // The current grammar, live and both terminal states, plus the sketches.
    for current in [
        "01-requirements-plan-k1.md",
        "01-DONE-design-spec-k2.md",
        "03-ABANDONED-impl-extract-k7.md",
        "03-integrate-review-design-sync-design-integrate-k15.md",
        "01-research-a-sync-survey-a-k17.md",
        "NN-<session-kind>-<slug>-k<key>.md",
        "NN-[DONE-|ABANDONED-]<session-kind>-<slug>-k<key>.md",
        "01-<session-kind>-<first-child-slug>-k<new>.md",
    ] {
        assert_eq!(
            classify_example(current),
            Example::Kinded,
            "{current} is the current grammar"
        );
    }

    // The collector's own boundaries: a node directory and a bare position
    // reference are not filenames and must not enter the sweep at all.
    assert!(candidates_in("a node `NN-<slug>-k<key>/` holding `01-…`, `02-…`").is_empty());
    assert_eq!(
        candidates_in("l1[\"01-DONE-design-spec-k2.md — retired\"]"),
        ["01-DONE-design-spec-k2.md"]
    );
}

// ---------------------------------------------------------------------------
// Launch policy has one home, and the methodology may not name another

/// The one `GROVE_*` name the methodology legitimately mentions: the loop's
/// completion channel, which the driver grants its child and `complete` writes.
/// It is a capability, not a setting — every removed routing variable is
/// classified by `tests/removed_surface.rs`, which deliberately leaves `content/`
/// to this file.
const LIVE_CONTROL_CHANNEL: &str = "GROVE_SIGNAL_FILE";

fn grove_names_in(line: &str) -> Vec<String> {
    const PREFIX: &str = "GROVE_";
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(offset) = line[from..].find(PREFIX) {
        let start = from + offset;
        let preceded_by_identifier =
            start > 0 && (bytes[start - 1].is_ascii_alphanumeric() || bytes[start - 1] == b'_');
        let mut end = start + PREFIX.len();
        while end < bytes.len()
            && (bytes[end].is_ascii_uppercase()
                || bytes[end].is_ascii_digit()
                || bytes[end] == b'_')
        {
            end += 1;
        }
        if !preceded_by_identifier && end > start + PREFIX.len() {
            found.push(line[start..end].to_owned());
        }
        from = end.max(start + PREFIX.len());
    }
    found
}

#[test]
fn the_methodology_names_no_launch_policy_environment_variable() {
    let mut live = 0;
    let mut findings = Vec::new();
    for (path, text) in provisioned_markdown() {
        for (offset, line) in text.lines().enumerate() {
            for name in grove_names_in(line) {
                if name == LIVE_CONTROL_CHANNEL {
                    live += 1;
                    continue;
                }
                findings.push(format!("{}:{}: {name}", path.display(), offset + 1));
            }
        }
    }
    assert!(
        findings.is_empty(),
        "configuration is the only launch policy; the provisioned methodology \
         still names:\n  {}",
        findings.join("\n  ")
    );
    assert!(
        live > 0,
        "the sweep found no {LIVE_CONTROL_CHANNEL} either — a scan that finds \
         nothing reports a clean surface for the wrong reason"
    );
}

#[test]
fn the_environment_sweep_finds_a_reintroduced_routing_variable() {
    assert_eq!(
        grove_names_in("routed by one `GROVE_REVIEW_HARNESS` line, not a leaf"),
        ["GROVE_REVIEW_HARNESS"]
    );
    assert_ne!(
        grove_names_in("GROVE_REVIEW_HARNESS")[0],
        LIVE_CONTROL_CHANNEL
    );
    assert_eq!(
        grove_names_in("writes to `GROVE_SIGNAL_FILE`"),
        [LIVE_CONTROL_CHANNEL]
    );
}

// ---------------------------------------------------------------------------
// No example task body carries a launch field

/// The whole declared body-marker set: the two composition relationships a
/// generated chain writes. That these are the *only* markers Grove writes is
/// asserted by enumeration over real generated bodies in
/// `tests/task_marker_surface.rs`; this file's claim is the documentation half —
/// the methodology may not show a body field Grove does not write.
const DECLARED_BODY_MARKERS: [&str; 2] = ["**Reviews:**", "**Integrates:**"];

/// Every `**…:**` marker inside a code block — fenced, or indented four spaces.
/// The block boundary is what separates an *example of file content* from a bold
/// label in prose, so the sweep needs no exception for `**Signs you want …:**`.
fn markers_in_code_blocks(text: &str) -> Vec<(usize, String)> {
    let mut fence: Option<(char, usize)> = None;
    let mut found = Vec::new();
    for (offset, line) in text.lines().enumerate() {
        let trimmed = line.trim_start_matches(' ');
        let indent = line.len() - trimmed.len();
        let run = |marker: char| trimmed.chars().take_while(|c| *c == marker).count();

        if let Some((marker, length)) = fence {
            if run(marker) >= length && trimmed.trim_end().chars().all(|c| c == marker) {
                fence = None;
                continue;
            }
        } else if indent <= 3 && matches!(trimmed.chars().next(), Some('`') | Some('~')) {
            let marker = trimmed.chars().next().unwrap();
            if run(marker) >= 3 {
                fence = Some((marker, run(marker)));
                continue;
            }
        }

        let in_block = fence.is_some() || (indent >= 4 && !trimmed.is_empty());
        if !in_block {
            continue;
        }
        let mut rest = line;
        while let Some(start) = rest.find("**") {
            let after = &rest[start + 2..];
            let Some(end) = after.find(":**") else { break };
            found.push((offset + 1, format!("**{}:**", &after[..end])));
            rest = &after[end + 3..];
        }
    }
    found
}

#[test]
fn no_example_task_body_carries_a_launch_field() {
    let findings: Vec<String> = provisioned_markdown()
        .into_iter()
        .flat_map(|(path, text)| {
            markers_in_code_blocks(&text)
                .into_iter()
                .filter(|(_, marker)| !DECLARED_BODY_MARKERS.contains(&marker.as_str()))
                .map(move |(line, marker)| format!("{}:{line}: {marker}", path.display()))
                .collect::<Vec<_>>()
        })
        .collect();
    assert!(
        findings.is_empty(),
        "a task body carries composition relationships and nothing about a \
         launch; the methodology still shows:\n  {}",
        findings.join("\n  ")
    );
}

#[test]
fn the_body_marker_sweep_reads_examples_and_not_prose() {
    let with_field = "text\n\n```markdown\n# slug-k1\n\n**Kind:** impl\n```\n";
    assert_eq!(
        markers_in_code_blocks(with_field),
        [(6, "**Kind:**".to_owned())]
    );

    let indented = "text\n\n    **Harness:** codex\n";
    assert_eq!(
        markers_in_code_blocks(indented),
        [(3, "**Harness:**".into())]
    );

    let declared = "```\n**Reviews:** slug-k1\n```\n";
    assert_eq!(
        markers_in_code_blocks(declared),
        [(2, "**Reviews:**".into())]
    );

    // A bold label in prose is not a body field and must not enter the sweep.
    assert!(markers_in_code_blocks("**Signs you want a research leaf:**\n").is_empty());
}

// ---------------------------------------------------------------------------
// Documented flags exist on the real verb

/// Every long flag the `grove-llm` command model exposes, at any depth.
fn real_long_flags() -> BTreeSet<String> {
    fn walk(command: &clap::Command, out: &mut BTreeSet<String>) {
        for argument in command.get_arguments() {
            if let Some(long) = argument.get_long() {
                out.insert(long.to_owned());
            }
            for alias in argument.get_all_aliases().unwrap_or_default() {
                out.insert(alias.to_owned());
            }
        }
        for sub in command.get_subcommands() {
            walk(sub, out);
        }
    }
    let mut out = BTreeSet::new();
    walk(&grove::llm_cli::Cli::command(), &mut out);
    assert!(
        out.contains("kind"),
        "the flag walk found no `--kind` — a mis-scoped walk accepts every \
         invented flag"
    );
    out
}

/// The verb names that mark a line as being *about* `grove-llm`: the hyphenated
/// ones. Derived from the command model, and hyphenated because the rest
/// (`kind`, `pick`, `resolve`, `complete`) are ordinary English words that appear
/// throughout the prose.
fn hyphenated_verbs() -> Vec<String> {
    let verbs: Vec<String> = grove::llm_cli::Cli::command()
        .get_subcommands()
        .map(|sub| sub.get_name().to_owned())
        .filter(|name| name.contains('-'))
        .collect();
    assert!(
        verbs.iter().any(|verb| verb == "leaf-add-pair"),
        "the verb walk missed `leaf-add-pair`: {verbs:?}"
    );
    verbs
}

fn long_flags_in(line: &str) -> Vec<String> {
    let bytes = line.as_bytes();
    let mut found = Vec::new();
    let mut from = 0;
    while let Some(offset) = line[from..].find("--") {
        let start = from + offset;
        // A flag starts at a word boundary. Without this, the `--` inside a
        // markdown anchor (`#the-review-chain--when-doubt…`) reads as a flag.
        let boundary = start == 0 || matches!(bytes[start - 1], b' ' | b'`' | b'(' | b'\t');
        let mut end = start + 2;
        while end < bytes.len() && (bytes[end].is_ascii_lowercase() || bytes[end] == b'-') {
            end += 1;
        }
        if boundary && end > start + 2 {
            found.push(line[start + 2..end].trim_end_matches('-').to_owned());
        }
        from = end.max(start + 2);
    }
    found
}

#[test]
fn every_documented_grove_llm_flag_exists_on_the_real_verb() {
    let real = real_long_flags();
    let verbs = hyphenated_verbs();
    let mut checked = 0;
    let mut findings = Vec::new();
    for (path, text) in provisioned_markdown() {
        for (offset, line) in text.lines().enumerate() {
            if !verbs.iter().any(|verb| line.contains(verb.as_str())) {
                continue;
            }
            for flag in long_flags_in(line) {
                checked += 1;
                if !real.contains(&flag) {
                    findings.push(format!("{}:{}: --{flag}", path.display(), offset + 1));
                }
            }
        }
    }
    assert!(
        checked > 0,
        "no documented flag was checked — the guidance shows `--kind` on a verb \
         line, so the scan is mis-scoped"
    );
    assert!(
        findings.is_empty(),
        "the methodology documents flags `grove-llm` does not have:\n  {}",
        findings.join("\n  ")
    );
}

#[test]
fn the_flag_sweep_rejects_an_invented_selector_and_skips_an_anchor() {
    assert_eq!(
        long_flags_in("grove-llm leaf-add-pair . stem --harness-a claude --harness-b codex"),
        ["harness-a", "harness-b"]
    );
    let real = real_long_flags();
    assert!(!real.contains("harness-a") && !real.contains("harness"));
    assert!(real.contains("kind") && real.contains("done"));

    // A markdown anchor's double hyphen is not a flag.
    assert!(
        long_flags_in("[chain](#the-review-chain--when-doubt-earns-its-own-leaves)").is_empty()
    );
    assert_eq!(
        long_flags_in("`grove-llm leaf-add-chain [12] stem --kind design`"),
        ["kind"]
    );
}

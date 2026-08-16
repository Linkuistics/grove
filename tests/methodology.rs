//! Claims about the **real corpus** — what `content/` is, and what a session
//! reading the provisioned skill will find there.
//!
//! Every claim here needs the shipped corpus rather than a fixture, and that is
//! the whole membership rule for this file. There is nothing else to assert
//! *about* the embed: it is plain markdown, carried whole, with no grammar over
//! it and no verb serving it by the piece. What used to live here — a unit
//! listing's field grammar, a byte-exact fetch, an unknown-id exit code, the
//! environment a lookup had to answer in — went with the machinery those were
//! contracts of.
//!
//! Two of the survivors are the checks the **build gate** used to make, moved to
//! where a claim about the corpus's content belongs: every kind's reference file
//! exists and is reachable from the routing table, and `SKILL.md` stays inside
//! its budget. The other two are the pair that make the build boundary safe —
//! the methodology instructs no `grove-llm` verb the embedded CLI lacks, and the
//! flat verb surface that makes that comparison mean what it says.

use clap::CommandFactory;
use grove::methodology;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use tempfile::TempDir;

/// **The linked embed and `content/` on disk are the same tree.**
///
/// `include_dir!` reads `content/` at compile time and, on stable Rust, does not
/// register those reads with Cargo's change tracker — `build.rs`'s per-file
/// `rerun-if-changed` walk is the only thing that makes an edit, an addition or
/// a removal reach the binary. That walk is now the **whole** of the build
/// script, so nothing else would notice it going wrong; this is what does.
///
/// The two sides are gathered by different traversals on purpose. `on_disk`
/// walks the directory with `fs::read_dir`; `embedded` walks `include_dir`'s
/// linked tree. A stale embed shows up as a difference between them, which a
/// single traversal compared against itself could never produce.
#[test]
fn the_linked_embed_carries_every_markdown_file_on_disk() {
    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let mut on_disk = BTreeSet::new();
    collect_markdown_paths(&content, &content, &mut on_disk);

    let embedded: BTreeSet<String> = methodology::markdown_files()
        .expect("the real embed must be readable")
        .into_iter()
        .map(|(path, _)| path)
        .collect();

    assert_eq!(
        embedded, on_disk,
        "the linked embed and content/ on disk disagree. A file on disk and not \
         in the embed means `build.rs`'s change tracking missed it and the \
         binary is carrying a stale corpus; a file in the embed and not on disk \
         means a deletion has not been rebuilt."
    );
    assert!(
        !on_disk.is_empty(),
        "an empty corpus would satisfy the equality above without asserting anything"
    );
}

/// **The routing table is the first screen, and every row it prints resolves.**
///
/// The opening routes rather than introduces: a session that arrived by
/// description match rather than by a mandate still lands in its kind's
/// reference file (`docs/specs/skill-delivered-methodology.md`, *How `SKILL.md`
/// opens*). A row naming a file that is not there sends exactly that session
/// nowhere, and prose cannot notice.
///
/// Ten rows for nineteen kinds, because the corpus already treats each family as
/// one unit — the five `review-*` kinds share a file, as do the five
/// `integrate-review-*` and the two research producers. The **map** the driver
/// will own is an exhaustive `match` over the kind enum and is checked where it
/// is written; this checks the table a human reads.
///
/// **The pairing is what is asserted, not the path set.** Collecting the
/// right-hand paths alone would discard the kind column and collapse duplicate
/// rows, so a table with a kind missing, a kind twice, an extra row, or a path
/// against the wrong kind would still pass while sending a session to the wrong
/// discipline — which is the entire failure this test exists to catch. The rows
/// are therefore compared as an ordered sequence of `(kind label, path)`.
#[test]
fn the_skills_routing_table_names_a_reference_file_that_exists() {
    let content = Path::new(env!("CARGO_MANIFEST_DIR")).join("content");
    let skill =
        fs::read_to_string(content.join("SKILL.md")).expect("content/SKILL.md must be readable");

    // A row is `| <kind label> | `references/<file>` |`; the header and its
    // `|---|---|` rule carry no backticked path and drop out here.
    let routed: Vec<(String, String)> = skill
        .lines()
        .skip_while(|line| !line.starts_with("## Read your kind's reference file first"))
        .take_while(|line| !line.starts_with("## The spine"))
        .filter(|line| line.starts_with('|'))
        .filter_map(|line| {
            let mut cells = line.trim_matches('|').split('|').map(str::trim);
            let kind = cells.next()?;
            let path = cells
                .next()?
                .strip_prefix("`references/")?
                .strip_suffix('`')?;
            Some((kind.to_owned(), path.to_owned()))
        })
        .collect();

    let expected: Vec<(String, String)> = [
        ("`requirements`", "requirements.md"),
        ("`design`", "design.md"),
        ("`planning`", "planning.md"),
        ("`prototype`", "prototype.md"),
        ("`impl`", "impl.md"),
        ("the five `review-*` kinds", "review.md"),
        ("the five `integrate-review-*` kinds", "integrate-review.md"),
        ("`research-a`, `research-b`", "research.md"),
        ("`combine-research`", "combine-research.md"),
        ("`finish`", "finish.md"),
    ]
    .into_iter()
    .map(|(kind, path)| (kind.to_owned(), path.to_owned()))
    .collect();

    assert_eq!(
        routed, expected,
        "content/SKILL.md's routing table must carry exactly these ten \
         kind→reference-file rows, in this order — a session that arrived \
         without a mandate reads the left column to find the right one"
    );
    for (kind, path) in &routed {
        assert!(
            content.join("references").join(path).is_file(),
            "content/SKILL.md routes {kind} to references/{path}, which does not exist"
        );
    }
}

/// **The house progressive-disclosure ceiling, on the body rather than the file.**
///
/// The spec measures `SKILL.md`'s *body* — the frontmatter is a routing header a
/// harness reads, not guidance a session reads, and counting it would let a
/// description rewrite eat into the ceiling
/// (`docs/specs/skill-delivered-methodology.md`, *Scenario: body budget*).
///
/// 500 is a **ceiling**, not the rewrite's target: the corpus rewrite estimates
/// ~200 lines, so a body approaching this number has re-grown procedure that the
/// split moved out. Like the loop-section alarm below, it establishes nothing
/// semantic — *no procedure in `SKILL.md`* has no classifier now that the unit
/// markers are deleted, and is a review obligation with its own scenario. **This
/// passing is not evidence for it.**
///
/// It is also, with the routing check above, what the deleted **build gate**
/// bought: a corpus malformation caught by the suite instead of by `cargo
/// build`. That is a weaker instrument by design — a contributor sees it at
/// `cargo test` rather than at compile — and it is the right one for a claim
/// about prose, which was never what a grammar gate could decide.
#[test]
fn the_skill_body_fits_the_progressive_disclosure_ceiling() {
    const LIMIT: usize = 500;
    let skill = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("content/SKILL.md"))
        .expect("content/SKILL.md must be readable");

    let measured = body_lines(&skill).expect("content/SKILL.md must have YAML frontmatter");
    assert!(
        measured <= LIMIT,
        "content/SKILL.md's body is {measured} lines against a ceiling of {LIMIT}. \
         Move a procedure into the reference file its condition names rather than \
         raising the ceiling."
    );

    // The control. A measure that cannot fail is worth nothing, and one that
    // silently returned zero on an unrecognised header would pass forever.
    let oversized = format!("---\nname: x\n---\n{}", "x\n".repeat(LIMIT));
    assert_eq!(body_lines(&oversized), Some(LIMIT));
    assert_eq!(body_lines("no frontmatter here\n"), None);
}

/// Lines after the closing `---` of YAML frontmatter. `None` when the document
/// does not open with frontmatter, so a stripped header fails rather than
/// measuring the whole file.
fn body_lines(text: &str) -> Option<usize> {
    let mut lines = text.lines();
    (lines.next()? == "---").then_some(())?;
    let mut lines = lines.skip_while(|line| *line != "---");
    lines.next()?;
    Some(lines.count())
}

/// **Constraint 7 made checkable**, and the measure is the whole claim.
///
/// "One page of rules" is unmeasurable as written, so the spec fixes a measure a
/// reader can recompute: the lines from a section's heading to the next heading
/// of the same level, blank lines included
/// (`docs/specs/skill-delivered-methodology.md`, *Scenario: the loop section fits
/// a page*). 100 is the **alarm** on a narrative the rewrite estimates at ~80,
/// not a budget the prose was fitted to — a loop section that reaches it has
/// grown a third of a page since the split, which is the point at which growth
/// has become visible.
///
/// It establishes nothing semantic. That `SKILL.md` states conditions and no
/// procedure has no classifier now that the unit markers are deleted, and is a
/// review obligation with its own scenario; a line count passing says nothing
/// about it.
#[test]
fn the_loop_section_of_the_skill_fits_a_page() {
    const LIMIT: usize = 100;
    let skill = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("content/SKILL.md"))
        .expect("content/SKILL.md must be readable");

    let measured =
        section_lines(&skill, "## The loop").expect("content/SKILL.md must have a loop section");
    assert!(
        measured <= LIMIT,
        "content/SKILL.md's loop section is {measured} lines against an alarm of \
         {LIMIT}. Move a procedure into the reference file its condition names \
         rather than trimming the alarm."
    );

    // The control. A measure that cannot fail is worth nothing, and this one
    // would silently pass on any heading it failed to find if it returned zero.
    let oversized = format!("## The loop\n{}## Artifacts\n", "x\n".repeat(LIMIT));
    assert_eq!(section_lines(&oversized, "## The loop"), Some(LIMIT + 1));
    assert_eq!(section_lines("# nothing here\n", "## The loop"), None);
}

/// Lines from `heading` to the line before the next heading of the same level,
/// or to end of text. `None` when the heading is absent, so a renamed section
/// fails rather than measuring zero.
fn section_lines(text: &str, heading: &str) -> Option<usize> {
    let level = heading
        .chars()
        .take_while(|character| *character == '#')
        .count();
    let opener = format!("{} ", "#".repeat(level));
    let mut lines = text.lines().skip_while(|line| *line != heading).peekable();
    lines.peek()?;
    Some(
        1 + lines
            .skip(1)
            .take_while(|line| !line.starts_with(&opener))
            .count(),
    )
}

fn collect_markdown_paths(root: &Path, dir: &Path, out: &mut BTreeSet<String>) {
    for entry in fs::read_dir(dir).expect("content/ must be readable") {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_markdown_paths(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).unwrap();
            out.insert(relative.to_string_lossy().into_owned());
        }
    }
}

/// **`content/SKILL.md` must reach a harness as a skill.**
///
/// Its leading `---` block is what every harness reads to discover the
/// provisioned skill — `name:` and `description:` are the entry a session sees
/// in its skill list. Provisioning copies the embed verbatim, so the only way to
/// break this is to edit the file; asserting it on the **extracted** copy is
/// what makes the claim about what a harness actually reads rather than about
/// what the embed happens to hold.
///
/// The old third limb — that the body then opened on a file directive and a unit
/// marker — went with the markers. Nothing stands in for it, because there is
/// nothing left for the frontmatter to be distinguished *from*: the body is
/// prose from its first byte.
#[test]
fn the_skill_frontmatter_survives_provisioning() {
    let dest = TempDir::new().unwrap();
    grove::provision::provision_into(dest.path()).unwrap();

    let skill = fs::read_to_string(dest.path().join("SKILL.md")).unwrap();
    let mut lines = skill.lines();
    assert_eq!(
        lines.next(),
        Some("---"),
        "the provisioned SKILL.md must still open with its frontmatter block"
    );
    let front: Vec<&str> = lines.by_ref().take_while(|line| *line != "---").collect();
    assert!(
        front.iter().any(|line| line.starts_with("name:"))
            && front.iter().any(|line| line.starts_with("description:")),
        "the two fields a harness discovers the skill by must survive: {front:?}"
    );
}

// -- The embed instructs no verb the CLI lacks ------------------------------
//
// **Relocated from `tests/provision.rs`, and improved by the move.** Both checks
// below are claims about the *embed*, not about provisioning — the architecture
// already names the first as the enforceable half of the build boundary — and
// the grove that retires provisioning deletes their old home. They used to scan
// a **provisioned extraction** of the embed, which meant spending a tempdir and
// a sweep to obtain bytes the binary already carries. They now scan the embed
// itself, through `methodology::markdown_files`.

/// **The methodology and the CLI it instructs ship as one artifact**, and this
/// is the invariant that makes the build boundary safe rather than merely real.
///
/// `content/` is embedded with `include_dir!`, so a session reads the
/// methodology its own `grove` binary was **built** with — never the one
/// committed in some working tree (`docs/ARCHITECTURE.md` §Embedded
/// methodology). The tempting reading of that is a defect: a grove *of grove*
/// commits `content/SKILL.md` and the next session still reads the old one. It
/// is a feature, because **any** skew between the skill and the CLI it
/// instructs is unsafe, in both directions — a newer skill names verbs added
/// since the binary, an older one names verbs *removed* since the skill. This
/// repository holds the second case's ingredients: the `v17.0.0` skill
/// instructs `leaf-add-chain`, removed after that tag, so that skill beside any
/// later binary is broken. So there is no safe direction to drift in, and one
/// embed per build is what keeps the drift at zero.
///
/// No test can see a future build, so "the installed skill is current" is not
/// the statable claim. **This one is**: the embed is internally consistent —
/// every `grove-llm` verb the embedded methodology instructs is a verb the
/// embedded CLI exposes. A binary that ships therefore cannot hand a session a
/// verb it lacks, whatever the working tree it was built from happened to say.
///
/// Mentions come from the corpus rather than from a list, so a verb invented in
/// prose — or one deleted from the CLI while an instruction survives — fails
/// here without anyone having remembered to add it. The reading rule lives on
/// [`scan_instructed_verbs`] and the grain both sides are compared at lives on
/// [`exposed_verbs`]; each is pinned by its own control below, because a claim
/// this universal is only as good as the reader that gathers its corpus.
#[test]
fn the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks() {
    let exposed = exposed_verbs();
    let mut instructed = Vec::new();
    for (path, text) in methodology::markdown_files().expect("the real embed must be readable") {
        scan_instructed_verbs(&path, text, &mut instructed);
    }

    let unknown: Vec<&(String, usize, String)> = instructed
        .iter()
        .filter(|(_, _, verb)| !exposed.contains(verb))
        .collect();
    assert!(
        unknown.is_empty(),
        "the embedded methodology instructs {} verb(s) the embedded `grove-llm` \
         does not expose — a shipped binary would hand a session a call that \
         cannot succeed. Exposed: {exposed:?}. Offending mentions: {unknown:?}",
        unknown.len()
    );

    // Positive control, pinned complete rather than floored. A scan that quietly
    // matches nothing satisfies the assertion above forever, and a *floor* only
    // bounds how much it may quietly lose: `>= 8` against a corpus of eleven let
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
        "the embedded methodology's instructed-verb set moved. Verbs found but \
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

/// Every `grove-llm` verb the embedded methodology instructs, named in full so
/// the corpus is pinned rather than merely floored — see the completeness
/// control in [`the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks`].
///
/// **Eleven**, and `methodology` is the one that left: it was the twelfth for as
/// long as `content/MANDATE.md` told a session that a `defers=<id>` marker had a
/// body `grove-llm methodology <id>` would serve. The corpus stopped reaching for
/// it when it began routing by **path** — a condition names the reference file
/// carrying its procedure, and a session opens it — and the verb itself is now
/// off the CLI with the rest of the mandate machinery.
///
/// This check is **more** load-bearing for that, not less: the skill is once
/// again the only thing teaching a session which verbs exist, so a verb it names
/// wrongly is a call a session cannot make and has no second source to correct
/// it from.
const INSTRUCTED_VERBS: [&str; 11] = [
    "brief-chain",
    "complete",
    "finish-commit",
    "leaf-add",
    "leaf-add-pair",
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
    grove::llm_cli::Cli::command()
        .get_subcommands()
        .map(|sub| sub.get_name().to_string())
        .collect()
}

/// `--content-hash` is a **flag**, and this is what that buys: it stays out of
/// the agent grammar. `exposed_verbs` reads subcommands, so a verb would have to
/// be instructed somewhere or fail the completeness control in
/// [`the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks`] — and
/// instructing it would be a lie, because no session ever calls it.
///
/// It followed the two helpers here from `tests/provision.rs`: its subject is
/// the agent verb surface and the instruction scanner, both of which now live
/// beside it, and the identity it is about outlives provisioning anyway.
#[test]
fn the_identity_flag_is_not_part_of_the_agent_verb_surface() {
    assert!(
        !exposed_verbs().contains("content-hash"),
        "`--content-hash` must stay a flag: it is metadata the driver reads from \
         outside any session, not something a session invokes"
    );
    let mut instructed = Vec::new();
    scan_instructed_verbs(
        "FLAG.md",
        "The driver asks `grove-llm --content-hash` for the build's identity.",
        &mut instructed,
    );
    assert!(
        instructed.is_empty(),
        "a flag mention must not read as an instructed verb: {instructed:?}"
    );
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
    let nested: Vec<String> = grove::llm_cli::Cli::command()
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
/// `content/SKILL.md` splits at least one invocation across a line break, and a
/// line-based scan drops it silently — the exact failure mode that makes a
/// removal check worthless.
///
/// It is also why the corpus is a **file** rather than a unit. Units partition a
/// file's body *past its directive*, so scanning them one at a time would put a
/// boundary through the middle of a wrapped invocation — and a marker line always
/// follows such a boundary, so the loss would land in the fail-open direction. A
/// file also carries the two regions no unit covers, which a scan of units would
/// skip; neither can hold an instruction today, and a corpus that cannot lose one
/// tomorrow is the cheaper claim.
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

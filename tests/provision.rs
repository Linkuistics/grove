// Integration tests for the global-skill provisioning (src/provision.rs):
// the `grove` binary embeds `content/` and extracts it idempotently to the
// global personal skill dir. These drive the *real* embed via the public
// `provision_into`, against a throwaway tempdir, and prove the three contract
// scenarios named in the leaf brief: extract-fresh, extract-idempotent,
// extract-on-change (self-extension-core-and-methodology / task-tree-scheme).

mod support;

use clap::CommandFactory;
use grove::provision::provision_target;
use grove::provision::{provision_into, METHODOLOGY_IDENTITY, STAMP_FILE};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::sync::Mutex;
use support::EnvGuard;
use tempfile::TempDir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

/// A phrase that exists only in `content/`, used to ask a *binary* whether it
/// carries the embed. Asserted present in `grove` as well as absent from
/// `grove-llm`, so a phrase that quietly leaves the methodology fails here
/// rather than manufacturing a clean answer for the interesting half.
const CONTENT_MARKER: &str = "hierarchical, self-extending workstreams";

/// **Only `grove` links the embed**, which is the whole reason the methodology
/// identity is a compile-time constant rather than a runtime hash.
///
/// `grove-llm` needs to name its own identity — the driver asks it for the pair
/// check, and it compares its own against every installed skill directory's
/// stamp — but it never *extracts* content. Calling `provision::content_hash`
/// there would pull `include_dir!`'s static into the second binary and cost it
/// the size of `content/` for a value known when it was built.
///
/// Asserted on the linked artifacts because that is where the claim lives: no
/// amount of reading `provision.rs` shows what the linker kept.
#[test]
fn only_grove_carries_the_embedded_methodology() {
    let carries = |binary: &str| {
        let bytes = fs::read(binary).unwrap();
        bytes
            .windows(CONTENT_MARKER.len())
            .any(|window| window == CONTENT_MARKER.as_bytes())
    };

    assert!(
        carries(env!("CARGO_BIN_EXE_grove")),
        "`grove` extracts the methodology, so it must carry it — if this fails, \
         {CONTENT_MARKER:?} has left content/ and the marker needs updating \
         before the claim below means anything"
    );
    assert!(
        !carries(env!("CARGO_BIN_EXE_grove-llm")),
        "`grove-llm` now links the embedded content/. It only ever needs its \
         own methodology *identity*, which `build.rs` supplies as a constant; \
         something has reached for `provision`'s embed instead."
    );
}

/// The flag and the stamp are two views of one value, and the pair check is only
/// as good as their agreement: the driver compares what a `grove-llm` *reports*
/// against what its paired `grove` *wrote*, so a divergence would make every
/// correctly paired machine look mismatched.
#[test]
fn the_reported_identity_is_the_one_provisioning_stamps() {
    let dest = TempDir::new().unwrap();
    provision_into(dest.path()).unwrap();
    let stamped = fs::read_to_string(dest.path().join(STAMP_FILE)).unwrap();

    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("--content-hash")
        .output()
        .unwrap();

    assert!(output.status.success());
    let reported = String::from_utf8(output.stdout).unwrap();
    assert_eq!(reported.trim(), stamped);
    assert_eq!(reported.trim(), METHODOLOGY_IDENTITY);
}

/// `--content-hash` is a **flag**, and this is what that buys: it stays out of
/// the agent grammar. `exposed_verbs` reads subcommands, so a verb would have to
/// be instructed somewhere or fail the completeness control in
/// [`the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks`] — and
/// instructing it would be a lie, because no session ever calls it.
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

/// The check inside the session — the only one a clobber landing *after* launch
/// can reach, and the one whose two operands are the ones that matter: the CLI
/// actually invoked, and the methodology actually on disk in front of it.
///
/// It warns and does nothing else. A refusal mid-task destroys more than the
/// mismatch does, and the session least able to absorb a hard stop is one
/// already holding uncommitted work — so the verb still answers, on stdout,
/// exit zero.
#[test]
fn a_foreign_stamp_warns_without_changing_what_a_verb_does() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let skill_dir = home.join(".claude/skills/grove");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(skill_dir.join(STAMP_FILE), "another-build").unwrap();
    let worktree = git_worktree_with_one_leaf(fixture.path());

    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("pick")
        .current_dir(&worktree)
        .env("HOME", &home)
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "a mismatch must not refuse: {stderr}"
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("01-impl-first-k1.md"),
        "the verb must still answer: {stderr}"
    );
    assert!(
        stderr.contains(skill_dir.to_str().unwrap())
            && stderr.contains("another-build")
            && stderr.contains(METHODOLOGY_IDENTITY),
        "the warning must name the directory and both identities: {stderr}"
    );
}

/// **Absence is not disagreement.** The claim on offer is "this directory
/// belongs to another build", and nothing can say that about a directory that
/// does not exist — an unprovisioned root, or a harness that is not installed,
/// is silent.
#[test]
fn an_unstamped_or_absent_skill_directory_is_silent() {
    let fixture = TempDir::new().unwrap();
    let worktree = git_worktree_with_one_leaf(fixture.path());

    let unstamped = fixture.path().join("unstamped-home");
    fs::create_dir_all(unstamped.join(".claude/skills/grove")).unwrap();
    let uninstalled = fixture.path().join("uninstalled-home");
    fs::create_dir_all(&uninstalled).unwrap();

    for home in [&unstamped, &uninstalled] {
        let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
            .arg("pick")
            .current_dir(&worktree)
            .env("HOME", home)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(output.status.success(), "{stderr}");
        assert!(
            stderr.is_empty(),
            "{} must produce no pairing warning, got: {stderr}",
            home.display()
        );
    }
}

/// A minimal current-format grove, enough for `pick` to have an answer.
fn git_worktree_with_one_leaf(fixture: &Path) -> std::path::PathBuf {
    let worktree = fixture.join("worktree");
    fs::create_dir_all(&worktree).unwrap();
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(&worktree)
        .status()
        .unwrap()
        .success());
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(grove.join("BRIEF.md"), "# worktree — brief\n").unwrap();
    fs::write(grove.join("01-impl-first-k1.md"), "# first-k1\n").unwrap();
    worktree
}

#[test]
fn extract_fresh_writes_the_full_content_tree() {
    let dest = TempDir::new().unwrap();

    let extracted = provision_into(dest.path()).unwrap();

    assert!(extracted, "a fresh dir must be extracted into");
    // The skill entrypoint, a prompt under a subdir, and a top-level format
    // guide all travel — i.e. the *whole* tree, nested dirs included.
    assert!(dest.path().join("SKILL.md").is_file());
    assert!(dest.path().join("prompts/continue.md").is_file());
    assert!(dest.path().join("driving.md").is_file());
    // A deeply-nested file (LICENSES/) confirms recursion to the leaves.
    assert!(dest.path().join("LICENSES").is_dir());
    // The stamp is written so the next launch can detect a warm dir.
    assert!(dest.path().join(STAMP_FILE).is_file());
}

/// The launcher surface, enumerated rather than listed. `start.md` and
/// `retire.md` died with the lifecycle verbs that launched them: one bare
/// command drives every kind of session now, so one launcher covers them all.
/// The claim committed here is therefore not "those two files are gone" — a
/// pair of `!exists()` assertions that would pass unchanged the day a third
/// launcher appears — but **exactly one launcher**, which fails on the next one
/// anybody adds.
///
/// Extracting the real embed settles both halves of the removal at once: what
/// travels inside the binary is precisely what lands in a skill dir, so a file
/// that is not embedded cannot be swept, and one that is swept was embedded.
#[test]
fn exactly_one_launcher_is_embedded_and_provisioned() {
    let dest = TempDir::new().unwrap();
    provision_into(dest.path()).unwrap();

    let mut prompts: Vec<String> = fs::read_dir(dest.path().join("prompts"))
        .unwrap()
        .map(|entry| entry.unwrap().file_name().to_string_lossy().into_owned())
        .collect();
    prompts.sort();
    assert_eq!(
        prompts,
        ["continue.md"],
        "the embedded methodology must carry one launcher and no lifecycle-verb prompts"
    );

    // ...and it is the same bytes the driver puts in front of every mandate, so
    // the provisioned copy cannot drift from the launched one.
    assert_eq!(
        fs::read_to_string(dest.path().join("prompts/continue.md")).unwrap(),
        grove::provision::continue_prompt().unwrap(),
        "the launcher the driver embeds in a session prompt is the one it sweeps out"
    );
}

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
/// Mentions come from the tree rather than from a list, so a verb invented in
/// prose — or one deleted from the CLI while an instruction survives — fails
/// here without anyone having remembered to add it. The reading rule lives on
/// [`scan_instructed_verbs`] and the grain both sides are compared at lives on
/// [`exposed_verbs`]; each is pinned by its own control below, because a claim
/// this universal is only as good as the reader that gathers its corpus.
#[test]
fn the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks() {
    let dest = TempDir::new().unwrap();
    provision_into(dest.path()).unwrap();

    let exposed = exposed_verbs();
    let mut instructed = Vec::new();
    collect_instructed_verbs(dest.path(), dest.path(), &mut instructed);

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
    // whole leaf came from: a removed constructor still instructed in prose.
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

/// Walk the provisioned tree — the real extracted embed, so what is scanned is
/// exactly what lands in a skill dir — collecting `(file, line, verb)` for every
/// instructed verb in every markdown file.
fn collect_instructed_verbs(
    root: &std::path::Path,
    dir: &std::path::Path,
    out: &mut InstructedVerbs,
) {
    let mut entries: Vec<_> = fs::read_dir(dir)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            collect_instructed_verbs(root, &path, out);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path.strip_prefix(root).unwrap().display().to_string();
            scan_instructed_verbs(&relative, &fs::read_to_string(&path).unwrap(), out);
        }
    }
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

#[test]
fn extract_is_idempotent_on_a_warm_dir() {
    let dest = TempDir::new().unwrap();

    assert!(provision_into(dest.path()).unwrap(), "first call extracts");
    let before = fs::read_to_string(dest.path().join("SKILL.md")).unwrap();

    assert!(
        !provision_into(dest.path()).unwrap(),
        "a warm dir with a matching stamp is a no-op"
    );
    let after = fs::read_to_string(dest.path().join("SKILL.md")).unwrap();
    assert_eq!(before, after, "warm no-op leaves content untouched");
}

#[test]
fn extract_re_extracts_and_clears_stale_files_when_the_stamp_mismatches() {
    let dest = TempDir::new().unwrap();
    provision_into(dest.path()).unwrap();

    // Simulate a dir produced by a *different* (older) embed: a stale stamp plus
    // a file the current embed does not contain.
    let stale = dest.path().join("STALE.md");
    fs::write(&stale, "left over from an older embed").unwrap();
    fs::write(
        dest.path().join(STAMP_FILE),
        "0000000000000000000000000000000000000000000000000000000000000000",
    )
    .unwrap();

    let extracted = provision_into(dest.path()).unwrap();

    assert!(
        extracted,
        "a changed embed (mismatched stamp) must re-extract"
    );
    assert!(!stale.exists(), "files from the old embed must be cleared");
    assert!(
        dest.path().join("SKILL.md").is_file(),
        "the current embed is re-written"
    );
}

#[test]
fn provision_replaces_a_symlinked_grove_entry_with_a_real_dir() {
    let tmp = TempDir::new().unwrap();
    // Simulate today's layout: a real provisioned dir + a skills entry that
    // symlinks to it (the current ~/.codex/skills/grove and
    // ~/.pi/agent/skills/grove setups).
    let real = tmp.path().join("claude-skills/grove");
    fs::create_dir_all(&real).unwrap();
    provision_into(&real).unwrap();
    let linked = tmp.path().join("codex-skills/grove");
    fs::create_dir_all(linked.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&real, &linked).unwrap();

    let wrote = provision_target(&linked).unwrap();

    // `real` already carries a matching stamp (from `provision_into` above),
    // so this is the property that's actually load-bearing here: a symlink
    // must be unconditionally replaced even when its *target* looks warm —
    // `sync_to_stamp`'s own stamp comparison would otherwise treat a
    // stamp-matching symlink as already-done and never replace it, which
    // would leave the cross-harness link farm in place forever.
    assert!(wrote, "a symlinked entry is replaced, not treated as warm");
    let meta = fs::symlink_metadata(&linked).unwrap();
    assert!(meta.is_dir(), "the symlink becomes a real directory");
    assert!(linked.join("SKILL.md").is_file());
    // Regression guard, not a live-risk check (branch-review-k14 T2): removing
    // the symlink must never affect `real`'s own content. This can't currently
    // be violated by *how* the symlink is removed — `std::fs::remove_dir_all`
    // documents (see its "TOCTOU race conditions" section, stable since 1.0.0)
    // that on every platform except Miri/QNX/Redox/VxWorks (none grove targets)
    // it `lstat`s a top-level symlink argument and unlinks rather than
    // recursing, exactly like `remove_file` — verified against this repo's
    // own std (rustc 1.97.1, `sys/fs/unix.rs::remove_dir_all_modern` and the
    // `sys/fs/common.rs` fallback both special-case it). So `rust-version`
    // needs no bump for this, whatever it currently says: the guarantee
    // predates any floor Grove could claim. Kept as documentation of the
    // invariant, not as a mutation-resistant check.
    assert!(
        real.join("SKILL.md").is_file(),
        "replacing the symlink must never delete through it"
    );
}

#[test]
fn provision_refuses_a_foreign_directory() {
    let tmp = TempDir::new().unwrap();
    let dest = tmp.path().join("skills/grove");
    fs::create_dir_all(&dest).unwrap();
    fs::write(dest.join("precious.txt"), "user data, not ours").unwrap();

    let err = provision_target(&dest).unwrap_err().to_string();

    // branch-review-k14 T8: the message interpolates `dest`, never the
    // offending file's own name — an `err.contains("precious")` disjunct here
    // would never be true and silently widen the assertion; assert only the
    // clause the message actually satisfies.
    assert!(
        err.contains("not a grove-provisioned"),
        "must refuse to clobber a dir grove didn't write (err: {err})"
    );
    assert!(
        dest.join("precious.txt").is_file(),
        "the foreign content must be untouched"
    );
}

// Provisioning has no primary any more: configured commands are opaque, so the
// driver cannot know which harness a session eventually reaches and does not
// guess. It refreshes every *installed* known root instead — presence of the
// home marker is the whole rule, and an absent root is skipped rather than
// created.
#[test]
fn provision_installed_sweeps_every_installed_harness_root() {
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();
    fs::create_dir_all(home.path().join(".claude")).unwrap();
    fs::create_dir_all(home.path().join(".pi")).unwrap();

    let mut env = EnvGuard::new();
    env.set("HOME", home.path());

    grove::provision::provision_installed().unwrap();

    assert!(
        home.path().join(".claude/skills/grove/SKILL.md").is_file(),
        "an installed claude root is provisioned"
    );
    assert!(
        home.path()
            .join(".pi/agent/skills/grove/SKILL.md")
            .is_file(),
        "an installed pi root is provisioned — note the agent/ nesting"
    );
    assert!(
        !home.path().join(".codex").exists(),
        "an absent harness root is skipped, not created"
    );
}

// There is no destination override to route around this. The foreign-directory
// guard is what stands between a user's own `skills/grove` and an extraction
// over the top of it, so provisioning refuses rather than proceeding — and the
// refusal reaches the driver, which never owns the working tree.
#[test]
fn provision_installed_refuses_a_foreign_skill_directory() {
    let _g = support::lock_env(&ENV_LOCK);
    let home = TempDir::new().unwrap();
    fs::create_dir_all(home.path().join(".pi/agent/skills/grove")).unwrap();
    fs::write(
        home.path().join(".pi/agent/skills/grove/precious.txt"),
        "user data, not ours",
    )
    .unwrap();

    let mut env = EnvGuard::new();
    env.set("HOME", home.path());

    let err = grove::provision::provision_installed()
        .unwrap_err()
        .to_string();

    assert!(
        err.contains("not a grove-provisioned"),
        "a foreign dir must bail (err: {err})"
    );
    assert!(
        home.path()
            .join(".pi/agent/skills/grove/precious.txt")
            .is_file(),
        "the foreign content must be untouched"
    );
}

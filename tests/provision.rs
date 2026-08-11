// Integration tests for the global-skill provisioning (src/provision.rs):
// the `grove` binary embeds `content/` and extracts it idempotently to the
// global personal skill dir. These drive the *real* embed via the public
// `provision_into`, against a throwaway tempdir, and prove the three contract
// scenarios named in the leaf brief: extract-fresh, extract-idempotent,
// extract-on-change (self-extension-core-and-methodology / task-tree-scheme).

mod support;

use clap::CommandFactory;
use grove::provision::provision_target;
use grove::provision::{provision_into, STAMP_FILE};
use std::collections::BTreeSet;
use std::fs;
use std::sync::Mutex;
use support::EnvGuard;
use tempfile::TempDir;

static ENV_LOCK: Mutex<()> = Mutex::new(());

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
/// [`scan_instructed_verbs`]: `grove-llm`, whitespace, then a lowercase token.
/// Prose naming the bare binary writes it in backticks, which the rule skips
/// already, since the follower is then a backtick rather than whitespace.
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

    // Positive controls. A scan that quietly matches nothing satisfies the
    // assertion above forever, so the floor is asserted rather than assumed —
    // on distinct verbs, not raw mentions, so re-wording one paragraph cannot
    // drift the corpus below it while genuinely instructing the same set.
    let distinct: BTreeSet<&String> = instructed.iter().map(|(_, _, verb)| verb).collect();
    assert!(
        distinct.len() >= 8,
        "the scan found only {} distinct instructed verb(s) ({distinct:?}) — the \
         methodology instructs far more than that, so the scanner is broken \
         rather than the corpus clean",
        distinct.len()
    );
    assert!(
        distinct.contains(&"leaf-add".to_string()),
        "`leaf-add` is the verb the flat/lazy chain is built from; a scan that \
         misses it is reading something other than the methodology"
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

    // The reading rule, pinned on the four shapes that decide it. The wrapped
    // case is here because the corpus really does wrap an invocation and the
    // first version of this scan — line by line — dropped it without a word.
    let mut shapes = Vec::new();
    scan_instructed_verbs(
        "SHAPES.md",
        "Prefer `grove-llm\nresolve <ref>` for a durable reference.\n\
         The `grove-llm` binary is agent-facing.\n\
         Run `grove-llm --version` to check.\n\
         Then: grove-llm leaf-retire <leaf-path>\n\
         A block ending in grove-llm\n\n\
         paragraph text follows.\n",
        &mut shapes,
    );
    assert_eq!(
        shapes
            .iter()
            .map(|(_, line, verb)| (*line, verb.as_str()))
            .collect::<Vec<_>>(),
        [(1, "resolve"), (5, "leaf-retire")],
        "a wrapped invocation counts and reports the line it starts on; bare \
         `grove-llm` in prose, a flag, and a blank-line gap all do not"
    );
}

/// Every subcommand `grove-llm` exposes, from clap's model rather than from
/// `--help` — the question is a fact about the command tree, not about a
/// rendering (the reasoning `tests/help_surfaces.rs` sets out at length).
fn exposed_verbs() -> BTreeSet<String> {
    fn walk(command: &clap::Command, out: &mut BTreeSet<String>) {
        for sub in command.get_subcommands() {
            out.insert(sub.get_name().to_string());
            walk(sub, out);
        }
    }
    let mut out = BTreeSet::new();
    walk(&grove::llm_cli::Cli::command(), &mut out);
    out
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

/// `grove-llm`, whitespace, then a lowercase token — wherever it occurs: inline
/// in backticks, in a fenced or indented code block, or in a mermaid node label,
/// all four of which the corpus uses.
///
/// **Scanned over the whole file rather than line by line**, because prose wraps:
/// `content/SKILL.md` splits at least one invocation across a line break, and a
/// line-based scan drops it silently — the exact failure mode that makes a
/// removal check worthless. The separator may therefore include one newline, but
/// not a blank line, so a bare `grove-llm` ending a code block cannot swallow the
/// first word of the paragraph after it.
///
/// A leading `-` is deliberately not accepted as the token's first character, so
/// a future `grove-llm --version` mention reads as a flag rather than as a verb
/// named `--version`. A non-whitespace follower is skipped outright, which is
/// what excludes prose naming the bare binary in backticks.
fn scan_instructed_verbs(file: &str, text: &str, out: &mut InstructedVerbs) {
    const MARKER: &str = "grove-llm";
    let mut offset = 0;
    while let Some(at) = text[offset..].find(MARKER) {
        let start = offset + at;
        offset = start + MARKER.len();
        let after = &text[offset..];
        let separator: &str = after
            .split_terminator(|character: char| !character.is_whitespace())
            .next()
            .unwrap_or("");
        if separator.is_empty() || separator.matches('\n').count() > 1 {
            continue;
        }
        let rest = &after[separator.len()..];
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

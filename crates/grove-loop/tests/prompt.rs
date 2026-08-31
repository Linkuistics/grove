//! The **guaranteed core** — three driver-authored parts, and nothing else.
//!
//! Every interesting check runs through the `prompt` seam. The driver's launch
//! path is a thin wrapper covered by the existing loop-driver seam, so nothing
//! here spawns one.
//!
//! **What is mechanically checkable is narrower than what the design claims,
//! and the split is stated rather than blurred.** The three-part shape, the
//! published version, the skill-name coupling and the size alarm are all
//! checkable. What the wording *says* — whether it gets a session to read the
//! skill — is not, and is carried by `docs/research/wording-micro-test.md`.
//!
//! **The one coupling this suite exists for changed subject at
//! `prompt-names-the-kind-k18`.** It used to be the prompt against the embedded
//! corpus: a skill name and nineteen reference paths, both asserted against
//! `content/`. Composition reads nothing now, so what is left to hold in step is
//! the prompt against the **shipped plugin** — every `grove-<kind>` the prompt
//! can name has to be a skill the plugin ships, under the plugin the prompt
//! namespaces it with. That is asserted here over the working tree's own
//! `plugins/grove`, which is the closest thing to the installed set a `cargo
//! test` can reach; what a *machine* has installed is outside any suite, and the
//! methodology answers it by stating the version grove is.
//!
//! **Every generated claim carries a control.** A sweep that cannot fail is
//! worth nothing, so the skill-set claim is shown failing on a kind the plugin
//! does not ship, and the size alarm on a synthetic oversized prompt.

mod support;

use grove_loop::prompt::{self, Mandate};
use grove_loop::{Handle, Kind};
use jj_workspace::Workspace;
use std::collections::BTreeSet;
use std::path::PathBuf;

/// A [`Kind`] for a test that needs one, by its label.
///
/// A kind is an **open token** since `open-kind-k20`, so a test names the token
/// it means rather than a variant, and an invalid one is a test bug that panics
/// here rather than a compile error somewhere else.
fn a_kind(label: &str) -> Kind {
    Kind::new(label).expect("a test kind must be well-formed")
}

/// The alarm on the too-late test, per kind — **not a budget the design was
/// fitted to**.
///
/// Measured composition leaves well over half of it free. A core that reaches
/// 4 KiB has gained roughly a page of prose, which the too-late test admits only
/// if the driver has acquired a new runtime fact — a rare and visible event.
/// Everything else that could push it there is importance-as-a-criterion, which
/// is exactly what this exists to see.
///
/// It lives in the suite rather than the build because it measures a judgement
/// against an admittedly arbitrary number, and failing a contributor's build on
/// that is a gate this design is careful not to erect.
const SIZE_ALARM: usize = 4 * 1024;

/// The launch-varying values held fixed, so a composition is reproducible on any
/// machine.
const FIXTURE_HANDLE: &str = "prompt-names-the-kind-k18";
const FIXTURE_VERSION: &str = "19.4.0";

/// The plugin directory this repository ships, and the source of truth for which
/// `grove-<kind>` skills exist.
fn plugin_dir() -> PathBuf {
    support::repo_root().join("plugins/grove")
}

/// A resolved workspace to state the version control from.
///
/// The repository itself is one — the suite runs inside it — so no fixture tree
/// is scaffolded for a value the prompt only renders.
fn workspace() -> Workspace {
    Workspace::resolve(&support::repo_root())
        .expect("this repository is a jj workspace, which is what the prompt states")
}

fn compose_with(kind: &Kind, handle: &str) -> String {
    let handle = Handle::parse(handle).expect("the fixture handle must parse");
    prompt::compose(&Mandate {
        handle: &handle,
        kind,
        workspace: &workspace(),
        version: FIXTURE_VERSION,
    })
}

fn compose(kind: &Kind) -> String {
    compose_with(kind, FIXTURE_HANDLE)
}

// -- The shape ---------------------------------------------------------------

/// **The three parts, in order** — and the fourth claim is the one a `contains`
/// sweep cannot make: that there is *nothing else*.
///
/// Asserted structurally rather than by substring. The prompt is reassembled
/// from its parts and compared whole, so a driver-authored sentence added
/// anywhere — an introduction, a footer, a restated rule — fails here even
/// though every `contains` below would still pass.
///
/// Order is the session's own timeline: the load instruction first because it is
/// the first action, the runtime facts, and the signalling contract last because
/// it describes the last action.
#[test]
fn the_prompt_is_three_parts_in_the_sessions_own_timeline_order() {
    let root = workspace().root().display().to_string();

    for kind in shipped_kinds() {
        let prompt = compose(&kind);
        let skill = prompt::skill_name(&kind);

        let load_ends = prompt
            .find("\nGrove mandate: ")
            .unwrap_or_else(|| panic!("`{}`'s prompt states no handle", kind.label()));
        let (load, rest) = prompt.split_at(load_ends + 1);

        assert!(
            load.starts_with(&format!("**Load the `{skill}` skill now**")),
            "`{}`'s prompt must open with the imperative naming its one target, not with \
             anything else:\n{load}",
            kind.label()
        );
        assert!(
            load.contains(&format!("that is `{}:{skill}`", prompt::PLUGIN)),
            "and it must give the namespaced spelling of the same target in the same \
             imperative, which is what lets grove branch on no harness:\n{load}"
        );
        assert!(
            load.contains("Do this **first**, before anything else you might reach for: before reading the\ntask file, before running any `grove-llm` verb, before looking at `.grove/`,\nbefore inspecting the working tree, and before answering a question."),
            "the ordering clause must enumerate the tempting alternatives — without the \
             enumeration it is advice, and the enumeration is what the micro-test measured"
        );

        let expected_facts = format!(
            "Grove mandate: the leaf selected for this session is `{FIXTURE_HANDLE}`.\n\n\
             Version control: this working tree is jj-enabled (jj workspace root: \
             `{root}`).\n\n\
             Grove version: {FIXTURE_VERSION}.\n"
        );
        // Whole-string equality, not a prefix or a `contains`: the fourth claim
        // this test makes is that there is *nothing else*, and a footer appended
        // after the contract passes every other assertion in this file —
        // including the cross-kind identity check, since a footer identical for
        // all nineteen kinds is identical for all nineteen kinds.
        assert_eq!(
            rest,
            format!("{expected_facts}\n{}", signalling_contract()),
            "`{}`'s prompt after the load instruction must be exactly the three runtime \
             facts and then the signalling contract — no introduction, no footer, and no \
             rule restated from the skill",
            kind.label()
        );
    }
}

/// **Every substitution is spent.**
///
/// A placeholder surviving into a session's prompt is the failure a `format!`
/// would have caught at compile time and a `replace` chain cannot, so it is
/// caught here instead — over the whole kind set, so a shape only one kind
/// reaches cannot hide. This is the guard the `#[cfg(test)]` module in
/// `src/prompt.rs` used to carry; it moved here with the rest of this file's
/// subject rather than going with the embed, because the composition it watches
/// did not stop being a `replace` chain.
#[test]
fn no_placeholder_survives_composition() {
    for kind in shipped_kinds() {
        let prompt = compose(&kind);
        assert!(
            !prompt.contains('{') && !prompt.contains('}'),
            "an unspent placeholder reached {}'s prompt:\n{prompt}",
            kind.label()
        );
    }
}

/// **The load instruction states that the kind was resolved and there is nothing
/// to select.**
///
/// This is the micro-test's load-bearing element in its own words, and it is the
/// half of part 1 that a check on the opening imperative does not reach: the
/// imperative names the target, this sentence is why the session has no reason
/// to defer opening it. Pinned by the kind's own label as well as the clause, so
/// a prompt that names one kind and tells the session it is another fails.
#[test]
fn the_load_instruction_says_the_kind_was_already_resolved() {
    for kind in shipped_kinds() {
        let prompt = compose(&kind);
        assert!(
            prompt.contains(&format!(
                "Your kind is `{}`; Grove resolved\nthat before this session existed, so \
                 there is no selection for you to make.",
                kind.label()
            )),
            "`{}`'s prompt no longer states that its kind was resolved before the session \
             existed. That is the element `docs/research/wording-micro-test.md` measured as \
             load-bearing — a session with a selection to make defers it — so a rewrite that \
             drops it has to restate the evidence against it.\n{prompt}",
            kind.label()
        );
    }
}

/// **The signalling contract's load-bearing clauses, pinned.**
///
/// The contract is the one part of the prompt with no counterpart anywhere: no
/// skill states it, so nothing else can fail when it goes. Three clauses carry
/// the design and are pinned by their own words rather than by the part's size.
///
/// **The first is the one the `finish` hazard rests on.** The prompt states an
/// ordinary default, and the only thing keeping that default from reading as an
/// imperative to a session whose ending is different is that the kind's own
/// ending is the sentence's object and the default is subordinate to it.
/// Deleting *your kind's skill states how this session ends* leaves the default
/// as the main verb, in the one channel a session cannot skip.
#[test]
fn the_signalling_contract_states_the_mechanism_and_defers_the_ending() {
    let contract = signalling_contract();
    for (clause, what) in [
        (
            "Your kind's skill states how this session ends",
            "that the ending is the kind's to state — without it the ordinary default below \
             is a bare imperative, and for `finish` it is the wrong one",
        ),
        (
            "last action — then do nothing else",
            "that the signal comes last — the driver ends the session once the file appears, \
             so anything after it may not run",
        ),
        (
            "A session that ends *without* signalling stops the loop instead",
            "that a session which never signals stops the loop — the fact that separates a \
             task you finished from a session that died",
        ),
    ] {
        assert!(
            contract.contains(clause),
            "the signalling contract no longer says \"{clause}\", so it no longer states \
             {what}. Nothing else in the corpus states it, so this is the only check there \
             is.\n---\n{contract}\n---"
        );
    }
}

/// The third part, sliced off a composed prompt rather than re-declared, so a
/// check on it cannot pass against a copy the driver does not send.
fn signalling_contract() -> String {
    let prompt = compose(&a_kind("impl"));
    let at = prompt
        .find("**Signal.**")
        .expect("every prompt ends on grove's signalling contract");
    prompt[at..].to_owned()
}

/// **The signalling contract is one text, identical for every kind.**
///
/// This is the deletion `prompt-names-the-kind-k18` landed, asserted as a
/// property rather than as an absence of code: nineteen kinds, one ending. The
/// eighteen-and-one split it replaces was the last place the driver interpreted
/// a kind.
#[test]
fn every_kind_gets_the_same_signalling_contract() {
    let contract = |kind: &Kind| {
        let prompt = compose(kind);
        let at = prompt
            .find("**Signal.**")
            .unwrap_or_else(|| panic!("`{}`'s prompt states no contract", kind.label()));
        prompt[at..].to_owned()
    };
    let first = contract(&a_kind("requirements"));
    assert_eq!(first, signalling_contract());
    for kind in shipped_kinds() {
        assert_eq!(
            contract(&kind),
            first,
            "`{}`'s signalling contract differs from every other kind's — the driver has \
             started interpreting a kind again, which is the `match` this leaf deleted",
            kind.label()
        );
    }
}

/// **No prompt states the stop flag.**
///
/// `--done` is an ending only a `finish` session takes, decided by what that
/// session did, and the shipped `grove-finish` skill states the three outcomes
/// it chooses between. A prompt naming it would hand the flag to the eighteen
/// kinds it is not an ending for, in the one channel a session cannot skip.
#[test]
fn no_prompt_states_the_stop_flag() {
    for kind in shipped_kinds() {
        assert!(
            !compose(&kind).contains("--done"),
            "`{}`'s prompt names `--done`. The prompt states the mechanism; which ending a \
             kind takes is that kind's skill's, and only `grove-finish` passes the flag.",
            kind.label()
        );
    }
}

/// **The other end of that split: `grove-finish` states all three outcomes.**
///
/// An absence in the prompt is indistinguishable from a rule that was deleted
/// rather than moved, and the deleted case is the worst failure this design has:
/// a `finish` session that tears the tree down and takes the prompt's default
/// ending relaunches the loop onto a grove that is no longer there. So the two
/// are asserted together.
#[test]
fn the_finish_skill_carries_the_three_endings_the_prompt_no_longer_routes() {
    let skill = std::fs::read_to_string(plugin_dir().join("skills/grove-finish/SKILL.md"))
        .expect("the plugin must ship the `finish` kind's skill");
    for outcome in [
        "`grove-llm complete --done`",
        "`grove-llm complete`",
        "no signal",
    ] {
        assert!(
            skill.contains(outcome),
            "grove-finish/SKILL.md must carry the {outcome} outcome; stating fewer than \
             three either states the wrong ending or leaves the last action to memory"
        );
    }
    assert!(
        skill.contains("override the default ending your prompt states"),
        "and it must say that these override the prompt's default — a session reads the \
         prompt's contract first, so the skill has to claim the override rather than \
         silently disagree"
    );
}

/// **The runtime facts are values, and carry no normative tail.**
///
/// The closed fact test hands every consequence of a value to the skill: *this
/// selection is authoritative*, *do not probe for the version control*. Both have
/// counterparts in the shipped spine, and a copy here would be the second source
/// the whole design exists to avoid. Pinned by the phrases the retired prose
/// actually used, so a reinstatement fails by name rather than by size.
#[test]
fn the_runtime_facts_restate_no_rule_the_skill_owns() {
    for kind in shipped_kinds() {
        let prompt = compose(&kind);
        for smuggled in [
            "do not probe",
            "authoritative",
            "disregard any harness banner",
            "grove-llm pick",
        ] {
            assert!(
                !prompt.contains(smuggled),
                "`{}`'s prompt carries \"{smuggled}\" — a rule with a counterpart in the \
                 shipped spine, which the closed fact test keeps in the skill. A value has \
                 nothing to drift from; a restated rule has exactly the counterpart the \
                 zero-drift claim denies.",
                kind.label()
            );
        }
    }
}

/// **The other end of the closure: the spine carries the rules the core sheds.**
///
/// Located by the condition's own bold opener, asserted on what follows it. A
/// paragraph can be emptied, weakened or reversed under an untouched heading, so
/// the claim is made against the body's own words; a missing opener fails by name
/// with the rule it was carrying spelled out, which is the moment to re-point
/// this test rather than to delete it.
#[test]
fn the_spine_carries_the_two_rules_the_core_sheds() {
    let spine = std::fs::read_to_string(plugin_dir().join("skills/grove/SKILL.md"))
        .expect("the plugin must ship the shared spine");
    for (opener, what, must_say) in [
        (
            "**Do not pick again.**",
            "that the driver's pick is authoritative and must not be re-walked",
            ["`grove-llm pick`", "the mandate wins"],
        ),
        (
            "**The stated VCS is definitive.**",
            "that the driver's stated version control is definitive and is not re-derived",
            ["definitive", "Do not re-derive"],
        ),
    ] {
        let start = spine.find(opener).unwrap_or_else(|| {
            panic!(
                "the spine carries no condition opening \"{opener}\", so it no longer states \
                 {what}. The core sheds every normative consequence of a value on the \
                 understanding that the skill states it; shedding it from both leaves the \
                 rule stated nowhere."
            )
        });
        let body = &spine[start..];
        let body = match body.find("\n\n") {
            Some(end) => &body[..end],
            None => body,
        };
        for phrase in must_say {
            assert!(
                body.contains(phrase),
                "the spine's condition opening \"{opener}\" no longer says \"{phrase}\", so \
                 it no longer states {what}.\n---\n{body}\n---"
            );
        }
    }
}

// -- The published version ---------------------------------------------------

/// **Grove publishes the workspace's release version, in the runtime facts.**
///
/// Decision 10 inverts the compatibility check: the machinery states what it is,
/// and the methodology decides whether that is good enough. The value has to be
/// the one `grove --version` renders — clap derives that from the same constant —
/// or the fallback would disagree with the mechanism.
#[test]
fn the_prompt_publishes_the_release_version_the_version_flag_renders() {
    let handle: Handle = Handle::parse(FIXTURE_HANDLE).expect("the fixture handle must parse");
    let prompt = prompt::compose(&Mandate {
        handle: &handle,
        kind: &a_kind("impl"),
        workspace: &workspace(),
        version: env!("CARGO_PKG_VERSION"),
    });
    assert!(
        prompt.contains(&format!("Grove version: {}.", env!("CARGO_PKG_VERSION"))),
        "the runtime facts must publish the crate's own version, which is what `grove \
         --version` renders:\n{prompt}"
    );
}

// -- The coupling to the shipped plugin --------------------------------------

/// **Every kind the driver can name is a skill the plugin ships**, asserted as
/// a round trip now that the plugin is the only place the set exists.
///
/// The prompt may not name a skill that does not exist, and nothing in the
/// compiler stops it: `skill_name` renders a token. Before `open-kind-k20` this
/// swept a compiled roster and asked whether the plugin had caught up; there is
/// no roster now, so the claim runs the other way — for every skill the plugin
/// ships, the kind read out of its directory name composes a prompt naming *that
/// same directory*. A skill whose name is not `grove-<token>`, or a token
/// `skill_name` renders differently, breaks it.
///
/// This walks **directories**, which is half the claim;
/// [`every_shipped_skill_declares_the_name_of_its_own_directory`] is the other
/// half, and the two are only a statement about what a harness registers when
/// both hold.
#[test]
fn every_kind_names_a_skill_the_plugin_ships() {
    let shipped = shipped_skills();
    let kinds = shipped_kinds();
    assert_eq!(
        kinds.len() + 1,
        shipped.len(),
        "every shipped skill but the bare `{}` spine must be a `grove-<kind>`: {shipped:?}",
        prompt::PLUGIN
    );
    for kind in kinds {
        let skill = prompt::skill_name(&kind);
        assert!(
            shipped.contains(&skill),
            "`{}`'s prompt names the `{skill}` skill, which `plugins/grove/skills` does not \
             ship — a session of that kind is told to load nothing. Shipped: {shipped:?}",
            kind.label()
        );
    }
}

/// **The control on that claim.** The sweep above passes over a plugin where
/// every skill happens to exist; this shows the same predicate saying no, so a
/// green sweep means the skills are there rather than that the check is inert.
#[test]
fn the_skill_set_claim_fails_on_a_kind_the_plugin_does_not_ship() {
    assert!(
        !shipped_skills().contains(&format!("{}-no-such-kind", prompt::PLUGIN)),
        "the predicate the sweep uses must be able to say no"
    );
}

/// **The namespace the prompt spells is the plugin entry's declared `name`.**
///
/// `grove:grove-<kind>` is one target spelled twice, and the left half is the
/// **name** a harness registers the plugin under, not the directory it was
/// sourced from. Asserting the `source` path instead would pass unchanged the
/// day the entry is renamed, leaving every prompt's namespaced spelling pointing
/// at nothing — so the claim is made against the field the harness actually
/// reads, and the source path is checked alongside it because the two must agree
/// for the entry to describe this plugin at all.
#[test]
fn the_namespace_is_the_shipped_plugin_entrys_declared_name() {
    let manifest =
        std::fs::read_to_string(support::repo_root().join(".claude-plugin/marketplace.json"))
            .expect("the marketplace manifest must be readable");
    let entry = manifest
        .split("{")
        .find(|block| block.contains(&format!("\"source\": \"./plugins/{}\"", prompt::PLUGIN)))
        .unwrap_or_else(|| {
            panic!(
                "the marketplace registers no plugin sourced from `./plugins/{}`",
                prompt::PLUGIN
            )
        });
    assert!(
        entry.contains(&format!("\"name\": \"{}\"", prompt::PLUGIN)),
        "the marketplace entry for `./plugins/{}` declares a different `name`, and the name \
         is what a harness namespaces its skills under — so `{}:grove-<kind>`, which every \
         prompt spells, addresses nothing:\n{entry}",
        prompt::PLUGIN,
        prompt::PLUGIN
    );
}

/// **A skill's directory name is the name it declares.**
///
/// The coupling the prompt actually has is to the name a harness *registers*,
/// and a harness reads that from the skill's own frontmatter — so
/// [`every_kind_names_a_skill_the_plugin_ships`], which walks directories, is
/// only a claim about the prompt if the two agree. They are separate strings in
/// separate files, nothing in the compiler holds them together, and a `name:`
/// edited inside `grove-finish/SKILL.md` would leave every `finish` prompt
/// naming a skill no harness has, with the sweep above still green.
///
/// Asserted over the whole shipped set rather than over the nineteen the kind
/// list reaches: the spine is subject to the same rule, and a set-wide claim
/// cannot be satisfied by the one skill someone remembered.
#[test]
fn every_shipped_skill_declares_the_name_of_its_own_directory() {
    for directory in shipped_skills() {
        let skill = std::fs::read_to_string(
            plugin_dir()
                .join("skills")
                .join(&directory)
                .join("SKILL.md"),
        )
        .unwrap_or_else(|error| panic!("`{directory}` must ship a SKILL.md: {error}"));
        let declared = skill
            .lines()
            .find_map(|line| line.strip_prefix("name:"))
            .unwrap_or_else(|| panic!("`{directory}`'s SKILL.md declares no frontmatter name"))
            .trim();
        assert_eq!(
            declared, directory,
            "the skill in `skills/{directory}` registers itself as `{declared}`. A harness \
             addresses it by the name it declares, and the prompt spells the directory — so \
             a session of that kind is told to load a skill that is not in its list."
        );
    }
}

/// The kinds this repository's plugin ships a skill for.
///
/// **This is the whole of what "the kind set" means now**, and it is the
/// methodology's rather than grove's: `open-kind-k20` deleted `Kind::ALL`, so
/// there is no compiled roster for a sweep to walk and the shipped plugin is the
/// only enumerable set there is. The bare `grove` directory is excluded — it is
/// the shared spine every kind reads, not a kind.
///
/// A directory whose token is not well-formed panics here rather than being
/// skipped: the prompt would spell it into a skill name either way, and a
/// silently dropped kind is a sweep that shrinks without saying so.
fn shipped_kinds() -> Vec<Kind> {
    let prefix = format!("{}-", prompt::PLUGIN);
    shipped_skills()
        .iter()
        .filter_map(|directory| directory.strip_prefix(&prefix))
        .map(|label| {
            Kind::new(label).unwrap_or_else(|error| {
                panic!("`skills/{prefix}{label}` does not name a session kind: {error}")
            })
        })
        .collect()
}

/// The `grove-*` skill directories the plugin ships, by directory name — the
/// name a harness registers, and the name a prompt spells.
fn shipped_skills() -> BTreeSet<String> {
    std::fs::read_dir(plugin_dir().join("skills"))
        .expect("the plugin must ship a skills directory")
        .map(|entry| entry.expect("readable directory entry"))
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect()
}

// -- The size alarm ----------------------------------------------------------

#[test]
fn every_kinds_prompt_stays_under_the_size_alarm() {
    for kind in shipped_kinds() {
        let size = compose(&kind).len();
        assert!(
            size <= SIZE_ALARM,
            "the `{}` prompt is {size} bytes, past the {SIZE_ALARM}-byte alarm. Size is a \
             consequence of the too-late test, never a criterion for admission — so the \
             question is not whether the addition is small but whether the driver has \
             acquired a new runtime fact. If it has not, the sentence belongs in the skill.",
            kind.label()
        );
    }
}

/// **The control on the size alarm.** A sweep that cannot fail is worth nothing,
/// so the same predicate is driven past the line — through the real seam, on a
/// real composition, rather than against a `String` of `x`s that would only show
/// `>` working.
#[test]
fn the_size_alarm_fires_on_an_oversized_prompt() {
    let oversized = compose_with(&a_kind("impl"), &format!("{}-k1", "pad".repeat(SIZE_ALARM)));
    assert!(
        oversized.len() > SIZE_ALARM,
        "the alarm's comparison must be able to say no about a composed prompt"
    );
}

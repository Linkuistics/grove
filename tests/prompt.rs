//! The **guaranteed core**, against the real embed.
//!
//! Every interesting check runs through the `prompt` seam
//! (`docs/specs/skill-delivered-methodology.md`, *Test seams*). The driver's
//! launch path is a thin wrapper covered by the existing loop-driver seam, so
//! nothing here spawns one.
//!
//! **What is mechanically checkable is narrower than what the design claims,
//! and the split is stated rather than blurred.** The three-part shape, the
//! ending's bytes, the two open couplings and the size alarm are all checkable.
//! What the wording *says* — whether it gets a session to read the skill — is
//! not, and is carried by `docs/research/wording-micro-test.md` and by
//! `delivery-acceptance-k11`. Recording where the automated boundary stops is
//! the same discipline the ending guard this replaces followed.
//!
//! **Every generated claim carries a control.** A sweep that cannot fail is
//! worth nothing, so the mapping check is shown failing on a removed path and
//! the size alarm on a synthetic oversized prompt.

use grove::leaf::Kind;
use grove::prompt;
use std::collections::BTreeSet;
use std::path::Path;

/// The alarm on the too-late test, per kind — **not a budget the design was
/// fitted to**.
///
/// Measured composition is well under half of it. A core that reaches 4 KiB has
/// gained roughly a page of prose, which the too-late test admits only if the
/// driver has acquired a new runtime fact — a rare and visible event. Everything
/// else that could push it there is importance-as-a-criterion, which is exactly
/// what this exists to see.
///
/// It lives in the suite rather than the build for the reason the previous alarm
/// did: it measures a judgement against an admittedly arbitrary number, and
/// failing a contributor's build on that is a gate this design is careful not to
/// erect.
const SIZE_ALARM: usize = 4 * 1024;

fn compose(kind: Kind) -> String {
    prompt::compose(
        kind,
        "guaranteed-core-k9",
        "this working tree is jj-enabled (jj workspace root: `/w`)",
        &["/home/u/.claude/skills/grove"],
    )
    .unwrap_or_else(|error| {
        panic!(
            "the real embed must compose a `{}` prompt: {error:#}",
            kind.label()
        )
    })
}

fn ending_of(kind: Kind) -> Option<&'static str> {
    prompt::ending_of(kind).expect("the real embed must carry the ending it maps to")
}

fn embedded(path: &str) -> &'static str {
    let file = grove::methodology::embed()
        .get_file(path)
        .unwrap_or_else(|| panic!("the embed must carry content/{path}"));
    std::str::from_utf8(file.contents()).expect("embedded markdown is UTF-8")
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
/// the first action, the runtime facts, and the ending last because it is the
/// last action. Recency is the whole reason for the third position, and it is
/// what a fixed template a human verifies by eye replaces the retired
/// file-ordering machinery with.
#[test]
fn the_prompt_is_three_parts_in_the_sessions_own_timeline_order() {
    for kind in Kind::ALL {
        let prompt = compose(kind);
        let reference = prompt::reference_file(kind);

        let load_ends = prompt
            .find("\nGrove mandate: ")
            .unwrap_or_else(|| panic!("`{}`'s prompt states no handle", kind.label()));
        let (load, rest) = prompt.split_at(load_ends + 1);

        assert!(
            load.starts_with(&format!(
                "**Load the `grove` skill now, and read its `{reference}`.**"
            )),
            "`{}`'s prompt must open with the imperative naming both targets, not with \
             anything else:\n{load}",
            kind.label()
        );
        assert!(
            load.contains("Do this **first**, before anything else you might reach for: before reading the\ntask file, before running any `grove-llm` verb, before looking at `.grove/`,\nbefore inspecting the working tree, and before answering a question."),
            "the ordering clause must enumerate the tempting alternatives — without the \
             enumeration it is advice, and the enumeration is what the micro-test measured"
        );
        assert!(
            load.contains("`/home/u/.claude/skills/grove`"),
            "the provisioned locations must appear by absolute path — that is what makes \
             the instruction actionable by plain file read"
        );

        let expected_facts =
            "Grove mandate: the leaf selected for this session is `guaranteed-core-k9`.\n\n\
             Version control: this working tree is jj-enabled (jj workspace root: `/w`).\n";
        match ending_of(kind) {
            Some(ending) => assert_eq!(
                rest,
                format!("{expected_facts}\n{ending}"),
                "`{}`'s prompt after the load instruction must be exactly the two runtime \
                 facts and then the ending — no introduction, no footer, and no rule \
                 restated from the skill",
                kind.label()
            ),
            None => assert_eq!(
                rest,
                expected_facts,
                "`{}` takes no fixed ending, so its prompt must end at the runtime facts",
                kind.label()
            ),
        }
    }
}

/// **A prompt states an ending only in its ending**, and never states the stop
/// flag.
///
/// The mandate-era version of this asked what a *composed mandate* carried, and
/// went with the mandate; what survives is narrower and sharper, because the
/// prompt is the one channel a session cannot skip. Two failures it catches: a
/// driver-authored sentence about signalling growing beside the runtime facts —
/// a second ending with nothing holding it in step with `content/SIGNAL.md` —
/// and `--done` reaching the eighteen kinds it is not an ending for.
#[test]
fn no_prompt_states_an_ending_outside_its_ending() {
    const COMPLETION_VERB: &str = "grove-llm complete";
    const STOP_FLAG: &str = "--done";

    for kind in Kind::ALL {
        let prompt = compose(kind);
        let before_ending = match ending_of(kind) {
            Some(ending) => prompt.strip_suffix(ending).unwrap_or(&prompt).to_string(),
            None => prompt.clone(),
        };
        assert!(
            !before_ending.contains(COMPLETION_VERB),
            "`{}`'s prompt names `{COMPLETION_VERB}` outside its ending. Either a second \
             ending has grown in Rust — with nothing holding it in step with the embed's \
             own — or the ending moved and this claim was not told.",
            kind.label()
        );
        assert!(
            !prompt.contains(STOP_FLAG),
            "`{}`'s prompt names `{STOP_FLAG}`. It is an ending only a `finish` session \
             takes, decided by what that session did, and `content/references/finish.md` \
             is where the three outcomes are stated together.",
            kind.label()
        );
    }
}

/// **The runtime facts are values, and carry no normative tail.**
///
/// The closed fact test hands every consequence of a value to the skill: *this
/// selection is authoritative*, *do not probe for the version control*. Both
/// have counterparts in `content/SKILL.md`, and a copy here would be the second
/// source the whole design exists to avoid. Pinned by the phrases the retired
/// prose actually used, so a reinstatement fails by name rather than by size.
#[test]
fn the_runtime_facts_restate_no_rule_the_skill_owns() {
    for kind in Kind::ALL {
        let prompt = compose(kind);
        for smuggled in [
            "do not probe",
            "authoritative",
            "disregard any harness banner",
            "grove-llm pick",
        ] {
            assert!(
                !prompt.contains(smuggled),
                "`{}`'s prompt carries \"{smuggled}\" — a rule with a counterpart in \
                 `content/`, which the closed fact test keeps in the skill. A value has \
                 nothing to drift from; a restated rule has exactly the counterpart the \
                 zero-drift claim denies.",
                kind.label()
            );
        }
    }
}

/// **The other end of the closure: the skill carries the rules the core sheds.**
///
/// [`the_runtime_facts_restate_no_rule_the_skill_owns`] asserts an absence, and
/// an absence on its own is indistinguishable from a rule that was deleted
/// rather than moved — which is exactly the closure's one real cost going
/// unpaid. So the two are asserted together: the consequences leave the prompt
/// *and* land in `content/SKILL.md`, where they depend on the skill being read
/// like every other rule.
///
/// Pinned on the two conditions the corpus declares, not on prose, so a rewrite
/// of either condition's wording does not fail here while its deletion does.
#[test]
fn the_skill_carries_the_two_rules_the_core_sheds() {
    let skill = embedded("SKILL.md");
    for (unit, what) in [
        (
            "skill-do-not-pick-again",
            "that the driver's pick is authoritative and must not be re-walked",
        ),
        (
            "skill-stated-vcs-is-definitive",
            "that the driver's stated version control is definitive and is not re-derived",
        ),
    ] {
        assert!(
            skill.contains(&format!("<!-- unit: {unit} ")),
            "content/SKILL.md no longer states {what}. The core sheds every normative \
             consequence of a value on the understanding that the skill states it; \
             shedding it from both leaves the rule stated nowhere."
        );
    }
}

/// **The ending is the embedded file's bytes**, so a driver-side copy cannot
/// reappear as a Rust literal without failing.
///
/// The one genuine duplicate between the two channels is not duplicated: the
/// driver inlines the embed's own signal file, from the same embed that is
/// provisioned. One source, two deliveries, and no build boundary between them.
#[test]
fn the_ending_is_the_embedded_signal_files_bytes() {
    let signal = embedded("SIGNAL.md");
    let mut ended = 0;
    for kind in Kind::ALL {
        let Some(ending) = ending_of(kind) else {
            continue;
        };
        assert_eq!(
            ending,
            signal,
            "`{}`'s ending is not `content/SIGNAL.md`'s bytes — a copy has grown in Rust",
            kind.label()
        );
        assert!(
            compose(kind).ends_with(signal),
            "`{}`'s prompt must *end* with it; recency is the whole reason for the position",
            kind.label()
        );
        ended += 1;
    }
    assert_eq!(
        ended,
        Kind::ALL.len() - 1,
        "eighteen kinds take the fixed ending and `finish` takes none; a different \
         count means the exception moved without this claim moving with it"
    );
}

/// **`finish` is the one kind with no fixed ending, and the exception is a
/// correctness one.**
///
/// A `finish` session has three endings chosen by what it did — `complete
/// --done` after teardown, bare `complete` if it externalised work instead, no
/// signal if the human declined or was absent (`content/references/finish.md`).
/// Inlining `SIGNAL.md` would put *run `grove-llm complete`* last in the prompt
/// of the one session that may have just deleted the task tree, relaunching the
/// loop onto a torn-down grove which the driver then re-scaffolds.
///
/// Its ending rides its reference file instead, which the load instruction names
/// first and by path — so this asserts that file still carries the three-way
/// choice, and not merely that the prompt omits one.
#[test]
fn a_finish_prompt_states_no_ending_and_its_reference_file_carries_all_three() {
    let prompt = compose(Kind::Finish);
    assert!(
        !prompt.contains("grove-llm complete"),
        "a finish prompt must not state an ending; two of its three outcomes would \
         make a fixed one wrong:\n{prompt}"
    );
    assert!(
        prompt.contains("`references/finish.md`"),
        "and it must name the file that does carry them"
    );

    let finish = embedded("references/finish.md");
    for outcome in [
        "`grove-llm complete --done`",
        "`grove-llm complete`",
        "no signal",
    ] {
        assert!(
            finish.contains(outcome),
            "content/references/finish.md must still carry the {outcome} outcome — it is \
             the only place a finish session is told how to end"
        );
    }
}

// -- The two couplings not closed by construction ----------------------------

/// **The skill name the core states, against the embedded `SKILL.md`'s own
/// `name:`.** The core points at a skill by the name a harness will have
/// registered it under, and this is what keeps "the prose drift surface is zero"
/// from being read as "there is nothing to hold in step".
#[test]
fn the_core_names_the_skill_as_the_embed_names_itself() {
    let declared = embedded("SKILL.md")
        .lines()
        .find_map(|line| line.strip_prefix("name:"))
        .expect("the embedded SKILL.md must declare a frontmatter name")
        .trim();
    assert_eq!(
        prompt::SKILL_NAME,
        declared,
        "the core tells a session to load `{}` while the embedded SKILL.md registers \
         itself as `{declared}` — every session would be told to open a skill that is \
         not in its list",
        prompt::SKILL_NAME
    );
}

/// **Every mapped reference path exists in the embed**, generated from the
/// closed kind set rather than enumerated — so a twentieth kind fails until it
/// is classified. The precedent is the existing kind-guidance suite, which
/// already generates its claims this way.
///
/// Golden per-kind prompt snapshots are deliberately dropped. The ids-not-bytes
/// golden existed because nineteen ~48 kB mandates could not be held as bytes;
/// nineteen ~2 KiB prompts differ only in one path and one handle, so this says
/// everything a golden would and says it by name.
#[test]
fn every_kind_maps_to_a_reference_file_the_embed_carries() {
    for kind in Kind::ALL {
        let path = prompt::reference_file(kind);
        assert!(
            grove::methodology::embed().get_file(path).is_some(),
            "`{}` is mapped to content/{path}, which the embedded corpus does not carry — \
             a session of that kind is told to read nothing",
            kind.label()
        );
    }
}

/// **The control on the mapping claim.** The sweep above passes over a corpus
/// where every path happens to exist; this shows the same predicate failing on a
/// path that does not, so a green sweep means the paths resolve rather than that
/// the check is inert.
#[test]
fn the_mapping_claim_fails_on_a_path_the_embed_does_not_carry() {
    assert!(
        grove::methodology::embed()
            .get_file("references/no-such-kind.md")
            .is_none(),
        "the predicate the sweep uses must be able to say no"
    );
}

/// **Ten files serve the nineteen kinds, and each family agrees with itself.**
///
/// The five `review-*` kinds name one file, as do the five `integrate-review-*`
/// and the two research producers. Asserted as the family's *set* being a
/// singleton rather than by naming the expected path, so this cannot pass by
/// agreeing with the map it is checking.
#[test]
fn a_familys_kinds_share_one_reference_file() {
    let families: [(&str, &[Kind]); 3] = [
        (
            "review-*",
            &[
                Kind::ReviewRequirements,
                Kind::ReviewDesign,
                Kind::ReviewPlanning,
                Kind::ReviewPrototype,
                Kind::ReviewImpl,
            ],
        ),
        (
            "integrate-review-*",
            &[
                Kind::IntegrateReviewRequirements,
                Kind::IntegrateReviewDesign,
                Kind::IntegrateReviewPlanning,
                Kind::IntegrateReviewPrototype,
                Kind::IntegrateReviewImpl,
            ],
        ),
        ("research", &[Kind::ResearchA, Kind::ResearchB]),
    ];
    for (family, kinds) in families {
        let paths: BTreeSet<&str> = kinds.iter().map(|k| prompt::reference_file(*k)).collect();
        assert_eq!(
            paths.len(),
            1,
            "the {family} kinds must share one reference file; they name {paths:?}"
        );
    }

    let distinct: BTreeSet<&str> = Kind::ALL
        .iter()
        .map(|k| prompt::reference_file(*k))
        .collect();
    assert_eq!(
        distinct.len(),
        10,
        "ten files serve the nineteen kinds, matching the routing table `SKILL.md` \
         prints for a session that arrived without a mandate; the map names {distinct:?}"
    );
}

// -- The size alarm ----------------------------------------------------------

#[test]
fn every_kinds_prompt_stays_under_the_size_alarm() {
    for kind in Kind::ALL {
        let size = compose(kind).len();
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

/// **The control on the size alarm.** A sweep that cannot fail is worth
/// nothing, so the same predicate is driven past the line — through the real
/// seam, on a real composition, rather than against a `String` of `x`s that
/// would only show `>` working.
#[test]
fn the_size_alarm_fires_on_an_oversized_prompt() {
    let oversized = prompt::compose(
        Kind::Impl,
        &"pad".repeat(SIZE_ALARM),
        "plain Git (worktree root: `/w`)",
        &["/home/u/.claude/skills/grove"],
    )
    .expect("composition must not depend on the handle's size");
    assert!(
        oversized.len() > SIZE_ALARM,
        "the alarm's comparison must be able to say no about a composed prompt"
    );
}

// -- The absent-destination case ---------------------------------------------

/// A machine with no known harness root still gets a prompt, and it says
/// something true rather than trailing off after "provisioned for you at".
///
/// The launch proceeds — Grove reports and never refuses — and
/// `report_absent_skill_destination` has already said so loudly on the same
/// iteration.
#[test]
fn a_machine_with_no_harness_root_still_composes_a_truthful_prompt() {
    let prompt =
        prompt::compose::<&Path>(Kind::Impl, "x-k1", "plain Git (worktree root: `/w`)", &[])
            .expect("composition must not depend on a harness being installed");
    assert!(
        prompt.contains("no known harness root"),
        "the location sentence must say so rather than name nowhere:\n{prompt}"
    );
}

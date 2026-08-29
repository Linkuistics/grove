//! The **guaranteed core**, against the real embed.
//!
//! Every interesting check runs through the `prompt` seam
//! (`docs/ARCHITECTURE.md`, *the embed test seam*). The driver's
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
//! worth nothing, so the mapping check is shown failing on a removed path, the
//! family check on a future family member filed under another family's
//! discipline, and the size alarm on a synthetic oversized prompt.

use grove::leaf::Kind;
use grove::prompt;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

/// The alarm on the too-late test, per kind — **not a budget the design was
/// fitted to**.
///
/// Measured composition leaves roughly 40% of it free. A core that reaches 4 KiB
/// has gained roughly a page of prose, which the too-late test admits only if the
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

fn ending_of(kind: Kind) -> &'static str {
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
        assert_eq!(
            rest,
            format!("{expected_facts}\n{}", ending_of(kind)),
            "`{}`'s prompt after the load instruction must be exactly the two runtime \
             facts and then the ending — no introduction, no footer, and no rule \
             restated from the skill",
            kind.label()
        );
    }
}

/// **A prompt states an ending only in its ending**, and never states the stop
/// flag.
///
/// The mandate-era version of this asked what a *composed mandate* carried, and
/// went with the mandate; what survives is narrower and sharper, because the
/// prompt is the one channel a session cannot skip. Two failures it catches: a
/// driver-authored sentence about signalling growing beside the runtime facts —
/// a second ending with nothing holding it in step with the embed's own — and
/// `--done` reaching the eighteen kinds it is not an ending for.
///
/// **`--done` is scoped by kind rather than banned outright.** It belongs in
/// `finish`'s ending, where it is one row of a three-outcome table, and nowhere
/// else in any prompt — including nowhere else in `finish`'s own.
#[test]
fn no_prompt_states_an_ending_outside_its_ending() {
    const COMPLETION_VERB: &str = "grove-llm complete";
    const STOP_FLAG: &str = "--done";

    for kind in Kind::ALL {
        let prompt = compose(kind);
        let ending = ending_of(kind);
        let before_ending = prompt.strip_suffix(ending).unwrap_or_else(|| {
            panic!("`{}`'s prompt must end on its ending", kind.label());
        });
        assert!(
            !before_ending.contains(COMPLETION_VERB),
            "`{}`'s prompt names `{COMPLETION_VERB}` outside its ending. Either a second \
             ending has grown in Rust — with nothing holding it in step with the embed's \
             own — or the ending moved and this claim was not told.",
            kind.label()
        );
        assert!(
            !before_ending.contains(STOP_FLAG),
            "`{}`'s prompt names `{STOP_FLAG}` outside its ending, where nothing holds it \
             in step with the embed.",
            kind.label()
        );
        assert_eq!(
            ending.contains(STOP_FLAG),
            kind == Kind::Finish,
            "`{STOP_FLAG}` is an ending only a `finish` session takes, decided by what \
             that session did — so `content/SIGNAL-FINISH.md` states it and \
             `content/SIGNAL.md` must not. `{}`'s ending disagrees.",
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
/// **Located by the condition's opening sentence, asserted on what follows it.**
/// The locator was a unit marker until the marker machinery was deleted; it is
/// now the condition's own bold opener, and the paragraph it begins is the body.
/// That is a weaker *locator* — an opener can be reworded — and deliberately not
/// a weaker *claim*: a missing opener fails here by name with the rule it was
/// carrying spelled out, which is the moment to re-point this test rather than
/// to delete it.
///
/// What has not changed is that the claim is made against the body's **own
/// words**. A locator is evidence that a condition is present and nothing more:
/// a paragraph can be emptied, weakened, or reversed under an untouched heading,
/// and a check on the heading alone would still report the rule as stated.
///
/// The located phrases are the load-bearing clause of each condition rather than
/// its whole prose, so a rewrite that keeps the rule passes and one that drops it
/// fails.
#[test]
fn the_skill_carries_the_two_rules_the_core_sheds() {
    let skill = embedded("SKILL.md");
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
        let start = skill.find(opener).unwrap_or_else(|| {
            panic!(
                "content/SKILL.md carries no condition opening \"{opener}\", so it no longer \
                 states {what}. The core sheds every normative consequence of a value on the \
                 understanding that the skill states it; shedding it from both leaves the \
                 rule stated nowhere."
            )
        });
        // One paragraph: from the opener to the next blank line, or to end of
        // file for a condition that closes the page.
        let body = &skill[start..];
        let body = match body.find("\n\n") {
            Some(end) => &body[..end],
            None => body,
        };
        for phrase in must_say {
            assert!(
                body.contains(phrase),
                "content/SKILL.md's condition opening \"{opener}\" no longer says \
                 \"{phrase}\", so it no longer states {what}. The opener is still there, \
                 which is why this asserts the text and not the heading.\n---\n{body}\n---"
            );
        }
    }
}

/// **The ending is the embedded file's bytes**, so a driver-side copy cannot
/// reappear as a Rust literal without failing.
///
/// The one genuine duplicate between the two channels is not duplicated: the
/// driver inlines the embed's own signal file, from the same embed that is
/// provisioned. One source, two deliveries, and no build boundary between them.
///
/// Two signal files serve the nineteen kinds, and the split is asserted by count
/// as well as by bytes: eighteen take `SIGNAL.md`'s single instruction and
/// `finish` takes `SIGNAL-FINISH.md`'s three-outcome choice.
#[test]
fn the_ending_is_the_embedded_signal_files_bytes() {
    let signal = embedded("SIGNAL.md");
    let finish_signal = embedded("SIGNAL-FINISH.md");
    assert_ne!(
        signal, finish_signal,
        "the two signal files must differ; one of them has become a copy of the other"
    );

    let mut took_finish_signal = 0;
    for kind in Kind::ALL {
        let expected = if kind == Kind::Finish {
            took_finish_signal += 1;
            finish_signal
        } else {
            signal
        };
        assert_eq!(
            ending_of(kind),
            expected,
            "`{}`'s ending is not its embedded signal file's bytes — a copy has grown in \
             Rust, or the mapping moved",
            kind.label()
        );
        assert!(
            compose(kind).ends_with(expected),
            "`{}`'s prompt must *end* with it; recency is the whole reason for the position",
            kind.label()
        );
    }
    assert_eq!(
        took_finish_signal, 1,
        "exactly one kind takes the three-outcome ending; a different count means the \
         `finish` case moved without this claim moving with it"
    );
}

/// **A `finish` prompt ends on the choice, not on one of its branches.**
///
/// A `finish` session has three endings chosen by what it did — `complete
/// --done` after teardown, bare `complete` if it externalised work instead, no
/// signal if the human declined or was absent. Inlining `SIGNAL.md` would put
/// *run `grove-llm complete`* last in the prompt of the one session that may have
/// just deleted the task tree, relaunching the loop onto a torn-down grove which
/// the driver then re-scaffolds.
///
/// **Omitting the part instead answers that at the cost of the thing the core is
/// for.** A `finish` session that tears the tree down and then forgets to signal
/// leaves the loop waiting on a session that will not end — the observed failure,
/// in the session where it costs most — and no amount of reading
/// `references/finish.md` at bootstrap repairs a forgotten last action. So the
/// three-outcome table is the ending, delivered like every other one.
#[test]
fn a_finish_prompt_ends_on_all_three_outcomes() {
    let prompt = compose(Kind::Finish);
    for outcome in [
        "`grove-llm complete --done`",
        "`grove-llm complete`",
        "no signal",
    ] {
        assert!(
            prompt.contains(outcome),
            "a finish prompt must carry the {outcome} outcome; stating fewer than three \
             either states the wrong ending or leaves the last action to memory:\n{prompt}"
        );
    }
    assert!(
        prompt.contains("`references/finish.md`"),
        "and it must still name the file carrying the teardown steps the outcomes turn on"
    );

    // The routing half of *one source, two deliveries*: the reference file sends
    // a reader to the same bytes rather than restating them, so the skill and the
    // prompt cannot disagree about what the outcomes are.
    let reference = embedded("references/finish.md");
    assert!(
        reference.contains("`SIGNAL-FINISH.md`"),
        "content/references/finish.md must route to the outcomes' one source"
    );
    assert!(
        !reference.contains("| teardown completed |"),
        "and must not restate the table it routes to — a second copy is exactly what \
         routing exists to avoid"
    );
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
/// and the two research producers.
///
/// **Membership is derived from the taxonomy, never enumerated**, and that is
/// what makes this a check rather than a restatement. The exhaustive `match` at
/// `src/prompt.rs` forces a twentieth kind into *some* arm, but nothing in the
/// compiler forces it into the arm its label implies — a future
/// `review-security` mapped to `references/impl.md` still yields ten distinct
/// paths, and a hand-written membership list simply never sees it, so review
/// sessions receive implementation discipline with the suite green. That was the
/// live taxonomy claim the deleted mandate-era family-scope guard used to hold,
/// and it is held here instead: [`routing_group`] reads each kind's own label,
/// so a new variant joins its family the moment it is named.
///
/// Two claims, both over the derived grouping: every group names exactly one
/// file, and no two groups name the same one. Together those say the map is a
/// bijection between the ten routing groups and the ten reference files — which
/// is the routing table `SKILL.md` prints, stated independently of it.
#[test]
fn a_familys_kinds_share_one_reference_file() {
    let grouped = group_by_routing_group(&live_mapping());

    assert_eq!(
        split_groups(&grouped),
        Vec::<String>::new(),
        "a routing group's kinds must all name one reference file — a kind \
         routed away from its family receives another family's discipline"
    );
    assert_eq!(
        grouped.len(),
        10,
        "ten routing groups serve the nineteen kinds, matching the table \
         `SKILL.md` prints for a session that arrived without a mandate; the \
         taxonomy yields {:?}",
        grouped.keys().collect::<Vec<_>>()
    );
    let files: BTreeSet<&str> = grouped.values().flatten().copied().collect();
    assert_eq!(
        files.len(),
        grouped.len(),
        "two routing groups share a reference file, so one family is reading \
         another's discipline: {grouped:?}"
    );
}

/// **The control on the family claim.** The sweep above passes over a taxonomy
/// whose every member is filed correctly; this drives the same predicate over
/// one that is not, so a green sweep means the families hold rather than that
/// the check is inert.
///
/// The misfiling is the exact shape the review named: a future review kind
/// mapped to implementation discipline. It is applied to the *label* set, which
/// is why the check had to be derived — no `Kind` variant needs inventing to
/// show the guard working, and the guard would see the real variant the same way.
#[test]
fn the_family_check_rejects_a_misfiled_future_family_member() {
    let mut misfiled = live_mapping();
    misfiled.push(("review-security", "references/impl.md"));

    let complaints = split_groups(&group_by_routing_group(&misfiled));
    assert_eq!(
        complaints.len(),
        1,
        "a future `review-*` kind routed to implementation discipline must be \
         reported, once, against its own family: {complaints:?}"
    );
    assert!(
        complaints[0].starts_with("review:"),
        "the complaint must name the family the misfiled kind belongs to: {complaints:?}"
    );
}

/// Every kind's label paired with the reference file the driver maps it to.
fn live_mapping() -> Vec<(&'static str, &'static str)> {
    Kind::ALL
        .iter()
        .map(|kind| (kind.label(), prompt::reference_file(*kind)))
        .collect()
}

/// The routing group a kind's **own label** puts it in: its family for the three
/// family-shaped kinds, otherwise itself.
///
/// Read off the closed taxonomy's naming rule (`content/references/*`, and
/// `TASK-FORMAT`'s nineteen kinds) rather than off the map being checked, so the
/// two cannot agree by construction. `integrate-review-` is tested before
/// `review-`, since every integration label contains the review one; and
/// `combine-research` is deliberately its own group — it takes the `research-`
/// producers' output but not their discipline or their file.
fn routing_group(label: &str) -> &str {
    for prefix in ["integrate-review-", "review-", "research-"] {
        if label.starts_with(prefix) {
            return prefix.trim_end_matches('-');
        }
    }
    label
}

/// The reference files each routing group names, gathered from a label→path
/// mapping.
fn group_by_routing_group<'a>(
    mapping: &[(&'a str, &'a str)],
) -> BTreeMap<&'a str, BTreeSet<&'a str>> {
    let mut grouped: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (label, path) in mapping {
        grouped
            .entry(routing_group(label))
            .or_default()
            .insert(path);
    }
    grouped
}

/// Groups whose kinds do not agree on one file, one line each — the failure the
/// derivation exists to see.
fn split_groups(grouped: &BTreeMap<&str, BTreeSet<&str>>) -> Vec<String> {
    grouped
        .iter()
        .filter(|(_, paths)| paths.len() != 1)
        .map(|(group, paths)| format!("{group}: {paths:?}"))
        .collect()
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
        "this working tree is jj-enabled (jj workspace root: `/w`)",
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
        prompt::compose::<&Path>(Kind::Impl, "x-k1", "this working tree is jj-enabled (jj workspace root: `/w`)", &[])
            .expect("composition must not depend on a harness being installed");
    assert!(
        prompt.contains("no known harness root"),
        "the location sentence must say so rather than name nowhere:\n{prompt}"
    );
}

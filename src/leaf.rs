// Shared leaf vocabulary that outlived the old-format reader/grower. **One** item
// survives: `Kind`, the leaf-kind enum — a **closed, parameterised set of
// nineteen** (five producers, each with its own `review-` and
// `integrate-review-` step, two research producers, their combine step, and
// finish) written into a current task filename. Live: the grow/lifecycle verbs
// (`task_grow` / `tree_lifecycle`) and the `grove-llm` CLI surface (`llm_cli`)
// parse and carry it.
//
// Everything else this module once held — the `NNN-slug` / `done/` reader and
// grower (`add`, `insert`, header rewriting, cross-ref surfacing) — was the old
// verb path, made dead by the migration and deleted with it. Its last lodger,
// the `NNN-slug` prefix parser, went when that layout stopped being migrated:
// what recognises a withdrawn layout now is a private matcher beside its refusal
// in `tree_migrate`, which is where the only caller is.

use anyhow::{bail, Result};

/// The previous spelling of `impl`, still accepted on **read** so a live grove's
/// task files keep working across the rename (task-kind-taxonomy). Refused on
/// write, with an error naming the replacement — see [`Kind::parse`].
const WORK_ALIAS: &str = "work";

/// A leaf's session kind — a closed, **parameterised** set of nineteen
/// (task-kind-taxonomy): five producers, each with its own `review-` and
/// `integrate-review-` step, two research producers, their combine step, and the
/// driver-owned finish step. Adding a twentieth is a deliberate code change, never
/// a free-text label a leaf may coin. Only `Planning` carries methodological
/// force (the loop's sole Execute branch, the only kind that grows the tree);
/// every other kind produces some artifact, differing in discipline. The label
/// is also the key a session's command template is configured under, which is
/// why the set is closed: a kind Grove cannot spell is a kind the configuration
/// cannot cover. Each kind's discipline and its HITL/AFK mark are
/// `docs/ARCHITECTURE.md#task-kind-taxonomy`; this enum owns only the set and
/// its spelling.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    // Producers.
    Requirements,
    Design,
    Planning,
    Prototype,
    Impl,
    // Research and its binary combine step.
    ResearchA,
    ResearchB,
    CombineResearch,
    // Driver-owned complete-finish-cycle sentinel.
    Finish,
    // One adversarial read per producer.
    ReviewRequirements,
    ReviewDesign,
    ReviewPlanning,
    ReviewPrototype,
    ReviewImpl,
    // One triage-and-apply step per review.
    IntegrateReviewRequirements,
    IntegrateReviewDesign,
    IntegrateReviewPlanning,
    IntegrateReviewPrototype,
    IntegrateReviewImpl,
}

impl Kind {
    /// Every kind, in taxonomy order (producers, research, reviews,
    /// integrations — the order `docs/ARCHITECTURE.md#task-kind-taxonomy` presents
    /// them). The single source of truth for the *set*, as [`Kind::label`] is
    /// for its *spelling*: parsing, the `--kind` error listing, and the
    /// configuration's completeness check all derive from these two, so none of
    /// them can drift from the enum. That mattered little at five kinds and
    /// matters a lot at nineteen.
    pub const ALL: [Kind; 19] = [
        Kind::Requirements,
        Kind::Design,
        Kind::Planning,
        Kind::Prototype,
        Kind::Impl,
        Kind::ResearchA,
        Kind::ResearchB,
        Kind::CombineResearch,
        Kind::Finish,
        Kind::ReviewRequirements,
        Kind::ReviewDesign,
        Kind::ReviewPlanning,
        Kind::ReviewPrototype,
        Kind::ReviewImpl,
        Kind::IntegrateReviewRequirements,
        Kind::IntegrateReviewDesign,
        Kind::IntegrateReviewPlanning,
        Kind::IntegrateReviewPrototype,
        Kind::IntegrateReviewImpl,
    ];

    /// **The two chain steps are not derived here any more, and there is no
    /// `review_steps` to call.** They were, for `leaf-add-chain`, which took one
    /// producer kind and emitted all three leaves; with review composition flat
    /// and lazy (flat-lazy-review) a session cuts each step itself with
    /// `leaf-add … --kind review-<producer>`, so the derivation became guidance
    /// rather than a constructor input, and a derivation nothing calls is dead
    /// API this crate deletes on sight (`src/lib.rs`'s header states the rule).
    ///
    /// What makes that guidance *safe* is not a function but a property of the
    /// label set — every producer's two steps are its own label under the
    /// `review-` and `integrate-review-` prefixes — and that property is pinned
    /// below, where a rename that broke it would fail rather than silently
    /// sending a session to `--kind` a label the set does not contain.
    ///
    /// Write gates (task-kind-taxonomy): a grow verb rejects an unrecognised
    /// `--kind` with an error listing the whole set, so a typo is caught at
    /// authoring time, when a human is present to fix it. Legacy migration and
    /// installed-driver wire compatibility use the private read parser below;
    /// current task filenames never degrade.
    ///
    /// `work` is refused here rather than silently accepted: it is the previous
    /// spelling of `impl`, and the gate exists to retrain a human who is present
    /// to be retrained. The error names the replacement instead of only listing
    /// the nineteen, because "not in the list" is unhelpful for a word that was
    /// correct last week.
    pub fn parse(s: &str) -> Result<Kind> {
        if let Some(kind) = Kind::from_label(s) {
            return Ok(kind);
        }
        if s == WORK_ALIAS {
            bail!(
                "--kind `work` was renamed `impl` (task-kind-taxonomy); use `--kind impl`. \
                 Legacy migration still reads `work` as `impl`."
            );
        }
        bail!("--kind must be one of {}, got {:?}", Kind::label_list(), s)
    }

    /// Compatibility parser for legacy migration and installed-driver wire
    /// values. Current filename parsing uses the strict known-label set.
    pub(crate) fn parse_read(s: &str) -> Option<Kind> {
        Kind::from_label(s)
            .or_else(|| (s == WORK_ALIAS).then_some(Kind::Impl))
            .or_else(|| (s == "research").then_some(Kind::ResearchA))
    }

    /// The lowercase label written into a task filename and printed by
    /// `grove-llm kind`. The single source of truth for the label
    /// direction: the grow verbs' leaf template (`task_grow`), the `kind` verb,
    /// [`Kind::from_label`], and the loop driver's env-var suffixes all read
    /// through it, so none of them can disagree on the spelling.
    pub fn label(self) -> &'static str {
        match self {
            Kind::Requirements => "requirements",
            Kind::Design => "design",
            Kind::Planning => "planning",
            Kind::Prototype => "prototype",
            Kind::Impl => "impl",
            Kind::ResearchA => "research-a",
            Kind::ResearchB => "research-b",
            Kind::CombineResearch => "combine-research",
            Kind::Finish => "finish",
            Kind::ReviewRequirements => "review-requirements",
            Kind::ReviewDesign => "review-design",
            Kind::ReviewPlanning => "review-planning",
            Kind::ReviewPrototype => "review-prototype",
            Kind::ReviewImpl => "review-impl",
            Kind::IntegrateReviewRequirements => "integrate-review-requirements",
            Kind::IntegrateReviewDesign => "integrate-review-design",
            Kind::IntegrateReviewPlanning => "integrate-review-planning",
            Kind::IntegrateReviewPrototype => "integrate-review-prototype",
            Kind::IntegrateReviewImpl => "integrate-review-impl",
        }
    }

    /// The exact inverse of [`Kind::label`], derived from it rather than
    /// hand-written, so the round-trip holds by construction. No aliases and no
    /// error text — the two parse entry points above layer their own policy on
    /// top of this one shared lookup.
    pub(crate) fn from_label(s: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.label() == s)
    }

    /// Split `<session-kind>-<slug>` using a known kind label. The closed label
    /// set has a stronger invariant: no label plus `-` prefixes another label,
    /// so every rendered filename has exactly one matching kind and round-trips
    /// without changing the stable slug. Longest selection stays defensive.
    pub(crate) fn split_filename_prefix(value: &str) -> Option<(Kind, &str)> {
        Kind::ALL
            .into_iter()
            .filter_map(|kind| {
                value
                    .strip_prefix(kind.label())
                    .and_then(|rest| rest.strip_prefix('-'))
                    .map(|slug| (kind, slug))
            })
            .max_by_key(|(kind, _)| kind.label().len())
    }

    /// Every label, backtick-quoted and comma-joined, for the `--kind` error.
    /// Built from [`Kind::ALL`] so a new kind is listed the moment it exists.
    pub(crate) fn label_list() -> String {
        Kind::ALL
            .iter()
            .map(|k| format!("`{}`", k.label()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod inline_tests {
    use super::*;

    /// Each kind's own slot in [`Kind::ALL`] — the witness that `ALL` holds
    /// **every** variant, and the reason a twentieth kind cannot reach the rest
    /// of the suite unseen.
    ///
    /// `ALL` is a hand-written array, and until this existed no exhaustive
    /// `match` anywhere forced a new variant *into* it. [`Kind::label`],
    /// [`is_producer`] and the family classifier in
    /// `tests/session_kind_guidance.rs` each make the author classify the new
    /// kind, and every one of them still compiles with `ALL` left at nineteen —
    /// while every sweep in this crate and in `tests/` enumerates the kind set
    /// *through* `ALL`, [`the_set_is_nineteen_distinct_kinds`] included, which
    /// counts what `ALL` holds rather than what the enum declares. So the one
    /// hazard `ALL` carries is silent in the direction that matters.
    ///
    /// This closes it with the same device those classifiers use, one level
    /// down. A twentieth variant must claim a slot; every slot `0..=18` is
    /// visibly taken by the arm above it; and the slot left to claim is
    /// `Kind::ALL[19]`, a constant index past the end of a fixed-size array,
    /// which rustc rejects at **compile time** under the deny-by-default
    /// `unconditional_panic` lint — naming the length `ALL` actually has.
    /// Adding the kind to `ALL` is what clears it.
    ///
    /// **Its one residue**, stated rather than papered over: an author who
    /// answers that error by aliasing a slot already taken compiles, because the
    /// test below iterates `ALL` and so never evaluates the arm of a kind `ALL`
    /// does not hold. Nothing about the compile error suggests that answer —
    /// what stands against it is the same thing that stands against filing a
    /// `review-` kind under the producers.
    ///
    /// **And the one place this crate's standing rule cannot be met.** Every
    /// other sweep here carries a control showing it fail, because a sweep that
    /// cannot fail is worth nothing; a *compile* error is not a value a test in
    /// the same crate can observe, so this one is demonstrated by hand instead —
    /// add a variant, leave `ALL` alone, and the build stops naming the length.
    /// That is a weaker record than a control and is why the mechanism is spelled
    /// out above rather than asserted below.
    fn slot(kind: Kind) -> Kind {
        match kind {
            Kind::Requirements => Kind::ALL[0],
            Kind::Design => Kind::ALL[1],
            Kind::Planning => Kind::ALL[2],
            Kind::Prototype => Kind::ALL[3],
            Kind::Impl => Kind::ALL[4],
            Kind::ResearchA => Kind::ALL[5],
            Kind::ResearchB => Kind::ALL[6],
            Kind::CombineResearch => Kind::ALL[7],
            Kind::Finish => Kind::ALL[8],
            Kind::ReviewRequirements => Kind::ALL[9],
            Kind::ReviewDesign => Kind::ALL[10],
            Kind::ReviewPlanning => Kind::ALL[11],
            Kind::ReviewPrototype => Kind::ALL[12],
            Kind::ReviewImpl => Kind::ALL[13],
            Kind::IntegrateReviewRequirements => Kind::ALL[14],
            Kind::IntegrateReviewDesign => Kind::ALL[15],
            Kind::IntegrateReviewPlanning => Kind::ALL[16],
            Kind::IntegrateReviewPrototype => Kind::ALL[17],
            Kind::IntegrateReviewImpl => Kind::ALL[18],
        }
    }

    #[test]
    fn all_holds_every_kind_in_the_slot_it_claims() {
        // Two claims at once. Each kind occupying the slot it named is what
        // makes the slots a bijection onto `ALL` — two arms claiming one slot
        // leaves the other kind reading back as its neighbour — and the slots
        // are *positions*, so this is also where reordering `ALL` reports that
        // they need renumbering.
        for kind in Kind::ALL {
            assert_eq!(
                slot(kind),
                kind,
                "{:?} claims the slot `ALL` fills with {:?}; the slots are \
                 positions in `ALL`, so reordering it renumbers them",
                kind,
                slot(kind)
            );
        }
    }

    #[test]
    fn the_set_is_nineteen_distinct_kinds() {
        // What is left for this to catch once [`slot`] forces every variant
        // *into* `ALL`: the other direction, where `ALL` holds something the
        // enum did not gain — a duplicated entry, or one paired with a
        // miscount. `slot` is quiet on both (a duplicate still reads back as
        // itself), and the figure is the spec's own.
        assert_eq!(Kind::ALL.len(), 19);
        let mut labels: Vec<&str> = Kind::ALL.iter().map(|k| k.label()).collect();
        labels.sort_unstable();
        let distinct = labels.len();
        labels.dedup();
        assert_eq!(
            labels.len(),
            distinct,
            "two kinds share a label: {labels:?}"
        );
    }

    /// The five kinds a session may name as a review chain's producer, spelled
    /// out here because nothing in the crate derives them any longer. This is
    /// the set `content/TASK-FORMAT.md` teaches; if it and the enum ever
    /// disagree, the tests below are where that shows up.
    const PRODUCERS: [Kind; 5] = [
        Kind::Requirements,
        Kind::Design,
        Kind::Planning,
        Kind::Prototype,
        Kind::Impl,
    ];

    /// Is this kind a **producer** — one a session may cut a `review-` and an
    /// `integrate-review-` step for?
    ///
    /// Deliberately an **exhaustive `match`** rather than a lookup in
    /// [`PRODUCERS`], and that is the whole point of the function. Deleting
    /// `Kind::review_steps` took the crate's last exhaustive match over the
    /// enum, leaving `PRODUCERS` and [`Kind::ALL`] as two hand-maintained
    /// rosters that can only disagree at *runtime* — a twentieth kind added as
    /// a producer without its two steps compiles, and every test below still
    /// passes, because they check the five that are listed rather than the set
    /// that exists. This match restores the compile-time force: a new variant
    /// fails to build until someone classifies it, at the one place where
    /// "producer or not" is the question being answered.
    ///
    /// Test-only on purpose. Production has no caller — the derivation became
    /// guidance a session applies, not a constructor input — and dead API is
    /// what `src/lib.rs`'s header says this crate deletes on sight.
    fn is_producer(kind: Kind) -> bool {
        match kind {
            Kind::Requirements | Kind::Design | Kind::Planning | Kind::Prototype | Kind::Impl => {
                true
            }
            // A pair's steps are cut together by `leaf-add-pair`, and a chain on
            // a chain's output (`review-review-impl`) is exactly what the closed
            // set exists to make unspellable.
            Kind::ResearchA
            | Kind::ResearchB
            | Kind::CombineResearch
            | Kind::Finish
            | Kind::ReviewRequirements
            | Kind::ReviewDesign
            | Kind::ReviewPlanning
            | Kind::ReviewPrototype
            | Kind::ReviewImpl
            | Kind::IntegrateReviewRequirements
            | Kind::IntegrateReviewDesign
            | Kind::IntegrateReviewPlanning
            | Kind::IntegrateReviewPrototype
            | Kind::IntegrateReviewImpl => false,
        }
    }

    #[test]
    fn every_kind_is_classified_and_the_producer_roster_agrees_with_that_classification() {
        // The check the hand-written rosters cannot make for themselves: the
        // exhaustive classifier is the authority, and both `PRODUCERS` and the
        // labels that actually carry `review-` steps are held to it. A twentieth
        // kind now fails to *compile* until it is classified, and then fails
        // *here* if it was classified a producer without gaining its two steps.
        let classified: Vec<Kind> = Kind::ALL.into_iter().filter(|k| is_producer(*k)).collect();
        assert_eq!(
            classified,
            PRODUCERS.to_vec(),
            "PRODUCERS must list exactly the kinds the exhaustive match calls producers"
        );

        for kind in Kind::ALL {
            let has_steps = Kind::ALL
                .iter()
                .any(|other| other.label() == format!("review-{}", kind.label()))
                && Kind::ALL
                    .iter()
                    .any(|other| other.label() == format!("integrate-review-{}", kind.label()));
            assert_eq!(
                is_producer(kind),
                has_steps,
                "{:?} is classified {} but {} its two chain steps",
                kind,
                if is_producer(kind) {
                    "a producer"
                } else {
                    "not a producer"
                },
                if has_steps { "has" } else { "lacks" }
            );
        }
    }

    #[test]
    fn every_producer_s_chain_steps_are_its_own_labels_prefixed() {
        // The invariant that makes the hand-written convention work at all: a
        // session told to cut `--kind review-<producer>` must always name a
        // kind the set really has. Written against the labels rather than the
        // variants, because that is the direction a session reads — and getting
        // it wrong there is well-formed, so nothing downstream catches it.
        for producer in PRODUCERS {
            for step in [
                format!("review-{}", producer.label()),
                format!("integrate-review-{}", producer.label()),
            ] {
                assert_eq!(
                    Kind::parse(&step).map(|k| k.label()).ok(),
                    Some(step.as_str()),
                    "a session cutting {step:?} by hand must reach a real kind"
                );
            }
        }
    }

    #[test]
    fn exactly_the_five_producers_have_chain_steps() {
        // The count is the load-bearing half. `research-*` gaining steps would
        // make `leaf-add-pair` redundant; a *step* gaining them would let a
        // chain be built on a chain's output — `review-review-impl` — and the
        // check that this cannot happen is that no such label exists.
        let labelled: Vec<&str> = Kind::ALL
            .iter()
            .map(|k| k.label())
            .filter(|label| {
                Kind::ALL
                    .iter()
                    .any(|other| other.label() == format!("review-{label}"))
            })
            .collect();
        assert_eq!(
            labelled,
            vec!["requirements", "design", "planning", "prototype", "impl"]
        );
        assert_eq!(
            labelled.len(),
            PRODUCERS.len(),
            "PRODUCERS and the labels that actually carry steps must agree"
        );
    }

    #[test]
    fn every_kind_label_round_trips_through_parse() {
        for k in Kind::ALL {
            assert_eq!(Kind::parse(k.label()).unwrap(), k);
            assert_eq!(Kind::parse_read(k.label()), Some(k));
        }
    }

    #[test]
    fn filename_kind_labels_never_prefix_one_another() {
        for kind in Kind::ALL {
            let prefix = format!("{}-", kind.label());
            for other in Kind::ALL {
                if kind != other {
                    assert!(
                        !other.label().starts_with(&prefix),
                        "kind labels {:?} and {:?} make filename parsing ambiguous",
                        kind.label(),
                        other.label()
                    );
                }
            }
        }
    }

    #[test]
    fn the_labels_are_exactly_the_taxonomy_spellings() {
        // Pinned verbatim: these strings are a public surface (task files on
        // disk, env-var suffixes, `grove-llm kind` output), so a "harmless"
        // rename must fail here rather than silently orphan a live grove.
        let labels: Vec<&str> = Kind::ALL.iter().map(|k| k.label()).collect();
        assert_eq!(
            labels,
            vec![
                "requirements",
                "design",
                "planning",
                "prototype",
                "impl",
                "research-a",
                "research-b",
                "combine-research",
                "finish",
                "review-requirements",
                "review-design",
                "review-planning",
                "review-prototype",
                "review-impl",
                "integrate-review-requirements",
                "integrate-review-design",
                "integrate-review-planning",
                "integrate-review-prototype",
                "integrate-review-impl",
            ]
        );
    }

    #[test]
    fn kind_parse_rejects_unknown_labels() {
        assert!(Kind::parse("bogus").is_err());
        assert!(Kind::parse("").is_err());
        // Near-misses that must not be smuggled in by a loose matcher.
        assert!(
            Kind::parse("review").is_err(),
            "`review` is a chain-step prefix, not a kind"
        );
        assert!(Kind::parse("integrate-review").is_err());
        assert!(Kind::parse("Impl").is_err(), "labels are lowercase");
    }

    #[test]
    fn kind_parse_error_lists_every_kind() {
        let err = Kind::parse("reserch").unwrap_err().to_string();
        for k in Kind::ALL {
            assert!(
                err.contains(k.label()),
                "error {err:?} missing {:?}",
                k.label()
            );
        }
    }

    // The `work` → `impl` rename, both halves. The asymmetry *is* the decision
    // (task-kind-taxonomy): read silently accepts the previous spelling so the
    // six live groves whose task files say `work` keep working untouched; write
    // refuses it so a human authoring a new leaf is retrained once.
    #[test]
    fn work_reads_as_impl_but_is_refused_on_write() {
        assert_eq!(Kind::parse_read("work"), Some(Kind::Impl));
        let err = Kind::parse("work").unwrap_err().to_string();
        assert!(
            err.contains("impl"),
            "the error must name the replacement: {err:?}"
        );
        assert!(err.contains("work"), "…and the word it replaces: {err:?}");
    }

    #[test]
    fn legacy_research_reads_as_research_a_but_is_refused_on_write() {
        assert_eq!(Kind::parse_read("research"), Some(Kind::ResearchA));
        let err = Kind::parse("research").unwrap_err().to_string();
        assert!(err.contains("research-a"), "{err}");
        assert!(err.contains("research-b"), "{err}");
    }

    // Longest match, stated as the property that makes it necessary: the two
    // chain-step prefixes are not disjoint as strings — `integrate-review-impl`
    // contains `review-impl` — so any `contains`-style reasoning over these
    // labels would pair an integration step with itself, or route it as the
    // review it is supposed to hand back. `split_filename_prefix`'s longest
    // match is what keeps filename parsing honest about it.
    #[test]
    fn the_chain_step_prefixes_overlap_which_is_why_matching_is_longest_first() {
        assert!(Kind::IntegrateReviewImpl
            .label()
            .contains(Kind::ReviewImpl.label()));
        assert_eq!(
            Kind::split_filename_prefix("integrate-review-impl-fix-k4"),
            Some((Kind::IntegrateReviewImpl, "fix-k4")),
            "naive matching would read this as review-impl with slug `impl-fix-k4`"
        );
    }

    #[test]
    fn a_chain_step_prefix_is_never_a_kind() {
        // `review` and `integrate-review` are how the two steps' labels are
        // *built*, not labels themselves. If either parsed as a kind, a leaf
        // could be written whose filename names a step of nothing.
        for prefix in ["review", "integrate-review"] {
            assert!(Kind::parse(prefix).is_err(), "{prefix}");
            assert_eq!(Kind::parse_read(prefix), None, "{prefix}");
        }
    }

    #[test]
    fn parse_read_rejects_a_genuinely_unknown_label() {
        // The alias must not turn `parse_read` into "anything goes" — an
        // unrecognised wire or legacy-migration token still comes back `None`.
        assert_eq!(Kind::parse_read("reserch"), None);
        assert_eq!(Kind::parse_read(""), None);
    }
}

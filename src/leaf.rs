// Shared leaf vocabulary that outlived the old-format reader/grower (swept in
// 090/9.4 once the 060 migration unwired the old verb path). Two items survive:
//
//   * `Kind` — the leaf-kind enum, a **closed, parameterised set of seventeen**
//     (five producers, each with its own `review-` and `integrate-review-` step,
//     plus `research` and `combine-research` — ADR `task-kind-taxonomy`,
//     membership in `docs/specs/task-kind-taxonomy.md`) written into a task
//     file's `**Kind:**` line. Live: the grow/lifecycle verbs (`tree_grow` /
//     `tree_lifecycle`) and the `grove-llm` CLI surface (`llm_cli`) parse and
//     carry it.
//   * `split_prefix` — the old-format `NNN-slug` prefix parser. Per task-tree-scheme the
//     *only* surviving consumer of the old format is `grove migrate`, which
//     reads an old tree exactly once on adoption; this is the reader it leans on.
//
// Everything else this module once held — the `NNN-slug` / `done/` reader and
// grower (`add`, `insert`, header rewriting, cross-ref surfacing) — was the old
// verb path, made dead by the 060 migration and deleted in 090.

use anyhow::{bail, Result};

/// The previous spelling of `impl`, still accepted on **read** so a live grove's
/// task files keep working across the rename (task-kind-taxonomy). Refused on
/// write, with an error naming the replacement — see [`Kind::parse`].
const WORK_ALIAS: &str = "work";

/// A leaf's declared session kind — a closed, **parameterised** set of
/// seventeen (ADR `task-kind-taxonomy`): five producers, each with its own
/// `review-` and `integrate-review-` step, plus `research` and
/// `combine-research`. Adding an eighteenth is a deliberate code change, never
/// a free-text label a leaf may coin. Only `Planning` carries methodological
/// force (the loop's sole Execute branch, the only kind that grows the tree);
/// every other kind produces some artifact, differing in discipline and in
/// routing (`model-per-task-kind`). Each kind's discipline and its HITL/AFK
/// mark are `docs/specs/task-kind-taxonomy.md`; this enum owns only the set and
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
    Research,
    CombineResearch,
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
    /// integrations — the order `docs/specs/task-kind-taxonomy.md` presents
    /// them). The single source of truth for the *set*, as [`Kind::label`] is
    /// for its *spelling*: parsing, the `--kind` error listing, and the loop
    /// driver's env-var sweep all derive from these two, so none of them can
    /// drift from the enum. That mattered little at five kinds and matters a
    /// lot at seventeen.
    pub const ALL: [Kind; 17] = [
        Kind::Requirements,
        Kind::Design,
        Kind::Planning,
        Kind::Prototype,
        Kind::Impl,
        Kind::Research,
        Kind::CombineResearch,
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

    /// Write gates (task-kind-taxonomy): a grow verb rejects an unrecognised
    /// `--kind` with an error listing the whole set, so a typo is caught at
    /// authoring time, when a human is present to fix it. Read degrades
    /// instead — see [`Kind::parse_read`] and its caller `tree_read::read_kind`,
    /// the read-path counterpart that never bails through this error.
    ///
    /// `work` is refused here rather than silently accepted: it is the previous
    /// spelling of `impl`, and the gate exists to retrain a human who is present
    /// to be retrained. The error names the replacement instead of only listing
    /// the seventeen, because "not in the list" is unhelpful for a word that was
    /// correct last week.
    pub fn parse(s: &str) -> Result<Kind> {
        if let Some(kind) = Kind::from_label(s) {
            return Ok(kind);
        }
        if s == WORK_ALIAS {
            bail!(
                "--kind `work` was renamed `impl` (task-kind-taxonomy); use `--kind impl`. \
                 A task file still saying `**Kind:** work` keeps reading as `impl` and needs \
                 no edit."
            );
        }
        bail!("--kind must be one of {}, got {:?}", Kind::label_list(), s)
    }

    /// The read-side counterpart of [`Kind::parse`], and the reason the
    /// `work` → `impl` rename is safe: it additionally accepts `work` as
    /// `Impl`, **silently**. That asymmetry is grove's existing gate-on-write /
    /// degrade-on-read rule (task-kind-taxonomy), not a new one — an alias is a
    /// read-side concession, and a warning on a file whose only sin is the
    /// previous spelling would be noise on every task file of every live grove.
    ///
    /// `None` means genuinely unrecognised; deciding what *that* means is the
    /// caller's (`tree_read::read_kind` warns and degrades to `Impl`).
    pub(crate) fn parse_read(s: &str) -> Option<Kind> {
        Kind::from_label(s).or_else(|| (s == WORK_ALIAS).then_some(Kind::Impl))
    }

    /// The lowercase label written into a task file's `**Kind:**` line and
    /// printed by `grove-llm kind`. The single source of truth for the label
    /// direction: the grow verbs' leaf template (`tree_grow`), the `kind` verb,
    /// [`Kind::from_label`], and the loop driver's env-var suffixes all read
    /// through it, so none of them can disagree on the spelling.
    pub fn label(self) -> &'static str {
        match self {
            Kind::Requirements => "requirements",
            Kind::Design => "design",
            Kind::Planning => "planning",
            Kind::Prototype => "prototype",
            Kind::Impl => "impl",
            Kind::Research => "research",
            Kind::CombineResearch => "combine-research",
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
    fn from_label(s: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.label() == s)
    }

    /// Every label, backtick-quoted and comma-joined, for the `--kind` error.
    /// Built from [`Kind::ALL`] so a new kind is listed the moment it exists.
    fn label_list() -> String {
        Kind::ALL
            .iter()
            .map(|k| format!("`{}`", k.label()))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

/// Split `NNN-rest` (NNN exactly three ASCII digits) into `(NNN, rest)`. The
/// old-format prefix parser; `pub(crate)` so `grove migrate` (060/020) can reuse
/// it read-only to consume an old `NNN-slug` tree (it is the *only* surviving
/// consumer of the old format — task-tree-scheme).
pub(crate) fn split_prefix(name: &str) -> Option<(u32, &str)> {
    if name.len() < 4 {
        return None;
    }
    let (head, tail) = name.split_at(3);
    if !head.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !tail.starts_with('-') {
        return None;
    }
    head.parse::<u32>().ok().map(|n| (n, &tail[1..]))
}

#[cfg(test)]
mod inline_tests {
    use super::*;

    #[test]
    fn split_prefix_accepts_three_digits_then_dash() {
        assert_eq!(split_prefix("010-foo"), Some((10, "foo")));
        assert_eq!(split_prefix("990-x-y"), Some((990, "x-y")));
    }

    #[test]
    fn split_prefix_rejects_wrong_shapes() {
        assert!(split_prefix("BRIEF.md").is_none());
        assert!(split_prefix("01-foo").is_none()); // two digits, not three
        assert!(split_prefix("0100-foo").is_none()); // four-digit prefix
        assert!(split_prefix("010_foo").is_none()); // wrong separator
    }

    #[test]
    fn the_set_is_seventeen_distinct_kinds() {
        // The count and the distinctness are the two things `ALL` can get wrong
        // that the compiler cannot catch (a duplicated entry, a forgotten one
        // paired with a duplicate). Spelled out against the spec's own figure.
        assert_eq!(Kind::ALL.len(), 17);
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

    #[test]
    fn every_kind_label_round_trips_through_parse() {
        for k in Kind::ALL {
            assert_eq!(Kind::parse(k.label()).unwrap(), k);
            assert_eq!(Kind::parse_read(k.label()), Some(k));
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
                "research",
                "combine-research",
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
            "`review` is a family, not a kind"
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
    fn parse_read_still_reports_a_genuinely_unknown_label() {
        // The alias must not turn `parse_read` into "anything goes" — an
        // unrecognised token still comes back `None`, so `read_kind` can warn.
        assert_eq!(Kind::parse_read("reserch"), None);
        assert_eq!(Kind::parse_read(""), None);
    }
}

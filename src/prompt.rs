//! The **guaranteed core** — the whole of `${prompt}`.
//!
//! A session receives a couple of KiB pointing at the provisioned skill, not the
//! methodology itself. What earns a place here is settled by the **too-late
//! test** (ADR *skill-delivers-the-methodology*, *The core rule*): a
//! sentence rides `${prompt}` only if its failure mode is one the skill cannot
//! repair, because by the time the skill could speak the moment has passed.
//! Three shapes pass — a fact only the driver holds, the instruction to open the
//! skill, and the session's last action — and the test is closed on the word
//! *fact*:
//!
//! > A driver fact is a **launch-varying value** that `content/` cannot know at
//! > build time. Its static meaning, and every normative consequence of it, stay
//! > in the skill.
//!
//! That closure is why the runtime facts below are bare values. *This selection
//! is authoritative*, *do not probe for the version control* — both are rules
//! with counterparts in `content/` ([`crate::methodology`]'s embed states them
//! in `SKILL.md`), and restating either here would be the second source this
//! design exists to avoid.
//!
//! **The prose drift surface is zero bytes**, and the property is structural
//! rather than a claim about size: the prompt is a fixed template in which
//! exactly one part is embedded content, for **every** kind. The load
//! instruction and the provisioned locations have no counterpart in `content/` —
//! a skill cannot tell you to read it, and cannot know which directories a
//! particular driver wrote — and the ending is the embed's own signal file,
//! inlined byte-exact. Two signal files serve the nineteen kinds, because
//! `finish`'s ending is a choice between three outcomes rather than one
//! instruction; which file a kind takes varies, that it takes one does not.
//!
//! **Four couplings are shared with the other channel**, and two are closed by
//! construction: the inlined signal file is embedded at compile time, and the
//! locations are computed by the same registry that writes them. The other two —
//! the skill *name* this module states, and the reference path each kind maps
//! to — are closed by assertion in `tests/prompt.rs`, and both fail by name.
//!
//! This module **depends on** [`crate::methodology`] rather than absorbing it:
//! composition reads the embed to inline the ending and to assert its mapped
//! paths exist. Moving embed ownership here would put provisioning's supplier
//! behind a prompt-composition seam.

use crate::leaf::Kind;

use anyhow::{Context, Result};
use std::path::Path;

/// The name the core tells a session to load, and the name a harness will have
/// registered the provisioned directory under.
///
/// Stated here rather than read from the embed: the core is driver prose, and a
/// name recovered from the very document it points at could not detect the two
/// disagreeing. `tests/prompt.rs` asserts it against the embedded `SKILL.md`'s
/// own `name:`, which is the coupling made to fail by name.
pub const SKILL_NAME: &str = "grove";

/// The embed-relative path of the session-ending text for the eighteen kinds
/// that end exactly one way, inlined verbatim as the core's third part.
const SIGNAL_FILE: &str = "SIGNAL.md";

/// The same, for the one kind whose ending is a **choice** rather than an
/// instruction. See [`ending_file`] for why it is a second file and not an
/// omission.
const FINISH_SIGNAL_FILE: &str = "SIGNAL-FINISH.md";

/// **Part 1 — the load instruction.** First, because it is the first action.
///
/// The wording is lifted verbatim from
/// `docs/research/wording-micro-test.md`, *The winning wording*, and the three
/// elements it carries are the ones that arm of the test measured:
///
/// * **One imperative naming both targets** — the skill by name and this kind's
///   reference file by path. The driver resolved the kind before the session
///   existed, so the session performs no selection and has nothing to defer.
///   This is the element the test measured as load-bearing: the control opened
///   `SKILL.md` in 9 of 10 sessions and reached its kind's *procedure* only
///   after starting work, in every session of both arms.
/// * **An ordering clause that enumerates the tempting alternatives.** The
///   enumeration is what makes an ordering clause bite; "read this first"
///   without it is advice.
/// * **The provisioned directories, by absolute path** — what makes the
///   instruction actionable by plain file read, the one capability every harness
///   has.
///
/// **A rationalization table and a *this prompt is not a summary* clause were
/// designed and are not here.** An ablation carrying only the three elements
/// above scored identically to the full wording (10/10 against the control's
/// 0/10), so nothing measured is attributable to them. That is a weaker result
/// than it looks — the variant is at ceiling, so only a *large* negative effect
/// was detectable — and the honest ground for the cut is that the two elements
/// are **unmeasured**, and unmeasured prose does not ride the one channel a
/// session cannot skip. `docs/research/wording-micro-test.md` carries the reopen
/// condition, which `delivery-acceptance-k11` is the setting for.
const LOAD_INSTRUCTION: &str = "\
**Load the `{skill}` skill now, and read its `{reference}`.** Your kind is
`{kind}`; Grove resolved that before this session existed, so there is no
selection for you to make — those two files, in that order.

Do this **first**, before anything else you might reach for: before reading the
task file, before running any `grove-llm` verb, before looking at `.grove/`,
before inspecting the working tree, and before answering a question.

The skill was provisioned for you at {locations}. If your harness offers no
skill-loading affordance, read `SKILL.md` and `{reference}` under one of those
directories directly, by path.
";

/// **Part 2 — the runtime facts.** Two launch-varying values and no normative
/// tail; see this module's header for why the tail left.
const RUNTIME_FACTS: &str = "\
Grove mandate: the leaf selected for this session is `{handle}`.

Version control: {stated_vcs}.
";

/// What stands in for the location list on a machine with no known harness root.
///
/// The launch proceeds — Grove reports and never refuses — and
/// [`crate::provision::report_absent_skill_destination`] has already said so
/// loudly on the same iteration. The core still has to say something true, and
/// "provisioned at nowhere" is the true thing.
const NO_LOCATIONS: &str = "(nowhere — this machine has no known harness root)";

/// The reference file a session of `kind` is told to read, embed-relative.
///
/// **An exhaustive `match`, in the driver**, because routing by kind is what the
/// driver already owns. Its hazard — a twentieth kind silently absorbed into a
/// family's file — is closed the way this repository closes such things: a match
/// with no wildcard fails to compile until someone classifies the new variant,
/// and `tests/prompt.rs` asserts that every path this yields exists in the
/// embed.
///
/// Ten files for nineteen kinds: each family shares one, which is the same
/// grouping the skill's own routing table prints for a session that arrived
/// without a mandate.
pub fn reference_file(kind: Kind) -> &'static str {
    match kind {
        Kind::Requirements => "references/requirements.md",
        Kind::Design => "references/design.md",
        Kind::Planning => "references/planning.md",
        Kind::Prototype => "references/prototype.md",
        Kind::Impl => "references/impl.md",
        Kind::ResearchA | Kind::ResearchB => "references/research.md",
        Kind::CombineResearch => "references/combine-research.md",
        Kind::Finish => "references/finish.md",
        Kind::ReviewRequirements
        | Kind::ReviewDesign
        | Kind::ReviewPlanning
        | Kind::ReviewPrototype
        | Kind::ReviewImpl => "references/review.md",
        Kind::IntegrateReviewRequirements
        | Kind::IntegrateReviewDesign
        | Kind::IntegrateReviewPlanning
        | Kind::IntegrateReviewPrototype
        | Kind::IntegrateReviewImpl => "references/integrate-review.md",
    }
}

/// The session-ending text for `kind` — the embed's own bytes.
///
/// Public as the seam the ending's checks run through: a driver-side copy
/// reappearing as a Rust literal fails against this.
pub fn ending_of(kind: Kind) -> Result<&'static str> {
    embedded(ending_file(kind), "the session-ending text")
}

/// The embed-relative path of the session-ending text for `kind`.
///
/// **`finish` gets a different file, not no file, and the distinction is the
/// whole point.** Eighteen kinds end exactly one way — retire, commit,
/// `grove-llm complete` — and `SIGNAL.md` says so. A `finish` session has
/// *three* endings, chosen by what it did: `complete --done` after teardown,
/// bare `complete` if it externalised work instead, and no signal at all if the
/// human declined or was absent. Inlining `SIGNAL.md` for it would put *run
/// `grove-llm complete`* last in the prompt of the one session that may have
/// just deleted the task tree — relaunching the loop onto a torn-down grove,
/// which then scaffolds a fresh one. The too-late test admits "the session's
/// last action"; it does not license stating the wrong one.
///
/// **Omitting the part instead was the wrong reading of that.** A `finish`
/// session's failure mode is the one the core exists for and is the worst
/// instance of it: teardown completes, the session forgets `--done`, and the
/// loop it was supposed to stop is left waiting on a session that will not end.
/// Reading the branches in the skill at bootstrap cannot repair a forgotten
/// signal an hour later, which is exactly what the too-late test admits a
/// sentence for. So the three-outcome *choice* gets an embedded source of its
/// own — `SIGNAL-FINISH.md`, which `references/finish.md` routes to rather than
/// restating — and the core inlines those bytes. One source, two deliveries, and
/// every kind's prompt is three parts with exactly one of them embedded content.
///
/// Exhaustive for the same reason [`reference_file`] is: a twentieth kind must
/// not inherit an ending by falling through a wildcard.
fn ending_file(kind: Kind) -> &'static str {
    match kind {
        Kind::Finish => FINISH_SIGNAL_FILE,
        Kind::Requirements
        | Kind::Design
        | Kind::Planning
        | Kind::Prototype
        | Kind::Impl
        | Kind::ResearchA
        | Kind::ResearchB
        | Kind::CombineResearch
        | Kind::ReviewRequirements
        | Kind::ReviewDesign
        | Kind::ReviewPlanning
        | Kind::ReviewPrototype
        | Kind::ReviewImpl
        | Kind::IntegrateReviewRequirements
        | Kind::IntegrateReviewDesign
        | Kind::IntegrateReviewPlanning
        | Kind::IntegrateReviewPrototype
        | Kind::IntegrateReviewImpl => SIGNAL_FILE,
    }
}

/// One embedded file's text, by embed-relative path.
///
/// A missing path fails the launch rather than degrading it, and names both the
/// path and what wanted it. The mapping is asserted against the embed in the
/// suite, so reaching this error means the two moved apart between a test run
/// and a build — which is a bug, not a contributor's mistake, and still not a
/// reason to hand a session a prompt with a hole in it.
fn embedded(path: &str, wanted_by: &str) -> Result<&'static str> {
    let file = crate::methodology::embed()
        .get_file(path)
        .with_context(|| format!("the embedded methodology carries no content/{path}"))
        .with_context(|| format!("composing the guaranteed core: {wanted_by}"))?;
    std::str::from_utf8(file.contents())
        .with_context(|| format!("embedded content/{path} is not UTF-8"))
}

/// Render the provisioned skill directories for the load instruction.
///
/// Backticked absolute paths, in registry order, comma-joined with a final
/// "and" — a list a human reads and a session can paste into a file read.
fn render_locations<P: AsRef<Path>>(locations: &[P]) -> String {
    let rendered: Vec<String> = locations
        .iter()
        .map(|path| format!("`{}`", path.as_ref().display()))
        .collect();
    match rendered.split_last() {
        None => NO_LOCATIONS.to_string(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// Compose the whole of `${prompt}` for one launch.
///
/// Three parts in the session's own timeline order — load instruction, runtime
/// facts, ending — joined by a blank line, for every kind. Which file the ending
/// comes from varies ([`ending_file`]); that there is one does not.
///
/// **Recency is the whole reason the ending is last**, and the reason is
/// inherited rather than re-derived: it was moved to compose last after sessions
/// were seen finishing correctly and then not signalling. Its advantage is much
/// weaker here — it trails ~1.5 KiB rather than seven files — so the position is
/// nearly free and buys correspondingly less.
///
/// `stated_vcs` is a **value**, rendered by the caller: this module states no
/// consequence of it.
pub fn compose<P: AsRef<Path>>(
    kind: Kind,
    handle: &str,
    stated_vcs: &str,
    locations: &[P],
) -> Result<String> {
    let reference = reference_file(kind);
    // Asserted at composition time, not only in the suite: a session pointed at
    // a reference file the embed does not carry is told to read nothing.
    embedded(reference, "this kind's reference file")?;

    let load = LOAD_INSTRUCTION
        .replace("{skill}", SKILL_NAME)
        .replace("{reference}", reference)
        .replace("{kind}", kind.label())
        .replace("{locations}", &render_locations(locations));
    let facts = RUNTIME_FACTS
        .replace("{handle}", handle)
        .replace("{stated_vcs}", stated_vcs);

    let ending = ending_of(kind)?;
    Ok([load, facts, ending.to_string()].join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The list renderer's three shapes. Pinned here rather than through
    /// [`compose`] because the empty case is the one a real embed never
    /// produces, and it is the case a machine with no harness root hits.
    #[test]
    fn locations_render_as_a_readable_list() {
        assert_eq!(render_locations::<&Path>(&[]), NO_LOCATIONS);
        assert_eq!(render_locations(&[Path::new("/a")]), "`/a`");
        assert_eq!(
            render_locations(&[Path::new("/a"), Path::new("/b")]),
            "`/a` and `/b`"
        );
        assert_eq!(
            render_locations(&[Path::new("/a"), Path::new("/b"), Path::new("/c")]),
            "`/a`, `/b` and `/c`"
        );
    }

    /// Every substitution is spent. A placeholder surviving into a session's
    /// prompt is the failure a `format!` would have caught at compile time and a
    /// `replace` chain cannot, so it is caught here instead — over the whole
    /// kind set, so a shape only one kind reaches cannot hide.
    #[test]
    fn no_placeholder_survives_composition() {
        for kind in Kind::ALL {
            let prompt = compose(
                kind,
                "slug-k1",
                "this working tree is jj-enabled (jj workspace root: `/w`)",
                &["/x"],
            )
            .expect("the real embed must compose");
            assert!(
                !prompt.contains('{') && !prompt.contains('}'),
                "an unspent placeholder reached {}'s prompt:\n{prompt}",
                kind.label()
            );
        }
    }
}

//! **The lifecycle invariants the corpus refactor must not change**, asserted
//! over the loaded path a session of each kind actually opens.
//!
//! This is the net the corpus rewrites fall into. Every claim here is written
//! against the corpus **as it stands**, so the file is green before those
//! rewrites and must stay green after; a rule that moves between files is
//! expected, a rule that stops being reachable is the failure.
//!
//! **What is asserted, and what is not.** A `cargo test` cannot watch a session
//! behave, so what runs here is the *delivery* half of a behavioural claim: the
//! rule is present, and reachable, on the loaded path of every kind it binds. The
//! boundary is stated rather than blurred, and the reasoning — including why a
//! model-in-the-loop evaluation is the wrong standing instrument — is
//! `docs/adr/behavioural-coverage-asserts-delivery.md`. A green run is not
//! evidence that a session obeyed.
//!
//! **The loaded path is composed, not enumerated** ([`loaded_path`]): the
//! guaranteed core `src/prompt.rs` builds for that kind, `content/SKILL.md`,
//! `prompt::reference_file(kind)`, and the transitive closure of the corpus files
//! those name by path. Two consequences do real work:
//!
//! * A rule surviving only in a file no session's path reaches fails here, which
//!   a sweep over the whole corpus could not see.
//! * The path is genuinely **per kind**, so `signal-is-the-last-action` is
//!   asserted on the eighteen kinds that end one way and
//!   `finish-three-endings` on the one kind that chooses between three — and each
//!   is asserted *absent* from the other's path.
//!
//! **Being on the path is necessary and not sufficient: delivery goes through the
//! rule's own trigger** ([`delivery_site`]). What a session has to be *holding* is
//! the rule's condition — it cannot look up a procedure it does not know applies —
//! so a rule counts as delivered exactly two ways: it is stated on the **static**
//! path (`${prompt}`, `SKILL.md`, this kind's reference file), which needs no
//! trigger; or a paragraph on the path states **this rule's situation** and the
//! procedure is in that paragraph's own file or in a file it names by path. A file
//! reached by some *other* condition, carrying the procedure for a condition that
//! is gone, is the deleted-in-effect state this file exists to reject.
//!
//! **Every claim carries its own control.** Each claim names a plausible rewrite
//! that keeps the rule's subject and drops *that claim's* force, and is shown
//! failing; the reachability walk is shown following a named path, stopping when
//! the naming sentence goes, refusing to reach a file nothing names, and refusing
//! to deliver a rule whose own condition was deleted while a sibling condition
//! still opens its owner.
//!
//! Scope is the **nine** required areas of
//! `docs/specs/corpus-rule-ownership.md`. Eight of them are the **B★** set, and
//! they were written here by `behavior-evals-k3` against the corpus as it stood.
//! The ninth — the requirements interview threshold — could not be: it is a
//! contradiction being *resolved* rather than a behaviour being preserved, so it
//! was red until `kind-references-k5` stated the threshold once, and it joined
//! this table with that fix. It is green from that commit onward, on the same
//! terms as the other eight.

use grove::leaf::Kind;
use grove::{methodology, prompt};
use std::collections::{BTreeMap, BTreeSet};

// -- The corpus, and the loaded path over it ---------------------------------

/// The embedded corpus as a path→text map. Owned rather than borrowed because
/// every reachability control is a *mutant* of it.
type Corpus = BTreeMap<String, String>;

/// One member of a loaded path: where the bytes came from, and the bytes.
type Loaded = (String, String);

/// The pseudo-path the composed guaranteed core occupies. It is not a corpus
/// file — it is the driver's own template with one embedded file inlined — and it
/// is on the path of every session because `${prompt}` is the one channel a
/// session cannot skip.
const CORE: &str = "${prompt}";

/// How many leading members of a [`loaded_path`] are **static** — the only three
/// things `src/prompt.rs` puts on a path unconditionally, and therefore the only
/// places a rule can be stated with no condition sending a session there.
const STATIC_MEMBERS: usize = 3;

fn corpus() -> Corpus {
    methodology::markdown_files()
        .expect("the real embed must be readable")
        .into_iter()
        .map(|(path, text)| (path, text.to_owned()))
        .collect()
}

/// The guaranteed core for `kind`, composed through the production seam so that
/// which signal file it inlines is the driver's decision and not this file's.
fn core(kind: Kind) -> String {
    prompt::compose(
        kind,
        "lifecycle-invariants-k3",
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

/// **The bytes a session of `kind` reads on its normal path**, in the order it
/// reaches them: the composed core, `SKILL.md`, that kind's reference file, then
/// the transitive closure of the corpus files those name.
///
/// The first [`STATIC_MEMBERS`] are the *static* half, and the closure is the
/// conditional half, reached because a sentence a session read named a file by
/// path. The walk is deliberately file-grained: what an edge reaches is a **file**,
/// so once one condition lands a session in `decompose.md` every in-file condition
/// there is available to it (`docs/specs/corpus-rule-ownership.md`, *Cross-file
/// rows only*). Which *rule* that edge delivers is a separate question, and
/// [`delivery_site`] is where it is answered.
fn loaded_path(corpus: &Corpus, kind: Kind) -> Vec<Loaded> {
    let mut path: Vec<Loaded> = vec![(CORE.to_owned(), core(kind))];
    for statik in ["SKILL.md", prompt::reference_file(kind)] {
        let text = corpus.get(statik).unwrap_or_else(|| {
            panic!("the corpus must carry the static path member content/{statik}")
        });
        path.push((statik.to_owned(), text.clone()));
    }
    assert_eq!(
        path.len(),
        STATIC_MEMBERS,
        "the static half of a loaded path is the core, SKILL.md and reference_file(kind); \
         STATIC_MEMBERS is what every delivery check slices by"
    );

    let mut reached: BTreeSet<String> = path.iter().map(|(name, _)| name.clone()).collect();
    let mut next = 0;
    while next < path.len() {
        let (name, text) = path[next].clone();
        next += 1;
        for target in named_files(&name, &text, corpus) {
            if reached.insert(target.clone()) {
                path.push((target.clone(), corpus[target.as_str()].clone()));
            }
        }
    }
    path
}

/// The corpus files `text` names by path — the outgoing edges of one loaded-path
/// member.
///
/// A literal path occurrence is the test, which is sound only because no corpus
/// path is a substring of another; [`no_corpus_path_hides_inside_another`] holds
/// that assumption rather than commenting it.
fn named_files<'a>(source: &str, text: &str, corpus: &'a Corpus) -> Vec<&'a String> {
    let scanned = citing_region(source, text);
    corpus
        .keys()
        .filter(|target| target.as_str() != source && scanned.contains(target.as_str()))
        .collect()
}

/// The part of a file whose file mentions are **edges**.
///
/// For every file but one that is the whole text. `SKILL.md`'s routing table is
/// the exception and the reason is the driver: it prints all ten reference files
/// so that a session arriving *without* a mandate can find its own, and a
/// mandated session was handed one path before it existed. Reading the table as
/// ten edges would put every kind's discipline on every kind's path and collapse
/// the per-kind measure this file rests on —
/// [`the_routing_table_is_a_selector_rather_than_an_edge`] is the control.
fn citing_region(source: &str, text: &str) -> String {
    if source != "SKILL.md" {
        return text.to_owned();
    }
    const TABLE: &str = "## Read your kind's reference file first";
    let mut kept: Vec<&str> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        if line.starts_with(TABLE) {
            inside = true;
            continue;
        }
        if inside && line.starts_with("## ") {
            inside = false;
        }
        if !inside {
            kept.push(line);
        }
    }
    assert_ne!(
        kept.len(),
        text.lines().count(),
        "content/SKILL.md no longer carries a section opening \"{TABLE}\", so the routing \
         table this excision exists for has been renamed. Re-point the heading; excising \
         nothing silently puts all ten reference files on every kind's path."
    );
    kept.join("\n")
}

// -- Matching a rule on a path ------------------------------------------------

/// One alternative wording group. A group is met by **any** of its members, which
/// is what lets a rewrite reword a rule without going red; what stops that from
/// degenerating into a topic match is each claim's own near-miss fixture.
type Group = &'static [&'static str];

/// One statement a rule must make. Every group must occur in a **single
/// paragraph**, so a conjunction assembled out of two unrelated paragraphs does
/// not count as the rule being stated.
struct Claim {
    what: &'static str,
    /// Required, all of them, in one paragraph.
    groups: &'static [Group],
    /// Wordings that **disqualify** the paragraph even when every group is met.
    /// This is how a claim keeps its polarity or its scope where the two endings
    /// differ by a suffix: *run `grove-llm complete` as your last action* and *run
    /// `grove-llm complete --done` as your last action* are different
    /// instructions, and the second must not read as the first.
    without: &'static [&'static str],
    /// A plausible rewrite that keeps the **rule's** subject and drops *this
    /// claim's* force. Asserted to fail this claim, which is what shows the check
    /// is sensitive to its own load-bearing clause rather than to the topic. One
    /// per claim, because an invariant-wide fixture leaves every other claim
    /// uncontrolled — a claim can then be a topic match, or accept the opposite
    /// rule, while the invariant still fails on a sibling.
    near_miss: &'static str,
}

/// Which kinds a rule binds, and therefore whose loaded paths must carry it.
#[derive(PartialEq)]
enum Binds {
    /// All nineteen.
    EveryKind,
    /// The eighteen that end exactly one way — `content/SIGNAL.md`'s scope.
    EveryKindButFinish,
    /// `finish` alone — `content/SIGNAL-FINISH.md`'s scope.
    OnlyFinish,
    /// `requirements` alone. A rule the inventory binds to **one kind** is
    /// `static({requirements})` by construction — rule 2 sends it to that kind's
    /// own reference file, and `SKILL.md` says nothing at all about it — so
    /// binding it here is what asserts it stayed in `references/requirements.md`
    /// rather than drifting into a file the driver never routes a `requirements`
    /// session to.
    OnlyRequirements,
}

struct Invariant {
    /// The rule's id in `docs/specs/corpus-rule-ownership.md`'s inventory.
    rule: &'static str,
    /// The required area it serves, from the nine the requirements name.
    area: &'static str,
    binds: Binds,
    /// **The rule's own situation** — its `Occasion`, in the words the canonical
    /// trigger sentence and the corpus statement share. A conditional rule is
    /// delivered only through a paragraph that states *this* situation, so a
    /// sibling condition naming the same owner file cannot stand in for it.
    ///
    /// Empty for a rule the inventory classes `own` or byte-frozen-and-inlined:
    /// those are stated on the static path or nowhere, and admitting a trigger
    /// for them would weaken that.
    situation: &'static [Group],
    claims: &'static [Claim],
}

impl Binds {
    fn kinds(&self) -> Vec<Kind> {
        Kind::ALL
            .into_iter()
            .filter(|kind| match self {
                Binds::EveryKind => true,
                Binds::EveryKindButFinish => *kind != Kind::Finish,
                Binds::OnlyFinish => *kind == Kind::Finish,
                Binds::OnlyRequirements => *kind == Kind::Requirements,
            })
            .collect()
    }
}

/// One text, normalised for matching: emphasis and code markers dropped,
/// whitespace collapsed, lowercased — so a rule wrapped across lines, carrying
/// `**bold**`, or written as `` `review-*` leaf `` reads the same as one written
/// flat. Applied to the corpus **and** to every wording a claim asks for, so a
/// group member can be written the way the corpus writes it.
///
/// Underscores are deliberately **not** stripped — `GROVE_SIGNAL_FILE` is a name
/// a claim may need to match.
fn normalise(text: &str) -> String {
    text.replace(['*', '`'], "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}

/// A file's paragraphs, normalised.
fn paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(normalise)
        .filter(|block| !block.is_empty())
        .collect()
}

fn meets_all(paragraph: &str, groups: &[Group]) -> bool {
    groups.iter().all(|group| {
        group
            .iter()
            .any(|wording| paragraph.contains(&normalise(wording)))
    })
}

fn paragraph_meets(paragraph: &str, claim: &Claim) -> bool {
    meets_all(paragraph, claim.groups)
        && !claim
            .without
            .iter()
            .any(|wording| paragraph.contains(&normalise(wording)))
}

/// The loaded-path members that state `claim`, in path order. This is *presence*,
/// not delivery: [`delivery_site`] is what decides whether a session is told.
fn claim_sites(path: &[Loaded], claim: &Claim) -> Vec<String> {
    let mut sites = Vec::new();
    for (name, text) in path {
        if states(text, claim) && !sites.contains(name) {
            sites.push(name.clone());
        }
    }
    sites
}

fn states(text: &str, claim: &Claim) -> bool {
    paragraphs(text)
        .iter()
        .any(|paragraph| paragraph_meets(paragraph, claim))
}

/// **Where, and how, a session of this kind is told `claim`** — or `None`.
///
/// Two modes, and no third:
///
/// 1. **Static.** A paragraph of `${prompt}`, `SKILL.md` or this kind's reference
///    file states it. Nothing has to send the session there, so no trigger is
///    involved. This is how an `own` row and an inlined signal file deliver.
/// 2. **Through its own condition.** A paragraph somewhere on the path states the
///    rule's `situation`, and the procedure is either in that paragraph's own file
///    or in a file that paragraph names by path.
///
/// What the second mode refuses is the case the walk alone cannot see: the owner
/// file is on the path because a *different* condition names it, and the condition
/// for *this* rule is gone. The procedure is then present and unreachable — a
/// session that meets the situation has nothing telling it to open the file — which
/// is exactly what `docs/specs/corpus-rule-ownership.md` calls deleted in effect.
fn delivery_site(path: &[Loaded], invariant: &Invariant, claim: &Claim) -> Option<String> {
    if let Some((name, _)) = path
        .iter()
        .take(STATIC_MEMBERS)
        .find(|(_, text)| states(text, claim))
    {
        return Some(name.clone());
    }
    if invariant.situation.is_empty() {
        return None;
    }
    for (name, text) in path {
        for paragraph in paragraphs(&citing_region(name, text)) {
            if !meets_all(&paragraph, invariant.situation) {
                continue;
            }
            for (target, target_text) in path {
                let reached = target == name || paragraph.contains(&target.to_lowercase());
                if reached && states(target_text, claim) {
                    return Some(format!("{name} -> {target}"));
                }
            }
        }
    }
    None
}

/// The claims of `invariant` that nothing on `path` delivers.
fn unmet<'a>(path: &[Loaded], invariant: &'a Invariant) -> Vec<&'a str> {
    invariant
        .claims
        .iter()
        .filter(|claim| delivery_site(path, invariant, claim).is_none())
        .map(|claim| claim.what)
        .collect()
}

// -- The nine areas, and the thirteen rules inside them -----------------------

const AREAS: [&str; 9] = [
    "no second pick",
    "no VCS reprobe",
    "stale launch",
    "the decomposition boundary",
    "human-only pruning",
    "retire -> commit -> complete",
    "the review budget",
    "all three finish-signal outcomes",
    "the interview threshold",
];

const INVARIANTS: &[Invariant] = &[
    Invariant {
        rule: "mandate-is-authoritative",
        area: "no second pick",
        binds: Binds::EveryKind,
        situation: &[],
        claims: &[Claim {
            what: "the driver's single pre-session pick is the mandate, and nothing modulates it",
            groups: &[
                &["authoritative", "nothing modulates", "nothing modulating"],
                &["pick"],
                &["mandate", "selected leaf"],
            ],
            without: &[],
            near_miss: "The driver makes a pick before the session exists and hands you the \
                        selected leaf as your mandate; a second walk normally agrees with it.",
        }],
    },
    Invariant {
        rule: "no-second-pick",
        area: "no second pick",
        binds: Binds::EveryKind,
        situation: &[],
        claims: &[Claim {
            what: "`grove-llm pick` is a diagnostic, and on disagreement the mandate wins",
            groups: &[
                &["grove-llm pick"],
                &["diagnostic"],
                &["the mandate wins", "not this session's dispatcher"],
            ],
            without: &[],
            near_miss: "Run `grove-llm pick` to confirm the mandate you were given; if the two \
                        disagree, take the one that fits the tree.",
        }],
    },
    Invariant {
        rule: "stated-vcs-is-definitive",
        area: "no VCS reprobe",
        binds: Binds::EveryKind,
        situation: &[],
        claims: &[Claim {
            what: "the stated version control is definitive, is not re-derived, and beats a \
                   harness banner",
            groups: &[
                &["definitive", "statement wins"],
                &["re-derive", "probe"],
                &["harness banner"],
            ],
            without: &[],
            near_miss: "The driver states this working tree's version control. A harness banner \
                        may disagree with it, so check the working tree when the two differ.",
        }],
    },
    Invariant {
        rule: "stale-launch-stops",
        area: "stale launch",
        binds: Binds::EveryKind,
        // Canonical trigger 2: *when the mandated handle resolves to nothing or
        // to a terminal leaf*.
        situation: &[&[
            "resolves to nothing",
            "resolving to nothing",
            "does not resolve",
        ]],
        claims: &[Claim {
            what: "a handle resolving to nothing or to a terminal leaf is a stale launch, not \
                   work to redo",
            // "terminal leaf" is the abstraction the inventory and the canonical
            // trigger both use, so a rewrite that stops spelling out both
            // terminal states is licensed rather than red.
            groups: &[
                &["resolves to nothing", "resolving to nothing"],
                &["abandoned", "terminal leaf", "terminal task", "is terminal"],
                &["not work to redo", "rather than work to redo"],
            ],
            without: &[],
            near_miss: "If the handle resolves to nothing, or to a terminal leaf, the tree has \
                        moved on: redo the work against the leaf that is live now.",
        }],
    },
    Invariant {
        rule: "externalize-by-default",
        area: "the decomposition boundary",
        binds: Binds::EveryKind,
        // Canonical trigger 7: *when work surfaces that does not serve this
        // leaf's goal*.
        situation: &[&["does not serve", "not serve"]],
        claims: &[
            Claim {
                what: "work that does not serve this leaf's stated goal goes to the tree",
                groups: &[
                    &["does not serve", "not serve"],
                    &["stated goal", "leaf's goal", "this goal"],
                    &["the tree", "externalis", "externaliz", "leaf-add"],
                ],
                without: &[],
                near_miss: "Work that surfaces mid-session and does not serve this leaf's stated \
                            goal may be absorbed here when you already hold the context for it.",
            },
            Claim {
                what: "the bar for continuing inline is \"fits this session\"",
                groups: &[&["fits this session"]],
                without: &[],
                near_miss: "Absorb the work inline whenever you already hold the context, rather \
                            than externalising it with `leaf-add` — the bar is whether you can \
                            finish it.",
            },
        ],
    },
    Invariant {
        rule: "bigger-than-brief-decomposes",
        area: "the decomposition boundary",
        binds: Binds::EveryKind,
        // Canonical trigger 8: *when this leaf proves bigger than its brief*.
        situation: &[&["proves bigger", "proved bigger", "bigger than its brief"]],
        claims: &[
            Claim {
                what: "a leaf that proves bigger than its brief becomes a node",
                // The second group carries the polarity: "stays a leaf, not a
                // node" mentions a node and states the opposite rule.
                groups: &[
                    &["proves bigger", "proved bigger", "bigger than its brief"],
                    &[
                        "becomes a node",
                        "is a node",
                        "into a node",
                        "leaf-decompose",
                        "decompose it",
                    ],
                ],
                without: &[],
                near_miss: "A leaf that proves bigger than its brief stays a leaf, not a node: \
                            keep going and let the session run as long as it needs.",
            },
            Claim {
                what: "the decomposing session does only the first child",
                groups: &[&["only the first child"]],
                without: &[],
                near_miss: "A leaf that proves bigger than its brief assumed becomes a node with \
                            `grove-llm leaf-decompose`; work through as many of its children as \
                            the session has room for.",
            },
        ],
    },
    Invariant {
        rule: "pruning-is-hitl",
        area: "human-only pruning",
        binds: Binds::EveryKind,
        // Canonical trigger 13: *when this leaf's path looks decided against*.
        // Deliberately **not** the word "prune": the situation is the session's,
        // and the sentence that names the verb is the one being looked up.
        situation: &[&["decided against", "abandonment decision"]],
        claims: &[
            Claim {
                what: "an agent never prunes on its own",
                groups: &[
                    &["prun"],
                    &["never prunes on its own", "an agent never prunes"],
                ],
                without: &[],
                near_miss: "Pruning marks a path decided against with an ABANDONED infix rather \
                            than deleting it: run `grove-llm leaf-prune <path>` once the path is \
                            closed.",
            },
            Claim {
                what: "the session stops and asks rather than deciding the abandonment itself",
                groups: &[
                    &["prun"],
                    &[
                        "explicit human confirmation",
                        "stop and ask",
                        "says so and stops",
                    ],
                ],
                without: &[],
                near_miss: "Pruning is the agent's call once a path is clearly closed: an AFK \
                            session marks the leaf ABANDONED and moves on to the next one.",
            },
        ],
    },
    Invariant {
        rule: "retire-before-commit",
        area: "retire -> commit -> complete",
        binds: Binds::EveryKind,
        // Canonical trigger 12: *when the work is done*, retire before committing.
        situation: &[&["the work is done", "before you commit", "before committ"]],
        claims: &[
            Claim {
                what: "retirement precedes the commit",
                groups: &[
                    &["retire"],
                    &[
                        "before you commit",
                        "before committ",
                        "retire precedes commit",
                    ],
                ],
                without: &[],
                near_miss: "Retire the just-finished leaf by running `grove-llm leaf-retire \
                            <leaf-path>`, which adds a DONE infix in place; commit the task \
                            whenever the work is ready.",
            },
            Claim {
                what: "the rename lands inside that one commit",
                groups: &[
                    &["rename"],
                    &["land inside", "lands in it", "landing inside"],
                ],
                without: &[],
                near_miss: "Retire before you commit; the DONE rename can be committed on its \
                            own once the parent-chain cascade has settled.",
            },
        ],
    },
    Invariant {
        rule: "one-focused-commit",
        area: "retire -> commit -> complete",
        binds: Binds::EveryKind,
        // Canonical trigger 16: *when the leaf is retired*, commit it.
        situation: &[&["the leaf is retired", "once retired", "one focused commit"]],
        claims: &[Claim {
            what: "one task's commit carries the artifact, the grow-verb writes and the rename \
                   together",
            // Four required groups, not one group of alternatives: the rule is
            // the conjunction, so a commit carrying the artifact while the
            // rename rides along later must fail. The first group is the
            // togetherness clause, which the canonical row states as "together"
            // rather than as the phrase "one focused commit".
            groups: &[
                &["one focused commit", "one task's commit", "together"],
                &["artifact"],
                &["grow verb", "grow-verb"],
                &["rename"],
            ],
            // Co-occurrence alone cannot tell "these three together" from "two of
            // them together, the third later" — both name all three. So the
            // deferral is disqualified explicitly.
            without: &[
                "a later commit",
                "a second commit",
                "land later",
                "ride along",
            ],
            near_miss: "One focused commit carries the artifact and the DONE rename; whatever \
                        the grow verbs wrote can land in a later commit once the cascade has \
                        settled.",
        }],
    },
    Invariant {
        rule: "signal-is-the-last-action",
        area: "retire -> commit -> complete",
        binds: Binds::EveryKindButFinish,
        situation: &[],
        claims: &[
            Claim {
                what: "`grove-llm complete` is the session's last action",
                groups: &[&["grove-llm complete"], &["last action"]],
                // `complete --done` is the *other* kind set's ending and contains
                // this one as a substring, so a paragraph teaching teardown must
                // not read as teaching the ordinary ending. Without this, a
                // `finish` path carrying step 3 of the teardown satisfies the
                // eighteen-kind rule.
                without: &["complete --done"],
                near_miss: "Once the task is retired and committed, run `grove-llm complete`: it \
                            writes the relaunch flag to the loop's signal file and returns.",
            },
            Claim {
                what: "ending without signalling stops the loop",
                groups: &[&["ending without signalling stops the loop"]],
                without: &[],
                near_miss: "Run `grove-llm complete` as your last action; the loop driver takes \
                            care of everything after that.",
            },
        ],
    },
    Invariant {
        rule: "review-budget",
        area: "the review budget",
        binds: Binds::EveryKind,
        // Canonical trigger 3: *when considering an in-session reviewer*.
        situation: &[&[
            "in-session review",
            "in-session doubt",
            "considering a review",
            "review ownership",
        ]],
        claims: &[
            Claim {
                what: "at most one in-session reviewer",
                groups: &[&["at most one"], &["review"]],
                without: &[],
                near_miss: "A picked plain producer may materialise fresh-context reviewers as \
                            the work needs; keep the number small.",
            },
            Claim {
                what: "the allowance is spent across the whole picked leaf",
                groups: &[&["whole picked leaf", "whole leaf", "leaf-wide"]],
                without: &[],
                near_miss: "A plain producer may materialise at most one reviewer per artifact, \
                            so a second artifact in the same session earns its own.",
            },
            Claim {
                // The adjoining `review-budget-predicate` row of the same
                // inventory group. It rides here rather than as a thirteenth
                // pinned rule because the twelve are the B* set exactly.
                what: "the allowance belongs to a mandated session that adopted the mandate by \
                       running Bootstrap",
                groups: &[
                    &["bootstrap"],
                    &[
                        "adopted the mandate",
                        "adopted that mandate",
                        "adopted the driver's mandate",
                    ],
                ],
                without: &[],
                // The corpus's own generic Bootstrap paragraph, which carries
                // both of the two common words the superseded groups asked for
                // and none of the predicate.
                near_miss: "Bootstrap. A session's mandate is assembled by reading — the \
                            glossary, the cited ADRs, the BRIEF.md chain root to leaf, the task \
                            file — and that is the whole of it.",
            },
            Claim {
                // The starred row's second half, which the suite previously
                // left uncovered: a rewrite could keep the cap and delete what
                // to do once it is spent.
                what: "a second review need becomes a `review-*` leaf rather than another \
                       in-session reviewer",
                groups: &[
                    &[
                        "a second need",
                        "a second review",
                        "beyond it",
                        "once spent",
                    ],
                    &[
                        "leaf rather than another reviewer",
                        "a `review-*` leaf",
                        "a review-<producer> leaf",
                        "tree-sized",
                    ],
                ],
                without: &[],
                near_miss: "The allowance is leaf-wide rather than per artifact, so a second \
                            review need in the same leaf simply has to wait its turn.",
            },
        ],
    },
    Invariant {
        rule: "finish-three-endings",
        area: "all three finish-signal outcomes",
        binds: Binds::OnlyFinish,
        situation: &[],
        claims: &[
            Claim {
                what: "teardown ends with `complete --done` and the loop stops",
                groups: &[&["teardown"], &["complete --done"], &["the loop stops"]],
                without: &[],
                near_miss: "When teardown completed, run `grove-llm complete --done`; the loop \
                            driver takes it from there.",
            },
            Claim {
                what: "externalised work ends with a bare `complete` and the loop relaunches",
                groups: &[&["externalised work", "externalized work"], &["relaunch"]],
                without: &[],
                near_miss: "If you externalised work instead of tearing down, signal in the \
                            ordinary way and let the driver decide what happens next.",
            },
            Claim {
                what: "declining sends no signal and leaves the leaf live",
                groups: &[&["no signal"], &["stays live", "resumable"]],
                without: &[],
                near_miss: "If you decline, or find no human to ask, send no signal and stop.",
            },
        ],
    },
    Invariant {
        // The ninth area, and the one rule that could not be written against the
        // corpus as it stood: `references/requirements.md` carried the
        // always-form and the three-question trigger as two unreconciled
        // statements, and `references/execute.md` carried the always-form alone,
        // so a session reading either in isolation got a different rule. What is
        // pinned here is the *resolution* — the deliverable is unconditional, the
        // procedure is not — because a check for the threshold alone would pass a
        // corpus that had resolved the contradiction the other way, by deleting
        // the always-form and making `requirements` an interview kind.
        rule: "grilling-threshold",
        area: "the interview threshold",
        // `static({requirements})` — so no `situation`, and the delivery check
        // holds the rule to that kind's own reference file.
        binds: Binds::OnlyRequirements,
        situation: &[],
        claims: &[
            Claim {
                what: "the full procedure runs only at three or more interdependent open \
                       questions",
                groups: &[
                    &["grilling"],
                    &[
                        "only when three or more",
                        "only at three or more",
                        "only above three",
                    ],
                    &["interdepend"],
                ],
                without: &[],
                // The always-form bullet the file carried twice. It is the whole
                // reason this claim exists, so it is the fixture: on topic, and
                // silent about when the interview is owed.
                near_miss: "This is where the grilling lives (`grilling.md`): interview one \
                            question at a time, propose a recommended answer for each, and walk \
                            the design tree until shared understanding is reached.",
            },
            Claim {
                // Half a threshold is not a threshold. A rule that says when the
                // interview runs and not what happens below it leaves the session
                // to invent the other branch, which is what a session reading
                // "three or more" as advice already does.
                what: "below the threshold the session records the decisions and proceeds",
                groups: &[
                    &["below that threshold", "below it", "below that"],
                    &["record the decisions", "records the decisions"],
                ],
                without: &[],
                near_miss: "The full one-question-at-a-time procedure runs only when three or \
                            more of this leaf's open questions have interdependent answers.",
            },
            Claim {
                // The other half of the contradiction, and the half a threshold
                // could quietly eat: `requirements` **always** establishes what
                // is wanted, whatever the question count does to the interview.
                what: "establishing what is wanted is unconditional while the interview is not",
                groups: &[&["unconditional"], &["interview"]],
                without: &[],
                near_miss: "**requirements** (HITL) — establish *what* should be built, in the \
                            human's own words. This is where the grilling lives: interview one \
                            question at a time.",
            },
        ],
    },
];

// -- The claim ---------------------------------------------------------------

/// **Every starred invariant is delivered on the loaded path of every kind it
/// binds.**
///
/// The whole point of driving this per kind rather than over the corpus is that a
/// rule stated only in a file that kind's path never reaches fails here — and,
/// since [`delivery_site`], so does a rule whose own condition is gone while its
/// owner file stays reachable through a sibling one.
///
/// **One complaint per unmet claim, not per kind.** A rule dropped from a file
/// every kind loads fails for all nineteen at once, and nineteen copies of one
/// sentence — each printing a seventeen-file path — buries the one fact a reader
/// needs. So the kinds are gathered onto the complaint, and it also carries where
/// the rule's *other* claims are delivered from: the first question on a red run
/// is always whether the rule moved, lost its trigger, or went.
#[test]
fn every_lifecycle_invariant_is_on_the_loaded_path_of_every_kind_it_binds() {
    let corpus = corpus();
    let mut complaints: Vec<String> = Vec::new();

    for invariant in INVARIANTS {
        for claim in invariant.claims {
            let mut failing: Vec<&str> = Vec::new();
            let mut elsewhere: Vec<String> = Vec::new();
            for kind in invariant.binds.kinds() {
                let path = loaded_path(&corpus, kind);
                if delivery_site(&path, invariant, claim).is_none() {
                    if failing.is_empty() {
                        elsewhere = invariant
                            .claims
                            .iter()
                            .filter(|other| !std::ptr::eq(*other, claim))
                            .map(|other| {
                                format!(
                                    "{:?} delivered at {:?}, stated at {:?}",
                                    other.what,
                                    delivery_site(&path, invariant, other),
                                    claim_sites(&path, other)
                                )
                            })
                            .collect();
                        // Where this claim is *stated* even though nothing
                        // delivers it: the difference between "the rule went"
                        // and "its condition went".
                        elsewhere.push(format!(
                            "this claim is stated at {:?}",
                            claim_sites(&path, claim)
                        ));
                    }
                    failing.push(kind.label());
                }
            }
            if !failing.is_empty() {
                complaints.push(format!(
                    "{} ({}): nothing on the loaded path of {:?} delivers {:?}. Context: \
                     {elsewhere:?}",
                    invariant.rule, invariant.area, failing, claim.what,
                ));
            }
        }
    }

    assert_eq!(
        complaints,
        Vec::<String>::new(),
        "a lifecycle invariant this refactor must not change is no longer delivered to a \
         session of a kind it binds. Either the rule was dropped, or it was rehomed into a \
         file nothing on that kind's path names, or its own condition went while its owner \
         file stayed reachable through another one — all three are deleted in effect while \
         sitting in content/. {} complaint(s).",
        complaints.len()
    );
}

/// **The coverage table is the claim, so losing a row must fail by name.**
///
/// A word-count or a row count could only report that the file got smaller. This
/// pins the nine required areas and the thirteen inventory rules inside them, so
/// a deleted invariant fails saying which one, and an area with no invariant
/// fails saying which area.
#[test]
fn the_table_covers_the_nine_required_areas_and_no_others() {
    let covered: BTreeSet<&str> = INVARIANTS.iter().map(|invariant| invariant.area).collect();
    assert_eq!(
        covered,
        BTreeSet::from(AREAS),
        "these are the nine areas docs/specs/corpus-rule-ownership.md requires behavioural \
         coverage for: the eight B* areas behavior-evals-k3 wrote against the corpus as it \
         stood, and the interview threshold, which kind-references-k5 added with the fix that \
         made it green"
    );

    let rules: Vec<&str> = INVARIANTS.iter().map(|invariant| invariant.rule).collect();
    assert_eq!(
        rules.iter().collect::<BTreeSet<_>>().len(),
        rules.len(),
        "each inventory rule gets one row: {rules:?}"
    );
    assert_eq!(
        rules.iter().copied().collect::<BTreeSet<&str>>(),
        BTreeSet::from([
            "bigger-than-brief-decomposes",
            "externalize-by-default",
            "finish-three-endings",
            "grilling-threshold",
            "mandate-is-authoritative",
            "no-second-pick",
            "one-focused-commit",
            "pruning-is-hitl",
            "retire-before-commit",
            "review-budget",
            "signal-is-the-last-action",
            "stated-vcs-is-definitive",
            "stale-launch-stops",
        ]),
        "the thirteen inventory rows inside the nine required areas — twelve B* and \
         grilling-threshold. A row leaving here is coverage leaving the suite, so it fails by \
         name rather than by count."
    );
}

// -- The controls on the matcher ----------------------------------------------

/// **The control on every claim: a plausible rewrite that keeps the rule's
/// subject and drops *that claim's* force must fail it.**
///
/// This is what separates these checks from a topic match, and it is **per
/// claim** rather than per invariant. An invariant-wide fixture proves only that
/// *some* claim binds: the others are then free to be two common words, or to
/// accept the opposite rule, because a sibling claim carries the failure.
///
/// Each fixture is prose a rewriting session could genuinely produce — the
/// vocabulary of the rule without the clause that binds. It is asserted to be *on
/// topic* for the rule (it meets at least one group of some claim, so the fixture
/// is about this rule rather than about nothing) and *insufficient* for its own
/// claim. On-topic is measured against the rule rather than the claim because a
/// claim whose group is one distinctive phrase can have no fixture that is both
/// on topic for it and failing it.
#[test]
fn a_rewrite_that_keeps_the_subject_and_drops_the_force_fails_the_claim_it_controls() {
    for invariant in INVARIANTS {
        for claim in invariant.claims {
            let fixture = paragraphs(claim.near_miss);
            assert!(
                !fixture
                    .iter()
                    .any(|paragraph| paragraph_meets(paragraph, claim)),
                "{}'s near-miss fixture for {:?} states the claim, so it is not a near miss and \
                 the check it controls is unproven:\n{}",
                invariant.rule,
                claim.what,
                claim.near_miss
            );

            let on_topic = invariant.claims.iter().any(|other| {
                other.groups.iter().any(|group| {
                    fixture.iter().any(|paragraph| {
                        group
                            .iter()
                            .any(|wording| paragraph.contains(&normalise(wording)))
                    })
                })
            });
            assert!(
                on_topic,
                "{}'s near-miss fixture for {:?} shares no wording with the rule at all, so it \
                 shows only that the matcher rejects unrelated prose:\n{}",
                invariant.rule, claim.what, claim.near_miss
            );
        }
    }
}

/// **A `without` wording disqualifies a paragraph that meets every group.**
///
/// The shape it exists for is real and asymmetric: `complete --done` contains
/// `grove-llm complete`, so the teardown instruction would otherwise read as the
/// ordinary ending and the two endings' scopes would silently overlap.
#[test]
fn a_disqualifying_wording_rejects_a_paragraph_that_meets_every_group() {
    let claim = Claim {
        what: "the ordinary ending",
        groups: &[&["grove-llm complete"], &["last action"]],
        without: &["complete --done"],
        near_miss: "unused here",
    };

    let ordinary = vec![(
        "ORDINARY.md".to_owned(),
        "Run `grove-llm complete` as your last action.\n".to_owned(),
    )];
    assert_eq!(claim_sites(&ordinary, &claim), vec!["ORDINARY.md"]);

    let teardown = vec![(
        "TEARDOWN.md".to_owned(),
        "Run `grove-llm complete --done` as the very last action.\n".to_owned(),
    )];
    assert!(
        claim_sites(&teardown, &claim).is_empty(),
        "a paragraph teaching the teardown ending must not read as teaching the ordinary one"
    );
}

/// **The rewordings the rewrite is licensed to make pass, and the incomplete
/// forms beside them do not.**
///
/// The near-miss fixtures show the matcher rejecting prose that drops a rule's
/// force. This is the other direction, and it is the one that blocks a leaf rather
/// than losing a rule: eight rewrites are *licensed to reword*, so a group with no
/// alternative for the canonical wording turns an intended edit red and trains a
/// reader to re-point the test instead of reading it.
///
/// The two pinned here are the ones whose canonical wording differs from the
/// corpus's current wording: the inventory and canonical trigger both abstract
/// `DONE`/`ABANDONED` to *a terminal leaf*, and the commit row states its contract
/// as the four kinds of change landing *together* rather than as the phrase "one
/// focused commit".
#[test]
fn the_canonical_rewordings_pass_without_admitting_the_incomplete_forms() {
    let stale = &INVARIANTS
        .iter()
        .find(|invariant| invariant.rule == "stale-launch-stops")
        .expect("the table carries stale-launch-stops")
        .claims[0];
    let terminal = vec![(
        "REWRITTEN.md".to_owned(),
        "A mandated handle that resolves to nothing, or to a terminal leaf, is a stale launch \
         and not work to redo.\n"
            .to_owned(),
    )];
    assert_eq!(
        claim_sites(&terminal, stale),
        vec!["REWRITTEN.md"],
        "the abstraction the inventory and trigger 2 both use must not go red"
    );

    let commit = &INVARIANTS
        .iter()
        .find(|invariant| invariant.rule == "one-focused-commit")
        .expect("the table carries one-focused-commit")
        .claims[0];
    let together = vec![(
        "REWRITTEN.md".to_owned(),
        "One task's commit carries the artifact, the grow-verb writes, the `DONE` rename and \
         whatever the cascade promoted, together, named by handle.\n"
            .to_owned(),
    )];
    assert_eq!(
        claim_sites(&together, commit),
        vec!["REWRITTEN.md"],
        "the canonical commit row's own wording must not go red"
    );

    // And the licence does not extend to splitting the commit: the same four
    // classes of change, named in the same sentence, with one of them deferred.
    let split = vec![(
        "SPLIT.md".to_owned(),
        "One task's commit carries the artifact and the `DONE` rename together; whatever the \
         grow verbs wrote can land in a later commit.\n"
            .to_owned(),
    )];
    assert!(
        claim_sites(&split, commit).is_empty(),
        "a commit that defers one of the four classes is not this rule, however many of them \
         the sentence names"
    );
}

/// **A conjunction assembled out of two paragraphs is not a statement of the
/// rule.** Two halves of a rule in two places is exactly the drift this corpus
/// refactor exists to remove, so a claim's groups have to land together.
#[test]
fn groups_split_across_paragraphs_do_not_meet_a_claim() {
    let claim = Claim {
        what: "a two-group claim",
        groups: &[&["first half"], &["second half"]],
        without: &[],
        near_miss: "unused here",
    };

    let together = vec![(
        "TOGETHER.md".to_owned(),
        "the first half and the second half\n".to_owned(),
    )];
    assert_eq!(claim_sites(&together, &claim), vec!["TOGETHER.md"]);

    let apart = vec![(
        "APART.md".to_owned(),
        "the first half\n\nthe second half\n".to_owned(),
    )];
    assert!(
        claim_sites(&apart, &claim).is_empty(),
        "two paragraphs each carrying one group must not add up to the claim"
    );
}

/// **Normalisation earns its place on the shapes the corpus really uses.**
///
/// All three were live holes rather than hypotheticals: the corpus writes *Retire
/// **before** you commit* with the emphasis inside the phrase, wraps that phrase
/// across a line break, and writes the over-budget outcome as `` `review-*` leaf
/// `` with two markers inside one noun phrase. An unnormalised matcher misses all
/// three silently, which is the fail-open direction.
#[test]
fn emphasis_code_markers_and_line_wrapping_are_normalised_away() {
    let claim = Claim {
        what: "retirement precedes the commit",
        groups: &[&["retire before you commit"]],
        without: &[],
        near_miss: "unused here",
    };
    let wrapped = vec![(
        "WRAPPED.md".to_owned(),
        "Retire\n*before* you commit, so the rename lands in it.\n".to_owned(),
    )];
    assert_eq!(claim_sites(&wrapped, &claim), vec!["WRAPPED.md"]);

    let absent = vec![("ABSENT.md".to_owned(), "Retire, then commit.\n".to_owned())];
    assert!(
        claim_sites(&absent, &claim).is_empty(),
        "the normaliser must not make unrelated prose match"
    );

    // A group member written the way the corpus writes it, matched against text
    // carrying the same markers — the needle is normalised too, so neither side
    // has to spell the other's punctuation.
    let backticked = Claim {
        what: "the over-budget outcome",
        groups: &[&["a `review-*` leaf"]],
        without: &[],
        near_miss: "unused here",
    };
    let corpus_shape = vec![(
        "SHAPE.md".to_owned(),
        "beyond it the answer is a `review-*` leaf rather than another reviewer\n".to_owned(),
    )];
    assert_eq!(claim_sites(&corpus_shape, &backticked), vec!["SHAPE.md"]);
}

// -- The controls on the reachability walk ------------------------------------

/// A phrase that exists in exactly one corpus file, and in a file that is on no
/// kind's *static* path — so reaching it proves the walk followed an edge.
const DOWNSTREAM_ONLY: &str = "Only on explicit human confirmation, run";

fn probe() -> Claim {
    Claim {
        what: "a phrase stated only in references/retire.md",
        groups: &[&[DOWNSTREAM_ONLY]],
        without: &[],
        near_miss: "unused here",
    }
}

/// **The walk reaches a file because a sentence names it, and stops when that
/// sentence goes.**
///
/// `references/retire.md` is on no kind's static path: it is reached because
/// `SKILL.md` names it. Both halves are asserted, because reaching it proves
/// nothing on its own — a matcher that simply scanned the whole corpus would
/// reach it too, and would go on reaching it after the naming sentence was gone.
#[test]
fn the_walk_follows_a_named_path_and_stops_when_the_name_goes() {
    let corpus = corpus();
    let real = loaded_path(&corpus, Kind::Impl);
    assert_eq!(
        claim_sites(&real, &probe()),
        vec!["references/retire.md"],
        "content/references/retire.md must be on an `impl` session's loaded path, reached \
         through the sentence in SKILL.md that names it"
    );

    let mut unnamed = corpus.clone();
    let skill = unnamed
        .get_mut("SKILL.md")
        .expect("the corpus carries SKILL.md");
    *skill = skill.replace("references/retire.md", "the Retire step's reference file");
    let severed = loaded_path(&unnamed, Kind::Impl);
    assert!(
        !severed
            .iter()
            .any(|(name, _)| name == "references/retire.md"),
        "with nothing naming it by path, content/references/retire.md must drop off the \
         loaded path — a rule reachable only through prose that stopped naming its file is \
         deleted in effect"
    );
    assert!(
        claim_sites(&severed, &probe()).is_empty(),
        "and the rule it carried must read as absent. If this passes, the walk is scanning \
         the corpus rather than following edges, and every per-kind claim in this file is a \
         claim about content/ instead."
    );
}

/// **A rule surviving only in a file nothing names is not on any loaded path** —
/// present in `content/`, deleted in effect. This is the failure `driving.md`
/// already commits against two `impl` disciplines, and the one a
/// whole-corpus sweep certifies as fine.
#[test]
fn a_rule_only_in_an_unnamed_file_is_off_every_loaded_path() {
    let mut relocated = corpus();
    let retire = relocated
        .get_mut("references/retire.md")
        .expect("the corpus carries references/retire.md");
    *retire = retire.replace(DOWNSTREAM_ONLY, "Run");
    relocated.insert(
        "references/orphan.md".to_owned(),
        format!("{DOWNSTREAM_ONLY} `grove-llm leaf-prune <path>`.\n"),
    );

    let path = loaded_path(&relocated, Kind::Impl);
    assert!(
        relocated.contains_key("references/orphan.md"),
        "the rule is in the corpus — that is the whole point of the case"
    );
    assert!(
        !path.iter().any(|(name, _)| name == "references/orphan.md"),
        "a corpus file no file on the path names must not join the path"
    );
    assert!(
        claim_sites(&path, &probe()).is_empty(),
        "a rule embedded in content/ but unreachable from any static path must read as \
         absent, or this file's checks would pass a refactor that filed every rule where no \
         session opens it"
    );
}

/// **A rule is delivered by its own condition, not by a sibling condition that
/// happens to open the same file.**
///
/// This is the failure the file-grained walk cannot see on its own. Two canonical
/// triggers name `references/retire.md` — *when the work is done, retire* and
/// *when this leaf's path looks decided against, stop and ask*. Delete the second
/// and the walk still lands every session in `retire.md`, whose pruning prose is
/// untouched; a session whose path looks decided against nonetheless has nothing
/// telling it to look. So the mutant keeps the owner file, its prose, and one
/// edge to it, and removes only the condition — and `pruning-is-hitl` must go red
/// even though its wording is still sitting on the path.
#[test]
fn a_rule_whose_own_condition_went_is_not_delivered_by_a_sibling_edge() {
    const PRUNING_CONDITION: &str = "- When this leaf's path looks decided against, stop and ask, \
                                     as\n  `references/retire.md` directs.\n";

    let mut mutant = corpus();
    let skill = mutant
        .get_mut("SKILL.md")
        .expect("the corpus carries SKILL.md");
    assert!(
        skill.contains(PRUNING_CONDITION),
        "content/SKILL.md no longer carries the pruning condition this control excises \
         verbatim. Re-point it at the current wording; excising nothing would make the \
         control pass for the wrong reason."
    );
    *skill = skill.replace(PRUNING_CONDITION, "");

    let pruning = INVARIANTS
        .iter()
        .find(|invariant| invariant.rule == "pruning-is-hitl")
        .expect("the table carries pruning-is-hitl");
    let asks = &pruning.claims[1];
    let path = loaded_path(&mutant, Kind::Impl);

    // The owner file is still there, still reached, and still says it. Nothing
    // about reachability has changed — which is what makes the next assertion a
    // statement about triggers rather than about the walk.
    assert!(
        claim_sites(&path, asks).contains(&"references/retire.md".to_owned()),
        "the mutant must leave content/references/retire.md on the path and its pruning prose \
         intact, or this control is testing unreachability again"
    );

    assert!(
        unmet(&path, pruning).contains(&asks.what),
        "with its own condition deleted, pruning-is-hitl must not count as delivered — the \
         procedure is present and no session meeting the situation is told to open it. If \
         this passes, delivery is following any mention of the owner file and a rewrite may \
         silently drop a trigger sentence."
    );
    assert_eq!(
        unmet(&path, pruning).len(),
        pruning.claims.len(),
        "and every claim of the rule goes with the condition, not just the one this control \
         reads first"
    );
}

/// **The routing table is a selector, not an edge.**
///
/// It names all ten reference files so a session that arrived without a mandate
/// can find its own. Reading those as edges would put every family's discipline
/// on every kind's path — and then a rule stated only in `references/design.md`
/// would count as delivered to an `impl` session, which is exactly the per-kind
/// claim this file makes.
#[test]
fn the_routing_table_is_a_selector_rather_than_an_edge() {
    let corpus = corpus();
    let path: Vec<String> = loaded_path(&corpus, Kind::Impl)
        .into_iter()
        .map(|(name, _)| name)
        .collect();

    assert!(
        path.contains(&"references/impl.md".to_owned()),
        "the mandated kind's own reference file is static and must be there"
    );
    for other in [
        "references/design.md",
        "references/review.md",
        "references/finish.md",
    ] {
        assert!(
            !path.contains(&other.to_owned()),
            "content/{other} reached an `impl` session's loaded path. If the routing table is \
             being read as ten edges, every per-kind claim in this file has quietly become a \
             claim about the whole corpus: {path:?}"
        );
    }

    // The control on the excision itself. Both sides read the same bytes; only
    // the region scanned differs, so what the difference isolates is the table.
    let skill = &corpus["SKILL.md"];
    let with_the_table = named_files("(unexcised)", skill, &corpus);
    let without_it = named_files("SKILL.md", skill, &corpus);
    let dropped: Vec<&&String> = with_the_table
        .iter()
        .filter(|target| !without_it.contains(target))
        .collect();
    assert!(
        dropped.len() >= 9,
        "read whole, SKILL.md names every reference file its routing table prints — which is \
         what makes the excision load-bearing rather than decorative. Excising it dropped \
         only {dropped:?}"
    );
    assert!(
        !without_it.is_empty(),
        "and the excision must not swallow the rest of the page's own file mentions"
    );
}

/// **The two per-kind endings do not leak into each other's paths — not even
/// partly.**
///
/// `content/SIGNAL.md` is named by no corpus file at all: it reaches a session
/// only because the core inlines its bytes for the eighteen kinds that end one
/// way. `content/SIGNAL-FINISH.md` reaches `finish` the same way. So the two
/// rules genuinely bind different kind sets, and asserting the absence is what
/// makes the positive claim about delivery rather than about the corpus.
///
/// **Every claim of the foreign rule must be absent, not merely one of them.**
/// Either partial leak is already the dangerous wrong action: a `finish` path
/// that acquires *run bare `grove-llm complete` as your last action* relaunches
/// the loop onto a torn-down grove, and an ordinary path that acquires
/// `complete --done` stops the loop with work still live. Asserting only that
/// something is missing would pass both.
#[test]
fn each_endings_rule_is_absent_from_the_other_kinds_path() {
    let corpus = corpus();

    let three_endings = INVARIANTS
        .iter()
        .find(|invariant| invariant.rule == "finish-three-endings")
        .expect("the table carries finish-three-endings");
    let last_action = INVARIANTS
        .iter()
        .find(|invariant| invariant.rule == "signal-is-the-last-action")
        .expect("the table carries signal-is-the-last-action");

    for kind in Kind::ALL {
        let path = loaded_path(&corpus, kind);
        let invariant = if kind == Kind::Finish {
            last_action
        } else {
            three_endings
        };
        let absent = unmet(&path, invariant);
        assert_eq!(
            absent.len(),
            invariant.claims.len(),
            "`{}`'s loaded path delivers part of {}, which binds the other kind set: {:?} \
             reached it. Either a signal file's scope moved or one of its instructions became \
             reachable as prose — and a `{}` session told any part of the wrong ending is the \
             failure the split exists to prevent.",
            kind.label(),
            invariant.rule,
            invariant
                .claims
                .iter()
                .map(|claim| claim.what)
                .filter(|what| !absent.contains(what))
                .collect::<Vec<_>>(),
            kind.label()
        );
    }
}

// -- The assumption the walk rests on ----------------------------------------

/// **No corpus path hides inside another**, which is what makes a literal
/// occurrence a sound test for *this file names that one*. `SIGNAL.md` inside
/// `SIGNAL-FINISH.md` is the near miss the naming scheme already avoids; a future
/// `references/x.md` beside a root `x.md` would not.
#[test]
fn no_corpus_path_hides_inside_another() {
    let corpus = corpus();
    let overlapping: Vec<String> = corpus
        .keys()
        .flat_map(|outer| {
            corpus
                .keys()
                .filter(move |inner| *inner != outer && outer.contains(inner.as_str()))
                .map(move |inner| format!("{inner} inside {outer}"))
        })
        .collect();
    assert_eq!(
        overlapping,
        Vec::<String>::new(),
        "one corpus path is a substring of another, so `text.contains(path)` can no longer \
         tell which file a sentence names. Teach `named_files` to match a delimited path \
         before adding such a name."
    );
    assert!(
        corpus.len() > 20,
        "the sweep above is only worth something over the real corpus"
    );
}

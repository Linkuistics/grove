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
//! **Every claim carries a control.** The matcher is shown rejecting a plausible
//! rewrite that keeps a rule's subject and drops its force; the reachability walk
//! is shown following a named path, stopping when the naming sentence goes, and
//! refusing to reach a file nothing names.
//!
//! Scope is the eight **B★** areas of `docs/specs/corpus-rule-ownership.md`.
//! The ninth required area — the requirements interview threshold — is a
//! contradiction being resolved rather than a behaviour being preserved, so its
//! test is red until `kind-references-k5` lands the fix and belongs to that leaf.

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
/// The first three are the *static* half — the only three things
/// `src/prompt.rs` puts on a path unconditionally — and the closure is the
/// conditional half, reached because a sentence a session read named a file by
/// path.
fn loaded_path(corpus: &Corpus, kind: Kind) -> Vec<Loaded> {
    let mut path: Vec<Loaded> = vec![(CORE.to_owned(), core(kind))];
    for statik in ["SKILL.md", prompt::reference_file(kind)] {
        let text = corpus.get(statik).unwrap_or_else(|| {
            panic!("the corpus must carry the static path member content/{statik}")
        });
        path.push((statik.to_owned(), text.clone()));
    }

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
/// degenerating into a topic match is each invariant's near-miss fixture.
type Group = &'static [&'static str];

/// One statement a rule must make. Every group must occur in a **single
/// paragraph**, so a conjunction assembled out of two unrelated paragraphs does
/// not count as the rule being stated.
struct Claim {
    what: &'static str,
    groups: &'static [Group],
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
}

struct Invariant {
    /// The rule's id in `docs/specs/corpus-rule-ownership.md`'s inventory.
    rule: &'static str,
    /// The required area it serves, from the eight this leaf owns.
    area: &'static str,
    binds: Binds,
    claims: &'static [Claim],
    /// A plausible rewrite that keeps the rule's subject and drops its force.
    /// Asserted to fail, which is what shows the check is sensitive to the
    /// load-bearing clause rather than to the topic.
    near_miss: &'static str,
}

impl Binds {
    fn kinds(&self) -> Vec<Kind> {
        Kind::ALL
            .into_iter()
            .filter(|kind| match self {
                Binds::EveryKind => true,
                Binds::EveryKindButFinish => *kind != Kind::Finish,
                Binds::OnlyFinish => *kind == Kind::Finish,
            })
            .collect()
    }
}

/// A file's paragraphs, normalised: emphasis markers dropped and whitespace
/// collapsed, so a rule wrapped across lines or carrying `**bold**` reads the
/// same as one written flat. Matching is case-insensitive.
///
/// Underscores are deliberately **not** stripped — `GROVE_SIGNAL_FILE` is a name
/// a claim may need to match, and emphasis in this corpus is written with
/// asterisks.
fn paragraphs(text: &str) -> Vec<String> {
    text.split("\n\n")
        .map(|block| {
            block
                .replace('*', "")
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ")
                .to_lowercase()
        })
        .filter(|block| !block.is_empty())
        .collect()
}

fn paragraph_meets(paragraph: &str, claim: &Claim) -> bool {
    claim.groups.iter().all(|group| {
        group
            .iter()
            .any(|wording| paragraph.contains(&wording.to_lowercase()))
    })
}

/// The loaded-path members that state `claim`, in path order.
fn claim_sites(path: &[Loaded], claim: &Claim) -> Vec<String> {
    let mut sites = Vec::new();
    for (name, text) in path {
        if paragraphs(text)
            .iter()
            .any(|paragraph| paragraph_meets(paragraph, claim))
            && !sites.contains(name)
        {
            sites.push(name.clone());
        }
    }
    sites
}

/// The claims of `invariant` that nothing on `path` states.
fn unmet<'a>(path: &[Loaded], invariant: &'a Invariant) -> Vec<&'a str> {
    invariant
        .claims
        .iter()
        .filter(|claim| claim_sites(path, claim).is_empty())
        .map(|claim| claim.what)
        .collect()
}

// -- The eight areas, and the twelve rules inside them ------------------------

const AREAS: [&str; 8] = [
    "no second pick",
    "no VCS reprobe",
    "stale launch",
    "the decomposition boundary",
    "human-only pruning",
    "retire -> commit -> complete",
    "the review budget",
    "all three finish-signal outcomes",
];

const INVARIANTS: &[Invariant] = &[
    Invariant {
        rule: "mandate-is-authoritative",
        area: "no second pick",
        binds: Binds::EveryKind,
        claims: &[Claim {
            what: "the driver's single pre-session pick is the mandate, and nothing modulates it",
            groups: &[
                &["authoritative", "nothing modulates", "nothing modulating"],
                &["pick"],
                &["mandate", "selected leaf"],
            ],
        }],
        near_miss: "The driver makes a pick before the session exists and hands you the \
                    selected leaf as your mandate; a second walk normally agrees with it.",
    },
    Invariant {
        rule: "no-second-pick",
        area: "no second pick",
        binds: Binds::EveryKind,
        claims: &[Claim {
            what: "`grove-llm pick` is a diagnostic, and on disagreement the mandate wins",
            groups: &[
                &["grove-llm pick"],
                &["diagnostic"],
                &["the mandate wins", "not this session's dispatcher"],
            ],
        }],
        near_miss: "Run `grove-llm pick` to confirm the mandate you were given; if the two \
                    disagree, take the one that fits the tree.",
    },
    Invariant {
        rule: "stated-vcs-is-definitive",
        area: "no VCS reprobe",
        binds: Binds::EveryKind,
        claims: &[Claim {
            what: "the stated version control is definitive, is not re-derived, and beats a \
                   harness banner",
            groups: &[
                &["definitive", "statement wins"],
                &["re-derive", "probe"],
                &["harness banner"],
            ],
        }],
        near_miss: "The driver states this working tree's version control. A harness banner \
                    may disagree with it, so check the working tree when the two differ.",
    },
    Invariant {
        rule: "stale-launch-stops",
        area: "stale launch",
        binds: Binds::EveryKind,
        claims: &[Claim {
            what: "a handle resolving to nothing or to a terminal leaf is a stale launch, not \
                   work to redo",
            groups: &[
                &["resolves to nothing", "resolving to nothing"],
                &["abandoned"],
                &["not work to redo"],
            ],
        }],
        near_miss: "If the handle resolves to nothing, or to a DONE or ABANDONED leaf, the \
                    tree has moved on: redo the work against the leaf that is live now.",
    },
    Invariant {
        rule: "externalize-by-default",
        area: "the decomposition boundary",
        binds: Binds::EveryKind,
        claims: &[
            Claim {
                what: "work that does not serve this leaf's stated goal goes to the tree",
                groups: &[
                    &["does not serve", "not serve"],
                    &["stated goal", "leaf's goal"],
                    &["the tree", "externalis", "externaliz", "leaf-add"],
                ],
            },
            Claim {
                what: "the bar for continuing inline is \"fits this session\"",
                groups: &[&["fits this session"]],
            },
        ],
        near_miss: "Work that surfaces mid-session and does not serve this leaf's stated goal \
                    may be absorbed here when you already hold the context for it.",
    },
    Invariant {
        rule: "bigger-than-brief-decomposes",
        area: "the decomposition boundary",
        binds: Binds::EveryKind,
        claims: &[
            Claim {
                what: "a leaf that proves bigger than its brief becomes a node",
                groups: &[
                    &["proves bigger", "proved bigger", "bigger than its brief"],
                    &["node", "leaf-decompose"],
                ],
            },
            Claim {
                what: "the decomposing session does only the first child",
                groups: &[&["only the first child"]],
            },
        ],
        near_miss: "A leaf that proves bigger than its brief assumed becomes a node with \
                    `grove-llm leaf-decompose`; work through as many of its children as the \
                    session has room for.",
    },
    Invariant {
        rule: "pruning-is-hitl",
        area: "human-only pruning",
        binds: Binds::EveryKind,
        claims: &[
            Claim {
                what: "an agent never prunes on its own",
                groups: &[
                    &["prun"],
                    &["never prunes on its own", "an agent never prunes"],
                ],
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
            },
        ],
        near_miss: "Pruning marks a path decided against with an ABANDONED infix rather than \
                    deleting it: run `grove-llm leaf-prune <path>` once the path is closed.",
    },
    Invariant {
        rule: "retire-before-commit",
        area: "retire -> commit -> complete",
        binds: Binds::EveryKind,
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
            },
            Claim {
                what: "the rename lands inside that one commit",
                groups: &[
                    &["rename"],
                    &["land inside", "lands in it", "landing inside"],
                ],
            },
        ],
        near_miss: "Retire the just-finished leaf by running `grove-llm leaf-retire \
                    <leaf-path>`, which adds a DONE infix in place; commit the task whenever \
                    the work is ready.",
    },
    Invariant {
        rule: "one-focused-commit",
        area: "retire -> commit -> complete",
        binds: Binds::EveryKind,
        claims: &[Claim {
            what: "one task is one focused commit carrying the artifact, the grow-verb writes \
                   and the rename",
            groups: &[
                &["one focused commit"],
                &["rename"],
                &["artifact", "grow verb", "grow-verb"],
            ],
        }],
        near_miss: "Commit the artifact and whatever the grow verbs wrote; the DONE rename can \
                    ride along in a later commit once the cascade has settled.",
    },
    Invariant {
        rule: "signal-is-the-last-action",
        area: "retire -> commit -> complete",
        binds: Binds::EveryKindButFinish,
        claims: &[
            Claim {
                what: "`grove-llm complete` is the session's last action",
                groups: &[&["grove-llm complete"], &["last action"]],
            },
            Claim {
                what: "ending without signalling stops the loop",
                groups: &[&["ending without signalling stops the loop"]],
            },
        ],
        near_miss: "Once the task is retired and committed, run `grove-llm complete`: it \
                    writes the relaunch flag to the loop's signal file and returns.",
    },
    Invariant {
        rule: "review-budget",
        area: "the review budget",
        binds: Binds::EveryKind,
        claims: &[
            Claim {
                what: "at most one in-session reviewer",
                groups: &[&["at most one"], &["review"]],
            },
            Claim {
                what: "the allowance is spent across the whole picked leaf",
                groups: &[&["whole picked leaf", "whole leaf", "leaf-wide"]],
            },
            Claim {
                what: "the allowance belongs to a mandated session that adopted the mandate by \
                       running Bootstrap",
                groups: &[&["bootstrap"], &["mandate"]],
            },
        ],
        near_miss: "Review ownership inside a picked leaf applies once the driver mandated this \
                    session and Bootstrap adopted that mandate: a plain producer may \
                    materialise reviewers across the whole picked leaf as the work needs.",
    },
    Invariant {
        rule: "finish-three-endings",
        area: "all three finish-signal outcomes",
        binds: Binds::OnlyFinish,
        claims: &[
            Claim {
                what: "teardown ends with `complete --done` and the loop stops",
                groups: &[&["teardown"], &["complete --done"], &["the loop stops"]],
            },
            Claim {
                what: "externalised work ends with a bare `complete` and the loop relaunches",
                groups: &[&["externalised work", "externalized work"], &["relaunch"]],
            },
            Claim {
                what: "declining sends no signal and leaves the leaf live",
                groups: &[&["no signal"], &["stays live", "resumable"]],
            },
        ],
        near_miss: "How this session ends is decided by what it did: when teardown completed, \
                    run `grove-llm complete --done` and the loop stops.",
    },
];

// -- The claim ---------------------------------------------------------------

/// **Every starred invariant is on the loaded path of every kind it binds.**
///
/// The whole point of driving this per kind rather than over the corpus is that a
/// rule stated only in a file that kind's path never reaches fails here.
///
/// **One complaint per unmet claim, not per kind.** A rule dropped from a file
/// every kind loads fails for all nineteen at once, and nineteen copies of one
/// sentence — each printing a seventeen-file path — buries the one fact a reader
/// needs. So the kinds are gathered onto the complaint, and it also carries where
/// the rule's *other* claims still live: the first question on a red run is always
/// whether the rule moved or went.
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
                if claim_sites(&path, claim).is_empty() {
                    if failing.is_empty() {
                        elsewhere = invariant
                            .claims
                            .iter()
                            .filter(|other| !std::ptr::eq(*other, claim))
                            .map(|other| {
                                format!("{:?} at {:?}", other.what, claim_sites(&path, other))
                            })
                            .collect();
                    }
                    failing.push(kind.label());
                }
            }
            if !failing.is_empty() {
                complaints.push(format!(
                    "{} ({}): no paragraph on the loaded path of {:?} states {:?}. The rule's \
                     other claims are still at: {elsewhere:?}",
                    invariant.rule, invariant.area, failing, claim.what,
                ));
            }
        }
    }

    assert_eq!(
        complaints,
        Vec::<String>::new(),
        "a lifecycle invariant this refactor must not change is no longer reachable by a \
         session of a kind it binds. Either the rule was dropped, or it was rehomed into a \
         file nothing on that kind's path names — the second is deleted in effect while \
         sitting in content/. {} complaint(s).",
        complaints.len()
    );
}

/// **The coverage table is the claim, so losing a row must fail by name.**
///
/// A word-count or a row count could only report that the file got smaller. This
/// pins the eight required areas and the twelve inventory rules inside them, so a
/// deleted invariant fails saying which one, and an area with no invariant fails
/// saying which area.
#[test]
fn the_table_covers_the_eight_required_areas_and_no_others() {
    let covered: BTreeSet<&str> = INVARIANTS.iter().map(|invariant| invariant.area).collect();
    assert_eq!(
        covered,
        BTreeSet::from(AREAS),
        "the areas this leaf owns are exactly the eight B* areas of \
         docs/specs/corpus-rule-ownership.md; the ninth (the requirements interview \
         threshold) lands with kind-references-k5 because it cannot be green before that fix"
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
        "the twelve B* rows of the inventory's eight areas. A row leaving here is coverage \
         leaving the suite, so it fails by name rather than by count."
    );
}

// -- The controls on the matcher ----------------------------------------------

/// **The control on every invariant: a plausible rewrite that keeps the subject
/// and drops the force must fail.**
///
/// This is what separates these checks from a topic match. Each fixture is prose
/// a rewriting session could genuinely produce — the vocabulary of the rule
/// without the clause that binds — and is asserted to be *on topic* (it meets at
/// least one group) and *insufficient* (the invariant is unmet).
#[test]
fn a_rewrite_that_keeps_the_subject_and_drops_the_force_fails() {
    for invariant in INVARIANTS {
        let path = vec![("NEAR-MISS.md".to_owned(), invariant.near_miss.to_owned())];

        let unmet = unmet(&path, invariant);
        assert!(
            !unmet.is_empty(),
            "{}'s near-miss fixture satisfies the whole invariant, so it is not a near miss \
             and the check it is meant to control is unproven:\n{}",
            invariant.rule,
            invariant.near_miss
        );

        let on_topic = invariant.claims.iter().any(|claim| {
            claim.groups.iter().any(|group| {
                paragraphs(invariant.near_miss).iter().any(|paragraph| {
                    group
                        .iter()
                        .any(|wording| paragraph.contains(&wording.to_lowercase()))
                })
            })
        });
        assert!(
            on_topic,
            "{}'s near-miss fixture shares no wording with the rule at all, so it shows only \
             that the matcher rejects unrelated prose:\n{}",
            invariant.rule, invariant.near_miss
        );
    }
}

/// **A conjunction assembled out of two paragraphs is not a statement of the
/// rule.** Two halves of a rule in two places is exactly the drift this corpus
/// refactor exists to remove, so a claim's groups have to land together.
#[test]
fn groups_split_across_paragraphs_do_not_meet_a_claim() {
    let claim = Claim {
        what: "a two-group claim",
        groups: &[&["first half"], &["second half"]],
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
/// Both were live holes rather than hypotheticals: the corpus writes *Retire
/// **before** you commit* with the emphasis inside the phrase, and wraps that
/// phrase across a line break. An unnormalised matcher misses both silently,
/// which is the fail-open direction.
#[test]
fn emphasis_and_line_wrapping_are_normalised_away() {
    let claim = Claim {
        what: "retirement precedes the commit",
        groups: &[&["retire before you commit"]],
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
}

// -- The controls on the reachability walk ------------------------------------

/// A phrase that exists in exactly one corpus file, and in a file that is on no
/// kind's *static* path — so reaching it proves the walk followed an edge.
const DOWNSTREAM_ONLY: &str = "Only on explicit human confirmation, run";

fn probe() -> Claim {
    Claim {
        what: "a phrase stated only in references/retire.md",
        groups: &[&[DOWNSTREAM_ONLY]],
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

/// **The two per-kind endings do not leak into each other's paths.**
///
/// `content/SIGNAL.md` is named by no corpus file at all: it reaches a session
/// only because the core inlines its bytes for the eighteen kinds that end one
/// way. `content/SIGNAL-FINISH.md` reaches `finish` the same way. So the two
/// rules genuinely bind different kind sets, and asserting the absence is what
/// makes the positive claim about delivery rather than about the corpus.
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
        assert!(
            !unmet(&path, invariant).is_empty(),
            "`{}`'s loaded path states the whole of {}, which binds the other kind set. \
             Either a signal file's scope moved or one of them became reachable as prose — \
             and a `{}` session told the wrong last action is the failure the split exists \
             to prevent.",
            kind.label(),
            invariant.rule,
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

//! One case table for Grove's caller assets in both runtimes: the static
//! example, the sample, the review adapter, the two review examples and the
//! provider-evidence wrapper.
//!
//! Each case states the expected outcome on Ravel 1.1, which runs the
//! TypeScript assets, and on Ravel 2.0, which runs the Rhai assets. A run
//! checks the cases against the build that `GROVE_TEST_RAVEL` and
//! `GROVE_TEST_RAVEL_RUNTIME` select, through public `start-session` with fake
//! agents. A case whose outcomes differ names its reason beside it: an accepted
//! runtime difference of the delivery specification, or the directory route,
//! which only 2.0 has.
//!
//! The comparison covers the routing choice (provider, model, effort and
//! reason), the fake-harness argument vector, the delivered context, the
//! receipt bytes in `GROVE_PROVIDER_EVIDENCE`, and the refusal code, stage,
//! exit status and remedy. It covers the message of an owner refusal too,
//! except where a decoder or the system writes part of it. It excludes fields
//! that only one runtime reports, such as measured reads. Paths of the case
//! directory read as `<tmp>`, and the snapshot root of the configured skill
//! source reads as `<snapshot>`. A receipt's fresh attempt reads as
//! `<attempt>` once it is a UUID v4. Ravel computes each source digest, so a
//! source record's `sha256` reads as `<sha256>`, and its name and bytes stay.
//!
//! The review cases live in `ravel_asset_cases/review.rs`.
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

mod support;

#[path = "ravel_asset_cases/review.rs"]
mod review;

use support::ravel::{InstalledRavel, Runtime};

const PROMPT: &str = "literal prompt\nwith 'quotes', $variables and --flags";

/// The last argument when it is `PROMPT`, the caller's prompt unchanged.
const ORIGINAL: &str = "<prompt>";

/// The last argument when it is the prompt that Ravel prepared for skill
/// delivery: the notice's byte count, `PROMPT` inside, and the snapshot named.
const PREPARED: &str = "<prepared prompt>";

const ASSETS: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../assets/ravel-policy");

// ---------------------------------------------------------------------------
// The table.

/// The asset under test, and how an owner uses it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Asset {
    /// `examples/grove-static`, imported whole by its absolute path.
    Static,
    /// `examples/grove-static`, its table extended with two kinds of the owner.
    Extended,
    /// `sample/policy.ts` or `sample/config.rhai`, copied whole.
    Sample,
    /// `examples/grove-review`, used whole: the task-file adapter, Grove's
    /// review tables and the provider-evidence wrapper.
    GroveReview,
    /// `examples/review`, used whole, with caller facts in `--context`.
    Review,
    /// The review selector of `examples/review` with owner tables whose one
    /// route is a gateway of origin `openai`.
    Gateway,
    /// The provider-evidence wrapper around an owner whose selection a
    /// `Setup::Owner` states.
    Provenance,
}

/// What a case puts in its directory before the launch.
#[derive(Clone, Debug)]
enum Setup {
    /// `.ravel-choice` with these bytes.
    Choice(Vec<u8>),
    /// `.ravel-choice` as a directory.
    ChoiceDirectory,
    /// A secondary jj workspace: `.jj/repo` is a file with these bytes, and
    /// `main-repository/.jj/repo` is the store directory.
    Store(Vec<u8>),
    /// A primary jj workspace: `.jj/repo` is the store directory itself.
    PrimaryStore,
    /// A secondary workspace whose `.jj/repo` file cannot be opened.
    UnreadableStore,
    /// Owner settings that enable skill delivery from one local source.
    Skills,
    /// `task.md` with these bytes, passed as `--task-file`.
    Task(Vec<u8>),
    /// `context.json` with this text, passed as `--context`.
    Context(String),
    /// `--task-id` with this value.
    TaskId(String),
    /// The owner of `Asset::Provenance`: its selection as JSON, where an
    /// argument `<prompt>` stands for `request.prompt`, and its version.
    Owner {
        selection: serde_json::Value,
        version: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Code {
    /// An owner refusal: Ravel's `policy_refused` on 1.1 or `config_refused`
    /// on 2.0, with this owner code.
    Owner(&'static str),
    /// A refusal that Ravel owns, with this code.
    Ravel(&'static str),
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Outcome {
    Selected {
        argv: Vec<String>,
        provider: String,
        model: String,
        effort: String,
        reason: String,
        /// The delivered context without `measured`, or `None` when the
        /// notice reports no context.
        context: Option<serde_json::Value>,
        /// The receipt in `GROVE_PROVIDER_EVIDENCE`, or `None` when the agent
        /// received none.
        receipt: Option<String>,
    },
    Refused {
        code: Code,
        stage: String,
        exit: i32,
        remedy: String,
        /// The owner's message, or Ravel's whole message. `None` where a
        /// decoder or the system writes part of it.
        message: Option<String>,
    },
}

/// The differences between the runtimes that Grove's delivery specification
/// accepts, in "Runtime parity". Each reason is checked beside its case.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Accepted {
    ChoiceBound,
    Whitespace,
    StoreUtf8,
    StoreBound,
    Surrogate,
}

impl Accepted {
    const ALL: [Accepted; 5] = [
        Accepted::ChoiceBound,
        Accepted::Whitespace,
        Accepted::StoreUtf8,
        Accepted::StoreBound,
        Accepted::Surrogate,
    ];

    fn reason(self) -> &'static str {
        match self {
            Accepted::ChoiceBound => {
                "Rhai reads the choice file with a measured read of 4,096 bytes. A larger file \
                 ends the evaluation as source_too_large, and a catch cannot recover a breach \
                 of a read bound. The 1.1 helper refused it as choice_unreadable."
            }
            Accepted::Whitespace => {
                "Rhai and JavaScript split and trim by different whitespace sets: JavaScript \
                 counts U+FEFF and not U+0085, and Rhai counts U+0085 and not U+FEFF."
            }
            Accepted::StoreUtf8 => {
                "Rhai holds UTF-8 only, so a jj store path that is not UTF-8 refuses as \
                 jj_store_unreadable. The 1.1 sample decoded it with replacement characters."
            }
            Accepted::StoreBound => {
                "Rhai reads the jj store path through a measured read, so a file beyond \
                 Ravel's source bound ends the evaluation as source_too_large. The 1.1 sample \
                 read it without a bound."
            }
            Accepted::Surrogate => {
                "A JavaScript string can hold an unpaired surrogate, and a Rhai string cannot. \
                 The TypeScript review loader accepts a receipt that escapes one, and Ravel 1.1 \
                 then refuses the context that carries it as context_invalid. The Rhai loader \
                 refuses the receipt as creator_line_malformed. Neither launches a reviewer, and \
                 the owner's declaration of the provider is the remedy."
            }
        }
    }
}

/// The expected outcome for each runtime.
#[derive(Clone, Debug)]
enum Expect {
    /// The same outcome on both runtimes.
    Both(Outcome),
    /// An accepted runtime difference, with both outcomes.
    Differs {
        v1_1: Outcome,
        v2_0: Outcome,
        why: Accepted,
    },
    /// The directory route, which Ravel 1.1 does not have: its settings refuse
    /// the `skills` key.
    Directory { v1_1: Outcome, v2_0: Outcome },
}

impl Expect {
    fn for_runtime(&self, runtime: Runtime) -> &Outcome {
        match self {
            Expect::Both(outcome) => outcome,
            Expect::Differs { v1_1, v2_0, .. } | Expect::Directory { v1_1, v2_0 } => {
                runtime.pick(v1_1, v2_0)
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Case {
    name: String,
    asset: Asset,
    kind: &'static str,
    setup: Vec<Setup>,
    expect: Expect,
}

fn case(name: impl Into<String>, asset: Asset, kind: &'static str, expect: Expect) -> Case {
    Case {
        name: name.into(),
        asset,
        kind,
        setup: Vec::new(),
        expect,
    }
}

impl Case {
    fn with(mut self, setup: Setup) -> Self {
        self.setup.push(setup);
        self
    }
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_owned()).collect()
}

/// A static route: a wrapper with `--model`, `--effort` and the prompt.
fn wrapper(reviewer: bool, kind: &str, effort: &str, prompt: &str) -> Outcome {
    let (program, provider, model) = if reviewer {
        ("my-claude-wrapper", "anthropic", "your-claude-model")
    } else {
        ("my-codex-wrapper", "openai", "your-codex-model")
    };
    Outcome::Selected {
        argv: strings(&[program, "--model", model, "--effort", effort, prompt]),
        provider: provider.into(),
        model: model.into(),
        effort: effort.into(),
        reason: format!(
            "routes[{}] gives {program} with model {model} at effort {effort}",
            json(kind)
        ),
        context: None,
        receipt: None,
    }
}

/// The harness that a sample route selects.
#[derive(Clone, Copy)]
enum Agent {
    Codex,
    Claude,
    Notes,
}

/// What a sample route selects.
struct Route<'a> {
    /// The names in the choice file, or the default.
    names: &'a str,
    chosen: bool,
    kind: &'a str,
    agent: Agent,
    model: &'a str,
    effort: &'a str,
    /// The main repository of a secondary jj workspace.
    store: bool,
    /// The snapshot root of the skill source.
    snapshot: bool,
    prompt: &'a str,
}

impl<'a> Route<'a> {
    fn default(kind: &'a str, agent: Agent, model: &'a str, effort: &'a str) -> Self {
        Route {
            names: "claude-led codex-sol",
            chosen: false,
            kind,
            agent,
            model,
            effort,
            store: false,
            snapshot: false,
            prompt: ORIGINAL,
        }
    }

    fn chosen(
        names: &'a str,
        kind: &'a str,
        agent: Agent,
        model: &'a str,
        effort: &'a str,
    ) -> Self {
        Route {
            names,
            chosen: true,
            ..Route::default(kind, agent, model, effort)
        }
    }

    fn store(self) -> Self {
        Route {
            store: true,
            ..self
        }
    }

    fn delivered(self) -> Self {
        Route {
            snapshot: true,
            prompt: PREPARED,
            ..self
        }
    }

    fn outcome(&self) -> Outcome {
        let grant = "<tmp>/main-repository";
        let mut argv = Vec::new();
        let (program, provider) = match self.agent {
            Agent::Codex => {
                argv.extend(strings(&[
                    "codex",
                    "--no-alt-screen",
                    "--model",
                    self.model,
                    "-c",
                ]));
                argv.push(format!("model_reasoning_effort={}", self.effort));
                argv.extend(strings(&[
                    "-c",
                    "default_permissions=:danger-full-access",
                    "--ask-for-approval",
                    "never",
                ]));
                if self.store {
                    argv.extend(strings(&["--add-dir", grant]));
                }
                ("codex", "openai")
            }
            Agent::Claude => {
                argv.push("claude".into());
                if self.store {
                    argv.extend(strings(&["--add-dir", grant]));
                }
                if self.snapshot {
                    argv.extend(strings(&["--add-dir", "<snapshot>"]));
                }
                argv.extend(strings(&["--model", self.model, "--effort", self.effort]));
                ("claude", "anthropic")
            }
            Agent::Notes => {
                argv.extend(strings(&["codex-headless.sh", self.model, self.effort]));
                ("/bin/bash", "openai")
            }
        };
        argv.push(self.prompt.into());
        let from = if self.chosen {
            "chosen by .ravel-choice in <tmp>"
        } else {
            "the default"
        };
        Outcome::Selected {
            argv,
            provider: provider.into(),
            model: self.model.into(),
            effort: self.effort.into(),
            reason: format!(
                "{} ({from}) routes {} to {program} with model {} at effort {}",
                self.names,
                json(self.kind),
                self.model,
                self.effort
            ),
            context: None,
            receipt: None,
        }
    }
}

fn owner_refusal(
    code: &'static str,
    message: Option<String>,
    remedy: impl Into<String>,
) -> Outcome {
    Outcome::Refused {
        code: Code::Owner(code),
        stage: "selection".into(),
        exit: 3,
        remedy: remedy.into(),
        message,
    }
}

const OFFERED: &str =
    "codex-led, claude-led, codex-design-claude-impl, claude-design-codex-impl, codex-sol, high-effort";

fn choice_refusal(code: &'static str, message: Option<&str>) -> Outcome {
    owner_refusal(
        code,
        message.map(|message| format!("the choice file <tmp>/.ravel-choice {message}")),
        format!("write names from {OFFERED} in <tmp>/.ravel-choice, separated by whitespace, or delete it"),
    )
}

fn arrangement_refusal(count: usize) -> Outcome {
    owner_refusal(
        "choice_arrangement",
        Some(format!(
            ".ravel-choice in <tmp> names {count} arrangements, and a selection has exactly one"
        )),
        "name exactly one of codex-led, claude-led, codex-design-claude-impl, \
         claude-design-codex-impl there, with any of codex-sol, high-effort",
    )
}

fn store_refusal(message: Option<&str>) -> Outcome {
    owner_refusal(
        "jj_store_unreadable",
        message.map(|message| format!("<tmp>/.jj/repo names no jj store: {message}")),
        "run from a jj workspace whose <tmp>/.jj/repo names its store, or from a directory with no .jj",
    )
}

/// Ravel 2.0's end of an evaluation whose read exceeded its bound.
fn too_large() -> Outcome {
    Outcome::Refused {
        code: Code::Ravel("source_too_large"),
        stage: "context".into(),
        exit: 3,
        remedy: "read it with a larger max_bytes, up to the context budget of 262144 bytes, \
                 or read a smaller source; a read is never truncated"
            .into(),
        message: None,
    }
}

/// Ravel 1.1's refusal of a settings file that holds `skills`.
fn skills_unknown() -> Outcome {
    Outcome::Refused {
        code: Code::Ravel("settings_invalid"),
        stage: "cli".into(),
        exit: 2,
        remedy: "make it one JSON object holding any of timeoutMs, contextBytes and policyEnv \
                 (an array of variable names), or remove it to set nothing"
            .into(),
        message: None,
    }
}

/// Grove's session kinds, the lead or reviewer of the static table, and the
/// effort of each.
const STATIC_KINDS: [(&str, bool, &str); 23] = [
    ("requirements", false, "max"),
    ("review-requirements", true, "xhigh"),
    ("integrate-review-requirements", false, "xhigh"),
    ("design", false, "xhigh"),
    ("review-design", true, "xhigh"),
    ("integrate-review-design", false, "xhigh"),
    ("planning", false, "xhigh"),
    ("review-planning", true, "xhigh"),
    ("integrate-review-planning", false, "xhigh"),
    ("prototype", false, "medium"),
    ("review-prototype", true, "medium"),
    ("integrate-review-prototype", false, "medium"),
    ("impl", false, "high"),
    ("review-impl", true, "high"),
    ("integrate-review-impl", false, "medium"),
    ("research-a", false, "high"),
    ("research-b", true, "high"),
    ("combine-research", false, "xhigh"),
    ("draft", false, "high"),
    ("copy-edit", true, "medium"),
    ("art", false, "medium"),
    ("proof", true, "high"),
    ("finish", false, "medium"),
];

/// The default route of the sample for each session kind: Claude leads and
/// Codex, pinned to Sol, reviews.
const SAMPLE_DEFAULTS: [(&str, bool, &str); 23] = [
    ("requirements", false, "max"),
    ("review-requirements", true, "high"),
    ("integrate-review-requirements", false, "xhigh"),
    ("design", false, "xhigh"),
    ("review-design", true, "high"),
    ("integrate-review-design", false, "xhigh"),
    ("planning", false, "xhigh"),
    ("review-planning", true, "high"),
    ("integrate-review-planning", false, "xhigh"),
    ("prototype", false, "medium"),
    ("review-prototype", true, "medium"),
    ("integrate-review-prototype", false, "medium"),
    ("impl", false, "high"),
    ("review-impl", true, "high"),
    ("integrate-review-impl", false, "medium"),
    ("research-a", false, "high"),
    ("research-b", true, "high"),
    ("combine-research", false, "xhigh"),
    ("draft", false, "high"),
    ("copy-edit", true, "medium"),
    ("art", false, "high"),
    ("proof", true, "medium"),
    ("finish", false, "medium"),
];

/// The canonical receipt of one selection, as both wrappers write it, with
/// its attempt as `<attempt>`. The members keep their constant order.
fn receipt(provider: &str, version: &str, kind: &str, task: Option<&str>) -> String {
    let task = task
        .map(|task| format!(",\"taskId\":{}", json(task)))
        .unwrap_or_default();
    format!(
        "{{\"schemaVersion\":1,\"attemptId\":\"<attempt>\",\"provider\":{},\"policyVersion\":{},\"kind\":{}{task}}}",
        json(provider),
        json(version),
        json(kind)
    )
}

/// Every case of the table.
fn cases() -> Vec<Case> {
    let mut cases = route_cases();
    cases.extend(review::cases());
    cases
}

/// The cases of the static example and the sample. Each runtime wraps both in
/// its provider-evidence wrapper, so each selection carries the receipt of
/// the asset's version.
fn route_cases() -> Vec<Case> {
    let mut cases = static_and_sample_cases();
    for case in &mut cases {
        let version = match case.asset {
            Asset::Static => "grove-static 3",
            Asset::Sample => "grove sample 3",
            _ => continue,
        };
        let kind = case.kind;
        let stamp = |outcome: &mut Outcome| {
            if let Outcome::Selected {
                provider,
                receipt: slot,
                ..
            } = outcome
            {
                *slot = Some(receipt(provider, version, kind, None));
            }
        };
        match &mut case.expect {
            Expect::Both(outcome) => stamp(outcome),
            Expect::Differs { v1_1, v2_0, .. } | Expect::Directory { v1_1, v2_0 } => {
                stamp(v1_1);
                stamp(v2_0);
            }
        }
    }
    cases
}

fn static_and_sample_cases() -> Vec<Case> {
    let mut cases = Vec::new();

    // The static example routes every kind exactly and refuses another.
    for (kind, reviewer, effort) in STATIC_KINDS {
        cases.push(case(
            format!("static {kind}"),
            Asset::Static,
            kind,
            Expect::Both(wrapper(reviewer, kind, effort, ORIGINAL)),
        ));
    }
    let unlisted = || {
        owner_refusal(
            "incomplete_mapping",
            Some(r#"the routes name no command for kind "unmapped""#.into()),
            "add a route for this kind in your copy of the policy",
        )
    };
    cases.push(case(
        "static unlisted kind",
        Asset::Static,
        "unmapped",
        Expect::Both(unlisted()),
    ));
    cases.push(
        case(
            "static passes the delivered prompt",
            Asset::Static,
            "impl",
            Expect::Directory {
                v1_1: skills_unknown(),
                v2_0: wrapper(false, "impl", "high", PREPARED),
            },
        )
        .with(Setup::Skills),
    );

    // An owner imports the static table and its two harnesses by path.
    for (kind, reviewer) in [
        ("my-implementation", false),
        ("my-review", true),
        ("impl", false),
    ] {
        cases.push(case(
            format!("extended {kind}"),
            Asset::Extended,
            kind,
            Expect::Both(wrapper(reviewer, kind, "high", ORIGINAL)),
        ));
    }
    cases.push(case(
        "extended unlisted kind",
        Asset::Extended,
        "unmapped",
        Expect::Both(unlisted()),
    ));

    // The sample's default covers every kind, and the notes writer.
    for (kind, reviewer, effort) in SAMPLE_DEFAULTS {
        let route = if reviewer {
            Route::default(kind, Agent::Codex, "gpt-6.1-sol", effort)
        } else {
            Route::default(kind, Agent::Claude, "claude-opus-5-5", effort)
        };
        cases.push(case(
            format!("sample default {kind}"),
            Asset::Sample,
            kind,
            Expect::Both(route.outcome()),
        ));
    }
    cases.push(case(
        "sample default release-notes",
        Asset::Sample,
        "release-notes",
        Expect::Both(
            Route::default("release-notes", Agent::Notes, "gpt-6.1-sol", "medium").outcome(),
        ),
    ));

    // A choice file selects an arrangement and modifiers.
    for (choice, kind, agent, model, effort) in [
        ("codex-led", "impl", Agent::Codex, "gpt-6.1-sol", "high"),
        (
            "claude-led",
            "review-design",
            Agent::Codex,
            "gpt-6.1-sol",
            "xhigh",
        ),
        (
            "codex-design-claude-impl",
            "impl",
            Agent::Claude,
            "claude-sonnet-5-5",
            "high",
        ),
        (
            "claude-design-codex-impl",
            "review-prototype",
            Agent::Claude,
            "claude-sonnet-5-5",
            "medium",
        ),
        (
            "codex-led high-effort",
            "prototype",
            Agent::Codex,
            "gpt-6.1-sol",
            "high",
        ),
        (
            "claude-led codex-sol",
            "review-design",
            Agent::Codex,
            "gpt-6.1-sol",
            "high",
        ),
        (
            "codex-design-claude-impl high-effort",
            "copy-edit",
            Agent::Claude,
            "claude-sonnet-5-5",
            "medium",
        ),
        (
            "claude-design-codex-impl codex-sol",
            "proof",
            Agent::Codex,
            "gpt-6.1-sol",
            "high",
        ),
        (
            "high-effort claude-led",
            "release-notes",
            Agent::Notes,
            "gpt-6.1-sol",
            "medium",
        ),
    ] {
        cases.push(
            case(
                format!("sample chooses {choice:?} for {kind}"),
                Asset::Sample,
                kind,
                Expect::Both(Route::chosen(choice, kind, agent, model, effort).outcome()),
            )
            .with(Setup::Choice(choice.into())),
        );
    }
    cases.push(
        case(
            "sample choice with surrounding whitespace",
            Asset::Sample,
            "impl",
            Expect::Both(
                Route::chosen(
                    "codex-led high-effort",
                    "impl",
                    Agent::Codex,
                    "gpt-6.1-sol",
                    "high",
                )
                .outcome(),
            ),
        )
        .with(Setup::Choice(b"\n  codex-led\thigh-effort \n".to_vec())),
    );
    let mut bounded = b"codex-led".to_vec();
    bounded.resize(4096, b' ');
    cases.push(
        case(
            "sample choice of exactly 4,096 bytes",
            Asset::Sample,
            "impl",
            Expect::Both(
                Route::chosen("codex-led", "impl", Agent::Codex, "gpt-6.1-sol", "high").outcome(),
            ),
        )
        .with(Setup::Choice(bounded)),
    );

    // The sample's refusals.
    for (choice, outcome) in [
        (
            "unknown",
            choice_refusal(
                "choice_unoffered",
                Some(&format!(
                    r#"names "unknown", which this policy does not offer; it offers {OFFERED}"#
                )),
            ),
        ),
        ("", arrangement_refusal(0)),
        ("high-effort", arrangement_refusal(0)),
        ("codex-led claude-led", arrangement_refusal(2)),
    ] {
        cases.push(
            case(
                format!("sample refuses choice {choice:?}"),
                Asset::Sample,
                "impl",
                Expect::Both(outcome),
            )
            .with(Setup::Choice(choice.into())),
        );
    }
    let unmapped = || {
        owner_refusal(
            "incomplete_mapping",
            Some(r#"ROUTES names no route for kind "unmapped""#.into()),
            "add a route for this kind to ROUTES in your policy",
        )
    };
    cases.push(case(
        "sample unlisted kind",
        Asset::Sample,
        "unmapped",
        Expect::Both(unmapped()),
    ));
    cases.push(
        case(
            "sample checks the kind before the choice",
            Asset::Sample,
            "unmapped",
            Expect::Both(unmapped()),
        )
        .with(Setup::Choice(b"unknown".to_vec())),
    );
    cases.push(
        case(
            "sample choice that is a directory",
            Asset::Sample,
            "impl",
            Expect::Both(choice_refusal(
                "choice_unreadable",
                Some("is not a regular file"),
            )),
        )
        .with(Setup::ChoiceDirectory),
    );
    cases.push(
        case(
            "sample choice that is not UTF-8",
            Asset::Sample,
            "impl",
            Expect::Both(choice_refusal("choice_unreadable", None)),
        )
        .with(Setup::Choice(b"codex-led \xff".to_vec())),
    );

    // The accepted differences of the choice file.
    let mut oversized = b"codex-led".to_vec();
    oversized.resize(4097, b' ');
    cases.push(
        case(
            "sample choice beyond 4,096 bytes",
            Asset::Sample,
            "impl",
            Expect::Differs {
                v1_1: choice_refusal("choice_unreadable", Some("holds more than 4096 bytes")),
                v2_0: too_large(),
                why: Accepted::ChoiceBound,
            },
        )
        .with(Setup::Choice(oversized)),
    );
    let bom = "codex-led\u{feff}high-effort";
    cases.push(
        case(
            "sample choice separated by U+FEFF",
            Asset::Sample,
            "prototype",
            Expect::Differs {
                v1_1: Route::chosen(
                    "codex-led high-effort",
                    "prototype",
                    Agent::Codex,
                    "gpt-6.1-sol",
                    "high",
                )
                .outcome(),
                v2_0: choice_refusal(
                    "choice_unoffered",
                    Some(&format!(
                        "names {}, which this policy does not offer; it offers {OFFERED}",
                        json(bom)
                    )),
                ),
                why: Accepted::Whitespace,
            },
        )
        .with(Setup::Choice(bom.into())),
    );
    let nel = "codex-led\u{85}high-effort";
    cases.push(
        case(
            "sample choice separated by U+0085",
            Asset::Sample,
            "prototype",
            Expect::Differs {
                v1_1: choice_refusal(
                    "choice_unoffered",
                    Some(&format!(
                        "names {}, which this policy does not offer; it offers {OFFERED}",
                        json(nel)
                    )),
                ),
                v2_0: Route::chosen(
                    "codex-led high-effort",
                    "prototype",
                    Agent::Codex,
                    "gpt-6.1-sol",
                    "high",
                )
                .outcome(),
                why: Accepted::Whitespace,
            },
        )
        .with(Setup::Choice(nel.into())),
    );

    // A secondary jj workspace grants the main repository to both harnesses.
    let store = || Setup::Store(b"../main-repository/.jj/repo".to_vec());
    for (choice, agent, model) in [
        ("codex-led", Agent::Codex, "gpt-6.1-sol"),
        ("claude-led", Agent::Claude, "claude-opus-5-5"),
    ] {
        cases.push(
            case(
                format!("sample secondary workspace under {choice}"),
                Asset::Sample,
                "impl",
                Expect::Both(
                    Route::chosen(choice, "impl", agent, model, "high")
                        .store()
                        .outcome(),
                ),
            )
            .with(Setup::Choice(choice.into()))
            .with(store()),
        );
    }
    cases.push(
        case(
            "sample secondary workspace with a trailing newline",
            Asset::Sample,
            "impl",
            Expect::Both(
                Route::default("impl", Agent::Claude, "claude-opus-5-5", "high")
                    .store()
                    .outcome(),
            ),
        )
        .with(Setup::Store(b"../main-repository/.jj/repo\n".to_vec())),
    );
    cases.push(
        case(
            "sample primary workspace",
            Asset::Sample,
            "impl",
            Expect::Both(
                Route::default("impl", Agent::Claude, "claude-opus-5-5", "high").outcome(),
            ),
        )
        .with(Setup::PrimaryStore),
    );
    for (name, bytes) in [("empty", &b""[..]), ("blank", &b" \n\t"[..])] {
        cases.push(
            case(
                format!("sample {name} jj store file"),
                Asset::Sample,
                "impl",
                Expect::Both(store_refusal(Some("it is empty"))),
            )
            .with(Setup::Store(bytes.to_vec())),
        );
    }
    cases.push(
        case(
            "sample unreadable jj store file",
            Asset::Sample,
            "impl",
            Expect::Both(store_refusal(None)),
        )
        .with(Setup::UnreadableStore),
    );

    // The accepted differences of the jj store path.
    let replaced = Outcome::Selected {
        argv: strings(&[
            "claude",
            "--add-dir",
            "<tmp>/main\u{fffd}",
            "--model",
            "claude-opus-5-5",
            "--effort",
            "high",
            ORIGINAL,
        ]),
        provider: "anthropic".into(),
        model: "claude-opus-5-5".into(),
        effort: "high".into(),
        reason: r#"claude-led codex-sol (the default) routes "impl" to claude with model claude-opus-5-5 at effort high"#
            .into(),
        context: None,
        receipt: None,
    };
    cases.push(
        case(
            "sample jj store path that is not UTF-8",
            Asset::Sample,
            "impl",
            Expect::Differs {
                v1_1: replaced,
                v2_0: store_refusal(None),
                why: Accepted::StoreUtf8,
            },
        )
        .with(Setup::Store(b"../main\xff/.jj/repo".to_vec())),
    );
    let mut padded = b"../main-repository/.jj/repo".to_vec();
    padded.resize(70_000, b' ');
    cases.push(
        case(
            "sample jj store file beyond the source bound",
            Asset::Sample,
            "impl",
            Expect::Differs {
                v1_1: Route::default("impl", Agent::Claude, "claude-opus-5-5", "high")
                    .store()
                    .outcome(),
                v2_0: too_large(),
                why: Accepted::StoreBound,
            },
        )
        .with(Setup::Store(padded)),
    );

    // The directory route: Claude reads each snapshot root, and Codex and the
    // notes writer run with full access.
    for (name, choice, kind, route) in [
        (
            "claude reads the snapshot",
            None,
            "impl",
            Route::default("impl", Agent::Claude, "claude-opus-5-5", "high"),
        ),
        (
            "codex needs no grant",
            Some("codex-led"),
            "impl",
            Route::chosen("codex-led", "impl", Agent::Codex, "gpt-6.1-sol", "high"),
        ),
        (
            "the notes writer needs no grant",
            None,
            "release-notes",
            Route::default("release-notes", Agent::Notes, "gpt-6.1-sol", "medium"),
        ),
    ] {
        let mut entry = case(
            format!("sample directory route: {name}"),
            Asset::Sample,
            kind,
            Expect::Directory {
                v1_1: skills_unknown(),
                v2_0: route.delivered().outcome(),
            },
        )
        .with(Setup::Skills);
        if let Some(choice) = choice {
            entry = entry.with(Setup::Choice(choice.into()));
        }
        cases.push(entry);
    }
    cases.push(
        case(
            "sample directory route in a secondary workspace",
            Asset::Sample,
            "impl",
            Expect::Directory {
                v1_1: skills_unknown(),
                v2_0: Route::default("impl", Agent::Claude, "claude-opus-5-5", "high")
                    .store()
                    .delivered()
                    .outcome(),
            },
        )
        .with(Setup::Skills)
        .with(store()),
    );

    cases
}

fn json(text: &str) -> String {
    serde_json::to_string(text).unwrap()
}

// ---------------------------------------------------------------------------
// The launch.

/// One copied build, with fake agents on its private PATH.
struct Fixture {
    temp: tempfile::TempDir,
    pair: InstalledRavel,
    capture: PathBuf,
    evidence: PathBuf,
    /// The asset tree that the cases import: this checkout's, or a mutated
    /// copy for a control.
    assets: PathBuf,
}

/// The fake agents. `agent=fixture` has `=` in its name, which the wrapper's
/// shell must keep.
const AGENTS: [&str; 7] = [
    "my-codex-wrapper",
    "my-claude-wrapper",
    "my-agent-wrapper",
    "my-other-agent-wrapper",
    "agent=fixture",
    "codex",
    "claude",
];

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let source =
            std::env::var_os("GROVE_TEST_RAVEL").expect("explicit installed Ravel required");
        let pair =
            InstalledRavel::copy_from(Path::new(&source), &temp.path().join("independent-ravel"));
        let capture = temp.path().join("arguments");
        let evidence = temp.path().join("evidence");
        for name in AGENTS {
            let file = pair.bin().join(name);
            fs::write(&file, agent_script(&capture, &evidence, &pair)).unwrap();
            fs::set_permissions(file, fs::Permissions::from_mode(0o755)).unwrap();
        }
        Self {
            temp,
            pair,
            capture,
            evidence,
            assets: fs::canonicalize(ASSETS).unwrap(),
        }
    }

    /// The same build, importing a copy of the asset tree that `mutate`
    /// changes.
    fn with_mutated_assets(mut self, mutate: impl FnOnce(&Path)) -> Self {
        let copy = self.temp.path().join("mutated-assets");
        copy_tree(&self.assets, &copy);
        mutate(&copy);
        self.assets = fs::canonicalize(copy).unwrap();
        self
    }

    fn runtime(&self) -> Runtime {
        self.pair.runtime()
    }

    /// Run `case` in a fresh directory, which is also its HOME. `sample`
    /// replaces the source of the Rhai sample, for the controls.
    fn observe(&self, case: &Case, sample: Option<&str>) -> Outcome {
        let dir = tempfile::tempdir_in(self.temp.path()).unwrap();
        let cwd = fs::canonicalize(dir.path()).unwrap();
        // The sample's notes route runs /bin/bash with the staged writer in
        // the cwd, as `grove run release-notes` stages it.
        fs::write(
            cwd.join("codex-headless.sh"),
            agent_script(&self.capture, &self.evidence, &self.pair),
        )
        .unwrap();
        let config = self.install(&cwd, case, sample);
        for setup in &case.setup {
            apply(&cwd, setup);
        }
        for file in [&self.capture, &self.evidence] {
            if file.exists() {
                fs::remove_file(file).unwrap();
            }
        }
        let mut command = self.pair.command();
        command
            .args([
                "start-session",
                "--json",
                "--kind",
                case.kind,
                "--prompt",
                PROMPT,
            ])
            .env("HOME", &cwd)
            // A receipt reaches the agent only from a wrapper of the case.
            .env_remove("GROVE_PROVIDER_EVIDENCE")
            .current_dir(&cwd);
        for setup in &case.setup {
            match setup {
                Setup::Task(_) => {
                    command.arg("--task-file").arg(cwd.join("task.md"));
                }
                Setup::Context(_) => {
                    command.arg("--context").arg(cwd.join("context.json"));
                }
                Setup::TaskId(id) => {
                    command.args(["--task-id", id]);
                }
                _ => {}
            }
        }
        if let Some(config) = config {
            command.arg("--config").arg(config);
        }
        let output = command.output().unwrap();
        let stderr = String::from_utf8_lossy(&output.stderr);
        let lines: Vec<serde_json::Value> = stderr
            .lines()
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();
        let snapshot = case
            .setup
            .iter()
            .any(|setup| matches!(setup, Setup::Skills))
            .then(|| self.snapshot_root(&cwd))
            .flatten();
        let normal = |text: &str| normalise(text, &cwd, snapshot.as_deref());

        if let Some(error) = lines.iter().find_map(|line| line.get("error")) {
            assert!(
                !self.capture.exists() && !self.evidence.exists(),
                "{}: a refusal launched an agent",
                case.name
            );
            let ravel_code = error["code"].as_str().unwrap();
            let owner_code = format!("{}Code", self.runtime().prefix());
            let (code, message) = if ravel_code == self.runtime().code("refused") {
                let message = error["message"].as_str().unwrap();
                // A refusal of `load_context` names the phase before the
                // owner's message; the stage records it.
                let owner = message
                    .split_once("refused the selection")
                    .and_then(|(_, rest)| rest.split_once(": "))
                    .map(|(_, owner)| owner)
                    .unwrap_or_else(|| {
                        panic!("{}: owner refusal without its message: {error}", case.name)
                    });
                (
                    Code::Owner(leak(error[&owner_code].as_str().unwrap())),
                    owner.to_owned(),
                )
            } else {
                (
                    Code::Ravel(leak(ravel_code)),
                    error["message"].as_str().unwrap().to_owned(),
                )
            };
            assert_eq!(
                error["exit"],
                output.status.code().unwrap(),
                "{}: {error}",
                case.name
            );
            return Outcome::Refused {
                code,
                stage: error["stage"].as_str().unwrap().into(),
                exit: error["exit"].as_i64().unwrap() as i32,
                remedy: normal(error["remedy"].as_str().unwrap()),
                message: Some(normal(&message)),
            };
        }

        assert!(output.status.success(), "{}: {output:?}", case.name);
        assert_eq!(
            output.stdout, b"grove-continue",
            "{}: {output:?}",
            case.name
        );
        let notice = lines
            .iter()
            .find(|line| line.get("selection").is_some())
            .unwrap_or_else(|| panic!("{}: no routing notice: {stderr}", case.name));
        let context = match &notice["context"] {
            serde_json::Value::Null => None,
            reported => Some(comparable_context(&reported["value"], &normal)),
        };
        // Each argument ends with a NUL, so the last piece is empty. An empty
        // argument stays.
        let captured = fs::read(&self.capture).unwrap();
        let mut argv: Vec<String> = captured
            .strip_suffix(b"\0")
            .unwrap_or(&captured)
            .split(|byte| *byte == 0)
            .map(|arg| String::from_utf8(arg.to_vec()).unwrap())
            .collect();
        let last = argv.pop().expect("the prompt argument");
        argv = argv.iter().map(|arg| normal(arg)).collect();
        argv.push(if last == PROMPT {
            ORIGINAL.into()
        } else if prepared(&last, notice, snapshot.as_deref()) {
            PREPARED.into()
        } else {
            normal(&last)
        });
        let receipt = self
            .evidence
            .exists()
            .then(|| fresh_attempt(&fs::read_to_string(&self.evidence).unwrap()));
        let selection = &notice["selection"];
        Outcome::Selected {
            argv,
            provider: selection["provider"].as_str().unwrap().into(),
            model: selection["model"].as_str().unwrap().into(),
            effort: selection["effort"].as_str().unwrap().into(),
            reason: normal(selection["reason"].as_str().unwrap()),
            context,
            receipt,
        }
    }

    /// Write the configuration of `case` for this runtime. The 1.1 policy is
    /// named by `--config`, which this returns. The 2.0 script is the default
    /// entry, as an owner copies it.
    fn install(&self, cwd: &Path, case: &Case, sample: Option<&str>) -> Option<PathBuf> {
        let source = match self.runtime() {
            Runtime::V1_1 => self.typescript(case),
            Runtime::V2_0 => self.rhai(cwd, case, sample),
        };
        match self.runtime() {
            Runtime::V1_1 => {
                let policy = cwd.join("policy.ts");
                fs::write(&policy, source).unwrap();
                Some(policy)
            }
            Runtime::V2_0 => {
                let dir = cwd.join(".config/ravel");
                fs::create_dir_all(&dir).unwrap();
                fs::write(dir.join("config.rhai"), source).unwrap();
                None
            }
        }
    }

    /// The 1.1 policy of `case`.
    fn typescript(&self, case: &Case) -> String {
        let assets = &self.assets;
        let provenance = assets.join("provenance.ts");
        match case.asset {
            // The provenance wrapper must preserve these selections.
            Asset::Static | Asset::Sample => {
                let file = if case.asset == Asset::Static {
                    "examples/grove-static.ts"
                } else {
                    "sample/policy.ts"
                };
                format!(
                    "import {{ withProvenance }} from {provenance:?};\n\
                     import {{ policy as owner }} from {:?};\n\
                     export const policy = withProvenance(owner);\n",
                    assets.join(file)
                )
            }
            Asset::Extended => format!(
                "import {{ definePolicy }} from \"ravel/sdk\";\n\
                 import {{ selectRoute }} from \"ravel/examples/static\";\n\
                 import {{ lead, review, routes }} from {:?};\n\
                 export const policy = definePolicy({{\n\
                 \x20 schemaVersion: 2,\n\
                 \x20 version: \"ordinary-import-fixture\",\n\
                 \x20 select: (request) => selectRoute(\n\
                 \x20   {{ ...routes, \"my-implementation\": lead(\"high\"), \"my-review\": review(\"high\") }},\n\
                 \x20   request,\n\
                 \x20 ),\n\
                 }});\n",
                assets.join("examples/grove-static.ts")
            ),
            // `grove-review.ts` wraps its own policy in the wrapper.
            Asset::GroveReview => format!(
                "export {{ policy }} from {:?};\n",
                assets.join("examples/grove-review.ts")
            ),
            Asset::Review => format!(
                "export {{ policy }} from {:?};\n",
                assets.join("examples/review.ts")
            ),
            Asset::Gateway => format!(
                "import {{ reviewSelector }} from {:?};\n\
                 const route = (request) => ({{ program: \"my-codex-wrapper\", args: [request.prompt],\n\
                 \x20 provider: \"openai\", model: \"gateway\", effort: \"high\" }});\n\
                 export const policy = {{ schemaVersion: 2, version: \"gateway-fixture\", ...reviewSelector({{\n\
                 \x20 routes: {{ impl: route }}, reviews: {{ \"review-impl\": {{ openai: route, anthropic: route }} }},\n\
                 }}) }};\n",
                assets.join("examples/review.ts")
            ),
            Asset::Provenance => {
                let (selection, version) = owner(case);
                format!(
                    "import {{ withProvenance }} from {provenance:?};\n\
                     const selection = JSON.parse({});\n\
                     export const policy = withProvenance({{\n\
                     \x20 schemaVersion: 2,\n\
                     \x20 version: {},\n\
                     \x20 loadContext(request, host) {{\n\
                     \x20   if (request.taskFile === undefined) return request.context ?? {{ schemaVersion: 1 }};\n\
                     \x20   const read = host.readText(request.taskFile);\n\
                     \x20   return {{ schemaVersion: 1, facts: {{ marker: read.text }}, sources: [read.source] }};\n\
                     \x20 }},\n\
                     \x20 select(request) {{\n\
                     \x20   const chosen = JSON.parse(JSON.stringify(selection));\n\
                     \x20   if (Array.isArray(chosen?.args)) chosen.args = chosen.args.map((arg) => arg === \"<prompt>\" ? request.prompt : arg);\n\
                     \x20   return chosen;\n\
                     \x20 }},\n\
                     }});\n",
                    support::ravel::quote(&selection.to_string()),
                    support::ravel::quote(&version),
                )
            }
        }
    }

    /// The 2.0 configuration script of `case`.
    fn rhai(&self, cwd: &Path, case: &Case, sample: Option<&str>) -> String {
        let assets = &self.assets;
        let provenance = assets.join("provenance");
        let grove_static = assets.join("examples/grove-static");
        let header = "export const SCHEMA_VERSION = 4;\n";
        match case.asset {
            // The provenance wrapper must preserve these selections.
            Asset::Static => format!(
                "import {grove_static:?} as grove;\n\
                 import {provenance:?} as provenance;\n\
                 {header}\
                 export const VERSION = grove::VERSION;\n\
                 fn select(request, context) {{\n\
                 \x20   provenance::with_provenance(grove::select(request, context), grove::VERSION, request)\n\
                 }}\n"
            ),
            // The owner's copy of the sample is a module beside the script.
            Asset::Sample => {
                let copy = cwd.join(".config/ravel/sample.rhai");
                fs::create_dir_all(copy.parent().unwrap()).unwrap();
                let source = sample.map(str::to_owned).unwrap_or_else(|| {
                    fs::read_to_string(assets.join("sample/config.rhai")).unwrap()
                });
                fs::write(&copy, source).unwrap();
                format!(
                    "import {:?} as owner;\n\
                     import {provenance:?} as provenance;\n\
                     {header}\
                     export const VERSION = owner::VERSION;\n\
                     fn select(request, context) {{\n\
                     \x20   provenance::with_provenance(owner::select(request, context), owner::VERSION, request)\n\
                     }}\n",
                    copy.with_extension("")
                )
            }
            Asset::Extended => format!(
                "import {grove_static:?} as grove;\n\
                 {header}\
                 export const VERSION = \"ordinary-import-fixture\";\n\
                 const ROUTES = grove::ROUTES + #{{\n\
                 \x20   \"my-implementation\": grove::lead(\"high\"),\n\
                 \x20   \"my-review\": grove::review(\"high\"),\n\
                 }};\n\
                 fn select(request, context) {{ grove::select_route(global::ROUTES, request) }}\n"
            ),
            Asset::GroveReview => format!(
                "import {:?} as grove_review;\n\
                 {header}\
                 export const VERSION = grove_review::VERSION;\n\
                 fn load_context(request) {{ grove_review::load_context(request) }}\n\
                 fn select(request, context) {{ grove_review::select(request, context) }}\n",
                assets.join("examples/grove-review")
            ),
            Asset::Review => format!(
                "import {:?} as review;\n\
                 {header}\
                 export const VERSION = review::VERSION;\n\
                 fn load_context(request) {{ review::load_context(request) }}\n\
                 fn select(request, context) {{ review::select(request, context) }}\n",
                assets.join("examples/review")
            ),
            Asset::Gateway => format!(
                "import {:?} as review;\n\
                 {header}\
                 export const VERSION = \"gateway-fixture\";\n\
                 fn route() {{ |request| #{{ program: \"my-codex-wrapper\", args: [request.prompt],\n\
                 \x20   provider: \"openai\", model: \"gateway\", effort: \"high\" }} }}\n\
                 const RULES = #{{ routes: #{{ impl: route() }},\n\
                 \x20   reviews: #{{ \"review-impl\": #{{ openai: route(), anthropic: route() }} }} }};\n\
                 fn load_context(request) {{ review::load_context(request) }}\n\
                 fn select(request, context) {{ review::select_with(global::RULES, request, context) }}\n",
                assets.join("examples/review")
            ),
            Asset::Provenance => {
                let (selection, version) = owner(case);
                format!(
                    "import {provenance:?} as provenance;\n\
                     {header}\
                     export const VERSION = {};\n\
                     const SELECTION = ravel::json_parse({});\n\
                     fn load_context(request) {{\n\
                     \x20   if request.taskFile == () {{\n\
                     \x20       return if request.context == () {{ #{{ schemaVersion: 1 }} }} else {{ request.context }};\n\
                     \x20   }}\n\
                     \x20   let read = ravel::read_text(request.taskFile);\n\
                     \x20   #{{ schemaVersion: 1, facts: #{{ marker: read.text }}, sources: [read.source] }}\n\
                     }}\n\
                     fn owner(request) {{\n\
                     \x20   let chosen = global::SELECTION;\n\
                     \x20   if type_of(chosen) == \"map\" && type_of(chosen.get(\"args\")) == \"array\" {{\n\
                     \x20       let args = [];\n\
                     \x20       for arg in chosen.args {{\n\
                     \x20           args.push(if type_of(arg) == \"string\" && arg == \"<prompt>\" {{ request.prompt }} else {{ arg }});\n\
                     \x20       }}\n\
                     \x20       chosen.args = args;\n\
                     \x20   }}\n\
                     \x20   chosen\n\
                     }}\n\
                     fn select(request, context) {{\n\
                     \x20   provenance::with_provenance(owner(request), global::VERSION, request)\n\
                     }}\n",
                    support::ravel::quote(&version),
                    support::ravel::quote(&selection.to_string()),
                )
            }
        }
    }

    /// The snapshot root of the configured source, from the public
    /// `skills resolve`. Ravel 1.1 has no such verb.
    fn snapshot_root(&self, home: &Path) -> Option<String> {
        if self.runtime() == Runtime::V1_1 {
            return None;
        }
        let output = self
            .pair
            .command()
            .args(["skills", "resolve", "--json"])
            .env("HOME", home)
            .current_dir(home)
            .output()
            .unwrap();
        assert!(output.status.success(), "skills resolve: {output:?}");
        let resolved: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        Some(
            resolved["sources"][0]["snapshotRoot"]
                .as_str()
                .unwrap()
                .to_owned(),
        )
    }

    /// Every case of `cases`, checked against this runtime. Returns one line
    /// for each case that differs.
    fn check(&self, cases: &[Case], sample: Option<&str>) -> Vec<String> {
        let runtime = self.runtime();
        cases
            .iter()
            .filter_map(|case| {
                let expected = case.expect.for_runtime(runtime);
                let observed = comparable(self.observe(case, sample), expected);
                (&observed != expected).then(|| {
                    format!(
                        "{} on {}:\n  expected {expected:?}\n  observed {observed:?}",
                        case.name,
                        runtime.label()
                    )
                })
            })
            .collect()
    }
}

/// The owner selection and version of an `Asset::Provenance` case.
fn owner(case: &Case) -> (serde_json::Value, String) {
    case.setup
        .iter()
        .find_map(|setup| match setup {
            Setup::Owner { selection, version } => Some((selection.clone(), version.clone())),
            _ => None,
        })
        .unwrap_or_else(|| panic!("{}: a provenance case states its owner", case.name))
}

/// A fake agent: it writes its name and arguments, each ending with a NUL, and
/// any receipt that it received, then ends the session.
fn agent_script(capture: &Path, evidence: &Path, pair: &InstalledRavel) -> String {
    format!(
        "#!/bin/sh\nprintf '%s\\0' \"${{0##*/}}\" \"$@\" > '{}'\n\
         if [ \"${{GROVE_PROVIDER_EVIDENCE+set}}\" = set ]; then printf '%s' \"$GROVE_PROVIDER_EVIDENCE\" > '{}'; fi\n\
         '{}' end-session grove-continue\n",
        capture.display(),
        evidence.display(),
        pair.executable().display(),
    )
}

fn copy_tree(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for entry in fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let target = to.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

/// `receipt` with its attempt as `<attempt>` when the attempt is a lowercase
/// UUID v4. Another attempt stays, so that the comparison shows it.
fn fresh_attempt(receipt: &str) -> String {
    const KEY: &str = "\"attemptId\":\"";
    let Some(start) = receipt.find(KEY).map(|at| at + KEY.len()) else {
        return receipt.to_owned();
    };
    let Some(id) = receipt.get(start..start + 36) else {
        return receipt.to_owned();
    };
    let valid = id.char_indices().all(|(index, character)| match index {
        8 | 13 | 18 | 23 => character == '-',
        14 => character == '4',
        19 => "89ab".contains(character),
        _ => character.is_ascii_hexdigit() && !character.is_ascii_uppercase(),
    });
    if valid {
        format!("{}<attempt>{}", &receipt[..start], &receipt[start + 36..])
    } else {
        receipt.to_owned()
    }
}

/// A number without a fraction as an integer. Ravel 2.0 writes a JSON float
/// of the caller's document as a float, such as `1.0`, and Ravel 1.1 writes it
/// as `1`.
fn integral(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Number(number) => {
            if let Some(float) = number
                .as_f64()
                .filter(|_| !number.is_i64() && !number.is_u64())
            {
                if float.fract() == 0.0 && float.abs() < 9.0e15 {
                    *value = (float as i64).into();
                }
            }
        }
        serde_json::Value::Array(items) => items.iter_mut().for_each(integral),
        serde_json::Value::Object(fields) => fields.values_mut().for_each(integral),
        _ => {}
    }
}

/// The delivered context `value` without `measured`, its source digests as
/// `<sha256>`, its integral numbers as integers and its paths normalised.
fn comparable_context(
    value: &serde_json::Value,
    normal: &dyn Fn(&str) -> String,
) -> serde_json::Value {
    let mut value: serde_json::Value = serde_json::from_str(&normal(&value.to_string())).unwrap();
    integral(&mut value);
    if let Some(fields) = value.as_object_mut() {
        fields.remove("measured");
        if let Some(sources) = fields
            .get_mut("sources")
            .and_then(|sources| sources.as_array_mut())
        {
            for source in sources {
                if let Some(digest) = source.get_mut("sha256") {
                    *digest = "<sha256>".into();
                }
            }
        }
    }
    value
}

fn apply(cwd: &Path, setup: &Setup) {
    let choice = cwd.join(".ravel-choice");
    match setup {
        Setup::Choice(bytes) => fs::write(choice, bytes).unwrap(),
        Setup::ChoiceDirectory => fs::create_dir(choice).unwrap(),
        Setup::Store(bytes) => {
            fs::create_dir_all(cwd.join("main-repository/.jj/repo")).unwrap();
            fs::create_dir_all(cwd.join(".jj")).unwrap();
            fs::write(cwd.join(".jj/repo"), bytes).unwrap();
        }
        Setup::PrimaryStore => fs::create_dir_all(cwd.join(".jj/repo")).unwrap(),
        Setup::UnreadableStore => {
            apply(cwd, &Setup::Store(b"../main-repository/.jj/repo".to_vec()));
            fs::set_permissions(cwd.join(".jj/repo"), fs::Permissions::from_mode(0o000)).unwrap();
        }
        Setup::Skills => {
            let skill = cwd.join("skill-source/skills/demo");
            fs::create_dir_all(&skill).unwrap();
            fs::write(
                skill.join("SKILL.md"),
                "---\nname: demo\ndescription: A demo.\n---\nBody.\n",
            )
            .unwrap();
            let settings = serde_json::json!({ "skills": {
                "schemaVersion": 1,
                "sources": [{ "id": "craft", "type": "local",
                    "path": cwd.join("skill-source"), "roots": ["skills"] }],
                "mandatory": [],
            }});
            let dir = cwd.join(".config/ravel");
            fs::create_dir_all(&dir).unwrap();
            fs::write(dir.join("settings.json"), settings.to_string()).unwrap();
        }
        Setup::Task(bytes) => fs::write(cwd.join("task.md"), bytes).unwrap(),
        Setup::Context(text) => fs::write(cwd.join("context.json"), text).unwrap(),
        Setup::TaskId(_) | Setup::Owner { .. } => {}
    }
}

/// `text` with the snapshot root as `<snapshot>` and the case directory as
/// `<tmp>`.
fn normalise(text: &str, cwd: &Path, snapshot: Option<&str>) -> String {
    let mut text = text.to_owned();
    if let Some(snapshot) = snapshot {
        text = text.replace(snapshot, "<snapshot>");
    }
    text.replace(cwd.to_str().unwrap(), "<tmp>")
}

/// Whether `prompt` is the prompt that Ravel prepared from `PROMPT`: the byte
/// count of the notice, `PROMPT` inside it, and the snapshot named in it.
fn prepared(prompt: &str, notice: &serde_json::Value, snapshot: Option<&str>) -> bool {
    let skills = &notice["skills"];
    snapshot.is_some_and(|snapshot| prompt.contains(snapshot))
        && prompt != PROMPT
        && prompt.contains(PROMPT)
        && skills["prompt"]["bytes"].as_u64() == Some(prompt.len() as u64)
        && skills["originalPrompt"]["bytes"].as_u64() == Some(PROMPT.len() as u64)
}

/// `observed`, without the message where `expected` leaves it out.
fn comparable(observed: Outcome, expected: &Outcome) -> Outcome {
    match (observed, expected) {
        (
            Outcome::Refused {
                code,
                stage,
                exit,
                remedy,
                ..
            },
            Outcome::Refused { message: None, .. },
        ) => Outcome::Refused {
            code,
            stage,
            exit,
            remedy,
            message: None,
        },
        (observed, _) => observed,
    }
}

fn leak(text: &str) -> &'static str {
    Box::leak(text.to_owned().into_boxed_str())
}

fn assets(cases: Vec<Case>, wanted: &[Asset]) -> Vec<Case> {
    cases
        .into_iter()
        .filter(|case| wanted.contains(&case.asset))
        .collect()
}

fn report(failures: Vec<String>) {
    assert!(
        failures.is_empty(),
        "{} cases differ:\n{}",
        failures.len(),
        failures.join("\n")
    );
}

// ---------------------------------------------------------------------------
// The checks.

#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn static_example_matches_the_case_table() {
    let fixture = Fixture::new();
    report(fixture.check(&assets(cases(), &[Asset::Static, Asset::Extended]), None));
}

#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn sample_matches_the_case_table() {
    let fixture = Fixture::new();
    report(fixture.check(&assets(cases(), &[Asset::Sample]), None));
}

#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn review_assets_match_the_case_table() {
    let fixture = Fixture::new();
    report(fixture.check(
        &assets(
            cases(),
            &[
                Asset::GroveReview,
                Asset::Review,
                Asset::Gateway,
                Asset::Provenance,
            ],
        ),
        None,
    ));
}

/// The controls of the Rhai review assets: a wrapper whose receipt has another
/// member order, and helpers that trim with Rhai's own `trim`.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn rhai_review_controls_fail() {
    if Fixture::new().runtime() != Runtime::V2_0 {
        eprintln!("not run: the Rhai review assets need the 2.0 runtime");
        return;
    }
    let all = cases();
    let named = |names: &[&str]| -> Vec<Case> {
        let chosen: Vec<Case> = all
            .iter()
            .filter(|case| names.contains(&case.name.as_str()))
            .cloned()
            .collect();
        assert_eq!(chosen.len(), names.len(), "{names:?}");
        chosen
    };

    // Each receipt that the wrapper records, and the loader's reading of one,
    // must fail when the member order changes.
    let order = named(&[
        "provenance records recording 0",
        "provenance records recording 1",
        "grove review reads recording 1",
    ]);
    let reordered = Fixture::new().with_mutated_assets(|assets| {
        let file = assets.join("provenance.rhai");
        let source = fs::read_to_string(&file).unwrap();
        let changed = source.replace(
            r#"const MEMBERS = ["schemaVersion", "attemptId", "provider","#,
            r#"const MEMBERS = ["schemaVersion", "provider", "attemptId","#,
        );
        assert_ne!(changed, source, "the order control must change the wrapper");
        fs::write(file, changed).unwrap();
    });
    assert_eq!(reordered.check(&order, None).len(), order.len());
    assert!(Fixture::new().check(&order, None).is_empty());

    // Rhai's `trim` keeps U+FEFF and removes U+0085.
    let blank = named(&[
        "grove review declared U+FEFF is blank",
        "grove review declared U+0085 is a label",
        "provenance refuses a blank provider U+FEFF",
        "grove review receipt version U+FEFF is blank",
    ]);
    let trimmed = Fixture::new().with_mutated_assets(|assets| {
        let file = assets.join("provenance.rhai");
        let source = fs::read_to_string(&file).unwrap();
        let changed = source.replace(
            "fn blank(text) {",
            "fn blank(text) {\n    let trimmed = text;\n    trimmed.trim();\n    return trimmed == \"\";",
        );
        assert_ne!(changed, source, "the trim control must change the helpers");
        fs::write(file, changed).unwrap();
    });
    assert_eq!(trimmed.check(&blank, None).len(), blank.len());
    assert!(Fixture::new().check(&blank, None).is_empty());
}

// A changed expectation in the table must fail against the same build.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn a_changed_expectation_fails() {
    let fixture = Fixture::new();
    let runtime = fixture.runtime();
    let mut changed: Vec<Case> = cases()
        .into_iter()
        .filter(|case| {
            ["static impl", "sample refuses choice \"unknown\""].contains(&case.name.as_str())
        })
        .collect();
    assert_eq!(changed.len(), 2);
    for case in &mut changed {
        let mut outcome = case.expect.for_runtime(runtime).clone();
        match &mut outcome {
            Outcome::Selected { effort, .. } => *effort = "low".into(),
            Outcome::Refused { remedy, .. } => remedy.push_str(" changed"),
        }
        case.expect = Expect::Both(outcome);
    }
    assert_eq!(fixture.check(&changed, None).len(), 2);
}

/// The controls of the Rhai sample: a source that passes the caller's
/// original prompt, and one that grants a directory with delivery disabled.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn rhai_sample_controls_fail() {
    let fixture = Fixture::new();
    if fixture.runtime() != Runtime::V2_0 {
        eprintln!("not run: the Rhai sample needs the 2.0 runtime");
        return;
    }
    let source = fs::read_to_string(Path::new(ASSETS).join("sample/config.rhai")).unwrap();
    let all = cases();
    let named = |name: &str| -> Vec<Case> {
        all.iter()
            .filter(|case| case.name == name)
            .cloned()
            .collect()
    };

    let original = source.replace(
        "harness_args(name, model, effort, request.prompt,",
        "harness_args(name, model, effort, request.originalPrompt,",
    );
    assert_ne!(
        original, source,
        "the prompt control must change the sample"
    );
    let delivered = named("sample directory route: claude reads the snapshot");
    assert_eq!(fixture.check(&delivered, Some(&original)).len(), 1);
    assert!(fixture.check(&delivered, None).is_empty());

    let granted = source.replace(
        "if skills == () { return []; }",
        "if skills == () { return [\"/granted-without-delivery\"]; }",
    );
    assert_ne!(granted, source, "the grant control must change the sample");
    let disabled = named("sample default impl");
    assert_eq!(fixture.check(&disabled, Some(&granted)).len(), 1);
    assert!(fixture.check(&disabled, None).is_empty());
}

// The table itself, without Ravel: each accepted difference has a case with two
// different outcomes, and a case that states one outcome states it for both.
#[test]
fn every_accepted_difference_has_a_case_with_both_outcomes() {
    let cases = cases();
    for accepted in Accepted::ALL {
        assert!(!accepted.reason().is_empty());
        let proving: Vec<&Case> = cases
            .iter()
            .filter(|case| matches!(case.expect, Expect::Differs { why, .. } if why == accepted))
            .collect();
        assert!(!proving.is_empty(), "{accepted:?} has no case");
        for case in proving {
            let Expect::Differs { v1_1, v2_0, .. } = &case.expect else {
                unreachable!()
            };
            assert_ne!(
                v1_1, v2_0,
                "{}: an accepted difference with equal outcomes",
                case.name
            );
        }
    }
    let mut names: Vec<&str> = cases.iter().map(|case| case.name.as_str()).collect();
    names.sort_unstable();
    let count = names.len();
    names.dedup();
    assert_eq!(names.len(), count, "case names are unique");
    for case in &cases {
        if let Expect::Directory { v1_1, v2_0 } = &case.expect {
            assert_eq!(v1_1, &skills_unknown(), "{}", case.name);
            assert!(
                case.setup
                    .iter()
                    .any(|setup| matches!(setup, Setup::Skills)),
                "{}",
                case.name
            );
            assert!(
                matches!(v2_0, Outcome::Selected { argv, .. } if argv.last().unwrap() == PREPARED)
            );
        } else {
            assert!(
                !case
                    .setup
                    .iter()
                    .any(|setup| matches!(setup, Setup::Skills)),
                "{}",
                case.name
            );
        }
    }
}

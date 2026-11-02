//! The review cases of the asset table: the task-file adapter with Grove's
//! review tables (`Asset::GroveReview`), the generic review example
//! (`Asset::Review`), owner tables with a gateway route (`Asset::Gateway`) and
//! the provider-evidence wrapper (`Asset::Provenance`).
//!
//! **Receipts cross runtimes.** `RECORDINGS` lists selections. For each one, a
//! wrapper case states the receipt bytes that the wrapper of either runtime
//! must write, and a loader case gives those bytes, with a fixed attempt, to a
//! review loader. The wrapper of each runtime writes the same bytes, and the
//! loader of each runtime accepts them. So a receipt that one runtime records
//! passes the loader of the other, in both directions.
use super::*;

/// The attempt of each receipt that a loader case reads.
pub(super) const ATTEMPT: &str = "12345678-1234-4123-8123-123456789abc";

const REVIEW_VERSION: &str = "grove/examples/grove-review 1";
const TASK: &str = "<tmp>/task.md";

/// One selection of the wrapper: provider, owner version, kind and task.
type Recording = (
    &'static str,
    &'static str,
    &'static str,
    Option<&'static str>,
);

/// The selections whose receipts cross runtimes. The labels hold characters
/// that the JSON encoders must write alike: a control character as `\u0001`,
/// a tab, backspace and form feed by their short escapes, U+007F, U+2028, `/`,
/// U+0085 and a character beyond the Basic Multilingual Plane as themselves,
/// and a quotation mark and a backslash escaped.
const RECORDINGS: [Recording; 4] = [
    ("openai", "old-owner", "art", Some("closing-child-k3")),
    (
        "openai",
        "owner \u{e9}\u{1}\u{7f}\u{2028}/\"\\ \u{1F600} \u{85}",
        "impl",
        Some("task-k1"),
    ),
    ("openai", "v\tb\u{8}\u{c}", "impl", None),
    ("your-provider", "original-owner", "feature", None),
];

/// The characters that ECMAScript's `trim` removes, as Bun 1.4.3 enumerated
/// them over every code point.
pub(super) const ECMASCRIPT_SPACE: [char; 25] = [
    '\u{9}', '\u{a}', '\u{b}', '\u{c}', '\u{d}', '\u{20}', '\u{a0}', '\u{1680}', '\u{2000}',
    '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}', '\u{2008}',
    '\u{2009}', '\u{200a}', '\u{2028}', '\u{2029}', '\u{202f}', '\u{205f}', '\u{3000}', '\u{feff}',
];

/// Characters that one runtime's own trimming would class wrongly: Rust and
/// Rhai count U+0085 as whitespace, and older Unicode counted U+180E.
const NOT_SPACE: [char; 2] = ['\u{85}', '\u{180e}'];

const REVIEW_KINDS: [(&str, &str); 5] = [
    ("review-requirements", "xhigh"),
    ("review-design", "xhigh"),
    ("review-planning", "xhigh"),
    ("review-prototype", "medium"),
    ("review-impl", "high"),
];

/// `template` with the fixed attempt.
fn recorded(template: &str) -> String {
    template.replace("<attempt>", ATTEMPT)
}

fn recording_receipt((provider, version, kind, task): Recording) -> String {
    receipt(provider, version, kind, task)
}

/// The receipt that a loader delivers in `facts.creator.receipt`.
fn receipt_value(template: &str) -> serde_json::Value {
    serde_json::from_str(&recorded(template)).unwrap()
}

/// A task file whose relationship is `producer-k1` and whose creator line is
/// `creator`.
fn review_task(creator: &str) -> Vec<u8> {
    format!("**Reviews:** producer-k1\n**Creator:** {creator}\n").into_bytes()
}

/// The context that the adapter delivers for `task` with `facts`.
fn task_context(task: &[u8], facts: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "schemaVersion": 1,
        "facts": facts,
        "sources": [{ "name": TASK, "bytes": task.len(), "sha256": "<sha256>" }],
    })
}

/// The creator evidence in a delivered context.
enum Creator {
    Declared(&'static str),
    /// A receipt template and its provider.
    Recorded(String, &'static str),
}

impl Creator {
    fn provider(&self) -> &'static str {
        match self {
            Creator::Declared(provider) | Creator::Recorded(_, provider) => provider,
        }
    }

    fn fact(&self) -> serde_json::Value {
        match self {
            Creator::Declared(provider) => {
                serde_json::json!({ "evidence": "declared", "provider": provider })
            }
            Creator::Recorded(template, provider) => serde_json::json!({
                "evidence": "recorded", "provider": provider, "receipt": receipt_value(template),
            }),
        }
    }

    fn description(&self) -> String {
        match self {
            Creator::Declared(provider) => {
                format!("creator origin {} declared by the owner", json(provider))
            }
            Creator::Recorded(template, provider) => {
                let value = receipt_value(template);
                format!(
                    "creator attempt {ATTEMPT} (kind {}, task {}) recorded origin {}",
                    value["kind"],
                    value.get("taskId").unwrap_or(&serde_json::Value::Null),
                    json(provider)
                )
            }
        }
    }
}

/// A selected reviewer: its argument vector and labels.
struct Reviewer {
    program: &'static str,
    provider: &'static str,
    model: &'static str,
    /// Whether the route's arguments are `--model`, `--effort` and the prompt.
    flags: bool,
}

const LEAD: Reviewer = Reviewer {
    program: "my-codex-wrapper",
    provider: "openai",
    model: "your-codex-model",
    flags: true,
};
const CLAUDE: Reviewer = Reviewer {
    program: "my-claude-wrapper",
    provider: "anthropic",
    model: "your-claude-model",
    flags: true,
};
const AGENT: Reviewer = Reviewer {
    program: "my-agent-wrapper",
    provider: "your-provider",
    model: "your-model",
    flags: true,
};
const OTHER: Reviewer = Reviewer {
    program: "my-other-agent-wrapper",
    provider: "your-other-provider",
    model: "your-other-model",
    flags: true,
};
const GATEWAY: Reviewer = Reviewer {
    program: "my-codex-wrapper",
    provider: "openai",
    model: "gateway",
    flags: false,
};

/// The selection of a review kind: the reviewer of `creator`'s origin.
fn selected_review(
    kind: &str,
    effort: &str,
    artifact: &str,
    creator: &Creator,
    reviewer: &Reviewer,
    context: serde_json::Value,
    receipt: Option<String>,
) -> Outcome {
    let argv = if reviewer.flags {
        strings(&[
            reviewer.program,
            "--model",
            reviewer.model,
            "--effort",
            effort,
            ORIGINAL,
        ])
    } else {
        strings(&[reviewer.program, ORIGINAL])
    };
    Outcome::Selected {
        argv,
        provider: reviewer.provider.into(),
        model: reviewer.model.into(),
        effort: effort.into(),
        reason: format!(
            "review kind {} of {}: {}; reviewer origin {}, effort {effort}",
            json(kind),
            json(artifact),
            creator.description(),
            json(reviewer.provider)
        ),
        context: Some(context),
        receipt,
    }
}

/// The selection of Grove's review tables for a task file with `creator`.
fn grove_review(kind: &str, effort: &str, task: &[u8], creator: &Creator) -> Outcome {
    let reviewer = if creator.provider() == "openai" {
        &CLAUDE
    } else {
        &LEAD
    };
    let facts = serde_json::json!({
        "reviewedArtifact": { "id": "producer-k1" },
        "creator": creator.fact(),
    });
    selected_review(
        kind,
        effort,
        "producer-k1",
        creator,
        reviewer,
        task_context(task, facts),
        Some(receipt(reviewer.provider, REVIEW_VERSION, kind, None)),
    )
}

/// `outcome` with `context` and `receipt`.
fn delivered(
    mut outcome: Outcome,
    context: serde_json::Value,
    receipt_text: Option<String>,
) -> Outcome {
    if let Outcome::Selected {
        context: slot,
        receipt: receipt_slot,
        ..
    } = &mut outcome
    {
        *slot = Some(context);
        *receipt_slot = receipt_text;
    }
    outcome
}

/// A refusal of the task-file adapter, which refuses in `load_context`.
fn loader_refusal(code: &'static str, message: String, remedy: &str) -> Outcome {
    Outcome::Refused {
        code: Code::Owner(code),
        stage: "context".into(),
        exit: 3,
        remedy: remedy.into(),
        message: Some(message),
    }
}

fn malformed(line: usize) -> Outcome {
    loader_refusal(
        "creator_line_malformed",
        format!("line {line} of task file {TASK} has invalid creator evidence"),
        "repair \"**Creator:** recorded <canonical receipt>\" or have the owner write \
         \"**Creator:** declared <provider>\"; no duplicate/extra fields, newlines or NULs, at \
         most 8192 receipt bytes",
    )
}

fn origin_unlisted(kind: &str, creator: &Creator, origins: &str) -> Outcome {
    owner_refusal(
        "creator_origin_unlisted",
        Some(format!(
            "{}; reviews[{}] lists {origins}. Origins match exactly",
            creator.description(),
            json(kind)
        )),
        "correct the declaration or list the recorded origin in the review table; a \
         relabelled provider is still the same provider",
    )
}

fn task_refusal(code: &'static str, message: String, remedy: &str) -> Outcome {
    loader_refusal(code, message, remedy)
}

fn duplicate(marker: &str, lines: &str) -> Outcome {
    let code = if marker == "reviews" {
        "reviews_line_duplicate"
    } else {
        "creator_line_duplicate"
    };
    task_refusal(
        code,
        format!("task file {TASK} has duplicate {marker} lines at {lines}"),
        "keep one relationship and its finishing creator; indent examples, since markers \
         count in fenced blocks too",
    )
}

fn reviews_malformed(line: usize) -> Outcome {
    task_refusal(
        "reviews_line_malformed",
        format!("line {line} of task file {TASK} is not \"**Reviews:** <handle>\""),
        "use lowercase letters, digits and single dashes then -k and a key with no leading \
         zero; one space, no trailing text",
    )
}

fn creator_missing() -> Outcome {
    task_refusal(
        "creator_line_missing",
        format!(
            "task file {TASK} reviews \"producer-k1\" but its creator is unknown. An earlier \
             same-task attempt supplies no evidence"
        ),
        "have the owner write \"**Creator:** declared <provider>\" directly under Reviews, or \
         copy the finishing session's recorded receipt",
    )
}

fn misplaced(creator: usize, reviews: usize) -> Outcome {
    task_refusal(
        "creator_line_misplaced",
        format!(
            "task file {TASK}'s Creator at line {creator} is not directly beneath Reviews at \
             line {reviews}"
        ),
        "move the one Creator line directly beneath Reviews",
    )
}

fn creator_invalid() -> Outcome {
    owner_refusal(
        "creator_invalid",
        Some("creator evidence for \"external\" is invalid".into()),
        "repair the recorded receipt and matching provider or have the owner declare the \
         original provider",
    )
}

fn artifact_missing(kind: &str) -> Outcome {
    owner_refusal(
        "reviewed_artifact_missing",
        Some(format!(
            "review kind {} has no valid reviewed artifact",
            json(kind)
        )),
        "supply facts.reviewedArtifact.id and facts.creator, or use Grove's task-file adapter",
    )
}

fn provenance_invalid() -> Outcome {
    owner_refusal(
        "provenance_invalid",
        Some("the selected provider evidence is invalid or exceeds 8192 bytes".into()),
        "use nonblank single-line labels and a receipt within Grove's 8192-byte bound",
    )
}

fn command_invalid() -> Outcome {
    owner_refusal(
        "provenance_command_invalid",
        Some("the owner selected an invalid program or argument vector".into()),
        "repair the owner policy command",
    )
}

/// A generic caller context whose creator is `creator`, as JSON text.
fn caller_facts(artifact: serde_json::Value, creator: Option<&str>) -> String {
    let creator = creator
        .map(|creator| format!(",\"creator\":{creator}"))
        .unwrap_or_default();
    format!("{{\"schemaVersion\":1,\"facts\":{{\"reviewedArtifact\":{artifact}{creator}}}}}")
}

/// The owner selection of a wrapper case, with the prompt last.
fn owner_selection(provider: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "status": "selected", "program": "my-agent-wrapper", "args": ["<prompt>"],
        "provider": provider, "model": "owner-model", "effort": "none", "reason": "owner route",
    })
}

fn owner(selection: serde_json::Value, version: &str) -> Setup {
    Setup::Owner {
        selection,
        version: version.into(),
    }
}

/// The selection of the wrapper case whose owner selected `provider`.
fn wrapped(provider: &str, receipt: String) -> Outcome {
    Outcome::Selected {
        argv: strings(&["my-agent-wrapper", ORIGINAL]),
        provider: provider.into(),
        model: "owner-model".into(),
        effort: "none".into(),
        reason: "owner route".into(),
        context: Some(serde_json::json!({ "schemaVersion": 1 })),
        receipt: Some(receipt),
    }
}

pub(super) fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    grove_review_cases(&mut cases);
    generic_review_cases(&mut cases);
    gateway_cases(&mut cases);
    provenance_cases(&mut cases);
    cases
}

fn grove_review_cases(cases: &mut Vec<Case>) {
    let art = recording_receipt(RECORDINGS[0]);
    // Each review kind with a recorded receipt and both declarations, in a
    // CRLF task. The reviewer has the other origin.
    for (kind, effort) in REVIEW_KINDS {
        for creator in [
            Creator::Recorded(art.clone(), "openai"),
            Creator::Declared("openai"),
            Creator::Declared("anthropic"),
        ] {
            let line = match &creator {
                Creator::Declared(provider) => format!("declared {provider}"),
                Creator::Recorded(template, _) => format!("recorded {}", recorded(template)),
            };
            let task = format!("# review\r\n**Reviews:** producer-k1\r\n**Creator:** {line}\r\n")
                .into_bytes();
            let name = match &creator {
                Creator::Declared(provider) => format!("grove review {kind} declared {provider}"),
                Creator::Recorded(..) => format!("grove review {kind} recorded"),
            };
            cases.push(
                case(
                    name,
                    Asset::GroveReview,
                    kind,
                    Expect::Both(grove_review(kind, effort, &task, &creator)),
                )
                .with(Setup::Task(task)),
            );
        }
    }

    // Each recorded selection passes the loader.
    for (index, recording) in RECORDINGS.iter().enumerate().skip(1) {
        let template = recording_receipt(*recording);
        let task = review_task(&format!("recorded {}", recorded(&template)));
        let provider = recording.0;
        let outcome = if provider == "openai" {
            grove_review(
                "review-impl",
                "high",
                &task,
                &Creator::Recorded(template, "openai"),
            )
        } else {
            origin_unlisted(
                "review-impl",
                &Creator::Recorded(template, provider),
                "\"anthropic\", \"openai\"",
            )
        };
        cases.push(
            case(
                format!("grove review reads recording {index}"),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(outcome),
            )
            .with(Setup::Task(task)),
        );
    }

    // The nonreview kinds keep the static routes, wrapped.
    for (kind, reviewer, effort) in STATIC_KINDS {
        if kind.starts_with("review-") {
            continue;
        }
        cases.push(case(
            format!("grove review routes {kind}"),
            Asset::GroveReview,
            kind,
            Expect::Both(delivered(
                wrapper(reviewer, kind, effort, ORIGINAL),
                serde_json::json!({ "schemaVersion": 1 }),
                Some(receipt(
                    if reviewer { "anthropic" } else { "openai" },
                    REVIEW_VERSION,
                    kind,
                    None,
                )),
            )),
        ));
    }
    cases.push(case(
        "grove review unlisted kind",
        Asset::GroveReview,
        "unmapped",
        Expect::Both(owner_refusal(
            "incomplete_mapping",
            Some(r#"the routes name no command for kind "unmapped""#.into()),
            "add a route for this kind in your copy of the policy",
        )),
    ));

    // Malformed relationships refuse before any agent runs.
    let declared = "**Creator:** declared openai";
    let bad: Vec<(&str, String, Outcome)> = vec![
        (
            "no reviews line",
            "# review\n".into(),
            task_refusal(
                "reviews_line_missing",
                format!("task file {TASK} has no **Reviews:** line"),
                "add \"**Reviews:** <handle>\" and a Creator line directly beneath it",
            ),
        ),
        (
            "no creator line",
            "**Reviews:** producer-k1\n".into(),
            creator_missing(),
        ),
        (
            "a leading zero in the key",
            format!("**Reviews:** producer-k01\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a trailing space after the handle",
            format!("**Reviews:** producer-k1 \n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "no space after the reviews marker",
            format!("**Reviews:**producer-k1\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a bare reviews marker",
            format!("**Reviews:**\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "an uppercase handle",
            format!("**Reviews:** Producer-k1\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a double dash in the handle",
            format!("**Reviews:** producer--k1\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a handle without a key",
            format!("**Reviews:** producer\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a key without digits",
            format!("**Reviews:** producer-k\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a key without its letter",
            format!("**Reviews:** producer-x1\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a letter after the key's digits",
            format!("**Reviews:** producer-k1a\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a handle that is only a key",
            format!("**Reviews:** k1\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "a non-ASCII letter in the handle",
            format!("**Reviews:** produc\u{e9}r-k1\n{declared}\n"),
            reviews_malformed(1),
        ),
        (
            "two reviews lines",
            format!("**Reviews:** producer-k1\n**Reviews:** other-k2\n{declared}\n"),
            duplicate("reviews", "1, 2"),
        ),
        (
            "two creator lines",
            format!("**Reviews:** producer-k1\n{declared}\n**Creator:** declared anthropic\n"),
            duplicate("creator", "2, 3"),
        ),
        (
            "a blank line before the creator",
            format!("**Reviews:** producer-k1\n\n{declared}\n"),
            misplaced(3, 1),
        ),
        (
            "the creator above the relationship",
            format!("{declared}\n**Reviews:** producer-k1\n"),
            misplaced(1, 2),
        ),
        (
            "an indented creator",
            format!("**Reviews:** producer-k1\n {declared}\n"),
            creator_missing(),
        ),
        (
            "no space after the creator marker",
            "**Reviews:** producer-k1\n**Creator:**declared openai\n".into(),
            malformed(2),
        ),
        (
            "a declaration without a provider",
            "**Reviews:** producer-k1\n**Creator:** declared \n".into(),
            malformed(2),
        ),
        (
            "a NUL in the declaration",
            format!("**Reviews:** producer-k1\n{declared}\0\n"),
            malformed(2),
        ),
        (
            "a carriage return inside the declaration",
            "**Reviews:** producer-k1\n**Creator:** declared open\rai\n".into(),
            malformed(2),
        ),
        (
            "an unknown evidence form",
            "**Reviews:** producer-k1\n**Creator:** asserted openai\n".into(),
            malformed(2),
        ),
        (
            "a legacy dispatch run",
            "**Reviews:** producer-k1\n**Creator:** run 12345678-1234-4123-8123-123456789abc\n"
                .into(),
            task_refusal(
                "creator_legacy_unsupported",
                format!(
                    "task file {TASK} names an original-v22 dispatch run; the new consumer has \
                     no run lookup"
                ),
                "have the owner write \"**Creator:** declared <provider>\" or finish again \
                 under a receipt-producing policy; keep the original v22 driver's ending \
                 contract",
            ),
        ),
        (
            "a marker inside a fence",
            format!("**Reviews:** producer-k1\n{declared}\n```\n**Reviews:** other-k2\n```\n"),
            duplicate("reviews", "1, 4"),
        ),
    ];
    for (name, body, outcome) in bad {
        cases.push(
            case(
                format!("grove review refuses {name}"),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(outcome),
            )
            .with(Setup::Task(body.into_bytes())),
        );
    }

    // Origins match exactly.
    for (name, provider) in [
        ("uppercase", "OPENAI"),
        ("a trailing space", "openai "),
        ("a prototype key", "__proto__"),
    ] {
        let creator = Creator::Declared(provider);
        cases.push(
            case(
                format!("grove review refuses the origin with {name}"),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(origin_unlisted(
                    "review-impl",
                    &creator,
                    "\"anthropic\", \"openai\"",
                )),
            )
            .with(Setup::Task(review_task(&format!("declared {provider}")))),
        );
    }

    // A receipt is only the canonical record.
    let valid = recorded(&art);
    let mut receipts: Vec<(String, String)> =
        vec![
        ("an empty receipt".into(), String::new()),
        ("an empty object".into(), "{}".into()),
        ("an array".into(), "[]".into()),
        ("null".into(), "null".into()),
        ("a leading space".into(), format!(" {valid}")),
        ("a trailing space".into(), format!("{valid} ")),
        (
            "a carriage return inside".into(),
            valid.replace(",\"kind\"", "\r,\"kind\""),
        ),
        ("a NUL inside".into(), valid.replace(",\"kind\"", "\0,\"kind\"")),
        (
            "a duplicate key".into(),
            valid.replace("\"provider\":", "\"provider\":\"anthropic\",\"provider\":"),
        ),
        (
            "another member order".into(),
            valid.replace(
                "\"attemptId\":\"12345678-1234-4123-8123-123456789abc\",\"provider\":\"openai\"",
                "\"provider\":\"openai\",\"attemptId\":\"12345678-1234-4123-8123-123456789abc\"",
            ),
        ),
        (
            "an escaped letter".into(),
            valid.replace("\"openai\"", "\"\\u006fpenai\""),
        ),
        (
            "a space after a colon".into(),
            valid.replace("\"kind\":", "\"kind\": "),
        ),
        (
            "a float schema version".into(),
            valid.replace("\"schemaVersion\":1", "\"schemaVersion\":1.0"),
        ),
        (
            "an uppercase attempt".into(),
            valid.replace(ATTEMPT, &ATTEMPT.to_uppercase()),
        ),
        (
            "a misplaced dash in the attempt".into(),
            valid.replace("12345678-1234", "1234567-81234"),
        ),
        (
            "an attempt of version 1".into(),
            valid.replace("-4123-", "-1123-"),
        ),
        (
            "an attempt of another variant".into(),
            valid.replace("-8123-", "-c123-"),
        ),
        (
            "a receipt beyond 8192 bytes".into(),
            valid.replace("closing-child-k3", &"x".repeat(8192)),
        ),
    ];
    // One member changed or added, the others canonical.
    let base: Vec<(&str, serde_json::Value)> = vec![
        ("schemaVersion", 1.into()),
        ("attemptId", ATTEMPT.into()),
        ("provider", "openai".into()),
        ("policyVersion", "old-owner".into()),
        ("kind", "art".into()),
        ("taskId", "closing-child-k3".into()),
    ];
    let members = |list: &[(&str, serde_json::Value)]| {
        let joined: Vec<String> = list
            .iter()
            .map(|(name, value)| format!("{}:{value}", json(name)))
            .collect();
        format!("{{{}}}", joined.join(","))
    };
    assert_eq!(members(&base), valid);
    for (field, replacement) in [
        ("schemaVersion", serde_json::json!(2)),
        ("attemptId", serde_json::json!("not-a-launch")),
        ("provider", serde_json::json!("")),
        ("provider", serde_json::json!(false)),
        ("policyVersion", serde_json::json!(null)),
        ("kind", serde_json::json!("impl\nreview")),
        ("taskId", serde_json::json!("")),
        ("unknown", serde_json::json!("extra")),
    ] {
        let mut list = base.clone();
        match list.iter_mut().find(|(name, _)| *name == field) {
            Some(slot) => slot.1 = replacement.clone(),
            None => list.push((field, replacement.clone())),
        }
        receipts.push((format!("{field} {replacement}"), members(&list)));
    }
    for (name, receipt) in receipts {
        cases.push(
            case(
                format!("grove review refuses the receipt: {name}"),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(malformed(2)),
            )
            .with(Setup::Task(review_task(&format!("recorded {receipt}")))),
        );
    }

    // A label is blank by ECMAScript's trim in both runtimes.
    for character in ECMASCRIPT_SPACE {
        cases.push(
            case(
                format!("grove review declared U+{:04X} is blank", character as u32),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(malformed(2)),
            )
            .with(Setup::Task(review_task(&format!("declared {character}")))),
        );
    }
    for character in NOT_SPACE {
        let provider: &'static str = leak(&character.to_string());
        cases.push(
            case(
                format!(
                    "grove review declared U+{:04X} is a label",
                    character as u32
                ),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(origin_unlisted(
                    "review-impl",
                    &Creator::Declared(provider),
                    "\"anthropic\", \"openai\"",
                )),
            )
            .with(Setup::Task(review_task(&format!("declared {provider}")))),
        );
    }
    // Ravel of either major refuses an owner version or a provider of U+0085
    // alone, by Rust's whitespace, so no wrapper records one. The declared
    // cases above class U+0085 through the same helper.
    for character in ['\u{feff}', '\u{3000}'] {
        let template = art.replace("\"old-owner\"", &json(&character.to_string()));
        cases.push(
            case(
                format!(
                    "grove review receipt version U+{:04X} is blank",
                    character as u32
                ),
                Asset::GroveReview,
                "review-impl",
                Expect::Both(malformed(2)),
            )
            .with(Setup::Task(review_task(&format!(
                "recorded {}",
                recorded(&template)
            )))),
        );
    }

    // The accepted surrogate difference. The TypeScript loader accepts the
    // receipt, and Ravel 1.1 then refuses the context that carries it. The
    // Rhai loader refuses the receipt itself.
    let surrogate = art.replace("\"old-owner\"", "\"old\\ud800owner\"");
    let task = review_task(&format!("recorded {}", recorded(&surrogate)));
    let accepted = Outcome::Refused {
        code: Code::Ravel("context_invalid"),
        stage: "context".into(),
        exit: 3,
        remedy: "correct what loadContext in <tmp>/policy.ts returns at \
                 context.facts[\"creator\"][\"receipt\"][\"policyVersion\"]; the types in \
                 ravel/sdk describe a version-1 context"
            .into(),
        message: Some(
            "the context loadContext returned is invalid: a string with a lone surrogate \
             cannot be carried in JSON, so no context can deliver it"
                .into(),
        ),
    };
    cases.push(
        case(
            "grove review receipt escapes an unpaired surrogate",
            Asset::GroveReview,
            "review-impl",
            Expect::Differs {
                v1_1: accepted,
                v2_0: malformed(2),
                why: Accepted::Surrogate,
            },
        )
        .with(Setup::Task(task)),
    );

    // Task evidence against the kind and the caller.
    cases.push(
        case(
            "grove review refuses Reviews for a nonreview kind",
            Asset::GroveReview,
            "impl",
            Expect::Both(task_refusal(
                "review_kind_unlisted",
                format!(
                    "task file {TASK} declares **Reviews:** at line 1, but kind \"impl\" is not \
                     listed as a review"
                ),
                "list the kind in reviews so the provider rule applies, or remove Reviews for a \
                 nonreview task",
            )),
        )
        .with(Setup::Task(review_task("declared openai"))),
    );
    let plain = b"# implementation\n".to_vec();
    cases.push(
        case(
            "grove review attributes the task of a nonreview kind",
            Asset::GroveReview,
            "impl",
            Expect::Both(delivered(
                wrapper(false, "impl", "high", ORIGINAL),
                serde_json::json!({
                    "schemaVersion": 1,
                    "sources": [{ "name": TASK, "bytes": plain.len(), "sha256": "<sha256>" }],
                }),
                Some(receipt("openai", REVIEW_VERSION, "impl", None)),
            )),
        )
        .with(Setup::Task(plain)),
    );
    cases.push(
        case(
            "grove review refuses caller review facts",
            Asset::GroveReview,
            "review-impl",
            Expect::Both(task_refusal(
                "reviewed_artifact_conflict",
                "caller review facts conflict with the task's Reviews/Creator lines".into(),
                "leave facts.reviewedArtifact and facts.creator out of --context; the task \
                 names them",
            )),
        )
        .with(Setup::Task(review_task("declared openai")))
        .with(Setup::Context(
            r#"{"schemaVersion":1,"facts":{"creator":{"evidence":"declared","provider":"anthropic"}}}"#
                .into(),
        )),
    );
    let task = review_task("declared openai");
    cases.push(
        case(
            "grove review keeps other caller facts and sources",
            Asset::GroveReview,
            "review-impl",
            Expect::Both(selected_review(
                "review-impl",
                "high",
                "producer-k1",
                &Creator::Declared("openai"),
                &CLAUDE,
                serde_json::json!({
                    "schemaVersion": 1,
                    "summary": "caller summary",
                    "facts": {
                        "note": "kept",
                        "reviewedArtifact": { "id": "producer-k1" },
                        "creator": { "evidence": "declared", "provider": "openai" },
                    },
                    "sources": [
                        { "name": "caller-source", "version": "1" },
                        { "name": TASK, "bytes": task.len(), "sha256": "<sha256>" },
                    ],
                }),
                Some(receipt("anthropic", REVIEW_VERSION, "review-impl", None)),
            )),
        )
        .with(Setup::Task(task))
        .with(Setup::Context(
            r#"{"schemaVersion":1,"summary":"caller summary","facts":{"note":"kept"},"sources":[{"name":"caller-source","version":"1"}]}"#
                .into(),
        )),
    );
    cases.push(case(
        "grove review refuses a review without a task file",
        Asset::GroveReview,
        "review-impl",
        Expect::Both(task_refusal(
            "task_file_missing",
            "kind \"review-impl\" is a review but has no task file".into(),
            "pass --task-file for the review, or use the generic review selector with explicit \
             caller facts",
        )),
    ));
}

fn generic_review_cases(cases: &mut Vec<Case>) {
    let feature = recording_receipt(RECORDINGS[3]);
    let external = serde_json::json!({ "id": "external" });
    let recorded_creator = |provider: &str, receipt: &str| {
        format!(
            r#"{{"evidence":"recorded","provider":{},"receipt":{receipt}}}"#,
            json(provider)
        )
    };
    for (kind, effort) in [("code-review", "high"), ("architecture-review", "xhigh")] {
        for (creator, text, reviewer) in [
            (
                Creator::Recorded(feature.clone(), "your-provider"),
                recorded_creator("your-provider", &recorded(&feature)),
                &OTHER,
            ),
            (
                Creator::Declared("your-other-provider"),
                r#"{"evidence":"declared","provider":"your-other-provider"}"#.to_owned(),
                &AGENT,
            ),
        ] {
            let context = caller_facts(external.clone(), Some(&text));
            let name = format!(
                "review {kind} {}",
                if matches!(creator, Creator::Declared(_)) {
                    "declared"
                } else {
                    "recorded"
                }
            );
            cases.push(
                case(
                    name,
                    Asset::Review,
                    kind,
                    Expect::Both(selected_review(
                        kind,
                        effort,
                        "external",
                        &creator,
                        reviewer,
                        serde_json::from_str(&context).unwrap(),
                        None,
                    )),
                )
                .with(Setup::Context(context)),
            );
        }
    }

    // JSON transport can reorder a receipt object, and a number equal to 1 is
    // the schema version in both runtimes.
    let value = receipt_value(&feature);
    let mut reordered = serde_json::Map::new();
    for key in [
        "kind",
        "provider",
        "attemptId",
        "policyVersion",
        "schemaVersion",
    ] {
        reordered.insert(key.into(), value[key].clone());
    }
    let reordered = serde_json::Value::Object(reordered).to_string();
    let float = recorded(&feature).replace("\"schemaVersion\":1", "\"schemaVersion\":1.0");
    for (name, receipt_text) in [("reordered", reordered), ("with a float version", float)] {
        let context = caller_facts(
            external.clone(),
            Some(&recorded_creator("your-provider", &receipt_text)),
        );
        let mut delivered_context: serde_json::Value = serde_json::from_str(&context).unwrap();
        // The notice carries the caller's document as each runtime parsed it.
        delivered_context["facts"]["creator"]["receipt"] = value.clone();
        cases.push(
            case(
                format!("review accepts a receipt {name}"),
                Asset::Review,
                "code-review",
                Expect::Both(selected_review(
                    "code-review",
                    "high",
                    "external",
                    &Creator::Recorded(feature.clone(), "your-provider"),
                    &OTHER,
                    delivered_context,
                    None,
                )),
            )
            .with(Setup::Context(context)),
        );
    }

    let receipt_text = recorded(&feature);
    for (name, creator) in [
        (
            "a provider that is not the receipt's",
            recorded_creator("your-other-provider", &receipt_text),
        ),
        (
            "an extra member",
            format!(
                r#"{{"evidence":"recorded","provider":"your-provider","receipt":{receipt_text},"extra":true}}"#
            ),
        ),
        (
            "a receipt beside a declaration",
            r#"{"evidence":"declared","provider":"your-provider","receipt":{}}"#.into(),
        ),
        ("null", "null".into()),
        ("an array", "[]".into()),
        (
            "an unknown evidence form",
            r#"{"evidence":"run","provider":"your-provider"}"#.into(),
        ),
        (
            "an empty receipt",
            r#"{"evidence":"recorded","provider":"your-provider","receipt":{}}"#.into(),
        ),
        (
            "a blank provider U+3000",
            r#"{"evidence":"declared","provider":"\u3000"}"#.into(),
        ),
        (
            "a blank provider U+FEFF",
            r#"{"evidence":"declared","provider":"\ufeff"}"#.into(),
        ),
        (
            "a schema version of 2.0",
            recorded_creator(
                "your-provider",
                &receipt_text.replace("\"schemaVersion\":1", "\"schemaVersion\":2.0"),
            ),
        ),
        (
            "a receipt with an unknown member",
            recorded_creator(
                "your-provider",
                &receipt_text.replace("\"kind\":\"feature\"", "\"kind\":\"feature\",\"extra\":1"),
            ),
        ),
        (
            "a null receipt",
            r#"{"evidence":"recorded","provider":"your-provider","receipt":null}"#.into(),
        ),
        (
            "recorded evidence without a receipt",
            r#"{"evidence":"recorded","provider":"your-provider"}"#.into(),
        ),
        (
            "a null task",
            recorded_creator(
                "your-provider",
                &receipt_text.replace(
                    "\"kind\":\"feature\"",
                    "\"kind\":\"feature\",\"taskId\":null",
                ),
            ),
        ),
    ] {
        let context = caller_facts(external.clone(), Some(&creator));
        cases.push(
            case(
                format!("review refuses creator {name}"),
                Asset::Review,
                "code-review",
                Expect::Both(creator_invalid()),
            )
            .with(Setup::Context(context)),
        );
    }

    // The reviewed artifact is a nonblank label.
    for (name, artifact) in [
        ("no artifact", None),
        (
            "a blank id U+FEFF",
            Some(serde_json::json!({ "id": "\u{feff}" })),
        ),
        (
            "a blank id U+2028",
            Some(serde_json::json!({ "id": "\u{2028}" })),
        ),
        (
            "an id that is a number",
            Some(serde_json::json!({ "id": 7 })),
        ),
    ] {
        let context = match artifact {
            Some(artifact) => caller_facts(
                artifact,
                Some(r#"{"evidence":"declared","provider":"your-provider"}"#),
            ),
            None => r#"{"schemaVersion":1,"facts":{"creator":{"evidence":"declared","provider":"your-provider"}}}"#.into(),
        };
        cases.push(
            case(
                format!("review refuses {name}"),
                Asset::Review,
                "code-review",
                Expect::Both(artifact_missing("code-review")),
            )
            .with(Setup::Context(context)),
        );
    }
    let context = caller_facts(
        serde_json::json!({ "id": "\u{85}" }),
        Some(r#"{"evidence":"declared","provider":"your-provider"}"#),
    );
    let creator = Creator::Declared("your-provider");
    cases.push(
        case(
            "review accepts an id U+0085",
            Asset::Review,
            "code-review",
            Expect::Both(selected_review(
                "code-review",
                "high",
                "\u{85}",
                &creator,
                &OTHER,
                serde_json::from_str(&context).unwrap(),
                None,
            )),
        )
        .with(Setup::Context(context)),
    );
    cases.push(
        case(
            "review refuses a review without a creator",
            Asset::Review,
            "code-review",
            Expect::Both(owner_refusal(
                "creator_missing",
                Some("creator of \"external\" is unknown".into()),
                "have the owner supply facts.creator with evidence declared and the original \
                 provider",
            )),
        )
        .with(Setup::Context(caller_facts(external.clone(), None))),
    );
    cases.push(case(
        "review refuses a review without facts",
        Asset::Review,
        "code-review",
        Expect::Both(artifact_missing("code-review")),
    ));
    cases.push(case(
        "review routes a nonreview kind",
        Asset::Review,
        "feature",
        Expect::Both(Outcome::Selected {
            argv: strings(&[
                "my-agent-wrapper",
                "--model",
                "your-model",
                "--effort",
                "high",
                ORIGINAL,
            ]),
            provider: "your-provider".into(),
            model: "your-model".into(),
            effort: "high".into(),
            reason:
                "routes[\"feature\"] gives my-agent-wrapper with model your-model at effort high"
                    .into(),
            context: Some(serde_json::json!({ "schemaVersion": 1 })),
            receipt: None,
        }),
    ));
}

fn gateway_cases(cases: &mut Vec<Case>) {
    let external = serde_json::json!({ "id": "external" });
    let declared =
        |provider: &str| format!(r#"{{"evidence":"declared","provider":{}}}"#, json(provider));
    let openai = Creator::Declared("openai");
    for (name, kind, context, outcome) in [
        (
            "a gateway keeps its origin",
            "review-impl",
            caller_facts(external.clone(), Some(&declared("openai"))),
            owner_refusal(
                "same_origin",
                Some(format!(
                    "{}; my-codex-wrapper has the same origin \"openai\". A gateway keeps the \
                     model's origin",
                    openai.description()
                )),
                "map reviews[\"review-impl\"][\"openai\"] to a command of another origin",
            ),
        ),
        (
            "a review without a creator",
            "review-impl",
            caller_facts(external.clone(), None),
            owner_refusal(
                "creator_missing",
                Some("creator of \"external\" is unknown".into()),
                "have the owner supply facts.creator with evidence declared and the original \
                 provider",
            ),
        ),
        (
            "a review without an artifact",
            "review-impl",
            format!(
                r#"{{"schemaVersion":1,"facts":{{"creator":{}}}}}"#,
                declared("anthropic")
            ),
            artifact_missing("review-impl"),
        ),
        (
            "an unknown evidence form",
            "review-impl",
            caller_facts(
                external.clone(),
                Some(r#"{"evidence":"run","provider":"anthropic"}"#),
            ),
            creator_invalid(),
        ),
        (
            "an empty receipt",
            "review-impl",
            caller_facts(
                external.clone(),
                Some(r#"{"evidence":"recorded","provider":"anthropic","receipt":{}}"#),
            ),
            creator_invalid(),
        ),
        (
            "review facts for a nonreview kind",
            "impl",
            caller_facts(external.clone(), Some(&declared("anthropic"))),
            owner_refusal(
                "review_kind_unlisted",
                Some("kind \"impl\" is not listed as a review but carries review facts".into()),
                "list the kind in reviews or remove the review facts for a nonreview task",
            ),
        ),
    ] {
        cases.push(
            case(
                format!("gateway refuses {name}"),
                Asset::Gateway,
                kind,
                Expect::Both(outcome),
            )
            .with(Setup::Context(context)),
        );
    }
    let context = caller_facts(external, Some(&declared("anthropic")));
    cases.push(
        case(
            "gateway reviews another origin",
            Asset::Gateway,
            "review-impl",
            Expect::Both(selected_review(
                "review-impl",
                "high",
                "external",
                &Creator::Declared("anthropic"),
                &GATEWAY,
                serde_json::from_str(&context).unwrap(),
                None,
            )),
        )
        .with(Setup::Context(context)),
    );
}

fn provenance_cases(cases: &mut Vec<Case>) {
    // The wrapper keeps the owner's loader, labels and literal arguments,
    // including `=` in the program and an empty argument.
    let task = b"measured task".to_vec();
    cases.push(
        case(
            "provenance keeps the owner's selection, context and arguments",
            Asset::Provenance,
            "impl",
            Expect::Both(Outcome::Selected {
                argv: strings(&["agent=fixture", "--flag", "", ORIGINAL]),
                provider: "gateway-origin".into(),
                model: "anthropic-looking-model".into(),
                effort: "none".into(),
                reason: "owner route".into(),
                context: Some(serde_json::json!({
                    "schemaVersion": 1,
                    "facts": { "marker": "measured task" },
                    "sources": [{ "name": TASK, "bytes": task.len(), "sha256": "<sha256>" }],
                })),
                receipt: Some(receipt(
                    "gateway-origin",
                    "original-owner-version",
                    "impl",
                    Some("producer-k7"),
                )),
            }),
        )
        .with(Setup::Task(task))
        .with(Setup::TaskId("producer-k7".into()))
        .with(owner(
            serde_json::json!({
                "status": "selected", "program": "agent=fixture", "args": ["--flag", "", "<prompt>"],
                "provider": "gateway-origin", "model": "anthropic-looking-model", "effort": "none",
                "reason": "owner route",
            }),
            "original-owner-version",
        )),
    );

    // Each recorded selection, whose receipt a loader case reads.
    for (index, recording) in RECORDINGS.iter().enumerate() {
        let (provider, version, kind, task) = *recording;
        let mut entry = case(
            format!("provenance records recording {index}"),
            Asset::Provenance,
            kind,
            Expect::Both(wrapped(provider, recording_receipt(*recording))),
        )
        .with(owner(owner_selection(provider.into()), version));
        if let Some(task) = task {
            entry = entry.with(Setup::TaskId(task.into()));
        }
        cases.push(entry);
    }

    // The receipt bound counts UTF-8 bytes: a receipt of exactly 8192 bytes
    // records, and one byte more refuses, as do 4097 two-byte characters.
    let base = recorded(&receipt("", "v", "impl", None)).len();
    let fits = "x".repeat(8192 - base);
    cases.push(
        case(
            "provenance records a receipt of 8192 bytes",
            Asset::Provenance,
            "impl",
            Expect::Both(wrapped(&fits, receipt(&fits, "v", "impl", None))),
        )
        .with(owner(owner_selection(fits.clone().into()), "v")),
    );
    for (name, provider) in [
        ("8193 bytes", "x".repeat(8193 - base)),
        (
            "two-byte characters beyond 8192 bytes",
            "\u{e9}".repeat(4097),
        ),
    ] {
        cases.push(
            case(
                format!("provenance refuses a receipt of {name}"),
                Asset::Provenance,
                "impl",
                Expect::Both(provenance_invalid()),
            )
            .with(owner(owner_selection(provider.into()), "v")),
        );
    }

    // Labels.
    let mut labels: Vec<(String, serde_json::Value)> = vec![
        ("an empty provider".into(), "".into()),
        ("a provider with a line feed".into(), "open\nai".into()),
        ("a provider with a NUL".into(), "open\0ai".into()),
        ("a provider that is a number".into(), 5.into()),
        ("a null provider".into(), serde_json::Value::Null),
    ];
    for character in ['\u{feff}', '\u{3000}', '\u{2028}', '\u{a0}', '\u{9}'] {
        labels.push((
            format!("a blank provider U+{:04X}", character as u32),
            character.to_string().into(),
        ));
    }
    for (name, provider) in labels {
        cases.push(
            case(
                format!("provenance refuses {name}"),
                Asset::Provenance,
                "impl",
                Expect::Both(provenance_invalid()),
            )
            .with(owner(owner_selection(provider), "v")),
        );
    }
    for (name, selection) in [
        ("a string", serde_json::json!("x")),
        ("an array", serde_json::json!([])),
        ("a number", serde_json::json!(5)),
    ] {
        cases.push(
            case(
                format!("provenance refuses an owner result that is {name}"),
                Asset::Provenance,
                "impl",
                Expect::Both(provenance_invalid()),
            )
            .with(owner(selection, "v")),
        );
    }
    let mut missing = owner_selection("openai".into());
    missing.as_object_mut().unwrap().remove("provider");
    cases.push(
        case(
            "provenance refuses a missing provider",
            Asset::Provenance,
            "impl",
            Expect::Both(provenance_invalid()),
        )
        .with(owner(missing, "v")),
    );
    // U+180E is not whitespace for either runtime. Ravel of either major
    // refuses a provider of U+0085 alone by Rust's whitespace, after the
    // wrapper, so the declared cases class U+0085 instead.
    let provider = "\u{180e}".to_owned();
    cases.push(
        case(
            "provenance records the label U+180E",
            Asset::Provenance,
            "impl",
            Expect::Both(wrapped(&provider, receipt(&provider, "v", "impl", None))),
        )
        .with(owner(owner_selection(provider.clone().into()), "v")),
    );

    // Commands. A provider check comes first.
    let command = |program: serde_json::Value, args: serde_json::Value| {
        let mut selection = owner_selection("openai".into());
        selection["program"] = program;
        selection["args"] = args;
        selection
    };
    for (name, selection) in [
        (
            "an empty program",
            command("".into(), serde_json::json!(["<prompt>"])),
        ),
        (
            "a blank program U+FEFF",
            command("\u{feff}".into(), serde_json::json!(["<prompt>"])),
        ),
        (
            "a blank program U+3000",
            command("\u{3000}".into(), serde_json::json!(["<prompt>"])),
        ),
        (
            "a NUL in the program",
            command("my-agent\0wrapper".into(), serde_json::json!(["<prompt>"])),
        ),
        (
            "a program that is a number",
            command(5.into(), serde_json::json!(["<prompt>"])),
        ),
        (
            "arguments that are a string",
            command("my-agent-wrapper".into(), "<prompt>".into()),
        ),
        (
            "an argument that is a number",
            command(
                "my-agent-wrapper".into(),
                serde_json::json!([5, "<prompt>"]),
            ),
        ),
        (
            "a NUL in an argument",
            command(
                "my-agent-wrapper".into(),
                serde_json::json!(["a\0b", "<prompt>"]),
            ),
        ),
    ] {
        cases.push(
            case(
                format!("provenance refuses {name}"),
                Asset::Provenance,
                "impl",
                Expect::Both(command_invalid()),
            )
            .with(owner(selection, "v")),
        );
    }
    let mut both = command("".into(), serde_json::json!(["<prompt>"]));
    both["provider"] = "".into();
    cases.push(
        case(
            "provenance checks the provider before the command",
            Asset::Provenance,
            "impl",
            Expect::Both(provenance_invalid()),
        )
        .with(owner(both, "v")),
    );
    cases.push(
        case(
            "provenance passes a refusal through",
            Asset::Provenance,
            "impl",
            Expect::Both(owner_refusal(
                "owner_declined",
                Some("the owner declined".into()),
                "ask the owner",
            )),
        )
        .with(owner(
            serde_json::json!({ "status": "refused", "code": "owner_declined",
                "message": "the owner declined", "remedy": "ask the owner" }),
            "v",
        )),
    );
}

/// Each receipt that a loader case accepts is the receipt of a wrapper case,
/// with the fixed attempt. With the wrapper cases, this is the check in both
/// directions: either wrapper writes those bytes, and either loader reads them.
#[test]
fn every_accepted_receipt_is_a_wrapper_receipt() {
    let cases = cases();
    let written: Vec<String> = cases
        .iter()
        .filter(|case| case.asset == Asset::Provenance)
        .filter_map(|case| match &case.expect {
            Expect::Both(Outcome::Selected {
                receipt: Some(receipt),
                ..
            }) => Some(recorded(receipt)),
            _ => None,
        })
        .collect();
    let mut read = 0;
    for case in &cases {
        let Expect::Both(Outcome::Selected {
            context: Some(context),
            ..
        }) = &case.expect
        else {
            continue;
        };
        if case.asset != Asset::GroveReview {
            continue;
        }
        let Some(body) = case.setup.iter().find_map(|setup| match setup {
            Setup::Task(body) => Some(String::from_utf8(body.clone()).unwrap()),
            _ => None,
        }) else {
            continue;
        };
        let Some(line) = body.lines().find_map(|line| {
            line.trim_end_matches('\r')
                .strip_prefix("**Creator:** recorded ")
        }) else {
            continue;
        };
        assert_eq!(context["facts"]["creator"]["evidence"], "recorded");
        assert!(
            written.iter().any(|receipt| receipt == line),
            "{}: its receipt is no wrapper case's",
            case.name
        );
        read += 1;
    }
    assert!(read >= RECORDINGS.len() - 1, "{read} recorded loader cases");
}

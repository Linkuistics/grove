//! Grove's TypeScript review and provenance assets through the independent
//! installed Ravel 1.1 worker. The static example and the sample share the case
//! table of `ravel_asset_cases` with their Rhai counterparts.
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Output;

mod support;

const PROMPT: &str = "literal prompt\nwith 'quotes', $variables and --flags";

struct Fixture {
    temp: tempfile::TempDir,
    pair: support::ravel::InstalledRavel,
    policy: PathBuf,
    capture: PathBuf,
}

/// The review adapter, the review examples and the provenance wrapper are
/// TypeScript only until grove-rhai-review-assets-k96 ports them. The static
/// example and the sample have their Rhai checks in `ravel_asset_cases`.
fn review_assets_unported() -> bool {
    let unported = support::runtime() == support::ravel::Runtime::V2_0;
    if unported {
        eprintln!("not run: the Rhai review assets arrive with grove-rhai-review-assets-k96");
    }
    unported
}

impl Fixture {
    fn new(asset: &str) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let source =
            std::env::var_os("GROVE_TEST_RAVEL").expect("explicit installed Ravel required");
        let pair = support::ravel::InstalledRavel::copy_from(
            Path::new(&source),
            &temp.path().join("independent-ravel"),
        );
        let capture = temp.path().join("arguments");
        let script = format!(
            "#!/bin/sh\nprintf '%s\\0' \"${{0##*/}}\" \"$@\" > '{}'\n'{}' end-session grove-continue\n",
            capture.display(),
            pair.executable().display(),
        );
        for name in [
            "my-codex-wrapper",
            "my-claude-wrapper",
            "my-agent-wrapper",
            "my-other-agent-wrapper",
            "codex",
            "claude",
        ] {
            let file = pair.bin().join(name);
            fs::write(&file, &script).unwrap();
            fs::set_permissions(file, fs::Permissions::from_mode(0o755)).unwrap();
        }
        // The sample's one-shot route intentionally runs /bin/bash, not a
        // PATH harness. Supply the staged headless writer at its actual seam.
        fs::write(temp.path().join("codex-headless.sh"), &script).unwrap();
        let asset = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/ravel-policy")
            .join(asset);
        assert!(
            asset.is_file(),
            "missing Grove-readable asset: {}",
            asset.display()
        );
        let asset = fs::canonicalize(asset).unwrap();
        let policy = temp.path().join("policy.ts");
        fs::write(
            &policy,
            format!(
                "export {{ policy }} from {};\n",
                serde_json::to_string(&asset).unwrap()
            ),
        )
        .unwrap();
        Self {
            temp,
            pair,
            policy,
            capture,
        }
    }

    fn wrap(&self) {
        let source = fs::read_to_string(&self.policy).unwrap();
        let wrapper =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/ravel-policy/provenance.ts");
        fs::write(&self.policy, format!("import {{ withProvenance }} from {:?};\n{}\nexport const policy = withProvenance(owner);\n",
            wrapper.to_str().unwrap(), source.replace("export { policy } from", "import { policy as owner } from"))).unwrap();
    }

    fn run(&self, kind: &str) -> Output {
        self.run_with(kind, &[])
    }

    fn run_with(&self, kind: &str, extra: &[&str]) -> Output {
        if self.capture.exists() {
            fs::remove_file(&self.capture).unwrap();
        }
        self.pair
            .command()
            .args([
                "start-session",
                "--json",
                "--kind",
                kind,
                "--prompt",
                PROMPT,
                "--config",
            ])
            .arg(&self.policy)
            .args(extra)
            .env("HOME", self.temp.path())
            .current_dir(self.temp.path())
            .output()
            .unwrap()
    }

    fn arguments(&self) -> Vec<String> {
        fs::read(&self.capture)
            .unwrap()
            .split(|byte| *byte == 0)
            .filter(|arg| !arg.is_empty())
            .map(|arg| String::from_utf8(arg.to_vec()).unwrap())
            .collect()
    }
}

// Provider evidence follows the selected label, even when program/model labels
// would suggest another provider. The exec wrapper must preserve literal argv,
// including '=' in the executable and empty arguments.
#[test]
#[ignore = "requires installed Ravel: task ravel:provenance RAVEL=/absolute/path/to/ravel"]
fn provenance_wrapper_delivers_selection_labels_and_preserves_context_and_argv() {
    if review_assets_unported() {
        return;
    }
    let fixture = Fixture::new("examples/grove-static.ts");
    fixture.wrap();
    let wrapper =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/ravel-policy/provenance.ts");
    let evidence = fixture.temp.path().join("evidence.json");
    fs::write(
        &fixture.policy,
        format!(
            r#"import {{ withProvenance }} from {wrapper:?};
export const policy = withProvenance({{
  schemaVersion: 2, version: 'original-owner-version',
  loadContext(request, host) {{
    const read = host.readText(request.taskFile);
    return {{ schemaVersion: 1, facts: {{ marker: read.text }}, sources: [read.source] }};
  }},
  select(request, context) {{
    if (context.facts.marker !== 'measured task') throw new Error('loader lost');
    return {{ status: 'selected', program: 'agent=fixture',
      args: ['--flag', '', request.prompt], provider: 'gateway-origin',
      model: 'anthropic-looking-model', effort: 'none', reason: 'owner route' }};
  }}
}});
"#,
            wrapper = wrapper.to_str().unwrap()
        ),
    )
    .unwrap();
    fs::write(
        fixture.pair.bin().join("agent=fixture"),
        format!(
            "#!/bin/sh\nprintf '%s' \"$GROVE_PROVIDER_EVIDENCE\" > '{}'\nprintf '%s\\0' \"$@\" > '{}'\n'{}' end-session grove-continue\n",
            evidence.display(), fixture.capture.display(), fixture.pair.executable().display()
        ),
    )
    .unwrap();
    fs::set_permissions(
        fixture.pair.bin().join("agent=fixture"),
        fs::Permissions::from_mode(0o755),
    )
    .unwrap();
    let task = fixture.temp.path().join("task.md");
    fs::write(&task, "measured task").unwrap();
    let output = fixture
        .pair
        .command()
        .args([
            "start-session",
            "--kind",
            "impl",
            "--prompt",
            PROMPT,
            "--task-id",
            "producer-k7",
            "--task-file",
        ])
        .arg(&task)
        .arg("--config")
        .arg(&fixture.policy)
        .current_dir(fixture.temp.path())
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"grove-continue");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(evidence).unwrap()).unwrap();
    let attempt = receipt
        .as_object_mut()
        .unwrap()
        .remove("attemptId")
        .unwrap();
    assert_eq!(attempt.as_str().unwrap().len(), 36);
    assert_eq!(
        receipt,
        serde_json::json!({"schemaVersion": 1, "provider": "gateway-origin",
        "policyVersion": "original-owner-version", "kind": "impl", "taskId": "producer-k7"})
    );
    assert_eq!(
        fs::read(&fixture.capture).unwrap(),
        format!("--flag\0\0{PROMPT}\0").as_bytes()
    );
}

#[test]
#[ignore = "requires installed Ravel: task ravel:provenance RAVEL=/absolute/path/to/ravel"]
fn recorded_evidence_is_validated_before_becoming_measured_caller_context() {
    if review_assets_unported() {
        return;
    }
    let fixture = Fixture::new("examples/grove-static.ts");
    fixture.wrap();
    let wrapper =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/ravel-policy/provenance.ts");
    let receipt = serde_json::json!({"schemaVersion": 1,
        "attemptId": "12345678-1234-4123-8123-123456789abc", "provider": "actual-origin",
        "policyVersion": "original", "kind": "impl", "taskId": "child-k3"});
    // Canonical production field order is part of the byte-copy contract.
    let valid = "{\"schemaVersion\":1,\"attemptId\":\"12345678-1234-4123-8123-123456789abc\",\"provider\":\"actual-origin\",\"policyVersion\":\"original\",\"kind\":\"impl\",\"taskId\":\"child-k3\"}";
    let mut invalid = vec![
        "".to_string(),
        "{}".to_string(),
        "[]".to_string(),
        "null".to_string(),
        format!("{valid}\n"),
        format!(" {valid}"),
        valid.replace("\"provider\":", "\"provider\":\"stale\",\"provider\":"),
        valid.replace("actual-origin", &"x".repeat(8192)),
    ];
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
        let mut changed = receipt.clone();
        changed[field] = replacement;
        invalid.push(changed.to_string());
    }
    let record = fixture.temp.path().join("receipt.txt");
    fs::write(&record, valid).unwrap();
    fs::write(&fixture.policy, format!(r#"
import {{ parseEvidence }} from {wrapper:?};
export const policy = {{ schemaVersion: 2, version: 'measured-receipt-consumer',
  loadContext(request, host) {{
    for (const text of {invalid}) if (parseEvidence(text) !== undefined) throw new Error('invalid receipt accepted');
    const read = host.readText({record:?});
    const receipt = parseEvidence(read.text);
    if (!receipt) throw new Error('valid receipt rejected');
    return {{ schemaVersion: 1, facts: {{ creator: {{ evidence: "recorded", provider: receipt.provider, receipt: {{ ...receipt }} }} }}, sources: [read.source] }};
  }},
  select(request, context) {{
    if (context.facts.creator.provider !== 'actual-origin' || context.measured.length !== 1) throw new Error('context lost');
    return {{ status: 'selected', program: 'my-codex-wrapper', args: [request.prompt],
      provider: 'review-origin', model: 'fake', effort: 'none', reason: 'measured caller facts' }};
  }}
}};
"#, wrapper = wrapper.to_str().unwrap(), record = record.to_str().unwrap(),
        invalid = serde_json::to_string(&invalid).unwrap())).unwrap();
    let output = fixture.run("review-impl");
    assert!(output.status.success(), "{output:?}");
    assert_eq!(output.stdout, b"grove-continue");
}

// Dropping provider diversity, accepting malformed markers, or trusting supplied
// review facts must refuse before a fake agent can be launched.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_grove_review_parses_measured_evidence_and_refuses_bad_relationships() {
    if review_assets_unported() {
        return;
    }
    let fixture = Fixture::new("examples/grove-static.ts");
    let adapter = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../assets/ravel-policy/examples/grove-review.ts");
    fs::write(&fixture.policy, format!(r#"
import {{ policy as owner }} from {adapter:?};
export const policy = {{ ...owner,
  async loadContext(request, host) {{
    const context = await owner.loadContext(request, host);
    if ('status' in context) return context;
    if (request.kind.startsWith('review-') && context.sources.length !== 1) throw new Error('task not attributed');
    return context;
  }},
  async select(request, context, host) {{
    if (request.kind.startsWith('review-') && context.measured.length !== 1) throw new Error('task not measured');
    return owner.select(request, context, host);
  }}
}};
"#, adapter = adapter.to_str().unwrap())).unwrap();
    let task = fixture.temp.path().join("review.md");
    let extra = ["--task-file", task.to_str().unwrap()];
    let receipt = r#"{"schemaVersion":1,"attemptId":"12345678-1234-4123-8123-123456789abc","provider":"openai","policyVersion":"old-owner","kind":"art","taskId":"closing-child-k3"}"#;
    for (kind, effort) in [
        ("review-requirements", "xhigh"),
        ("review-design", "xhigh"),
        ("review-planning", "xhigh"),
        ("review-prototype", "medium"),
        ("review-impl", "high"),
    ] {
        for (creator, agent, model) in [
            (
                format!("recorded {receipt}"),
                "my-claude-wrapper",
                "your-claude-model",
            ),
            (
                "declared openai".into(),
                "my-claude-wrapper",
                "your-claude-model",
            ),
            (
                "declared anthropic".into(),
                "my-codex-wrapper",
                "your-codex-model",
            ),
        ] {
            // CRLF is legitimate; node-close request identity differs on purpose.
            fs::write(
                &task,
                format!("# review\r\n**Reviews:** producer-k1\r\n**Creator:** {creator}\r\n"),
            )
            .unwrap();
            let output = fixture.run_with(kind, &extra);
            assert!(output.status.success(), "{kind}: {output:?}");
            assert_eq!(output.stdout, b"grove-continue");
            assert_eq!(
                fixture.arguments(),
                [agent, "--model", model, "--effort", effort, PROMPT]
            );
            assert!(String::from_utf8_lossy(&output.stderr).contains(
                if creator.starts_with("recorded") {
                    "recorded"
                } else {
                    "declared"
                }
            ));
        }
    }
    let mut bad = vec![
        ("# review\n".to_owned(), "reviews_line_missing"),
        ("**Reviews:** producer-k1\n".into(), "creator_line_missing"),
        ("**Reviews:** producer-k01\n**Creator:** declared openai\n".into(), "reviews_line_malformed"),
        ("**Reviews:** producer-k1 \n**Creator:** declared openai\n".into(), "reviews_line_malformed"),
        ("**Reviews:**producer-k1\n**Creator:** declared openai\n".into(), "reviews_line_malformed"),
        ("**Reviews:** producer-k1\n**Reviews:** other-k2\n**Creator:** declared openai\n".into(), "reviews_line_duplicate"),
        ("**Reviews:** producer-k1\n**Creator:** declared openai\n**Creator:** declared anthropic\n".into(), "creator_line_duplicate"),
        ("**Reviews:** producer-k1\n\n**Creator:** declared openai\n".into(), "creator_line_misplaced"),
        ("**Creator:** declared openai\n**Reviews:** producer-k1\n".into(), "creator_line_misplaced"),
        ("**Reviews:** producer-k1\n **Creator:** declared openai\n".into(), "creator_line_missing"),
        ("**Reviews:** producer-k1\n**Creator:**declared openai\n".into(), "creator_line_malformed"),
        ("**Reviews:** producer-k1\n**Creator:** declared \n".into(), "creator_line_malformed"),
        ("**Reviews:** producer-k1\n**Creator:** declared openai\0\n".into(), "creator_line_malformed"),
        ("**Reviews:** producer-k1\n**Creator:** run 12345678-1234-4123-8123-123456789abc\n".into(), "creator_legacy_unsupported"),
        ("**Reviews:** producer-k1\n**Creator:** recorded {}\n".into(), "creator_line_malformed"),
        ("**Reviews:** producer-k1\n**Creator:** declared OPENAI\n".into(), "creator_origin_unlisted"),
        ("**Reviews:** producer-k1\n**Creator:** declared openai \n".into(), "creator_origin_unlisted"),
        ("**Reviews:** producer-k1\n**Creator:** declared __proto__\n".into(), "creator_origin_unlisted"),
        ("**Reviews:** producer-k1\n**Creator:** declared openai\n```\n**Reviews:** other-k2\n```\n".into(), "reviews_line_duplicate"),
    ];
    for invalid in [
        format!(" {receipt}"),
        receipt.replace("\"provider\":", "\"provider\":\"anthropic\",\"provider\":"),
        receipt.replace("openai", &"x".repeat(8192)),
    ] {
        bad.push((
            format!("**Reviews:** producer-k1\n**Creator:** recorded {invalid}\n"),
            "creator_line_malformed",
        ));
    }
    for (body, code) in bad {
        fs::write(&task, &body).unwrap();
        let output = fixture.run_with("review-impl", &extra);
        assert_eq!(output.status.code(), Some(3), "{code}: {output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(code),
            "{code}: {output:?}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("remedy"),
            "{output:?}"
        );
        assert!(output.stdout.is_empty());
        assert!(!fixture.capture.exists());
    }
    // A declaration repairs the refusal and launches the other provider.
    fs::write(
        &task,
        "**Reviews:** producer-k1\n**Creator:** declared openai\n",
    )
    .unwrap();
    assert!(fixture.run_with("review-impl", &extra).status.success());
    let output = fixture.run_with("impl", &extra);
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("review_kind_unlisted"));
    assert!(!fixture.capture.exists());
    // Composition must preserve every nonreview static route after removing
    // the five review entries from that table.
    for (kind, agent, model, effort) in [
        (
            "requirements",
            "my-codex-wrapper",
            "your-codex-model",
            "max",
        ),
        ("design", "my-codex-wrapper", "your-codex-model", "xhigh"),
        ("planning", "my-codex-wrapper", "your-codex-model", "xhigh"),
        (
            "prototype",
            "my-codex-wrapper",
            "your-codex-model",
            "medium",
        ),
        ("impl", "my-codex-wrapper", "your-codex-model", "high"),
        ("research-a", "my-codex-wrapper", "your-codex-model", "high"),
        (
            "research-b",
            "my-claude-wrapper",
            "your-claude-model",
            "high",
        ),
        (
            "combine-research",
            "my-codex-wrapper",
            "your-codex-model",
            "xhigh",
        ),
        ("draft", "my-codex-wrapper", "your-codex-model", "high"),
        (
            "copy-edit",
            "my-claude-wrapper",
            "your-claude-model",
            "medium",
        ),
        ("art", "my-codex-wrapper", "your-codex-model", "medium"),
        ("proof", "my-claude-wrapper", "your-claude-model", "high"),
        ("finish", "my-codex-wrapper", "your-codex-model", "medium"),
        (
            "integrate-review-requirements",
            "my-codex-wrapper",
            "your-codex-model",
            "xhigh",
        ),
        (
            "integrate-review-design",
            "my-codex-wrapper",
            "your-codex-model",
            "xhigh",
        ),
        (
            "integrate-review-planning",
            "my-codex-wrapper",
            "your-codex-model",
            "xhigh",
        ),
        (
            "integrate-review-prototype",
            "my-codex-wrapper",
            "your-codex-model",
            "medium",
        ),
        (
            "integrate-review-impl",
            "my-codex-wrapper",
            "your-codex-model",
            "medium",
        ),
    ] {
        let output = fixture.run(kind);
        assert!(output.status.success(), "{kind}: {output:?}");
        assert_eq!(
            fixture.arguments(),
            [agent, "--model", model, "--effort", effort, PROMPT]
        );
    }
    let output = fixture.run("unlisted");
    assert_eq!(output.status.code(), Some(3));
    assert!(!fixture.capture.exists());
    let supplied = fixture.temp.path().join("context.json");
    fs::write(
        &supplied,
        r#"{"schemaVersion":1,"facts":{"creator":{"evidence":"declared","provider":"anthropic"}}}"#,
    )
    .unwrap();
    let output = fixture.run_with(
        "review-impl",
        &[
            "--task-file",
            task.to_str().unwrap(),
            "--context",
            supplied.to_str().unwrap(),
        ],
    );
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("reviewed_artifact_conflict"));
    assert!(!fixture.capture.exists());
    let output = fixture.run("review-impl");
    assert_eq!(output.status.code(), Some(3));
    assert!(String::from_utf8_lossy(&output.stderr).contains("task_file_missing"));
    assert!(!fixture.capture.exists());
}

#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_review_rule_keeps_gateway_origin_and_validates_generic_caller_facts() {
    if review_assets_unported() {
        return;
    }
    let fixture = Fixture::new("examples/grove-static.ts");
    let selector =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../assets/ravel-policy/examples/review.ts");
    fs::write(&fixture.policy, format!(r#"
import {{ reviewSelector }} from {selector:?};
const route = (request) => ({{ program: 'my-codex-wrapper', args: [request.prompt], provider: 'openai', model: 'gateway', effort: 'high' }});
export const policy = {{ schemaVersion: 2, version: 'gateway-fixture', ...reviewSelector({{
  routes: {{ impl: route }}, reviews: {{ 'review-impl': {{ openai: route, anthropic: route }} }}
}}) }};
"#, selector = selector.to_str().unwrap())).unwrap();
    let context = fixture.temp.path().join("context.json");
    for (facts, kind, code) in [
        (
            serde_json::json!({"reviewedArtifact":{"id":"external"},"creator":{"evidence":"declared","provider":"openai"}}),
            "review-impl",
            "same_origin",
        ),
        (
            serde_json::json!({"reviewedArtifact":{"id":"external"}}),
            "review-impl",
            "creator_missing",
        ),
        (
            serde_json::json!({"creator":{"evidence":"declared","provider":"anthropic"}}),
            "review-impl",
            "reviewed_artifact_missing",
        ),
        (
            serde_json::json!({"reviewedArtifact":{"id":"external"},"creator":{"evidence":"run","provider":"anthropic"}}),
            "review-impl",
            "creator_invalid",
        ),
        (
            serde_json::json!({"reviewedArtifact":{"id":"external"},"creator":{"evidence":"recorded","provider":"anthropic","receipt":{}}}),
            "review-impl",
            "creator_invalid",
        ),
        (
            serde_json::json!({"reviewedArtifact":{"id":"external"},"creator":{"evidence":"declared","provider":"anthropic"}}),
            "impl",
            "review_kind_unlisted",
        ),
    ] {
        fs::write(
            &context,
            serde_json::json!({"schemaVersion":1,"facts":facts}).to_string(),
        )
        .unwrap();
        let output = fixture.run_with(kind, &["--context", context.to_str().unwrap()]);
        assert_eq!(output.status.code(), Some(3), "{code}: {output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(code),
            "{code}: {output:?}"
        );
        assert!(!fixture.capture.exists());
    }
    fs::write(&context, r#"{"schemaVersion":1,"facts":{"reviewedArtifact":{"id":"external"},"creator":{"evidence":"declared","provider":"anthropic"}}}"#).unwrap();
    let output = fixture.run_with("review-impl", &["--context", context.to_str().unwrap()]);
    assert!(output.status.success(), "{output:?}");
    assert_eq!(fixture.arguments(), ["my-codex-wrapper", PROMPT]);
}

#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_generic_review_example_accepts_both_evidence_forms_and_preserves_effort() {
    if review_assets_unported() {
        return;
    }
    let fixture = Fixture::new("examples/review.ts");
    let context = fixture.temp.path().join("context.json");
    let receipt = r#"{"schemaVersion":1,"attemptId":"12345678-1234-4123-8123-123456789abc","provider":"your-provider","policyVersion":"original-owner","kind":"feature"}"#;
    for (kind, effort) in [("code-review", "high"), ("architecture-review", "xhigh")] {
        for (creator, agent, model) in [
            (
                format!(
                    r#"{{"evidence":"recorded","provider":"your-provider","receipt":{receipt}}}"#
                ),
                "my-other-agent-wrapper",
                "your-other-model",
            ),
            (
                r#"{"evidence":"declared","provider":"your-other-provider"}"#.to_owned(),
                "my-agent-wrapper",
                "your-model",
            ),
        ] {
            fs::write(&context, format!(r#"{{"schemaVersion":1,"facts":{{"reviewedArtifact":{{"id":"external"}},"creator":{creator}}}}}"#)).unwrap();
            let output = fixture.run_with(kind, &["--context", context.to_str().unwrap()]);
            assert!(output.status.success(), "{output:?}");
            assert_eq!(
                fixture.arguments(),
                [agent, "--model", model, "--effort", effort, PROMPT]
            );
        }
    }
    for creator in [
        format!(
            r#"{{"evidence":"recorded","provider":"your-other-provider","receipt":{receipt}}}"#
        ),
        format!(
            r#"{{"evidence":"recorded","provider":"your-provider","receipt":{receipt},"extra":true}}"#
        ),
        r#"{"evidence":"declared","provider":"your-provider","receipt":{}}"#.to_owned(),
        "null".to_owned(),
        "[]".to_owned(),
    ] {
        fs::write(&context, format!(r#"{{"schemaVersion":1,"facts":{{"reviewedArtifact":{{"id":"external"}},"creator":{creator}}}}}"#)).unwrap();
        let output = fixture.run_with("code-review", &["--context", context.to_str().unwrap()]);
        assert_eq!(output.status.code(), Some(3), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("creator_invalid"),
            "{output:?}"
        );
        assert!(!fixture.capture.exists());
    }
}

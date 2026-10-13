//! Grove-readable policies through the independent installed Ravel worker.
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
        for name in ["my-codex-wrapper", "my-claude-wrapper", "codex", "claude"] {
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

    fn choice(&self, choice: Option<&str>) {
        let path = self.temp.path().join(".ravel-choice");
        if let Some(text) = choice {
            fs::write(path, text).unwrap();
        } else if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }

    fn selected(&self, kind: &str) -> Vec<String> {
        let output = self.run(kind);
        assert!(output.status.success(), "{kind}: {output:?}");
        assert_eq!(output.stdout, b"grove-continue", "{kind}: {output:?}");
        let arguments = self.arguments();
        let provider = match arguments[0].as_str() {
            "my-codex-wrapper" | "codex" | "codex-headless.sh" => "openai",
            "my-claude-wrapper" | "claude" => "anthropic",
            other => panic!("unexpected fake agent {other}"),
        };
        let notice = String::from_utf8_lossy(&output.stderr)
            .lines()
            .filter_map(|line| serde_json::from_str::<serde_json::Value>(line).ok())
            .find(|value| value.get("selection").is_some())
            .expect("public routing notice on stderr");
        assert_eq!(
            notice["selection"]["provider"], provider,
            "{kind}: {notice}"
        );
        arguments
    }
}

// Removing a kind or routing it to the wrong harness/effort must fail against
// these literal expectations, through the shipped worker rather than a mock.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_static_routes_preserve_all_kinds_and_prompt_arguments() {
    let fixture = Fixture::new("examples/grove-static.ts");
    fixture.wrap();
    for (kind, reviewer, effort) in [
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
    ] {
        let (agent, model) = if reviewer {
            ("my-claude-wrapper", "your-claude-model")
        } else {
            ("my-codex-wrapper", "your-codex-model")
        };
        assert_eq!(
            fixture.selected(kind),
            [agent, "--model", model, "--effort", effort, PROMPT]
        );
    }
    let output = fixture.run("unmapped");
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    assert!(output.stdout.is_empty());
    assert!(!fixture.capture.exists());
}

// The sample must retain its choices, precedence and real harness arguments.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_sample_applies_arrangements_modifiers_and_refusals() {
    let fixture = Fixture::new("sample/policy.ts");
    fixture.wrap();
    for (choice, kind, agent, model, effort) in [
        (None, "impl", "claude", "claude-opus-5-5", "high"),
        (None, "review-design", "codex", "gpt-6.1-sol", "high"),
        (Some("codex-led"), "impl", "codex", "gpt-6.1-sol", "high"),
        (
            Some("claude-led"),
            "review-design",
            "codex",
            "gpt-6.1-sol",
            "xhigh",
        ),
        (
            Some("codex-design-claude-impl"),
            "impl",
            "claude",
            "claude-sonnet-5-5",
            "high",
        ),
        (
            Some("claude-design-codex-impl"),
            "review-prototype",
            "claude",
            "claude-sonnet-5-5",
            "medium",
        ),
        (
            Some("codex-led high-effort"),
            "prototype",
            "codex",
            "gpt-6.1-sol",
            "high",
        ),
        (
            Some("claude-led codex-sol"),
            "review-design",
            "codex",
            "gpt-6.1-sol",
            "high",
        ),
        (
            Some("codex-design-claude-impl high-effort"),
            "copy-edit",
            "claude",
            "claude-sonnet-5-5",
            "medium",
        ),
    ] {
        fixture.choice(choice);
        let args = fixture.selected(kind);
        let expected = if agent == "codex" {
            vec![
                agent,
                "--model",
                model,
                "-c",
                "unused",
                "-c",
                "default_permissions=:danger-full-access",
                "--ask-for-approval",
                "never",
                PROMPT,
            ]
        } else {
            vec![agent, "--model", model, "--effort", effort, PROMPT]
        };
        let expected: Vec<String> = expected.into_iter().map(str::to_owned).collect();
        let mut expected = expected;
        if agent == "codex" {
            expected[4] = format!("model_reasoning_effort={effort}");
        }
        assert_eq!(args, expected, "{choice:?}, {kind}");
    }
    fixture.choice(None);
    assert_eq!(
        fixture.selected("release-notes"),
        ["codex-headless.sh", "gpt-6.1-sol", "medium", PROMPT]
    );
    for (choice, code) in [
        (Some("unknown"), "choice_unoffered"),
        (Some(""), "choice_arrangement"),
        (Some("high-effort"), "choice_arrangement"),
        (Some("codex-led claude-led"), "choice_arrangement"),
        (None, "incomplete_mapping"),
    ] {
        fixture.choice(choice);
        let output = fixture.run(if choice.is_none() { "unmapped" } else { "impl" });
        assert_eq!(output.status.code(), Some(3), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains(code),
            "{output:?}"
        );
        assert!(output.stdout.is_empty());
        assert!(!fixture.capture.exists());
    }
}

// Losing the secondary-workspace --add-dir grant strands shared jj/git objects.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_sample_keeps_secondary_jj_grants_and_refuses_unreadable_stores() {
    let fixture = Fixture::new("sample/policy.ts");
    fixture.wrap();
    let main = fixture.temp.path().join("main-repository");
    fs::create_dir_all(main.join(".jj/repo")).unwrap();
    fs::create_dir_all(fixture.temp.path().join(".jj")).unwrap();
    fs::write(
        fixture.temp.path().join(".jj/repo"),
        "../main-repository/.jj/repo",
    )
    .unwrap();
    for (choice, agent) in [("codex-led", "codex"), ("claude-led", "claude")] {
        fixture.choice(Some(choice));
        let args = fixture.selected("impl");
        let grant = args.iter().position(|arg| arg == "--add-dir").unwrap();
        assert_eq!(args[0], agent);
        assert_eq!(
            args[grant + 1],
            fs::canonicalize(&main).unwrap().to_str().unwrap()
        );
        assert_eq!(args.last().unwrap(), PROMPT);
    }
    fs::write(fixture.temp.path().join(".jj/repo"), "").unwrap();
    let output = fixture.run("impl");
    assert_eq!(output.status.code(), Some(3), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("jj_store_unreadable"),
        "{output:?}"
    );
    assert!(!fixture.capture.exists());
}

// A dropped sample route must fail independently of the static example table.
#[test]
#[ignore = "requires installed Ravel: task ravel:policy RAVEL=/absolute/path/to/ravel"]
fn readable_sample_defaults_cover_every_session_kind() {
    let fixture = Fixture::new("sample/policy.ts");
    fixture.wrap();
    for (kind, agent, effort) in [
        ("requirements", "claude", "max"),
        ("review-requirements", "codex", "high"),
        ("integrate-review-requirements", "claude", "xhigh"),
        ("design", "claude", "xhigh"),
        ("review-design", "codex", "high"),
        ("integrate-review-design", "claude", "xhigh"),
        ("planning", "claude", "xhigh"),
        ("review-planning", "codex", "high"),
        ("integrate-review-planning", "claude", "xhigh"),
        ("prototype", "claude", "medium"),
        ("review-prototype", "codex", "medium"),
        ("integrate-review-prototype", "claude", "medium"),
        ("impl", "claude", "high"),
        ("review-impl", "codex", "high"),
        ("integrate-review-impl", "claude", "medium"),
        ("research-a", "claude", "high"),
        ("research-b", "codex", "high"),
        ("combine-research", "claude", "xhigh"),
        ("draft", "claude", "high"),
        ("copy-edit", "codex", "medium"),
        ("art", "claude", "high"),
        ("proof", "codex", "medium"),
        ("finish", "claude", "medium"),
    ] {
        let args = fixture.selected(kind);
        assert_eq!(args[0], agent, "{kind}");
        assert_eq!(args.last().unwrap(), PROMPT);
        if agent == "codex" {
            assert_eq!(args[2], "gpt-6.1-sol", "{kind}");
            assert_eq!(
                args[4],
                format!("model_reasoning_effort={effort}"),
                "{kind}"
            );
        } else {
            assert_eq!(args[2], "claude-opus-5-5", "{kind}");
            assert_eq!(args[4], effort, "{kind}");
        }
    }
}

// Provider evidence follows the selected label, even when program/model labels
// would suggest another provider. The exec wrapper must preserve literal argv,
// including '=' in the executable and empty arguments.
#[test]
#[ignore = "requires installed Ravel: task ravel:provenance RAVEL=/absolute/path/to/ravel"]
fn provenance_wrapper_delivers_selection_labels_and_preserves_context_and_argv() {
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

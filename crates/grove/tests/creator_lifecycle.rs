//! Grove review conventions through installed Ravel, without a Ravel store or
//! embedded adapter. The policy is a caller-owned fixture; provenance is explicit.
mod support;

use std::fs;
use std::os::unix::process::CommandExt;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    home: PathBuf,
    script: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("work");
        fs::create_dir(&root).unwrap();
        support::init_jj_repo(&root);
        fs::create_dir(root.join(".grove")).unwrap();
        fs::write(root.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
        Self {
            home: temp.path().join("home"),
            script: temp.path().join("session.sh"),
            _temp: temp,
            root,
        }
    }

    fn file(&self, name: &str, body: &str) {
        let path = self.root.join(".grove").join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(path, body).unwrap();
    }

    fn policy(&self, producer_origin: &str) {
        let policy = format!(
            r#"import {{ readFileSync }} from 'node:fs';
export const policy = {{ schemaVersion: 2, version: 'caller-review-fixture', select(request) {{
  let origin = {producer_origin:?};
  if (request.kind === 'review-impl') {{
    const creator = readFileSync(request.taskFile, 'utf8').match(/^\*\*Creator:\*\* declared (.+)$/m)?.[1];
    if (!creator) return {{ status: 'refused', code: 'creator_missing', message: 'declare the original creator', remedy: 'supply caller provenance' }};
    origin = creator === 'origin-a' ? 'origin-b' : 'origin-a';
  }}
  return {{ status: 'selected', program: '/bin/sh', args: [{script:?}, request.prompt, origin],
    provider: origin, model: 'fixture', effort: 'none', reason: 'explicit caller provenance' }};
}} }};
"#,
            script = self.script.to_str().unwrap()
        );
        support::write_policy(&self.home, &policy);
    }

    fn steps(&self, actions: &str) {
        let llm = support::grove_llm();
        let ravel = support::installed_ravel();
        fs::write(
            &self.script,
            format!(
                r#"set -eu
prompt=$1
origin=$2
handle=${{prompt#*'Grove mandate: the leaf selected for this session is `'}}
handle=${{handle%%'`'*}}
llm='{llm}'
settle_without_provenance() {{
  for review in $(grep -rlx --include='[0-9]*.md' "\*\*Reviews:\*\* $1" .grove); do
    case ${{review##*/}} in [0-9]*-DONE-* | [0-9]*-ABANDONED-*) continue ;; esac
    awk '!/^\*\*Creator:\*\*/' "$review" > settled
    cat settled > "$review"
  done
}}
settle_recorded() {{
  test -n "$GROVE_PROVIDER_EVIDENCE"
  for review in $(grep -rlx --include='[0-9]*.md' "\*\*Reviews:\*\* $1" .grove); do
    case ${{review##*/}} in [0-9]*-DONE-* | [0-9]*-ABANDONED-*) continue ;; esac
    awk '!/^\*\*Creator:\*\*/ {{ print; if ($0 ~ /^\*\*Reviews:\*\*/) print "**Creator:** recorded " ENVIRON["GROVE_PROVIDER_EVIDENCE"] }}' "$review" > settled
    cat settled > "$review"
  done
}}
case $handle in
{actions}
*) exit 94 ;;
esac
'{ravel}' end-session grove-continue
"#,
                llm = llm.display(),
                ravel = ravel.display()
            ),
        )
        .unwrap();
    }

    fn drive(&self) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
        for name in support::grove_env_names() {
            command.env_remove(name);
        }
        command
            .env("HOME", &self.home)
            .env("PATH", support::ravel_path())
            .current_dir(&self.root)
            .stdin(Stdio::null());
        // SAFETY: detach this test's driver from the developer's terminal.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{output:?}");
        output
    }

    fn read(&self, name: &str) -> String {
        fs::read_to_string(self.root.join(".grove").join(name)).unwrap()
    }

    fn provenance_policy(&self, origin: &str) {
        let wrapper = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../assets/ravel-policy/provenance.ts");
        support::write_policy(
            &self.home,
            &format!(
                r#"
import {{ withProvenance, parseEvidence }} from {wrapper:?};
export const policy = withProvenance({{
  schemaVersion: 2, version: 'actual-caller-fixture',
  loadContext(request, host) {{
    if (request.kind !== 'review-impl') return {{ schemaVersion: 1 }};
    const read = host.readText(request.taskFile);
    const text = read.text.match(/^\*\*Creator:\*\* recorded (.+)$/m)?.[1];
    const receipt = text === undefined ? undefined : parseEvidence(text);
    if (!receipt) return {{ status: 'refused', code: 'creator_invalid', message: 'missing/invalid evidence', remedy: 'repair the receipt or declare the provider' }};
    return {{ schemaVersion: 1, facts: {{ creator: {{ evidence: "recorded", provider: receipt.provider, receipt: {{ ...receipt }} }} }}, sources: [read.source] }};
  }},
  select(request, context) {{
    const creator = context?.facts?.creator;
    const origin = request.kind === 'review-impl' ? (creator.provider === 'origin-a' ? 'origin-b' : 'origin-a') : {origin:?};
    return {{ status: 'selected', program: '/bin/sh', args: [{script:?}, request.prompt, origin],
      provider: origin, model: 'irrelevant-label', effort: 'none', reason: 'actual selection evidence' }};
  }}
}});
"#,
                wrapper = wrapper.to_str().unwrap(),
                script = self.script.to_str().unwrap()
            ),
        );
    }
}

fn review(creator: &str, producer: &str) -> String {
    format!(
        "# review\n\n**Reviews:** {producer}\n{}",
        if creator.is_empty() {
            String::new()
        } else {
            format!("{creator}\n")
        }
    )
}

#[test]
fn retirement_clears_an_attempt_then_owner_declaration_survives_reordering_and_remapping() {
    let fixture = Fixture::new();
    fixture.file("01-impl--parser-k1.md", "# producer\n");
    fixture.file(
        "02-review-impl--parser-k2.md",
        &review("**Creator:** run stale-attempt", "parser-k1"),
    );
    fixture.policy("origin-a");
    fixture.steps(
        r#"parser-k1)
      "$llm" leaf-retire .grove/01-impl--parser-k1.md
      settle_without_provenance parser-k1
      "$llm" leaf-insert --kind impl parser-k2 groundwork
      touch "$GROVE_LAUNCH_DIR/teardown" ;;
    groundwork-k3) "$llm" leaf-retire .grove/02-impl--groundwork-k3.md ;;
    parser-k2) printf '%s' "$origin" > reviewer-origin; touch "$GROVE_LAUNCH_DIR/teardown" ;;"#,
    );
    fixture.drive();
    assert_eq!(
        fixture.read("03-review-impl--parser-k2.md"),
        review("", "parser-k1")
    );
    fixture.file(
        "03-review-impl--parser-k2.md",
        &review("**Creator:** declared origin-a", "parser-k1"),
    );
    fixture.policy("origin-b");
    fixture.drive();
    assert_eq!(
        fixture.read("03-review-impl--parser-k2.md"),
        review("**Creator:** declared origin-a", "parser-k1")
    );
    assert_eq!(
        fs::read_to_string(fixture.root.join("reviewer-origin")).unwrap(),
        "origin-b"
    );
}

#[test]
fn a_node_cascade_clears_all_live_reviews_of_finished_producers_only() {
    let fixture = Fixture::new();
    fixture.file("01-k1/_parser.md", "# parser\n");
    fixture.file("01-k1/01-k2/_piece.md", "# piece\n");
    fixture.file("01-k1/01-k2/01-impl--work-k3.md", "# work\n");
    for (name, producer) in [
        ("02-review-impl--parser-review-k4.md", "parser-k1"),
        ("03-review-impl--piece-review-k5.md", "piece-k2"),
        ("04-review-impl--child-review-k6.md", "work-k3"),
        ("05-review-impl--unrelated-k7.md", "elsewhere-k99"),
        ("06-DONE-review-impl--finished-k8.md", "parser-k1"),
    ] {
        fixture.file(name, &review("**Creator:** run earlier-attempt", producer));
    }
    fixture.policy("origin-a");
    fixture.steps(
        r#"work-k3)
      "$llm" leaf-retire .grove/01-k1/01-k2/01-impl--work-k3.md
      settle_without_provenance work-k3
      settle_without_provenance piece-k2
      settle_without_provenance parser-k1
      touch "$GROVE_LAUNCH_DIR/teardown" ;;"#,
    );
    fixture.drive();
    for name in [
        "02-review-impl--parser-review-k4.md",
        "03-review-impl--piece-review-k5.md",
        "04-review-impl--child-review-k6.md",
    ] {
        assert!(!fixture.read(name).contains("**Creator:**"));
    }
    for name in [
        "05-review-impl--unrelated-k7.md",
        "06-DONE-review-impl--finished-k8.md",
    ] {
        assert!(fixture
            .read(name)
            .contains("**Creator:** run earlier-attempt"));
    }
    fixture.drive();
    assert!(fixture
        .read("02-review-impl--parser-review-k4.md")
        .contains("**Reviews:** parser-k1"));
}

#[test]
fn a_review_without_provenance_refuses_then_resumes_after_an_owner_declaration() {
    let fixture = Fixture::new();
    fixture.file("01-review-impl--review-k1.md", &review("", "producer-k99"));
    fixture.policy("origin-a");
    fixture.steps("review-k1) printf '%s' \"$origin\" > reviewer-origin; touch \"$GROVE_LAUNCH_DIR/teardown\" ;;");
    let refused = fixture.drive();
    assert!(
        String::from_utf8_lossy(&refused.stderr).contains("creator_missing"),
        "{refused:?}"
    );
    assert!(!fixture.root.join("reviewer-origin").exists());
    fixture.file(
        "01-review-impl--review-k1.md",
        &review("**Creator:** declared origin-a", "producer-k99"),
    );
    fixture.drive();
    assert_eq!(
        fs::read_to_string(fixture.root.join("reviewer-origin")).unwrap(),
        "origin-b"
    );
}

#[test]
fn recorded_finishing_attempt_survives_remapping_and_review_retry() {
    let fixture = Fixture::new();
    fixture.file("01-impl--producer-k1.md", "# producer\n");
    fixture.file(
        "02-review-impl--review-k2.md",
        &review("**Creator:** declared stale", "producer-k1"),
    );
    fixture.provenance_policy("origin-b");
    fixture.steps(r#"producer-k1) printf '%s' "$GROVE_PROVIDER_EVIDENCE" > first-attempt; touch "$GROVE_LAUNCH_DIR/teardown" ;;"#);
    fixture.drive();
    assert!(fixture
        .read("02-review-impl--review-k2.md")
        .contains("declared stale"));
    fixture.provenance_policy("origin-a");
    fixture.steps(
        r#"producer-k1)
      "$llm" leaf-retire .grove/01-impl--producer-k1.md
      settle_recorded producer-k1
      printf '%s' "$GROVE_PROVIDER_EVIDENCE" > finishing-attempt
      touch "$GROVE_LAUNCH_DIR/teardown" ;;
    review-k2) printf '%s' "$origin" >> reviewer-origins; touch "$GROVE_LAUNCH_DIR/teardown" ;;"#,
    );
    fixture.drive();
    let body = fixture.read("02-review-impl--review-k2.md");
    let receipt = body
        .lines()
        .find_map(|line| line.strip_prefix("**Creator:** recorded "))
        .unwrap();
    assert_eq!(
        receipt,
        fs::read_to_string(fixture.root.join("finishing-attempt")).unwrap()
    );
    assert_ne!(
        receipt,
        fs::read_to_string(fixture.root.join("first-attempt")).unwrap()
    );
    let parsed: serde_json::Value = serde_json::from_str(receipt).unwrap();
    assert_eq!(parsed["provider"], "origin-a");
    assert_eq!(parsed["taskId"], "producer-k1");
    fixture.provenance_policy("origin-b");
    fixture.drive();
    fixture.drive();
    assert_eq!(fixture.read("02-review-impl--review-k2.md"), body);
    assert_eq!(
        fs::read_to_string(fixture.root.join("reviewer-origins")).unwrap(),
        "origin-borigin-b"
    );
}

#[test]
fn recorded_node_close_attributes_every_finished_producer_to_closing_child_only() {
    let fixture = Fixture::new();
    fixture.file("01-k1/_producer.md", "# producer\n");
    fixture.file("01-k1/01-k2/_piece.md", "# piece\n");
    fixture.file("01-k1/01-k2/01-impl--child-k3.md", "# child\n");
    for (name, producer) in [
        ("02-review-impl--whole-k4.md", "producer-k1"),
        ("03-review-impl--part-k5.md", "piece-k2"),
        ("04-review-impl--child-k6.md", "child-k3"),
        ("05-review-impl--prefix-k7.md", "producer-k13"),
        ("06-DONE-review-impl--past-k8.md", "producer-k1"),
        ("07-ABANDONED-review-impl--abandoned-k9.md", "producer-k1"),
    ] {
        fixture.file(name, &review("**Creator:** declared earlier", producer));
    }
    fixture.provenance_policy("origin-a");
    fixture.steps(
        r#"child-k3)
      "$llm" leaf-retire .grove/01-k1/01-k2/01-impl--child-k3.md
      settle_recorded child-k3; settle_recorded piece-k2; settle_recorded producer-k1
      touch "$GROVE_LAUNCH_DIR/teardown" ;;"#,
    );
    fixture.drive();
    let mut receipts = Vec::new();
    for name in [
        "02-review-impl--whole-k4.md",
        "03-review-impl--part-k5.md",
        "04-review-impl--child-k6.md",
    ] {
        let body = fixture.read(name);
        let receipt = body
            .lines()
            .find_map(|line| line.strip_prefix("**Creator:** recorded "))
            .unwrap()
            .to_owned();
        let parsed: serde_json::Value = serde_json::from_str(&receipt).unwrap();
        assert_eq!(parsed["taskId"], "child-k3");
        receipts.push(receipt);
    }
    assert!(receipts.windows(2).all(|pair| pair[0] == pair[1]));
    for name in [
        "05-review-impl--prefix-k7.md",
        "06-DONE-review-impl--past-k8.md",
        "07-ABANDONED-review-impl--abandoned-k9.md",
    ] {
        assert!(fixture.read(name).contains("**Creator:** declared earlier"));
    }
}

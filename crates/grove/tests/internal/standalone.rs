use crate::standalone::{execute, Args, Ui};
use std::fs;
use std::path::PathBuf;

struct Fixture {
    dir: tempfile::TempDir,
    config: PathBuf,
    script: PathBuf,
}

impl Fixture {
    fn new(body: &str) -> Self {
        let dir = tempfile::tempdir().unwrap();
        let script = dir.path().join("harness.sh");
        fs::write(&script, body).unwrap();
        let template = format!("/bin/sh '{}' ${{prompt}}", script.display());
        let config = dir.path().join("config.kdl");
        fs::write(&config, format!("config {{\ncommand \"fixture\" {template:?}\nbind \"writer\" \"fixture\"\nroute \"release-notes\" \"writer\"\n}}\n")).unwrap();
        fs::write(dir.path().join(".grove.kdl"), "not valid configuration").unwrap();
        Self {
            dir,
            config,
            script,
        }
    }

    fn args(&self) -> Args {
        Args {
            kind: "release-notes".into(),
            prompt: Some("Produce notes.".into()),
            prompt_file: None,
            input: vec![],
            output: vec![self.dir.path().join("notes.md")],
            runtime_read: vec![self.script.clone()],
            ui: Ui::Inline,
        }
    }

    fn run(&self, args: Args) -> anyhow::Result<()> {
        execute(
            args,
            &self.config,
            std::path::Path::new("/bin/sh"),
            &self.dir.path().join("logs"),
        )
    }
}

/// Optional real-harness probe. Normal tests use deterministic scripts only.
/// The caller supplies their named command's template and explicit runtime files.
#[test]
#[ignore = "requires an explicitly configured native harness and credentials"]
fn native_harness_smoke() {
    let template = std::env::var("GROVE_SMOKE_COMMAND").expect("set GROVE_SMOKE_COMMAND");
    let fixture = Fixture::new("");
    fs::write(&fixture.config, format!("config {{\ncommand \"native\" {template:?}\nbind \"writer\" \"native\"\nroute \"release-notes\" \"writer\"\n}}\n")).unwrap();
    let mut args = fixture.args();
    args.prompt = Some("Write exactly OK followed by a newline to notes.md, then follow the completion instruction. Use no other tools or files.".into());
    args.runtime_read = std::env::var("GROVE_SMOKE_RUNTIME_READ")
        .unwrap_or_default()
        .lines()
        .map(PathBuf::from)
        .collect();
    args.input = std::env::var("GROVE_SMOKE_INPUTS")
        .unwrap_or_default()
        .lines()
        .map(PathBuf::from)
        .collect();
    let helper = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/debug/grove-llm");
    execute(
        args,
        &fixture.config,
        &helper,
        &fixture.dir.path().join("logs"),
    )
    .unwrap();
    assert_eq!(
        fs::read_to_string(fixture.dir.path().join("notes.md")).unwrap(),
        "OK\n"
    );
}

#[test]
fn standalone_stages_inputs_exports_notes_and_never_needs_a_project() {
    let fixture = Fixture::new("set -eu\ntest -z \"${GROVE_SIGNAL_FILE-}\"\ntest -z \"${TMUX-}\"\ncat message.txt > notes.md\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n");
    let input = fixture.dir.path().join("message.txt");
    fs::write(&input, "Verified release notes.\n").unwrap();
    let mut args = fixture.args();
    args.input.push(input);
    fixture.run(args).unwrap();
    assert_eq!(
        fs::read_to_string(fixture.dir.path().join("notes.md")).unwrap(),
        "Verified release notes.\n"
    );
    assert!(!fixture.dir.path().join(".grove").exists());
    assert!(!fixture.dir.path().join(".jj").exists());
}

#[test]
fn failed_or_unacknowledged_harnesses_do_not_publish_outputs() {
    for tail in [
        "exit 0",
        "printf bogus > \"$GROVE_RUN_SIGNAL_FILE\"",
        "printf done > \"$GROVE_RUN_SIGNAL_FILE\"; exit 7",
    ] {
        let fixture = Fixture::new(&format!("printf partial > notes.md\n{tail}\n"));
        assert!(fixture.run(fixture.args()).is_err());
        assert!(!fixture.dir.path().join("notes.md").exists());
    }
}

#[test]
fn missing_output_or_symlink_prevents_publication_of_the_whole_set() {
    for body in ["printf first > notes.md", "ln -s /etc/passwd notes.md"] {
        let fixture = Fixture::new(&format!(
            "{body}\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n"
        ));
        let mut args = fixture.args();
        args.output.push(fixture.dir.path().join("second.md"));
        assert!(fixture.run(args).is_err());
        assert!(!fixture.dir.path().join("notes.md").exists());
    }
}

#[test]
fn existing_output_is_never_overwritten() {
    let fixture =
        Fixture::new("printf changed > notes.md\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n");
    let destination = fixture.dir.path().join("notes.md");
    fs::write(&destination, "preserved").unwrap();
    assert!(fixture.run(fixture.args()).is_err());
    assert_eq!(fs::read_to_string(destination).unwrap(), "preserved");
}

#[test]
#[ignore = "requires public DNS and HTTPS; no credentials or model requests"]
fn native_network_smoke() {
    let fixture = Fixture::new("set -eu\n/usr/bin/curl --silent --show-error --max-time 10 https://chatgpt.com/backend-api/codex/models -o /dev/null\nprintf 'OK\\n' > notes.md\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n");
    fixture.run(fixture.args()).unwrap();
}

#[test]
fn replacing_the_staging_directory_cannot_redirect_export_to_host_files() {
    let fixture = Fixture::new("");
    let outside = fixture.dir.path().join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("notes.md"), "private parent contents").unwrap();
    fs::write(&fixture.script, format!("set -eu\ncd ..\nmv work original\nln -s '{}' work\nprintf done > \"$GROVE_RUN_SIGNAL_FILE\"\n", outside.display())).unwrap();
    assert!(fixture.run(fixture.args()).is_err());
    assert!(!fixture.dir.path().join("notes.md").exists());
}

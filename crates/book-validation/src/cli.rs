use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Component, Path, PathBuf};
use std::sync::Mutex;

use clap::{error::ErrorKind, Parser, ValueEnum};
use serde_json::json;

use crate::{validate, BookSnapshot, Check, Request, Scope, SOURCE_PATHS};

const AFTER_HELP: &str = "Exit status:\n  0  valid\n  1  deterministic findings\n  2  invalid invocation or input load failure\n  3  internal validator failure\n\nJSON output uses a versioned envelope with status, scope, coverage, and diagnostics.\n\nExamples:\n  cargo run --quiet -p book-validation --bin book-check -- --repo . --book docs/ordinal-fs-tree/book --through read-path-k14 --check fragments\n  cargo run --quiet -p book-validation --bin book-check -- --repo . --book docs/ordinal-fs-tree/book --final --check fragments";

#[derive(Debug, Parser)]
#[command(
    name = "book-check",
    about = "Validate the ordinal-fs-tree walkthrough book",
    long_about = "Validate the ordinal-fs-tree walkthrough book without modifying the book or production source. The command is read-only and never fetches network resources.",
    after_help = AFTER_HELP,
    group = clap::ArgGroup::new("scope").required(true).multiple(false).args(["through", "final_"])
)]
struct Cli {
    /// Explicit repository root; relative paths resolve from the process cwd.
    #[arg(long)]
    repo: PathBuf,
    /// Normalized repository-relative book directory.
    #[arg(long)]
    book: PathBuf,
    /// Validate the canonical prefix through this source-owning slice.
    #[arg(long, value_parser = [
        "orientation-k11",
        "name-seam-k12",
        "reference-domain-k13",
        "read-path-k14",
        "mutation-algebra-k15",
        "filesystem-interpreter-k16",
        "syllabus-cli-k17",
    ])]
    through: Option<String>,
    /// Validate the complete fifteen-file corpus with no deferred holes.
    #[arg(long = "final")]
    final_: bool,
    /// Select fragment checks. Only `fragments` is available until `markdown-validation-k9`.
    #[arg(long, value_enum, default_value_t = CheckArg::Fragments)]
    check: CheckArg,
    /// Render deterministic human text or the stable JSON envelope.
    #[arg(long, value_enum, default_value_t = OutputArg::Text)]
    output: OutputArg,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
enum CheckArg {
    Fragments,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum OutputArg {
    Text,
    Json,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunOutput {
    pub stdout: String,
    pub stderr: String,
    pub exit: i32,
}

pub fn run_from<I, T>(arguments: I) -> RunOutput
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let arguments: Vec<OsString> = arguments.into_iter().map(Into::into).collect();
    let wants_json = requests_json(&arguments);
    catch_operation(wants_json, || run_arguments(arguments, wants_json))
}

fn catch_operation<F>(json_output: bool, operation: F) -> RunOutput
where
    F: FnOnce() -> RunOutput,
{
    static PANIC_HOOK_LOCK: Mutex<()> = Mutex::new(());

    let _guard = PANIC_HOOK_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let previous_hook = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let result = panic::catch_unwind(AssertUnwindSafe(operation));
    panic::set_hook(previous_hook);
    match result {
        Ok(output) => output,
        Err(payload) => internal_error(panic_category(payload.as_ref()), json_output),
    }
}

fn run_arguments(arguments: Vec<OsString>, wants_json: bool) -> RunOutput {
    let cli = match Cli::try_parse_from(arguments) {
        Ok(cli) => cli,
        Err(error)
            if matches!(
                error.kind(),
                ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
            ) =>
        {
            return RunOutput {
                stdout: error.to_string(),
                stderr: String::new(),
                exit: 0,
            };
        }
        Err(error) => return command_error("U001", &error.to_string(), wants_json),
    };
    if !normalized_relative(&cli.book) {
        return command_error(
            "U001",
            "--book must be a normalized repository-relative path with no `..` component",
            cli.output == OutputArg::Json,
        );
    }

    let repository = if cli.repo.is_absolute() {
        cli.repo.clone()
    } else {
        match std::env::current_dir() {
            Ok(cwd) => cwd.join(&cli.repo),
            Err(error) => {
                return input_error(
                    LoadFailure::new("<command>", "process cwd", &error),
                    cli.output == OutputArg::Json,
                );
            }
        }
    };
    let snapshot = match load_snapshot(&repository, &cli.book) {
        Ok(snapshot) => snapshot,
        Err(error) => return input_error(error, cli.output == OutputArg::Json),
    };
    let request = Request {
        scope: cli.through.map_or(Scope::Final, Scope::Through),
        check: match cli.check {
            CheckArg::Fragments => Check::Fragments,
        },
    };
    let report = validate(&snapshot, request);
    let exit = if report.valid { 0 } else { 1 };
    let stdout = match cli.output {
        OutputArg::Json => format!(
            "{}\n",
            serde_json::to_string(&report).expect("report is serializable")
        ),
        OutputArg::Text => render_text(&report),
    };
    RunOutput {
        stdout,
        stderr: String::new(),
        exit,
    }
}

fn load_snapshot(repository: &Path, book: &Path) -> Result<BookSnapshot, LoadFailure> {
    let book_root = repository.join(book);
    let entries = fs::read_dir(&book_root)
        .map_err(|error| LoadFailure::new(book, "book directory", &error))?;
    let mut paths = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|error| LoadFailure::new(book, "book entry", &error))?;
        paths.push(entry.path());
    }
    paths.sort_by_key(|path| path.to_string_lossy().replace('\\', "/"));
    let mut book_files = BTreeMap::new();
    for path in paths {
        if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path
                .strip_prefix(repository)
                .expect("book path was joined to repository")
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = fs::read(&path)
                .map_err(|error| LoadFailure::new(&relative, "book file", &error))?;
            book_files.insert(relative, bytes);
        }
    }
    let mut source_files = BTreeMap::new();
    for relative in SOURCE_PATHS {
        let bytes = fs::read(repository.join(relative))
            .map_err(|error| LoadFailure::new(relative, "ledger source", &error))?;
        source_files.insert((*relative).into(), bytes);
    }
    Ok(BookSnapshot {
        book_files,
        source_files,
    })
}

fn normalized_relative(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn requests_json(arguments: &[OsString]) -> bool {
    arguments
        .windows(2)
        .any(|pair| pair[0] == "--output" && pair[1] == "json")
        || arguments.iter().any(|argument| argument == "--output=json")
}

fn command_error(code: &str, message: &str, json_output: bool) -> RunOutput {
    let message = message.split_whitespace().collect::<Vec<_>>().join(" ");
    if json_output {
        let envelope = json!({
            "schema": 1,
            "status": "invocation-error",
            "valid": false,
            "scope": null,
            "coverage": null,
            "diagnostics": [{
                "code": code,
                "phase": "parse",
                "message": message,
                "primary": { "path": "<command>", "byte": 0, "line": null, "column": null },
                "fragment_id": null,
                "root_id": null,
                "source": null,
                "related": [],
                "remedy": "run `book-check --help` for accepted arguments"
            }]
        });
        RunOutput {
            stdout: format!("{envelope}\n"),
            stderr: String::new(),
            exit: 2,
        }
    } else {
        RunOutput {
            stdout: String::new(),
            stderr: format!(
                "{code} <command>: {message}\n  remedy: run `book-check --help` for accepted arguments\n"
            ),
            exit: 2,
        }
    }
}

#[derive(Debug)]
struct LoadFailure {
    path: String,
    subject: &'static str,
    category: &'static str,
}

impl LoadFailure {
    fn new(path: impl AsRef<Path>, subject: &'static str, error: &std::io::Error) -> Self {
        Self {
            path: path.as_ref().to_string_lossy().replace('\\', "/"),
            subject,
            category: io_category(error.kind()),
        }
    }

    fn message(&self) -> String {
        format!(
            "cannot load {} `{}`: {}",
            self.subject, self.path, self.category
        )
    }
}

fn io_category(kind: std::io::ErrorKind) -> &'static str {
    match kind {
        std::io::ErrorKind::NotFound => "not-found",
        std::io::ErrorKind::PermissionDenied => "permission-denied",
        std::io::ErrorKind::InvalidData => "invalid-data",
        std::io::ErrorKind::InvalidInput => "invalid-input",
        std::io::ErrorKind::UnexpectedEof => "unexpected-eof",
        std::io::ErrorKind::AlreadyExists => "already-exists",
        std::io::ErrorKind::IsADirectory => "is-a-directory",
        std::io::ErrorKind::NotADirectory => "not-a-directory",
        _ => "io-failure",
    }
}

fn input_error(error: LoadFailure, json_output: bool) -> RunOutput {
    non_validation_error(
        "U002",
        "invocation-error",
        "parse",
        &error.message(),
        &error.path,
        "restore read access to the required repository input",
        2,
        json_output,
    )
}

fn internal_error(category: &str, json_output: bool) -> RunOutput {
    non_validation_error(
        "I001",
        "internal-error",
        "bytes",
        &format!("internal invariant failed: category `{category}`; retry has no defined remedy"),
        "<internal>",
        "retry has no defined remedy; report the stable internal category",
        3,
        json_output,
    )
}

#[allow(clippy::too_many_arguments)]
fn non_validation_error(
    code: &str,
    status: &str,
    phase: &str,
    message: &str,
    path: &str,
    remedy: &str,
    exit: i32,
    json_output: bool,
) -> RunOutput {
    if json_output {
        let envelope = json!({
            "schema": 1,
            "status": status,
            "valid": false,
            "scope": null,
            "coverage": null,
            "diagnostics": [{
                "code": code,
                "phase": phase,
                "message": message,
                "primary": { "path": path, "byte": 0, "line": null, "column": null },
                "fragment_id": null,
                "root_id": null,
                "source": null,
                "related": [],
                "remedy": remedy
            }]
        });
        RunOutput {
            stdout: format!("{envelope}\n"),
            stderr: String::new(),
            exit,
        }
    } else {
        RunOutput {
            stdout: String::new(),
            stderr: format!("{code} {path}: {message}\n  remedy: {remedy}\n"),
            exit,
        }
    }
}

fn panic_category(payload: &(dyn std::any::Any + Send)) -> &'static str {
    if payload.is::<&'static str>() || payload.is::<String>() {
        "validator-panic"
    } else {
        "unknown-panic"
    }
}

fn render_text(report: &crate::ValidationReport) -> String {
    if report.valid {
        return format!(
            "valid: {} files, {} resolved lines, {} deferred lines, final={}\n",
            report.coverage.files,
            report.coverage.resolved_lines,
            report.coverage.deferred_lines,
            report.coverage.final_
        );
    }
    let mut output = String::new();
    for diagnostic in &report.diagnostics {
        if diagnostic.primary.line == 0 || diagnostic.primary.column == 0 {
            let _ = writeln!(
                output,
                "{} {}: {}",
                diagnostic.code, diagnostic.primary.path, diagnostic.message
            );
        } else {
            let _ = writeln!(
                output,
                "{} {}:{}:{}: {}",
                diagnostic.code,
                diagnostic.primary.path,
                diagnostic.primary.line,
                diagnostic.primary.column,
                diagnostic.message
            );
        }
        for related in &diagnostic.related {
            let _ = writeln!(
                output,
                "  related {} {}:{}:{}",
                related.label, related.path, related.line, related.column
            );
        }
        if let Some(remedy) = &diagnostic.remedy {
            let _ = writeln!(output, "  remedy: {remedy}");
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::{catch_operation, internal_error, render_text, run_from};
    use crate::{
        Coverage, Diagnostic, Location, RelatedLocation, ReportStatus, Scope, ValidationReport,
    };

    #[test]
    fn internal_failure_is_exit_three_with_i001_envelope() {
        let output = internal_error("validator-panic", true);
        let value: serde_json::Value = serde_json::from_str(&output.stdout).unwrap();

        assert_eq!(output.exit, 3);
        assert_eq!(value["status"], "internal-error");
        assert_eq!(value["scope"], serde_json::Value::Null);
        assert_eq!(value["coverage"], serde_json::Value::Null);
        assert_eq!(value["diagnostics"][0]["code"], "I001");
        assert_eq!(value["diagnostics"][0]["primary"]["path"], "<internal>");
        assert!(value["diagnostics"][0]["message"]
            .as_str()
            .unwrap()
            .contains("retry has no defined remedy"));
    }

    #[test]
    fn caught_panics_use_only_the_i001_envelope() {
        let output = catch_operation(true, || panic!("unstable panic payload"));

        assert_eq!(output.exit, 3);
        assert!(output.stderr.is_empty());
        assert!(!output.stdout.contains("unstable panic payload"));
        assert_eq!(
            serde_json::from_str::<serde_json::Value>(&output.stdout).unwrap()["diagnostics"][0]
                ["code"],
            "I001"
        );
    }

    #[test]
    fn text_and_json_failures_are_byte_identical_across_runs() {
        let text_arguments = ["book-check", "--final", "--unknown"];
        let json_arguments = ["book-check", "--final", "--unknown", "--output", "json"];

        assert_eq!(run_from(text_arguments), run_from(text_arguments));
        assert_eq!(run_from(json_arguments), run_from(json_arguments));
    }

    #[test]
    fn text_rendering_follows_the_exact_location_related_and_remedy_lines() {
        let report = ValidationReport {
            schema: 1,
            status: ReportStatus::Findings,
            valid: false,
            scope: Scope::Final,
            coverage: Coverage::default(),
            diagnostics: vec![Diagnostic {
                code: "F001".into(),
                phase: "identity".into(),
                message: "duplicate ID".into(),
                primary: Location {
                    path: "docs/ordinal-fs-tree/book/source-index.md".into(),
                    byte: 10,
                    line: 2,
                    column: 1,
                },
                fragment_id: Some("same".into()),
                root_id: None,
                source: None,
                related: vec![RelatedLocation {
                    path: "docs/ordinal-fs-tree/book/source-index.md".into(),
                    byte: 20,
                    line: 4,
                    column: 1,
                    label: "duplicate occurrence".into(),
                }],
                remedy: Some("give every ID a unique name".into()),
            }],
        };

        assert_eq!(
            render_text(&report),
            concat!(
                "F001 docs/ordinal-fs-tree/book/source-index.md:2:1: duplicate ID\n",
                "  related duplicate occurrence docs/ordinal-fs-tree/book/source-index.md:4:1\n",
                "  remedy: give every ID a unique name\n",
            )
        );
    }
}

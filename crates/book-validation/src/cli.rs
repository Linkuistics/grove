use std::collections::BTreeMap;
use std::ffi::OsString;
use std::fmt::Write as _;
use std::fs;
use std::path::{Component, Path, PathBuf};

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
    /// Select fragment checks. Markdown checks land in the next validator increment.
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
                return command_error(
                    "U002",
                    &format!("cannot load process cwd: {error}"),
                    cli.output == OutputArg::Json,
                )
            }
        }
    };
    let snapshot = match load_snapshot(&repository, &cli.book) {
        Ok(snapshot) => snapshot,
        Err(message) => return command_error("U002", &message, cli.output == OutputArg::Json),
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

fn load_snapshot(repository: &Path, book: &Path) -> Result<BookSnapshot, String> {
    let book_root = repository.join(book);
    let entries = fs::read_dir(&book_root)
        .map_err(|error| format!("cannot load book directory `{}`: {error}", book.display()))?;
    let mut book_files = BTreeMap::new();
    for entry in entries {
        let entry = entry.map_err(|error| format!("cannot load book entry: {error}"))?;
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "md") {
            let relative = path
                .strip_prefix(repository)
                .expect("book path was joined to repository")
                .to_string_lossy()
                .replace('\\', "/");
            let bytes = fs::read(&path)
                .map_err(|error| format!("cannot load book file `{relative}`: {error}"))?;
            book_files.insert(relative, bytes);
        }
    }
    let mut source_files = BTreeMap::new();
    for relative in SOURCE_PATHS {
        let bytes = fs::read(repository.join(relative))
            .map_err(|error| format!("cannot load ledger source `{relative}`: {error}"))?;
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
                "message": message.trim(),
                "primary": { "path": "<command>", "byte": 0, "line": null, "column": null },
                "fragment_id": null,
                "root_id": null,
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
            stderr: format!("{code} <command>: {}\n", message.trim()),
            exit: 2,
        }
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
        let _ = writeln!(
            output,
            "{} {}:{}:{}: {}",
            diagnostic.code,
            diagnostic.primary.path,
            diagnostic.primary.line,
            diagnostic.primary.column,
            diagnostic.message
        );
        if let Some(remedy) = &diagnostic.remedy {
            let _ = writeln!(output, "  remedy: {remedy}");
        }
    }
    output
}

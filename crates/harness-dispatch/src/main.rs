//! `harness-dispatch` — evaluate an owner's harness-selection policy, then report
//! or run the joint harness/model/effort choice it makes.
//!
//! The contract is `docs/specs/harness-selection-and-execution.md`. This entry
//! point parses the command line, runs the command, and renders its result or
//! refusal; the modules own everything else.

mod argv;
mod authority;
mod choice;
mod cli;
mod frame;
mod inputs;
mod inspect;
mod policy;
mod program;
mod record;
mod refusal;
mod run;
mod run_id;
mod store;
mod worker;

use std::ffi::OsString;
use std::io::Write as _;
use std::process::ExitCode;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::Parser as _;

use crate::cli::{Cli, Command, RecordCommand};
use crate::refusal::{Failure, Refusal, Stage, EXIT_MALFORMED};

fn main() -> ExitCode {
    let arguments: Vec<OsString> = std::env::args_os().collect();
    // Only a command line that did not parse, or whose words are never parsed,
    // is scanned for `--json`, so that it still honours the one-JSON-error
    // contract. Once a command parses, its own flag decides: a `--json` that
    // clap took as the value of `--prompt` or `--task-id` is caller data.
    let scanned = arguments
        .iter()
        .skip(1)
        .any(|argument| argument == "--json");

    let cli = match Cli::try_parse_from(&arguments) {
        Ok(cli) => cli,
        Err(error) => return parse_failure(&error, &arguments, scanned),
    };
    let json = match &cli.command {
        Command::Inspect(args) => args.json,
        Command::Run(args) => args.json,
        Command::Record(record) => match &record.command {
            RecordCommand::Show(args) => args.json,
            RecordCommand::Observe(_) => scanned,
        },
    };
    let result = match &cli.command {
        Command::Inspect(args) => inspect::inspect(args).map(|report| {
            if args.json {
                println!("{}", report.to_json());
            } else {
                eprint!("{}", report.diagnostics().to_text());
                print!("{}", report.to_text());
            }
        }),
        Command::Run(args) => Err(run::run(args)),
        Command::Record(record) => match &record.command {
            RecordCommand::Show(args) => record::show(args)
                .map(|export| {
                    if args.json {
                        println!("{}", export.to_json());
                    } else {
                        print!("{}", export.to_text());
                    }
                })
                .map_err(Failure::from),
            RecordCommand::Observe(_) => {
                Err(cli::unsupported("`record observe`", "record observe").into())
            }
        },
    };
    match result {
        Ok(()) => {
            let _ = std::io::stdout().flush();
            ExitCode::SUCCESS
        }
        Err(failure) => report_failure(&failure, json),
    }
}

/// A command line clap could not parse, as a refusal like any other: a stable
/// code, the stage, clap's message, the argument involved, and a remedy that
/// carries clap's own tip and usage line. Help and version requests, and the
/// help clap shows for a missing subcommand, are not refusals and print as
/// clap renders them.
fn parse_failure(error: &clap::Error, arguments: &[OsString], json: bool) -> ExitCode {
    let informational = matches!(
        error.kind(),
        ErrorKind::DisplayHelp
            | ErrorKind::DisplayVersion
            | ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
    );
    if informational {
        let _ = error.print();
        return ExitCode::from(u8::try_from(error.exit_code()).unwrap_or(EXIT_MALFORMED));
    }
    // Clap renders paragraphs: the message, then any `tip:` lines, the usage
    // line and a pointer to --help.
    let rendered = error.render().to_string();
    let paragraphs: Vec<String> = rendered
        .split("\n\n")
        .map(|paragraph| {
            paragraph
                .lines()
                .map(str::trim)
                .collect::<Vec<_>>()
                .join(" ")
        })
        .filter(|paragraph| !paragraph.is_empty())
        .collect();
    let message = paragraphs
        .first()
        .map_or("the command line cannot be parsed", |message| {
            message.trim_start_matches("error: ")
        });
    let mut remedy: Vec<String> = paragraphs
        .iter()
        .filter(|paragraph| paragraph.starts_with("tip: "))
        .map(|tip| tip.trim_start_matches("tip: ").to_owned())
        .collect();
    if let Some(usage) = paragraphs
        .iter()
        .find(|paragraph| paragraph.starts_with("Usage: "))
    {
        remedy.push(format!("usage: {}", usage.trim_start_matches("Usage: ")));
    }
    remedy.push(format!("see {} --help", command_path(arguments)));
    let refusal = Refusal::new(
        "malformed_input",
        Stage::Cli,
        EXIT_MALFORMED,
        message,
        remedy.join("; "),
    )
    .input(parse_input(error, arguments));
    report_failure(&refusal.into(), json)
}

/// The argument a parse failure is about. Clap names it as it renders it in
/// usage (`--task-file <PATH>`), and the refusal's input is the flag alone.
/// Clap names no argument for invalid UTF-8, so the first argument that is not
/// UTF-8 is found here, by the flag it is the value of when there is one. (With
/// two such arguments, one of them the value of a flag that accepts any bytes,
/// the first is named even if it was not the one refused.) Anything else clap
/// cannot name falls back to the whole command line.
fn parse_input(error: &clap::Error, arguments: &[OsString]) -> String {
    for kind in [ContextKind::InvalidArg, ContextKind::InvalidSubcommand] {
        let named = match error.get(kind) {
            Some(ContextValue::String(argument)) => Some(argument),
            Some(ContextValue::Strings(arguments)) => arguments.first(),
            _ => None,
        };
        if let Some(flag) = named.and_then(|argument| argument.split(' ').next()) {
            return flag.to_owned();
        }
    }
    let words = arguments.get(1..).unwrap_or_default();
    let invalid_utf8 = error.kind() == ErrorKind::InvalidUtf8;
    if let Some(at) = words
        .iter()
        .position(|word| invalid_utf8 && word.to_str().is_none())
    {
        let word = words[at].to_string_lossy();
        let flag = |word: &str| word.starts_with("--").then(|| word.to_owned());
        return match word.split_once('=') {
            Some((name, _)) if name.starts_with("--") => name.to_owned(),
            _ => at
                .checked_sub(1)
                .and_then(|before| words[before].to_str())
                .and_then(flag)
                .unwrap_or_else(|| word.into_owned()),
        };
    }
    words
        .iter()
        .map(|word| word.to_string_lossy())
        .collect::<Vec<_>>()
        .join(" ")
}

/// The subcommand the caller reached, for pointing at its help.
fn command_path(arguments: &[OsString]) -> String {
    let words: Vec<&str> = arguments
        .iter()
        .skip(1)
        .map_while(|word| word.to_str())
        .collect();
    match words.as_slice() {
        ["record", "show", ..] => "harness-dispatch record show".to_owned(),
        [command @ ("inspect" | "run" | "record"), ..] => format!("harness-dispatch {command}"),
        _ => "harness-dispatch".to_owned(),
    }
}

fn report_failure(failure: &Failure, json: bool) -> ExitCode {
    if json {
        eprintln!("{}", failure.to_json());
    } else {
        eprint!("{}", failure.to_text());
    }
    ExitCode::from(failure.refusal.exit)
}

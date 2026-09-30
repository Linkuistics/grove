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
mod refusal;
mod run;
mod worker;

use std::ffi::OsString;
use std::io::Write as _;
use std::process::ExitCode;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::Parser as _;

use crate::cli::{Cli, Command};
use crate::refusal::{Failure, Refusal, Stage, EXIT_MALFORMED};

fn main() -> ExitCode {
    let arguments: Vec<OsString> = std::env::args_os().collect();
    // Decided before parsing, so a malformed command line still honours the
    // one-JSON-error contract when the caller asked for JSON.
    let json = arguments
        .iter()
        .skip(1)
        .any(|argument| argument == "--json");

    let cli = match Cli::try_parse_from(&arguments) {
        Ok(cli) => cli,
        Err(error) => return parse_failure(&error, json),
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
        Command::Record(_) => Err(cli::unsupported("`record`", "record").into()),
    };
    match result {
        Ok(()) => {
            let _ = std::io::stdout().flush();
            ExitCode::SUCCESS
        }
        Err(failure) => report_failure(&failure, json),
    }
}

fn parse_failure(error: &clap::Error, json: bool) -> ExitCode {
    let informational = matches!(
        error.kind(),
        ErrorKind::DisplayHelp | ErrorKind::DisplayVersion
    );
    if informational || !json {
        let _ = error.print();
        return ExitCode::from(u8::try_from(error.exit_code()).unwrap_or(EXIT_MALFORMED));
    }
    // Clap's message runs to the first blank line; usage and hints follow it.
    let rendered = error.render().to_string();
    let message = rendered
        .lines()
        .take_while(|line| !line.trim().is_empty())
        .map(str::trim)
        .collect::<Vec<_>>()
        .join(" ");
    let mut refusal = Refusal::new(
        "malformed_input",
        Stage::Cli,
        EXIT_MALFORMED,
        message.trim_start_matches("error: "),
        "run harness-dispatch inspect --help or harness-dispatch run --help for the accepted inputs",
    );
    // Clap names the argument as it renders it in usage (`--task-file <PATH>`);
    // the refusal's input is the flag alone.
    let argument = match error.get(ContextKind::InvalidArg) {
        Some(ContextValue::String(argument)) => Some(argument),
        Some(ContextValue::Strings(arguments)) => arguments.first(),
        _ => None,
    };
    if let Some(flag) = argument.and_then(|argument| argument.split(' ').next()) {
        refusal = refusal.input(flag);
    }
    report_failure(&refusal.into(), true)
}

fn report_failure(failure: &Failure, json: bool) -> ExitCode {
    if json {
        eprintln!("{}", failure.to_json());
    } else {
        eprint!("{}", failure.to_text());
    }
    ExitCode::from(failure.refusal.exit)
}

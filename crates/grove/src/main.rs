//! The human's binary: lifecycle, standalone work, observation and examples.
//!
//! The CLI dispatches `run`, `view` and `config` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. The loop and viewer use public library entry points;
//! standalone artifact handling and sample delivery stay here. Main propagates exit status
//! (`docs/specs/module-decomposition.md`, decision 9).

mod cli;
mod config;
mod config_json;
mod examples;
mod provision;
mod run_display;
mod standalone;

fn main() -> std::process::ExitCode {
    cli::run()
}

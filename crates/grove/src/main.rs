//! The human's binary: lifecycle, observation and inactive example delivery.
//!
//! The CLI dispatches `view` and `config` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. Both application lifetimes sit behind public library
//! entry points; sample delivery stays in the binary. Main propagates exit status
//! (`docs/specs/module-decomposition.md`, decision 9).

mod cli;
mod config;
mod config_json;
mod examples;

fn main() -> std::process::ExitCode {
    cli::run()
}

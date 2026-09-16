//! The human's binary: lifecycle execution and read-only observation.
//!
//! The CLI dispatches `view` and `config show` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. Both application lifetimes sit behind public library
//! entry points; main propagates the CLI's reported exit status
//! (`docs/specs/module-decomposition.md`, decision 9).

mod cli;
mod config;
mod config_json;

fn main() -> std::process::ExitCode {
    cli::run()
}

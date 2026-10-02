//! The human's binary: lifecycle, standalone work and observation.
//!
//! The CLI dispatches `run` and `view` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. The loop and viewer use public library entry points;
//! standalone artifact handling stays here. Main propagates exit status
//! (`docs/specs/module-decomposition.md`, decision 9).

mod cli;
mod dispatch;
mod provision;
mod run_display;
mod standalone;

fn main() -> std::process::ExitCode {
    cli::run()
}

//! The human's binary: lifecycle execution and read-only observation.
//!
//! The CLI dispatches `view` to the viewer before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. Both application lifetimes sit behind public library
//! entry points; main only propagates the result
//! (`docs/specs/module-decomposition.md`, decision 9).

mod cli;

fn main() -> anyhow::Result<()> {
    cli::run()
}

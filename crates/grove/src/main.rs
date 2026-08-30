//! The human's binary: bare `grove`, and nothing else.
//!
//! Three steps, and every one of them is something the loop cannot do for
//! itself — parse the human's (empty) command line, resolve the working tree it
//! was invoked in, and take the one-driver lease over it. The loop is
//! [`grove_loop::run`], and everything after the call below is behind it
//! (`docs/specs/module-decomposition.md`, decision 9).

mod cli;

fn main() -> anyhow::Result<()> {
    cli::run()
}

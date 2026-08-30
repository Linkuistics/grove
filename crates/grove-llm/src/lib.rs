//! **`grove-llm`: the verbs the LLM driving a grove session invokes.**
//!
//! The binary is three lines over this library, and this library is a clap
//! surface over [`grove_loop::verbs`]. It is a separate crate from the loop it
//! drives so that *the binary is thin* is a fact the compiler holds rather than
//! a discipline review has to keep (`docs/specs/module-decomposition.md`,
//! decision 1): everything reachable from here is something `grove-loop` or
//! `grove` chose to publish.
//!
//! There **is** a library target as well as the binary, and it carries the CLI
//! rather than any logic. It exists so the verb surface can be inspected by this
//! crate's own tests — a clap command tree is not something a spawned process
//! can be asked about — and it costs the guarantee above nothing, because the
//! code the binary must not reimplement is in a different crate either way.

pub mod cli;

//! Re-embed `content/` whenever any file under it changes.
//!
//! `src/methodology.rs` pulls `content/` into both binaries with `include_dir!`,
//! but on stable Rust that macro does not register its file reads with Cargo's
//! change tracker — so editing a content file would otherwise leave a stale
//! embed baked into the binary. A bare `rerun-if-changed=content` only catches
//! adds/removes (the directory mtime), so we walk the tree and emit a hint per
//! file: any edit then forces a rebuild, which is what makes the binary's
//! embedded methodology faithfully match the working `content/`.
//!
//! **That walk is now the whole script.** It used to gate the embed as well,
//! parsing every file against a unit-marker grammar and refusing to embed a
//! malformed corpus. The markers are gone: the methodology reaches a session as
//! a provisioned skill it reads as ordinary markdown, so there is no grammar to
//! be malformed against and nothing for a build error to protect a stranger's
//! loop from. What the gate genuinely bought — that every kind's reference file
//! exists and that `SKILL.md` stays inside its budget — is asserted over the
//! embed in `tests/methodology.rs`, which is where a claim about the corpus's
//! *content* belongs (`docs/adr/skill-delivers-the-methodology.md`).

use std::path::Path;

fn main() {
    emit_rerun(Path::new("content"));
}

fn emit_rerun(path: &Path) {
    println!("cargo:rerun-if-changed={}", path.display());
    if path.is_dir() {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                emit_rerun(&entry.path());
            }
        }
    }
}

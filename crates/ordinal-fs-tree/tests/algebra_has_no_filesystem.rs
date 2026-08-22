//! The algebra cannot reach the filesystem.
//!
//! Every decision the library makes — which siblings move, in what order, what
//! each is renamed to — is a pure function of the snapshot, so it is testable
//! without a directory and modellable without an abstraction of one. Inside one
//! crate that is a convention, and a convention is not a seam: this test is what
//! makes it one, and it is what keeps a later split of the crate into
//! separately-modellable units mechanical.
//!
//! The rule it holds: **`src/fs/` may name the filesystem, and no other module
//! may do anything with it but declare it.** A new module is inside the algebra
//! by default, which is the direction that fails safe — a later leaf that adds a
//! pure module gets the guarantee without having to remember to ask for it.
//!
//! # How a use is detected: ask the lexer
//!
//! Every source outside `src/fs/` is handed to `proc-macro2` and scanned for the
//! **identifier** `fs`. That is the whole rule, and it is the repair of
//! `reading-k19`'s Medium finding. This test used to strip comments with a
//! hand-written scanner and then look for the word in what was left; the scanner
//! did not know what a string literal was, so a source containing
//! `const OPEN: &str = "/*";` put it at positive block-comment depth and every
//! filesystem use after it vanished — the guard reporting clean on a file that
//! reached the filesystem freely.
//!
//! A real lexer removes the whole class. Comments never reach the token stream;
//! a doc comment arrives as `#[doc = "…"]`, whose text is a `Literal`; a string,
//! raw string, byte string or character literal is a `Literal` too. Only actual
//! code produces an `Ident`, so `fs` as an identifier is a use of the module and
//! nothing else is. Raw strings and their hash counts, and the `'a` lifetime
//! that looks exactly like an unterminated character literal, are the lexer's
//! problem rather than this file's.
//!
//! What still defeats it, stated accurately because a guard whose limits are
//! documented wrongly is worse than one with none:
//!
//! - A path assembled by a macro, or reached through something re-exported from
//!   outside `src/`, produces no `fs` identifier here. Unchanged from before, and
//!   still the reason this is a guard against drift rather than against someone
//!   working to defeat it.
//! - A source that does not lex at all is an instrument failure, so it panics
//!   rather than scanning to nothing.
//! - A *binding* called `fs` is a false positive. It fails loudly, in the safe
//!   direction.
//!
//! # The one carve-out, and why it makes the rule stronger
//!
//! Something has to write `pub mod fs;`, and under the rule as `seam-k8` stated
//! it that line was the crate root's own violation: the promised path could not
//! be declared at all. `reading-k9` met that as the first leaf to need the
//! module. So the scan exempts exactly the token sequence `mod fs ;` — a
//! declaration, which imports no path and names no item — and nothing else.
//!
//! A *re-export* stays a violation, and that is the part worth reading twice.
//! `pub use fs::ReadGuard;` in `lib.rs` would let an algebra module write
//! `use crate::ReadGuard;` and reach the filesystem through an alias this scan
//! cannot see, which is the hole named among the limits above. Refusing the
//! re-export closes it: the guards live at
//! `ordinal_fs_tree::fs::{read, write}` and nowhere else, so there is no
//! crate-root alias to launder anything through.
//!
//! # This instrument reports clean when it is broken
//!
//! A scan that finds no files reports exactly what a scan that finds no
//! violations reports. Three separate instruments in this workstream have
//! already done that — a model, a tool and a runner each reporting *found
//! nothing* and *succeeded* with the same bytes (`docs/formalism-findings.md`,
//! entry 003) — so the guard here is not "it passes". It is:
//!
//! 1. [`the_detector_fires_on_a_deliberate_violation`] — the positive control,
//!    on synthetic sources, so the detector is known to be able to say no.
//! 2. [`the_detector_is_not_blinded_by_a_literal`] — the control for the defect
//!    above: comment delimiters inside every literal form Rust has, each
//!    followed by a real use that must still be caught.
//! 3. [`the_scan_reaches_the_sources`] — the coverage control, so a scan that
//!    walked nothing fails rather than passes.
//! 4. [`the_exemption_is_the_promised_path_and_nothing_else`] and
//!    [`the_carve_out_is_the_declaration_and_nothing_else`] — because a scan
//!    that skips a file, or a line, reports what a scan that clears it reports,
//!    and those two are where things get skipped.
//! 5. A violation added by hand to a real module, watched failing, and removed.
//!    That was done when this test was written, again when `seam-k18` changed
//!    the detector, again at `reading-k9`, and again at `reading-k20` when the
//!    mechanism became a token scan — where the pair run was
//!    `use std::{fs, path::Path};` in `src/snapshot.rs` (caught) and
//!    `pub use fs::ReadGuard;` in `src/lib.rs` (caught, which is the evidence
//!    for the re-export claim above rather than an assertion of it). Redo both
//!    if the mechanism changes again.

use std::fs;
use std::path::{Path, PathBuf};

use proc_macro2::{Ident, TokenStream, TokenTree};

/// The one path, relative to `src/`, that is allowed to name the filesystem:
/// the directory `src/fs/`, and the file `src/fs.rs` that would declare it.
const FILESYSTEM_MODULE: &str = "fs";

fn src_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src")
}

/// Whether a source file, named relative to `src/`, is the module allowed to
/// name the filesystem.
///
/// By path, and not by file name. `seam-k17` found this written as *any
/// directory called `fs`, and any file called `fs.rs`*, which quietly exempts
/// `src/algebra/fs.rs` — a module in the middle of the algebra, free to hold
/// whatever it likes because of what it is called.
fn is_the_filesystem_module(relative: &Path) -> bool {
    let mut components = relative
        .components()
        .map(|c| c.as_os_str().to_string_lossy().into_owned());
    match components.next() {
        // `src/fs/…`: the directory, and everything under it.
        Some(first) if first == FILESYSTEM_MODULE => components.next().is_some(),
        // `src/fs.rs`: the file that declares it, and only at the top level.
        Some(first) => first == format!("{FILESYSTEM_MODULE}.rs") && components.next().is_none(),
        None => false,
    }
}

/// Every source file under `src/` that is not the one module allowed to name
/// the filesystem, as a path relative to `src/`.
fn algebra_sources(src: &Path) -> Vec<PathBuf> {
    fn walk(dir: &Path, prefix: &Path, out: &mut Vec<PathBuf>) {
        let listing =
            fs::read_dir(dir).unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
        for entry in listing {
            let path = entry.expect("a readable directory entry").path();
            let name = path.file_name().expect("a named entry").to_owned();
            let relative = prefix.join(&name);
            if path.is_dir() {
                walk(&path, &relative, out);
            } else if relative.extension().is_some_and(|e| e == "rs") {
                out.push(relative);
            }
        }
    }
    let mut found = Vec::new();
    walk(src, Path::new(""), &mut found);
    found.retain(|relative| !is_the_filesystem_module(relative));
    found.sort();
    found
}

/// Whether an identifier is the filesystem module's name.
///
/// Raw-aware: `r#fs` renders as `r#fs`, and it is the same identifier.
fn is_the_filesystem_ident(ident: &Ident) -> bool {
    ident
        .to_string()
        .trim_start_matches("r#")
        .eq(FILESYSTEM_MODULE)
}

/// The lines of a token stream that name the filesystem, one-indexed.
///
/// Recurses into groups, because a use inside a function body, a macro
/// invocation's parentheses or an inline module's braces is still a use. The
/// only sequence it steps over is `mod fs ;`.
fn scan(stream: TokenStream, out: &mut Vec<usize>) {
    let tokens: Vec<TokenTree> = stream.into_iter().collect();
    let mut at = 0;
    while at < tokens.len() {
        match &tokens[at] {
            TokenTree::Group(group) => scan(group.stream(), out),
            TokenTree::Ident(ident) if is_the_filesystem_ident(ident) => {
                let declared = at > 0
                    && matches!(&tokens[at - 1], TokenTree::Ident(before) if before == "mod")
                    && matches!(tokens.get(at + 1), Some(TokenTree::Punct(p)) if p.as_char() == ';');
                if declared {
                    // The declaration, and only the declaration. `mod fs { … }`
                    // has no semicolon and is not this shape, and its contents
                    // are scanned like any other group.
                    at += 2;
                    continue;
                }
                out.push(ident.span().start().line);
            }
            _ => {}
        }
        at += 1;
    }
}

/// The lines of `source` that name the filesystem, one-indexed, with the line
/// itself for the failure message.
///
/// # Panics
///
/// If the source does not lex. A guard that scanned nothing would report what a
/// guard that cleared everything reports, which is this file's whole subject.
fn violations(source: &str) -> Vec<(usize, String)> {
    let stream: TokenStream = source
        .parse()
        .unwrap_or_else(|e| panic!("the scanner could not lex this source: {e}"));
    let mut lines = Vec::new();
    scan(stream, &mut lines);
    lines.sort_unstable();
    lines.dedup();
    lines
        .into_iter()
        .map(|line| {
            let text = source.lines().nth(line - 1).unwrap_or_default();
            (line, text.trim().to_string())
        })
        .collect()
}

/// The positive control. A detector that cannot say no has never said yes.
#[test]
fn the_detector_fires_on_a_deliberate_violation() {
    for source in [
        "use std::fs;\n",
        "use std::fs::read_dir;\n",
        "    let entries = std::fs::read_dir(root)?;\n",
        "    fs::rename(from, to)?;\n",
        "use std::os::unix::fs::symlink;\n",
        // The grouped forms `seam-k17` found slipping past. The first is how
        // most Rust files actually import it; the second then makes every call
        // in the file read `f::…`, with the word `fs` nowhere near it.
        "use std::{fs, path::Path};\n",
        "use std::{fs as f, path::PathBuf};\n",
        "use std::{io::Write, fs::File};\n",
    ] {
        assert!(
            !violations(source).is_empty(),
            "the detector missed: {source}"
        );
    }
}

/// The carve-out is the declaration and nothing else. A test that only checked
/// the exempted shape would pass while the exemption swallowed every `use`, so
/// both halves are here: what it must exempt, and what it must still catch.
#[test]
fn the_carve_out_is_the_declaration_and_nothing_else() {
    for exempt in [
        "mod fs;",
        "pub mod fs;",
        "pub(crate) mod fs;",
        "pub(super) mod fs;",
        "pub(in crate::algebra) mod fs;",
        "    pub mod fs;   ",
    ] {
        assert!(
            violations(&format!("{exempt}\n")).is_empty(),
            "the declaration should be exempt: {exempt}"
        );
    }
    for caught in [
        // A re-export is not a declaration, and refusing it is what keeps an
        // algebra module from reaching the filesystem through a crate-root
        // alias this scan cannot see.
        "pub use fs::ReadGuard;",
        "pub use crate::fs::read;",
        "use crate::fs::read;",
        "    let guard = fs::read(root)?;",
        // A declaration followed by a use: the exemption is the token sequence
        // `mod fs ;` and steps over exactly those three tokens, so what comes
        // after is scanned like anything else.
        "mod fs; use std::fs;",
        "mod fs2;\nuse std::fs;",
        // An *inline* module called `fs` in the middle of the algebra: not a
        // declaration — no semicolon — and its body is scanned too.
        "mod fs { }",
        "mod other { use std::fs; }",
    ] {
        assert!(
            !violations(&format!("{caught}\n")).is_empty(),
            "the detector missed: {caught}"
        );
    }
}

/// The other half of the control: prose about the filesystem is not a use of
/// it, or every doc comment in this crate would be a violation and the test
/// would have to be weakened until it caught nothing.
#[test]
fn the_detector_ignores_a_mention_in_a_comment() {
    for source in [
        "// the algebra never calls std::fs\n",
        "/// See `std::fs` — which this module does not use.\n",
        "//! `fs::rename` lives one layer down.\n",
        "/* std::fs::read_dir /* nested */ */\nlet x = 1;\n",
    ] {
        assert!(
            violations(source).is_empty(),
            "the detector false-positived on: {source}"
        );
    }
}

/// The control for `reading-k19`'s Medium finding: a literal is not a comment.
///
/// Every literal form Rust has, each holding comment-shaped text, each followed
/// by a real filesystem use that must still be caught. Under the textual
/// stripper this file used to carry, the first of these left the scanner at
/// block-comment depth 1 and everything after it — in that file, not only on
/// that line — disappeared.
#[test]
fn the_detector_is_not_blinded_by_a_literal() {
    for source in [
        // A string literal holding an opening delimiter.
        "const OPEN: &str = \"/*\";\nuse std::fs;\n",
        // A raw string, whose hash count is the lexer's problem and not this
        // file's.
        "const RAW: &str = r#\"/* \" */\"#;\nuse std::fs;\n",
        "const RAWER: &str = r##\"/*\"##;\nuse std::fs;\n",
        // A byte string, and a byte character.
        "const BYTES: &[u8] = b\"/*\";\nuse std::fs;\n",
        "const BYTE: u8 = b'/';\nuse std::fs;\n",
        // Character literals, including the one whose neighbour is a lifetime.
        "const SLASH: char = '/';\nuse std::fs;\n",
        "fn f<'a>(s: &'a str) -> &'a str { s }\nuse std::fs;\n",
        // A closing delimiter with no opening one, which a depth counter
        // would either ignore or underflow on.
        "const CLOSE: &str = \"*/\";\nuse std::fs;\n",
        // Both on one line: the scan is per-token, so a literal earlier on a
        // line hides nothing later on it.
        "let scheme = \"file://\"; use std::fs;\n",
        // A comment delimiter inside a doc comment's own text.
        "/// A doc comment holding /* an opener.\nuse std::fs;\n",
    ] {
        assert!(
            !violations(source).is_empty(),
            "a literal blinded the detector: {source}"
        );
    }
}

/// The exemption is a path and not a file name, which is the other way this
/// instrument can read clean while broken: a module called `fs` in the middle
/// of the algebra would be skipped by the scan and could then hold anything.
#[test]
fn the_exemption_is_the_promised_path_and_nothing_else() {
    for exempt in ["fs.rs", "fs/mod.rs", "fs/lock.rs", "fs/snapshot/read.rs"] {
        assert!(
            is_the_filesystem_module(Path::new(exempt)),
            "`src/{exempt}` is the filesystem module and should be exempt"
        );
    }
    for guarded in [
        "lib.rs",
        "name.rs",
        "algebra/fs.rs",
        "algebra/fs/plan.rs",
        "nested/deep/fs.rs",
    ] {
        assert!(
            !is_the_filesystem_module(Path::new(guarded)),
            "`src/{guarded}` is the algebra and must not be exempt"
        );
    }
}

/// The coverage control. A scan that reached nothing must fail, not pass — this
/// is the assertion that distinguishes a green suite from a dead one.
#[test]
fn the_scan_reaches_the_sources() {
    let sources = algebra_sources(&src_dir());
    assert!(
        sources.len() >= 4,
        "the scan found {} source file(s); the crate has more than that",
        sources.len()
    );
    for expected in [
        "lib.rs",
        "name.rs",
        "conformance.rs",
        "reference.rs",
        "snapshot.rs",
        "error.rs",
    ] {
        assert!(
            sources.iter().any(|p| p.as_os_str() == expected),
            "the scan missed {expected}"
        );
    }
    let bytes: usize = sources
        .iter()
        .map(|p| {
            fs::read_to_string(src_dir().join(p))
                .expect("a readable source file")
                .len()
        })
        .sum();
    assert!(bytes > 0, "the scan read no bytes");
}

/// The claim itself.
#[test]
fn the_algebra_cannot_reach_the_filesystem() {
    let mut offences = Vec::new();
    let src = src_dir();
    for path in algebra_sources(&src) {
        let source = fs::read_to_string(src.join(&path)).expect("a readable source file");
        for (line, text) in violations(&source) {
            offences.push(format!("{}:{line}: {text}", path.display()));
        }
    }
    assert!(
        offences.is_empty(),
        "the algebra names the filesystem. Move this into `src/{FILESYSTEM_MODULE}/`:\n{}",
        offences.join("\n")
    );
}

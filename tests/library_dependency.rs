// What grove's dependency on `ordinal-fs-tree` costs it, held as a test rather
// than as a comment (gh issue #13, increment 2, question 4).
//
// The claim is two-part and neither half implies the other. The **library**
// promises that its imposed dependency set is exactly `libc` once its `cli`
// feature is off — that promise lives in `crates/ordinal-fs-tree/Cargo.toml` and
// is the crate's own to keep. **grove** promises to have actually asked for that
// set, by taking the dependency `default-features = false` — and grove's
// manifest is the only place that can be true or false. A test of the first half
// alone would pass while grove silently took `clap` through the default feature;
// a test of the second alone would pass while the library grew a dependency
// nobody noticed. So both are here.
//
// **Reconciled at `delete-provisioning-k19`, and unchanged by it.** That leaf
// shrank grove's *direct* dependency set — `include_dir` and `sha2` deleted with
// the embed and its content hash, `tempfile` demoted to a dev-dependency with the
// provisioning staging directory — but this file's subject is the set
// `ordinal-fs-tree` *imposes*, which no direct dependency of grove's can move.
// Both halves below still hold, and neither was weakened: the library still
// imposes only `libc`, and grove still asks for that set by taking the dependency
// `default-features = false`.
//
// `cargo tree` rather than `cargo metadata` for the first half, deliberately.
// `metadata`'s resolve unifies features across the whole workspace, so `clap`
// appears in it whatever grove asks for — grove takes `clap` directly. `-p
// ordinal-fs-tree --no-default-features` resolves *that package's* graph on its
// own, which is the graph an external consumer taking the same line would get,
// and it is transitive: a dependency of `libc` would show up here too.

use std::collections::BTreeSet;
use std::path::Path;
use std::process::Command;

/// The `cargo` that is running this test, so the check cannot drift onto a
/// different toolchain than the build.
const CARGO: &str = env!("CARGO");
const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn cargo(arguments: &[&str]) -> String {
    let output = Command::new(CARGO)
        .current_dir(Path::new(MANIFEST_DIR))
        .args(arguments)
        .output()
        .unwrap_or_else(|error| panic!("running cargo {arguments:?}: {error}"));
    assert!(
        output.status.success(),
        "cargo {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("cargo output is UTF-8")
}

/// Every crate in `ordinal-fs-tree`'s normal (non-dev, non-build) dependency
/// graph, under the given feature selection, itself excluded.
fn imposed_crates(extra: &[&str]) -> BTreeSet<String> {
    let mut arguments = vec![
        "tree",
        "-p",
        "ordinal-fs-tree",
        "-e",
        "normal",
        "--prefix",
        "none",
        "--format",
        "{lib}",
    ];
    arguments.extend_from_slice(extra);
    cargo(&arguments)
        .lines()
        .map(str::trim)
        // `cargo tree` marks a repeated subtree with a trailing ` (*)`.
        .map(|line| line.trim_end_matches(" (*)").to_string())
        .filter(|line| !line.is_empty() && line != "ordinal_fs_tree")
        .collect()
}

/// The library half: with the `cli` feature off, the whole transitive normal
/// dependency graph is `libc` and nothing else.
///
/// `libc` is there for `flock`, which has no `std` equivalent, and it costs
/// grove nothing because grove already takes it for the loop driver's signals.
#[test]
fn the_library_imposes_only_libc() {
    let imposed = imposed_crates(&["--no-default-features"]);
    assert_eq!(
        imposed,
        BTreeSet::from(["libc".to_string()]),
        "ordinal-fs-tree's imposed dependency set is no longer exactly `libc`"
    );
}

/// The control, and the reason the test above is not vacuous. A check that would
/// read the same whether or not the feature flag did anything says nothing —
/// `docs/formalism-findings.md` entry 003 is this workstream's three instruments
/// that each reported *found nothing* and *succeeded* with the same bytes. With
/// the default features on, the CLI's `clap` is in the graph, so turning them off
/// is doing observable work.
#[test]
fn the_cli_feature_is_what_the_flag_turns_off() {
    let with_cli = imposed_crates(&[]);
    assert!(
        with_cli.contains("clap"),
        "the `cli` feature no longer pulls `clap`, so `--no-default-features` may now be \
         checking nothing: {with_cli:?}"
    );
}

/// grove's half: the dependency line actually asks for that set.
///
/// Read out of the manifest through `cargo metadata --no-deps`, which reports
/// each declared dependency's `uses_default_features` — the resolved feature
/// graph cannot answer this, because the workspace unifies grove's own `clap`
/// into it either way.
#[test]
fn grove_takes_the_library_with_default_features_off() {
    let metadata = cargo(&["metadata", "--format-version", "1", "--no-deps"]);
    let grove = package(&metadata, "grove");
    let dependency = grove
        .split("{\"name\":\"")
        .find(|chunk| chunk.starts_with("ordinal-fs-tree\""))
        .expect("grove declares a dependency on ordinal-fs-tree");
    let dependency = &dependency[..dependency.find('}').expect("the dependency object ends")];
    assert!(
        dependency.contains("\"uses_default_features\":false"),
        "grove's ordinal-fs-tree dependency must read `default-features = false`, so the \
         imposed set stays `libc`: {dependency}"
    );
}

/// The `packages` entry for one workspace member, as raw JSON.
///
/// Hand-sliced rather than deserialised: grove takes no JSON dependency at all
/// since `delete-finish-transaction-k8` dropped `serde` and `serde_json` with
/// the evacuation manifest, and pulling one back in to read two fields would
/// make this file the reason it returns. The two facts wanted are flat string
/// fields on a flat object, and the assertions above name the whole slice when
/// they fail, so a shape change is legible rather than silent.
fn package<'a>(metadata: &'a str, name: &str) -> &'a str {
    let marker = format!("\"name\":\"{name}\",");
    let start = metadata
        .find(&marker)
        .unwrap_or_else(|| panic!("cargo metadata names the package {name:?}"));
    let rest = &metadata[start..];
    let end = rest
        .find("\"manifest_path\"")
        .unwrap_or_else(|| panic!("the {name:?} package entry has a manifest_path"));
    &rest[..end]
}

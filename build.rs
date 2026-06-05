//! Build the `grove-nav` zellij WASM plugin and hand its path to the compiler so
//! `src/zellij.rs` can `include_bytes!` it (leaf 070-nav-plugin).
//!
//! The plugin (`crates/grove-nav`) is a separate, workspace-excluded crate that
//! only compiles to `wasm32-wasip1`. We build it here — once, target-independent,
//! since wasm is the same for every host triple grove releases for — into a
//! private target dir under `OUT_DIR`, then export its path as the `GROVE_NAV_WASM`
//! rustc env. `src/zellij.rs` reads it with `include_bytes!(env!("GROVE_NAV_WASM"))`,
//! so the wasm rides inside the single grove binary (mirroring how the tamed
//! config + layout are embedded) and is written to the cache dir at launch.
//!
//! Recursive `cargo` is made safe by giving the inner build its **own**
//! `--target-dir` (no shared package-cache lock with the outer build) and by
//! clearing host `RUSTFLAGS`/target env that would be meaningless — or harmful —
//! for a wasm target.

use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir = PathBuf::from(env_var("CARGO_MANIFEST_DIR"));
    let nav_dir = manifest_dir.join("crates").join("grove-nav");
    let nav_manifest = nav_dir.join("Cargo.toml");
    let out_dir = PathBuf::from(env_var("OUT_DIR"));
    let nav_target = out_dir.join("grove-nav-target");

    // Only rebuild the plugin when its sources or manifest change (cargo walks the
    // src dir), not on every grove rebuild.
    println!("cargo:rerun-if-changed={}", nav_dir.join("src").display());
    println!("cargo:rerun-if-changed={}", nav_manifest.display());
    println!("cargo:rerun-if-changed=build.rs");

    // The wasm target must be installed; a clear message beats an opaque rustc
    // "can't find crate for `core`".
    let cargo = env_var("CARGO");
    let status = Command::new(&cargo)
        .arg("build")
        .arg("--release")
        .arg("--manifest-path")
        .arg(&nav_manifest)
        .arg("--target")
        .arg("wasm32-wasip1")
        .arg("--target-dir")
        .arg(&nav_target)
        // The outer build's flags/target are wrong for wasm; don't let them leak
        // into the inner invocation.
        .env_remove("CARGO_ENCODED_RUSTFLAGS")
        .env_remove("RUSTFLAGS")
        .env_remove("CARGO_BUILD_RUSTFLAGS")
        .env_remove("CARGO_BUILD_TARGET")
        .status()
        .unwrap_or_else(|e| panic!("failed to spawn `{cargo}` to build grove-nav: {e}"));

    if !status.success() {
        panic!(
            "building the grove-nav wasm plugin failed (status {:?}). If this is a \
             missing target, run: rustup target add wasm32-wasip1",
            status.code()
        );
    }

    let release = nav_target.join("wasm32-wasip1").join("release");
    let wasm = find_wasm(&release).unwrap_or_else(|| {
        panic!(
            "grove-nav built but no .wasm was found in {}",
            release.display()
        )
    });

    println!("cargo:rustc-env=GROVE_NAV_WASM={}", wasm.display());
}

fn env_var(key: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| panic!("build.rs: missing env var {key}"))
}

/// The first `*.wasm` in `dir` (a bin crate emits exactly one), robust to whether
/// cargo names it `grove-nav.wasm` or `grove_nav.wasm`.
fn find_wasm(dir: &Path) -> Option<PathBuf> {
    std::fs::read_dir(dir)
        .ok()?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| p.extension().map(|x| x == "wasm").unwrap_or(false))
}

//! The policy worker's environment (`docs/specs/harness-selection-and-execution.md`,
//! *Policy authority and runtime discovery*).
//!
//! The worker starts from nothing but HOME, a PATH snapshot, TMPDIR, LANG and
//! `LC_*` from the caller, plus each name the owner grants with
//! `--policy-env`, exactly as named. Some names are never granted, because
//! through them the environment could run code in the worker or tell it what
//! it must not know:
//!
//! - `BUN_*`, Bun's own: `BUN_OPTIONS` can preload a module before any of the
//!   worker's code runs, and `BUN_BE_BUN` turns the worker into Bun itself;
//! - `NODE_OPTIONS`, `NODE_PATH` and `NODE_PRESERVE_SYMLINKS`, which change
//!   what loads and from where;
//! - `NODE_CHANNEL_*`, which names a descriptor Bun adopts as its IPC channel,
//!   and so could hand it the private protocol channel on descriptor 3;
//! - `LD_*` and `DYLD_*`, which the dynamic loaders read to inject libraries;
//! - `HARNESS_DISPATCH_*`, harness-dispatch's own, which it sets for the
//!   harness: the worker is never told the run or where its records are.
//!
//! Beside them the front sets one value of its own, [`HOST`]: Bun's runtime
//! transpiler cache is off. Left on, Bun reads the transpiled output of any
//! imported file of 4 KiB or more from a cache under HOME, or under a granted
//! `XDG_CACHE_HOME`, keyed by the file's bytes, and runs that output in place
//! of the file. An entry nobody named as policy could then change what the
//! admitted file does, while inspection and the run record report the file's
//! own digest. No caller value replaces the setting, since `BUN_*` is never
//! granted, and Bun consults it before either cache location.
//!
//! Nothing ambient reaches the worker without a grant, installation control
//! included: the worker's location comes from the front's own path alone
//! (`worker::locate`). The command cannot know a caller's completion
//! variables, such as Grove's `GROVE_SIGNAL_FILE`, so it cannot refuse to
//! grant them. They are absent unless granted, and granting one is the owner
//! explicitly handing the worker that authority. The final harness is not
//! affected: it inherits the caller's whole environment, as before.
//!
//! No value is ever printed. Inspection shows each granted name and whether
//! the caller had it set.

use std::ffi::{OsStr, OsString};
use std::os::unix::ffi::OsStrExt as _;

use serde_json::{json, Value};

use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};

/// The names the owner granted, each once, in the order first given.
#[derive(Debug, Default)]
pub struct Grants(Vec<Grant>);

#[derive(Debug)]
struct Grant {
    name: String,
    /// Whether the caller's environment has it; unset, the worker lacks it.
    set: bool,
}

impl Grants {
    /// Check each `--policy-env` name, refusing an excluded or malformed one.
    pub fn read(names: &[OsString]) -> Result<Grants, Refusal> {
        let mut grants: Vec<Grant> = Vec::new();
        for name in names {
            let name = checked(name)?;
            if grants.iter().all(|grant| grant.name != name) {
                grants.push(Grant {
                    set: std::env::var_os(&name).is_some(),
                    name,
                });
            }
        }
        Ok(Grants(grants))
    }

    /// The worker's whole environment: the base set and the granted names,
    /// with the caller's values, and the front's own [`HOST`] settings.
    pub fn worker_environment(&self) -> Vec<(OsString, OsString)> {
        std::env::vars_os()
            .filter(|(name, _)| {
                base(name.as_bytes())
                    || self
                        .0
                        .iter()
                        .any(|grant| grant.name.as_bytes() == name.as_bytes())
            })
            .chain(
                HOST.iter()
                    .map(|(name, value)| (OsString::from(name), OsString::from(value))),
            )
            .collect()
    }

    /// `[{ name, set }]`, never a value.
    pub fn to_json(&self) -> Value {
        self.0
            .iter()
            .map(|grant| json!({ "name": grant.name, "set": grant.set }))
            .collect()
    }

    pub fn to_text(&self) -> String {
        if self.0.is_empty() {
            return "none granted; the worker has only HOME, PATH, TMPDIR, LANG and LC_* of yours"
                .to_owned();
        }
        let granted: Vec<String> = self
            .0
            .iter()
            .map(|grant| {
                let set = if grant.set { "set" } else { "not set" };
                format!("{} ({set})", grant.name)
            })
            .collect();
        format!("{}; values are never shown", granted.join(", "))
    }
}

/// The values the front itself sets in the worker's environment, each under an
/// excluded name, so no caller value or grant can replace it.
/// `BUN_RUNTIME_TRANSPILER_CACHE_PATH` of `0` turns Bun's runtime transpiler
/// cache off (bun-v1.4.2 `src/jsc/RuntimeTranspilerCache.rs`,
/// `really_get_cache_dir`).
pub const HOST: [(&str, &str); 1] = [("BUN_RUNTIME_TRANSPILER_CACHE_PATH", "0")];

/// Whether the worker gets `name` without a grant.
fn base(name: &[u8]) -> bool {
    matches!(name, b"HOME" | b"PATH" | b"TMPDIR" | b"LANG") || name.starts_with(b"LC_")
}

/// Why `name` is never granted, if it is not.
fn excluded(name: &str) -> Option<&'static str> {
    if name.starts_with("BUN_") {
        Some(
            "Bun reads its runtime options from BUN_* variables, and BUN_OPTIONS can preload \
             code and BUN_BE_BUN turn the worker into Bun itself",
        )
    } else if matches!(
        name,
        "NODE_OPTIONS" | "NODE_PATH" | "NODE_PRESERVE_SYMLINKS"
    ) {
        Some(
            "NODE_OPTIONS, NODE_PATH and NODE_PRESERVE_SYMLINKS change what code loads, and from \
             where",
        )
    } else if name.starts_with("NODE_CHANNEL_") {
        Some(
            "NODE_CHANNEL_* variables name a descriptor that Bun adopts as its IPC channel, which \
             could make the policy worker's private protocol channel its own",
        )
    } else if name.starts_with("LD_") || name.starts_with("DYLD_") {
        Some("the dynamic loaders read LD_* and DYLD_* variables, which can inject libraries")
    } else if name.starts_with("HARNESS_DISPATCH_") {
        Some(
            "HARNESS_DISPATCH_* variables are harness-dispatch's own, set for the harness it runs, \
             and the worker is never told the run or where its records are",
        )
    } else {
        None
    }
}

fn checked(name: &OsStr) -> Result<String, Refusal> {
    let malformed = |message: String| {
        Refusal::new(
            "malformed_input",
            Stage::Cli,
            EXIT_MALFORMED,
            message,
            "name one environment variable per --policy-env, such as --policy-env ANTHROPIC_API_KEY; \
             its value comes from harness-dispatch's own environment and is never shown",
        )
        .input("--policy-env")
    };
    let name = name
        .to_str()
        .ok_or_else(|| malformed("a --policy-env name is not valid UTF-8".to_owned()))?;
    if name.is_empty() {
        return Err(malformed("--policy-env must not be empty".to_owned()));
    }
    if name.contains('=') {
        return Err(malformed(format!(
            "--policy-env {name} contains `=`: it takes a variable's name, never a value"
        )));
    }
    if let Some(why) = excluded(name) {
        return Err(Refusal::new(
            "excluded_grant",
            Stage::Cli,
            EXIT_MALFORMED,
            format!("--policy-env {name} is never granted to the policy worker: {why}"),
            "remove it; the worker's environment is HOME, PATH, TMPDIR, LANG and LC_* plus the \
             names you grant, and BUN_*, NODE_OPTIONS, NODE_PATH, NODE_PRESERVE_SYMLINKS, \
             NODE_CHANNEL_*, LD_*, DYLD_* and HARNESS_DISPATCH_* are excluded from grants",
        )
        .input("--policy-env"));
    }
    Ok(name.to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exclusion_is_by_class_and_leaves_its_near_misses_grantable() {
        for name in [
            "BUN_OPTIONS",
            "BUN_BE_BUN",
            "BUN_INSTALL",
            "NODE_OPTIONS",
            "NODE_PATH",
            "NODE_PRESERVE_SYMLINKS",
            "NODE_CHANNEL_FD",
            "NODE_CHANNEL_SERIALIZATION_MODE",
            "LD_PRELOAD",
            "LD_LIBRARY_PATH",
            "LD_AUDIT",
            "DYLD_INSERT_LIBRARIES",
            "DYLD_LIBRARY_PATH",
            "HARNESS_DISPATCH_STATE_DIR",
            "HARNESS_DISPATCH_RUN_ID",
        ] {
            assert!(excluded(name).is_some(), "{name} is grantable");
        }
        for name in [
            "BUN",
            "BUNDLE_GEMFILE",
            "NODE_EXTRA_CA_CERTS",
            "NODE_PRESERVE_SYMLINKS_MAIN",
            "NODE_CHANNEL",
            "NODE_UNIQUE_ID",
            "LDFLAGS",
            "HARNESS_DISPATCHER",
            "bun_options",
            "GROVE_SIGNAL_FILE",
        ] {
            assert_eq!(excluded(name), None, "{name} is excluded");
        }
    }

    #[test]
    fn a_grant_is_one_name_and_duplicates_are_one_grant() {
        let grants = Grants::read(&["A_TOKEN".into(), "OTHER".into(), "A_TOKEN".into()]).unwrap();
        let names: Vec<&str> = grants.0.iter().map(|grant| grant.name.as_str()).collect();
        assert_eq!(names, ["A_TOKEN", "OTHER"]);
        for (name, code) in [
            ("", "malformed_input"),
            ("A=B", "malformed_input"),
            ("NODE_OPTIONS", "excluded_grant"),
        ] {
            let refusal = Grants::read(&[name.into()]).unwrap_err();
            assert_eq!(refusal.code, code, "{name}");
        }
    }
}

//! Owner settings (`docs/specs/harness-selection-and-execution.md`, *Policy
//! authority and runtime discovery*).
//!
//! What an owner sets about every invocation with the caller passing nothing:
//! the whole-selection bound, the context budget, the record directory and the
//! names granted to the worker. They live in one optional JSON file beside the
//! personal policy, and every command reads it before any worker starts, the
//! record commands included, so that all of them find the same store.
//!
//! The file has the personal policy's authority: it is found from HOME alone,
//! and no variable, cwd or repository file supplies or replaces it. Without an
//! absolute HOME it has no location and sets nothing. A flag replaces a
//! setting, and `--policy-env` adds to the granted names. The settings are the
//! same for every kind, and the policy entry has none.
//!
//! A value is held to its flag's own rules and refuses as the flag would. A
//! file that cannot be read, is not a JSON object or holds an unknown key
//! refuses as `settings_invalid`. Either names the file and the key.

use std::ffi::OsStr;
use std::fs;
use std::io::ErrorKind;
use std::ops::RangeInclusive;
use std::path::{Path, PathBuf};

use serde_json::Value;

use crate::environment;
use crate::limits::{CONTEXT_MAX_BYTES, CONTEXT_MIN_BYTES, SELECTION_MAX_MS, SELECTION_MIN_MS};
use crate::refusal::{Refusal, Stage, EXIT_MALFORMED};

/// The settings file, relative to HOME, beside the personal policy.
pub const SETTINGS_FILE: &str = ".config/harness-dispatch/settings.json";

/// Where a value the file set is reported as coming from.
pub const ORIGIN: &str = "settings.json";

#[derive(Debug, Default)]
pub struct Settings {
    pub timeout_ms: Option<u64>,
    pub context_bytes: Option<u64>,
    pub state_dir: Option<PathBuf>,
    pub policy_env: Vec<String>,
}

impl Settings {
    /// The owner's settings, checked whole. A missing file sets nothing.
    pub fn read(home: Option<&OsStr>) -> Result<Settings, Refusal> {
        let Some(home) = home.map(Path::new).filter(|home| home.is_absolute()) else {
            return Ok(Settings::default());
        };
        let path = home.join(SETTINGS_FILE);
        let file = path.to_string_lossy().into_owned();
        // A regular file only, as for the policy entry: a FIFO there would
        // hold every command before any bound applies.
        match fs::metadata(&path) {
            Err(error) if error.kind() == ErrorKind::NotFound => return Ok(Settings::default()),
            Err(error) => return Err(invalid(&file, format!("cannot be read: {error}"))),
            Ok(metadata) if !metadata.is_file() => {
                return Err(invalid(&file, "is not a regular file"))
            }
            Ok(_) => {}
        }
        let bytes =
            fs::read(&path).map_err(|error| invalid(&file, format!("cannot be read: {error}")))?;
        let document: Value = serde_json::from_slice(&bytes)
            .map_err(|error| invalid(&file, format!("is not JSON: {error}")))?;
        let Value::Object(document) = document else {
            return Err(invalid(&file, "is not a JSON object"));
        };

        let mut settings = Settings::default();
        for (key, value) in &document {
            match key.as_str() {
                "timeoutMs" => {
                    let range = SELECTION_MIN_MS..=SELECTION_MAX_MS;
                    settings.timeout_ms = Some(bound(&file, key, value, "milliseconds", range)?);
                }
                "contextBytes" => {
                    let range = CONTEXT_MIN_BYTES..=CONTEXT_MAX_BYTES;
                    settings.context_bytes = Some(bound(&file, key, value, "bytes", range)?);
                }
                "stateDir" => {
                    let dir = value.as_str().map(Path::new);
                    let dir = dir.filter(|dir| dir.is_absolute()).ok_or_else(|| {
                        invalid(
                            &file,
                            format!("holds a stateDir, {value}, that is not an absolute path"),
                        )
                        .location(key)
                    })?;
                    settings.state_dir = Some(dir.to_owned());
                }
                "policyEnv" => {
                    let names = value.as_array().ok_or_else(|| {
                        invalid(
                            &file,
                            format!("holds a policyEnv, {value}, that is not an array"),
                        )
                        .location(key)
                    })?;
                    for name in names {
                        let name = name.as_str().ok_or_else(|| {
                            invalid(
                                &file,
                                format!("holds {name} in policyEnv, which is not a string"),
                            )
                            .location(key)
                        })?;
                        environment::grantable(name, &environment::Named::Setting { file: &file })?;
                        settings.policy_env.push(name.to_owned());
                    }
                }
                _ => {
                    return Err(
                        invalid(&file, format!("holds the unknown key {key:?}")).location(key)
                    )
                }
            }
        }
        Ok(settings)
    }
}

fn invalid(file: &str, what: impl std::fmt::Display) -> Refusal {
    Refusal::new(
        "settings_invalid",
        Stage::Cli,
        EXIT_MALFORMED,
        format!("the owner settings file {file} {what}"),
        "make it one JSON object holding any of timeoutMs, contextBytes, stateDir (an absolute \
         path) and policyEnv (an array of variable names), or remove it to set nothing",
    )
    .source(file)
}

/// A bound's setting, held to its flag's rule: a whole number within `range`.
fn bound(
    file: &str,
    key: &str,
    value: &Value,
    unit: &str,
    range: RangeInclusive<u64>,
) -> Result<u64, Refusal> {
    value
        .as_u64()
        .filter(|value| range.contains(value))
        .ok_or_else(|| {
            Refusal::new(
                "malformed_input",
                Stage::Cli,
                EXIT_MALFORMED,
                format!(
                    "{key} in the owner settings file {file} is {value}, not a whole number of \
                     {unit} from {} to {}",
                    range.start(),
                    range.end()
                ),
                format!("set {key} to a whole number in that range, or remove it for the default"),
            )
            .source(file)
            .location(key)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn read(contents: &str) -> Result<Settings, Refusal> {
        let home = tempfile::tempdir().unwrap();
        let path = home.path().join(SETTINGS_FILE);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, contents).unwrap();
        Settings::read(Some(home.path().as_os_str()))
    }

    #[test]
    fn a_missing_file_or_a_home_that_cannot_place_one_sets_nothing() {
        let home = tempfile::tempdir().unwrap();
        for home in [
            Some(home.path().as_os_str()),
            Some(OsStr::new("relative")),
            None,
        ] {
            let settings = Settings::read(home).unwrap();
            assert_eq!(settings.timeout_ms, None);
            assert_eq!(settings.context_bytes, None);
            assert_eq!(settings.state_dir, None);
            assert!(settings.policy_env.is_empty());
        }
    }

    #[test]
    fn every_key_is_optional_and_each_is_read() {
        assert_eq!(read("{}").unwrap().timeout_ms, None);
        let settings = read(
            r#"{ "timeoutMs": 600000, "contextBytes": 1, "stateDir": "/records", "policyEnv": ["A_TOKEN"] }"#,
        )
        .unwrap();
        assert_eq!(settings.timeout_ms, Some(600_000));
        assert_eq!(settings.context_bytes, Some(1));
        assert_eq!(settings.state_dir.as_deref(), Some(Path::new("/records")));
        assert_eq!(settings.policy_env, ["A_TOKEN"]);
    }

    #[test]
    fn a_value_is_held_to_its_flags_rule_and_the_file_to_its_shape() {
        for (contents, code, key) in [
            (
                r#"{ "timeoutMs": 999 }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "timeoutMs": 600001 }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "timeoutMs": "5000" }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "timeoutMs": 5000.5 }"#,
                "malformed_input",
                Some("timeoutMs"),
            ),
            (
                r#"{ "contextBytes": 0 }"#,
                "malformed_input",
                Some("contextBytes"),
            ),
            (
                r#"{ "contextBytes": 8388609 }"#,
                "malformed_input",
                Some("contextBytes"),
            ),
            (
                r#"{ "policyEnv": ["BUN_OPTIONS"] }"#,
                "excluded_grant",
                Some("policyEnv"),
            ),
            (
                r#"{ "policyEnv": ["A=B"] }"#,
                "malformed_input",
                Some("policyEnv"),
            ),
            (
                r#"{ "policyEnv": [""] }"#,
                "malformed_input",
                Some("policyEnv"),
            ),
            (
                r#"{ "policyEnv": [7] }"#,
                "settings_invalid",
                Some("policyEnv"),
            ),
            (
                r#"{ "policyEnv": "A_TOKEN" }"#,
                "settings_invalid",
                Some("policyEnv"),
            ),
            (
                r#"{ "stateDir": "records" }"#,
                "settings_invalid",
                Some("stateDir"),
            ),
            (r#"{ "stateDir": 7 }"#, "settings_invalid", Some("stateDir")),
            (
                r#"{ "config": "/p/policy.ts" }"#,
                "settings_invalid",
                Some("config"),
            ),
            ("[]", "settings_invalid", None),
            ("{", "settings_invalid", None),
            ("", "settings_invalid", None),
        ] {
            let refusal = read(contents).unwrap_err();
            assert_eq!(refusal.code, code, "{contents}");
            assert_eq!(refusal.exit, EXIT_MALFORMED, "{contents}");
            assert_eq!(refusal.location.as_deref(), key, "{contents}");
            let file = refusal.source.as_deref().expect("the file is named");
            assert!(file.ends_with(SETTINGS_FILE), "{contents}: {file}");
            assert!(
                refusal.message.contains(file),
                "{contents}: {}",
                refusal.message
            );
        }
    }

    #[test]
    fn a_settings_path_that_is_not_a_regular_file_refuses() {
        let home = tempfile::tempdir().unwrap();
        fs::create_dir_all(home.path().join(SETTINGS_FILE)).unwrap();
        let refusal = Settings::read(Some(home.path().as_os_str())).unwrap_err();
        assert_eq!(refusal.code, "settings_invalid");
    }
}

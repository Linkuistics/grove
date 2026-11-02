//! PATH and CLI compatibility at Grove's independent Ravel boundary.
use std::ffi::OsStr;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;

use anyhow::{bail, ensure, Context, Result};

pub(crate) fn locate() -> Result<PathBuf> {
    locate_on_path(std::env::var_os("PATH").as_deref())
}

fn locate_on_path(path: Option<&OsStr>) -> Result<PathBuf> {
    if let Some(path) = path {
        for directory in std::env::split_paths(path).filter(|directory| directory.is_absolute()) {
            let candidate = directory.join("ravel");
            if !std::fs::metadata(&candidate)
                .is_ok_and(|file| file.is_file() && file.permissions().mode() & 0o111 != 0)
            {
                continue;
            }
            let executable = std::fs::canonicalize(&candidate)
                .with_context(|| format!("resolving Ravel executable {}", candidate.display()))?;
            check_version(&executable)?;
            return Ok(executable);
        }
    }
    bail!(
        "Ravel is missing from absolute PATH directories. Install it with \
           `brew install linkuistics/taps/ravel` and add its bin directory to PATH."
    )
}

fn check_version(executable: &std::path::Path) -> Result<()> {
    let remedy = || {
        format!(
            "{} must report an admitted stable Ravel CLI version. Grove admits \
             1.x from 1.1.0, which reads `~/.config/ravel/policy.ts`, and 2.x from \
             2.0.0, which reads `~/.config/ravel/config.rhai`. Put the bin directory \
             of the installation that matches your configuration first on the PATH \
             of this invocation. Homebrew installs 2.x as the keg-only `ravel@2`, \
             in `$(brew --prefix ravel@2)/bin`. Do not upgrade a shared installation \
             that other sessions or configurations still use.",
            executable.display()
        )
    };
    let mut probe = Command::new(executable);
    // A version probe is not an admitted session and must inherit no ending
    // or Grove tree authority, even when PATH selects an executable wrapper.
    grove_loop::ravel_job::scrub(&mut probe);
    let output = probe.arg("--version").output().with_context(remedy)?;
    // Ravel's public version flag identifies the two-verb CLI, not its worker protocol.
    // https://github.com/Linkuistics/Ravel/blob/v1.0.0/docs/session-cli.md
    // 1.1.0 corrected the failure paths of the managed job that Grove relies on,
    // so 1.0.x is refused. A canonical minor below the floor is exactly "0".
    // https://github.com/Linkuistics/Ravel/blob/v1.1.0/CHANGELOG.md
    // 2.0.0 keeps the public session CLI and replaces policy.ts with config.rhai.
    // Admission selects no route: the PATH, configuration and settings do.
    // docs/specs/codex-skill-provisioning.md#launch-and-compatibility
    let supported = std::str::from_utf8(&output.stdout)
        .ok()
        .and_then(|text| text.trim().strip_prefix("ravel "))
        .is_some_and(|version| {
            let mut parts = version.split('.');
            let (Some(major), Some(minor), Some(patch), None) =
                (parts.next(), parts.next(), parts.next(), parts.next())
            else {
                return false;
            };
            let admitted = match major {
                "1" => minor != "0",
                "2" => true,
                _ => false,
            };
            admitted
                && [minor, patch].into_iter().all(|part| {
                    !part.is_empty()
                        && part.bytes().all(|byte| byte.is_ascii_digit())
                        && (part == "0" || !part.starts_with('0'))
                })
        });
    ensure!(
        output.status.success() && supported,
        "{} Received status {} and version {:?}.",
        remedy(),
        output.status,
        String::from_utf8_lossy(&output.stdout)
    );
    Ok(())
}

#[cfg(test)]
#[path = "../../../../testing/support.rs"]
mod support;

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs::{symlink, PermissionsExt};
    use std::path::Path;

    use tempfile::TempDir;

    use super::*;

    fn executable(dir: &Path, body: &str) -> PathBuf {
        fs::create_dir_all(dir).unwrap();
        let file = dir.join("ravel");
        fs::write(&file, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
        file
    }

    fn path(dirs: &[&Path]) -> std::ffi::OsString {
        std::env::join_paths(dirs).unwrap()
    }

    #[test]
    fn resolves_first_executable_to_its_real_path() {
        let temp = TempDir::new().unwrap();
        let real = executable(&temp.path().join("ravel-1.2.7/bin"), "echo 'ravel 1.2.7'");
        let links = temp.path().join("links");
        fs::create_dir(&links).unwrap();
        symlink(&real, links.join("ravel")).unwrap();
        let later = temp.path().join("later");
        executable(&later, "echo 'ravel 1.1.0'");
        assert_eq!(
            locate_on_path(Some(&path(&[&links, &later]))).unwrap(),
            fs::canonicalize(real).unwrap()
        );
    }

    #[test]
    fn skips_nonexecutables_and_directories() {
        let temp = TempDir::new().unwrap();
        let nonexec = temp.path().join("nonexec");
        let file = executable(&nonexec, "exit 99");
        fs::set_permissions(file, fs::Permissions::from_mode(0o644)).unwrap();
        let directory = temp.path().join("directory");
        fs::create_dir_all(directory.join("ravel")).unwrap();
        let valid = temp.path().join("valid");
        let file = executable(&valid, "echo 'ravel 1.1.0'");
        assert_eq!(
            locate_on_path(Some(&path(&[&nonexec, &directory, &valid]))).unwrap(),
            fs::canonicalize(file).unwrap()
        );
    }

    #[test]
    fn missing_absolute_path_candidate_has_install_remedy() {
        let temp = TempDir::new().unwrap();
        for value in [
            None,
            Some(OsStr::new("")),
            Some(OsStr::new(".:bin")),
            Some(temp.path().as_os_str()),
        ] {
            let error = locate_on_path(value).unwrap_err().to_string();
            assert!(
                error.contains("brew install linkuistics/taps/ravel"),
                "{error}"
            );
        }
    }

    #[test]
    fn rejects_the_selected_incompatible_version_without_falling_through() {
        let temp = TempDir::new().unwrap();
        let selected = temp.path().join("selected");
        let later = temp.path().join("later");
        executable(&later, "echo 'ravel 1.1.0'");
        for version in [
            "ravel 0.1.0",
            "ravel 0.9.9",
            "ravel 1.0.0",
            "ravel 1.0.9",
            "ravel 1.1.0-beta",
            "ravel 1.1",
            "ravel 1",
            "ravel 01.1.0",
            "ravel 1.01.0",
            "ravel 1.1.00",
            "ravel 1.1.0.0",
            "ravel 2.0.0-rc.1",
            "ravel 2.0.0-beta",
            "ravel 2.0",
            "ravel 2",
            "ravel 02.0.0",
            "ravel 2.00.0",
            "ravel 2.0.01",
            "ravel 2.0.0.0",
            "ravel 2.0.0 extra",
            "ravel 3.0.0",
            "ravel 3.1.0",
            "ravel 10.0.0",
            "ravel 0.0.0",
            "harness-dispatch 1.1.0",
            "harness-dispatch 2.0.0",
            "ravel 1.1.0 extra",
        ] {
            executable(&selected, &format!("echo '{version}'"));
            let error = locate_on_path(Some(&path(&[&selected, &later])))
                .unwrap_err()
                .to_string();
            assert_admission_remedy(&error);
            assert!(error.contains(&selected.display().to_string()), "{error}");
        }
    }

    /// The remedy names both admitted majors with their configuration files and
    /// the keg-only 2.x prefix, and recommends no undirected shared upgrade.
    fn assert_admission_remedy(error: &str) {
        for part in [
            "1.x from 1.1.0",
            "policy.ts",
            "2.x from 2.0.0",
            "config.rhai",
            "ravel@2",
            "first on the PATH",
        ] {
            assert!(error.contains(part), "missing {part:?}: {error}");
        }
        assert!(!error.contains("brew upgrade"), "{error}");
    }

    #[test]
    fn accepts_stable_versions_from_the_1_1_0_and_2_0_0_floors() {
        let temp = TempDir::new().unwrap();
        for version in [
            "ravel 1.1.0",
            "ravel 1.1.9",
            "ravel 1.12.3",
            "ravel 2.0.0",
            "ravel 2.0.7",
            "ravel 2.3.0",
            "ravel 2.10.12",
        ] {
            let file = executable(temp.path(), &format!("echo '{version}'"));
            assert_eq!(
                locate_on_path(Some(temp.path().as_os_str())).unwrap(),
                fs::canonicalize(file).unwrap(),
                "{version}"
            );
        }
    }

    #[test]
    fn unsuccessful_version_probe_is_not_a_supported_installation() {
        let temp = TempDir::new().unwrap();
        for version in ["ravel 1.1.0", "ravel 2.0.0"] {
            executable(temp.path(), &format!("echo '{version}'; exit 1"));
            let error = locate_on_path(Some(temp.path().as_os_str()))
                .unwrap_err()
                .to_string();
            assert_admission_remedy(&error);
        }
    }

    /// Homebrew's layout until promotion: the unversioned `ravel` links into the
    /// 1.1.0 keg, and the keg-only `ravel@2` has only its `opt` link. The first
    /// PATH entry selects the installation, and the resolver returns its real
    /// keg path, which the ending commands quote.
    #[test]
    fn a_keg_only_ravel_2_first_on_path_is_selected_by_its_keg_path() {
        let temp = TempDir::new().unwrap();
        let cellar = temp.path().join("Cellar");
        let one = executable(&cellar.join("ravel/1.1.0/bin"), "echo 'ravel 1.1.0'");
        let two = executable(&cellar.join("ravel@2/2.0.0/bin"), "echo 'ravel 2.0.0'");
        let default_bin = temp.path().join("bin");
        fs::create_dir(&default_bin).unwrap();
        symlink(&one, default_bin.join("ravel")).unwrap();
        let opt = temp.path().join("opt");
        fs::create_dir(&opt).unwrap();
        symlink(cellar.join("ravel@2/2.0.0"), opt.join("ravel@2")).unwrap();
        let keg_bin = opt.join("ravel@2/bin");

        assert_eq!(
            locate_on_path(Some(&path(&[&default_bin, &keg_bin]))).unwrap(),
            fs::canonicalize(&one).unwrap()
        );
        let selected = locate_on_path(Some(&path(&[&keg_bin, &default_bin]))).unwrap();
        assert_eq!(selected, fs::canonicalize(&two).unwrap());
        assert_eq!(
            grove_loop::session_command::end_session(&selected).unwrap(),
            format!("'{}' end-session grove-continue", selected.display())
        );
    }

    #[test]
    fn cwd_decoy_cannot_override_absolute_path() {
        let temp = TempDir::new().unwrap();
        executable(temp.path(), "exit 99");
        executable(&temp.path().join("bin"), "exit 99");
        let selected = executable(&temp.path().join("selected"), "echo 'ravel 1.1.0'");
        let output = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "dispatch::ravel::tests::resolver_process_fixture",
                "--ignored",
            ])
            .current_dir(temp.path())
            .env(
                "PATH",
                path(&[
                    Path::new(""),
                    Path::new("."),
                    Path::new("bin"),
                    selected.parent().unwrap(),
                ]),
            )
            .env(
                "GROVE_TEST_EXPECTED_RAVEL",
                fs::canonicalize(selected).unwrap(),
            )
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn version_probe_scrubs_every_enclosing_control() {
        let temp = TempDir::new().unwrap();
        let checks: String = super::support::LOOP_CONTROL_ENV.iter().map(|name| format!(
            "if [ \"${{{name}-}}\" = /inert-enclosing-control ]; then printf 'version-observed:{name}\\n' >&2; exit 93; fi\n"
        )).collect();
        let front = executable(temp.path(), &format!("{checks}echo 'ravel 1.1.0'"));
        for name in super::support::LOOP_CONTROL_ENV {
            let output = Command::new(&front)
                .env_clear()
                .env(name, "/inert-enclosing-control")
                .arg("--version")
                .output()
                .unwrap();
            assert_eq!(output.status.code(), Some(93), "{name}: {output:?}");
            assert!(
                String::from_utf8_lossy(&output.stderr).contains(name),
                "{output:?}"
            );
        }
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "dispatch::ravel::tests::resolver_process_fixture",
                "--ignored",
            ])
            .env("PATH", temp.path())
            .env(
                "GROVE_TEST_EXPECTED_RAVEL",
                fs::canonicalize(front).unwrap(),
            );
        for name in super::support::LOOP_CONTROL_ENV {
            command.env(name, "/inert-enclosing-control");
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{output:?}");
    }

    #[test]
    #[ignore = "subprocess role driven by cwd_decoy_cannot_override_absolute_path"]
    fn resolver_process_fixture() {
        assert_eq!(
            locate().unwrap(),
            PathBuf::from(std::env::var_os("GROVE_TEST_EXPECTED_RAVEL").unwrap())
        );
    }

    #[test]
    #[ignore = "requires installed Ravel: task ravel:resolution RAVEL=/absolute/path/to/ravel"]
    fn installed_front_and_worker_return_only_the_session_result() {
        let temp = TempDir::new().unwrap();
        let prefix = temp.path().join("independent-ravel-1.1.0");
        let source = std::env::var_os("GROVE_TEST_RAVEL").expect("GROVE_TEST_RAVEL is required");
        let pair = super::support::ravel::InstalledRavel::copy_from(Path::new(&source), &prefix);
        let selected = locate_on_path(Some(pair.bin().as_os_str())).unwrap();
        assert_eq!(selected, pair.executable());
        let runtime = pair.runtime();
        let policy = temp.path().join(runtime.config_file());
        let selected_json = serde_json::to_string(&selected).unwrap();
        let source = match runtime {
            super::support::ravel::Runtime::V1_1 => format!(
                r#"import {{ definePolicy }} from 'ravel/sdk';
export const policy = definePolicy({{
  schemaVersion: 2, version: 'grove-resolver-fixture',
  select: () => ({{ status: 'selected', program: '/bin/sh',
    args: ['-c', 'printf session-noise; "$1" end-session grove-continue', 'fixture', {selected_json}],
    provider: 'fixture', model: 'fixture', effort: 'low', reason: 'installed pair' }})
}});
"#
            ),
            super::support::ravel::Runtime::V2_0 => {
                use super::support::ravel::{Arg, Route};
                let mut route = Route::new(
                    "grove-resolver-fixture",
                    "/bin/sh",
                    vec![
                        Arg::Text("-c"),
                        Arg::Text(r#"printf session-noise; "$1" end-session grove-continue"#),
                        Arg::Text("fixture"),
                        Arg::Text(selected.to_str().unwrap()),
                    ],
                );
                (route.model, route.effort, route.reason) = ("fixture", "low", "installed pair");
                route.source(runtime)
            }
        };
        fs::write(&policy, source).unwrap();
        let mut command = pair.command();
        command
            .args([
                "start-session",
                "--kind",
                "fixture",
                "--prompt",
                "unchanged",
                "--config",
            ])
            .arg(&policy)
            .env("HOME", temp.path())
            .current_dir(temp.path());
        let output = command.output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert_eq!(output.stdout, b"grove-continue");
        assert!(String::from_utf8_lossy(&output.stderr).contains("session-noise"));
    }
}

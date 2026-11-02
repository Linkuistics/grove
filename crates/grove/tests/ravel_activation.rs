//! Actual callers through an explicit installed public Ravel pair.
mod support;

use std::{fs, path::Path, process::Command};

#[test]
fn both_actual_callers_use_installed_ravel_and_separate_its_result() {
    let root = tempfile::tempdir().unwrap();
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("set GROVE_TEST_RAVEL");
    let pair = support::ravel::InstalledRavel::copy_from(
        Path::new(&source),
        &root.path().join("ravel-1.1.0"),
    );
    let home = root.path().join("home");
    fs::create_dir(&home).unwrap();
    let tree = root.path().join("project");
    fs::create_dir(&tree).unwrap();
    support::init_jj_repo(&tree);
    fs::create_dir(tree.join(".grove")).unwrap();
    fs::write(tree.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
    fs::write(tree.join(".grove/01-impl--work-k1.md"), "# work\n").unwrap();
    let ending = support::ravel::quote(pair.executable().to_str().unwrap());
    let lifecycle = support::ravel::quote(
        r#"printf managed-noise; touch "$GROVE_LAUNCH_DIR/teardown"; "$1" end-session grove-continue"#,
    );
    let standalone = support::ravel::quote(
        r#"printf managed-noise; printf published > result.txt; "$1" end-session grove-continue"#,
    );
    let source = match pair.runtime() {
        support::ravel::Runtime::V1_1 => format!(
            r#"export const policy = {{
      schemaVersion: 2, version: 'activation', select: request => ({{
        status: 'selected', program: '/bin/sh',
        args: ['-c', request.kind === 'impl' ? {lifecycle} : {standalone},
          'fixture', {ending}], provider: 'fixture', model: 'none', effort: 'none', reason: 'public boundary'
      }})
    }};
    "#
        ),
        support::ravel::Runtime::V2_0 => format!(
            r#"export const SCHEMA_VERSION = 4;
export const VERSION = "activation";

fn select(request, context) {{
    let script = if request.kind == "impl" {{ {lifecycle} }} else {{ {standalone} }};
    #{{ status: "selected", program: "/bin/sh", args: ["-c", script, "fixture", {ending}],
        provider: "fixture", model: "none", effort: "none", reason: "public boundary" }}
}}
"#
        ),
    };
    support::write_config(&home, &source);
    // Grove's version/prefix differs; a decoy old sibling must never launch.
    let bin = root.path().join("grove-22.1.0/bin");
    fs::create_dir_all(&bin).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), bin.join("grove")).unwrap();
    fs::write(bin.join("harness-dispatch"), "decoy old sibling").unwrap();
    let path = std::env::join_paths(
        std::iter::once(pair.bin().to_path_buf())
            .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    for standalone in [false, true] {
        let mut command = Command::new(bin.join("grove"));
        pair.isolate(&mut command);
        command
            .env("HOME", &home)
            .env("PATH", &path)
            .current_dir(&tree);
        if standalone {
            command
                .args(["run", "fixture", "Write output", "--ui=inline", "--output"])
                .arg(tree.join("result.txt"));
        }
        let output = command.output().unwrap();
        assert!(output.status.success(), "{output:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("managed-noise"),
            "{output:?}"
        );
        assert!(!String::from_utf8_lossy(&output.stdout).contains("grove-continue"));
    }
    assert_eq!(
        fs::read_to_string(tree.join("result.txt")).unwrap(),
        "published"
    );
}

#[test]
fn both_actual_readers_reject_missing_and_unsupported_results() {
    let temp = tempfile::tempdir().unwrap();
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("set GROVE_TEST_RAVEL");
    let pair = support::ravel::InstalledRavel::copy_from(
        Path::new(&source),
        &temp.path().join("ravel-1.1.0"),
    );
    let home = temp.path().join("home");
    let work = temp.path().join("work");
    fs::create_dir(&work).unwrap();
    support::init_jj_repo(&work);
    fs::create_dir(work.join(".grove")).unwrap();
    fs::write(work.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
    let leaf = work.join(".grove/01-impl--work-k1.md");
    fs::write(&leaf, "# work\n").unwrap();
    for result in [
        None,
        Some(""),
        Some("continue"),
        Some("grove-continue\n"),
        Some("grove-continue-extra"),
    ] {
        let ending = result.map_or("exit 0".to_owned(), |value| {
            format!("\"$1\" end-session '{}'", value.replace('\'', "'\\''"))
        });
        let body = format!("printf staged > output.txt; printf managed-noise; {ending}");
        let mut route = support::ravel::Route::new(
            "result-fixture",
            "/bin/sh",
            vec![
                support::ravel::Arg::Text("-c"),
                support::ravel::Arg::Text(&body),
                support::ravel::Arg::Text("fixture"),
                support::ravel::Arg::Text(pair.executable().to_str().unwrap()),
            ],
        );
        route.reason = "result fixture";
        support::write_config(&home, &route.source(pair.runtime()));
        for standalone in [false, true] {
            let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
            pair.isolate(&mut command);
            command
                .env("HOME", &home)
                .env("PATH", support::path_with_entry(pair.bin()))
                .current_dir(&work);
            if standalone {
                command
                    .args(["run", "fixture", "Write", "--ui=inline", "--output"])
                    .arg(work.join("published.txt"));
            }
            let output = command.output().unwrap();
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(stderr.contains("managed-noise"), "{result:?}: {output:?}");
            assert!(
                !stderr.contains("grove finished — loop complete"),
                "{output:?}"
            );
            if standalone {
                assert!(!output.status.success(), "{result:?}: {output:?}");
            } else {
                assert!(stderr.contains("loop stopped"), "{result:?}: {output:?}");
            }
            assert!(leaf.is_file());
            assert!(!work.join("published.txt").exists());
        }
    }
}

#[test]
fn both_actual_callers_refuse_missing_old_and_malformed_path_selections() {
    use std::os::unix::fs::PermissionsExt;
    let temp = tempfile::tempdir().unwrap();
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("set GROVE_TEST_RAVEL");
    let pair = support::ravel::InstalledRavel::copy_from(
        Path::new(&source),
        &temp.path().join("ravel-1.1.0"),
    );
    let home = temp.path().join("home");
    let work = temp.path().join("work");
    let earlier = temp.path().join("earlier");
    fs::create_dir_all(&earlier).unwrap();
    fs::create_dir(&work).unwrap();
    support::init_jj_repo(&work);
    fs::create_dir(work.join(".grove")).unwrap();
    fs::write(work.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
    fs::write(work.join(".grove/01-impl--work-k1.md"), "# work\n").unwrap();
    // Executable cwd and old Grove-sibling decoys must never be fallbacks.
    let bin = temp.path().join("grove-22.1.0/bin");
    fs::create_dir_all(&bin).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), bin.join("grove")).unwrap();
    for file in [
        work.join("ravel"),
        bin.join("harness-dispatch"),
        bin.join("ravel"),
    ] {
        fs::write(&file, "#!/bin/sh\nexit 99\n").unwrap();
        fs::set_permissions(file, fs::Permissions::from_mode(0o755)).unwrap();
    }
    for response in [None, Some("ravel 0.0.9"), Some("malformed version")] {
        let path = if let Some(response) = response {
            let decoy = earlier.join("ravel");
            fs::write(&decoy, format!("#!/bin/sh\nprintf '%s\\n' '{response}'\n")).unwrap();
            fs::set_permissions(decoy, fs::Permissions::from_mode(0o755)).unwrap();
            std::env::join_paths([earlier.as_path(), pair.bin()]).unwrap()
        } else {
            std::env::join_paths([Path::new("."), Path::new("/usr/bin"), Path::new("/bin")])
                .unwrap()
        };
        for standalone in [false, true] {
            let mut command = Command::new(bin.join("grove"));
            pair.isolate(&mut command);
            command
                .env("HOME", &home)
                .env("PATH", &path)
                .current_dir(&work);
            if standalone {
                command.args(["run", "fixture", "Write", "--ui=inline"]);
            }
            let output = command.output().unwrap();
            assert!(!output.status.success(), "{response:?}: {output:?}");
            let stderr = String::from_utf8_lossy(&output.stderr);
            let remedies: &[&str] = if response.is_some() {
                &["policy.ts", "config.rhai", "ravel@2"]
            } else {
                &["brew install linkuistics/taps/ravel"]
            };
            for remedy in remedies {
                assert!(stderr.contains(remedy), "{response:?}: {output:?}");
            }
            assert!(!stderr.contains("brew upgrade"), "{output:?}");
            assert!(!stderr.contains("grove: launching"), "{output:?}");
            assert!(work.join(".grove/01-impl--work-k1.md").is_file());
        }
    }
}

/// Before the cut, a stub proves the admission of `ravel 2.0.0` output. It is
/// the keg-only `ravel@2`, reached through Homebrew's `opt` link, and it records
/// the prompt that each caller passes. Both callers quote its keg path in the
/// ending commands. The unversioned 1.1 stub later on PATH never runs.
#[test]
fn both_actual_callers_quote_a_keg_only_ravel_2_selected_first_on_path() {
    use std::os::unix::fs::{symlink, PermissionsExt};
    let temp = tempfile::tempdir().unwrap();
    let work = temp.path().join("work");
    fs::create_dir(&work).unwrap();
    support::init_jj_repo(&work);
    fs::create_dir(work.join(".grove")).unwrap();
    fs::write(work.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
    let leaf = work.join(".grove/01-impl--work-k1.md");
    fs::write(&leaf, "# work\n").unwrap();
    let prompt = temp.path().join("prompt");
    let stub = |dir: &Path, version: &str| {
        fs::create_dir_all(dir).unwrap();
        let file = dir.join("ravel");
        fs::write(
            &file,
            format!(
                "#!/bin/sh\nif [ \"$1\" = --version ]; then echo '{version}'; exit 0; fi\n\
                 for arg; do case \"$arg\" in --prompt=*) printf '%s' \"${{arg#--prompt=}}\" > '{}';; esac; done\n\
                 exit 1\n",
                prompt.with_extension(version.replace(' ', "-")).display()
            ),
        )
        .unwrap();
        fs::set_permissions(&file, fs::Permissions::from_mode(0o755)).unwrap();
        file
    };
    let cellar = temp.path().join("Cellar");
    let two = stub(&cellar.join("ravel@2/2.0.0/bin"), "ravel 2.0.0");
    let one = stub(&cellar.join("ravel/1.1.0/bin"), "ravel 1.1.0");
    let opt = temp.path().join("opt");
    fs::create_dir(&opt).unwrap();
    symlink(cellar.join("ravel@2/2.0.0"), opt.join("ravel@2")).unwrap();
    let default_bin = temp.path().join("bin");
    fs::create_dir(&default_bin).unwrap();
    symlink(&one, default_bin.join("ravel")).unwrap();
    // The host PATH follows, for jj. The resolver never reaches its `ravel`.
    let path = std::env::join_paths(
        [opt.join("ravel@2/bin"), default_bin]
            .into_iter()
            .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    let keg = fs::canonicalize(&two).unwrap();
    let selected = prompt.with_extension("ravel-2.0.0");
    let decoy = prompt.with_extension("ravel-1.1.0");
    for standalone in [false, true] {
        let _ = fs::remove_file(&selected);
        let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
        for name in support::grove_env_names() {
            command.env_remove(name);
        }
        command
            .env("HOME", temp.path())
            .env("PATH", &path)
            .current_dir(&work);
        if standalone {
            command.args(["run", "fixture", "Write", "--ui=inline"]);
        }
        let output = command.output().unwrap();
        assert!(!output.status.success() || !standalone, "{output:?}");
        let recorded = fs::read_to_string(&selected)
            .unwrap_or_else(|error| panic!("{standalone}: {error}: {output:?}"));
        for result in ["grove-continue", "grove-force-stop"] {
            let ending = format!("'{}' end-session {result}", keg.display());
            assert!(recorded.contains(&ending), "{standalone}: {recorded}");
        }
        assert!(!recorded.contains("opt/ravel@2"), "{recorded}");
        assert!(!decoy.exists(), "the 1.1 stub ran");
        assert!(leaf.is_file());
    }
}

//! Actual callers through an explicit installed public Ravel pair.
mod support;

use std::{fs, path::Path, process::Command};

#[test]
fn both_actual_callers_use_installed_ravel_and_separate_its_result() {
    let root = tempfile::tempdir().unwrap();
    let source = std::env::var_os("GROVE_TEST_RAVEL").expect("set GROVE_TEST_RAVEL");
    let pair = support::ravel::InstalledRavel::copy_from(
        Path::new(&source),
        &root.path().join("ravel-1.0.0"),
    );
    let home = root.path().join("home");
    fs::create_dir(&home).unwrap();
    let tree = root.path().join("project");
    fs::create_dir(&tree).unwrap();
    support::init_jj_repo(&tree);
    fs::create_dir(tree.join(".grove")).unwrap();
    fs::write(tree.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
    fs::write(tree.join(".grove/01-impl--work-k1.md"), "# work\n").unwrap();
    let policy = home.join(".config/ravel/policy.ts");
    fs::create_dir_all(policy.parent().unwrap()).unwrap();
    let ending = serde_json::to_string(pair.executable()).unwrap();
    fs::write(&policy, format!(r#"export const policy = {{
      schemaVersion: 2, version: 'activation', select: request => ({{
        status: 'selected', program: '/bin/sh',
        args: ['-c', request.kind === 'impl'
          ? 'printf managed-noise; touch "$GROVE_LAUNCH_DIR/teardown"; "$1" end-session grove-continue'
          : 'printf managed-noise; printf published > result.txt; "$1" end-session grove-continue',
          'fixture', {ending}], provider: 'fixture', model: 'none', effort: 'none', reason: 'public boundary'
      }})
    }};
    "#)).unwrap();
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
        &temp.path().join("ravel-1.0.0"),
    );
    let home = temp.path().join("home");
    let work = temp.path().join("work");
    fs::create_dir(&work).unwrap();
    support::init_jj_repo(&work);
    fs::create_dir(work.join(".grove")).unwrap();
    fs::write(work.join(".grove/_BRIEF.md"), "# fixture\n").unwrap();
    let leaf = work.join(".grove/01-impl--work-k1.md");
    fs::write(&leaf, "# work\n").unwrap();
    let policy = home.join(".config/ravel/policy.ts");
    fs::create_dir_all(policy.parent().unwrap()).unwrap();
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
        let body = serde_json::to_string(&format!(
            "printf staged > output.txt; printf managed-noise; {ending}"
        ))
        .unwrap();
        let front = serde_json::to_string(pair.executable()).unwrap();
        fs::write(&policy, format!(r#"export const policy = {{ schemaVersion: 2, version: 'result-fixture',
          select: () => ({{ status: 'selected', program: '/bin/sh', args: ['-c', {body}, 'fixture', {front}],
            provider: 'fixture', model: 'none', effort: 'none', reason: 'result fixture' }}) }};
        "#)).unwrap();
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
        &temp.path().join("ravel-1.0.0"),
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
            let remedy = if response.is_some() {
                "brew upgrade ravel"
            } else {
                "brew install linkuistics/taps/ravel"
            };
            assert!(stderr.contains(remedy), "{response:?}: {output:?}");
            assert!(!stderr.contains("grove: launching"), "{output:?}");
            assert!(work.join(".grove/01-impl--work-k1.md").is_file());
        }
    }
}

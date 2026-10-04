use grove_loop::verbs::{self, Recorded};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use tempfile::TempDir;

#[test]
fn no_launch_records_nothing_even_outside_a_workspace() {
    let work = TempDir::new().unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("record-teardown")
        .current_dir(work.path())
        .env_remove("GROVE_LAUNCH_DIR")
        .env_remove("GROVE_SIGNAL_FILE")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("recorded nothing"));
    assert_eq!(fs::read_dir(work.path()).unwrap().count(), 0);
}

#[test]
fn teardown_refuses_every_existing_grove_shape_and_is_idempotent_after_removal() {
    let work = TempDir::new().unwrap();
    let launch = TempDir::new().unwrap();
    let grove = work.path().join(".grove");
    for shape in ["directory", "file", "dangling-link"] {
        match shape {
            "directory" => fs::create_dir(&grove).unwrap(),
            "file" => fs::write(&grove, "tree").unwrap(),
            _ => std::os::unix::fs::symlink("missing", &grove).unwrap(),
        }
        let refusal = verbs::record_teardown(work.path(), Some(launch.path())).unwrap_err();
        assert!(refusal.to_string().contains("finish-commit"), "{refusal}");
        assert!(!launch.path().join("teardown").exists());
        if shape == "directory" {
            fs::remove_dir(&grove).unwrap();
        } else {
            fs::remove_file(&grove).unwrap();
        }
    }
    let record = launch.path().join("teardown");
    assert_eq!(
        verbs::record_teardown(work.path(), Some(launch.path())).unwrap(),
        Recorded::Wrote(record.clone())
    );
    assert_eq!(
        fs::metadata(&record).unwrap().permissions().mode() & 0o777,
        0o600
    );
    fs::write(&record, "preserve").unwrap();
    assert_eq!(
        verbs::record_teardown(work.path(), Some(launch.path())).unwrap(),
        Recorded::Wrote(record.clone())
    );
    assert_eq!(fs::read_to_string(&record).unwrap(), "preserve");
}

#[test]
fn a_record_cannot_follow_a_link_or_replace_a_directory() {
    let work = TempDir::new().unwrap();
    let launch = TempDir::new().unwrap();
    let record = launch.path().join("teardown");
    let outside = work.path().join("outside");
    fs::write(&outside, "preserve").unwrap();
    std::os::unix::fs::symlink(&outside, &record).unwrap();
    assert!(verbs::record_teardown(work.path(), Some(launch.path())).is_err());
    assert_eq!(fs::read_to_string(&outside).unwrap(), "preserve");
    fs::remove_file(&record).unwrap();
    fs::create_dir(&record).unwrap();
    assert!(verbs::record_teardown(work.path(), Some(launch.path())).is_err());
    assert!(record.is_dir());
}

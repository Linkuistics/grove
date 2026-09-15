//! Exact, read-only discovery needs no jj repository or executable.
use std::fs;
use std::path::{Path, PathBuf};

use jj_workspace::Workspace;
use tempfile::TempDir;

fn snapshot(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    let mut entries = Vec::new();
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            entries.push((path.clone(), Vec::new()));
            entries.extend(snapshot(&path));
        } else {
            entries.push((path.clone(), fs::read(&path).unwrap()));
        }
    }
    entries.sort();
    entries
}

#[test]
fn discovery_does_not_create_a_workspace_or_namespace() {
    let tmp = TempDir::new().unwrap();
    assert_eq!(
        Workspace::discover_control_dir(tmp.path(), "notes").unwrap(),
        None
    );
    assert!(snapshot(tmp.path()).is_empty());

    fs::create_dir(tmp.path().join(".jj")).unwrap();
    let before = snapshot(tmp.path());
    assert_eq!(
        Workspace::discover_control_dir(tmp.path(), "notes").unwrap(),
        None
    );
    assert_eq!(snapshot(tmp.path()), before);
}

#[test]
fn discovery_uses_the_exact_workspace_without_reading_its_repository_pointer() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().canonicalize().unwrap();
    fs::create_dir_all(root.join(".jj/notes")).unwrap();
    // An invalid secondary pointer makes Workspace::resolve fail by asking jj.
    fs::write(root.join(".jj/repo"), "no repository exists here").unwrap();
    fs::write(root.join(".jj/notes/record"), "keep these bytes").unwrap();
    fs::create_dir(root.join("nested")).unwrap();
    let before = snapshot(&root);

    for _ in 0..2 {
        assert_eq!(
            Workspace::discover_control_dir(&root, "notes").unwrap(),
            Some(root.join(".jj/notes"))
        );
        assert_eq!(
            Workspace::discover_control_dir(&root.join("nested"), "notes").unwrap(),
            None
        );
    }
    assert_eq!(snapshot(&root), before);
}

#[test]
fn discovery_and_creation_agree_on_namespace_validation_and_location() {
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join(".jj")).unwrap();
    let workspace = Workspace::resolve(tmp.path()).unwrap();
    for name in [
        "",
        ".",
        "..",
        "../outside",
        "one/two",
        "one\\two",
        "repo",
        "working_copy",
        ".gitignore",
    ] {
        let before = snapshot(tmp.path());
        assert!(
            Workspace::discover_control_dir(tmp.path(), name).is_err(),
            "{name}"
        );
        assert!(workspace.control_dir(name).is_err(), "{name}");
        assert_eq!(snapshot(tmp.path()), before);
    }
    let created = workspace.control_dir("notes").unwrap();
    let before = snapshot(tmp.path());
    assert_eq!(
        Workspace::discover_control_dir(tmp.path(), "notes").unwrap(),
        Some(created)
    );
    assert_eq!(snapshot(tmp.path()), before);
}

#[test]
fn discovery_refuses_nondirectories_instead_of_reporting_absence() {
    let tmp = TempDir::new().unwrap();
    fs::write(tmp.path().join(".jj"), "not a directory").unwrap();
    assert!(Workspace::discover_control_dir(tmp.path(), "notes").is_err());
    fs::remove_file(tmp.path().join(".jj")).unwrap();
    fs::create_dir(tmp.path().join(".jj")).unwrap();
    fs::write(tmp.path().join(".jj/notes"), "not a directory").unwrap();
    let before = snapshot(tmp.path());
    assert!(Workspace::discover_control_dir(tmp.path(), "notes").is_err());
    assert_eq!(snapshot(tmp.path()), before);
}

#[cfg(unix)]
#[test]
fn discovery_accepts_workspace_aliases_and_refuses_dangling_namespace_links() {
    use std::os::unix::fs::symlink;
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("workspace");
    fs::create_dir_all(root.join(".jj/notes")).unwrap();
    let alias = tmp.path().join("alias");
    symlink(&root, &alias).unwrap();
    assert_eq!(
        Workspace::discover_control_dir(&alias, "notes").unwrap(),
        Some(root.canonicalize().unwrap().join(".jj/notes"))
    );
    symlink("absent", root.join(".jj/broken")).unwrap();
    assert!(Workspace::discover_control_dir(&root, "broken").is_err());
}

#[cfg(unix)]
#[test]
fn discovery_refuses_a_fifo_without_opening_it() {
    use std::process::Command;
    use std::sync::mpsc;
    use std::time::Duration;
    let tmp = TempDir::new().unwrap();
    fs::create_dir(tmp.path().join(".jj")).unwrap();
    assert!(Command::new("mkfifo")
        .arg(tmp.path().join(".jj/notes"))
        .status()
        .unwrap()
        .success());
    let (send, receive) = mpsc::channel();
    std::thread::spawn(move || {
        send.send(Workspace::discover_control_dir(tmp.path(), "notes").is_err())
            .unwrap();
    });
    assert!(receive
        .recv_timeout(Duration::from_secs(5))
        .expect("discovery blocked on a FIFO"));
}

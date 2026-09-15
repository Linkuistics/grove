#![cfg(unix)]
#[path = "../../grove-tui/tests/support/pty.rs"]
mod pty;

use pty::Pty;
use std::process::Command;

// Run the same smoke suite against an extracted/installed release candidate.
fn binary() -> Command {
    Command::new(
        std::env::var_os("TEST_GROVE_VIEW_BINARY")
            .unwrap_or_else(|| env!("CARGO_BIN_EXE_grove").into()),
    )
}

#[test]
fn actual_binary_restores_terminal_on_quit_interrupt_and_termination() {
    for exit in ["q", "ctrl-c", "sigterm", "sigint", "sighup"] {
        let directory = tempfile::tempdir().unwrap();
        let tree = directory.path().join(".grove");
        std::fs::create_dir(&tree).unwrap();
        std::fs::write(tree.join("_BRIEF.md"), "ROOT_CONTENT").unwrap();
        std::fs::write(tree.join("01-impl--probe-k1.md"), "SELECTED_CONTENT").unwrap();
        let mut pty = Pty::spawn(binary().arg("view").arg(directory.path()));
        pty.until("ROOT_CONTENT");
        pty.send(b"j");
        pty.until("SELECTED_CONTENT");
        pty.resize(8, 40);
        pty.until("Resize");
        pty.output.clear();
        pty.resize(24, 100);
        pty.until("SELECTED_CONTENT");
        pty.send(b"\t?");
        pty.until("Key");
        match exit {
            "q" => pty.send(b"q"),
            "ctrl-c" => pty.send(b"\x03"),
            "sigterm" => pty.signal(libc::SIGTERM),
            "sigint" => pty.signal(libc::SIGINT),
            "sighup" => pty.signal(libc::SIGHUP),
            _ => unreachable!(),
        }
        assert!(pty.wait().success(), "exit mode {exit}");
        pty.assert_restored();
        assert_eq!(
            std::fs::read_to_string(tree.join("_BRIEF.md")).unwrap(),
            "ROOT_CONTENT"
        );
        assert_eq!(
            std::fs::read_to_string(tree.join("01-impl--probe-k1.md")).unwrap(),
            "SELECTED_CONTENT"
        );
        assert_eq!(std::fs::read_dir(&tree).unwrap().count(), 2);
    }
}

#[test]
fn closed_master_exits_and_is_reaped() {
    let directory = tempfile::tempdir().unwrap();
    let mut pty = Pty::spawn(binary().arg("view").arg(directory.path()));
    pty.until("q/Ctrl-c");
    pty.close_master();
    let _ = pty.wait(); // Endpoint is gone: no claim about termios or delivered output.
}

#[test]
fn termination_interrupts_an_incomplete_escape_sequence() {
    let directory = tempfile::tempdir().unwrap();
    let mut pty = Pty::spawn(binary().arg("view").arg(directory.path()));
    pty.until("q/Ctrl-c");
    pty.send(b"\x1b[");
    std::thread::sleep(std::time::Duration::from_millis(200));
    pty.signal(libc::SIGTERM);
    assert!(pty.wait().success());
    pty.assert_restored();
}

#[test]
fn actual_binary_observes_selected_bytes_without_input() {
    use std::time::{Duration, Instant};
    let directory = tempfile::tempdir().unwrap();
    let tree = directory.path().join(".grove");
    std::fs::create_dir(&tree).unwrap();
    std::fs::write(tree.join("_BRIEF.md"), "ROOT_CONTENT").unwrap();
    let file = tree.join("01-impl--probe-k1.md");
    std::fs::write(&file, "AAAAAAAAAAAA").unwrap();
    let mut pty = Pty::spawn(binary().arg("view").arg(directory.path()));
    pty.until("ROOT_CONTENT");
    pty.send(b"j");
    pty.until("AAAAAAAAAAAA");
    pty.output.clear();
    let modified = std::fs::metadata(&file).unwrap().modified().unwrap();
    let start = Instant::now();
    std::fs::write(&file, "ZZZZZZZZZZZZ").unwrap();
    std::fs::File::options()
        .write(true)
        .open(&file)
        .unwrap()
        .set_times(std::fs::FileTimes::new().set_modified(modified))
        .unwrap();
    pty.until("ZZZZZZZZZZZZ");
    assert!(start.elapsed() < Duration::from_secs(1));
    pty.send(b"q");
    assert!(pty.wait().success());
    pty.assert_restored();
    assert_eq!(std::fs::read_to_string(file).unwrap(), "ZZZZZZZZZZZZ");
    assert_eq!(
        std::fs::read_to_string(tree.join("_BRIEF.md")).unwrap(),
        "ROOT_CONTENT"
    );
    assert_eq!(std::fs::read_dir(tree).unwrap().count(), 2);
}

#[test]
fn actual_binary_reads_wide_markdown_recovers_and_allows_grove_retirement() {
    use std::{collections::BTreeMap, fs, time::Duration};
    let directory = tempfile::tempdir().unwrap();
    let tree = directory.path().join(".grove");
    fs::create_dir(&tree).unwrap();
    fs::write(tree.join("_BRIEF.md"), "ROOT_CONTENT").unwrap();
    let file = tree.join("01-impl--probe-k1.md");
    let markdown = format!("```text\n{}WIDE_MARKER\n```\n", "x".repeat(70));
    fs::write(&file, &markdown).unwrap();
    let snapshot = || -> BTreeMap<String, Vec<u8>> {
        fs::read_dir(&tree)
            .unwrap()
            .map(|entry| {
                let path = entry.unwrap().path();
                (
                    path.file_name().unwrap().to_string_lossy().into_owned(),
                    fs::read(path).unwrap(),
                )
            })
            .collect()
    };
    let initial = snapshot();
    let mut pty = Pty::spawn(binary().arg("view").arg(directory.path()));
    pty.until("ROOT_CONTENT");
    pty.send(b"j\t");
    pty.until("xxxxxxxxxxxx");
    assert!(!String::from_utf8_lossy(&pty.output).contains("WIDE_MARKER"));
    for _ in 0..80 {
        pty.send(b"\x1b[C");
    }
    pty.until("WIDE_MARKER");
    assert_eq!(snapshot(), initial);

    pty.output.clear();
    fs::rename(tree.join("_BRIEF.md"), tree.join("interrupted-edit")).unwrap();
    let mut interrupted = initial.clone();
    let brief = interrupted.remove("_BRIEF.md").unwrap();
    interrupted.insert("interrupted-edit".into(), brief);
    pty.until("STALE");
    assert_eq!(snapshot(), interrupted);
    pty.resize(8, 40);
    pty.until("Resize");
    pty.output.clear();
    fs::rename(tree.join("interrupted-edit"), tree.join("_BRIEF.md")).unwrap();
    fs::write(&file, "RECOVERED_CONTENT").unwrap();
    let mut recovered = initial.clone();
    recovered.insert("01-impl--probe-k1.md".into(), b"RECOVERED_CONTENT".to_vec());
    pty.resize(30, 120);
    pty.until("RECOVERED_CONTENT");
    assert_eq!(snapshot(), recovered);

    // Use Grove's real write admission and retirement, bounded independently of
    // the viewer. A viewer retaining a read lock cannot satisfy this deadline.
    let worktree = directory.path().to_path_buf();
    let (sent, received) = std::sync::mpsc::channel();
    pty.output.clear();
    let worker = std::thread::spawn(move || {
        let grove_loop::Writing::Tree(writable) = grove_loop::write(&worktree).unwrap() else {
            panic!("expected tree");
        };
        let retired = grove_loop::verbs::leaf_retire(&writable, &file).unwrap();
        sent.send(retired).unwrap();
    });
    let retired = received
        .recv_timeout(Duration::from_secs(1))
        .expect("Grove mutation blocked");
    worker.join().unwrap();
    assert!(pty.child.try_wait().unwrap().is_none());
    // Ratatui emits only changed cells: LIVE -> DONE sends "DON", retaining E.
    pty.until("DON");
    pty.output.clear();
    pty.resize(31, 121);
    pty.until("probe-k1 impl DONE");
    let mut expected = recovered;
    let body = expected.remove("01-impl--probe-k1.md").unwrap();
    expected.insert("01-DONE-impl--probe-k1.md".into(), body);
    assert_eq!(snapshot(), expected);
    assert_eq!(fs::read_to_string(retired).unwrap(), "RECOVERED_CONTENT");
    pty.send(b"q");
    assert!(pty.wait().success());
    pty.assert_restored();
    assert_eq!(snapshot(), expected);
}

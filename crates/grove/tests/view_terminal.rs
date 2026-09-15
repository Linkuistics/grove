#![cfg(unix)]
#[path = "../../grove-tui/tests/support/pty.rs"]
mod pty;

use pty::Pty;
use std::process::Command;

#[test]
fn actual_binary_restores_terminal_on_quit_interrupt_and_termination() {
    for exit in ["q", "ctrl-c", "sigterm", "sigint", "sighup"] {
        let directory = tempfile::tempdir().unwrap();
        let tree = directory.path().join(".grove");
        std::fs::create_dir(&tree).unwrap();
        std::fs::write(tree.join("_BRIEF.md"), "ROOT_CONTENT").unwrap();
        std::fs::write(tree.join("01-impl--probe-k1.md"), "SELECTED_CONTENT").unwrap();
        let mut pty = Pty::spawn(
            Command::new(env!("CARGO_BIN_EXE_grove"))
                .arg("view")
                .arg(directory.path()),
        );
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
    let mut pty = Pty::spawn(
        Command::new(env!("CARGO_BIN_EXE_grove"))
            .arg("view")
            .arg(directory.path()),
    );
    pty.until("q/Ctrl-c");
    pty.close_master();
    let _ = pty.wait(); // Endpoint is gone: no claim about termios or delivered output.
}

#[test]
fn termination_interrupts_an_incomplete_escape_sequence() {
    let directory = tempfile::tempdir().unwrap();
    let mut pty = Pty::spawn(
        Command::new(env!("CARGO_BIN_EXE_grove"))
            .arg("view")
            .arg(directory.path()),
    );
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
    let mut pty = Pty::spawn(
        Command::new(env!("CARGO_BIN_EXE_grove"))
            .arg("view")
            .arg(directory.path()),
    );
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

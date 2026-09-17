use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use grove_loop::{ActivityObservation, DriverLease, TemplateSource, Workspace};
use grove_tui::{Action, Viewer};
use ratatui::{
    backend::TestBackend,
    buffer::Buffer,
    style::{Color, Modifier},
    Terminal,
};

const LIMIT: Duration = Duration::from_secs(10);

fn put(root: &Path, name: &str, text: &str) {
    let path = root.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

// Separate processes keep the real loop's signal handlers out of the test runner.
#[test]
#[ignore = "self-spawned driver"]
fn driver_helper() {
    if std::env::var("GROVE_TEST_TUI_PROCESS").as_deref() != Ok("1") {
        return;
    }
    let root = std::env::current_dir().unwrap();
    let workspace = Workspace::resolve(&root).unwrap();
    let lease = DriverLease::acquire(&workspace).unwrap();
    let outcome = grove_loop::run(
        &workspace,
        lease,
        &TemplateSource::under(root.join("config")),
    );
    if root.join("expect-spawn-error").exists() {
        assert!(outcome.is_err());
    } else {
        assert_eq!(outcome.unwrap(), grove_loop::LoopOutcome::Stopped);
    }
}

#[test]
#[ignore = "real configured session"]
fn session_helper() {
    if std::env::var("GROVE_TEST_TUI_PROCESS").as_deref() != Ok("1") {
        return;
    }
    let mut stream = UnixStream::connect("session.sock").unwrap();
    stream.set_read_timeout(Some(LIMIT)).unwrap();
    stream.write_all(b"ready").unwrap();
    let mut command = [0];
    loop {
        stream.read_exact(&mut command).unwrap();
        match command[0] {
            b'x' => return,
            b's' => {
                fs::write(
                    std::env::var_os("GROVE_SIGNAL_FILE").unwrap(),
                    b"relaunch\n",
                )
                .unwrap();
                stream.write_all(b"s").unwrap();
            }
            b'p' => stream.write_all(b"p").unwrap(),
            other => panic!("unexpected session command {other}"),
        }
    }
}

struct Launch {
    driver: Child,
    session: Option<UnixStream>,
    listener: UnixListener,
}

impl Launch {
    fn start(root: &Path) -> Self {
        fs::create_dir(root.join(".jj")).unwrap();
        let exe = std::env::current_exe().unwrap();
        let exe = exe.to_str().unwrap();
        assert!(!exe.contains('\''));
        put(
            root,
            "harness",
            &format!("exec '{exe}' --exact session_helper --ignored --nocapture\n"),
        );
        put(
            root,
            "config/.config/grove/config.kdl",
            "config {\n    command \"runner\" \"/bin/sh harness '${prompt}'\"\n    bind \"lead\" \"runner\"\n    route \"impl\" \"lead\"\n    route \"finish\" \"lead\"\n}\n",
        );
        let listener = UnixListener::bind(root.join("session.sock")).unwrap();
        listener.set_nonblocking(true).unwrap();
        let driver = Command::new(exe)
            .args(["--exact", "driver_helper", "--ignored", "--nocapture"])
            .current_dir(root)
            .env("GROVE_TEST_TUI_PROCESS", "1")
            .env_remove("GROVE_SIGNAL_FILE")
            .env_remove("GROVE_HARNESS_PID")
            .env_remove("GROVE_CLAUDE_PID")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        let mut launch = Self {
            driver,
            session: None,
            listener,
        };
        launch.accept_session(root);
        launch
    }

    fn accept_session(&mut self, root: &Path) {
        let deadline = Instant::now() + LIMIT;
        let mut stream = loop {
            match self.listener.accept() {
                Ok((stream, _)) => break stream,
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(error) => panic!("accept session: {error}"),
            }
            assert!(
                self.driver.try_wait().unwrap().is_none(),
                "driver exited before readiness"
            );
            assert!(Instant::now() < deadline, "session readiness timed out");
            std::thread::sleep(Duration::from_millis(2));
        };
        stream.set_nonblocking(false).unwrap();
        stream.set_read_timeout(Some(LIMIT)).unwrap();
        let mut ready = [0; 5];
        stream.read_exact(&mut ready).unwrap();
        assert_eq!(&ready, b"ready");
        self.session = Some(stream);
        // Child readiness can precede the parent's Started callback. Wait for
        // production witness evidence, never infer publication from elapsed time.
        loop {
            if matches!(
                grove_loop::try_observe(root, &[]).activity,
                ActivityObservation::Running(_)
            ) {
                return;
            }
            assert!(Instant::now() < deadline, "Started publication timed out");
            std::thread::sleep(Duration::from_millis(2));
        }
    }

    fn exchange(&mut self, command: u8) {
        let stream = self.session.as_mut().unwrap();
        stream.write_all(&[command]).unwrap();
        let mut response = [0];
        stream.read_exact(&mut response).unwrap();
        assert_eq!(response, [command]);
    }

    fn finish(mut self) {
        self.session.as_mut().unwrap().write_all(b"x").unwrap();
        let deadline = Instant::now() + LIMIT;
        loop {
            if let Some(status) = self.driver.try_wait().unwrap() {
                assert!(status.success(), "driver did not finish normally: {status}");
                return;
            }
            assert!(Instant::now() < deadline, "driver did not confirm reap");
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}

impl Drop for Launch {
    fn drop(&mut self) {
        if let Some(session) = &mut self.session {
            let _ = session.write_all(b"x");
        }
        let deadline = Instant::now() + LIMIT;
        loop {
            if self.driver.try_wait().ok().flatten().is_some() {
                return;
            }
            if Instant::now() >= deadline {
                let _ = self.driver.kill();
                let _ = self.driver.wait();
                return;
            }
            std::thread::sleep(Duration::from_millis(2));
        }
    }
}

fn screen(viewer: &mut Viewer, width: u16, height: u16) -> (String, Buffer) {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|frame| viewer.render(frame)).unwrap();
    let buffer = terminal.backend().buffer().clone();
    let text = buffer
        .content
        .chunks(usize::from(width))
        .map(|row| row.iter().map(|cell| cell.symbol()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    (text, buffer)
}

fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(base: &Path, path: &Path, files: &mut BTreeMap<String, Vec<u8>>) {
        for entry in fs::read_dir(path).unwrap() {
            let path = entry.unwrap().path();
            let name = path
                .strip_prefix(base)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            if path.is_dir() {
                files.insert(format!("{name}/"), Vec::new());
                visit(base, &path, files);
            } else {
                files.insert(name, fs::read(path).unwrap());
            }
        }
    }
    let mut files = BTreeMap::new();
    // The fixture's socket is outside these two observation-owned surfaces.
    for name in [".grove", ".jj"] {
        if root.join(name).exists() {
            visit(root, &root.join(name), &mut files);
        }
    }
    files
}

#[test]
fn real_launch_keeps_lifecycle_running_and_finish_forecast_independent() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-finish--finish-k2.md", "finish");
    put(&root, "02-impl--work-k1.md", "work");
    let launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::End); // Selected running row must retain its own styles.
    let mut old = "02-impl--work-k1.md";
    for (name, lifecycle, color, glyph) in [
        ("02-impl--work-k1.md", "LIVE", Color::Reset, " "),
        ("02-DONE-impl--work-k1.md", "DONE", Color::Green, "✓"),
        (
            "02-ABANDONED-impl--work-k1.md",
            "ABANDONED",
            Color::Red,
            "✗",
        ),
    ] {
        if old != name {
            fs::rename(root.join(old), root.join(name)).unwrap();
        }
        old = name;
        let before = snapshot(work.path());
        viewer.act(Action::Refresh);
        let (text, buffer) = screen(&mut viewer, 60, 10);
        assert!(
            text.lines().nth(2).unwrap().contains("RUNNING: work-k1"),
            "{text}"
        );
        assert!(
            text.lines().nth(3).unwrap().contains("NEXT: finish-k2"),
            "{text}"
        );
        let y = text
            .lines()
            .position(|line| line.contains(lifecycle) && line.contains("work-k1"))
            .unwrap();
        assert!(text.lines().nth(y).unwrap().contains("RUNNING"), "{text}");
        assert_eq!(buffer[(1, y as u16)].symbol(), ">");
        assert_eq!(buffer[(3, y as u16)].symbol(), glyph);
        assert_eq!(buffer[(5, y as u16)].fg, color);
        for x in [15, 27] {
            let cell = &buffer[(x, y as u16)];
            assert_eq!(cell.fg, Color::Yellow, "{text}");
            assert!(cell.modifier.contains(Modifier::BOLD));
            assert!(!cell.modifier.contains(Modifier::REVERSED));
        }
        let next_y = text
            .lines()
            .position(|line| line.contains("LIVE") && line.contains("finish-k2"))
            .unwrap();
        assert_eq!(buffer[(15, next_y as u16)].fg, Color::Reset);
        assert!(buffer[(15, next_y as u16)]
            .modifier
            .contains(Modifier::BOLD));
        assert!(!buffer[(27, next_y as u16)]
            .modifier
            .contains(Modifier::BOLD));
        viewer.act(Action::Focus);
        let mut another = Viewer::new(work.path().into());
        assert!(screen(&mut another, 60, 10).0.contains("RUNNING: work-k1"));
        let (file, _) = screen(&mut viewer, 60, 10);
        assert!(
            file.contains("RUNNING: work-k1") && file.contains("NEXT: finish-k2"),
            "{file}"
        );
        viewer.act(Action::Focus);
        assert_eq!(snapshot(work.path()), before);
    }
    launch.finish(); // Actual driver return follows confirmed child reap.
    viewer.act(Action::Refresh);
    let (text, _) = screen(&mut viewer, 60, 10);
    assert!(text.contains("RUNNING: none (idle)"), "{text}");
}

#[test]
fn real_launch_follows_key_into_a_renamed_moved_branch_and_hidden_summaries() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-impl--work-k1.md", "work");
    put(&root, "02-finish--finish-k2.md", "finish");
    let _launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    fs::rename(
        root.join("01-impl--work-k1.md"),
        root.join("04-impl--renamed-k1.md"),
    )
    .unwrap();
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10)
        .0
        .contains("RUNNING: renamed-k1"));
    let renamed = "a-very-long-renamed-running-item-that-must-preserve-the-key";
    put(&root, "03-k3/_container.md", "container");
    put(&root, &format!("03-k3/01-k1/_{renamed}.md"), "new branch");
    fs::remove_file(root.join("04-impl--renamed-k1.md")).unwrap();
    put(&root, "03-k3/01-k1/01-impl--child-k4.md", "child");
    viewer.act(Action::Refresh);
    let (text, _) = screen(&mut viewer, 140, 18);
    let row = text
        .lines()
        .skip(4)
        .find(|line| line.contains(renamed))
        .unwrap();
    assert!(row.contains("RUNNING") && row.contains("branch"), "{text}");
    assert!(!text
        .lines()
        .skip(4)
        .find(|line| line.contains("container-k3"))
        .unwrap()
        .contains("RUNNING"));
    assert!(
        text.lines().nth(3).unwrap().contains("NEXT: child-k4"),
        "{text}"
    );
    viewer.act(Action::Home);
    viewer.act(Action::Toggle); // Fold root; all activity rows disappear.
    for file in [false, true] {
        if file {
            viewer.act(Action::Focus);
        }
        let (text, _) = screen(&mut viewer, 60, 10);
        let summary = text.lines().nth(2).unwrap();
        assert!(
            summary.starts_with("RUNNING: a-very-long") && summary.trim_end().ends_with("-k1"),
            "{text}"
        );
        assert!(
            text.lines().nth(3).unwrap().contains("NEXT: child-k4"),
            "{text}"
        );
        assert!(!text
            .lines()
            .skip(4)
            .any(|line| line.contains("RUNNING") || line.contains("NEXT")));
    }
}

#[test]
fn real_launch_never_attaches_to_retained_rows_or_selects_an_invalid_tree() {
    use std::os::fd::AsRawFd;
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-impl--work-k1.md", "work");
    put(&root, "02-finish--finish-k2.md", "finish");
    let _launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: work-k1"));
    let lock = fs::File::open(work.path()).unwrap();
    assert_eq!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0
    );
    viewer.tick(Instant::now() + Duration::from_secs(1));
    let (text, _) = screen(&mut viewer, 60, 10);
    assert!(text.contains("WAITING for tree writer"), "{text}");
    assert!(text.contains("NEXT: unavailable"), "{text}");
    assert!(
        !text
            .lines()
            .skip(4)
            .any(|line| line.contains("RUNNING") || line.contains("NEXT")),
        "{text}"
    );
    assert!(viewer.act(Action::Quit));
    drop(lock);
    for invalid in [
        "03-DONE-impl--duplicate-k1.md",
        "03-k1/_duplicate.md",
        "03-finish--second-k3.md",
    ] {
        viewer.act(Action::Refresh);
        assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: work-k1"));
        put(&root, invalid, "invalid");
        viewer.act(Action::Refresh);
        let (text, _) = screen(&mut viewer, 60, 10);
        assert!(text.contains("STALE"), "{text}");
        assert!(text.contains("NEXT: unavailable"), "{text}");
        assert!(
            text.lines().nth(2).unwrap().contains("tree unavailable"),
            "{text}"
        );
        assert!(text.lines().nth(2).unwrap().contains("work-k1"), "{text}");
        assert!(
            !text
                .lines()
                .skip(4)
                .any(|line| line.contains("RUNNING") || line.contains("NEXT")),
            "{text}"
        );
        fs::remove_file(root.join(invalid)).unwrap();
        if invalid.contains('/') {
            fs::remove_dir(root.join("03-k1")).unwrap();
        }
    }
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: work-k1"));
}

#[test]
fn real_launch_preserves_absent_item_and_tree_identity_at_minimum_width() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let slug = "a-very-long-launched-item-that-must-retain-its-key-and-qualifier";
    let name = format!("01-impl--{slug}-k1.md");
    put(&root, "_BRIEF.md", "root");
    put(&root, &name, "work");
    put(&root, "02-finish--finish-k2.md", "finish");
    let launch = Launch::start(work.path());
    let aliases = tempfile::tempdir().unwrap();
    let alias = aliases.path().join("alias");
    std::os::unix::fs::symlink(work.path(), &alias).unwrap();
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 60, 10)
        .0
        .lines()
        .nth(2)
        .unwrap()
        .contains("-k1"));
    fs::remove_file(root.join(name)).unwrap();
    for absent_tree in [false, true] {
        if absent_tree {
            fs::remove_dir_all(&root).unwrap();
        }
        let before = snapshot(work.path());
        viewer.act(Action::Refresh);
        let mut arriving = Viewer::new(work.path().into());
        let mut aliased = Viewer::new(alias.clone());
        for current in [&mut viewer, &mut arriving, &mut aliased] {
            for _ in 0..2 {
                let (text, _) = screen(current, 60, 10);
                let running = text.lines().nth(2).unwrap();
                assert!(running.starts_with("RUNNING: a-very-long"), "{text}");
                assert!(running.contains("-k1"), "{text}");
                assert!(
                    running.contains(if absent_tree {
                        "tree absent"
                    } else {
                        "item absent"
                    }),
                    "{text}"
                );
                assert!(
                    text.lines().nth(3).unwrap().contains(if absent_tree {
                        "NEXT: unavailable"
                    } else {
                        "NEXT: finish-k2"
                    }),
                    "{text}"
                );
                assert!(
                    !text.lines().skip(4).any(|line| line.contains("RUNNING")),
                    "{text}"
                );
                current.act(Action::Focus);
            }
        }
        assert_eq!(snapshot(work.path()), before);
    }
    launch.finish();
}

#[test]
fn witnessed_replacement_resets_folds_and_reading_positions_but_brief_edit_preserves_them() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let content = format!(
        "```\n{}\n```",
        (0..30)
            .map(|n| format!("LINE{n:02} {}", "abcdefghij".repeat(12)))
            .collect::<Vec<_>>()
            .join("\n")
    );
    put(&root, "_BRIEF.md", "OLD ROOT");
    put(&root, "01-k2/_branch.md", "branch");
    put(&root, "01-k2/01-impl--work-k1.md", &content);
    let _launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::End);
    viewer.act(Action::Focus);
    screen(&mut viewer, 60, 10);
    viewer.act(Action::PageDown);
    viewer.act(Action::Right);
    let old_file = screen(&mut viewer, 60, 10).0;
    put(&root, "replacement", "EDITED ROOT");
    fs::rename(root.join("replacement"), root.join("_BRIEF.md")).unwrap();
    viewer.act(Action::Refresh);
    assert_eq!(screen(&mut viewer, 60, 10).0, old_file);
    viewer.act(Action::Focus);
    viewer.act(Action::Home);
    viewer.act(Action::Down);
    viewer.act(Action::Toggle);
    let folded = screen(&mut viewer, 60, 10).0;
    assert!(!folded.lines().skip(4).any(|line| line.contains("work-k1")));
    put(&root, "_BRIEF.md", "ANOTHER ROOT EDIT");
    viewer.act(Action::Refresh);
    assert_eq!(screen(&mut viewer, 60, 10).0, folded);
    fs::rename(&root, work.path().join("old-tree")).unwrap();
    put(&root, "_BRIEF.md", "NEW ROOT");
    put(&root, "01-k2/_branch.md", "branch");
    put(&root, "01-k2/01-impl--work-k1.md", &content);
    viewer.act(Action::Refresh);
    let tree = screen(&mut viewer, 60, 10).0;
    assert!(
        tree.lines().skip(4).any(|line| line.contains("work-k1")),
        "{tree}"
    );
    viewer.act(Action::Focus);
    assert!(screen(&mut viewer, 60, 10).0.contains("NEW ROOT"));
    viewer.act(Action::Focus);
    viewer.act(Action::End);
    viewer.act(Action::Focus);
    let fresh_file = screen(&mut viewer, 60, 10).0;
    assert!(fresh_file.contains("LINE00"), "{fresh_file}");
    assert!(fresh_file.contains("previous tree"), "{fresh_file}");
}

#[test]
fn real_launch_replacement_does_not_attach_or_exclude_reused_key() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let name = "01-impl--a-very-long-launched-item-that-needs-key-and-qualifier-space-k1.md";
    put(&root, "_BRIEF.md", "OLD ROOT");
    put(&root, name, "OLD ITEM");
    let _launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    viewer.act(Action::Focus);
    fs::rename(&root, work.path().join("old-tree")).unwrap();
    put(&root, "_BRIEF.md", "NEW ROOT");
    put(&root, name, "NEW ITEM");
    let before = snapshot(work.path());
    viewer.act(Action::Refresh);
    let mut arriving = Viewer::new(work.path().into());
    for current in [&mut viewer, &mut arriving] {
        let (text, _) = screen(current, 60, 10);
        assert!(text.contains("previous tree"), "{text}");
        assert!(
            text.lines().nth(2).unwrap().contains("-k1 (previous tree)"),
            "{text}"
        );
        assert!(
            text.lines()
                .nth(3)
                .unwrap()
                .starts_with("NEXT: a-very-long"),
            "{text}"
        );
        assert!(
            text.lines().nth(3).unwrap().trim_end().ends_with("-k1"),
            "{text}"
        );
        assert!(
            !text.lines().skip(4).any(|line| line.contains("RUNNING")),
            "{text}"
        );
        current.act(Action::Focus);
        let (text, _) = screen(current, 60, 10);
        assert!(
            text.contains("NEW ROOT") && !text.contains("NEW ITEM"),
            "{text}"
        );
        assert!(
            text.contains("previous tree") && text.contains("NEXT: a-very-long"),
            "{text}"
        );
    }
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn real_launch_signal_keeps_running_until_reap_then_hands_off() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-impl--first-k1.md", "first");
    put(&root, "02-impl--second-k2.md", "second");
    let mut launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: first-k1"));
    fs::rename(
        root.join("01-impl--first-k1.md"),
        root.join("01-DONE-impl--first-k1.md"),
    )
    .unwrap();
    launch.exchange(b's'); // Acknowledged signal; the child still awaits exit.
    viewer.act(Action::Refresh);
    let (text, _) = screen(&mut viewer, 60, 10);
    assert!(
        text.contains("RUNNING: first-k1") && text.contains("NEXT: second-k2"),
        "{text}"
    );
    launch.exchange(b'p');
    launch.session.take().unwrap().write_all(b"x").unwrap();
    launch.accept_session(work.path()); // Next launch requires predecessor reap.
    viewer.tick(Instant::now() + Duration::from_secs(1));
    let (text, _) = screen(&mut viewer, 60, 10);
    assert!(
        text.contains("RUNNING: second-k2") && text.contains("NEXT: none"),
        "{text}"
    );
    launch.finish();
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10)
        .0
        .contains("RUNNING: none (idle)"));
}

#[test]
fn killed_driver_clears_running_despite_started_bytes_and_live_session() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-impl--work-k1.md", "work");
    let mut launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: work-k1"));
    let before = snapshot(work.path());
    launch.driver.kill().unwrap();
    assert!(!launch.driver.wait().unwrap().success());
    launch.exchange(b'p'); // The configured child demonstrably outlives its driver.
    viewer.tick(Instant::now() + Duration::from_secs(1));
    let (text, _) = screen(&mut viewer, 60, 10);
    assert!(
        text.contains("RUNNING: none (idle)") && text.contains("NEXT: work-k1"),
        "{text}"
    );
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn real_launch_root_open_failure_retains_summary_and_recovers() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-impl--work-k1.md", "work");
    let _launch = Launch::start(work.path());
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: work-k1"));
    fs::rename(&root, work.path().join("old-tree")).unwrap();
    std::os::unix::fs::symlink(".grove", &root).unwrap();
    let mut arriving = Viewer::new(work.path().into());
    viewer.act(Action::Refresh);
    for current in [&mut viewer, &mut arriving] {
        let (text, _) = screen(current, 60, 10);
        assert!(
            text.contains("RUNNING: work-k1 (tree unavailable)"),
            "{text}"
        );
        assert!(text.contains("NEXT: unavailable"), "{text}");
        assert!(
            !text.lines().skip(4).any(|line| line.contains("RUNNING")),
            "{text}"
        );
        assert!(current.retry_after(Instant::now()).unwrap() <= Duration::from_millis(500));
        current.act(Action::Help);
        current.tick(Instant::now() + Duration::from_secs(1));
        screen(current, 40, 5);
        assert!(current.act(Action::Quit));
        current.act(Action::Dismiss);
    }
    fs::remove_file(&root).unwrap();
    fs::rename(work.path().join("old-tree"), &root).unwrap();
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10).0.contains("RUNNING: work-k1"));
}

#[test]
fn failed_and_immediate_launches_leave_no_running_attachment() {
    for failed in [false, true] {
        let work = tempfile::tempdir().unwrap();
        let root = work.path().join(".grove");
        put(&root, "_BRIEF.md", "root");
        put(&root, "01-impl--work-k1.md", "work");
        fs::create_dir(work.path().join(".jj")).unwrap();
        let command = if failed {
            "/missing-grove-test-executable"
        } else {
            "/usr/bin/true"
        };
        put(
            work.path(),
            "config/.config/grove/config.kdl",
            &format!(
                "config {{\n    command \"runner\" \"{command} '${{prompt}}'\"\n    bind \"lead\" \"runner\"\n    route \"impl\" \"lead\"\n}}\n"
            ),
        );
        if failed {
            put(work.path(), "expect-spawn-error", "");
        }
        let mut driver = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "driver_helper", "--ignored", "--nocapture"])
            .current_dir(work.path())
            .env("GROVE_TEST_TUI_PROCESS", "1")
            .env_remove("GROVE_SIGNAL_FILE")
            .env_remove("GROVE_HARNESS_PID")
            .env_remove("GROVE_CLAUDE_PID")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .spawn()
            .unwrap();
        let deadline = Instant::now() + LIMIT;
        loop {
            if let Some(status) = driver.try_wait().unwrap() {
                assert!(status.success());
                break;
            }
            if Instant::now() >= deadline {
                driver.kill().unwrap();
                driver.wait().unwrap();
                panic!("driver did not return after failed/immediate launch");
            }
            std::thread::sleep(Duration::from_millis(2));
        }
        let before = snapshot(work.path());
        let mut viewer = Viewer::new(work.path().into());
        let (text, _) = screen(&mut viewer, 60, 10);
        assert!(
            text.contains("RUNNING: none (idle)") && text.contains("NEXT: work-k1"),
            "{text}"
        );
        assert_eq!(snapshot(work.path()), before);
    }
}

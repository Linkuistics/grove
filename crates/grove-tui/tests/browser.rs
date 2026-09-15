use grove_tui::{Action, Viewer};
use ratatui::{backend::TestBackend, Terminal};
use std::{collections::BTreeMap, fs, path::Path};

fn put(root: &Path, name: &str, text: &str) {
    let path = root.join(name);
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, text).unwrap();
}

fn screen(viewer: &mut Viewer, width: u16, height: u16) -> String {
    let mut terminal = Terminal::new(TestBackend::new(width, height)).unwrap();
    terminal.draw(|frame| viewer.render(frame)).unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .chunks(usize::from(width).max(1))
        .map(|row| row.iter().map(|cell| cell.symbol()).collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn snapshot(root: &Path) -> BTreeMap<String, Vec<u8>> {
    fn visit(base: &Path, path: &Path, out: &mut BTreeMap<String, Vec<u8>>) {
        for item in fs::read_dir(path).unwrap() {
            let path = item.unwrap().path();
            let name = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if path.is_dir() {
                out.insert(format!("{name}/"), Vec::new());
                visit(base, &path, out);
            } else {
                out.insert(name, fs::read(path).unwrap());
            }
        }
    }
    let mut out = BTreeMap::new();
    visit(root, root, &mut out);
    out
}

#[test]
fn nested_browser_selects_briefs_and_tasks_without_writing() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT CHARTER");
    put(&root, "01-k1/_branch.md", "BRANCH CHARTER");
    put(
        &root,
        "01-k1/01-DONE-unusual-kind--first-k2.md",
        "FIRST BODY",
    );
    put(
        &root,
        "01-k1/02-ABANDONED-impl--second-k3.md",
        "SECOND BODY",
    );
    put(&root, "02-impl--third-k4.md", "THIRD BODY");
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    let initial = screen(&mut viewer, 180, 24);
    for text in [
        "ROOT CHARTER",
        "branch-k1",
        "first-k2",
        "unusual-kind",
        "DONE",
        "ABANDONED",
        "third-k4",
        "LIVE",
    ] {
        assert!(initial.contains(text), "missing {text}: {initial}");
    }
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 180, 24).contains("BRANCH CHARTER"));
    viewer.act(Action::Toggle);
    let collapsed = screen(&mut viewer, 180, 24);
    assert!(!collapsed.contains("first-k2"));
    assert!(collapsed.contains("DONE 1"));
    assert!(collapsed.contains("ABANDONED 1"));
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 180, 24).contains("THIRD BODY"));
    viewer.act(Action::Up);
    viewer.act(Action::Toggle);
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 180, 24).contains("FIRST BODY"));
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 180, 24).contains("ROOT CHARTER"));
    assert!(viewer.act(Action::Quit));
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn branch_aggregates_include_hidden_leaves_and_empty_branches() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "");
    put(&root, "01-k1/_done.md", "");
    put(&root, "01-k1/01-DONE-impl--done-k2.md", "");
    put(&root, "02-k3/_abandoned.md", "");
    put(&root, "02-k3/01-ABANDONED-impl--gone-k4.md", "");
    put(&root, "03-k5/_empty.md", "");
    let mut viewer = Viewer::new(work.path().into());
    let text = screen(&mut viewer, 220, 24);
    for (handle, status) in [
        ("root", "DONE"),
        ("done-k1", "DONE"),
        ("abandoned-k3", "ABANDONED"),
        ("empty-k5", "EMPTY"),
    ] {
        let row = text.lines().find(|row| row.contains(handle)).unwrap();
        assert!(row.contains(&format!("branch {status}")), "{row}");
    }
}

#[test]
fn reload_reports_stale_missing_and_malformed_states_and_recovers() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 150, 20).contains("Missing"));
    assert!(!root.exists());
    put(&root, "_BRIEF.md", "VALID ROOT");
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 150, 20).contains("VALID ROOT"));
    put(&root, "01-bad.md", "bad");
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 150, 20).contains("STALE"));
    fs::remove_file(root.join("01-bad.md")).unwrap();
    viewer.act(Action::Refresh);
    assert!(!screen(&mut viewer, 150, 20).contains("STALE"));
    fs::remove_dir_all(&root).unwrap();
    viewer.act(Action::Refresh);
    let missing = screen(&mut viewer, 150, 20);
    assert!(missing.contains("Missing"));
    assert!(!missing.contains("VALID ROOT"));
}

#[test]
fn file_errors_do_not_silently_select_a_different_file() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(&root, "01-impl--task-k1.md", "TASK BODY");
    let mut viewer = Viewer::new(work.path().into());
    fs::remove_file(root.join("01-impl--task-k1.md")).unwrap();
    viewer.act(Action::Down);
    let text = screen(&mut viewer, 180, 24);
    assert!(text.contains("File error"), "{text}");
    assert!(!text.contains("ROOT BODY"));
    put(&root, "01-impl--task-k1.md", "RECOVERED BODY");
    viewer.act(Action::Refresh);
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 180, 24).contains("RECOVERED BODY"));
}

#[test]
fn page_scroll_reaches_long_files_and_sanitizes_terminal_controls() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let content = (0..120)
        .map(|n| format!("source line {n:03}\n"))
        .collect::<String>();
    put(
        &root,
        "_BRIEF.md",
        &format!("\u{1b}[2J\u{7}\u{9}SAFE\u{9b}31m\n{content}"),
    );
    let mut viewer = Viewer::new(work.path().into());
    let first = screen(&mut viewer, 120, 15);
    assert!(first.contains("SAFE"));
    assert!(!first.contains('\u{1b}'));
    assert!(!first.contains('\u{7}'));
    assert!(!first.contains('\u{9b}'));
    viewer.act(Action::PageDown);
    assert!(!screen(&mut viewer, 120, 15).contains("source line 000"));
    for _ in 0..30 {
        viewer.act(Action::PageDown);
    }
    assert!(screen(&mut viewer, 120, 15).contains("source line 119"));
    for _ in 0..30 {
        viewer.act(Action::PageUp);
    }
    assert!(screen(&mut viewer, 120, 15).contains("SAFE"));
    for (w, h) in [(0, 0), (1, 1), (20, 3), (59, 9)] {
        screen(&mut viewer, w, h);
    }
}

#[test]
fn idle_viewers_release_the_shared_lock() {
    use std::os::fd::AsRawFd;
    let work = tempfile::tempdir().unwrap();
    put(&work.path().join(".grove"), "_BRIEF.md", "");
    let mut first = Viewer::new(work.path().into());
    let mut second = Viewer::new(work.path().into());
    screen(&mut first, 100, 20);
    screen(&mut second, 100, 20);
    let directory = fs::File::open(work.path()).unwrap();
    // SAFETY: the descriptor is live; closing it releases this test's lock.
    assert_eq!(
        unsafe { libc::flock(directory.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0
    );
}

/// The fallback release bounds even a regression to the blocking reader.
struct HeldWriter {
    release: Option<std::sync::mpsc::Sender<()>>,
    worker: Option<std::thread::JoinHandle<()>>,
}

impl Drop for HeldWriter {
    fn drop(&mut self) {
        drop(self.release.take());
        self.worker.take().unwrap().join().unwrap();
    }
}

fn hold_writer(worktree: &Path) -> HeldWriter {
    use std::os::fd::AsRawFd;
    let lock = fs::File::open(worktree).unwrap();
    // SAFETY: lock owns this independent open file description.
    assert_eq!(
        unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) },
        0
    );
    let (release, wait) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        let _ = wait.recv_timeout(std::time::Duration::from_secs(3));
        drop(lock);
    });
    HeldWriter {
        release: Some(release),
        worker: Some(worker),
    }
}

#[test]
fn busy_startup_is_visible_and_quit_remains_responsive() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    put(&work.path().join(".grove"), "_BRIEF.md", "CHARTER");
    let before = snapshot(work.path());
    let release = hold_writer(work.path());
    let start = Instant::now();
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 140, 20).contains("WAITING"));
    assert!(viewer.act(Action::Quit));
    assert!(start.elapsed() < Duration::from_secs(1));
    drop(release);
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn busy_selection_retries_the_latest_file_without_an_event_backlog() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "CHARTER");
    put(&root, "01-impl--first-k1.md", "FIRST FILE");
    put(&root, "02-impl--second-k2.md", "SECOND FILE");
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    let release = hold_writer(work.path());
    let start = Instant::now();
    viewer.act(Action::Down);
    viewer.act(Action::Down);
    assert!(start.elapsed() < Duration::from_secs(1));
    let waiting = screen(&mut viewer, 140, 20);
    assert!(waiting.contains("WAITING"));
    assert!(waiting.contains("CHARTER"));
    let deadline = start + Duration::from_millis(600);
    viewer.tick(deadline);
    assert_eq!(
        viewer.retry_after(deadline),
        Some(Duration::from_millis(500))
    );
    drop(release);
    viewer.tick(deadline + Duration::from_secs(1));
    assert!(screen(&mut viewer, 140, 20).contains("SECOND FILE"));
    assert_eq!(viewer.retry_after(deadline), None);
    put(&root, "02-impl--second-k2.md", "MANUAL ONLY");
    viewer.tick(deadline + Duration::from_secs(20));
    assert!(screen(&mut viewer, 140, 20).contains("SECOND FILE"));
    fs::write(root.join("02-impl--second-k2.md"), "SECOND FILE").unwrap();
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn busy_refresh_recovers_and_nonbusy_errors_wait_for_manual_repair() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "OLD CHARTER");
    let release = hold_writer(work.path());
    let mut viewer = Viewer::new(work.path().into());
    drop(release);
    viewer.tick(Instant::now() + Duration::from_secs(1));
    assert!(screen(&mut viewer, 140, 20).contains("OLD CHARTER"));
    let release = hold_writer(work.path());
    let start = Instant::now();
    viewer.act(Action::Refresh);
    assert!(start.elapsed() < Duration::from_secs(1));
    assert!(screen(&mut viewer, 140, 20).contains("OLD CHARTER"));
    put(&root, "_BRIEF.md", "NEW CHARTER");
    drop(release);
    viewer.tick(Instant::now() + Duration::from_secs(1));
    assert!(screen(&mut viewer, 140, 20).contains("NEW CHARTER"));
    put(&root, "01-bad.md", "malformed");
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 140, 20).contains("STALE"));
    assert_eq!(viewer.retry_after(Instant::now()), None);
    fs::remove_file(root.join("01-bad.md")).unwrap();
    viewer.tick(Instant::now() + Duration::from_secs(5));
    assert!(screen(&mut viewer, 140, 20).contains("STALE"));
    viewer.act(Action::Refresh);
    assert!(!screen(&mut viewer, 140, 20).contains("STALE"));
    fs::remove_dir_all(root).unwrap();
    viewer.act(Action::Refresh);
    let missing = screen(&mut viewer, 140, 20);
    assert!(missing.contains("Missing"));
    assert!(!missing.contains("NEW CHARTER"));
}

#[test]
fn removing_the_observed_worktree_clears_the_previous_display() {
    let parent = tempfile::tempdir().unwrap();
    let work = parent.path().join("work");
    put(&work.join(".grove"), "_BRIEF.md", "DELETED CHARTER");
    let mut viewer = Viewer::new(work.clone());
    assert!(screen(&mut viewer, 140, 20).contains("DELETED CHARTER"));
    fs::remove_dir_all(work).unwrap();
    viewer.act(Action::Refresh);
    let missing = screen(&mut viewer, 140, 20);
    assert!(missing.contains("Missing"));
    assert!(!missing.contains("DELETED CHARTER"));
}

fn key(viewer: &mut Viewer, code: crossterm::event::KeyCode) -> bool {
    use crossterm::event::{KeyEvent, KeyModifiers};
    Action::from_key(KeyEvent::new(code, KeyModifiers::NONE))
        .is_some_and(|action| viewer.act(action))
}

#[test]
fn keys_navigate_visible_rows_and_parents_and_keep_selection_visible() {
    use crossterm::event::KeyCode::*;
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(&root, "01-k1/_branch.md", "BRANCH BODY");
    put(&root, "01-k1/01-impl--child-k2.md", "CHILD BODY");
    for n in 2..30 {
        put(
            &root,
            &format!("{n:02}-impl--task-k{}.md", n + 1),
            &format!("BODY {n:02}"),
        );
    }
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    assert!(screen(&mut viewer, 160, 12).contains("Tree [focus]"));
    key(&mut viewer, Right);
    assert!(screen(&mut viewer, 160, 12).contains("BRANCH BODY"));
    key(&mut viewer, Char('h'));
    assert!(!screen(&mut viewer, 160, 12).contains("child-k2"));
    key(&mut viewer, Char('j'));
    assert!(screen(&mut viewer, 160, 12).contains("BODY 02"));
    key(&mut viewer, Char('k'));
    key(&mut viewer, Char('l'));
    assert!(screen(&mut viewer, 160, 12).contains("BRANCH BODY"));
    key(&mut viewer, Right);
    assert!(screen(&mut viewer, 160, 12).contains("CHILD BODY"));
    key(&mut viewer, Left);
    assert!(screen(&mut viewer, 160, 12).contains("BRANCH BODY"));
    key(&mut viewer, Char(' '));
    assert!(!screen(&mut viewer, 160, 12).contains("child-k2"));
    key(&mut viewer, Enter);
    assert!(screen(&mut viewer, 160, 12).contains("child-k2"));
    key(&mut viewer, End);
    let last = screen(&mut viewer, 160, 12);
    assert!(
        last.lines()
            .any(|line| line.contains('>') && line.contains("task-k30")),
        "{last}"
    );
    assert!(last.contains("BODY 29"));
    key(&mut viewer, Home);
    assert!(screen(&mut viewer, 160, 12).contains("ROOT BODY"));
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn focused_file_keys_scroll_lines_pages_and_unicode_columns() {
    use crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let content = (0..50)
        .map(|n| format!("line {n:02}\n"))
        .collect::<String>();
    put(&root, "_BRIEF.md", &content);
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 100, 15); // ten file rows
    key(&mut viewer, Tab);
    key(&mut viewer, Down);
    let line = screen(&mut viewer, 100, 15);
    assert!(line.contains("File [focus]"));
    assert!(!line.contains("line 00"));
    assert!(line.contains("line 01"));
    key(&mut viewer, Up);
    assert!(screen(&mut viewer, 100, 15).contains("line 00"));
    for (code, expected) in [('d', "line 10"), ('u', "line 00")] {
        let action = Action::from_key(KeyEvent::new(Char(code), KeyModifiers::CONTROL)).unwrap();
        viewer.act(action);
        assert!(screen(&mut viewer, 100, 15).contains(expected));
    }
    key(&mut viewer, End);
    assert!(screen(&mut viewer, 100, 15).contains("line 49"));
    let taller = screen(&mut viewer, 100, 25);
    assert!(taller.contains("line 30"));
    assert!(taller.contains("line 49"));
    key(&mut viewer, Home);
    assert!(screen(&mut viewer, 100, 15).contains("line 00"));
    assert_eq!(snapshot(work.path()), before);

    put(
        &root,
        "_BRIEF.md",
        &format!("界e\u{301}{}TAIL", "界".repeat(40)),
    );
    key(&mut viewer, Char('r'));
    for _ in 0..200 {
        key(&mut viewer, Right);
    }
    let wide = screen(&mut viewer, 100, 15);
    assert!(wide.contains("TAIL"), "{wide}");
    for _ in 0..200 {
        key(&mut viewer, Left);
    }
    let start = screen(&mut viewer, 100, 15);
    // TestBackend stores an empty continuation cell after the wide character.
    assert!(start.contains("界 e\u{301}"), "{start}");
    key(&mut viewer, Right); // begins inside a two-cell character
    assert!(screen(&mut viewer, 100, 15).contains(" e\u{301}"));
}

#[test]
fn help_and_small_frames_preserve_navigation_and_allow_global_actions() {
    use crossterm::event::{KeyCode::*, KeyEvent, KeyModifiers};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(
        &root,
        "01-impl--task-k1.md",
        &(0..40)
            .map(|n| format!("task line {n:02}\n"))
            .collect::<String>(),
    );
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 100, 15);
    key(&mut viewer, Down);
    key(&mut viewer, Tab);
    key(&mut viewer, PageDown);
    let saved = screen(&mut viewer, 100, 15);
    key(&mut viewer, Char('?'));
    let help = screen(&mut viewer, 60, 10);
    for expected in ["Key help", "Ctrl-u", "Escape"] {
        assert!(help.contains(expected), "{help}");
    }
    key(&mut viewer, Home);
    key(&mut viewer, Tab);
    key(&mut viewer, Esc);
    assert_eq!(screen(&mut viewer, 100, 15), saved);
    for (w, h) in [(0, 0), (1, 1), (59, 10), (60, 9)] {
        let small = screen(&mut viewer, w, h);
        if w > 1 {
            assert!(small.contains("Resize"));
        }
        key(&mut viewer, Home);
        key(&mut viewer, Tab);
        assert_eq!(screen(&mut viewer, 100, 15), saved);
    }
    key(&mut viewer, Char('?'));
    assert!(key(&mut viewer, Char('q')));
    assert!(viewer.act(Action::from_key(KeyEvent::new(Char('c'), KeyModifiers::CONTROL)).unwrap()));
    screen(&mut viewer, 59, 9);
    key(&mut viewer, Char('r'));
    key(&mut viewer, Esc);
    assert!(screen(&mut viewer, 100, 15).contains("ROOT BODY"));
    assert_eq!(snapshot(work.path()), before);
}

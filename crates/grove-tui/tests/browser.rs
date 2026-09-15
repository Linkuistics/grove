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

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
    assert!(screen(&mut viewer, 180, 24).contains("FIRST BODY"));
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
fn disappearance_selects_root_with_an_explicit_notice() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(&root, "01-impl--task-k1.md", "TASK BODY");
    let mut viewer = Viewer::new(work.path().into());
    fs::remove_file(root.join("01-impl--task-k1.md")).unwrap();
    viewer.act(Action::Down);
    let text = screen(&mut viewer, 180, 24);
    assert!(text.contains("disappeared"), "{text}");
    assert!(text.contains("ROOT BODY"));
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
    assert_eq!(
        viewer.retry_after(deadline + Duration::from_secs(1)),
        Some(Duration::from_millis(500))
    );
    put(&root, "02-impl--second-k2.md", "AUTOMATIC EDIT");
    viewer.tick(deadline + Duration::from_secs(20));
    assert!(screen(&mut viewer, 140, 20).contains("AUTOMATIC EDIT"));
    fs::write(root.join("02-impl--second-k2.md"), "SECOND FILE").unwrap();
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn busy_refresh_and_nonbusy_errors_recover_automatically() {
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
    assert!(viewer.retry_after(Instant::now()).is_some());
    fs::remove_file(root.join("01-bad.md")).unwrap();
    viewer.tick(Instant::now() + Duration::from_secs(5));
    assert!(!screen(&mut viewer, 140, 20).contains("STALE"));
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
    put(&root, "_BRIEF.md", &format!("```\n{content}```"));
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
        &format!("```\n界e\u{301}{}TAIL\n```", "界".repeat(40)),
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
    assert_eq!(screen(&mut viewer, 100, 15), saved);
    assert_eq!(snapshot(work.path()), before);
}

// Removing parser styles or treating list/table syntax as text breaks this seam.
#[test]
fn markdown_formats_real_task_and_branch_documents_without_writes() {
    use ratatui::style::{Color, Modifier};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let markdown = "# Heading\n\n## Subheading\n\n**bold** *italic* `inline`\n\n- outer\n  3. nested\n     - inner\n\n> quoted\n\n---\n\n```rust\n    let x = 1;\n```\n\n    indented\n\n| Name | Count |\n| :--- | ---: |\n| a | 12 |\n| longer | 3 |\n\n[label](https://example.test) ![alt text](https://image.test)\n\n<div>inert</div>\n";
    put(&root, "_BRIEF.md", "root");
    put(&root, "01-k1/_branch.md", markdown);
    put(&root, "01-k1/01-impl--task-k2.md", markdown);
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    for _ in 0..2 {
        viewer.act(Action::Down);
        let text = screen(&mut viewer, 220, 55);
        for expected in [
            "Heading",
            "## Subheading",
            "bold italic inline",
            "• outer",
            "3. nested",
            "• inner",
            "│ quoted",
            "────",
            "    let x = 1;",
            "indented",
            "longer",
            "label (https://example.test)",
            "alt text",
            "<div>inert</div>",
        ] {
            assert!(text.contains(expected), "missing {expected}: {text}");
        }
        assert!(!text.contains("**bold**"));
        assert!(!text.contains("![alt"));
        let mut terminal = Terminal::new(TestBackend::new(220, 55)).unwrap();
        terminal.draw(|f| viewer.render(f)).unwrap();
        let buffer = terminal.backend().buffer();
        for (word, modifier) in [
            ("Heading", Modifier::BOLD),
            ("bold", Modifier::BOLD),
            ("italic", Modifier::ITALIC),
        ] {
            let cells = buffer
                .content
                .chunks(220)
                .find_map(|row| {
                    row.windows(word.len())
                        .find(|cells| cells.iter().map(|c| c.symbol()).collect::<String>() == word)
                })
                .unwrap();
            assert!(
                cells.iter().all(|c| c.modifier.contains(modifier)),
                "{word}"
            );
        }
        assert!(buffer
            .content
            .iter()
            .any(|c| c.symbol() == "i" && c.fg == Color::Yellow));
        let table: Vec<_> = text
            .lines()
            .filter(|l| l.contains("│ Name") || l.contains("│ a ") || l.contains("│ longer"))
            .collect();
        assert_eq!(table.len(), 3, "{text}");
        for row in [
            "│ Name   │ Count │",
            "│ a      │    12 │",
            "│ longer │     3 │",
        ] {
            assert!(text.contains(row), "unaligned table: {text}");
        }
    }
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn markdown_reflow_and_revisits_keep_the_visible_source_marker() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let repeated =
        "repeated words fill a paragraph with ordinary prose and enough text to reflow.\n\n";
    let text = format!(
        "{}UNIQUE-MARKER here is the reading position.\n\n{}",
        repeated.repeat(20),
        repeated.repeat(30)
    );
    put(&root, "_BRIEF.md", &text);
    put(&root, "01-impl--task-k1.md", "other document");
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    // Put the marker at the top, not merely at an arbitrary visible row.
    for _ in 0..200 {
        let view = screen(&mut viewer, 160, 15);
        if view.lines().nth(3).unwrap().contains("UNIQUE-MARKER") {
            break;
        }
        viewer.act(Action::Down);
    }
    assert!(screen(&mut viewer, 160, 15)
        .lines()
        .nth(3)
        .unwrap()
        .contains("UNIQUE-MARKER"));
    assert!(screen(&mut viewer, 60, 15).contains("UNIQUE-MARKER"));
    viewer.act(Action::Focus);
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 60, 15).contains("other document"));
    viewer.act(Action::Up);
    assert!(screen(&mut viewer, 120, 15).contains("UNIQUE-MARKER"));
    assert_eq!(snapshot(work.path()), before);
}

fn scroll_to(viewer: &mut Viewer, marker: &str) {
    for _ in 0..300 {
        if screen(viewer, 160, 15)
            .lines()
            .nth(3)
            .unwrap()
            .contains(marker)
        {
            return;
        }
        viewer.act(Action::Down);
    }
    panic!("marker never reached top: {marker}");
}

#[test]
fn edited_reading_positions_follow_source_on_refresh_and_revisit() {
    for revisit in [false, true] {
        let work = tempfile::tempdir().unwrap();
        let root = work.path().join(".grove");
        let text = format!(
            "{}MARKER here\n\n{}",
            "before\n\n".repeat(25),
            "after\n\n".repeat(30)
        );
        put(&root, "_BRIEF.md", &text);
        put(&root, "01-impl--other-k1.md", "other document");
        let mut viewer = Viewer::new(work.path().into());
        screen(&mut viewer, 160, 15);
        viewer.act(Action::Focus);
        scroll_to(&mut viewer, "MARKER");
        if revisit {
            viewer.act(Action::Focus);
            viewer.act(Action::Down);
            assert!(screen(&mut viewer, 160, 15).contains("other document"));
        }
        put(
            &root,
            "_BRIEF.md",
            &format!("{}{}", "inserted\n\n".repeat(20), text),
        );
        let expected = snapshot(work.path());
        if revisit {
            viewer.act(Action::Up);
        } else {
            viewer.act(Action::Refresh);
        }
        let view = screen(&mut viewer, 160, 15);
        assert!(view.lines().nth(3).unwrap().contains("MARKER"), "{view}");
        assert!(screen(&mut viewer, 70, 15).contains("MARKER"));
        assert_eq!(snapshot(work.path()), expected);
    }
}

#[test]
fn edited_duplicate_lines_follow_their_surrounding_passage() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let first = "first context\nDUPLICATE\nfirst continuation\n";
    let second = "second context\nDUPLICATE\nsecond continuation\n";
    let tail = "tail\n".repeat(30);
    put(
        &root,
        "_BRIEF.md",
        &format!("```\n{first}{}{second}{tail}```", "gap\n".repeat(20)),
    );
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "second context");
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 160, 15)
        .lines()
        .nth(3)
        .unwrap()
        .contains("DUPLICATE"));
    // Change both ends so matching cannot rely on an unchanged suffix alone.
    put(
        &root,
        "_BRIEF.md",
        &format!(
            "```\nnew start\n{first}{}{second}{tail}new end\n```",
            "gap\n".repeat(60)
        ),
    );
    let expected = snapshot(work.path());
    viewer.act(Action::Refresh);
    for _ in 0..2 {
        let view = screen(&mut viewer, 160, 15);
        assert!(view.lines().nth(3).unwrap().contains("DUPLICATE"), "{view}");
        assert!(
            view.lines().nth(4).unwrap().contains("second continuation"),
            "{view}"
        );
        viewer.act(Action::Refresh);
    }
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn deleted_reading_line_uses_nearest_survivor_then_empty_content_clamps() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let prefix = "before\n".repeat(25);
    let tail = "after\n".repeat(30);
    put(
        &root,
        "_BRIEF.md",
        &format!("```\n{prefix}DELETE ME\nNEXT SURVIVOR\n{tail}```"),
    );
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "DELETE ME");
    put(
        &root,
        "_BRIEF.md",
        &format!("```\nnew\n{prefix}NEXT SURVIVOR\n{tail}```"),
    );
    let expected = snapshot(work.path());
    viewer.act(Action::Refresh);
    let view = screen(&mut viewer, 160, 15);
    assert!(
        view.lines().nth(3).unwrap().contains("NEXT SURVIVOR"),
        "{view}"
    );
    assert_eq!(snapshot(work.path()), expected);
    for replacement in ["", "短い新規文書", "short replacement"] {
        put(&root, "_BRIEF.md", replacement);
        let expected = snapshot(work.path());
        viewer.act(Action::Refresh);
        let view = screen(&mut viewer, 160, 15);
        let shown = view.lines().nth(3).unwrap().replace(' ', "");
        assert!(shown.contains(&replacement.replace(' ', "")), "{view}");
        assert!(!view.contains("NEXT SURVIVOR"));
        assert_eq!(snapshot(work.path()), expected);
    }
}

#[test]
fn edited_repeated_block_keeps_its_relative_reading_position() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let text = format!(
        "```\nSTART\n{}END MARKER\n{}old ending\n```",
        "repeat\n".repeat(30),
        "tail\n".repeat(30)
    );
    put(&root, "_BRIEF.md", &text);
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "END MARKER");
    for _ in 0..7 {
        viewer.act(Action::Up);
    }
    let before = screen(&mut viewer, 160, 15);
    assert!(before.lines().nth(10).unwrap().contains("END MARKER"));
    put(
        &root,
        "_BRIEF.md",
        &format!("inserted\n\n{}", text.replace("old ending", "new ending")),
    );
    let expected = snapshot(work.path());
    viewer.act(Action::Refresh);
    assert_eq!(screen(&mut viewer, 160, 15), before);
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn deleted_duplicate_does_not_steal_an_unchanged_occurrence() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let text = format!(
        "```\nA\nDUPLICATE\nB\nDUPLICATE\nC SURVIVOR\n{}```",
        "tail\n".repeat(30)
    );
    put(&root, "_BRIEF.md", &text);
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "C SURVIVOR");
    viewer.act(Action::Up);
    put(&root, "_BRIEF.md", &text.replace("B\nDUPLICATE\n", "B\n"));
    let expected = snapshot(work.path());
    viewer.act(Action::Refresh);
    let view = screen(&mut viewer, 160, 15);
    assert!(
        view.lines().nth(3).unwrap().contains("C SURVIVOR"),
        "{view}"
    );
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn edited_file_errors_keep_the_saved_source_until_recovery_and_revisit() {
    use std::os::unix::fs::PermissionsExt;
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let text = format!(
        "{}MARKER here\n\n{}",
        "before\n\n".repeat(25),
        "after\n\n".repeat(30)
    );
    put(&root, "_BRIEF.md", &text);
    put(&root, "01-impl--other-k1.md", "other document");
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "MARKER");
    let file = root.join("_BRIEF.md");
    fs::set_permissions(&file, fs::Permissions::from_mode(0o0)).unwrap();
    let denied = fs::read(&file).is_err();
    viewer.act(Action::Refresh);
    let error = screen(&mut viewer, 160, 15);
    screen(&mut viewer, 70, 15);
    viewer.act(Action::Focus);
    viewer.act(Action::Down);
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    if denied {
        assert!(error.contains("File error"), "{error}");
    }
    put(
        &root,
        "_BRIEF.md",
        &format!("{}{}", "inserted\n\n".repeat(20), text),
    );
    let expected = snapshot(work.path());
    viewer.act(Action::Up);
    let view = screen(&mut viewer, 160, 15);
    assert!(view.lines().nth(3).unwrap().contains("MARKER"), "{view}");
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn markdown_wide_blocks_partial_edits_unicode_and_empty_files_are_readable() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(
        &root,
        "_BRIEF.md",
        &format!(
            "```\n界e\u{301}{}CODETAIL\n```\n\n| Wide |\n| --- |\n| {}TABLETAIL |\n",
            "x".repeat(80),
            "x".repeat(80)
        ),
    );
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 60, 20);
    viewer.act(Action::Focus);
    for _ in 0..200 {
        viewer.act(Action::Right);
    }
    let view = screen(&mut viewer, 60, 20);
    assert!(view.contains("CODETAIL"), "{view}");
    assert!(view.contains("TABLETAIL"), "{view}");
    for _ in 0..200 {
        viewer.act(Action::Left);
    }
    assert!(screen(&mut viewer, 60, 20).contains("界 e\u{301}"));
    assert_eq!(snapshot(work.path()), before);
    for text in [
        "",
        "**unfinished [link\n\n```\n  partial",
        "Unicode 界 e\u{301} prose\n\n&#27;[2J\u{1b}[2J",
    ] {
        put(&root, "_BRIEF.md", text);
        let expected = snapshot(work.path());
        viewer.act(Action::Refresh);
        let view = screen(&mut viewer, 60, 20);
        assert!(!view.contains('\u{1b}'));
        if text.contains("partial") {
            assert!(view.contains("  partial"));
        }
        if text.contains("Unicode") {
            assert!(view.contains("界  e\u{301}"), "{view}");
        }
        assert_eq!(snapshot(work.path()), expected);
    }
}

#[test]
fn transformed_markdown_and_blank_code_lines_keep_reading_anchors() {
    for body in [
        format!(
            "`{}UNIQUE-MARKER {} `",
            "alpha bravo ".repeat(60),
            "charlie delta ".repeat(60)
        ),
        format!(
            "[label](https://example.test/{}UNIQUE-MARKER/{})",
            "part/".repeat(147),
            "tail/".repeat(150)
        ),
        format!(
            "```\nalpha\n{}UNIQUE-MARKER\n{}\n```",
            "\n".repeat(20),
            "tail\n".repeat(30)
        ),
    ] {
        let work = tempfile::tempdir().unwrap();
        let root = work.path().join(".grove");
        put(&root, "_BRIEF.md", &body);
        put(&root, "01-impl--other-k1.md", "other");
        let mut viewer = Viewer::new(work.path().into());
        screen(&mut viewer, 100, 15);
        viewer.act(Action::Focus);
        let mut found = false;
        for _ in 0..250 {
            let view = screen(&mut viewer, 100, 15);
            if view.lines().nth(3).unwrap().contains("UNIQUE-MARKER") {
                found = true;
                break;
            }
            viewer.act(Action::Down);
        }
        assert!(found, "marker not reachable: {body}");
        put(
            &root,
            "_BRIEF.md",
            &format!("{}{}", "inserted\n\n".repeat(20), body),
        );
        let expected = snapshot(work.path());
        viewer.act(Action::Refresh);
        let edited = screen(&mut viewer, 100, 15);
        assert!(
            edited.lines().nth(3).unwrap().contains("UNIQUE-MARKER"),
            "{edited}"
        );
        viewer.act(Action::Focus);
        viewer.act(Action::Down);
        screen(&mut viewer, 100, 15);
        viewer.act(Action::Up);
        let revisited = screen(&mut viewer, 100, 15);
        assert!(
            revisited.lines().nth(3).unwrap().contains("UNIQUE-MARKER"),
            "{revisited}"
        );
        assert!(screen(&mut viewer, 70, 20).contains("UNIQUE-MARKER"));
        assert_eq!(snapshot(work.path()), expected);
    }
}

#[test]
fn edited_wide_code_preserves_horizontal_offset_and_item_positions() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let lines: String = (0..80)
        .map(|n| format!("WIDE {n:03} {} TAIL {n:03}\n", "wide ".repeat(30)))
        .collect();
    let body = format!("```\n{lines}```");
    put(&root, "_BRIEF.md", &body);
    put(&root, "01-impl--other-k1.md", "other starts at top");
    let mut viewer = Viewer::new(work.path().into());
    screen(&mut viewer, 160, 15);
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "WIDE 020");
    for _ in 0..15 {
        viewer.act(Action::Right);
    }
    let before = screen(&mut viewer, 160, 15);
    assert!(!before.contains("WIDE 020"));
    viewer.act(Action::Focus);
    viewer.act(Action::Down);
    assert!(screen(&mut viewer, 160, 15).contains("other starts at top"));
    put(
        &root,
        "_BRIEF.md",
        &format!("{}{}", "inserted\n\n".repeat(20), body),
    );
    let expected = snapshot(work.path());
    viewer.act(Action::Up);
    viewer.act(Action::Focus);
    assert_eq!(screen(&mut viewer, 160, 15), before);
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn live_ticks_follow_keys_and_reveal_a_moved_selection() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(&root, "01-k1/_branch.md", "BRANCH BODY");
    put(&root, "01-k1/01-impl--hidden-k2.md", "HIDDEN BODY");
    put(&root, "02-impl--chosen-k3.md", "CHOSEN BODY");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    viewer.act(Action::Toggle);
    viewer.act(Action::Down);
    viewer.act(Action::Focus);
    assert!(screen(&mut viewer, 180, 24).contains("CHOSEN BODY"));
    fs::rename(
        root.join("02-impl--chosen-k3.md"),
        root.join("01-k1/02-DONE-impl--renamed-k3.md"),
    )
    .unwrap();
    let expected = snapshot(work.path());
    viewer.tick(Instant::now() + Duration::from_secs(1));
    let shown = screen(&mut viewer, 180, 24);
    assert!(shown.contains("renamed-k3 impl DONE"), "{shown}");
    assert!(shown.contains("CHOSEN BODY"), "{shown}");
    assert!(!shown.contains("HIDDEN BODY"));
    assert!(
        shown.contains("hidden-k2"),
        "moved selection must reveal ancestor: {shown}"
    );
    assert!(shown.contains("File [focus]"));
    assert_eq!(snapshot(work.path()), expected);
    fs::remove_file(root.join("01-k1/02-DONE-impl--renamed-k3.md")).unwrap();
    viewer.act(Action::Refresh);
    let shown = screen(&mut viewer, 180, 24);
    assert!(shown.contains("disappeared"), "{shown}");
    assert!(shown.contains("BRANCH BODY"), "{shown}");
}

#[test]
fn live_root_replacement_clears_reused_key_state_but_brief_edits_do_not() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "OLD ROOT");
    put(&root, "01-impl--chosen-k1.md", "OLD ITEM");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    put(&root, "replacement", "EDITED ROOT");
    fs::rename(root.join("replacement"), root.join("_BRIEF.md")).unwrap();
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 160, 20).contains("OLD ITEM"));
    fs::rename(&root, work.path().join("old-tree")).unwrap();
    put(&root, "_BRIEF.md", "NEW ROOT");
    put(&root, "01-impl--chosen-k1.md", "NEW ITEM");
    let expected = snapshot(work.path());
    viewer.act(Action::Refresh);
    let shown = screen(&mut viewer, 160, 20);
    assert!(shown.contains("NEW ROOT"), "{shown}");
    assert!(!shown.contains("NEW ITEM"));
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn live_decomposition_and_duplicate_key_recovery() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(&root, "01-impl--chosen-k1.md", "CHOSEN BODY");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    fs::create_dir(root.join("01-k1")).unwrap();
    fs::rename(
        root.join("01-impl--chosen-k1.md"),
        root.join("01-k1/_chosen.md"),
    )
    .unwrap();
    viewer.act(Action::Refresh);
    let shown = screen(&mut viewer, 160, 20);
    assert!(shown.contains("CHOSEN BODY"), "{shown}");
    assert!(shown.contains(">   - chosen-k1"), "{shown}");
    put(&root, "02-impl--duplicate-k1.md", "WRONG BODY");
    viewer.act(Action::Refresh);
    let shown = screen(&mut viewer, 180, 20);
    assert!(shown.contains("duplicate key"), "{shown}");
    assert!(shown.contains("STALE"));
    assert!(!shown.contains("WRONG BODY"));
    fs::remove_file(root.join("02-impl--duplicate-k1.md")).unwrap();
    viewer.tick(std::time::Instant::now() + std::time::Duration::from_secs(1));
    assert!(!screen(&mut viewer, 180, 20).contains("STALE"));
}

#[test]
fn live_unchanged_ticks_keep_reading_position_and_selected_bytes_are_reread() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    let text: String = (0..100).map(|n| format!("ROW {n:03}\n\n")).collect();
    put(&root, "01-impl--chosen-k1.md", &text);
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    screen(&mut viewer, 160, 20);
    viewer.act(Action::Focus);
    viewer.act(Action::PageDown);
    let before = screen(&mut viewer, 160, 20);
    viewer.tick(Instant::now() + Duration::from_secs(1));
    assert_eq!(screen(&mut viewer, 160, 20), before);
    let file = root.join("01-impl--chosen-k1.md");
    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    fs::write(&file, text.replace("ROW", "NEW")).unwrap();
    fs::File::options()
        .write(true)
        .open(&file)
        .unwrap()
        .set_times(fs::FileTimes::new().set_modified(modified))
        .unwrap();
    let expected = snapshot(work.path());
    viewer.tick(Instant::now() + Duration::from_secs(2));
    assert!(screen(&mut viewer, 160, 20).contains("NEW"));
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn live_unreadable_replacement_discards_the_previous_lifetime() {
    use std::os::unix::fs::PermissionsExt;
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "PRIVATE OLD ROOT");
    let mut viewer = Viewer::new(work.path().into());
    fs::rename(&root, work.path().join("old")).unwrap();
    put(&root, "_BRIEF.md", "REPLACEMENT ROOT");
    fs::set_permissions(&root, fs::Permissions::from_mode(0o0)).unwrap();
    let denied = fs::File::open(&root).is_err();
    viewer.act(Action::Refresh);
    let shown = screen(&mut viewer, 180, 20);
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    assert!(!shown.contains("PRIVATE OLD ROOT"), "{shown}");
    if denied {
        assert!(shown.contains("Error"), "{shown}");
    }
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 180, 20).contains("REPLACEMENT ROOT"));
}

#[test]
fn live_fifo_replacement_cannot_block_observation() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "OLD ROOT");
    let mut viewer = Viewer::new(work.path().into());
    fs::remove_dir_all(&root).unwrap();
    use std::os::unix::ffi::OsStrExt;
    let fifo_name = std::ffi::CString::new(root.as_os_str().as_bytes()).unwrap();
    // SAFETY: a live NUL-terminated path and ordinary owner read/write mode.
    assert_eq!(unsafe { libc::mkfifo(fifo_name.as_ptr(), 0o600) }, 0);
    // Bound the old blocking implementation so this regression fails instead
    // of hanging the suite. RDWR opens a FIFO without waiting for a peer.
    let fifo = root.clone();
    let release = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(400));
        fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(fifo)
            .unwrap()
    });
    let start = Instant::now();
    viewer.act(Action::Refresh);
    let elapsed = start.elapsed();
    let writer = release.join().unwrap();
    drop(writer);
    assert!(
        elapsed < Duration::from_millis(200),
        "blocked for {elapsed:?}"
    );
    assert!(!screen(&mut viewer, 160, 20).contains("OLD ROOT"));
    assert!(viewer.act(Action::Quit));
}

#[test]
fn live_file_read_errors_keep_the_tree_and_recover_the_saved_anchor() {
    use std::os::unix::fs::PermissionsExt;
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    let text: String = (0..100).map(|n| format!("LINE {n:03}\n\n")).collect();
    put(&root, "01-impl--chosen-k1.md", &text);
    let file = root.join("01-impl--chosen-k1.md");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    screen(&mut viewer, 180, 24);
    viewer.act(Action::Focus);
    viewer.act(Action::PageDown);
    let before = screen(&mut viewer, 180, 24);
    fs::set_permissions(&file, fs::Permissions::from_mode(0o0)).unwrap();
    let denied = fs::read(&file).is_err();
    viewer.act(Action::Refresh);
    let error = screen(&mut viewer, 180, 24);
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    if denied {
        assert!(error.contains("File error"), "{error}");
        assert!(error.contains("chosen-k1"));
        assert!(error.contains("denied"), "{error}");
        assert!(!error.contains("ROOT BODY"));
    }
    viewer.tick(Instant::now() + Duration::from_secs(1));
    assert_eq!(screen(&mut viewer, 180, 24), before);
}

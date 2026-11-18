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

// Visit the selected file and return to Tree without changing selection.
fn visit_file(viewer: &mut Viewer, width: u16, height: u16) -> String {
    viewer.act(Action::Focus);
    let file = screen(viewer, width, height);
    viewer.act(Action::Focus);
    file
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
fn idle_next_survives_folding_file_view_and_minimum_chrome() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT BODY");
    put(&root, "01-finish--finish-k1.md", "");
    put(&root, "02-k2/_branch.md", "BRANCH BODY");
    put(&root, "02-k2/01-impl--forecast-k3.md", "TASK BODY");
    put(&root, "03-impl--later-k4.md", "");
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    for file in [false, true] {
        if file {
            viewer.act(Action::Focus);
        }
        let shown = screen(&mut viewer, 60, 10);
        assert!(
            shown.lines().nth(2).unwrap().starts_with("► RUNNING: none"),
            "{shown}"
        );
        assert!(
            shown
                .lines()
                .nth(3)
                .unwrap()
                .starts_with("» NEXT: forecast-k3"),
            "{shown}"
        );
        assert!(shown.lines().nth(4).unwrap().ends_with('┐'), "{shown}");
        assert!(shown.lines().nth(8).unwrap().ends_with('┘'), "{shown}");
    }
    viewer.act(Action::Focus);
    viewer.act(Action::Toggle); // Fold the root; NEXT remains in chrome.
    let shown = screen(&mut viewer, 60, 10);
    assert!(shown.contains("NEXT: forecast-k3"), "{shown}");
    assert!(!shown
        .lines()
        .skip(4)
        .any(|line| line.contains("forecast-k3")));
    assert_eq!(snapshot(work.path()), before);

    fs::remove_file(root.join("02-k2/01-impl--forecast-k3.md")).unwrap();
    fs::remove_file(root.join("03-impl--later-k4.md")).unwrap();
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10).contains("NEXT: finish-k1"));
    fs::remove_file(root.join("01-finish--finish-k1.md")).unwrap();
    let empty = snapshot(work.path());
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10).contains("NEXT: none"));
    assert_eq!(snapshot(work.path()), empty); // Never allocate finish.
}

#[test]
fn missing_tree_preserves_runtime_observation_and_recovers() {
    for initial_tree in [false, true] {
        let work = tempfile::tempdir().unwrap();
        if initial_tree {
            put(work.path(), ".grove/_BRIEF.md", "ROOT");
        }
        runtime_records(work.path(), false);
        let mut viewer = Viewer::new(work.path().into());
        if initial_tree {
            viewer.act(Action::Focus);
            fs::remove_dir_all(work.path().join(".grove")).unwrap();
        }
        for active in [false, true, false] {
            runtime_records(work.path(), active);
            let before = snapshot(work.path());
            viewer.act(Action::Refresh);
            let shown = screen(&mut viewer, 120, 15);
            assert!(shown.contains("Missing .grove"), "{shown}");
            assert!(shown.contains(if active {
                "RUNNING: unavailable — active epoch record has no supported observation witness"
            } else {
                "RUNNING: none (idle)"
            }), "{shown}");
            assert!(
                shown.contains("NEXT: unavailable — tree unavailable"),
                "{shown}"
            );
            assert_eq!(snapshot(work.path()), before);
        }
        put(work.path(), ".grove/_BRIEF.md", "ROOT");
        put(work.path(), ".grove/01-impl--next-k1.md", "");
        viewer.act(Action::Refresh);
        assert!(screen(&mut viewer, 60, 10).contains("NEXT: next-k1"));
    }
}

#[test]
fn root_open_error_preserves_idle_activity() {
    let work = tempfile::tempdir().unwrap();
    std::os::unix::fs::symlink(".grove", work.path().join(".grove")).unwrap();
    let mut viewer = Viewer::new(work.path().into());
    let shown = screen(&mut viewer, 100, 15);
    assert!(shown.contains("Error:"), "{shown}");
    assert!(shown.contains("RUNNING: none (idle)"), "{shown}");
    assert!(
        shown.contains("NEXT: unavailable — tree unavailable"),
        "{shown}"
    );
}

// Legacy records use the admission grammar, read by the real typed observer.
fn runtime_records(work: &Path, active: bool) {
    use std::os::unix::{ffi::OsStrExt, fs::MetadataExt};
    let metadata = fs::metadata(work).unwrap();
    let encoded: String = work
        .as_os_str()
        .as_bytes()
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    let process = format!("worktree-device={}\nworktree-inode={}\nworktree-path-hex={encoded}\nnonce=0123456789abcdef0123456789abcdef\n", metadata.dev(), metadata.ino());
    put(work, ".jj/grove/driver.lease", &process);
    let state = if active {
        "state=active\nlaunch-dir-hex=2f746d702f6c61756e6368\n"
    } else {
        "state=inactive\n"
    };
    put(
        work,
        ".jj/grove/session.epoch",
        &format!("{process}{state}"),
    );
}

#[test]
fn real_runtime_transitions_clear_next_and_retry_in_both_views() {
    use std::time::{Duration, Instant};
    for file in [false, true] {
        let work = tempfile::tempdir().unwrap();
        put(work.path(), ".grove/_BRIEF.md", "READABLE ROOT");
        put(work.path(), ".grove/01-impl--next-k1.md", "READABLE TASK");
        runtime_records(work.path(), false);
        let mut viewer = Viewer::new(work.path().into());
        if file {
            viewer.act(Action::Focus);
        }
        assert!(screen(&mut viewer, 100, 15).contains("NEXT: next-k1"));
        runtime_records(work.path(), true);
        let before = snapshot(work.path());
        viewer.act(Action::Help);
        viewer.tick(Instant::now() + Duration::from_secs(1));
        viewer.act(Action::Dismiss);
        let shown = screen(&mut viewer, 100, 15);
        assert!(shown.contains("RUNNING: unavailable"), "{shown}");
        assert!(shown.contains("NEXT: unavailable"), "{shown}");
        assert!(!shown.contains("NEXT: next-k1"));
        assert!(shown.contains(if file { "READABLE ROOT" } else { "next-k1" }));
        assert!(!shown.contains("STALE")); // Runtime failure did not reject the tree.
        assert!(!shown.lines().skip(4).any(|line| line.contains("»")));
        assert_eq!(snapshot(work.path()), before);

        screen(&mut viewer, 20, 5);
        runtime_records(work.path(), false);
        let before = snapshot(work.path());
        viewer.tick(Instant::now() + Duration::from_secs(2));
        assert!(screen(&mut viewer, 60, 10).contains("NEXT: next-k1"));
        assert_eq!(snapshot(work.path()), before);

        let holder = hold_writer(&work.path().join(".jj/grove/session.epoch"));
        let start = Instant::now();
        viewer.tick(start + Duration::from_secs(3));
        let shown = screen(&mut viewer, 100, 15);
        assert!(shown.contains("RUNNING: WAITING"), "{shown}");
        assert!(shown.contains("NEXT: WAITING"), "{shown}");
        assert!(!shown.lines().skip(4).any(|line| line.contains("»")));
        assert!(start.elapsed() < Duration::from_secs(1));
        assert!(viewer.act(Action::Quit));
        drop(holder);
        viewer.tick(Instant::now() + Duration::from_secs(4));
        assert!(screen(&mut viewer, 100, 15).contains("NEXT: next-k1"));

        put(work.path(), ".grove/02-impl--duplicate-k1.md", "");
        viewer.act(Action::Refresh);
        let shown = screen(&mut viewer, 100, 15);
        assert!(shown.contains("duplicate key k1"), "{shown}");
        assert!(shown.contains("RUNNING: none (idle)"), "{shown}");
        assert!(shown.contains("NEXT: unavailable"));
        assert!(!shown.lines().skip(4).any(|line| line.contains("»")));
    }
}

#[test]
fn aliases_and_multiple_viewers_only_read_exact_workspace_administration() {
    let work = tempfile::tempdir().unwrap();
    put(work.path(), ".grove/_BRIEF.md", "ROOT");
    put(work.path(), ".grove/01-impl--next-k1.md", "");
    runtime_records(work.path(), false);
    let alias_dir = tempfile::tempdir().unwrap();
    let alias = alias_dir.path().join("alias");
    std::os::unix::fs::symlink(work.path(), &alias).unwrap();
    let before = snapshot(work.path());
    let mut first = Viewer::new(work.path().into());
    let mut second = Viewer::new(alias);
    for viewer in [&mut first, &mut second] {
        assert!(screen(viewer, 60, 10).contains("NEXT: next-k1"));
        viewer.act(Action::Focus);
        viewer.act(Action::Refresh);
        assert!(screen(viewer, 60, 10).contains("ROOT"));
    }
    assert_eq!(snapshot(work.path()), before);
    runtime_records(work.path(), true);
    put(work.path(), "sub/.grove/_BRIEF.md", "SUB ROOT");
    put(work.path(), "sub/.grove/01-impl--sub-k2.md", "");
    let before = snapshot(work.path());
    let mut sub = Viewer::new(work.path().join("sub"));
    assert!(screen(&mut sub, 60, 10).contains("NEXT: sub-k2"));
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn long_next_keeps_its_key_offscreen_and_tree_contention_withholds_next() {
    let work = tempfile::tempdir().unwrap();
    put(work.path(), ".grove/_BRIEF.md", "ROOT");
    let name = format!(".grove/01-impl--{}-k4294967295.md", "long-name".repeat(12));
    put(work.path(), &name, "NEXT BODY");
    for key in 1..8 {
        put(
            work.path(),
            &format!(".grove/{:02}-impl--later-k{key}.md", key + 1),
            "",
        );
    }
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::End);
    let shown = screen(&mut viewer, 60, 10);
    let summary = shown.lines().nth(3).unwrap();
    assert!(summary.starts_with("» NEXT: long-name"), "{shown}");
    // The legend glyph costs slug text, never the permanent key.
    assert!(summary.trim_end().ends_with("…-k4294967295"), "{shown}");
    assert!(!shown
        .lines()
        .skip(4)
        .any(|line| line.contains("k4294967295")));
    let before = snapshot(work.path());
    let holder = hold_writer(work.path());
    viewer.act(Action::Refresh);
    let shown = screen(&mut viewer, 60, 10);
    assert!(shown.lines().nth(1).unwrap().contains("WAITING"));
    assert!(shown.contains("RUNNING: none (idle)"), "{shown}");
    assert!(
        shown.contains("NEXT: unavailable — tree unavailable"),
        "{shown}"
    );
    assert!(!shown.lines().skip(4).any(|line| line.contains("»")));
    drop(holder);
    viewer.act(Action::Refresh);
    assert!(screen(&mut viewer, 60, 10).contains("…-k4294967295"));
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn shared_validation_refuses_ambiguous_trees_without_writes_and_recovers() {
    for (names, diagnostic) in [
        (
            vec![
                "01-impl--work-k1.md",
                "02-DONE-impl--old-k7.md",
                "03-k7/_branch.md",
            ],
            "duplicate key k7",
        ),
        (
            vec!["01-finish--finish-k1.md", "02-finish--again-k2.md"],
            "multiple live `finish` leaves",
        ),
    ] {
        let work = tempfile::tempdir().unwrap();
        let root = work.path().join(".grove");
        put(&root, "_BRIEF.md", "ROOT");
        for name in &names {
            put(&root, name, "body");
        }
        let before = snapshot(work.path());
        let mut viewer = Viewer::new(work.path().into());
        let shown = screen(&mut viewer, 160, 20);
        assert!(shown.contains(diagnostic), "{shown}");
        viewer.act(Action::Focus);
        viewer.act(Action::Down);
        assert_eq!(snapshot(work.path()), before);

        // Restore an unambiguous shape; no new identity is allocated by viewing.
        let duplicate = root.join(names.last().unwrap());
        if duplicate.file_name().unwrap() == "_branch.md" {
            fs::remove_dir_all(duplicate.parent().unwrap()).unwrap();
        } else {
            fs::remove_file(duplicate).unwrap();
        }
        let repaired = snapshot(work.path());
        viewer.act(Action::Refresh);
        let shown = screen(&mut viewer, 160, 20);
        assert!(!shown.contains(diagnostic), "{shown}");
        assert_eq!(snapshot(work.path()), repaired);
    }
}

#[test]
fn full_width_switching_preserves_independent_views_and_poll_deadline() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let text = format!(
        "```\n{}```",
        (0..80)
            .map(|n| format!("ROW {n:03} {} TAIL\n", "wide ".repeat(30)))
            .collect::<String>()
    );
    put(&root, "_BRIEF.md", &text);
    for n in 1..30 {
        put(&root, &format!("{n:02}-impl--task-k{n}.md"), &text);
    }
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    let tree = screen(&mut viewer, 60, 10);
    assert!(tree.contains("Tree [active]"), "{tree}");
    assert!(tree.contains("Tab: File"));
    assert!(!tree.contains("ROW 000"));
    viewer.act(Action::End);
    let tree = screen(&mut viewer, 60, 10);
    let now = std::time::Instant::now();
    let deadline = viewer.retry_after(now);
    viewer.act(Action::PageDown); // Tree must not move the hidden file.
    viewer.act(Action::Focus);
    assert_eq!(viewer.retry_after(now), deadline);
    let file = screen(&mut viewer, 60, 10);
    assert!(file.contains("File [active]"), "{file}");
    assert!(file.contains("Tab: Tree"));
    assert!(file.contains("ROW 000"));
    assert!(!file.contains("task-k29"));
    assert_eq!(file.lines().nth(4).unwrap().chars().last(), Some('┐'));
    viewer.act(Action::PageDown);
    for _ in 0..12 {
        viewer.act(Action::Right);
    }
    let saved = screen(&mut viewer, 60, 10);
    viewer.act(Action::Focus);
    // Tree pages move the cursor, not the saved File. A page up and a page
    // down return to the same row and the same Tree screen.
    viewer.act(Action::PageUp);
    assert_ne!(screen(&mut viewer, 60, 10), tree);
    viewer.act(Action::PageDown);
    assert_eq!(screen(&mut viewer, 60, 10), tree);
    // A selection move observes the tree. Switching views after it must not.
    let deadline = viewer.retry_after(now);
    // Hidden File must not be clamped to the wider Tree dimensions.
    screen(&mut viewer, 220, 30);
    screen(&mut viewer, 60, 10);
    viewer.act(Action::Focus);
    assert_eq!(screen(&mut viewer, 60, 10), saved);
    assert_eq!(viewer.retry_after(now), deadline);
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn hidden_file_edits_and_resizes_keep_the_source_anchor() {
    use std::time::{Duration, Instant};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let text = format!(
        "{}MARKER reading position\n\n{}",
        "before words fill a paragraph that wraps at narrow widths\n\n".repeat(25),
        "after\n\n".repeat(30)
    );
    put(&root, "_BRIEF.md", &text);
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Focus);
    scroll_to(&mut viewer, "MARKER");
    viewer.act(Action::Focus);
    screen(&mut viewer, 60, 10);
    put(
        &root,
        "_BRIEF.md",
        &format!("{}{}", "inserted\n\n".repeat(20), text),
    );
    let expected = snapshot(work.path());
    // Observe in Tree; opening File reflows to the new viewport width.
    viewer.tick(Instant::now() + Duration::from_secs(1));
    viewer.act(Action::Focus);
    let narrow = screen(&mut viewer, 60, 10);
    assert!(
        narrow.lines().nth(5).unwrap().contains("MARKER"),
        "{narrow}"
    );
    let wide = screen(&mut viewer, 180, 24);
    assert!(wide.lines().nth(5).unwrap().contains("MARKER"), "{wide}");
    assert_eq!(snapshot(work.path()), expected);

    // Switching must not itself reload selected bytes.
    viewer.act(Action::Focus);
    put(&root, "_BRIEF.md", "NEW CONTENT");
    viewer.act(Action::Focus);
    assert!(screen(&mut viewer, 180, 24).contains("MARKER"));
    viewer.act(Action::Help);
    viewer.tick(Instant::now() + Duration::from_secs(2));
    viewer.act(Action::Dismiss);
    assert!(screen(&mut viewer, 180, 24).contains("NEW CONTENT"));
    screen(&mut viewer, 20, 5);
    put(&root, "_BRIEF.md", "SMALL FRAME EDIT");
    viewer.tick(Instant::now() + Duration::from_secs(3));
    assert!(screen(&mut viewer, 60, 10).contains("SMALL FRAME EDIT"));
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
        "branch-k1",
        "first-k2",
        "unusual-kind",
        "✓",
        "✗",
        "third-k4",
        "◦",
    ] {
        assert!(initial.contains(text), "missing {text}: {initial}");
    }
    assert!(visit_file(&mut viewer, 180, 24).contains("ROOT CHARTER"));
    viewer.act(Action::Down);
    assert!(visit_file(&mut viewer, 180, 24).contains("BRANCH CHARTER"));
    viewer.act(Action::Toggle);
    let collapsed = screen(&mut viewer, 180, 24);
    assert!(!collapsed.contains("first-k2"));
    assert!(collapsed.contains("branch-k1 ◦0 ✓1 ✗1"), "{collapsed}");
    viewer.act(Action::Down);
    assert!(visit_file(&mut viewer, 180, 24).contains("THIRD BODY"));
    viewer.act(Action::Up);
    viewer.act(Action::Toggle);
    viewer.act(Action::Down);
    assert!(visit_file(&mut viewer, 180, 24).contains("FIRST BODY"));
    viewer.act(Action::Refresh);
    assert!(visit_file(&mut viewer, 180, 24).contains("FIRST BODY"));
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
    for (handle, glyph, counts) in [
        ("root", "✓", "◦0 ✓1 ✗1"),
        ("done-k1", "✓", "◦0 ✓1 ✗0"),
        ("abandoned-k3", "✗", "◦0 ✓0 ✗1"),
        ("empty-k5", "∅", "◦0 ✓0 ✗0"),
    ] {
        let row = text.lines().find(|row| row.contains(handle)).unwrap();
        assert_eq!(row.chars().nth(6).unwrap().to_string(), glyph, "{row}");
        assert!(row.contains(&format!("{handle} {counts}")), "{row}");
    }
}

#[test]
fn glyph_rows_keep_depth_colors_and_bold_kinds_under_cursor_selection() {
    use ratatui::style::{Color, Modifier};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "");
    put(&root, "01-impl--live-k1.md", "");
    put(&root, "02-DONE-impl--done-k2.md", "");
    put(&root, "03-ABANDONED-impl--gone-k3.md", "");
    put(&root, "04-k4/_empty.md", "");
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    // Columns inside the border: depth 1..3, cursor 4, lifecycle 6, activity 8,
    // item from 10. The symbols alone are what a terminal without colour shows.
    for selected in 0..5 {
        let mut terminal = Terminal::new(TestBackend::new(100, 14)).unwrap();
        terminal.draw(|frame| viewer.render(frame)).unwrap();
        let buffer = terminal.backend().buffer();
        for (index, (depth, glyph, color, item)) in [
            (" 0", "◦", Color::Reset, "- root ◦1 ✓1 ✗1"),
            (" 1", "◦", Color::Reset, "    live-k1 impl"),
            (" 1", "✓", Color::Green, "    done-k2 impl"),
            (" 1", "✗", Color::Red, "    gone-k3 impl"),
            (" 1", "∅", Color::Reset, "  - empty-k4 ◦0 ✓0 ✗0"),
        ]
        .into_iter()
        .enumerate()
        {
            let cells = &buffer.content[(index + 5) * 100..(index + 6) * 100];
            let text: String = cells.iter().map(|cell| cell.symbol()).collect();
            let symbols = |range: std::ops::Range<usize>| {
                cells[range].iter().map(|c| c.symbol()).collect::<String>()
            };
            assert_eq!(symbols(1..3), depth, "{text}");
            assert_eq!(symbols(3..4), " ");
            assert_eq!(symbols(4..6), if selected == index { "> " } else { "  " });
            assert_eq!(symbols(6..8), format!("{glyph} "), "{text}");
            assert_eq!(symbols(8..10), if index == 1 { "» " } else { "  " });
            assert_eq!(symbols(10..10 + item.chars().count()), item, "{text}");
            // NEXT is bold in the normal foreground, in its glyph cell only.
            assert_eq!(cells[8].fg, Color::Reset);
            assert_eq!(cells[8].modifier.contains(Modifier::BOLD), index == 1);
            // The lifecycle glyph and the whole item share the lifecycle colour.
            let item_end = 10 + item.chars().count();
            for cell in std::iter::once(&cells[6]).chain(&cells[12..item_end]) {
                assert_eq!(cell.fg, color, "{text}");
            }
            // Only the kind is bold within the item; counts and handles are not.
            let kind_at = item.find(" impl").map(|at| 10 + at + 1);
            for (x, cell) in cells.iter().enumerate() {
                let bold =
                    (x == 8 && index == 1) || kind_at.is_some_and(|k| (k..k + 4).contains(&x));
                assert_eq!(
                    cell.modifier.contains(Modifier::BOLD),
                    bold,
                    "x {x}: {text}"
                );
                assert!(!cell.modifier.contains(Modifier::REVERSED), "{text}");
            }
            // Depth and cursor cells carry no colour of their own.
            assert!(cells[1..6].iter().all(|cell| cell.fg == Color::Reset));
        }
        viewer.act(Action::Down);
    }
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn deep_rows_keep_status_and_key_at_minimum_size() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "");
    let mut parent = root.clone();
    for key in 1..=15 {
        parent = parent.join(format!("01-k{key}"));
        put(&parent, "_branch.md", "");
    }
    let slug = "recognizable-long-name".repeat(6);
    put(
        &parent,
        &format!("01-ABANDONED-impl--{slug}-k4294967295.md"),
        "",
    );
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::End);
    let mut terminal = Terminal::new(TestBackend::new(60, 10)).unwrap();
    terminal.draw(|frame| viewer.render(frame)).unwrap();
    let cells = terminal
        .backend()
        .buffer()
        .content
        .chunks(60)
        .find(|row| row[4].symbol() == ">")
        .unwrap_or_else(|| panic!("{}", screen(&mut viewer, 60, 10)));
    // The fixed area is nine cells, so 49 of the 58 inner cells hold the item.
    assert_eq!(
        cells[1..10].iter().map(|c| c.symbol()).collect::<String>(),
        "16 > ✗   "
    );
    let item: String = cells[10..59].iter().map(|c| c.symbol()).collect();
    assert!(item.starts_with('…'), "{item}");
    assert!(item.contains("…-k4294967295"), "{item}");
    assert!(item.contains("rec…-k4294967295"), "{item}");
}

#[test]
fn depth_column_widens_for_every_row_beyond_ninety_nine() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "");
    let mut parent = root.clone();
    for key in 1..=99 {
        parent = parent.join(format!("01-k{key}"));
        put(&parent, "_b.md", "");
    }
    put(&parent, "01-impl--deep-k100.md", "");
    let mut viewer = Viewer::new(work.path().into());
    let cursor_row = |viewer: &mut Viewer| {
        let mut terminal = Terminal::new(TestBackend::new(60, 10)).unwrap();
        terminal.draw(|frame| viewer.render(frame)).unwrap();
        let buffer = terminal.backend().buffer();
        // A three-digit depth moves the cursor from cell 4 to cell 5.
        buffer
            .content
            .chunks(60)
            .find(|row| row[5].symbol() == ">")
            .map(|row| row[1..10].iter().map(|c| c.symbol()).collect::<String>())
            .unwrap_or_else(|| panic!("{}", screen(viewer, 60, 10)))
    };
    assert_eq!(cursor_row(&mut viewer), "  0 > ◦  ");
    viewer.act(Action::End);
    assert_eq!(cursor_row(&mut viewer), "100 > ◦ »");
    assert!(screen(&mut viewer, 60, 10).contains("deep-k100"));
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
    assert!(visit_file(&mut viewer, 150, 20).contains("VALID ROOT"));
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
    assert!(visit_file(&mut viewer, 180, 24).contains("ROOT BODY"));
    put(&root, "01-impl--task-k1.md", "RECOVERED BODY");
    viewer.act(Action::Refresh);
    viewer.act(Action::Down);
    assert!(visit_file(&mut viewer, 180, 24).contains("RECOVERED BODY"));
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
    viewer.act(Action::Focus);
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
    viewer.act(Action::Focus);
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
    assert!(visit_file(&mut viewer, 140, 20).contains("OLD CHARTER"));
    let release = hold_writer(work.path());
    let start = Instant::now();
    viewer.act(Action::Refresh);
    assert!(start.elapsed() < Duration::from_secs(1));
    assert!(visit_file(&mut viewer, 140, 20).contains("OLD CHARTER"));
    put(&root, "_BRIEF.md", "NEW CHARTER");
    drop(release);
    viewer.tick(Instant::now() + Duration::from_secs(1));
    assert!(visit_file(&mut viewer, 140, 20).contains("NEW CHARTER"));
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
    assert!(visit_file(&mut viewer, 140, 20).contains("DELETED CHARTER"));
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
    assert!(screen(&mut viewer, 160, 12).contains("Tree [active]"));
    key(&mut viewer, Right);
    assert!(visit_file(&mut viewer, 160, 12).contains("BRANCH BODY"));
    key(&mut viewer, Char('h'));
    assert!(!screen(&mut viewer, 160, 12)
        .lines()
        .skip(4)
        .any(|line| line.contains("child-k2")));
    key(&mut viewer, Char('j'));
    assert!(visit_file(&mut viewer, 160, 12).contains("BODY 02"));
    key(&mut viewer, Char('k'));
    key(&mut viewer, Char('l'));
    assert!(visit_file(&mut viewer, 160, 12).contains("BRANCH BODY"));
    key(&mut viewer, Right);
    assert!(visit_file(&mut viewer, 160, 12).contains("CHILD BODY"));
    key(&mut viewer, Left);
    assert!(visit_file(&mut viewer, 160, 12).contains("BRANCH BODY"));
    key(&mut viewer, Char(' '));
    assert!(!screen(&mut viewer, 160, 12)
        .lines()
        .skip(4)
        .any(|line| line.contains("child-k2")));
    key(&mut viewer, Enter);
    assert!(screen(&mut viewer, 160, 12).contains("child-k2"));
    key(&mut viewer, End);
    let last = screen(&mut viewer, 160, 12);
    assert!(
        last.lines()
            .any(|line| line.contains('>') && line.contains("task-k30")),
        "{last}"
    );
    assert!(visit_file(&mut viewer, 160, 12).contains("BODY 29"));
    key(&mut viewer, Home);
    assert!(visit_file(&mut viewer, 160, 12).contains("ROOT BODY"));
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
    screen(&mut viewer, 100, 15); // eight file rows
    key(&mut viewer, Tab);
    screen(&mut viewer, 100, 15);
    key(&mut viewer, Down);
    let line = screen(&mut viewer, 100, 15);
    assert!(line.contains("File [active]"));
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
    assert!(taller.contains("line 32"));
    assert!(taller.contains("line 49"));
    key(&mut viewer, Home);
    assert!(screen(&mut viewer, 100, 15).contains("line 00"));
    assert_eq!(snapshot(work.path()), before);

    put(
        &root,
        "_BRIEF.md",
        &format!("```\n界e\u{301}{}TAIL\n```", "界".repeat(80)),
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
    screen(&mut viewer, 100, 15);
    key(&mut viewer, PageDown);
    let saved = screen(&mut viewer, 100, 15);
    key(&mut viewer, Char('?'));
    let help = screen(&mut viewer, 60, 10);
    for expected in [
        "Key help",
        "File active",
        "Tab: Tree",
        "full-width",
        "Ctrl-u",
        "Esc",
    ] {
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
        viewer.act(Action::Focus);
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
        viewer.act(Action::Focus);
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
        if view.lines().nth(5).unwrap().contains("UNIQUE-MARKER") {
            break;
        }
        viewer.act(Action::Down);
    }
    assert!(screen(&mut viewer, 160, 15)
        .lines()
        .nth(5)
        .unwrap()
        .contains("UNIQUE-MARKER"));
    assert!(screen(&mut viewer, 60, 15).contains("UNIQUE-MARKER"));
    viewer.act(Action::Focus);
    viewer.act(Action::Down);
    assert!(visit_file(&mut viewer, 60, 15).contains("other document"));
    viewer.act(Action::Up);
    assert!(visit_file(&mut viewer, 120, 15).contains("UNIQUE-MARKER"));
    assert_eq!(snapshot(work.path()), before);
}

fn scroll_to(viewer: &mut Viewer, marker: &str) {
    for _ in 0..300 {
        if screen(viewer, 160, 15)
            .lines()
            .nth(5)
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
            assert!(visit_file(&mut viewer, 160, 15).contains("other document"));
        }
        put(
            &root,
            "_BRIEF.md",
            &format!("{}{}", "inserted\n\n".repeat(20), text),
        );
        let expected = snapshot(work.path());
        if revisit {
            viewer.act(Action::Up);
            viewer.act(Action::Focus);
        } else {
            viewer.act(Action::Refresh);
        }
        let view = screen(&mut viewer, 160, 15);
        assert!(view.lines().nth(5).unwrap().contains("MARKER"), "{view}");
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
        .nth(5)
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
        assert!(view.lines().nth(5).unwrap().contains("DUPLICATE"), "{view}");
        assert!(
            view.lines().nth(6).unwrap().contains("second continuation"),
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
        view.lines().nth(5).unwrap().contains("NEXT SURVIVOR"),
        "{view}"
    );
    assert_eq!(snapshot(work.path()), expected);
    for replacement in ["", "短い新規文書", "short replacement"] {
        put(&root, "_BRIEF.md", replacement);
        let expected = snapshot(work.path());
        viewer.act(Action::Refresh);
        let view = screen(&mut viewer, 160, 15);
        let shown = view.lines().nth(5).unwrap().replace(' ', "");
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
    assert!(before.lines().nth(12).unwrap().contains("END MARKER"));
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
        view.lines().nth(5).unwrap().contains("C SURVIVOR"),
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
    viewer.act(Action::Focus);
    let view = screen(&mut viewer, 160, 15);
    assert!(view.lines().nth(5).unwrap().contains("MARKER"), "{view}");
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
            "tail/".repeat(300)
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
            if view.lines().nth(5).unwrap().contains("UNIQUE-MARKER") {
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
            edited.lines().nth(5).unwrap().contains("UNIQUE-MARKER"),
            "{edited}"
        );
        viewer.act(Action::Focus);
        viewer.act(Action::Down);
        screen(&mut viewer, 100, 15);
        viewer.act(Action::Up);
        viewer.act(Action::Focus);
        let revisited = screen(&mut viewer, 100, 15);
        assert!(
            revisited.lines().nth(5).unwrap().contains("UNIQUE-MARKER"),
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
    assert!(visit_file(&mut viewer, 160, 15).contains("other starts at top"));
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
    viewer.act(Action::Focus);
    let tree = screen(&mut viewer, 180, 24);
    viewer.act(Action::Focus);
    let row = tree
        .lines()
        .find(|row| row.contains("renamed-k3 impl"))
        .unwrap();
    assert!(row.starts_with("│ 2 > ✓"), "{tree}");
    assert!(shown.contains("CHOSEN BODY"), "{shown}");
    assert!(!shown.contains("HIDDEN BODY"));
    assert!(
        tree.contains("hidden-k2"),
        "moved selection must reveal ancestor: {shown}"
    );
    assert!(shown.contains("File [active]"));
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
    assert!(visit_file(&mut viewer, 160, 20).contains("OLD ITEM"));
    fs::rename(&root, work.path().join("old-tree")).unwrap();
    put(&root, "_BRIEF.md", "NEW ROOT");
    put(&root, "01-impl--chosen-k1.md", "NEW ITEM");
    let expected = snapshot(work.path());
    viewer.act(Action::Refresh);
    let shown = visit_file(&mut viewer, 160, 20);
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
    assert!(
        visit_file(&mut viewer, 160, 20).contains("CHOSEN BODY"),
        "{shown}"
    );
    let row = shown
        .lines()
        .find(|row| row.contains("chosen-k1 ◦0 ✓0 ✗0"))
        .unwrap();
    assert!(row.starts_with("│ 1 > ∅"), "{shown}");
    assert!(row.contains("- chosen-k1"), "{shown}");
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
    screen(&mut viewer, 160, 20);
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
    assert!(visit_file(&mut viewer, 180, 20).contains("REPLACEMENT ROOT"));
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
    screen(&mut viewer, 180, 24);
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

/// Drive exactly the production clock/deadline seam; never advance synthetic
/// time or deliver Refresh. A broken detector fails at a bounded screen check.
fn await_live(
    viewer: &mut Viewer,
    started: std::time::Instant,
    expected: impl Fn(&str) -> bool,
) -> String {
    use std::time::{Duration, Instant};
    let deadline = started + Duration::from_secs(1);
    loop {
        viewer.tick(Instant::now());
        let frame = screen(viewer, 140, 24);
        let matches = expected(&frame);
        assert!(Instant::now() < deadline, "live update timed out:\n{frame}");
        if matches {
            return frame;
        }
        std::thread::sleep(
            viewer
                .retry_after(Instant::now())
                .unwrap()
                .min(Duration::from_millis(10)),
        );
    }
}

#[test]
fn production_clock_observes_nested_additions_and_deletions_without_writes() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT");
    let mut viewer = Viewer::new(work.path().into());
    let started = std::time::Instant::now();
    put(&root, "01-k1/_branch.md", "BRANCH");
    put(&root, "01-k1/01-impl--added-k2.md", "ADDED");
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| s.contains("added-k2"));
    assert_eq!(snapshot(work.path()), expected);
    viewer.act(Action::Down);
    viewer.act(Action::Down);
    let started = std::time::Instant::now();
    fs::remove_file(root.join("01-k1/01-impl--added-k2.md")).unwrap();
    let expected = snapshot(work.path());
    let frame = await_live(&mut viewer, started, |s| s.contains("disappeared"));
    assert!(
        visit_file(&mut viewer, 140, 24).contains("BRANCH"),
        "{frame}"
    );
    viewer.act(Action::Focus);
    assert_eq!(snapshot(work.path()), expected);
    let started = std::time::Instant::now();
    fs::remove_dir_all(root.join("01-k1")).unwrap();
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| {
        s.contains("ROOT") && !s.contains("│BRANCH")
    });
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn production_clock_reads_same_length_bytes_with_restored_mtime() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT");
    put(&root, "01-impl--selected-k1.md", "AAAAAAAAAAAA");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    viewer.act(Action::Focus);
    let file = root.join("01-impl--selected-k1.md");
    let modified = fs::metadata(&file).unwrap().modified().unwrap();
    let started = std::time::Instant::now();
    fs::write(&file, "ZZZZZZZZZZZZ").unwrap();
    fs::File::options()
        .write(true)
        .open(&file)
        .unwrap()
        .set_times(fs::FileTimes::new().set_modified(modified))
        .unwrap();
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| s.contains("ZZZZZZZZZZZZ"));
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn production_clock_coalesces_rapid_bursts_to_current_content() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "INITIAL");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Focus);
    let started = std::time::Instant::now();
    for n in 0..30 {
        put(&root, "_BRIEF.md", &format!("BURST_{n:02}"));
        let expected = snapshot(work.path());
        viewer.tick(std::time::Instant::now());
        screen(&mut viewer, 140, 24);
        assert_eq!(snapshot(work.path()), expected);
    }
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| s.contains("BURST_29"));
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn production_clock_recovers_from_interrupted_decomposition_and_duplicate_keys() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT");
    put(&root, "01-impl--selected-k1.md", "SELECTED");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    let started = std::time::Instant::now();
    fs::create_dir(root.join("01-k1")).unwrap();
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| s.contains("STALE"));
    assert!(visit_file(&mut viewer, 140, 24).contains("SELECTED"));
    assert_eq!(snapshot(work.path()), expected);
    let started = std::time::Instant::now();
    fs::rename(
        root.join("01-impl--selected-k1.md"),
        root.join("01-k1/_selected.md"),
    )
    .unwrap();
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| {
        s.contains("selected-k1 ◦0 ✓0 ✗0") && !s.contains("STALE")
    });
    assert_eq!(snapshot(work.path()), expected);
    let started = std::time::Instant::now();
    put(&root, "02-impl--duplicate-k1.md", "WRONG");
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| s.contains("duplicate key"));
    let file = visit_file(&mut viewer, 140, 24);
    assert!(file.contains("SELECTED") && !file.contains("WRONG"));
    assert_eq!(snapshot(work.path()), expected);
    let started = std::time::Instant::now();
    fs::remove_file(root.join("02-impl--duplicate-k1.md")).unwrap();
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| {
        s.contains("selected-k1") && !s.contains("STALE")
    });
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn production_clock_replacement_between_polls_resets_reused_keys() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "OLD_ROOT");
    put(&root, "01-impl--selected-k1.md", "OLD_SELECTION");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    let started = std::time::Instant::now();
    fs::rename(&root, work.path().join("old-root")).unwrap();
    put(&root, "_BRIEF.md", "REPLACEMENT_ROOT");
    put(&root, "01-impl--selected-k1.md", "REUSED_SELECTION");
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| {
        s.lines()
            .any(|row| row.starts_with("│ 0 > ") && row.contains("- root"))
    });
    let frame = visit_file(&mut viewer, 140, 24);
    assert!(frame.contains("REPLACEMENT_ROOT"));
    assert!(!frame.contains("OLD_SELECTION") && !frame.contains("REUSED_SELECTION"));
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn production_clock_removal_and_delayed_recreation_clear_the_old_view() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "OLD_ROOT");
    let mut viewer = Viewer::new(work.path().into());
    let started = std::time::Instant::now();
    fs::remove_dir_all(&root).unwrap();
    let expected = snapshot(work.path());
    let frame = await_live(&mut viewer, started, |s| s.contains("WAITING"));
    assert!(!frame.contains("OLD_ROOT"));
    assert_eq!(snapshot(work.path()), expected);
    // Stay absent across another real observation deadline.
    std::thread::sleep(viewer.retry_after(std::time::Instant::now()).unwrap());
    viewer.tick(std::time::Instant::now());
    assert_eq!(snapshot(work.path()), expected);
    let started = std::time::Instant::now();
    put(&root, "_BRIEF.md", "RECREATED_ROOT");
    let expected = snapshot(work.path());
    await_live(&mut viewer, started, |s| !s.contains("WAITING"));
    assert!(visit_file(&mut viewer, 140, 24).contains("RECREATED_ROOT"));
    assert_eq!(snapshot(work.path()), expected);
}

#[test]
fn production_clock_recovers_from_busy_and_unreadable_selected_content() {
    use std::os::unix::fs::PermissionsExt;
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    put(&root, "_BRIEF.md", "ROOT");
    put(&root, "01-impl--selected-k1.md", "SELECTED_CONTENT");
    let file = root.join("01-impl--selected-k1.md");
    let mut viewer = Viewer::new(work.path().into());
    viewer.act(Action::Down);
    viewer.act(Action::Focus);
    let started = std::time::Instant::now();
    let writer = hold_writer(work.path());
    let expected = snapshot(work.path());
    let frame = await_live(&mut viewer, started, |s| s.contains("WAITING"));
    assert!(frame.contains("SELECTED_CONTENT"));
    assert!(screen(&mut viewer, 40, 8).contains("Resize"));
    let started = std::time::Instant::now();
    drop(writer);
    await_live(&mut viewer, started, |s| {
        s.contains("SELECTED_CONTENT") && !s.contains("WAITING")
    });
    assert_eq!(snapshot(work.path()), expected);

    // Keep an open descriptor to compare bytes while pathname permissions deny
    // the viewer's fresh open; changing permissions is solely the test's work.
    let mut readable = fs::File::open(&file).unwrap();
    let started = std::time::Instant::now();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o0)).unwrap();
    if fs::read(&file).is_err() {
        let frame = await_live(&mut viewer, started, |s| s.contains("File error"));
        assert!(frame.contains("selected-k1") && !frame.contains("│ROOT"));
        assert_eq!(fs::metadata(&file).unwrap().permissions().mode() & 0o777, 0);
        use std::io::Read;
        let mut bytes = Vec::new();
        readable.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, expected[".grove/01-impl--selected-k1.md"]);
    }
    let started = std::time::Instant::now();
    fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).unwrap();
    await_live(&mut viewer, started, |s| {
        s.contains("SELECTED_CONTENT") && !s.contains("File error")
    });
    assert_eq!(snapshot(work.path()), expected);

    let started = std::time::Instant::now();
    fs::set_permissions(&root, fs::Permissions::from_mode(0o0)).unwrap();
    if fs::read_dir(&root).is_err() {
        let frame = await_live(&mut viewer, started, |s| s.contains("STALE"));
        assert!(frame.contains("SELECTED_CONTENT"));
    }
    let started = std::time::Instant::now();
    fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).unwrap();
    await_live(&mut viewer, started, |s| {
        s.contains("SELECTED_CONTENT") && !s.contains("STALE")
    });
    assert_eq!(snapshot(work.path()), expected);
}

fn press(
    viewer: &mut Viewer,
    code: crossterm::event::KeyCode,
    modifiers: crossterm::event::KeyModifiers,
) -> bool {
    Action::from_key(crossterm::event::KeyEvent::new(code, modifiers))
        .is_some_and(|action| viewer.act(action))
}

/// Type plain keys, one character each, as a terminal reports them. An
/// uppercase letter carries Shift. Like the terminal driver, draw a frame
/// (100 × 15) before each key, so each key sees the view that it acts on.
fn type_keys(viewer: &mut Viewer, keys: &str) {
    use crossterm::event::{KeyCode::Char, KeyModifiers};
    for key in keys.chars() {
        let modifiers = if key.is_uppercase() {
            KeyModifiers::SHIFT
        } else {
            KeyModifiers::NONE
        };
        screen(viewer, 100, 15);
        press(viewer, Char(key), modifiers);
    }
}

/// The cursor row's place in the tree pane, counted from the pane's first
/// content row, and its handle.
fn cursor(screen: &str) -> (usize, String) {
    // Four chrome lines and the top border precede the first content row.
    // The cursor follows the border and the two-cell depth column.
    let (row, line) = screen
        .lines()
        .skip(5)
        .enumerate()
        .find(|(_, line)| line.chars().nth(4) == Some('>'))
        .unwrap_or_else(|| panic!("no cursor:\n{screen}"));
    let handle = line
        .split_whitespace()
        .find(|word| word.starts_with("task-k") || *word == "root")
        .unwrap_or_else(|| panic!("no handle: {line}"));
    (row, handle.to_owned())
}

/// The root and 29 leaves, `task-k1` to `task-k29`, so row n shows `task-kn`.
fn thirty_rows(root: &Path) {
    put(root, "_BRIEF.md", "ROOT BODY");
    for n in 1..30 {
        put(
            root,
            &format!("{n:02}-impl--task-k{n}.md"),
            &format!("BODY {n:02}"),
        );
    }
}

fn at(row: usize, handle: &str) -> (usize, String) {
    (row, handle.to_owned())
}

#[test]
fn vim_keys_decode_to_their_actions() {
    use crossterm::event::{KeyCode::*, KeyEvent, KeyEventKind, KeyModifiers};
    let control = KeyModifiers::CONTROL;
    let none = KeyModifiers::NONE;
    for (code, modifiers, action) in [
        (Char('f'), control, Action::PageDown),
        (Char('f'), none, Action::PageDown),
        (PageDown, none, Action::PageDown),
        (Char('b'), control, Action::PageUp),
        (Char('b'), none, Action::B),
        (PageUp, none, Action::PageUp),
        (Char('d'), control, Action::HalfPageDown),
        (Char('d'), none, Action::HalfPageDown),
        (Char('u'), control, Action::HalfPageUp),
        (Char('u'), none, Action::HalfPageUp),
        (Char('g'), none, Action::G),
        (Char('G'), KeyModifiers::SHIFT, Action::End),
        (Char('z'), none, Action::Z),
        (Char('t'), none, Action::T),
        (Char('z'), control, Action::Unbound),
        (Char('x'), none, Action::Unbound),
        (F(5), none, Action::Unbound),
        (Char('c'), control, Action::Quit),
    ] {
        assert_eq!(
            Action::from_key(KeyEvent::new(code, modifiers)),
            Some(action),
            "{code:?} {modifiers:?}"
        );
    }
    let mut release = KeyEvent::new(Char('g'), none);
    release.kind = KeyEventKind::Release;
    assert_eq!(Action::from_key(release), None);
}

#[test]
fn tree_jumps_and_pages_move_the_cursor_and_the_view_together() {
    use crossterm::event::{KeyCode::*, KeyModifiers};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    thirty_rows(&root);
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    // 100 × 15 leaves eight tree rows, so half a page is four rows.
    let view = |viewer: &mut Viewer| cursor(&screen(viewer, 100, 15));
    assert_eq!(view(&mut viewer), at(0, "root"));
    type_keys(&mut viewer, "G");
    assert_eq!(view(&mut viewer), at(7, "task-k29"));
    assert!(visit_file(&mut viewer, 100, 15).contains("BODY 29"));
    type_keys(&mut viewer, "gg");
    assert_eq!(view(&mut viewer), at(0, "root"));
    assert!(visit_file(&mut viewer, 100, 15).contains("ROOT BODY"));
    type_keys(&mut viewer, "jj");
    assert_eq!(view(&mut viewer), at(2, "task-k2"));
    // Each page key keeps the cursor on the third row of the pane.
    let control = KeyModifiers::CONTROL;
    let none = KeyModifiers::NONE;
    for (code, modifiers, handle) in [
        (Char('f'), none, "task-k10"),
        (Char('b'), none, "task-k2"),
        (Char('f'), control, "task-k10"),
        (Char('b'), control, "task-k2"),
        (PageDown, none, "task-k10"),
        (PageUp, none, "task-k2"),
        (Char('d'), none, "task-k6"),
        (Char('u'), none, "task-k2"),
        (Char('d'), control, "task-k6"),
        (Char('u'), control, "task-k2"),
    ] {
        press(&mut viewer, code, modifiers);
        assert_eq!(view(&mut viewer), at(2, handle), "{code:?} {modifiers:?}");
    }
    assert!(visit_file(&mut viewer, 100, 15).contains("BODY 02"));
    // At the ends of the tree the view stops and the cursor moves on alone.
    type_keys(&mut viewer, "fff");
    assert_eq!(view(&mut viewer), at(4, "task-k26"));
    type_keys(&mut viewer, "f");
    assert_eq!(view(&mut viewer), at(7, "task-k29"));
    type_keys(&mut viewer, "bbb");
    assert_eq!(view(&mut viewer), at(5, "task-k5"));
    type_keys(&mut viewer, "b");
    assert_eq!(view(&mut viewer), at(0, "root"));
    type_keys(&mut viewer, "u");
    assert_eq!(view(&mut viewer), at(0, "root"));
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn z_placements_scroll_the_tree_and_leave_the_cursor_until_it_leaves_the_pane() {
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    thirty_rows(&root);
    let mut viewer = Viewer::new(work.path().into());
    let view = |viewer: &mut Viewer| cursor(&screen(viewer, 100, 15));
    view(&mut viewer);
    type_keys(&mut viewer, &"j".repeat(15));
    assert_eq!(view(&mut viewer), at(7, "task-k15"));
    type_keys(&mut viewer, "zt");
    assert_eq!(view(&mut viewer), at(0, "task-k15"));
    type_keys(&mut viewer, "zb");
    assert_eq!(view(&mut viewer), at(7, "task-k15"));
    // Eight rows have two middle rows. The cursor takes the upper one.
    type_keys(&mut viewer, "zz");
    assert_eq!(view(&mut viewer), at(3, "task-k15"));
    assert!(visit_file(&mut viewer, 100, 15).contains("BODY 15"));
    // The placement holds through refreshes, ticks and moves inside the pane.
    type_keys(&mut viewer, "r");
    viewer.tick(std::time::Instant::now() + std::time::Duration::from_secs(1));
    assert_eq!(view(&mut viewer), at(3, "task-k15"));
    type_keys(&mut viewer, "jjjj");
    assert_eq!(view(&mut viewer), at(7, "task-k19"));
    type_keys(&mut viewer, &"k".repeat(7));
    assert_eq!(view(&mut viewer), at(0, "task-k12"));
    // Once the cursor leaves the pane, the view moves only as far as it must.
    type_keys(&mut viewer, "k");
    assert_eq!(view(&mut viewer), at(0, "task-k11"));
    // Near either end of the tree the placement clamps, with no blank rows.
    type_keys(&mut viewer, "ggjzzzb");
    assert_eq!(view(&mut viewer), at(1, "task-k1"));
    type_keys(&mut viewer, "Gzt");
    assert_eq!(view(&mut viewer), at(7, "task-k29"));
    type_keys(&mut viewer, "kzz");
    assert_eq!(view(&mut viewer), at(6, "task-k28"));
    let tree = screen(&mut viewer, 100, 15);
    assert!(tree.lines().nth(12).unwrap().contains("task-k29"), "{tree}");
}

#[test]
fn a_prefix_waits_for_its_second_key_and_any_other_key_cancels_it() {
    use crossterm::event::{KeyCode::*, KeyModifiers};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    thirty_rows(&root);
    let mut viewer = Viewer::new(work.path().into());
    let view = |viewer: &mut Viewer| cursor(&screen(viewer, 100, 15));
    view(&mut viewer);
    type_keys(&mut viewer, "jjjjj");
    // A prefix alone moves nothing.
    type_keys(&mut viewer, "g");
    assert_eq!(view(&mut viewer), at(5, "task-k5"));
    // A key that ends no sequence cancels the prefix, then acts as usual.
    type_keys(&mut viewer, "j");
    assert_eq!(view(&mut viewer), at(6, "task-k6"));
    type_keys(&mut viewer, "g");
    press(&mut viewer, F(5), KeyModifiers::NONE);
    type_keys(&mut viewer, "g");
    assert_eq!(view(&mut viewer), at(6, "task-k6"));
    type_keys(&mut viewer, "zj");
    assert_eq!(view(&mut viewer), at(7, "task-k7"));
    type_keys(&mut viewer, "zgg");
    assert_eq!(view(&mut viewer), at(0, "root"));
    type_keys(&mut viewer, &format!("G{}", "k".repeat(10)));
    assert_eq!(view(&mut viewer), at(0, "task-k19"));
    type_keys(&mut viewer, "gzz");
    assert_eq!(view(&mut viewer), at(3, "task-k19"));
    // `t` alone does nothing. `b` alone pages up.
    type_keys(&mut viewer, "t");
    assert_eq!(view(&mut viewer), at(3, "task-k19"));
    type_keys(&mut viewer, "b");
    assert_eq!(view(&mut viewer), at(3, "task-k11"));
    // Tab, help, Escape and refresh also cancel a prefix.
    for interrupt in [&[Tab, Tab][..], &[Char('?'), Esc], &[Esc], &[Char('r')]] {
        type_keys(&mut viewer, "g");
        for &code in interrupt {
            press(&mut viewer, code, KeyModifiers::NONE);
        }
        type_keys(&mut viewer, "g");
        assert_eq!(view(&mut viewer), at(3, "task-k11"), "{interrupt:?}");
        type_keys(&mut viewer, "k");
        type_keys(&mut viewer, "j");
    }
    // A prefix typed while help is open is not held after help closes.
    type_keys(&mut viewer, "?g");
    press(&mut viewer, Esc, KeyModifiers::NONE);
    type_keys(&mut viewer, "g");
    assert_eq!(view(&mut viewer), at(3, "task-k11"));
}

#[test]
fn file_view_takes_the_scroll_keys_without_the_tree_placements() {
    use crossterm::event::{KeyCode::*, KeyModifiers};
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    let content = (0..50)
        .map(|n| format!("line {n:02}\n"))
        .collect::<String>();
    put(&root, "_BRIEF.md", &format!("```\n{content}```"));
    let before = snapshot(work.path());
    let mut viewer = Viewer::new(work.path().into());
    // The first file row of a 100 × 15 screen, which shows eight file rows.
    let top = |viewer: &mut Viewer| {
        let file = screen(viewer, 100, 15);
        file.lines().nth(5).unwrap().to_owned()
    };
    key(&mut viewer, Tab);
    assert!(top(&mut viewer).contains("line 00"));
    let control = KeyModifiers::CONTROL;
    let none = KeyModifiers::NONE;
    for (code, modifiers, expected) in [
        (Char('f'), none, "line 08"),
        (Char('b'), none, "line 00"),
        (Char('f'), control, "line 08"),
        (Char('b'), control, "line 00"),
        (PageDown, none, "line 08"),
        (PageUp, none, "line 00"),
        (Char('d'), none, "line 04"),
        (Char('u'), none, "line 00"),
        (Char('d'), control, "line 04"),
        (Char('u'), control, "line 00"),
    ] {
        press(&mut viewer, code, modifiers);
        assert!(
            top(&mut viewer).contains(expected),
            "{code:?} {modifiers:?}: {}",
            top(&mut viewer)
        );
    }
    type_keys(&mut viewer, "G");
    assert!(screen(&mut viewer, 100, 15).contains("line 49"));
    type_keys(&mut viewer, "gg");
    assert!(top(&mut viewer).contains("line 00"));
    // File view has no cursor. `zz`, `zt` and `zb` move nothing, and `zb`
    // is not a page up.
    type_keys(&mut viewer, "ff");
    for keys in ["zz", "zt", "zb"] {
        type_keys(&mut viewer, keys);
        assert!(top(&mut viewer).contains("line 16"), "{keys}");
    }
    type_keys(&mut viewer, "b");
    assert!(top(&mut viewer).contains("line 08"));
    // The placements typed in File view do not wait for Tree view.
    key(&mut viewer, Tab);
    let tree = screen(&mut viewer, 100, 15);
    assert_eq!(cursor(&tree), at(0, "root"));
    assert_eq!(snapshot(work.path()), before);
}

#[test]
fn help_lists_every_key_and_the_glyph_legend_at_the_minimum_size() {
    use crossterm::event::KeyCode::*;
    let work = tempfile::tempdir().unwrap();
    let root = work.path().join(".grove");
    thirty_rows(&root);
    let mut viewer = Viewer::new(work.path().into());
    let lines = [
        "Tab: switch full-width view | r refresh | q/Ctrl-c quit",
        "?: toggle help | Esc: close help | Enter/Space: fold",
        "j/k Up/Down: row or line | gg/G Home/End: first or last",
        "l/Right expand/child | h/Left fold/parent | File: scroll",
        "f/b Ctrl-f/Ctrl-b PageDown/PageUp: page down/up",
        "d/u Ctrl-d/Ctrl-u: half page down/up",
        "zz/zt/zb: tree cursor row to middle/top/bottom",
        "◦ LIVE ✓ DONE ✗ ABANDONED ∅ EMPTY ► RUNNING » NEXT",
    ];
    for title in ["Tree active | Tab: File", "File active | Tab: Tree"] {
        key(&mut viewer, Char('?'));
        let help = screen(&mut viewer, 60, 10);
        assert!(help.contains(title), "{help}");
        // Each line shows in full inside the border, one per content row.
        for (row, line) in lines.iter().enumerate() {
            assert!(
                help.lines().nth(row + 1).unwrap().contains(line),
                "{line}\n{help}"
            );
        }
        key(&mut viewer, Esc);
        key(&mut viewer, Tab);
    }
}

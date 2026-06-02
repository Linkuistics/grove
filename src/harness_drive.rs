//! Controller-side **harness driving**: open / focus / close a grove's
//! [[workspace]] — a native zellij **tab** running `grove do <name>` — tracking
//! each grove's stable tab id (leaf
//! `060-harness-pane/040-grove-integration/060-workspace-tabs`).
//!
//! Per ADR-0018 each grove is a zellij *tab* (not a sibling pane): the home
//! dashboard is the "home" tab, opening a grove creates its tab, and switching
//! is native `GoToTab` keybinds — which work in `default_mode "locked"`, unlike
//! the superseded `focus-pane-id`-between-panes model (which had no bound key
//! back to the dashboard). Per ADR-0015/0016 the **controller** owns all
//! tab-decision state and drives zellij with grove's shell-out idiom — the
//! `zellij action` tab verbs (zellij 0.44.3):
//!
//! - `zellij --session <s> action new-tab --cwd <repo> --name <name> -- <grove>
//!   do <name>` creates the grove's tab, runs the harness in it, and **prints
//!   the created tab's stable id** (a bare number) on stdout — so id discovery
//!   is native, exactly as the 040 pane driver discovered pane ids.
//! - `zellij --session <s> action go-to-tab-by-id <id>` switches to it.
//! - `zellij --session <s> action close-tab-by-id <id>` closes it.
//!
//! This repurposes the 040 pane driver onto tabs: the state machine (track
//! `{grove → id}`, open-or-focus, close-and-forget, reopen-on-stale) is
//! unchanged; only the three `zellij action` verbs swap pane→tab (ADR-0018:
//! "superseded, not wasted").
//!
//! The controller runs *outside* zellij (it spawned zellij as a child, then
//! renders into a proxy pane inside it), so every action targets the session
//! **explicitly** via `--session`. That same explicitness is what keeps the
//! driving layer repo-agnostic: [`HarnessTabs::open_or_focus`] takes the repo
//! as an argument and opens with `--cwd <repo>`, so 070's cross-repo fleet
//! reuses it unchanged.
//!
//! The tab map ([`HarnessTabs`]) is the controller's single source of truth
//! (ADR-0016): no tab-decision state lives in any proxy.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

/// The three `zellij action` shell-outs, behind a trait so the
/// [`HarnessTabs`] state machine is unit-testable without a running zellij.
/// The production implementation is [`ZellijCli`]; tests substitute a fake.
pub trait ZellijDriver {
    /// Create the grove's tab in `session` running `grove do <name>` in `repo`,
    /// returning the created tab's stable id (a bare number), parsed from
    /// zellij's stdout.
    fn new_tab(&self, session: &str, name: &str, repo: &Path) -> Result<String>;
    /// Switch to the tab with stable id `id` in `session`. Errors if the tab is
    /// gone — [`HarnessTabs::open_or_focus`] treats that as "reopen".
    fn go_to_tab(&self, session: &str, id: &str) -> Result<()>;
    /// Close the tab with stable id `id` in `session`.
    fn close_tab(&self, session: &str, id: &str) -> Result<()>;
}

/// What a drive request did, for the dashboard status line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// A new harness tab was opened (and zellij focused it).
    Opened,
    /// The grove's existing harness tab was focused.
    Focused,
    /// The grove's harness tab was closed and forgotten.
    Closed,
    /// Close was a no-op: no harness tab was tracked for the grove.
    NotOpen,
}

impl Action {
    /// A short status-line message naming the grove acted on.
    pub fn status_line(self, name: &str) -> String {
        match self {
            Action::Opened => format!("opened harness: {name}"),
            Action::Focused => format!("switched to harness: {name}"),
            Action::Closed => format!("closed harness: {name}"),
            Action::NotOpen => format!("no open harness: {name}"),
        }
    }
}

/// The controller's record of which groves have a live harness tab, keyed by
/// grove name → stable zellij tab id. The single source of truth for
/// tab decisions (ADR-0016, ADR-0018).
pub struct HarnessTabs {
    /// The zellij session the dashboard runs in; every action targets it.
    session: String,
    /// grove name → stable tab id (a bare number).
    tabs: BTreeMap<String, String>,
    /// The `zellij action` driver (real CLI in production, fake in tests).
    driver: Box<dyn ZellijDriver>,
}

impl HarnessTabs {
    /// Build a tracker that drives the real `zellij` CLI for `session`.
    pub fn new(session: String) -> Self {
        Self::with_driver(session, Box::new(ZellijCli))
    }

    /// Build a tracker with an injected driver (used by tests).
    pub fn with_driver(session: String, driver: Box<dyn ZellijDriver>) -> Self {
        Self {
            session,
            tabs: BTreeMap::new(),
            driver,
        }
    }

    /// Open the harness for `name` (running `grove do <name>` in `repo`) as a
    /// native zellij tab, or switch to its existing tab if one is already open —
    /// so a second invocation on the same grove switches to it rather than
    /// opening a duplicate. If switching fails (the user closed the tab in
    /// zellij), the stale id is forgotten and a fresh tab is opened.
    ///
    /// The "already on this tab" benign-error case the 040 pane driver had to
    /// special-case cannot arise here: the open key is pressed *from the home
    /// dashboard*, so the active tab is never the grove's own tab — any
    /// `go-to-tab-by-id` failure is therefore a genuinely-gone tab, treated as
    /// "reopen".
    ///
    /// `repo` is explicit (opened with `--cwd <repo>`) so the cross-repo fleet
    /// (070) reuses this driving layer unchanged.
    pub fn open_or_focus(&mut self, name: &str, repo: &Path) -> Result<Action> {
        if let Some(id) = self.tabs.get(name).cloned() {
            match self.driver.go_to_tab(&self.session, &id) {
                Ok(()) => return Ok(Action::Focused),
                // The tracked tab is gone — drop it and fall through to reopen.
                Err(_) => {
                    self.tabs.remove(name);
                }
            }
        }
        let id = self
            .driver
            .new_tab(&self.session, name, repo)
            .with_context(|| format!("opening harness tab for '{name}'"))?;
        self.tabs.insert(name.to_string(), id);
        Ok(Action::Opened)
    }

    /// Close the harness tab for `name` and forget its id. A no-op
    /// ([`Action::NotOpen`]) when no tab is tracked for the grove.
    pub fn close(&mut self, name: &str) -> Result<Action> {
        match self.tabs.remove(name) {
            Some(id) => {
                self.driver
                    .close_tab(&self.session, &id)
                    .with_context(|| format!("closing harness tab for '{name}'"))?;
                Ok(Action::Closed)
            }
            None => Ok(Action::NotOpen),
        }
    }

    /// Whether a harness tab is currently tracked for `name`.
    #[cfg(test)]
    pub fn is_open(&self, name: &str) -> bool {
        self.tabs.contains_key(name)
    }
}

// ---------------------------------------------------------------------------
// The real driver: shell out to `zellij action`.

/// Production [`ZellijDriver`] that shells out to the installed `zellij`,
/// exactly as the 020 spike verified (zellij 0.44.3).
pub struct ZellijCli;

impl ZellijDriver for ZellijCli {
    fn new_tab(&self, session: &str, name: &str, repo: &Path) -> Result<String> {
        // Run the same `grove` binary as the controller so dev builds
        // (target/debug/grove) and installed builds both open a matching
        // harness, mirroring how the head binary resolves the proxy command.
        let grove_bin = std::env::current_exe().context("locating the running grove binary")?;
        let out = Command::new("zellij")
            .args(new_tab_args(session, &grove_bin.display().to_string(), name, repo))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("running zellij action new-tab")?;
        if !out.status.success() {
            anyhow::bail!(
                "zellij action new-tab failed (status {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        parse_tab_id(&String::from_utf8_lossy(&out.stdout))
    }

    fn go_to_tab(&self, session: &str, id: &str) -> Result<()> {
        // `go-to-tab-by-id` exits 0 even when the tab is gone (verified live on
        // 0.44.3), so — unlike the 040 `focus-pane-id` driver — it cannot signal
        // "reopen" by erroring. Detect a closed tab ourselves: if the tracked id
        // is no longer in `list-tabs`, error so `open_or_focus` reopens a fresh
        // tab rather than silently no-op'ing on a ghost. (No benign "already on
        // this tab" case to handle — the open key is pressed from the home tab,
        // never the grove's own; see `HarnessTabs::open_or_focus`.)
        if !tab_id_exists(session, id)? {
            anyhow::bail!("zellij tab {id} no longer exists");
        }
        run_action(&go_to_tab_args(session, id), "go-to-tab-by-id")
    }

    fn close_tab(&self, session: &str, id: &str) -> Result<()> {
        run_action(&close_tab_args(session, id), "close-tab-by-id")
    }
}

/// Run a `zellij` invocation that produces no useful stdout, mapping a non-zero
/// exit to an error carrying zellij's stderr.
fn run_action(args: &[String], what: &str) -> Result<()> {
    let out = Command::new("zellij")
        .args(args)
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("running zellij action {what}"))?;
    if !out.status.success() {
        anyhow::bail!(
            "zellij action {what} failed (status {:?}): {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Pure argv builders + stdout parsing (unit-tested for the exact CLI form).

/// Argv for `zellij --session <s> action new-tab --cwd <repo> --name <name> --
/// <grove> do <name>`. The `--session` flag is global (before `action`) so the
/// controller, which lives outside zellij, targets the dashboard's session. The
/// tab is named for the grove; the harness runs as the tab's initial command.
fn new_tab_args(session: &str, grove_bin: &str, name: &str, repo: &Path) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "new-tab".into(),
        "--cwd".into(),
        repo.display().to_string(),
        "--name".into(),
        name.into(),
        "--".into(),
        grove_bin.into(),
        "do".into(),
        name.into(),
    ]
}

/// Whether the tab with stable `id` still exists in `session`, per a live
/// `zellij action list-tabs`. Used to detect a tab closed out-of-band, since
/// `go-to-tab-by-id` exits 0 regardless. A failed query is reported as an error
/// (the caller treats it conservatively, as "exists", to avoid spuriously
/// reopening on a transient zellij hiccup — see [`ZellijCli::go_to_tab`]).
fn tab_id_exists(session: &str, id: &str) -> Result<bool> {
    let out = Command::new("zellij")
        .args(list_tabs_args(session))
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .context("running zellij action list-tabs")?;
    if !out.status.success() {
        anyhow::bail!(
            "zellij action list-tabs failed (status {:?}): {}",
            out.status.code(),
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(tab_id_present(&String::from_utf8_lossy(&out.stdout), id))
}

/// Argv for `zellij --session <s> action list-tabs`.
fn list_tabs_args(session: &str) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "list-tabs".into(),
    ]
}

/// Whether tab id `id` appears in `zellij action list-tabs` output — a
/// `TAB_ID  POSITION  NAME` table whose first whitespace-separated column is the
/// stable id. Matches the id against each row's first token exactly (so `1`
/// does not match a `10` row, and the `TAB_ID` header is never a false match).
fn tab_id_present(list_tabs_stdout: &str, id: &str) -> bool {
    list_tabs_stdout
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        // Only data rows: a tab id is all digits, so the `TAB_ID` header (and
        // any other non-numeric line) is skipped without special-casing it.
        .filter(|first| !first.is_empty() && first.bytes().all(|b| b.is_ascii_digit()))
        .any(|first| first == id)
}

/// Argv for `zellij --session <s> action go-to-tab-by-id <id>` (id positional).
fn go_to_tab_args(session: &str, id: &str) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "go-to-tab-by-id".into(),
        id.into(),
    ]
}

/// Argv for `zellij --session <s> action close-tab-by-id <id>` (id positional).
fn close_tab_args(session: &str, id: &str) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "close-tab-by-id".into(),
        id.into(),
    ]
}

/// Parse the tab id `zellij action new-tab` prints on stdout: a single number
/// (the tab's stable id). Take the last non-empty line and require it be all
/// digits — the form `go-to-tab-by-id` / `close-tab-by-id` later accept.
fn parse_tab_id(stdout: &str) -> Result<String> {
    let id = stdout
        .lines()
        .map(str::trim)
        .rfind(|l| !l.is_empty())
        .unwrap_or("");
    if id.is_empty() {
        anyhow::bail!("zellij new-tab returned no tab id (empty stdout)");
    }
    if !id.bytes().all(|b| b.is_ascii_digit()) {
        anyhow::bail!("unexpected zellij new-tab output (not a tab id): {id:?}");
    }
    Ok(id.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::path::PathBuf;
    use std::rc::Rc;

    // -- pure builders / parser ------------------------------------------

    #[test]
    fn new_tab_argv_targets_session_cwd_name_and_grove_do() {
        let args = new_tab_args("grove-acme", "/opt/grove", "auth", Path::new("/work/acme"));
        assert_eq!(
            args,
            vec![
                "--session", "grove-acme",
                "action", "new-tab",
                "--cwd", "/work/acme",
                "--name", "auth",
                "--", "/opt/grove", "do", "auth",
            ]
        );
    }

    #[test]
    fn go_to_tab_argv_is_positional_stable_id() {
        assert_eq!(
            go_to_tab_args("grove-acme", "3"),
            vec!["--session", "grove-acme", "action", "go-to-tab-by-id", "3"]
        );
    }

    #[test]
    fn close_tab_argv_is_positional_stable_id() {
        assert_eq!(
            close_tab_args("grove-acme", "3"),
            vec!["--session", "grove-acme", "action", "close-tab-by-id", "3"]
        );
    }

    #[test]
    fn parse_tab_id_accepts_a_bare_number() {
        assert_eq!(parse_tab_id("3\n").unwrap(), "3");
        assert_eq!(parse_tab_id("12").unwrap(), "12");
        // Tolerates leading chatter: the id is the last non-empty line.
        assert_eq!(parse_tab_id("starting...\n9\n").unwrap(), "9");
    }

    #[test]
    fn parse_tab_id_rejects_empty_or_non_numeric() {
        assert!(parse_tab_id("").is_err());
        assert!(parse_tab_id("\n  \n").is_err());
        assert!(parse_tab_id("terminal_3").is_err());
        assert!(parse_tab_id("not-an-id").is_err());
    }

    #[test]
    fn tab_id_present_reads_the_list_tabs_table() {
        // `zellij action list-tabs` prints a `TAB_ID POSITION NAME` table; the
        // first whitespace-separated column is the stable id. We must detect a
        // tracked id's *absence* ourselves because `go-to-tab-by-id` exits 0
        // even for a gone tab (verified live on 0.44.3) — so it can't signal
        // "reopen" by erroring on its own.
        let table = "TAB_ID  POSITION  NAME\n0  0  Tab #1\n1  1  auth\n";
        assert!(tab_id_present(table, "0"));
        assert!(tab_id_present(table, "1"));
        // A closed tab's id is gone from the table → drives a reopen.
        assert!(!tab_id_present(table, "2"));
        // The header's first token must not be mistaken for a tab id.
        assert!(!tab_id_present(table, "TAB_ID"));
        // No partial-match: "1" must not match a row whose id is "10".
        let wide = "TAB_ID  POSITION  NAME\n10  0  auth\n";
        assert!(!tab_id_present(wide, "1"));
        assert!(tab_id_present(wide, "10"));
    }

    // -- state machine with a fake driver --------------------------------

    /// Records every call and lets a test script the next tab id and whether
    /// the next `go_to_tab` should fail (the user closed the tab in zellij).
    #[derive(Default)]
    struct FakeDriver {
        calls: RefCell<Vec<String>>,
        next_id: RefCell<u32>,
        go_to_fails: RefCell<bool>,
    }

    impl FakeDriver {
        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    // Implemented on `Rc<FakeDriver>` so a test can hold one handle to inspect
    // the call log while `HarnessTabs` drives through another.
    impl ZellijDriver for Rc<FakeDriver> {
        fn new_tab(&self, session: &str, name: &str, _repo: &Path) -> Result<String> {
            *self.next_id.borrow_mut() += 1;
            let id = format!("{}", self.next_id.borrow());
            self.calls
                .borrow_mut()
                .push(format!("new_tab {session} {name} -> {id}"));
            Ok(id)
        }
        fn go_to_tab(&self, session: &str, id: &str) -> Result<()> {
            self.calls
                .borrow_mut()
                .push(format!("go_to_tab {session} {id}"));
            if *self.go_to_fails.borrow() {
                anyhow::bail!("tab {id} not found");
            }
            Ok(())
        }
        fn close_tab(&self, session: &str, id: &str) -> Result<()> {
            self.calls
                .borrow_mut()
                .push(format!("close_tab {session} {id}"));
            Ok(())
        }
    }

    /// A `HarnessTabs` over a shared fake driver, plus the handle to inspect
    /// the exact `zellij action` calls it issued.
    fn fixture() -> (Rc<FakeDriver>, HarnessTabs) {
        let driver = Rc::new(FakeDriver::default());
        let tabs = HarnessTabs::with_driver("grove-acme".into(), Box::new(Rc::clone(&driver)));
        (driver, tabs)
    }

    fn repo() -> PathBuf {
        PathBuf::from("/work/acme")
    }

    #[test]
    fn first_open_creates_and_tracks_a_tab() {
        let (driver, mut h) = fixture();
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened);
        assert!(h.is_open("auth"));
        assert_eq!(driver.calls(), vec!["new_tab grove-acme auth -> 1"]);
    }

    #[test]
    fn reopen_focuses_existing_tab_without_a_duplicate() {
        let (driver, mut h) = fixture();
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened);
        // A second invocation on the same grove focuses its tab, never opens a
        // duplicate.
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Focused);
        assert_eq!(
            driver.calls(),
            vec![
                "new_tab grove-acme auth -> 1".to_string(),
                "go_to_tab grove-acme 1".to_string(),
            ]
        );
    }

    #[test]
    fn close_forgets_the_tab_then_is_a_noop() {
        let (driver, mut h) = fixture();
        h.open_or_focus("auth", &repo()).unwrap();
        assert_eq!(h.close("auth").unwrap(), Action::Closed);
        assert!(!h.is_open("auth"));
        // Closing again is a no-op: nothing is tracked.
        assert_eq!(h.close("auth").unwrap(), Action::NotOpen);
        assert_eq!(
            driver.calls(),
            vec![
                "new_tab grove-acme auth -> 1".to_string(),
                "close_tab grove-acme 1".to_string(),
            ]
        );
    }

    #[test]
    fn stale_go_to_tab_failure_reopens_a_fresh_tab() {
        let (driver, mut h) = fixture();
        *driver.go_to_fails.borrow_mut() = true;
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened); // tab 1
        // go_to_tab fails (the user closed the tab in zellij) → drop the stale
        // id and open a fresh tab rather than focusing a ghost.
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened); // tab 2
        assert!(h.is_open("auth"));
        assert_eq!(
            driver.calls(),
            vec![
                "new_tab grove-acme auth -> 1".to_string(),
                "go_to_tab grove-acme 1".to_string(),
                "new_tab grove-acme auth -> 2".to_string(),
            ]
        );
    }

    #[test]
    fn distinct_groves_get_distinct_tabs() {
        let (_driver, mut h) = fixture();
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened);
        assert_eq!(h.open_or_focus("billing", &repo()).unwrap(), Action::Opened);
        assert!(h.is_open("auth") && h.is_open("billing"));
    }
}

//! Controller-side **harness driving**: open / focus / close `grove do <name>`
//! sessions as native zellij panes, tracking each grove's stable pane id
//! (leaf `060-harness-pane/040-grove-integration/040-harness-driving`).
//!
//! Per ADR-0015 the substrate is grove-owned zellij and harnesses are native
//! zellij panes; per ADR-0016 the **controller** owns all pane-decision state
//! and drives zellij with grove's existing shell-out idiom — here, the
//! `zellij action` verbs the 020 spike validated on zellij 0.44.3:
//!
//! - `zellij --session <s> action new-pane --cwd <repo> -- <grove> do <name>`
//!   opens the harness and **prints the created pane id** (`terminal_<n>`) on
//!   stdout — so pane-id discovery is native, no `dump-layout` parse needed.
//! - `zellij --session <s> action focus-pane-id <id>` switches focus to it.
//! - `zellij --session <s> action close-pane --pane-id <id>` closes it.
//!
//! The controller runs *outside* zellij (it spawned zellij as a child, then
//! renders into a proxy pane inside it), so every action targets the session
//! **explicitly** via `--session`. That same explicitness is what keeps the
//! driving layer repo-agnostic: [`HarnessPanes::open_or_focus`] takes the repo
//! as an argument and opens with `--cwd <repo>`, so 070's cross-repo fleet
//! reuses it unchanged.
//!
//! The pane map ([`HarnessPanes`]) is the controller's single source of truth
//! (ADR-0016): no pane-decision state lives in any proxy.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};

/// The three `zellij action` shell-outs, behind a trait so the
/// [`HarnessPanes`] state machine is unit-testable without a running zellij.
/// The production implementation is [`ZellijCli`]; tests substitute a fake.
pub trait ZellijDriver {
    /// Open `grove do <name>` in `repo` as a new pane in `session`, returning
    /// the created pane id (`terminal_<n>`), parsed from zellij's stdout.
    fn new_pane(&self, session: &str, name: &str, repo: &Path) -> Result<String>;
    /// Focus the pane with stable id `id` in `session`. Errors if the pane is
    /// gone — [`HarnessPanes::open_or_focus`] treats that as "reopen".
    fn focus(&self, session: &str, id: &str) -> Result<()>;
    /// Close the pane with stable id `id` in `session`.
    fn close(&self, session: &str, id: &str) -> Result<()>;
}

/// What a drive request did, for the dashboard status line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// A new harness pane was opened (and zellij focused it).
    Opened,
    /// The grove's existing harness pane was focused.
    Focused,
    /// The grove's harness pane was closed and forgotten.
    Closed,
    /// Close was a no-op: no harness pane was tracked for the grove.
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

/// The controller's record of which groves have a live harness pane, keyed by
/// grove name → stable zellij pane id. The single source of truth for
/// pane decisions (ADR-0016).
pub struct HarnessPanes {
    /// The zellij session the dashboard runs in; every action targets it.
    session: String,
    /// grove name → stable pane id (`terminal_<n>`).
    panes: BTreeMap<String, String>,
    /// The `zellij action` driver (real CLI in production, fake in tests).
    driver: Box<dyn ZellijDriver>,
}

impl HarnessPanes {
    /// Build a tracker that drives the real `zellij` CLI for `session`.
    pub fn new(session: String) -> Self {
        Self::with_driver(session, Box::new(ZellijCli))
    }

    /// Build a tracker with an injected driver (used by tests).
    pub fn with_driver(session: String, driver: Box<dyn ZellijDriver>) -> Self {
        Self {
            session,
            panes: BTreeMap::new(),
            driver,
        }
    }

    /// Open the harness for `name` (running `grove do <name>` in `repo`) as a
    /// native zellij pane, or focus its existing pane if one is already open —
    /// so a second invocation on the same grove switches to it rather than
    /// opening a duplicate. If focusing fails (the user closed the pane in
    /// zellij), the stale id is forgotten and a fresh pane is opened.
    ///
    /// `repo` is explicit (opened with `--cwd <repo>`) so the cross-repo fleet
    /// (070) reuses this driving layer unchanged.
    pub fn open_or_focus(&mut self, name: &str, repo: &Path) -> Result<Action> {
        if let Some(id) = self.panes.get(name).cloned() {
            match self.driver.focus(&self.session, &id) {
                Ok(()) => return Ok(Action::Focused),
                // The tracked pane is gone — drop it and fall through to reopen.
                Err(_) => {
                    self.panes.remove(name);
                }
            }
        }
        let id = self
            .driver
            .new_pane(&self.session, name, repo)
            .with_context(|| format!("opening harness pane for '{name}'"))?;
        self.panes.insert(name.to_string(), id);
        Ok(Action::Opened)
    }

    /// Close the harness pane for `name` and forget its id. A no-op
    /// ([`Action::NotOpen`]) when no pane is tracked for the grove.
    pub fn close(&mut self, name: &str) -> Result<Action> {
        match self.panes.remove(name) {
            Some(id) => {
                self.driver
                    .close(&self.session, &id)
                    .with_context(|| format!("closing harness pane for '{name}'"))?;
                Ok(Action::Closed)
            }
            None => Ok(Action::NotOpen),
        }
    }

    /// Whether a harness pane is currently tracked for `name`.
    #[cfg(test)]
    pub fn is_open(&self, name: &str) -> bool {
        self.panes.contains_key(name)
    }
}

// ---------------------------------------------------------------------------
// The real driver: shell out to `zellij action`.

/// Production [`ZellijDriver`] that shells out to the installed `zellij`,
/// exactly as the 020 spike verified (zellij 0.44.3).
pub struct ZellijCli;

impl ZellijDriver for ZellijCli {
    fn new_pane(&self, session: &str, name: &str, repo: &Path) -> Result<String> {
        // Run the same `grove` binary as the controller so dev builds
        // (target/debug/grove) and installed builds both open a matching
        // harness, mirroring how the head binary resolves the proxy command.
        let grove_bin = std::env::current_exe().context("locating the running grove binary")?;
        let out = Command::new("zellij")
            .args(new_pane_args(session, &grove_bin.display().to_string(), name, repo))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .context("running zellij action new-pane")?;
        if !out.status.success() {
            anyhow::bail!(
                "zellij action new-pane failed (status {:?}): {}",
                out.status.code(),
                String::from_utf8_lossy(&out.stderr).trim()
            );
        }
        parse_pane_id(&String::from_utf8_lossy(&out.stdout))
    }

    fn focus(&self, session: &str, id: &str) -> Result<()> {
        let out = Command::new("zellij")
            .args(focus_args(session, id))
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output()
            .context("running zellij action focus-pane-id")?;
        if out.status.success() {
            return Ok(());
        }
        let stderr = String::from_utf8_lossy(&out.stderr);
        // zellij exits non-zero when the target pane is *already* the focused
        // one ("Pane … is already focused"). That is success for us — the pane
        // the user asked to switch to is focused. Any other non-zero means the
        // pane is gone/unknown, which `open_or_focus` turns into a reopen.
        if focus_already_satisfied(&stderr) {
            return Ok(());
        }
        anyhow::bail!(
            "zellij action focus-pane-id failed (status {:?}): {}",
            out.status.code(),
            stderr.trim()
        );
    }

    fn close(&self, session: &str, id: &str) -> Result<()> {
        run_action(&close_args(session, id), "close-pane")
    }
}

/// Whether a non-zero `focus-pane-id` exit is benign: zellij reports an error
/// when the requested pane is already the focused pane, which is the outcome we
/// wanted. Distinguishing this from a genuinely-gone pane keeps
/// [`HarnessPanes::open_or_focus`] from spuriously opening a duplicate when the
/// grove's harness happens to already be focused.
fn focus_already_satisfied(stderr: &str) -> bool {
    stderr.to_lowercase().contains("already focused")
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

/// Argv for `zellij --session <s> action new-pane --cwd <repo> --name … --
/// <grove> do <name>`. The `--session` flag is global (before `action`) so the
/// controller, which lives outside zellij, targets the dashboard's session.
fn new_pane_args(session: &str, grove_bin: &str, name: &str, repo: &Path) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "new-pane".into(),
        "--cwd".into(),
        repo.display().to_string(),
        "--name".into(),
        format!("grove: {name}"),
        "--".into(),
        grove_bin.into(),
        "do".into(),
        name.into(),
    ]
}

/// Argv for `zellij --session <s> action focus-pane-id <id>` (id is positional).
fn focus_args(session: &str, id: &str) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "focus-pane-id".into(),
        id.into(),
    ]
}

/// Argv for `zellij --session <s> action close-pane --pane-id <id>`.
fn close_args(session: &str, id: &str) -> Vec<String> {
    vec![
        "--session".into(),
        session.into(),
        "action".into(),
        "close-pane".into(),
        "--pane-id".into(),
        id.into(),
    ]
}

/// Parse the pane id `zellij action new-pane` prints on stdout. zellij emits
/// the id (`terminal_<n>`, or for plugins `plugin_<n>`) — take the last
/// non-empty line and sanity-check it looks like an id zellij will later accept
/// for `focus-pane-id` / `close-pane --pane-id` (those also accept a bare int).
fn parse_pane_id(stdout: &str) -> Result<String> {
    let id = stdout
        .lines()
        .map(str::trim)
        .rfind(|l| !l.is_empty())
        .unwrap_or("");
    if id.is_empty() {
        anyhow::bail!("zellij new-pane returned no pane id (empty stdout)");
    }
    let digits = id
        .strip_prefix("terminal_")
        .or_else(|| id.strip_prefix("plugin_"))
        .unwrap_or(id);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        anyhow::bail!("unexpected zellij new-pane output (not a pane id): {id:?}");
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
    fn new_pane_argv_targets_session_cwd_and_grove_do() {
        let args = new_pane_args("grove-acme", "/opt/grove", "auth", Path::new("/work/acme"));
        assert_eq!(
            args,
            vec![
                "--session", "grove-acme",
                "action", "new-pane",
                "--cwd", "/work/acme",
                "--name", "grove: auth",
                "--", "/opt/grove", "do", "auth",
            ]
        );
    }

    #[test]
    fn focus_argv_is_positional_pane_id() {
        assert_eq!(
            focus_args("grove-acme", "terminal_3"),
            vec!["--session", "grove-acme", "action", "focus-pane-id", "terminal_3"]
        );
    }

    #[test]
    fn close_argv_uses_pane_id_flag() {
        assert_eq!(
            close_args("grove-acme", "terminal_3"),
            vec!["--session", "grove-acme", "action", "close-pane", "--pane-id", "terminal_3"]
        );
    }

    #[test]
    fn parse_pane_id_accepts_terminal_plugin_and_bare_int() {
        assert_eq!(parse_pane_id("terminal_4\n").unwrap(), "terminal_4");
        assert_eq!(parse_pane_id("plugin_2").unwrap(), "plugin_2");
        assert_eq!(parse_pane_id("7\n").unwrap(), "7");
        // Tolerates leading chatter: the id is the last non-empty line.
        assert_eq!(parse_pane_id("starting...\nterminal_9\n").unwrap(), "terminal_9");
    }

    #[test]
    fn parse_pane_id_rejects_empty_or_garbage() {
        assert!(parse_pane_id("").is_err());
        assert!(parse_pane_id("\n  \n").is_err());
        assert!(parse_pane_id("not-an-id").is_err());
        assert!(parse_pane_id("terminal_").is_err());
    }

    #[test]
    fn focus_already_satisfied_recognises_zellij_message() {
        // The exact message zellij 0.44.3 emits when re-focusing the focused
        // pane (captured live: "Pane Terminal(1) is already focused").
        assert!(focus_already_satisfied("Pane Terminal(1) is already focused"));
        assert!(focus_already_satisfied("PANE IS ALREADY FOCUSED"));
        // A genuinely-missing pane is not benign — it must drive a reopen.
        assert!(!focus_already_satisfied("Pane with id terminal_9 not found"));
        assert!(!focus_already_satisfied(""));
    }

    // -- state machine with a fake driver --------------------------------

    /// Records every call and lets a test script the next pane id and whether
    /// the next focus should fail (the user closed the pane in zellij).
    #[derive(Default)]
    struct FakeDriver {
        calls: RefCell<Vec<String>>,
        next_id: RefCell<u32>,
        focus_fails: RefCell<bool>,
    }

    impl FakeDriver {
        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    // Implemented on `Rc<FakeDriver>` so a test can hold one handle to inspect
    // the call log while `HarnessPanes` drives through another.
    impl ZellijDriver for Rc<FakeDriver> {
        fn new_pane(&self, session: &str, name: &str, _repo: &Path) -> Result<String> {
            *self.next_id.borrow_mut() += 1;
            let id = format!("terminal_{}", self.next_id.borrow());
            self.calls
                .borrow_mut()
                .push(format!("new_pane {session} {name} -> {id}"));
            Ok(id)
        }
        fn focus(&self, session: &str, id: &str) -> Result<()> {
            self.calls.borrow_mut().push(format!("focus {session} {id}"));
            if *self.focus_fails.borrow() {
                anyhow::bail!("pane {id} not found");
            }
            Ok(())
        }
        fn close(&self, session: &str, id: &str) -> Result<()> {
            self.calls.borrow_mut().push(format!("close {session} {id}"));
            Ok(())
        }
    }

    /// A `HarnessPanes` over a shared fake driver, plus the handle to inspect
    /// the exact `zellij action` calls it issued.
    fn fixture() -> (Rc<FakeDriver>, HarnessPanes) {
        let driver = Rc::new(FakeDriver::default());
        let panes = HarnessPanes::with_driver("grove-acme".into(), Box::new(Rc::clone(&driver)));
        (driver, panes)
    }

    fn repo() -> PathBuf {
        PathBuf::from("/work/acme")
    }

    #[test]
    fn first_open_creates_and_tracks_a_pane() {
        let (driver, mut h) = fixture();
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened);
        assert!(h.is_open("auth"));
        assert_eq!(driver.calls(), vec!["new_pane grove-acme auth -> terminal_1"]);
    }

    #[test]
    fn reopen_focuses_existing_pane_without_a_duplicate() {
        let (driver, mut h) = fixture();
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened);
        // A second invocation on the same grove focuses, never opens a duplicate.
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Focused);
        assert_eq!(
            driver.calls(),
            vec![
                "new_pane grove-acme auth -> terminal_1".to_string(),
                "focus grove-acme terminal_1".to_string(),
            ]
        );
    }

    #[test]
    fn close_forgets_the_pane_then_is_a_noop() {
        let (driver, mut h) = fixture();
        h.open_or_focus("auth", &repo()).unwrap();
        assert_eq!(h.close("auth").unwrap(), Action::Closed);
        assert!(!h.is_open("auth"));
        // Closing again is a no-op: nothing is tracked.
        assert_eq!(h.close("auth").unwrap(), Action::NotOpen);
        assert_eq!(
            driver.calls(),
            vec![
                "new_pane grove-acme auth -> terminal_1".to_string(),
                "close grove-acme terminal_1".to_string(),
            ]
        );
    }

    #[test]
    fn stale_focus_failure_reopens_a_fresh_pane() {
        let (driver, mut h) = fixture();
        *driver.focus_fails.borrow_mut() = true;
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened); // terminal_1
        // Focus fails (the user closed the pane in zellij) → drop the stale id
        // and open a fresh pane rather than focusing a ghost.
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened); // terminal_2
        assert!(h.is_open("auth"));
        assert_eq!(
            driver.calls(),
            vec![
                "new_pane grove-acme auth -> terminal_1".to_string(),
                "focus grove-acme terminal_1".to_string(),
                "new_pane grove-acme auth -> terminal_2".to_string(),
            ]
        );
    }

    #[test]
    fn distinct_groves_get_distinct_panes() {
        let (_driver, mut h) = fixture();
        assert_eq!(h.open_or_focus("auth", &repo()).unwrap(), Action::Opened);
        assert_eq!(h.open_or_focus("billing", &repo()).unwrap(), Action::Opened);
        assert!(h.is_open("auth") && h.is_open("billing"));
    }
}

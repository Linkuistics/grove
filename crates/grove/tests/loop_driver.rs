// Integration tests for the self-driving loop (src/loop_driver.rs).
//
// Every case drives the real Grove process with a temporary HOME and personal
// Ravel policy. The independently installed front and worker select fake agents
// through an isolated PATH; GROVE_TEST_RAVEL names the test installation.
// There is no Grove launch configuration or in-process driver shortcut.
// These cases observe epoch admission, stderr, cancellation and terminal recovery.
// Ravel owns harness supervision; Grove interprets its opaque result after reap.

mod support;

use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsString;
use std::fs;
use std::os::unix::ffi::{OsStrExt, OsStringExt};
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Output, Stdio};
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// This build's own agent CLI, for fixtures that need to *call* it: they bake
/// this path into their script rather than relying on `PATH`.
///
/// The driver itself never invokes `grove-llm` — it did once, to probe the
/// build pairing, and that probe went with provisioning at
/// `delete-provisioning-k19` — so nothing here has to point it anywhere and a
/// suite run on a machine whose installed CLI is a different build is
/// unaffected.
/// It is a function rather than a `const` now: `grove-llm` is its own package
/// since `loop-crate-verbs-k21`, so its path is not a compile-time constant of
/// *this* package any more.
fn own_grove_llm() -> PathBuf {
    support::grove_llm()
}

fn init_worktree(dir: &Path) {
    init_jj_worktree(dir, false);
}

fn jj(dir: &Path, args: &[&str]) {
    assert!(
        Command::new("jj")
            .args([
                "--config",
                "user.name=Test",
                "--config",
                "user.email=t@example.com",
            ])
            .args(args)
            .current_dir(dir)
            .status()
            .unwrap()
            .success(),
        "jj {args:?} failed"
    );
}

fn write_exec(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut perms = fs::metadata(path).unwrap().permissions();
    perms.set_mode(0o755);
    fs::set_permissions(path, perms).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

/// The prefix of the driver-authored sentence naming the leaf selected for one
/// session, up to the opening backtick of the handle.
///
/// The one prompt sentence a fixture may read the handle out of. The driver
/// authors prose only for the facts it resolves at runtime — the selected handle
/// and the stated version control — and everything else in `${prompt}` is either
/// the fixed load instruction or grove's own signalling contract, whose prose
/// names handle-shaped strings of its own that a looser marker would find.
const MANDATED_LEAF: &str = "Grove mandate: the leaf selected for this session is `";

/// Plant a minimal current-format tree with one live leaf.
fn plant_tree(worktree: &Path, leaf: &str) {
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(grove.join(leaf), "# planted\n").unwrap();
}

/// A missing production scrub leaks the enclosing ending context to a policy
/// that the owner explicitly granted it, before any fresh session is launched.
#[test]
fn lifecycle_scrubs_enclosing_ravel_and_retired_controls_before_selection() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let controls = [
        "GROVE_SIGNAL_FILE",
        "GROVE_RUN_SIGNAL_FILE",
        "GROVE_HARNESS_PID",
        "GROVE_CLAUDE_PID",
        "GROVE_PROVIDER_EVIDENCE",
    ];
    dispatch.policy(&format!(
        "export const policy = {{ schemaVersion: 2, version: 'authority-probe', \
         select: () => {{\n\
         if (process.env.OWNER_GRANT !== 'granted' || \
         {}.some(name => process.env[name] !== undefined)) \
         return {{ status: 'refused', code: 'enclosing_authority', \
         message: 'enclosing authority reached selection', remedy: 'scrub before selection' }};\n\
         return {{ status: 'selected', program: {:?}, args: [], provider: 'fixture', \
         model: 'none', effort: 'none', reason: 'scrubbed controls' }}; }} }};\n",
        serde_json::to_string(&controls).unwrap(),
        dispatch.harness.to_str().unwrap(),
    ));
    dispatch.settings(&serde_json::json!({"policyEnv": ["OWNER_GRANT", "GROVE_SIGNAL_FILE", "GROVE_RUN_SIGNAL_FILE", "GROVE_HARNESS_PID", "GROVE_CLAUDE_PID", "GROVE_PROVIDER_EVIDENCE"]}).to_string());

    let mut direct = Command::new(support::installed_ravel());
    for name in support::grove_env_names() {
        direct.env_remove(name);
    }
    direct
        .current_dir(&dispatch.worktree)
        .env("HOME", &dispatch.home)
        .env("OWNER_GRANT", "granted")
        .args(["start-session", "--kind=impl", "--prompt=control"]);
    for name in controls {
        let dirty = direct
            .env(name, "/inert-enclosing-control")
            .output()
            .unwrap();
        assert_eq!(dirty.status.code(), Some(3), "{name}: {dirty:?}");
        assert!(String::from_utf8_lossy(&dirty.stderr)
            .contains("enclosing authority reached selection"));
        direct.env_remove(name);
    }
    let probe = support::ravel_entry_probe(&dispatch.root.join("entry-bin"));
    support::observe_dirty_entry(&probe, &dispatch.home);

    let mut grove = grove_driver(&dispatch.worktree, &dispatch.home);
    grove
        .env("OWNER_GRANT", "granted")
        .env("PATH", support::path_with_entry(probe.parent().unwrap()));
    for name in support::LOOP_CONTROL_ENV {
        grove.env(name, "/inert-enclosing-control");
    }
    let output = grove.output().unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("grove finished — loop complete"),
        "{output:?}"
    );
    assert_eq!(dispatch.launch_count(), 1);
}

/// The bare driver, with the whole legacy launch-policy environment scrubbed
/// out of the child. A `Command` inherits this process's ambient environment,
/// and this repo dogfoods Grove, so scrubbing is what makes the fixture the
/// only input.
fn driver_command(worktree: &Path, home: &Path) -> Command {
    // The sibling this `grove` launches every session through.
    support::installed_ravel();
    driver_command_at(Path::new(env!("CARGO_BIN_EXE_grove")), worktree, home)
}

/// [`driver_command`] for the `grove` executable at `grove`.
fn driver_command_at(grove: &Path, worktree: &Path, home: &Path) -> Command {
    let mut command = Command::new(grove);
    command.current_dir(worktree);
    for name in support::grove_env_names() {
        command.env_remove(name);
    }
    // A dispatched session's run identity is ambient to everything under it,
    // as its channel is. This suite runs inside such a session, and its driver
    // must start as a human's does, with neither.
    for name in ["HARNESS_DISPATCH_RUN_ID", "HARNESS_DISPATCH_STATE_DIR"] {
        command.env_remove(name);
    }
    command.env("HOME", home).env("PATH", support::ravel_path());
    command
}

/// [`driver_command`], detached from every terminal.
fn grove_driver(worktree: &Path, home: &Path) -> Command {
    detached(driver_command(worktree, home))
}

/// `command`, detached from every terminal.
fn detached(mut command: Command) -> Command {
    // Captured output does not detach /dev/tty. A driver sharing cargo's
    // session can hand the developer's terminal to its fake child, leaving
    // the release job in the background and vulnerable to SIGTTOU. Keep both
    // terminal ownership and input outside these noninteractive fixtures.
    command.stdin(Stdio::null());
    // SAFETY: setsid is async-signal-safe, and this child is not yet a process
    // group leader. Failure is reported through Command's spawn error path.
    unsafe {
        command.pre_exec(|| {
            if libc::setsid() == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    command
}

#[test]
fn test_sessions_cannot_access_the_runners_terminal() {
    const UNDER_TERMINAL: &str = "GROVE_TEST_RUNNER_TERMINAL";
    if std::env::var_os(UNDER_TERMINAL).is_none() {
        use std::os::fd::{AsRawFd, FromRawFd};

        // Re-run this test with a real controlling terminal, even when cargo
        // itself is headless. Merely redirecting stdin leaves /dev/tty usable.
        let (mut master, mut slave) = (-1, -1);
        // SAFETY: valid out pointers; default terminal settings and size.
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut master,
                    &mut slave,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            },
            0
        );
        // SAFETY: openpty returned two fresh owned descriptors.
        let (_master, slave) =
            unsafe { (fs::File::from_raw_fd(master), fs::File::from_raw_fd(slave)) };
        for fd in [_master.as_raw_fd(), slave.as_raw_fd()] {
            // SAFETY: live descriptors; neither endpoint should leak on exec.
            assert_ne!(
                unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
                -1
            );
        }
        let capture = TempDir::new().unwrap();
        let diagnostics = capture.path().join("test-output");
        let output = fs::File::create(&diagnostics).unwrap();
        let mut command = Command::new(std::env::current_exe().unwrap());
        command
            .args([
                "--exact",
                "test_sessions_cannot_access_the_runners_terminal",
                "--nocapture",
            ])
            .env(UNDER_TERMINAL, "1")
            .stdin(Stdio::from(slave.try_clone().unwrap()))
            .stdout(Stdio::from(output.try_clone().unwrap()))
            .stderr(Stdio::from(output));
        // SAFETY: only async-signal-safe calls between fork and exec. The new
        // session owns this private PTY, never the developer's terminal.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() == -1
                    || libc::ioctl(libc::STDIN_FILENO, libc::TIOCSCTTY as _, 0) == -1
                {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        let mut child = command.spawn().unwrap();
        let deadline = Instant::now() + Duration::from_secs(20);
        let status = loop {
            if let Some(status) = child.try_wait().unwrap() {
                break status;
            }
            if Instant::now() >= deadline {
                // SIGKILLing the nested runner skips DriverProcess::drop. Its
                // driver and configured session have separate process groups,
                // so stop each parent before walking and killing its children.
                fn kill_tree(pid: libc::pid_t) {
                    // SAFETY: only this probe's subprocess tree is traversed.
                    if unsafe { libc::kill(pid, libc::SIGSTOP) } != 0 {
                        return;
                    }
                    for (child, _) in children_of(pid) {
                        kill_tree(child);
                    }
                    // SAFETY: the stopped process cannot fork more children.
                    unsafe { libc::kill(pid, libc::SIGKILL) };
                }
                kill_tree(child.id() as libc::pid_t);
                child.wait().unwrap();
                panic!(
                    "terminal isolation probe timed out: {}",
                    fs::read_to_string(&diagnostics).unwrap()
                );
            }
            thread::sleep(Duration::from_millis(10));
        };
        assert!(
            status.success(),
            "{}",
            fs::read_to_string(diagnostics).unwrap()
        );
        return;
    }
    assert!(
        fs::File::open("/dev/tty").is_ok(),
        "probe must own a terminal"
    );
    // SAFETY: isatty inspects the inherited descriptor without changing it.
    assert_eq!(unsafe { libc::isatty(libc::STDIN_FILENO) }, 1);

    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        r#"#!/bin/sh
if ( : < /dev/tty ) 2>/dev/null; then
    echo inherited-controlling-terminal
fi
if [ -t 0 ]; then
    echo inherited-terminal-input
fi
echo session-finished
"#,
    );
    support::route_every_kind_to(&home, &configured);
    let output = run_driver(&worktree, &home);
    assert!(output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("session-finished\n"),
        "a test session must not be able to change the runner's terminal"
    );
}

/// A driver process whose streams are captured to **files** rather than pipes,
/// and which takes the session it launched down with it when it is killed.
///
/// Two hazards live in the obvious `Stdio::piped()` spawn, and both of them
/// present as a wedged suite rather than a failed assertion
/// (driver-test-timeout-path-unbounded-k194):
///
/// - **The pipe outlives the driver.** The configured session inherits the
///   driver's stderr, and `Child::output` and `Child::wait_with_output` read to
///   EOF — which arrives only when *every* writer has closed. So a branch that
///   kills the driver in order to *report* what it said blocks in `read()`
///   forever instead, because the session it left behind still holds the write
///   end. A file has no writers to wait for, so the collection below is bounded
///   by the driver's own lifetime and nothing else, and it still picks up what
///   a surviving grandchild wrote.
/// - **The session outlives the run.** `Child::kill` signals the driver alone,
///   and `keyed-launch` puts every session in a process group of its own
///   (`run.rs`, `command.process_group(0)`), so a SIGKILLed driver leaves its
///   `configured-command.sh` running and reparented to pid 1. Those accumulate,
///   and idle orphans are load — which is what pushes these fixtures past their
///   deadlines in the first place.
///
/// The idiom is not new here: `tests/driver_lease.rs` captures its driver to a
/// file for the neighbouring reason recorded at
/// `driver-lease-readiness-flake-k145`, which is also why nulling the streams is
/// not the answer — a driver that stopped before its session started has only
/// that account to offer.
struct DriverProcess {
    child: Child,
    stdout: PathBuf,
    stderr: PathBuf,
    /// Held for its lifetime, not read: dropping it removes the capture files.
    _capture: TempDir,
}

impl DriverProcess {
    fn spawn(worktree: &Path, home: &Path) -> Self {
        Self::capture(grove_driver(worktree, home))
    }

    fn capture(mut command: Command) -> Self {
        let capture = TempDir::new().unwrap();
        let stdout = capture.path().join("stdout");
        let stderr = capture.path().join("stderr");
        let child = command
            .stdout(Stdio::from(fs::File::create(&stdout).unwrap()))
            .stderr(Stdio::from(fs::File::create(&stderr).unwrap()))
            .spawn()
            .unwrap();
        Self {
            child,
            stdout,
            stderr,
            _capture: capture,
        }
    }

    fn id(&self) -> libc::pid_t {
        self.child.id() as libc::pid_t
    }

    fn try_wait(&mut self) -> Option<ExitStatus> {
        self.child.try_wait().unwrap()
    }

    /// The file the driver's own diagnostics are accumulating in — readable
    /// *while it runs*, which a pipe could not offer without a drain thread.
    fn diagnostics(&self) -> &Path {
        &self.stderr
    }

    fn wait_for_ready(&mut self, marker: &Path) {
        let diagnostics = self.stderr.clone();
        support::wait_for_ready(marker, &mut self.child, Some(&diagnostics));
    }

    /// SIGKILL the driver **and** the process group of every session it
    /// launched, then reap it.
    ///
    /// The children are read *before* the signal lands. A dead parent's children
    /// are reparented to pid 1 immediately, and that erases the only link back
    /// to them — which is how the orphans this type exists to stop got loose.
    fn kill(&mut self) {
        let mut sessions = children_of(self.id());
        let mut next = 0;
        while next < sessions.len() {
            sessions.extend(children_of(sessions[next].0));
            next += 1;
        }
        let _ = self.child.kill();
        let _ = self.child.wait();
        for (pid, group) in sessions {
            // SAFETY: `kill(2)` on a process this driver spawned. The negative
            // pid reaches the whole group — the tools the session itself
            // launched — and is sent only when the child really is that group's
            // leader, so a pid that happens to match an unrelated group id
            // cannot be mistaken for one. Nothing reaps the session between the
            // `ps` above and here, so the pid cannot have been recycled under
            // us in the window.
            unsafe {
                if group == pid {
                    libc::kill(-group, libc::SIGKILL);
                }
                libc::kill(pid, libc::SIGKILL);
            }
        }
    }

    /// Block until the driver exits, then collect what it wrote.
    fn finish(&mut self) -> Output {
        let status = self.child.wait().unwrap();
        Output {
            status,
            stdout: fs::read(&self.stdout).unwrap_or_default(),
            stderr: fs::read(&self.stderr).unwrap_or_default(),
        }
    }

    /// [`DriverProcess::finish`], failing the test rather than hanging if the
    /// driver is still running after `limit`. The driver and every session it
    /// launched are killed first, and the failure carries what it said.
    fn finish_within(&mut self, limit: Duration) -> Output {
        let deadline = Instant::now() + limit;
        while self.try_wait().is_none() {
            if Instant::now() >= deadline {
                self.kill();
                panic!(
                    "the driver was still running after {limit:?}: {}",
                    diagnostics_of(&self.stderr)
                );
            }
            thread::sleep(Duration::from_millis(20));
        }
        self.finish()
    }
}

impl Drop for DriverProcess {
    /// A test that panics before it finishes must not leave the driver — or the
    /// session behind it — running.
    fn drop(&mut self) {
        if self.child.try_wait().unwrap_or(None).is_none() {
            self.kill();
        }
    }
}

/// The direct children of `parent`, each paired with its process-group id.
///
/// Through `ps` because there is no portable process-table read in `std`, and
/// this runs only on a teardown path where one extra process is free.
fn children_of(parent: libc::pid_t) -> Vec<(libc::pid_t, libc::pid_t)> {
    let table = Command::new("ps")
        .args(["-eo", "pid=,ppid=,pgid="])
        .output()
        .unwrap();
    String::from_utf8_lossy(&table.stdout)
        .lines()
        .filter_map(|line| {
            let mut fields = line.split_whitespace();
            let pid = fields.next()?.parse().ok()?;
            let ppid: libc::pid_t = fields.next()?.parse().ok()?;
            let group = fields.next()?.parse().ok()?;
            (ppid == parent).then_some((pid, group))
        })
        .collect()
}

fn run_driver(worktree: &Path, home: &Path) -> Output {
    DriverProcess::spawn(worktree, home).finish()
}

// The session epoch is what admits an agent's `grove-llm` calls, so its window
// has to be exactly the child's lifetime: active before the spawn (or the very
// first call the session makes is refused) and inactive after the reap (or a
// dead session's signal path stays admissible). Observed from *inside* the
// session, which is the only vantage point that can tell "active before spawn"
// from "active shortly after".
#[test]
fn the_driver_activates_immediately_before_spawn_and_invalidates_after_reap() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");

    let observed_epoch = fixture.path().join("observed-epoch");
    let observed_signal = fixture.path().join("observed-signal");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\ncp \"$PWD/.jj/grove/session.epoch\" {epoch}\nprintf '%s\\n' \"$GROVE_LAUNCH_DIR/result-placeholder\" > {signal}\nexit 0\n",
            epoch = shell_quote(&observed_epoch),
            signal = shell_quote(&observed_signal),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    let output = run_driver(&worktree, &home);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );

    let during_launch = fs::read_to_string(&observed_epoch).unwrap();
    let signal_path = fs::read_to_string(&observed_signal).unwrap();
    assert!(
        during_launch.starts_with("state=active\n"),
        "{during_launch:?}"
    );
    let signal_hex = Path::new(signal_path.trim_end())
        .parent()
        .unwrap()
        .as_os_str()
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<Vec<_>>()
        .join("");
    assert!(
        during_launch.contains(&format!("launch-dir-hex={signal_hex}\n")),
        "the live epoch must name this launch's own channel: {during_launch:?}"
    );
    let after_reap = fs::read_to_string(worktree.join(".jj/grove/session.epoch")).unwrap();
    assert!(after_reap.starts_with("state=inactive\n"), "{after_reap:?}");
    assert!(!after_reap.contains("launch-dir-hex="), "{after_reap:?}");
}

// Every `grove-llm` mutator takes the **exclusive** tree-access lock on the
// working-tree root, and the driver takes that same lock to select. If the
// driver held its guard across the launch window, the first `leaf-add` any
// session ran would block until the session holding it exited — the loop would
// deadlock on its own first task, and every grove would stall at Retire.
//
// A read cannot detect that: `pick` takes a *shared* lock, so a driver still
// holding one would admit it. Only a session-side **mutation** proves release,
// which is why this drives the two real grow/retire verbs rather than writing
// the files directly the way the mandate fixtures do.
//
// Two iterations, because release is only half the claim: the mutations must
// also be what the *next* pick sees. The session retires its own leaf and adds
// the successor, and the loop must then select that successor by name.
#[test]
fn a_session_mutates_the_tree_through_grove_llm_without_deadlocking_the_driver() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");

    let mandates = fixture.path().join("mandates");
    let verbs = fixture.path().join("verb-output");
    let first_run = fixture.path().join("first-run");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\n\
             printf '%s\\n' \"$1\" >> {mandates}\n\
             if [ -e {first_run} ]; then exit 0; fi\n\
             : > {first_run}\n\
             {grove_llm} leaf-add . follow-up --kind design >> {verbs} 2>&1 || exit 91\n\
             {grove_llm} leaf-retire .grove/01-impl--subject-k1.md >> {verbs} 2>&1 || exit 92\n\
             ravel end-session grove-continue\n\
             exit 0\n",
            mandates = shell_quote(&mandates),
            verbs = shell_quote(&verbs),
            first_run = shell_quote(&first_run),
            grove_llm = shell_quote(&own_grove_llm()),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    // Bounded rather than `run_driver`, because the failure this test exists to
    // catch is a *hang*: a blocking wait would turn it into a stuck suite
    // instead of a named assertion.
    let mut driver = DriverProcess::spawn(&worktree, &home);
    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = driver.try_wait() {
            break status;
        }
        if Instant::now() >= deadline {
            driver.kill();
            let said = fs::read_to_string(driver.diagnostics()).unwrap_or_default();
            panic!(
                "the driver never returned: a session-side `grove-llm` mutation blocked on a \
                 tree-access guard the driver still held across its launch window. It said:\n\
                 {said}"
            );
        }
        thread::sleep(Duration::from_millis(50));
    };
    let output = driver.finish();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(status.success(), "{stderr}");
    let verb_output = fs::read_to_string(&verbs).unwrap_or_default();
    assert!(
        worktree
            .join(".grove/01-DONE-impl--subject-k1.md")
            .is_file(),
        "the session's own `leaf-retire` must have completed: {verb_output}"
    );
    assert!(
        worktree.join(".grove/02-design--follow-up-k2.md").is_file(),
        "the session's own `leaf-add` must have completed: {verb_output}"
    );
    // The lock blocks rather than failing fast, so "it completed" alone would
    // also hold for a guard released late. `acquire_worktree` announces every
    // wait, and a launch-window session must never see one.
    assert!(
        !verb_output.contains("waiting for active Grove tree operation"),
        "the driver must release its tree-access guard *before* launching, not \
         while the session waits on it: {verb_output}"
    );

    // The mandate is a multi-line prompt, so the handles are read out of it
    // rather than counted by line.
    let mandates = fs::read_to_string(&mandates).unwrap();
    let handles: Vec<&str> = mandates
        .match_indices(MANDATED_LEAF)
        .map(|(at, marker)| {
            let rest = &mandates[at + marker.len()..];
            &rest[..rest.find('`').expect("unterminated mandate handle")]
        })
        .collect();
    assert_eq!(
        handles,
        ["subject-k1", "follow-up-k2"],
        "the next pick must see the tree the previous session mutated: {mandates:?}"
    );
}

// The mandate **states** the VCS the driver resolved, so no session detects it
// (`docs/ARCHITECTURE.md#symmetric-vcs-rule`). One seam, driver-level: the real
// driver against a real working tree of each kind, the prompt read back out of
// the fake harness's own `$1`. A unit test of the formatter would assert a
// subset of the same claim while proving nothing about what a session receives.
//
// Two fixtures, because one is satisfiable by a hardcoded string. Grove drives
// a single lane now (`docs/adr/jj-is-the-only-lane.md`), so the pair that still
// has a real difference is **native** and **colocated**: one carries a `.git`
// beside its `.jj` and the other does not, and both must state the same thing
// about the same resolved root. A resolution that ever consulted `.git` again
// would separate them.
#[derive(Clone, Copy, Debug)]
enum Shape {
    Native,
    Colocated,
}

const JJ_CLAUSE: &str = "this working tree is jj-enabled (jj workspace root: ";

/// A jj working tree. Colocation is forced either way rather than inherited,
/// because an ambient jj config may default it on and would silently turn the
/// native fixture into a second copy of the colocated one.
fn init_jj_worktree(dir: &Path, colocate: bool) {
    fs::create_dir_all(dir).unwrap();
    assert!(
        Command::new("jj")
            .args([
                "--config",
                "user.name=Test",
                "--config",
                "user.email=t@example.com",
                "--config",
                &format!("git.colocate={colocate}"),
                "git",
                "init",
                "--quiet",
                ".",
            ])
            .current_dir(dir)
            .status()
            .unwrap()
            .success(),
        "jj git init failed"
    );
    assert_eq!(
        dir.join(".git").exists(),
        colocate,
        "the fixture must be exactly the shape it names"
    );
}

fn assert_the_mandate_states_the_resolved_vcs(shape: Shape) {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    fs::create_dir_all(&home).unwrap();
    let worktree = fixture.path().join("worktree");
    init_jj_worktree(&worktree, matches!(shape, Shape::Colocated));
    plant_tree(&worktree, "01-impl--subject-k1.md");

    let mandate_path = fixture.path().join("mandate");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\nprintf '%s' \"$1\" > {mandate}\nexit 0\n",
            mandate = shell_quote(&mandate_path),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    let output = run_driver(&worktree, &home);
    assert!(
        output.status.success(),
        "the driver must launch in a {shape:?} jj tree: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let mandate = fs::read_to_string(&mandate_path).unwrap();
    // The lease canonicalizes the working tree from its marker, so the stated
    // root is the resolved form — on macOS `/private/var/...`, not the
    // `/var/...` the fixture handed out.
    let root = worktree.canonicalize().unwrap();
    assert!(
        mandate.contains(&format!("{JJ_CLAUSE}`{}`)", root.display())),
        "a {shape:?} tree's mandate must state jj and the root the driver \
         resolved: {mandate:?}"
    );
    // **The value, and no consequence of it.** *Do not probe for it, and
    // disregard a harness banner that disagrees* used to ride this line and now
    // rides the spine's `SKILL.md`: every normative consequence of a value is
    // the skill's, and `tests/prompt.rs` holds both ends of that.
    assert!(
        mandate.contains(&format!(
            "Version control: {JJ_CLAUSE}`{}`).\n",
            root.display()
        )),
        "the version-control line must be exactly the value, ending the sentence \
         where the value ends: {mandate:?}"
    );
}

// The native shape is the one the session cannot get right on its own: a harness
// banner computed from `.git` alone reads a native jj workspace as no repository
// at all (claude-code#41435), which is exactly what this line overrides.
#[test]
fn the_mandate_states_a_native_jj_workspace_and_its_root() {
    assert_the_mandate_states_the_resolved_vcs(Shape::Native);
}

#[test]
fn the_mandate_states_a_colocated_jj_workspace_and_its_root() {
    assert_the_mandate_states_the_resolved_vcs(Shape::Colocated);
}

// A `done` signal — the finish cycle's last teardown action — must end the loop
// exactly once, cleanly, and must not be confused with either a relaunch or the
// no-signal stop. The abandoned-channel housekeeping a replacement driver does
// on acquiring the lease rides along here, because its failure modes have to
// stay advisory: a channel it cannot clean is reported and stepped over, never
// a reason not to launch.
#[test]
fn a_done_signal_finishes_the_loop_once_and_housekeeping_stays_advisory() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");

    let control_dir = worktree.join(".jj/grove");
    fs::create_dir_all(&control_dir).unwrap();
    let abandoned_signal = control_dir.join("signal-00000000000000000000000000000000");
    let blocked_signal = control_dir.join("signal-11111111111111111111111111111111");
    let unrelated_control = control_dir.join("signal-not-a-grove-channel");
    fs::write(&abandoned_signal, "old completion\n").unwrap();
    fs::create_dir(&blocked_signal).unwrap();
    fs::write(&unrelated_control, "keep me\n").unwrap();

    let log = fixture.path().join("log");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\nprintf 'ran\\n' >> {log}\n: > \"$GROVE_LAUNCH_DIR/teardown\"; ravel end-session grove-continue\nexit 0\n",
            log = shell_quote(&log),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    let output = run_driver(&worktree, &home);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "abandoned-channel housekeeping must not prevent launch: {stderr}"
    );
    assert!(
        stderr.contains("could not clean every signal channel abandoned by a previous driver"),
        "an abandoned-channel cleanup failure must remain visible: {stderr}"
    );
    assert!(
        stderr.contains("grove finished — loop complete"),
        "a `done` signal must end the loop with a clean finish: {stderr}"
    );

    let log = fs::read_to_string(&log).unwrap();
    assert_eq!(
        log.lines().filter(|line| !line.is_empty()).count(),
        1,
        "the loop must run exactly once then finish — no relaunch (log: {log:?})"
    );
    assert!(
        !abandoned_signal.exists(),
        "a replacement driver must clean abandoned signal channels after acquiring the lease"
    );
    assert!(
        blocked_signal.is_dir(),
        "an unremovable abandoned channel must remain inert while the loop continues"
    );
    assert_eq!(
        fs::read_to_string(unrelated_control).unwrap(),
        "keep me\n",
        "cleanup must ignore names outside Grove's exact channel grammar"
    );
}

// Outcome precedence: the signal the session actually wrote decides the loop,
// and a later failure to tidy that channel away cannot retroactively change it.
// The session's own disposition is the authoritative fact; channel removal is
// housekeeping, and housekeeping reports rather than overrules.
#[test]
fn a_signal_removal_failure_does_not_override_a_done_disposition() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");
    let control_dir = worktree.join(".jj/grove");

    let signal_log = fixture.path().join("signal-log");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\n: > \"$GROVE_LAUNCH_DIR/teardown\"; ravel end-session grove-continue\nprintf '%s\\n' \"$GROVE_LAUNCH_DIR/teardown\" > {log}\nchmod 0500 \"$GROVE_LAUNCH_DIR\"\nexit 0\n",
            log = shell_quote(&signal_log),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    let output = run_driver(&worktree, &home);

    // Restore before asserting, so a failing assertion cannot leave the fixture
    // undeletable.
    let mut permissions = fs::metadata(&control_dir).unwrap().permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&control_dir, permissions).unwrap();
    let signal_path = PathBuf::from(fs::read_to_string(signal_log).unwrap().trim());
    fs::set_permissions(
        signal_path.parent().unwrap(),
        fs::Permissions::from_mode(0o700),
    )
    .unwrap();
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "channel housekeeping must not make a clean finish fail: {stderr}"
    );
    assert!(
        stderr.contains("could not remove the interpreted foreground-session launch directory"),
        "the removal failure must remain visible: {stderr}"
    );
    assert!(
        stderr.contains("grove finished — loop complete"),
        "the interpreted done disposition must remain authoritative: {stderr}"
    );
    assert!(
        signal_path.exists(),
        "the fixture must force the removal failure whose outcome precedence this test exercises"
    );
}

// Signal-file identity (signal-file-identity-k6): two loops with the *same
// grove name* in *different worktrees* must not interfere, even running truly
// concurrently. The pre-fix path was derived from the name alone
// (`$TMPDIR/grove-loop-<name>.signal`), so two worktrees sharing a basename —
// generic names like "bugs"/"plan"/"docs" are the norm — collided on one file.
//
// The "attacker" signals `done` immediately then hangs; the "victim" never
// signals and outlives the attacker's whole kill sequence before exiting on its
// own. Pre-fix, the attacker's `done` would land in the file the victim's
// watcher was also polling, killing the victim early and reporting a phantom
// clean finish from content it never wrote.
#[test]
fn concurrent_loops_with_the_same_grove_name_in_different_worktrees_do_not_interfere() {
    let fixture = TempDir::new().unwrap();

    let setup = |role: &str, body: &str| {
        let home = fixture.path().join(format!("{role}-home"));
        // Same *basename* in both trees — that is the whole point.
        let worktree = fixture.path().join(role).join("samegrove");
        init_worktree(&worktree);
        plant_tree(&worktree, "01-impl--subject-k1.md");
        let configured = fixture.path().join(format!("{role}-command.sh"));
        write_exec(&configured, body);
        support::route_every_kind_to(&home, &configured);
        (home, worktree)
    };

    let (attacker_home, attacker_tree) = setup(
        "attacker",
        "#!/bin/sh\n: > \"$GROVE_LAUNCH_DIR/teardown\"; ravel end-session grove-continue\nexec sleep 30\n",
    );
    let (victim_home, victim_tree) = setup("victim", "#!/bin/sh\nsleep 1.5\nexit 0\n");

    let started = Instant::now();
    let mut attacker = DriverProcess::spawn(&attacker_tree, &attacker_home);
    let mut victim = DriverProcess::spawn(&victim_tree, &victim_home);

    let attacker_out = attacker.finish();
    let victim_out = victim.finish();
    let elapsed = started.elapsed();

    let attacker_stderr = String::from_utf8_lossy(&attacker_out.stderr);
    let victim_stderr = String::from_utf8_lossy(&victim_out.stderr);
    assert!(
        attacker_stderr.contains("grove finished — loop complete"),
        "sanity check: the attacker's own `done` signal still ends its own loop: {attacker_stderr}"
    );
    assert!(
        victim_stderr.contains("without a supported Grove result"),
        "the victim's session ended without ever signalling anything of its own — a \
         foreign `done` from the other worktree's loop must not be mistaken for its \
         own completion signal: {victim_stderr}"
    );
    assert!(
        elapsed >= Duration::from_millis(1200),
        "the victim must run its full ~1.5s session, not be cut short by the \
         attacker's kill sequence (elapsed: {elapsed:?})"
    );
}

// The real binary installs the SIGTERM/SIGHUP handler that forwards termination
// to a live child and reaps it through the ordinary watcher escalation. A
// driver that merely died would orphan an interactive session onto the TTY.
//
// **And it reports the killing to whoever started it.** Every other way this
// loop ends is a decision it was designed to reach and exits cleanly; this one
// is the loop being taken away mid-grove, and a systemd unit, a `timeout(1)` or
// a shell `wait` can only tell the two apart through the wait status. So the
// driver dies of the same signal after its cleanup rather than exiting 0.
#[test]
fn a_sigtermed_driver_stops_and_reaps_its_child() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");

    // Never signals, never exits on its own — a session sitting mid-task when
    // the driver is killed from outside. `exec` so the pid the driver signals
    // is the sleeping process itself.
    let launched = fixture.path().join("launched");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\n: > {launched}\nexec sleep 60\n",
            launched = shell_quote(&launched),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    // Streams to files rather than `Stdio::null()`, through `DriverProcess`: a
    // driver that stops before its session ever starts is the failure the wait
    // below has to report, and nulling threw away the only account of why.
    let mut driver = DriverProcess::spawn(&worktree, &home);
    let diagnostics = driver.diagnostics().to_path_buf();

    // Wait for the child marker so SIGTERM lands mid-session rather than racing
    // the driver's own startup. Conditioned on the driver's own liveness, not a
    // fixed budget: start-up cost is not fixed, and a driver that has ended can
    // never write the marker (loop-driver-readiness-deadline-k170).
    driver.wait_for_ready(&launched);

    unsafe { libc::kill(driver.id(), libc::SIGTERM) };

    let deadline = Instant::now() + Duration::from_secs(20);
    let status = loop {
        if let Some(status) = driver.try_wait() {
            break status;
        }
        assert!(
            Instant::now() < deadline,
            "a SIGTERM'd driver must stop, not hang"
        );
        thread::sleep(Duration::from_millis(50));
    };

    use std::os::unix::process::ExitStatusExt as _;
    assert_eq!(
        status.signal(),
        Some(libc::SIGTERM),
        "a SIGTERM'd driver must die of the signal, so its own parent reads \
         128+N rather than a clean finish: {status:?}"
    );
    assert_eq!(
        status.code(),
        None,
        "there is no exit code that means \"killed\", which is why the signal \
         is re-raised rather than mapped to one"
    );
    assert!(
        !diagnostics_of(&diagnostics).is_empty(),
        "the interruption must still be reported in the driver's own output"
    );
    assert!(
        diagnostics_of(&diagnostics).contains("interrupted by signal 15"),
        "the diagnostic must name the signal that ended the loop: {}",
        diagnostics_of(&diagnostics)
    );
}

fn diagnostics_of(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_default()
}

// The escalation signals the session's **process group**, so a command the
// session spawned — a tool subprocess, a language server, an agent's own
// in-flight `grove-llm` — is reaped with it rather than surviving the SIGKILL
// and staying attached to the terminal.
//
// The expensive failure this closes is not untidiness. A surviving `grove-llm`
// holds *shared* epoch admission for the whole of its operation, so the
// driver's post-reap `invalidate_session_epoch` waits out its full 30s bound
// and then turns a session that finished correctly into a fatal error with its
// token discarded uninterpreted. Here the session signals `done`, so the loop
// must reach a clean finish inside the escalation's own timescale instead.
#[test]
fn the_escalation_reaps_the_sessions_descendants() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");

    let descendant_pid = fixture.path().join("descendant-pid");
    let configured = fixture.path().join("configured-command.sh");
    // Signals and then declines to end — an interactive session returning to
    // its prompt — so the launch reaches the full escalation, which is what
    // this test is about.
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\n\
             sh -c 'while : ; do sleep 0.05 ; done' &\n\
             printf '%s\\n' \"$!\" > {pid}\n\
             : > \"$GROVE_LAUNCH_DIR/teardown\"; ravel end-session grove-continue\n\
             while : ; do sleep 0.05 ; done\n",
            pid = shell_quote(&descendant_pid),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    // The cross-check: the same shape of process, in *this* process's group
    // rather than the session's. A probe that reported every pid gone would
    // read identically without it.
    let mut bystander = Reaped(
        Command::new("sh")
            .arg("-c")
            .arg("while : ; do sleep 0.05 ; done")
            .spawn()
            .unwrap(),
    );

    let output = run_driver(&worktree, &home);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("grove finished — loop complete"),
        "the `done` signal must survive the escalation and finish the loop: {stderr}"
    );

    let descendant: i32 = fs::read_to_string(&descendant_pid)
        .expect("the session never reported its descendant")
        .trim()
        .parse()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let reaped = loop {
        let alive = unsafe { libc::kill(descendant, 0) } == 0;
        if !alive {
            break true;
        }
        if Instant::now() >= deadline {
            break false;
        }
        thread::sleep(Duration::from_millis(50));
    };
    if !reaped {
        // The assertion below is about to fail, and a failing assertion must
        // not also leave the process it is complaining about running: this is
        // the `sh -c 'while : ; do sleep 0.05 ; done'` shape found reparented to
        // pid 1 days later (driver-test-timeout-path-unbounded-k194).
        // SAFETY: `kill(2)` on a pid the fixture reported.
        unsafe { libc::kill(descendant, libc::SIGKILL) };
    }
    assert!(
        reaped,
        "a command the session spawned outlived the escalation that killed it"
    );
    assert!(
        bystander.0.try_wait().unwrap().is_none(),
        "the escalation reached a process outside the session's own group"
    );
}

/// A child that is killed and reaped when it goes out of scope, however the
/// scope ends. A bare `Child` whose teardown is the last two statements of a
/// test is torn down only when every assertion before it passed
/// (driver-test-timeout-path-unbounded-k194).
struct Reaped(Child);

impl Drop for Reaped {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

// A leaf whose filename is in no grammar this binary reads is a tree the driver
// cannot select from, so it refuses before launching anything rather than
// guessing a kind. The refusal names the offending file, because
// the fix is an edit to that filename.
#[test]
fn an_unrecognised_filename_kind_refuses_to_launch() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-reserch-a-k1.md");
    jj(
        &worktree,
        &["commit", "-m", "tree with an unrecognised kind"],
    );

    let log = fixture.path().join("log");
    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            "#!/bin/sh\nprintf 'ran\\n' >> {log}\nexit 0\n",
            log = shell_quote(&log)
        ),
    );
    support::route_every_kind_to(&home, &configured);

    let output = run_driver(&worktree, &home);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(
        stderr.contains("01-reserch-a-k1.md"),
        "the refusal must name the offending filename: {stderr}"
    );
    assert!(
        !log.exists(),
        "nothing may be launched for a leaf whose kind cannot be resolved"
    );
}

// An orphaned agent command that was admitted under the live epoch, then blocked
// on the tree lock, holds shared epoch admission after its foreground parent is
// gone. The driver must not park the loop waiting for it: its post-reap
// invalidation is bounded, and it stops with a visible diagnostic while leaving
// the completion signal deliberately unconsumed — a relaunch on a tree some
// orphan is still mutating is the outcome the bound exists to prevent.
//
// **The orphan is spawned by this test, not by the session**, and that is the
// whole difference between an orphan and a descendant. Since
// `child-signal-disposition-k31` the escalation signals the session's process
// *group*, so a command the session itself launched is reaped with it and the
// contention below never arises — which is the point of that change, and is
// asserted next door in `the_escalation_reaps_the_sessions_descendants`. What
// survives a group kill is a process that was never in the group: one launched
// from a different session, a different terminal, or by a human. That is the
// process this fixture stands in for, and it is why the bounded timeout is
// still load-bearing.
//
// Slow by construction (~40s): the fixed invalidation timeout is the subject.
#[test]
fn an_orphaned_epoch_guard_stops_before_consuming_the_relaunch_signal() {
    let fixture = TempDir::new().unwrap();
    let home = fixture.path().join("home");
    let worktree = fixture.path().join("worktree");
    init_worktree(&worktree);
    plant_tree(&worktree, "01-impl--subject-k1.md");

    let launch_ready = fixture.path().join("launch-ready");
    let epoch_held = fixture.path().join("epoch-held");
    let orphan_done = fixture.path().join("orphan-done");
    let orphan_stderr = fixture.path().join("orphan-stderr");
    let term_received = fixture.path().join("term-received");
    let session_pid = fixture.path().join("session-pid");
    let observed_signal = fixture.path().join("observed-signal");
    let launch_count = fixture.path().join("launch-count");

    let configured = fixture.path().join("configured-command.sh");
    write_exec(
        &configured,
        &format!(
            r#"#!/bin/sh
n=$(cat {count} 2>/dev/null || echo 0)
n=$((n + 1))
printf '%s\n' "$n" > {count}
printf '%s\n' "$GROVE_LAUNCH_DIR/unconsumed" > {observed}
printf '%s\n' "$$" > {pid}
: > {ready}
while [ ! -e {held} ]; do sleep 0.01; done
ravel end-session grove-continue
trap ': > {term}' TERM
while :; do sleep 0.1; done
"#,
            count = shell_quote(&launch_count),
            observed = shell_quote(&observed_signal),
            pid = shell_quote(&session_pid),
            ready = shell_quote(&launch_ready),
            held = shell_quote(&epoch_held),
            term = shell_quote(&term_received),
        ),
    );
    support::route_every_kind_to(&home, &configured);

    let (release_tx, release_rx) = mpsc::channel();
    let lock_worktree = worktree.clone();
    let lock_launch_ready = launch_ready.clone();
    let lock_observed_signal = observed_signal.clone();
    let lock_orphan_stderr = orphan_stderr.clone();
    let lock_orphan_done = orphan_done.clone();
    let lock_epoch_held = epoch_held.clone();
    let lock_thread = thread::spawn(move || {
        use std::os::fd::AsRawFd;
        let deadline = Instant::now() + Duration::from_secs(20);
        while !lock_launch_ready.exists() {
            assert!(
                Instant::now() < deadline,
                "configured session did not start"
            );
            thread::sleep(Duration::from_millis(10));
        }
        // The channel path the driver published to *this* session, which is
        // what makes the command below admissible under the live epoch. Read
        // from the session's own report rather than reconstructed, because the
        // suffix is fresh OS randomness.
        let signal_path = loop {
            assert!(
                Instant::now() < deadline,
                "the session never reported its signal path"
            );
            match fs::read_to_string(&lock_observed_signal) {
                Ok(reported) if !reported.trim().is_empty() => break reported.trim().to_string(),
                _ => thread::sleep(Duration::from_millis(10)),
            }
        };

        let tree_guard = fs::File::open(&lock_worktree).unwrap();
        assert_eq!(
            unsafe { libc::flock(tree_guard.as_raw_fd(), libc::LOCK_EX) },
            0,
            "locking the worktree failed"
        );

        // Spawned **here**, in the test process's own process group, so the
        // driver's escalation cannot reach it however wide it signals. A
        // command the session launched would be a descendant and would be
        // reaped with it.
        let mut orphan = Command::new(support::grove_llm())
            .arg("pick")
            .current_dir(&lock_worktree)
            .env(
                "GROVE_LAUNCH_DIR",
                Path::new(&signal_path).parent().unwrap(),
            )
            .stdout(Stdio::null())
            .stderr(Stdio::from(fs::File::create(&lock_orphan_stderr).unwrap()))
            .spawn()
            .unwrap();

        let epoch_path = lock_worktree.join(".jj/grove/session.epoch");
        loop {
            assert!(
                Instant::now() < deadline,
                "orphaned tree command never acquired shared epoch admission"
            );
            let probe = fs::File::open(&epoch_path).unwrap();
            let result = unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) };
            if result != 0 {
                let error = std::io::Error::last_os_error();
                assert!(
                    matches!(
                        error.raw_os_error(),
                        Some(code) if code == libc::EWOULDBLOCK || code == libc::EAGAIN
                    ),
                    "probing shared epoch admission failed unexpectedly: {error}"
                );
                break;
            }
            assert_eq!(
                unsafe { libc::flock(probe.as_raw_fd(), libc::LOCK_UN) },
                0,
                "releasing the epoch probe failed"
            );
            thread::sleep(Duration::from_millis(10));
        }
        fs::write(&lock_epoch_held, "").unwrap();
        release_rx.recv().unwrap();
        drop(tree_guard);
        orphan.wait().unwrap();
        fs::write(&lock_orphan_done, "").unwrap();
    });

    let started = Instant::now();
    let mut driver = DriverProcess::spawn(&worktree, &home);

    let setup_deadline = Instant::now() + Duration::from_secs(25);
    while !epoch_held.exists() && Instant::now() < setup_deadline {
        if driver.try_wait().is_some() {
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    if !epoch_held.exists() {
        // This is the branch that hung. It killed the driver and then read its
        // *piped* stderr to EOF — which the session the driver had already
        // launched was still holding open, so the read never returned
        // (driver-test-timeout-path-unbounded-k194). The kill still comes
        // first, so the account is as complete as the driver ever made it; what
        // changed is that it is read back from a file, where there are no
        // writers to wait for.
        driver.kill();
        let said = fs::read_to_string(driver.diagnostics()).unwrap_or_default();
        let _ = release_tx.send(());
        let _ = lock_thread.join();
        panic!("orphan contention fixture did not reach shared epoch admission: {said}");
    }

    let contention_observed = Instant::now();
    // The built-in grace (2s) plus kill-grace (5s) precede the fixed 30s
    // invalidation timeout, so the process-level bound is the timeout plus that
    // escalation plus slack — not the timeout alone.
    let stop_deadline = contention_observed + Duration::from_secs(60);
    let stopped_within_bound = loop {
        if driver.try_wait().is_some() {
            break true;
        }
        if Instant::now() >= stop_deadline {
            // Takes the session's process group with it, so the diagnostic this
            // branch is collecting is not also a leaked `configured-command.sh`.
            driver.kill();
            break false;
        }
        thread::sleep(Duration::from_millis(25));
    };
    let output = driver.finish();
    let total_elapsed = started.elapsed();
    let contention_elapsed = contention_observed.elapsed();
    let orphan_outlived_killed_parent = !orphan_done.exists();
    let signal_path = fs::read_to_string(&observed_signal)
        .ok()
        .map(|path| PathBuf::from(path.trim()));
    let signal_was_left_unconsumed = signal_path
        .as_ref()
        .is_some_and(|path| path.parent().unwrap().exists());

    let _ = release_tx.send(());
    lock_thread.join().unwrap();
    let orphan_deadline = Instant::now() + Duration::from_secs(10);
    while !orphan_done.exists() {
        assert!(
            Instant::now() < orphan_deadline,
            "orphaned tree command did not finish after the tree lock was released: {}",
            fs::read_to_string(&orphan_stderr).unwrap_or_default()
        );
        thread::sleep(Duration::from_millis(10));
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stopped_within_bound,
        "driver exceeded its watchdog after contention began: {stderr}"
    );
    assert!(
        !output.status.success(),
        "epoch handoff must fail: {stderr}"
    );
    assert!(
        stderr.contains(
            "post-reap session epoch invalidation blocked; launch ending left unconsumed"
        ),
        "unexpected error: {stderr}"
    );
    assert!(
        stderr.contains("timed out after 30s waiting for exclusive session epoch lock"),
        "unexpected error: {stderr}"
    );
    let contention_report = "waiting for exclusive session epoch lock for post-reap invalidation";
    assert_eq!(
        stderr
            .lines()
            .filter(|line| *line == contention_report)
            .count(),
        1,
        "real contention must be reported exactly once: {stderr}"
    );
    assert!(
        total_elapsed >= Duration::from_secs(30),
        "the fixed timeout fired too early: {total_elapsed:?}"
    );
    assert!(
        contention_elapsed < Duration::from_secs(60),
        "the fixed 30s timeout exceeded its process-level bound: {contention_elapsed:?}"
    );
    assert!(
        term_received.exists(),
        "the foreground parent must receive SIGTERM before forced death"
    );
    assert!(
        orphan_outlived_killed_parent,
        "the admitted background command must still be blocked after its foreground parent died"
    );
    let session_pid: i32 = fs::read_to_string(session_pid)
        .unwrap()
        .trim()
        .parse()
        .unwrap();
    assert_eq!(unsafe { libc::kill(session_pid, 0) }, -1);
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ESRCH),
        "the TERM-ignoring foreground parent must have been SIGKILLed"
    );
    assert!(signal_was_left_unconsumed);
    assert_eq!(fs::read_to_string(&launch_count).unwrap().trim(), "1");
}

// ---------------------------------------------------------------------------
// The launch boundary
//
// Grove launches lifecycle sessions through ravel start-session, using the
// once-selected independent installation. A temporary personal Ravel policy
// selects a fake agent; there is no Ravel record store. These cases observe the
// request, authority separation and the public result at Grove's boundary.

/// The independently installed Ravel front and its compiled worker.
fn ravel() -> PathBuf {
    support::installed_ravel()
}

/// A fixture for the launch boundary: a HOME, a jj working tree, a fake
/// harness, and the places the harness and the policy leave their evidence.
struct Dispatch {
    _fixture: TempDir,
    /// The fixture's own directory, for evidence a case adds.
    root: PathBuf,
    home: PathBuf,
    worktree: PathBuf,
    /// The fake harness: the program a dispatch policy's `select` returns.
    harness: PathBuf,
    /// One numbered directory per harness start, in launch order.
    launches: PathBuf,
    /// Where a policy probe writes what it observed.
    view: PathBuf,
}

/// What the fake harness observed at one start.
struct Launch {
    args: Vec<OsString>,
    /// `RAVEL_EXIT_FILE`, or `<unset>`.
    channel: String,
    launch_dir: String,
    /// The driver's session epoch, as it stood while the harness ran.
    epoch: String,
}

impl Dispatch {
    /// A fixture whose working tree is named `worktree`. Its fake harness
    /// ends each session with `done` until [`Dispatch::harness_then`] says
    /// otherwise.
    fn new(worktree: &str) -> Self {
        let fixture = TempDir::new().unwrap();
        let root = fixture.path().to_path_buf();
        let worktree = root.join(worktree);
        init_worktree(&worktree);
        let launches = root.join("launches");
        fs::create_dir(&launches).unwrap();
        let dispatch = Dispatch {
            home: root.join("home"),
            harness: root.join("fake-harness"),
            view: root.join("view.json"),
            _fixture: fixture,
            root,
            worktree,
            launches,
        };
        dispatch.harness_then(&complete_done());
        dispatch
    }

    /// Rewrite the fake harness so that it records how it was started and
    /// then runs `then`, a shell fragment in which `$n` is this start's
    /// number, counted from 0.
    fn harness_then(&self, then: &str) {
        write_exec(
            &self.harness,
            &format!(
                "#!/bin/sh\n\
                 n=0\n\
                 until mkdir {launches}/$n 2>/dev/null; do\n\
                 n=$((n + 1)); [ $n -lt 10 ] || exit 90\n\
                 done\n\
                 record={launches}/$n\n\
                 {RECORD_START}\
                 {then}\n",
                launches = shell_quote(&self.launches),
            ),
        );
    }

    /// Replace the personal dispatch policy, at the default entry path.
    fn policy(&self, source: &str) {
        support::write_policy(&self.home, source);
    }

    /// Replace the owner settings, beside the policy. Grove passes dispatch no
    /// bound and no grant, so a case that needs either sets it as an owner does.
    fn settings(&self, json: &str) {
        let file = self.home.join(".config/ravel/settings.json");
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, json).unwrap();
    }

    /// Run the loop until a session signals `done`, returning what the driver
    /// said.
    fn drive_to_completion(&self) -> String {
        let output = run_driver(&self.worktree, &self.home);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(output.status.success(), "{stderr}");
        assert!(
            stderr.contains("grove finished — loop complete"),
            "the loop did not end on its session's `done`: {stderr}"
        );
        stderr
    }

    fn launch_count(&self) -> usize {
        fs::read_dir(&self.launches).unwrap().count()
    }

    fn launch(&self, n: usize) -> Launch {
        let record = self.launches.join(n.to_string());
        let bytes = fs::read(record.join("args")).expect("the fake harness started");
        let mut args: Vec<OsString> = bytes
            .split(|byte| *byte == 0)
            .map(|word| OsString::from_vec(word.to_vec()))
            .collect();
        assert_eq!(args.pop(), Some(OsString::new()), "NUL-terminated");
        Launch {
            args,
            channel: fs::read_to_string(record.join("channel")).unwrap(),
            launch_dir: fs::read_to_string(record.join("launch-dir")).unwrap(),
            epoch: fs::read_to_string(record.join("epoch")).unwrap(),
        }
    }

    /// What the policy probe last wrote, consumed so that a later launch
    /// cannot be read through an earlier one's view.
    fn view(&self) -> serde_json::Value {
        let view = fs::read_to_string(&self.view).expect("the policy probe ran");
        fs::remove_file(&self.view).unwrap();
        serde_json::from_str(&view).unwrap()
    }

    /// The selected leaf's task file as the driver names it: under the
    /// canonical working-tree root.
    fn task_file(&self, leaf: &str) -> PathBuf {
        self.worktree
            .canonicalize()
            .unwrap()
            .join(".grove")
            .join(leaf)
    }
}

/// How a fake harness records its start in `$record`: its arguments, its run
/// identity, its channel, and the session epoch as it stood, which is read
/// from the working tree.
const RECORD_START: &str = "\
    for argument in \"$@\"; do printf '%s\\0' \"$argument\"; done > \"$record/args\"\n\
    printf '%s' \"${RAVEL_EXIT_FILE-<unset>}\" > \"$record/channel\"\n\
    printf '%s' \"$GROVE_LAUNCH_DIR\" > \"$record/launch-dir\"\n\
    cp .jj/grove/session.epoch \"$record/epoch\"\n";

/// Mark teardown directly in this fixture so cases can inspect the retained
/// task tree, then publish the real Ravel result. The teardown-verb case
/// below exercises the real finish-commit/record-teardown precondition.
fn complete_done() -> String {
    format!(
        "touch \"$GROVE_LAUNCH_DIR/teardown\"; exec {} end-session grove-continue",
        shell_quote(&ravel())
    )
}

/// A dispatch policy whose `select` returns the fake harness for each of
/// `kinds`, and refuses any other kind as `incomplete_mapping`, its own code.
/// The harness's arguments are what `select` received: the kind, the task
/// file, the task identity and the prompt, then every parameter as one JSON
/// object, then the caller's directory.
///
/// At import it writes `view` with the names in the worker's environment and
/// in the environment of a child it spawns, and the worker's value of
/// `RAVEL_EXIT_FILE`, so that what selection code can reach is observed
/// where it runs.
fn probing_policy(view: &Path, harness: &Path, kinds: &[&str]) -> String {
    let kinds = kinds
        .iter()
        .map(|kind| format!("{kind:?}"))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
import {{ execFileSync }} from "node:child_process";
const names = (lines: string) => lines.split("\n").filter(Boolean).map((line) => line.slice(0, line.indexOf("="))).sort();
writeFileSync({view:?}, JSON.stringify({{
  worker: Object.keys(process.env).sort(),
  child: names(execFileSync("/usr/bin/env", [], {{ encoding: "utf8" }})),
  channel: process.env.GROVE_LAUNCH_DIR ?? null,
  launchDir: process.env.GROVE_LAUNCH_DIR ?? null,
}}));
export const policy = {{
  schemaVersion: 2,
  version: "grove-seam-1",
  select(request) {{
    if (![{kinds}].includes(request.kind)) {{
      return {{
        status: "refused",
        code: "incomplete_mapping",
        message: `this policy names no command for kind ${{JSON.stringify(request.kind)}}`,
        remedy: "add the kind to the policy",
      }};
    }}
    return {{
      status: "selected",
      program: {harness:?},
      args: [request.kind, request.taskFile, request.taskId, request.prompt, JSON.stringify(request.params), request.cwd],
      provider: "origin-a",
      model: "model-a",
      effort: "high",
      reason: `kind ${{request.kind}} runs the fake harness`,
    }};
  }},
}};
"#,
        view = view.to_str().unwrap(),
        harness = harness.to_str().unwrap(),
    )
}

/// Whether a session epoch names `channel` as its live signal path.
fn epoch_names(epoch: &str, channel: &str) -> bool {
    let hex: String = channel
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    epoch.starts_with("state=active\n") && epoch.contains(&format!("launch-dir-hex={hex}\n"))
}

// A lifecycle session's policy receives the driver's own selection as native
// data, from the working-tree root: the kind, the absolute task path, the
// handle and the mandate byte for byte, and no parameter at all. The policy
// places each in the fake agent's arguments whole, where this test observes them.
// No Grove configuration file exists anywhere. The working tree is a secondary
// workspace, so its root is not the main repository's, and its name puts
// spaces, quotes and shell punctuation into the task path and into the prompt,
// which states the root and runs over many lines.
//
// Only the final harness holds the authority to end the session. It receives
// the channel the driver's live epoch names, and completes through it. The
// policy worker, and a child it spawns, see no Grove variable at all. The
// control grants the channel to the worker in the owner settings, which no
// owner should do, and the same probe then reads it: the absence is the
// scrubbing, not a probe that cannot see.
#[test]
fn a_session_s_task_reaches_select_as_native_data_and_only_its_harness_holds_the_channel() {
    let mut dispatch = Dispatch::new("main repository");
    let repository = dispatch.worktree.canonicalize().unwrap();
    let name = "work tree 'single' \"double\" $(touch x) `y`; a|b & *";
    let secondary = dispatch.root.join(name);
    jj(
        &dispatch.worktree,
        &[
            "workspace",
            "add",
            "--quiet",
            "--name",
            "secondary",
            secondary.to_str().unwrap(),
        ],
    );
    dispatch.worktree = secondary;
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["impl"],
    ));

    dispatch.drive_to_completion();
    assert!(
        !dispatch.home.join(".config/grove").exists(),
        "the launch needed no Grove configuration"
    );
    let launch = dispatch.launch(0);
    let root = dispatch.worktree.canonicalize().unwrap();
    assert_ne!(root, repository);
    let task_file = dispatch.task_file("01-impl--subject-k1.md");
    assert!(task_file.is_absolute() && task_file.is_file());
    let mandate = grove_loop::compose(&grove_loop::Mandate {
        handle: &grove_loop::Handle::parse("subject-k1").unwrap(),
        kind: &grove_loop::Kind::new("impl").unwrap(),
        workspace: &grove_loop::Workspace::resolve(&root).unwrap(),
        version: grove_loop::VERSION,
        ending_command: &grove_loop::session_command::end_session(&ravel()).unwrap(),
        force_stop_command: &grove_loop::session_command::force_stop(&ravel()).unwrap(),
    });
    assert!(
        mandate.contains(&format!("{MANDATED_LEAF}subject-k1`"))
            && mandate.contains(root.to_str().unwrap())
            && mandate.lines().count() > 1,
        "the mandate carries the awkward root over many lines: {mandate}"
    );
    let params = serde_json::json!({});
    let [kind, file, id, prompt, received, cwd] = launch.args.as_slice() else {
        panic!("the harness's arguments: {:?}", launch.args);
    };
    assert_eq!(
        [kind, file, id],
        [
            &OsString::from("impl"),
            &task_file.clone().into_os_string(),
            &OsString::from("subject-k1")
        ]
    );
    assert_eq!(
        prompt.to_str().unwrap(),
        mandate,
        "the prompt must be the mandate, unchanged"
    );
    let received: serde_json::Value = serde_json::from_str(received.to_str().unwrap()).unwrap();
    assert_eq!(received, params, "no parameter reaches select");
    assert_eq!(
        cwd,
        root.as_os_str(),
        "dispatch runs in the working-tree root, not the main repository's"
    );

    assert!(
        epoch_names(&launch.epoch, &launch.launch_dir),
        "the harness's channel {:?} is not the one the live epoch names: {:?}",
        launch.channel,
        launch.epoch
    );

    let view = dispatch.view();
    assert!(
        view["launchDir"].is_null(),
        "worker received launch authority: {view}"
    );
    for place in ["worker", "child"] {
        let names: Vec<&str> = view[place]
            .as_array()
            .unwrap()
            .iter()
            .map(|name| name.as_str().unwrap())
            .collect();
        assert!(names.contains(&"HOME"), "the probe read nothing: {view}");
        assert!(
            !names.iter().any(|name| name.starts_with("GROVE_")),
            "the policy's {place} holds a Grove variable: {view}"
        );
    }
    assert_eq!(view["channel"], serde_json::Value::Null, "{view}");

    // The control: granted, the channel reaches the same probe, and it is the
    // harness's own, fresh for that launch.
    dispatch.settings(r#"{ "policyEnv": ["GROVE_LAUNCH_DIR"] }"#);
    dispatch.drive_to_completion();
    let granted = dispatch.launch(1);
    assert_eq!(dispatch.view()["channel"], granted.launch_dir.as_str());
    assert_ne!(
        granted.channel, launch.channel,
        "each launch gets a fresh channel"
    );
}

// A kind the policy does not route is caught when its leaf launches, and
// nowhere earlier. A session authors a `design` leaf although the policy names
// no command for that kind: the tree verb consults no policy. The launch then
// refuses with the policy's own refusal, its code and remedy beside dispatch's
// stable one, and the `inspect` invocation that reproduces it. No harness
// starts, Grove reports the kind, the handle and the status and stops, and the
// leaf stays live. Once the owner adds the kind, the next run launches that
// same leaf.
#[test]
fn a_kind_the_policy_refuses_leaves_its_leaf_live_and_launches_once_the_policy_routes_it() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let grove_llm = shell_quote(&own_grove_llm());
    let authored = dispatch.root.join("authored");
    dispatch.harness_then(&format!(
        "if [ $n = 0 ]; then\n\
         {grove_llm} leaf-add . follow-up --kind design > {authored} 2>&1 || exit 91\n\
         {grove_llm} leaf-retire .grove/01-impl--subject-k1.md > /dev/null 2>&1 || exit 92\n\
         exec {exit_dispatch} end-session grove-continue\n\
         fi\n\
         {complete}",
        authored = shell_quote(&authored),
        exit_dispatch = shell_quote(&ravel()),
        complete = complete_done(),
    ));
    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["impl"],
    ));

    let output = run_driver(&dispatch.worktree, &dispatch.home);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    let leaf = dispatch.task_file("02-design--follow-up-k2.md");
    assert_eq!(
        fs::read_to_string(&authored).unwrap(),
        format!("{}\n", leaf.display()),
        "authoring a leaf of a kind the policy refuses must succeed"
    );
    assert!(dispatch.task_file("01-DONE-impl--subject-k1.md").is_file());
    assert_eq!(
        dispatch.launch_count(),
        1,
        "the refused launch must reach no harness: {stderr}"
    );
    for said in [
        "refused (policy_refused, stage selection)",
        "  policy code: incomplete_mapping",
        "  remedy: add the kind to the policy",
        "session ended without a supported Grove result — status exit status: 3",
        "session kind `design` for `follow-up-k2` failed",
        "rerun `grove` to continue",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }

    // Reproduce through the same public request; Ravel has no inspect verb.
    let reproduced = Command::new(ravel())
        .args([
            "start-session",
            "--kind=design",
            "--task-id=follow-up-k2",
            "--prompt=control",
        ])
        .arg(format!("--task-file={}", leaf.display()))
        .env_clear()
        .env("HOME", &dispatch.home)
        .env("PATH", support::ravel_path())
        .output()
        .unwrap();
    assert_eq!(reproduced.status.code(), Some(3), "{reproduced:?}");
    assert!(String::from_utf8_lossy(&reproduced.stderr).contains("incomplete_mapping"));
    assert_eq!(dispatch.launch_count(), 1);
    assert!(leaf.is_file());

    // Still live: add the kind, and the next run launches that leaf.
    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["impl", "design"],
    ));
    dispatch.drive_to_completion();
    assert_eq!(
        dispatch.launch(1).args[..3],
        [
            OsString::from("design"),
            leaf.into_os_string(),
            OsString::from("follow-up-k2"),
        ]
    );
}

// A `config.kdl` and a `.grove.kdl` left on disk are never read and refuse
// nothing. Each state below changed the launch, or refused it, while Grove
// launched from those files: a valid pair whose delta selects the personal
// file's other command, an invalid pair, and a tracked delta. The harness now
// receives exactly what it receives with no such file, and the command the
// valid pair names never runs.
#[test]
fn old_configuration_files_left_on_disk_change_nothing_about_a_launch() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["impl"],
    ));
    dispatch.drive_to_completion();
    let unconfigured = dispatch.launch(0).args;

    let ran = dispatch.root.join("old-command-ran");
    let old_command = dispatch.root.join("old-command");
    write_exec(
        &old_command,
        &format!("#!/bin/sh\n: > {}\n", shell_quote(&ran)),
    );
    let template = format!("{} '${{prompt}}'", shell_quote(&old_command));
    let valid = format!(
        "config {{\n    command \"old\" {template:?}\n    command \"other\" {template:?}\n    \
         bind \"lead\" \"old\"\n    route \"impl\" \"lead\"\n    \
         profile \"opposite\" {{ bind \"lead\" \"other\"; }}\n}}\n"
    );
    let selecting = "config { select \"opposite\"; }\n";
    let invalid = "not valid configuration";
    let personal = dispatch.home.join(".config/grove/config.kdl");
    fs::create_dir_all(personal.parent().unwrap()).unwrap();
    let delta = dispatch.worktree.join(".grove.kdl");

    for (n, (state, personal_text, delta_text, tracked)) in [
        ("valid", valid.as_str(), selecting, false),
        ("invalid", invalid, invalid, false),
        ("tracked", valid.as_str(), selecting, true),
    ]
    .into_iter()
    .enumerate()
    {
        fs::write(&personal, personal_text).unwrap();
        fs::write(&delta, delta_text).unwrap();
        let ignore = if tracked { "" } else { ".grove.kdl\n" };
        fs::write(dispatch.worktree.join(".gitignore"), ignore).unwrap();
        // The listing snapshots the working copy, so the delta is tracked
        // exactly when nothing ignores it.
        let files = support::jj(&dispatch.worktree, &["file", "list"]);
        assert_eq!(
            files.lines().any(|file| file == ".grove.kdl"),
            tracked,
            "{state}: {files}"
        );

        dispatch.drive_to_completion();
        assert_eq!(dispatch.launch(n + 1).args, unconfigured, "{state}");
        assert!(!ran.exists(), "{state}: the old command ran");
    }
}

// Root scaffolding consults no policy. In a working tree with no grove, under
// a HOME with no policy, bare `grove` writes the root and its first leaf, and
// only then asks dispatch, which refuses for want of a policy and names the
// subcommand that installs one. The leaf stays live, and once a policy exists
// the next run launches it.
#[test]
fn a_fresh_tree_is_scaffolded_with_no_policy_and_its_first_launch_requests_a_policy() {
    let dispatch = Dispatch::new("worktree");

    let output = run_driver(&dispatch.worktree, &dispatch.home);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    let leaf = dispatch.task_file("01-requirements--plan-k1.md");
    assert!(
        leaf.is_file() && dispatch.task_file("_BRIEF.md").is_file(),
        "the root and its first leaf must be written with no policy: {stderr}"
    );
    for said in [
        "refused (policy_missing",
        "exporting a `policy`",
        "session ended without a supported Grove result",
        "session kind `requirements` for `plan-k1` failed",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    assert_eq!(dispatch.launch_count(), 0, "{stderr}");

    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["requirements"],
    ));
    dispatch.drive_to_completion();
    assert_eq!(
        dispatch.launch(0).args[..3],
        [
            OsString::from("requirements"),
            leaf.into_os_string(),
            OsString::from("plan-k1"),
        ]
    );
}

// A Grove copy without Ravel on its isolated PATH refuses before scaffolding.
// Other cases use a separate Ravel installation and prove it need not be a sibling.
#[test]
fn a_missing_ravel_on_path_reports_the_independent_install_remedy() {
    let dispatch = Dispatch::new("worktree");
    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["requirements"],
    ));
    let alone = dispatch.root.join("alone/bin");
    fs::create_dir_all(&alone).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), alone.join("grove")).unwrap();

    let mut command = driver_command_at(&alone.join("grove"), &dispatch.worktree, &dispatch.home);
    command.env("PATH", "/usr/bin:/bin");
    let output = DriverProcess::capture(detached(command)).finish();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(
        stderr.contains("brew install linkuistics/taps/ravel"),
        "{stderr}"
    );
    assert!(!dispatch.worktree.join(".grove").exists(), "{stderr}");
    assert_eq!(dispatch.launch_count(), 0);
}

#[test]
fn installed_ravel_help_exposes_only_the_public_launch_boundary() {
    let output = Command::new(ravel())
        .args(["start-session", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    let help = String::from_utf8_lossy(&output.stdout);
    for flag in [
        "--kind",
        "--prompt",
        "--task-file",
        "--task-id",
        "--exit-dir",
        "--runtime-read",
    ] {
        assert!(help.contains(flag), "{help}");
    }
    assert!(!help.contains("--ending-file"), "{help}");
}

const SESSION_LIMIT: Duration = Duration::from_secs(60);
const INTERRUPT: u8 = 0x03;
const LAST_SIGNAL: libc::c_int = 64;

struct Pty {
    master: fs::File,
    slave: fs::File,
    /// The device a process with this terminal on stdin names.
    name: String,
    /// Everything written to the terminal so far, as its reader received it.
    shown: Arc<Mutex<Vec<u8>>>,
}

impl Pty {
    fn open() -> Self {
        use std::os::fd::{AsRawFd, FromRawFd};

        let (mut master, mut slave) = (-1, -1);
        // SAFETY: valid out pointers; default terminal settings and size.
        assert_eq!(
            unsafe {
                libc::openpty(
                    &mut master,
                    &mut slave,
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                    std::ptr::null_mut(),
                )
            },
            0
        );
        // SAFETY: openpty returned two fresh owned descriptors.
        let (master, slave) =
            unsafe { (fs::File::from_raw_fd(master), fs::File::from_raw_fd(slave)) };
        for fd in [master.as_raw_fd(), slave.as_raw_fd()] {
            // SAFETY: live descriptors; neither endpoint should leak on exec.
            assert_ne!(
                unsafe { libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) },
                -1
            );
        }
        // The cases type the interrupt character, so the line discipline must
        // turn it into SIGINT whatever this platform's defaults are.
        // SAFETY: a termios read from, and written back to, a live terminal.
        unsafe {
            let mut settings: libc::termios = std::mem::zeroed();
            assert_eq!(libc::tcgetattr(slave.as_raw_fd(), &mut settings), 0);
            settings.c_lflag |= libc::ISIG;
            settings.c_cc[libc::VINTR] = INTERRUPT;
            assert_eq!(
                libc::tcsetattr(slave.as_raw_fd(), libc::TCSANOW, &settings),
                0
            );
        }
        let mut name = [0 as libc::c_char; 128];
        // SAFETY: ttyname_r writes a NUL-terminated name within the buffer.
        assert_eq!(
            unsafe { libc::ttyname_r(slave.as_raw_fd(), name.as_mut_ptr(), name.len()) },
            0
        );
        // SAFETY: NUL-terminated by the successful call above.
        let name = unsafe { std::ffi::CStr::from_ptr(name.as_ptr()) }
            .to_str()
            .unwrap()
            .to_owned();
        // A terminal's reader, as an emulator is. On macOS a session leader's
        // exit first drains its controlling terminal's output, and the echo of
        // a typed Ctrl-C is output: unread, it leaves the driver exiting
        // forever. The thread ends once no slave descriptor is left open.
        let mut reader = master.try_clone().unwrap();
        let shown = Arc::new(Mutex::new(Vec::new()));
        let screen = Arc::clone(&shown);
        thread::spawn(move || {
            use std::io::Read as _;
            let mut buffer = [0; 1024];
            while let Ok(read @ 1..) = reader.read(&mut buffer) {
                screen.lock().unwrap().extend_from_slice(&buffer[..read]);
            }
        });
        Pty {
            master,
            slave,
            name,
            shown,
        }
    }

    /// What the terminal has shown so far.
    fn shown(&self) -> String {
        String::from_utf8_lossy(&self.shown.lock().unwrap()).into_owned()
    }

    /// Type Ctrl-C.
    fn interrupt(&self) {
        use std::io::Write as _;
        (&self.master).write_all(&[INTERRUPT]).unwrap();
    }
}

/// The signal state a driver starts with: these ignored, these blocked, and
/// every other signal at its default, whatever this test process inherited.
#[derive(Clone, Copy)]
struct Entry {
    ignored: &'static [libc::c_int],
    blocked: &'static [libc::c_int],
}

/// Nothing ignored and nothing blocked.
const PLAIN: Entry = Entry {
    ignored: &[],
    blocked: &[],
};

/// SIGUSR1 ignored and SIGUSR2 blocked, neither of which Grove resets, so
/// both reach every session it launches. A reference that shows them is not a
/// constant.
const MARKED: Entry = Entry {
    ignored: &[libc::SIGUSR1],
    blocked: &[libc::SIGUSR2],
};

impl DriverProcess {
    /// A driver started as a shell starts a foreground job: leading a session
    /// whose controlling terminal is `terminal`, so its group is the
    /// terminal's foreground group, with `entry` as its signal state. The
    /// session is the driver's own, never the developer's terminal.
    fn spawn_on(worktree: &Path, home: &Path, terminal: &Pty, entry: Entry) -> Self {
        Self::capture(command_on(worktree, home, terminal, entry))
    }
}

/// The driver's command for [`DriverProcess::spawn_on`]: its stdin and its
/// controlling terminal are `terminal`, and its other streams are the caller's
/// to place.
fn command_on(worktree: &Path, home: &Path, terminal: &Pty, entry: Entry) -> Command {
    command_on_at(
        Path::new(env!("CARGO_BIN_EXE_grove")),
        worktree,
        home,
        terminal,
        entry,
    )
}

fn command_on_at(
    program: &Path,
    worktree: &Path,
    home: &Path,
    terminal: &Pty,
    entry: Entry,
) -> Command {
    {
        let mut command = driver_command_at(program, worktree, home);
        command.stdin(Stdio::from(terminal.slave.try_clone().unwrap()));
        let Entry { ignored, blocked } = entry;
        // SAFETY: between fork and exec the closure makes only setsid, ioctl,
        // signal, sigemptyset, sigaddset and sigprocmask calls, over static
        // slices.
        unsafe {
            command.pre_exec(move || {
                if libc::setsid() == -1
                    || libc::ioctl(libc::STDIN_FILENO, libc::TIOCSCTTY as _, 0) == -1
                {
                    return Err(std::io::Error::last_os_error());
                }
                for signal in 1..=LAST_SIGNAL {
                    if signal != libc::SIGKILL && signal != libc::SIGSTOP {
                        libc::signal(signal, libc::SIG_DFL);
                    }
                }
                for &signal in ignored {
                    libc::signal(signal, libc::SIG_IGN);
                }
                let mut mask: libc::sigset_t = std::mem::zeroed();
                libc::sigemptyset(&mut mask);
                for &signal in blocked {
                    libc::sigaddset(&mut mask, signal);
                }
                if libc::sigprocmask(libc::SIG_SETMASK, &mask, std::ptr::null_mut()) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        command
    }
}

/// The session probe, compiled once per test binary with the host's C
/// compiler (`$CC`, else `cc`, which the bundled SQLite build already needs)
/// into Cargo's scratch directory. A compiler that is missing or fails fails
/// the case; nothing is skipped.
fn session_probe() -> &'static Path {
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    BUILT.get_or_init(|| {
        let source = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/session-probe.c");
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR"));
        fs::create_dir_all(dir).unwrap();
        let built = dir.join("session-probe");
        // A private name first, so a concurrent run never executes a
        // half-written file.
        let partial = dir.join(format!("session-probe.{}", std::process::id()));
        let compiler = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
        let output = Command::new(&compiler)
            .args(["-Wall", "-Wextra", "-Werror", "-o"])
            .arg(&partial)
            .arg(source)
            .output()
            .unwrap_or_else(|error| panic!("cannot run the C compiler {compiler:?}: {error}"));
        assert!(
            output.status.success(),
            "compiling {source} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        fs::rename(&partial, &built).unwrap();
        built
    })
}

/// What the session probe reported about the process exec made it.
#[derive(Debug)]
struct Probed {
    pid: libc::pid_t,
    parent: libc::pid_t,
    group: libc::pid_t,
    /// The controlling terminal's foreground group, or -1 without one.
    foreground: libc::pid_t,
    /// The terminal on stdin, or `-`.
    stdin: String,
    cwd: PathBuf,
    signals: Signals,
}

/// Signal state as sets of signal numbers.
#[derive(Debug, PartialEq, Eq)]
struct Signals {
    ignored: BTreeSet<libc::c_int>,
    blocked: BTreeSet<libc::c_int>,
    pending: BTreeSet<libc::c_int>,
    caught: BTreeSet<libc::c_int>,
}

impl Probed {
    fn read(path: &Path) -> Self {
        let report = fs::read_to_string(path)
            .unwrap_or_else(|error| panic!("no probe report at {}: {error}", path.display()));
        let fields: BTreeMap<&str, &str> = report
            .lines()
            .map(|line| line.split_once(' ').unwrap_or((line, "")))
            .collect();
        let field = |name: &str| {
            *fields
                .get(name)
                .unwrap_or_else(|| panic!("no {name} in the probe report: {report}"))
        };
        let number = |name: &str| field(name).parse().unwrap();
        let set = |name: &str| {
            field(name)
                .split_whitespace()
                .map(|number| number.parse().unwrap())
                .collect()
        };
        Probed {
            pid: number("pid"),
            parent: number("ppid"),
            group: number("pgid"),
            foreground: number("foreground"),
            stdin: field("stdin").to_owned(),
            cwd: PathBuf::from(field("cwd")),
            signals: Signals {
                ignored: set("ignored"),
                blocked: set("blocked"),
                pending: set("pending"),
                caught: set("caught"),
            },
        }
    }
}

/// The process Grove launched for a session, read from the process table as
/// the driver's only child while the session held, and the terminal it ran on.
struct Held {
    pid: libc::pid_t,
    group: libc::pid_t,
    terminal: String,
}

impl Dispatch {
    /// The step the session probe execs once it has reported.
    fn after_probe(&self) -> PathBuf {
        self.root.join("after-probe")
    }

    /// Write that step. It records the harness's start as the fake harness
    /// does, in the record the probe made, and then runs `then`. The probe has
    /// already reported the cwd, so the step first moves to the working tree,
    /// whose epoch it reads and which an altered probe left.
    fn probe_then(&self, then: &str) {
        write_exec(
            &self.after_probe(),
            &format!(
                "#!/bin/sh\n\
                 record=$SESSION_RECORD\n\
                 cd {worktree} || exit 89\n\
                 {RECORD_START}\
                 {then}\n",
                worktree = shell_quote(&self.worktree),
            ),
        );
    }

    /// The directory launch `n` records in.
    fn record(&self, n: usize) -> PathBuf {
        self.launches.join(n.to_string())
    }

    fn probed(&self, n: usize) -> Probed {
        Probed::read(&self.record(n).join("process"))
    }

    /// A policy whose `select` returns the probe for every kind, altering
    /// what it was handed if `alter` says so. At import it writes the view: its stdin and
    /// a descriptor on the controlling terminal as one measurement reads them,
    /// `/dev/null`'s device, the names in its environment, and the channel's
    /// value if it has one. `prelude` runs next.
    fn terminal_policy(&self, alter: bool, prelude: &str) -> String {
        let args: String = [alter.then_some("--alter")]
            .into_iter()
            .flatten()
            .chain([
                self.launches.to_str().unwrap(),
                self.after_probe().to_str().unwrap(),
            ])
            .map(|arg| format!("{arg:?}, "))
            .collect();
        format!(
            r#"import {{ closeSync, fstatSync, openSync, renameSync, statSync, writeFileSync }} from "node:fs";
import {{ isatty }} from "node:tty";
const device = (fd: number) => ({{ terminal: isatty(fd), rdev: fstatSync(fd).rdev }});
const controlling = openSync("/dev/tty", "r");
writeFileSync({view:?}, JSON.stringify({{
  stdin: device(0),
  controlling: device(controlling),
  null: statSync("/dev/null").rdev,
  env: Object.keys(process.env).sort(),
  channel: process.env.GROVE_LAUNCH_DIR ?? null,
}}));
closeSync(controlling);
{prelude}
export const policy = {{
  schemaVersion: 2,
  version: "grove-terminal-1",
  select: (request) => ({{
    status: "selected",
    program: {probe:?},
    args: [{args}request.prompt],
    provider: "origin-a",
    model: "model-a",
    effort: "high",
    reason: "every kind runs the probe",
  }}),
}};
"#,
            view = self.view.to_str().unwrap(),
            probe = session_probe().to_str().unwrap(),
        )
    }

    /// Run the loop on a terminal of its own until launch `n`'s session
    /// completes, holding that session until the process Grove launched for
    /// it has been read from the process table.
    fn drive_held(&self, entry: Entry, n: usize) -> Held {
        let terminal = Pty::open();
        let mut driver = DriverProcess::spawn_on(&self.worktree, &self.home, &terminal, entry);
        driver.wait_for_ready(&self.record(n).join("ready"));
        let children = children_of(driver.id());
        fs::write(self.record(n).join("go"), "").unwrap();
        let output = driver.finish_within(SESSION_LIMIT);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success() && stderr.contains("grove finished — loop complete"),
            "the held session did not complete the loop: {stderr}"
        );
        let [(pid, group)] = children.as_slice() else {
            panic!("the driver had other than one child while its session held: {children:?}");
        };
        Held {
            pid: *pid,
            group: *group,
            terminal: terminal.name,
        }
    }

    /// Run the loop on a terminal of its own until it stops on a session that
    /// ended without a supported Grove result, doing `meanwhile` to the running
    /// driver and its terminal first. Returns the status Grove reported for
    /// that session, and all it said.
    fn drive_to_stop(&self, meanwhile: impl FnOnce(&mut DriverProcess, &Pty)) -> (String, String) {
        let terminal = Pty::open();
        let mut driver = DriverProcess::spawn_on(&self.worktree, &self.home, &terminal, PLAIN);
        meanwhile(&mut driver, &terminal);
        let output = driver.finish_within(SESSION_LIMIT);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(output.status.success(), "{stderr}");
        assert!(
            !stderr.contains("interrupted by signal"),
            "the driver itself was interrupted: {stderr}"
        );
        let status = stderr
            .split_once("session ended without a supported Grove result — status ")
            .and_then(|(_, rest)| rest.split_once(", elapsed "))
            .unwrap_or_else(|| panic!("no ended session reported: {stderr}"))
            .0
            .to_owned();
        (status, stderr)
    }

    /// Wait until launch `n`'s harness has become the `sleep` its step execs,
    /// and type Ctrl-C at `terminal`.
    fn interrupt_running(&self, driver: &mut DriverProcess, terminal: &Pty, n: usize) {
        // The step writes `args` after the probe's report is closed.
        driver.wait_for_ready(&self.record(n).join("args"));
        let pid = self.probed(n).pid.to_string();
        let deadline = Instant::now() + SESSION_LIMIT;
        loop {
            let command = Command::new("ps")
                .args(["-o", "comm=", "-p", &pid])
                .output()
                .unwrap();
            let command = String::from_utf8_lossy(&command.stdout);
            if command.trim().rsplit('/').next() == Some("sleep") {
                break;
            }
            assert!(
                Instant::now() < deadline,
                "launch {n}'s harness never became sleep: {command:?}"
            );
            thread::sleep(Duration::from_millis(10));
        }
        terminal.interrupt();
    }
}

/// A step that says it is ready, holds until told to go, and then completes
/// the loop.
fn held_then_done() -> String {
    format!(
        ": > \"$record/ready\"\n\
         until [ -e \"$record/go\" ]; do sleep 0.01; done\n\
         {}",
        complete_done()
    )
}

/// A policy prelude that records the worker's PID in `pid_file` and then
/// holds selection, for `ms` milliseconds or, without them, until the worker
/// is stopped. A live timer keeps the worker's event loop busy, so the await
/// is never reported as stuck.
///
/// The file is renamed into place whole. A case interrupts the selection as
/// soon as the file exists, and a file created and then written could be seen
/// empty by a worker stopped between the two.
fn holding(pid_file: &Path, ms: Option<u64>) -> String {
    let settle = match ms {
        Some(ms) => format!("setTimeout(resolve, {ms})"),
        None => "setInterval(() => {}, 1000)".to_owned(),
    };
    let pid_file = pid_file.to_str().unwrap();
    format!(
        "writeFileSync({partial:?}, String(process.pid));\n\
         renameSync({partial:?}, {pid_file:?});\n\
         await new Promise((resolve) => {{ {settle}; }});",
        partial = format!("{pid_file}.partial"),
    )
}

/// Whether `pid` names any process at all, a zombie included.
fn exists(pid: libc::pid_t) -> bool {
    // SAFETY: signal 0 only checks that the process exists.
    let result = unsafe { libc::kill(pid, 0) };
    result == 0 || std::io::Error::last_os_error().raw_os_error() != Some(libc::ESRCH)
}

/// Whether `pid` is gone within `limit`, since an orphan is reaped by whatever
/// adopts it rather than by this test.
fn gone_within(pid: libc::pid_t, limit: Duration) -> bool {
    let deadline = Instant::now() + limit;
    while exists(pid) {
        if Instant::now() >= deadline {
            return false;
        }
        thread::sleep(Duration::from_millis(20));
    }
    true
}

// The harness is the job dispatch launched beneath the one Grove launched.
// Grove spawned one process into a group of its own and handed it the
// terminal; the front and its worker ran as that process and its group, and
// the front spawned the harness as its own child, into a group of its own, and
// handed the terminal on. So the harness reports the PID the process table
// gave Grove's only child as its parent, a group it leads holding the
// terminal, the terminal as its stdin, and the working tree as its cwd. Its signal state is the one Grove's spawn gives its
// child, SIGPIPE's included: the driver starts with SIGUSR1 ignored and SIGUSR2
// blocked, neither of which Grove resets, and the harness holds exactly those
// two and nothing dispatch added. The policy worker has `/dev/null` for stdin
// and no Grove or dispatch control variable, and the harness has the fresh
// channel the live epoch names, which it completes through.
//
// The controls are one altered run. The probe forks, and its child, whose
// parent is the probe, leaves the group, swaps its stdin for `/dev/null`, moves to `/`, flips SIGPIPE and
// blocks SIGALRM before it reports; the driver starts plain; and the worker is
// granted the channel. Every observation then changes, and each change has
// one cause, since the entry and the alteration touch different signals. The
// worker's stdin cannot be altered from outside the front, so the same
// measurement is also taken of a descriptor on the controlling terminal, and
// reads it as a terminal.
#[test]
fn the_harness_is_the_foreground_job_grove_launched() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.probe_then(&held_then_done());
    let worktree = dispatch.worktree.canonicalize().unwrap();

    dispatch.policy(&dispatch.terminal_policy(false, ""));
    let held = dispatch.drive_held(MARKED, 0);
    let probed = dispatch.probed(0);
    assert_eq!(held.pid, held.group, "Grove's child leads its own group");
    assert_eq!(
        probed.parent, held.pid,
        "the harness is the child of the process Grove launched: {probed:?}"
    );
    assert_eq!(
        probed.group, probed.pid,
        "the harness leads a job of its own: {probed:?}"
    );
    assert_eq!(
        probed.foreground, probed.group,
        "the harness's group holds the terminal: {probed:?}"
    );
    assert_eq!(probed.stdin, held.terminal, "{probed:?}");
    assert_eq!(probed.cwd, worktree, "{probed:?}");
    assert_eq!(
        probed.signals,
        Signals {
            ignored: BTreeSet::from([libc::SIGUSR1]),
            blocked: BTreeSet::from([libc::SIGUSR2]),
            pending: BTreeSet::new(),
            caught: BTreeSet::new(),
        },
        "dispatch must hand on the signal state Grove's spawn handed it"
    );

    let launch = dispatch.launch(0);
    assert_ne!(launch.channel, "<unset>");
    assert!(
        epoch_names(&launch.epoch, &launch.launch_dir),
        "the harness's channel {:?} is not the one the live epoch names: {:?}",
        launch.channel,
        launch.epoch
    );

    let view = dispatch.view();
    assert!(
        view["null"].as_u64().is_some_and(|rdev| rdev != 0),
        "{view}"
    );
    assert_eq!(
        view["stdin"]["rdev"], view["null"],
        "the worker's stdin is /dev/null: {view}"
    );
    assert_eq!(view["stdin"]["terminal"], false, "{view}");
    assert_eq!(
        view["controlling"]["terminal"], true,
        "the same measurement reads the controlling terminal as one: {view}"
    );
    let names: Vec<&str> = view["env"]
        .as_array()
        .unwrap()
        .iter()
        .map(|name| name.as_str().unwrap())
        .collect();
    assert!(names.contains(&"HOME"), "the probe read nothing: {view}");
    assert!(
        !names.iter().any(|name| name.starts_with("GROVE_")
            || name.starts_with("HARNESS_DISPATCH_")
            || name.starts_with("RAVEL_")),
        "the worker holds a control variable: {view}"
    );
    assert_eq!(view["channel"], serde_json::Value::Null, "{view}");

    // The controls.
    dispatch.policy(&dispatch.terminal_policy(true, ""));
    dispatch.settings(r#"{ "policyEnv": ["GROVE_LAUNCH_DIR"] }"#);
    let held = dispatch.drive_held(PLAIN, 1);
    let altered = dispatch.probed(1);
    assert_ne!(altered.parent, held.pid, "{altered:?}");
    assert_ne!(altered.group, held.group, "{altered:?}");
    assert_ne!(altered.foreground, altered.group, "{altered:?}");
    assert_eq!(altered.stdin, "-", "{altered:?}");
    assert_eq!(altered.cwd, Path::new("/"), "{altered:?}");
    let signals = &altered.signals;
    assert!(!signals.ignored.contains(&libc::SIGUSR1), "{altered:?}");
    assert!(!signals.blocked.contains(&libc::SIGUSR2), "{altered:?}");
    assert!(signals.ignored.contains(&libc::SIGPIPE), "{altered:?}");
    assert!(signals.blocked.contains(&libc::SIGALRM), "{altered:?}");
    let granted = dispatch.launch(1);
    assert_eq!(dispatch.view()["channel"], granted.launch_dir.as_str());
    assert_ne!(
        granted.channel, launch.channel,
        "each launch gets a fresh channel"
    );
}

// The harness's own ending reaches Grove unmodified: an exit code, here one
// dispatch itself exits with when it refuses, and a death by a signal the
// harness sends itself. The harness ran each time, and dispatch refused
// nothing, so the code is the harness's. Each ending is the other's control:
// Grove reports them differently, so a report that did not follow the
// harness's ending could not match both.
#[test]
fn the_harness_s_exit_and_signal_death_reach_grove_unmodified() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.policy(&dispatch.terminal_policy(false, ""));
    for (n, (ending, reported)) in [
        ("exit 3", "exit status: 3".to_owned()),
        (
            "kill -USR1 $$",
            format!("signal: {} (SIGUSR1)", libc::SIGUSR1),
        ),
    ]
    .into_iter()
    .enumerate()
    {
        dispatch.probe_then(ending);
        let (status, stderr) = dispatch.drive_to_stop(|_, _| {});
        assert_eq!(status, "exit status: 1", "`{ending}`: {stderr}");
        assert!(stderr.contains(&reported), "{stderr}");
        assert!(stderr.contains("session_result_missing"), "{stderr}");
        assert_ne!(
            dispatch.launch(n).channel,
            "<unset>",
            "launch {n} never reached its harness"
        );
    }
}

// A refused launch says why on the terminal the session would have had. The
// driver's streams are the terminal here, as a shell gives them, and what the
// terminal shows holds dispatch's own diagnostic, with its remedy and the
// `inspect` invocation, above Grove's report of the kind, the handle and the
// stopped loop. The control is the same leaf once the policy routes its kind:
// a fresh terminal then shows the loop finishing and no refusal.
#[test]
fn a_refused_launch_s_diagnostic_reaches_the_terminal() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let shown_after = |kinds: &[&str], until: &str| {
        dispatch.policy(&probing_policy(&dispatch.view, &dispatch.harness, kinds));
        let terminal = Pty::open();
        let mut command = command_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
        command
            .stdout(Stdio::from(terminal.slave.try_clone().unwrap()))
            .stderr(Stdio::from(terminal.slave.try_clone().unwrap()));
        let mut driver = Reaped(command.spawn().unwrap());
        // The reader is a thread, so what the driver wrote last may arrive
        // after the driver is gone.
        let deadline = Instant::now() + SESSION_LIMIT;
        loop {
            let shown = terminal.shown();
            if shown.contains(until) && driver.0.try_wait().unwrap().is_some() {
                return shown;
            }
            assert!(
                Instant::now() < deadline,
                "the terminal never showed {until:?}: {shown}"
            );
            thread::sleep(Duration::from_millis(20));
        }
    };

    let shown = shown_after(&["design"], "rerun `grove` to continue");
    let at = |said: &str| {
        shown
            .find(said)
            .unwrap_or_else(|| panic!("no {said:?} on the terminal: {shown}"))
    };
    let refusal = at("ravel: refused (policy_refused, stage selection)");
    for said in [
        "  policy code: incomplete_mapping",
        "  remedy: add the kind to the policy",
    ] {
        assert!(at(said) > refusal, "{shown}");
    }
    assert!(
        at("session kind `impl` for `subject-k1` failed")
            > at("  remedy: add the kind to the policy"),
        "Grove's report points at a diagnostic above it: {shown}"
    );
    assert_eq!(dispatch.launch_count(), 0, "{shown}");

    let shown = shown_after(&["impl"], "grove finished — loop complete");
    assert!(!shown.contains("refused ("), "{shown}");
    assert_eq!(dispatch.launch_count(), 1, "{shown}");
}

// Ctrl-C typed at the terminal reaches the foreground job, whichever stage it
// is at, and Grove answers the job's end the same way. Interrupted while the
// policy holds selection, the front and its worker are that job: the front
// reports the cancellation and dies of the signal, having launched nothing,
// recorded nothing and left no worker, and Grove reports that status and
// stops. The control is the same hold ended after a moment, which launches,
// and whose harness is then interrupted as it runs: Grove reports the same
// status for it. The driver is never interrupted itself: the terminal was the
// job's.
#[test]
fn an_interrupt_typed_at_the_terminal_ends_the_job_during_selection_and_during_execution() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.probe_then("exec sleep 60");
    let interrupted = format!("signal: {} (SIGINT)", libc::SIGINT);

    let worker = dispatch.root.join("worker-pid");
    dispatch.policy(&dispatch.terminal_policy(false, &holding(&worker, None)));
    // A selection bound far past the case, so the refusal cannot be a timeout.
    dispatch.settings(r#"{ "timeoutMs": 60000 }"#);
    let (status, stderr) = dispatch.drive_to_stop(|driver, terminal| {
        driver.wait_for_ready(&worker);
        terminal.interrupt();
    });
    assert_eq!(status, interrupted, "{stderr}");
    for said in [
        "refused (session_cancelled, stage exec)",
        "signal: SIGINT",
        "session kind `impl` for `subject-k1` failed",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    assert_eq!(dispatch.launch_count(), 0, "a harness launched: {stderr}");
    let pid: libc::pid_t = fs::read_to_string(&worker).unwrap().trim().parse().unwrap();
    assert!(!exists(pid), "worker {pid} outlived its selection");

    dispatch.policy(&dispatch.terminal_policy(false, &holding(&worker, Some(300))));
    let (status, stderr) =
        dispatch.drive_to_stop(|driver, terminal| dispatch.interrupt_running(driver, terminal, 0));
    assert_eq!(status, "exit status: 1", "{stderr}");
    assert!(stderr.contains(&interrupted), "{stderr}");
    assert!(stderr.contains("session_result_missing"), "{stderr}");
    assert_ne!(dispatch.launch(0).channel, "<unset>");
}

// Dispatch's escalation reaps the harness's descendants under a terminal
// as it does detached: the harness leads the group dispatch signals. The harness spawns a
// descendant, completes through the channel, and declines to end, so the
// escalation runs. The bystander, the same shape of process in this test's
// own group, is the control: a probe that read every process gone would read
// it gone too.
#[test]
fn the_escalation_reaps_the_session_s_descendants_under_a_terminal() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.probe_then(&format!(
        "sh -c 'while : ; do sleep 0.05 ; done' &\n\
         printf '%s\\n' \"$!\" > \"$record/descendant\"\n\
         touch \"$GROVE_LAUNCH_DIR/teardown\"; {dispatch_exit} end-session grove-continue || exit 91\n\
         while : ; do sleep 0.05 ; done",
        dispatch_exit = shell_quote(&ravel()),
    ));
    dispatch.policy(&dispatch.terminal_policy(false, ""));
    let mut bystander = Reaped(
        Command::new("sh")
            .arg("-c")
            .arg("while : ; do sleep 0.05 ; done")
            .spawn()
            .unwrap(),
    );

    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stderr.contains("grove finished — loop complete"),
        "{stderr}"
    );
    assert_ne!(dispatch.launch(0).channel, "<unset>");
    let descendant: libc::pid_t = fs::read_to_string(dispatch.record(0).join("descendant"))
        .expect("the session never reported its descendant")
        .trim()
        .parse()
        .unwrap();
    let reaped = gone_within(descendant, Duration::from_secs(5));
    if !reaped {
        // SAFETY: `kill(2)` on a pid the fixture reported, so a failing
        // assertion does not leave it running.
        unsafe { libc::kill(descendant, libc::SIGKILL) };
    }
    assert!(reaped, "the session's descendant outlived the escalation");
    assert!(
        bystander.0.try_wait().unwrap().is_none(),
        "the escalation reached a process outside the session's own group"
    );
}

#[test]
fn force_stop_leaves_remaining_work_resumable_after_continuation() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    fs::write(
        dispatch.worktree.join(".grove/02-impl--remaining-k2.md"),
        "# remaining-k2\n",
    )
    .unwrap();
    let llm = shell_quote(&own_grove_llm());
    dispatch.harness_then(&format!(
        "set -e\n\
         case $n in\n\
         0) {llm} leaf-retire .grove/01-impl--subject-k1.md; jj describe -m fixture; jj new; \
            exec {ravel} end-session grove-continue ;;\n\
         1) sh -c 'while :; do sleep 0.05; done' &\n\
            printf '%s' \"$!\" > \"$record/descendant\"\n\
            {ravel} end-session grove-force-stop\n\
            while :; do sleep 0.05; done ;;\n\
         2) {} ;;\n\
         *) exit 91 ;;\n\
         esac",
        complete_done(),
        ravel = shell_quote(&ravel()),
    ));
    support::route_every_kind_to(&dispatch.home, &dispatch.harness);
    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.code(), Some(1), "{stderr}");
    assert!(stderr.contains("force-stop requested"), "{stderr}");
    assert!(!stderr.contains("grove finished"), "{stderr}");
    assert_eq!(dispatch.launch_count(), 2, "{stderr}");
    assert!(dispatch.task_file("01-DONE-impl--subject-k1.md").exists());
    assert!(dispatch.task_file("02-impl--remaining-k2.md").exists());
    let descendant = fs::read_to_string(dispatch.record(1).join("descendant"))
        .unwrap()
        .parse()
        .unwrap();
    assert!(gone_within(descendant, Duration::from_secs(5)));

    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stderr.contains("grove finished"),
        "{stderr}"
    );
    assert_eq!(dispatch.launch_count(), 3, "{stderr}");
}

#[test]
fn launch_directory_teardown_finishes_after_the_ravel_result() {
    for ending in ["exit_signal", "harness_exit", "force_stop"] {
        let dispatch = Dispatch::new("worktree");
        plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
        let llm = shell_quote(&own_grove_llm());
        dispatch.harness_then(&format!(
            "set -e\n\
             printf '%s' \"$GROVE_LAUNCH_DIR\" > \"$record/launch-dir\"\n\
             test -d \"$GROVE_LAUNCH_DIR\"\n\
             if {llm} record-teardown > \"$record/refusal\" 2>&1; then exit 91; fi\n\
             test ! -e \"$GROVE_LAUNCH_DIR/teardown\"\n\
             {llm} leaf-retire .grove/01-impl--subject-k1.md\n\
             jj describe -m fixture\n\
             jj new\n\
             printf '# finish-k2\\n' > .grove/02-finish--finish-k2.md\n\
             jj describe -m finish-fixture\n\
             jj new\n\
             {llm} finish-commit finish-k2\n\
             {llm} record-teardown\n\
             test -f \"$GROVE_LAUNCH_DIR/teardown\"\n\
             {llm} record-teardown\n\
             {}",
            if ending == "exit_signal" {
                format!("exec {} end-session grove-continue", shell_quote(&ravel()))
            } else if ending == "force_stop" {
                format!(
                    "exec {} end-session grove-force-stop",
                    shell_quote(&ravel())
                )
            } else {
                "exit 7".into()
            }
        ));
        support::route_every_kind_to(&dispatch.home, &dispatch.harness);
        let terminal = Pty::open();
        let mut driver =
            DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
        let output = driver.finish_within(SESSION_LIMIT);
        let stderr = String::from_utf8_lossy(&output.stderr);
        if ending == "force_stop" {
            assert_eq!(output.status.code(), Some(1), "{ending}: {stderr}");
            assert!(stderr.contains("force-stop requested"), "{stderr}");
            assert!(!stderr.contains("grove finished"), "{stderr}");
        } else {
            assert!(
                output.status.success() && stderr.contains("grove finished — loop complete"),
                "{ending}: {stderr}"
            );
        }
        assert_eq!(dispatch.launch_count(), 1);
        let record = dispatch.launches.join("0");
        assert!(fs::read_to_string(record.join("refusal"))
            .unwrap()
            .contains("finish-commit"));
        let launch = PathBuf::from(fs::read_to_string(record.join("launch-dir")).unwrap());
        let name = launch.file_name().unwrap().to_str().unwrap();
        let suffix = name.strip_prefix("launch-").unwrap();
        assert_eq!(suffix.len(), 32);
        assert!(suffix
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)));
        assert_eq!(
            launch.parent().unwrap(),
            dispatch.worktree.canonicalize().unwrap().join(".jj/grove")
        );
        assert_eq!(
            Path::new(&dispatch.launch(0).channel)
                .parent()
                .and_then(Path::parent),
            Some(launch.as_path())
        );
        assert!(
            !launch.exists(),
            "interpreted launch directory was retained"
        );
    }
}

// Moving teardown above Interrupted must fail this real launch boundary.
#[test]
fn a_driver_interrupt_overrides_a_recorded_teardown_under_a_terminal() {
    use std::os::unix::process::ExitStatusExt;
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let llm = shell_quote(&own_grove_llm());
    let ready = dispatch.root.join("teardown-ready");
    dispatch.harness_then(&format!(
        "set -e\n\
         {llm} leaf-retire .grove/01-impl--subject-k1.md\n\
         jj describe -m fixture\n\
         jj new\n\
         printf '# finish-k2\\n' > .grove/02-finish--finish-k2.md\n\
         jj describe -m finish-fixture\n\
         jj new\n\
         {llm} finish-commit finish-k2\n\
         {llm} record-teardown\n\
         touch {ready}\n\
         exec sleep 60",
        ready = shell_quote(&ready),
    ));
    support::route_every_kind_to(&dispatch.home, &dispatch.harness);
    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    driver.wait_for_ready(&ready);
    assert_eq!(unsafe { libc::kill(driver.id(), libc::SIGTERM) }, 0);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!(output.status.signal(), Some(libc::SIGTERM), "{stderr}");
    assert!(stderr.contains("interrupted by signal"), "{stderr}");
    assert!(!stderr.contains("grove finished"), "{stderr}");
    assert_eq!(dispatch.launch_count(), 1, "{stderr}");
}

#[test]
fn a_post_reap_interrupt_overrides_teardown_and_force_stop() {
    use std::os::unix::process::ExitStatusExt;
    for result in ["grove-continue", "grove-force-stop"] {
        let dispatch = Dispatch::new("worktree");
        plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
        dispatch.harness_then(&format!(
            "touch \"$GROVE_LAUNCH_DIR/teardown\"; exec {} end-session {result}",
            shell_quote(&ravel()),
        ));
        support::route_every_kind_to(&dispatch.home, &dispatch.harness);
        let bin = dispatch.root.join("terminal-reset-bin");
        fs::create_dir(&bin).unwrap();
        let ready = dispatch.root.join("reset-ready");
        let release = dispatch.root.join("reset-release");
        write_exec(&bin.join("stty"), &format!(
            "#!/bin/sh\nprintf '' > {ready}\nuntil test -f {release}; do /bin/sleep 0.01; done\nexec /bin/stty \"$@\"\n",
            ready = shell_quote(&ready), release = shell_quote(&release),
        ));
        let terminal = Pty::open();
        let mut command = command_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
        command.env(
            "PATH",
            std::env::join_paths(
                std::iter::once(bin).chain(std::env::split_paths(&support::ravel_path())),
            )
            .unwrap(),
        );
        let mut driver = DriverProcess::capture(command);
        driver.wait_for_ready(&ready);
        assert_eq!(unsafe { libc::kill(driver.id(), libc::SIGTERM) }, 0);
        fs::write(&release, "").unwrap();
        let output = driver.finish_within(SESSION_LIMIT);
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(
            output.status.signal(),
            Some(libc::SIGTERM),
            "{result}: {stderr}"
        );
        assert!(stderr.contains("interrupted by signal"), "{stderr}");
        assert!(!stderr.contains("grove finished"), "{stderr}");
        assert!(!stderr.contains("force-stop requested"), "{stderr}");
        assert_eq!(dispatch.launch_count(), 1);
    }
}

#[test]
fn launch_directory_rotates_and_rejects_stale_tree_and_teardown_verbs() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let llm = shell_quote(&own_grove_llm());
    dispatch.harness_then(&format!(
        "set -e\n\
         printf '%s' \"$GROVE_LAUNCH_DIR\" > \"$record/launch-dir\"\n\
         ls -ld \"$GROVE_LAUNCH_DIR\" > \"$record/mode\"\n\
         if [ \"$n\" = 0 ]; then exec {exit_dispatch} end-session grove-continue; fi\n\
         old=$(cat {launches}/0/launch-dir)\n\
         test ! -e \"$old\"\n\
         for verb in 'pick' 'leaf-add . stale --kind impl' 'record-teardown'; do\n\
             if env GROVE_LAUNCH_DIR=\"$old\" {llm} $verb > \"$record/stale-$verb\" 2>&1; then exit 92; fi\n\
         done\n\
         touch \"$GROVE_LAUNCH_DIR/teardown\"; exec {exit_dispatch} end-session grove-continue",
        launches = shell_quote(&dispatch.launches),
        exit_dispatch = shell_quote(&ravel())
    ));
    support::route_every_kind_to(&dispatch.home, &dispatch.harness);
    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stderr.contains("grove finished — loop complete"),
        "{stderr}"
    );
    assert_eq!(dispatch.launch_count(), 2);
    let directory = |n| {
        PathBuf::from(
            fs::read_to_string(dispatch.launches.join(format!("{n}/launch-dir"))).unwrap(),
        )
    };
    assert_ne!(directory(0), directory(1));
    for n in 0..2 {
        assert!(!directory(n).exists());
        assert!(
            fs::read_to_string(dispatch.launches.join(format!("{n}/mode")))
                .unwrap()
                .starts_with("drwx------")
        );
    }
    for verb in ["pick", "leaf-add . stale --kind impl", "record-teardown"] {
        let refusal =
            fs::read_to_string(dispatch.launches.join(format!("1/stale-{verb}"))).unwrap();
        assert!(refusal.contains("stale Grove session"), "{refusal}");
    }
    assert!(!dispatch
        .worktree
        .join(".grove/02-impl--stale-k2.md")
        .exists());
}

#[test]
fn replacement_driver_removes_abandoned_launches_without_interpreting_contents() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let control = dispatch.worktree.join(".jj/grove");
    fs::create_dir_all(&control).unwrap();
    let abandoned = control.join(format!("launch-{}", "a".repeat(32)));
    fs::create_dir(&abandoned).unwrap();
    fs::write(
        abandoned.join("teardown"),
        "abandoned teardown must not finish this driver",
    )
    .unwrap();
    let fifo = std::ffi::CString::new(abandoned.join("ending.json").to_str().unwrap()).unwrap();
    // The normal reader rejects this FIFO without blocking; cleanup unlinks it.
    assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
    let stale_exit = control.join(format!("launch-{}", "c".repeat(32)));
    fs::create_dir(&stale_exit).unwrap();
    fs::write(
        stale_exit.join("ending.json"),
        r#"{"schemaVersion":1,"source":"harness-dispatch","measurements":{"ending":{"state":"observed","value":"exit_signal"}}}"#,
    ).unwrap();
    let retained = control.join("launch-not-a-128-bit-suffix");
    fs::create_dir(&retained).unwrap();
    let outside = dispatch.root.join("outside");
    fs::create_dir(&outside).unwrap();
    fs::write(outside.join("keep"), "keep").unwrap();
    let link = control.join(format!("launch-{}", "b".repeat(32)));
    std::os::unix::fs::symlink(&outside, &link).unwrap();
    dispatch.harness_then("exit 0");
    support::route_every_kind_to(&dispatch.home, &dispatch.harness);
    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stderr.contains("loop stopped"),
        "{stderr}"
    );
    assert!(
        !stderr.contains("grove finished"),
        "an abandoned teardown was interpreted: {stderr}"
    );
    assert_eq!(
        dispatch.launch_count(),
        1,
        "an abandoned exit signal relaunched: {stderr}"
    );
    assert!(!stale_exit.exists());
    assert!(!abandoned.exists());
    assert!(!link.exists());
    assert!(outside.join("keep").is_file());
    assert!(retained.is_dir());
}

// The driver must act on dispatch's ending, independent of the harness status.
#[test]
fn dispatch_exit_signal_relaunches_and_own_exit_stops_under_a_terminal() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let exit = shell_quote(&ravel());
    dispatch.harness_then(&format!(
        "test -z \"${{GROVE_SIGNAL_FILE-}}\" || exit 91\n\
         if [ $n = 0 ]; then {exit} end-session grove-continue || exit 92; exit 7; fi\n\
         exit 3"
    ));
    support::route_every_kind_to(&dispatch.home, &dispatch.harness);
    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert_eq!(dispatch.launch_count(), 2, "{stderr}");
    assert!(dispatch.task_file("01-impl--subject-k1.md").is_file());
    assert!(stderr.contains("loop stopped"), "{stderr}");
    assert!(!stderr.contains("grove finished"), "{stderr}");
}
// Hold the driver's reset at stty, after runner recovery and before any reset
// changes the handed-over modes. The helper joins the driver's group.
fn hold_terminal_reset(dispatch: &Dispatch) -> (OsString, PathBuf, PathBuf) {
    let bin = dispatch.root.join("reset-bin");
    let reports = dispatch.root.join("reset-reports");
    let ready = dispatch.root.join("reset-ready");
    let go = dispatch.root.join("reset-go");
    fs::create_dir(&bin).unwrap();
    fs::create_dir(&reports).unwrap();
    let after = dispatch.root.join("after-reset-probe");
    write_exec(
        &after,
        &format!(
        "#!/bin/sh\ntouch {ready}\nwhile [ ! -e {go} ]; do sleep 0.01; done\nexec /bin/stty sane\n",
        ready = shell_quote(&ready), go = shell_quote(&go)),
    );
    write_exec(
        &bin.join("stty"),
        &format!(
            "#!/bin/sh\n/bin/stty -g > {modes}\nexec {probe} {reports} {after}\n",
            modes = shell_quote(&dispatch.root.join("restored-modes")),
            probe = shell_quote(session_probe()),
            reports = shell_quote(&reports),
            after = shell_quote(&after)
        ),
    );
    let path = std::env::join_paths(
        std::iter::once(bin).chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
    )
    .unwrap();
    (path, ready, go)
}

fn terminal_modes(terminal: &Pty) -> Vec<u8> {
    let output = Command::new("/bin/stty")
        .arg("-g")
        .stdin(Stdio::from(terminal.slave.try_clone().unwrap()))
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    saved_modes(output.stdout)
}

fn saved_modes(bytes: Vec<u8>) -> Vec<u8> {
    #[cfg(target_os = "macos")]
    {
        // XNU sets PENDIN when tcsetattr returns from raw to canonical mode.
        // This is queued-input state, not a saved mode; compare every other bit.
        // https://github.com/apple-oss-distributions/xnu/blob/main/bsd/kern/tty.c
        let text = String::from_utf8(bytes).unwrap();
        text.split(':')
            .map(|field| {
                if let Some(flags) = field.strip_prefix("lflag=") {
                    format!(
                        "lflag={:x}",
                        u64::from_str_radix(flags, 16).unwrap() & !libc::PENDIN
                    )
                } else {
                    field.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(":")
            .into_bytes()
    }
    #[cfg(not(target_os = "macos"))]
    bytes
}

#[test]
fn dispatch_death_recovers_the_terminal_while_its_raw_harness_survives() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.policy(&dispatch.terminal_policy(false, ""));
    dispatch.probe_then("trap '' HUP\n/bin/stty raw -echo\ntouch \"$record/raw\"\nexec sleep 60");
    let terminal = Pty::open();
    let modes = terminal_modes(&terminal);
    let (path, ready, go) = hold_terminal_reset(&dispatch);
    let mut command = command_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    command.env("PATH", path);
    let mut driver = DriverProcess::capture(command);
    driver.wait_for_ready(&dispatch.record(0).join("raw"));
    let harness = dispatch.probed(0);
    assert_eq!(harness.foreground, harness.group);
    assert_ne!(
        terminal_modes(&terminal),
        modes,
        "control: the harness must set raw mode"
    );
    let children = children_of(driver.id());
    let &[(front, _)] = children.as_slice() else {
        panic!("no dispatch child");
    };
    // Reap cleanup on failure too; this group survives dispatch's death.
    struct Orphan(i32);
    impl Drop for Orphan {
        fn drop(&mut self) {
            unsafe {
                libc::kill(-self.0, libc::SIGKILL);
            }
        }
    }
    let orphan = Orphan(harness.group);
    assert_eq!(unsafe { libc::kill(front, libc::SIGKILL) }, 0);
    driver.wait_for_ready(&ready);
    let restored = Probed::read(&dispatch.root.join("reset-reports/0/process"));
    assert_eq!(restored.group, driver.id());
    assert_eq!(restored.foreground, driver.id());
    assert_eq!(
        saved_modes(fs::read(dispatch.root.join("restored-modes")).unwrap()),
        modes
    );
    assert!(
        exists(harness.pid),
        "the foreground must be reclaimed while the orphan lives"
    );
    fs::write(go, "").unwrap();
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("harness may have survived"), "{stderr}");
    assert_eq!(dispatch.launch_count(), 1);
    assert!(dispatch.task_file("01-impl--subject-k1.md").is_file());
    drop(orphan);
}

#[test]
fn a_driver_term_cancels_dispatch_and_restores_the_terminal_after_reaping_the_harness() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.policy(&dispatch.terminal_policy(false, ""));
    dispatch.probe_then("trap '' TERM\n/bin/stty raw -echo\nsleep 60 &\nprintf '%s' \"$!\" > \"$record/descendant\"\ntouch \"$record/raw\"\nwhile :; do sleep 1; done");
    let terminal = Pty::open();
    let modes = terminal_modes(&terminal);
    let (path, ready, go) = hold_terminal_reset(&dispatch);
    let mut command = command_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    command.env("PATH", path);
    let mut driver = DriverProcess::capture(command);
    driver.wait_for_ready(&dispatch.record(0).join("raw"));
    let harness = dispatch.probed(0);
    let descendant = fs::read_to_string(dispatch.record(0).join("descendant"))
        .unwrap()
        .parse()
        .unwrap();
    assert_eq!(unsafe { libc::kill(driver.id(), libc::SIGTERM) }, 0);
    driver.wait_for_ready(&ready);
    let restored = Probed::read(&dispatch.root.join("reset-reports/0/process"));
    assert_eq!(restored.foreground, driver.id());
    assert_eq!(
        saved_modes(fs::read(dispatch.root.join("restored-modes")).unwrap()),
        modes
    );
    assert!(gone_within(harness.pid, Duration::from_secs(2)));
    assert!(gone_within(descendant, Duration::from_secs(2)));
    fs::write(go, "").unwrap();
    let output = driver.finish_within(SESSION_LIMIT);
    use std::os::unix::process::ExitStatusExt as _;
    assert_eq!(output.status.signal(), Some(libc::SIGTERM));
    assert!(String::from_utf8_lossy(&output.stderr).contains("interrupted by signal 15"));
    assert!(dispatch.task_file("01-impl--subject-k1.md").is_file());
}

#[test]
fn a_driver_started_in_the_background_takes_no_foreground() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    dispatch.policy(&dispatch.terminal_policy(false, ""));
    dispatch.probe_then("exit 0");
    let terminal = Pty::open();
    let modes = terminal_modes(&terminal);
    let reports = dispatch.root.join("owner-reports");
    fs::create_dir(&reports).unwrap();
    let mut command = command_on_at(
        Path::new("/bin/sh"),
        &dispatch.worktree,
        &dispatch.home,
        &terminal,
        PLAIN,
    );
    command.args([
        "-c",
        &format!(
            "set -m\n{grove} &\nwait $! || exit 91\n/bin/stty -g > {modes_file}\nexec {probe} {reports} /usr/bin/true",
            grove = shell_quote(Path::new(env!("CARGO_BIN_EXE_grove"))),
            probe = shell_quote(session_probe()),
            reports = shell_quote(&reports),
            modes_file = shell_quote(&dispatch.root.join("owner-modes"))
        ),
    ]);
    let mut owner = DriverProcess::capture(command);
    let output = owner.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    let restored = Probed::read(&reports.join("0/process"));
    assert_eq!(
        restored.foreground, restored.group,
        "the owner lost the foreground"
    );
    let harness = dispatch.probed(0);
    assert_eq!(harness.foreground, restored.group);
    assert_ne!(harness.foreground, harness.group);
    assert_eq!(
        saved_modes(fs::read(dispatch.root.join("owner-modes")).unwrap()),
        modes
    );
    assert!(dispatch.task_file("01-impl--subject-k1.md").is_file());
}

#[test]
fn a_stale_exit_signal_cannot_end_the_next_dispatch_launch() {
    let dispatch = Dispatch::new("worktree");
    plant_tree(&dispatch.worktree, "01-impl--subject-k1.md");
    let exit = shell_quote(&ravel());
    dispatch.harness_then(&format!(
        "if [ $n = 0 ]; then exec {exit} end-session grove-continue; fi\n\
         old=$(cat {launches}/0/channel)\n\
         if env RAVEL_EXIT_FILE=\"$old\" {exit} end-session grove-continue > \"$record/stale\" 2>&1; then exit 91; fi\n\
         printf stale-rejected > \"$record/stale-result\"\n\
         sleep 3\nexit 0", launches = shell_quote(&dispatch.launches)));
    support::route_every_kind_to(&dispatch.home, &dispatch.harness);
    let terminal = Pty::open();
    let mut driver = DriverProcess::spawn_on(&dispatch.worktree, &dispatch.home, &terminal, PLAIN);
    let output = driver.finish_within(SESSION_LIMIT);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert_eq!(dispatch.launch_count(), 2, "{stderr}");
    assert_ne!(dispatch.launch(0).channel, dispatch.launch(1).channel);
    assert!(
        fs::read_to_string(dispatch.record(1).join("stale-result")).unwrap() == "stale-rejected"
    );
    assert!(dispatch.task_file("01-impl--subject-k1.md").is_file());
    assert!(stderr.contains("loop stopped"), "{stderr}");
}

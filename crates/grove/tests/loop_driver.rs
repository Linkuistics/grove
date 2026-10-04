// Integration tests for the self-driving loop (src/loop_driver.rs).
//
// Every test here drives the **real bare `grove` process** against an isolated
// `$HOME` carrying a personal harness-dispatch policy, which the real
// `harness-dispatch` front and its compiled worker select a fake harness from.
// That is the only way in: there is no Grove configuration to write, no stand-in
// for dispatch, no binary override to point somewhere else, and no in-process
// entry point that skips the driver's signal handlers. The worker is not a cargo
// artifact: `task dispatch:worker` builds it, and without it the front refuses
// with exit 5, so these cases fail rather than skip. What the process seam buys is exactly what these tests are
// about — the driver's own stderr, its session-epoch bookkeeping, its response
// to being signalled, and its ownership of one foreground child.
//
// The watcher's grace → SIGTERM → kill-grace → SIGKILL escalation is *not*
// tested here. Its two durations are built-in constants passed into
// `wait_with_watcher_result`, so the escalation is driven on test timescales
// through that module-local parameter, in `src/loop_driver.rs`'s own unit tests.
// Reaching it from out here would need a process-configuration knob, which is
// the thing this leaf removed.

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

/// The bare driver, with the whole legacy launch-policy environment scrubbed
/// out of the child. A `Command` inherits this process's ambient environment,
/// and this repo dogfoods Grove, so scrubbing is what makes the fixture the
/// only input.
fn driver_command(worktree: &Path, home: &Path) -> Command {
    // The sibling this `grove` launches every session through.
    support::harness_dispatch();
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
    command.env("HOME", home);
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
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        "session-finished\n",
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
        let sessions = children_of(self.id());
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
            "#!/bin/sh\ncp \"$PWD/.jj/grove/session.epoch\" {epoch}\nprintf '%s\\n' \"$GROVE_SIGNAL_FILE\" > {signal}\nexit 0\n",
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
             printf 'relaunch\\n' > \"$GROVE_SIGNAL_FILE\"\n\
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
            "#!/bin/sh\nprintf 'ran\\n' >> {log}\nprintf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\nexit 0\n",
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
            "#!/bin/sh\nprintf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\nprintf '%s\\n' \"$GROVE_SIGNAL_FILE\" > {log}\nchmod 0500 \"$(dirname \"$GROVE_SIGNAL_FILE\")\"\nexit 0\n",
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
        "#!/bin/sh\nprintf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\nexec sleep 30\n",
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
        victim_stderr.contains("without a completion signal"),
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
             printf 'done\\n' > \"$GROVE_SIGNAL_FILE\"\n\
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
printf '%s\n' "$GROVE_SIGNAL_FILE" > {observed}
printf '%s\n' "$$" > {pid}
: > {ready}
while [ ! -e {held} ]; do sleep 0.01; done
: > "$GROVE_SIGNAL_FILE"
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
            .env("GROVE_SIGNAL_FILE", &signal_path)
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
    let signal_was_left_unconsumed = signal_path.as_ref().is_some_and(|path| path.exists());

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
            "post-reap session epoch invalidation blocked; completion signal left unconsumed"
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
// Grove launches every lifecycle session by running `harness-dispatch run`
// itself, and reads no configuration of its own
// (`docs/specs/harness-selection-and-execution.md`, *Grove integration*). These
// cases put the real front and its compiled worker between the real driver and
// a fake harness, under a temporary personal dispatch policy in the fixture's
// HOME, where the owner settings and the default record store live too. What
// they pin is the seam seen from Grove's side: what reaches the policy's
// `select`; that the authority to end the session reaches the harness and not
// the policy; and that a kind is asked about when its leaf launches and nowhere
// earlier.

/// The `harness-dispatch` front, whose worker `task dispatch:worker` builds
/// beside it. A missing or stale worker refuses with exit 5, so every case
/// here fails rather than skips without one.
fn harness_dispatch() -> PathBuf {
    support::harness_dispatch()
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
    /// `HARNESS_DISPATCH_RUN_ID`, or `<unset>`.
    run_id: String,
    /// `GROVE_SIGNAL_FILE`, or `<unset>`.
    channel: String,
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
        let file = self.home.join(".config/harness-dispatch/settings.json");
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
            run_id: fs::read_to_string(record.join("run-id")).unwrap(),
            channel: fs::read_to_string(record.join("channel")).unwrap(),
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

    /// `harness-dispatch record show` of one run, from the default store
    /// under this fixture's HOME.
    fn recorded(&self, run_id: &str) -> serde_json::Value {
        let output = Command::new(harness_dispatch())
            .env_clear()
            .env("HOME", &self.home)
            .args(["record", "show", "--run", run_id, "--json"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "run {run_id} is not recorded: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
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
    printf '%s' \"${HARNESS_DISPATCH_RUN_ID-<unset>}\" > \"$record/run-id\"\n\
    printf '%s' \"${GROVE_SIGNAL_FILE-<unset>}\" > \"$record/channel\"\n\
    cp .jj/grove/session.epoch \"$record/epoch\"\n";

/// `grove-llm complete --done`, the fake harness's usual last act. It passes
/// the session-epoch admission against the channel it was handed before it
/// writes, so a loop that ends on it shows the harness held the live channel.
fn complete_done() -> String {
    format!("exec {} complete --done", shell_quote(&own_grove_llm()))
}

/// What Grove passes `harness-dispatch run` for a lifecycle session, each a
/// flag joined to its value: the selection inputs, and no parameter.
const PASSED: [&str; 4] = ["--kind=", "--task-file=", "--task-id=", "--prompt="];

/// A dispatch policy whose `select` returns the fake harness for each of
/// `kinds`, and refuses any other kind as `incomplete_mapping`, its own code.
/// The harness's arguments are what `select` received: the kind, the task
/// file, the task identity and the prompt, then every parameter as one JSON
/// object, then the caller's directory.
///
/// At import it writes `view` with the names in the worker's environment and
/// in the environment of a child it spawns, and the worker's value of
/// `GROVE_SIGNAL_FILE`, so that what selection code can reach is observed
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
  channel: process.env.GROVE_SIGNAL_FILE ?? null,
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
    let channel = Path::new(channel).parent().unwrap().to_str().unwrap();
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
// places each in the harness's arguments whole, and the run record holds them.
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

    let recorded = dispatch.recorded(&launch.run_id);
    assert_eq!(recorded["runId"], launch.run_id.as_str(), "{recorded}");
    let fields = &recorded["launch"];
    assert_eq!(fields["kind"], "impl", "{recorded}");
    assert_eq!(fields["taskId"], "subject-k1", "{recorded}");
    assert_eq!(
        fields["taskFile"],
        task_file.to_str().unwrap(),
        "{recorded}"
    );
    assert_eq!(
        fields["candidate"]["program"],
        dispatch.harness.to_str().unwrap(),
        "{recorded}"
    );
    assert_eq!(fields["candidate"]["provider"], "origin-a", "{recorded}");
    assert_eq!(fields["params"], params, "{recorded}");

    assert!(
        epoch_names(&launch.epoch, &launch.channel),
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
    dispatch.settings(r#"{ "policyEnv": ["GROVE_SIGNAL_FILE"] }"#);
    dispatch.drive_to_completion();
    let granted = dispatch.launch(1);
    assert_eq!(dispatch.view()["channel"], granted.channel.as_str());
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
         exec {grove_llm} complete\n\
         fi\n\
         {complete}",
        authored = shell_quote(&authored),
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
        "session ended without a completion signal — status exit status: 3",
        "session kind `design` for `follow-up-k2` failed",
        "rerun `grove` to continue",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }

    // The refusal's own reproduction, run as its reader would run it, refuses
    // the same selection for the same task without launching anything.
    let line = stderr
        .lines()
        .find_map(|line| line.strip_prefix("  inspect: "))
        .unwrap_or_else(|| panic!("no inspect line: {stderr}"));
    for word in ["inspect", "design", "follow-up-k2"] {
        assert!(line.contains(word), "{line}");
    }
    assert!(
        !line.contains("--param"),
        "no parameter to reproduce: {line}"
    );
    assert!(line.contains(leaf.to_str().unwrap()), "{line}");
    let reproduced = Command::new("/bin/sh")
        .args(["-c", line])
        .env_clear()
        .env("HOME", &dispatch.home)
        .env("PATH", std::env::var_os("PATH").unwrap())
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&reproduced.stderr);
    assert_eq!(reproduced.status.code(), Some(3), "{line}\n{said}");
    for expected in [
        "refused (policy_refused, stage selection)",
        "  policy code: incomplete_mapping",
    ] {
        assert!(said.contains(expected), "no {expected:?} in: {said}");
    }
    assert_eq!(dispatch.launch_count(), 1);
    assert!(leaf.is_file(), "the refused leaf must still be live");

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
fn a_fresh_tree_is_scaffolded_with_no_policy_and_its_first_launch_refuses_naming_init() {
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
        "harness-dispatch init",
        "session ended without a completion signal",
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

// Grove finds `harness-dispatch` beside its own executable and nowhere else. A
// copy of `grove` with no sibling reports the path it looked at and that the
// two install together, before it scaffolds or launches anything. Every other
// case here is the control: the same driver, with its sibling, launches.
#[test]
fn a_missing_harness_dispatch_beside_grove_is_reported_with_its_path() {
    let dispatch = Dispatch::new("worktree");
    dispatch.policy(&probing_policy(
        &dispatch.view,
        &dispatch.harness,
        &["requirements"],
    ));
    let alone = dispatch.root.join("alone/bin");
    fs::create_dir_all(&alone).unwrap();
    fs::copy(env!("CARGO_BIN_EXE_grove"), alone.join("grove")).unwrap();

    let output = DriverProcess::capture(detached(driver_command_at(
        &alone.join("grove"),
        &dispatch.worktree,
        &dispatch.home,
    )))
    .finish();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    let missing = alone.canonicalize().unwrap().join("harness-dispatch");
    assert!(stderr.contains(missing.to_str().unwrap()), "{stderr}");
    assert!(
        stderr.contains("Install grove and harness-dispatch together"),
        "{stderr}"
    );
    assert!(!dispatch.worktree.join(".grove").exists(), "{stderr}");
    assert_eq!(dispatch.launch_count(), 0);
}

/// The Grove invocations `text` quotes: each from `harness-dispatch run
/// --kind=` to the end of its code span or its line, whichever comes first.
fn quoted_grove_invocations(text: &str) -> Vec<&str> {
    text.match_indices("harness-dispatch run --kind=")
        .map(|(at, _)| {
            let rest = &text[at..];
            &rest[..rest.find(['`', '\n']).unwrap_or(rest.len())]
        })
        .collect()
}

// The invocation an owner is shown is the one Grove makes. `harness-dispatch
// --help` and `run --help` each carry a Grove example, on one line, and each
// word of it after `run` is one of the flags the native-data case above shows
// reaching `select`, joined to a placeholder, in the driver's order. Any Grove
// invocation that dispatch's README, the usage guide or the configure-grove
// skill quotes is held to the same form. So a surface that drops the example,
// wraps it, or quotes another form fails here instead of drifting from what
// was tested.
#[test]
fn the_grove_invocation_dispatch_s_help_shows_is_the_one_the_driver_makes() {
    let help = |args: &[&str]| {
        let output = Command::new(harness_dispatch())
            .args(args)
            .output()
            .unwrap();
        assert!(output.status.success(), "{args:?}");
        String::from_utf8(output.stdout).unwrap()
    };
    let root = support::repo_root();
    let document = |relative: &str| fs::read_to_string(root.join(relative)).unwrap();
    let mut skill = String::new();
    let mut directories = vec![root.join("plugins/grove/skills/configure-grove")];
    while let Some(directory) = directories.pop() {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                directories.push(path);
            } else if path.extension().is_some_and(|extension| extension == "md") {
                skill += &fs::read_to_string(&path).unwrap();
            }
        }
    }
    let surfaces = [
        ("harness-dispatch --help", help(&["--help"]), true),
        (
            "harness-dispatch run --help",
            help(&["run", "--help"]),
            true,
        ),
        ("the configure-grove skill", skill, false),
        (
            "crates/harness-dispatch/README.md",
            document("crates/harness-dispatch/README.md"),
            false,
        ),
        ("docs/USAGE.md", document("docs/USAGE.md"), false),
    ];

    for (surface, text, required) in &surfaces {
        let quoted = quoted_grove_invocations(text);
        assert!(
            !required || !quoted.is_empty(),
            "{surface} quotes no Grove invocation of dispatch"
        );
        for command in quoted {
            let words: Vec<&str> = command.split_whitespace().collect();
            let flags: Vec<&str> = words[2..]
                .iter()
                .map(|word| &word[..word.rfind('=').map_or(0, |at| at + 1)])
                .collect();
            assert!(
                words[..2] == ["harness-dispatch", "run"] && flags == PASSED,
                "{surface} quotes {command:?}, whose inputs are not {PASSED:?}"
            );
        }
    }
}

// ---------------------------------------------------------------------------
// The original creator across Grove's lifecycle
//
// A review routed through the shipped Grove review example learns its
// producer's original creator from the `**Creator:**` line of its own task
// file (`docs/specs/harness-selection-and-execution.md`, *Identity and
// original creator*). Grove's code writes no such line. The methodology has
// the session that finishes a producer settle it, in the plugin's
// `references/retire.md`, *Naming your run on what you finish*: on every live
// review naming a producer it finished, by retiring its leaf or by closing a
// node, it writes its own `HARNESS_DISPATCH_RUN_ID`, or removes the line when
// it has none.
//
// These cases run fake sessions that follow that procedure behind the real
// driver and the real front and worker, under the example activated whole
// from a personal policy. A fake session reads its handle from the mandate,
// resolves it, and grows, retires and signals through `grove-llm`, as a
// session does. Whether a real session complies is the conformance rows'
// concern, not this seam's. What these pin is what a review's launch selects
// from once the tree has moved under its line.
//
// One session here does not follow the procedure, and is there because it
// does not: the first attempt of the no-run case names its run while its leaf
// is live, which the procedure forbids, to leave the stale line a finishing
// session must remove.

/// A fake session's own procedure, after [`RECORD_START`]: which leaf it was
/// launched for, and the methodology's steps as functions for its step to
/// call.
///
/// `name_run` is *Naming your run on what you finish* for one finished
/// producer, `$1`: on every live review leaf whose `**Reviews:**` line names
/// it, the line under that one becomes this session's run, or goes. `finish`
/// retires the session's leaf and walks the close cascade: each ancestor node
/// the retirement left with no live leaf is a producer this session finished
/// too, innermost first. `signal` ends the session through `grove-llm
/// complete`, first leaving every creator line in the tree where a case can
/// read what stood when the session ended.
const SESSION: &str = r#"for prompt; do :; done
handle=${prompt#*"$marker"}
handle=${handle%%'`'*}
printf '%s' "$handle" > "$record/handle"
leaf=$("$grove_llm" resolve "$handle")
[ -f "$leaf" ] || exit 93
name_run() {
  for review in $(grep -rlx --include='[0-9]*.md' "\*\*Reviews:\*\* $1" .grove); do
    case ${review##*/} in [0-9]*-DONE-* | [0-9]*-ABANDONED-*) continue ;; esac
    awk -v run="${HARNESS_DISPATCH_RUN_ID-}" '
      /^\*\*Creator:\*\*/ { next }
      { print }
      /^\*\*Reviews:\*\* / && run != "" { print "**Creator:** run " run }
    ' "$review" > "$record/settled" && cat "$record/settled" > "$review" || exit 91
  done
}
finish() {
  "$grove_llm" leaf-retire "$leaf" > /dev/null || exit 92
  name_run "$handle"
  node=${leaf%/*}
  while [ "${node##*/}" != .grove ]; do
    live=$(find "$node" -type f -name '[0-9]*.md' ! -name '[0-9]*-DONE-*' ! -name '[0-9]*-ABANDONED-*')
    [ -z "$live" ] || break
    brief=$(basename "$node"/_*.md .md)
    name_run "${brief#_}-${node##*-}"
    node=${node%/*}
  done
}
signal() {
  grep -rh '^\*\*Creator:\*\*' .grove > "$record/creators"
  exec "$grove_llm" complete "$@"
}
"#;

/// The shipped example, whole, as an owner's personal policy activates it.
const REVIEW_EXAMPLE: &str = "export { policy } from \"harness-dispatch/examples/grove-review\";\n";

/// The owner's later mapping: the lead at `high`, which the example runs for
/// `impl` and gives an `anthropic` creator's `review-impl`, is now the other
/// provider's harness. Read from this policy, an `impl` creator is
/// `anthropic`, whose reviewer is that same command, of that same origin, so
/// the review would refuse.
const REMAPPED: &str = r#"import { reviews, routes, groveReviewSelector } from "harness-dispatch/examples/grove-review";
import { review } from "harness-dispatch/examples/grove-static";
const today = review("high");
export const policy = {
  schemaVersion: 2,
  version: "remapped-1",
  ...groveReviewSelector({
    routes: { ...routes, impl: today },
    reviews: { ...reviews, "review-impl": { ...reviews["review-impl"], anthropic: today } },
  }),
};
"#;

/// The planted review's body, with `creator` as its `**Creator:**` line or
/// with none. A fake session's `name_run` writes and removes exactly that
/// line, so each state of the leaf is one of these, byte for byte.
fn review_body(creator: Option<&str>) -> String {
    review_leaf("parser-k2", "parser-k1", creator)
}

/// The body of the review leaf `own`, which reviews `reviewed`, in the shape
/// of [`review_body`].
fn review_leaf(own: &str, reviewed: &str, creator: Option<&str>) -> String {
    let creator = creator.map(|line| format!("{line}\n")).unwrap_or_default();
    format!("# {own}\n\n**Reviews:** {reviewed}\n{creator}\n## Goal\n\nReview the parser.\n")
}

/// A `**Creator:**` line naming `run_id`, as a dispatched session writes it.
fn creator_run(run_id: &str) -> String {
    format!("**Creator:** run {run_id}")
}

/// The first four arguments the example's wrappers receive for `model` at
/// `high`: which harness ran, at what effort. The prompt follows them.
fn selected(model: &str) -> [OsString; 4] {
    ["--model", model, "--effort", "high"].map(OsString::from)
}

/// A fixture for the creator lifecycle: a [`Dispatch`] whose tree holds the
/// producer `parser-k1` and, cut before it ran, its review `parser-k2`, and
/// whose personal policy is the shipped Grove review example.
struct Lifecycle {
    dispatch: Dispatch,
    /// The driver's `PATH`: the two wrapper programs the example's commands
    /// run, each of which execs the fake harness, then the ambient one.
    path: OsString,
}

impl Lifecycle {
    /// The planted producer and its pre-cut review, at their first positions.
    const PRODUCER: &str = "01-impl--parser-k1.md";
    const REVIEW: &str = "02-review-impl--parser-k2.md";

    /// A fixture whose fake sessions do `steps`: the arms of a shell `case`
    /// on the session's handle, each ending in `signal`, which relaunches, or
    /// `signal --done`, which returns control to the case.
    fn new(steps: &str) -> Self {
        let dispatch = Dispatch::new("worktree");
        let grove = dispatch.worktree.join(".grove");
        fs::create_dir_all(&grove).unwrap();
        fs::write(grove.join("_BRIEF.md"), "# g — brief\n").unwrap();
        fs::write(
            grove.join(Self::PRODUCER),
            "# parser-k1\n\n## Goal\n\nBuild the parser.\n",
        )
        .unwrap();
        fs::write(grove.join(Self::REVIEW), review_body(None)).unwrap();

        let bin = dispatch.root.join("bin");
        fs::create_dir(&bin).unwrap();
        for wrapper in ["my-codex-wrapper", "my-claude-wrapper"] {
            write_exec(
                &bin.join(wrapper),
                &format!(
                    "#!/bin/sh\nexec {} \"$@\"\n",
                    shell_quote(&dispatch.harness)
                ),
            );
        }
        let ambient = std::env::var_os("PATH").unwrap();
        let path =
            std::env::join_paths(std::iter::once(bin).chain(std::env::split_paths(&ambient)))
                .unwrap();

        dispatch.policy(REVIEW_EXAMPLE);
        dispatch.harness_then(&format!(
            "grove_llm={grove_llm}\n\
             marker='{MANDATED_LEAF}'\n\
             {SESSION}\
             case $handle in\n\
             {steps}\n\
             *) exit 94 ;;\n\
             esac",
            grove_llm = shell_quote(&own_grove_llm()),
        ));
        Lifecycle { dispatch, path }
    }

    /// Run the loop until it stops, returning what the driver said.
    fn drive(&self) -> String {
        let mut driver = grove_driver(&self.dispatch.worktree, &self.dispatch.home);
        driver.env("PATH", &self.path);
        let output = DriverProcess::capture(driver).finish_within(SESSION_LIMIT);
        let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
        assert!(output.status.success(), "{stderr}");
        stderr
    }

    /// [`Lifecycle::drive`], until a session signals `done`.
    fn drive_to_done(&self) {
        let stderr = self.drive();
        assert!(
            stderr.contains("grove finished — loop complete"),
            "the loop did not end on a session's `done`: {stderr}"
        );
    }

    /// The handle start `n` read from its mandate.
    fn handle(&self, n: usize) -> String {
        fs::read_to_string(self.dispatch.record(n).join("handle")).unwrap()
    }

    /// Every `**Creator:**` line in the tree as start `n`'s session ended.
    fn creators(&self, n: usize) -> String {
        fs::read_to_string(self.dispatch.record(n).join("creators")).unwrap()
    }

    /// The body of the review leaf, which must be at `leaf` under `.grove/`.
    fn review(&self, leaf: &str) -> String {
        let path = self.dispatch.task_file(leaf);
        fs::read_to_string(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
    }

    /// `harness-dispatch` as the owner runs it by hand in the working tree.
    fn harness_dispatch(&self) -> Command {
        let mut command = Command::new(harness_dispatch());
        command
            .env_clear()
            .env("HOME", &self.dispatch.home)
            .env("PATH", &self.path)
            .current_dir(&self.dispatch.worktree);
        command
    }

    /// Inspect the review at `leaf`, as its refused launch's `inspect:` line
    /// does, with any `extra` words.
    fn inspect(&self, leaf: &str, extra: &[&str]) -> Output {
        self.harness_dispatch()
            .args(["inspect", "--kind", "review-impl", "--task-file"])
            .arg(self.dispatch.task_file(leaf))
            .args(["--task-id", "parser-k2"])
            .args(extra)
            .output()
            .unwrap()
    }
}

// Retirement and reordering move the producer's file and the review's, and
// the review's creator is neither: it is the run the review's own line names.
// The dispatched producer finishes, so it writes its run on the review cut
// before it ran, retires, and inserts new work ahead of that review, which
// moves the review to another position. The owner then points the lead at
// `high` at the other provider's harness. The inserted leaf launches under
// that mapping and records it, which is the control that it was live. The
// review, at its new path, still selects from the provider recorded for the
// producer's run: the reviewer is the one the example gives an `openai`
// creator, where today's policy would have refused the review as same-origin.
#[test]
fn retiring_and_reordering_a_producer_leaves_its_review_s_creator_unchanged() {
    let lifecycle = Lifecycle::new(
        "parser-k1)\n\
         finish\n\
         \"$grove_llm\" leaf-insert --kind impl parser-k2 groundwork > /dev/null || exit 95\n\
         signal --done ;;\n\
         groundwork-k3) finish; signal ;;\n\
         parser-k2) signal --done ;;",
    );
    let dispatch = &lifecycle.dispatch;

    lifecycle.drive_to_done();
    assert_eq!(lifecycle.handle(0), "parser-k1");
    let producer = dispatch.launch(0);
    assert_eq!(producer.args[..4], selected("your-codex-model"));
    assert!(dispatch.task_file("01-DONE-impl--parser-k1.md").is_file());
    assert!(dispatch.task_file("02-impl--groundwork-k3.md").is_file());
    let named = review_body(Some(&creator_run(&producer.run_id)));
    assert_eq!(lifecycle.review("03-review-impl--parser-k2.md"), named);

    dispatch.policy(REMAPPED);
    lifecycle.drive_to_done();
    assert_eq!(dispatch.launch_count(), 3);

    // The control: the inserted leaf ran under today's mapping.
    assert_eq!(lifecycle.handle(1), "groundwork-k3");
    let inserted = dispatch.launch(1);
    assert_eq!(inserted.args[..4], selected("your-claude-model"));
    let today = dispatch.recorded(&inserted.run_id);
    assert_eq!(
        today["launch"]["candidate"]["program"], "my-claude-wrapper",
        "{today}"
    );
    assert_eq!(
        today["launch"]["candidate"]["provider"], "anthropic",
        "{today}"
    );

    assert_eq!(lifecycle.handle(2), "parser-k2");
    let reviewer = dispatch.launch(2);
    assert_eq!(reviewer.args[..4], selected("your-claude-model"));
    assert_eq!(
        lifecycle.review("03-review-impl--parser-k2.md"),
        named,
        "no later session finished parser-k1, so none may touch its line"
    );
    let recorded = dispatch.recorded(&reviewer.run_id);
    let launch = &recorded["launch"];
    assert_eq!(launch["policy"]["version"], "remapped-1", "{recorded}");
    assert_eq!(launch["taskId"], "parser-k2", "{recorded}");
    assert_eq!(
        launch["taskFile"],
        dispatch
            .task_file("03-review-impl--parser-k2.md")
            .to_str()
            .unwrap(),
        "{recorded}"
    );
    assert_eq!(launch["candidate"]["provider"], "anthropic", "{recorded}");
    assert_eq!(
        launch["reviewedArtifact"],
        serde_json::json!({ "id": "parser-k1", "creator": { "run": producer.run_id } }),
        "{recorded}"
    );
    let creator = &launch["creator"];
    assert_eq!(creator["evidence"], "execution_recorded", "{recorded}");
    assert_eq!(creator["provider"], "openai", "{recorded}");
    let lookup = &creator["lookup"];
    assert_eq!(lookup["taskId"], "parser-k1", "{recorded}");
    assert_eq!(
        (&lookup["provider"], &lookup["model"], &lookup["effort"]),
        (
            &serde_json::json!("openai"),
            &serde_json::json!("your-codex-model"),
            &serde_json::json!("high"),
        ),
        "{recorded}"
    );

    // The control: a creator read from today's policy is `anthropic`, and
    // the review of one refuses as same-origin.
    let declared = review_body(Some("**Creator:** declared anthropic"));
    let leaf = dispatch.root.join("declared-review.md");
    fs::write(&leaf, declared).unwrap();
    let refused = lifecycle
        .harness_dispatch()
        .args(["inspect", "--kind", "review-impl", "--task-file"])
        .arg(&leaf)
        .output()
        .unwrap();
    let said = String::from_utf8_lossy(&refused.stderr);
    assert_eq!(refused.status.code(), Some(3), "{said}");
    assert!(said.contains("  policy code: same_origin"), "{said}");
}

// A producer that proves too big decomposes, and the review cut before it did
// still names its handle, now a node's. Nothing finishes that producer until
// a retirement leaves the node with no live leaf. Here `parser-k1` becomes a
// node holding `lexer-k3` and `grammar-k4`, and `grammar-k4` a node in turn,
// holding `tokens-k5`. Retiring `lexer-k3` closes nothing, so the review still
// has no line. Retiring `tokens-k5` closes `grammar-k4` and, through it,
// `parser-k1`, in one cascade, so that session names its run on the review.
// The review then selects from that run, whose task identity is the child's:
// launch compares no handles, and inspection shows the two side by side.
#[test]
fn a_decomposed_producer_s_review_carries_the_run_whose_retirement_closed_its_node() {
    let lifecycle = Lifecycle::new(
        "parser-k1)\n\
         \"$grove_llm\" leaf-decompose \"$leaf\" lexer > /dev/null || exit 95\n\
         \"$grove_llm\" leaf-add parser-k1 grammar --kind impl > /dev/null || exit 95\n\
         signal ;;\n\
         lexer-k3) finish; signal ;;\n\
         grammar-k4)\n\
         \"$grove_llm\" leaf-decompose \"$leaf\" tokens > /dev/null || exit 95\n\
         signal ;;\n\
         tokens-k5) finish; signal ;;\n\
         parser-k2) signal --done ;;",
    );
    let dispatch = &lifecycle.dispatch;

    lifecycle.drive_to_done();
    let handles: Vec<String> = (0..dispatch.launch_count())
        .map(|n| lifecycle.handle(n))
        .collect();
    assert_eq!(
        handles,
        [
            "parser-k1",
            "lexer-k3",
            "grammar-k4",
            "tokens-k5",
            "parser-k2"
        ]
    );
    assert!(dispatch
        .task_file("01-k1/02-k4/01-DONE-impl--tokens-k5.md")
        .is_file());

    // No session before the one that closed the node named a run, the one
    // that retired a leaf under it included.
    for (n, handle) in handles.iter().enumerate().take(3) {
        assert_eq!(lifecycle.creators(n), "", "after {handle}");
    }
    let closer = dispatch.launch(3);
    assert_eq!(
        lifecycle.creators(3),
        format!("{}\n", creator_run(&closer.run_id))
    );
    assert_eq!(
        lifecycle.review(Lifecycle::REVIEW),
        review_body(Some(&creator_run(&closer.run_id)))
    );

    let reviewer = dispatch.launch(4);
    assert_eq!(reviewer.args[..4], selected("your-claude-model"));
    let recorded = dispatch.recorded(&reviewer.run_id);
    let launch = &recorded["launch"];
    assert_eq!(launch["reviewedArtifact"]["id"], "parser-k1", "{recorded}");
    assert_eq!(
        launch["creator"]["reference"],
        serde_json::json!({ "run": closer.run_id }),
        "{recorded}"
    );
    assert_eq!(
        launch["creator"]["lookup"]["taskId"], "tokens-k5",
        "{recorded}"
    );

    // Inspection shows the reviewed handle and the named run's task identity,
    // in both forms.
    let inspected = lifecycle.inspect(Lifecycle::REVIEW, &["--json"]);
    assert!(
        inspected.status.success(),
        "{}",
        String::from_utf8_lossy(&inspected.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&inspected.stdout).unwrap();
    assert_eq!(report["reviewedArtifact"]["id"], "parser-k1", "{report}");
    assert_eq!(
        report["creator"]["lookup"]["taskId"], "tokens-k5",
        "{report}"
    );
    let inspected = lifecycle.inspect(Lifecycle::REVIEW, &[]);
    let text = String::from_utf8_lossy(&inspected.stdout);
    for shown in ["parser-k1", "tokens-k5", closer.run_id.as_str()] {
        assert!(text.contains(shown), "no {shown:?} in:\n{text}");
    }
}

// The step's set is every live review of every producer the session finished,
// wherever the review sits, and no other file. Each case above holds one
// review, at the grove root, of one producer, so none of them can tell that
// set from a part of it or from more than it. This tree holds the rest. A
// second live review of `parser-k1` sits inside another node. The session
// whose retirement closes `grammar-k19` and, through it, `parser-k1` cuts the
// inner node's review itself, and writes its body, before it finishes. Two
// reviews of `parser-k1` are terminal, one done and one abandoned, each
// holding the line it had. A live review of another producer, `parser-k17`,
// names a handle that `parser-k1` begins, and ends a line of its prose with
// `parser-k1`'s own line.
//
// The closing session's run goes on the three live reviews of the two nodes
// it closed, and nowhere else. Both reviews of `parser-k1` then launch and
// select from that run, the nested one included.
#[test]
fn a_close_cascade_settles_every_live_review_of_each_producer_it_finishes_and_no_other() {
    const NESTED: &str = "03-k3/01-review-impl--parser-k4.md";
    const DONE: &str = "04-DONE-review-impl--parser-k5.md";
    const ABANDONED: &str = "05-ABANDONED-review-impl--parser-k6.md";
    const UNRELATED: &str = "07-review-impl--parser-k8.md";
    const INNER: &str = "08-review-impl--grammar-k21.md";
    const EARLIER: &str = "**Creator:** run 00000000-0000-4000-8000-000000000001";
    const DECLARED: &str = "**Creator:** declared anthropic";

    let lifecycle = Lifecycle::new(&format!(
        "parser-k1)\n\
         \"$grove_llm\" leaf-decompose \"$leaf\" lexer > /dev/null || exit 95\n\
         \"$grove_llm\" leaf-add parser-k1 grammar --kind impl > /dev/null || exit 95\n\
         signal ;;\n\
         lexer-k18) finish; signal ;;\n\
         grammar-k19)\n\
         \"$grove_llm\" leaf-decompose \"$leaf\" tokens > /dev/null || exit 95\n\
         signal ;;\n\
         tokens-k20)\n\
         cut=$(\"$grove_llm\" leaf-add . grammar --kind review-impl) || exit 95\n\
         printf '{inner}' > \"$cut\"\n\
         finish; signal ;;\n\
         parser-k2) finish; signal ;;\n\
         parser-k4) signal --done ;;",
        inner = review_leaf("grammar-k21", "grammar-k19", None).replace('\n', "\\n"),
    ));
    let dispatch = &lifecycle.dispatch;
    let nested = |creator| review_leaf("parser-k4", "parser-k1", creator);
    let done = review_leaf("parser-k5", "parser-k1", Some(EARLIER));
    let abandoned = review_leaf("parser-k6", "parser-k1", Some(DECLARED));
    let unrelated = format!(
        "{}\nIts line is not **Reviews:** parser-k1\n",
        review_leaf("parser-k8", "parser-k17", Some(DECLARED))
    );
    fs::create_dir(dispatch.task_file("03-k3")).unwrap();
    for (leaf, body) in [
        ("03-k3/_audit.md", "# audit-k3 — brief\n"),
        (NESTED, nested(None).as_str()),
        (DONE, done.as_str()),
        (ABANDONED, abandoned.as_str()),
        ("06-DONE-impl--parser-k17.md", "# parser-k17\n"),
        (UNRELATED, unrelated.as_str()),
    ] {
        fs::write(dispatch.task_file(leaf), body).unwrap();
    }
    let sorted = |lines: &[&str]| {
        let mut lines: Vec<String> = lines.iter().map(|line| line.to_string()).collect();
        lines.sort();
        lines
    };
    let creators = |n| {
        let recorded = lifecycle.creators(n);
        sorted(&recorded.lines().collect::<Vec<_>>())
    };

    lifecycle.drive_to_done();
    let handles: Vec<String> = (0..dispatch.launch_count())
        .map(|n| lifecycle.handle(n))
        .collect();
    assert_eq!(
        handles,
        [
            "parser-k1",
            "lexer-k18",
            "grammar-k19",
            "tokens-k20",
            "parser-k2",
            "parser-k4"
        ]
    );

    // Until the cascade closed a node, the tree held the planted lines only.
    for (n, handle) in handles.iter().enumerate().take(3) {
        assert_eq!(
            creators(n),
            sorted(&[EARLIER, DECLARED, DECLARED]),
            "after {handle}"
        );
    }
    let closer = dispatch.launch(3);
    let named = creator_run(&closer.run_id);
    assert_eq!(
        creators(3),
        sorted(&[EARLIER, DECLARED, DECLARED, &named, &named, &named]),
        "the closing session names its run on three reviews and no more"
    );

    // Every live review of the outer node, wherever it sits.
    assert_eq!(
        lifecycle.review("02-DONE-review-impl--parser-k2.md"),
        review_body(Some(&named))
    );
    assert_eq!(lifecycle.review(NESTED), nested(Some(&named)));
    // The inner node is a producer the cascade finished too, and its review is
    // the one this session cut.
    assert_eq!(
        lifecycle.review(INNER),
        review_leaf("grammar-k21", "grammar-k19", Some(&named))
    );
    // No terminal review, and no review of another producer.
    assert_eq!(lifecycle.review(DONE), done);
    assert_eq!(lifecycle.review(ABANDONED), abandoned);
    assert_eq!(lifecycle.review(UNRELATED), unrelated);

    // Both reviews of `parser-k1` launched from the closing session's run.
    for (n, task) in [(4, "parser-k2"), (5, "parser-k4")] {
        let reviewer = dispatch.launch(n);
        assert_eq!(reviewer.args[..4], selected("your-claude-model"), "{task}");
        let recorded = dispatch.recorded(&reviewer.run_id);
        let launch = &recorded["launch"];
        assert_eq!(launch["taskId"], task, "{recorded}");
        assert_eq!(
            launch["reviewedArtifact"],
            serde_json::json!({ "id": "parser-k1", "creator": { "run": closer.run_id } }),
            "{recorded}"
        );
        assert_eq!(
            launch["creator"]["lookup"]["taskId"], "tokens-k20",
            "{recorded}"
        );
    }
}

// A producer's earlier attempt is not its creator, whatever line it left. The
// stale line is planted by a fault: the dispatched attempt names its run on
// the review while its leaf is live, then dies. No session that follows the
// procedure does that, because it retires first and a leaf left live writes
// nothing. An attempt that wrote nothing would give the finish nothing to
// remove, and this case would pass with a fake that never removes. The next
// session has no run to name: its harness drops `HARNESS_DISPATCH_RUN_ID`
// before it works, as one started by hand in the working tree has none. It
// does follow the procedure, so it finishes the producer and removes the line
// it found. The review that follows in the same loop refuses, though runs of
// the producer's task are in the store, and its remedy is the declaration.
// The owner declares the provider that finished the artifact, and the review
// launches on the other one. Had the attempt's line survived, its `openai`
// run would have selected the `anthropic` reviewer: the provider declared to
// have finished the artifact.
#[test]
fn a_finish_by_a_session_with_no_run_removes_an_attempt_s_run_and_the_review_refuses_until_declared(
) {
    let lifecycle = Lifecycle::new(
        "parser-k1)\n\
         if [ $n = 0 ]; then name_run \"$handle\"; exit 0; fi\n\
         unset HARNESS_DISPATCH_RUN_ID\n\
         finish; signal ;;\n\
         parser-k2) signal --done ;;",
    );
    let dispatch = &lifecycle.dispatch;

    let stderr = lifecycle.drive();
    assert!(
        stderr.contains("session ended without a completion signal"),
        "{stderr}"
    );
    let attempt = dispatch.launch(0);
    assert_eq!(attempt.args[..4], selected("your-codex-model"));
    assert!(dispatch.task_file(Lifecycle::PRODUCER).is_file());
    assert_eq!(
        lifecycle.review(Lifecycle::REVIEW),
        review_body(Some(&creator_run(&attempt.run_id))),
        "the attempt's line must be there for the finish to remove"
    );

    let stderr = lifecycle.drive();
    assert_eq!(lifecycle.handle(1), "parser-k1");
    assert!(dispatch.task_file("01-DONE-impl--parser-k1.md").is_file());
    assert_eq!(lifecycle.creators(1), "");
    assert_eq!(lifecycle.review(Lifecycle::REVIEW), review_body(None));
    assert_eq!(
        dispatch.launch_count(),
        2,
        "the refused review must reach no harness: {stderr}"
    );
    for said in [
        "refused (policy_refused, stage context)",
        "creator_line_missing",
        "reviews \"parser-k1\" but has no **Creator:** line",
        "If parser-k1 was finished without harness-dispatch, declare its origin: \
         directly under the **Reviews:** line, write \"**Creator:** declared <origin>\"",
        "session kind `review-impl` for `parser-k2` failed",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    // The attempt's run of the producer's task is recorded, and stood in for
    // nothing.
    assert_eq!(
        dispatch.recorded(&attempt.run_id)["launch"]["taskId"],
        "parser-k1"
    );
    let refused = lifecycle.inspect(Lifecycle::REVIEW, &[]);
    assert_eq!(refused.status.code(), Some(3));

    // The owner's declaration: the artifact was finished by `anthropic`.
    fs::write(
        dispatch.task_file(Lifecycle::REVIEW),
        review_body(Some("**Creator:** declared anthropic")),
    )
    .unwrap();
    lifecycle.drive_to_done();
    assert_eq!(lifecycle.handle(2), "parser-k2");
    let reviewer = dispatch.launch(2);
    assert_eq!(reviewer.args[..4], selected("your-codex-model"));
    let recorded = dispatch.recorded(&reviewer.run_id);
    let launch = &recorded["launch"];
    assert_eq!(
        launch["candidate"]["program"], "my-codex-wrapper",
        "{recorded}"
    );
    assert_eq!(launch["candidate"]["provider"], "openai", "{recorded}");
    assert_eq!(
        launch["creator"],
        serde_json::json!({
            "reference": { "declared": "anthropic" },
            "evidence": "declared",
            "provider": "anthropic",
            "lookup": null,
        }),
        "{recorded}"
    );
}

// A review knows its producer's run from its own task file, and that is all
// it needs to attach what it found. The run and its observations live in the
// record store, outside the tree, so they outlast it.
//
// This is also the ordinary chain, where no review waits beforehand: the
// producer cuts its review as its last act, writes the body, and names its
// run on the leaf it cut. The review session reads the run from that line.
// After `.grove/` is gone, as teardown leaves it, an observation of that run
// is imported and `record show` returns it beside the producer's launch
// fields.
#[test]
fn a_review_s_findings_attach_to_the_producer_s_run_after_the_tree_is_removed() {
    let lifecycle = Lifecycle::new(&format!(
        "parser-k1)\n\
         cut=$(\"$grove_llm\" leaf-add . parser --kind review-impl) || exit 95\n\
         printf '{body}' > \"$cut\"\n\
         finish; signal ;;\n\
         parser-k2)\n\
         sed -n 's/^\\*\\*Creator:\\*\\* run //p' \"$leaf\" > \"$record/creator\"\n\
         signal --done ;;",
        body = review_body(None).replace('\n', "\\n"),
    ));
    let dispatch = &lifecycle.dispatch;
    fs::remove_file(dispatch.task_file(Lifecycle::REVIEW)).unwrap();

    lifecycle.drive_to_done();
    let producer = dispatch.launch(0);
    let reviewer = dispatch.launch(1);
    assert_eq!(
        lifecycle.review(Lifecycle::REVIEW),
        review_body(Some(&creator_run(&producer.run_id))),
        "the producer names its run on the review it cut"
    );
    let named = fs::read_to_string(dispatch.record(1).join("creator")).unwrap();
    assert_eq!(named, format!("{}\n", producer.run_id));
    let run_id = named.trim_end();

    fs::remove_dir_all(dispatch.worktree.join(".grove")).unwrap();
    // The control: nothing but dispatch's own end observation has observed the
    // producer's run yet, and it measured no findings.
    let before = dispatch.recorded(run_id);
    assert_eq!(
        before["observations"].as_array().map(Vec::len),
        Some(1),
        "{before}"
    );
    assert_eq!(
        before["observations"][0]["source"], "harness-dispatch",
        "{before}"
    );
    assert_eq!(
        before["measurements"]["missedDefects"]["state"], "unobserved",
        "{before}"
    );

    let observation = serde_json::json!({
        "schemaVersion": 1,
        "observationId": "parser-k2-findings",
        "runId": run_id,
        "source": format!("review-impl session parser-k2, run {}", reviewer.run_id),
        "observedAt": "2026-10-01T09:30:00Z",
        "evidence": "the review's findings",
        "measurements": {
            "acceptance": { "state": "observed", "value": "rejected" },
            "missedDefects": { "state": "observed", "value": [{ "id": "F1", "summary": "unbounded read" }] },
        },
    });
    let document = dispatch.root.join("observation.json");
    fs::write(&document, observation.to_string()).unwrap();
    let observed = lifecycle
        .harness_dispatch()
        .args(["record", "observe", "--run", run_id, "--file"])
        .arg(&document)
        .output()
        .unwrap();
    assert!(
        observed.status.success(),
        "{}",
        String::from_utf8_lossy(&observed.stderr)
    );

    let export = dispatch.recorded(run_id);
    assert_eq!(export["launch"]["taskId"], "parser-k1", "{export}");
    assert_eq!(
        export["observations"][1]["observationId"], "parser-k2-findings",
        "{export}"
    );
    assert_eq!(
        export["measurements"]["missedDefects"]["current"][0]["value"][0]["id"], "F1",
        "{export}"
    );
    assert_eq!(
        export["measurements"]["acceptance"]["state"], "observed",
        "{export}"
    );
}

// ---------------------------------------------------------------------------
// The launch boundary under a controlling terminal
//
// A human starts Grove from a shell, so the driver owns a terminal and leads
// its foreground group, and keyed-launch hands that terminal to each session
// it launches (`docs/adr/the-launched-child-is-a-job.md`). Every case above
// detaches its driver instead. These start it on a pseudo-terminal of their
// own, as the leader of a session whose controlling terminal it is, and the
// master side stands for the human: writing the interrupt character to it is
// a typed Ctrl-C, and what it reads is what the human would see. The driver's
// streams go to files, which the harness inherits, so the terminal is its
// stdin and its controlling terminal; the one case about what the terminal
// shows puts the driver's streams on it as a shell does.
//
// The harness is `session-probe` (`tests/support/session-probe.c`), which
// reports the process exec made it and then execs a shell step that records
// the rest and does the case's action. What the harness is handed is what
// Grove hands the process it spawns: std keeps the mask and resets SIGPIPE at
// the spawn, and keyed-launch resets seven terminal signals. Dispatch, which
// is that process and spawns the harness as its own child, hands it that entry
// state and must add nothing to it.

/// The longest a case's loop may run before the case fails rather than hangs.
const SESSION_LIMIT: Duration = Duration::from_secs(60);

/// Ctrl-C.
const INTERRUPT: u8 = 0x03;

/// Every signal number either platform uses.
const LAST_SIGNAL: libc::c_int = 64;

/// A pseudo-terminal for one driver to take as its controlling terminal.
///
/// One per driver run: on macOS a session leader's exit revokes its
/// controlling terminal, so a terminal outlives the first driver that owned it
/// only as a dead descriptor. The master is held for as long as the terminal
/// is, since closing it hangs the terminal up, and is read throughout.
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
    {
        let mut command = driver_command(worktree, home);
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

    /// The default record store under this fixture's HOME.
    fn store(&self) -> PathBuf {
        self.home
            .join(".local/state/harness-dispatch/records.sqlite3")
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
  channel: process.env.GROVE_SIGNAL_FILE ?? null,
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
    /// ended without a completion signal, doing `meanwhile` to the running
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
            .split_once("session ended without a completion signal — status ")
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
    assert_ne!(launch.run_id, "<unset>");
    assert!(
        epoch_names(&launch.epoch, &launch.channel),
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
        !names
            .iter()
            .any(|name| name.starts_with("GROVE_") || name.starts_with("HARNESS_DISPATCH_")),
        "the worker holds a control variable: {view}"
    );
    assert_eq!(view["channel"], serde_json::Value::Null, "{view}");

    // The controls.
    dispatch.policy(&dispatch.terminal_policy(true, ""));
    dispatch.settings(r#"{ "policyEnv": ["GROVE_SIGNAL_FILE"] }"#);
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
    assert_eq!(dispatch.view()["channel"], granted.channel.as_str());
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
        assert_eq!(status, reported, "`{ending}`: {stderr}");
        assert!(!stderr.contains("refused ("), "{stderr}");
        assert_ne!(
            dispatch.launch(n).run_id,
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
    let refusal = at("harness-dispatch: refused (policy_refused, stage selection)");
    for said in [
        "  policy code: incomplete_mapping",
        "  remedy: add the kind to the policy",
        "  inspect: ",
    ] {
        assert!(at(said) > refusal, "{shown}");
    }
    assert!(
        at("session kind `impl` for `subject-k1` failed") > at("  inspect: "),
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
        "refused (selection_cancelled, stage evaluation)",
        "signal: SIGINT",
        "session kind `impl` for `subject-k1` failed",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    assert_eq!(dispatch.launch_count(), 0, "a harness launched: {stderr}");
    let pid: libc::pid_t = fs::read_to_string(&worker).unwrap().trim().parse().unwrap();
    assert!(!exists(pid), "worker {pid} outlived its selection");
    assert!(!dispatch.store().exists(), "a run was recorded: {stderr}");

    dispatch.policy(&dispatch.terminal_policy(false, &holding(&worker, Some(300))));
    let (status, stderr) =
        dispatch.drive_to_stop(|driver, terminal| dispatch.interrupt_running(driver, terminal, 0));
    assert_eq!(status, interrupted, "{stderr}");
    assert!(!stderr.contains("refused ("), "{stderr}");
    assert_ne!(dispatch.launch(0).run_id, "<unset>");
}

// Grove's escalation reaps a session's descendants on a terminal as it does
// detached: the harness leads the group Grove signals, because the front
// became it, so a command it spawned dies with it. The harness spawns a
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
         {grove_llm} complete --done || exit 91\n\
         while : ; do sleep 0.05 ; done",
        grove_llm = shell_quote(&own_grove_llm()),
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
    assert_ne!(dispatch.launch(0).run_id, "<unset>");
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
fn launch_directory_teardown_finishes_after_legacy_signal_or_own_exit() {
    for ending in ["complete", "exit"] {
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
            if ending == "complete" {
                format!("exec {llm} complete")
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
        assert!(
            output.status.success() && stderr.contains("grove finished — loop complete"),
            "{ending}: {stderr}"
        );
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
            Path::new(&dispatch.launch(0).channel).parent(),
            Some(launch.as_path())
        );
        assert!(
            !launch.exists(),
            "interpreted launch directory was retained"
        );
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
         if [ \"$n\" = 0 ]; then exec {llm} complete; fi\n\
         old=$(cat {launches}/0/launch-dir)\n\
         test ! -e \"$old\"\n\
         for verb in 'pick' 'leaf-add . stale --kind impl' 'record-teardown'; do\n\
             if env GROVE_LAUNCH_DIR=\"$old\" {llm} $verb > \"$record/stale-$verb\" 2>&1; then exit 92; fi\n\
         done\n\
         exec {llm} complete --done",
        launches = shell_quote(&dispatch.launches)
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
    let fifo = std::ffi::CString::new(abandoned.join("ending").to_str().unwrap()).unwrap();
    // Reading this abandoned ending would block; cleanup must only unlink it.
    assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
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
    assert!(!abandoned.exists());
    assert!(!link.exists());
    assert!(outside.join("keep").is_file());
    assert!(retained.is_dir());
}

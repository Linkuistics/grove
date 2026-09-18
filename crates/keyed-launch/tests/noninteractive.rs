//! A standalone invocation must not share its caller's terminal or stdin.
use std::fs;
use std::io::{Read, Write};
use std::os::fd::AsRawFd;
use std::os::unix::process::{CommandExt, ExitStatusExt};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::time::{Duration, Instant};

use keyed_launch::{Channel, End, Escalation, Launch, Templates, Vocabulary};

#[test]
#[ignore = "subprocess fixture"]
fn child() {
    if std::env::var("RUNNER_ROLE").as_deref() != Ok("child") {
        return;
    }
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    // SAFETY: queries with no mutable memory or ownership transferred.
    let (sid, pid, inherited) = unsafe {
        (
            libc::getsid(0),
            libc::getpid(),
            libc::fcntl(197, libc::F_GETFD),
        )
    };
    fs::write(
        std::env::var_os("RUNNER_REPORT").unwrap(),
        format!("{sid} {pid} {inherited}\n{input}"),
    )
    .unwrap();
}

fn publish_ready() {
    let report = PathBuf::from(std::env::var_os("RUNNER_READY").unwrap());
    let temporary = report.with_extension("tmp");
    // SAFETY: queries of this fixture's own process and process group.
    let (pid, group) = unsafe { (libc::getpid(), libc::getpgrp()) };
    fs::write(&temporary, format!("{pid} {group}\n")).unwrap();
    fs::rename(temporary, report).unwrap();
}

#[test]
#[ignore = "subprocess fixture"]
fn stubborn_child() {
    // SAFETY: this fixture is an isolated process and intentionally ignores
    // both cancellation signals to require supervisor-driven SIGKILL cleanup.
    unsafe {
        libc::signal(libc::SIGTERM, libc::SIG_IGN);
        libc::signal(libc::SIGINT, libc::SIG_IGN);
    }
    publish_ready();
    loop {
        std::thread::sleep(Duration::from_secs(1));
    }
}

#[test]
#[ignore = "subprocess fixture"]
fn writing_helper() {
    let mut heartbeat = fs::File::create(std::env::var_os("RUNNER_HEARTBEAT").unwrap()).unwrap();
    heartbeat.write_all(b"alive\n").unwrap();
    publish_ready();
    loop {
        heartbeat.write_all(b"alive\n").unwrap();
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
#[ignore = "subprocess fixture"]
fn exiting_leader() {
    let helper = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "writing_helper", "--ignored", "--nocapture"])
        .spawn()
        .unwrap();
    let ready = PathBuf::from(std::env::var_os("RUNNER_READY").unwrap());
    let deadline = Instant::now() + Duration::from_secs(5);
    while !ready.exists() {
        assert!(Instant::now() < deadline, "helper never became ready");
        std::thread::sleep(Duration::from_millis(10));
    }
    // Dropping Child does not reap or terminate it: the launcher must clean up
    // the still-running group member after this group leader exits.
    drop(helper);
}

#[test]
#[ignore = "subprocess fixture"]
fn supervisor() {
    if std::env::var("RUNNER_ROLE").as_deref() != Ok("supervisor") {
        return;
    }
    let dir = tempfile::tempdir().unwrap();
    let config = dir.path().join("config.kdl");
    let executable = std::env::current_exe().unwrap();
    let scenario = std::env::var("RUNNER_SCENARIO").unwrap_or_else(|_| "child".into());
    let template = format!(
        "env RUNNER_ROLE=child '{}' --exact {scenario} --ignored --nocapture",
        executable.display()
    );
    fs::write(
        &config,
        format!("config {{\ncommand \"test\" {template:?}\nbind \"test\" \"test\"\nroute \"test\" \"test\"\n}}\n"),
    )
    .unwrap();
    let templates = Templates::load(&config, None, Vocabulary { slots: &[] }).unwrap();
    let argv = templates.expand("test", &[]).unwrap();
    let channel = Channel::allocate(dir.path()).unwrap();
    let result = keyed_launch::run_noninteractive(
        Launch {
            argv: &argv,
            channel: &channel,
            channel_var: "RUNNER_CHANNEL",
            scrub: &[],
            cwd: Some(dir.path()),
            escalation: Escalation {
                grace: Duration::ZERO,
                // A detached cancellation must not inherit this full wait. The
                // outer fixture gives cleanup less than its own five-second grace.
                kill_grace: Duration::from_secs(10),
            },
        },
        fs::File::create(dir.path().join("harness.log")).unwrap(),
    )
    .unwrap();
    if scenario == "stubborn_child" {
        let expected_signal = std::env::var("RUNNER_EXPECTED_SIGNAL")
            .unwrap()
            .parse()
            .unwrap();
        assert_eq!(
            result.end,
            End::Interrupted {
                signal: expected_signal
            }
        );
        assert_eq!(result.status.signal(), Some(libc::SIGKILL));
    } else {
        assert!(result.status.success());
    }
    if scenario != "child" {
        let ready = PathBuf::from(std::env::var_os("RUNNER_READY").unwrap());
        let (pid, _) = read_ready(&ready).unwrap();
        // Check at the return boundary, before an outer wait could hide late
        // cleanup. An existing PID, including a zombie, is not confirmed gone.
        assert_process_gone(pid);
    }
    if scenario == "exiting_leader" {
        assert_eq!(result.end, End::Exited);
        let heartbeat = PathBuf::from(std::env::var_os("RUNNER_HEARTBEAT").unwrap());
        let contents = fs::read(&heartbeat).unwrap();
        assert!(!contents.is_empty(), "helper never wrote anything");
        std::thread::sleep(Duration::from_millis(150));
        assert_eq!(
            fs::read(heartbeat).unwrap(),
            contents,
            "helper kept writing after return"
        );
    }
}

fn read_ready(path: &Path) -> Option<(libc::pid_t, libc::pid_t)> {
    let contents = fs::read_to_string(path).ok()?;
    let mut fields = contents.split_whitespace();
    Some((fields.next()?.parse().ok()?, fields.next()?.parse().ok()?))
}

fn assert_process_gone(pid: libc::pid_t) {
    // SAFETY: signal zero only checks the recorded fixture process's presence.
    assert_eq!(
        unsafe { libc::kill(pid, 0) },
        -1,
        "fixture process {pid} survived runner return"
    );
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ESRCH)
    );
}

struct RunningSupervisor {
    process: Child,
    ready: PathBuf,
    log: PathBuf,
}

impl RunningSupervisor {
    fn start(directory: &Path, scenario: &str, signal: i32) -> Self {
        let ready = directory.join("ready");
        let log = directory.join("supervisor.log");
        let stdout = fs::File::create(&log).unwrap();
        let stderr = stdout.try_clone().unwrap();
        let process = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "supervisor", "--ignored", "--nocapture"])
            .env("RUNNER_ROLE", "supervisor")
            .env("RUNNER_SCENARIO", scenario)
            .env("RUNNER_READY", &ready)
            .env("RUNNER_HEARTBEAT", directory.join("heartbeat"))
            .env("RUNNER_EXPECTED_SIGNAL", signal.to_string())
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .unwrap();
        Self {
            process,
            ready,
            log,
        }
    }

    fn await_ready(&mut self) -> (libc::pid_t, libc::pid_t) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(ready) = read_ready(&self.ready) {
                return ready;
            }
            assert!(
                self.process.try_wait().unwrap().is_none(),
                "supervisor ended before readiness: {}",
                fs::read_to_string(&self.log).unwrap()
            );
            assert!(Instant::now() < deadline, "fixture did not become ready");
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn await_exit(&mut self, deadline: Instant) -> ExitStatus {
        loop {
            if let Some(status) = self.process.try_wait().unwrap() {
                assert!(
                    status.success(),
                    "supervisor failed: {}",
                    fs::read_to_string(&self.log).unwrap()
                );
                return status;
            }
            assert!(
                Instant::now() < deadline,
                "supervisor exceeded cleanup deadline: {}",
                fs::read_to_string(&self.log).unwrap()
            );
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for RunningSupervisor {
    fn drop(&mut self) {
        if let Some((_, group)) = read_ready(&self.ready) {
            // SAFETY: the fixture reports the private group created by the
            // launcher. This also prevents a failing test from leaking it.
            unsafe { libc::kill(-group, libc::SIGKILL) };
        }
        let _ = self.process.kill();
        let _ = self.process.wait();
    }
}

fn assert_cancellation_cleanup(signal: i32) {
    let directory = tempfile::tempdir().unwrap();
    let mut supervisor = RunningSupervisor::start(directory.path(), "stubborn_child", signal);
    let (child, _) = supervisor.await_ready();
    let deadline = Instant::now() + Duration::from_secs(4);
    // SAFETY: the supervisor is our live direct child and readiness confirms
    // its cancellation handler has already been installed.
    assert_eq!(
        unsafe { libc::kill(supervisor.process.id() as libc::pid_t, signal) },
        0
    );
    supervisor.await_exit(deadline);
    assert_process_gone(child);
}

#[test]
fn sigint_cancellation_kills_a_stubborn_child_before_the_outer_grace() {
    assert_cancellation_cleanup(libc::SIGINT);
}

#[test]
fn sigterm_cancellation_kills_a_stubborn_child_before_the_outer_grace() {
    assert_cancellation_cleanup(libc::SIGTERM);
}

#[test]
fn leader_exit_stops_a_group_helper_before_runner_returns() {
    let directory = tempfile::tempdir().unwrap();
    let mut supervisor = RunningSupervisor::start(directory.path(), "exiting_leader", 0);
    let (helper, group) = supervisor.await_ready();
    assert_ne!(helper, group, "fixture did not create a separate helper");
    supervisor.await_exit(Instant::now() + Duration::from_secs(4));
    assert_process_gone(helper);
}

#[test]
fn standalone_child_has_a_new_session_and_cannot_consume_callers_stdin() {
    let dir = tempfile::tempdir().unwrap();
    let report = dir.path().join("report");
    let secret = fs::File::create(dir.path().join("parent-secret")).unwrap();
    let descriptor = secret.as_raw_fd();
    let mut command = Command::new(std::env::current_exe().unwrap());
    // SAFETY: dup2 is async-signal-safe and both descriptor numbers are valid.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(descriptor, 197) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let mut supervisor = command
        .args(["--exact", "supervisor", "--ignored", "--nocapture"])
        .env("RUNNER_ROLE", "supervisor")
        .env("RUNNER_REPORT", &report)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    supervisor
        .stdin
        .take()
        .unwrap()
        .write_all(b"parent input")
        .unwrap();
    let result = supervisor.wait_with_output().unwrap();
    assert!(result.status.success(), "{result:?}");
    let report = fs::read_to_string(report).unwrap();
    let fields: Vec<_> = report.split_whitespace().collect();
    assert_eq!(fields.len(), 3, "child consumed parent input: {report}");
    assert_eq!(fields[0], fields[1], "child is not a session leader");
    assert_eq!(fields[2], "-1", "child inherited a parent descriptor");
}

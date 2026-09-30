//! Driving a worker directly, without the front: the firing configurations of
//! the hostile classes (`docs/specs/harness-selection-and-execution.md`, the
//! firing-configuration table under *Agreed test seams and acceptance*).
//!
//! A class the public launcher keeps inert is proved only beside a
//! configuration in which the same fixture is seen to fire. For three classes
//! that configuration is a probe build, the shipped worker's source with one
//! control removed, which `task dispatch:probes` compiles for tests only. A
//! probe reports an identity no front accepts, so it cannot be run through the
//! front at all. For `BUN_OPTIONS` it is the shipped worker itself, started
//! without the front's scrubbing. Either way the test plays the front's part:
//! it starts the worker with the cwd and environment the case names, reads its
//! hello, hands it an entry, and reads what it reports.

use std::ffi::OsString;
use std::io::{Read as _, Write as _};
use std::os::fd::AsRawFd as _;
use std::os::unix::net::UnixStream;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Duration;

use serde_json::{json, Value};

use super::FRONT;

/// A probe build, by the control it removes.
#[derive(Clone, Copy, Debug)]
pub enum Probe {
    /// dotenv and bunfig autoloading on, as in Bun's defaults.
    Autoload,
    /// tsconfig and package.json autoloading on.
    Tsconfig,
    /// No embedded-module registration.
    Unregistered,
}

impl Probe {
    pub fn name(self) -> &'static str {
        match self {
            Probe::Autoload => "autoload",
            Probe::Tsconfig => "tsconfig",
            Probe::Unregistered => "unregistered",
        }
    }
}

/// The checkout's target directory: the front is `<target>/<profile>/`, as its
/// worker's layout assumes.
fn target_dir() -> PathBuf {
    let front = Path::new(FRONT);
    front
        .parent()
        .and_then(Path::parent)
        .expect("the front sits in <target>/<profile>/")
        .to_owned()
}

/// The worker the checkout's front uses, which `task dispatch:worker` builds.
pub fn shipped_worker() -> PathBuf {
    let worker = target_dir().join("libexec/harness-dispatch/harness-dispatch-policy");
    assert!(
        worker.is_file(),
        "no worker at {}; run `task dispatch:worker`",
        worker.display()
    );
    worker
}

/// A probe build, which `task dispatch:probes` compiles. Its absence fails
/// the test: a firing configuration that cannot run proves nothing, so it is
/// never skipped.
pub fn probe_build(probe: Probe) -> PathBuf {
    let worker = target_dir()
        .join("probes/harness-dispatch")
        .join(probe.name())
        .join("harness-dispatch-policy");
    assert!(
        worker.is_file(),
        "no {} probe build at {}; run `task dispatch:probes`",
        probe.name(),
        worker.display()
    );
    worker
}

/// What a directly driven worker did.
#[derive(Debug)]
pub struct Driven {
    /// Its first frame, if it sent one.
    pub hello: Option<Value>,
    /// Its report on the entry: a `policy` frame or a `failure`, if it got
    /// that far.
    pub report: Option<Value>,
    pub stdout: String,
    pub stderr: String,
}

impl Driven {
    /// The report, after asserting the worker loaded the entry's policy.
    pub fn loaded(&self) -> &Value {
        let report = self.report.as_ref().unwrap_or_else(|| {
            panic!(
                "the worker reported nothing on the entry\nhello: {:?}\nstdout: {}\nstderr: {}",
                self.hello, self.stdout, self.stderr
            )
        });
        assert_eq!(
            report["type"], "policy",
            "{report}\nstderr: {}",
            self.stderr
        );
        report
    }
}

/// Start `worker` in `cwd` with exactly `env`, as the front would but without
/// its private directory or its scrubbing, and have it evaluate `entry`.
/// Returns once the worker has exited.
pub fn drive(worker: &Path, cwd: &Path, env: &[(&str, OsString)], entry: &Path) -> Driven {
    let (front, worker_end) = UnixStream::pair().expect("a socket pair");
    let channel = worker_end.as_raw_fd();
    let mut command = Command::new(worker);
    command
        .env_clear()
        .envs(env.iter().map(|(name, value)| (name, value)))
        .current_dir(cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // SAFETY: dup2 and fcntl are async-signal-safe, and the descriptor was
    // computed before the fork. The pair is close-on-exec, so a pair that
    // already sits at 3 must have that flag cleared rather than be duplicated
    // onto itself.
    unsafe {
        command.pre_exec(move || {
            if channel == 3 {
                let flags = libc::fcntl(3, libc::F_GETFD);
                if flags == -1 || libc::fcntl(3, libc::F_SETFD, flags & !libc::FD_CLOEXEC) == -1 {
                    return Err(std::io::Error::last_os_error());
                }
            } else if libc::dup2(channel, 3) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let child = command.spawn().expect("the worker starts");
    drop(worker_end);
    front
        .set_read_timeout(Some(Duration::from_secs(30)))
        .expect("a read timeout");

    let mut front = front;
    let hello = read_frame(&mut front);
    let mut report = None;
    if hello.is_some() {
        let evaluate = json!({
            "type": "evaluate",
            "protocol": 1,
            "entry": entry.to_str().expect("a UTF-8 entry"),
            "request": {
                "schemaVersion": 1,
                "kind": "impl",
                "cwd": cwd.to_str().expect("a UTF-8 cwd"),
                "limits": {
                    "selectionMs": 30_000, "contextBytes": 262_144, "sourceBytes": 65_536,
                    "sources": 256, "messageBytes": 1_048_576, "diagnosticsBytes": 262_144,
                },
            },
            "bounds": {
                "contextBytes": 262_144, "sourceBytes": 65_536, "sources": 256,
                "messageBytes": 1_048_576,
            },
            "measured": [],
        });
        write_frame(&mut front, &evaluate);
        report = read_frame(&mut front);
    }
    // The closed channel is the worker's sign that nothing more is asked.
    drop(front);
    let output = child.wait_with_output().expect("the worker exits");
    Driven {
        hello,
        report,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    }
}

/// The bound on a frame the worker sends, the `messageBytes` limit `drive`
/// hands it.
const MESSAGE_BYTES: u32 = 1_048_576;

/// One frame: a four-byte big-endian length and that much JSON, as
/// `src/frame.rs` reads it. `None` once the worker has closed its end. A
/// length over the bound is `{ "type": "malformed", "header": <its four
/// bytes> }`, unread beyond them, as the front refuses it: whatever wrote
/// those bytes, it was not the worker's framing.
fn read_frame(channel: &mut UnixStream) -> Option<Value> {
    let mut length = [0; 4];
    channel.read_exact(&mut length).ok()?;
    if u32::from_be_bytes(length) > MESSAGE_BYTES {
        return Some(json!({ "type": "malformed", "header": length }));
    }
    let mut body = vec![0; u32::from_be_bytes(length) as usize];
    channel.read_exact(&mut body).expect("a whole frame");
    Some(serde_json::from_slice(&body).expect("a JSON frame"))
}

fn write_frame(channel: &mut UnixStream, frame: &Value) {
    let body = serde_json::to_vec(frame).expect("an encodable frame");
    let length = u32::try_from(body.len()).expect("a frame under 4 GiB");
    channel
        .write_all(&length.to_be_bytes())
        .and_then(|()| channel.write_all(&body))
        .expect("the worker reads its frame");
}

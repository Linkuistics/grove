//! The command seam: the real front executable, run as a subprocess against
//! temporary policies and a fake harness, with no Grove binary, configuration or
//! task tree.
//!
//! Every invocation starts from an empty environment plus a private HOME and a
//! minimal PATH, in a private working directory, so nothing from the developer's
//! own configuration or a live Grove session can take part. The compiled worker
//! must already be built (`task dispatch:worker`); if it is absent or stale the
//! front refuses with exit 5 and every test that expects a selection fails. No
//! test is skipped for want of it.

#![allow(dead_code)] // each test binary uses its own subset

pub mod direct;
pub mod hold;
pub mod probe;
pub mod stall;

use std::ffi::CString;
use std::fs;
use std::io;
use std::ops::RangeInclusive;
use std::os::fd::{AsRawFd as _, FromRawFd as _, OwnedFd, RawFd};
use std::os::unix::ffi::OsStrExt as _;
use std::os::unix::fs::PermissionsExt as _;
use std::os::unix::process::{CommandExt as _, ExitStatusExt as _};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use serde_json::Value;
use tempfile::TempDir;

pub const FRONT: &str = env!("CARGO_BIN_EXE_harness-dispatch");

/// A one-candidate policy that routes `impl`, the shape most tests start from.
pub const ROUTED: &str = r#"export const policy = {
  schemaVersion: 1,
  version: "seam-1",
  catalog: [
    { id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: [{ slot: "prompt" }] },
  ],
  routes: { impl: "deep" },
};
"#;

/// The fake harness every sandbox puts on PATH. It records how it was started
/// under `$FAKE_HARNESS_RECORD` (its arguments NUL-separated, its physical cwd
/// and its PID), copies stdin to stdout, and then behaves as the environment
/// tells it: it writes to descriptor 7, kills itself with a signal, or exits
/// with a code. The record directory is created exclusively, so its presence
/// is the marker that the harness ran, and a second start fails loudly.
///
/// It also records the run identity it was handed, a copy of the record store
/// as it found it (so a test can see the handoff was committed before the
/// harness started), which of descriptors 3 to 9 it inherited open, and the
/// names in its environment.
pub const FAKE_HARNESS: &str = r#"#!/bin/sh
record=$FAKE_HARNESS_RECORD
mkdir "$record" || exit 90
for argument in "$@"; do printf '%s\0' "$argument"; done > "$record/args"
pwd -P > "$record/cwd"
echo $$ > "$record/pid"
printf '%s' "${HARNESS_DISPATCH_RUN_ID-<unset>}" > "$record/run-id"
printf '%s' "${HARNESS_DISPATCH_STATE_DIR-<unset>}" > "$record/state-dir"
if [ -f "$HARNESS_DISPATCH_STATE_DIR/records.sqlite3" ]; then
  cp "$HARNESS_DISPATCH_STATE_DIR/records.sqlite3" "$record/store-at-start"
fi
for fd in 3 4 5 6 7 8 9; do
  if (: <&"$fd") 2>/dev/null; then printf '%s\n' "$fd"; fi
done > "$record/fds"
env | sed 's/=.*//' | LC_ALL=C sort > "$record/env"
cat
if [ -n "$FAKE_HARNESS_FD7" ]; then echo "$FAKE_HARNESS_FD7" >&7; fi
if [ -n "$FAKE_HARNESS_SIGNAL" ]; then kill -s "$FAKE_HARNESS_SIGNAL" $$; fi
exit "${FAKE_HARNESS_EXIT:-0}"
"#;

pub struct Sandbox {
    _dir: TempDir,
    pub root: PathBuf,
    pub home: PathBuf,
    pub cwd: PathBuf,
    pub tmp: PathBuf,
    /// First on PATH, holding `fake-harness`.
    pub bin: PathBuf,
    /// Where the fake harness records a start; absent until it runs.
    pub record: PathBuf,
}

impl Sandbox {
    pub fn new() -> Self {
        let dir = tempfile::tempdir().expect("temporary directory");
        // Canonical, so paths the front reports (it canonicalizes) compare
        // equal on macOS, where the temporary directory sits behind a symlink.
        let root = fs::canonicalize(dir.path()).expect("canonical temporary directory");
        let (home, cwd, tmp, bin) = (
            root.join("home"),
            root.join("work"),
            root.join("tmp"),
            root.join("bin"),
        );
        for dir in [&home, &cwd, &tmp, &bin] {
            fs::create_dir(dir).expect("sandbox directory");
        }
        executable(&bin.join("fake-harness"), FAKE_HARNESS);
        let record = root.join("harness-record");
        Sandbox {
            _dir: dir,
            root,
            home,
            cwd,
            tmp,
            bin,
            record,
        }
    }

    /// Whether the fake harness started.
    pub fn harness_ran(&self) -> bool {
        self.record.exists()
    }

    /// The arguments the fake harness received after its argv[0], exactly.
    pub fn harness_args(&self) -> Vec<String> {
        let bytes = fs::read(self.record.join("args")).expect("the fake harness ran");
        let text = String::from_utf8(bytes).expect("UTF-8 arguments");
        let mut args: Vec<String> = text.split('\0').map(str::to_owned).collect();
        assert_eq!(args.pop().as_deref(), Some(""), "arguments end with a NUL");
        args
    }

    /// The fake harness's physical working directory.
    pub fn harness_cwd(&self) -> PathBuf {
        let cwd = fs::read_to_string(self.record.join("cwd")).expect("the fake harness ran");
        PathBuf::from(cwd.strip_suffix('\n').expect("pwd ends its line"))
    }

    /// `HARNESS_DISPATCH_RUN_ID` as the fake harness received it.
    pub fn harness_run_id(&self) -> String {
        fs::read_to_string(self.record.join("run-id")).expect("the fake harness ran")
    }

    /// `HARNESS_DISPATCH_STATE_DIR` as the fake harness received it.
    pub fn harness_state_dir(&self) -> String {
        fs::read_to_string(self.record.join("state-dir")).expect("the fake harness ran")
    }

    /// The descriptors from 3 to 9 the fake harness inherited open.
    pub fn harness_fds(&self) -> Vec<u32> {
        let fds = fs::read_to_string(self.record.join("fds")).expect("the fake harness ran");
        fds.lines()
            .map(|fd| fd.parse().expect("a descriptor"))
            .collect()
    }

    /// The names in the fake harness's environment, sorted.
    pub fn harness_env(&self) -> Vec<String> {
        let names = fs::read_to_string(self.record.join("env")).expect("the fake harness ran");
        names.lines().map(str::to_owned).collect()
    }

    /// The default record store, under the sandbox's HOME.
    pub fn default_store(&self) -> PathBuf {
        self.home
            .join(".local/state/harness-dispatch/records.sqlite3")
    }

    pub fn harness_pid(&self) -> u32 {
        let pid = fs::read_to_string(self.record.join("pid")).expect("the fake harness ran");
        pid.trim().parse().expect("a PID")
    }

    pub fn personal_path(&self) -> PathBuf {
        self.home.join(".config/harness-dispatch/policy.ts")
    }

    pub fn personal_policy(&self, source: &str) -> PathBuf {
        let path = self.personal_path();
        write(&path, source);
        path
    }

    /// Write a file relative to the sandbox's working directory.
    pub fn file(&self, relative: &str, contents: &str) -> PathBuf {
        let path = self.cwd.join(relative);
        write(&path, contents);
        path
    }

    /// The front with an empty environment plus HOME, PATH, TMPDIR and the
    /// fake harness's record location.
    pub fn command(&self) -> Command {
        self.command_for(Path::new(FRONT))
    }

    /// The same, for a front executable at another path (a copy or symlink).
    pub fn command_for(&self, front: &Path) -> Command {
        let mut command = Command::new(front);
        command
            .env_clear()
            .env("HOME", &self.home)
            .env("PATH", format!("{}:/usr/bin:/bin", text(&self.bin)))
            .env("TMPDIR", &self.tmp)
            .env("FAKE_HARNESS_RECORD", &self.record)
            .current_dir(&self.cwd)
            .stdin(Stdio::null());
        command
    }

    pub fn inspect(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.arg("inspect").args(args);
        run(&mut command)
    }

    pub fn run(&self, args: &[&str]) -> Run {
        let mut command = self.command();
        command.arg("run").args(args);
        run(&mut command)
    }
}

pub fn write(path: &Path, contents: &str) {
    fs::create_dir_all(path.parent().expect("a parent")).expect("parent directory");
    fs::write(path, contents).expect("write file");
}

pub fn executable(path: &Path, script: &str) {
    write(path, script);
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).expect("chmod");
}

/// The descriptors the fake harness probes, and so the ones
/// `Sandbox::harness_fds` can report.
pub const PROBED: RangeInclusive<RawFd> = 3..=9;

/// Start `command` as a caller that left `held` open on its number and held
/// nothing else among the probed descriptors.
///
/// Neither half can be left to the numbers this process's descriptors happen
/// to have, because the other tests' threads decide those. A file opened here
/// can land on the very number it is to be handed down on, and `dup2` onto
/// its own number does nothing: the close-on-exec flag Rust opened the file
/// with stays set, and the descriptor closes as the command starts. So the
/// file is first duplicated above every probed number, and the `dup2` never
/// names one descriptor twice. And where there is no `pipe2`, as on macOS,
/// Rust makes a pipe and marks it close-on-exec in two steps, so a child
/// forked between them inherits a pipe another thread is still making:
/// https://github.com/rust-lang/rust/blob/1.98.1/library/std/src/sys/pipe/unix.rs
/// The forked child has no other thread, so what it marks there stays marked.
pub fn caller_leaves_open(command: &mut Command, held: Option<(&fs::File, RawFd)>) {
    let held = held.map(|(file, number)| {
        assert!(PROBED.contains(&number), "descriptor {number} is probed");
        // SAFETY: F_DUPFD_CLOEXEC returns a new descriptor this function owns.
        let above =
            unsafe { libc::fcntl(file.as_raw_fd(), libc::F_DUPFD_CLOEXEC, *PROBED.end() + 1) };
        assert_ne!(above, -1, "duplicate: {}", io::Error::last_os_error());
        // SAFETY: `above` is a fresh descriptor owned by nothing else.
        (unsafe { OwnedFd::from_raw_fd(above) }, number)
    });
    // SAFETY: the closure runs between fork and exec and calls only `dup2`
    // and `fcntl`, which are async-signal-safe, on descriptors computed
    // before the fork.
    unsafe {
        command.pre_exec(move || {
            let mut kept = None;
            if let Some((above, number)) = &held {
                if libc::dup2(above.as_raw_fd(), *number) == -1 {
                    return Err(io::Error::last_os_error());
                }
                kept = Some(*number);
            }
            for fd in PROBED {
                if Some(fd) != kept
                    && libc::fcntl(fd, libc::F_SETFD, libc::FD_CLOEXEC) == -1
                    && io::Error::last_os_error().raw_os_error() != Some(libc::EBADF)
                {
                    return Err(io::Error::last_os_error());
                }
            }
            Ok(())
        });
    }
}

pub fn mkfifo(path: &Path) {
    let name = CString::new(path.as_os_str().as_bytes()).unwrap();
    // SAFETY: mkfifo reads the NUL-terminated path and creates a FIFO.
    assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0, "mkfifo");
}

pub struct Run {
    pub code: Option<i32>,
    /// The signal the front died of, if it did not exit.
    pub signal: Option<i32>,
    pub stdout: String,
    pub stderr: String,
}

pub fn run(command: &mut Command) -> Run {
    Run::from(command.output().expect("the front executable runs"))
}

impl From<Output> for Run {
    fn from(output: Output) -> Self {
        Run {
            code: output.status.code(),
            signal: output.status.signal(),
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        }
    }
}

impl Run {
    /// The whole of stdout as one JSON document, after asserting success.
    pub fn report(&self) -> Value {
        assert_eq!(
            self.code,
            Some(0),
            "expected success\nstdout: {}\nstderr: {}",
            self.stdout,
            self.stderr
        );
        serde_json::from_str(&self.stdout).unwrap_or_else(|error| {
            panic!("stdout is not one JSON document ({error}): {}", self.stdout)
        })
    }

    /// The JSON refusal on stderr, after asserting the exit result, that stdout
    /// carries no partial object, and the refusal contract: a stable code, a
    /// stage, a message, a remedy, and the input or source involved.
    pub fn refusal(&self, exit: i32) -> Value {
        assert_eq!(
            self.code,
            Some(exit),
            "expected exit {exit}\nstdout: {}\nstderr: {}",
            self.stdout,
            self.stderr
        );
        assert_eq!(self.stdout, "", "a refusal prints nothing on stdout");
        let document: Value = serde_json::from_str(&self.stderr).unwrap_or_else(|error| {
            panic!("stderr is not one JSON document ({error}): {}", self.stderr)
        });
        assert_eq!(document["schemaVersion"], 1);
        assert_eq!(document["error"]["exit"], exit);
        for field in ["code", "stage", "message", "remedy"] {
            assert!(
                document["error"][field]
                    .as_str()
                    .is_some_and(|text| !text.is_empty()),
                "refusal lacks {field}: {document}"
            );
        }
        let named = |field: &str| {
            document["error"][field]
                .as_str()
                .is_some_and(|text| !text.is_empty())
        };
        assert!(
            named("input") || named("source"),
            "refusal names neither its input nor its source: {document}"
        );
        document
    }

    /// The JSON cancellation on stderr, after asserting that the front died of
    /// `signal`, named `name`, and printed nothing on stdout: the refusal
    /// contract, with the signal named and the `128 + N` a shell reports.
    pub fn cancelled(&self, signal: i32, name: &str) -> Value {
        assert_eq!(
            (self.code, self.signal),
            (None, Some(signal)),
            "expected death by {name}\nstdout: {}\nstderr: {}",
            self.stdout,
            self.stderr
        );
        assert_eq!(self.stdout, "", "a cancellation prints nothing on stdout");
        let document: Value = serde_json::from_str(&self.stderr).unwrap_or_else(|error| {
            panic!("stderr is not one JSON document ({error}): {}", self.stderr)
        });
        assert_eq!(document["schemaVersion"], 1);
        let error = &document["error"];
        assert_eq!(error["code"], "selection_cancelled", "{document}");
        assert_eq!(error["stage"], "evaluation", "{document}");
        assert_eq!(error["signal"], name, "{document}");
        assert_eq!(error["exit"], 128 + signal, "{document}");
        for field in ["message", "remedy", "source"] {
            assert!(
                error[field].as_str().is_some_and(|text| !text.is_empty()),
                "cancellation lacks {field}: {document}"
            );
        }
        document
    }
}

pub fn text(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}

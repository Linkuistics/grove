//! The signal-state probe: a fake harness, written in C, that reports the
//! mask and the ignored, pending and caught signals exec delivered to it
//! (`signal-probe.c` says why C). And the caller's side: a front started with
//! exactly the ignored set and mask a case names, whatever the test process
//! itself inherited.

use std::collections::BTreeSet;
use std::fs;
use std::os::unix::process::CommandExt as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

use super::Sandbox;

const SOURCE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/support/signal-probe.c");

/// Every signal number either platform uses.
const LAST_SIGNAL: libc::c_int = 64;

/// The probe, compiled once per test binary with the host's C compiler (`$CC`,
/// else `cc`, which the bundled SQLite build already needs) into Cargo's
/// scratch directory for integration tests. A compiler that is missing or
/// fails fails the test; nothing is skipped.
pub fn built() -> &'static Path {
    static BUILT: OnceLock<PathBuf> = OnceLock::new();
    BUILT.get_or_init(|| {
        let dir = Path::new(env!("CARGO_TARGET_TMPDIR"));
        fs::create_dir_all(dir).expect("Cargo's scratch directory");
        let built = dir.join("signal-probe");
        // A private name first, so a concurrent test run never executes a
        // half-written file.
        let partial = dir.join(format!("signal-probe.{}", std::process::id()));
        let compiler = std::env::var_os("CC").unwrap_or_else(|| "cc".into());
        let output = Command::new(&compiler)
            .args(["-Wall", "-Wextra", "-Werror", "-o"])
            .arg(&partial)
            .arg(SOURCE)
            .output()
            .unwrap_or_else(|error| panic!("cannot run the C compiler {compiler:?}: {error}"));
        assert!(
            output.status.success(),
            "compiling {SOURCE} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        fs::rename(&partial, &built).expect("install the compiled probe");
        built
    })
}

/// A policy routing `impl` to the probe, installed on the sandbox's PATH as
/// `signal-probe`, with `args` before the prompt.
pub fn install(sandbox: &Sandbox, args: &str) {
    fs::copy(built(), sandbox.bin.join("signal-probe")).expect("copy the probe");
    sandbox.personal_policy(&format!(
        r#"export const policy = {{
  schemaVersion: 1,
  version: "probe-1",
  catalog: [
    {{ id: "probe", provider: "origin-a", model: "model-a", effort: "low", program: "signal-probe", args: [{args}{{ slot: "prompt" }}] }},
  ],
  routes: {{ impl: "probe" }},
}};
"#
    ));
}

/// Signal state as sets of signal numbers.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct State {
    pub ignored: BTreeSet<libc::c_int>,
    pub blocked: BTreeSet<libc::c_int>,
    pub pending: BTreeSet<libc::c_int>,
    pub caught: BTreeSet<libc::c_int>,
}

impl State {
    /// A caller's state: `ignored` ignored, `blocked` blocked, nothing else.
    pub fn caller(ignored: &[libc::c_int], blocked: &[libc::c_int]) -> State {
        State {
            ignored: ignored.iter().copied().collect(),
            blocked: blocked.iter().copied().collect(),
            ..State::default()
        }
    }

    /// What the probe in `sandbox` reported.
    pub fn observed(sandbox: &Sandbox) -> State {
        let report = fs::read_to_string(sandbox.record.join("signals"))
            .expect("the probe ran and wrote its report");
        let mut state = State::default();
        for line in report.lines() {
            let mut words = line.split(' ');
            let set = match words.next() {
                Some("ignored") => &mut state.ignored,
                Some("blocked") => &mut state.blocked,
                Some("pending") => &mut state.pending,
                Some("caught") => &mut state.caught,
                other => panic!("unexpected probe line {other:?} in {report}"),
            };
            set.extend(words.map(|number| number.parse::<libc::c_int>().expect("a signal")));
        }
        state
    }

    /// Start `command` with exactly this ignored set and mask: every other
    /// signal is set to default first, so nothing the test process inherited
    /// takes part.
    pub fn apply_to(&self, command: &mut Command) {
        let ignored: Vec<libc::c_int> = self.ignored.iter().copied().collect();
        let blocked: Vec<libc::c_int> = self.blocked.iter().copied().collect();
        // SAFETY: between fork and exec the closure makes only sigaction,
        // signal and sigprocmask calls, over vectors built before the fork.
        unsafe {
            command.pre_exec(move || {
                for signal in 1..=LAST_SIGNAL {
                    if signal != libc::SIGKILL && signal != libc::SIGSTOP {
                        libc::signal(signal, libc::SIG_DFL);
                    }
                }
                for &signal in &ignored {
                    libc::signal(signal, libc::SIG_IGN);
                }
                let mut mask: libc::sigset_t = std::mem::zeroed();
                libc::sigemptyset(&mut mask);
                for &signal in &blocked {
                    libc::sigaddset(&mut mask, signal);
                }
                if libc::sigprocmask(libc::SIG_SETMASK, &mask, std::ptr::null_mut()) != 0 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
    }
}

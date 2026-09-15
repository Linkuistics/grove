use super::*;
// The shared fixture also serves binary tests that exercise input and resize.
#[allow(dead_code)]
#[path = "../tests/support/pty.rs"]
mod pty;

#[test]
fn fault_child() {
    let Ok(fault) = std::env::var("GROVE_TEST_TERMINAL_FAULT") else {
        return;
    };
    let directory = tempfile::tempdir().unwrap();
    let mut viewer = Viewer::new(directory.path().to_owned());
    let result = with_session(|session, stopped| {
        session.raw()?;
        if fault == "raw" {
            bail!("INJECTED raw failure");
        }
        session.alternate()?;
        if fault == "alternate" {
            bail!("INJECTED alternate failure");
        }
        session.hide()?;
        if fault == "hide" {
            bail!("INJECTED hide failure");
        }
        let mut terminal = Terminal::new(CrosstermBackend::new(FaultWriter {
            fail: fault == "draw",
        }))?;
        drive_with(&mut viewer, stopped, &mut terminal, |_| {
            match fault.as_str() {
                "input" => Err(io::Error::other("INJECTED input failure")),
                "panic" => panic!("INJECTED PANIC"),
                _ => panic!("draw failure was not propagated"),
            }
        })
    });
    if let Err(error) = result {
        eprintln!("{error:#}");
        std::process::exit(1);
    }
}

struct FaultWriter {
    fail: bool,
}

impl io::Write for FaultWriter {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if self.fail {
            Err(io::Error::other("INJECTED draw failure"))
        } else {
            io::stdout().write(bytes)
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

#[test]
fn faults_restore_before_diagnostics() {
    for fault in ["raw", "alternate", "hide", "input", "draw", "panic"] {
        let mut child = pty::Pty::spawn(
            std::process::Command::new(std::env::current_exe().unwrap())
                .args(["--exact", "terminal::tests::fault_child", "--nocapture"])
                .env("GROVE_TEST_TERMINAL_FAULT", fault),
        );
        assert!(!child.wait().success(), "{fault}");
        child.assert_restored();
        let output = String::from_utf8_lossy(&child.output);
        for cleanup in ["\x1b[?1049l", "\x1b[?25h"] {
            assert!(
                output.find(cleanup).unwrap() < output.find("INJECTED").unwrap(),
                "{fault}: {output}"
            );
        }
    }
}

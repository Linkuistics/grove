// Compiled as a private run-module test, not as an integration target.
use super::*;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::os::unix::process::ExitStatusExt as _;
use std::rc::Rc;

type Trace = Rc<RefCell<Vec<&'static str>>>;
struct FakeProcess {
    polls: VecDeque<std::io::Result<Option<ExitStatus>>>,
    reaps: VecDeque<std::io::Result<ExitStatus>>,
    trace: Trace,
}
impl Process for FakeProcess {
    fn try_wait(&mut self) -> std::io::Result<Option<ExitStatus>> {
        self.trace.borrow_mut().push("poll");
        self.polls.pop_front().expect("unexpected poll")
    }
    fn wait(&mut self) -> std::io::Result<ExitStatus> {
        self.trace.borrow_mut().push("wait");
        self.reaps.pop_front().expect("unexpected wait")
    }
    fn signal(&mut self, signal: i32) {
        self.trace.borrow_mut().push(if signal == libc::SIGTERM {
            "term"
        } else {
            "kill"
        });
    }
}
fn failed<T>() -> std::io::Result<T> {
    Err(std::io::Error::other("injected wait failure"))
}

#[test]
fn confirmed_reap_precedes_token_read_and_recovery_on_every_wait_path() {
    for path in ["poll", "recovery", "escalation"] {
        for confirmed in [false, true] {
            if path == "poll" && !confirmed {
                continue;
            }
            let dir = tempfile::TempDir::new().unwrap();
            let channel = Channel::allocate(dir.path()).unwrap();
            let trace = Trace::default();
            let status = ExitStatus::from_raw(0);
            let polls = match path {
                "poll" => vec![Ok(Some(status))],
                "recovery" => vec![failed()],
                _ => {
                    std::fs::write(channel.path(), "before").unwrap();
                    vec![Ok(None), Ok(None), Ok(None)]
                }
            };
            let child = FakeProcess {
                polls: polls.into(),
                reaps: vec![if confirmed { Ok(status) } else { failed() }].into(),
                trace: trace.clone(),
            };
            let outcome = supervise(
                child,
                &channel,
                Escalation {
                    grace: Duration::ZERO,
                    kill_grace: Duration::ZERO,
                },
                None,
                0,
                &mut |event| {
                    assert_eq!(event, LaunchEvent::Reaped);
                    trace.borrow_mut().push("reaped");
                    std::fs::write(channel.path(), "after").unwrap();
                },
                || trace.borrow_mut().push("recover"),
            );
            let expected: &[&str] = match (path, confirmed) {
                ("poll", _) => &["poll", "reaped", "recover"],
                ("recovery", true) => &["poll", "kill", "wait", "reaped", "recover"],
                ("recovery", false) => &["poll", "kill", "wait", "recover"],
                (_, true) => &[
                    "poll", "poll", "term", "poll", "kill", "wait", "reaped", "recover",
                ],
                (_, false) => &["poll", "poll", "term", "poll", "kill", "wait", "recover"],
            };
            assert_eq!(&*trace.borrow(), expected, "{path}, confirmed={confirmed}");
            if path == "recovery" || !confirmed {
                let error = outcome.unwrap_err().to_string();
                assert!(error.contains("injected wait failure"));
                if path == "recovery" {
                    assert!(error.contains(if confirmed {
                        "and reaped"
                    } else {
                        "could not be reaped"
                    }));
                }
            } else {
                assert_eq!(outcome.unwrap().token.unwrap().as_str(), "after");
            }
        }
    }
}

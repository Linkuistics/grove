//! The launch half of the crate's public interface, exercised end to end
//! against a fake child and no consumer at all.
//!
//! Every child here is `/bin/sh` running a script the test wrote, which is what
//! makes the seam real: nothing below knows what a session is, and the argv
//! always arrives the way a launcher's would — authored by a template, read out
//! of a configuration file. There is no `Argv` constructor, and these tests do
//! not want one.

use std::ffi::{OsStr, OsString};
use std::fs;
use std::os::unix::process::ExitStatusExt as _;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Duration;

use keyed_launch::{
    run, Channel, End, Escalation, Launch, Requirement, Slot, SlotRule, Templates, Vocabulary,
};
use tempfile::TempDir;

/// One required slot, so every template below has to name the script it runs
/// and nothing is smuggled in past expansion.
const SLOTS: [SlotRule<'static>; 1] = [SlotRule {
    name: "script",
    requirement: Requirement::ExactlyOnce,
}];

/// The escalation on test timescales. Long enough that a poll tick lands inside
/// each phase, short enough that the whole suite stays in single-digit seconds.
const FAST: Escalation = Escalation {
    grace: Duration::from_millis(600),
    kill_grace: Duration::from_millis(900),
};

/// The two signals of the escalation, by number, so the tests can say which
/// step ran without taking a `libc` dependency of their own.
const SIGTERM: i32 = 15;
const SIGKILL: i32 = 9;

struct Harness {
    dir: TempDir,
    templates: Templates,
}

impl Harness {
    /// A configuration whose one key runs `sh <script>`, and a control directory
    /// for the channel.
    fn new() -> Self {
        let dir = TempDir::new().unwrap();
        let config = dir.path().join("config.kdl");
        fs::write(&config, "child \"sh ${script}\"\n").unwrap();
        let templates = Templates::load(&config, None, Vocabulary { slots: &SLOTS }).unwrap();
        fs::create_dir(dir.path().join("control")).unwrap();
        Self { dir, templates }
    }

    fn control(&self) -> PathBuf {
        self.dir.path().join("control")
    }

    fn script(&self, body: &str) -> PathBuf {
        let path = self.dir.path().join("child.sh");
        fs::write(&path, body).unwrap();
        path
    }

    /// Expand the one key against a script — the only route to an `Argv`.
    fn argv(&self, script: &Path) -> keyed_launch::Argv {
        self.templates
            .expand(
                "child",
                &[Slot {
                    name: "script",
                    value: script.as_os_str(),
                }],
            )
            .unwrap()
    }
}

fn launch<'a>(
    argv: &'a keyed_launch::Argv,
    channel: &'a Channel,
    scrub: &'a [&'a OsStr],
    cwd: Option<&'a Path>,
) -> Launch<'a> {
    Launch {
        argv,
        channel,
        channel_var: "TEST_CHANNEL",
        scrub,
        cwd,
        escalation: FAST,
    }
}

// ---------------------------------------------------------------------------
// The channel reaches the child, and the child's token comes back

/// The whole loop in one case: the launcher publishes a path, the child writes
/// a token to it, and the launcher reads that token back. Nothing in between
/// knows what "done" means.
#[test]
fn a_child_signals_through_the_published_path_and_the_token_comes_back() {
    let harness = Harness::new();
    let script = harness.script("printf 'done\\n' > \"$TEST_CHANNEL\"\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert_eq!(ended.token.as_ref().map(|t| t.as_str()), Some("done"));
    assert!(ended.status.success());
    assert_eq!(
        ended.end,
        End::Exited,
        "the child exited on its own before the grace elapsed, so nothing escalated"
    );
}

#[test]
fn a_child_that_never_signals_ends_with_no_token() {
    let harness = Harness::new();
    let script = harness.script("exit 3\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert_eq!(ended.token, None);
    assert_eq!(ended.end, End::Exited);
    assert_eq!(ended.status.code(), Some(3));
    assert!(
        !channel.path().exists(),
        "an unsignalled launch leaves no channel file"
    );
}

/// A child that signals and then takes its time still exits on its own terms,
/// so nothing escalates and `End` says so — even though a token came back.
///
/// The grace is long here on purpose: the child's own exit has to land well
/// inside it, or the case under test is not the case being run.
#[test]
fn a_child_that_signals_and_exits_inside_the_grace_is_never_touched() {
    let harness = Harness::new();
    let script = harness.script("printf 'done\\n' > \"$TEST_CHANNEL\"\nsleep 1\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let patient = Escalation {
        grace: Duration::from_secs(30),
        kill_grace: FAST.kill_grace,
    };

    let ended = run(Launch {
        escalation: patient,
        ..launch(&argv, &channel, &[], None)
    })
    .unwrap();

    assert_eq!(ended.token.as_ref().map(|t| t.as_str()), Some("done"));
    assert_eq!(
        ended.end,
        End::Exited,
        "a token is not an escalation — nothing was sent to this child"
    );
    assert!(ended.status.success());
    assert!(
        ended.elapsed < patient.grace,
        "the run must not have waited out the grace: {:?}",
        ended.elapsed
    );
}

// ---------------------------------------------------------------------------
// The escalation

/// The case the escalation exists for: a child that has signalled and then goes
/// on waiting forever, exactly as an interactive one does when it returns to
/// its prompt.
#[test]
fn a_signalled_child_that_keeps_waiting_is_terminated_after_the_grace() {
    let harness = Harness::new();
    // A loop of short sleeps rather than one long one: the child is killed
    // mid-wait, and a long-lived grandchild would go on holding the inherited
    // stdout pipe after its parent is reaped — which hangs the test *runner*,
    // not the test.
    let script = harness
        .script("printf 'relaunch\\n' > \"$TEST_CHANNEL\"\nwhile : ; do sleep 0.05 ; done\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert_eq!(ended.end, End::Signalled);
    assert_eq!(ended.token.as_ref().map(|t| t.as_str()), Some("relaunch"));
    assert!(
        !ended.status.success(),
        "a terminated child does not exit successfully: {:?}",
        ended.status
    );
    assert!(
        ended.elapsed >= FAST.grace,
        "the grace must elapse before anything is sent: {:?}",
        ended.elapsed
    );
    // Which signal ended it, rather than how long it took. The elapsed time
    // cannot separate the two steps: the poll interval is coarser than either
    // grace, so a child that dies promptly on SIGTERM can still be observed
    // later than `grace + kill_grace`. The signal says exactly which step ran.
    assert_eq!(
        ended.status.signal(),
        Some(SIGTERM),
        "a child that dies on SIGTERM must never reach SIGKILL"
    );
}

/// SIGTERM is a request. A child that declines it is still ended, which is what
/// makes the second step of the escalation load-bearing rather than defensive.
#[test]
fn a_child_that_ignores_sigterm_is_killed_after_the_kill_grace() {
    let harness = Harness::new();
    let script = harness.script(
        "trap '' TERM\nprintf 'done\\n' > \"$TEST_CHANNEL\"\nwhile : ; do sleep 0.05 ; done\n",
    );
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert_eq!(ended.end, End::Signalled);
    assert_eq!(
        ended.status.signal(),
        Some(SIGKILL),
        "a child that declines SIGTERM must still be ended"
    );
    assert!(
        ended.elapsed >= FAST.grace + FAST.kill_grace,
        "SIGKILL must wait out both graces: {:?}",
        ended.elapsed
    );
}

/// An unsignalled child is never touched: the token, not a timeout, is what
/// authorises the escalation.
#[test]
fn an_unsignalled_child_runs_to_its_own_exit_untouched() {
    let harness = Harness::new();
    let script = harness.script("sleep 1\nexit 0\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert!(
        ended.status.success(),
        "an unsignalled child must exit on its own terms: {:?}",
        ended.status
    );
    assert_eq!(ended.end, End::Exited);
}

// ---------------------------------------------------------------------------
// The environment the child receives

/// Scrubbing is a **removal**, not an omission. The distinction is the whole
/// point: an environment is inherited, so a launcher that merely declines to
/// set a variable still passes on whatever its own held.
///
/// The two variables are `HOME` and `PATH` rather than invented ones, because
/// inventing them would mean writing the process environment — a global that
/// every parallel sibling test's own spawn reads at the same moment, which
/// hangs the runner rather than failing the test. These two are inherited by
/// construction, which is exactly the property under test.
#[test]
fn a_scrubbed_variable_is_removed_from_an_inherited_environment() {
    assert!(
        std::env::var_os("HOME").is_some() && std::env::var_os("PATH").is_some(),
        "this test needs both variables present in its own environment to have anything to say"
    );
    let harness = Harness::new();
    let script = harness
        .script("printf '%s|%s\\n' \"${PATH-<unset>}\" \"${HOME-<unset>}\" > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let scrub: [&OsStr; 1] = [OsStr::new("HOME")];

    let ended = run(launch(&argv, &channel, &scrub, None)).unwrap();

    let reported = ended.token.expect("the child reported its environment");
    let (path, home) = reported.as_str().split_once('|').unwrap();
    assert_eq!(
        home, "<unset>",
        "a scrubbed variable must be removed, not merely left unset"
    );
    assert_eq!(
        path,
        std::env::var("PATH").unwrap(),
        "everything else must arrive by ordinary inheritance"
    );
}

/// The grant survives a scrub list that names the channel variable itself.
///
/// That is the expected shape rather than a caller's mistake: the scrub list is
/// the launch-control variables a nested launcher must not inherit, and the
/// channel variable is the first of them. If the grant were applied before the
/// scrub, this launch would remove the path it had just published and the child
/// could never signal — which shows up as a session that hangs, not as an error.
#[test]
fn granting_the_channel_survives_a_scrub_list_that_names_it() {
    let harness = Harness::new();
    let script = harness.script("printf 'done\\n' > \"${TEST_CHANNEL?unset}\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let scrub: [&OsStr; 2] = [OsStr::new("TEST_CHANNEL"), OsStr::new("HOME")];

    let ended = run(launch(&argv, &channel, &scrub, None)).unwrap();

    assert_eq!(
        ended.token.as_ref().map(|t| t.as_str()),
        Some("done"),
        "the child could not reach its channel, so the grant was scrubbed away"
    );
}

#[test]
fn the_channel_path_is_published_under_the_callers_chosen_variable_name() {
    let harness = Harness::new();
    // Written through the *caller's* name and read back from the channel the
    // launcher holds: the two agree only because `channel_var` carried it.
    let script = harness.script("printf '%s\\n' \"$TEST_CHANNEL\" > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert_eq!(
        ended.token.map(|t| t.into_string()),
        Some(channel.path().display().to_string())
    );
}

#[test]
fn the_child_starts_in_the_given_directory() {
    let harness = Harness::new();
    let elsewhere = harness.dir.path().join("elsewhere");
    fs::create_dir(&elsewhere).unwrap();
    let script = harness.script("pwd -P > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], Some(&elsewhere))).unwrap();

    let reported = PathBuf::from(ended.token.unwrap().into_string());
    assert_eq!(
        reported.canonicalize().unwrap(),
        elsewhere.canonicalize().unwrap()
    );
}

// ---------------------------------------------------------------------------
// Refusals

#[test]
fn a_program_that_does_not_exist_names_itself_and_says_what_to_check() {
    let dir = TempDir::new().unwrap();
    let config = dir.path().join("config.kdl");
    fs::write(&config, "child \"no-such-program-anywhere ${script}\"\n").unwrap();
    let templates = Templates::load(&config, None, Vocabulary { slots: &SLOTS }).unwrap();
    let argv = templates
        .expand(
            "child",
            &[Slot {
                name: "script",
                value: OsStr::new("x"),
            }],
        )
        .unwrap();
    fs::create_dir(dir.path().join("control")).unwrap();
    let channel = Channel::allocate(&dir.path().join("control")).unwrap();

    let error = run(launch(&argv, &channel, &[], None)).unwrap_err();

    let message = error.to_string();
    assert!(message.contains("no-such-program-anywhere"), "{message}");
    assert!(message.contains("executable"), "{message}");
}

/// Successive launches in one directory get channels that name them alone, so
/// one launch can never read the token another left.
#[test]
fn successive_launches_get_independent_channels() {
    let harness = Harness::new();
    let script = harness.script("printf '%s\\n' \"$TEST_CHANNEL\" > \"$TEST_CHANNEL\"\n");
    let argv = harness.argv(&script);

    let first = Channel::allocate(&harness.control()).unwrap();
    let first_ended = run(launch(&argv, &first, &[], None)).unwrap();
    let second = Channel::allocate(&harness.control()).unwrap();
    let second_ended = run(launch(&argv, &second, &[], None)).unwrap();

    assert_ne!(first.path(), second.path());
    assert_ne!(first_ended.token, second_ended.token);
    assert_eq!(
        second.read(),
        second_ended.token,
        "the second launch's channel holds the second launch's token"
    );

    first.discard().unwrap();
    second.discard().unwrap();
    assert_eq!(
        fs::read_dir(harness.control()).unwrap().count(),
        0,
        "a discarded channel leaves nothing behind"
    );
}

/// The launcher's `OsString` argv reaches the child unaltered — nothing is
/// re-split, quoted or handed to a shell on the way.
#[test]
fn arguments_reach_the_child_as_written() {
    let harness = Harness::new();
    let script = harness.script("printf '%s\\n' \"$1\" > \"$TEST_CHANNEL\"\n");
    let dir = harness.dir.path().to_path_buf();
    let config = dir.join("literal.kdl");
    fs::write(&config, "child \"sh ${script} 'one two  three'\"\n").unwrap();
    let templates = Templates::load(&config, None, Vocabulary { slots: &SLOTS }).unwrap();
    let argv = templates
        .expand(
            "child",
            &[Slot {
                name: "script",
                value: script.as_os_str(),
            }],
        )
        .unwrap();
    assert_eq!(argv.args().len(), 2, "{:?}", argv.args());
    assert_eq!(argv.args()[1], OsString::from("one two  three"));
    let channel = Channel::allocate(&harness.control()).unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    assert_eq!(
        ended.token.as_ref().map(|t| t.as_str()),
        Some("one two  three")
    );
}

// ---------------------------------------------------------------------------
// The child is a job of its own: its dispositions and its process group

/// A path interpolated into a fixture script, single-quoted.
///
/// Every path below is a `TempDir`'s, so this only has to survive spaces —
/// `/var/folders/…` on macOS. A path containing a single quote would need the
/// real dance, and no fixture here can produce one.
fn quoted(path: &Path) -> String {
    format!("'{}'", path.display())
}

/// Ignore SIGINT for as long as this value lives — a launcher's own policy,
/// which is what a driver that must survive a terminal Ctrl-C actually sets.
///
/// Restored on drop, because a disposition is process-global and every other
/// test in this binary shares it.
struct IgnoringSigint(libc::sighandler_t);

impl IgnoringSigint {
    fn new() -> Self {
        // SAFETY: `signal(2)` setting and later restoring one disposition.
        Self(unsafe { libc::signal(libc::SIGINT, libc::SIG_IGN) })
    }
}

impl Drop for IgnoringSigint {
    fn drop(&mut self) {
        // SAFETY: restoring the disposition this value replaced.
        unsafe { libc::signal(libc::SIGINT, self.0) };
    }
}

/// **An ignored disposition is the one kind that survives `execve`.** A
/// launcher that ignores SIGINT for its own reasons hands the ignore to its
/// child, to that child's children, and to every wrapper the template names —
/// and a non-interactive shell that inherits an ignored SIGINT keeps ignoring
/// it *and forces it on what it spawns*. An interactive session under
/// `sh -lc '…'` then cannot be interrupted at all, and nothing in it can say
/// why.
///
/// The child here **reports what it inherited** rather than installing a
/// handler of its own, which is the only fixture that can see the fault: a
/// child that installs a handler overwrites the inherited disposition, so it
/// behaves identically whether or not the launcher leaked one. `kill -INT $$`
/// against the inherited disposition is that report — the default action kills
/// a non-interactive shell, and an inherited ignore lets it run on to the line
/// below.
#[test]
fn an_ignored_sigint_in_the_launcher_does_not_reach_the_child() {
    const SIGINT: i32 = 2;

    let harness = Harness::new();
    let survived = harness.dir.path().join("survived-sigint");
    let script = harness.script(&format!(
        "kill -INT $$\nprintf 'inherited\\n' > {marker}\n",
        marker = quoted(&survived)
    ));

    let ignoring = IgnoringSigint::new();

    // The positive control, and it runs first on purpose. The same script under
    // a plain `Command` — no runner, no reset — *does* inherit the ignore and
    // writes the marker. Without seeing that, the assertion below would pass
    // just as well on a fixture that could never detect an inherited ignore at
    // all, which is the one thing it exists to rule out.
    let control = Command::new("sh").arg(&script).status().unwrap();
    assert!(
        survived.exists(),
        "control: a plain spawn must inherit the launcher's ignored SIGINT — \
         the fixture cannot detect the fault it is testing for (status {control:?})"
    );
    fs::remove_file(&survived).unwrap();

    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();
    let ended = run(launch(&argv, &channel, &[], None)).unwrap();

    drop(ignoring);

    assert!(
        !survived.exists(),
        "the child inherited the launcher's ignored SIGINT: a session under a \
         wrapper cannot be interrupted, and neither can anything it spawns"
    );
    assert_eq!(
        ended.status.signal(),
        Some(SIGINT),
        "the child must have died of the SIGINT it sent itself, which is what \
         the default disposition means: {:?}",
        ended.status
    );
}

/// The escalation signals the child's **process group**, so a grandchild — a
/// tool subprocess, a language server, an agent's own in-flight command — is
/// reaped with its parent instead of surviving it and staying attached to the
/// terminal. A survivor is not merely untidy: it can hold a lock the launcher's
/// caller is about to wait on, and then the SIGKILL buys a stall rather than a
/// teardown.
#[test]
fn the_escalation_reaps_the_childs_descendants() {
    let harness = Harness::new();
    let grandchild_pid = harness.dir.path().join("grandchild-pid");
    // Signals, then declines to end: the launch reaches the full escalation,
    // and what it reaches for is what this test is about.
    let script = harness.script(&format!(
        "sh -c 'while : ; do sleep 0.05 ; done' &\n\
         printf '%s\\n' \"$!\" > {pid}\n\
         : > \"$TEST_CHANNEL\"\n\
         while : ; do sleep 0.05 ; done\n",
        pid = quoted(&grandchild_pid)
    ));
    let argv = harness.argv(&script);
    let channel = Channel::allocate(&harness.control()).unwrap();

    // The cross-check: the same shape of process, started at the same moment,
    // in *this* process's group rather than the child's. It must be untouched.
    // A fixture that reported "gone" for any pid — a `kill(2)` probe reading
    // the wrong errno, say — would look identical without it.
    let mut bystander = Command::new("sh")
        .arg("-c")
        .arg("while : ; do sleep 0.05 ; done")
        .spawn()
        .unwrap();

    let ended = run(launch(&argv, &channel, &[], None)).unwrap();
    assert_eq!(ended.end, End::Signalled);

    let grandchild: i32 = fs::read_to_string(&grandchild_pid)
        .expect("the fixture never reported its grandchild")
        .trim()
        .parse()
        .unwrap();
    assert!(
        gone(grandchild),
        "the grandchild outlived the escalation that killed its parent"
    );
    assert!(
        bystander.try_wait().unwrap().is_none(),
        "the escalation reached a process outside the launched job's group"
    );

    bystander.kill().unwrap();
    bystander.wait().unwrap();
}

/// Whether `pid` is gone, waiting out the moment between its parent's death and
/// `init` reaping it.
fn gone(pid: i32) -> bool {
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        // SAFETY: `kill(2)` with signal 0 — the existence probe, which sends
        // nothing.
        if unsafe { libc::kill(pid, 0) } == -1
            && std::io::Error::last_os_error().raw_os_error() == Some(libc::ESRCH)
        {
            return true;
        }
        if std::time::Instant::now() >= deadline {
            return false;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

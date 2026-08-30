mod support;

use grove::driver_lease::DriverLease;
use std::fs;
use std::io::Read;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::Mutex;
use std::thread;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const HOLDER_ROOT: &str = "GROVE_TEST_LEASE_HOLDER_ROOT";
const HOLDER_READY: &str = "GROVE_TEST_LEASE_HOLDER_READY";
const HOLDER_PANIC: &str = "GROVE_TEST_LEASE_HOLDER_PANIC";
const EXEC_READY: &str = "GROVE_TEST_LEASE_EXEC_READY";
const EXEC_RELEASED: &str = "GROVE_TEST_LEASE_EXEC_RELEASED";

static ENV_LOCK: Mutex<()> = Mutex::new(());

const SESSION_KINDS: &[&str] = &[
    "requirements",
    "review-requirements",
    "integrate-review-requirements",
    "design",
    "review-design",
    "integrate-review-design",
    "planning",
    "review-planning",
    "integrate-review-planning",
    "prototype",
    "review-prototype",
    "integrate-review-prototype",
    "impl",
    "review-impl",
    "integrate-review-impl",
    "research-a",
    "research-b",
    "combine-research",
    "finish",
];

/// A bare `.jj` marker: everything the resolver needs, and nothing a jj command
/// could act on. The lease is untracked coordination, so most cases here need
/// the marker rather than a repository.
fn fake_jj_worktree(path: &Path) {
    fs::create_dir_all(path.join(".jj")).unwrap();
}

/// A **colocated** jj workspace — real on both markers, so a child that inherits
/// ambient `GIT_*` has a repository those variables can genuinely select.
fn init_colocated_worktree(path: &Path) {
    fs::create_dir_all(path).unwrap();
    run_command(
        "jj",
        path,
        &[
            "--config",
            "user.name=Grove Test",
            "--config",
            "user.email=grove-test@example.com",
            "git",
            "init",
            "--colocate",
            "--quiet",
            ".",
        ],
    );
}

fn run_command(binary: &str, directory: &Path, arguments: &[&str]) {
    let output = Command::new(binary)
        .args(arguments)
        .current_dir(directory)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn command_stdout(binary: &str, directory: &Path, arguments: &[&str]) -> String {
    let output = Command::new(binary)
        .args(arguments)
        .current_dir(directory)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{binary} {arguments:?} failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    let mut permissions = fs::metadata(path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

fn grove_driver(root: &Path, harness: &Path, home: &Path) -> Command {
    fs::create_dir_all(home.join(".codex")).unwrap();
    let config_dir = home.join(".config/grove");
    fs::create_dir_all(&config_dir).unwrap();
    let template = format!("{} '${{prompt}}'", shell_quote(harness));
    let document = SESSION_KINDS
        .iter()
        .map(|kind| format!("{kind} {template:?}\n"))
        .collect::<String>();
    fs::write(config_dir.join("config.kdl"), document).unwrap();

    let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
    command.current_dir(root);
    for name in support::grove_env_names() {
        command.env_remove(name);
    }
    command.env("HOME", home);
    command
}

fn stop_driver(child: &mut Child) -> bool {
    unsafe {
        libc::kill(child.id() as libc::pid_t, libc::SIGTERM);
    }
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if child.try_wait().unwrap().is_some() {
            return true;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            let _ = child.wait();
            return false;
        }
        thread::sleep(Duration::from_millis(10));
    }
}

struct DriverProcess {
    child: Option<Child>,
    diagnostics: PathBuf,
}

impl DriverProcess {
    /// Spawn a driver with both its streams captured to `diagnostics` instead of
    /// discarded. A driver that stops before its session ever starts is exactly
    /// the failure a readiness wait has to report, and `Stdio::null()` threw
    /// away the only account of why (driver-lease-readiness-flake-k145).
    fn spawn(command: &mut Command, diagnostics: &Path) -> Self {
        let log = fs::File::create(diagnostics).unwrap();
        command
            .stdout(Stdio::from(log.try_clone().unwrap()))
            .stderr(Stdio::from(log));
        Self {
            child: Some(command.spawn().unwrap()),
            diagnostics: diagnostics.to_path_buf(),
        }
    }

    fn wait_for_ready(&mut self, path: &Path) {
        let diagnostics = self.diagnostics.clone();
        let driver = self.child.as_mut().expect("a spawned driver");
        support::wait_for_ready(path, driver, Some(&diagnostics));
    }
}

impl Drop for DriverProcess {
    fn drop(&mut self) {
        if let Some(child) = self.child.as_mut() {
            let _ = stop_driver(child);
        }
    }
}

fn lease_path(root: &Path) -> PathBuf {
    root.join(".jj/grove/driver.lease")
}

fn epoch_path(root: &Path) -> PathBuf {
    root.join(".jj/grove/session.epoch")
}

fn record_nonce(record: &str) -> &str {
    record
        .lines()
        .find_map(|line| line.strip_prefix("nonce="))
        .unwrap()
}

/// The fixed readiness deadline this binary used to impose on every wait, kept
/// only so the regressions below can hold a healthy producer back for longer
/// than it — a test that stays inside the old budget would have passed against
/// the bug it pins. `tests/loop_driver.rs` carried the same mistake at twenty
/// seconds, which these do *not* outlast: a twenty-one-second regression buys
/// nothing the six-second one has not already shown about the seam, and that
/// deadline was never observed expiring (loop-driver-readiness-deadline-k170).
const REMOVED_READINESS_DEADLINE: Duration = Duration::from_secs(5);

fn spawn_producer(script: &str, diagnostics: Option<&Path>) -> Child {
    let mut command = Command::new("sh");
    command.args(["-c", script]).stdin(Stdio::null());
    match diagnostics {
        Some(path) => {
            let log = fs::File::create(path).unwrap();
            command
                .stdout(Stdio::from(log.try_clone().unwrap()))
                .stderr(Stdio::from(log));
        }
        None => {
            command.stdout(Stdio::null()).stderr(Stdio::null());
        }
    }
    command.spawn().unwrap()
}

// The three regressions below pin `support::readiness` itself, because every
// process test that depends on it — in this binary and in `tests/loop_driver.rs`
// — is asserting about something else and none of them can express what the
// seam must get right. They live here rather than beside the seam because
// `tests/support/mod.rs` is compiled into every consumer binary, so a `#[test]`
// there would run a copy per binary; this is the binary whose fixtures can hold
// a producer back past the removed deadline, so it is the one home they get
// (loop-driver-readiness-deadline-k170).
//
// The first two are the directions the removed fixed deadline got wrong: it
// failed healthy producers that were merely slow, and it reported dead ones only
// after waiting out a budget they were never going to meet
// (driver-lease-readiness-flake-k145).

#[test]
fn a_producer_slower_than_the_removed_deadline_is_still_waited_for() {
    let tmp = TempDir::new().unwrap();
    let ready = tmp.path().join("ready");
    let mut producer = spawn_producer(
        &format!(
            "sleep {}; : > {}",
            REMOVED_READINESS_DEADLINE.as_secs() + 1,
            shell_quote(&ready)
        ),
        None,
    );

    let started = Instant::now();
    support::readiness(&ready, &mut producer, None).expect("a live producer must be waited for");

    assert!(
        started.elapsed() > REMOVED_READINESS_DEADLINE,
        "this regression has to outlast the deadline it pins, or it also passes against the bug"
    );
    assert!(producer.wait().unwrap().success());
}

#[test]
fn a_producer_that_ends_without_writing_is_reported_when_it_is_observed() {
    let tmp = TempDir::new().unwrap();
    let never_written = tmp.path().join("ready");
    let diagnostics = tmp.path().join("diagnostics");
    let mut producer = spawn_producer(
        "printf 'grove: config.kdl is missing\\n' >&2; exit 3",
        Some(&diagnostics),
    );

    let started = Instant::now();
    let failure = support::readiness(&never_written, &mut producer, Some(&diagnostics))
        .expect_err("a producer that has ended can never write its readiness file");

    assert!(
        started.elapsed() < REMOVED_READINESS_DEADLINE,
        "an ended producer must be reported when it is observed, not waited out: {failure}"
    );
    assert!(
        failure.contains("exit status: 3"),
        "the failure must say how the producer ended: {failure}"
    );
    assert!(
        failure.contains("grove: config.kdl is missing"),
        "the failure must quote what the producer said: {failure}"
    );
}

/// The third pins the correction that making an ended producer a failure at all
/// invites: *a readiness file outlives the process that wrote it*, so an ended
/// producer is only a failure once its file has been looked for. The lease
/// holder in `owner_panic_releases_the_lease_during_unwind` writes and dies in
/// the same breath, and its parent's first poll routinely lands after both — so
/// an implementation that returns as soon as `try_wait` yields a status fails
/// that test intermittently and this one every time.
///
/// It cannot see the subtler sibling of that mistake, reading the file *before*
/// the producer's status: the file already exists here, so both orders pass.
/// That ordering is argued rather than tested, in [`readiness`]'s own comment.
#[test]
fn a_file_written_by_a_producer_that_has_already_ended_is_ready() {
    let tmp = TempDir::new().unwrap();
    let ready = tmp.path().join("ready");
    let mut producer = spawn_producer(&format!(": > {}", shell_quote(&ready)), None);
    assert!(producer.wait().unwrap().success());

    support::readiness(&ready, &mut producer, None)
        .expect("a readiness file outlives the process that wrote it");
}

struct Holder {
    child: Child,
}

impl Holder {
    fn spawn(root: &Path, ready: &Path) -> Self {
        let mut child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "lease_holder_process", "--nocapture"])
            .env(HOLDER_ROOT, root)
            .env(HOLDER_READY, ready)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        support::wait_for_ready(ready, &mut child, None);
        Self { child }
    }

    fn release_normally(mut self) {
        drop(self.child.stdin.take());
        assert!(self.child.wait().unwrap().success());
    }

    fn kill(mut self) {
        self.child.kill().unwrap();
        let _ = self.child.wait();
    }
}

impl Drop for Holder {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

#[test]
fn lease_holder_process() {
    let Ok(root) = std::env::var(HOLDER_ROOT) else {
        return;
    };
    let ready = PathBuf::from(std::env::var_os(HOLDER_READY).unwrap());
    let lease = DriverLease::acquire(Path::new(&root)).unwrap();
    fs::write(&ready, b"ready").unwrap();

    if std::env::var_os(HOLDER_PANIC).is_some() {
        panic!("intentional lease-holder panic");
    }

    if let Some(exec_ready) = std::env::var_os(EXEC_READY) {
        let exec_ready = PathBuf::from(exec_ready);
        let script = format!("printf ready > '{}'; sleep 30", exec_ready.display());
        let mut descendant = Command::new("sh")
            .args(["-c", &script])
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        support::wait_for_ready(&exec_ready, &mut descendant, None);
        drop(lease);
        let released = PathBuf::from(std::env::var_os(EXEC_RELEASED).unwrap());
        fs::write(released, b"released").unwrap();
        let mut input = Vec::new();
        std::io::stdin().read_to_end(&mut input).unwrap();
        let _ = descendant.kill();
        let _ = descendant.wait();
        return;
    }

    let mut input = Vec::new();
    std::io::stdin().read_to_end(&mut input).unwrap();
}

#[test]
fn an_alias_equivalent_second_owner_is_refused_immediately() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let alias = tmp.path().join("alias");
    std::os::unix::fs::symlink(&root, &alias).unwrap();
    let ready = tmp.path().join("ready");
    let holder = Holder::spawn(&root, &ready);

    let started = Instant::now();
    let error = DriverLease::acquire(&alias).unwrap_err();

    assert!(started.elapsed() < Duration::from_secs(1));
    assert!(
        error
            .to_string()
            .contains("existing Grove driver must stop"),
        "unexpected error: {error:#}"
    );
    assert!(error.to_string().contains(root.to_str().unwrap()));
    drop(holder);
}

#[test]
fn driver_uses_the_on_disk_worktree_while_the_configured_command_inherits_git_context() {
    let tmp = TempDir::new().unwrap();
    let intended = tmp.path().join("intended");
    let foreign = tmp.path().join("foreign");
    init_colocated_worktree(&intended);
    init_colocated_worktree(&foreign);
    // A Git-side redirection Grove never consults: resolution walks for `.jj`,
    // so the session's own `git` view can point somewhere else entirely and the
    // driver still drives the tree on disk. That divergence is the assertion.
    run_command(
        "git",
        &intended,
        &["config", "core.worktree", foreign.to_str().unwrap()],
    );
    fs::create_dir_all(intended.join(".grove")).unwrap();
    fs::write(intended.join(".grove/BRIEF.md"), "# intended — brief\n").unwrap();
    fs::write(intended.join(".grove/01-impl--test-k1.md"), "# test-k1\n").unwrap();

    let launch_log = tmp.path().join("launch-log");
    let fake_harness = tmp.path().join("fake-claude.sh");
    let quoted_log = shell_quote(&launch_log);
    write_executable(
        &fake_harness,
        &format!(
            "#!/bin/sh\n\
             printf 'cwd=%s\\n' \"$(pwd -P)\" > {quoted_log}\n\
             printf 'git_dir=%s\\n' \"${{GIT_DIR-<unset>}}\" >> {quoted_log}\n\
             printf 'git_work_tree=%s\\n' \"${{GIT_WORK_TREE-<unset>}}\" >> {quoted_log}\n\
             printf 'git_common_dir=%s\\n' \"${{GIT_COMMON_DIR-<unset>}}\" >> {quoted_log}\n\
             unset GIT_DIR GIT_WORK_TREE GIT_COMMON_DIR\n\
             printf 'configured_top=%s\\n' \"$(git rev-parse --show-toplevel)\" >> {quoted_log}\n"
        ),
    );
    let home = tmp.path().join("home");

    let output = grove_driver(&intended, &fake_harness, &home)
        .env("GIT_DIR", foreign.join(".git"))
        .env("GIT_WORK_TREE", &foreign)
        .env("GIT_COMMON_DIR", foreign.join(".git"))
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "driver rejected its on-disk worktree: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        launch_log.exists(),
        "driver did not launch the intended grove"
    );
    let expected = format!(
        "cwd={}\ngit_dir={}\ngit_work_tree={}\ngit_common_dir={}\nconfigured_top={}\n",
        intended.canonicalize().unwrap().display(),
        foreign.join(".git").display(),
        foreign.display(),
        foreign.join(".git").display(),
        foreign.canonicalize().unwrap().display()
    );
    assert_eq!(fs::read_to_string(&launch_log).unwrap(), expected);
    assert!(
        !intended.join(".grove-stamps").exists(),
        "config-driven launch must not create a harness stamp"
    );
    assert!(
        !foreign.join(".grove-stamps").exists(),
        "driver wrote launch state into the environment-selected repository"
    );
}

#[test]
fn bare_scaffolding_is_anchored_before_the_configured_command_inherits_git_context() {
    let tmp = TempDir::new().unwrap();
    let intended = tmp.path().join("intended");
    let foreign = tmp.path().join("foreign");
    init_colocated_worktree(&intended);
    init_colocated_worktree(&foreign);
    // **No `.grove/` at all**, so the driver's lifecycle transition has a
    // mutation to anchor: creating the grove. It is the only tree mutation the
    // driver still performs — migration is gone (`delete-migration-k6`), and the
    // partial-scaffold repair that used to serve as this fixture went with the
    // window it closed (`collapse-tree-access-k13`).

    let launch_log = tmp.path().join("launch-log");
    let fake_harness = tmp.path().join("fake-claude.sh");
    write_executable(
        &fake_harness,
        &format!(
            "#!/bin/sh\nprintf '%s\\n%s\\n%s\\n' \"$GIT_DIR\" \"$GIT_WORK_TREE\" \"$GIT_COMMON_DIR\" > {}\n",
            shell_quote(&launch_log)
        ),
    );
    let home = tmp.path().join("home");

    let output = grove_driver(&intended, &fake_harness, &home)
        .env("GIT_DIR", foreign.join(".git"))
        .env("GIT_WORK_TREE", &foreign)
        .env("GIT_COMMON_DIR", foreign.join(".git"))
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "bare scaffolding was redirected by ambient Git context: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(&launch_log).unwrap(),
        format!(
            "{}\n{}\n{}\n",
            foreign.join(".git").display(),
            foreign.display(),
            foreign.join(".git").display()
        )
    );
    assert!(intended
        .join(".grove/01-requirements--plan-k1.md")
        .is_file());
    assert!(!foreign.join(".grove").exists());
}

#[test]
fn distinct_worktrees_hold_independent_leases() {
    let tmp = TempDir::new().unwrap();
    let first = tmp.path().join("first");
    let second = tmp.path().join("second");
    fake_jj_worktree(&first);
    fake_jj_worktree(&second);
    let holder = Holder::spawn(&first, &tmp.path().join("ready"));

    let second_lease = DriverLease::acquire(&second).unwrap();

    second_lease.revalidate().unwrap();
    drop(holder);
}

/// One default/secondary workspace pair per shape Grove supports — native jj and
/// colocated jj. Both derive `.jj/grove` from their **own** root, which is the
/// property under test: two workspaces of one repository must not share a
/// control directory, or the second driver is refused for no reason. The
/// colocated pair is the one carrying `.git` *and* `.jj` at the same root, so a
/// resolution that ever looked at `.git` again would collapse them.
fn worktree_shape_pairs(tmp: &Path) -> Vec<(&'static str, PathBuf, PathBuf)> {
    fn init_jj(path: &Path, colocate: bool) {
        fs::create_dir_all(path).unwrap();
        run_command(
            "jj",
            path,
            &[
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
            ],
        );
    }

    fn add_jj_workspace(default: &Path, secondary: &Path) {
        run_command(
            "jj",
            default,
            &[
                "--config",
                "user.name=Test",
                "--config",
                "user.email=t@example.com",
                "workspace",
                "add",
                "--quiet",
                secondary.to_str().unwrap(),
            ],
        );
    }

    let jj_native = tmp.join("jj-native");
    init_jj(&jj_native, false);
    let jj_native_secondary = tmp.join("jj-native-secondary");
    add_jj_workspace(&jj_native, &jj_native_secondary);

    let jj_colocated = tmp.join("jj-colocated");
    init_jj(&jj_colocated, true);
    assert!(
        jj_colocated.join(".git").exists() && jj_colocated.join(".jj").is_dir(),
        "the colocated fixture must carry both markers or it tests nothing"
    );
    let jj_colocated_secondary = tmp.join("jj-colocated-secondary");
    add_jj_workspace(&jj_colocated, &jj_colocated_secondary);

    vec![
        ("native jj workspace", jj_native, jj_native_secondary),
        (
            "colocated jj workspace",
            jj_colocated,
            jj_colocated_secondary,
        ),
    ]
}

#[test]
fn every_worktree_shape_holds_independent_default_and_secondary_leases() {
    let tmp = TempDir::new().unwrap();

    for (shape, default, secondary) in worktree_shape_pairs(tmp.path()) {
        let default_lease = DriverLease::acquire(&default)
            .unwrap_or_else(|error| panic!("{shape} default: {error:#}"));
        let secondary_lease = DriverLease::acquire(&secondary)
            .unwrap_or_else(|error| panic!("{shape} secondary: {error:#}"));
        default_lease.revalidate().unwrap();
        secondary_lease.revalidate().unwrap();
    }
}

// The lease is keyed to a canonical worktree identity, not to the path the
// operator typed, and every shape derives that identity through its own branch.
// A symlinked route into any of them is therefore the same tree, and a second
// driver reaching it that way must be refused as immediately — and as
// informatively, naming the real root — as one that spelled the path out.
#[test]
fn an_alias_into_any_worktree_shape_is_refused_by_the_live_owner() {
    let tmp = TempDir::new().unwrap();

    for (shape, default, secondary) in worktree_shape_pairs(tmp.path()) {
        for (role, root) in [("default", &default), ("secondary", &secondary)] {
            let alias = tmp.path().join(format!(
                "alias-{}-{role}",
                root.file_name().unwrap().to_str().unwrap()
            ));
            std::os::unix::fs::symlink(root, &alias).unwrap();
            let ready = tmp.path().join(format!(
                "ready-{}-{role}",
                root.file_name().unwrap().to_str().unwrap()
            ));
            let holder = Holder::spawn(root, &ready);

            let started = Instant::now();
            let error = DriverLease::acquire(&alias)
                .expect_err(&format!("{shape} {role}: an alias was admitted twice"));

            assert!(
                started.elapsed() < Duration::from_secs(1),
                "{shape} {role}: a contended driver lease must be refused, not queued"
            );
            assert!(
                error
                    .to_string()
                    .contains("existing Grove driver must stop"),
                "{shape} {role}: unexpected error: {error:#}"
            );
            assert!(
                error
                    .to_string()
                    .contains(root.canonicalize().unwrap().to_str().unwrap()),
                "{shape} {role}: the refusal must name the real root, not the alias: {error:#}"
            );
            drop(holder);
        }
    }
}

#[test]
fn normal_owner_exit_releases_the_lease_without_cleanup() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let holder = Holder::spawn(&root, &tmp.path().join("ready"));

    holder.release_normally();

    DriverLease::acquire(&root).unwrap().revalidate().unwrap();
}

#[test]
fn forced_owner_exit_releases_the_lease_without_pid_cleanup() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let holder = Holder::spawn(&root, &tmp.path().join("ready"));

    holder.kill();

    DriverLease::acquire(&root).unwrap().revalidate().unwrap();
}

#[test]
fn owner_panic_releases_the_lease_during_unwind() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let ready = tmp.path().join("ready");
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "lease_holder_process", "--nocapture"])
        .env(HOLDER_ROOT, &root)
        .env(HOLDER_READY, &ready)
        .env(HOLDER_PANIC, "1")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    // The holder writes its readiness file and dies microseconds later, so this
    // wait routinely observes the exit first — the case
    // `a_file_written_by_a_producer_that_has_already_ended_is_ready` pins.
    support::wait_for_ready(&ready, &mut child, None);

    assert!(!child.wait().unwrap().success());
    DriverLease::acquire(&root).unwrap().revalidate().unwrap();
}

#[test]
fn revalidation_refuses_a_replaced_lease_path() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let lease = DriverLease::acquire(&root).unwrap();
    let path = lease_path(&root);
    fs::rename(&path, path.with_extension("replaced")).unwrap();
    fs::write(&path, b"replacement").unwrap();

    let error = lease.revalidate().unwrap_err();

    let error_chain = format!("{error:#}");
    assert!(
        error_chain.contains("driver lease path was replaced"),
        "unexpected error: {error_chain}"
    );
}

#[test]
fn each_owner_writes_one_fresh_128_bit_process_nonce() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);

    let first_owner = Holder::spawn(&root, &tmp.path().join("first-ready"));
    let first = fs::read_to_string(lease_path(&root)).unwrap();
    first_owner.release_normally();

    let second_owner = Holder::spawn(&root, &tmp.path().join("second-ready"));
    let second = fs::read_to_string(lease_path(&root)).unwrap();
    second_owner.release_normally();

    assert_ne!(record_nonce(&first), record_nonce(&second));
    assert_eq!(record_nonce(&first).len(), 32);
    assert!(record_nonce(&first)
        .bytes()
        .all(|byte| byte.is_ascii_hexdigit()));
    assert_eq!(
        first
            .lines()
            .filter(|line| line.starts_with("nonce="))
            .count(),
        1
    );
}

#[test]
fn a_new_owner_installs_an_inactive_epoch_for_its_exact_lease() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);

    let _lease = DriverLease::acquire(&root).unwrap();

    let lease_record = fs::read_to_string(lease_path(&root)).unwrap();
    let epoch_record = fs::read_to_string(epoch_path(&root)).unwrap();
    assert!(epoch_record.contains("state=inactive\n"));
    assert!(epoch_record.contains(&format!("nonce={}\n", record_nonce(&lease_record))));
    for line in lease_record
        .lines()
        .filter(|line| line.starts_with("worktree-"))
    {
        assert!(
            epoch_record.lines().any(|epoch_line| epoch_line == line),
            "epoch record omitted lease identity line {line:?}: {epoch_record:?}"
        );
    }
    assert!(
        !epoch_record
            .lines()
            .any(|line| line.starts_with("signal-path-hex=")),
        "inactive epochs must not retain a launch signal path: {epoch_record:?}"
    );
}

#[test]
fn a_non_directory_control_location_fails_before_a_lease_is_created() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    fs::write(root.join(".jj/grove"), b"not a directory").unwrap();

    let error = DriverLease::acquire(&root).unwrap_err();

    assert!(
        error.to_string().contains("is not usable"),
        "unexpected error: {error:#}"
    );
}

#[test]
fn an_unwritable_control_parent_fails_before_a_lease_is_created() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let jj_directory = root.join(".jj");
    fs::set_permissions(&jj_directory, fs::Permissions::from_mode(0o500)).unwrap();

    let result = DriverLease::acquire(&root);

    fs::set_permissions(&jj_directory, fs::Permissions::from_mode(0o700)).unwrap();
    let error = result.unwrap_err();
    assert!(
        error.to_string().contains("is not usable"),
        "unexpected error: {error:#}"
    );
    assert!(!lease_path(&root).exists());
}

#[test]
fn an_execed_descendant_does_not_inherit_driver_ownership() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_jj_worktree(&root);
    let ready = tmp.path().join("holder-ready");
    let exec_ready = tmp.path().join("exec-ready");
    let released = tmp.path().join("lease-released");
    let mut child = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "lease_holder_process", "--nocapture"])
        .env(HOLDER_ROOT, &root)
        .env(HOLDER_READY, &ready)
        .env(EXEC_READY, &exec_ready)
        .env(EXEC_RELEASED, &released)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    support::wait_for_ready(&ready, &mut child, None);
    support::wait_for_ready(&exec_ready, &mut child, None);
    support::wait_for_ready(&released, &mut child, None);

    DriverLease::acquire(&root).unwrap().revalidate().unwrap();

    drop(child.stdin.take());
    assert!(child.wait().unwrap().success());
}

#[test]
fn a_second_driver_refuses_before_tree_access_or_launch() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    init_colocated_worktree(&root);
    fs::create_dir_all(root.join(".claude")).unwrap();
    fs::create_dir_all(root.join(".grove")).unwrap();
    fs::write(root.join(".grove/BRIEF.md"), "# test — brief\n").unwrap();
    fs::write(root.join(".grove/01-impl--test-k1.md"), "# test-k1\n").unwrap();
    run_command("git", &root, &["add", ".grove"]);
    run_command(
        "git",
        &root,
        &[
            "-c",
            "user.name=Test",
            "-c",
            "user.email=t@example.com",
            "commit",
            "-q",
            "-m",
            "seed current grove",
        ],
    );

    let gate = tmp.path().join("harness-active");
    let harness_pid = tmp.path().join("harness-pid");
    let first_ready = tmp.path().join("first-ready");
    let duplicate_launch = tmp.path().join("duplicate-launch");
    let fake_harness = tmp.path().join("fake-claude.sh");
    write_executable(
        &fake_harness,
        &format!(
            "#!/bin/sh\nprintf '%s' \"$$\" > '{}'\nif mkdir '{}' 2>/dev/null; then\n  touch '{}'\n  trap 'exit 0' TERM\n  while :; do sleep 1; done\nelse\n  touch '{}'\n  exit 0\nfi\n",
            harness_pid.display(),
            gate.display(),
            first_ready.display(),
            duplicate_launch.display()
        ),
    );
    let home = tmp.path().join("home");
    let mut first_command = grove_driver(&root, &fake_harness, &home);
    let mut first = DriverProcess::spawn(&mut first_command, &tmp.path().join("first-driver-log"));
    first.wait_for_ready(&first_ready);
    first.wait_for_ready(&harness_pid);
    // Leave no tree at all — the one shape the lifecycle transition would
    // mutate, now that creating a grove is the only mutation it performs. A
    // driver that refuses before tree access leaves it exactly so.
    fs::remove_dir_all(root.join(".grove")).unwrap();
    let head_before_second = command_stdout("git", &root, &["rev-parse", "HEAD"]);

    let second = grove_driver(&root, &fake_harness, &home).output().unwrap();

    assert!(
        !second.status.success(),
        "second driver unexpectedly succeeded"
    );
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(
        stderr.contains("existing Grove driver must stop"),
        "unexpected stderr: {stderr}"
    );
    assert!(
        !duplicate_launch.exists(),
        "second driver launched a harness"
    );
    assert!(
        !root.join(".grove").exists(),
        "second driver scaffolded the live driver's task tree"
    );
    assert_eq!(
        command_stdout("git", &root, &["rev-parse", "HEAD"]),
        head_before_second,
        "second driver committed before acquiring ownership"
    );
    drop(first);
    let harness_pid: libc::pid_t = fs::read_to_string(harness_pid).unwrap().parse().unwrap();
    assert_eq!(
        unsafe { libc::kill(harness_pid, 0) },
        -1,
        "driver drop orphaned harness process {harness_pid}"
    );
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ESRCH)
    );
}

// A lease replaced under a running driver refuses rather than driving a working
// tree it no longer owns. The driver revalidates at two points — at the top of
// every loop transition and again immediately before the foreground launch — and
// this is the black-box witness for the first.
//
// **It used to witness the second, and no longer can.** The hook was the
// agent-CLI identity probe: the one subprocess the driver ran between owning the
// tree and launching, which a `PATH`-front shadow could reach and use to replace
// the lease inside that window. `delete-provisioning-k19` deleted the probe with
// the build-pairing report, and nothing else runs in that window, so a black-box
// fixture has no way to interpose there any more. The narrower witness below is
// what survives: the session itself replaces the lease, and the *next*
// transition refuses before selecting, mutating or launching again. The
// pre-launch call is still there and still tested in-process, through
// [`DriverLease::revalidate`] directly.
#[test]
fn a_lease_replaced_under_a_running_driver_refuses_the_next_transition() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    init_colocated_worktree(&root);
    fs::create_dir_all(root.join(".grove")).unwrap();
    fs::write(root.join(".grove/BRIEF.md"), "# test — brief\n").unwrap();
    fs::write(root.join(".grove/01-impl--test-k1.md"), "# test-k1\n").unwrap();

    let launch_log = tmp.path().join("launch-log");
    let configured = tmp.path().join("configured-command.sh");
    // One session: count the launch, replace the lease with a fresh file at the
    // same path, then signal so the driver takes another transition. The `mv`
    // and truncate together give the path a new identity without removing it —
    // the shape `revalidate` exists to catch, and the one a bare `rm` would not
    // produce.
    write_executable(
        &configured,
        &format!(
            "#!/bin/sh\nprintf x >> {log}\nmv {held} {held}.old\n: > {held}\nprintf relaunch > \"$GROVE_SIGNAL_FILE\"\n",
            log = shell_quote(&launch_log),
            held = shell_quote(&lease_path(&root)),
        ),
    );

    let home = tmp.path().join("home");
    let output = grove_driver(&root, &configured, &home).output().unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "unexpected success: {stderr}");
    assert!(
        stderr.contains("driver lease path was replaced"),
        "unexpected error: {stderr}"
    );
    assert_eq!(
        fs::read_to_string(&launch_log).unwrap(),
        "x",
        "the driver launched again after losing the lease: {stderr}"
    );
}

#[test]
fn grove_llm_admits_only_the_live_epoch_while_version_remains_exempt() {
    let _environment_lock = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    init_colocated_worktree(&root);
    let grove = root.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("BRIEF.md"), "# test — brief\n").unwrap();
    fs::write(grove.join("01-impl--test-k1.md"), "# test-k1\n").unwrap();
    let ready = tmp.path().join("harness-ready");
    let signal_log = tmp.path().join("signal-path");
    let fake_harness = tmp.path().join("fake-claude.sh");
    write_executable(
        &fake_harness,
        &format!(
            "#!/bin/sh\nprintf '%s' \"$GROVE_SIGNAL_FILE\" > '{}'\ntouch '{}'\ntrap 'exit 0' TERM\nwhile :; do sleep 1; done\n",
            signal_log.display(),
            ready.display()
        ),
    );
    let home = tmp.path().join("home");
    let mut driver_command = grove_driver(&root, &fake_harness, &home);
    let mut driver = DriverProcess::spawn(&mut driver_command, &tmp.path().join("driver-log"));
    driver.wait_for_ready(&ready);
    let live_signal = PathBuf::from(fs::read_to_string(&signal_log).unwrap());

    let live = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("pick")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &live_signal)
        .output()
        .unwrap();
    assert!(
        live.status.success(),
        "live epoch was refused: {}",
        String::from_utf8_lossy(&live.stderr)
    );

    let wrong_signal = live_signal.with_file_name("signal-22222222222222222222222222222222");
    let misdirected_complete = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .args(["complete", "--signal-file"])
        .arg(&wrong_signal)
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &live_signal)
        .output()
        .unwrap();
    assert!(
        !misdirected_complete.status.success(),
        "complete wrote outside the admitted epoch"
    );
    assert!(
        !wrong_signal.exists(),
        "complete created a signal path other than the active epoch's channel"
    );
    assert!(
        String::from_utf8_lossy(&misdirected_complete.stderr)
            .contains("does not match the admitted session epoch"),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&misdirected_complete.stderr)
    );

    let stale = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("pick")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &wrong_signal)
        .output()
        .unwrap();
    assert!(
        !stale.status.success(),
        "mismatched epoch unexpectedly picked"
    );
    assert!(
        String::from_utf8_lossy(&stale.stderr).contains("stale Grove session"),
        "unexpected stderr: {}",
        String::from_utf8_lossy(&stale.stderr)
    );

    let version = Command::new(env!("CARGO_BIN_EXE_grove-llm"))
        .arg("--version")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &wrong_signal)
        .output()
        .unwrap();
    assert!(
        version.status.success(),
        "--version must not need a live epoch: {}",
        String::from_utf8_lossy(&version.stderr)
    );
    drop(driver);
}

#[test]
fn a_reinitialized_tree_reuses_plan_k1_without_reusing_the_old_session() {
    let _environment_lock = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    init_colocated_worktree(&root);
    let grove_llm = env!("CARGO_BIN_EXE_grove-llm");

    let initialize = |root: &Path| {
        let output = Command::new(grove_llm)
            .arg("root-init")
            .current_dir(root)
            .env_remove("GROVE_SIGNAL_FILE")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "root-init failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(
            root.join(".grove/01-requirements--plan-k1.md").exists(),
            "root-init did not allocate plan-k1"
        );
    };
    initialize(&root);
    run_command("git", &root, &["add", "-A"]);
    run_command("git", &root, &["commit", "-q", "-m", "first grove"]);

    let first_ready = tmp.path().join("first-ready");
    let first_signal_log = tmp.path().join("first-signal-path");
    let first_harness = tmp.path().join("first-claude.sh");
    write_executable(
        &first_harness,
        &format!(
            "#!/bin/sh\nprintf '%s' \"$GROVE_SIGNAL_FILE\" > '{}'\ntouch '{}'\ntrap 'exit 0' TERM\nwhile :; do sleep 1; done\n",
            first_signal_log.display(),
            first_ready.display()
        ),
    );
    let home = tmp.path().join("home");
    let mut first_command = grove_driver(&root, &first_harness, &home);
    let mut first_driver =
        DriverProcess::spawn(&mut first_command, &tmp.path().join("first-driver-log"));
    first_driver.wait_for_ready(&first_ready);
    let old_signal = PathBuf::from(fs::read_to_string(&first_signal_log).unwrap());

    let old_pick = Command::new(grove_llm)
        .arg("pick")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &old_signal)
        .output()
        .unwrap();
    assert!(
        old_pick.status.success(),
        "the first session was not admitted"
    );
    assert!(
        String::from_utf8_lossy(&old_pick.stdout).contains("plan-k1"),
        "the first session did not resolve plan-k1"
    );

    let retire = Command::new(grove_llm)
        .args([
            "leaf-retire",
            root.join(".grove/01-requirements--plan-k1.md")
                .to_str()
                .unwrap(),
        ])
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &old_signal)
        .output()
        .unwrap();
    assert!(
        retire.status.success(),
        "retiring the first grove failed: {}",
        String::from_utf8_lossy(&retire.stderr)
    );
    fs::write(
        root.join(".grove/02-finish--finish-k2.md"),
        "# finish-k2\n\n## Goal\n\nFinish.\n",
    )
    .unwrap();
    let finish = Command::new(grove_llm)
        .args(["finish-commit", "finish-k2"])
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &old_signal)
        .output()
        .unwrap();
    assert!(
        finish.status.success(),
        "finishing the first grove failed: {}",
        String::from_utf8_lossy(&finish.stderr)
    );
    assert!(!root.join(".grove").exists());

    drop(first_driver);
    initialize(&root);

    let second_ready = tmp.path().join("second-ready");
    let second_signal_log = tmp.path().join("second-signal-path");
    let second_harness = tmp.path().join("second-claude.sh");
    write_executable(
        &second_harness,
        &format!(
            "#!/bin/sh\nprintf '%s' \"$GROVE_SIGNAL_FILE\" > '{}'\ntouch '{}'\ntrap 'exit 0' TERM\nwhile :; do sleep 1; done\n",
            second_signal_log.display(),
            second_ready.display()
        ),
    );
    let mut second_command = grove_driver(&root, &second_harness, &home);
    let mut second_driver =
        DriverProcess::spawn(&mut second_command, &tmp.path().join("second-driver-log"));
    second_driver.wait_for_ready(&second_ready);
    let new_signal = PathBuf::from(fs::read_to_string(&second_signal_log).unwrap());
    assert_ne!(
        new_signal, old_signal,
        "a new launch reused the old signal path"
    );

    let stale_pick = Command::new(grove_llm)
        .arg("pick")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &old_signal)
        .output()
        .unwrap();
    assert!(
        !stale_pick.status.success(),
        "the old session resolved the new plan-k1"
    );
    assert!(
        String::from_utf8_lossy(&stale_pick.stderr).contains("stale Grove session"),
        "unexpected stale-session diagnostic: {}",
        String::from_utf8_lossy(&stale_pick.stderr)
    );

    let stale_mutation = Command::new(grove_llm)
        .args(["leaf-add", ".", "stale-mutation"])
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &old_signal)
        .output()
        .unwrap();
    assert!(
        !stale_mutation.status.success(),
        "the old session mutated the reinitialized tree"
    );
    assert!(
        !root.join(".grove/02-impl--stale-mutation-k2.md").exists(),
        "stale mutation allocated a leaf in the new tree"
    );

    let stale_complete = Command::new(grove_llm)
        .arg("complete")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &old_signal)
        .output()
        .unwrap();
    assert!(
        !stale_complete.status.success(),
        "the old session completed the reinitialized tree"
    );
    assert!(
        !old_signal.exists(),
        "stale complete recreated its old signal"
    );

    let new_pick = Command::new(grove_llm)
        .arg("pick")
        .current_dir(&root)
        .env("GROVE_SIGNAL_FILE", &new_signal)
        .output()
        .unwrap();
    assert!(
        new_pick.status.success(),
        "the new session was refused: {}",
        String::from_utf8_lossy(&new_pick.stderr)
    );
    assert!(
        String::from_utf8_lossy(&new_pick.stdout).contains("plan-k1"),
        "the new session did not resolve the reused plan-k1 handle"
    );

    fs::write(&old_signal, "done\n").unwrap();
    thread::sleep(Duration::from_millis(150));
    assert!(
        second_driver
            .child
            .as_mut()
            .unwrap()
            .try_wait()
            .unwrap()
            .is_none(),
        "an old completion signal affected the new foreground launch"
    );
    drop(second_driver);
}

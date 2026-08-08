mod support;

use grove::driver_lease::DriverLease;
use grove::{harness, harness_stamp, loop_driver};
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

fn fake_git_worktree(path: &Path) {
    fs::create_dir_all(path.join(".git")).unwrap();
}

fn init_git_worktree(path: &Path) {
    fs::create_dir_all(path).unwrap();
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(path)
        .status()
        .unwrap()
        .success());
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

fn grove_driver(root: &Path, harness: &Path, skill_dir: &Path) -> Command {
    let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
    command
        .args(["do", "--harness", "claude"])
        .current_dir(root);
    for name in support::grove_env_names() {
        command.env_remove(name);
    }
    command
        .env("GROVE_HARNESS_BIN", harness)
        .env("GROVE_LLM_BIN", env!("CARGO_BIN_EXE_grove-llm"))
        .env("GROVE_SKILL_DIR", skill_dir)
        .env("GROVE_IMPL_MODEL", "test-model")
        .env("GROVE_KILL_GRACE", "0")
        .env("GROVE_KILL_GRACE_KILL", "0");
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
}

impl DriverProcess {
    fn spawn(command: &mut Command) -> Self {
        Self {
            child: Some(command.spawn().unwrap()),
        }
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
    root.join(".git/grove/driver.lease")
}

fn record_nonce(record: &str) -> &str {
    record
        .lines()
        .find_map(|line| line.strip_prefix("nonce="))
        .unwrap()
}

fn wait_for(path: &Path) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while !path.exists() {
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {}",
            path.display()
        );
        thread::sleep(Duration::from_millis(10));
    }
}

struct Holder {
    child: Child,
}

impl Holder {
    fn spawn(root: &Path, ready: &Path) -> Self {
        let child = Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "lease_holder_process", "--nocapture"])
            .env(HOLDER_ROOT, root)
            .env(HOLDER_READY, ready)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .unwrap();
        wait_for(ready);
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
        wait_for(&exec_ready);
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
    fake_git_worktree(&root);
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
fn driver_launch_uses_the_on_disk_worktree_despite_foreign_git_environment() {
    let tmp = TempDir::new().unwrap();
    let intended = tmp.path().join("intended");
    let foreign = tmp.path().join("foreign");
    init_git_worktree(&intended);
    init_git_worktree(&foreign);
    run_command(
        "git",
        &intended,
        &["config", "core.worktree", foreign.to_str().unwrap()],
    );
    fs::create_dir_all(intended.join(".grove")).unwrap();
    fs::write(intended.join(".grove/FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(intended.join(".grove/BRIEF.md"), "# intended — brief\n").unwrap();
    fs::write(intended.join(".grove/01-impl-test-k1.md"), "# test-k1\n").unwrap();

    let launch_log = tmp.path().join("launch-log");
    let fake_harness = tmp.path().join("fake-claude.sh");
    write_executable(
        &fake_harness,
        &format!(
            "#!/bin/sh\ngit rev-parse --show-toplevel > '{}'\n",
            launch_log.display()
        ),
    );
    let skill_dir = tmp.path().join("global-skill");

    let output = grove_driver(&intended, &fake_harness, &skill_dir)
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
    assert_eq!(
        fs::read_to_string(&launch_log).unwrap().trim(),
        intended.canonicalize().unwrap().to_str().unwrap(),
        "foreground harness inherited a foreign Git worktree"
    );
    let grove_name = intended.file_name().unwrap().to_string_lossy();
    assert!(
        harness_stamp::path(&intended, &grove_name).exists(),
        "driver stamped a repository other than its leased worktree"
    );
    assert!(
        !harness_stamp::path(&foreign, &grove_name).exists(),
        "driver wrote launch state into the environment-selected repository"
    );
}

#[test]
fn distinct_worktrees_hold_independent_leases() {
    let tmp = TempDir::new().unwrap();
    let first = tmp.path().join("first");
    let second = tmp.path().join("second");
    fake_git_worktree(&first);
    fake_git_worktree(&second);
    let holder = Holder::spawn(&first, &tmp.path().join("ready"));

    let second_lease = DriverLease::acquire(&second).unwrap();

    second_lease.revalidate().unwrap();
    drop(holder);
}

#[test]
fn linked_git_worktrees_and_secondary_jj_workspaces_hold_independent_leases() {
    let tmp = TempDir::new().unwrap();

    let git_main = tmp.path().join("git-main");
    init_git_worktree(&git_main);
    run_command(
        "git",
        &git_main,
        &[
            "-c",
            "user.email=t@example.com",
            "-c",
            "user.name=Test",
            "commit",
            "-q",
            "--allow-empty",
            "-m",
            "seed",
        ],
    );
    let git_linked = tmp.path().join("git-linked");
    run_command(
        "git",
        &git_main,
        &["worktree", "add", "-q", git_linked.to_str().unwrap()],
    );
    let git_main_lease = DriverLease::acquire(&git_main).unwrap();
    let git_linked_lease = DriverLease::acquire(&git_linked).unwrap();
    git_main_lease.revalidate().unwrap();
    git_linked_lease.revalidate().unwrap();

    let jj_main = tmp.path().join("jj-main");
    fs::create_dir_all(&jj_main).unwrap();
    run_command(
        "jj",
        &jj_main,
        &[
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
            "--config",
            "git.colocate=false",
            "git",
            "init",
            "--quiet",
            ".",
        ],
    );
    let jj_secondary = tmp.path().join("jj-secondary");
    run_command(
        "jj",
        &jj_main,
        &[
            "--config",
            "user.name=Test",
            "--config",
            "user.email=t@example.com",
            "workspace",
            "add",
            "--quiet",
            jj_secondary.to_str().unwrap(),
        ],
    );
    let jj_main_lease = DriverLease::acquire(&jj_main).unwrap();
    let jj_secondary_lease = DriverLease::acquire(&jj_secondary).unwrap();
    jj_main_lease.revalidate().unwrap();
    jj_secondary_lease.revalidate().unwrap();
}

#[test]
fn normal_owner_exit_releases_the_lease_without_cleanup() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
    let holder = Holder::spawn(&root, &tmp.path().join("ready"));

    holder.release_normally();

    DriverLease::acquire(&root).unwrap().revalidate().unwrap();
}

#[test]
fn forced_owner_exit_releases_the_lease_without_pid_cleanup() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
    let holder = Holder::spawn(&root, &tmp.path().join("ready"));

    holder.kill();

    DriverLease::acquire(&root).unwrap().revalidate().unwrap();
}

#[test]
fn owner_panic_releases_the_lease_during_unwind() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
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
    wait_for(&ready);

    assert!(!child.wait().unwrap().success());
    DriverLease::acquire(&root).unwrap().revalidate().unwrap();
}

#[test]
fn revalidation_refuses_a_replaced_lease_path() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
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
    fake_git_worktree(&root);

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
fn a_non_directory_control_location_fails_before_a_lease_is_created() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
    fs::write(root.join(".git/grove"), b"not a directory").unwrap();

    let error = DriverLease::acquire(&root).unwrap_err();

    assert!(
        error
            .to_string()
            .contains("creating Grove control directory"),
        "unexpected error: {error:#}"
    );
}

#[test]
fn an_unwritable_control_parent_fails_before_a_lease_is_created() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
    let git_directory = root.join(".git");
    fs::set_permissions(&git_directory, fs::Permissions::from_mode(0o500)).unwrap();

    let result = DriverLease::acquire(&root);

    fs::set_permissions(&git_directory, fs::Permissions::from_mode(0o700)).unwrap();
    let error = result.unwrap_err();
    assert!(
        error
            .to_string()
            .contains("creating Grove control directory"),
        "unexpected error: {error:#}"
    );
    assert!(!lease_path(&root).exists());
}

#[test]
fn an_execed_descendant_does_not_inherit_driver_ownership() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    fake_git_worktree(&root);
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
    wait_for(&ready);
    wait_for(&exec_ready);
    wait_for(&released);

    DriverLease::acquire(&root).unwrap().revalidate().unwrap();

    drop(child.stdin.take());
    assert!(child.wait().unwrap().success());
}

#[test]
fn a_second_driver_reprovisions_then_refuses_before_tree_access_or_launch() {
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    init_git_worktree(&root);
    fs::create_dir_all(root.join(".claude")).unwrap();
    fs::create_dir_all(root.join(".grove")).unwrap();
    fs::write(root.join(".grove/FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(root.join(".grove/BRIEF.md"), "# test — brief\n").unwrap();
    fs::write(root.join(".grove/01-impl-test-k1.md"), "# test-k1\n").unwrap();
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
    let skill_dir = tmp.path().join("global-skill");
    let mut first_command = grove_driver(&root, &fake_harness, &skill_dir);
    first_command.stdout(Stdio::null()).stderr(Stdio::null());
    let first = DriverProcess::spawn(&mut first_command);
    wait_for(&first_ready);
    wait_for(&harness_pid);
    fs::remove_file(root.join(".grove/FORMAT")).unwrap();
    fs::remove_file(root.join(".grove/01-impl-test-k1.md")).unwrap();
    fs::write(
        root.join(".grove/1-[1]-test.md"),
        "# test-k1\n\n**Kind:** impl\n",
    )
    .unwrap();
    let head_before_second = command_stdout("git", &root, &["rev-parse", "HEAD"]);
    fs::remove_file(skill_dir.join("SKILL.md")).unwrap();
    fs::write(skill_dir.join(".grove-content-hash"), "stale-hash\n").unwrap();

    let second = grove_driver(&root, &fake_harness, &skill_dir)
        .output()
        .unwrap();

    assert!(
        !second.status.success(),
        "second driver unexpectedly succeeded"
    );
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(
        stderr.contains("existing Grove driver must stop"),
        "unexpected stderr: {stderr}"
    );
    assert!(skill_dir.join("SKILL.md").is_file());
    assert!(
        !duplicate_launch.exists(),
        "second driver launched a harness"
    );
    assert!(
        root.join(".grove/1-[1]-test.md").exists(),
        "second driver migrated the live driver's task tree"
    );
    assert!(
        !root.join(".grove/FORMAT").exists(),
        "second driver wrote the current-format witness"
    );
    assert_eq!(
        command_stdout("git", &root, &["rev-parse", "HEAD"]),
        head_before_second,
        "second driver committed a migration before acquiring ownership"
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

#[test]
fn lease_path_loss_after_tree_selection_refuses_the_foreground_launch() {
    let _lock = support::lock_env(&ENV_LOCK);
    let tmp = TempDir::new().unwrap();
    let root = tmp.path().join("worktree");
    init_git_worktree(&root);
    fs::create_dir_all(root.join(".grove")).unwrap();
    fs::write(root.join(".grove/FORMAT"), "session-kinds-v1\n").unwrap();
    fs::write(root.join(".grove/BRIEF.md"), "# test — brief\n").unwrap();
    fs::write(root.join(".grove/01-impl-test-k1.md"), "# test-k1\n").unwrap();

    let launch_log = tmp.path().join("launch-log");
    let fake_harness = tmp.path().join("fake-claude.sh");
    write_executable(
        &fake_harness,
        &format!("#!/bin/sh\nprintf launched > '{}'\n", launch_log.display()),
    );
    let wrapper = tmp.path().join("grove-llm-wrapper.sh");
    let held_path = lease_path(&root);
    write_executable(
        &wrapper,
        &format!(
            "#!/bin/sh\nif [ \"$1\" = --version ]; then\n  printf 'grove-llm {}\\n'\n  exit 0\nfi\nif [ \"$1\" = kind ]; then\n  mv '{}' '{}.old'\n  : > '{}'\nfi\nexec '{}' \"$@\"\n",
            env!("CARGO_PKG_VERSION"),
            held_path.display(),
            held_path.display(),
            held_path.display(),
            env!("CARGO_BIN_EXE_grove-llm")
        ),
    );
    let skill_dir = tmp.path().join("skill");
    fs::create_dir_all(skill_dir.join("prompts")).unwrap();
    fs::write(skill_dir.join("prompts/continue.md"), "CONTINUE").unwrap();
    let mut env = support::EnvGuard::new();
    env.clear_grove_env()
        .set("GROVE_HARNESS_BIN", &fake_harness)
        .set("GROVE_LLM_BIN", &wrapper)
        .set("GROVE_SKILL_DIR", &skill_dir)
        .set("GROVE_IMPL_MODEL", "test-model");

    let error = loop_driver::run_loop(
        harness::by_name("claude").unwrap(),
        &root,
        &root,
        "lease-loss",
    )
    .unwrap_err();

    let error_chain = format!("{error:#}");
    assert!(
        error_chain.contains("driver lease path was replaced"),
        "unexpected error: {error_chain}"
    );
    assert!(
        !launch_log.exists(),
        "foreground harness launched after lease loss"
    );
}

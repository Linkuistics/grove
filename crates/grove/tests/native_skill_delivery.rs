//! **Native skill delivery belongs to the owner, and startup leaves it alone.**
//!
//! Grove names the skill a session must load; the user supplies it through a
//! harness's native installation or through Ravel's directory delivery
//! (`docs/adr/skill-delivery-is-the-users-choice.md`). The removed Codex
//! provisioner wrote `~/.agents/skills` links, a `current` pointer and complete
//! snapshots under `~/.agents/.grove`. Old drivers, lazy link readers and
//! detached processes can still read that output, so startup must neither
//! adopt, repair, repoint nor delete it
//! (`docs/specs/codex-skill-provisioning.md`, *Existing installations and active
//! readers*).
//!
//! These tests are behavioural. Each one captures the native trees before a
//! real `grove` run and compares every entry afterwards: type, mode bits, inode,
//! modification time, file bytes and link targets. A rewrite with identical
//! bytes still changes an inode, and a staged file that was later removed still
//! changes its directory's modification time. The legacy layout includes entries
//! that the removed provisioner would have refused, repaired or removed, so the
//! old binary fails these assertions.
//!
//! The native-only lifecycle test proves the caller and lifecycle behaviour with
//! explicitly installed skills and actual Ravel 1.1, which has no directory
//! injection. A managed fixture stands in for the agent. It proves that the
//! complete native installation is readable to each session, not that an LLM
//! follows the mandate.
//!
//! The missing-guidance test proves Grove's interpretation of the mandate's
//! missing-skill rule. A managed session reads its skill name and both ending
//! commands from the mandate text, reports, and force-stops. Actual Claude Code
//! and Codex behaviour with a missing kind skill belongs to acceptance.

mod support;

use std::collections::BTreeMap;
use std::fs;
use std::io::Write as _;
use std::os::unix::fs::{symlink, MetadataExt, PermissionsExt};
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use tempfile::TempDir;

/// One entry of a captured native tree, compared exactly before and after.
#[derive(Debug, PartialEq, Eq)]
enum Entry {
    Directory {
        mode: u32,
        inode: u64,
        modified: (i64, i64),
    },
    File {
        mode: u32,
        inode: u64,
        modified: (i64, i64),
        bytes: Vec<u8>,
    },
    Link {
        inode: u64,
        target: PathBuf,
    },
}

/// Capture `root` without following links. An absent root captures as empty.
fn capture(root: &Path) -> BTreeMap<PathBuf, Entry> {
    fn walk(root: &Path, path: &Path, out: &mut BTreeMap<PathBuf, Entry>) {
        let meta = match fs::symlink_metadata(path) {
            Ok(meta) => meta,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => panic!("inspect {}: {error}", path.display()),
        };
        let relative = path.strip_prefix(root).unwrap().to_path_buf();
        let modified = (meta.mtime(), meta.mtime_nsec());
        let mode = meta.permissions().mode() & 0o7777;
        if meta.file_type().is_symlink() {
            let target = fs::read_link(path).unwrap();
            out.insert(
                relative,
                Entry::Link {
                    inode: meta.ino(),
                    target,
                },
            );
        } else if meta.is_dir() {
            out.insert(
                relative,
                Entry::Directory {
                    mode,
                    inode: meta.ino(),
                    modified,
                },
            );
            let mut children: Vec<_> = fs::read_dir(path)
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .collect();
            children.sort();
            for child in children {
                walk(root, &child, out);
            }
        } else {
            out.insert(
                relative,
                Entry::File {
                    mode,
                    inode: meta.ino(),
                    modified,
                    bytes: fs::read(path).unwrap(),
                },
            );
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

/// The native roots a run must leave untouched, captured together.
fn capture_native(home: &Path) -> Vec<(&'static str, BTreeMap<PathBuf, Entry>)> {
    [".agents", ".codex", "codex-alias"]
        .into_iter()
        .map(|name| (name, capture(&home.join(name))))
        .collect()
}

fn write_executable(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn write_mode(path: &Path, body: &str, mode: u32) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, body).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).unwrap();
}

struct Fixture {
    temp: TempDir,
    home: PathBuf,
    work: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let temp = TempDir::new().unwrap();
        let home = temp.path().join("home");
        let work = temp.path().join("work");
        fs::create_dir_all(home.join(".codex")).unwrap();
        fs::create_dir(&work).unwrap();
        support::init_jj_repo(&work);
        Self { temp, home, work }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.temp.path().join(name)
    }

    /// Route every kind to a session that records what its native links serve.
    fn record_native_reads(&self) {
        let script = self.path("session.sh");
        let reads = self.path("reads");
        write_executable(
            &script,
            &format!(
                "#!/bin/sh\n\
                 for link in \"$HOME/.agents/skills/grove\" \"$HOME/.codex/skills/grove\"; do\n\
                 cat \"$link/SKILL.md\" >> '{reads}'\n\
                 done\n\
                 exit 0\n",
                reads = reads.display()
            ),
        );
        support::installed_ravel();
        support::route_every_kind_to(&self.home, &script);
    }

    fn command(&self, path: std::ffi::OsString) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_grove"));
        command
            .current_dir(&self.work)
            .env("HOME", &self.home)
            .env("PATH", path)
            .env_remove("XDG_CONFIG_HOME")
            .stdin(Stdio::null());
        for name in support::grove_env_names() {
            command.env_remove(name);
        }
        // SAFETY: setsid is async-signal-safe before exec.
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

    fn run(&self) -> Output {
        self.command(support::ravel_path()).output().unwrap()
    }

    /// Route every kind to `/bin/sh script prompt args...` through actual Ravel.
    fn route_to_script(&self, script: &Path, args: &[&Path]) {
        let mut words = vec![
            support::ravel::Arg::Text(script.to_str().unwrap()),
            support::ravel::Arg::Prompt,
        ];
        words.extend(
            args.iter()
                .map(|arg| support::ravel::Arg::Text(arg.to_str().unwrap())),
        );
        let mut route = support::ravel::Route::new("native-only-1", "/bin/sh", words);
        route.reason = "a managed fixture stands in for the agent";
        support::write_config(&self.home, &route.source(support::runtime()));
    }
}

/// The layout the removed provisioner left, plus what an owner added beside it.
///
/// Several entries are ones the provisioner would have changed: a foreign
/// directory and file at bundled skill names (refused launch), an obsolete
/// owned link (removed), a damaged snapshot and a lost executable bit (new
/// snapshot and repointed `current`), and a developer checkout link (adopted).
struct Legacy {
    snapshot_skill: PathBuf,
}

fn legacy_installation(home: &Path, alias: bool) -> Legacy {
    let agents = home.join(".agents");
    let root = agents.join(".grove");
    let old = root.join("snapshots/bundle-old");
    let older = root.join("snapshots/bundle-older");
    write_mode(&root.join("install.lock"), "", 0o644);
    write_mode(
        &old.join("plugins/grove/skills/grove/SKILL.md"),
        "old spine\n",
        0o644,
    );
    write_mode(
        &old.join("plugins/grove/skills/grove-impl/SKILL.md"),
        "damaged older bundle\n",
        0o600,
    );
    write_mode(&old.join("plugins/install.sh"), "#!/bin/sh\n", 0o644);
    write_mode(
        &older.join("plugins/grove/skills/grove/SKILL.md"),
        "older spine\n",
        0o444,
    );
    symlink(&old, root.join("current")).unwrap();

    let skills = agents.join("skills");
    fs::create_dir_all(&skills).unwrap();
    let current = root.join("current/plugins/grove/skills");
    symlink(current.join("grove"), skills.join("grove")).unwrap();
    symlink(current.join("grove-impl"), skills.join("grove-impl")).unwrap();
    symlink(current.join("obsolete"), skills.join("obsolete")).unwrap();
    symlink(
        older.join("plugins/grove/skills/grove"),
        skills.join("retained"),
    )
    .unwrap();
    let checkout = home.join("checkout/plugins/grove/skills/grove-design");
    write_mode(&checkout.join("SKILL.md"), "developer content\n", 0o644);
    symlink(&checkout, skills.join("grove-design")).unwrap();
    fs::create_dir(skills.join("coding-style-rust")).unwrap();
    write_mode(&skills.join("coding-style-rust/SKILL.md"), "mine\n", 0o600);
    symlink("/foreign/missing", skills.join("machine-environment")).unwrap();
    write_mode(&skills.join("notes.txt"), "owner notes\n", 0o600);

    let codex = home.join(".codex");
    if alias {
        // Canonical and legacy locations name one directory.
        symlink(&skills, codex.join("skills")).unwrap();
    } else {
        let legacy = codex.join("skills");
        fs::create_dir_all(&legacy).unwrap();
        symlink(current.join("grove"), legacy.join("grove")).unwrap();
        write_mode(
            &legacy.join("coding-style-rust"),
            "unrelated legacy\n",
            0o640,
        );
    }
    symlink(&codex, home.join("codex-alias")).unwrap();
    // Read-only modes the provisioner could not have written through.
    fs::set_permissions(&older, fs::Permissions::from_mode(0o555)).unwrap();
    Legacy {
        snapshot_skill: old.join("plugins/grove/skills/grove"),
    }
}

/// Restore write access so the temporary directory can be removed.
fn release(home: &Path) {
    let older = home.join(".agents/.grove/snapshots/bundle-older");
    if older.exists() {
        fs::set_permissions(older, fs::Permissions::from_mode(0o755)).unwrap();
    }
}

/// A reader that resolved the old snapshot before startup and reads it after.
fn old_reader(directory: &Path) -> Child {
    Command::new("/bin/sh")
        .args(["-c", "read go; cat SKILL.md"])
        .current_dir(directory)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
}

fn finish_reader(mut reader: Child) -> String {
    reader.stdin.take().unwrap().write_all(b"go\n").unwrap();
    let output = reader.wait_with_output().unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn startup_leaves_a_legacy_installation_and_its_readers_untouched() {
    for alias in [false, true] {
        for codex_home in [None, Some("codex-alias")] {
            let f = Fixture::new();
            let legacy = legacy_installation(&f.home, alias);
            f.record_native_reads();
            let before = capture_native(&f.home);
            let reader = old_reader(&legacy.snapshot_skill);

            let mut command = f.command(support::ravel_path());
            if let Some(name) = codex_home {
                command.env("CODEX_HOME", f.home.join(name));
            }
            let output = command.output().unwrap();

            let case = format!("alias={alias} CODEX_HOME={codex_home:?}");
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(output.status.success(), "{case}: {stderr}");
            assert!(!stderr.contains("Codex"), "{case}: {stderr}");
            assert_eq!(capture_native(&f.home), before, "{case}");
            assert_eq!(finish_reader(reader), "old spine\n", "{case}");
            assert_eq!(
                fs::read_to_string(f.path("reads")).unwrap(),
                "old spine\nold spine\n",
                "{case}: the launched session reads the existing links"
            );
            release(&f.home);
        }
    }
}

#[test]
fn a_failed_startup_leaves_a_legacy_installation_untouched() {
    let f = Fixture::new();
    let legacy = legacy_installation(&f.home, false);
    f.record_native_reads();
    // An incompatible Ravel is refused after the point where provisioning ran.
    let bin = f.path("old-ravel");
    fs::create_dir(&bin).unwrap();
    write_executable(&bin.join("ravel"), "#!/bin/sh\necho 'ravel 1.0.0'\n");
    let before = capture_native(&f.home);
    let reader = old_reader(&legacy.snapshot_skill);

    let output = f.command(support::path_with_entry(&bin)).output().unwrap();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(!output.status.success(), "{stderr}");
    assert!(stderr.contains("1.1.0"), "{stderr}");
    assert!(!f.path("reads").exists(), "a session launched: {stderr}");
    assert_eq!(capture_native(&f.home), before);
    assert_eq!(finish_reader(reader), "old spine\n");
    release(&f.home);
}

#[test]
fn fresh_codex_homes_receive_no_native_installation() {
    let f = Fixture::new();
    f.record_native_reads();
    let custom = f.home.join("custom-codex");
    let before = capture(&f.home.join(".codex"));
    for args in [vec![], vec!["--help"], vec!["--version"]] {
        let output = f
            .command(support::ravel_path())
            .args(&args)
            .env("CODEX_HOME", &custom)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    let output = f.run();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(!f.home.join(".agents").exists());
    assert!(!custom.exists());
    assert_eq!(capture(&f.home.join(".codex")), before);
}

/// Each session resolves its kind from its leaf filename, not from mandate
/// wording, then reads the spine, its kind skill and one supporting resource
/// through the native Codex installation before it acts.
const NATIVE_SESSION: &str = r#"set -eu
prompt=$1 llm=$2 ravel=$3 log=$4
handle=${prompt#*'Grove mandate: the leaf selected for this session is `'}
handle=${handle%%'`'*}
leaf=$(cd .grove && ls | grep -e "--$handle\.md\$")
kind=${leaf#*-}
kind=${kind%%--*}
for file in grove/SKILL.md "grove-$kind/SKILL.md" grove/references/retire.md; do
  path=$HOME/.codex/skills/$file
  test -r "$path"
  cat "$path" > /dev/null
  resolved=$(cd "$(dirname "$path")" && pwd -P)
  printf '%s %s %s\n' "$handle" "$file" "$resolved" >> "$log"
done
case $kind in
finish)
  "$llm" finish-commit "$handle"
  "$llm" record-teardown
  ;;
*)
  "$llm" leaf-retire ".grove/$leaf"
  jj describe -m "$handle"
  jj new
  ;;
esac
exec "$ravel" end-session grove-continue
"#;

#[test]
fn a_native_only_lifecycle_completes_with_explicitly_installed_skills() {
    let f = Fixture::new();
    // The owner's explicit native route: the manual installer, run for Codex.
    let install = Command::new("bash")
        .arg(support::repo_root().join("plugins/install.sh"))
        .arg("--force")
        .env("HOME", &f.home)
        .env_remove("CODEX_HOME")
        .output()
        .unwrap();
    assert!(
        install.status.success(),
        "{}",
        String::from_utf8_lossy(&install.stderr)
    );
    let plugins = support::repo_root().join("plugins").canonicalize().unwrap();
    for name in ["grove", "grove-impl", "grove-finish"] {
        assert!(
            f.home
                .join(".codex/skills")
                .join(name)
                .join("SKILL.md")
                .is_file(),
            "the installer did not supply {name}"
        );
    }
    let installed = capture(&f.home.join(".codex"));

    fs::create_dir(f.work.join(".grove")).unwrap();
    fs::write(f.work.join(".grove/_BRIEF.md"), "# native — brief\n").unwrap();
    fs::write(f.work.join(".grove/01-impl--work-k1.md"), "# work-k1\n").unwrap();
    let script = f.path("session.sh");
    let log = f.path("reads.log");
    let prompts = f.path("prompts");
    write_executable(
        &script,
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$1\" >> '{prompts}'\n{NATIVE_SESSION}",
            prompts = prompts.display()
        ),
    );
    let ravel = support::installed_ravel();
    f.route_to_script(&script, &[&support::grove_llm(), &ravel, &log]);

    let output = f.run();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success() && stderr.contains("loop complete"),
        "{stderr}"
    );
    let reads = fs::read_to_string(&log).unwrap();
    let rows: Vec<Vec<&str>> = reads.lines().map(|l| l.split(' ').collect()).collect();
    let kinds: Vec<&str> = rows
        .iter()
        .filter(|row| row[1].ends_with("SKILL.md") && row[1] != "grove/SKILL.md")
        .map(|row| row[1])
        .collect();
    assert_eq!(
        kinds,
        ["grove-impl/SKILL.md", "grove-finish/SKILL.md"],
        "{reads}"
    );
    for row in &rows {
        assert!(
            Path::new(row[2]).starts_with(&plugins),
            "a read was served outside the explicit installation: {reads}"
        );
    }
    // Ravel 1.1 injects no skill directory; the prompt names no source path.
    let prompts = fs::read_to_string(&prompts).unwrap();
    assert!(
        !prompts.contains(plugins.to_str().unwrap()),
        "the prompt carried a skill location: {prompts}"
    );
    // The finish helper removed and committed the tree after the impl leaf retired.
    assert!(!f.work.join(".grove").exists());
    assert!(!f.home.join(".agents").exists());
    assert_eq!(capture(&f.home.join(".codex")), installed);
}

/// A managed session that follows the mandate text rather than fixture
/// knowledge. It reads the skill name from the load instruction, the ending
/// command from the signalling contract and the force-stop command from the
/// missing-guidance rule. A skill it cannot read produces a report and that
/// force-stop. Otherwise it retires the leaf, or finishes, and continues.
const MANDATE_SESSION: &str = r#"set -eu
prompt=$1 llm=$2 skills=$3 log=$4
handle=${prompt#*'Grove mandate: the leaf selected for this session is `'}
handle=${handle%%'`'*}
skill=${prompt#'**Load the `'}
skill=${skill%%'`'*}
ending=$(printf '%s\n' "$prompt" | sed -n '/then do nothing else\*\*:$/{n;s/^`\(.*\)`$/\1/p;}')
stop=$(printf '%s\n' "$prompt" | sed -n '/^action instead:$/{n;s/^`\(.*\)`$/\1/p;}')
test -n "$ending"
test -n "$stop"
leaf=$(cd .grove && ls | grep -e "--$handle\.md\$")
# A relaunch of the same live leaf repeats without end. Bound it, so a broken
# interpretation fails the test instead of hanging it.
test "$(cat "$log" 2>/dev/null | grep -c '^mandate ')" -lt 4 || exit 93
printf 'mandate %s %s\n' "$handle" "$skill" >> "$log"
if ! cat "$skills/$skill/SKILL.md" > /dev/null 2>&1; then
  printf 'report %s guidance is missing or unreadable\n' "$skill" >> "$log"
  test ! -e "$GROVE_LAUNCH_DIR/teardown"
  printf 'stop %s\n' "$stop" >> "$log"
  eval "exec $stop"
fi
case $leaf in
*-finish--*)
  "$llm" finish-commit "$handle"
  "$llm" record-teardown
  ;;
*)
  "$llm" leaf-retire ".grove/$leaf"
  jj describe -m "$handle"
  jj new
  ;;
esac
printf 'end %s\n' "$ending" >> "$log"
eval "exec $ending"
"#;

/// **A missing or unreadable kind skill force-stops with the leaf live.**
///
/// The leaf has a custom kind, so the open-kind path and the missing skill
/// share one case: no tree verb or launch admission checks for the skill. The
/// first run must stop on the mandate's exact force-stop command, leave both
/// leaves live, record no teardown, commit nothing and launch once. After the
/// owner repairs delivery, a rerun mandates the same handle and the same skill,
/// and the loop continues normally to completion.
#[test]
fn missing_or_unreadable_kind_guidance_force_stops_with_the_leaf_live() {
    let ravel = support::installed_ravel();
    let force_stop = grove_loop::session_command::force_stop(&ravel).unwrap();
    let continue_command = grove_loop::session_command::end_session(&ravel).unwrap();
    for case in ["missing", "unreadable"] {
        let f = Fixture::new();
        let skills = f.path("skills");
        write_mode(&skills.join("grove-impl/SKILL.md"), "impl\n", 0o644);
        write_mode(&skills.join("grove-finish/SKILL.md"), "finish\n", 0o644);
        let widget = skills.join("grove-widget/SKILL.md");
        if case == "unreadable" {
            write_mode(&widget, "widget\n", 0o000);
            assert!(
                fs::read(&widget).is_err(),
                "the unreadable case needs a file this user cannot read"
            );
        }
        let grove = f.work.join(".grove");
        fs::create_dir(&grove).unwrap();
        fs::write(grove.join("_BRIEF.md"), "# mandate — brief\n").unwrap();
        fs::write(grove.join("01-widget--subject-k1.md"), "# subject-k1\n").unwrap();
        fs::write(grove.join("02-impl--remaining-k2.md"), "# remaining-k2\n").unwrap();
        // Snapshot the planted tree first, so the comparison sees only session work.
        jj_snapshot(&f.work);
        let tree = capture(&grove);
        let operations = jj_operations(&f.work);
        let script = f.path("session.sh");
        write_executable(&script, &format!("#!/bin/sh\n{MANDATE_SESSION}"));
        let log = f.path("session.log");
        f.route_to_script(&script, &[&support::grove_llm(), &skills, &log]);

        let output = f.run();

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert_eq!(output.status.code(), Some(1), "{case}: {stderr}");
        assert!(stderr.contains("force-stop requested"), "{case}: {stderr}");
        assert!(!stderr.contains("grove finished"), "{case}: {stderr}");
        assert_eq!(
            fs::read_to_string(&log).unwrap(),
            format!(
                "mandate subject-k1 grove-widget\n\
                 report grove-widget guidance is missing or unreadable\n\
                 stop {force_stop}\n"
            ),
            "{case}: one launch, stopped by the mandate's exact command"
        );
        assert_eq!(capture(&grove), tree, "{case}: the tree changed");
        assert_eq!(
            jj_operations(&f.work),
            operations,
            "{case}: work was committed"
        );

        // The owner repairs the chosen delivery route and reruns.
        if case == "missing" {
            write_mode(&widget, "widget\n", 0o644);
        } else {
            fs::set_permissions(&widget, fs::Permissions::from_mode(0o644)).unwrap();
        }
        fs::remove_file(&log).unwrap();

        let output = f.run();

        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            output.status.success() && stderr.contains("loop complete"),
            "{case}: {stderr}"
        );
        let log = fs::read_to_string(&log).unwrap();
        let mandates: Vec<&str> = log
            .lines()
            .filter_map(|line| line.strip_prefix("mandate "))
            .collect();
        assert_eq!(mandates.len(), 3, "{case}: {log}");
        assert_eq!(mandates[0], "subject-k1 grove-widget", "{case}: {log}");
        assert_eq!(mandates[1], "remaining-k2 grove-impl", "{case}: {log}");
        assert!(mandates[2].ends_with(" grove-finish"), "{case}: {log}");
        assert!(!log.contains("report "), "{case}: {log}");
        assert_eq!(
            log.lines().filter(|line| line.starts_with("end ")).count(),
            3,
            "{case}: {log}"
        );
        assert!(
            log.lines()
                .filter_map(|line| line.strip_prefix("end "))
                .all(|ending| ending == continue_command),
            "{case}: {log}"
        );
        assert!(!grove.exists(), "{case}: the finish did not tear down");
    }
}

/// The jj operation log's identifiers, newest first.
fn jj_operations(work: &Path) -> String {
    let output = Command::new("jj")
        .args(["op", "log", "--no-graph", "-T", "id ++ \"\\n\""])
        .current_dir(work)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap()
}

/// Record the working copy in jj, as any jj command would before it reads.
fn jj_snapshot(work: &Path) {
    let status = Command::new("jj")
        .arg("status")
        .current_dir(work)
        .stdout(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

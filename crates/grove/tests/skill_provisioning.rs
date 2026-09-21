mod support;

use std::fs;
use std::os::unix::{fs::PermissionsExt, process::CommandExt};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use tempfile::TempDir;

struct Fixture {
    home: TempDir,
    work: TempDir,
}
impl Fixture {
    fn new(codex: bool) -> Self {
        let home = TempDir::new().unwrap();
        let work = TempDir::new().unwrap();
        support::init_jj_repo(work.path());
        if codex {
            fs::create_dir(home.path().join(".codex")).unwrap();
        }
        let script = home.path().join("child.sh");
        fs::write(&script, "#!/bin/sh\nif test -f \"$HOME/.agents/skills/grove/SKILL.md\"; then touch \"$HOME/skills-seen\"; fi\ntouch \"$HOME/child-ran\"\nexit 0\n").unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o755)).unwrap();
        fs::create_dir_all(home.path().join(".config/grove")).unwrap();
        let routes: String = support::EVERY_SESSION_KIND
            .iter()
            .map(|kind| format!("route {kind:?} \"lead\"\n"))
            .collect();
        let template = format!("'{}' '${{prompt}}'", script.display());
        fs::write(
            home.path().join(".config/grove/config.kdl"),
            format!(
                "config {{\ncommand \"agent\" {template:?}\nbind \"lead\" \"agent\"\n{routes}}}\n"
            ),
        )
        .unwrap();
        Self { home, work }
    }
    fn command(&self) -> Command {
        let mut cmd = Command::new(env!("CARGO_BIN_EXE_grove"));
        cmd.current_dir(self.work.path())
            .env("HOME", self.home.path())
            .env_remove("CODEX_HOME")
            .env_remove("XDG_CONFIG_HOME")
            .stdin(Stdio::null());
        for name in support::grove_env_names() {
            cmd.env_remove(name);
        }
        // SAFETY: setsid is async-signal-safe before exec.
        unsafe {
            cmd.pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        cmd
    }
    fn run(&self) -> Output {
        self.command().output().unwrap()
    }
    fn skills(&self) -> std::path::PathBuf {
        self.home.path().join(".agents/skills")
    }
}
fn assert_installed(skills: &Path) {
    for name in ["grove", "coding-style-rust", "using-testanyware"] {
        assert!(
            skills.join(name).join("SKILL.md").is_file(),
            "missing installed skill {name}"
        );
    }
    assert!(!skills.join("guardrail").exists());
}
#[test]
fn startup_installs_bundled_skills_before_configured_child() {
    let f = Fixture::new(true);
    let output = f.run();
    assert_installed(&f.skills());
    assert!(
        f.home.path().join("skills-seen").exists(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn repeat_repairs_damage_and_missing_links_without_rewriting_valid_snapshot() {
    use std::os::unix::fs::{symlink, MetadataExt};
    let f = Fixture::new(true);
    f.run();
    let skill = f.skills().join("grove/SKILL.md");
    let original = fs::read(&skill).unwrap();
    let inode = fs::metadata(&skill).unwrap().ino();
    let current = f.home.path().join(".agents/.grove/current");
    let snapshot = fs::read_link(&current).unwrap();
    let link_inode = fs::symlink_metadata(f.skills().join("grove"))
        .unwrap()
        .ino();
    let current_inode = fs::symlink_metadata(&current).unwrap().ino();
    let unchanged = f.run();
    assert!(!String::from_utf8_lossy(&unchanged.stderr).contains("updated bundled Codex skills"));
    assert_eq!(
        fs::symlink_metadata(f.skills().join("grove"))
            .unwrap()
            .ino(),
        link_inode
    );
    assert_eq!(fs::symlink_metadata(&current).unwrap().ino(), current_inode);
    assert_eq!(fs::metadata(&skill).unwrap().ino(), inode);
    assert_eq!(fs::read_link(&current).unwrap(), snapshot);
    fs::remove_file(f.skills().join("grove")).unwrap();
    symlink(
        current.join("plugins/grove/skills/obsolete"),
        f.skills().join("obsolete"),
    )
    .unwrap();
    f.run();
    assert_installed(&f.skills());
    assert!(fs::symlink_metadata(f.skills().join("obsolete")).is_err());
    fs::write(&skill, "damaged older bundle").unwrap();
    f.run();
    assert_eq!(fs::read(&skill).unwrap(), original);
    assert_ne!(fs::read_link(&current).unwrap(), snapshot);
    let script = current.join("plugins/install.sh");
    assert!(fs::metadata(script).unwrap().permissions().mode() & 0o111 != 0);
    fs::write(current.join("extra"), "not bundled").unwrap();
    f.run();
    assert!(!current.join("extra").exists());
}

#[test]
fn foreign_occupants_prevent_launch_and_are_preserved() {
    use std::os::unix::fs::symlink;
    for kind in ["file", "directory", "symlink"] {
        let f = Fixture::new(true);
        fs::create_dir_all(f.skills()).unwrap();
        let occupied = f.skills().join("grove");
        match kind {
            "file" => fs::write(&occupied, "mine").unwrap(),
            "directory" => fs::create_dir(&occupied).unwrap(),
            _ => symlink("/foreign/missing", &occupied).unwrap(),
        }
        let before = fs::symlink_metadata(&occupied).unwrap().file_type();
        let output = f.run();
        assert!(!output.status.success());
        assert!(!f.home.path().join("child-ran").exists());
        assert_eq!(fs::symlink_metadata(&occupied).unwrap().file_type(), before);
        let message = String::from_utf8_lossy(&output.stderr);
        assert!(message.contains(occupied.to_str().unwrap()), "{message}");
        assert!(
            !f.skills().join("coding-style-rust").exists(),
            "preflight before installing any links"
        );
    }
}

#[test]
fn adopts_developer_links_and_refreshes_only_existing_owned_legacy_links() {
    use std::os::unix::fs::symlink;
    let f = Fixture::new(true);
    fs::create_dir_all(f.skills()).unwrap();
    let old = f.home.path().join("checkout/plugins/grove/skills/grove");
    fs::create_dir_all(&old).unwrap();
    fs::write(old.join("SKILL.md"), "developer content").unwrap();
    symlink(&old, f.skills().join("grove")).unwrap();
    let legacy = f.home.path().join(".codex/skills");
    fs::create_dir_all(&legacy).unwrap();
    symlink(&old, legacy.join("grove")).unwrap();
    fs::write(legacy.join("coding-style-rust"), "unrelated legacy").unwrap();
    f.run();
    assert_installed(&f.skills());
    assert_eq!(
        fs::read(legacy.join("grove/SKILL.md")).unwrap(),
        fs::read(f.skills().join("grove/SKILL.md")).unwrap()
    );
    assert_eq!(
        fs::read_to_string(old.join("SKILL.md")).unwrap(),
        "developer content"
    );
    assert_eq!(
        fs::read_to_string(legacy.join("coding-style-rust")).unwrap(),
        "unrelated legacy"
    );
    assert!(!legacy.join("using-testanyware").exists());
}

#[test]
fn detection_and_non_launch_commands_do_not_provision() {
    let f = Fixture::new(false);
    f.run();
    assert!(!f.home.path().join(".agents").exists());
    fs::create_dir(f.home.path().join(".codex")).unwrap();
    for args in [
        vec!["--help"],
        vec!["--version"],
        vec!["config", "show", "--json"],
    ] {
        let output = f.command().args(args).output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(!f.home.path().join(".agents").exists());
    }
}

#[test]
fn custom_codex_homes_share_one_serialized_installation() {
    let f = Fixture::new(false);
    let other = TempDir::new().unwrap();
    support::init_jj_repo(other.path());
    let mut one = f
        .command()
        .env("CODEX_HOME", f.home.path().join("codex-a"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut two = f
        .command()
        .current_dir(other.path())
        .env("CODEX_HOME", f.home.path().join("codex-b"))
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    assert!(one.wait().unwrap().success());
    assert!(two.wait().unwrap().success());
    assert_installed(&f.skills());
    assert!(f.home.path().join("skills-seen").exists());
    assert!(!f.home.path().join("codex-a/skills").exists());
    assert!(!f.home.path().join("codex-b/skills").exists());
    let snapshots = fs::read_dir(f.home.path().join(".agents/.grove/snapshots"))
        .unwrap()
        .count();
    assert_eq!(snapshots, 1);
}

#[test]
fn canonical_legacy_aliases_remove_obsolete_links_once_and_launch() {
    use std::os::unix::fs::symlink;
    for indirect in [false, true] {
        let f = Fixture::new(true);
        fs::create_dir_all(f.skills()).unwrap();
        let codex = if indirect {
            let codex = f.home.path().join(".codex");
            symlink(f.skills(), codex.join("skills")).unwrap();
            codex
        } else {
            f.home.path().join(".agents")
        };
        symlink(
            f.home
                .path()
                .join(".agents/.grove/current/plugins/grove/skills/obsolete"),
            f.skills().join("obsolete"),
        )
        .unwrap();
        let output = f.command().env("CODEX_HOME", codex).output().unwrap();
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(fs::symlink_metadata(f.skills().join("obsolete")).is_err());
        assert!(f.home.path().join("skills-seen").exists());
    }
}

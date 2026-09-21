//! Binary-owned Codex delivery. A shared user lock protects snapshots and links;
//! publication switches one symlink only after all bundled bytes are staged.
use std::collections::BTreeSet;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::os::fd::AsRawFd;
use std::os::unix::fs::{symlink, MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};

include!(concat!(env!("OUT_DIR"), "/plugin_inventory.rs"));

pub(crate) fn ensure_codex_skills() -> Result<()> {
    let Some(home) = std::env::var_os("HOME")
        .filter(|s| !s.is_empty())
        .map(PathBuf::from)
    else {
        if std::env::var_os("CODEX_HOME").is_some() {
            bail!("Codex skill provisioning needs HOME; set HOME to your user directory and retry");
        }
        return Ok(());
    };
    let configured = std::env::var_os("CODEX_HOME").filter(|s| !s.is_empty());
    let codex = configured
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".codex"));
    if configured.is_none() && !codex.is_dir() {
        return Ok(());
    }
    // Absolute links remain valid when Codex reads them from another directory.
    let cwd = std::env::current_dir()?;
    let home = cwd.join(home);
    let codex = cwd.join(codex);
    if install(&home, &codex).with_context(|| format!("Codex skill provisioning failed under {}; correct the reported path or permissions and retry Grove", home.join(".agents").display()))? {
        eprintln!("grove: updated bundled Codex skills in {}", home.join(".agents/skills").display());
    }
    Ok(())
}

fn metadata(path: &Path) -> Result<Option<fs::Metadata>> {
    match fs::symlink_metadata(path) {
        Ok(value) => Ok(Some(value)),
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
        Err(error) => Err(error).with_context(|| format!("inspect {}", path.display())),
    }
}

fn directory(path: &Path) -> Result<()> {
    if let Some(meta) = metadata(path)? {
        if !meta.is_dir() {
            bail!(
                "{} must be a real directory; move the conflicting entry and retry",
                path.display()
            );
        }
    } else {
        fs::create_dir(path).with_context(|| format!("create {}", path.display()))?;
    }
    Ok(())
}

fn lock(root: &Path) -> Result<File> {
    let path = root.join("install.lock");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .custom_flags(libc::O_NOFOLLOW)
        .open(&path)
        .with_context(|| format!("open provisioning lock {}", path.display()))?;
    if !file.metadata()?.is_file() {
        bail!("{} must be a regular lock file", path.display());
    }
    let started = Instant::now();
    loop {
        // SAFETY: file owns a live descriptor; nonblocking flock has no pointer arguments.
        if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0 {
            return Ok(file);
        }
        let error = io::Error::last_os_error();
        if error.kind() != io::ErrorKind::WouldBlock && error.kind() != io::ErrorKind::Interrupted {
            return Err(error).with_context(|| format!("lock {}", path.display()));
        }
        if started.elapsed() >= Duration::from_secs(30) {
            bail!(
                "timed out waiting for {}; let the other Grove installation finish and retry",
                path.display()
            );
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn owned(target: &Path, root: &Path) -> bool {
    for base in [root.join("current/plugins"), root.join("snapshots")] {
        if target.strip_prefix(base).is_ok_and(|tail| {
            tail.components()
                .all(|part| matches!(part, std::path::Component::Normal(_)))
        }) {
            return true;
        }
    }
    let parts: Vec<_> = target.components().collect();
    if parts.len() < 4 {
        return false;
    }
    let tail = &parts[parts.len() - 4..];
    matches!(tail[3], std::path::Component::Normal(_))
        && tail[0].as_os_str() == "plugins"
        && tail[2].as_os_str() == "skills"
        && SKILLS
            .iter()
            .any(|(_, path)| Path::new(path).components().nth(1) == Some(tail[1]))
}

fn owned_link(path: &Path, root: &Path) -> Result<bool> {
    let Some(meta) = metadata(path)? else {
        return Ok(false);
    };
    if !meta.file_type().is_symlink() {
        return Ok(false);
    }
    let target = fs::read_link(path).with_context(|| format!("read {}", path.display()))?;
    Ok(owned(
        &path.parent().unwrap_or(Path::new("/")).join(target),
        root,
    ))
}

fn inventory(path: &Path, base: &Path, found: &mut BTreeSet<PathBuf>) -> Result<bool> {
    for entry in
        fs::read_dir(path).with_context(|| format!("read snapshot directory {}", path.display()))?
    {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            if !inventory(&entry.path(), base, found)? {
                return Ok(false);
            }
        } else if kind.is_file() {
            found.insert(entry.path().strip_prefix(base)?.to_path_buf());
        } else {
            return Ok(false);
        }
    }
    Ok(true)
}

fn matches_snapshot(path: &Path) -> Result<bool> {
    if !metadata(path)?.is_some_and(|meta| meta.is_dir()) {
        return Ok(false);
    }
    let mut found = BTreeSet::new();
    if !inventory(path, path, &mut found)?
        || found
            != FILES
                .iter()
                .map(|(name, _, _)| PathBuf::from(name))
                .collect()
    {
        return Ok(false);
    }
    for (name, bytes, mode) in FILES {
        let file = path.join(name);
        let actual =
            fs::read(&file).with_context(|| format!("read snapshot file {}", file.display()))?;
        let metadata = fs::metadata(&file)
            .with_context(|| format!("inspect snapshot file {}", file.display()))?;
        if actual != *bytes || metadata.permissions().mode() & 0o111 != *mode {
            return Ok(false);
        }
    }
    Ok(true)
}

fn replace_link(path: &Path, target: &Path) -> Result<()> {
    let parent = path
        .parent()
        .context("a provisioned link must have a parent directory")?;
    let temporary = tempfile::Builder::new()
        .prefix("link-")
        .tempdir_in(parent)?;
    let staged = temporary.path().join("link");
    symlink(target, &staged)?;
    fs::rename(&staged, path).with_context(|| format!("publish {}", path.display()))
}

fn install(home: &Path, codex: &Path) -> Result<bool> {
    let agents = home.join(".agents");
    // create_dir can race with another first startup before the lock exists.
    match fs::create_dir(&agents) {
        Ok(()) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e).with_context(|| format!("create {}", agents.display())),
    }
    directory(&agents)?;
    let root = agents.join(".grove");
    match fs::create_dir(&root) {
        Ok(()) => (),
        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => (),
        Err(e) => return Err(e).with_context(|| format!("create {}", root.display())),
    }
    directory(&root)?;
    let _lock = lock(&root)?;
    let skills = agents.join("skills");
    directory(&skills)?;
    let snapshots = root.join("snapshots");
    directory(&snapshots)?;
    let current = root.join("current");
    let active = if let Some(meta) = metadata(&current)? {
        if !meta.file_type().is_symlink() {
            bail!(
                "{} is not a Grove snapshot link; move the conflicting entry and retry",
                current.display()
            );
        }
        let target = root.join(fs::read_link(&current)?);
        if target.parent() != Some(snapshots.as_path()) {
            bail!(
                "{} points outside Grove snapshots; move the conflicting entry and retry",
                current.display()
            );
        }
        Some(target)
    } else {
        None
    };
    // Preflight canonical collisions before publishing anything. Legacy foreign
    // entries are unrelated: only already-owned links there participate.
    let legacy = codex.join("skills");
    // Canonical and legacy locations can name one directory through symlinks
    // or mounts. Reconcile that directory once, including obsolete removals.
    let legacy = if legacy.is_dir() {
        let canonical_meta = fs::metadata(&skills)?;
        let legacy_meta = fs::metadata(&legacy)?;
        ((canonical_meta.dev(), canonical_meta.ino()) != (legacy_meta.dev(), legacy_meta.ino()))
            .then_some(legacy)
    } else {
        None
    };
    let mut links = Vec::new();
    for (name, relative) in SKILLS {
        let path = skills.join(name);
        if metadata(&path)?.is_some() && !owned_link(&path, &root)? {
            bail!(
                "{} conflicts with a bundled Codex skill; move or rename it and retry",
                path.display()
            );
        }
        links.push((path, current.join(relative)));
        if let Some(legacy) = &legacy {
            if owned_link(&legacy.join(name), &root)? {
                links.push((legacy.join(name), current.join(relative)));
            }
        }
    }
    let mut obsolete = Vec::new();
    for directory in std::iter::once(&skills).chain(legacy.as_ref()) {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            if !SKILLS.iter().any(|(name, _)| entry.file_name() == *name)
                && owned_link(&entry.path(), &root)?
            {
                obsolete.push(entry.path());
            }
        }
    }
    let mut changed = false;
    if !active
        .as_ref()
        .map(|path| matches_snapshot(path))
        .transpose()?
        .unwrap_or(false)
    {
        let stage = tempfile::Builder::new()
            .prefix("bundle-")
            .tempdir_in(&snapshots)?;
        for (name, bytes, mode) in FILES {
            let path = stage.path().join(name);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&path, bytes).with_context(|| format!("write {}", path.display()))?;
            fs::set_permissions(&path, fs::Permissions::from_mode(0o644 | mode))
                .with_context(|| format!("set executable flags on {}", path.display()))?;
        }
        let published = stage.keep();
        replace_link(&current, &published)?;
        changed = true;
    }
    for (path, target) in links {
        if fs::read_link(&path).ok().as_ref() != Some(&target) {
            replace_link(&path, &target)?;
            changed = true;
        }
    }
    for path in obsolete {
        fs::remove_file(&path)
            .with_context(|| format!("remove obsolete link {}", path.display()))?;
        changed = true;
    }
    Ok(changed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unreadable_snapshot_error_names_the_affected_file() {
        let home = tempfile::tempdir().unwrap();
        let codex = home.path().join(".codex");
        install(home.path(), &codex).unwrap();
        let snapshot = fs::read_link(home.path().join(".agents/.grove/current")).unwrap();
        let file = snapshot.join("plugins/grove/skills/grove/SKILL.md");
        fs::set_permissions(&file, fs::Permissions::from_mode(0o000)).unwrap();
        let result = install(home.path(), &codex);
        fs::set_permissions(&file, fs::Permissions::from_mode(0o644)).unwrap();
        let message = format!("{:#}", result.unwrap_err());
        assert!(message.contains(file.to_str().unwrap()), "{message}");
    }

    #[test]
    fn link_repair_stages_in_destination_without_writing_snapshot_cache() {
        let home = tempfile::tempdir().unwrap();
        let codex = home.path().join(".codex");
        install(home.path(), &codex).unwrap();
        let root = home.path().join(".agents/.grove");
        let skill = home.path().join(".agents/skills/grove");
        fs::remove_file(&skill).unwrap();
        fs::set_permissions(&root, fs::Permissions::from_mode(0o555)).unwrap();
        let repaired = install(home.path(), &codex);
        fs::set_permissions(&root, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(repaired.unwrap());
        assert!(skill.join("SKILL.md").is_file());
        assert!(
            !fs::read_dir(skill.parent().unwrap()).unwrap().any(|entry| {
                entry
                    .unwrap()
                    .file_name()
                    .to_string_lossy()
                    .starts_with("link-")
            })
        );
    }

    #[test]
    fn foreign_links_cannot_claim_ownership_by_escaping_the_cache() {
        let home = tempfile::tempdir().unwrap();
        let root = home.path().join(".agents/.grove");
        let target = root.join("snapshots/../../../foreign");
        assert!(!owned(&target, &root));
    }

    #[test]
    fn repairs_executable_flags_and_missing_files_and_keeps_previous_snapshot() {
        let home = tempfile::tempdir().unwrap();
        let codex = home.path().join(".codex");
        assert!(install(home.path(), &codex).unwrap());
        let current = home.path().join(".agents/.grove/current");
        let original = fs::read_link(&current).unwrap();
        let script = current.join("plugins/install.sh");
        fs::set_permissions(&script, fs::Permissions::from_mode(0o644)).unwrap();
        assert!(install(home.path(), &codex).unwrap());
        assert!(original.is_dir());
        assert_ne!(fs::read_link(&current).unwrap(), original);
        assert_ne!(
            fs::metadata(script).unwrap().permissions().mode() & 0o111,
            0
        );
        fs::remove_file(current.join("plugins/grove/skills/grove/SKILL.md")).unwrap();
        assert!(install(home.path(), &codex).unwrap());
        assert!(!install(home.path(), &codex).unwrap());
    }

    #[test]
    fn refuses_foreign_current_and_cache_directories() {
        for component in [".grove", "skills"] {
            let home = tempfile::tempdir().unwrap();
            let foreign = tempfile::tempdir().unwrap();
            fs::create_dir(home.path().join(".agents")).unwrap();
            symlink(foreign.path(), home.path().join(".agents").join(component)).unwrap();
            assert!(install(home.path(), &home.path().join(".codex")).is_err());
            assert_eq!(fs::read_dir(foreign.path()).unwrap().count(), 0);
        }
        let home = tempfile::tempdir().unwrap();
        let root = home.path().join(".agents/.grove");
        fs::create_dir_all(&root).unwrap();
        symlink("/foreign/snapshot", root.join("current")).unwrap();
        let error = install(home.path(), &home.path().join(".codex")).unwrap_err();
        assert!(error.to_string().contains("current"));
        assert_eq!(
            fs::read_link(root.join("current")).unwrap(),
            Path::new("/foreign/snapshot")
        );
    }
}

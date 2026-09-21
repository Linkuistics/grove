# Lifecycle startup
<!-- book-page id="three-steps" slice="one-call" order="3" -->
[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)

<a id="one-call"></a>
## Dispatch, then prepare the lifecycle

The CLI first parses the command and chooses the error format; `execute` then
dispatches standalone invocation, its hidden log viewer and sample delivery
before obtaining the current directory. For
`view`, it calls the viewer with the supplied path or that directory and
returns. `config show` likewise returns before lease acquisition, but resolves
a workspace and loads launch policy through SessionConfig. The remaining
statements belong to the bare lifecycle. Tree viewing needs no workspace or
configuration; configuration inspection needs no tree, lease or session epoch.

For bare `grove`, the workspace is resolved once and shared with the lease and
loop. The binary provisions bundled Codex-compatible skills before calling the
loop, so a failed installation cannot launch a child. The loop returns its reason
for stopping; the final match turns that
reason into the process's exit behavior.

<a id="the-block"></a>
## The dispatch block in five fragments

The composite assembles the function's contract and implementation in source
order. The documentation separates ordinary errors from a signal that took the
driver away; the body returns from standalone, display, sample and observation operations
before lifecycle setup.

<!-- fragment «surface-resolve-lease-run» owner="one-call" source="crates/grove/src/cli.rs" lines="120-168" parent="source-command-surface" -->
<!-- insert «run-seam-doc» -->
<!-- insert «run-signal-doc» -->
<!-- insert «run-errors-doc» -->
<!-- insert «run-three-steps» -->
<!-- insert «run-call-and-endings» -->
<!-- /fragment -->

The provisioning adapter is its own complete source root. Keeping it beside
startup makes the causal boundary visible: the lease is already held, but no
template has been loaded and no child can start until this function succeeds.

<!-- fragment «codex-provisioning» owner="one-call" source="crates/grove/src/provision.rs" lines="1-401" parent="source-codex-provisioning" -->
<!-- insert «provision-entry-and-detection» -->
<!-- insert «provision-filesystem-guards» -->
<!-- insert «provision-lock» -->
<!-- insert «provision-link-ownership» -->
<!-- insert «provision-snapshot-comparison» -->
<!-- insert «provision-link-publication» -->
<!-- insert «provision-install-flow» -->
<!-- insert «provision-unit-tests» -->
<!-- /fragment -->

<a id="the-entry-point"></a>
## The entry point

`main.rs` declares the CLI, configuration, provisioning, standalone and display modules and calls
the CLI’s `run`. Its module comment
explains why the binaries and library are separate packages: the application
entry point should call public seams rather than compile private modules into
itself. The CLI reports errors once and returns an `ExitCode`; main propagates that
status without adding a second message.

<!-- fragment «entry-point-three-steps» owner="one-call" source="crates/grove/src/main.rs" lines="1-19" parent="source-entry-point" -->
<!-- insert «entry-point-module-doc» -->
<!-- insert «entry-point-module-and-main» -->
<!-- /fragment -->

The module documentation states which actor owns each path. The loop and viewer
sit behind public library entry points; standalone artifact handling and sample
delivery and Codex provisioning remain binary adapters. Main propagates their selected exit status.

<!-- fragment «entry-point-module-doc» owner="one-call" source="crates/grove/src/main.rs" lines="1-7" parent="entry-point-three-steps" -->
````rust
//! The human's binary: lifecycle, standalone work, observation and examples.
//!
//! The CLI dispatches `run`, `view` and `config` before lifecycle setup. Bare `grove`
//! resolves the working tree and takes the one-driver lease before calling
//! [`grove_loop::run`]. The loop and viewer use public library entry points;
//! standalone artifact handling and sample delivery stay here. Main propagates exit status
//! (`docs/specs/module-decomposition.md`, decision 9).
````
<!-- /fragment -->

The module declaration and main function implement that boundary: one call
returns the CLI result directly to the process runtime.

<!-- fragment «entry-point-module-and-main» owner="one-call" source="crates/grove/src/main.rs" lines="8-19" parent="entry-point-three-steps" -->
````rust

mod cli;
mod config;
mod config_json;
mod examples;
mod provision;
mod run_display;
mod standalone;

fn main() -> std::process::ExitCode {
    cli::run()
}
````
<!-- /fragment -->

<a id="one-resolution"></a>
## One workspace identity

The loop and lease need the same resolved workspace. Resolving independently
would leave two answers that could differ after a path change. `Workspace::resolve`
therefore runs once and passes its result to both consumers. This contract is
about the lifecycle; the viewer's absolute observation path is not a workspace
resolution.

<!-- fragment «run-seam-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="120-128" parent="surface-resolve-lease-run" -->
````rust

/// Dispatch inspection, viewing or example delivery before the lifecycle.
///
/// The workspace is resolved **here**, once, and handed to both the lease and
/// the loop. That is the shape `loop-crate-driver-k22` gave the seam: the lease
/// used to resolve a path itself and hold the answer, so a caller that also
/// needed a workspace had two derivations of one fact and no way to see that
/// they agreed (`docs/adr/one-live-driver-per-working-tree.md`).
///
````
<!-- /fragment -->

<a id="the-three-steps"></a>
## The setup statements

| Statement | Input and result |
|---|---|
| `Cli::try_parse_from` | Shell arguments become an optional command; help/version exit here |
| `standalone::run` | Execute one kind and return before workspace setup |
| `run_display::watch` | Display a parent-selected log and status file |
| `current_dir` | Capture the caller's working directory, or return its I/O error |
| `grove_tui::run` | A view request observes a path and returns before the following steps |
| `config::show` | Validate and display configuration before lifecycle setup |
| `Workspace::resolve` | Find the enclosing jj workspace for bare invocation |
| `DriverLease::acquire` | Refuse a competing lifecycle driver, otherwise hold the lease |
| `provision::ensure_codex_skills` | Install or repair bundled Codex skills, or skip when Codex is absent |
| `TemplateSource::from_env` | Locate launch configuration for the loop to load |

An unset home directory can fail lifecycle configuration discovery. It is never
consulted for a view request. A non-jj directory can be observed, even though
bare invocation there is refused. Both outcomes follow from the early return.

`config examples` follows the standalone and hidden log-viewer branches: it resolves only HOME and returns
before current-directory lookup, workspace discovery or leasing. Missing or broken
active policy and stale ambient epochs therefore cannot change sample delivery.

<!-- fragment «run-three-steps» owner="one-call" source="crates/grove/src/cli.rs" lines="142-162" parent="surface-resolve-lease-run" -->
````rust
fn execute(cli: Cli) -> anyhow::Result<()> {
    if let Some(Command::Run(args)) = cli.command {
        return crate::standalone::run(args);
    }
    if let Some(Command::RunLog { log, status }) = cli.command {
        return crate::run_display::watch(&log, &status);
    }
    if let Some(Command::Config(ConfigCommand::Examples)) = cli.command {
        return crate::examples::run();
    }
    let cwd = std::env::current_dir()?;
    if let Some(Command::View { worktree }) = cli.command {
        return grove_tui::run(&worktree.unwrap_or(cwd));
    }
    if let Some(Command::Config(ConfigCommand::Show { kind, json })) = cli.command {
        return crate::config::show(&cwd, kind.as_deref(), json);
    }
    let workspace = Workspace::resolve(&cwd)?;
    let lease = DriverLease::acquire(&workspace)?;
    crate::provision::ensure_codex_skills()?;
    let templates = TemplateSource::from_env()?;
````
<!-- /fragment -->

<a id="codex-provisioning"></a>
## Provision Codex skills before launch

`ensure_codex_skills` first decides whether Codex exists. An explicit,
nonempty `CODEX_HOME` opts in. Without it, `$HOME/.codex` must already be a
directory; a Claude-only home therefore causes no writes. `HOME` is required
when `CODEX_HOME` is configured because the shared installation lives below
`$HOME/.agents`. The adapter then uses absolute paths so Codex can resolve the
links independently of Grove's current directory.

Every eligible bundled skill is delivered. The build script supplies two
private constants: `FILES` contains every file under `plugins/`, its bytes
and executable bits; `SKILLS` contains skill directories whose frontmatter
declares `harnesses: [any]` or includes `codex`. It sorts both inventories,
rejects duplicate skill names and emits Cargo rebuild directives. That
`build.rs` behavior is evidence for these generated names; the build script
is outside this book's exact corpus.

The shared state is `$HOME/.agents/.grove`: `install.lock` serializes all
Codex homes for the user, `snapshots/` retains complete plugin trees, and
`current` names the active snapshot. Discoverable
`$HOME/.agents/skills/<name>` links point through `current` into each
plugin. The lock covers inspection, staging and publication.

`matches_snapshot` compares the whole recursive inventory, file bytes and
executable flags. A matching snapshot with correct links returns `false`, so
startup is quiet and performs no replacement. A missing or damaged snapshot is
written completely into a temporary directory under `snapshots/`; only then
does one rename replace `current`. Old snapshot directories remain valid for
readers that already resolved them.

Before publishing, the adapter classifies links. It may replace links into
`current/plugins`, retained snapshots, or the recognizable old repository
`plugins/<plugin>/skills/<skill>` layout. A foreign symlink, file or directory
at a canonical skill name is a collision and stops startup. Existing owned
links under legacy `$CODEX_HOME/skills` are repaired, but a fresh legacy tree
is never created. Obsolete links are removed only when the same ownership test
recognizes them. If the legacy and canonical skill paths resolve to the same
directory, device and inode identity collapses them to one reconciliation pass,
so an obsolete owned link cannot be scheduled for removal twice.

The unit tests exercise destination-local link repair, the ownership escape
check, repair of executable bits and missing files with retention of the previous
snapshot, and refusal of foreign cache or current paths without modifying their
targets. The unreadable-file regression verifies that inspection errors name the
affected snapshot file. Operational errors carry the affected path;
`ensure_codex_skills` adds the corrective instruction and returns the error to the CLI, preventing launch. Successful
changes produce one stderr line.

This composite assembles the adapter from eight source fragments. Its actor is
the binary, its input is the detected homes plus the generated inventories, and its result
is either a coherent shared snapshot and link set, a no-op, or an actionable
error before the loop.

The entry and detection fragment owns the public startup operation. It turns
environment state into either a deliberate skip, a selected Codex home, or one
call into the installer.

<!-- fragment «provision-entry-and-detection» owner="one-call" source="crates/grove/src/provision.rs" lines="1-41" parent="codex-provisioning" -->
````rust
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
````
<!-- /fragment -->

Filesystem inspection treats absence as data and every other error as a path
specific failure. Required directories must be real directories rather than
symlinks or other node types.

<!-- fragment «provision-filesystem-guards» owner="one-call" source="crates/grove/src/provision.rs" lines="42-63" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

The shared lock is a regular file opened without following a symlink. Its
bounded nonblocking wait serializes every Grove process using this user's
shared skill directory.

<!-- fragment «provision-lock» owner="one-call" source="crates/grove/src/provision.rs" lines="64-96" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

Ownership is structural. Targets must remain beneath the current or retained
snapshot trees, or match the old bundled repository layout beneath a plugin
named by the generated inventory. The final skill name can be obsolete, allowing
cleanup of old developer links.

<!-- fragment «provision-link-ownership» owner="one-call" source="crates/grove/src/provision.rs" lines="97-132" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

Snapshot comparison rejects special nodes and compares the complete relative
path set before checking each file's bytes and executable bits. This makes an
extra file as stale as a missing or damaged one.

<!-- fragment «provision-snapshot-comparison» owner="one-call" source="crates/grove/src/provision.rs" lines="133-178" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

Publication creates a replacement link beside its destination and renames it
into place. This keeps the rename on one filesystem even when a custom
`CODEX_HOME` is mounted separately, and avoids requiring a writable snapshot
cache merely to repair a missing link.

<!-- fragment «provision-link-publication» owner="one-call" source="crates/grove/src/provision.rs" lines="179-190" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

The install fragment owns the transaction: create and validate shared
directories, acquire the lock, validate current, classify every canonical and
legacy link, stage when required, publish links, and remove only obsolete owned
links. Its boolean reports whether any managed state changed.

<!-- fragment «provision-install-flow» owner="one-call" source="crates/grove/src/provision.rs" lines="191-307" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

The in-module tests exercise ownership refusal, snapshot repair and foreign
directory or current-link refusal using temporary homes. They are inside the
production file's source root and therefore remain exact corpus bytes.

<!-- fragment «provision-unit-tests» owner="one-call" source="crates/grove/src/provision.rs" lines="308-401" parent="codex-provisioning" -->
````rust

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
````
<!-- /fragment -->

<a id="generated-inventory"></a>
## The generated inventory boundary

The `include!` near the top is the runtime/build boundary. Runtime code never
walks the repository and needs no network: it compares and writes the bytes
compiled into `FILES`, then derives links from `SKILLS`. The authoritative
plugin files remain under `plugins/`; `build.rs` converts them to Rust data
for this binary.

<a id="the-signal-path"></a>
## Preserve an interrupted driver’s exit status

A driver killed by SIGTERM or SIGHUP must not report a clean finish. The loop
cleans up its child, drops the lease as it returns, and reports
`LoopOutcome::Interrupted(signal)`. The CLI then calls `reraise` so its parent
observes termination by that signal. This is the lifecycle's contract; the
viewer's current Ctrl-c handling is an ordinary quit through its own terminal
lifetime, and full signal hardening is a later viewer increment.

<!-- fragment «run-signal-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="129-137" parent="surface-resolve-lease-run" -->
````rust
/// **A driver that was killed does not exit 0.** The loop returns *why* it
/// stopped, and one of the reasons is that this process was sent SIGTERM or
/// SIGHUP mid-grove. Every other reason is an outcome the loop was designed to
/// reach and exits cleanly; that one is the loop being taken away, and the only
/// way to say so through a wait status is to die of the same signal after the
/// cleanup — the lease is dropped by the `run` above, and the session was
/// already reaped by the runner. Whoever started `grove` — a systemd unit, a
/// `timeout(1)`, a shell `wait` — then reads `128 + N` instead of success.
///
````
<!-- /fragment -->

The final match maps the returned lifecycle outcome to clean return or signal
re-raising. The standalone runner, log viewer, observation commands and sample installer
have already returned before this match can run.

<!-- fragment «run-call-and-endings» owner="one-call" source="crates/grove/src/cli.rs" lines="163-168" parent="surface-resolve-lease-run" -->
````rust
    match grove_loop::run(&workspace, lease, &templates)? {
        LoopOutcome::Finished | LoopOutcome::Stopped => Ok(()),
        LoopOutcome::Interrupted(signal) => grove_loop::reraise(signal),
    }
}

````
<!-- /fragment -->

<a id="worked-run"></a>
## Worked example: two commands from one directory

From `/work/atlas/src`, with `/work/atlas` a jj workspace:

```text
grove
  parse: no subcommand
  current_dir: /work/atlas/src
  resolve: workspace /work/atlas
  lease: claim the lifecycle driver for /work/atlas
  provision: ensure bundled Codex-compatible skills are available
  templates: locate personal launch policy
  run: the loop launches each selected session and reads its completion signal
```

The same directory gives a different observation path:

```text
grove view
  parse: View with no explicit path
  current_dir: /work/atlas/src
  view: observe /work/atlas/src/.grove
  return: no workspace resolution, lease or template source
```

If that `.grove` is absent, view shows a Missing state and accepts `r` to retry.
An explicit `grove view /work/atlas` observes the parent's tree without changing
the command's no-upward-search rule.

| Lifecycle outcome | Exit behavior |
|---|---|
| `Finished` | `Ok(())`, process exits 0 |
| `Stopped` | `Ok(())`, process exits 0; a later bare invocation resumes |
| `Interrupted(15)` | Re-raise SIGTERM; shell convention reports 143 |
| `Interrupted(1)` | Re-raise SIGHUP; shell convention reports 129 |

A session that ends without signalling stops the loop; it is different from a
signal sent to the driver itself. The returned outcome preserves that distinction.

<a id="what-run-refuses"></a>
## Errors return through main

Workspace, lease and provisioning refusals stop the lifecycle before any session launch.
Configuration or runtime failures can stop it later. A viewer can return a
non-TTY refusal or a terminal setup/input/draw error; its terminal owner restores
modes before that error reaches the reporting boundary. `execute` returns the error
to `run`, which writes human or JSON diagnostics and returns exit 1 to main.

<!-- fragment «run-errors-doc» owner="one-call" source="crates/grove/src/cli.rs" lines="138-141" parent="surface-resolve-lease-run" -->
````rust
/// # Errors
///
/// A working tree that is not a jj workspace, a lease another driver holds, or
/// anything the loop refuses, or a viewer terminal setup/input/draw failure.
````
<!-- /fragment -->

<a id="one-iteration"></a>
## The lifecycle behind the call

One lifecycle iteration reads the tree and launch configuration, performs any
required lifecycle transition, selects a leaf, prepares its session epoch and
completion channel, launches the configured command and waits for the result.
The completion signal decides whether to launch the next session or finish;
an ending without that signal stops the loop. Those details belong to the
loop walkthrough at `docs/walkthroughs/grove-loop/README.md`, whose public `run` is this
chapter's boundary. No viewer action enters that loop.

[Previous: The surface](02-the-surface.md) | [Contents](README.md) | [Next: Proving a negative](04-proving-a-negative.md)

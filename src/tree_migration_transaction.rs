use crate::tree_access::{self, MIGRATION_TRANSACTION};
use crate::tree_migrate;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};

const MANIFEST_FILE: &str = "plan.json";
const READY_FILE: &str = "READY";
const COMMITTED_FILE: &str = "COMMITTED";
const ROLLBACK_DIRECTORY: &str = "rollback";
const MOVED_DIRECTORY: &str = "moved";
const STAGED_DIRECTORY: &str = "staged";
const MANIFEST_VERSION: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TransactionOutcome {
    Migrated,
    RootInitRecovered,
    AlreadyCurrent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Transition {
    WitnessCreated,
    RollbackStaged(PathBuf),
    DestinationStaged(PathBuf),
    ManifestStaged,
    ReadyStaged,
    IncompleteWitnessRemoved,
    SourceMoved(PathBuf),
    DestinationLanded(PathBuf),
    SourceDirectoryRemoved(PathBuf),
    FinalTreeVerified,
    FormatWritten,
    CommitCompleted,
    CommitRecorded,
    WitnessRemoved,
    RollbackFormatRemoved,
    RollbackDestinationRemoved(PathBuf),
    RollbackDestinationDirectoryRemoved(PathBuf),
    RollbackSourceRestored(PathBuf),
}

#[derive(Debug, Deserialize, Serialize)]
struct Manifest {
    version: u32,
    files: Vec<ManifestFile>,
}

#[derive(Debug, Deserialize, Serialize)]
struct ManifestFile {
    source: PathBuf,
    destination: PathBuf,
    source_sha256: String,
    destination_sha256: String,
}

pub(crate) fn run(
    grove_root: &Path,
    commit: impl FnMut() -> Result<()>,
) -> Result<TransactionOutcome> {
    run_observed(grove_root, commit, |_| Ok(()))
}

fn run_observed(
    grove_root: &Path,
    mut commit: impl FnMut() -> Result<()>,
    mut observer: impl FnMut(&Transition) -> Result<()>,
) -> Result<TransactionOutcome> {
    let guard = tree_access::write_for_migration(grove_root)?;
    let grove_root = guard.root();
    let transaction = grove_root.join(MIGRATION_TRANSACTION);
    if transaction.exists() {
        if !transaction.join(READY_FILE).is_file() {
            fs::remove_dir_all(&transaction).with_context(|| {
                format!(
                    "discarding incomplete migration preparation {}",
                    transaction.display()
                )
            })?;
            observer(&Transition::IncompleteWitnessRemoved)?;
        } else {
            let manifest = read_manifest(&transaction)?;
            if transaction.join(COMMITTED_FILE).is_file() {
                verify_final_tree(grove_root, &manifest)?;
                crate::tree_format::require_current(grove_root)?;
                fs::remove_dir_all(&transaction).with_context(|| {
                    format!("removing migration witness {}", transaction.display())
                })?;
                observer(&Transition::WitnessRemoved)?;
                return Ok(TransactionOutcome::Migrated);
            }
            finish_or_rollback(
                grove_root,
                &transaction,
                &manifest,
                &mut commit,
                &mut observer,
            )?;
            return Ok(TransactionOutcome::Migrated);
        }
    }

    if path_entry_exists(&grove_root.join("FORMAT"))? {
        crate::tree_format::require_current(grove_root)?;
        return Ok(TransactionOutcome::AlreadyCurrent);
    }
    if crate::tree_lifecycle::recover_partial_root_init_unlocked(grove_root)? {
        return Ok(TransactionOutcome::RootInitRecovered);
    }

    let plan = tree_migrate::plan_current(grove_root)?;
    if plan.files.is_empty() {
        anyhow::bail!(
            "Grove root {} is neither an exact partial fresh-tree scaffold nor a recognizable \
             legacy tree; refusing to install a current-format witness",
            grove_root.display()
        );
    }

    let manifest = match prepare(grove_root, &transaction, plan, &mut observer) {
        Ok(manifest) => manifest,
        Err(error) => {
            if transaction.exists() {
                if let Err(cleanup_error) = fs::remove_dir_all(&transaction) {
                    anyhow::bail!(
                        "session-kind migration preparation failed: {error:#}; discarding the \
                         incomplete witness failed: {cleanup_error}; the tree remains blocked by {}",
                        transaction.display()
                    );
                }
            }
            return Err(error);
        }
    };
    finish_or_rollback(
        grove_root,
        &transaction,
        &manifest,
        &mut commit,
        &mut observer,
    )?;
    Ok(TransactionOutcome::Migrated)
}

fn finish_or_rollback(
    grove_root: &Path,
    transaction: &Path,
    manifest: &Manifest,
    commit: &mut impl FnMut() -> Result<()>,
    observer: &mut impl FnMut(&Transition) -> Result<()>,
) -> Result<()> {
    let mut commit_completed = false;
    let error = match finish_transaction(
        grove_root,
        transaction,
        manifest,
        commit,
        &mut commit_completed,
        observer,
    ) {
        Ok(()) => return Ok(()),
        Err(error) => error,
    };
    if commit_completed || transaction.join(COMMITTED_FILE).is_file() {
        return Err(error.context(format!(
            "session-kind migration committed but recovery remains pending at {}",
            transaction.display()
        )));
    }

    match rollback(grove_root, transaction, manifest, observer) {
        Ok(()) => Err(error),
        Err(rollback_error) => anyhow::bail!(
            "session-kind migration failed: {error:#}; rollback failed: {rollback_error:#}; \
             the tree remains blocked by {}",
            transaction.display()
        ),
    }
}

fn prepare(
    grove_root: &Path,
    transaction: &Path,
    plan: tree_migrate::CurrentPlan,
    observer: &mut impl FnMut(&Transition) -> Result<()>,
) -> Result<Manifest> {
    fs::create_dir(transaction)
        .with_context(|| format!("creating migration witness {}", transaction.display()))?;
    observer(&Transition::WitnessCreated)?;
    let mut files = Vec::with_capacity(plan.files.len());
    for planned in plan.files {
        let source = grove_root.join(&planned.from_rel);
        let source_body = fs::read(&source)
            .with_context(|| format!("reading migration source {}", source.display()))?;
        let rollback = transaction.join(ROLLBACK_DIRECTORY).join(&planned.from_rel);
        write_new(&rollback, &source_body)?;
        observer(&Transition::RollbackStaged(planned.from_rel.clone()))?;
        let staged = transaction.join(STAGED_DIRECTORY).join(&planned.to_rel);
        write_new(&staged, &planned.body)?;
        observer(&Transition::DestinationStaged(planned.to_rel.clone()))?;
        files.push(ManifestFile {
            source: planned.from_rel,
            destination: planned.to_rel,
            source_sha256: sha256(&source_body),
            destination_sha256: sha256(&planned.body),
        });
    }
    let manifest = Manifest {
        version: MANIFEST_VERSION,
        files,
    };
    let manifest_body =
        serde_json::to_vec_pretty(&manifest).context("serializing migration plan")?;
    write_new(&transaction.join(MANIFEST_FILE), &manifest_body)?;
    observer(&Transition::ManifestStaged)?;
    write_new(&transaction.join(READY_FILE), b"ready\n")?;
    observer(&Transition::ReadyStaged)?;
    Ok(manifest)
}

fn finish_transaction(
    grove_root: &Path,
    transaction: &Path,
    manifest: &Manifest,
    commit: &mut impl FnMut() -> Result<()>,
    commit_completed: &mut bool,
    observer: &mut impl FnMut(&Transition) -> Result<()>,
) -> Result<()> {
    land(grove_root, transaction, manifest, observer)?;
    if path_entry_exists(&grove_root.join("FORMAT"))? {
        crate::tree_format::require_current(grove_root)?;
    } else {
        crate::tree_format::write_current_last(grove_root)?;
        observer(&Transition::FormatWritten)?;
    }
    commit().context("committing the session-kind migration")?;
    *commit_completed = true;
    observer(&Transition::CommitCompleted)?;
    write_new(&transaction.join(COMMITTED_FILE), b"committed\n")?;
    observer(&Transition::CommitRecorded)?;
    verify_final_tree(grove_root, manifest)?;
    crate::tree_format::require_current(grove_root)?;
    fs::remove_dir_all(transaction)
        .with_context(|| format!("removing migration witness {}", transaction.display()))?;
    observer(&Transition::WitnessRemoved)?;
    Ok(())
}

fn land(
    grove_root: &Path,
    transaction: &Path,
    manifest: &Manifest,
    observer: &mut impl FnMut(&Transition) -> Result<()>,
) -> Result<()> {
    for file in &manifest.files {
        let source = grove_root.join(&file.source);
        let moved = transaction.join(MOVED_DIRECTORY).join(&file.source);
        if source.exists() {
            verify_hash(&source, &file.source_sha256, "migration source")?;
            anyhow::ensure!(
                !moved.exists(),
                "migration source exists both in the tree and witness: {}",
                file.source.display()
            );
            create_parent(&moved)?;
            fs::rename(&source, &moved).with_context(|| {
                format!(
                    "moving migration source {} into {}",
                    source.display(),
                    moved.display()
                )
            })?;
            observer(&Transition::SourceMoved(file.source.clone()))?;
        } else {
            verify_hash(&moved, &file.source_sha256, "moved migration source")?;
        }
    }
    for file in &manifest.files {
        let staged = transaction.join(STAGED_DIRECTORY).join(&file.destination);
        let destination = grove_root.join(&file.destination);
        if staged.exists() {
            verify_hash(
                &staged,
                &file.destination_sha256,
                "staged migration destination",
            )?;
            anyhow::ensure!(
                !destination.exists(),
                "migration destination exists both staged and final: {}",
                file.destination.display()
            );
            create_parent(&destination)?;
            fs::rename(&staged, &destination).with_context(|| {
                format!(
                    "landing migration destination {} at {}",
                    staged.display(),
                    destination.display()
                )
            })?;
            observer(&Transition::DestinationLanded(file.destination.clone()))?;
        } else {
            verify_hash(
                &destination,
                &file.destination_sha256,
                "migration destination",
            )?;
        }
    }
    remove_empty_directories(
        grove_root,
        manifest.files.iter().map(|file| file.source.clone()),
        "legacy migration",
        Transition::SourceDirectoryRemoved,
        observer,
    )?;
    verify_final_tree(grove_root, manifest)?;
    observer(&Transition::FinalTreeVerified)?;
    Ok(())
}

fn remove_empty_directories(
    grove_root: &Path,
    paths: impl IntoIterator<Item = PathBuf>,
    role: &str,
    transition: impl Fn(PathBuf) -> Transition,
    observer: &mut impl FnMut(&Transition) -> Result<()>,
) -> Result<()> {
    let mut candidates = BTreeSet::new();
    for path in paths {
        let mut parent = path.parent();
        while let Some(relative) = parent {
            if relative.as_os_str().is_empty() {
                break;
            }
            candidates.insert(relative.to_path_buf());
            parent = relative.parent();
        }
    }
    let mut candidates = candidates.into_iter().collect::<Vec<_>>();
    candidates.sort_by(|left, right| {
        right
            .components()
            .count()
            .cmp(&left.components().count())
            .then_with(|| left.cmp(right))
    });

    for relative in candidates {
        let directory = grove_root.join(&relative);
        match fs::remove_dir(&directory) {
            Ok(()) => observer(&transition(relative))?,
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::DirectoryNotEmpty
                ) => {}
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("removing empty {role} directory {}", directory.display())
                });
            }
        }
    }
    Ok(())
}

fn rollback(
    grove_root: &Path,
    transaction: &Path,
    manifest: &Manifest,
    observer: &mut impl FnMut(&Transition) -> Result<()>,
) -> Result<()> {
    let format_path = grove_root.join("FORMAT");
    if format_path.exists() {
        crate::tree_format::require_current(grove_root)?;
        fs::remove_file(&format_path).with_context(|| {
            format!(
                "removing migration format witness {}",
                format_path.display()
            )
        })?;
        observer(&Transition::RollbackFormatRemoved)?;
    }

    for file in manifest.files.iter().rev() {
        let destination = grove_root.join(&file.destination);
        if destination.exists() {
            verify_hash(
                &destination,
                &file.destination_sha256,
                "migration destination during rollback",
            )?;
            let staged = transaction.join(STAGED_DIRECTORY).join(&file.destination);
            anyhow::ensure!(
                !staged.exists(),
                "migration destination exists both staged and final during rollback: {}",
                file.destination.display()
            );
            create_parent(&staged)?;
            fs::rename(&destination, &staged).with_context(|| {
                format!(
                    "moving migration destination back to staging during rollback {}",
                    destination.display()
                )
            })?;
            observer(&Transition::RollbackDestinationRemoved(
                file.destination.clone(),
            ))?;
        }
    }
    remove_empty_directories(
        grove_root,
        manifest.files.iter().map(|file| file.destination.clone()),
        "migration rollback destination",
        Transition::RollbackDestinationDirectoryRemoved,
        observer,
    )?;

    for file in manifest.files.iter().rev() {
        let source = grove_root.join(&file.source);
        if source.exists() {
            verify_hash(&source, &file.source_sha256, "restored migration source")?;
            continue;
        }
        let moved = transaction.join(MOVED_DIRECTORY).join(&file.source);
        create_parent(&source)?;
        if moved.exists() {
            verify_hash(&moved, &file.source_sha256, "moved migration source")?;
            fs::rename(&moved, &source).with_context(|| {
                format!(
                    "restoring migration source {} from {}",
                    source.display(),
                    moved.display()
                )
            })?;
        } else {
            let rollback_source = transaction.join(ROLLBACK_DIRECTORY).join(&file.source);
            let body = fs::read(&rollback_source).with_context(|| {
                format!("reading rollback source {}", rollback_source.display())
            })?;
            write_new(&source, &body)?;
        }
        observer(&Transition::RollbackSourceRestored(file.source.clone()))?;
    }

    for file in &manifest.files {
        verify_hash(
            &grove_root.join(&file.source),
            &file.source_sha256,
            "rolled-back migration source",
        )?;
    }
    fs::remove_dir_all(transaction)
        .with_context(|| format!("removing rolled-back migration {}", transaction.display()))?;
    Ok(())
}

fn read_manifest(transaction: &Path) -> Result<Manifest> {
    let path = transaction.join(MANIFEST_FILE);
    let body =
        fs::read(&path).with_context(|| format!("reading migration plan {}", path.display()))?;
    let manifest: Manifest = serde_json::from_slice(&body)
        .with_context(|| format!("parsing migration plan {}", path.display()))?;
    validate_manifest(transaction, &manifest)?;
    Ok(manifest)
}

fn validate_manifest(transaction: &Path, manifest: &Manifest) -> Result<()> {
    anyhow::ensure!(
        manifest.version == MANIFEST_VERSION,
        "unsupported migration plan version {} in {}",
        manifest.version,
        transaction.display()
    );
    let mut sources = BTreeSet::new();
    let mut destinations = BTreeSet::new();
    for file in &manifest.files {
        validate_relative_path(&file.source)?;
        validate_relative_path(&file.destination)?;
        anyhow::ensure!(
            sources.insert(&file.source),
            "duplicate migration source in plan: {}",
            file.source.display()
        );
        anyhow::ensure!(
            destinations.insert(&file.destination),
            "duplicate migration destination in plan: {}",
            file.destination.display()
        );
        verify_hash(
            &transaction.join(ROLLBACK_DIRECTORY).join(&file.source),
            &file.source_sha256,
            "rollback source",
        )?;
    }
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    anyhow::ensure!(
        !path.as_os_str().is_empty(),
        "migration plan contains an empty path"
    );
    anyhow::ensure!(
        path.components()
            .all(|component| matches!(component, Component::Normal(_))),
        "migration plan contains a non-relative path: {}",
        path.display()
    );
    Ok(())
}

fn verify_final_tree(grove_root: &Path, manifest: &Manifest) -> Result<()> {
    for file in &manifest.files {
        let destination = grove_root.join(&file.destination);
        let body = fs::read(&destination).with_context(|| {
            format!("verifying migration destination {}", destination.display())
        })?;
        anyhow::ensure!(
            sha256(&body) == file.destination_sha256,
            "migration destination does not match its staged body: {}",
            destination.display()
        );
    }
    Ok(())
}

fn verify_hash(path: &Path, expected: &str, role: &str) -> Result<()> {
    let body = fs::read(path).with_context(|| format!("reading {role} {}", path.display()))?;
    anyhow::ensure!(
        sha256(&body) == expected,
        "{role} does not match the migration plan: {}",
        path.display()
    );
    Ok(())
}

fn write_new(path: &Path, body: &[u8]) -> Result<()> {
    create_parent(path)?;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    std::io::Write::write_all(&mut options.open(path)?, body)
        .with_context(|| format!("writing migration transaction file {}", path.display()))
}

fn create_parent(path: &Path) -> Result<()> {
    let parent = path
        .parent()
        .with_context(|| format!("migration path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent)
        .with_context(|| format!("creating migration directory {}", parent.display()))
}

fn path_entry_exists(path: &Path) -> Result<bool> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(error) => Err(error).with_context(|| format!("checking path entry {}", path.display())),
    }
}

fn sha256(body: &[u8]) -> String {
    format!("{:x}", Sha256::digest(body))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use std::fs;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    fn legacy_tree() -> (tempfile::TempDir, std::path::PathBuf) {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        fs::create_dir(&grove_root).unwrap();
        fs::write(grove_root.join("BRIEF.md"), "# demo — brief\n").unwrap();
        fs::write(
            grove_root.join("01-task-k1.md"),
            "# task-k1\n\n**Kind:** impl\n\n## Goal\nShip it.\n",
        )
        .unwrap();
        (worktree, grove_root)
    }

    fn old_nnn_tree() -> (tempfile::TempDir, std::path::PathBuf) {
        let worktree = tempfile::tempdir().unwrap();
        let grove_root = worktree.path().join(".grove");
        fs::create_dir_all(grove_root.join("done")).unwrap();
        fs::create_dir_all(grove_root.join("020-node")).unwrap();
        fs::write(grove_root.join("BRIEF.md"), "# demo — brief\n").unwrap();
        fs::write(grove_root.join("done/010-old.md"), "# 010-old\n").unwrap();
        fs::write(grove_root.join("020-node/BRIEF.md"), "# 020-node — brief\n").unwrap();
        fs::write(grove_root.join("020-node/010-child.md"), "# 010-child\n").unwrap();
        (worktree, grove_root)
    }

    const ROOT_BRIEF: u8 = 0b01;
    const ROOT_LEAF: u8 = 0b10;

    fn partial_root_scaffold(present: u8) -> (tempfile::TempDir, PathBuf, Vec<u8>, Vec<u8>) {
        let worktree = tempfile::tempdir().unwrap();
        crate::tree_lifecycle::root_init(worktree.path(), "plan").unwrap();
        let grove_root = worktree.path().join(".grove");
        let expected_brief = fs::read(grove_root.join("BRIEF.md")).unwrap();
        let expected_leaf = fs::read(grove_root.join("01-requirements-plan-k1.md")).unwrap();

        fs::remove_dir_all(&grove_root).unwrap();
        fs::create_dir(&grove_root).unwrap();
        if present & ROOT_BRIEF != 0 {
            fs::write(grove_root.join("BRIEF.md"), &expected_brief).unwrap();
        }
        if present & ROOT_LEAF != 0 {
            fs::write(
                grove_root.join("01-requirements-plan-k1.md"),
                &expected_leaf,
            )
            .unwrap();
        }

        (worktree, grove_root, expected_brief, expected_leaf)
    }

    #[test]
    fn every_exact_partial_root_scaffold_is_completed_without_a_migration_commit() {
        for present in 0..=(ROOT_BRIEF | ROOT_LEAF) {
            let (_worktree, grove_root, expected_brief, expected_leaf) =
                partial_root_scaffold(present);
            let commits = Cell::new(0);

            let outcome = run(&grove_root, || {
                commits.set(commits.get() + 1);
                Ok(())
            })
            .unwrap_or_else(|error| {
                panic!("partial scaffold mask {present:#04b} failed: {error:#}")
            });

            assert_eq!(outcome, TransactionOutcome::RootInitRecovered);
            assert_eq!(commits.get(), 0, "partial scaffold mask {present:#04b}");
            assert_eq!(
                fs::read(grove_root.join("BRIEF.md")).unwrap(),
                expected_brief,
                "partial scaffold mask {present:#04b}"
            );
            assert_eq!(
                fs::read(grove_root.join("01-requirements-plan-k1.md")).unwrap(),
                expected_leaf,
                "partial scaffold mask {present:#04b}"
            );
            assert_eq!(
                fs::read(grove_root.join("FORMAT")).unwrap(),
                b"session-kinds-v1\n",
                "partial scaffold mask {present:#04b}"
            );
            assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
        }
    }

    #[test]
    fn an_exact_interrupted_format_temporary_file_is_reused_by_recovery() {
        let (_worktree, grove_root, expected_brief, expected_leaf) =
            partial_root_scaffold(ROOT_BRIEF | ROOT_LEAF);
        fs::write(grove_root.join(".FORMAT.tmp"), b"session-kinds-v1\n").unwrap();

        let outcome = run(&grove_root, || anyhow::bail!("must not commit")).unwrap();

        assert_eq!(outcome, TransactionOutcome::RootInitRecovered);
        assert_eq!(
            fs::read(grove_root.join("BRIEF.md")).unwrap(),
            expected_brief
        );
        assert_eq!(
            fs::read(grove_root.join("01-requirements-plan-k1.md")).unwrap(),
            expected_leaf
        );
        assert_eq!(
            fs::read(grove_root.join("FORMAT")).unwrap(),
            b"session-kinds-v1\n"
        );
        assert!(!grove_root.join(".FORMAT.tmp").exists());
    }

    #[test]
    fn a_complete_current_root_bypasses_partial_recovery() {
        let worktree = tempfile::tempdir().unwrap();
        crate::tree_lifecycle::root_init(worktree.path(), "plan").unwrap();
        let grove_root = worktree.path().join(".grove");
        let commits = Cell::new(0);

        let outcome = run(&grove_root, || {
            commits.set(commits.get() + 1);
            Ok(())
        })
        .unwrap();

        assert_eq!(outcome, TransactionOutcome::AlreadyCurrent);
        assert_eq!(commits.get(), 0);
    }

    #[test]
    fn differing_partial_scaffold_bodies_are_refused_without_writing_missing_files() {
        for (present, path) in [
            (ROOT_BRIEF, "BRIEF.md"),
            (ROOT_LEAF, "01-requirements-plan-k1.md"),
        ] {
            let (_worktree, grove_root, _, _) = partial_root_scaffold(present);
            fs::write(grove_root.join(path), b"near match\n").unwrap();

            let error = run(&grove_root, || anyhow::bail!("must not commit")).unwrap_err();

            let diagnostic = format!("{error:#}");
            assert!(diagnostic.contains("partial root scaffold"), "{diagnostic}");
            assert!(diagnostic.contains(path), "{diagnostic}");
            assert_eq!(fs::read(grove_root.join(path)).unwrap(), b"near match\n");
            assert!(!grove_root.join("FORMAT").exists());
            if present == ROOT_BRIEF {
                assert!(!grove_root.join("01-requirements-plan-k1.md").exists());
            } else {
                assert!(!grove_root.join("BRIEF.md").exists());
            }
        }
    }

    #[test]
    fn extra_task_structure_makes_an_exact_partial_scaffold_ambiguous() {
        let (_worktree, grove_root, expected_brief, _) = partial_root_scaffold(ROOT_BRIEF);
        fs::write(
            grove_root.join("02-task-k2.md"),
            b"# task-k2\n\n**Kind:** impl\n",
        )
        .unwrap();

        let error = run(&grove_root, || anyhow::bail!("must not commit")).unwrap_err();

        let diagnostic = format!("{error:#}");
        assert!(
            diagnostic.contains("ambiguous partial root scaffold"),
            "{diagnostic}"
        );
        assert!(diagnostic.contains("02-task-k2.md"), "{diagnostic}");
        assert_eq!(
            fs::read(grove_root.join("BRIEF.md")).unwrap(),
            expected_brief
        );
        assert!(grove_root.join("02-task-k2.md").is_file());
        assert!(!grove_root.join("01-requirements-plan-k1.md").exists());
        assert!(!grove_root.join("FORMAT").exists());
    }

    #[test]
    fn scaffold_path_collisions_are_refused_without_overwrite() {
        let (_worktree, grove_root, expected_brief, _) = partial_root_scaffold(ROOT_BRIEF);
        let leaf_path = grove_root.join("01-requirements-plan-k1.md");
        fs::create_dir(&leaf_path).unwrap();
        fs::write(leaf_path.join("foreign.txt"), b"keep\n").unwrap();

        let error = run(&grove_root, || anyhow::bail!("must not commit")).unwrap_err();

        let diagnostic = format!("{error:#}");
        assert!(diagnostic.contains("partial root scaffold"), "{diagnostic}");
        assert!(
            diagnostic.contains("01-requirements-plan-k1.md"),
            "{diagnostic}"
        );
        assert_eq!(
            fs::read(grove_root.join("BRIEF.md")).unwrap(),
            expected_brief
        );
        assert_eq!(fs::read(leaf_path.join("foreign.txt")).unwrap(), b"keep\n");
        assert!(!grove_root.join("FORMAT").exists());
    }

    #[test]
    fn a_foreign_only_partial_root_is_not_promoted_to_an_empty_current_grove() {
        let (_worktree, grove_root, _, _) = partial_root_scaffold(0);
        fs::write(grove_root.join("notes.txt"), b"keep\n").unwrap();

        let error = run(&grove_root, || anyhow::bail!("must not commit")).unwrap_err();

        let diagnostic = format!("{error:#}");
        assert!(
            diagnostic.contains("exact partial fresh-tree scaffold"),
            "{diagnostic}"
        );
        assert!(
            diagnostic.contains("recognizable legacy tree"),
            "{diagnostic}"
        );
        assert_eq!(fs::read(grove_root.join("notes.txt")).unwrap(), b"keep\n");
        assert!(!grove_root.join("BRIEF.md").exists());
        assert!(!grove_root.join("01-requirements-plan-k1.md").exists());
        assert!(!grove_root.join("FORMAT").exists());
        assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
    }

    #[test]
    fn legacy_migration_refuses_a_symlinked_format_temporary_file_without_clobbering_target() {
        let (worktree, grove_root) = legacy_tree();
        let external = worktree.path().join("external-format-target");
        fs::write(&external, b"keep\n").unwrap();
        std::os::unix::fs::symlink(&external, grove_root.join(".FORMAT.tmp")).unwrap();
        let commits = Cell::new(0);

        let error = run(&grove_root, || {
            commits.set(commits.get() + 1);
            Ok(())
        })
        .unwrap_err();

        let diagnostic = format!("{error:#}");
        assert!(diagnostic.contains(".FORMAT.tmp"), "{diagnostic}");
        assert_eq!(commits.get(), 0);
        assert_eq!(fs::read(&external).unwrap(), b"keep\n");
        assert!(fs::symlink_metadata(grove_root.join(".FORMAT.tmp"))
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(!grove_root.join("FORMAT").exists());
    }

    #[test]
    fn legacy_migration_refuses_a_dangling_format_symlink_without_replacing_it() {
        let (worktree, grove_root) = legacy_tree();
        let missing_target = worktree.path().join("missing-format-target");
        std::os::unix::fs::symlink(&missing_target, grove_root.join("FORMAT")).unwrap();
        let commits = Cell::new(0);

        let error = run(&grove_root, || {
            commits.set(commits.get() + 1);
            Ok(())
        })
        .unwrap_err();

        let diagnostic = format!("{error:#}");
        assert!(diagnostic.contains("FORMAT"), "{diagnostic}");
        assert_eq!(commits.get(), 0);
        assert!(fs::symlink_metadata(grove_root.join("FORMAT"))
            .unwrap()
            .file_type()
            .is_symlink());
        assert!(grove_root.join("01-task-k1.md").is_file());
        assert!(!grove_root.join("01-impl-task-k1.md").exists());
    }

    #[test]
    fn transaction_lands_the_complete_plan_and_writes_format_last() {
        let (_worktree, grove_root) = legacy_tree();
        let commits = Cell::new(0);

        let outcome = run(&grove_root, || {
            commits.set(commits.get() + 1);
            Ok(())
        })
        .unwrap();

        assert_eq!(outcome, TransactionOutcome::Migrated);
        assert_eq!(commits.get(), 1);
        assert!(!grove_root.join("01-task-k1.md").exists());
        assert_eq!(
            fs::read_to_string(grove_root.join("01-impl-task-k1.md")).unwrap(),
            "# task-k1\n\n\n## Goal\nShip it.\n"
        );
        assert_eq!(
            fs::read_to_string(grove_root.join("FORMAT")).unwrap(),
            "session-kinds-v1\n"
        );
        assert!(!grove_root.join("MIGRATING-session-kinds").exists());
    }

    #[test]
    fn transaction_removes_empty_legacy_source_directories() {
        let (_worktree, grove_root) = old_nnn_tree();

        run(&grove_root, || Ok(())).unwrap();

        assert!(!grove_root.join("done").exists());
        assert!(!grove_root.join("020-node").exists());
        assert!(grove_root.join("01-DONE-impl-old-k1.md").is_file());
        assert!(grove_root.join("02-node-k2/BRIEF.md").is_file());
        assert!(grove_root.join("02-node-k2/01-impl-child-k3.md").is_file());
    }

    #[test]
    fn every_transaction_transition_boundary_recovers_after_interruption() {
        let (_baseline_worktree, baseline_root) = old_nnn_tree();
        let mut transitions = Vec::new();
        run_observed(
            &baseline_root,
            || Ok(()),
            |transition| {
                transitions.push(transition.clone());
                Ok(())
            },
        )
        .unwrap();
        assert!(!transitions.is_empty());

        for (interrupt_at, interrupted_transition) in transitions.iter().enumerate() {
            let (_worktree, grove_root) = old_nnn_tree();
            let mut observed = 0;
            let interrupted = catch_unwind(AssertUnwindSafe(|| {
                let _ = run_observed(
                    &grove_root,
                    || Ok(()),
                    |_| {
                        if observed == interrupt_at {
                            panic!("simulated process interruption");
                        }
                        observed += 1;
                        Ok(())
                    },
                );
            }));
            assert!(
                interrupted.is_err(),
                "boundary {interrupt_at} ({:?}) did not interrupt",
                interrupted_transition
            );

            run(&grove_root, || Ok(())).unwrap_or_else(|error| {
                panic!(
                    "boundary {interrupt_at} ({:?}) did not recover: {error:#}",
                    interrupted_transition
                )
            });
            assert!(
                grove_root.join("01-DONE-impl-old-k1.md").is_file(),
                "boundary {interrupt_at}: {:?}",
                interrupted_transition
            );
            assert!(grove_root.join("02-node-k2/BRIEF.md").is_file());
            assert!(grove_root.join("02-node-k2/01-impl-child-k3.md").is_file());
            assert!(!grove_root.join("done").exists());
            assert!(!grove_root.join("020-node").exists());
            assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
        }
    }

    #[test]
    fn every_rollback_transition_boundary_recovers_after_interruption() {
        fn is_rollback(transition: &Transition) -> bool {
            matches!(
                transition,
                Transition::RollbackFormatRemoved
                    | Transition::RollbackDestinationRemoved(_)
                    | Transition::RollbackDestinationDirectoryRemoved(_)
                    | Transition::RollbackSourceRestored(_)
            )
        }

        let (_baseline_worktree, baseline_root) = old_nnn_tree();
        let mut rollback_transitions = Vec::new();
        run_observed(
            &baseline_root,
            || Ok(()),
            |transition| {
                if matches!(transition, Transition::FormatWritten) {
                    anyhow::bail!("trigger baseline rollback");
                }
                if is_rollback(transition) {
                    rollback_transitions.push(transition.clone());
                }
                Ok(())
            },
        )
        .unwrap_err();
        assert!(!rollback_transitions.is_empty());

        for (interrupt_at, interrupted_transition) in rollback_transitions.iter().enumerate() {
            let (_worktree, grove_root) = old_nnn_tree();
            let mut observed = 0;
            let interrupted = catch_unwind(AssertUnwindSafe(|| {
                let _ = run_observed(
                    &grove_root,
                    || Ok(()),
                    |transition| {
                        if matches!(transition, Transition::FormatWritten) {
                            anyhow::bail!("trigger interrupted rollback");
                        }
                        if is_rollback(transition) {
                            if observed == interrupt_at {
                                panic!("simulated rollback interruption");
                            }
                            observed += 1;
                        }
                        Ok(())
                    },
                );
            }));
            assert!(
                interrupted.is_err(),
                "rollback boundary {interrupt_at} ({:?}) did not interrupt",
                interrupted_transition
            );

            run(&grove_root, || Ok(())).unwrap_or_else(|error| {
                panic!(
                    "rollback boundary {interrupt_at} ({:?}) did not recover: {error:#}",
                    interrupted_transition
                )
            });
            assert!(grove_root.join("01-DONE-impl-old-k1.md").is_file());
            assert!(grove_root.join("02-node-k2/BRIEF.md").is_file());
            assert!(grove_root.join("02-node-k2/01-impl-child-k3.md").is_file());
            assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
        }
    }

    #[test]
    fn reported_precommit_failure_rolls_back_exact_source_bytes() {
        let (_worktree, grove_root) = legacy_tree();
        let original = fs::read(grove_root.join("01-task-k1.md")).unwrap();
        let commits = Cell::new(0);

        let error = run_observed(
            &grove_root,
            || {
                commits.set(commits.get() + 1);
                Ok(())
            },
            |transition| {
                if matches!(transition, Transition::DestinationLanded(_)) {
                    anyhow::bail!("injected landing failure");
                }
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error.to_string().contains("injected landing failure"));
        assert_eq!(commits.get(), 0);
        assert_eq!(
            fs::read(grove_root.join("01-task-k1.md")).unwrap(),
            original
        );
        assert!(!grove_root.join("01-impl-task-k1.md").exists());
        assert!(!grove_root.join("FORMAT").exists());
        assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
    }

    #[test]
    fn reported_precommit_failure_restores_the_exact_legacy_tree_shape() {
        let (_worktree, grove_root) = old_nnn_tree();

        let error = run_observed(
            &grove_root,
            || Ok(()),
            |transition| {
                if matches!(transition, Transition::FormatWritten) {
                    anyhow::bail!("injected failure after complete landing");
                }
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("injected failure after complete landing"));
        assert_eq!(
            fs::read_to_string(grove_root.join("done/010-old.md")).unwrap(),
            "# 010-old\n"
        );
        assert_eq!(
            fs::read_to_string(grove_root.join("020-node/BRIEF.md")).unwrap(),
            "# 020-node — brief\n"
        );
        assert_eq!(
            fs::read_to_string(grove_root.join("020-node/010-child.md")).unwrap(),
            "# 010-child\n"
        );
        assert!(!grove_root.join("01-DONE-impl-old-k1.md").exists());
        assert!(!grove_root.join("02-node-k2").exists());
        assert!(!grove_root.join("FORMAT").exists());
        assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
    }

    #[test]
    fn reported_preparation_failure_discards_the_witness_without_touching_sources() {
        let (_worktree, grove_root) = legacy_tree();
        let original = fs::read(grove_root.join("01-task-k1.md")).unwrap();

        let error = run_observed(
            &grove_root,
            || Ok(()),
            |transition| {
                if matches!(transition, Transition::ReadyStaged) {
                    anyhow::bail!("injected preparation failure");
                }
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error.to_string().contains("injected preparation failure"));
        assert_eq!(
            fs::read(grove_root.join("01-task-k1.md")).unwrap(),
            original
        );
        assert!(!grove_root.join("01-impl-task-k1.md").exists());
        assert!(!grove_root.join("FORMAT").exists());
        assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
    }

    #[test]
    fn failure_after_commit_never_rolls_the_tree_back() {
        let (_worktree, grove_root) = legacy_tree();
        let commits = Cell::new(0);

        let error = run_observed(
            &grove_root,
            || {
                commits.set(commits.get() + 1);
                Ok(())
            },
            |transition| {
                if matches!(transition, Transition::CommitCompleted) {
                    anyhow::bail!("injected post-commit failure");
                }
                Ok(())
            },
        )
        .unwrap_err();

        assert!(error
            .to_string()
            .contains("committed but recovery remains pending"));
        assert_eq!(commits.get(), 1);
        assert!(!grove_root.join("01-task-k1.md").exists());
        assert!(grove_root.join("01-impl-task-k1.md").is_file());
        assert!(grove_root.join("FORMAT").is_file());
        assert!(grove_root.join(MIGRATION_TRANSACTION).is_dir());

        run(&grove_root, || {
            commits.set(commits.get() + 1);
            Ok(())
        })
        .unwrap();
        assert_eq!(commits.get(), 2);
        assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
    }

    #[test]
    fn commit_failure_is_precommit_and_rolls_back() {
        let (_worktree, grove_root) = legacy_tree();
        let original = fs::read(grove_root.join("01-task-k1.md")).unwrap();

        let error = run(&grove_root, || anyhow::bail!("injected commit failure")).unwrap_err();

        assert!(format!("{error:#}").contains("injected commit failure"));
        assert_eq!(
            fs::read(grove_root.join("01-task-k1.md")).unwrap(),
            original
        );
        assert!(!grove_root.join("01-impl-task-k1.md").exists());
        assert!(!grove_root.join("FORMAT").exists());
        assert!(!grove_root.join(MIGRATION_TRANSACTION).exists());
    }

    #[test]
    fn committed_final_mismatch_keeps_the_witness_and_refuses_recovery() {
        let (_worktree, grove_root) = legacy_tree();
        let commits = Cell::new(0);

        let interrupted = catch_unwind(AssertUnwindSafe(|| {
            let _ = run_observed(
                &grove_root,
                || {
                    commits.set(commits.get() + 1);
                    Ok(())
                },
                |transition| {
                    if matches!(transition, Transition::CommitRecorded) {
                        panic!("simulated interruption after commit record");
                    }
                    Ok(())
                },
            );
        }));
        assert!(interrupted.is_err());
        fs::write(
            grove_root.join("01-impl-task-k1.md"),
            "corrupt committed destination\n",
        )
        .unwrap();

        let error = run(&grove_root, || {
            commits.set(commits.get() + 1);
            Ok(())
        })
        .unwrap_err();

        assert!(error.to_string().contains("does not match its staged body"));
        assert_eq!(commits.get(), 1);
        assert!(grove_root.join(MIGRATION_TRANSACTION).is_dir());
    }

    #[test]
    fn rollback_failure_leaves_the_tree_blocked_by_the_witness() {
        let (_worktree, grove_root) = legacy_tree();

        let error = run_observed(
            &grove_root,
            || Ok(()),
            |transition| match transition {
                Transition::DestinationLanded(_) => anyhow::bail!("injected landing failure"),
                Transition::RollbackDestinationRemoved(_) => {
                    anyhow::bail!("injected rollback failure")
                }
                _ => Ok(()),
            },
        )
        .unwrap_err();

        let diagnostic = error.to_string();
        assert!(diagnostic.contains("injected landing failure"));
        assert!(diagnostic.contains("injected rollback failure"));
        assert!(diagnostic.contains("tree remains blocked"));
        assert!(grove_root.join(MIGRATION_TRANSACTION).is_dir());
        let read_error = match tree_access::read(&grove_root) {
            Ok(_) => panic!("reader admitted a failed rollback"),
            Err(error) => error,
        };
        assert!(read_error
            .to_string()
            .contains("pending Grove session-kind migration"));
    }
}

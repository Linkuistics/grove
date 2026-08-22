//! The interpreter: the one component that applies a plan and unwinds what it
//! applied.
//!
//! One interpreter and one rollback, shared by every operation. That is the
//! whole reason a plan is a *value* rather than five hand-written procedures:
//! five unwinds are five things that drift apart, and atomicity becomes five
//! properties instead of one. If an operation ever needs a different rollback,
//! `ARCHITECTURE.md` is explicit that this is a finding about the plan shape and
//! not a licence to add a second interpreter.
//!
//! The specification is `docs/ordinal-fs-tree/ARCHITECTURE.md`, sections *How an
//! operation runs*, *When rollback fails* and the invariant *Plan atomicity*;
//! the model is `operations.qnt`'s `applyStep…`/`unwindStep…` actions and its
//! `failures` and `rollback_fails` instances.
//!
//! # The promise is bounded, and the bound is in the type
//!
//! Rollback covers **reported errors**. A process killed mid-apply is not
//! recoverable and the library says so rather than implying otherwise, and a
//! rollback that *itself* fails leaves the tree in neither the state it was
//! found in nor the one intended — which is [`Error::FailedPartiallyRolledBack`]
//! and not [`Error::Failed`]. Two variants, because a consumer that cannot tell
//! them apart has been promised something the library does not do: on the
//! promotion path the second one leaves a leaf and a node sharing an ordinal and
//! a key, and that is the single path by which this library damages a tree it
//! was handed.
//!
//! # Every destination is claimed, not assumed
//!
//! A `create_new` for a leaf, a `create_dir` for a node, and — for a rename,
//! which `rename(2)` would otherwise perform *over* whatever is there — an
//! explicit unfollowed look before the call. The algebra already folded the plan
//! through the snapshot, so under the lock this can only fire on a plan that
//! collides with itself, and `operations.qnt`'s
//! `inv_interpreterNeverFindsADestinationTaken` says it never does. It stays
//! because **the lock is advisory**: a writer that does not take it can occupy a
//! destination between the snapshot and the apply, and that neighbour is the
//! only thing left that this check catches. Without this paragraph the check
//! reads as dead code to whoever next tidies up.

use std::fs;
use std::io;
use std::io::Write as _;
use std::path::{Path, PathBuf};

use crate::plan::{Effect, Level, Plan};
use crate::report::Report;
use crate::{EntryName, EntryNameExt, Error, Snapshot, Species};

/// Apply a plan under the exclusive lock, or leave the tree as it was found.
pub(super) fn apply<N: EntryName>(
    root: &Path,
    snapshot: &Snapshot<N>,
    plan: &Plan<N>,
    faults: Faults,
) -> Result<Report<N>, Error<N>> {
    // The seventh obligation, at the second of the two boundaries where a name
    // becomes a path — and **before any effect runs**, so a plan carrying one
    // bad name changes nothing rather than landing what it can and unwinding.
    // The snapshot's own names were checked when it was read, so between the two
    // checks every rendering this function will join is one path component.
    for effect in plan.effects() {
        let rendered = effect.name().to_string();
        if let Some(reason) = crate::name::not_one_component(&rendered) {
            return Err(Error::NameIsNotOneComponent {
                root: root.to_path_buf(),
                rendered,
                reason,
            });
        }
    }
    let mut run = Run {
        root,
        snapshot,
        faults,
        landed: Vec::new(),
        moved: Vec::new(),
        undo: Vec::new(),
        report: Report::empty(),
    };
    for (index, effect) in plan.effects().iter().enumerate() {
        if let Err(failure) = run.step(index, effect) {
            return Err(run.unwind(failure));
        }
    }
    Ok(run.report)
}

/// One application of one plan.
struct Run<'a, N> {
    root: &'a Path,
    snapshot: &'a Snapshot<N>,
    faults: Faults,
    /// The destination of each effect that has landed, by its position in the
    /// plan — which is what [`Level::Created`] names.
    landed: Vec<PathBuf>,
    /// Where each entry this run has moved now lives, so that a plan moving one
    /// entry twice reads its *current* path rather than the snapshot's stale
    /// one.
    moved: Vec<(usize, PathBuf)>,
    /// How to undo what has landed, in the order it landed — captured against
    /// the state before each effect, and walked backwards.
    undo: Vec<Undo>,
    report: Report<N>,
}

impl<N: EntryName> Run<'_, N> {
    /// Apply one effect, recording how to undo it the moment its destination is
    /// claimed.
    fn step(&mut self, index: usize, effect: &Effect<N>) -> Result<(), Failure> {
        let landed = match effect {
            Effect::Create { at, name, content } => {
                let path = self.level_path(*at).join(name.to_string());
                self.faults.strike_effect(index, &path)?;
                match name.species() {
                    // `create_dir` fails rather than succeeding when something
                    // is already there, which is the exclusive claim.
                    Species::Node => {
                        fs::create_dir(&path).map_err(|source| Failure {
                            path: path.clone(),
                            doing: "creating the node",
                            source,
                        })?;
                        self.undo.push(Undo::Remove {
                            path: path.clone(),
                            species: Species::Node,
                        });
                    }
                    // `create_new` is the same claim for a regular file: it is
                    // one syscall with `O_EXCL`, so nothing can slip between the
                    // question and the answer.
                    Species::Leaf | Species::Distinguished => {
                        let mut file = fs::OpenOptions::new()
                            .write(true)
                            .create_new(true)
                            .open(&path)
                            .map_err(|source| Failure {
                                path: path.clone(),
                                doing: "creating the leaf",
                                source,
                            })?;
                        // Registered before the bytes are written, deliberately:
                        // from here the path is this run's to remove, and a
                        // write that fails half way must still be unwound.
                        self.undo.push(Undo::Remove {
                            path: path.clone(),
                            species: Species::Leaf,
                        });
                        // And this is the control on that *deliberately*. The
                        // ordering above is a second mechanism behind atomicity,
                        // and `strike_effect` above cannot reach it: it fires
                        // before the create, so moving the registration below the
                        // write leaves every whole-effect test green while a real
                        // short write returns `Error::Failed` over a partial file
                        // the variant promises was removed. This seam stands in
                        // for that write, in the one interval where the file
                        // exists and its bytes do not.
                        self.faults.strike_content(index, &path)?;
                        file.write_all(content).map_err(|source| Failure {
                            path: path.clone(),
                            doing: "writing the leaf's content",
                            source,
                        })?;
                    }
                }
                self.report.record_created(name.clone(), path.clone());
                path
            }
            Effect::MoveTo { entry, to, name } => {
                let from = self.entry_path(*entry);
                let path = self.level_path(*to).join(name.to_string());
                self.faults.strike_effect(index, &path)?;
                // A move onto the entry's own path, which is what a `rewrite` to
                // the parts an entry already carries plans. The algebra excludes
                // the mover from occupancy for exactly this — `operations.qnt`'s
                // `wit_rewriteToSameParts` says the no-op must **succeed** — and
                // that exclusion has to be carried across the boundary or the
                // interpreter refuses the plan the algebra just proved
                // applicable. One property, two mechanisms; this is the second.
                //
                // Nothing is claimed and nothing is undone, and both follow from
                // the same fact: the destination is the source, so it is occupied
                // by the very entry being moved, and `rename(2)` on one path is
                // defined to change nothing. An `Undo::Restore` here would be a
                // rename onto an occupied path — its own — which `claim_vacant`
                // would refuse, turning a clean unwind into
                // `FailedPartiallyRolledBack`.
                let noop = from == path;
                if !noop {
                    claim_vacant(&path, "renaming onto")?;
                    fs::rename(&from, &path).map_err(|source| Failure {
                        path: path.clone(),
                        doing: "renaming the entry to",
                        source,
                    })?;
                    self.undo.push(Undo::Restore {
                        from: path.clone(),
                        to: from.clone(),
                    });
                }
                self.moved.push((*entry, path.clone()));
                // Reported either way. The operation did place this name, and a
                // consumer reading `renamed()` to learn where an entry now lives
                // needs the answer whether or not the filesystem was touched.
                self.report.record_renamed(name.clone(), from, path.clone());
                path
            }
        };
        self.landed.push(landed);
        Ok(())
    }

    /// Undo what landed, in reverse, and say which of the two failures this was.
    fn unwind(self, failure: Failure) -> Error<N> {
        for (index, step) in self.undo.into_iter().rev().enumerate() {
            if let Err(unwound) = step.perform(index, self.faults) {
                return Error::FailedPartiallyRolledBack {
                    path: failure.path,
                    doing: failure.doing,
                    source: failure.source,
                    unwinding: unwound.path,
                    undoing: unwound.doing,
                    unwind_source: unwound.source,
                };
            }
        }
        Error::Failed {
            path: failure.path,
            doing: failure.doing,
            source: failure.source,
        }
    }

    /// The directory a plan's [`Level`] names.
    ///
    /// # Panics
    ///
    /// If the level names an effect that has not landed, or one that created no
    /// directory. Neither is reachable through this crate's operations: a plan
    /// is built in order, so a `Created` level always names an earlier effect.
    fn level_path(&self, level: Level) -> PathBuf {
        match level {
            Level::Root => self.root.to_path_buf(),
            Level::Entry(index) => self.entry_path(index),
            Level::Created(effect) => self
                .landed
                .get(effect)
                .expect("a plan names only levels its earlier effects created")
                .clone(),
        }
    }

    /// Where an entry of the snapshot is *now*: the caller's spelling of the
    /// root, then every containing node's name, then its own — unless this run
    /// has already moved it, in which case where it moved it to.
    fn entry_path(&self, index: usize) -> PathBuf {
        if let Some((_, path)) = self.moved.iter().rev().find(|(moved, _)| *moved == index) {
            return path.clone();
        }
        let entry = self.snapshot.at(index);
        let mut path = self.root.to_path_buf();
        for container in entry.ancestors() {
            if let Some(node) = container.entry() {
                path.push(node.name().to_string());
            }
        }
        path.push(entry.name().to_string());
        path
    }
}

/// How to undo one effect that landed.
///
/// Two variants, and the first is the only removal this library ever performs.
/// `operations.qnt` gives `Effect` a `Remove` variant and then records in a
/// comment that it *never appears in a forward plan*; here that comment is the
/// type system's, because a `Remove` can only be built from a `Create` this run
/// just performed. The model's `inv_rollbackRemovesOnlyItsOwn` — *rollback
/// removes only entries the run itself created, so it cannot destroy something
/// that was already there* — is therefore structural rather than checked at
/// run time.
enum Undo {
    /// Remove something this run created, one moment ago.
    Remove {
        /// Exactly the path the create claimed.
        path: PathBuf,
        /// Which of the two removals it takes.
        species: Species,
    },
    /// Put an entry this run moved back where it was.
    Restore {
        /// Where the run left it.
        from: PathBuf,
        /// Where it was found.
        to: PathBuf,
    },
}

impl Undo {
    fn perform(self, index: usize, faults: Faults) -> Result<(), Failure> {
        match self {
            Self::Remove { path, species } => {
                faults.strike_unwind(index, &path)?;
                // A directory this run created can only hold things this run
                // put in it, and those were unwound first — so it is empty, and
                // `remove_dir` refusing a non-empty one is a check rather than
                // an obstacle.
                let removal = if species == Species::Node {
                    fs::remove_dir(&path)
                } else {
                    fs::remove_file(&path)
                };
                removal.map_err(|source| Failure {
                    path,
                    doing: "removing what this operation had created at",
                    source,
                })
            }
            Self::Restore { from, to } => {
                faults.strike_unwind(index, &to)?;
                claim_vacant(&to, "restoring")?;
                fs::rename(&from, &to).map_err(|source| Failure {
                    path: to,
                    doing: "putting back the entry this operation had moved from",
                    source,
                })
            }
        }
    }
}

/// Refuse a destination that is occupied by anything at all, deciding it
/// **without following links**.
///
/// `rename(2)` replaces its destination silently, so a rename is the one effect
/// whose claim cannot be the syscall itself. macOS has no portable no-replace
/// rename, so the claim is a look followed by the call — which is not atomic
/// against a racing writer, and does not need to be: the writer it would have to
/// beat is one that ignores the advisory lock, which is already outside what the
/// library defends against. What this does defend is the common case that
/// matters, a destination that was occupied *before* the rename started.
///
/// An occupancy that cannot be determined is a refusal rather than an
/// assumption, which is why anything but `NotFound` is a failure.
fn claim_vacant(path: &Path, doing: &'static str) -> Result<(), Failure> {
    // `symlink_metadata` and not `metadata`: a symbolic link occupies a name
    // whatever it points at, and a dangling one occupies it too.
    match fs::symlink_metadata(path) {
        Ok(_) => Err(Failure {
            path: path.to_path_buf(),
            doing,
            source: io::Error::new(
                io::ErrorKind::AlreadyExists,
                "something is already there, and this operation will not replace it",
            ),
        }),
        Err(source) if source.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(source) => Err(Failure {
            path: path.to_path_buf(),
            doing,
            source,
        }),
    }
}

/// One effect, or one undo, that did not happen.
struct Failure {
    path: PathBuf,
    doing: &'static str,
    source: io::Error,
}

/// The seam that makes an effect fail on demand.
///
/// **Internal, and it must stay internal.** A second *public* seam would
/// contradict `docs/adr/entry-name-is-the-only-seam.md`, which is the record
/// saying the entry name is the only genericity this library has. It is here
/// because atomicity is otherwise untestable: the property is *after a mutation
/// returns an error, either every effect landed or none did*, and without a way
/// to make effect two of three fail there is no error to observe.
///
/// A production build carries the three `None`s and the branches that read
/// them, and nothing else — the constructors below are compiled only under
/// `cfg(test)`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Faults {
    effect: Option<usize>,
    content: Option<usize>,
    unwind: Option<usize>,
}

impl Faults {
    /// What every operation runs with.
    pub(crate) const fn none() -> Self {
        Self {
            effect: None,
            content: None,
            unwind: None,
        }
    }

    /// Fail the effect at this position in the plan, before it has touched
    /// anything.
    #[cfg(test)]
    pub(crate) const fn at_effect(index: usize) -> Self {
        Self {
            effect: Some(index),
            content: None,
            unwind: None,
        }
    }

    /// Fail the create at this position in the plan **after** its destination
    /// has been claimed exclusively and before its bytes are written — the one
    /// interval in which a leaf exists on disk and its content does not.
    ///
    /// A whole-effect failure cannot reach it, which is the point: the
    /// registration of the undo sits inside that interval, so it is a mechanism
    /// with no control in front of it otherwise.
    #[cfg(test)]
    pub(crate) const fn at_content(index: usize) -> Self {
        Self {
            effect: None,
            content: Some(index),
            unwind: None,
        }
    }

    /// Fail the effect at this position in the plan, and then the unwind step at
    /// this position in the rollback — the rollback runs backwards, so step 0 is
    /// the undo of the *last* effect that landed.
    #[cfg(test)]
    pub(crate) const fn at_effect_and_unwind(effect: usize, unwind: usize) -> Self {
        Self {
            effect: Some(effect),
            content: None,
            unwind: Some(unwind),
        }
    }

    fn strike_effect(self, index: usize, path: &Path) -> Result<(), Failure> {
        Self::strike(self.effect, index, path, "applying an effect to")
    }

    fn strike_content(self, index: usize, path: &Path) -> Result<(), Failure> {
        Self::strike(self.content, index, path, "writing the leaf's content to")
    }

    fn strike_unwind(self, index: usize, path: &Path) -> Result<(), Failure> {
        Self::strike(self.unwind, index, path, "unwinding an effect at")
    }

    fn strike(
        at: Option<usize>,
        index: usize,
        path: &Path,
        doing: &'static str,
    ) -> Result<(), Failure> {
        if at != Some(index) {
            return Ok(());
        }
        Err(Failure {
            path: path.to_path_buf(),
            doing,
            source: io::Error::other("a failure injected by this crate's own tests"),
        })
    }
}

#[cfg(test)]
mod tests;

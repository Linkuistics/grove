//! Resolve a Jujutsu workspace, refuse a working tree that is not one, and take
//! a path-scoped commit.
//!
//! That sentence is the whole crate. There is no repository abstraction here
//! and no second lane behind it: **jj is the version control system**, and a
//! working tree with no `.jj/` at or above it is refused with the command that
//! fixes it, before anything is created or changed. A `.git` beside a `.jj` is a
//! colocated repository and is jj's business — nothing here reads it, spawns
//! `git`, or branches on its presence.
//!
//! # It knows nothing about its consumer
//!
//! Every name in this crate is one jj already uses. Where a guarantee cannot be
//! stated without naming *whose* files it is about, the consumer supplies the
//! name: [`Workspace::control_dir`] takes a namespace rather than handing back a
//! grove-shaped directory, and rather than handing back jj's administrative
//! directory raw — which would put a consumer's generic filenames straight into
//! a namespace jj owns and may extend. Naming the consumer is what makes the
//! postcondition sayable in this crate's own vocabulary: *this directory is
//! yours, it is inside the workspace, and nothing tracks it.*
//!
//! # It takes commits; it does not implement transactions
//!
//! There is no witness, no manifest, no rollback proof, no index image, no
//! quarantine and no recovery path, because jj already owns all of them: it
//! snapshots the working copy before every command, and its operation log *is*
//! the transaction record. So [`Workspace::commit`] is one path-scoped
//! `jj commit`, and the only thing this crate adds to it is the refusal it
//! returns when that command does not complete — which names jj's own repair
//! and runs none of it.
//!
//! # Reads add no history
//!
//! [`Workspace::resolve`] is a filesystem walk, and the probes that must ask jj
//! about something the working copy cannot change — which workspace holds the
//! repository, what a commit's change id is — pass `--ignore-working-copy` so
//! they cannot record an operation at all.
//!
//! [`Workspace::is_tracked`] deliberately does **not**, because its answer does
//! depend on the working copy. jj's model is that the working copy is always
//! snapshotted, so a probe that skipped the snapshot would answer about a state
//! the tree has already left. Measured (jj 0.44.0) rather than assumed: a
//! snapshotting probe records an operation only when the working copy has
//! actually changed, and that is the same snapshot jj would take at the next
//! command for any reason — taken earlier, not taken twice.

mod jj;
mod refusal;

pub use refusal::Refusal;

use std::fs;
use std::path::{Path, PathBuf};

/// Names inside `.jj/` that belong to Jujutsu, and so cannot be handed to a
/// consumer as a control namespace. Refusing them is cheap and one-directional:
/// the cost of a name jj adds later and this list has not heard of is a
/// collision, and the cost of a name listed here that jj drops is a consumer
/// picking a different word.
const JJ_OWNED_NAMES: [&str; 2] = ["repo", "working_copy"];

/// A commit this crate took.
///
/// The change id rather than the commit id: a change id survives the rewrites —
/// `jj describe`, `jj squash`, a rebase — that give a commit a new commit id,
/// so it is the identity that still names this work afterwards.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Commit {
    pub change_id: String,
}

/// One resolved jj workspace.
///
/// Holding one is the proof that the precondition passed: it cannot be
/// constructed for a working tree that is not jj-enabled, so an operation
/// reached through it never has to re-ask.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Workspace {
    root: PathBuf,
    main_repo: PathBuf,
}

impl Workspace {
    /// Resolve the workspace at or above `path` — the closest ancestor holding
    /// a `.jj/` directory, whether it is a native, secondary or colocated
    /// checkout.
    ///
    /// **This is the precondition gate, not a dispatch.** There is one lane, so
    /// absence is never a case to handle; it is a refusal that names the command
    /// that fixes it, returned before anything has been read or changed.
    ///
    /// The walk is the filesystem's, not jj's: no repository discovery is
    /// invoked, so repository-selection variables in the environment and a
    /// shared repository store cannot redirect the answer. The root is
    /// canonical, so symlink and relative-path aliases of one workspace resolve
    /// to one [`Workspace`] rather than to several that disagree.
    pub fn resolve(path: &Path) -> Result<Self, Refusal> {
        let candidate = path
            .ancestors()
            .find(|dir| dir.join(".jj").is_dir())
            .ok_or_else(|| Refusal::not_a_workspace(path))?;
        let root = canonical(candidate)?;
        let main_repo = main_repo_of(&root)?;
        Ok(Self { root, main_repo })
    }

    /// The workspace root — the directory holding `.jj/`, already found by the
    /// walk. No jj binary is spawned to answer it.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// The root of the **default** workspace: the one that holds the
    /// repository, which every other workspace borrows. A native or colocated
    /// checkout is its own.
    pub fn main_repo(&self) -> &Path {
        &self.main_repo
    }

    /// A directory this workspace reserves for `namespace`'s own untracked
    /// coordination files: inside the workspace, never tracked, never shared
    /// with another namespace, and created if absent.
    ///
    /// The consumer's filenames are its own and cannot collide with jj's, which
    /// is the whole of what the namespace buys. It lives in the workspace's
    /// administrative directory rather than in the tracked working copy because
    /// jj snapshots the working copy on the next command, which would make a
    /// coordination file an artifact of the work it coordinates.
    ///
    /// A namespace is one plain directory name. Anything that is a path, or a
    /// name jj already uses inside `.jj/`, is refused rather than quietly
    /// reinterpreted — the guarantee is *never shared*, and a namespace that
    /// escapes or collides cannot keep it.
    ///
    /// Creation is all this promises. It does not write a probe file to prove
    /// the directory writable: the first coordination file the consumer opens
    /// there proves it, at the moment the answer matters, and a probe would be
    /// a second answer that can already be stale.
    pub fn control_dir(&self, namespace: &str) -> Result<PathBuf, Refusal> {
        let namespace = validated_namespace(namespace)?;
        let path = self.root.join(".jj").join(namespace);
        fs::create_dir_all(&path).map_err(|cause| Refusal::control_dir(&path, cause))?;
        Ok(path)
    }

    /// Does this workspace **track** `path`?
    ///
    /// About the tree as it is on disk *now*, which is why this is the one
    /// probe here that lets jj snapshot first. A caller reads this answer to
    /// decide something about the file in front of it — whether removing it
    /// could be undone, whether a repository could have shipped it — and an
    /// answer about a state the tree has already left would be wrong for both.
    /// The snapshot is not extra history: jj takes it at the next command
    /// whatever that command is, and takes none at all when nothing changed.
    ///
    /// A directory answers for everything beneath it: `true` when anything
    /// under it is held.
    ///
    /// `path` may be absolute or relative to [`root`](Self::root); a path
    /// outside this workspace is refused, because a workspace answers only for
    /// its own files.
    pub fn is_tracked(&self, path: &Path) -> Result<bool, Refusal> {
        let fileset = self.fileset(path)?;
        jj::produced_output(&self.root, &["file", "list", &fileset])
    }

    /// Take a commit scoped to `paths` and seal the working copy.
    ///
    /// Path-scoped, so unrelated working-copy changes stay in the working copy:
    /// jj snapshots everything and then commits only the fileset named here.
    /// An empty `paths` commits nothing and is refused rather than silently
    /// widened to the whole working copy — the scope is the point of the call.
    ///
    /// The refusal returned when the commit itself does not land is the only
    /// one that means *there is no commit*; it names jj's operation-log repair.
    /// A refusal from reading the new commit's identity afterwards means the
    /// commit landed and could not be named.
    pub fn commit(&self, paths: &[&Path], message: &str) -> Result<Commit, Refusal> {
        if paths.is_empty() {
            return Err(Refusal::not_scoped("no paths were named"));
        }
        let filesets = paths
            .iter()
            .map(|path| self.fileset(path))
            .collect::<Result<Vec<_>, _>>()?;

        let mut args = vec!["commit", "-m", message];
        args.extend(filesets.iter().map(String::as_str));
        jj::output(&self.root, &args)
            .map_err(|cause| Refusal::commit_not_recorded(&self.root, cause))?;

        // The commit just taken is the working copy's parent: `jj commit` leaves
        // a fresh empty working-copy commit on top of it. Read read-only, so
        // naming the commit does not add an operation of its own.
        let change_id = jj::output(
            &self.root,
            &[
                "log",
                "-r",
                "@-",
                "--no-graph",
                "--ignore-working-copy",
                "-T",
                "change_id",
            ],
        )?;
        Ok(Commit {
            change_id: change_id.trim().to_owned(),
        })
    }

    /// `path` as a jj fileset rooted at this workspace.
    ///
    /// `root:"…"` is what makes the scope the *workspace's*, rather than
    /// relative to whatever directory the command runs in — the two are the
    /// same here, and stating it keeps them the same if that ever stops being
    /// true.
    fn fileset(&self, path: &Path) -> Result<String, Refusal> {
        let relative = self.relative(path)?;
        let mut quoted = String::from("root:\"");
        for ch in relative.chars() {
            if ch == '"' || ch == '\\' {
                quoted.push('\\');
            }
            quoted.push(ch);
        }
        quoted.push('"');
        Ok(quoted)
    }

    /// `path` expressed relative to the workspace root, with `/` separators.
    ///
    /// Tried on the path as given first, and only then on a canonicalised
    /// version: the root is canonical, so a caller that reached the path
    /// through a symlinked ancestor would otherwise be told its own workspace
    /// does not contain it. Canonicalisation is applied to the **parent**, so a
    /// path that no longer exists — the one the caller is about to commit the
    /// deletion of — still resolves.
    fn relative(&self, path: &Path) -> Result<String, Refusal> {
        let absolute = if path.is_absolute() {
            path.to_path_buf()
        } else {
            self.root.join(path)
        };
        let relative = match absolute.strip_prefix(&self.root) {
            Ok(relative) => relative.to_path_buf(),
            Err(_) => {
                let parent = absolute
                    .parent()
                    .ok_or_else(|| Refusal::outside_workspace(path, &self.root))?;
                let name = absolute
                    .file_name()
                    .ok_or_else(|| Refusal::outside_workspace(path, &self.root))?;
                canonical(parent)?
                    .strip_prefix(&self.root)
                    .map_err(|_| Refusal::outside_workspace(path, &self.root))?
                    .join(name)
            }
        };
        if relative.as_os_str().is_empty() {
            return Err(Refusal::not_scoped(
                "the workspace root is not a scope inside itself",
            ));
        }
        let mut rendered = String::new();
        for component in relative.components() {
            if !rendered.is_empty() {
                rendered.push('/');
            }
            rendered.push_str(&component.as_os_str().to_string_lossy());
        }
        Ok(rendered)
    }
}

/// The default workspace's root, from a workspace root that is already
/// canonical.
///
/// **Answered from the filesystem wherever it can be.** jj's own on-disk shape
/// distinguishes the two cases: a workspace that *borrows* another's repository
/// has `.jj/repo` as a pointer **file**, and one that holds its own has the
/// repository there instead — the same file-versus-directory shape Git uses for
/// `.git`. So only a borrowed repository has to be followed, which is the
/// uncommon case, and jj is asked to follow it rather than the pointer being
/// parsed here.
///
/// The test is *is there a pointer*, not *is the repository intact*. Resolution
/// is the precondition gate for **being a workspace**, and a `.jj/` whose
/// contents are damaged is a different failure with a different remedy — one jj
/// will state, loudly, at the first command that needs the repository.
/// Diagnosing it here would mean every resolution paid a subprocess to find out
/// something it was not asked.
///
/// `--ignore-working-copy` keeps the borrowed-repository probe read-only:
/// without it every jj command snapshots the working copy, a mutation no
/// resolution step should perform, and one that would fail outright in a stale
/// workspace.
fn main_repo_of(root: &Path) -> Result<PathBuf, Refusal> {
    if !root.join(".jj").join("repo").is_file() {
        return Ok(root.to_path_buf());
    }
    let printed = jj::output(
        root,
        &[
            "workspace",
            "root",
            "--name",
            "default",
            "--ignore-working-copy",
        ],
    )?;
    canonical(Path::new(printed.trim()))
}

fn canonical(path: &Path) -> Result<PathBuf, Refusal> {
    path.canonicalize()
        .map_err(|cause| Refusal::unresolvable_path(path, cause))
}

fn validated_namespace(namespace: &str) -> Result<&str, Refusal> {
    if namespace.is_empty() {
        return Err(Refusal::namespace(namespace, "it is empty"));
    }
    if namespace.contains('/') || namespace.contains('\\') || namespace.contains('\0') {
        return Err(Refusal::namespace(
            namespace,
            "it is a path rather than one directory name",
        ));
    }
    if namespace == "." || namespace == ".." {
        return Err(Refusal::namespace(
            namespace,
            "it names a directory other than itself",
        ));
    }
    if JJ_OWNED_NAMES.contains(&namespace) {
        return Err(Refusal::namespace(
            namespace,
            "Jujutsu owns that name inside `.jj`",
        ));
    }
    Ok(namespace)
}

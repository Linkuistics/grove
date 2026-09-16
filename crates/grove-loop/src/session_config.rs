//! Grove's side of the template configuration: **which** files take part, and
//! whether the second one is admissible.
//!
//! Everything a template *is* — the KDL grammar, the slot rules, whole-word
//! expansion, aggregate diagnostics with source locations, and the rule that a
//! key resolves only if the primary file declares it — belongs to
//! [`keyed_launch`], which knows nothing about grove. What is left here is the
//! part that is grove's alone: the personal file's path, the two roots the
//! [configuration delta](`DELTA_FILE_NAME`) is searched at, the refusal of a
//! **tracked** delta, and the four slots grove's templates are written against.
//!
//! The trackedness refusal in particular could not move. It is a question about
//! grove's worktree, answered through grove's version control seam, and it is
//! the boundary between an untrusted repository and arbitrary code execution
//! (`docs/adr/untracked-configuration-delta.md`). A runner that took it would be
//! taking a security decision it has no standing to make.

use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use jj_workspace::Workspace;
use keyed_launch::{
    Argv, Catalog, ConfigError, Diagnostic, Inspection, Requirement, Selection, Slot, SlotRule,
    Source, SourceRole, Templates, Vocabulary,
};

const CONFIG_PATH: &str = ".config/grove/config.kdl";
/// The configuration delta's fixed name, searched at the two roots
/// [`DeltaRoots`] carries (`docs/adr/untracked-configuration-delta.md`).
pub const DELTA_FILE_NAME: &str = ".grove.kdl";

/// The four slots grove's command templates are written against, and the whole
/// of what grove tells the runner about its own domain.
///
/// `${prompt}` is required because a launch that does not carry the prompt
/// launches a session with no mandate; the other three are conveniences a
/// template may take or leave. There is no fifth, and adding one is a change to
/// this list and to `docs/CONFIGURATION.md` together.
const SLOTS: [SlotRule<'static>; 4] = [
    SlotRule {
        name: "prompt",
        requirement: Requirement::ExactlyOnce,
    },
    SlotRule {
        name: "session_name",
        requirement: Requirement::AtMostOnce,
    },
    SlotRule {
        name: "worktree",
        requirement: Requirement::AtMostOnce,
    },
    SlotRule {
        name: "repo",
        requirement: Requirement::AtMostOnce,
    },
];

/// Grove's slot vocabulary, supplied at load so every template rule is checked
/// before anything is spawned (`docs/specs/module-decomposition.md`, decision 7).
#[must_use]
pub fn vocabulary() -> Vocabulary<'static> {
    Vocabulary { slots: &SLOTS }
}

pub struct ExpansionContext<'a> {
    pub prompt: &'a str,
    pub session_name: &'a str,
    pub worktree: &'a Path,
    pub repository: &'a Path,
}

/// The two roots the [configuration delta](`DELTA_FILE_NAME`) is searched at,
/// **in that order** — the same two `${worktree}` and `${repo}` expand to.
///
/// They are *taken*, never re-derived here. A second notion of "the repository
/// root" computed inside this module is exactly the drift that would let the
/// search order disagree with what `${repo}` expands to in the very template it
/// selected; the VCS seam's `main_repo` is the one derivation, and its result
/// arrives through this struct. Naming both fields also makes a caller-side swap
/// of two same-typed paths impossible.
pub struct DeltaRoots<'a> {
    pub worktree: &'a Path,
    pub repository: &'a Path,
}

/// **Where each iteration's launch templates are read from** — the third
/// argument [`crate::run`] takes.
///
/// It is a *source* rather than a snapshot, and that is deliberate. The loop
/// reads the configuration twice per iteration: once before the tree is
/// mutated, so the just-in-time presence rule is asked against the document
/// that was live then, and once after the leaf is selected, so a session that
/// added a kind to `config.kdl` is launched from the document as it stands
/// (`docs/adr/complete-session-configuration.md`). A loaded [`SessionConfig`]
/// handed in once could express neither.
///
/// The delta roots are not held here: they come from the workspace `run` is
/// given, so there is one derivation of them and nothing to keep in step.
pub struct TemplateSource {
    home: PathBuf,
}

impl TemplateSource {
    /// The personal configuration under `home`.
    #[must_use]
    pub fn under(home: impl Into<PathBuf>) -> Self {
        Self { home: home.into() }
    }

    /// The personal configuration under `$HOME`.
    ///
    /// # Errors
    ///
    /// `$HOME` unset, which leaves nothing to locate the file from.
    pub fn from_env() -> Result<Self, crate::Error> {
        let home = std::env::var_os("HOME").map(PathBuf::from).ok_or_else(|| {
            crate::Error::from(source_error(
                "source_read",
                None,
                "$HOME is not set; cannot locate ~/.config/grove/config.kdl. Set it, or run \
                 `grove` from a login shell.",
                "Set HOME to the directory containing .config/grove/config.kdl.",
            ))
        })?;
        Ok(Self { home })
    }

    /// The personal file's path — the one a diagnostic names when nothing
    /// overrode it.
    #[must_use]
    pub fn personal_path(&self) -> PathBuf {
        SessionConfig::path(&self.home)
    }

    /// Read the personal file, and at most one delta laid over it.
    pub(crate) fn load(&self, roots: &DeltaRoots<'_>) -> Result<SessionConfig> {
        SessionConfig::read(&self.home, roots)
    }
}

pub struct SessionConfig {
    templates: Templates,
}

impl SessionConfig {
    pub fn path(home: &Path) -> PathBuf {
        home.join(CONFIG_PATH)
    }

    /// Where the delta is looked for, in search order: the worktree root, then
    /// the main repository root. The two coincide in a single-worktree
    /// repository, which is harmless — the first candidate found wins outright.
    pub fn delta_candidates(roots: &DeltaRoots<'_>) -> [PathBuf; 2] {
        [
            roots.worktree.join(DELTA_FILE_NAME),
            roots.repository.join(DELTA_FILE_NAME),
        ]
    }

    /// The personal file, then at most one delta laid over it per kind.
    ///
    /// The local selection replaces the personal default, including an empty
    /// list. Without either declaration, resolve the base with no profiles.
    /// Both sources are structurally validated, then the active composition is
    /// validated globally. An unreadable, unparseable, invalid
    /// or **tracked** delta fails the load rather than falling back to the very
    /// policy its owner was moving work away from.
    ///
    /// # Errors
    ///
    /// A personal file that is missing, unparseable or invalid; a delta that is
    /// any of those or **tracked**; or a candidate whose state could not be
    /// established at all.
    pub fn load(home: &Path, roots: &DeltaRoots<'_>) -> Result<Self, crate::Error> {
        Ok(Self::read(home, roots)?)
    }

    fn read(home: &Path, roots: &DeltaRoots<'_>) -> Result<Self> {
        let path = Self::path(home);
        let delta = find_delta(roots)?;
        if let Some(delta) = &delta {
            refuse_a_tracked_delta(delta)?;
        }
        let catalog =
            Catalog::load(&path, delta.as_deref(), vocabulary()).map_err(configuration_error)?;
        let empty = Selection::default();
        let selection = catalog
            .overlay_selection()
            .or_else(|| catalog.primary_selection())
            .unwrap_or(&empty);
        let templates = catalog.resolve(selection).map_err(configuration_error)?;
        Ok(SessionConfig { templates })
    }

    /// Load from the worktree a verb is running in, resolving `$HOME` and the
    /// two delta roots the same way the loop driver does.
    ///
    /// The seam's `main_repo` is the one derivation of *the repository root*, so
    /// a verb and the driver search the delta in the same order.
    ///
    /// # Errors
    ///
    /// `$HOME` unset, a working tree that is not a jj workspace, or any refusal
    /// [`Self::load`] makes.
    pub fn load_for_worktree(worktree: &Path) -> Result<Self, crate::Error> {
        let source = TemplateSource::from_env()?;
        let workspace = Workspace::resolve(worktree).map_err(|error| {
            source_error(
                "source_admission",
                Some(worktree.join(DELTA_FILE_NAME)),
                error.to_string(),
                "Run from a jj workspace so Grove can discover and admit its local configuration.",
            )
        })?;
        Ok(source.load(&DeltaRoots {
            worktree,
            repository: workspace.main_repo(),
        })?)
    }

    /// The file the resolved template for `kind` was read from — the personal
    /// file, or the delta that overrode it.
    pub fn source(&self, kind: &str) -> Option<&Path> {
        self.templates.source(kind)
    }

    /// The captured selection, sources, compiled commands and their provenance.
    /// Inspection and expansion share one validated snapshot and perform no I/O.
    #[must_use]
    pub fn inspect(&self) -> &Inspection {
        self.templates.inspect()
    }

    /// Does this kind resolve to exactly one complete template?
    ///
    /// The just-in-time half of `docs/adr/complete-session-configuration.md`:
    /// asked before grove writes a leaf of this kind, and again — through
    /// [`Self::expand`] — before it launches one.
    ///
    /// # Errors
    ///
    /// A kind no template resolves for, or one whose template is incomplete.
    pub fn require(&self, kind: &str) -> Result<(), crate::Error> {
        self.templates.require(kind).map_err(configuration_error)?;
        Ok(())
    }

    /// The launch this kind names, as the runner's own `Argv`.
    ///
    /// Handed on rather than flattened to a word list: `Argv` has no
    /// constructor, so passing it through to `keyed_launch::run` is what makes
    /// *nothing reaches a spawn that a template did not author* a fact about
    /// the types rather than a convention grove keeps.
    ///
    /// # Errors
    ///
    /// As [`Self::require`], plus a slot the template spells that grove's
    /// vocabulary does not supply.
    pub fn expand(&self, kind: &str, context: &ExpansionContext<'_>) -> Result<Argv, crate::Error> {
        let argv = self
            .templates
            .expand(
                kind,
                &[
                    Slot {
                        name: "prompt",
                        value: context.prompt.as_ref(),
                    },
                    Slot {
                        name: "session_name",
                        value: context.session_name.as_ref(),
                    },
                    Slot {
                        name: "worktree",
                        value: context.worktree.as_os_str(),
                    },
                    Slot {
                        name: "repo",
                        value: context.repository.as_os_str(),
                    },
                ],
            )
            .map_err(configuration_error)?;
        Ok(argv)
    }
}

/// The first of the two searched paths that **holds anything at all**; the other
/// is not read, and the two are never merged with each other.
///
/// `symlink_metadata` rather than `is_file`, so a broken symlink or a directory
/// at the searched path is a candidate that then fails closed on read, not an
/// absence that silently resolves to the personal file.
///
/// **Only `NotFound` is absence** (`docs/adr/untracked-configuration-delta.md`).
/// Any other error means this candidate's state could not be established, and
/// the two things a caller would otherwise do with it are both wrong: at the
/// worktree root it would move on and read the repository root, inverting the
/// search precedence that record fixes, and at the repository root it would fall
/// through to the very personal file the delta exists to move work away from. An
/// unresolvable candidate is therefore the same refusal an unreadable delta
/// already is, reported against the path whose state is unknown.
fn find_delta(roots: &DeltaRoots<'_>) -> Result<Option<PathBuf>> {
    for candidate in SessionConfig::delta_candidates(roots) {
        match fs::symlink_metadata(&candidate) {
            Ok(_) => return Ok(Some(candidate)),
            Err(error) if error.kind() == ErrorKind::NotFound => continue,
            Err(error) => {
                return Err(source_error(
                    "source_read",
                    Some(candidate.clone()),
                    format!(
                        "failed to determine whether a Grove configuration delta is present at {}: {error}",
                        candidate.display()
                    ),
                    "Make the candidate path accessible; only an absent candidate permits repository fallback.",
                ));
            }
        }
    }
    Ok(None)
}

/// Refuse the selected delta if it is **tracked**, before anything reads it.
///
/// Trackedness is validated on the delta the search already selected, never used
/// to select it: a tracked file at the worktree root is a refusal, not a reason
/// to read the repository root. The probe runs only because a candidate file
/// exists, so a checkout with no delta pays nothing for it, and a probe that
/// cannot be completed fails closed like any other unresolved validation.
///
/// The remedy names the ignore line *first* because jj enforces that order:
/// `jj file untrack` refuses a path that is not already ignored, so the
/// otherwise natural untrack-then-ignore sequence fails on its first step
/// (`jj file untrack --help`, jj 0.44.0 — "Paths to untrack. They must already
/// be ignored.").
fn refuse_a_tracked_delta(path: &Path) -> Result<()> {
    let tracked = delta_is_tracked(path).map_err(|error| {
        source_error(
            "source_admission",
            Some(path.to_owned()),
            format!(
                "checking whether the Grove configuration delta at {} is tracked: {error:#}",
                path.display()
            ),
            "Repair the jj workspace or executable so trackedness can be checked; no fallback is used.",
        )
    })?;
    if tracked {
        return Err(source_error(
            "source_admission",
            Some(path.to_owned()),
            format!(
            "refusing the Grove configuration delta at {path}: it is tracked in version control, \
             and a tracked delta lets a repository choose what Grove executes in every checkout \
             of it.\n  Untrack it (`jj file untrack {name}`, after adding `/{name}` to \
             `.gitignore` — jj refuses to untrack a file it would immediately re-add).",
            path = path.display(),
            name = DELTA_FILE_NAME
            ),
            "Add /.grove.kdl to .gitignore, then run `jj file untrack .grove.kdl` in its workspace.",
        ));
    }
    Ok(())
}

/// Is the delta at `path` **tracked** by the workspace it sits in?
///
/// The one read-only question grove asks the version control system outside the
/// finish path, and the enforcement behind the untracked configuration delta
/// (`docs/adr/untracked-configuration-delta.md`): a delta names a program to
/// execute, so a repository that could ship one would choose what Grove spawns
/// in any checkout of it. Documentation cannot establish that boundary and
/// neither can an ignore rule — a file already committed stays tracked when an
/// ignore line is added.
///
/// Anchored to the candidate's **own** directory rather than to the leased
/// worktree, because the two searched roots may live in different workspaces (a
/// secondary jj workspace) and the one that owns the file is the one whose
/// working-copy commit can hold it.
///
/// **No workspace at all answers `false` rather than refusing.** This is the one
/// place absence is an answer rather than a precondition failure: nothing owns
/// the file, so nothing tracks it, and the hostile repository this guards
/// against has a marker by definition. That is why the refusal is discarded
/// instead of propagated — resolution declines for exactly one reason a caller
/// can act on, *this is not a Jujutsu working tree*, and here that reason is the
/// answer. A probe that cannot be *completed* — the binary missing, the command
/// failing — is still an error, and its caller fails closed.
fn delta_is_tracked(path: &Path) -> Result<bool> {
    let directory = path
        .parent()
        .with_context(|| format!("candidate path has no parent directory: {}", path.display()))?;
    let Ok(workspace) = Workspace::resolve(directory) else {
        return Ok(false);
    };
    Ok(workspace.is_tracked(path)?)
}

/// Generic diagnostic messages need not repeat their structured source paths.
/// Keep the records intact and add those paths to Grove's human error chain.
fn configuration_error(error: ConfigError) -> anyhow::Error {
    let mut paths = Vec::new();
    for diagnostic in error.diagnostics() {
        if let Some(source) = &diagnostic.source {
            if !paths.contains(&source.path) {
                paths.push(source.path.clone());
            }
        }
    }
    let sources = paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    if sources.is_empty() {
        error.into()
    } else {
        anyhow::Error::new(error).context(format!("Grove configuration from {sources}"))
    }
}

/// Discovery has no captured text, so retain its path without inventing a span.
#[derive(Debug)]
pub(crate) struct SourceError(pub Diagnostic);

impl std::fmt::Display for SourceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}\n  {}", self.0.message, self.0.remedy)
    }
}

impl std::error::Error for SourceError {}

fn source_error(
    category: &str,
    path: Option<PathBuf>,
    message: impl Into<String>,
    remedy: &str,
) -> anyhow::Error {
    SourceError(Diagnostic {
        category: category.to_owned(),
        message: message.into(),
        source: path.map(|path| Source {
            role: SourceRole::Overlay,
            path,
        }),
        primary: None,
        related: Vec::new(),
        occurrence_chain: Vec::new(),
        key: None,
        binding: None,
        command: None,
        parameter: None,
        remedy: remedy.to_owned(),
    })
    .into()
}

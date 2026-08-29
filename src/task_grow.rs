// Grove's **grow verbs** — `leaf-add`, `leaf-add-pair` and `leaf-insert` —
// expressed through `ordinal-fs-tree` (gh issue #13, increment 2, the *migrate*
// stage's third leaf). `append`, `append_many` and `insert` are the operations;
// what is left here is grove's own: the reference grammar, the preconditions the
// library cannot see, the task-file template, and the cross-reference lint.
//
// # What grove still owns, and why each piece could not move
//
// * **The `<parent>` / `<target>` reference.** Grove's grammar is a path, `[n]`,
//   `n`, `<slug>-k<key>` or a bare slug, which is wider than a key and has an
//   *ambiguous* outcome the library has no counterpart for. Resolution happens
//   against the guard's own snapshot — the one the operation then plans from —
//   which is clause 1 of `docs/ARCHITECTURE.md#library-refusals`.
// * **The preconditions.** A session kind, `finish`-reservation and slug
//   validity are grove's alone; so is *this parent is a node*, which grove
//   checks with the library's own predicate off the same snapshot (contents are
//   `Some`) rather than a second one (clause 2).
// * **The template.** The library has no content model, so
//   [`task_template_body`] is the bytes handed to `NewEntry::new` — and because
//   those bytes embed the key, grove predicts the allocation and checks it
//   ([`allocated`]).
// * **The cross-reference lint.** A renumber leaves position-prefixed references
//   stale, and nothing in the library knows what a reference is.
//
// # What went, and stayed gone
//
// # Three helpers here are `pub(crate)`, and `leaf-decompose` is why
//
// `new_leaf`, `leaf_slug` and `refuse_finish_kind` are shared with
// `tree_lifecycle`'s `leaf_decompose`, whose `promote` optionally creates a first
// child in the same unit — so it composes a new leaf exactly as the grow verbs
// do, key prediction and all. One constructor rather than two is the point: a
// second spelling of *what a new grove leaf is* would let the two drift on the
// template, the slug grammar or the `finish` reservation.
//
// The whole of the path-walking appender's collision machinery — the up-front
// destination sweep, the `O_EXCL` claim, the per-run rollback, the injected
// post-claim failure — because `append_many` *is* the atomic run: one snapshot answers
// every ordinal and every key, the plan is checked against itself before a byte
// is written, and the interpreter unwinds its own effects. `leaf-add-pair` got
// simpler, exactly as its brief predicted.

use std::io::Write;
use std::path::{Path, PathBuf};

use anyhow::{bail, Context, Result};
use ordinal_fs_tree::{Entry, Key, NewEntry, Report, Snapshot, Species, Target};

use crate::leaf::Kind;
use crate::task_name::{Outcome, Parts, Slug, TaskName};
use crate::task_tree::{self, TreeWrite};

/// `leaf-add <parent> <slug>`: append a child leaf under `parent` at the next
/// free ordinal with a fresh key. `parent` is `.` for the grove root, or a node
/// by key, handle, slug or path. Returns the new leaf's absolute path.
/// Working-tree only — no commit.
///
/// The kind is part of the filename; the body is the bare template — the stable
/// header and empty task sections — which the creating session then fills in.
///
/// **This verb is how a review chain is built** (flat-lazy-review). A producer's
/// last act is `leaf-add <parent> <stem> --kind review-<producer>` when review is
/// required, and the review's last act is the matching `integrate-review-…` leaf
/// when it has findings worth acting on — every step slugged with the same bare
/// stem, since the kind states its role. The steps are flat siblings, so nothing
/// here knows they compose one artifact.
pub fn leaf_add(grove_root: &Path, parent: &str, slug: &str, kind: Kind) -> Result<PathBuf> {
    refuse_finish_kind(kind, "leaf-add")?;
    let slug = leaf_slug(slug)?;
    let tree = task_tree::write(grove_root)?;
    let (target, key) = {
        let target = parent_node(&tree, parent)?;
        (target, task_tree::next_key(tree.snapshot()))
    };
    let entry = new_leaf(key, Outcome::Live, kind, &slug);
    let report = tree.append(target, entry).map_err(task_tree::raised)?;
    Ok(allocated(&report, &[key])?.remove(0))
}

/// `leaf-add-pair <parent> <stem>`: append a whole **research pair** — three flat
/// siblings all slugged with the bare `stem`, at consecutive ordinals with
/// consecutive keys — under `parent`, as one unit. Returns the three paths in
/// ordinal order.
///
/// The steps have fixed filename kinds `research-a`, `research-b` and
/// `combine-research`, and **the kind is the only thing that distinguishes
/// them**. It is also the only thing that needs to: the kind field is the
/// canonical statement of a leaf's role, so the slug names the artifact and does
/// not restate it.
///
/// **The pair stays eager while the review chain went lazy** (flat-lazy-review).
/// Lazy creation is actively *wrong* here: a `research-b` cut by `research-a`'s
/// own session would inherit that session's framing and corpus, and the
/// independence of the two corpora is the entire reason a pair is run.
///
/// # This is not `leaf_add` three times, and `append_many` is why
///
/// Three calls would be three snapshots, three guards and three chances to stop
/// half way — and a live prefix of a pair looks exactly like a deliberately
/// hand-cut partial one. `append_many` plans the whole run from **one** snapshot,
/// so the ordinals are contiguous and the keys consecutive by construction, and
/// applies it as a unit, so either the whole pair lands or none of it does. The
/// verb no longer sweeps destinations, claims files or rolls back: all three were
/// grove's own reconstruction of what the library's interpreter already does.
pub fn leaf_add_pair(grove_root: &Path, parent: &str, stem: &str) -> Result<Vec<PathBuf>> {
    // The stem *is* the slug, three times over. Validated at this verb's own
    // boundary rather than by borrowing the loop below: a stem is a slug, and
    // this verb takes a stem.
    let stem = leaf_slug(stem)?;
    let tree = task_tree::write(grove_root)?;
    let (target, keys) = {
        let target = parent_node(&tree, parent)?;
        // One prediction per step, walking the same `max + 1` the library walks
        // across a run. A run whose first key would already overflow predicts
        // nothing at all, and the library refuses it.
        let first = task_tree::next_key(tree.snapshot());
        let keys: Vec<Option<Key>> = (0..PAIR.len() as u32)
            .map(|step| {
                first
                    .and_then(|key| key.get().checked_add(step))
                    .map(Key::new)
            })
            .collect();
        (target, keys)
    };
    let entries = PAIR
        .iter()
        .zip(keys.iter())
        .map(|(kind, key)| new_leaf(*key, Outcome::Live, *kind, &stem))
        .collect();
    let report = tree
        .append_many(target, entries)
        .map_err(task_tree::raised)?;
    allocated(&report, &keys)
}

/// The pair's three steps, in the order they land. The kind is the whole of what
/// distinguishes them, so this is the whole of the shape.
pub(crate) const PAIR: [Kind; 3] = [Kind::ResearchA, Kind::ResearchB, Kind::CombineResearch];

/// What a `leaf-insert` did: where the new leaf landed, and every sibling whose
/// ordinal moved.
#[derive(Debug)]
pub struct Inserted {
    /// The new leaf's absolute path.
    pub path: PathBuf,
    /// The shifted siblings, ascending by new ordinal.
    pub renumbers: Vec<Renumber>,
}

/// `leaf-insert <target> <slug>`: insert a new leaf at the slot `target` holds,
/// shifting `target` and every later sibling in its level up by one. `target` is
/// an existing entry — leaf or node — by key, handle, slug or path. Inserting
/// past the last sibling is `leaf-add`'s job, so the target must exist.
/// Working-tree only — no commit.
///
/// # The shift rewrites zero file contents, and that is structural
///
/// Each shift is one rename of one entry whose new name is
/// `compose(new_ordinal, key, parts)` and nothing else, so it cannot disturb a
/// key, a slug or an outcome; a shifted **node** is one directory rename, with
/// its whole subtree — child names *and* keys — riding along untouched. In-file
/// `# <slug>-k<key>` headers are position-free, so nothing in any body refers to
/// the ordinal that moved. The renames run highest-ordinal-first, which keeps
/// ordinals distinct at every intermediate state rather than merely avoiding a
/// collision (`crates/ordinal-fs-tree/src/ops.rs`, *Why highest-first*).
///
/// # grove names an entry where the library names an ordinal
///
/// `insert` takes the ordinal directly, and `cli-k16` found that *good*: an
/// operator who guesses is told the level's occupied span by the refusal itself.
/// grove's operators never see it — grove reads the ordinal off the entry the
/// operator named, in the snapshot the insert plans from, so `at` is occupied by
/// construction and [`Refusal::NoOccupantAtOrdinal`](ordinal_fs_tree::Refusal)
/// is unreachable in all three of its messages. What grove owes in its place is a
/// refusal for the things a *reference* can be and an ordinal cannot: nothing at
/// all, two entries at once, the root, or the charter brief.
pub fn leaf_insert(grove_root: &Path, target: &str, slug: &str, kind: Kind) -> Result<Inserted> {
    refuse_finish_kind(kind, "leaf-insert")?;
    let slug = leaf_slug(slug)?;
    let tree = task_tree::write(grove_root)?;
    let root = tree.root().to_path_buf();
    let (level, at, key) = {
        let entry = match task_tree::reference(&root, tree.snapshot(), target)? {
            task_tree::Target::Root => bail!(
                "cannot insert at the grove root (leaf-insert takes the slot of an \
                 existing entry; use `leaf-add .` to append at the root): {}",
                root.display()
            ),
            task_tree::Target::Entry(entry) => entry,
        };
        let Some(triple) = entry.triple() else {
            bail!("cannot insert at the brief: {}", entry.name())
        };
        // The reference named *this* entry, so the slot has to be this entry's
        // and not a twin's. `addressable_key` is `marking-k32`'s finding applied
        // to an ordinal rather than to a key: a duplicated key makes *the entry
        // the operator named* ambiguous, whatever the verb then reads off it.
        task_tree::addressable_key(&root, tree.snapshot(), &entry)?;
        (
            containing_level(&root, tree.snapshot(), &entry)?,
            triple.ordinal,
            task_tree::next_key(tree.snapshot()),
        )
    };
    let entry = new_leaf(key, Outcome::Live, kind, &slug);
    let report = tree.insert(level, at, entry).map_err(task_tree::raised)?;
    Ok(Inserted {
        path: allocated(&report, &[key])?.remove(0),
        renumbers: renumbers(&report)?,
    })
}

/// One entry of a `leaf-insert` renumber: a sibling whose ordinal shifted up by
/// one. The key, the slug and the outcome (and, for a node, its whole subtree)
/// are invariant — only the `NN` in this one entry's own name changed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Renumber {
    pub old_position: u32,
    pub new_position: u32,
    pub old_name: String,
    pub new_name: String,
}

/// The renumber log, read off the library's own report rather than kept by the
/// caller as it renames.
///
/// [`Report::renamed`] is in the order the renames ran — highest ordinal first —
/// and every one of them is a shift, since an `insert` renames nothing else. The
/// log is reported ascending by new ordinal, which is how an operator reads a
/// level.
fn renumbers(report: &Report<TaskName>) -> Result<Vec<Renumber>> {
    let mut log = Vec::with_capacity(report.renamed().len());
    for renamed in report.renamed() {
        let TaskName::Positioned { ordinal, .. } = &renamed.name else {
            bail!("the library reported a shift of the charter brief, which has no ordinal")
        };
        let new_position = ordinal.get();
        log.push(Renumber {
            // A shift is `+1` by definition, so the old ordinal is derived
            // rather than remembered — the one fact `Renamed` does not carry.
            old_position: new_position
                .checked_sub(1)
                .context("the library reported a shift to ordinal 0")?,
            new_position,
            old_name: file_name(&renamed.from)?,
            new_name: file_name(&renamed.to)?,
        });
    }
    log.sort_by_key(|renumber| renumber.new_position);
    Ok(log)
}

/// Surface stray **position-prefixed** cross-references left stale by a
/// `leaf-insert` renumber, as a lint on stderr — never an auto-rewrite (durable
/// references should use the stable `<slug>-k<key>` handle, which a renumber
/// never changes, so the operator reviews each occurrence). Emits one
/// `path:line: <old-name> (context)` per hit. A stable `<slug>-k<key>` reference
/// is *not* surfaced (it did not move); only the position-prefixed form is stale.
/// An empty renumber log means there is nothing to do.
///
/// **It scans the tree, where it used to scan the directory.** The bodies are
/// every leaf and every `BRIEF.md` the snapshot holds, which is what grove has
/// under the guard and exactly the set the reader admits. A foreign `.md` file
/// dropped inside `.grove/` by hand is no longer scanned; grove writes none, and
/// the alternative is a second, wider notion of *what is in the tree* than every
/// other verb uses.
pub fn surface_cross_refs(
    grove_root: &Path,
    renumbers: &[Renumber],
    out: &mut impl Write,
) -> Result<()> {
    if renumbers.is_empty() {
        return Ok(());
    }
    // A second observation, and it has to be: a mutation consumes its guard, and
    // the paths inside a shifted node moved with it, so the tree this scans is
    // the one the shift *left*. Reopened exclusively — the lint's output is
    // written while it is held, so nothing renames underneath a hit it is about
    // to print — and without a second waiting diagnostic, because the wait this
    // command made was announced by the insert.
    let tree = task_tree::reopen_write(grove_root)?;
    // The stale tokens are the *old* position-prefixed names the renumber moved
    // (`02-mid-k3`), with any `.md` extension dropped so a path reference
    // `02-mid-k3/01-impl-x-k4.md` matches the directory token. The `-k<digits>`
    // tail makes these specific enough to scan as plain substrings.
    let stale: Vec<&str> = renumbers
        .iter()
        .map(|renumber| stem(&renumber.old_name))
        .collect();

    let mut bodies: Vec<PathBuf> = tree
        .walk()
        .filter(|entry| matches!(entry.species(), Species::Leaf | Species::Distinguished))
        .map(|entry| task_tree::entry_path(tree.root(), entry))
        .collect();
    bodies.sort();

    for path in &bodies {
        let Ok(body) = std::fs::read_to_string(path) else {
            continue;
        };
        for (index, line) in body.lines().enumerate() {
            for token in &stale {
                if line.contains(token) {
                    writeln!(
                        out,
                        "{}:{}: {} ({})",
                        path.display(),
                        index + 1,
                        token,
                        line.trim()
                    )
                    .ok();
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// the seam

/// The library target a `<parent>` argument names, refused here if it is not a
/// node.
///
/// **Clause 2, with the library's own predicate.** A node is an entry whose
/// `contents()` are `Some` — never a path that `is_dir` — because a second
/// predicate for one condition would let grove refuse where the library would
/// have proceeded. Grove keeps the check in front of
/// [`Refusal::TargetNotNode`](ordinal_fs_tree::Refusal) because it needs it
/// anyway: the charter brief is an entry with no key, so it cannot be handed to
/// the library as a target at all.
fn parent_node(tree: &TreeWrite, parent: &str) -> Result<Target> {
    let root = tree.root();
    match task_tree::reference(root, tree.snapshot(), parent)? {
        task_tree::Target::Root => Ok(Target::Root),
        task_tree::Target::Entry(entry) => {
            if entry.contents().is_none() {
                bail!(
                    "parent is not a node directory (need a directory named \
                     NN-<slug>-k<key>): {}",
                    task_tree::entry_path(root, entry).display()
                );
            }
            Ok(Target::Key(task_tree::addressable_key(
                root,
                tree.snapshot(),
                &entry,
            )?))
        }
    }
}

/// The level an entry sits in, as a library target: the root, or the node that
/// contains it.
///
/// `leaf-insert` names the entry whose slot the new leaf takes, and `insert`
/// takes the *level* the child goes into — so the target is the entry's
/// container, which is a node by construction. That is why
/// [`Refusal::TargetNotNode`](ordinal_fs_tree::Refusal) is unreachable from this
/// verb.
fn containing_level(
    root: &Path,
    snapshot: &Snapshot<TaskName>,
    entry: &Entry<'_, TaskName>,
) -> Result<Target> {
    match entry.container().entry() {
        None => Ok(Target::Root),
        Some(node) => Ok(Target::Key(task_tree::addressable_key(
            root, snapshot, &node,
        )?)),
    }
}

/// One leaf to create: its parts, and the template body carrying the handle the
/// key predicts.
///
/// A `None` key is an exhausted keyspace, which is
/// [`Refusal::KeysExhausted`](ordinal_fs_tree::Refusal) and the library's to
/// state. The entry carries no bytes then, and never needs any: a refusal writes
/// nothing.
pub(crate) fn new_leaf(
    key: Option<Key>,
    outcome: Outcome,
    kind: Kind,
    slug: &Slug,
) -> NewEntry<Parts> {
    let parts = Parts::leaf(outcome, kind, slug.clone());
    match key {
        Some(key) => NewEntry::new(
            parts,
            task_template_body(slug.as_str(), key.get()).into_bytes(),
        ),
        None => NewEntry::empty(parts),
    }
}

/// The paths of what an operation created, checked against the keys grove
/// predicted for them.
///
/// **The one place the prediction is held to account.** Grove renders the
/// handle inside each leaf's body from `task_tree::next_key`, so a key the
/// library allocated differently would leave a file whose first line contradicts
/// its own filename — silently, and forever. The check reads the key off the
/// name in the report, which is the library's own answer, and the operation has
/// already landed when it fires: a disagreement is a broken contract to report,
/// not a case to recover from, and the message says which file carries it.
pub(crate) fn allocated(
    report: &Report<TaskName>,
    predicted: &[Option<Key>],
) -> Result<Vec<PathBuf>> {
    let created = report.created();
    if created.len() != predicted.len() {
        bail!(
            "the library created {} entries where {} were asked for",
            created.len(),
            predicted.len()
        );
    }
    let mut paths = Vec::with_capacity(created.len());
    for (created, predicted) in created.iter().zip(predicted) {
        let TaskName::Positioned { key, .. } = &created.name else {
            bail!(
                "the library created a charter brief, which carries no key: {}",
                created.path.display()
            )
        };
        if Some(*key) != *predicted {
            bail!(
                "the library allocated key {} where grove's template wrote {}: the \
                 handle in {} contradicts its filename and must be corrected by hand",
                key.get(),
                predicted.map_or("no key".to_string(), |key| key.get().to_string()),
                created.path.display()
            );
        }
        paths.push(created.path.clone());
    }
    Ok(paths)
}

// ---------------------------------------------------------------------------
// grove's own preconditions

/// A new leaf's slug, refused here rather than by the grammar at render time.
///
/// [`Slug::new`] is the domain's own validator — the same one
/// [`TaskName::parse`](ordinal_fs_tree::EntryName::parse) reads a filename
/// through — so a slug this accepts is one the tree can carry and read back.
pub(crate) fn leaf_slug(slug: &str) -> Result<Slug> {
    Slug::new(slug).map_err(|error| anyhow::anyhow!("slug {slug:?}: {error}"))
}

/// The `finish` kind is the driver's own and no operator verb may create one.
pub(crate) fn refuse_finish_kind(kind: Kind, verb: &str) -> Result<()> {
    if kind == Kind::Finish {
        bail!("`finish` is driver-reserved and cannot be created by `{verb}`");
    }
    Ok(())
}

/// The bare task-file template: the position-free handle `# <slug>-k<key>` and
/// the empty sections the creating session fills in.
///
/// **One template, no parameters beyond the handle.** Session kind and harness
/// are launch-time configuration, and a step's relationship to its neighbours is
/// prose the *creating session* writes into the body afterwards
/// (`content/TASK-FORMAT.md`, the `**Reviews:**` / `**Integrates:**`
/// convention). That is the whole point of creating a review step late: the
/// session that cuts it knows the specific finding the step exists for, which no
/// constructor rendering a goal sentence from a handle could.
pub(crate) fn task_template_body(slug: &str, key: u32) -> String {
    format!("# {slug}-k{key}\n\n\n## Goal\n\n\n\n## Context\n\n## Done when\n\n## Notes\n")
}

/// The path's final component as an owned `String`.
fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .with_context(|| format!("path {} has no filename", path.display()))
}

/// Drop a trailing `.md` from a name so a directory-token reference matches; a
/// node directory's name has no extension and is returned as-is.
fn stem(name: &str) -> &str {
    name.strip_suffix(".md").unwrap_or(name)
}

#[cfg(test)]
mod tests;

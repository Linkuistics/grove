use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use grove_loop::{entry_path, Handle, Kind, Outcome, Parts, TreeObservation};

#[derive(PartialEq, Eq)]
pub(crate) struct Row {
    pub key: Item,
    pub path: PathBuf,
    pub handle: Option<Handle>,
    pub kind: Option<Kind>,
    pub lifecycle: Lifecycle,
    pub depth: usize,
    pub branch: bool,
    pub expanded: bool,
    pub counts: [usize; 3],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Lifecycle {
    Live,
    Done,
    Abandoned,
    Empty,
}

/// Root has its own identity, separate from permanent task keys.
pub(crate) type Item = Option<u32>;

pub(crate) use grove_loop::TreeLifetime as Root;

type Content = Result<Vec<u8>, String>;
type DisplayCapture = Observation<(Vec<Row>, usize, Content)>;
#[derive(PartialEq, Eq)]
pub(crate) enum Observation<T> {
    Ready(T),
    Vacant,
    Busy,
}

/// Two bounded captures detect visible non-cooperating edits without claiming
/// atomicity. Each drops its guard before another acquisition. Never loop until
/// stable here: busy/changing trees must leave input and quit responsive.
pub(crate) fn capture(worktree: &Path, candidates: &[Item]) -> Result<DisplayCapture> {
    let (first, first_root) = capture_once(worktree, candidates)?;
    if !matches!(first, Observation::Ready(_)) {
        return Ok(first);
    }
    let (second, second_root) = capture_once(worktree, candidates)?;
    // Contention on the verification read is still Busy, not malformed data.
    if !matches!(second, Observation::Ready(_)) {
        return Ok(second);
    }
    anyhow::ensure!(
        matches!((&first_root, &second_root), (Some(a), Some(b)) if a.same(b)?),
        "root changed during observation; retrying"
    );
    anyhow::ensure!(first == second, "tree changed during observation; retrying");
    Ok(second)
}

/// Build display rows from a loop capture whose tree guard is already released.
fn capture_once(worktree: &Path, candidates: &[Item]) -> Result<(DisplayCapture, Option<Root>)> {
    let tree = match grove_loop::try_observe(worktree, candidates)? {
        TreeObservation::Busy => return Ok((Observation::Busy, None)),
        TreeObservation::Vacant => return Ok((Observation::Vacant, None)),
        TreeObservation::Ready(tree) => tree,
    };
    let root = tree.snapshot().root();
    let brief = root.distinguished().context("root has no brief")?;
    let path = entry_path(tree.root(), brief);

    let mut rows = vec![Row {
        key: None,
        path,
        handle: None,
        kind: None,
        lifecycle: Lifecycle::Empty,
        depth: 0,
        branch: true,
        expanded: true,
        counts: [0; 3],
    }];
    for entry in tree.snapshot().walk() {
        let Some(triple) = entry.triple() else {
            continue;
        };
        let (path, handle, kind, counts, branch) = match triple.parts {
            Parts::Leaf { outcome, kind, .. } => {
                let handle = Handle::of_leaf(entry.name()).context("leaf has no handle")?;
                let index = match outcome {
                    Outcome::Live => 0,
                    Outcome::Done => 1,
                    Outcome::Abandoned => 2,
                };
                let mut counts = [0; 3];
                counts[index] = 1;
                (
                    entry_path(tree.root(), entry),
                    handle,
                    Some(kind.clone()),
                    counts,
                    false,
                )
            }
            Parts::Node => {
                let brief = entry
                    .contents()
                    .and_then(|level| level.distinguished())
                    .context("branch has no brief")?;
                let handle =
                    Handle::of_node(entry.name(), brief.name()).context("branch has no handle")?;
                (entry_path(tree.root(), brief), handle, None, [0; 3], true)
            }
        };
        rows.push(Row {
            key: Some(triple.key.get()),
            path,
            handle: Some(handle),
            kind,
            lifecycle: Lifecycle::Empty,
            depth: entry.depth(),
            branch,
            expanded: true,
            counts,
        });
    }
    // Reverse pre-order folds each subtree into its immediate parent, including
    // collapsed descendants and empty branches (which contribute no leaves).
    let mut parents = vec![0];
    let mut parent_of = vec![0; rows.len()];
    for i in 1..rows.len() {
        while parents.len() > rows[i].depth {
            parents.pop();
        }
        parent_of[i] = *parents.last().context("entry has no parent")?;
        if rows[i].branch {
            parents.push(i);
        }
    }
    for i in (1..rows.len()).rev() {
        let counts = rows[i].counts;
        for (n, count) in counts.into_iter().enumerate() {
            rows[parent_of[i]].counts[n] += count;
        }
    }
    for row in &mut rows {
        let [live, done, abandoned] = row.counts;
        row.lifecycle = if live > 0 {
            Lifecycle::Live
        } else if done > 0 {
            Lifecycle::Done
        } else if abandoned > 0 {
            Lifecycle::Abandoned
        } else {
            Lifecycle::Empty
        };
    }
    let selected = rows
        .iter()
        .position(|row| row.key == tree.selected)
        .unwrap_or(0);
    Ok((
        Observation::Ready((rows, selected, tree.content)),
        Some(tree.lifetime),
    ))
}

/// Never put control bytes from names, paths, diagnostics or file contents on
/// the terminal. Keep source newlines, render tabs as spaces and controls inert.
pub(crate) fn safe_text(text: &str) -> String {
    text.chars()
        .flat_map(|ch| match ch {
            '\n' => "\n".chars().collect::<Vec<_>>(),
            '\t' => "    ".chars().collect(),
            ch if ch.is_control() => vec!['�'],
            ch => vec![ch],
        })
        .collect()
}

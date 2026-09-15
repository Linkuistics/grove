use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use grove_loop::{entry_path, ActivityObservation, Handle, Kind, Outcome, Parts, TreeObservation};

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
type DisplayCapture = Observation<(Vec<Row>, usize, Content, Option<u32>)>;
#[derive(PartialEq, Eq)]
pub(crate) enum Observation<T> {
    Ready(T),
    Vacant,
    Busy,
}

/// Two bounded captures detect visible non-cooperating edits without claiming
/// atomicity. Each drops its guard before another acquisition. Never loop until
/// stable here: busy/changing trees must leave input and quit responsive.
pub(crate) fn capture(
    worktree: &Path,
    candidates: &[Item],
) -> (Result<DisplayCapture>, ActivityObservation) {
    capture_with(|| grove_loop::try_observe(worktree, candidates))
}

fn capture_with(
    mut observe: impl FnMut() -> grove_loop::ObservationGuard,
) -> (Result<DisplayCapture>, ActivityObservation) {
    let first_sample = observe();
    let second_sample = observe();
    let activity = if first_sample.activity == second_sample.activity {
        second_sample.activity
    } else {
        ActivityObservation::Busy("activity changed during observation; retrying".into())
    };
    let tree = (|| {
        let (first, first_root) = display_capture(first_sample.tree?)?;
        let (second, second_root) = display_capture(second_sample.tree?)?;
        if !matches!(second, Observation::Ready(_)) {
            return Ok(second);
        }
        anyhow::ensure!(
            matches!((&first_root, &second_root), (Some(a), Some(b)) if a.same(b)?),
            "root changed during observation; retrying"
        );
        anyhow::ensure!(first == second, "tree changed during observation; retrying");
        Ok(second)
    })();
    (tree, activity)
}

/// Build display rows from a loop capture whose tree guard is already released.
fn display_capture(tree: TreeObservation) -> Result<(DisplayCapture, Option<Root>)> {
    let tree = match tree {
        TreeObservation::Busy => return Ok((Observation::Busy, None)),
        TreeObservation::Vacant => return Ok((Observation::Vacant, None)),
        TreeObservation::Ready(tree) => tree,
    };
    let next = grove_loop::select_snapshot(tree.root(), tree.snapshot(), None)?
        .map(|selection| selection.handle.key().get());
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
        Observation::Ready((rows, selected, tree.content, next)),
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn tree_failure_does_not_skip_activity_comparison() {
        for fail_first in [false, true] {
            let work = tempfile::tempdir().unwrap();
            fs::create_dir(work.path().join(".grove")).unwrap();
            fs::write(work.path().join(".grove/_BRIEF.md"), "root bytes").unwrap();
            let duplicate = work.path().join(".grove/02-impl--duplicate-k1.md");
            fs::write(work.path().join(".grove/01-impl--next-k1.md"), "").unwrap();
            if fail_first {
                fs::write(&duplicate, "").unwrap();
            }
            let mut calls = 0;
            let (tree, activity) = capture_with(|| {
                calls += 1;
                let observed = grove_loop::try_observe(work.path(), &[]);
                if calls == 1 {
                    if fail_first {
                        fs::remove_file(&duplicate).unwrap();
                    } else {
                        fs::write(&duplicate, "").unwrap();
                    }
                    fs::create_dir_all(work.path().join(".jj/grove")).unwrap();
                    fs::write(work.path().join(".jj/grove/driver.lease"), "bad").unwrap();
                }
                observed
            });
            assert_eq!(calls, 2);
            assert!(tree.is_err());
            assert!(matches!(activity, ActivityObservation::Busy(_)));
        }
    }

    #[test]
    fn changing_runtime_preserves_a_consistent_tree_and_bounds_captures() {
        let work = tempfile::tempdir().unwrap();
        fs::create_dir(work.path().join(".grove")).unwrap();
        fs::write(work.path().join(".grove/_BRIEF.md"), "root bytes").unwrap();
        fs::write(work.path().join(".grove/01-impl--next-k1.md"), "").unwrap();
        let mut calls = 0;
        let (tree, activity) = capture_with(|| {
            calls += 1;
            let observed = grove_loop::try_observe(work.path(), &[]);
            if calls == 1 {
                fs::create_dir_all(work.path().join(".jj/grove")).unwrap();
                fs::write(work.path().join(".jj/grove/driver.lease"), "bad").unwrap();
            }
            observed
        });
        assert_eq!(calls, 2);
        let Observation::Ready((rows, _, content, next)) = tree.unwrap() else {
            panic!("tree discarded")
        };
        assert_eq!(rows.len(), 2);
        assert_eq!(content.unwrap(), b"root bytes");
        assert_eq!(next, Some(1));
        assert!(matches!(activity, ActivityObservation::Busy(_)));
    }
}

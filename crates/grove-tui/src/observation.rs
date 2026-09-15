use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use grove_loop::{entry_path, Handle, Outcome, Parts, Reading};

pub(crate) struct Row {
    pub path: PathBuf,
    pub label: String,
    pub depth: usize,
    pub branch: bool,
    pub expanded: bool,
    counts: [usize; 3],
}

type Content = Result<Vec<u8>, String>;
type Capture = Option<(Vec<Row>, Content)>;

/// Copy rows and root bytes while guarded. No guard escapes this function.
pub(crate) fn capture(worktree: &Path) -> Result<Capture> {
    let Reading::Tree(tree) = grove_loop::read(worktree)? else {
        return Ok(None);
    };
    let root = tree.snapshot().root();
    let brief = root.distinguished().context("root has no brief")?;
    let path = entry_path(tree.root(), brief);
    let content = read_file(&path);
    let mut rows = vec![Row {
        path,
        label: "root".into(),
        depth: 0,
        branch: true,
        expanded: true,
        counts: [0; 3],
    }];
    for entry in tree.walk() {
        let Some(triple) = entry.triple() else {
            continue;
        };
        let (path, label, counts, branch) = match triple.parts {
            Parts::Leaf { outcome, kind, .. } => {
                let handle = Handle::of_leaf(entry.name()).context("leaf has no handle")?;
                let (status, index) = match outcome {
                    Outcome::Live => ("LIVE", 0),
                    Outcome::Done => ("DONE", 1),
                    Outcome::Abandoned => ("ABANDONED", 2),
                };
                let mut counts = [0; 3];
                counts[index] = 1;
                (
                    entry_path(tree.root(), entry),
                    format!("{handle} {} {status}", kind.label()),
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
                (
                    entry_path(tree.root(), brief),
                    handle.to_string(),
                    [0; 3],
                    true,
                )
            }
        };
        rows.push(Row {
            path,
            label,
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
        if row.branch {
            let [live, done, abandoned] = row.counts;
            let status = if live > 0 {
                "LIVE"
            } else if done > 0 {
                "DONE"
            } else if abandoned > 0 {
                "ABANDONED"
            } else {
                "EMPTY"
            };
            row.label = format!(
                "{} branch {status} [LIVE {live} DONE {done} ABANDONED {abandoned}]",
                row.label
            );
        }
    }
    Ok(Some((rows, content)))
}

/// Re-open under the existing blocking reader. A stale row cannot redirect to
/// an arbitrary path: require its file still to belong to this typed snapshot.
pub(crate) fn read_selected(worktree: &Path, path: &Path) -> Content {
    let read = || -> Result<Vec<u8>> {
        let Reading::Tree(tree) = grove_loop::read(worktree)? else {
            anyhow::bail!("tree is missing")
        };
        let entry = tree
            .walk()
            .find(|entry| entry_path(tree.root(), *entry) == path)
            .context("selected file disappeared; refresh the tree")?;
        let path = entry_path(tree.root(), entry);
        std::fs::read(&path).with_context(|| format!("cannot read {}", path.display()))
    };
    read().map_err(|error| format!("{error:#}"))
}

fn read_file(path: &Path) -> Content {
    std::fs::read(path).map_err(|error| format!("{}: {error}", path.display()))
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

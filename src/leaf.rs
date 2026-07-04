// Shared leaf vocabulary that outlived the old-format reader/grower (swept in
// 090/9.4 once the 060 migration unwired the old verb path). Two items survive:
//
//   * `Kind` — the leaf-kind enum (`work` / `planning`) written into a task
//     file's `**Kind:**` line. Live: the new dotted-decimal grow/lifecycle
//     verbs (`leaf_grow` / `leaf_lifecycle`) and the `grove-llm` CLI surface
//     (`llm_cli`) parse and carry it.
//   * `split_prefix` — the old-format `NNN-slug` prefix parser. Per task-tree-scheme the
//     *only* surviving consumer of the old format is `grove migrate`, which
//     reads an old tree exactly once on adoption; this is the reader it leans on.
//
// Everything else this module once held — the `NNN-slug` / `done/` reader and
// grower (`add`, `insert`, header rewriting, cross-ref surfacing) — was the old
// verb path, made dead by the 060 migration and deleted in 090.

use anyhow::{bail, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Work,
    Planning,
}

impl Kind {
    pub fn parse(s: &str) -> Result<Kind> {
        match s {
            "work" => Ok(Kind::Work),
            "planning" => Ok(Kind::Planning),
            other => bail!("--kind must be `work` or `planning`, got {:?}", other),
        }
    }
}

/// Split `NNN-rest` (NNN exactly three ASCII digits) into `(NNN, rest)`. The
/// old-format prefix parser; `pub(crate)` so `grove migrate` (060/020) can reuse
/// it read-only to consume an old `NNN-slug` tree (it is the *only* surviving
/// consumer of the old format — task-tree-scheme).
pub(crate) fn split_prefix(name: &str) -> Option<(u32, &str)> {
    if name.len() < 4 {
        return None;
    }
    let (head, tail) = name.split_at(3);
    if !head.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !tail.starts_with('-') {
        return None;
    }
    head.parse::<u32>().ok().map(|n| (n, &tail[1..]))
}

#[cfg(test)]
mod inline_tests {
    use super::*;

    #[test]
    fn split_prefix_accepts_three_digits_then_dash() {
        assert_eq!(split_prefix("010-foo"), Some((10, "foo")));
        assert_eq!(split_prefix("990-x-y"), Some((990, "x-y")));
    }

    #[test]
    fn split_prefix_rejects_wrong_shapes() {
        assert!(split_prefix("BRIEF.md").is_none());
        assert!(split_prefix("01-foo").is_none()); // two digits, not three
        assert!(split_prefix("0100-foo").is_none()); // four-digit prefix
        assert!(split_prefix("010_foo").is_none()); // wrong separator
    }

    #[test]
    fn kind_parses_the_two_valid_labels_and_rejects_others() {
        assert_eq!(Kind::parse("work").unwrap(), Kind::Work);
        assert_eq!(Kind::parse("planning").unwrap(), Kind::Planning);
        assert!(Kind::parse("bogus").is_err());
        assert!(Kind::parse("").is_err());
    }
}

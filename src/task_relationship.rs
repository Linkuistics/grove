//! The stable composition relationships a Grove chain step declares.
//!
//! Two markers, `**Reviews:**` and `**Integrates:**`, and they survive the
//! launch-policy removals for one reason: they describe *artifact composition*,
//! not how a session is launched (`docs/specs/config-driven-sessions.md`,
//! *Session kinds live in filenames*). A review names the producer it reviews; an
//! integration names the review it applies. Nothing else in a task body is
//! metadata any more.
//!
//! **This module is the format, in one place.** Before the contraction it was a
//! receipt-shaped reader — a `TaskRelationships { reviews, integrates }` struct
//! that parsed a whole task's metadata at once, which is how a *receipt* consumer
//! composed — and it had no callers at all: the launch policy that read it went
//! with `**Producer launch:**`. Meanwhile the live behaviour spelled the format
//! out three more times, as string literals in two other modules. So the shape
//! that earns its place is neither that struct nor nothing: it is the two
//! writers and the one reader agreeing *by construction* rather than by
//! coincidence.
//!
//! What that buys, concretely: [`Relationship::declare`] is the only place a
//! declaration line is rendered, so `tree_grow`'s hand-cut-equivalent chain and
//! `tree_promotion`'s promoted one cannot drift apart; and [`declaring`] takes a
//! [`Relationship`] rather than a field name, so the lookup can no longer be
//! asked for `"Review"` and answer "nobody declares that" — which, at
//! `tree_promotion`'s scheduled-review gate, is a silently *wrong* answer rather
//! than an error.
//!
//! **Matching is exact-remainder equality, and deliberately not a parse.** Every
//! caller arrives with a known handle and asks who declares it, so the question
//! is a lookup; there is no live caller for "what does this task declare?" and
//! inventing one would put a second reader back. A trailing token therefore does
//! not match — `**Reviews:** build-k1 (stale)` declares nothing — which keeps a
//! half-edited relationship out of the promotion gates instead of silently
//! standing in for the real one.
//!
//! The retired `**Producer launch:**` marker is not named here at all. It
//! survives in exactly one place, and it is not a relationship: `tree_migrate`
//! recognises the line so it can *delete* it from a legacy body, alongside
//! `**Kind:**` and `**Harness:**`. Nothing downstream of migration carries one,
//! and a body that somehow does is prose.

use crate::tree_read::sorted_entries;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};

/// A stable relationship one chain step declares on another's handle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Relationship {
    /// A review names the producer it reviews.
    Reviews,
    /// An integration names the review whose findings it applies.
    Integrates,
}

impl Relationship {
    /// The body marker. Private: a caller that needs the text needs
    /// [`declare`](Self::declare), and one that needs to find it needs
    /// [`declaring`] — exposing the marker is how the format leaked before.
    const fn marker(self) -> &'static str {
        match self {
            Self::Reviews => "**Reviews:**",
            Self::Integrates => "**Integrates:**",
        }
    }

    /// The declaration line a generated step carries.
    pub(crate) fn declare(self, handle: &str) -> String {
        format!("{} {handle}", self.marker())
    }

    /// The generated step's goal sentence. It lives beside the declaration
    /// because the two are written together and were previously duplicated
    /// verbatim across `tree_grow` and `tree_promotion` — a chain cut by
    /// `leaf-add-chain` and one promoted by `leaf-promote-chain` are the same
    /// shape, and that is a property of this format rather than of either verb.
    pub(crate) fn step_goal(self, handle: &str) -> String {
        match self {
            Self::Reviews => format!(
                "Adversarially review `{handle}` and record concrete findings for its integration step."
            ),
            Self::Integrates => format!(
                "Apply the verified findings from `{handle}` while preserving the reviewed artifact's contract."
            ),
        }
    }

    /// Whether `body` declares this relationship on exactly `handle`.
    fn declared_in(self, body: &str, handle: &str) -> bool {
        body.lines().any(|line| {
            line.trim_start()
                .strip_prefix(self.marker())
                .is_some_and(|rest| rest.trim() == handle)
        })
    }
}

/// Every entry directly under `directory` that declares `relationship` on
/// `handle`, in per-level tree order.
///
/// Decomposing a task moves its stable metadata into the new node's `BRIEF.md`,
/// so a directory is searched through its charter and reported by the *node*
/// path. Callers therefore keep one handle/path abstraction whether the
/// relationship's carrier is a leaf or a node.
///
/// Unreadable entries are skipped rather than refused: this is a lookup over
/// whatever is beside the producer, and a foreign file that cannot be read
/// declares nothing. A malformed *name* is likewise not this module's business —
/// the caller re-derives identity from the paths it gets back.
pub(crate) fn declaring(
    directory: &Path,
    relationship: Relationship,
    handle: &str,
) -> Result<Vec<PathBuf>> {
    let mut matches = Vec::new();
    for entry in sorted_entries(directory)? {
        let path = entry.path();
        let file_type = entry.file_type()?;
        let body_path = if file_type.is_file() {
            path.clone()
        } else if file_type.is_dir() {
            path.join("BRIEF.md")
        } else {
            continue;
        };
        let Ok(body) = fs::read_to_string(&body_path) else {
            continue;
        };
        if relationship.declared_in(&body, handle) {
            matches.push(path);
        }
    }
    Ok(matches)
}

/// The one entry declaring `relationship` on `handle`, or `None`.
///
/// Zero and several collapse to the same answer on purpose. The caller is
/// reconstructing a completed chain from the tree, and a producer named by two
/// reviews is no more a reconstructable shape than one named by none — in both
/// cases there is no single review this producer's chain *is*. Reporting which
/// of the two it was would only invite a caller to act on an ambiguity it cannot
/// resolve; [`declaring`] is there when the distinction matters, and
/// `tree_promotion`'s scheduled-review gate uses it to name the offenders.
pub(crate) fn sole_declarer(
    directory: &Path,
    relationship: Relationship,
    handle: &str,
) -> Result<Option<PathBuf>> {
    let mut found = declaring(directory, relationship, handle)?;
    if found.len() == 1 {
        Ok(found.pop())
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn dir() -> TempDir {
        TempDir::new().unwrap()
    }

    fn write(root: &Path, name: &str, body: &str) -> PathBuf {
        let path = root.join(name);
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, body).unwrap();
        path
    }

    /// The writers' half of the contract, stated against literals rather than
    /// against [`Relationship::marker`] — a rendering asserted through its own
    /// renderer is not evidence of what lands on disk.
    #[test]
    fn a_declaration_renders_the_marker_and_the_handle() {
        assert_eq!(
            Relationship::Reviews.declare("build-k1"),
            "**Reviews:** build-k1"
        );
        assert_eq!(
            Relationship::Integrates.declare("build-review-k2"),
            "**Integrates:** build-review-k2"
        );
    }

    /// The round trip is the property the two writers and the one reader share:
    /// whatever `declare` renders, `declaring` finds. Stated here so a change to
    /// the marker text cannot pass by changing both halves in step *and* leaving
    /// on-disk chains unreadable — the literals above pin the text, this pins the
    /// agreement.
    #[test]
    fn what_a_writer_declares_is_what_the_lookup_finds() {
        let tmp = dir();
        for relationship in [Relationship::Reviews, Relationship::Integrates] {
            let body = format!(
                "# step-k9\n\n{}\n\n## Goal\n\n{}\n",
                relationship.declare("target-k1"),
                relationship.step_goal("target-k1")
            );
            let path = write(tmp.path(), "01-impl-step-k9.md", &body);
            assert_eq!(
                declaring(tmp.path(), relationship, "target-k1").unwrap(),
                vec![path],
                "{relationship:?} did not find its own declaration"
            );
        }
    }

    /// The two markers do not answer for each other. Without this, a lookup that
    /// ignored the marker entirely and matched the handle alone would pass every
    /// other test in this file.
    #[test]
    fn a_lookup_matches_only_its_own_marker_and_handle() {
        let tmp = dir();
        write(
            tmp.path(),
            "02-review-impl-build-review-k2.md",
            "# build-review-k2\n\n**Reviews:** build-k1\n",
        );

        assert!(declaring(tmp.path(), Relationship::Integrates, "build-k1")
            .unwrap()
            .is_empty());
        assert!(declaring(tmp.path(), Relationship::Reviews, "build-k2")
            .unwrap()
            .is_empty());
    }

    /// A decomposed producer's review is a *node*: `leaf-decompose` moved the
    /// leaf's metadata into the charter, so the declaration is in `BRIEF.md` and
    /// the answer is the directory. This is the case that makes the caller's
    /// handle/path abstraction uniform.
    #[test]
    fn a_node_declares_through_its_charter_and_answers_by_its_own_path() {
        let tmp = dir();
        write(
            tmp.path(),
            "02-build-review-k2/BRIEF.md",
            "# build-review-k2 — brief\n\n**Reviews:** build-k1\n",
        );

        assert_eq!(
            declaring(tmp.path(), Relationship::Reviews, "build-k1").unwrap(),
            vec![tmp.path().join("02-build-review-k2")],
            "the node path, not its brief, is what a caller resolves identity from"
        );
    }

    /// Exact-remainder equality, stated where it can fail. A half-edited
    /// relationship must not stand in for the real one at a promotion gate.
    #[test]
    fn a_trailing_token_is_not_a_declaration() {
        let tmp = dir();
        write(
            tmp.path(),
            "02-review-impl-build-review-k2.md",
            "# build-review-k2\n\n**Reviews:** build-k1 (stale)\n",
        );

        assert!(declaring(tmp.path(), Relationship::Reviews, "build-k1")
            .unwrap()
            .is_empty());
    }

    /// A marker Grove does not declare is prose — it neither yields a
    /// relationship nor rejects the file. The retired receipt is now just one
    /// such marker, and this is where that stops being a claim about a deleted
    /// parser and becomes a claim about the surviving one.
    #[test]
    fn an_undeclared_body_marker_is_prose_including_a_pre_migration_receipt_line() {
        let tmp = dir();
        let path = write(
            tmp.path(),
            "02-review-impl-build-review-k2.md",
            "# build-review-k2\n\n**Reviews:** build-k1\n\
             **Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\"}\n\
             **Producer launch:** not even JSON\n\
             **Invented:** whatever\n",
        );

        assert_eq!(
            declaring(tmp.path(), Relationship::Reviews, "build-k1").unwrap(),
            vec![path]
        );
    }

    /// Zero and several are the same answer, and [`declaring`] is what tells
    /// them apart. Both directions matter: the collapse is the contract, and the
    /// escape hatch is what makes it safe.
    #[test]
    fn sole_declarer_collapses_zero_and_several_while_declaring_separates_them() {
        let tmp = dir();
        assert_eq!(
            sole_declarer(tmp.path(), Relationship::Reviews, "build-k1").unwrap(),
            None
        );

        let first = write(
            tmp.path(),
            "02-review-impl-build-review-k2.md",
            "# build-review-k2\n\n**Reviews:** build-k1\n",
        );
        assert_eq!(
            sole_declarer(tmp.path(), Relationship::Reviews, "build-k1").unwrap(),
            Some(first.clone())
        );

        let second = write(
            tmp.path(),
            "03-review-impl-build-other-review-k3.md",
            "# build-other-review-k3\n\n**Reviews:** build-k1\n",
        );
        assert_eq!(
            sole_declarer(tmp.path(), Relationship::Reviews, "build-k1").unwrap(),
            None,
            "two reviews are no more a reconstructable chain than none"
        );
        assert_eq!(
            declaring(tmp.path(), Relationship::Reviews, "build-k1").unwrap(),
            vec![first, second],
            "the caller that must name the offenders still can, in tree order"
        );
    }

    /// Order is the per-level tree comparator, not `read_dir`'s. The gate that
    /// reports several declarers renders them into one diagnostic, and a
    /// diagnostic whose order depends on directory-entry layout is not
    /// reproducible.
    #[test]
    fn several_declarers_come_back_in_position_order() {
        let tmp = dir();
        for name in [
            "10-review-impl-j-review-k10.md",
            "02-review-impl-b-review-k2.md",
            "09-review-impl-i-review-k9.md",
        ] {
            write(tmp.path(), name, "**Reviews:** build-k1\n");
        }

        let found = declaring(tmp.path(), Relationship::Reviews, "build-k1").unwrap();
        let names: Vec<String> = found
            .iter()
            .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
            .collect();
        assert_eq!(
            names,
            [
                "02-review-impl-b-review-k2.md",
                "09-review-impl-i-review-k9.md",
                "10-review-impl-j-review-k10.md",
            ],
            "position 9 sorts before position 10 — the lexical order this is not"
        );
    }
}

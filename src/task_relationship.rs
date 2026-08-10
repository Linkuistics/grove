//! Stable relationships carried by Grove task files.
//!
//! The interface parses the `Reviews` / `Integrates` lines a task may declare.
//! Callers never scan siblings or edit task-file metadata.
//!
//! Launch policy is not part of this seam and leaves no residue in it. A
//! review's target is explicit configuration policy, not something Grove infers
//! from a task body (`docs/adr/grove-owns-escalated-review.md`), so the ambient
//! session target, the structured routing peek, the producer/review comparison
//! that rendered diversity notices, and the `**Producer launch:**` receipt they
//! materialised are all gone — the format as well as the writer.
//!
//! The marker survives in exactly one place, and it is not a relationship:
//! `tree_migrate` still recognises the line so it can *delete* it from a legacy
//! body, alongside `**Kind:**` and `**Harness:**`. Nothing downstream of
//! migration carries one, and a body that somehow does is prose here.

use crate::tree_id::validate_slug;
use anyhow::{bail, Context, Result};
use std::fs;
use std::path::Path;

const REVIEWS_MARKER: &str = "**Reviews:**";
const INTEGRATES_MARKER: &str = "**Integrates:**";

/// All stable relationship metadata a task file may carry.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaskRelationships {
    pub reviews: Option<String>,
    pub integrates: Option<String>,
}

impl TaskRelationships {
    pub fn read(path: &Path) -> Result<Self> {
        let text = fs::read_to_string(path)
            .with_context(|| format!("reading task relationships from {}", path.display()))?;
        Self::parse(&text)
    }

    pub fn parse(text: &str) -> Result<Self> {
        let reviews = parse_handle_marker(text, REVIEWS_MARKER)?;
        let integrates = parse_handle_marker(text, INTEGRATES_MARKER)?;
        Ok(Self {
            reviews,
            integrates,
        })
    }
}

fn parse_handle_marker(text: &str, marker: &str) -> Result<Option<String>> {
    let mut value = None;
    for line in text.lines() {
        let Some(rest) = line.trim_start().strip_prefix(marker) else {
            continue;
        };
        if value.is_some() {
            bail!("task carries more than one `{marker}` line");
        }
        let token = rest
            .split_whitespace()
            .next()
            .with_context(|| format!("task carries an empty `{marker}` line"))?;
        validate_handle(token).with_context(|| format!("invalid `{marker}` handle {token:?}"))?;
        value = Some(token.to_string());
    }
    Ok(value)
}

fn validate_handle(handle: &str) -> Result<()> {
    let (slug, key) = handle
        .rsplit_once("-k")
        .with_context(|| format!("stable handle lacks terminal -k<key>: {handle:?}"))?;
    validate_slug(slug)?;
    if key.is_empty() || !key.bytes().all(|byte| byte.is_ascii_digit()) {
        bail!("stable handle has a non-numeric key: {handle:?}");
    }
    let _: u32 = key
        .parse()
        .with_context(|| format!("stable handle key is out of range: {handle:?}"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relationship_parser_reads_both_stable_fields() {
        let relationships =
            TaskRelationships::parse("**Reviews:** build-k1\n**Integrates:** build-review-k2\n")
                .unwrap();
        assert_eq!(relationships.reviews.as_deref(), Some("build-k1"));
        assert_eq!(relationships.integrates.as_deref(), Some("build-review-k2"));
    }

    #[test]
    fn a_declared_handle_is_still_validated_and_may_not_be_declared_twice() {
        for invalid in [
            "**Reviews:** build\n",
            "**Reviews:** build-kx\n",
            "**Reviews:**\n",
            "**Reviews:** build-k1\n**Reviews:** other-k2\n",
        ] {
            assert!(
                TaskRelationships::parse(invalid).is_err(),
                "accepted {invalid:?}"
            );
        }
    }

    /// The removal, stated where it can fail. A body marker Grove does not
    /// declare is prose — it neither yields a relationship nor rejects the file
    /// — and the retired receipt is now just one such marker. Reintroducing a
    /// strict reader for it would break this, which a `!contains` assertion
    /// somewhere else could not detect.
    #[test]
    fn an_undeclared_body_marker_is_prose_including_a_pre_migration_receipt_line() {
        let relationships = TaskRelationships::parse(
            "# build-review-k2\n\n**Reviews:** build-k1\n\
             **Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\",\"model\":null}\n\
             **Producer launch:** not even JSON\n\
             **Invented:** whatever\n",
        )
        .unwrap();

        assert_eq!(
            relationships,
            TaskRelationships {
                reviews: Some("build-k1".to_string()),
                integrates: None,
            }
        );
    }
}

//! Stable relationships carried by Grove task files.
//!
//! The interface keeps three coupled concerns local: parsing the `Reviews` /
//! `Integrates` / producer-launch lines, validating the ephemeral foreground
//! session target before retirement, and atomically replacing the linked
//! review's receipt. Callers never scan siblings or edit task-file metadata.

use crate::harness;
use crate::tree_id::{parse, validate_slug, Entry};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub const SESSION_TARGET_ENV: &str = "GROVE_SESSION_TARGET";

const REVIEWS_MARKER: &str = "**Reviews:**";
const INTEGRATES_MARKER: &str = "**Integrates:**";
const PRODUCER_LAUNCH_MARKER: &str = "**Producer launch:**";

/// A harness plus the exact model selector Grove passed to it. `None` means
/// that harness's own default; it is not a provider-independent model identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchTarget {
    pub harness: String,
    pub model: Option<String>,
}

/// Ephemeral context exported only to the real foreground session. It is
/// evidence about the routing peek, never authority to override the factual
/// tree pick observed at retirement.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct SessionTarget {
    pub worktree: PathBuf,
    pub handle: String,
    #[serde(flatten)]
    pub target: LaunchTarget,
}

impl SessionTarget {
    pub(crate) fn for_launch(
        worktree: &Path,
        handle: String,
        harness: &str,
        model: Option<String>,
    ) -> Self {
        Self {
            worktree: worktree
                .canonicalize()
                .unwrap_or_else(|_| worktree.to_path_buf()),
            handle,
            target: LaunchTarget {
                harness: harness.to_string(),
                model,
            },
        }
    }

    pub(crate) fn to_json(&self) -> Result<String> {
        serde_json::to_string(self).context("serialising the foreground session target")
    }
}

/// Historical producer target materialised in the review task that consumes it.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProducerLaunchReceipt {
    pub producer: String,
    #[serde(flatten)]
    pub target: LaunchTarget,
}

/// All stable relationship metadata a task file may carry. The next slice's
/// review-target comparison reads through this same seam rather than inventing
/// a second parser.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TaskRelationships {
    pub reviews: Option<String>,
    pub integrates: Option<String>,
    pub producer_launch: Option<ProducerLaunchReceipt>,
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
        let producer_launch = parse_receipt_marker(text)?;
        Ok(Self {
            reviews,
            integrates,
            producer_launch,
        })
    }
}

/// Retirement's advisory side effect, prepared under the tree's exclusive lock
/// before `DONE` and enacted only after that terminal rename succeeds.
pub(crate) enum ReceiptPlan {
    NotReviewed,
    Ready(PreparedReceipt),
    Uncheckable(ReceiptDiagnostic),
}

impl ReceiptPlan {
    pub(crate) fn materialize(self) {
        if let Some(diagnostic) = self.materialization_diagnostic() {
            diagnostic.report();
        }
    }

    fn materialization_diagnostic(self) -> Option<ReceiptDiagnostic> {
        match self {
            Self::NotReviewed => None,
            Self::Uncheckable(diagnostic) => Some(diagnostic),
            Self::Ready(receipt) => {
                let producer = receipt.producer.clone();
                let stale = receipt.replaced_existing;
                if let Err(error) = receipt.write() {
                    Some(ReceiptDiagnostic {
                        producer,
                        reason: "receipt-write-failed",
                        detail: if stale {
                            format!(
                                "{error:#}; the pre-existing producer launch receipt may be stale"
                            )
                        } else {
                            format!("{error:#}")
                        },
                    })
                } else {
                    None
                }
            }
        }
    }
}

pub(crate) fn prepare_producer_receipt(
    grove_root: &Path,
    producer_path: &Path,
    producer_handle: &str,
    current_pick: Result<Option<PathBuf>>,
) -> ReceiptPlan {
    match prepare(grove_root, producer_path, producer_handle, current_pick) {
        Ok(Some(receipt)) => ReceiptPlan::Ready(receipt),
        Ok(None) => ReceiptPlan::NotReviewed,
        Err(problem) => ReceiptPlan::Uncheckable(problem),
    }
}

fn prepare(
    grove_root: &Path,
    producer_path: &Path,
    producer_handle: &str,
    current_pick: Result<Option<PathBuf>>,
) -> std::result::Result<Option<PreparedReceipt>, ReceiptDiagnostic> {
    let review_path = unique_review_sibling(producer_path, producer_handle)
        .map_err(|error| diagnostic(producer_handle, error))?;
    let Some(review_path) = review_path else {
        return Ok(None);
    };

    let raw = std::env::var(SESSION_TARGET_ENV).map_err(|_| ReceiptDiagnostic {
        producer: producer_handle.to_string(),
        reason: "session-target-missing",
        detail: format!("{SESSION_TARGET_ENV} is absent"),
    })?;
    if raw.is_empty() {
        return Err(ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "session-target-missing",
            detail: format!("{SESSION_TARGET_ENV} is empty"),
        });
    }
    let session: SessionTarget = serde_json::from_str(&raw).map_err(|error| ReceiptDiagnostic {
        producer: producer_handle.to_string(),
        reason: "session-target-malformed",
        detail: error.to_string(),
    })?;
    validate_handle(&session.handle).map_err(|error| ReceiptDiagnostic {
        producer: producer_handle.to_string(),
        reason: "session-target-malformed",
        detail: format!("invalid routed handle: {error:#}"),
    })?;
    if harness::by_name(&session.target.harness).is_none() {
        return Err(ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "session-target-malformed",
            detail: format!("unknown harness {:?}", session.target.harness),
        });
    }

    let worktree = grove_root
        .parent()
        .unwrap_or(grove_root)
        .canonicalize()
        .map_err(|error| ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "worktree-unresolvable",
            detail: error.to_string(),
        })?;
    if session.worktree != worktree {
        return Err(ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "worktree-mismatch",
            detail: format!(
                "session target names {}, retiring tree is {}",
                session.worktree.display(),
                worktree.display()
            ),
        });
    }
    if session.handle != producer_handle {
        return Err(ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "routed-handle-mismatch",
            detail: format!("session target names {}", session.handle),
        });
    }

    let picked = current_pick.map_err(|error| ReceiptDiagnostic {
        producer: producer_handle.to_string(),
        reason: "current-pick-unreadable",
        detail: format!("{error:#}"),
    })?;
    let producer_identity = producer_path
        .canonicalize()
        .map_err(|error| ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "producer-unresolvable",
            detail: error.to_string(),
        })?;
    let picked_identity = picked
        .as_deref()
        .map(Path::canonicalize)
        .transpose()
        .map_err(|error| ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "current-pick-unreadable",
            detail: error.to_string(),
        })?;
    if picked_identity.as_deref() != Some(producer_identity.as_path()) {
        return Err(ReceiptDiagnostic {
            producer: producer_handle.to_string(),
            reason: "factual-pick-mismatch",
            detail: picked_identity.map_or_else(
                || "the tree has no live factual pick".to_string(),
                |path| format!("the factual pick is {}", path.display()),
            ),
        });
    }

    let receipt = ProducerLaunchReceipt {
        producer: producer_handle.to_string(),
        target: session.target,
    };
    PreparedReceipt::new(review_path, receipt)
        .map(Some)
        .map_err(|error| diagnostic(producer_handle, error))
}

pub(crate) struct PreparedReceipt {
    path: PathBuf,
    producer: String,
    content: String,
    permissions: fs::Permissions,
    replaced_existing: bool,
}

impl PreparedReceipt {
    fn new(path: PathBuf, receipt: ProducerLaunchReceipt) -> Result<Self> {
        let original = fs::read_to_string(&path)
            .with_context(|| format!("reading linked review task {}", path.display()))?;
        let permissions = fs::metadata(&path)
            .with_context(|| format!("reading permissions for {}", path.display()))?
            .permissions();
        let replaced_existing = original
            .lines()
            .any(|line| line.trim_start().starts_with(PRODUCER_LAUNCH_MARKER));
        let content = render_replaced_receipt(&original, &receipt)?;
        Ok(Self {
            path,
            producer: receipt.producer,
            content,
            permissions,
            replaced_existing,
        })
    }

    fn write(self) -> Result<()> {
        let parent = self
            .path
            .parent()
            .with_context(|| format!("review task has no parent: {}", self.path.display()))?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent).with_context(|| {
            format!("creating a receipt temporary file in {}", parent.display())
        })?;
        temporary
            .as_file_mut()
            .write_all(self.content.as_bytes())
            .with_context(|| {
                format!(
                    "writing producer launch receipt for {}",
                    self.path.display()
                )
            })?;
        temporary
            .as_file_mut()
            .set_permissions(self.permissions)
            .with_context(|| format!("preserving permissions for {}", self.path.display()))?;
        temporary
            .persist(&self.path)
            .map_err(|error| error.error)
            .with_context(|| format!("atomically replacing {}", self.path.display()))?;
        Ok(())
    }
}

pub(crate) struct ReceiptDiagnostic {
    producer: String,
    reason: &'static str,
    detail: String,
}

impl ReceiptDiagnostic {
    fn report(self) {
        eprintln!(
            "grove: producer launch receipt uncheckable (producer={}, reason={}): {}; \
             retirement remains DONE",
            self.producer, self.reason, self.detail
        );
    }
}

fn diagnostic(producer: &str, error: anyhow::Error) -> ReceiptDiagnostic {
    let detail = format!("{error:#}");
    let reason = if detail.contains("more than one sibling") {
        "review-relationship-ambiguous"
    } else if detail.contains("Reviews") {
        "review-relationship-malformed"
    } else {
        "receipt-preparation-failed"
    };
    ReceiptDiagnostic {
        producer: producer.to_string(),
        reason,
        detail,
    }
}

fn unique_review_sibling(producer_path: &Path, producer_handle: &str) -> Result<Option<PathBuf>> {
    let parent = producer_path
        .parent()
        .with_context(|| format!("producer has no sibling level: {}", producer_path.display()))?;
    let mut matches = Vec::new();
    for entry in fs::read_dir(parent)
        .with_context(|| format!("reading producer sibling level {}", parent.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if !matches!(parse(name), Some(Entry::Leaf { .. })) {
            continue;
        }
        let text = fs::read_to_string(entry.path())
            .with_context(|| format!("reading task file {}", entry.path().display()))?;
        if parse_handle_marker(&text, REVIEWS_MARKER)?.as_deref() == Some(producer_handle) {
            matches.push(entry.path());
        }
    }
    match matches.len() {
        0 => Ok(None),
        1 => Ok(matches.pop()),
        count => bail!(
            "more than one sibling ({count}) declares `{REVIEWS_MARKER} {producer_handle}`: {}",
            matches
                .iter()
                .map(|path| path.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
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

fn parse_receipt_marker(text: &str) -> Result<Option<ProducerLaunchReceipt>> {
    let mut receipt = None;
    for line in text.lines() {
        let Some(rest) = line.trim_start().strip_prefix(PRODUCER_LAUNCH_MARKER) else {
            continue;
        };
        if receipt.is_some() {
            bail!("task carries more than one `{PRODUCER_LAUNCH_MARKER}` line");
        }
        let parsed: ProducerLaunchReceipt = serde_json::from_str(rest.trim())
            .with_context(|| format!("parsing `{PRODUCER_LAUNCH_MARKER}` JSON"))?;
        validate_handle(&parsed.producer)
            .with_context(|| format!("invalid receipt producer {:?}", parsed.producer))?;
        if harness::by_name(&parsed.target.harness).is_none() {
            bail!(
                "producer launch receipt names unknown harness {:?}",
                parsed.target.harness
            );
        }
        receipt = Some(parsed);
    }
    Ok(receipt)
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

fn render_replaced_receipt(original: &str, receipt: &ProducerLaunchReceipt) -> Result<String> {
    let line = format!(
        "{PRODUCER_LAUNCH_MARKER} {}\n",
        serde_json::to_string(receipt).context("serialising the producer launch receipt")?
    );
    let mut rendered = String::with_capacity(original.len() + line.len());
    let mut inserted = false;
    for source_line in original.split_inclusive('\n') {
        let trimmed = source_line.trim_start();
        if trimmed.starts_with(PRODUCER_LAUNCH_MARKER) {
            continue;
        }
        rendered.push_str(source_line);
        if trimmed.starts_with(REVIEWS_MARKER) && !inserted {
            if !source_line.ends_with('\n') {
                rendered.push('\n');
            }
            rendered.push_str(&line);
            inserted = true;
        }
    }
    if !inserted {
        bail!("linked review task lost its `{REVIEWS_MARKER}` relationship before replacement");
    }
    Ok(rendered)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn relationship_parser_reads_all_three_fields() {
        let relationships = TaskRelationships::parse(
            "**Reviews:** build-k1\n**Integrates:** build-review-k2\n\
             **Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\",\"model\":null}\n",
        )
        .unwrap();
        assert_eq!(relationships.reviews.as_deref(), Some("build-k1"));
        assert_eq!(relationships.integrates.as_deref(), Some("build-review-k2"));
        assert_eq!(
            relationships.producer_launch.unwrap().target,
            LaunchTarget {
                harness: "claude".to_string(),
                model: None,
            }
        );
    }

    #[test]
    fn replacement_is_adjacent_and_unconditional() {
        let receipt = ProducerLaunchReceipt {
            producer: "build-k1".to_string(),
            target: LaunchTarget {
                harness: "claude".to_string(),
                model: Some("opus".to_string()),
            },
        };
        let rendered = render_replaced_receipt(
            "**Producer launch:** broken\n**Reviews:** build-k1\nbody\n",
            &receipt,
        )
        .unwrap();
        assert_eq!(rendered.matches(PRODUCER_LAUNCH_MARKER).count(), 1);
        assert!(rendered
            .starts_with("**Reviews:** build-k1\n**Producer launch:** {\"producer\":\"build-k1\""));
    }

    #[test]
    fn post_done_write_failure_stays_advisory_and_marks_old_receipt_stale() {
        for (prior_receipt, should_warn_stale) in [("", false), ("old", true)] {
            let tmp = TempDir::new().unwrap();
            let producer = tmp.path().join("01-build-k1.md");
            let done = tmp.path().join("01-DONE-build-k1.md");
            let review = tmp.path().join("02-build-review-k2.md");
            fs::write(&producer, "# build-k1\n").unwrap();
            let prior = if prior_receipt.is_empty() {
                String::new()
            } else {
                format!(
                    "{PRODUCER_LAUNCH_MARKER} \
                     {{\"producer\":\"build-k1\",\"harness\":\"codex\",\"model\":\"old\"}}\n"
                )
            };
            fs::write(
                &review,
                format!("**Reviews:** build-k1\n{prior}review body\n"),
            )
            .unwrap();
            let prepared = PreparedReceipt::new(
                review.clone(),
                ProducerLaunchReceipt {
                    producer: "build-k1".to_string(),
                    target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: Some("opus".to_string()),
                    },
                },
            )
            .unwrap();

            fs::rename(&producer, &done).unwrap();
            fs::remove_file(&review).unwrap();
            fs::create_dir(&review).unwrap();

            let diagnostic = ReceiptPlan::Ready(prepared)
                .materialization_diagnostic()
                .expect("replacing a directory must fail advisory receipt materialisation");
            assert_eq!(diagnostic.reason, "receipt-write-failed");
            assert_eq!(
                diagnostic
                    .detail
                    .contains("pre-existing producer launch receipt may be stale"),
                should_warn_stale
            );
            assert!(done.is_file(), "receipt failure must not reverse DONE");
        }
    }
}

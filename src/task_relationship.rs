//! Stable relationships carried by Grove task files.
//!
//! The interface keeps four coupled concerns local: parsing the `Reviews` /
//! `Integrates` / producer-launch lines, validating the ephemeral foreground
//! session target before retirement, atomically replacing the linked review's
//! receipt, and comparing that receipt with the review target at launch.
//! Callers never scan siblings or edit task-file metadata.

use crate::harness;
use crate::tree_id::{parse, validate_slug, Entry};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
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
    #[serde(deserialize_with = "deserialize_nullable_model")]
    pub model: Option<String>,
}

fn deserialize_nullable_model<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
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
/// `session` and `generation` are absent only on a legacy direct-leaf receipt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProducerLaunchReceipt {
    pub producer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generation: Option<String>,
    #[serde(flatten)]
    pub target: LaunchTarget,
}

#[derive(Default)]
enum Present<T> {
    #[default]
    Missing,
    Value(T),
}

impl<'de, T> Deserialize<'de> for Present<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Self::Value)
    }
}

#[derive(Deserialize)]
struct ProducerLaunchReceiptWire {
    producer: String,
    #[serde(default)]
    session: Present<String>,
    #[serde(default)]
    generation: Present<String>,
    harness: String,
    #[serde(deserialize_with = "deserialize_nullable_model")]
    model: Option<String>,
}

impl<'de> Deserialize<'de> for ProducerLaunchReceipt {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let wire = ProducerLaunchReceiptWire::deserialize(deserializer)?;
        let (session, generation) = match (wire.session, wire.generation) {
            (Present::Missing, Present::Missing) => (None, None),
            (Present::Value(session), Present::Value(generation)) => {
                (Some(session), Some(generation))
            }
            _ => {
                return Err(serde::de::Error::custom(
                    "receipt fields `session` and `generation` must either both be present or both be absent",
                ))
            }
        };
        Ok(Self {
            producer: wire.producer,
            session,
            generation,
            target: LaunchTarget {
                harness: wire.harness,
                model: wire.model,
            },
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) struct ProducerReceiptCandidate {
    pub path: PathBuf,
    pub handle: String,
    pub generation: u32,
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

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "status", rename_all = "kebab-case", deny_unknown_fields)]
pub enum ReviewEvidence {
    Checkable {
        producer: String,
        session: String,
        generation: String,
        #[serde(rename = "producer-target")]
        producer_target: LaunchTarget,
    },
    Uncheckable {
        #[serde(deserialize_with = "deserialize_nullable_model")]
        producer: Option<String>,
        reason: String,
    },
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

pub(crate) fn review_evidence_unlocked(grove_root: &Path, review_path: &Path) -> ReviewEvidence {
    let text = match fs::read_to_string(review_path) {
        Ok(text) => text,
        Err(_) => return uncheckable(None, "review-task-unreadable"),
    };
    let producer = match parse_handle_marker(&text, REVIEWS_MARKER) {
        Ok(Some(producer)) => producer,
        Ok(None) => return uncheckable(None, "review-relationship-missing"),
        Err(_) => return uncheckable(None, "review-relationship-malformed"),
    };
    let receipt = match parse_receipt_marker(&text) {
        Ok(Some(receipt)) => receipt,
        Ok(None) => return uncheckable(Some(producer), "producer-receipt-missing"),
        Err(_) => return uncheckable(Some(producer), "producer-receipt-malformed"),
    };
    if receipt.producer != producer {
        return uncheckable(Some(producer), "receipt-producer-mismatch");
    }

    let producer_path = match crate::tree_read::resolve_unlocked(grove_root, &producer) {
        Ok(crate::tree_read::Resolution::Found { path, .. })
            if path_handle(&path).as_deref() == Some(producer.as_str()) =>
        {
            path
        }
        Ok(crate::tree_read::Resolution::NotFound) => {
            return uncheckable(Some(producer), "reviewed-producer-missing")
        }
        _ => return uncheckable(Some(producer), "reviewed-producer-unresolvable"),
    };
    let producer_is_node = producer_path.is_dir();
    if producer_is_node {
        if !producer_path.join("BRIEF.md").is_file() {
            return uncheckable(Some(producer), "reviewed-producer-not-decomposition");
        }
        match contains_live_leaf(&producer_path) {
            Ok(true) => return uncheckable(Some(producer), "reviewed-producer-live"),
            Err(_) => return uncheckable(Some(producer), "reviewed-producer-unreadable"),
            Ok(false) => {}
        }
    } else if !is_terminal_leaf(&producer_path) {
        return uncheckable(Some(producer), "reviewed-producer-live");
    }

    let generation_key = match crate::tree_read::producer_generation_unlocked(&producer_path) {
        Ok(generation) => generation,
        Err(_) => return uncheckable(Some(producer), "producer-generation-unreadable"),
    };
    let generation = format!("k{generation_key}");
    let (session, recorded_generation) = match (&receipt.session, &receipt.generation) {
        (Some(session), Some(generation)) => (session.clone(), generation.clone()),
        (None, None) if !producer_is_node => (producer.clone(), generation.clone()),
        (None, None) => return uncheckable(Some(producer), "producer-receipt-legacy-node"),
        _ => return uncheckable(Some(producer), "producer-receipt-malformed"),
    };
    if recorded_generation != generation {
        return uncheckable(Some(producer), "producer-generation-mismatch");
    }

    let source_path = match crate::tree_read::resolve_unlocked(grove_root, &session) {
        Ok(crate::tree_read::Resolution::Found { path, .. })
            if path_handle(&path).as_deref() == Some(session.as_str()) =>
        {
            path
        }
        Ok(crate::tree_read::Resolution::NotFound) => {
            return uncheckable(Some(producer), "producer-session-missing")
        }
        _ => return uncheckable(Some(producer), "producer-session-unresolvable"),
    };
    if producer_is_node {
        if !source_path.starts_with(&producer_path) || !is_terminal_leaf(&source_path) {
            return uncheckable(Some(producer), "producer-session-not-terminal-descendant");
        }
    } else if source_path != producer_path {
        return uncheckable(Some(producer), "producer-session-mismatch");
    }

    ReviewEvidence::Checkable {
        producer,
        session,
        generation,
        producer_target: receipt.target,
    }
}

fn uncheckable(producer: Option<String>, reason: &str) -> ReviewEvidence {
    ReviewEvidence::Uncheckable {
        producer,
        reason: reason.to_string(),
    }
}

fn path_handle(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .and_then(parse)
        .and_then(|entry| entry.handle())
}

fn is_terminal_leaf(path: &Path) -> bool {
    matches!(
        path.file_name()
            .and_then(|name| name.to_str())
            .and_then(parse),
        Some(Entry::Leaf {
            outcome: crate::tree_id::Outcome::Done | crate::tree_id::Outcome::Abandoned,
            ..
        })
    )
}

fn contains_live_leaf(node: &Path) -> Result<bool> {
    for entry in fs::read_dir(node).with_context(|| format!("reading {}", node.display()))? {
        let entry = entry?;
        let Some(parsed) = entry.file_name().to_str().and_then(parse) else {
            continue;
        };
        match parsed {
            Entry::Leaf {
                outcome: crate::tree_id::Outcome::Live,
                ..
            } => return Ok(true),
            Entry::Node { .. } if contains_live_leaf(&entry.path())? => return Ok(true),
            _ => {}
        }
    }
    Ok(false)
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ReviewTargetDiversity {
    Diverse,
    Matching {
        producer: String,
        session: String,
        producer_target: LaunchTarget,
        harness: bool,
        model: bool,
    },
    Uncheckable {
        producer: Option<String>,
        reason: String,
    },
}

/// Render the advisory target-diversity notice from the guarded routing peek.
///
/// The caller retains this evidence from the same structured peek that selected
/// the route. Consuming it directly is load-bearing: rereading the task after
/// the shared tree guard opens a TOCTOU window where the warning can describe a
/// different review than the launch was routed for.
pub(crate) fn review_diversity_notice(
    review_handle: &str,
    evidence: &ReviewEvidence,
    review_target: &LaunchTarget,
) -> Option<String> {
    let comparison = compare_review_target_diversity(evidence, review_target);
    match comparison {
        ReviewTargetDiversity::Diverse => None,
        _ => Some(render_review_diversity_warning(
            review_handle,
            review_target,
            &comparison,
        )),
    }
}

fn compare_review_target_diversity(
    evidence: &ReviewEvidence,
    review_target: &LaunchTarget,
) -> ReviewTargetDiversity {
    let (producer, session, producer_target) = match evidence {
        ReviewEvidence::Checkable {
            producer,
            session,
            producer_target,
            ..
        } => (producer, session, producer_target),
        ReviewEvidence::Uncheckable { producer, reason } => {
            return ReviewTargetDiversity::Uncheckable {
                producer: producer.clone(),
                reason: reason.clone(),
            };
        }
    };

    let harness = producer_target.harness == review_target.harness;
    let model = match (&producer_target.model, &review_target.model) {
        (Some(producer), Some(review)) => producer == review,
        (None, None) => harness,
        _ => false,
    };
    if !harness && !model {
        ReviewTargetDiversity::Diverse
    } else {
        ReviewTargetDiversity::Matching {
            producer: producer.clone(),
            session: session.clone(),
            producer_target: producer_target.clone(),
            harness,
            model,
        }
    }
}

fn render_review_diversity_warning(
    review_handle: &str,
    review_target: &LaunchTarget,
    comparison: &ReviewTargetDiversity,
) -> String {
    let review_target = render_launch_target(review_target);
    match comparison {
        ReviewTargetDiversity::Diverse => String::new(),
        ReviewTargetDiversity::Matching {
            producer,
            session,
            producer_target,
            harness,
            model,
        } => {
            let matching = match (*harness, *model) {
                (true, true) => "harness+model",
                (true, false) => "harness",
                (false, true) => "model",
                (false, false) => unreachable!("a matching result has at least one axis"),
            };
            let source_session = if session != producer {
                format!(", session={session}")
            } else {
                String::new()
            };
            format!(
                "grove: review target diversity warning (review={review_handle}, \
                 producer={producer}{source_session}, matching={matching}, producer-target={}, \
                 review-target={review_target}); applies only if factual pick is \
                 review={review_handle}; discard otherwise; launch continues",
                render_launch_target(producer_target)
            )
        }
        ReviewTargetDiversity::Uncheckable { producer, reason } => format!(
            "grove: review target diversity warning (review={review_handle}, producer={}, \
             uncheckable(reason={reason}), producer-target=unavailable, \
             review-target={review_target}); applies only if factual pick is \
             review={review_handle}; discard otherwise; launch continues",
            producer.as_deref().unwrap_or("unknown")
        ),
    }
}

fn render_launch_target(target: &LaunchTarget) -> String {
    let model = target
        .model
        .as_ref()
        .map(|model| format!("{model:?}"))
        .unwrap_or_else(|| format!("default({})", target.harness));
    format!("{}/{model}", target.harness)
}

/// Retirement's advisory side effect, prepared under the tree's exclusive lock
/// before `DONE` and enacted only after that terminal rename succeeds.
pub(crate) enum ReceiptPlan {
    NotReviewed,
    Ready(PreparedReceipt),
    Uncheckable(ReceiptDiagnostic),
}

pub(crate) struct ReceiptPlans(Vec<ReceiptPlan>);

impl ReceiptPlans {
    pub(crate) fn materialize(self) {
        for plan in self.0 {
            plan.materialize();
        }
    }
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
                let producer = receipt.receipt.producer.clone();
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

pub(crate) fn prepare_producer_receipts(
    grove_root: &Path,
    factual_leaf_path: &Path,
    factual_leaf_handle: &str,
    candidates: Vec<ProducerReceiptCandidate>,
    current_pick: Result<Option<PathBuf>>,
) -> ReceiptPlans {
    let mut plans = Vec::new();
    let mut live_reviews = Vec::new();
    for candidate in candidates {
        match unique_review_sibling(&candidate.path, &candidate.handle) {
            Ok(None) => plans.push(ReceiptPlan::NotReviewed),
            Ok(Some((_, outcome))) if outcome != crate::tree_id::Outcome::Live => {
                plans.push(ReceiptPlan::Uncheckable(ReceiptDiagnostic {
                    producer: candidate.handle,
                    reason: "review-terminal",
                    detail: "the linked review is terminal and remains byte-identical".to_string(),
                }));
            }
            Ok(Some((review_path, _))) => live_reviews.push((candidate, review_path)),
            Err(error) => plans.push(ReceiptPlan::Uncheckable(diagnostic(
                &candidate.handle,
                error,
            ))),
        }
    }

    if live_reviews.is_empty() {
        return ReceiptPlans(plans);
    }

    let session = validate_session_target(
        grove_root,
        factual_leaf_path,
        factual_leaf_handle,
        current_pick,
    );
    for (candidate, review_path) in live_reviews {
        let plan = match &session {
            Ok(session) => {
                let receipt = ProducerLaunchReceipt {
                    producer: candidate.handle.clone(),
                    session: Some(factual_leaf_handle.to_string()),
                    generation: Some(format!("k{}", candidate.generation)),
                    target: session.target.clone(),
                };
                match PreparedReceipt::new(review_path, receipt) {
                    Ok(receipt) => ReceiptPlan::Ready(receipt),
                    Err(error) => ReceiptPlan::Uncheckable(diagnostic(&candidate.handle, error)),
                }
            }
            Err(problem) => ReceiptPlan::Uncheckable(ReceiptDiagnostic {
                producer: candidate.handle,
                reason: problem.reason,
                detail: problem.detail.clone(),
            }),
        };
        plans.push(plan);
    }
    ReceiptPlans(plans)
}

struct SessionProblem {
    reason: &'static str,
    detail: String,
}

fn validate_session_target(
    grove_root: &Path,
    factual_leaf_path: &Path,
    factual_leaf_handle: &str,
    current_pick: Result<Option<PathBuf>>,
) -> std::result::Result<SessionTarget, SessionProblem> {
    let raw = std::env::var(SESSION_TARGET_ENV).map_err(|_| SessionProblem {
        reason: "session-target-missing",
        detail: format!("{SESSION_TARGET_ENV} is absent"),
    })?;
    if raw.is_empty() {
        return Err(SessionProblem {
            reason: "session-target-missing",
            detail: format!("{SESSION_TARGET_ENV} is empty"),
        });
    }
    let session: SessionTarget = serde_json::from_str(&raw).map_err(|error| SessionProblem {
        reason: "session-target-malformed",
        detail: error.to_string(),
    })?;
    validate_handle(&session.handle).map_err(|error| SessionProblem {
        reason: "session-target-malformed",
        detail: format!("invalid routed handle: {error:#}"),
    })?;
    if harness::by_name(&session.target.harness).is_none() {
        return Err(SessionProblem {
            reason: "session-target-malformed",
            detail: format!("unknown harness {:?}", session.target.harness),
        });
    }
    if session.target.model.as_deref() == Some("") {
        return Err(SessionProblem {
            reason: "session-target-malformed",
            detail: "model selector is empty".to_string(),
        });
    }

    let worktree = grove_root
        .parent()
        .unwrap_or(grove_root)
        .canonicalize()
        .map_err(|error| SessionProblem {
            reason: "worktree-unresolvable",
            detail: error.to_string(),
        })?;
    if session.worktree != worktree {
        return Err(SessionProblem {
            reason: "worktree-mismatch",
            detail: format!(
                "session target names {}, retiring tree is {}",
                session.worktree.display(),
                worktree.display()
            ),
        });
    }
    if session.handle != factual_leaf_handle {
        return Err(SessionProblem {
            reason: "routed-handle-mismatch",
            detail: format!("session target names {}", session.handle),
        });
    }

    let picked = current_pick.map_err(|error| SessionProblem {
        reason: "current-pick-unreadable",
        detail: format!("{error:#}"),
    })?;
    let factual_leaf_identity =
        factual_leaf_path
            .canonicalize()
            .map_err(|error| SessionProblem {
                reason: "producer-unresolvable",
                detail: error.to_string(),
            })?;
    let picked_identity = picked
        .as_deref()
        .map(Path::canonicalize)
        .transpose()
        .map_err(|error| SessionProblem {
            reason: "current-pick-unreadable",
            detail: error.to_string(),
        })?;
    if picked_identity.as_deref() != Some(factual_leaf_identity.as_path()) {
        return Err(SessionProblem {
            reason: "factual-pick-mismatch",
            detail: picked_identity.map_or_else(
                || "the tree has no live factual pick".to_string(),
                |path| format!("the factual pick is {}", path.display()),
            ),
        });
    }
    Ok(session)
}

pub(crate) struct PreparedReceipt {
    path: PathBuf,
    receipt: ProducerLaunchReceipt,
    replaced_existing: bool,
}

impl PreparedReceipt {
    fn new(path: PathBuf, receipt: ProducerLaunchReceipt) -> Result<Self> {
        let original = fs::read_to_string(&path)
            .with_context(|| format!("reading linked review task {}", path.display()))?;
        let relationships = TaskRelationships::parse(&original)?;
        if relationships.reviews.as_deref() != Some(receipt.producer.as_str()) {
            bail!(
                "linked review task no longer declares `{REVIEWS_MARKER} {}`",
                receipt.producer
            );
        }
        let replaced_existing = original
            .lines()
            .any(|line| line.trim_start().starts_with(PRODUCER_LAUNCH_MARKER));
        Ok(Self {
            path,
            receipt,
            replaced_existing,
        })
    }

    fn write(self) -> Result<()> {
        let original = fs::read_to_string(&self.path)
            .with_context(|| format!("re-reading linked review task {}", self.path.display()))?;
        let permissions = fs::metadata(&self.path)
            .with_context(|| format!("reading permissions for {}", self.path.display()))?
            .permissions();
        let content = render_replaced_receipt(&original, &self.receipt)?;
        let parent = self
            .path
            .parent()
            .with_context(|| format!("review task has no parent: {}", self.path.display()))?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent).with_context(|| {
            format!("creating a receipt temporary file in {}", parent.display())
        })?;
        temporary
            .as_file_mut()
            .write_all(content.as_bytes())
            .with_context(|| {
                format!(
                    "writing producer launch receipt for {}",
                    self.path.display()
                )
            })?;
        temporary
            .as_file_mut()
            .set_permissions(permissions)
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

fn unique_review_sibling(
    producer_path: &Path,
    producer_handle: &str,
) -> Result<Option<(PathBuf, crate::tree_id::Outcome)>> {
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
        let Some(Entry::Leaf { outcome, .. }) = parse(name) else {
            continue;
        };
        let text = fs::read_to_string(entry.path())
            .with_context(|| format!("reading task file {}", entry.path().display()))?;
        if parse_handle_marker(&text, REVIEWS_MARKER)?.as_deref() == Some(producer_handle) {
            matches.push((entry.path(), outcome));
        }
    }
    match matches.len() {
        0 => Ok(None),
        1 => Ok(matches.pop()),
        count => bail!(
            "more than one sibling ({count}) declares `{REVIEWS_MARKER} {producer_handle}`: {}",
            matches
                .iter()
                .map(|(path, _)| path.display().to_string())
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
        if let Some(session) = &parsed.session {
            validate_handle(session)
                .with_context(|| format!("invalid receipt session {session:?}"))?;
        }
        if let Some(generation) = &parsed.generation {
            validate_generation(generation)
                .with_context(|| format!("invalid receipt generation {generation:?}"))?;
        }
        if harness::by_name(&parsed.target.harness).is_none() {
            bail!(
                "producer launch receipt names unknown harness {:?}",
                parsed.target.harness
            );
        }
        if parsed.target.model.as_deref() == Some("") {
            bail!("producer launch receipt carries an empty model selector");
        }
        receipt = Some(parsed);
    }
    Ok(receipt)
}

fn validate_generation(generation: &str) -> Result<u32> {
    let digits = generation
        .strip_prefix('k')
        .with_context(|| "generation must start with `k`")?;
    if digits.is_empty() || !digits.bytes().all(|byte| byte.is_ascii_digit()) {
        bail!("generation must be `k<positive digits>`");
    }
    let key = digits
        .parse::<u32>()
        .context("generation key is too large")?;
    if key == 0 {
        bail!("generation key must be positive");
    }
    Ok(key)
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
    fn review_target_comparison_covers_matching_and_default_model_axes() {
        let cases = [
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("sonnet".to_string()),
                },
                ReviewTargetDiversity::Matching {
                    producer: "build-k1".to_string(),
                    session: "build-k1".to_string(),
                    producer_target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: Some("opus".to_string()),
                    },
                    harness: true,
                    model: false,
                },
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("shared".to_string()),
                },
                LaunchTarget {
                    harness: "codex".to_string(),
                    model: Some("shared".to_string()),
                },
                ReviewTargetDiversity::Matching {
                    producer: "build-k1".to_string(),
                    session: "build-k1".to_string(),
                    producer_target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: Some("shared".to_string()),
                    },
                    harness: false,
                    model: true,
                },
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                ReviewTargetDiversity::Matching {
                    producer: "build-k1".to_string(),
                    session: "build-k1".to_string(),
                    producer_target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: Some("opus".to_string()),
                    },
                    harness: true,
                    model: true,
                },
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                LaunchTarget {
                    harness: "codex".to_string(),
                    model: Some("o3".to_string()),
                },
                ReviewTargetDiversity::Diverse,
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: None,
                },
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: None,
                },
                ReviewTargetDiversity::Matching {
                    producer: "build-k1".to_string(),
                    session: "build-k1".to_string(),
                    producer_target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: None,
                    },
                    harness: true,
                    model: true,
                },
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: None,
                },
                LaunchTarget {
                    harness: "codex".to_string(),
                    model: None,
                },
                ReviewTargetDiversity::Diverse,
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: None,
                },
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                ReviewTargetDiversity::Matching {
                    producer: "build-k1".to_string(),
                    session: "build-k1".to_string(),
                    producer_target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: None,
                    },
                    harness: true,
                    model: false,
                },
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: None,
                },
                ReviewTargetDiversity::Matching {
                    producer: "build-k1".to_string(),
                    session: "build-k1".to_string(),
                    producer_target: LaunchTarget {
                        harness: "claude".to_string(),
                        model: Some("opus".to_string()),
                    },
                    harness: true,
                    model: false,
                },
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: None,
                },
                LaunchTarget {
                    harness: "codex".to_string(),
                    model: Some("o3".to_string()),
                },
                ReviewTargetDiversity::Diverse,
            ),
            (
                LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
                LaunchTarget {
                    harness: "codex".to_string(),
                    model: None,
                },
                ReviewTargetDiversity::Diverse,
            ),
        ];

        for (producer_target, review_target, expected) in cases {
            let evidence = ReviewEvidence::Checkable {
                producer: "build-k1".to_string(),
                session: "build-k1".to_string(),
                generation: "k1".to_string(),
                producer_target,
            };
            assert_eq!(
                compare_review_target_diversity(&evidence, &review_target),
                expected
            );
        }
    }

    #[test]
    fn review_target_comparison_preserves_every_uncheckable_evidence_reason() {
        let target = LaunchTarget {
            harness: "codex".to_string(),
            model: Some("o3".to_string()),
        };
        let cases = [
            (None, "review-relationship-missing"),
            (None, "review-relationship-malformed"),
            (Some("build-k1"), "producer-receipt-missing"),
            (Some("build-k1"), "producer-receipt-malformed"),
            (Some("build-k1"), "receipt-producer-mismatch"),
        ];

        for (producer, reason) in cases {
            let evidence = ReviewEvidence::Uncheckable {
                producer: producer.map(str::to_string),
                reason: reason.to_string(),
            };
            assert_eq!(
                compare_review_target_diversity(&evidence, &target),
                ReviewTargetDiversity::Uncheckable {
                    producer: producer.map(str::to_string),
                    reason: reason.to_string(),
                }
            );
        }
    }

    #[test]
    fn review_warning_names_only_a_valid_relationship_and_renders_defaults() {
        let review_target = LaunchTarget {
            harness: "claude".to_string(),
            model: None,
        };
        let matching = ReviewTargetDiversity::Matching {
            producer: "build-k1".to_string(),
            session: "finish-k7".to_string(),
            producer_target: review_target.clone(),
            harness: true,
            model: true,
        };
        assert_eq!(
            render_review_diversity_warning("build-review-k2", &review_target, &matching),
            "grove: review target diversity warning (review=build-review-k2, producer=build-k1, session=finish-k7, matching=harness+model, producer-target=claude/default(claude), review-target=claude/default(claude)); applies only if factual pick is review=build-review-k2; discard otherwise; launch continues"
        );

        let uncheckable = ReviewTargetDiversity::Uncheckable {
            producer: None,
            reason: "review-relationship-malformed".to_string(),
        };
        assert_eq!(
            render_review_diversity_warning("build-review-k2", &review_target, &uncheckable),
            "grove: review target diversity warning (review=build-review-k2, producer=unknown, uncheckable(reason=review-relationship-malformed), producer-target=unavailable, review-target=claude/default(claude)); applies only if factual pick is review=build-review-k2; discard otherwise; launch continues"
        );

        let explicit = LaunchTarget {
            harness: "claude".to_string(),
            model: Some("default(claude)\nnext-line".to_string()),
        };
        let rendered = render_launch_target(&explicit);
        assert_eq!(rendered, "claude/\"default(claude)\\nnext-line\"");
        assert!(!rendered.contains('\n'));
        assert_ne!(rendered, render_launch_target(&review_target));
    }

    #[test]
    fn a_receipt_must_carry_model_even_when_its_value_is_null() {
        let error = TaskRelationships::parse(
            "**Reviews:** build-k1\n\
             **Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\"}\n",
        )
        .unwrap_err();

        assert!(
            format!("{error:#}").contains("missing field `model`"),
            "unexpected parse error: {error:#}"
        );
    }

    #[test]
    fn receipt_wire_is_extensible_and_legacy_identity_is_all_or_nothing() {
        let modern = TaskRelationships::parse(
            "**Reviews:** build-k1\n\
             **Producer launch:** {\"producer\":\"build-k1\",\"session\":\"finish-k7\",\"generation\":\"k9\",\"harness\":\"claude\",\"model\":\"opus\",\"future\":true}\n",
        )
        .unwrap()
        .producer_launch
        .unwrap();
        assert_eq!(modern.session.as_deref(), Some("finish-k7"));
        assert_eq!(modern.generation.as_deref(), Some("k9"));

        let legacy = TaskRelationships::parse(
            "**Reviews:** build-k1\n\
             **Producer launch:** {\"producer\":\"build-k1\",\"harness\":\"claude\",\"model\":null}\n",
        )
        .unwrap()
        .producer_launch
        .unwrap();
        assert_eq!((legacy.session, legacy.generation), (None, None));

        for invalid in [
            r#"{"producer":"build-k1","session":"finish-k7","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":null,"generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":null,"harness":"claude","model":"opus"}"#,
        ] {
            let text = format!("**Reviews:** build-k1\n**Producer launch:** {invalid}\n");
            assert!(
                TaskRelationships::parse(&text).is_err(),
                "accepted {invalid}"
            );
        }
    }

    #[test]
    fn review_evidence_wire_requires_nullable_producer_and_rejects_unknown_fields() {
        let explicit_null =
            r#"{"status":"uncheckable","producer":null,"reason":"receipt-missing"}"#;
        assert!(serde_json::from_str::<ReviewEvidence>(explicit_null).is_ok());
        assert!(serde_json::from_str::<ReviewEvidence>(
            r#"{"status":"uncheckable","reason":"receipt-missing"}"#
        )
        .is_err());
        assert!(serde_json::from_str::<ReviewEvidence>(
            r#"{"status":"uncheckable","producer":null,"reason":"receipt-missing","future":true}"#
        )
        .is_err());
    }

    #[test]
    fn receipt_wire_rejects_invalid_known_values_and_old_strict_readers_reject_new_fields() {
        for invalid in [
            r#"{"session":"finish-k7","generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":"claude"}"#,
            r#"{"producer":"bad handle","session":"finish-k7","generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"bad handle","generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k0","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":"unknown","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":"claude","model":""}"#,
            r#"{"producer":1,"session":"finish-k7","generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":7,"generation":"k9","harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":9,"harness":"claude","model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":7,"model":"opus"}"#,
            r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":"claude","model":7}"#,
        ] {
            let text = format!("**Reviews:** build-k1\n**Producer launch:** {invalid}\n");
            assert!(
                TaskRelationships::parse(&text).is_err(),
                "accepted {invalid}"
            );
        }

        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        #[allow(dead_code)]
        struct LegacyStrictReceipt {
            producer: String,
            harness: String,
            #[serde(deserialize_with = "deserialize_nullable_model")]
            model: Option<String>,
        }
        let new_receipt = r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":"claude","model":"opus"}"#;
        assert!(serde_json::from_str::<LegacyStrictReceipt>(new_receipt).is_err());
    }

    #[test]
    fn replacement_is_adjacent_and_unconditional() {
        let receipt = ProducerLaunchReceipt {
            producer: "build-k1".to_string(),
            session: Some("build-k1".to_string()),
            generation: Some("k1".to_string()),
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
                    session: Some("build-k1".to_string()),
                    generation: Some("k1".to_string()),
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

    #[test]
    fn materialisation_re_reads_the_review_after_done() {
        let tmp = TempDir::new().unwrap();
        let review = tmp.path().join("02-build-review-k2.md");
        fs::write(&review, "**Reviews:** build-k1\noriginal\n").unwrap();
        let prepared = PreparedReceipt::new(
            review.clone(),
            ProducerLaunchReceipt {
                producer: "build-k1".to_string(),
                session: Some("finish-k7".to_string()),
                generation: Some("k9".to_string()),
                target: LaunchTarget {
                    harness: "claude".to_string(),
                    model: Some("opus".to_string()),
                },
            },
        )
        .unwrap();
        fs::write(&review, "**Reviews:** build-k1\noriginal\nlate edit\n").unwrap();

        assert!(ReceiptPlan::Ready(prepared)
            .materialization_diagnostic()
            .is_none());

        let rendered = fs::read_to_string(review).unwrap();
        assert!(rendered.contains("late edit\n"));
        assert!(rendered.contains("\"session\":\"finish-k7\""));
    }
}

//! Stable relationships carried by Grove task files.
//!
//! The interface parses the `Reviews` / `Integrates` / producer-launch lines a
//! task may declare. Callers never scan siblings or edit task-file metadata.
//!
//! Launch policy is no longer part of this seam. The ambient session target,
//! the structured routing peek's review evidence, and the producer/review
//! target comparison that rendered diversity notices are removed: a review's
//! target is explicit configuration policy, not something Grove infers from a
//! task body (`docs/adr/grove-owns-escalated-review.md`). The
//! `**Producer launch:**` receipt survives here only as a *parsed* legacy
//! relationship, preserved byte-for-byte by retirement and pruning; nothing
//! writes one.

use crate::harness;
use crate::tree_id::validate_slug;
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::fs;
use std::path::Path;

const REVIEWS_MARKER: &str = "**Reviews:**";
const INTEGRATES_MARKER: &str = "**Integrates:**";
const PRODUCER_LAUNCH_MARKER: &str = "**Producer launch:**";

/// A harness plus the exact model selector a legacy receipt recorded. `None`
/// means that harness's own default; it is not a provider-independent model
/// identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LaunchTarget {
    pub harness: String,
    #[serde(deserialize_with = "deserialize_required_nullable")]
    pub model: Option<String>,
}

fn deserialize_required_nullable<'de, D>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer)
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
    #[serde(deserialize_with = "deserialize_required_nullable")]
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

/// All stable relationship metadata a task file may carry.
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

#[cfg(test)]
mod tests {
    use super::*;

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
            #[serde(deserialize_with = "deserialize_required_nullable")]
            model: Option<String>,
        }
        let new_receipt = r#"{"producer":"build-k1","session":"finish-k7","generation":"k9","harness":"claude","model":"opus"}"#;
        assert!(serde_json::from_str::<LegacyStrictReceipt>(new_receipt).is_err());
    }
}

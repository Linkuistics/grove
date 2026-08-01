use crate::leaf::Kind;
use crate::tree_access;
use crate::tree_grow::write_task_template;
use crate::tree_id::{next_keys, parse, sort_key, Entry, Outcome};
use crate::tree_read::{self, Resolution};
use crate::tree_rename::{
    capture_git_index_entry, prepare_promotion_index, rename_entry, restore_promotion_index,
    GitIndexEntry,
};
use anyhow::{anyhow, bail, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromotedItem {
    pub path: PathBuf,
    pub handle: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PromotionResult {
    pub node: PromotedItem,
    pub producer: PromotedItem,
    pub review: PromotedItem,
    pub integration: PromotedItem,
    pub changed: bool,
}

impl PromotionResult {
    pub fn paths(&self) -> [&Path; 4] {
        [
            &self.node.path,
            &self.producer.path,
            &self.review.path,
            &self.integration.path,
        ]
    }
}

/// Promote one picked plain producer into a brief-less review-chain node. This
/// is intentionally the module's only public operation: source resolution,
/// strict kind parsing, relationship checks, key allocation, transaction
/// recovery, VCS movement, rollback, and result reconstruction stay behind it.
pub fn promote(grove_root: &Path, producer_reference: &str) -> Result<PromotionResult> {
    let guard = tree_access::write_for_promotion(grove_root)?;
    let grove_root = guard.root();

    if let Some(pending) = tree_access::find_pending(grove_root)? {
        return recover_pending(&pending, producer_reference);
    }

    if let Some(key) = reference_key(producer_reference) {
        if let Some(completed) = completed_shape(grove_root, key)? {
            return Ok(completed);
        }
    }

    promote_new(grove_root, producer_reference)
}

fn promote_new(grove_root: &Path, producer_reference: &str) -> Result<PromotionResult> {
    let producer_path = resolve_producer(grove_root, producer_reference)?;
    let producer_path = producer_path
        .canonicalize()
        .with_context(|| format!("resolving producer {}", producer_path.display()))?;
    let producer_name = file_name(&producer_path)?;
    let (position, producer_slug, producer_key) = live_leaf_identity(&producer_name)?;
    let producer_handle = handle(&producer_slug, producer_key);

    let picked = tree_read::pick_unlocked(grove_root)?
        .ok_or_else(|| anyhow!("no live leaves; there is no picked producer to promote"))?
        .canonicalize()
        .context("resolving the currently picked leaf")?;
    if picked != producer_path {
        bail!(
            "only the currently picked producer can be promoted: picked {}, got {}",
            picked.display(),
            producer_path.display()
        );
    }

    let producer_kind = read_strict_producer_kind(&producer_path)?;
    let (review_kind, integration_kind) = producer_kind.review_steps_or_refuse()?;
    let parent = producer_path
        .parent()
        .context("producer has no parent directory")?;

    let existing_reviews = declarations(parent, "Reviews", &producer_handle)?;
    if !existing_reviews.is_empty() {
        bail!(
            "producer {producer_handle} already has scheduled review work: {}",
            display_paths(&existing_reviews)
        );
    }
    if parent != grove_root && !parent.join("BRIEF.md").is_file() {
        bail!(
            "producer {producer_handle} is already inside a brief-less composition-managed node: {}",
            parent.display()
        );
    }

    let keys = next_keys(collect_tree_names(grove_root)?, 3)
        .context("allocating promotion keys (nothing was changed)")?;
    let node_key = keys[0];
    let review_key = keys[1];
    let integration_key = keys[2];
    let node_slug = format!("{producer_slug}-chain");
    let final_node_name = Entry::Node {
        position,
        slug: node_slug.clone(),
        key: node_key,
    }
    .name();
    let final_node = parent.join(&final_node_name);
    if final_node.exists() {
        bail!(
            "promotion destination already exists: {} (nothing was changed)",
            final_node.display()
        );
    }
    let transaction_name = format!("{}{}", tree_access::promoting_prefix(), final_node_name);
    let transaction = parent.join(&transaction_name);
    if transaction.exists() {
        bail!(
            "promotion transaction already exists: {}",
            transaction.display()
        );
    }

    let producer_child_name = Entry::Leaf {
        position: 1,
        slug: producer_slug.clone(),
        key: producer_key,
        outcome: Outcome::Live,
    }
    .name();

    let original_relative = PathBuf::from(&producer_name);
    let staging_relative = Path::new(&transaction_name).join(&producer_child_name);
    let final_relative = Path::new(&final_node_name).join(&producer_child_name);
    let git_entry = capture_git_index_entry(parent, &original_relative)?;

    fs::create_dir(&transaction)
        .with_context(|| format!("creating promotion transaction {}", transaction.display()))?;
    let review_slug = format!("{producer_slug}-review");
    let review_handle = handle(&review_slug, review_key);
    let review_child_name = Entry::Leaf {
        position: 2,
        slug: review_slug.clone(),
        key: review_key,
        outcome: Outcome::Live,
    }
    .name();
    let integration_slug = format!("{producer_slug}-integrate");
    let integration_handle = handle(&integration_slug, integration_key);
    let integration_child_name = Entry::Leaf {
        position: 3,
        slug: integration_slug.clone(),
        key: integration_key,
        outcome: Outcome::Live,
    }
    .name();

    let prepared = (|| -> Result<()> {
        promotion_test_checkpoint("after-transaction-created")?;
        write_generated_steps(
            &transaction,
            &producer_handle,
            &review_slug,
            review_key,
            review_kind,
            &review_handle,
            &integration_slug,
            integration_key,
            integration_kind,
        )?;
        promotion_test_checkpoint("after-generated-steps")?;
        rename_entry(parent, &producer_name, &staging_relative)?;
        promotion_test_checkpoint("after-producer-move")?;
        if let Some(entry) = &git_entry {
            prepare_promotion_index(parent, entry, &staging_relative, &final_relative)?;
        }
        promotion_test_checkpoint("after-index-prepare")?;
        fs::rename(&transaction, &final_node).with_context(|| {
            format!(
                "landing promotion transaction {} -> {}",
                transaction.display(),
                final_node.display()
            )
        })?;
        Ok(())
    })();

    if let Err(error) = prepared {
        return Err(PromotionRollback {
            parent,
            original_name: &producer_name,
            producer_child_name: &producer_child_name,
            transaction_name: &transaction_name,
            transaction: &transaction,
            git_entry: git_entry.as_ref(),
            original_relative: &original_relative,
            staging_relative: &staging_relative,
            final_relative: &final_relative,
        }
        .run(error));
    }

    Ok(PromotionResult {
        node: PromotedItem {
            path: final_node.clone(),
            handle: handle(&node_slug, node_key),
        },
        producer: PromotedItem {
            path: final_node.join(producer_child_name),
            handle: producer_handle,
        },
        review: PromotedItem {
            path: final_node.join(review_child_name),
            handle: review_handle,
        },
        integration: PromotedItem {
            path: final_node.join(integration_child_name),
            handle: integration_handle,
        },
        changed: true,
    })
}

fn recover_pending(transaction: &Path, producer_reference: &str) -> Result<PromotionResult> {
    let transaction_name = file_name(transaction)?;
    let final_name = transaction_name
        .strip_prefix(tree_access::promoting_prefix())
        .context("invalid promotion transaction name")?;
    let (position, node_slug, node_key) = match parse(final_name) {
        Some(Entry::Node {
            position,
            slug,
            key,
        }) => (position, slug, key),
        _ => bail!(
            "invalid promotion transaction path: {}",
            transaction.display()
        ),
    };
    let producer_slug = node_slug
        .strip_suffix("-chain")
        .context("promotion node slug has no -chain suffix")?
        .to_string();
    let parent = transaction.parent().context("transaction has no parent")?;

    let requested_key = reference_key(producer_reference);
    let staged = live_leaves(transaction)?
        .into_iter()
        .filter(|(_, slug, _, _)| slug == &producer_slug)
        .collect::<Vec<_>>();
    let outside = live_leaves(parent)?
        .into_iter()
        .filter(|(_, slug, _, path)| {
            slug == &producer_slug
                && requested_key.map_or(true, |key| {
                    parse(&file_name(path).unwrap_or_default()).and_then(|entry| entry.key())
                        == Some(key)
                })
        })
        .collect::<Vec<_>>();

    let (source_path, producer_key, already_staged) = match (staged.as_slice(), outside.as_slice()) {
        ([(.., key, path)], []) => (path.clone(), *key, true),
        ([], [(.., key, path)]) => (path.clone(), *key, false),
        _ => bail!(
            "cannot recover {}: expected exactly one `{producer_slug}` producer, found staged={} outside={}. Inspect the named transaction; no other tree operation will run until it is recovered",
            transaction.display(),
            staged.len(),
            outside.len()
        ),
    };
    let producer_handle = handle(&producer_slug, producer_key);
    if let Some(key) = requested_key {
        if key != producer_key {
            bail!(
                "pending transaction {} belongs to {producer_handle}, not the requested key k{key}",
                transaction.display()
            );
        }
    }
    let producer_kind = read_strict_producer_kind(&source_path)?;
    let (review_kind, integration_kind) = producer_kind.review_steps_or_refuse()?;
    let review_key = node_key
        .checked_add(1)
        .context("promotion review key overflow")?;
    let integration_key = node_key
        .checked_add(2)
        .context("promotion integration key overflow")?;
    let review_slug = format!("{producer_slug}-review");
    let review_handle = handle(&review_slug, review_key);
    let integration_slug = format!("{producer_slug}-integrate");
    let integration_handle = handle(&integration_slug, integration_key);
    write_generated_steps(
        transaction,
        &producer_handle,
        &review_slug,
        review_key,
        review_kind,
        &review_handle,
        &integration_slug,
        integration_key,
        integration_kind,
    )?;

    let producer_child_name = Entry::Leaf {
        position: 1,
        slug: producer_slug.clone(),
        key: producer_key,
        outcome: Outcome::Live,
    }
    .name();
    let original_name = Entry::Leaf {
        position,
        slug: producer_slug.clone(),
        key: producer_key,
        outcome: Outcome::Live,
    }
    .name();
    let original_relative = PathBuf::from(&original_name);
    let staging_relative = Path::new(&transaction_name).join(&producer_child_name);
    let final_relative = Path::new(final_name).join(&producer_child_name);
    let git_entry = if already_staged {
        capture_git_index_entry(parent, &staging_relative)?
    } else {
        capture_git_index_entry(parent, &original_relative)?
    };
    if !already_staged {
        let source_name = file_name(&source_path)?;
        rename_entry(parent, source_name, &staging_relative)?;
    }
    if let Some(entry) = &git_entry {
        prepare_promotion_index(parent, entry, &staging_relative, &final_relative)?;
    }

    let final_node = parent.join(final_name);
    if final_node.exists() {
        bail!(
            "cannot recover onto existing destination: {}",
            final_node.display()
        );
    }
    fs::rename(transaction, &final_node).with_context(|| {
        format!(
            "landing recovered promotion {} -> {}",
            transaction.display(),
            final_node.display()
        )
    })?;

    Ok(PromotionResult {
        node: PromotedItem {
            path: final_node.clone(),
            handle: handle(&node_slug, node_key),
        },
        producer: PromotedItem {
            path: final_node.join(&producer_child_name),
            handle: producer_handle,
        },
        review: PromotedItem {
            path: final_node.join(
                Entry::Leaf {
                    position: 2,
                    slug: review_slug,
                    key: review_key,
                    outcome: Outcome::Live,
                }
                .name(),
            ),
            handle: review_handle,
        },
        integration: PromotedItem {
            path: final_node.join(
                Entry::Leaf {
                    position: 3,
                    slug: integration_slug,
                    key: integration_key,
                    outcome: Outcome::Live,
                }
                .name(),
            ),
            handle: integration_handle,
        },
        changed: true,
    })
}

#[allow(clippy::too_many_arguments)]
fn write_generated_steps(
    transaction: &Path,
    producer_handle: &str,
    review_slug: &str,
    review_key: u32,
    review_kind: Kind,
    review_handle: &str,
    integration_slug: &str,
    integration_key: u32,
    integration_kind: Kind,
) -> Result<()> {
    let review_path = transaction.join(
        Entry::Leaf {
            position: 2,
            slug: review_slug.to_string(),
            key: review_key,
            outcome: Outcome::Live,
        }
        .name(),
    );
    write_task_template(
        &review_path,
        review_slug,
        review_key,
        review_kind,
        None,
        &[format!("**Reviews:** {producer_handle}")],
        Some(&format!(
            "Adversarially review `{producer_handle}` and record concrete findings for its integration step."
        )),
    )?;

    let integration_path = transaction.join(
        Entry::Leaf {
            position: 3,
            slug: integration_slug.to_string(),
            key: integration_key,
            outcome: Outcome::Live,
        }
        .name(),
    );
    write_task_template(
        &integration_path,
        integration_slug,
        integration_key,
        integration_kind,
        None,
        &[format!("**Integrates:** {review_handle}")],
        Some(&format!(
            "Apply the verified findings from `{review_handle}` while preserving the reviewed artifact's contract."
        )),
    )?;
    Ok(())
}

struct PromotionRollback<'a> {
    parent: &'a Path,
    original_name: &'a str,
    producer_child_name: &'a str,
    transaction_name: &'a str,
    transaction: &'a Path,
    git_entry: Option<&'a GitIndexEntry>,
    original_relative: &'a Path,
    staging_relative: &'a Path,
    final_relative: &'a Path,
}

impl PromotionRollback<'_> {
    fn run(self, cause: anyhow::Error) -> anyhow::Error {
        if let Some(entry) = self.git_entry {
            if let Err(error) = restore_promotion_index(
                self.parent,
                entry,
                self.original_relative,
                self.staging_relative,
                self.final_relative,
            ) {
                return cause.context(format!(
                    "promotion failed and rollback could not restore the Git index: {error}. Transaction remains at {}",
                    self.transaction.display()
                ));
            }
        }
        let staged = self.transaction.join(self.producer_child_name);
        if staged.exists() && !self.parent.join(self.original_name).exists() {
            if let Err(error) = rename_entry(
                self.parent,
                Path::new(self.transaction_name).join(self.producer_child_name),
                self.original_name,
            ) {
                return cause.context(format!(
                    "promotion failed and rollback could not restore the producer: {error}. Transaction remains at {}",
                    self.transaction.display()
                ));
            }
        }
        if self.transaction.exists() {
            if let Err(error) = fs::remove_dir_all(self.transaction) {
                return cause.context(format!(
                    "promotion failed and rollback could not remove {}: {error}",
                    self.transaction.display()
                ));
            }
        }
        cause.context(
            "promotion failed; the original producer was restored and the transaction rolled back",
        )
    }
}

fn completed_shape(grove_root: &Path, producer_key: u32) -> Result<Option<PromotionResult>> {
    let producer_path = match tree_read::resolve_unlocked(grove_root, &producer_key.to_string())? {
        Resolution::Found { path, .. } if path.is_file() => path,
        _ => return Ok(None),
    };
    let producer_name = file_name(&producer_path)?;
    let (_, producer_slug, key) = leaf_identity(&producer_name)?;
    if key != producer_key {
        return Ok(None);
    }
    let producer_handle = handle(&producer_slug, key);
    let parent = producer_path.parent().context("producer has no parent")?;
    if parent == grove_root || parent.join("BRIEF.md").is_file() {
        return Ok(None);
    }
    let parent_name = file_name(parent)?;
    let (node_slug, node_key) = match parse(&parent_name) {
        Some(Entry::Node { slug, key, .. }) => (slug, key),
        _ => return Ok(None),
    };
    // Promotion allocates the containing node *after* the producer already
    // exists, so its fresh key is necessarily greater. A proactively created
    // chain has the opposite order (node first, then producer) and is scheduled
    // review to refuse, not an idempotent completion of this operation.
    if node_key <= producer_key {
        return Ok(None);
    }
    let reviews = declarations(parent, "Reviews", &producer_handle)?;
    if reviews.len() != 1 {
        return Ok(None);
    }
    let review_path = reviews[0].clone();
    let review_name = file_name(&review_path)?;
    let (_, review_slug, review_key) = leaf_identity(&review_name)?;
    let review_handle = handle(&review_slug, review_key);
    let integrations = declarations(parent, "Integrates", &review_handle)?;
    if integrations.len() != 1 {
        return Ok(None);
    }
    let integration_path = integrations[0].clone();
    let integration_name = file_name(&integration_path)?;
    let (_, integration_slug, integration_key) = leaf_identity(&integration_name)?;

    Ok(Some(PromotionResult {
        node: PromotedItem {
            path: parent.to_path_buf(),
            handle: handle(&node_slug, node_key),
        },
        producer: PromotedItem {
            path: producer_path,
            handle: producer_handle,
        },
        review: PromotedItem {
            path: review_path,
            handle: review_handle,
        },
        integration: PromotedItem {
            path: integration_path,
            handle: handle(&integration_slug, integration_key),
        },
        changed: false,
    }))
}

fn resolve_producer(grove_root: &Path, reference: &str) -> Result<PathBuf> {
    let path = Path::new(reference);
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        grove_root.join(path)
    };
    if candidate.exists() {
        return Ok(candidate);
    }
    match tree_read::resolve_unlocked(grove_root, reference)? {
        Resolution::Found { path, .. } => Ok(path),
        Resolution::NotFound => bail!("producer reference not found: {reference}"),
        Resolution::Ambiguous(matches) => bail!(
            "producer reference is ambiguous: {reference} ({})",
            display_paths(
                &matches
                    .into_iter()
                    .map(|item| item.path)
                    .collect::<Vec<_>>()
            )
        ),
    }
}

fn read_strict_producer_kind(path: &Path) -> Result<Kind> {
    let body = fs::read_to_string(path)
        .with_context(|| format!("reading producer kind from {}", path.display()))?;
    let token = body.lines().find_map(|line| {
        line.trim_start()
            .strip_prefix("**Kind:**")
            .map(str::trim)
            .and_then(|rest| rest.split_whitespace().next())
    });
    let token = token.with_context(|| {
        format!(
            "producer {} must declare a non-empty **Kind:** line",
            path.display()
        )
    })?;
    Kind::parse_read(token).with_context(|| {
        format!(
            "producer {} declares unknown kind {token:?}; promotion reads kinds strictly",
            path.display()
        )
    })
}

fn declarations(directory: &Path, field: &str, value: &str) -> Result<Vec<PathBuf>> {
    let marker = format!("**{field}:**");
    let mut matches = Vec::new();
    for entry in sorted_entries(directory)? {
        if !entry.file_type()?.is_file() {
            continue;
        }
        let path = entry.path();
        let body = match fs::read_to_string(&path) {
            Ok(body) => body,
            Err(_) => continue,
        };
        if body.lines().any(|line| {
            line.trim_start()
                .strip_prefix(&marker)
                .is_some_and(|rest| rest.trim() == value)
        }) {
            matches.push(path);
        }
    }
    Ok(matches)
}

fn live_leaves(directory: &Path) -> Result<Vec<(u32, String, u32, PathBuf)>> {
    let mut leaves = Vec::new();
    for entry in sorted_entries(directory)? {
        if !entry.file_type()?.is_file() {
            continue;
        }
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        if let Some(Entry::Leaf {
            outcome: Outcome::Live,
            position,
            slug,
            key,
        }) = parse(name)
        {
            leaves.push((position, slug, key, entry.path()));
        }
    }
    Ok(leaves)
}

fn collect_tree_names(grove_root: &Path) -> Result<Vec<String>> {
    let mut names = Vec::new();
    collect_tree_names_into(grove_root, &mut names)?;
    Ok(names)
}

fn collect_tree_names_into(directory: &Path, names: &mut Vec<String>) -> Result<()> {
    for entry in sorted_entries(directory)? {
        let name = entry.file_name();
        let Some(name) = name.to_str() else {
            continue;
        };
        let Some(parsed) = parse(name) else {
            continue;
        };
        names.push(name.to_string());
        if matches!(parsed, Entry::Node { .. }) && entry.file_type()?.is_dir() {
            collect_tree_names_into(&entry.path(), names)?;
        }
    }
    Ok(())
}

fn sorted_entries(directory: &Path) -> Result<Vec<fs::DirEntry>> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .with_context(|| format!("reading {}", directory.display()))?
        .collect::<std::io::Result<Vec<_>>>()?;
    entries.sort_by_key(|entry| sort_key(&entry.file_name().to_string_lossy()));
    Ok(entries)
}

fn live_leaf_identity(name: &str) -> Result<(u32, String, u32)> {
    match parse(name) {
        Some(Entry::Leaf {
            outcome: Outcome::Live,
            position,
            slug,
            key,
        }) => Ok((position, slug, key)),
        Some(Entry::Leaf {
            outcome: Outcome::Done,
            ..
        }) => bail!("cannot promote a retired (DONE) producer: {name}"),
        Some(Entry::Leaf {
            outcome: Outcome::Abandoned,
            ..
        }) => bail!("cannot promote an abandoned (ABANDONED) producer: {name}"),
        Some(Entry::Node { .. }) => bail!("cannot promote a node: {name}"),
        Some(Entry::Brief) => bail!("cannot promote a brief"),
        None => bail!("not a v2 producer leaf: {name}"),
    }
}

fn leaf_identity(name: &str) -> Result<(u32, String, u32)> {
    match parse(name) {
        Some(Entry::Leaf {
            position,
            slug,
            key,
            ..
        }) => Ok((position, slug, key)),
        _ => bail!("not a v2 leaf: {name}"),
    }
}

fn reference_key(reference: &str) -> Option<u32> {
    let name = Path::new(reference)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or(reference);
    if name.starts_with(tree_access::promoting_prefix()) {
        return None;
    }
    if let Some(entry) = parse(name) {
        return entry.key();
    }
    let bare = reference
        .strip_prefix('[')
        .and_then(|rest| rest.split_once(']').map(|(key, _)| key))
        .unwrap_or(reference);
    if let Ok(key) = bare.parse() {
        return Some(key);
    }
    reference
        .rsplit_once("-k")
        .and_then(|(_, key)| key.parse().ok())
}

fn file_name(path: &Path) -> Result<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(str::to_string)
        .with_context(|| format!("path has no UTF-8 filename: {}", path.display()))
}

fn handle(slug: &str, key: u32) -> String {
    format!("{slug}-k{key}")
}

fn display_paths(paths: &[PathBuf]) -> String {
    paths
        .iter()
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>()
        .join(", ")
}

/// Deterministic subprocess barriers/failures for the process-interruption
/// contract. Inert unless a test sets the exact checkpoint plus a barrier or
/// failure variable; ordinary Grove sessions never enter the loop.
fn promotion_test_checkpoint(name: &str) -> Result<()> {
    if std::env::var("GROVE_TEST_PROMOTION_FAIL_AT").as_deref() == Ok(name) {
        bail!("injected promotion failure at {name}");
    }
    if std::env::var("GROVE_TEST_PROMOTION_PAUSE_AT").as_deref() != Ok(name) {
        return Ok(());
    }
    let barrier = std::env::var_os("GROVE_TEST_PROMOTION_BARRIER")
        .context("GROVE_TEST_PROMOTION_PAUSE_AT needs GROVE_TEST_PROMOTION_BARRIER")?;
    let barrier = PathBuf::from(barrier);
    fs::write(&barrier, name)
        .with_context(|| format!("writing promotion test barrier {}", barrier.display()))?;
    while barrier.exists() {
        thread::sleep(Duration::from_millis(10));
    }
    Ok(())
}

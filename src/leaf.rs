// The `grove-llm leaf-add` and `grove-llm leaf-insert` verbs. Promote the
// deterministic "add a leaf" / "insert with renumber" mechanics from the
// Decompose step of `content/SKILL.md` into CLI verbs. Per ADR-0006 these
// verbs live on the LLM-driven binary.
//
// The two verbs share prefix arithmetic, sibling enumeration, and the leaf
// template; `insert` adds the renumber+header-rewrite+cross-ref-surface
// machinery on top. Both operate on the working tree only — no `git commit`.

use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Work,
    Planning,
}

impl Kind {
    fn label(self) -> &'static str {
        match self {
            Kind::Work => "work",
            Kind::Planning => "planning",
        }
    }

    pub fn parse(s: &str) -> Result<Kind> {
        match s {
            "work" => Ok(Kind::Work),
            "planning" => Ok(Kind::Planning),
            other => bail!("--kind must be `work` or `planning`, got {:?}", other),
        }
    }
}

/// A sibling entry in the target node directory — either a leaf `.md` file
/// or a decomposed sub-node directory.
struct Sibling {
    prefix: u32,
    /// The full on-disk name (e.g. `040-foo.md` for a leaf, `040-foo` for a
    /// directory).
    name: String,
    is_dir: bool,
}

/// A rename produced by `insert`: an existing sibling shifted from
/// `old_prefix` to `new_prefix`.
#[derive(Clone, Debug)]
pub struct Renumber {
    pub old_prefix: u32,
    pub new_prefix: u32,
    pub old_name: String,
    pub new_name: String,
    pub is_dir: bool,
}

/// Append a new leaf at the next free numeric prefix in `node_dir` (or at
/// `explicit_prefix` if supplied and free). Returns the absolute path of
/// the new leaf.
pub fn add(
    node_dir: &Path,
    slug: &str,
    explicit_prefix: Option<u32>,
    kind: Kind,
) -> Result<PathBuf> {
    validate_slug(slug)?;
    require_dir(node_dir)?;

    let siblings = read_siblings(node_dir)?;
    let prefix = match explicit_prefix {
        Some(p) => {
            validate_prefix(p)?;
            if siblings.iter().any(|s| s.prefix == p) {
                bail!(
                    "prefix {:03} is already taken in {}",
                    p,
                    node_dir.display()
                );
            }
            p
        }
        None => next_free_prefix(&siblings)?,
    };

    let filename = format!("{:03}-{}.md", prefix, slug);
    let path = node_dir.join(&filename);
    if path.exists() {
        bail!("destination already exists: {}", path.display());
    }
    write_template(&path, prefix, slug, kind)?;
    Ok(path)
}

/// Insert a new leaf at `prefix`, shifting every sibling at or after
/// `prefix` up by 10. Renames affected siblings via `git mv`, rewrites the
/// first-line header of each renamed file (or its `BRIEF.md`, for directory
/// siblings), and returns the renumber log alongside the new leaf's path.
///
/// If no siblings are at or after `prefix`, `insert` degenerates to `add`
/// at that explicit prefix; the returned renumber log is empty.
pub fn insert(
    node_dir: &Path,
    prefix: u32,
    slug: &str,
    kind: Kind,
) -> Result<(PathBuf, Vec<Renumber>)> {
    validate_slug(slug)?;
    validate_prefix(prefix)?;
    require_dir(node_dir)?;

    let siblings = read_siblings(node_dir)?;
    let mut to_shift: Vec<&Sibling> = siblings.iter().filter(|s| s.prefix >= prefix).collect();
    // Highest-first so the intermediate filesystem state stays collision-free
    // (we shift NNN+10 into a slot that the next-higher sibling has just left).
    to_shift.sort_by_key(|s| std::cmp::Reverse(s.prefix));

    // Pre-flight: refuse to start the renumber if any shifted prefix would
    // overflow 990. This keeps the operation all-or-nothing at the verb level
    // (a partial shift after a mid-loop overflow would leave the tree in a
    // weird intermediate state).
    for s in &to_shift {
        if s.prefix + 10 > 990 {
            bail!(
                "renumber would overflow: sibling {} cannot shift to {:03} (max prefix is 990)",
                s.name,
                s.prefix + 10
            );
        }
    }

    let mut renumbers: Vec<Renumber> = Vec::with_capacity(to_shift.len());
    for s in &to_shift {
        let new_name = bump_prefix_in_name(&s.name, 10);
        git_mv(node_dir, &s.name, &new_name)?;
        let dst = node_dir.join(&new_name);
        rewrite_header(&dst, s.prefix + 10, s.is_dir)?;
        renumbers.push(Renumber {
            old_prefix: s.prefix,
            new_prefix: s.prefix + 10,
            old_name: s.name.clone(),
            new_name,
            is_dir: s.is_dir,
        });
    }

    let filename = format!("{:03}-{}.md", prefix, slug);
    let path = node_dir.join(&filename);
    if path.exists() {
        bail!(
            "destination already exists after renumber: {} (renumber log: {:?})",
            path.display(),
            renumbers
        );
    }
    write_template(&path, prefix, slug, kind)?;

    Ok((path, renumbers))
}

/// Scan the renumbered files and every `.md` sibling in `node_dir` for
/// `\bNNN-` tokens whose `NNN` matches an old prefix from `renumbers`. Emit
/// one `file:line: NNN- (context)` line per match to `out`. The operator
/// reviews and decides per occurrence — the verb deliberately does not
/// auto-rewrite (cross-references may be historical references, not live
/// pointers).
pub fn surface_cross_refs(
    node_dir: &Path,
    renumbers: &[Renumber],
    out: &mut impl std::io::Write,
) -> Result<()> {
    if renumbers.is_empty() {
        return Ok(());
    }
    let old_prefixes: std::collections::HashSet<u32> =
        renumbers.iter().map(|r| r.old_prefix).collect();

    let mut targets: Vec<PathBuf> = Vec::new();
    // Renumbered files: the new path for files, the inside-BRIEF for dirs.
    for r in renumbers {
        let new_path = node_dir.join(&r.new_name);
        if r.is_dir {
            let brief = new_path.join("BRIEF.md");
            if brief.is_file() {
                targets.push(brief);
            }
        } else if new_path.is_file() {
            targets.push(new_path);
        }
    }
    // Every `.md` directly under `node_dir` — picks up `BRIEF.md`, untouched
    // sibling leaves, and the freshly-written new leaf so its body is also
    // scrubbed for stale references the LLM may have typed.
    for entry in std::fs::read_dir(node_dir).with_context(|| format!("reading {}", node_dir.display()))? {
        let entry = entry?;
        let p = entry.path();
        if p.is_file() && p.extension().and_then(|s| s.to_str()) == Some("md") {
            targets.push(p);
        }
    }
    targets.sort();
    targets.dedup();

    for target in &targets {
        let body = match std::fs::read_to_string(target) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for (lineno, line) in body.lines().enumerate() {
            for prefix in find_prefix_tokens(line) {
                if old_prefixes.contains(&prefix) {
                    writeln!(
                        out,
                        "{}:{}: {:03}- ({})",
                        target.display(),
                        lineno + 1,
                        prefix,
                        line.trim()
                    )
                    .ok();
                }
            }
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// helpers

fn require_dir(p: &Path) -> Result<()> {
    if !p.is_dir() {
        bail!("node directory not found: {}", p.display());
    }
    Ok(())
}

fn validate_slug(slug: &str) -> Result<()> {
    if slug.is_empty() {
        bail!("slug must not be empty");
    }
    if slug.starts_with('-') || slug.ends_with('-') {
        bail!("slug must not start or end with a dash: {:?}", slug);
    }
    for c in slug.chars() {
        if !(c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
            bail!(
                "slug must contain only lowercase ASCII letters, digits, and dashes: {:?}",
                slug
            );
        }
    }
    Ok(())
}

fn validate_prefix(p: u32) -> Result<()> {
    if p == 0 || p > 990 || p % 10 != 0 {
        bail!(
            "prefix must be a multiple of 10 between 010 and 990 (got {:03})",
            p
        );
    }
    Ok(())
}

fn read_siblings(node_dir: &Path) -> Result<Vec<Sibling>> {
    let mut v = Vec::new();
    for entry in std::fs::read_dir(node_dir).with_context(|| format!("reading {}", node_dir.display()))? {
        let entry = entry?;
        let name = match entry.file_name().to_str() {
            Some(s) => s.to_string(),
            None => continue,
        };
        let ft = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };
        if ft.is_dir() {
            if name == "done" {
                continue;
            }
            if let Some((p, _)) = split_prefix(&name) {
                v.push(Sibling {
                    prefix: p,
                    name,
                    is_dir: true,
                });
            }
        } else if ft.is_file() {
            if name == "BRIEF.md" {
                continue;
            }
            if !name.ends_with(".md") {
                continue;
            }
            if let Some((p, _)) = split_prefix(&name) {
                v.push(Sibling {
                    prefix: p,
                    name,
                    is_dir: false,
                });
            }
        }
    }
    v.sort_by_key(|s| s.prefix);
    Ok(v)
}

/// Split `NNN-rest` (NNN exactly three ASCII digits) into `(NNN, rest)`.
fn split_prefix(name: &str) -> Option<(u32, &str)> {
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

fn next_free_prefix(siblings: &[Sibling]) -> Result<u32> {
    let next = match siblings.iter().map(|s| s.prefix).max() {
        Some(m) => m + 10,
        None => 10,
    };
    if next > 990 {
        bail!("no free prefix available (highest sibling is at 990)");
    }
    Ok(next)
}

fn bump_prefix_in_name(name: &str, delta: u32) -> String {
    let (p, rest) = split_prefix(name).expect("caller ensured the prefix is parseable");
    format!("{:03}-{}", p + delta, rest)
}

fn git_mv(node_dir: &Path, src: &str, dst: &str) -> Result<()> {
    let out = Command::new("git")
        .arg("-C")
        .arg(node_dir)
        .arg("mv")
        .arg(src)
        .arg(dst)
        .output()
        .with_context(|| format!("running git mv {} {}", src, dst))?;
    if !out.status.success() {
        bail!(
            "git mv {} -> {} failed: {}",
            src,
            dst,
            String::from_utf8_lossy(&out.stderr).trim()
        );
    }
    Ok(())
}

/// Rewrite the first-line `# NNN-...` header of a renamed leaf — or of its
/// `BRIEF.md`, for renamed directory siblings — so the new prefix matches
/// the new on-disk name. A first line that doesn't match the `# NNN-...`
/// shape is left alone (the verb is conservative — leaves with hand-edited
/// titles stay hand-edited).
fn rewrite_header(path: &Path, new_prefix: u32, is_dir: bool) -> Result<()> {
    let target: PathBuf = if is_dir {
        path.join("BRIEF.md")
    } else {
        path.to_path_buf()
    };
    if !target.is_file() {
        return Ok(());
    }
    let body =
        std::fs::read_to_string(&target).with_context(|| format!("reading {}", target.display()))?;
    let (first, rest) = match body.split_once('\n') {
        Some((f, r)) => (f, Some(r)),
        None => (body.as_str(), None),
    };
    let new_first = match bump_header_prefix(first, new_prefix) {
        Some(u) => u,
        None => return Ok(()),
    };
    let mut out = String::with_capacity(body.len());
    out.push_str(&new_first);
    if let Some(r) = rest {
        out.push('\n');
        out.push_str(r);
    }
    std::fs::write(&target, out.as_bytes())
        .with_context(|| format!("writing {}", target.display()))?;
    Ok(())
}

fn bump_header_prefix(line: &str, new_prefix: u32) -> Option<String> {
    let body = line.strip_prefix("# ")?;
    if body.len() < 4 {
        return None;
    }
    let (head, tail) = body.split_at(3);
    if !head.chars().all(|c| c.is_ascii_digit()) {
        return None;
    }
    if !tail.starts_with('-') {
        return None;
    }
    Some(format!("# {:03}{}", new_prefix, tail))
}

fn write_template(path: &Path, prefix: u32, slug: &str, kind: Kind) -> Result<()> {
    let body = format!(
        "# {prefix:03}-{slug}\n\n**Kind:** {kind}\n\n## Goal\n\n## Context\n\n## Done when\n\n## Notes\n",
        prefix = prefix,
        slug = slug,
        kind = kind.label(),
    );
    std::fs::write(path, body.as_bytes())
        .with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}

/// Find each three-digit numeric prefix in `line` that occurs in a
/// `\bNNN-` position (no preceding alphanumeric). Returns the numeric
/// values in order.
fn find_prefix_tokens(line: &str) -> Vec<u32> {
    let bytes = line.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 4 <= bytes.len() {
        if bytes[i..i + 3].iter().all(|b| b.is_ascii_digit()) && bytes[i + 3] == b'-' {
            let prev_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            if prev_ok {
                if let Ok(p) = std::str::from_utf8(&bytes[i..i + 3]).unwrap().parse::<u32>() {
                    out.push(p);
                    i += 4;
                    continue;
                }
            }
        }
        i += 1;
    }
    out
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
    fn bump_header_prefix_only_rewrites_NNN_dash_form() {
        assert_eq!(
            bump_header_prefix("# 050-foo", 60),
            Some("# 060-foo".to_string())
        );
        assert_eq!(
            bump_header_prefix("# 050-foo — brief", 60),
            Some("# 060-foo — brief".to_string())
        );
        assert_eq!(bump_header_prefix("# Some title", 60), None);
        assert_eq!(bump_header_prefix("Not a header", 60), None);
    }

    #[test]
    fn find_prefix_tokens_extracts_NNN_dash_starts() {
        assert_eq!(find_prefix_tokens("see 050-foo for details"), vec![50]);
        assert_eq!(
            find_prefix_tokens("040- and 050-bar"),
            vec![40, 50]
        );
        // Embedded digits (e.g. inside a year) should not match.
        assert_eq!(find_prefix_tokens("2026-05-28"), Vec::<u32>::new());
    }

    #[test]
    fn validate_slug_rejects_uppercase_and_punctuation() {
        assert!(validate_slug("good-slug").is_ok());
        assert!(validate_slug("good-1-2").is_ok());
        assert!(validate_slug("Bad").is_err());
        assert!(validate_slug("with space").is_err());
        assert!(validate_slug("-leading").is_err());
        assert!(validate_slug("trailing-").is_err());
        assert!(validate_slug("").is_err());
    }

    #[test]
    fn validate_prefix_requires_multiple_of_ten() {
        assert!(validate_prefix(10).is_ok());
        assert!(validate_prefix(990).is_ok());
        assert!(validate_prefix(0).is_err());
        assert!(validate_prefix(5).is_err());
        assert!(validate_prefix(45).is_err());
        assert!(validate_prefix(1000).is_err());
    }
}

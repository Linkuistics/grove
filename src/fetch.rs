use anyhow::{anyhow, Context, Result};
use serde::Deserialize;
use std::io::Read;

pub trait Fetcher {
    /// Return the highest semver-sorted `v*` tag.
    fn latest_version(&self) -> Result<String>;
    /// Download the tarball for `tag` and return its bytes.
    fn fetch_tarball(&self, tag: &str) -> Result<Vec<u8>>;
}

pub struct GithubFetcher {
    base_url: String,
}

impl GithubFetcher {
    pub fn new() -> Self {
        Self {
            base_url: "https://api.github.com".to_string(),
        }
    }

    /// For tests: point at a local mock server.
    pub fn with_base_url(base_url: String) -> Self {
        Self { base_url }
    }
}

impl Default for GithubFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Deserialize)]
struct Tag {
    name: String,
}

type SemverParts = (u64, u64, u64);

fn parse_semver(tag: &str) -> Option<SemverParts> {
    let stripped = tag.strip_prefix('v')?;
    let mut parts = stripped.split('.');
    let major: u64 = parts.next()?.parse().ok()?;
    let minor: u64 = parts.next()?.parse().ok()?;
    let patch: u64 = parts.next()?.parse().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((major, minor, patch))
}

impl Fetcher for GithubFetcher {
    fn latest_version(&self) -> Result<String> {
        let url = format!("{}/repos/Linkuistics/grove/tags?per_page=100", self.base_url);
        let tags: Vec<Tag> = ureq::get(&url)
            .set("User-Agent", "grove-cli")
            .call()
            .context("fetching tag list from GitHub")?
            .into_json()
            .context("parsing GitHub tag list")?;

        let mut versions: Vec<(SemverParts, String)> = tags
            .into_iter()
            .filter_map(|t| parse_semver(&t.name).map(|parts| (parts, t.name)))
            .collect();
        versions.sort_by(|a, b| b.0.cmp(&a.0)); // descending
        versions
            .into_iter()
            .next()
            .map(|(_, name)| name)
            .ok_or_else(|| anyhow!("no v* tags found on Linkuistics/grove"))
    }

    fn fetch_tarball(&self, tag: &str) -> Result<Vec<u8>> {
        // GitHub serves archive tarballs at
        // github.com/<owner>/<repo>/archive/refs/tags/<tag>.tar.gz
        // (not api.github.com). Translate base_url:
        // - production:  api.github.com  →  github.com
        // - tests:       keep base_url so the mock can serve both endpoints
        let archive_host = if self.base_url == "https://api.github.com" {
            "https://github.com".to_string()
        } else {
            self.base_url.clone()
        };
        let url = format!(
            "{}/Linkuistics/grove/archive/refs/tags/{}.tar.gz",
            archive_host, tag
        );
        let mut bytes = Vec::new();
        ureq::get(&url)
            .set("User-Agent", "grove-cli")
            .call()
            .context("fetching tarball from GitHub")?
            .into_reader()
            .read_to_end(&mut bytes)
            .context("reading tarball body")?;
        Ok(bytes)
    }
}

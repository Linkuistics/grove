use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn write(install_dir: &Path, harness: &str, version: &str) -> Result<()> {
    let date = today_iso();
    let content = format!(
        "# grove — materialised version\n\
         \n\
         A materialised copy of the grove skill for the `{harness}` harness.\n\
         \n\
         | | |\n\
         |---|---|\n\
         | version | `{version}` |\n\
         | materialised on | {date} |\n\
         | materialised into | `.{harness}/skills/grove/` |\n\
         \n\
         ## Updating\n\
         \n\
         ```\n\
         grove update --version <tag>\n\
         ```\n",
    );
    fs::write(install_dir.join("VERSION.md"), content)
        .with_context(|| format!("writing VERSION.md to {}", install_dir.display()))?;
    Ok(())
}

/// Read the `| version | `vX.Y.Z` |` row from a VERSION.md.
pub fn read_version(install_dir: &Path) -> Result<String> {
    let content = fs::read_to_string(install_dir.join("VERSION.md"))
        .with_context(|| format!("reading VERSION.md in {}", install_dir.display()))?;
    for line in content.lines() {
        let line = line.trim();
        if let Some(rest) = line.strip_prefix("| version | `") {
            if let Some(end) = rest.find('`') {
                return Ok(rest[..end].to_string());
            }
        }
    }
    anyhow::bail!("no version line found in VERSION.md");
}

/// Today's date as YYYY-MM-DD, computed from system time without pulling in chrono.
fn today_iso() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time before epoch")
        .as_secs();
    // Days since epoch.
    let days = (secs / 86400) as i64;
    let (y, m, d) = days_to_ymd(days);
    format!("{:04}-{:02}-{:02}", y, m, d)
}

/// Civil-from-days algorithm (Howard Hinnant): convert days since
/// 1970-01-01 (UTC) into (year, month, day).
fn days_to_ymd(days_since_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_epoch + 719468;
    let era = if z >= 0 { z } else { z - 146096 } / 146097;
    let doe = (z - era * 146097) as u64; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = (yoe as i64) + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5) + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y as i32, m as u32, d as u32)
}

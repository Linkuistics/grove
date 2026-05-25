use crate::harness::HARNESSES;
use crate::version_md;
use anyhow::Result;

pub fn run() -> Result<()> {
    println!("grove cli {}", env!("CARGO_PKG_VERSION"));
    let Ok(main_repo) = crate::repo::resolve(None) else {
        return Ok(()); // not in a git repo; just CLI version
    };
    for h in HARNESSES {
        let dest = h.install_path(&main_repo);
        if !dest.exists() {
            continue;
        }
        match version_md::read_version(&dest) {
            Ok(v) => println!("  {} → {}", h.name, v),
            Err(_) => println!("  {} → (VERSION.md missing or malformed)", h.name),
        }
    }
    Ok(())
}

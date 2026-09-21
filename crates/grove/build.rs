//! Generate the binary's private, offline plugin inventory from authoritative files.
use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

fn collect(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    println!("cargo:rerun-if-changed={}", path.display());
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let kind = entry.file_type()?;
        if kind.is_dir() {
            collect(&entry.path(), files)?;
        } else if kind.is_file() {
            println!("cargo:rerun-if-changed={}", entry.path().display());
            files.push(entry.path());
        }
    }
    Ok(())
}

fn eligible(body: &str) -> bool {
    let mut lines = body.lines();
    if lines.next() != Some("---") {
        return false;
    }
    for line in lines.take_while(|line| *line != "---") {
        if let Some(value) = line.strip_prefix("harnesses:") {
            let value = value.split('#').next().unwrap_or_default().trim();
            let Some(list) = value.strip_prefix('[').and_then(|s| s.strip_suffix(']')) else {
                panic!("harnesses must be a YAML flow list: {line}");
            };
            return list
                .split(',')
                .any(|item| matches!(item.trim().trim_matches(['\'', '"']), "any" | "codex"));
        }
    }
    false
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR")?).join("../../plugins");
    let root = root.canonicalize()?;
    let mut files = Vec::new();
    collect(&root, &mut files)?;
    files.sort();
    let mut output = String::from("const FILES: &[(&str, &[u8], u32)] = &[\n");
    let mut skills = Vec::new();
    let mut names = BTreeSet::new();
    for path in files {
        let relative = path
            .strip_prefix(&root)?
            .to_str()
            .ok_or("non-UTF8 plugin path")?;
        let mode = fs::metadata(&path)?.permissions().mode() & 0o111;
        output.push_str(&format!(
            "({:?}, include_bytes!({:?}), {mode}),\n",
            format!("plugins/{relative}"),
            path
        ));
        let parts: Vec<_> = relative.split('/').collect();
        if parts.len() == 4
            && parts[1] == "skills"
            && parts[3] == "SKILL.md"
            && eligible(&fs::read_to_string(&path)?)
        {
            if !names.insert(parts[2].to_owned()) {
                return Err(format!("duplicate Codex skill name: {}", parts[2]).into());
            }
            skills.push((
                parts[2].to_owned(),
                format!("plugins/{}/{}/{}", parts[0], parts[1], parts[2]),
            ));
        }
    }
    output.push_str("];\nconst SKILLS: &[(&str, &str)] = &[\n");
    skills.sort();
    for (name, path) in skills {
        output.push_str(&format!("({name:?}, {path:?}),\n"));
    }
    output.push_str("];\n");
    fs::write(
        PathBuf::from(std::env::var("OUT_DIR")?).join("plugin_inventory.rs"),
        output,
    )?;
    Ok(())
}

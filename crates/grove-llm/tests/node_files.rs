//! Node-file identity and whole-tree refusal through the agent-facing binary.
use assert_cmd::Command;
use std::fs;
use std::path::Path;
use std::process::Output;
use tempfile::TempDir;

mod support;

fn fixture() -> TempDir {
    let temp = TempDir::new().unwrap();
    support::init_jj_repo(temp.path());
    fs::create_dir(temp.path().join(".grove")).unwrap();
    fs::write(temp.path().join(".grove/_BRIEF.md"), "root brief\n").unwrap();
    fs::write(
        temp.path().join(".grove/01-impl--first-k1.md"),
        "custom heading\nbody\n",
    )
    .unwrap();
    temp
}

fn run(root: &Path, args: &[&str]) -> Output {
    Command::cargo_bin("grove-llm")
        .unwrap()
        .current_dir(root)
        .env("HOME", support::fixture_home())
        .env_remove("GROVE_SIGNAL_FILE")
        .args(args)
        .output()
        .unwrap()
}

fn success(root: &Path, args: &[&str]) -> String {
    let out = run(root, args);
    assert!(
        out.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap()
}

fn snapshot(root: &Path) -> Vec<(String, Vec<u8>)> {
    fn walk(root: &Path, dir: &Path, rows: &mut Vec<(String, Vec<u8>)>) {
        let mut entries: Vec<_> = fs::read_dir(dir).unwrap().map(Result::unwrap).collect();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let ty = entry.file_type().unwrap();
            let name = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if ty.is_dir() {
                rows.push((format!("{name}/"), vec![]));
                walk(root, &path, rows);
            } else if ty.is_symlink() {
                rows.push((
                    name,
                    fs::read_link(path)
                        .unwrap()
                        .as_os_str()
                        .as_encoded_bytes()
                        .to_vec(),
                ));
            } else {
                rows.push((name, fs::read(path).unwrap()));
            }
        }
    }
    let mut rows = vec![];
    walk(root, root, &mut rows);
    rows
}

#[test]
fn decomposition_titles_follow_node_files_across_renumber_and_retirement() {
    let temp = fixture();
    let repo = temp.path();
    let grove = repo.join(".grove");
    let output = success(
        repo,
        &["leaf-decompose", ".grove/01-impl--first-k1.md", "child"],
    );
    assert!(
        output.lines().next().unwrap().ends_with("/01-k1/_first.md"),
        "{output}"
    );
    assert_eq!(
        fs::read(grove.join("01-k1/_first.md")).unwrap(),
        b"custom heading\nbody\n"
    );
    for reference in ["1", "first-k1", "first"] {
        assert!(success(repo, &["resolve", reference])
            .trim()
            .ends_with("/01-k1"));
    }
    let chain = success(repo, &["brief-chain"]);
    assert_eq!(
        chain
            .lines()
            .map(|line| Path::new(line).file_name().unwrap().to_str().unwrap())
            .collect::<Vec<_>>(),
        vec!["_BRIEF.md", "_first.md"]
    );
    for path in [".grove/01-k1", ".grove/01-k1/_first.md", ".grove/_BRIEF.md"] {
        assert!(!run(repo, &["kind", path]).status.success());
    }
    success(
        repo,
        &["leaf-insert", "first-k1", "earlier", "--kind", "impl"],
    );
    assert!(success(repo, &["resolve", "first-k1"])
        .trim()
        .ends_with("/02-k1"));
    assert_eq!(
        fs::read(grove.join("02-k1/_first.md")).unwrap(),
        b"custom heading\nbody\n"
    );
    success(repo, &["leaf-retire", ".grove/02-k1/01-impl--child-k2.md"]);
    assert!(grove.join("02-k1/01-DONE-impl--child-k2.md").is_file());
    fs::rename(
        grove.join("02-k1/_first.md"),
        grove.join("02-k1/_renamed.md"),
    )
    .unwrap();
    assert!(success(repo, &["resolve", "first-k1"]).is_empty());
    for reference in ["1", "renamed", "renamed-k1"] {
        assert!(success(repo, &["resolve", reference])
            .trim()
            .ends_with("/02-k1"));
    }
    let output = run(repo, &["resolve", "renamed"]);
    assert!(
        output.stderr.is_empty(),
        "one node file must not create a second slug match"
    );
}

#[test]
fn ambiguous_node_titles_report_handles_without_reading_bodies() {
    let temp = fixture();
    for (dir, file) in [("02-k2", "_same.md"), ("03-k3", "_same.md")] {
        fs::create_dir(temp.path().join(".grove").join(dir)).unwrap();
        fs::write(
            temp.path().join(".grove").join(dir).join(file),
            "# misleading-k99\n",
        )
        .unwrap();
    }
    let out = run(temp.path(), &["resolve", "same"]);
    assert!(out.status.success());
    assert!(out.stdout.is_empty());
    let error = String::from_utf8(out.stderr).unwrap();
    for handle in ["same-k2", "same-k3"] {
        assert!(error.contains(handle), "{error}");
    }
    assert!(success(temp.path(), &["resolve", "misleading"]).is_empty());
}

#[test]
fn malformed_later_levels_refuse_every_verb_before_any_effect() {
    for bad in [
        "missing-root",
        "missing-node",
        "competing-root",
        "competing-node",
        "misplaced-root",
        "misplaced-node",
        "bad-underscore",
        "bad-digit",
        "file-as-dir",
        "dir-as-file",
        "node-file-dir",
        "node-file-symlink",
        "noncanonical-node",
    ] {
        let temp = fixture();
        let grove = temp.path().join(".grove");
        let node = grove.join("02-k2");
        fs::create_dir(&node).unwrap();
        fs::write(node.join("_later.md"), "later brief").unwrap();
        let offending = match bad {
            "missing-root" => {
                fs::remove_file(grove.join("_BRIEF.md")).unwrap();
                "_BRIEF.md"
            }
            "missing-node" => {
                fs::remove_file(node.join("_later.md")).unwrap();
                "02-k2"
            }
            "competing-root" => {
                fs::write(grove.join("_extra.md"), "").unwrap();
                "_extra.md"
            }
            "competing-node" => {
                fs::write(node.join("_extra.md"), "").unwrap();
                "_extra.md"
            }
            "misplaced-root" => {
                fs::rename(grove.join("_BRIEF.md"), grove.join("_title.md")).unwrap();
                "_title.md"
            }
            "misplaced-node" => {
                fs::rename(node.join("_later.md"), node.join("_BRIEF.md")).unwrap();
                "_BRIEF.md"
            }
            "bad-underscore" => {
                fs::write(node.join("_Broken.md"), "").unwrap();
                "_Broken.md"
            }
            "bad-digit" => {
                fs::create_dir(node.join("01-broken")).unwrap();
                "01-broken"
            }
            "file-as-dir" => {
                fs::create_dir(node.join("01-impl--wrong-k3.md")).unwrap();
                "01-impl--wrong-k3.md"
            }
            "dir-as-file" => {
                fs::write(node.join("01-k3"), "").unwrap();
                "01-k3"
            }
            "node-file-dir" => {
                fs::remove_file(node.join("_later.md")).unwrap();
                fs::create_dir(node.join("_later.md")).unwrap();
                "_later.md"
            }
            "node-file-symlink" => {
                fs::remove_file(node.join("_later.md")).unwrap();
                std::os::unix::fs::symlink("../_BRIEF.md", node.join("_later.md")).unwrap();
                "_later.md"
            }
            "noncanonical-node" => {
                fs::rename(&node, grove.join("002-k2")).unwrap();
                "002-k2"
            }
            _ => unreachable!(),
        };
        let before = snapshot(&grove);
        for args in [
            vec!["pick"],
            vec!["resolve", "first-k1"],
            vec!["brief-chain"],
            vec!["kind"],
            vec!["leaf-add", ".", "new", "--kind", "impl"],
            vec!["leaf-insert", "first-k1", "new", "--kind", "impl"],
            vec!["leaf-decompose", ".grove/01-impl--first-k1.md", "child"],
            vec!["leaf-retire", "first-k1"],
            vec!["leaf-prune", "first-k1"],
        ] {
            let out = run(temp.path(), &args);
            let error = String::from_utf8_lossy(&out.stderr);
            assert!(
                !out.status.success(),
                "{bad}: {args:?} accepted malformed tree"
            );
            assert!(
                out.stdout.is_empty(),
                "{bad}: {args:?} returned an early match"
            );
            assert!(
                error.contains(offending) && error.contains(".grove"),
                "{bad}: {args:?}: {error}"
            );
            assert_eq!(
                snapshot(&grove),
                before,
                "{bad}: {args:?} changed the malformed tree"
            );
        }
    }
}

//! Policy authority and the worker's isolation, through the command seam.
//!
//! Only the personal default and an explicit `--config` select an entry. The
//! hostile cwd fixtures here have a firing configuration in the same test: the
//! identical file, named explicitly, runs and writes its sentinel, so a clean
//! result cannot come from a fixture that never could have fired.

mod support;

use std::fs;
use std::os::fd::AsRawFd;
use std::os::unix::process::CommandExt as _;
use std::path::Path;
use std::process::Stdio;

use serde_json::Value;
use support::{run, text, Sandbox, ROUTED};

/// A valid policy that records, at import, that it ran.
fn sentinel_policy(sentinel: &Path) -> String {
    format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"fired\");\n{ROUTED}",
        text(sentinel)
    )
}

#[test]
fn no_cwd_search_or_environment_variable_selects_an_entry() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("cwd-policy-ran");
    let hostile = sentinel_policy(&sentinel);
    for place in [
        "policy.ts",
        "harness-dispatch.ts",
        ".harness-dispatch/policy.ts",
        ".config/harness-dispatch/policy.ts",
        "harness-dispatch/policy.ts",
    ] {
        sandbox.file(place, &hostile);
    }
    // In a subdirectory too, so a search of parent directories would find them.
    let nested = sandbox.cwd.join("nested/deeper");
    fs::create_dir_all(&nested).unwrap();

    let mut command = sandbox.command();
    command
        .current_dir(&nested)
        .env("XDG_CONFIG_HOME", sandbox.cwd.join(".config"))
        .env("HARNESS_DISPATCH_CONFIG", sandbox.cwd.join("policy.ts"))
        .env("HARNESS_DISPATCH_POLICY", sandbox.cwd.join("policy.ts"))
        .args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut command).refusal(3);

    assert_eq!(refusal["error"]["code"], "policy_missing");
    assert_eq!(refusal["error"]["stage"], "authority");
    assert_eq!(refusal["error"]["source"], text(&sandbox.personal_path()));
    assert!(
        !sentinel.exists(),
        "a cwd policy ran without being selected"
    );

    // The firing configuration: the same file, named explicitly, runs.
    let report = sandbox
        .inspect(&["--kind", "impl", "--config", "policy.ts", "--json"])
        .report();
    assert_eq!(report["policy"]["authority"], "explicit");
    assert!(
        sentinel.exists(),
        "the explicit entry did not run its import"
    );
}

#[test]
fn an_explicit_relative_config_resolves_against_the_original_cwd_and_replaces_the_default() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(&ROUTED.replace("seam-1", "personal-version"));
    let entry = sandbox.file(
        "policies/review.ts",
        &ROUTED.replace("seam-1", "explicit-version"),
    );

    let report = sandbox
        .inspect(&["--kind", "impl", "--config", "policies/review.ts", "--json"])
        .report();

    assert_eq!(report["policy"]["authority"], "explicit");
    assert_eq!(report["policy"]["argument"], "policies/review.ts");
    assert_eq!(report["policy"]["path"], text(&entry));
    // No merge with the personal default.
    assert_eq!(report["policy"]["version"], "explicit-version");

    let human = sandbox.inspect(&["--kind", "impl", "--config", "policies/review.ts"]);
    assert!(
        human.stdout.contains("authority  explicit (--config policies/review.ts, resolved against the current directory)"),
        "{}",
        human.stdout
    );
}

#[test]
fn personal_policy_may_import_a_repository_entry_explicitly() {
    // Relative imports resolve from the importing module, never the cwd.
    let sandbox = Sandbox::new();
    let repository_entry = sandbox.root.join("repository/dispatch/policy.ts");
    support::write(
        &repository_entry,
        "export { policy } from \"./routes.ts\";\n",
    );
    support::write(&sandbox.root.join("repository/dispatch/routes.ts"), ROUTED);
    sandbox.personal_policy(&format!(
        "export {{ policy }} from {:?};\n",
        text(&repository_entry)
    ));

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();

    assert_eq!(report["policy"]["authority"], "personal");
    assert_eq!(report["selection"]["candidateId"], "deep");
}

/// `ROUTED`, with its version made of `parts`, a TypeScript expression list:
/// what the policy's imports gave it, as inspection reports it.
fn versioned_by(imports: &str, parts: &str) -> String {
    format!(
        "{imports}{}",
        ROUTED.replace("\"seam-1\"", &format!("[{parts}].join(\" \")"))
    )
}

#[test]
fn a_package_loads_by_the_entry_point_its_package_json_declares() {
    // Packages beside the personal policy, as an owner's `npm install` there
    // leaves them: one with only an `index.js`, one whose `main` names its
    // entry, one with an `exports` map and a subpath, one whose `exports`
    // chooses by condition, and one that reaches its own file through its
    // `imports` map.
    let sandbox = Sandbox::new();
    let packages = sandbox.home.join(".config/harness-dispatch/node_modules");
    let package = |name: &str, manifest: Option<&str>, files: &[(&str, &str)]| {
        if let Some(manifest) = manifest {
            support::write(&packages.join(name).join("package.json"), manifest);
        }
        for (file, which) in files {
            support::write(
                &packages.join(name).join(file),
                &format!("export const which = {which:?};\n"),
            );
        }
    };
    package("by-layout", None, &[("index.js", "layout")]);
    package(
        "by-main",
        Some(r#"{ "name": "by-main", "main": "./lib/entry.js" }"#),
        &[("lib/entry.js", "main")],
    );
    package(
        "by-exports",
        Some(
            r#"{ "name": "by-exports", "type": "module", "exports": { ".": "./dist/entry.js", "./sub": "./dist/sub.js" } }"#,
        ),
        &[("dist/entry.js", "exports"), ("dist/sub.js", "subpath")],
    );
    package(
        "by-condition",
        Some(
            r#"{ "name": "by-condition", "exports": { ".": { "production": "./production.js", "development": "./development.js", "bun": "./bun.js", "default": "./default.js" } } }"#,
        ),
        &[
            ("production.js", "production"),
            ("development.js", "development"),
            ("bun.js", "bun"),
            ("default.js", "default"),
        ],
    );
    package(
        "by-imports",
        Some(
            r##"{ "name": "by-imports", "exports": "./index.js", "imports": { "#internal": "./internal.js" } }"##,
        ),
        &[("internal.js", "imports")],
    );
    support::write(
        &packages.join("by-imports/index.js"),
        "export { which } from \"#internal\";\n",
    );
    sandbox.personal_policy(&versioned_by(
        r#"import { which as layout } from "by-layout";
import { which as main } from "by-main";
import { which as exported } from "by-exports";
import { which as subpath } from "by-exports/sub";
import { which as condition } from "by-condition";
import { which as imports } from "by-imports";
"#,
        "layout, main, exported, subpath, condition, imports",
    ));
    let loaded = "layout main exports subpath bun imports";

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(report["policy"]["version"], loaded);

    // The condition is the runtime's own. Even a granted NODE_ENV selects no
    // `production` or `development` entry, so no variable chooses the code.
    for node_env in ["production", "development"] {
        let mut command = sandbox.command();
        command.env("NODE_ENV", node_env).args([
            "inspect",
            "--kind",
            "impl",
            "--policy-env",
            "NODE_ENV",
            "--json",
        ]);
        let report = run(&mut command).report();
        assert_eq!(report["policy"]["version"], loaded, "NODE_ENV={node_env}");
    }
}

#[test]
fn an_entrys_own_package_json_applies_its_imports_map_and_answers_its_own_name() {
    // The nearest `package.json` at or above an importing module that reads
    // as one is that module's package. Its `imports` map resolves `#` names,
    // and a bare import of its own name resolves through its `exports`,
    // ahead of a `node_modules` package of that name, as Node's package
    // resolution has it. The file sits where `node_modules` is already
    // trusted.
    let sandbox = Sandbox::new();
    sandbox.file(
        "policies/package.json",
        r##"{ "name": "owner-policies", "type": "module", "imports": { "#routes": "./routes.ts" }, "exports": { "./shared": "./shared.ts" } }"##,
    );
    sandbox.file("policies/routes.ts", "export const which = \"imports\";\n");
    sandbox.file("policies/shared.ts", "export const which = \"own-name\";\n");
    sandbox.file(
        "policies/nested/node_modules/owner-policies/shared.js",
        "export const which = \"node_modules\";\n",
    );
    sandbox.file(
        "policies/nested/policy.ts",
        &versioned_by(
            r##"import { which as mapped } from "#routes";
import { which as named } from "owner-policies/shared";
"##,
            "mapped, named",
        ),
    );

    let report = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--config",
            "policies/nested/policy.ts",
            "--json",
        ])
        .report();

    assert_eq!(report["policy"]["version"], "imports own-name");
}

#[test]
fn the_nearest_package_json_that_reads_as_one_is_a_modules_own_and_one_that_does_not_is_passed_over(
) {
    // An entry one directory below a package whose `imports` map answers
    // `#which`. What sits beside the entry as `package.json` decides whether
    // that outer map is reached.
    let sandbox = Sandbox::new();
    sandbox.file(
        "policies/package.json",
        r##"{ "name": "outer", "imports": { "#which": "./outer.ts" } }"##,
    );
    sandbox.file("policies/outer.ts", "export const which = \"outer\";\n");
    sandbox.file(
        "policies/nested/inner.ts",
        "export const which = \"inner\";\n",
    );
    sandbox.file(
        "policies/nested/policy.ts",
        &versioned_by("import { which } from \"#which\";\n", "which"),
    );
    let nearest = sandbox.cwd.join("policies/nested/package.json");
    let inspect = || {
        sandbox.inspect(&[
            "--kind",
            "impl",
            "--config",
            "policies/nested/policy.ts",
            "--json",
        ])
    };
    let answers = |case: &str, which: &str| {
        assert_eq!(inspect().report()["policy"]["version"], which, "{case}");
    };
    let own_map = r##"{ "imports": { "#which": "./inner.ts" } }"##;

    // With no nearer file the outer package is the entry's own: the control
    // for each case below in which a nearer one is passed over.
    answers("no nearer package.json", "outer");

    // A nearer file that reads as a package is the entry's own, with a name
    // or without one, and reached through a link or not. This is also the
    // control that the nearer place is the one consulted first.
    support::write(&nearest, own_map);
    answers("a nameless package.json with its own map", "inner");
    fs::remove_file(&nearest).unwrap();
    let elsewhere = sandbox.file("elsewhere.json", own_map);
    std::os::unix::fs::symlink(&elsewhere, &nearest).unwrap();
    answers("a link to one", "inner");
    fs::remove_file(&nearest).unwrap();

    // One that reads as a package and has no entry for the name ends the
    // search there. The import refuses, though the outer map has the name.
    for (case, manifest) in [
        ("an empty object", "{}"),
        ("a name and no imports field", r#"{ "name": "nearer" }"#),
        (
            "a map without the name",
            r##"{ "imports": { "#other": "./inner.ts" } }"##,
        ),
    ] {
        support::write(&nearest, manifest);
        let refusal = inspect().refusal(3);
        assert_eq!(
            refusal["error"]["code"], "policy_import_failed",
            "{case}: {refusal}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains("#which"),
            "{case}: {refusal}"
        );
    }
    fs::remove_file(&nearest).unwrap();

    // One that does not read as a package is passed over, with no
    // diagnostic, and the search goes on upward: the outer map answers. A
    // broken `package.json` beside a policy therefore does not stop the next
    // one above it from choosing the code, where Node refuses the import.
    for (case, manifest) in [
        ("one that does not parse", "{ broken"),
        ("JSON that is not an object", "[]"),
    ] {
        support::write(&nearest, manifest);
        answers(case, "outer");
    }
    fs::remove_file(&nearest).unwrap();
    fs::create_dir(&nearest).unwrap();
    answers("a directory of that name", "outer");
}

#[test]
fn a_symlinked_entry_is_reported_and_imported_at_its_real_path() {
    let sandbox = Sandbox::new();
    let real = sandbox.root.join("owner/policy.ts");
    support::write(&real, ROUTED);
    let personal = sandbox.personal_path();
    fs::create_dir_all(personal.parent().unwrap()).unwrap();
    std::os::unix::fs::symlink(&real, &personal).unwrap();

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();

    assert_eq!(report["policy"]["path"], text(&real));
}

#[test]
fn a_missing_unreadable_or_unlocatable_entry_refuses_naming_the_path() {
    let sandbox = Sandbox::new();

    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_missing");
    assert!(refusal["error"]["message"]
        .as_str()
        .unwrap()
        .contains(&text(&sandbox.personal_path())));

    fs::create_dir_all(sandbox.cwd.join("a-directory")).unwrap();
    let refusal = sandbox
        .inspect(&["--kind", "impl", "--config", "a-directory", "--json"])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_unreadable");
    assert_eq!(
        refusal["error"]["source"],
        text(&sandbox.cwd.join("a-directory"))
    );

    let mut no_home = sandbox.command();
    no_home
        .env_remove("HOME")
        .args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut no_home).refusal(3);
    assert_eq!(refusal["error"]["code"], "home_unset");

    // A relative HOME would make the personal policy depend on the cwd.
    sandbox.file("home/.config/harness-dispatch/policy.ts", ROUTED);
    let mut relative_home = sandbox.command();
    relative_home
        .env("HOME", "home")
        .args(["inspect", "--kind", "impl", "--json"]);
    let refusal = run(&mut relative_home).refusal(3);
    assert_eq!(refusal["error"]["code"], "home_unset");
}

#[test]
fn the_worker_evaluates_policy_in_the_root_directory_with_null_stdin_and_a_fresh_environment() {
    let sandbox = Sandbox::new();
    let report_path = sandbox.root.join("worker-view.json");
    sandbox.personal_policy(&format!(
        r#"import {{ fstatSync, readSync, writeFileSync }} from "node:fs";
const open = (fd: number) => {{ try {{ fstatSync(fd); return true; }} catch {{ return false; }} }};
const stdin = Buffer.alloc(64);
writeFileSync({report:?}, JSON.stringify({{
  cwd: process.cwd(),
  env: Object.keys(process.env).sort(),
  stdinBytes: readSync(0, stdin, 0, 64, null),
  channelOpen: open(3),
  callerDescriptorOpen: open(7),
}}));
{ROUTED}"#,
        report = text(&report_path)
    ));
    let inherited = fs::File::open(sandbox.personal_path()).unwrap();

    let mut command = sandbox.command();
    command
        .env("LANG", "C")
        .env("LC_ALL", "C")
        .env("GROVE_SIGNAL_FILE", sandbox.root.join("completion"))
        .env("NODE_OPTIONS", "--require=/nonexistent")
        .env("SECRET_TOKEN", "must-not-reach-policy")
        .args(["inspect", "--kind", "impl", "--json"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    // The front starts holding a descriptor 7 that a caller might have left
    // open.
    support::caller_leaves_open(&mut command, Some((&inherited, 7)));
    let mut child = command.spawn().unwrap();
    {
        use std::io::Write as _;
        let mut stdin = child.stdin.take().unwrap();
        let _ = stdin.write_all(b"caller terminal input\n");
    }
    let output = child.wait_with_output().unwrap();
    let report = support::Run::from(output).report();
    assert_eq!(report["selection"]["candidateId"], "deep");

    let view: Value = serde_json::from_str(&fs::read_to_string(&report_path).unwrap()).unwrap();
    // The worker has left the private directory it started in
    // (`worker::the_front_starts_its_worker_in_a_private_empty_directory_and_removes_it`)
    // by the time any policy code runs, and the front has removed it since.
    assert_eq!(view["cwd"], "/");
    assert_eq!(
        fs::read_dir(&sandbox.tmp).unwrap().count(),
        0,
        "the private directory outlived the worker"
    );
    assert_eq!(
        view["env"],
        serde_json::json!([
            "BUN_RUNTIME_TRANSPILER_CACHE_PATH",
            "HOME",
            "LANG",
            "LC_ALL",
            "PATH",
            "TMPDIR"
        ])
    );
    assert_eq!(view["stdinBytes"], 0);
    // The probe can see an open descriptor (the channel), and sees none of
    // the caller's.
    assert_eq!(view["channelOpen"], true);
    assert_eq!(view["callerDescriptorOpen"], false);
}

#[test]
fn a_caller_descriptor_above_the_soft_descriptor_limit_never_reaches_the_worker() {
    let sandbox = Sandbox::new();
    let report_path = sandbox.root.join("worker-view.json");
    sandbox.personal_policy(&format!(
        r#"import {{ fstatSync, writeFileSync }} from "node:fs";
const open = (fd: number) => {{ try {{ fstatSync(fd); return true; }} catch {{ return false; }} }};
writeFileSync({report:?}, JSON.stringify({{ channelOpen: open(3), callerDescriptorOpen: open(100) }}));
{ROUTED}"#,
        report = text(&report_path)
    ));
    let inherited = fs::File::open(sandbox.personal_path()).unwrap();
    let inherited_fd = inherited.as_raw_fd();

    let mut command = sandbox.command();
    command.args(["inspect", "--kind", "impl", "--json"]);
    // SAFETY: dup2, getrlimit and setrlimit are async-signal-safe. The front
    // starts holding descriptor 100 under a soft limit of 64, which lowering
    // the limit does not close: a sweep bounded by the limit never reaches it.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(inherited_fd, 100) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            let mut limit: libc::rlimit = std::mem::zeroed();
            if libc::getrlimit(libc::RLIMIT_NOFILE, &mut limit) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            limit.rlim_cur = 64;
            if libc::setrlimit(libc::RLIMIT_NOFILE, &limit) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let report = run(&mut command).report();
    assert_eq!(report["selection"]["candidateId"], "deep");

    let view: Value = serde_json::from_str(&fs::read_to_string(&report_path).unwrap()).unwrap();
    // The probe sees an open descriptor (the channel), and not the caller's.
    assert_eq!(view["channelOpen"], true);
    assert_eq!(view["callerDescriptorOpen"], false);
}

#[test]
fn an_entry_path_the_worker_cannot_import_exactly_refuses_before_any_code_runs() {
    let sandbox = Sandbox::new();
    // Each admitted name has a sibling that Bun would import in its place,
    // reading `?` as the start of a query.
    for (admitted, substitute) in [("policy.ts?x", "policy.ts"), ("d?q/policy.ts", "d.ts")] {
        let sentinel = sandbox.root.join(format!("{substitute}-ran"));
        sandbox.file(admitted, ROUTED);
        sandbox.file(substitute, &sentinel_policy(&sentinel));

        let refusal = sandbox
            .inspect(&["--kind", "impl", "--config", admitted, "--json"])
            .refusal(3);
        let error = &refusal["error"];
        assert_eq!(error["code"], "policy_unreadable", "{admitted}: {refusal}");
        assert_eq!(error["stage"], "authority", "{admitted}");
        assert_eq!(
            error["source"],
            text(&sandbox.cwd.join(admitted)),
            "{admitted}"
        );
        assert!(
            error["message"].as_str().unwrap().contains("`?`"),
            "{refusal}"
        );
        assert!(
            !sentinel.exists(),
            "{substitute} ran in place of {admitted}"
        );

        // The firing configuration: the substitute, admitted by its own name,
        // runs and leaves its sentinel.
        let report = sandbox
            .inspect(&["--kind", "impl", "--config", substitute, "--json"])
            .report();
        assert_eq!(report["selection"]["candidateId"], "deep");
        assert!(sentinel.exists(), "{substitute} never fired");
    }

    // A name that only looks special to a URL still imports exactly itself.
    let sentinel = sandbox.root.join("exact-ran");
    sandbox.file("a b#c%41.ts", &sentinel_policy(&sentinel));
    let report = sandbox
        .inspect(&["--kind", "impl", "--config", "a b#c%41.ts", "--json"])
        .report();
    assert_eq!(
        report["policy"]["path"],
        text(&sandbox.cwd.join("a b#c%41.ts"))
    );
    assert!(sentinel.exists());
}

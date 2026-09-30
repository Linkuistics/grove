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
fn the_worker_runs_in_a_private_empty_directory_with_null_stdin_and_a_fresh_environment() {
    let sandbox = Sandbox::new();
    let report_path = sandbox.root.join("worker-view.json");
    sandbox.personal_policy(&format!(
        r#"import {{ fstatSync, readdirSync, readSync, writeFileSync }} from "node:fs";
const open = (fd: number) => {{ try {{ fstatSync(fd); return true; }} catch {{ return false; }} }};
const stdin = Buffer.alloc(64);
writeFileSync({report:?}, JSON.stringify({{
  cwd: process.cwd(),
  cwdEntries: readdirSync(process.cwd()),
  env: Object.keys(process.env).sort(),
  stdinBytes: readSync(0, stdin, 0, 64, null),
  channelOpen: open(3),
  callerDescriptorOpen: open(7),
}}));
{ROUTED}"#,
        report = text(&report_path)
    ));
    let inherited = fs::File::open(sandbox.personal_path()).unwrap();
    let inherited_fd = inherited.as_raw_fd();

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
    // SAFETY: dup2 is async-signal-safe; it hands the front a descriptor 7
    // that a caller might have left open.
    unsafe {
        command.pre_exec(move || {
            if libc::dup2(inherited_fd, 7) == -1 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
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
    let cwd = view["cwd"].as_str().unwrap();
    assert_ne!(cwd, text(&sandbox.cwd));
    assert!(
        Path::new(cwd).starts_with(fs::canonicalize(&sandbox.tmp).unwrap()),
        "{cwd}"
    );
    assert_eq!(view["cwdEntries"], serde_json::json!([]));
    assert!(
        !Path::new(cwd).exists(),
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

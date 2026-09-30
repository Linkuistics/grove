//! Hostile directories, environments and policy output, through the public
//! launcher (`docs/specs/harness-selection-and-execution.md`, *Policy authority
//! and runtime discovery* and the firing-configuration table under *Agreed
//! test seams and acceptance*).
//!
//! Each class stays inert through the front, and in the same test its
//! fixture is seen to fire in the class's firing configuration, so a clean
//! result cannot come from a fixture that never could have fired. The
//! configurations that need a probe build, or the shipped worker without the
//! front's scrubbing, drive the worker directly (`support::direct`). The cwd
//! policy entry class is `authority::no_cwd_search_or_environment_variable_selects_an_entry`,
//! whose firing configuration is the same file named by `--config`.

mod support;

use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::{json, Value};
use support::direct::{self, probe_build, shipped_worker, Probe};
use support::{run, text, Sandbox, FRONT, ROUTED};

/// A module that writes `fired` to `sentinel` when it is imported or preloaded.
fn sentinel_module(sentinel: &Path) -> String {
    format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"fired\");\nexport const shadowed = true;\n",
        text(sentinel)
    )
}

/// The environment a directly driven worker gets: the front's base set.
fn base_env(sandbox: &Sandbox) -> Vec<(&'static str, OsString)> {
    vec![
        ("HOME", sandbox.home.clone().into()),
        ("PATH", "/usr/bin:/bin".into()),
        ("TMPDIR", sandbox.tmp.clone().into()),
    ]
}

/// The shipped worker's build identity, as the checkout's front reports it.
fn shipped_build() -> String {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    report["worker"]["buildId"].as_str().unwrap().to_owned()
}

/// Assert that a probe announced itself, built from this checkout's source.
fn assert_probe_identity(driven: &direct::Driven, probe: Probe, shipped: &str) {
    let hello = driven.hello.as_ref().unwrap_or_else(|| {
        panic!(
            "the {} probe sent no hello\nstderr: {}",
            probe.name(),
            driven.stderr
        )
    });
    assert_eq!(
        hello["buildId"],
        format!("probe-{}-{shipped}", probe.name()),
        "the {} probe is stale; run `task dispatch:probes`",
        probe.name()
    );
}

fn read_json(path: &Path) -> Value {
    let view = fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("{} was not written: {error}", path.display()));
    fs::remove_file(path).unwrap();
    serde_json::from_str(&view).unwrap()
}

/// A routes policy that records, at import, what the dotenv fixtures set.
fn dotenv_view_policy(view: &Path) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
writeFileSync({view:?}, JSON.stringify({{
  dotenv: process.env.HOSTILE_DOTENV ?? null,
  dotenvLocal: process.env.HOSTILE_DOTENV_LOCAL ?? null,
}}));
{ROUTED}"#,
        view = text(view)
    )
}

#[test]
fn a_cwd_dotenv_and_bunfig_preload_stay_inert_and_fire_under_the_autoload_probe() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let preloaded = sandbox.root.join("bunfig-preload-ran");
    let view = sandbox.root.join("view.json");
    // The caller's cwd is the hostile directory.
    sandbox.file(".env", "HOSTILE_DOTENV=from-dotenv\n");
    sandbox.file(".env.local", "HOSTILE_DOTENV_LOCAL=from-dotenv-local\n");
    sandbox.file("bunfig.toml", "preload = [\"./hostile-preload.ts\"]\n");
    sandbox.file("hostile-preload.ts", &sentinel_module(&preloaded));
    let entry = sandbox.personal_policy(&dotenv_view_policy(&view));
    let inert = json!({ "dotenv": null, "dotenvLocal": null });

    // Through the public launcher, both commands.
    sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(read_json(&view), inert);
    let ran = sandbox.run(&["--kind", "impl", "--prompt", "p"]);
    assert_eq!(ran.code, Some(0), "{}", ran.stderr);
    assert_eq!(read_json(&view), inert);
    assert!(
        !preloaded.exists(),
        "a cwd bunfig preload ran through the front"
    );

    // The firing configuration: the autoload probe, run in the hostile
    // directory, loads both files and runs the preload.
    let driven = direct::drive(
        &probe_build(Probe::Autoload),
        &sandbox.cwd,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Autoload, &shipped);
    driven.loaded();
    assert_eq!(
        read_json(&view),
        json!({ "dotenv": "from-dotenv", "dotenvLocal": "from-dotenv-local" })
    );
    assert!(
        preloaded.exists(),
        "the autoload probe never ran the preload"
    );
    fs::remove_file(&preloaded).unwrap();

    // Either control alone keeps the class inert: the shipped worker in the
    // hostile directory, and the autoload probe in an empty one, as the
    // front's private directory is.
    let empty = sandbox.root.join("empty");
    fs::create_dir(&empty).unwrap();
    for (worker, cwd) in [
        (shipped_worker(), &sandbox.cwd),
        (probe_build(Probe::Autoload), &empty),
    ] {
        direct::drive(&worker, cwd, &base_env(&sandbox), &entry).loaded();
        assert_eq!(read_json(&view), inert, "{}", worker.display());
        assert!(!preloaded.exists(), "{}", worker.display());
    }
}

#[test]
fn bun_runtime_variables_stay_inert_through_the_front_and_fire_in_the_worker_started_directly() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let preloaded = sandbox.root.join("options-preload-ran");
    let preload = sandbox.root.join("options-preload.ts");
    support::write(&preload, &sentinel_module(&preloaded));
    let options = format!("--preload {}", text(&preload));

    // Through the public launcher the worker is itself: it identifies itself,
    // evaluates the policy and runs no preload.
    for (name, value) in [("BUN_OPTIONS", options.as_str()), ("BUN_BE_BUN", "1")] {
        let mut command = sandbox.command();
        command
            .env(name, value)
            .args(["inspect", "--kind", "impl", "--json"]);
        let report = run(&mut command).report();
        assert_eq!(report["selection"]["candidateId"], "deep", "{name}");
        assert!(!preloaded.exists(), "{name} preloaded through the front");
    }

    // The firing configurations: the shipped worker, started directly with
    // each variable. BUN_OPTIONS preloads the module before the worker's
    // own code, and BUN_BE_BUN turns the worker into Bun, running any code
    // it is handed.
    let mut env = base_env(&sandbox);
    env.push(("BUN_OPTIONS", options.clone().into()));
    let entry = sandbox.personal_path();
    direct::drive(&shipped_worker(), &sandbox.root, &env, &entry).loaded();
    assert!(preloaded.exists(), "BUN_OPTIONS never preloaded");

    let be_bun = sandbox.root.join("be-bun-ran");
    let output = Command::new(shipped_worker())
        .env_clear()
        .envs(base_env(&sandbox))
        .env("BUN_BE_BUN", "1")
        .args([
            "-e",
            &format!(
                "require(\"node:fs\").writeFileSync({:?}, \"fired\")",
                text(&be_bun)
            ),
        ])
        .output()
        .unwrap();
    assert!(output.status.success(), "{output:?}");
    assert!(be_bun.exists(), "BUN_BE_BUN did not make the worker Bun");
}

/// Every specifier the worker documents: its SDK and one per shipped example.
/// `grove-review-adapter-k37` adds `harness-dispatch/grove`.
fn documented_specifiers() -> Vec<String> {
    let examples = Path::new(env!("CARGO_MANIFEST_DIR")).join("worker/examples");
    let mut specifiers = vec!["harness-dispatch/sdk".to_owned()];
    let mut stems: Vec<String> = fs::read_dir(&examples)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "ts"))
        .map(|path| path.file_stem().unwrap().to_string_lossy().into_owned())
        .collect();
    stems.sort();
    assert!(
        !stems.is_empty(),
        "no shipped examples in {}",
        examples.display()
    );
    specifiers.extend(
        stems
            .iter()
            .map(|stem| format!("harness-dispatch/examples/{stem}")),
    );
    specifiers
}

/// A file-name form of a specifier.
fn slug(specifier: &str) -> String {
    specifier.replace('/', "_")
}

/// A `node_modules/harness-dispatch` shadow in `dir`, answering each of
/// `specifiers` with a module that writes its sentinel under `fired` and
/// exports only `shadowed`. It is laid out as files, with no `package.json`:
/// the shipped worker reads no `package.json` at run time, so a package's
/// `exports` or `main` never applies, and an `exports`-only shadow would not
/// load under any build (runtime evidence, *Ambient authority*).
fn shadow_package(dir: &Path, specifiers: &[String], fired: &Path) {
    let package = dir.join("node_modules/harness-dispatch");
    for specifier in specifiers {
        let subpath = specifier.strip_prefix("harness-dispatch/").unwrap();
        support::write(
            &package.join(format!("{subpath}.js")),
            &sentinel_module(&fired.join(slug(specifier))),
        );
    }
}

/// An entry importing each specifier and recording each module's export
/// names at `view`.
fn importing_entry(specifiers: &[String], view: &Path) -> String {
    let mut source = String::from("import { writeFileSync } from \"node:fs\";\n");
    for (index, specifier) in specifiers.iter().enumerate() {
        source.push_str(&format!("import * as m{index} from {specifier:?};\n"));
    }
    let fields: Vec<String> = specifiers
        .iter()
        .enumerate()
        .map(|(index, specifier)| format!("{specifier:?}: Object.keys(m{index}).sort()"))
        .collect();
    source.push_str(&format!(
        "writeFileSync({:?}, JSON.stringify({{ {} }}));\n{ROUTED}",
        text(view),
        fields.join(", ")
    ));
    source
}

#[test]
fn every_documented_specifier_resolves_to_its_embedded_module_beside_a_package_shadow() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let documented = documented_specifiers();
    // The registered list is the worker's own, and it is the documented one.
    let main = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("worker/src/main.ts"))
        .unwrap();
    for specifier in &documented {
        assert!(
            main.contains(&format!("{specifier:?}: ")),
            "{specifier} is documented but not in main.ts's embedded table"
        );
    }
    assert_eq!(
        main.matches("\"harness-dispatch/").count(),
        documented.len(),
        "main.ts registers a specifier that is not documented"
    );

    // An unregistered name under the prefix, which the shadow also answers:
    // the prefix itself reserves nothing, so it resolves to the shadow even
    // through the front, and shows the fixture is live beside this entry.
    let unregistered = "harness-dispatch/unregistered-name".to_owned();
    let mut imported = documented.clone();
    imported.push(unregistered.clone());
    let fired = sandbox.root.join("shadow-fired");
    fs::create_dir(&fired).unwrap();
    let view = sandbox.root.join("view.json");
    shadow_package(&sandbox.cwd.join("policies"), &imported, &fired);
    let entry = sandbox.file("policies/policy.ts", &importing_entry(&imported, &view));

    let report = sandbox
        .inspect(&["--kind", "impl", "--config", "policies/policy.ts", "--json"])
        .report();
    assert_eq!(report["policy"]["authority"], "explicit");
    let seen = read_json(&view);
    for specifier in &documented {
        let keys = seen[specifier].as_array().unwrap();
        assert!(
            !keys.is_empty() && !keys.contains(&json!("shadowed")),
            "{specifier} did not resolve to its embedded module: {seen}"
        );
        assert!(
            !fired.join(slug(specifier)).exists(),
            "the shadow of {specifier} loaded through the front"
        );
    }
    assert_eq!(seen[&unregistered], json!(["shadowed"]));
    assert!(fired.join(slug(&unregistered)).exists());
    fs::remove_file(fired.join(slug(&unregistered))).unwrap();

    // The firing configuration: the unregistered probe, with the same entry.
    let driven = direct::drive(
        &probe_build(Probe::Unregistered),
        &sandbox.root,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Unregistered, &shipped);
    driven.loaded();
    let seen = read_json(&view);
    for specifier in &imported {
        assert_eq!(seen[specifier], json!(["shadowed"]), "{specifier}: {seen}");
        assert!(
            fired.join(slug(specifier)).exists(),
            "the shadow of {specifier} never fired"
        );
    }
}

/// A tsconfig and package.json in `dir` mapping the bare `hostile-alias` to a
/// module there that writes `sentinel`.
fn tsconfig_alias(dir: &Path, sentinel: &Path) {
    support::write(
        &dir.join("tsconfig.json"),
        r#"{ "compilerOptions": { "baseUrl": ".", "paths": { "hostile-alias": ["./hostile-alias.ts"] } } }"#,
    );
    support::write(
        &dir.join("package.json"),
        r#"{ "name": "owner-policies", "private": true, "type": "module" }"#,
    );
    support::write(&dir.join("hostile-alias.ts"), &sentinel_module(sentinel));
}

fn aliased_entry() -> String {
    format!("import {{ shadowed }} from \"hostile-alias\";\nvoid shadowed;\n{ROUTED}")
}

#[test]
fn tsconfig_paths_beside_an_admitted_entry_stay_inert_and_fire_under_the_tsconfig_probe() {
    let sandbox = Sandbox::new();
    let shipped = shipped_build();
    let aliased = sandbox.root.join("alias-ran");
    tsconfig_alias(&sandbox.cwd.join("policies"), &aliased);
    let entry = sandbox.file("policies/aliased.ts", &aliased_entry());

    // Through the front the alias does not apply, so the import is missing.
    let refusal = sandbox
        .inspect(&[
            "--kind",
            "impl",
            "--config",
            "policies/aliased.ts",
            "--json",
        ])
        .refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains("hostile-alias"),
        "{refusal}"
    );
    assert!(
        !aliased.exists(),
        "a tsconfig alias loaded through the front"
    );

    // The firing configuration: the tsconfig probe, with the same entry.
    let driven = direct::drive(
        &probe_build(Probe::Tsconfig),
        &sandbox.root,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Tsconfig, &shipped);
    driven.loaded();
    assert!(
        aliased.exists(),
        "the tsconfig probe never applied the alias"
    );
}

/// Fail loudly if a class reported as having no firing configuration has
/// gained one: it would then be a class to prove and count, not to report.
fn still_unfired(class: &str, fired: bool) {
    assert!(
        !fired,
        "the {class} class fired under its most permissive probe build: it now has a firing \
         configuration, so prove it inert through the front, count it, and update the spec's \
         firing-configuration table and the runtime evidence"
    );
}

#[test]
fn classes_with_no_known_firing_configuration_are_reported_not_counted() {
    // These are tripwires, not controls. Neither class has been seen to fire
    // in any build, so an inert result here proves nothing about the front.
    // What the test does show is that the pinned Bun still gives them no
    // firing configuration; if one appears, it fails and says so.
    let sandbox = Sandbox::new();
    let shipped = shipped_build();

    // A HOME bunfig preload. HOME is in the worker's base environment.
    let preloaded = sandbox.root.join("home-bunfig-ran");
    support::write(
        &sandbox.home.join("hostile-preload.ts"),
        &sentinel_module(&preloaded),
    );
    support::write(
        &sandbox.home.join(".bunfig.toml"),
        &format!(
            "preload = [{:?}]\n",
            text(&sandbox.home.join("hostile-preload.ts"))
        ),
    );
    let entry = sandbox.personal_policy(ROUTED);
    sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert!(
        !preloaded.exists(),
        "a HOME bunfig preload ran through the front"
    );
    let empty = sandbox.root.join("empty");
    fs::create_dir(&empty).unwrap();
    let driven = direct::drive(
        &probe_build(Probe::Autoload),
        &empty,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Autoload, &shipped);
    driven.loaded();
    still_unfired("HOME bunfig", preloaded.exists());

    // A tsconfig alias in the caller's cwd, for an entry elsewhere.
    let aliased = sandbox.root.join("cwd-alias-ran");
    tsconfig_alias(&sandbox.cwd, &aliased);
    let entry = sandbox.personal_policy(&aliased_entry());
    let refusal = sandbox.inspect(&["--kind", "impl", "--json"]).refusal(3);
    assert_eq!(
        refusal["error"]["code"], "policy_import_failed",
        "{refusal}"
    );
    assert!(
        !aliased.exists(),
        "a cwd tsconfig alias loaded through the front"
    );
    let driven = direct::drive(
        &probe_build(Probe::Tsconfig),
        &sandbox.cwd,
        &base_env(&sandbox),
        &entry,
    );
    assert_probe_identity(&driven, Probe::Tsconfig, &shipped);
    still_unfired("cwd tsconfig", aliased.exists());
}

#[test]
fn a_probe_build_is_never_accepted_as_an_installations_worker() {
    // So no archive can ship one as its worker: the installed smoke test
    // inspects through each archive's front before a release publishes.
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);
    let prefix = sandbox.root.join("prefix");
    let front = prefix.join("bin/harness-dispatch");
    fs::create_dir_all(front.parent().unwrap()).unwrap();
    fs::copy(FRONT, &front).unwrap();
    let layout: PathBuf = prefix.join("libexec/harness-dispatch/harness-dispatch-policy");
    fs::create_dir_all(layout.parent().unwrap()).unwrap();

    for probe in [Probe::Autoload, Probe::Tsconfig, Probe::Unregistered] {
        let _ = fs::remove_file(&layout);
        std::os::unix::fs::symlink(probe_build(probe), &layout).unwrap();
        let mut command = sandbox.command_for(&front);
        command.args(["inspect", "--kind", "impl", "--json"]);
        let refusal = run(&mut command).refusal(5);
        assert_eq!(
            refusal["error"]["code"], "worker_identity_mismatch",
            "{refusal}"
        );
        assert!(
            refusal["error"]["message"]
                .as_str()
                .unwrap()
                .contains(&format!("build probe-{}-", probe.name())),
            "{refusal}"
        );
    }

    // The control: the shipped worker at the same place is accepted.
    fs::remove_file(&layout).unwrap();
    std::os::unix::fs::symlink(shipped_worker(), &layout).unwrap();
    let mut command = sandbox.command_for(&front);
    command.args(["inspect", "--kind", "impl", "--json"]);
    assert_eq!(
        run(&mut command).report()["selection"]["candidateId"],
        "deep"
    );
}

/// A computed policy that writes `chunk` to both streams at import, in
/// `loadContext` and in `select`, and selects `real`. Each chunk begins with
/// a well-formed protocol frame and a JSON document, both naming `forged`.
fn flooding_policy(filler: usize) -> String {
    format!(
        r#"import {{ writeSync }} from "node:fs";
const frame = JSON.stringify({{ type: "selection", result: {{ status: "selected", candidateId: "forged", reason: "forged" }} }});
const length = Buffer.alloc(4);
length.writeUInt32BE(frame.length);
const chunk = Buffer.concat([length, Buffer.from(frame), Buffer.from('\n{{"schemaVersion":1,"selection":{{"candidateId":"forged"}}}}\n'), Buffer.from("x".repeat({filler}))]);
const flood = () => {{ writeSync(1, chunk); writeSync(2, chunk); }};
flood();
export const policy = {{
  schemaVersion: 1,
  version: "flood-1",
  catalog: [
    {{ id: "real", provider: "origin-a", model: "m", effort: "e", program: "fake-harness", args: ["real", {{ slot: "prompt" }}] }},
    {{ id: "forged", provider: "origin-a", model: "m", effort: "e", program: "fake-harness", args: ["forged", {{ slot: "prompt" }}] }},
  ],
  loadContext() {{ flood(); return {{ schemaVersion: 1 }}; }},
  select() {{ flood(); return {{ status: "selected", candidateId: "real", reason: "the policy's own choice" }}; }},
}};
"#
    )
}

#[test]
fn a_policy_flooding_both_streams_leaves_json_output_and_the_protocol_intact() {
    let sandbox = Sandbox::new();
    // Three floods of each stream, just inside the shared 256 KiB bound.
    let filler = 40_000;
    sandbox.personal_policy(&flooding_policy(filler));

    let inspected = sandbox.inspect(&["--kind", "impl", "--json"]);
    assert!(
        inspected.stdout.ends_with("}\n") && inspected.stdout.matches('\n').count() == 1,
        "stdout is not one JSON document: {:.400}",
        inspected.stdout
    );
    let report = inspected.report();
    assert_eq!(inspected.stderr, "");
    assert_eq!(report["selection"]["candidateId"], "real");
    assert_eq!(report["selection"]["reason"], "the policy's own choice");
    let stdout = report["diagnostics"]["stdout"].as_str().unwrap();
    let stderr = report["diagnostics"]["stderr"].as_str().unwrap();
    assert_eq!(stdout, stderr);
    assert_eq!(stdout.matches("\"candidateId\":\"forged\"").count(), 6);
    assert!(stdout.len() > 3 * filler && stdout.len() + stderr.len() <= 262_144);

    let human = sandbox.inspect(&["--kind", "impl"]);
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    assert!(
        human.stdout.starts_with("Proposal only"),
        "{:.400}",
        human.stdout
    );
    assert!(!human.stdout.contains("forged"), "{:.400}", human.stdout);
    assert!(
        human
            .stderr
            .lines()
            .all(|line| line.starts_with("policy stdout: ") || line.starts_with("policy stderr: ")),
        "an unprefixed policy line on stderr"
    );

    let ran = sandbox.run(&["--kind", "impl", "--prompt", "p", "--json"]);
    assert_eq!(ran.code, Some(0), "{:.400}", ran.stderr);
    assert_eq!(
        ran.stdout, "",
        "the policy's output reached the harness's stdout"
    );
    assert_eq!(ran.stderr.matches('\n').count(), 1, "{:.400}", ran.stderr);
    let notice: Value = serde_json::from_str(ran.stderr.trim_end()).unwrap();
    assert_eq!(notice["handoff"]["candidateId"], "real");
    assert_eq!(sandbox.harness_args(), ["real", "p"]);
}

//! Explicit-source installed Ravel builds for Grove's consumer fixtures.
//!
//! This is test-only support. It copies the installed public layout of one
//! build; it neither builds Ravel nor changes the source installation.
//!
//! **The runtime is explicit.** `GROVE_TEST_RAVEL` names the front, and
//! `GROVE_TEST_RAVEL_RUNTIME` names the configuration runtime that the front
//! reads: `1.1` for the TypeScript `policy.ts` of schema 2, or `2.0` for the
//! Rhai `config.rhai` of schema 4. The support never reads the runtime from the
//! version output, because a build of the 2.0 feature line reports
//! `ravel 1.1.0` until the release cut. It reads the layout only to refuse a
//! mismatch: a 1.x build has the `libexec/ravel/ravel-policy` worker, and a
//! 2.x build installs one executable and no worker. An unset runtime keeps the
//! established 1.1 meaning, which only a build with that worker can satisfy.
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// The variable that selects the configuration runtime of the build under test.
pub const RUNTIME_VAR: &str = "GROVE_TEST_RAVEL_RUNTIME";

/// The worker of a 1.x installation, relative to its prefix.
const WORKER: &str = "libexec/ravel/ravel-policy";

/// The configuration runtime of one Ravel build.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Runtime {
    /// Ravel 1.1: `policy.ts`, request and policy schema 2, a Bun worker.
    V1_1,
    /// Ravel 2.0: `config.rhai`, `SCHEMA_VERSION` 4, one executable.
    V2_0,
}

impl Runtime {
    /// The runtime that `GROVE_TEST_RAVEL_RUNTIME` selects for the build whose
    /// front is `front`. A selection that the layout of the build cannot
    /// satisfy fails here, before any fixture runs.
    pub fn for_build(front: &Path) -> Self {
        let selected = std::env::var(RUNTIME_VAR).unwrap_or_default();
        Self::named(front, RUNTIME_VAR, &selected)
    }

    /// The runtime that `selected` names for the build whose front is `front`,
    /// as the variable `variable` gives it. The layout check and the remedies
    /// are those of [`Runtime::for_build`].
    pub fn named(front: &Path, variable: &str, selected: &str) -> Self {
        let prefix = prefix_of(front);
        let worker = prefix.join(WORKER).is_file();
        match selected {
            "1.1" => {
                assert!(
                    worker,
                    "{variable}=1.1 selects the 1.x layout, but {} has no {WORKER} worker. \
                     A 2.x build installs one executable: set {variable}=2.0 for it.",
                    prefix.display()
                );
                Runtime::V1_1
            }
            "2.0" => {
                assert!(
                    !worker,
                    "{variable}=2.0 selects the 2.x layout, but {} holds the 1.x worker \
                     {WORKER}. Set {variable}=1.1 for a 1.x build.",
                    prefix.display()
                );
                Runtime::V2_0
            }
            "" => {
                assert!(
                    worker,
                    "{variable} is unset, and {} has no {WORKER} worker, so it is no 1.x \
                     build. Set {variable}=2.0 for a 2.x build. The support never reads \
                     the runtime from the version output.",
                    prefix.display()
                );
                Runtime::V1_1
            }
            other => panic!("{variable}={other:?} names no runtime: use 1.1 or 2.0"),
        }
    }

    /// The name of the runtime, as `GROVE_TEST_RAVEL_RUNTIME` spells it.
    pub fn label(self) -> &'static str {
        match self {
            Runtime::V1_1 => "1.1",
            Runtime::V2_0 => "2.0",
        }
    }

    /// The basename of the default configuration entry under
    /// `$HOME/.config/ravel`.
    pub fn config_file(self) -> &'static str {
        match self {
            Runtime::V1_1 => "policy.ts",
            Runtime::V2_0 => "config.rhai",
        }
    }

    /// The settings key that grants environment names to selection.
    pub fn grant_key(self) -> &'static str {
        match self {
            Runtime::V1_1 => "policyEnv",
            Runtime::V2_0 => "configEnv",
        }
    }

    /// The word that prefixes Ravel-owned refusal codes and labels the owner
    /// code in a refusal: `policy` on 1.1 and `config` on 2.0.
    pub fn prefix(self) -> &'static str {
        match self {
            Runtime::V1_1 => "policy",
            Runtime::V2_0 => "config",
        }
    }

    /// A Ravel-owned refusal code: `policy_missing` on 1.1 is
    /// `config_missing` on 2.0.
    pub fn code(self, suffix: &str) -> String {
        format!("{}_{suffix}", self.prefix())
    }

    /// Words of the remedy that Ravel gives when the default entry is missing.
    pub fn missing_remedy(self) -> &'static str {
        match self {
            Runtime::V1_1 => "exporting a `policy`",
            Runtime::V2_0 => "defining SCHEMA_VERSION, VERSION and `fn select(request, context)`",
        }
    }

    /// One of two values, by runtime.
    pub fn pick<T>(self, v1_1: T, v2_0: T) -> T {
        match self {
            Runtime::V1_1 => v1_1,
            Runtime::V2_0 => v2_0,
        }
    }
}

/// Rhai helpers for a 2.0 fixture that reads the environment. `granted`
/// returns the caller's value of a name, or `()` when the name is unset or no
/// grant names it, which matches what a 1.1 policy sees in `process.env`.
/// `or_unset` writes `()` as `<unset>`.
pub const RHAI_GRANTED: &str = r#"fn granted(name) {
    let value = ();
    try { value = ravel::env(name); } catch (error) { if error.code != "env_not_granted" { throw error; } }
    value
}

fn or_unset(value) { if type_of(value) == "()" { "<unset>" } else { value } }
"#;

/// A string literal that both TypeScript and Rhai read as `text`.
///
/// Only `"`, `\` and the control characters are escaped, and each control
/// character as `\uXXXX`, which both languages accept. Every other character
/// is written as itself.
pub fn quote(text: &str) -> String {
    let mut quoted = String::with_capacity(text.len() + 2);
    quoted.push('"');
    for character in text.chars() {
        match character {
            '"' => quoted.push_str("\\\""),
            '\\' => quoted.push_str("\\\\"),
            '\n' => quoted.push_str("\\n"),
            '\t' => quoted.push_str("\\t"),
            '\r' => quoted.push_str("\\r"),
            control if (control as u32) < 0x20 || control == '\u{7f}' => {
                quoted.push_str(&format!("\\u{:04x}", control as u32));
            }
            other => quoted.push(other),
        }
    }
    quoted.push('"');
    quoted
}

/// One argument of a [`Route`]: fixed text or a field of the request.
#[derive(Clone, Copy, Debug)]
pub enum Arg<'a> {
    Text(&'a str),
    Kind,
    TaskFile,
    TaskId,
    /// `request.prompt`: the prompt to deliver. With delivery disabled on 2.0
    /// it holds the bytes of `request.originalPrompt`.
    Prompt,
    /// `request.originalPrompt` on 2.0. 1.1 has no such field, so a route
    /// that names it is a 2.0 route.
    OriginalPrompt,
    /// The parameter object, as one JSON text.
    Params,
    Cwd,
}

impl Arg<'_> {
    fn render(self, runtime: Runtime) -> String {
        match (self, runtime) {
            (Arg::Text(text), _) => quote(text),
            (Arg::Kind, _) => "request.kind".to_owned(),
            (Arg::TaskFile, _) => "request.taskFile".to_owned(),
            (Arg::TaskId, _) => "request.taskId".to_owned(),
            (Arg::Prompt, _) => "request.prompt".to_owned(),
            (Arg::OriginalPrompt, Runtime::V2_0) => "request.originalPrompt".to_owned(),
            (Arg::OriginalPrompt, Runtime::V1_1) => {
                panic!("request.originalPrompt exists only in the 2.0 request")
            }
            (Arg::Params, Runtime::V1_1) => "JSON.stringify(request.params)".to_owned(),
            (Arg::Params, Runtime::V2_0) => "ravel::json_encode(request.params)".to_owned(),
            (Arg::Cwd, _) => "request.cwd".to_owned(),
        }
    }
}

/// A fixture configuration that selects one command for the kinds it routes,
/// and refuses every other kind with the owner code `incomplete_mapping`.
///
/// [`Route::source`] writes the same route for either runtime, so a case that
/// needs no other selection logic states it once.
#[derive(Clone, Debug)]
pub struct Route<'a> {
    pub version: &'a str,
    pub program: &'a str,
    pub args: Vec<Arg<'a>>,
    /// The routed kinds, or `None` for every kind.
    pub kinds: Option<Vec<&'a str>>,
    pub provider: &'a str,
    pub model: &'a str,
    pub effort: &'a str,
    pub reason: &'a str,
    /// The remedy of the refusal for a kind outside `kinds`.
    pub remedy: &'a str,
}

impl<'a> Route<'a> {
    /// A route for every kind, with fixture labels.
    pub fn new(version: &'a str, program: &'a str, args: Vec<Arg<'a>>) -> Self {
        Route {
            version,
            program,
            args,
            kinds: None,
            provider: "fixture",
            model: "none",
            effort: "none",
            reason: "the fixture's command",
            remedy: "add the kind to the policy",
        }
    }

    /// The configuration source for `runtime`.
    pub fn source(&self, runtime: Runtime) -> String {
        let args = self
            .args
            .iter()
            .map(|arg| arg.render(runtime))
            .collect::<Vec<_>>()
            .join(", ");
        let kinds = self.kinds.as_ref().map(|kinds| {
            kinds
                .iter()
                .map(|kind| quote(kind))
                .collect::<Vec<_>>()
                .join(", ")
        });
        let (program, version, reason, remedy) = (
            quote(self.program),
            quote(self.version),
            quote(self.reason),
            quote(self.remedy),
        );
        let labels = format!(
            "provider: {}, model: {}, effort: {}",
            quote(self.provider),
            quote(self.model),
            quote(self.effort)
        );
        match runtime {
            Runtime::V1_1 => {
                let guard = kinds.map_or(String::new(), |kinds| {
                    format!(
                        "    if (![{kinds}].includes(request.kind)) {{\n\
                         \x20     return {{ status: \"refused\", code: \"incomplete_mapping\", \
                         message: `this policy names no command for kind ${{JSON.stringify(request.kind)}}`, \
                         remedy: {remedy} }};\n\
                         \x20   }}\n"
                    )
                });
                format!(
                    "export const policy = {{\n\
                     \x20 schemaVersion: 2,\n\
                     \x20 version: {version},\n\
                     \x20 select(request) {{\n\
                     {guard}\
                     \x20   return {{ status: \"selected\", program: {program}, args: [{args}], \
                     {labels}, reason: {reason} }};\n\
                     \x20 }},\n\
                     }};\n"
                )
            }
            Runtime::V2_0 => {
                let guard = kinds.map_or(String::new(), |kinds| {
                    format!(
                        "    if ![{kinds}].contains(request.kind) {{\n\
                         \x20       return #{{ status: \"refused\", code: \"incomplete_mapping\", \
                         message: `this policy names no command for kind ${{ravel::json_encode(request.kind)}}`, \
                         remedy: {remedy} }};\n\
                         \x20   }}\n"
                    )
                });
                format!(
                    "export const SCHEMA_VERSION = 4;\n\
                     export const VERSION = {version};\n\
                     \n\
                     fn select(request, context) {{\n\
                     {guard}\
                     \x20   #{{ status: \"selected\", program: {program}, args: [{args}], \
                     {labels}, reason: {reason} }}\n\
                     }}\n"
                )
            }
        }
    }
}

pub struct InstalledRavel {
    executable: PathBuf,
    bin: PathBuf,
    runtime: Runtime,
}

/// The installation prefix of a front at `prefix/bin/ravel`.
fn prefix_of(front: &Path) -> PathBuf {
    let front = fs::canonicalize(front).unwrap_or_else(|error| {
        panic!(
            "GROVE_TEST_RAVEL names {}, which cannot be read: {error}",
            front.display()
        )
    });
    front
        .parent()
        .and_then(Path::parent)
        .expect("an installed Ravel front sits in prefix/bin")
        .to_path_buf()
}

impl InstalledRavel {
    /// Copy the layout of the build at `source` to a fresh fixture prefix: the
    /// front and its worker for 1.1, the one executable for 2.0. The runtime
    /// comes from `GROVE_TEST_RAVEL_RUNTIME`. The caller owns the
    /// destination's temporary-directory lifetime.
    pub fn copy_from(source: &Path, prefix: &Path) -> Self {
        Self::copy_as(source, prefix, Runtime::for_build(source))
    }

    /// [`InstalledRavel::copy_from`] for a build whose runtime the caller has
    /// already established, such as the second build of a pair case.
    pub fn copy_as(source: &Path, prefix: &Path, runtime: Runtime) -> Self {
        let source = fs::canonicalize(source).expect("explicit installed Ravel front");
        let source_prefix = prefix_of(&source);
        let bin = prefix.join("bin");
        fs::create_dir_all(&bin).expect("fixture Ravel bin directory");
        let executable = bin.join("ravel");
        let mut files = vec![(source.clone(), executable.clone())];
        if runtime == Runtime::V1_1 {
            let worker_dir = prefix.join("libexec/ravel");
            fs::create_dir_all(&worker_dir).expect("fixture Ravel worker directory");
            files.push((source_prefix.join(WORKER), worker_dir.join("ravel-policy")));
        }
        // Refuse an occupied destination; no fixture may overwrite another's build.
        for (from, to) in files {
            let mut input = fs::File::open(&from).unwrap_or_else(|error| {
                panic!("opening Ravel fixture source {}: {error}", from.display())
            });
            let mut output = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&to)
                .unwrap_or_else(|error| {
                    panic!("creating Ravel fixture file {}: {error}", to.display())
                });
            std::io::copy(&mut input, &mut output).expect("copying installed Ravel fixture file");
            output
                .set_permissions(
                    input
                        .metadata()
                        .expect("Ravel source metadata")
                        .permissions(),
                )
                .expect("preserving Ravel executable permissions");
        }
        Self {
            executable: fs::canonicalize(executable).expect("copied Ravel executable"),
            bin: fs::canonicalize(bin).expect("copied Ravel bin directory"),
            runtime,
        }
    }

    pub fn executable(&self) -> &Path {
        &self.executable
    }

    pub fn bin(&self) -> &Path {
        &self.bin
    }

    pub fn runtime(&self) -> Runtime {
        self.runtime
    }

    /// A process isolated from inherited session authority and host tool lookup.
    /// Tests may explicitly add their own HOME, policy, arguments and cwd.
    pub fn command(&self) -> Command {
        let mut command = Command::new(&self.executable);
        self.isolate(&mut command);
        command
    }

    /// Apply the same isolation to a caller binary or an entry-observation probe.
    pub fn isolate(&self, command: &mut Command) {
        command.env("PATH", &self.bin);
        for name in super::grove_env_names() {
            command.env_remove(name);
        }
    }
}

// The directory route on Ravel 2.0 through Grove's lifecycle caller.
//
// Every case drives the real `grove` driver with a temporary HOME of its own,
// a jj working tree and a copied installed Ravel build. Owner settings name
// local skill sources outside the working tree: one plain directory and one
// plain Git checkout at a commit. Native copies are absent. The configuration
// passes `request.prompt` to a fake agent. That agent takes the mandatory
// paths and the search directories from Ravel's preamble, and the skill name
// from Grove's mandate in the caller block, as a real agent must.
//
// The default-directory cases instead copy the 2.0 build into the keg of a
// Homebrew-shaped prefix and write its installation record. The record names
// the prefix's `share/ravel/skills`, which holds relative links to the source
// aliases of kegs. Those cases resolve in a HOME without `skills` settings.
//
// These fixtures prove delivery and Grove's interpretation of it, not the
// behaviour of an LLM. The real harness trials belong to acceptance.
//
// The cases need the 2.0 directory build, so each is ignored by default.
// `task ravel:directory RAVEL=<2.0 front> OLD=<1.1 front>` runs them:
// GROVE_TEST_RAVEL names the 2.0 build, GROVE_TEST_RAVEL_RUNTIME is `2.0` and
// GROVE_TEST_RAVEL_OLD names the 1.1 build of the pair case.

mod support;

use support::ravel::{quote, Arg, InstalledRavel, Route, Runtime};

use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Output, Stdio};
use std::time::{Duration, Instant};
use tempfile::TempDir;

/// The variable that names the 1.1 front of the pair case.
const OLD_VAR: &str = "GROVE_TEST_RAVEL_OLD";

/// The spine of the fixture core pack. Its one supporting resource is named
/// relative to the directory of the skill.
const SPINE: &str = "# grove spine (fixture)\nresource: references/execute.md\n";
const RESOURCE_V1: &str = "execute procedure, first revision\n";
const RESOURCE_V2: &str = "execute procedure, second revision\n";
const KIND_V1: &str = "# grove-impl (fixture), first revision\n";
const KIND_V2: &str = "# grove-impl (fixture), second revision\n";

/// The macOS sandbox profile that denies every network operation to the
/// process it starts and to all its descendants.
const NO_NETWORK: &str = "(version 1)(allow default)(deny network*)";

/// The 2.0 build under test. The directory route exists only there.
fn build() -> PathBuf {
    assert_eq!(
        support::runtime(),
        Runtime::V2_0,
        "the directory route needs the Ravel 2.0 directory build: set GROVE_TEST_RAVEL \
         to its front and GROVE_TEST_RAVEL_RUNTIME=2.0, or run `task ravel:directory`"
    );
    support::installed_ravel()
}

/// The 1.1 build of the pair case, copied once per test process.
fn old_build() -> &'static InstalledRavel {
    static BUILD: std::sync::OnceLock<(TempDir, InstalledRavel)> = std::sync::OnceLock::new();
    &BUILD
        .get_or_init(|| {
            let source = std::env::var_os(OLD_VAR).unwrap_or_else(|| {
                panic!("the pair case requires {OLD_VAR}=/absolute/ravel-1.1/bin/ravel")
            });
            let source = PathBuf::from(source);
            let runtime = Runtime::named(&source, OLD_VAR, "1.1");
            let temp = TempDir::new().unwrap();
            let build = InstalledRavel::copy_as(&source, &temp.path().join("prefix"), runtime);
            (temp, build)
        })
        .1
}

fn write_exec(path: &Path, body: &str) {
    fs::write(path, body).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o755)).unwrap();
}

fn shell_quote(path: &Path) -> String {
    let value = path.to_str().unwrap();
    assert!(!value.contains('\''), "test fixture path contains a quote");
    format!("'{value}'")
}

/// Plant a minimal current-format tree with one live leaf.
fn plant_tree(worktree: &Path, leaf: &str) {
    let grove = worktree.join(".grove");
    fs::create_dir_all(&grove).unwrap();
    fs::write(grove.join("_BRIEF.md"), "# g — brief\n").unwrap();
    fs::write(grove.join(leaf), "# planted\n").unwrap();
}

/// The shell functions of the fake agent. The harness script defines `$1` as
/// the delivered prompt and `$record` as this launch's evidence directory.
///
/// `load` makes the reads in the order the preamble sets: the first mandatory
/// file, the resource it names relative to its own directory, and then the
/// skill that the caller block names, found beneath the search directories.
/// `discover` is for a launch with no mandatory file: it finds the named
/// skill, then the spine by its name `grove`, beneath the search directories,
/// and then the resource that the spine names. Each read appends its path to
/// `$record/reads` and copies the bytes to `$record/<label>`. `finish` records teardown and runs the exact continue
/// command from the caller block. `continue_` runs it without teardown.
const AGENT: &str = r#"preamble=$(printf '%s\n' "$1" | sed '/^Caller task$/q')
block=$(printf '%s\n' "$1" | sed -n '/^BEGIN RAVEL CALLER TASK /,/^END RAVEL CALLER TASK /p')
[ -n "$block" ] || block=$1
nl='
'
read_as() { printf '%s\n' "$2" >> "$record/reads"; cat "$2" > "$record/$1" || exit 95; }
mandatory() { printf '%s\n' "$preamble" | sed -n 's/^[0-9][0-9]*\. "\(.*\)"$/\1/p'; }
directories() { printf '%s\n' "$preamble" | sed -n 's/^- "\(.*\)"$/\1/p'; }
named_skill() { printf '%s\n' "$block" | sed -n 's/^\*\*Load the `\([^`]*\)` skill now\.\*\*.*/\1/p' | head -n 1; }
find_skill() {
    IFS=$nl
    for dir in $(directories); do
        if [ -f "$dir/$1/SKILL.md" ]; then unset IFS; printf '%s\n' "$dir/$1/SKILL.md"; return 0; fi
    done
    unset IFS
    return 1
}
ending() { printf '%s\n' "$block" | grep -o "\`'[^']*' end-session $1\`" | head -n 1 | tr -d '`'; }
load() {
    spine=$(mandatory | head -n 1)
    [ -n "$spine" ] || exit 94
    read_as spine "$spine"
    resource="$(dirname "$spine")/$(sed -n 's/^resource: //p' "$spine")"
    read_as resource "$resource"
    skill=$(named_skill)
    [ -n "$skill" ] || exit 93
    kind=$(find_skill "$skill") || exit 92
    read_as kind "$kind"
}
discover() {
    skill=$(named_skill)
    [ -n "$skill" ] || exit 93
    kind=$(find_skill "$skill") || exit 92
    read_as kind "$kind"
    spine=$(find_skill grove) || exit 91
    read_as spine "$spine"
    resource="$(dirname "$spine")/$(sed -n 's/^resource: //p' "$spine")"
    read_as resource "$resource"
}
finish() { touch "$GROVE_LAUNCH_DIR/teardown"; eval "exec $(ending grove-continue)"; }
continue_() { eval "exec $(ending grove-continue)"; }
"#;

/// A shell fragment that waits up to a minute for `file`, then gives up with
/// exit 96 so a broken case cannot hang the suite.
fn wait_for(file: &Path) -> String {
    format!(
        "i=0; until [ -e {file} ]; do i=$((i + 1)); [ $i -lt 1200 ] || exit 96; sleep 0.05; done\n",
        file = shell_quote(file)
    )
}

/// One route's fixture: its own HOME, a jj working tree, a fake agent and the
/// Ravel build it launches through.
struct Lane {
    _temp: TempDir,
    root: PathBuf,
    home: PathBuf,
    worktree: PathBuf,
    harness: PathBuf,
    launches: PathBuf,
    /// Where a 2.0 configuration hands the request fields it observed.
    view: PathBuf,
    ravel: PathBuf,
    bin: PathBuf,
}

impl Lane {
    fn new(ravel: &Path) -> Self {
        let temp = TempDir::new().unwrap();
        let root = fs::canonicalize(temp.path()).unwrap();
        let worktree = root.join("worktree");
        fs::create_dir_all(&worktree).unwrap();
        support::init_jj_repo(&worktree);
        let home = root.join("home");
        fs::create_dir_all(&home).unwrap();
        let launches = root.join("launches");
        fs::create_dir(&launches).unwrap();
        Lane {
            harness: root.join("fake-agent"),
            view: root.join("view.json"),
            ravel: ravel.to_path_buf(),
            bin: ravel.parent().unwrap().to_path_buf(),
            _temp: temp,
            root,
            home,
            worktree,
            launches,
        }
    }

    /// Write the fake agent. It records its start, then runs `then`, a shell
    /// fragment in which `$n` is this start's number, counted from 0.
    fn agent(&self, then: &str) {
        write_exec(
            &self.harness,
            &format!(
                "#!/bin/sh\n\
                 n=0\n\
                 until mkdir {launches}/$n 2>/dev/null; do n=$((n + 1)); [ $n -lt 10 ] || exit 90; done\n\
                 record={launches}/$n\n\
                 if [ -f {view} ]; then mv {view} \"$record/view.json\"; fi\n\
                 printf '%s' \"$1\" > \"$record/prompt\"\n\
                 printf '%s' \"${{RAVEL_SKILL_DIRECTORIES-<unset>}}\" > \"$record/descriptor-env\"\n\
                 {AGENT}\
                 {then}\n",
                launches = shell_quote(&self.launches),
                view = shell_quote(&self.view),
            ),
        );
    }

    /// The 2.0 configuration of the directory route. It hands
    /// `request.originalPrompt` and `request.skills` to `/bin/sh`, which writes
    /// them to the view and execs the fake agent with `request.prompt`.
    fn directory_config(&self) {
        let write_view = r#"printf '%s' "$1" > "$2"; shift 2; exec "$0" "$@""#;
        self.config(&format!(
            "export const SCHEMA_VERSION = 4;\n\
             export const VERSION = \"directory-route-1\";\n\
             \n\
             fn select(request, context) {{\n\
             \x20   let view = #{{ originalPrompt: request.originalPrompt, skills: request.skills }};\n\
             \x20   #{{\n\
             \x20       status: \"selected\",\n\
             \x20       program: \"/bin/sh\",\n\
             \x20       args: [\"-c\", {write_view}, {harness}, ravel::json_encode(view), {view}, request.prompt],\n\
             \x20       provider: \"fixture\",\n\
             \x20       model: \"none\",\n\
             \x20       effort: \"none\",\n\
             \x20       reason: \"the directory route's fake agent\",\n\
             \x20   }}\n\
             }}\n",
            write_view = quote(write_view),
            harness = quote(self.harness.to_str().unwrap()),
            view = quote(self.view.to_str().unwrap()),
        ));
    }

    /// The 1.1 policy of the native route: the fake agent with the prompt.
    fn native_policy(&self) {
        let route = Route::new(
            "native-route-1",
            self.harness.to_str().unwrap(),
            vec![Arg::Prompt],
        );
        self.write_ravel_file("policy.ts", &route.source(Runtime::V1_1));
    }

    fn config(&self, source: &str) {
        self.write_ravel_file("config.rhai", source);
    }

    fn settings(&self, json: &serde_json::Value) {
        self.write_ravel_file("settings.json", &json.to_string());
    }

    fn write_ravel_file(&self, name: &str, text: &str) {
        let file = self.home.join(".config/ravel").join(name);
        fs::create_dir_all(file.parent().unwrap()).unwrap();
        fs::write(file, text).unwrap();
    }

    /// The PATH of this lane: `first`, then the Ravel build, then the host's.
    fn path(&self, first: Option<&Path>) -> std::ffi::OsString {
        std::env::join_paths(
            first
                .map(Path::to_path_buf)
                .into_iter()
                .chain(std::iter::once(self.bin.clone()))
                .chain(std::env::split_paths(&std::env::var_os("PATH").unwrap())),
        )
        .unwrap()
    }

    /// The driver of this lane, with no inherited session authority and no
    /// terminal. `guard` names a sandbox profile to start it under, and
    /// `first` a directory to put first on its PATH.
    fn driver(&self, guard: Option<&str>, first: Option<&Path>) -> Command {
        let grove = env!("CARGO_BIN_EXE_grove");
        let mut command = match guard {
            Some(profile) => {
                let mut command = Command::new("/usr/bin/sandbox-exec");
                command.args(["-p", profile, grove]);
                command
            }
            None => Command::new(grove),
        };
        command.current_dir(&self.worktree);
        for name in support::grove_env_names() {
            command.env_remove(name);
        }
        for name in [
            "HARNESS_DISPATCH_RUN_ID",
            "HARNESS_DISPATCH_STATE_DIR",
            "RAVEL_SKILL_DIRECTORIES",
        ] {
            command.env_remove(name);
        }
        command
            .env("HOME", &self.home)
            .env("PATH", self.path(first))
            .stdin(Stdio::null());
        // SAFETY: setsid is async-signal-safe, and this child is not yet a
        // process-group leader. Failure reaches Command's spawn error.
        unsafe {
            command.pre_exec(|| {
                if libc::setsid() == -1 {
                    return Err(std::io::Error::last_os_error());
                }
                Ok(())
            });
        }
        command
    }

    fn spawn(&self, command: Command) -> Driver {
        Driver::capture(command, &self.root)
    }

    fn run(&self) -> Output {
        self.spawn(self.driver(None, None)).finish()
    }

    /// Ravel run directly under this lane's HOME and working tree.
    fn direct(&self) -> Output {
        Command::new(&self.ravel)
            .args(["start-session", "--kind=impl", "--prompt=control"])
            .env_clear()
            .env("HOME", &self.home)
            .env("PATH", self.path(None))
            .current_dir(&self.worktree)
            .output()
            .unwrap()
    }

    fn launch_count(&self) -> usize {
        fs::read_dir(&self.launches).unwrap().count()
    }

    fn record(&self, n: usize) -> PathBuf {
        self.launches.join(n.to_string())
    }

    fn read(&self, n: usize, name: &str) -> String {
        let file = self.record(n).join(name);
        fs::read_to_string(&file).unwrap_or_else(|error| panic!("{}: {error}", file.display()))
    }

    fn view(&self, n: usize) -> serde_json::Value {
        serde_json::from_str(&self.read(n, "view.json")).unwrap()
    }

    fn task_file(&self, leaf: &str) -> PathBuf {
        self.worktree.join(".grove").join(leaf)
    }

    /// The mandate Grove composes for `handle` of kind `kind` in this lane.
    fn mandate(&self, handle: &str, kind: &str) -> String {
        let root = fs::canonicalize(&self.worktree).unwrap();
        grove_loop::compose(&grove_loop::Mandate {
            handle: &grove_loop::Handle::parse(handle).unwrap(),
            kind: &grove_loop::Kind::new(kind).unwrap(),
            workspace: &grove_loop::Workspace::resolve(&root).unwrap(),
            version: grove_loop::VERSION,
            ending_command: &grove_loop::session_command::end_session(&self.ravel).unwrap(),
            force_stop_command: &grove_loop::session_command::force_stop(&self.ravel).unwrap(),
        })
    }
}

/// A driver process whose streams go to files, so that a case can read what
/// it said while it runs.
struct Driver {
    child: Child,
    stderr: PathBuf,
    stdout: PathBuf,
}

impl Driver {
    fn capture(mut command: Command, root: &Path) -> Self {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let capture = root.join(format!("driver-{n}"));
        fs::create_dir(&capture).unwrap();
        let stdout = capture.join("stdout");
        let stderr = capture.join("stderr");
        let child = command
            .stdout(Stdio::from(fs::File::create(&stdout).unwrap()))
            .stderr(Stdio::from(fs::File::create(&stderr).unwrap()))
            .spawn()
            .unwrap();
        Driver {
            child,
            stderr,
            stdout,
        }
    }

    fn wait_for_ready(&mut self, marker: &Path) {
        support::wait_for_ready(marker, &mut self.child, Some(&self.stderr));
    }

    fn finish(&mut self) -> Output {
        let deadline = Instant::now() + Duration::from_secs(180);
        while self.child.try_wait().unwrap().is_none() {
            if Instant::now() >= deadline {
                let _ = self.child.kill();
                panic!(
                    "the driver was still running after three minutes: {}",
                    fs::read_to_string(&self.stderr).unwrap_or_default()
                );
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        Output {
            status: self.child.wait().unwrap(),
            stdout: fs::read(&self.stdout).unwrap_or_default(),
            stderr: fs::read(&self.stderr).unwrap_or_default(),
        }
    }
}

impl Drop for Driver {
    fn drop(&mut self) {
        if self.child.try_wait().unwrap_or(None).is_none() {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

/// What the driver said, after it finished the loop on a session's teardown.
fn completed(output: &Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(output.status.success(), "{stderr}");
    assert!(
        stderr.contains("grove finished — loop complete"),
        "the loop did not end on its session's teardown: {stderr}"
    );
    stderr
}

/// The two local sources outside the working tree: the core pack as a plain
/// directory, and the engineering pack as a plain Git checkout at a commit.
struct Sources {
    core: PathBuf,
    engineering: PathBuf,
    commit: String,
}

impl Sources {
    fn plant(lane: &Lane) -> Self {
        let core = lane.root.join("sources/core");
        fs::create_dir_all(core.join("skills/grove/references")).unwrap();
        fs::write(core.join("skills/grove/SKILL.md"), SPINE).unwrap();
        fs::write(core.join("skills/grove/references/execute.md"), RESOURCE_V1).unwrap();

        let engineering = lane.root.join("sources/engineering");
        fs::create_dir_all(engineering.join("skills/grove-impl")).unwrap();
        fs::write(engineering.join("skills/grove-impl/SKILL.md"), KIND_V1).unwrap();
        let git = |args: &[&str]| {
            let output = Command::new("git")
                .args([
                    "-c",
                    "user.name=Grove Test",
                    "-c",
                    "user.email=t@example.com",
                ])
                .args(args)
                .current_dir(&engineering)
                .env("GIT_CONFIG_GLOBAL", "/dev/null")
                .env("GIT_CONFIG_NOSYSTEM", "1")
                .output()
                .unwrap();
            assert!(output.status.success(), "git {args:?}: {output:?}");
            String::from_utf8(output.stdout).unwrap().trim().to_owned()
        };
        git(&["init", "--quiet"]);
        git(&["add", "--all"]);
        git(&["commit", "--quiet", "--message", "engineering pack"]);
        let commit = git(&["rev-parse", "HEAD"]);
        assert_eq!(git(&["status", "--porcelain"]), "", "the checkout is clean");
        Sources {
            core,
            engineering,
            commit,
        }
    }

    /// Owner settings for the directory route: both sources, with the spine
    /// as the one mandatory file.
    fn settings(&self) -> serde_json::Value {
        serde_json::json!({
            "skills": {
                "schemaVersion": 1,
                "sources": [
                    { "id": "core", "type": "local", "path": self.core, "roots": ["skills"] },
                    { "id": "engineering", "type": "local", "path": self.engineering, "roots": ["skills"] },
                ],
                "mandatory": [{ "source": "core", "path": "skills/grove/SKILL.md" }],
            }
        })
    }
}

/// The caller block of a prepared prompt: the bytes between the line
/// `BEGIN RAVEL CALLER TASK <nonce>` and the newline before
/// `END RAVEL CALLER TASK <nonce>`. That newline belongs to Ravel's wrapper.
fn caller_block(prompt: &str) -> Result<&str, String> {
    const BEGIN: &str = "\nCaller task\nBEGIN RAVEL CALLER TASK ";
    let start = prompt
        .find(BEGIN)
        .ok_or_else(|| format!("no caller block in {prompt:?}"))?
        + BEGIN.len();
    let nonce_end = start
        + prompt[start..]
            .find('\n')
            .ok_or("the opening delimiter has no end")?;
    let nonce = &prompt[start..nonce_end];
    if nonce.len() != 32 || !nonce.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("the nonce {nonce:?} is not 32 hexadecimal digits"));
    }
    let close = format!("\nEND RAVEL CALLER TASK {nonce}\n");
    if !prompt.ends_with(&close) {
        return Err(format!("the prompt does not end with {close:?}"));
    }
    Ok(&prompt[nonce_end + 1..prompt.len() - close.len()])
}

/// Every path the fake agent read lies inside a retained snapshot that the
/// request names, and none lies in a configured source or a native skill
/// directory under HOME.
fn reads_from_snapshots(reads: &str, skills: &serde_json::Value) -> Result<(), String> {
    let roots = |field: &str| -> Vec<PathBuf> {
        skills["sources"]
            .as_array()
            .unwrap()
            .iter()
            .map(|source| fs::canonicalize(source[field].as_str().unwrap()).unwrap())
            .collect()
    };
    let snapshots = roots("snapshotRoot");
    let origins = roots("originPath");
    if reads.lines().count() < 3 {
        return Err(format!("the agent made fewer than three reads: {reads:?}"));
    }
    for read in reads.lines() {
        let path = fs::canonicalize(read).map_err(|error| format!("{read}: {error}"))?;
        if !snapshots.iter().any(|root| path.starts_with(root)) {
            return Err(format!(
                "{read} is outside every retained snapshot {snapshots:?}"
            ));
        }
        if origins.iter().any(|origin| path.starts_with(origin)) {
            return Err(format!("{read} is inside a configured source"));
        }
    }
    Ok(())
}

/// No `SKILL.md` exists under `home` outside Ravel's skill cache.
fn no_native_copy(home: &Path) -> Result<(), String> {
    let cache = home.join(".cache/ravel/skills");
    let mut pending = vec![home.to_path_buf()];
    while let Some(dir) = pending.pop() {
        if dir.starts_with(&cache) {
            continue;
        }
        for entry in fs::read_dir(&dir).unwrap() {
            let entry = entry.unwrap();
            let kind = entry.file_type().unwrap();
            if kind.is_dir() {
                pending.push(entry.path());
            } else if entry.file_name() == "SKILL.md" {
                return Err(format!(
                    "a native copy exists at {}",
                    entry.path().display()
                ));
            }
        }
    }
    Ok(())
}

/// The checks every successful directory launch shares: the delivered prompt,
/// the request fields, the reads and the absence of native copies.
fn assert_directory_launch(lane: &Lane, n: usize, handle: &str) -> serde_json::Value {
    let prompt = lane.read(n, "prompt");
    let view = lane.view(n);
    let mandate = lane.mandate(handle, "impl");
    assert!(
        mandate.contains("Do this **first**, after any mandatory guidance and before"),
        "the mandate qualifies \"first\": {mandate}"
    );
    assert_eq!(
        view["originalPrompt"].as_str(),
        Some(mandate.as_str()),
        "request.originalPrompt is the composed mandate byte for byte"
    );
    assert_eq!(
        caller_block(&prompt),
        Ok(mandate.as_str()),
        "the caller block is the composed mandate byte for byte"
    );
    for command in [
        grove_loop::session_command::end_session(&lane.ravel).unwrap(),
        grove_loop::session_command::force_stop(&lane.ravel).unwrap(),
    ] {
        assert!(
            caller_block(&prompt)
                .unwrap()
                .contains(&format!("`{command}`")),
            "the caller block holds the exact ending {command}"
        );
    }

    let skills = &view["skills"];
    let mandatory = skills["mandatory"][0]["path"].as_str().unwrap();
    let names_mandatory = prompt
        .find(&format!(
            "Mandatory files, in order:\n1. {}\n",
            quote(mandatory)
        ))
        .expect("the preamble names the mandatory file");
    let caller = prompt
        .find("\nCaller task\nBEGIN RAVEL CALLER TASK ")
        .unwrap();
    assert!(
        names_mandatory < caller,
        "the preamble names the mandatory reads before the caller block"
    );
    assert!(
        prompt[..caller].contains("Read the mandatory files in the order above")
            && prompt[..caller].contains("before any task action"),
        "the preamble orders the mandatory reads before task actions: {prompt}"
    );
    assert_eq!(
        lane.read(n, "descriptor-env"),
        skills["descriptorPath"].as_str().unwrap(),
        "the agent's environment names this launch's descriptor"
    );

    for source in skills["sources"].as_array().unwrap() {
        let origin = Path::new(source["originPath"].as_str().unwrap());
        assert!(
            !origin.starts_with(fs::canonicalize(&lane.worktree).unwrap()),
            "the source {origin:?} is outside cwd"
        );
    }

    let reads = lane.read(n, "reads");
    assert_eq!(
        reads.lines().next(),
        Some(mandatory),
        "the first read is the mandatory file"
    );
    reads_from_snapshots(&reads, skills).unwrap();
    no_native_copy(&lane.home).unwrap();
    assert_eq!(lane.read(n, "spine"), SPINE);
    view
}

// A lifecycle launch on the 2.0 build uses `skills` settings in a HOME of its
// own. One source is a plain directory and one is a plain Git checkout at a
// commit, both outside the working tree. No native copy exists in that HOME.
// The fake agent reads the spine, its supporting resource and the named kind
// from the injected directories, and ends with the exact continue command
// from the caller block. `request.originalPrompt` and the caller block each
// equal the composed mandate byte for byte, and the preamble names the
// mandatory reads before the caller block.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_directory_launch_reads_the_spine_resource_and_kind_from_sources_outside_cwd() {
    let lane = Lane::new(&build());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    let sources = Sources::plant(&lane);
    lane.settings(&sources.settings());
    lane.directory_config();
    lane.agent("load\nfinish");

    completed(&lane.run());
    assert_eq!(lane.launch_count(), 1);
    let view = assert_directory_launch(&lane, 0, "subject-k1");
    assert_eq!(lane.read(0, "resource"), RESOURCE_V1);
    assert_eq!(lane.read(0, "kind"), KIND_V1);

    let engineering = view["skills"]["sources"]
        .as_array()
        .unwrap()
        .iter()
        .find(|source| source["id"] == "engineering")
        .unwrap();
    assert_eq!(
        Path::new(engineering["originPath"].as_str().unwrap()),
        sources.engineering,
        "the checkout at {} is the engineering source",
        sources.commit
    );
    assert!(sources.engineering.join(".git").is_dir());
    assert!(
        !Path::new(engineering["snapshotRoot"].as_str().unwrap())
            .join(".git")
            .exists(),
        "the snapshot of the checkout leaves out its .git"
    );

    // The comparison is byte exact: a caller block that differs by one byte
    // from the mandate fails it.
    let prompt = lane.read(0, "prompt");
    let mandate = lane.mandate("subject-k1", "impl");
    let inside = prompt.find("Grove mandate: the leaf").unwrap();
    let mut altered = prompt.clone().into_bytes();
    altered[inside] ^= 0x20;
    let altered = String::from_utf8(altered).unwrap();
    assert_ne!(caller_block(&altered), Ok(mandate.as_str()));
}

// The control on the read check. The same launch, with a native copy of the
// kind planted in HOME and a fake agent that reads it there, fails both the
// snapshot check and the native-copy check.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_fake_agent_that_reads_a_native_copy_fails_the_snapshot_check() {
    let lane = Lane::new(&build());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    let sources = Sources::plant(&lane);
    lane.settings(&sources.settings());
    lane.directory_config();
    let native = lane.home.join(".claude/skills/grove-impl/SKILL.md");
    fs::create_dir_all(native.parent().unwrap()).unwrap();
    fs::write(&native, KIND_V1).unwrap();
    lane.agent(&format!(
        "spine=$(mandatory | head -n 1)\n\
         read_as spine \"$spine\"\n\
         read_as resource \"$(dirname \"$spine\")/references/execute.md\"\n\
         read_as kind {native}\n\
         finish",
        native = shell_quote(&native)
    ));

    completed(&lane.run());
    let view = lane.view(0);
    let refused = reads_from_snapshots(&lane.read(0, "reads"), &view["skills"]).unwrap_err();
    assert!(
        refused.contains(".claude/skills/grove-impl/SKILL.md"),
        "{refused}"
    );
    assert!(no_native_copy(&lane.home).is_err());
}

// A missing checkout refuses as `skill_source_missing` before any worker.
// The configuration script is not valid Rhai, so a worker that ran would
// refuse differently. Grove reports Ravel's first line and remedy as the same
// build states them directly, the leaf stays live, no agent starts and the
// loop does not relaunch. Restored, the same source reaches the script, which
// then refuses for its own reason.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_missing_checkout_refuses_before_any_worker_and_leaves_the_leaf_live() {
    let lane = Lane::new(&build());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    let sources = Sources::plant(&lane);
    lane.settings(&sources.settings());
    lane.config("this is not a configuration script\n");
    lane.agent("finish");
    let moved = lane.root.join("moved-checkout");
    fs::rename(&sources.engineering, &moved).unwrap();

    let direct = lane.direct();
    let stated = String::from_utf8_lossy(&direct.stderr).into_owned();
    assert_eq!(direct.status.code(), Some(3), "{stated}");
    let first = stated.lines().next().unwrap();
    assert!(
        first.starts_with("ravel: refused (skill_source_missing, stage skills)"),
        "{stated}"
    );
    assert!(
        stated.contains(sources.engineering.to_str().unwrap()),
        "{stated}"
    );
    let remedy = stated
        .lines()
        .find(|line| line.starts_with("  remedy: "))
        .unwrap_or_else(|| panic!("no remedy in {stated}"));

    let output = lane.run();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    for said in [
        first,
        remedy,
        "session kind `impl` for `subject-k1` failed",
        "rerun `grove` to continue",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    assert_eq!(
        stderr
            .matches("session kind `impl` for `subject-k1`")
            .count(),
        1,
        "the loop does not relaunch: {stderr}"
    );
    assert_eq!(lane.launch_count(), 0, "{stderr}");
    assert!(lane.task_file("01-impl--subject-k1.md").is_file());
    assert!(!stderr.contains("grove finished"), "{stderr}");

    fs::rename(&moved, &sources.engineering).unwrap();
    let restored = lane.direct();
    let restored = String::from_utf8_lossy(&restored.stderr);
    assert!(
        restored.starts_with("ravel: refused (") && !restored.contains("skill_source_missing"),
        "with the source restored, the script refuses for its own reason: {restored}"
    );
}

// One directory launch runs under a sandbox profile that denies the network
// to the driver, Ravel and the agent. A spy `git` first on the PATH records
// no call while the checkout resolves and the agent reads it. Two controls
// are seen to fail in the same case: an unguarded `nc` reaches a local
// listener that the guarded agent cannot reach, and the spy records an
// ordinary `git --version` on the same PATH.
#[cfg(target_os = "macos")]
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_directory_launch_needs_no_network_and_no_git() {
    let lane = Lane::new(&build());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    let sources = Sources::plant(&lane);
    lane.settings(&sources.settings());
    lane.directory_config();

    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = listener.local_addr().unwrap().port().to_string();
    let nc = |guard: Option<&str>| {
        let mut command = match guard {
            Some(profile) => {
                let mut command = Command::new("/usr/bin/sandbox-exec");
                command.args(["-p", profile, "/usr/bin/nc"]);
                command
            }
            None => Command::new("/usr/bin/nc"),
        };
        command
            .args(["-z", "-w", "2", "127.0.0.1", &port])
            .output()
            .unwrap()
            .status
            .success()
    };
    assert!(nc(None), "an unguarded probe reaches the listener");
    assert!(!nc(Some(NO_NETWORK)), "the guard denies the same probe");

    let spy = lane.root.join("spy");
    fs::create_dir(&spy).unwrap();
    let calls = lane.root.join("git-calls");
    write_exec(
        &spy.join("git"),
        &format!(
            "#!/bin/sh\nprintf '%s\\n' \"$*\" >> {calls}\nexit 97\n",
            calls = shell_quote(&calls)
        ),
    );
    let reached = Command::new("git")
        .arg("--version")
        .env("PATH", lane.path(Some(&spy)))
        .output()
        .unwrap();
    assert_eq!(reached.status.code(), Some(97));
    assert_eq!(fs::read_to_string(&calls).unwrap(), "--version\n");
    fs::remove_file(&calls).unwrap();

    lane.agent(&format!(
        "load\n\
         /usr/bin/nc -z -w 2 127.0.0.1 {port} > /dev/null 2>&1; printf '%s' $? > \"$record/network\"\n\
         finish"
    ));
    completed(
        &lane
            .spawn(lane.driver(Some(NO_NETWORK), Some(&spy)))
            .finish(),
    );
    assert_directory_launch(&lane, 0, "subject-k1");
    assert_eq!(lane.read(0, "kind"), KIND_V1);
    assert_ne!(
        lane.read(0, "network"),
        "0",
        "the agent ran under the guard and could not reach the listener"
    );
    assert!(
        !calls.exists(),
        "the launch called git: {}",
        fs::read_to_string(&calls).unwrap_or_default()
    );
    drop(listener);
}

// A native lifecycle launch on 1.1 and a directory launch on 2.0 run at the
// same time, each under its own HOME. Each agent waits until both have
// started, so neither session can end before the other runs. The 1.1 session
// receives the mandate unchanged and ends with its own 1.1 ending command.
// Then the 1.1 HOME receives `skills` settings, and the next 1.1 launch
// refuses as `settings_invalid` with Ravel's statement and leaves the leaf
// live.
#[test]
#[ignore = "needs the Ravel 2.0 directory build and a 1.1 build: task ravel:directory"]
fn a_native_launch_on_1_1_and_a_directory_launch_on_2_0_run_at_once_in_separate_homes() {
    let directory = Lane::new(&build());
    let native = Lane::new(old_build().executable());
    assert_ne!(directory.home, native.home);
    let barrier = TempDir::new().unwrap();
    let barrier = fs::canonicalize(barrier.path()).unwrap();
    let (one, two) = (barrier.join("native"), barrier.join("directory"));
    let meet = |own: &Path| {
        format!(
            "touch {own}\n{}{}",
            wait_for(&one),
            wait_for(&two),
            own = shell_quote(own)
        )
    };

    plant_tree(&native.worktree, "01-impl--subject-k1.md");
    native.native_policy();
    native.agent(&format!("{}finish", meet(&one)));

    plant_tree(&directory.worktree, "01-impl--subject-k1.md");
    let sources = Sources::plant(&directory);
    directory.settings(&sources.settings());
    directory.directory_config();
    directory.agent(&format!("load\n{}finish", meet(&two)));

    let mut native_driver = native.spawn(native.driver(None, None));
    let mut directory_driver = directory.spawn(directory.driver(None, None));
    completed(&native_driver.finish());
    completed(&directory_driver.finish());

    assert_eq!(native.launch_count(), 1);
    assert_eq!(
        native.read(0, "prompt"),
        native.mandate("subject-k1", "impl"),
        "the 1.1 session receives the mandate unchanged"
    );
    assert_eq!(native.read(0, "descriptor-env"), "<unset>");
    assert!(!native.home.join(".config/ravel/settings.json").exists());
    no_native_copy(&native.home).unwrap();
    assert_eq!(directory.launch_count(), 1);
    assert_directory_launch(&directory, 0, "subject-k1");
    assert!(!native.home.join(".cache/ravel").exists());

    // The 1.1 HOME with `skills` settings refuses, and the leaf stays live.
    native.settings(&sources.settings());
    let direct = native.direct();
    let stated = String::from_utf8_lossy(&direct.stderr).into_owned();
    assert_eq!(direct.status.code(), Some(2), "{stated}");
    let first = stated.lines().next().unwrap();
    assert!(
        first.starts_with("ravel: refused (settings_invalid, stage cli)"),
        "{stated}"
    );
    let remedy = stated
        .lines()
        .find(|line| line.starts_with("  remedy: "))
        .unwrap_or_else(|| panic!("no remedy in {stated}"));
    let output = native.run();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    for said in [
        first,
        remedy,
        "session kind `impl` for `subject-k1` failed",
        "rerun `grove` to continue",
    ] {
        assert!(stderr.contains(said), "no {said:?} in: {stderr}");
    }
    assert_eq!(native.launch_count(), 1, "{stderr}");
    assert!(native.task_file("01-impl--subject-k1.md").is_file());
}

// A running session keeps its resolved directories while its sources and the
// settings change. The first session reads the first revision, waits while
// the case changes the kind in the checkout, the resource in the plain
// directory and the settings, then reads its own paths again and still sees
// the first revision. It continues without teardown, so the loop launches
// the same live leaf again. That next launch receives a new descriptor and
// the second revision.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_running_session_keeps_its_directories_and_the_next_launch_sees_the_new_view() {
    let lane = Lane::new(&build());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    let sources = Sources::plant(&lane);
    lane.settings(&sources.settings());
    lane.directory_config();
    let edited = lane.root.join("edited");
    lane.agent(&format!(
        "load\n\
         if [ $n = 0 ]; then\n\
         touch \"$record/ready\"\n\
         {wait}\
         cat \"$kind\" > \"$record/kind-after\"\n\
         cat \"$resource\" > \"$record/resource-after\"\n\
         cat \"$RAVEL_SKILL_DIRECTORIES\" > \"$record/descriptor-after\"\n\
         continue_\n\
         fi\n\
         finish",
        wait = wait_for(&edited)
    ));

    let mut driver = lane.spawn(lane.driver(None, None));
    driver.wait_for_ready(&lane.record(0).join("ready"));
    let descriptor = lane.read(0, "descriptor-env");
    let before = fs::read_to_string(&descriptor).unwrap();
    fs::write(
        sources.engineering.join("skills/grove-impl/SKILL.md"),
        KIND_V2,
    )
    .unwrap();
    fs::write(
        sources.core.join("skills/grove/references/execute.md"),
        RESOURCE_V2,
    )
    .unwrap();
    let mut settings = sources.settings();
    settings["skills"]["resolutionTimeoutMs"] = serde_json::json!(60_000);
    lane.settings(&settings);
    fs::write(&edited, "").unwrap();
    completed(&driver.finish());

    assert_eq!(lane.launch_count(), 2);
    assert_directory_launch(&lane, 0, "subject-k1");
    assert_eq!(lane.read(0, "kind"), KIND_V1);
    assert_eq!(lane.read(0, "kind-after"), KIND_V1);
    assert_eq!(lane.read(0, "resource-after"), RESOURCE_V1);
    assert_eq!(lane.read(0, "descriptor-after"), before);

    assert_directory_launch(&lane, 1, "subject-k1");
    assert_eq!(lane.read(1, "kind"), KIND_V2);
    assert_eq!(lane.read(1, "resource"), RESOURCE_V2);
    let next = lane.read(1, "descriptor-env");
    assert_ne!(
        next, descriptor,
        "the next launch receives a new descriptor"
    );
    assert_eq!(
        fs::read_to_string(&descriptor).unwrap(),
        before,
        "the first descriptor is retained unchanged"
    );
}

/// The evidence files that the standalone agent writes in its working
/// directory. Grove publishes each one as an output.
const EVIDENCE: [&str; 9] = [
    "prompt",
    "view.json",
    "cwd",
    "reads",
    "spine",
    "resource",
    "kind",
    "probes",
    "started",
];

/// The probes of the standalone case. Each line is `name=ok` when the
/// operation succeeds and `name=denied` when it fails. The arguments are the
/// directory to write in, the snapshot spine, the snapshot kind and the
/// descriptor. Nothing confines the agent, so the snapshot writes fail only by
/// the file modes that Ravel publishes.
const PROBES: &str = "probe() { name=$1; shift; if \"$@\" > /dev/null 2>&1; then echo \"$name=ok\"; else echo \"$name=denied\"; fi; }\n\
     append() { printf x >> \"$1\"; }\n\
     probe descriptor-read cat \"$4\"\n\
     probe cwd-write touch \"$1/probe\"\n\
     probe snapshot-write touch \"$(dirname \"$2\")/probe\"\n\
     probe snapshot-append append \"$3\"\n";

impl Lane {
    /// The standalone agent. Its records go to its working directory, which
    /// Grove exports. It loads the skills as the
    /// lifecycle agent does, runs the probes and acknowledges with the exact
    /// command of the standalone instructions.
    fn standalone_agent(&self, probes: &Path) {
        write_exec(
            &self.harness,
            &format!(
                "#!/bin/sh\n\
                 record=$(pwd -P)\n\
                 printf 'fake agent: started\\n' >&2\n\
                 printf started > \"$record/started\"\n\
                 printf '%s' \"$record\" > \"$record/cwd\"\n\
                 printf '%s' \"$1\" > \"$record/prompt\"\n\
                 printf '%s' \"$2\" > \"$record/view.json\"\n\
                 {AGENT}\
                 load\n\
                 /bin/sh {probes} \"$record\" \"$spine\" \"$kind\" \"$RAVEL_SKILL_DIRECTORIES\" > \"$record/probes\"\n\
                 eval \"$(printf '%s\\n' \"$1\" | sed -n '/exact command as your final action:/{{n;p;}}')\"\n",
                probes = shell_quote(probes),
            ),
        );
    }

    /// The 2.0 configuration of the standalone case. The agent receives
    /// `request.prompt`, then `request.originalPrompt` and `request.skills` as
    /// one JSON text.
    fn standalone_config(&self) {
        self.config(&format!(
            "export const SCHEMA_VERSION = 4;\n\
             export const VERSION = \"directory-standalone-1\";\n\
             \n\
             fn select(request, context) {{\n\
             \x20   let view = #{{ originalPrompt: request.originalPrompt, skills: request.skills }};\n\
             \x20   #{{\n\
             \x20       status: \"selected\",\n\
             \x20       program: {harness},\n\
             \x20       args: [request.prompt, ravel::json_encode(view)],\n\
             \x20       provider: \"fixture\",\n\
             \x20       model: \"none\",\n\
             \x20       effort: \"none\",\n\
             \x20       reason: \"the standalone fake agent\",\n\
             \x20   }}\n\
             }}\n",
            harness = quote(self.harness.to_str().unwrap()),
        ));
    }

    /// `grove run impl` in `cwd`, which is no project, with each evidence
    /// file as an output in `out`.
    fn standalone(&self, cwd: &Path, out: &Path) -> Command {
        let mut command = self.driver(None, None);
        command.current_dir(cwd).args([
            "run",
            "impl",
            "**Load the `grove-impl` skill now.** Then write the evidence files.",
            "--ui",
            "inline",
        ]);
        for name in EVIDENCE {
            command.arg("--output").arg(out.join(name));
        }
        command
    }
}

/// The probe results of one run, by name.
fn probe_results(text: &str) -> std::collections::BTreeMap<String, String> {
    text.lines()
        .filter_map(|line| line.split_once('='))
        .map(|(name, result)| (name.to_owned(), result.to_owned()))
        .collect()
}

// A standalone `grove run` on the 2.0 build with directory delivery. The
// sources lie outside cwd, and no native copy exists. The agent reads the
// spine, its resource and the named kind from the retained snapshots, and
// reads the descriptor. Its writes to a snapshot fail by the file modes of the
// snapshot. Grove publishes the evidence only after the exact acknowledgement,
// and the caller block equals `request.originalPrompt`.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_standalone_run_reads_injected_skills_from_read_only_snapshots() {
    let lane = Lane::new(&build());
    let sources = Sources::plant(&lane);
    lane.settings(&sources.settings());
    lane.standalone_config();
    let probes = lane.root.join("probes.sh");
    fs::write(&probes, PROBES).unwrap();
    lane.standalone_agent(&probes);
    let cwd = lane.root.join("cwd");
    let out = lane.root.join("out");
    fs::create_dir(&cwd).unwrap();
    fs::create_dir(&out).unwrap();

    let output = lane.spawn(lane.standalone(&cwd, &out)).finish();
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(output.status.success(), "{stderr}");
    assert!(stderr.contains("grove run [impl]: completed"), "{stderr}");
    let read = |name: &str| fs::read_to_string(out.join(name)).unwrap();
    assert_eq!(read("started"), "started");

    let prompt = read("prompt");
    let view: serde_json::Value = serde_json::from_str(&read("view.json")).unwrap();
    let skills = &view["skills"];
    let block = caller_block(&prompt).unwrap();
    assert_eq!(
        view["originalPrompt"].as_str(),
        Some(block),
        "the caller block is request.originalPrompt byte for byte"
    );
    for command in [
        grove_loop::session_command::end_session(&lane.ravel).unwrap(),
        grove_loop::session_command::force_stop(&lane.ravel).unwrap(),
    ] {
        assert!(
            block.contains(&format!("\n{command}\n")),
            "the caller block holds the exact ending {command}: {block}"
        );
    }
    assert!(block.contains("Standalone invocation: work only on the staged files"));

    let mandatory = skills["mandatory"][0]["path"].as_str().unwrap();
    let reads = read("reads");
    assert_eq!(reads.lines().next(), Some(mandatory));
    reads_from_snapshots(&reads, skills).unwrap();
    let work = PathBuf::from(read("cwd"));
    for path in
        reads.lines().chain(
            skills["sources"]
                .as_array()
                .unwrap()
                .iter()
                .flat_map(|source| {
                    [
                        source["originPath"].as_str().unwrap(),
                        source["snapshotRoot"].as_str().unwrap(),
                    ]
                }),
        )
    {
        assert!(
            !fs::canonicalize(path).unwrap().starts_with(&work),
            "{path} is outside the working directory {work:?}"
        );
    }
    assert_eq!(read("spine"), SPINE);
    assert_eq!(read("resource"), RESOURCE_V1);
    assert_eq!(read("kind"), KIND_V1);
    no_native_copy(&lane.home).unwrap();

    let probes = probe_results(&read("probes"));
    let expected = [
        ("descriptor-read", "ok"),
        ("cwd-write", "ok"),
        ("snapshot-write", "denied"),
        ("snapshot-append", "denied"),
    ];
    assert_eq!(
        probes,
        expected
            .iter()
            .map(|(name, result)| ((*name).to_owned(), (*result).to_owned()))
            .collect(),
        "the probes"
    );
}

/// The installation record of a Ravel prefix, relative to that prefix.
const RECORD: &str = "share/ravel/installation.json";

/// The craft skill of the fixture engineering source.
const CRAFT: &str = "# doubt (fixture craft)\n";

/// The settings of a HOME that a 1.1 reader and a 2.0 reader share. Both
/// runtimes accept these keys, and neither key concerns skills.
fn shared_settings() -> serde_json::Value {
    serde_json::json!({ "timeoutMs": 60_000, "contextBytes": 262_144 })
}

/// One source of the fixture default directory: its ID, the keg of its
/// formula as `<formula>/<version>`, and its files with their contents.
struct KegSource {
    id: &'static str,
    keg: &'static str,
    files: &'static [(&'static str, &'static str)],
}

/// A Homebrew-shaped installation outside every HOME and working tree.
///
/// - `prefix/Cellar/ravel@2/2.0.0` is a private copy of the 2.0 build, and
///   its record names `prefix/share/ravel/skills`, as Ravel's formula writes
///   it.
/// - Each source lies in the keg of its formula, at the private
///   `Cellar/<formula>/<version>/libexec/skill-sources/<id>`. The keg's
///   `share/ravel/skills/<id>` is a relative directory symlink to it, and the
///   default skill directory holds a relative link to that alias, as `brew
///   link` makes it. "The keg layout" of
///   `docs/specs/codex-skill-provisioning.md` states the layout.
///
/// The sources are `engineering` with the roots `skills` and `craft`,
/// `grove-core` with `skills`, and the craft-only `personal`. The first two
/// share the `grove` keg, as Grove's formula carries core and the packs.
struct Homebrew {
    _temp: TempDir,
    directory: PathBuf,
    ravel: InstalledRavel,
    /// Each source ID with the canonical directory that its link reaches,
    /// in the byte order of the IDs.
    sources: Vec<(&'static str, PathBuf)>,
}

impl Homebrew {
    fn plant() -> Self {
        let temp = TempDir::new().unwrap();
        let prefix = fs::canonicalize(temp.path()).unwrap().join("homebrew");
        let directory = prefix.join("share/ravel/skills");
        fs::create_dir_all(&directory).unwrap();
        let ravel = InstalledRavel::copy_as(
            &build(),
            &prefix.join("Cellar/ravel@2/2.0.0"),
            Runtime::V2_0,
        );
        let record = ravel
            .executable()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(RECORD);
        fs::create_dir_all(record.parent().unwrap()).unwrap();
        fs::write(
            &record,
            serde_json::json!({ "schemaVersion": 1, "skillDirectory": directory }).to_string(),
        )
        .unwrap();

        let files = [
            KegSource {
                id: "engineering",
                keg: "grove/22.9.0",
                files: &[
                    ("skills/grove-impl/SKILL.md", KIND_V1),
                    ("craft/doubt/SKILL.md", CRAFT),
                ],
            },
            KegSource {
                id: "grove-core",
                keg: "grove/22.9.0",
                files: &[
                    ("skills/grove/SKILL.md", SPINE),
                    ("skills/grove/references/execute.md", RESOURCE_V1),
                ],
            },
            KegSource {
                id: "personal",
                keg: "personal/1.0.0",
                files: &[("craft/machine/SKILL.md", "# machine\n")],
            },
        ];
        let mut sources = Vec::new();
        for KegSource { id, keg, files } in files {
            let keg = format!("Cellar/{keg}");
            for (path, text) in files {
                let file = prefix
                    .join(&keg)
                    .join("libexec/skill-sources")
                    .join(id)
                    .join(path);
                fs::create_dir_all(file.parent().unwrap()).unwrap();
                fs::write(file, text).unwrap();
            }
            let alias = prefix.join(&keg).join("share/ravel/skills").join(id);
            fs::create_dir_all(alias.parent().unwrap()).unwrap();
            std::os::unix::fs::symlink(format!("../../../libexec/skill-sources/{id}"), &alias)
                .unwrap();
            std::os::unix::fs::symlink(
                format!("../../../{keg}/share/ravel/skills/{id}"),
                directory.join(id),
            )
            .unwrap();
            sources.push((id, fs::canonicalize(directory.join(id)).unwrap()));
        }
        assert_eq!(
            sources[0].1,
            prefix.join("Cellar/grove/22.9.0/libexec/skill-sources/engineering")
        );
        Homebrew {
            _temp: temp,
            directory,
            ravel,
            sources,
        }
    }

    /// The search directories that a launch must receive, in Ravel's order:
    /// each source in ID order, and in each source `skills`, then `craft`,
    /// where the root exists. `snapshot` maps a source ID to its snapshot.
    fn expected_directories(&self, snapshot: impl Fn(&str) -> PathBuf) -> Vec<PathBuf> {
        self.sources
            .iter()
            .flat_map(|(id, origin)| {
                let snapshot = snapshot(id);
                ["skills", "craft"]
                    .into_iter()
                    .filter(|root| origin.join(root).is_dir())
                    .map(move |root| snapshot.join(root))
            })
            .collect()
    }
}

/// The search directories that the preamble of `prompt` names, in order.
fn preamble_directories(prompt: &str) -> Vec<PathBuf> {
    let caller = prompt
        .find("\nCaller task\nBEGIN RAVEL CALLER TASK ")
        .unwrap_or(prompt.len());
    prompt[..caller]
        .lines()
        .filter_map(|line| line.strip_prefix("- \"")?.strip_suffix('"'))
        .map(PathBuf::from)
        .collect()
}

/// The checks of a launch that received the sources of `homebrew`'s default
/// skill directory and no mandatory file: the exact mandate, every source with
/// the directory that its link reaches, the search directories in Ravel's
/// order, and the reads of the kind, the spine and its resource from the
/// snapshots. No native copy exists under HOME.
fn assert_default_sources(lane: &Lane, homebrew: &Homebrew, n: usize, handle: &str) {
    let prompt = lane.read(n, "prompt");
    let view = lane.view(n);
    let mandate = lane.mandate(handle, "impl");
    assert_eq!(view["originalPrompt"].as_str(), Some(mandate.as_str()));
    assert_eq!(caller_block(&prompt), Ok(mandate.as_str()));

    let skills = &view["skills"];
    assert!(skills.is_object(), "delivery is on: {view}");
    assert_eq!(
        skills["mandatory"],
        serde_json::json!([]),
        "no settings name a mandatory file"
    );
    let sources = skills["sources"].as_array().unwrap();
    let delivered: Vec<(&str, PathBuf)> = sources
        .iter()
        .map(|source| {
            (
                source["id"].as_str().unwrap(),
                PathBuf::from(source["originPath"].as_str().unwrap()),
            )
        })
        .collect();
    assert_eq!(
        delivered, homebrew.sources,
        "each source of the default skill directory, by the keg that its link reaches"
    );
    let snapshot = |id: &str| {
        let source = sources.iter().find(|source| source["id"] == id).unwrap();
        PathBuf::from(source["snapshotRoot"].as_str().unwrap())
    };
    assert_eq!(
        preamble_directories(&prompt),
        homebrew.expected_directories(snapshot),
        "the preamble names `skills` and `craft` of each source: {prompt}"
    );
    assert_eq!(
        lane.read(n, "descriptor-env"),
        skills["descriptorPath"].as_str().unwrap()
    );

    reads_from_snapshots(&lane.read(n, "reads"), skills).unwrap();
    assert_eq!(lane.read(n, "kind"), KIND_V1);
    assert_eq!(lane.read(n, "spine"), SPINE);
    assert_eq!(lane.read(n, "resource"), RESOURCE_V1);
    no_native_copy(&lane.home).unwrap();
}

/// The checks of a launch with delivery disabled: the agent receives the
/// mandate unchanged, `request.skills` is `()`, no descriptor reaches the
/// agent and Ravel does no cache work in HOME.
fn assert_delivery_disabled(lane: &Lane, n: usize, handle: &str) {
    let mandate = lane.mandate(handle, "impl");
    assert_eq!(
        lane.read(n, "prompt"),
        mandate,
        "request.prompt is the mandate"
    );
    let view = lane.view(n);
    assert_eq!(view["originalPrompt"].as_str(), Some(mandate.as_str()));
    assert_eq!(view["skills"], serde_json::Value::Null, "{view}");
    assert_eq!(lane.read(n, "descriptor-env"), "<unset>");
    assert!(
        !lane.home.join(".cache/ravel/skills").exists(),
        "a disabled launch does no cache work"
    );
}

// With no `skills` key and no settings file, a lifecycle launch through the
// copied 2.0 build with an installation record receives each source of the
// fixture default directory. The agent finds the named kind and then the
// spine beneath the search directories, as no mandatory file is named, and
// reads every file from the snapshots.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_launch_without_skills_settings_receives_each_source_of_the_default_directory() {
    let homebrew = Homebrew::plant();
    let lane = Lane::new(homebrew.ravel.executable());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    lane.directory_config();
    lane.agent("discover\nfinish");
    assert!(!lane.home.join(".config/ravel/settings.json").exists());

    completed(&lane.run());
    assert_eq!(lane.launch_count(), 1);
    assert_default_sources(&lane, &homebrew, 0, "subject-k1");
}

// `"sources": []` beside the same populated default directory turns delivery
// off. The agent receives the mandate unchanged and no descriptor.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn empty_sources_beside_the_default_directory_deliver_nothing() {
    let homebrew = Homebrew::plant();
    let lane = Lane::new(homebrew.ravel.executable());
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    lane.settings(&serde_json::json!({ "skills": { "schemaVersion": 1, "sources": [] } }));
    lane.directory_config();
    lane.agent("finish");

    completed(&lane.run());
    assert_eq!(lane.launch_count(), 1);
    assert_delivery_disabled(&lane, 0, "subject-k1");
    assert!(fs::read_dir(&homebrew.directory).unwrap().count() == 3);
}

// A copied 2.0 build without an installation record has no default skill
// directory, so it delivers nothing while the fixture directory is populated.
// The case with a record above is the control that differs only by the
// record.
#[test]
#[ignore = "needs the Ravel 2.0 directory build: task ravel:directory"]
fn a_copied_build_without_a_record_delivers_nothing() {
    let homebrew = Homebrew::plant();
    let copied = build();
    assert!(
        !copied
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join(RECORD)
            .exists(),
        "the shared copy of the build holds no record"
    );
    let lane = Lane::new(&copied);
    plant_tree(&lane.worktree, "01-impl--subject-k1.md");
    lane.directory_config();
    lane.agent("finish");

    completed(&lane.run());
    assert_eq!(lane.launch_count(), 1);
    assert_delivery_disabled(&lane, 0, "subject-k1");
    assert!(fs::read_dir(&homebrew.directory).unwrap().count() == 3);
}

// A 1.1 reader and a 2.0 reader share one HOME, whose settings hold no
// `skills` key. The native launch on 1.1 receives the mandate unchanged. The
// 2.0 launch through the build with a record then receives the sources of the
// fixture default directory, and the 1.1 reader still launches natively
// after it.
#[test]
#[ignore = "needs the Ravel 2.0 directory build and a 1.1 build: task ravel:directory"]
fn a_shared_home_without_skills_delivers_the_default_sources_on_2_0() {
    let homebrew = Homebrew::plant();
    let native = Lane::new(old_build().executable());
    let mut directory = Lane::new(homebrew.ravel.executable());
    directory.home = native.home.clone();
    native.settings(&shared_settings());
    native.native_policy();
    directory.directory_config();
    assert!(native.home.join(".config/ravel/policy.ts").is_file());
    assert!(native.home.join(".config/ravel/config.rhai").is_file());

    plant_tree(&native.worktree, "01-impl--subject-k1.md");
    native.agent("finish");
    completed(&native.run());
    assert_eq!(native.launch_count(), 1);
    assert_eq!(
        native.read(0, "prompt"),
        native.mandate("subject-k1", "impl")
    );
    assert_eq!(native.read(0, "descriptor-env"), "<unset>");

    plant_tree(&directory.worktree, "01-impl--subject-k1.md");
    directory.agent("discover\nfinish");
    completed(&directory.run());
    assert_eq!(directory.launch_count(), 1);
    assert_default_sources(&directory, &homebrew, 0, "subject-k1");

    // The leaf stays live, because the fake agent does not retire it.
    completed(&native.run());
    assert_eq!(native.launch_count(), 2);
    assert_eq!(
        native.read(1, "prompt"),
        native.mandate("subject-k1", "impl")
    );
    assert_eq!(native.read(1, "descriptor-env"), "<unset>");
}

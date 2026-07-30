//! `codex-bridge`: the falsification harness for AgentAnyware's
//! `success-demonstration-k11` grove — **not** a supported Grove feature, and
//! not wired into `grove`'s README, `--help`, or `grove do`'s own launch path.
//! It implements that grove's `codex-bridge-k54` work item, designed in
//! `demo-architecture-and-plan-k53`. Both are recoverable from the AgentAnyware
//! repo's **commit log** rather than from any path: that grove's `.grove/` tree
//! was deleted at its own finish cycle, so the handles are what survive it
//! (task-tree-scheme §5). Why the binary looks like this is stated below rather
//! than cited — see the next paragraph, and the workspace `Cargo.toml` for why
//! it is a separate crate.
//!
//! It spawns `agentanyware run --adapter codex --cwd <path>` as a child
//! process and exposes its stdin/stdout as one-shot CLI primitives — a
//! driving Claude Code session calls one of these per Bash tool call, reads
//! the printed result, and decides what to do next itself. This binary holds
//! **no supervisor-resolution-envelope, no policy tiers, and no phase
//! mapping**: every resolution decision (which option, which answer) is
//! supplied by the caller's own arguments, never computed in here. A
//! reviewer should be able to read this whole file in a few minutes and see
//! that it decides nothing.
//!
//! `agentanyware run` already mirrors `records.jsonl` byte-for-byte to
//! stdout as line-delimited JSON, and reads a matching envelope on stdin —
//! this binary wraps that wire, it does not invent a new one. **Correction
//! against this leaf's own Context note**, found empirically the same way
//! every adapter leaf before it found its own gaps: stdin is not a bare
//! `AgentInput` line. `agentanyware-core::reader::InputLine` wraps it as
//! `{"v":1,"session_id":<the session's own id>,"payload":<AgentInput>}` —
//! `v` mirrors `agentanyware-core::wire::PROTOCOL_VERSION` (hardcoded to
//! `1` below; there is no other way to track it from a separate repo), and
//! `session_id` is read back from the first line of `stdout.log` (always
//! the session's own `SessionLaunching` event) rather than threaded through
//! as a flag, so a caller never has to know or repeat it. The JSON shapes
//! for `payload` (`{"Deliver":{"Prompt":{"content":<base64>}}}` etc.) are
//! `agentanyware-core`'s `AgentInput`/`Delivery`/`Resolution`/`Control`
//! serde encoding, reproduced by hand rather than imported: `agentanyware-
//! core` is an internal crate of a different repo, without a published or
//! path-stable way to depend on it from here, so this binary treats the
//! wire as the contract, the same way any non-Rust consumer would have to.
//!
//! # Usage
//!
//! ```text
//! codex-bridge <session-dir> start --cwd <path>
//!     [--agentanyware-binary <path>] [--codex-binary <path>]
//! codex-bridge <session-dir> next-event [--timeout-secs <n>]
//! codex-bridge <session-dir> prompt <text...>
//! codex-bridge <session-dir> resolve <solicitation-id> approve <option-id>
//! codex-bridge <session-dir> resolve <solicitation-id> deny <option-id>
//! codex-bridge <session-dir> resolve <solicitation-id> answer <question-id> <text...>
//! codex-bridge <session-dir> interrupt
//! codex-bridge <session-dir> terminate
//! ```
//!
//! `<session-dir>` is where this bridge keeps everything it needs to find
//! its child again from a fresh process on the next call, since each of the
//! commands above is its own separate invocation: `stdin.fifo` (the child's
//! stdin — a named pipe, so a new writer can open it on every call without a
//! resident process holding it), `stdout.log` / `stderr.log` (the child's
//! own streams, redirected to plain files so `next-event` can tail them from
//! a saved offset), `cursor` (that offset) and `child.pid`.

use std::{
    env,
    fs::{self, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Command, ExitCode, Stdio},
    time::{Duration, Instant},
};

use base64::Engine as _;

/// How long `next-event` waits for a new line to appear before giving up —
/// generous for an interactive demo session, not tuned for anything else.
const DEFAULT_TIMEOUT_SECS: u64 = 30;
const POLL_INTERVAL: Duration = Duration::from_millis(100);

/// `agentanyware-core::wire::PROTOCOL_VERSION`, copied rather than imported
/// (this file's own top doc comment explains why) — bump this if that
/// constant ever moves.
const PROTOCOL_VERSION: u32 = 1;

/// How long a `prompt`/`resolve`/`interrupt`/`terminate` call will wait for
/// `stdout.log`'s first line (the session's own `SessionLaunching` event,
/// which carries the `session_id` every input line must name) before giving
/// up — covers the case where a caller races a write against `start` before
/// the child has produced anything yet.
const SESSION_ID_TIMEOUT: Duration = Duration::from_secs(10);

fn main() -> ExitCode {
    let mut args = env::args().skip(1);
    match run(&mut args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("codex-bridge: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: &mut dyn Iterator<Item = String>) -> Result<(), String> {
    let session_dir = PathBuf::from(args.next().ok_or_else(usage)?);
    let command = args.next().ok_or_else(usage)?;
    let rest: Vec<String> = args.collect();

    match command.as_str() {
        "start" => cmd_start(&session_dir, rest),
        "next-event" => cmd_next_event(&session_dir, rest),
        "prompt" => cmd_prompt(&session_dir, rest),
        "resolve" => cmd_resolve(&session_dir, rest),
        "interrupt" => cmd_control(&session_dir, "Interrupt"),
        "terminate" => cmd_control(&session_dir, "Terminate"),
        other => Err(format!(
            "unknown command '{other}'; expected start, next-event, prompt, resolve, interrupt, or terminate"
        )),
    }
}

fn usage() -> String {
    "usage: codex-bridge <session-dir> <start|next-event|prompt|resolve|interrupt|terminate> [args...]"
        .to_string()
}

// ---------------------------------------------------------------------------
// start
// ---------------------------------------------------------------------------

fn cmd_start(session_dir: &Path, args: Vec<String>) -> Result<(), String> {
    let mut cwd = None;
    let mut agentanyware_binary = None;
    let mut codex_binary = None;

    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--cwd" => cwd = Some(PathBuf::from(next_value(&mut iter, "--cwd")?)),
            "--agentanyware-binary" => {
                agentanyware_binary = Some(PathBuf::from(next_value(
                    &mut iter,
                    "--agentanyware-binary",
                )?))
            }
            "--codex-binary" => {
                codex_binary = Some(PathBuf::from(next_value(&mut iter, "--codex-binary")?))
            }
            other => return Err(format!("start: unrecognised argument '{other}'")),
        }
    }
    let cwd = cwd.ok_or("start: --cwd is required")?;

    let agentanyware_binary = agentanyware_binary
        .or_else(|| locate_on_path("agentanyware"))
        .ok_or("start: agentanyware not found on PATH; pass --agentanyware-binary")?;

    if session_dir.join("child.pid").exists() {
        return Err(format!(
            "{} already has a child.pid; use a fresh --session-dir per session",
            session_dir.display()
        ));
    }
    fs::create_dir_all(session_dir)
        .map_err(|err| format!("could not create {}: {err}", session_dir.display()))?;

    let fifo_path = session_dir.join("stdin.fifo");
    if !fifo_path.exists() {
        let status = Command::new("mkfifo")
            .arg(&fifo_path)
            .status()
            .map_err(|err| format!("could not run mkfifo: {err}"))?;
        if !status.success() {
            return Err(format!("mkfifo {} failed", fifo_path.display()));
        }
    }

    // Opening a FIFO for read-only blocks until a writer connects. Opening
    // it O_RDWR instead never blocks and gives the child a read-capable fd
    // it can hold for its whole lifetime, so every later `prompt`/`resolve`/
    // `interrupt`/`terminate` invocation's write-only open succeeds
    // immediately instead of waiting for a reader that is always already
    // there.
    let stdin_fifo = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&fifo_path)
        .map_err(|err| format!("could not open {}: {err}", fifo_path.display()))?;

    let stdout_log = fs::File::create(session_dir.join("stdout.log"))
        .map_err(|err| format!("could not create stdout.log: {err}"))?;
    let stderr_log = fs::File::create(session_dir.join("stderr.log"))
        .map_err(|err| format!("could not create stderr.log: {err}"))?;
    fs::write(session_dir.join("cursor"), b"0")
        .map_err(|err| format!("could not write cursor: {err}"))?;

    let mut command = Command::new(&agentanyware_binary);
    command
        .arg("run")
        .arg("--adapter")
        .arg("codex")
        .arg("--cwd")
        .arg(&cwd)
        .stdin(Stdio::from(stdin_fifo))
        .stdout(Stdio::from(stdout_log))
        .stderr(Stdio::from(stderr_log));
    if let Some(codex_binary) = &codex_binary {
        command.arg("--binary").arg(codex_binary);
    }
    // Its own process group: this `start` invocation is short-lived, but the
    // child it leaves running is not — a signal aimed at `start`'s own shell
    // job must not reach the process it spawned and detached from.
    command.process_group(0);

    let child = command
        .spawn()
        .map_err(|err| format!("could not spawn {}: {err}", agentanyware_binary.display()))?;
    fs::write(session_dir.join("child.pid"), child.id().to_string())
        .map_err(|err| format!("could not write child.pid: {err}"))?;

    println!(
        "started pid {} in {}; call next-event to read its first record",
        child.id(),
        session_dir.display()
    );
    Ok(())
}

// ---------------------------------------------------------------------------
// next-event
// ---------------------------------------------------------------------------

fn cmd_next_event(session_dir: &Path, args: Vec<String>) -> Result<(), String> {
    let mut timeout_secs = DEFAULT_TIMEOUT_SECS;
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--timeout-secs" => {
                let value = next_value(&mut iter, "--timeout-secs")?;
                timeout_secs = value
                    .parse()
                    .map_err(|_| format!("invalid --timeout-secs value '{value}'"))?;
            }
            other => return Err(format!("next-event: unrecognised argument '{other}'")),
        }
    }

    let stdout_path = session_dir.join("stdout.log");
    let cursor_path = session_dir.join("cursor");
    let mut offset = read_cursor(&cursor_path);
    let deadline = Instant::now() + Duration::from_secs(timeout_secs);

    loop {
        if let Some((line, new_offset)) = read_next_line(&stdout_path, offset)? {
            offset = new_offset;
            write_cursor(&cursor_path, offset)?;
            if line.is_empty() {
                continue;
            }
            let value: serde_json::Value = serde_json::from_slice(&line)
                .map_err(|err| format!("stdout.log line was not valid JSON: {err}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(&value).map_err(|err| err.to_string())?
            );
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(format!("no new record within {timeout_secs}s"));
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn read_cursor(cursor_path: &Path) -> usize {
    fs::read_to_string(cursor_path)
        .ok()
        .and_then(|contents| contents.trim().parse().ok())
        .unwrap_or(0)
}

fn write_cursor(cursor_path: &Path, offset: usize) -> Result<(), String> {
    fs::write(cursor_path, offset.to_string())
        .map_err(|err| format!("could not write {}: {err}", cursor_path.display()))
}

/// Reads whatever is new since `offset` and returns the first complete line
/// (without its trailing `\n`) plus the offset just past it — or `None` if
/// no full line has arrived yet. Seeks to `offset` rather than re-reading
/// the whole file, so this stays cheap to poll.
fn read_next_line(stdout_path: &Path, offset: usize) -> Result<Option<(Vec<u8>, usize)>, String> {
    let mut file = fs::File::open(stdout_path)
        .map_err(|err| format!("could not open {}: {err}", stdout_path.display()))?;
    file.seek(SeekFrom::Start(offset as u64))
        .map_err(|err| format!("could not seek {}: {err}", stdout_path.display()))?;
    let mut tail = Vec::new();
    file.read_to_end(&mut tail)
        .map_err(|err| format!("could not read {}: {err}", stdout_path.display()))?;

    match tail.iter().position(|&byte| byte == b'\n') {
        Some(newline_at) => {
            let line = tail[..newline_at].to_vec();
            Ok(Some((line, offset + newline_at + 1)))
        }
        None => Ok(None),
    }
}

// ---------------------------------------------------------------------------
// prompt / resolve / interrupt / terminate — each builds an `AgentInput`
// payload and hands it to `write_input_line`, which wraps it in the actual
// `{v, session_id, payload}` envelope and writes it to stdin.fifo
// ---------------------------------------------------------------------------

fn cmd_prompt(session_dir: &Path, args: Vec<String>) -> Result<(), String> {
    if args.is_empty() {
        return Err("prompt: expected text".to_string());
    }
    let content = base64::engine::general_purpose::STANDARD.encode(args.join(" "));
    let input = serde_json::json!({ "Deliver": { "Prompt": { "content": content } } });
    write_input_line(session_dir, &input)
}

fn cmd_resolve(session_dir: &Path, args: Vec<String>) -> Result<(), String> {
    let mut iter = args.into_iter();
    let solicitation_id = iter.next().ok_or("resolve: expected a solicitation id")?;
    validate_uuid(&solicitation_id)?;
    let verb = iter
        .next()
        .ok_or("resolve: expected approve, deny, or answer")?;

    let resolution = match verb.as_str() {
        "approve" => {
            let option_id = next_value(&mut iter, "resolve approve")?;
            validate_uuid(&option_id)?;
            serde_json::json!({ "Approve": { "option": option_id } })
        }
        "deny" => {
            let option_id = next_value(&mut iter, "resolve deny")?;
            validate_uuid(&option_id)?;
            serde_json::json!({ "Deny": { "option": option_id } })
        }
        "answer" => {
            let question_id = next_value(&mut iter, "resolve answer")?;
            validate_uuid(&question_id)?;
            let text: Vec<String> = iter.collect();
            if text.is_empty() {
                return Err("resolve answer: expected text after the question id".to_string());
            }
            serde_json::json!({
                "Answer": { "answers": [[question_id, { "Text": text.join(" ") }]] }
            })
        }
        other => {
            return Err(format!(
                "resolve: unknown verb '{other}'; expected approve, deny, or answer"
            ))
        }
    };

    let input = serde_json::json!({
        "Deliver": {
            "Resolve": {
                "solicitation_id": solicitation_id,
                "resolution": resolution,
            }
        }
    });
    write_input_line(session_dir, &input)
}

fn cmd_control(session_dir: &Path, variant: &str) -> Result<(), String> {
    let input = serde_json::json!({ "Control": variant });
    write_input_line(session_dir, &input)
}

fn write_input_line(session_dir: &Path, payload: &serde_json::Value) -> Result<(), String> {
    let session_id = read_session_id(session_dir)?;
    let envelope = serde_json::json!({
        "v": PROTOCOL_VERSION,
        "session_id": session_id,
        "payload": payload,
    });
    let mut line = serde_json::to_string(&envelope).map_err(|err| err.to_string())?;
    line.push('\n');

    let fifo_path = session_dir.join("stdin.fifo");
    let mut fifo = OpenOptions::new()
        .write(true)
        .open(&fifo_path)
        .map_err(|err| {
            format!(
                "could not open {} for writing (was 'start' run first?): {err}",
                fifo_path.display()
            )
        })?;
    fifo.write_all(line.as_bytes())
        .map_err(|err| format!("could not write to {}: {err}", fifo_path.display()))?;
    println!("sent: {line}");
    Ok(())
}

/// The session's own id, read back from the first line of `stdout.log` —
/// always that session's `SessionLaunching` event, which (like every
/// `Record`) carries `session_id` at the top level. Polls briefly rather
/// than reading once: a caller may reach here before the child has written
/// anything yet.
fn read_session_id(session_dir: &Path) -> Result<String, String> {
    let stdout_path = session_dir.join("stdout.log");
    let deadline = Instant::now() + SESSION_ID_TIMEOUT;
    loop {
        if let Some((line, _)) = read_next_line(&stdout_path, 0)? {
            let value: serde_json::Value = serde_json::from_slice(&line)
                .map_err(|err| format!("first line of stdout.log was not valid JSON: {err}"))?;
            return value
                .get("session_id")
                .and_then(|id| id.as_str())
                .map(|id| id.to_string())
                .ok_or_else(|| {
                    format!(
                        "first record in {} had no session_id field",
                        stdout_path.display()
                    )
                });
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "{} has no complete first line yet; was 'start' run first?",
                stdout_path.display()
            ));
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

fn validate_uuid(candidate: &str) -> Result<(), String> {
    uuid::Uuid::parse_str(candidate)
        .map(|_| ())
        .map_err(|err| format!("'{candidate}' is not a valid id: {err}"))
}

fn next_value(iter: &mut impl Iterator<Item = String>, flag: &str) -> Result<String, String> {
    iter.next()
        .ok_or_else(|| format!("{flag} requires a value"))
}

// ---------------------------------------------------------------------------
// PATH lookup — mirrors `agentanyware-adapter-codex::preflight::locate_on_path`
// (a path argument or a PATH lookup, never a hardcoded location), reimplemented
// rather than imported for the same cross-repo reason as the wire shapes above.
// ---------------------------------------------------------------------------

fn locate_on_path(name: &str) -> Option<PathBuf> {
    let path_var = env::var_os("PATH")?;
    env::split_paths(&path_var).find_map(|dir| {
        let candidate = dir.join(name);
        candidate_is_executable(&candidate).then_some(candidate)
    })
}

fn candidate_is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|meta| meta.is_file() && meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

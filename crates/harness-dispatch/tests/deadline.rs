//! The whole-selection deadline, through the command seam.
//!
//! A policy that holds evaluation, by spinning or by awaiting a promise that
//! live work keeps pending, at import or inside its `loadContext` or `select`,
//! is stopped when the caller's bound runs out: the front exits 124, launches
//! nothing and leaves no worker behind, with the handlers that cancel
//! selection on a signal installed. The holds and the watchdog are
//! `support::hold`'s. Variants whose hold ends within the bound reach the fake
//! harness, so no timeout case can pass by never evaluating.

mod support;

use std::fs;
use std::time::Duration;

use serde_json::Value;
use support::hold::{exists, guarded, holding, recorded_pid, Hold, Place};
use support::{text, Sandbox, ROUTED};

/// The watchdog for a timeout case: the bound, the cleanup grace, the
/// drains' grace and generous slack for a loaded machine.
const WATCHDOG: Duration = Duration::from_secs(20);

/// The explicit bound the timeout cases use. It counts from the worker's
/// start, so it must leave a loaded machine time to start the worker and
/// reach the policy's first line, which records the evidence these tests
/// read. Under the full workspace check, a fake worker has failed to reach
/// its first line within one second.
const BOUND_MS: u64 = 3000;

/// A policy that holds forever at `place`, run as `command` under `BOUND_MS`.
fn assert_stopped_at_the_deadline(command: &str, place: Place, hold: Hold, prelude: &str) -> Value {
    let bound = BOUND_MS.to_string();
    let sandbox = Sandbox::new();
    let pid_file = sandbox.root.join("worker-pid");
    sandbox.personal_policy(&holding(&pid_file, place, hold, None, prelude));

    let mut invocation = sandbox.command();
    invocation.args([
        command,
        "--kind",
        "impl",
        "--prompt",
        "the prompt",
        "--timeout-ms",
        &bound,
        "--json",
    ]);
    let timed = guarded(&mut invocation, &pid_file, WATCHDOG);
    let refusal = timed.run.refusal(124);

    let context = format!("{command} {place:?} {hold:?}: {refusal}");
    assert_eq!(refusal["error"]["code"], "selection_timeout", "{context}");
    assert_eq!(refusal["error"]["stage"], "evaluation", "{context}");
    assert_eq!(
        refusal["error"]["bound"],
        serde_json::json!({ "name": "selection", "ms": BOUND_MS, "from": "--timeout-ms" }),
        "{context}"
    );
    assert_eq!(refusal["error"]["input"], "--timeout-ms", "{context}");
    assert_eq!(
        refusal["error"]["source"],
        text(&sandbox.personal_path()),
        "{context}"
    );
    assert!(
        refusal["error"]["message"]
            .as_str()
            .unwrap()
            .contains(&format!("{BOUND_MS} ms")),
        "{context}"
    );
    let pid = recorded_pid(&pid_file)
        .unwrap_or_else(|| panic!("the policy never began evaluating: {context}"));
    assert!(!exists(pid), "worker {pid} survived the front: {context}");
    assert!(!sandbox.harness_ran(), "a harness was launched: {context}");
    assert!(
        timed.elapsed >= Duration::from_millis(BOUND_MS),
        "refused before the bound ran out, after {:?}: {context}",
        timed.elapsed
    );
    assert!(
        timed.elapsed < Duration::from_millis(BOUND_MS + 4000),
        "took {:?} to refuse: {context}",
        timed.elapsed
    );
    refusal
}

#[test]
fn a_policy_that_spins_at_import_is_stopped_at_the_deadline() {
    for command in ["inspect", "run"] {
        assert_stopped_at_the_deadline(command, Place::Import, Hold::Spin, "");
    }
}

#[test]
fn a_policy_awaiting_a_promise_that_live_work_keeps_pending_is_stopped_at_the_deadline() {
    for command in ["inspect", "run"] {
        assert_stopped_at_the_deadline(command, Place::Import, Hold::Pending, "");
    }
}

#[test]
fn a_select_that_spins_is_stopped_at_the_deadline() {
    for command in ["inspect", "run"] {
        assert_stopped_at_the_deadline(command, Place::Select, Hold::Spin, "");
    }
}

#[test]
fn a_select_awaiting_a_promise_that_live_work_keeps_pending_is_stopped_at_the_deadline() {
    for command in ["inspect", "run"] {
        assert_stopped_at_the_deadline(command, Place::Select, Hold::Pending, "");
    }
}

#[test]
fn a_load_context_that_spins_is_stopped_at_the_deadline() {
    for command in ["inspect", "run"] {
        assert_stopped_at_the_deadline(command, Place::LoadContext, Hold::Spin, "");
    }
}

#[test]
fn a_load_context_awaiting_a_promise_that_live_work_keeps_pending_is_stopped_at_the_deadline() {
    for command in ["inspect", "run"] {
        assert_stopped_at_the_deadline(command, Place::LoadContext, Hold::Pending, "");
    }
}

#[test]
fn the_host_signal_aborts_when_the_deadline_stops_a_waiting_loader() {
    // The loader listens on its host's signal and then awaits work that never
    // settles. At the deadline the front sends TERM, the signal aborts and its
    // listener runs, and the worker still ends and is reaped. A loader with a
    // TERM listener of its own, registered after it took the signal, owns its
    // exit: its listener runs as well, after the abort. The control is the
    // same loader returning at once: it selects, and the signal never aborts.
    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Loader {
        Waits,
        WaitsWithItsOwnListener,
        Returns,
    }
    let policy = |sandbox: &Sandbox, loader: Loader| {
        let own = match loader {
            Loader::WaitsWithItsOwnListener => format!(
                "process.on(\"SIGTERM\", () => {{ writeFileSync({:?}, \"own\"); process.exit(0); }});",
                text(&sandbox.root.join("own"))
            ),
            _ => String::new(),
        };
        format!(
            r#"import {{ writeFileSync }} from "node:fs";
export const policy = {{
  schemaVersion: 1,
  version: "seam-1",
  catalog: [
    {{ id: "deep", provider: "origin-a", model: "model-large", effort: "high", program: "fake-harness", args: [{{ slot: "prompt" }}] }},
  ],
  async loadContext(request, host) {{
    writeFileSync({pid:?}, String(process.pid));
    host.signal.addEventListener("abort", () => writeFileSync({marker:?}, host.signal.reason.message));
    {own}
    if ({wait}) await new Promise(() => setInterval(() => {{}}, 20));
    return {{ schemaVersion: 1 }};
  }},
  select() {{ return {{ status: "selected", candidateId: "deep", reason: "r" }}; }},
}};
"#,
            pid = text(&sandbox.root.join("worker-pid")),
            marker = text(&sandbox.root.join("aborted")),
            wait = loader != Loader::Returns,
        )
    };
    for loader in [
        Loader::Waits,
        Loader::WaitsWithItsOwnListener,
        Loader::Returns,
    ] {
        let sandbox = Sandbox::new();
        let pid_file = sandbox.root.join("worker-pid");
        let aborted = sandbox.root.join("aborted");
        sandbox.personal_policy(&policy(&sandbox, loader));
        let mut invocation = sandbox.command();
        let bound = BOUND_MS.to_string();
        invocation.args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "p",
            "--timeout-ms",
            &bound,
            "--json",
        ]);
        let timed = guarded(&mut invocation, &pid_file, WATCHDOG);
        if loader == Loader::Returns {
            assert_eq!(timed.run.code, Some(0), "{}", timed.run.stderr);
            assert!(sandbox.harness_ran());
            assert!(!aborted.exists(), "the signal aborted without a stop");
        } else {
            let refusal = timed.run.refusal(124);
            assert_eq!(refusal["error"]["code"], "selection_timeout", "{loader:?}");
            assert_eq!(
                fs::read_to_string(&aborted).ok().as_deref(),
                Some("harness-dispatch stopped this selection"),
                "{loader:?}"
            );
            assert!(!sandbox.harness_ran(), "{loader:?}");
        }
        assert_eq!(
            sandbox.root.join("own").exists(),
            loader == Loader::WaitsWithItsOwnListener,
            "{loader:?}: the policy's own TERM listener"
        );
        assert!(!exists(recorded_pid(&pid_file).unwrap()), "{loader:?}");
    }
}

#[test]
fn a_hold_that_ends_within_the_bound_reaches_the_harness() {
    // The positive control for every timeout fixture: the same holds, at
    // import and in select, ended after longer than the timeout cases' bound
    // and well within this one. Each selects and reaches the fake harness, so
    // the fixtures do evaluate, and the bound is the caller's rather than a
    // fixed one.
    let held = BOUND_MS + 500;
    for (place, hold) in [
        (Place::Import, Hold::Spin),
        (Place::Import, Hold::Pending),
        (Place::Select, Hold::Spin),
        (Place::Select, Hold::Pending),
        (Place::LoadContext, Hold::Spin),
        (Place::LoadContext, Hold::Pending),
    ] {
        let hold_name = format!("{place:?} {hold:?}");
        let sandbox = Sandbox::new();
        let pid_file = sandbox.root.join("worker-pid");
        sandbox.personal_policy(&holding(&pid_file, place, hold, Some(held), ""));

        let mut invocation = sandbox.command();
        invocation.args([
            "run",
            "--kind",
            "impl",
            "--prompt",
            "the prompt",
            "--timeout-ms",
            "15000",
            "--json",
        ]);
        let timed = guarded(&mut invocation, &pid_file, WATCHDOG);

        assert_eq!(
            timed.run.code,
            Some(0),
            "{hold_name}\nstdout: {}\nstderr: {}",
            timed.run.stdout,
            timed.run.stderr
        );
        let notice: Value = serde_json::from_str(&timed.run.stderr).unwrap();
        let selected_by = match place {
            Place::Import => "route",
            Place::Select | Place::LoadContext => "select",
        };
        assert_eq!(notice["handoff"]["selectedBy"], selected_by, "{hold_name}");
        assert!(sandbox.harness_ran(), "{hold_name}: the harness never ran");
        assert_eq!(sandbox.harness_args(), ["the prompt"]);
        assert!(
            timed.elapsed >= Duration::from_millis(held),
            "{hold_name}: the hold ended early, after {:?}",
            timed.elapsed
        );
    }
}

#[test]
fn a_worker_that_ignores_term_is_killed_after_at_most_a_second_of_grace() {
    // A handler for TERM keeps the signal from ending the process, and a
    // synchronous spin keeps the handler from ever running, so only KILL ends
    // this worker. The refusal still arrives within the bound and one second.
    for command in ["inspect", "run"] {
        let refusal = assert_stopped_at_the_deadline(
            command,
            Place::Import,
            Hold::Spin,
            "process.on(\"SIGTERM\", () => {});",
        );
        assert_eq!(refusal["error"]["code"], "selection_timeout");
    }
}

#[test]
fn the_worker_is_offered_its_cleanup_grace_before_kill() {
    // An awaiting worker's event loop runs its TERM handler, which records
    // that it ran. That it did shows the front sent TERM and waited, rather
    // than killing outright.
    let sandbox = Sandbox::new();
    let marker = sandbox.root.join("term-handled");
    let prelude = format!(
        "process.on(\"SIGTERM\", () => {{ writeFileSync({:?}, \"term\"); process.exit(0); }});",
        text(&marker)
    );
    let pid_file = sandbox.root.join("worker-pid");
    sandbox.personal_policy(&holding(
        &pid_file,
        Place::Import,
        Hold::Pending,
        None,
        &prelude,
    ));

    let mut invocation = sandbox.command();
    let bound = BOUND_MS.to_string();
    invocation.args([
        "inspect",
        "--kind",
        "impl",
        "--timeout-ms",
        &bound,
        "--json",
    ]);
    let timed = guarded(&mut invocation, &pid_file, WATCHDOG);

    let refusal = timed.run.refusal(124);
    assert_eq!(refusal["error"]["code"], "selection_timeout");
    assert_eq!(fs::read_to_string(&marker).ok().as_deref(), Some("term"));
    assert!(!exists(recorded_pid(&pid_file).unwrap()));
}

#[test]
fn a_worker_that_will_not_exit_after_its_result_is_killed_after_the_grace() {
    // The result arrives, and then an exit handler spins. The selection
    // stands, since it arrived within the bound, and the front does not wait
    // for the worker longer than the cleanup grace. The bound is generous:
    // here it must not run out. A routes policy's worker exits once the front
    // closes the channel, and a computed one's once it has sent its selection.
    for (command, place) in [
        ("inspect", Place::Import),
        ("run", Place::Import),
        ("inspect", Place::Select),
        ("run", Place::Select),
    ] {
        let sandbox = Sandbox::new();
        let pid_file = sandbox.root.join("worker-pid");
        sandbox.personal_policy(&holding(
            &pid_file,
            place,
            Hold::Spin,
            Some(0),
            "process.on(\"exit\", () => { while (true) {} });",
        ));

        let mut invocation = sandbox.command();
        invocation.args([
            command,
            "--kind",
            "impl",
            "--prompt",
            "the prompt",
            "--timeout-ms",
            "15000",
        ]);
        let timed = guarded(&mut invocation, &pid_file, WATCHDOG);

        assert_eq!(
            timed.run.code,
            Some(0),
            "{command} {place:?}\nstdout: {}\nstderr: {}",
            timed.run.stdout,
            timed.run.stderr
        );
        assert_eq!(
            sandbox.harness_ran(),
            command == "run",
            "{command} {place:?}"
        );
        let pid = recorded_pid(&pid_file).unwrap();
        assert!(
            !exists(pid),
            "{command} {place:?}: worker {pid} survived the front"
        );
        assert!(
            timed.elapsed < Duration::from_secs(8),
            "{command} {place:?}: took {:?}",
            timed.elapsed
        );
    }
}

#[test]
fn the_bound_is_one_to_one_hundred_and_twenty_seconds_in_milliseconds() {
    let sandbox = Sandbox::new();
    let sentinel = sandbox.root.join("policy-ran");
    sandbox.personal_policy(&format!(
        "import {{ writeFileSync }} from \"node:fs\";\nwriteFileSync({:?}, \"ran\");\n{ROUTED}",
        text(&sentinel)
    ));
    for malformed in [
        "999",
        "120001",
        "0",
        "abc",
        "1.5",
        "+1000",
        " 1000",
        "1e3",
        "",
        "18446744073709551616",
    ] {
        let refusal = sandbox
            .inspect(&["--kind", "impl", "--timeout-ms", malformed, "--json"])
            .refusal(2);
        assert_eq!(
            refusal["error"]["code"], "malformed_input",
            "{malformed:?}: {refusal}"
        );
        assert_eq!(refusal["error"]["input"], "--timeout-ms", "{malformed:?}");
        assert!(!sentinel.exists(), "{malformed:?}: the policy ran");
    }
    // The boundary values are accepted, and each is the effective bound. One
    // second can run out on a loaded machine before the worker returns; the
    // refusal then names the bound, which is just as much the effective one.
    for accepted in [1000, 120_000] {
        let run = sandbox.inspect(&[
            "--kind",
            "impl",
            "--timeout-ms",
            &accepted.to_string(),
            "--json",
        ]);
        let effective = if run.code == Some(0) {
            run.report()["bounds"]["selection"].clone()
        } else {
            let mut bound = run.refusal(124)["error"]["bound"].clone();
            assert_eq!(bound["name"], "selection");
            bound.as_object_mut().unwrap().remove("name");
            bound
        };
        assert_eq!(
            effective,
            serde_json::json!({ "ms": accepted, "from": "--timeout-ms" })
        );
    }
    assert!(sentinel.exists(), "the accepted bounds never evaluated");
}

#[test]
fn inspection_reports_the_effective_selection_bound() {
    let sandbox = Sandbox::new();
    sandbox.personal_policy(ROUTED);

    let report = sandbox.inspect(&["--kind", "impl", "--json"]).report();
    assert_eq!(
        report["bounds"]["selection"],
        serde_json::json!({ "ms": 30_000, "from": "default" })
    );

    let human = sandbox.inspect(&["--kind", "impl"]);
    assert_eq!(human.code, Some(0), "{}", human.stderr);
    assert!(
        human
            .stdout
            .contains("selection within 30000 ms (the default)"),
        "{}",
        human.stdout
    );
    let human = sandbox.inspect(&["--kind", "impl", "--timeout-ms", "20000"]);
    assert!(
        human
            .stdout
            .contains("selection within 20000 ms (--timeout-ms)"),
        "{}",
        human.stdout
    );
}

#[test]
fn a_timeout_in_text_mode_names_the_bound_and_keeps_the_policy_output() {
    let sandbox = Sandbox::new();
    let pid_file = sandbox.root.join("worker-pid");
    sandbox.personal_policy(&holding(
        &pid_file,
        Place::Import,
        Hold::Pending,
        None,
        "console.log(\"loading the review table\");",
    ));

    let mut invocation = sandbox.command();
    let bound = BOUND_MS.to_string();
    invocation.args([
        "run",
        "--kind",
        "impl",
        "--prompt",
        "the prompt",
        "--timeout-ms",
        &bound,
    ]);
    let timed = guarded(&mut invocation, &pid_file, WATCHDOG);

    assert_eq!(timed.run.code, Some(124), "{}", timed.run.stderr);
    assert_eq!(timed.run.stdout, "");
    let stderr = &timed.run.stderr;
    assert!(
        stderr.contains("policy stdout: loading the review table"),
        "{stderr}"
    );
    assert!(
        stderr.contains("refused (selection_timeout, stage evaluation)"),
        "{stderr}"
    );
    assert!(
        stderr.contains(&format!("bound: selection {BOUND_MS} ms (--timeout-ms)")),
        "{stderr}"
    );
    assert!(!sandbox.harness_ran());
}

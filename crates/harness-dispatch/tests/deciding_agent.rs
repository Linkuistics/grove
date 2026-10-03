//! Dynamic dispatch, through the command seam
//! (`docs/specs/harness-selection-and-execution.md`, *Execution and
//! authority*).
//!
//! A policy hands the prompt to a deciding agent that it starts in the
//! caller's directory, waits for and reaps, and maps the agent's answer to a
//! command. No agent policy ships and no test calls a model: a script stands
//! in for the agent. It records where it ran and the prompt it was given, and
//! answers what the test told it to, so the answer is seen to decide which
//! command launches. A stand-in still running at the selection bound launches
//! nothing, and the bound is the owner's setting.

mod support;

use std::fs;
use std::time::{Duration, Instant};

use serde_json::json;
use support::hold::{exists, guarded, recorded_pid};
use support::{executable, text, Sandbox};

/// A prompt with the punctuation and newlines a mandate holds.
const PROMPT: &str = "Load the skill; then \"carry out\" $TASK.\n\nDo it `now`.\n";

/// The answer that makes the stand-in wait instead of answering.
const NEVER: &str = "<never>";

const WATCHDOG: Duration = Duration::from_secs(20);

/// The stand-in for a deciding agent, on the worker's PATH. It records its
/// PID, its physical cwd and its one argument beside the test's `answer`
/// file, then prints that file's text, or waits when the text is [`NEVER`].
fn stand_in(sandbox: &Sandbox, answer: &str) {
    let agent = sandbox.root.join("agent");
    fs::create_dir(&agent).unwrap();
    fs::write(agent.join("answer"), answer).unwrap();
    executable(
        &sandbox.bin.join("deciding-agent"),
        &format!(
            "#!/bin/sh\n\
             agent={agent:?}\n\
             echo $$ > \"$agent/pid\"\n\
             pwd -P > \"$agent/cwd\"\n\
             printf '%s' \"$1\" > \"$agent/prompt\"\n\
             answer=$(cat \"$agent/answer\")\n\
             if [ \"$answer\" = '{NEVER}' ]; then exec sleep 60; fi\n\
             printf '%s\\n' \"$answer\"\n",
            agent = text(&agent),
        ),
    );
}

/// A policy whose `select` starts the stand-in in the caller's directory with
/// the prompt, under the host's signal, reaps it, and looks its answer up
/// among the commands the policy itself wrote. The answer picks an entry and
/// never reaches the program or its arguments.
fn deciding_policy(sandbox: &Sandbox) -> String {
    format!(
        r#"import {{ writeFileSync }} from "node:fs";
const COMMANDS = {{
  deep: {{ model: "model-large", effort: "high" }},
  quick: {{ model: "model-small", effort: "low" }},
}};
export const policy = {{
  schemaVersion: 2,
  version: "deciding-1",
  async select(request, context, host) {{
    writeFileSync({pid:?}, String(process.pid));
    // cwd, a piped stdout, `signal` and `exited`:
    // https://bun.com/docs/runtime/child-process
    const agent = Bun.spawn(["deciding-agent", request.prompt], {{
      cwd: request.cwd, stdin: "ignore", stdout: "pipe", stderr: "ignore", signal: host.signal,
    }});
    const answer = (await new Response(agent.stdout).text()).trim();
    await agent.exited;
    const chosen = COMMANDS[answer];
    if (chosen === undefined) {{
      return {{ status: "refused", code: "answer_unknown", message: `the deciding agent answered ${{JSON.stringify(answer)}}`, remedy: "have it answer deep or quick" }};
    }}
    return {{
      status: "selected", program: "fake-harness", args: ["--effort", chosen.effort, request.prompt],
      provider: "origin-a", model: chosen.model, effort: chosen.effort,
      reason: `the deciding agent answered ${{answer}}`,
    }};
  }},
}};
"#,
        pid = text(&sandbox.root.join("worker-pid")),
    )
}

fn deciding(answer: &str) -> Sandbox {
    let sandbox = Sandbox::new();
    stand_in(&sandbox, answer);
    sandbox.personal_policy(&deciding_policy(&sandbox));
    sandbox
}

fn recorded(sandbox: &Sandbox, name: &str) -> String {
    fs::read_to_string(sandbox.root.join("agent").join(name))
        .unwrap_or_else(|error| panic!("the stand-in recorded no {name}: {error}"))
}

#[test]
fn a_deciding_agents_answer_decides_which_command_launches() {
    for (answer, model, effort) in [
        ("deep", "model-large", "high"),
        ("quick", "model-small", "low"),
    ] {
        let sandbox = deciding(answer);
        let launched = sandbox.run(&["--kind", "impl", "--prompt", PROMPT, "--json"]);
        assert_eq!(launched.code, Some(0), "{answer}: {}", launched.stderr);

        // The stand-in ran in the caller's directory, where the worker itself
        // never is, and was given the prompt byte for byte.
        assert_eq!(recorded(&sandbox, "cwd").trim_end(), text(&sandbox.cwd));
        assert_eq!(recorded(&sandbox, "prompt"), PROMPT);
        // Its answer chose the command, and the labels the run records.
        assert_eq!(sandbox.harness_args(), ["--effort", effort, PROMPT]);
        let notice = launched.handoff();
        assert_eq!(notice["handoff"]["model"], model, "{answer}");
        assert_eq!(notice["handoff"]["effort"], effort, "{answer}");
        assert_eq!(
            notice["handoff"]["reason"],
            format!("the deciding agent answered {answer}")
        );
        // The policy reaped it before it returned.
        let agent = recorded(&sandbox, "pid").trim().parse().unwrap();
        assert!(!exists(agent), "{answer}: the stand-in outlived selection");
    }
}

#[test]
fn inspection_runs_the_deciding_agent_and_reports_its_choice() {
    let sandbox = deciding("quick");
    let report = sandbox
        .inspect(&["--kind", "impl", "--prompt", PROMPT, "--json"])
        .report();
    assert_eq!(recorded(&sandbox, "prompt"), PROMPT);
    assert_eq!(
        report["command"]["args"],
        json!(["--effort", "low", PROMPT])
    );
    assert_eq!(report["selection"]["model"], "model-small");
    assert!(!sandbox.harness_ran());
}

#[test]
fn an_answer_the_policy_does_not_offer_launches_nothing() {
    // The stand-in answers with text no entry holds. The policy refuses, and
    // the text never becomes a program or an argument.
    let sandbox = deciding("fake-harness --effort max");
    let refusal = sandbox
        .run(&["--kind", "impl", "--prompt", PROMPT, "--json"])
        .refusal(3);
    assert_eq!(refusal["error"]["code"], "policy_refused", "{refusal}");
    assert_eq!(
        refusal["error"]["policyCode"], "answer_unknown",
        "{refusal}"
    );
    assert!(!sandbox.harness_ran());
}

#[test]
fn a_deciding_agent_still_running_at_the_bound_launches_nothing() {
    const BOUND_MS: u64 = 3000;
    for command in ["inspect", "run"] {
        let sandbox = deciding(NEVER);
        // The owner's setting is the agent's time: no flag is passed.
        sandbox.settings(&json!({ "timeoutMs": BOUND_MS }));
        let mut invocation = sandbox.command();
        invocation.args([command, "--kind", "impl", "--prompt", PROMPT, "--json"]);
        let timed = guarded(&mut invocation, &sandbox.root.join("worker-pid"), WATCHDOG);

        let agent: libc::pid_t = recorded(&sandbox, "pid").trim().parse().unwrap();
        // The front stops its worker, whose host signal aborts, which stops
        // the child the policy started under it. An orphan is reaped by
        // whatever adopts it, so its exit is waited for.
        let waited = Instant::now();
        while exists(agent) && waited.elapsed() < Duration::from_secs(5) {
            std::thread::sleep(Duration::from_millis(10));
        }
        let survived = exists(agent);
        if survived {
            // SAFETY: kill only sends a signal.
            unsafe { libc::kill(agent, libc::SIGKILL) };
        }

        let refusal = timed.run.refusal(124);
        assert_eq!(refusal["error"]["code"], "selection_timeout", "{refusal}");
        assert_eq!(
            refusal["error"]["bound"],
            json!({ "name": "selection", "ms": BOUND_MS, "from": "settings.json" }),
            "{command}"
        );
        assert!(
            timed.elapsed >= Duration::from_millis(BOUND_MS),
            "{command}: refused before the bound ran out, after {:?}",
            timed.elapsed
        );
        // It was running, with the prompt, when the bound ran out.
        assert_eq!(recorded(&sandbox, "prompt"), PROMPT, "{command}");
        assert!(!sandbox.harness_ran(), "{command}: a harness was launched");
        assert!(
            !sandbox.default_store().exists(),
            "{command}: a run was recorded"
        );
        let worker = recorded_pid(&sandbox.root.join("worker-pid")).unwrap();
        assert!(!exists(worker), "{command}: the worker survived the front");
        assert!(!survived, "{command}: the stand-in survived the selection");
    }
}

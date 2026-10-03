# plan-k1


## Goal

Establish what "use harness-dispatch as the harness execution wrapper" should
build, in the owner's words: settle the open questions with the human one at a
time, agree the test seams the work will be tested through, and leave the
tree ready for the session that designs it.

## Context

The owner's own words, the only inputs a fresh grove has:

- The process-wrapper direction recorded under *Keep a supervisor after
  launching the harness* in `docs/adr/policy-evaluation-precedes-process-replacement.md`
  (promoted by `finish-k25`): "The owner wants that design eventually:
  dispatch would handle the launched harness's completion signal and so become
  the process wrapper for one run of an interactive harness. That moves the
  runner's completion channel, supervision, kill escalation, terminal handover
  and confinement under dispatch."
- In this session: "I think we can require that harness-dispatch is run from
  the same location that the prompt assumes, as if it were run in the
  repo-root in the case of grove. So we don't need to pass the repo as a
  param."

Records this work reopens, each of which names the reopening:

- `docs/adr/policy-evaluation-precedes-process-replacement.md` — *Keep a
  supervisor after launching the harness*: "Reopen only with an independently
  agreed supervisor/terminal design."
- `docs/adr/harness-selection-is-owned-by-policy.md` — *Have the executable
  launch a confined invocation too*: "Reopen when the executable supervises the
  harness itself … or when a standalone invocation needs a run record."
- `docs/specs/harness-selection-and-execution.md` — *Out of scope*:
  "Supervising the harness and handling its completion. This design keeps the
  `exec` handoff." Its package-boundary table gives Grove "the foreground job,
  the completion channel and standalone confinement".
- `docs/adr/the-launched-child-is-a-job.md`, `docs/adr/one-live-driver-per-working-tree.md`
  and decision 7 of `docs/specs/module-decomposition.md` — the runner
  (`keyed-launch`) and the epoch-bound loop control channel it watches.

Facts established from the code before any question was asked:

- The sample policy, and the owner's installed copy of it, place the `repo`
  parameter in `--add-dir` for both `claude` and `codex`, so that a harness in
  a secondary jj workspace reaches its store, and refuse a launch without it.
  `repo` is the main repository root, which differs from the working tree in a
  secondary workspace. No shipped policy reads `worktree`.
- A secondary workspace's `.jj/repo` is a one-line file holding the store's
  path relative to `.jj/` (`../../grove/.jj/repo` in this grove).
- Confinement is `sandbox-exec` (Seatbelt) on macOS and bubblewrap on Linux,
  inside `keyed-launch`.

## Done when

- Every open question below the threshold is recorded, and every one above it
  has been put to the human singly and answered, each in the running log.
- The test seams are agreed with the human and recorded.
- The root brief states the settled requirements and the next leaf is cut.

## Notes

## Decisions (running log)

**D1 — dispatch runs where the prompt assumes, and Grove passes no location.**
The caller runs `harness-dispatch` in the location the prompt assumes: the
working-tree root for a lifecycle session, the staged directory for `grove
run`. That is dispatch's documented contract, and the policy may rely on it.
Grove stops passing `--param worktree=` (already redundant with the request's
`cwd`) and `--param repo=`. A policy that must grant a secondary workspace's
harness its jj store derives the store from `<cwd>/.jj/repo` itself; the
sample policy shows how, and nothing jj-specific enters dispatch's core, SDK
or adapter. The owner's principle, in their words: dispatch "has the same
prompt and cwd" as the harness, "and we have already required the harness to
have the required skills". Accepted cost: an owner policy that lists `repo` as
required (the installed sample does) refuses every launch it routes to
`claude` or `codex` until edited, so the release notes must say so.

**D2 — Grove passes no selection parameter at all; session naming is the
skill's.** Asked whether `--param session_name=` survives D1 as the one value
Grove still supplies, the owner answered: "If it is the skill that does it, we
should leave it to the skill." The methodology already states the convention
(`<repo-basename>: <name> grove`) and derives both names from the working tree
(`references/driver.md`, *Deriving the session name yourself*), so Grove stops
computing and passing `session_name` for lifecycle sessions and
`standalone:<kind>` for `grove run`. A lifecycle launch then carries the kind,
the task file, the task identity and the prompt, in the working-tree root.
Consequence stated to the owner: launch-time naming, which the sample does
today with `claude -n`, becomes a policy's own derivation from its cwd if an
owner wants it; otherwise the skill's `/rename` suggestion is the mechanism.

**D3 — `harness-dispatch run` always supervises.** Offered always-supervise,
opt-in supervision, or always-supervise plus a separate verb that keeps
today's `exec` handoff, the owner answered "3 would be ok": `run` spawns the
harness as its child and stays its parent to the end, for every caller, and a
separate verb may keep the `exec` handoff for a caller that needs the harness
to keep its PID. Grove launches through `run`. Facts behind it: the Grove
driver is the only caller of `run` in the repository (`grove run` uses
`inspect`), and the spec already says neither the result contract nor
inspection depends on `exec`.

**D4 — dispatch alone catches the harness's exit, and a session's completion
signal becomes dispatch's, purely an exit signal.** In the owner's words: "I
want harness-dispatch to be *solely* responsible for catching the harness exit
- i.e. grove complete needs to change to being the harness-dispatch
responsibility, as purely an exit signal." So the verb a session ends with is
dispatch's, its meaning is "this harness run is over", and dispatch ends the
harness, reaps it and reports the end to its caller; Grove no longer watches a
channel or escalates. Consequences carried to design, not yet decided: Grove's
session-epoch admission keys on the `GROVE_SIGNAL_FILE` path today, so the
stale-session guarantee for `grove-llm` tree verbs needs a launch identity
that survives the signal moving; and the loop's three endings (teardown,
reopening, declining — *Complete finish cycle*) need the disposition that the
`relaunch`/`done` token carried today to reach Grove some other way (D5).

**D5 — Grove records teardown itself; the exit signal carries nothing.**
Offered an opaque value that dispatch hands back unread, a Grove-side record,
or inference from `.grove/` being gone, the owner chose: "Grove records done
itself". So dispatch's exit signal is literally pure. Dispatch reports to its
caller whether the run ended through that signal or by the harness exiting on
its own, which every option needed: without it a human quitting the harness
would relaunch the leaf instead of stopping the loop. A Grove verb records the
teardown disposition in Grove's own launch-scoped control area, and a finish
session runs it before its final exit signal, so a teardown is two final
commands and stays a deliberate last act (this repo's finish procedure sends
it only after integration and release). The loop then reads: teardown
recorded → finished; ended through the exit signal → reopening; otherwise →
declining. Left to design: the verb's name and form (today's `complete
--done`, reduced to recording, is one candidate), and what a recorded teardown
followed by the harness exiting on its own means.

**D6 — confinement moves under dispatch (pre-decided by the owner's
direction).** The direction lists confinement with the completion channel,
supervision, kill escalation and terminal handover as what moves under
dispatch, and the policy-ownership ADR's *Have the executable launch a
confined invocation too* names this as its reopen condition. So `grove run`
launches through dispatch's `run`, which selects outside the sandbox with the
owner's grants and bounds and confines the harness it spawns. Grove keeps
what is a standalone invocation's own: staging the private directory, the
inputs and the declared outputs, and publishing after success. Consequence: a
standalone invocation gets a run record and its harness a run ID, because the
reason it had none ("the caller [would] set dispatch's run variables for a
harness dispatch did not launch") no longer holds; the confined harness still
cannot read the policy, the owner settings or the record store.

**D7 — dispatch records the run's end itself.** Offered recording the end,
recording nothing new, or the end plus an after-run policy hook for usage, the
owner chose to record the end: when the harness ends, dispatch appends its own
observation to the run — execution confirmed, how the run ended (its exit
signal, the harness exiting on its own, or cancellation), the exit code or
signal, and the duration — in the existing observation format, with dispatch
as its source. Usage, acceptance and the other measurements stay with outside
observers through `record observe`. A usage hook stays on the horizon. A
dispatch that is itself killed leaves its run as it stood.

**D8 — the finish cuts a major release.** The owner chose `task
release:major` over CLAUDE.md's default minor: owner policies that read `repo`
or `session_name` refuse (D1, D2), `run` stops replacing its process (D3), and
the verb a session ends with moves (D4), so the skills and the binaries must be
upgraded together. The release notes tell an owner to update such a policy and
to upgrade both halves at once. This is the increment the finish session
proposes; CLAUDE.md's whole finish sequence otherwise stands.

**D9 — the test seams, agreed as sketched.** Four existing seams are the
public acceptance instruments; internal tests support them and never replace
them, and no automated test calls a model:

1. Dispatch's command with fake harnesses under a controlling PTY, absorbing
   the runner's own interface cases: the harness in its own process group with
   the terminal, reclaimed afterwards with its modes restored even after a
   raw-mode fake is killed; the entry signal state reaching it; a typed Ctrl-C
   reaching only it; the exit signal driving grace → TERM → kill-grace → KILL
   on its group, descendants reaped; its own exit code or signal reproduced;
   the run's ending reported to the caller; a fresh exit channel per run, so a
   stale signal cannot end a later run.
2. Dispatch's records: after a supervised run, `record show` carries dispatch's
   own end observation (D7); a killed dispatch leaves the attempt as it stood;
   outside imports still append and correct.
3. Grove's launch boundary — the real driver, front and worker, a fake harness
   and a temporary-HOME policy, under a PTY: no `--param` reaches `select` and
   its `cwd` is the working-tree root; the three endings (exit signal →
   relaunch; Grove's teardown record then exit signal → finished; the harness
   exiting on its own → stopped with the leaf live); a stale session's tree
   verbs refused after the epoch rotates.
4. `grove run` under real confinement with a deterministic harness: dispatch
   selects outside the sandbox and confines the harness, which receives a
   recorded run ID and cannot read the policy, the owner settings or the
   record store; outputs publish only on the harness's acknowledgement, never
   on its exit alone.

**D10 — no `review-requirements`; the next leaf is `design`.** The owner
confirmed the shared understanding. Every decision above is the owner's own
answer, given live, so an adversarial read of this leaf would review the
owner's choices rather than a producer's synthesis; the load-bearing artifact
is the reworked specification, and the design session judges its review
chain. The root brief carries the settled requirements, the guarantees to
keep, the questions left to design and the agreed seams, because later
sessions read the brief chain and not this retired leaf. `harness-wrapper-k2`
(`design`) is cut; it externalizes planning.

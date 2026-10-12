# plan-k1

## Goal

Establish the requirements for extracting harness-dispatch into the top-level
Ravel project: ownership, packaging, deployment, Grove's consumption of it, and the
acceptance checks for the extraction. Include dispatch-owned controlled-session
instructions and explicit force-stop and continue endings.

## Context

- `docs/specs/harness-selection-and-execution.md` defines the existing package
  boundary and anticipates extraction with keyed-launch.
- `docs/adr/harness-selection-is-owned-by-policy.md` records why Grove is a
  consumer of a generic tool.
- `docs/adr/dispatch-supervises-the-harness.md` records the shared runner and
  the run-ending interface Grove consumes.
- The old Ravel checkout's `jj status` showed only a README edit, snapshotted in
  change `uwlrknks` (commit `93f202ba`) above `main` at `2f452e51`. Preserve that
  work before replacing the active project. Its remote is
  `https://github.com/Linkuistics/Ravel.git`.

## Done when

The human and this session share an explicit extraction scope, delivery and
dependency choices, and agreed test seams. Record the decisions as they settle
and leave a concrete handoff for the next session.

## Notes

The human's intent: "I want to extract the harness-dispatch tool to a top-level
project. After the mechanical issue of moving code, packaging, deployment, I
eventually want to build an agent-server that provides a centralised place that
manages named agent sessions using a hierarchical naming schema, allows agents
to communicate, allows tooling to present the entirety of the work i.e. a
dashboard, and allows a terminal-like system to selectively interact with
agents."

## Decisions (running log)

**Immediate goal and longer-term direction.** Extract the existing tool to a
top-level project, including the move, packaging and deployment. The agent-server
is the later direction: hierarchical session names, agent communication, a view
of the whole work, and selective terminal interaction. Its architecture and
implementation are not requirements to invent during the mechanical extraction.

**Independent delivery.** The human selected a separate repository, releases and
Homebrew formula. Grove consumes the independently installed tool rather than
releasing its own copy of the tool. The concrete project, command, repository
and formula names must follow the replacement name requested below.

**Controlled-session instructions.** When dispatch starts an agent, dispatch is
responsible for injecting notification that it is running in a controlled
session, plus instructions for both ways to end that session.

**The two endings.** In the human's words, "stop-now == force-stop, next-step ==
continue (which might, at the end of a grove session, also stop the grove
loop)". Continue permits the caller's normal continuation decision; it does not
promise another launch. Force-stop requests that the current sequence stop.

**Default ending.** The human selected continue as the default for a plain
`harness-dispatch exit`. Force-stop is explicit. Preserve the bare command's
existing continuation meaning for current callers and session instructions.
The corresponding renamed command is `ravel exit`.

**Runtime installation.** The human wants Homebrew dependencies to ensure the
launch executable is installed. After clarification of the existing runner's
library shape, they explicitly required "no shared rust library - just an
executable". Grove's cross-project dependency is the installed `ravel` command.
There will be no Git-tag or crates.io Rust-library dependency between Grove and
Ravel. Keep each project's internal implementation local and preserve launch,
cancellation, reaping and terminal behavior at the executable boundary.

**Replacement name.** The human requested that the extracted project cease to
be called harness-dispatch and chose **Ravel**. There is an existing older Ravel
project that may be retired; the agreed replacement route is recorded below.

**Retiring old Ravel.** The human selected reuse of the existing
`Linkuistics/Ravel` repository and `/Users/antony/Development/Ravel` checkout.
Preserve the old revision, including its current local README edit, under a
named retired revision before replacing the active source. Earlier daemon
designs are retired history, not requirements for the extracted tool.

**Minimal increments.** The human explicitly requested "minimal increments"
and "not go down any rabbit holes or excursions". Prefer the existing
architecture and test seams. Deliver extraction/renaming/independent installation
first, then the controlled-session framing and explicit endings as a separate
small increment. Keep the agent-server as a horizon note; cut only work whose
next useful result is already concrete.

**Agreed test seams.** The human accepted reuse of the existing seams: the
public Ravel CLI with its real worker and fake agents for framing, endings and
records; Grove's lifecycle and standalone launch suites through the installed
executable for continuation, completion, force-stop and terminal cleanup; and
archive/Homebrew installation checks on the current supported targets. Preserve
the existing internal coverage as supporting evidence. No model calls or new
testing infrastructure are needed for these acceptance checks.

**Caller independence.** In response to the written scope review, the human
clarified: "Ravel does not need to know anything about grove. It can be used in
other contexts, with other drivers. Grove needs only know about the ravel cli
and params." Ravel's package, worker, SDK, defaults and controlled-session
instructions stay generic. Grove-specific adapters, policies, examples and
integration tests belong on Grove's side. Grove uses the documented CLI and
parameters; no embedded adapter, private protocol or cross-project Rust library
forms part of that boundary. This replaces the brief's initial suggestion to
keep the optional Grove adapter in the extracted package.

**Configuration continuity.** The human required the existing harness-dispatch
configuration format and copying their current configuration to the new config
directory. Use `~/.config/ravel` and copy the actual existing
`~/.config/harness-dispatch` contents, preserving policy choices and settings.
At requirements inspection the source contained `policy.ts` and the destination
did not exist. Include this in the first runnable increment. Necessary import
relocations must honor the caller-independent boundary without rewriting the
owner's selection policy or adding a new configuration format.

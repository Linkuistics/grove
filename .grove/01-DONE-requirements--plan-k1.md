# plan-k1

## Goal

Establish, in the human's own words, what this grove builds. Its only input was
its name, *use harness-dispatch directly for execution*: both briefs were empty
stubs.

## Context

- The consolidated result is the root brief's *Settled requirements*, *Done
  when* and *Out of scope*. This file holds how each was reached.
- Read against: `crates/harness-dispatch/README.md`,
  `docs/specs/harness-selection-and-execution.md`,
  `docs/adr/harness-selection-is-owned-by-policy.md`,
  `docs/adr/policy-evaluation-precedes-process-replacement.md`,
  `docs/adr/complete-session-configuration.md`,
  `docs/adr/the-launched-child-is-a-job.md`, `docs/CONFIGURATION.md`,
  `docs/ARCHITECTURE.md` (*Session configuration*, *The harness-dispatch
  package*), `docs/specs/standalone-invocations.md`,
  `docs/research/grove-model-effort-routing.md`, and the human's own
  `~/.config/grove/config.kdl`, pinned as `.grove/source-config.kdl`.

## Done when

- Every open question whose answer is the human's has been put to them, one at a
  time, and its answer is in the log below.
- The human has confirmed the whole understanding.
- The root brief states the result, the terms resolved are in `CONTEXT.md`, and
  the leaves that continue the work exist.

## Notes

The log is in the order the decisions settled, and where two entries differ the
later one binds. Three early entries were refined: a direct harness as *a policy
candidate* and the launch values reaching dispatch by a mechanism left to design
were both settled by *a successful `select` returns the command itself*; and the
entry that has harness-dispatch install the sample when no policy exists was
replaced by the one after it. Entries marked *derived, not asked* were put to
the human in the closing summary, which they confirmed.

## Decisions (running log)

**What the grove's name means (human, 2026-10-02).** *Use harness-dispatch
directly for execution* means: Grove launches its lifecycle sessions through
`harness-dispatch run` as its own built-in route, with no `config.kdl` command
definition wrapping it, and `~/.config/harness-dispatch/policy.ts` becomes where
harness, model and reasoning effort are chosen. Chosen over three other readings
the name admits: migrating the human's personal setup only, with no change to
what Grove launches; giving confined `grove run` a dispatch integration; and
having sessions call dispatch for work they delegate in-session.

**`config.kdl` is removed outright for lifecycle sessions (human, 2026-10-02).**
Dispatch is the only way a lifecycle session launches. The modular configuration
(commands, bindings, routes, parameters, profiles, selection), the `.grove.kdl`
delta, `grove config show` and `grove config examples` are deleted in the same
change, as a breaking release; a direct harness is written as a policy candidate
instead. Chosen over keeping `config.kdl` as a permanent per-kind override, and
over deprecating it for a transition period. The human accepted the costs named
with the recommendation: the break for every owner, and three capabilities with
no policy equivalent yet (`${session_name}` / `${repo}` / `${worktree}`, the
per-worktree delta, and `grove run`'s standalone routes), each of which is its
own question below.

**Standalone `grove run` launches through dispatch too (human, 2026-10-02).**
`policy.ts` routes a standalone kind such as `release-notes` like any other, so
`config.kdl` and all of its machinery are deleted completely, with no surviving
consumer. Chosen over keeping `config.kdl` for standalone kinds only, and over
deferring the move to a later grove. The accepted cost: this grove takes on the
confined integration the dispatch spec lists as out of scope. `grove run`
confines its child's writes to the invocation directory, while
`harness-dispatch run` reads the personal policy and commits a run record under
`~/.local/state/`, and always `exec`s the harness itself; so selection has to
happen outside the sandbox and only the harness inside it, which dispatch cannot
do today. How is the design session's. The release tasks' notes path
(`scripts/release-notes.sh` → `grove run release-notes`) changes under them.

**harness-dispatch stays independent of Grove (human, 2026-10-02).** It remains
a separate command and package with no Grove dependency and no Grove concept in
its core; Grove hard-requires it and is one caller among many. The dependency
reverses in one direction only. Whatever Grove needs from dispatch is added as a
generic caller capability. Chosen over folding the package into Grove as its
launch module, and over a separate command that carries Grove-specific flags and
slots in its core. This keeps *harness-selection-is-owned-by-policy*'s
independence and reverses only its sentence that Grove's command configuration
is the integration point, together with the architecture's rule that Grove must
not depend on the package in any shipped source.

**Derived, not asked: no launch value is lost.** Every value a Grove command
template can place in argv today stays available to a policy candidate's argv as
one whole argument: beside the prompt, kind, task file and task identity that
dispatch already has, that is the session name, the working-tree root and the
main-repository root (for a standalone invocation, the values `grove run` gives
those names today). The human's own commands use two of them (`-n
${session_name}`, `--add-dir ${repo}`, the second being what lets a secondary jj
workspace reach its store), and dispatch has no slot for any of the three. By the
independence decision they reach dispatch as generic caller-supplied data, not
as Grove vocabulary; the mechanism is the design session's.

**Selection must be able to see the grove's context (human, 2026-10-02,
unprompted).** In the human's words: *"We need to ensure that harness-dispatch
has enough data to pass to an LLM to do the selection - this means it needs
enough context from the grove. Maybe the prompt does this by referring to
skills/step files/brief chain."* So an LLM-driven `select` has to be feasible,
and the context it needs is named: the kind's skill and step files, and the
brief chain, beside the task file. Facts found against it: the policy never
receives the launch prompt, by the dispatch spec's own rule (*supplying it or
not cannot change the selection*); that prompt holds pointers rather than
content, and the request already carries the same pointers as data (`kind`,
`taskFile`, `taskId`); the Grove adapter reads the one task file and "no brief,
sibling or other file of the tree"; and no shipped policy calls a model. The
routing research anticipated this: *"A Grove context loader can read its brief
chain"*, with the caution *"Do not stuff a whole repository into every selection
call."*

**The launch prompt is exposed to the selection policy (human, 2026-10-02).**
The policy receives the prompt and resolves its references itself; Grove passes
nothing new for selection's sake. Chosen against the recommendation (Grove
passes the authoritative locations as caller data and the prompt stays
withheld), and over Grove assembling the text and over the adapter deriving the
chain. This reverses a rule the dispatch spec and README both state, that the
worker *sees neither the launch prompt nor reserved final environment* and that
*supplying it or not cannot change the selection*, and the routing research's
*not the control-bearing Grove mandate by default*. Consequences the design
session inherits rather than decisions made here: `inspect` reproduces a
prompt-dependent selection only when given the same prompt, and a refused
launch's `inspect:` line omits the prompt today; the prompt has a 1 MiB bound
of its own beside a 256 KiB default context budget; and the text a selector
receives is mandate-shaped, so a selector that can act (an agent with tools)
must be kept from executing the task or touching the tree. The completion
channel is not in the prompt: `GROVE_SIGNAL_FILE` is an environment variable
the worker is never given.

**Dynamic dispatch is done by a deciding agent, and the prompt is its input
(human, 2026-10-02).** In the human's words: *"We should assume that the agent
that reads the prompt for a dynamic dispatch will a) have access to all the
skills (as every agent should have); and b) enough context (as simple as the
right directory so it knows the grove) to resolve the files. Note that having
the skills means that the deciding agent will have access to the grove skill,
enough to full evaluate the task to determine the right harness/model/effort."*
And, unprompted: *"I think the prompt is actually critical for the dispatch when
it does dynamic dispatch based on an evaluation of the task. It needs more than
the task kind, even though that is an input. For static/non-LLM based dispatch,
the prompt is irrelevant, but not for dynamic dispatch."* So: the selector for
dynamic dispatch is an agent that is handed the launch prompt; it is assumed to
have the skills installed and to run in the grove's working tree, and from those
two it resolves everything else the way a session does. Static dispatch never
depends on the prompt, so the old invariant survives for a `routes` policy and
for any policy that ignores the prompt. What this asks of dispatch is small and
mostly already true: the prompt reaches the policy (decided above), a policy may
start a child and reap it (permitted today), and the caller's directory reaches
the policy as `request.cwd` (today; the worker itself runs in `/`).

**Derived, not asked: the selection bound must admit a deciding agent.** The
whole selection is bounded at 30 seconds by default and 120 at most, sized for a
table lookup or an HTTP call. An agent that loads a skill and reads a brief
chain to *"full[y] evaluate the task"* needs minutes. The bound stays finite and
owner-set, and its default stays short so a static policy still fails fast; the
ceiling rises to the minutes range. The number is the design session's.

**This grove makes dynamic dispatch possible and ships no agent policy (human,
2026-10-02).** It delivers what a deciding agent needs (the prompt, the
directory, child processes, a workable bound, credentials) and proves it with a
scripted stand-in for the agent. The human writes their own deciding-agent
policy afterwards. Chosen over shipping an inactive example policy (the
recommendation), over an example plus a methodology skill for the deciding
agent, and over making the shipped Grove starter dynamic by default. *No local
LLM selector ships* therefore still holds, and no test calls a model. Because
nothing ships that wraps the prompt, keeping a deciding agent from executing
the task it is evaluating is its policy author's responsibility; what this
grove owes is a statement of that hazard where an owner writing such a policy
will read it.

**Per-worktree variation ships as a supported helper (human, 2026-10-02).** With
`.grove.kdl` gone, one checkout gets different launch choices from another
through policy code: a supported SDK helper or example that reads a local choice
file, documented and tested as delivered. Chosen over building nothing and
leaving the pattern to each owner (the recommendation), and over dispatch
discovering a local override file itself, which would reverse its
no-repository-override rule. The properties that make this safe without the
delta's trackedness refusal are requirements on the helper: the file is read
through a measured read, so its digest is in the run record; it can only steer
the choice among candidates the owner's catalog already holds, never name a
program or an argument; and it is taken up only because the owner's policy
imports the helper. The file's name, format and search locations are the design
session's.

**An unroutable kind is caught at launch only (human, 2026-10-02).** Tree verbs
consult no policy. Today Grove checks that kind K resolves before it writes a
leaf of K (`leaf-add`, `leaf-insert`, `leaf-decompose`, root scaffolding),
re-validates its configuration before every tree mutation, and requires a
`requirements` route before creating `.grove/`; all three go. An unroutable kind
surfaces when its leaf launches: dispatch refuses with its remedy and its
`inspect:` line, the leaf stays live, the loop stops, and rerunning `grove`
after fixing the policy continues. Chosen over an early check where it is cheap
(a new dispatch query answering for a static table only), and over checking
policy validity without the route. It extends to every kind the cost
*harness-selection-is-owned-by-policy* already accepted for dispatched ones, and
no early check could cover a deciding agent. What is given up: a session that
cuts a leaf of an unroutable kind no longer fails on the spot.

**Leftover files are ignored, and a converted sample ships instead of a
converter (human, 2026-10-02).** In the human's words: *"We should ignore
.grove.kdl files, although we should ship a sample config for harness-dispatch
that is a conversion of the existing grove config."* So Grove neither refuses
nor converts: an old file on disk is simply never read (the recommendation was a
refusal while one is present, by the delta ADR's own no-warn-and-fall-back
argument; the human chose otherwise). A missing policy is still reported, by
dispatch's own `policy_missing` refusal. What an upgrading owner gets in place
of a converter is a shipped sample policy that is a conversion of the existing
Grove configuration.

**The shipped sample converts the human's own `config.kdl` (human,
2026-10-02).** Their personal file as it stands, with the real `codex` and
`claude` command lines, models and efforts, covering the `routes`, `claude-led`,
`codex-sol` and other profiles. Chosen over converting the packaged example set
(the recommendation), and over shipping the packaged set while converting the
personal file only as an unshipped check. The human accepted that a release then
carries real harness flags and model names, where every shipped example so far
uses illustrative wrappers and placeholder models. The source is pinned so that
no later session depends on a live file outside the repository:
`.grove/source-config.kdl` is a byte copy of `~/.config/grove/config.kdl` as
modified 2026-09-30 12:27:14, 6063 bytes, SHA-256
`2294bd271586ae1723bdda4b6e6977a324666264ac7149be9560a78a3de22647`. It is a
foreign file the tree walk skips. What it exercises is the point: its commands
use `-n ${session_name}` and `--add-dir ${repo}`, a parameter inside a word
(`model_reasoning_effort=${param.effort}`), per-kind effort and model overrides,
four arrangements, two modifier profiles, and one standalone route
(`release-notes`).

**A policy is always a `select` function; static and dynamic are invisible
outside it (human, 2026-10-02).** Asked which meaning *dynamic* should carry,
since the shipped `harness-dispatch/examples/dynamic` is a deterministic
`select` that calls no model, the human answered: *"I think the config should
\*always\* be a select function, and whether it is static or dynamic is
invisible outside of the config file. What we call static just means that it is
driven by the kind without reference to the prompt or any content."* So the
terminology question dissolves into a contract change: the policy has one form,
`select`, and the separate `routes` form goes. Today a policy has *exactly one
of* `routes` and `select`, and the command reports which: `selection.form`,
`selectedBy: route`, the `incomplete_mapping` refusal located at
`policy.routes`, and a `--choice` a routes table takes without being able to
refuse it. All of that is the form being visible outside the file, so it goes
too. A table from kind to candidate survives as something an owner's `select`
does, which the SDK may supply as a helper; what the command used to check for a
table (every route names a candidate the catalog has) is then the helper's or
the type checker's to check. *Static* and *dynamic* stay as plain descriptions
of what an owner's `select` consults, and name nothing the command knows. The
example called `dynamic` is static in this sense, so it cannot keep that name.
Both terms and *deciding agent* are in `CONTEXT.md`.

**The test seams are the four that exist; no new one (human, 2026-10-02).** Put
to the human as "these are the seams, do they match what you expected?" and
agreed as listed: (1) the `harness-dispatch` command with temporary policies and
fake harnesses; (2) the Grove launch boundary, which is the real driver, the
real front and its compiled worker, a fake harness, and the policy in a
temporary HOME, with its controlling-terminal variant; (3) `grove run` under
real confinement with a deterministic harness; (4) the runner's own API with
fake programs. Every Grove test that launches a session goes through the real
front and worker, so the Bun-built worker becomes a precondition for those
tests, as it already is for the dispatch cases. Chosen over adding a way for
Grove's own tests to substitute a fake `harness-dispatch`: the fake would be a
second implementation of the command's contract, free to drift from the real
one. A deciding agent is stood in for by a script, and no test calls a model.

**The tool's shape, and where it is going (human, 2026-10-02, unprompted).** In
the human's words: *"Ultimately this looks like a trivial TS-execution wrapper
that passes it's CLI params as args to a TS function and acts according to the
return value (which should be an ADT). However, I would eventually like this
tool to handle the completion i.e. the ability for the executed harness to
signal it's completion. Then this becomes a way to execute a single run of an
interactive harness."* And then: *"... so it would also be a process wrapper."*
Facts found against it: `select`'s result is already a two-variant tagged union
(`selected` naming a candidate ID, `refused` with a code, message and remedy),
and the argv comes from the catalog entry that ID names, not from the function.
The process wrapper the human describes exists today on Grove's side: the
runner in `keyed-launch` allocates the completion channel, spawns the child
into its own process group, hands it the terminal, supervises it, escalates
grace, TERM and KILL to the group, and confines a standalone invocation.
Dispatch is deliberately not one: it `exec`s the harness and *nothing
supervises it afterwards*, and *policy-evaluation-precedes-process-replacement*
rejects keeping a supervisor, with the reopen condition *only with an
independently agreed supervisor/terminal design*.

**The process wrapper is a later grove (human, 2026-10-02).** This grove keeps
the `exec` handoff, and Grove's runner keeps the completion channel, supervision
and the terminal. Chosen over taking the wrapper on here, which would reopen the
ADR's rejected supervisor and need the terminal design it asks for (dispatch
supervising from inside the child's process group, a second terminal handover,
and the session epoch's binding to a channel path Grove no longer allocates).
What this grove owes the later one is to leave it open: the runner stays a
domain-free unit separable from Grove's loop, so that it can move under
dispatch; confined `grove run` is met by a small generic capability that
survives that move, not by machinery that assumes `exec` forever; and no
document this grove writes turns the ADR's reopen condition into a refusal.

**A successful `select` returns the command itself (human, 2026-10-02).** The
function receives every CLI parameter and returns the program and arguments to
run, with their provider, model and effort labels; the other variant stays a
refusal with a code, a message and a remedy. There is no exported catalog, no
slot vocabulary and no tool-level `--choice` rule. Chosen over keeping today's
contract, where the function names a catalog entry by ID and the tool expands
that entry's slots. This is what the human's description of the tool means, and
it settles two things recorded above as open: the launch values Grove must not
lose (session name, working-tree root, main-repository root) reach argv because
the function builds argv from its parameters, with no slot mechanism to extend;
and a parameter inside a word is ordinary string building. The human accepted
what the catalog bought and now moves: validating every candidate at load, and
the guarantee that no function result (and so no deciding agent's answer) can
introduce a program or an argument, become the job of the owner's function, the
type checker and SDK helpers. The human also accepted the cost: the contract
shipped in 21.13.0, and this rewrites the SDK, the worker protocol, the front's
catalog and choice handling, all five examples and their documentation. The
labels still have to reach the run record, because the review rule reads a
creator's provider back from it. For the per-worktree helper above, *candidates
the owner's catalog already holds* now reads *options the owner's policy offers
the helper*: the local file still names one of them and can introduce nothing.

**Shared understanding confirmed, with one addition (human, 2026-10-02).** The
whole summary was put to the human, the derived items included: a leftover
`config.kdl` is ignored as a leftover `.grove.kdl` is; Grove keeps no personal
configuration, and what an owner tunes about a launch is set on dispatch's side;
the selection ceiling rises into the minutes range; the sample's acceptance is
argument-for-argument parity with what the pinned `config.kdl` produces for
every kind; the release is a major one; and every current-state document
follows the code. The answer: *"yes, although harness-dispatch should install
the sample config if there is no config."* So when no policy exists,
harness-dispatch installs the sample as the owner's policy. That reverses a
rule stated in three places today: the dispatch spec's *no … installation of
policy into the user's configuration*, the README's *neither personal file
exists until you write it*, and *harness-selection-is-owned-by-policy*'s *the
executable invents no default*. It also changes what the sample is: until now
every shipped example was inert until an owner's policy imported it, and this
one becomes the policy a machine without one runs on.

**Installing the sample is an explicit subcommand; with no policy, dispatch
stops (human, 2026-10-02).** This replaces the entry above. Asked whether the
invocation that finds no policy should install and stop or install and launch,
given that the sample carries the human's `codex` command with full access and
approvals off, the human answered: *"actually, let's make the
install-a-sample-config into a specific subcommand, and if there is no config
otherwise we stop."* So nothing is installed implicitly: a harness-dispatch
subcommand installs the sample as the owner's policy, and `run` and `inspect`
with no policy refuse as they do today. The three rules the superseded entry
would have reversed therefore stand. What is new is the subcommand, and that
the missing-policy refusal can name it as its remedy. Its name, and what it
does when a policy already exists, are the design session's; the precedent is
`grove config examples`, which checks every destination first, creates only
what is missing and has no force option.

**Simplicity governs (human, 2026-10-02, unprompted, while the leaves were being
written).** *"Simplicity of this code is critical - do not overcomplicate,
otherthink or let the perfect be the enemy of the good."* It binds every later
session: where two designs meet a requirement, the one with less mechanism wins;
nothing is built that no requirement asks for; and a working part the
requirements do not touch is left alone. It is in the root brief under *Goal*,
and the design and planning leaves quote it. Under it the brief states two
requirements more narrowly than the entries above. The per-worktree helper is
held to what the human chose (supported, tested, and a file that can introduce
nothing); reading that file through a measured read is the design's to keep if
measured reads stay. And an owner's settings are required only for mechanisms
that survive the design.

**The requirements review is kept (human, 2026-10-02).** By that rule the
producing session proposed abandoning `plan-k2`, the review of this leaf, and
going straight to design. The human kept it. Its body was cut down so that it
reports only what would change what gets built or how it is tested.

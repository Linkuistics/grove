# session-probes-k281

## Goal

Add `session --harness claude|codex --route ravel|native` to the probe
program, on the measured launcher of probe-service-lifecycle-k280.

## Context

- The node brief states the observations that each route requires.
- probe-service-lifecycle-k280 delivers the launcher, its result line and
  the switch-side cleanup. Read its decisions log and its evidence section.
- `docs/evaluations/owner-skill-delivery/staging_trial.py` judges the same
  observations in the staging HOME: the mandatory order, unnamed and named
  skill reads, and the native listing. `scripts/harness_trial.py` holds the
  adapters of both harnesses.

## Done when

- A `ravel` session requires each observation of the node brief that the
  harness supports. A `native` session requires native discovery of the
  superseded skills and a native read of a Grove kind skill, other than
  `finish`, with one quoted line.
- Probe comparisons see each ordinary child of both native skill roots. They
  skip only the exact subtrees `synced` and `.system`.
- Each session mode passes once in the staging HOME, or carries a recorded
  limit. A control shows that each mode fails for a wrong layout.
- Each file that a probe session writes is recorded, with whether its MCP
  servers rewrite skill copies or agent definitions.

## Decisions (running log)

- The kind skill of every session probe is `grove-planning`. The
  `grove-core` source holds only six skills, and of its kind skills only the
  three planning kinds are not `finish`. The real HOME's native roots hold
  `grove-planning` too.
- The quoted line is the first line of the skill body that begins with
  `**`. That line differs between the snapshot copy and the native copy, so
  the quote shows which copy the agent read.
- The probe program imports `harness_trial` from its own folder, as it
  imports `owner_switch`. The adapters and the classifier extract the
  events. The verdicts are small functions in the probe program, modelled
  on `staging_trial.py`, because the prepared folder must not carry the
  staging runner and its real-HOME constants.
- The ravel route's `sh -c` script writes Ravel's request view and the
  harness's event stream to files in the work folder. Ravel prints only the
  session result on standard output.
- The superseded names are the base names of the plan's `absent` assertions
  below a native skill root. On the ravel route, each skill of the request
  view's search directories is a superseded name too.
- The write record walks the harness and tool folders of HOME and lists
  each new, changed or removed entry. It does not walk `.config`, `.cache`
  or `.local` as wholes, which hold over a million files. The record states
  that scope.
- The staging pass runs each case in a `cp -c` clone of the staging HOME.
  The Claude Code ravel case installs `technical-communication-hooks` in its
  clone, because owner-staging-hooks-k251 installs it in the retained HOME
  only later. A native case plants copies of the real HOME's superseded
  Codex entries in both native roots of its clone.
- The craft verdict accepts only `cli-tool-design` or `coding-style-python`,
  the craft skills that the probe's task needs. A first run credited
  `technical-communication`, which the hooks' instruction names. The tool
  task is now the first step of the prompt, as in the staging discovery
  case.
- The Claude Code session probe uses `claude-opus-5-5` at medium effort, as
  the owner's policy does for each Grove kind. The Codex probe uses
  `gpt-6.1-sol` at medium effort.
- The hooks plugin wrote `__pycache__/ste-lint.cpython-315.pyc` into the
  candidate keg's `writing` source during the first Claude Code ravel case.
  That changed the source's `resourceDigest`, and the next case's digest
  verdict failed. tc-hook-bytecode-k284 takes the fix before the hooks
  staging leaf. The staging runner removes and records each stray
  `__pycache__` before each case.
- No review chain follows this leaf. Each verdict has a fixture control
  that a mutation turned red. The staging pass ran each mode with the right
  layout and with a wrong one. The plan, rehearsal and acceptance leaves
  run these probes again before the switch.
- The write record also walks `~/.local/state/technical-communication`. The
  hooks keep their reply log there. The staging record showed it only as a
  new top-level `.local`, which the real HOME already has.

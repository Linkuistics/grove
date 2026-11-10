# owner-staging-hooks-k251

## Goal

Measure the complete simplified switch in the staging HOME. An idle,
signed-in Claude Code session with the owner's MCP servers runs beside it. Prove the quiet
interval that the design assumes.

## Context

- "The hooks installation inside finish" and "Claude Code's own writes" of
  `docs/specs/owner-skill-delivery.md` state the commands. The program backs
  up the whole write set, then runs Claude Code's commands serially. It
  registers `writing-hooks` from Grove's keg and installs
  `technical-communication-hooks@writing-hooks` disabled. It uses no held
  link and no exchange.
- The section "Test seams" requires this measurement. It covers the
  installation, native deactivation, capability enabling, both probes and
  receipt writes. The design's quiet-interval assumption depends on it, even
  with one owner.
- In staging, Grove's keg is the candidate keg of owner-staging-setup-k248.
  owner-switch-probes-k257 supplies the probes, and owner-switch-contraction-k270
  the program. Build a staging plan from the draft generator against the
  staging HOME.
- Claude Code writes `.in_use/<pid>` into each version directory that a
  session loaded. The section "Re-inventory before the cutover" of
  `docs/owner-skill-delivery-evidence.md` records it.

## Done when

- A signed-in Claude Code session with the owner's MCP servers stays idle in
  the staging HOME through the whole measurement. It uses that HOME and the
  held Claude Code version.
- The record shows each write of `settings.json` and of each registration
  during the interval, including writes that a later write overwrites. It
  separates the program's commands from the live parent and its servers.
- The record shows the metadata of each written file before and after. No
  ACL or extended attribute is lost without a refusal first.
- No plugin version directory that the running session loaded changes,
  apart from `.in_use` markers. No registration of such a plugin changes.
  No `.orphaned_at` marker appears. The record states how it treated the
  markers.
- The record names the Claude Code version.
- After the measurement, the staging HOME holds the hooks plugin installed
  for owner-staging-capabilities-k250.

## Notes

An unsolicited settings write or a changed loaded registration falsifies the
quiet-interval assumption. Then stop, and ask the owner to resolve the
writer before the cutover. An unchanged final file alone does not prove that
no intermediate write occurred.

The dry run of owner-switch-rehearsal-k255 tests the commands with the held
version in a disposable copy. It cannot measure the live parent harness.

## Decisions (running log)

- Claude Code is 2.1.322 on 2026-11-10, not the draft's 2.1.319. No hold
  exists before owner-marketplace-freeze-k253. The measurement uses the
  installed 2.1.322 and names it. A different held version needs a rerun.
- A scratch HOME showed that `claude plugin marketplace add` and `claude
  plugin install` drop a user extended attribute from `settings.json` and
  from both registries. The program refused lost metadata only for
  `settings.json`. `lost_metadata` now checks all three `HOOK_FILES`. Two
  fixture controls cover the registries, and a mutant that checks only
  `settings.json` fails both.
- Settings temporaries are `settings.json.tmp.<pid>.<hex>`, so their writer
  is attributable. Registry temporaries are `<file>.tmp.<hex8>` with no PID,
  and `known_marketplaces.json.lock` appears. Registry writes are attributed
  by the program's command windows and the live process set.
- The idle parent is an interactive Claude Code session in tmux that runs the
  switch through its own Bash tool, as finish does. The process inspection
  cannot admit it beside this operator session, so the staging run sets
  `OWNER_SWITCH_REHEARSAL_SCOPE`, which the program allows outside the real
  HOME. The rehearsal hook records the command windows.
- A staging prefix stands for `$(brew --prefix)`: a copy of the accepted 2.0
  build with an installation record, links to the ten sources, and
  `opt/grove` at the candidate keg. Stub `brew` and `mise` answer only the
  gates' metadata queries. Resolution, probes and Claude Code's commands are
  real.
- The staging HOME gets a native starting state before the switch: copies of
  the plan's superseded entries, and the five superseded plugins installed and
  enabled. The idle parent then loads plugins that the switch disables, as
  finish does.
- The baseline instrument classifies a dangling link as `dangling` by its
  form, so dereferenced placeholders were `UNCLASSIFIED`. Seeding makes each
  dangling entry a link to `H/.k251-absent/<text>`, which dangles inside H.
- The baseline instrument reads `~/.local/bin/codebase-memory-mcp` and
  `~/.quint-llm-kit` relative to HOME. Seeding clones both into H. The MCP
  entries keep their real absolute paths.
- `codebase-memory-mcp` is one daemon per account and refuses a client with
  another cache folder. The staging entry gains `CBM_CACHE_DIR` set to the
  daemon's `~/.cache/codebase-memory-mcp`, as finish's own parent shares it.
  `claude mcp list` then connected quint-kb, Playwright and
  codebase-memory-mcp. Context7 needs authentication without a credential.
- The auto-mode classifier refused a parent in `bypassPermissions`. The
  parent runs in the default permission mode, with one allow rule for
  `Bash(/bin/sh W/run-switch.sh)`.
- Claude Code 2.1.322 shows the folder trust prompt with "No, exit"
  selected. The run records trust for the parent's folder in the staging
  `.claude.json` before the launch. The staging settings select the vim
  editor mode, so readiness also accepts the "-- INSERT --" footer.
- The switch refused the first staging plan: its Ravel settings original was
  absent, and native-off had written the off value. The stored real draft
  has the same defect, so owner-switch-plan-k254 received a note. `prepare`
  now runs native-off before it drafts.

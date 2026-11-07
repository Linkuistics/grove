# owner-staging-hooks-k251

## Goal

Measure the hooks installation of step 3 in the staging HOME. A signed-in
Claude Code session with the owner's MCP servers runs beside it.

## Context

- The sections "The hooks installation inside finish" and "Claude Code's
  own writes" of `docs/specs/owner-skill-delivery.md` state the commands.
  They register `writing-hooks` from Grove's keg and install
  `technical-communication-hooks@writing-hooks`, each under a held link.
  The program then disables the plugin by its own exchange.
- The section "Test seams" requires this staging measurement. It digests
  each plugin version directory that a running session loaded, and each
  registration, before and after the commands. It watches `settings.json`
  during each command. Any replacement by another process counts against
  the evidence of the accepted case, and the owner decides the case again.
- In staging, Grove's keg is the candidate keg of owner-staging-setup-k248.
- Claude Code writes `.in_use/<pid>` into each version directory that a
  session loaded. The section "Re-inventory before the cutover" of
  `docs/owner-skill-delivery-evidence.md` records it.

## Done when

- A signed-in Claude Code session with the owner's MCP servers runs in the
  staging HOME during the measurement.
- The installation's commands run under held links, and the program
  disables the plugin. The measurement records each command's write of
  `settings.json` and any write by another process.
- No plugin version directory that the running session loaded changes,
  apart from `.in_use` markers. No registration of such a plugin changes.
  No `.orphaned_at` marker appears. The record states how it treated the
  markers.
- The record names the Claude Code version.
- After the measurement, the staging HOME holds the hooks plugin installed
  for owner-staging-capabilities-k250.

## Notes

If another process replaced `settings.json` during a command, stop and ask
the owner to decide the accepted case again.

The rehearsal reruns `settings-writes.sh` for the version that it uses.
This leaf measures the staging case only.

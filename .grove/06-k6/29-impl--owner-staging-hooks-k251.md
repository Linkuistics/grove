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

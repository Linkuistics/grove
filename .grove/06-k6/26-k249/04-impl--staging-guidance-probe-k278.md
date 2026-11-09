# staging-guidance-probe-k278

## Goal

Run the Claude Code trial that disables the plugin of its own guidance.
Then close this node's evidence and reconcile "Not measured".

## Context

- The node brief states the trial. "The final session and new launches"
  and "The switch inside finish" of `docs/specs/owner-skill-delivery.md`
  give its purpose: finish reads its guidance from absolute plugin-cache
  paths, and the switch disables that plugin.
- The staging HOME holds no superseded plugin. Install the guidance plugin
  only in a disposable copy, or remove it with Claude Code's own commands
  afterwards. The retained staging HOME must end with its original content.
- "Not measured" of `docs/owner-skill-delivery-evidence.md` lists open
  measurements.

## Done when

- A Claude Code session loads its guidance from a plugin and disables that
  plugin. It then launches a nested probe. After the probe, it reads its
  guidance again by absolute path.
- The evidence reconciles each item of "Not measured" that a discovery
  trial of this node measured.
- The evidence section of this node states each command that the operator
  reruns after a released source changes.

## Notes

A failure is a finding for the owner design or for a pack, not a reason to
weaken a case.

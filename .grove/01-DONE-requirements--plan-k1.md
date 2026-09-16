# plan-k1

## Goal

Establish the requirements for making Grove configuration easy to reuse, vary,
and switch, including experiments that differ only in small ways.

## Context

The root brief records the user's problem. The existing configuration contract
is described by `docs/CONFIGURATION.md`,
`docs/adr/complete-session-configuration.md`, and
`docs/adr/untracked-configuration-delta.md`.

## Done when

- The user and this session share an understanding of configuration reuse,
  variation, selection, and compatibility requirements.
- Acceptance scenarios and test seams are agreed and recorded.
- The brief gives the next session enough settled context to proceed.

## Notes

Requirements work includes checking the existing configuration and launch seams,
resolving the interdependent choices one question at a time, comparing concrete
approaches, and reviewing the resulting requirements with the user. Design and
implementation follow the Grove workflow once those requirements are settled.

Existing behavior checked against the code and documentation:

- `keyed_launch::Templates::load` reads the primary document and an optional
  overlay. An overlay replaces a whole template only for keys the primary
  document declares.
- `validate_node` in `crates/keyed-launch/src/templates.rs` rejects properties
  and child blocks and requires exactly one string argument per key.
- `SessionConfig::load_for_worktree` supplies the worktree and main repository
  roots for local configuration selection. The existing delta's search order
  and untrackedness contract are documented in the cited ADR.
- `crates/grove-loop/tests/session_config.rs` tests the public configuration
  adapter. `crates/grove/tests/lifecycle_cutover.rs` includes an acceptance test
  showing that the next session observes an edited configuration.

These are evidence about today's behavior, not restrictions on the requested
replacement. The current ADR explicitly rejects configuration reuse within a
command; this work reopens that decision because of the user's maintenance and
experimentation problem.

The user's existing personal configuration was read as an example without being
changed. It repeats the same Codex command with different reasoning-effort
values across producers, integrations, and other kinds; Claude entries repeat a
second command with a few effort changes. Reuse therefore needs to address both
many kinds sharing a command and commands differing in individual argument
values. Requirements should not hard-code today's models or vendors.

## Decisions (running log)

The user reports that switching between Codex-led and Claude Code-led processes
requires too much editing, that both scenarios cannot conveniently coexist in
the main configuration, and that most entries repeat the same content with only
a few changes. The work must make reuse and switching easy.

The user clarifies: "It's not only those two - I want to be able to easily try
different configurations that differ only in small ways." The requirement is
general configuration variation, not a fixed choice between two harnesses.

Variant selection is per worktree, with a global default. The user accepted this
so experimentation in one grove does not change other groves' selections.

Variants inherit later changes to shared settings unless they explicitly
override a value. They record their differences rather than freezing an
independent copy of the configuration from which they were derived.

The user requests "a concept of combinable profiles, that could then be selected
in the workspace-local .grove.kdl". Profiles are the user-facing concept for
reusable, composable configuration variations. Their selection belongs in the
existing workspace-local file; it is not limited to choosing one independent
variant. Composition and conflict rules are recorded in the decisions below.

Reusable commands have user-defined parameters. The user accepted this in
preference to built-in harness/model/effort settings. Grove must remain generic:
the command author controls how parameters become arguments, including for
other harnesses and wrappers.

Selected profiles compose in the listed order, with later profiles winning when
they set the same value. The user accepted this so a small experimental profile
can override an earlier shared setup without treating intentional overlap as an
error.

Profiles may include other profiles. The user accepted the example of a `daily`
profile including `base` and `codex-led`, with a workspace then selecting `daily`
and `high-effort`. Common combinations can therefore have reusable names.

Existing flat configurations must keep working unchanged. The user also asks for
a sample configuration using the new form and override examples in the Grove
configuration directory, `~/.config/grove/`. Deliver examples under separate
filenames so the existing active personal configuration is preserved.

A workspace's explicit profile list replaces the global default profile list.
It does not append implicitly. To extend a default named `daily`, the workspace
explicitly selects `daily` followed by its experiment profiles. The user agreed
so the workspace file shows the whole selected combination.

An unfinished, unselected profile with missing parameters or unresolved
references must not block a working configuration. The user agreed to semantic
validation of the selected combination, while the whole file must remain valid
KDL.

During review, the user queried the phrase "only the selected file is read"
because the worktree override must be resolved against the global configuration.
The clarification is explicit: `~/.config/grove/config.kdl` is always read;
only the two local candidates are alternatives. The worktree's `.grove.kdl`
wins over the main repository's `.grove.kdl`, and the chosen local file applies
over the global personal configuration rather than replacing it.

After that clarification, the user said, "Ah, in that case I approve." The root
brief's requirements, supporting behavior, acceptance scenarios, and proposed
test seams are approved. Those seams are configuration loading and expansion
through the public interface, and Grove acceptance tests with temporary jj
workspaces and fake executables. The next work is design of the grammar,
resolution interface, diagnostics, inspection, and example delivery against this
approved contract.

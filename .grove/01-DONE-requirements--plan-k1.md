# plan-k1

## Goal

Establish the behavior and scope of a standalone Taskfile command that generates
release notes through `grove run`, including any required kind skill and
configuration support.

## Context

- `Taskfile.yml` exposes complete patch, minor and major release workflows.
- `docs/RELEASING.md` describes automatic notes generation through the
  `release-notes` route when Unreleased is empty.

## Done when

The command's inputs, output, rerun behavior, configuration expectations and test
seams are settled with the human and recorded for implementation.

## Notes

Requirements, design and the small decomposition are captured in the root brief.
The existing generation path supplies the core behavior; no research or separate
design leaf is needed. Implementation and the real runner setup are separate
focused increments.

## Decisions (running log)

The human wants "A taskfile command", using "a standalone grove run", and notes
that this may require "a release-notes kind, and associated skill and
configuration entry". The work is a reusable release-notes command; the exact
supporting pieces remain to be checked against the existing implementation.

The human clarified: "It needs to be all the changes since the last release."
The input must describe the complete release interval, not only the latest jj
change. For the standalone command, the proposed endpoint is the current jj
change, including its unreleased ancestors; the full-release workflow continues
to use `main`. The baseline must be the last release in that endpoint's history.

The human selected "Refresh Unreleased (recommended)": each explicit
`task release:notes` run refreshes that section using both its existing notes and
all changes since the last release, incorporating newly added changes and
retaining useful existing wording. Versioned release notes remain untouched.

The human selected "Use these test seams (recommended)": real temporary jj
repositories and a fake LLM runner cover the full release range, refreshing
Unreleased, preserving history and jj state, and leaving the changelog unchanged
on failure; one live confined invocation verifies the actual configuration.

The bootstrap settles the design in the root brief and cuts two implementation
leaves: the shared release-notes command and skill, followed by the personal
headless route and live verification. The reusable command can be verified with
the deterministic runner fixture before the machine-specific setup is complete.

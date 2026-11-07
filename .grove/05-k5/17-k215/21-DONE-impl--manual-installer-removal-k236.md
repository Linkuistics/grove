# manual-installer-removal-k236

## Goal

Remove Grove's manual installer, with its `--pack` option and its tests.
Make the delivery wording of Grove's code comments and walkthrough books agree
with Homebrew installation and Ravel delivery.

## Context

- "Records" in the running log of `homebrew-delivery-design-k216`: the
  installer goes, and the `harnesses:` key stays as metadata that no route
  reads.
- `plugins/install.sh`, `plugins/install.test.sh` and the task
  `check:skills` in `Taskfile.yml`.
- "Resources and conformance" and "Packaging and rollout" of
  `docs/specs/codex-skill-provisioning.md`.
- `docs/specs/walkthrough-books.md` states how the source-exact books follow
  the code.
- The documented Pi command of `grove-hook-adapters-k230` replaces the
  installer's last use.

## Done when

- `plugins/install.sh` and `plugins/install.test.sh` are gone. No task,
  script or test runs them.
- `plugins/README.md`, `plugins/CONTEXT.md` and `docs/USAGE.md` keep no
  installer instruction. The statement on `harnesses:` says that no
  installer reads the key.
- Enumerate every comment and book passage that names the installer, the
  marketplace or native provisioning as a delivery route. Classify each one.
  Each passage now describes the current route, or it is history and says
  so. A sweep with a positive control and a cross-tree control shows the
  result.
- The books' own checks pass.
- `bash scripts/check.sh` passes.

## Notes

This leaf removes what retires. `homebrew-delivery-guidance-k237` writes the
installation guidance of the final state.

## Decisions (running log)

**Removal (2026-11-07).** `plugins/install.sh` and `plugins/install.test.sh`
are deleted. `scripts/check.sh` drops their shellcheck entries and the
"plugin install" step. `task check:skills` drops its installer step.

**The native-only lifecycle test (2026-11-07).** The test
`a_native_only_lifecycle_completes_with_explicitly_installed_skills` ran the
installer for Codex. Its seam, "Native-only bootstrap on 1.1", stays: Grove
still admits the native routes, and a session reads a native installation
that already exists. The test now links each core skill into
`~/.codex/skills` itself.

**Scope at the edge of k237 (2026-11-07).** README.md, `plugins/grove/README.md`,
`docs/ARCHITECTURE.md`, `CONTEXT-MAP.md` and `docs/RELEASING.md` also tell
users to run the installer. This leaf removes those instructions, because
they would name a deleted script. k237 still writes the final installation
guidance.

**The caveats and the formula test (2026-11-07).** The caveats lose the
installer commands. The formula test loses that expectation, and also the
expectation of `/plugin install grove@linkuistics`, which k235 removed from
the caveats but left in the test. The rest of the caveats' "Skills" text
stays for k237, whose task names it.

**The glossary (2026-11-07).** `plugins/CONTEXT.md` replaces "Symlink
install", "Skip-loudly default", "Link reconciliation" and "Workspace guard"
with one entry, "Manual installer" (retired). Each link to "Symlink install"
now names Ravel's delivery.

**Boundary with k237 (2026-11-07).** This leaf removes installer
instructions from guidance. Marketplace instructions in guidance stay for
k237, except in sentences that this leaf rewrites anyway. In comments and
books, this leaf corrects every route: installer, marketplace and native
provisioning.

**The sweep (2026-11-07).** The sweep classifies every comment and book line
that matches `install`, `marketplace`, `provision`, `native` or `--pack`, and
fails on a line without a class. Test names that say "provisioned" stay,
because the moves record cites them. Their comments now say "shipped". The
`codex-provisioning` anchor of the `overview` book stays, because its concept
index links to it. A scratch comment and a scratch book line that named the
installer as a route failed the sweep. The script and its review table are in
the session's scratchpad, and `docs/repository-moves-record.md`, "The
removed installer", records the method and the result.

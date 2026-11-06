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

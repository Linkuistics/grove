# bun-pin-drift-k188

## Goal

Make `bash scripts/check.sh` select the Bun version that
`scripts/ravel-policy-typecheck.sh` pins, so that the check does not depend on
the machine's current Bun.

## Context

`core-bare-references-k170` found the drift. Its full check failed one step of
21, "Ravel caller assets and provenance". The policy typecheck requires Bun
1.4.3 and refused Bun 1.4.4. The owner's `~/.config/mise/config.toml` selects
`"brew:oven-sh/bun/bun" = "latest"`, and Homebrew now supplies 1.4.4. Mise
still has Bun 1.4.3 installed. The repository has no `mise.toml`, so the pin
exists only inside the script.

With `$(mise where bun@1.4.3)/bin` first on `PATH`, that step passed, and the
other 20 steps passed in the full run. The defect is in the environment
contract, not in the code under check. It will fail the full check of each
later leaf and the finish session's integrated check.

## Done when

- The check selects the pinned Bun through the project's own configuration,
  or the pin moves to a version that the check obtains reproducibly. Follow
  the `machine-environment` skill and the repository's Taskfile conventions.
- One source states the Bun version. The script guard and any project tool
  configuration agree with it.
- `bash scripts/check.sh` passes in a shell where the global Bun is not the
  pinned version.

## Notes

Do not change the owner's global mise configuration for this leaf. A
repository pin does the job and leaves the owner's tools alone.

## Decisions (running log)

- The root `mise.toml` holds the one Bun pin, 1.4.3. mise's shims and hook
  read it in the repository. A plain `[tools]` file needs no `mise trust`.
- `scripts/pinned-bun.sh` reads that pin and runs that Bun. It uses `PATH`
  Bun when the version agrees, and otherwise asks mise for the pinned
  install. The compiler test runs its copy in a scratch directory outside the
  repository. There, the shims select the global Bun, so the helper cannot
  rely on `PATH`.
- The typecheck script, `ravel:policy:deps` and the release doctor call the
  helper. The doctor no longer reads the version from `@types/bun`.
  `@types/bun` stays a lockfile package pin, separate from the runtime pin.

# ravel-git-confinement-k73

## Goal

Prove that Git and mixed snapshots satisfy the read, confinement and lifetime
contract of local snapshots, on both real backends.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Read access and
  delegation" and the "Lifetime", "Nested sessions", "Harness runs" and "Real
  confinement" rows.
- The fixtures of `ravel-skill-lifetime-k58` and `ravel-skill-confinement-k59`.

## Done when

- Under `--confine`, a child and its descendants read Git and local resources
  outside the working directory. They cannot write them or read the protected
  configuration. The grants hold the published snapshots and the descriptor,
  not the staging, the acquisition storage or a source repository.
- A fake harness in a harness run reads the Git resources under `--confine`.
- Independent and detached readers keep Git resources after the parent exits,
  the repository goes and the settings change.
- A nested resolution with a Git source sees the current pins and returns only
  its own result. Under a confined parent, an unavailable transport, cache or
  settings refuses without escape.
- Writable overlap and protected aliases refuse as `skill_access_unavailable`
  with mixed sources, with no unconfined fallback.
- The fixtures pass on macOS and through `task test:native-linux`. A defect
  that a fixture finds is repaired in this leaf when it is small. A larger
  defect earns a leaf inserted before `ravel-git-install-k74`.

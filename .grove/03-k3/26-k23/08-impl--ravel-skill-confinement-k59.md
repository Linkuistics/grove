# ravel-skill-confinement-k59

## Goal

Grant confined sessions read-only access to the exact published descriptor and
snapshot trees on macOS `sandbox-exec` and Linux bubblewrap. Replace the
interim confined refusal of `ravel-skill-launch-k55` with these grants.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Read access and
  delegation".
- Ravel `docs/specs/session-execution.md`: "Confinement".
- Ravel book chapter 9, "Apply filesystem confinement".
- Ravel `task test:native-linux` and `task native-linux:image` for the Linux
  backend.

## Done when

- Under `--confine`, the managed process and each harness run read the
  descriptor directory and every snapshot tree, also outside the working
  directory. The grants are read-only. They exclude the staging area, the
  acquisition storage and the mutable sources. They never cover the whole home
  or the whole cache.
- Admitted relative links inside a snapshot traverse and read, with no write
  access to their targets.
- A grant that overlaps the configuration directory, the settings file or a
  known result channel refuses as `skill_access_unavailable`, also through a
  canonical alias. A writable grant that overlaps a snapshot or the descriptor
  refuses in the same way. That includes a working directory inside the cache.
- A grant that the backend cannot give refuses as `skill_access_unavailable`.
  No failure retries unconfined.
- The interim refusal and its fixture are gone. Confined launches with
  delivery disabled do not change.
- Real backend fixtures on macOS and on Linux cover the acceptance row "Real
  confinement". They also cover the confined part of "Harness runs" and the
  confined part of "Nested sessions". A confined child and its descendants read
  the resources, and they cannot write them or read the protected
  configuration.
- The book chapter on confinement, the session specification and
  `CHANGELOG.md` agree. The affected Ravel checks and the Linux image tests
  pass. The bookmark advances by jj, and nobody pushes it.

## Notes

The public `--runtime-read` flag keeps its meaning of one regular file. The
skill grants are internal. Record each platform limit that a test meets, with
the backend version.

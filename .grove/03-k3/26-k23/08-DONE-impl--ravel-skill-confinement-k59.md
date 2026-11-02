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

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing, and Ravel `main` is still
  `24a545236185`. The line holds k58's `4b0f54c575a7`, so no rebase was
  necessary at entry.
- Runner seam. `keyed_launch::FilesystemGrants` gains `read_trees`: whole
  directories, read-only. Seatbelt allows `file-read*` and
  `file-map-executable` on a `subpath`. Bubblewrap adds `--ro-bind`. The
  runner refuses a tree that a writable root holds or that holds one, so the
  default deny alone keeps a tree read-only. An explicit `deny file-write*`
  had no control that could fail, and it went.
- The grant set is the descriptor directory and each snapshot `tree`, from
  `Delivery::read_trees`. The snapshot object's `manifest`, the staging area,
  the cache root and the mutable sources are not granted. `Basis` carries the
  set, so the managed process and each harness run share it.
- Protected overlap. A tree that overlaps the configuration directory, the
  settings file or the inherited `RAVEL_EXIT_FILE` of an enclosing session
  refuses. Each path is canonical first, which covers aliases. The
  `--exit-dir` and the private run directory are writable grants, so the
  writable check covers them.
- Writable overlap. A writable grant that overlaps the cache root refuses,
  not only one that overlaps a granted object. A working directory in the
  staging area could otherwise change a publication in progress. The spec
  sentence widens to match.
- Backend grant. With the probe, Ravel lists each tree with `ls -a` inside a
  sandbox that grants only the trees. A failure refuses. A public launch
  cannot reach a missing or unlistable tree, because its resolution checks
  the same objects first (a descriptor directory with mode 0100 gives
  `skill_cache_corrupt`). A unit test of `Confined::prepare_for` covers it.
- Every refusal of this leaf is `skill_access_unavailable`, stage `skills`,
  exit 3, input `--confine`, with no unconfined fallback. The interim
  refusal, its `confine` parameter and its fixture are gone.
- macOS observation (Darwin 27.0.0, `/usr/bin/sandbox-exec`): `chmod` skips
  a mode that does not change, so a mode check needs a file without a write
  bit. A nested `start-session` in a confined parent refuses as
  `settings_invalid`, exit 2, because Seatbelt shows the metadata of the
  settings file and refuses its read.
- Firing controls, each alone, restored from a copy, with the digest of the
  Rust sources the same before and after. M1 and M2: no tree grant for the
  managed process or the harness run. M3: a writable Seatbelt tree grant. M4:
  no protected check. M5: no writable check. M6: no probe. M7: grant the
  whole cache. M8: the runner admits an overlapping tree. Each failed its
  fixture. M4 and M5 fail at the first case of their fixtures, so the later
  cases have no control of their own.
- Linux observations (arm64 container, kernel 7.0.14-linuxkit, bubblewrap
  0.11.1). Bubblewrap makes the parent directories of each mount in an empty
  root, so `ls` of the home succeeds and shows only the path to the cache.
  The listing probes of the fixture therefore look for a real entry. Dash
  ends the shell when a redirection of the builtin `:` fails, so the runner
  fixture makes that probe in a subshell.
- Nested launch in a confined parent. On Linux the namespace holds no
  settings file, so a nested launch sees delivery disabled. With the default
  entry it refuses as `config_missing`, exit 3. With a readable `--config` it
  runs, with `skills` null. On macOS both refuse as `settings_invalid`. The
  spec sentence that promised a refusal for unavailable settings now states
  each backend. Ravel cannot tell a hidden file from an absent one, and a
  rule on the inherited variable would break k58's disabled-delivery case.
  The nested session stays inside the parent's sandbox, so no grant widens.
- Documentation. Chapter 5 adds the grant row and the refusals. Chapter 6
  replaces the interim note, the variable row and the failure row. Chapter
  9 adds the tree grant, the bubblewrap parent directories and the runner
  test row. The session and skill-discovery specs and the changelog state the
  same rules. The strict STE check passes on chapters 5, 6 and 9.
  `breaks.sh --strict` reports two runs over 450 words in chapter 6. The
  parent has both runs, and this leaf added 12 words to the second.
- Seatbelt metadata check, by hand with the leaf's tree rule. A confined
  shell could not hard-link a tree file into its writable directory (`ln`:
  operation not permitted). `xattr -w`, `touch -t`, `chflags uchg`,
  `chmod` and `mv` of a tree file each failed, and its times, flags and
  attributes stayed the same. On Linux a link across the bind mount fails
  with `EXDEV`. These checks are not fixtures.
- Review. The leaf changes a security boundary that the compiler cannot
  check, so it earns `review-impl`. The leaf inserts it before
  `ravel-local-handoff-k61`, beside the other stage reviews.
- Repair of k53. Linux clippy failed on an unused `Path` import in
  `crates/ravel/tests/rhai_request.rs`, which only macOS code uses. This leaf
  did not touch that file. The import is now macOS-only, so the Linux check
  of this stage passes.
- **Output (2026-11-02).** The Ravel commit is `e343587583eb` (change
  `otowrruz`) on bookmark `skill-directory-delivery`, with parent
  `4b0f54c575a7`. The bookmark is not pushed. On macOS, one script on a tree
  whose digest was the same before and after ran `check`, `test:cli`,
  `docs:book:build`, `docs:examples:test`, `docs:excerpts:test`,
  `docs:figures:test`, `docs:ste:test`, `docs:breaks:test`,
  `book:integrity` and `smoke`, and all passed. `check` ran 715 tests: 715
  passed and 48 were skipped. `task test:native-linux PLATFORM=arm64` then
  ran on the same tree. Every step passed except `rust-clippy`, which failed
  on the k53 import. Its `check` and `test` ran 709 tests each: all passed
  and 48 were skipped, with unchanged subjects. After the import repair,
  clippy passed on macOS and in the arm64 container, and `rhai_request`
  passed in the container. The two full runs came before that repair. No
  amd64 run occurred.

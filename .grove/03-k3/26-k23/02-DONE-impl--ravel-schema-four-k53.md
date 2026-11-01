# ravel-schema-four-k53

## Goal

Move the Ravel feature line to script and request schema 4 with skill delivery
disabled. The result is the native-only 2.0 contract: every request carries
`originalPrompt`, `prompt` and `skills: ()`, and the evaluate frame has its
derived bound and a checked send.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Request and
  configuration script" and "Schemas and identities".
- Ravel `docs/specs/configuration-evaluation.md`: the request table, "Frame
  bounds" and "Requirement: Request schema 4".
- Ravel `docs/skill-discovery-design-evidence.md`: the source observations for
  `rhai_request`, `request_map`, `load_context`, `FRONT_FRAME_BYTES`,
  `read_frame` and `llm.rhai`.
- Grove `docs/specs/coordinated-release.md`: every schema-3 example, fixture
  and book page moves to 4.

## Done when

- At entry, the session fetches Ravel and rebases `skill-directory-delivery`
  onto the current `main`. It reconciles upstream changes, reruns the affected
  checks and records the exact base in the running log.
- `SCHEMA_VERSION` is 4 in the worker. A script that exports 3 refuses as
  `unsupported_version`, exit 3. The remedy names both changes: export 4, and
  read `request.originalPrompt` where the script analyses the task.
- The CLI half (`rhai_request`) and the worker half (`request_map`) both carry
  `originalPrompt`, `prompt` and `skills`. With delivery disabled, `prompt`
  equals `originalPrompt` byte for byte, and `skills` is `()`. The JSON routing
  notice carries `skills: null`. The text notice does not change.
- The frame bound is 38,541,312 bytes. The CLI compares the encoded evaluate
  frame with a limit that it receives as an argument, before it sends the frame.
  A larger frame refuses as `malformed_input`, stage `cli`, exit 2. The refusal
  names the encoded size and the largest members. The CLI then stops and reaps
  the worker. The
  three frame-bound tests of the specification pass where their platform admits
  them.
- The session establishes on which release platforms test 3 can run. It
  records each platform where it cannot, in the Ravel evidence document.
- Public fixtures cover three acceptance rows for the disabled case:
  - "Disabled delivery": empty, newline and Unicode prompts stay exact.
  - "Request and evaluation": `load_context` and `select` receive one request.
    A change to the copy in `load_context` does not reach `select`. Top-level
    code sees no request.
  - "Version transition": schema 3 refuses without sources. A mismatched CLI
    and worker pair refuses.
- Every example, fixture and book page that exports 3 moves to 4. `llm.rhai`
  reads `request.originalPrompt` for its classifier. The smoke configuration
  scripts move too. `CHANGELOG.md` records the schema change under the
  unreleased entry.
- `task check`, `task test:cli`, `task docs:examples:test`,
  `task docs:excerpts:test` and the other book tasks that the change touches
  pass. The bookmark advances by jj, and nobody pushes it.

## Notes

The repeated-member refusal of the settings file is a 2.0 rule for every key.
It lands with the `skills` settings object in `ravel-skills-resolve-k54`,
because the duplicate detection belongs in the same parser change.

`RAVEL_SKILL_DIRECTORIES`, the `skills` settings key and the stage `skills` do
not exist yet. Do not add a settings key or a failure stage without a reader.

## Decisions (running log)

- Entry base. `jj git fetch` in `Ravel.skill-directory-delivery` reported no
  change on 2026-11-02. Local `main` and `main@origin` are both
  `24a545236185b0910ba6117fe8f9e8b2e2c8a748`. The bookmark
  `skill-directory-delivery` is `822c0ed2f67d9a446b74476fdad7d05922552b87`,
  already on that `main`. No rebase and no reconciliation was necessary, so no
  check had to run again for upstream changes.
- The worker accepts `skills` as null or an object, the documented type of
  the field. The CLI always sends null at this stage. No settings key and no
  failure stage were added.
- The frame check `evaluate_fits` lives in `rhai_worker::serve` and runs on
  the encoded evaluate frame, after the entry read and before the send. A
  refusal returns `Conversation::Refused`, so `worker.rs` stops and reaps the
  worker as for an unreadable entry.
- The refusal names the three largest members and also the three members that
  Ravel does not bound. In the first long-kind run, the kind of 3,600,002
  bytes was not among the three largest members. A list of the largest members
  alone would therefore not name the cause.
- A mismatched pair at the public seam: a copy of the binary reports another
  build identity, and macOS needs a new signature for it. The copy replaces
  the CLI file while the CLI waits on a FIFO prompt. The CLI resolves the worker path after it reads the inputs.
  The fixture runs on macOS only, because Linux launches `/proc/self/exe`.
- Firing control for test 3: with the limit replaced by `usize::MAX`, the same
  request failed as `worker_failed`, exit 5, and the test failed. The source
  was restored from a copy.
- Frame-bound platforms, measured on 2026-11-02. macOS admits one argument of
  1,039,252 bytes, so test 3 runs there with a kind of 600,000 bytes. arm64
  GNU/Linux admits 131,071 bytes in one argument, so test 3 cannot run there.
  x86-64 GNU/Linux was not measured, because `rustc` crashed under QEMU in
  the amd64 image. Its row is a derivation from the 32-page argument limit.
- The working directory is not bounded by the platform at these sizes:
  `current_dir` returned 600,218 bytes on macOS and 600,073 bytes on arm64
  Linux. This contradicted the specification sentence "The platform also
  bounds the working directory". The Frame bounds paragraph of
  `configuration-evaluation.md` now states the measured fact.
- A variant of test 3 uses a working directory 2,000 levels deep and runs on
  each release platform. It passed on macOS and in `ravel-native-linux:arm64`.
  With the check removed it failed on macOS, so it detects that removal.
- The measurements are in the new section "Frame-bound platforms" of
  `docs/skill-discovery-design-evidence.md`.
- The two book fragments of the chapter-5 notice carried the digest of the
  schema-3 script (`813135…`, checked by hashing the schema-3 text). They
  now carry the digest of a fresh run with this build (`9d4526…`) and the
  `skills` member. The reference fragment of the request is a fresh run too.
- A wall-clock control of `rhai_limits` failed once in a full run and passed
  five times alone. The three frame tests encode frames of about 38 MB. The
  probable cause is their load beside that control, but no run proved it. They
  now take the whole run's slots through `.config/nextest.toml`, as the
  project's two deadline-sensitive groups already do. The next full check
  passed.
- Output. Ravel commit `eac3c98731d0d92002f8e2f938f41e569d9becce` on the
  bookmark `skill-directory-delivery`, with parent `822c0ed2`. Nobody pushed
  it. The build of this commit reports `ravel 1.1.0`.
- Verification at that commit, on macOS: `task check` passed, with 622 tests
  passed and 48 skipped. `task test:cli`, `task docs:book:build`,
  `task docs:examples` (22 examples), `task docs:examples:test`,
  `task docs:excerpts`, `task docs:excerpts:test`, `task docs:ste:test`,
  `task docs:breaks:test`, `task docs:figures:test`, `task book:integrity`,
  `task smoke` and `task rhai:upgrade-proof` passed. `task setup` ran first.
  The strict STE and prose-break checks passed on the three changed chapters.
- Not run: `task release:formula`, which needs Homebrew release preparation,
  although the formula template moved to schema 4. The emulated x86-64 Linux
  suite did not run. No test observes the reap of the worker after a frame
  refusal at the public seam. The code path is the one of an unreadable
  entry, `Conversation::Refused`, which `worker.rs` stops and reaps.

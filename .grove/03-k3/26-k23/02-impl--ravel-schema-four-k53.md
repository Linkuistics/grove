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

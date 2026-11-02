# ravel-skill-launch-k55

## Goal

Make `start-session` resolve the configured local sources and prepare the
prompt automatically, before confinement and before the Rhai worker launches. A
schema-4 script that passes `request.prompt` to its harness then delivers the
directories with no helper and no change to its policy.

## Context

- Ravel `docs/specs/skill-discovery.md` at `822c0ed2`: "Ownership and order",
  "The prepared prompt", "Evaluation order and reads", "Diagnostics",
  "`RAVEL_SKILL_DIRECTORIES`" and "Caller migration".
- Ravel `docs/specs/configuration-evaluation.md`: "Harness runs" and the
  reserved `RAVEL_*` class of the environment grants.
- Ravel `docs/adr/skill-preparation-precedes-evaluation.md`.
- The resolver of `ravel-skills-resolve-k54`.

## Done when

- `start-session` runs the resolver after it reads the settings, the inputs and
  the entry, and before confinement and the worker. A skill refusal creates no
  result channel and launches no worker or managed process.
- With delivery enabled, `prompt` is the preamble, the label line `Caller task`
  and one nonce-delimited block. The nonce has 32 lowercase hexadecimal digits
  from the operating system. Ravel chooses again when the caller bytes contain
  it. The caller bytes appear once and unchanged. Preparation checks the
  recorded byte range. A prepared prompt over 1 MiB refuses as
  `skill_limit_exceeded` and reports both prompt byte counts.
- Both request halves carry the prepared `prompt` and the `skills` map with
  `descriptorPath`, `sources`, `directories` and `mandatory`, and no offsets.
- The JSON routing notice carries the `skills` object. It holds
  `descriptorPath`, `resolutionMs` and the source records with disposition
  `captured`. It also holds the byte count and SHA-256 of each prompt. No
  diagnostic shows a prompt, a skill body
  or an environment value.
- Ravel sets `RAVEL_SKILL_DIRECTORIES` to the descriptor path in the managed
  process and in each harness run, and replaces a stale value. With delivery
  disabled, it removes the name. The worker never receives it. A `configEnv`
  grant of the name refuses as `excluded_grant`. `ravel::env` of the name
  throws `env_not_granted`.
- A launch checks its actual `--config` directory against the origin record of
  each snapshot that it would reuse, as `skill_access_unavailable`.
- Until `ravel-skill-confinement-k59`, a `--confine` launch with delivery
  enabled refuses before the worker as `skill_access_unavailable`, stage
  `skills`, exit 3. The remedy says that confined grants are unfinished. A
  fixture proves the refusal. Confined launches with delivery disabled do not
  change.
- Public fixtures at the `start-session` seam cover the launch parts of these
  acceptance rows. The first rows are "Directory delivery", "Filesystem versus
  skill semantics" and "Settings and bootstrap". Next come "Relative
  resources", "Request and evaluation", "Host reads" and "Standalone
  acquisition". The unconfined part of "Harness runs" is also covered. So is
  the empty-task case of "Bounds and frames". "Version transition" adds schema
  3 with sources and schema 4 with sources.
- A fake managed program reads the delivered prompt, the descriptor and
  sibling and parent-relative resources. It sends its opaque result through its
  own channel. A fixture agent reports missing or ambiguous guidance, and Ravel
  chooses no skill.
- One installed example builds a read grant from `request.skills`. The book
  chapters on configuration scripts, sessions and the session walkthrough
  describe the route, with exact source excerpts. `CHANGELOG.md` records it.
  The affected Ravel checks pass. The bookmark advances by jj, and nobody
  pushes it.

## Notes

The interim confined refusal is a running-log decision of
`ravel-local-delivery-k24`. It uses an existing contract code and must not
reach a release. The confinement leaf removes it and its fixture.

## Handoff from ravel-skills-resolve-k54

Ravel commit `777e609f` adds a temporary refusal, `skill_launch_unsupported`,
in `crates/ravel/src/choice.rs` (`refuse_enabled_delivery`). It keeps
`start-session` from silently dropping guidance while delivery is enabled.
Remove it, its fixture
`start_session_refuses_enabled_delivery_until_the_launch_stage` in
`crates/ravel/tests/skills_resolve.rs` and its note in the book reference.

## Decisions (running log)

- Entry base. `jj git fetch` changed nothing, and Ravel `main` is still
  `24a545236185`. The line holds k54's `777e609f` above k53's `eac3c987`, so
  no rebase was necessary at entry.
- The launch protects one more path than `skills resolve`: the canonical
  directory of the selected configuration entry. That is the `--config`
  directory, or `$HOME/.config/ravel` for the personal entry. A local
  snapshot's origin record is its canonical source directory, so the check of
  the origin against that directory is the same check as for a fresh capture.
- The confined interim refusal comes before acquisition, after the settings,
  inputs and entry. A launch that cannot proceed then writes no cache object.
- The prepared prompt is the preamble, a blank line, the line `Caller task`,
  the line `BEGIN RAVEL CALLER TASK <nonce>`, the caller bytes, a newline and
  the line `END RAVEL CALLER TASK <nonce>` with its final newline. Ravel
  chooses the nonce again until it occurs exactly twice in the whole prompt.
  That rule covers the caller bytes and also the paths of the preamble.
- A failure of the operating system's random source refuses as
  `random_unavailable`, stage `skills`, exit 5. The worker already uses that
  code for `ravel::uuid_v4`.
- `skills::launch` owns the launch half: `prepare`, the wrapper, the nonce,
  the 1 MiB check of the prepared prompt and the notice member. The resolver
  gains `resolve_protecting`, and `skills resolve` keeps `resolve` with no
  extra path. `choice::prepare` takes the `--confine` flag, so the interim
  refusal sits beside acquisition. `RunSetup` carries the descriptor path to
  the harness runs. `run::skill_environment` and `run::scrubbed` give the
  managed process and each harness run the same rule.
- The worker needs no change. Its environment is already empty, and
  `RAVEL_SKILL_DIRECTORIES` is already in the excluded class `RAVEL_*`. The
  fixtures prove `excluded_grant` by flag and by setting, and
  `env_not_granted` with a stale caller value present.
- The caller-byte range check is an assertion. A failure would be a defect of
  the wrapper, and no input can reach it.
- `skill_limit_exceeded` for the prepared prompt reports the bound
  `preparedPromptBytes`, fixed at 1,048,576, and both byte counts in the
  message. The remaining bounds stay with `ravel-skill-bounds-k57`.
- The example `harness-table.rhai` (version `harness-table 4`) gives the lead
  `--add-dir` for each snapshot root, and the reviewer nothing, because it
  runs with full access. With delivery disabled its arguments do not change,
  so the frozen observations still pass.
- The `start-session` help no longer says that the prompt routes unchanged.
  The `--prompt` help says that `select` receives it as `originalPrompt`.
- Public fixtures are `crates/ravel/tests/skills_launch.rs`, 15 tests, plus
  four unit tests in `skills::launch`. The settings grammar refuses a control
  character in a source path, so the hostile-path case uses a quote and a
  root named like a delimiter line.
- Firing controls, each run alone and restored from a copy: removal of the
  managed grant, of the disabled-delivery scrub, of the harness-run grant, of
  the prepared prompt in the request, of the configuration-directory
  protection, of the confined refusal, of the nonce-uniqueness rule and of the
  prompt bound. Each made its fixture fail.
- Not covered by an executed fixture: that the worker process environment
  lacks the name. The worker launch clears its environment and grants
  nothing, and the existing worker tests own that rule.
- No review earned. Each claim that the compiler cannot check here has a
  public fixture with a firing control. The resolver beneath this stage
  already has its review, `ravel-skills-resolve-k62`, before the handoff.
- Output. Ravel commit `d870d0fc99dc5746bf338980f35fa7e3690f97d5` on the
  bookmark `skill-directory-delivery`, with parent `777e609f`. Nobody pushed
  it. The build reports `ravel 1.1.0`.
- Verification on macOS, one script on a frozen tree whose digest was the same
  before and after the run: `task check` passed with 666 tests passed and 48
  skipped. `task test:cli`, `task docs:book:build`, `task docs:examples:test`,
  `task docs:excerpts:test`, `task docs:figures:test`, `task docs:ste:test`,
  `task docs:breaks:test`, `task book:integrity` and `task smoke` passed.
  `task docs:excerpts` and `task docs:examples` passed after the run. The
  strict STE check passed on chapters 4, 5, 6 and 8.
- Not run: Linux. Real confined grants, publication races, the remaining
  bounds, lifetime and recovery are later leaves of this node.

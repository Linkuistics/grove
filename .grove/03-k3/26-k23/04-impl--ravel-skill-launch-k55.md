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

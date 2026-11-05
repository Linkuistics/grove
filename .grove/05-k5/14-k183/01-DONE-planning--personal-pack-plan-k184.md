# personal-pack-plan-k184

## Goal

Cut the implementation leaves of personal-pack-k183.

## Done when

- Each Done when of the parent brief belongs to a leaf with a concrete,
  verifiable outcome.
- Each handoff leaves Grove's `bash scripts/check.sh` green.
- The guardrail plugin trial and the removal of the `linkuistics` plugin each
  have their own leaf.

## Decisions (running log)

- The node is one move with three leaves: the expand, the native trial and
  the contraction. Each step leaves the product working. Grove keeps its
  `linkuistics` plugin and its marketplace entry until the contraction, and
  the owner's native plugins stay unchanged.
- One expand commit holds both plugins. The `guardrail` plugin has three
  files, so a second expand commit, as in writing-craft-pack-k181, adds a
  handoff and no working increment.
- The trial comes before the removal, as in k181. A repair before the
  removal changes only the pack. A repair after it needs another Grove pin
  change.
- The ordinary plugin declares the `guardrail` plugin as a dependency. A
  native user of `linkuistics@linkuistics` gets `guardrail` today, and the
  dependency keeps it for a native user of the ordinary plugin. This is the
  pattern of the writing plugin and its hooks plugin. k212 can reverse it
  with a recorded reason.
- The `guardrail` plugin root must hold `skills/guardrail/`. Its frontmatter
  hook names `${CLAUDE_PLUGIN_ROOT}/skills/guardrail/scripts/`, so that
  layout keeps the hook's bytes.
- Planning finding: Grove's `scripts/check.sh` runs neither
  `guardrail-hook.test.sh` nor ShellCheck on the guardrail scripts. The
  pack's `task check` owns both checks from its first commit.
- Planning finding: no Grove test reads the bytes of a moving skill. The
  rows of `plugin_fallback.rs` and the assertion of `commit_guidance.rs` read
  Grove's own files. The pack therefore needs no `GROVE_COMMIT`, and no last
  pack commit has to record the removal commit.
- Planning finding: `check-packs.sh` checks only the sources that the
  marketplace pins. The directory-only report for `personal` therefore
  belongs to the removal, which changes the pin.
- Planning finding: the harness trial cases name `plugins/linkuistics` as a
  source. They stay as evidence at their recorded commits, as in the design
  and engineering removals.
- The parent's Done when maps to the leaves as follows. The source
  directory, the root, the manifest, the `guardrail` plugin and the
  dependency belong to k212. The skill bytes, `disable-model-invocation`,
  the pack checks, the installer proof and the public commit also belong to
  k212. The command and hook trial
  belongs to k213. The removal, the marketplace, the consumers,
  `check:packs`, the machine-settings statement and the full record belong
  to k214.
- The node brief asks for a trial of the command and the hook. The
  membership decision gives the command to the staging trials. k213
  follows the stricter node brief.
- No in-session reviewer and no `review-planning` leaf. The subtree follows
  the pattern of five finished moves, and the membership specification had
  its review. The trial leaf answers the one open native question with an
  observation.

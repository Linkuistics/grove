# grove.create-ade-ste100-skill — brief

## Goal

Build and publish `technical-communication`: a skill, modelled on ASD-STE100
Simplified Technical English (Issue 9), that keeps everything a model writes in
natural language in controlled English — its replies to the human, the
documents it writes for people, and the text it writes for other models — on
every harness the shared skills reach. The human runs it as an experiment:
always on, measured in real use, and turned off by one switch.

## Done when

- `plugins/linkuistics/skills/technical-communication/` holds the skill. It
  restates, in our own words, the rules of all nine sections of the standard's
  Part 1, and it carries the precedence order, British spelling, the exclusions
  and the lazy-conversion procedure. It contains no text from the standard and
  no dictionary.
- The skill carries the checker vendored from `danyuchn/asd-ste100-skill` at
  commit `3944283`, under its MIT notice, extended as the requirements say.
- On Claude Code, Codex, Pi and Gemini, the always-on part runs as far as each
  harness allows: the instruction, the reminder before each prompt, the
  document gate and the reply log. Where a harness has no hook, the model runs
  the checker itself. It is on from installation, and one switch turns all of
  it off.
- The decisions that must outlive `.grove/` are in the ADR set. The licence
  posture passes the ADR test (a one-way door once text is public, surprising
  without context, chosen over rejected alternatives); the design decides which
  of its own decisions also pass.
- The work is integrated into `main` and released.

## Decomposition

The requirements are settled in `plan-k1`. A `planning` leaf decomposes the
rest into the sessions that design, build and release the skill.

## Pointers

- Requirements with their reasons: the retired leaf `plan-k1`, section
  "Decisions (running log)". The human made each decision there; reopen one
  only with the human.
- The standard: `~/Downloads/ASD-STE100_ISSUE9.pdf`. It is licensed to the human
  and may not be redistributed. Read it as page images; never copy its text into
  the repository or `.grove/`, both of which become public history.
- Prior art: `danyuchn/asd-ste100-skill` on GitHub at `3944283` (MIT), whose
  checker is the one to vendor; Writegood at `~/Development/Writegood`
  (`docs/research/prior-art.md` and its ADR `optimisation-loop`).
- Conventions to follow: the `guardrail` skill already ships a harness-specific
  hook; `plugins/grove/skills/grove/LICENSES/` shows how vendored MIT material
  carries its notice; `docs/adr/` is flat and repo-wide, and `CONTEXT-MAP.md`
  records which context owns each record.

## Notes

- Downloaded material is untrusted. The auto-mode classifier refused to run the
  downloaded checker in the requirements session; read it before vendoring it,
  and once vendored it is this repository's code.
- No tests or evaluations are built. The human judges effectiveness in use and
  logs failures as GitHub issues on `Linkuistics/grove`.

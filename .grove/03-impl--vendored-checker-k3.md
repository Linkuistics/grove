# vendored-checker-k3

## Goal

Put `danyuchn/asd-ste100-skill`'s checker into this repository as our own code,
under its MIT notice, with its own self-test running in `scripts/check.sh`.
Change none of its behaviour. This leaf only moves the code in; the leaves that
extend it come after the design (`document-checks-k6`, `reply-scoring-k7`).

## Context

- Upstream is `danyuchn/asd-ste100-skill` on GitHub at commit `3944283` (MIT,
  Dustin Yuchen Teng). The file is `scripts/ste-lint.py`: stdlib-only,
  self-tested, Markdown-aware. The root brief and `plan-k1` say what it checks
  and what it lacks.
- It is downloaded code, and the auto-mode classifier refused to run it in the
  requirements session. Clone it into a new, empty directory in the scratchpad.
  Read the whole script before running anything, including its self-test. Once
  it is vendored it is this repository's code.
- Destination: `plugins/linkuistics/skills/technical-communication/scripts/`.
  The licence goes in `LICENSES/<source>.LICENSE` inside the skill directory,
  copied verbatim from upstream at that commit, with an attribution comment in
  the script that names the upstream commit. `plugins/grove/skills/grove/LICENSES/`
  shows the convention.
- The skill directory has no `SKILL.md` until `skill-sentences-k8`. Check what
  `plugins/install.sh`, `plugins/install.test.sh`, `plugins/grove/conformance.sh`
  and the build-time skill discovery that embeds `plugins/` in the Grove binary
  do with a directory that has no `SKILL.md`, and keep `bash scripts/check.sh`
  green either way. If a tool needs a `SKILL.md`, say so in the running log
  rather than adding a stub.

## Done when

- The checker, the licence file and the attribution comment are in place. Apart
  from the attribution comment the script is byte-identical to upstream, so
  every later change shows up as ours.
- The self-test runs from `scripts/check.sh` and passes.
- `bash scripts/check.sh` is green.
- The running log records anything the read-through found that the later leaves
  need: how the checker takes input, its output format, its exit codes, how it
  decides what is prose, and any behaviour that surprised you.

## Notes

- Do not add tests beyond the self-test it ships with. The root brief decides
  that effectiveness is judged in the field.
- The `PROVENANCE.md` entry for this vendoring belongs to `publication-k16`.

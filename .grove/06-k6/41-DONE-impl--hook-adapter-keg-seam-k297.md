# hook-adapter-keg-seam-k297

## Goal

Repeat the hook-adapter seam on revised formula output, in isolated
Claude Code, Codex and Pi homes. Repeat guardrail's capability trial from
the revised `personal` keg, with a no-op control that must fail.

## Context

- The row "Hook adapters" of "Test seams" and the section "Native-only hook
  acceptance" of `docs/specs/codex-skill-provisioning.md` own the
  observations. "Skill formulae" of `docs/specs/owner-skill-delivery.md`
  owns packaging-only acceptance of native-only plugins.
- The evidence sections "The hook adapters from a keg" and "The hook
  plugins across a keg change" of `docs/homebrew-skill-delivery-evidence.md`
  used real source directories. They establish the earlier layout only.
  `docs/evaluations/hook-adapters/keg-registration.sh` is their instrument.
- The section "Guardrail plugin trial" of `docs/repository-moves-record.md`
  records guardrail's earlier capability trial and its limits.
- grove-keg-projection-k293 supplies Grove's revised formula output from
  the working copy. tap-skill-keg-projection-k295 supplies the revised
  `personal` formula in the tap working copy.

## Done when

- Each harness runs the documented commands against an `opt` path of the
  revised kegs, in a HOME of its own. No real HOME, registration or keg
  changes.
- Claude Code runs the writing hooks in place through the registered `opt`
  path, and no native skill appears. Codex accepts the local source at the
  private payload and copies the complete plugin into its cache. Pi reaches
  its extension through the public alias.
- An actual upgrade installs a second revised keg and cleans up the first.
  Claude Code and Pi then read the new bytes. For Codex, the evidence records
  the registration failure and the cached hook behaviour. It also records
  the repair by the documented commands and renewed interactive trust of
  the changed bytes.
- The complete `guardrail` payload of the revised keg equals its accepted
  subject in files, modes, links and hook declarations. Claude Code
  registers it from the revised keg, and its capability trial passes.
- A no-op `guardrail` hook that succeeds fails the trial's refusal
  assertion. Watch that control fail before crediting the pass.
- The evidence document records each harness version, isolation, subject
  digest and observation. It marks the earlier readings as evidence of the
  earlier layout only.

## Notes

The upgrade case needs two revised kegs of one formula. Two fixture
revisions of the same output, with one changed hook byte, are enough.

These trials spend real harness sessions, so no gate runs them. Keep the
instrument rerunnable and name its rerun command in the evidence.

If a harness refuses the private path, stop and report the refusal. The
root outcome "Hooks and plugin capabilities remain usable where required"
then needs a design decision, not a workaround.

## Decisions (running log)

- A new instrument, `docs/evaluations/hook-adapters/projected-keg.sh`,
  measures the revised layout. The earlier `keg-registration.sh` and its
  output stay unchanged, because the earlier evidence section cites them.
- The Grove subject is the native archive `5f6093a6e6bb…` in
  `target/archives`. The shared-directory seam measured that archive, and
  its marketplace files name the private writing payload. The instrument
  compares them with `assets/keg-root`.
- The disposable `grove` formula takes the `SKILL_SOURCES` constant and the
  install block of the template, because the projected install block reads
  that constant.
- The `personal` subject is the revised formula of the tap working copy,
  installed verbatim by the private Homebrew. Its accepted `guardrail`
  subject is the owner's keg `personal` 2026-11-06, read only. The staging
  capability trials loaded `guardrail` from that keg.
- The capability trial repeats the two turns of the earlier trial in a
  signed-in scratch HOME. The no-op control runs first, from a scratch copy
  of the revised keg whose hook script only exits 0. One verdict function
  judges both sessions.
- The first full run passed every reading, and its control failed as
  required. Three readings of that instrument were defective: the owner
  files kept the real home path, the slash-command check read the stream
  instead of the session file, and each init event printed twice. The
  corrected instrument ran again from an empty scratch directory. Only the
  second run is evidence.

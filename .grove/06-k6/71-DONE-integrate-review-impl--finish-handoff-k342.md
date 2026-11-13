# finish-handoff-k342

**Integrates:** finish-handoff-k341


## Goal

Triage the findings of finish-handoff-k341 against the current artifact and
its owning contracts. Apply each finding that is valid and actionable.



## Context

- Review finish-handoff-k341 reads producer finish-handoff-k324, commit
  `b85b9815`. Read its Findings and Evidence and limits in the review's
  committed task file.
- The reviewed artifact is `docs/finish-handoff.md` and "The finish
  extension" of `.grove/_BRIEF.md`.
- The review's Context names the owning release and owner-delivery rules
  and the command sources inspected. Read those rules when settling each
  disposition.

## Done when

- Each review finding has a recorded disposition grounded in the current
  source and contract. Rejected findings state the reason.
- Each accepted correction reaches every affected part of the handoff and
  its root-brief extension. Their commands, order, pass conditions and
  recovery agree with the owning rules.
- Verification appropriate to the accepted changes is recorded. Distinguish
  source inspection from executed checks and from future finish operations.

## Notes

The review ran no test, build, lint, format, release, promotion or switch
command. Its findings are review input, not an instruction to perform the
finish sequence. Preserve finish's one confirmation, helper-owned teardown
and original ending command.

## Dispositions

Each finding is graded against producer commit `b85b9815` and current source.

- **F1, accepted as a real issue.** `docs/RELEASING.md:854` runs a plain
  `brew upgrade grove`. Homebrew's `cleanup.rb:348–365` and `426–446` skip
  cleanup only under `HOMEBREW_NO_INSTALL_CLEANUP` or a per-formula
  exemption. `~/.homebrew/brew.env` exempts only `ravel`. Handoff step 4 now
  requires the variable on each Homebrew mutation that finish runs outside
  the task. The recovery row for a partial release names the exact
  protected commands. The root brief's step 3 states the same rule.
- **F2, accepted as a real issue.** `docs/specs/owner-skill-delivery.md:730`
  requires a pause of the settings writers before `native-off`.
  `~/.config/mise/config.toml` tracks `~/.config/ravel` through
  `[dotfiles]`, so mise synchronisation is a concrete writer. Handoff step 7
  now asks for that narrow pause before promotion. The step 8 recovery row
  keeps the pause. The root brief's step 6 states it. The `native-off`
  section of the prepared instructions states it for the owner's recovery
  after a crash.
- **F3, accepted as a real issue.** Step 3 runs before the cut, on the
  integrated source. `Taskfile.yml:964–969` runs `scripts/release-smoke.sh
  --archives target/dist` after the cut. `ravel_target_inputs.py` records
  the 2.0 lane for `aarch64-apple-darwin`. Ravel's archive manifest holds no
  installation record, so that lane reads no default directory. Step 6 now
  maps `native-2.0-delivery-disabled` to that run, names its pass lines and
  separates it from a Homebrew installation with the off value. The first
  item of Limits states the same distinction.

## Verification

- Source inspection: each cited line above was read in this session.
- Executed: `task owner:switch-plan:prepare` after the earlier
  `INSTRUCTIONS.md` moved to `~/.local/state/owner-switch/superseded/k342`.
  It wrote one file and kept the plan digest `8679483c…`. The new SHA-256
  `433f2e2d…` is in the handoff's inputs and the evidence record.
- Executed: the technical-communication checker reports 0 hard findings in
  the handoff, the prepared instructions and the root brief. The evidence
  record keeps its 14 earlier hard findings, none in the added lines. A
  link and anchor check of the handoff and the instructions found 0 broken
  links.
- Not run: `bash scripts/check.sh`. The change touches documentation only,
  and its book check covers only `docs/walkthroughs/`.
- Future finish operations: none of the corrected steps was executed.

## Close of discovery-rollout-k6

This leaf was the last live leaf. The node's Done when holds for ordinary
work. Its activation, final integration and release items belong to finish
through the root brief's finish extension and `docs/finish-handoff.md`. No
further promotion is needed, and no review names the node.

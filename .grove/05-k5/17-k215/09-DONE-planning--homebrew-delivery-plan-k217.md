# homebrew-delivery-plan-k217

## Goal

Cut the implementation leaves of `homebrew-skill-delivery-k215` from the
settled design.

## Context

- The parent brief states the outcomes.
- `homebrew-delivery-design-k216`, `ravel-default-skill-directory-k218` and
  any review chain that they earned settle the design.
- `native-settings-recovery-k221` and any review chain that it earns settle
  the native settings of the owner design. `hooks-installation-writes-k222`
  settles the writes of the hooks installation, and
  `native-settings-recovery-k224` integrated the review of both. The running
  log of `homebrew-delivery-design-k220` lists shipped guidance that changes
  with the implementation.

## Done when

- Each outcome in the parent brief's "Done when" belongs to a leaf with a
  concrete, checkable result.
- The leaves cover each of these subjects:
  - Grove's release archive and formula.
  - Grove's record of pack commits, and `check:packs`.
  - The removal of the marketplace's pack entries and of the installer's
    skill links.
  - The formulae of personal, superpowers and quint-llm-kit, and
    TestAnyware's formula.
  - The marketplace files and the documented commands of the hook adapters.
  - The README changes in the six packs and in TestAnyware, and Grove's
    guidance.
  - Ravel's default skill directory, in Ravel's `skill-directory-delivery`
    line before the 2.0.0 cut.
  - The candidate keg for the staging trials, and the removal of
    `task owner:checkouts`.
  - The retired listing of Grove's marketplace, its seam and the release
    notes.
  - The removal of WritingSkills' root Codex marketplace file, and the wording
    of `authoring-conventions`.
  - The removal of the manual installer, and the delivery wording of Grove's
    code comments and walkthrough books.
- Each leaf leaves Grove's `bash scripts/check.sh` green. Each pack change
  passes the checks of its own pack.

## Notes

A change to a pack's README is a new pack commit on its public remote. Grove's
record then names the commit that Grove's pack check accepts.

`discovery-rollout-k6` owns the owner's cutover. Put a leaf there only when
the design gives a step to rollout.

## Decisions (running log)

**The increments (2026-11-06).** The design gives five chains of dependency.
Each step of a chain leaves its repository working.

- Ravel's default skill directory comes first. Grove's checks of the
  default and the installed tests of the keg need a 2.0 build that reads an
  installation record.
- Grove's record of pack commits comes before the keg sources. The release
  copies each pack from the checkout of `check:packs`.
- The keg sources come before the hook adapters. The adapters' marketplace
  files name paths inside `share/ravel/skills/writing`.
- The pack commits come after the hook adapters. WritingSkills removes its
  root Codex marketplace file only after Grove's keg holds the replacement.
  The personal formula pins the PersonalSkills commit that changes
  `authoring-conventions`.
- The retirement of the native routes comes after every formula exists. The
  installer goes after the documented Pi command replaces its last use.

**Separate leaves for Ravel (2026-11-06).** Ravel's code and its formula
channels are two increments. The code works with any record, and its tests
write the record. The formula channels and the book follow it.

**Separate leaves for the formulae (2026-11-06).** The personal formula,
the two third-party formulae and TestAnyware's release are independent. Each
has its own repository or upstream and its own checks.

**Guidance last (2026-11-06).** Grove's guidance describes the final state,
so one leaf rewrites it after the retirement and the installer removal. Those
leaves only remove what they retire. The guidance edits `plugins/grove`, so the
candidate keg comes after it.

**Outcomes that rollout owns (2026-11-06).** The parent brief names two seams
and one outcome that run in the owner's cutover. The staging trials read
kegs, and the hooks installation inside finish changes no loaded plugin. The
switch program installs the writing hooks plugin in its step 3. The owner
design assigns these to `owner-switch-program-k91` and the staging leaves of
`owner-ravel-skill-inventory-k38`.

This node supplies the candidate keg that they read. The rollout brief's handoff now names the amendments of k218,
k221, k222 and k224 for those leaves.

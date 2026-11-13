# finish-handoff-k324

## Goal

Write the durable handoff to the driver-created finish session. Extend the
root charter with the finish steps that the designs assign to that session.

## Context

- The parent brief, section "Owner-delivery handoff", subsection "The finish
  handoff": each item there is an obligation of this document.
- `docs/specs/coordinated-release.md`, steps 6 and 7 and "Recovery
  boundaries". `docs/specs/owner-skill-delivery.md`, "The native settings
  inside finish", "The hooks installation inside finish", "The switch inside
  finish" and "Resuming the switch".
- `docs/owner-cutover/owner-checklist.md` and
  `docs/owner-cutover/prepared-instructions.md`.
- `docs/owner-skill-delivery-evidence.md`, "Owner switch rehearsal" and
  "Owner switch plan redraft". owner-switch-plan-redraft-k340 replaced the
  plan, the comparison instrument, the instructions and the dry-run result.
  Name the digests of the redraft section, not the earlier ones. Name step
  15 of the owner's checklist, the plan report before each attempt.
- The receipts and records of every earlier leaf of this node.
- `CLAUDE.md`, "Finish, integrate, and release", and `docs/RELEASING.md`.

## Done when

- A durable document in Grove's `docs/` names the whole finish sequence in
  order. The sequence is finish-commit, main integration, the integrated check,
  `task release:minor` and installed verification of both admitted Ravel
  routes. It continues with the readiness receipt and `release:promote
  VERSION=2.0.0`, then `native-off`. Then come the installation of Grove's
  revised formula and resolution of all ten default IDs after promotion and
  any link-conflict recovery. The freeze check, `claude --version`,
  `claude doctor` and the dry run against the real prefix follow, then the
  switch, the probes and the receipt. `grove-llm record-teardown` and the
  original ending command come last.
- The document names the required Ravel version 2.0.0, the published archive
  digests, the `ravel@2` prefix and the decision of
  promotion-upgrade-rehearsal-k318.
- It names the owner's checklist with its pause and resume instructions. It
  names the held Claude Code version, the frozen marketplaces and the release
  of the hold after the switch.
- It names the switch program, its probes, the native probe and the hooks
  installation inside the switch. It names the comparison command, the
  attempt note and `restore --backup`. It states the drift refusal of a later
  restore and the append-only manual.
- It names the rows of the recovery table and the Grove patch route after a
  failed core trial. It names the purge report before purge. It lists the
  staging HOME and the candidate keg as removable after the receipt.
- It names where finish records the promotion and switch receipts for the
  paused workstreams.
- It states that Ravel's cut never repeats. A partial release resumes and
  never bumps again. The original 1.1.0 keg stays through the final signal.
- An enumeration of the document and of the prepared instructions finds no
  instruction about an inverse, a merge, a pending record or a resumed run.
- The root charter names this document and each finish step that the designs
  assign. Finish keeps its one confirmation, its teardown helper and the
  exact original ending command.

## Notes

Ravel `docs/specs/release-channels.md`, sections "Promotion interface" and
"Consumer readiness receipt", give the exact command line, the receipt
shape, the receipt command and the example receipt
`docs/specs/release-channels-readiness.example.json`.

A request to plan is not the finish confirmation. The finish session proposes
the concrete whole sequence and obtains its one confirmation.

This document governs irreversible steps. If an adversarial read is earned,
cut `review-impl` before you retire. Its integration then runs before finish.

promotion-controls-decision-k336 decided that finish promotes. Ravel's
`release-promote.sh` checks only that the tap's consumer formula carries the
receipt's `consumer.version`. A receipt that names an incompatible Grove
therefore passes. The document must state that finish writes the receipt
only after the installed verification of the released Grove's three routes.
`docs/promotion-upgrade-rehearsal-record.md`, section "Decision", gives the
evidence and the second condition, promotion-upgrade-remedy-docs-k338.

cross-repo-docs-reconciliation-k319 found that the switch's manual entry
covers activation only. After a successful switch, these statements of the
manual `AI Dev Tooling` no longer hold:

- the rows "Claude-specific", "Codex-specific" and "Shared Agent Skills" of
  "System map"
- the first four items of "Shared skills"
- the sentence about `skills/` in "Configuration ownership"
- steps 4 and 5 of "Bootstrap and recovery".

Step 5 names
Grove's Codex provisioning and `plugins/install.sh`, which the integrated
`main` no longer holds. The manual is append-only. The document must tell
finish to append one dated correction for these statements after the
receipt, by the manual's own "Synchronise the record" workflow. The workflow
also covers the maintenance checker's skill rows, which still classify
native links. `docs/cross-repo-docs-reconciliation-record.md`, section
"The maintained manual", lists each statement.

paused-grove-handoffs-k320 wrote `docs/paused-grove-handoffs.md`. Its section
"Promotion and switch receipts" is the place where finish records those
receipts for the paused workstreams. Each row states "Not yet recorded" until
finish writes it. The handoff names that section and tells finish to fill
each row. The paused workstreams resume only after finish ends.

## Decisions (running log)

- The handoff is `docs/finish-handoff.md`. It orders the steps and links
  each rule to its owning section. It restates no rule as a second owner.
- The receipt's three routes map to checks that exist. The integrated
  check runs in both lanes, 1.1 and the published `ravel@2`. The release
  task's installed checks run `brew test grove` and the installed smoke in
  both lanes. No installed check runs the native 2.0 route with delivery
  disabled. The 2.0 lane of the integrated check covers that route on the
  tagged source, and `native-off` with the promoted Ravel covers its
  resolution later. The document states that limit.
- The resolution of all ten default IDs uses the promoted Ravel in a new
  empty HOME under `env -i`. The settings of the real HOME hold the off
  value after `native-off`, so a resolution there reports `disabled`. This
  session ran the same command with `ravel@2` and saw the four installed
  sources resolve.
- Finish's own dry run uses the command of the evidence section "The
  command", with the real prefix. It has no `--path` to a stand-in. Its
  `--prefix` values name the canonical released Grove keg and promoted
  Ravel keg.
- After the switch receipt, finish fills "Promotion and switch receipts" of
  `docs/paused-grove-handoffs.md` and commits it to `main` through jj. That
  change touches no packaged file, so no release follows it.
- Finish runs `HOMEBREW_NO_INSTALL_CLEANUP=1 task release:minor`. The
  task's `brew upgrade grove` sets no suppression. Homebrew's
  `install_formula_clean!` would then remove the Grove 22.8.0 keg, and the
  running driver executes `/opt/homebrew/Cellar/grove/22.8.0/bin/grove`.
  `cleanup.rb` at the owner's Homebrew returns early from both the install
  cleanup and `periodic_clean!` when the variable is set. "Activation and
  running readers" of the coordinated release design requires suppression
  for every mutation, Grove's task included. `HOMEBREW_NO_CLEANUP_FORMULAE`
  names only `ravel`. Gate 4 of the switch accepts a retained earlier keg.
- `native-off` is step 8, directly after the promotion of step 7. The
  read-only verification of the pair follows it, because the native settings
  section requires `native-off` immediately after promotion.
- The enumeration covered every distinct word of the handoff and of the
  prepared instructions. No instruction names an inverse, a merge, a pending
  record or a resumed run. The handoff's section "Verification of this
  document" records each class of occurrence.
- An adversarial read is earned. The document governs the release, the
  promotion and the owner switch, all after `finish-commit`, where no leaf
  can carry a repair. Several of its commands ran in no earlier leaf, and
  this leaf found one defect that the earlier records missed. This leaf cuts
  `review-impl` with the stem `finish-handoff` and spends no in-session
  reviewer.

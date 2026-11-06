# homebrew-delivery-design-k219

**Reviews:** homebrew-delivery-design-k216, ravel-default-skill-directory-k218

## Goal

Read the Homebrew delivery design adversarially before
`homebrew-delivery-plan-k217` cuts implementation leaves from it. The design
is the amendments of both producers together, in Grove and in Ravel. Produce
findings, and no fixes.

## Context

- Grove: the commit of `homebrew-delivery-design-k216` ("Design Homebrew skill
  delivery") and the commit of `ravel-default-skill-directory-k218`. Each
  commit message names its handle. The running logs of both task files hold
  the decisions and the human's words.
- Ravel: commit `78d833d24364` on the bookmark `skill-directory-delivery` of
  the workspace `~/Development/AgenticOrchestration/Ravel.skill-directory-delivery`.
  It amends `docs/specs/skill-discovery.md`, adds the decision record
  `installed-skill-sources-are-the-default` and amends the glossary.
- The amended Grove documents: `docs/specs/owner-skill-delivery.md`,
  `docs/specs/codex-skill-provisioning.md`, `docs/specs/coordinated-release.md`,
  `docs/specs/external-pack-contract.md`, `docs/specs/pack-membership.md`, the
  decision record `homebrew-installs-skills-and-ravel-delivers-them` and
  `docs/homebrew-skill-delivery-evidence.md`.
- The owner design runs its switch against the real HOME inside finish. Its
  recovery rests on `docs/models/owner-switch.qnt`, and
  `task model:owner-switch` runs it.

## Done when

- Each finding names the document and section, the defect and its
  consequence. A finding states whether it concerns k216's amendments, k218's
  or both.
- If a finding is worth acting on, an `integrate-review-design` leaf is cut
  as the review's last act. Its body names this review's handle.

## Notes

The producers' specific doubts. Read each against the artifacts rather than
taking it as established:

- **The native settings.** k218 made the off value the earlier value of the
  `skills` cell. Finish writes it by a guarded step after promotion. The
  producer claims the five properties of the recovery model still hold
  without a model change, because the model's cells are abstract. Look for a
  state that the state table misreads. The guarded step, an interruption or
  an outside write can reach it. Look also at the rule that a classification
  never accepts another earlier value for that cell.
- **The upgrade interval.** Homebrew unlinks a formula's earlier keg before it
  installs the new one. Without `sources`, a launch in that interval delivers
  the other sources and refuses nothing. Judge whether accepting that silent
  absence is right, and whether every Grove and owner rule that assumed a
  refusal was amended.
- **The default belongs to the installation.** A separate HOME no longer
  isolates the keg sources. Find any check, trial, rehearsal or acceptance
  case that expects no keg source. Such a case runs a 2.0 build with a record
  in a HOME without `"sources": []`.
- **The installation record.** Ravel finds it two levels above its canonical
  executable. Check the keg-only `ravel@2`, Linux, a copied build, and the
  claim that no release archive holds a record.
- **Replacement and roots.** Explicit `sources` replace the default, and every
  default source has the roots `skills` and `craft`. Check each of the ten
  sources and the quint formula's changed source directory.
- **k216's amendments.** Read the keg layout, the identity of a keg source and
  the candidate keg. Read the hooks installation inside finish and the pack
  record. Read the retired marketplace and the Codex registration after each
  upgrade.
- **One owner per rule.** Grove's documents must cite Ravel's contract and
  restate none of it. Look for a Grove sentence that restates a Ravel rule,
  and for a Ravel rule that a Grove document contradicts.

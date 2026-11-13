# finish-handoff-k341

**Reviews:** finish-handoff-k324

## Goal

Read the finish handoff adversarially before the finish session uses it.
Find each step whose command, order or pass condition is wrong. A wrong step
fails, harms the owner's machine or reports a success without evidence.
Produce findings, not changes.

## Context

- The subject is `docs/finish-handoff.md` and the section "The finish
  extension" of the root brief `.grove/_BRIEF.md`.
- The producer's task file `finish-handoff-k324` holds its Done when and its
  running decision log.
- The owners of the rules: `docs/specs/coordinated-release.md`, steps 6 and
  7 and "Recovery boundaries", and `docs/specs/owner-skill-delivery.md`,
  from "The final session and new launches" to "Purge".
- `docs/adr/finish-promotes-after-the-grove-release.md`, `CLAUDE.md`
  "Finish, integrate, and release", and `docs/RELEASING.md`.
- `docs/owner-cutover/owner-checklist.md` and
  `docs/owner-cutover/prepared-instructions.md`.
- Ravel's `docs/specs/release-channels.md`, "Promotion interface" and
  "Consumer readiness receipt", in
  `~/Development/AgenticOrchestration/Ravel`.

## Done when

- Each finding names the step, a concrete state that shows the defect, and
  the rule, command or source line that it breaks.
- The findings cover each doubt in Notes. Each doubt needs a reading of the
  command's source or help, not only of the document's prose.

## Notes

Every step after `finish-commit` runs with no `.grove/`, so no leaf can
carry a correction there. A defect found now costs one integration. The same
defect found inside finish costs a force-stop and a manual recovery.

The doubts to test:

- **The cleanup suppression.** The producer added
  `HOMEBREW_NO_INSTALL_CLEANUP=1` to `task release:minor`, because the
  task's `brew upgrade grove` would otherwise remove the running driver's
  Grove 22.8.0 keg. Verify that reading against
  `/opt/homebrew/Library/Homebrew/cleanup.rb`. Find any other Homebrew
  mutation in the sequence that runs without suppression. Find any step of
  the release task that the variable changes.
- **The receipt's routes.** Step 6 maps `native-1.1`,
  `native-2.0-delivery-disabled` and `directory-2.0` to the runs of steps 3
  and 5. Read `scripts/grove-installed-smoke.sh` and the release task. Find
  each route that the named run does not in fact exercise. The 2.0 lane of
  the installed smoke reads the real installation record, so decide which
  route its lifecycle runs.
- **The dry run against the real prefix.** Step 14 drops `--path` and gives
  `--prefix` the canonical kegs. Read `scripts/owner_switch_dry_run.py`.
  The comparison of step 15 names the `opt` links as its prefixes. Decide
  whether that run and that comparison agree row by row. Decide whether the run can pass gates 4
  to 6 against the real prefix, and whether its copied layout is complete.
- **The default-directory resolution.** Step 10 resolves in a new empty HOME
  under `env -i`. Decide whether that resolution reads the same shared
  directory that a default launch reads. Decide whether the jq output lets
  the operator verify each `originPath` against its owning keg.
- **The order.** Compare the order of steps 6 to 19 with step 7 of the
  coordinated release design. Compare it also with the switch's nine steps.
  The owner's checklist establishes the quiet interval at step 12, after the
  Homebrew mutations. Decide whether `native-off` at step 8 needs any part of the
  checklist first.
- **A differing digest.** Step 10 stops in native delivery when a digest
  differs. Gate 4 of `scripts/owner_switch.py` reruns the staging trials of
  a differing Grove source by itself. Decide which behaviour the design
  requires and whether the document states it.
- **Recovery.** Compare the recovery table with "Recovery boundaries" row by
  row. Find a failure in finish that maps to no row, and a row whose action
  the document changed.
- **The ending.** Verify that the document keeps the one confirmation and
  the teardown helper. Verify that `grove-llm record-teardown` comes before
  the original ending command. Verify the force-stop command for an
  incomplete finish. Verify that no step removes the Ravel 1.1.0 keg or the
  Grove 22.8.0 keg before the final signal.

## Decisions (running log)

- The review reads producer commit `b85b9815` and the current source. It
  runs no test, build, lint, format, release, promotion or switch command.
  The findings remain here for the paired integration to triage.
- The normal release command suppresses cleanup correctly. Its recovery
  instructions do not carry that suppression to the resumed installation.
- `native-off` needs a pause of writers of its settings file before it
  runs. The full switch checklist may come later, but the handoff omits
  this narrower prerequisite.
- The native 2.0 evidence needs correction. The integrated check precedes
  the release cut. The release task also exercises the released archives,
  which the handoff's route mapping and limitation omit.
- Current source inspection confirms the three findings and the dispositions
  of the remaining doubts. They warrant a paired integration before finish.
  That task will triage the findings rather than inherit them as obligations.

## Findings

### F1 — P1: release recovery loses cleanup protection for the running driver

**Location:** `docs/finish-handoff.md:406`, step 4's recovery row; also
steps 4 and 5 at lines 107 and 130.

Suppose the release publishes its tag, archives and tap commit, then stops
before upgrading the owner's Grove. The recovery row sends finish to
`docs/RELEASING.md`. Its installation commands at lines 852–855 run plain
`brew upgrade grove`. The assignment at handoff line 107 applies only to
the original `task release:minor` process and its children. It does not
remain set for a later recovery command. The persistent protection named
in the handoff covers `ravel`, not `grove`.

At `/opt/homebrew/Library/Homebrew/cleanup.rb:348–365`,
`install_cleanup_formulae` and `install_formula_clean!` suppress the old
Grove keg's cleanup only when the cleanup setting or that formula's
exemption applies. Lines 426–446 give periodic cleanup the same setting.
The resumed upgrade can therefore remove
`/opt/homebrew/Cellar/grove/22.8.0` while the driver still runs from it.
This breaks `docs/specs/coordinated-release.md:169–180`, which requires
suppression for every Homebrew mutation and preservation of active
readers. Protection must cover recovery mutations as well as the original
task invocation. Ravel's promotion installer does export the setting at
`Ravel/scripts/release-install.sh:123`; that route is not the gap.

### F2 — P1: the settings writer pause comes after `native-off`

**Location:** `docs/finish-handoff.md:171–183`, step 8, and line 236,
step 12; `.grove/_BRIEF.md:114–118` carries the same ordering omission.

The handoff first asks the owner to pause writers at step 12. Step 8 has
already replaced `~/.config/ravel/settings.json`. A settings writer can
still run then. For example, a configuration synchronisation can publish
an earlier settings file immediately after `native-off` returns, removing
the off value and enabling directory delivery beside native delivery.
With an existing settings file, a concurrent write between the last
comparison and replacement can also be overwritten.

`docs/specs/owner-skill-delivery.md:717–730` permits `native-off` before
the completed switch checklist but explicitly requires the owner to pause
writers of this one file. The program does not establish that pause:
`scripts/owner_switch.py:1711–1727` checks other Ravel drivers, then
publishes the settings. Its comparisons at lines 1749–1780 can detect
some drift, but do not exclude writers after the comparison or after the
command. The handoff needs that narrow prerequisite before promotion's
immediate `native-off`, while preserving the later full switch checklist.
The backup-independent recovery instructions have the same prerequisite.

### F3 — P2: the readiness receipt cites the wrong artifact for native 2.0 coverage

**Location:** `docs/finish-handoff.md:153–157`, step 6; the first item of
"Limits" at lines 467–470 repeats the claim.

After the documented sequence passes, the receipt says that the released
Grove's `native-2.0-delivery-disabled` route was verified. The explanation
attributes that verification to step 3 "on the tagged source". Step 3
runs before step 4 cuts the release. `scripts/check.sh:138–139` runs Cargo
tests and doctests against the working source; it does not run the new
release binaries. Its isolated Ravel support at
`testing/ravel.rs:357–395` copies the 2.0 executable without its
installation record. That evidence is useful, but is not the claimed
verification of the tagged installed Grove.

The post-installation 2.0 lane of step 5 also does not supply the missing
disabled setting: `scripts/grove-installed-smoke.sh:172–196` creates a
fresh HOME with a Rhai configuration and no settings, then uses the real
`ravel@2` executable and its installation record. Its lifecycle therefore
uses the default directory. The private resolution at lines 114–149
checks the six sources separately.

There is relevant released-artifact evidence that the handoff omits.
`Taskfile.yml:964–969` builds and smokes `target/dist` after the cut and
before publication. `scripts/release-smoke-target.sh:75–86` extracts the
admitted Ravel 2.0 archive and runs the installed Grove cases against it.
Ravel's `scripts/release-common.sh:33–43` enumerates that archive's payload
without a Homebrew installation record, so this lane exercises
the released binaries without default directory delivery. The route
mapping should identify the actual released-artifact run and distinguish
that archive layout from a Homebrew installation with an explicit off
value. The blanket statement that no installed test covers the native
2.0 route obscures this evidence. The promotion decision requires the
receipt to follow installed verification of the released Grove's routes
(`docs/promotion-upgrade-rehearsal-record.md:713–721`); citing the pre-cut
suite does not establish that condition.

## Evidence and limits

The reviewed artifact is producer commit `b85b9815`, with no intervening
implementation change. This review uses inspection and the producer's
recorded results. It makes no fresh test or live installation claim.

| Mandated doubt | Source inspection and disposition |
| --- | --- |
| Cleanup suppression | Read the release task, Homebrew's installed `cleanup.rb`, and Ravel's promotion installer. The ordinary Grove task and Ravel promotion suppress cleanup. The setting changes the release's Homebrew cleanup behaviour; the archive runner does not invoke Homebrew. F1 concerns recovery commands outside that task's environment. |
| Receipt routes | Read the installed smoke, formula template, release task, archive runner and isolated Ravel test support. The real 2.0 keg's lifecycle reads its installation record and default directory. The private six-source resolution is a separate observation. F3 distinguishes these from the archive and pre-cut source runs. |
| Real-prefix dry run | `parse_prefixes` at `scripts/owner_switch_dry_run.py:588–598` canonicalises both the run's prefixes and the comparison's `opt` links. They agree when the links stay on those kegs. `copy_list` at lines 614–641 copies the prepared folder, plan paths, inputs, local helpers, staging paths and explicit includes. The includes retain the previously measured layout. Gates 4–6 at `scripts/owner_switch.py:2107–2239` can admit the real prefix once the new Grove is linked, Ravel is promoted, the formula versions agree and the ten digests match. No additional defect was established by this read. This review did not execute that future run. |
| Default-directory resolution | Read Ravel's `crates/ravel/src/skills/installed.rs:54–90` and `skills/mod.rs:815–825`. The executable's canonical prefix selects the installation record independently of HOME. The empty HOME bypasses the owner's off value while retaining the promoted installation's shared directory. The jq projection preserves each source's ID, digest and origin path; it permits the specified owning-keg comparison. |
| Order | Compared release steps 6–7 with the switch's nine steps and its checklist. Promotion remains after compatible Grove verification, hooks remain inside the backup, and source acceptance precedes mutation. F2 is the missing narrow writer pause before the full checklist. |
| Differing digest | Read `gate_4`, `gate_6` and the owner's source-acceptance rule. Gate 4 can rerun and admit changed Grove sources by itself, but handoff step 10 stops earlier and requires staging, revised acceptance and a new dry run. That earlier stop preserves the design's requirement; this review does not count gate 4's automatic admission as revised acceptance. |
| Recovery | Compared each post-promotion boundary with the table and read the promotion retry and switch/restore code. A freeze failure at step 13 has no dedicated row; the table's explicit fallback at handoff lines 399–401 requires force-stop when recovery cannot complete. The core-trial row places patch repair with the owner after an unsuccessful finish, rather than completing it in the current session. The owner design's "Resuming the switch" permits later recovery and no successful finish is claimed. F1 identifies the unsafe concrete recovery command. |
| Ending | The handoff preserves the one confirmation, helper-owned teardown and original executable. Steps 23–24 place `record-teardown` before the inherited ending command, and the recovery preamble retains force-stop for incomplete work. Explicit old-keg cleanup is after the last driver's reap. F1 is the automatic cleanup path that breaks the intended preservation. No ending command was exercised as a test. |

Graph evidence used Tier 2 verification for project
`Users-antony-Development-AgenticOrchestration-grove.gh-issue-25`, generation
`2026-11-13T08:46:00Z`. Exact evidence paths returned matching coverage
metadata with no recorded issue. This remains a best-effort coverage
signal. The Ravel default-workspace graph is generation
`2026-11-01T07:59:38Z`; its scripts and docs are excluded and its relevant
source paths are untracked or changed in that generation. The Ravel claims
therefore use current, directly read source and documentation rather than
negative graph results. Homebrew's installed source was also read directly.

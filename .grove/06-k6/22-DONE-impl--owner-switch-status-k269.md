# owner-switch-status-k269

## Goal

Move the owner's `mac-check` switch area and the purge report to the
simplified switch. Report delivery health and restore eligibility
separately.

## Context

- The section "Current health and restore eligibility" of
  `docs/specs/owner-skill-delivery.md` owns the status rules. "Purge" owns
  the report. "Test seams" lists the `mac-check` and purge controls.
- owner-switch-restore-k268 supplies the receipt, its delivery assertions
  and its whole-file restore identities.
- `mac-check` runs `~/.local/lib/machine-config/check.py`. Its switch area
  now runs `<state>/prepared/owner_switch.py state --json` and reports six
  states and flagged copies. owner-switch-entries-k92 records that change.
  The machine configuration contract owns the file.
- The fixture suite runs the real `mac-check` with HOME set to each
  disposable HOME.

## Done when

- The program has a read-only status entry for `mac-check`. It evaluates the
  delivery assertions, the current inventory and the sources for health. It
  compares whole files for restore eligibility.
- `mac-check` reports native delivery, a verified switched layout, or a
  layout needing restore or review. Without prepared evidence it reports
  that the switch is not prepared. It names differing files when restore is
  ineligible.
- A receipt alone never shows current success. A partial layout or stale
  evidence requires review. Formula drift keeps its separate area.
- The read-only purge report compares retained identities. It lists the
  conditions that only the owner can establish. A changed retained resource
  or an unverified switch blocks purge.
- Fixture controls cover each case of "Test seams". An unrelated model key
  or hook edit after success keeps health accurate and makes restore
  ineligible.
- The changed `mac-check` follows the machine configuration contract. The
  evidence document records its controls and one mutant that each control
  detects.

## Notes

The old `state` entry stays until owner-switch-contraction-k270 removes it.
The real HOME has no prepared plan yet, so the real `mac-check` reports that
the switch is not prepared.

## Decisions (running log)

- The new read-only entries are `status --plan PLAN [--json]` and
  `purge-report --plan PLAN`. The old `state` and `purge-check` stay with
  their controls until owner-switch-contraction-k270 removes them.
  `mac-check` moves to `status`, so the nine old `mac-check-*` controls
  are replaced. They tested the old states and the flagged copies.
- `status` reads the serial records: attempt notes, published backups,
  valid and invalid receipts and restored records. The latest switch
  attempt with records governs. Health is one of four values:
  - `native`: the plan's native assertions and `inventory.before` hold.
  - `switched`: a valid receipt exists, its plan is the prepared plan,
    and the plan's assertions, `inventory.after`, the receipt's reader
    evidence and the current sources hold.
  - `restore`: a published backup has no receipt and no restored record,
    or a recovery backup waits for its retry.
  - `review`: anything else, such as a partial layout or stale evidence.
- Restore eligibility is a separate report. With a valid receipt, each
  restore destination is compared with its post-switch identity, and
  the differing paths are named. A restore that waits names its command.
- A restored attempt whose manual lacks its entries reports the pending
  entry beside native health.
- The plan gains `switch.native_assertions`, because native health needs
  assertions of its own. An assertion may give `values`, a list of
  accepted values, in place of `value`. The real plan before promotion
  can then accept an absent `skills` key and the off value.
- Sources: `status` resolves the current Ravel `skills` value in a
  disposable HOME, as gate 6 does, and compares the IDs and digests with
  the receipt's. The real HOME is never written.
- Step 8 of the switch runs `mac-check` before the receipt exists. The
  switch is then an ancestor of `status`, which finds it by its
  `--attempt` argument. That attempt is evaluated as switched from the
  plan and its backup's evidence, and its restore is "in progress". A
  crashed attempt has no such ancestor and reports `restore`.
- The receipt's retained items gain an `identity`. A path's identity is
  the change identity without the top-level `.in_use` reader markers. A
  registration's identity is the digest of its JSON entry. Items whose
  only condition is "this receipt is written" carry none.
- `purge-report` blocks unless health is `switched` and each retained
  identity is unchanged. It lists the conditions that only the owner can
  establish and writes nothing.
- The fixture plan's retained Ravel keg named the real
  `/opt/homebrew/Cellar/ravel/1.1.0`, which the purge report would read.
  It now names the case's own fixture Ravel, so the rehearsal reads no
  file outside its case.
- The real plan needs `mac-check --only "Owner skill switch"` at step 8,
  because the full `mac-check` exits 10 for CCC alone. It also needs a
  native Ravel assertion with `values` before promotion. Both notes are in
  owner-switch-plan-k254.
- Verification: `task owner:switch:rehearse` passed all 315 controls on
  the final bytes, with unchanged subject digests. Its rerun of
  `settings-writes.sh` missed the range by 1 ms for `add` under load, and
  passed alone. All 17 mutants made their controls fail. The checker's
  tests passed 56 of 56, with Ruff and strict pyright clean. `bash
  scripts/check.sh` passed its 18 principal checks. The real `mac-check`
  reports that the switch is not prepared. The section "Owner switch
  status" of `docs/owner-skill-delivery-evidence.md` records the details.
- Retirement: no ADR changes, because the specification owns these rules
  and now names the status entry. The controls and mutants are executable,
  so this leaf cuts no review and spent no in-session reviewer. No review
  names this leaf. The parent node keeps live leaves, so no node closes.

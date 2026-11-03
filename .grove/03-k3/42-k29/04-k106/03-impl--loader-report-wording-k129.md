# loader-report-wording-k129

## Goal

Make the preamble require a visible report after a native loader error and
after a blocked restriction. Then measure `explicit-native-error` and
`explicit-native-only` again. Also state in the preamble that a resume with
the whole history is not a re-entry.

## Context

- The record sections `explicit-native-error` and `explicit-native-only`, and
  "Explicit request rows in Codex".
- The triage section of `docs/common-loading-acceptance-record.md`.
- Ravel's `crates/ravel/src/skills/preamble.rs`, obligations 4, 6 and 10, in
  the `Ravel.skill-directory-delivery` workspace.
- The record sections `continuation` and `continuation in Codex`.
- Ravel's `docs/specs/skill-discovery.md`: the sentence "A native loader
  error other than 'no match' is a report" and the request table.

## Done when

- On the unpushed bookmark `skill-directory-delivery`, obligation 4 tells the
  agent to report another native error before dependent work. Obligations 4
  and 6 state that the report comes before any ending command. The
  specification agrees, and the Ravel tests that pin the preamble pass.
- Obligation 10 and item 4 of "The preamble" in the specification state the
  re-entry rule. A resume that keeps the whole earlier conversation is not a
  re-entry and needs no re-read. A re-read follows a compaction or a fresh
  context. The
  human chose this during acceptance-gap-triage-k124.
- The `continuation` sections of the record state that the re-read row holds
  under the clarified rule, by the human's decision, and name the build.
- The new build is in `scripts/harness-trials/builds.json` and in the record,
  by commit, build identity and `bin/ravel` SHA-256, before any result.
- `explicit-native-error` and `explicit-native-only` run once each in Claude
  Code, and `explicit-native-only` runs once in Codex. A case passes when the
  agent states the error or the limit in text before dependent work and
  before the force-stop command.
- A case that fails again stays a gap, and the leaf asks the human a scope
  question about it.

## Notes

The original runs force-stopped with no text. In a headless run, the text
before the ending command is the only report the human sees.

This change moves the bookmark. The Linux floor and dual-Ravel evidence then
names an older build. `stage-evidence-reconciliation-k125` reconciles that.

## Decisions (running log)

- **Base.** Ravel `main` and `main@origin` are both `24a545236185`. `jj git
  fetch` reported no change, so the bookmark needs no rebase. The change goes
  above `1ca4aecbeae1`.
- **Report wording.** Obligations 4 and 6 say "report … in text before
  dependent work and before any ending command". "In text" names the channel,
  because the original runs had the error only in a tool result. "Report"
  stays the verb, as in obligations 5, 8 and 9.
- **Scope of the wording.** Obligations 5, 8 and 9 keep their text. Their rows
  passed under it, and the Done when names only 4 and 6.
- **Re-entry rule.** Obligation 10 defines a re-entry as a compaction or a
  fresh context. A resume with the whole earlier conversation is not a
  re-entry and needs no re-read.
- **Build record.** `builds.json` takes the new build and points its `source`
  at the record section of this leaf. The dual-Ravel stage record keeps its
  older build for `stage-evidence-reconciliation-k125`.

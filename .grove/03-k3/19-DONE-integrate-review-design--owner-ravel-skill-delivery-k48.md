# owner-ravel-skill-delivery-k48

**Integrates:** owner-ravel-skill-delivery-k41

## Goal

Triage the owner-delivery design review and apply the supported corrections
before the Rhai directory contract and owner cutover planning consume it.

## Context

Read the findings from `owner-ravel-skill-delivery-k41` in its committed
review, and the producer's `owner-ravel-skill-delivery-k37` decision log.
The artifact set is `docs/specs/owner-skill-delivery.md`, its evidence and
inventory, the instrument under `docs/evaluations/owner-skill-delivery/`, the
owner views and `finish-promotes-after-the-grove-release` decision record.
Reconcile the committed Ravel inputs named by the review where material.

## Done when

- Classify every finding as an unclear contract, a real issue, a visible
  trade-off or noise, and record the evidence for the classification.
- Apply accepted corrections to their owning artifacts and reconcile the
  specification, inventory evidence, views and downstream handoffs.
- Verify each correction through the relevant source or controlled evidence
  before k34 and k38 rely on it. Preserve native installations and the live
  driver's original ending contract.
- Externalise any substantial redesign as its own producer/review chain.

## Notes

This integration changes design and evidence. It performs no installation,
owner activation, native deactivation, release or other-workstream tree edit.
Use the review's findings as claims to assess, not as an imposed repair list.

## Decisions (running log)

- Inputs: review commit `fe85297d2b26` against producer `6b2ec873`. The
  Ravel recipe was read at `2a77ffbda3b2510f0997c740c0642b18afaeece7` in
  `Ravel.coordinated-release-design`. Codex 0.166.0 is the Homebrew cask
  binary. The Claude Code loading reference was read on 2026-11-02.
- Finding 1 is a real issue (high). `scripts/harness-smoke.sh:64` runs
  `link_store .codex`. The owner's `~/.codex` holds `config.toml`,
  `hooks.json`, `agents`, `plugins`, `skills` and `auth.json`. The Codex
  binary honours `CODEX_HOME` and names `.codex/config.toml`, `.codex/agents`,
  `.codex/hooks` and `.agents/skills`. The owner policy passes only `-c`
  overrides and sets no `CODEX_HOME`. The repair specifies a separate Codex
  tree with an authentication-only reuse route, and k34 item 8 changes.
  This session launches no harness, because a token refresh through a
  replaced link could invalidate the owner's real sign-in. The first staging
  trial measures the route.
- Finding 2 is a real issue (medium), wider than reported. Six
  `openai-curated-remote` plugins hold 30 skill files under
  `~/.codex/plugins/cache`, including 20 `openai-templates` skills. Each has a
  remote install record and no `config.toml` entry. A whole-home sweep found
  three further skill areas outside every walked root: the `openai-curated`
  marketplace clone, the Codex curated skill catalog and a remote-install
  staging bundle. Their layout shows install sources, not discovery roots.
- Finding 3 is a real issue (medium). The instrument keys hooks by event and
  guessed owner and drops later hooks. Claude Code's `SessionStart` holds six
  hook groups. MCP rows keep names, agent rows keep filenames, tool rows are
  literals, and `digest` omits executable bits and directory links. The
  repair defines a change identity per entry class inside the instrument.
- Finding 4 is an unclear contract (low). The official reference states an
  `.orphaned_at` marker on update or uninstall and removal 14 days later. It
  states no protection for an active reader. Disabling is neither operation, and the reference
  says a running session keeps its loaded plugin set until a reload. The
  active Grove, Linkuistics, TestAnyware and superpowers versions carry no
  marker. Older Linkuistics versions do. The repair restates the rule and
  adds orphan state to the reader evidence.
- Findings 2 and 3 are instrument repairs, not a redesign. The classes,
  dispositions and switch order stay. This session repairs the instrument,
  adds fixture controls and regenerates the baseline read-only.
- Instrument repair: `inventory.py` now defines a change identity for every
  entry class. It walks every Codex plugin-cache marketplace and adds a
  coverage sweep over the four harness homes. Missing inputs say
  `NEEDS-REVIEW`. The new `controls.py` passes 14 of 14 fixture cases. The
  reviewed revision `6138bacf…9792` passes only the two unchanged cases, and
  its whole output stays byte-identical under the other 12.
- Baseline: 444 rows, SHA-256 `cbfac5b8…a0d0`, from two identical runs with
  unchanged subject digests. No skill row changed class. The baseline adds
  54 installed Codex plugin rows, three hidden Claude Code hooks, one skill
  override and 11 coverage rows. No platform or tool skill has a
  Ravel-delivered name.
- Specification: the staging section specifies the authentication-only route
  and a first isolation trial. The lifetime section states the orphan rule.
  The switch compares change identities and records reader evidence. Item 8
  requires the narrower sandbox recipe. The evidence document, the
  final-session view, the view index, the parent brief and the k34 and k38
  leaves carry the corrections.
- `Taskfile.yml` gains `owner:inventory` and `owner:inventory:controls` for
  k38 and finish to rerun. The finish decision record needs no change.
- Verification: the Grove reference-navigation and guide-coverage tests pass,
  and `git diff --check` is clean. The changed prose passes the
  technical-communication checker with no new finding. This session spent no
  reviewer, because the change controls cover the instrument repair. It
  launched no harness, changed no installation and did not run the full
  `scripts/check.sh`.

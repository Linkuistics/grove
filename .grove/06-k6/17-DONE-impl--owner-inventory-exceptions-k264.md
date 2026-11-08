# owner-inventory-exceptions-k264

## Goal

Make the owner inventory and its comparison accept the two exceptions of the
simplified switch design, and nothing wider. These are D3, the platform
subtrees of the native skill roots, and D5, the enabled native guardrail.

## Context

- The section "Concurrent writers" of `docs/specs/owner-skill-delivery.md`
  states both inventory rules. The section "Test seams" states their
  controls.
- `docs/evaluations/owner-skill-delivery/inventory.py` is the instrument,
  and `task owner:inventory:controls` runs its fixture controls. The
  section "Owner switch plan draft integration" of
  `docs/owner-skill-delivery-evidence.md` records D3 and D5.
- The instrument already lists `synced` and `.system` as platform-managed.
  It classifies `guardrail` from any origin except the personal plugin as
  `UNCLASSIFIED`, and it cannot see an enabled `guardrail@personal` as valid.
- The switch's final comparison and the plan's expected inventories read
  this instrument. Find each consumer that compares the native skill roots
  before and after probes, and apply the D3 rule there too.

## Done when

- The comparison sees each ordinary child of `~/.claude/skills` and
  `~/.codex/skills`. It skips only the exact subtrees
  `~/.claude/skills/synced` and `~/.codex/skills/.system`. It still compares
  `~/.agents/skills`, Pi entries and the other baseline roots.
- An enabled `guardrail@personal` passes only from the personal keg's
  registered marketplace, with the native guardrail skill as its only skill.
  An extra skill or another origin fails.
- Fixture controls show each case of "Test seams". A superseded skill beside
  each excluded subtree, at both roots and through a link, fails. A change
  inside `synced` or `.system` alone passes.
- A deliberate mutation of the exclusion or of the origin rule makes its
  control fail. The evidence document records that observation.
- `task owner:inventory` still reads the real HOME only, and its rows for
  the current native state are unchanged.

## Notes

No exclusion may name a parent root or suppress an ordinary sibling. A probe
that writes an incidental file is not a reason for a new exclusion.

## Decisions (running log)

- The instrument gains a `compare BEFORE AFTER` command. Each consumer
  that compares two inventories uses it. The D3 exclusion lives only there,
  as two exact pairs of harness and location.
- The walker treats `synced` and `.system` as platform subtrees only at
  `~/.claude/skills` and `~/.codex/skills`. Elsewhere each is an ordinary
  child. Other hidden children of a root become ordinary children too.
- A platform subtree that is not a plain directory gets its own anomaly row.
  A link in place of `synced` therefore fails, but a change inside it passes.
- D5 reads the origin from the `personal` entry of
  `~/.claude/plugins/known_marketplaces.json`. The source must be the
  directory `/opt/homebrew/opt/personal`, which is the documented
  `marketplace add` path. The plugin's `skills/` must hold only `guardrail`.
- The consumers are `switch_plan_draft.py`, whose `outside` manifest omits
  both native roots, and `outside_changes` in `scripts/owner_switch.py`. The
  draft adds both roots without their platform subtrees. The program skips
  exactly those two subtrees. owner-switch-plan-k254 regenerates the
  committed draft from the real HOME.
- `scripts/check.sh` failed only `every_repository_markdown_reference_resolves`.
  Six `.py#L<n>` links of the plan draft review already failed it in the
  parent revision, and this change moved the line of one. The links lose
  their fragments. Their labels keep the reviewed revision's line numbers.
- `.claude/settings.json` was in the working copy before this session. It is
  not this leaf's work, so the commit leaves it out.

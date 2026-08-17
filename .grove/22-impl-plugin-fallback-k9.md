# plugin-fallback-k9

## Goal

Remove Grove's **silent** dependency on the optional `linkuistics` plugin: audit
every deferral in the rewritten corpus and give each a minimal Grove-local safety
fallback — the preferred answer — or explicit provisioning verification where a
fallback genuinely cannot stand in.

## Context

`CONTEXT-MAP.md` records the dependency plainly: grove's methodology defers
decision-record philosophy to `linkuistics:decision-records` and seam judgement to
`linkuistics:codebase-design`, and the dependency "is still **not
install-enforced**". The `grove` binary provisions its own methodology and nothing
else. So a session in a checkout without the plugin reads an instruction to
consult a skill that is not there — and nothing tells it so.

At the start of this grove there were **14** such deferrals across **7** files:

| file | defers to |
|---|---|
| `content/SKILL.md` | `codebase-design`, `decision-records`, `using-jujutsu` |
| `content/references/grove.md` | `codebase-design`, `decision-records`, `using-jujutsu` |
| `content/grilling.md` | `codebase-design`, `decision-records` |
| `content/ADR-FORMAT.md` | `decision-records` |
| `content/SPEC-FORMAT.md` | `codebase-design` |
| `content/driving.md` | `decision-records` |
| `content/references/commit.md` | `using-jujutsu` |
| `content/references/execute.md` | `decision-records` |
| `content/references/retire.md` | `decision-records` |

Re-derive this list against the corpus as it then stands — leaves 04–07 will have
moved, merged and deleted several of these files, and `plugin-skills-k7` will have
changed what the deferral targets actually say.

**Preferred shape: the minimal fallback.** Grove states enough locally to act
correctly without the plugin, and defers to it for depth. For the ADR AND test
that is one sentence naming the three conjuncts. For the jj commit boundary, grove
already states where its boundary falls and defers the lane — check whether the
local statement is sufficient to commit correctly, since a session that cannot
commit is a session that cannot end. For seam judgement, decide honestly whether a
minimal local statement is possible or whether this is the case that needs
verification instead.

The **duplication objection is real and is the trade-off**: a local fallback is a
second statement of a rule the plugin owns, and this grove's whole thesis is one
canonical source per rule. The resolution is that the fallback is a *permitted
mirror* with a recorded reason — which is precisely what the inventory's mirrors
column exists to hold. Record each one there.

## Done when

- Every deferral in the corpus is enumerated fresh and classified: minimal local
  fallback, or explicit provisioning verification.
- Each fallback is written, and each is recorded as a permitted mirror in the
  inventory with its justification.
- A session in a checkout without the `linkuistics` plugin can complete an
  ordinary task — bootstrap, produce, retire, commit, signal — without reading any
  plugin skill. This is the acceptance test; write it.
- Where verification is chosen over a fallback, the verification is explicit and
  its failure is legible — a session told the skill is missing, not a dangling
  reference it silently steps over.
- `CONTEXT-MAP.md`'s "not install-enforced" relationship paragraph is reconciled
  with whatever this leaf decides, rather than left describing the old state.
- The ADR *Grove keeps a minimal local fallback rather than hard-depending on an
  optional plugin* is raised if it clears the AND test — reworking the set in
  place, never appended beside an existing record.

## Notes

- Sequenced after `plugin-skills-k7` deliberately: auditing deferrals against
  skills that are about to be rewritten would audit text that no longer exists.
- The opposite failure is worth naming: a "fallback" that restates the whole
  plugin skill locally makes the plugin pointless and doubles the drift surface.
  Minimal means minimal — enough to act, not enough to teach.
- Do not resolve this by making the plugin a hard install dependency of the
  binary. The requirement prefers the minimal fallback, and the two contexts ship
  by different paths on purpose (`docs/ARCHITECTURE.md`, *Repository products*).

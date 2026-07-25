# Research surveys

Ten prior-art surveys and comparisons, each commissioned by a research leaf of a past grove. **They are closed historical records, not current-state documents.** Each describes what was true on the date it was surveyed, and none is maintained.

Read them for **provenance** — the evidence chain behind a decision — and otherwise leave them alone. The one edit worth making is to a claim the world has since falsified, and even then the fix usually belongs in the artifact that *cites* the survey rather than here. For when a finding should be retired into an ADR instead of left in place, see [`../../content/driving.md`](../../content/driving.md); the policy that research outlives the grove that commissioned it is grove's constraint 6, not a local rule.

## Why they are not deletable

They are load-bearing as citations. Five skill bodies attribute their prior art here — `using-jujutsu` and `decision-records` name a specific survey in their opening attribution note, and `authoring-conventions`, `doubt-driven-development` and `guardrail` cite `skill-repo-prior-art.md` by finding ID. Two ADRs ([`../adr/symmetric-vcs-rule.md`](../adr/symmetric-vcs-rule.md), [`../adr/self-driving-loop.md`](../adr/self-driving-loop.md)) rest their evidence on a survey, as do [`../driving-a-grove.md`](../driving-a-grove.md) and [`../../content/driving.md`](../../content/driving.md). Removing one breaks a citation in a current-state document.

## Two lineages

Seven came from grove's own history. Three — `grove-recommendations.md`, `jj-agent-prior-art.md` and `skill-repo-prior-art.md` — arrived with the `Linkuistics/skills` graft ([`../adr/skills-monorepo.md`](../adr/skills-monorepo.md)). The distinction matters only when reading a survey's framing: a skills-lineage document takes the skill corpus as its subject, not grove.

Expect unresolvable references. Several surveys cite task handles from groves that no longer exist, some in the numbering that predates the current `<slug>-k<key>` scheme ([`../adr/task-tree-scheme.md`](../adr/task-tree-scheme.md)) — `020-loop-substrate-spike`, `050`, `010`. Those handles are dead by design; `.grove/` is deleted when a grove finishes.

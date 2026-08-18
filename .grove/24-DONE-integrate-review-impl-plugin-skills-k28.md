# plugin-skills-k28

**Integrates:** plugin-skills-k27

## Goal

Triage and integrate the actionable findings from the adversarial review of
`plugin-skills-k7`: restore behaviour lost by compression, keep decision-making
rules on the correct side of each router, and reconcile metadata that still
describes the deleted generic skill.

## Context

### Findings

1. **P1 — `using-jujutsu` moved a push-critical decision into its optional
   command reference.** The body calls itself the contract and sends
   “bookmarks, pushing” to the command surface
   (`plugins/linkuistics/skills/using-jujutsu/SKILL.md:12-19`), but the reference
   contains the behavioural fact that bookmarks never advance across `jj new`
   and therefore must be repointed at the sealed change before pushing
   (`plugins/linkuistics/skills/using-jujutsu/references/jj-commands.md:24-52`). A
   session that reads the contract but not the reference can run a syntactically
   valid `jj git push` that omits the change it meant to publish. This is not
   command lookup detail; it decides the pre-push action, contradicting the
   changelog's claim that only command surface moved to references
   (`CHANGELOG.md:57-66`). Keep the bookmark-advancement rule and the conditional
   pre-push obligation in the body; the exact commands may remain in the
   reference.

2. **P1 — deleting generic `coding-style` dropped its explicit TDD/test-first
   rule rather than overlapping it.** The predecessor required TDD and “write
   tests first” for every language
   (`3fb0fb0f:plugins/linkuistics/skills/coding-style/SKILL.md:12-18`), but none of
   the six surviving language skills states test-first, and Go, Java, C and every
   other uncovered language now trigger no linkuistics coding skill at all
   (`plugins/README.md:19-35`). Moving uniform naming into `codebase-design` does
   not preserve that discipline. The current assertion that the deleted content
   was either model-default advice or language-skill overlap is therefore false
   (`CHANGELOG.md:78-86`). Either retain a narrowly scoped cross-language
   discipline, put test-first into every appropriate surviving trigger path, or
   explicitly justify and document a dependency that actually supplies the same
   behaviour.

3. **P1 — `cli-tool-design` scopes the audit but leaves its refactoring procedure
   universal.** The new applicability rule says a single-purpose/status script
   inherits error, exit-code and non-interactive guidance and little else
   (`plugins/linkuistics/skills/cli-tool-design/SKILL.md:14-22`), and the audit
   checklist now tells reviewers to strike excused lines
   (`plugins/linkuistics/skills/cli-tool-design/references/auditing-and-refactoring.md:6-14`).
   The same reference then unconditionally orders “Add `--json` everywhere”,
   stabilize its schema, and add examples to every help page
   (`plugins/linkuistics/skills/cli-tool-design/references/auditing-and-refactoring.md:75-88`).
   Applied to `plugins/install.sh` — the task's single-purpose example whose
   output is status lines, not parsed data — the audit correctly excuses JSON,
   while the refactoring procedure immediately adds it. Scope the refactoring
   sequence through the same applicability decision.

4. **P2 — `simplify-project` lost two actionable checks during its “prose-level
   only” compression.** The predecessor required asking an authorized contract
   owner when their answer would change a removal's disposition and explicitly
   checking documentation ownership statements
   (`3fb0fb0f:plugins/linkuistics/skills/simplify-project/SKILL.md:51-57`,
   `:128-141`). The current body jumps straight from unknown authority to
   **Defer** and its proof checklist omits ownership statements
   (`plugins/linkuistics/skills/simplify-project/SKILL.md:46-51`, `:109-122`).
   The first omission can strand answerable work; the second can leave stale
   ownership/packaging prose while every link still resolves. Restore both
   behaviours rather than treating them as expendable explanation.

5. **P2 — the `codebase-design` compression weakened a conditional design-it-twice
   branch.** The predecessor required 3–4 parallel candidates and, when a real
   ports-and-adapters seam was in play, one candidate specifically designed
   around it
   (`3fb0fb0f:plugins/linkuistics/skills/codebase-design/SKILL.md:170-179`). The
   surviving text asks only for parallel alternatives with an unspecified
   divergent pressure (`plugins/linkuistics/skills/codebase-design/SKILL.md:98-111`).
   That no longer binds either the candidate count or the seam-specific branch,
   so a session may generate only the three generic interface alternatives and
   never probe the load-bearing seam. Preserve the conditional pressure; decide
   explicitly whether the 3–4 count is still intended.

6. **P2 — the codebase-memory router now makes a false absolute claim.** It says
   “Every failure mode of this tool is silent” while the same body explains that
   ordinary CLI failures write an error to stderr and exit `1`
   (`plugins/linkuistics/skills/using-codebase-memory/SKILL.md:16-22`, `:68-88`).
   The reference's *answer-corrupting* cases are silent and exit zero; the tool's
   failure modes as a whole are not. Narrow the router sentence to the silent
   well-formed-answer cases so it does not contradict its guard procedure.

7. **P2 — the plugin manifest still advertises a deleted universal capability.**
   `plugins/linkuistics/.claude-plugin/plugin.json:4` says the plugin ships
   “universal coding standards” even though the generic skill was removed and the
   remaining coverage is six languages. The enumerated README, context,
   marketplace, changelog, authoring counts and installer comments were updated,
   but this consumer was missed. Describe the surviving per-language coverage,
   or restore a universal capability before retaining that phrase.

### Confirmed verdicts

- `decision-records` retains the ADR three-part **AND** test and the
  minimum-coherent-set/current-state discipline. `codebase-design` retains the
  seam definition, deletion test, two-adapters rule and dependency-category
  table. `using-jujutsu` retains describe-early in its body.
- `using-codebase-memory` keeps the mechanics of a correct CLI call in the body
  and, unlike the jj router, explicitly requires the reference before treating
  output as a codebase answer. Its moved silent-failure catalogue remains
  complete apart from the over-broad router sentence above.
- The git-to-jj mapping tables survived the fold, `git submodule` / `git lfs`
  remain in the body as the narrow git exception, and the absorbed description
  now triggers on translating a git command or concept.
- `cli-tool-design`'s applicability section materially changes the
  `plugins/install.sh` audit: JSON/schema, subcommand-tree, pagination, typed-ID
  and capabilities findings are excused by shape; actionable errors,
  non-interactive behaviour and meaningful exit statuses remain in scope. The
  contradiction is in the later refactoring recipe, not in that audit result.
- All six repo-config-first sentences name configuration their language uses and
  make it authoritative, with the house rules explicitly demoted to fallback.
  Every retained description still has a capability plus an explicit `Use when`
  trigger, summarizes no workflow, and remains comfortably below the 1,024-byte
  frontmatter limit. The new references stay one level deep and below the
  house's table-of-contents threshold.
- The named consumer sweep is otherwise coherent: the README lists fifteen
  skills, `authoring-conventions` says thirteen model-invoked of fifteen and four
  skills with references, the marketplace and installer comments match the new
  distribution, and remaining old skill names are confined to changelog history
  or the folded reference path.

## Done when

- All seven findings are triaged against the committed predecessor and fixed or
  accepted visibly with a reason.
- The jj body prevents a push from omitting the just-sealed change even when its
  command reference has not been loaded.
- The generic-skill deletion has an honest disposition for test-first behaviour
  and unsupported languages.
- CLI applicability scopes both audit findings and the refactoring sequence.
- The dropped simplification checks and design-it-twice conditional are restored
  or deliberately superseded without false changelog claims.
- Codebase-memory distinguishes loud call failures from silent wrong answers.
- Plugin-facing metadata describes the skill set that actually ships.
- Relevant post-fix verification is run by this integration session.

## Notes

This review was inspection-only. It ran no test, build, lint or format command
and edited no production or test file.

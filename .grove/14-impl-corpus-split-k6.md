# corpus-split-k6

## Goal

Split the supporting corpus along the boundary the inventory draws: format
grammar stays, policy moves to its canonical home, rationale and history move to
non-normative docs — while every normative operational rule stays embedded under
`content/` and explicitly reachable by an installed session.

## Context

This is the largest remaining body of prose and the least loaded. Words at the
start of this grove:

| file | words | nominal role |
|---|---|---|
| `content/driving.md` | 5,817 | field-guide habits — mostly rationale |
| `content/TASK-FORMAT.md` | 3,012 | format grammar |
| `content/SPEC-FORMAT.md` | 778 | format grammar |
| `content/BRIEF-FORMAT.md` | 727 | format grammar |
| `content/grilling.md` | 714 | procedure |
| `content/CONTEXT-FORMAT.md` | 483 | format grammar |
| `content/ADR-FORMAT.md` | 317 | format grammar + placement policy |

`driving.md` is the largest single file in the corpus and is on no session's
normal loaded path — it is reached only when a condition names it. **Whether it
survives is settled and no longer this leaf's call: it does not.** What *is* this
leaf's work is the condition on that deletion, and it is not a formality — the
design's first pass claimed the residue was non-normative and direct inspection
refuted it, finding eight sections still carrying imperatives that alter session
conduct. So:

- The spec's **relocation table** names every surviving imperative and its new
  owner. Delete the file only once **every** row is discharged, most of them by
  `loop-step-references-k11` before you run. `rule-ownership-k15` added the row the
  table was missing — `steps-share-the-producers-stem`, the *give every step the
  producer's bare stem* imperative at `driving.md:469-479`, which sat between two
  listed rules under a listed section. Read the table as a list of **rules**, and if
  an imperative in the file resolves to no row, that is a finding, not a residue to
  delete.
- **`TASK-FORMAT.md:21-22` states two rules that are not format grammar** — *one
  task is one session* and *pruning is HITL*. Shed both: the first to `SKILL.md`'s
  `one-task-is-one-session` (`own`), the second as a procedure-register duplicate of
  `references/retire.md`'s `pruning-is-hitl`. Both owners state them before you run.
  What stays is `convention-not-grammar`, widened to cover the shared stem and the
  relative ordering as well as the two declaration lines.
- **Eight `SKILL.md` conditions point at `driving.md` today.** Every one must name
  its rule's new owner instead. A trigger sentence naming a deleted file is exactly
  the reachability failure this grove exists to remove, and it would ship silently.
- Two sections are addressed to the **human** driving a grove, not to a session —
  *Ask the LLM "WDYT"* and *Ask for pushback* — so a session cannot obey either.
  Their session-facing residue is already `escalation-names-the-tradeoff` and
  `grilling-procedure`; the operator half goes to **`docs/USAGE.md`**, which
  already exists for that audience.
- The rest is argument, worked example and provenance, and it is **deleted rather
  than relocated**. The worked examples are anchored on the sync-semantics grove
  and cite a work item that resolves nowhere — which the glossary's own handle rule
  forbids in provisioned content — and the arguments that still bind are in the
  ADRs. `docs/` earns a relocation only where a human-facing document already wants
  the material.

**The hard constraint, restated because this is the leaf most likely to breach
it:** moving a normative operational rule into `docs/` deletes it. An installed
session reads the provisioned skill directory and cannot open the repository. So
for every rule you move, either it is not normative (rationale, history,
argument — move freely) or it stays under `content/` and something on a loaded
path reaches it. "A future maintainer can find it in the repo" is not
reachability.

**`ADR-FORMAT.md` is the settled home of the ADR AND-test**, stated locally rather
than cited to the plugin. Make it stated once across `ADR-FORMAT.md`,
`grilling.md`, `references/execute.md`, `references/design.md` and
`references/retire.md`, which today say it four ways including one OR-form.

Two more moves are this leaf's under the corrected map, both because an artifact
Occasion outranks a loop-step one:

- **`records-are-current-state` splits.** It becomes
  `adr-set-is-minimum-coherent` in `ADR-FORMAT.md` and `spec-set-is-current-state`
  in `SPEC-FORMAT.md`. `references/execute.md` states neither afterwards — and
  because `loop-step-references-k11` deliberately leaves its copy alone, **you
  remove it in the same commit that lands the two format-file statements**, so the
  rule is never homeless between two commits.
- **`glossary-is-the-forcing-function` and `challenge-and-sharpen-terms` land in
  `CONTEXT-FORMAT.md`**, which is the file every session about to write a glossary
  entry opens. `grilling.md`'s statements of them, and of
  `glossary-is-only-a-glossary`, are procedure-register duplicates and become
  pointers. `research-to-adr-bridge` lands in `ADR-FORMAT.md`.

`grilling.md` sheds four duplicate sections this way and keeps three rows —
`grilling-procedure`, `no-writes-before-shared-understanding` (inside the
byte-intact `<what-to-do>` block, so the bundled sentence *is* the canonical
statement) and `probe-with-concrete-scenarios`.

## Done when

- Each file above is classified — grammar, policy, procedure, or rationale — and
  its content moved to the home the inventory assigns.
- `driving.md`'s disposition is decided and executed, with the reasoning recorded.
- No normative operational rule now lives only outside `content/`; for each
  relocation, the leaf can name what reaches it.
- The ADR AND-test has exactly one canonical statement.
- Non-normative rationale and history relocated to `docs/` do not duplicate
  `docs/ARCHITECTURE.md` — fold into it where it is already the subject rather
  than writing a second account.
- Every markdown cross-reference still resolves (`tests/methodology.rs`'s
  reference checks pass), and the linked embed still carries every markdown file
  on disk under `content/`.
- `behavior-evals-k3` is still green.

## Notes

- Bundled-source files carry provenance comments and license references
  (`content/LICENSES/`, and the `mattpocock` / `addyosmani` / `openspec`
  attributions). Moving or splitting one must carry its attribution with it —
  check `content/grilling.md` and `content/CONTEXT-FORMAT.md` in particular, whose
  headers record deliberate divergences from upstream that must not be silently
  dropped.
- Deleting content is in scope where it is genuinely dead; deleting a rule because
  it reads as rationale is the failure mode. When a passage argues *and* rules,
  split it rather than choosing.
- This leaf is the one whose mistakes are least visible — a rule quietly stranded
  outside every loaded path fails no test that exists before
  `loaded-path-budgets-k10`. Cut `review-impl` as this leaf's last act, with that
  specific doubt written in: *which normative rule is now unreachable?*

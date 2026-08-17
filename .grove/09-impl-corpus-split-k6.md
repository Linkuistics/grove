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
normal loaded path — it is reached only when a condition names it. Whether it
survives as an embedded file at all, or becomes a non-normative doc with its few
genuinely operational rules rehomed, is this leaf's call.

**The hard constraint, restated because this is the leaf most likely to breach
it:** moving a normative operational rule into `docs/` deletes it. An installed
session reads the provisioned skill directory and cannot open the repository. So
for every rule you move, either it is not normative (rationale, history,
argument — move freely) or it stays under `content/` and something on a loaded
path reaches it. "A future maintainer can find it in the repo" is not
reachability.

`ADR-FORMAT.md` is the likely home of the **ADR AND-test** canonical statement if
the inventory put it in a format file rather than a kind reference — check, and
make sure it is stated once across `ADR-FORMAT.md`, `grilling.md`,
`references/execute.md` and `references/retire.md`, which today say it four ways.

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

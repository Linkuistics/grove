# node-methodology-k8

## Goal

Make the shipped methodology guide sessions through the node-file grammar and
mandatory brief chain, with its conformance and installation checks agreeing.

## Context

- Depends on `node-files-k7`. The root brief and the canonical naming ADR
  supply the terminology; the reviewed design supplies no naming history.
- Primary surface: `plugins/grove/skills/`, its format and reference files,
  the conformance rule registry and checker, plus plugin guidance and other
  shipped skill references that actually state Grove's task-tree grammar.
- Read the skill-authoring instructions before editing skills. Distinguish a
  node file (filename role) from its brief (body); respect canonical rule owners
  and reachability instead of copying the rule into every kind.

## Done when

- Bootstrap, decomposition, retirement, planning and artifact guidance name
  the root `_BRIEF.md` and positioned node `_<slug>.md` accurately. The
  mandatory node-file rule replaces advice to skip a missing Grove brief;
  `brief-chain` is the source of paths, with malformed levels refused.
  In `grove/references/driver.md`, remove the assertion that fresh-tree
  creation writes a format witness. Describe the actual root file and first
  leaf only, and reconcile any other witness-creation teaching encountered.
- Format examples show slugless node directories, named node files and
  unchanged leaf filenames. Handles and heading conventions remain accurate,
  and title extraction is always from names. Skill references to their own
  format document `BRIEF-FORMAT.md` remain real links.
- Classify the grammar-bearing instructions across the shipped corpus,
  registry and tests. Update each rule at its owner and reconcile consumers;
  no unrelated rule-owner redesign or record of naming history is introduced.
- Plugin conformance, its own tests, install tests and the instructed-verbs
  seam pass. Run `bash scripts/check.sh` before committing.
- No source book update is normally owed. If a checker or test correction
  changes a declared source root, include that book's fragments and prose now;
  no obligation is passed to `node-documentation-k9`.

## Notes

The independent demonstration is the shipped skill set and its conformance
suite teaching a coherent bootstrap/decompose/retire path for the built CLI.
Do not refresh installed skills or the marketplace cache yet: the installed
20.2.0 driver still owns this tree until `node-cutover-k10`.

Required review work belongs before `node-documentation-k9`.

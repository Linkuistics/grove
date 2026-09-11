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

## Decisions (running log)

1. Keep the existing rule owners: `BRIEF-FORMAT.md` owns node-file grammar
   and the mandatory file; bootstrap owns consuming `brief-chain` paths and
   refusing gaps; retirement consumes that validated chain. Other references
   name the brief as body/context rather than repeating the grammar. Preserve
   skill-format links and upstream provenance names.
2. Use the existing conformance control suite for the checker change: root and
   titled node files are tree artifacts, while `BRIEF-FORMAT.md` must resolve
   as a shipped skill reference. The graph CLI refused startup because an
   incompatible generation is active; discovery uses direct source reads.
3. The shipped-corpus classification is: node-file grammar and heading examples
   in `BRIEF-FORMAT.md`; leaf grammar and handle consumers in `TASK-FORMAT.md`;
   path consumption in bootstrap; scaffold, walk and refusal in driver;
   body/context consumers in planning, draft, spec, decomposition, editorial,
   artifact and retirement guidance. Format-document references remain skill
   links; upstream `AGENT-BRIEF.md` is provenance. Plugin guidance, installation
   tests and the other shipped skill families state no additional Grove node
   filename rule. Registry owners stay in place, with gap tolerance and driver
   migration replaced by current refusal rules.
4. The checker classifies node filenames as tree references and accepts leaf
   positions wider than two digits. Scratch probes demonstrated both former
   misclassifications. Existing control tests now cover these references,
   missing format links and deletion of the mandatory-file/refusal instructions.
   The instructed-verbs seam needs no change, and no changed file is a declared
   book source root. The first full check passed all eight groups and all six
   books; every tracked file's SHA-256 was unchanged across that run.
5. A focused review of the shipped guidance is earned before documentation:
   phrase-based conformance cannot establish semantic consistency across the
   bootstrap/decompose/retire path. Schedule it ahead of `node-documentation-k9`;
   installation and this live tree's naming remain owned by cutover.

## Verification

The final `bash scripts/check.sh` passed all eight principal checks: 21 plugin
installation cases, 16 conformance controls, plugin conformance, workspace tests
(including the instructed-verbs seam), and all six books, alongside formatting,
shell lint and Clippy. SHA-256 digests of every tracked file matched before and
after the run. No book source root changed. Review is queued as
`node-methodology-k20`, before documentation.

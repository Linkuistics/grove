# distinguished-names-k6

## Goal

Deliver the library's supplied distinguished names and enforced level validation
as one working API, with all consumers compiling and behaving correctly.

## Context

- The reviewed design is `node-grammar-k2`, integrated at `node-grammar-k5`.
  `docs/ordinal-fs-tree/ARCHITECTURE.md` owns the trait, enforcement points and
  conformance obligations; `docs/adr/entry-name-is-the-only-seam.md` owns their
  placement. Read the model evidence and limits in formalism finding 049.
- Primary surface: `ordinal-fs-tree`'s names, snapshot reader, operations,
  projected plans, guarded filesystem API, errors and conformance kit. Include
  the reference domain, syllabus CLI, every test domain and Grove's API callers.
  Source entry points include `name.rs`, `fs/read.rs`, `fs/mod.rs`, `ops.rs`,
  `plan.rs`, `conformance.rs`, `reference.rs` and `bin/syllabus.rs`.
- Grove's adapters include `TaskName`'s trait/conformance implementation and
  `tree_lifecycle`'s initialization and promotion calls. Their grammar switch
  belongs to `node-files-k7`.

## Done when

- `EntryName` accepts deterministic root-or-node level validation. Promotion
  receives its destination distinguished name and initialization an optional
  name-and-bytes pair. Callers supply names explicitly; no fixed-name factory
  or temporary compatibility interface remains.
- Distinguished identity compares canonical rendered names. Positioned identity
  retains both view and species. Names still render as one component; the
  library extracts no labels, reads no content and introduces no second domain
  seam.
- Every complete direct listing is validated before exposure, and the whole
  reachable tree is checked before any search answers. Domain errors preserve
  the path and grammar; the library independently rejects multiple distinguished
  children even when a permissive domain accepts them.
- The same check validates projected final levels before effects for append,
  batch, insert, initialization and its entries, promotion and its optional
  child, and node rewrites. Missing required files and inappropriate supplied
  names refuse without effects. Successful operations leave valid levels;
  reported-failure rollback and process-interruption limits remain intact.
- The conformance kit takes explicit distinguished-name and expected-level
  samples, including distinct names, missing/single/competing sets, permutations
  and root/node placement. Expectations come from fixtures. Reader and planner
  tests demonstrate enforcement separately from the kit. Include a node-parts
  policy test because the checked models do not establish that case.
- Existing Rust seams cover acceptance and refusal, guard boundaries, unchanged
  trees on refusal, malformed later subtrees despite early matches, and ordinary
  reference-domain behavior. The library's kit runs over Grove's `TaskName`;
  Grove's existing grammar and CLI fixtures still pass after the API adaptation.
- Every changed source root lands with its book fragments, ledgers, indexes and
  explanatory prose in this commit. Expect `ordinal-fs-tree` and `grove-loop`;
  discover any others from the manifests. `bash scripts/check.sh` passes.

## Notes

The independent demonstration is a domain that supplies different valid names
for different levels, successfully initializes/promotes, and rejects invalid
levels on reads and plans. This is the library increment, with its necessary
caller adaptation, not a second grammar in Grove. If implementation exposes a
model disagreement, record it and run the relevant model runner before changing
the contract; do not replace model limits with claims of exhaustive proof.

Keep the installed driver, installed plugin and this live tree untouched. Use
the installed `grove-llm` for task bookkeeping. Any review earned here must be
inserted before `node-files-k7`, so its dependent does not consume unreviewed
API decisions. Decompose at a working seam if the scope proves too large.

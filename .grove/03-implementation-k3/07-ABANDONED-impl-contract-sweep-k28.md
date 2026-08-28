# contract-sweep-k28


## Goal

Prove the redesigned repository is coherent, contract-preserving, model-conformant, and free of superseded structure.



## Context

This is a contraction and verification task, not a place to hide new features. Compare against `experiment-baseline-k4` and the formal/documentation gates; resolve every stale consumer before deleting its old owner.

## Done when

- Old root modules, migration remnants, reach-through imports, compatibility shims, duplicate fixtures/tests/docs, empty directories, unused dependencies/features, and stale build/release inputs are removed with consumer evidence.
- Repository-wide search finds no stale migration promise, old model/doc path, old module name, or contradicted finish behaviour in source, comments, docs, scripts, manifests, tests, examples, or packaging.
- The common model runner executes non-zero Alloy and Quint witnesses/checks/tests/verifications successfully; every component README and formal claim link is current.
- Formatting, linting, workspace tests, CLI snapshots/contracts, doctests/examples, real Git/native-jj/colocated-jj fault suites, MSRV 1.85 checks, glibc 2.17/release build-doctor, and embed/install packaging checks pass from clean state.
- Baseline comparison explicitly accounts for every observable difference; migration removal is the only planned break, and any model-justified diagnostic/artifact change is documented with approval evidence.
- Before/after crate dependency surface, public interfaces, source layout, direct filesystem call sites, unsafe code, and concept inventory are reported as explanatory metrics, not substituted for acceptance tests.
- Every durable conclusion from the deleted `TODO.finish_process.md` exists in models, tests, current-state docs, or code. Its durable half is already `docs/adr/finish-keeps-a-cleanup-layer-it-has-not-proved-forced.md`; what this sweep checks is that the *code* and *test* obligations it names are met and that no reference to the removed file survives. The next Grove traversal sees no unretired work in this mandate.

## Notes

If verification exposes a new semantic defect, create a narrowly scoped leaf before this one and fix it there; do not waive or bury it in the final report.

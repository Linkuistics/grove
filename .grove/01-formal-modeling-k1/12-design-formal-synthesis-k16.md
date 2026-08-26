# formal-synthesis-k16


## Goal

Convert the checked models and replay evidence into binding design decisions and an executable hand-off to documentation.



## Context

This is the formal-phase gate. Its conclusions are local to this experiment, not universal claims about Quint, Alloy, or formal methods. Durable evidence belongs in model READMEs and `docs/formalism-findings.md`, not only in this task file.

## Done when

- Both model families and the common runner are green, have non-zero witnesses/checks, and document every bound, assumption, omitted behaviour, and unresolved tool limitation.
- The shared claim catalogue and component/system model READMEs agree on stable semantics, error taxonomy, VCS refinements, filesystem responsibility, and model-to-crate ownership.
- Experiment 2 contains a bounded synthesis: which formalism caught what, what neither established, cost/counterfactual/verdict, useful combined workflow, and concrete changes to design/tests/docs.
- Every proposed finish simplification from `TODO.finish_process.md` is classified keep, delete/replace, or defer with a claim and replayed evidence. “Model is smaller” is not evidence.
- The ordinal lifecycle experiment has a keep/defer/reject decision. If kept, insert an `impl` leaf immediately before `extract-task-tree-k24` using `grove-llm leaf-insert extract-task-tree-k24 ordinal-root-lifecycle --kind impl`.
- For each model-earned finish simplification, insert one narrowly named `impl` leaf immediately before `collapse-application-k27`, preserving the intended execution order. Do not create a generic “simplify finish” bucket.
- Documentation tasks have no unresolved semantic questions and all durable formal artifacts are linked from their component owners.

## Notes

Retire the formal subtree only after the model commands have been rerun from a clean checkout-equivalent state. Review chains should be added here only for decisions whose uncertainty or blast radius warrants an independent session.

# decomposition-k2

## Goal

Turn `minimalism-k1`'s five module contracts into interfaces a session can build
against, and land the agreement point: a spec under `docs/specs/`, signed off by
the human, before decomposition cuts impl leaves.

## Context

`minimalism-k1` is the whole input and it is unusually complete — read its
`## Decisions (running log)`, `## Module contracts` and `## Deletion list` before
anything else. Nine decisions were settled with the human and none of them is
open for re-litigation; `SPEC-FORMAT.md`'s *synthesise, never re-interview* rule
binds hard here.

The four that most constrain this session:

- **jj only.** The git lane is dropped, which is what makes the 10,366-line
  finish group deletable rather than lane-conditional.
- **Five workspace crates under `crates/`**, one release process. A module *is* a
  crate, so "testable through its own interface without the other four" is
  compiler-enforced.
- **The kind is a skill name.** `${prompt}` says *load `grove-<kind>`*; grove
  holds no set of kinds, no reference-file mapping and no `match`. A kind exists
  iff its skill does.
- **The methodology ships as a plugin and the version check inverts** — grove
  publishes a version, the skill checks it and decides.

Also read `docs/research/wording-micro-test.md` before touching prompt
composition: the 0/10-vs-10/10 result is the reason the prompt names one target
and asks the session to select nothing, and any redesign of `${prompt}` has to
preserve that property or restate the evidence against it.

## Done when

- Each of the five crates has a named public interface — types and signatures,
  not prose — that a reader can implement against.
- The spec is written under `docs/specs/`, cites the ADRs in its area rather than
  restating them, and carries the four agreed test seams.
- The human has agreed the shape.
- The next leaf is cut. `references/decompose.md`'s **expand → migrate →
  contract** rule almost certainly applies: this is a wide refactor whose blast
  radius makes any single vertical slice unable to land green.

## Notes

**Seven things `minimalism-k1` deliberately left to this session.**

1. **Crate names.** Five of them, and `ordinal-fs-tree` is already taken and
   staying.
2. **The store's four new operations.** `exists?`, `initialize`, `delete` and a
   no-outcome answer to a search. The last is the sharp one: grove's current
   no-work signal is `Option<SelectedLeaf>` (`task_tree.rs:584`), whose predicate
   is grove vocabulary and **cannot move as-is**. The store needs a way to say
   *found nothing* that is domain-free, alongside `Refusal`'s twelve variants,
   all of which are refusals to *mutate*.
3. **What the name parser yields once `leaf::Kind` loses its closed enum.** It
   still parses a kind out of a filename; it simply no longer validates one
   against a compiled set.
4. **The handle type.** Principle 3 says one type owns a name end to end, and the
   handle is a projection of the same name — six hand-rolled implementations
   today (`task_tree.rs:513`, `tree_lifecycle.rs:220`, `finish_cleanup.rs:121`,
   `task_grow.rs:475`, `tree_lifecycle.rs:1174` produce; `task_tree.rs:952` and
   `task_name.rs:609` peel). `EntryName` in the store is the model to follow.
5. **What grove publishes as its version/signature, and how a skill reads it.**
   One workspace, one release version (decision 2), so the value exists; the
   surface does not.
6. **How fat the nineteen skills are.** The kind is the skill name, and whether
   each is a thin pointer over a shared `grove` spine or a full standalone skill
   was left open on purpose.
7. **The plugin's conformance runner.** `behavioural-coverage-asserts-delivery`'s
   rule survives and its instrument moves out of `cargo test`; the plugin has no
   test runner today.

**On ADRs.** `minimalism-k1` rewrote none, deliberately — `docs/adr/` describes
the design's *current state*, and rewriting a record to describe unbuilt code
would make the set lie. Its `## ADR reconciliation` section names which records
each decision obliges and who reworks them. The spec is where the target design
lives until then; cite the records, do not restate them.

# stale-module-headers-k14

**Kind:** impl

## Goal

Sweep the "Built **isolated**" module headers that still describe a v1 verb path
this repo no longer has.

## Context

Raised by `chain-node-integrate-k11` while reconciling a *different* stale claim
in the same headers, and externalized rather than absorbed: this is v2-flip
residue, not chain-node work, and it spans five modules.

Five `src/` modules open by declaring themselves new, isolated code that "does NOT
touch the live v1 verb path", naming modules that no longer exist:

| File | Names as live |
|---|---|
| `src/tree_id.rs` (~L36) | `leaf_read` / `leaf_grow` / `leaf_lifecycle` / `migrate` |
| `src/tree_grow.rs` (~L28) | `leaf_grow` and "the `llm_cli` dispatch to it" |
| `src/tree_read.rs` (~L14) | the v1 read path |
| `src/tree_lifecycle.rs` (~L30) | `leaf_lifecycle` |
| `src/tree_migrate.rs` (~L30) | `migrate.rs`, "until the user-gated re-flip (11.6)" |

Only `src/leaf_id.rs` survives of that generation, and only as `tree_migrate`'s
one-time v1 parser. The re-flip they all forward-reference has happened.

`src/leaf_id.rs:13` carries its own variant ("Per D9 this module is **new,
isolated code**"), which needs the opposite treatment: say what it is *now* — the
migration's v1-flat name parser, and nothing else.

## Done when

Each header describes what the module is today. The forward-references to a gate
that has already passed are gone, along with the internal `11.x` / D9 step numbers
where they no longer locate anything a reader can follow.

Also confirm — do not assume — that `leaf_id`'s only remaining caller is
`tree_migrate`; if its `next_key` / `parse` surface is genuinely dead beyond that,
say so rather than trimming it in this leaf.

## Notes

**Prose only, or a finding.** If the sweep turns up code that is actually dead
rather than merely mis-described, that is a finding to surface, not a deletion to
fold in — a module's header being wrong is cheap to fix, and removing a symbol is
a different decision with a different blast radius.

**This is the failure mode `chain-node-review-k10` named**, one generation back: a
current-state document that kept a claim after the thing it described changed.
Worth a moment on whether these headers should carry step numbers at all, given
`.grove/` dies at the finish cycle and takes the numbering's referent with it.

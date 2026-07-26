# kind-set-k13

**Kind:** work

## Goal

Grow `leaf::Kind` from five kinds to seventeen and rename `work` to `impl`,
without breaking a single live grove. No routing changes — every new kind falls
through to the stamped harness, which is what makes this slice demoable on its
own.

## Context

- `src/leaf.rs` — `Kind`, its `parse` (the write gate) and `label` (the inverse,
  and the single source of truth for spelling, so the grow verbs' template and
  the `kind` verb can never disagree).
- `src/tree_read.rs` — `read_kind`, the read-side counterpart that **degrades**:
  a missing, empty or unrecognised `**Kind:**` line warns and yields `work`.
- `src/loop_driver.rs` — `KIND_SUFFIXES` and `env_suffix`, the kind → env-name
  mapping. Both grow. `any_model_env` and `any_harness_override_env` sweep the
  suffix list, so its length is a hot path on every launch.
- `src/llm_cli.rs` — the `--kind` help text on `leaf-add` / `leaf-insert` /
  `leaf-decompose` enumerates the five by hand.
- The spec from `taxonomy-spec-k12` is the authority on the kind list.

## Done when

- All seventeen kinds parse, round-trip through `label`, and are listed in the
  `--kind` error.
- **`work` is accepted on read as an alias for `impl`.** This is the
  load-bearing compatibility rule: six live groves have task files saying
  `**Kind:** work`, and without the alias every one of them starts emitting an
  "unrecognised kind" warning and silently degrading. Writes gate to `impl`.
- The read-degrade target is `impl` (the renamed default), and the degrade path
  still never errors.
- `KIND_SUFFIXES` and `env_suffix` cover all seventeen; the suffix for a
  hyphenated kind is its uppercased underscore form (`review-impl` ⇒
  `REVIEW_IMPL`).
- Existing tests still pass, plus coverage for: the alias, the seventeen-way
  round-trip, and the error listing every kind.

## Notes

**Do not delete the `work` alias in this leaf.** It is the reason the rename is
safe, and removing it is a separate decision with its own blast radius (every
live grove's task files would need rewriting first). If it should ever go, that
is a `leaf-add`, not a line in this one.

The alias asymmetry is deliberate and matches the taxonomy's existing shape:
**write gates** (a human is present to fix a typo), **read degrades** (the
unattended loop must never jam). An alias is a read-side concession only.

Watch the hot path: `any_model_env` runs `KIND_SUFFIXES.len() × 2` env lookups
on every launch to decide whether the kind peek is worth a subprocess. At five
that was 10; at seventeen it is 34. Still trivial, but confirm the
zero-subprocess unconfigured path is genuinely unchanged rather than assuming it.

**Do not invest in that path beyond keeping it working.**
`required-model-vars-k18` deletes it outright — once a model var is *required*,
the peek must run every iteration and the `any_model_env` short-circuit has
nothing left to decide. Grow the suffix list correctly here; leave the
optimisation alone.

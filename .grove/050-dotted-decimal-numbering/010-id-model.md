# 010-id-model

**Kind:** work

## Goal

Build the new-format **id model** — the foundational type + pure functions every
other verb (020/030/040) consumes: parse a filename into its parts, the
position-vector **version-sort comparator**, and **next-key** assignment. New,
isolated code (D9) — do **not** touch the live `grove-llm` verb path.

## Context

Read **ADR-0033** and the parent BRIEF's running log **D1, D2, D3, D6** first.
Grammar (authoritative, D6): `<position>-[<key>]-<slug>[.BRIEF|.DONE].md`; root
brief `BRIEF.md`. The current old-format parsing lives in `src/leaf.rs`
(`split_prefix`, etc.) and `src/pick.rs` (`sort_key`) — leave both untouched (D9);
build the new model **alongside** (new module, e.g. `src/leaf_id.rs`).

## Done when

- A parsed type captures `(position: Vec<u32>, key: Option<u32>, slug, is_brief,
  is_done)` from a filename; root `BRIEF.md` parses as the unkeyed root brief.
- Parse is **lenient** (D2): a name that doesn't match the grammar is "not a
  leaf/brief" — no panic, no error.
- The **comparator** orders by the position vector only (element-wise integer,
  shorter-prefix-first); key/slug/`.BRIEF`/`.DONE` excluded; total order
  (filename tie-break for malformed same-position collisions). Unit-tested against
  D2's edges: `[2,9] < [2,10]`, `[2,2] < [2,2,1]`, `[] < [1]` (root first),
  foreign-sorts-last.
- **next-key**(tree) = max `[n]` over all files (live **and** `.DONE`) + 1; `1` on
  an empty tree. Tested incl. a retired `.DONE` file preserving the max (no reuse).
- Round-trip: parts → filename → parse → same parts.
- All pure functions, exhaustive unit tests. **Not** wired into the live verbs.

## Notes

- Reserved slug words `BRIEF`/`DONE`; slug excludes `.`/`[`/`]` (extend slug
  validation for the new grammar — old `validate_slug` is in `src/leaf.rs`).
- This is the foundation; 020/030/040 import it. Keep it dependency-free of the
  verb modules.

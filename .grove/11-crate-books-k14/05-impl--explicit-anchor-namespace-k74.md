# explicit-anchor-namespace-k74

## Goal

Make `explicit_anchors()` in `crates/grove/tests/reference_navigation.rs` collect
the anchors `docs/ARCHITECTURE.md` actually **publishes**, so every check that
resolves against that namespace — the repository-wide Markdown sweep, the ADR
citation scan, and the Rust-source anchor scan — agrees with what a browser
resolves. Land it green against today's tree.

## Context

Found by the adversarial read inside `architecture-anchors-k19`, which built the
Rust-source scan on this helper and inherited its namespace. Both defects are
**latent today** — `docs/ARCHITECTURE.md` currently has no fenced, single-quoted
or `<a name=>` anchor — so this is a hardening leaf, not a bug fix. Both were
reproduced against a scratch document during that session and reverted.

- **A fenced `<a id="…">` joins the namespace.** `explicit_anchors` is a naive
  `split("<a id=")` over the whole document with no fence tracking, unlike
  `markdown_headings` beside it, which tracks fences properly. An anchor written
  inside a code fence is literal text to a renderer and carries no `id`, so a
  citation naming it resolves nowhere in a browser while every check here passes
  it. Reproduced: a fenced `<a id="ghost-anchor"></a>` plus a
  `docs/ARCHITECTURE.md#ghost-anchor` citation in `crates/grove-loop/src/lib.rs`
  went green.
- **A single-quoted `<a id='…'>` is not in the namespace.** The helper hard-requires
  `strip_prefix('"')`, so the other legal spelling of the same explicit anchor is
  reported as broken. Reproduced: an appended `<a id='quoted-anchor'></a>` and a
  citation of it went red.
- The fence machinery to reuse is already in the file — `fence_start`,
  `closes_fence`, `up_to_three_space_indented`, and the loop shape in
  `markdown_headings`.

## Done when

- `explicit_anchors` skips fenced blocks and accepts both quote spellings, and
  each behaviour has a fixture asserting it.
- Each is seen to fail before it is credited: a fenced anchor that used to
  resolve stops resolving, a single-quoted one that used to be rejected starts
  resolving, and both are put back.
- `bash scripts/check.sh` passes with nothing else changed.

## Notes

**`<a name="…">` is deliberately out of scope unless the session argues its way
in.** It is legacy HTML and not this repository's convention: the *Documentation
ownership* section names the `<a id="…"></a>` form, and the book outbound-link
contract requires exactly that spelling. Admitting `<a name=>` would widen the
namespace to a form nothing here is allowed to write.

**This runs before `architecture-move-k31` on purpose.** The move restructures
the document these anchors live in, and a namespace that admits anchors the
renderer does not publish is at its most dangerous exactly then.

**Do not touch `docs/ARCHITECTURE.md`.** The document is `architecture-move-k31`'s;
this leaf changes only the test helper. If the fix turns anything red, that is a
finding to report, not a document to edit.

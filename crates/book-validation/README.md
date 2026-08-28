# Book validation

`book-check` validates the walkthrough's fragment graph, Markdown structure,
and local navigation without changing the book or production source. During
authoring, validate the canonical prefix through the current source-owning
slice:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --through read-path-k14 \
  --check all
```

Final assembly requires all fifteen source roots, all 6,929 source lines, and
no deferred ranges:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --final \
  --check all
```

Use `--check fragments` or `--check markdown` to isolate one validation class;
the default is `--check all`. Markdown validation checks the canonical page
inventory, identities, heading and explicit-anchor shape, navigation, local
files and anchors, and external URL syntax. It never fetches an external URL.

Use `--output json` for the versioned machine-readable envelope. Exit status
`0` means valid, `1` means deterministic validation findings, `2` means invalid
invocation or an input load failure, and `3` is reserved for an internal
validator failure. The command is non-interactive, read-only, and reports
findings in stable order.

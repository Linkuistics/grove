# Book validation

`book-check` validates the walkthrough's fragment graph without changing the
book or production source. During authoring, validate the canonical prefix
through the current source-owning slice:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --through read-path-k14 \
  --check fragments
```

Final assembly requires all fifteen source roots, all 6,618 source lines, and
no deferred ranges:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --final \
  --check fragments
```

Use `--output json` for the versioned machine-readable envelope. The later
diagnostic-contract increment completes its stable evidence and ordering. Exit status
`0` means valid, `1` means deterministic validation findings, `2` means invalid
invocation or an input load failure, and `3` is reserved for an internal
validator failure. The command is non-interactive and read-only.

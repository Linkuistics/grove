# Book validation

`book-check` validates a walkthrough book's fragment graph, Markdown structure,
and local navigation without changing the book or production source. It knows
nothing about any particular book: the page inventory, the page-to-slice
mapping, the canonical slice order and the accepted `--through` values all come
from the `walkthrough.toml` manifest in the directory named by `--book`.

During authoring, validate the canonical prefix through the current
source-owning slice — `--through` takes one of the slices the named book's
manifest declares, and an unrecognized value lists them:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/walkthroughs/<book> \
  --through <slice> \
  --check all
```

Final assembly requires every source root the book declares, every line of them,
and no deferred ranges:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/walkthroughs/<book> \
  --final \
  --check all
```

Use `--check fragments` or `--check markdown` to isolate one validation class;
the default is `--check all`. Markdown validation checks the canonical regular
file inventory (without following symlinks), identities, heading and
explicit-anchor shape, navigation, local files and anchors, literal-fragment
introductions, and external URL syntax. Relative links may target only the book's
own declared pages and its declared source roots. The validator never fetches an
external URL.

Use `--output json` for the versioned machine-readable envelope. Exit status
`0` means valid, `1` means deterministic validation findings, `2` means invalid
invocation or an input load failure, and `3` is reserved for an internal
validator failure. The command is non-interactive, read-only, and reports
findings in stable order.

`docs/specs/walkthrough-books.md` is the contract this implements, and its *The
manifest* section is the schema `walkthrough.toml` is checked against.

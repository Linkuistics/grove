# Documentation guidance: visual companion

These views accompany the [documentation guidance spec](../../specs/documentation-guidance.md),
which owns every decision. Captions state each view's scope and omissions and
are not a second decision log.

## Views

- **Owner map** (`owners.d2`) — which file owns each new rule, and which
  sessions open it.
- **Teaching placement** (`teaching-options.d2`) — where the teaching rules
  that documents and exchanges share live, and which sessions open them.
- **Contract flow** (`contract.d2`) — who writes and who reads each part of
  the document contract.
- **House style** (`house-style.d2`) — which prose contract governs one
  point, ending at the house style when a set declares none.
- **Reader checkpoint** (`checkpoint.d2`) — where a human read runs, and where
  its corrections go.
- **Increments** (`increments.d2`) — the slicing and its dependencies.

Every view marks its reading entry with a dark "Start here" node and carries a
legend for its shapes and arrows.

## Sources, exports and regeneration

The `.d2` files are the editable sources. Each `.svg` beside one is its
tool-generated export, committed with it. [`regenerate.sh`](regenerate.sh) is
the recorded regeneration command: it requires D2 v0.9.0 and renders with
`--layout=elk --theme=0 --dark-theme=200 --pad=24`. From the repository root:

```sh
task figures:regenerate   # rewrite the SVGs after editing a source
task figures:check        # regenerate elsewhere and compare bytes
```

The check shows that each committed SVG matches its source under the pinned
tool. It does not show that the views render where they are read. That was
checked separately by opening the served viewer in a browser.

## Viewing

`index.html` is the static viewer shipped by the `grove-design` skill, copied
unchanged. It needs no network access, because every entry is a
self-contained SVG. From the repository root, serve only this directory:

```sh
python3 -m http.server 8767 --bind 127.0.0.1 --directory docs/design/documentation-guidance
```

Then open <http://127.0.0.1:8767/#discussion>.

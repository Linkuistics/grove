# Modular configuration visual document

These editable views accompany the [design](../../specs/modular-configuration.md).
`diagrams.json` groups the views and identifies the current discussion. Diagram
sources are Mermaid files; the bundled Grove design viewer uses Mermaid 12.0.0
from jsDelivr and needs network access for rendering.

From the repository root, serve only this directory:

```sh
python3 -m http.server 8766 --bind 127.0.0.1 --directory docs/design/modular-configuration
```

Open [the current discussion](http://127.0.0.1:8766/#discussion),
[command reuse](http://127.0.0.1:8766/#diagram-commands),
[composition](http://127.0.0.1:8766/#diagram-resolution), or
[validation and inspection](http://127.0.0.1:8766/#diagram-validation).
Reload after editing sources or the manifest. The text sources are authoritative;
there are no generated diagram exports to keep synchronized.

`index.html` is the static viewer supplied by the installed `grove-design`
skill, copied unchanged. The specification owns decisions; captions identify
the scope of each view and do not constitute a second decision log.

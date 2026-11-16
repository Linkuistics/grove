<!-- grove reference file — dated capability facts about documentation formats, renderers and viewers -->

# Documentation toolchain — dated capability facts

This file holds facts about formats, hosts, renderers and viewers. Each fact
carries the date it was checked. The file states no rule and makes no
recommendation. The rule that uses it is
`format-per-set-with-the-human`, in `DOCUMENT-CONTRACT-FORMAT.md` under
*Format and media*. Before a choice in the format part turns on one of these
facts, recheck the fact. Cite it with its date.

Unless a fact states otherwise, it was checked on **2026-09-30**. The sources
were official documentation, release pages, repository source, issue trackers,
and direct observation. "Observed" marks a fact that a run on macOS arm64, a
call to GitHub's REST API or a direct HTTP request showed on that date.

## Formats and hosts

| Format | On GitHub | Local preview (VS Code 1.139) | Published output | Without the tool |
|---|---|---|---|---|
| GFM | Full render: alerts, footnotes, Mermaid (11.17.2), SVG images by `<img>` | The built-in preview renders CommonMark through markdown-it. It has no GFM alerts, footnotes or Mermaid. | Depends on the site generator | Plain text |
| AsciiDoc | Secure-mode preview: includes become links, diagram blocks show source, no admonition icons. TOC, cross-references and callouts work. | Needs the AsciiDoc extension (4.2.2, 2026-08-19) | Asciidoctor HTML5 and PDF. Asciidoctor Diagram renders diagrams at build time. | Plain text. Asciidoctor Reducer output is one flattened `.adoc` that reads correctly on GitHub. |
| DocBook | XML source, not rendered | No built-in preview | xslTNG 2.8.5 HTML5 and print. It needs an XSLT 3.0 processor, in practice Saxon on Java. | Verbose XML |
| Typst | Source, syntax-highlighted. A committed PDF renders, with its links ignored. | Not built in | PDF, tagged by default. HTML is experimental. SVG and PNG have no machine-readable text. | Source only. A committed PDF stays readable. |

Facts behind the table, checked 2026-09-30:

- GitHub documents five alert types, footnotes and Mermaid rendering.
  Observed: all three render, and GitHub draws Mermaid client-side in a
  separate frame.
- GitHub runs Asciidoctor in secure mode. "There is no such thing as
  GitHub-flavored AsciiDoc." Includes, and so tagged regions, are disabled on
  GitHub.
- GFM has no include mechanism and no annotation syntax. AsciiDoc numbered
  callouts render in secure mode (observed).
- GFM heading anchors change when a heading is edited.
- Typst's HTML export is "still very incomplete". Its SVG text cannot be read
  by a screen reader: observed, the SVG has no `<text>` elements. PDF/UA-1 is
  the only PDF accessibility part it supports.
- Typst packages need an exact version.
- The OASIS DocBook technical committee closed in September 2024. The XSLT
  1.0 stylesheets last released in 2016 (1.79.2).
- Neither github/markup nor GitHub's list of rendered prose formats includes
  Typst or DocBook.
- GitHub embeds an SVG by `<img>`, so a screen reader receives only its
  `alt` text. GitHub states that not all Mermaid charts are accessible.

## Renderers

| Tool | Version, licence | SVG route | Legend | Source in SVG | Byte-stable across runs | SVG text |
|---|---|---|---|---|---|---|
| PlantUML | 1.2026.8 (2026-09-05), several licences | `plantuml -tsvg`. Needs Java, and Graphviz for several diagram kinds, else Smetana. | `legend … end legend` | Yes, a `plantuml-src` processing instruction | Yes (observed) | Real `<text>`. Title and description available. No ARIA. |
| C4-PlantUML | 2.14.0 (2026-08-26), MIT. PlantUML 1.2026.8's standard library ships 2.13.0. | As PlantUML | `SHOW_LEGEND()` | As PlantUML | Yes (observed) | As PlantUML |
| Mermaid | 12.0.0 (2026-09-10), MIT. mmdc 12.0.0 (2026-09-24). | `mmdc`. Needs Node 22.13 or later, and headless Chrome. | Pie and XY charts only. None for C4. | No | Yes with mmdc (observed) | Title and description with ARIA from `accTitle` and `accDescr`. Labels in `foreignObject` by default. |
| D2 | 0.9.0 (2026-09-07), MPL-2.0 | `d2 in.d2 out.svg`, one binary | `d2-legend` | No | Yes, all three layouts (observed) | Real `<text>`. Title only from tooltips. No ARIA. |
| Structurizr | 2026.09.19, Apache-2.0, open-core server | `export -format svg` through Playwright, or export to PlantUML or Mermaid | A generated key, in a separate file | No | No source found | Real `<text>`. No title or description. |
| LikeC4 | 1.59.4 (2026-09-21), MIT | None direct. It generates D2, Mermaid, PlantUML or DOT, which another tool then renders. | Experimental | Not applicable | No source found | PNG only, so no text |
| Kroki | 0.32.1 (2026-08-12), MIT | HTTP POST to a public or self-hosted instance | As the underlying tool | PlantUML-based types: yes | Same instance: yes. kroki.io ran PlantUML 1.2026.6. | As the underlying tool |
| Asciidoctor Diagram | 3.2.1 (2026-03-20), MIT | At build time, through local tools | As the underlying tool | As the underlying tool | No source found | As the underlying tool |
| Typst packages | Several, such as tiago 0.1.0 for D2 syntax | Inside Typst | Per package | Not applicable | Typst SVG: yes (observed) | Glyphs are paths, so no text |

Facts behind the table, checked 2026-09-30:

- Structurizr keeps one model with many views, C4 levels and a generated key.
  Manual layout lives only in `workspace.json`, which is not designed to be
  edited by hand. Prebuilt server binaries need a licence.
- Mermaid C4 is still experimental, and it has no legend.
- Mermaid 12.0.0 re-lays out and recolours existing flowcharts, state and
  class diagrams. ELK became its default layout.
- D2 0.9.0 SVG exports are about 24% smaller than 0.8, with unchanged
  appearance. D2 bundles the dagre, ELK and TALA layouts.
- LikeC4 has no SVG export.

## Viewers

### VS Code

- **Built-in preview.** Local resources load from all workspace folders when
  the file belongs to one. Otherwise, they load only from the file's own
  directory. Opening the containing folder therefore fixes a sibling-directory
  image, in Restricted Mode as well. Only VS Code's source documents this.
- **New Markdown Editor.** This optional, experimental editor loads no
  workspace images in Restricted Mode, not even from the file's own
  directory. Observed in the installed 1.139.1.
- **Links that start with `/`.** VS Code resolves them from the workspace
  root, and GitHub resolves them from the repository root. The two agree
  only when the opened folder is the repository root.
- **A request for a local read.** When requesting a local VS Code read,
  specify opening the containing documentation folder or repository root
  before the preview. (Checked 2026-09-29.)

### GitHub: SVG and relative links

- A relative link resolves from the current file. A link that starts with
  `/` resolves from the repository root.
- Observed: GitHub serves repository SVGs from `raw.githubusercontent.com` as
  `image/svg+xml`, under a sandboxing content security policy.
- An SVG embedded by `<img>` runs no scripts and loads no external
  resources, fonts included. Embedded fonts (D2, mmdc 12) therefore render.
  Referenced fonts (PlantUML, Structurizr) fall back.
- Observed: inline `<svg>` in Markdown is stripped to its text. GitHub
  rewrites external images through `camo.githubusercontent.com`.
- Known breakage: SVG release assets do not embed. SVGs do not show in the
  iOS app, and may not render in Firefox. Relative links inside tables, and
  parent-directory links, failed in reports to GitHub.
- `<picture>` with `prefers-color-scheme` is supported.

### What a session can check mechanically

| Property | Mechanical check | Needs the viewer |
|---|---|---|
| Local link, image and heading targets exist | VS Code's `markdown.validate.enabled`, or any script | — |
| GitHub's Markdown HTML: alerts, footnotes, sanitisation | `POST /markdown` with `mode=gfm` | — |
| Relative image paths resolve on GitHub | Not by the API, which returns relative `src` unchanged (observed) | Yes |
| GitHub's rendered Mermaid and its version | Only by inspecting the served bundle | Yes |
| AsciiDoc as GitHub previews it | `asciidoctor -S secure -a env-github`, with `-v --failure-level=INFO` for references (observed) | Yes |
| Typst references | `typst compile` fails on a missing label (observed) | — |
| DocBook structure | RELAX NG and Schematron validation | — |
| An SVG's text, title and embedded fonts | Parse the SVG (observed) | Yes: font fallback and screen-reader output |
| Served SVG content type and policy | `curl -I` on `raw.githubusercontent.com` (observed) | — |
| `<picture>` theme switching, Firefox and iOS gaps | — | Yes |
| VS Code webview image loading | Read the resource rule from source | Yes |

No primary source documents a headless check for GitHub's AsciiDoc HTML, VS
Code's webview, or GitHub's SVG acceptance.

## Pinning a regenerate-and-compare check

A pinned check regenerates each figure from its source and compares the
result with the committed file. Checked 2026-09-30, such a check needs these
choices:

- **Pin the tool and its runtime.**
  - PlantUML: the jar version, the Graphviz version or Smetana, and the Java
    runtime.
  - Mermaid: the mmdc version and the browser that Puppeteer resolves.
  - Structurizr: the image tag, and the locale and timezone unless its
    metadata line is disabled.
  - Homebrew installs whatever is current. A pin needs a versioned download,
    a container image, or `npx pkg@version`.
- **Pin the layout settings.** For D2, pin `--layout` and `--tala-seeds`. For
  Mermaid, pin `layout`, `theme` and `look`, because 12.0.0 changed all three
  defaults.
- **Decide what version stamps mean.** PlantUML's `<?plantuml VERSION?>` and
  D2's `data-d2-version` make every upgrade a byte difference. D2's
  `--omit-version` and PlantUML's `--disable-metadata` remove them.
  `--disable-metadata` also removes the embedded source. A check can compare
  bytes and expect a difference on upgrade, or compare after it normalises
  the stamps.
- **Fix timestamps** for PDF output with `SOURCE_DATE_EPOCH`.
- **Account for fonts.** Tools that embed fonts render the same in GitHub's
  `<img>`. Tools that reference fonts depend on the viewer.
- **Know the limit.** A byte comparison proves that the committed SVG
  matches its source under the pinned tool. It does not prove that the image
  renders where the reader reads it.

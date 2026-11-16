# Documentation toolchain: formats, diagram tools and reading environments, rechecked

**Checked 2026-09-30.** This survey rechecks the tool, format and host facts
that [#17], [#19], section 1 of the [#9 comment], the [#18 VS Code
comment][i18-k73] and survey [§8](documentation-basis-a.md#8-formats-diagram-notations-and-the-walk-away-check)
recorded on 2026-09-23 (2026-09-29 for the VS Code fact). Those facts are
inventoried as CF-01 to CF-40 in
[`documentation-evidence.md`](documentation-evidence.md#dated-capability-facts).
It answers the four question groups of the `documentation-toolchain-k6` brief:
format selection, figure tools, the reader's environment and regeneration
recipes.

**This is a fact survey, not a recommendation.** Choosing formats and tools is
the design leaf's job (`documentation-guidance-k7`). Where a fact bears on
existing guidance, this survey names the fact and the guidance it touches. It
does not say what the guidance should become.

## Scope and confidence

**Sources.** Every claim cites a primary source: official documentation, a
release page, repository source at a named path, an issue tracker, or a
direct observation made on 2026-09-30. Observations are labelled
**observed** and come from three places:

- the development machine this grove runs on (macOS, arm64);
- GitHub's REST API (`POST /markdown`, repository metadata, release lists);
- direct HTTP fetches of served assets, such as GitHub's Mermaid viewer bundle
  and `raw.githubusercontent.com` headers.

**Method.** The web rechecks ran as three delegated searches with the same
instructions: tool families (Structurizr, LikeC4, Kroki, Asciidoctor Diagram,
C4-PlantUML), renderers (PlantUML, Mermaid, D2, Typst packages), and formats
and viewers. Each was told to search first for breakage in real use. The
local regeneration experiments and the VS Code source inspection were run
directly. One search sent two toy diagrams containing no project content to
the public kroki.io service, to compare its output with local PlantUML.

Every cited URL returned HTTP 200 on 2026-09-30, except the two Structurizr
announcements on Patreon, which refuse scripted fetches with 403. The vNext
post's quoted text was captured by a browser fetch. The shutdown date is also
stated in Structurizr's own documentation, which is cited beside it.

**Confidence is uneven.**

- Version, licence and export-format facts are firm; they come from release
  APIs and source.
- Byte stability rests on this machine's runs, three per tool. No tool
  documents a guarantee. A stable run here is evidence about one machine, not
  a property of the tool.
- Accessibility rests on the SVG's structure (real `<text>`, `<title>`,
  `<desc>`, ARIA attributes). No screen reader was run. Structure is
  necessary for screen-reader access; it does not show that access works.
- What a reader sees on GitHub was established through the Markdown API and
  served assets, not by opening github.com in a browser. The limits are in
  [§4.4](#44-what-a-session-can-check-mechanically).

**One silence runs through the whole survey.** No tool examined here documents
a byte-stability guarantee for its SVG output. The local results below are
observations only.

## Findings at a glance

1. **Four 2026-09-23 facts were wrong or have changed; the rest hold.**
   - **CF-02:** PlantUML's offline standard library still ships C4-PlantUML
     2.13.0, not 2.14.0.
   - **CF-13:** Typst has a D2-syntax package, `tiago`, which is a third-party
     re-implementation.
   - **CF-20:** PlantUML documents `svgTitle` and `svgDesc` for accessible
     SVG titles.
   - **CF-27:** DocBook moved to Codeberg; the current versions are 5.2.1 and
     xslTNG 2.8.5.

   Two more carry a corrected detail. **CF-25:** Asciidoctor reports invalid
   references at INFO, not WARNING, so `--failure-level=WARN` does not catch
   them. **CF-09:** Java 21 or later is available on this machine, but not on
   `PATH`. [§1](#1-status-of-the-2026-09-23-facts) has every verdict.
2. **GitHub renders Mermaid with 11.17.2 (observed), not 12.** Mermaid 12.0.0
   re-lays out and recolours existing flowcharts, state and class diagrams. A
   source pinned to 12.0.0, as Grove's design viewer is, therefore draws
   differently on GitHub. GitHub publishes no version statement.
3. **Structurizr's cloud service ends today with no hosted successor.** The
   free, open-source commands (`local`, `export`, `playground`) cover
   authoring and SVG export. Prebuilt `server` binaries need a paid licence.
   Its SVG export needs a separate Playwright build, and the export page
   contradicts the binaries page about whether that build exists.
4. **Every renderer that is installed or runnable here produced byte-identical
   SVG across three runs of one pinned version.** That covers PlantUML, D2
   (dagre, ELK and TALA), Mermaid CLI and Typst. Every one also stamps its
   version or depends on an unpinned runtime:
   - PlantUML's `<?plantuml VERSION?>` and the Graphviz it calls;
   - D2's `data-d2-version`;
   - Mermaid's headless Chrome;
   - Typst PDF's creation timestamp.

   So a regenerate-and-compare check must pin more than the tool.
5. **The VS Code preview rule of CF-40 still holds.** A new experimental
   Markdown Editor loads no workspace images at all in Restricted Mode.
6. **Several properties can only be seen by opening the viewer.** GitHub's
   rendered Mermaid, `<picture>` theme switching, GitHub's final AsciiDoc HTML
   and VS Code's webview image loading are among them. GitHub's Markdown API
   does not resolve relative image paths.

## 1. Status of the 2026-09-23 facts

Each fact is marked **unchanged**, **changed** (with the current fact) or
**unverifiable**. The fact's wording is in
[`documentation-evidence.md`](documentation-evidence.md#dated-capability-facts).

### 1.1 Diagram tools

| ID | Verdict | Current fact (2026-09-30) |
|---|---|---|
| CF-01 | Unchanged, with a correction | One model, many views, C4 levels and a generated key are confirmed ([Structurizr][structurizr-home]; [notation][structurizr-notation]). Manual layout lives only in `workspace.json`, "not designed to be edited by hand" ([file types][structurizr-filetypes]). PNG/SVG export drives headless Chromium through Playwright ([PNG and SVG][structurizr-pngsvg]). **Correction:** the repository is Apache-2.0, but the server is now open core, and prebuilt `server` binaries need a licence ([binaries][structurizr-binaries]; [vNext post][structurizr-vnext]; [pricing][structurizr-pricing]). |
| CF-02 | **Changed** | C4-PlantUML v2.14.0 (2026-08-26, MIT) is the latest release ([releases][c4plantuml-releases]). PlantUML 1.2026.8's standard library ships **2.13.0** (**observed**: `plantuml -stdlib`), so offline `!include <C4/…>` gives 2.13.0. The diagram types and `SHOW_LEGEND()` are unchanged ([README][c4plantuml]). |
| CF-03 | Unchanged | Mermaid C4 is still "an experimental diagram … syntax and properties can change", with five types; legend is among the unsupported features ([C4 syntax][mermaid-c4]). `mmdc` 12.0.0 needs Node ≥ 22.13 and Puppeteer 25, which drives headless Chrome ([mermaid-cli 12.0.0][mmdc-12]). GitHub renders Mermaid natively; see CF-17 for its version. |
| CF-04 | Unchanged | D2 v0.9.0 (2026-09-07, MPL-2.0) is still the latest release; the repository is now `d2lang/d2` ([v0.9.0][d2-090]). The C4 theme (ID 303), `c4-person`, `layers` with `link`, and `d2-legend` date from 0.7 ([C4 post][d2-c4]; [legend][d2-legend]). SVG is the default output, and dagre, ELK and TALA are all bundled (**observed**: `d2 layout`). |
| CF-05 | Unchanged, with an addition | LikeC4 v1.59.4 (2026-09-21), MIT, allows "any number of nested levels" ([README][likec4]). It still has **no SVG export**. The maintainer says "SVG as export format is not supported" ([likec4#1429][likec4-1429]), and the CLI registers only png, jpg, json, drawio and markdown exports. **Addition:** `export png`/`jpg` screenshots views through Playwright, with a `--notation` flag that adds the key ([CLI][likec4-cli]). Notations remain "Experimental" ([notations][likec4-notations]). |
| CF-06 | Unchanged | `structurizr/lite`, `cli` and `onpremises` are archived, and so are `ui`, `java` and `puppeteer`. `structurizr/structurizr` is live and Apache-2.0 (GitHub API). The end-of-life page names each replacement: `local` for Lite, `pull`/`push`/`export` for the CLI, and `server` for on-premises ([EOL][structurizr-eol]). Precision: `lite` and `onpremises` were MIT-licensed. |
| CF-07 | Unchanged | `export` formats are plantuml (Structurizr and C4-PlantUML variants), mermaid, websequencediagrams, static, png, svg, json, theme and custom exporters ([export][structurizr-export]). There is no D2, DOT or Ilograph. DOT and Ilograph survive only as custom-exporter classes, and there is no D2 exporter at all ([ExportCommand.java][structurizr-exportcmd]). |
| CF-08 | Unchanged | The documentation banner reads "The Structurizr cloud service will reach its End of Life (EOL) on 30 September 2026" ([cloud migration][structurizr-cloud]; [announcement][structurizr-cloud-eol]). The EOL page lists the cloud service with "(no replacement)" ([EOL][structurizr-eol]). On 2026-09-30, structurizr.com still answered, with an end-of-life banner. |
| CF-09 | Unchanged requirement; **changed** local fact | The `.war` "Requires Java 21"; the Docker image uses Temurin 21 ([binaries][structurizr-binaries]). **Observed:** `java` on `PATH` is 16.0.1, but Homebrew's OpenJDK 27 is at `/opt/homebrew/opt/openjdk`, and Docker 28.1.1 is running. The requirement can be met here by invoking either explicitly. |
| CF-10 | Unchanged, with a caveat | Asciidoctor Diagram 3.2.1 (2026-03-20, MIT) renders PlantUML, Mermaid, D2, Structurizr and others with local tools ([docs][asciidoctor-diagram]). **Caveat:** its Structurizr support still loads the end-of-life CLI's jars ([Structurizr page][ad-structurizr]; open [#513][ad-513]). Each recent PlantUML release has needed a compatibility fix ([CHANGELOG][ad-changelog]). |
| CF-11 | Unchanged | Kroki v0.32.1 (2026-08-12), MIT, can be self-hosted ([Kroki][kroki]; [install][kroki-install]). kroki.io still has no privacy statement: `/privacy` and `/terms` return 404, and "Write and publish privacy policy" has been open since 2021 ([kroki.io#41][kroki-41]). The maintainer says "the public instance doesn't have SLA" ([kroki#1835][kroki-1835]). LikeC4 is not supported ([kroki#2046][kroki-2046]). |
| CF-12 | Unchanged | GitHub previews AsciiDoc without diagrams: "On GitLab, you can find support for include directives, admonition icons, diagrams … GitHub, on the other hand, provides a table of contents browser" ([Asciidoctor tooling][asciidoctor-tooling]). **Observed:** a `[plantuml]` block rendered at GitHub's settings came out as plain source. |
| CF-13 | **Changed** (already wrong on 2026-09-23) | The Typst package index has **`tiago`** 0.1.0 (2026-03-31, Apache-2.0): "Draw diagrams in Typst via Diago, a D2 implementation in MoonBit". It is a third-party re-implementation with dagre and ELK layouts and no TALA, it says "some features are not properly supported", and its demos include a C4 diagram ([tiago][typst-tiago]). There is still no PlantUML package and no dedicated C4 package ([index][typst-index]). fletcher, CeTZ, diagraph, pintorita, chronos, digidraw, merman and mmdr are all present. |
| CF-14 | Unchanged in substance | [typst#1421][typst-1421] was closed as "not planned" (2024-09-10). Typst now warns when an embedded SVG contains `foreignObject` ([typst#4932][typst-4932]). Mermaid's `htmlLabels` still defaults to true. **Observed:** a default Mermaid 12 flowchart put its labels in three `foreignObject` elements and had no `<text>`. The thread's `mmdc --pdfFit` workaround no longer exists in mmdc 12 ([mermaid-cli 12.0.0][mmdc-12]). |
| CF-15 | Unchanged | PlantUML writes the source into every SVG as a `<?plantuml-src …?>` processing instruction. `--extract-source` recovers it, and `--disable-metadata` (legacy `-nometadata`) turns it off ([command line][plantuml-cli]). **Observed** with 1.2026.8: all three work. The single-dash `-extract-source` reported "No diagram found". |
| CF-16 | Unchanged, one cause removed | D2 SVGs still carry a `<style>` block with theme CSS and embedded fonts. The maintainer says viewers without CSS support are out of scope ([d2#1273][d2-1273]). Since 0.9.0, Markdown labels are native SVG rather than HTML in `foreignObject` ([v0.9.0][d2-090]). **Observed:** three `@font-face` rules with WOFF data URIs. |
| CF-17 | Unchanged (confirmed) | Mermaid 12.0.0: "Existing flowcharts, state and class diagrams will re-lay out and recolour"; ELK is now the default layout ([12.0.0 notes][mermaid-12]). Several ELK-default regressions are fixed on `develop` but unreleased, including [#8292][mermaid-8292] and [#8329][mermaid-8329]. |
| CF-18 | Unchanged (confirmed) | D2 0.9.0: SVG exports are "approximately 24% smaller across the E2E corpus and 18% smaller across real-world fixtures, with unchanged appearance" ([v0.9.0][d2-090]). |
| CF-19 | Unchanged (confirmed) | TALA became open source on 2026-09-07 under MPL-2.0 and is bundled with D2 0.9.0 ([announcement][d2-tala-oss]). The TALA docs say it "Has randomness. A small change to a label can cascade into an entirely different layout" ([TALA][d2-tala]). 0.9.0 adds `--tala-seeds`. |
| CF-20 | **Changed** | Mermaid still documents `accTitle`/`accDescr` for "all diagrams/chart types" ([accessibility][mermaid-a11y]). **PlantUML now documents** `svgTitle` and `svgDesc`, which write SVG `<title>` and `<desc>` ([SVG][plantuml-svg]), added in V1.2025.7 "for accessibility" ([plantuml#2292][plantuml-2292]). D2 still has no accessibility documentation; its tooltips become `<title>` elements ([interactive][d2-interactive]). |

### 1.2 Document formats and hosts

| ID | Verdict | Current fact (2026-09-30) |
|---|---|---|
| CF-21 | Unchanged | GitHub documents five alert types, footnotes and Mermaid rendering ([basic syntax][gh-basic]; [diagrams][gh-diagrams]). **Observed** via `POST /markdown`: all three render, and Mermaid is drawn client-side in a `viewscreen.githubusercontent.com` frame. github/markup calls Asciidoctor with `:safe => :secure` ([markups.rb][gh-markups]). **Observed** with those settings locally: includes became links, diagram blocks showed source, and TOC, cross-references, callouts and captions worked. DocBook and Typst appear as source; PDFs render, but "links within PDFs are ignored" ([non-code files][gh-noncode]). |
| CF-22 | Unchanged | GFM has no include mechanism ([spec][gfm-spec]). AsciiDoc tagged regions ([tagged regions][asciidoc-tags]) are disabled on GitHub because includes are. xslTNG's XInclude supports RFC 5147 fragments and a `search` scheme ([xslTNG guide][xsltng-guide]). Typst `read()` takes no line range ([read][typst-read]), and `raw()` has no region parameter ([raw][typst-raw]). |
| CF-23 | Unchanged for AsciiDoc and DocBook; unverifiable for Markdown and Typst | **Observed:** AsciiDoc numbered callouts render in secure mode. DocBook 5.2.1's notes correct "Schematron rules related to `callout` cross references" ([DocBook releases][docbook-releases]). GFM has no annotation syntax in its spec ([spec][gfm-spec]). No search was made for current Typst packages keyed to line numbers. |
| CF-24 | Unchanged for GFM and AsciiDoc; unverifiable for DocBook and Typst | "Five types of alerts are available", with fixed titles ([basic syntax][gh-basic]). AsciiDoc has "five admonition types", with captions set by `<type>-caption` ([admonitions][asciidoc-admonitions]). DocBook's element set and Typst's admonition packages were not rechecked. |
| CF-25 | **Changed in detail** | GFM heading anchors change when a heading is edited, and custom anchors are "not included in the document outline" and unchecked ([custom anchors][gh-anchors]). **Asciidoctor:** the docs still show `WARNING: invalid reference: foobar` under `-v` ([xref validation][asciidoc-xref]). **Observed** with 2.0.26, `-v` logs `INFO: possible invalid reference: foobar`, so only `--failure-level=INFO` fails the build. **Observed** with Typst 0.15.1, a missing label is a compile error. DocBook's Schematron checks are unchanged. |
| CF-26 | Unverifiable | Glossary and index support was not rechecked in any format. |
| CF-27 | **Changed** for DocBook | Asciidoctor 2.0.26 (2025-10-24) and Typst 0.15.1 (2026-07-17) are unchanged. **DocBook moved to Codeberg**: the schema is 5.2.1 (2026-03-19) and xslTNG is 2.8.5 (2026-09-27) ([DocBook releases][docbook-releases]; [xslTNG releases][xsltng-releases]). The GitHub repositories stop at "Moved to Codeberg". **Addition:** Typst 0.15.0 added an experimental `bundle` export (multi-file output) and MathML in HTML ([Typst 0.15.0][typst-0150]). Asciidoctor PDF is 2.3.27 (2026-09-12). |
| CF-28 | Unverifiable (a judgement) | The legibility ranking was the survey's judgement, not a measurement, and was not re-judged. |
| CF-29 | Unchanged | "Version 0.29-gfm (2019-04-06)": "GFM is a strict superset of CommonMark", with the five listed extensions ([spec][gfm-spec]). Alerts, footnotes and Mermaid are GitHub features outside the spec. |
| CF-30 | Unchanged | Linguist still requires "at least 2000 files per extension or filename indexed in the last year" ([CONTRIBUTING][linguist-contrib]). Linguist governs highlighting, not rendering: Typst is in Linguist and is highlighted but not rendered ([languages.yml][linguist-languages]). |
| CF-31 | Unchanged | "GitHub processes AsciiDoc files using the SECURE mode" ([safe modes][asciidoctor-safe]). "There is no such thing as GitHub-flavored AsciiDoc" ([asciidoctor.org#939][asciidoctor-939]). Asciidoctor Reducer v1.1.2 (2025-05-09) is "ready-made to solve this problem" ([Reducer][asciidoctor-reducer]). The latest specification tag is still 0.3.0, a milestone build from 2024-02-29 ([asciidoc-lang][asciidoc-lang]). Until ratification, "AsciiDoc is defined by the Asciidoctor implementation" ([docs][asciidoc-docs]). |
| CF-32 | Unchanged, with an addition | The OASIS TC closed in September 2024 ([docbook.org][docbook-org]; [OASIS page][docbook-oasis]). The XSLT 1.0 stylesheets' last release is 1.79.2 (2016-12-09). xslTNG runs on "any conformant XSLT 3.0 processor … that means Saxon … Saxon 10.1 or later is recommended", and ships a `java -jar` runner ([xslTNG guide][xsltng-guide]). **Addition:** development continues on Codeberg (CF-27). |
| CF-33 | Unchanged | HTML export is "still very incomplete" and "Do not use this feature for production use cases" ([HTML][typst-html]). SVG text "cannot be extracted automatically, for example by copy/paste or a screen reader" ([SVG][typst-svg]); **observed:** zero `<text>` elements, with glyphs drawn as paths. PDF is tagged by default, and PDF/UA-1 is the only UA part supported ([PDF][typst-pdf]). Packages need an exact version. Immutability is a policy: "will not be changed or removed without good reason" ([packages][typst-packages]). |
| CF-34 | Unverifiable (a practice claim) | That repository review means reading raw markup comes from [Moir 2024][moir-2024]. It is not a tool capability and was not rechecked. |
| CF-35 | Unchanged | Neither github/markup's list ([markup][gh-markup]) nor GitHub's prose-rendering list ([non-code files][gh-noncode]) includes Typst or DocBook. |

### 1.3 Local installation (this machine, observed 2026-09-30)

The 2026-09-23 survey did not identify its machine. The observations below
are this grove's machine; the versions match CF-36 to CF-39.

| ID | Verdict | Observed |
|---|---|---|
| CF-36 | Unchanged | PlantUML 1.2026.8 (Homebrew) with Graphviz 16.1.0. The wrapper runs Homebrew's OpenJDK 27, not the `PATH` Java. |
| CF-37 | Unchanged | D2 0.9.0, with dagre, ELK and TALA all bundled. |
| CF-38 | Unchanged | No `mmdc` on `PATH`. `npx -y @mermaid-js/mermaid-cli@12.0.0` runs it without a global install, using the installed Google Chrome 154.0.8037.58. Typst 0.15.1. |
| CF-39 | Unchanged | Asciidoctor 2.0.26 and Asciidoctor PDF 2.3.15; upstream PDF is 2.3.27. Neither `asciidoctor-diagram` nor `asciidoctor-reducer` is installed. |

Also absent: Structurizr, LikeC4, Kroki and Saxon. Also present: `xsltproc`
(XSLT 1.0 only), Docker 28.1.1, Node 26.10.0, VS Code 1.139.1 and go-task
3.53.1.

### 1.4 Viewing environment

| ID | Verdict | Current fact |
|---|---|---|
| CF-40 | Unchanged, with an addition | Upstream `getMarkdownLocalResourceRoots` adds all workspace folders when the file is in one, and otherwise only the file's own directory ([resources.ts][vscode-resources]). The built-in preview calls it without a trust condition. **Observed:** the installed 1.139.1 extension bundles the same logic. **Addition:** see [§4.1](#41-vs-code) for the new Markdown Editor, which in Restricted Mode drops workspace roots altogether ([markdownEditorProvider.ts][vscode-mdeditor]). |

### 1.5 Dated statements outside the CF inventory

- **#17's table** says C4-PlantUML renders through "PlantUML `--svg` (Java,
  Graphviz)". **Changed:** since V1.2026.7, PlantUML falls back to the Smetana
  layout when `dot` is unavailable ([commit][plantuml-smetana-fallback]).
  Layout can therefore depend on whether Graphviz is installed; this is an
  inference from the changelog, not observed. The same release also moved
  the default sequence engine from Puma to Teoz ([commit][plantuml-teoz]).
- **Survey §8.2** marks LikeC4 "Not checked" locally. It is still not
  installed.
- **Survey §8.2** has Kroki reaching C4 "through C4-PlantUML, Structurizr and
  D2". Unchanged ([Kroki][kroki]).
- **The [#9 comment] §1** makes the Structurizr points that became DS-01. All
  are rechecked in CF-06 to CF-08 above.
- **`visual-design-discussion.md`** pins D2 0.9.0 and Mermaid 12.0.0. Both
  are still the latest releases, and the jsDelivr URL returns HTTP 200. Two
  current facts touch that text:
  - GitHub renders Mermaid 11.17.2 ([§3.2](#32-per-tool-detail));
  - `--layout=tala` needs no separate TALA install, because TALA is bundled
    and open source (CF-19).

## 2. Format selection

What the intended reader sees in each of the three places, and what is left
without the tool.

| Format | On GitHub | Local editor preview (VS Code 1.139) | Published site | Tool uninstalled |
|---|---|---|---|---|
| GFM | Full render: alerts, footnotes, Mermaid (11.17.2), SVG images by `<img>` | Built-in preview renders CommonMark via markdown-it. It does **not** support GFM: no alerts, footnotes or Mermaid ([VS Code Markdown][vscode-md]) | Depends on the site generator (not surveyed) | Plain text; GitHub renders it with no local tool |
| AsciiDoc | Secure-mode preview: includes become links, diagram blocks show source, no admonition icons; TOC, cross-references and callouts work | Needs the AsciiDoc extension (4.2.2, 2026-08-19); its preview loads local files only from workspace folders, or the file's folder when outside one ([preview][asciidoc-vscode-preview]) | Asciidoctor HTML5 and PDF; Asciidoctor Diagram renders diagrams at build time | Plain text; Reducer output is one flattened `.adoc` that reads correctly on GitHub |
| DocBook | XML source (no Linguist entry) | No built-in preview (not searched further) | xslTNG 2.8.5 HTML5 and print; needs an XSLT 3.0 processor, in practice Saxon on Java | Verbose XML; committed build output stays readable |
| Typst | Source, syntax-highlighted; committed PDFs render with links ignored | Not built in; the Tinymist extension (0.15.8) exists, and its preview was not examined | PDF (tagged by default); HTML experimental; SVG/PNG without machine-readable text | Source only; a committed PDF stays readable |

**Changed cells of the 2026-09-23 table ([#19]).**

- The DocBook version column: 5.2 becomes 5.2.1, and xslTNG is 2.8.5.
- AsciiDoc's cross-reference cell: invalid references are reported at INFO
  under `-v`.
- Typst's outputs cell gains an experimental `bundle` target.

No other cell changed. The "local editor preview" column is new to this
recheck. Its main fact is that VS Code's built-in preview is not a GFM
renderer, so a GFM document previewed locally loses its alerts, footnotes
and Mermaid.

**Accessibility of each format's output.**

- **GitHub SVG images:** they are embedded by `<img>`, so a screen reader
  gets only the `alt` text. Descendants of an image role are presentational
  ([MDN img role][mdn-img-role]).
- **GitHub Mermaid:** GitHub's own docs say "Not all charts are a11y
  compliant" ([known issues][gh-noncode-known]).
- **Typst:** only its PDF output carries readable text. SVG and PNG do not
  ([SVG][typst-svg]; [PNG][typst-png]).

## 3. Figure tools

### 3.1 Summary

| Tool | Version, licence | SVG route | Legend | Source in SVG | Stable across runs here | SVG text |
|---|---|---|---|---|---|---|
| PlantUML | 1.2026.8 (2026-09-05); jars under GPL, LGPL, Apache, BSD, EPL, MIT ([download][plantuml-download]) | `plantuml -tsvg`; Java; Graphviz for several diagram kinds, else Smetana | `legend … end legend` ([commons][plantuml-commons]) | Yes, `plantuml-src` PI | Yes (observed) | Real `<text>`; `svgTitle`/`svgDesc` available; no ARIA |
| C4-PlantUML | 2.14.0 (2026-08-26), MIT; stdlib 2.13.0 | As PlantUML | `SHOW_LEGEND()` | As PlantUML | Yes (observed) | As PlantUML |
| Mermaid | 12.0.0 (2026-09-10), MIT; mmdc 12.0.0 (2026-09-24) | `mmdc`; Node ≥ 22.13 and headless Chrome | Pie and XY charts only; none for C4 ([#2110][mermaid-2110]) | No (no source found) | Yes with mmdc (observed); library IDs date-based by default | `<title>`/`<desc>` with ARIA from `accTitle`/`accDescr`; labels in `foreignObject` by default |
| D2 | 0.9.0 (2026-09-07), MPL-2.0 | `d2 in.d2 out.svg`; one binary | `d2-legend` | No | Yes, all three layouts (observed) | Real `<text>`; `<title>` only from tooltips; no ARIA |
| Structurizr | app 2026.09.19, Apache-2.0 (open-core server) | `export -format svg` via a Playwright build, or export to PlantUML/Mermaid | Generated key, written as a separate file | No | No source found; a date line and locale make it unlikely by default | Real `<text>`; no `<title>`/`<desc>` |
| LikeC4 | 1.59.4 (2026-09-21), MIT | None direct; `gen d2`/`mermaid`/`plantuml`/`dot`, then render | Experimental notations; `--notation` on PNG | Not applicable | No source found | PNG only: no text |
| Kroki | 0.32.1 (2026-08-12), MIT | HTTP POST to a public or self-hosted instance | As the underlying tool; Structurizr key via `output=legend` | PlantUML-based types: yes (`no-metadata` removes it) | Same instance: yes; kroki.io runs PlantUML 1.2026.6, so it differs from 1.2026.8 | As the underlying tool |
| Asciidoctor Diagram | 3.2.1 (2026-03-20), MIT | At build time, via local tools | As the underlying tool | As the underlying tool | No source found | As the underlying tool |
| Typst packages | tiago 0.1.0 (D2 syntax), merman 0.3.0 and mmdr 0.2.2 (Mermaid), others | Inside Typst | Per package | Not applicable | Typst SVG yes (observed) | SVG glyphs are paths: no text |

Sources are in [§3.2](#32-per-tool-detail); the local runs are in
[§5.2](#52-local-byte-stability-runs).

### 3.2 Per-tool detail

**PlantUML and C4-PlantUML.**

- **Licence:** jars are published under several licences, and "the LGPL
  version is available without embedded GraphViz" ([download][plantuml-download]).
- **Graphviz:** needed for use-case, class, object, component, deployment,
  state and legacy activity diagrams. `!pragma layout smetana` and
  `!pragma layout elk` are alternatives ([Graphviz][plantuml-graphviz]).
- **Deterministic IDs:** element IDs are seeded from the source. The SVG's
  first child is `<?plantuml VERSION?>`, so bytes change on every upgrade
  ([SvgGraphics.java][plantuml-svggraphics]).
- **Accessibility:** a request for accessible SVGs has been open since 2019
  ([plantuml#243][plantuml-243]). **Observed:** the C4 diagram had 44
  `<text>` elements, and a sequence diagram's `<title>` elements named its
  participants.
- **Walk-away:** the `.puml` text stays legible. So does the committed SVG,
  whose text is real and whose source can be extracted again.
- **Breakage in real use:**
  - C4-PlantUML 2.12.0 changed layout, and `!NO_LAY_ROTATE=1` restores it
    ([C4-PlantUML#376][c4plantuml-376]).
  - Asciidoctor Diagram needed fixes for PlantUML 1.2025.9, 1.2026.1 and
    1.2026.2 ([CHANGELOG][ad-changelog]).

**Mermaid.**

- **Stable IDs:** the configuration schema says that with `deterministicIds`
  false, "the IDs are generated based on the current date and thus are not
  deterministic. This is the default behavior"
  ([config schema][mermaid-config-schema]). Stable IDs are broken even when
  enabled, per reports on [#6530][mermaid-6530] and [#1601][mermaid-1601].
  mmdc renders with a fixed root ID, `my-svg`, and **observed** output was
  byte-identical across three runs. That result belongs to mmdc; the library
  offers no such guarantee.
- **mmdc 12 changes:** it "embeds fonts into SVG files by default"
  (`--no-font-embed` disables it) and replaces `--width`/`--height` with
  `--size` ([mermaid-cli 12.0.0][mmdc-12]).
- **Accessibility:** **observed**, a flowchart with `accTitle`/`accDescr`
  produced `<title>`, `<desc>`, `aria-labelledby`, `aria-describedby` and
  `role="graphics-document document"`. Its labels were in `foreignObject`.
  With `htmlLabels: false`, the three labels became `<text>`. Open
  screen-reader issues: [#2395][mermaid-2395] and [#5632][mermaid-5632].
- **Host gap:** GitHub's Mermaid viewer bundle contains
  `renderer.draw(e,t,"11.17.2",b)` and `securityLevel:"antiscript"`
  (**observed**, [bundle][gh-mermaid-bundle]). GitHub's docs give no version
  and say to render an `info` block to find it ([diagrams][gh-diagrams]).
  ELK on GitHub is an open request ([discussion 138426][gh-elk-discussion]).
- **Breakage in real use:**
  - 11.17.0 moved C4 to a new shape renderer, and long labels stopped
    wrapping until 11.17.1 ([11.17.1 notes][mermaid-11171]).
  - 12.0.0 re-lays out existing diagrams (CF-17).
- **Walk-away:** the `.mmd` source is needed, because the SVG does not embed
  it. `foreignObject` labels survive only in HTML-capable viewers.

**D2.**

- **Stable output:** IDs are a hash of the diagram. The SVG carries
  `data-d2-version` unless `--omit-version` is set, a flag added because every
  upgrade otherwise changed every image ([d2#2377][d2-2377]).
- **Layout drift:** 0.8.2 "intentionally changes node ordering and
  coordinates" and tells users to "regenerate and review stored SVG"
  ([v0.8.2][d2-082]).
- **TALA:** it runs several seeds and "selects the best result
  deterministically" ([TALA README][d2-tala-readme]). **Observed:** three TALA
  runs were byte-identical.
- **Accessibility:** no D2 accessibility documentation was found. The only
  mention of accessibility in its docs is a roadmap item to "increase contrast
  for accessibility". No ARIA attributes appear in the renderer.
  **Observed:** real `<text>` for every label, the legend and the title.
- **Walk-away:** the SVG is self-contained, with embedded fonts and native
  text, but viewers without CSS support may misrender it. The source is not
  recoverable from the SVG.

**Structurizr.**

- **Export build:** SVG export was added in 2026.04.19
  ([release][structurizr-20260419]). Its documentation page still says the
  feature is available only from source or a preview `.war`
  ([PNG and SVG][structurizr-pngsvg]). The binaries page, however, offers
  `2026.09.19-playwright` Docker and `.war` builds for it
  ([binaries][structurizr-binaries]).
- **Stable output:** exports render a last-modified line from
  `toLocaleString` unless `structurizr.metadata` is false
  ([ui#53][structurizr-ui53]). The Open Sans font is referenced, not embedded
  ([ui#52][structurizr-ui52]).
- **Auto-layout:** the new application dropped Graphviz layout for Dagre
  ([vNext post][structurizr-vnext]). Dagre fails on some deployment views, and
  the author was "yet to find a resolution" on 2026-09-17
  ([structurizr#97][structurizr-97]).
- **Walk-away:** the DSL is readable text. Manual layout is locked in JSON.
  Exported PlantUML or Mermaid text can be re-rendered by those tools.

**LikeC4.**

- **Breakage in real use:**
  - PNG export failed after an upgrade ([#1885][likec4-1885]).
  - `--theme light` produced dark PNGs ([#3181][likec4-3181]).
  - Manual layout edits get lost (open, [#2975][likec4-2975]).
  - The viewer did not announce node names to screen readers (fixed in
    2026-06, [#2988][likec4-2988]).
- **Walk-away:** the `.c4` source is legible. Committed PNGs are raster, and
  generated D2, Mermaid or PlantUML text can still be re-rendered.

**Kroki.** The public instance upgrades without notice: it ran PlantUML
1.2026.6 (**observed**) and its SVG differed from local 1.2026.8 for the same
source. Two identical requests returned identical bytes. Its Structurizr
rendering ignores manual layout ([kroki#1758][kroki-1758]) and does not match
Structurizr's own look ([kroki#1775][kroki-1775]). **Walk-away:** the source
stays with the author, but rendering depends on a third party with neither a
service-level agreement nor a privacy policy.

**Asciidoctor Diagram.**

- It "will invoke the necessary external diagram rendering tools" at build
  time ([docs][asciidoctor-diagram]).
- Its Structurizr path requires the archived CLI and Java 17
  ([Structurizr page][ad-structurizr]).
- Whether its Structurizr-to-D2 route still works after the CLI dropped D2
  ([asciidoctor-diagram#494][ad-494]) is unverified.
- **Walk-away:** the diagram source is inline in the `.adoc`. On GitHub it
  shows as source (CF-12).

**Typst diagram packages.** Packages are pinned by exact version. Typst SVG
export makes all text into paths, so an SVG exported from Typst is
inaccessible whatever package drew it ([SVG][typst-svg]).

## 4. The reader's environment

### 4.1 VS Code

- **Built-in preview.** Local resources load from all workspace folders when
  the file belongs to one, and otherwise only from the file's own directory
  ([resources.ts][vscode-resources]). Opening the containing folder therefore
  fixes a sibling-directory image, in Restricted Mode as well. This is
  documented only in source.
  - The user documentation covers the other security settings: scripts are
    disabled, only `https` resources load, and three security levels are
    available ([VS Code Markdown][vscode-md]).
  - The Workspace Trust page does not mention the Markdown preview
    ([workspace trust][vscode-trust]).
- **New Markdown Editor.** `vscode.markdown.editor` is an optional,
  experimental custom editor. In an untrusted workspace it asks whether to
  open the file anyway, then passes
  `includeWorkspaceResources: vscode.workspace.isTrusted`
  ([markdownEditorProvider.ts][vscode-mdeditor]). The result is that **in
  Restricted Mode it loads no workspace images, not even from the file's own
  directory**.
  - **Observed:** the installed 1.139.1 bundle registers both custom editors,
    "Markdown Preview" and "Markdown Editor", and contains the same
    trust-gated call.
- **Links starting with `/`.** VS Code resolves them relative to the
  workspace root ([VS Code Markdown][vscode-md]); GitHub resolves them to the
  repository root ([relative links][gh-relative]). The two agree only when the
  opened folder is the repository root.

### 4.2 GitHub: images, SVG and relative links

- **Relative links.** "The path of the link will be relative to the current
  file. Links starting with `/` will be relative to the repository root. You
  can use all relative link operands, such as `./` and `../`"
  ([relative links][gh-relative]).
- **How repository SVGs are served.** **Observed:** they come from
  `raw.githubusercontent.com` as `image/svg+xml`, with CSP
  `default-src 'none'; style-src 'unsafe-inline'; sandbox`.
  - Embedded by `<img>`, an SVG runs no scripts and loads no external
    resources, fonts included, unless inlined as `data:` URLs
    ([MDN SVG as an image][mdn-svg-image]). So D2's and Mermaid CLI's embedded
    fonts render, while PlantUML's and Structurizr's referenced fonts fall
    back.
  - "If you are using the Firefox browser, SVGs on GitHub may not render"
    ([non-code files][gh-noncode]).
- **Inline and external SVG.** **Observed:** inline `<svg>` in Markdown is
  stripped, leaving only its text. External images are rewritten to
  `camo.githubusercontent.com`.
- **Breakage in real use:**
  - SVG release assets are served as `application/octet-stream` and do not
    embed ([discussion 59781][gh-59781]).
  - SVG fonts fall back on GitHub ([discussion 58335][gh-58335]).
  - SVGs do not show in the iOS app ([discussion 4103][gh-4103]).
  - Relative links inside tables broke, and the report was closed as stale
    without a fix ([markup#1773][gh-markup-1773]).
  - Relative links inside non-paragraph blocks on the repository home page
    are still open ([discussion 67750][gh-67750]).
  - Parent-directory `[..](..)` links fail ([discussion 13849][gh-13849]).
- **Theme-dependent images.** `<picture>` with `prefers-color-scheme` is
  supported ([quickstart][gh-quickstart]). No primary source was found on
  whether `#gh-dark-mode-only` is deprecated.

### 4.3 Accessibility across the route

A committed SVG that GitHub embeds reaches a screen reader only as its `alt`
text, whatever `<title>` or `<desc>` it contains ([MDN img role][mdn-img-role]).
Titles and descriptions inside the SVG therefore help when the SVG is opened
directly or inlined, not when GitHub shows it through `<img>`. This is an
inference from the MDN rule and GitHub's `<img>` embedding; it was not tested
with a screen reader.

### 4.4 What a session can check mechanically

| Property | Mechanical check | Needs the viewer |
|---|---|---|
| Local link, image and heading targets exist | VS Code's `markdown.validate.enabled` checks local links, not `http(s)` ([VS Code Markdown][vscode-md]), or any script | — |
| GitHub's Markdown HTML: alerts, footnotes, sanitisation | `POST /markdown` with `mode=gfm` ([REST][gh-rest-markdown]) | — |
| Relative image paths resolve on GitHub | Not by the API: **observed**, it returns relative `src` unchanged | Yes, the blob page and the repository home page |
| GitHub's rendered Mermaid and its version | Only by inspecting the served bundle (a proxy) | Yes: the `info` block renders client-side |
| AsciiDoc as GitHub previews it | `asciidoctor -S secure -a env-github`, plus `-v --failure-level=INFO` for references (**observed**) | Yes: GitHub's post-render sanitisation |
| Typst references | `typst compile` fails on a missing label (**observed**) | — |
| DocBook structure | RELAX NG and Schematron validation | — |
| An SVG's text, title and embedded fonts | Parse the SVG (**observed** in [§5.2](#52-local-byte-stability-runs)) | Yes: font fallback and screen-reader output |
| Served SVG content type and CSP | `curl -I` on `raw.githubusercontent.com` (**observed**) | — |
| `<picture>` theme switching; Firefox and iOS gaps | — | Yes |
| VS Code webview image loading | Read the resource-root rule from source | Yes: no documented headless check |

No primary source documents a headless check for GitHub's AsciiDoc HTML, VS
Code's webview, or GitHub's SVG acceptance rules.

## 5. Regeneration recipes

### 5.1 What is installed here

See [§1.3](#13-local-installation-this-machine-observed-2026-09-30). Four
renderers can be regenerated on this machine: PlantUML with C4-PlantUML 2.13.0,
D2, Typst, and Mermaid through `npx`. Structurizr would need a Docker
`-playwright` image or an explicit Java 21+ path. LikeC4 needs Node and
Playwright. Asciidoctor Diagram is not installed.

### 5.2 Local byte-stability runs

Each subject was rendered three times in fresh output directories on
2026-09-30, with the same tool version, and compared by SHA-1. The sources are
toy diagrams: a C4 container view, a sequence diagram, a flowchart with
`accTitle`/`accDescr`, and a Typst paragraph.

| Tool and route | Result | Observed in the output |
|---|---|---|
| PlantUML 1.2026.8, `-tsvg` (C4-PlantUML stdlib and a sequence diagram) | Identical ×3 | `<?plantuml 1.2026.8?>` and a `plantuml-src` PI; real `<text>` |
| D2 0.9.0, `--layout dagre`, `elk` and `tala` | Identical ×3 per engine | `data-d2-version="v0.9.0"`; three embedded WOFF fonts; real `<text>`, including the legend |
| mermaid-cli 12.0.0 via `npx`, system Chrome 154 (a flowchart and C4Context) | Identical ×3 per diagram | Root `id="my-svg"`; embedded fonts; flowchart labels in `foreignObject` |
| Typst 0.15.1, SVG | Identical ×2 | No `<text>` |
| Typst 0.15.1, PDF | **Different** ×2 | Identical once `SOURCE_DATE_EPOCH=0` was set (`--creation-timestamp` is the flag form) |

These runs establish repeatability on one machine and one day. They do not
establish stability across machines, architectures, fonts, browser updates or
tool upgrades. The sources above document drift for every one of those
upgrades: CF-17, CF-18, D2 0.8.2, PlantUML 1.2026.7 and C4-PlantUML 2.12.0.

### 5.3 What a pinned regenerate-and-compare check needs

These are the facts a Taskfile recipe would have to handle. Whether to add
one is for the design leaf to decide.

- **Pin the tool and its runtime, not just the tool.**
  - PlantUML: the jar version, the Graphviz version (or Smetana, stated
    explicitly), and the Java runtime.
  - Mermaid: the mmdc version and the browser. mmdc drives whatever Chrome
    Puppeteer resolves; here that was an auto-updating system Chrome.
  - Structurizr: the image tag, and the locale and timezone unless its
    metadata line is disabled.
  - Homebrew installs whatever is current, so a pin needs a versioned
    download, a container image, or `npx pkg@version`.
- **Pin layout settings.** D2's `--layout` and `--tala-seeds`; Mermaid's
  `layout`, `theme` and `look`, since 12.0.0 changed all three defaults.
- **Decide what version stamps mean.**
  - PlantUML's `<?plantuml VERSION?>` and D2's `data-d2-version` make every
    upgrade a byte diff.
  - D2's `--omit-version` and PlantUML's `--disable-metadata` remove them, but
    `--disable-metadata` also removes the embedded source that CF-15 values.
  - A check can compare bytes, and treat an upgrade diff as expected, or
    compare after normalising those stamps.
- **Fix timestamps** for PDF output (`SOURCE_DATE_EPOCH`).
- **Fonts.** Tools that embed fonts (D2, mmdc 12) render the same in GitHub's
  `<img>`. Tools that reference fonts (PlantUML, Structurizr) depend on the
  viewer.
- **A byte comparison checks the sources, not the reader's view.** It proves
  that the committed SVG matches its source under the pinned tool. It does
  not prove that the image renders where the reader reads it
  ([§4.4](#44-what-a-session-can-check-mechanically)).

## 6. Silences

No primary source was found for:

- a byte-stability guarantee from PlantUML, Mermaid, D2, Structurizr, LikeC4
  or Kroki;
- byte stability of Structurizr or LikeC4 image export, in any source;
- an official GitHub statement of its Mermaid version;
- Mermaid or mmdc embedding diagram source in the SVG;
- D2 accessibility documentation: no ARIA and no diagram-level
  title or description;
- `<title>`/`<desc>` support in Structurizr, LikeC4 or Kroki output;
- a hosted Structurizr successor, or a notice that the cloud shutdown has
  happened (the site still answered on 2026-09-30);
- a kroki.io privacy, terms or data-retention statement;
- whether Asciidoctor Diagram's Structurizr-to-D2 route still works;
- a Typst PlantUML package or a dedicated Typst C4 package;
- GitHub stripping `foreignObject` or other elements from repository SVGs,
  or documenting SVG validity rules;
- whether `#gh-dark-mode-only` is deprecated;
- a documented headless check of GitHub's AsciiDoc HTML or VS Code's webview;
- VS Code user documentation of the preview's local-resource-root rule. Only
  the source states it; the AsciiDoc extension documents the same rule for its
  own preview.

Not searched in this pass, and so neither confirmed nor refuted: glossary and
index support in each format (CF-26); Typst line-annotation packages (CF-23);
DocBook and Typst admonitions (CF-24); site generators for GFM; the Tinymist
Typst preview.

[#9 comment]: https://github.com/Linkuistics/grove/issues/9#issuecomment-5789802715
[#17]: https://github.com/Linkuistics/grove/issues/17
[#19]: https://github.com/Linkuistics/grove/issues/19
[i18-k73]: https://github.com/Linkuistics/grove/issues/18#issuecomment-5884943848
[ad-494]: https://github.com/asciidoctor/asciidoctor-diagram/issues/494
[ad-513]: https://github.com/asciidoctor/asciidoctor-diagram/issues/513
[ad-changelog]: https://github.com/asciidoctor/asciidoctor-diagram/blob/master/CHANGELOG.adoc
[ad-structurizr]: https://docs.asciidoctor.org/diagram-extension/latest/diagram_types/structurizr/
[asciidoc-admonitions]: https://docs.asciidoctor.org/asciidoc/latest/blocks/admonitions/
[asciidoc-docs]: https://docs.asciidoctor.org/asciidoc/latest/
[asciidoc-lang]: https://gitlab.eclipse.org/eclipse/asciidoc-lang/asciidoc-lang
[asciidoc-tags]: https://docs.asciidoctor.org/asciidoc/latest/directives/include-tagged-regions/
[asciidoc-vscode-preview]: https://github.com/asciidoctor/asciidoctor-vscode/blob/main/docs/modules/ROOT/pages/preview.adoc
[asciidoc-xref]: https://docs.asciidoctor.org/asciidoc/latest/macros/xref-validate/
[asciidoctor-939]: https://github.com/asciidoctor/asciidoctor.org/issues/939
[asciidoctor-diagram]: https://docs.asciidoctor.org/diagram-extension/latest/
[asciidoctor-reducer]: https://github.com/asciidoctor/asciidoctor-reducer
[asciidoctor-safe]: https://docs.asciidoctor.org/asciidoctor/latest/safe-modes/
[asciidoctor-tooling]: https://docs.asciidoctor.org/asciidoctor/latest/tooling/
[c4plantuml]: https://github.com/plantuml-stdlib/C4-PlantUML
[c4plantuml-376]: https://github.com/plantuml-stdlib/C4-PlantUML/issues/376
[c4plantuml-releases]: https://github.com/plantuml-stdlib/C4-PlantUML/releases
[d2-082]: https://github.com/d2lang/d2/releases/tag/v0.8.2
[d2-090]: https://github.com/d2lang/d2/releases/tag/v0.9.0
[d2-1273]: https://github.com/d2lang/d2/issues/1273
[d2-2377]: https://github.com/d2lang/d2/issues/2377
[d2-c4]: https://d2lang.com/blog/c4/
[d2-interactive]: https://d2lang.com/tour/interactive/
[d2-legend]: https://d2lang.com/tour/legend/
[d2-tala]: https://d2lang.com/tour/tala/
[d2-tala-oss]: https://d2lang.com/blog/tala-is-open-source/
[d2-tala-readme]: https://github.com/d2lang/d2/tree/master/d2layouts/d2talalayout
[docbook-oasis]: https://docbook.org/oasis/
[docbook-org]: https://docbook.org/
[docbook-releases]: https://codeberg.org/docbook/docbook/releases
[gfm-spec]: https://github.github.com/gfm/
[gh-13849]: https://github.com/orgs/community/discussions/13849
[gh-4103]: https://github.com/orgs/community/discussions/4103
[gh-58335]: https://github.com/orgs/community/discussions/58335
[gh-59781]: https://github.com/orgs/community/discussions/59781
[gh-67750]: https://github.com/orgs/community/discussions/67750
[gh-anchors]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax#custom-anchors
[gh-basic]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/basic-writing-and-formatting-syntax
[gh-diagrams]: https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/creating-diagrams
[gh-elk-discussion]: https://github.com/orgs/community/discussions/138426
[gh-markup]: https://github.com/github/markup
[gh-markup-1773]: https://github.com/github/markup/issues/1773
[gh-markups]: https://github.com/github/markup/blob/master/lib/github/markups.rb
[gh-mermaid-bundle]: https://viewscreen.githubusercontent.com/static/assets/mermaidMarkdown-035ded29910819bc6e5e.js
[gh-noncode]: https://docs.github.com/en/repositories/working-with-files/using-files/working-with-non-code-files
[gh-noncode-known]: https://docs.github.com/en/repositories/working-with-files/using-files/working-with-non-code-files#known-issues
[gh-quickstart]: https://docs.github.com/en/get-started/writing-on-github/getting-started-with-writing-and-formatting-on-github/quickstart-for-writing-on-github
[gh-relative]: https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-readmes#relative-links-and-image-paths-in-markdown-files
[gh-rest-markdown]: https://docs.github.com/en/rest/markdown/markdown
[kroki]: https://kroki.io/
[kroki-1758]: https://github.com/yuzutech/kroki/issues/1758
[kroki-1775]: https://github.com/yuzutech/kroki/issues/1775
[kroki-1835]: https://github.com/yuzutech/kroki/issues/1835
[kroki-2046]: https://github.com/yuzutech/kroki/issues/2046
[kroki-41]: https://github.com/yuzutech/kroki.io/issues/41
[kroki-install]: https://docs.kroki.io/kroki/setup/install/
[likec4]: https://github.com/likec4/likec4
[likec4-1429]: https://github.com/likec4/likec4/issues/1429
[likec4-1885]: https://github.com/likec4/likec4/issues/1885
[likec4-2975]: https://github.com/likec4/likec4/issues/2975
[likec4-2988]: https://github.com/likec4/likec4/issues/2988
[likec4-3181]: https://github.com/likec4/likec4/issues/3181
[likec4-cli]: https://likec4.dev/tooling/cli/
[likec4-notations]: https://likec4.dev/dsl/notations/
[linguist-contrib]: https://github.com/github-linguist/linguist/blob/main/CONTRIBUTING.md
[linguist-languages]: https://github.com/github-linguist/linguist/blob/main/lib/linguist/languages.yml
[mdn-img-role]: https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/Reference/Roles/img_role
[mdn-svg-image]: https://developer.mozilla.org/en-US/docs/Web/SVG/Guides/SVG_as_an_image
[mermaid-11171]: https://github.com/mermaid-js/mermaid/releases/tag/mermaid%4011.17.1
[mermaid-12]: https://github.com/mermaid-js/mermaid/releases/tag/mermaid%4012.0.0
[mermaid-1601]: https://github.com/mermaid-js/mermaid/issues/1601
[mermaid-2110]: https://github.com/mermaid-js/mermaid/issues/2110
[mermaid-2395]: https://github.com/mermaid-js/mermaid/issues/2395
[mermaid-5632]: https://github.com/mermaid-js/mermaid/issues/5632
[mermaid-6530]: https://github.com/mermaid-js/mermaid/issues/6530
[mermaid-8292]: https://github.com/mermaid-js/mermaid/issues/8292
[mermaid-8329]: https://github.com/mermaid-js/mermaid/issues/8329
[mermaid-a11y]: https://mermaid.js.org/config/accessibility.html
[mermaid-c4]: https://mermaid.js.org/syntax/c4.html
[mermaid-config-schema]: https://github.com/mermaid-js/mermaid/blob/develop/packages/mermaid/src/schemas/config.schema.yaml
[mmdc-12]: https://github.com/mermaid-js/mermaid-cli/releases/tag/12.0.0
[moir-2024]: https://thisisimportant.net/posts/docs-as-code-broken-promise/
[plantuml-2292]: https://github.com/plantuml/plantuml/issues/2292
[plantuml-243]: https://github.com/plantuml/plantuml/issues/243
[plantuml-cli]: https://plantuml.com/command-line
[plantuml-commons]: https://plantuml.com/commons
[plantuml-download]: https://plantuml.com/download
[plantuml-graphviz]: https://plantuml.com/graphviz-dot
[plantuml-smetana-fallback]: https://github.com/plantuml/plantuml/commit/a77b87e38d4b88ea6cbc95ccc69a865d4f3562f9
[plantuml-svg]: https://plantuml.com/svg
[plantuml-svggraphics]: https://github.com/plantuml/plantuml/blob/master/src/main/java/net/sourceforge/plantuml/klimt/drawing/svg/SvgGraphics.java
[plantuml-teoz]: https://github.com/plantuml/plantuml/commit/7d1d71800d4ba67b1e21c38dc6c1f77ce35f5ae5
[structurizr-20260419]: https://github.com/structurizr/structurizr/releases/tag/v2026.04.19
[structurizr-97]: https://github.com/structurizr/structurizr/issues/97
[structurizr-binaries]: https://docs.structurizr.com/binaries
[structurizr-cloud]: https://docs.structurizr.com/cloud
[structurizr-cloud-eol]: https://www.patreon.com/posts/142577083
[structurizr-eol]: https://docs.structurizr.com/eol
[structurizr-export]: https://docs.structurizr.com/export
[structurizr-exportcmd]: https://github.com/structurizr/structurizr/blob/main/structurizr-application/src/main/java/com/structurizr/command/ExportCommand.java
[structurizr-filetypes]: https://docs.structurizr.com/workspaces/file-types
[structurizr-home]: https://structurizr.com/
[structurizr-notation]: https://docs.structurizr.com/server/diagrams/notation
[structurizr-pngsvg]: https://docs.structurizr.com/export/png-and-svg
[structurizr-pricing]: https://docs.structurizr.com/server/pricing
[structurizr-ui52]: https://github.com/structurizr/ui/issues/52
[structurizr-ui53]: https://github.com/structurizr/ui/issues/53
[structurizr-vnext]: https://www.patreon.com/posts/146923136
[typst-0150]: https://github.com/typst/typst/releases/tag/v0.15.0
[typst-1421]: https://github.com/typst/typst/issues/1421
[typst-4932]: https://github.com/typst/typst/pull/4932
[typst-html]: https://typst.app/docs/reference/html/
[typst-index]: https://packages.typst.org/preview/index.json
[typst-packages]: https://github.com/typst/packages
[typst-pdf]: https://typst.app/docs/reference/pdf/
[typst-png]: https://typst.app/docs/reference/png/
[typst-raw]: https://typst.app/docs/reference/text/raw/
[typst-read]: https://typst.app/docs/reference/data-loading/read/
[typst-svg]: https://typst.app/docs/reference/svg/
[typst-tiago]: https://github.com/typst/packages/tree/main/packages/preview/tiago/0.1.0
[vscode-md]: https://code.visualstudio.com/docs/languages/markdown
[vscode-mdeditor]: https://github.com/microsoft/vscode/blob/main/extensions/markdown-language-features/src/preview/markdownEditorProvider.ts
[vscode-resources]: https://github.com/microsoft/vscode/blob/main/extensions/markdown-language-features/src/util/resources.ts
[vscode-trust]: https://code.visualstudio.com/docs/editing/workspaces/workspace-trust
[xsltng-guide]: https://xsltng.docbook.org/guide/2.8.5/ch-using.html
[xsltng-releases]: https://codeberg.org/docbook/xslTNG/releases

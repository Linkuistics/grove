# A rendered figure ships with its source

A figure drawn in a tool-rendered notation (C4 through any tool, PlantUML, D2,
Mermaid and the like) is committed as two files together: its editable source
and its tool-generated SVG. The commit records the tool version, the layout
settings and the regeneration command, and a repeatable recipe checks that the
SVG regenerates from the source.

A figure made of the page's own bytes needs none of this, because it is its own
source. That covers a table, a list figure and a drawing in a text fence.

The clause binds whatever commits a figure: an `art` stage placing a figure in
a document, or a `requirements` or `design` session preserving agreed exchange
views. A live sketch that is never committed is not bound.

## The trade-off

**The reader should see what the author saw, and host rendering does not
guarantee it.** On 2026-09-30, GitHub rendered Mermaid 11.17.2, while Grove's
design viewer pinned 12.0.0. Version 12.0.0 re-lays out and recolours existing
flowcharts, state diagrams and class diagrams (CF-17 in
[`documentation-toolchain-a.md`](../research/documentation-toolchain-a.md)). A
committed Mermaid source therefore draws differently depending on where it is
opened. Some hosts do not render some notations at all. GitHub shows PlantUML
and D2 source as text, and shows AsciiDoc diagram blocks as source (CF-12).
A committed SVG shows the same drawing wherever an image loads, and to a reader
with neither the tool nor a network connection.

**The source keeps the SVG editable and honest.** An SVG alone cannot be edited
as the author meant, and nothing shows it still matches its source. The
recorded command and the check close that gap. They are cheap in practice: in
the 2026-09-30 recheck, PlantUML, D2, Mermaid CLI and Typst each produced
byte-identical SVG across three runs of one pinned version.

**No tool guarantees byte stability.** Version stamps, such as PlantUML's
processing instruction and D2's `data-d2-version`, make every upgrade a byte
difference. Mermaid's output also depends on the browser it drives. So the
recipe pins the tool and its runtime, and treats drift after an upgrade as
expected: regenerate, then review.

**What it costs.** Generated files live in version control, and a figure gains
a build step and a check. The check proves agreement between source and SVG,
and nothing more. Whether the figure renders where its reader reads it is a
separate check, made in the declared viewing environment.

## Considered options

- **Rely on the host's native rendering**, such as GitHub's Mermaid. Rejected:
  the host's version drifts, and some notations it does not render at all.
  GitHub also warns that not all of its rendered charts are accessible. Reopen
  if a host publishes and pins its renderer version.
- **Commit the source only.** Rejected because a reader without the tool sees a
  diagram language where a drawing was meant. Reopen only together with the
  option above.
- **Commit the SVG only.** Rejected because it cannot be edited as authored,
  and nothing can show that it still matches its source.
- **Render at build time** with Kroki or Asciidoctor Diagram. Rejected as a
  default. The public Kroki instance receives the source, and no privacy
  statement was found. GitHub runs neither tool (CF-11, CF-12). A document
  whose format part declares such a build may still choose it.

## Relation to the walkthrough books

[A book carries no asset](a-book-carries-no-asset.md) still holds. A
walkthrough book's contract admits only the page's own bytes, so this clause
never fires there.

## What would reopen this

- A tool that guarantees byte-stable output across upgrades. The check could
  then compare meaning instead of bytes.
- A declared viewing environment where committed SVG fails but native rendering
  works. The 2026-09-30 recheck recorded one: SVGs are not shown in GitHub's iOS
  app.

The design this records is [`documentation-guidance`](../specs/documentation-guidance.md),
section 4.

# Walkthrough books

## Problem

`docs/walkthroughs/` holds one book and is about to hold six. Everything that
made the first book checkable was written for that book by name: the validator
compiles in its seventeen source roots, its thirty-three ownership blocks, its
eight slices, its page table and its early-use rows, and a repository test
compares those constants against normative tables in this document. A second book
has nowhere to put any of it.

The generalisation is not "make the constants a list of six". A book's corpus,
its slice order and its page inventory are **per-book data**; what this
specification owns is the *system* those data are read by — the page and
navigation conventions, the fragment language, the assurance contract, the prose
contract, and the shape of the data itself.

## Solution

A book is a directory under `docs/walkthroughs/` containing Markdown pages and
one **manifest**, `walkthrough.toml`. The manifest is the book's authoring
contract: which files it must reconstruct, which slice owns which range, what its
final page sequence is, and which documents outside itself it may cite. The
Markdown carries the fragment graph and every reconstructed byte. `book-check`
reads the manifest and validates the Markdown against it.

The line between the two is the subject of
[`a-book-cannot-witness-its-own-corpus`](../adr/a-book-cannot-witness-its-own-corpus.md)
and it is load-bearing throughout this document: **the manifest carries
obligation, the Markdown carries content.** Expansion reads the Markdown and the
source files and nothing else.

## Status and authority

This specification fixes the structure, source ownership, fragment notation,
manifest format, authoring checks, and prose contract for every walkthrough book
under `docs/walkthroughs/`. Production source is authoritative. A book quotes
that source exactly; neither a book nor its validator generates or overwrites
production files.

A book's in-scope corpus is frozen while the book is being authored. An accepted
source change requires the affected ownership ranges and fragments to change,
followed by final validation against the new bytes — and, where the change
touches source more than one book reconstructs, one commit carrying the source
change, every affected manifest and page, and a green validator run over every
book it touched. `scripts/check.sh` performs final validation of every committed
book against the committed corpus, so a source edit cannot bypass that comparison
by changing only a fixture or an isolated command.

## Intended outcome

A book is a self-contained walkthrough that introduces code in reader-dependency
order rather than filesystem order and reconstructs every in-scope source file
byte for byte from fragments embedded in its Markdown.

The system has five deliberate properties.

1. The numbered pages form one canonical reading path.
2. Source fragments sit beside the explanation that gives them meaning.
3. **Raw Markdown is sufficient to reconstruct the code.** Expansion consumes the
   book's Markdown and the authoritative source files and nothing else, and every
   fragment's parent, children, source range and owning slice are visible in raw
   Markdown without a renderer and without reading the manifest. The manifest
   contributes zero bytes to any reconstructed file, names no fragment below a
   source root or a top-level ownership block, and declares no parent or child
   edge. A reader holding the book and the crate recovers every source byte with
   no tooling.
4. **What a book owes is stated outside it, and checked in two directions.** The
   manifest declares the corpus as a rule whose base patterns are fixed by the
   book's declared subject rather than chosen, with named additions and
   exclusions drawn from a closed class, and the validator requires the declared
   root set to equal the set derived from the real directory. Derivation alone
   would not make the corpus external — it proves that the declared patterns
   matched, never that the author declared the right patterns — so every
   exception is also restated in this specification and cross-checked against the
   manifests. A book cannot narrow the corpus it proves.
5. Scoped checks prove one authoring increment without confusing a named future
   hole with an accidental unresolved reference; final checks prove the whole
   corpus with no holes.

Properties 3 and 4 are the pair, and neither is safe alone. A later change that
gives expansion a manifest field breaks 3; a later change that lets a book
enumerate its own roots with nothing to check them against — or choose its own
base patterns, or take an exception nothing outside the book agreed to — breaks
4.

## What a book is

A book is one directory `docs/walkthroughs/<book-id>/` containing:

- `walkthrough.toml` — the manifest, and the only non-Markdown file;
- `README.md` — the entry page and canonical contents;
- one numbered Markdown page per chapter, `NN-<stem>.md` from `01`;
- `source-index.md` and `concept-index.md` — lookup surfaces, not chapters and
  not alternate explanatory paths.

The book directory contains nothing else at any depth. A book id matches
`[a-z][a-z0-9]*(?:-[a-z0-9]+)*` and equals its directory name.

Every page has a **role**: `contents` for `README.md`, `chapter` for a numbered
page, `lookup` for the two indexes. A chapter additionally has a **slice** — the
authoring increment that owns it — and an **order**, which is its position among
chapters. `README.md` links every chapter in the current prefix and lists later
planned titles as plain text; a future page is never a broken link. Both indexes
exist from the first slice and grow with the prefix.

Slice identity is deliberately distinct from page identity so a page rename does
not migrate ownership directives, ledger rows and `--through` values. Slice IDs
are opaque tokens matching the fragment-ID grammar; they remain valid after
`.grove/` is removed and require no live Grove handle.

## The manifest

`walkthrough.toml` is TOML, UTF-8, LF-only, and ends in LF. It is loaded by the
CLI and passed to the core as data; the core performs no file discovery of its
own. An unparseable or schema-invalid manifest is a `U002` load failure with exit
status 2, reported before any validation runs.

Unknown keys are rejected rather than ignored: a mistyped field that is silently
dropped is an obligation that silently disappears, which is the failure the
manifest exists to prevent.

### Shape

```toml
schema = 1

[book]
id      = "ordinal-fs-tree"
title   = "Ordinal filesystem tree"
subject = "crates/ordinal-fs-tree"

[corpus]
include = [
  "crates/ordinal-fs-tree/Cargo.toml",
  "crates/ordinal-fs-tree/src/**/*.rs",
]

[[corpus.add]]
path   = "crates/ordinal-fs-tree/bin/syllabus.rs"
class  = "production-outside-src"
reason = "the demonstration consumer is production source and lives outside src/"

[[corpus.exclude]]
path   = "crates/ordinal-fs-tree/src/fixtures.rs"
class  = "test-support"
reason = "test-support module: evidence, not production source"

[[page]]
file  = "README.md"
id    = "contents"
title = "Ordinal filesystem tree"
role  = "contents"

[[page]]
file  = "01-orientation.md"
id    = "orientation"
title = "Orientation"
role  = "chapter"
slice = "orientation-k11"

[[page]]
file  = "source-index.md"
id    = "source-index"
title = "Source index"
role  = "lookup"

[[root]]
id    = "source-name"
path  = "crates/ordinal-fs-tree/src/name.rs"
lines = 717

[[block]]
id    = "name-seam-source"
root  = "source-name"
owner = "name-seam-k12"
lines = "1-717"

[[early-use]]
symbols   = "`Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName`"
first-use = "01-orientation.md#working-vocabulary"
owner     = "name-seam-k12"
statement = "Ordinal is mutable sibling position, key is stable tree identity, …"

[guide]
omitted = "ordinal-fs-tree is a bounded context of its own (CONTEXT-MAP.md); its walkthrough is self-contained and its reader need not have driven a grove."

[[glossary]]
path    = "docs/ordinal-fs-tree/CONTEXT.md"
anchors = ["entry", "ordinal", "key"]
```

### Groups

**`schema`** is the integer `1`. A manifest declaring any other value is a `U002`
load failure naming the supported version. The field exists so a future format
change is a refusal rather than a misreading.

**`[book]`** carries `id`, `title` and `subject`. `id` equals the directory name.
`title` is the `README.md` H1 and its page-table title. `subject` is the
repository-relative directory the corpus rule is rooted in, and exists so a
diagnostic can name what the book is about without inferring it from a path.

**`[corpus]`** carries `include`, an ordered non-empty array of patterns, and any
number of `[[corpus.add]]` and `[[corpus.exclude]]` entries. Each `add` and
`exclude` carries `path`, a `class` from the closed list below, and a non-empty
`reason`. Two pattern forms are accepted and no others:

- an exact repository-relative file path; and
- `<dir>/**/*.<ext>` — every file at any depth under `<dir>` whose name ends
  `.<ext>`.

A richer glob language is deliberately not accepted. Two forms cover every book
in this repository, they are implementable without a dependency, and a pattern a
reader cannot evaluate by inspection is a corpus boundary nobody checks. A
pattern in any other form is a `U002` load failure.

**`include` is anchored to `[book].subject`, not chosen.** It must contain both
base patterns — the exact path `<subject>/Cargo.toml` and the pattern
`<subject>/src/**/*.rs` — and a manifest omitting either is a `U002` load
failure naming the missing pattern. A book may declare further patterns; it may
not declare fewer. Without this, a manifest could `include` one file, declare
that one root, and satisfy every check in this specification while proving
nothing: derivation compares the declared roots against the tree *the patterns
reach*, so a rule the author narrows narrows the witness with it.

**The exception classes are closed.** `[[corpus.exclude]] class` is
`inline-test-module` or `test-support`; `[[corpus.add]] class` is
`production-outside-src`. Any other value is a `U002` load failure, and a book
needing one is describing something this specification has not seen — an
amendment here, not a field the book fills in freely. An `inline-test-module`
exclusion whose path's file name is not `tests.rs` is also `U002`: where a class
has a mechanically checkable form, the class claim is checked rather than
believed. `reason` remains free prose and is never validated, which is why it
carries the argument and `class` carries the constraint.

**`[[page]]`** entries are the final page inventory, in canonical order.
`README.md` must be first with `role = "contents"`; chapters follow in ascending
order; the two `role = "lookup"` pages follow. `file`, `id` and `title` are
required for every page; `slice` is required for a chapter and forbidden
otherwise. A chapter's `id` equals its filename stem without the numeric prefix.
Chapter `order` is not a field: it is the entry's position among chapters, so
there is no second place for it to disagree.

**`[[root]]`** entries are the source roots, in the order they appear in
`source-index.md`. `id` is a fragment ID, `path` is repository-relative, and
`lines` is the file's exact line count.

**`[[block]]`** entries are the top-level ownership blocks. `id` is a fragment
ID, `root` names a declared root, `owner` names a declared slice, and `lines` is
`N-M` with `1 <= N <= M`. The blocks of one root, in array order, must partition
that root's declared line count exactly: ordered, adjacent, non-overlapping, and
covering `1` to `lines`.

**Identities are unique, and the schema says so.** Within one manifest, no two
`[[page]]` entries share a `file` or an `id`; no two chapters share a `slice`; no
two `[[root]]` entries share an `id` or a `path`; and no two `[[block]]` entries
share an `id`. A duplicate in any of those is a `U002` load failure naming the
repeated value. Chapter slice uniqueness is the load-bearing one: the scoped
domain below is derived from those values, a `[[block]]` names one of them as its
`owner`, and `F010` reports a fragment defined outside its owner's chapter — so a
slice naming two chapters would make the accepted `--through` list carry a
duplicate and would leave `F010` with no single required page to name. Every
`[[block]] owner` must be the `slice` of a declared chapter, and every
`[[block]] root` a declared root `id`; a reference to neither is `U002`.

**`[[early-use]]`** entries are the minimum required early-use rows: the symbol
family, the first-use page and anchor, the owning slice, and the minimum local
statement. Authors add further rows to the book's own ledger; the manifest states
the ones a book may not omit.

**`[guide]`** is required, and is exactly one of two shapes: `path` plus a
**non-empty** `anchors` array, or `omitted` with a non-empty reason. A `[guide]`
carrying `path` with no anchors is a `U002` load failure. **`[[glossary]]`**
entries are optional and carry `path` plus `anchors`. Both are the subject of
*Outbound links* below.

### The scoped-slice domain is derived

The ordered slice list is the `slice` values of the `[[page]]` chapters, in
order. A slice is a valid `--through` value **iff at least one `[[block]]` names
it as `owner`** — a slice that owns no source has no prefix to prove and is
final-only. Under the relocated book that derivation yields the same seven
scoped values and the same final-only eighth as the constant it replaces.

Deriving rather than declaring removes a field that could disagree with the
blocks beside it. It also changes the CLI: `--through` can no longer be a
compile-time `value_parser` list, because the accepted values are not known until
`--book` has been resolved and its manifest loaded. `--through` therefore takes
any string, and an unrecognised value is a `U001` invocation error at exit
status 2 whose message lists the book's accepted values in order. `--help` says
that the accepted values come from the book's manifest and names neither a
specific book nor a specific slice.

### What the manifest may not carry

- Any fragment that is neither a source root nor a top-level ownership block.
- Any parent or child edge, insertion order, or fence body.
- Any source bytes, or anything expansion reads.
- Any prose that belongs to a page.

These are property 3 restated as a schema rule. A change that adds such a field
is a change to this specification and to
[`a-book-cannot-witness-its-own-corpus`](../adr/a-book-cannot-witness-its-own-corpus.md),
not a convenience.

## The corpus rule and its witness

The CLI enumerates every file under each `[corpus] include` pattern's directory,
applies the two accepted pattern forms, removes every `[[corpus.exclude]]` path,
adds every `[[corpus.add]]` path, and passes the resulting **derived set** to the
core alongside the manifest. The core requires the derived set to equal the set
of `[[root]]` paths, and reports:

- `F006` for a derived path with no root — a file the book must reconstruct and
  does not;
- `F006` for a root with no derived path — a root the rule does not reach and no
  `add` accounts for;
- `U002` for an `add` or `exclude` path that does not exist, so a stale exception
  is a failure rather than a silent no-op.

An `exclude` whose path the include patterns never matched is also `U002`: an
exception that excludes nothing is either a typo or a rule that has moved, and
both are worth a refusal.

**What derivation is stronger at, and what it cannot do.** It replaces the
two-hand-written-lists cross-check the compiled constants carried, and in one
direction it is stronger: a list checked against the tree it describes cannot be
wrong in the same direction as the tree, so a production file added to the crate
and forgotten by the book is a failure rather than a silence. In another
direction it is weaker, and saying otherwise would misdescribe the control.
Derivation proves that the declared patterns matched. It cannot prove the author
declared the right patterns, and it cannot judge an exception: a `reason` is
prose, and one manifest edit could otherwise move both the asserted rule and the
roots checked against it. Two of this specification's rules exist to close that,
and they are what make the corpus external rather than self-declared:

1. the base patterns are fixed by `[book].subject` (*Groups*), so the rule's
   floor is not the author's to choose; and
2. every exception is declared twice — in the manifest, and in the normative
   inventory below — so an addition or exclusion is an agreement between the book
   and this specification rather than an assertion the book makes about itself.

**The corpus exception inventory.** These are the complete `[[corpus.add]]` and
`[[corpus.exclude]]` entries every book in this repository may carry. The
five campaign deliverables that appear in no row declare none.

| Book | Kind | Path | Class |
|---|---|---|---|
| `ordinal-fs-tree` | `add` | `crates/ordinal-fs-tree/bin/syllabus.rs` | `production-outside-src` |
| `ordinal-fs-tree` | `exclude` | `crates/ordinal-fs-tree/src/fixtures.rs` | `test-support` |
| `ordinal-fs-tree` | `exclude` | `crates/ordinal-fs-tree/src/fs/apply/tests.rs` | `inline-test-module` |
| `ordinal-fs-tree` | `exclude` | `crates/ordinal-fs-tree/src/ops/tests.rs` | `inline-test-module` |
| `ordinal-fs-tree` | `exclude` | `crates/ordinal-fs-tree/src/plan/tests.rs` | `inline-test-module` |
| `ordinal-fs-tree` | `exclude` | `crates/ordinal-fs-tree/src/snapshot/tests.rs` | `inline-test-module` |
| `grove-loop` | `exclude` | `crates/grove-loop/src/task_grow/tests.rs` | `inline-test-module` |

A repository test compares this table against the `[[corpus.add]]` and
`[[corpus.exclude]]` entries of every manifest under `docs/walkthroughs/` and
requires the two sets to be equal, per book, path for path and class for class.
That test is the successor to the interim
`compiled_corpus_copy_matches_the_book_ledger_tables` bridge, and it must exist
and have been seen to fail — against a manifest carrying an exception this table
does not — before that bridge is deleted. It is a **repository** test rather than
a `book-check` diagnostic on purpose: `book-check` runs against one book
directory and would be reading the book's own account of itself again, whereas
this comparison is with a document the book does not own.

Six rows is not the six sets of per-book tables the compiled design would have
needed. The obligation that scales with the corpus — every root, every block —
stays derived and per-book; only the exceptions, which derivation provably
cannot check, are restated here, and they are few because the rule is good.

**The rule is per-book data because books differ.** The relocated
`ordinal-fs-tree` book adds a consumer outside `src/` and excludes a
test-support module and four inline test modules; the campaign's five
deliverables take the bare rule with one exclusion between them. Every exclusion
in both cases is an inline test module or a test-support module — evidence rather
than production source, which is the classification
`linkuistics:writing-code-walkthroughs` asks for at intake.

## Navigation

Every chapter has the same visible navigation line at its top and bottom. The
accepted raw forms are exact Markdown lines:

```markdown
[Contents](README.md)
[Contents](README.md) | [Next: Name seam](02-name-seam.md)
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Reference domain](03-reference-domain.md)
[Previous: Syllabus CLI](07-syllabus-cli.md) | [Contents](README.md)
```

The lines are, in order, the one-page-prefix form, the first-page form, the
interior form, and the final-page form. Filenames and labels are substituted from
the manifest's `file` and `title`. The last page in a scoped prefix uses the
final-page form even when the manifest gives it a successor. Adding a page
changes the preceding page's two navigation lines as well as the contents. In
final mode the last chapter is the only chapter without Next.

The two lookup indexes link back to `README.md` and do not participate in
Previous/Next order. `README.md` links both indexes as optional lookup.

Every page has exactly one H1 whose text is its manifest `title`. Headings do not
skip levels. A section that is a link target has an explicit HTML anchor
immediately before its heading:

```html
<a id="rollback"></a>
## Rollback
```

Anchor IDs match `[a-z][a-z0-9]*(?:-[a-z0-9]+)*` and are unique within their
file. Links never depend on renderer-generated heading slugs.

Every book file declares one page identity on a column-zero line immediately
after its H1. Chapters use this exact attribute order and spacing:

```markdown
<!-- book-page id="orientation" slice="orientation-k11" order="1" -->
```

The ID, slice and order match the manifest. The entry and lookup pages use,
respectively:

```markdown
<!-- book-page id="contents" role="contents" -->
<!-- book-page id="concept-index" role="lookup" -->
<!-- book-page id="source-index" role="lookup" -->
```

The top navigation line immediately follows the identity on chapters. The bottom
navigation line is their final nonblank content. No other `book-page` attribute,
order, spacing, or role is accepted.

## Fragment language

### Identifiers and paths

Fragment and source-root IDs match:

```text
[a-z][a-z0-9]*(?:-[a-z0-9]+)*
```

IDs are globally unique within a book. Source paths are repository-relative, use
`/`, and must name one of the manifest's declared root paths. A source root has
an ID in the same namespace as ordinary fragments.

Line ranges are 1-based and inclusive. Every boundary is a source-line boundary.
A book's frozen corpus must be UTF-8-compatible, LF-only, end each file with LF,
and contain no fragment directive collision. A source file that violates one of
those facts cannot be a root until a grammar decision is taken; it is a `U002`
load failure, not a silent skip.

### Shared byte-level lexer

Fragment and Markdown validation consume the same directive and fenced-range
token stream from one shared byte-level lexer. Neither check rescans comments or
fences independently. The Markdown link scanner additionally recognizes exact
inline-code spans once, using the accepted form in the link contract, so links
inside them remain opaque without making inline code part of the fragment
language. Book files must be valid UTF-8, contain only LF line endings, and end
in LF. The lexer records the zero-based byte offset and 1-based line and column
of every token.

Outside a fence, these are the complete reserved line forms. Every space shown is
one ASCII space, every directive begins at column zero, attribute order is fixed,
decimal values have no leading zero, and no trailing space or unshown attribute
is permitted:

```text
<!-- book-page id="ID" slice="SLICE" order="N" -->
<!-- book-page id="ID" role="contents" -->
<!-- book-page id="ID" role="lookup" -->
<!-- source-root «ID» source="PATH" lines="N-M" -->
<!-- /source-root -->
<!-- fragment «ID» owner="SLICE" source="PATH" lines="N-M" parent="ID" -->
<!-- /fragment -->
<!-- insert «ID» -->
<!-- defer «ID» owner="SLICE" lines="N-M" -->
```

`ID` uses the fragment-ID grammar above for fragment directives and a manifest
page `id` for `book-page`. `SLICE` is a slice the manifest declares. `PATH` is a
root path the manifest declares. `N` and `M` are positive canonical decimal
integers and `N <= M`. The page `order` is the chapter's canonical position.

A line beginning with one of the reserved prefixes `<!-- book-page`,
`<!-- source-root`, `<!-- fragment`, `<!-- insert`, `<!-- defer`,
`<!-- /source-root`, or `<!-- /fragment` that is not one exact form is a `P001`
malformed-directive finding. An exact directive in a disallowed context is
`P002`. Other HTML comments are ordinary Markdown and emit no fragment tokens.

An ordinary fenced block opens at column zero with a run of three or at least
five identical backticks or tildes, an optional nonempty ASCII info token
immediately following the run, and LF. It closes with exactly the same delimiter
run and LF. The info token matches `[A-Za-z0-9][A-Za-z0-9_-]*`; no other
opening-line bytes are accepted in the book subset. The shared lexer treats every
body byte, including directive-looking lines, as opaque Markdown. An unclosed
ordinary fence is `P002`.

Exactly four backticks are reserved for production literals. Immediately after a
`fragment` opening line, one of these exact lines starts a literal:

~~~text
````rust
````toml
~~~

The language must match the source path's extension: `rust` for `.rs`, `toml` for
a Cargo manifest. The literal ends at this exact column-zero line:

~~~text
````
~~~

Each delimiter line ends in LF. The closing delimiter must be immediately
followed by `<!-- /fragment -->` and LF. The literal body is opaque raw bytes, so
triple-backtick doc-comment lines and directive-looking source are never
tokenized. No other four-backtick fence is accepted. A fragment opening followed
instead by `insert` or `defer` directives is a composite. Blank lines, prose,
fences, and non-directive comments are forbidden inside a source root or
composite. Roots and fragments cannot nest, closing directives must match the
active construct, and EOF with an active construct is `P002`.

Four backticks are fixed because production source in this repository contains
runs of three backticks in documentation comments but no run of four. A book
whose corpus contains a run of four backticks at column zero cannot be authored
under this grammar and requires a format decision; the validator reports it
rather than mis-tokenizing it.

Unlabelled `rust` or `toml` fences in book files are rejected so copied
production source cannot silently sit outside the graph. Examples that are not
production source use a different label such as `text` or `console`.

`P003` covers invalid UTF-8, CR bytes, and a missing final LF. Lexical findings
do not prevent the lexer from resuming at the next LF when that is safe. Both
validators receive the same successfully recognized directives, literal byte
ranges, ordinary-fence ranges, and lexical findings.

### Source roots

Every declared root has exactly one root directive in `source-index.md`:

```markdown
<!-- source-root «source-demo-rs» source="crates/demo/src/demo.rs" lines="1-4" -->
<!-- insert «demo-opening» -->
<!-- defer «demo-tail» owner="syllabus-cli-k17" lines="3-4" -->
<!-- /source-root -->
```

A source root contains only ordered `insert` or `defer` directive lines. It emits
their expansions in order. Its declared range is the complete file, and its `M`
equals the manifest's `lines` for that root.

### Literal fragments

A literal definition is an opening directive, one immediately following raw
four-backtick fence, and a closing directive:

`````markdown
<!-- fragment «demo-opening» owner="orientation-k11" source="crates/demo/src/demo.rs" lines="1-2" parent="source-demo-rs" -->
````rust
fn run() {
    begin();
````
<!-- /fragment -->
`````

The fence body is read from the Markdown file as bytes. The opening and closing
fence lines and all fragment directive lines emit no bytes. The body is not
trimmed, dedented, reindented, interpolated, decoded and re-encoded, or
newline-normalized.

### Composite fragments

A composite definition uses the same metadata but contains only insertion
directives:

```markdown
<!-- fragment «demo-body» owner="orientation-k11" source="crates/demo/src/demo.rs" lines="2-3" parent="source-demo-rs" -->
<!-- insert «demo-begin» -->
<!-- insert «demo-end» -->
<!-- /fragment -->
```

A definition is literal or composite, never mixed. An insertion is exactly:

```markdown
<!-- insert «demo-begin» -->
```

It must occupy its whole physical line at column zero. The directive line,
including its LF, emits zero bytes and is replaced by the referenced expansion.
Definition order in the Markdown has no expansion meaning.

### Deferred holes

A future-owned source range is represented distinctly:

```markdown
<!-- defer «demo-tail» owner="syllabus-cli-k17" lines="3-4" -->
```

A defer is legal only as a direct child of a source root, occupies a complete
column-zero line, names one strictly later slice from the manifest, and declares
the exact contiguous range it reserves. A defer inside a composite or any other
context is `P002`. Its target definition may be absent. The named owner owns the
range immediately; the earlier slice does not receive credit for it.

When the owning slice lands, it replaces the defer line with the corresponding
`insert` line and adds the definition. An `insert` whose target is absent is an
ordinary unresolved reference and always fails. Final mode rejects every `defer`,
even if a definition with the deferred ID already exists.

### Graph and expansion rules

Each source file forms one tree:

- A non-root fragment has exactly one declared parent and is inserted exactly
  once by that parent.
- A source root has no parent.
- Parent and child source paths are identical; cross-source insertion fails.
- A child's line range is contained by its parent. Sibling ranges are ordered,
  adjacent, and non-overlapping.
- The children of a composite form a gapless partition of its range.
- A literal fragment's fence bytes equal the named source range exactly.
- Every descendant of a top-level ownership block has the same owner.
- Every defined fragment is reachable from exactly one source root.

Expansion recursively replaces insertions in their written order. There are no
aliases, reusable fragments, continuations, implicit concatenation, indentation
directives, templates, conditionals, substitutions, or transforms. Reusing a
passage is prose plus a link, not a second insertion of the same fragment.

An owning slice may refine one of its top-level blocks into smaller,
intent-named descendants. The top-level ID, owner, source path, range, and
position in its root remain stable and continue to match the manifest.
Descendant ranges must still form a gapless partition. Line-count credit is
assigned to the top-level block only, so refinement cannot change slice ownership
totals.

## Source and ownership ledger

`source-index.md` contains, from the first slice onward:

1. one source-root definition per declared root, in manifest order;
2. a table of every top-level ownership block, owner, range, line count, and
   current `deferred` or `resolved` state;
3. a fragment index with ID, page, source, range, owner, parent, and direct
   children; and
4. the early-use ledger.

The four tables appear under the exact H2 headings `Source roots`, `Ownership
blocks`, `Fragment index`, and `Early uses`. Their header names and column order
are fixed by the schemas below. After the heading and one blank line, the header
is the next line, followed by the separator and one or more data rows; the first
non-row line ends the table. Source-root directive blocks follow the Source roots
table before the next H2. A table row is one LF-terminated physical line. It
begins and ends with `|`; cells have exactly one space between the delimiter and
content. Data cells contain no raw `|`, backslash escape, or line break.
Identifiers, paths, ranges, states, and page locations are wrapped in one
backtick on each side. Decimal counts contain ASCII digits and optional grouping
commas. The separator row contains `---` in every cell. Source-root, ownership,
and fragment rows follow manifest root order, then ascending source range, then
fragment ID; the early-use order is defined with that table. A missing, extra,
reordered, or malformed column is `F009`.

**The four tables are derived indexes, not authority.** The manifest is the
contract, the directives are the execution, and the tables are the reconciled
human-readable view of both. The validator recomputes every relationship and
reports `F009` for a row that disagrees with the manifest or with the directives.
That is the stated trust order, and it is why raw Markdown still provides
outward relationships in parents and inward lookup without an author-maintained
second copy of the graph.

The Source roots and Ownership blocks tables restate manifest rows. This is the
one restatement the system accepts, because a reader of the book needs the
corpus visible in the book, and it is safe precisely because it is checked in
both directions rather than trusted.

The source-roots schema:

```markdown
| Root ID | Source path | Lines |
|---|---|---:|
| `source-name` | `crates/ordinal-fs-tree/src/name.rs` | 717 |
```

The ownership schema. The State column is `resolved` once the owning slice has
replaced its defer with an insert and added the definition, and `deferred` until
then:

```markdown
| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `name-seam-source` | `source-name` | `name-seam-k12` | `1-717` | 717 | `deferred` |
```

The fragment index has this exact schema:

```markdown
| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `ops-insert` | `mutation-algebra` | `source-operations` | `composite` | `mutation-algebra-k15` | `176-251` | `mutation-operations-source` | `ops-insert-resolve`, `ops-insert-shifts`, `ops-insert-plan` |
```

`Kind` is `root`, `literal`, or `composite`. A root's page is `source-index`,
owner and parent are `—`, and its direct child IDs include resolved inserts and
deferred top-level IDs in source order. A non-root definition must occur in the
chapter the manifest assigns to its owner; its Page ID must match that page. A
fragment with no children uses `—`; otherwise Child IDs are backtick-wrapped and
separated by comma plus one space. Deferred targets do not receive non-root
fragment rows until defined; their outbound occurrence remains visible in the
root row and ownership table.

`source-index.md` also carries an owned-source totals table — slice, page, and
owned lines, with a total row naming the root count and the corpus line count.
The totals count each source line once at its top-level block and do not count
composite ancestors or lookup prose. A scoped report prints completed lines,
deferred lines, and `final=false`; it never presents a prefix as exhaustive.

## Early-use ledger

When a page first uses a codebase-specific type whose source belongs to a later
slice, or reproduces source bytes whose referent belongs to a later slice,
`source-index.md` records:

- the symbol or closely coupled type family;
- the first-use page and explicit anchor;
- the later owning slice;
- the minimum definition or behavior that the earlier page must state locally;
  and
- whether the later page has supplied its full explanation.

The schema is:

```markdown
| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
```

`Status` is `pending` while the owner page is outside the current prefix and
`explained` from the owner's scoped increment onward. The owning slice changes
only its rows. Rows sort by first-use page order, first-use anchor occurrence in
that page, owner order, then the bytewise Symbol family cell.

The manifest's `[[early-use]]` entries are the rows a book may not omit; authors
add further rows before introducing any additional later-owned type. The
validator checks row structure, owner order, page and anchor existence, presence
of every manifest-required row, and resolution status. Technical review checks
that the local statement is present and sufficient; keyword matching is not a
substitute for that judgment.

## Outbound links: the guide and the glossary

A book is self-contained for its *claims* — every fact needed to understand it is
inside it — but it is not the reader's entry point to the system it describes.
Two documents outside a book are therefore citable, and no others.

**The guide.** `docs/USAGE.md` is the reader's entry point for the `grove`
binary. A book whose subject is a Grove runtime crate links it from **exactly one
place: its `README.md` reader contract.** One link per book, not one per page:
the coupling this bounds is a book's dependence on the guide's anchors, and
putting the link on every page multiplies that dependence by the page count for
no reader benefit a contents page cannot give.

**The glossary.** Grove's vocabulary is linked, never re-taught. A book may cite
the glossary of its own context — `CONTEXT.md` for a Grove runtime crate,
`docs/ordinal-fs-tree/CONTEXT.md` for the extracted library — from any page, at
the first use of a term it does not own.

### Anchors are declared, and the declaration is what is checked

A citation with no anchor — `[the user guide](../../USAGE.md)` — is permitted
anywhere except the one place the contract binds: a book declaring a `[guide]`
`path` must cite one of its declared guide anchors from its `README.md` reader
contract, and a `README.md` whose guide link carries no anchor is `M201`. That
single obligation is what the guide-first ordering buys, and without it the
ordering buys nothing — see *The link contract*'s note below. Everywhere else,
and for every other target, a citation naming an anchor is permitted only when
**both** hold:

1. the anchor is listed in that book's `[guide]` or `[[glossary]]` `anchors`
   array; and
2. the anchor exists in the target document as an explicit `<a id="…"></a>` line
   immediately preceding a heading.

A citation naming an unlisted anchor is `M201`. A listed anchor that the target
document does not carry is `M201` against the manifest, reported once per
missing anchor whether or not any page cites it — so a guide edit that removes an
anchor fails the books that reserved it, rather than failing later and elsewhere.

Requiring the explicit form is the whole mechanism. The repository-wide sweep
`every_repository_markdown_reference_resolves` accepts a renderer-generated
heading slug, which changes silently when a heading is retitled; an explicit
anchor does not. `walkthroughs-k3` decision 5 ordered the guide ahead of the books
so books would not inherit unstable anchors, and this is the check that makes
that ordering mean something rather than merely happen.

**Two obligations follow from this, are not discharged here, and are placed
rather than noted.** `docs/USAGE.md` and `CONTEXT.md` carry **no** explicit
anchors today, and a book cannot be validated until every anchor it reserves
exists in its target in the explicit form. Because a book must now cite at least
one guide anchor, these are not contingent on a future author's taste:

- **`docs/USAGE.md`.** The leaf that writes the guide publishes an explicit
  `<a id="…"></a>` anchor for every entry point its coverage inventory names as
  stable, and that anchor set is what books reserve from. Anchors are added
  before the books, which is the whole content of the guide-before-books
  ordering.
- **`CONTEXT.md`.** The first book that cites the glossary adds the explicit
  anchors it reserves, and each later book adds any it needs. Adding one is
  additive and breaks nothing that exists.

Neither obligation is discharged by a book linking the file with no anchor. That
is the outcome this contract exists to exclude, because it satisfies a link check
while leaving the anchor-stability property — the reason two sessions sit ahead of
the pilot — asserted by nothing.

### The link contract

Outside code fences and inline code, local Markdown links use the simple form
`[descriptive label](relative/path)` with an optional `#explicit-anchor`.
Markdown heading destinations require the explicit anchor; file-only links do
not. Nested labels, link titles, and escaped destinations are outside the
accepted book subset. Labels `here`, `this`, and `more` are rejected as
non-descriptive. `http`, `https`, and `mailto` destinations are syntax-checked
but never fetched.

An inline-code span opens with a run of one or more unescaped backticks and
closes only with an unescaped run of exactly the same length; its bytes are
opaque to the one link scanner.

The permitted local targets are exactly: this book's own declared pages; this
book's declared source roots; the `[guide]` path; and each `[[glossary]]` path.
Every other repository artifact is not a permitted link target, and a link to
one is `M201`. The mechanism is the snapshot: the CLI loads the book's pages, its
root sources, and its declared outbound documents, and a link that resolves to
nothing in the snapshot fails.

### The relocated book declares no guide link

`docs/walkthroughs/ordinal-fs-tree/` is **not** brought into guide-link
conformance, and no leaf is cut to do it. `CONTEXT-MAP.md` holds `ordinal-fs-tree`
as a bounded context of its own whose walkthrough is the self-contained
reader-facing account of the crate; its stated audience is a reader proficient in
Rust and operating-system APIs, who need not have driven a grove and for whom the
`grove` binary's user guide is not an entry point. A mandatory link from that book
into that guide would assert a dependency the context map denies.

The `[guide] omitted` field carries that reason in the book's own manifest, so
the exemption is a declared decision a reader can find rather than an absence.
Everything else in this specification binds the relocated book unchanged.

## The structure brief

The source does not contain enough to structure a book. Audience, conceptual
order and what deserves emphasis are nowhere in the code, so **each book takes a
human-authored structure brief as an input artifact**, produced by a
`requirements` leaf immediately ahead of the book it serves.

A structure brief settles, and this specification does not:

- the chapter sequence: one entry per chapter, its title, its slice ID, and the
  concepts and seams it is responsible for;
- the mapping from that sequence onto the corpus: which top-level ownership block
  each chapter owns, and where a file's concerns split across chapters;
- the complete worked example each chapter carries, its required anchor, and the
  example's start and observable end;
- the early uses the order forces — every codebase-specific type an earlier
  chapter must name before its owner explains it; and
- what the book deliberately does not cover, and why.

The brief's chapter sequence and ownership mapping are what the manifest's
`[[page]]` and `[[block]]` groups record; the brief is the human decision and the
manifest is its machine-readable form. A book without a structure brief has a
conceptual order nobody chose.

**A structure brief is a durable artifact, not a session input that expires.**
It lives at `docs/specs/<book-id>-book-structure.md` — outside the book
directory, which holds nothing but the manifest and the book's own pages, and
under `docs/specs/` because it specifies the book's shape. It is the document a
later edit or review of that book is read against: the finished Markdown shows
what was written, never which responsibilities were binding, so a book whose
brief exists only in a session transcript or in version-control history cannot be
reviewed against its own design.

**Every book has one, the relocated `ordinal-fs-tree` book included.** Its
responsibilities were chosen and written down — they were the page-by-page
concept and worked-example sections of the one-book specification this document
replaced — and rewriting that specification for six books moved them out without
putting them anywhere. Recovering them into the form above is work this
specification requires and does not perform.

## Authoring workflow and scoped proof

The first slice creates `README.md`, both lookup indexes, the first chapter, every
source-root directive, the complete top-level ownership ledger, its owned
fragment definitions, and defers for every later-owned block. The manifest is
complete from the start: it is the plan, and a plan authored incrementally cannot
be compared against a prefix.

Each later source-owning slice performs this sequence:

1. Read its fixed top-level blocks and early-use rows from the manifest.
2. Partition each owned block into intent-named, line-aligned literal or
   composite fragments beside the page's explanation.
3. Replace every defer naming that slice with an insert of the same ID.
4. Change the corresponding source-index state from `deferred` to `resolved` and
   add fragment index rows.
5. Add its page to the canonical prefix and update contents and navigation.
6. Complete or add early-use rows and curate concept-index entries.
7. Run `book-check --check all` through its slice, and the relevant crate checks.

A slice may decompose into child work at a conceptual seam. The owning slice ID
and top-level ranges remain unchanged; children divide the page or fragment work
without changing ledger ownership.

A book's final chapter may own no source. Such a slice resolves no legitimate
production hole — every source-owning slice must already have replaced its
defers — and it is final-only: it adds synthesis, closes the indexes, runs final
fragment and Markdown validation, and records the final evidence. The manifest
does not declare that; it follows from the slice owning no `[[block]]`.

## Fragment validator contract

The implementation exposes one deep operation, expressed here as interface
pseudocode rather than a required Rust type layout:

```text
validate(BookSnapshot, Manifest, Request { scope, checks }) -> ValidationReport

scope  = Through(slice-index) | Final
checks = Fragments | Markdown | All
```

`Through` carries a **resolved index into the manifest's scoped-slice domain**,
not a free string: the core cannot represent an unknown scoped value or a
final-only slice. Resolving the caller's text against that domain, and reporting
an invocation error when it does not resolve, belongs to the CLI — which is where
the domain first becomes known, since it comes from the manifest.

`BookSnapshot` contains the recursive book-directory entry inventory, including
which entries are non-regular; the bytes of the book's declared files; the bytes
of its declared source roots; the bytes of its declared outbound documents; and
the derived corpus set described under *The corpus rule and its witness* — all
under explicit repository-relative paths. The core validator has no clock, VCS,
network, environment, or directory-discovery dependency. A thin CLI loads the
snapshot and the manifest. Tests construct both in memory.

The CLI requires `--repo PATH`. It resolves a relative value against the process
working directory and uses that directory as the only repository root. `--book
PATH` is required, must be a normalized repository-relative path with no `..`
component, and is joined to `--repo`; every manifest path is joined to the same
root. The book root must be a real directory rather than a symlink, and must
contain a readable `walkthrough.toml`. Inventory traversal records symlinks and
other non-regular children but never follows them. An unreadable unexpected child
directory remains an inventory entry and does not prevent its `M101` finding. The
validator neither searches ancestors nor consults VCS.

The runnable author command is:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/walkthroughs/ordinal-fs-tree \
  --through read-path-k14 \
  --check all
```

The corresponding runnable final command is:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/walkthroughs/ordinal-fs-tree \
  --final \
  --check all
```

`--through` and `--final` are mutually exclusive and one is required. `--check`
accepts `fragments`, `markdown`, or `all` and defaults to `all`. `--output`
accepts `text` or `json` and defaults to `text`.

`book-check --help` contains one runnable scoped and one runnable final
invocation, states that `--through` takes a slice declared by the named book's
manifest, lists all three check selections, explains that validation is
read-only, lists the exit statuses, and describes the JSON schema. It names no
particular book as its subject. The command is always non-interactive, never
starts a pager or spinner, and emits no color. Validation reports go to stdout.
Invocation and internal load failures go to stderr in text mode; after `--output
json` has been recognized, they use the same versioned JSON envelope on stdout
and leave stderr empty.

Exit statuses are stable:

- `0`: the requested scope is valid;
- `1`: deterministic validation findings exist;
- `2`: invalid invocation or required input cannot be loaded; and
- `3`: internal validator failure.

Validation is read-only. It has no fix, tangle-to-disk, or source-generation
mode.

### Scoped fragment checks

`Through(slice)` means the complete canonical prefix ending in that slice. It
requires:

- every manifest source root and every top-level ledger row;
- exact definitions and byte comparison for every block owned by the prefix;
- an explicit defer, with the manifest's exact ID, owner, and range, for every
  later-owned block;
- no defer owned by the prefix or by an unknown/non-later slice;
- no early definition of a later-owned block;
- valid global uniqueness for every definition already present; and
- an exact sum of resolved plus deferred ranges for every root.

Known byte segments are compared at their declared source offsets. Deferred
ranges are reported in coverage totals as deferred, not reconstructed and not
successful source coverage. An unresolved insert is an error in every scope.

### Final fragment checks

Final mode requires exactly the manifest's source roots and source paths, no
extras, every top-level block resolved, no defer directives, one reachable
definition per ID, gapless non-overlapping coverage, and byte-for-byte equality
for every recursively expanded root. The final report states the book's root
count, its corpus line count, zero deferred ranges, and `final=true` — all taken
from the manifest and the comparison, never from a compiled constant.

### Deterministic diagnostics

The validator collects every independent finding it can safely determine.
Duplicate IDs become ambiguous and are excluded from expansion so they do not
produce misleading cascades. A structurally invalid root skips its byte
comparison while unrelated roots continue.

Diagnostic phases and sort order are fixed:

1. input, manifest and directive parsing;
2. source inventory, corpus derivation and ledger reconciliation;
3. duplicate definitions and roots;
4. insert and defer validity;
5. cycles and reachability;
6. parent, range, ownership, and coverage validity;
7. byte comparison;
8. Markdown structure and local links.

The phase names in structured output are `parse`, `inventory`, `identity`,
`reference`, `graph`, `coverage`, `bytes`, and `markdown`, in that order. Within
a phase, findings sort by these total-order keys:

1. manifest root index, with null after every root;
2. source byte offset, with null after every offset;
3. manifest page index, then lexical path for non-book paths;
4. primary Markdown byte offset, with null after every offset;
5. code;
6. fragment ID, then root ID, with null after strings;
7. the canonical serialization of related locations; and
8. message.

Byte-identical diagnostic records are deduplicated before sorting. Paths are
repository-relative, wording is platform-independent, and text output has no
color. A cycle is rotated to its lexicographically smallest ID and follows
children in source position, then ID order. Byte comparison reports the first
mismatch per source root.

Stable diagnostic classes:

| Code | Meaning and required evidence |
|---|---|
| `P001` | Malformed reserved directive; raw offending line and expected line form. |
| `P002` | Valid directive or fence form in an invalid lexical context, mismatched close, nesting, or unclosed construct; active context and location. |
| `P003` | Invalid book encoding or line ending; first invalid byte or EOF. |
| `F001` | Duplicate fragment or source-root ID; one finding per duplicated ID, earliest canonical location as primary and every later occurrence in `related`. |
| `F002` | Insert target unresolved or ambiguous; parent and location. |
| `F003` | Invalid defer: unknown/non-later owner, range mismatch, overdue owner, early definition, or any defer in final mode. |
| `F004` | Cycle; canonical complete witness path. |
| `F005` | Defined fragment unreachable or reachable from the wrong/multiple source roots; expected and observed roots. |
| `F006` | Missing, extra, duplicated, or wrong-path source root, **including a root the corpus rule derives that the manifest does not declare and a declared root the rule does not reach**; manifest row, derived path, and the rule element responsible. |
| `F007` | Missing, overlapping, duplicated, out-of-order, or wrong-owner source range; range and all responsible fragments. |
| `F008` | Literal bytes or expanded root differ from source; first source byte offset, 1-based source line, expected/actual byte or EOF, emitting fragment, owner, and root-to-fragment path. |
| `F009` | Ledger or fragment-index row disagrees with the manifest or the directives; both values and locations. |
| `F010` | Fragment definition appears outside its owner's assigned chapter; actual and required page. |
| `M101` | Page/inventory shape, including a directory entry the manifest does not declare. |
| `M102` | Heading/anchor shape. |
| `M103` | Navigation. |
| `M104` | Forbidden untracked source fence. |
| `M105` | A literal fragment whose nearest preceding nonblank block is not a paragraph block. |
| `M201` | Local link: unresolvable target, non-permitted target, non-descriptive label, undeclared anchor, or a declared anchor the target document does not carry. |
| `U001` | Invalid flag combination or value, including a `--through` value outside the book's scoped-slice domain; accepted form or accepted values, and command location. |
| `U002` | Required repository, book, manifest, ledger source, or outbound-document input cannot be loaded or is schema-invalid; path and operating-system error category or schema reason. |
| `I001` | Internal invariant failed after valid input; stable category and a statement that retry has no defined remedy. |

Every diagnostic record has every field below. Nullable fields are JSON null,
never omitted. `related` is an ordered array and `remedy` is null only when no
known caller action exists:

```json
{
  "code": "F008",
  "phase": "bytes",
  "message": "expanded bytes differ from authoritative source",
  "primary": {
    "path": "docs/walkthroughs/ordinal-fs-tree/05-mutation-algebra.md",
    "byte": 8421,
    "line": 214,
    "column": 1
  },
  "fragment_id": "ops-insert-plan",
  "root_id": "source-operations",
  "source": {
    "path": "crates/ordinal-fs-tree/src/ops.rs",
    "byte": 9012,
    "line": 231
  },
  "related": [
    {
      "path": "docs/walkthroughs/ordinal-fs-tree/source-index.md",
      "byte": 18002,
      "line": 311,
      "column": 1,
      "label": "root insertion"
    }
  ],
  "remedy": "restore the literal bytes from the declared source range"
}
```

JSON output is one versioned envelope. It uses status `valid`, `findings`,
`invocation-error`, or `internal-error`; those statuses map to exits 0, 1, 2, and
3. `valid` is true only for status `valid`. Scope and coverage are null for
invocation/internal errors that occur before they can be established. The
diagnostic array always contains the ordered records, including `U001`, `U002`,
or `I001` for non-validation failures:

```json
{
  "schema": 1,
  "status": "valid",
  "valid": true,
  "book": "ordinal-fs-tree",
  "scope": { "kind": "through", "slice": "read-path-k14" },
  "coverage": {
    "files": 15,
    "resolved_lines": 3057,
    "deferred_lines": 3872,
    "final": false
  },
  "diagnostics": []
}
```

Text renders each diagnostic without wrapping as `CODE path:line:column:
message`. A record with null line and column renders as `CODE path: message`.
Each related location follows on its own line as `  related LABEL
path:line:column`; a non-null remedy is the final line `  remedy: TEXT`. Records
are separated by one LF and the report ends in LF. Command-wide text failures use
virtual paths `<command>` or `<internal>`.

Fixtures cover every diagnostic class, LF/trailing-space/final-newline drift, a
valid later defer, a defer whose named slice never fills it, a definition present
while its parent still says defer, a manifest declaring a root the corpus rule
does not derive, and a corpus rule deriving a file the manifest does not declare.
Tests also prove repeat runs yield byte-identical text and JSON output.

## Markdown and link validator contract

Markdown validation is a separate check behind the same command and report. It
consumes the shared lexer's page, ordinary-fence, and fragment-boundary tokens;
it never recognizes those constructs independently and never compares source
bytes. It checks:

- the recursive directory inventory against the manifest's page list plus
  `walkthrough.toml`, and the canonical prefix or final page order;
- one H1 per page matching the manifest title, no heading-level skips, and
  required explicit anchors;
- unique anchors within each page;
- exact top and bottom navigation, correct adjacency, and contents links;
- `README.md` prefix/final contents and reachability of both indexes;
- the presence of a `[guide]` citation in `README.md` when the manifest declares
  a guide path, and its absence when the manifest declares `omitted`;
- source-index and concept-index links to existing pages and explicit anchors;
- every relative file and page link, including its explicit Markdown anchor and
  its membership of the permitted-target set;
- every anchor declared in `[guide]` or `[[glossary]]` existing in its target
  document as an explicit anchor; and
- the prohibition on untracked `rust` or `toml` fences reported by the shared
  lexer.

Scoped mode permits later planned pages only as plain text in the contents. It
does not permit missing page links or missing future anchors. Final mode requires
the book directory to contain exactly `walkthrough.toml` and the manifest's
declared pages as regular files, and no other file, directory, symlink, or
special entry at any depth, and every link must resolve.

`concept-index.md` is curated optional navigation, not an exhaustive concept
registry and not an authority for repetition decisions. Each entry is one
descriptive local Markdown link; link syntax and targets are checked like every
other book link. Entry selection and reader-useful ordering remain editorial.

## Prose contract

Mechanical validation deliberately does not claim to prove prose quality or
technical truth. Author and reviewer apply these operational criteria, and they
are the same for every book.

### Audience

A reader who knows Rust and jj and has driven a grove. Grove's vocabulary is
**linked to the glossary, never re-taught**. A book explains surprising uses of
Rust or operating-system behavior in the crate it documents, but does not teach
ownership, traits, iterators, filesystem paths, directories, advisory locks,
command-line parsing, or standard test mechanics from first principles. It does
explain codebase-specific lifetime choices, guard consumption, filename
identity, locking scope, effect order, and rollback semantics.

The relocated `ordinal-fs-tree` book states a narrower audience of its own — a
reader proficient in Rust, common crates, and operating-system APIs, with no
grove experience assumed — for the reason given under *Outbound links*. Audience
is therefore per-book prose, stated in each `README.md`; what this section fixes
is the default and the never-re-teach rule.

### Self-containedness

The whole book supplies every fact required to understand its claims. Repository
docs, ADRs, tests, formal models, and research are evidence for authors, never
prerequisites for readers — the guide and the glossary are entry and vocabulary,
not missing chapters. A page is independently intelligible at its entry point
when it states its main claim, names its required earlier concepts, and restates
the small codebase-specific behavior needed to connect them. It need not repeat
the definitions of ordinary Rust or filesystem concepts assumed by the audience.

For each chapter, the reviewer reads from that H1 without opening a repository
document and answers four questions: What layer or seam is this page about? What
input becomes what output? Which invariant or failure boundary governs that
change? What does the page's complete example produce? An answer that depends
only on a cross-reference is a self-containedness finding. Every early-use row
whose First use is on the page must also be stated at that anchor.

### Direct declarative style

Sentences identify the actor, input, behavior, result, and invariant when those
distinctions matter. Prose does not use rhetorical questions, narrative suspense,
metaphors, idioms, emotive language, or persuasive framing. It labels refusal,
environmental failure, rollback failure, and implementation defect as different
categories.

### Local context and repetition

A later page repeats context only when at least one of these is true:

- two previously explained concepts interact in a new way;
- removing the link would make the local claim incomplete;
- the earlier explanation is in a form unsuitable for the current operation;
- a concrete value or artifact is needed to follow the example; or
- the current page has a different immediate goal that changes which details are
  load-bearing.

The repetition is the minimum semantic contract, not copied source or a second
full explanation. A cross-reference supplies navigation or optional depth. If
deleting a link makes the containing claim impossible to understand, the page
must add local context.

### Source-fragment introductions

Every literal fragment's opening directive has a prose paragraph as its nearest
preceding nonblank block: one or more adjacent nonblank lines that are not a
heading, list, table, fence, HTML comment, or fragment directive. Blank lines may
separate the paragraph from the directive; no other nonblank block may intervene.
`M105` enforces that structural predecessor. The paragraph answers five editorial
review questions: Why is this fragment present here? Which actor owns the
behavior? What relevant input becomes what output? Which invariant does it
establish or use? What role does it have in the page's current example? One
paragraph may answer the questions in several sentences; an unanswered question
is an editorial finding.

### Worked examples

Every chapter carries one complete worked example under a named explicit anchor.
Its required anchor and its start-to-observable-end boundary are settled by the
book's structure brief, not here, because they depend on the conceptual order
that brief chose.

What binds every book: the example section shows concrete values — filenames,
identifiers, paths, effects, reports, streams, outcomes — for whatever its
boundary uses. Vocabulary needed for the trace may precede it. **No earlier
section may primarily enumerate three or more public queries, operations, verbs,
error variants, or conformance obligations**; those catalogues follow the
complete example. The reviewer checks that the stated start, transition steps,
and observable end are all present. Where a book's brief reuses one operation
across several chapters, the later chapters reuse that domain with only the local
setup their boundaries need.

Technical review checks claims against source, tests, models, docs, source
coverage, and these contracts. Editorial review checks ordering, cognitive load,
self-containedness, repetition/link choices, examples, and style across the whole
book.

## Assurance

Assurance is machine-held wherever it can be.

- **Every book is gated.** `scripts/check.sh` runs `book-check --final --check
  all` over **every book root under `docs/walkthroughs/`, by discovery** — never
  a named list. A sixth book joins the gate by existing.
- **Every book root is in the curated user-documentation surface**, by discovery,
  so the repository-wide reference sweep covers it. The surface is a set of
  Markdown documents; `walkthrough.toml` is machinery inside a reader-facing
  directory and any surface rule must admit it by name rather than trip over it.
- **Every book has a tested row** in `docs/ARCHITECTURE.md`'s *Documentation
  ownership* table, by discovery.
- **Both discovery checks are seen to fail first.** A discovery check that has
  never been observed red against a book deliberately withheld is
  indistinguishable from a check that finds nothing.

The three obligations above are `book-assurance-surface-k39`'s to implement; they
are stated here because they are properties of the book system rather than of any
one book.

## Test seams

Preferring existing seams to new ones, and the highest seam available:

- **`book-check` itself** is the primary seam: structure, reconstruction, links,
  ledger reconciliation and corpus derivation all report through one command and
  one diagnostic contract. New checks go here rather than into new commands.
- **`scripts/check.sh`** is the umbrella that runs it over every book.
- **`every_repository_markdown_reference_resolves`**
  (`crates/grove/tests/reference_navigation.rs`) sweeps every Markdown file in
  the repository and covers a new book for free. It accepts generated heading
  anchors, so it is the weaker of the two link checks and never a substitute for
  the book contract's explicit-anchor rule.
- **`book-validation`'s own test suite** constructs snapshots and manifests in
  memory; no test needs a repository on disk to exercise the core.

The seam that is **deleted** by this design is
`compiled_corpus_copy_matches_the_normative_spec_tables`, which compared the
compiled constants against normative tables in the one-book specification. Both
of its subjects are gone. Its replacement is the corpus-derivation check, and the
replacement must be seen to fail — against a manifest missing a real root, and
against a manifest declaring a root the rule does not derive — before the old
test is removed.

## Out of scope

**Figures and assets.** No manifest group describes a figure, a diagram, an image
or any asset, and the omission is deliberate rather than pending. The publishing
pipeline's art stage runs by hand in the pilot and its measure says whether art
paid; `figure-contract-k18` produces either the contract or a recorded rejection
afterwards. A group defined now would be machinery ordered ahead of the
measurement meant to justify it. Until that leaf reports, a book uses what
Markdown already gives it and the validator knows nothing about assets.

**Cross-book fragment reuse.** Two books documenting adjacent crates may explain
the same seam; neither may insert the other's fragments. Fragment IDs are unique
within a book and expansion never leaves one.

**Rendering.** No book is compiled, bundled or served. `docs/walkthroughs/` is
read as Markdown, and the walk-away property depends on that staying true.

## Rejected alternatives and limits

### Directives inside literal fences

Embedding insert and defer markers among source bytes makes a compact example,
but it forces the scanner to distinguish notation from possible source and makes
line terminator and indentation replacement semantics harder to inspect. External
whole-line composition keeps literal fences entirely authoritative source bytes.

### A sidecar manifest for the fragment graph

A sidecar carrying parents, children, insertion order or ranges below the
top-level block duplicates what raw Markdown already shows, forces a choice about
which representation to trust when they drift, and ends the property that raw
Markdown alone reconstructs the code. That rejection stands, and property 3
states it positively.

**What changed** is the *authoring contract* — the corpus boundary, the ownership
plan, the page inventory and the slice order — and it changed in two different
ways, which the earlier account of this reversal ran together. The corpus
boundary, the page inventory and the slice order were never in Markdown to be
duplicated: they were compiled into `validator.rs`, and one book could leave the
question of where they belonged unasked. The **top-level ownership plan is
different** — its blocks and ranges were visible in Markdown then and are visible
now, in the Ownership blocks table, so the manifest genuinely duplicates them and
the old clause genuinely reaches them. That duplication is accepted here and
nowhere else, under the trust order *Source and ownership ledger* states: the
manifest is the contract, the directives are the execution, the tables are the
derived index, and a disagreement in either direction is `F009`. The old
rejection's force was that drift would leave an unanswerable question about which
copy is right; the answer is now written down and checked, which is what makes
the reversal safe rather than merely convenient. `walkthrough.toml` is where the
contract belongs for six books. The reasoning is
[`a-book-cannot-witness-its-own-corpus`](../adr/a-book-cannot-witness-its-own-corpus.md);
the TOML spelling the earlier rejection named is deliberately kept, because
changing the format to avoid the appearance of a reversal would misdescribe what
moved.

### Deriving the whole contract from the book directory

Reading the roots, blocks, slices and pages out of `source-index.md` and the
present files needs no new file and honours the old rejection literally. It fails
on scoped proof: `--through` compares the present prefix against a **plan** — the
later-owned blocks that must be deferred, the chapters that must not yet exist —
and a plan is a statement about artifacts that do not exist, so it cannot be read
out of them. It also makes every completeness claim self-referential, since a
book that omits a file omits it from its own description too.

### Whole-file ownership with no planned holes

Whole-file slices minimize author coordination but conflict with real conceptual
splits: a `Cargo.toml` contains both library and consumer concerns, and a module
can interleave read and write concerns. Explicit top-level blocks preserve reader
order and make each real handoff visible.

### Book-generated production source

Treating a book as the source of truth would make expansion capable of
overwriting code and would reverse the review relationship. Validation remains
read-only and compares in memory against the frozen authoritative files.

### Flexible fragment reuse and transforms

Fragment DAGs, aliases, continuation definitions, arbitrary byte cuts,
indentation transforms, templates, and conditionals can reduce repetition in a
general literate-programming system. These books have fixed, line-aligned
corpora. Single-parent source trees make duplicated ownership, reachability,
exact bytes, and scoped progress simpler and deterministic. A future need outside
this shape requires a new manifest `schema` value rather than an implicit
extension.

### Page IDs as slice IDs

Using page IDs for ownership and scope values would remove the historical `-kN`
spelling, but it would make a presentation-level page rename a
fragment-ownership, ledger, manifest and CLI migration. The book system defines a
separate stable slice domain. Its values remain meaningful because the manifest
enumerates them, not because the Grove task tree that originally named them
remains live.

### A richer glob language for the corpus rule

A full glob or ignore-file syntax would express corpus rules this repository does
not have, at the cost of a dependency and of a boundary a reader can no longer
evaluate by inspection. Two forms — an exact path and `<dir>/**/*.<ext>` — cover
every book here. The escape hatch is `[[corpus.add]]` and `[[corpus.exclude]]`,
each carrying a reason, which makes an irregular corpus legible instead of
encoding it in a pattern nobody reads.

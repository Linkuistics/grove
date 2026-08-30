# Ordinal filesystem tree book system

## Status and authority

This specification fixes the structure, source ownership, fragment notation,
authoring checks, and prose contract for the `ordinal-fs-tree` walkthrough
book. The crate source is authoritative. The book quotes that source exactly;
neither the book nor its validator generates or overwrites production files.

The in-scope source corpus is frozen while the book node is active. An accepted
source change requires the affected ownership ranges and fragments to change,
followed by final validation against the new bytes.
The repository's default test run performs final validation of the committed
book against the committed fifteen-file corpus, so a source edit cannot bypass
that comparison by changing only a fixture or an isolated command.

## Intended outcome

The result is a self-contained walkthrough for a reader already proficient in
Rust, common crates, and operating-system APIs. It introduces the code in
reader-dependency order rather than filesystem order and reconstructs every
in-scope source file byte for byte from fragments embedded in the Markdown.

The system has four deliberate properties:

1. The numbered pages form one canonical reading path.
2. Source fragments sit beside the explanation that gives them meaning.
3. Raw Markdown exposes every fragment's parent, children, source range, and
   owning slice without requiring an author-maintained sidecar manifest.
4. Scoped checks prove one authoring increment without confusing a named future
   hole with an accidental unresolved reference; final checks prove the whole
   corpus with no holes.

## Book location and pages

The book lives at `docs/ordinal-fs-tree/book/`:

```text
docs/ordinal-fs-tree/book/
├── README.md
├── 01-orientation.md
├── 02-name-seam.md
├── 03-reference-domain.md
├── 04-read-path.md
├── 05-mutation-algebra.md
├── 06-filesystem-interpreter.md
├── 07-syllabus-cli.md
├── 08-invariants-and-trade-offs.md
├── concept-index.md
└── source-index.md
```

`README.md` is the entry page and canonical contents. The eight numbered pages
are the canonical reading sequence. `concept-index.md` and `source-index.md`
are lookup surfaces, not additional chapters and not alternate explanatory
paths.

The final page inventory and identities are normative:

| Filename | Page ID | Title | Role | Slice | Order | Previous | Next |
|---|---|---|---|---|---:|---|---|
| `README.md` | `contents` | Ordinal filesystem tree | `contents` | — | 0 | — | `orientation` |
| `01-orientation.md` | `orientation` | Orientation | `chapter` | `orientation-k11` | 1 | — | `name-seam` |
| `02-name-seam.md` | `name-seam` | Name seam | `chapter` | `name-seam-k12` | 2 | `orientation` | `reference-domain` |
| `03-reference-domain.md` | `reference-domain` | Reference domain | `chapter` | `reference-domain-k13` | 3 | `name-seam` | `read-path` |
| `04-read-path.md` | `read-path` | Read path | `chapter` | `read-path-k14` | 4 | `reference-domain` | `mutation-algebra` |
| `05-mutation-algebra.md` | `mutation-algebra` | Mutation algebra | `chapter` | `mutation-algebra-k15` | 5 | `read-path` | `filesystem-interpreter` |
| `06-filesystem-interpreter.md` | `filesystem-interpreter` | Filesystem interpreter | `chapter` | `filesystem-interpreter-k16` | 6 | `mutation-algebra` | `syllabus-cli` |
| `07-syllabus-cli.md` | `syllabus-cli` | Syllabus CLI | `chapter` | `syllabus-cli-k17` | 7 | `filesystem-interpreter` | `invariants-and-trade-offs` |
| `08-invariants-and-trade-offs.md` | `invariants-and-trade-offs` | Invariants and trade-offs | `chapter` | `book-assembly-k18` | 8 | `syllabus-cli` | — |
| `concept-index.md` | `concept-index` | Concept index | `lookup` | — | — | — | — |
| `source-index.md` | `source-index` | Source index | `lookup` | — | — | — | — |

The eight Slice values in this table are normative book-system identifiers.
They were derived from the producing work items, but the specification defines
them as opaque tokens: they remain valid after `.grove/` is removed and do not
require a live Grove handle. Slice identity is deliberately distinct from Page
ID so a page rename does not also migrate ownership directives, ledger rows,
and `--through` values.

During scoped authoring, the numbered pages present must be an exact prefix of
the final sequence. `README.md` links every page in that prefix and lists later
planned titles as plain text. A future page is never represented by a broken
link. `concept-index.md` and `source-index.md` exist from the orientation slice;
their content grows with the prefix.

### Navigation

Every numbered page has the same visible navigation line at its top and bottom.
The accepted raw forms are exact Markdown lines:

```markdown
[Contents](README.md)
[Contents](README.md) | [Next: Name seam](02-name-seam.md)
[Previous: Orientation](01-orientation.md) | [Contents](README.md) | [Next: Reference domain](03-reference-domain.md)
[Previous: Syllabus CLI](07-syllabus-cli.md) | [Contents](README.md)
```

The lines are, in order, the one-page-prefix form, first-page form, interior
form, and final-page form. Filenames and labels are substituted from the page
table's Filename and Title. The last page in a scoped prefix uses the
final-page form, even when it has a final successor in the table. Adding a page
changes the preceding page's two navigation lines as well as the contents. In
final mode the eighth page is the only numbered page without Next.

The two lookup indexes link back to `README.md` and do not participate in
Previous/Next order. `README.md` links both indexes as optional lookup.

Every page has exactly one H1 whose text is its page-table Title. Headings do
not skip levels. A section that is a link target has an explicit HTML anchor
immediately before its heading:

```html
<a id="rollback"></a>
## Rollback
```

Anchor IDs match `[a-z][a-z0-9]*(?:-[a-z0-9]+)*` and are unique within their
file. Links never depend on renderer-generated heading slugs.

Every book file declares one page identity on a column-zero line immediately
after its H1. Numbered pages use this exact attribute order and spacing:

```markdown
<!-- book-page id="orientation" slice="orientation-k11" order="1" -->
```

The ID matches the filename stem without its numeric prefix. The slice and
order match the page table. The entry and lookup files use, respectively:

```markdown
<!-- book-page id="contents" role="contents" -->
<!-- book-page id="concept-index" role="lookup" -->
<!-- book-page id="source-index" role="lookup" -->
```

The top navigation line immediately follows the identity on numbered pages.
The bottom navigation line is their final nonblank content. No other
`book-page` attribute, order, spacing, or role is accepted.

## Concept sequence and page responsibilities

### `README.md` — reader contract

State the audience, scope, exclusions, source-authority rule, exact-fragment
claim, canonical page order, lookup paths, and the distinction between scoped
and final completeness. Explain how to recognize a fragment definition,
insertion, source root, and deferred hole without duplicating the full grammar.

### `01-orientation.md` — purpose and one complete operation

Establish the package's purpose and minimum vocabulary: root, entry, leaf,
node, level, ordinal, key, distinguished child, consumer, and operator. Explain
the library/consumer boundary, feature/dependency boundary, and public crate
surface from the non-CLI manifest ranges and `src/lib.rs`.

Then follow one `lesson-insert` operation at low resolution through this exact
control path:

```text
CLI parse and run
→ CLI insert helper
→ fs::write
→ WriteGuard::insert with its captured Snapshot
→ ops::insert, which builds the effects and returns
  Plan::of(effects).guarded(snapshot) as a Decision
→ WriteGuard::run
→ fs::apply::apply
→ Run::step and, on failure, Run::unwind
→ Report or Error
→ CLI stdout/stderr rendering and process exit status
```

The operation is `insert` because it exposes mutable ordinals, stable keys,
highest-first sibling shifts, ordered effects, guarded planning, rollback, and
observable reporting without requiring promotion's exceptional transient
duplicate ordinal/key state.

The trace uses an actual command, an explicit before/after tree, real names and
values, the planned effect order, the resulting report order, stream ownership,
and exit category. It names the exact functions and types above, but it does not
copy source owned by later slices. Each later-owned type receives the minimum
local statement in the early-use ledger below. This preserves the orientation
slice's settled fragment ownership while making the path technically exact.

### `02-name-seam.md` — filename algebra and the consumer seam

Explain ordinal versus key, parsing and composition, `Found`, `Verdict`,
`Species`, `EntryName::Parts`, `Triple`, one-component rendering, and `EntryName`. Connect
the seam to the surrounding read and mutation flow rather than presenting an
API catalogue. Show concrete accepted, foreign, malformed, and reserved names
without yet treating the syllabus vocabulary as a library default.

### `03-reference-domain.md` — one worked consumer

Explain `SyllabusName`, labels, status, lessons, modules, overviews,
distinguished names, positioning, parsing, formatting, recovery advice, and the
conformance kit. Separate assumptions checked by the type system from
properties exercised by reusable conformance checks. Establish examples stable
enough for the remaining pages to reuse with small local restatements.

### `04-read-path.md` — filesystem discovery to immutable views

Follow a complete read from unfollowed directory entries through filename
classification, recursion, deterministic sibling ordering, snapshot building,
and public `ReadGuard` behavior. Explain foreign-name skipping versus
malformed/reserved halting, root versus entry views, levels, traversal, key
lookup, predicates, ancestors, distinguished chains, shared locking, and the
snapshot lifetime. State read errors and their recovery meaning locally.

### `05-mutation-algebra.md` — total decisions without a filesystem

Explain `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, guarded
planning, and `Report`. Derive append, append-many, insert, promote, and rewrite
from one immutable snapshot. Follow the opening insert in full through target
resolution, hole refusal, highest-first shifting, maximum-key allocation,
effect construction, guarding, and report meaning. Locate each refusal at the
decision that creates it and state that refusal performs no effects. Keep the
algebra/filesystem seam explicit.

### `06-filesystem-interpreter.md` — locks, effects, and recovery

Explain shared and exclusive locks, snapshot timing, the consuming write guard,
name-to-path validation, ordered application, effect-specific steps, unwind,
successful rollback, partial rollback, concurrency limits, crash-visible
intermediate states, and the error taxonomy. Follow both a successful insert
and a forward failure. Do not call a multi-effect operation atomic; state what
the exclusive lock hides from cooperating processes and what it cannot protect
against.

### `07-syllabus-cli.md` — the external consumer and full operation

Explain the demonstration consumer outside `src/`, its CLI-only dependency and
binary declaration, every verb, target syntax, filters, record output,
advisories, refusals, errors, exit categories, idempotency, and omitted
features. Keep operator vocabulary distinct from consumer vocabulary.

Return to the exact command and starting tree used by orientation. Resolve the
same insert at full source resolution through parsing, parts construction,
guard, snapshot, algebra, plan, interpreter, report, stdout/stderr, and exit
status. Use links and brief restatements to revisit earlier source; do not
duplicate fragments.

### `08-invariants-and-trade-offs.md` — assembly and synthesis

Assemble the cross-cutting invariants, model evidence, failure/refusal map,
architecture summary, concurrency and recovery limits, and explicit design
trade-offs. This page owns no production source. It closes the early-use and
source ledgers, confirms every deferral has become an insertion, and records the
final verification commands and results.

## Fragment language

### Identifiers and paths

Fragment and source-root IDs match:

```text
[a-z][a-z0-9]*(?:-[a-z0-9]+)*
```

IDs are globally unique across the whole book. Source paths are repository-
relative, use `/`, and must name one of the fifteen ledger paths. A source root
has an ID in the same namespace as ordinary fragments.

Line ranges are 1-based and inclusive. Every boundary is a source-line
boundary. The frozen corpus is UTF-8-compatible, LF-only, ends each file with
LF, and contains no fragment directive collision. A future source change that
invalidates one of those facts requires a grammar decision before authoring
continues.

### Shared byte-level lexer

Fragment and Markdown validation consume the same directive and fenced-range
token stream from one shared byte-level lexer. Neither check rescans comments
or fences independently. The Markdown link scanner additionally recognizes
exact inline-code spans once, using the accepted form in the link contract, so
links inside them remain opaque without making inline code part of the fragment
language. Book files must be valid UTF-8, contain only LF line endings, and end
in LF. The lexer records the zero-based byte offset and 1-based line and column
of every token.

Outside a fence, these are the complete reserved line forms. Every space shown
is one ASCII space, every directive begins at column zero, attribute order is
fixed, decimal values have no leading zero, and no trailing space or unshown
attribute is permitted:

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

`ID` uses the fragment-ID grammar above for fragment directives and the page
IDs in the page table for `book-page`. `SLICE` is one of the eight enumerated
slice IDs in the page table. `PATH` is one of the fifteen exact source paths in
the source-root table. `N` and `M` are positive canonical decimal integers and
`N <= M`. The page `order` is the canonical positive decimal integer from the
page table.

A line beginning with one of the reserved prefixes `<!-- book-page`,
`<!-- source-root`, `<!-- fragment`, `<!-- insert`, `<!-- defer`,
`<!-- /source-root`, or `<!-- /fragment` that is not one exact form is a
`P001` malformed-directive finding. An exact directive in a disallowed context
is `P002`. Other HTML comments are ordinary Markdown and emit no fragment
tokens.

An ordinary fenced block opens at column zero with a run of three or at least
five identical backticks or tildes, an optional nonempty ASCII info token
immediately following the run, and LF. It closes with exactly the same delimiter
run and LF. The info token matches `[A-Za-z0-9][A-Za-z0-9_-]*`; no other
opening-line bytes are accepted in the book subset. The shared lexer treats
every body byte, including directive-looking lines, as opaque Markdown. An
unclosed ordinary fence is `P002`.

Exactly four backticks are reserved for production literals. Immediately after
a `fragment` opening line, one of these exact lines starts a literal:

~~~text
````rust
````toml
~~~

The language must match the source path. The literal ends at this exact
column-zero line:

~~~text
````
~~~

Each delimiter line ends in LF. The closing delimiter must be immediately
followed by `<!-- /fragment -->` and LF. The literal body is opaque raw bytes,
so triple-backtick doc-comment lines and directive-looking source are never
tokenized. No other four-backtick fence is accepted. A
fragment opening followed instead by `insert` or `defer` directives is a
composite. Blank lines, prose, fences, and non-directive comments are forbidden
inside a source root or composite. Roots and fragments cannot nest, closing
directives must match the active construct, and EOF with an active construct is
`P002`.

`P003` covers invalid UTF-8, CR bytes, and a missing final LF. Lexical findings
do not prevent the lexer from resuming at the next LF when that is safe. Both
validators receive the same successfully recognized directives, literal byte
ranges, ordinary-fence ranges, and lexical findings.

### Source roots

Every source file has exactly one root in `source-index.md`:

```markdown
<!-- source-root «source-demo-rs» source="crates/demo/src/demo.rs" lines="1-4" -->
<!-- insert «demo-opening» -->
<!-- defer «demo-tail» owner="syllabus-cli-k17" lines="3-4" -->
<!-- /source-root -->
```

A source root contains only ordered `insert` or `defer` directive lines. It
emits their expansions in order. Its declared range is the complete file.

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

`rust` is used for Rust source and `toml` for Cargo manifests. The fence body is
read from the Markdown file as bytes. The opening and closing fence lines and
all fragment directive lines emit no bytes. The body is not trimmed, dedented,
reindented, interpolated, decoded and re-encoded, or newline-normalized.

Four backticks are fixed because the current source contains runs of three
backticks in documentation comments but no run of four. Unlabelled `rust` or
`toml` fences in book files are rejected so copied production source cannot
silently sit outside the graph. Examples that are not production source use a
different label such as `text` or `console`.

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
column-zero line, names one strictly later slice from the ledger, and declares
the exact contiguous range it reserves. A defer inside a composite or any other
context is `P002`. Its target definition may be absent.
The named owner owns the range immediately; the earlier slice does not receive
credit for it.

When the owning slice lands, it replaces the defer line with the corresponding
`insert` line and adds the definition. An `insert` whose target is absent is an
ordinary unresolved reference and always fails. Final mode rejects every
`defer`, even if a definition with the deferred ID already exists.

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
position in its root remain stable. Descendant ranges must still form a gapless
partition. Line-count credit is assigned to the top-level block only, so
refinement cannot change slice ownership totals.

## Source and ownership ledger

`source-index.md` contains, from the orientation slice onward:

1. the fifteen source-root definitions in the fixed order below;
2. a table of every top-level ownership block, owner, range, line count, and
   current `deferred` or `resolved` state;
3. a fragment index with ID, page, source, range, owner, parent, and direct
   children; and
4. the early-use ledger.

The four tables appear under the exact H2 headings `Source roots`, `Ownership
blocks`, `Fragment index`, and `Early uses`. Their header names and column order
are fixed by the examples in this section. After the heading and one blank line,
the header is the next line, followed by the separator and one or more data
rows; the first non-row line ends the table. Source-root directive blocks follow
the Source roots table before the next H2. A table row is one LF-terminated
physical line. It begins and ends with `|`; cells have exactly one space between
the delimiter and content. Data cells contain no raw `|`, backslash escape, or
line break. Identifiers, paths, ranges, states, and page locations are wrapped
in one backtick on each side. Decimal counts contain ASCII digits and optional
grouping commas. The separator row contains `---` in every cell. Source-root,
ownership, and fragment rows follow source-root order, then ascending source
range, then fragment ID; the early-use order is defined with that table. A
missing, extra, reordered, or malformed column is `F009`.

The source-root and top-level ownership tables in this specification are the
design authority for the fixed corpus and its 26 ownership blocks. The
validator's `ROOTS` and `BLOCKS` constants are a checked compiled copy: the
repository test suite parses these normative tables and compares every field
with the constants. The source-index directives are the authority for the
fragment graph. Its four tables are mandatory derived indexes: the validator
recomputes every relationship and reports `F009` for a row that disagrees with
the compiled corpus copy or directives. Thus raw Markdown provides outward
relationships in parents and inward lookup without an author-maintained second
file format.

The fragment index has this exact schema:

```markdown
| Fragment ID | Page ID | Root ID | Kind | Owner | Source lines | Parent ID | Child IDs |
|---|---|---|---|---|---|---|---|
| `ops-insert` | `mutation-algebra` | `source-operations` | `composite` | `mutation-algebra-k15` | `176-251` | `mutation-operations-source` | `ops-insert-resolve`, `ops-insert-shifts`, `ops-insert-plan` |
```

`Kind` is `root`, `literal`, or `composite`. A root's page is `source-index`,
owner and parent are `—`, and its direct child IDs include resolved inserts and
deferred top-level IDs in source order. A non-root definition must occur in the
numbered page assigned to its owner by the page table; its Page ID must match
that page. A fragment with no children uses `—`; otherwise Child IDs are
backtick-wrapped and separated by comma plus one space. Deferred targets do not
receive non-root fragment rows until defined; their outbound occurrence remains
visible in the root row and ownership table.

### Source roots

| Root ID | Source path | Lines |
|---|---|---:|
| `source-crate-manifest` | `crates/ordinal-fs-tree/Cargo.toml` | 116 |
| `source-syllabus-cli` | `crates/ordinal-fs-tree/bin/syllabus.rs` | 1,439 |
| `source-library` | `crates/ordinal-fs-tree/src/lib.rs` | 94 |
| `source-conformance` | `crates/ordinal-fs-tree/src/conformance.rs` | 636 |
| `source-error` | `crates/ordinal-fs-tree/src/error.rs` | 342 |
| `source-name` | `crates/ordinal-fs-tree/src/name.rs` | 700 |
| `source-operations` | `crates/ordinal-fs-tree/src/ops.rs` | 543 |
| `source-plan` | `crates/ordinal-fs-tree/src/plan.rs` | 568 |
| `source-reference` | `crates/ordinal-fs-tree/src/reference.rs` | 555 |
| `source-report` | `crates/ordinal-fs-tree/src/report.rs` | 152 |
| `source-snapshot` | `crates/ordinal-fs-tree/src/snapshot.rs` | 650 |
| `source-filesystem-module` | `crates/ordinal-fs-tree/src/fs/mod.rs` | 393 |
| `source-filesystem-read` | `crates/ordinal-fs-tree/src/fs/read.rs` | 179 |
| `source-filesystem-apply` | `crates/ordinal-fs-tree/src/fs/apply.rs` | 471 |
| `source-filesystem-lock` | `crates/ordinal-fs-tree/src/fs/lock.rs` | 91 |

### Top-level ownership blocks

The State column below is the required state after `orientation-k11`. A later
owner changes only its own rows from `deferred` to `resolved` when it replaces
the matching defer with an insert and adds the definition.

| Block ID | Root ID | Owner | Source lines | Count | State |
|---|---|---|---|---|---|
| `manifest-package-and-library-dependency` | `source-crate-manifest` | `orientation-k11` | `1-42` | 42 | `resolved` |
| `manifest-cli-feature` | `source-crate-manifest` | `syllabus-cli-k17` | `43-45` | 3 | `deferred` |
| `manifest-library-cli-boundary` | `source-crate-manifest` | `orientation-k11` | `46-61` | 16 | `resolved` |
| `manifest-cli-binary` | `source-crate-manifest` | `syllabus-cli-k17` | `62-65` | 4 | `deferred` |
| `manifest-development-and-release` | `source-crate-manifest` | `orientation-k11` | `66-116` | 51 | `resolved` |
| `syllabus-cli-source` | `source-syllabus-cli` | `syllabus-cli-k17` | `1-1439` | 1,439 | `deferred` |
| `library-crate-surface` | `source-library` | `orientation-k11` | `1-94` | 94 | `resolved` |
| `reference-conformance-source` | `source-conformance` | `reference-domain-k13` | `1-636` | 636 | `deferred` |
| `filesystem-error-source` | `source-error` | `filesystem-interpreter-k16` | `1-342` | 342 | `deferred` |
| `name-seam-source` | `source-name` | `name-seam-k12` | `1-700` | 700 | `deferred` |
| `mutation-operations-source` | `source-operations` | `mutation-algebra-k15` | `1-543` | 543 | `deferred` |
| `mutation-plan-source` | `source-plan` | `mutation-algebra-k15` | `1-568` | 568 | `deferred` |
| `reference-domain-source` | `source-reference` | `reference-domain-k13` | `1-555` | 555 | `deferred` |
| `mutation-report-source` | `source-report` | `mutation-algebra-k15` | `1-152` | 152 | `deferred` |
| `read-snapshot-source` | `source-snapshot` | `read-path-k14` | `1-650` | 650 | `deferred` |
| `filesystem-read-opening` | `source-filesystem-module` | `read-path-k14` | `1-86` | 86 | `deferred` |
| `filesystem-write-acquire` | `source-filesystem-module` | `filesystem-interpreter-k16` | `87-105` | 19 | `deferred` |
| `filesystem-read-acquire-and-guard` | `source-filesystem-module` | `read-path-k14` | `106-131` | 26 | `deferred` |
| `filesystem-write-guard` | `source-filesystem-module` | `filesystem-interpreter-k16` | `132-154` | 23 | `deferred` |
| `filesystem-read-guard-api` | `source-filesystem-module` | `read-path-k14` | `155-168` | 14 | `deferred` |
| `filesystem-write-guard-api` | `source-filesystem-module` | `filesystem-interpreter-k16` | `169-378` | 210 | `deferred` |
| `filesystem-read-deref` | `source-filesystem-module` | `read-path-k14` | `379-386` | 8 | `deferred` |
| `filesystem-write-deref` | `source-filesystem-module` | `filesystem-interpreter-k16` | `387-393` | 7 | `deferred` |
| `read-filesystem-source` | `source-filesystem-read` | `read-path-k14` | `1-179` | 179 | `deferred` |
| `filesystem-interpreter-source` | `source-filesystem-apply` | `filesystem-interpreter-k16` | `1-471` | 471 | `deferred` |
| `filesystem-lock-source` | `source-filesystem-lock` | `filesystem-interpreter-k16` | `1-91` | 91 | `deferred` |

Only two roots have more than one owner. `source-crate-manifest` contains the
five blocks shown above. `source-filesystem-module` contains the eight blocks
shown above. Every other root contains one full-file top-level block.

### Owned-source totals

| Slice | Page | Owned lines |
|---|---|---:|
| `orientation-k11` | `01-orientation.md` | 203 |
| `name-seam-k12` | `02-name-seam.md` | 700 |
| `reference-domain-k13` | `03-reference-domain.md` | 1,191 |
| `read-path-k14` | `04-read-path.md` | 963 |
| `mutation-algebra-k15` | `05-mutation-algebra.md` | 1,263 |
| `filesystem-interpreter-k16` | `06-filesystem-interpreter.md` | 1,163 |
| `syllabus-cli-k17` | `07-syllabus-cli.md` | 1,446 |
| `book-assembly-k18` | `08-invariants-and-trade-offs.md` | 0 |
| **Total** | 15 source roots | **6,929** |

The totals count each source line once at its top-level block and do not count
composite ancestors or lookup prose. A scoped report prints completed lines,
deferred lines, and final=false; it never presents a prefix as exhaustive.

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

The initial ledger has this exact schema and these minimum rows:

| Symbol family | First use | Owner | Minimum local statement | Status |
|---|---|---|---|---|
| `Ordinal`, `Key`, `Found`, `Verdict`, `Species`, `EntryName` | `01-orientation.md#working-vocabulary` | `name-seam-k12` | Ordinal is mutable sibling position, key is stable tree identity, observed file kind is not followed, verdict separates foreign, accepted, and refused names, species controls file versus directory shape, and EntryName is the consumer parsing and composition seam. | `pending` |
| `manifest-cli-binary` | `01-orientation.md#package-contract` | `syllabus-cli-k17` | The binary declaration is CLI-owned and deferred; it maps the demonstration executable to its external consumer source and requires the CLI feature. | `pending` |
| `manifest-cli-feature` | `01-orientation.md#package-contract` | `syllabus-cli-k17` | The optional parser dependency is activated by a later CLI-owned feature range, enabled by default while library consumers may disable default features. | `pending` |
| `Label`, `Status`, `reference::Parts`, `SyllabusName` | `01-orientation.md#insert-tour` | `reference-domain-k13` | These values are the syllabus consumer's vocabulary and seam implementation, not library defaults. | `pending` |
| `Snapshot`, `Entry`, `ReadGuard` | `01-orientation.md#insert-tour` | `read-path-k14` | A snapshot is the immutable parsed tree captured under a guard, entries are borrowed views, and a read guard couples a shared lock, caller-spelled root, and snapshot. | `pending` |
| `Target`, `NewEntry`, `Decision`, `Refusal`, `Plan`, `Effect`, `Report` | `01-orientation.md#insert-tour` | `mutation-algebra-k15` | Target names the root or a stable key, new entry carries opaque parts and optional bytes, every input yields refusal or a guarded ordered plan, and the report records landed effects in its documented orders. | `pending` |
| `WriteGuard`, `Error`, `apply::Faults`, `apply::Run` | `01-orientation.md#insert-tour` | `filesystem-interpreter-k16` | A write guard couples an exclusive lock and snapshot and is consumed by one mutation, errors distinguish refusal, clean rollback, partial rollback, and boundary failure, Faults is a test seam, and Run owns per-plan forward and undo state. | `pending` |
| `Cli`, `Verb`, `Streams`, `Failure` | `01-orientation.md#insert-tour` | `syllabus-cli-k17` | Parsed verbs drive dispatch, stdout is result data, stderr carries advisories and errors, and failure pairs operator-facing text with an exit category. | `pending` |

`Status` is `pending` while the owner page is outside the current prefix and
`explained` from the owner's scoped increment onward. The owning slice changes
only its rows. Rows sort by first-use page order, first-use anchor occurrence in
that page, owner order, then the bytewise Symbol family cell.

Authors add a row before introducing any additional later-owned type. The
validator checks row structure, owner order, page and anchor existence, and
resolution status. Technical review checks that the local statement is present
and sufficient; keyword matching is not a substitute for that judgment.

## Authoring workflow and scoped proof

The orientation slice creates `README.md`, both lookup indexes, the first
numbered page, all fifteen source roots, the complete top-level ownership
ledger, its owned fragment definitions, and defers for every later-owned block.
The committed validator fixture may demonstrate the same grammar, but
orientation is the first check against real book content.

Each later source-owning slice performs this sequence:

1. Read its fixed top-level blocks and early-use rows.
2. Partition each owned block into intent-named, line-aligned literal or
   composite fragments beside the page's explanation.
3. Replace every defer naming that slice with an insert of the same ID.
4. Change the corresponding source-index state from `deferred` to `resolved`
   and add fragment index rows.
5. Add its page to the canonical prefix and update contents and navigation.
6. Complete or add early-use rows and curate concept-index entries.
7. Run both validators through its slice and the relevant crate checks.

A slice may decompose into child work at a conceptual seam. The owning slice
ID and top-level ranges remain unchanged; children divide the page or fragment
work without changing ledger ownership.

`book-assembly-k18` owns no source. It resolves no legitimate production hole:
every source-owning slice must already have replaced its defers. Assembly adds
synthesis, closes indexes, runs final fragment and Markdown validation, checks
the frozen corpus and crate, and records the final evidence.

## Fragment validator contract

The implementation exposes one deep operation, expressed here as interface
pseudocode rather than a required Rust type layout:

```text
validate(BookSnapshot, Request { scope, checks }) -> ValidationReport

scope  = Through(scoped-slice) | Final
checks = Fragments | Markdown | All
```

`scoped-slice` is a typed seven-value domain containing the source-owning
prefixes from `orientation-k11` through `syllabus-cli-k17`. The core cannot
represent an unknown scoped value or the final-only `book-assembly-k18`; parsing
arbitrary command text and reporting invocation errors belong to the CLI.

`BookSnapshot` contains the recursive book-directory entry inventory, including
which entries are non-regular, and the bytes of the eleven book files and
in-scope source files under explicit repository-relative paths. The core
validator has no clock, VCS, network, environment, or directory-discovery
dependency. A thin CLI loads the snapshot. Tests construct snapshots in memory.

The CLI requires `--repo PATH`. It resolves a relative value against the process
working directory and uses that directory as the only repository root.
`--book PATH` is required, must be a normalized repository-relative path with
no `..` component, and is joined to `--repo`; all ledger source paths are joined
to the same root. The book root must be a real directory rather than a symlink.
Inventory traversal records symlinks and other non-regular children but never
follows them. An unreadable unexpected child directory remains an inventory
entry and does not prevent its `M101` finding. The validator neither searches
ancestors nor consults VCS.

During the fragment-validator stage, the runnable author command is:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --through read-path-k14 \
  --check all
```

The corresponding runnable final command is:

```console
cargo run --quiet -p book-validation --bin book-check -- \
  --repo . \
  --book docs/ordinal-fs-tree/book \
  --final \
  --check all
```

`--through` and `--final` are mutually exclusive and one is required. `--check`
accepts `fragments`, `markdown`, or `all` and defaults to `all`. `--output`
accepts `text` or `json` and defaults to `text`.

The accepted `--through` values, in order, are `orientation-k11`,
`name-seam-k12`, `reference-domain-k13`, `read-path-k14`,
`mutation-algebra-k15`, `filesystem-interpreter-k16`, and
`syllabus-cli-k17`. They correspond to page prefixes of lengths one through
seven. `book-assembly-k18` is final-only; passing it to `--through`, or passing
an unknown slice, is an invocation error with exit status 2.

`book-check --help` contains the runnable scoped and final invocations above,
lists all three check selections, explains that validation is read-only, lists
the exit statuses, and describes the JSON schema. The command is always
non-interactive, never starts a pager or spinner, and emits no color. Validation
reports go to stdout.
Invocation and internal load failures go to stderr in text mode; after
`--output json` has been recognized, they use the same versioned JSON envelope
on stdout and leave stderr empty.

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

- all fifteen source roots and all top-level ledger rows;
- exact definitions and byte comparison for every block owned by the prefix;
- an explicit defer, with the ledger's exact ID, owner, and range, for every
  later-owned block;
- no defer owned by the prefix or by an unknown/non-later slice;
- no early definition of a later-owned block;
- valid global uniqueness for every definition already present; and
- an exact sum of resolved plus deferred ranges for every root.

Known byte segments are compared at their declared source offsets. Deferred
ranges are reported in coverage totals as deferred, not reconstructed and not
successful source coverage. An unresolved insert is an error in every scope.

### Final fragment checks

Final mode requires exactly the fifteen source roots and source paths in this
specification, no extras, every top-level block resolved, no defer directives,
one reachable definition per ID, gapless non-overlapping coverage, and byte-for-
byte equality for every recursively expanded root. The final report must state
15 files, 6,929 lines, zero deferred ranges, and final=true.

### Deterministic diagnostics

The validator collects every independent finding it can safely determine.
Duplicate IDs become ambiguous and are excluded from expansion so they do not
produce misleading cascades. A structurally invalid root skips its byte
comparison while unrelated roots continue.

Diagnostic phases and sort order are fixed:

1. input and directive parsing;
2. source inventory and ledger reconciliation;
3. duplicate definitions and roots;
4. insert and defer validity;
5. cycles and reachability;
6. parent, range, ownership, and coverage validity;
7. byte comparison;
8. Markdown structure and local links.

The phase names in structured output are `parse`, `inventory`, `identity`,
`reference`, `graph`, `coverage`, `bytes`, and `markdown`, in that order.
Within a phase, findings sort by these total-order keys:

1. source-root table index, with null after every root;
2. source byte offset, with null after every offset;
3. canonical page-table index, then lexical path for non-book paths;
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

Stable diagnostic classes include:

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
| `F006` | Missing, extra, duplicated, or wrong-path source root; ledger and source path. |
| `F007` | Missing, overlapping, duplicated, out-of-order, or wrong-owner source range; range and all responsible fragments. |
| `F008` | Literal bytes or expanded root differ from source; first source byte offset, 1-based source line, expected/actual byte or EOF, emitting fragment, owner, and root-to-fragment path. |
| `F009` | Ledger or fragment-index row disagrees with directives; both values and locations. |
| `F010` | Fragment definition appears outside its owner's assigned numbered page; actual and required page. |
| `U001` | Invalid flag combination or value; accepted form and command location. |
| `U002` | Required repository, book, ledger source, or page input cannot be loaded; path and operating-system error category. |
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
    "path": "docs/ordinal-fs-tree/book/05-mutation-algebra.md",
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
      "path": "docs/ordinal-fs-tree/book/source-index.md",
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
`invocation-error`, or `internal-error`; those statuses map to exits 0, 1, 2,
and 3. `valid` is true only for status `valid`. Scope and coverage are null for
invocation/internal errors that occur before they can be established. The
diagnostic array always contains the ordered records, including `U001`, `U002`,
or `I001` for non-validation failures:

```json
{
  "schema": 1,
  "status": "valid",
  "valid": true,
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

Text renders each diagnostic without wrapping as
`CODE path:line:column: message`. A record with null line and column renders as
`CODE path: message`. Each related location follows on its own line as
`  related LABEL path:line:column`; a non-null remedy is the final line
`  remedy: TEXT`. Records are separated by one LF and the report ends in LF.
Command-wide text failures use virtual paths `<command>` or `<internal>`.

Fixtures cover every diagnostic class, LF/trailing-space/final-newline drift,
a valid later defer, a defer whose named slice never fills it, and a definition
present while its parent still says defer. Tests also prove repeat runs yield
byte-identical text and JSON output.

## Markdown and link validator contract

Markdown validation is a separate check behind the same command and report. It
consumes the shared lexer's page, ordinary-fence, and fragment-boundary tokens;
it never recognizes those constructs independently and never compares source
bytes. It checks:

- the recursive fixed file inventory and canonical prefix/final page order;
- one H1 per page, no heading-level skips, and required explicit anchors;
- unique anchors within each page;
- exact top and bottom navigation, correct adjacency, and contents links;
- `README.md` prefix/final contents and reachability of both indexes;
- source-index and concept-index links to existing pages and explicit anchors;
- every relative file and page link, including its explicit Markdown anchor;
- the prohibition on untracked `rust` or `toml` fences reported by the shared
  lexer; and
- paths remaining inside the eleven book files or the fifteen frozen source
  paths. Other repository artifacts are not permitted link targets.

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

Scoped mode permits later planned pages only as plain text in the contents. It
does not permit missing page links or missing future anchors. Final mode
requires the book directory to contain exactly the eleven named regular files
and no other file, directory, symlink, or special entry at any depth, and every
link must resolve.

Markdown diagnostics use codes `M101` for page/inventory shape, `M102` for
heading/anchor shape, `M103` for navigation, `M104` for forbidden source fences,
`M105` for a literal fragment whose nearest preceding nonblank block is not a
paragraph block, and `M201` for local links. They report page, byte offset, line
and column, offending text, and the expected target or structure, and follow
the shared deterministic ordering.

`concept-index.md` is curated optional navigation, not an exhaustive concept
registry and not an authority for repetition decisions. Each entry is one
descriptive local Markdown link; link syntax and targets are checked like every
other book link. Entry selection and reader-useful ordering remain editorial.

## Prose contract

Mechanical validation deliberately does not claim to prove prose quality or
technical truth. Author and reviewer apply these operational criteria.

### Self-containedness

The whole book supplies every fact required to understand its claims. Repository
docs, ADRs, tests, formal models, and research are evidence for authors, never
prerequisites for readers. A page is independently intelligible at its entry
point when it states its main claim, names its required earlier concepts, and
restates the small codebase-specific behavior needed to connect them. It need
not repeat the definitions of ordinary Rust or filesystem concepts assumed by
the audience.

For each numbered page, the reviewer reads from that H1 without opening a
repository document and answers four questions: What layer or seam is this page
about? What input becomes what output? Which invariant or failure boundary
governs that change? What does the page's complete example produce? An answer
that depends only on a cross-reference is a self-containedness finding. Every
early-use row whose First use is on the page must also be stated at that anchor.

### Direct declarative style

Sentences identify the actor, input, behavior, result, and invariant when those
distinctions matter. Prose does not use rhetorical questions, narrative
suspense, metaphors, idioms, emotive language, or persuasive framing. It labels
refusal, environmental failure, rollback failure, and implementation defect as
different categories.

### Local context and repetition

A later page repeats context only when at least one of these is true:

- two previously explained concepts interact in a new way;
- removing the link would make the local claim incomplete;
- the earlier explanation is in a form unsuitable for the current operation;
- a concrete value or artifact is needed to follow the example; or
- the current page has a different immediate goal that changes which details
  are load-bearing.

The repetition is the minimum semantic contract, not copied source or a second
full explanation. A cross-reference supplies navigation or optional depth. If
deleting a link makes the containing claim impossible to understand, the page
must add local context.

### Source-fragment introductions

Every literal fragment's opening directive has a prose paragraph as its nearest
preceding nonblank block: one or more adjacent nonblank lines that are not a
heading, list, table, fence, HTML comment, or fragment directive. Blank lines
may separate the paragraph from the directive; no other nonblank block may
intervene. `M105` enforces that structural predecessor. The paragraph answers
five editorial review questions: Why is this fragment
present here? Which actor owns the behavior? What relevant input becomes what
output? Which invariant does it establish or use? What role does it have in the
page's current example? One paragraph may answer the questions in several
sentences; an unanswered question is an editorial finding.

### Worked examples

These pages contain the named example section:

| Page | Required anchor | Complete example boundary |
|---|---|---|
| `01-orientation.md` | `insert-tour` | One command from before-tree through exit status at low resolution. |
| `02-name-seam.md` | `worked-names` | At least one accepted name and one foreign, malformed, or reserved name through observation, parse verdict, and rendering/recovery consequence. |
| `03-reference-domain.md` | `worked-reference-name` | One syllabus entry through parts, formatting, parsing, species, and relevant conformance obligation. |
| `04-read-path.md` | `worked-read` | One directory tree from unfollowed entries through snapshot construction and one public query result. |
| `05-mutation-algebra.md` | `worked-insert-decision` | The orientation insert from target resolution through refusal-or-guarded-plan and report meaning. |
| `06-filesystem-interpreter.md` | `worked-apply-and-unwind` | The same plan through successful application and a second trace through forward failure and unwind outcome. |
| `07-syllabus-cli.md` | `worked-cli-insert` | The orientation command and starting tree through every source layer, streams, and exit status at full resolution. |

Each section shows concrete filenames, ordinals, keys, parts, paths, effects,
reports, streams, and outcomes when that page's boundary uses them. Vocabulary
needed for the trace may precede it. No earlier section may primarily enumerate
three or more public queries, operations, verbs, error variants, or conformance
obligations; those catalogues follow the complete example. The reviewer checks
that the stated start, transition steps, and observable end are all present.
The orientation and CLI rows use the identical command and starting tree. The
read, mutation, and interpreter rows reuse that domain with only the local setup
their boundaries need.

### Audience boundary

The book explains surprising uses of Rust or operating-system behavior in this
crate, but it does not teach ownership, traits, iterators, filesystem paths,
directories, advisory locks, command-line parsing, or standard test mechanics
from first principles. It does explain codebase-specific lifetime choices,
guard consumption, unfollowed file kinds, filename identity, locking scope,
effect order, and rollback semantics.

Technical review checks claims against source, tests, models, docs, source
coverage, and these contracts. Editorial review checks ordering, cognitive
load, self-containedness, repetition/link choices, examples, and style across
the whole book.

## Rejected alternatives and limits

### Directives inside literal fences

Embedding insert and defer markers among source bytes makes a compact example,
but it forces the scanner to distinguish notation from possible source and
makes line terminator and indentation replacement semantics harder to inspect.
External whole-line composition keeps literal fences entirely authoritative
source bytes.

### A TOML sidecar manifest

A sidecar simplifies parser implementation but duplicates parents, children,
ownership, and ranges already visible in Markdown. Drift would require choosing
which representation to trust, and raw Markdown would no longer be sufficient
to reconstruct the code. The source index therefore carries both directives
and reconciled human-readable tables.

### Whole-file ownership with no planned holes

Whole-file slices minimize author coordination but conflict with the settled
conceptual split: `Cargo.toml` contains both library and CLI concerns, and
`src/fs/mod.rs` interleaves read and write/interpreter concerns. Explicit
top-level blocks preserve reader order and make the two real handoffs visible.

### Orientation owning cross-layer source

Copying narrow fragments from every layer into orientation would make its low-
resolution trace visibly source-backed, but it would violate the settled leaf
boundary and create seven additional cross-slice ownership joins. Exact
identifiers, concrete values, and source-backed later pages provide the same
technical trace with a smaller graph.

### Book-generated production source

Treating the book as the source of truth would make expansion capable of
overwriting code and would reverse the review relationship. Validation remains
read-only and compares in memory against the frozen authoritative files.

### Flexible fragment reuse and transforms

Fragment DAGs, aliases, continuation definitions, arbitrary byte cuts,
indentation transforms, templates, and conditionals can reduce repetition in a
general literate-programming system. This book has a fixed, line-aligned corpus.
Single-parent source trees make duplicated ownership, reachability, exact bytes,
and scoped progress simpler and deterministic. A future need outside this
corpus requires a new format version rather than an implicit extension.

### Page IDs as slice IDs

Using the eight Page IDs for ownership and scope values would remove the
historical `-kN` spelling, but it would make a presentation-level page rename a
fragment-ownership, ledger, fixture, and CLI migration. The book system instead
defines a separate stable Slice domain. Its values remain meaningful because
this specification and the validator enumerate them, not because the Grove task
tree that originally named them remains live.

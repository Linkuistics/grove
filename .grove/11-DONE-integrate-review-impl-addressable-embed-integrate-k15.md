# addressable-embed-integrate-k15

**Integrates:** `addressable-embed-review-k14`

## Goal

Triage and apply the four findings from `addressable-embed-review-k14` before
`embed-wide-gate-k8` or the real classification builds on the parser and CLI
contracts. Reconcile the settled spec where a finding shows that the current
wording cannot hold; do not silently choose a new design in code.


## Context

The producer is `addressable-embed-k7`. The review was inspection-only and did
not re-run its recorded green verification. These findings are copied verbatim
so this integration does not have to reconstruct them against a moved tree.

#### `addressable-embed-review-k14 B1` — High: an ordinary leading thematic break can hide classified bytes as “preamble”

**Anchor:** `src/methodology/parse.rs:302`–`319`, introduced by
`addressable-embed-k7` as the new generic opaque-preamble reader.

`preamble_end` treats any first line whose `trim_end()` is `---` as a preamble
opener and the next such line as its close. But `---` in that position is also a
valid Markdown thematic break. The parser has no discriminator between the two.
For example, in this shape:

```markdown
---
<!-- unit: hidden kinds=* class=triggering -->
prose
---
<!-- unit: visible kinds=* class=triggering -->
```

the `hidden` marker and its prose are excluded from the body without an error;
only `visible` is classified. With no second `---`, the same valid Markdown
opening fails as an unterminated preamble. This is not merely a preference about
frontmatter syntax: it gives an ordinary Markdown spelling a path around the
load-bearing claim that every body byte is classified.

The integration must reconcile the design, not silently reinterpret it: either
make the preamble opener distinguishable, or explicitly reserve/reject a leading
thematic break and pin the collision shape. The current combination — generic,
opaque, and indistinguishable — cannot tell which region the author intended.

#### `addressable-embed-review-k14 B2` — High: fence trimming disagrees with CommonMark in both unsafe directions

**Anchor:** `src/methodology/parse.rs:333`–`349`, introduced by
`addressable-embed-k7` as `Fence::opened_by` / `Fence::is_closed_by`.

Both functions call `trim()`, so any amount of indentation is accepted on an
opening or closing fence. CommonMark permits at most three leading spaces on
either. An over-indented opener can therefore make the parser swallow a later
real unit marker into the preceding unit; an over-indented closer can make the
parser return to neutral early and recognise as a unit a marker that CommonMark
still renders inside code. Either direction defeats the “example markers are
prose, real markers are boundaries” reading rule. The current corpus has no
indented fence, so the existing suite cannot expose it.

The opener is broader in another unpinned way: after counting a three-character
prefix it accepts every suffix. That includes valid info strings, which an
existing fenced-info-string fixture relies on, but also backtick-fence info strings
containing backticks, which CommonMark rejects. The settled spec's shorthand
“three or more of the same character” does not describe that implementation.

Reconcile the parser and spec around one explicit rule and pin the boundary
shapes. If the intended rule is CommonMark, the relevant source is
[CommonMark 0.31.2 §4.5](https://spec.commonmark.org/0.31.2/#fenced-code-blocks):
up to three leading spaces, an optional info string (with the backtick
restriction), and a close of the same character/run length with only trailing
spaces or tabs.

#### `addressable-embed-review-k14 B3` — Medium: multi-unit fetch can erase the next marker's line boundary

**Anchor:** `src/llm_cli.rs:525`–`543`, introduced by
`addressable-embed-k7`; the source of a final unit is accepted through EOF at
`src/methodology/parse.rs:277`–`279` with no trailing-newline requirement.

`cmd_methodology` resolves every requested id and then uses `selected.concat()`.
If the first requested unit is the final unit of a file that has no trailing
newline, the next unit begins immediately after its last prose byte. Its
`<!-- unit: ... -->` marker is no longer a whole line, so the output no longer
has the self-addressing shape the verb exists to deliver. The command supports
multiple ids, while the test exercises only today's newline-terminated corpus.

The “verbatim and framed by nothing” contract and useful multi-fetch output
cannot both hold for this admitted input. Either make a final newline an embed
invariant or revise the fetch framing/spec; pin the no-trailing-newline case.

#### `addressable-embed-review-k14 B4` — Medium: the five-field listing assumes, but does not enforce, delimiter-safe paths

**Anchor:** `src/llm_cli.rs:546`–`568`, introduced by
`addressable-embed-k7`; `src/methodology.rs:82`–`95` copies an arbitrary embedded
path into `Unit.file` unchanged.

The listing argument proves four fields delimiter-safe, then calls the fifth
safe because it is “this repository's own filenames.” Repository filenames are
mutable data, not a grammar: a markdown filename may contain a tab or newline.
Such a file passes the build gate, but its listing row has more than five tab
fields or more than one line. `tests/methodology.rs` would catch that only if the
contributor runs the suite; the binary itself and `cargo build` accept it.

Keep the no-escaping design if desired, but make its premise structural: reject
listing delimiter bytes in embedded paths at the build boundary (and pin them),
or specify and implement escaping. As written, the claimed grammar is an
assumption about today's tree.

## Done when

- B1–B4 are each explicitly triaged. Apply every finding that holds; if one does
  not, record the concrete counter-evidence in this leaf rather than silently
  dropping it.
- The parser, CLI contract, settled spec, and durable glossary/ADR wording agree
  after the repair; no second interpretation survives in comments or tests.
- The boundary shapes named by the findings are pinned by tests that would fail
  on the reviewed implementation.
- Run the producer's full verification after the fixes: `cargo build`,
  `cargo test`, `cargo fmt --check`, and `cargo clippy --all-targets`. Also
  demonstrate the build gate refusing any newly forbidden embed shape where the
  repair adds a build-time invariant.

## Triage

**All four hold; all four are applied.** They are one finding wearing four
hats: a rule stated in the spec as a *shorthand* and implemented as a **broader
predicate** than the shorthand describes — "`---` is preamble" (but `---` is also
a thematic break), "three or more of the same character" (but the code trims any
indentation), "framed by nothing" (but concatenation needs a line boundary the
corpus supplies by luck), "no field can contain a tab" (but a path is data, not a
grammar). Each repair therefore does the same three things: narrow the predicate,
make the narrowing **structural at the build boundary**, and rewrite the spec so
the shorthand stops being load-bearing.

**B1 — reserved, and the hiding half made an error.** The collision is real and
unresolvable from the bytes, so the spelling is **reserved**: a first line of
exactly `---` always opens the opaque preamble. Reserving costs an author
nothing, and the argument is already in the design rather than newly invented — a
body must begin with a unit marker, so a leading thematic break is
`BodyBeforeFirstMarker` in every *other* spelling (`***`, `___`, `----`,
`- - -`); `---` was the one spelling that got swallowed instead of rejected. On
top of that, `Fault::MarkerInPreamble` makes a marker inside the block a build
error, which removes the half the finding is actually about: whatever an
over-long block swallows, it can no longer be a **classified unit**. The residue
recorded in the spec, deliberately not closed: a swallowed region of pure
**prose**, reachable only by writing on line one a construct already erroring in
every other spelling — closing it would mean *interpreting* the block, which is
the one thing the rule exists not to do.

**B2 — CommonMark 0.31.2 §4.5, exactly, both ends.** Opener: at most three
columns of indentation, three or more of one fence character, and an info string
carrying no backtick on a backtick fence. Close: at most three columns, at least
as many of the same character, then only spaces and tabs. The spec now argues why
the exactness is not pedantry — a loose *opener* swallows real markers into the
preceding unit (wrong but **visible**, the id set moves), while a loose *close*
returns to neutral inside a block a reader sees as code and promotes an example
marker to a boundary (wrong and **silent**). `trim()` bought both. The
marker/fence indentation asymmetry the review asked about is kept and argued: it
only ever *withholds* unit-hood, which is the safe direction. What is
deliberately not modelled is container context, stated in the spec rather than
left as a surprise; its residue is one-directional and falls in the visible
direction.

**B3 — the invariant, not the framing.** "Verbatim and framed by nothing" is the
half worth keeping, so `Fault::MissingFinalNewline` supplies what makes it
compatible with a multi-id fetch: an embedded markdown file ends in a newline,
therefore every unit's bytes do, therefore every marker keeps its own line
however many units are concatenated. A separator inserted by the fetch would have
been framing — the thing the contract refuses.

**B4 — the premise made structural.** `Fault::UnlistablePath` rejects any control
character in an embedded path at the build boundary, so the listing's
no-escaping design rests on a property the build enforces rather than on today's
filenames. The check lives in `parse_units` deliberately: that is the **one
implementation both traversals share**, so it cannot hold on the filesystem walk
and not on the embed walk.

### Evidence

- `cargo build`, `cargo test` (**1003 passing**, up from 993), `cargo fmt
  --check` and `cargo clippy --all-targets` all green.
- **The new pins were falsified against the reviewed implementation**: with
  `parse.rs`'s logic reverted to B1/B2/B3/B4's anchors and the new tests left in
  place, exactly the six behaviour-changing tests fail and every pre-existing
  test still passes.
- **The gate was demonstrated refusing each newly forbidden shape**, on the real
  `content/`, then reverted — `content/` is byte-identical:
  - `content/SKILL.md:2:4: this unit marker is inside the leading `---` block…`
  - `content/ADR-FORMAT.md:39:2065: the file does not end in a newline…`
  - `content/two<TAB>fields.md:1:0: this file's path holds U+0009…`
  - `content/SPEC-FORMAT.md:75:3110: this fenced block is never closed…` (a
    closing fence indented four columns, which now correctly closes nothing)
- Reconciled so no second interpretation survives: the spec's *Fence state*,
  *A leading `---` block is opaque preamble*, *A malformed embed fails the build*
  and *`grove-llm methodology` fetches bytes, or lists rows* sections, its
  Requirements scenarios and Test seams; `CONTEXT.md`'s [[Methodology unit]]
  entry; `src/llm_cli.rs`'s `listing_row` and `cmd_methodology` doc comments —
  the latter two carried B4's and B3's exact superseded arguments verbatim.

**No follow-up leaf is cut.** Nothing surfaced that does not serve B1–B4: each
finding's own prescribed remedy was taken, `embed-wide-gate-k8` already owns the
whole-embed classes, and the one question declined here (container-block
modelling) is declined *in the spec*, not deferred as work.

## Notes

- This leaf integrates findings only. New concerns that do not serve B1–B4 go
  to new leaves; if the repair itself no longer fits one focused session,
  decompose this leaf and execute only its first child.

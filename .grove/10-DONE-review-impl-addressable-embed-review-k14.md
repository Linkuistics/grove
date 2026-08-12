# addressable-embed-review-k14

**Reviews:** `addressable-embed-k7`

## Goal

A fresh context asked to **disprove** the marker grammar, the parser, the
per-file build gate, and `grove-llm methodology` — before `embed-wide-gate-k8`
extends the gate onto them and before `classification-k9` writes 139 kB of real
judgement against them.

**Inserted ahead of both**, deliberately. A `review-*` step normally re-derives
from its producer's commit and is content to land wherever `leaf-add` puts it;
this one is not, for the reason the grove brief already gives for
`increments-review-k11`: the two leaves after it *build on* what is being
reviewed, so a review appended behind them reviews a parser that is already
load-bearing and a marking that has already been replaced. The root brief also
names this leaf's output as the workstream's risk concentration — the thing that
gets "its own release and its own pair of eyes first".

Read the producer's commit (named by the `addressable-embed-k7` handle) against
the current source. Inspection only: no test, build, lint or format commands, no
edits to production or test code. Findings only; the `integrate-review-impl` step
you cut if you find something owns every fix.

## Context

### The four specific doubts, in the order they would cost most

1. **The opaque-preamble rule had no fresh reader before this.**
   `ordering-key-placement-k6` introduced it — it was in none of the four
   candidates that leaf was handed — and retired **without** a `review-design`
   leaf, on the grounds that it is the unavoidable mechanical consequence of the
   bounded question it was cut to answer. `addressable-embed-k7` was therefore
   the first session to build against it, and you are the first to read it
   adversarially. The spec section is *A leading `---` block is opaque preamble*.
   Does the implemented rule match it, and does the rule itself hold? Two shapes
   worth thinking about specifically: a file whose *first* line is `---` as an
   ordinary Markdown horizontal rule rather than a frontmatter opener, and the
   interaction (spec says there is none) between `---` and a fence.

2. **A parser's worst failure is going blind to a marker shape, and the design's
   defence is that partition makes that visible.** Check the defence actually
   holds in the code: is there any input where a marker-shaped line is silently
   *neither* recognised nor absorbed into the preceding unit? Read
   `Fence::opened_by` / `is_closed_by` against the spec's fence rule (three or
   more of the same character; the close is at least as long and nothing else)
   and against CommonMark, and ask what the divergences cost. The implementation
   recognises a fence on the **trimmed** line while recognising a marker only
   **unindented** — is that asymmetry right, and is it the safe direction? The
   corpus has no indented fence today, so nothing in the suite would notice.

3. **The build gate is a `#[path]` include, and its traversal is not the runtime
   one.** `build.rs` walks the filesystem; `methodology::units` walks
   `include_dir`'s embed. One *parser*, two *enumerations*. The removed hash had
   an equality test for exactly this shape of risk, and this leaf replaced that
   with `tests/methodology.rs::every_embedded_markdown_file_is_classified`, which
   compares the classified file set against `content/` on disk. Is that the same
   guarantee, a weaker one, or a differently-shaped one? Also: the script emits
   `rerun-if-changed` for the two included sources by hand — is that list
   complete, and what happens to the gate if it is not?

4. **Two claims cannot go red in `cargo test`, so read them by eye.**
   `scripts/release-common.sh`'s `assert_methodology_pairing` inverted (both
   binaries must now carry the embed) and its caller comment in
   `release-build.sh` changed with it. A release-path check fails at the next
   release cut, on a tagged tree, which is why it was scheduled here rather than
   discovered. Confirm the inversion is right and complete, and that
   `tests/provision.rs::the_release_path_scans_for_the_same_marker` still pins
   the two scans together in a way the inversion did not quietly loosen.

### Also worth a look, lower stakes

- **The identity cutover.** `METHODOLOGY_IDENTITY` (a `build.rs` constant) became
  `methodology::identity()` (a `OnceLock` over the linked embed). The hash
  *construction* had to stay byte-identical or every already-provisioned skill
  directory would look foreign and re-extract once. Did it? Its three live
  readers are `--content-hash`, the driver's pairing report, and provisioning's
  stamp write plus `warn_on_foreign_skill_dirs`.
- **The pre-admission dispatch.** `methodology` returns before
  `warn_on_foreign_skill_dirs()` and before `admit_ambient_session`. Is anything
  else in that path skipped that should not be? The test pins the environment
  with a control (`pick` refused in the same environment); check the control is
  really the one that makes the test able to fail.
- **The verb's output contracts.** Fetch concatenates source bytes with no
  separator — is that right when a unit's source does not end in a newline (the
  last unit of a file with no trailing newline)? The listing's five fields are
  claimed un-escapable because no field can contain a tab; is that argued from
  the data or assumed?
- **The trivial marking.** Nine files, one `class=triggering kinds=*` unit each,
  ids derived from filenames. It is legal under the *full* gate (no procedural
  units, so reachability is vacuous; no `defers=`, so nothing can dangle), which
  is what makes splitting the gate safe. Confirm that, and confirm the marker
  placements did not disturb the two vendored files' provenance comments
  (`content/grilling.md`, `content/CONTEXT-FORMAT.md`).

### Where the answers live

- `docs/specs/mandate-delivered-methodology.md` — the settled design. Do not
  re-decide it; a finding is "the code does not do what this says" or "this says
  something that cannot hold", not "I would have designed it differently".
- `docs/adr/one-build-owns-a-session.md` — already written forward for the
  identity cutover; check the code now makes it true.
- `.grove/09-DONE-impl-addressable-embed-k7.md` — the producer's own leaf, whose
  *What is deliberately not here* section is the scope boundary. Whole-embed
  validation, the pinned complete id set, real classification judgement and the
  composer are **out of scope**; finding their absence is not a finding.

## Done when

- Each of the four doubts above is answered — confirmed sound, or written up as a
  finding with the file and line and what it costs.
- Findings are recorded here, with enough detail that the integrating session
  does not have to re-derive them. Anchor each to `path:line` **and** to what the
  reviewed commit did there, since an integration reads a tree that has moved.
- If there is anything worth acting on, cut
  `leaf-add . addressable-embed-integrate --kind integrate-review-impl` — and cut
  it with `leaf-insert` against the first later sibling entry whose subtree still
  holds live work, which on today's tree is `embed-wide-gate-k8`. An integration
  consumes `path:line` coordinates that intervening work moves silently.
- A review that finds nothing cuts nothing and simply retires. That empty session
  is the one the lazy chain exists to remove.

## Notes

- The producer session ran `cargo build`, `cargo test` (993 passing), `cargo fmt
  --check` and `cargo clippy --all-targets`, all green, and demonstrated the gate
  failing by hand on a corrupted marker. Do not re-run them — you are inspecting
  the artifact and its recorded evidence, not re-verifying it.
- The producer did **not** install the build. This is a meta-grove, so the
  observable demo the leaf describes — `grove-llm methodology` from an ordinary
  shell — needs a rebuild and install that would change which binaries the
  *running loop* resolves, which is the human's call rather than a session's. The
  equivalent was exercised from a non-repository temp directory against the built
  binary, in `tests/methodology.rs`.

## Review result

Four findings are worth acting on. No production or test code was edited, and
the producer's recorded build/test/lint evidence was inspected rather than
re-run.

### Findings

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

### Confirmed sound

- Outside B1/B2, the partition defence is real: a candidate marker is either
  parsed or errors; text before the first recognised marker errors; and a missed
  later marker is absorbed into the preceding unit's source rather than dropped.
- The build/runtime traversal check is **differently shaped**, not the removed
  hash equality in disguise, and is sufficient for its stated file-coverage
  claim: `build.rs` and runtime share `parse.rs`, the test compares runtime unit
  files with a separately walked on-disk set, and `build.rs` explicitly tracks
  both `src/methodology/parse.rs` and its only included dependency,
  `src/leaf.rs`. Identity no longer needs a traversal equality test because its
  implementation is the former embed-side hash moved byte-for-byte into
  `methodology` and every reader now calls that one function.
- The release inversion is complete: `assert_methodology_pairing` checks every
  binary argument, `release-build.sh` passes both staged binaries, and the test
  still pins the same content marker used by the release scan.
- The pre-admission dispatch is correctly placed and its control can fail:
  `Methodology` returns before the foreign-skill warning and epoch admission,
  while the paired `pick` invocation under the same stale signal path is required
  to be refused.
- The trivial one-triggering-unit-per-file marking is legal under the future
  whole-embed gate: ids are unique, there are no procedural units or deferrals,
  and reachability is vacuous. The vendored provenance comments remain intact
  immediately after their new marker lines.

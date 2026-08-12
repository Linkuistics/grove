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

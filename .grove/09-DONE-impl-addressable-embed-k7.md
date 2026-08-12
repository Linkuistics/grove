# addressable-embed-k7

## Goal

Make the embed **addressable end to end**: `content/` marked, a shared parser and
a per-file build gate over it, and `grove-llm methodology` serving units out of
the installed binary. The increment ends at an observable listing — a human runs
one command and reads the classification back — not at green scaffolding.

Design: `docs/specs/mandate-delivered-methodology.md` (*Units partition a file*,
*The marker grammar*, *A unit names the procedure it defers to*, *Fence state*,
*A leading `---` block is opaque preamble*, *The file's mandate order is a
comment directive, and it arrives with the composer*, *A malformed embed fails
the build*, *`grove-llm methodology` fetches bytes, or lists rows*, *`grove-llm`
links the embed, and the methodology identity simplifies*, *Test seams*). Do not
re-decide it. The one question it left open has been **settled** by
`ordering-key-placement-k6` and folded into those first two named sections: there
is **no file-level ordering carrier in this grove**, and the parser instead skips
a leading `---`-delimited block uninterpreted.

**This leaf was redrawn by `increments-integrate-k12`.** It and
`embed-wide-gate-k8` were previously cut as `unit-grammar-k7` (parser, gate,
marking — inert by construction) and `methodology-verb-k8` (the verb). That
boundary made the first implementation leaf horizontal: nothing read a unit and
no behaviour changed, which the planning contract forbids
(`content/TASK-FORMAT.md`, *planning*, *review-planning*). The boundary now runs
**per-file addressability with its reader** here, **whole-embed validation** in
`embed-wide-gate-k8`.

## Context

### What lands

- The `methodology` module seam: the parse function over `(path, text)` and the
  embed's unit set. **Not** the composition function — composition belongs to the
  successor grove, and so does everything that measures a composed mandate.
- The marker grammar, exactly as specified: fixed attribute order `id`, `kinds`,
  `class`, `defers`; `kinds` required on triggering and forbidden on procedural;
  `defers` optional on either; ids kebab-case.
- Total partition: every body byte in exactly one unit, marker line included in
  the unit's source, no nesting, no gaps, no close markers.
- Fence state tracked across the whole file body; a marker recognised only as an
  unindented whole line at neutral state; **unbalanced fence state at end of file
  is a build error**.
- **The opaque-preamble rule**: a `---`-delimited block occupying a file's first
  bytes is skipped uninterpreted, is neither required nor rejected, and belongs to
  no unit; body — and therefore partition — begins after it. This is small but it
  is **not optional and not deferrable**, because `content/SKILL.md` carries such
  a block *today* and the per-file gate lands here: without the rule, the first
  build after this leaf rejects the real embed for body text before the first
  marker. It is also the whole reason provisioning keeps working through this
  grove and the next — that block is the `name:`/`description:` every harness
  reads to discover the skill.
- **One parser implementation shared with the crate**, not a second traversal in
  `build.rs`. `build.rs` today is standalone and links nothing; the equality test
  between its hash traversal and `provision::content_hash` exists because that
  duplication was accepted once already. Do not accept it twice.
- `content/` marked throughout with a **trivially correct** marking, and **no
  file-level ordering carrier** — `ordering-key-placement-k6` settled that the
  ordering directive arrives with the composer, in the successor grove. Nothing
  here writes, parses or gates one.
- `grove-llm methodology`, both modes, and the structural change that lets it
  exist: `grove-llm` starts linking the embed.
- The identity cutover and the release-scan inversion, both of which follow from
  linking the embed. See below — they are not deferrable.

### The build gate here is **per file**; the whole-embed half is `embed-wide-gate-k8`

The spec enumerates three malformation classes. This leaf lands everything a
single `(path, text)` decides:

- unparseable marker, unknown attribute, attributes out of order, missing
  `class`, `kinds` on a procedural unit, a `kinds=` member outside the closed
  nineteen, a file declaring no unit, body text before the first marker,
  unbalanced fence at end of file, an unterminated leading `---` block.

`embed-wide-gate-k8` lands what needs every file at once: id uniqueness across
the embed, `defers=` target resolution and its class check, and procedural
reachability. **Splitting the
gate is safe because the trivial marking is valid under the full one** — one
`class=triggering kinds=*` unit per file has no `defers=`, no procedural unit and
no duplicate id — so nothing invalid can ship in the window, and the whole gate
is in place before `classification-k9` makes a single real judgement.

Do not pull the whole-embed classes forward. If they land here, `k8` has nothing
left that is not scaffolding, and this leaf grows past one session again.

### The trivial marking is `class=triggering kinds=*`, not `class=procedural`

`increments-k4` corrects the design leaf here, and the correction survived review.
The plan written before the design review was integrated said *one unit per file,
`class=procedural`* — a legal total partition. It is no longer legal:
`mandate-delivery-integrate-k5` added `defers=` and the reachability rule, and a
corpus of nothing but procedural units satisfies no reachability check.

One unit per file at `class=triggering kinds=*` is the marking that passes: total
partition holds, there are no procedural units so reachability is vacuous, and no
`defers=` target can dangle. It is also *only* legal while composition does not
exist — the per-kind 64 KiB size alarm would reject an all-triggering 139 kB
corpus outright — which is a second reason composition and its alarm stay in the
successor grove.

### The identity cutover happens **here**, because this is where `grove-llm` links the embed

The spec couples the two directly (*`grove-llm` links the embed, and the
methodology identity simplifies*): the compile-time constant exists precisely so
that naming the identity does not link the embed, and once `grove-llm` links it
anyway both binaries can hash it directly. `increments-k4` left the constant here
and scheduled its removal for the successor grove's stage 4, which would preserve
a duplicate traversal for two whole groves after its only justification ended.
`increments-integrate-k12` moved it back and struck it from the successor charter.

What lands with it:

- `build.rs` keeps its `rerun-if-changed` emission and gains the parse gate, and
  **loses the hash** — and with it `GROVE_CONTENT_HASH` and the equality test that
  existed only to keep two traversals in step.
- The direct embed hash serves all three live readers: `--content-hash`, the
  driver's pre-launch pairing report, and — still live in this grove —
  provisioning's stamp write and `warn_on_foreign_skill_dirs`. Provisioning does
  not retire until the successor grove's stage 4, so **none of those may break**.

### Two release-path claims are not deferrable either

Both are consequences of `grove-llm` carrying the content marker for the first
time, and one of them goes red in `cargo test` the moment it does:

- `tests/provision.rs`'s `only_grove_carries_the_embedded_methodology` asserts
  `grove-llm` does **not** carry `CONTENT_MARKER`. It must invert with the link,
  in this commit, or the suite is red.
- `scripts/release-common.sh`'s `assert_methodology_pairing` fails a release if
  `grove-llm` carries the marker; it must assert **both** binaries carry it.
  `the_release_path_scans_for_the_same_marker` pins the two together and moves
  with them. This is a release-path check, so it will not go red during
  development — schedule it here rather than discover it at the next release cut.

The **two relocations** out of `tests/provision.rs` — the instructed-verb scan
and the flat-verb-surface pin — are *not* here. They are whole-embed claims and
they move to `embed-wide-gate-k8` with the rest.

### `INSTRUCTED_VERBS` does not gain `methodology` — verified, do not re-derive

The design leaf scheduled that addition for the verb increment. It is pinned to
what `content/` actually instructs: the test scans the embedded methodology for
`grove-llm <verb>` and asserts the scanned set equals the pinned one.
`increments-integrate-k12` enumerated the embed and found eleven distinct verbs —
`leaf-add`, `complete`, `leaf-insert`, `resolve`, `leaf-add-pair`, `leaf-prune`,
`finish-commit`, `pick`, `leaf-retire`, `leaf-decompose`, `brief-chain` — matching
`INSTRUCTED_VERBS: [&str; 11]` exactly, and **no** occurrence of `grove-llm
methodology`. Nothing in this grove adds one: the framing that tells a session to
fetch a deferred body is `content/MANDATE.md`, which is the successor's. Leave the
pinned set at eleven. If some prose edit here does start naming the verb, add it
then and say so.

### The verb's command contract, and the environment it must work in

`increments-review-k11` B3 flagged that a straightforward new match arm would be
"trapped behind session admission". **That reason is wrong and the correction is
recorded in `increments-integrate-k12`'s triage** — `admit_session` returns
`Ok(None)` immediately when `GROVE_SIGNAL_FILE` is absent or empty
(`src/driver_lease.rs`), so an ordinary shell outside a Grove loop reaches every
verb today. Do not write an acceptance test against the refusal that finding
described; it does not happen, and the test would be green by construction.

What *is* true is narrower and still worth pinning. Whenever `GROVE_SIGNAL_FILE`
**is** set, admission resolves a working tree (`repo::workspace_control`) and
compares it against the epoch record, so a caller in a non-repository directory —
or one holding a stale signal path from a dead launch — is refused before the
verb runs. Those are exactly the environments the successor grove's sessions
fetch deferred procedural bodies from, and a refused *lookup* there is a
split-brain inside one rule.

So the contract, on the merits rather than on B3's reason: **`grove-llm
methodology` reads only the binary's own embed and touches no tree**, which makes
it the same species as `--content-hash`, whose own comment already states the
exemption ("metadata first, and before anything that resolves a working tree").
Dispatch it in that pre-admission block. Note that the block also sits ahead of
`warn_on_foreign_skill_dirs()`, so the verb's stderr stays clean — worth having
for an inspection tool that will be piped, though stdout was never at risk.

**Pin it with a test that can fail**: run the built `grove-llm` from a
non-repository temporary directory **with `GROVE_SIGNAL_FILE` set** to a path
under a directory holding no epoch record, and assert the listing is produced.
That is the environment in which a tree-resolving arm is refused, so the test
goes red if the verb is dispatched after admission. A run with the variable unset
proves nothing.

### What is deliberately not here

- The composer, `content/MANDATE.md`, driver wiring, golden per-kind snapshots,
  the completeness invariant's *mandate* claims, and the size alarm. Successor
  grove.
- Whole-embed validation, the pinned complete id set, and the two relocations.
  `embed-wide-gate-k8`.
- Any real classification judgement. `classification-k9`.
- `content/prompts/continue.md` **stays and stays true**. Provisioning is live for
  the whole of this grove, so the launcher's "use the grove skill" is still the
  fact. It is an embedded markdown file like any other, so it carries a marker
  and nothing else — there is no ordering carrier, and it has no leading `---`
  block. It is not renamed and its text does not change.

## Done when

- `cargo build` fails, by name and with the file and offset, on every **per-file**
  malformation the spec enumerates: unparseable marker, unknown attribute,
  attributes out of order, missing `class`, `kinds` on a procedural unit, a
  `kinds=` member outside the closed nineteen, a file declaring no unit, body text
  before the first marker, unbalanced fence at end of file, and an **unterminated
  leading `---` block**. **There is no file-level ordering carrier and therefore
  no carrier case to gate** — settled by `ordering-key-placement-k6`, do not
  invent one.
- A leading `---`-delimited block is accepted and skipped uninterpreted, belongs
  to no unit, and is neither required nor rejected. `content/SKILL.md` builds with
  its YAML intact and **still discovers as a skill** after a provisioning sweep.
  An *unterminated* one fails, named on its opening line — the same hole the
  unterminated-fence rule closes, in the second place a delimiter can run away
  with a file.
- Parse shapes are pinned on the forms that decide the reading rule — accepted
  **and** ignored alike, including a balanced fenced example marker, an indented
  marker-shaped line, **a file with a leading `---` block and one without**. The
  repository's instructed-verb scanner is the precedent, and its two live holes
  are why. The leading-block pair matters most: a fixture set that never carries
  one goes green on a parser that rejects the real embed.
- The parser is shown able to fail, on a synthetic malformed marker and a
  synthetic well-formed one.
- `grove-llm methodology <id>...` writes those units' source bytes, in the order
  given, byte-for-byte, with no framing.
- `grove-llm methodology` with no argument writes the five-field tab-separated
  listing — `<id>` `<class>` `<scope>` `<defers>` `<file>`, `-` in both optional
  fields — asserted **as a grammar**, not as a golden string. **Every id in the
  listing is accepted as a fetch argument unchanged**: an inventory an agent
  cannot feed back into the verb is prose.
- An unknown id exits non-zero naming the id and pointing at the listing. This is
  an ordinary **runtime user error**, distinct from a bad `defers=` inside the
  embed, which is a contributor's mistake and fails the build.
- The verb runs from a non-repository directory with `GROVE_SIGNAL_FILE` set, and
  a test pins it there.
- `--content-hash`, the driver's pairing report, and provisioning's stamp and
  foreign-directory warning all still work, now off a directly hashed embed;
  `GROVE_CONTENT_HASH`, the `build.rs` hash traversal and the equality test are
  gone.
- `only_grove_carries_the_embedded_methodology` asserts **both** binaries carry
  the marker, `assert_methodology_pairing` inverts with it, and
  `the_release_path_scans_for_the_same_marker` still pins the two together.
- Help output stays in the register the rest of `grove-llm` speaks
  (`tests/help_surfaces.rs`), with no output-format flag on a surface a test
  deliberately pins flat.
- `cargo test` and `cargo build` are green with `content/` trivially marked, and
  the running loop is unaffected.

## Notes

- The demo that earns this leaf its boundary: build, install, and run `grove-llm
  methodology` from an ordinary shell. Nine rows come back and each id fetches its
  file's bytes. That is what makes `classification-k9`'s judgement auditable
  outside `cargo test`, and it is the "useful, verifiable behavior for its
  successor" the planning contract asks of an increment. Do not overclaim it in
  `docs/` or `CHANGELOG.md` — under provisioning a session still receives whole
  documents, so this is an **inspection tool**, not something a session uses yet.
- `--json` is rejected on the merits in the spec's *Out of scope*, and so is a
  `--kind` filter. Neither is a gap to fill; both name the condition that would
  reopen them.
- Tabs need no escaping rule because no field can contain one — ids are
  kebab-case, class and scope are closed sets, and the paths are this
  repository's own filenames. That is a property of the data, so do not invent a
  quoting convention to guard it.
- Two content files are **vendored** — `content/grilling.md` and
  `content/CONTEXT-FORMAT.md`, bundled from `mattpocock/skills` under the notices
  in their leading HTML comments, and `content/CONTEXT-FORMAT.md` is deliberately
  not re-synced upstream. Marking them is required (they are embedded markdown),
  and marker placement must not disturb the provenance comments or invite a
  re-sync that would drop grove's divergence.
- Sharing the parser with `build.rs` decides a small layout question (a `#[path]`
  include, a separate crate, or a workspace member). Whichever way it goes, the
  reason is one implementation, and the header comment should say so where the
  next reader will look. `build.rs`'s header currently cites
  `docs/adr/one-build-owns-a-session.md` for "only `grove` should carry it" — that
  claim dies here, and the spec schedules the citation rather than repairing it
  because the successor grove deletes the code holding it.
- **The opaque-preamble rule is the newest thing in the spec and no fresh context
  has read it.** `ordering-key-placement-k6` introduced it — it was in none of the
  four candidates that leaf was handed — and retired without a `review-design`
  leaf, on the grounds that it is the unavoidable mechanical consequence of the
  bounded question that leaf was cut to answer, not a second decision. You are the
  first session to build against it, and a wrong rule here fails **loudly**, at
  `cargo build`, on the real embed. If it does not hold up — if the block turns out
  to need reading, or interacts with something the spec did not anticipate — that
  is a finding to raise, not a spec to quietly reinterpret.
- This is a large leaf even after the redraw. If it proves bigger than one focused
  session, **decompose it rather than absorbing** — and cut the first child
  end-to-end (parser plus the verb over it), never a parser-only child, which is
  the exact shape the redraw removed.

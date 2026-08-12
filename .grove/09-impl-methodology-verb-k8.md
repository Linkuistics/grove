# methodology-verb-k8

## Goal

Ship `grove-llm methodology`: fetch units' source bytes by id, list them in a
parseable grammar with no argument, and error by name on an unknown id. This is
the increment that makes the embed **addressable from outside the binary**, and
it is what this grove delivers.

Design: `docs/specs/mandate-delivered-methodology.md` (*`grove-llm methodology`
fetches bytes, or lists rows*, *`grove-llm` links the embed*, and the last two
bullets of *Test seams*).

## Context

### What lands

- The verb, both modes. **Fetch**: the named units' source bytes, in the order
  given, verbatim and framed by nothing — the output *is* the methodology, so any
  decoration is driver-authored prose arriving through a second door. **List**:
  one unit per line, tab-separated, five fields in fixed order —
  `<id>` `<class>` `<scope>` `<defers>` `<file>` — with `-` in the scope field of
  a procedural unit and in the defers field of a unit that defers to nothing.
- The unknown-id error: exit non-zero, name the id, direct the caller to the
  listing. This is an ordinary **runtime user error**, distinct from a bad
  `defers=` inside the embed, which is a contributor's mistake and fails the
  build.
- `grove-llm` starts linking the embed. That is the increment's one structural
  change and it is what the two moves below follow from.
- **Two checks relocate out of `tests/provision.rs`**, because the successor
  grove deletes their home and both are claims about the embed rather than about
  provisioning: `the_embedded_methodology_instructs_no_verb_the_embedded_cli_lacks`
  (with `collect_instructed_verbs`, `scan_instructed_verbs`, `INSTRUCTED_VERBS`)
  and `the_grove_llm_verb_surface_is_flat` (with `exposed_verbs`), which is what
  makes the first comparison mean what it claims. Their corpus improves in the
  move: they scan the **embed** rather than a provisioned extraction of it.
- `scripts/release-common.sh`'s `assert_methodology_pairing` **inverts**. It today
  fails a release if `grove-llm` carries `CONTENT_MARKER`; it must assert **both**
  binaries carry it. This is a release-path check, not a `cargo test` one, so it
  will not go red during development — schedule it here rather than discover it at
  the next release cut. `tests/provision.rs`'s
  `the_release_path_scans_for_the_same_marker` and
  `only_grove_carries_the_embedded_methodology` are the two tests that pin the old
  direction; both change with it, and the first relocates with the others.

### `INSTRUCTED_VERBS` does not gain `methodology` here — check before assuming it does

The design leaf scheduled that addition for this increment. It is pinned to
what `content/` actually instructs: the test scans the embedded methodology for
`grove-llm <verb>` and asserts the scanned set equals the pinned one. Nothing in
`content/` instructs `grove-llm methodology` today, and nothing in this grove
needs to — the framing that tells a session to fetch a deferred body is
`content/MANDATE.md`, which belongs to the successor grove. Add the verb to
`INSTRUCTED_VERBS` **when and only when** `content/` starts naming it, and if
that is not in this increment, leave the pinned set at eleven and say so.

### What this verb is for, in this grove

Honestly: it is an **inspection tool**, not something a session uses yet. Under
provisioning a session still receives whole documents, so it has every procedural
body already and has been told nothing about unit ids. What the verb buys now is
that a human — and the successor grove's sessions — can read the classification
straight out of an installed binary. That is what makes
`classification-k9`'s judgement auditable outside `cargo test`, and it is the
"useful, verifiable behavior for its successor" that earns this grove its
boundary. Do not overclaim it in `docs/` or `CHANGELOG.md`.

## Done when

- `grove-llm methodology <id>...` writes those units' source bytes, in the order
  given, byte-for-byte, with no framing.
- `grove-llm methodology` with no argument writes the five-field tab-separated
  listing, and **every id in the listing is accepted as a fetch argument
  unchanged**. The round trip is the point: an inventory an agent cannot feed back
  into the verb is prose.
- The listing is asserted **as a grammar**, not as a golden string — five fields,
  the `-` placeholder in both optional fields.
- An unknown id exits non-zero naming the id and pointing at the listing.
- The two relocated checks run against the embed and still fail on the shapes they
  were pinned against; the flat-verb-surface pin still fails the day a
  `grove-llm` subcommand grows a subcommand.
- The release path asserts the content marker in **both** binaries, and the tests
  pinning the old direction move with it.
- Help output stays in the register the rest of `grove-llm` speaks
  (`tests/help_surfaces.rs`), with no output-format flag on a surface a test
  deliberately pins flat.

## Notes

- `--json` is rejected on the merits in the spec's *Out of scope*, and so is a
  `--kind` filter. Neither is a gap to fill; both name the condition that would
  reopen them.
- Tabs need no escaping rule because no field can contain one — ids are
  kebab-case, class and scope are closed sets, and the paths are this
  repository's own filenames. That is a property of the data, so do not invent a
  quoting convention to guard it.
- This increment runs **before** `classification-k9` deliberately: the verb
  depends on the parser and on *some* marking, not on the marking being right, and
  putting the judgement-heavy leaf last keeps its `review-impl` chain contiguous
  with it in the tree.

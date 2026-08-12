# embed-wide-gate-review-k16

**Reviews:** `embed-wide-gate-k8`

## Goal

A fresh context asked to **disprove** the whole-embed half of the build
gate — id uniqueness, `defers=` resolution and its class check, procedural
reachability — plus the pinned complete unit set and the two relocations out of
`tests/provision.rs`.

**Inserted ahead of the two remaining leaves**, for the reason
`addressable-embed-review-k14` and `increments-review-k11` were: both of them
rewrite the corpus this gate validates. `step-suffix-redundancy-k10` and its
implementation edit `content/SKILL.md` and `content/TASK-FORMAT.md` — 76 kB of
the nine files the gate reads — and `classification-k9` rewrites the marking of
all nine and updates `EMBEDDED_UNITS` batch by batch. A review appended behind
them would be reconciling a historical diff against a tree that had moved, and
worse, `classification-k9` is the leaf whose *safety* this gate is supposed to
supply. Reviewing the safety net after the fall is the wrong order.

Read the producer's commit (named by the `embed-wide-gate-k8` handle) against the
current source. Inspection only: no test, build, lint or format commands, no
edits to production or test code. Findings only; the `integrate-review-impl` step
you cut if you find something owns every fix and all post-fix verification.

## Context

### What landed

- `src/methodology/whole_embed.rs` — new, `#[path]`-included into `build.rs`
  beside `parse.rs`. `check(&[Unit])` runs three checks in dependency order and
  reports the first failure as `file:line:offset: <fault>`.
- `src/methodology/parse.rs` — `Unit` gained an `offset` field, so a whole-embed
  failure is located the same way a per-file one is.
- `src/methodology.rs` — `units()` re-runs `whole_embed::check` over the linked
  embed; `markdown_files()` is new and **public**, and `units()` is now written
  on top of it.
- `build.rs` — `gate()` accumulates units across files and checks the set, with
  a second help block for the whole-embed half.
- `tests/methodology.rs` — `EMBEDDED_UNITS` pinned complete at nine;
  `the_linked_embed_passes_the_whole_embed_gate`; and the relocated
  instructed-verb scan, flat-verb-surface pin and identity-flag test.

### The specific doubts, in the order they would cost most

1. **Reachability's roots are triggering units, not mandates.** The spec says
   *reachable from some kind's mandate by following `defers=` from a triggering
   unit that mandate carries*; the code walks from **every** triggering unit and
   never enumerates a kind. The equivalence argued in `check_reachability`'s doc
   comment is that a triggering unit's scope is `*` or a non-empty kind list —
   both of which the parser enforces — so every triggering unit reaches at least
   one mandate. **Is that equivalence sound, and is it still sound once a
   composer exists?** If it is not, this check is weaker than the requirement it
   claims to implement, and nothing in the suite would say so, because today's
   corpus has no procedural unit at all. This is the doubt with the longest fuse:
   the mandate claims arrive in the successor grove and would be built on top of
   this reading.

2. **A vacuous check reads exactly like a passing one.** The trivial marking has
   nine triggering units, no procedural unit, no `defers=`, and no duplicate id,
   so **every one of these three checks is satisfied by an empty walk over the
   real embed**. The evidence offered instead is (a) ten fixture tests in
   `whole_embed.rs`, and (b) a recorded demonstration of all four classes failing
   `cargo build` against a mutated real `content/`. Ask whether that pair is
   actually load-bearing: are the fixtures shapes the corpus will really take,
   and is there a malformation `classification-k9` could plausibly author that
   none of them covers? Look hard at the fixtures that build a *set* by parsing
   several files and concatenating — is that the same assembly order both real
   callers use?

3. **`units()` now fails where it used to serve.** The runtime path gained a
   check that can turn `grove-llm methodology` from a lookup into an error. The
   argument is that the build already rejected such an embed so the branch is
   unreachable, and that running it makes the invariant true where a consumer
   reads it. **Is the added failure mode worth it?** A session following a
   `defers=` id in the middle of a task is the caller, and the design is
   elsewhere very deliberate about not refusing a session mid-task
   (`docs/adr/one-build-owns-a-session.md` reports rather than gates). Is this
   the same question wearing a different hat, or genuinely different because the
   embed is the binary's own artifact?

4. **`methodology::markdown_files` is new public surface whose only external
   caller is a test.** `src/lib.rs` states the rule — *a public item whose only
   callers are tests stops being module API* — and names two exemptions. This
   claims the seam exemption: production reaches the corpus through
   `include_dir`, which a test cannot open without promoting a runtime dependency
   to a dev one. **Test that claim.** The alternatives not taken were adding
   `include_dir` to `[dev-dependencies]` (it is already in the lock file) and
   reconstructing file text from the units' sources. Is the exemption real, or is
   this the pattern the rule exists to catch?

5. **The relocation changed the scanned corpus.** The instructed-verb scan used
   to walk a **provisioned extraction**; it now walks
   `methodology::markdown_files()`. The claim is that these are the same nine
   files, and the evidence is that `INSTRUCTED_VERBS` is still exactly eleven and
   still passes. **Is the corpus genuinely identical?** Consider what
   `provision_into` writes that the embed's markdown walk does not enumerate, and
   the reverse. If they differ, the check quietly narrowed and the test would not
   have noticed, because the eleven verbs are all mentioned many times over.

6. **A third test moved that the leaf did not enumerate.**
   `the_identity_flag_is_not_part_of_the_agent_verb_surface` went to
   `tests/methodology.rs` too, because both of its operands (`exposed_verbs`,
   `scan_instructed_verbs`) did and it would otherwise not compile. Also,
   `collect_instructed_verbs` was **deleted** rather than relocated — the new
   corpus needs no recursive walk. The leaf named both as things to move. Was
   dissolving one and moving a third the right reading, or did something get lost
   in the difference?

7. **Diagnostics: the second location loses its prefix.** `build.rs` renders
   `content/{error}`, so the primary location reads `content/grilling.md:1:0`
   while a *referenced* one inside the message reads `driving.md:1`. Both are
   embed-relative and the tree has one root, so this was judged cosmetic and left
   alone. Is that right — is a build error a contributor cannot paste into an
   editor still "openable" in the sense the design means?

### What is not yours to review

- The per-file gate classes, the parser, the marker grammar, the verb, the
  identity cutover and the release-scan inversion: `addressable-embed-k7`,
  already reviewed and repaired by `addressable-embed-integrate-k15`.
- Any real classification judgement: `classification-k9`.
- The absence of an ordering-key check. Settled by `ordering-key-placement-k6`
  and restated in this leaf's Context — there is no key in this grove to be
  duplicated, so a check over one would be green by construction. **Do not
  reopen it**; if you think it belongs here, that is a finding against the
  *design record*, not against this implementation.
- The mandate claims of the completeness invariant, golden snapshots and the size
  alarm. All measure a composed mandate, and there is no composer in this grove.

## Done when

- Each doubt above is either dismissed with a reason or recorded as a finding
  with a file, a line, and the shape of the repair.
- Anything found beyond the seven is recorded too — the list is where to look
  first, not a boundary.
- If there are findings worth acting on, an `integrate-review-impl` leaf is cut
  (`grove-llm leaf-insert <first blocking sibling> embed-wide-gate-integrate
  --kind integrate-review-impl`) carrying them verbatim, and inserted where
  `pick` reaches it next so its `path:line` coordinates do not drift under the
  two leaves that follow.
- If there are none, this leaf retires having created nothing. That is the
  outcome the lazy chain exists to make cheap.

## Notes

- The producer's own evidence, so you can check it rather than repeat it:
  `cargo test` **1016 passing** (from 1003), `cargo fmt --check` and
  `cargo clippy --all-targets` green; and all four whole-embed classes
  demonstrated failing `cargo build` against a mutated real `content/`, with
  `content/` verified byte-identical afterwards by content hash. The four
  messages are quoted in the producer's commit message.
- `INSTRUCTED_VERBS` staying at eleven was **verified by
  `increments-integrate-k12` and re-verified by the relocation passing**. Do not
  re-derive it; if you doubt it, doubt the *reader*, which is where doubt 5
  points.

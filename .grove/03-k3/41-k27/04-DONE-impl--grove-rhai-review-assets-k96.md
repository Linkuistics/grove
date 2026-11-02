# grove-rhai-review-assets-k96

## Goal

Port the review adapter, the two review examples and the provider-evidence
wrapper to Rhai. Keep receipt bytes and provider-safety outcomes identical
across runtimes.

## Context

- The "Runtime parity" section of the delivery specification, including the
  one accepted receipt difference.
- `assets/ravel-policy/grove.ts`, `provenance.ts`, `examples/review.ts` and
  `examples/grove-review.ts`, and their cases in the asset checks.
- `docs/grove-provider-evidence.md` owns the receipt and context contract.
- The case table and the Rhai static example of grove-rhai-static-assets-k95.

## Done when

- `grove.rhai`, `provenance.rhai`, `examples/review.rhai` and
  `examples/grove-review.rhai` sit beside their counterparts. Each names its
  runtime and schema. The TypeScript files keep their installed paths.
- The Rhai wrapper writes the receipt bytes of the TypeScript wrapper. It
  keeps the constant member order and joins members that `json_encode` wrote.
- A receipt that the wrapper of one runtime records passes the review loader
  of the other runtime. The case table checks both directions.
- The Rhai helpers test the ECMAScript whitespace set explicitly for blank
  labels. Label, receipt and command checks give the same outcome in both
  runtimes.
- A receipt that escapes an unpaired surrogate passes the TypeScript loader
  and refuses as `creator_line_malformed` in Rhai. Its case keeps the reason.
- `ravel:review` and `ravel:provenance` pass against each build. That includes
  actual Grove retirement and retry sessions with fake agents.
- `docs/grove-provider-evidence.md` holds the runtime-neutral definition of a
  blank label. The assets README describes the Rhai review composition.

## Notes

The typecheck files stay TypeScript only. Controls: a Rhai receipt with a
changed member order must fail the byte comparison. A Rhai loader that trims
with Rhai's own `trim` must fail a U+FEFF or U+0085 case. Watch each fail once.

This port carries provider-diversity enforcement. Earn a `review-impl` unless
the cross-runtime receipt and safety cases cover every changed branch.

grove-ravel-runtime-fixtures-k94 left the six cases of
`crates/grove/tests/creator_lifecycle.rs` behind the guard
`review_assets_unported`. Under the 2.0 runtime they report that and run
nothing. Give them Rhai review policies and remove the guard, so that
`ravel:provenance` runs them against each build.

grove-rhai-static-assets-k95 leaves three handoffs. A Rhai script that composes
the routes of `grove-static.rhai` selects through its `select_route`. Rhai
finds the function of a closure only while a function of its module runs. The
review and provenance tests of `ravel_policy_assets.rs` carry the guard
`review_assets_unported`, which this leaf removes. Under 2.0 the case table runs the
Rhai static example and sample without a wrapper. Wrap them in the Rhai
provenance wrapper, as the table wraps the TypeScript set.

## Decisions (running log)

**2.0 build.** On 2026-11-03 this session installed the 2.0 build from
`Ravel.skill-directory-delivery` at output commit `4f831feb4d0f`, with an empty
working copy, by `task install PREFIX=<scratch>`. Its `bin/ravel` had SHA-256
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`, which
matches the node brief.

**The ECMAScript whitespace set.** Bun 1.4.3 enumerated every code point that
`String.prototype.trim` removes. There are 25: U+0009 to U+000D, U+0020,
U+00A0, U+1680, U+2000 to U+200A, U+2028, U+2029, U+202F, U+205F, U+3000 and
U+FEFF. U+0085 and U+180E are not in the set. The Rhai helpers test
membership in this list, character by character.

**Probes of the 2.0 build.** `json_parse` refuses an escaped unpaired surrogate
as `json_invalid`. `json_encode` writes lowercase `\u0001`, and it writes
U+007F, U+2028 and `/` unescaped, as `JSON.stringify` does. `1.0 == 1` is true
in Rhai, so the receipt checks test the type `i64`. A duplicate JSON key keeps
its last value. A string's `len` counts characters and its `bytes` counts
UTF-8 bytes.

**Closures across modules.** A probe confirmed the rule of k95 and found its
extent. A closure resolves while a function of the module that made it is on
the call stack, at any depth. A closure of the entry script always resolves.
The entry script cannot call a closure of a module directly, and a function
pointer to a module function does not resolve.

**Composition of routes.** `review.rhai` calls each route through the
`select_route` of `static.rhai`, so routes of `static.rhai`, of `review.rhai`
and of the entry script resolve. `grove-static.rhai` gains `within(selection)`,
which calls a closure while a function of that module runs.
`grove-review.rhai` runs `review::select_with` inside it, so the routes of
`grove-static.rhai` resolve too. A closure held in one top-level constant and
named in another fails to load, so a fixture makes its route in a function.

**API names.** The Rhai assets mirror the TypeScript names in snake case:
`with_provenance(selection, version, request)`, `parse_evidence`,
`grove_context(request, reviews)`, `read_creator`, and `select_with(rules,
request, context)` for `reviewSelector(rules).select`. `grove-review.rhai`
adds `load_context_with(rules, request)` for `groveReviewSelector`. Each
example also has a whole `select`, and the two review examples a
`load_context`.

**Origin order.** Rhai keeps map keys in code point order, so the
`creator_origin_unlisted` message of `review.rhai` lists origins in that
order. `review.ts` listed them in insertion order. It now sorts them by code
point, so both messages are the same. This is the one change to a TypeScript
asset.

**Float schema version.** Generic `--context` carries a receipt object. Ravel
1.1 reads `"schemaVersion":1.0` as the number 1, and `review.ts` accepts it.
Ravel 2.0 gives Rhai a float, so `read_creator` writes a float equal to 1 as
the integer 1 before it encodes the canonical receipt. The table has a case.

**Table.** The review cases are in `ravel_asset_cases/review.rs` and join the
one table. Four assets were added: Grove's review example, the generic review
example, owner tables with a gateway route, and the wrapper around an owner
selection stated as JSON. The table now compares the receipt bytes, with the
attempt as `<attempt>` once it is a UUID v4, and the delivered context
without `measured`. Source digests read as `<sha256>`, because Ravel computes
them. Each runtime now wraps the static example and the sample, so their
selections carry receipts too. The five tests of `ravel_policy_assets.rs` are
cases of the table now, and that file is removed.

**Both directions.** `RECORDINGS` lists four selections with labels that test
the encoders: `\u0001`, `\t`, `\b`, `\f`, U+007F, U+2028, `/`, a quotation
mark, a backslash, U+0085 and an astral character. A wrapper case states the
bytes for each, and a loader case reads the same bytes with a fixed attempt.
A test without Ravel checks that every receipt that a Grove loader case
accepts is a wrapper case's receipt.

**Ravel's own label checks.** Ravel of both majors refuses an owner version
and a provider of U+0085 alone, because its own check uses Rust's
whitespace. No wrapper therefore records such a label. The U+0085 cases go
through the declared creator of a task and a generic artifact id instead.

**Surrogate outcome.** On 1.1 the TypeScript loader accepts a receipt with an
escaped unpaired surrogate, as the specification says. Ravel 1.1 then refuses
the delivered context as `context_invalid`, because JSON cannot carry the
string. The Rhai loader refuses the receipt as `creator_line_malformed`. So
neither runtime launches a reviewer. The case states both outcomes and this
reason.

**Controls, each seen to fail.** `rhai_review_controls_fail` copies the asset
tree. A wrapper whose `MEMBERS` put `provider` before `attemptId` gave three
mismatches out of three: two wrapper cases and one loader case. A `blank` that
trims with Rhai's `trim` gave four mismatches out of four: U+FEFF and U+0085
declared, a U+FEFF provider and a U+FEFF receipt version. The real tree gave
none.

**Table runs.** Against the 1.1 keg and the 2.0 prefix, all eight tests of
`ravel_asset_cases` passed. The first 2.0 run found three fixture faults: the
stage of a loader refusal is `context`, the message prefix of such a refusal
names `load_context`, and the closure constant above. The first 1.1 run found
the U+0085 version and the surrogate outcome.

**Coverage gaps closed.** A walk of every branch of the Rhai assets found four
uncovered shapes. The table now has two handles (`producer-x1` and
`producer-k1a`), an attempt with a misplaced dash, and three generic receipt
shapes: an unknown member, a null receipt and recorded evidence without a
receipt. A `try` in `review.rhai` was unreachable, because a value from JSON
always encodes, so it is gone.

**The in-session reviewer.** This session spent the leaf's one reviewer on the
claim that each Rhai asset gives the outcome of its TypeScript counterpart. A
fresh context read the four pairs and Ravel's host sources. Its three findings,
classified:

- D1, valid and actionable: an owner result that is a string, an array or a
  number refused as `provenance_invalid` in TypeScript, and threw in Rhai. The
  Rhai wrapper now reads the provider only from a map. Three cases cover it on
  both builds.
- D2, a contract stated unclearly: a script that passes routes of
  `grove-static.rhai` straight to `review::select_with` throws, because the
  closures do not resolve. The asset headers said so. The specification now
  says so too. The evaluation ends with an error, and nothing launches.
- D3, a visible trade-off: a task file of more than 1,048,576 lines would
  exceed Rhai's array bound. That needs a context bound above Ravel's default,
  and the evaluation fails closed. The code stays as it is.

The reviewer found no difference in the receipt bytes, the ECMAScript space
set, `parse_evidence`, `valid_command`, the handle grammar, the messages or
the remedies.

**No `review-impl`.** The cases of the table and of `creator_lifecycle` cover
every changed branch of the Rhai assets and the one TypeScript change. The fix
of D1 is covered by executable cases. So the work earns no `review-impl` leaf.

**`GROVE_*` sweep.** The first full check failed one test:
`removed_surface` classifies every `GROVE_*` name in the tree, and a test
constant was named `GROVE_REVIEW`. It is `REVIEW_VERSION` now.

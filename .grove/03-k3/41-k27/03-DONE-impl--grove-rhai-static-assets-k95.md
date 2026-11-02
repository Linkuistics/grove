# grove-rhai-static-assets-k95

## Goal

Add the Rhai sample configuration and the Rhai static example. Put one case
table under the checks of both asset sets.

## Context

- The "Caller assets" and "Runtime parity" sections of
  `docs/specs/codex-skill-provisioning.md`.
- `assets/ravel-policy/sample/policy.ts`, `examples/grove-static.ts` and their
  checks in `crates/grove/tests/ravel_policy_assets.rs`.
- Ravel's "Frozen comparison rules" and "Measured reads" in
  `docs/specs/configuration-evaluation.md` at the directory build.
- The runtime-explicit test seam of grove-ravel-runtime-fixtures-k94.

## Done when

- `sample/config.rhai` and `examples/grove-static.rhai` sit beside their
  TypeScript counterparts. Grove ships its own copy of Ravel's static example,
  imported by its path. Each asset names its runtime and schema.
- The Rhai sample keeps the four arrangements, the two modifiers, the default
  choice and the refusals. It reads the choice file with the 4,096-byte bound.
  It reads the jj store path through a measured read. It keeps the secondary
  jj workspace grants.
- The Rhai sample passes `request.prompt` to each harness. For a harness that
  needs explicit directories, it adds one read grant for each `snapshotRoot` in
  `request.skills`. With delivery disabled, it adds no grant.
- One case table drives the static and sample checks of both runtimes. Each
  case states the expected outcome for each runtime. It compares routing
  choices, argument vectors, context values and refusal code, stage, exit and
  remedy.
- The three accepted differences each have a case with both outcomes and the
  reason beside it. They are the oversized choice file, U+FEFF and U+0085, and
  the jj store path that is not UTF-8.
- The TypeScript checks keep their strict typecheck against the explicit 1.1
  build. The Rhai checks launch through public `start-session` of the explicit
  2.0 build with fake agents.
- The assets README states both sets, their runtimes, the layout and how to
  copy each sample. No owner configuration is written.

## Notes

Keep the TypeScript files at their installed paths, byte for byte, unless a
check needs a change. Controls: a changed expected outcome in the table must
fail. A Rhai sample that passes `request.originalPrompt` must fail. A grant
for a `snapshotRoot` with delivery disabled must fail. Watch each fail once.

Earn a `review-impl` if a non-accepted runtime difference remains after the
work.

## Decisions (running log)

**2.0 build.** On 2026-11-03 this session installed the 2.0 build from
`Ravel.skill-directory-delivery` at output commit `4f831feb4d0f`, with an empty
working copy, by `task install PREFIX=<scratch>`. Its `bin/ravel` had SHA-256
`151635d25169e6a76c7eed0a1de64af661b3d77554c9bbef88ab85a59afb823d`, which
matches the node brief.

**Static remedy.** The 1.x `ravel/examples/static` refuses an unlisted kind with
the remedy "add a route for this kind in your copy of the policy". Ravel's
`static.rhai` at `4f831feb4d0f` says "your copy of the configuration script".
The table compares remedies, and the specification keeps each refusal of a
TypeScript counterpart. Grove's copy `examples/static.rhai` therefore changes
that one remedy back to the 1.x words, and its header records the change. Grove's
own assets already say "policy" in their remedies.

**Closure resolution across modules.** A probe under the 2.0 build showed that
an owner table holding the closures of `grove-static.rhai` fails when the
`select_route` of `static.rhai` calls them from the owner's script. Rhai
reports `Function not found: anon$…`. The call succeeds when it passes through
a function of `grove-static.rhai`, also for closures of the owner. So
`grove-static.rhai` exports `select_route`, which delegates to the starter, and
its header tells an owner who extends `ROUTES` to call it. The review example of
grove-rhai-review-assets-k96 composes these routes, as `grove-review.ts`
imports them, and it must select through the same function.

**Case table.** `crates/grove/tests/ravel_asset_cases.rs` holds one table for
the static example, an owner extension of it, and the sample. Each case states
the outcome on 1.1 and on 2.0. It compares the provider, model, effort and
reason, the fake-agent argument vector and the notice's context.

For a refusal
it compares the Ravel code or owner code, stage, exit, remedy and the owner's
message. A case leaves the message out only where a decoder or the system
writes part of it. `ravel:policy:runtime` runs it beside `ravel_policy_assets`.

The four old static and sample tests of `ravel_policy_assets` are gone. Its
review and provenance tests report "not run" under 2.0 until k96, as in
`creator_lifecycle`.

**Accepted differences.** Four cases carry an accepted difference and its
reason: the oversized choice file, U+FEFF, U+0085 and the jj store path that is
not UTF-8. A fifth case covers the jj store file beyond the source bound, which
the specification also accepts. A test without Ravel checks that each accepted
difference has a case with two different outcomes. No other runtime difference
remains, so the work earns no `review-impl`.

**Directory route.** Five cases enable delivery with one local source. On 2.0
Claude receives `--add-dir` for the snapshot root, after any jj grant. Codex and
the notes writer receive none. Each receives the prepared prompt, which the
table identifies by the notice's byte count, the snapshot path and `PROMPT`
inside it. The snapshot root comes from public `skills resolve --json`.

On 1.1
these cases expect the `settings_invalid` refusal of the `skills` key.

**Provenance wrapping.** On 1.1 the table wraps the static example and the
sample in `withProvenance`, as the old tests did. On 2.0 it runs them unwrapped,
because the Rhai wrapper arrives with k96.

**Manifest.** `release.test.sh` requires the archive manifest to equal the
asset tree, and the build copies that tree whole. So `release-common.sh` lists
the three Rhai files now. k100 still owns the installed smoke and the formula.

**Controls, each seen to fail once.** `a_changed_expectation_fails` changes
one selection and one refusal and requires two mismatches, on each build.
`rhai_sample_controls_fail` runs a sample that passes `request.originalPrompt`
and one that grants a directory with delivery disabled. Each must give one
mismatch, and the real sample none.

A manual run of both mutations on the real
sample gave 25 mismatches under 2.0: each Claude case with delivery disabled
and each directory case. The file was then restored to SHA-256 `b1c9ae93…`.

**Lanes.** `task ravel:policy` passed against the 1.1 keg with Bun 1.4.3: the
strict typecheck, the compiler suite and nine runtime tests. With
`RAVEL_RUNTIME=2.0` and the 2.0 prefix it skipped the TypeScript steps and
passed the same nine tests, the five review tests reporting "not run". The
first table run on 1.1 found one wrong expectation of the table, which this
session corrected. The Rhai assets passed their first 2.0 run.

# one-build-owns-a-session-review-k20

**Reviews:** one-build-owns-a-session-k17

## Goal

Try to **disprove** the implementation of
`docs/adr/one-build-owns-a-session.md`. The ADR and the docs were reviewed and
corrected before the code existed (`shared-skill-dir-clobber-review-k16` →
`…-integrate-k19`), so the design is not what is in question. What is in
question is the implementation's own decisions — including two the producer made
that **no document covers**, named below.

Inspection only: read the producer's commit (named by its handle
`one-build-owns-a-session-k17`), the source, the ADR, and the recorded
verification. Do not run build/test/lint commands, edit code, or redo the work.
Findings only.

## Context

The producer reports: `cargo test` 963 passing across 39 binaries, `cargo clippy
--all-targets` clean, `cargo fmt --check` clean. It also reports a mutation check
— removing the three new production call sites and confirming each new test fails
— with two deliberate exceptions noted as vacuous negative controls
(`a_paired_grove_llm_is_reported_as_nothing`,
`an_unstamped_or_absent_skill_directory_is_silent`).

## Specific doubts to attack first

These are the producer's own uncertainties, strongest first. Each is a place a
confident implementation could still be wrong.

1. **`reverify_installed()?` propagates, so a foreign skill directory appearing
   mid-loop now stops the loop.** This is a *new* loop-stop path and the ADR does
   not discuss it. The producer's reasoning was that provisioning has always been
   able to refuse (`provision_target` bails on an unstamped non-empty directory),
   that the start-of-run sweep behaves identically, and that this is Grove's own
   operation rather than a prediction about a session — the line the ADR draws.
   The counter-argument to test: `LoopOutcome::Stopped`'s doc now says pairing is
   "deliberately *not* among the reasons" the loop stops, and a reader could
   reasonably call this a pairing stop wearing provisioning's hat. Is the
   distinction real, or is it a rationalisation? What does a human actually see
   when it fires mid-loop, and is it recoverable?

2. **`warn_on_foreign_skill_dirs()` runs on every verb, and reads `$HOME`.** That
   widens *every* `grove-llm` verb's inputs to include process-global state it
   previously ignored — it broke nine `tests/kind.rs` assertions, fixed by
   pointing their `HOME` at the fixture. Check: is the placement right (before
   the session-epoch guard, so a refused verb still warns)? Does it belong on
   `complete`, the last action of a task, where extra stderr lands next to the
   loop's own output? Is there a verb where the warning is actively harmful — one
   whose stdout a session parses, or one where the extra `$HOME` read could fail
   in a sandbox that previously had no reason to grant it?

3. **`only_grove_carries_the_embedded_methodology` rests on linker
   dead-stripping.** The producer verified the marker is absent from `grove-llm`
   in *both* debug and release on this machine (release: 2.4M `grove` vs 2.0M
   `grove-llm`). But the test asserts a linker behaviour, not a source property,
   and it only ever runs against the debug artifacts `cargo test` builds. Ask
   whether the claim is stated at the right grain, and whether a plausible change
   — an `opt-level`/LTO/`panic` setting, a new `pub` re-export, a future caller in
   `llm_cli` reaching for `provision::content_hash` — could make the shipped
   binary carry the embed while this test stays green.

4. **The identity's payload-only grain.** `build.rs` and `provision::content_hash`
   are two independent traversals held together by one equality test. Confirm the
   two really do agree by construction and not by luck — path separator, path
   rendering (`to_string_lossy` vs `include_dir`'s own), sort order (build.rs
   sorts `(String, Vec<u8>)` tuples; `content_hash` sorts by `&Path`), and the
   length-prefixing. A divergence would be silent in the worst direction: every
   build reporting an identity no directory it wrote could match.

5. **`parse_methodology_identity` is the mismatch/unidentifiable discriminator.**
   Anything it wrongly accepts becomes a *mismatch* claim against a possibly
   correct machine; anything it wrongly rejects becomes "unidentifiable". Check
   the boundary cases the unit test does not: leading/trailing whitespace, CRLF,
   a digest plus a trailing comment, a binary that writes the digest to stderr.

## Also worth checking

- The three diagnostics: do they name the resolved path and both identities in
  every branch, and does the *missing* branch (which has no path) still say
  something actionable? The ADR forbids prescribing `cargo install --path .` as
  *the* remedy; confirm no branch does.
- `resolve_in` treats an empty `PATH` entry as the current directory (POSIX). Is
  that right for a *driver*, whose cwd is the worktree — could it resolve a
  `grove-llm` checked into a repository?
- The removed hard stop: confirm nothing else consumed `checked_grove_llm`'s
  return value or relied on the loop refusing before `.grove/` was created.
- `docs/`, `CONTEXT.md`, `CONTEXT-MAP.md` and `docs/specs/config-driven-sessions.md`
  were written before the code. Verify the code matches them rather than the
  reverse — and if one is wrong, that is a finding, not a licence to edit.
- The `CHANGELOG.md` entry: does it state the given-up property plainly?

## Notes

The producer changed `content/` **not at all**, so this leaf carries none of the
release-boundary coupling the root brief describes.

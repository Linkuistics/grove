# mandate-delivery-k13

**Reviews:** mandate-delivery-k8

## Goal

Inspect the commit that made the composed mandate the thing a session receives
and reduced the launcher to framing. The mechanical claims are covered by tests
that all pass; what is uncovered is **prose and classification**, which is
exactly the residue the design says gets an adversarial pass
(`docs/specs/mandate-delivered-methodology.md`, *The classification is data*).

Inspection only — read the committed diff, the source, the spec and the recorded
evidence. Do not run the suite, edit code, or redo the work; findings go to an
`integrate-review-impl` leaf if there are any worth acting on, and to nothing at
all if there are not.

## Context

The producer's commit names `mandate-delivery-k8`. It changed five things:

1. `content/prompts/continue.md` → `content/MANDATE.md`, `content/prompts/`
   removed, the unit re-bodied and renamed `continue-launcher-framing` →
   `mandate-framing` (kept `kinds=* class=triggering`, kept file `order=1`).
2. `src/loop_driver.rs` `mandate_prompt` now takes `selection.kind` and returns
   `compose(units, kind)` + `\n` + the handle paragraph + the stated VCS;
   `provision::continue_prompt()` is deleted.
3. Tests: `EMBEDDED_UNITS` and `INSTRUCTED_VERBS` updated, the golden
   regenerated (19 rows, one per kind), `exactly_one_launcher_is_embedded_and_provisioned`
   deleted with a comment recording where both its halves went, and three
   `finish_lifecycle` fixtures re-pointed off a `*finish-k*` glob.
4. Prose reconciled in `CONTEXT.md`, `docs/ARCHITECTURE.md`,
   `docs/CONFIGURATION.md`, `docs/specs/config-driven-sessions.md` and
   `docs/specs/mandate-delivered-methodology.md`.
5. `continue.md` added to `tests/legacy_claim_sweep.rs`'s `LEGACY_TOKENS`, with
   three refutation quotations.

Verification the producer actually ran, so you can judge it rather than repeat
it: `cargo test` green (40 binaries, 1034 tests), `cargo fmt --check` clean,
`cargo clippy --all-targets` silent, and one real driver launch into a throwaway
`HOME` and git worktree — an `impl` mandate of **48 083 bytes**, opening with the
`mandate-framing` marker and closing with the handle and VCS paragraphs.

## Done when

Each doubt below is either cleared or written up as a finding. They are ordered
by how little the suite says about them.

- **The framing unit carries no instruction.** That is the reduction's whole
  claim (D1) and nothing tests it. Read `content/MANDATE.md` as a session would:
  is any sentence telling the session to *do* something rather than saying what
  the text it holds is? The `grove-llm methodology <id>` sentence is the one to
  weigh — the spec licenses framing that names the verb serving a deferred body,
  and the line between that and an instruction is the judgement call.
- **The framing unit's claims are true.** It says the mandate is complete with
  respect to triggering conditions and that a `defers=` body is fetchable. Both
  are properties of the composer and the gate; confirm the prose does not
  overstate them, and in particular that "there is no situation you are expected
  to detect for yourself" is not falsified by any `kinds=*` unit that still asks
  a session to notice something.
- **Nothing that was delivered by the launcher is now delivered by nobody.** The
  duplicate inventory was confirmed complete by `specialised-ending-k6` against
  six `kinds=*` units. Re-derive it against the *composed* `impl` mandate rather
  than against the claim: for each of the launcher's five instruction sentences,
  find the unit that now carries it.
- **The deleted test's claims really are re-housed.** `exactly_one_launcher_is_embedded_and_provisioned`
  is gone; the comment in `tests/provision.rs` argues its enumeration half is
  subsumed by `EMBEDDED_UNITS` (every file must declare a unit, so a new launcher
  file fails there) and its drift half is no longer expressible. Test that
  argument — a `content/prompts/foo.md` added tomorrow, does anything red?
- **The `finish_lifecycle` fixture fix is right and complete.** Three fixtures
  matched `*finish-k*` against the whole prompt, which every mandate now contains
  because the methodology names `finish-k<key>` in prose; they now match
  `LAUNCHED_FOR_THE_FINISH_LEAF`. Two questions: is matching the driver's
  "resolve and execute \`finish-k" sentence sound (could a composed mandate ever
  contain that string for another kind?), and is any *other* fixture in the suite
  discriminating on a prompt substring the mandate now supplies? The three that
  broke were found by running the suite, not by a sweep.
- **The prose reconciliation is current-state, not aspirational.** Five surfaces
  changed tense or claim. The one to read hardest is
  `docs/specs/mandate-delivered-methodology.md`: its section heading and three
  paragraphs were moved to past tense on the strength of this slice having
  landed, and a spec that describes an unshipped state is worse than one that is
  merely stale.
- **The legacy-token addition earns its bookkeeping.** `continue.md` joining
  `start.md`/`retire.md` cost three refutation quotations, two of which pin
  sentences in `CONTEXT.md` that a later session may reasonably reword. Is the
  token specific enough (does it risk matching prose that is not about the
  launcher), and are the quotations the right grain?

## Notes

**One branch on session kind still ships, deliberately.** `skill-signal` remains
`kinds=*` and still states both endings; `session-ending-k9` removes it. A
finding that the composed mandate still hands every session an `if` on its own
kind is **already known and already scheduled** — do not raise it.

**The behavioural check is not available here either.** This is a meta-grove
across the build boundary: the driver change reaches no session in this loop
until the binary is rebuilt and installed, and provisioning is still live. Judge
by the composed mandate and the loop-driver seam, never by watching a session
behave differently.

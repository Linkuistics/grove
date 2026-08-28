# keyed-launch-templates-k10

## Goal

Create `crates/keyed-launch` and move the template half into it: a config file of
`key → complete command template`, whole-word expansion into an argv, and the
per-kind just-in-time presence rule that replaces configuration completeness.

## Context

`docs/specs/module-decomposition.md`, decision 7 (the `Vocabulary`, `SlotRule`,
`Requirement`, `Templates`, `Slot`, `Argv`, `ConfigError`, `conformance` half of
the interface) and decision 6 (what the completeness rule becomes). Both are
stated; neither is to be redesigned.

`minimalism-k1` measured `src/session_config.rs` (633 lines) at **81
grove-vocabulary occurrences, 57 of them `kind`** — the lookup key, which a
generic runner needs as an opaque string. The rest is 2 `leaf` and 2 `grove`.
Everything else is already KDL parse, validation with source locations,
shell-word splitting without a shell, whole-word substitution, diagnostics.

`decomposition-k2`'s running log and design-review finding 4 carry the two
corrections this leaf must honour, and they are the sharp part of the work.

## Done when

- `crates/keyed-launch` exists as a workspace member with the decision-7 template
  surface, and grove consumes it.
- **The slot vocabulary is an input to `load`, not to `expand`.** This is what
  keeps decision 6's *document-eager* half true: every current template rule is a
  rule about slot *names* — whole-word substitution, declared slot, required slot
  exactly once, optional slot at most once — and a loader that will not learn the
  names until expansion can check none of them. Supplied at load, the whole of
  both documents is validated before anything is spawned, and expansion is left
  with one obligation: that the values offered fill the declared slots.
- **A key resolves only if the primary file declares it.** The overlay overrides
  and never supplies; where only the overlay declares a key, it does not resolve
  and the refusal names the key **and the primary file that must declare it**.
  This is the per-kind restatement of what the all-nineteen rule bought — a
  second source that cannot introduce a program the operator never chose.
- **Presence becomes just-in-time; nothing else does.** Before writing a leaf of
  kind K and before launching kind K, K must resolve to exactly one complete
  template read whole out of one file. Grove's `Kind::ALL`-derived completeness
  check is deleted here. (`Kind` itself stays a closed enum until
  `open-kind-k20`; this leaf removes the *quantifier*, not the set.)
- `keyed_launch::conformance::check(config, vocabulary)` exists — test seam 3,
  the cross-crate seam that keeps *reusable outside grove* true without a second
  repository.
- The crate's own tests exercise its interface without grove;
  `tests/session_config.rs` shrinks to what is genuinely grove's.
- `docs/adr/complete-session-configuration.md` **amended** — the quantifier
  becomes per-kind and just-in-time. State what is lost precisely: the early
  warning **for a kind not yet reached**, and nothing else. A stale personal
  configuration now fails at the first `leaf-add` of that kind rather than at the
  next tree mutation of any kind.
- `docs/adr/untracked-configuration-delta.md` **amended** — its safety argument
  currently borrows the completeness rule (*"that record's completeness rule
  still binds the personal file whatever a delta says"*). After this it does not.
  The record states the primary-declares rule as **its own** property rather than
  borrowing one.
- `docs/CONFIGURATION.md` reflects both; `cargo test` and
  `cargo clippy --all-targets` clean; `CHANGELOG.md` updated.

## Notes

**Lands green**, and it is independent of every other starting point — the
runner's extraction depends on nothing.

**The name is for the interface, not the behaviour.** `keyed-launch`: the key is
what a consumer names, supervision is what sits behind it. Its vocabulary is
*key*, *template*, *launch*, *child*, *signal*, *escalation*, and it deliberately
avoids **session**, which would add a fourth row to `CONTEXT-MAP.md`'s collision
table. Hold that line while writing doc comments — it is the thing that decays
first.

**Do not move the channel or the spawn here.** `keyed-launch-run-k11` does that.
Splitting the crate's two halves across two leaves is deliberate: the template
half carries two ADR amendments and a safety-property restatement, and the run
half carries process supervision. Either alone is a session; together they are
not.

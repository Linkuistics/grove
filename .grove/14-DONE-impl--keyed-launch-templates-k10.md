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

## Decisions (running log)

**The crate takes the parse/validate/expand half; grove keeps *which files* and
*whether the overlay is admissible*.** `Templates::load(primary, overlay,
vocabulary)` receives two paths and reads them. Delta discovery (the two searched
roots, first-holder-wins) and the trackedness refusal are
`untracked-configuration-delta`'s policy and stay in `src/session_config.rs`:
they are statements about grove's worktree and about version control, and a
domain-free runner that took them would be taking grove's security boundary with
them. So grove vets a candidate path and hands `load` an already-admissible
`Option<&Path>`.

**The unknown-key check dies with the quantifier, and that is the spec's own
enumeration rather than a widening.** Decision 6 lists what stays eager —
*syntax, duplicates, node shape, and every template rule* — and `unknown session
kind` is in none of those four. It could not survive anywhere else either: it is
a claim about a closed key set, and the crate holds no set while grove's own set
is exactly the enumeration decision 5 is deleting. What replaces it is the
primary-declares rule: a key nobody uses costs nothing, and a key that is used
must resolve. `Kind` stays a closed enum for *filenames* (k20 opens that); it
simply no longer constrains configuration keys.

**A slot is named bare in the vocabulary and spelled `${name}` in a template.**
`SlotRule { name: "prompt", .. }` matches the whole word `${prompt}`. The crate
never learns what a name means.

**`expand` requires the values to cover the whole vocabulary, not just the slots
this template happens to use.** The spec leaves expansion one obligation — *that
the values offered fill the slots the vocabulary declared* — and taking it
literally is also the stronger contract: a consumer cannot have a call that
works for one key and fails for its neighbour purely because the two templates
mention different optional slots. Grove offers all four every time, so it costs
nothing.

**The crate's diagnostics say `configuration` and `configuration overlay`, not
`Grove configuration` and `Grove configuration delta`.** The noun is the first
thing that decays when a domain-free crate is written by its first consumer, and
a caller-supplied noun would be interface surface the spec does not have.
Grove's user-visible strings change accordingly, and `docs/CONFIGURATION.md`
follows.

**The presence check is placed by enumerating leaf writers, not by listing
verbs.** The obvious list — the five `grove-llm` verbs that write a leaf — is
complete only as far as the list. Grepping the one constructor
(`task_grow::new_leaf`) and the two lifecycle entry points instead found a sixth
site the list missed: the driver's own `materialize_finish`, which mints the
`finish` leaf. Under the list, a configuration silent about `finish` would have
written that leaf and *then* failed at `expand` — leaving the tree holding a leaf
whose kind cannot launch, which is the state the rule exists to prevent. The
check now runs against the pre-transition load, before the write.

**`leaf-decompose` reads its child's kind leniently.** With no `--kind` the first
child inherits the decomposed leaf's own kind, so the rule has to be asked about
a kind read off a filename. A read that *fails* is left to the verb: `decompose`
refuses a brief, a retired leaf and a malformed name with its own message, and a
presence check that errored first would replace those refusals with a complaint
about configuration.

**The verb-driving test binaries needed a fixture `$HOME`.** Writing a leaf now
consults `~/.config/grove/config.kdl`, and until this leaf every CLI test
inherited the developer's real one — so the suite would have passed on a
configured machine and failed on a fresh checkout, testing the machine rather
than the code either way. `tests/support::fixture_home()` is one per test binary,
built once.

**No framework decision needed verification against a source.** `kdl` 4.7 and
`shell-words` 1.1 are the pinned versions the deleted module already used, and
the parsing code moved verbatim; nothing here chose an API on the strength of
what an API looked like. The one version-sensitive claim in the moved code — that
`jj file untrack` refuses a path that is not already ignored — already carries its
citation at the decision site (`src/session_config.rs`, jj 0.44.0) and is
unchanged.

**This is not a cutover leaf.** Re-derived rather than inherited, by the matrix
`delete-migration-k6` ran: the leaf makes no tree-visible change at all — no
filename grammar moves, no witness file appears or disappears, no `.grove/` entry
changes shape — so there is no cell where the installed 19.3.0 build meets the
tree this leaf leaves and fails. Nothing is released and nothing is installed.

**The safety property was checked by reading, not by a dispatched reviewer.**
This is the leaf's one hard-to-reverse, compiler-uncheckable claim — that the
primary-declares rule preserves what the completeness rule bought — and
`references/impl.md` names exactly that combination as an occasion for the
in-session allowance. The session is under a standing instruction not to dispatch
subagents, so the claim was traced directly instead, the way
`integrate-review-design-decomposition-k5` traced its own two:

- `Templates` exposes `source`, `require`, `expand` and `keys`, all reading the
  one map; an overlay-only key is never inserted into it, and `Argv` has no
  public constructor, so expansion is the only route to a spawnable value and it
  cannot reach a template the primary did not declare.
- Both documents are validated before either is merged, so an overlay cannot
  smuggle a second declaration of a key past the duplicate rule, and a primary
  whose own template is malformed fails the load whatever the overlay says.
- `Kind::label()` and a KDL node name are compared verbatim on both sides, with
  no normalisation, so there is no spelling that resolves to a key the primary
  did not write.

**One widening is accepted rather than hidden.** The trackedness probe now runs
just before `Templates::load`, which reads the primary first, so the window
between *is this delta tracked* and *the delta is read* is a little wider than
when one function did both. It is not the threat: trackedness is VCS state rather
than file bytes, and an attacker able to untrack and retrack inside that window
already has local write access to the checkout. Closing it would mean handing the
crate bytes instead of a path, which puts *which file* back inside a crate that
must not know.

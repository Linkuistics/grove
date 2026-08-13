# mandate-delivery-k14

**Integrates:** `mandate-delivery-k13`, the review of producer
`mandate-delivery-k8`

## Goal

Resolve the four findings from `mandate-delivery-k13`: make the framing unit
state only properties the composition gate establishes, remove static
methodology from the driver-authored handle fact, stop finish fixtures from
inferring kind from a legal stable handle, and reconcile the mandate-delivery
spec's remaining old-current claims.

## Context

### Finding 1 — the framing unit overstates what the mandate proves and carries

`content/MANDATE.md:5-10` says both that there is no situation the session must
detect for itself and that the mandate carries no procedure. Neither follows
from the structural gate. The gate proves that every unit *classified* as
triggering for the kind is included and every unit classified as procedural is
excluded; the classification's correctness is explicitly the review residue.

The delivered text also contradicts the literal claims. `skill-decompose`
requires the session to notice that a new concern surfaced or that the current
item grew beyond one session (`content/SKILL.md:206`), while `skill-bootstrap`
contains the actionable resolve-and-read sequence directly in a triggering unit
(`content/SKILL.md:127`). Similar delivered triggers require the session to
notice pruning and node-close conditions. The session no longer has to discover
*which rule applies without being told the rule exists*; it still has to detect
the situation the delivered condition describes, and triggering units that are
complete as delivered can contain procedure.

Rebody the framing unit around the mechanically true contract: every unit marked
triggering for this kind is present, procedural units are withheld, and a
declared deferral names the body the verb can fetch. Do not turn the framing into
instructions while removing the two absolutes.

### Finding 2 — the runtime “fact” still duplicates static methodology

`src/loop_driver.rs:225` does more than append the selected handle: it tells the
session to “resolve and execute” it and not to call `grove-llm pick`. The composed
mandate already carries `skill-do-not-pick-again`, including the authoritative
selection and the reason a second walk may disagree
(`content/SKILL.md:119-125`), while `skill-bootstrap` supplies the resolve step.
The session therefore receives the same rule twice, once byte-exact from
`content/` and once as driver-authored Rust prose.

That is the drift shape the spec rules out at
`docs/specs/mandate-delivered-methodology.md:570-579`, and it makes the new source
comment's “only those two facts” / “no sentence of methodology lives in Rust”
claim false (`src/loop_driver.rs:196-204`). Reduce the runtime paragraph to the
dynamic handle fact; let the composed units carry what to do with it.

### Finding 3 — the finish fixture pattern identifies a slug, not a session kind

`tests/finish_lifecycle.rs:37-49` calls
`*'resolve and execute \`finish-k'*` a discriminator for the finish sentinel.
It is only a discriminator for a stable handle whose slug is `finish`.
`validate_slug` reserves `BRIEF` and `DONE`, not `finish`
(`src/leaf_id.rs:193-214`), so an ordinary leaf such as
`NN-impl-finish-k42.md` is valid and produces a mandate matching the pattern.
The three fixtures can therefore send a non-finish configured session down the
teardown branch.

Make those fixtures distinguish the configured `finish` target directly — for
example, a finish-specific template/argument — rather than recovering kind from
free slug text in `${prompt}`. This also decouples them from the driver sentence
Finding 2 removes.

### Finding 4 — the spec still calls the removed launcher today's delivery path

The producer correctly moved the `content/MANDATE.md` section to current-state
prose, but the same spec still says “Today's mandate” is a 1.1 kB launcher plus
runtime facts (`docs/specs/mandate-delivered-methodology.md:5-9`) and that the
“current delivery path” reaches sessions only as whole documents in a shared
directory (`docs/specs/mandate-delivered-methodology.md:34-39`). The producer's
recorded 48,083-byte composed launch and the new `mandate_prompt` make both
claims false in its own commit.

Recast those problem statements as the pre-composer state while preserving the
still-current distinction that global provisioning remains live as a transient
second path. Do not move the later provisioning-retirement design into the past
before that separate increment lands.

## Done when

- `content/MANDATE.md` distinguishes structural inclusion from semantic
  detection and does not claim the composed mandate contains no procedure in the
  ordinary-language sense.
- `mandate_prompt` appends only the dynamic handle fact and stated VCS; the
  resolve / authoritative-pick rules have one owner in composed `content/`.
- Finish lifecycle fixtures select their finish-specific configured route
  without matching a legal ordinary work-item handle.
- The spec's Problem section describes the launcher-only path as prior state and
  accurately names composition plus still-live provisioning as the present
  transition.
- Focused verification for the fixes and the repository's ordinary Rust checks
  are green.

## Notes

The other reviewed doubts need no integration work. Naming
`grove-llm methodology <id>` as the capability serving a declared deferral is
framing rather than an imperative by itself. The old launcher's five instruction
sentences remain covered in the `impl` mandate by the six universal units the
spec inventories: `skill-bootstrap`, `skill-decompose`, `skill-retire`,
`skill-commit`, `skill-signal`, and `skill-finish`. The deleted launcher
enumeration is re-housed by the per-file classification check plus the pinned
complete unit-id set, and the old byte-drift comparison has no whole-file prompt
operand left. No other test fixture branches on a prompt substring that newly
arrives through composition. The `continue.md` legacy token has only the three
live-document historical/refutation occurrences its table names; its exact-file
token and per-occurrence quotations earn the bookkeeping. The known
`skill-signal` kind branch belongs to `session-ending-k9` and is not a finding.

Per `review-impl` discipline, this review ran no test, build, lint, or format
command and edited no production or test source.

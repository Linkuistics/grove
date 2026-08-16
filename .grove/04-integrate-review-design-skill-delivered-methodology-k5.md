# skill-delivered-methodology-k5

**Integrates:** skill-delivered-methodology-k3

## Goal

Rework `docs/specs/skill-delivered-methodology.md` to apply the review findings
below before planning consumes it.

## Context

The review found five actionable defects. Preserve the design's settled direction
and its cleared estimates; change only the claims and verification boundaries named
here.

## Findings

### 1. High — trigger strength is the load-bearing claim and has no pre-landing evidence

The spec classifies the old failure as a discipline failure and therefore chooses
prohibition/rationalization apparatus (`docs/specs/skill-delivered-methodology.md:248`),
but then concedes that the observed launcher was only a mild instruction and was
not evidence of a session ignoring a forceful one
(`docs/specs/skill-delivered-methodology.md:259`). Rejecting a runtime read receipt
does not answer whether the proposed wording works before implementation
(`docs/specs/skill-delivered-methodology.md:321`), and making all behavioural
evaluation out of scope leaves the design's only answer to the first measured
failure untested (`docs/specs/skill-delivered-methodology.md:716`). The house rule
requires behaviour-shaping wording to be micro-tested against a no-guidance
control with at least five fresh-context repetitions
(`plugins/linkuistics/skills/authoring-conventions/SKILL.md:83`).

Amend the design to require a cheap RED/control and variant micro-test of the load
instruction before the rewrite ships, including the configured model/harness
targets the workstream will rely on. Keep the human-watched real Grove run as the
end-to-end acceptance check; it should not be the first experiment that can falsify
the central wording claim.

### 2. High — the fact/rule split already admits a duplicate source of truth

The candidate table admits both the handle and its authority as a driver-held fact
and then generalises that overlap into a rule that facts ride the core while rules
stay in the skill (`docs/specs/skill-delivered-methodology.md:121`,
`docs/specs/skill-delivered-methodology.md:135`). Authority is not a launch-varying
fact with no `content/` counterpart: the present methodology separately states
that the driver makes an authoritative pick and that the session must not pick
again (`content/SKILL.md:107`, `content/SKILL.md:120`). The proposed core therefore
duplicates static semantics while the spec claims its prose drift surface is zero
(`docs/specs/skill-delivered-methodology.md:186`), and the same move can smuggle
almost any rule into the core by restating it declaratively.

Close the test: a driver fact is only a launch-varying value that `content/` cannot
know at build time. Static meaning or normative consequences of that value remain
in the skill. Remove "authoritative" from the core's handle fact, or explicitly
make it embedded content and account for that extra shared source.

### 3. Medium — `prompt` cannot replace the `methodology` seam one for one as specified

The test-seam section says `prompt` replaces `methodology` one for one, then in the
same sentence leaves the embed and identity behind in `methodology`
(`docs/specs/skill-delivered-methodology.md:660`,
`docs/specs/skill-delivered-methodology.md:664`). Those are live responsibilities,
not inert helpers: provisioning consumes the whole embed
(`src/methodology.rs:41`), and both per-launch and per-verb pairing checks consume
the identity (`src/methodology.rs:122`, `src/provision.rs:64`). Under the stated
design, `methodology` narrows and `prompt` is a second seam; pretending it is a
replacement leaves ownership, visibility, and the architecture's module table
undecided before planning.

Choose and specify one coherent shape: retain a narrowed embedded-methodology seam
and add `prompt`, or move embed/identity ownership behind an explicitly named
existing seam so `prompt` truly replaces it. Reconcile the test strategy and the
architecture table with that choice.

### 4. Medium — two SHALLs have no defined verification boundary

The progressive-disclosure requirement says `SKILL.md` SHALL contain no procedure
and the loop SHALL fit one page (`docs/specs/skill-delivered-methodology.md:624`,
`docs/specs/skill-delivered-methodology.md:634`), but the test-seam list supplies
only line budgets and a table-of-contents check
(`docs/specs/skill-delivered-methodology.md:690`). Unlike the prompt wording, whose
semantic boundary is explicitly assigned to review
(`docs/specs/skill-delivered-methodology.md:672`), these two limbs read as
machine-held requirements even though "procedure" and "page" have no defined
classifier or measure once unit markers are deleted.

Bound them honestly: define a deterministic line/section measure for the one-page
claim, and name the no-procedure claim as a review obligation with the evidence a
reviewer inspects. Do not imply the corpus-budget tests establish either semantic
claim.

### 5. Low — the reference-file set is not exactly the existing narrowed scopes

The spec calls the ten per-kind reference files "exactly the existing narrowed
marker scopes" and a derivation rather than a judgement
(`docs/specs/skill-delivered-methodology.md:331`), but the current corpus has an
additional distinct narrowed scope: the eighteen-kind `skill-signal` unit
(`content/SIGNAL.md:2`). The table is probably the intended design because signal
is extracted into the guaranteed core, but the derivation as written would yield
eleven scope classes and can misdirect a mechanical rewrite.

State the actual derivation: remove the ending unit into the core first, then
recover the ten distinct non-ending narrowed scope classes for kind references.
Keep the thin `design` reference; direct kind selection justifies the small file.

## Done when

- The spec resolves all five findings without changing the settled delivery
  direction.
- Its requirements and test-seam claims agree about which properties are
  mechanical and which are review evidence.
- The resulting spec is ready for `skill-delivered-methodology-k4` to decompose.

## Notes

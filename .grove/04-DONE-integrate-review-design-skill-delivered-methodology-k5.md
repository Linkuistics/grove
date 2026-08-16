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

All five findings verified against their cited sources before being applied —
none were noise, and none were a contract stated unclearly. What each turned into:

1. **Micro-test added as design validation, not a gate.** The house rule's arms
   are named concretely (control = the ~1.1 kB pre-mandate launcher, which is the
   arm the field failure was actually measured on; variant = the designed core),
   with a stand-in corpus so the test does not depend on the rewrite. The
   `Out of scope` bullet is reconciled rather than contradicted: a *gate* is what
   a build or `cargo test` must satisfy; these are experiments a human reads once.
   Recorded deliberately: **either outcome is useful** — a variant that does not
   beat the control is a stop, and a control that does not fail means the
   prohibition/rationalization apparatus is unwarranted and should be cut.
2. **The fact test is closed on the word "fact"** — a driver fact is a
   launch-varying *value*; static meaning and normative consequence stay in the
   skill. Applied consistently, this took *authoritative* off the handle row **and**
   *do not probe for it* off the version-control row, which the review named only
   the first half of. The cost is stated rather than hidden: two rules move to the
   skill and now depend on the skill being read, and a new requirement scenario
   obliges `content/` to carry them.
3. **`methodology` narrows; `prompt` is a new seam.** Chosen over moving embed
   ownership behind `prompt`, because `provision` consumes the embed and
   `identity()` feeds both pairing checks — putting provisioning's supplier behind
   a prompt-composition seam is an inversion. The architecture table gains a row
   and rewrites one; that is now planning input rather than an open question.
4. **The one-page claim gets a deterministic measure** (loop section,
   heading-to-heading, ≤100 lines — an alarm above the rewrite's ~80-line
   estimate, chosen the way the 4 KiB alarm was), and **no-procedure becomes a
   review obligation** with a scenario naming its evidence. The test-seam bullet
   now says explicitly that budget tests establish nothing semantic.
5. **The derivation is stated in order** — remove the ending unit into the core
   first, *then* recover ten scopes. The corpus really carries eleven distinct
   narrowed scopes (`skill-signal` is narrowed to the eighteen non-`finish`
   kinds), so the unordered claim would have misdirected a mechanical rewrite.

No new concern surfaced that needed externalizing; the spec's own open items
(trigger strength as the unsettled claim, `leaf-prune`'s HITL gap, the
undetectable launch target) are unchanged and remain the brief's horizon notes.

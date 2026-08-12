# ordering-key-placement-k6

## Goal

Settle **one** question the settled design leaves in conflict with its own
increment order, and record the answer in
`docs/specs/mandate-delivered-methodology.md`: where the per-file
mandate-ordering key lives, given that [[Global skill provisioning]] stays live
for the whole of this grove and the one after it, up to that grove's final
stage.

Nothing else in the design is reopened. This is not a re-review of
`mandate-delivery-k2`; four of its five review findings were already integrated
by `mandate-delivery-integrate-k5` and are settled.

## Context

`increments-k4` found this while ordering the increments. The design says two
things that cannot both hold while provisioning is live:

- **Frontmatter is required on every embedded markdown file**, is `---`-delimited
  **KDL**, and carries the file's composition order
  (`docs/specs/mandate-delivered-methodology.md`, *Per-file frontmatter carries
  the file's mandate order*). Its absence is a build error, and that requirement
  is part of the stage-1 gate — the very first increment.
- **`content/SKILL.md`'s YAML frontmatter "exists solely for harness skill
  discovery"**, and *"retiring provisioning frees that slot: once nothing
  provisions the file, no parser depends on it."*

The second sentence is true only **after** provisioning retires, which the
increment order puts in the *successor* grove's final stage. Until then, `grove`
sweeps `content/SKILL.md` into every installed harness's global skill directory,
and Claude Code discovers the grove skill by reading `name:` / `description:`
out of that file's YAML frontmatter. Replace that block with KDL in this grove
and the next installed build silently stops offering the `grove` skill to every
session — while `content/prompts/continue.md` still tells each one to *"use the
grove skill"*. That is a live-loop break, shipped by the increment whose whole
point is that it changes nothing a reader or a session sees.

The conflict does not move by reordering. It lands wherever frontmatter lands,
for as long as provisioning is live, and provisioning cannot retire before the
mandate delivers.

### Candidate answers, as found — not a shortlist to pick from blind

1. **KDL everywhere, `content/SKILL.md` exempted temporarily.** The gate names
   one file it does not hold to the rule, and the successor grove's retirement
   stage removes the exemption. Cheapest, and it puts a named hole in the
   invariant that exists precisely to have no holes.
2. **Carry the ordering key in an HTML comment instead of frontmatter** — the
   same device the unit markers already use, so `content/` gains no second
   metadata language and YAML frontmatter survives untouched wherever a harness
   needs it. Costs the *Per-file frontmatter* decision, which chose KDL to keep
   the repository to one metadata language; a file-scoped comment directive keeps
   it to **zero** new ones.
3. **Drop frontmatter from this grove's gate entirely and add it with the
   composer.** Defensible because the hole frontmatter is credited with closing —
   *"an unmarked file contributes no units"* — is already closed by the rule that
   **body text before the first marker is a build error**: a file with no marker
   is entirely body-text-before-the-first-marker. Frontmatter's load-bearing job
   is the ordering key, and ordering has no consumer until composition exists.
   This defers the conflict rather than resolving it, since composition still
   precedes retirement in the successor grove.
4. **Keep both blocks readable** — a single `---` block that parses as KDL *and*
   as the YAML two-field subset a harness reads. Verify before believing:
   `description:`'s value is a long unquoted string, which KDL does not accept.

Weigh 2 against 1 and 3 explicitly. Whatever is chosen, say in the spec **what
the gate requires of a file**, since that sentence is what the first
implementation increment builds.

## Done when

- `docs/specs/mandate-delivered-methodology.md` states where the per-file
  ordering key lives and what the build gate requires of an embedded markdown
  file, with the provisioning-still-live constraint named as the reason.
- Any consequence for `docs/adr/mandate-delivers-the-methodology.md`, `CONTEXT.md`
  ([[Methodology unit]]) or `docs/ARCHITECTURE.md` is reconciled in place —
  reworked, not appended to.
- `unit-grammar-k7` can be started without re-deciding anything.

## Notes

- Verify the premise before designing around it: confirm that Claude Code (and
  the other harnesses `provision_installed` sweeps) really do read
  `content/SKILL.md`'s YAML frontmatter from the provisioned copy, and that
  nothing else in the repository parses it. `src/provision.rs` is the writer;
  the reader is outside this repository.
- The question is bounded and the answer is a spec edit. If it turns out to be
  one paragraph, say so and retire — a `review-design` leaf is for a
  load-bearing artifact, and one field's placement may not be one.

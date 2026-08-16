<!-- file: order=20 -->
<!-- unit: skill-the-spine-in-full class=procedural -->
## The seven constraints, argued

grove drives long work *without* becoming brittle, constraining machinery.
These seven rules are non-negotiable; everything else is subordinate to them.

1. **Artifacts, not state.** No phase file, no session log, no status file.
   The directory tree under `.grove/` is the only state; the VCS holds the
   history.
2. **Read, don't run.** A session bootstraps by *reading markdown* — no script
   must succeed before work begins. Its one command, `grove-llm resolve
   <handle>`, is a lookup you could do by eye: the handle is in the filename.
   (Keeping this guidance in step with the `grove-llm` it instructs is the
   `grove` binary's own job — it *embeds* this methodology at build time, so the
   guidance you are reading and the verbs on your `PATH` are always the same
   build. The boundary is that build, not a commit: in a grove *of grove*,
   editing `content/` changes nothing any session receives until the binary is
   rebuilt and installed.)
3. **Suggested shape, not enforced schema.** Task files and briefs are freeform
   markdown. The format files are guides; nothing validates them.
4. **Lazy and optional.** Every artifact — brief, ADR, spec, glossary entry — is
   created only when it earns its place, never because a step demands it. Lazy
   means *just-in-time, not few*: a tree that keeps sprouting small, concrete
   leaves is healthy, not a smell.
5. **grove guides, it does not gate.** grove never refuses to proceed. A task
   may be done by hand, reordered, or skipped.
6. **Walk-away-able.** Delete this skill and `.grove/` is still a legible
   folder of notes; every durable output is standard, team-readable markdown.
7. **One page of rules.** If the loop does not fit on a page, it is too
   complex — cut until it does.

<!-- unit: skill-why-the-glossary-holds class=procedural -->
## Why the glossary is the forcing function

The acute failure mode of multi-session work is terminology drift: a later
session, with no memory of an earlier one, reinvents its term under a new name
or reuses the words with a shifted meaning. `CONTEXT.md`, read every session and
appended *inline* whenever a term is resolved, is the forcing function against
that — inline rather than batched, because a term resolved and not written down
is a term the next session will re-resolve differently. Keep it a glossary and
nothing else — terse definitions, aliases-to-avoid, no implementation detail
(`CONTEXT-FORMAT.md`).

<!-- unit: skill-specs class=procedural defers="spec-suggested-shape spec-test-seams" -->
## Specs

A **spec** is the human-facing, team-shareable design of an area of the system,
produced lazily by a `design` task *when the increment is a genuine agreement
point*. The flow there: grill → spec (review & agree) → decompose → execute.
Specs live in `docs/specs/<slug>.md` and, like ADRs, are a **minimum coherent
set describing the current design**: edited, merged, split in place; deleted
when one no longer describes anything (constraint 1 — the VCS holds the past).

Two rules keep the set honest. **Membership:** would a session on an unrelated
future grove need to read this? If not, it is a `BRIEF.md` and it dies with
`.grove/`. **Grain:** an ADR records one decision and its trade-off; a spec
describes how an area works, and *cites* the ADRs in its area rather than
restating them. Shape and the seam-sketching rule: `SPEC-FORMAT.md`.

<!-- unit: skill-what-the-plugin-carries class=procedural -->
## What the `linkuistics` plugin carries, and why it is separate

Three bodies of guidance grove leans on live in a **separately installed**
plugin: ADR philosophy in `linkuistics:decision-records`, what a test seam is
and how to judge one in `linkuistics:codebase-design`, and the
working-copy-as-commit lane in `linkuistics:using-jujutsu`, which the Commit
step cites rather than restates. The plugin is developed in grove's own repo
(`plugins/linkuistics/`), but the `grove` binary provisions only grove's
methodology — never the plugin — so it is installed on its own, through the
Claude Code marketplace or the repo's `plugins/install.sh`.

Self-containment is not a constraint for any of the three: `ADR-FORMAT.md` and
`SPEC-FORMAT.md` keep only grove's placement and recording conventions, and the
Commit step keeps only where its boundary falls. A session raising or reworking
an ADR, sketching a spec's test seams, or driving a jj-enabled tree should
consult the matching skill. The dependency is documentation-level, not
install-enforced; everything else grove needs stays self-contained
(constraint 6).

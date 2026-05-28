# 070-audit-llm-cli-boundaries

**Kind:** planning

## Goal

Audit grove's existing prompts, bundled `SKILL.md` procedures, and the
non-CLI parts of its workflow for **deterministic steps currently
expressed as prose instructions to the LLM that would be better
expressed as CLI verbs the LLM invokes**. Output: a classified
inventory of each step — keep as prose, promote to a CLI verb, or
hybrid (prose narrates; CLI does the mechanical part) — with a brief
rationale and a recommendation for whether the promotion belongs in
its own work leaf, folds into an existing leaf in this subtree, or is
deferred as a seed to a future grove.

The motivating principle: every prose instruction that tells the LLM
to do a deterministic thing (a `mv`, a path computation, a directory
walk, a parent-chain traversal, a triage gesture with a fixed shape)
is a latent failure mode — the LLM can mis-execute it, drift across
sessions, or hallucinate an adjacent variant. Promoting such steps to
verbs converts each failure mode into a CLI bug to be fixed once.

## Why this came after the rename in pick order

The rename leaf (060) is mechanical and well-scoped (move strings,
add `grove meta init`). Its scope does not intersect the
reclassifications this audit will produce. Gating the rename on a
broader planning task would block obviously useful work for a
not-yet-defined audit outcome. The audit may, however, surface CLI
verbs whose implementation overlaps with the rename's code surface
(e.g. anything touching the `grove-meta` materialisation path) — in
which case those promotions land in new sibling work leaves rather
than retroactively reopening the rename.

## Context

- `content/SKILL.md` — the bundled methodology. Read top-to-bottom
  for prose steps that describe a mechanical action the LLM is to
  perform (the loop's *Pick*, *Bootstrap*, *Commit*, *Retire*, and
  *Finish* paragraphs are the obvious candidates; the artifacts
  table and the "inboxes and capture" section are secondary).
- `content/prompts/*.md` — the launcher prompts read by `grove start`,
  `grove continue`, `grove takeover`, `grove retire`, `grove finish`.
  These are the per-verb entry points; each is small, each contains
  some prose-coded mechanical step (e.g. "drain the inbox at
  `<repo>/.grove-meta/inboxes/<name>.md`").
- `.claude/skills/grove/SKILL.md` and friends — the materialised
  in-tree copy. Same content as `content/`; do not audit twice.
- The existing CLI surface (`grove --help`, plus the recent inbox
  verbs from b71c6d5). Map each prose step to whether a verb already
  exists; if so, the issue may be "prompt asks the LLM to do it
  manually instead of calling the verb" rather than "no verb exists".
- The shape of past failures: in particular, places where session-to-
  session drift or LLM mis-interpretation has been observed. The
  user is the source of truth here; the audit session should grill
  on "where has the LLM gotten this wrong before?" rather than
  guessing.
- Glossary terms in play (see `CONTEXT.md`): the audit may add new
  glossary terms for promoted verbs only if they introduce a new
  domain concept (most will not; `grove pick` doesn't need a glossary
  entry just to exist).

## Candidate steps to inspect

Not exhaustive — the auditing session walks `SKILL.md` and the prompts
top-to-bottom. This list is a starting set:

- **Pick** (depth-first walk under `.grove/`, skipping `done/`).
  Currently prose. Candidate: `grove pick` prints the path of the
  next leaf. Possible objection: trivial enough that `find` /
  `ls` suffices.
- **Drain inbox**. Currently prose tells the LLM to read the inbox
  file, triage entries, and after triage call `grove inbox drain`.
  Could be `grove inbox triage` that streams entries one at a time
  in a structured form (id, body), accepts `incorporate | defer |
  reject | seed <other-grove>` per entry, and commits the cleared
  state at the end.
- **Retire a leaf**. `grove retire` exists. Is the prompt prose
  consistent with the verb (i.e. does it tell the LLM to call the
  verb, or to perform the mechanical steps by hand)?
- **Parent-chain cascade after retire**. Currently prose tells the
  LLM to walk parents and ask the user before retiring each empty
  node. Could parts of this be a verb (`grove retire --cascade` that
  surfaces empty ancestors and asks per node) rather than prose?
- **Finish**. `grove finish` exists. Audit whether the "promote
  anything from briefs that should outlive the grove" step is a
  judgement call (keep as prose) or has mechanical sub-steps the
  CLI could orchestrate.
- **Renumber a leaf during planning**. Currently `git mv` driven by
  the LLM. Is this common enough to deserve a verb (`grove leaf
  renumber <from> <to>` that also updates the leaf's `# NNN-name`
  header and warns on cross-references)? Probably premature; flag
  as a future candidate if renumbers continue to be common.
- **Capture seed name conventions**. The LLM has to invent the seed
  name (`tui-multi-repo`, etc.). Could a `grove inbox list` /
  `grove inbox names` verb at least show existing inbox names so
  the LLM doesn't accidentally create a near-duplicate? Probably a
  small win.
- **Bootstrap reading order**. Currently prose tells the LLM what
  to read in what order. This is judgement-shaped (which ADRs
  matter depends on the brief chain), but the *enumeration* (which
  briefs exist in the chain) is mechanical and could be surfaced
  by a verb (`grove brief-chain` printing the chain root→leaf).

## Done when

- Each prose step inspected has a classification: **keep**,
  **promote**, or **hybrid**, with a one-sentence rationale.
- Each promotion has a recommended placement: a new work leaf in
  this subtree (numbered after 070), a fold-in to an existing leaf
  (specify which and how), or a seed for a future grove (the
  recommendation captures the suggested seed name and a one-paragraph
  description for the seed file).
- If the audit produces enough promotions to warrant a dedicated
  implementation phase, this planning leaf is replaced by
  `070-audit-llm-cli-boundaries/` with `BRIEF.md` and child work
  leaves. Otherwise it produces one or two new sibling leaves and a
  short summary of which steps stay as prose and why.
- If any step's "this should be a verb" conclusion is non-obvious
  (real trade-off, hard to reverse), it gets a short ADR. Otherwise
  the classification and rationale live in this leaf's notes during
  grilling, then are promoted into the new work leaves' Context
  sections.

## Notes

- **Resist the "make everything a verb" gravity.** Prose is the right
  shape for judgement-heavy steps: grilling, reading-and-
  synthesising, deciding what an ADR should say. The audit's bias
  should be *promote when deterministic and repeatedly LLM-driven*,
  not *promote on principle*.
- **Resist scope expansion into TUI territory.** Some promoted verbs
  may overlap with what the TUI (080) wants to display; that is
  fine and intentional. But the TUI's design is the 080 leaf's
  concern; this audit only inventories what should be a verb, not
  how to surface it.
- **Walk-away-ability still rules.** A promoted verb should leave
  the same filesystem artifacts behind. If a verb's output is
  visible only through that verb, it has failed constraint 6.
- **The `content/prompts/*.md` files are part of the audit surface,
  not just `SKILL.md`.** Each prompt is small, but each contains
  prose-coded steps that fire every session. They are
  disproportionately important to get right.
- **Reference the rename leaf's verb namespace.** If new verbs are
  recommended under `grove meta`, the rename leaf (060) is the
  first place that establishes that namespace; coordinate so the
  cluster grows coherently rather than scattering verbs across
  inconsistent parents.

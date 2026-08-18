## Raising records, and keeping each set minimal

Whichever kind is running: raise ADRs *sparingly* (`ADR-FORMAT.md` for
placement; the `linkuistics:decision-records` skill for the philosophy, format,
and when-to-write test), and write a spec only at a genuine agreement point
(`SPEC-FORMAT.md`). Treat the ADR set as a **minimum coherent set describing the
current design**: when a session *changes* a decision an ADR already records,
**rework the set in place** — merge / split / delete — and reconcile the briefs
that cite it; never append a superseding ADR (the VCS holds the history). The
same rule governs `docs/specs/`, one grain coarser.

## Review ownership inside a picked leaf

**The allowance is a picked leaf's, and only a picked leaf's.** It applies after
the driver launched this session with a selected-leaf mandate in `${prompt}` and
this session adopted that mandate by running Bootstrap. A `.grove/` directory in
the checkout and inherited Grove control variables do not count; outside that
predicate, doubt keeps its standalone bounded cycles unchanged.

**A picked plain producer may materialise at most one in-session reviewer across
the whole picked leaf.** The allowance is leaf-wide rather than per decision or
per artifact, and each independently materialised context spends it — a
diverse-lens pass with N subagents spends N, while one reviewer asked to inspect
several named axes spends one. An explicitly chosen cross-model reviewer spends
the same one; do not offer a second pass after it. A **second** need — normally
re-review after a substantive non-mechanical fix — is the mechanical signal that
review has become tree-sized work: cut a `review-<producer>` leaf, writing the
specific doubt into its body, and finish only to a coherent reviewable boundary.
Trivia, noise, a visible accepted trade-off, and a fix conclusively covered by an
executable test seam do not force it.

The other kinds are deliberate exceptions, and the allowance differs by kind:

- a producer that already has a `review-*` leaf beside it spends **none** — its
  review is scheduled;
- a `review-*` session spends **none** — it *is* the adversarial read, and
  produces findings rather than fixes;
- an `integrate-review-*` session may spend **one narrow** reviewer, and
  externalises substantial redesign as a new producer review chain beside the
  leaf it is integrating;
- `research-a`, `research-b` and `combine-research` spend **none** — the pair
  supplies independent corpora and the combiner supplies the adversarial move.

**Spending the allowance is a four-step pass**, not an invitation to a second
opinion. Use it for a narrow, unexpected claim whose correctness the compiler
cannot establish:

1. State the claim and why it matters.
2. Extract only the artifact and its contract, stripping the conclusion.
3. Give one fresh context the artifact and contract with an adversarial *find
   what is wrong* prompt.
4. Re-read the artifact and classify **every** finding four ways: a contract you
   stated unclearly, valid and actionable, a visible trade-off, or noise.

**Once review is escalated to the tree, grove owns the route.** It launches the
`review-*` kind's own configured command, so do not add a competing in-session
reviewer beside a scheduled review.

## Verifying a claim about the repo itself

A repo-wide claim about your own codebase — *"every X is now Y"*, *"that pattern
is gone"* — is not evidenced by a clean grep. **Check that the output resembles
what you asked for, not merely that the command exited 0**: the flag you meant is
often a different flag (in ripgrep `-E` is `--encoding` and `-r` is `--replace`,
so `rg -rn '<pat>' .` succeeds and prints every match with the pattern
substituted away), the search space may exclude what you are searching for (`.grove/`
is a dotdir ripgrep skips without `--hidden`), rendered output is the wrong
surface to scrape, and a well-formed pattern that matches nothing *anywhere*
reads exactly like a clean repo.

**Two controls turn a clean result into evidence.** A broken instrument reads
clean *everywhere*, so clean-here alone proves nothing — but clean-here plus
dirty-there cannot be produced by a broken instrument.

- A **positive control**: the same command with the same flags must find
  something you know is present in the same trees.
- A **cross-tree control**: the same pattern must still find the class where it
  legitimately lives — the docs that discuss it, the fixtures that illustrate it.

**Enumerate, then classify. Do not sweep a pattern list.** A list of patterns is
complete only as far as the list, and re-running it with a longer list moves the
leak rather than closing it. Extract *every* candidate token from the whole
surface, then classify each one; that is complete by construction, and it is what
finds the sibling class nobody thought to list.

**The sweep's scope is the claim's scope**, and three narrowings happen without
anyone deciding to narrow anything — each has passed for *done* and then leaked:

- **Grep the claim, not a file list.** A file list is written *before* the work
  and goes stale; the claim cannot, because the claim *is* what went stale.
- **A path scope goes stale the same way**, and can never reach the files that
  are in **no** tree at all — a root manifest, a dotfile.
- **A finding against a *section* does not reach the summary layer.** A roll-up
  summarises sections and is untouched by correcting one, so when a finding lands
  against a heading, sweep the abstract, the table and the overview too.

**Never document a claim with a count of itself.** *"The string occurs three
times, only one is a heading"* is self-invalidating the moment the sentence
stating it adds another occurrence. State the structural fact instead — *only the
heading is ever a whole line* — and say explicitly not to replace it with a count.

**A clause rescued by its neighbour is not correct**; it is load-bearing on the
sentence you are about to delete. Before removing a false clause, check whether
the true one beside it only reads as true in its company.

## Recording decisions as they settle

Append each settled decision to the task file's `## Decisions (running log)`
section **at the moment it settles** (`TASK-FORMAT.md` for the section's shape).
That survives interruption — a long session that drops leaves the log as the
source of truth for what has and has not been settled — and it produces an audit
trail without the phase file constraint 1 forbids.

**Never reconstruct a session's decisions at the end of it.** By then each one is
already recorded where it landed, so a session-summary file, or a commit message
that re-tells every decision, duplicates those records, rots against them, and is
exactly the status artifact constraint 1 rejects. The running log is not the ADR:
the log is for the conversation, the ADR for the durable record.

## Handing a question back to a human

When you hand a question back, **name the trade-off you want input on**. *"Let me
know if you have any questions"* is not a prompt — a vague invitation buys a vague
response, and the human has to invent your agenda before they can answer it.
Three parts, and the third is what makes the escalation answerable:

1. **Name the specific trade-off** — the decision, and the two or three ways it
   could go.
2. **Propose a recommended answer.** Deference that withholds a recommendation
   costs the human the one view they cannot get anywhere else.
3. **Give the evidence** behind that recommendation, so disagreement is a debate
   about the evidence rather than about whose preference wins.

That turns an escalation into a decision instead of an open-ended handback.

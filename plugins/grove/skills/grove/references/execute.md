<!-- tc: pending because the conversion has not reached this part of the document -->

## Reusable development tasks

Use the repository and machine owner's documented environment to invoke
reusable tasks. On Antony's Mac, use the `machine-environment` skill for tool
selection and machine configuration ownership before setup or development
commands. Its personal paths apply when the documented local setup is present;
elsewhere, follow the repository and machine owner's environment contract. The
local task and environment rules below bind when the skill is absent.

Check for a project Taskfile in **every new or existing project**. If it has
none, always suggest adding a repository-root `Taskfile.yml`. If one exists,
use and extend it. Carry forward a settled project choice rather than repeating
the suggestion each session.

**The Taskfile is the entry point for deterministic reusable development work**:
setup, formatting, linting, tests, builds, checks, releases, and other repeatable
project workflows. Capture as many concrete reusable tasks as the work reveals;
grow the file from actual needs, without inventing speculative tasks. Wrap
existing scripts and tools where they already own the mechanics. Use the tasks
on subsequent runs instead of having an LLM reconstruct command sequences each
time, and verify new or changed tasks before relying on them. Keep steps needing
human or LLM judgment explicit. A deterministic recipe may invoke a configured
LLM step, supply its inputs, and validate its outputs; it must not pretend the
judgment itself is deterministic.

This executable task file belongs to the project and survives the grove; it is
distinct from the markdown task notes under `.grove/`.

<!-- tc: on -->

## Visible delegated work

Keep background, delegated and spawned tasks visible. Announce the purpose of
each task. When the current harness has a nested-task UI, show the progress of
each task there. Report the outcome of each task.

For external one-shot work, use a `grove run` kind that the owner's policy
routes. Its live transcript and its persistent log are the baseline. In a
supported active multiplexer, `--ui auto` can add a view.

Keep control of the terminal and its panes with the supervisor. A child must
not inherit the terminal, the multiplexer connection or the completion
authority of its parent. For a `grove run` child, Grove enforces this rule.
Grove runs the child as a detached job and removes the multiplexer variables
and the loop controls from its environment. If no embedded UI or pane is
available, keep inline progress and a log. Do not detach silently.

<!-- tc: pending because the conversion has not reached this part of the document -->

## Review ownership inside a picked leaf

This policy applies after the driver launched this session with a selected-leaf
mandate in its prompt and this session adopted that mandate by running Bootstrap.
A checkout's `.grove/` directory and inherited control variables are not that
predicate; outside that predicate, doubt keeps its standalone bounded cycles unchanged.

**Classify the purpose of a review before choosing its route.** Checking a named
requirement, comparing an implementation with its contract, checking sources,
running tests and verifying a fix are ordinary implementation work. Keep those
checks in-session. Prefer bounded subagents for separable implementation work,
routine reviews and follow-up checks that can use the parent session's model
and framing. **Routine checks have no per-leaf reviewer quota.** A second check
or a fresh helper context does not itself earn a Grove task. Keep each check
bounded by a concrete question and stop when it is answered.

**Work that requires a different model belongs in a separate Grove task**, with
the kind appropriate to its purpose, so the owner's policy selects the model.
Do not switch a helper's model to bypass that routing. A model preference alone
does not justify extra work; use a same-model helper when it can answer the
question.

**An adversarial review requires a separate `review-<producer>` leaf.** Its
purpose is an independent attempt to break a load-bearing artifact, challenge
its assumptions or discover defects the producer has not named. Freshness from
the producer's framing is part of the deliverable. Finish to a coherent,
reviewable boundary and cut that sibling through `references/decompose.md`;
write the specific doubt and the artifact's contract into its body. Do not
substitute an in-session adversarial reviewer, or relabel that work as routine
verification to bypass the route. The reason to create the task is the need for
independent challenge, never the number of earlier checks.

Give the adversarial reviewer the artifact and its contract, **stripping the conclusion**.
It reports findings rather than fixes. The integration session re-reads the
artifact and classifies every finding as an unclear contract, valid and
actionable, a visible trade-off or noise, using `references/integrate-review.md`'s
acceptance and material-risk threshold. It can verify fixes in-session;
substantial redesign needs a producer step and a new adversarial review only
when independent challenge is warranted again.

A scheduled `review-*` leaf owns that adversarial read; do not duplicate it with
another reviewer in the producer, review or integration session. Ordinary
verification remains available in each kind. Research pairs retain their
independent corpora and the combiner supplies the adversarial move; routine
helpers do not replace either research session or the combiner. The pair is
`research-a` and `research-b`; the combiner is `combine-research`.

**Once review is escalated to the tree, grove owns the route.** It launches the
command the owner's policy selects for the `review-*` kind.

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

**A control that has never been seen to fail is not a control.** A positive
control that would find something whatever the tree held, and a cross-tree
control whose pattern matches everywhere, read exactly the same whether or not
the instrument works — which is the one property they were introduced to rule
out. Watch each control come back **dirty** against a subject you know is wrong
before you credit the clean read: a deliberate mutation of the pattern, of the
flags, or of a scratch file is enough, and it costs one command. Until you have
seen it fail, a control and the broken instrument it was guarding against are
the same observation.

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

## The provenance of a measurement

The section above is about the *pattern you searched for*. This one is about the
*run*, and it fails more quietly: nothing errors, every command exits 0, and a
number comes back describing something other than what you meant to measure. **A
suite, a script, a sweep is an instrument, and an instrument you adjust
mid-reading has not read anything.**

**Finish every edit, then measure.** A script rewritten under the shell running
it, or an input rewritten under the loop reading it, is undefined rather than
untidy — and *the edit looked harmless* is not the test, because you cannot tell
a harmless edit from a load-bearing one except by the reading you no longer have.
Restarting a run two minutes in is cheaper than the run you have to throw away.

**Digest every subject before and after, and say which files those were.** A
run's subjects are everything it *reads*, not only what it executes: the manifest
it takes its work from, the notes it takes its exclusions from, the fixtures it
compares against. Freezing is the point of the digest — a run whose subject moved
under it is not a measurement of the thing it reports on, however small the move.

**One measurement, one writer.** Two runs appending to one output produce a
reading that is self-contradictory rather than merely incomplete, and the
contradiction is visible only if you read the whole of it. And **never infer that
a background job has finished from the return of the thing that launched it**: a
launcher returns when the launch succeeded, which says nothing about the work.

**Confirm a re-run item by item, not by matching totals**, since two runs can
agree on a total while disagreeing about which item did what. Compare the
per-item record, and where there is no per-item record the re-run confirms
nothing.

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

### The exchange in a requirements or design session

A `requirements` or `design` session hands the human questions in an
exchange, and these rules bind that exchange. A view drawn for the exchange
meets `FIGURE-FORMAT.md`. The exchange has no document contract, so its prose
takes the house style in `references/house-style.md` directly. A point of the
house style applies only where its form occurs. A comparison of options
follows the teaching section's rule on comparison, which this section does not
restate.

<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **Before a trade-off that depends on a term or domain fact, the question
  defines it or asks about it, unless the exchange has already explained it or
  the human has shown they know what the trade-off needs of it. The human has
  shown that by stating it, reasoning with it correctly, or confirming it as
  the exchange stated it, in the exchange or in their own writing the session
  has read. Using, quoting or asking about a term is not that evidence. A
  doubt they voice about a meaning outweighs any earlier explanation or
  evidence.**
- **The question states the problem and what it costs before any option.**
<!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **At an agreement point (a spec or a decision that meets the ADR bar), the
  session asks the human to restate the decision in their own words and
  records the restatement beside their answer.**
- **Prose handed to the human in the exchange follows the house style's
  points wherever their form occurs.**
<!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
- **Prose handed to the human in the exchange follows the teaching section,
  with the human as its reader. What the exchange has already explained, and
  what the human has shown they know, as `vocabulary-before-trade-off` defines
  both, is what they already read. A name they have used for an entity is
  kept rather than replaced by a synonym, whether or not they have shown they
  know what it names; where the project uses that name for a different
  entity, the question says which is meant. A question reasons with plain
  names: an identifier, code or file name, the human's own included, follows
  the plain statement of what it names, only as a pointer where the human
  needs to find the thing. Where a name or notation is itself what the human
  decides, the candidates follow the plain statement of what they express.
  The rules reach every part the human decides from: the message, a
  question's header, each option's label and description, and the text of any
  view drawn for it.**

**Required unanswered input keeps the leaf live and the interactive harness
session open.** Follow this handoff:

1. Ask the question with the recommendation and evidence above. An asynchronous
   tool's acknowledgement that it accepted the question is not the human's
   answer.
2. Complete useful work that is independent of the answer. Keep dependent work
   pending until the human supplies the required input.
3. When only the answer remains, state the pending decision and **finish the
   agent turn**, yielding the interactive prompt to the human in the same
   session. Waiting needs no invented deadline, polling, heartbeat or repeated
   sleep. While only waiting, use neither `end-session grove-continue` nor
   `end-session grove-force-stop`; both end the session that must receive the
   answer.
4. When the answer arrives, continue this session, record the settled decision
   and complete the dependent work. Then follow the normal retirement, commit
   and session-ending procedure.

If the human explicitly requests termination, honour that request through the
launch contract's stop procedure.

<!-- tc: on -->

## Teaching the reader

These rules bind `draft` and `copy-edit` while they work on a document. They
teach the reader that the document's contract declares. They also bind a
`requirements` or `design` session in its exchange with the human, and there
the human is the reader. Each rule applies within the charter of the stage
that applies it. A rule marked with one stage, such as *`draft` alone*, binds
that editorial stage and no other editorial stage. The mark does not remove
the rule from an exchange.

`draft` writes what the text teaches. `copy-edit` repairs how its sentences
carry that teaching, and it sends missing teaching back to `draft`.

- **Teach the model before the notation.** Explain the subject model and the
  reader's task first, then describe the notation, identifiers or codes that
  express them by reference to that model. <!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **Establish the actors before a label.** Before an abstract label,
  establish in the local explanation who acts, on what, and with what
  consequence, whose viewpoint a relative term takes, and what each case
  actually does. After any passage, whether or not it introduces a label, the
  intended reader can say who acts, what they do and what follows; a synonym
  for an abstract noun leaves that gap open. <!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
- **Explain what this reader cannot infer.** Explain the referents,
  relationships, conditions and consequences this reader cannot infer, in an
  explained example as in running text. Each explanatory sentence adds meaning
  this reader needs, and none narrates what they already read, such as a
  familiar token. Explanation length follows that need: a length limit yields
  to it, and a document's draft records the override. Removing narration never
  compresses an unfamiliar concept back into jargon.
- **Compare, then state the principle, then name the limit.** Teach a new
  distinction, or put a choice between options, by setting the cases or
  options side by side, stating the principle that separates them, and naming
  where the comparison breaks. An analogy states its basis and where it fails;
  a distinction between kinds is shown through their uses. <!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
- **One name for one entity.** Text read together (introductions, code
  labels, captions and tables) uses one name for one entity, and a use
  colliding with a declared term risk is flagged. <!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **Keep the domain, the roles and the choices distinct** (*`draft` alone*).
  Establish the intended domain before comparing alternatives and judge a
  source's assumptions against it; keep subject, producer and consumer roles
  apart; keep what the problem requires apart from what the design chose,
  with its costs and alternatives in view; a list of examples implies no
  property the list does not establish. <!-- tc: depart long-sentence,semicolon because the conformance inventory pins this rule statement verbatim -->
- **Normative text traces to its rule** (*`draft` alone*). Every normative
  statement traces to its rule, a rejected form names the rule that rejects
  it, an ambiguity in the subject is recorded rather than resolved in silence,
  and added teaching preserves normative force and exact restrictions. <!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->
- **Invent no observation** (*`draft` alone*). Expected observations are
  checkable by inspection, and no output is shown that no tool produced.
- **State the subject positively, and keep every restriction recoverable**
  (*`copy-edit` alone*). Headings and openings name the subject, its operation
  and its conditions positively, and every exact prohibition and rejection
  condition stays recoverable from its rule. <!-- tc: depart long-sentence because the conformance inventory pins this rule statement verbatim -->

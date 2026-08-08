# Driving a grove session well

The grove `SKILL.md` and `grilling.md` files state *what* the loop is. This
file is about *how* to drive it well — the moves a human collaborator makes
that turn the loop into productive design work. It is a field guide, not a
specification; treat it as a starting set of habits, not a checklist. Most of
it concerns the *design* kinds — `research`, `requirements`, `design`,
`planning` — but the later sections are habits for `impl` sessions too:
grounding framework decisions in the source, doubting a decision before it
stands, and externalizing surfaced work into new leaves rather than absorbing
it.

The examples are stated as reusable shapes rather than as the history of the
workstream that first produced them. A finished grove deletes its task tree;
the lesson belongs here only when it still helps a future session.

## In this guide

- **Starting and research:** [when not to start](#when-not-to-start-a-grove),
  [commissioning prior art](#when-to-commission-prior-art-research),
  [writing a research leaf](#how-to-write-a-research-leaf-brief), and
  [running vendor pairs](#running-the-vendor-pair).
- **Requirements and durable evidence:**
  [grilling](#when-to-invoke-a-design-discussion-grilling),
  [retiring research](#when-to-retire-research-into-adrs-versus-leave-it-in-docsresearch),
  [verifying framework decisions](#verifying-framework-decisions-against-the-source),
  and [verifying repo claims](#verifying-a-claim-about-the-repo-itself).
- **Doubt and decomposition:**
  [doubting in one leaf](#doubting-inside-a-picked-grove-leaf),
  [escalating to a review chain](#the-review-chain--when-doubt-earns-its-own-leaves),
  [externalizing work](#externalizing-surfaced-work),
  and [triaging stalled work](#prune-reorder-or-file-an-issue--the-triage-a-status-word-would-hide).
- **Quick reference:** [anti-patterns](#anti-patterns) and
  [the shortest version](#the-shortest-version).

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Chart the map" step 2's no-fog
     early exit) — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## When not to start a grove

Before reaching for `root-init`, check that the journey actually needs a map.
If a first bootstrap session surfaces no real fog — the path to done is
already clear and the whole thing fits in one sitting — do the work directly
instead: a grove would only wrap it in a tree, a brief chain, and a relaunch
cycle it doesn't need. Reach for `root-init` once genuine session-to-session
fog shows up — work whose shape you can't see past the next session or two.
`root-init` will always mint a first leaf on request; that mechanical fact is
not itself the signal to run it.

## When to commission prior-art research

A `requirements` leaf is the right unit for a grilling session when the
design tree fits in one session. When the leaf's design depends on lessons
that prior tools have learned the hard way — and those lessons are not
obvious from the current codebase — *insert a `research` leaf ahead of
it*. The research leaf's job is to surface the failure modes the
grilling would otherwise have to learn from scratch.

**Signs you want a research leaf:**

- The leaf sits in an architectural neighbourhood with well-known
  prior art (issue trackers, sync protocols, schema registries, etc.).
- Earlier leaves in the subtree have already touched architectural
  decisions the new leaf depends on — i.e. several downstream
  `requirements` / `design` leaves share a common evidence base.
- The grilling surfaces a question like "has anyone tried
  this before, and what happened to them?" — that question itself is the
  signal.

A useful research leaf sits *after* the design questions are visible but
*before* the downstream leaves that need the answers. Its synthesis should
answer those leaves' questions directly, so later grilling can spend its time
on the remaining decisions instead of rediscovering prior-art failure modes.

## How to write a research leaf brief

The single most leveraged move is to **name the downstream questions the
research must answer**, leaf by leaf. The researcher doesn't have to
guess what's load-bearing.

For example, a brief can name its audience by subject rather than position:

> The audience is the four open planning leaves: sync semantics and
> inbox shape, the `grove meta` rename, the LLM/CLI boundary audit,
> and the TUI. Each system section ends with a *takeaway for grove*
> pointing at the leaf(es) its lesson informs. The final
> **Synthesis** section answers the leaf-specific questions in one
> place.

Follow that paragraph with concrete questions per downstream leaf, and
structure the output around those questions.

**Bias the search.** "What's already been tried" produces broad,
shallow surveys. "What went wrong after years of real multi-user,
multi-machine use" produces post-mortems. The survey brief explicitly
demanded post-mortem framing per system, and explicitly biased toward
*non-obvious paradigms* (the previous prior-art survey had already
established that Linear/GitHub-Issues integration was the obvious
fallback to beat, not the starting candidate). Both moves cut the
researcher's degrees of freedom in productive directions.

**Demand a walk-away check per system.** For each prior tool, the
researcher must answer: with the tool uninstalled, what is still
legible? This is the cheapest invariant to require and the most
revealing — it separates the architectures that can be borrowed from
the ones that cannot.

**Demand a citation per failure-mode claim.** "git-bug had this
problem" without an issue link, blog post, or thread quote is mood,
not evidence. The survey brief required primary sources; the resulting
postmortem doc cites primary issues by URL and quotes from them
directly. When you later sit down to write an ADR, those citations
*are* the ADR's rationale section.

**Acknowledge missing sources.** When the researcher searches and
finds silence, that's a finding too. The survey doc flags multiple "no
primary source found" notes (e.g. Google's internal use or non-use of
git-appraise, CRDT-merge surprises in Radicle COBs) — the absence is
itself a confidence signal, and recording it stops future readers from
re-doing the same fruitless search.

## Running the vendor pair

One survey is one vendor's corpus and one vendor's blind spots. When the
question is load-bearing enough to justify the cost, run the **vendor pair**:
two `research` leaves on the same question, differing *only* by which harness
runs them, then a `combine-research` leaf. This is why a leaf may name its own
harness — the two leaves are the same kind, so no `GROVE_RESEARCH_HARNESS`
policy can send one elsewhere:

```
grove-llm leaf-add-pair . sync-protocols --harness-a claude --harness-b codex
```

That is the whole pair — a `sync-protocols-pair/` node holding
`sync-protocols-a`, `sync-protocols-b` and `sync-protocols-combine`, off one
stem, with both producers' `**Harness:**` lines written.

**Name both vendors, even the one you would have got anyway.** The verb requires
it and refuses two names that are the same. Declaring only the second would leave
the first resolving through kind → family → stamp at *launch* time, which makes
"two corpora" a forecast about routing policy rather than a fact in the tree —
and one that quietly stops being true if the policy changes between cutting the
leaves and running them. A pair that looks like one and is not is worse than no
pair at all. The `combine` step is left undeclared; add a `**Harness:**` line by
hand if you want it run somewhere particular.

**The stem gets suffixes** — `-a`, `-b`, `-combine` — so all three sort together
and the pair is visible in `find .grove` without opening a file
(`TASK-FORMAT.md`). The two producers are labelled `a` and `b` rather than one
bare and one `-second`, because they are peers; a bare stem beside a `-second`
implies a producer/step relation the pair does not have.

**Give both researchers the same brief.** The pair buys breadth, and breadth
comes from the corpora differing, not the questions. Two briefs means two
surveys you cannot union.

**Do not run the researchers adversarially.** Framing the second as "find what
the first got wrong" discards the breadth you paid for and biases it toward the
first survey's frame. Both run breadth-seeking; the adversarial move belongs to
the combine step.

**The combine step's job is the one check neither survey can perform on
itself.** Union the coverage, flag every disagreement — and treat **agreement
without independent primary sourcing as a red flag, not a confirmation**. Two
vendors trained on overlapping corpora can agree on something false, and a
purely confirmatory combine raises confidence exactly where it should lower it:
a correlated error laundered as corroboration. So for each agreed claim, ask
whether the two surveys reached it through *different* primary sources. If they
cite the same blog post, or neither cites anything, that agreement is worth less
than a disagreement.

## When to invoke a design discussion (grilling)

The trigger is: a `requirements` leaf's brief lists three or more questions
whose answers interdepend. Grilling is the procedure that walks the
dependency tree without the LLM making decisions on the human's
behalf.

The grilling skill (`grilling.md`) says it briefly: interview one
question at a time, propose a recommended answer for each, walk down
the design tree until shared understanding is reached. The moves
below make that interview productive rather than ceremonial.

### Ask the LLM "WDYT" before committing

When a question feels close to settled, the easy default is to nod
and move on. **Don't.** Ask the LLM what it thinks — explicitly,
including when you already have a strong view yourself.

Two things happen when you ask:

1. The LLM produces a recommendation it would otherwise suppress out
   of deference. The recommendation is informed by the bootstrap
   context (glossary, briefs, ADRs, research) which is exactly the
   evidence base you want surfaced before you commit.
2. If the LLM's recommendation diverges from yours, that divergence
   is the cheapest signal you have that the question still has
   something to teach. Don't dismiss the divergence — interrogate
   it.

In the sync-semantics grilling, the LLM's recommendation for
inbox-shape was "directory of files" (heavily pre-judged by the
research). The recommendation for entry-naming was a UUID
short-suffix. The user's "WDYT, but the slug should be descriptive"
steer shifted the sub-decision toward content-hash suffixes for
idempotency — a materially better outcome than either party's initial
proposal. That particular step exists in the conversation record
because the user asked for the LLM's view before committing.

### Ask for pushback when the LLM agrees too easily

Models default to agreement under social pressure. When the LLM
agrees with your proposal without surfacing trade-offs, push back
yourself: "What would push you toward the other option?" or "What
breaks if we do it this way?" or simply "Pushback please."

The grilling format is built around recommended-answers-with-evidence
specifically to make pushback structural rather than personal. When
the recommendation cites primary evidence, the pushback is a debate
about that evidence, not about whose preference wins.

### Don't merge questions

The grilling skill's "ask the questions one at a time" rule is
load-bearing. Two questions in one prompt — even closely related ones
— produce answers that conflate. Resist the urge to batch.

Where two questions truly interdepend, sequence them: ask the
*foundational* one first, propose the recommended answer, wait, then
ask the *derived* one with the foundational answer already in hand.
The sync-semantics grilling sequenced Q1 (shape) before Q5 (entry
naming) specifically because the entry-naming decision is only
meaningful once shape is settled.

### Record decisions inline, while they're fresh

The running-log pattern: each settled question gets a paragraph
appended to a `## Decisions (running log)` section in the task file
*at the moment it settles*, not at the end of the session. This
serves two purposes:

- Survives interruption. A grilling session can run an hour or more;
  if the conversation drops, the log is the source of truth for what
  has and hasn't been settled.
- Produces audit trail without a separate phase file. The grove
  spine's first constraint (`SKILL.md` constraint 1) forbids phase
  files; the inline log gives the same legibility for free.

The decisions log is *not* the ADR. ADRs come at the end of grilling
(or sometimes during, if a decision is genuinely durable enough to
deserve one). The log is for the conversation; the ADR is for the
durable record.

## When to retire research into ADRs versus leave it in `docs/research/`

Research outlives the grove that commissioned it (constraint 6).
ADRs are the place where research findings become *binding* on
future work.

The rule of thumb: a research finding that *changed a decision* gets
cited in the relevant ADR's rationale section. A research finding
that *confirmed an existing decision* gets a "validated here against
…" note in the relevant ADR, or stays in the research doc with a
forward pointer. Either way you are editing the ADR **in place** — the
set is current-state, so a finding that overturns a recorded decision
rewrites (or merges / deletes) that ADR rather than spawning a
superseding one; see *Reworking ADRs and briefs* below.

The sync-semantics grilling's "Findings adopted" pattern is the bridge
in both directions: the research doc gets a section pointing forward at
the ADRs its findings landed in, and each ADR has a rationale section
citing the survey by primary source. A future reader of either
artifact can trace the evidence chain without re-doing the research.

## Reworking ADRs and briefs as understanding shifts

An ADR set is a **minimum coherent set describing the current design**, not a
changelog (`linkuistics:decision-records`). So when a later leaf's work *changes*
a decision an earlier ADR recorded, the move is **not** to add a new ADR that
supersedes the old one — that grows exactly the chronology grove rejects (it is
constraint 1 again: the artifacts hold the present, the VCS holds the past, now
applied to `docs/adr/`). Instead:

- **Edit in place.** Rewrite the affected ADR to state what *now* binds and why.
  Merge two ADRs whose decisions have converged; split one that turned out to
  cover two independent calls; delete one whose decision no longer holds (the
  VCS keeps it). No `superseded by`, no status line.
- **Keep the set minimal.** After the edit, the set should be the fewest ADRs
  that coherently explain the current design. A rework that leaves both the old
  ADR *and* a new one live has failed the test.
- **Reconcile every citation.** An ADR you merged or deleted leaves dangling
  references behind. Grep its slug across the briefs, the other ADRs, and
  `docs/`; fix or drop each pointer. A dangling citation is a defect, not
  acceptable collateral — and when the rework is big enough that you might miss a
  caller, run the doubt pass (below) over the reconciled set.

The natural checkpoints are the two the loop already has: a **grilling session**
that changes a recorded decision, and **retiring** a leaf or node (`SKILL.md`'s
Plan and Retire steps), where you promote brief material upward and reconcile the
ADR set in the same pass.

<!-- The two sections below are adapted (paraphrased into grove's voice, not
     bundled verbatim) from addyosmani/agent-skills@13e43f23 —
     skills/source-driven-development and skills/doubt-driven-development.
     MIT licensed; see LICENSES/addyosmani-agent-skills.LICENSE. -->

## Verifying framework decisions against the source

The research-leaf discipline — a citation per claim, primary sources, and a
note for what you *couldn't* verify — is not only for research. It applies to
**`impl` tasks** too, whenever you write framework- or library-specific code
whose correctness depends on the version. Training data goes stale; an API you
"remember" may have been deprecated two releases ago.

For that kind of code — and only that kind, not version-invariant logic,
renames, or plumbing:

- **Read the manifest first.** `Cargo.toml`, `package.json`, `pyproject.toml`,
  `go.mod` — whatever pins the version. The version decides which pattern is
  correct, so guessing it defeats the exercise.
- **Fetch the official source, not your memory.** Context7
  (`resolve-library-id` → `query-docs`) or your harness's web-fetch tool
  against the project's own docs or changelog — docs.rs for a crate, the
  framework's reference for a framework. Official docs and changelogs outrank
  Stack Overflow, blog posts, and training data. This is reading, not running:
  constraint 2 is satisfied.
- **Cite at the decision site.** A one-line code comment with the source URL
  next to the non-obvious call, so the next reader can check it without you. A
  citation that lives only in the chat evaporates; one in the code is
  walk-away-able.
- **Flag what you couldn't verify.** "No official source found; based on
  training data, verify before relying on it" beats false confidence — and, as
  with the research-leaf "no primary source found" note, the absence is itself
  a finding.

When the decision is also hard to reverse, this pairs with the doubt pass
below: cite the source *and* have a fresh context try to break it.

## Verifying a claim about the repo itself

The counterpart to the section above. Sessions assert things about their own
codebase constantly — *"every X is now Y"*, *"that pattern is gone"*, *"this
appears in five places"* — and a repo-wide grep is the usual evidence. It is a
**worse instrument than it looks**, because each of its failure modes produces a
*clean-looking* result.

**Check the output resembles what you asked for, not merely that the command
exited 0.**

- **The flag you meant is a different flag.** In ripgrep, `-E` is `--encoding`,
  not GNU grep's extended-regex; `-r` is `--replace`, so `rg -rn '<pat>' .`
  *succeeds* and prints every match with the pattern substituted away — a
  plausible hit list with fabricated contents. Add `2>/dev/null` and a flag
  error becomes indistinguishable from a clean sweep.
- **The search space excludes what you are searching for.** `.grove/` is a
  dotdir ripgrep skips without `--hidden` — the live mandate sits outside the
  default search space.
- **Rendered output is the wrong surface.** Scraping `--help` means reproducing
  the renderer's layout rules, and a multi-paragraph description can silently
  switch a whole command into a different layout. Ask the library for the fact
  (walk the command tree) rather than parsing what it printed.
- **The instrument is correct but blind.** A well-formed pattern that matches
  nothing *anywhere* reads exactly like a clean repo. This is the mode that
  survives careful flag-checking, and only the control pair catches it.

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
surface, then classify each one. That is complete by construction, and it is
what finds the sibling class nobody thought to list.

**Three ways a sweep is narrowed without anyone deciding to narrow it**, each of
which has passed for "done" and then leaked:

- **By claim, not file list.** A file list is written *before* the work and goes
  stale; the claim cannot, because the claim *is* what went stale. Grep the
  claim.
- **By scope.** A claim's scope is part of the claim, and a scope stated as a
  path or a set of directories goes stale exactly as a file list does — and can
  never reach the files that are in **no** tree (a root manifest, a dotfile).
- **By structural level.** A finding against a *section* does not reach the
  document's **summary layer**. A roll-up summarises sections and is not touched
  by correcting one, so when a finding lands against a heading, sweep the
  abstract, the table, and the overview too.

**Never document a claim with a count of itself.** *"The string occurs three
times, only one is a heading"* is self-invalidating the moment the sentence
stating it adds another occurrence. State the structural fact instead (*only the
heading is ever a whole line*), and say explicitly not to replace it with a
count.

**A clause rescued by its neighbour is not correct** — it is load-bearing on the
sentence you are about to delete. Before removing a false clause, check whether
the true one beside it only reads as true in its company.

## Doubting inside a picked Grove leaf

The composition rule starts from a procedural fact: this session ran
Bootstrap, invoked `grove-llm pick` itself, and adopted the returned leaf.
Merely finding `.grove/` or inheriting `GROVE_*` values does not activate it.
Outside that predicate, follow doubt-driven development's standalone bounded
cycle unchanged.

Inside it, a picked plain producer may materialise **at most one** fresh-context
reviewer across the **whole picked leaf**. The allowance is not per decision or
artifact. Each independently spawned diverse-lens reviewer counts, and an
explicitly chosen cross-model reviewer would spend the same one allowance; do
not offer a second pass after it.

Use that one pass for a narrow, unexpected claim whose correctness the compiler
cannot establish:

1. State the claim and why it matters.
2. Extract only the artifact and its contract, stripping the conclusion.
3. Give one fresh context the artifact and contract with an adversarial "find
   what is wrong" prompt.
4. Re-read the artifact and classify every finding as contract misread, valid
   and actionable, visible trade-off, or noise.

If the pass finds a substantive actionable issue, a non-mechanical fix normally
creates a second review need. Do not spawn it: promote the producer into Grove's
review chain and finish only to a coherent reviewable boundary. Trivial findings,
noise, visible accepted trade-offs, and fixes conclusively covered by an
executable test seam stay inside the leaf.

The other kinds are deliberate exceptions. A producer already in a chain runs
no in-session reviewer because its `review-*` is scheduled. A `review-*` leaf is
already the adversarial read and produces findings, not fixes. An
`integrate-review-*` leaf may use one narrow reviewer; substantial redesign is a
new producer review chain inside the owning chain node. The two `research`
producers and `combine-research` use their two-corpus and adversarial-combine
disciplines instead. A load-bearing decision derived from research gets its own
reviewed producer chain.

## The review chain — when doubt earns its own leaves

The review chain is `X` → `review-X` → `integrate-review-X`: a producer, a fresh
context asked to disprove it, and a leaf licensed to act on verified findings.
Cut one proactively for a load-bearing artifact such as a landed spec, a
decomposition others will build on, or a subsystem. Promote reactively when a
picked plain producer reaches the second-review boundary above. This is an
orchestration boundary, not a proxy for artifact size or vendor preference.

Cutting one under node `[12]`, for a design that has not been written yet:

```
grove-llm leaf-add-chain [12] sync-design --kind design
```

One call, one `sync-design-chain/` node holding three leaves — `sync-design`,
`sync-design-review`, `sync-design-integrate`. You name the producer kind and
nothing else: the verb derives `review-design` and `integrate-review-design`,
which is the part worth not doing by hand. `--kind review-impl` beside a `design`
producer is a perfectly valid invocation, and what it buys is a reviewer reading
for correctness, security and tests where it should be asking whether the ADRs
are a minimum coherent set — a discipline mismatch nothing downstream detects.

Six habits make the chain worth its three sessions:

- **Cut planned chains together.** `leaf-add-chain` derives the two later kinds,
  keeps all three steps contiguous inside one brief-less node, and writes stable
  `Reviews` / `Integrates` relationships. A late step for an existing chain uses
  `leaf-add <chain-node> <stem>-late-step`, keeping it ahead of work outside.
- **Promote a picked plain producer atomically.** Once its one-review allowance
  is spent and more review is needed, run:

      grove-llm leaf-promote-chain <picked-producer>

  It moves the producer byte-for-byte into a new brief-less node, preserves its
  handle, derives both steps and relationships, and prints node, relocated
  producer, review, and integration paths. A `PROMOTING-*` witness makes an
  interruption fail closed; retry the stable handle, stale path, or exact
  witness path rather than cutting leaves by hand.
- **Name the chain off the producer's stem** — `<stem>`, `<stem>-review`,
  `<stem>-integrate`, under a `<stem>-chain/` node. The chain verb does this for
  you, and it is what makes `find .grove` — and any file manager — show the chain
  as a chain without opening a file. The suffix goes on the end for a reason: a
  prefix (`review-sync-design`) groups every review together and scatters the
  chains (`TASK-FORMAT.md`). Don't write a `BRIEF.md` into the node: its absence
  is what tells the Retire cascade this is a chain rather than a decomposition, so
  its close has nothing to do — no `Done when` to check, no brief to promote.
- **The reviewer produces findings, not fixes.** A reviewer that starts editing
  has collapsed the chain back into one session and lost the independence that
  was the point. `review-prototype` is the sharpest case: it is *not* a code
  review — a prototype is judged on whether it informed a decision, and polish
  in one is a defect.
- **The review is inspection-only.** Inspect the producer's committed diff,
  source, requirements or specifications, and recorded verification evidence.
  Do not run test, build, lint, or format commands or redo the implementation;
  the paired integration task owns every fix and all post-fix verification.
- **The `integrate-review-*` step triages; it does not capitulate.** Each finding is one of
  four things: a contract you stated unclearly (fix the contract), a real issue
  (fix the artifact), a real trade-off (accept it *visibly*), or noise raised for
  want of context (note it, move on). If integration discovers substantial
  redesign, add a new producer review chain **inside the owning chain node** so
  it runs before work outside; an integration leaf itself is not promotable.
- **Route the review, don't hand-place it.** "Reviews go to codex" is policy —
  one `GROVE_REVIEW_HARNESS` line covering all five `review-*` kinds — not a
  per-leaf declaration. `leaf-add-chain` refuses `--harness` for exactly this
  reason. Reach for a `**Harness:**` line only where a policy genuinely cannot
  express the shape, which is the vendor pair and nothing else.
- **Treat diversity as a warning, not a gate.** Producer retirement applies
  `DONE` first, then writes the live linked review's `Producer launch` receipt
  best-effort. A direct producer names itself as source session; a reviewed
  decomposition node names the factual closing leaf and an independently
  computed producer generation, so reorder stays current and reopen becomes
  stale. Review launch consumes evidence retained by the guarded routing peek,
  warns unless both harness and exact model selector differ (or comparison is
  uncheckable), scopes the notice to the factual pick, and still launches. Grove
  owns that route; do not add a competing doubt reviewer.

grove does not require a review after every producer or parse step suffixes and
positions as grammar; skipping a chain remains normal. It does parse the stable
relationships needed for promotion and producer handoff. A chain is still not a
scheduling unit: `pick` walks through it and then into the next sibling, but a
sibling-level insert cannot split its contained children.

## Externalizing surfaced work

grove's value is many small, low-context sessions — each leaf picked up fresh,
driven, and retired before the next begins. That value leaks away the moment a
session quietly absorbs work that should have been its own leaf: context fills
with half-related threads, the session swells past one focused unit, and the
clean fresh-relaunch boundary the loop is built on is gone. So when work
surfaces mid-session, the **default is to externalize it as a new leaf**, not to
fold it in. `SKILL.md`'s Decompose step states the rule; this is the habit that
honours it.

Two triggers, two verbs:

- **A new concern surfaces** — the human raises something, or a tangent appears
  that does not serve *this leaf's stated goal*. Append it to the tree with
  `leaf-add` (or `leaf-insert` when it must sequence ahead of live leaves) and
  keep driving the current leaf. The pull to handle it "while you're here" is
  exactly the pull to resist.
- **The current leaf proves bigger** than its brief assumed. Decompose it with
  `leaf-decompose` and do only the first child this session; the remaining
  children are leaves a later session picks up with fresh context.

How to tell inline from externalize: the test is **"does this still fit *this*
session," not "can I finish it."** You almost always *can* finish it — that's
the trap that grows a runaway session. Ask instead whether the work still serves
this leaf's stated goal *and* stays inside one focused, low-context session. If
either answer is no, externalize.

Externalizing is cheap, so spend it freely. A permanent key never moves, a
renumber is a single file move that rewrites zero file contents, and `leaf-insert`
exists precisely so a late-surfacing concern can slot ahead of queued work
without disturbing it. A tree that keeps sprouting small, concrete leaves is the
system working as intended — lazy means *just-in-time, not few* (`SKILL.md`
constraint 4), so don't ration leaves to keep the tree looking tidy.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/to-tickets/SKILL.md, vertical-slice-rules and the
     wide-refactor expand-contract exception) — MIT licensed; see
     LICENSES/mattpocock-skills.LICENSE. -->

## Find working increments before child leaves

Before slicing a design into leaves, search actively for the **smallest
independently useful working increments** and order them by dependency. Create a
separate grove for each obvious stage that leaves the product working and
delivers useful, verifiable behavior on which its successor can build. Changes
that cannot independently leave the product working stay in the same increment
even when their code edits lie in different modules. Only then cut the current
increment into child leaves.

The boundary is product behavior, not code location or one design document's
scope. Schema expansion, caller migration, lifecycle cutover, cleanup,
methodology, and documentation often form dependency-ordered groves when every
handoff remains green. A new schema and the only reader that makes it usable do
not: neither half is a working increment on its own.

## What a good child leaf looks like

Externalizing tells you *when* to split; this is what the split should
produce. A good child leaf is a **vertical slice**: it cuts a narrow but
complete path through everything its goal touches, not a horizontal layer
that sits dead until its siblings land. The test is independence — can this
leaf's work be demoed or verified on its own, without waiting on a sibling? A
leaf that fails this test needs its lines redrawn, even if it still fits one
session. This is a second axis alongside "fits this session" (`SKILL.md`'s
Decompose step), not a replacement for it.

**Wide refactors are the exception.** A mechanical change whose blast radius
fans across the whole codebase — rename a shared column, retype a symbol
every caller touches — breaks too many call sites at once for any vertical
slice to land green. Sequence it **expand → contract** instead, each stage
its own leaf, added in order with `leaf-add`: first a leaf that adds the new
form beside the old so nothing breaks; then a leaf per migration batch (sized
by blast radius — per package, per directory), each keeping CI green because
the old form still exists alongside the new; finally a leaf that deletes the
old form once no caller remains.

<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Fog of war" / "Not yet
     specified") — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## Recording fog without pre-slicing it

grove's laziness (`SKILL.md` constraint 4) means a leaf either exists or it
doesn't — there's nowhere to keep the dim view of work you can see coming but
can't yet leaf-shape. A brief's **On the horizon** note (`BRIEF-FORMAT.md`)
is that place: a line or two recording foreseen work without pre-slicing it
into a leaf.

The line between a horizon note and a leaf is the **fog-or-ticket test**: can
you state the question precisely right now — not whether you can answer it.
A question you can already phrase precisely earns a leaf immediately with
`leaf-add`, even if it's blocked and unanswerable today. A question still too
dim to phrase that precisely stays a horizon note until a later session
sharpens it enough to graduate.

## Prune, reorder, or file an issue — the triage a status word would hide

A leaf whose place in the tree is in doubt — surfacing new, or already sitting
there under a doubted premise — tempts a status word: `blocked`, `deferred`,
`superseded`. Resist it; that is exactly the taxonomy ADR *pruning* rejects,
and reaching for a fourth state is how a tree starts lying about what is still
live. The doubt always resolves to one of three existing mechanisms:

- **Not now, but still ours** → a **reorder**. The work is good and belongs in
  this tree, just not next: leave the leaf live and `leaf-insert` something
  ahead of it (or reorder by hand). Nothing is decided against — `pick` will
  still return it in its turn.
- **Not ours at all** → a **GitHub issue**, not a leaf. Work that belongs to a
  different repo, owner, or workstream gets filed and dropped rather than grown
  into this tree — this grove's own charter (issue #2) is the worked example.
  If the concern already has a leaf here, prune it once it's filed: the work
  still needs doing, just not in this grove.
- **Decided against** → a **prune** (`grove-llm leaf-prune`, HITL, `SKILL.md`
  "Retire"). The path was considered on its merits and rejected; the leaf
  stays in the tree `ABANDONED`, and the *why* goes to the ADR set, not the
  filename (ADR *pruning*).

Scope the prune to the decision. Pruning only a reviewed producer deliberately
writes no handoff receipt and leaves its sibling review next, uncheckable. When
the producer, review, and integration path are all rejected, prune the enclosing
review-chain node instead so every live step closes together.

Misfiling any of the three corrupts the tree in a different way: reordering a
decided-against leaf keeps a dead end reading as "still coming"; pruning a
not-yet-due leaf erases work nobody rejected; leafing a not-ours concern grows
the tree with work this grove will never do. When a leaf's status is in doubt,
name which of the three sentences above is actually true before reaching for
the CLI — the sentence picks the verb.

## Anti-patterns

- **The wizard.** A capture verb that opens an interactive prompt
  sequence ("title? description? type? component?") breaks
  mid-flow concentration. The prior-art evidence on this is
  emphatic — see ditz's failure mode in the postmortem survey.
  Capture must be one non-interactive gesture (flags or stdin).
- **The decision summary at session end.** Don't reconstruct
  decisions in the commit message or a session-summary file. The
  inline running log and the ADRs together are the durable record;
  anything else duplicates and rots.
- **The "ask if you have questions" non-prompt.** Vague invitations
  produce vague responses. If you want input on a specific
  trade-off, ask about that trade-off specifically. The grilling
  format's recommended-answer-with-evidence structure makes the
  trade-off visible by construction.
- **The pre-baked answer.** If you already know the answer and just
  want it executed, you don't need a grilling session — that's an
  `impl` task. The grilling discipline exists for genuinely open
  decisions; using it for pre-decided ones is theatre.

## The shortest version

If you remember one paragraph of this doc, remember this:

> Commission research with a brief that names the downstream
> questions; demand primary-source citations and per-system
> walk-away checks. When the research is in, grill one question at
> a time with recommended-answer-and-evidence; ask the LLM "WDYT"
> before you commit and ask for pushback when it agrees too
> easily; record each settled decision inline in the task file as
> it lands. At the end, ADRs cite the research by primary source;
> the research doc gets a "Findings adopted" pointer back. That's
> the loop.

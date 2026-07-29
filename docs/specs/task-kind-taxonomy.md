# task-kind-taxonomy

## Problem

Every grove leaf declares a **task kind**, and the kind is the only thing grove
knows about a session before it launches it. It has to carry two loads at once:
it names the *discipline* the session runs under, and it is the routing key that
decides *where* and *on what* the session runs.

A set of five — `planning`, `research`, `prototype`, `work`, `review` — carried
neither load well once a workstream ran across more than one vendor.

- **One `review` for everything.** Reviewing a decomposition ("are these slices
  vertical, is anything missing?") is not the same read as reviewing code, and
  neither is reviewing a requirements list. One label meant one discipline and
  one model bucket for five genuinely different reads.
- **No step after a review.** A review produces findings; something has to
  triage and apply them. That session had no name, so it borrowed `work`.
- **`planning` did two jobs.** Eliciting *what* to build and deciding *how* to
  build it are different sessions with different participants, fused under one
  label.
- **`work` named both a member and its own category.** The other four kinds were
  routinely described as "work-shaped sessions".
- **Routing could not express a vendor pair.** Two research surveys that differ
  *only* by which vendor runs them are the same kind, and a kind→harness function
  cannot send one kind to two places.

## Solution

Seventeen kinds, **parameterised rather than flat**: five producers, each with
its own review and integrate-review step, plus a research pair.

|  | producer | review | integrate |
|---|---|---|---|
| | `requirements` | `review-requirements` | `integrate-review-requirements` |
| | `design` | `review-design` | `integrate-review-design` |
| | `planning` | `review-planning` | `integrate-review-planning` |
| | `prototype` | `review-prototype` | `integrate-review-prototype` |
| | `impl` | `review-impl` | `integrate-review-impl` |
| | `research` | | `combine-research` |

The set is still **closed**, still enumerable, still inherited by
`leaf-decompose` — ADR *task-kind-taxonomy* holds, and its gate-on-write /
degrade-on-read asymmetry is untouched. Only the count and the membership change.

Two composition patterns use the set, and **grove enforces neither** (see *The
grammar is documented, not enforced*):

- **The review chain** — `X` → `review-X` → `integrate-review-X`. Sequential, and
  each step is a *different kind*, so per-kind routing alone expresses it. This
  is the pattern behind "reviews go to codex, integration goes back to the
  implementer."
- **The vendor pair** — `research` → `research` → `combine-research`. The two
  producers are the *same kind differing only by vendor*, which is exactly what a
  kind→harness function cannot express — so this pattern is why a per-leaf
  harness declaration exists. The fan is a **pair**, not an N-way fan-out, so the
  combine step is binary and nothing generalises past it.

The two patterns differ in character, not only in shape. The review chain is
**adversarial**: the reviewer's job is to find fault. The research pair is
**breadth-and-confirmation**: two independent surveys, unioned.

## Decisions

### The disciplines

Each kind is marked **HITL** or **AFK** — see *HITL and AFK* below for what the
mark means and does not mean.

**Producers**

- **`requirements` (HITL)** — establish *what* should be built. This is where the
  grilling procedure lives: interview one question at a time, propose a
  recommended answer for each, walk the design tree until shared understanding is
  reached. Sharpen the glossary inline as terms resolve.
- **`design` (AFK)** — given requirements, establish *how*. The deliverable is a
  spec, an ADR set, or both. A `design` session that finds itself cutting
  *implementation* leaves has drifted into planning's job and should externalize
  a `planning` leaf instead.
- **`planning` (AFK)** — given the design, cut it into vertical slices and
  **grow the tree**. The deliverable is *more tree*. The only kind with
  methodological force: the sole branch in the loop's Execute step.
- **`prototype` (HITL)** — a cheap, deliberately throwaway artifact built to
  react to, not to ship. The point is the reaction it provokes, not the code's
  survival.
- **`impl` (AFK)** — produce code, docs, or tests. The deliverable is an
  artifact that ships.

**Research**

- **`research` (AFK)** — a citation-disciplined literature or prior-art survey
  producing `docs/research/<slug>.md`. Breadth-seeking: a citation per
  failure-mode claim, primary sources, and an explicit note where a search found
  silence (the absence is itself a finding).
- **`combine-research` (AFK)** — union two surveys' coverage and flag every
  disagreement. This kind, not `research`, carries the **adversarial move**: two
  vendors on overlapping corpora can agree on something false, so
  **agreement without independent primary sourcing is a red flag, not a
  confirmation**. That is the one check neither survey can perform on itself.

**Reviews** — each is a fresh-context adversarial read of one artifact,
producing findings, not a fix. All AFK. They differ in what they look for:

- **`review-requirements`** — is anything missing? Is each requirement
  falsifiable? Does this describe the human's actual need or a solution smuggled
  in as a requirement?
- **`review-design`** — does the design satisfy the requirements? Are the ADRs a
  minimum coherent set? Are the seams at the right height and the right count?
- **`review-planning`** — are the slices vertical? Does each land green on its
  own without waiting on a sibling? Is anything missing from the decomposition?
- **`review-prototype`** — does the prototype actually probe the question it was
  built for? Unusually, this is *not* a code review: a prototype is judged on
  whether it informs a decision, and polish is a defect in it.
- **`review-impl`** — correctness, security, tests, and adherence to the
  project's conventions.

**Integrations** — each triages one review's findings and applies the real ones.
All AFK. The shared discipline is the receiving-review move: verify each finding
rather than performatively agreeing, and classify it as *a contract stated
unclearly* (fix the contract), *a real issue* (fix the artifact), *a real
trade-off* (accept it visibly), or *noise raised for want of context*.

What separates them is **what the session is permitted to change**, which is a
real constraint gradient rather than a change of noun:

- **`integrate-review-requirements`** edits what was asked for. A finding of
  "this requirement is unclear" can be resolved by editing; a finding of "this
  requirement is wrong" cannot be resolved alone. This is the one borderline cell
  in the HITL table — it is marked AFK, but it is the kind most likely to stop
  and ask, which is always legitimate.
- **`integrate-review-design`** may rework the ADR set, which has its own
  in-place discipline: merge, split, or delete, never a superseding record, and
  reconcile every citation the rework leaves dangling.
- **`integrate-review-planning`** may reshape the tree — the same reactive
  decomposition any kind may perform, applied to a decomposition under review.
- **`integrate-review-prototype`** decides what the prototype *taught*, and
  normally discards it. Preserving prototype code is the failure mode here.
- **`integrate-review-impl`** edits code, fully within the agent's remit.

### HITL and AFK

A kind is **HITL** when *a human's own words are the session's input or its
deliverable* — not merely when a human would want to read the output, which is
true of everything and which the VCS already serves. Two kinds qualify:
`requirements` (the requirements are the human's; nobody else can supply them)
and `prototype` (the deliverable is the reaction it provokes).

**The mark predicts; it does not permit or forbid.** HITL means the session
*cannot* complete unattended by construction. AFK means it *normally* can — not
that it may not stop and ask. **Any kind may ask a human a question at any time,
and doing so is always legitimate, never a fault.** `design` in particular often
will. There is no machinery behind either mark; a HITL leaf reached by an
unattended relaunch of the self-driving loop simply waits, which is correct
behaviour.

### A chain is a node directory, named by a shared stem plus a step suffix

The kind lives inside the task file, so a tree of leaves shows *what* was worked
on and never *how it was composed*. A chain carries that in its **shape** — it is
a node directory — and its steps carry it in their **names**:

```
07-sync-design-chain-k11/            # chain node — no BRIEF.md
  01-sync-design-k12.md              # design
  02-sync-design-review-k13.md       # review-design
  03-sync-design-integrate-k14.md    # integrate-review-design

08-sync-survey-pair-k15/             # chain node — no BRIEF.md
  01-sync-survey-a-k16.md            # research, **Harness:** claude
  02-sync-survey-b-k17.md            # research, **Harness:** codex
  03-sync-survey-combine-k18.md      # combine-research
```

**The directory is what makes the group structural.** A flat run of stem-sharing
leaves is *contiguous and similarly prefixed*; nothing in a listing distinguishes
it from three adjacent unrelated leaves that happen to share a prefix. A directory
is a first-class object in every tree viewer there is — `yazi`, Finder, `ls -R`,
`find .grove` — collapsible, selectable and countable without any of them being
taught the convention. That is the decisive point, and it is the one the earlier
rejection got wrong: the flat shape's legibility argument was carried by the
[[Tree viewer plugin]] and by `find`, and grove controls exactly one of the
viewers a human actually uses.

**The three arguments that had rejected it have all lapsed.** They are recorded
here because the request is a natural one to re-raise, and re-raising it should
start from why the answer changed rather than from the conclusion:

- *"No verb behind it — a node is created only by `leaf-decompose`, a deliberately
  reactive verb, so cutting a chain up front applies it speculatively."* Dead:
  `leaf-add-chain` / `leaf-add-pair` are proactive shape-emitting verbs that did
  not exist when this was written, and emitting a directory is the same one call.
- *"Buys a `BRIEF.md` written because a step demanded it (constraint 4)."* Dead:
  a chain node is **brief-less by rule** (below), so no brief is written at all.
- *"The Retire cascade asks the human before treating a node as done, which is
  pure noise for a chain whose integrate step finishing means it is finished by
  construction."* Dead, and it is the brief-less rule that kills it — the cascade
  asks of **brief-carrying nodes only**, so a chain node closes silently.

What survives is that node-ness now means two things: *this proved bigger than one
session*, and *these steps compose one artifact*. The `BRIEF.md` discriminator is
what keeps that a distinction rather than an overload.

**The node is brief-less by rule**, and the rule does real work beyond saving a
stub. It gives the Retire cascade a discriminator that is a **file's presence, not
a name pattern** — so the cascade can skip a chain's confirmation without any
reader parsing the step suffix, which is the line the next section says must never
be crossed. A charter-writing alternative was available and rejected: the verb
could emit a stub naming the artifact, which `brief-chain` would then deliver to
all three sessions as shared context. It loses on constraint 4 — the stub is
written because a step demanded it, by a verb that knows only a stem — and on the
discriminator, which a brief-carrying chain node would destroy. Nothing is
enforced: a human who writes a `BRIEF.md` into a chain node has made it
brief-carrying and gets the confirmation that follows.

**The children keep the stem; the node takes a `-chain` / `-pair` token.** The
tempting shape is `NN-<stem>-k<key>/` holding `01-impl`, `02-review`,
`03-integrate` — much the cleanest listing, and needing no naming convention at
all. It is rejected on two checkable facts about references, both in
*task-tree-scheme* §5. `resolve` matches a bare slug **exactly** and reports more
than one match as ambiguous, so a second chain in the same grove makes `resolve
review` ambiguous. And `.grove/` is deleted at the finish cycle, leaving commit
messages as the only surviving record — where `review-k4` names a role and no
artifact, against `sync-design-review-k13` which still says what it was. The
redundancy in the listing is the price of a handle that outlives the tree that
explained it. The node's `-chain` / `-pair` token is the same constraint applied
one level up: without it the node's slug collides with its own first child's.

The pair's producers are `-a` / `-b` rather than a bare stem beside a `-second`,
because they are peers rather than a producer and a step.

**The suffix is terminal, not leading.** A suffix keeps a chain's steps sorted
under their stem; a prefix (`review-<stem>`) sorts every review beside every
*other* review. Containment makes this weaker than it was — the steps are a node's
children now, so nothing unrelated is interleaved with them either way — but it
still decides how a chain's *handles* read in a commit log, which is the surface
that outlives the tree.

**Nothing parses the suffix**, and adding a parser would be the signal the
convention had overreached — see the next section. It remains a habit, on the same
footing as the patterns it names; the directory, not the suffix, is what any
machine or viewer keys on.

**Existing flat chains are not migrated, and the reason is load-bearing.**
Detecting one in a live tree means matching `-review` / `-integrate` against a
preceding stem — precisely the suffix parsing this design forbids — and it would
misfire on a leaf legitimately named `foo-review`. A flat chain is not malformed:
nothing parses the convention, so a run of adjacent leaves is a valid tree that
`pick` walks correctly. Old chains stay flat and keep working; the verbs write the
new shape from here. There is no format bump and no work for `grove do`'s
migration step (*task-tree-scheme*, *Migration*).

### The grammar is documented, not enforced

The two patterns are conventions a human composes by hand. grove does not
validate that a `review-X` leaf follows an `X` leaf, and does not warn when one
does not.

The reason is not that sibling positions are mutable, though they are
(`leaf-insert` shifts every later sibling, so any rule over sibling order would
be re-litigated by the one verb that exists to reorder work). The load-bearing
reason is that **a grammar is a relation between leaves, and grove expresses no
relation between leaves.** That is the same principle that keeps "the reviewer
must not be the author" out of grove. Enforcing a grammar would also gate, which
grove's spine forbids outright.

A non-blocking lint was considered and rejected: it would fire on a tree the
human deliberately shaped, demand no action, and re-trigger on every insert.

### A chain is not a unit either

Beyond the grammar, a chain was costed as a **first-class group**: a thing `pick`
will not walk out of once its first step is done, and whose close skips the Retire
cascade's confirmation. It is not adopted, and the reasons are worth keeping
because the request is a natural one to raise again.

**Two of the three motivating costs are not real.**

- **`pick` does not wander out of a chain.** `pick` returns the first live leaf in
  pre-order; a chain's steps are one node's children at adjacent positions, so once
  the producer is `DONE` the next name in the walk *is* the review. That is exactly
  the ordering a decomposition node's children get — contiguity is the only
  ordering grove offers anywhere, so the chain is not the weaker case, it is the
  ordinary one. A chain also survives a step being decomposed with no special
  handling: the nested node is walked in place and the remaining steps follow it.
- **A chain's close asks nothing.** The cascade's confirmation is asked of
  **brief-carrying** nodes, and a chain node is brief-less by rule — so the noise
  the group construct would remove is not there to remove. (This bullet used to
  read "the confirmation is asked per node, and a chain is not a node", which was a
  second argument *against* node-per-chain. Making the chain a node did not
  resurrect the cost; the brief-less rule is what keeps it dead, and the
  discriminator is a file's presence rather than any reading of the naming.)

**The one real gap has closed** — as a by-product, not by adopting the group
construct. It was that contiguity is unprotected against a sibling-level
`leaf-insert`, where a node's children are protected by containment; a chain is now
a node, so its steps *are* protected. The lazy-cut shape closed with it: a review
decided on after its producer ran is `leaf-add <chain-node> <stem>-review`, which
appends inside the node — immediately after its stem-mates and ahead of everything
outside — where the flat shape had `leaf-add` append behind every unrelated live
leaf. The one case still needing `leaf-insert` is a producer that was cut as a
plain leaf, which no directory can retrofit (see *Out of scope*).

**What closing it via `pick` would have cost** is `pick`'s defining property — it
would stop being a walk and become a scheduler (*task-tree-scheme*). That decision
stands and is untouched by the gap closing: `pick` is stateless and local, knows
nothing about chains, and leaves a chain node for the next sibling as freely as it
leaves any node. No grouping of leaves can be honoured by such a walk; the answer
was to make the grouping *structural* instead, where the filesystem already
enforces it for free.

**And it would gate** (constraint 5). A `pick` that refuses to leave a chain is
grove overruling the position order a human set. `leaf-insert` exists precisely so
a human can say *this goes first*; immunity would mean a defect found mid-review
cannot be sequenced ahead of the integrate step, which is grove refusing work on
process grounds — the one thing constraint 5 forbids outright. Note which way that
cuts: the request was framed as *removing* a gate (a confirmation with one sensible
answer), but the confirmation is never asked of a chain, while the containment
would be a real gate. Constraint 3 is the milder objection and points the same way —
a `**Chain:**` field is schema in the one place the spine keeps freeform — but it
is not the load-bearing one, since the kind-inference mark needs no field at all.

**The two candidate marks, costed anyway**, since the answer would be one of them
if the walk were free:

- **A brief field** (`**Chain:** true`, or a node brief declaring itself one) —
  explicit and readable, but presupposes the directory shape already rejected, is
  one more thing to hand-maintain against constraint 3, and can drift out of sync
  with what the leaves actually are.
- **Inference from the children's kinds** — a run of `X` / `review-X` /
  `integrate-review-X` *is* a chain, with no new field at all. Strictly better:
  it costs no schema and cannot drift. It is still not adopted, because the mark
  was never the expensive part — reading it inside `pick` is.

**The tempting middle option, also rejected:** `leaf-add <parent> <stem>-review`
placing the new leaf after the sibling named `<stem>` instead of at the end. It
closes the lazy-cut gap with no change to `pick`. Rejected because it makes a verb
named *add* sometimes *insert*, on the strength of a slug prefix — which means
parsing the suffix convention, the one thing the convention says it must never do
— and it is wrong whenever the human wants the step at the end deliberately.

**What would reopen this.** Not friction with chains — the gap that caused it is
now closed structurally, so the motivating cost is gone rather than merely
tolerated. Only a case where `pick` acquires a legitimate reason to be non-local
for some *other* purpose: chain-awareness could then ride along at near-zero cost.
Today it would be the sole reason, and it does not carry the weight alone.

**Blast radius of *this* answer being no:** zero code. The group construct would
have changed `pick`; declining it leaves `pick`, `leaf-add` and `leaf-insert`
exactly as they are. (The adjacent chain-as-node decision above *does* change code
— the two construction verbs — and one line of the Retire cascade's prose; that is
its blast radius, not this one's.)

### Constructing a chain is one call

*Sequencing* a chain needs no mechanism (above). **Constructing** one gets two
verbs, each emitting a whole shape in a single call:

```
grove-llm leaf-add-chain <parent> <stem> --kind <producer>
   NN    <stem>-chain-k<a>/                   chain node — no BRIEF.md
     01  <stem>-k<a+1>            **Kind:** <producer>
     02  <stem>-review-k<a+2>     **Kind:** review-<producer>
     03  <stem>-integrate-k<a+3>  **Kind:** integrate-review-<producer>

grove-llm leaf-add-pair <parent> <stem> --harness-a <name> --harness-b <other>
   NN    <stem>-pair-k<a>/                    chain node — no BRIEF.md
     01  <stem>-a-k<a+1>          **Kind:** research, **Harness:** <name>
     02  <stem>-b-k<a+2>          **Kind:** research, **Harness:** <other>
     03  <stem>-combine-k<a+3>    **Kind:** combine-research
```

The arguments are `leaf-add`'s: the same `<parent>` (`.` for the grove root, or a
node by key or path). The **node** is appended at the parent's next free position;
its three children start at `01`, contiguous, with fresh consecutive keys and the
same leaf template. Four keys per shape, not three — the node holds the first.
Stdout is four absolute paths, the node directory then its three leaves, so a
caller can immediately `leaf-add <node>` a late step into it. A generated shape is
**byte-identical to the same directory and leaves cut by hand**, so nothing
downstream can tell them apart; `leaf-add`, `leaf-insert` and `pick` are untouched.

**One call is one mutation.** This is the part that is *not* `leaf-add` three
times over, and it is the reason a composite verb is worth having rather than a
snippet in the guidance. Three separate calls give three chances to stop
half-way, and a live prefix of a chain is indistinguishable from a deliberately
hand-cut partial one — the wrong-but-well-formed residue the verb exists to
prevent, reintroduced by the verb itself. So the contract is:

- **every slug is validated and the parent resolved before the first write** — a
  malformed stem or a bad `--kind` costs nothing and creates nothing;
- **positions and keys come from one snapshot**, which is what makes contiguous
  positions and consecutive keys a property rather than a race;
- **every destination is checked free before any is written**, so the realistic
  collision is an up-front refusal rather than a rollback;
- **anything that still fails mid-write rolls the run back** — and a rollback
  that cannot complete names the residue by path, because the one case where a
  caller has something to clean up is the one case worth stating loudly. The node
  directory makes this materially easier than the flat shape did: every file the
  run created is inside one directory that did not exist beforehand, so the
  rollback is one recursive remove rather than three unlinks that must each be
  matched against what actually landed;
- **stdout is the whole shape or nothing.** The four paths print *after* the
  mutation succeeded, never as each leaf lands: a run that failed was rolled
  back, so paths on stdout would describe files that are no longer there.

The observable property is **no silent partial shape**, and it is what the tests
assert — a happy path that made three files does not test the reason the verb
exists. A retry after a refusal therefore gets the positions and keys the failed
run had planned, rather than appending a duplicate shape behind the first one's
debris.

**Why a verb here, when the answer to the group construct was no.** The two
questions look adjacent and are not. A group would have changed `pick`'s defining
property — a spine-level, permanent cost — to buy an ordering that contiguity
already gives. A construction verb changes nothing that exists; it performs a
**derivation grove already owns and currently asks a session to redo from prose**:
`<producer>` ⇒ `review-<producer>` ⇒ `integrate-review-<producer>` is precisely
what *parameterised, not flat* bought, and nothing has spent it yet. ADR
*cli-binary-split* states the rule this falls under — every deterministic step
with a stable input/output shape is a verb rather than an instruction a model
might paraphrase wrong — and bounds it (*What earns a `grove-llm` verb*).

That derivation closes an error class prose cannot. A mistyped kind is already
caught (`--kind` gates on write); a **well-formed wrong** one is not.
`--kind review-impl` beside a `design` producer is a perfectly valid invocation,
and what it costs is the whole reason the set is parameterised rather than flat:
the reviewer reads for correctness, security and tests where it should be asking
whether the ADRs are a minimum coherent set and the seams are at the right
height, and the integrate step arrives licensed to *edit code freely* rather than
to rework a decision set. That cost is unconditional. The routing cost rides on
top of it and is not: with only the family var `GROVE_REVIEW_MODEL` set, the two
resolve identically, so a misroute bites only where kind-level vars are — which
makes the discipline mismatch, not the model bucket, the load-bearing half of
this argument. Transcribing a seventeen-row table by hand three times is where
that error comes from, and a derivation is where it stops.

**Two verbs, not one with a mode.** The obvious economy is one verb dispatching
on `--kind`, and it is a false one: in pair mode `research` is the *only* legal
value, so the flag would be a mode selector wearing a parameter's clothes. The
shapes differ in slug (`-review`/`-integrate` vs `-a`/`-b`/`-combine`), in flag
(`--harness` refused vs required), and in whether a kind is a choice at all.
Splitting them lets each verb carry zero dispatch, and their refusals point at
each other.

**The kind-set consequence is settled by construction.** `research` is the one
kind with no `review-` sibling, which is exactly why it needs its own verb rather
than a shape the chain verb can reach: `leaf-add-chain --kind research` is
**refused**, naming `leaf-add-pair`, and `leaf-add-pair` takes no `--kind` at
all. Both shapes are covered, neither is implied by the other.

A third refusal falls out of the same derivation: `--kind review-design` — a
chain's *output* passed where its input goes — is refused too, listing the five
producers. The derivation is an exhaustive `match` on the kind rather than a
label transform, for the same two reasons the kind→family map is one: the
compiler makes an eighteenth kind *declare* whether it heads a chain, and the two
family labels overlap as strings (`integrate-review-impl` contains `review`), so any prefix
derivation is one loose matcher away from pairing a step with itself.

**The flags encode the routing doctrine.** `--harness` is **refused** by
`leaf-add-chain`, naming `GROVE_REVIEW_HARNESS`, because routing reviews is a
policy and not a per-leaf fact (*Routing*). It is refused *by name* rather than
merely undefined: clap's bare "unexpected argument" would reject it correctly and
teach nothing, and the one moment a session is deciding is the one moment the
doctrine is worth stating. `leaf-add-pair` refuses `--kind` the same way, naming
`leaf-add-chain`; the two refusals point at each other.

**The pair declares *both* producers, and they must differ.** Requiring a single
`--harness` — the first draft of this design — does not establish a pair. It
leaves producer A undeclared, resolving through leaf → kind → family → stamp at
*launch* time, so "these two run on different vendors" is a **forecast about the
environment**, not a fact in the tree: it is unverifiable at construction, and it
silently decays if routing policy changes afterwards. That is the same inversion
*session-leaf-binding-k28* found in the driver's routing peek, and it is answered
the same way — the tree wins. So both producers carry a `**Harness:**` line, and
an equal pair is **refused before anything is written**: two surveys on one
vendor is three ordinary research leaves wearing a pair's names, which is exactly
the "looks like one but is not" state the flag was supposed to prevent.

Declaring A costs nothing it did not already cost: a leaf naming the *stamped*
harness is not a reroute (*Routing*), so the unscoped model keys still apply to
it. What the caller gives up is the ability to say *"the usual vendor, and a
different one"* — and that is the point, because that sentence is not expressible
as a fact about a leaf. The `combine-research` step stays **undeclared**: it is
not a peer of the two producers and routes by policy like every other kind.

**`--kind` is required by `leaf-add-chain`**, where `leaf-add` defaults it to
`impl`. A default here would silently choose the producer, which is precisely the
wrong-but-well-formed kind the verb exists to stop a session picking by accident;
the asymmetry is the same gate-on-write reasoning, applied to an argument that
now parameterises three leaves instead of one.

**This does not gate** (constraint 5), and the test is the one the request itself
named: a verb that makes the chain the *easy* path is compatible; a verb that
makes a bare `leaf-add` harder, or that validates a tree's chain-completeness, is
not. `leaf-add` is untouched, no tree is inspected, nothing is warned about, and
skipping a chain stays a normal choice. The refusals above are **authoring-time
argument validation** with a human present, the same class as `--kind reserch`
and `--harness bogus` — not grove refusing work on process grounds.

**Nor is it schema** (constraint 3). The verb *writes* the naming convention; it
still never *reads* one. Nothing parses a suffix, and the convention remains a
habit a human may deviate from by using `leaf-add`. Supplying a convention rather
than asking each session to re-derive it from prose is what the templated
`**Kind:**` line already does.

**Nor is it speculative** (constraint 4). The lazily-grown thing is the *tree*,
and the decision to grow it by three is the caller's escalation call — cut a
chain for a load-bearing artifact, reach for a mid-session subagent for a
one-file change (`content/driving.md`) — made *before* the call and nowhere
inside it. The verb writes three leaves for the same reason `leaf-decompose`
writes a brief and a first child, and `root-init` a root brief and a first leaf:
the caller named a shape, not a file count.

It is, however, **lightly strained** on the other axis, and that is worth stating
rather than arguing away. The code was adopted after **one** maximally-primed
post-k29 chain and **no observed wrong-kind incident** — the error class it
closes is real and reasoned about, not measured in the field. The evidence for
the *mechanism* is therefore thinner than the evidence for the naming convention
k29 fixed, which had 26 leaves and zero chains behind it. What keeps runtime tree
growth genuinely lazy is that only the caller can reach for the composite verb;
nothing grows a chain because a step demanded one. Do not restate this as
stronger empirical support than it is.

**How a session comes to use it** — the failure mode this must clear, because it
is *compose-task-chains-k29*'s failure with a compile step. Five reference
surfaces documenting chains produced zero chains in 26 leaves; what fixed it was
putting the guidance where a session reads *while cutting*. A verb inherits that
requirement and also improves on it in a way prose cannot: `SKILL.md`'s Decompose
step and `content/driving.md` already enumerate *the verbs a session reaches for*
(`leaf-add`, `leaf-insert`, `leaf-decompose`), so the chain verbs join a list
already read at the cutting moment rather than sitting in a paragraph beside it —
and they appear in `grove-llm --help`, the bootstrap-recovery surface, adjacent to
the `leaf-add` every session already calls. The verb converts a chain from *a
paragraph to be recalled* into *an entry in the list of things you can do*. This
is a claim about placement, and it is only as good as the placement: whatever
guidance names the hand-cut procedure must name the verb instead.

**Partial chains stay `leaf-add`'s job.** Neither verb takes a `--steps` flag.
The friction they remove is the *whole shape*; a partial mode reintroduces the
per-step decision they exist to collapse, and there is no evidence anyone wants
one. Adding it later is additive.

**Why this section and no ADR of its own.** The durable record is here, plus a
bounded rework of ADR *cli-binary-split* — which owns the verb surface and the
bar a verb clears, and whose *Considered options* now carries the `leaf-add
--chain` rejection this is often mistaken for. A record for the decision itself
would fail the when-to-write test's first leg: a verb is cheap to remove, the
leaves it writes are indistinguishable from hand-cut ones, and `.grove/` trees
are ephemeral, so nothing about it is hard to reverse. The trade-off it *does*
carry (constraint 4 against a derivation grove owns) is stated above, where the
reader who needs it already is.

**Out of scope, and what would reopen each:**

- **A `leaf-insert-chain` / retrofit verb** — giving an *existing* producer leaf
  its review steps in place, which is the shape that actually breaks a chain
  (`leaf-add` appends behind every unrelated live leaf). Not adopted: it has
  never happened, and today's repair is two `leaf-insert` calls, which is exactly
  the work the verb would do. Reopen it if a retrofit becomes routine — the
  design carries over unchanged, taking a *target* where these take a *parent*.
  Its scope narrowed when chains became nodes: a step added late to a producer
  that *is already in a chain node* is a plain `leaf-add <node> <stem>-review`,
  which lands inside the node ahead of everything outside it. What is left for a
  retrofit verb is only the producer cut as a bare leaf, which would have to
  become a node — i.e. `leaf-decompose` with a derived set of children rather than
  an insert at all.
- **`leaf-add --chain`** — stays rejected; see *Considered options* in ADR
  *cli-binary-split*.
- **A `--steps` / partial mode on either verb** — stays out; see *Partial chains
  stay `leaf-add`'s job* above.
- **Reading the convention back.** Nothing parses a `-review` suffix, relates two
  leaves, or checks a tree for chain-completeness, and nothing here proposes to.
  That is the line between writing a convention and enforcing a schema
  (constraint 3), and the verbs sit firmly on the writing side. Reopening it
  would need a case that a *reader* cannot serve by opening the file — and a
  chain linter fails ADR *cli-binary-split*'s third leg before it reaches
  constraint 5.
  **The chain node does not cross this line, and the tempting way to build it
  would.** Two readers now need to know a chain node from a decomposition node —
  the Retire cascade, and anything that reports on a tree — and the obvious
  discriminator is the `-chain` / `-pair` token in the node's slug. That is a name
  parse, and it inherits every objection above: the token is ordinary slug text a
  human may omit, misspell, or use for something else. The discriminator is
  therefore the **presence of `BRIEF.md`** — a file test that holds regardless of
  naming, and that a human can change deliberately by writing a charter. The
  naming token exists solely to keep the node's slug distinct from its first
  child's for `resolve`; nothing may key on it.
- **Migrating existing flat chains.** Not adopted, and not a deferral: detecting a
  flat chain requires the suffix parsing the entry above forbids, and a flat run
  of leaves is a valid tree that `pick` walks correctly. See *A chain is a node
  directory* for the full reasoning.

**Reconciled at implementation** (this section's own out-of-scope list, now
discharged). The design deliberately left every *description of the tool* alone
until the tool existed — `content/` is provisioned to real sessions **from the
binary**, so naming a verb there before it shipped would have been this repo's
in-the-tree-not-in-the-binary gap run in reverse. With both verbs implemented,
`content/SKILL.md`, `content/TASK-FORMAT.md`, `content/driving.md`,
`content/prompts/start.md` and `docs/grove.md` name the verbs in place of the
hand-cut procedure — **replacing** it rather than sitting beside it, because
parallel old-and-new guidance is how *compose-task-chains-k29* failed the first
time — and ADR *cli-binary-split*'s normative enumeration lists both. The
guidance nonetheless reaches real sessions only when the next release is cut.

**The same obligation is open again** for the chain-node shape. This document, ADR
*task-tree-scheme* and `CONTEXT.md` now describe a shape the verbs do not yet
write, and `content/` plus `docs/grove.md` still describe the flat one — correctly,
because they describe what ships. They are the implementation's job, on the same
discipline: **replace** the flat-shape guidance rather than let the two sit side by
side. The surfaces are `content/SKILL.md` (Decompose, and the node definition),
`content/TASK-FORMAT.md`, `content/BRIEF-FORMAT.md` (which asserts every node
carries a charter), `content/driving.md`, `content/prompts/*.md` and
`docs/grove.md` — plus the Retire step's cascade prose, where "ask before treating
a node as done" becomes "ask before treating a *brief-carrying* node as done".

### Routing

Two mechanisms, answering two different questions. Neither is redundant.

- **A policy** — `GROVE_<KIND>_HARNESS` is one rule for every grove, and no tree
  knows about it: *"reviews go to codex, because that is what I pay for."*
- **A fact about one leaf** — a leaf may name its own harness, *because its
  sibling does not*. This exists for the vendor pair, which no kind→harness
  function can express.

**The leaf names the seat; the environment names who sits in it.** A per-leaf
declaration selects a *harness*; the model for that (harness, kind) pair still
comes from the environment. It is written as a `**Harness:** <name>` line beside
`**Kind:**` (`content/TASK-FORMAT.md`), and the loop driver reads both facts in
**one** peek — the peek already runs on every iteration, and a second subprocess
to read a neighbouring line would double that cost for a declaration almost no
leaf carries.

**Precedence on the harness axis is leaf, then kind, then family, then stamp.**
A declaration is a fact about *one* leaf, which is strictly more specific than a
policy that knows nothing about any tree, so it outranks both env keys. A leaf
naming the *stamped* harness is not a reroute — `rerouted` is computed against
the stamp on this axis exactly as on the env axis — so the unscoped model keys
still apply to it.

**The declaration is read strictly, where the kind is read leniently.** An
unrecognised harness name, or an empty `**Harness:**` line, **refuses to
launch**, naming the file and the registry; it does not degrade. The asymmetry
with gate-on-write / degrade-on-read is real and deliberate: a wrong *discipline
label* costs a warning and a model from the wrong bucket, while a wrong *harness*
would run the leaf on a vendor the tree explicitly said not to — the same silent
misroute a degraded kind peek already refuses. Executing a declaration grove
cannot honour is not what constraint 5 protects; that constraint is about
refusing on *process* grounds.

**Pre-flight covers the env axis only.** `grove do` resolves every harness a
*configured* var could route to before it commits to anything, because that
surface is static and knowable up front. It deliberately does not walk the tree
for declarations: the tree grows while the loop runs, so a snapshot would be
silent about every leaf a later planning session writes — which is most of them —
while duplicating a check that has to exist at launch anyway. A leaf-declared
harness that is not installed is therefore refused at launch, by name.

**Routing keys on a family, not only the full kind.** Two families exist —
`review-*` and `integrate-review-*`; the other seven kinds stand alone. A family
var lets one line cover all five of a family's kinds. This is not a new concept:
grove already runs *specific beats general* on the harness axis, and the family
axis extends the same rule along the kind axis.

**Precedence is harness-major.** Model resolution is four keys:

1. `GROVE_<HARNESS>_<KIND>_MODEL`
2. `GROVE_<HARNESS>_<FAMILY>_MODEL`
3. `GROVE_<KIND>_MODEL`
4. `GROVE_<FAMILY>_MODEL`

The harness axis outranks the kind axis because the two are different *kinds* of
constraint. The harness axis is a **correctness** axis — a codex profile name is
garbage to pi, so crossing it can yield a value that is not merely suboptimal but
invalid for the binary being launched. The kind axis is a **preference** axis — a
family's model is less specific but still the user's choice, and still valid for
that binary. Kind-major ordering would let a set harness-scoped family var lose
to an unscoped exact-kind var written with a different harness in mind, which is
the precise failure the harness axis exists to prevent.

This composes with the existing reroute rule rather than complicating it: a
**rerouted** launch (launch harness ≠ stamped harness) consults no unscoped var,
so the lattice truncates to keys 1–2. Harness-major makes that a truncation of
one ordering rather than a different ordering.

Harness override has no harness axis — scoping a harness choice by harness is
meaningless — so it is two keys: `GROVE_<KIND>_HARNESS` before
`GROVE_<FAMILY>_HARNESS`.

Env var names are formed by uppercasing the label and mapping `-` to `_`. The
grammar is unambiguous because harness names and kind labels share no token.

### Model selection is required

**A kind that resolves no model var is a configuration error, and grove fails
loudly rather than launching.** There is no implicit fall-through to the harness's
own default. See ADR *model-per-task-kind*, which this inverts from its previous
rule, and for why the inversion is not a re-opening of the rejected fallback
chain.

Exemptions, which are absences of a question rather than defaults:

- **No live leaf** — the finish-cycle iteration has no task to require a var
  *for*.
- **A harness whose model-flag template is empty** has opted out of model
  selection entirely; requiring a flag it cannot pass would make it unlaunchable.
- **Harness absence is not a missing default.** No `GROVE_<KIND>_HARNESS` means
  the session runs on the harness the grove is *stamped* to, and the stamp is an
  explicit binding recorded on disk.

A **degraded kind peek** — the kind could not be determined at all — now fails
in every case, not only when a harness override is configured. Once model
selection is required, an unknown kind can no longer be routed by guessing.

The configuration surface is **95** vars at its ceiling (17 kinds × 5, plus 2
families × 5), and full coverage needs about **9** of them: seven standalone
kinds plus two family vars. The ceiling is not the burden — the stamped harness
absorbs every kind that is not rerouted.

**The stamp absorbs the harness axis, not the model axis**, and the nine-var
figure counts one stamped harness resolving through the *unscoped* keys. Falling
through to the stamp still requires a model var for every kind that harness runs;
what the stamp saves is a `GROVE_<KIND>_HARNESS` line, not a
`GROVE_<KIND>_MODEL` one. So someone who drives groves stamped to more than one
harness needs the **harness-scoped** spellings — nine per harness — because an
unscoped value would follow a kind onto a harness it was never written for, which
is the same crossing the harness-major ordering exists to prevent. Nine is the
floor for a single-harness setup, not the expected total.

### `work` is renamed `impl`

`work` named both a member of the set and the category containing it. The rename
must not break a live grove's task files, so the two paths are asymmetric — which
is grove's existing gate-on-write / degrade-on-read asymmetry, not a new rule:

- **Read** — `**Kind:** work` resolves **silently** to `impl`. It is the previous
  spelling, not a typo, and a warning would be noise on a correct file.
- **Write** — `--kind work` on a grow verb is **refused**, with an error naming
  the replacement rather than only listing the seventeen. A human is present at
  authoring time, and the gate exists to retrain.
- The read-degradation target for an unrecognised kind moves from `work` to
  `impl` — the same kind under its new label.

The environment side needs no migration path: `GROVE_IMPL_MODEL` is required, so
a config still carrying `GROVE_WORK_MODEL` fails loudly, and that failure *is*
the migration signal.

## Test seams

No new seams. The work tests through the existing partition:

- **The whole-`grove do` seam** — run the real driver against a fake harness
  binary and assert on the recorded argv (which harness, which model flag and
  value) and on stderr plus exit code for the refusal cases. Every routing
  decision is observable here, at the process boundary: family precedence,
  harness-major ordering, reroute truncation, and the required-var failures.
  Two properties of the seam itself, both of which silently corrupt a run that
  ignores them. A **fake codex is spawned twice per launch** — the sandbox
  pre-flight, then the session — so it must discriminate on `exec` as argv[1] or
  every codex launch double-counts (*codex-gitdir-grant*, and the pre-flight's
  own asymmetry: `codex exec` carries on where the TUI dies). And **only the
  scoped `GROVE_HARNESS_BIN_<HARNESS>` spelling survives a reroute** — plain
  `GROVE_HARNESS_BIN` is deliberately ignored there, so faking *both* ends of a
  reroute needs the scoped form for each.
- **The `kind` verb seam** — task file in, label out. The `work`→`impl` read
  alias and the degradation warning land here, and so does the whole read
  contract of the `**Harness:**` line: the second output line under
  `--with-harness`, and the refusals.
- **The kind enum's own unit tests** — label round-trip, the seventeen labels,
  and the `--kind work` refusal text.
- **The grow-verb seam** (`src/tree_grow.rs`'s own tests) — where
  `leaf-add-chain` / `leaf-add-pair` land: a `.grove/` fixture in, one node
  directory holding three named and kinded files out, plus the refusals. It is the
  seam `leaf_add` and `leaf_insert` already test through, and the composed verbs
  are a directory create followed by `leaf_add` three times *into it* with derived
  slugs and kinds, so they need no seam of their own.
  Three properties here are worth **falsifying by mutation** rather than merely
  asserting, because each is load-bearing somewhere the test does not look:
  - the *derivation* — break `<producer>` ⇒ `review-<producer>` and the kind
    assertions must fail, since a wrong-but-well-formed kind is the error class
    the verb exists to close, and an assertion that only counts files would not
    notice;
  - the *absence of a `BRIEF.md`* — write one and a test must fail. This reads
    like a cosmetic assertion and is not: brief-presence is the discriminator the
    Retire cascade uses to tell a chain node from a decomposition node, so a stray
    charter silently restores the confirmation the design removed;
  - the *rollback* — a run that fails mid-write must leave no node directory
    behind, which is the whole-shape-or-nothing property restated for the shape
    that now has a container.

## Out of scope

- **Grammar enforcement or linting.** See *The grammar is documented, not
  enforced*.
- **A chain as a first-class group** — a unit `pick` will not walk out of, closing
  without the cascade's confirmation. See *A chain is not a unit either*; the cost
  is `pick`'s walk, and all three costs it would have removed are now gone without
  it — two were never real, and the third (a sibling insert splitting a chain)
  closed when chains became node directories.
- **Auto-creating a chain from one `leaf-add`.** A `--chain` flag on the verb
  that means *one leaf* would mint three where the caller asked for one, and
  would have to guess the escalation call (chain, or a mid-session subagent?)
  that is the only judgement in the pattern. Still rejected — but the objection
  is specific to a flag on `leaf-add`, and does **not** carry to a distinctly
  named verb, where the caller has already made that call. See *Constructing a
  chain is one call*, which adopts one. (An earlier version of this entry
  generalised to "the verbs stay one-leaf-per-call"; that was never true —
  `root-init` writes a root brief and a first leaf, `leaf-decompose` a brief and
  a first child.)
- **Parallelism.** The loop launches one foreground session owning the real TTY
  and watches one signal file, so N-vendor work is expressed as sequential leaves
  that do not read each other's output, plus a combine step. Behaviourally
  identical, since grove sessions share no context anyway; real concurrency would
  need separate workspaces and separate loops.
- **"The reviewer must not be the author."** grove expresses no relation between
  leaves, so a global review policy can route a leaf to the harness that authored
  it. Not adopted: the payoff is the same unquantified one *model-per-task-kind*
  declined to buy on the cross-family question.
- **A per-leaf model axis.** Every model currently in use is distinguishable by
  harness alone, so a per-leaf `Model` declaration would be machinery for a case
  that is not live. It is additive when it comes — a second optional line, not a
  design to unpick. What would reopen it: one model family genuinely running on
  two harnesses.
- **Cross-family review as a *methodological* choice.** Routing reviews to a
  different vendor here is a commercial decision — the subscription being paid
  for — not reviewer-bias mitigation. *model-per-task-kind*'s rejection of
  cross-family *selection* is untouched by this area.

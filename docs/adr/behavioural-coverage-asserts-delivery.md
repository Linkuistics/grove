# A behavioural rule is covered by asserting its delivery

The inventory classes a rule **B** when it is *observable in a session's
conduct*. A deterministic assertion enforces such a rule. The assertion states
that the rule is **present and reachable on the composed loaded path of every
kind that it binds**. No evaluation watches a session behave.

The instrument is a normalised phrase match over the loaded path that the
shipped skills compose. That path is a kind's own `grove-<kind>/SKILL.md` and
the transitive closure of what it reaches. The closure takes each skill that a
file loads by name, such as the spine, and each skill file that a file names by
path. It crosses packs, so it covers core and each supplied external pack.

The instrument is `plugins/grove/conformance.sh`, a shell runner with no
dependencies, over the files that a harness installs. It checks one pack alone,
or core together with the supplied external packs, as the
[external pack contract](../specs/external-pack-contract.md#conformance) states.
Its controls are in `plugins/grove/conformance.test.sh`.

**The instrument moved because two of the four things its walk covered stopped
existing.** It used to walk what the *binary* composed. That was the guaranteed
core of the prompt module, the provisioned `content/SKILL.md` acting as a kind
router, `reference_file(kind)`, and the closure of what those named. Once the
methodology ships as a plugin, the middle two have no counterpart. The prompt
names one `grove-<kind>` skill, and no mapping from kind to reference remains.

A walk defined over those two would therefore measure a composition that no
session receives. The runner walks the closure, which it enters through the two
`SKILL.md` files that a session actually opens.

The guaranteed core survives, but it moves **out of reach**. It is prose that
the driver composes at launch, and a runner over an installed skill set cannot
read a prompt. **One** rule is delivered there and nowhere else: the signal
step. The manifest therefore gives that rule the owner `${prompt}` and asserts
nothing about it. That is a stated gap, not a check that quietly covers
everything except that rule.

There were two such rules until `prompt-names-the-kind-k18`. The three endings
of a `finish` session rode the prompt, because the driver inlined a signal file
for each kind. The prompt now states one signalling contract for **every**
kind. The text is the same whatever the token, which makes it a contract and
not a decision for each kind.

The ending that a kind takes moved to `grove-finish/SKILL.md`, where the spine
already said that a kind's own ending belongs. So the gap became one rule
smaller. That happened because the rule came back inside the runner's reach,
not because the runner learnt to read a prompt.

**What Rust no longer asserts.** `tests/rule_ownership.rs` was the
single-source half, and it is deleted. Its 68 pinned wordings and its 3 retired
paraphrases are now in `plugins/grove/conformance/`, word for word. The coverage
walk of `tests/lifecycle_invariants.rs` and the per-kind word budgets of
`tests/loaded_path_budgets.rs` did **not** move to this runner. Their subject
was the corpus that the binary embedded and the path that it composed.

`delete-provisioning-k19` deleted that corpus, and the two tests went with it.
They were kept until then, because an assertion whose subject still ships must
not retire first.

**Reachable means reached by *this rule's* condition.** The walk works at the
grain of a file, because the owner record says that an edge reaches a file. All
the conditions in that file are then available. Delivery does not work at that
grain.

A rule counts as delivered only in one of two cases. The rule is stated on the
**static** path, which needs no condition. Or a paragraph on the path states
**the situation of that rule**. The procedure is then in that paragraph's own
file, or in a file that it names by path.

To follow any mention of the owner file instead would certify the case that the
owner record calls deleted in effect. Two conditions name one file, and the
corpus drops one of them. The procedure for the dropped condition still reads as
delivered, to a session that has nothing telling it to look.

**What binds is the boundary, stated and not blurred.** The assertion covers
*the session was told, on a path that it opens for its own kind*. It does not
cover *the session obeyed*, and no green run is evidence of obedience. That gap
is the whole cost of the decision, and the corpus pays it knowingly. The failure
that the corpus refactor can actually commit is to delete a rule, or to move it
where no session opens it. A delivery assertion catches exactly that failure.

## Why not an evaluation with a model in the loop

An evaluation that drives a real session and scores its conduct would measure
the property that the class names. It is rejected as the *standing* instrument
on three grounds, in order of weight:

1. **It is not repeatable, so it cannot be a net.** The rewrites need a check
   that is green before them and green after them. Every contributor runs it on
   every change to the methodology. A scored session is a sample. A corpus
   rewrite that weakened a rule would fail it only sometimes.
2. **It cannot localise.** A session that conducts itself wrongly does not say
   which sentence went missing. The delivery assertion fails. Its message names
   the rule, the kind, the unmet claim and the files that still state it.
3. **Cost.** Every kind times every rule that it binds is not an occasional
   expense.

The rejection is of the instrument's *place in the suite*, not of its value. A
measurement with a model in the loop is the right tool for the claim that
delivery cannot make. That claim is whether a session reads what it receives. This repository
already uses it there, deliberately and outside the suite, in
`docs/research/wording-micro-test.md`.

**What would reopen this** is a rule whose failure delivery cannot see: a rule
that every session demonstrably receives and demonstrably ignores. That is a
measurement, not a suspicion. It argues for an evaluation *beside* these
assertions, not instead of them.

## Considered options

- **A sweep for unique phrases over the whole corpus.** It is rejected because
  it answers a different question. A sweep proves that one file states a rule.
  It cannot tell a rule in a file that a session opens from a rule in a file
  that no session's path reaches. The second is deleted in effect, and the
  corpus already holds that failure. The two instruments complement each other:
  uniqueness is the **S** half of a `B+S` row, and neither replaces the other.
  Reopen never. This is a statement of scope, not a rejection.
- **Match against the whole shipped skill set instead of a path for each
  kind.** It is rejected because it passes on exactly the regression that the
  refactor risks: a rule moved into an unreachable file. Its one attraction is
  that it never goes red by mistake, which is the same thing stated as a
  defect. Reopen it if a session ever receives every skill at once. The two
  measures would then be identical.
- **Pin each rule to the file that states it today.** It is rejected. Eight
  leaves move rules on purpose, so a site pin fails on the intended change and
  reports nothing about the unintended one. The site is diagnostic output on a
  failure, not an assertion.
- **Exact-phrase assertions instead of normalised alternative groups.** It is
  rejected because a rewrite may reword. An exact pin turns every legitimate
  edit red, and it trains a reader to point the test elsewhere instead of
  reading it. The groups name the load-bearing clause of each rule and accept
  any wording that carries it.

  A **near-miss fixture for each claim** keeps that from becoming a topic
  match. The fixture is a plausible rewrite that keeps the rule's subject and
  drops the force of *that claim*, and it must fail. It is for each claim, not
  for each rule, because one fixture controls only the claim that it fails.
  The other claims could then be two common words, or accept the inverse of
  the rule, while the invariant still goes red on a sibling.

  Co-occurrence cannot always express the clause. A sentence can name a
  required conjunction while it defers one term. An instruction can contain
  another as a substring, such as `complete` inside `complete --done`. In those
  cases a claim also names the wordings that **disqualify** a paragraph.
  Reopen it for a rule whose wording is frozen. Such a rule can pin its phrase
  exactly.

# A document declares its contract

A document that passes through the editorial stages declares a **document
contract**. The contract has three parts and one schedule:

- a **structure brief** per document;
- a **prose contract** and a **format and media** part, usually shared by a
  document set;
- the **reader checkpoints**, which are the schedule.

Each part is declared by path from the document's leaf body or node brief.

The contract is an artifact, so it has its own format file in the `grove`
spine, `DOCUMENT-CONTRACT-FORMAT.md`. Authoring sessions reach that file
through one trigger in the spine's condition register. The four stages reach it
through `editorial.md`, and each stage's own skill names the parts it reads.

`draft`'s precondition keeps its three citable items, and each one grows.

1. **The reader** now includes their expertise per domain and per
   representation, and what they need no teaching in.
2. **The ordered section plan** now puts the subject model before its
   notation, and names each page's purpose and the entry routes the document
   serves.
3. **What deserves emphasis, and what is excluded** now includes the intended
   domain, stated apart from the examples that motivated the document.

Every other field takes a stated default when absent and stops no stage.

**The prose contract's default is Grove's house style.** Grove ships one prose
contract, a closed list aimed at prose a reader can understand and that reads
as a careful human writer's. Its points are literary and visual: sentence and
word conventions, no stock phrasing, a simple layout, and aids visually set
apart from normative text. It follows Google's developer documentation style
guide on the points it names and on no others. It departs from Google on code
blocks, and adds a BCP 14 reading of capitalised keywords, which Google does
not address. A set whose contract declares no
prose contract takes it. A declared prose contract governs every point it
states, and what it leaves unstated comes from the base it names: a style
guide, another prose contract, or the house style. A contract that names no
base builds on the house style. A style guide named as the base sits above the
house style: where the guide settles a point otherwise, the guide's version
governs the whole point, and the house style supplies every point the guide is
silent on or agrees with. A base the walk cannot follow, whether a missing file
or a contract already met, ends the walk at the house style for any point no
contract on it states. So every point resolves and every walk ends.
Requirements and design sessions write to the human in the house style too,
because every exchange with the human needs the understanding a document does.

## The trade-off

**The placement was computed, not chosen.** The contract's rules bind the
sessions that write it (`requirements`, `design`, `planning`) and all four
stages that read it. No single kind or family covers that set.
[Every normative rule has one owner](corpus-rules-have-one-owner.md) therefore
places the rules by occasion, and they apply at an artifact. That sends them to
the artifact's format file. Nothing else was a candidate once the input was
stated honestly.

**The evidence is about what the brief failed to carry.** Five human reading
checkpoints on APIAnyware's documentation (RC-1 to RC-5 in
[`documentation-evidence.md`](../research/documentation-evidence.md)) found
these defects:

- compressed relationships;
- notation taught before the model it expresses;
- direct rule entries without context;
- notation lessons for a reader already fluent in the notation;
- one motivating platform read as the whole domain.

Every one traces to a structure decision that the brief then required did not
record: expertise per representation, model-first order, entry routes and
intended domain. These facts are not in the sources either. The precondition
exists for exactly that class of fact, as
[the editorial pipeline is four kinds](the-editorial-pipeline-is-four-kinds.md)
explains. So the three items grow, and their number does not.

**Stages read named parts, not the whole contract.** A stage opens one format
file and reads the sections its own skill names. A session opens one more file
than a self-contained skill would need, and that is the cost the one-owner
record accepts. In return, there is no documentation skill that every session
loads, and the human ruled that out (X-01). One exception is deliberate. Nine
teaching rules bind requirements and design exchanges as well as `draft`,
`copy-edit` or both, so the one-owner record places them in `execute.md`, which
every session opens. They add about 480 words to each session's path. On
2026-09-30 the human accepted that rather than file them by topic, first for
five rules and then for the four more a design review found an exchange can
show ([`documentation-guidance`](../specs/documentation-guidance.md), §3).

**Without a default prose contract, nothing makes a set's conventions
uniform.** A set with no prose contract left `copy-edit` with only its own
tests. Each `draft` and `copy-edit` session then settled task headings,
procedure introductions, register, keywords and code typography its own way. On
2026-09-30 the human directed that Grove ship a default instead.

**The guides' conflicts were the reason to ship none, and are now the reason
the default names its sides.** The style guides disagree on task headings,
procedure introductions, register and brevity (CE-12 to CE-15). A default cannot
avoid taking a side, so each side is stated with its evidence, and a set that
disagrees states the other side. The four quotations were fetched from the
guides' current pages on 2026-09-30. All four conflicts hold, and Microsoft is
itself mixed on task headings. Where Grove has evidence beyond the guides'
defaults, it sides with Google:

- **Length.** The human directed that length limits yield to needed explanation
  (DR-34), and redundancy harms expert readers (DR-31). Google's global-audience
  page agrees: "Repeat a word if the redundancy improves comprehension".
  Microsoft's "Shorter is always better" is illustrated with marketing copy.
- **Register.** Google gives its reason: "you probably speak more colloquially
  and verbosely than you should write, at least for developer documentation".
  Microsoft's "Write like you speak" is illustrated with interface and
  marketing text. `copy-edit`'s measured figure-of-speech test already removes
  idiom (CE-10).
- **Task headings.** Front-loading evidence (CE-07, observational) breaks the
  tie, because "To" spends one of the first words a scanning reader sees.

For the procedure introduction the only reason is consistency with Google as
the base, and the house style says so.

**A closed list keeps the default measurable and safe for normative text.**
Naming Google's whole guide would add to `copy-edit` a body of knowledge no
preregistered class could score. Its content would depend on the model's recall
or on network access. It would also bring rules written for shipped products
into specs and proposals: the timeless-documentation rule would remove "not
implemented", and "avoid *should*" would contradict a BCP 14 convention (survey
§6 of [`documentation-basis-a.md`](../research/documentation-basis-a.md)).
Every point on the list traces to an inventory ID, to the human's direction,
or to both. The human added three at agreement, restating the house style as
"a mix of literary constraints and visual design constraints": no stock
phrasing, a simple layout, and aids visually set apart from normative text,
which restores the visual form X-24 proposed. A set that wants the whole guide
names it as its base.

**The house style's points are contract values, not stage rules.** Any point of
it yields to a declared contract, and no stage rule does. So its points get no
owner in the placement function. The rule that makes them bind is one of the
contract format file's rules. The house style's own file sits in the spine's
references and is opened only when a set takes it. No stage skill may state one
of its points, because a point stated there would bind even where a set's
contract replaces it.

**The cost, stated.** Nothing validates a contract's completeness (constraint 3).
A stage that ignores a field it was told to read is not detected: the
conformance runner asserts delivery, not obedience
([a behavioural rule is covered by asserting its delivery](behavioural-coverage-asserts-delivery.md)).
The preregistered comparison in
[`documentation-guidance`](../specs/documentation-guidance.md) measures whether
stages act on it, and scores the house style as its own class (T12), with its
precedence guarded as a regression (H5). Grove also now maintains a list that
tracks two external guides. A change to the house style silently changes every
set that relies on it, so a change to it reworks this record.

## Considered options

- **One documentation skill that every documentation session loads.** Rejected:
  the human asked for the knowledge to be distributed across the stages (X-01).
  Such a skill would also put every stage's material on every stage's path.
  Reopen never.
- **The editorial family file owns the contract.** Rejected because the authors
  are not family members. Either `requirements`, `design` and `planning` load a
  family file they do not belong to, or the authoring rules are stated a second
  time. Reopen if contract authorship ever moves wholly inside the family.
- **Each kind states its part.** Rejected as the drift the one-owner record
  documents: five statements of one contract, with nothing to keep them equal.
  Reopen never.
- **Stop `draft` unless every field is present.** Rejected. Term risks, a
  coverage inventory, a prose contract and a format declaration all have safe
  defaults: no coverage claim, only the charters' own rules, the house style,
  and figures in the page's own bytes. Stopping on them would halt short
  documents and every existing structure brief for no gain. Reopen if a default
  is shown producing a defect a stage then cannot correct.
- **Keep the precondition exactly as it was.** Rejected because it leaves the
  checkpoints' defects ungated at the one point where a missing brief stops the
  pipeline.
- **Ship no default prose contract.** This was the recorded decision until
  2026-09-30. It kept Grove out of the guides' conflicts, but it left every set
  without a contract with no basis for uniform conventions, and the human
  reversed it. Reopen if the comparison shows the house style moving no T12
  defect, or causing a regression that narrowing cannot remove.
- **Name Google's whole guide as the default.** Rejected: it cannot be scored,
  it depends on recall or the network, and it imports product rules that damage
  normative and proposal text. Reopen if Grove gains an offline, versioned copy
  of a guide and a way to score conformance to it.
- **Take Microsoft as the base.** Rejected: where Grove has its own evidence, it
  sides with Google, and Microsoft's own pages are mixed on task headings.
- **State the house style's points in `editorial.md` or in `copy-edit`'s
  skill.** Rejected: a point stated there binds even where a declared contract
  replaces it. The family file would also load the defaults on every stage's
  path, and neither file reaches the authors who decide whether a set takes the
  default.
- **Add a project-level lookup for a project's house style.** Rejected for now:
  a project's house style is already a prose-contract file that each set's
  contract names, as its contract or as its base. Reopen if sets in one project
  repeatedly name the same file, which would make the lookup an observation
  rather than a guess.
- **Replace the house style wholly whenever a set declares any prose
  contract.** Rejected: a contract that states only its term list would silently
  lose every other convention. Building on a named base, with the house style as
  the base when none is named, keeps each point's source traceable.
- **Let a named style guide replace the house style beneath it.** This was
  the design until a review on 2026-09-30 found that a contract based on
  Google with no exceptions then has no term-list, example-label or BCP 14
  convention. Stages would invent their own forms for fields the contract
  format requires and `draft` writes to, which is the drift the house style
  exists to prevent. Grove's own points, such as no stock phrasing, a simple
  layout and aids set apart, would also vanish beneath any guide. The human
  first chose to let the required fields fall through. Then, restating the rule
  as "google takes priority, but where there is any ambiguity, that is resolved
  by reference to the house style", they chose to keep every point the guide
  does not settle otherwise. A point is taken whole, because completing a
  guide's version from the house style could contradict it. Reopen if sets
  that name a guide repeatedly decline the same house-style point.

## What would reopen this

- **A preregistered comparison showing the deepened precondition does not move
  the defects it targets**, which would question whether the brief is the right
  lever.
- **A stage shown needing a field that a default covers badly**, which is a
  reason to promote that field into the precondition.
- **Grove gaining a validator for contracts**, which would make completeness
  checkable rather than a discipline.
- **Google changing a side the house style takes**, or sets repeatedly
  declaring the same exception to one of its points. Either is a reason to
  change that point. The quotations carry the date they were checked.

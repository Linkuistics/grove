# addressing-k38

**Integrates:** `classification-k35`

## Goal

Repair the two misdirected marker attributes and the three named
standalone-readability defects — the subset of `classification-k35`'s findings
that changes **no unit's existence, id or class**.

## The boundary, and it is your own check

**The pinned unit set must be byte-identical when you finish.**
`EMBEDDED_UNITS` in `tests/methodology.rs` is 143 ids; this leaf mints none,
retires none and reclassifies none. If you find yourself editing that array, you
have crossed into `narrative-k39`'s work — stop and externalize instead.

That also means the byte-partition and coverage arithmetic are preserved by
construction here in a way they are **not** in the sibling leaf: you are editing
prose *inside* unit boundaries and attributes *on* existing markers, never the
set of boundaries.

## Finding 1 — row 23 sends "citation discipline" to the wrong body **[high]**

Verbatim from the review:

> **Row 23 sends "citation discipline" to the wrong body.**
> `skill-adrs-and-specs` promises field-guide habits for grilling,
> research-leaf commissioning, and the review chain. Its fourth field-guide
> address is currently `driving-cite-framework-decisions-to-the-source`, whose
> condition and procedure are specifically about code depending on a framework
> version. The promised research-leaf habit is instead stated by
> `driving-how-to-write-a-research-leaf-brief`: demand a citation for every
> failure-mode claim, prefer primary sources, and record missing sources. Point
> the edge at the body the source actually promises; existing reachability is
> not evidence that the current edge is semantically valid.

**What is already verified, so you need not re-derive it** (checked at bootstrap,
against the tree as `classification-k35` left it):

- `driving-how-to-write-a-research-leaf-brief` is **`class=procedural`**
  (`content/driving.md:81`) — a legal `defers=` target.
- Dropping the current member does **not** strand
  `driving-cite-framework-decisions-to-the-source`. It keeps two other inbound
  edges: `task-producer-impl` (`content/TASK-FORMAT.md:106`, `kinds=impl`) and
  `driving-when-code-depends-on-a-framework-version`
  (`content/driving.md:324`, `kinds=*`) — the latter being the condition that
  *actually* raises it.
- The marker to edit is `content/SKILL.md:230`, a nine-member `defers=`.

**Rule on the swap, do not just perform it.** The source promises **three**
field-guide habits (grilling, research-leaf commissioning, the review chain) and
carries **four** addresses beyond the ADR/spec set. Decide and record whether the
right repair is a straight substitution or whether the review-chain habit is also
mis-addressed — `driving-review-chain-habits` exists and is procedural. A
promise of *N* addresses is only honoured if all *N* land, which is the standard
`shape-cutting-k30` and `execute-k29` wrote these rows under.

## Finding 3 — `driving-when-to-invoke-grilling` scope/prose mismatch **[medium]**

Verbatim:

> Its marker says `kinds=*`, but its complete trigger says only "a
> `requirements` leaf's brief lists three or more questions whose answers
> interdepend." Either narrow the marker to the condition actually written or
> rewrite the condition to state the intended all-kind behaviour, including
> what a non-requirements session does when it encounters those questions.
> The batch rationale cannot silently broaden semantics that are absent from
> the delivered bytes.

`content/driving.md:187`. Both repairs are legitimate; pick one and say why.

**The consequence to weigh, already traced:** the marker roots five bodies
(`grilling-interrogate`, `driving-ask-wdyt`, `driving-ask-for-pushback`,
`driving-dont-merge-questions`, `driving-record-decisions-inline`). Narrowing to
`kinds=requirements` breaks no build — the gate's reachability is **global**
(`check_reachability` in `src/methodology/whole_embed.rs`: *reachable from some
kind's mandate*), and four of the five keep other `kinds=*` roots anyway. Only
`driving-dont-merge-questions` has this as its **sole** inbound edge, so
narrowing makes it a `requirements`-only procedure. Judge whether that is correct
(the node brief says *reachability is per kind, not universal, and that is
correct, not a gap*) rather than whether it is green.

`research-moves-k25` recorded this `kinds=*` as its **least confident call**, and
noted it widened `grilling.md`'s reachability beyond the plan's rooting table.
That is context, not a verdict — the review's point stands either way: the
delivered bytes must state the semantics the marker claims.

## Finding 4, the three named repairs **[medium]**

Verbatim, first sentence only — **the closing clause of finding 4 (*"remove
file-reader-only residue and near-verbatim hub restatements"*) is
`narrative-k39`'s, not yours**:

> Rewrite `driving-when-asserting-a-repo-wide-claim` so it names its subject
> instead of opening with "The counterpart to the section above"; remove or
> localize `grilling.md`'s cross-unit `<supporting-info>` wrapper; and attach
> the relevant licence attribution comment to each separated adapted unit.

**a. The back-reference opening.** `content/driving.md:363–370`. It opens *"The
counterpart to the section above."* — meaningless in a mandate, where a unit
arrives joined to its neighbours by a blank line and nothing else. Name the
subject. `lifecycle-k31` recorded this as a **pattern**, not an instance:
condition-then-procedure prose reliably opens the procedure with a back-reference
(`skill-integration-placement` opens *"So"*, `skill-node-close-steps` opens
*"Instead"*). Those two are procedural bodies reached by an explicit `defers=`
from the condition that precedes them, so their back-reference resolves to
something the session just fetched; this one opens a **triggering** unit that
ships cold. Rule on whether that distinction holds — if it does, only this unit
needs the rewrite, and saying so retires the pattern rather than leaving it open.

**b. The `<supporting-info>` wrapper.** `content/grilling.md`: `<supporting-info>`
opens at L20 inside `grilling-domain-awareness` and closes at L111 inside
`grilling-agree-the-test-seams` — **six units sit inside an unclosed tag** when
fetched alone. Harmless to the parser (it tracks fences, not HTML blocks) and the
file is **vendored** (`mattpocock/skills`, bundled verbatim rather than
paraphrased — see its head comment). That vendoring is the tension: removing the
wrapper edits bundled prose. Decide between removing it and localizing it, and
record the call against the bundling note.

**c. Licence attribution on separated adapted units.** Review design finding 7,
three instances, all in `content/driving.md` — a licence comment is one
contiguous block, so it travelled with the *first* section it attributes:

| comment | at | attributes | unit left with no attribution |
|---|---|---|---|
| addyosmani/agent-skills | L325–328, inside `driving-when-code-depends-on-a-framework-version` | *"the two sections below"* — §*Verifying framework decisions against the source* **and** §*Doubting inside a picked Grove leaf*, which are **not adjacent** (grove's own §*Verifying a claim about the repo itself* sits between them) | `driving-doubting-inside-a-picked-leaf` (L431) |
| mattpocock to-tickets | L643–647, inside `driving-find-working-increments` | vertical-slice-rules **and** the wide-refactor expand-contract exception | `driving-what-a-good-child-leaf-looks-like` (L665), which holds the exception |
| mattpocock wayfinder (fog) | L688–691, inside `driving-recording-fog` | *"Fog of war" / "Not yet specified"* | check `driving-the-fog-or-ticket-test` (L701) and rule on it |

`LICENSES/` still carries every licence, so this is attribution **locality**, not
a compliance defect — but a session fetching `driving-doubting-inside-a-picked-leaf`
receives adapted MIT-licensed prose with nothing attached. The fix in each case
is to duplicate the comment onto the separated unit. Note that a duplicated
comment is prose *inside* an existing unit, so it stays inside this leaf's
boundary.

## Done when

- Findings 1 and 3 are repaired, each with the **reason** recorded, not just the
  edit — including whether finding 1's repair is a substitution or a wider
  re-address of `skill-adrs-and-specs`'s field-guide promise.
- The three named finding-4 repairs land, and the back-reference **pattern** is
  ruled on rather than only its one instance.
- **`EMBEDDED_UNITS` is unchanged** — the leaf's own boundary check. State it
  from the diff, not from memory.
- `cargo build` and `cargo test` green, including
  `the_embedded_unit_set_is_pinned_complete`.
- Verified from a **rebuilt, installed** binary, not the source tree:
  `grove-llm methodology` still lists 143 units, and the repaired paths are
  spot-fetched — at minimum `skill-adrs-and-specs`'s corrected fourth address,
  and `driving-when-to-invoke-grilling` at whatever scope you settled.
- Every unit you touched reads correctly fetched **alone**.

## Notes

- The residue-and-restatement sweep, `skill-loop-diagram`, `driving-field-guide`
  and `driving-anti-patterns` are **`narrative-k39`'s**. If a repair here seems
  to need one of them, that is the signal you are at the seam — record it for
  the sibling rather than reaching across.
- Retiring this leaf closes nothing: `narrative-k39` is live beside it.

## Batch record

**Eleven units edited across three files, no unit minted, retired or
reclassified.** `cargo build` green; `cargo test` green — **1,023 tests, 0
failures**, including `the_embedded_unit_set_is_pinned_complete`.

### The boundary claim, checked from the diff rather than asserted

`jj diff --name-only` matches nothing under `src/`, `tests/` or `build.rs`:
**only `content/` and `.grove/` changed.** `EMBEDDED_UNITS` is untouched at 143
ids, and the binary's listing still reports 143 rows. That was this leaf's stated
boundary and it held with no pressure against it — nothing in these four repairs
wanted a unit boundary moved.

### Finding 1 — repaired as a substitution, and the wider question ruled on

`skill-adrs-and-specs`'s fourth field-guide address is now
`driving-how-to-write-a-research-leaf-brief` (procedural, verified from the
binary). `driving-cite-framework-decisions-to-the-source` is **not stranded** —
it keeps `driving-when-code-depends-on-a-framework-version` (`*`), the condition
that genuinely raises it, and `task-producer-impl` (`impl`).

**The leaf asked whether the review-chain habit is also mis-addressed. It is
not, and the reason is that the source carries two lists, not one.** The sentence
names three *subjects* (grilling, research-leaf commissioning, the review chain);
its parenthesis names four *habits* (WDYT, pushback, running decision log,
citation discipline). The marker carries exactly four field-guide addresses, so
they answer the **parenthesis**, 1:1 — and node-brief inventory row 23's own
stated target is *"`driving.md` grilling-moves bodies"*, which is why
`driving-cite-framework-decisions-to-the-source` was the defect: it is not a
grilling move and not a habit the parenthesis names.

Declined to add a review-chain member, with the reason: `driving-the-review-chain`
and `driving-review-chain-habits` are both already addressed from `task-two-shapes`
(`triggering kinds=*`) — the family-F owner, a condition that genuinely raises
them, which `skill-adrs-and-specs`'s condition does not. Adding a fifth member
would be redundancy of exactly the kind the node brief warns launders itself.

### Finding 3 — narrowed, not rewritten, and why that is the smaller claim

`driving-when-to-invoke-grilling` is now **`kinds=requirements`**. Narrowing was
chosen over rewriting the condition because the review's complaint is that *the
marker claims semantics the delivered bytes do not state* — narrowing removes the
discrepancy at its root, where rewriting would invent new all-kind prose shipping
in nineteen mandates forever.

**Checked rather than assumed: a non-`requirements` session is not left without
the fact.** `skill-execute` ships at `kinds=*` and states *"`requirements` opens
with a **grilling session** (`grilling.md`)"* **and** defers `grilling-interrogate`
— so every kind holds both the gloss and the address. That is precisely the
argument `kinds-k22` used to narrow ten `task-producer-*` units and the aggregate
review audited and accepted; this is the same shape, not a new one.

It also returns the unit to the node brief's own rooting table, which had
`grilling.md` reachable from `requirements` alone. `research-moves-k25` recorded
the `kinds=*` as its **least confident call** and flagged it as a departure; this
declines the departure rather than making a fresh decision.

Verified per kind from the binary: `requirements` carries it, `impl`, `design`
and `review-impl` do not.

### Finding 4's three named repairs

**a. The back-reference — and the pattern behind it is ruled on, not left open.**
`driving-when-asserting-a-repo-wide-claim` now opens *"Grounding a claim about
your own codebase is the counterpart to grounding one about a framework — the
same evidence discipline, turned inward."* It names its subject and keeps the
pairing without a positional reference.

**The ruling: a back-reference is a defect in a *triggering* unit and acceptable
in a *procedural* unit its own condition addresses.** `lifecycle-k31` recorded
three instances as one pattern, but they are not one class. `skill-integration-placement`
(*"So"*) and `skill-node-close-steps` (*"Instead"*) are procedural bodies whose
`defers=` source is the condition immediately preceding them — a session holds
that condition *because fetching the body is what the condition told it to do*,
so the back-reference resolves. This unit is **triggering**: it ships cold, joined
to arbitrary neighbours by a blank line, and *"the section above"* resolves to
nothing. Only this one needed the rewrite, and the pattern is retired rather than
left for the successor.

**b. `<supporting-info>` removed, and the line drawn on span rather than tag.**
It opened in `grilling-domain-awareness` and closed in `grilling-agree-the-test-seams`
— spanning **eight** units, so seven shipped inside an unclosed tag and the eighth
closed one it never opened. **`<what-to-do>` is kept**: it sits wholly inside
`grilling-interrogate` and delivers intact, which makes it the counter-example
that shows the tag was never the problem — the *span* was.

Removal over localization because localizing would mean wrapping each of eight
units separately (noise) or wrapping only the first (what it already effectively
did). The file is vendored, so the departure is recorded **in the file's own
provenance note** rather than left implicit; it already carried grove-specific
prose in four places (the fused-skills note, the ADR-split paragraph, the
`linkuistics:decision-records` rewrite, the test-seams section), so "bundled
verbatim" was already loose.

**c. Attribution now travels with every adapted unit.** Verified by fetching each
from the binary:

| unit | now carries |
|---|---|
| `driving-when-code-depends-on-a-framework-version` | source-driven-development |
| `driving-cite-framework-decisions-to-the-source` | source-driven-development *(added)* |
| `driving-doubting-inside-a-picked-leaf` | doubt-driven-development *(added)* |
| `driving-find-working-increments` | to-tickets, vertical-slice-rules |
| `driving-what-a-good-child-leaf-looks-like` | to-tickets, wide-refactor exception *(added)* |

**A second defect in the same comment, which `evidence-moves-k26` found and the
review's finding 7 did not carry:** *"The two sections below are adapted…"* had
come to read, in the file, as the **next two** sections — one of which
(§*Verifying a claim about the repo itself*) is **grove's own prose**. So the
comment was not merely stranded, it was mis-attributing. Splitting it into two
per-section comments fixes both at once.

**Ruled on and declined:** `driving-the-fog-or-ticket-test` gets no attribution.
The wayfinder comment attributes *"Fog of war" / "Not yet specified"*, and that
concept is delivered by `driving-recording-fog`, which carries the comment. The
fog-or-ticket test is grove's own device with grove's own verb. `decompose-moves-k28`
carved both units and flagged only the to-tickets case; this agrees, deliberately,
rather than inventing a fourth instance.

### Evidence, with the instrument controlled first

**The vacuous-check trap fired once and was caught, which is the corpus's own
discipline working.** The first reconstruction reported all three files as
differing — because zsh does not word-split unquoted expansions, so the whole
newline-separated id list reached `grove-llm methodology` as a *single* id. The
fetch errored, the output was empty, and `diff` dutifully reported everything
missing: a clean-looking result from a broken instrument, exactly
`driving-turning-a-sweep-into-evidence`'s *"the instrument is correct but blind"*.
Re-run with `${(f)...}` splitting:

| file | units | result |
|---|---|---|
| `SKILL.md` | 48 | **partition exact** — 55,870 B reconstructed byte-for-byte |
| `driving.md` | 29 | **partition exact** — 46,099 B |
| `grilling.md` | 9 | **partition exact** — 5,803 B |
| `TASK-FORMAT.md` | 38 | **partition exact** — 32,779 B — **positive control**, a file this leaf never touched |

The `supporting-info` sweep carries its control too: one hit remains in
`content/`, and it is the provenance note that names the dropped tag; the
`what-to-do` pair is intact at 2 (plus the same note).

**Completeness by enumeration rather than by trusting my own list.** Every unit
in the three files was sliced from the parent revision and from the working copy
and compared — the slicer first controlled against the binary's own bytes on the
current file (MATCH on both probes). **Exactly eleven units moved, all of them
intended**, and their deltas sum to the whole-file deltas (`driving.md` +738,
`grilling.md` +194, `SKILL.md` −4) with nothing unaccounted for.

### Per-mandate effect

Only four of the eleven edited units are triggering, so most of this work costs
**a fetch, not a mandate**:

- every kind: −4 (`skill-adrs-and-specs`) −67 (`driving-when-code-depends-…`)
  +107 (`driving-when-asserting-…`) = **+36 B**
- `requirements`: **+47 B** (it keeps the grilling trigger, whose marker grew 11 B)
- the other eighteen kinds: **−709 B** (they lose the 745 B grilling trigger)

All seven procedural edits — every licence attribution and the wrapper
removal — ship in **no** mandate. Current triggering totals: `requirements`
52,686 B over 57 units; `impl` 50,109 B over 53.

### One deviation from this leaf's own `Done when`, stated rather than glossed

The leaf asked for verification from a *rebuilt, **installed*** binary. **I
rebuilt but did not install**, and verified from `./target/debug/grove-llm` — a
linked binary carrying the embed, which is what makes the claim *"served from the
binary, not read from `content/`"* true. Installing was declined deliberately:
`grove-llm` on `PATH` here is `/opt/homebrew/bin/grove-llm`, **the binary driving
this very loop**, and replacing it mid-session is an unrequested, hard-to-reverse
change to the user's machine for no additional semantic evidence — `cargo install`
is a release build plus a copy, and the copy cannot change the embed. The
node-level installed-binary check is already discharged by `finish-cycle-k32`.
Flagging it so the aggregate reviewer sees a choice, not an omission.

### Doubts for the aggregate review, by id

- **`skill-adrs-and-specs`** — the decline above rests on reading the parenthesis
  as the address list and the subject list as framing. If a reviewer reads the
  three subjects as the promise, a fifth member (`driving-review-chain-habits`)
  is owed. Recorded as the one call here that could go the other way.
- **`driving-when-to-invoke-grilling`** — narrowing withholds a 745 B condition
  from eighteen kinds. The mitigation is `skill-execute`'s `kinds=*` gloss; if a
  reviewer judges that gloss insufficient, the answer is the *other* repair the
  review offered (rewrite the condition), not a re-widened marker.
- **`grilling-domain-awareness` / `grilling-agree-the-test-seams`** — editing
  vendored prose. Recorded in the file, but it is a precedent.

### The in-session reviewer, unspent

Not used, and the reason is the predicate rather than thrift: an
`integrate-review-*` leaf may spend one **narrow** reviewer for an unexpected
doubt, and there was none. The four judgement calls are reasoned and recorded
above, the structural half is proved by reconstruction and by 1,023 tests, and
`narrative-k39` reads this record as its own starting context.

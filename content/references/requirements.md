- **requirements** (HITL) — establish *what* should be built. This is where the
  grilling lives (`grilling.md`): interview one question at a time, propose a
  recommended answer for each, walk the design tree until shared understanding
  is reached. Sharpen `CONTEXT.md` inline as terms resolve.
- **requirements** runs the grilling procedure (`grilling.md`) to interrogate
  *what* is wanted, and updates `CONTEXT.md` **inline** as terms are resolved —
  never batched.

**A fresh grove's bootstrap leaf is the standing example of that fusion**, and
it is `requirements` (the driver mints it before any agent exists, with no
`--kind` to change it). Its
only input is the human's own words — nothing else is on disk yet — which is the
HITL rule, so it is labelled for the discipline that *always* applies. A small
workstream's bootstrap session may go on to cut the leaves itself; a larger one
adds a `planning` leaf and lets a fresh session do the decomposition
(fresh-grove-start-contract).

<!-- grove reference file — the field guide: habits for driving a session well -->
<!-- adapted (paraphrased into grove's voice, not bundled verbatim) from
     mattpocock/skills@d574778f94cf620fcc8ce741584093bc650a61d3
     (skills/engineering/wayfinder/SKILL.md, "Chart the map" step 2's no-fog
     early exit) — MIT licensed; see LICENSES/mattpocock-skills.LICENSE. -->

## When not to start a grove

Before starting a grove, check that the journey actually needs a map.
If a first bootstrap session surfaces no real fog — the path to done is
already clear and the whole thing fits in one sitting — do the work directly
instead: a grove would only wrap it in a tree, a brief chain, and a relaunch
cycle it doesn't need. Start one once genuine session-to-session
fog shows up — work whose shape you can't see past the next session or two.
Bare `grove` will always scaffold a tree and mint a first leaf in a rootless
working tree; that mechanical fact is not itself the signal to run it.

## When to invoke a design discussion (grilling)

The trigger is: a `requirements` leaf's brief lists three or more questions
whose answers interdepend. Grilling is the procedure that walks the
dependency tree without the LLM making decisions on the human's
behalf.

The grilling skill (`grilling.md`) says it briefly: interview one
question at a time, propose a recommended answer for each, walk down
the design tree until shared understanding is reached. The moves
below make that interview productive rather than ceremonial.

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

## A pre-decided question is not a grilling question

The grilling discipline exists for genuinely open decisions. If you already know
the answer to a question in this leaf's brief and only want it executed, do not
stage a round of interrogation over it — record the decision and move on. If the
brief's questions are *all* pre-decided, say so rather than performing the
interview: the work standing behind them is an `impl` leaf, and running a
grilling over settled ground is theatre that costs a human's attention and
returns nothing.

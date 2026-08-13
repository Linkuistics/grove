# templated-mandate-k12

## Goal

Grill the alternative the human raised while `composition-k11` was integrating:
**could all of this be done by a single templated prompt file, with the logic
that produces the prompt living in the template?** Establish what that design
actually is, what it would replace, and what evidence would decide between it
and the composer — before anyone builds either further.

The human asked for it to run **after this grove finishes**, which is why it
sits last, behind `unit-scope-audit-k4`. That ordering is deliberate: the
question is easier to answer against a composer that exists than against one
that does not.

## Context

**The question, as asked:** *"whether all of this can be done using a single
templated prompt file with the logic in the template to produce the prompt."*

That is under-specified in exactly the way a grilling resolves, and at least
three readings fit — sharpen which is meant before evaluating anything:

1. **Template replaces the composer.** One file with conditionals
   (`{{#if kind == "finish"}}…`) emitting the whole mandate. `content/`'s nine
   marked files, `compose`, and the `kinds=`/`class=` grammar all go.
2. **Template replaces the selection, not the corpus.** Units stay marked in
   `content/`; a template decides which are included and in what order,
   replacing the Rust selection rule rather than the classification data.
3. **Template replaces only the framing and runtime facts** — roughly what
   `content/MANDATE.md` plus the driver's two runtime facts already do, with
   the composer untouched. This is the narrowest reading and may already be
   what ships after `mandate-delivery-k8`.

**What the current design bought, so a replacement can be priced against it**
rather than against a blank page. Each of these is a property something in the
tree currently checks, and the grilling should ask of each: does the template
keep it, and if not, is losing it acceptable?

- **The completeness invariant.** Every triggering unit reaches every kind its
  scope admits; every procedural unit reaches none; every procedural unit is
  reachable by following `defers=`. A template's conditionals are code, and no
  equivalent sweep over "every branch reaches the kind it should" is obvious.
- **Total partition.** Every body byte past the file directive is in exactly
  one unit, so unclassified prose cannot exist. In a template, prose outside
  every conditional is just prose that always ships — the failure is silent,
  which is the hole this design was built to close
  (`docs/adr/mandate-delivers-the-methodology.md`).
- **Byte-exactness.** A slice is the source bytes, so `content/` stays
  canonical and a mandate cannot paraphrase it. A template that interpolates is
  fine here; one that rewrites is a second source of truth.
- **Fail-on-kind-addition.** `kinds=` admits `*` and explicit lists and nothing
  else, precisely so a twentieth kind cannot be silently absorbed. Ask what a
  template does when a kind is added — most template languages fall through a
  missing branch silently, which is the failure mode
  `docs/adr/complete-session-configuration.md` already paid to avoid.
- **`content/` reads as prose.** The markers are HTML comments, so the nine
  files are still whole readable documents — and `content/SKILL.md` is still
  provisioned verbatim as a harness skill while both delivery paths are live.
  Template syntax in the body would end that, and the grilling should establish
  whether it matters after provisioning retires.
- **`grove-llm methodology <id>` addressing.** The deferred procedural half is
  fetched by id. A template with no unit grain has nothing to address.

**Prior art already in the tree.** This is not the first time a metadata
carrier was chosen: *The file's mandate order is a comment directive* in the
spec records KDL frontmatter being weighed and rejected, and the reasoning
there ("a comment directive adds nothing to own and reuses a reader the parser
must have anyway") is the same shape of argument a template has to beat. A
template language would be a **new** thing to own — read that section before
re-litigating it.

**Where the real cost of the current design is, if the answer is "yes, mostly".**
Be honest about it rather than defending what exists: nine marked files, a
parser, a whole-embed gate, a pinned id set, per-kind goldens, and a marker on
every unit is a lot of machinery, and every `content/` edit now has to pick a
class and a scope. That is the price the template would be buying out.

## Done when

- Which of the three readings (or a fourth) the human means is settled, in
  their words, and recorded here or in `CONTEXT.md`.
- Each property above is either kept by the proposed design, or knowingly
  given up with the reason written down.
- The outcome is one of: an ADR recording the composer's design as reaffirmed
  against a genuine alternative; a `design` leaf for the template; or a note
  that the question is closed and why. Do not leave it as an open opinion.
- If the answer needs evidence rather than argument — a spike, or a survey of
  what template engines do on a missing branch — cut the leaf for it rather
  than doing it here.

## Notes

**This is a grilling, not a decision.** Interview one question at a time,
propose a recommended answer for each, and update `CONTEXT.md` inline as terms
resolve (`grilling.md`). The human raised this as a research question, and the
first job is to make it a sharp one.

**Do not start by defending the composer.** It was built one increment before
this leaf runs, which is exactly the position from which sunk cost argues
loudest. The when-to-write test in `linkuistics:decision-records` is the
yardstick for whether the outcome earns an ADR.

# planning-grills-leftovers-k21

**Kind:** impl

## Goal

Finish the `planning`-no-longer-grills relabel in the two `docs/` files
`config-sweep-k16` missed, so no current-state document still tells a reader that
a `planning` session opens with a grilling pass.

## Context

Surfaced by `bootstrap-leaf-kind-k19` while settling the bootstrap leaf's kind,
and externalized rather than absorbed: it is a *different* stale claim
(`planning` grills) from the one that leaf settled (which kind `root-init`
mints), and it spans a whole file the bootstrap change had no reason to enter.

`06` relabelled `content/driving.md` — the bundled copy — thoroughly, but its
repo-facing sibling only in part (that file *is* in `06`'s commit, so this is an
incomplete sweep, not an untouched file), plus one line of a workflow
walkthrough:

- **`docs/driving-a-grove.md`** still says "a planning leaf is the right unit for
  a grilling session" and uses *planning leaf* throughout the research section
  (~10 sites) in the pre-taxonomy sense. `content/driving.md`'s equivalent
  section is already correct and is the model to follow — but the two files are
  **not** copies (different worked examples, different audiences), so this is a
  read-and-relabel, not a diff-and-apply.
- **`docs/workflows/multi-step.md`**, *Iteration 2*: "the leaf's kind tells the
  session to open with a grilling pass before doing anything else" — said of a
  **planning** leaf. Under `docs/specs/task-kind-taxonomy.md` that session cuts
  slices; it does not interrogate.

## Done when

- Neither file claims `planning` grills, and each names `requirements` where the
  session it describes is genuinely a grilling.
- `docs/driving-a-grove.md`'s research section reads coherently after the
  relabel — the surrounding argument is about *which* leaf earns a research leaf
  ahead of it, which survives the rename but should not be left half-translated.
- A repo-wide grep for a grilling claim attached to `planning` in a current-state
  doc comes back empty (`docs/research/` and historical `CHANGELOG.md` entries
  are dated artifacts and stay as they are).

## Notes

Docs only — no code, no ADR. If the relabel turns up a *third* file, add it here
rather than growing the scope past "the leftovers `06` missed".

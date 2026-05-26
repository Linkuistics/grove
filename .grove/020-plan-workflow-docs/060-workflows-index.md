# 060-workflows-index

**Kind:** work

## Goal
Write `docs/workflows/README.md` as the index for the five lifecycle walkthroughs, and add short cross-link pointers from `README.md` and `docs/grove.md` into `docs/workflows/`. Last leaf so it links real files, not phantom paths.

## Context
- The five sibling leaves (`010` through `050`) shipped the actual walkthroughs. This leaf adds the *entry point* to them.
- `README.md`'s "Use" section currently describes the CLI verbs without lifecycle context — it should gain a one-sentence "for end-to-end walkthroughs of each verb, see `docs/workflows/`".
- `docs/grove.md`'s "Driving a grove" section talks about what each verb does conceptually — it should gain a sibling pointer to `docs/workflows/` for the worked examples.
- Order in the index should match the natural reading order (install → update → start → multi-step → finish), which is also the numeric prefix order in this node and the on-disk filename order in `docs/workflows/`.

## Done when
- `docs/workflows/README.md` exists with:
  - A one-paragraph statement of what these docs are (lifecycle walkthroughs, per `CONTEXT.md`'s definition) and what they are not (not a CLI reference — that's `README.md`; not the methodology — that's `content/SKILL.md` / `docs/grove.md`).
  - An ordered list linking the five walkthroughs with a one-sentence "what this one covers" per entry.
  - A short note on the running example (`acme/orders-api`, grove `add-rate-limiting`) and a pointer to `CONTEXT.md` for terminology.
- `README.md` has a short pointer into `docs/workflows/` placed where a new reader would find it (likely just after the "Use" section).
- `docs/grove.md` has a parallel pointer into `docs/workflows/` from the "Driving a grove" section (or wherever fits without disrupting the existing flow).
- The cross-link wording is consistent across `README.md` and `docs/grove.md` so a future change updates both predictably.

## Notes
- This is the *only* leaf that touches `README.md` and `docs/grove.md`. Earlier leaves should have left them alone (per the planning brief). If they didn't, reconcile here.
- Keep the index spartan — the five walkthroughs are the content; this file is a router. Resist adding a "Common patterns across walkthroughs" section unless one emerges genuinely from writing them.
- After this leaf commits, every live leaf in this subtree is done — the next `grove continue` should `retire` the subtree, promoting any still-live notes from `BRIEF.md` upward into the root brief or into ADRs/glossary.

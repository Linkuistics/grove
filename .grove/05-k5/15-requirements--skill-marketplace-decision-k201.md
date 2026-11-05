# skill-marketplace-decision-k201

## Goal

Stop before `skill-marketplace-k185`. The human is not sure that this
workstream needs a separate skills marketplace. Get the human's decision
before any marketplace work starts.

## Context

- The human inserted this stop during `design-native-namespace-k200`, with
  the words "Please insert a stop before k185 - I'm not sure I want a skills
  marketplace".
- The root brief's "Repository destinations" requires the marketplace-only
  repository `~/Development/Delivery/skill-marketplace`. Its "Done when" says
  that the marketplace references the source repositories.
- `skill-marketplace-k185` states the planned marketplace and its open
  questions in "On the horizon".
- Native installation remains a supported user choice in the root brief.
  Today, Grove's `.claude-plugin/marketplace.json` lists the packs with
  `git-subdir` sources. The native trials of the moves install from it.
- `moves-handoff-k187` compares the marketplace pins with the owner's pins.

## Done when

- The human chooses one route, in their own words. Possible routes:
  1. Keep `skill-marketplace-k185` as planned.
  2. Keep the marketplace in Grove's `.claude-plugin/` and prune the
     separate repository.
  3. Drop native marketplace distribution for the packs, and use only
     Ravel's directory delivery for the owner.
  4. Another route that the human names.
- The root brief, the brief of `repository-moves-k5` and the affected live
  leaves agree with that decision. These include `skill-marketplace-k185`,
  `moves-handoff-k187` and any rollout text. Prune a node only after the
  human confirms it.

## Notes

Before the interview, state the effect of each route on a native user, on
the owner's cutover and on the marketplace trials. Give a recommendation with its evidence.

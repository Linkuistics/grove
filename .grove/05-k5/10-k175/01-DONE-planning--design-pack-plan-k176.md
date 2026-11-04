# design-pack-plan-k176

## Goal

Cut the implementation leaves of design-pack-k175.

## Done when

- Each Done when of the parent brief belongs to a leaf with a concrete,
  verifiable outcome.
- The leaves follow the expand and contract order of the contract. Each
  handoff leaves Grove's `bash scripts/check.sh` green.
- The plan reuses what requirements-pack-k173 established, and adds only the
  work of the `craft/` root and the craft skills.

## Decisions (running log)

- The node is one move with six leaves. Two Grove-only increments come first:
  the installer's craft root (k195) and the runner's supplier refusal (k196).
  Each is useful alone and leaves Grove green. The expand (k197), the
  contraction (k198) and the two trials (k199, k200) follow the order of
  requirements-pack-k173.
- At planning, `install.sh --pack` required `skills/`, and its link test knew
  only `plugins/<plugin>/skills/`. The runner had no supplier refusal. Both
  are therefore real work, not readings.
- The conformance runner reads only `skills/` of a pack. The craft root
  therefore needs no change to conformance, as the contract states.
- The craft skills and the kinds move in one contraction. The aggregate
  covers only the kinds. A split of the craft removal is still allowed if
  k198 proves too large, and its brief names that seam.
- The parent's Done when maps to the leaves as follows. The layout, the
  manifest, the openings, the craft bytes and the pinned citation row belong
  to k197. The provenance file, the README sentence on the formats and the
  first commit also belong to k197. The kind table, the tests, the citation
  registry and the decision record paragraph belong to k198. The marketplace
  pin, the `plugins/linkuistics` cleanup and the second commit also belong to
  k198. The installer proof belongs
  to k195, and the refusal belongs to k196. The craft-root trials belong to
  k199 and k200. Each leaf appends its results to the moves record.
- Planning findings for k197. `model-led-development` names
  `linkuistics:doubt-driven-development` in a plugin-qualified form, which
  becomes a bare name. It names Grove's formal-methods records by repository
  path, which becomes a public URL. `decision-records` adapts an MIT passage,
  so k197 checks whether that skill needs its own `LICENSES/`.
- The craft skills link `../../PROVENANCE.md`. From `craft/<skill>/`, that path
  is `plugins/design/PROVENANCE.md`, so the pack takes those sections there.
- The owner design names the Ravel source `design`. The plugin directory is
  therefore `plugins/design`, and the native namespace is `design:`.
- No in-session reviewer and no `review-planning` leaf. The subtree follows a
  proven pattern, and the membership specification already had its review.
  The two new Grove changes have executable controls that must fail first.

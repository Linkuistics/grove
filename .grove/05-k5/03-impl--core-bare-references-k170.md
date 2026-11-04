# core-bare-references-k170

## Goal

Make core's skill text name other skills by bare names only, as rule 2 of
"References between skills" in `docs/specs/external-pack-contract.md`
requires. Do this before the craft skills leave the `linkuistics` plugin, so
that no core sentence names a plugin that a later move removes.

## Context

The contract gives this node two obligations. The kinds that stay in core
take the bare-name opening. Each moving kind takes it in its own move. The
kinds that stay in core are `planning`, `review-planning`,
`integrate-review-planning` and `finish`.

At planning, the spine also cited craft skills by the `linkuistics:` alias:
`technical-communication`, `machine-environment`, `using-jujutsu`,
`codebase-design` and `decision-records`. The citing files were the spine's
`SKILL.md`, `references/grove.md`, `references/execute.md`,
`references/commit.md`, `references/review.md`, `ADR-FORMAT.md` and
`SPEC-FORMAT.md`. `references/grove.md` also states a rule about "the
`linkuistics` plugin" as one unit. After the moves, those craft skills ship in
three packs.

## Done when

- The four core kinds open with the bare-name text that the contract gives
  for `combine-research`, changed only in the kind's name.
- No file in `plugins/grove/skills/` writes a plugin-qualified skill name.
  Enumerate every `<word>:<word>` token in those files and classify each one.
  Do not search only for the known prefixes.
- The rule in `references/grove.md` names the craft skills by bare name. It
  states what binds without each one, as
  `docs/adr/a-skill-states-what-binds-without-its-dependencies.md` requires.
  It names no plugin and no pack that a later move can change.
- Each changed pinned phrase changes in `plugins/grove/conformance/rules.tsv`
  in the same change. A retired phrase goes to `removed-paraphrases.tsv`.
- `plugins/CONTEXT.md` and `plugins/grove/README.md` describe the same rule.
  A worked example can keep a qualified name when the text says it is an
  example of a native spelling.
- Core conformance, `bash plugins/grove/conformance.test.sh`, `task check:packs`
  and `bash scripts/check.sh` pass.

## Notes

The moving kinds keep their `grove:grove` openings in this leaf. The contract
gives that change to each move. A later move then changes its kinds' bytes in
the same change that moves them.

Rust tests embed spine files with `include_str!` and can assert on phrases.
Run the whole check, not only conformance.

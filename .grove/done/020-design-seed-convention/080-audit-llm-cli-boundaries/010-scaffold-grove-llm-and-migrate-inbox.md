# 010-scaffold-grove-llm-and-migrate-inbox

**Kind:** work

## Goal

Stand up a second binary `grove-llm` in the existing Cargo crate,
migrate the two LLM-driven inbox verbs (`grove inbox add` →
`grove-llm inbox-add`; `grove inbox drain` → `grove-llm inbox-drain`)
to that binary, sweep the methodology surface for the renamed
commands, and record the binary-separation decision in an ADR. This
is the foundational leaf for the rest of the subtree — every other
leaf depends on `grove-llm` existing.

## Context

- Cargo crate layout: `Cargo.toml` declares one `[[bin]]` for
  `grove` today. Add a second `[[bin]]` for `grove-llm`. Share all
  internal libs (the dispatch, the worktree/git plumbing, the
  inbox/meta state machines, the templating). The two binaries
  are thin clap dispatchers over the same library code.
- Existing inbox verb implementations: `grove inbox add` and
  `grove inbox drain` already live in `src/`; locate their
  command structs (likely in `src/commands/inbox/` or similar)
  and re-mount them under the new binary's clap definition. No
  semantic change — just relocation of the wiring.
- `grove inbox show` stays on the human surface. After migration
  `grove inbox` would be a one-subcommand cluster. Decide
  (during this leaf): keep `grove inbox show` nested, or flatten
  to a top-level `grove inbox-show`. My recommendation is to
  flatten — a one-subcommand cluster reads worse than a flat
  verb. Confirm during execution.
- Methodology and prompt surface to sweep:
  - `content/SKILL.md` — Bootstrap paragraph references
    `grove inbox drain`; the "Inboxes and capture" paragraph
    references `grove inbox add`. Update both. Also update the
    artifacts table row for the `grove-meta` branch.
  - `content/prompts/continue.md` mentions inbox drain in its
    one-liner — review.
  - `content/prompts/*.md` are otherwise thin delegations to
    the skill; recheck after the SKILL.md sweep.
- Glossary updates (`CONTEXT.md`): the `Inbox`, `Drain`, and
  `grove-meta branch` entries name the old verbs by hand —
  update inline.
- Homebrew formula at `Formula/grove.rb` (or wherever the tap
  publishes from — see `Linkuistics/taps` formula source). One
  extra `bin.install` line is the whole change.
- Documentation surface: `README.md`, `docs/workflows/*.md`, and
  the ADRs that reference `grove inbox add|drain` by name
  (ADR-0002, ADR-0004, ADR-0005). Update verb references; the
  decisions themselves are unchanged.
- Backward compatibility: decide in this leaf. Two options:
  - **Hard cutover** — `grove inbox add|drain` removed entirely.
    Cleanest. The project is pre-v1 in spirit and the inbox
    feature just shipped; users adopting it will read the new
    docs.
  - **Deprecation aliases** — keep `grove inbox add|drain` as
    shims that print a deprecation warning and dispatch to the
    new binary, removable in a later release.
  My recommendation: **hard cutover**, since the only known
  caller surface is the LLM driven by the methodology
  (`content/SKILL.md`), which this leaf updates atomically.

## Done when

- `Cargo.toml` declares a second `[[bin]]` for `grove-llm`. Both
  binaries build with `cargo build`.
- `grove-llm inbox-add` and `grove-llm inbox-drain` work
  identically to the previous `grove inbox add|drain`. The two
  old subcommands are removed from `grove`'s clap definition.
- `grove inbox show` is reachable (either as `grove inbox show`
  or as the flattened `grove inbox-show`, per the in-leaf
  decision).
- `content/SKILL.md`, every `content/prompts/*.md`, the
  glossary entries in `CONTEXT.md`, the affected ADRs, and the
  Homebrew formula reference the new commands. `git grep -F
  'grove inbox add'` and `git grep -F 'grove inbox drain'`
  return only legitimate historical references (e.g. in
  retired BRIEF.md files under `.grove/done/` — those are
  durable history and stay).
- An ADR exists at `docs/adr/0006-grove-llm-binary-separation.md`
  (next free number) recording the audience-split decision:
  context, decision, alternatives considered (single binary
  with subcommand, single binary with `argv[0]` dispatch),
  consequences. Cites the audit's running log (this node's
  BRIEF) as primary source.
- The materialised in-tree skill copy at
  `.claude/skills/grove/SKILL.md` is regenerated from
  `content/SKILL.md` so the worktree-local copy doesn't drift.
- Tests cover the migrated verbs at their new home. Existing
  integration tests against `grove inbox add|drain` are either
  ported to `grove-llm` or removed if redundant with new tests.
- This leaf is committed as one focused commit and retired into
  `done/`.

## Pointers

- ADR-0002 (`docs/adr/0002-grove-meta-branch-and-inbox-model.md`)
  and ADR-0004 (`docs/adr/0004-inbox-as-directory-of-observation-files.md`)
  introduce the verbs being migrated; ADR-0005
  (`docs/adr/0005-grove-meta-sync-semantics.md`) governs their
  sync behavior. None of those decisions change — only the verb
  names do.
- Parent BRIEF (`../BRIEF.md`) "Decisions (running log)" Q2/Q2a/Q3/Q4
  spell out the rationale that motivates the binary separation
  and the inbox-verb migration. The new ADR cites these.

## Notes

- **If this leaf overruns its session budget**, split into
  `010-scaffold-grove-llm.md` (binary set-up + ADR + stub
  dispatch) and `015-migrate-inbox-verbs.md` (move the two
  inbox verbs, sweep prose). The two halves are clean cuts:
  the first lands the binary with no behavior; the second
  reshuffles behavior into it.
- **The migrated verbs are renamed.** `grove inbox add`'s
  destination is `grove-llm inbox-add` (hyphenated), not
  `grove-llm inbox add`. The parent BRIEF's Q3 explains why
  flat verb names win on the LLM surface.
- **Internal callers.** Search for any in-process code that
  invokes the migrated verbs by name (e.g. `grove install`
  triggering drain on bootstrap). Those callers move to the
  internal library entry point, not to a shelled-out
  `grove-llm` invocation.

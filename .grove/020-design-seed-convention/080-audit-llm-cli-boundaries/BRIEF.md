# 080-audit-llm-cli-boundaries — brief

## Goal

Implement the agreed outcome of the LLM/CLI boundary audit: a dedicated
`grove-llm` binary that holds every CLI verb whose primary caller is
the LLM driving the grove process. The verbs migrate the deterministic
mechanics currently expressed as prose instructions in `content/SKILL.md`
into a stable, parser-checked surface. The human-facing `grove` binary
shrinks correspondingly: only the verbs a human invokes (launchers,
repo admin, diagnostics) remain on it.

The audit also surfaced one human-side QoL verb (`grove do`) which
ships in this subtree alongside the LLM verbs because it fits the
"abstract deterministic mechanics into CLI" principle at the human
audience level.

## Done when

- A second `[[bin]]` target `grove-llm` exists in the Cargo crate,
  installed alongside `grove` by the Homebrew formula, with these
  flat verbs implemented:

  ```
  grove-llm pick
  grove-llm brief-chain       [<leaf-path>]
  grove-llm leaf-add          <slug> [--prefix NNN] [--kind work|planning]
  grove-llm leaf-insert       <prefix>-<slug> [--kind ...]
  grove-llm leaf-decompose    <leaf-path>
  grove-llm leaf-retire       <leaf-path>
  grove-llm inbox-add         --to=<name> --body{,-file,-stdin}=...   (migrated from `grove inbox add`)
  grove-llm inbox-drain       --for=<name> [--incorporated=... --deferred=... --rejected=...]   (migrated)
  ```

- `grove inbox show` survives on the human surface (re-confirmed by
  the migration leaf; may be flattened to `grove inbox-show` if the
  one-subcommand `inbox` cluster reads worse than the flat verb).

- `grove do <name>` exists on the human surface: starts the grove if
  absent, continues it if a worktree exists, otherwise reports the
  state and offers a path forward (the launched session does the
  judgement, the verb just dispatches).

- `content/SKILL.md` and every `content/prompts/*.md` reference the
  new verbs. Prose that currently directs the LLM to perform a
  mechanical step is rewritten to direct the LLM to call the
  corresponding `grove-llm` verb.

- An ADR records the binary-separation decision (audience-split:
  porcelain vs LLM-process surface). Hard to reverse, surprising
  without context, real trade-off — meets `grilling.md`'s three
  criteria.

- Glossary entries in `CONTEXT.md` that name old verbs (`Inbox`,
  `Drain`, `grove-meta branch`) are updated. No new glossary terms
  expected; the new verbs are not new domain concepts.

- All six child leaves retired into `done/`; this node retires when
  none remain live.

## The audit's outcome (in one screen)

- **Promotion criterion: determinism.** Every deterministic step in
  the SKILL.md loop with a stable input and output shape is a
  CLI-verb candidate. Observed-failure is not required; the
  motivation is "abstract deterministic mechanics so the LLM can
  focus on the non-repetitive work, and also provides a stronger
  guarantee of correctness."

- **Audience split: separate binary `grove-llm`.** All LLM-driven
  mid-session verbs live on a binary distinct from `grove`. The
  human's `grove --help` shows no LLM verbs; the LLM's
  `grove-llm --help` shows nothing else. Precedent: git
  porcelain/plumbing, kubectl/kubeadm.

- **Within `grove-llm`: flat verb names.** No noun-cluster nesting.
  One `grove-llm --help` invocation lists every verb the LLM
  should call — important for bootstrap-recovery if a session
  drops context. `leaf-insert` reads identically to `leaf insert`
  for an LLM, but flat gives clap a global suggestion pool on
  typo.

- **Migration scope.** Two existing verbs migrate to `grove-llm`:
  `grove inbox add` → `grove-llm inbox-add`, `grove inbox drain` →
  `grove-llm inbox-drain`. Launcher-shaped verbs
  (`start|continue|takeover|retire|finish`) and repo admin
  (`install|update|uninstall|version|status|list`,
  `meta init|remote|sync`) stay on `grove` — they are
  human-launched. Backward-compat (hard cutover vs deprecated
  aliases) is decided in leaf 010's brief.

- **Steps that stay as prose.** Execute (the grilling procedure
  itself, code/doc authorship, ADR judgement); deciding which ADRs
  to read from each brief during Bootstrap; commit message
  authorship and scope; deciding what content survives a node
  retirement into ADR/glossary/parent brief; deciding what
  promotes during Finish.

## Decomposition

Six work leaves, ordered to minimise blocking. The scaffold leaf
must land first; everything else depends on the `grove-llm` binary
existing. The remaining leaves are independent of each other and
can ship in any order (the order below is roughly easiest-first,
which is the sequence that gives the binary dispatch the most
exercise before the harder verbs land).

- `010-scaffold-grove-llm-and-migrate-inbox.md` — stand up the
  `grove-llm` binary, migrate `inbox add` and `inbox drain`, write
  the audience-split ADR, sweep SKILL.md/prompts/Homebrew formula
  for the renamed verbs, update `CONTEXT.md` glossary entries.
- `020-grove-llm-pick.md` — depth-first walk over `.grove/`
  skipping `done/`; print the next live leaf's path. Update
  SKILL.md's Pick paragraph.
- `030-grove-llm-brief-chain.md` — walk ancestors from a given
  leaf (or `pick`'s output) and print `BRIEF.md` paths root→leaf.
  Update SKILL.md's Bootstrap paragraph.
- `040-grove-llm-leaf-add-and-insert.md` — both verbs in one leaf
  because `insert` is "add then renumber siblings" and they share
  prefix arithmetic + leaf-templating code. The renumber side
  also surfaces numeric cross-references (regex pass across the
  renumbered files and their siblings) for the operator to
  review. Update SKILL.md's Decompose paragraph.
- `050-grove-llm-leaf-decompose-and-retire.md` — `leaf-decompose`
  converts `NNN-x.md` → `NNN-x/BRIEF.md` so child leaves can be
  added; `leaf-retire` does the mechanical `git mv` into
  `.grove/done/` preserving relative path. Cascade walk stays
  prose (judgement). Update SKILL.md's Decompose and Retire
  paragraphs.
- `060-grove-do.md` — `grove do <name>` on the **human** surface.
  Dispatches to start/continue based on grove state; explains and
  offers a path forward for finished or orphaned groves. The
  in-session LLM handles any judgement; this verb is dispatch
  only. Independent of the other five; cherry-pickable first if a
  QoL win is wanted sooner.

## Pointers

- Existing CLI surface to reshape: `cargo run --bin grove -- --help`
  enumerates current verbs; `grove inbox --help` and
  `grove meta --help` show the existing sub-clusters that frame
  the migration decisions.
- The featured insert-and-renumber problem is documented inline in
  the running log below and in `020-design-seed-convention/BRIEF.md`
  notes §2 — four manual renumbers during this subtree's planning
  history are the concrete motivation.
- Methodology constraint that frames the audit:
  `content/SKILL.md` constraint 1 ("artifacts, not state").
  Promoting prose mechanics to CLI verbs hardens this constraint —
  state-shape can no longer drift across sessions because the
  verb owns the shape.

## Notes

- **The scaffold leaf is the only one that can balloon.** Binary
  set-up + clap dispatch + two migrated verbs + SKILL.md and
  prompt sweep + Homebrew formula + ADR-writing is a lot of
  surface for one session. Split during execution if it overruns
  (e.g. into `010-scaffold-grove-llm.md` and
  `015-migrate-inbox-verbs.md`) rather than over-decomposing
  preemptively.
- **`leaf-add` + `leaf-insert` bundle hides insert's complexity.**
  If the cross-reference detection (regex across BRIEF and sibling
  leaves) turns out to need its own design pass, split 040 then.
- **No PRD.** Audit decisions are fully captured by this brief plus
  the audience-split ADR planned for leaf 010. No human-facing
  agreement checkpoint beyond what those two artifacts record.
- **The `grove do` leaf is independent.** Its only entanglement
  with the rest of the subtree is that it touches `grove` (the
  human binary). The migration leaf does not modify `grove`
  substantively apart from removing the migrated `inbox add|drain`
  subcommands.
- **Audit deliverable retained in this BRIEF.** The pre-decomposition
  inventory and the per-question running log are kept below as the
  durable audit record. They document why each promote/keep
  classification landed where it did, and feed forward into each
  child leaf's brief.

## Pre-decomposition inventory

Walked `content/SKILL.md` top-to-bottom and the five
`content/prompts/*.md` files during grilling. The prompts are
one-line delegations to the skill; the mechanical steps live in
SKILL.md's loop paragraphs. The CLI already covers `inbox add|drain|show`,
`meta init|remote|sync`, and the existing `start|continue|takeover|retire|finish`.

| # | SKILL.md step | Mechanical content | Existing verb? | Outcome |
|---|---|---|---|---|
| P | **Pick** — depth-first walk, skip `done/` | Filesystem walk with a fixed ordering rule | none | → `grove-llm pick` (leaf 020) |
| B1 | **Bootstrap, brief chain** | Enumeration along the path | none | → `grove-llm brief-chain` (leaf 030) |
| B2 | **Bootstrap, ADRs cited** | Citation extraction | none | prose (judgement); optional `--with-adrs` flag deferred |
| B3 | **Bootstrap, drain inbox** | Already verb-driven | `grove inbox drain` | → migrate to `grove-llm inbox-drain` (leaf 010) |
| D1 | **Leaf→node conversion** | `git mv` + `BRIEF.md` creation | none | → `grove-llm leaf-decompose` (leaf 050) |
| D2 | **Insert + renumber siblings** (featured) | Shift prefixes, update headers, surface cross-refs | none | → `grove-llm leaf-insert` (leaf 040) |
| D3 | **Append leaf at end** | Templated leaf at next prefix | none | → `grove-llm leaf-add` (leaf 040) |
| R1 | **Retire a finished leaf** — `mv` into `done/` | The `mv`, path preservation | partial (`grove retire` is node-shaped) | → `grove-llm leaf-retire` (leaf 050) |
| R2 | **Parent-chain cascade after retire** | Empty-ancestor detection + ask user + promote brief | partial | prose for cascade-walk and brief-promotion; leaf-mv via `leaf-retire` |
| F1 | **Finish, promote outliving content** | Judgement | yes (`grove finish`) | prose |
| F2 | **Finish, delete `.grove/`** | `rm -rf` + commit | yes (inside `grove finish`) | already verbed |
| I1 | **Inbox name discovery on capture** | Could collide via near-duplicate | partial | deferred — implement if a near-duplicate is observed |
| C1 | **Capture an observation** | Already verb-driven | `grove inbox add` | → migrate to `grove-llm inbox-add` (leaf 010) |
| H | **`grove do <name>`** (human QoL) | State-dispatch | none | → `grove do` on human binary (leaf 060) |

## Decisions (running log)

**Q1 — Promotion criterion (settled 2026-05-28).** The audit promotes
on **determinism**, not on observed failure. Rationale (user):
"I have seen no failure. I am wanting to abstract the deterministic
mechanics of the grove process [away from] the LLM [so it] can focus
on the non-repetitive work, and also it provides a stronger guarantee
of correctness." Consequence: every deterministic step in SKILL.md
that has a stable input shape and a stable output shape is a
**promote** candidate; "keep as prose" is reserved for judgement
(reading-and-synthesising, deciding what to ADR, deciding what brief
content survives a node retirement). The hybrid bin (prose narrates
the judgement, CLI does the mechanical sub-step) is the dominant
pattern in the final inventory.

**Q2 — Namespace shape (settled 2026-05-28; refined by Q4).** The
audit introduces an **audience-split** in the CLI: a dedicated
LLM-driven surface holds the verbs that exist for the LLM to drive
the process deterministically. Rationale (user): "I anticipate two
CLI surfaces — those that the user drives, and those purely for an
LLM to drive the process. I would even be happy to have `grove llm`
to if not enforce that, at least make it more obvious." Initially
proposed as a `grove llm` subcommand; refined by Q4 to a separate
binary `grove-llm`.

**Q2a — Migration of existing LLM-driven verbs (settled 2026-05-28).**
Existing verbs whose primary caller is the LLM mid-session **migrate
to the LLM surface** as part of this audit's decomposition.
Rationale (user): "The existing LLM driven verbs, that aren't really
meant for human consumption, should definitely be rationalised and
moved to the llm subcommand. The smaller the human CLI surface, the
less confusing it is." Inventory: `grove inbox add` → `grove-llm
inbox-add`; `grove inbox drain` → `grove-llm inbox-drain`. Borderline:
`grove inbox show` stays on the human surface as a diagnostic — to be
re-confirmed in leaf 010. Launcher-shaped and repo-admin verbs are
explicitly out of scope (they are human-invoked).

**Q3 — Within the LLM surface, flat vs nested (settled 2026-05-28).**
Verbs are **flat**. Rationale: considered *entirely from LLM use*
(user steer). One `--help` invocation lists every verb the LLM
should call — faster bootstrap-recovery if a session drops context.
Flat verb names give clap a global suggestion pool on typo and
read identically to an LLM regardless of dash-vs-space. The
conceptual-hierarchy and future-proofing arguments for nesting were
human-centric and did not survive the LLM-only frame.

**Q4 — Separate binary `grove-llm` (settled 2026-05-28; supersedes
the `grove llm` subcommand shape of Q2/Q3).** Rationale (user):
"I would be happy with a separate cli even — `grove-llm`." The
audience split moves from nominal (a subcommand the human is told
to ignore) to **structural** (a separate binary that does not
appear in `grove --help` at all). Precedent: git porcelain/plumbing,
kubectl/kubeadm. Cost: one extra `[[bin]]` target in the existing
Cargo crate, sharing all internal libs; one extra line in the
Homebrew formula.

**Q5 — Granularity of decomposition (settled 2026-05-28).** This
planning leaf decomposes into a node `080-audit-llm-cli-boundaries/`
with `BRIEF.md` and five child work leaves (E-2 shape). Each leaf
bundles its verb(s) with the SKILL.md prose update for the
corresponding loop paragraph. Pairings respect shared mechanics:
`leaf-add` and `leaf-insert` share templating and prefix
arithmetic; `leaf-decompose` and `leaf-retire` are both `mv`-flavored
operations on an existing leaf path. `pick` and `brief-chain` ship
separately because they share no code and split-first lets the
binary's dispatch wiring validate on a small verb. The scaffold
leaf also authors an ADR for the binary separation (Q4 meets all
three of `grilling.md`'s ADR criteria) — no separate ADR-write
leaf.

**Q6 — `grove do <name>` (settled 2026-05-28, added during this
session).** User QoL: a single verb on the **human** surface that
starts the grove if absent, continues it if a worktree exists, and
otherwise reports the state and offers a path forward (re-attach
orphaned branch; explain a finished grove). Lives in `grove`, not
`grove-llm`, because the audience is the human at the terminal and
the value is that one line in shell history works across all
session states. Slots in as the sixth child leaf, independent of
the `grove-llm` work and cherry-pickable first if a QoL win is
wanted sooner.
